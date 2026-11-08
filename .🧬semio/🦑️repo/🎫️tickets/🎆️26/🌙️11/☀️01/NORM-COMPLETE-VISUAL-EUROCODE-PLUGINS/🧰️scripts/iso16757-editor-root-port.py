p='🦀️.rs'
s=open(p,encoding='utf8').read()
def rep(old,new,count=1):
    global s
    assert old in s, old[:80]
    s=s.replace(old,new,count)

rep("use crate::commands::{apply_remedy, insert_item, remove_item, selected_check, set_field};","use crate::commands::{apply_remedy, insert_item, remove_item, selected_check, set_field, set_fields, preview_remedy, clear_preview, cancel_evaluation, evaluate_units, copy_entities, duplicate_entities, paste_entities, move_entity, delete_entities};")
rep("use crate::modes::edit::windows::{inputs, results};","use crate::modes::edit::windows::{checks, diagrams, outliner, properties, viewport};")
rep("//#region 🔖️Constants\n","//#region 🔖️Constants\n/// 🏷️ The scene entity kinds ISO 16757 projects: one interaction granularity each.\npub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[\n    semio_s_plugin_norm_structural_scene::EntityKind::Group,\n    semio_s_plugin_norm_structural_scene::EntityKind::Solid,\n    semio_s_plugin_norm_structural_scene::EntityKind::Fastener,\n];\n\n")
rep('''        "applyRemedy" as "apply-remedy" => apply_remedy::ApplyRemedy,
''','''        "applyRemedy" as "apply-remedy" => apply_remedy::ApplyRemedy,
        "setFields" as "set-fields" => set_fields::SetFields,
        "previewRemedy" as "preview-remedy" => preview_remedy::PreviewRemedy,
        "clearPreview" as "clear-preview" => clear_preview::ClearPreview,
        "cancelEvaluation" as "cancel-evaluation" => cancel_evaluation::CancelEvaluation,
        "evaluateUnits" as "evaluate-units" => evaluate_units::EvaluateUnits,
        "viewportAct" as "viewport-act" => viewport_act::ViewportAct,
        "copyEntities" as "copy-entities" => copy_entities::CopyEntities,
        "duplicateEntities" as "duplicate-entities" => duplicate_entities::DuplicateEntities,
        "pasteEntities" as "paste-entities" => paste_entities::PasteEntities,
        "moveEntity" as "move-entity" => move_entity::MoveEntity,
        "deleteEntities" as "delete-entities" => delete_entities::DeleteEntities,
''')
rep('''    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = semio_plugin::NoTransient;
    type TransientMutation = semio_plugin::NoTransientMutation;
''','''    type Presence = crate::lanes::NormLanePresence;
    type PresenceMutation = crate::lanes::NormLanePresenceMutation;
    type Transient = crate::lanes::NormLaneTransient;
    type TransientMutation = crate::lanes::NormLaneTransientMutation;
''')
rep("use semio_plugin::{NoPresence, NoPresenceMutation};\n","")
rep("    type Command = Iso16757Command;\n","    type Command = Iso16757Command;\n\n    fn interaction_topology(doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<semio_plugin::InteractionTopology, semio_framework_value::ValueError> {\n        semio_s_plugin_norm_app_surface::live_interaction_topology::<Self>(doc.snapshot, checks::INTERACTION_DOMAIN)\n    }\n\n    fn fault_notices() -> &'static [(&'static str, semio_ui_locale::LocalizedLabel)] {\n        semio_s_plugin_norm_app_surface::norm_fault_notices()\n    }\n")
rep("norm_exact_store_ownership!();","norm_exact_store_ownership!(live);")
rep('tools: ["setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy"]','tools: ["setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy", "setFields", "previewRemedy", "clearPreview", "cancelEvaluation", "evaluateUnits", "viewportAct", "copyEntities", "duplicateEntities", "pasteEntities", "moveEntity", "deleteEntities"]')
rep("registry.register::<results::ResultsWindowConfigOwner>()","registry.register::<checks::ChecksWindowConfigOwner>()")
rep('''    fn command_id(command: &Iso16757Command)''','''    fn ephemeral(
        command: &Iso16757Command,
        doc: &ArtifactView<'_, Iso16757Snapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        presence: &semio_plugin::PresenceView<'_, Self::Presence>,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        semio_s_plugin_norm_app_surface::live_ephemeral::<Self>(command, doc, presence, transient)
    }

    fn command_id(command: &Iso16757Command)''')
