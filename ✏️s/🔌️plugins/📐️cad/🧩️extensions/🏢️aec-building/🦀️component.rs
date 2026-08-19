//! 🧩️ CAD aec-building extension — contributes building STEP import profile to `cad-play`, plus (ticket
//! `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` W3-B pilot P2) a real
//! composite mutation and an inference contributed onto cad's OWN `s.cad.cad` artifact — the second
//! extensibility tier: an extension registering mutations/inferences on an artifact it does not own,
//! gated by a declared `.depends_on("cad", …)` runtime dependency (contract freeze §3/§4).

use semio_framework_plugin::app::ArtifactContribution;
use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferenceService, ArtifactInferenceServiceMetadata, ExecutionMode, ExtensionBundle};
use semio_s_plugin_cad::artifacts::cad::mutations::change_active_model_definition::mutation::ChangeActiveModelDefinition;
use semio_s_plugin_cad::artifacts::cad::mutations::create_node::mutation::CreateNode;
use semio_s_plugin_cad::artifacts::cad::{CadMutation, CadNode, CadSnapshot, CAD_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-aec-building";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "aec-building";

/// 🗿️ Canonical artifact kind cad registers itself under (`s.cad.cad` — see
/// `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
/// `#[artifact_schema(id = "s.cad.cad")]`), NOT the pre-migration app-manifest id `"3d.cad"` cad's own
/// `artifact_kind()` still carries — the `ArtifactContribution`/policy `plugin-dependency/contribution-
/// target` gate both resolve a contribution's owner by splitting this canonical `s.<plugin>.<artifact>`
/// grammar (`ArtifactKindId::parse(..).plugin()`), which `"3d.cad"` cannot satisfy.
const CAD_ARTIFACT_KIND: &str = "s.cad.cad";

/// 💡️ This extension's own contributed-inference namespace — contract freeze §3: a contributed
/// inference's `inference_schema` MUST start with `s.<contributor-plugin-id>.`.
const AEC_BUILDING_INFERENCE_SCHEMA: &str = "s.cad-extension-aec-building.building-structure-summary";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CadImportProfileManifest {
    model_definition_id: &'static str,
    layer_typology: BTreeMap<&'static str, &'static str>,
    fallback_typology: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_presentation_layers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presentation_geometry: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_domain: Option<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CadComputersManifest {
    model_definition_ids: Vec<&'static str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    stat_computers: Vec<&'static str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    property_computers: Vec<&'static str>,
    import_profiles: Vec<CadImportProfileManifest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    transformation_appliers: Vec<&'static str>,
}

async fn building_layer_typology() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("slab", "building.building.slab"),
        ("slabs", "building.building.slab"),
        ("beam", "building.building.beam"),
        ("beams", "building.building.beam"),
        ("column", "building.building.column"),
        ("columns", "building.building.column"),
        ("wall", "building.building.wall"),
        ("walls", "building.building.wall"),
        ("roof", "building.building.roof"),
        ("roofs", "building.building.roof"),
        ("foundation", "building.building.foundation"),
        ("foundations", "building.building.foundation"),
        ("stair", "building.building.stair"),
        ("stairs", "building.building.stair"),
        ("ceiling", "building.building.ceiling"),
        ("ceilings", "building.building.ceiling"),
        ("railing", "building.building.railing"),
        ("railings", "building.building.railing"),
        ("door", "building.building.door"),
        ("doors", "building.building.door"),
        ("window", "building.building.window"),
        ("windows", "building.building.window"),
    ])
}

async fn computers_manifest() -> CadComputersManifest {
    CadComputersManifest {
        model_definition_ids: vec!["aec.building"],
        stat_computers: Vec::new(),
        property_computers: Vec::new(),
        import_profiles: vec![CadImportProfileManifest {
            model_definition_id: "aec.building",
            layer_typology: building_layer_typology(),
            fallback_typology: "building.building.slab",
            prefer_presentation_layers: None,
            presentation_geometry: None,
            namespaced_domain: None,
        }],
        transformation_appliers: Vec::new(),
    }
}

