//! 🧾️ Typed, unit-aware properties inspector of the norm app surface, replacing the blind value-tree walker. See [`super`].

use crate::*;
use semio_framework_value::DslValue;

#[path = "🎛️controls/🦀️.rs"]
mod controls;
pub use controls::*;


#[path = "🗃️bulk-inspector/🦀️.rs"]
mod bulk_inspector;
pub use bulk_inspector::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️admission/🦀️.rs"]
mod admission_tests;

//#region 🧾️Options
/// 🧾️ Window-level inspector options: the unit system, the disclosure tier, whether clause rows open the clause panel, the outliner state fed by the host and the selected check whose parameters get a section.
#[derive(Clone, Debug, PartialEq)]
pub struct InspectorOptions {
    pub preset: UnitSystem,
    pub tier: Tier,
    pub clause_links: bool,
    pub unit_picker: bool,
    pub outliner: OutlinerState,
    pub annex_parameters: Option<AnnexParametersView>,
    pub check: Option<usize>,
}

impl InspectorOptions {
    /// 🧾️ Options with the two choices every window must make explicitly (unit system and disclosure tier); the remaining fields start empty.
    pub fn new(preset: UnitSystem, tier: Tier) -> Self {
        Self { preset, tier, clause_links: false, unit_picker: false, outliner: OutlinerState::default(), annex_parameters: None, check: None }
    }
}

/// 🏳️ Which registry rows the inspector lists as "Annex parameters": the id prefix of the family (`ndp.en1993.`) and the annex of the document; [`render_inspector`] derives it from the family and the document when unset.
#[derive(Clone, Debug, PartialEq)]
pub struct AnnexParametersView {
    pub prefix: String,
    pub annex: semio_s_plugin_norm_compliance::AnnexChoice,
}

struct InspectorContext<'a, 'w> {
    outliner: OutlinerContext<'a, 'w>,
    meta_fn: Option<NormFieldMetaFn>,
    preset: UnitSystem,
    tier: Tier,
    clause_links: bool,
    root: &'a DslValue,
}
//#endregion 🧾️Options

//#region 🧾️FieldViews
fn leaf_key(path: &str) -> &str {
    path.rsplit('.').next().unwrap_or(path)
}

fn collection_options(root: &DslValue, name: &str) -> Vec<FieldOption> {
    let DslValue::Object(entries) = root else { return Vec::new() };
    let Some((_, DslValue::Array(items))) = entries.iter().find(|(key, _)| key == name) else { return Vec::new() };
    items.iter().enumerate().filter(|(_, item)| !entity_id(item).is_empty()).map(|(index, item)| FieldOption { value: entity_id(item), label: entity_title(item, index) }).collect()
}

fn reference_collection(root: &DslValue, key: &str) -> Option<String> {
    let stem = key.strip_suffix("Id")?;
    let DslValue::Object(entries) = root else { return None };
    [format!("{stem}s"), format!("{stem}es"), stem.to_string()].into_iter().find(|name| entries.iter().any(|(key, value)| key == name && matches!(value, DslValue::Array(_))))
}

fn choice_options(choices: &[NormFieldChoice], locale: Locale) -> Vec<FieldOption> {
    choices.iter().map(|choice| FieldOption { value: choice.value.to_string(), label: chrome(choice.label_en, choice.label_de, locale) }).collect()
}

fn inferred_control(key: &str, value: &DslValue, root: &DslValue) -> FieldControl {
    match value {
        DslValue::Bool(_) => FieldControl::Toggle,
        DslValue::Number(number) if number.is_integer() => FieldControl::Int,
        DslValue::Number(_) => FieldControl::Quantity { unit: UnitDisplay::plain() },
        _ => match reference_collection(root, key) {
            Some(to) => FieldControl::EntityRef { options: collection_options(root, &to), to },
            None => FieldControl::Text,
        },
    }
}

