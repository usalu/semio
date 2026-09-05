//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, HostMediaHandlerDeclaration, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for both GIS artifact surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum GisApps: PluginApp {
        Gis2dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::gis2d::Gis2dPlayApp>>),
        GisMapViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::gismap::GisMapViewer>>),
        Gis3dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::gis3d::Gis3dPlayApp>>),
        GisTerrainViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::gisterrain::GisTerrainViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) declares both owned artifacts (`gismap`,
/// `gisterrain`); `Gis2dPlayApp::app_schema()`/`Gis3dPlayApp::app_schema()` are registered
/// automatically by each `.editor()` call below. `.editor()`/`.viewer()` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the retired `.document_app()` — each
/// subset now registers an independent editor and viewer surface.
pub fn plugin() -> Result<Plugin<GisApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<GisApps>::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .package_id("semio:gis")
        .artifact_kind(crate::artifacts::gismap::artifact_kind())
        .artifact(crate::artifacts::gismap::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::gisterrain::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::two_d_svg_export(
            "s.gis.host-media.two-d-svg",
            crate::artifacts::gismap::artifact_kind(),
            crate::artifacts::gismap::GIS_MAP_SCHEMA,
            "gis2d",
            crate::artifacts::gismap::schema::gis2d_document_json_to_svg,
        )?)
        .editor::<crate::editor::gis2d::Gis2dPlayApp>(crate::editor::gis2d::create_gis2d_app())
        .editor_mutation_roster::<crate::editor::gis2d::Gis2dPlayApp>()
        .viewer::<crate::viewer::gismap::GisMapViewer>(crate::viewer::gismap::create_gismap_viewer())
        .viewer_mutation_roster::<crate::viewer::gismap::GisMapViewer>()
        .editor::<crate::editor::gis3d::Gis3dPlayApp>(crate::editor::gis3d::create_gis3d_app())
        .editor_mutation_roster::<crate::editor::gis3d::Gis3dPlayApp>()
        .viewer::<crate::viewer::gisterrain::GisTerrainViewer>(crate::viewer::gisterrain::create_gisterrain_viewer())
        .viewer_mutation_roster::<crate::viewer::gisterrain::GisTerrainViewer>()
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M5 — `.activation(…)`/`.execution(…)`/
        // `.requests(…)` (`📓️design-abi.md` §3/§6). Only `gismap` gets an activation event:
        // `gisterrain` is a composed CHILD artifact (`Option<store::ArtifactChild<SemioMeshSnapshot>>`
        // on `gismap`'s own snapshot), never a standalone `ArtifactKindSpec` — confirmed by grep,
        // `🗿️artifacts/🏔️gisterrain/🦀️.rs` defines no `artifact_kind()` function.
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::gismap::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("documents.write".into()),
            scope: "plugin".into(),
            reason: "persist gis2d/gis3d editor edits (map features, terrain) to the open gismap document".into(),
            optional: false,
        })
        .requests(CapabilityRequest {
            id: CapabilityId("shell.navigate".into()),
            scope: "plugin".into(),
            reason: "the `shell` command opens an external basemap/attribution URL (Effect::OpenExternalUrl)".into(),
            optional: false,
        })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — the real
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
    //! new_viewer}` (closed by w0-f, gap 2), used directly rather than local stand-ins.
    use crate::editor::gis2d::Gis2dPlayApp;
    use crate::editor::gis3d::Gis3dPlayApp;
    use crate::viewer::gismap::GisMapViewer;
    use crate::viewer::gisterrain::GisTerrainViewer;
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn gis_component_assembly_declares_exact_package_identity_before_descriptor_emission() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🪪️artifact-identity/🔣️.json")).unwrap();
        let plugin = super::plugin().expect("GIS component must assemble with its exact semio:gis package identity");
        assert_eq!(plugin.manifest.plugin_id, fixture["pluginId"].as_str().unwrap());
        assert_eq!(plugin.artifact_definitions().len(), fixture["artifacts"].as_array().unwrap().len());
        for row in fixture["artifacts"].as_array().unwrap() {
            let kind = row["kind"].as_str().unwrap();
            let identity = semio_framework_plugin::ArtifactIdentity::parse(kind).unwrap();
            let definition = plugin.artifact_definitions().get(&identity).expect("literal canonical identity is registered");
            assert!(definition.capabilities().all(|capability| capability.identity().as_str().starts_with(&format!("{kind}."))));
            assert!(definition.capabilities().any(|capability| capability.claims().iter().any(|claim| claim.value() == row["nativeDialect"].as_str().unwrap())));
            assert!(definition.capabilities().any(|capability| capability.claims().iter().any(|claim| claim.value() == row["documentSchema"].as_str().unwrap())));
            assert!(definition.capabilities().any(|capability| capability.claims().iter().any(|claim| claim.namespace().as_str() == "codec-extension" && claim.value() == row["codecExtension"].as_str().unwrap())));
            assert!(!definition.capabilities().any(|capability| capability.claims().iter().any(|claim| claim.namespace().as_str() == "extension")), "GIS native codec extensions are schema-scoped, not global format authority");
            for role in row["roles"].as_array().unwrap() {
                let expected = format!("{}#{}", row["nativeDialect"].as_str().unwrap(), role.as_str().unwrap());
                assert_eq!(plugin.manifest.apps.iter().filter(|app| app.id == expected).count(), 1, "each literal role has exactly one actual app");
            }
        }
        for kind in fixture["hostileKinds"].as_array().unwrap() {
            let identity = semio_framework_plugin::ArtifactIdentity::parse(kind.as_str().unwrap()).unwrap();
            assert!(plugin.artifact_definitions().get(&identity).is_none());
        }
        assert_eq!(plugin.manifest.apps.len(), 4, "both GIS artifacts retain their editor and viewer");
        assert_eq!(crate::artifacts::gismap::artifact_kind().id, fixture["artifacts"][0]["kind"].as_str().unwrap());
    }

    #[semio_framework_async_macros::async_test]
    async fn gismap_viewer_never_mutates() {
        assert_viewer_never_mutates::<GisMapViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn gismap_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<Gis2dPlayApp, GisMapViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn gisterrain_viewer_never_mutates() {
        assert_viewer_never_mutates::<GisTerrainViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn gisterrain_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<Gis3dPlayApp, GisTerrainViewer>();
    }
}
//#endregion 🧪️SurfaceTests
