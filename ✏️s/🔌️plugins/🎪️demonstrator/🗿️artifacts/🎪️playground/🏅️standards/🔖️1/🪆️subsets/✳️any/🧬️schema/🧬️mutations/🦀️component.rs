//! 🧬️ Transparent playground semantic mutation aggregate.

use crate::artifacts::playground::standards::v1::subsets::any::schema::diff::PlaygroundDiff;
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use serde::{Deserialize, Serialize};

pub use super::change_schema::{ChangeSchema, KINDS, apply_playground_mutation_json, round_trip_playground_dsl, undo_playground_mutation_json};

//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for a playground document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = PlaygroundSnapshot, diff = PlaygroundDiff, schema = "s.demonstrator.playground")]
pub enum PlaygroundMutation {
    ChangeSchema(ChangeSchema),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;
    use protocol::SemanticMutation;

    #[test]
    fn direct_owner_descriptor_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let owner = mutation_root.join("✒️change-schema");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
        let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
        let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema");
        let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("language-neutral oracle catalog must be valid JSON");
        let descriptors = PlaygroundMutation::kinds();

        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].kind, "change-schema");
        assert!(source.contains("protocol::MutationKind"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], "change-schema");
        assert_eq!(descriptor["aggregateVariant"], "ChangeSchema");
        assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
        assert_eq!(payload_schema["title"], "ChangeSchema");
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
            assert!(surface_source.contains("change-schema") || surface_source.contains("ChangeSchema"));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains("change-schema") || surface_source.contains("ChangeSchema"));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
            assert!(surface_source.contains("change-schema") || surface_source.contains("ChangeSchema"));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
            assert!(surface_source.contains("change-schema") || surface_source.contains("ChangeSchema"));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
            assert!(surface_source.contains("change-schema") || surface_source.contains("ChangeSchema"));
        }
        assert!(catalog["mutationCatalogs"][0]["kinds"].as_array().expect("catalog kinds").iter().any(|kind| kind == "change-schema"));
    }
}
//#endregion 🧪️StructuralCorrespondence
