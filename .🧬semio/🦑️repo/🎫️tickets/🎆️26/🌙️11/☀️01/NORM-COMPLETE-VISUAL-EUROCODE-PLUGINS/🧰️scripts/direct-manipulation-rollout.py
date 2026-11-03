import os, re, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
ARTIFACTS = {
    "en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1993": "🔩️", "en1994": "🧩️",
    "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️", "en1998": "🫨️", "en1999": "🪶️",
}
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"
ED = "✏️editor"
VIEWPORT = os.path.join(ED, "🎭️modes", "✏️edit", "🪟️windows", "🧊️viewport", "🦀️.rs")
TICKET = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"


def path(code, *parts):
    return os.path.join(ROOT, ARTIFACTS[code] + code, SUBSET, *parts)


def read(p):
    with open(p, encoding="utf-8") as handle:
        return handle.read()


def write(p, text):
    with open(p, "w", encoding="utf-8") as handle:
        handle.write(text)


def sub1(text, pattern, repl, what, flags=0):
    new, count = re.subn(pattern, lambda m: repl if isinstance(repl, str) else repl(m), text, count=1, flags=flags)
    if count != 1:
        raise SystemExit(f"anchor not found: {what}")
    return new


ROWS = '''        "setFields" as "set-fields" => set_fields::SetFields,
        "previewRemedy" as "preview-remedy" => preview_remedy::PreviewRemedy,
        "clearPreview" as "clear-preview" => clear_preview::ClearPreview,
        "cancelEvaluation" as "cancel-evaluation" => cancel_evaluation::CancelEvaluation,
        "evaluateUnits" as "evaluate-units" => evaluate_units::EvaluateUnits,
'''

ACTIONS = '''            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("setFields", LocalizedLabel::native("Set Fields", "Felder setzen"), semio_framework_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_framework_plugin::ActionArgDef::any("writes", LocalizedLabel::native("Writes", "Schreibvorgänge")).required(),
                        semio_framework_plugin::ActionArgDef::any("gesture", LocalizedLabel::native("Gesture", "Geste")),
                    ]),
            )
            .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("previewRemedy", LocalizedLabel::native("Preview Remedy", "Abhilfe vorschauen"), semio_framework_plugin::ActionKind::View)
                    .with_args(vec![
                        semio_framework_plugin::ActionArgDef::text("checkId", LocalizedLabel::native("Check", "Nachweis")),
                        semio_framework_plugin::ActionArgDef::index("remedyIndex", LocalizedLabel::native("Remedy", "Abhilfe")),
                        semio_framework_plugin::ActionArgDef::index("optionIndex", LocalizedLabel::native("Option", "Variante")),
                        semio_framework_plugin::ActionArgDef::any("insert", LocalizedLabel::native("Insert ghost", "Einfügevorschau")),
                    ]),
            )
            .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("clearPreview", LocalizedLabel::native("Clear Preview", "Vorschau verwerfen"), semio_framework_plugin::ActionKind::View, "close"))
            .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("cancelEvaluation", LocalizedLabel::native("Cancel Evaluation", "Auswertung abbrechen"), semio_framework_plugin::ActionKind::View, "close"))
            .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("evaluateUnits", LocalizedLabel::native("Evaluate Units", "Einheiten auswerten"), semio_framework_plugin::ActionKind::View)
                    .with_args(vec![
                        semio_framework_plugin::ActionArgDef::any("units", LocalizedLabel::native("Units", "Einheiten")),
                        semio_framework_plugin::ActionArgDef::text("reason", LocalizedLabel::native("Reason", "Anlass")),
                    ]),
            )
            .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
'''

DESCRIBE = '''            .action_describe("setFields", LocalizedLabel::native("Writes several document fields as one undoable edit; a handle gesture only edits the document on release or Enter.", "Schreibt mehrere Dokumentfelder als eine rückgängig machbare Änderung; eine Griffgeste ändert das Dokument erst beim Loslassen oder mit Enter."))
            .action_describe("previewRemedy", LocalizedLabel::native("Shows what a remedy, or an inserted element at the cursor, would change as a ghost; the document is not changed.", "Zeigt als Vorschau, was eine Abhilfe oder ein am Cursor eingefügtes Element ändern würde; das Dokument ändert sich nicht."))
            .action_describe("clearPreview", LocalizedLabel::native("Discards the drag and ghost previews without a trace.", "Verwirft die Zieh- und Geistervorschau spurlos."))
            .action_describe("cancelEvaluation", LocalizedLabel::native("Cancels the evaluation in flight; the latest edit is evaluated next.", "Bricht die laufende Auswertung ab; als Nächstes wird die letzte Änderung ausgewertet."))
            .action_describe("evaluateUnits", LocalizedLabel::native("Re-runs the given check units, or all of them, now.", "Wertet die angegebenen Nachweiseinheiten oder alle sofort neu aus."))
'''


