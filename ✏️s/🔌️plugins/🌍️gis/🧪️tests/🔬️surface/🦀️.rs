//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — the real
//! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
//! new_viewer}` (closed by w0-f, gap 2), used directly rather than local stand-ins.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};
use semio_s_artifact_gis_gismap::editor::gis2d::Gis2dPlayApp;
use semio_s_artifact_gis_gismap::viewer::gismap::GisMapViewer;
use semio_s_artifact_gis_gisterrain::editor::gis3d::Gis3dPlayApp;
use semio_s_artifact_gis_gisterrain::viewer::gisterrain::GisTerrainViewer;

#[test]
fn gis_component_assembly_declares_exact_package_identity_before_descriptor_emission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️artifact-identity/🔣️.json")).unwrap();
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
    assert_eq!(semio_s_artifact_gis_gismap::artifact_kind().id, fixture["artifacts"][0]["kind"].as_str().unwrap());
}

#[semio_framework_async_macros::async_test]
async fn gismap_viewer_never_mutates() {
    assert_viewer_never_mutates::<GisMapViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn gismap_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<Gis2dPlayApp, GisMapViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn gisterrain_viewer_never_mutates() {
    assert_viewer_never_mutates::<GisTerrainViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn gisterrain_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<Gis3dPlayApp, GisTerrainViewer>().await;
}
