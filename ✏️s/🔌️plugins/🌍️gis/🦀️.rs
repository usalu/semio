//! 🌍️ GIS composition of independently owned map and terrain artifacts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path = "📇️native-codecs/🦀️.rs"]
pub mod native_codecs;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, HostMediaHandlerDeclaration, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for both GIS artifact surfaces.
    pub enum GisApps: PluginApp {
        Gis2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_gis_gismap::editor::gis2d::Gis2dPlayApp>>),
        GisMapViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_gis_gismap::viewer::gismap::GisMapViewer>>),
        Gis3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_gis_gisterrain::editor::gis3d::Gis3dPlayApp>>),
        GisTerrainViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_gis_gisterrain::viewer::gisterrain::GisTerrainViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Composes map and terrain editors, viewers and native media capabilities.
pub fn plugin() -> Result<Plugin<GisApps>, PluginAssemblyError> {
    let dependency = semio_s_plugin_stdio::registry::native_artifact_catalog_dependency()?;
    let catalog = semio_s_plugin_stdio::registry::native_artifact_catalog_contribution()?;
    Plugin::<GisApps>::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .package_id("semio:gis")
        .depends_on(dependency.plugin_id, dependency.version)
        .contributes_topic(catalog)
        .artifact_kind(semio_s_artifact_gis_gismap::artifact_kind())
        .artifact(semio_s_artifact_gis_gismap::declaration().map_err(PluginAssemblyError::definition)?)
        .artifact(semio_s_artifact_gis_gisterrain::declaration().map_err(PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::two_d_svg_export(
            "s.gis.host-media.two-d-svg",
            semio_s_artifact_gis_gismap::artifact_kind(),
            semio_s_artifact_gis_gismap::GIS_MAP_SCHEMA,
            "gis2d",
            semio_s_artifact_gis_gismap::schema::gis2d_document_json_to_svg,
        )?)
        .editor::<semio_s_artifact_gis_gismap::editor::gis2d::Gis2dPlayApp>(semio_s_artifact_gis_gismap::editor::gis2d::create_gis2d_app())
        .editor_mutation_roster::<semio_s_artifact_gis_gismap::editor::gis2d::Gis2dPlayApp>()
        .viewer::<semio_s_artifact_gis_gismap::viewer::gismap::GisMapViewer>(semio_s_artifact_gis_gismap::viewer::gismap::create_gismap_viewer())
        .viewer_mutation_roster::<semio_s_artifact_gis_gismap::viewer::gismap::GisMapViewer>()
        .editor::<semio_s_artifact_gis_gisterrain::editor::gis3d::Gis3dPlayApp>(semio_s_artifact_gis_gisterrain::editor::gis3d::create_gis3d_app())
        .editor_mutation_roster::<semio_s_artifact_gis_gisterrain::editor::gis3d::Gis3dPlayApp>()
        .viewer::<semio_s_artifact_gis_gisterrain::viewer::gisterrain::GisTerrainViewer>(semio_s_artifact_gis_gisterrain::viewer::gisterrain::create_gisterrain_viewer())
        .viewer_mutation_roster::<semio_s_artifact_gis_gisterrain::viewer::gisterrain::GisTerrainViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_gis_gismap::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist gis2d/gis3d editor edits (map features, terrain) to the open gismap document".into(), optional: false })
        .requests(CapabilityRequest { id: CapabilityId("shell.navigate".into()), scope: "plugin".into(), reason: "the `shell` command opens an external basemap/attribution URL (Effect::OpenExternalUrl)".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, GisApps);
