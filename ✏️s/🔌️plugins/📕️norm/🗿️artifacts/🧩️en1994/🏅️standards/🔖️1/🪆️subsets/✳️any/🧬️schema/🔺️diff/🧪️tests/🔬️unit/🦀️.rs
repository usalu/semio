//! 🧪️ Keyed row-delta algebra of En1994: id-keyed lanes through absorb, coalescing, and the negative diff.

use semio_s_artifact_norm_en1994_diff::{En1994BeamsActionsInserted, En1994BeamsActionsModified, En1994BeamsActionsPatch, En1994BeamsActionsRemoved, En1994BeamsActionsRows, En1994BeamsPatch, En1994BeamsRows, En1994ColumnsActionsInserted, En1994ColumnsActionsRemoved, En1994ColumnsActionsRows, En1994ColumnsPatch, En1994ColumnsRows, En1994Diff, En1994SlabsActionsRemoved, En1994SlabsActionsRows, En1994SlabsPatch, En1994SlabsRows};
use semio_s_artifact_norm_en1994_mut_beams::change_beam_action_q_area_pa::ChangeBeamActionQAreaPa;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_span_m::ChangeBeamSpanM;
use semio_s_artifact_norm_en1994_mut_beams::insert_beam::InsertBeam;
use semio_s_artifact_norm_en1994_mut_beams::remove_beam::RemoveBeam;
use semio_s_artifact_norm_en1994_mutations::En1994Mutation;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use protocol::{ColdDiffAlgebra as _, ColdMutation as _, DiffAlgebra as _, MutationDiff as _};
use protocol::ColdMutationDiff;

fn base() -> En1994Snapshot {
    let mut base = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
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

fn span(base: &En1994Snapshot, index: usize, value: f64) -> En1994Mutation {
    En1994Mutation::ChangeBeamSpanM(ChangeBeamSpanM { id: base.beams[index].id.clone(), new_span_m: value })
}

fn fresh(base: &En1994Snapshot) -> semio_s_artifact_norm_en1994_model::CompositeBeam {
    let mut beam = base.beams[0].clone();
    beam.id = "beam-created".into();
    beam
}

async fn law(base: &En1994Snapshot, first: &En1994Mutation, second: impl Fn(&En1994Snapshot) -> En1994Mutation) -> En1994Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff_cold(&d1, base).expect("first");
    let d2 = diff_of(&second(&mid), &mid);
    let mut sum = d1.clone();
    sum.absorb(d2.clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(base, d1, d2).await;
    sum
}

#[semio_framework_async_macros::async_test]
async fn a_row_inserted_before_a_patched_row_leaves_the_patch_on_the_same_row_id() {
    let base = base();
    let sum = law(&base, &En1994Mutation::InsertBeam(InsertBeam { index: 0, beam: fresh(&base) }), |mid| span(mid, 1, 31.0)).await;
    let rows = sum.beams.expect("beams");
    assert_eq!(rows.inserted.len(), 1);
    assert_eq!(rows.inserted[0].index, 0);
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].id, base.beams[0].id);
}

