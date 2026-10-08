use super::*;

#[test]
fn phases_map_by_name_and_default_to_new() {
    assert_eq!(phase("Existing"), Phase::Existing);
    assert_eq!(phase("Demolished"), Phase::Demolished);
    assert_eq!(phase("Temporary"), Phase::Temporary);
    assert_eq!(phase(""), Phase::New);
}
