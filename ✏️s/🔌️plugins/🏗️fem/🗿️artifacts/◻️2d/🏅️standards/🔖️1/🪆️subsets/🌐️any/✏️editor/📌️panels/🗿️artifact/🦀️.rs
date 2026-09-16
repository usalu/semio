//! 📄️ Fem2d play app panel — the artifact tree (outliner): every document entity as one row, in the
//! order an engineer builds a model (geometry, then restraints, then actions, then the properties
//! they reference, then the analysis settings).
//!
//! 🕹️ The tree is bound to the framework-owned `"fem2d"` interaction domain and every row's key is
//! the RAW entity id, so a row click, a viewport pick and the inspector address the one selection.
//! Picking is declared, not bound: the tree carries ONE `interactionSelect` and each row names the
//! granularity it picks at, so a wide section costs no argument arena at all.
//! 🪟️ Every section and every nesting row is a WINDOW over its own list: the host says which slice it
//! is looking at (`ViewModel::tree_windows`), the guest materialises exactly that slice and stamps the
//! full `total` beside it. A document past one viewport streams; it never truncates and it never
//! closes a section with a `+N` row (see `semio_framework_plugin::TreeWindows`).

use crate::editor::fem2d::interaction::{
    Fem2dInteractionSnapshot, FEM2D_GRANULARITY_COMBINATION, FEM2D_GRANULARITY_ELEMENT, FEM2D_GRANULARITY_LOAD, FEM2D_GRANULARITY_LOAD_CASE, FEM2D_GRANULARITY_MATERIAL, FEM2D_GRANULARITY_NODE, FEM2D_GRANULARITY_REGION, FEM2D_GRANULARITY_SECTION,
    FEM2D_GRANULARITY_SUPPORT, FEM2D_INTERACTION_DOMAIN,
};
use crate::editor::fem2d::terminology::Fem2dLabels;
use crate::editor::fem2d::{ui_label, FEM2D_PLAY_CONTROLLER_ID};
use crate::{element_id, load_id, FemCombination, FemDof, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport, Fem2dSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, Label as UiLabel};
use semio_framework_plugin::{
    tree_item_desc, tree_window_item, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

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

fn icon_text(icon: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(icon).ok_or_else(|| capacity_error("fem2d artifact icon admission failed"))
}

fn granularity_text(granularity: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(granularity).ok_or_else(|| capacity_error("fem2d artifact granularity admission failed"))
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
/// 🌳️ One selectable entity row, keyed by the raw entity id and declared a pick target of the tree's
/// own `"fem2d"` domain at `granularity` — the very selection the Canvas2d windows dispatch for a
/// viewport pick, so a row click and a canvas click land in one selection. It binds NOTHING and
/// carries no argument map: a page of rows authoring a pick map each is exactly what exhausts the
/// one-page `UiValue` argument arena and starves every panel rendered beside the tree (the cad
/// incident, and this ticket's first browser probe), and focus and delete live in the inspector for
/// the same reason. A clipped label keeps a pathological name from refusing the whole render.
fn entity_item(id: &str, granularity: &str, label: &str, description: &LabelText, icon: &str, dimmed: bool) -> UiAssemblyResult<ui::TreeItemBuilder> {
    let item = ui::tree_item(UiLabel(UiText::clipped(label))).try_id(id).map_err(|_| capacity_error("fem2d artifact row id admission failed"))?;
    Ok(item.icon(icon_text(icon)?).description(UiText::clipped(description.as_str())).dimmed(dimmed).granularity(granularity_text(granularity)?))
}

fn entity_row(id: &str, granularity: &str, label: &str, description: &LabelText, icon: &str, dimmed: bool) -> UiAssemblyResult<BuiltNode> {
    entity_item(id, granularity, label, description, icon, dimmed)?.default_open(false).try_build().map_err(|_| capacity_error("fem2d artifact row admission failed"))
}

/// 🔗️ One combination term: read-only, composite-keyed — a term is not a domain target, only the
/// combination that owns it is.
fn term_row(combination: &FemCombination, term: &crate::FemCombinationTerm, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{TREE_NAMESPACE}.term.{}.{}", combination.id, term.case_id);
    tree_item_desc(id, UiLabel(UiText::clipped(&format!("{} {}", fem2d_scalar(term.factor), term.case_id))), Some(labels.term.as_str().to_string()))
}
//#endregion 🔖️Rows

//#region 🔖️Predicates
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
//#endregion 🔖️Predicates

//#region 🔖️Render
fn marked_ids(ids: &[String]) -> Vec<String> {
    ids.iter().take(MARKED_IDS_LIMIT).cloned().collect()
}

/// 🌳️ The outliner: nine windowed sections bound to the `"fem2d"` domain and marked from the live
/// interaction snapshot. Each section (and each load case / combination row that nests children)
/// materialises only the slice the host is looking at and stamps its own `total`.
pub fn build_artifact_tree(document: &Fem2dSnapshot, interaction: &Fem2dInteractionSnapshot, labels: &Fem2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let placeholder = ui_label(labels.none.as_str())?;
    let analysis = [fem2d_analysis_description(document, labels)];
    PanelTreeBuilder::new(TREE_NAMESPACE)?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.nodes"),
            Some(section_label(&labels.nodes, document.nodes.len())?),
            true,
            &document.nodes,
            |node| entity_row(&node.id, FEM2D_GRANULARITY_NODE, &fem2d_node_label(node), &labels.node, "circle-dot", false),
            placeholder.clone(),
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.elements"),
            Some(section_label(&labels.elements, document.elements.len())?),
            true,
            &document.elements,
            |element| entity_row(element_id(element), FEM2D_GRANULARITY_ELEMENT, &fem2d_element_label(element, labels), &labels.element, "minus", element_is_dangling(document, element)),
            placeholder.clone(),
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.regions"),
            Some(section_label(&labels.regions, document.regions.len())?),
            true,
            &document.regions,
            |region| {
                let dimmed = !document.materials.iter().any(|material| material.id == region.material_id);
                entity_row(&region.id, FEM2D_GRANULARITY_REGION, &fem2d_region_label(region, labels), &labels.region, "square", dimmed)
            },
            placeholder.clone(),
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.supports"),
            Some(section_label(&labels.supports, document.supports.len())?),
            true,
            &document.supports,
            |support| entity_row(&support.id, FEM2D_GRANULARITY_SUPPORT, &fem2d_support_label(support, labels), &labels.support, "anchor", !has_node(document, &support.node_id)),
            placeholder.clone(),
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.load-cases"),
            Some(section_label(&labels.load_cases, document.load_cases.len())?),
            true,
            &document.load_cases,
            |case| {
                let dimmed = case.loads.is_empty() && !case.self_weight;
                let item = entity_item(&case.id, FEM2D_GRANULARITY_LOAD_CASE, &fem2d_load_case_label(case, labels), &labels.load_case, "list", dimmed)?;
                tree_window_item(windows, item, &case.id, true, &case.loads, |load| {
                    entity_row(load_id(load), FEM2D_GRANULARITY_LOAD, &fem2d_load_label(load, labels), &labels.load, "arrow-down", load_is_dangling(document, load))
                })
            },
            placeholder.clone(),
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.combinations"),
            Some(section_label(&labels.combinations, document.combinations.len())?),
            true,
            &document.combinations,
            |combination| {
                let dimmed = combination.terms.iter().any(|term| !document.load_cases.iter().any(|case| case.id == term.case_id));
                let item = entity_item(&combination.id, FEM2D_GRANULARITY_COMBINATION, &fem2d_combination_label(combination), &labels.combination, "link", dimmed)?;
                tree_window_item(windows, item, &combination.id, true, &combination.terms, |term| term_row(combination, term, labels))
            },
            placeholder.clone(),
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.materials"),
            Some(section_label(&labels.materials, document.materials.len())?),
            false,
            &document.materials,
            |material| entity_row(&material.id, FEM2D_GRANULARITY_MATERIAL, &fem2d_material_label(material), &labels.material, "layers", false),
            placeholder.clone(),
        )?
        .window_section_or_placeholder(
            windows,
            &format!("{TREE_NAMESPACE}.sections"),
            Some(section_label(&labels.sections, document.sections.len())?),
            false,
            &document.sections,
            |section| entity_row(&section.id, FEM2D_GRANULARITY_SECTION, &fem2d_section_label(section), &labels.section, "ruler", false),
            placeholder,
        )?
        .window_section(windows, &format!("{TREE_NAMESPACE}.analysis"), Some(ui_label(labels.analysis.as_str())?), false, &analysis, |description| {
            tree_item_desc(format!("{TREE_NAMESPACE}.analysis.settings"), ui_label(labels.analysis.as_str())?, Some(description.clone()))
        })?
        .interaction_domain(FEM2D_PLAY_CONTROLLER_ID, FEM2D_INTERACTION_DOMAIN)?
        .selected(marked_ids(&interaction.selected_ids))?
        .highlighted(marked_ids(&interaction.hovered_ids))?
        .build()
}

pub fn render(doc: &Fem2dSnapshot, interaction: &Fem2dInteractionSnapshot, labels: &Fem2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    build_artifact_tree(doc, interaction, labels, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
