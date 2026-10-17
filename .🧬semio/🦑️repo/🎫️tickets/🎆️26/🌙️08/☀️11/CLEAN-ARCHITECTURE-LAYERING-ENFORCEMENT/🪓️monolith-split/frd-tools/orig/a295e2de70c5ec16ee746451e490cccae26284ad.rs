#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use crate::io::text::snapshot::decode_kit_snapshot_json;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use crate::schema::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn minimal_schema_valid_json_defaults_collection_slots() {
    let schema: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🔣️.json")).expect("committed JSON Schema");
    // Every collection slot is required (the stdio.semio sibling convention); only the single
    // `properties` child is optional, and it is omitted — never `null` — when absent.
    assert_eq!(schema["required"], serde_json::json!(["schema", "types", "designs", "objects", "models", "representations"]));
    let minimal = serde_json::json!({ "schema": STDIO_SEMIOKIT_DOCUMENT_SCHEMA, "types": [], "designs": [], "objects": [], "models": [], "representations": [] });
    let minimal_text = minimal.to_string();
    let snapshot = decode_kit_snapshot_json(&minimal_text).expect("schema-valid minimal Kit snapshot");
    assert!(snapshot.types.is_empty());
    assert!(snapshot.designs.is_empty());
    assert!(snapshot.objects.is_empty());
    assert!(snapshot.models.is_empty());
    assert!(snapshot.properties.is_none());
    assert!(snapshot.representations.is_empty());
    let encoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&snapshot)).expect("snapshot reencodes");
    assert_eq!(encoded["schema"], minimal["schema"]);
    assert!(encoded.get("properties").is_none());
    let artifact: crate::schema::SemioKitArtifact = semio_framework_pack_json::from_json_str(&minimal_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("schema-valid minimal Kit artifact");
    assert!(artifact.types.is_empty() && artifact.designs.is_empty() && artifact.objects.is_empty() && artifact.models.is_empty());
    assert!(artifact.properties.is_none() && artifact.representations.is_empty());
    let encoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&artifact)).expect("artifact reencodes");
    assert!(encoded.get("properties").is_none());
    assert!(decode_kit_snapshot_json(r#"{"schema":"stdio.semio.kit","properties":null}"#).is_err());
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioKitSnapshot::default();
    let bytes = <SemioKitSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    let back = <SemioKitSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioKitSnapshot::default();
    let text = <SemioKitSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioKitSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = demo_kit_snapshot();
    let bytes = <SemioKitSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    let back = <SemioKitSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioKitSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioKitSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

/// 🧪️ A parent snapshot NEVER embeds owned-child content — only handles. Links are references
/// by design (never owned), so this proves both composition primitives stay handle-only.
#[semio_framework_async_macros::async_test]
async fn parent_snapshot_stores_only_handles_never_child_content() {
    let snap = demo_kit_snapshot();
    let text = <SemioKitSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
    assert!(text.contains(&enc_str("obj-01")));
    assert!(!text.to_lowercase().contains("primitives") && !text.to_lowercase().contains("elements"), "must never embed object/model field names — only the handle");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
