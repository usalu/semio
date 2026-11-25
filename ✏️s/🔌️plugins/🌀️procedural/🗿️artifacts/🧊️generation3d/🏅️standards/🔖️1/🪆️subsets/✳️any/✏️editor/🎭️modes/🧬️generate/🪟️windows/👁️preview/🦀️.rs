//! 👁️ Generation3d play app — the generation output-preview window (generate mode): a tessellated
//! preview of the patched fixture's evaluated geometry.

use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_window_measures;
use semio_s_artifact_procedural_3d_terminology::Generation3dLabels;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_PLAY_APP_ID;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_camera_json;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_payload;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_selection_json;
use semio_s_artifact_procedural_3d_preview_eval::preview_status_json;
use semio_s_artifact_procedural_3d_preview_eval::preview_window_status_json;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewPayload;
use semio_s_artifact_procedural_3d_preview_eval::{PreviewEvaluation, PreviewEvaluationSource, PreviewStatusDebug};
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_DOMAIN;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_GRANULARITY;
use semio_s_artifact_procedural_3d_model::generation_by_id;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
use semio_os_flow::FlowEvalSession;
use semio_plugin::world3d_scene;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowMeasure;
use semio_plugin::WindowOptions;

#[path = "🫧️transient/🦀️.rs"]
pub mod transient;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "generation3d-generate-preview";
pub const GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
const GENERATION_3D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🔬️ Shares the Edit viewport's controls in the Generate viewport.
pub fn window_measures(config: &Generation3dConfig, is_de: bool, procedural_action: impl Fn(&str, Option<semio_plugin::DslValue>) -> semio_plugin::ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    preview_window_measures(config, is_de, procedural_action)
}
//#endregion 🔖️Definition

/// 📈 The ONE status object this window publishes, in every state — the generate-mode ENTRY state
/// included. It carries the session's tessellation `phase`/`progress` (so the shell's world host can
/// show and cancel an evaluation the same way edit-mode preview does), the `debug` counters the
/// browser probe reads back off `data-status-json`, the `previewEval` run's abort affordance, and — only
/// while there is nothing to show — the
/// authored `hint`.
///
/// 🐛️ Why the window is a world host even with nothing evaluated: falling back to a `TextEditor`
/// surface published NO status host at all, so entering generate mode left the playground with
/// `hosts=[]` and no window a status could ever appear on — measured on 6018 as 189 s of
/// `[data-status-json]`-less generate mode (`🗑️generated/journey-3/results.json`, ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END). A hint is a state of this window, not a different window.
fn generate_preview_status_json(session: &FlowEvalSession, run: Option<semio_plugin::ToolRunPollView<'_>>, payload: &PreviewPayload, preview_status: Option<String>, hint: Option<&str>) -> Option<String> {
    preview_window_status_json(Some(session), run, preview_status, &PreviewStatusDebug { meshes_json: &payload.meshes_json, instances_json: &payload.instances_json }, hint)
}

//#region 🔖️Render
pub fn render(
    host_snapshot: &FlowHostSnapshot,
    generation: &GenerationPlayState,
    selected_id: Option<&str>,
    preview_eval_text: Option<&str>,
    cfg: &Generation3dConfig,
    labels: &Generation3dLabels,
    active_utility: &str,
    marks: &PreviewInteractionMarks,
    session: &FlowEvalSession,
    run: Option<semio_plugin::ToolRunPollView<'_>>,
    windows: &semio_plugin::TreeWindows<'_>,
    locale: semio_plugin::Locale,
    hover_groups: &mut Option<semio_s_artifact_procedural_3d_preview_eval::PreviewHoverGroups>,
) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let selected_generation = generation_by_id(generation, selected_id);
    let evaluation = PreviewEvaluation::from_source(selected_generation.and(preview_eval_text).unwrap_or_default(), PreviewEvaluationSource::Current);
    let marks = PreviewInteractionMarks { components: Default::default(), ..marks.clone() };
    let (mut payload, preview_status) = match selected_generation {
        Some(_) => {
            let payload = preview_payload(&evaluation, host_snapshot, cfg, Some(session), &marks);
            let preview_status = preview_status_json(&evaluation, host_snapshot);
            (payload, preview_status)
        }
        None => (PreviewPayload::default(), None),
    };
    *hover_groups = Some(std::mem::take(&mut payload.hover_groups));
    let empty = payload.meshes_json == "[]" && payload.instances_json == "[]";
    let status_json = generate_preview_status_json(session, run, &payload, preview_status, empty.then(|| labels.preview_hint.as_str()));
    let fit_json = semio_s_artifact_procedural_3d_preview_eval::preview_fit_json(host_snapshot, payload.bounds);
    let sun = cfg.sun();
    let selection_json = preview_selection_json(cfg, active_utility, &payload);
    let _ = GENERATION_3D_PLAY_APP_ID;
    let surface = semio_s_procedural_core_semantic_ui::accessible_scene_surface(
        GENERATION_3D_PLAY_SURFACE_GENERATE_PREVIEW,
        semio_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_ui_declaration::ui::World3dScene {
            status_json,
            fit_json: Some(fit_json),
            domain_id: Some(GENERATION_3D_INTERACTION_DOMAIN.into()),
            domain_granularity_id: Some(GENERATION_3D_INTERACTION_GRANULARITY.into()),
            ..world3d_scene(preview_camera_json(cfg), payload.meshes_json, payload.instances_json, selection_json, &sun)
        },
        labels.preview_canvas.as_str(),
        labels.preview_canvas_hint.as_str(),
        semio_ui_contract::Liveness::Polite,
    )?;
    let rows = semio_s_artifact_procedural_3d_preview_eval::preview_value_rows(&evaluation);
    let hover = semio_s_artifact_procedural_3d_preview_eval::PreviewChannelHover::new(host_snapshot, &marks.hovered, None);
    let hovered_ports = hover.matching(rows.iter().map(|row| row.id.as_str()));
    semio_s_procedural_core_semantic_ui::scene_with_channel_values(surface, windows, GENERATION_3D_PLAY_APP_ID, "generation3d-generate-values", &semio_s_procedural_core_semantic_ui::ChannelValueLabels::new(locale, labels.graph_input_port.as_str(), labels.graph_output_port.as_str()), &rows, &hovered_ports)
}
//#endregion 🔖️Render
