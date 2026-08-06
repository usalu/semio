//! ⚖️ FEM 2D artifact — binary operation protocol surface + laws (constitutional: spr).
//!
//! Renamed from the pre-migration `📡️protocol` crate: only `encode_op`/`decode_op` for `Fem2dOperation`
//! live here. The old crate's hand-rolled `Fem2dCommand` enum does NOT move here — it is rebuilt by
//! `app_commands!` in the app's `🦀️component.rs` (see `crate::apps::fem2d::Fem2dCommand`).

use crate::artifacts::fem2d::op::Fem2dOperation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


/// 📦️ Encodes a `Fem2dOperation` to its binary command form.
pub fn encode_op(operation: &Fem2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Fem2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem2dOperation, protocol::ProtocolError> {
    Fem2dOperation::decode_op(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
    use store::{create_document_envelope, DocumentCommand};

    fn simply_supported_beam_doc() -> crate::artifacts::fem2d::Fem2dDocument {
        crate::artifacts::fem2d::Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 📌️ LAW: the pre-migration operation wire format, byte for byte. The hex was dumped from the old
    /// `📡️protocol` crate before the seven-crate merge (ticket
    /// `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-before-2d.txt`) — the round-trip laws above are self-consistent and would happily
    /// pass on a silently rewritten format, so this pin is the only real proof.
    #[test]
    fn operation_bytes_match_the_pre_migration_baseline() {
        let operation = Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } };
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), "01100001000e0d0300040501040202050000000000002440");
    }

    #[test]
    fn fem2d_document_text_round_trips_through_the_store() {
        let mut store = crate::artifacts::fem2d::op::Fem2dStore::new(create_document_envelope(crate::artifacts::fem2d::FEM_2D_SCHEMA, "fem2d", crate::artifacts::fem2d::engine::empty_fem2d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Fem2dOperation::SetDocument { document: simply_supported_beam_doc() }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
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
        let operation = Fem2dOperation::SetAnalysisSettings {
            settings: crate::artifacts::fem2d::FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 },
        };
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }
}

