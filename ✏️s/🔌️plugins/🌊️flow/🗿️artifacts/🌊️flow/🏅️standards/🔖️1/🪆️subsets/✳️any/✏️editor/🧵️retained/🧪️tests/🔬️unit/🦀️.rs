use super::*;
use semio_framework_job::InteractiveJobCloseStep as Step;
use semio_framework_plugin::retained_command::ArtifactCommandWork;
use semio_framework_value::RetainedCloneGrant;

fn quoted_grant(work: &super::super::FlowDirectStoreWork, body: usize) -> RetainedCloneGrant {
    let copy = work.next_close_copy_byte_demand().expect("quoted copy demand").max(body);
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: work.next_close_capacity_byte_demand(copy).expect("quoted capacity demand"), maximum_release_bytes: work.next_close_release_byte_demand().expect("quoted release demand"), maximum_depth: work.next_close_depth_demand().expect("quoted depth demand").max(1) }
}

fn drain(work: &mut super::super::FlowDirectStoreWork, body: usize) {
    for turn in 0..100_000 {
        if work.terminal_is_empty() {
            return;
        }
        let grant = quoted_grant(work, body);
        match work.close_step(grant) {
            Step::Pending { progress } => assert!(progress.fits(grant)),
            Step::Complete { progress } => {
                assert!(progress.fits(grant));
                return;
            }
            other => panic!("unexpected direct close result at turn {turn}: {other:?}"),
        }
    }
    panic!("direct close did not reach its terminal-empty witness");
}

#[test]
fn direct_preview_close_finishes_with_quoted_grants() {
    for (value, body) in [("tiny".to_owned(), 4096), ("🌊".repeat(4096), 4096), ("🌊".repeat(4096), 1)] {
        let mut work = super::super::FlowDirectStoreWork::new("setPreviewOff");
        work.preview_off = Some(vec![value]);
        work.begin_close();
        drain(&mut work, body);
        assert!(work.terminal_is_empty());
    }
}

#[test]
fn restore_never_drops_live_retained_owners() {
    let mut work = super::super::FlowDirectStoreWork::new("setPreviewOff");
    work.preview_off = Some(vec!["owned".into()]);
    assert!(work.restore(&[0; 34]).is_err());
    assert_eq!(work.preview_off.as_ref().unwrap(), &["owned"]);
    work.begin_close();
    drain(&mut work, 1);
    assert!(work.terminal_is_empty());
}

#[test]
fn retirement_obeys_language_neutral_grants_and_reaches_terminal_empty() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
        let body = row["grantBytes"].as_u64().unwrap() as usize;
        let mut retirement = Retirement::default();
        retirement.push(vec![text]);
        let quoted = |retirement: &Retirement, items: usize| {
            let copy = retirement.copy_byte_demand().unwrap().max(body);
            RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: copy, maximum_capacity_bytes: retirement.capacity_byte_demand(copy).unwrap(), maximum_release_bytes: retirement.release_byte_demand().unwrap(), maximum_depth: retirement.depth_demand().unwrap().max(1) }
        };
        let zero = quoted(&retirement, 0);
        assert!(matches!(retirement.step(zero), Step::Pending { progress } if progress == Default::default()));
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 100_000);
            let grant = quoted(&retirement, 1);
            match retirement.step(grant) {
                Step::Pending { progress } => assert!(progress.fits(grant)),
                Step::Complete { progress } => {
                    assert!(progress.fits(grant));
                    break;
                }
                other => panic!("unexpected retirement result: {other:?}"),
            }
        }
        assert!(retirement.is_empty());
    }
}
