//! 🖥️ Play app of the BIM editor: the `BimModelApp` and its `ArtifactEditor` implementation. See [`super`].

use crate::editor::bim::commands::{analyse_model, export_sheets, export_schedule_csv, export_model};
use semio_s_artifact_bim_model_tests::editor::bim::config::app_schema_descriptor;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_entities::editor::bim::entities::kind_holding;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner as outliner_panel;
use semio_s_artifact_bim_model_tests::editor::bim::presence::{BimPresence, BimPresenceMutation};
use semio_s_artifact_bim_model_tests::editor::bim::terminology::{bim_labels, BimLabels};
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::{BIM_MODEL_DIALECT, BIM_MODEL_DOCUMENT_SCHEMA};
use semio_framework_artifact_reference::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandInputs, ArtifactRetainedCommandPayload};
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::HashMap;
use crate::editor::bim::{args_bridge};
use crate::editor::bim::*;
use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
use semio_s_artifact_bim_model_tests::editor::bim::config;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::transient;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️BimModelApp
#[derive(Default)]
pub struct BimModelApp;

/// 🧬️ The window-kind to render-body table: which body key a window kind or panel renders.
fn selection_ids(interaction: &InteractionView<'_>, domain: &str) -> Vec<String> {
    interaction.selection(domain).ids.clone()
}

fn hover_ids(interaction: &InteractionView<'_>, domain: &str) -> Vec<String> {
    interaction.hover(domain, "pointer").ids.clone()
}

/// 🚨️ The findings of the model by severity, from the instance's inference (the diagnostic index).
fn problems_of(doc: &ArtifactView<'_, ModelSnapshot>) -> semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::SeverityCounts {
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, doc.snapshot, |inference| inference.diagnostic_index.total)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_body(instance: semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>, body_key: &str, doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel, elements: &[String], library: &[String], hover: &[String], preview: &semio_s_artifact_bim_model_storey_levels::editor::bim::marks::Preview) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let snapshot = doc.snapshot;
    let labels = bim_labels(view_state);
    let utility = semio_s_artifact_bim_model_kit::editor::bim::utilities::active(view_state);
    let windows = || TreeWindows::for_body(view_state, body_key);
    let node = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(instance, snapshot, |inference| match body_key {
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::BODY_KEY => {
            let config = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::current(cfg);
            let revision = semio_s_artifact_bim_model_plan_linework::render::plan::framing_revision(&semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::active_view(snapshot, &config).unwrap_or_default());
            let overlay = semio_s_artifact_bim_model_move_storey::editor::bim::gestures::overlay::plan_records(snapshot, elements, utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            let body = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::render_over(snapshot, inference, &config, elements, hover, utility, revision, labels, &overlay)?;
            semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::options::with_switcher(snapshot, inference, &config.selected_options, &config.workset_visibility, labels, &windows(), body)
        }
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::BODY_KEY => {
            let config = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::current(cfg);
            let body = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::render_over(snapshot, inference, &config, elements, hover, semio_s_artifact_bim_model_plan_linework::render::plan::framing_revision(&format!("{}{}", config.isolated_storey, config.projection.kind)), preview)?;
            semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::options::with_switcher(snapshot, inference, &config.selected_options, &config.workset_visibility, labels, &windows(), body)
        }
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::BODY_KEY => {
            let config = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::current(cfg);
            let (start, end) = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::plane_of(snapshot, &config).map_or(([0.0, 0.0], [0.0, 0.0]), |plane| ([plane.start.x, plane.start.y], [plane.end.x, plane.end.y]));
            let overlay = semio_s_artifact_bim_model_move_storey::editor::bim::gestures::overlay::section_records(inference, start, end, utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            let revision = semio_s_artifact_bim_model_plan_linework::render::plan::framing_revision(&semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::active_view(snapshot, &config).unwrap_or_default());
            semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::render_over(snapshot, inference, &config, elements, utility, revision, labels, &overlay)
        }
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::BODY_KEY => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::render(snapshot, inference, &semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::config::current(cfg), labels),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family::BODY_KEY => semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family::render(snapshot, inference, elements, library, labels),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family::VIEW_BODY_KEY => semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family::view_render(snapshot, inference, elements, library),
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::BODY_KEY => {
            let config = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::config::current(cfg);
            let active = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::active_sheet(snapshot, &config);
            let layout = active.as_ref().and_then(|id| inference.sheet_layouts.get(id));
            let overlay = semio_s_artifact_bim_model_move_storey::editor::bim::gestures::overlay::sheet_records(layout, elements, utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::render_over(snapshot, inference, &config, elements, utility, semio_s_artifact_bim_model_plan_linework::render::plan::framing_revision(&active.unwrap_or_default()), labels, &overlay)
        }
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::options::BODY_KEY => semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::options::render(snapshot, inference, labels, &windows()),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner::BODY_KEY => semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner::render(snapshot, inference, labels, &windows()),
        semio_s_artifact_bim_model_gestures::editor::bim::panels::properties::BODY_KEY => semio_s_artifact_bim_model_gestures::editor::bim::panels::properties::render(snapshot, inference, elements, library, labels),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::library::BODY_KEY => semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::library::render(snapshot, labels, &windows()),
        semio_s_artifact_bim_model_gestures::editor::bim::panels::families::BODY_KEY => semio_s_artifact_bim_model_gestures::editor::bim::panels::families::render(snapshot, inference, labels, library, &windows()),
        semio_s_artifact_bim_model_gestures::editor::bim::panels::classification::BODY_KEY => semio_s_artifact_bim_model_gestures::editor::bim::panels::classification::render(snapshot, labels, elements, library, &windows()),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics::BODY_KEY => semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics::render(snapshot, inference, labels, &windows()),
        semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::CLASHES_KEY => semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::render_clashes(snapshot, inference, labels, &windows()),
        semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::RULES_KEY => semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::render_rules(snapshot, inference, labels, &windows()),
        semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::ISSUES_KEY => semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::render_issues(snapshot, labels, &windows()),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("{}: {body_key}", labels.unknown_body.as_str()))).map_err(|_| semio_s_artifact_bim_model_kit::editor::bim::kit::ui_capacity_error()),
    })?;
    Ok(semio_framework_plugin::built_to_component_tree(accessible_surface(node, body_key, labels)?))
}

