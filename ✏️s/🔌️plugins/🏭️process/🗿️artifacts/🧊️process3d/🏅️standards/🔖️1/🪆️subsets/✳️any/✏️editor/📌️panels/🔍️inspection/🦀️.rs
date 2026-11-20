//! 🔍️ Process 3d play app panel — the field inspector for whatever is selected: the stock, a process
//! step, or a workshop machine.
//!
//! 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `Process3dPlayApp::render_with_request_context`
//! resolves the live `"geometry"` domain selection (`PROCESS3D_INTERACTION_DOMAIN`) once per render and
//! threads the selected ids here — the SAME canonical targets `🗿️artifact`/`🛠️workshop` bind their trees
//! to (`snapshot.stock_id`, a `step_payloads` entry's own `id`, or a workshop machine's `"machine:{id}"`).
//! This panel resolves that id against the document and renders its real fields; an empty selection (or
//! one that resolves to nothing, e.g. a just-deleted step) still falls back to the empty state.

use semio_s_artifact_process_process3d_kit::{editor::process3d::terminology::process3d_measure_label, editor::process3d::terminology::Process3dLabels};
use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::Pose, snapshot::Process3dSnapshot, snapshot::ProcessMeasure, snapshot::ProcessStep, snapshot::WorkingSolid, snapshot::WorkshopMachine};
use semio_s_artifact_process_process3d_kit::{process3d_action, ui_label, ui_value_map, ui_value_number, ui_value_text};
use semio_plugin::plugin_app_close_prelude::{input, tree_item_builder, Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_plugin::{tree_item, tree_item_with_action, PluginAssemblyError, UiText};
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PanelGroup;
use semio_plugin::PanelTabDefinition;
use semio_plugin::PanelTabKind;
use semio_plugin::PanelTreeBuilder;
use semio_plugin::TreeWindows;
use semio_plugin::UiAssemblyResult;
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_INSPECTION: &str = "process.play.inspection";
pub const PROCESS_3D_PLAY_INSPECTOR_SECTION: &str = "process3d-play-inspector.section";
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
struct Field {
    id: String,
    label: String,
    value: String,
    control: Option<FieldControl>,
}

enum FieldControl {
    Edit { name: String, kind: InputKind },
    Move { index: usize },
}

fn push_field(fields: &mut Vec<Field>, id: impl Into<String>, label: &str, value: impl std::fmt::Display) {
    fields.push(Field { id: id.into(), label: label.into(), value: value.to_string(), control: None });
}

fn push_editable(fields: &mut Vec<Field>, id: impl Into<String>, label: &str, value: impl std::fmt::Display, name: impl Into<String>, kind: InputKind) {
    fields.push(Field { id: id.into(), label: label.into(), value: value.to_string(), control: Some(FieldControl::Edit { name: name.into(), kind }) });
}

fn field_row(entry: &Field) -> UiAssemblyResult<BuiltNode> {
    let text = if entry.value.is_empty() { entry.label.clone() } else { format!("{}: {}", entry.label, entry.value) };
    tree_item(&entry.id, text.as_str())
}

fn move_field(id: &str, label: &str, index: usize) -> Field {
    Field { id: id.into(), label: label.into(), value: String::new(), control: Some(FieldControl::Move { index }) }
}

fn field_control_error() -> PluginAssemblyError {
    PluginAssemblyError::new("ui.inspection", "fixed inspector field admission failed")
}

fn field_control_row(entry: &Field, target: &str) -> UiAssemblyResult<BuiltNode> {
    let (name, kind) = match &entry.control {
        Some(FieldControl::Edit { name, kind }) => (name, *kind),
        Some(FieldControl::Move { index }) => {
            let args = ui_value_map([("id", ui_value_text(target)?), ("index", ui_value_number(*index as f64))])?;
            return tree_item_with_action(&entry.id, ui_label(&entry.label)?, None, process3d_action("moveStep", Some(args))?);
        }
        None => return field_row(entry),
    };
    let args = ui_value_map([("target", ui_value_text(target)?), ("field", ui_value_text(name)?)])?;
    let (action, args) = process3d_action("patchInspector", Some(args))?;
    let value = UiText::try_from_str(&entry.value).ok_or_else(field_control_error)?;
    let draft_target = UiText::try_from_str(&format!("{target}:{name}")).ok_or_else(field_control_error)?;
    let control = input(kind).value(value).commit(UiText::clipped("blur")).draft_target(draft_target)
        .try_id(format!("{}.input", entry.id)).map_err(|_| field_control_error())?
        .try_label(&entry.label).map_err(|_| field_control_error())?;
    let control = match args {
        Some(args) => control.try_on_with(Trigger::Commit, action, args),
        None => control.try_on(Trigger::Commit, action),
    }.map_err(|_| field_control_error())?.try_build().map_err(|_| field_control_error())?;
    tree_item_builder(ui_label(&entry.label)?).try_id(&entry.id).map_err(|_| field_control_error())?
        .try_child(control).map_err(|_| field_control_error())?.try_build().map_err(|_| field_control_error())
}

/// 🧱️ A `WorkingSolid`'s kind plus its own variant-specific dimensions.
fn push_working_solid_fields(fields: &mut Vec<Field>, prefix: &str, solid: &WorkingSolid, labels: &Process3dLabels) {
    match solid {
        WorkingSolid::Box { width, depth, height } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_box.as_str());
            push_editable(fields, format!("{prefix}.width"), labels.field_width.as_str(), width, "width", InputKind::Number);
            push_editable(fields, format!("{prefix}.depth"), labels.field_depth.as_str(), depth, "depth", InputKind::Number);
            push_editable(fields, format!("{prefix}.height"), labels.field_height.as_str(), height, "height", InputKind::Number);
        }
        WorkingSolid::Cylinder { radius, height } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_cylinder.as_str());
            push_editable(fields, format!("{prefix}.radius"), labels.field_radius.as_str(), radius, "radius", InputKind::Number);
            push_editable(fields, format!("{prefix}.height"), labels.field_height.as_str(), height, "height", InputKind::Number);
        }
        WorkingSolid::Sphere { radius } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_sphere.as_str());
            push_editable(fields, format!("{prefix}.radius"), labels.field_radius.as_str(), radius, "radius", InputKind::Number);
        }
        WorkingSolid::ImportedMesh { .. } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_imported_mesh.as_str());
        }
        WorkingSolid::ImportedSolid { .. } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_imported_solid.as_str());
        }
        WorkingSolid::Reference { reference_id } => {
            push_field(fields, format!("{prefix}.kind"), labels.kind_field.as_str(), labels.stock_kind_reference.as_str());
            push_field(fields, format!("{prefix}.reference"), labels.label_field.as_str(), semio_s_artifact_process_process3d_model::snapshot::reference_solid(reference_id).map_or(reference_id.as_str(), |reference| reference.label));
        }
    }
}

