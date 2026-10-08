//! 🧪️ Positional-diff algebra of En1994: index translation through absorb, coalescing, the negative diff and the state delta.

use super::En1994Diff;
use crate::mutations::change_beam_action_q_area_pa::ChangeBeamActionQAreaPa;
use crate::mutations::change_beam_span_m::ChangeBeamSpanM;
use crate::mutations::insert_beam::InsertBeam;
use crate::mutations::remove_beam::RemoveBeam;
use crate::mutations::En1994Mutation;
use crate::En1994Snapshot;
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

fn base() -> En1994Snapshot {
    let mut base = En1994Snapshot::default();
    let mut extra = base.beams[0].clone();
    extra.id = "beam-extra".into();
    base.beams.push(extra);
    base
}

fn diff_of(mutation: &En1994Mutation, base: &En1994Snapshot) -> En1994Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

fn span(index: usize, value: f64) -> En1994Mutation {
    En1994Mutation::ChangeBeamSpanM(ChangeBeamSpanM { index, new_span_m: value })
}

fn fresh(base: &En1994Snapshot) -> crate::CompositeBeam {
    let mut beam = base.beams[0].clone();
    beam.id = "beam-created".into();
    beam
}

async fn law(base: &En1994Snapshot, first: &En1994Mutation, second: impl Fn(&En1994Snapshot) -> En1994Mutation) -> En1994Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff(&d1, base).expect("first");
    let d2 = diff_of(&second(&mid), &mid);
    let mut sum = d1.clone();
    sum.absorb(d2.clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(base, d1, d2).await;
    sum
}

#[semio_framework_async_macros::async_test]
async fn a_row_inserted_before_a_patched_row_shifts_the_patch_to_the_base_position() {
    let base = base();
    let sum = law(&base, &En1994Mutation::InsertBeam(InsertBeam { index: 0, beam: fresh(&base) }), |_| span(1, 31.0)).await;
    let rows = sum.beams.expect("beams");
    assert_eq!(rows.inserted.len(), 1);
    assert_eq!(rows.inserted[0].index, 0);
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].index, 0);
}

#[semio_framework_async_macros::async_test]
async fn a_removed_row_shifts_later_patches_to_their_base_position() {
    let base = base();
    let sum = law(&base, &En1994Mutation::RemoveBeam(RemoveBeam { index: 0 }), |_| span(0, 33.0)).await;
    let rows = sum.beams.expect("beams");
    assert_eq!(rows.removed, vec![0]);
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].index, 1);
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_cancels_and_a_patch_on_an_inserted_row_folds_into_it() {
    let base = base();
    let sum = law(&base, &En1994Mutation::InsertBeam(InsertBeam { index: 1, beam: fresh(&base) }), |_| En1994Mutation::RemoveBeam(RemoveBeam { index: 1 })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    let sum = law(&base, &En1994Mutation::InsertBeam(InsertBeam { index: 1, beam: fresh(&base) }), |_| span(1, 41.0)).await;
    let rows = sum.beams.expect("beams");
    assert!(rows.modified.is_empty());
    assert_eq!(rows.inserted[0].row.span_m, 41.0);
}

#[semio_framework_async_macros::async_test]
async fn action_patches_coalesce_by_position() {
    let base = base();
    assert!(!base.beams[0].actions.is_empty());
    let set = |value: f64| En1994Mutation::ChangeBeamActionQAreaPa(ChangeBeamActionQAreaPa { index: 0, action_index: 0, new_q_area_pa: value });
    let sum = law(&base, &set(1000.0), |_| set(2000.0)).await;
    let patches = sum.beams.expect("beams").modified;
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].actions.as_ref().expect("actions").modified.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_and_between_follow_the_state() {
    let base = base();
    let inserted = diff_of(&En1994Mutation::InsertBeam(InsertBeam { index: 0, beam: fresh(&base) }), &base);
    let after = protocol::apply_diff(&inserted, &base).expect("insert");
    assert_eq!(protocol::apply_diff(&inserted.inverse(&base), &after).expect("inverse"), base);
    let removed = diff_of(&En1994Mutation::RemoveBeam(RemoveBeam { index: 0 }), &base);
    let after = protocol::apply_diff(&removed, &base).expect("remove");
    assert_eq!(protocol::apply_diff(&removed.inverse(&base), &after).expect("inverse"), base);
    let mut other = base.clone();
    for mutation in [En1994Mutation::InsertBeam(InsertBeam { index: 1, beam: fresh(&base) }), span(0, 29.0)] {
        other = protocol::apply_diff(&diff_of(&mutation, &other), &other).expect("apply");
    }
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<En1994Snapshot, En1994Diff>(&base, &other).await;
}
