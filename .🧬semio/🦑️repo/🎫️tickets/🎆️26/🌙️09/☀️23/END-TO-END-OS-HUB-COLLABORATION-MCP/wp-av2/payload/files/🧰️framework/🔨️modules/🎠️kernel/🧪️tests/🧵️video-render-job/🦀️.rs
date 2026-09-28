use super::*;

/// 🧫️ `🧫️fixtures/🧵️video-render-job/🔣️.json` — folded by this law and by its TypeScript twin alike.
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧵️video-render-job/🔣️.json")).expect("video render job fixture")
}

fn events(value: &serde_json::Value) -> Vec<VideoRenderJobEvent> {
    serde_json::from_value(value.clone()).expect("fixture events decode")
}

/// ⚖️ LAW: every stream folds, event by event, into exactly the declared running rows and last job id; every event also
/// crosses the value codec (the WIT `pack`) unchanged.
#[test]
fn streams_fold_into_the_declared_task_list() {
    let fixture = fixture();
    assert_eq!(fixture["capability"], MEDIA_VIDEO_RENDER_CAPABILITY);
    for stream in fixture["streams"].as_array().expect("streams") {
        let id = stream["id"].as_str().expect("id");
        let events = events(&stream["events"]);
        let ledger = VideoRenderJobLedger::fold(&events).unwrap_or_else(|(index, error)| panic!("{id}: event {index} refused: {error}"));
        assert_eq!(serde_json::to_value(ledger.running()).expect("rows"), stream["running"], "{id}: running rows");
        assert_eq!(ledger.last_job(), stream["lastJob"].as_u64().expect("lastJob"), "{id}: last job");
        for event in &events {
            let value = dsl::ToValue::to_value(event);
            assert_eq!(&<VideoRenderJobEvent as dsl::FromValue>::from_value(value).expect("value round trip"), event, "{id}: value codec");
        }
        assert_eq!(serde_json::to_value(&events).expect("serde"), stream["events"], "{id}: camelCase wire");
    }
}

/// ⚖️ LAW: the last event of each refusal stream is refused with the declared code and leaves the ledger untouched.
#[test]
fn refused_events_leave_the_ledger_untouched() {
    for case in fixture()["refusals"].as_array().expect("refusals") {
        let id = case["id"].as_str().expect("id");
        let events = events(&case["events"]);
        let (last, head) = events.split_last().expect("at least one event");
        let mut ledger = VideoRenderJobLedger::fold(head).unwrap_or_else(|(index, error)| panic!("{id}: head event {index} refused: {error}"));
        let before = ledger.clone();
        let error = ledger.apply(last).expect_err("refused");
        assert_eq!(error.code(), case["error"].as_str().expect("error"), "{id}: {error}");
        assert_eq!(ledger, before, "{id}: a refused event changes nothing");
        assert_eq!(VideoRenderJobLedger::fold(&events).expect_err("the whole log is refused").0, head.len(), "{id}: fold names the refused index");
    }
}
