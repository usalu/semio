use super::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, print_dsl, FEM3D_EXAMPLE_TEXT};
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::inferences::bounds::compute_fem3d_bounds;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::inferences::{Fem3dBounds, Fem3dInference};
use protocol::Inference;
use semio_s_fem_receiving::testkit::retire;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use std::collections::BTreeMap;

fn persist(value: &Fem3dSnapshotValue) -> Fem3dSnapshot {
    Fem3dSnapshot::from_reference(value.clone()).expect("fixture loads as a persistent snapshot")
}


// #region 🔖️Fixtures
fn cantilever_doc() -> Fem3dSnapshotValue {
    Fem3dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: 3.0, y: 0.0, z: 0.0 }],
        elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.00000060 }],
        solids: vec![],
        supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
        load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

fn truss_doc() -> Fem3dSnapshotValue {
    Fem3dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n3".into(), x: 1.0, y: 1.0, z: 2.0 }, FemNodeValue { id: "n4".into(), x: 1.0, y: -1.0, z: 0.0 }],
        elements: vec![
            FemElement::Bar { id: "b1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            FemElement::Bar { id: "b2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            FemElement::Bar { id: "b3".into(), start: "n4".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
        ],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "rod".into(), name: "Rod".into(), area: 0.001, iy: 1e-6, iz: 1e-6, j: 1e-6 }],
        solids: vec![],
        supports: vec![
            FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() },
            FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: FemDof::ALL.to_vec() },
            FemSupport { id: "s3".into(), node_id: "n4".into(), fixed: FemDof::ALL.to_vec() },
        ],
        load_cases: vec![FemLoadCase { id: "drop".into(), name: "Drop".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: FemDof::Tz, value: -1000.0 }], self_weight: false }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}

fn solid_slab_doc() -> Fem3dSnapshotValue {
    Fem3dSnapshotValue {
        nodes: vec![FemNodeValue { id: "sc0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "sc1".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "sc2".into(), x: 2.0, y: 1.0, z: 0.0 }, FemNodeValue { id: "sc3".into(), x: 0.0, y: 1.0, z: 0.0 }],
        elements: vec![],
        materials: vec![FemMaterialValue { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
        sections: vec![],
        solids: vec![FemSolid { id: "sol1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 1.0, material_id: "concrete".into(), axis: crate::standards::v1::subsets::any::schema::structure::FemAxis::Z }],
        supports: vec![
            FemSupport { id: "s1".into(), node_id: "sc0".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            FemSupport { id: "s2".into(), node_id: "sc1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            FemSupport { id: "s3".into(), node_id: "sc2".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            FemSupport { id: "s4".into(), node_id: "sc3".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
        ],
        load_cases: vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }],
        combinations: vec![],
        analysis: FemAnalysisSettings::default(),
    }
}
// #endregion 🔖️Fixtures

fn round_trips(value: &Fem3dSnapshotValue) {
    let source = persist(value);
    assert_eq!(&source.to_reference(), value, "the persistent snapshot reproduces its reference rows");
    let text = print_dsl(&source);
    let via_dsl = parse_dsl(&text).expect("printed DSL parses");
    assert_eq!(print_dsl(&via_dsl), text, "DSL print is a fixed point");
    let packed = pack::snapshot::ArtifactPack::encode_pack(&source);
    let via_pack = <Fem3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&packed).expect("pack decodes");
    assert_eq!(&via_dsl.to_reference(), value, "dsl round trip diverged from source snapshot");
    assert_eq!(&via_pack.to_reference(), value, "pack round trip diverged from source snapshot");
    assert_eq!(via_pack.to_reference(), via_dsl.to_reference(), "pack and dsl round trips diverged from each other");
    retire(source);
    retire(via_dsl);
    retire(via_pack);
}


fn extent_doc() -> Fem3dSnapshotValue {
    Fem3dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n1".into(), x: -2.0, y: 1.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: 5.0, y: 1.0, z: -3.0 }, FemNodeValue { id: "n3".into(), x: 5.0, y: 7.5, z: 6.0 }],
        elements: vec![FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }],
        ..Default::default()
    }
}
#[test]
fn fem3d_dsl_round_trips_bundled_default_example() {
    let document = parse_dsl(FEM3D_EXAMPLE_TEXT).expect("parse default example");
    round_trips(&document.to_reference());
    retire(document);
}
#[test]
fn fem3d_dsl_and_pack_round_trip_fixture_documents() {
    round_trips(&Fem3dSnapshotValue::default());
    round_trips(&cantilever_doc());
    round_trips(&truss_doc());
    round_trips(&solid_slab_doc());
    let mut with_combination = cantilever_doc();
    with_combination.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35)]) });
    round_trips(&with_combination);
}
#[test]
fn inference_determinism_default_and_node_extent_laws() {
    let snapshot = persist(&Fem3dSnapshotValue { nodes: vec![FemNodeValue { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: 4.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n3".into(), x: 4.0, y: 3.0, z: 2.0 }], ..Default::default() });
    let inferred = Fem3dInference::infer(&snapshot).expect("inference");
    assert_eq!(inferred, Fem3dInference::infer(&snapshot).expect("inference"));
    assert_eq!(inferred.bounds.node_count, 3);
    assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0, 0.0]);
    assert_eq!(inferred.bounds.bounding_box.max, [4.0, 3.0, 2.0]);
    retire(snapshot);
    let empty = Fem3dSnapshot::default();
    assert_eq!(Fem3dInference::infer(&empty).expect("inference"), Fem3dInference::default());
    assert_eq!(compute_fem3d_bounds(&empty), Fem3dBounds::default());
    retire(empty);
}
#[test]
fn bounds_match_hand_built_node_extent() {
    let snapshot = persist(&extent_doc());
    let bounds = compute_fem3d_bounds(&snapshot);
    assert_eq!(bounds, compute_fem3d_bounds(&snapshot));
    assert_eq!(bounds.bounding_box.min, [-2.0, 1.0, -3.0]);
    assert_eq!(bounds.bounding_box.max, [5.0, 7.5, 6.0]);
    assert_eq!(bounds.node_count, 3);
    assert_eq!(bounds.element_count, 1);
    retire(snapshot);
}