rep("norm_command_from_action!(Iso16757Command, semio_s_artifact","norm_command_from_action!(Iso16757Command, live, semio_s_artifact")
rep("    fn handle(\n        command: &Iso16757Command,","    semio_s_plugin_norm_app_surface::norm_context_menu!();\n\n    fn handle(\n        command: &Iso16757Command,")
a=s.index("    fn render(body_key: &str")
b=s.index("    //#region 🔖️MediaPorts")
s=s[:a]+'''    fn render(body_key: &str, doc: &ArtifactView<'_, Iso16757Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &[], &semio_s_plugin_norm_app_surface::LaneFrame::default())
    }

    fn render_with_request_context(
        _owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Iso16757Snapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_plugin::ViewModel,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        _cx: &mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &interaction.selection(&semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT)).ids, &semio_s_plugin_norm_app_surface::LaneFrame::of(transient.snapshot, interaction, &semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT))).map(Some).map_err(Into::into)
    }

'''+s[b:]
rep("//#endregion 🔖️Iso16757PlayApp\n",'''//#endregion 🔖️Iso16757PlayApp

//#region 🪄️LiveRender
fn render_body(body_key: &str, doc: &ArtifactView<'_, Iso16757Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel, picked: &[String], frame: &semio_s_plugin_norm_app_surface::LaneFrame) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
    let live = semio_s_plugin_norm_app_surface::LiveView::<Iso16757Family>::with_lanes(doc.snapshot.clone(), semio_s_plugin_norm_results_config::current::<checks::ChecksWindowConfigOwner>(cfg).selected_check_index, picked, &frame.transient);
    let host = &live.host;
    let selection = &live.selection;
    let selected = selection.checks_selected();
    let focus = live.focus.clone();
    let worst = live.worst.as_deref();
    match body_key {
        outliner::BODY_OUTLINER => outliner::render(host, selection, view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, outliner::BODY_OUTLINER)),
        properties::BODY_PROPERTIES => properties::render(host, selection, view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, properties::BODY_PROPERTIES)),
        viewport::BODY_VIEWPORT => viewport::render(host.document(), host.report(), selection, frame, view_state.locale),
        diagrams::BODY_DIAGRAMS => diagrams::render(host.document(), host.report(), focus.as_deref().or(worst), &semio_plugin::TreeWindows::for_body(view_state, diagrams::BODY_DIAGRAMS), view_state.locale),
        checks::BODY_CHECKS => live.render_checks(&semio_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale, CONTROLLER_ID, checks::INTERACTION_DOMAIN),
        document_panel::BODY_ARTIFACT => document_panel::render(host, view_state.locale),
        catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(Iso16757PlayApp::examples(), view_state.locale, CONTROLLER_ID),
        inspection_panel::BODY_INSPECTION => inspection_panel::render(host, selected, view_state.locale, Some(CONTROLLER_ID)),
        _ => semio_s_plugin_norm_app_surface::render_unknown_body(body_key, view_state.locale),
    }
    .map(semio_plugin::built_to_component_tree)
}
//#endregion 🪄️LiveRender

//#region 🫧️LiveEditor
fn live_entities(document: &Iso16757Snapshot) -> Vec<semio_s_plugin_norm_structural_scene::EntityRef> {
    viewport::scene_of(document, &<Iso16757Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document)).entities.into_iter().map(|entity| entity.entity).collect()
}

fn live_handles(document: &Iso16757Snapshot, selection: &[semio_s_plugin_norm_structural_scene::EntityRef]) -> Vec<semio_s_plugin_norm_structural_scene::Handle> {
    viewport::handles_of(document, selection)
}

semio_s_plugin_norm_app_surface::norm_live_editor!(Iso16757PlayApp, Iso16757Command, handles = live_handles, entities = live_entities, palette = viewport::insert_specs, resolve = semio_s_artifact_norm_iso16757_mutations::resolve_edit, scene = |d, r, _| viewport::scene_of(d, r), diagrams = |d, r| diagrams::all(d, r));
semio_s_plugin_norm_app_surface::norm_live_acceptance!(Iso16757PlayApp, entities = live_entities);
//#endregion 🫧️LiveEditor
''')
rep("    type ResultsWindowConfigOwner = results::ResultsWindowConfigOwner;\n","""    type ResultsWindowConfigOwner = checks::ChecksWindowConfigOwner;

    const TOOL_IDS: &'static [&'static str] = semio_s_plugin_norm_app_surface::NORM_EDITOR_TOOL_IDS;
    const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = semio_s_plugin_norm_app_surface::NORM_EDITOR_PUBLICATION_CONTRACTS;

    fn check_focus_effects(command: &Iso16757Command, document: &Iso16757Snapshot) -> Vec<semio_plugin::Effect> {
        match command {
            Iso16757Command::SetSelectedCheckIndex(payload) => semio_s_plugin_norm_app_surface::live_check_focus_effects::<Self>(document, payload.index, checks::INTERACTION_DOMAIN),
            _ => Vec::new(),
        }
    }

    fn view_focus_effects(command: &Self::Command, document: &Self::Snapshot, transient: &Self::Transient, interaction: &protocol::InteractionState) -> Vec<semio_plugin::Effect> {
        semio_s_plugin_norm_app_surface::live_view_focus_effects::<Self>(command, document, transient, interaction, checks::INTERACTION_DOMAIN)
    }
""")
rep('''        semio_s_artifact_norm_iso16757_inferences::evaluate(document)
    }
''','''        semio_s_artifact_norm_iso16757_inferences::live::report(document).as_ref().clone()
    }

    fn live_report(document: &Iso16757Snapshot) -> std::sync::Arc<semio_s_plugin_norm_compliance::CheckReport> {
        semio_s_artifact_norm_iso16757_inferences::live::report(document)
    }

    fn live_progress() -> semio_s_plugin_norm_compliance::incremental_evaluation::EvalProgress {
        semio_s_artifact_norm_iso16757_inferences::live::progress()
    }

    fn live_responsive(document: &Iso16757Snapshot, budget: std::time::Duration) -> (std::sync::Arc<semio_s_plugin_norm_compliance::CheckReport>, bool) {
        semio_s_artifact_norm_iso16757_inferences::live::responsive(document, budget)
    }

    fn cancel_live() {
        semio_s_artifact_norm_iso16757_inferences::live::cancel()
    }
''')
rep(".window_kind_def(inputs::definition())\n            .window_kind_def(results::definition())\n            .default_layout(edit_mode::layout())",""".interaction(semio_s_plugin_norm_app_surface::viewport_interaction_definition(VARIANT, LocalizedLabel::native("ISO 16757 model", "ISO-16757-Modell"), ENTITY_KIND_SCENE_KINDS))
            .window_kind_def(outliner::definition())
            .window_kind_def(properties::definition())
            .window_kind_def(viewport::definition())
            .window_kind_def(diagrams::definition())
            .window_kind_def(checks::definition())
            .default_layout(edit_mode::layout())
            .named_layout(semio_s_plugin_norm_app_surface::edit_named_layout(VARIANT, "desktop"))
            .named_layout(semio_s_plugin_norm_app_surface::edit_named_layout(VARIANT, "mobile"))
            .named_layout(semio_s_plugin_norm_app_surface::edit_named_layout(VARIANT, "tablet"))""")
