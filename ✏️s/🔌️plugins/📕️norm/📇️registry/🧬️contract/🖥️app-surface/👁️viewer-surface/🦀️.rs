//! 👁️ Viewer surface: the ephemeral-local view lane of a read-only norm viewer. A viewer keeps a transient store of its own (the same [`NormTransient`] lane as the editors, never persisted, never shared) and folds the `viewportAct` command into it, so stepping checks and combinations works without any document, config or draft write.
//!
//! 🔗️ Steps: [`step_view`](crate::step_view); keys: [`shortcut_view_act`](crate::shortcut_view_act); editor twin: [`live_ephemeral`](crate::live_ephemeral).

use super::*;
use semio_s_plugin_norm_structural_scene::projection::ViewOptions;
use semio_s_plugin_norm_structural_scene::StructuralScene;

/// 📋️ The settled check report a viewer steps over, as the shared handle of the live session (stepping copies nothing).
pub type NormViewReport = std::sync::Arc<CheckReport>;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🧾️ The one tool id of a norm viewer.
pub const NORM_VIEWER_TOOL_IDS: &[&str] = &["viewportAct"];

/// 🚦️ The viewer's only publication lane is the ephemeral local transient.
pub const NORM_VIEWER_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "viewportAct", lanes: &[ArtifactToolPublicationLane::Transient] }];

/// 🧵️ What a norm viewer states to keep its view lane: how it settles its report, projects its scene and reads a `viewportAct` command.
pub trait NormViewerSurface: semio_framework_plugin::ArtifactViewer<Config = NoConfig, ConfigMutation = NoConfigMutation> {
    const VARIANT: &'static str;
    const CONTROLLER: &'static str;
    const INTERACTION_DOMAIN: &'static str;
    const VIEWPORT_BODY: &'static str;
    const CHECKS_BODY: &'static str;
    const DIAGRAMS_BODY: &'static str;
    fn view_report(document: &Self::Snapshot) -> NormViewReport;
    fn view_scene(document: &Self::Snapshot, report: &NormViewReport, view: &ViewOptions) -> StructuralScene;
    fn view_diagrams(document: &Self::Snapshot, report: &NormViewReport, focus: Option<&str>) -> Vec<semio_s_plugin_norm_structural_scene::Diagram2d>;
    fn view_lane(transient: &Self::Transient) -> ViewLane;
    fn lane_event(command: &Self::Command) -> Option<LaneEvent>;
}

/// 🕹️ The readonly scene exposes the same entity and check selection vocabulary as the editor, with no edit tools.
pub fn viewer_interaction_definition(variant: &str) -> semio_framework_plugin::InteractionDefinition {
    use semio_s_plugin_norm_structural_scene::EntityKind;
    crate::viewport_interaction_definition(variant, semio_framework_ui_locale::LocalizedLabel::native("Model", "Modell"), &[EntityKind::Member, EntityKind::Plate, EntityKind::Shell, EntityKind::Solid, EntityKind::Fastener, EntityKind::Support, EntityKind::Load, EntityKind::Surface, EntityKind::Group])
}

/// 🗺️ Keeps readonly viewport and check addresses admitted when the framework revalidates its interaction state.
pub fn viewer_interaction_topology<V: NormViewerSurface>(document: &V::Snapshot) -> semio_framework_plugin::InteractionTopology
where
    V::Snapshot: semio_framework_value::ToValue,
{
    let report = V::view_report(document);
    let scene = V::view_scene(document, &report, ViewOptions::none());
    crate::norm_interaction_topology(V::INTERACTION_DOMAIN, &semio_framework_value::ToValue::to_value(document), &scene, &report)
}

/// 🪟️ One readonly projection shared by the model, checks and diagrams of a viewer.
pub struct NormViewerFrame {
    pub report: NormViewReport,
    pub selection: NormSelection,
    pub lane: ViewLane,
}

impl NormViewerFrame {
    pub fn new(report: NormViewReport, lane: ViewLane, picked: &[String]) -> Self {
        let selection = NormSelection::from_interaction(&report, lane.check, picked);
        Self { report, selection, lane }
    }

    /// 🎯️ The selected document owner, or the governing owner while nothing is selected.
    pub fn diagram_focus(&self) -> Option<String> {
        self.selection.primary(&self.report).or_else(|| self.report.checks.iter().filter(|check| check.status != CheckStatus::NotApplicable && !check.subject.entity_id.is_empty()).max_by(|a, b| a.utilization.total_cmp(&b.utilization)).map(|check| check.subject.entity_id.clone())).map(|id| semio_s_plugin_norm_structural_scene::projection_kit::base_id(&id).to_owned())
    }

    pub fn render_viewport(&self, body: &str, scene: &StructuralScene, domain: &'static str, controller: &'static str, locale: semio_framework_ui_locale::Locale) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
        let selected = self.selection.highlights(scene.entities.iter().map(|entity| (entity.entity.kind, entity.entity.id.as_str())));
        crate::toolbar::render_viewport_panel(body, scene, &crate::toolbar::ViewportView { lane: self.lane.clone(), ..Default::default() }, &selected, domain, controller, locale)
    }

    pub fn render_checks(&self, windows: &semio_framework_plugin::TreeWindows<'_>, domain: &'static str, controller: &'static str, locale: semio_framework_ui_locale::Locale) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
        crate::render_checks_with(&self.report, &self.selection.checks_state(&self.report, ChecksViewState::default()), &ChecksWindowSpec { controller_id: Some(controller), interaction_domain: Some(domain), ..Default::default() }, windows, locale)
    }

    pub fn render_diagrams(&self, body: &str, diagrams: &[semio_s_plugin_norm_structural_scene::Diagram2d], windows: &semio_framework_plugin::TreeWindows<'_>, locale: semio_framework_ui_locale::Locale) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
        crate::diagrams2d::render_diagrams(body, diagrams, &crate::diagrams2d::DiagramsWindowSpec::default(), windows, locale)
    }
}

/// 👁️ Paints every readonly Eurocode window from the same report, interaction selection and local view lane.
pub fn render_norm_viewer<V: NormViewerSurface>(body_key: &str, doc: &ArtifactView<'_, V::Snapshot>, cfg: &ConfigView<'_, V::Config>, view: &semio_framework_plugin::ViewModel, transient: &semio_framework_plugin::TransientView<'_, V::Transient>, interaction: &semio_framework_plugin::app::InteractionView<'_>) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
    let frame = NormViewerFrame::new(V::view_report(doc.snapshot), V::view_lane(transient.snapshot), &interaction.selection(V::INTERACTION_DOMAIN).ids);
    let rendered = match body_key {
        key if key == V::VIEWPORT_BODY => frame.render_viewport(body_key, &V::view_scene(doc.snapshot, &frame.report, &view_options_of(&frame.lane)), V::INTERACTION_DOMAIN, V::CONTROLLER, view.locale),
        key if key == V::CHECKS_BODY => frame.render_checks(&semio_framework_plugin::TreeWindows::for_body(view, body_key), V::INTERACTION_DOMAIN, V::CONTROLLER, view.locale),
        key if key == V::DIAGRAMS_BODY => frame.render_diagrams(body_key, &V::view_diagrams(doc.snapshot, &frame.report, frame.diagram_focus().as_deref()), &semio_framework_plugin::TreeWindows::for_body(view, body_key), view.locale),
        _ => return V::render(body_key, doc, cfg, view).map(Some).map_err(Into::into),
    };
    rendered.map(|node| Some(semio_framework_plugin::built_to_component_tree(node))).map_err(Into::into)
}

/// 🫧️ The lane mutations of one view event: a stepping intent resolves against the report and the scene (projected only for a combination step), every other event folds directly; no mutation when the lane does not change.
pub fn view_lane_mutations<T: LaneTransient>(event: LaneEvent, lane: &T, report: impl FnOnce() -> NormViewReport, scene: impl Fn(&NormViewReport, &ViewLane) -> StructuralScene) -> Vec<T::Mutation> {
    let current = lane.lane();
    let events = match event {
        LaneEvent::ViewStep { .. } => {
            let report = report();
            resolve_view_steps(vec![event], &report, || scene(&report, &current.view), &current.view)
        }
        other => vec![other],
    };
    let next = events.into_iter().fold(current.clone(), NormTransient::apply);
    if next == current { Vec::new() } else { vec![T::replace(&next)] }
}

/// 🫧️ `ArtifactViewer::ephemeral` of every norm viewer: the transient mutations of the command, none for presence; the document is only read.
pub fn viewer_ephemeral<V: NormViewerSurface>(command: &V::Command, doc: &ArtifactView<'_, V::Snapshot>, transient: &semio_framework_plugin::TransientView<'_, V::Transient>) -> (Vec<V::PresenceMutation>, Vec<V::TransientMutation>)
where
    V::Transient: LaneTransient<Mutation = V::TransientMutation>,
{
    let Some(event) = V::lane_event(command) else { return (Vec::new(), Vec::new()) };
    (Vec::new(), view_lane_mutations(event, transient.snapshot, || V::view_report(doc.snapshot), |report, lane| V::view_scene(doc.snapshot, report, &view_options_of(lane))))
}

/// 🔎️ The check the view selected, if the report has it.
pub fn view_selected_check(lane: &ViewLane) -> Option<usize> {
    lane.check.map(|index| index as usize)
}

//#region 🧵️RetainedCommands
/// 🧵️ The retained reducer of a viewer: `viewportAct` writes nothing but the transient lane, so the emission is empty.
#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub fn norm_view_reduce<V: NormViewerSurface>(
    command: &V::Command,
    snapshot: &V::Snapshot,
    _config: &V::Config,
    _history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::ViewerApp<V>>>,
    _operation: &semio_framework_plugin::AppOperationContext,
) -> NormRetainedCommandResult<V::Mutation> {
    if !NORM_VIEWER_TOOL_IDS.contains(&V::command_id(command)) {
        return Err(Fault::from("norm-command-retained-route-rejected"));
    }
    let mut emit = Emit::default();
    if let (Some(context), Some(event)) = (context, V::lane_event(command)) {
        let report = V::view_report(snapshot);
        let lane = V::view_lane(context.transient.get());
        let picked = interaction.selection.get(V::INTERACTION_DOMAIN).map(|selection| selection.ids.as_slice()).unwrap_or_default();
        emit.effects.extend(crate::check_step_selection_effect(V::INTERACTION_DOMAIN, &report, &lane, picked, event, || V::view_scene(snapshot, &report, &view_options_of(&lane))));
    }
    Ok(emit)
}

/// 📏️ The viewer command is one bounded step.
pub fn norm_view_extent<V: NormViewerSurface>(_command: &V::Command, _snapshot: &V::Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 🏭️ The one owned bounded tool-job factory serving all norm viewers, generic over the viewer.
pub struct NormViewBoundedCommandJobFactory<V: NormViewerSurface> {
    keys: Vec<semio_framework::ToolFactoryKey>,
    owner: std::marker::PhantomData<fn() -> V>,
}

impl<V: NormViewerSurface> NormViewBoundedCommandJobFactory<V> {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: NORM_VIEWER_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect(), owner: std::marker::PhantomData }
    }
}

impl<V: NormViewerSurface> semio_framework::ToolJobFactory for NormViewBoundedCommandJobFactory<V> {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::ViewerApp<V>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::ViewerApp<V>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        NORM_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
        semio_framework_plugin::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        norm_bounded_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: NORM_RETAINED_RAW_BYTES, checkpoint_resume: true);
}

impl<V: NormViewerSurface> semio_framework_plugin::ArtifactOwnedToolJobFactory for NormViewBoundedCommandJobFactory<V> {
    type Owner = semio_framework_plugin::ViewerApp<V>;
    const TOOL_IDS: &'static [&'static str] = NORM_VIEWER_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = <V as semio_framework_plugin::ArtifactViewer>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = NORM_VIEWER_PUBLICATION_CONTRACTS;
}

/// 🧵️ `ArtifactViewer::build_tool_job` for every norm viewer.
pub fn build_norm_view_tool_job<V: NormViewerSurface>(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::ViewerApp<V>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "norm-command-retained-producer-request-absent"))?;
    if !NORM_VIEWER_TOOL_IDS.contains(&original.tool_id.as_str()) {
        return Ok(false);
    }
    let tool_id = V::command_id(&original.command);
    if tool_id != original.tool_id {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "norm-command-tool-mismatch").into());
    }
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, V::command_id, NORM_RETAINED_RAW_BYTES, 1, true, |request| -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::ViewerApp<V>>> { Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, norm_view_reduce::<V>, norm_view_extent::<V>)) })
}
//#endregion 🧵️RetainedCommands

//#region 🏭️Macros
/// 🏭️ Declares one norm viewer's owned factory as a newtype over [`NormViewBoundedCommandJobFactory`] (the proof joins the `factory:` literal against the type name) with its `register` entry point.
#[macro_export]
macro_rules! norm_owned_view_tool_job_factory {
    ($factory:ident, $viewer:ty) => {
        pub struct $factory($crate::NormViewBoundedCommandJobFactory<$viewer>);

        impl $factory {
            pub fn new(controller_id: &str) -> Self {
                Self($crate::NormViewBoundedCommandJobFactory::<$viewer>::new(controller_id))
            }

            pub fn register(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::ViewerApp<$viewer>>) -> Result<(), semio_framework_plugin::Fault> {
                let controller = registry.controller_id().to_string();
                registry.register(Self::new(&controller))
            }
        }

        impl semio_framework::ToolJobFactory for $factory {
            type Payload = <$crate::NormViewBoundedCommandJobFactory<$viewer> as semio_framework::ToolJobFactory>::Payload;
            type Job = <$crate::NormViewBoundedCommandJobFactory<$viewer> as semio_framework::ToolJobFactory>::Job;

            fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
                semio_framework::ToolJobFactory::keys(&self.0)
            }

            fn payload_schema_id(&self) -> &str {
                semio_framework::ToolJobFactory::payload_schema_id(&self.0)
            }

            fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
                semio_framework::ToolJobFactory::classification(&self.0)
            }

            fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
                semio_framework::ToolJobFactory::execution_contract(&self.0)
            }

            fn admission_demands(&self, operation: &semio_framework_job::Operation, payload: &Self::Payload, input: Option<&semio_framework_action_bus::RetainedToolWireInput>, checkpoint: Option<&semio_framework_action_bus::RetainedToolWireInput>) -> Result<semio_framework_action_bus::ToolJobAdmissionDemand, semio_framework_value::ValueError> {
                semio_framework::ToolJobFactory::admission_demands(&self.0, operation, payload, input, checkpoint)
            }

            fn admit_job(&mut self, context: &mut semio_framework_job::StepContext<'_>, operation: &semio_framework_job::Operation, payload: &mut Option<Box<Self::Payload>>, input: &mut Option<semio_framework_action_bus::RetainedToolWireInput>, checkpoint: &mut Option<semio_framework_action_bus::RetainedToolWireInput>, output: &mut Option<Box<Self::Job>>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
                semio_framework::ToolJobFactory::admit_job(&mut self.0, context, operation, payload, input, checkpoint, output)
            }
        }

        impl semio_framework_plugin::ArtifactOwnedToolJobFactory for $factory {
            type Owner = semio_framework_plugin::ViewerApp<$viewer>;
            const TOOL_IDS: &'static [&'static str] = $crate::NORM_VIEWER_TOOL_IDS;
            const DOCUMENT_SCHEMA: &'static str = <$viewer as semio_framework_plugin::ArtifactViewer>::DOCUMENT_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = $crate::NORM_VIEWER_PUBLICATION_CONTRACTS;
        }
    };
}

/// 🧱️ Declares the `viewport_act` command module of a viewer (the payload and its read-only `handle`) in the invoking module, for the `view_commands!` row `"viewportAct" as "viewport-act" => viewport_act::ViewportAct`.
#[macro_export]
macro_rules! norm_viewer_commands {
    ($snapshot:ty) => {
        pub mod viewport_act {
            #[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
            #[dsl(keyword = "viewport-act")]
            pub struct ViewportAct {
                pub action: String,
                pub text: Option<String>,
                pub number: Option<f64>,
            }
            pub fn handle(_payload: &ViewportAct, _doc: &semio_framework_plugin::ArtifactView<'_, $snapshot>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Result<semio_framework_plugin::ViewEmit<semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault> {
                Ok(semio_framework_plugin::ViewEmit::default())
            }
        }
    };
}

/// 🌉️ Installs the `{action, args}` to `ViewportAct` bridge of a viewer: the `viewportAct` verb with its arguments and every `norm.shortcut.<step>` id.
#[macro_export]
macro_rules! norm_view_command_from_action {
    ($command:ident) => {
        fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework_plugin::Fault>) -> Result<$command, semio_framework_plugin::Fault> {
            if let Some(fault) = original_fault.take() {
                return Err(fault);
            }
            let args = args.as_ref();
            match action {
                "viewportAct" => Ok($command::ViewportAct(viewport_act::ViewportAct {
                    action: args.and_then(|value| value.get("action")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string(),
                    text: args.and_then(|value| value.get("text")).and_then(semio_framework_value::DslValue::as_str).map(str::to_string),
                    number: args.and_then(|value| value.get("number")).and_then(semio_framework_value::DslValue::as_f64),
                })),
                other => match $crate::shortcut_view_act(other) {
                    Some((action, direction)) => Ok($command::ViewportAct(viewport_act::ViewportAct { action: action.to_string(), text: None, number: Some(direction) })),
                    None => Err(semio_framework_plugin::Fault::new(
                        semio_framework_plugin::FaultOrigin::App,
                        semio_framework_plugin::FaultCode::new("norm.unhandled-action"),
                        format!("action '{other}' is not one of this viewer's declared verbs (viewportAct/norm.shortcut.<step>)"),
                    )),
                },
            }
        }
    };
}

/// 🧩️ Implements [`NormViewerSurface`] for a viewer from its report, scene and command enum (row `ViewportAct`).
#[macro_export]
macro_rules! norm_viewer_surface {
    ($viewer:ty, $command:ident, variant = $variant:literal, report = $report:expr, scene = $scene:expr, diagrams = $diagrams:expr) => {
        impl $crate::NormViewerSurface for $viewer {
            const VARIANT: &'static str = $variant;
            const CONTROLLER: &'static str = concat!("s.norm.", $variant, "@1/*#viewer");
            const INTERACTION_DOMAIN: &'static str = concat!("norm.", $variant);
            const VIEWPORT_BODY: &'static str = concat!("norm.", $variant, ".view.viewport");
            const CHECKS_BODY: &'static str = concat!("norm.", $variant, ".view.checks");
            const DIAGRAMS_BODY: &'static str = concat!("norm.", $variant, ".view.diagrams");

            fn view_report(document: &<$viewer as semio_framework_plugin::ArtifactViewer>::Snapshot) -> $crate::NormViewReport {
                ($report)(document)
            }

            fn view_scene(document: &<$viewer as semio_framework_plugin::ArtifactViewer>::Snapshot, report: &$crate::NormViewReport, view: &semio_s_plugin_norm_structural_scene::projection::ViewOptions) -> semio_s_plugin_norm_structural_scene::StructuralScene {
                ($scene)(document, report, view)
            }

            fn view_diagrams(document: &<$viewer as semio_framework_plugin::ArtifactViewer>::Snapshot, report: &$crate::NormViewReport, focus: Option<&str>) -> Vec<semio_s_plugin_norm_structural_scene::Diagram2d> {
                ($diagrams)(document, report, focus)
            }

            fn view_lane(transient: &<$viewer as semio_framework_plugin::ArtifactViewer>::Transient) -> $crate::ViewLane {
                $crate::LaneTransient::lane(transient).view
            }

            fn lane_event(command: &$command) -> Option<$crate::LaneEvent> {
                match command {
                    $command::ViewportAct(p) => $crate::view_event(&p.action, p.text.as_deref(), p.number),
                }
            }
        }
    };
}
//#endregion 🏭️Macros
