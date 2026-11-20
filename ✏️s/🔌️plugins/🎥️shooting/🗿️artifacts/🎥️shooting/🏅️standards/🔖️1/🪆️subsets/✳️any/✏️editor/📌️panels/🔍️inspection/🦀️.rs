//! 🔍️ Shooting inspector — edits the selected or active shot through semantic field controls.

use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::ui_helpers_and_io::{shooting_action, ui_capacity_error, ui_label, ui_text, ui_value_list, ui_value_map, ui_value_text};
use semio_plugin::ui_node_list;
use crate::{ShootingShot, ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use semio_plugin::plugin_app_close_prelude::{InputKind, Trigger};
use semio_plugin::{Buildable, HasBase, HasChildren};
use semio_plugin::tree_item;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PanelGroup;
use semio_plugin::PanelTabDefinition;
use semio_plugin::PanelTabKind;
use semio_plugin::PanelTreeBuilder;
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
use semio_ui_contract::{input, BuiltNode, UiNumberBound, UiNumberLimits};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
const ROOT: &str = "shooting-play-inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SHOOTING_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn control_row(row_id: &str, label: &str, control: BuiltNode) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    semio_ui_contract::tree_item(ui_label(label)?)
        .try_id(row_id)
        .map_err(|_| ui_capacity_error())?
        .try_child(control)
        .map_err(|_| ui_capacity_error())?
        .try_build()
        .map_err(|_| ui_capacity_error())
}

fn picker_color(value: &str) -> std::borrow::Cow<'_, str> {
    if let Some(hex) = value.strip_prefix('#').filter(|hex| hex.len() == 3) {
        let mut full = String::from("#");
        for digit in hex.chars() {
            full.push(digit);
            full.push(digit);
        }
        return full.into();
    }
    value.into()
}

enum Field<'a> {
    Input(InputKind),
    Choice(&'a [(&'a str, &'a str)]),
}

fn shot_field(shot: &ShootingShot, name: &'static str, label: &str, value: &str, field: Field<'_>) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let id = format!("{ROOT}.shot.{name}");
    let args = ui_value_map([("field", ui_value_text(name)?), ("shotIds", ui_value_list([ui_value_text(&shot.id)?])?)])?;
    let (action, args) = shooting_action("patchShots", Some(args))?;
    let args = args.ok_or_else(ui_capacity_error)?;
    let input_id = format!("{id}.input");
    let control = match field {
        Field::Input(kind) => {
            let builder = input(kind).value(ui_text(value)?);
            let (builder, trigger) = match kind {
                InputKind::Text | InputKind::Number => (builder.commit(ui_text("blur")?).draft_target(ui_text(format!("{}:{name}", shot.id))?), Trigger::Commit),
                _ => (builder, Trigger::Change),
            };
            let bound = |value, exclusive| Some(UiNumberBound { value, exclusive, refusal: None });
            let builder = match name {
                "width" | "height" => builder.min(1.0).max(16_384.0).step(1.0),
                "visibleWidth" | "hiddenWidth" => builder.min(0.0).max(64.0).step(0.1)
                    .limits(UiNumberLimits { min: bound(0.0, true), max: bound(64.0, false) }),
                "fillOpacity" => builder.min(0.0).max(1.0).step(0.01),
                "lineEasing" => builder.min(0.0).max(10.0).step(0.1),
                "zoom" => builder.min(0.0).step(0.1)
                    .limits(UiNumberLimits { min: bound(0.0, true), max: None }),
                "fov" => builder.min(0.0).max(180.0).step(0.1)
                    .limits(UiNumberLimits { min: bound(0.0, true), max: bound(180.0, true) }),
                _ if kind == InputKind::Number => builder.step(0.1),
                _ => builder,
            };
            builder.try_label(label).map_err(|_| ui_capacity_error())?
                .try_id(input_id).map_err(|_| ui_capacity_error())?
                .try_on_with(trigger, action, args).map_err(|_| ui_capacity_error())?
                .try_build()
        }
        Field::Choice(options) => {
            let mut select = semio_ui_contract::select(ui_text(value)?).try_label(label).map_err(|_| ui_capacity_error())?;
            for (key, text) in options {
                select = select.try_item(ui_text(*key)?, ui_label(text)?).map_err(|_| ui_capacity_error())?;
            }
            select.try_id(input_id).map_err(|_| ui_capacity_error())?
                .try_on_with(Trigger::Change, action, args).map_err(|_| ui_capacity_error())?
                .try_build()
        }
    }.map_err(|_| ui_capacity_error())?;
    control_row(&id, label, control)
}

