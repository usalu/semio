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
use procedural::editor::procedural3d::{create_procedural3d_app, Procedural3dPlayApp};
use process::editor::process3d::{create_process3d_app, Process3dPlayApp};
use process::viewer::process3d::{create_process3d_viewer, Process3dViewer};
use puzzle::editor::puzzle3d::{create_puzzle3d_app, Puzzle3dPlayApp};
use sourcing::editor::sourcing::{create_sourcing_curation_app, SourcingCurationApp};
use sourcing::viewer::sourcing::{create_sourcing_viewer, SourcingViewer};

const PLUGIN_ID: &str = "demonstrator";
const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
const PLUGIN_VERSION: &str = "0.1.0";

//#region 🔌️Plugin
/// 🗃️ Closed runtime app fleet for the demonstrator's owned and bundled surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum DemonstratorApps: PluginApp {
        PlaygroundEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::playground::PlaygroundEditor>>),
        PlaygroundViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::playground::PlaygroundViewer>>),
        Procedural3dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Procedural3dPlayApp>>),
        CadEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<CadPlayApp>>),
        Puzzle3dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Puzzle3dPlayApp>>),
        SourcingEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<SourcingCurationApp>>),
        SourcingViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<SourcingViewer>>),
        ProcessEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Process3dPlayApp>>),
        ProcessViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<Process3dViewer>>),
        GisEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<Gis2dPlayApp>>),
    }
}

