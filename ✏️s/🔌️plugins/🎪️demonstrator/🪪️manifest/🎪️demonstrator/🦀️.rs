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
use semio_s_artifact_gis_gismap::editor::gis2d::{create_gis2d_app, Gis2dPlayApp};
use semio_s_artifact_procedural_generation3d::editor::generation3d::{create_generation3d_app, Generation3dPlayApp};
use process::editor::process3d::{create_process3d_app, Process3dPlayApp};
use process::viewer::process3d::{create_process3d_viewer, Process3dViewer};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::{create_puzzle3d_app, Puzzle3dPlayApp};
use sourcing::editor::sourcing::{create_sourcing_curation_app, SourcingCurationApp};
use sourcing::viewer::sourcing::{create_sourcing_viewer, SourcingViewer};

const PLUGIN_ID: &str = "demonstrator";
const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
/// 🔢️ The version this bundle and every foreign plugin it composes are built at: all of them are members of one
/// workspace (`version.workspace = true`), and one tree is one catalog.
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

//#region 🔌️Plugin
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the demonstrator's owned and bundled surfaces — every variant
    /// spelled through `EditorSurfaceApp`/`ViewerSurfaceApp`, so each bundled app brings its OWN member
    /// roster (`ArtifactEditor::Members`) instead of this bundle guessing one.
    pub enum DemonstratorApps: PluginApp {
        PlaygroundEditor(EditorSurfaceApp<crate::editor::playground::PlaygroundEditor>),
        PlaygroundViewer(ViewerSurfaceApp<crate::viewer::playground::PlaygroundViewer>),
        Generation3dEditor(EditorSurfaceApp<Generation3dPlayApp>),
        CadEditor(EditorSurfaceApp<CadPlayApp>),
        Puzzle3dEditor(EditorSurfaceApp<Puzzle3dPlayApp>),
        SourcingEditor(EditorSurfaceApp<SourcingCurationApp>),
        SourcingViewer(ViewerSurfaceApp<SourcingViewer>),
        ProcessEditor(EditorSurfaceApp<Process3dPlayApp>),
        ProcessViewer(ViewerSurfaceApp<Process3dViewer>),
        GisEditor(EditorSurfaceApp<Gis2dPlayApp>),
    }
}

/// 📌️ Pins one composed plugin exactly at the version of the tree this bundle is compiled from: a trusted catalog
/// admits only exact dependency pins inside its own closure (`=x.y.z`, `trustedBootstrapDescriptorClaims`), never a range.
fn same_tree_pin() -> Result<VersionReq, PluginAssemblyError> {
    Version::parse(PLUGIN_VERSION).map(VersionReq::Exact).map_err(|error| PluginAssemblyError::new("plugin-assembly.dependency-version", format!("compiled workspace version is not semver: {error}")))
}

/// 🔌️ Builds the concrete demonstrator bundle: declares its owned playground artifact, registers
/// its own native editor+viewer surfaces over that artifact, then registers the six foreign plugins'
/// surfaces in their preserved order (`sourcing`/`process` each contribute an editor+viewer pair).
pub fn plugin() -> Result<Plugin<DemonstratorApps>, PluginAssemblyError> {
    Plugin::<DemonstratorApps>::builder(PLUGIN_ID)
        .label(PLUGIN_LABEL)
        .version(PLUGIN_VERSION)
        .package_id("semio:demonstrator")
        .depends_on("cad", same_tree_pin()?)
        .depends_on("gis", same_tree_pin()?)
        .depends_on("procedural", same_tree_pin()?)
        .depends_on("process", same_tree_pin()?)
        .depends_on("puzzle", same_tree_pin()?)
        .depends_on("sourcing", same_tree_pin()?)
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
