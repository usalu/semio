use crate::standards::v1::subsets::any::io::text::snapshot::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemDof;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoad;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSolid;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use std::collections::BTreeMap;

// #region 🔖️Fixtures
fn cantilever_fixture() -> Fem3dSnapshot {
    Fem3dSnapshot {
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

fn truss_fixture() -> Fem3dSnapshot {
    Fem3dSnapshot {
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

fn solid_slab_doc() -> Fem3dSnapshot {
    Fem3dSnapshot {
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

#[test]
fn fem3d_dsl_round_trips_bundled_default_example() {
    let document = parse_dsl(FEM3D_EXAMPLE_TEXT).expect("parse default example");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&document);
}

#[test]
fn fem3d_dsl_round_trips_fixture_documents() {
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&Fem3dSnapshot::default());
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&cantilever_fixture());
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&truss_fixture());
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&solid_slab_doc());
    let mut with_combination = cantilever_fixture();
    with_combination.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35)]) });
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&with_combination);
}
