use super::*;
use crate::diff::{ProgramStakeholdersDelta, ProgramStakeholdersPatchEntry};
use crate::registers::{Stakeholder, StakeholderPatch};
use crate::sample_plugin;

fn stakeholder(id: &str, name: &str) -> Stakeholder {
    let mut row = sample_plugin().stakeholders[0].clone();
    row.header.id = EntityId(id.into());
    row.header.name = name.into();
    row.department = None;
    row
}

fn base() -> Vec<Stakeholder> {
    vec![stakeholder("a", "Alpha"), stakeholder("b", "Beta"), stakeholder("c", "Gamma")]
}

fn names(rows: &[Stakeholder]) -> Vec<(String, String)> {
    rows.iter().map(|row| (row.header.id.0.clone(), row.header.name.clone())).collect()
}

fn create(row: Stakeholder) -> ProgramStakeholdersDelta {
    ProgramStakeholdersDelta { added: vec![row], ..Default::default() }
}

fn delete(id: &str) -> ProgramStakeholdersDelta {
    ProgramStakeholdersDelta { removed: vec![id.into()], ..Default::default() }
}

fn rename(id: &str, name: &str) -> ProgramStakeholdersDelta {
    ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: id.into(), patch: StakeholderPatch { name: Some(name.into()), ..Default::default() } }], ..Default::default() }
}

fn replace(rows: &[Stakeholder], row: Stakeholder) -> ProgramStakeholdersDelta {
    let position = rows.iter().position(|existing| existing.header.id == row.header.id).expect("replaced row exists");
    let reordered = (position + 1 != rows.len()).then(|| rows.iter().map(|existing| existing.header.id.0.clone()).collect());
    ProgramStakeholdersDelta { removed: vec![row.header.id.0.clone()], added: vec![row], reordered, ..Default::default() }
}

fn applied(rows: &[Stakeholder], delta: &ProgramStakeholdersDelta) -> Vec<Stakeholder> {
    let mut next = rows.to_vec();
    apply(delta, &mut next).expect("valid delta applies");
    next
}

fn assert_absorb_law(rows: &[Stakeholder], first: ProgramStakeholdersDelta, later: ProgramStakeholdersDelta) -> ProgramStakeholdersDelta {
    let sequential = applied(&applied(rows, &first), &later);
    let merged = absorb(first, later);
    assert_eq!(applied(rows, &merged), sequential, "absorb(d1, d2).apply(base) must equal d2.apply(d1.apply(base))");
    merged
}

fn assert_inverse_law(rows: &[Stakeholder], delta: &ProgramStakeholdersDelta) {
    let after = applied(rows, delta);
    assert_eq!(applied(&after, &inverse(delta, rows)), rows, "inverse(d, base).apply(d.apply(base)) must equal base, row positions included");
}

#[semio_framework_async_macros::async_test]
async fn patch_then_patch_coalesces_into_one_patch() {
    let merged = assert_absorb_law(&base(), rename("b", "One"), rename("b", "Two"));
    assert_eq!(merged.patched.len(), 1, "two patches of one id are one entry");
    assert_eq!(merged.patched[0].patch.name.as_deref(), Some("Two"), "the later field wins");
    assert!(merged.added.is_empty() && merged.removed.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn patch_then_patch_of_different_fields_keeps_both() {
    let first = ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: "b".into(), patch: StakeholderPatch { role: Some("Sponsor".into()), ..Default::default() } }], ..Default::default() };
    let merged = assert_absorb_law(&base(), first, rename("b", "Two"));
    assert_eq!(merged.patched.len(), 1);
    assert_eq!(merged.patched[0].patch.role.as_deref(), Some("Sponsor"));
    assert_eq!(merged.patched[0].patch.name.as_deref(), Some("Two"));
}

#[semio_framework_async_macros::async_test]
async fn create_then_delete_cancels_to_nothing() {
    let merged = assert_absorb_law(&base(), create(stakeholder("d", "Delta")), delete("d"));
    assert!(is_empty(&merged), "create∘delete leaves no entry: {merged:?}");
}

