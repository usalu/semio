use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Phase;
use semio_s_artifact_stdio_contract::part21::Part21Value;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader::{reals, text, Doc};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::Import;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Building, Site, Storey};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point2;
#[allow(unused_imports)]
use std::collections::BTreeMap;
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::spatial::{degrees, number_of, string_of, phase};
use semio_s_artifact_stdio_contract::part21::Part21Decimal;

fn real(value: f64) -> Part21Value {
    Part21Value::Real(Part21Decimal::from_f64(value))
}

#[test]
fn degrees_reverse_the_compound_angle_with_its_sign() {
    assert!((degrees(&[47, 22, 36, 840_000].map(f64::from)) - 47.3769).abs() < 1e-9);
    assert!((degrees(&[0.0, -30.0, 0.0, 0.0]) + 0.5).abs() < 1e-12);
    assert_eq!(degrees(&[]), 0.0);
}

#[test]
fn numbers_and_strings_are_read_through_their_type_wrappers() {
    let wrapped = Part21Value::Typed { name: "IFCREAL".into(), items: vec![real(2.5)] };
    assert_eq!(number_of(&wrapped), Some(2.5));
    assert_eq!(number_of(&real(1.0)), Some(1.0));
    let label = Part21Value::Typed { name: "IFCLABEL".into(), items: vec![Part21Value::Str("x".into())] };
    assert_eq!(string_of(&label).as_deref(), Some("x"));
    assert_eq!(string_of(&Part21Value::Str("y".into())).as_deref(), Some("y"));
    assert_eq!(string_of(&Part21Value::Unset), None);
}

#[test]
fn phases_map_by_name_and_default_to_new() {
    assert_eq!(phase("Existing"), Phase::Existing);
    assert_eq!(phase("Demolished"), Phase::Demolished);
    assert_eq!(phase("Temporary"), Phase::Temporary);
    assert_eq!(phase("New"), Phase::New);
    assert_eq!(phase(""), Phase::New);
}
