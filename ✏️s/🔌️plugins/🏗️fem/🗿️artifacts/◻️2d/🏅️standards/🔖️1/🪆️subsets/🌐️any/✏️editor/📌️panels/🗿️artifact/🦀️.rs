//! 📄️ Fem2d play app panel — the artifact tree (outliner): every document entity as one row, in the
//! order an engineer builds a model (geometry, then restraints, then actions, then the properties
//! they reference, then the analysis settings).
//!
//! 🕹️ The tree is bound to the framework-owned `"fem2d"` interaction domain and every row's key is
//! the RAW entity id, so a row click, a viewport pick and the inspector address the one selection.
//! 🪙️ One interactive-row page for the whole tree: each section reserves the rows the sections after
//! it still need, and a document past the page closes with a `+N` continuation row rather than
//! failing an argument admission mid-row (see `semio_framework_plugin::paged_panel_section`).

use crate::editor::fem2d::interaction::{
    Fem2dInteractionSnapshot, FEM2D_GRANULARITY_COMBINATION, FEM2D_GRANULARITY_ELEMENT, FEM2D_GRANULARITY_LOAD, FEM2D_GRANULARITY_LOAD_CASE, FEM2D_GRANULARITY_MATERIAL, FEM2D_GRANULARITY_NODE, FEM2D_GRANULARITY_REGION, FEM2D_GRANULARITY_SECTION,
    FEM2D_GRANULARITY_SUPPORT, FEM2D_INTERACTION_DOMAIN,
};
use crate::editor::fem2d::terminology::Fem2dLabels;
use crate::editor::fem2d::{fem2d_action, ui_label, ui_node_list};
use crate::{element_id, load_id, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport, Fem2dSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, BuiltNode, Component, Label as UiLabel, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    paged_panel_section, panel_page_rows, tree_item_desc, tree_item_with_action, ActionId, LabelText, LocalizedLabel, PanelGroup, PanelRowBudget, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiFixedList,
    UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID,
};

//#region 🔖️Constants
pub const BODY_KEY: &str = "fem2d.play.artifact";
/// 🌳️ Id namespace every section key is minted under; row keys stay RAW entity ids.
pub const TREE_NAMESPACE: &str = "fem2d-play-artifact";
/// 🧾️ Sections this tree assembles, in order — nodes, elements, regions, supports, load cases,
/// combinations, materials, sections, analysis.
pub const SECTIONS: usize = 9;
/// 🎯️ Ids the builder records as selected/hovered — one `UiFixedList`, so a wider selection marks
/// its first page rather than refusing the whole render.
const MARKED_IDS_LIMIT: usize = semio_framework_ui_contract::UI_FIXED_LIST_ITEMS;
const FOCUS_ENTITY_ACTION: &str = "focusEntity";
const REMOVE_SELECTION_ACTION: &str = "removeSelection";
const FOCUS_ICON: &str = "focus";
const DELETE_ICON: &str = "trash-2";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"), group: PanelGroup::Workbench, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️UiValues
fn capacity_error(what: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", what)
}

fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value.as_ref()).map(UiValue::Text).ok_or_else(|| capacity_error("fem2d artifact UI text admission failed"))
}

fn ui_value_list(values: impl IntoIterator<Item = UiValue>) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| capacity_error("fem2d artifact UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| capacity_error("fem2d artifact UI list item admission failed"))?;
    }
    Ok(UiValue::List(builder.finish()))
}

/// 🗺️ One fixed argument map. 🔑️ `UiMapBuilder::push` admits keys in strictly ascending order only,
/// so every caller lists its entries sorted by key.
fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| capacity_error("fem2d artifact UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| capacity_error("fem2d artifact UI map entry admission failed"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

fn icon_text(icon: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(icon).ok_or_else(|| capacity_error("fem2d artifact icon admission failed"))
}
//#endregion 🔖️UiValues

//#region 🔖️Formatting
/// 🔢️ Display spelling of one scalar: scientific below a milli and above a mega (a section's `iy`
/// reads `3.69e-5`, never `0.0000369`), plain otherwise, and always with the typographic minus.
pub fn fem2d_scalar(value: f64) -> String {
    let magnitude = value.abs();
    let text = if value == 0.0 || (1e-3..1e6).contains(&magnitude) { format!("{value}") } else { format!("{value:.2e}") };
    typographic_minus(&text)
}

/// 📐️ A coordinate: always two decimals, so a column of nodes reads as a column.
pub fn fem2d_coordinate(value: f64) -> String {
    typographic_minus(&format!("{value:.2}"))
}

fn typographic_minus(text: &str) -> String {
    match text.strip_prefix('-') {
        Some(rest) => format!("−{rest}"),
        None => text.to_string(),
    }
}

fn dof_symbol(dof: FemDof, labels: &Fem2dLabels) -> &'static str {
    match dof {
        FemDof::Tx => labels.tx.as_str(),
        FemDof::Ty => labels.ty.as_str(),
        FemDof::Rz => labels.rz.as_str(),
        FemDof::Tz => "Tz",
        FemDof::Rx => "Rx",
        FemDof::Ry => "Ry",
    }
}

