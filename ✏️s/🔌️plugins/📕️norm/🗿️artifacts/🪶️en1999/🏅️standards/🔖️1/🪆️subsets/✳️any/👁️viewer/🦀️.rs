//! 👁️ EN 1999 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `En1999Viewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<En1999Viewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use crate::modes::view;
use crate::modes::view::windows::{checks, diagrams, viewport};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_model::{EN1999_DIALECT, EN1999_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, ViewEmit, Viewer};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_ui_locale::Label;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Command
semio_framework_plugin::view_commands! {
    /// 👁️ `En1999Viewer::Command`: the sole dispatch surface of this read-only viewer, `viewportAct` (stepping, field, track, playback). `dispatch` returns `ViewEmit`, so no row can carry an artifact or draft mutation; the step itself only folds into the ephemeral local view lane (`ephemeral`).
    pub enum En1999ViewCommand for En1999Snapshot, NoConfig, NoConfigMutation {
        "viewportAct" as "viewport-act" => viewport_act::ViewportAct,
    }
}

semio_s_plugin_norm_app_surface::norm_viewer_commands!(En1999Snapshot);

semio_s_plugin_norm_app_surface::norm_owned_view_tool_job_factory!(En1999ViewBoundedCommandJobFactory, En1999Viewer);

semio_s_plugin_norm_app_surface::norm_viewer_surface!(En1999Viewer, En1999ViewCommand, report = |d: &En1999Snapshot| semio_s_artifact_norm_en1999_inferences::live::report(d), scene = |d: &En1999Snapshot, r: &semio_s_plugin_norm_app_surface::NormViewReport, _| semio_s_artifact_norm_en1999_scene::live_scene_of(d, r));
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct En1999Viewer;

impl En1999Viewer {
    /// 🔒️ The explicit read-only marker of the viewer: it emits no artifact, config or draft mutation and projects the persisted placements only.
    pub const READ_ONLY: bool = true;
}

impl ArtifactViewer for En1999Viewer {
    /// 📢️ Localized notices of the refusal and warning codes this norm app raises.
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
        semio_s_plugin_norm_app_surface::norm_fault_notices()
    }
    type Snapshot = En1999Snapshot;
    type Mutation = semio_s_artifact_norm_en1999_mutations::En1999Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::lanes::NormLaneTransient;
    type TransientMutation = crate::lanes::NormLaneTransientMutation;
    type Command = En1999ViewCommand;

    const DIALECT: Dialect = EN1999_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = EN1999_DOCUMENT_SCHEMA;

    semio_s_plugin_norm_app_surface::norm_exact_viewer_store_ownership!(lanes);

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::ViewerApp<Self>>) -> Result<(), Fault> {
        En1999ViewBoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::ViewerApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        semio_s_plugin_norm_app_surface::build_norm_view_tool_job::<Self>(request, output, cx)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::ViewerApp<En1999Viewer>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.norm.en1999@1/*#viewer",
        artifact_schema: "semio.norm.en1999/v1",
        factory: "En1999ViewBoundedCommandJobFactory",
        factory_type: En1999ViewBoundedCommandJobFactory,
        contract: semio_s_plugin_norm_app_surface::norm_bounded_contract(),
        tools: ["viewportAct"]
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.command_id()
    }

    semio_s_plugin_norm_app_surface::norm_view_command_from_action!(En1999ViewCommand);

    /// 🫧️ The ephemeral local view lane: stepping checks and combinations folds into this client's transient, never into the document.
    fn ephemeral(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &semio_framework_plugin::PresenceView<'_, Self::Presence>,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        semio_s_plugin_norm_app_surface::viewer_ephemeral::<Self>(command, doc, transient)
    }

    fn reference_birth()->Option<fn()->Result<En1999Snapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(En1999Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    })}

    /// 👁️ Structurally read-only: `ViewEmit` cannot carry an artifact or draft mutation, and the one row (`viewportAct`) emits nothing at all; its effect is the ephemeral view lane.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
        _cx: &mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let lane = semio_s_plugin_norm_app_surface::LaneTransient::lane(transient.snapshot).view;
        let rendered = match body_key {
            viewport::BODY_VIEWPORT => viewport::render_lane(doc.snapshot, &lane, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            checks::BODY_CHECKS => checks::render_selected(doc.snapshot, semio_s_plugin_norm_app_surface::view_selected_check(&lane), &semio_framework_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            _ => Self::render(body_key, doc, cfg, view_state),
        };
        rendered.map(Some).map_err(Into::into)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            viewport::BODY_VIEWPORT => viewport::render(doc.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            diagrams::BODY_DIAGRAMS => diagrams::render(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, diagrams::BODY_DIAGRAMS), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            checks::BODY_CHECKS => checks::render(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_en1999_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(EN1999_DIALECT)
        .document(["semio", "norm", "en1999"])
        .icon_id("check-circle")
        .mode_def(view::definition())
        .default_mode_id(semio_s_plugin_norm_app_surface::MODE_VIEW)
        .window_kind_def(viewport::definition())
        .window_kind_def(checks::definition())
        .window_kind_def(diagrams::definition())
        .default_layout(view::layout())
        .named_layout(semio_s_plugin_norm_app_surface::view_named_layout("en1999", "desktop"))
        .named_layout(semio_s_plugin_norm_app_surface::view_named_layout("en1999", "mobile"))
        .named_layout(semio_s_plugin_norm_app_surface::view_named_layout("en1999", "tablet"))
        .action_with(
            semio_framework_plugin::ActionDefinition::new_catalog("viewportAct", semio_framework_ui_locale::LocalizedLabel::native("Viewport Action", "Viewport-Aktion"), semio_framework_plugin::ActionKind::View).with_args(vec![
                semio_framework_plugin::ActionArgDef::text("action", semio_framework_ui_locale::LocalizedLabel::native("Action", "Aktion")),
                semio_framework_plugin::ActionArgDef::text("text", semio_framework_ui_locale::LocalizedLabel::native("Text", "Text")),
                semio_framework_plugin::ActionArgDef::any("number", semio_framework_ui_locale::LocalizedLabel::native("Number", "Zahl")),
            ]),
        )
        .action_interactive_job("viewportAct", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_describe("viewportAct", semio_framework_ui_locale::LocalizedLabel::native("Steps the checks and combinations or changes the colour field, track and playback of the viewport; only this client's ephemeral view changes, never the document.", "Schaltet Nachweise und Kombinationen weiter oder ändert Farbfeld, Verlauf und Wiedergabe des Viewports; nur die flüchtige Ansicht dieses Clients ändert sich, nie das Dokument."))
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🪢️Mounts
#[path = "."]
pub mod config {
    #[path = "🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod modes {
    #[path = "🎭️modes/👁️view/🦀️.rs"]
    pub mod view;
}
#[path = "."]
pub mod presence {
    #[path = "👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod lanes {
    semio_s_plugin_norm_app_surface::norm_viewer_lane_stores!(transient_extension = "norm.en1999.view.transient", owner = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🫧️lanes");
}
//#endregion 🪢️Mounts