def root_edit(code):
    p = path(code, ED, "🦀️.rs")
    text = read(p)
    if "norm_live_editor!" in text:
        return "root: already"
    P = "En" + code[2:]
    n = code
    m = re.search(r"use crate::commands::\{([^}]*)\};", text)
    if not m:
        raise SystemExit("commands use")
    names = [x.strip() for x in m.group(1).split(",") if x.strip()] + ["set_fields", "preview_remedy", "clear_preview", "cancel_evaluation", "evaluate_units"]
    text = text.replace(m.group(0), "use crate::commands::{" + ", ".join(names) + "};", 1)
    text = sub1(text, r"norm_command_from_action!\((En\d+Command), (?:text, )?(semio_s_artifact_norm_en\d+_io::[^)]*)\)", lambda m: f"norm_command_from_action!({m.group(1)}, live, {m.group(2)})", "command_from_action")
    text = text.replace("use semio_framework_plugin::{NoPresence, NoPresenceMutation};\n", "", 1)
    for old, new in [
        ("type Presence = NoPresence;", "type Presence = crate::lanes::NormLanePresence;"),
        ("type PresenceMutation = NoPresenceMutation;", "type PresenceMutation = crate::lanes::NormLanePresenceMutation;"),
        ("type Transient = semio_framework_plugin::NoTransient;", "type Transient = crate::lanes::NormLaneTransient;"),
        ("type TransientMutation = semio_framework_plugin::NoTransientMutation;", "type TransientMutation = crate::lanes::NormLaneTransientMutation;"),
        ("norm_exact_store_ownership!();", "norm_exact_store_ownership!(live);"),
        ('tools: ["evaluate", "setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy"]', 'tools: ["evaluate", "setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy", "setFields", "previewRemedy", "clearPreview", "cancelEvaluation", "evaluateUnits"]'),
    ]:
        if text.count(old) != 1:
            raise SystemExit(f"{code}: anchor {old}")
        text = text.replace(old, new)
    text = sub1(text, r'(        "applyRemedy" as "apply-remedy" => apply_remedy::ApplyRemedy,\n)', lambda m: m.group(1) + ROWS, "rows")
    ephemeral = f'''    fn ephemeral(
        command: &{P}Command,
        doc: &ArtifactView<'_, {P}Snapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        presence: &semio_framework_plugin::PresenceView<'_, Self::Presence>,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {{
        semio_s_plugin_norm_app_surface::live_ephemeral::<Self>(command, doc, presence, transient)
    }}

'''
    text = sub1(text, rf"(    fn command_id\(command: &{P}Command\))", lambda m: ephemeral + m.group(1), "command_id")
    text = text.replace("render_body(body_key, doc, cfg, view_state, &[])", "render_body(body_key, doc, cfg, view_state, &[], &semio_s_plugin_norm_app_surface::LaneFrame::default())", 1)
    text = text.replace("_transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,", "transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,", 1)
    old = "render_body(body_key, doc, cfg, view_state, &interaction.selection(&semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT)).ids)"
    if text.count(old) != 1:
        raise SystemExit(f"{code}: render ctx")
    text = text.replace(old, "render_body(body_key, doc, cfg, view_state, &interaction.selection(&semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT)).ids, &semio_s_plugin_norm_app_surface::LaneFrame::of(transient.snapshot, interaction, &semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT)))")
    text = text.replace("view_state: &semio_framework_plugin::ViewModel, picked: &[String]) -> semio_framework_plugin::UiAssemblyResult", "view_state: &semio_framework_plugin::ViewModel, picked: &[String], frame: &semio_s_plugin_norm_app_surface::LaneFrame) -> semio_framework_plugin::UiAssemblyResult", 1)
    text = sub1(text, rf"LiveView::<{P}Family>::new\(doc\.snapshot\.clone\(\), (.*?), picked\);", lambda m: f"LiveView::<{P}Family>::with_lanes(doc.snapshot.clone(), {m.group(1)}, picked, &frame.transient);", "liveview")
    text = sub1(text, r"viewport::render\(doc\.snapshot, host\.report\(\), selection, (combination\.as_deref\(\), )?view_state\.locale\)", lambda m: f"viewport::render(host.document(), host.report(), selection, {m.group(1) or ''}frame, view_state.locale)", "viewport arm")
    text = sub1(text, r"diagrams::render\(doc\.snapshot, ", "diagrams::render(host.document(), ", "diagrams arm")
    text = sub1(text, r"(    type ResultsWindowConfigOwner = checks::ChecksWindowConfigOwner;\n)", lambda m: m.group(1) + "\n    const TOOL_IDS: &'static [&'static str] = semio_s_plugin_norm_app_surface::NORM_EDITOR_TOOL_IDS;\n    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = semio_s_plugin_norm_app_surface::NORM_EDITOR_PUBLICATION_CONTRACTS;\n", "consts")
    scene_crate = f"semio_s_artifact_norm_{n}_scene"
    live_editor = f'''
//#region 🫧️LiveEditor
fn live_entities(document: &{P}Snapshot) -> Vec<semio_s_plugin_norm_structural_scene::EntityRef> {{
    viewport::scene_of(document, &<{P}Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document){", None" if code == "en1990" else ""}).entities.into_iter().map(|entity| entity.entity).collect()
}}

fn live_handles(document: &{P}Snapshot, selection: &[semio_s_plugin_norm_structural_scene::EntityRef]) -> Vec<semio_s_plugin_norm_structural_scene::Handle> {{
    viewport::handles_of(document, selection)
}}

semio_s_plugin_norm_app_surface::norm_live_editor!({P}PlayApp, {P}Command, handles = live_handles, resolve = semio_s_artifact_norm_{n}_mutations::resolve);
semio_s_plugin_norm_app_surface::norm_live_acceptance!({P}PlayApp, entities = live_entities);
//#endregion 🫧️LiveEditor
'''
    text = sub1(text, r"(\n//#region 🧵️RetainedCommands)", lambda m: live_editor + m.group(1), "retained region")
    text = sub1(text, r'(            \.keybinding\("mod\+z", "undo"\))', lambda m: ACTIONS + m.group(1), "keybinding")
    text = sub1(text, r"(            \.build_definition\(\))", lambda m: DESCRIBE + m.group(1), "build_definition")
    text = sub1(text, r'(    #\[path = "🎮️commands/☑️selected-check/🦀️.rs"\]\n    pub mod selected_check;\n    #\[path = "🎮️commands/✏️set-field/🦀️.rs"\]\n    pub mod set_field;\n)', lambda m: m.group(1) + f"    semio_s_plugin_norm_app_surface::norm_live_commands!(crate::{P}PlayApp);\n", "commands mount")
    owner = f"✏️s/🔌️plugins/📕️norm/🗿️artifacts/{ARTIFACTS[code]}{code}/{SUBSET}/{ED}/🫧️lanes"
    lanes = f'''#[path = "."]
pub mod lanes {{
    semio_s_plugin_norm_app_surface::norm_lane_stores!(presence_extension = "norm.{n}.live.presence", transient_extension = "norm.{n}.live.transient", owner = "{owner}");
}}
'''
    text = sub1(text, r"(//#endregion 🪢️Mounts)", lambda m: lanes + m.group(1), "mounts end")
    write(p, text)
    return "root: ok"


