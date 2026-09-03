//! 🌿️ Transparent VCS semantic mutation aggregate.

use crate::artifacts::vcs::VcsSnapshot;

pub use super::add_tag::{add_tag, AddTag};
pub use super::change_counter::{change_counter, ChangeCounter};
pub use super::change_notes::{change_notes, ChangeNotes};
pub use super::change_status::{change_status, ChangeStatus};
pub use super::remove_tag::{remove_tag, RemoveTag};
pub use super::rename_vcs::{rename_vcs, RenameVcs};
pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = VcsSnapshot, diff = crate::artifacts::vcs::VcsDiff, schema = "vcs.vcs")]
pub enum VcsDemoMutation {
    RenameVcs(RenameVcs),
    ChangeCounter(ChangeCounter),
    ChangeNotes(ChangeNotes),
    ChangeStatus(ChangeStatus),
    AddTag(AddTag),
    RemoveTag(RemoveTag),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;
    use protocol::SemanticMutation;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("valid catalog");
        let catalog_kinds: Vec<_> = catalog["mutationCatalogs"][0]["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("kind")).collect();
        let descriptor_kinds: Vec<_> = VcsDemoMutation::kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        {
            let kind = "rename-vcs";
            let variant = "RenameVcs";
            let owner = mutation_root.join("✏️rename-vcs");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🧬️wire/🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
        }
        {
            let kind = "change-counter";
            let variant = "ChangeCounter";
            let owner = mutation_root.join("🔢change-counter");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🧬️wire/🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
        }
        {
            let kind = "change-notes";
            let variant = "ChangeNotes";
            let owner = mutation_root.join("📝change-notes");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🧬️wire/🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
        }
        {
            let kind = "change-status";
            let variant = "ChangeStatus";
            let owner = mutation_root.join("🚦change-status");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🧬️wire/🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
        }
        {
            let kind = "add-tag";
            let variant = "AddTag";
            let owner = mutation_root.join("🏷️add-tag");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🧬️wire/🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
        }
        {
            let kind = "remove-tag";
            let variant = "RemoveTag";
            let owner = mutation_root.join("🗑️remove-tag");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🧬️wire/🔣️.schema.json")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
