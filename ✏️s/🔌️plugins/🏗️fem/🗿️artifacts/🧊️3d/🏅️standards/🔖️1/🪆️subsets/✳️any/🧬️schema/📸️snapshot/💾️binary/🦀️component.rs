//! 📦️ FEM 3D artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::fem3d::Fem3dSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Fem3dSnapshot` to its binary pack form.
pub async fn encode(document: &Fem3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Fem3dSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<Fem3dSnapshot, PackError> {
    <Fem3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem3d::{FemAnalysisSettings, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
    use std::collections::BTreeMap;

    // #region 🔖️Fixtures
    async fn cantilever_fixture() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 3.0, y: 0.0, z: 0.0 }],
            elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.00000060 }],
            solids: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -5000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    async fn truss_fixture() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "n3".into(), x: 1.0, y: 1.0, z: 2.0 }, FemNode { id: "n4".into(), x: 1.0, y: -1.0, z: 0.0 }],
            elements: vec![
                FemElement::Bar { id: "b1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b3".into(), start: "n4".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
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

    async fn solid_slab_doc() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "sc0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "sc1".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "sc2".into(), x: 2.0, y: 1.0, z: 0.0 }, FemNode { id: "sc3".into(), x: 0.0, y: 1.0, z: 0.0 }],
            elements: vec![],
            materials: vec![FemMaterial { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
            sections: vec![],
            solids: vec![FemSolid { id: "sol1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 1.0, material_id: "concrete".into() }],
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

    #[semio_framework_async_macros::async_test]
    async fn fem3d_pack_agrees_with_dsl_for_bundled_default_example() {
        let document = crate::artifacts::fem3d::dsl::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).expect("parse default example");
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn fem3d_pack_agrees_with_dsl_for_fixture_documents() {
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&Fem3dSnapshot::default());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&cantilever_fixture());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&truss_fixture());
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&solid_slab_doc());
        let mut with_combination = cantilever_fixture();
        with_combination.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35)]) });
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&with_combination);
    }
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }
    #[semio_framework_async_macros::async_test]
    async fn verify_protocol_bytes_against_encoded_pack() {
        use crate::artifacts::fem3d::Fem3dSnapshot;
        let document = Fem3dSnapshot::default();
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
}