# extra actions copied from en1996 manifest
src=open('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs',encoding='utf8').read()
a=src.index('            .action_with(\n                semio_plugin::ActionDefinition::new_catalog("setFields"')
b=src.index('            .action_describe("setSelectedCheckIndex"')
extra=src[a:b].replace('.keybinding("mod+z", "undo")\n            .keybinding("mod+shift+z", "redo")\n            .keybinding("escape", "clearPreview")\n','')
c=src.index('            .action_describe("setFields"')
d=src.index('            .build_definition()',c)
desc=src[c:d]
rep('            .keybinding("mod+z", "undo")\n            .keybinding("mod+shift+z", "redo")\n',extra+'            .keybinding("mod+z", "undo")\n            .keybinding("mod+shift+z", "redo")\n            .keybinding("escape", "clearPreview")\n')
rep('            .build_definition()',desc+'            .build_definition()')
# mounts
rep("""    #[path = "🎮️commands/✏️set-field/🦀️.rs"]
    pub mod set_field;
}""","""    #[path = "🎮️commands/✏️set-field/🦀️.rs"]
    pub mod set_field;
    semio_s_plugin_norm_app_surface::norm_live_commands!(crate::Iso16757PlayApp);
}""")
rep("""#[path = "."]
pub mod presence {""","""#[path = "."]
pub mod lanes {
    semio_s_plugin_norm_app_surface::norm_lane_stores!(presence_extension = "norm.iso16757.live.presence", transient_extension = "norm.iso16757.live.transient", owner = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️lanes");
}
#[path = "."]
pub mod presence {""")
open(p,'w',encoding='utf8').write(s)
