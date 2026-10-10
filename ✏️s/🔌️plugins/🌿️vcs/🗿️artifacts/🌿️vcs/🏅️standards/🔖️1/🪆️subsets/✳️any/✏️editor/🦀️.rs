//! 🖥️ VCS editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, headless compute in the artifact's `🧬️schema` (dissolved from `⚙️engine` per ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). This file is a routing table: `handle` →
//! `VcsCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::editor::vcs::commands::edit as edit_command;
use crate::editor::vcs::commands::example::set_active_example;
use crate::editor::vcs::commands::{change_counter, change_notes, change_status, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, increment_counter, no_operation, rename_vcs, text_edit};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::editor::vcs::modes::edit;
use crate::editor::vcs::modes::edit::windows::{editor, history};
use crate::editor::vcs::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::vcs::presence::{VcsDemoPresence, VcsDemoPresenceMutation};
use crate::editor::vcs::terminology::vcs_play_labels;
use crate::{op::VcsDemoMutation, VcsSnapshot, VCS_DOCUMENT_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use std::collections::BTreeSet;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants
pub const VCS_PLAY_APP_ID: &str = "vcs-play";
pub use document_panel::VCS_PLAY_BODY_ARTIFACT;
pub use editor::VCS_PLAY_BODY_EDITOR;
pub use history::VCS_PLAY_BODY_HISTORY;
pub use inspection_panel::VCS_PLAY_BODY_INSPECTION;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎭️modes/*/🪟️windows/*`) builds its `on_change`/item actions with.
pub fn vcs_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(VCS_PLAY_APP_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🏷️ Resolves an `app_labels!`-checked `LabelText` (locale/terminology already folded) into the UI
/// contract's own fixed-capacity `Label` — the contract type deliberately has no `From<LabelText>`,
/// so every chrome node in this surface bridges here.
pub fn ui_fixed_label(label: semio_framework_ui_locale::LabelText) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(label.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI label admission failed"))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub use semio_framework_plugin::ui_node_list;

//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "history" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: multi-select highlighting over the seeded checkpoint history, `Flat` (checkpoints
/// have no selectable-entity nesting — the DAG's `parent_id` links only matter to the swimlane graph
/// layout, not to selection range/closure), one granularity `"commit"`. Distinct from the per-row
/// `checkoutCheckpoint`/`switchAlternative` click actions the document tree already declares (those are
/// navigation — they change the working checkpoint/alternative — not entity selection), which stay as
/// ordinary actions.
pub const VCS_INTERACTION_HISTORY: &str = "history";
//#endregion 🔖️Interaction

//#region 📚️ExampleDocument
pub const VCS_APP_ID: &str = "s.vcs.vcs@1/*#editor";

/// 🧬️ Whole-document replace is banned from the `Mutation` enum, so the example switch builds an
/// `Effect::LoadDocument` — the same lane `🗒️note`/`✒️writer`/`🏛️architect` use. The spr comes from
/// `store::empty_document_spr`, never from a minted `ArtifactEnvelope`: an envelope is a terminal
/// store shell whose `Drop` asserts its bounded retirement authority detached every nested owner
/// first, so building the effect that way panics (ticket 26/09/18, B1a fix #7).
pub fn vcs_example_document_effect() -> semio_framework_plugin::Effect {
    let pack = <VcsSnapshot as store::ArtifactPack>::encode_pack(&crate::examples::demo::snapshot());
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr(VCS_APP_ID, VCS_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 📚️ExampleDocument

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `VcsPlayApp::Command` — the SOLE dispatch surface for the vcs demo app's own behavior. The six
    /// history actions (undo/redo/commitCheckpoint/createAlternative/switchAlternative/checkoutCheckpoint)
    /// never reach here — `VcsArtifactApp` intercepts those itself as host mechanics, not app behavior
    /// (see `shooting_protocol::ShootingCommand`'s identical doc). Field shapes mirror each action's old
    /// JSON `args` object exactly. **Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.**
    pub enum VcsCommand for VcsSnapshot, VcsDemoMutation, VcsDemoConfig, VcsDemoConfigMutation {
        "incrementCounter" as "increment-counter" => increment_counter::IncrementCounter,
        "renameVcs" as "rename-vcs" => rename_vcs::RenameVcs,
        "changeCounter" as "change-counter" => change_counter::ChangeCounter,
        "changeStatus" as "change-status" => change_status::ChangeStatus,
        "changeNotes" as "change-notes" => change_notes::ChangeNotes,
        "textEdit" as "text-edit" => text_edit::TextEdit,
        "edit" as "edit" => edit_command::Edit,
        "noMutation" as "no-operation" => no_operation::NoMutation,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
    }
}
//#endregion 🔖️Commands

//#region 🔖️DocumentHelpers
// 🌱️ `seed_vcs_demo_history` (test-only demo history seeding) now lives in the `🔖️UnitTests` region
// below — it must dispatch through `VcsArtifactApp`'s public surface (`dispatch_typed`/
// `handle_action`), not a raw `store::ArtifactStore`, since `ArtifactApp::seed(&mut ArtifactStore)`
// (this app's old direct-store-touch hook) no longer exists on the trait as of ticket
// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4 (`ArtifactApp::genesis() -> Vec<Self::Mutation>`
// replaced it, but `genesis` can only emit flat document mutations — it has no way to express
// `CommitCheckpoint`/`CreateAlternative`/`SwitchAlternative`, so it cannot reconstruct branching
// checkpoint history at construction time). Consequence: this demo's rich seeded history is reachable
// from tests (`context::app`/`app_with_registry` seed it explicitly) but no longer auto-populates a
// freshly constructed production instance the way `ArtifactApp::seed` used to — restoring that would
// need a framework-level hook `genesis` doesn't provide, which is out of this plugin's boundary
// (`🔌️plugin/🦀️.rs` is W1-owned).
//#endregion 🔖️DocumentHelpers

//#region 🔖️VcsPlayApp
/// 🧪️ B1: unit struct — the former `VcsPlayApp::selected_checkpoint_ids` `RefCell` field passed through
/// `crate::editor::vcs::config::VcsDemoConfig` before becoming the framework-owned "history" interaction
/// domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, see `VCS_INTERACTION_HISTORY`'s
/// doc comment); the config no longer duplicates host-owned view preferences.
#[derive(Default)]
pub struct VcsPlayApp;

//#region 🧵️RetainedCommands
const VCS_BOUNDED_TOOL_IDS: &[&str] = &["incrementCounter", "renameVcs", "changeCounter", "changeStatus", "changeNotes", "noMutation", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasWheel", "setActiveExample"];
const VCS_RESUMABLE_TOOL_IDS: &[&str] = &["textEdit", "edit"];
const VCS_BOUNDED_PAYLOAD_SCHEMA: &str = "vcs.vcs.tool-command.v1";
const VCS_BOUNDED_RAW_BYTES: usize = 8_192;
const VCS_BOUNDED_WORK_ITEMS: usize = 1;
const VCS_EDIT_MAXIMUM_TAGS: usize = 4_096;
const VCS_EDIT_MAXIMUM_OUTPUT_BYTES: usize = 16_384;
const VCS_EDIT_MAXIMUM_WORK_ITEMS: usize = 16_400;
const VCS_BOUNDED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "incrementCounter", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "renameVcs", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "changeCounter", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "changeStatus", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "changeNotes", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "noMutation", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasWheel", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];
const VCS_RESUMABLE_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] =
    &[ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[ArtifactToolPublicationLane::Artifact] }, ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::Artifact] }];

fn vcs_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(VCS_BOUNDED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

fn vcs_resumable_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(VCS_BOUNDED_RAW_BYTES, VCS_EDIT_MAXIMUM_WORK_ITEMS, 1, VCS_EDIT_MAXIMUM_OUTPUT_BYTES, 7_500, 1, 1)
}

fn vcs_bounded_extent(command: &VcsCommand, _snapshot: &VcsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        VcsCommand::IncrementCounter(_) | VcsCommand::NoMutation(_) | VcsCommand::CanvasPointerDown(_) | VcsCommand::CanvasPointerUp(_) | VcsCommand::CanvasWheel(_) => 0,
        // 🧵️ A batched move carries `samples.len()` pairs of f64 (design L4) — priced, never dropped.
        VcsCommand::CanvasPointerMove(payload) => payload.samples.len().checked_mul(2 * size_of::<f64>())?,
        VcsCommand::RenameVcs(payload) => payload.title.len(),
        VcsCommand::ChangeCounter(_) => 0,
        VcsCommand::ChangeStatus(payload) => payload.status.len(),
        VcsCommand::ChangeNotes(payload) => payload.notes.len(),
        VcsCommand::SetActiveExample(payload) if payload.example_id.len() <= 256 => 0,
        VcsCommand::SetActiveExample(_) => return None,
        VcsCommand::TextEdit(_) | VcsCommand::Edit(_) => return None,
    };
    (bytes <= VCS_BOUNDED_RAW_BYTES).then_some(VCS_BOUNDED_WORK_ITEMS)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn vcs_bounded_reduce(
    command: &VcsCommand,
    snapshot: &VcsSnapshot,
    config: &VcsDemoConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<VcsPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

fn vcs_edit_text(command: &VcsCommand) -> Option<&str> {
    match command {
        VcsCommand::TextEdit(payload) => Some(&payload.text),
        VcsCommand::Edit(payload) => Some(&payload.text),
        _ => None,
    }
}

fn vcs_edit_extent(command: &VcsCommand, snapshot: &VcsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let text = vcs_edit_text(command)?;
    (text.len() <= VCS_BOUNDED_RAW_BYTES && snapshot.tags.len() <= VCS_EDIT_MAXIMUM_TAGS).then_some(VCS_EDIT_MAXIMUM_WORK_ITEMS)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VcsEditPhase {
    Decode,
    Reserve,
    Scalars,
    CurrentIndex,
    NextIndex,
    Additions,
    Removals,
    Complete,
}

struct VcsEditCommandWork {
    tool_id: &'static str,
    phase: VcsEditPhase,
    cursor: usize,
    next: Option<VcsSnapshot>,
    current_tags: BTreeSet<String>,
    next_tags: BTreeSet<String>,
    mutations: Vec<VcsDemoMutation>,
    output_bytes: usize,
    steps: u64,
    replay_target: u64,
    complete: bool,
    closing: bool,
}

impl VcsEditCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, phase: VcsEditPhase::Decode, cursor: 0, next: None, current_tags: BTreeSet::new(), next_tags: BTreeSet::new(), mutations: Vec::new(), output_bytes: 0, steps: 0, replay_target: 0, complete: false, closing: false }
    }

    fn charge_output(&mut self, bytes: usize) -> Result<(), Fault> {
        self.output_bytes = self.output_bytes.checked_add(bytes).filter(|total| *total <= VCS_EDIT_MAXIMUM_OUTPUT_BYTES).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
        Ok(())
    }

    fn advance(&mut self, command: &VcsCommand, snapshot: &VcsSnapshot) -> Result<Option<Emit<VcsDemoMutation, VcsDemoConfigMutation, NoDraftMutation>>, Fault> {
        use crate::mutations::{add_tag_at, change_counter, change_notes, change_status, remove_tag, rename_vcs};
        match self.phase {
            VcsEditPhase::Decode => {
                let text = vcs_edit_text(command).ok_or_else(|| Fault::from("vcs-edit-command-mismatch"))?;
                if text.len() > VCS_BOUNDED_RAW_BYTES || snapshot.tags.len() > VCS_EDIT_MAXIMUM_TAGS {
                    return Err(Fault::from("vcs-edit-input-capacity"));
                }
                match semio_framework_pack_json::from_json_str::<VcsSnapshot>(text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                    Ok(next) if next.tags.len() <= VCS_EDIT_MAXIMUM_TAGS => {
                        self.next = Some(next);
                        self.phase = VcsEditPhase::Reserve;
                    }
                    Ok(_) => return Err(Fault::from("vcs-edit-tag-capacity")),
                    Err(_) => self.phase = VcsEditPhase::Complete,
                }
            }
            VcsEditPhase::Reserve => {
                let next_tags = self.next.as_ref().map_or(0, |next| next.tags.len());
                let capacity = snapshot.tags.len().checked_add(next_tags).and_then(|count| count.checked_add(4)).ok_or_else(|| Fault::from("vcs-edit-mutation-capacity"))?;
                self.mutations.try_reserve_exact(capacity).map_err(|_| Fault::from("vcs-edit-mutation-capacity"))?;
                self.phase = VcsEditPhase::Scalars;
            }
            VcsEditPhase::Scalars => {
                let next = self.next.as_ref().ok_or_else(|| Fault::from("vcs-edit-next-snapshot-absent"))?;
                let mut staged = Vec::with_capacity(4);
                let mut bytes = 0_usize;
                if next.title != snapshot.title {
                    bytes = bytes.checked_add(next.title.len()).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
                    staged.push(rename_vcs(next.title.clone()));
                }
                if next.counter != snapshot.counter {
                    staged.push(change_counter(next.counter));
                }
                if next.status != snapshot.status {
                    bytes = bytes.checked_add(next.status.len()).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
                    staged.push(change_status(next.status.clone()));
                }
                if next.notes != snapshot.notes {
                    bytes = bytes.checked_add(next.notes.len()).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
                    staged.push(change_notes(next.notes.clone()));
                }
                self.charge_output(bytes)?;
                self.mutations.extend(staged);
                self.cursor = 0;
                self.phase = VcsEditPhase::CurrentIndex;
            }
            VcsEditPhase::CurrentIndex => {
                if let Some(tag) = snapshot.tags.get(self.cursor) {
                    if tag.len() > VCS_BOUNDED_RAW_BYTES {
                        return Err(Fault::from("vcs-edit-current-tag-capacity"));
                    }
                    self.current_tags.insert(tag.clone());
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.phase = VcsEditPhase::NextIndex;
                }
            }
            VcsEditPhase::NextIndex => {
                let next = self.next.as_ref().ok_or_else(|| Fault::from("vcs-edit-next-snapshot-absent"))?;
                if let Some(tag) = next.tags.get(self.cursor) {
                    if tag.len() > VCS_BOUNDED_RAW_BYTES {
                        return Err(Fault::from("vcs-edit-next-tag-capacity"));
                    }
                    self.next_tags.insert(tag.clone());
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.phase = VcsEditPhase::Removals;
                }
            }
            VcsEditPhase::Removals => {
                if let Some(tag) = snapshot.tags.get(self.cursor) {
                    let mutation = (!self.next_tags.contains(tag.as_str())).then(|| remove_tag(tag.clone()));
                    if let Some(mutation) = mutation {
                        self.charge_output(tag.len())?;
                        self.mutations.push(mutation);
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.phase = VcsEditPhase::Additions;
                }
            }
            VcsEditPhase::Additions => {
                let next = self.next.as_ref().ok_or_else(|| Fault::from("vcs-edit-next-snapshot-absent"))?;
                if let Some(tag) = next.tags.get(self.cursor) {
                    let mutation = (!self.current_tags.contains(tag.as_str())).then(|| add_tag_at(tag.clone(), self.cursor as u32));
                    if let Some(mutation) = mutation {
                        self.charge_output(tag.len())?;
                        self.mutations.push(mutation);
                    }
                    self.cursor += 1;
                } else {
                    self.phase = VcsEditPhase::Complete;
                }
            }
            VcsEditPhase::Complete => {
                if self.complete {
                    return Err(Fault::from("vcs-edit-work-repeated"));
                }
                self.complete = true;
                let mutations = std::mem::take(&mut self.mutations);
                return Ok(Some(Emit::mutations(mutations)));
            }
        }
        Ok(None)
    }

    fn mutation_bytes(mutation: &VcsDemoMutation) -> usize {
        match mutation {
            VcsDemoMutation::RenameVcs(payload) => payload.new_title.len(),
            VcsDemoMutation::ChangeCounter(_) => 0,
            VcsDemoMutation::ChangeNotes(payload) => payload.new_notes.len(),
            VcsDemoMutation::ChangeStatus(payload) => payload.new_status.len(),
            VcsDemoMutation::AddTag(payload) => payload.tag.len(),
            VcsDemoMutation::RemoveTag(payload) => payload.tag.len(),
        }
    }
}

impl ArtifactCommandWork<EditorApp<VcsPlayApp>> for VcsEditCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &VcsCommand, snapshot: &VcsSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<VcsPlayApp>>>) -> Option<usize> {
        vcs_edit_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<VcsPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<VcsPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
        let replaying = self.steps < self.replay_target;
        match self.advance(command, snapshot)? {
            Some(_emit) if replaying => Err(Fault::from("vcs-edit-checkpoint-beyond-completion")),
            Some(emit) => Ok(ArtifactCommandWorkStep::Complete(emit)),
            None => {
                self.steps = self.steps.checked_add(1).ok_or_else(|| Fault::from("vcs-edit-step-overflow"))?;
                if self.steps > VCS_EDIT_MAXIMUM_WORK_ITEMS as u64 {
                    return Err(Fault::from("vcs-edit-work-capacity"));
                }
                if replaying {
                    Ok(ArtifactCommandWorkStep::Replay { stage: "vcs-edit-replay", preview: b"{\"en\":\"Restoring text edit\",\"de\":\"Textbearbeitung wird wiederhergestellt\"}" })
                } else {
                    Ok(ArtifactCommandWorkStep::Progress { stage: "vcs-edit-diff", preview: b"{\"en\":\"Comparing text edit\",\"de\":\"Textbearbeitung wird verglichen\"}" })
                }
            }
        }
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 16 {
            return Err(Fault::from("vcs-edit-checkpoint-capacity"));
        }
        target[..16].fill(0);
        target[..4].copy_from_slice(b"VEC1");
        target[8..16].copy_from_slice(&self.steps.max(self.replay_target).to_le_bytes());
        Ok(16)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 16 || &checkpoint[..4] != b"VEC1" || checkpoint[4..8] != [0; 4] {
            return Err(Fault::from("vcs-edit-checkpoint-invalid"));
        }
        if self.next.is_some() || !self.current_tags.is_empty() || !self.next_tags.is_empty() || !self.mutations.is_empty() {
            return Err(Fault::from("vcs-edit-checkpoint-workspace-not-empty"));
        }
        let target = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("vcs-edit-checkpoint-cursor"))?);
        if target > VCS_EDIT_MAXIMUM_WORK_ITEMS as u64 {
            return Err(Fault::from("vcs-edit-checkpoint-cursor"));
        }
        self.phase = VcsEditPhase::Decode;
        self.cursor = 0;
        self.output_bytes = 0;
        self.steps = 0;
        self.replay_target = target;
        self.complete = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(mutation) = self.mutations.last() {
            let bytes = Self::mutation_bytes(mutation);
            if bytes > maximum_bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.mutations.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(tag) = self.current_tags.first() {
            let bytes = tag.len();
            if bytes > maximum_bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.current_tags.pop_first();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(tag) = self.next_tags.first() {
            let bytes = tag.len();
            if bytes > maximum_bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.next_tags.pop_first();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(next) = self.next.as_mut() {
            if let Some(tag) = next.tags.last() {
                let bytes = tag.len();
                if bytes > maximum_bytes {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                next.tags.pop();
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
            }
            let bytes = next.schema.len().saturating_add(next.title.len()).saturating_add(next.notes.len()).saturating_add(next.status.len());
            if bytes > maximum_bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.next = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.next.is_none() && self.current_tags.is_empty() && self.next_tags.is_empty() && self.mutations.is_empty()
    }
}

struct VcsBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl VcsBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: VCS_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for VcsBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<VcsPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<VcsPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        VCS_BOUNDED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        vcs_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > VCS_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("VCS bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for VcsBoundedCommandJobFactory {
    type Owner = EditorApp<VcsPlayApp>;
    const TOOL_IDS: &'static [&'static str] = VCS_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = VCS_BOUNDED_PUBLICATION_CONTRACTS;
}

struct VcsResumableCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl VcsResumableCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: VCS_RESUMABLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for VcsResumableCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<VcsPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<VcsPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        VCS_BOUNDED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        vcs_resumable_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > VCS_BOUNDED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("VCS resumable command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for VcsResumableCommandJobFactory {
    type Owner = EditorApp<VcsPlayApp>;
    const TOOL_IDS: &'static [&'static str] = VCS_RESUMABLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = VCS_RESUMABLE_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct VcsOneItemPreparationFactory<P, M> {
    lane: store::HistoryLane,
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> VcsOneItemPreparationFactory<P, M> {
    fn new(lane: store::HistoryLane) -> Self {
        Self { lane, marker: std::marker::PhantomData }
    }
}

struct VcsOneItemPreparation<P, M> {
    base: std::mem::ManuallyDrop<Option<store::SnapshotRead<P>>>,

    mutation: std::mem::ManuallyDrop<Option<M>>,

    inverse: std::mem::ManuallyDrop<Option<Vec<M>>>,
    refused: std::mem::ManuallyDrop<Option<(protocol::Edit<M>, std::sync::Arc<P>)>>,
    apply_refusal: std::mem::ManuallyDrop<Option<protocol::MutationApplyError>>,
    authority: std::mem::ManuallyDrop<Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>>,

    prepared: std::mem::ManuallyDrop<Option<store::ArtifactStoreOneItemPrepared<P, M>>>,

    mutation_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>>>,
    snapshot_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<P>>>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
}

impl<P, M> VcsOneItemPreparation<P, M>
where P: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static, M: semio_framework_value::retirement::RetireOwned + Send + 'static,
{
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return nested(store::artifact_retirement_box_demands(active, body)?); }
        if self.prepared.is_some() { let birth = store::ArtifactStoreOneItemPrepared::<P, M>::retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.mutation)?); }
        if self.inverse.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.inverse)?); }
        if self.refused.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.refused)?); }
        if self.apply_refusal.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.apply_refusal)?); }
        if self.base.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.base)?); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for VcsOneItemPreparationFactory<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn preflight(&self, mutation: &M, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != self.lane {
            return Err("VCS one-item preparation rejected its lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &M, _lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand {capacity_bytes:std::mem::size_of::<VcsOneItemPreparation<P,M>>(),depth:1})
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand=match self.begin_demand(&request.mutation,request.lane){Ok(demand)=>demand,Err(error)=>return Err((error,request))};
        let progress=match demand.admit(grant.retained_grant()){Ok(progress)=>progress,Err(error)=>return Err((error,request))};
        if request.lane != self.lane
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preparation rejected original publication authority"),request));
        }
        Ok((Box::new(VcsOneItemPreparation {
            base: std::mem::ManuallyDrop::new(Some(request.base)),
            mutation: std::mem::ManuallyDrop::new(Some(request.mutation)),
            inverse: std::mem::ManuallyDrop::new(None),
            refused: std::mem::ManuallyDrop::new(None),
            apply_refusal: std::mem::ManuallyDrop::new(None),
            authority: std::mem::ManuallyDrop::new(Some(request.authority)),
            prepared: std::mem::ManuallyDrop::new(None),
            mutation_retirement: std::mem::ManuallyDrop::new(Some(request.mutation_retirement)),
            snapshot_retirement: std::mem::ManuallyDrop::new(Some(request.snapshot_retirement)),
            active: std::mem::ManuallyDrop::new(None),
            factories: std::mem::ManuallyDrop::new(Default::default()),
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }),progress))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for VcsOneItemPreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.refused.is_some() || self.apply_refusal.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let authority = self.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS one-item preparation lost its Store authority"))?;
                let base = self.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS one-item preparation lost its exact base root"))?;
        let mutation = self.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS one-item preparation lost its mutation owner"))?;
        let inverse = match mutation.inverse(base.get()) {
                    Ok(inverse) => inverse,
                    Err(error) => { *self.mutation = Some(mutation); return Err(error); }
                };
        let post = match protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()) {
                    Ok(post) => post,
                    Err(error) => {
                        *self.mutation = Some(mutation);
                        *self.inverse = Some(inverse);
                        *self.apply_refusal = Some(error);
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
                    }
                };

        let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => { *self.refused = Some((edit, post)); return Err(error); }
                };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
        let empty = RetainedCloneProgress::default(); let grant = grant.retained_grant();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close exceeds original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return store::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.prepared.is_some() {
            if self.mutation_retirement.is_none() || self.snapshot_retirement.is_none() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation retains its original installed issuers")); }
            let original = self.prepared.take().expect("observed original prepared candidate");
            let mutations = self.mutation_retirement.take().expect("original mutation issuer");
            let snapshots = self.snapshot_retirement.take().expect("original snapshot issuer");
            return match original.admit_retirement(mutations, snapshots, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original prepared close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation child changed its actual admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original, mutations, snapshots)) => { *self.prepared = Some(original); *self.mutation_retirement = Some(mutations); *self.snapshot_retirement = Some(snapshots); Err(error) },
            };
        }
        if self.mutation.is_some() { return store::artifact_retirement_admit_owned(&mut self.mutation, &mut self.active, child); }
        if self.inverse.is_some() { return store::artifact_retirement_admit_owned(&mut self.inverse, &mut self.active, child); }
        if self.refused.is_some() { return store::artifact_retirement_admit_owned(&mut self.refused, &mut self.active, child); }
        if self.apply_refusal.is_some() { return store::artifact_retirement_admit_owned(&mut self.apply_refusal, &mut self.active, child); }
        if self.base.is_some() { return store::artifact_retirement_admit_owned(&mut self.base, &mut self.active, child); }
        if let Some(authority) = self.authority.take() { return match authority.retire(child) { Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original preparation authority close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation authority changed admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) }, Err((error, original)) => { *self.authority = Some(original); Err(error) } }; }
        if let Some(factory) = self.mutation_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(factory) = self.snapshot_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(slot) = self.factories.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().expect("original preparation factory alias"); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original preparation factory close")?; if factory.terminal_is_empty() { *slot = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.active.is_none() && self.factories.iter().all(Option::is_none) && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

impl<P, M> Drop for VcsOneItemPreparation<P, M> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.active.is_none() && self.factories.iter().all(Option::is_none)), "preparation must retain original owners until supplied-grant terminal closure"); }
}
//#endregion 📬️StorePreparation

//#region 🧾️ProofCatalogs
struct VcsBoundedProofs;
impl VcsBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<VcsPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.vcs.vcs@1/*#editor",
        artifact_schema: "vcs.vcs",
        factory: "VcsBoundedCommandJobFactory",
        factory_type: VcsBoundedCommandJobFactory,
        tools: {
            "incrementCounter" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "renameVcs" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "changeCounter" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "changeStatus" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "changeNotes" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "noMutation" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerDown" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerMove" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerUp" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasWheel" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setActiveExample" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }
}

struct VcsResumableProofs;
impl VcsResumableProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<VcsPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.vcs.vcs@1/*#editor",
        artifact_schema: "vcs.vcs",
        factory: "VcsResumableCommandJobFactory",
        factory_type: VcsResumableCommandJobFactory,
        tools: {
            "textEdit" => ToolExecutionContract::resumable(8_192, 16_400, 1, 16_384, 7_500, 1, 1),
            "edit" => ToolExecutionContract::resumable(8_192, 16_400, 1, 16_384, 7_500, 1, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs

impl ArtifactEditor for VcsPlayApp {
    type Snapshot = VcsSnapshot;
    type Mutation = VcsDemoMutation;
    type Config = VcsDemoConfig;
    type ConfigMutation = VcsDemoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = VcsDemoPresence;
    type PresenceMutation = VcsDemoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = VcsCommand;

    const DIALECT: Dialect = crate::VCS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;

    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 1]> = std::sync::LazyLock::new(|| [("vcs.command.payload-too-large", LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden."))]);
        NOTICES.as_slice()
    }

    /// 🧺️ Without these owners the document store holds no `initial_snapshot_retirement_factory`, so
    /// the FIRST verb that returns a snapshot read fails validation with `returned snapshot read
    /// requires its exact owned-snapshot retirement factory` — and because that poisons the
    /// instance, every later dispatch fails with `plugin.internal.prior-outcome`. Measured against
    /// the live React playground: all nine of this app's Actions-pane verbs were refused, the first
    /// on the missing factory and the other eight on the prior outcome.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    /// 📝️ Vcs keeps no draft lane, but the framework's close ladder still demands the lane's exact
    /// bounded disposer — without it every close faults `app owner did not provide the required
    /// bounded disposer for draft-store` and the app can never retire.
    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 👥️ Without these the `PresenceStore` holds no local retirement owner, so the FIRST command
    /// whose ephemeral leg reads local presence is refused with `presence local read requires a live
    /// exact local retirement owner` — the `ArtifactEditor` default is `None` and `EditorApp` passes
    /// it through unchanged (`no_presence_*` only types over `NoPresence`, never over a real presence
    /// payload). `VcsDemoPresence` is the empty terminal, so its exact bounded root retirement is the
    /// generic owned-value one, exactly like shooting.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("VcsDemoPresence is the exact empty terminal")))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(VcsOneItemPreparationFactory::new(store::HistoryLane::Document)))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(VcsOneItemPreparationFactory::new(store::HistoryLane::Document)))
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        VcsBoundedProofs::bounded_first_step_tool_proofs().into_iter().chain(VcsResumableProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(VcsBoundedCommandJobFactory::new(&controller))?;
        registry.register(VcsResumableCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let bounded = VCS_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str());
        let resumable = VCS_RESUMABLE_TOOL_IDS.contains(&request.tool_id.as_str());
        if !bounded && !resumable {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "VCS command does not match its exact registered tool"));
        }
        let extent = if bounded { vcs_bounded_extent(&request.command, &request.snapshot, &request.interaction_state) } else { vcs_edit_extent(&request.command, &request.snapshot, &request.interaction_state) };
        if extent.is_none() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = if bounded { Box::new(BoundedArtifactCommandWork::new(tool_id, vcs_bounded_reduce, vcs_bounded_extent)) } else { Box::new(VcsEditCommandWork::new(tool_id)) };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            VcsCommand::command_id,
            VCS_BOUNDED_RAW_BYTES,
            if bounded { VCS_BOUNDED_WORK_ITEMS } else { VCS_EDIT_MAXIMUM_WORK_ITEMS },
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::vcs::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> VcsSnapshot {
        crate::standards::v1::subsets::any::schema::empty_vcs_snapshot()
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    fn command_id(command: &VcsCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(semio_framework_value::DslValue::Null);
        let text_arg = |key: &str| args.get(key).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
        let bounded_text = |key: &str| {
            let text = text_arg(key);
            if text.len() > VCS_BOUNDED_RAW_BYTES {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity"));
            }
            Ok(text)
        };
        let pointer_samples = || {
            args.get("samples")
                .and_then(semio_framework_value::DslValue::as_array)
                .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "canvasPointerMove requires its `samples` batch"))?
                .iter()
                .map(|item| match item.as_array() {
                    Some([x, y]) => x.as_f64().zip(y.as_f64()).map(|(x, y)| [x, y]),
                    _ => None,
                }
                .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "every canvasPointerMove sample is an [x, y] number pair")))
                .collect::<Result<Vec<[f64; 2]>, Fault>>()
        };
        let pointer_cancelled = || args.get("cancelled").and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "canvasPointerUp requires its `cancelled` flag"));
        match action {
            "incrementCounter" => Ok(VcsCommand::IncrementCounter(increment_counter::IncrementCounter {})),
            "setActiveExample" => Ok(VcsCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text_arg("exampleId") })),
            "renameVcs" => Ok(VcsCommand::RenameVcs(rename_vcs::RenameVcs { title: bounded_text("value")? })),
            "changeStatus" => Ok(VcsCommand::ChangeStatus(change_status::ChangeStatus { status: bounded_text("value")? })),
            "changeNotes" => Ok(VcsCommand::ChangeNotes(change_notes::ChangeNotes { notes: bounded_text("value")? })),
            "changeCounter" => bounded_text("value")?
                .trim()
                .parse::<i64>()
                .map(|value| VcsCommand::ChangeCounter(change_counter::ChangeCounter { value }))
                .map_err(|_| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "changeCounter requires an integer `value`")),
            "textEdit" => {
                let text = text_arg("text");
                if text.len() > VCS_BOUNDED_RAW_BYTES {
                    return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity"));
                }
                Ok(VcsCommand::TextEdit(text_edit::TextEdit { text }))
            }
            "edit" => {
                let text = text_arg("text");
                if text.len() > VCS_BOUNDED_RAW_BYTES {
                    return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity"));
                }
                Ok(VcsCommand::Edit(edit_command::Edit { text }))
            }
            "noMutation" => Ok(VcsCommand::NoMutation(no_operation::NoMutation {})),
            "canvasPointerDown" => Ok(VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {})),
            "canvasPointerMove" => Ok(VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: pointer_samples()? })),
            "canvasPointerUp" => Ok(VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: pointer_cancelled()? })),
            "canvasWheel" => Ok(VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {})),
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("unknown VCS app action '{other}'"))),
        }
    }

    fn handle(
        command: &VcsCommand,
        doc: &ArtifactView<'_, VcsSnapshot>,
        cfg: &ConfigView<'_, VcsDemoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = vcs_play_labels(view_state);
        match body_key {
            VCS_PLAY_BODY_EDITOR => editor::render(doc.snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_HISTORY => history::render(doc.history).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_ARTIFACT => document_panel::render(doc.history, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, VCS_PLAY_BODY_ARTIFACT)).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_INSPECTION => inspection_panel::render(doc.snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️VcsPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_vcs_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::VCS_DIALECT)
            .document(["semio", "vcs"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("git-branch")
            .mode_def(edit::definition())
            .default_mode_id(edit::VCS_PLAY_MODE_EDIT)
            .window_kind_def(editor::definition())
            .window_kind_def(history::definition())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("incrementCounter", LocalizedLabel::native("Increment Counter", "Zähler erhöhen"))
            .mutation("renameVcs", LocalizedLabel::native("Rename", "Umbenennen"))
            .mutation("changeCounter", LocalizedLabel::native("Change Counter", "Zähler ändern"))
            .mutation("changeStatus", LocalizedLabel::native("Change Status", "Status ändern"))
            .mutation("changeNotes", LocalizedLabel::native("Change Notes", "Notizen ändern"))
            .action_with(semio_framework_plugin::ActionDefinition::new("textEdit", LocalizedLabel::native("Edit Text", "Text bearbeiten"), semio_framework_plugin::ActionKind::Mutation, "typography"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("noMutation", LocalizedLabel::native("No-operation", "Keine Aktion"))
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Leinwand-Mausrad"))
            .action_audience("canvasWheel", semio_framework_plugin::CapabilityAudience::Input)
            // 📚️ The playground navbar dispatches `setActiveExample` for its fixture combobox on
            // boot; without an app-level declaration the shell drops it before dispatch and the
            // example picker never renders at all.
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .action_interactive_job("incrementCounter", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameVcs", InteractiveJobClassification::Migrated)
            .action_interactive_job("changeCounter", InteractiveJobClassification::Migrated)
            .action_interactive_job("changeStatus", InteractiveJobClassification::Migrated)
            .action_interactive_job("changeNotes", InteractiveJobClassification::Migrated)
            .action_interactive_job("noMutation", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasWheel", InteractiveJobClassification::Migrated)
            .action_interactive_job("textEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("edit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_args("setActiveExample", vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![semio_framework_plugin::ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label())])])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(edit::layout())
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "history" interaction
            // domain — one granularity ("commit"), `HierarchyProvider::Flat` (see `VCS_INTERACTION_HISTORY`'s
            // doc comment for why this is entity selection, not navigation, and why it is Flat). Multi-select
            // via Pick (tree rows) only — no canvas/marquee surface exists for checkpoints — all five merges
            // since the document tree is a plain ordered list (shift-range over the seeded history reads
            // naturally). Replaces the deleted bespoke `setSelection` action/config field/command.
            .interaction(InteractionDefinition {
                id: VCS_INTERACTION_HISTORY.into(),
                label: LocalizedLabel::native("History", "Verlauf"),
                granularities: vec![GranularityDefinition { id: "commit".into(), label: LocalizedLabel::native("Commit", "Commit"), icon_id: "git-commit".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(history::VCS_PLAY_WINDOW_HISTORY, vec![InteractionRef::new(VCS_INTERACTION_HISTORY)])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // this app has no user-visible sticky defaults, so `config_spec()` stays the trait default
            // `ConfigSpec::empty()`; declared anyway for parity with every other converted app.
            .config(VcsPlayApp::config_spec())
            // 🚧️ SDK GAP (contract §2.4): `Editor::builder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so this app never had either call to
            // port (the old `create_vcs_app` had none), noted here anyway for parity with the other W2
            // packets' identical gap note. The subset's own `📚️examples/🎬️demo-session` facet (real
            // content, moved intact) is the modern, role-agnostic replacement surface for this.
            .action_describe("incrementCounter", LocalizedLabel::native("Adds one to the demo document's counter.", "Erhöht den Zähler des Demodokuments um eins."))
            .action_describe("renameVcs", LocalizedLabel::native("Sets the demo document's title.", "Setzt den Titel des Demodokuments."))
            .action_describe("changeCounter", LocalizedLabel::native("Sets the demo document's counter to an integer.", "Setzt den Zähler des Demodokuments auf eine ganze Zahl."))
            .action_describe("changeStatus", LocalizedLabel::native("Sets the demo document's status.", "Setzt den Status des Demodokuments."))
            .action_describe("changeNotes", LocalizedLabel::native("Sets the demo document's notes.", "Setzt die Notizen des Demodokuments."))
            .action_describe("textEdit", LocalizedLabel::native("Reads the given text as the demo document's projection and writes the title, counter, status and notes that differ; a live typing run commits as one edit.", "Liest den angegebenen Text als Projektion des Demodokuments und schreibt abweichenden Titel, Zähler, Status und Notizen; ein fortlaufender Tipplauf wird als eine Änderung übernommen."))
            .action_describe("edit", LocalizedLabel::native("Reads the given text as the demo document's projection and writes every field that differs as one edit.", "Liest den angegebenen Text als Projektion des Demodokuments und schreibt jedes abweichende Feld als eine Änderung."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole demo document with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte Demodokument durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_audience("noMutation", semio_framework_plugin::CapabilityAudience::Chrome)
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
