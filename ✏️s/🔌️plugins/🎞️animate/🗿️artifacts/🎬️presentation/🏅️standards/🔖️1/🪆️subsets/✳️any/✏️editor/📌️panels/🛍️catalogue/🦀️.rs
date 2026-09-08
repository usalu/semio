//! 🛍️ Animate presentation app panel — the catalogue: tile-seeding templates and the active figure source.

use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::terminology::AnimatePresentationLabels;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use semio_framework_ui_contract::{button, column, field, input, section, text, BuiltNode, HasBase, InputKind, Trigger, UiValue};
use crate::editor::animate::{animate_presentation_action, ui_capacity_error, ui_children, ui_label, ui_map, ui_node, ui_text};

//#region 🔖️Constants
pub const PRESENTATION_PLAY_BODY_CATALOGUE: &str = "animate.presentation.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PRESENTATION_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn catalogue_button(id: &str, label: &str, action: &str, args: Option<UiValue>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let builder = button(ui_label(label)?).icon(ui_text("plus")?);
    let (action, args) = animate_presentation_action(action, args)?;
    let builder = match args {
        Some(args) => builder.try_on_with(Trigger::Activate, action, args),
        None => builder.try_on(Trigger::Activate, action),
    }.map_err(|_| ui_capacity_error())?;
    ui_node(builder, id)
}

fn source_args() -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let source = crate::artifacts::presentation::default_figure_tile_source();
    let frame = ui_map([("height", UiValue::Number(source.frame.height)), ("width", UiValue::Number(source.frame.width)), ("x", UiValue::Number(source.frame.x)), ("y", UiValue::Number(source.frame.y))])?;
    let mut entries = vec![("frame", frame), ("kind", UiValue::Text(ui_text(&source.kind)?))];
    if let Some(value) = source.pdf_page { entries.push(("pdfPage", UiValue::Number(f64::from(value)))); }
    if let Some(value) = source.source_aspect { entries.push(("sourceAspect", UiValue::Number(value))); }
    entries.push(("src", UiValue::Text(ui_text(&source.src)?)));
    ui_map([("source", ui_map(entries)?)])
}

pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (source, _) = crate::artifacts::presentation::presentation_working_scene(deck);
    let templates = [
        ui_node(text(ui_label(labels.catalogue_seed_desc.as_str())?), "animate.presentation.play.catalogue.description")?,
        catalogue_button("animate.presentation.play.catalogue.seed-2x2", labels.catalogue_seed_2x2.as_str(), "seedGrid", Some(ui_map([("columns", UiValue::Number(2.0)), ("rows", UiValue::Number(2.0))])?))?,
        catalogue_button("animate.presentation.play.catalogue.seed-3x5", labels.catalogue_seed_3x5.as_str(), "seedGrid", Some(ui_map([("columns", UiValue::Number(5.0)), ("rows", UiValue::Number(3.0))])?))?,
        catalogue_button("animate.presentation.play.catalogue.add-tile", labels.catalogue_add_tile.as_str(), "addTile", None)?,
        catalogue_button("animate.presentation.play.catalogue.clear", labels.catalogue_clear_tiles.as_str(), "clearTiles", None)?,
    ];
    let templates = ui_node(ui_children(section(ui_label(labels.catalogue_tile_templates.as_str())?).default_open(true), templates)?, "animate.presentation.play.catalogue.templates")?;
    let source_input = ui_node(input(InputKind::Text).value(ui_text(&source.src)?).disabled(true), "animate.presentation.play.catalogue.figure.src.readonly")?;
    let figure = [
        catalogue_button("animate.presentation.play.catalogue.figure.catalogue", labels.catalogue_use_figure.as_str(), "setSource", Some(source_args()?))?,
        ui_node(ui_children(field(ui_label(labels.catalogue_active_source.as_str())?), [source_input])?, "animate.presentation.play.catalogue.figure.src")?,
        ui_node(text(ui_label(format!("{}: {}", labels.catalogue_media_kind.as_str(), source.kind))?), "animate.presentation.play.catalogue.figure.kind")?,
    ];
    let figure = ui_node(ui_children(section(ui_label(labels.catalogue_figure_templates.as_str())?).default_open(true), figure)?, "animate.presentation.play.catalogue.figure")?;
    ui_node(ui_children(column(), [templates, figure])?, "animate.presentation.play.catalogue")
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{presentation_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_CATALOGUE));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_templates() {
        let mut app = presentation_app().await;
        assert!(render_body(&mut app, PRESENTATION_PLAY_BODY_CATALOGUE).await.contains("animate.presentation.play.catalogue.templates"));
    }
}
//#endregion 🧪️Tests
