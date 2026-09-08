//! 🛂️ Demonstrator plugin manifest — its own `🎪️playground` editor/viewer surfaces (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) plus its six foreign plugins' surface
//! registrations (eight surfaces: four editor-only, two editor+viewer pairs).

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{Plugin, PluginApp};

// 🎫️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `cad`'s W2 packet dissolved
// `apps::cad` into `editor::cad`/`viewer::cad` (module path read off `cad`'s OWN
// `📦️packages/🦀️rust/🦀️.rs` `pub mod` nesting, never guessed from directory layout).
// `CadPlayApp` now implements `ArtifactEditor`, not `ArtifactApp`, and `create_cad_app()` returns
// `AppDefinition`, not `App` — see `.editor::<…>(…)` below.
use cad::editor::cad::{create_cad_app, CadPlayApp};
use gis::editor::gis2d::{create_gis2d_app, Gis2dPlayApp};
use procedural::editor::generation3d::{create_generation3d_app, Generation3dPlayApp};
use process::editor::process3d::{create_process3d_app, Process3dPlayApp};
use process::viewer::process3d::{create_process3d_viewer, Process3dViewer};
use puzzle::editor::puzzle3d::{create_puzzle3d_app, Puzzle3dPlayApp};
use sourcing::editor::sourcing::{create_sourcing_curation_app, SourcingCurationApp};
use sourcing::viewer::sourcing::{create_sourcing_viewer, SourcingViewer};

const PLUGIN_ID: &str = "demonstrator";
const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
const PLUGIN_VERSION: &str = "0.1.0";

//#region 🔌️Plugin
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the demonstrator's owned and bundled surfaces.
    pub enum DemonstratorApps: PluginApp {
        PlaygroundEditor(VcsArtifactApp<EditorApp<crate::editor::playground::PlaygroundEditor>>),
        PlaygroundViewer(VcsArtifactApp<ViewerApp<crate::viewer::playground::PlaygroundViewer>>),
        Generation3dEditor(VcsArtifactApp<EditorApp<Generation3dPlayApp>>),
        CadEditor(VcsArtifactApp<EditorApp<CadPlayApp>>),
        Puzzle3dEditor(VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>),
        SourcingEditor(VcsArtifactApp<EditorApp<SourcingCurationApp>>),
        SourcingViewer(VcsArtifactApp<ViewerApp<SourcingViewer>>),
        ProcessEditor(VcsArtifactApp<EditorApp<Process3dPlayApp>>),
        ProcessViewer(VcsArtifactApp<ViewerApp<Process3dViewer>>),
        GisEditor(VcsArtifactApp<EditorApp<Gis2dPlayApp>>),
    }
}

/// 🔌️ Builds the concrete demonstrator bundle: declares its owned playground artifact, registers
/// its own native editor+viewer surfaces over that artifact, then registers the six foreign plugins'
/// surfaces in their preserved order (`sourcing`/`process` each contribute an editor+viewer pair).
pub fn plugin() -> Result<Plugin<DemonstratorApps>, PluginAssemblyError> {
    Plugin::<DemonstratorApps>::builder(PLUGIN_ID)
        .label(PLUGIN_LABEL)
        .version(PLUGIN_VERSION)
        .package_id("semio:demonstrator")
        .depends_on("cad", VersionReq::Any)
        .depends_on("gis", VersionReq::Any)
        .depends_on("procedural", VersionReq::Any)
        .depends_on("process", VersionReq::Any)
        .depends_on("puzzle", VersionReq::Any)
        .depends_on("sourcing", VersionReq::Any)
        .artifact(crate::artifacts::playground::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<crate::editor::playground::PlaygroundEditor>(crate::editor::playground::create_playground_editor())
        .editor_mutation_roster::<crate::editor::playground::PlaygroundEditor>()
        .viewer::<crate::viewer::playground::PlaygroundViewer>(crate::viewer::playground::create_playground_viewer())
        .viewer_mutation_roster::<crate::viewer::playground::PlaygroundViewer>()
        .editor::<Generation3dPlayApp>(create_generation3d_app())
        .editor_mutation_roster::<Generation3dPlayApp>()
        .editor::<CadPlayApp>(create_cad_app())
        .editor_mutation_roster::<CadPlayApp>()
        .editor::<Puzzle3dPlayApp>(create_puzzle3d_app())
        .editor_mutation_roster::<Puzzle3dPlayApp>()
        .editor::<SourcingCurationApp>(create_sourcing_curation_app())
        .editor_mutation_roster::<SourcingCurationApp>()
        .viewer::<SourcingViewer>(create_sourcing_viewer())
        .viewer_mutation_roster::<SourcingViewer>()
        .editor::<Process3dPlayApp>(create_process3d_app())
        .editor_mutation_roster::<Process3dPlayApp>()
        .viewer::<Process3dViewer>(create_process3d_viewer())
        .viewer_mutation_roster::<Process3dViewer>()
        .editor::<Gis2dPlayApp>(create_gis2d_app())
        .editor_mutation_roster::<Gis2dPlayApp>()
        .try_build()
}
//#endregion 🔌️Plugin

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
