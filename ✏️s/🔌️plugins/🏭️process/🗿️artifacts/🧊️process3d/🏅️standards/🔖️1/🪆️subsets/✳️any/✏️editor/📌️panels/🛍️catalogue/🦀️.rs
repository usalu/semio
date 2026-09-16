//! 🛍️ Process 3d play app panel — the workshop capability catalogue plus quick-swap stock kinds.

use crate::editor::process3d::iconed_tree_item_with_action;
use crate::editor::process3d::installed_catalogs;
use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::Process3dLabels;
use crate::schema::inferences::{validate_capability, validation_reason, ValidationContext};
use crate::{Capability, MachineCatalog, Process3dSnapshot, WorkingSolid, WorkshopMachine};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_CATALOGUE: &str = "process.play.catalogue";
pub const PROCESS_3D_PLAY_CATALOGUE_WORKSHOP: &str = "process3d-play-catalogue.workshop";
pub const PROCESS_3D_PLAY_CATALOGUE_STOCK: &str = "process3d-play-catalogue.stock";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCESS_3D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🏷️ Display label for a catalog id, resolved against `installed_catalogs()` — falls back to the raw
/// id if the catalog that seeded a workshop machine was since uninstalled (never resolved back, per
/// `WorkshopMachine::catalog_id`'s informational-only contract).
fn catalog_label(contributions_json: &str, catalog_id: &str) -> String {
    installed_catalogs(contributions_json).into_iter().find(|catalog| catalog.catalog_id() == catalog_id).map_or_else(|| catalog_id.to_string(), |catalog| catalog.label().to_string())
}

/// 🏭️ Every `(machine, capability)` pair a section lists, flattened once so the section's window can
/// slice it — a window is over ENTRIES, and a capability is the entry a reader clicks.
fn capability_entries<'a>(machines: impl IntoIterator<Item = &'a WorkshopMachine>) -> Vec<(&'a WorkshopMachine, &'a Capability)> {
    machines.into_iter().flat_map(|machine| machine.capabilities.iter().map(move |capability| (machine, capability))).collect()
}

/// 🔧 One capability row: an `addStep` binding when the current stock satisfies the capability's
/// rules, otherwise a non-clickable row carrying the failure as its reason.
fn capability_row(machine: &WorkshopMachine, capability: &Capability, ctx: &ValidationContext) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let failures = validate_capability(capability, ctx);
    let id = format!("process3d-catalogue.{}.{}", machine.id, capability.id);
    let label = format!("{} — {}", machine.label, capability.label);
    if failures.is_empty() {
        let args = crate::editor::process3d::ui_value_map([("capabilityId", crate::editor::process3d::ui_value_text(&capability.id)?), ("machineId", crate::editor::process3d::ui_value_text(&machine.id)?)])?;
        return iconed_tree_item_with_action(id, &label, &capability.icon_id, process3d_action("addStep", Some(args)));
    }
    let mut item = tree_item_desc(id, crate::editor::process3d::ui_label(&label)?, Some(validation_reason(&failures)))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(&capability.icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.icon", "fixed capability icon admission failed"))?);
    }
    Ok(item)
}