fn shot_inspector_group(snapshot: &ShootingSnapshot, cfg: &ShootingConfig, shot: &ShootingShot, labels: &ShootingLabels) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let fields = ui_node_list([
        shot_field(shot, "label", labels.field_label.as_str(), &shot.label, Field::Input(InputKind::Text)),
        shot_field(shot, "format", labels.field_format.as_str(), &shot.format, Field::Choice(&[("svg", labels.format_svg.as_str()), ("png", labels.format_png.as_str())])),
        shot_field(shot, "shape", labels.field_shape.as_str(), &shot.shape, Field::Choice(&[("rectangle", labels.shape_rectangle.as_str()), ("ellipse", labels.shape_ellipse.as_str())])),
        shot_field(shot, "width", labels.field_width.as_str(), &shot.width.to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "height", labels.field_height.as_str(), &shot.height.to_string(), Field::Input(InputKind::Number)),
    ])?;
    let camera = crate::shooting_resolve_shot_camera(snapshot, shot, &cfg.camera);
    let up = camera.up.unwrap_or([0.0, 0.0, 1.0]);
    let has_saved_camera = shot.camera_id.as_ref().is_some_and(|id| snapshot.saved_cameras.iter().any(|saved| &saved.id == id));
    let framing = match shot.auto_fit {
        Some(true) => "fit",
        Some(false) => "manual",
        None => "scene",
    };
    let projection = ui_node_list([
        shot_field(shot, "autoFit", labels.shot_framing.as_str(), framing, Field::Choice(&[
            ("scene", labels.shot_framing_scene.as_str()),
            ("fit", labels.shot_framing_fit.as_str()),
            ("manual", labels.shot_framing_manual.as_str()),
        ])),
        shot_field(shot, "projection", labels.drawing_projection.as_str(), camera.projection.as_deref().unwrap_or("perspective"), Field::Choice(&[("perspective", labels.drawing_perspective.as_str()), ("orthographic", labels.drawing_orthographic.as_str())])),
        shot_field(shot, "zoom", labels.drawing_zoom.as_str(), &camera.zoom.to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "fov", labels.drawing_fov.as_str(), &camera.fov.to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "eyeX", labels.eye_x.as_str(), &camera.position[0].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "eyeY", labels.eye_y.as_str(), &camera.position[1].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "eyeZ", labels.eye_z.as_str(), &camera.position[2].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "targetX", labels.target_x.as_str(), &camera.target[0].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "targetY", labels.target_y.as_str(), &camera.target[1].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "targetZ", labels.target_z.as_str(), &camera.target[2].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "upX", labels.up_x.as_str(), &up[0].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "upY", labels.up_y.as_str(), &up[1].to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "upZ", labels.up_z.as_str(), &up[2].to_string(), Field::Input(InputKind::Number)),
    ])?;
    let panel = PanelTreeBuilder::new(ROOT)?
        .section(format!("{ROOT}.shot"), Some(ui_label(labels.shot.as_str())?), true, fields)?
        .section(format!("{ROOT}.projection"), Some(ui_label(if has_saved_camera { labels.drawing_saved.as_str() } else { labels.drawing_live.as_str() })?), true, projection)?;
    if shot.format != "svg" {
        return panel.build();
    }
    let style = shot.drawing.clone().unwrap_or_default();
    let boolean = [("true", labels.drawing_enabled.as_str()), ("false", labels.drawing_disabled.as_str())];
    let drawing = ui_node_list([
        shot_field(shot, "background", labels.drawing_background.as_str(), &style.background.to_string(), Field::Choice(&boolean)),
        shot_field(shot, "fill", labels.drawing_fill.as_str(), &style.fill.to_string(), Field::Choice(&boolean)),
        shot_field(shot, "fillColor", labels.drawing_fill_color.as_str(), style.fill_color.as_deref().unwrap_or(""), Field::Input(InputKind::Text)),
        shot_field(shot, "fillOpacity", labels.drawing_fill_opacity.as_str(), &style.fill_opacity.to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "visibleLines", labels.drawing_visible_lines.as_str(), &style.visible_lines.to_string(), Field::Choice(&boolean)),
        shot_field(shot, "hiddenLines", labels.drawing_hidden_lines.as_str(), &style.hidden_lines.to_string(), Field::Choice(&boolean)),
        shot_field(shot, "visibleColor", labels.drawing_visible_color.as_str(), &picker_color(&style.visible_color), Field::Input(InputKind::Color)),
        shot_field(shot, "hiddenColor", labels.drawing_hidden_color.as_str(), &picker_color(&style.hidden_color), Field::Input(InputKind::Color)),
        shot_field(shot, "visibleWidth", labels.drawing_visible_width.as_str(), &style.visible_width.to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "hiddenWidth", labels.drawing_hidden_width.as_str(), &style.hidden_width.to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "lineEasing", labels.drawing_line_easing.as_str(), &style.line_easing.to_string(), Field::Input(InputKind::Number)),
        shot_field(shot, "mergeLines", labels.drawing_merge_lines.as_str(), &style.merge_lines.to_string(), Field::Choice(&boolean)),
    ])?;
    panel
        .section(format!("{ROOT}.drawing"), Some(ui_label(labels.drawing_section.as_str())?), true, drawing)?
        .build()
}

/// 🔍️ Resolves shot configuration selection, then the document's active shot, then a localized summary.
pub fn render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig, labels: &ShootingLabels) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let selected = cfg.selected_shot_ids.first().and_then(|id| snapshot.shots.iter().find(|shot| &shot.id == id));
    if let Some(shot) = selected.or_else(|| crate::active_shot(snapshot)) {
        return shot_inspector_group(snapshot, cfg, shot, labels);
    }
    let summary = ui_node_list([
        tree_item(format!("{ROOT}.schema"), SHOOTING_DOCUMENT_SCHEMA),
        tree_item(format!("{ROOT}.shots"), format!("{}: {}", labels.shots.as_str(), snapshot.shots.len())),
        tree_item(format!("{ROOT}.assets"), format!("{}: {}", labels.assets.as_str(), snapshot.assets.len())),
    ])?;
    PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.empty"), Some(ui_label(labels.inspection_title.as_str())?), true, summary)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests


