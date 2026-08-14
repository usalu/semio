//! 🎛️ Apps facet for `🎪️demonstrator` — the six entwerfen-mit-bestand surfaces (generator,
//! koordinator, aggregator, aussuchen, bearbeiten, verfolgen), each served by a FOREIGN plugin's
//! app: 🌀️procedural's `procedural3d-play`, 📐️cad's `cad-play`, 🧩️puzzle's `puzzle3d-play`,
//! 🪵️sourcing's `sourcing-curate`, 🏭️process's `process3d-play`, 🌍️gis's `gis2d-play`.
//!
//! Ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION D3 DISSOLVED the former
//! `🎪️panes/` facet into this one file rather than relocating it. The closed-shape policy row for
//! `🎪️panes/` proposed moving each pane into `🎛️apps/<app>/📌️panels/`, which is wrong here: those
//! `🎛️apps` belong to the six SOURCE plugins, and a `📌️panels/entwerfen-mit-bestand-*` under
//! 📐️cad or 🌍️gis would push demonstrator identity into plugins that must not carry it. Once D2
//! moved every foreign-kind IO registration to its owning plugin, each pane file was down to one
//! `register_document_codec_for_app` line plus one `register_document_app` line — six two-line
//! wrappers around a call this file can simply make itself.
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

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

/// 🎁️ Layers all six surfaces' host exports + apps onto `bundle` (the demonstrator root builds
/// `bundle` via `Plugin::builder(...).artifact(playground::declaration()).build()`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, then hands it here).
///
/// Export registration runs for all six BEFORE any app is registered, matching the order the
/// pre-consolidation bundle used: the OS mesh/solid/dwg registries are process-global, so a
/// surface's handlers must be in place before the host can resolve a document it hands back.
///
/// Every registration below now names a schema the demonstrator's own dependency actually exports
/// for its own app type — the foreign-KIND registrations (`"3d.cad"`, `"2d.map"`, `"3d.procedural"`,
/// `"3d.process"`) that used to sit here belong to 📐️cad / 🌍️gis / 🌀️procedural / 🏭️process and
/// are self-registered by those plugins (D2). 🧩️puzzle publishes its whole host-export set as one
/// `register_puzzle3d_exports()` entry point, so that one call stays a delegation.
pub fn bundle(bundle: Plugin) -> Plugin {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Procedural3dPlayApp>(PROCEDURAL_3D_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<CadPlayApp>(CAD_DOCUMENT_SCHEMA);
    register_puzzle3d_exports();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<SourcingCurateApp>(SOURCING_CURATE_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Process3dPlayApp>(PROCESS_3D_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Gis2dPlayApp>(GIS_MAP_SCHEMA);
    bundle
        .register_document_app::<Procedural3dPlayApp>(create_procedural3d_app())
        .register_document_app::<CadPlayApp>(create_cad_app())
        .register_document_app::<Puzzle3dPlayApp>(create_puzzle3d_app())
        .register_document_app::<SourcingCurateApp>(create_sourcing_curate_app())
        .register_document_app::<Process3dPlayApp>(create_process3d_app())
        .register_document_app::<Gis2dPlayApp>(create_gis2d_app())
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const PLUGIN_ID: &str = "demonstrator";
    const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
    const PLUGIN_VERSION: &str = "0.1.0";

    /// 🧪️ `bundle()` only layers the six surfaces onto a plugin it is handed — these tests exercise
    /// it in isolation with a bare `Plugin::new`, matching exactly the identity fields the real
    /// `crate::plugin()` passes through `Plugin::builder(...).build()`.
    fn test_bundle() -> Plugin {
        bundle(Plugin::new(PLUGIN_ID, PLUGIN_LABEL, PLUGIN_VERSION))
    }

    /// 🧪️ Bundle identity is the plugin's wire identity — the registry rows, playground variants and
    /// wasm component filename all key on `"demonstrator"`, so a rename here is a breaking change.
    #[test]
    fn bundle_keeps_its_plugin_identity() {
        let manifest = test_bundle().manifest;
        assert_eq!(manifest.plugin_id, PLUGIN_ID);
        assert_eq!(manifest.label, PLUGIN_LABEL);
        assert_eq!(manifest.version, PLUGIN_VERSION);
    }

    /// 🧪️ The six registered app ids, in registration order, are exactly the six `app = "…"` values
    /// the playground rows in `Cargo.toml` name. This is the demonstrator's whole observable surface:
    /// it owns no document schema, so this list — not a wire-format diff — is what the pane
    /// dissolution had to preserve byte-for-byte.
    #[test]
    fn bundle_registers_the_six_demonstrator_panes() {
        let ids: Vec<String> = test_bundle().manifest.apps.iter().map(|app| app.id.clone()).collect();
        assert_eq!(ids, vec!["procedural3d-play", "cad-play", "puzzle3d-play", "sourcing-curate", "process3d-play", "gis2d-play"]);
    }

    /// 🧪️ Every registered app declares at least one document schema, so the host can route a document
    /// to a surface — a surface whose codec registration was dropped would surface as an empty list here.
    #[test]
    fn every_pane_declares_a_document_schema() {
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
        let procedural = test_bundle().manifest.apps.into_iter().find(|app| app.id == "procedural3d-play").expect("procedural pane");
        let tick = procedural.commands.iter().find(|command| command.id == "flowEvalTick").expect("recursive evaluation command");
        assert!(!tick.in_palette);
        assert!(tick.args.is_empty());
    }
}
//#endregion 🧪️Tests
