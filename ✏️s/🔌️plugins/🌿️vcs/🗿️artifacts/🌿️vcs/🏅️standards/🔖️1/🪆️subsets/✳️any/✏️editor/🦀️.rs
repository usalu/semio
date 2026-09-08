//! 🖥️ VCS editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, headless compute in the artifact's `🧬️schema` (dissolved from `⚙️engine` per ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). This file is a routing table: `handle` →
//! `VcsCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::{op::VcsDemoMutation, VcsSnapshot, VCS_DOCUMENT_SCHEMA};
use crate::editor::vcs::commands::edit as edit_command;
use crate::editor::vcs::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, increment_counter, no_operation, patch_snapshot, text_edit};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::editor::vcs::modes::edit;
use crate::editor::vcs::modes::edit::windows::{editor, history};
use crate::editor::vcs::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::vcs::presence::{VcsDemoPresence, VcsDemoPresenceMutation};
use crate::editor::vcs::terminology::vcs_play_labels;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider,
    HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
};
use std::collections::BTreeSet;
use store::EngineHandles;

//#region 🔖️Constants
pub const VCS_PLAY_APP_ID: &str = "vcs-play";
pub use document_panel::VCS_PLAY_BODY_DOCUMENT;
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
pub fn ui_fixed_label(label: semio_framework_plugin::LabelText) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(label.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI label admission failed"))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

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
        "patchSnapshot" as "patch-snapshot" => patch_snapshot::PatchSnapshot,
        "textEdit" as "text-edit" => text_edit::TextEdit,
        "edit" as "edit" => edit_command::Edit,
        "noMutation" as "no-operation" => no_operation::NoMutation,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
    }
}
//#endregion 🔖️Commands

//#region 🔖️DocumentHelpers
// 🌱️ `seed_vcs_demo_history` (test-only demo history seeding) now lives in the `🔖️Testkit` region
// below — it must dispatch through `VcsArtifactApp`'s public surface (`dispatch_typed`/
// `handle_action`), not a raw `store::ArtifactStore`, since `ArtifactApp::seed(&mut ArtifactStore)`
// (this app's old direct-store-touch hook) no longer exists on the trait as of ticket
// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4 (`ArtifactApp::genesis() -> Vec<Self::Mutation>`
// replaced it, but `genesis` can only emit flat document mutations — it has no way to express
// `CommitCheckpoint`/`CreateAlternative`/`SwitchAlternative`, so it cannot reconstruct branching
// checkpoint history at construction time). Consequence: this demo's rich seeded history is reachable
// from tests (`testkit::app`/`app_with_registry` seed it explicitly) but no longer auto-populates a
// freshly constructed production instance the way `ArtifactApp::seed` used to — restoring that would
// need a framework-level hook `genesis` doesn't provide, which is out of this plugin's boundary
// (`🔌️plugin/🦀️.rs` is W1-owned).
//#endregion 🔖️DocumentHelpers

//#region 🔖️VcsPlayApp
/// 🧪️ B1: unit struct — the former `VcsPlayApp::selected_checkpoint_ids` `RefCell` field passed through
/// `crate::editor::vcs::config::VcsDemoConfig` before becoming the framework-owned "history" interaction
/// domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, see `VCS_INTERACTION_HISTORY`'s
/// doc comment); `locale` is the only field left in `Config`, written through `VcsDemoConfigMutation`s.
#[derive(Default)]
pub struct VcsPlayApp;

//#region 🧵️RetainedCommands
const VCS_BOUNDED_TOOL_IDS: &[&str] = &["incrementCounter", "patchSnapshot", "noMutation", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasWheel"];
const VCS_RESUMABLE_TOOL_IDS: &[&str] = &["textEdit", "edit"];
const VCS_BOUNDED_PAYLOAD_SCHEMA: &str = "vcs.vcs.tool-command.v1";
const VCS_BOUNDED_RAW_BYTES: usize = 8_192;
const VCS_BOUNDED_WORK_ITEMS: usize = 1;
const VCS_EDIT_MAXIMUM_TAGS: usize = 4_096;
const VCS_EDIT_MAXIMUM_OUTPUT_BYTES: usize = 16_384;
const VCS_EDIT_MAXIMUM_WORK_ITEMS: usize = 16_400;
const VCS_BOUNDED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "incrementCounter", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSnapshot", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "noMutation", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasWheel", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];
const VCS_RESUMABLE_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn vcs_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(VCS_BOUNDED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

fn vcs_resumable_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(VCS_BOUNDED_RAW_BYTES, VCS_EDIT_MAXIMUM_WORK_ITEMS, 1, VCS_EDIT_MAXIMUM_OUTPUT_BYTES, 7_500, 1, 1)
}

