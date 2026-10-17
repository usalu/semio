use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn logical_snapshot_and_facets_have_no_shadow_state() {
    let json = format!("{:?}", PptxSnapshot::default());
    for forbidden in ["physical", "sourceBytes", "nativeArchive", "semanticBlake3"] {
        assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔗️.graphql"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛰️.proto"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🟦️.ts"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔗️.graphql"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔣️.json"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🛰️.proto"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔗️.graphql"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🛰️.proto"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔗️.graphql"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔣️.json"),
        crate::runtime_fixture::text("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🛰️.proto"),
    ] {
        for forbidden in ["PptxPhysical", "sourceBytes", "source_bytes", "nativeArchive", "native_archive", "semanticBlake3", "semantic_blake3", "archiveBytes", "archive_bytes", "rawXml", "raw_xml"] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}