/// 🧭️ A `Pose`'s position, axis and angle.
fn push_pose_fields(fields: &mut Vec<Field>, prefix: &str, pose: &Pose, labels: &Process3dLabels) {
    push_editable(fields, format!("{prefix}.x"), labels.field_pos_x.as_str(), pose.position[0], "posX", InputKind::Number);
    push_editable(fields, format!("{prefix}.y"), labels.field_pos_y.as_str(), pose.position[1], "posY", InputKind::Number);
    push_editable(fields, format!("{prefix}.z"), labels.field_pos_z.as_str(), pose.position[2], "posZ", InputKind::Number);
    push_field(fields, format!("{prefix}.axis"), labels.axis_field.as_str(), format!("[{}, {}, {}]", pose.axis[0], pose.axis[1], pose.axis[2]));
    push_editable(fields, format!("{prefix}.angle"), labels.field_angle.as_str(), pose.angle, "angle", InputKind::Number);
}
//#endregion 🔖️Fields

//#region 🔖️Sections
fn empty_state(labels: &Process3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut fields = Vec::new();
    push_field(&mut fields, "process3d-play-inspector.empty", labels.no_selection.as_str(), "");
    PanelTreeBuilder::new("process3d-play-inspector")?
        .window_section(windows, PROCESS_3D_PLAY_INSPECTOR_SECTION, Some(semio_s_artifact_process_process3d_kit::ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, &fields, field_row)?
        .build()
}

/// 🪵️ Id, label, `WorkingSolid` dimensions and `Pose` of the selected stock.
fn render_stock(snapshot: &Process3dSnapshot, labels: &Process3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut fields = Vec::new();
    push_field(&mut fields, "process3d-play-inspector.stock.id", labels.id_field.as_str(), &snapshot.stock_id);
    push_editable(&mut fields, "process3d-play-inspector.stock.label", labels.label_field.as_str(), &snapshot.stock_label, "label", InputKind::Text);
    push_working_solid_fields(&mut fields, "process3d-play-inspector.stock.solid", &snapshot.stock_payload.solid, labels);
    push_pose_fields(&mut fields, "process3d-play-inspector.stock.pose", &snapshot.stock_pose, labels);
    PanelTreeBuilder::new("process3d-play-inspector")?
        .window_section(windows, "process3d-play-inspector.stock", Some(ui_label(labels.stock.as_str())?), true, &fields, |field| field_control_row(field, &snapshot.stock_id))?
        .build()
}

/// 🎞️ Label, enabled flag, `StepOrigin` (machine + capability) and `ProcessMeasure` of the selected step.
fn render_step(step: &ProcessStep, index: usize, total: usize, labels: &Process3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let mut fields = Vec::new();
    push_field(&mut fields, "process3d-play-inspector.step.id", labels.id_field.as_str(), &step.id);
    push_editable(&mut fields, "process3d-play-inspector.step.label", labels.label_field.as_str(), &step.label, "label", InputKind::Text);
    push_field(&mut fields, "process3d-play-inspector.step.enabled", labels.enabled.as_str(), step.enabled);
    if let Some(origin) = &step.origin {
        push_field(&mut fields, "process3d-play-inspector.step.machine", labels.machine_field.as_str(), &origin.machine_id);
        push_field(&mut fields, "process3d-play-inspector.step.capability", labels.capability_field.as_str(), &origin.capability_id);
    }
    push_field(&mut fields, "process3d-play-inspector.step.kind", labels.kind_field.as_str(), process3d_measure_label(&step.measure, labels).as_str());
    match &step.measure {
        ProcessMeasure::Cut { tool, pose } => {
            push_working_solid_fields(&mut fields, "process3d-play-inspector.step.tool", tool, labels);
            push_pose_fields(&mut fields, "process3d-play-inspector.step.pose", pose, labels);
        }
        ProcessMeasure::Drill { radius, depth, pose } => {
            push_editable(&mut fields, "process3d-play-inspector.step.radius", labels.field_radius.as_str(), radius, "radius", InputKind::Number);
            push_editable(&mut fields, "process3d-play-inspector.step.depth", labels.field_depth.as_str(), depth, "depth", InputKind::Number);
            push_pose_fields(&mut fields, "process3d-play-inspector.step.pose", pose, labels);
        }
        ProcessMeasure::Attach { component, pose } => {
            push_working_solid_fields(&mut fields, "process3d-play-inspector.step.component", component, labels);
            push_pose_fields(&mut fields, "process3d-play-inspector.step.pose", pose, labels);
        }
    }
    if index > 0 {
        fields.push(move_field("process3d-play-inspector.step.move-earlier", if labels.is_de() { "Nach oben" } else { "Move Earlier" }, index - 1));
    }
    if index + 1 < total {
        fields.push(move_field("process3d-play-inspector.step.move-later", if labels.is_de() { "Nach unten" } else { "Move Later" }, index + 1));
    }
    PanelTreeBuilder::new("process3d-play-inspector")?
        .window_section(windows, "process3d-play-inspector.step", Some(ui_label(labels.step_control.as_str())?), true, &fields, |field| field_control_row(field, &step.id))?
        .build()
}

/// 🛠️ Label, icon and every capability (with its parameters) of the selected workshop machine, one
/// subsection per capability, mirroring `🛠️workshop`'s own per-catalog section idiom.
fn render_machine(machine: &WorkshopMachine, labels: &Process3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let target = format!("machine:{}", machine.id);
    let mut summary = Vec::new();
    push_field(&mut summary, "process3d-play-inspector.machine.id", labels.id_field.as_str(), &machine.id);
    push_editable(&mut summary, "process3d-play-inspector.machine.label", labels.label_field.as_str(), &machine.label, "label", InputKind::Text);
    push_field(&mut summary, "process3d-play-inspector.machine.icon", labels.icon_field.as_str(), &machine.icon_id);
    let mut builder = PanelTreeBuilder::new("process3d-play-inspector")?
        .window_section(windows, "process3d-play-inspector.machine", Some(ui_label(labels.machine_field.as_str())?), true, &summary, |field| field_control_row(field, &target))?;
    for capability in &machine.capabilities {
        let section_id = format!("process3d-play-inspector.capability.{}", capability.id);
        let fields = capability_parameter_fields(capability);
        builder = builder.window_section(windows, &section_id, Some(ui_label(&capability.label)?), false, &fields, |field| field_control_row(field, &target))?;
    }
    builder.build()
}

fn capability_parameter_fields(capability: &Capability) -> Vec<Field> {
    let mut fields = Vec::new();
    for parameter in &capability.parameters {
        let name = format!("{}.{}", capability.id, parameter.id);
        push_editable(&mut fields, format!("process3d-play-inspector.capability.{name}"), parameter.label.as_str(), parameter.value, name, InputKind::Number);
    }
    fields
}
//#endregion 🔖️Sections

//#region 🔖️Render
/// 🔍️ Resolves `selected_ids.first()` against `snapshot` (stock, then a workshop machine's
/// `"machine:{id}"`, then a step) and renders that selection's real fields; an empty or
/// unresolvable selection renders the empty state.
pub fn render(snapshot: &Process3dSnapshot, selected_ids: &[String], labels: &Process3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let Some(selected_id) = selected_ids.first() else {
        return empty_state(labels, windows);
    };
    if selected_id == &snapshot.stock_id {
        return render_stock(snapshot, labels, windows);
    }
    if let Some(machine_id) = selected_id.strip_prefix("machine:") {
        if let Some(machine) = snapshot.workshop.machines.iter().find(|machine| machine.id == machine_id) {
            return render_machine(machine, labels, windows);
        }
    }
    if let Some((index, step)) = snapshot.step_payloads.iter().enumerate().find(|(_, step)| &step.id == selected_id) {
        return render_step(step, index, snapshot.step_payloads.len(), labels, windows);
    }
    empty_state(labels, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
//#endregion 🧪️Tests
