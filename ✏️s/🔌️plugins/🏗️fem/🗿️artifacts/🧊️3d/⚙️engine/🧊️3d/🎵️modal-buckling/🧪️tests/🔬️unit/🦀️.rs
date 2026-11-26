use super::*;
#[path = "../../../🧪️tests/🧰️persisted/🦀️.rs"]
mod persisted;
use persisted::{persist, Persisted};
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemMaterialValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemNodeValue;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSupport;

fn cantilever_fixture() -> Persisted {
    let (e, g, a, iy, iz, j, l, p) = (210e9, 80.77e9, 0.00538, 0.0000369, 0.0000133, 0.00000060, 3.0, 5000.0);
    persist(Fem3dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: l, y: 0.0, z: 0.0 }],
        elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e, g, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
        solids: vec![],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
        load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    })
}

#[test]
fn fem3d_modal_returns_requested_mode_count() {
    let doc = cantilever_fixture();
    let result = fem3d_modal(&doc).expect("modal solves");
    assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
    for w in result.frequencies_hz.windows(2) {
        assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
    }
    for &f in &result.frequencies_hz {
        assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
    }
}

#[test]
fn fem3d_modal_mode_values_returns_node_displacements() {
    let doc = cantilever_fixture();
    let (freq, values) = fem3d_modal_mode_values(&doc, 0).expect("modal mode values solves");
    assert!(freq.is_finite() && freq >= 0.0);
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

/// 🏛️ The cantilever of `cantilever_fixture` loaded in compression along its axis: the classical fixed-free Euler column.
fn column_fixture() -> Persisted {
    let mut doc = cantilever_fixture();
    doc.load_cases[0].loads = vec![semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tx, value: -5000.0 }];
    doc.analysis.buckling_count = 2;
    doc
}

#[test]
fn fem3d_buckling_returns_euler_factor_of_the_fixed_free_column() {
    let doc = column_fixture();
    let result = fem3d_buckling(&doc, "point").expect("buckling solves");
    assert_eq!(result.factors.len(), doc.analysis.buckling_count);
    assert!(result.factors.windows(2).all(|pair| pair[0] <= pair[1]), "factors should be ascending: {:?}", result.factors);
    let euler = std::f64::consts::PI.powi(2) * 210e9 * 0.0000133 / (4.0 * 3.0_f64.powi(2)) / 5000.0;
    assert!((result.factors[0] - euler).abs() / euler < 0.02, "first factor {} vs Euler {euler}", result.factors[0]);
}

#[test]
fn fem3d_buckling_transverse_load_has_no_positive_factors() {
    let doc = cantilever_fixture();
    let result = fem3d_buckling(&doc, "point").expect("buckling solves");
    assert!(result.factors.is_empty());
}

#[test]
fn fem3d_buckling_mode_values_returns_node_displacements() {
    let doc = column_fixture();
    let (factor, values) = fem3d_buckling_mode_values(&doc, "point", 0).expect("buckling mode values solves");
    assert!(factor.is_finite() && factor > 0.0);
    assert!(values.contains_key("n1"));
    assert!(values.contains_key("n2"));
}

#[test]
fn fem3d_buckling_unknown_case_errors() {
    let doc = cantilever_fixture();
    let err = fem3d_buckling(&doc, "missing").err().expect("expected error");
    assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
}

fn demo_document() -> Persisted {
    Persisted::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("bundled demo"))
}

