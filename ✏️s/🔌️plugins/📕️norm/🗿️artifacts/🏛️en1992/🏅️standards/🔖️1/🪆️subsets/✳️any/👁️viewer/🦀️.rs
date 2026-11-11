//! 👁️ EN 1992 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `En1992Viewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<En1992Viewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use crate::modes::view;
use crate::modes::view::windows::{checks, diagrams, viewport};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use semio_s_artifact_norm_en1992_model::{EN1992_DIALECT, EN1992_DOCUMENT_SCHEMA};
use semio_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, ViewEmit, Viewer};
use semio_plugin::app::{Dialect, InteractionView};
use semio_ui_locale::Label;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Command
semio_plugin::view_commands! {
    /// 👁️ `En1992Viewer::Command`: the sole dispatch surface of this read-only viewer, `viewportAct` (stepping, field, track, playback). `dispatch` returns `ViewEmit`, so no row can carry an artifact or draft mutation; the step itself only folds into the ephemeral local view lane (`ephemeral`).
    pub enum En1992ViewCommand for En1992Snapshot, NoConfig, NoConfigMutation {
        "viewportAct" as "viewport-act" => viewport_act::ViewportAct,
    }
}

semio_s_plugin_norm_app_surface::norm_viewer_commands!(En1992Snapshot);

semio_s_plugin_norm_app_surface::norm_owned_view_tool_job_factory!(En1992ViewBoundedCommandJobFactory, En1992Viewer);

semio_s_plugin_norm_app_surface::norm_viewer_surface!(En1992Viewer, En1992ViewCommand, variant = "en1992", report = |d: &En1992Snapshot| semio_s_artifact_norm_en1992_inferences::live::responsive(d, semio_s_plugin_norm_app_surface::PAINT_BUDGET).0, scene = |d: &En1992Snapshot, r: &semio_s_plugin_norm_app_surface::NormViewReport, _| semio_s_artifact_norm_en1992_scene::live_scene_of(d, r), diagrams = |d: &En1992Snapshot, r: &semio_s_plugin_norm_app_surface::NormViewReport, focus: Option<&str>| semio_s_plugin_norm_structural_scene::projection::memo_diagrams("en1992", d, r, focus.unwrap_or("\u{0}"), || semio_s_artifact_norm_en1992_scene::diagrams(d, r, focus)).to_vec());
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct En1992Viewer;

impl En1992Viewer {
    /// 🔒️ The explicit read-only marker of the viewer: it emits no artifact, config or draft mutation and projects the persisted placements only.
    pub const READ_ONLY: bool = true;
}

