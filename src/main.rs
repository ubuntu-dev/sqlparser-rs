extern crate sqlparser;

use sqlparser::sqlparser::*;
use sqlparser::dialect::GenericSqlDialect;

fn main() {
    let sql = " (a + b) - (c + d)";
    let dialect = GenericSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql.to_string()).unwrap();

    println!("AST: {:?}", ast);
}

// SQLBinaryExpr { 
//     left: SQLBinaryExpr { 
//         left: SQLBinaryExpr { 
//             left: SQLIdentifier("first_name"), 
//             op: Eq, 
//             right: SQLLiteralString("a") }, 
//         op: Or, 
//         right: SQLBinaryExpr { 
//             left: SQLIdentifier("last_name"), 
//             op: Eq, 
//             right: SQLLiteralString("b") 
//         } 
//     }, 
//     op: And, 
//     right: SQLBinaryExpr { 
//         left: SQLIdentifier("num"), 
//         op: Eq, 
//         right: SQLLiteralLong(3) 
//     } 
// }

// selection: Some(SQLBinaryExpr { 
//     left: SQLBinaryExpr { 
//         left: SQLIdentifier("first_name"), 
//         op: Eq, 
//         right: SQLLiteralString("a") 
//     }, 
//     op: Or, 
//     right: SQLBinaryExpr { 
//         left: SQLBinaryExpr { 
//             left: SQLIdentifier("last_name"), 
//             op: Eq, right: SQLLiteralString("b") 
//         }, 
//         op: And, 
//         right: SQLBinaryExpr { 
//             left: SQLIdentifier("num"), op: Eq, right: SQLLiteralLong(3) } } }), order_by: None, group_by: None, having: None, limit: None }
// check tree rotation?
// Some(SQLBinaryExpr { 
//     left: SQLBinaryExpr { 
//         left: SQLBinaryExpr { 
//             left: SQLIdentifier("first_name"), 
//             op: Eq, 
//             right: SQLLiteralString("a") 
//         }, 
//         op: Or, 
//         right: SQLBinaryExpr { 
//             left: SQLIdentifier("last_name"), 
//             op: Eq, 
//             right: SQLLiteralString("b") 
//         } 
//     }, 
//     op: And, 
//     right: SQLBinaryExpr { 
//         left: SQLIdentifier("num"), 
//         op: Eq, 
//         right: SQLLiteralLong(3) 
//     } 
// }