pub fn fem2d_node_label(node: &FemNode) -> String {
    format!("{} · ({}, {})", node.id, fem2d_coordinate(node.x), fem2d_coordinate(node.y))
}

pub fn fem2d_element_label(element: &FemElement, labels: &Fem2dLabels) -> String {
    match element {
        FemElement::Bar { id, start, end, .. } => format!("{id} · {} {start} → {end}", labels.bar.as_str()),
        FemElement::Beam { id, start, end, .. } => format!("{id} · {} {start} → {end}", labels.beam.as_str()),
    }
}

pub fn fem2d_region_label(region: &FemRegion, labels: &Fem2dLabels) -> String {
    format!("{} · {} · {} {}", region.id, region.name, region.outline.len(), labels.points.as_str())
}

pub fn fem2d_support_label(support: &FemSupport, labels: &Fem2dLabels) -> String {
    let dofs = support.fixed.iter().map(|dof| dof_symbol(*dof, labels)).collect::<Vec<_>>().join(" ");
    let dofs = if dofs.is_empty() { labels.none.as_str().to_string() } else { dofs };
    format!("{} · {} · {dofs}", support.id, support.node_id)
}

pub fn fem2d_load_label(load: &FemLoad, labels: &Fem2dLabels) -> String {
    match load {
        FemLoad::Nodal { node_id, dof, value, .. } => format!("{} {} N @ {node_id}", dof_symbol(*dof, labels), fem2d_scalar(*value)),
        FemLoad::MemberUdl { element_id, wx, wy, .. } => {
            if *wx == 0.0 {
                format!("{} {} N/m @ {element_id}", labels.wy.as_str(), fem2d_scalar(*wy))
            } else {
                format!("{} {} · {} {} N/m @ {element_id}", labels.wx.as_str(), fem2d_scalar(*wx), labels.wy.as_str(), fem2d_scalar(*wy))
            }
        }
        FemLoad::Area { region_id, pressure, .. } => format!("{} Pa @ {region_id}", fem2d_scalar(*pressure)),
    }
}

pub fn fem2d_load_case_label(case: &FemLoadCase, labels: &Fem2dLabels) -> String {
    let count = case.loads.len();
    let noun = if count == 1 { labels.load.as_str() } else { labels.loads.as_str() };
    let mut text = format!("{} · {count} {noun}", case.name);
    if case.self_weight {
        text.push_str(" · ");
        text.push_str(labels.self_weight.as_str());
    }
    text
}

pub fn fem2d_combination_label(combination: &FemCombination) -> String {
    let terms = combination.terms.iter().map(|term| format!("{} {}", fem2d_scalar(term.factor), term.case_id)).collect::<Vec<_>>().join(" + ");
    if terms.is_empty() {
        combination.name.clone()
    } else {
        format!("{} · {terms}", combination.name)
    }
}

/// 🧱️ `E` in gigapascals — the unit an engineer reads a stiffness in, never the raw 2.1e11 Pa. The
/// quantity symbols (`E`, `A`, `Iy`) are ISO notation, identical in every locale, so they are not
/// label rows: `labels.youngs_modulus` names the same quantity where a row has room to spell it.
pub fn fem2d_material_label(material: &FemMaterial) -> String {
    format!("{} · E {} GPa", material.name, fem2d_scalar(material.e / 1e9))
}

pub fn fem2d_section_label(section: &FemSection) -> String {
    format!("{} · A {} m² · Iy {} m⁴", section.name, fem2d_scalar(section.area), fem2d_scalar(section.iy))
}

pub fn fem2d_analysis_description(document: &Fem2dSnapshot, labels: &Fem2dLabels) -> String {
    format!(
        "{} {} · {} {} · {} {}",
        labels.modal_count.as_str(),
        document.analysis.modal_count,
        labels.buckling_count.as_str(),
        document.analysis.buckling_count,
        labels.deformation_scale.as_str(),
        fem2d_scalar(document.analysis.deformation_scale)
    )
}
//#endregion 🔖️Formatting

