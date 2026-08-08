//! 📦️ Mathematical artifact — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack` for `MathProjection` is implemented here, next to the DSL-mirror conversions it
//! delegates through — see `crate::artifacts::mathematical::dsl`'s module doc for why this no longer needs
//! to sit next to `MathProjection` itself now that the old 7-crate orphan-rule boundary is gone.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::mathematical::dsl::{math_projection_from_dsl, math_projection_to_dsl, MathProjectionDsl};
use crate::artifacts::mathematical::MathProjection;
use store::{DocumentPack, PackDecodeOptions, PackEncodeOptions, PackError};

//#region 🔖️Pack
impl DocumentPack for MathProjection {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        math_projection_to_dsl(self).encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let dsl_projection = MathProjectionDsl::decode_pack_with(bytes, options)?;
        math_projection_from_dsl(dsl_projection).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}
//#endregion 🔖️Pack

//#region 🔖️PackBinary
/// 📦️ Encodes a `MathProjection` to its binary pack form.
pub fn encode(projection: &MathProjection) -> Vec<u8> {
    DocumentPack::encode_pack(projection)
}

/// 📖️ Decodes a `MathProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<MathProjection, PackError> {
    <MathProjection as DocumentPack>::decode_pack(bytes)
}
//#endregion 🔖️PackBinary

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mathematical::{MathGeometry, MathGraph};

    #[test]
    fn math_projection_dsl_pack_equivalence_default() {
        store::test_support::assert_dsl_pack_equivalence(&MathProjection::default());
    }

    #[test]
    fn math_projection_dsl_pack_equivalence_with_seed_and_empty_collections() {
        let mut graph = MathGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..MathGraph::default() };
        graph.nodes.clear();
        graph.edges.clear();
        let projection = MathProjection { graph, geometry: MathGeometry { points: Vec::new() } };
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `MathMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::mathematical::op::MathMutation;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<MathProjection, MathMutation> = DocumentStore::new(create_document_envelope("semio.mathematical/v1", "math-demo", MathProjection::default(), None));
        let graph = MathGraph { algorithm: "components".into(), ..MathGraph::default() };
        store.dispatch(DocumentCommand::Apply { mutations: vec![MathMutation::SetGraph { graph }], description: None }).expect("apply");
        let edit: &Edit<MathMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<MathProjection, MathMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
