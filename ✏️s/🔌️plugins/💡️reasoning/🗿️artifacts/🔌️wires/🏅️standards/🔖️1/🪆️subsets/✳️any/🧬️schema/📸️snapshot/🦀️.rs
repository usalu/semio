//! 🧬️ Wires snapshot schema — artifact-lane fields only.
//!
//! `content` composes stdio's neutral `s.stdio.semio@v1/graph` subset: the board's nodes and edges live ONLY in that child
//! (design §20.15); `wires_snapshot` is the parent's identity layer (`schema`, `identities`) and `meta` the kind catalogues,
//! while the camera is owned by each concrete canvas window.
//!
//! Snapshot representation implementations live under IO; this schema owns the native artifact fields.

use crate::WiresContentChild;
use semio_framework_value::retirement::{deferred, deferred_birth_bytes, deferred_birth_bytes_for, sequence, sequence_birth_bytes, arc_custody::SharedControlledRetirement, RetireOwned, RetirementCursor};
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted wires document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresSnapshot {
    #[state(artifact)]
    pub wires_snapshot: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: WiresContentChild,
    #[state(artifact)]
    pub meta: DslValue,
}

//#endregion 🔖️Snapshot

type ContentOwner = Option<SharedControlledRetirement<SemioGraphSnapshot>>;

/// ♻️ The board content child retires as a lease of its materialized graph, so the last alias pays the graph's release in paid turns and the others only drop their count.
impl RetireOwned for WiresSnapshot {
    fn retirement(mut self) -> Box<dyn RetirementCursor> {
        let content_owner: ContentOwner = self.content.take_local_owner::<SemioGraphSnapshot>().ok().flatten().map(SharedControlledRetirement::lease);
        let Self { wires_snapshot, content, meta } = self;
        sequence(vec![deferred(wires_snapshot), deferred(content), deferred(meta), deferred(content_owner)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        sequence_birth_bytes(&[deferred_birth_bytes_for(&self.wires_snapshot), deferred_birth_bytes_for(&self.content), deferred_birth_bytes_for(&self.meta), deferred_birth_bytes::<ContentOwner>()])
    }
    fn controlled_retirement_supported() -> bool { <ContentOwner as RetireOwned>::controlled_retirement_supported() }
}
