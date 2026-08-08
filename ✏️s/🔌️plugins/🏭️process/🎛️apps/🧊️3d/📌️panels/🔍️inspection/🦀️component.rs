//! 🔍️ Process 3d play app panel — the field inspector for whatever is selected: the stock, a process
//! step, or a workshop machine.

use crate::apps::process3d::config::Process3dConfig;
use crate::apps::process3d::process3d_action;
use crate::apps::process3d::terminology::{process3d_measure_label, Process3dLabels};
use crate::artifacts::process3d::engine::{find_capability, processed_volume, validate_capability, validation_context_for_stock, validation_reason};
use crate::artifacts::process3d::{CapabilityRule, Process3dDocument, ProcessStep, SolidSpec, Stock, StockQuantity, WorkshopMachine};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_INSPECTION: &str = "process.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PROCESS_3D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Fields
fn number_field(id: impl Into<String>, label: impl Into<Label>, value: f64, target: &str, field: &str) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{id}.input"),
            input_kind: "number".into(),
            value: value.to_string(),
            placeholder: None,
            commit: None,
            on_change: process3d_action("patchInspector", Some(json!({ "target": target, "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        menu: None,
    })
}

fn text_field(id: impl Into<String>, label: impl Into<Label>, value: &str, target: &str, field: &str) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: None,
            on_change: process3d_action("patchInspector", Some(json!({ "target": target, "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        menu: None,
    })
}
//#endregion 🔖️Fields

//#region 🔖️StockInspector
fn build_stock_inspector(stock: &Stock, fixture: &Process3dDocument, labels: &Process3dLabels) -> UiNode {
    let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &stock.label, &stock.id, "label")];
    match &stock.solid {
        SolidSpec::Box { width, depth, height } => {
            fields.push(number_field("process3d-inspector.width", labels.field_width, *width, &stock.id, "width"));
            fields.push(number_field("process3d-inspector.depth", labels.field_depth, *depth, &stock.id, "depth"));
            fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &stock.id, "height"));
        }
        SolidSpec::Cylinder { radius, height } => {
            fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &stock.id, "radius"));
            fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &stock.id, "height"));
        }
        SolidSpec::Sphere { radius } => {
            fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &stock.id, "radius"));
        }
        SolidSpec::ImportedMesh { mesh_url } => {
            fields.push(ui_inspector_readonly_field("process3d-inspector.source", labels.source, mesh_url.clone()));
        }
        SolidSpec::ImportedSolid { solid_handle } => {
            fields.push(ui_inspector_readonly_field("process3d-inspector.source", labels.source, format!("solid #{solid_handle}")));
        }
    }
    fields.push(number_field("process3d-inspector.posX", labels.field_pos_x, stock.pose.position[0], &stock.id, "posX"));
    fields.push(number_field("process3d-inspector.posY", labels.field_pos_y, stock.pose.position[1], &stock.id, "posY"));
    fields.push(number_field("process3d-inspector.posZ", labels.field_pos_z, stock.pose.position[2], &stock.id, "posZ"));
    fields.push(number_field("process3d-inspector.angle", labels.field_angle, stock.pose.angle, &stock.id, "angle"));
    if let Some(volume) = processed_volume(fixture) {
        fields.push(ui_inspector_readonly_field("process3d-inspector.volume", labels.volume, format!("{volume:.4} m³")));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "process3d-inspector.stock".into(), label: labels.stock.into(), default_open: Some(true), fields }])
}
//#endregion 🔖️StockInspector

//#region 🔖️MachineInspector
/// 🏷️ Human-readable summary of one capability rule, for the machine inspector's readonly rule list.
fn describe_capability_rule(rule: &CapabilityRule) -> String {
    let (quantity, parameter, margin, is_min) = match rule {
        CapabilityRule::Min { quantity, parameter, margin } => (*quantity, parameter, *margin, true),
        CapabilityRule::Max { quantity, parameter, margin } => (*quantity, parameter, *margin, false),
    };
    let axis = match quantity {
        StockQuantity::Width => "width",
        StockQuantity::Depth => "depth",
        StockQuantity::Height => "height",
        StockQuantity::MaxDimension => "max dimension",
        StockQuantity::MinDimension => "min dimension",
    };
    let comparator = if is_min { "≥" } else { "≤" };
    if margin != 0.0 {
        format!("{axis} {comparator} {parameter} ± {margin}m")
    } else {
        format!("{axis} {comparator} {parameter}")
    }
}

