
use super::*;
use protocol::Mutation as _;

fn seal(mut page: store::os_directory::DirectoryEventPageV1) -> store::os_directory::DirectoryEventPageV1 {
    page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    page
}

/// 📬️ The page route's answer against `transient`, with its one item (if any) folded the way the transient lane folds it.
fn answer(transient: &HomeTransient, page: &store::os_directory::DirectoryEventPageV1) -> Result<(DirectoryProjectionReceiptV1, Option<HomeTransient>), Fault> {
    let answer = directory_page_answer(&pack::to_json_string(page), transient.directory())?;
    let next = answer.item.map(|item| item.diff(transient).diff().clone());
    Ok((answer.receipt, next))
}

#[semio_framework_async_macros::async_test]
async fn sealed_page_replaces_projection_once_and_rejects_races() {
    let binding = "a".repeat(64);
    let first = seal(store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: binding.clone(),
        authorization_generation: 7,
        after_seq_exclusive: 0,
        through_seq_inclusive: 5,
        has_more: true,
        events: Vec::new(),
        receipt_sha256: String::new(),
    });
    let initial = HomeTransient::default();
    let (receipt, next) = answer(&initial, &first).expect("first sealed page");
    let current = next.expect("the first page is one transient item");
    assert_eq!(current.directory().cursor(), 5, "invisible raw holes advance the resume frontier");
    assert_eq!(receipt, current.directory().receipt().expect("current receipt"));
    let (duplicate_receipt, duplicate) = answer(&current, &first).expect("idempotent replay");
    assert!(duplicate.is_none(), "an already-held frontier folds nothing");
    assert_eq!(duplicate_receipt, current.directory().receipt().expect("current receipt"), "an already-held frontier returns the same terminal receipt");

    let raced = seal(store::os_directory::DirectoryEventPageV1 { after_seq_exclusive: 3, through_seq_inclusive: 6, receipt_sha256: String::new(), ..first.clone() });
    assert!(answer(&current, &raced).is_err(), "a same-authority page that does not continue the held frontier cannot replace the projection");

    let event = store::os_directory::DirectoryEvent {
        seq: 7,
        id: "event-7".into(),
        hlc: store::os_directory::Hlc { physical_ms: 7, logical: 0 },
        actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "user:u1#s1".into() },
        space_id: Some("space-1".into()),
        user_id: None,
        body: store::os_directory::DirectoryEventBody::SpaceCreated {
            space_id: "space-1".into(),
            name: "Werkstatt".into(),
            space_kind: store::os_directory::DirectorySpaceKind::Studio,
            visibility: store::os_directory::DirectorySpaceVisibility::Private,
            owner_user_id: "u1".into(),
        },
        recorded_at_ms: 7,
    };
    let second = seal(store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: binding,
        authorization_generation: 7,
        after_seq_exclusive: 5,
        through_seq_inclusive: 7,
        has_more: false,
        events: vec![event],
        receipt_sha256: String::new(),
    });
    let (receipt, advanced) = answer(&current, &second).expect("ordered successor page");
    let advanced = advanced.expect("an ordered successor is one transient item");
    assert_eq!(receipt.schema, DirectoryProjectionReceiptV1::SCHEMA);
    assert_eq!(receipt.through_seq_inclusive, 7);
    assert_eq!(receipt.receipt_sha256, second.receipt_sha256);
    assert_eq!(advanced.directory().cursor(), 7);
    assert!(advanced.directory().space("space-1").is_some());
    assert_eq!(current.directory().space_count(), 0, "the captured root the successor was folded from never changes");

    let replacement = seal(store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: "b".repeat(64),
        authorization_generation: 8,
        after_seq_exclusive: 0,
        through_seq_inclusive: 0,
        has_more: false,
        events: Vec::new(),
        receipt_sha256: String::new(),
    });
    let replaced = answer(&advanced, &replacement).expect("new authority rebootstrap").1.expect("a new authority rebuilds");
    assert_eq!(replaced.directory().cursor(), 0);
    assert_eq!(replaced.directory().space_count(), 0);

    let origin_replay = seal(store::os_directory::DirectoryEventPageV1 { after_seq_exclusive: 0, through_seq_inclusive: 4, has_more: true, events: Vec::new(), receipt_sha256: String::new(), ..second.clone() });
    let rebuilt = answer(&advanced, &origin_replay).expect("a same-authority replay from the origin rebuilds the projection").1.expect("an origin replay is one item");
    assert_eq!(rebuilt.directory().cursor(), 4, "the rebuild restarts at the replayed page's frontier");
    assert_eq!(rebuilt.directory().space_count(), 0, "nothing folded before the origin replay survives it");

    let mut forged = second;
    forged.receipt_sha256 = "c".repeat(64);
    assert!(answer(&current, &forged).is_err(), "forged receipt is terminally rejected");
}

/// 🚫️ The direct lane never lands a page: no transient item, no receipt, a named refusal.
#[test]
fn the_direct_lane_refuses_by_name() {
    let history = semio_framework_plugin::HistoryView::empty();
    let document = SHomeSnapshot::default();
    let view = ArtifactView::new(&document, &history);
    let config = HomeConfig::default();
    let refused = handle(&ApplyDirectoryEventPage { page_json: String::new() }, &view, &ConfigView { snapshot: &config, window: None });
    assert!(matches!(refused, Err(fault) if fault.code.0.as_str() == "s.home.directory-event-page.requires-retained-job"));
}
