"""🙈️ (ii) for process3d in the overlay: step/machine rows carry ONE target; setStepEnabled requires its value."""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
ED = ROOT / "🦀️.rs"
s = ED.read_text()

def once(text, old, new):
    n = text.count(old)
    assert n == 1, (old[:90], n)
    return text.replace(old, new)

s = once(s, '"setStepEnabled" => Ok(Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled { id: string_field("id").unwrap_or_default(), enabled: field("enabled").and_then(DslValue::as_bool).unwrap_or(true) })),',
         '"setStepEnabled" => Ok(Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled {\n                id: string_field("id").unwrap_or_default(),\n                enabled: field("enabled").and_then(DslValue::as_bool).ok_or_else(|| process3d_action_fault(action, "requires the boolean \'enabled\' it sets"))?,\n            })),')
s = once(s, '            .action_args("setStock", vec![\n',
         '            .action_args("setStepEnabled", vec![\n'
         '                ActionArgDef::text("id", LocalizedLabel::native("Step", "Schritt")).required(),\n'
         '                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),\n'
         '            ])\n'
         '            .action_args("setStock", vec![\n')
s = once(s, '.action_describe("setStepEnabled", LocalizedLabel::native("Turns one process step on or off; a disabled step stays in the timeline but is not applied to the stock.", "Schaltet einen Prozessschritt ein oder aus; ein deaktivierter Schritt bleibt in der Zeitleiste, wird aber nicht auf das Rohteil angewendet."))',
         '.action_describe("setStepEnabled", LocalizedLabel::native("Sets one process step on or off, to exactly the value passed, so repeating it changes nothing; a disabled step stays in the timeline but is not applied to the stock.", "Schaltet einen Prozessschritt genau nach dem übergebenen Wert ein oder aus, sodass eine Wiederholung nichts ändert; ein deaktivierter Schritt bleibt in der Zeitleiste, wird aber nicht auf das Rohteil angewendet."))')
ED.write_text(s)

ST = ROOT / "🎮️commands/🪜️step/🦀️.rs"
t = ST.read_text()
t = once(t, '''    /// 🔘 See `MoveStep::handle`'s doc comment — existence is validated at diff time.
    pub fn handle(
        payload: &SetStepEnabled,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let _ = doc;
        Ok(Emit::mutations(vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: payload.id.clone(), new_enabled: payload.enabled })]))
    }''', '''    /// 🔘 Sets the step to exactly `enabled` — the step row target's explicit next state — and emits nothing when the step
    /// already holds it, so a replayed click leaves the one edit the first one made. An unknown id still reaches the
    /// mutation, whose diff validates existence (see `MoveStep::handle`'s doc comment).
    pub fn handle(
        payload: &SetStepEnabled,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        if doc.snapshot.step_payloads.iter().any(|step| step.id == payload.id && step.enabled == payload.enabled) {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: payload.id.clone(), new_enabled: payload.enabled })]))
    }''')
ST.write_text(t)

AR = ROOT / "📌️panels/🗿️artifact/🦀️.rs"
a = AR.read_text()
a = once(a, "use semio_framework_plugin::{tree_item, ActionBinding, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowAction, RowActionPlacement, Trigger, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};",
         "use semio_framework_plugin::{row_action, row_target, tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowActionPlacement, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};")
a = a.replace("use crate::editor::process3d::process3d_action;\n", "")
start = a.index("/// 🎞️ One step row: its own verbs stay ROW ACTIONS")
end = a.index("/// 📄️ Renders the document tree:")
a = a[:start] + '''/// 🎞️ One step row: ONE target `{enabled, id}` — `enabled` states the INVERSE of the step's current state, so the eye's
/// `setStepEnabled` sets a value and never flips one, and the menu's `removeStep` reads `id` alone — and its verbs stay ROW
/// ACTIONS naming only a verb, while selection is the tree's: the row declares its granularity and the tree carries the
/// single `interactionSelect` binding both it and the canvas pick through.
fn step_row(index: usize, step: &ProcessStep, cursor: usize, labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item(&step.id, crate::editor::process3d::ui_label(&step.label)?)?;
    let args = crate::editor::process3d::ui_value_map([("enabled", crate::editor::process3d::ui_value_bool(!step.enabled)), ("id", crate::editor::process3d::ui_value_text(&step.id)?)])?;
    let target = row_target(crate::editor::process3d::PROCESS_3D_PLAY_CONTROLLER_ID, Some(args), None)?;
    let mut row_actions = semio_framework_plugin::UiFixedList::default();
    row_actions.try_push(row_action(if step.enabled { "eye" } else { "eye-off" }, labels.enabled.as_str(), "setStepEnabled", RowActionPlacement::Row)?).map_err(|_| document_error("row-actions"))?;
    row_actions.try_push(row_action("trash", labels.remove.as_str(), "removeStep", RowActionPlacement::Menu)?).map_err(|_| document_error("row-actions"))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.description = if index >= cursor { Some(ui_text("pending")?) } else { None };
        props.icon = Some(ui_text(process3d_measure_icon(&step.measure))?);
        props.dimmed = Some(!step.enabled);
        props.granularity = Some(ui_text(PROCESS3D_GRANULARITY_OBJECT)?);
        props.row_actions = row_actions;
        props.target = Some(target);
    }
    Ok(item)
}

''' + a[end:]
AR.write_text(a)

WS = ROOT / "📌️panels/🛠️workshop/🦀️.rs"
w = WS.read_text()
w = once(w, "use semio_framework_plugin::{tree_item, ActionBinding, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowAction, RowActionPlacement, Trigger, TreeWindows, UiAssemblyResult};",
         "use semio_framework_plugin::{row_action, row_target, tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, RowActionPlacement, TreeWindows, UiAssemblyResult};")
start = w.index("/// 🛠️ One installed-machine row:")
end = w.index("/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): installed-machine item ids are")
w = w[:start] + '''/// 🛠️ One installed-machine row: ONE target `{id}` and `removeWorkshopMachine` as its own menu row action naming only
/// the verb; selection is the tree's — the row declares the `object` granularity and picks through the tree's single
/// `interactionSelect` binding.
fn machine_row(machine: &WorkshopMachine, labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let args = crate::editor::process3d::ui_value_map([("id", crate::editor::process3d::ui_value_text(&machine.id)?)])?;
    let target = row_target(crate::editor::process3d::PROCESS_3D_PLAY_CONTROLLER_ID, Some(args), None)?;
    let mut row_actions = semio_framework_plugin::UiFixedList::default();
    row_actions.try_push(row_action("trash", labels.remove_machine.as_str(), "removeWorkshopMachine", RowActionPlacement::Menu)?).map_err(|_| workshop_error("row-actions"))?;
    let mut item = tree_item(format!("machine:{}", machine.id), crate::editor::process3d::ui_label(&machine.label)?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(ui_text(&machine.icon_id)?);
        props.granularity = Some(ui_text(PROCESS3D_GRANULARITY_OBJECT)?);
        props.row_actions = row_actions;
        props.target = Some(target);
    }
    Ok(item)
}

''' + w[end:]
WS.write_text(w)
print("ok")