impl ArtifactViewer for En1992Viewer {
    /// 📢️ Localized notices of the refusal and warning codes this norm app raises.
    fn fault_notices() -> &'static [(&'static str, semio_ui_locale::LocalizedLabel)] {
        semio_s_plugin_norm_app_surface::norm_fault_notices()
    }
    type Snapshot = En1992Snapshot;
    type Mutation = semio_s_artifact_norm_en1992_mutations::En1992Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::lanes::NormLaneTransient;
    type TransientMutation = crate::lanes::NormLaneTransientMutation;
    type Command = En1992ViewCommand;

    const DIALECT: Dialect = EN1992_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = EN1992_DOCUMENT_SCHEMA;

    semio_s_plugin_norm_app_surface::norm_exact_viewer_store_ownership!(lanes);

    fn register_tool_job_factories(registry: &mut semio_plugin::ArtifactToolFactoryRegistry<'_, semio_plugin::ViewerApp<Self>>) -> Result<(), Fault> {
        En1992ViewBoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: &mut Option<semio_plugin::ArtifactOwnedToolJobRequest<semio_plugin::ViewerApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, _construction:&mut semio_plugin::__job::OriginalConstructionAdmission,cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        semio_s_plugin_norm_app_surface::build_norm_view_tool_job::<Self>(request, output, cx)
    }

    semio_plugin::bounded_first_step_tool_proofs! {
        owner: semio_plugin::ViewerApp<En1992Viewer>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.norm.en1992@1/*#viewer",
        artifact_schema: "semio.norm.en1992/v1",
        factory: "En1992ViewBoundedCommandJobFactory",
        factory_type: En1992ViewBoundedCommandJobFactory,
        contract: semio_s_plugin_norm_app_surface::norm_bounded_contract(),
        tools: ["viewportAct"]
    }

    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<semio_plugin::InteractionTopology, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_norm_app_surface::viewer_interaction_topology::<Self>(doc.snapshot))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.command_id()
    }

    semio_s_plugin_norm_app_surface::norm_view_command_from_action!(En1992ViewCommand);

    /// 🫧️ The ephemeral local view lane: stepping checks and combinations folds into this client's transient, never into the document.
    fn ephemeral(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &semio_plugin::PresenceView<'_, Self::Presence>,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        semio_s_plugin_norm_app_surface::viewer_ephemeral::<Self>(command, doc, transient)
    }

    fn reference_birth()->Option<fn()->Result<En1992Snapshot,semio_plugin::Fault>>{Some(||{
        Ok(En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    })}

    /// 👁️ Structurally read-only: `ViewEmit` cannot carry an artifact or draft mutation, and the one row (`viewportAct`) emits nothing at all; its effect is the ephemeral view lane.
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

    fn render_with_request_context(
        _owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_plugin::ViewModel,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        _cx: &mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        semio_s_plugin_norm_app_surface::render_norm_viewer::<Self>(body_key, doc, cfg, view_state, transient, interaction)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            viewport::BODY_VIEWPORT => viewport::render(doc.snapshot, view_state.locale).map(semio_plugin::built_to_component_tree),
            diagrams::BODY_DIAGRAMS => diagrams::render(doc.snapshot, &semio_plugin::TreeWindows::for_body(view_state, diagrams::BODY_DIAGRAMS), view_state.locale).map(semio_plugin::built_to_component_tree),
            checks::BODY_CHECKS => checks::render(doc.snapshot, &semio_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale).map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_en1992_viewer() -> semio_plugin::AppDefinition {
    Viewer::builder(EN1992_DIALECT)
        .document(["semio", "norm", "en1992"])
        .interaction(semio_s_plugin_norm_app_surface::viewer_interaction_definition("en1992"))
        .icon_id("check-circle")
        .mode_def(view::definition())
        .default_mode_id(semio_s_plugin_norm_app_surface::MODE_VIEW)
        .window_kind_def(viewport::definition())
        .window_kind_def(checks::definition())
        .window_kind_def(diagrams::definition())
        .default_layout(view::layout())
        .named_layout(semio_s_plugin_norm_app_surface::view_named_layout("en1992", "desktop"))
        .named_layout(semio_s_plugin_norm_app_surface::view_named_layout("en1992", "mobile"))
        .named_layout(semio_s_plugin_norm_app_surface::view_named_layout("en1992", "tablet"))
        .action_with(
            semio_plugin::ActionDefinition::new_catalog("viewportAct", semio_ui_locale::LocalizedLabel::native("Viewport Action", "Viewport-Aktion"), semio_plugin::ActionKind::View).with_args(vec![
                semio_plugin::ActionArgDef::text("action", semio_ui_locale::LocalizedLabel::native("Action", "Aktion")),
                semio_plugin::ActionArgDef::text("text", semio_ui_locale::LocalizedLabel::native("Text", "Text")),
                semio_plugin::ActionArgDef::any("number", semio_ui_locale::LocalizedLabel::native("Number", "Zahl")),
            ]),
        )
        .action_interactive_job("viewportAct", semio_plugin::InteractiveJobClassification::Migrated)
        .action_describe("viewportAct", semio_ui_locale::LocalizedLabel::native("Steps the checks and combinations or changes the colour field, track and playback of the viewport; only this client's ephemeral view changes, never the document.", "Schaltet Nachweise und Kombinationen weiter oder ändert Farbfeld, Verlauf und Wiedergabe des Viewports; nur die flüchtige Ansicht dieses Clients ändert sich, nie das Dokument."))
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
    semio_s_plugin_norm_app_surface::norm_viewer_lane_stores!(transient_extension = "norm.en1992.view.transient", owner = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🫧️lanes");
}
//#endregion 🪢️Mounts
