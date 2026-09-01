//! 🎬️ Sequence semantic mutation aggregate and leaf detection registry.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

pub use super::change_step_collapsed::{change_step_collapsed, ChangeStepCollapsed};
pub use super::connect_steps::{connect_steps, ConnectSteps};
pub use super::create_step::{create_step, CreateStep};
pub use super::delete_step::{delete_step, DeleteStep};
pub use super::disconnect_steps::{disconnect_steps, DisconnectSteps};
pub use super::duplicate_step::{duplicate_step, DuplicateStep};
pub use super::edit_step_params::{edit_step_params, EditStepParams};
pub use super::move_step::{move_step, MoveStep};
pub use crate::artifacts::sequence::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Closed sequence mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SequenceSnapshot, diff = SequenceDiff, schema = "sequence.sequence")]
pub enum SequenceMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    MoveStep(MoveStep),
    EditStepParams(EditStepParams),
    ChangeStepCollapsed(ChangeStepCollapsed),
    ConnectSteps(ConnectSteps),
    DisconnectSteps(DisconnectSteps),
    DuplicateStep(DuplicateStep),
}
//#endregion 🔖️Aggregate

//#region 🔎️DetectionRegistry
pub const DETECTORS: &[SequenceMutationDetector] =
    &[super::create_step::detect, super::delete_step::detect, super::move_step::detect, super::edit_step_params::detect, super::change_step_collapsed::detect, super::connect_steps::detect, super::disconnect_steps::detect];
//#endregion 🔎️DetectionRegistry

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral catalog")).expect("valid catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        assert_eq!(DETECTORS.len(), 7);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        {
            let kind = "create-step";
            let variant = "CreateStep";
            let directory = "🌱create-step";
            let participation = "detect";
            let owner = mutation_root.join("🌱create-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "delete-step";
            let variant = "DeleteStep";
            let directory = "🗑️delete-step";
            let participation = "detect";
            let owner = mutation_root.join("🗑️delete-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "move-step";
            let variant = "MoveStep";
            let directory = "📍move-step";
            let participation = "detect";
            let owner = mutation_root.join("📍move-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "edit-step-params";
            let variant = "EditStepParams";
            let directory = "🔧edit-step-params";
            let participation = "detect";
            let owner = mutation_root.join("🔧edit-step-params");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "change-step-collapsed";
            let variant = "ChangeStepCollapsed";
            let directory = "🗂️change-step-collapsed";
            let participation = "detect";
            let owner = mutation_root.join("🗂️change-step-collapsed");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "connect-steps";
            let variant = "ConnectSteps";
            let directory = "🔗connect-steps";
            let participation = "detect";
            let owner = mutation_root.join("🔗connect-steps");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "disconnect-steps";
            let variant = "DisconnectSteps";
            let directory = "✂️disconnect-steps";
            let participation = "detect";
            let owner = mutation_root.join("✂️disconnect-steps");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "duplicate-step";
            let variant = "DuplicateStep";
            let directory = "🧬duplicate-step";
            let participation = "apply-only";
            let owner = mutation_root.join("🧬duplicate-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️payload.schema.json").is_file());
            }
            {
                assert!(owner.join("🔣️wire.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️component.ts").is_file());
            }
            {
                assert!(owner.join("🔗️component.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️component.proto").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
