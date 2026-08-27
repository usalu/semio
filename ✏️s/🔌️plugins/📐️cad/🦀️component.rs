//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, HostMediaHandlerDeclaration, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the CAD editor and viewer.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum CadApps: PluginApp {
        Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::cad::CadPlayApp>>),
        Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::cad::CadViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M2, `📓️design-abi.md`
/// §3) are this crate's proof-of-migration: the host activates one instance whenever a `"3d.cad"`
/// artifact (`crate::artifacts::cad::artifact_kind().id`) is opened, this plugin's actor runs
/// `Isolated` (no publisher trust assumed beyond the sandbox default — nothing in this crate's own
/// effects, all UI-chrome/RPC `Effect` variants with no documented `CapabilityId`, justifies
/// otherwise), and it asks the broker for document write access to persist edits.
pub fn plugin() -> Result<Plugin<CadApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<CadApps>::builder("cad")
        .label("CAD")
        .version("0.1.0")
        .artifact(crate::artifacts::cad::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::mesh_dwg_bridge("s.cad.host-media.mesh-dwg", crate::artifacts::cad::artifact_kind(), crate::artifacts::cad::CAD_DOCUMENT_SCHEMA, crate::artifacts::cad::io::cad_document_from_mesh)?)
        .editor::<crate::editor::cad::CadPlayApp>(crate::editor::cad::create_cad_app())
        .editor_mutation_roster::<crate::editor::cad::CadPlayApp>()
        .viewer::<crate::viewer::cad::CadViewer>(crate::viewer::cad::create_cad_viewer())
        .viewer_mutation_roster::<crate::viewer::cad::CadViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::cad::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist cad edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    impl Default for crate::viewer::cad::CadViewCommand {
        fn default() -> Self { Self::Noop }
    }

    #[semio_framework_async_macros::async_test]
    async fn cad_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::cad::CadViewer>().await;
    }

    #[semio_framework_async_macros::async_test]
    async fn cad_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::cad::CadPlayApp, crate::viewer::cad::CadViewer>().await;
    }
}
//#endregion 🧪️SurfaceTests

//#region 🧪️AssemblyTests
/// 🧪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, lane E2E-ASSEMBLY: `plugin()` must
/// assemble for real (not fall back to the WASM wire's `"assembly-failed"` manifest stub minted by
/// `require_declared_capability_or_record` in `🔌️plugin/🦀️component.rs`) and must carry both the
/// editor and viewer app surfaces.
#[cfg(test)]
mod assembly_tests {
    #[semio_framework_async_macros::async_test]
    async fn cad_plugin_assembles_with_editor_and_viewer_apps() {
        let bundle = super::plugin().expect("cad plugin() must assemble; see require_declared_capability_or_record for the exact missing/misdeclared capability claim");
        let manifest = semio_framework_plugin::PluginProgram::manifest(&bundle);
        assert_eq!(manifest.plugin_id, "cad");
        let app_ids: Vec<&str> = manifest.apps.iter().map(|app| app.id.as_str()).collect();
        assert!(app_ids.contains(&"s.cad.cad@1/*#editor"), "manifest apps {app_ids:?} missing the cad editor surface");
        assert!(app_ids.contains(&"s.cad.cad@1/*#viewer"), "manifest apps {app_ids:?} missing the cad viewer surface");
    }

    /// 🔎️ Diagnostic-only: walks every `ComposerEntry` cad's declaration feeds
    /// `ArtifactDeclaration::composers(...)` and reports, per entry, the exact dialect coordinate
    /// claim `require_declared_capability_or_record` derives from it and whether cad's own
    /// `definition()` declares a matching composer capability — pinpoints which entry (not just
    /// "some composer") is responsible when `cad_plugin_assembles_with_editor_and_viewer_apps` fails.
    #[semio_framework_async_macros::async_test]
    async fn cad_composer_entries_have_declared_capabilities() {
        let definition = crate::artifacts::cad::definition().expect("cad definition() must build");
        let entries = crate::artifacts::cad::standards::v1::subsets::any::io::io_registry::entries();
        let mut missing = Vec::new();
        for entry in entries {
            let coordinate = semio_framework::ArtifactDialect::from(entry.writes).to_coordinate();
            let claim = semio_framework_plugin::ArtifactIdentityClaim::new(semio_framework_plugin::ArtifactIdentityNamespace::dialect(), coordinate.clone()).expect("coordinate is a valid claim value");
            let claims = vec![claim];
            let declared = definition.capabilities_of(&semio_framework_plugin::ArtifactCapabilityKind::composer()).any(|capability| capability.claims() == claims);
            if !declared {
                missing.push(coordinate);
            }
        }
        assert!(missing.is_empty(), "composer entries with no matching declared composer capability: {missing:?}");
    }
}
//#endregion 🧪️AssemblyTests
