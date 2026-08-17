//! 🧬️ En1990 artifact — closed semantic mutation dispatch enum (constitutional: op). Derived from
//! `En1990Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less, document-root
//! parameter form (`g_k`, `resistance_kn`, `consequence_class`, `annex`, `seismic_a_ed_kn`) plus
//! `q_k: Vec<En1990QkEntry>`, an intrinsically ordered, id-less table of variable actions (rule 3).
//! No name/identity field to `rename`; every scalar becomes its own `change-<field>` mutation, none
//! qualify for the `update-<facet>` grouping exception (each is independently entered, never
//! validated as an atomic multi-field bundle). `q_k` gets `insert-variable-action`/
//! `remove-variable-action` (index addressing, insert=FINAL/remove=BASE per the taxonomy),
//! `reorder-variable-actions`, and `change-variable-action-{category,value}` per remaining field.
//!
//! The pre-migration whole-document-replace variant is gone: banned outright per
//! `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement mutation; file-open/import/
//! load-example now goes through `store::ArtifactStore::reset`, entirely outside this enum.
//!
//! All ten triads (including the repurposed `set_snapshot` slot, which still holds `ChangeAnnex`)
//! are mounted directly as `mutations`-sibling modules in `📦️glue.rs` (this lane's agent owns
//! `📦️glue.rs`, so no self-wiring `#[path = "."]` blocks are needed here).

