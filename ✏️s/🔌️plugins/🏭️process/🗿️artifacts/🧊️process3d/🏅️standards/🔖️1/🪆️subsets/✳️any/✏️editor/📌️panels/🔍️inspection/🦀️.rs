//! 🔍️ Process 3d play app panel — the field inspector for whatever is selected: the stock, a process
//! step, or a workshop machine.
//!
//! 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `Process3dPlayApp::render_with_request_context`
//! resolves the live `"geometry"` domain selection (`PROCESS3D_INTERACTION_DOMAIN`) once per render and
//! threads the selected ids here — the SAME canonical targets `🗿️artifact`/`🛠️workshop` bind their trees
//! to (`fixture.stock_id`, a `step_payloads` entry's own `id`, or a workshop machine's `"machine:{id}"`).
//! This panel resolves that id against the document and renders its real fields; an empty selection (or
//! one that resolves to nothing, e.g. a just-deleted step) still falls back to the empty state.

use crate::editor::process3d::terminology::{process3d_measure_label, Process3dLabels};
use crate::{Capability, Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, Stock, WorkingSolid, WorkshopMachine};
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
/// 🧾️ One read-only `"{label}: {value}"` row, admitted into a field list under `id`.
fn push_field(fields: &mut semio_framework_plugin::UiFixedList<BuiltNode>, id: impl AsRef<str>, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<()> {
    fields.try_push(tree_item(id, format!("{label}: {value}"))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed inspector field admission failed"))
}

/// 🧱️ A `WorkingSolid`'s kind plus its own variant-specific dimensions.
fn push_working_solid_fields(fields: &mut semio_framework_plugin::UiFixedList<BuiltNode>, prefix: &str, solid: &WorkingSolid, labels: &Process3dLabels) -> UiAssemblyResult<()> {
    match solid {
        WorkingSolid::Box { width, depth, height } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_box.as_str())?;
            push_field(fields, format!("{prefix}.width"), labels.field_width.as_str(), width)?;
            push_field(fields, format!("{prefix}.depth"), labels.field_depth.as_str(), depth)?;
            push_field(fields, format!("{prefix}.height"), labels.field_height.as_str(), height)
        }
        WorkingSolid::Cylinder { radius, height } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_cylinder.as_str())?;
            push_field(fields, format!("{prefix}.radius"), labels.field_radius.as_str(), radius)?;
            push_field(fields, format!("{prefix}.height"), labels.field_height.as_str(), height)
        }
        WorkingSolid::Sphere { radius } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_sphere.as_str())?;
            push_field(fields, format!("{prefix}.radius"), labels.field_radius.as_str(), radius)
        }
        WorkingSolid::ImportedMesh { mesh_url } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_imported_mesh.as_str())?;
            push_field(fields, format!("{prefix}.url"), labels.label_field.as_str(), mesh_url)
        }
        WorkingSolid::ImportedSolid { solid_handle } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_imported_solid.as_str())?;
            push_field(fields, format!("{prefix}.handle"), labels.label_field.as_str(), solid_handle)
        }
    }
}

/// 🧭️ A `Pose`'s position, axis and angle.
fn push_pose_fields(fields: &mut semio_framework_plugin::UiFixedList<BuiltNode>, prefix: &str, pose: &Pose, labels: &Process3dLabels) -> UiAssemblyResult<()> {
    push_field(fields, format!("{prefix}.x"), labels.field_pos_x.as_str(), pose.position[0])?;
    push_field(fields, format!("{prefix}.y"), labels.field_pos_y.as_str(), pose.position[1])?;
    push_field(fields, format!("{prefix}.z"), labels.field_pos_z.as_str(), pose.position[2])?;
    push_field(fields, format!("{prefix}.axis"), labels.axis_field.as_str(), format!("[{}, {}, {}]", pose.axis[0], pose.axis[1], pose.axis[2]))?;
    push_field(fields, format!("{prefix}.angle"), labels.field_angle.as_str(), pose.angle)
}
//#endregion 🔖️Fields

