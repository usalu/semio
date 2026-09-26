use super::*;
use crate::kernel::{EntityHeader, TextField};
use crate::registers::{AuditAction, AuditEvent};
use crate::{empty_plugin, sample_plugin, EntityId};

fn event(timestamp: &str, success: bool) -> AuditEvent {
    AuditEvent {
        header: EntityHeader::new(EntityId("audit-event".into()), "Reception brief updated"),
        action: AuditAction::Updated,
        actor_id: None,
        subject_id: EntityId("element-reception".into()),
        subject_kind: "element".into(),
        timestamp: timestamp.into(),
        details: TextField::plain("Area target changed to 25 m²"),
        before_state: None,
        after_state: None,
        ip_address: None,
        client: None,
        session_id: None,
        change_record_id: None,
        trace_link: None,
        success,
        error_message: None,
        correlation_id: None,
        compliance_tags: Vec::new(),
        retention_until: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_event_feed_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_TRACE);
    assert!(matches!(definition.surface_kind, SurfaceKind::EventFeed));
}

#[test]
fn rfc3339_projection_keeps_epoch_domain_data_and_rejects_invalid_calendar_dates() {
    assert_eq!(rfc3339_utc_epoch_ms("2026-08-14T00:00:00Z"), Some(1_786_665_600_000));
    assert_eq!(rfc3339_utc_epoch_ms("2026-08-14T00:00:00.125Z"), Some(1_786_665_600_125));
    assert_eq!(rfc3339_utc_epoch_ms("2026-02-29T00:00:00Z"), None);
}

#[semio_framework_async_macros::async_test]
async fn authored_audit_record_projects_to_a_real_event_feed_scene() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral EventFeed producer fixture");
    let source = &fixture["source"];
    let mut program = sample_plugin();
    let mut audit = event(source["timestamp"].as_str().expect("timestamp"), source["success"].as_bool().expect("success"));
    audit.header.id = EntityId(source["id"].as_str().expect("id").into());
    audit.header.name = source["title"].as_str().expect("title").into();
    audit.details = TextField::plain(source["detail"].as_str().expect("detail"));
    program.audit_events = vec![audit];
    let json = crate::editor::architect::unit_tests::context::project_render(render(&program, &TreeWindows::unhosted()));
    assert!(json.contains("event-feed"), "missing event-feed surface: {json}");
    assert!(!json.contains("text-editor"), "trace must not fall back to a generic text surface: {json}");
    let scene: EventFeedScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&json).expect("packed event-feed scene");
    let entries: Vec<serde_json::Value> = serde_json::from_str(&scene.entries_json).expect("event entries");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0], fixture["expected"]);
    assert_eq!(scene.follow, Some(true));
    assert_eq!(scene.activate_action, None);
}

#[semio_framework_async_macros::async_test]
async fn empty_audit_trail_is_an_empty_authored_feed() {
    let scene_json = crate::editor::architect::unit_tests::context::project_render(render(&empty_plugin(), &TreeWindows::unhosted()));
    let scene: EventFeedScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&scene_json).expect("packed event-feed scene");
    let entries: Vec<serde_json::Value> = serde_json::from_str(&scene.entries_json).expect("event entries");
    assert!(entries.is_empty());
}