fn vcs_bounded_extent(command: &VcsCommand, _snapshot: &VcsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        VcsCommand::IncrementCounter(_) | VcsCommand::NoMutation(_) | VcsCommand::CanvasPointerDown(_) | VcsCommand::CanvasPointerMove(_) | VcsCommand::CanvasPointerUp(_) | VcsCommand::CanvasWheel(_) => 0,
        VcsCommand::PatchSnapshot(payload) => payload.field.len().checked_add(payload.value.len())?,
        VcsCommand::TextEdit(_) | VcsCommand::Edit(_) => return None,
    };
    (bytes <= VCS_BOUNDED_RAW_BYTES).then_some(VCS_BOUNDED_WORK_ITEMS)
}

fn vcs_bounded_reduce(
    command: &VcsCommand,
    snapshot: &VcsSnapshot,
    config: &VcsDemoConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<EditorApp<VcsPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
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
        use crate::mutations::{add_tag, change_counter, change_notes, change_status, remove_tag, rename_vcs};
        match self.phase {
            VcsEditPhase::Decode => {
                let text = vcs_edit_text(command).ok_or_else(|| Fault::from("vcs-edit-command-mismatch"))?;
                if text.len() > VCS_BOUNDED_RAW_BYTES || snapshot.tags.len() > VCS_EDIT_MAXIMUM_TAGS {
                    return Err(Fault::from("vcs-edit-input-capacity"));
                }
                match dsl::json::from_json_str::<VcsSnapshot>(text) {
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
                    self.phase = VcsEditPhase::Additions;
                }
            }
            VcsEditPhase::Additions => {
                let next = self.next.as_ref().ok_or_else(|| Fault::from("vcs-edit-next-snapshot-absent"))?;
                if let Some(tag) = next.tags.get(self.cursor) {
                    let mutation = (!self.current_tags.contains(tag.as_str())).then(|| add_tag(tag.clone()));
                    if let Some(mutation) = mutation {
                        self.charge_output(tag.len())?;
                        self.mutations.push(mutation);
                    }
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
                    self.phase = VcsEditPhase::Complete;
                }
            }
            VcsEditPhase::Complete => {
                if self.complete {
                    return Err(Fault::from("vcs-edit-work-repeated"));
                }
                self.complete = true;
                let mutations = std::mem::take(&mut self.mutations);
                return Ok(Some(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) }));
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

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<VcsPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<VcsPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
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
struct VcsOneItemPreparationFactory<P, M> {
    lane: store::HistoryLane,
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> VcsOneItemPreparationFactory<P, M> {
    fn new(lane: store::HistoryLane) -> Self { Self { lane, marker: std::marker::PhantomData } }
}

struct VcsOneItemPreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn vcs_one_item_edit<M>(forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("vcs-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for VcsOneItemPreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn preflight(&self, _mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != self.lane || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) { return Err("VCS one-item preparation rejected its lane or description envelope".into()); }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != self.lane || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES { return Err(request); }
        Ok(Box::new(VcsOneItemPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for VcsOneItemPreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        let base = self.base.as_ref().ok_or_else(|| "VCS one-item preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "VCS one-item preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "VCS one-item preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(vcs_one_item_edit(mutation, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("VCS one-item preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            let bytes = authority.actor().len();
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none() }
}
//#endregion 📬️StorePreparation

//#region 🧾️ProofCatalogs
struct VcsBoundedProofs;
impl VcsBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<VcsPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.vcs.vcs@1/*#editor",
        document_schema: "vcs.vcs",
        factory: "VcsBoundedCommandJobFactory",
        factory_type: VcsBoundedCommandJobFactory,
        tools: {
            "incrementCounter" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "patchSnapshot" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "noMutation" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerDown" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerMove" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerUp" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasWheel" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }
}

struct VcsResumableProofs;
impl VcsResumableProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<VcsPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.vcs.vcs@1/*#editor",
        document_schema: "vcs.vcs",
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
            return Err(Fault::from("vcs-command-tool-mismatch"));
        }
        let extent = if bounded { vcs_bounded_extent(&request.command, &request.snapshot, &request.interaction_state) } else { vcs_edit_extent(&request.command, &request.snapshot, &request.interaction_state) };
        if extent.is_none() {
            return Err(Fault::from("vcs-command-payload-too-large"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = if bounded { Box::new(BoundedArtifactCommandWork::new(tool_id, vcs_bounded_reduce, vcs_bounded_extent)) } else { Box::new(VcsEditCommandWork::new(tool_id)) };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
            VcsCommand::command_id,
            VCS_BOUNDED_RAW_BYTES,
            if bounded { VCS_BOUNDED_WORK_ITEMS } else { VCS_EDIT_MAXIMUM_WORK_ITEMS },
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::vcs::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> VcsSnapshot {
        crate::standards::v1::subsets::any::schema::empty_vcs_snapshot()
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    fn command_id(command: &VcsCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(dsl::DslValue::Null);
        let text_arg = |key: &str| args.get(key).and_then(dsl::DslValue::as_str).unwrap_or_default().to_string();
        match action {
            "incrementCounter" => Ok(VcsCommand::IncrementCounter(increment_counter::IncrementCounter {})),
            "patchSnapshot" => {
                let field = text_arg("field");
                let value = text_arg("value");
                if field.len().checked_add(value.len()).is_none_or(|bytes| bytes > VCS_BOUNDED_RAW_BYTES) {
                    return Err(Fault::from("vcs-command-payload-too-large"));
                }
                Ok(VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field, value }))
            }
            "textEdit" => {
                let text = text_arg("text");
                if text.len() > VCS_BOUNDED_RAW_BYTES {
                    return Err(Fault::from("vcs-command-payload-too-large"));
                }
                Ok(VcsCommand::TextEdit(text_edit::TextEdit { text }))
            }
            "edit" => {
                let text = text_arg("text");
                if text.len() > VCS_BOUNDED_RAW_BYTES {
                    return Err(Fault::from("vcs-command-payload-too-large"));
                }
                Ok(VcsCommand::Edit(edit_command::Edit { text }))
            }
            "noMutation" => Ok(VcsCommand::NoMutation(no_operation::NoMutation {})),
            "canvasPointerDown" => Ok(VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {})),
            "canvasPointerMove" => Ok(VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {})),
            "canvasPointerUp" => Ok(VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {})),
            "canvasWheel" => Ok(VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {})),
            other => Err(Fault::from(format!("unknown VCS app action '{other}'"))),
        }
    }

    fn handle(
        command: &VcsCommand,
        doc: &ArtifactView<'_, VcsSnapshot>,
        cfg: &ConfigView<'_, VcsDemoConfig>,
        _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, VcsSnapshot>, cfg: &ConfigView<'_, VcsDemoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = vcs_play_labels(view_state);
        match body_key {
            VCS_PLAY_BODY_EDITOR => editor::render(doc.snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_HISTORY => history::render(doc.history).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_DOCUMENT => document_panel::render(doc.history, labels).map(semio_framework_plugin::built_to_component_tree),
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
            .mutation("patchSnapshot", LocalizedLabel::native("Patch Projection", "Projektion aktualisieren"))
            .mutation("textEdit", LocalizedLabel::native("Edit Text", "Text bearbeiten"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("noMutation", LocalizedLabel::native("No-operation", "Keine Aktion"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
            .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"))
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Leinwand-Mausrad"))
            .action_interactive_job("incrementCounter", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("noMutation", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasWheel", InteractiveJobClassification::Migrated)
            .action_interactive_job("textEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("edit", InteractiveJobClassification::Migrated)
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
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
