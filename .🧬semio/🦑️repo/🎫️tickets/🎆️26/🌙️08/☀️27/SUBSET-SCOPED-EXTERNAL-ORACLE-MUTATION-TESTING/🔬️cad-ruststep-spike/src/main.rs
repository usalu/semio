//! 🔍️ Evidence spike: does `ruststep` 0.4.0 actually READ and WRITE Part-21?
//!
//! Run:  cargo run --offline --manifest-path <this crate>/Cargo.toml
//!
//! READ — passes. The `parse` half below prints the fully resolved entity graph for a file in exactly
//! the shape `SemioCadToStep` emits, including `#N` reference resolution and `$` placeholders.
//!
//! WRITE — fails, two ways, and both are why the STEP fixtures are classed `handcrafted`:
//!
//!   1. No AST type reaches text. Uncomment the `NO_TEXT_WRITER` block and the crate refuses to
//!      compile with three errors:
//!         error[E0277]: `Record` doesn't implement `std::fmt::Display`
//!         error[E0277]: `DataSection` doesn't implement `std::fmt::Display`
//!         error[E0277]: `Exchange` doesn't implement `std::fmt::Display`
//!      A grep of the whole crate finds `fmt::Display` impls only on `error::TokenizeFailed` and
//!      `primitive::logical::Logical`.
//!
//!   2. Even at AST level the serializer disagrees with the crate's own parser: `to_record` FLATTENS
//!      a coordinate list into sibling parameters, losing the nesting the STEP grammar requires. The
//!      `serialize` half below prints both so they can be compared directly.

use ruststep::ast::{ser::to_record, Exchange, Parameter, Record};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
#[serde(rename = "CARTESIAN_POINT")]
struct CartesianPoint {
    name: String,
    coordinates: Vec<f64>,
}

const CAD_SHAPED_STEP: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio-cad-export','',(''),(''),'','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.0,0.0,0.0));\n#2=DIRECTION('',(1.0,0.0,0.0));\n#3=VECTOR('',#2,5.0);\n#4=LINE('',#1,#3);\n#5=CARTESIAN_POINT('',(2.0,2.0,0.0));\n#6=AXIS2_PLACEMENT_3D('',#5,$,$);\n#7=CIRCLE('',#6,1.5);\nENDSEC;\nEND-ISO-10303-21;\n";

fn main() {
    println!("== READ: ruststep parses a cad-shaped Part-21 file ==");
    let exchange = Exchange::from_str(CAD_SHAPED_STEP).expect("ruststep parses the cad-shaped file");
    println!("header records = {}", exchange.header.len());
    for section in &exchange.data {
        println!("data entities  = {}", section.entities.len());
        for entity in &section.entities {
            println!("  {entity:?}");
        }
    }

    println!();
    println!("== WRITE: to_record disagrees with the crate's own parser ==");
    let serialized: Record = to_record(&CartesianPoint { name: String::new(), coordinates: vec![1.0, 2.0, 0.0] }).expect("to_record");
    println!("ser::to_record  -> {:?}", serialized.parameter);
    let parsed = Record::from_str("CARTESIAN_POINT('',(1.0,2.0,0.0))").expect("parse the same entity");
    println!("parser          -> {:?}", parsed.parameter);
    let agrees = matches!(&serialized.parameter, Parameter::List(v) if v.len() == 2);
    println!("nesting agrees  -> {agrees}   (false: the coordinate list was flattened into siblings)");

    // NO_TEXT_WRITER — uncomment to reproduce the three E0277 Display errors quoted above.
    // println!("{}", serialized);
    // println!("{}", exchange);
}
