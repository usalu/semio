//! 🛂️ Demonstrator plugin manifest and its six foreign application registrations.

use semio_framework_plugin::Plugin;

use cad::apps::cad::{create_cad_app, CadPlayApp};
use cad::artifacts::cad::CAD_DOCUMENT_SCHEMA;
use gis::apps::gis2d::{create_gis2d_app, Gis2dPlayApp};
use gis::artifacts::gismap::GIS_MAP_SCHEMA;
use procedural::apps::procedural3d::{create_procedural3d_app, Procedural3dPlayApp};
use procedural::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA;
use process::apps::process3d::{create_process3d_app, Process3dPlayApp};
use process::artifacts::process3d::PROCESS_3D_SCHEMA;
use puzzle::apps::puzzle3d::{create_puzzle3d_app, register_puzzle3d_exports, Puzzle3dPlayApp};
use sourcing::apps::curate::{create_sourcing_curate_app, SourcingCurateApp};
use sourcing::artifacts::curate::SOURCING_CURATE_SCHEMA;

const PLUGIN_ID: &str = "demonstrator";
const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
const PLUGIN_VERSION: &str = "0.1.0";

//#region 🔌️Plugin
/// 🔌️ Builds the concrete demonstrator bundle, declaring its owned playground artifact before
/// registering the six foreign document surfaces in their preserved order.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    assemble(
        Plugin::builder(PLUGIN_ID)
            .label(PLUGIN_LABEL)
            .version(PLUGIN_VERSION)
            .artifact(crate::artifacts::playground::declaration())
            .build(),
    )
}

fn assemble(bundle: Plugin) -> Plugin {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Procedural3dPlayApp>(PROCEDURAL_3D_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<CadPlayApp>(CAD_DOCUMENT_SCHEMA);
    register_puzzle3d_exports();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<SourcingCurateApp>(SOURCING_CURATE_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Process3dPlayApp>(PROCESS_3D_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Gis2dPlayApp>(GIS_MAP_SCHEMA);
    bundle
        .document_app::<Procedural3dPlayApp>(create_procedural3d_app())
        .document_app::<CadPlayApp>(create_cad_app())
        .document_app::<Puzzle3dPlayApp>(create_puzzle3d_app())
        .document_app::<SourcingCurateApp>(create_sourcing_curate_app())
        .document_app::<Process3dPlayApp>(create_process3d_app())
        .document_app::<Gis2dPlayApp>(create_gis2d_app())
}
//#endregion 🔌️Plugin

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn test_bundle() -> Plugin {
        assemble(Plugin::new(PLUGIN_ID, PLUGIN_LABEL, PLUGIN_VERSION))
    }

    #[test]
    fn bundle_keeps_its_plugin_identity() {
        let manifest = test_bundle().manifest;
        assert_eq!(manifest.plugin_id, PLUGIN_ID);
        assert_eq!(manifest.label, PLUGIN_LABEL);
        assert_eq!(manifest.version, PLUGIN_VERSION);
    }

    #[test]
    fn bundle_registers_the_six_demonstrator_surfaces() {
        let ids: Vec<String> = test_bundle().manifest.apps.iter().map(|app| app.id.clone()).collect();
        assert_eq!(ids, vec!["procedural3d-play", "cad-play", "puzzle3d-play", "sourcing-curate", "process3d-play", "gis2d-play"]);
    }

    #[test]
    fn every_surface_declares_a_document_schema() {
        for app in test_bundle().manifest.apps {
            assert!(!app.io.document_schema.is_empty(), "app {} declares no document schema", app.id);
        }
    }

    #[test]
    fn contribution_consumers_declare_the_hidden_app_command() {
        let consumers: Vec<String> = test_bundle()
            .manifest
            .apps
            .iter()
            .filter(|app| app.commands.iter().any(|command| command.id == "setContributions"))
            .map(|app| app.id.clone())
            .collect();
        assert_eq!(consumers, vec!["procedural3d-play", "cad-play", "sourcing-curate", "process3d-play"]);
        for app in test_bundle().manifest.apps {
            if let Some(command) = app.commands.iter().find(|command| command.id == "setContributions") {
                assert!(!command.in_palette, "host catalogue command leaked into {}'s palette", app.id);
                assert_eq!(command.args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), vec!["json"]);
            }
        }
        let procedural = test_bundle().manifest.apps.into_iter().find(|app| app.id == "procedural3d-play").expect("procedural surface");
        let tick = procedural.commands.iter().find(|command| command.id == "flowEvalTick").expect("recursive evaluation command");
        assert!(!tick.in_palette);
        assert!(tick.args.is_empty());
    }
}
//#endregion 🧪️Tests
