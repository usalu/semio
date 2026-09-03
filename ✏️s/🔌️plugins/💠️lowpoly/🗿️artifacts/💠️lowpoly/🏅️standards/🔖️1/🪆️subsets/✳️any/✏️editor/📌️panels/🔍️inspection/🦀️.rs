//! 🔍️ Lowpoly play app panel — the active object's inspector (name, smooth shading, selection summary,
//! transform utility, staged utility-param sliders).

use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;
use crate::editor::lowpoly::{lowpoly_action, ui_label, ui_value_map, ui_value_text};
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{active_object, utility_params_value, LowpolyView};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_INSPECTION: &str = "lowpoly.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(LOWPOLY_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🔢️ Builds one editable "label + number input" field row that dispatches `setUtilityParam`.
fn inspector_utility_param_field(id: &str, label: semio_framework_plugin::LabelText, key: &str, value: &serde_json::Value) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let current = value.get(key).map_or_else(|| "0".to_string(), |entry| entry.to_string());
    let (action, args) = lowpoly_action("setUtilityParam", Some(ui_value_map([("key", ui_value_text(key)?)])?))?;
    let mut number_input = ui::input(InputKind::Number)
        .value(UiText::try_from_string(current).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector param value admission failed"))?)
        .try_id(format!("lowpoly-play-inspector.{id}.input"))
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector param id admission failed"))?;
    number_input = match args {
        Some(args) => number_input.try_on_with(Trigger::Change, action, args),
        None => number_input.try_on(Trigger::Change, action),
    }
    .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector param action admission failed"))?;
    ui::field(ui_label(label.as_str())?)
        .try_id(format!("lowpoly-play-inspector.{id}"))
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector param field id admission failed"))?
        .try_children([number_input])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector param field children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector param field build failed"))
}

/// 🔒️ Builds a read-only "label + disabled text" field row — no action binding.
fn inspector_readonly_field(id: &str, label: semio_framework_plugin::LabelText, value: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let text_input = ui::input(InputKind::Text)
        .value(UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector readonly value admission failed"))?)
        .try_id(format!("lowpoly-play-inspector.{id}.input"))
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector readonly id admission failed"))?
        .disabled(true);
    ui::field(ui_label(label.as_str())?)
        .try_id(format!("lowpoly-play-inspector.{id}"))
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector readonly field id admission failed"))?
        .try_children([text_input])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector readonly field children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector readonly field build failed"))
}

pub fn render(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(object) = active_object(view) else {
        return ui::column()
            .try_id("lowpoly-play-inspector.empty")
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector empty id admission failed"))?
            .try_children([
                ui::text(ui_label(format!("Schema: {LOWPOLY_DOCUMENT_SCHEMA}"))?),
                ui::text(ui_label("No active object")?),
            ])
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector empty children admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector empty build failed"));
    };
    let config = view.config;
    let params = utility_params_value(config);

    let (name_action, name_args) = lowpoly_action("patchObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?), ("field", ui_value_text("name")?)])?))?;
    let mut name_input = ui::input(InputKind::Text)
        .value(UiText::try_from_string(object.name.clone()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector name value admission failed"))?)
        .try_id("lowpoly-play-inspector.object.name.input")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector name id admission failed"))?;
    name_input = match name_args {
        Some(args) => name_input.try_on_with(Trigger::Change, name_action, args),
        None => name_input.try_on(Trigger::Change, name_action),
    }
    .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector name action admission failed"))?;
    let name_field = ui::field(ui_label(labels.name.as_str())?)
        .try_id("lowpoly-play-inspector.object.name")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector name field id admission failed"))?
        .try_children([name_input])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector name field children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector name field build failed"))?;

    let (smooth_action, smooth_args) = lowpoly_action("patchObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?), ("field", ui_value_text("smoothShading")?)])?))?;
    let mut smooth_toggle = ui::toggle(object.smooth_shading)
        .icon(UiText::try_from_str("sun").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector smooth icon admission failed"))?)
        .try_id("lowpoly-play-inspector.object.smooth.toggle")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector smooth id admission failed"))?;
    smooth_toggle = match smooth_args {
        Some(args) => smooth_toggle.try_on_with(Trigger::Change, smooth_action, args),
        None => smooth_toggle.try_on(Trigger::Change, smooth_action),
    }
    .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector smooth action admission failed"))?;
    let smooth_field = ui::field(ui_label(labels.smooth_shading.as_str())?)
        .try_id("lowpoly-play-inspector.object.smooth")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector smooth field id admission failed"))?
        .try_children([smooth_toggle])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector smooth field children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector smooth field build failed"))?;

    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the selection summary/mode
    // rows used to read `LowpolyConfig`; the mesh domain's selection is framework-owned
    // `InteractionState` now, and `ArtifactApp::render` is not threaded an `InteractionView`
    // this wave — dropped rather than shown stale. Peer/self selection surfaces generically.
    let object_group = ui::section(ui_label(labels.object.as_str())?)
        .try_id("lowpoly-play-inspector.object")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector object group id admission failed"))?
        .try_children([name_field, smooth_field])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector object group children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector object group build failed"))?;

    let transform_group = ui::section(ui_label(labels.transform.as_str())?)
        .try_id("lowpoly-play-inspector.transform")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector transform group id admission failed"))?
        .try_children([inspector_readonly_field("transform.utility", labels.utility, active_utility)?])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector transform group children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector transform group build failed"))?;

    let utility_params_group = ui::section(ui_label(labels.utility_params.as_str())?)
        .default_open(true)
        .try_id("lowpoly-play-inspector.utility-params")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector utility-params group id admission failed"))?
        .try_children([
            inspector_utility_param_field("extrude", labels.extrude_distance, "extrudeDistance", &params)?,
            inspector_utility_param_field("inset", labels.inset_amount, "insetAmount", &params)?,
            inspector_utility_param_field("bevel", labels.bevel_amount, "bevelAmount", &params)?,
            inspector_utility_param_field("bevel-segments", labels.bevel_segments, "bevelSegments", &params)?,
            inspector_utility_param_field("loop-cuts", labels.loop_cuts, "loopCuts", &params)?,
            inspector_utility_param_field("decimate", labels.decimate_ratio, "decimateRatio", &params)?,
            inspector_utility_param_field("snap", labels.snap_grid, "snapGrid", &params)?,
            inspector_utility_param_field("mirror", labels.mirror_axis, "mirrorAxis", &params)?,
            inspector_utility_param_field("brush-size", labels.brush_size, "brushSize", &params)?,
            inspector_utility_param_field("brush-opacity", labels.brush_opacity, "brushOpacity", &params)?,
            inspector_utility_param_field("brush-hardness", labels.brush_hardness, "brushHardness", &params)?,
        ])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector utility-params group children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector utility-params group build failed"))?;

    ui::column()
        .try_id("lowpoly-play-inspector")
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector root id admission failed"))?
        .try_children([object_group, transform_group, utility_params_group])
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector root children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly inspector root build failed"))
}
//#endregion 🔖️Render
