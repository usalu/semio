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
//! `SetSnapshot` — the pre-migration whole-document replace — is gone: banned outright per
//! `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement mutation; file-open/import/
//! load-example now goes through `store::ArtifactStore::reset`, entirely outside this enum.
//!
//! `📄set-snapshot` keeps its pre-migration directory name — `📦️glue.rs` path-includes that exact
//! triad outside this facet's writable boundary, so it was repurposed in place (same path,
//! rewritten `🦠️mutation`/`🔺️diff`/`↩️inverse` content) to hold `ChangeAnnex` instead of being
//! renamed; see this ticket's wave2 report `sharedFileRequests` for the rename once a later pass
//! can touch `📦️glue.rs`. The other nine triads have no pre-migration slot and are self-wired
//! directly below via nested `#[path = "."] pub mod <name> { ... }` blocks (mirrors this ticket's
//! `din16798`/`process3d` precedent — `#[path]` resolves per physical file, not per logical mod
//! nesting, so this works without touching `📦️glue.rs`).

use crate::artifacts::en1990::diff::En1990Diff;
use crate::artifacts::en1990::En1990Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️NewLeaves
#[path = "."]
pub mod change_permanent_action {
    #[path = "🏛️change-permanent-action/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🏛️change-permanent-action/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🏛️change-permanent-action/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_resistance {
    #[path = "🛡️change-resistance/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🛡️change-resistance/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🛡️change-resistance/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_consequence_class {
    #[path = "🎯change-consequence-class/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🎯change-consequence-class/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🎯change-consequence-class/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_seismic_action {
    #[path = "🌍change-seismic-action/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🌍change-seismic-action/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🌍change-seismic-action/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod insert_variable_action {
    #[path = "➕insert-variable-action/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "➕insert-variable-action/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "➕insert-variable-action/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_variable_action {
    #[path = "➖remove-variable-action/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "➖remove-variable-action/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "➖remove-variable-action/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_variable_action_category {
    #[path = "🏷️change-variable-action-category/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🏷️change-variable-action-category/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🏷️change-variable-action-category/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_variable_action_value {
    #[path = "🔢change-variable-action-value/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔢change-variable-action-value/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔢change-variable-action-value/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod reorder_variable_actions {
    #[path = "🔀reorder-variable-actions/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔀reorder-variable-actions/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔀reorder-variable-actions/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
//#endregion 🔖️NewLeaves

//#region 🔖️RepurposedLeaves
// 🌱️ `set_snapshot` is declared by `📦️glue.rs` as a sibling of `component` (this file) under
// `pub mod mutations { ... }` — brought into this file's own scope the same way `din16798`'s
// already-migrated `🧬️mutations/🦀️component.rs` reaches its own repurposed sibling.
use super::set_snapshot;
//#endregion 🔖️RepurposedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1990 document, derived per
/// `📓️derivation-rules.md` from `En1990Snapshot`'s flat scalar + `q_k` table shape.
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
        let forward = vcs::apply_mutation(base, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back);
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

    #[test]
    fn insert_remove_variable_action_round_trips() {
        let base = En1990Snapshot::default();

        let insert = En1990Mutation::InsertVariableAction(insert_variable_action::mutation::InsertVariableAction { index: 1, category: "snow".into(), value: 20.0 });
        let after_insert = round_trip(&base, &insert);
        assert_eq!(after_insert.q_k.len(), base.q_k.len() + 1);
        assert_eq!(after_insert.q_k[1].category, "snow");

        let undo = insert.inverse(&base);
        assert_eq!(undo, vec![En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 1 })]);

        let remove = En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 0 });
        let after_remove = round_trip(&base, &remove);
        assert_eq!(after_remove.q_k.len(), base.q_k.len() - 1);
        assert_eq!(after_remove.q_k[0], base.q_k[1]);
    }

    #[test]
    fn remove_variable_action_of_an_out_of_range_index_has_an_empty_inverse() {
        let base = En1990Snapshot::default();
        let remove = En1990Mutation::RemoveVariableAction(remove_variable_action::mutation::RemoveVariableAction { index: 99 });
        assert!(remove.inverse(&base).is_empty(), "removing an absent index has nothing to undo");
        assert_eq!(remove.diff(&base).apply(&base), base, "an out-of-range remove is a no-op");
    }

    #[test]
    fn reorder_variable_actions_round_trips() {
        let base = En1990Snapshot::default();
        assert!(base.q_k.len() >= 2, "fixture must have at least two variable actions to exercise reorder");

        let reorder = En1990Mutation::ReorderVariableActions(reorder_variable_actions::mutation::ReorderVariableActions { from: 0, to: 1 });
        let after = round_trip(&base, &reorder);
        assert_eq!(after.q_k[0], base.q_k[1]);
        assert_eq!(after.q_k[1], base.q_k[0]);
    }

    #[test]
    fn change_variable_action_category_and_value_round_trip() {
        let base = En1990Snapshot::default();

        let category = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory { index: 0, new_category: "storage".into() });
        let after = round_trip(&base, &category);
        assert_eq!(after.q_k[0].category, "storage");
        assert_eq!(after.q_k[0].value, base.q_k[0].value);

        let value = En1990Mutation::ChangeVariableActionValue(change_variable_action_value::mutation::ChangeVariableActionValue { index: 0, new_value: 65.0 });
        let after = round_trip(&base, &value);
        assert_eq!(after.q_k[0].value, 65.0);

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
        let d1 = mutation.diff(&base);
        let d2 = En1990Mutation::ChangeResistance(change_resistance::mutation::ChangeResistance { new_resistance_kn: 400.0 }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_resistance_satisfies_the_inverse_and_absorb_laws() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeResistance(change_resistance::mutation::ChangeResistance { new_resistance_kn: 400.0 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1990Mutation::ChangePermanentAction(change_permanent_action::mutation::ChangePermanentAction { new_g_k: 130.0 }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_variable_action_value_satisfies_the_inverse_and_absorb_laws() {
        let base = En1990Snapshot::default();
        let mutation = En1990Mutation::ChangeVariableActionValue(change_variable_action_value::mutation::ChangeVariableActionValue { index: 0, new_value: 65.0 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = En1990Mutation::ChangeVariableActionCategory(change_variable_action_category::mutation::ChangeVariableActionCategory { index: 1, new_category: "storage".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
