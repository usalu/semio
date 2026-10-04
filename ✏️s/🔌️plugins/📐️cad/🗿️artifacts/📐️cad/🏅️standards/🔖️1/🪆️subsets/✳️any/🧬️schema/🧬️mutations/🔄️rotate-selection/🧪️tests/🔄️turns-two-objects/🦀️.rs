//! 🧪️ `rotate-selection` law — `🔄️turns-two-objects`. Rust-constructed for the reason `drag-selection`'s law states.

use crate::mutations::{rotate_objects::inverse::CAD_IDENTITY_ORIENTATION, rotate_selection::RotateSelection, CadMutation};
use crate::sample_scene_fixture::{materialized_objects, materialized_shape_scene, sample_object};
use crate::CadPaneId;
use protocol::Mutation;
use protocol::MutationDiff;
use protocol::SemanticMutation;
use semio_framework_diagnostic::Severity;

fn base() -> crate::CadSnapshot {
    materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0]), sample_object("object-b", [4.0, 0.0, 0.0])])
}

fn turn(targets: &[&str], axis: [f64; 3], angle: f64) -> CadMutation {
    CadMutation::RotateSelection(RotateSelection { pane: CadPaneId::Shape, targets: targets.iter().map(|id| id.to_string()).collect(), axis, angle })
}

fn orientation(document: &crate::CadSnapshot, id: &str) -> [f64; 4] {
    materialized_objects(document, CadPaneId::Shape).into_iter().find(|object| object.id == id).expect("object survives").orientation.unwrap_or(CAD_IDENTITY_ORIENTATION)
}

fn close(left: [f64; 4], right: [f64; 4]) -> bool {
    left.iter().zip(right).all(|(a, b)| (a - b).abs() < 1e-12)
}

/// ▶️ Both objects turn in place a quarter about +Z (the axis length is ignored); origins stay.
#[semio_framework_async_macros::async_test]
async fn turns_every_addressed_object_in_place() {
    let base = base();
    let after = turn(&["object-a", "object-b"], [0.0, 0.0, 2.0], std::f64::consts::FRAC_PI_2).diff(&base).diff().apply(&base).expect("turn applies");
    let quarter = [0.0, 0.0, std::f64::consts::FRAC_PI_4.sin(), std::f64::consts::FRAC_PI_4.cos()];
    assert!(close(orientation(&after, "object-a"), quarter), "{:?}", orientation(&after, "object-a"));
    assert!(close(orientation(&after, "object-b"), quarter));
    assert_eq!(materialized_objects(&after, CadPaneId::Shape)[1].origin, [4.0, 0.0, 0.0], "a turn in place keeps the origin");
}

/// 🔁️ Two quarter turns compose to a half turn: the leaf reads the BASE orientation, never an absolute pose.
#[semio_framework_async_macros::async_test]
async fn composes_onto_the_base_orientation() {
    let base = base();
    let once = turn(&["object-a"], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2).diff(&base).diff().apply(&base).expect("first turn applies");
    let twice = turn(&["object-a"], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2).diff(&once).diff().apply(&once).expect("second turn applies");
    assert!(close(orientation(&twice, "object-a"), [0.0, 0.0, 1.0, 0.0]), "{:?}", orientation(&twice, "object-a"));
}

/// ↩️ ONE absolute `rotate-objects` restores the exact base.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_exact_base() {
    let base = base();
    let mutation = turn(&["object-a", "object-b"], [1.0, 1.0, 0.0], 0.7);
    let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1);
    assert!(matches!(&inverse[0], CadMutation::RotateObjects(step) if step.placements.len() == 2), "{inverse:?}");
    let mut snapshot = mutation.diff(&base).diff().apply(&base).expect("turn applies");
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("inverse applies");
    }
    assert_eq!(snapshot, base, "rotate-selection inverse did not restore the before-snapshot");
}

/// 🚫️ A zero axis is the Fatal `axis-nonzero`, a zero angle a no-op, a missing pane object partial or target-missing.
#[semio_framework_async_macros::async_test]
async fn refuses_by_the_outcome_vocabulary() {
    let base = base();
    let degenerate = turn(&["object-a"], [0.0, 0.0, 0.0], 1.0).diff(&base);
    assert!(degenerate.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.level == Severity::Fatal && message.message.starts_with("axis-nonzero")), "{:?}", degenerate.messages());
    store::os_spr::protocol_laws::assert_fatal_never_applies(&degenerate).await;
    assert!(turn(&["object-a"], [0.0, 0.0, 1.0], 0.0).diff(&base).messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    assert!(turn(&["object-a", "ghost"], [0.0, 0.0, 1.0], 1.0).diff(&base).messages().iter().any(|message| message.code.0 == "mutation.partial"));
    assert!(turn(&["ghost"], [0.0, 0.0, 1.0], 1.0).diff(&base).messages().iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == Severity::Error));
}

/// 🏷️ The row shows the angle in degrees, in English and German.
#[test]
fn labels_the_row_in_degrees() {
    let label = turn(&["object-a"], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2).label();
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Rotate 1 object by 90°");
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "1 Objekt um 90° drehen");
}