/// 🧪️ LAW: the demo's natural frequencies are finite, strictly positive and ascending, and every mode index selects its own shape.
#[test]
fn the_demo_modal_frequencies_are_positive_ascending_and_each_index_has_its_own_shape() {
    let doc = demo_document();
    let result = fem3d_modal(&doc).expect("demo modal solves");
    assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
    assert!(result.frequencies_hz.iter().all(|frequency| frequency.is_finite() && *frequency > 0.0), "{:?}", result.frequencies_hz);
    assert!(result.frequencies_hz.windows(2).all(|pair| pair[0] <= pair[1]), "{:?}", result.frequencies_hz);
    let shapes: Vec<_> = (0..doc.analysis.modal_count.min(3)).map(|index| fem3d_modal_mode_values(&doc, index).expect("mode values")).collect();
    for (index, (frequency, values)) in shapes.iter().enumerate() {
        assert!((frequency - result.frequencies_hz[index]).abs() <= 1e-9 * frequency.abs().max(1.0), "mode {index} carries its own frequency");
        assert!(values.values().any(|node| node.iter().any(|component| component.abs() > 0.0)), "mode {index} deforms");
    }
    for first in 0..shapes.len() {
        for second in first + 1..shapes.len() {
            assert_ne!(format!("{:?}", shapes[first].1.iter().collect::<std::collections::BTreeMap<_, _>>()), format!("{:?}", shapes[second].1.iter().collect::<std::collections::BTreeMap<_, _>>()), "modes {first} and {second} are different shapes");
        }
    }
}

/// 🧪️ LAW: the demo's buckling either returns finite, positive, ascending factors or an explicit empty set that the mode-values entry point refuses by index, never a generic failure.
#[test]
fn the_demo_buckling_returns_positive_factors_or_a_localized_refusal() {
    let doc = demo_document();
    let case = doc.load_cases.first().expect("demo load case").id.clone();
    let result = fem3d_buckling(&doc, &case).expect("demo buckling solves");
    assert!(result.factors.iter().all(|factor| factor.is_finite() && *factor > 0.0) && result.factors.windows(2).all(|pair| pair[0] <= pair[1]), "{:?}", result.factors);
    match fem3d_buckling_mode_values(&doc, &case, 0) {
        Ok((factor, values)) => {
            assert!(!result.factors.is_empty() && factor == result.factors[0]);
            assert!(values.values().any(|node| node.iter().any(|component| component.abs() > 0.0)), "the first buckling mode deforms");
        }
        Err(error) => assert!(result.factors.is_empty() && error.to_string().contains("mode index out of range"), "{error}"),
    }
}

/// 🧪️ LAW: an eight-element cantilever's first two dense-engine frequencies land on the Euler-Bernoulli closed form within two percent (the independent result the mounted job is held to as well).
#[test]
fn the_dense_cantilever_frequencies_match_the_euler_bernoulli_closed_form() {
    let (e, rho, area, iy, iz, length) = (210e9_f64, 7850.0_f64, 0.00538_f64, 3.692e-5_f64, 1.336e-5_f64, 4.0_f64);
    let mut value = Fem3dSnapshotValue {
        nodes: (0..=8).map(|index| FemNodeValue { id: format!("n{index}"), x: length * index as f64 / 8.0, y: 0.0, z: 0.0 }).collect(),
        elements: (0..8).map(|index| FemElement::Frame { id: format!("e{index}"), start: format!("n{index}"), end: format!("n{}", index + 1), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }).collect(),
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e, g: 80.769230769e9, nu: 0.3, rho }],
        sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area, iy, iz, j: 2.098e-7 }],
        supports: vec![FemSupport { id: "fixity".into(), node_id: "n0".into(), fixed: FemDof::ALL.to_vec() }],
        load_cases: vec![FemLoadCase { id: "idle".into(), name: "Idle".into(), loads: vec![], self_weight: false }],
        ..Default::default()
    };
    value.analysis.modal_count = 2;
    let doc = persist(value);
    let result = fem3d_modal(&doc).expect("cantilever modal solves");
    let closed = |second_moment: f64| 1.8751040687_f64.powi(2) / (std::f64::consts::TAU * length * length) * (e * second_moment / (rho * area)).sqrt();
    for (index, expected) in [closed(iz), closed(iy)].into_iter().enumerate() {
        assert!((result.frequencies_hz[index] - expected).abs() / expected < 0.02, "mode {index}: {} Hz against the closed form {expected} Hz", result.frequencies_hz[index]);
    }
}
