use super::*;
use serde_json::Value;

const BUDGET: usize = ui_contract::TREE_WINDOW_BODY_NODE_BUDGET;

fn requests_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪟️window-requests/🔣️.json")).expect("the neutral window-request vectors parse")
}

fn served_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧬️contract/🧫️fixtures/🪟️tree-window-served.json")).expect("the neutral served-window vectors parse")
}

fn number(value: &Value) -> f64 {
    value.as_f64().expect("a number")
}

fn whole(value: &Value) -> u32 {
    value.as_u64().expect("a whole number") as u32
}

fn measures(case: &Value, row_px: &Value) -> Vec<TreeWindowContainerMeasure> {
    case["containers"]
        .as_array()
        .expect("containers")
        .iter()
        .map(|container| TreeWindowContainerMeasure {
            key: container["key"].as_str().expect("key").to_owned(),
            total: whole(&container["total"]),
            offset: whole(&container["offset"]),
            length: whole(&container["length"]),
            top: number(&container["top"]),
            height: number(&container["height"]),
            row_px: number(&row_px[container["rowExtent"].as_str().expect("row extent")]),
            rows: container["rows"].as_array().expect("rows").iter().map(|row| TreeWindowRowMeasure { index: whole(&row[0]), top: number(&row[1]) }).collect(),
        })
        .collect()
}

fn requests(value: &Value) -> Vec<TreeWindowRequest> {
    value.as_array().expect("requests").iter().map(|request| TreeWindowRequest { key: request["key"].as_str().expect("key").to_owned(), offset: whole(&request["offset"]), rows: whole(&request["rows"]) }).collect()
}

/// 🪟️ LAW (ticket 26/09/23 session 14d, WG11 P5): for every neutral viewport vector React's rule wrote
/// (`🧫️fixtures/🪟️window-requests/🔣️.json`), the wgpu twin measures the same visible rows and asks the same per-container and
/// body-wide windows — React and wgpu request the same windows for the same viewport.
#[test]
fn the_wgpu_rule_asks_the_windows_react_asks_for_every_neutral_viewport() {
    let fixture = requests_fixture();
    assert_eq!(whole(&fixture["budget"]) as usize, BUDGET, "the vectors were written against this body budget");
    assert_eq!(whole(&fixture["overscan"]), TREE_WINDOW_OVERSCAN_ROWS);
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let containers = measures(case, &fixture["rowExtentPx"]);
        let viewport = number(&case["viewportHeight"]);
        let visible = visible_rows_for_viewport(&containers, 0.0, viewport);
        let expected: Vec<(String, u32, u32, u32, f64)> = case["visible"]
            .as_array()
            .expect("visible")
            .iter()
            .map(|entry| (entry["key"].as_str().expect("key").to_owned(), whole(&entry["total"]), whole(&entry["visibleRows"]), whole(&entry["firstVisibleRow"]), number(&entry["distancePx"])))
            .collect();
        assert_eq!(visible.len(), expected.len(), "{name}: visible containers");
        for ((key, rows), (expected_key, total, visible_rows, first, distance)) in visible.iter().zip(&expected) {
            assert_eq!((key, rows.total, rows.visible_rows, rows.first_visible_row), (expected_key, *total, *visible_rows, *first), "{name}");
            assert!((rows.distance_px - distance).abs() < 1e-9, "{name}: distance {} vs {distance}", rows.distance_px);
        }
        assert_eq!(requests_for_viewport(&containers, 0.0, viewport, TREE_WINDOW_OVERSCAN_ROWS), requests(&case["requests"]), "{name}: per-container requests");
        assert_eq!(body_requests(&containers, viewport, BUDGET), requests(&case["body"]), "{name}: body requests");
    }
}

/// 🧾️ LAW: no body answer asks the guest for more than its ledger holds — every container costs `1 + rows`.
#[test]
fn every_body_answer_fits_the_guest_ledger() {
    let fixture = requests_fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let body = body_requests(&measures(case, &fixture["rowExtentPx"]), number(&case["viewportHeight"]), BUDGET);
        let cost: usize = body.iter().map(|request| 1 + request.rows as usize).sum();
        assert!(cost <= BUDGET, "{}: {cost} > {BUDGET}", case["name"]);
    }
}

fn uniform(key: &str, total: u32, offset: u32, length: u32, top_rows: f64, row_px: f64) -> TreeWindowContainerMeasure {
    TreeWindowContainerMeasure {
        key: key.to_owned(),
        total,
        offset,
        length,
        top: top_rows * row_px,
        height: f64::from(total) * row_px,
        row_px,
        rows: (0..length).map(|position| TreeWindowRowMeasure { index: offset + position, top: (top_rows + f64::from(offset + position)) * row_px }).collect(),
    }
}

fn anchor(value: &Value) -> TreeWindowAnchor {
    match value.as_str().expect("anchor") {
        "end" => TreeWindowAnchor::End,
        _ => TreeWindowAnchor::Start,
    }
}