//#region 🔖️Rows
/// 🕹️ A tree-row pick in the `"fem2d"` domain — the very `interactionSelect` the Canvas2d windows
/// dispatch for a viewport pick, so a row click and a canvas click land in one selection.
fn pick_action(granularity: &str, id: &str) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    let target = protocol::InteractionTarget { granularity: granularity.into(), id: id.into() };
    let targets = protocol::json::to_json_string(&protocol::DslValue::Array(vec![protocol::ToValue::to_value(&target)]));
    let args = ui_value_map([("domainId", ui_value_text(FEM2D_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)])?;
    fem2d_action(INTERACTION_SELECT_ACTION_ID, Some(args))
}

fn row_action(icon: &str, label: &LabelText, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<RowAction> {
    let (action, args) = action;
    Ok(RowAction { icon: icon_text(icon)?, label: Some(ui_label(label.as_str())?), action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None }, placement: RowActionPlacement::Row })
}

/// 🌳️ One selectable entity row: keyed by the raw entity id, picking through the framework domain,
/// carrying at most the two row actions the UI contract admits — focus (geometric entities only)
/// and delete. A clipped label keeps a pathological name from refusing the whole render.
fn entity_row(id: &str, granularity: &str, label: &str, description: &LabelText, icon: &str, focusable: bool, dimmed: bool, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item_with_action(id, UiLabel(UiText::clipped(label)), None, pick_action(granularity, id)?)?;
    let Component::TreeItem(props) = &mut item.component else {
        return Err(capacity_error("fem2d artifact row is a tree item"));
    };
    props.icon = Some(icon_text(icon)?);
    props.description = Some(UiText::clipped(description.as_str()));
    props.dimmed = Some(dimmed);
    props.default_open = Some(false);
    let mut row_actions = UiFixedList::default();
    if focusable {
        let focus = fem2d_action(FOCUS_ENTITY_ACTION, Some(ui_value_map([("id", ui_value_text(id)?)])?))?;
        row_actions.try_push(row_action(FOCUS_ICON, &labels.focus, focus)?).map_err(|_| capacity_error("fem2d artifact focus row action admission failed"))?;
    }
    let remove = fem2d_action(REMOVE_SELECTION_ACTION, Some(ui_value_map([("ids", ui_value_list([ui_value_text(id)?])?)])?))?;
    row_actions.try_push(row_action(DELETE_ICON, &labels.delete, remove)?).map_err(|_| capacity_error("fem2d artifact delete row action admission failed"))?;
    props.row_actions = row_actions;
    Ok(item)
}

/// 🔗️ One combination term: read-only, composite-keyed — a term is not a domain target, only the
/// combination that owns it is.
fn term_row(combination: &FemCombination, index: usize, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    let term = &combination.terms[index];
    let id = format!("{TREE_NAMESPACE}.term.{}.{}", combination.id, term.case_id);
    tree_item_desc(id, UiLabel(UiText::clipped(&format!("{} {}", fem2d_scalar(term.factor), term.case_id))), Some(labels.term.as_str().to_string()))
}
//#endregion 🔖️Rows

//#region 🔖️Sections
fn section_label(noun: &LabelText, count: usize) -> UiAssemblyResult<UiLabel> {
    ui_label(format!("{} ({count})", noun.as_str()))
}

fn has_node(document: &Fem2dSnapshot, id: &str) -> bool {
    document.nodes.iter().any(|node| node.id == id)
}

fn element_is_dangling(document: &Fem2dSnapshot, element: &FemElement) -> bool {
    let (start, end, material_id, section_id) = match element {
        FemElement::Bar { start, end, material_id, section_id, .. } | FemElement::Beam { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
    };
    !has_node(document, start) || !has_node(document, end) || !document.materials.iter().any(|material| &material.id == material_id) || !document.sections.iter().any(|section| &section.id == section_id)
}

fn load_is_dangling(document: &Fem2dSnapshot, load: &FemLoad) -> bool {
    match load {
        FemLoad::Nodal { node_id, .. } => !has_node(document, node_id),
        FemLoad::MemberUdl { element_id: target, .. } => !document.elements.iter().any(|element| element_id(element) == target),
        FemLoad::Area { region_id, .. } => !document.regions.iter().any(|region| &region.id == region_id),
    }
}

fn nodes_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.nodes"), &document.nodes, rows, budget, |node, _| entity_row(&node.id, FEM2D_GRANULARITY_NODE, &fem2d_node_label(node), &labels.node, "circle-dot", true, false, labels))
}

