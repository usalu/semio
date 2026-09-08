
use super::*;
use crate::document::AnnexChoice;
use protocol::Mutation;

/// ⚖️ One value per `En1990Mutation` variant — the closed set the semantics/round-trip tests
/// iterate, mirroring `din16798`'s own `every_mutation()` fixture.
fn every_mutation() -> Vec<En1990Mutation> {
    vec![
        En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1990Mutation::ChangePermanentAction(change_permanent_action::ChangePermanentAction { new_g_k: 120.0 }),
        En1990Mutation::ChangeResistance(change_resistance::ChangeResistance { new_resistance_kn: 350.0 }),
        En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: 3 }),
        En1990Mutation::ChangeSeismicAction(change_seismic_action::ChangeSeismicAction { new_seismic_a_ed_kn: 60.0 }),
        En1990Mutation::InsertVariableAction(insert_variable_action::InsertVariableAction { index: 1, category: "snow".into(), value: 20.0 }),
        En1990Mutation::RemoveVariableAction(remove_variable_action::RemoveVariableAction { index: 0 }),
        En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::ChangeVariableActionCategory { index: 0, new_category: "storage".into() }),
        En1990Mutation::ChangeVariableActionValue(change_variable_action_value::ChangeVariableActionValue { index: 0, new_value: 65.0 }),
        En1990Mutation::ReorderVariableActions(reorder_variable_actions::ReorderVariableActions { from: 0, to: 1 }),
    ]
}

