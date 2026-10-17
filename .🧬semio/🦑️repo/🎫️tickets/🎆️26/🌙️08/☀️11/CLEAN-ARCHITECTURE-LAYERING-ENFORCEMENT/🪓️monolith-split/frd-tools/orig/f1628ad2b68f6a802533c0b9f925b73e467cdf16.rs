use semio_framework_value::{DslValue,FromValue,ToValue};

#[semio_framework_async_macros::async_test]
async fn history_branch_provenance_follows_portable_required_wire_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🔨️modules/🌿️vcs/🧪️tests/🧫️fixtures/🌿️branch-provenance/🔣️.json")).unwrap();
    for vector in fixture["vectors"].as_array().unwrap() {
        let decoded = crate::os_spr::Edit::<DslValue>::from_value(vector["edit"].clone().into());
        assert_eq!(decoded.is_ok(), vector["valid"].as_bool().unwrap(), "{}", vector["id"]);
        let Ok(edit) = decoded else { continue };
        let actual: serde_json::Value = edit.to_value().into();
        assert_eq!(actual["line"], vector["edit"]["line"]);
        let mut log = crate::os_spr::HistoryLog::default();
        log.doc_id = "document-1".into();
        log.schema = "neutral.counter.v1".into();
        log.edits.push(crate::os_spr::HistoryEdit {
            id: edit.id.clone(), actor: None, line: edit.line.clone(), started_at: edit.started_at,
            finished_at: None, verb: None,
            ops: Vec::new(), inverse: Vec::new(), meta: None, lane: None,
        });
        let text = crate::os_spr::history::print_ops_text(&log).unwrap();
        let text_twin = crate::os_spr::history::parse_ops_text(&text).unwrap();
        assert_eq!(text_twin.edits[0].line, edit.line);
        let binary = crate::os_spr::encode_history(&log, &crate::os_spr::EncodeOptions::default()).await.unwrap();
        let binary_twin = crate::os_spr::decode_history(&binary, &crate::os_spr::DecodeOptions::default()).await.unwrap();
        assert_eq!(binary_twin.edits[0].line, edit.line);
        let start = text.find(" line=[").unwrap();
        let end = start + text[start..].find(']').unwrap() + 1;
        let mut missing = text.clone();
        missing.replace_range(start..end, "");
        assert!(crate::os_spr::history::parse_ops_text(&missing).is_err());
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