#[semio_framework_async_macros::async_test]
async fn a_removed_row_leaves_later_patches_on_their_row_ids() {
    let base = base();
    let sum = law(&base, &En1994Mutation::RemoveBeam(RemoveBeam { id: base.beams[0].id.clone() }), |mid| span(mid, 0, 33.0)).await;
    let rows = sum.beams.expect("beams");
    assert_eq!(rows.removed.len(), 1);
    assert_eq!((rows.removed[0].id.as_str(), rows.removed[0].index), (base.beams[0].id.as_str(), 0));
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].id, base.beams[1].id);
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_cancels_and_a_patch_on_an_inserted_row_folds_into_it() {
    let base = base();
    let sum = law(&base, &En1994Mutation::InsertBeam(InsertBeam { index: 1, beam: fresh(&base) }), |mid| En1994Mutation::RemoveBeam(RemoveBeam { id: mid.beams[1].id.clone() })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    let sum = law(&base, &En1994Mutation::InsertBeam(InsertBeam { index: 1, beam: fresh(&base) }), |mid| span(mid, 1, 41.0)).await;
    let rows = sum.beams.expect("beams");
    assert!(rows.modified.is_empty());
    assert_eq!(rows.inserted[0].row.span_m, 41.0);
}

#[semio_framework_async_macros::async_test]
async fn action_patches_coalesce_by_id() {
    let base = base();
    assert!(!base.beams[0].actions.is_empty());
    let set = |value: f64| En1994Mutation::ChangeBeamActionQAreaPa(ChangeBeamActionQAreaPa { beam_id: base.beams[0].id.clone(), id: base.beams[0].actions[0].id.clone(), new_q_area_pa: value });
    let sum = law(&base, &set(1000.0), |_| set(2000.0)).await;
    let patches = sum.beams.expect("beams").modified;
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].actions.as_ref().expect("actions").modified.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_follows_the_state() {
    let base = base();
    let inserted = diff_of(&En1994Mutation::InsertBeam(InsertBeam { index: 0, beam: fresh(&base) }), &base);
    let after = protocol::apply_diff_cold(&inserted, &base).expect("insert");
    assert_eq!(protocol::apply_diff_cold(&inserted.inverse(&base), &after).expect("inverse"), base);
    let removed = diff_of(&En1994Mutation::RemoveBeam(RemoveBeam { id: base.beams[0].id.clone() }), &base);
    let after = protocol::apply_diff_cold(&removed, &base).expect("remove");
    assert_eq!(protocol::apply_diff_cold(&removed.inverse(&base), &after).expect("inverse"), base);
}

fn beam_actions(base: &En1994Snapshot, rows: En1994BeamsActionsRows) -> En1994Diff {
    En1994Diff { beams: Some(En1994BeamsRows::modification(&base.beams[0].id, En1994BeamsPatch { actions: Some(rows), ..Default::default() })), ..Default::default() }
}

fn column_actions(base: &En1994Snapshot, rows: En1994ColumnsActionsRows) -> En1994Diff {
    En1994Diff { columns: Some(En1994ColumnsRows::modification(&base.columns[0].id, En1994ColumnsPatch { actions: Some(rows), ..Default::default() })), ..Default::default() }
}

fn slab_actions(base: &En1994Snapshot, rows: En1994SlabsActionsRows) -> En1994Diff {
    En1994Diff { slabs: Some(En1994SlabsRows::modification(&base.slabs[0].id, En1994SlabsPatch { actions: Some(rows), ..Default::default() })), ..Default::default() }
}

fn fresh_beam_action(base: &En1994Snapshot, id: &str) -> semio_s_artifact_norm_en1994_model::CharacteristicAction {
    let mut action = base.beams[0].actions[0].clone();
    action.id = id.into();
    action
}

#[test]
fn nested_action_rows_apply_and_invert_by_id() {
    let base = base();
    let first = base.beams[0].actions[0].id.clone();
    let count = base.beams[0].actions.len();
    let inserted = beam_actions(&base, En1994BeamsActionsRows { inserted: vec![En1994BeamsActionsInserted { index: 1, row: fresh_beam_action(&base, "G-new") }], ..Default::default() });
    let after = protocol::apply_diff_cold(&inserted, &base).expect("insert");
    assert_eq!(after.beams[0].actions.len(), count + 1);
    assert_eq!(after.beams[0].actions[1].id, "G-new");
    assert_eq!(protocol::apply_diff_cold(&inserted.inverse(&base), &after).expect("inverse of insert"), base);
    let removed = beam_actions(&base, En1994BeamsActionsRows { removed: vec![En1994BeamsActionsRemoved { id: first.clone(), index: 0 }], ..Default::default() });
    let after = protocol::apply_diff_cold(&removed, &base).expect("remove");
    assert_eq!(after.beams[0].actions.len(), count - 1);
    assert_ne!(after.beams[0].actions[0].id, first);
    assert_eq!(protocol::apply_diff_cold(&removed.inverse(&base), &after).expect("inverse of remove"), base);
}

#[test]
fn nested_column_and_slab_action_rows_remove_and_restore() {
    let base = base();
    let column = column_actions(&base, En1994ColumnsActionsRows { removed: vec![En1994ColumnsActionsRemoved { id: base.columns[0].actions[0].id.clone(), index: 0 }], ..Default::default() });
    let after = protocol::apply_diff_cold(&column, &base).expect("remove column action");
    assert_eq!(after.columns[0].actions.len(), base.columns[0].actions.len() - 1);
    assert_eq!(protocol::apply_diff_cold(&column.inverse(&base), &after).expect("restore column action"), base);
    let slab = slab_actions(&base, En1994SlabsActionsRows { removed: vec![En1994SlabsActionsRemoved { id: base.slabs[0].actions[0].id.clone(), index: 0 }], ..Default::default() });
    let after = protocol::apply_diff_cold(&slab, &base).expect("remove slab action");
    assert_eq!(protocol::apply_diff_cold(&slab.inverse(&base), &after).expect("restore slab action"), base);
    let mut extra = base.columns[0].actions[0].clone();
    extra.id = "N-new".into();
    let insert = column_actions(&base, En1994ColumnsActionsRows { inserted: vec![En1994ColumnsActionsInserted { index: 0, row: extra }], ..Default::default() });
    let after = protocol::apply_diff_cold(&insert, &base).expect("insert column action");
    assert_eq!(after.columns[0].actions[0].id, "N-new");
    assert_eq!(protocol::apply_diff_cold(&insert.inverse(&base), &after).expect("undo column insert"), base);
}

#[test]
fn nested_action_removal_names_the_row_by_id_and_position() {
    let base = base();
    let wrong = beam_actions(&base, En1994BeamsActionsRows { removed: vec![En1994BeamsActionsRemoved { id: "not-there".into(), index: 0 }], ..Default::default() });
    assert!(protocol::apply_diff_cold(&wrong, &base).is_err(), "an id the row at that position does not carry must be refused");
    let past = beam_actions(&base, En1994BeamsActionsRows { removed: vec![En1994BeamsActionsRemoved { id: base.beams[0].actions[0].id.clone(), index: base.beams[0].actions.len() }], ..Default::default() });
    assert!(protocol::apply_diff_cold(&past, &base).is_err(), "a position past the end must be refused");
    let patch = En1994BeamsActionsModified { id: base.beams[0].actions[0].id.clone(), patch: En1994BeamsActionsPatch { kind: Some(semio_s_artifact_norm_en1994_model::ActionKind::Imposed), ..Default::default() } };
    let both = beam_actions(&base, En1994BeamsActionsRows { removed: vec![En1994BeamsActionsRemoved { id: base.beams[0].actions[0].id.clone(), index: 0 }], modified: vec![patch], ..Default::default() });
    assert!(protocol::apply_diff_cold(&both, &base).is_err(), "a row cannot be patched and removed by the same diff");
}

#[semio_framework_async_macros::async_test]
async fn nested_action_rows_obey_the_absorb_laws() {
    let base = base();
    let added = fresh_beam_action(&base, "G-new");
    let insert = beam_actions(&base, En1994BeamsActionsRows { inserted: vec![En1994BeamsActionsInserted { index: 1, row: added.clone() }], ..Default::default() });
    let remove_again = beam_actions(&base, En1994BeamsActionsRows { removed: vec![En1994BeamsActionsRemoved { id: "G-new".into(), index: 1 }], ..Default::default() });
    let mut cancelled = insert.clone();
    cancelled.absorb(remove_again.clone());
    assert!(cancelled.beams.as_ref().expect("beams").modified[0].actions.is_none(), "insert then remove of the same action cancels: {cancelled:?}");
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, insert.clone(), remove_again).await;
    let retouch = beam_actions(&base, En1994BeamsActionsRows { modified: vec![En1994BeamsActionsModified { id: "G-new".into(), patch: En1994BeamsActionsPatch { kind: Some(semio_s_artifact_norm_en1994_model::ActionKind::Imposed), ..Default::default() } }], ..Default::default() });
    let mut folded = insert.clone();
    folded.absorb(retouch.clone());
    let lane = folded.beams.as_ref().expect("beams").modified[0].actions.as_ref().expect("actions");
    assert_eq!(lane.inserted[0].row.kind, semio_s_artifact_norm_en1994_model::ActionKind::Imposed);
    assert!(lane.modified.is_empty());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, insert, retouch).await;
    let first = base.beams[0].actions[0].id.clone();
    let remove = beam_actions(&base, En1994BeamsActionsRows { removed: vec![En1994BeamsActionsRemoved { id: first.clone(), index: 0 }], ..Default::default() });
    let mid = protocol::apply_diff_cold(&remove, &base).expect("remove");
    let reinsert = beam_actions(&base, En1994BeamsActionsRows { inserted: vec![En1994BeamsActionsInserted { index: 0, row: base.beams[0].actions[0].clone() }], ..Default::default() });
    assert_eq!(protocol::apply_diff_cold(&reinsert, &mid).expect("reinsert"), base);
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, remove, reinsert).await;
}
