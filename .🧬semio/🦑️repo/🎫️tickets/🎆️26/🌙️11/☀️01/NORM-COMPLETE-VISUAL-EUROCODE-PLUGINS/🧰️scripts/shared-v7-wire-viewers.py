"""Wires the ten Eurocode viewers to the ephemeral local view lane (shared v7). Idempotent: skips viewers that already carry the marker."""
import re, pathlib, sys

NORM = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
CODES = {"en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1993": "🔩️", "en1994": "🧩️", "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️", "en1998": "🫨️", "en1999": "🪶️"}
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"
MARK = "norm_viewer_surface!"

def edit(path, fn):
    text = path.read_text()
    new = fn(text)
    if new != text:
        path.write_text(new)

def viewer(code, emoji):
    x, X, E = code, code.capitalize(), code.upper()
    base = NORM / f"{emoji}{x}" / SUB / "👁️viewer"
    f = base / "🦀️.rs"
    s = f.read_text()
    if MARK in s:
        return f"{code}: already wired"
    owner_file = f"✏️s/🔌️plugins/📕️norm/🗿️artifacts/{emoji}{x}/{SUB}/👁️viewer/🦀️.rs"
    lanes_owner = f"✏️s/🔌️plugins/📕️norm/🗿️artifacts/{emoji}{x}/{SUB}/👁️viewer/🫧️lanes"
    scene_call = "live_scene_of(d, r, None)" if code == "en1990" else "live_scene_of(d, r)"
    s = s.replace(", NoTransient, NoTransientMutation, ViewEmit, Viewer};", ", ViewEmit, Viewer};")
    s = s.replace("NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit", "NoPresenceMutation, ViewEmit")
    a = s.index("//#region 🔖️Command")
    b = s.index("//#endregion 🔖️Command") + len("//#endregion 🔖️Command")
    cmd = f'''//#region 🔖️Command
semio_framework_plugin::view_commands! {{
    /// 👁️ `{X}Viewer::Command`: the sole dispatch surface of this read-only viewer, `viewportAct` (stepping, field, track, playback). `dispatch` returns `ViewEmit`, so no row can carry an artifact or draft mutation; the step itself only folds into the ephemeral local view lane (`ephemeral`).
    pub enum {X}ViewCommand for {X}Snapshot, NoConfig, NoConfigMutation {{
        "viewportAct" as "viewport-act" => viewport_act::ViewportAct,
    }}
}}

semio_s_plugin_norm_app_surface::norm_viewer_commands!({X}Snapshot);

semio_s_plugin_norm_app_surface::norm_owned_view_tool_job_factory!({X}ViewBoundedCommandJobFactory, {X}Viewer);

semio_s_plugin_norm_app_surface::norm_viewer_surface!({X}Viewer, {X}ViewCommand, report = |d: &{X}Snapshot| semio_s_artifact_norm_{x}_inferences::live::report(d), scene = |d: &{X}Snapshot, r: &semio_s_plugin_norm_app_surface::CheckReport| semio_s_artifact_norm_{x}_scene::{scene_call});
//#endregion 🔖️Command'''
    s = s[:a] + cmd + s[b:]
    s = s.replace("    type Transient = NoTransient;\n    type TransientMutation = NoTransientMutation;", "    type Transient = crate::lanes::NormLaneTransient;\n    type TransientMutation = crate::lanes::NormLaneTransientMutation;")
    s = s.replace("semio_s_plugin_norm_app_surface::norm_exact_viewer_store_ownership!();", f'''semio_s_plugin_norm_app_surface::norm_exact_viewer_store_ownership!(lanes);

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::ViewerApp<Self>>) -> Result<(), Fault> {{
        {X}ViewBoundedCommandJobFactory::register(registry)
    }}

    fn build_tool_job(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::ViewerApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {{
        semio_s_plugin_norm_app_surface::build_norm_view_tool_job::<Self>(request, output, cx)
    }}

    semio_framework_plugin::bounded_first_step_tool_proofs! {{
        owner: semio_framework_plugin::ViewerApp<{X}Viewer>,
        owner_file: "{owner_file}",
        controller: "s.norm.{x}@1/*#viewer",
        artifact_schema: {E}_DOCUMENT_SCHEMA,
        factory: "{X}ViewBoundedCommandJobFactory",
        factory_type: {X}ViewBoundedCommandJobFactory,
        contract: semio_s_plugin_norm_app_surface::norm_bounded_contract(),
        tools: ["viewportAct"]
    }}

    fn command_id(command: &Self::Command) -> &'static str {{
        command.command_id()
    }}

    semio_s_plugin_norm_app_surface::norm_view_command_from_action!({X}ViewCommand);

    /// 🫧️ The ephemeral local view lane: stepping checks and combinations folds into this client's transient, never into the document.
    fn ephemeral(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &semio_framework_plugin::PresenceView<'_, Self::Presence>,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {{
        semio_s_plugin_norm_app_surface::viewer_ephemeral::<Self>(command, doc, transient)
    }}''')
    s = s.replace("""        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }""", """        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        command.dispatch(doc, cfg)
    }""")
    s = re.sub(r"    /// 👁️ Structurally read-only: the sole `\w+::Noop` variant never carries a config\n    /// change, so this always returns the empty `ViewEmit`\. Kept as a real dispatch \(not\n    /// `unreachable!\(\)`\) so a future view-only action is a pure addition here, never a signature change\.\n",
               "    /// 👁️ Structurally read-only: `ViewEmit` cannot carry an artifact or draft mutation, and the one row (`viewportAct`) emits nothing at all; its effect is the ephemeral view lane.\n", s)
    s = s.replace("""    fn render(body_key: &str,""", f"""    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
        _cx: &mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {{
        let lane = semio_s_plugin_norm_app_surface::LaneTransient::lane(transient.snapshot).view;
        let rendered = match body_key {{
            viewport::BODY_VIEWPORT => viewport::render_lane(doc.snapshot, &lane, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            checks::BODY_CHECKS => checks::render_selected(doc.snapshot, semio_s_plugin_norm_app_surface::view_selected_check(&lane), &semio_framework_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            _ => Self::render(body_key, doc, cfg, view_state),
        }};
        rendered.map(Some).map_err(Into::into)
    }}

    fn render(body_key: &str,""", 1)
    s = s.replace("""    #[path = "👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}""", f"""    #[path = "👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}}
#[path = "."]
pub mod lanes {{
    semio_s_plugin_norm_app_surface::norm_viewer_lane_stores!(transient_extension = "norm.{x}.view.transient", owner = "{lanes_owner}");
}}""")
    s = s.replace("        .build_definition()", """        .action_with(
            semio_framework_plugin::ActionDefinition::new_catalog("viewportAct", semio_framework_ui_locale::LocalizedLabel::native("Viewport Action", "Viewport-Aktion"), semio_framework_plugin::ActionKind::View).with_args(vec![
                semio_framework_plugin::ActionArgDef::text("action", semio_framework_ui_locale::LocalizedLabel::native("Action", "Aktion")),
                semio_framework_plugin::ActionArgDef::text("text", semio_framework_ui_locale::LocalizedLabel::native("Text", "Text")),
                semio_framework_plugin::ActionArgDef::any("number", semio_framework_ui_locale::LocalizedLabel::native("Number", "Zahl")),
            ]),
        )
        .action_interactive_job("viewportAct", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_describe("viewportAct", semio_framework_ui_locale::LocalizedLabel::native("Steps the checks and combinations or changes the colour field, track and playback of the viewport; only this client's ephemeral view changes, never the document.", "Schaltet Nachweise und Kombinationen weiter oder ändert Farbfeld, Verlauf und Wiedergabe des Viewports; nur die flüchtige Ansicht dieses Clients ändert sich, nie das Dokument."))
        .build_definition()""", 1)
    f.write_text(s)

    w = base / "🎭️modes/👁️view/🪟️windows"
    def vp(t):
        return t.replace("//#endregion 🔖️Render", f"""
/// 🎞️ The same window at the ephemeral local view lane of the viewer (colour field, track, value, playback).
pub fn render_lane(document: &{X}Snapshot, lane: &semio_s_plugin_norm_app_surface::ViewLane, locale: Locale) -> UiAssemblyResult<BuiltNode> {{
    render_with(document, &ViewportView {{ lane: lane.clone(), ..ViewportView::default() }}, locale)
}}
//#endregion 🔖️Render""", 1)
    edit(w / "🧊️viewport/🦀️.rs", vp)
    def ck(t):
        return t.replace("//#endregion 🔖️Render", f"""
/// ✅️ The settled report with the check the view lane selected (stepping), still without any controller.
pub fn render_selected(document: &{X}Snapshot, selected: Option<usize>, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {{
    let report = semio_s_artifact_norm_{x}_inferences::live::report(document);
    render_checks_with(&report, &ChecksViewState {{ selected, ..ChecksViewState::default() }}, &ChecksWindowSpec::default(), windows, locale)
}}
//#endregion 🔖️Render""", 1)
    edit(w / "✅️checks/🦀️.rs", ck)

    def cargo(t):
        add = []
        for dep, line in [("semio-framework ", "semio-framework = { workspace = true }"),
                          ("semio-framework-dsl-record ", 'semio-framework-dsl-record = { path = "../../../../../../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust" }'),
                          ("semio-framework-dsl-record-derive", "semio-framework-dsl-record-derive = { workspace = true }"),
                          ("semio-framework-diagnostic", "semio-framework-diagnostic = { workspace = true }")]:
            if not re.search(r"^" + re.escape(dep.strip()) + r"\s*=", t, re.M):
                add.append(line)
        return t.replace("[dependencies]\n", "[dependencies]\n" + "".join(l + "\n" for l in add), 1) if add else t
    edit(base / "📦️packages/🦀️rust/Cargo.toml", cargo)

    tf = base / "🧪️tests/🔬️unit/🦀️.rs"
    t = tf.read_text()
    t += f'''
#[test]
fn a_viewer_steps_checks_and_combinations_through_its_ephemeral_lane_without_any_edit() {{
    let snapshot = {X}Snapshot::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView {{ snapshot: &NoConfig::default(), window: None }};
    let lane = crate::lanes::NormLaneTransient::default();
    let transient = semio_framework_plugin::TransientView {{ snapshot: &lane, window: None }};
    for action in ["viewportStepCheck", "viewportStepGoverning", "viewportStepCombination"] {{
        let command = {X}ViewCommand::ViewportAct(viewport_act::ViewportAct {{ action: action.to_string(), text: None, number: Some(1.0) }});
        assert!(command.dispatch(&doc, &cfg).expect("dispatch").config_mutations.is_empty(), "{{action}} never writes the config");
        assert!(semio_s_plugin_norm_app_surface::viewer_ephemeral::<{X}Viewer>(&command, &doc, &transient).1.len() <= 1, "{{action}} publishes at most one transient mutation");
        assert_eq!(<{X}Viewer as ArtifactViewer>::command_id(&command), "viewportAct");
    }}
}}

#[test]
fn the_viewer_maps_the_stepping_shortcuts_to_its_view_action() {{
    assert!(<{X}Viewer as ArtifactViewer>::command_from_action("norm.shortcut.next-check", &mut None, &mut None).is_ok());
    assert!(<{X}Viewer as ArtifactViewer>::command_from_action("norm.unknown", &mut None, &mut None).is_err());
    assert!({X}Viewer::READ_ONLY);
}}
'''
    tf.write_text(t)
    return f"{code}: wired"

for code, emoji in CODES.items():
    print(viewer(code, emoji))
