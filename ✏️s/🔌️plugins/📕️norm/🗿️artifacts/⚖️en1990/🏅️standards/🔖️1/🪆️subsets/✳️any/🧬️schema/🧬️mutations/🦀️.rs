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
//! All ten semantic mutation triads
//! are mounted directly as `mutations`-sibling modules in `🦀️.rs` (this lane's agent owns
//! `🦀️.rs`, so no self-wiring `#[path = "."]` blocks are needed here).

use crate::diff::En1990Diff;
use crate::En1990Snapshot;

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the en1990 document, derived per
/// `📓️derivation-rules.md` from `En1990Snapshot`'s flat scalar + `q_k` table shape.
//#region 🔖️Leaves
use super::change_annex;
use super::change_consequence_class;
use super::change_permanent_action;
use super::change_resistance;
use super::change_seismic_action;
use super::change_variable_action_category;
use super::change_variable_action_value;
use super::insert_variable_action;
use super::remove_variable_action;
use super::reorder_variable_actions;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1990Snapshot, diff = En1990Diff, schema = "s.norm.en1990")]
pub enum En1990Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangePermanentAction(change_permanent_action::ChangePermanentAction),
    ChangeResistance(change_resistance::ChangeResistance),
    ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass),
    ChangeSeismicAction(change_seismic_action::ChangeSeismicAction),
    InsertVariableAction(insert_variable_action::InsertVariableAction),
    RemoveVariableAction(remove_variable_action::RemoveVariableAction),
    ChangeVariableActionCategory(change_variable_action_category::ChangeVariableActionCategory),
    ChangeVariableActionValue(change_variable_action_value::ChangeVariableActionValue),
    ReorderVariableActions(reorder_variable_actions::ReorderVariableActions),
}

/// 🏷️ Every declared kind of [`En1990Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🔮️oracle/🔣️.json` publishes as the `en1990-1-any`
/// mutation catalog and `../../🧪️tests/⚖️mutate-en1990-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-annex",
    "change-permanent-action",
    "change-resistance",
    "change-consequence-class",
    "change-seismic-action",
    "insert-variable-action",
    "remove-variable-action",
    "change-variable-action-category",
    "change-variable-action-value",
    "reorder-variable-actions",
];
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
        let base_q_k = crate::en1990_qk(base);
        let target_q_k = crate::en1990_qk(target);
        let mut mutations = Vec::with_capacity(5 + base_q_k.len() + target_q_k.len());
        mutations.push(En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        mutations.push(En1990Mutation::ChangePermanentAction(change_permanent_action::ChangePermanentAction { new_g_k: target.g_k }));
        mutations.push(En1990Mutation::ChangeResistance(change_resistance::ChangeResistance { new_resistance_kn: target.resistance_kn }));
        mutations.push(En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: target.consequence_class }));
        mutations.push(En1990Mutation::ChangeSeismicAction(change_seismic_action::ChangeSeismicAction { new_seismic_a_ed_kn: target.seismic_a_ed_kn }));
        for index in (0..base_q_k.len()).rev() {
            mutations.push(En1990Mutation::RemoveVariableAction(remove_variable_action::RemoveVariableAction { index }));
        }
        for (index, entry) in target_q_k.iter().enumerate() {
            mutations.push(En1990Mutation::InsertVariableAction(insert_variable_action::InsertVariableAction { index, category: entry.category.clone(), value: entry.value }));
        }
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Self-wired fixture cases for the EN 1990 mutation vocabulary: one handcrafted case per
// triad leaf, mounted here rather than in `🦀️.rs` because that file is shared by all
// fifteen norm artifacts and several lanes edit it at once. `#[path = "."]` keeps the
// inline module's own name out of the base directory, so every leaf path below is read
// straight off this `🧬️mutations/` directory (ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION,
// contract D1).
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`En1990Mutation`]. The generated test host of
/// `../../🧪️tests/⚖️mutate-en1990-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1990_mutation_json(text: &str) -> Result<En1990Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_en1990_mutation(base: &En1990Snapshot, mutation: &En1990Mutation) -> Result<(En1990Snapshot, Vec<String>), String> {
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `⚖️mutate-en1990-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_en1990_mutation(mutation: &En1990Mutation, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    <En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