/// 🧾️ Resolves how a document value is edited from its field spec (kind, unit, range, step, help, clause, annex parameter), falling back to the value type and the surrounding entity collections.
pub fn field_view(path: &str, value: &DslValue, root: &DslValue, locale: Locale, meta_fn: Option<NormFieldMetaFn>, system: UnitSystem) -> FieldView {
    let spec = resolve_field_meta(path, meta_fn);
    let label = field_label(path, spec, locale);
    let unit = spec.and_then(|spec| UnitDisplay::of_spec(&spec, system));
    let control = match (spec.map(|spec| spec.kind), unit) {
        (_, Some(unit)) => FieldControl::Quantity { unit },
        (Some(FieldKind::Text), _) => FieldControl::Text,
        (Some(FieldKind::Bool), _) => FieldControl::Toggle,
        (Some(FieldKind::Int), _) => FieldControl::Int,
        (Some(FieldKind::Choice(choices)), _) => FieldControl::Choice(choice_options(choices, locale)),
        (Some(FieldKind::EntityRef { to }), _) => FieldControl::EntityRef { to: to.to_owned(), options: collection_options(root, to) },
        _ => inferred_control(leaf_key(path), value, root),
    };
    let mut view = FieldView::new(path, label, control);
    view.read_only = spec.is_none_or(|spec| spec.read_only);
    view.precision = match (&view.control, spec) {
        (FieldControl::Int, _) => 0,
        (_, Some(spec)) => u16::from(spec.precision),
        _ => 3,
    };
    if let Some(spec) = spec {
        view.range = spec.range.map(|range| (range.min, range.max));
        view.soft_range = spec.soft_range.map(|range| (range.min, range.max));
        view.step = spec.step;
        view.help = spec.help.map(|help| help.resolve(locale).to_owned());
        view.clause = spec.clause;
        view.ndp = spec.ndp.map(|ndp| NdpBadge { symbol: ndp.symbol.to_owned(), clause: ndp.clause, default: ndp.default });
    }
    view
}

fn scalar_text(value: &DslValue) -> String {
    match value {
        DslValue::String(text) => text.clone(),
        DslValue::Bool(on) => on.to_string(),
        DslValue::Number(number) => number.as_f64().to_string(),
        _ => String::new(),
    }
}

fn shown_in(spec: Option<FieldSpec>, siblings: &[(String, DslValue)], tier: Tier) -> bool {
    let Some(spec) = spec else { return true };
    let predicate = spec.visible_if.is_none_or(|predicate| {
        let equal = siblings.iter().find(|(key, _)| key == predicate.path).is_some_and(|(_, value)| scalar_text(value) == predicate.value);
        equal == (predicate.op == PredicateOp::Equals)
    });
    predicate && spec.tier.rank() <= tier.rank()
}
//#endregion 🧾️FieldViews

//#region 🧾️Render
fn si_hint(field: &FieldView, value: &DslValue, locale: Locale) -> Option<String> {
    match (&field.control, value) {
        (FieldControl::Quantity { unit }, DslValue::Number(number)) if unit.factor != 1.0 => Some(format!("SI: {} {}", format_number(number.as_f64(), 6, locale), unit.si)),
        _ => None,
    }
}

fn field_row(context: &InspectorContext<'_, '_>, field: &FieldView, value: &DslValue) -> UiAssemblyResult<BuiltNode> {
    let locale = context.outliner.locale;
    let controller_id = context.outliner.controller_id;
    let row_id = format!("norm-inputs.{}", field.path);
    let badge = field.ndp.as_ref().map(|ndp| format!(" [NDP {}]", ndp.symbol)).unwrap_or_default();
    let mut item = ui::tree_item(norm_ui_label(format!("{}{badge}", field.labelled()))?).try_id(&row_id).map_err(|_| ui_error("ui.node.id"))?;
    if let Some(description) = field.help.clone().or_else(|| si_hint(field, value, locale)) {
        item = item.description(ui_text(description)?);
    }
    for control in field_controls(field, value, &row_id, controller_id, locale)? {
        item = item.try_child(control).map_err(|_| ui_error("ui.node.child"))?;
    }
    if let Some(message) = if let DslValue::Number(number) = value { field.violation(number.as_f64(), locale) } else { None } {
        item = item.try_child(tree_item_desc(format!("{row_id}.violation"), norm_ui_label(message)?, None)?).map_err(|_| ui_error("ui.node.child"))?;
    }
    if let Some(clause) = field.clause.or(field.ndp.as_ref().map(|ndp| ndp.clause)) {
        let clause = ClauseId::new(clause.family, clause.part, clause.section);
        item = item.try_child(clause_link_row(format!("{row_id}.clause"), &clause, locale, context.clause_links.then_some(controller_id))?).map_err(|_| ui_error("ui.node.child"))?;
    }
    if let Some(default) = field.ndp.as_ref().and_then(|ndp| ndp.default).filter(|_| !field.read_only) {
        if !matches!(value, DslValue::Number(number) if (number.as_f64() - default).abs() < f64::EPSILON) {
            let args = action_args_map(vec![("path", ui_value_text(&field.path)?), ("value", UiValue::Number(default))])?;
            let reset = tree_item_with_action(format!("{row_id}.ndp-reset"), norm_ui_label(chrome("Reset to annex default", "Auf Anhangswert zurücksetzen", locale))?, None, norm_action(controller_id, "setField", Some(args))?)?;
            item = item.try_child(reset).map_err(|_| ui_error("ui.node.child"))?;
        }
    }
    if let Some(feedback) = field_feedback(context.outliner.report, &field.path, locale) {
        item = item.try_child(utilization_bar(format!("{row_id}.utilisation"), feedback.status, feedback.utilization, &feedback.title, locale)?).map_err(|_| ui_error("ui.node.child"))?;
        if let Some(chip) = &feedback.remedy {
            let mut chip = chip.clone();
            chip.applicable &= !field.read_only;
            item = item.try_child(remedy_chip(format!("{row_id}.remedy"), &chip, controller_id, locale)?).map_err(|_| ui_error("ui.node.child"))?;
            if chip.applicable && controller_is_live(controller_id) {
                item = item.try_child(preview_row(format!("{row_id}.remedy.preview"), &chip.check_id, chip.remedy_index, 0, &chip.action, controller_id, locale)?).map_err(|_| ui_error("ui.node.child"))?;
                item = item.try_child(clear_preview_row(format!("{row_id}.remedy.preview-clear"), controller_id, locale)?).map_err(|_| ui_error("ui.node.child"))?;
            }
        }
    }
    item.try_build().map_err(|_| ui_error("ui.node.build"))
}

