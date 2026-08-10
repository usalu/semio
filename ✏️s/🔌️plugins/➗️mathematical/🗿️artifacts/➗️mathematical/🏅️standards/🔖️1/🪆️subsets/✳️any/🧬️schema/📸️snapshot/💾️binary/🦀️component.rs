//! 📦️ Mathematical artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::mathematical::MathematicalSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `MathematicalSnapshot` to its binary pack form.
pub fn encode(snapshot: &MathematicalSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `MathematicalSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<MathematicalSnapshot, PackError> {
    <MathematicalSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalGraph};

    #[test]
    fn mathematical_snapshot_dsl_pack_equivalence_default() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&MathematicalSnapshot::default());
    }

    #[test]
    fn mathematical_snapshot_dsl_pack_equivalence_with_seed_and_empty_collections() {
        let mut graph = MathematicalGraph {
            algorithm: "bfs".into(),
            algorithm_seed: Some("a".into()),
            ..MathematicalGraph::default()
        };
        graph.nodes.clear();
        graph.edges.clear();
        let snapshot = MathematicalSnapshot {
            graph,
            geometry: MathematicalGeometry { points: Vec::new() },
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
    }

    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::mathematical::op::MathematicalMutation;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<MathematicalSnapshot, MathematicalMutation> =
            ArtifactStore::new(create_document_envelope("semio.mathematical/v1", "math-demo", MathematicalSnapshot::default(), None));
        let graph = MathematicalGraph {
            algorithm: "components".into(),
            ..MathematicalGraph::default()
        };
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![MathematicalMutation::SetGraph { graph }],
                description: None,
            })
            .expect("apply");
        let edit: &Edit<MathematicalMutation> = store.envelope().vcs.edits.last().expect("edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<MathematicalSnapshot, MathematicalMutation>(
            edit,
            &ArtifactId(store.envelope().id.clone()),
            &SchemaId(store.envelope().schema.clone()),
        );
    }
}
//#endregion 🧪️Tests
