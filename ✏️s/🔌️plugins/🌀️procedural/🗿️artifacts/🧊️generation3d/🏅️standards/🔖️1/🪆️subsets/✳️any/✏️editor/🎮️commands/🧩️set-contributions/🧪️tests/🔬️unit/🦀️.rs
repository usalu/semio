use super::*;
use crate::editor::generation3d::testkit::{empty_history_view, retire_flow_eval_session};
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