async fn bundle() -> ExtensionBundle {
    ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building", "0.1.0")
        .extends("cad")
        .depends_on("cad", semio_framework::VersionReq::parse("^0.1.0").expect("valid version req"))
        // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
        // extension only contributes a topic (`cad.computer`) and, onto cad's OWN `s.cad.cad`
        // artifact, one composite mutation + one inference (both dispatched by the host through
        // the contributed-mutation/inference registries as bounded Cold job kinds at invocation
        // time, not by running this extension's own actor).
        .mode(ExecutionMode::Declarative)
        .contributes_topic(
            "cad.computer",
            serde_json::json!({
                "appId": HOST_APP_ID,
                "moduleId": MODULE_ID,
                "label": "AEC Building",
                "iconId": "building",
                "computersJson": serde_json::to_string(&computers_manifest()).unwrap_or_default(),
            }),
        )
        .contributes(building_storey_contribution())
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Manifest

//#region 🔖️Composite
/// 🏢️ Composite mutation contributed onto cad's `s.cad.cad` artifact — a real building-domain
/// workflow step cad itself has no notion of (a bare CAD tool has no concept of a "storey"), planned
/// entirely from two of cad's OWN leaf mutations (`create-node`, `change-active-model-definition`)
/// through `protocol::Planner::call`. Frozen id grammar (contract freeze §3):
/// `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` — assembled by
/// `ArtifactContribution::resolve`, never hand-formatted here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBuildingStorey {
    pub storey_id: String,
    pub level_index: i32,
    pub storey_name: String,
}

impl CreateBuildingStorey {
    async fn storey_label(&self) -> String {
        format!("Level {}: {}", self.level_index, self.storey_name)
    }
}

impl protocol::CompositeMutationKind<CadSnapshot, CadMutation> for CreateBuildingStorey {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "building-storey", kind: "create-building-storey", record: "CreatedBuildingStorey" };

    async fn plan(&self, _base: &CadSnapshot, planner: &mut protocol::Planner<CadSnapshot, CadMutation>) -> Result<(), protocol::PlanError> {
        planner.call(CadMutation::CreateNode(CreateNode { node: CadNode { id: self.storey_id.clone(), label: self.storey_label(), kind: "building-storey".into() } }))?;
        planner.call(CadMutation::ChangeActiveModelDefinition(ChangeActiveModelDefinition { new_model_definition_id: "aec.building".into() }))
    }

    async fn label(&self) -> String {
        format!("Create building storey \"{}\"", self.storey_label())
    }

    async fn target(&self) -> Vec<String> {
        vec![self.storey_id.clone()]
    }
}

/// 💡️ Contributed inference over cad's `s.cad.cad` artifact — a building-domain summary (does the
/// document have a building model attached, how many storeys has this extension's own composite
/// mutation created) that a generic CAD plugin has no reason to compute itself. `owner` MUST equal
/// this extension's own plugin id (contract freeze §4 rule 4, enforced by `register_contributions`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildingStructureSummary {
    building_model_present: bool,
    storey_count: u32,
}

async fn building_structure_summary_service() -> ArtifactInferenceService {
    ArtifactInferenceService::new(
        ArtifactInferenceServiceMetadata {
            owner: EXTENSION_ID,
            artifact_kind: CAD_ARTIFACT_KIND,
            artifact_schema: CAD_ARTIFACT_KIND,
            artifact_schema_version: 1,
            document_schema: CAD_DOCUMENT_SCHEMA,
            document_schema_version: 1,
            inference_schema: AEC_BUILDING_INFERENCE_SCHEMA,
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
        },
        infer_building_structure_summary,
    )
}

