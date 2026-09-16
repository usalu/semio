//! 🔍️ Lowpoly play app panel — the active object's inspector (name, smooth shading, selection summary,
//! transform utility, staged utility-param sliders).

use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{active_object, utility_params_value, LowpolyView};
use crate::editor::lowpoly::{lowpoly_action, ui_label, ui_node_list, ui_value_map, ui_value_text};
use crate::LOWPOLY_DOCUMENT_SCHEMA;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_INSPECTION: &str = "lowpoly.play.inspection";
const ROOT: &str = "lowpoly-play-inspector";
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
fn inspector_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("lowpoly inspector admission failed at {stage}"))
}

fn control_row(id: &str, label: semio_framework_plugin::LabelText, control: semio_framework_plugin::BuiltNode) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    ui::tree_item(ui_label(label.as_str())?)
        .try_id(id)
        .map_err(|_| inspector_error("row-id"))?
        .try_child(control)
        .map_err(|_| inspector_error("row-child"))?
        .try_build()
        .map_err(|_| inspector_error("row-build"))
}

fn inspector_utility_param_field(id: &str, label: semio_framework_plugin::LabelText, key: &str, value: &serde_json::Value) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let current = value.get(key).map_or_else(|| "0".to_string(), |entry| entry.to_string());
    let (action, args) = lowpoly_action("setUtilityParam", Some(ui_value_map([("key", ui_value_text(key)?)])?))?;
    let mut number_input = ui::input(InputKind::Number)
        .value(UiText::try_from_string(current).map_err(|_| inspector_error("param-value"))?)
        .try_id(format!("{ROOT}.{id}.input"))
        .map_err(|_| inspector_error("param-input-id"))?;
    number_input = match args {
        Some(args) => number_input.try_on_with(Trigger::Change, action, args),
        None => number_input.try_on(Trigger::Change, action),
    }
    .map_err(|_| inspector_error("param-binding"))?;
    control_row(&format!("{ROOT}.{id}"), label, number_input.try_build().map_err(|_| inspector_error("param-input-build"))?)
}

pub fn render(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(object) = active_object(view) else {
        let rows = ui_node_list([
            tree_item_desc(format!("{ROOT}.empty.schema"), ui_label(format!("Schema: {LOWPOLY_DOCUMENT_SCHEMA}"))?, None)?,
            tree_item_desc(format!("{ROOT}.empty.hint"), ui_label("No active object")?, None)?,
        ])?;
        return PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.empty"), Some(ui_label(labels.object.as_str())?), true, rows)?.build();
    };
    let config = view.config;
    let params = utility_params_value(config);

    let (name_action, name_args) = lowpoly_action("patchObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?), ("field", ui_value_text("name")?)])?))?;
    let mut name_input = ui::input(InputKind::Text)
        .value(UiText::try_from_string(object.name.clone()).map_err(|_| inspector_error("name-value"))?)
        .try_id(format!("{ROOT}.object.name.input"))
        .map_err(|_| inspector_error("name-input-id"))?;
    name_input = match name_args {
        Some(args) => name_input.try_on_with(Trigger::Change, name_action, args),
        None => name_input.try_on(Trigger::Change, name_action),
    }
    .map_err(|_| inspector_error("name-binding"))?;
    let name_row = control_row(&format!("{ROOT}.object.name"), labels.name, name_input.try_build().map_err(|_| inspector_error("name-input-build"))?)?;

    let (smooth_action, smooth_args) = lowpoly_action("patchObject", Some(ui_value_map([("objectId", ui_value_text(&object.id)?), ("field", ui_value_text("smoothShading")?)])?))?;
    let mut smooth_toggle = ui::toggle(object.smooth_shading)
        .icon(UiText::try_from_str("sun").ok_or_else(|| inspector_error("smooth-icon"))?)
        .try_id(format!("{ROOT}.object.smooth.toggle"))
        .map_err(|_| inspector_error("smooth-input-id"))?;
    smooth_toggle = match smooth_args {
        Some(args) => smooth_toggle.try_on_with(Trigger::Change, smooth_action, args),
        None => smooth_toggle.try_on(Trigger::Change, smooth_action),
    }
    .map_err(|_| inspector_error("smooth-binding"))?;
    let smooth_row = control_row(&format!("{ROOT}.object.smooth"), labels.smooth_shading, smooth_toggle.try_build().map_err(|_| inspector_error("smooth-input-build"))?)?;

    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the selection summary/mode
    // rows used to read `LowpolyConfig`; the mesh domain's selection is framework-owned
    // `InteractionState` now, and `ArtifactApp::render` is not threaded an `InteractionView`
    // this wave — dropped rather than shown stale. Peer/self selection surfaces generically.
    let object_rows = ui_node_list([name_row, smooth_row])?;
    let transform_rows = ui_node_list([tree_item_desc(format!("{ROOT}.transform.utility"), ui_label(labels.utility.as_str())?, Some(active_utility.to_string()))?])?;
    let utility_param_rows = ui_node_list([
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
    ])?;
    PanelTreeBuilder::new(ROOT)?
        .section(format!("{ROOT}.object"), Some(ui_label(labels.object.as_str())?), true, object_rows)?
        .section(format!("{ROOT}.transform"), Some(ui_label(labels.transform.as_str())?), true, transform_rows)?
        .section(format!("{ROOT}.utility-params"), Some(ui_label(labels.utility_params.as_str())?), true, utility_param_rows)?
        .build()
}
//#endregion 🔖️Render
