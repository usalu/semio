use super::*;
use semio_framework_job::InteractiveJobCloseStep as Step;
use semio_framework_plugin::retained_command::ArtifactCommandWork;

#[test]
fn config_copy_and_targeted_inverse_obey_maximum_text_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
        let grant = row["grantBytes"].as_u64().unwrap() as usize;
        let base = FlowConfig::default();
        let mutation = FlowConfigMutation::SetCatalogueSections { sections_json: text.clone() };
        let source = ConfigSource::post(&base, &mutation, false);
        let mut copy = ConfigCopy::new(&source, None);
        let mut copied = 0;
        for _ in 0..text.len() + 100 {
            let count = copy.step(&source, grant).unwrap();
            assert!(count <= grant);
            copied += count;
            if copy.complete() {
                break;
            }
        }
        let post = copy.take().unwrap();
        assert_eq!(post, FlowConfig { catalogue_sections_json: text, ..base.clone() });
        assert_eq!(copied, super::super::flow_config_text_bytes(&post));
        let source = ConfigSource::base(&base);
        let mut undo = ConfigCopy::new(&source, inverse_field(&mutation));
        for _ in 0..100 {
            undo.step(&source, grant).unwrap();
            if undo.complete() {
                break;
            }
        }
        assert_eq!(inverse(&mutation, undo.take().unwrap()), FlowConfigMutation::SetCatalogueSections { sections_json: base.catalogue_sections_json });
    }
}

#[test]
fn direct_preview_close_finishes_with_production_grant_and_exact_bytes() {
    for (value, grant) in [("tiny".to_owned(), 4096), ("🌊".repeat(4096), 4096), ("🌊".repeat(4096), 1)] {
        let expected = value.len();
        let mut work = super::super::FlowDirectStoreWork::new("setPreviewOff");
        work.preview_off = Some(vec![value]);
        work.begin_close();
        let mut released = 0;
        for _ in 0..expected + 10 {
            match work.close_step(1, grant) {
                Step::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= grant);
                    released += released_bytes;
                }
                Step::Complete => break,
                other => panic!("unexpected direct close result: {other:?}"),
            }
        }
        assert!(work.terminal_is_empty());
        assert_eq!(released, expected);
    }
}

#[test]
fn restore_never_drops_live_retained_owners() {
    let mut work = super::super::FlowDirectStoreWork::new("setPreviewOff");
    work.preview_off = Some(vec!["owned".into()]);
    assert!(work.restore(&[0; 17]).is_err());
    assert_eq!(work.preview_off.as_ref().unwrap(), &["owned"]);
    work.begin_close();
    for _ in 0..20 {
        if matches!(work.close_step(1, 1), Step::Complete) {
            break;
        }
    }
    assert!(work.terminal_is_empty());
}

#[test]
fn retirement_obeys_language_neutral_grants_and_releases_exact_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
        let expected = row["expectedTextBytes"].as_u64().unwrap() as usize;
        let grant = row["grantBytes"].as_u64().unwrap() as usize;
        let mut retirement = Retirement::default();
        retirement.push(Owner::Strings(vec![text]));
        assert!(matches!(retirement.step(0, grant), Step::Blocked));
        assert!(matches!(retirement.step(1, 0), Step::Blocked));
        let mut released = 0;
        let mut steps = 0;
        loop {
            steps += 1;
            assert!(steps < expected + 10);
            match retirement.step(1, grant) {
                Step::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= grant);
                    released += released_bytes;
                }
                Step::Complete => break,
                other => panic!("unexpected retirement result: {other:?}"),
            }
        }
        assert_eq!(released, expected);
        assert!(retirement.is_empty());
    }
}

#[test]
fn nested_dictionary_moves_without_cloning_and_retires_at_one_byte() {
    let text = "🌊".repeat(4096);
    let dictionary = neural::Dictionary::new().insert("key", neural::Value::Dictionary(neural::Dictionary::new().insert("nested", neural::Value::Atom(neural::Atom::String(text)))));
    let mut retirement = Retirement::default();
    retirement.push(Owner::Dictionary(dictionary));
    let mut bytes = 0;
    for _ in 0..16410 {
        match retirement.step(1, 1) {
            Step::Pending { released_bytes, .. } => bytes += released_bytes,
            Step::Complete => break,
            other => panic!("unexpected retirement result: {other:?}"),
        }
    }
    assert!(retirement.is_empty());
    assert_eq!(bytes, 16384 + 3 + 6);
}