/// 🧠️ LAW: the served rule answers the shared short-window vectors (`🧬️contract/🧫️fixtures/🪟️tree-window-served.json`, the same
/// cases the React Interpreter law reads) — capacity learned, window placed, anchor kept, exactly as React's.
#[test]
fn the_served_rule_answers_the_shared_short_window_vectors() {
    let row_px = 24.0;
    for case in served_fixture()["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let container = &case["container"];
        let key = container["key"].as_str().expect("key");
        let measure = uniform(key, whole(&container["total"]), whole(&container["offset"]), whole(&container["length"]), number(&container["topRows"]), row_px);
        let memory: Vec<(String, TreeWindowServedMemory)> = match &case["memory"] {
            Value::Null => Vec::new(),
            memory => vec![(
                key.to_owned(),
                TreeWindowServedMemory {
                    asked: TreeWindowRequest { key: key.to_owned(), offset: whole(&memory["asked"]["offset"]), rows: whole(&memory["asked"]["rows"]) },
                    capacity: memory["capacity"].as_u64().map(|capacity| capacity as u32),
                    anchor: anchor(&memory["anchor"]),
                    first_visible: memory["firstVisible"].as_u64().map(|row| row as u32),
                    total: whole(&memory["total"]),
                },
            )],
        };
        let (answer, kept) = served_requests(&[measure], number(&case["viewportRows"]) * row_px, &memory, BUDGET);
        let expect = &case["expect"];
        assert_eq!(answer, vec![TreeWindowRequest { key: key.to_owned(), offset: whole(&expect["request"]["offset"]), rows: whole(&expect["request"]["rows"]) }], "{name}");
        let kept = &kept.iter().find(|(kept_key, _)| kept_key == key).expect("memory kept").1;
        assert_eq!((kept.capacity, kept.anchor), (expect["capacity"].as_u64().map(|capacity| capacity as u32), anchor(&expect["anchor"])), "{name}");
    }
}

/// 🧪️ The guest's own answer rule, stated independently (the React law's `answer`): the asked offset clamped into the list,
/// then as many rows as were asked, exist and fit its capacity.
fn guest_answer(asked: &TreeWindowRequest, total: u32, capacity: u32) -> (u32, u32) {
    let offset = asked.offset.min(total.saturating_sub(asked.rows.max(1)));
    (offset, asked.rows.min(total - offset).min(capacity))
}

/// 🏁️ LAW (coordinator 07:4x): scrolling a 10 000-row list from its top to its end and back reaches EVERY row under a guest
/// that serves any capacity, settles silent at every position (the same question twice gets the same answer), and the window
/// at the end position ends on the last row.
#[test]
fn scrolling_a_ten_thousand_row_list_to_its_end_reaches_the_last_row() {
    let row_px = 24.0;
    for (total, viewport_rows, capacity) in [(10_000u32, 32u32, 27u32), (10_000, 20, 200), (10_000, 32, 48), (42, 32, 27), (140, 32, 40)] {
        let mut shown = (0u32, total.min(capacity).min(48));
        let mut memory = Vec::new();
        let mut reached = vec![false; total as usize];
        let last_scroll = total.saturating_sub(viewport_rows);
        let stride = (last_scroll / 400).max(1);
        let mut positions: Vec<u32> = (0..=last_scroll).step_by(stride as usize).collect();
        if positions.last() != Some(&last_scroll) {
            positions.push(last_scroll);
        }
        let back: Vec<u32> = positions.iter().rev().copied().collect();
        positions.extend(back);
        let mut previous: Option<u32> = None;
        for scroll in positions {
            let mut settled = false;
            for _ in 0..16 {
                let measure = uniform("rows", total, shown.0, shown.1, -f64::from(scroll), row_px);
                let (asked, kept) = served_requests(std::slice::from_ref(&measure), f64::from(viewport_rows) * row_px, &memory, BUDGET);
                let (repeated, _) = served_requests(std::slice::from_ref(&measure), f64::from(viewport_rows) * row_px, &kept, BUDGET);
                memory = kept;
                let next = guest_answer(&asked[0], total, capacity);
                settled = next == shown && repeated == asked;
                shown = next;
                if settled {
                    break;
                }
            }
            assert!(settled, "{total}/{viewport_rows}/{capacity}: the window settles at scroll {scroll}");
            let visible = viewport_rows.min(total);
            let served_visible = (scroll..scroll + visible).filter(|row| *row >= shown.0 && *row < shown.0 + shown.1).count() as u32;
            let expected_visible = if capacity >= visible { visible } else { capacity };
            assert_eq!(served_visible, expected_visible, "{total}/{viewport_rows}/{capacity}: every visible row the guest can serve is served at scroll {scroll}");
            if scroll == last_scroll && previous.is_some_and(|previous| previous < scroll) {
                assert_eq!(shown.0 + shown.1, total, "{total}/{viewport_rows}/{capacity}: the window at the end holds the last row");
            }
            for row in shown.0..shown.0 + shown.1 {
                reached[row as usize] = true;
            }
            previous = Some(scroll);
        }
        if stride == 1 {
            assert!(reached.iter().all(|row| *row), "{total}/{viewport_rows}/{capacity}: every row is reachable");
        }
        assert!(reached[total as usize - 1], "{total}/{viewport_rows}/{capacity}: the last row is reached");
    }
}
