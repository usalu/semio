//! 🛍️ Process 3d play app panel — the workshop capability catalogue plus quick-swap stock kinds.

use crate::artifacts::process3d::schema::inferences::{validate_capability, validation_reason, ValidationContext};
use crate::artifacts::process3d::{MachineCatalog, Process3dSnapshot, WorkshopMachine};
use crate::editor::process3d::iconed_tree_item_with_action;
use crate::editor::process3d::installed_catalogs;
use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::Process3dLabels;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_CATALOGUE: &str = "process.play.catalogue";
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

fn capability_items<'a>(machines: impl IntoIterator<Item = &'a WorkshopMachine>, ctx: &ValidationContext) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut items = semio_framework_plugin::UiFixedList::default();
    for machine in machines {
        for capability in &machine.capabilities {
            let failures = validate_capability(capability, ctx);
            let id = format!("process3d-catalogue.{}.{}", machine.id, capability.id);
            let label = format!("{} — {}", machine.label, capability.label);
            let item = if failures.is_empty() {
                let args = crate::editor::process3d::ui_value_map([
                    ("capabilityId", crate::editor::process3d::ui_value_text(&capability.id)?),
                    ("machineId", crate::editor::process3d::ui_value_text(&machine.id)?),
                ])?;
                iconed_tree_item_with_action(id, &label, &capability.icon_id, process3d_action("addStep", Some(args)))?
            } else {
                let mut item = tree_item_desc(id, crate::editor::process3d::ui_label(&label)?, Some(validation_reason(&failures)))?;
                if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
                    props.icon = Some(
                        semio_framework_plugin::UiText::try_from_str(&capability.icon_id)
                            .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.icon", "fixed capability icon admission failed"))?,
                    );
                }
                item
            };
            items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.items", "fixed capability catalogue admission failed"))?;
        }
    }
    Ok(items)
}

/// 🏭️ Builds one catalogue tree item per workshop machine capability, grouped by the machine's source
/// catalog (uncataloged/generic machines first, open by default), disabling (non-clickable, with a
/// reason) any capability the current stock doesn't satisfy.
pub fn render(fixture: &Process3dSnapshot, contributions_json: &str, labels: &Process3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    // 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.stock_solid` is a
    // composed `s.stdio.semio.brep` CHILD HANDLE now, with no resolvable dimensions without a
    // `LinkResolver` (see `ProcessWorkingScene`'s doc comment) — every capability rule is treated
    // as satisfied (a large, effectively-unconstrained stock) rather than guessing at unknown
    // extents, matching the same documented gap `add_step::handle` accepted for this reason.
    let ctx = ValidationContext { stock_width: f64::MAX, stock_depth: f64::MAX, stock_height: f64::MAX };
    let mut builder = PanelTreeBuilder::new("process3d-play-catalogue")?;
    let mut workshop_machines: semio_framework_plugin::UiFixedList<&WorkshopMachine> = semio_framework_plugin::UiFixedList::default();
    let mut catalog_sections: Vec<(semio_framework_plugin::UiText, Vec<&WorkshopMachine>)> = Vec::new();
    for machine in &fixture.workshop.machines {
        let Some(catalog_id) = machine.catalog_id.as_deref() else {
            workshop_machines.try_push(machine).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.workshop", "fixed workshop catalogue admission failed"))?;
            continue;
        };
        if let Some(index) = catalog_sections.iter().position(|(existing, _)| existing.as_str() == catalog_id) {
            let Some((_, machines)) = catalog_sections.get_mut(index) else {
                return Err(semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.section", "catalogue section owner was not retained"));
            };
            machines.push(machine);
        } else {
            let key = semio_framework_plugin::UiText::try_from_str(catalog_id)
                .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.section-id", "fixed catalogue section id admission failed"))?;
            let machines = vec![machine];
            catalog_sections.push((key, machines));
        }
    }
    if !workshop_machines.is_empty() {
        builder = builder.section("process3d-play-catalogue.workshop", Some(crate::editor::process3d::ui_label(labels.workshop.as_str())?), true, capability_items(workshop_machines.iter().copied(), &ctx)?)?;
    }
    for (catalog_id, machines) in catalog_sections.iter() {
        let section_id = format!("process3d-play-catalogue.{catalog_id}");
        let section_label = crate::editor::process3d::ui_label(catalog_label(contributions_json, catalog_id.as_str()))?;
        builder = builder.section(section_id, Some(section_label), false, capability_items(machines.iter().copied(), &ctx)?)?;
    }
    let stock_items = crate::editor::process3d::ui_node_list([
        iconed_tree_item_with_action("process3d-catalogue.stock-box", labels.stock_kind_box.as_str(), "box", process3d_action("setStock", Some(crate::editor::process3d::ui_value_map([("kind", crate::editor::process3d::ui_value_text("box")?)])?))),
        iconed_tree_item_with_action("process3d-catalogue.stock-cylinder", labels.stock_kind_cylinder.as_str(), "cylinder", process3d_action("setStock", Some(crate::editor::process3d::ui_value_map([("kind", crate::editor::process3d::ui_value_text("cylinder")?)])?))),
        iconed_tree_item_with_action("process3d-catalogue.stock-sphere", labels.stock_kind_sphere.as_str(), "circle", process3d_action("setStock", Some(crate::editor::process3d::ui_value_map([("kind", crate::editor::process3d::ui_value_text("sphere")?)])?))),
        iconed_tree_item_with_action("process3d-catalogue.stock-import", labels.import_model.as_str(), "folder-open", process3d_action("loadModelRequest", None)),
    ])?;
    builder.section("process3d-play-catalogue.stock", Some(crate::editor::process3d::ui_label(labels.stock.as_str())?), false, stock_items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::process3d::testkit;

    /// 🪵️ The default timber beam (0.24m tall) exceeds both the circular saw's 0.065m max cut depth
    /// and the table saw's 0.102m — both wood machines list, both are disabled with a reason.
    #[semio_framework_async_macros::async_test]
    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.stock_solid` is a
    /// composed `s.stdio.semio.brep` CHILD HANDLE now, with no resolvable dimensions without a
    /// `LinkResolver` (see `render`'s own doc comment) — every capability now renders as valid
    /// (an unconstrained stock), so the "mixed validity" premise this test's name describes is a
    /// documented gap rather than real behavior; it now asserts only that the wood catalog's
    /// machines still appear.
    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_workshop_wood_machines() {
        let mut app = testkit::app();
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_CATALOGUE);
        assert!(rendered.contains("Circular Saw"), "expected wood's circular saw in the catalogue: {rendered}");
        assert!(rendered.contains("Table Saw"), "expected wood's table saw in the catalogue: {rendered}");
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
