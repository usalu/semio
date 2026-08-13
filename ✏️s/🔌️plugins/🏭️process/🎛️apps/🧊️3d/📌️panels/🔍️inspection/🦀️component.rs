//! 🔍️ Process 3d play app panel — the field inspector for whatever is selected: the stock, a process
//! step, or a workshop machine.

use crate::apps::process3d::config::Process3dConfig;
use crate::apps::process3d::process3d_action;
use crate::apps::process3d::terminology::{process3d_measure_label, Process3dLabels};
use crate::artifacts::process3d::schema::inferences::{find_capability, validate_capability, validation_reason, ValidationContext};
use crate::artifacts::process3d::{CapabilityRule, Process3dSnapshot, ProcessStep, StockQuantity, WorkingSolid, WorkshopMachine};
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
/// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.stock_solid` is a
/// composed `s.stdio.semio.brep` CHILD HANDLE now, with no resolvable dimensions without a
/// `LinkResolver` (see `ProcessWorkingScene`'s doc comment) — the per-kind dimension fields (and
/// the replayed volume, which needs the same resolved content) are replaced by one readonly note
/// naming the child handle; `label`/pose fields stay real (inline persisted fields).
fn build_stock_inspector(fixture: &Process3dSnapshot, labels: &Process3dLabels) -> UiNode {
    let stock_id = fixture.stock_id.as_str();
    let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &fixture.stock_label, stock_id, "label")];
    fields.push(ui_inspector_readonly_field("process3d-inspector.source", labels.source, format!("brep #{}", fixture.stock_solid.child_id)));
    fields.push(number_field("process3d-inspector.posX", labels.field_pos_x, fixture.stock_pose.position[0], stock_id, "posX"));
    fields.push(number_field("process3d-inspector.posY", labels.field_pos_y, fixture.stock_pose.position[1], stock_id, "posY"));
    fields.push(number_field("process3d-inspector.posZ", labels.field_pos_z, fixture.stock_pose.position[2], stock_id, "posZ"));
    fields.push(number_field("process3d-inspector.angle", labels.field_angle, fixture.stock_pose.angle, stock_id, "angle"));
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
fn build_step_inspector(step: &ProcessStep, fixture: &Process3dSnapshot, labels: &Process3dLabels) -> UiNode {
    let target = format!("step:{}", step.id);
    let mut fields = vec![text_field("process3d-inspector.label", labels.label_field, &step.label, &target, "label")];
    if let Some(origin) = &step.origin {
        match find_capability(&fixture.workshop, &origin.machine_id, &origin.capability_id) {
            Some((machine, capability)) => {
                fields.push(ui_inspector_readonly_field("process3d-inspector.origin", labels.provenance, format!("{} · {}", machine.label, capability.label)));
                // 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: same documented gap
                // as `📌️panels/🛍️catalogue` — `fixture.stock_solid` carries no resolvable
                // dimensions without a resolver, so every capability is treated as satisfied.
                let ctx = ValidationContext { stock_width: f64::MAX, stock_depth: f64::MAX, stock_height: f64::MAX };
                let failures = validate_capability(capability, &ctx);
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
            if let WorkingSolid::Box { width, depth, height } = tool {
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
            if let WorkingSolid::Cylinder { radius, height } = component {
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
pub fn render(fixture: &Process3dSnapshot, cfg: &Process3dConfig, labels: &Process3dLabels) -> UiNode {
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
    if selected_id == fixture.stock_id {
        return build_stock_inspector(fixture, labels);
    }
    // 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.steps` is a composed
    // CHILD HANDLE now, with no resolvable content without a `LinkResolver` (see
    // `ProcessWorkingScene`'s doc comment) — the working scene's step list is honestly empty, so a
    // step-addressed selection falls through to the "missing" branch below, a documented gap.
    let scene = crate::artifacts::process3d::process_working_scene_from_snapshot(fixture);
    if let Some(step) = scene.steps.iter().find(|step| step.id == selected_id) {
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
    use crate::apps::process3d::commands::step::add_step;
    use crate::apps::process3d::testkit;
    use crate::apps::process3d::Process3dCommand;

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_INSPECTION));
    }

    //#region 🔖️AddStepIsADocumentedNoOp
    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `AddStep` dispatches a
    /// `CreateStep` mutation, a documented no-op now (`steps` composes an `s.stdio.semio.flow`
    /// CHILD HANDLE — no resolver, see `ProcessWorkingScene`'s doc comment), so the persisted
    /// `document.steps` this test suite used to read is gone. `add_step::handle`'s own capability-
    /// dimension VALIDATION gate is also a documented gap now (no resolvable stock extent — see
    /// its own doc comment), so every resolvable machine/capability pair succeeds unconditionally.
    /// These tests assert the honest post-migration behavior: the command still dispatches its
    /// (no-op) mutation and its real, unaffected config-mutation side effect (selecting the new
    /// step id), matching `📐️cad`'s own `..._is_a_documented_no_op_pending_the_child_dispatch_seam`
    /// precedent. The real, unaffected capability→measure sizing math is covered directly by
    /// `📌️panels/🛠️workshop`'s `workshop_machine_parameter_edit_sizes_the_capability_measure`.
    /// 🌉️ The selection itself (a real, unaffected `Process3dConfigMutation::SetSelectedId`) still
    /// commits — but the inspector can no longer resolve the selected step id against
    /// `scene.steps` (always empty, the same documented gap), so it falls through to the generic
    /// "missing" branch rather than the truly-no-selection "empty" branch. Both branches happen to
    /// render the same `labels.no_selection` text, so this asserts on the distinct section id
    /// instead, proving the selection dispatch really happened.
    #[test]
    fn add_step_dispatches_its_no_op_mutation_and_selects_the_new_id() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.mutations.is_empty(), "AddStep must still dispatch its (no-op) CreateStep mutation");
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("process3d-play-inspector.missing"), "expected the newly added step id to be selected (routes to the unresolved-selection branch): {rendered}");
    }

    /// 🌉️ Same documented gap as above, from the catalogue-routed (machine/capability-addressed)
    /// entry point: even a stock the pre-migration code would have rejected (circular saw needs
    /// height ≤ 0.065m; the default timber beam is 0.24m) now succeeds, since the dimension gate
    /// can no longer read real stock extents.
    #[test]
    fn add_step_via_catalogue_no_longer_gates_on_stock_dimensions() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        assert!(!result.mutations.is_empty(), "documented gap: the dimension-validation gate can no longer reject an oversized stock");
    }

    #[test]
    fn measure_arg_routes_to_generic_machine_and_dispatches() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.mutations.is_empty());
    }
    //#endregion 🔖️AddStepIsADocumentedNoOp
}
//#endregion 🧪️Tests
