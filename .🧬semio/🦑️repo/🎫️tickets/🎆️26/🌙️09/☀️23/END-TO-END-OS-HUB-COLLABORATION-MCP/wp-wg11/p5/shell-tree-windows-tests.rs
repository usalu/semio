use super::*;

fn request(key: &str, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { key: key.to_owned(), offset, rows }
}

/// 🪟️ LAW (ticket 26/09/23 session 14d, WG11 P5): window reports coalesce on the trailing debounce, a report that changes
/// nothing schedules nothing, and at most ONE refresh per body is in flight — the rules of React's
/// `createTreeWindowSchedulerV1`, so a scroll gesture is one partial refresh, not one per frame.
#[test]
fn window_reports_coalesce_and_a_body_refreshes_once_at_a_time() {
    let mut scheduler = TreeWindowScheduler::default();
    scheduler.report("home.body", &[request("spaces", 0, 36)], 20, 1_000);
    assert!(scheduler.due(1_000 + TREE_WINDOW_REPORT_DEBOUNCE_MS - 1).is_empty(), "the debounce holds a fresh report");
    assert_eq!(scheduler.due(1_000 + TREE_WINDOW_REPORT_DEBOUNCE_MS), vec!["home.body".to_owned()], "the trailing edge flushes it");
    scheduler.report("home.body", &[request("spaces", 0, 36)], 20, 2_000);
    assert!(scheduler.due(9_000).is_empty(), "the same windows again schedule nothing");
    scheduler.report("home.body", &[request("spaces", 40, 36)], 20, 3_000);
    assert!(scheduler.due(9_000).is_empty(), "a moved window waits while the body's refresh is in flight");
    scheduler.settled("home.body");
    assert_eq!(scheduler.due(9_001), vec!["home.body".to_owned()], "and is re-sent with the latest state the moment it settles");
}

/// 🪟️ LAW: an open toggle is due at once, and a container just opened asks one viewport until it is measured; a measured
/// container scrolled off screen asks nothing; an explicit fold asks nothing.
#[test]
fn an_opened_container_asks_one_viewport_until_it_is_measured() {
    let mut scheduler = TreeWindowScheduler::default();
    scheduler.report("outline", &[request("objects", 0, 30)], 24, 0);
    let _ = scheduler.due(TREE_WINDOW_REPORT_DEBOUNCE_MS);
    scheduler.settled("outline");
    scheduler.set_open("outline", "objects\u{241f}layers", true);
    assert_eq!(scheduler.due(1), vec!["outline".to_owned()], "an open toggle does not wait for the debounce");
    let (fields, viewport_rows) = scheduler.view_state_fields();
    assert_eq!(viewport_rows, Some(24));
    let opened = fields.iter().find(|field| field.node_key == "objects\u{241f}layers").expect("the opened container crosses");
    assert_eq!((opened.open, opened.rows), (Some(true), 24), "never measured: one viewport of rows");
    scheduler.settled("outline");
    scheduler.report("outline", &[request("objects", 0, 30)], 24, 10);
    let (fields, _) = scheduler.view_state_fields();
    assert_eq!(fields.iter().find(|field| field.node_key == "objects\u{241f}layers").map(|field| field.rows), Some(24), "still unmeasured after a report that did not include it");
    scheduler.set_open("outline", "objects", false);
    let (fields, _) = scheduler.view_state_fields();
    let folded = fields.iter().find(|field| field.node_key == "objects").expect("the folded container crosses");
    assert_eq!((folded.open, folded.offset, folded.rows), (Some(false), 0, 30), "a fold keeps its last window and says closed");
    assert_eq!(scheduler.window_of("outline", "objects"), Some((0, 0)), "a shell-owned builder materialises nothing for a folded container");
}

/// 🚧️ LAW: a path that is no view-context identifier never crosses (it would take the whole crossing down); the rest do.
#[test]
fn an_unsendable_window_path_is_dropped_and_the_rest_cross() {
    let mut scheduler = TreeWindowScheduler::default();
    scheduler.report("body", &[request("fine", 0, 10), request("bad\u{1f}path", 0, 10), request(&"x".repeat(257), 0, 10)], 10, 0);
    let (fields, _) = scheduler.view_state_fields();
    assert_eq!(fields.iter().map(|field| field.node_key.as_str()).collect::<Vec<_>>(), vec!["fine"]);
}
