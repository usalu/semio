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

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
#[cfg(test)]
#[path = "🌱create-step/🧪️tests/🪚️accepts/🦀️.rs"]
mod tests_accepts_a_rip_cut_step_and_changes_nothing;
#[cfg(test)]
#[path = "🗑️delete-step/🧪️tests/🚫️accepts/🦀️.rs"]
mod tests_accepts_a_step_id_and_changes_nothing;
#[cfg(test)]
#[path = "🏷️rename-step/🧪️tests/🔤️accepts/🦀️.rs"]
mod tests_accepts_a_new_label_and_changes_nothing;
#[cfg(test)]
#[path = "🔘change-step-enabled/🧪️tests/⏸️accepts/🦀️.rs"]
mod tests_accepts_a_disable_flag_and_changes_nothing;
#[cfg(test)]
#[path = "🧷change-step-origin/🧪️tests/🏭️accepts/🦀️.rs"]
mod tests_accepts_a_machine_provenance_and_changes_nothing;
#[cfg(test)]
#[path = "📐replace-step-measure/🧪️tests/🕳️accepts/🦀️.rs"]
mod tests_accepts_a_bore_measure_and_changes_nothing;
#[cfg(test)]
#[path = "🔀reorder-steps/🧪️tests/🔀️accepts/🦀️.rs"]
mod tests_accepts_a_target_index_and_changes_nothing;
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod component {
                        #[path = "."]
                        pub mod binary {
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                            pub mod mutations;
                        }
                        #[path = "."]
                        pub mod text {
                            #[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                            pub mod mutations;
                        }
                    }
                    pub use component::*;
                }
            }
        }
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_process_process3d_model::snapshot::ProcessStep;

use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MutationLeaves
// 🌱️ Every `🧬️mutations/<kind>/` triad leaf is `#[path]`-mounted as a sibling of this dispatch file
// directly in the plugin's `🦀️.rs` (this facet's fan-out ticket, SEMANTIC-MUTATIONS-OVERHAUL
// wave-C, owns `🦀️.rs` for this plugin); `use super::<kind>;` below brings each sibling into
// this file's scope so the enum body can reference `<kind>::<Type>`.
use semio_s_artifact_process_process3d_mut_machines::change_machine_icon;
use semio_s_artifact_process_process3d_mut_steps::change_step_enabled;
use semio_s_artifact_process_process3d_mut_steps::change_step_origin;
use semio_s_artifact_process_process3d_mut_stock::change_stock_label;
use semio_s_artifact_process_process3d_mut_machines::create_machine;
use semio_s_artifact_process_process3d_mut_steps::create_step;
use semio_s_artifact_process_process3d_mut_machines::delete_machine;
use semio_s_artifact_process_process3d_mut_steps::delete_step;
use semio_s_artifact_process_process3d_mut_stock::move_stock;
use semio_s_artifact_process_process3d_mut_machines::rename_machine;
use semio_s_artifact_process_process3d_mut_steps::rename_step;
use semio_s_artifact_process_process3d_mut_steps::reorder_steps;
use semio_s_artifact_process_process3d_mut_machines::replace_machine_capabilities;
use semio_s_artifact_process_process3d_mut_steps::replace_step_measure;
use semio_s_artifact_process_process3d_mut_stock::replace_stock_solid;
//#endregion 🔖️MutationLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the process3d document, derived per
/// `📓️derivation-rules.md` from `Process3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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

//#region 🔖️DocumentHelpers
/// ✂️➕️ Read-only operation builders for the two structural collection edits every mutating command
/// needs: inserting a step at the viewer's replay cursor and removing a step by id. Shared by the
/// `🎮️commands/🪜️step` and `🎮️commands/🌍️world` command modules — building `Process3dMutation`s from an
/// immutable `&Process3dSnapshot` keeps every handler free of manual mutation, since the VCS store applies them.
///
/// ⏱️ The cursor is VIEW state (`Process3dConfig.resolved_up_to`), never a document field: a step drawn while the
/// viewer replays the timeline lands at the cursor (clamped to the timeline), and the caller moves its own cursor
/// with [`process3d_cursor_after_insert`] / [`process3d_cursor_after_remove`] on the config lane.
pub fn insert_step_mutations(snapshot: &semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot, step: ProcessStep, cursor: Option<usize>) -> Vec<crate::Process3dMutation> {
    use crate::Process3dMutation;
    use semio_s_artifact_process_process3d_mut_steps::create_step;
    let index = cursor.map_or(snapshot.step_payloads.len(), |cursor| cursor.min(snapshot.step_payloads.len()));
    vec![Process3dMutation::CreateStep(create_step::CreateStep { index, step })]
}
pub fn remove_step_mutations(snapshot: &semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot, id: &str) -> Option<Vec<crate::Process3dMutation>> {
    use crate::Process3dMutation;
    use semio_s_artifact_process_process3d_mut_steps::delete_step;
    snapshot.step_payloads.iter().any(|step| step.id == id).then(|| vec![Process3dMutation::DeleteStep(delete_step::DeleteStep { id: id.to_string() })])
}
//#endregion 🔖️DocumentHelpers

/// 🧬️ The mutation facet's schema leaves (rust, typescript, graphql, json schema, proto).
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto"), };
