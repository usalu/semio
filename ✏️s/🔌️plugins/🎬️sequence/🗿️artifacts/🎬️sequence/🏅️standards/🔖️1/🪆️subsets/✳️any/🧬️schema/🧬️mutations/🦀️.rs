//! 🎬️ Sequence semantic mutation aggregate and leaf detection registry.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

pub use crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::change_step_collapsed::{change_step_collapsed, ChangeStepCollapsed};
pub use crate::artifacts::sequence::standards::v1::subsets::dependency::schema::mutations::connect_steps::{connect_steps, ConnectSteps};
pub use crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::create_step::{create_step, CreateStep};
pub use crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::delete_step::{delete_step, DeleteStep};
pub use crate::artifacts::sequence::standards::v1::subsets::dependency::schema::mutations::disconnect_steps::{disconnect_steps, DisconnectSteps};
pub use crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::duplicate_step::{duplicate_step, DuplicateStep};
pub use crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::edit_step_params::{edit_step_params, EditStepParams};
pub use crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::move_step::{move_step, MoveStep};
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
    &[
        crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::create_step::detect,
        crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::delete_step::detect,
        crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::move_step::detect,
        crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::edit_step_params::detect,
        crate::artifacts::sequence::standards::v1::subsets::step::schema::mutations::change_step_collapsed::detect,
        crate::artifacts::sequence::standards::v1::subsets::dependency::schema::mutations::connect_steps::detect,
        crate::artifacts::sequence::standards::v1::subsets::dependency::schema::mutations::disconnect_steps::detect,
    ];
//#endregion 🔎️DetectionRegistry

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let subsets_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets");
        // 🪆️ The six step-node kinds and two dependency-edge kinds physically live under their own subset now
        // (ticket 26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION);
        // ✳️any no longer owns any mutation directory.
        let step_subset_root = subsets_root.join("✳️step");
        let dependency_subset_root = subsets_root.join("✳️dependency");
        let step_mutation_root = step_subset_root.join("🧬️schema/🧬️mutations");
        let dependency_mutation_root = dependency_subset_root.join("🧬️schema/🧬️mutations");
        let catalogs: Vec<serde_json::Value> =
            [step_subset_root, dependency_subset_root].into_iter().map(|subset_root| serde_json::from_str(&std::fs::read_to_string(subset_root.join("🧪️oracle/🔣️.json")).expect("language-neutral catalog")).expect("valid catalog")).collect();
        let mutation_catalogs: Vec<_> = catalogs.iter().flat_map(|catalog| catalog["mutationCatalogs"].as_array().expect("mutation catalogs")).collect();
        let mut descriptor_kinds: Vec<_> = <SequenceMutation as protocol::SemanticMutation<SequenceSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let mut catalog_kinds: Vec<_> = mutation_catalogs.iter().flat_map(|catalog| catalog["kinds"].as_array().expect("catalog kinds")).map(|kind| kind.as_str().expect("string kind")).collect();
        descriptor_kinds.sort_unstable();
        catalog_kinds.sort_unstable();
        assert_eq!(descriptor_kinds, catalog_kinds);
        assert_eq!(DETECTORS.len(), 7);
        let vectors: Vec<_> = mutation_catalogs.iter().flat_map(|catalog| catalog["vectors"].as_array().expect("catalog vectors")).collect();
        {
            let kind = "create-step";
            let variant = "CreateStep";
            let directory = "🌱create-step";
            let participation = "detect";
            let owner = step_mutation_root.join("🌱create-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "delete-step";
            let variant = "DeleteStep";
            let directory = "🗑️delete-step";
            let participation = "detect";
            let owner = step_mutation_root.join("🗑️delete-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "move-step";
            let variant = "MoveStep";
            let directory = "📍move-step";
            let participation = "detect";
            let owner = step_mutation_root.join("📍move-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "edit-step-params";
            let variant = "EditStepParams";
            let directory = "🔧edit-step-params";
            let participation = "detect";
            let owner = step_mutation_root.join("🔧edit-step-params");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "change-step-collapsed";
            let variant = "ChangeStepCollapsed";
            let directory = "🗂️change-step-collapsed";
            let participation = "detect";
            let owner = step_mutation_root.join("🗂️change-step-collapsed");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "connect-steps";
            let variant = "ConnectSteps";
            let directory = "🔗connect-steps";
            let participation = "detect";
            let owner = dependency_mutation_root.join("🔗connect-steps");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "disconnect-steps";
            let variant = "DisconnectSteps";
            let directory = "✂️disconnect-steps";
            let participation = "detect";
            let owner = dependency_mutation_root.join("✂️disconnect-steps");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "duplicate-step";
            let variant = "DuplicateStep";
            let directory = "🧬duplicate-step";
            let participation = "apply-only";
            let owner = step_mutation_root.join("🧬duplicate-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(source.contains("pub fn detect("), participation == "detect");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["diffParticipation"], participation);
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🧬️wire/🔣️.schema.json").is_file());
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
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
