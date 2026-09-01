//! 🛍️ Process 3d play app panel — the workshop capability catalogue plus quick-swap stock kinds.

use crate::artifacts::process3d::schema::inferences::{validate_capability, validation_reason, ValidationContext};
use crate::artifacts::process3d::{MachineCatalog, Process3dSnapshot, WorkingSolid, WorkshopMachine};
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

/// 📐️ Real per-variant stock dimensions for capability-rule validation, derived from
/// `fixture.stock_payload.solid` — the snapshot's own inline, authoritative record since ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4 (`stock_solid` stays a composed-child HANDLE
/// with no resolvable content). `Box`/`Cylinder`/`Sphere` are analytic: a cylinder's/sphere's
/// width/depth are their diameter. `ImportedMesh`/`ImportedSolid` carry no persisted analytic
/// bounding box, so every dimension stays unconstrained (`f64::MAX`) exactly as the whole stock used
/// to be before this fix — never a guessed extent that could falsely fail a rule.
fn stock_validation_context(solid: &WorkingSolid) -> ValidationContext {
    match solid {
        WorkingSolid::Box { width, depth, height } => ValidationContext { stock_width: *width, stock_depth: *depth, stock_height: *height },
        WorkingSolid::Cylinder { radius, height } => ValidationContext { stock_width: radius * 2.0, stock_depth: radius * 2.0, stock_height: *height },
        WorkingSolid::Sphere { radius } => ValidationContext { stock_width: radius * 2.0, stock_depth: radius * 2.0, stock_height: radius * 2.0 },
        WorkingSolid::ImportedMesh { .. } | WorkingSolid::ImportedSolid { .. } => ValidationContext { stock_width: f64::MAX, stock_depth: f64::MAX, stock_height: f64::MAX },
    }
}

/// 🏭️ Builds one catalogue tree item per workshop machine capability, grouped by the machine's source
/// catalog (uncataloged/generic machines first, open by default), disabling (non-clickable, with a
/// reason) any capability the current stock doesn't satisfy — real dimensions via
/// `stock_validation_context`.
pub fn render(fixture: &Process3dSnapshot, contributions_json: &str, labels: &Process3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let ctx = stock_validation_context(&fixture.stock_payload.solid);
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

    /// 🪵️ Both wood machines' capabilities render as tree items (labels present) whether the current
    /// stock satisfies their rules or not — `stock_validation_context` now checks real dimensions, so
    /// this only asserts presence; the pass/fail split itself is covered by
    /// `catalogue_flags_a_violated_max_rule_and_not_a_satisfied_one` below with a fixture this test
    /// doesn't have to depend on the shared example document's own stock for.
    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_workshop_wood_machines() {
        let mut app = testkit::app();
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_CATALOGUE);
        assert!(rendered.contains("Circular Saw"), "expected wood's circular saw in the catalogue: {rendered}");
        assert!(rendered.contains("Table Saw"), "expected wood's table saw in the catalogue: {rendered}");
    }

    /// 🪚️ Real per-variant dimensions (`stock_validation_context`) must flag a capability whose `Max`
    /// rule a `0.2m`-tall stock violates (`maxCutDepth = 0.1`) and leave one it satisfies
    /// (`maxCutDepth = 0.5`) alone: the violated item renders `validation_reason`'s "needs stock…"
    /// text and no `addStep` action binding; the satisfied one binds `addStep` and carries no reason.
    #[semio_framework_async_macros::async_test]
    async fn catalogue_flags_a_violated_max_rule_and_not_a_satisfied_one() {
        use crate::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, StockQuantity, Stock, WorkingSolid, Workshop, WorkshopMachine};
        let mut fixture = crate::artifacts::process3d::empty_process3d_snapshot();
        fixture.stock_payload = Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 0.5, depth: 0.5, height: 0.2 }, pose: Default::default() };
        fixture.workshop = Workshop {
            machines: vec![WorkshopMachine {
                id: "saw".into(),
                label: "Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "shallowCrosscut".into(),
                        label: "Shallow Crosscut".into(),
                        icon_id: "scissors".into(),
                        recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                        parameters: vec![CapabilityParameter { id: "maxCutDepth".into(), label: "Max Cut Depth".into(), value: 0.1 }],
                        rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }],
                    },
                    Capability {
                        id: "deepCrosscut".into(),
                        label: "Deep Crosscut".into(),
                        icon_id: "scissors".into(),
                        recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                        parameters: vec![CapabilityParameter { id: "maxCutDepth".into(), label: "Max Cut Depth".into(), value: 0.5 }],
                        rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }],
                    },
                ],
            }],
        };
        let labels = crate::editor::process3d::terminology::process3d_labels(&crate::editor::process3d::config::Process3dConfig::default());
        let node = render(&fixture, "[]", labels).expect("catalogue renders");
        let rendered = serde_json::to_string(&node).expect("render json");
        assert!(rendered.contains("process3d-catalogue.saw.shallowCrosscut"), "expected the violated capability item: {rendered}");
        assert!(rendered.contains("process3d-catalogue.saw.deepCrosscut"), "expected the satisfied capability item: {rendered}");
        assert!(rendered.contains("needs stock height"), "expected the violated rule's reason text: {rendered}");
        assert!(rendered.contains("addStep"), "expected the satisfied capability to still bind addStep: {rendered}");
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
