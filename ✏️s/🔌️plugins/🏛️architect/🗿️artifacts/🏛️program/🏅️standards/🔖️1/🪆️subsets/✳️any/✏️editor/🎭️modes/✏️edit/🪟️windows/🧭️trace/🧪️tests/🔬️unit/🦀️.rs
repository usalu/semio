use super::*;
use crate::kernel::{EntityHeader, TextField};
use crate::registers::{AuditAction, AuditEvent};
use crate::sample_plugin;
use crate::EntityId;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_text_editor_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_TRACE);
    assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
}

#[semio_framework_async_macros::async_test]
async fn no_events_renders_the_empty_placeholder_row() {
    let json = crate::editor::architect::unit_tests::context::project_render(render(&sample_plugin(), &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("architect-trace.audit"));
    assert!(json.contains("architect-trace.audit.empty"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the audit feed is document-wide
/// now (no `InteractionView` in `render` to scope it to a selected entity) — every event shows,
/// not just ones touching one subject.
#[semio_framework_async_macros::async_test]
async fn renders_every_document_wide_audit_event() {
    let mut program = sample_plugin();
    program.audit_events.push(AuditEvent {
        header: EntityHeader::new(EntityId::new_serial("audit", "created"), "created"),
        action: AuditAction::Created,
        actor_id: None,
        subject_id: program.elements[0].header.id.clone(),
        subject_kind: "element".into(),
        timestamp: "2026-08-14T00:00:00Z".into(),
        details: TextField::plain("created"),
        before_state: None,
        after_state: None,
        ip_address: None,
        client: None,
        session_id: None,
        change_record_id: None,
        trace_link: None,
        success: true,
        error_message: None,
        correlation_id: None,
        compliance_tags: Vec::new(),
        retention_until: None,
    });
    let json = crate::editor::architect::unit_tests::context::project_render(render(&program, &semio_framework_plugin::TreeWindows::unhosted()));
    assert!(json.contains("architect-trace.audit.0"));
    assert!(!json.contains("architect-trace.audit.empty"));
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

/// 🪟️ An audit trail an order of magnitude past one viewport — the `.take(12)` truncation this window
/// used to carry would have hidden all but the first twelve of these.
fn oversized_trail(events: usize) -> crate::ProgramSnapshot {
    let mut program = sample_plugin();
    program.audit_events = (0..events)
        .map(|index| AuditEvent {
            header: EntityHeader::new(EntityId(format!("audit-{index}")), format!("event {index}")),
            action: AuditAction::Created,
            actor_id: None,
            subject_id: program.elements[0].header.id.clone(),
            subject_kind: "element".into(),
            timestamp: format!("2026-08-14T00:00:{:02}Z", index % 60),
            details: TextField::plain("created"),
            before_state: None,
            after_state: None,
            ip_address: None,
            client: None,
            session_id: None,
            change_record_id: None,
            trace_link: None,
            success: true,
            error_message: None,
            correlation_id: None,
            compliance_tags: Vec::new(),
            retention_until: None,
        })
        .collect();
    program
}

fn window_body(program: &crate::ProgramSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    crate::editor::architect::unit_tests::context::project_render(render(program, &TreeWindows::for_body(&view, ARCHITECT_BODY_TRACE)))
}

fn request(open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: ARCHITECT_BODY_TRACE.into(), node_key: "architect-trace.audit".into(), open, offset, rows }
}

/// 🪟️ Law (a): the audit section stamps its FULL extent and materialises at most one viewport — no
/// `+N`, no silent `.take(12)` cliff.
#[test]
fn an_oversized_trail_stamps_its_total_and_never_a_continuation_row() {
    let program = oversized_trail(400);
    let json = window_body(&program, Vec::new());
    let total = crate::standards::v1::subsets::any::schema::inferences::audit_trail(&program, None).events.len();
    assert!(json.contains(&format!("\"total\":{total}")), "the audit section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"key\":\"architect-trace.audit.").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): the section closed by the host stamps its total and materialises nothing.
#[test]
fn a_closed_audit_section_stamps_its_total_and_materialises_no_children() {
    let program = oversized_trail(400);
    let json = window_body(&program, vec![request(Some(false), 0, 0)]);
    let total = crate::standards::v1::subsets::any::schema::inferences::audit_trail(&program, None).events.len();
    assert!(json.contains(&format!("\"total\":{total}")), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("\"key\":\"architect-trace.audit."), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the event's own
/// absolute position in the trail — never renumbered per window.
#[test]
fn a_host_window_materialises_exactly_its_slice() {
    let json = window_body(&oversized_trail(400), vec![request(Some(true), 100, 10)]);
    assert!(json.contains("\"offset\":100"), "the section reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("\"key\":\"architect-trace.audit.{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"key\":\"architect-trace.audit.99\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"key\":\"architect-trace.audit.110\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d) for an UNBOUND tree: the audit feed is informational (`interactions: Vec::new()` on the
/// window definition), so it declares no domain, stamps no pick granularity, and its rows carry no
/// activation of their own.
#[test]
fn the_audit_feed_declares_no_domain_and_stamps_no_granularity() {
    let json = window_body(&oversized_trail(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "an informational feed binds no domain: {json}");
    assert!(!json.contains("granularity"), "an unbound tree stamps no pick granularity: {json}");
    assert!(!json.contains("interactionSelect"), "an unbound tree stamps no tree-level select: {json}");
}
//#endregion 🪟️WindowLaws
