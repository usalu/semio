use crate::GisTerrainSnapshot;
use semio_framework_test_fixtures::fixture_path;
#[allow(unused_imports)]
use crate::diff::GisTerrainDiff;
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use crate::standards::v1::subsets::any::schema::mutations::*;

#[test]
fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
    let mutation_root = fixture_path!("../../../🧬️mutations");
    let descriptor_kinds: Vec<_> = <GisTerrainMutation as protocol::SemanticMutation<GisTerrainSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(mutation_root.join("../../🔮️oracles/🔣️.json")).expect("language-neutral catalog")).expect("valid catalog");
    let mutation_catalog = &catalog["mutationCatalogs"][0];
    let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
    assert_eq!(descriptor_kinds, catalog_kinds);
    let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
    for (kind, variant, directory, tag, outcomes) in [
        ("change-exaggeration", "ChangeExaggeration", "🎚️change-exaggeration", 0, &["applied", "no-op"][..]),
        ("change-imported-features", "ChangeImportedFeatures", "📥change-imported-features", 1, &["applied", "no-op"][..]),
        ("replace-elevation-source", "ReplaceElevationSource", "🏔️replace-elevation-source", 2, &["applied", "no-op", "rejected"][..]),
    ] {
        let owner = mutation_root.join(directory);
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], tag);
        assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        for surface in ["🧬️schema/🔣️.json", "🟦️.ts", "🔗️.graphql", "🛰️.proto", "📝️text/🦀️.rs", "💾️binary/🦀️.rs"] {
            assert!(owner.join(surface).is_file(), "{kind}: {surface}");
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
    }
}
