//! [DEBUG] One-off verification for brep.solid.extrude (Extrude Curve).

use flow_module_wasm::evaluate_json;
use neural_engine::{Atom, Dictionary, Registry, Value};
use flow_module_brep::register;

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn vector(x: f64, y: f64, z: f64) -> Dictionary {
    Dictionary::with_schema("vector")
        .insert("x", Value::Atom(Atom::Decimal(x)))
        .insert("y", Value::Atom(Atom::Decimal(y)))
        .insert("z", Value::Atom(Atom::Decimal(z)))
}

fn main() {
    let mut reg = Registry::new();
    register(&mut reg);
    let wire_json = evaluate_json(
        &reg,
        "brep.curve.polygon",
        &serde_json::to_string(&Dictionary::new()
            .insert("radius", Value::Dictionary(number_dictionary(0.5)))
            .insert("sides", Value::Dictionary(number_dictionary(6.0))))
        .unwrap(),
    );
    println!("[DEBUG] wire: {wire_json}");
    let wire: Dictionary = serde_json::from_str(&wire_json).unwrap();
    let wire = wire.get("wire").and_then(|v| v.as_dictionary()).expect("wire channel");
    let solid_json = evaluate_json(
        &reg,
        "brep.solid.extrude",
        &serde_json::to_string(&Dictionary::new()
            .insert("wire", Value::Dictionary(wire.clone()))
            .insert("vector", Value::Dictionary(vector(0.0, 0.0, 6.0))))
        .unwrap(),
    );
    println!("[DEBUG] solid: {solid_json}");
    assert!(!solid_json.contains("\"error\""));
    assert!(solid_json.contains("\"solid\""));
}
