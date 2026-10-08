//! 🧪️ `create-frame` fixture — `🔲️inserts`.
//!
//! Proves the nested frame insert honours `index` AND registers the frame on the named layer.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕create-frame/🔲️inserts/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕create-frame/🔲️inserts/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕create-frame/🔲️inserts/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕create-frame/🔲️inserts/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕create-frame/🔲️inserts/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("create-frame/inserts-a-rect-frame-at-index-1: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("create-frame/inserts-a-rect-frame-at-index-1: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("create-frame/inserts-a-rect-frame-at-index-1: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("create-frame applies to its committed before-snapshot")
}

/// ▶️ `create-frame` inserts at the payload index inside the page, but APPENDS the id to the layer's `object_ids`.
#[semio_framework_async_macros::async_test]
async fn inserts_at_the_requested_index_and_joins_the_layer() {
    let after = applied();
    let page = &after.pages[0];
    assert_eq!(page.frames.iter().map(|frame| frame.id()).collect::<Vec<_>>(), vec!["frame-rect", "frame-badge", "frame-text"], "create-frame must insert at the payload's index, not append");
    assert_eq!(page.layers[0].object_ids, vec!["frame-rect".to_string(), "frame-text".to_string(), "frame-badge".to_string()], "create-frame appends the new id to the named layer's object list");
    assert_eq!(after.pages[1].frames.len(), 0, "create-frame must not add the frame to any other page");
    assert_eq!(after, expected_after(), "create-frame/inserts-a-rect-frame-at-index-1: applied state differs from the committed after-snapshot");
}

/// ↩️ `create-frame` always inverts to `delete-frame` on the same page — it never inspects BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_frame_from_the_same_page() {
    let base = before();
    let inverse = mutation().inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "create-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeleteFrame(step) => {
            assert_eq!(step.page_id, "page-1", "the inverse must address the page create-frame wrote to");
            assert_eq!(step.frame_id, "frame-badge", "the inverse must delete the frame id create-frame minted");
        }
        other => panic!("create-frame must invert to delete-frame, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = protocol::apply_diff(step.diff(&snapshot).diff(), &snapshot).expect("create-frame/inserts-a-rect-frame-at-index-1: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-frame/inserts-a-rect-frame-at-index-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-frame/inserts-a-rect-frame-at-index-1: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-frame/inserts-a-rect-frame-at-index-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-frame`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-frame/inserts-a-rect-frame-at-index-1: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-frame/inserts-a-rect-frame-at-index-1: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("create-frame fills the pages delta").modified[0].patch;
    let [added] = patch.frames.inserted.as_slice() else { panic!("create-frame fills the page patch's `frames.inserted` with exactly one row") };
    assert_eq!(added.row.id(), "frame-badge", "the inserted row carries the payload frame");
    assert_eq!(added.index, 1, "the inserted row carries its index in the resulting frame list");
    assert_eq!(added.row.layer_id(), "layer-1", "the inserted row carries the layer it registers on");
    assert!(patch.frames.removed.is_empty() && patch.frames.moved.is_empty() && patch.frames.modified.is_empty() && patch.layers.is_empty(), "create-frame emits only the `frames.inserted` row");
}

/// 🔺️ The sparse delta `create-frame` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the frame insert rides inside `pages.modified[0].patch.frames.inserted` — a NESTED page patch, never a top-level frames collection.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-frame/inserts-a-rect-frame-at-index-1: create-frame must emit the insert as a nested `frames.inserted` row of a page patch");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[value(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::LayoutDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-frame/inserts-a-rect-frame-at-index-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-frame` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::LayoutDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes into the artifact's diff type");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-frame/inserts-a-rect-frame-at-index-1: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
