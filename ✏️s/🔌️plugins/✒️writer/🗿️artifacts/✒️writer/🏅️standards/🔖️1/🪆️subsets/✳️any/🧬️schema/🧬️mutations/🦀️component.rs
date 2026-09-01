//! ✒️ Writer semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️component.rs` leaf.

use crate::artifacts::writer::{WriterDiff, WriterSnapshot};
use serde::{Deserialize, Serialize};

pub use super::change_language::{change_language, ChangeLanguage};
pub use super::change_uri::{change_uri, ChangeUri};
pub use super::edit_text::{edit_text, EditText};
pub use super::rename_writer::{rename_writer, RenameWriter};
pub use crate::artifacts::writer::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic Writer document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = WriterSnapshot, diff = WriterDiff, schema = "writer.writer")]
pub enum WriterMutation {
    RenameWriter(RenameWriter),
    ChangeUri(ChangeUri),
    ChangeLanguage(ChangeLanguage),
    EditText(EditText),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <WriterMutation as protocol::SemanticMutation<WriterSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("valid language-neutral oracle catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        {
            let kind = "rename-writer";
            let variant = "RenameWriter";
            let directory = "🏷️rename-writer";
            let tag = 0;
            let outcomes = &["applied", "warning"][..];
            let owner = mutation_root.join("🏷️rename-writer");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "change-uri";
            let variant = "ChangeUri";
            let directory = "🔗change-uri";
            let tag = 1;
            let outcomes = &["applied", "warning"][..];
            let owner = mutation_root.join("🔗change-uri");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "change-language";
            let variant = "ChangeLanguage";
            let directory = "🌐change-language";
            let tag = 2;
            let outcomes = &["applied", "warning"][..];
            let owner = mutation_root.join("🌐change-language");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "edit-text";
            let variant = "EditText";
            let directory = "✏️edit-text";
            let tag = 3;
            let outcomes = &["applied", "warning"][..];
            let owner = mutation_root.join("✏️edit-text");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
