//! ⚖️ FEM 3D artifact — binary operation protocol surface + laws (constitutional: spr, renamed from the
//! old `📡️protocol` crate — the old crate's hand-rolled `Fem3dCommand` enum moved to `app_commands!` in
//! `crate::editor::fem3d`; only the `Fem3dMutation` codec pair survives here).

use crate::artifacts::fem3d::schema::mutations::text::Fem3dMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
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
    use crate::artifacts::fem3d::mutations::update_analysis_settings;
    use crate::artifacts::fem3d::schema;
    use crate::artifacts::fem3d::{FemAnalysisSettings, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport};
    use store::{create_document_envelope, ArtifactCommand};

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
        let operation = Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 🧬️ The whole-document `SetSnapshot` mutation that used to seed this store round-trip test is
    /// banned outright (`📓️taxonomy.md`'s forbidden vocabulary — no replacement mutation). Builds the
    /// same `cantilever_fixture` content through a real sequence of semantic mutations instead, still
    /// exercising every collection kind (id-keyed create + a nested load) for the text/pack codecs.
    #[semio_framework_async_macros::async_test]
    async fn fem3d_document_text_round_trips_through_the_store() {
        let fixture = cantilever_fixture();
        let mut store =
            semio_framework_plugin::resolve_ready(crate::artifacts::fem3d::schema::mutations::Fem3dStore::new(create_document_envelope(crate::artifacts::fem3d::FEM_3D_SCHEMA, "fem3d", schema::empty_fem3d_snapshot(), None))).expect("valid store");
        let mutations = vec![
            Fem3dMutation::CreateMaterial(crate::artifacts::fem3d::schema::mutations::create_material::mutation::CreateMaterial { material: fixture.materials[0].clone() }),
            Fem3dMutation::CreateSection(crate::artifacts::fem3d::schema::mutations::create_section::mutation::CreateSection { section: fixture.sections[0].clone() }),
            Fem3dMutation::CreateNode(crate::artifacts::fem3d::schema::mutations::create_node::mutation::CreateNode { node: fixture.nodes[0].clone() }),
            Fem3dMutation::CreateNode(crate::artifacts::fem3d::schema::mutations::create_node::mutation::CreateNode { node: fixture.nodes[1].clone() }),
            Fem3dMutation::CreateElement(crate::artifacts::fem3d::schema::mutations::create_element::mutation::CreateElement { element: Box::new(fixture.elements[0].clone()) }),
            Fem3dMutation::CreateSupport(crate::artifacts::fem3d::schema::mutations::create_support::mutation::CreateSupport { support: fixture.supports[0].clone() }),
            Fem3dMutation::CreateLoadCase(crate::artifacts::fem3d::schema::mutations::create_load_case::mutation::CreateLoadCase { load_case: fixture.load_cases[0].clone() }),
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
    use crate::artifacts::fem3d::mutations::update_analysis_settings;

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        let operation = Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings: crate::artifacts::fem3d::FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }
}