def viewport_edit(code):
    p = path(code, VIEWPORT)
    text = read(p)
    if "scene_of" in text:
        return "viewport: already"
    P = "En" + code[2:]
    combination = code == "en1990"
    text = sub1(text, r"use semio_s_plugin_norm_app_surface::viewport3d::toolbar::\{render_viewport_panel, ViewportView\};", "use semio_s_plugin_norm_app_surface::viewport3d::lane_overlays::LaneOverlays;\nuse semio_s_plugin_norm_app_surface::viewport3d::toolbar::{render_viewport_panel, ViewportView};", "toolbar import")
    text = text.replace("use semio_s_plugin_norm_app_surface::{interaction_domain_id, NormSelection};", "use semio_s_plugin_norm_app_surface::{interaction_domain_id, LaneFrame, NormSelection};", 1)
    text = text.replace("use semio_s_plugin_norm_structural_scene::SceneRevision;", "use semio_s_plugin_norm_structural_scene::{EntityRef, Handle, SceneRevision, StructuralScene};", 1)
    comb_param = ", combination: Option<&str>" if combination else ""
    comb_arg = ", combination" if combination else ""
    scene_crate = f"semio_s_artifact_norm_{code}_scene"
    if code == "en1992":
        handles_body = f"""    let ids: Vec<&str> = selection.iter().map(|entity| entity.id.as_str()).collect();
    let report = <crate::En1992Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document);
    {scene_crate}::handles(&SceneInput {{ document, report: &report, placements: &[], revision: SceneRevision {{ document_seq: 0, report_seq: 0 }} }}).into_iter().filter(|handle| ids.contains(&handle.entity.id.as_str())).collect()"""
    else:
        handles_body = f"    {scene_crate}::handles(document, &[], selection)"
    m = re.search(r"/// 🧊️ Projects[^\n]*\npub fn render\(.*?\n\}\n\n/// 🎞️[^\n]*\npub fn render_with\(.*?\n\}\n", text, re.S)
    if not m:
        raise SystemExit(f"{code}: render fns")
    new = f"""/// 🧱️ The scene of one document revision; the live viewport projects the preview document, so a drag re-projects it at frame rate.
pub fn scene_of(document: &{P}Snapshot, report: &CheckReport{comb_param}) -> StructuralScene {{
    project(&SceneInput {{ document, report, placements: &[], revision: SceneRevision {{ document_seq: 0, report_seq: 0 }}{comb_arg} }})
}}

/// 🕹️ The drag handles of the selected entities of `document`.
pub fn handles_of(document: &{P}Snapshot, selection: &[EntityRef]) -> Vec<Handle> {{
{handles_body}
}}

/// 🧊️ Projects the document and lowers it to the `world-3d` surface; `selection` is the resolved selection whose scene entities are highlighted{" and `combination` the id of the active combination column whose design values scale the load glyphs" if combination else ""}, `frame` the live lanes: grips of the selection, the dimension-line entry of the open drag, the insert ghost and the selection of the other users.
pub fn render(document: &{P}Snapshot, report: &CheckReport, selection: &NormSelection{comb_param}, frame: &LaneFrame, locale: Locale) -> UiAssemblyResult<BuiltNode> {{
    render_with(document, report, selection{comb_arg}, &ViewportView {{ lanes: LaneOverlays {{ handles: Vec::new(), frame: frame.clone() }}, ..ViewportView::default() }}, locale)
}}

/// 🎞️ The same window at an explicit view: the persisted local-only configuration and the ephemeral playback state, with the field picker and playback toolbar above the surface; the grips of the selection are resolved here from the projected scene.
pub fn render_with(document: &{P}Snapshot, report: &CheckReport, selection: &NormSelection{comb_param}, view: &ViewportView, locale: Locale) -> UiAssemblyResult<BuiltNode> {{
    let scene = scene_of(document, report{comb_arg});
    let selected = selection.highlights(scene.entities.iter().map(|e| (e.entity.kind, e.entity.id.as_str())));
    let picked: Vec<EntityRef> = scene.entities.iter().filter(|e| selection.contains(&e.entity.id)).map(|e| e.entity.clone()).collect();
    let view = ViewportView {{ lanes: LaneOverlays {{ handles: handles_of(document, &picked), frame: view.lanes.frame.clone() }}, ..view.clone() }};
    render_viewport_panel(BODY_VIEWPORT, &scene, &view, &selected, &interaction_domain_id(crate::VARIANT), crate::CONTROLLER_ID, locale)
}}
"""
    text = text.replace(m.group(0), new, 1)
    write(p, text)
    return "viewport: ok"


