//! ⚖️ FEM 2D artifact — binary operation protocol surface + laws (constitutional: spr).
//!
//! Renamed from the pre-migration `📡️protocol` crate: only `encode_op`/`decode_op` for `Fem2dMutation`
//! live here. The old crate's hand-rolled `Fem2dCommand` enum does NOT move here — it is rebuilt by
//! `app_commands!` in the app's `🦀️.rs` (see `crate::editor::fem2d::Fem2dCommand`).

use crate::artifacts::fem2d::schema::mutations::text::Fem2dMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Fem2dMutation` to its binary command form.
pub fn encode_op(operation: &Fem2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Fem2dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem2dMutation, protocol::ProtocolError> {
    Fem2dMutation::decode_op(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem2d::mutations::update_analysis_settings;
    use crate::artifacts::fem2d::schema;
    use crate::artifacts::fem2d::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
    use store::{create_document_envelope, ArtifactCommand};

    fn simply_supported_beam_doc() -> crate::artifacts::fem2d::Fem2dSnapshot {
        crate::artifacts::fem2d::Fem2dSnapshot {
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
        let operation = Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 🧬️ The whole-document `SetSnapshot` mutation that used to seed this store round-trip test is
    /// banned outright (`📓️taxonomy.md`'s forbidden vocabulary — no replacement mutation). Builds the
    /// same `simply_supported_beam_doc` content through a real sequence of semantic mutations instead,
    /// still exercising every collection kind (id-keyed create + a nested load) for the text/pack codecs.
    #[semio_framework_async_macros::async_test]
    async fn fem2d_document_text_round_trips_through_the_store() {
        let fixture = simply_supported_beam_doc();
        let mut store =
            semio_framework_plugin::resolve_ready(crate::artifacts::fem2d::schema::mutations::Fem2dStore::new(create_document_envelope(crate::artifacts::fem2d::FEM_2D_SCHEMA, "fem2d", schema::empty_fem2d_snapshot(), None))).expect("valid store");
        let mutations = vec![
            Fem2dMutation::CreateMaterial(crate::artifacts::fem2d::schema::mutations::create_material::mutation::CreateMaterial { material: fixture.materials[0].clone() }),
            Fem2dMutation::CreateSection(crate::artifacts::fem2d::schema::mutations::create_section::mutation::CreateSection { section: fixture.sections[0].clone() }),
            Fem2dMutation::CreateNode(crate::artifacts::fem2d::schema::mutations::create_node::mutation::CreateNode { node: fixture.nodes[0].clone() }),
            Fem2dMutation::CreateNode(crate::artifacts::fem2d::schema::mutations::create_node::mutation::CreateNode { node: fixture.nodes[1].clone() }),
            Fem2dMutation::CreateElement(crate::artifacts::fem2d::schema::mutations::create_element::mutation::CreateElement { element: Box::new(fixture.elements[0].clone()) }),
            Fem2dMutation::CreateSupport(crate::artifacts::fem2d::schema::mutations::create_support::mutation::CreateSupport { support: fixture.supports[0].clone() }),
            Fem2dMutation::CreateSupport(crate::artifacts::fem2d::schema::mutations::create_support::mutation::CreateSupport { support: fixture.supports[1].clone() }),
            Fem2dMutation::CreateLoadCase(crate::artifacts::fem2d::schema::mutations::create_load_case::mutation::CreateLoadCase { load_case: fixture.load_cases[0].clone() }),
        ];
        store.dispatch(ArtifactCommand::Apply { mutations, description: None }).await.expect("apply");
        assert_eq!(semio_framework_plugin::resolve_ready(store.snapshot()).expect("snapshot"), fixture);
        semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&store).await;
        semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&store).await;
    }
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;
    use crate::artifacts::fem2d::mutations::update_analysis_settings;

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings: crate::artifacts::fem2d::FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }
}
