use super::*;

#[test]
fn result_projection_uses_eigenvectors_and_correct_units_for_both_eigenmodes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/👁️result-projection/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mode = ResultMode::try_from(case["mode"].as_str().unwrap()).unwrap().display(case["index"].as_u64().unwrap() as u32);
        assert_eq!(mode.uses_mode_shape(), case["modeShape"].as_bool().unwrap(), "{}", case["id"]);
        let actual = mode.display_eigenvalue(case["eigenvalue"].as_f64().unwrap());
        match case["displayValue"].as_f64() {
            Some(expected) => assert!((actual.unwrap() - expected).abs() < 1e-12, "{}", case["id"]),
            None => assert_eq!(actual, None),
        }
    }
    eprintln!("[DEBUG] FEM result projection five portable cases distinguish static displacement, modal frequency and buckling load multiplier");
}
