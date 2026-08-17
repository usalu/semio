//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::{HostMediaHandlerDeclaration, Plugin};

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("cad")
        .label("CAD")
        .version("0.1.0")
        .artifact(crate::artifacts::cad::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::mesh_dwg_bridge("s.cad.host-media.mesh-dwg", crate::artifacts::cad::artifact_kind(), crate::artifacts::cad::CAD_DOCUMENT_SCHEMA, crate::artifacts::cad::io::cad_document_from_mesh)?)
        .editor::<crate::editor::cad::CadPlayApp>(crate::editor::cad::create_cad_app())
        .editor_mutation_roster::<crate::editor::cad::CadPlayApp>()
        .viewer::<crate::viewer::cad::CadViewer>(crate::viewer::cad::create_cad_viewer())
        .viewer_mutation_roster::<crate::viewer::cad::CadViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 promises
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
    //! new_viewer}`, but as of this packet none of the three exist yet in the framework crate (grepped
    //! the whole tree, confirmed absent — `🧰️framework/**` is outside this packet's lease, so it
    //! cannot be added here). Local equivalents below exercise the same properties against
    //! `CadPlayApp`/`CadViewer` today; swap these three call sites for the canonical framework
    //! versions the moment W1-A lands them (tracked in `📓️w2-cad-report.md`'s migration recipe).
    // 🚧️ SDK GAP: see `✏️editor/🦀️component.rs`'s identical note — these three are only reachable
    // through `app`, not yet in the crate-root re-export list.
    use semio_framework_plugin::app::{ArtifactEditor, ArtifactViewer, ViewerApp};

    /// 👁️ Local stand-in for `testkit::new_viewer::<V>()` — `ViewerApp<V>` already implements the
    /// runtime `ArtifactApp` trait (the SDK adapter, contract §2.1), so the existing generic
    /// `testkit::new_app::<A: ArtifactApp>()` harness works today without any framework change.
    fn new_viewer<V: ArtifactViewer>() -> semio_framework_plugin::VcsArtifactApp<ViewerApp<V>> {
        semio_framework_plugin::testkit::new_app::<ViewerApp<V>>()
    }

    /// 👁️ Local stand-in for `testkit::assert_viewer_never_mutates::<V>()`. The real guarantee is a
    /// TYPE property, not a runtime one: `ArtifactViewer::handle` returns `ViewEmit<V::ConfigMutation>`
    /// (contract §2.2), a struct with NO field, constructor or method that accepts an artifact/draft
    /// mutation — so any `V` that compiles against this trait already cannot mutate, by construction.
    /// This still exercises the type end-to-end (builds a real `ViewerApp<V>` through the SDK adapter,
    /// the same path `PluginBuilder::viewer::<V>` uses) rather than asserting on the bare trait alone.
    fn assert_viewer_never_mutates<V: ArtifactViewer>() {
        // 🏗️ Builds a real `ViewerApp<V>` through the same SDK adapter path
        // `PluginBuilder::viewer::<V>` uses. `V::handle`'s return type,
        // `Result<ViewEmit<V::ConfigMutation>, Fault>`, is fixed by the `ArtifactViewer` trait
        // itself and structurally cannot name `artifact_mutations`/`draft_mutations` — the
        // guarantee is enforced at the `V: ArtifactViewer` bound above, not by anything this
        // function body could additionally check at runtime.
        let _app: semio_framework_plugin::VcsArtifactApp<ViewerApp<V>> = new_viewer::<V>();
    }

    /// ✏️👁️ Local stand-in for `testkit::assert_editor_and_viewer_share_dialect::<E, V>()`.
    fn assert_editor_and_viewer_share_dialect<E: ArtifactEditor, V: ArtifactViewer>() {
        assert_eq!(E::DIALECT, V::DIALECT, "an editor and viewer over the same subset must share one Dialect coordinate");
    }

    #[test]
    fn cad_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::cad::CadViewer>();
    }

    #[test]
    fn cad_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::cad::CadPlayApp, crate::viewer::cad::CadViewer>();
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
    #[test]
    fn cad_plugin_assembles_with_editor_and_viewer_apps() {
        let bundle = super::plugin().expect("cad plugin() must assemble; see require_declared_capability_or_record for the exact missing/misdeclared capability claim");
        let manifest = <semio_framework_plugin::Plugin as semio_framework_plugin::PluginProgram>::manifest(&bundle);
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
    #[test]
    fn cad_composer_entries_have_declared_capabilities() {
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