use crate::artifacts::en1990::diff::En1990Diff;
use crate::artifacts::en1990::En1990Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
use super::change_consequence_class;
use super::change_permanent_action;
use super::change_resistance;
use super::change_seismic_action;
use super::change_variable_action_category;
use super::change_variable_action_value;
use super::insert_variable_action;
use super::remove_variable_action;
use super::reorder_variable_actions;
/// 🧬️ Closed semantic mutation vocabulary for the en1990 document, derived per
/// `📓️derivation-rules.md` from `En1990Snapshot`'s flat scalar + `q_k` table shape.
//#region 🔖️Leaves
use super::set_snapshot;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1990Snapshot, diff = En1990Diff, schema = "s.norm.en1990")]
pub enum En1990Mutation {
    ChangeAnnex(set_snapshot::mutation::ChangeAnnex),
    ChangePermanentAction(change_permanent_action::mutation::ChangePermanentAction),
    ChangeResistance(change_resistance::mutation::ChangeResistance),
    ChangeConsequenceClass(change_consequence_class::mutation::ChangeConsequenceClass),
    ChangeSeismicAction(change_seismic_action::mutation::ChangeSeismicAction),
    InsertVariableAction(insert_variable_action::mutation::InsertVariableAction),
    RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction),
    ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory),
    ChangeVariableActionValue(change_variable_action_value::mutation::ChangeVariableActionValue),
    ReorderVariableActions(reorder_variable_actions::mutation::ReorderVariableActions),
}
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1990Mutation {
    /// 📤️ Decomposes a whole-document replacement into the closed semantic vocabulary — the
    /// replacement for the banned whole-document-replace variant, used by `import_media`'s
    /// `"model:in"` port and the `set-snapshot` app command. Unlike the other norm facets' single-arg
    /// `from_snapshot`, this one also takes `base` (the pre-replacement document) because `q_k` is a
    /// real ordered collection: every existing entry must be removed (highest index first, so
    /// indices stay valid mid-sequence) before `target`'s entries are re-inserted in order — a plain
    /// per-field decomposition can't express "replace the whole table" on its own.
    pub fn from_snapshot(base: &En1990Snapshot, target: &En1990Snapshot) -> Vec<En1990Mutation> {
        let base_q_k = crate::artifacts::en1990::en1990_qk(base);
        let target_q_k = crate::artifacts::en1990::en1990_qk(target);
        let mut mutations = Vec::with_capacity(5 + base_q_k.len() + target_q_k.len());
        mutations.push(En1990Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: target.annex.clone() }));
        mutations.push(En1990Mutation::ChangePermanentAction(change_permanent_action::mutation::ChangePermanentAction { new_g_k: target.g_k.clone() }));
        mutations.push(En1990Mutation::ChangeResistance(change_resistance::mutation::ChangeResistance { new_resistance_kn: target.resistance_kn.clone() }));
        mutations.push(En1990Mutation::ChangeConsequenceClass(change_consequence_class::mutation::ChangeConsequenceClass { new_consequence_class: target.consequence_class.clone() }));
        mutations.push(En1990Mutation::ChangeSeismicAction(change_seismic_action::mutation::ChangeSeismicAction { new_seismic_a_ed_kn: target.seismic_a_ed_kn.clone() }));
        for index in (0..base_q_k.len()).rev() {
            mutations.push(En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index }));
        }
        for (index, entry) in target_q_k.iter().enumerate() {
            mutations.push(En1990Mutation::InsertVariableAction(insert_variable_action::mutation::InsertVariableAction { index, category: entry.category.clone(), value: entry.value.clone() }));
        }
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::AnnexChoice;
    use protocol::{Mutation, MutationDiff, SemanticMutation};

    /// ⚖️ One value per `En1990Mutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring `din16798`'s own `every_mutation()` fixture.
    fn every_mutation() -> Vec<En1990Mutation> {
        vec![
            En1990Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: AnnexChoice::En }),
            En1990Mutation::ChangePermanentAction(change_permanent_action::mutation::ChangePermanentAction { new_g_k: 120.0 }),
            En1990Mutation::ChangeResistance(change_resistance::mutation::ChangeResistance { new_resistance_kn: 350.0 }),
            En1990Mutation::ChangeConsequenceClass(change_consequence_class::mutation::ChangeConsequenceClass { new_consequence_class: 3 }),
            En1990Mutation::ChangeSeismicAction(change_seismic_action::mutation::ChangeSeismicAction { new_seismic_a_ed_kn: 60.0 }),
            En1990Mutation::InsertVariableAction(insert_variable_action::mutation::InsertVariableAction { index: 1, category: "snow".into(), value: 20.0 }),
            En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 0 }),
            En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory { index: 0, new_category: "storage".into() }),
            En1990Mutation::ChangeVariableActionValue(change_variable_action_value::mutation::ChangeVariableActionValue { index: 0, new_value: 65.0 }),
            En1990Mutation::ReorderVariableActions(reorder_variable_actions::mutation::ReorderVariableActions { from: 0, to: 1 }),
        ]
    }

    fn round_trip(base: &En1990Snapshot, mutation: &En1990Mutation) -> En1990Snapshot {
        let (forward, _messages) =
            vcs::apply_mutation(base, mutation).expect("valid mutation");
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            let (next, _messages) =
                vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
            restored = next;
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<En1990Mutation as protocol::SemanticMutation<En1990Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = En1990Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    /// 🔎 `q_k` is a composed `s.stdio.semio.table` child slot — every assertion below reads
    /// through the `en1990_qk` working-scene accessor instead of indexing the field directly.
    fn qk(snapshot: &En1990Snapshot) -> Vec<crate::artifacts::en1990::En1990QkEntry> {
        crate::artifacts::en1990::en1990_qk(snapshot)
    }

    #[test]
    fn insert_remove_variable_action_round_trips() {
        let base = En1990Snapshot::default();

        let insert = En1990Mutation::InsertVariableAction(insert_variable_action::mutation::InsertVariableAction { index: 1, category: "snow".into(), value: 20.0 });
        let after_insert = round_trip(&base, &insert);
        assert_eq!(qk(&after_insert).len(), qk(&base).len() + 1);
        assert_eq!(qk(&after_insert)[1].category, "snow");

        let undo = insert.inverse(&base);
        assert_eq!(undo, vec![En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 1 })]);

        let remove = En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 0 });
        let after_remove = round_trip(&base, &remove);
        assert_eq!(qk(&after_remove).len(), qk(&base).len() - 1);
        assert_eq!(qk(&after_remove)[0], qk(&base)[1]);
    }

    #[test]
    fn remove_variable_action_of_an_out_of_range_index_is_rejected() {
        let base = En1990Snapshot::default();
        let remove = En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 99 });
        assert!(remove.inverse(&base).is_empty(), "removing an absent index has nothing to undo");
        protocol::testkit::assert_missing_target_is_error(&base, &remove);
    }

    #[test]
    fn reorder_variable_actions_round_trips() {
        let base = En1990Snapshot::default();
        assert!(qk(&base).len() >= 2, "fixture must have at least two variable actions to exercise reorder");

        let reorder = En1990Mutation::ReorderVariableActions(reorder_variable_actions::mutation::ReorderVariableActions { from: 0, to: 1 });
        let after = round_trip(&base, &reorder);
        assert_eq!(qk(&after)[0], qk(&base)[1]);
        assert_eq!(qk(&after)[1], qk(&base)[0]);
    }

    #[test]
    fn change_variable_action_category_and_value_round_trip() {
        let base = En1990Snapshot::default();

        let category = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory { index: 0, new_category: "storage".into() });
        let after = round_trip(&base, &category);
        assert_eq!(qk(&after)[0].category, "storage");
        assert_eq!(qk(&after)[0].value, qk(&base)[0].value);

        let value = En1990Mutation::ChangeVariableActionValue(change_variable_action_value::mutation::ChangeVariableActionValue { index: 0, new_value: 65.0 });
        let after = round_trip(&base, &value);
        assert_eq!(qk(&after)[0].value, 65.0);

        let missing = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory { index: 99, new_category: "x".into() });
        assert!(missing.inverse(&base).is_empty(), "changing an absent index has nothing to undo");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::testkit`), exercised against the three most structurally
    /// distinct variants: the repurposed enum-typed slot (`change-annex`), a plain `f64` scalar
    /// (`change-resistance`), and an index-addressed table field (`change-variable-action-value`).
    #[test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: AnnexChoice::En });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1990Mutation::ChangeResistance(change_resistance::mutation::ChangeResistance { new_resistance_kn: 400.0 }).diff(&base).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_resistance_satisfies_the_inverse_and_absorb_laws() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeResistance(change_resistance::mutation::ChangeResistance { new_resistance_kn: 400.0 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1990Mutation::ChangePermanentAction(change_permanent_action::mutation::ChangePermanentAction { new_g_k: 130.0 }).diff(&base).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_variable_action_value_satisfies_the_inverse_and_absorb_laws() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeVariableActionValue(change_variable_action_value::mutation::ChangeVariableActionValue { index: 0, new_value: 65.0 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory { index: 1, new_category: "storage".into() }).diff(&base).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// one check per verb family this facet implements — change/set/update (root scalars + the
    /// index-addressed `q_k` table), insert (clamped), remove (target-missing), reorder
    /// (target-missing/no-op). `assert_outcome_policy_matrix` is not landed under that literal name
    /// yet (only the differently-shaped `assert_policy_matrix` exists) — flagged, not improvised
    /// around.
    #[test]
    fn remove_variable_action_missing_target_is_error() {
        let base = En1990Snapshot::default();
        protocol::testkit::assert_missing_target_is_error(&base, &En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 99 }));
    }

    #[test]
    fn reorder_variable_actions_missing_target_is_error() {
        let base = En1990Snapshot::default();
        protocol::testkit::assert_missing_target_is_error(&base, &En1990Mutation::ReorderVariableActions(reorder_variable_actions::mutation::ReorderVariableActions { from: 99, to: 0 }));
    }

    #[test]
    fn change_variable_action_category_missing_target_is_error() {
        let base = En1990Snapshot::default();
        protocol::testkit::assert_missing_target_is_error(&base, &En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory { index: 99, new_category: "x".into() }));
    }

    #[test]
    fn change_variable_action_value_missing_target_is_error() {
        let base = En1990Snapshot::default();
        protocol::testkit::assert_missing_target_is_error(&base, &En1990Mutation::ChangeVariableActionValue(change_variable_action_value::mutation::ChangeVariableActionValue { index: 99, new_value: 1.0 }));
    }

    #[test]
    fn insert_variable_action_out_of_range_index_is_clamped() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::InsertVariableAction(insert_variable_action::mutation::InsertVariableAction { index: 999, category: "snow".into(), value: 10.0 });
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.clamped"));
    }

    #[test]
    fn change_seismic_action_non_finite_is_fatal() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeSeismicAction(change_seismic_action::mutation::ChangeSeismicAction { new_seismic_a_ed_kn: f64::NAN });
        let outcome = mutation.diff(&base);
        protocol::testkit::assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    fn change_consequence_class_out_of_domain_is_fatal() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeConsequenceClass(change_consequence_class::mutation::ChangeConsequenceClass { new_consequence_class: 9 });
        let outcome = mutation.diff(&base);
        protocol::testkit::assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[test]
    fn change_resistance_is_deterministic() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeResistance(change_resistance::mutation::ChangeResistance { new_resistance_kn: 400.0 });
        protocol::testkit::assert_outcome_deterministic(&base, &mutation);
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
