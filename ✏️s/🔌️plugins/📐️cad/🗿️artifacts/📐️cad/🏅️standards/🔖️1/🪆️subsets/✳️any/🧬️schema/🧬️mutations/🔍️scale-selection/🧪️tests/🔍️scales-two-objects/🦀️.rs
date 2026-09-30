//! 🧪️ `scale-selection` law — `🔍️scales-two-objects`. Rust-constructed for the reason `drag-selection`'s law states.

use crate::mutations::{scale_objects::inverse::CAD_IDENTITY_SCALE, scale_selection::ScaleSelection, CadMutation};
use crate::sample_scene_fixture::{materialized_objects, materialized_shape_scene, sample_object};
use crate::CadPaneId;
use protocol::{Mutation, MutationDiff, SemanticMutation, Severity};

fn base() -> crate::CadSnapshot {
    materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0]), sample_object("object-b", [4.0, 0.0, 0.0])])
}

fn scale(targets: &[&str], factors: [f64; 3]) -> CadMutation {
    CadMutation::ScaleSelection(ScaleSelection { pane: CadPaneId::Shape, targets: targets.iter().map(|id| id.to_string()).collect(), factors })
}

fn scale_of(document: &crate::CadSnapshot, id: &str) -> [f64; 3] {
    materialized_objects(document, CadPaneId::Shape).into_iter().find(|object| object.id == id).expect("object survives").scale.unwrap_or(CAD_IDENTITY_SCALE)
}

/// ▶️ Every addressed scale is multiplied by the factors, twice composing multiplicatively.
#[semio_framework_async_macros::async_test]
async fn multiplies_every_addressed_scale() {
    let base = base();
    let once = scale(&["object-a", "object-b"], [2.0, 1.5, 0.5]).diff(&base).diff().apply(&base).expect("scale applies");
    assert_eq!(scale_of(&once, "object-a"), [2.0, 1.5, 0.5]);
    let twice = scale(&["object-a"], [2.0, 2.0, 2.0]).diff(&once).diff().apply(&once).expect("second scale applies");
    assert_eq!(scale_of(&twice, "object-a"), [4.0, 3.0, 1.0]);
    assert_eq!(scale_of(&twice, "object-b"), [2.0, 1.5, 0.5], "an unaddressed object keeps its scale");
}

/// ↩️ ONE absolute `scale-objects` restores the exact base.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_exact_base() {
    let base = base();
    let mutation = scale(&["object-a", "object-b"], [3.0, 3.0, 3.0]);
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse.len(), 1);
    let mut snapshot = mutation.diff(&base).diff().apply(&base).expect("scale applies");
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("inverse applies");
    }
    assert_eq!(snapshot, base, "scale-selection inverse did not restore the before-snapshot");
}

/// 🚫️ A non-positive factor is Fatal, the unit factors a no-op, a missing object partial.
#[semio_framework_async_macros::async_test]
async fn refuses_by_the_outcome_vocabulary() {
    let base = base();
    for factors in [[0.0, 1.0, 1.0], [-1.0, 1.0, 1.0], [f64::INFINITY, 1.0, 1.0]] {
        let outcome = scale(&["object-a"], factors).diff(&base);
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.level == Severity::Fatal), "{factors:?}: {:?}", outcome.messages());
        store::os_spr::protocol_laws::assert_fatal_never_applies(&outcome).await;
    }
    assert!(scale(&["object-a"], [1.0, 1.0, 1.0]).diff(&base).messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    assert!(scale(&["object-a", "ghost"], [2.0, 2.0, 2.0]).diff(&base).messages().iter().any(|message| message.code.0 == "mutation.partial"));
}

/// 🏷️ The row reads the factors, in English and German.
#[test]
fn labels_the_row_from_its_factors() {
    let label = scale(&["object-a", "object-b"], [2.0, 2.0, 0.5]).label();
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Scale 2 objects by (2, 2, 0.5)");
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::De), "2 Objekte um (2; 2; 0,5) skalieren");
}
