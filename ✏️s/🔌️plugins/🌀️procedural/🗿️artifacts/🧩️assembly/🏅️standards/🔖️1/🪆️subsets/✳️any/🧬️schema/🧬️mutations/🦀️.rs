//! 🧬️ Assembly artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload, one per
//! `🧬️mutations/<slug>/` triad leaf wired by `🦀️.rs`. `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<AssemblySnapshot>` and `impl protocol::SemanticMutation<AssemblySnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
// 🧵 Deliberately NOT `use super::{create_slot, ...};` — this file's own `pub use X::mutation::x;`
// builder re-exports below, glob-re-exported back into `mutations` by the sibling `pub use
// component::*;` in `🦀️.rs`, would collide with a bare-name import of the same sibling
// submodules (E0252, hit and fixed once already this wave) — fully qualifying each variant's
// payload path below instead breaks that self-referential loop.

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = AssemblySnapshot, diff = AssemblyDiff, schema = "assembly")]
pub enum AssemblyMutation {
    CreateSlot(super::create_slot::CreateSlot),
    DeleteSlot(super::delete_slot::DeleteSlot),
    CreateRule(super::create_rule::CreateRule),
    DeleteRule(super::delete_rule::DeleteRule),
    ChangeWeight(super::change_weight::ChangeWeight),
    RemoveWeight(super::remove_weight::RemoveWeight),
    ConnectSlots(super::connect_slots::ConnectSlots),
    DisconnectSlots(super::disconnect_slots::DisconnectSlots),
    ChangeSeed(super::change_seed::ChangeSeed),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`AssemblyMutation`] variant, in declaration order — the exact
/// vocabulary the `assembly-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧩️mutate-assembly-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &["create-slot", "delete-slot", "create-rule", "delete-rule", "change-weight", "remove-weight", "connect-slots", "disconnect-slots", "change-seed"];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use super::change_seed::change_seed;
pub use super::change_weight::change_weight;
pub use super::connect_slots::connect_slots;
pub use super::create_rule::create_rule;
pub use super::create_slot::create_slot;
pub use super::delete_rule::delete_rule;
pub use super::delete_slot::delete_slot;
pub use super::disconnect_slots::disconnect_slots;
pub use super::remove_weight::remove_weight;
//#endregion 🔖️Builders

