use super::*;
use crate::editor::generation3d::testkit::{empty_history_view, retire_flow_eval_session};
use semio_framework_artifact_flow_flow::neural::ColdRetire;
use semio_framework_plugin::{ArtifactView, ConfigView};

fn dispatch(pages: &[(&str, u64, u64)]) -> Vec<Result<(), String>> {
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let outcomes = pages
        .iter()
        .map(|(json, page, page_count)| {
            handle(&SetContributions { json: (*json).to_string(), page: *page, page_count: *page_count }, &doc, &cfg, &mut session)
                .map(|emit| {
                    assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.effects.is_empty(), "a HostOnly contributions page publishes no store lane and no effect");
                })
                .map_err(|fault| fault.message.clone())
        })
        .collect();
    retire_flow_eval_session(session);
    outcomes
}

/// ⚖️ LAW: a one-page run installs the closure, and the registry resolves the operators it names —
/// this is the whole delivery path the served plugin runs, with no `install_flow_extension_manifest`
/// and no linked installer anywhere.
#[test]
fn a_paged_run_installs_the_contributed_registry() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let contributions = crate::editor::generation3d::testkit::staged_flow_extension_contributions_json(&[]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    assert!(pages.len() > 1, "the staged closure must exceed one public-invocation string page, else the paging law proves nothing");
    let addressed: Vec<(&str, u64, u64)> = pages.iter().enumerate().map(|(index, page)| (page.as_str(), index as u64, pages.len() as u64)).collect();
    for outcome in dispatch(&addressed) {
        outcome.expect("every page of a well-formed run is admitted");
    }
    assert_eq!(semio_framework_os_flow::host_flow_extension_contributions_pending_bytes(), 0, "the assembler retains nothing once its last page lands");
    assert!(semio_framework_os_flow::flow_extension_invocation_address("brep").is_ok(), "the contributed brep extension must be addressable after the run");
    assert!(semio_framework_os_flow::flow_extension_invocation_address("math").is_ok(), "the contributed math extension must be addressable after the run");
    assert_contributed_kind("brep.curve.polygon");
    assert_contributed_kind("math.vector");
}

fn contributed_kind_ids() -> Vec<String> {
    let catalogue: serde_json::Value = serde_json::from_str(&semio_framework_os_flow::flow_neuron_kind_infos_json()).expect("operator catalogue JSON");
    catalogue.as_array().expect("catalogue array").iter().filter_map(|item| item["id"].as_str().map(str::to_string)).collect()
}

fn assert_contributed_kind(kind: &str) {
    let ids = contributed_kind_ids();
    assert!(ids.iter().any(|id| id == kind), "contributed operator {kind} must be in the registry FlowHost indexes after setContributions, else evaluate answers unknown kind instead of PendingExtension; catalogue={ids:?}");
}

/// ⚖️ LAW: the packaged brep `manifestJson` the host actually pushes must parse as
/// [`semio_framework_os_flow::FlowExtensionManifest`]. `register_contributed_manifest` previously
/// returned on parse failure, so invocation addresses still resolved while every operator stayed
/// unknown (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_packaged_brep_manifest_parses_as_a_flow_extension_manifest() {
    let json = crate::flow_operators::resolve_ready(semio_s_plugin_flow_extension_brep::extension_manifest_json());
    let third_party: serde_json::Value = serde_json::from_str(&json).expect("RFC 8259 brep manifest");
    assert!(third_party["contributes"]["operators"].as_array().unwrap().iter().any(|operator| operator["id"] == "brep.curve.polygon"), "third-party JSON names brep.curve.polygon");
    let mut parsed: semio_framework_os_flow::FlowExtensionManifest = semio_framework_os_flow::os_pack::json::from_json_str(&json).unwrap_or_else(|error| panic!("packaged brep manifest must parse as FlowExtensionManifest: {error}"));
    let has_polygon = parsed.contributes.operators.iter().any(|operator| operator.id == "brep.curve.polygon");
    for operator in parsed.contributes.operators.drain(..) {
        operator.retire_cold();
    }
    for schema in parsed.contributes.schemas.drain(..) {
        schema.retire_cold();
    }
    assert!(has_polygon, "first-party FromValue must keep brep.curve.polygon");
}

/// ⚖️ LAW: the served shell sends page 0 of 1 as JSON.stringify-shaped contributions (camelCase
/// `pluginId` + extra payload fields), not the ToValue round-trip the paging assembler test uses.
#[test]
fn a_one_page_host_shaped_run_indexes_contributed_operators() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let contributions = host_shaped_contributions_json();
    assert!(contributions.contains("brep.curve.polygon") && contributions.contains("manifestJson"), "the host-shaped pack must carry the live operator ids");
    dispatch(&[(contributions.as_str(), 0, 1)]).into_iter().next().unwrap().expect("one-page host-shaped run is admitted");
    assert_contributed_kind("brep.curve.polygon");
    assert_contributed_kind("math.vector");
}

fn host_shaped_contributions_json() -> String {
    let brep = crate::flow_operators::resolve_ready(semio_s_plugin_flow_extension_brep::extension_manifest_json());
    let math = semio_s_plugin_flow_extension_math::extension_manifest_json();
    serde_json::json!([
        {
            "pluginId": crate::flow_operators::BREP_EXTENSION_PLUGIN_ID,
            "topicContribution": {
                "topic": "flow.extension",
                "payload": {
                    "appId": "generation3d",
                    "extensionId": "brep",
                    "iconId": "emoji:🧊",
                    "label": { "en": "Brep", "de": "Brep" },
                    "manifestJson": brep
                }
            }
        },
        {
            "pluginId": crate::flow_operators::MATH_EXTENSION_PLUGIN_ID,
            "topicContribution": {
                "topic": "flow.extension",
                "payload": {
                    "appId": "generation3d",
                    "extensionId": "math",
                    "iconId": "emoji:🧮",
                    "label": { "en": "Math", "de": "Mathe" },
                    "manifestJson": math
                }
            }
        }
    ])
    .to_string()
}

/// ⚖️ LAW: a gap in the run is refused and discards the partial buffer — a half-assembled payload
/// must never reach the registry.
#[test]
fn an_out_of_order_run_is_refused_and_discarded() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let outcomes = dispatch(&[("[", 0, 3), ("]", 2, 3)]);
    outcomes[0].as_ref().expect("page 0 opens the run");
    assert_eq!(outcomes[1].as_ref().unwrap_err(), "flow.contributions-page-out-of-order");
    assert_eq!(semio_framework_os_flow::host_flow_extension_contributions_pending_bytes(), 0, "a refused run retains nothing");
}

/// ⚖️ LAW: an address outside its own run is refused before a byte is buffered.
#[test]
fn an_invalid_page_address_is_refused() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let outcomes = dispatch(&[("[]", 0, 0), ("[]", 3, 3)]);
    assert_eq!(outcomes[0].as_ref().unwrap_err(), "flow.contributions-page-address-invalid");
    assert_eq!(outcomes[1].as_ref().unwrap_err(), "flow.contributions-page-address-invalid");
}
