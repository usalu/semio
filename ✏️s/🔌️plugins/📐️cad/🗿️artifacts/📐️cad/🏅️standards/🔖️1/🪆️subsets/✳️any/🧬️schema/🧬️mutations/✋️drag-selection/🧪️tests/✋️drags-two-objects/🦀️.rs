//! 🧪️ `drag-selection` law — `✋️drags-two-objects`.
//!
//! ⚠️ Rust-constructed like `move-objects`' law: the state this verb reads is the pane child's `local_owner`, which no
//! wire codec carries, so a committed before/after quintet would describe an empty pane. The committed wire witness
//! (`🧫️fixtures/🧬️mutations/✋️drag-selection/🧾️wire-witness`) pins the payload.

use crate::mutations::{drag_selection::DragSelection, CadMutation};
use crate::sample_scene_fixture::{materialized_objects, materialized_shape_scene, sample_object};
use crate::CadPaneId;
use protocol::{Mutation, MutationDiff, SemanticMutation, Severity};

fn base() -> crate::CadSnapshot {
    materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0]), sample_object("object-b", [4.0, 0.0, 0.0]), sample_object("object-c", [9.0, 9.0, 0.0])])
}

fn drag(targets: &[&str], offset: [f64; 3]) -> CadMutation {
    CadMutation::DragSelection(DragSelection { pane: CadPaneId::Shape, targets: targets.iter().map(|id| id.to_string()).collect(), offset })
}

fn origin(document: &crate::CadSnapshot, id: &str) -> [f64; 3] {
    materialized_objects(document, CadPaneId::Shape).into_iter().find(|object| object.id == id).expect("object survives").origin
}

/// ▶️ Both addressed objects move by the offset read off their BASE origins; the unaddressed one keeps its pose.
#[semio_framework_async_macros::async_test]
async fn moves_every_addressed_object_by_the_offset() {
    let base = base();
    let after = drag(&["object-a", "object-b"], [1.5, -2.0, 0.5]).diff(&base).diff().apply(&base).expect("drag applies");
    assert_eq!(origin(&after, "object-a"), [1.5, -2.0, 0.5]);
    assert_eq!(origin(&after, "object-b"), [5.5, -2.0, 0.5]);
    assert_eq!(origin(&after, "object-c"), [9.0, 9.0, 0.0], "an unaddressed object keeps its pose");
}

/// 🔁️ The leaf is parametric: replayed on a base where an object already sits elsewhere, it moves it RELATIVE to there.
#[semio_framework_async_macros::async_test]
async fn replays_relative_to_any_base() {
    let moved = materialized_shape_scene(vec![sample_object("object-a", [10.0, 10.0, 10.0]), sample_object("object-b", [4.0, 0.0, 0.0])]);
    let after = drag(&["object-a"], [1.0, 2.0, 3.0]).diff(&moved).diff().apply(&moved).expect("drag applies");
    assert_eq!(origin(&after, "object-a"), [11.0, 12.0, 13.0]);
}

/// ↩️ The inverse is ONE absolute `move-objects` carrying every pre-drag origin, and restores the exact base.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_exact_base() {
    let base = base();
    let mutation = drag(&["object-a", "object-b"], [0.1, 0.2, 0.3]);
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse.len(), 1, "a drag inverts to exactly one absolute setter");
    assert!(matches!(&inverse[0], CadMutation::MoveObjects(step) if step.placements.len() == 2), "the inverse must be one move-objects over both moved objects, got {inverse:?}");
    let mut snapshot = mutation.diff(&base).diff().apply(&base).expect("drag applies");
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("inverse applies");
    }
    assert_eq!(snapshot, base, "drag-selection inverse did not restore the before-snapshot");
    assert_eq!(origin(&snapshot, "object-b"), [4.0, 0.0, 0.0], "undo restores the materialized pose bit for bit");
}

/// ⚠️ A target the pane lacks is skipped with a `mutation.partial` Warning; the rest still moves.
#[semio_framework_async_macros::async_test]
async fn skips_a_missing_target_as_partial() {
    let base = base();
    let outcome = drag(&["object-a", "ghost"], [1.0, 0.0, 0.0]).diff(&base);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.partial" && message.level == Severity::Warning && message.target == vec!["ghost".to_string()]), "{:?}", outcome.messages());
    let after = outcome.diff().apply(&base).expect("partial drag applies");
    assert_eq!(origin(&after, "object-a"), [1.0, 0.0, 0.0]);
}

/// 🚫️ No target left, an identity offset, and a malformed payload are an Error, a no-op Warning and a Fatal.
#[semio_framework_async_macros::async_test]
async fn refuses_by_the_outcome_vocabulary() {
    let base = base();
    let missing = drag(&["ghost"], [1.0, 0.0, 0.0]).diff(&base);
    assert!(missing.messages().iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == Severity::Error), "{:?}", missing.messages());
    let still = drag(&["object-a"], [0.0, 0.0, 0.0]).diff(&base);
    assert!(still.messages().iter().any(|message| message.code.0 == "mutation.no-op" && message.level == Severity::Warning), "{:?}", still.messages());
    assert!(drag(&["object-a"], [0.0, 0.0, 0.0]).inverse(&base).is_empty(), "a no-op drag has no inverse step");
    for malformed in [drag(&[], [1.0, 0.0, 0.0]), drag(&["object-a", "object-a"], [1.0, 0.0, 0.0]), drag(&["object-a"], [f64::NAN, 0.0, 0.0])] {
        let outcome = malformed.diff(&base);
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.level == Severity::Fatal), "{malformed:?} must be Fatal, got {:?}", outcome.messages());
        store::os_spr::protocol_laws::assert_fatal_never_applies(&outcome).await;
    }
}

/// 🏷️ The history row reads the leaf's own inputs, in English and German.
#[test]
fn labels_the_row_from_its_inputs() {
    let label = drag(&["object-a", "object-b"], [1.5, -2.0, 0.0]).label();
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 2 objects by (1.5, -2, 0)");
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::De), "2 Objekte um (1,5; -2; 0) ziehen");
    assert_eq!(SemanticMutation::<crate::CadSnapshot>::target(&drag(&["object-a"], [1.0, 0.0, 0.0])), vec!["object-a".to_string()]);
}
