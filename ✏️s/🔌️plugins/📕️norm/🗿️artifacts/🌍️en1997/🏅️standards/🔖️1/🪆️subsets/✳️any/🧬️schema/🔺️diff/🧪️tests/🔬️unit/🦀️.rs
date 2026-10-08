//! 🧪️ Keyed-diff algebra of En1997: same-key coalescing, position tracking and the negative diff.

use super::En1997Diff;
use crate::mutations::change_footing_embedment::ChangeFootingEmbedment;
use crate::mutations::change_footing_width::ChangeFootingWidth;
use crate::mutations::insert_footing::InsertFooting;
use crate::mutations::remove_footing::RemoveFooting;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

fn base() -> En1997Snapshot {
    let mut base = En1997Snapshot::default();
    let mut extra = base.footings[0].clone();
    extra.id = "footing-extra".into();
    base.footings.push(extra);
    base
}

fn diff_of(mutation: &En1997Mutation, base: &En1997Snapshot) -> En1997Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

fn fresh(base: &En1997Snapshot) -> crate::SpreadFoundation {
    let mut footing = base.footings[0].clone();
    footing.id = "footing-created".into();
    footing
}

async fn law(base: &En1997Snapshot, first: &En1997Mutation, second: impl Fn(&En1997Snapshot) -> En1997Mutation) -> En1997Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff(&d1, base).expect("first");
    let d2 = diff_of(&second(&mid), &mid);
    let mut sum = d1.clone();
    sum.absorb(d2.clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(base, d1, d2).await;
    sum
}

#[semio_framework_async_macros::async_test]
async fn patches_on_one_row_coalesce_into_one_patch() {
    let base = base();
    let id = base.footings[0].id.clone();
    let sum = law(&base, &En1997Mutation::ChangeFootingWidth(ChangeFootingWidth { id: id.clone(), new_width: 2.9 }), |_| En1997Mutation::ChangeFootingEmbedment(ChangeFootingEmbedment { id: id.clone(), new_embedment: 1.7 })).await;
    let rows = sum.footings.expect("footings");
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].patch.width, Some(2.9));
    assert_eq!(rows.modified[0].patch.embedment, Some(1.7));
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_cancels_and_a_patch_on_an_added_row_folds_into_it() {
    let base = base();
    let created = fresh(&base);
    let sum = law(&base, &En1997Mutation::InsertFooting(InsertFooting { index: 1, footing: created.clone() }), |_| En1997Mutation::RemoveFooting(RemoveFooting { index: 1 })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    let sum = law(&base, &En1997Mutation::InsertFooting(InsertFooting { index: 0, footing: created.clone() }), |_| En1997Mutation::ChangeFootingWidth(ChangeFootingWidth { id: created.id.clone(), new_width: 3.3 })).await;
    let rows = sum.footings.expect("footings");
    assert!(rows.modified.is_empty());
    assert_eq!(rows.inserted[0].row.width, 3.3);
    assert_eq!(rows.inserted[0].index, 0);
}

#[semio_framework_async_macros::async_test]
async fn remove_then_insert_replaces_the_row_in_place() {
    let base = base();
    let removed = base.footings[0].clone();
    let sum = law(&base, &En1997Mutation::RemoveFooting(RemoveFooting { index: 0 }), |_| En1997Mutation::InsertFooting(InsertFooting { index: 0, footing: removed.clone() })).await;
    assert_eq!(protocol::apply_diff(&sum, &base).expect("replace"), base);
}

#[semio_framework_async_macros::async_test]
async fn inverse_follows_the_state() {
    let base = base();
    let inserted = diff_of(&En1997Mutation::InsertFooting(InsertFooting { index: 0, footing: fresh(&base) }), &base);
    let after = protocol::apply_diff(&inserted, &base).expect("insert");
    assert_eq!(protocol::apply_diff(&inserted.inverse(&base), &after).expect("inverse"), base);
    let removed = diff_of(&En1997Mutation::RemoveFooting(RemoveFooting { index: 0 }), &base);
    let after = protocol::apply_diff(&removed, &base).expect("remove");
    assert_eq!(protocol::apply_diff(&removed.inverse(&base), &after).expect("inverse"), base);
}
