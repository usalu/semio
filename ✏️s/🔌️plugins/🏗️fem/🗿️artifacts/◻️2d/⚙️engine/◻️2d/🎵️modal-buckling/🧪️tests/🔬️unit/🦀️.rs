use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::planar::FemMaterialValue;
use semio_s_fem_node::planar::FemNodeValue;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSupport;

// #region 🔖️Fixtures
fn simply_supported_beam_doc() -> Fem2dSnapshot {
    Fem2dSnapshot {
        nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "n2".into(), x: 6.0, y: 0.0 }],
        elements: vec![crate::FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
        regions: vec![],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
        load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

/// 🟩️ A 4x2m rectangular region (steel, 0.02m thick, 1m mesh) whose 4 corners are pre-placed as
/// document nodes.
fn rectangle_region_doc() -> Fem2dSnapshot {
    Fem2dSnapshot {
        nodes: vec![FemNodeValue { id: "c0".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "c1".into(), x: 4.0, y: 0.0 }, FemNodeValue { id: "c2".into(), x: 4.0, y: 2.0 }, FemNodeValue { id: "c3".into(), x: 0.0, y: 2.0 }],
        elements: vec![],
        regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![],
        supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
        load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}
// #endregion 🔖️Fixtures

#[test]
fn fem2d_modal_returns_requested_mode_count() {
    let doc = rectangle_region_doc();
    let result = fem2d_modal(&doc).expect("modal solves");
    assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
    for w in result.frequencies_hz.windows(2) {
        assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
    }
    for &f in &result.frequencies_hz {
        assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
    }
}

#[test]
fn fem2d_modal_mode_values_returns_node_displacements() {
    let doc = simply_supported_beam_doc();
    let (freq, values) = fem2d_modal_mode_values(&doc, 0).expect("modal mode values solves");
    assert!(freq.is_finite() && freq >= 0.0);
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

#[test]
fn fem2d_buckling_returns_requested_mode_count() {
    let doc = simply_supported_beam_doc();
    let result = fem2d_buckling(&doc, "dead").expect("buckling solves");
    assert_eq!(result.factors.len(), doc.analysis.buckling_count);
    for &f in &result.factors {
        assert!(f.is_finite(), "buckling factor should be finite: {f}");
    }
}

#[test]
fn fem2d_buckling_mode_values_returns_node_displacements() {
    let doc = simply_supported_beam_doc();
    let (factor, values) = fem2d_buckling_mode_values(&doc, "dead", 0).expect("buckling mode values solves");
    assert!(factor.is_finite());
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

#[test]
fn fem2d_buckling_unknown_case_errors() {
    let doc = simply_supported_beam_doc();
    let err = fem2d_buckling(&doc, "missing").err().expect("expected error");
    assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
}

#[test]
fn continuum_buckling_retains_mesh_nodes_elements_and_area_prestress() {
    let mut doc = rectangle_region_doc();
    doc.load_cases[0].self_weight = false;
    doc.load_cases[0].loads = vec![FemLoad::Area { id: "pressure".into(), region_id: "r1".into(), pressure: 3000.0 }];
    let (expected_nodes, expected_elements, _) = build_nodes_and_elements(&doc).unwrap();
    let (nodes, elements, _, case) = buckling_inputs(&doc, "self").unwrap();
    assert!(nodes.len() > doc.nodes.len());
    assert_eq!(nodes.len(), expected_nodes.len());
    assert_eq!(elements.len(), expected_elements.len());
    assert!((case.nodal_loads.iter().map(|load| load.value).sum::<f64>() + 24000.0).abs() < 1e-8);
    let (factor, values) = fem2d_buckling_mode_values(&doc, "self", 0).unwrap();
    assert!(factor.is_finite() && factor > 0.0);
    assert_eq!(values.len(), nodes.len());
}

#[test]
fn continuum_buckling_rejects_unknown_area_region() {
    let mut doc = rectangle_region_doc();
    doc.load_cases[0].loads = vec![FemLoad::Area { id: "pressure".into(), region_id: "missing".into(), pressure: 3000.0 }];
    assert!(matches!(buckling_inputs(&doc, "self"), Err(Fem2dError::UnknownRegionId(id)) if id == "missing"));
}

#[test]
fn continuum_static_modal_and_buckling_match_scikit_fem_and_scipy() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧫️fixtures/🧱️solves-fem2d-1-continuum/📊️expected.results.json")).unwrap();
    for (name, snapshot) in [
        ("🔺️triangle.snapshot.json", include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧫️fixtures/🧱️solves-fem2d-1-continuum/🔺️triangle.snapshot.json")),
        ("🌉️mixed.snapshot.json", include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧫️fixtures/🧱️solves-fem2d-1-continuum/🌉️mixed.snapshot.json")),
        ("🕳️hole.snapshot.json", include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧫️fixtures/🧱️solves-fem2d-1-continuum/🕳️hole.snapshot.json")),
        ("📏️scaled-hole.snapshot.json", include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧫️fixtures/🧱️solves-fem2d-1-continuum/📏️scaled-hole.snapshot.json")),
    ] {
        let report = semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::io::text::mutations::fem2d_continuum_report_json(snapshot, matches!(name, "🔺️triangle.snapshot.json" | "🌉️mixed.snapshot.json")).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&report).unwrap();
        fn assert_response(actual: &serde_json::Value, expected: &serde_json::Value, scales: &serde_json::Value, key: &str) {
            match (actual, expected) {
                (serde_json::Value::Object(actual), serde_json::Value::Object(expected)) => {
                    assert_eq!(actual.len(), expected.len());
                    for (key, value) in actual {
                        assert_response(value, &expected[key], scales, key);
                    }
                }
                (serde_json::Value::Array(actual), serde_json::Value::Array(expected)) => {
                    assert_eq!(actual.len(), expected.len());
                    for (actual, expected) in actual.iter().zip(expected) {
                        assert_response(actual, expected, scales, key);
                    }
                }
                _ => {
                    let axis = match key { "displacements" => "translation", "reactions" => "force", "frequenciesHz" => "frequency", "factors" => "factor", _ => "stress" };
                    let actual = actual.as_f64().unwrap();
                    let expected = expected.as_f64().unwrap();
                    assert!((actual - expected).abs() / scales[axis].as_f64().unwrap() < 1e-6, "{key}: {actual} != independent {expected}");
                }
            }
        }
        assert_response(&actual, &expected[name]["answer"], &expected[name]["scales"], "");
        eprintln!("[DEBUG] continuum static/modal/buckling independent reference matched for {name}");
    }
}
