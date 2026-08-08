//! 🎒️ CAD artifact — the binary document surface: `encode`/`decode` over the derive-generated
//! `store::DocumentPack`, and the law that pack and dsl are two projections of the same `CadProjection`.

use crate::artifacts::cad::CadProjection;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `CadProjection` to its binary pack form.
pub fn encode(document: &CadProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `CadProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<CadProjection, PackError> {
    <CadProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::testkit::{sample_geometry_without_anchors, sample_object, sample_scene_with};

    fn sample_scene() -> CadProjection {
        sample_scene_with(sample_geometry_without_anchors())
    }

    #[test]
    fn cad_scene_round_trips_through_pack() {
        store::test_support::assert_dsl_pack_equivalence(&sample_scene());
        let bytes = encode(&sample_scene());
        assert_eq!(decode(&bytes).expect("decode"), sample_scene());
    }

    #[test]
    fn cad_scene_with_all_geometry_panes_round_trips_through_pack() {
        let mut scene = sample_scene();
        scene.building_geometry = Some(sample_geometry_without_anchors());
        scene.energy_geometry = Some(sample_geometry_without_anchors());
        scene.structure_classic_geometry = Some(sample_geometry_without_anchors());
        store::test_support::assert_dsl_pack_equivalence(&scene);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `CadMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing dsl/pack round-trip laws (same pattern as `mathematical_pack`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::cad::op::CadMutation;
        use crate::artifacts::cad::{empty_cad_projection, CadPaneId, CAD_DOCUMENT_SCHEMA};
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<CadProjection, CadMutation> = DocumentStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad-demo", empty_cad_projection(), None));
        store.dispatch(DocumentCommand::Apply { mutations: vec![CadMutation::AddObject { pane: CadPaneId::Shape, object: sample_object("object-1") }], description: None }).expect("apply");
        let edit: &Edit<CadMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<CadProjection, CadMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
