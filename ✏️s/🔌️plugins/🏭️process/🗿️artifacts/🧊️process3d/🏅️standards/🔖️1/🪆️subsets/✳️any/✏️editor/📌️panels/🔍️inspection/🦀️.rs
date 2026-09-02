//! 🔍️ Process 3d play app panel — the field inspector for whatever is selected: the stock, a process
//! step, or a workshop machine.
//!
//! 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `Process3dPlayApp::render_with_request_context`
//! resolves the live `"geometry"` domain selection (`PROCESS3D_INTERACTION_DOMAIN`) once per render and
//! threads the selected ids here — the SAME canonical targets `📄️artifact`/`🛠️workshop` bind their trees
//! to (`fixture.stock_id`, a `step_payloads` entry's own `id`, or a workshop machine's `"machine:{id}"`).
//! This panel resolves that id against the document and renders its real fields; an empty selection (or
//! one that resolves to nothing, e.g. a just-deleted step) still falls back to the empty state.

use crate::artifacts::process3d::{Capability, Pose, ProcessMeasure, ProcessStep, Process3dSnapshot, Stock, WorkingSolid, WorkshopMachine};
use crate::editor::process3d::terminology::{process3d_measure_label, Process3dLabels};
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
    fields
        .try_push(tree_item(id, format!("{label}: {value}"))?)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed inspector field admission failed"))
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
mod tests {
    use super::*;
    use crate::editor::process3d::commands::step::add_step;
    use crate::editor::process3d::testkit;
    use crate::editor::process3d::{Process3dCommand, PROCESS3D_INTERACTION_DOMAIN};
    use semio_framework::DslValue;

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_INSPECTION));
    }

    //#region 🔖️AddStepDispatch
    /// 🌉️ `AddStep` dispatches a real `CreateStep` mutation against `step_payloads`.
    /// `add_step::handle`'s own capability-dimension VALIDATION gate is a documented gap (no
    /// resolvable stock extent — see its own doc comment), so every resolvable machine/capability
    /// pair succeeds unconditionally.
    #[semio_framework_async_macros::async_test]
    async fn add_step_dispatches_its_mutation() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.mutations.is_empty(), "AddStep must dispatch its CreateStep mutation");
    }

    /// 🌉️ Same documented gap as above, from the catalogue-routed (machine/capability-addressed)
    /// entry point: even a stock the pre-migration code would have rejected (circular saw needs
    /// height ≤ 0.065m; the default timber beam is 0.24m) now succeeds, since the dimension gate
    /// can no longer read real stock extents.
    #[semio_framework_async_macros::async_test]
    async fn add_step_via_catalogue_no_longer_gates_on_stock_dimensions() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        assert!(!result.mutations.is_empty(), "documented gap: the dimension-validation gate can no longer reject an oversized stock");
    }

    #[semio_framework_async_macros::async_test]
    async fn measure_arg_routes_to_generic_machine_and_dispatches() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.mutations.is_empty());
    }
    //#endregion 🔖️AddStepDispatch

    //#region 🔖️SelectionInspector
    fn select(app: &mut testkit::Process3dApp, id: &str) {
        let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "object".into(), id: id.into() }]).expect("targets serialize");
        let args = DslValue::Object(vec![
            ("domainId".to_string(), DslValue::String(PROCESS3D_INTERACTION_DOMAIN.into())),
            ("targets".to_string(), DslValue::String(targets)),
            ("merge".to_string(), DslValue::String("replace".into())),
            ("method".to_string(), DslValue::String("pick".into())),
        ]);
        testkit::action(app, semio_framework_plugin::INTERACTION_SELECT_ACTION_ID, Some(&args));
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_selection_still_renders_the_empty_state() {
        let mut app = testkit::app_with_registry();
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("process3d-play-inspector.empty"));
    }

    #[semio_framework_async_macros::async_test]
    async fn selected_stock_id_renders_its_dimensions() {
        let mut app = testkit::app_with_registry();
        let stock_id = app.snapshot().expect("snapshot").stock_id.clone();
        select(&mut app, &stock_id);
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("process3d-play-inspector.stock"), "expected the stock section: {rendered}");
        assert!(rendered.contains("Width: 1"), "expected the default box stock's width: {rendered}");
        assert!(rendered.contains("Height: 1"), "expected the default box stock's height: {rendered}");
    }

    #[semio_framework_async_macros::async_test]
    async fn selected_step_id_renders_its_label_and_measure_kind() {
        let mut app = testkit::app_with_registry();
        testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
        let step = app.snapshot().expect("snapshot").step_payloads.last().expect("added step").clone();
        select(&mut app, &step.id);
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("process3d-play-inspector.step"), "expected the step section: {rendered}");
        assert!(rendered.contains(&step.label), "expected the step's label {}: {rendered}", step.label);
        assert!(rendered.contains("Kind: Drill"), "expected the measure kind: {rendered}");
        assert!(rendered.contains("Radius: 0.05"), "expected the drill capability's own radius parameter: {rendered}");
    }

    #[semio_framework_async_macros::async_test]
    async fn selected_machine_id_renders_its_capabilities() {
        let mut app = testkit::app_with_registry();
        select(&mut app, "machine:saw");
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("process3d-play-inspector.machine"), "expected the machine section: {rendered}");
        assert!(rendered.contains("Generic Saw"), "expected the machine's label: {rendered}");
        assert!(rendered.contains("process3d-play-inspector.capability.cut"), "expected the saw's cut capability section: {rendered}");
        assert!(rendered.contains("Kerf: 0.05"), "expected the cut capability's own kerf parameter: {rendered}");
    }
    //#endregion 🔖️SelectionInspector
}
//#endregion 🧪️Tests
