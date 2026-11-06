//! 🪟️ Diagrams window body: every diagram of the current focus as a Canvas2d surface, its screen-reader description and its windowed table twin. See [`super`].

use super::*;
use crate::*;
use semio_plugin::scene_surface;

/// 🪟️ Canvas size and theme of one diagrams window, and the controller whose `setFields` the editable marker rows commit through (`None` keeps every marker row read-only).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiagramsWindowSpec {
    pub width: f64,
    pub height: f64,
    pub dark: bool,
    pub controller_id: Option<&'static str>,
}

impl Default for DiagramsWindowSpec {
    fn default() -> Self {
        Self { width: 560.0, height: 360.0, dark: false, controller_id: None }
    }
}

fn surface_id(body_key: &str, diagram: &Diagram2d) -> String {
    format!("{body_key}.{}", diagram.id.replace(':', "."))
}

fn table_rows(table: &TableTwin) -> Vec<(String, String)> {
    table.rows.iter().enumerate().map(|(index, cells)| (format!("{}", index + 1), cells.join(" · "))).collect()
}

/// 🖊️ The id of the inline editor in the marker row `index` of the table twin of the diagram surface `base`.
pub fn marker_cell_id(base: &str, index: usize) -> String {
    format!("{base}.marker.m{index}.input")
}

/// 🖊️ The arguments the cell of a marker commits with: the marker grip as the handle of an Enter gesture resting at the shown value; the text typed into the cell travels as the `value` of the commit and becomes the typed entry of that gesture, so the row and the drag are one commit path.
fn marker_cell_args(grip: &semio_s_plugin_norm_structural_scene::Handle, rest: f64) -> UiAssemblyResult<UiValue> {
    action_args_map(vec![("handle", ui_value_text(&grip.id)?), ("key", ui_value_text("Enter")?), ("s", UiValue::Number(rest)), ("origin", UiValue::Number(rest))])
}

/// 🖊️ The value of a cell to twelve significant digits without float noise or trailing zeros (`2.5`, `0.000125`); the text re-parses to the same number, so confirming an untouched cell commits nothing.
fn shown_value(value: f64) -> String {
    if value == 0.0 || !value.is_finite() {
        return "0".to_owned();
    }
    let places = (11 - value.abs().log10().floor() as i32).clamp(0, 15) as usize;
    let text = format!("{value:.places$}");
    if text.contains('.') { text.trim_end_matches('0').trim_end_matches('.').to_owned() } else { text }
}

/// 🖊️ One marker row: a read-only tree item, or, for a marker bound to a field, the row with an inline editor cell; the cell shows the bound value in the unit of the field, takes `350 cm` style text (Enter or leaving the cell commits, Escape restores the shown value) and is named by the marker and the unit for a screen reader.
fn marker_item(base: &str, index: usize, diagram: &Diagram2d, marker: &Marker, text: &str, controller_id: Option<&'static str>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{base}.marker.m{index}");
    let editable = controller_id.and_then(|controller| Some((controller, marker_grip(diagram, marker)?, marker_rest(diagram, marker)?)));
    let Some((controller_id, grip, rest)) = editable else { return tree_item_desc(row_id, norm_ui_label(text.to_owned())?, None) };
    let unit = grip.writes.first().map(|write| write.unit.clone()).unwrap_or_default();
    let shown = shown_value(grip.writes.first().map_or(rest, |write| write.value_at(rest)));
    let name = format!("{}, {}", copy_text(&marker.label, locale), chrome("edit value", "Wert bearbeiten", locale));
    let placeholder = if unit.is_empty() { chrome("value", "Wert", locale).to_owned() } else { format!("{} {unit}", chrome("value in", "Wert in", locale)) };
    let input = ui::input(InputKind::Text).value(ui_text(format!("{shown} {unit}").trim())?).placeholder(norm_ui_label(placeholder)?).commit(ui_text("blur")?).try_id(marker_cell_id(base, index)).map_err(|_| ui_error("ui.node.id"))?;
    let bound = bind_on(input, Trigger::Commit, controller_id, "setFields", marker_cell_args(&grip, rest)?)?.try_label(name.as_str()).map_err(|_| ui_error("ui.node.label"))?;
    control_row(&row_id, text, bound.into())
}

fn table_tree(body_key: &str, diagram: &Diagram2d, windows: &TreeWindows<'_>, controller_id: Option<&'static str>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let table = table_twin(diagram, locale);
    let base = surface_id(body_key, diagram);
    let markers: Vec<(usize, &Marker, String)> = diagram.markers.iter().zip(&table.marker_rows).enumerate().map(|(index, (marker, cells))| (index, marker, cells.join(" · "))).collect();
    PanelTreeBuilder::new(format!("{base}.table").as_str())?
        .section(
            format!("{base}.markers"),
            Some(norm_ui_label(table.marker_headers.join(" · "))?),
            true,
            ui_node_list(markers.iter().map(|(index, marker, text)| marker_item(&base, *index, diagram, marker, text, controller_id, locale)))?,
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{base}.rows"),
            Some(norm_ui_label(table.headers.join(" · "))?),
            false,
            &table_rows(&table),
            |(key, text)| tree_item_desc(format!("{base}.row.{key}"), norm_ui_label(text.clone())?, None),
            norm_ui_label(chrome("No data.", "Keine Daten.", locale))?,
        )?
        .build()
}

/// 🏷️ Text owned by one diagram, with the same identity for short and chunked content.
fn diagram_text(base: &str, role: &str, text: &str) -> UiAssemblyResult<BuiltNode> {
    let mut node = render_text_chunks(text)?;
    node.key = ui_text(format!("{base}.{role}"))?;
    Ok(node)
}

/// 📈️ The diagrams window body for `body_key`; an empty set shows a hint instead of an empty canvas. Every diagram is published for the screen reader as an image node described by the table node of its twin.
pub fn render_diagrams(body_key: &str, diagrams: &[Diagram2d], spec: &DiagramsWindowSpec, windows: &TreeWindows<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let lang = crate::viewport3d::config::Lang::of(locale);
    crate::accessibility::published_nodes::publish("diagrams", &crate::accessibility::published_nodes::domain_of_body(body_key), crate::accessibility::published_nodes::diagram_window_nodes(diagrams, lang));
    if diagrams.is_empty() {
        return render_text(chrome("Select a member, joint or check to see its diagrams.", "Bauteil, Anschluss oder Nachweis wählen, um dessen Diagramme zu sehen.", locale));
    }
    let mut children = Vec::new();
    for diagram in diagrams {
        let base = surface_id(body_key, diagram);
        let (canvas, _) = diagram_scene(diagram, &LoweringOptions { width: spec.width, height: spec.height, locale, dark: spec.dark, selected: None, hovered: None, min_target_px: crate::device_profiles::touch_hit_px() });
        children.push(diagram_text(&base, "title", &copy_text(&diagram.title, locale))?);
        if let Some(basis) = diagram.basis() {
            children.push(diagram_text(&base, "basis", &copy_text(basis, locale))?);
        }
        children.push(scene_surface(&base, semio_ui_contract::SurfaceKind::Canvas2d, &canvas)?);
        children.push(diagram_text(&base, "description", &accessible_description(diagram, locale))?);
        children.push(table_tree(body_key, diagram, windows, spec.controller_id, locale)?);
    }
    ui::column().try_children(children).map_err(|_| ui_error("ui.node.children"))?.try_build().map_err(|_| ui_error("ui.node.build"))
}