fn entries_section(context: &InspectorContext<'_, '_>, path: &str, label: String, entries: &[(String, DslValue)], default_open: bool, depth: usize) -> UiAssemblyResult<BuiltNode> {
    let locale = context.outliner.locale;
    let child_path_of = |key: &String| if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
    let pairs: Vec<(&String, &DslValue)> = entries.iter().filter(|(key, _)| shown_in(resolve_field_meta(&child_path_of(key), context.meta_fn), entries, context.tier)).map(|(key, child)| (key, child)).collect();
    tree_window_section_or_placeholder(
        context.outliner.windows,
        &inputs_section_id(path),
        norm_ui_label(format!("{label} ({})", pairs.len()))?,
        default_open,
        &pairs,
        |(key, child)| {
            render_node(context, &child_path_of(key), child, depth + 1)
        },
        norm_ui_label(chrome("Empty object", "Leeres Objekt", locale))?,
    )
}

fn scalar_list(context: &InspectorContext<'_, '_>, path: &str, label: String, items: &[DslValue], depth: usize) -> UiAssemblyResult<BuiltNode> {
    let locale = context.outliner.locale;
    let controller_id = context.outliner.controller_id;
    let indexed: Vec<(usize, &DslValue)> = items.iter().enumerate().collect();
    let section_id = inputs_section_id(path);
    let list = tree_window_section_or_placeholder(
        context.outliner.windows,
        &section_id,
        norm_ui_label(format!("{label} ({})", items.len()))?,
        false,
        &indexed,
        |(index, child)| {
            let child_path = list_element_path(path, child, *index);
            let remove_args = action_args_map(vec![("path", ui_value_text(path)?), ("index", UiValue::Number(*index as f64))])?;
            let mut row = UiFixedList::default();
            row.try_push(render_node(context, &child_path, child, depth + 1)?).map_err(|_| ui_error("ui.fixed-capacity"))?;
            if context.meta_fn.and_then(|lookup| lookup(path)).is_some_and(|meta| meta.can_remove) {
            row.try_push(tree_item_with_action(format!("norm-inputs.{path}.remove-{index}"), norm_ui_label(chrome("Remove", "Entfernen", locale))?, None, norm_action(controller_id, "removeItem", Some(remove_args))?)?).map_err(|_| ui_error("ui.fixed-capacity"))?;
            }
            tree_group(format!("norm-inputs.{path}.row-{index}"), norm_ui_label(list_element_selector(child, *index))?, false, row)
        },
        norm_ui_label(chrome("Empty list", "Leere Liste", locale))?,
    )?;
    if !context.outliner.windows.is_open(&section_id, false) {
        return Ok(list);
    }
    let mut children = UiFixedList::default();
    children.try_push(list).map_err(|_| ui_error("ui.fixed-capacity"))?;
    if context.meta_fn.and_then(|lookup| lookup(path)).is_some_and(|meta| meta.can_insert) {
    let insert_args = action_args_map(vec![("path", ui_value_text(path)?), ("index", UiValue::Number(items.len() as f64))])?;
    let add = tree_item_with_action(format!("norm-inputs.{path}.add"), norm_ui_label(chrome("Add item", "Eintrag hinzufügen", locale))?, None, norm_action(controller_id, "insertItem", Some(insert_args))?)?;
        children.try_push(add).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group(format!("norm-inputs.{path}.editor"), norm_ui_label(label)?, true, children)
}

fn render_node(context: &InspectorContext<'_, '_>, path: &str, value: &DslValue, depth: usize) -> UiAssemblyResult<BuiltNode> {
    let locale = context.outliner.locale;
    if depth > 24 {
        return tree_item_desc(format!("norm-inputs.{path}"), norm_ui_label(path)?, Some("…".into()));
    }
    let meta = resolve_field_meta(path, context.meta_fn);
    let label = field_label(path, meta, locale);
    match value {
        DslValue::Object(entries) => entries_section(context, path, if path.is_empty() { chrome("Document", "Dokument", locale) } else { label }, entries, path.is_empty(), depth),
        DslValue::Array(items) if is_entity_collection(items) => render_collection(&context.outliner, path, &label, items, false, context.meta_fn, |row| {
            let selected = context.outliner.state.selected.iter().any(|selected| *selected == row.path || (!row.id.is_empty() && *selected == row.id));
            let DslValue::Object(entries) = row.value else { return render_node(context, &row.path, row.value, depth + 1) };
            entries_section(context, &row.path, chrome("Properties", "Eigenschaften", locale), entries, selected, depth + 1)
        }),
        DslValue::Array(items) => scalar_list(context, path, label, items, depth),
        DslValue::Bytes(bytes) => tree_item_desc(format!("norm-inputs.{path}"), norm_ui_label(label)?, Some(format!("{} {}", bytes.len(), chrome("bytes", "Bytes", locale)))),
        scalar => field_row(context, &field_view(path, scalar, context.root, locale, context.meta_fn, context.preset), scalar),
    }
}

/// 📏️ The unit picker: one row per unit system (the chosen one marked) writing the view lane through `viewportAct`; the document stays in its stored units.
fn unit_picker(chosen: UnitSystem, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<BuiltNode> {
    let mut rows = UiFixedList::default();
    for (key, en, de, system) in [("engineering", "Engineering units (mm, kN, MPa)", "Ingenieureinheiten (mm, kN, MPa)", UnitSystem::Engineering), ("si", "SI base units (m, N, Pa)", "SI-Basiseinheiten (m, N, Pa)", UnitSystem::Si)] {
        let args = action_args_map(vec![("action", ui_value_text(VIEW_ACTION_UNITS)?), ("text", ui_value_text(key)?)])?;
        let label = format!("{} {}", if system == chosen { "●" } else { "○" }, chrome(en, de, locale));
        rows.try_push(tree_item_with_action(format!("norm-inputs.units.{key}"), norm_ui_label(label)?, None, norm_action(controller_id, "viewportAct", Some(args))?)?).map_err(|_| ui_error("ui.fixed-capacity"))?;
    }
    tree_group("norm-inputs.units".to_string(), norm_ui_label(chrome("Units", "Einheiten", locale))?, true, rows)
}

/// 🏳️ Registry text of a value: the number with its unit, the table or formula text, or a dash when the annex gives none.
fn ndp_value_text(value: semio_s_plugin_norm_compliance::NdpValue, unit: &str, locale: Locale) -> String {
    match value {
        semio_s_plugin_norm_compliance::NdpValue::Number(number) if unit.is_empty() => format_number(number, 6, locale),
        semio_s_plugin_norm_compliance::NdpValue::Number(number) => format!("{} {unit}", format_number(number, 6, locale)),
        semio_s_plugin_norm_compliance::NdpValue::Text(text) => text.to_string(),
        semio_s_plugin_norm_compliance::NdpValue::Unset => "—".to_string(),
    }
}

/// 🏳️ Numeric document fields bound to a registry id through `FieldSpec.ndp.id`: (id, path, stored value).
fn bound_fields(context: &InspectorContext<'_, '_>, path: &str, value: &DslValue, out: &mut Vec<(&'static str, String, f64)>, depth: usize) {
    if depth > 24 {
        return;
    }
    match value {
        DslValue::Object(entries) => {
            for (key, child) in entries {
                bound_fields(context, &if path.is_empty() { key.clone() } else { format!("{path}.{key}") }, child, out, depth + 1);
            }
        }
        DslValue::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                bound_fields(context, &list_element_path(path, child, index), child, out, depth + 1);
            }
        }
        DslValue::Number(number) => {
            if let Some(id) = resolve_field_meta(path, context.meta_fn).filter(|spec| !spec.read_only).and_then(|spec| spec.ndp).and_then(|ndp| ndp.id) {
                out.push((id, path.to_string(), number.as_f64()));
            }
        }
        _ => {}
    }
}

/// 🏳️ The "Annex parameters" section: every registry row of the family with its symbol and clause, the active value of the document annex, the EN and DE values, the verified or unverified badge, the source, and a reset to the active annex value for each document field bound to the row through `FieldSpec.ndp.id`. `None` when the family registers no parameter.
fn annex_parameters_section(context: &InspectorContext<'_, '_>, view: &AnnexParametersView) -> UiAssemblyResult<Option<BuiltNode>> {
    use semio_s_plugin_norm_compliance::{annex_parameter_rows, AnnexChoice, NdpStatus, NdpValue};
    let locale = context.outliner.locale;
    let controller_id = context.outliner.controller_id;
    let rows = annex_parameter_rows(&view.prefix, view.annex);
    if rows.is_empty() {
        return Ok(None);
    }
    let mut bound = Vec::new();
    bound_fields(context, "", context.root, &mut bound, 0);
    let section = tree_window_section_or_placeholder(
        context.outliner.windows,
        &inputs_section_id("annex-parameters"),
        norm_ui_label(format!("{} ({})", chrome("Annex parameters", "Anhangsparameter", locale), rows.len()))?,
        false,
        &rows,
        |row| {
            let entry = row.entry;
            let row_id = format!("norm-inputs.annex-parameters.{}", entry.id);
            let (en, de) = if view.annex == AnnexChoice::En { (row.active, row.other) } else { (row.other, row.active) };
            let badge = match entry.status {
                NdpStatus::Verified => chrome("verified", "geprüft", locale),
                NdpStatus::Unverified => chrome("unverified", "ungeprüft", locale),
            };
            let marker = if row.differs { "≠" } else { "=" };
            let description = format!(
                "{} {}: {} · EN {} / DE {} {marker}",
                chrome("Active", "Aktiv", locale),
                view.annex.label(),
                ndp_value_text(row.active, entry.unit, locale),
                ndp_value_text(en, entry.unit, locale),
                ndp_value_text(de, entry.unit, locale),
            );
            let mut item = ui::tree_item(norm_ui_label(format!("{} · {} [{badge}]", entry.symbol, semio_s_plugin_norm_compliance::designator(entry.clause, &locale)))?).try_id(&row_id).map_err(|_| ui_error("ui.node.id"))?.description(ui_text(description)?);
            item = item.try_child(tree_item_desc(format!("{row_id}.source"), norm_ui_label(chrome("Source", "Quelle", locale))?, Some(format!("{} · {} · {}", entry.source.document, semio_s_plugin_norm_compliance::locator_text(&entry.source, &locale), entry.source.edition)))?).map_err(|_| ui_error("ui.node.child"))?;
            if let NdpValue::Number(active) = row.active {
                for (index, (_, path, value)) in bound.iter().filter(|(id, _, value)| *id == entry.id && (value - active).abs() > f64::EPSILON).enumerate() {
                    let args = action_args_map(vec![("path", ui_value_text(path)?), ("value", UiValue::Number(active))])?;
                    let reset = tree_item_with_action(format!("{row_id}.reset-{index}"), norm_ui_label(format!("{} ({path}: {})", chrome("Reset to annex value", "Auf Anhangswert zurücksetzen", locale), format_number(*value, 6, locale)))?, None, norm_action(controller_id, "setField", Some(args))?)?;
                    item = item.try_child(reset).map_err(|_| ui_error("ui.node.child"))?;
                }
            }
            item.try_build().map_err(|_| ui_error("ui.node.build"))
        },
        norm_ui_label(chrome("No annex parameters", "Keine Anhangsparameter", locale))?,
    )?;
    Ok(Some(section))
}

/// 🧾️ Renders the typed inspector and outliner for a document value tree against a check report.
pub fn render_inspector_value(tree: &DslValue, report: &CheckReport, locale: Locale, controller_id: &'static str, meta_fn: Option<NormFieldMetaFn>, windows: &TreeWindows<'_>, options: &InspectorOptions) -> UiAssemblyResult<BuiltNode> {
    publish_host_selection(tree, &options.outliner.selected);
    if let Some(domain) = options.outliner.interaction_domain {
        publish_visible_rows(domain, tree, &options.outliner.filter);
    }
    let context = InspectorContext { outliner: OutlinerContext { report, locale, controller_id, windows, state: &options.outliner }, meta_fn, preset: options.preset, tier: options.tier, clause_links: options.clause_links, root: tree };
    let mut root = render_node(&context, "", tree, 0)?;
    if let Some(bulk) = render_selection_inspector(tree, &options.outliner.selected, locale, controller_id, meta_fn, options.preset)? {
        root = ui::column().try_children([bulk, root]).map_err(|_| ui_error("ui.node.children"))?.try_build().map_err(|_| ui_error("ui.node.build"))?;
    }
    let checked = options.check.map(|index| check_parameters_section(report, index, tree, locale, controller_id)).transpose()?.flatten();
    let annexed = options.annex_parameters.as_ref().map(|view| annex_parameters_section(&context, view)).transpose()?.flatten();
    let sections: Vec<BuiltNode> = checked.into_iter().chain(annexed).collect();
    if !sections.is_empty() {
        root = ui::column().try_children(std::iter::once(root).chain(sections)).map_err(|_| ui_error("ui.node.children"))?.try_build().map_err(|_| ui_error("ui.node.build"))?;
    }
    if !(options.unit_picker && controller_is_live(controller_id)) {
        return in_bottom_sheet(root);
    }
    let picker = unit_picker(options.preset, locale, controller_id)?;
    in_bottom_sheet(ui::column().try_children([picker, root]).map_err(|_| ui_error("ui.node.children"))?.try_build().map_err(|_| ui_error("ui.node.build"))?)
}

/// 🏳️ The annex parameters of a family: the registry prefix of its code (`EN 1993` is `ndp.en1993.`) and the annex the document selects (`annex` at the root, EN when absent).
fn annex_parameters_of<F: NormFamily>(tree: &DslValue) -> AnnexParametersView {
    let prefix = format!("ndp.{}.", F::family_id().label().to_ascii_lowercase().replace(' ', ""));
    let annex = match tree {
        DslValue::Object(entries) => entries.iter().find(|(key, _)| key == "annex").map(|(_, value)| value.clone()),
        _ => None,
    };
    let de = matches!(&annex, Some(DslValue::String(text)) if text.eq_ignore_ascii_case("de") || text.eq_ignore_ascii_case("de-na"));
    AnnexParametersView { prefix, annex: if de { semio_s_plugin_norm_compliance::AnnexChoice::De } else { semio_s_plugin_norm_compliance::AnnexChoice::En } }
}

/// 🧾️ The inputs window of a norm app: typed inspector and outliner over the host's document, with inline utilisation from its report.
pub fn render_inspector<F: NormFamily>(host: &NormHost<F>, locale: Locale, controller_id: &'static str, meta_fn: Option<NormFieldMetaFn>, windows: &TreeWindows<'_>, options: &InspectorOptions) -> UiAssemblyResult<BuiltNode> {
    render_inspector_projected(host, locale, controller_id, meta_fn, windows, options, |tree| tree)
}

/// 🧾️ [`render_inspector`] over the document value tree after `project` (views such as empty case-input rows that are not part of the document).
pub fn render_inspector_projected<F: NormFamily>(host: &NormHost<F>, locale: Locale, controller_id: &'static str, meta_fn: Option<NormFieldMetaFn>, windows: &TreeWindows<'_>, options: &InspectorOptions, project: impl FnOnce(DslValue) -> DslValue) -> UiAssemblyResult<BuiltNode> {
    let tree = project(semio_framework_value::ToValue::to_value(host.document()));
    let options = InspectorOptions { annex_parameters: options.annex_parameters.clone().or_else(|| Some(annex_parameters_of::<F>(&tree))), ..options.clone() };
    render_inspector_value(&tree, host.report(), locale, controller_id, meta_fn, windows, &options)
}
//#endregion 🧾️Render

/// 🧾️ Paint-path inputs window for a norm app: reads the settled live report for the document, then renders [`render_inspector`] with engineering units and the expert tier.
pub fn render_inspector_for<F: NormFamily>(document: &F::Document, locale: Locale, controller_id: &'static str, meta_fn: Option<NormFieldMetaFn>, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    render_inspector(&NormHost::<F>::live(document.clone()), locale, controller_id, meta_fn, windows, &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert))
}
