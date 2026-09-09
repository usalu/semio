//! 🧩️ CAD aec-building extension — contributes building STEP import profile to `cad-play`, plus (ticket
//! `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` W3-B pilot P2) a real
//! composite mutation and an inference contributed onto cad's OWN `s.cad.cad` artifact — the second
//! extensibility tier: an extension registering mutations/inferences on an artifact it does not own,
//! gated by a declared `.depends_on("cad", …)` runtime dependency (contract freeze §3/§4).

use pack::json::{self, Value as JsonValue};
use semio_framework_os_kernel::{pack_rt, DslValue, FromValue, ToValue};
use semio_framework_plugin::app::ArtifactContribution;
use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferenceService, ArtifactInferenceServiceMetadata, ExecutionMode, ExtensionBundle};
use semio_s_artifact_cad_cad::{CadMutation, CadSnapshot, CAD_DOCUMENT_SCHEMA};
use std::collections::BTreeMap;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-aec-building";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "aec-building";

/// 🗿️ Canonical artifact kind cad registers itself under (`s.cad.cad` — see
/// `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
/// `#[artifact_schema(id = "s.cad.cad")]`), NOT the pre-migration app-manifest id `"3d.cad"` cad's own
/// `artifact_kind()` still carries — the `ArtifactContribution`/policy `plugin-dependency/contribution-
/// target` gate both resolve a contribution's owner by splitting this canonical `s.<plugin>.<artifact>`
/// grammar (`ArtifactKindId::parse(..).plugin()`), which `"3d.cad"` cannot satisfy.
const CAD_ARTIFACT_KIND: &str = "s.cad.cad";

/// 💡️ This extension's own contributed-inference namespace — contract freeze §3: a contributed
/// inference's `inference_schema` MUST start with `s.<contributor-plugin-id>.`.
const AEC_BUILDING_INFERENCE_SCHEMA: &str = "s.cad-extension-aec-building.building-structure-summary";

// 🌱️ `computers_manifest`/`building_import_profile` below build a `pack::json::Value` directly
// (first-party `serde_json::Value` replacement) instead of a `#[derive(ToValue)]` DTO;
// `contributes_topic` now also speaks `DslValue` end to end, so `serde`/`serde_json` are fully
// gone from this crate.
/// 🗂️ `pack::json` analog of the former `CadImportProfileManifest`.
fn building_import_profile(model_definition_id: &'static str, prefer_presentation_layers: bool, presentation_geometry: Option<&'static str>) -> JsonValue {
    let mut entries: Vec<(String, JsonValue)> = vec![
        ("modelDefinitionId".to_string(), JsonValue::from(model_definition_id)),
        ("layerTypology".to_string(), json::object(building_layer_typology().into_iter().map(|(key, value)| (key.to_string(), JsonValue::from(value))))),
        ("fallbackTypology".to_string(), JsonValue::from("building.building.slab")),
    ];
    if prefer_presentation_layers {
        entries.push(("preferPresentationLayers".to_string(), JsonValue::from(true)));
    }
    if let Some(geometry) = presentation_geometry {
        entries.push(("presentationGeometry".to_string(), JsonValue::from(geometry)));
    }
    json::object(entries)
}