/// ♿️ The accessible name of a window or panel body, and for a drawing surface how to drive it without a pointer; a body this table does not know keeps its own node.
fn accessible_surface(node: semio_framework_plugin::BuiltNode, body_key: &str, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let (name, description) = match body_key {
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::BODY_KEY => (labels.window_plan, Some(labels.surface_plan_describe)),
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::BODY_KEY => (labels.window_world, Some(labels.surface_world_describe)),
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::BODY_KEY => (labels.window_section, Some(labels.surface_section_describe)),
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::BODY_KEY => (labels.window_schedule, Some(labels.surface_schedule_describe)),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family::BODY_KEY => (labels.window_family, Some(labels.surface_family_describe)),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family::VIEW_BODY_KEY => (labels.window_family_view, Some(labels.surface_family_view_describe)),
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::BODY_KEY => (labels.window_sheet, Some(labels.surface_sheet_describe)),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::options::BODY_KEY => (labels.panel_options, None),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner::BODY_KEY => (labels.panel_outliner, None),
        semio_s_artifact_bim_model_gestures::editor::bim::panels::properties::BODY_KEY => (labels.panel_properties, None),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::library::BODY_KEY => (labels.panel_library, None),
        semio_s_artifact_bim_model_gestures::editor::bim::panels::families::BODY_KEY => (labels.panel_family_browser, Some(labels.browser_surface_describe)),
        semio_s_artifact_bim_model_gestures::editor::bim::panels::classification::BODY_KEY => (labels.panel_classification, Some(labels.classification_surface_describe)),
        semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics::BODY_KEY => (labels.panel_diagnostics, Some(labels.diag_surface_describe)),
        semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::CLASHES_KEY => (labels.panel_clashes, Some(labels.clash_surface_describe)),
        semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::RULES_KEY => (labels.panel_rules, Some(labels.rules_surface_describe)),
        semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination::ISSUES_KEY => (labels.panel_issues, Some(labels.issues_surface_describe)),
        _ => return Ok(node),
    };
    semio_s_artifact_bim_model_kit::editor::bim::kit::accessible(node, name.as_str(), description.as_ref().map(|description| description.as_str()))
}

