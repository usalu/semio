use crate::standards::v2_0::subsets::base::schema::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn complete_header_state_is_explicit_without_a_raw_archive_shadow_cache() {
    let json = format!("{:?}", ZipSnapshot { entries: vec![ZipEntry::default()], ..ZipSnapshot::default() });
    assert!(json.contains("metadata") && json.contains("compression_method") && json.contains("comment_utf8"));
    for forbidden in ["physical", "sourceBytes", "nativeArchive", "rawMetadata"] {
        assert!(!json.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        runtime_fixture_text("../../../📸️snapshot/🟦️.ts"),
        runtime_fixture_text("../../../📸️snapshot/🔗️.graphql"),
        runtime_fixture_text("../../../📸️snapshot/🔣️.json"),
        runtime_fixture_text("../../../📸️snapshot/🛰️.proto"),
    ] {
        for (camel, snake) in [("ZipEntryMetadata", "ZipEntryMetadata"), ("compressionMethod", "compression_method"), ("extraFields", "extra_fields"), ("commentUtf8", "comment_utf8")] {
            assert!(facet.contains(camel) || facet.contains(snake), "snapshot facet omits explicit header state {camel}");
        }
    }
    for facet in [
        runtime_fixture_text("../../../🔺️diff/🟦️.ts"),
        runtime_fixture_text("../../../🔺️diff/🔗️.graphql"),
        runtime_fixture_text("../../../🔺️diff/🔣️.json"),
        runtime_fixture_text("../../../🔺️diff/🛰️.proto"),
    ] {
        assert!(facet.contains("metadata"));
        assert!(facet.contains("commentUtf8") || facet.contains("comment_utf8"));
    }
    for facet in [runtime_fixture_text("../../../📸️snapshot/🟦️.ts"), runtime_fixture_text("../../../📸️snapshot/🔗️.graphql"), runtime_fixture_text("../../../📸️snapshot/🔣️.json"), runtime_fixture_text("../../../📸️snapshot/🛰️.proto")] {
        for forbidden in ["nativeArchive", "sourceBytes", "rawMetadata", "raw_metadata"] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
