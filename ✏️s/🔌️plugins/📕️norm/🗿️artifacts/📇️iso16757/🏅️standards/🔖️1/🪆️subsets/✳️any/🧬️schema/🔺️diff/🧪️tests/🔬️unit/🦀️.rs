//! 🧪️ Keyed-diff algebra of Iso16757: id-keyed row coalescing and position tracking, positional constraint rows, map entries and the negative diff.

use super::Iso16757Diff;
use crate::mutations::add_selection_constraint::mutation::AddSelectionConstraint;
use crate::mutations::change_part_number_input::mutation::ChangePartNumberInput;
use crate::mutations::introduce_product_group::mutation::IntroduceProductGroup;
use crate::mutations::remove_selection_constraint::mutation::RemoveSelectionConstraint;
use crate::mutations::rename_product_group::mutation::RenameProductGroup;
use crate::mutations::retire_product_group::mutation::RetireProductGroup;
use crate::mutations::Iso16757Mutation;
use crate::Iso16757Snapshot;
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

fn diff_of(mutation: &Iso16757Mutation, base: &Iso16757Snapshot) -> Iso16757Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

async fn law(base: &Iso16757Snapshot, first: &Iso16757Mutation, second: impl Fn(&Iso16757Snapshot) -> Iso16757Mutation) -> Iso16757Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff(&d1, base).expect("first");
    let d2 = diff_of(&second(&mid), &mid);
    let mut sum = d1.clone();
    sum.absorb(d2.clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(base, d1, d2).await;
    sum
}

fn group(base: &Iso16757Snapshot) -> crate::part_1::ProductGroup {
    let mut group = base.catalogue.product_groups[0].clone();
    group.id = "group-created".into();
    group
}

#[semio_framework_async_macros::async_test]
async fn renames_of_one_group_coalesce_to_the_last_name() {
    let base = Iso16757Snapshot::default();
    let id = base.catalogue.product_groups[0].id.clone();
    let rename = |name: &str| Iso16757Mutation::RenameProductGroup(RenameProductGroup { id: id.clone(), new_name: name.into() });
    let sum = law(&base, &rename("Alpha"), |_| rename("Beta")).await;
    let rows = sum.product_groups.expect("groups");
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].patch.name.as_deref(), Some("Beta"));
}

#[semio_framework_async_macros::async_test]
async fn introduce_then_retire_cancels_and_retire_then_introduce_replaces() {
    let base = Iso16757Snapshot::default();
    let created = group(&base);
    let sum = law(&base, &Iso16757Mutation::IntroduceProductGroup(IntroduceProductGroup { product_group: created.clone(), index: Some(0) }), |_| Iso16757Mutation::RetireProductGroup(RetireProductGroup { id: created.id.clone() })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    let existing = base.catalogue.product_groups[0].clone();
    let sum = law(&base, &Iso16757Mutation::RetireProductGroup(RetireProductGroup { id: existing.id.clone() }), |_| Iso16757Mutation::IntroduceProductGroup(IntroduceProductGroup { product_group: existing.clone(), index: Some(0) })).await;
    assert_eq!(protocol::apply_diff(&sum, &base).expect("replace"), base);
}

#[semio_framework_async_macros::async_test]
async fn constraint_rows_follow_positions() {
    let base = Iso16757Snapshot::default();
    let mut constraint = base.selection.constraints[0].clone();
    constraint.id = "constraint-created".into();
    let sum = law(&base, &Iso16757Mutation::AddSelectionConstraint(AddSelectionConstraint { constraint: constraint.clone(), index: None }), |_| Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index: base.selection.constraints.len() })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    let sum = law(&base, &Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index: 0 }), |_| Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index: 0 })).await;
    assert_eq!(sum.selection_constraints.expect("constraints").removed, vec![0, 1]);
}

#[semio_framework_async_macros::async_test]
async fn part_number_inputs_are_added_then_changed_in_place() {
    let base = Iso16757Snapshot::default();
    let value = base.part_number_inputs.values().next().expect("input").clone();
    let set = |value: crate::CatalogueValue| Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key: "input-created".into(), new_value: value });
    let sum = law(&base, &set(value.clone()), |_| set(value.clone())).await;
    assert_eq!(sum.part_number_inputs.expect("inputs").added.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_follows_the_state() {
    let base = Iso16757Snapshot::default();
    let inserted = diff_of(&Iso16757Mutation::IntroduceProductGroup(IntroduceProductGroup { product_group: group(&base), index: Some(0) }), &base);
    let after = protocol::apply_diff(&inserted, &base).expect("insert");
    assert_eq!(protocol::apply_diff(&inserted.inverse(&base), &after).expect("inverse"), base);
}