fn round_trip(base: &En1990Snapshot, mutation: &En1990Mutation) -> En1990Snapshot {
    let (forward, _messages) = vcs::apply_mutation(base, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in mutation.inverse(base) {
        let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

#[semio_framework_async_macros::async_test]
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for mutation in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
    }
    assert_eq!(<En1990Mutation as protocol::SemanticMutation<En1990Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = En1990Snapshot::default();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

/// 🔎 `q_k` is a composed `s.stdio.semio.table` child slot — every assertion below reads
/// through the `en1990_qk` working-scene accessor instead of indexing the field directly.
fn qk(snapshot: &En1990Snapshot) -> Vec<crate::En1990QkEntry> {
    crate::en1990_qk(snapshot)
}

#[semio_framework_async_macros::async_test]
async fn insert_remove_variable_action_round_trips() {
    let base = En1990Snapshot::default();

    let insert = En1990Mutation::InsertVariableAction(insert_variable_action::InsertVariableAction { index: 1, category: "snow".into(), value: 20.0 });
    let after_insert = round_trip(&base, &insert);
    assert_eq!(qk(&after_insert).len(), qk(&base).len() + 1);
    assert_eq!(qk(&after_insert)[1].category, "snow");

    let undo = insert.inverse(&base);
    assert_eq!(undo, vec![En1990Mutation::RemoveVariableAction(remove_variable_action::RemoveVariableAction { index: 1 })]);

    let remove = En1990Mutation::RemoveVariableAction(remove_variable_action::RemoveVariableAction { index: 0 });
    let after_remove = round_trip(&base, &remove);
    assert_eq!(qk(&after_remove).len(), qk(&base).len() - 1);
    assert_eq!(qk(&after_remove)[0], qk(&base)[1]);
}

#[semio_framework_async_macros::async_test]
async fn remove_variable_action_of_an_out_of_range_index_is_rejected() {
    let base = En1990Snapshot::default();
    let remove = En1990Mutation::RemoveVariableAction(remove_variable_action::RemoveVariableAction { index: 99 });
    assert!(remove.inverse(&base).is_empty(), "removing an absent index has nothing to undo");
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &remove).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_variable_actions_round_trips() {
    let base = En1990Snapshot::default();
    assert!(qk(&base).len() >= 2, "fixture must have at least two variable actions to exercise reorder");

    let reorder = En1990Mutation::ReorderVariableActions(reorder_variable_actions::ReorderVariableActions { from: 0, to: 1 });
    let after = round_trip(&base, &reorder);
    assert_eq!(qk(&after)[0], qk(&base)[1]);
    assert_eq!(qk(&after)[1], qk(&base)[0]);
}

#[semio_framework_async_macros::async_test]
async fn change_variable_action_category_and_value_round_trip() {
    let base = En1990Snapshot::default();

    let category = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::ChangeVariableActionCategory { index: 0, new_category: "storage".into() });
    let after = round_trip(&base, &category);
    assert_eq!(qk(&after)[0].category, "storage");
    assert_eq!(qk(&after)[0].value, qk(&base)[0].value);

    let value = En1990Mutation::ChangeVariableActionValue(change_variable_action_value::ChangeVariableActionValue { index: 0, new_value: 65.0 });
    let after = round_trip(&base, &value);
    assert_eq!(qk(&after)[0].value, 65.0);

    let missing = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::ChangeVariableActionCategory { index: 99, new_category: "x".into() });
    assert!(missing.inverse(&base).is_empty(), "changing an absent index has nothing to undo");
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::testkit`), exercised against the three most structurally
/// distinct variants: the repurposed enum-typed slot (`change-annex`), a plain `f64` scalar
/// (`change-resistance`), and an index-addressed table field (`change-variable-action-value`).
#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1990Mutation::ChangeResistance(change_resistance::ChangeResistance { new_resistance_kn: 400.0 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_resistance_satisfies_the_inverse_and_absorb_laws() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeResistance(change_resistance::ChangeResistance { new_resistance_kn: 400.0 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1990Mutation::ChangePermanentAction(change_permanent_action::ChangePermanentAction { new_g_k: 130.0 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_variable_action_value_satisfies_the_inverse_and_absorb_laws() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeVariableActionValue(change_variable_action_value::ChangeVariableActionValue { index: 0, new_value: 65.0 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::ChangeVariableActionCategory { index: 1, new_category: "storage".into() }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// one check per verb family this facet implements — change/set/update (root scalars + the
/// index-addressed `q_k` table), insert (clamped), remove (target-missing), reorder
/// (target-missing/no-op). `assert_outcome_policy_matrix` is not landed under that literal name
/// yet (only the differently-shaped `assert_policy_matrix` exists) — flagged, not improvised
/// around.
#[semio_framework_async_macros::async_test]
async fn remove_variable_action_missing_target_is_error() {
    let base = En1990Snapshot::default();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &En1990Mutation::RemoveVariableAction(remove_variable_action::RemoveVariableAction { index: 99 })).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_variable_actions_missing_target_is_error() {
    let base = En1990Snapshot::default();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &En1990Mutation::ReorderVariableActions(reorder_variable_actions::ReorderVariableActions { from: 99, to: 0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_variable_action_category_missing_target_is_error() {
    let base = En1990Snapshot::default();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::ChangeVariableActionCategory { index: 99, new_category: "x".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_variable_action_value_missing_target_is_error() {
    let base = En1990Snapshot::default();
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &En1990Mutation::ChangeVariableActionValue(change_variable_action_value::ChangeVariableActionValue { index: 99, new_value: 1.0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn insert_variable_action_out_of_range_index_is_clamped() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::InsertVariableAction(insert_variable_action::InsertVariableAction { index: 999, category: "snow".into(), value: 10.0 });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.clamped"));
}

#[semio_framework_async_macros::async_test]
async fn change_seismic_action_non_finite_is_fatal() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeSeismicAction(change_seismic_action::ChangeSeismicAction { new_seismic_a_ed_kn: f64::NAN });
    let outcome = mutation.diff(&base);
    protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn change_consequence_class_out_of_domain_is_fatal() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: 9 });
    let outcome = mutation.diff(&base);
    protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn change_resistance_is_deterministic() {
    let base = En1990Snapshot::default();
    let mutation = En1990Mutation::ChangeResistance(change_resistance::ChangeResistance { new_resistance_kn: 400.0 });
    protocol::os_spr::testkit::assert_outcome_deterministic(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws
