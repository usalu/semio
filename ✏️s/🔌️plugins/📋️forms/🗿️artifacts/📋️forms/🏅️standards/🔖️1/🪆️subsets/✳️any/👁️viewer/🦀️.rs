//! 👁️ Forms viewer — the read-only counterpart of the sibling editor surface for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `FormsViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<FormsViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::op::FormMutation;
use crate::schema::empty_forms_snapshot;
use crate::viewer::forms::modes::view;
use crate::viewer::forms::modes::view::windows::try_wizard;
use crate::{FormsSnapshot, FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA};
use semio_plugin::ArtifactView;
use semio_plugin::ArtifactViewer;
use semio_plugin::ConfigView;
use semio_plugin::Fault;
use semio_ui_locale::Label;
use crate::forms_config::{FormsConfig, FormsConfigMutation};
use semio_plugin::NoPresence;
use semio_plugin::NoPresenceMutation;
use semio_plugin::NoTransient;
use semio_plugin::NoTransientMutation;
use semio_plugin::ViewEmit;
use semio_plugin::Viewer;
// 🚧️ `Dialect`/`InteractionView` are only reachable through `app`, not yet in the crate-root
// re-export list (see the identical note in the sibling editor surface's root `🦀️.rs`).
use semio_plugin::app::{Dialect, InteractionView};
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Command
pub const FORMS_VIEW_APP_ID: &str = "s.forms.forms@1/*#viewer";
#[path = "🎮️commands/🧩️set-contributions/🦀️.rs"]
mod set_contributions;
#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;

semio_plugin::view_commands! {
    /// 👁️ The host projection changes configuration while the document remains read-only.
    pub enum FormsViewCommand for FormsSnapshot, FormsConfig, FormsConfigMutation {
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
    }
}

impl Default for FormsViewCommand {
    fn default() -> Self { Self::SetContributions(set_contributions::SetContributions { json: "[]".into() }) }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct FormsViewer;

impl ArtifactViewer for FormsViewer {
    type Snapshot = FormsSnapshot;
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Mutation = FormMutation;
    type Config = FormsConfig;
    type ConfigMutation = FormsConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = FormsViewCommand;

    const DIALECT: Dialect = FORMS_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::forms_child_restore_projection`).
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::forms_genesis_child_pack(snapshot, slot, child_id)
    
})())
}

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::forms_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;

    fn reference_birth()->Option<fn()->Result<FormsSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(empty_forms_snapshot())
    })}

    fn config_schema() -> &'static str { "forms.config" }

    fn command_id(command: &Self::Command) -> &'static str { command.command_id() }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        if action != "setContributions" { return Err(Fault::from("Forms viewer action is not declared")); }
        let payload = <set_contributions::SetContributions as semio_framework_value::FromValue>::from_value(args.take().ok_or_else(|| Fault::from("Forms viewer contribution arguments are absent"))?).map_err(|error| Fault::from(error.to_string()))?;
        Ok(FormsViewCommand::SetContributions(payload))
    }

    fn register_tool_job_factories(registry: &mut semio_plugin::ArtifactToolFactoryRegistry<'_, semio_plugin::ViewerApp<Self>>) -> Result<(), Fault> {
        registry.register(retained_commands::FormsViewContributionFactory::new(registry.controller_id()))
    }

    fn build_tool_job(request: &mut Option<semio_plugin::ArtifactOwnedToolJobRequest<semio_plugin::ViewerApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, _construction:&mut semio_framework_job::OriginalConstructionAdmission,cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        retained_commands::build(request, output, cx)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        retained_commands::proofs()
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            try_wizard::BODY_KEY => Ok(semio_plugin::built_to_component_tree(try_wizard::render(doc.snapshot, &semio_framework::parse_contributions(&cfg.snapshot.contributions_json), view_state)?)),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_forms_viewer() -> semio_plugin::AppDefinition {
    Viewer::builder(FORMS_DIALECT)
        .command(semio_plugin::CommandDefinition { in_palette: false, ..semio_plugin::CommandDefinition::bounded_catalog("setContributions", semio_ui_locale::LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", semio_plugin::ActionKind::View).with_args([semio_plugin::ActionArgDef::text("json", semio_ui_locale::LocalizedLabel::native("Contributions", "Beiträge"))]) })
        .action_interactive_job("setContributions", semio_plugin::InteractiveJobClassification::Migrated)
        .config(FormsViewer::config_spec()).document(["semio", "forms"]).icon_id("forms").mode_def(view::definition()).default_mode_id(view::FORMS_VIEW_MODE_VIEW).window_kind_def(try_wizard::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️integration/🦀️.rs"]
mod integration_tests;
//#endregion 🧪️Tests