pub type AssemblyEnvelope = store::ArtifactEnvelope<AssemblySnapshot, AssemblyMutation>;
pub type AssemblyStore = store::ArtifactStore<AssemblySnapshot, AssemblyMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_assembly_mutation(projection: &mut AssemblySnapshot, mutation: &AssemblyMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_assembly_mutation(projection: &AssemblySnapshot, mutation: &AssemblyMutation) -> Vec<AssemblyMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::assembly::schema::snapshot::{AssemblyRule, AssemblySlot, AssemblySlotEdge};
    use protocol::{MutationDiff, SemanticMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    use vcs::apply_mutation;

    fn round_trip(projection: &AssemblySnapshot, mutation: &AssemblyMutation) -> AssemblySnapshot {
        let (forward, _) = apply_mutation(projection, mutation).expect("valid mutation");
        let mut restored = forward.clone();
        for back in mutation.inverse(projection) {
            restored = apply_mutation(&restored, &back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, projection, "inverse() must restore the pre-mutation document");
        forward
    }

    #[test]
    fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
        for kind in AssemblyMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(AssemblyMutation::kinds().len(), 9);
    }

    #[test]
    fn create_slot_inverse_law_round_trips() {
        let base = AssemblySnapshot::default();
        let mutation = create_slot(0, AssemblySlot { id: "s1".into(), x: 1.0, y: 2.0, z: 0.0, pinned_module_id: None });
        let after = round_trip(&base, &mutation);
        assert_eq!(after.slots.len(), 1);
    }

    #[test]
    fn delete_slot_inverse_law_round_trips() {
        let mut base = AssemblySnapshot::default();
        base.slots.push(AssemblySlot { id: "s1".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None });
        let mutation = delete_slot("s1".into());
        let after = round_trip(&base, &mutation);
        assert!(after.slots.is_empty());
    }

    #[test]
    fn delete_slot_cascades_incident_edges() {
        let mut base = AssemblySnapshot::default();
        base.slots.push(AssemblySlot { id: "s1".into(), ..Default::default() });
        base.slots.push(AssemblySlot { id: "s2".into(), ..Default::default() });
        base.edges.push(AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s2".into() });
        let mutation = delete_slot("s1".into());
        let after = round_trip(&base, &mutation);
        assert!(after.edges.is_empty(), "deleting a slot must cascade-delete its incident edges");
    }

    #[test]
    fn create_rule_inverse_law_round_trips() {
        let base = AssemblySnapshot::default();
        let mutation = create_rule(0, AssemblyRule { id: "r1".into(), module_a_id: "a".into(), module_b_id: "b".into(), allowed: true, params: SemioValue::default() });
        let after = round_trip(&base, &mutation);
        assert_eq!(after.rules.len(), 1);
    }

    #[test]
    fn delete_rule_inverse_law_round_trips() {
        let mut base = AssemblySnapshot::default();
        base.rules.push(AssemblyRule { id: "r1".into(), module_a_id: "a".into(), module_b_id: "b".into(), allowed: true, params: SemioValue::default() });
        let mutation = delete_rule("r1".into());
        let after = round_trip(&base, &mutation);
        assert!(after.rules.is_empty());
    }

    #[test]
    fn change_weight_inverse_law_restores_the_prior_value() {
        let mut base = AssemblySnapshot::default();
        base.weights.push(crate::artifacts::assembly::schema::snapshot::AssemblyModuleWeight { module_id: "m1".into(), weight: 1.0 });
        let mutation = change_weight("m1".into(), 9.0);
        let after = round_trip(&base, &mutation);
        assert_eq!(after.weights[0].weight, 9.0);
    }

    #[test]
    fn change_weight_on_unknown_module_inserts_and_inverse_removes() {
        let base = AssemblySnapshot::default();
        let mutation = change_weight("m1".into(), 4.0);
        let after = round_trip(&base, &mutation);
        assert_eq!(after.weights.len(), 1);
    }

    #[test]
    fn connect_slots_inverse_law_round_trips() {
        let mut base = AssemblySnapshot::default();
        base.slots.push(AssemblySlot { id: "s1".into(), ..Default::default() });
        base.slots.push(AssemblySlot { id: "s2".into(), ..Default::default() });
        let mutation = connect_slots(0, AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s2".into() });
        let after = round_trip(&base, &mutation);
        assert_eq!(after.edges.len(), 1);
    }

    #[test]
    fn disconnect_slots_inverse_law_round_trips() {
        let mut base = AssemblySnapshot::default();
        base.edges.push(AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s2".into() });
        let mutation = disconnect_slots("e1".into());
        let after = round_trip(&base, &mutation);
        assert!(after.edges.is_empty());
    }

    #[test]
    fn change_seed_inverse_law_round_trips() {
        let mut base = AssemblySnapshot::default();
        base.seed = 1;
        let mutation = change_seed(42);
        let after = round_trip(&base, &mutation);
        assert_eq!(after.seed, 42);
    }

    #[test]
    fn diff_absorb_composes_two_change_seed_mutations() {
        let base = AssemblySnapshot::default();
        let d1 = change_seed(1).diff(&base);
        let mid = d1.apply(&base).expect("valid mutation diff");
        let d2 = change_seed(2).diff(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base).expect("valid mutation diff"), d2.apply(&mid).expect("valid mutation diff"));
    }

    //#region 🧪️KindsCatalog
    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
    /// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
    /// declared vocabulary and the measured one from drifting apart.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <AssemblyMutation as protocol::SemanticMutation<AssemblySnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared AssemblyMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🔮️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🧪️KindsCatalog
}
//#endregion 🧪️Tests