fn elements_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.elements"), &document.elements, rows, budget, |element, _| {
        entity_row(element_id(element), FEM2D_GRANULARITY_ELEMENT, &fem2d_element_label(element, labels), &labels.element, "minus", true, element_is_dangling(document, element), labels)
    })
}

fn regions_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.regions"), &document.regions, rows, budget, |region, _| {
        let dimmed = !document.materials.iter().any(|material| material.id == region.material_id);
        entity_row(&region.id, FEM2D_GRANULARITY_REGION, &fem2d_region_label(region, labels), &labels.region, "square", true, dimmed, labels)
    })
}

fn supports_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.supports"), &document.supports, rows, budget, |support, _| {
        entity_row(&support.id, FEM2D_GRANULARITY_SUPPORT, &fem2d_support_label(support, labels), &labels.support, "anchor", true, !has_node(document, &support.node_id), labels)
    })
}

/// 🌳️ Load cases and their loads: the case row picks at `loadCase` granularity, each nested load row
/// at `load` granularity, and the nested page draws on the very budget its own section still holds.
fn load_cases_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let section_id = format!("{TREE_NAMESPACE}.load-cases");
    let rows = budget.remaining();
    paged_panel_section(&section_id, &document.load_cases, rows, budget, |case, nested| {
        let dimmed = case.loads.is_empty() && !case.self_weight;
        let mut item = entity_row(&case.id, FEM2D_GRANULARITY_LOAD_CASE, &fem2d_load_case_label(case, labels), &labels.load_case, "list", false, dimmed, labels)?;
        let load_rows = nested.remaining();
        let loads = paged_panel_section(&format!("{section_id}.{}", case.id), &case.loads, load_rows, nested, |load, _| {
            entity_row(load_id(load), FEM2D_GRANULARITY_LOAD, &fem2d_load_label(load, labels), &labels.load, "arrow-down", false, load_is_dangling(document, load), labels)
        })?;
        for load in loads {
            item.children.try_push(load).map_err(|_| capacity_error("fem2d artifact load row admission failed"))?;
        }
        if let Component::TreeItem(props) = &mut item.component {
            props.default_open = Some(true);
        }
        Ok(item)
    })
}

fn combinations_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.combinations"), &document.combinations, rows, budget, |combination, _| {
        let dimmed = combination.terms.iter().any(|term| !document.load_cases.iter().any(|case| case.id == term.case_id));
        let mut item = entity_row(&combination.id, FEM2D_GRANULARITY_COMBINATION, &fem2d_combination_label(combination), &labels.combination, "link", false, dimmed, labels)?;
        for index in 0..combination.terms.len() {
            item.children.try_push(term_row(combination, index, labels)?).map_err(|_| capacity_error("fem2d artifact term row admission failed"))?;
        }
        Ok(item)
    })
}

fn materials_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.materials"), &document.materials, rows, budget, |material, _| {
        entity_row(&material.id, FEM2D_GRANULARITY_MATERIAL, &fem2d_material_label(material), &labels.material, "layers", false, false, labels)
    })
}

fn sections_section(document: &Fem2dSnapshot, labels: &Fem2dLabels, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.sections"), &document.sections, rows, budget, |section, _| {
        entity_row(&section.id, FEM2D_GRANULARITY_SECTION, &fem2d_section_label(section), &labels.section, "ruler", false, false, labels)
    })
}

/// ⚙️ The analysis settings row — read-only and free: it binds no action, so it costs nothing of the
/// argument arena the entity rows page against.
fn analysis_section(document: &Fem2dSnapshot, labels: &Fem2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    ui_node_list([tree_item_desc(format!("{TREE_NAMESPACE}.analysis.settings"), ui_label(labels.analysis.as_str())?, Some(fem2d_analysis_description(document, labels)))])
}
//#endregion 🔖️Sections

//#region 🔖️Render
fn marked_ids(ids: &[String]) -> Vec<String> {
    ids.iter().take(MARKED_IDS_LIMIT).cloned().collect()
}

/// 🧾️ Interactive rows each section wants: its own entities, plus — for load cases — the load rows
/// nested under them, which draw on the same page.
pub fn section_demands(document: &Fem2dSnapshot) -> [usize; SECTIONS] {
    [
        document.nodes.len(),
        document.elements.len(),
        document.regions.len(),
        document.supports.len(),
        document.load_cases.len() + document.load_cases.iter().map(|case| case.loads.len()).sum::<usize>(),
        document.combinations.len(),
        document.materials.len(),
        document.sections.len(),
        0,
    ]
}

