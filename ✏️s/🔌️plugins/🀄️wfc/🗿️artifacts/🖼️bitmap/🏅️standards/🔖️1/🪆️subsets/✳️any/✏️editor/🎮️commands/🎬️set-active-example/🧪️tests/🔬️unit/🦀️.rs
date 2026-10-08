//! 🧪️ `setActiveExample` — the load laws: an example switch is one `LoadDocument` effect and no mutation, and nothing at all
//! when the document already IS the requested example.

use super::*;
use semio_framework_plugin::HistoryView;

fn answer(current: &BitmapSnapshot, example_id: &str) -> Emit<BitmapMutation> {
    handle(&SetActiveExample { example_id: example_id.into() }, &ArtifactView::new(current, &HistoryView::empty())).expect("the handler never faults")
}

#[test]
fn re_selecting_the_document_s_own_example_answers_nothing() {
    let rooms = crate::examples::rooms_16::snapshot();
    let emit = answer(&rooms, crate::examples::rooms_16::ID);
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "the boot announcement must not journal a phantom edit");
    let flowers = crate::examples::flowers_24::snapshot();
    let emit = answer(&flowers, crate::examples::flowers_24::ID);
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty());
}

#[test]
fn switching_between_the_two_examples_is_one_load_document_effect() {
    let rooms = crate::examples::rooms_16::snapshot();
    let flowers = crate::examples::flowers_24::snapshot();
    for (current, id) in [(&rooms, crate::examples::flowers_24::ID), (&flowers, crate::examples::rooms_16::ID)] {
        let emit = answer(current, id);
        assert!(emit.artifact_mutations.is_empty(), "an example switch is not a document edit");
        assert_eq!(emit.effects.len(), 1);
        assert!(matches!(emit.effects[0], semio_framework::kernel::Effect::LoadDocument { .. }), "{id}: a document swap rides LoadDocument");
    }
}

#[test]
fn an_unknown_example_id_is_a_no_op_rather_than_a_fault() {
    assert!(example_snapshot("no-such-example").is_none());
    assert_eq!(BITMAP_EXAMPLE_BOOT_ID, crate::examples::rooms_16::ID);
    let emit = answer(&crate::examples::rooms_16::snapshot(), "no-such-example");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty());
}