// 🚫️async: E1 pure — `BTreeMap::from` literal, zero suspension points — see R9.
fn building_layer_typology() -> BTreeMap<&'static str, &'static str> {
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

// 🚫️async: E1 pure — struct literal over `building_layer_typology` (sync), zero suspension points —
// see R9.
fn computers_manifest() -> JsonValue {
    json::object([("modelDefinitionIds".to_string(), json::array([JsonValue::from("aec.building")])), ("importProfiles".to_string(), json::array([building_import_profile("aec.building", false, None)]))])
}

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic`/`.contributes` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
// path_scope; `.contributes` is genuinely stateful registry work per that file's own doc comment,
// but is dressed as `async` with zero real suspension — same shape `.depends_on` already bridges in
// that same file); bridged here via `semio_framework::io::resolve_ready`, matching that established
// idiom. See this packet's lease-request asking the SDK owner to revert these to sync directly.
fn bundle() -> ExtensionBundle {
    let bundle = ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building", "0.1.0").extends("cad").depends_on("cad", semio_framework::VersionReq::parse("^0.1.0").expect("valid version req"));
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`cad.computer`) and, onto cad's OWN `s.cad.cad`
    // artifact, one composite mutation + one inference (both dispatched by the host through
    // the contributed-mutation/inference registries as bounded Cold job kinds at invocation
    // time, not by running this extension's own actor).
    let bundle = bundle.mode(ExecutionMode::Declarative);
    let bundle = bundle.contributes_topic(
        "cad.computer",
        DslValue::object([
            ("appId".to_string(), DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), DslValue::String(MODULE_ID.to_string())),
            ("label".to_string(), DslValue::String("AEC Building".to_string())),
            ("iconId".to_string(), DslValue::String("building".to_string())),
            ("computersJson".to_string(), DslValue::String(json::to_string(&computers_manifest()))),
        ]),
    );
    semio_framework::io::resolve_ready(bundle.contributes(building_storey_contribution()))
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Manifest

//#region 🔖️Composite
#[path = "🧬️schema/🧬️mutations/🏢️create-building-storey/🦀️.rs"]
mod create_building_storey;
pub use create_building_storey::CreateBuildingStorey;

/// 💡️ Contributed inference over cad's `s.cad.cad` artifact — a building-domain summary (does the
/// document have a building model attached, how many storeys has this extension's own composite
/// mutation created) that a generic CAD plugin has no reason to compute itself. `owner` MUST equal
/// this extension's own plugin id (contract freeze §4 rule 4, enforced by `register_contributions`).
// 🌱️ Never bound by a trait that requires serde — encoded/decoded directly through the
// first-party wire codec (`pack_rt::encode_wire_value`/`decode_wire_value` over `DslValue`) below,
// not JSON at all.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
struct BuildingStructureSummary {
    building_model_present: bool,
    storey_count: u32,
}

// 🚫️async: E1 pure — `ArtifactInferenceService::new` is already `const fn`; the fn-pointer argument
// `infer_building_structure_summary` is E4 (fn-pointer slot, see that fn's own tag) — see R9.
fn building_structure_summary_service() -> ArtifactInferenceService {
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

// 🚫️async: E4 fn-pointer slot — `semio_framework_plugin::ArtifactInference` is a plain
// `for<'a> fn(&ArtifactInferenceExecutionRequest<'a>) -> Result<..>` type alias (an `fn` item's
// pointer type is unnameable); also E1 pure (pack decode + struct literal, zero suspension) — see R9.
fn infer_building_structure_summary(request: &ArtifactInferenceExecutionRequest<'_>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError> {
    let snapshot = <CadSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload).map_err(|error| ArtifactInferenceExecutionError::new("cad-extension-aec-building.inference.snapshot-decode", error.to_string()))?;
    let summary = BuildingStructureSummary { building_model_present: snapshot.building_model.is_some(), storey_count: snapshot.nodes.iter().filter(|node| node.kind == "building-storey").count() as u32 };
    let canonical_payload = pack_rt::encode_wire_value(&ToValue::to_value(&summary));
    Ok(ArtifactInferenceExecution { canonical_payload, diagnostics: Vec::new(), validity: "valid".into(), quality: "complete".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone() })
}

/// 🗂️ The single `ArtifactContribution` this extension registers onto cad's `s.cad.cad` artifact —
/// one composite mutation, one inference, both gated by the `.depends_on("cad", …)` declared above.
// 🚫️async: E1 pure — consumed unawaited as the argument to `bundle()`'s `.contributes(...)` (which
// itself must be sync — see that fn's own tag). `ArtifactContribution::builder`/`.mutation`/
// `.inference_service`/`.build` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
// path_scope; that file's own `.mutation` doc comment already notes its body is "pure, no-real-
// suspension calls" bridged via `resolve_ready`) — bridged the same way here. See R9.
fn building_storey_contribution() -> ArtifactContribution {
    let contribution = semio_framework::io::resolve_ready(ArtifactContribution::builder(CAD_ARTIFACT_KIND));
    let contribution = semio_framework::io::resolve_ready(contribution.mutation::<CadSnapshot, CadMutation, CreateBuildingStorey>(CAD_DOCUMENT_SCHEMA, 1, 1));
    let contribution = semio_framework::io::resolve_ready(contribution.inference_service(building_structure_summary_service()));
    contribution.build()
}
//#endregion 🔖️Composite

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