/// 🪙️ Max-min fair split of one interactive-row page across the sections: the quota is the highest
/// per-section ceiling the page affords, every section under it gets ALL its rows, and the spare
/// goes to the widest sections. Reserving "the rows the sections after me still need" verbatim (cad's
/// four symmetric panes) would let a 12-node document's first section eat two thirds of the page and
/// leave the load cases one row; this keeps the small sections whole and truncates only the wide
/// ones, which is also what a reader of an outliner expects.
pub fn section_quotas(demands: [usize; SECTIONS], page: usize) -> [usize; SECTIONS] {
    let mut ceiling = 0;
    while ceiling < page && demands.iter().map(|demand| (*demand).min(ceiling + 1)).sum::<usize>() <= page {
        ceiling += 1;
    }
    let mut quotas = demands.map(|demand| demand.min(ceiling));
    let mut spare = page.saturating_sub(quotas.iter().sum::<usize>());
    while spare > 0 {
        let Some(index) = (0..SECTIONS).filter(|index| quotas[*index] < demands[*index]).max_by_key(|index| demands[*index] - quotas[*index]) else {
            break;
        };
        quotas[index] += 1;
        spare -= 1;
    }
    quotas
}

/// 🪙️ Runs one section against its quota, reserving the whole rest of the page for its siblings —
/// and settling whatever it under-spends straight back into the shared budget.
fn with_quota<R>(budget: &mut PanelRowBudget, quota: usize, build: impl FnOnce(&mut PanelRowBudget) -> R) -> R {
    let reserved = budget.remaining().saturating_sub(quota);
    budget.nested(reserved, build)
}

/// 🌳️ The outliner: nine sections over one interactive-row page, bound to the `"fem2d"` domain and
/// marked from the live interaction snapshot.
pub fn build_artifact_tree(document: &Fem2dSnapshot, interaction: &Fem2dInteractionSnapshot, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    let page = panel_page_rows();
    let quotas = section_quotas(section_demands(document), page);
    let budget = &mut PanelRowBudget::new(page);
    let nodes = with_quota(budget, quotas[0], |share| nodes_section(document, labels, share))?;
    let elements = with_quota(budget, quotas[1], |share| elements_section(document, labels, share))?;
    let regions = with_quota(budget, quotas[2], |share| regions_section(document, labels, share))?;
    let supports = with_quota(budget, quotas[3], |share| supports_section(document, labels, share))?;
    let load_cases = with_quota(budget, quotas[4], |share| load_cases_section(document, labels, share))?;
    let combinations = with_quota(budget, quotas[5], |share| combinations_section(document, labels, share))?;
    let materials = with_quota(budget, quotas[6], |share| materials_section(document, labels, share))?;
    let sections = with_quota(budget, quotas[7], |share| sections_section(document, labels, share))?;
    let analysis = analysis_section(document, labels)?;
    let placeholder = ui_label(labels.none.as_str())?;

    let builder = PanelTreeBuilder::new(TREE_NAMESPACE)?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.nodes"), Some(section_label(&labels.nodes, document.nodes.len())?), true, nodes, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.elements"), Some(section_label(&labels.elements, document.elements.len())?), true, elements, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.regions"), Some(section_label(&labels.regions, document.regions.len())?), true, regions, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.supports"), Some(section_label(&labels.supports, document.supports.len())?), true, supports, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.load-cases"), Some(section_label(&labels.load_cases, document.load_cases.len())?), true, load_cases, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.combinations"), Some(section_label(&labels.combinations, document.combinations.len())?), true, combinations, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.materials"), Some(section_label(&labels.materials, document.materials.len())?), false, materials, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.sections"), Some(section_label(&labels.sections, document.sections.len())?), false, sections, placeholder)?
        .section(format!("{TREE_NAMESPACE}.analysis"), Some(ui_label(labels.analysis.as_str())?), false, analysis)?
        .interaction_domain(FEM2D_INTERACTION_DOMAIN)?
        .selected(marked_ids(&interaction.selected_ids))?
        .highlighted(marked_ids(&interaction.hovered_ids))?;
    builder.build()
}

pub fn render(doc: &Fem2dSnapshot, interaction: &Fem2dInteractionSnapshot, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    build_artifact_tree(doc, interaction, labels)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
