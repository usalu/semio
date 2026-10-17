//! 🧬️ Drawing snapshot schema — artifact-lane fields only.

use crate::{DrawingArtboard, DrawingImageAsset, DrawingLayerNode, DRAWING_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

#[path = "🔎️lookup/🦀️.rs"]
pub mod lookup;

//#region 🔖️Snapshot
/// 📸️ Persisted drawing document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[value(retire_with = "crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_drawing_snapshot")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "drawing.drawing", layout = "lines")]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingSnapshot {
    #[state(artifact)]
    pub schema: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    pub id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub title: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    #[state(artifact)]
    #[dsl(statements, block)]
    pub layers: semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "semio_framework_value::paged::PagedMap::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "semio_framework_value::paged::PagedMap::is_empty"))]
    pub assets: semio_framework_value::paged::PagedMap<DrawingImageAsset, {usize::MAX}>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub artboard: Option<DrawingArtboard>,
}
// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` impls for `DrawingSnapshot` relocated to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️.rs` (design.md §1 CORRECTION: the native codec
// is one bidirectional thing per type and sits unsplit under `🚪️io`; this file keeps types + pure
// transforms only, per design.md rule 3).

impl Default for DrawingSnapshot {
    fn default() -> Self {
        Self { schema: DRAWING_DOCUMENT_SCHEMA.into(), id: Default::default(), title: None, layers: semio_framework_value::list::PagedList::new(), assets: semio_framework_value::paged::PagedMap::default(), artboard: Some(DrawingArtboard { width: 1024.0, height: 1024.0 }) }
    }
}

/// ♻️ Retires every decoded domain field through its incremental owned-value cursor.
pub fn retire_decoded_drawing_snapshot(value: DrawingSnapshot) {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    let birth = RetainedCloneGrant::one_capacity_turn(semio_framework_value::retirement::owned_retirement_birth_bytes::<DrawingSnapshot>(), usize::MAX);
    let mut cursor = match semio_framework_value::retirement::admit_owned_retirement(value, birth) {
        Ok((cursor, _)) => cursor,
        Err((error, _)) => panic!("Draw owner retirement refused its exact birth grant: {error:?}"),
    };
    loop {
        let copy = cursor.next_copy_byte_demand().expect("Draw owner retirement copy demand");
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: cursor.next_capacity_byte_demand(copy).expect("Draw owner retirement capacity demand"),
            maximum_release_bytes: cursor.next_release_byte_demand().expect("Draw owner retirement release demand"),
            maximum_depth: cursor.next_depth_demand().expect("Draw owner retirement depth demand").max(1),
        };
        if matches!(cursor.close_step(grant).expect("Draw owner retirement"), RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(cursor.terminal_is_empty());
}
//#endregion 🔖️Snapshot