#[semio_framework_async_macros::async_test]
async fn delete_then_create_is_a_replace() {
    let merged = assert_absorb_law(&base(), delete("b"), create(stakeholder("b", "Beta again")));
    assert_eq!(merged.removed, vec!["b".to_string()]);
    assert_eq!(merged.added.len(), 1);
    assert_eq!(merged.added[0].header.name, "Beta again");
    assert!(merged.patched.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn patch_then_delete_is_a_delete() {
    let merged = assert_absorb_law(&base(), rename("b", "Renamed"), delete("b"));
    assert_eq!(merged.removed, vec!["b".to_string()]);
    assert!(merged.patched.is_empty() && merged.added.is_empty(), "the patch dies with its row: {merged:?}");
}

#[semio_framework_async_macros::async_test]
async fn create_then_patch_folds_into_the_created_row() {
    let merged = assert_absorb_law(&base(), create(stakeholder("d", "Delta")), rename("d", "Delta 2"));
    assert_eq!(merged.added.len(), 1);
    assert_eq!(merged.added[0].header.name, "Delta 2");
    assert!(merged.patched.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn replace_then_patch_and_replace_then_delete_collapse() {
    let rows = base();
    let replaced = assert_absorb_law(&rows, replace(&rows, stakeholder("b", "Replaced")), rename("b", "Patched"));
    assert_eq!(replaced.removed, vec!["b".to_string()]);
    assert_eq!(replaced.added[0].header.name, "Patched");
    assert!(replaced.patched.is_empty());
    let removed = assert_absorb_law(&rows, replace(&rows, stakeholder("b", "Replaced")), delete("b"));
    assert_eq!(removed.removed, vec!["b".to_string()]);
    assert!(removed.added.is_empty() && removed.patched.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn absorb_is_associative_over_a_create_patch_delete_chain() {
    let rows = base();
    let (d1, d2, d3) = (create(stakeholder("d", "Delta")), rename("d", "Delta 2"), delete("a"));
    let left = absorb(absorb(d1.clone(), d2.clone()), d3.clone());
    let right = absorb(d1.clone(), absorb(d2.clone(), d3.clone()));
    assert_eq!(applied(&rows, &left), applied(&rows, &right));
    assert_eq!(names(&applied(&rows, &left)), vec![("b".into(), "Beta".into()), ("c".into(), "Gamma".into()), ("d".into(), "Delta 2".into())]);
}

#[semio_framework_async_macros::async_test]
async fn reordered_composes_with_later_adds_and_removes() {
    let rows = base();
    let reorder = ProgramStakeholdersDelta { reordered: Some(vec!["c".into(), "a".into(), "b".into()]), ..Default::default() };
    let merged = assert_absorb_law(&rows, reorder.clone(), create(stakeholder("d", "Delta")));
    assert_eq!(merged.reordered, Some(vec!["c".into(), "a".into(), "b".into(), "d".into()]));
    let merged = assert_absorb_law(&rows, reorder, delete("a"));
    assert_eq!(merged.reordered, Some(vec!["c".into(), "b".into()]));
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_row_content_and_position() {
    let rows = base();
    for delta in [delete("b"), delete("a"), delete("c"), create(stakeholder("d", "Delta")), rename("b", "Renamed"), replace(&rows, stakeholder("a", "Replaced")), replace(&rows, stakeholder("c", "Replaced")), ProgramStakeholdersDelta { reordered: Some(vec!["c".into(), "b".into(), "a".into()]), ..Default::default() }] {
        assert_inverse_law(&rows, &delta);
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_of_a_patch_that_sets_an_optional_field_falls_back_to_a_replacement() {
    let rows = base();
    let delta = ProgramStakeholdersDelta { patched: vec![ProgramStakeholdersPatchEntry { id: "b".into(), patch: StakeholderPatch { department: Some("Ops".into()), ..Default::default() } }], ..Default::default() };
    let undo = inverse(&delta, &rows);
    assert!(undo.patched.is_empty(), "a patch cannot clear the optional department");
    assert_eq!((undo.removed.clone(), undo.added.len()), (vec!["b".to_string()], 1));
    assert_inverse_law(&rows, &delta);
}

#[semio_framework_async_macros::async_test]
async fn between_rebuilds_the_target_and_is_empty_for_equal_rows() {
    let rows = base();
    let mut other = rows.clone();
    other.remove(0);
    other[0].header.name = "Beta 2".into();
    other[1].department = None;
    other.push(stakeholder("d", "Delta"));
    other.swap(0, 2);
    let delta: ProgramStakeholdersDelta = between(&rows, &other);
    assert_eq!(applied(&rows, &delta), other);
    assert!(is_empty(&between::<ProgramStakeholdersDelta>(&rows, &rows)));
    let mut cleared = rows.clone();
    cleared[1].department = Some("Ops".into());
    let delta: ProgramStakeholdersDelta = between(&cleared, &rows);
    assert_eq!(applied(&cleared, &delta), rows, "an optional field cleared between two states is carried as a replacement");
}

#[semio_framework_async_macros::async_test]
async fn apply_rejects_malformed_deltas_with_their_address() {
    let rows = base();
    for (delta, code) in [
        (delete("zz"), "mutation.apply.missing-target"),
        (ProgramStakeholdersDelta { removed: vec!["a".into(), "a".into()], ..Default::default() }, "mutation.apply.duplicate-target"),
        (create(stakeholder("a", "Again")), "mutation.apply.duplicate-target"),
        (rename("zz", "x"), "mutation.apply.missing-target"),
        (ProgramStakeholdersDelta { patched: vec![rename("a", "x").patched.remove(0), rename("a", "y").patched.remove(0)], ..Default::default() }, "mutation.apply.duplicate-target"),
        (ProgramStakeholdersDelta { reordered: Some(vec!["a".into(), "b".into()]), ..Default::default() }, "mutation.apply.invalid-order"),
        (ProgramStakeholdersDelta { reordered: Some(vec!["a".into(), "a".into(), "b".into()]), ..Default::default() }, "mutation.apply.invalid-order"),
    ] {
        let mut next = rows.clone();
        let error = apply(&delta, &mut next).expect_err("a malformed delta is rejected");
        assert_eq!(error.code, code);
    }
}