/// 🛠️ Inspector for a selected workshop machine: its label, plus one field group per capability with a
/// number field for every parameter and a readonly summary of its rules.
fn build_machine_inspector(machine: &WorkshopMachine, labels: &Process3dLabels) -> UiNode {
    let target = format!("machine:{}", machine.id);
    let machine_fields = vec![text_field("process3d-inspector.label", labels.label_field, &machine.label, &target, "label")];
    let mut groups = vec![UiInspectorFieldGroup { presence: UiPresence::default(), id: "process3d-inspector.machine".into(), label: labels.workshop.into(), default_open: Some(true), fields: machine_fields }];
    for capability in &machine.capabilities {
        let mut fields: Vec<UiNode> = capability
            .parameters
            .iter()
            .map(|parameter| {
                let field = format!("{}.{}", capability.id, parameter.id);
                number_field(format!("process3d-inspector.{field}"), Label::data(parameter.label.clone()), parameter.value, &target, &field)
            })
            .collect();
        if !capability.rules.is_empty() {
            let summary = capability.rules.iter().map(describe_capability_rule).collect::<Vec<_>>().join("; ");
            fields.push(ui_inspector_readonly_field(format!("process3d-inspector.{}.rules", capability.id), labels.validation_warning, summary));
        }
        groups.push(UiInspectorFieldGroup { presence: UiPresence::default(), id: format!("process3d-inspector.{}", capability.id), label: Label::data(capability.label.clone()), default_open: Some(true), fields });
    }
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️MachineInspector

//#region 🔖️StepInspector
fn build_step_inspector(step: &ProcessStep, fixture: &Process3dDocument, labels: &Process3dLabels) -> UiNode {
    let target = format!("step:{}", step.id);
    let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &step.label, &target, "label")];
    if let Some(origin) = &step.origin {
        match find_capability(&fixture.workshop, &origin.machine_id, &origin.capability_id) {
            Some((machine, capability)) => {
                fields.push(ui_inspector_readonly_field("process3d-inspector.origin", labels.provenance, format!("{} · {}", machine.label, capability.label)));
                let failures = validate_capability(capability, &validation_context_for_stock(&fixture.stock));
                if !failures.is_empty() {
                    fields.push(ui_inspector_readonly_field("process3d-inspector.validation", labels.validation_warning, validation_reason(&failures)));
                }
            }
            // 🏭️ Unresolvable provenance (the machine/capability was since removed from the workshop) —
            // StepOrigin is purely informational, so this shows the raw ids and never blocks the step.
            None => {
                fields.push(ui_inspector_readonly_field("process3d-inspector.origin", labels.provenance, format!("{} · {}", origin.machine_id, origin.capability_id)));
            }
        }
    }
    let pose = match &step.measure {
        crate::artifacts::process3d::ProcessMeasure::Cut { tool, pose } => {
            if let SolidSpec::Box { width, depth, height } = tool {
                fields.push(number_field("process3d-inspector.toolWidth", labels.field_width, *width, &target, "toolWidth"));
                fields.push(number_field("process3d-inspector.toolDepth", labels.field_depth, *depth, &target, "toolDepth"));
                fields.push(number_field("process3d-inspector.toolHeight", labels.field_height, *height, &target, "toolHeight"));
            }
            pose
        }
        crate::artifacts::process3d::ProcessMeasure::Drill { radius, depth, pose } => {
            fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &target, "radius"));
            fields.push(number_field("process3d-inspector.depth", labels.field_depth, *depth, &target, "depth"));
            pose
        }
        crate::artifacts::process3d::ProcessMeasure::Attach { component, pose } => {
            if let SolidSpec::Cylinder { radius, height } = component {
                fields.push(number_field("process3d-inspector.radius", labels.field_radius, *radius, &target, "radius"));
                fields.push(number_field("process3d-inspector.height", labels.field_height, *height, &target, "height"));
            }
            pose
        }
    };
    fields.push(number_field("process3d-inspector.posX", labels.field_pos_x, pose.position[0], &target, "posX"));
    fields.push(number_field("process3d-inspector.posY", labels.field_pos_y, pose.position[1], &target, "posY"));
    fields.push(number_field("process3d-inspector.posZ", labels.field_pos_z, pose.position[2], &target, "posZ"));
    fields.push(number_field("process3d-inspector.angle", labels.field_angle, pose.angle, &target, "angle"));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "process3d-inspector.step".into(), label: process3d_measure_label(&step.measure, labels).into(), default_open: Some(true), fields }])
}
//#endregion 🔖️StepInspector