async fn infer_building_structure_summary(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let snapshot = <CadSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload).map_err(|error| ArtifactInferenceExecutionError::new("cad-extension-aec-building.inference.snapshot-decode", error.to_string()))?;
    let summary = BuildingStructureSummary { building_model_present: snapshot.building_model.is_some(), storey_count: snapshot.nodes.iter().filter(|node| node.kind == "building-storey").count() as u32 };
    let canonical_payload = serde_json::to_vec(&summary).map_err(|error| ArtifactInferenceExecutionError::new("cad-extension-aec-building.inference.encode", error.to_string()))?;
    Ok(ArtifactInferenceExecution { canonical_payload, diagnostics: Vec::new(), validity: "valid".into(), quality: "complete".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
}

/// 🗂️ The single `ArtifactContribution` this extension registers onto cad's `s.cad.cad` artifact —
/// one composite mutation, one inference, both gated by the `.depends_on("cad", …)` declared above.
async fn building_storey_contribution() -> ArtifactContribution {
    ArtifactContribution::builder(CAD_ARTIFACT_KIND).mutation::<CadSnapshot, CadMutation, CreateBuildingStorey>(CAD_DOCUMENT_SCHEMA, 1, 1).inference_service(building_structure_summary_service()).build()
}
//#endregion 🔖️Composite

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, SemanticMutation};
    use semio_framework_plugin::{WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

    #[test]
    async fn bundle_contributes_building_import_profile() {
        let manifest = bundle().manifest;
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "cad.computer");
        assert_eq!(topic_contribution.payload["moduleId"], MODULE_ID);
        let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
        let parsed: serde_json::Value = serde_json::from_str(computers_json).expect("parse");
        assert!(parsed["importProfiles"][0]["layerTypology"]["beam"].as_str().is_some());
    }

    /// ✅️ Task requirement: "the contribution registers successfully against the declared
    /// dependency" — `bundle()` calling `.contributes(...)` without panicking already proves the
    /// registration gate accepted it; this test additionally pins the exact ids landed.
    #[test]
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
    #[test]
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
    #[test]
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
    /// applying `create-node` then `change-active-model-definition` directly.
    #[test]
    async fn plan_folds_to_the_same_snapshot_as_applying_cads_leaf_mutations_by_hand() {
        let base = semio_s_plugin_cad::artifacts::cad::empty_cad_snapshot();
        let kind = CreateBuildingStorey { storey_id: "storey-1".into(), level_index: 2, storey_name: "Level Two".into() };

        let folded = MutationDiff::apply(protocol::fold_plan_diff(&kind, &base).diff(), &base)
            .expect("valid folded plan diff");

        let create = CadMutation::CreateNode(CreateNode { node: CadNode { id: "storey-1".into(), label: kind.storey_label(), kind: "building-storey".into() } });
        let after_create = MutationDiff::apply(create.diff(&base).diff(), &base)
            .expect("valid create mutation diff");
        let switch = CadMutation::ChangeActiveModelDefinition(ChangeActiveModelDefinition { new_model_definition_id: "aec.building".into() });
        let after_switch = MutationDiff::apply(switch.diff(&after_create).diff(), &after_create)
            .expect("valid switch mutation diff");

        assert_eq!(folded, after_switch);
        assert_eq!(after_switch.active_model_definition_id, "aec.building");
        assert!(after_switch.nodes.iter().any(|node| node.id == "storey-1" && node.kind == "building-storey"));
    }

    #[test]
    async fn contributed_inference_computes_a_real_building_summary() {
        let mut base = semio_s_plugin_cad::artifacts::cad::empty_cad_snapshot();
        base.nodes.push(CadNode { id: "storey-1".into(), label: "Level One".into(), kind: "building-storey".into() });
        let pack = <CadSnapshot as store::ArtifactPack>::encode_pack(&base);
        let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 1 };
        let request = ArtifactInferenceExecutionRequest { policy: b"aec-building-test", budgets: &budgets, cancellation_id: "aec-building-test", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold, canonical_payload: &pack, dependencies: &[] };

        let execution = infer_building_structure_summary(&request).expect("inference succeeds");
        let summary: BuildingStructureSummary = serde_json::from_slice(&execution.canonical_payload).expect("summary decodes");
        assert_eq!(summary.storey_count, 1);
        assert!(!summary.building_model_present);

        let metadata = building_structure_summary_service().metadata();
        assert_eq!(metadata.owner, EXTENSION_ID);
        assert_eq!(metadata.artifact_kind, CAD_ARTIFACT_KIND);
    }
}
//#endregion 🧪️Tests
