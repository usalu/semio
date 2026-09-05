//! 🏔️ GIS terrain semantic mutation aggregate.

use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

pub use super::change_exaggeration::ChangeExaggeration;
pub use super::change_imported_features::ChangeImportedFeatures;
pub use crate::artifacts::gisterrain::schema::operations::*;

//#region 🔖️Aggregate
/// 🗺️ Typed terrain mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum, dsl::Mutations, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutations(snapshot = GisTerrainSnapshot, diff = GisTerrainDiff, schema = "gis.gisterrain")]
pub enum GisTerrainMutation {
    ChangeExaggeration(ChangeExaggeration),
    ChangeImportedFeatures(ChangeImportedFeatures),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <GisTerrainMutation as protocol::SemanticMutation<GisTerrainSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json")).expect("language-neutral catalog")).expect("valid catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        {
            let kind = "change-exaggeration";
            let variant = "ChangeExaggeration";
            let directory = "🎚️change-exaggeration";
            let tag = 0;
            let outcomes = &["applied", "warning"][..];
            let owner = mutation_root.join("🎚️change-exaggeration");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🧬️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            {
                assert!(owner.join("🧬️.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️.ts").is_file());
            }
            {
                assert!(owner.join("🔗️.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️.proto").is_file());
            }
            {
                assert!(owner.join("📝️text/🦀️.rs").is_file());
            }
            {
                assert!(owner.join("💾️binary/🦀️.rs").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "change-imported-features";
            let variant = "ChangeImportedFeatures";
            let directory = "📥change-imported-features";
            let tag = 1;
            let outcomes = &["applied", "warning"][..];
            let owner = mutation_root.join("📥change-imported-features");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🧬️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            {
                assert!(owner.join("🧬️.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️.ts").is_file());
            }
            {
                assert!(owner.join("🔗️.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️.proto").is_file());
            }
            {
                assert!(owner.join("📝️text/🦀️.rs").is_file());
            }
            {
                assert!(owner.join("💾️binary/🦀️.rs").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