/// 🔌️ Builds the concrete demonstrator bundle: declares its owned playground artifact, registers
/// its own native editor+viewer surfaces over that artifact, then registers the six foreign plugins'
/// surfaces in their preserved order (`sourcing`/`process` each contribute an editor+viewer pair).
pub fn plugin() -> Result<Plugin<DemonstratorApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<DemonstratorApps>::builder(PLUGIN_ID)
        .label(PLUGIN_LABEL)
        .version(PLUGIN_VERSION)
        .artifact(crate::artifacts::playground::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::playground::PlaygroundEditor>(crate::editor::playground::create_playground_editor())
        .editor_mutation_roster::<crate::editor::playground::PlaygroundEditor>()
        .viewer::<crate::viewer::playground::PlaygroundViewer>(crate::viewer::playground::create_playground_viewer())
        .viewer_mutation_roster::<crate::viewer::playground::PlaygroundViewer>()
        .editor::<Procedural3dPlayApp>(create_procedural3d_app())
        .editor_mutation_roster::<Procedural3dPlayApp>()
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
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5's
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
    //! (W0-F gap 2) are used directly here — no local stand-ins, exercised against demonstrator's own
    //! `🎪️playground` editor/viewer pair (the six foreign plugins' own surfaces registered above
    //! belong to their own owning plugins' surface tests, not this one).
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn playground_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::playground::PlaygroundViewer>();
    }

    #[test]
    fn playground_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::playground::PlaygroundEditor, crate::viewer::playground::PlaygroundViewer>();
    }
}
//#endregion 🧪️SurfaceTests

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn test_bundle() -> Plugin<DemonstratorApps> {
        plugin().unwrap_or_else(|error| panic!("{error}"))
    }

    #[test]
    fn bundle_keeps_its_plugin_identity() {
        let manifest = test_bundle().manifest;
        assert_eq!(manifest.plugin_id, PLUGIN_ID);
        assert_eq!(manifest.label, PLUGIN_LABEL);
        assert_eq!(manifest.version, PLUGIN_VERSION);
    }

    /// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: playground's own two native
    /// surfaces are registered first (right after `.artifact(...)`), then the six foreign plugins'
    /// surfaces in their preserved order — gis's own W2 packet (P7) has landed, so `gis2d-play` is now
    /// `s.gis.gismap@1/*#editor` (coordinator follow-up, done once gis's own report confirmed the new
    /// module path, per that packet's ground truth). W4-FIX: `sourcing`/`process` were still on the
    /// deleted `apps::`/`document_app` path (`s.demonstrator.playground@1/*#editor`-style ids never
    /// applied to them) — now `.editor::<E>()` + `.viewer::<V>()` like every other foreign plugin here,
    /// so each contributes TWO surfaces instead of one (8 foreign surfaces total, from 6 plugins).
    #[test]
    fn bundle_registers_its_own_and_the_six_foreign_demonstrator_surfaces() {
        let ids: Vec<String> = test_bundle().manifest.apps.iter().map(|app| app.id.clone()).collect();
        assert_eq!(
            ids,
            vec![
                "s.demonstrator.playground@1/*#editor",
                "s.demonstrator.playground@1/*#viewer",
                "s.procedural.procedural3d@1/*#editor",
                "s.cad.cad@1/*#editor",
                "s.puzzle.puzzle3d@1/*#editor",
                "s.sourcing.curation@1/*#editor",
                "s.sourcing.curation@1/*#viewer",
                "s.process.process3d@1/*#editor",
                "s.process.process3d@1/*#viewer",
                "s.gis.gismap@1/*#editor",
            ]
        );
    }

    #[test]
    fn every_surface_declares_a_document_schema() {
        for app in test_bundle().manifest.apps {
            assert!(!app.io.document_schema.is_empty(), "app {} declares no document schema", app.id);
        }
    }

    #[test]
    fn contribution_consumers_declare_the_hidden_app_command() {
        let consumers: Vec<String> = test_bundle().manifest.apps.iter().filter(|app| app.commands.iter().any(|command| command.id == "setContributions")).map(|app| app.id.clone()).collect();
        assert_eq!(consumers, vec!["s.cad.cad@1/*#editor", "s.sourcing.curation@1/*#editor", "s.process.process3d@1/*#editor"]);
        for app in test_bundle().manifest.apps {
            if let Some(command) = app.commands.iter().find(|command| command.id == "setContributions") {
                assert!(!command.in_palette, "host catalogue command leaked into {}'s palette", app.id);
                assert_eq!(command.args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), vec!["json"]);
                assert_eq!(command.semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "host catalogue command in {} must be admitted as migrated interactive work", app.id);
            }
        }
        let procedural = test_bundle().manifest.apps.into_iter().find(|app| app.id == "s.procedural.procedural3d@1/*#editor").expect("procedural surface");
        let tick = procedural.commands.iter().find(|command| command.id == "flowEvalTick").expect("recursive evaluation command");
        assert!(!tick.in_palette);
        assert!(tick.args.is_empty());
    }

    fn assert_tree_reconciles(tree: semio_framework_ui_runtime::ComponentTree, generation: u64, label: &str) {
        assert!(!tree.root.key.is_empty(), "{label} must contain an authored root");
        let mut producer = semio_framework_ui_runtime::ComponentTreeProducer::try_new(tree.root, generation).expect("nonzero aggregate tree generation");
        for _ in 0..65_536 {
            match producer.step(generation, false, false) {
                semio_framework_ui_runtime::ComponentTreeProducerStep::MoreWork => {}
                semio_framework_ui_runtime::ComponentTreeProducerStep::Complete => {
                    assert!(producer.take_complete().is_some(), "completed {label} tree transfers its exact owner");
                    return;
                }
                semio_framework_ui_runtime::ComponentTreeProducerStep::Fault(fault) => panic!("{label} tree must enter retained reconciliation: {fault:?}"),
            }
        }
        panic!("{label} tree producer did not settle within its fixed bound");
    }

    #[semio_framework_async_macros::async_test]
    async fn aggregate_runtime_renders_every_demonstrator_window() {
        let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::<DemonstratorApps>::new();
        semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, test_bundle());
        let apps: &[(&str, &[&str])] = &[
            ("s.procedural.procedural3d@1/*#editor", &["procedural.play.main", "procedural.play.preview", "procedural.play.generations", "procedural.play.generate-form", "procedural.play.generate-preview"]),
            ("s.cad.cad@1/*#editor", &["cad.play.shape", "cad.play.building", "cad.play.energy", "cad.play.structure-classic"]),
            ("s.puzzle.puzzle3d@1/*#editor", &["puzzle3d.play.composite"]),
            ("s.sourcing.curation@1/*#editor", &["sourcing.pool", "sourcing.curated", "sourcing.preview", "sourcing.grid"]),
            ("s.process.process3d@1/*#editor", &["process.play.main"]),
            ("s.gis.gismap@1/*#editor", &["gis2d.play.composite"]),
        ];
        let mut generation = 1_u64;
        for (app_index, (app_id, body_keys)) in apps.iter().enumerate() {
            let instance_id = u32::try_from(app_index + 1).expect("six aggregate app instances");
            semio_framework_plugin::plugin_runtime::plugin_create_app_with_id(&runtime, instance_id, app_id)
                .await
                .unwrap_or_else(|fault| panic!("aggregate app {app_id} opens: {fault:?}"));
            for body_key in *body_keys {
                let tree = semio_framework_plugin::plugin_runtime::plugin_render(&runtime, instance_id, body_key, "{}")
                    .await
                    .unwrap_or_else(|fault| panic!("aggregate body {body_key} renders: {fault:?}"));
                assert_tree_reconciles(tree, generation, body_key);
                generation += 1;
            }
        }
    }
}
//#endregion 🧪️Tests
