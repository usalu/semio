//! 🏔️ GIS terrain semantic mutation aggregate.

use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use serde::{Deserialize, Serialize};

pub use super::change_exaggeration::ChangeExaggeration;
pub use super::change_imported_features::ChangeImportedFeatures;
pub use crate::artifacts::gisterrain::schema::operations::*;

//#region 🔖️Aggregate
/// 🗺️ Typed terrain mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
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
        let direct_owners = [("change-exaggeration", "ChangeExaggeration", "🎚change-exaggeration", 0, &["applied", "warning"][..]), ("change-imported-features", "ChangeImportedFeatures", "📥change-imported-features", 1, &["applied", "warning"][..])];
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <GisTerrainMutation as protocol::SemanticMutation<GisTerrainSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral catalog")).expect("valid catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        for (kind, variant, directory, tag, outcomes) in direct_owners {
            let owner = mutation_root.join(directory);
            let source = std::fs::read_to_string(owner.join("🦀️component.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            for surface in ["🔣️payload.schema.json", "🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
                assert!(owner.join(surface).is_file(), "missing direct surface {surface}");
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
