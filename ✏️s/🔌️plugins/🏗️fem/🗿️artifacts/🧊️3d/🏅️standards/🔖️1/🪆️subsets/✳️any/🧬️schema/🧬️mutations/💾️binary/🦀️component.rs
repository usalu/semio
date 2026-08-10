//! ⚖️ FEM 3D artifact — binary operation protocol surface + laws (constitutional: spr, renamed from the
//! old `📡️protocol` crate — the old crate's hand-rolled `Fem3dCommand` enum moved to `app_commands!` in
//! `crate::apps::fem3d`; only the `Fem3dMutation` codec pair survives here).

use crate::artifacts::fem3d::schema::mutations::text::Fem3dMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


/// 📦️ Encodes a `Fem3dMutation` to its binary command form.
pub fn encode_op(operation: &Fem3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Fem3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem3dMutation, protocol::ProtocolError> {
    Fem3dMutation::decode_op(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem3d::engine;
    use crate::artifacts::fem3d::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
    use store::{create_document_envelope, DocumentCommand};

    fn cantilever_fixture() -> crate::artifacts::fem3d::Fem3dSnapshot {
        crate::artifacts::fem3d::Fem3dSnapshot {
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

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Fem3dMutation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 📌️ LAW: the pre-migration operation wire format, byte for byte. The hex was dumped from the old
    /// `📡️protocol` crate before the seven-crate merge (ticket
    /// `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-before-3d.txt`) — the round-trip laws above are self-consistent and would happily
    /// pass on a silently rewritten format, so this pin is the only real proof.
    #[test]
    fn operation_bytes_match_the_pre_migration_baseline() {
        let operation = Fem3dMutation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), "01100001000e0d0300040501040202050000000000002440");
    }

    #[test]
    fn fem3d_document_text_round_trips_through_the_store() {
        let mut store = crate::artifacts::fem3d::schema::mutations::Fem3dStore::new(create_document_envelope(crate::artifacts::fem3d::FEM_3D_SCHEMA, "fem3d", engine::empty_fem3d_snapshot(), None));
        store.dispatch(DocumentCommand::Apply { mutations: vec![Fem3dMutation::SetSnapshot { snapshot: cantilever_fixture() }], description: None }).expect("apply");
        semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&store);
        semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = Fem3dMutation::SetAnalysisSettings {
            settings: crate::artifacts::fem3d::FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 },
        };
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }

}

