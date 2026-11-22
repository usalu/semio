//! 🛍️ Process 3d play app panel — the workshop capability catalogue plus quick-swap stock kinds.

use semio_s_artifact_process_process3d_kit::iconed_tree_item_with_action;
use semio_s_artifact_process_process3d_kit::installed_catalogs;
use semio_s_artifact_process_process3d_kit::process3d_action;
use semio_plugin::{row_action, RowActionPlacement};
use semio_s_artifact_process_process3d_kit::editor::process3d::terminology::Process3dLabels;
use semio_s_artifact_process_process3d_inferences::{validate_capability, validation_context_for_stock, validation_reason, ValidationContext};
use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::MachineCatalog, snapshot::Process3dSnapshot, snapshot::WorkshopMachine};
use semio_plugin::tree_item_desc;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PanelGroup;
use semio_plugin::PanelTabDefinition;
use semio_plugin::PanelTabKind;
use semio_plugin::PanelTreeBuilder;
use semio_plugin::TreeWindows;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;

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

/// 🔧 Available capability rows add on label click and offer a separate positioned placement action.
fn capability_row(machine: &WorkshopMachine, capability: &Capability, ctx: Option<&ValidationContext>, labels: &Process3dLabels) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    let id = format!("process3d-catalogue.{}.{}", machine.id, capability.id);
    let label = format!("{} — {}", machine.label, capability.label);
    let reason = if let Some(ctx) = ctx {
        let failures = validate_capability(capability, ctx);
        if failures.is_empty() {
            let args = semio_s_artifact_process_process3d_kit::ui_value_map([("capabilityId", semio_s_artifact_process_process3d_kit::ui_value_text(&capability.id)?), ("machineId", semio_s_artifact_process_process3d_kit::ui_value_text(&machine.id)?)])?;
            let mut item = iconed_tree_item_with_action(id, &label, &capability.icon_id, process3d_action("addStep", Some(args)))?;
            if let semio_plugin::Component::TreeItem(props) = &mut item.component {
                props.row_actions.try_push(row_action("crosshair", labels.place_capability.as_str(), "placeCapability", RowActionPlacement::Row)?)
                    .map_err(|_| semio_plugin::PluginAssemblyError::new("ui.catalogue.row-actions", "capability placement action admission failed"))?;
            }
            return Ok(item);
        }
        validation_reason(&failures)
    } else if labels.is_de() {
        "Geschlossenen Körper mit gültigen Abmessungen laden.".to_string()
    } else {
        "Load a closed solid with valid dimensions.".to_string()
    };
    let mut item = tree_item_desc(id, semio_s_artifact_process_process3d_kit::ui_label(&label)?, Some(reason))?;
    if let semio_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(semio_plugin::UiText::try_from_str(&capability.icon_id).ok_or_else(|| semio_plugin::PluginAssemblyError::new("ui.catalogue.icon", "fixed capability icon admission failed"))?);
    }
    Ok(item)
}

/// 🏭️ Groups capability rows by catalog and shows why unavailable operations cannot be added.
pub fn render(snapshot: &Process3dSnapshot, contributions_json: &str, labels: &Process3dLabels, windows: &TreeWindows<'_>) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    let ctx = validation_context_for_stock(&snapshot.stock_payload);
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
        builder = builder.window_section(windows, PROCESS_3D_PLAY_CATALOGUE_WORKSHOP, Some(semio_s_artifact_process_process3d_kit::ui_label(labels.workshop.as_str())?), true, &entries, |(machine, capability)| {
            capability_row(machine, capability, ctx.as_ref(), labels)
        })?;
    }
    for (catalog_id, machines) in catalog_sections.iter() {
        let section_id = format!("process3d-play-catalogue.{catalog_id}");
        let section_label = semio_s_artifact_process_process3d_kit::ui_label(catalog_label(contributions_json, catalog_id))?;
        let entries = capability_entries(machines.iter().copied());
        builder = builder.window_section(windows, &section_id, Some(section_label), false, &entries, |(machine, capability)| capability_row(machine, capability, ctx.as_ref(), labels))?;
    }
    let stock_kinds: [(&str, &str, &str, Option<&str>); 4] = [
        ("process3d-catalogue.stock-box", labels.stock_kind_box.as_str(), "box", Some("box")),
        ("process3d-catalogue.stock-cylinder", labels.stock_kind_cylinder.as_str(), "cylinder", Some("cylinder")),
        ("process3d-catalogue.stock-sphere", labels.stock_kind_sphere.as_str(), "circle", Some("sphere")),
        ("process3d-catalogue.stock-import", labels.import_model.as_str(), "folder-open", None),
    ];
    builder
        .window_section(windows, PROCESS_3D_PLAY_CATALOGUE_STOCK, Some(semio_s_artifact_process_process3d_kit::ui_label(labels.stock.as_str())?), false, &stock_kinds, |(id, label, icon, kind)| {
            let action = match kind {
                Some(kind) => process3d_action("setStock", Some(semio_s_artifact_process_process3d_kit::ui_value_map([("kind", semio_s_artifact_process_process3d_kit::ui_value_text(kind)?)])?)),
                None => process3d_action("loadModelRequest", None),
            };
            iconed_tree_item_with_action(id, label, icon, action)
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
//#endregion 🧪️Tests
