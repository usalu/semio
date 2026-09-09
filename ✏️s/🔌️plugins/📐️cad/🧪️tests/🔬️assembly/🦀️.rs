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
