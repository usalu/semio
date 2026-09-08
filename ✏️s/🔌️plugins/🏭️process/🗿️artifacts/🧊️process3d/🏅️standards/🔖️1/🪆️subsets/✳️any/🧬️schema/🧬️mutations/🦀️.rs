//! 🧬️ Process3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Process3dSnapshot`'s shape (`workshop.machines: Vec<WorkshopMachine>`,
//! `stock: Stock`, `steps: Vec<ProcessStep>`, `resolved_up_to: Option<usize>`) per
//! `📓️derivation-rules.md`: an id-keyed, order-meaningful `steps` timeline
//! (`create`/`delete`/`rename`/`change-*-enabled`/`change-*-origin`/`replace-*-measure`/
//! `reorder-steps`), an id-keyed, unordered `machines` set
//! (`create`/`delete`/`rename`/`change-*-icon`/`replace-*-capabilities`), the document's single
//! `stock` facet split into its spatial (`move-stock`), identity (`change-stock-label`), and large
//! structured (`replace-stock-solid`) fields, and one document-level scalar (`change-cursor`).
//! Every variant wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Process3dSnapshot, Process3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to
//! each payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! The whole-collection `Steps { collection: ... }` / `Machines { collection: ... }` / `SetStock` /
//! `SetCursor` / whole-document-replacement variants — the pre-migration generic vocabulary — are
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
use super::change_cursor;
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
    ChangeCursor(change_cursor::ChangeCursor),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Process3dMutation` variant, in declaration order — the vocabulary the `process3d-1-any` mutation catalog
/// (`../../🔮️oracle/🔣️.json`) declares and the `🏭️mutate-process3d-1` exhaustive test case measures
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
    "change-cursor",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Process3dMutation` nor
/// `Process3dSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔮️oracle/🔣️.json — the catalog and the recorded no-oracle decision.
pub fn process3d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Process3dSnapshot, String> {
        let decoded: Process3dSnapshot = semio_framework_os_kernel::json::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Process3dMutation = semio_framework_os_kernel::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = semio_framework_os_kernel::json::object([
        ("base".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&base))),
        ("expectedSnapshot".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&expected))),
        ("snapshot".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&applied))),
        ("diff".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(forward.diff()))),
        ("messages".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&forward.messages().to_vec()))),
        ("inverseSteps".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&inverse))),
        ("inverseSnapshot".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&undone))),
        ("inverseMessages".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(semio_framework_os_kernel::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
