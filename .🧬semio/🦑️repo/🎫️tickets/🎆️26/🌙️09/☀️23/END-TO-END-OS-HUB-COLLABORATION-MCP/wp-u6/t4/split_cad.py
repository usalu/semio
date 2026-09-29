"""🙈️ (ii) verb split for cad in the overlay: setReferenceHidden/Locked with one reference-row target."""
import json, re, sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any"
ED = ROOT / "✏️editor/🦀️.rs"
s = ED.read_text()

def once(old, new):
    global s
    n = s.count(old)
    assert n == 1, (old[:90], n)
    s = s.replace(old, new)

once('use crate::editor::cad::commands::reference::{patch_cad_play_reference, reference_hover, set_reference_selection};',
     'use crate::editor::cad::commands::reference::{patch_cad_play_reference, reference_hover, set_reference_hidden, set_reference_locked, set_reference_selection};')
once('        "patchCadPlayReference" as "patch-cad-play-reference" => patch_cad_play_reference::PatchCadPlayReference,\n',
     '        "patchCadPlayReference" as "patch-cad-play-reference" => patch_cad_play_reference::PatchCadPlayReference,\n'
     '        "setReferenceHidden" as "set-reference-hidden" => set_reference_hidden::SetReferenceHidden,\n'
     '        "setReferenceLocked" as "set-reference-locked" => set_reference_locked::SetReferenceLocked,\n')
once('''        "engagementInput" => CadCommand::EngagementInput(''', '''        "setReferenceHidden" => CadCommand::SetReferenceHidden(set_reference_hidden::SetReferenceHidden {
            model_definition_id: str_field("modelDefinitionId").unwrap_or_default(),
            reference_id: str_field("referenceId").unwrap_or_default(),
            hidden: bool_field("hidden").ok_or_else(|| cad_flag_value_required(action, "hidden"))?,
        }),
        "setReferenceLocked" => CadCommand::SetReferenceLocked(set_reference_locked::SetReferenceLocked {
            model_definition_id: str_field("modelDefinitionId").unwrap_or_default(),
            reference_id: str_field("referenceId").unwrap_or_default(),
            locked: bool_field("locked").ok_or_else(|| cad_flag_value_required(action, "locked"))?,
        }),
        "engagementInput" => CadCommand::EngagementInput(''')
once('''/// 🌉️ Converts the host shell's declared action id and JSON arguments into cad's closed typed
/// command vocabulary before the app dispatches through the binary command path.
fn cad_command_from_action(''', '''/// 🙈️ A reference set-verb (`setReferenceHidden{hidden}`, `setReferenceLocked{locked}`) sets exactly the boolean its
/// arguments carry — the row target's explicit next state — so a missing value is refused, never defaulted into a flip.
fn cad_flag_value_required(action: &str, flag: &str) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("cad.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets"))
}

/// 🌉️ Converts the host shell's declared action id and JSON arguments into cad's closed typed
/// command vocabulary before the app dispatches through the binary command path.
fn cad_command_from_action(''')
once('const CAD_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addNode", "renameNode", "patchCadPlayReference", ',
     'const CAD_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addNode", "renameNode", "patchCadPlayReference", "setReferenceHidden", "setReferenceLocked", ')
n = s.count('\n    "patchCadPlayReference",\n')
assert n == 1, n
s = s.replace('\n    "patchCadPlayReference",\n', '\n    "patchCadPlayReference",\n    "setReferenceHidden",\n    "setReferenceLocked",\n')
once('\n            "patchCadPlayReference",\n', '\n            "patchCadPlayReference",\n            "setReferenceHidden",\n            "setReferenceLocked",\n')
once('    ArtifactToolPublicationContract { tool_id: "patchCadPlayReference", lanes: &[ArtifactToolPublicationLane::Artifact] },\n',
     '    ArtifactToolPublicationContract { tool_id: "patchCadPlayReference", lanes: &[ArtifactToolPublicationLane::Artifact] },\n'
     '    ArtifactToolPublicationContract { tool_id: "setReferenceHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },\n'
     '    ArtifactToolPublicationContract { tool_id: "setReferenceLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },\n')
once('            .action_with(ActionDefinition::bounded_catalog("patchCadPlayReference", LocalizedLabel::native("Patch Reference", "Referenz aktualisieren"), ActionKind::Mutation).in_palette(false))\n',
     '            .action_with(ActionDefinition::bounded_catalog("patchCadPlayReference", LocalizedLabel::native("Patch Reference", "Referenz aktualisieren"), ActionKind::Mutation).in_palette(false))\n'
     '            .action_with(cad_reference_flag_action("setReferenceHidden", "hidden", LocalizedLabel::native("Set Reference Hidden", "Referenz verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))\n'
     '            .action_with(cad_reference_flag_action("setReferenceLocked", "locked", LocalizedLabel::native("Set Reference Locked", "Referenz gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))\n')
