//! 🧪️ `setActiveExample` — the declared-state-diff laws: no edit when the document already IS the
//! requested example, and an exact replay in both directions between the two bundled examples.

use super::*;
use crate::mutations::apply_bitmap_mutation;

fn replay(current: &BitmapSnapshot, example_id: &str) -> BitmapSnapshot {
    let next = example_snapshot(example_id).expect("a bundled example");
    let mut snapshot = current.clone();
    for mutation in replace_document_operations(current, &next) {
        apply_bitmap_mutation(&mut snapshot, &mutation).unwrap_or_else(|error| panic!("replaying '{example_id}' applies: {error}"));
    }
    snapshot
}

#[test]
fn re_selecting_the_document_s_own_example_writes_no_edit() {
    let rooms = crate::examples::rooms_16::snapshot();
    assert!(replace_document_operations(&rooms, &rooms).is_empty(), "the boot announcement must not journal a phantom edit");
    let flowers = crate::examples::flowers_24::snapshot();
    assert!(replace_document_operations(&flowers, &flowers).is_empty());
}

#[test]
fn switching_between_the_two_examples_replays_each_one_exactly() {
    let rooms = crate::examples::rooms_16::snapshot();
    let flowers = crate::examples::flowers_24::snapshot();
    assert_eq!(replay(&rooms, crate::examples::flowers_24::ID), flowers, "rooms → flowers grows the palette before it repaints");
    assert_eq!(replay(&flowers, crate::examples::rooms_16::ID), rooms, "flowers → rooms repaints before it shrinks the palette");
}

#[test]
fn a_palette_shrink_is_ordered_after_the_repaint_that_frees_it() {
    let flowers = crate::examples::flowers_24::snapshot();
    let rooms = crate::examples::rooms_16::snapshot();
    let mutations = replace_document_operations(&flowers, &rooms);
    let kind = |mutation: &BitmapMutation| protocol::SemanticMutation::semantics(mutation).kind;
    let repaint = mutations.iter().position(|mutation| kind(mutation) == "set-input-pixels").expect("the buffer is rewritten");
    let shrink = mutations.iter().position(|mutation| kind(mutation) == "remove-palette-color").expect("the fourth colour is dropped");
    assert!(repaint < shrink, "removing a colour that is still painted is fatal");
    let release = mutations.iter().position(|mutation| kind(mutation) == "unpin-pixel").expect("the flower pin is released");
    assert!(release < shrink, "removing a colour that is still pinned is fatal");
}

#[test]
fn an_unknown_example_id_is_a_no_op_rather_than_a_fault() {
    assert!(example_snapshot("no-such-example").is_none());
    assert_eq!(BITMAP_EXAMPLE_BOOT_ID, crate::examples::rooms_16::ID);
}