/// 📐️ Real per-variant stock dimensions for capability-rule validation, derived from
/// `snapshot.stock_payload.solid` — the snapshot's own inline, authoritative record since ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4 (`stock_solid` stays a composed-child HANDLE
/// with no resolvable content). `Box`/`Cylinder`/`Sphere` are analytic: a cylinder's/sphere's
/// width/depth are their diameter; a `Reference` reads its shipped, kernel-pinned extent
/// (`stock_extent`). `ImportedMesh`/`ImportedSolid` carry no persisted analytic bounding box, so
/// every dimension stays unconstrained (`f64::MAX`) exactly as the whole stock used to be before this
/// fix — never a guessed extent that could falsely fail a rule.
fn stock_validation_context(solid: &WorkingSolid) -> ValidationContext {
    match solid {
        WorkingSolid::Box { width, depth, height } => ValidationContext { stock_width: *width, stock_depth: *depth, stock_height: *height },
        WorkingSolid::Cylinder { radius, height } => ValidationContext { stock_width: radius * 2.0, stock_depth: radius * 2.0, stock_height: *height },
        WorkingSolid::Sphere { radius } => ValidationContext { stock_width: radius * 2.0, stock_depth: radius * 2.0, stock_height: radius * 2.0 },
        WorkingSolid::Reference { .. } => {
            let [width, depth, height] = crate::schema::inferences::stock_extent(solid);
            ValidationContext { stock_width: width, stock_depth: depth, stock_height: height }
        }
        WorkingSolid::ImportedMesh { .. } | WorkingSolid::ImportedSolid { .. } => ValidationContext { stock_width: f64::MAX, stock_depth: f64::MAX, stock_height: f64::MAX },
    }
}

/// 🏭️ Builds one catalogue tree item per workshop machine capability, grouped by the machine's source
/// catalog (uncataloged/generic machines first, open by default), disabling (non-clickable, with a
/// reason) any capability the current stock doesn't satisfy — real dimensions via
/// `stock_validation_context`.
pub fn render(snapshot: &Process3dSnapshot, contributions_json: &str, labels: &Process3dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let ctx = stock_validation_context(&snapshot.stock_payload.solid);
    let mut builder = PanelTreeBuilder::new("process3d-play-catalogue")?;
    let mut workshop_machines: Vec<&WorkshopMachine> = Vec::new();
    let mut catalog_sections: Vec<(&str, Vec<&WorkshopMachine>)> = Vec::new();
    for machine in &snapshot.workshop.machines {
        let Some(catalog_id) = machine.catalog_id.as_deref() else {
            workshop_machines.push(machine);
            continue;
        };
        match catalog_sections.iter_mut().find(|(existing, _)| *existing == catalog_id) {
            Some((_, machines)) => machines.push(machine),
            None => catalog_sections.push((catalog_id, vec![machine])),
        }
    }
    if !workshop_machines.is_empty() {
        let entries = capability_entries(workshop_machines.iter().copied());
        builder = builder.window_section(windows, PROCESS_3D_PLAY_CATALOGUE_WORKSHOP, Some(crate::editor::process3d::ui_label(labels.workshop.as_str())?), true, &entries, |(machine, capability)| {
            capability_row(machine, capability, &ctx)
        })?;
    }
    for (catalog_id, machines) in catalog_sections.iter() {
        let section_id = format!("process3d-play-catalogue.{catalog_id}");
        let section_label = crate::editor::process3d::ui_label(catalog_label(contributions_json, catalog_id))?;
        let entries = capability_entries(machines.iter().copied());
        builder = builder.window_section(windows, &section_id, Some(section_label), false, &entries, |(machine, capability)| capability_row(machine, capability, &ctx))?;
    }
    let stock_kinds: [(&str, &str, &str, Option<&str>); 4] = [
        ("process3d-catalogue.stock-box", labels.stock_kind_box.as_str(), "box", Some("box")),
        ("process3d-catalogue.stock-cylinder", labels.stock_kind_cylinder.as_str(), "cylinder", Some("cylinder")),
        ("process3d-catalogue.stock-sphere", labels.stock_kind_sphere.as_str(), "circle", Some("sphere")),
        ("process3d-catalogue.stock-import", labels.import_model.as_str(), "folder-open", None),
    ];
    builder
        .window_section(windows, PROCESS_3D_PLAY_CATALOGUE_STOCK, Some(crate::editor::process3d::ui_label(labels.stock.as_str())?), false, &stock_kinds, |(id, label, icon, kind)| {
            let action = match kind {
                Some(kind) => process3d_action("setStock", Some(crate::editor::process3d::ui_value_map([("kind", crate::editor::process3d::ui_value_text(kind)?)])?)),
                None => process3d_action("loadModelRequest", None),
            };
            iconed_tree_item_with_action(id, label, icon, action)
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