impl ArtifactEditor for BimModelApp {
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        bim_fault_notices()
    }

    type Snapshot = ModelSnapshot;
    type Mutation = ModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = BimPresence;
    type PresenceMutation = BimPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = BimCommand;

    const DIALECT: Dialect = BIM_MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;

    bim_command_table!(tool_proofs);

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(BimCommandJobFactory::new(&controller))
    }

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::<semio_s_artifact_bim_model_gestures::editor::bim::gestures::GestureOwner>::default()
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !BIM_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id || bim_bounded_extent(&original.command).is_none() {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "Bim command does not match its exact registered tool").into());
        }
        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, BimCommand::command_id, BIM_BOUNDED_RAW_BYTES, BIM_BOUNDED_WORK_ITEMS, false, |request| -> Box<dyn ArtifactCommandWork<EditorApp<Self>>> { match &*request.command {
            BimCommand::ExportScheduleCsv(_) => Box::new(export_schedule_csv::ScheduleCsvWork::new(tool_id, request.instance_operation_owner.clone())),
            BimCommand::AnalyseModel(_) => Box::new(analyse_model::AnalyseWork::new(tool_id, request.instance_operation_owner.clone())),
            BimCommand::ExportModel(_) => Box::new(export_model::ModelExportWork::new(tool_id, request.instance_operation_owner.clone())),
            BimCommand::ExportSheets(_) => Box::new(export_sheets::SheetsExportWork::new(tool_id, request.instance_operation_owner.clone())),
            _ => Box::new(BimCommandWork::new(tool_id, request.instance_operation_owner.clone())),
        } })
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(app_schema_descriptor())
    }

    fn reference_birth()->Option<fn()->ModelSnapshot>{Some(||{
        semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
    })}

    fn io() -> Option<AppIo> {
        Some(bim_io())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        None
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::register(registry)?;
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::register(registry)?;
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::register(registry)?;
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::config::register(registry)?;
        semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        semio_s_artifact_bim_model_dispatch_context::editor::bim::transient::register(registry)
    }

    fn mounted_job_maintenance_demands(_instance_id: u32, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand::default())
    }

    fn mounted_job_maintenance_step(_instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,_grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure> {
        Ok(semio_framework_plugin::PluginLifecycleStep::Complete(semio_framework_value::retained_clone::RetainedCloneProgress::default()))
    }

    fn mounted_job_close_demands(_instance_id: u32, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand::default())
    }

    fn operation_progress_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![outliner_panel::BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    fn entity_label(snapshot: &ModelSnapshot, _kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        let name = kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id))?;
        Some(LocalizedLabel::native(&name, &name))
    }

    fn interaction_topology(doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
        Ok(semio_s_artifact_bim_model_interaction::editor::bim::interaction::topology(doc.snapshot))
    }

    fn command_id(command: &BimCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &BimCommand,
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ModelMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = BimDispatchCtx::new(selection_ids(interaction, semio_s_artifact_bim_model_interaction::editor::bim::interaction::BIM_ELEMENT_DOMAIN), selection_ids(interaction, semio_s_artifact_bim_model_interaction::editor::bim::interaction::BIM_LIBRARY_DOMAIN), view_state, cfg.window, None);
        command.dispatch(doc, cfg, &mut ctx)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(None, body_key, doc, cfg, view_state, &[], &[], &[], &Default::default())
    }

    fn render_with_instance_operation_owner(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, body_key: &str, doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(Some(owner), body_key, doc, cfg, view_state, &[], &[], &[], &Default::default())
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        use semio_s_artifact_bim_model_interaction::editor::bim::interaction::{BIM_ELEMENT_DOMAIN, BIM_LIBRARY_DOMAIN};
        let preview = semio_s_artifact_bim_model_storey_levels::editor::bim::marks::Preview::from_text(&semio_s_artifact_bim_model_dispatch_context::editor::bim::transient::current(transient).preview);
        render_body(Some(owner), body_key, doc, cfg, view_state, &selection_ids(interaction, BIM_ELEMENT_DOMAIN), &selection_ids(interaction, BIM_LIBRARY_DOMAIN), &hover_ids(interaction, BIM_ELEMENT_DOMAIN), &preview)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::engagements(doc.snapshot, view_state, semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), 0, &problems_of(doc))
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let selected = selection_ids(interaction, semio_s_artifact_bim_model_interaction::editor::bim::interaction::BIM_ELEMENT_DOMAIN).len();
        semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::engagements(doc.snapshot, view_state, semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), selected, &problems_of(doc))
    }

    fn window_measures(doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::measures(doc.snapshot, cfg, view_state)
    }
}
//#endregion 🔖️BimModelApp
