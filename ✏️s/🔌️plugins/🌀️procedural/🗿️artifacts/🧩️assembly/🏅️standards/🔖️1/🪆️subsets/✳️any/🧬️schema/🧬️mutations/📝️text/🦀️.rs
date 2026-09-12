//! ⚡️ Assembly artifact — the mutation vocabulary's single-line TEXT opcodes + grammar.
//!
//! One keyword per `AssemblyMutation` variant, in the `KINDS` order the catalog
//! (`../../../🔮️oracles/🔣️.json`) declares. The operation twin exists for the same reason the
//! snapshot's does — `AssemblyRule::params` is a foreign `SemioValue` — and is bridged through the
//! snapshot facet's own `slot_to_dsl`/`rule_to_dsl`/`edge_to_dsl` pairs, never a second conversion.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::mutations::AssemblyMutation;
use crate::schema::mutations::change_seed::ChangeSeed;
use crate::schema::mutations::change_weight::ChangeWeight;
use crate::schema::mutations::connect_slots::ConnectSlots;
use crate::schema::mutations::create_rule::CreateRule;
use crate::schema::mutations::create_slot::CreateSlot;
use crate::schema::mutations::delete_rule::DeleteRule;
use crate::schema::mutations::delete_slot::DeleteSlot;
use crate::schema::mutations::disconnect_slots::DisconnectSlots;
use crate::schema::mutations::remove_weight::RemoveWeight;
use crate::schema::snapshot::text::{edge_from_dsl, edge_to_dsl, rule_from_dsl, rule_to_dsl, slot_from_dsl, slot_to_dsl, AssemblyRuleDsl, AssemblySlotDsl, AssemblySlotEdgeDsl};

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum AssemblyOperationDsl {
    CreateSlot {
        index: usize,
        #[dsl(block)]
        slot: AssemblySlotDsl,
    },
    DeleteSlot {
        id: String,
    },
    CreateRule {
        index: usize,
        #[dsl(block)]
        rule: AssemblyRuleDsl,
    },
    DeleteRule {
        id: String,
    },
    ChangeWeight {
        module_id: String,
        weight: f64,
    },
    RemoveWeight {
        module_id: String,
    },
    ConnectSlots {
        index: usize,
        #[dsl(block)]
        edge: AssemblySlotEdgeDsl,
    },
    DisconnectSlots {
        id: String,
    },
    ChangeSeed {
        seed: u64,
    },
}

pub fn operation_to_dsl(operation: &AssemblyMutation) -> AssemblyOperationDsl {
    match operation {
        AssemblyMutation::CreateSlot(CreateSlot { index, slot }) => AssemblyOperationDsl::CreateSlot { index: *index, slot: slot_to_dsl(slot) },
        AssemblyMutation::DeleteSlot(DeleteSlot { id }) => AssemblyOperationDsl::DeleteSlot { id: id.clone() },
        AssemblyMutation::CreateRule(CreateRule { index, rule }) => AssemblyOperationDsl::CreateRule { index: *index, rule: rule_to_dsl(rule) },
        AssemblyMutation::DeleteRule(DeleteRule { id }) => AssemblyOperationDsl::DeleteRule { id: id.clone() },
        AssemblyMutation::ChangeWeight(ChangeWeight { module_id, weight }) => AssemblyOperationDsl::ChangeWeight { module_id: module_id.clone(), weight: *weight },
        AssemblyMutation::RemoveWeight(RemoveWeight { module_id }) => AssemblyOperationDsl::RemoveWeight { module_id: module_id.clone() },
        AssemblyMutation::ConnectSlots(ConnectSlots { index, edge }) => AssemblyOperationDsl::ConnectSlots { index: *index, edge: edge_to_dsl(edge) },
        AssemblyMutation::DisconnectSlots(DisconnectSlots { id }) => AssemblyOperationDsl::DisconnectSlots { id: id.clone() },
        AssemblyMutation::ChangeSeed(ChangeSeed { seed }) => AssemblyOperationDsl::ChangeSeed { seed: *seed },
    }
}

pub fn operation_from_dsl(operation: AssemblyOperationDsl) -> Result<AssemblyMutation, store::TextError> {
    Ok(match operation {
        AssemblyOperationDsl::CreateSlot { index, slot } => AssemblyMutation::CreateSlot(CreateSlot { index, slot: slot_from_dsl(slot) }),
        AssemblyOperationDsl::DeleteSlot { id } => AssemblyMutation::DeleteSlot(DeleteSlot { id }),
        AssemblyOperationDsl::CreateRule { index, rule } => AssemblyMutation::CreateRule(CreateRule { index, rule: rule_from_dsl(rule)? }),
        AssemblyOperationDsl::DeleteRule { id } => AssemblyMutation::DeleteRule(DeleteRule { id }),
        AssemblyOperationDsl::ChangeWeight { module_id, weight } => AssemblyMutation::ChangeWeight(ChangeWeight { module_id, weight }),
        AssemblyOperationDsl::RemoveWeight { module_id } => AssemblyMutation::RemoveWeight(RemoveWeight { module_id }),
        AssemblyOperationDsl::ConnectSlots { index, edge } => AssemblyMutation::ConnectSlots(ConnectSlots { index, edge: edge_from_dsl(edge) }),
        AssemblyOperationDsl::DisconnectSlots { id } => AssemblyMutation::DisconnectSlots(DisconnectSlots { id }),
        AssemblyOperationDsl::ChangeSeed { seed } => AssemblyMutation::ChangeSeed(ChangeSeed { seed }),
    })
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslOps`/`dsl::DslEnum` emit `DslVariants` only (P6).
impl protocol::OpText for AssemblyOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown assembly mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ `AssemblyMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for AssemblyMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        operation_from_dsl(<AssemblyOperationDsl as protocol::OpText>::parse_op(line)?)
    }

    fn print_op(&self) -> String {
        <AssemblyOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.assembly` mutation line.
pub fn parse_op(line: &str) -> Result<AssemblyMutation, store::TextError> {
    <AssemblyMutation as protocol::OpText>::parse_op(line)
}

/// 🖨️ Prints one `AssemblyMutation` back to its single-line form.
pub fn print_op(operation: &AssemblyMutation) -> String {
    protocol::OpText::print_op(operation)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_op`/`print_op` speak.
pub type AssemblyMutationText = String;
//#endregion 🚚️Carrier
