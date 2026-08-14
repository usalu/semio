//! 🔥️ CAD play app — the Energy window: the `aec.building.energy` pane's world-3d viewport.

use crate::apps::cad::modes::edit;
use crate::apps::cad::modes::edit::options;
use crate::apps::cad::terminology::CadLabels;
use crate::apps::cad::config::CadDislocateOptions;
use crate::apps::cad::{CadPlayRuntime, CadPlayView};
use crate::artifacts::cad::CadPaneId;
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowEngagement, WindowKindDefinition, WindowMeasure, WindowOptions};
use ui_wgpu::wgpu::SurfaceKind;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "cad-play-energy";
pub const BODY_KEY: &str = "cad.play.energy";
pub const SURFACE_ID: &str = "cad.play.scene3d/energy";
pub const PANE: CadPaneId = CadPaneId::Energy;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::cad::create_cad_app`. `options.measures` stays
/// empty on purpose: cad's measures are config-derived and rebuilt per frame by [`window_measures`],
/// never frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Energy", "Energie"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "sun".into(),
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

/// 🎚️ The live chrome measures for this window, collected from the mode's `🎚️options/*` components.
pub fn window_measures(runtime: &CadPlayRuntime, is_de: bool) -> Vec<WindowMeasure> {
    vec![options::projection::measure(runtime, PANE), options::sun::measure(runtime), options::dislocate::measure(runtime.dislocate_options(WINDOW_KIND_ID), is_de)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(view: &CadPlayView, active_utility: Option<&str>, options: CadDislocateOptions) -> UiNode {
    edit::build_world_scene_for_pane(view, PANE, SURFACE_ID, active_utility, options)
}

pub fn engagement(view: &CadPlayView, labels: &CadLabels) -> WindowEngagement {
    edit::cad_window_engagement(view, PANE, labels)
}
//#endregion 🔖️Render
