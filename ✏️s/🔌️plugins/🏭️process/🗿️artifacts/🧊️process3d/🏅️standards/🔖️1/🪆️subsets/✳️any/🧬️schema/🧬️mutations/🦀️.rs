//! 🧬️ Process3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Process3dSnapshot`'s shape (`workshop.machines: Vec<WorkshopMachine>`,
//! `stock: Stock`, `steps: Vec<ProcessStep>`) per
//! `📓️derivation-rules.md`: an id-keyed, order-meaningful `steps` timeline
//! (`create`/`delete`/`rename`/`change-*-enabled`/`change-*-origin`/`replace-*-measure`/
//! `reorder-steps`), an id-keyed, unordered `machines` set
//! (`create`/`delete`/`rename`/`change-*-icon`/`replace-*-capabilities`), the document's single
//! `stock` facet split into its spatial (`move-stock`), identity (`change-stock-label`), and large
//! structured (`replace-stock-solid`) fields. The replay cursor is view state (`Process3dConfig`), not a document
//! field, so it has no mutation here.
//! Every variant wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Process3dSnapshot, Process3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to
//! each payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! The whole-collection `Steps { collection: ... }` / `Machines { collection: ... }` / `SetStock` /
//! whole-document-replacement variants — the pre-migration generic vocabulary — are
//! gone. Whole-document replacement has NO replacement mutation (it is banned; file-open/import/
//! load-example goes through `store::ArtifactStore::reset`, outside this enum).
//!
//! Every triad-leaf directory now carries its target slug (`kind` name, emoji stripped) exactly —
//! the five directories that used to repurpose pre-migration names (`⏱️set-cursor` → `⏱️change-cursor`,
//! `🟤️set-snapshot` → `📐replace-step-measure`, `📋steps` → `🌱create-step`, `🛠️machines` →
//! `🏭create-machine`, `🧱set-stock` → `📍move-stock`) were renamed, and every duplicate emoji among
//! the fresh leaves was reassigned a unique one within this facet, as part of this ticket's
//! directory + glue trueing pass. See this facet's migration report for the emoji table.

use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MutationLeaves
// 🌱️ Every `🧬️mutations/<kind>/` triad leaf is `#[path]`-mounted as a sibling of this dispatch file
// directly in the plugin's `🦀️.rs` (this facet's fan-out ticket, SEMANTIC-MUTATIONS-OVERHAUL
// wave-C, owns `🦀️.rs` for this plugin); `use super::<kind>;` below brings each sibling into
// this file's scope so the enum body can reference `<kind>::<Type>`.
use super::change_machine_icon;
use super::change_step_enabled;
use super::change_step_origin;
use super::change_stock_label;
use super::create_machine;
use super::create_step;
use super::delete_machine;
use super::delete_step;
use super::move_stock;
use super::rename_machine;
use super::rename_step;
use super::reorder_steps;
use super::replace_machine_capabilities;
use super::replace_step_measure;
use super::replace_stock_solid;
//#endregion 🔖️MutationLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the process3d document, derived per
/// `📓️derivation-rules.md` from `Process3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Process3dSnapshot, diff = Process3dDiff, schema = "process.process3d")]
pub enum Process3dMutation {
    CreateStep(create_step::CreateStep),
    DeleteStep(delete_step::DeleteStep),
    RenameStep(rename_step::RenameStep),
    ChangeStepEnabled(change_step_enabled::ChangeStepEnabled),
    ChangeStepOrigin(change_step_origin::ChangeStepOrigin),
    ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure),
    ReorderSteps(reorder_steps::ReorderSteps),
    CreateMachine(create_machine::CreateMachine),
    DeleteMachine(delete_machine::DeleteMachine),
    RenameMachine(rename_machine::RenameMachine),
    ChangeMachineIcon(change_machine_icon::ChangeMachineIcon),
    ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities),
    MoveStock(move_stock::MoveStock),
    ChangeStockLabel(change_stock_label::ChangeStockLabel),
    ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Process3dMutation` variant, in declaration order — the vocabulary the `process3d-1-any` mutation catalog
/// (`../../🔮️oracles/🔣️.json`) declares and the `🏭️mutate-process3d-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "create-step",
    "delete-step",
    "rename-step",
    "change-step-enabled",
    "change-step-origin",
    "replace-step-measure",
    "reorder-steps",
    "create-machine",
    "delete-machine",
    "rename-machine",
    "change-machine-icon",
    "replace-machine-capabilities",
    "move-stock",
    "change-stock-label",
    "replace-stock-solid",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
