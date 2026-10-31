use super::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, print_dsl, FEM2D_EXAMPLE_TEXT};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::inferences::bounds::compute_fem2d_bounds;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::inferences::{Fem2dBounds, Fem2dInference};
use protocol::Inference;

fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1048576, maximum_capacity_bytes: 256 * 1048576, maximum_release_bytes: 256 * 1048576, maximum_depth: 128 };
    while !owner.terminal_is_empty() {
        owner.step(grant).unwrap();
    }
}

fn persist(value: &Fem2dSnapshotValue) -> Fem2dSnapshot {
    Fem2dSnapshot::from_reference(value.clone()).expect("fixture loads as a persistent snapshot")
}

fn simply_supported_beam_doc() -> Fem2dSnapshotValue {
    Fem2dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "n2".into(), x: 6.0, y: 0.0 }],
        elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
        regions: vec![],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
        load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

fn truss_doc() -> Fem2dSnapshotValue {
    Fem2dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "n2".into(), x: 4.0, y: 0.0 }, FemNodeValue { id: "n3".into(), x: 4.0, y: 3.0 }],
        elements: vec![
            FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            FemElement::Bar { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
        ],
        regions: vec![],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "rod".into(), name: "rod".into(), area: 0.001, iy: 0.0 }],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
        load_cases: vec![FemLoadCase {
            id: "dead".into(),
            name: "dead".into(),
            loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: FemDof::Ty, value: -1000.0 }, FemLoad::Nodal { id: "l2".into(), node_id: "n3".into(), dof: FemDof::Tx, value: -500.0 }],
            self_weight: false,
        }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

fn rectangle_region_doc() -> Fem2dSnapshotValue {
    Fem2dSnapshotValue {
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

fn rectangle_with_hole_region_doc() -> Fem2dSnapshotValue {
    let mut doc = rectangle_region_doc();
    doc.regions[0].holes = vec![vec![[1.5, 0.75], [2.5, 0.75], [2.5, 1.25], [1.5, 1.25]]];
    doc
}

fn extent_value() -> Fem2dSnapshotValue {
    Fem2dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n1".into(), x: -2.0, y: 1.0 }, FemNodeValue { id: "n2".into(), x: 5.0, y: 1.0 }, FemNodeValue { id: "n3".into(), x: 5.0, y: 7.5 }],
        elements: vec![FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }],
        materials: vec![FemMaterialValue { id: "m1".into(), name: "m1".into(), e: 1.0, nu: 0.3, rho: 1.0 }],
        sections: vec![FemSection { id: "s1".into(), name: "s1".into(), area: 1.0, iy: 1.0 }],
        ..Default::default()
    }
}

fn round_trips(value: &Fem2dSnapshotValue) {
    let source = persist(value);
    assert_eq!(&source.to_reference(), value, "the persistent snapshot reproduces its reference rows");
    let text = print_dsl(&source);
    let via_dsl = parse_dsl(&text).expect("printed DSL parses");
    assert_eq!(print_dsl(&via_dsl), text, "DSL print is a fixed point");
    let packed = pack::snapshot::ArtifactPack::encode_pack(&source);
    let via_pack = <Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&packed).expect("pack decodes");
    assert_eq!(&via_dsl.to_reference(), value, "dsl round trip diverged from source snapshot");
    assert_eq!(&via_pack.to_reference(), value, "pack round trip diverged from source snapshot");
    assert_eq!(via_pack.to_reference(), via_dsl.to_reference(), "pack and dsl round trips diverged from each other");
    drain(source);
    drain(via_dsl);
    drain(via_pack);
}

#[test]
fn fem2d_dsl_round_trips_bundled_default_example() {
    let document = parse_dsl(FEM2D_EXAMPLE_TEXT).expect("parse default example");
    round_trips(&document.to_reference());
    drain(document);
}

#[test]
fn fem2d_dsl_and_pack_round_trip_fixture_documents() {
    round_trips(&Fem2dSnapshotValue::default());
    round_trips(&simply_supported_beam_doc());
    round_trips(&truss_doc());
    round_trips(&rectangle_with_hole_region_doc());
    let mut with_combination = simply_supported_beam_doc();
    with_combination.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] });
    round_trips(&with_combination);
}

#[test]
fn inference_determinism_default_and_node_extent_laws() {
    let sample = Fem2dSnapshotValue { nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "n2".into(), x: 4.0, y: 0.0 }, FemNodeValue { id: "n3".into(), x: 4.0, y: 3.0 }], ..Default::default() };
    let snapshot = persist(&sample);
    assert_eq!(Fem2dInference::infer(&snapshot).expect("inference"), Fem2dInference::infer(&snapshot).expect("inference"));
    let inferred = Fem2dInference::infer(&snapshot).expect("inference");
    assert_eq!(inferred.bounds.node_count, 3);
    assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0]);
    assert_eq!(inferred.bounds.bounding_box.max, [4.0, 3.0]);
    drain(snapshot);
    let empty = Fem2dSnapshot::default();
    assert_eq!(Fem2dInference::infer(&empty).expect("inference"), Fem2dInference::default());
    assert_eq!(compute_fem2d_bounds(&empty), Fem2dBounds::default());
    drain(empty);
}

#[test]
fn bounds_match_hand_built_node_extent() {
    let snapshot = persist(&extent_value());
    assert_eq!(compute_fem2d_bounds(&snapshot), compute_fem2d_bounds(&snapshot));
    let bounds = compute_fem2d_bounds(&snapshot);
    assert_eq!(bounds.bounding_box.min, [-2.0, 1.0]);
    assert_eq!(bounds.bounding_box.max, [5.0, 7.5]);
    assert_eq!(bounds.node_count, 3);
    assert_eq!(bounds.element_count, 1);
    drain(snapshot);
}