//#region 🔖️Sections
fn empty_state(labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let items = crate::editor::process3d::ui_node_list([tree_item("process3d-play-inspector.empty", crate::editor::process3d::ui_label(labels.no_selection.as_str())?)])?;
    PanelTreeBuilder::new("process3d-play-inspector")?.section("process3d-play-inspector.section", Some(crate::editor::process3d::ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, items)?.build()
}

/// 🪵️ Id, label, `WorkingSolid` dimensions and `Pose` of the selected stock.
fn render_stock(stock: &Stock, labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut fields = semio_framework_plugin::UiFixedList::default();
    push_field(&mut fields, "process3d-play-inspector.stock.id", labels.id_field.as_str(), &stock.id)?;
    push_field(&mut fields, "process3d-play-inspector.stock.label", labels.label_field.as_str(), &stock.label)?;
    push_working_solid_fields(&mut fields, "process3d-play-inspector.stock.solid", &stock.solid, labels)?;
    push_pose_fields(&mut fields, "process3d-play-inspector.stock.pose", &stock.pose, labels)?;
    PanelTreeBuilder::new("process3d-play-inspector")?.section("process3d-play-inspector.stock", Some(crate::editor::process3d::ui_label(labels.stock.as_str())?), true, fields)?.build()
}

/// 🎞️ Label, enabled flag, `StepOrigin` (machine + capability) and `ProcessMeasure` of the selected step.
fn render_step(step: &ProcessStep, labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut fields = semio_framework_plugin::UiFixedList::default();
    push_field(&mut fields, "process3d-play-inspector.step.id", labels.id_field.as_str(), &step.id)?;
    push_field(&mut fields, "process3d-play-inspector.step.label", labels.label_field.as_str(), &step.label)?;
    push_field(&mut fields, "process3d-play-inspector.step.enabled", labels.enabled.as_str(), step.enabled)?;
    if let Some(origin) = &step.origin {
        push_field(&mut fields, "process3d-play-inspector.step.machine", labels.machine_field.as_str(), &origin.machine_id)?;
        push_field(&mut fields, "process3d-play-inspector.step.capability", labels.capability_field.as_str(), &origin.capability_id)?;
    }
    push_field(&mut fields, "process3d-play-inspector.step.kind", labels.kind_field.as_str(), process3d_measure_label(&step.measure, labels).as_str())?;
    match &step.measure {
        ProcessMeasure::Cut { tool, pose } => {
            push_working_solid_fields(&mut fields, "process3d-play-inspector.step.tool", tool, labels)?;
            push_pose_fields(&mut fields, "process3d-play-inspector.step.pose", pose, labels)?;
        }
        ProcessMeasure::Drill { radius, depth, pose } => {
            push_field(&mut fields, "process3d-play-inspector.step.radius", labels.field_radius.as_str(), radius)?;
            push_field(&mut fields, "process3d-play-inspector.step.depth", labels.field_depth.as_str(), depth)?;
            push_pose_fields(&mut fields, "process3d-play-inspector.step.pose", pose, labels)?;
        }
        ProcessMeasure::Attach { component, pose } => {
            push_working_solid_fields(&mut fields, "process3d-play-inspector.step.component", component, labels)?;
            push_pose_fields(&mut fields, "process3d-play-inspector.step.pose", pose, labels)?;
        }
    }
    PanelTreeBuilder::new("process3d-play-inspector")?.section("process3d-play-inspector.step", Some(crate::editor::process3d::ui_label(labels.step_control.as_str())?), true, fields)?.build()
}

/// 🛠️ Label, icon and every capability (with its parameters) of the selected workshop machine, one
/// subsection per capability, mirroring `🛠️workshop`'s own per-catalog section idiom.
fn render_machine(machine: &WorkshopMachine, labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut summary = semio_framework_plugin::UiFixedList::default();
    push_field(&mut summary, "process3d-play-inspector.machine.id", labels.id_field.as_str(), &machine.id)?;
    push_field(&mut summary, "process3d-play-inspector.machine.label", labels.label_field.as_str(), &machine.label)?;
    push_field(&mut summary, "process3d-play-inspector.machine.icon", labels.icon_field.as_str(), &machine.icon_id)?;
    let mut builder = PanelTreeBuilder::new("process3d-play-inspector")?.section("process3d-play-inspector.machine", Some(crate::editor::process3d::ui_label(labels.machine_field.as_str())?), true, summary)?;
    for capability in &machine.capabilities {
        builder = builder.section(format!("process3d-play-inspector.capability.{}", capability.id), Some(crate::editor::process3d::ui_label(&capability.label)?), false, capability_parameter_fields(capability)?)?;
    }
    builder.build()
}

fn capability_parameter_fields(capability: &Capability) -> UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
    let mut fields = semio_framework_plugin::UiFixedList::default();
    for parameter in &capability.parameters {
        push_field(&mut fields, format!("process3d-play-inspector.capability.{}.{}", capability.id, parameter.id), parameter.label.as_str(), parameter.value)?;
    }
    Ok(fields)
}
//#endregion 🔖️Sections

//#region 🔖️Render
/// 🔍️ Resolves `selected_ids.first()` against `fixture` (stock, then a workshop machine's
/// `"machine:{id}"`, then a step) and renders that selection's real fields; an empty or
/// unresolvable selection renders the empty state.
pub fn render(fixture: &Process3dSnapshot, selected_ids: &[String], labels: &Process3dLabels) -> UiAssemblyResult<BuiltNode> {
    let Some(selected_id) = selected_ids.first() else {
        return empty_state(labels);
    };
    if selected_id == &fixture.stock_id {
        return render_stock(&fixture.stock_payload, labels);
    }
    if let Some(machine_id) = selected_id.strip_prefix("machine:") {
        if let Some(machine) = fixture.workshop.machines.iter().find(|machine| machine.id == machine_id) {
            return render_machine(machine, labels);
        }
    }
    if let Some(step) = fixture.step_payloads.iter().find(|step| &step.id == selected_id) {
        return render_step(step, labels);
    }
    empty_state(labels)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