def cargo_edit(code):
    p = path(code, ED, "📦️packages", "🦀️rust", "Cargo.toml")
    text = read(p)
    head, _, tail = text.partition("[dev-dependencies]")
    changed = False
    for dep in ["semio-framework-diagnostic", "semio-framework-pack-json"]:
        if not re.search(rf"^{dep} =", head, re.M):
            head = head.replace("semio-framework-dsl-record-derive = { workspace = true }\n", f"semio-framework-dsl-record-derive = {{ workspace = true }}\n{dep} = {{ workspace = true }}\n", 1)
            changed = True
    if changed:
        write(p, head + "[dev-dependencies]" + tail)
    return "cargo: ok" if changed else "cargo: already"


def tests_edit(code):
    p = path(code, ED, "🧪️tests", "🔬️unit", "🦀️.rs")
    text = read(p)
    if "NORM_EDITOR_TOOL_IDS" in text:
        return "tests: already"
    if "NORM_RETAINED_TOOL_IDS" not in text:
        return "tests: no retained ids test"
    text = text.replace("NORM_RETAINED_TOOL_IDS", "NORM_EDITOR_TOOL_IDS")
    write(p, text)
    return "tests: ok"


def build_notes(code):
    d = os.path.join(TICKET, "🗑️generated", "build-" + code)
    os.makedirs(d, exist_ok=True)
    latest = os.path.join(d, "latest.md")
    line = "ready for rebuild: direct manipulation\n"
    existing = read(latest) if os.path.exists(latest) else ""
    if line not in existing:
        write(latest, existing + ("" if existing.endswith("\n") or not existing else "\n") + line)
    return "latest.md: ok"


if __name__ == "__main__":
    codes = sys.argv[1:] or list(ARTIFACTS)
    for code in codes:
        print(code, [f(code) for f in (root_edit, viewport_edit, cargo_edit, tests_edit, build_notes)])
