use super::*;
use protocol::{Mutation, MutationDiff, SemanticMutation};
use semio_framework_plugin::{WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};
use semio_s_artifact_cad_cad::mutations::create_node::CreateNode;
use semio_s_artifact_cad_cad::CadNode;

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_building_import_profile() {
    let manifest = bundle().manifest;
    let topic_contribution = &manifest.topic_contributions[0];
    assert_eq!(topic_contribution.topic, "cad.computer");
    assert_eq!(topic_contribution.payload["moduleId"].as_str(), Some(MODULE_ID));
    let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
    let parsed = json::parse(computers_json).expect("parse");
    let beam_typology = parsed.get("importProfiles").and_then(JsonValue::as_array).and_then(|profiles| profiles.first()).and_then(|profile| profile.get("layerTypology")).and_then(|typology| typology.get("beam"));
    assert!(beam_typology.and_then(JsonValue::as_str).is_some());
}

/// ✅️ Task requirement: "the contribution registers successfully against the declared
/// dependency" — `bundle()` calling `.contributes(...)` without panicking already proves the
/// registration gate accepted it; this test additionally pins the exact ids landed.
#[semio_framework_async_macros::async_test]
async fn bundle_declares_the_cad_dependency_and_registers_the_building_storey_contribution() {
    let manifest = bundle().manifest;
    assert_eq!(manifest.extends, "cad");
    assert_eq!(manifest.dependencies[0].plugin_id, "cad");

    let contribution = &manifest.contributions[0];
    assert_eq!(contribution.artifact_kind, CAD_ARTIFACT_KIND);

    let mutation = &contribution.mutations[0];
    assert_eq!(mutation.mutation_id, format!("{CAD_DOCUMENT_SCHEMA}#{EXTENSION_ID}:create-building-storey"));
    assert_eq!(mutation.semantics.verb, "create");
    assert_eq!(mutation.semantics.kind, "create-building-storey");
    assert_eq!(mutation.semantics.record, "CreatedBuildingStorey");

    // ✅️ Task requirement: "the inference's metadata passes the ownership gate" — contract
    // freeze §4 rule 4 (owner == contributor == this extension's own plugin id, artifact_kind ==
    // target); `.contributes()` would already have panicked had `register_contributions`
    // rejected this, so these equalities double-check the landed values directly.
    let inference = &contribution.inferences[0];
    assert_eq!(inference.inference_schema, AEC_BUILDING_INFERENCE_SCHEMA);
    assert_eq!(inference.owner, EXTENSION_ID);
    assert_eq!(inference.contributor, EXTENSION_ID);
    assert_eq!(inference.artifact_kind, CAD_ARTIFACT_KIND);
}

/// ✅️ Task requirement: "removing/mismatching the dependency makes registration fail with the
/// typed gate error" — `ExtensionBundle::contributes` panics on a `ContributionRegistrationError`
/// (same infallible-builder idiom `.extends`/`.depends_on` already use), so a missing
/// `.depends_on("cad", …)` must be caught here via `catch_unwind`, mirroring the framework's own
/// `extension_bundle_dependency_tests`.
#[semio_framework_async_macros::async_test]
async fn contribution_onto_cad_requires_a_declared_dependency() {
    let result = std::panic::catch_unwind(|| {
        ExtensionBundle::new("cad-extension-aec-building-test-missing-dep", "Test Missing Dep", "0.1.0")
                .extends("cad")
                // ⚠️ deliberately NO `.depends_on("cad", …)` here.
                .contributes(building_storey_contribution())
    });
    assert!(result.is_err(), "a contribution onto a non-dependency must be rejected by the typed gate, not silently accepted");
}

/// ✅️ Task requirement: "the contributed mutation id does not collide with any cad owner kind" —
/// structural proof (contract freeze §3's `:` segment) plus an explicit sweep of cad's own
/// `CadMutation::kinds()` roster.
#[semio_framework_async_macros::async_test]
async fn contributed_mutation_id_structurally_cannot_collide_with_any_cad_owner_kind() {
    let mutation_id = bundle().manifest.contributions[0].mutations[0].mutation_id.clone();
    let hash_at = mutation_id.rfind('#').expect("contributed id has a #");
    assert!(mutation_id[hash_at + 1..].contains(':'), "contributed id must carry the contributor ':' segment");

    for descriptor in <CadMutation as SemanticMutation<CadSnapshot>>::kinds() {
        let owner_id = format!("{CAD_DOCUMENT_SCHEMA}#{}", descriptor.kind);
        assert_ne!(owner_id, mutation_id, "contributed id must never equal an owner mutation id");
        assert!(!owner_id.contains(':'), "owner-mutation-id grammar never carries a ':' segment — the invariant the collision proof relies on");
    }
}

/// ✅️ Task requirement: "the plan folds to the same snapshot as applying cad's leaf mutations by
/// hand" — `protocol::fold_plan_diff` over `CreateBuildingStorey::plan` must equal sequentially
/// applying `create-node` directly.
#[semio_framework_async_macros::async_test]
async fn plan_folds_to_the_same_snapshot_as_applying_cads_leaf_mutations_by_hand() {
    let base = semio_s_artifact_cad_cad::empty_cad_snapshot();
    let kind = CreateBuildingStorey { storey_id: "storey-1".into(), level_index: 2, storey_name: "Level Two".into() };

    let folded = MutationDiff::apply(protocol::fold_plan_diff(&kind, &base).diff(), &base).expect("valid folded plan diff");

    let create = CadMutation::CreateNode(CreateNode { node: CadNode { id: "storey-1".into(), label: kind.storey_label(), kind: "building-storey".into() } });
    let after_create = MutationDiff::apply(create.diff(&base).diff(), &base).expect("valid create mutation diff");

    assert_eq!(folded, after_create);
    assert!(after_create.nodes.iter().any(|node| node.id == "storey-1" && node.kind == "building-storey"));
}

#[semio_framework_async_macros::async_test]
async fn contributed_inference_computes_a_real_building_summary() {
    let mut base = semio_s_artifact_cad_cad::empty_cad_snapshot();
    base.nodes.push(CadNode { id: "storey-1".into(), label: "Level One".into(), kind: "building-storey".into() });
    let pack = <CadSnapshot as store::ArtifactPack>::encode_pack(&base);
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 1 };
    let request = ArtifactInferenceExecutionRequest {
        policy: b"aec-building-test",
        budgets: &budgets,
        cancellation_id: "aec-building-test",
        previous_state: None,
        requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
        canonical_payload: &pack,
        dependencies: &[],
    };

    let execution = infer_building_structure_summary(&request).expect("inference succeeds");
    let decoded_value = pack_rt::decode_wire_value(&execution.canonical_payload).expect("summary decodes");
    let summary = BuildingStructureSummary::from_value(decoded_value).expect("summary decodes");
    assert_eq!(summary.storey_count, 1);
    assert!(!summary.building_model_present);

    let metadata = building_structure_summary_service().metadata();
    assert_eq!(metadata.owner, EXTENSION_ID);
    assert_eq!(metadata.artifact_kind, CAD_ARTIFACT_KIND);
}