//#region 🔖️Render
pub fn render(fixture: &Process3dDocument, cfg: &Process3dConfig, labels: &Process3dLabels) -> UiNode {
    let Some(selected_id) = cfg.selected_id.as_deref() else {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "process3d-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.no_selection)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    if selected_id == fixture.stock.id {
        return build_stock_inspector(&fixture.stock, fixture, labels);
    }
    if let Some(step) = fixture.steps.iter().find(|step| step.id == selected_id) {
        return build_step_inspector(step, fixture, labels);
    }
    if let Some(machine_id) = selected_id.strip_prefix("machine:") {
        if let Some(machine) = fixture.workshop.machines.iter().find(|machine| machine.id == machine_id) {
            return build_machine_inspector(machine, labels);
        }
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "process3d-play-inspector.missing".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![ui_text(labels.no_selection)],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::process3d::commands::inspector::patch_inspector;
    use crate::apps::process3d::commands::step::add_step;
    use crate::apps::process3d::testkit;
    use crate::apps::process3d::Process3dCommand;

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_INSPECTION));
    }

    #[test]
    fn add_step_action_inserts_and_selects() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
        let document = app.projection().expect("projection");
        assert_eq!(document.steps.len(), 5);
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(!rendered.contains("No selection"), "expected the newly added step to be selected: {rendered}");
    }

    #[test]
    fn add_step_via_catalogue_sets_origin_and_builds_capability_sized_tool() {
        use crate::artifacts::process3d::{ProcessMeasure, SolidSpec};
        let mut app = testkit::app();
        // 🪚️ Circular saw's realistic 0.065m max cut depth needs a shallower stock than the default 0.24m beam.
        testkit::dispatch(&mut app, Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }));
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        assert!(!result.mutations.is_empty(), "circular saw crosscut should be valid against the shrunk stock");
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        let origin = last.origin.as_ref().expect("origin");
        assert_eq!(origin.machine_id, "circularSaw");
        assert_eq!(origin.capability_id, "crosscut");
        let ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius, .. }, .. } = &last.measure else {
            panic!("expected a cylinder cut tool, got {:?}", last.measure);
        };
        assert!((radius - 0.092).abs() < 1e-9, "circular saw diameter 0.184 should size the tool to radius 0.092, got {radius}");
    }

    /// 🪵️ Table saw needs stock height <= 0.102m; the default timber beam is 0.24m tall.
    #[test]
    fn add_step_via_catalogue_rejected_when_validation_fails() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("tableSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        assert!(result.mutations.is_empty(), "table saw crosscut should be rejected server-side against oversized stock");
    }

    #[test]
    fn measure_arg_routes_to_generic_machine() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        let origin = last.origin.as_ref().expect("origin");
        assert_eq!(origin.machine_id, "saw");
        assert_eq!(origin.capability_id, "cut");
        assert!(matches!(last.measure, crate::artifacts::process3d::ProcessMeasure::Cut { .. }));
    }

    #[test]
    fn inspector_shows_validation_warning_after_stock_grows_above_step_requirement() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }));
        let add_result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        assert!(!add_result.mutations.is_empty());
        testkit::dispatch(&mut app, Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.5), text: None }));
        let step_id = app.projection().expect("projection").steps.last().expect("step").id.clone();
        testkit::dispatch(&mut app, Process3dCommand::SetSelection(crate::apps::process3d::commands::selection::set_selection::SetSelection { id: Some(step_id) }));
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("needs stock"), "expected a validation warning after growing stock above the step's max cut depth: {rendered}");
    }

    #[test]
    fn step_inspector_shows_raw_provenance_after_machine_removal() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }));
        testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        let step_id = app.projection().expect("projection").steps.last().expect("step").id.clone();
        testkit::dispatch(&mut app, Process3dCommand::RemoveWorkshopMachine(crate::apps::process3d::commands::workshop::remove_workshop_machine::RemoveWorkshopMachine { id: "circularSaw".into() }));
        testkit::dispatch(&mut app, Process3dCommand::SetSelection(crate::apps::process3d::commands::selection::set_selection::SetSelection { id: Some(step_id) }));
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("circularSaw") && rendered.contains("crosscut"), "step provenance must survive machine removal as raw ids: {rendered}");
    }
}
//#endregion 🧪️Tests
