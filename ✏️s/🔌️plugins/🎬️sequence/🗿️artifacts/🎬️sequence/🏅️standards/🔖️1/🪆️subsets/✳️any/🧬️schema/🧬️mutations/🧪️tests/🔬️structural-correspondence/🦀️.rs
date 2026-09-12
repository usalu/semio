use super::*;

#[test]
fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
    let subsets_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets");
    // 🪆️ The six step-node kinds and two dependency-edge kinds physically live under their own subset now
    // (ticket 26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION);
    // ✳️any no longer owns any mutation directory.
    let step_subset_root = subsets_root.join("🪜️step");
    let dependency_subset_root = subsets_root.join("🔗️dependency");
    let step_mutation_root = step_subset_root.join("🧬️schema/🧬️mutations");
    let dependency_mutation_root = dependency_subset_root.join("🧬️schema/🧬️mutations");
    let catalogs: Vec<serde_json::Value> =
        [step_subset_root, dependency_subset_root].into_iter().map(|subset_root| serde_json::from_str(&std::fs::read_to_string(subset_root.join("🔮️oracles/🔣️.json")).expect("language-neutral catalog")).expect("valid catalog")).collect();
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
        let directory = "🌱️create-step";
        let participation = "detect";
        let owner = step_mutation_root.join("🌱️create-step");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(source.contains("pub fn detect("), participation == "detect");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
        let directory = "📍️move-step";
        let participation = "detect";
        let owner = step_mutation_root.join("📍️move-step");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(source.contains("pub fn detect("), participation == "detect");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
        let directory = "🔧️edit-step-params";
        let participation = "detect";
        let owner = step_mutation_root.join("🔧️edit-step-params");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(source.contains("pub fn detect("), participation == "detect");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
        let directory = "🔗️connect-steps";
        let participation = "detect";
        let owner = dependency_mutation_root.join("🔗️connect-steps");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(source.contains("pub fn detect("), participation == "detect");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
        let directory = "🧬️duplicate-step";
        let participation = "apply-only";
        let owner = step_mutation_root.join("🧬️duplicate-step");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(source.contains("pub fn detect("), participation == "detect");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["diffParticipation"], participation);
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema"]));
        {
            assert!(owner.join("🧬️schema/🔣️.json").is_file());
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