once('            .action_interactive_job("patchCadPlayReference", InteractiveJobClassification::Migrated)\n',
     '            .action_interactive_job("patchCadPlayReference", InteractiveJobClassification::Migrated)\n'
     '            .action_interactive_job("setReferenceHidden", InteractiveJobClassification::Migrated)\n'
     '            .action_interactive_job("setReferenceLocked", InteractiveJobClassification::Migrated)\n')
m = re.search(r'            \.action_describe\("patchCadPlayReference".*\n', s)
assert m
s = s[:m.end()] + (
    '            .action_describe("setReferenceHidden", LocalizedLabel::native("Sets one reference overlay hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt eine Referenzüberlagerung oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))\n'
    '            .action_describe("setReferenceLocked", LocalizedLabel::native("Sets one reference overlay locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt eine Referenzüberlagerung oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))\n'
) + s[m.end():]
m = re.search(r'\npub fn cad_tree_item\(', s)
assert m
s = s[:m.start()] + '''
/// 🙈️ A reference set-verb's definition: the `{modelDefinitionId, referenceId}` a row names and the REQUIRED boolean `flag`
/// it sets — a missing value is refused, never defaulted.
fn cad_reference_flag_action(id: &str, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation)
        .with_args([
            ActionArgDef::text("modelDefinitionId", LocalizedLabel::native("Model Definition", "Modelldefinition")).required(),
            ActionArgDef::text("referenceId", LocalizedLabel::native("Reference", "Referenz")).required(),
            ActionArgDef::toggle(flag, value).required(),
        ])
        .in_palette(false)
}
''' + s[m.start():]
ED.write_text(s)

CM = ROOT / "✏️editor/🎮️commands/🖼️reference/🦀️.rs"
t = CM.read_text()
old = '//#endregion 🔖️PatchCadPlayReference\n'
assert t.count(old) == 1
t = t.replace(old, old + '''
//#region 🙈️SetReferenceFlag
/// 🔎️ The reference a set-verb addresses, if the document holds it.
fn addressed_reference<'a>(document: &'a CadSnapshot, model_definition_id: &str, reference_id: &str) -> Option<&'a crate::CadReference> {
    document.references_by_model_definition_id.get(model_definition_id).and_then(|references| references.iter().find(|reference| reference.id == reference_id))
}

/// 🙈️ `setReferenceHidden{hidden}` — the reference row's show/hide toggle, set to exactly the value its target carries; a
/// reference already in that state emits nothing, so a replayed click leaves the one edit the first one made.
pub mod set_reference_hidden {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-reference-hidden")]
    pub struct SetReferenceHidden {
        pub model_definition_id: String,
        pub reference_id: String,
        pub hidden: bool,
    }

    pub fn handle(payload: &SetReferenceHidden, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(match addressed_reference(doc.snapshot, &payload.model_definition_id, &payload.reference_id) {
            Some(reference) if reference.hidden != payload.hidden => Emit::mutations(vec![CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_hidden: payload.hidden })]),
            _ => Emit::default(),
        })
    }
}

/// 🔒️ `setReferenceLocked{locked}` — the reference row's lock toggle, set to exactly the value its target carries; a
/// reference already in that state emits nothing, so a replayed click leaves the one edit the first one made.
pub mod set_reference_locked {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-reference-locked")]
    pub struct SetReferenceLocked {
        pub model_definition_id: String,
        pub reference_id: String,
        pub locked: bool,
    }

    pub fn handle(payload: &SetReferenceLocked, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(match addressed_reference(doc.snapshot, &payload.model_definition_id, &payload.reference_id) {
            Some(reference) if reference.locked != payload.locked => Emit::mutations(vec![CadMutation::ChangeReferenceLocked(ChangeReferenceLocked { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_locked: payload.locked })]),
            _ => Emit::default(),
        })
    }
}
//#endregion 🙈️SetReferenceFlag
''')
t = t.replace("//! 🖼️ CAD play app commands — the per-pane reference overlays: patch, select, hover.", "//! 🖼️ CAD play app commands — the per-pane reference overlays: patch, set a flag, select, hover.")
CM.write_text(t)

FX = ROOT / "🧫️fixtures/🗄️retained-jobs/🔣️.json"
raw = FX.read_text()
j = json.loads(raw)
j["routeCount"] += 2
j["activation"]["proofRows"] += 2
i = j["admittedRoutes"].index("patchCadPlayReference")
j["admittedRoutes"][i + 1:i + 1] = ["setReferenceHidden", "setReferenceLocked"]
routes = []
for route in j["routes"]:
    routes.append(route)
    if route["id"] == "patchCadPlayReference":
        for verb in ("setReferenceHidden", "setReferenceLocked"):
            routes.append({**route, "id": verb})
assert len(routes) == len(j["routes"]) + 2
j["routes"] = routes
FX.write_text(json.dumps(j, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else ""))
print("ok")
