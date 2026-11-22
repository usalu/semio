//! 🎟️ Check the quote and ownership transfer against the real session producer.
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework_value_ordered::ordered::SharedOwner;
#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🎟️session/🦀️.rs"]
mod session;

fn funded<T>(d: RetirementDemand, step: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> T) -> T {
    let grant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: d.copy_bytes, maximum_capacity_bytes: d.capacity_bytes,
        maximum_release_bytes: d.release_bytes, maximum_depth: d.depth };
    semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),grant).unwrap()
        .with_effect(step).unwrap().unwrap().0
}

fn main() {
    let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!("🔣️session-live-text-cases.json")).unwrap();
    for case in cases {
        let source = case["text"].as_str().unwrap();
        let mut publication = session::SessionLivePublication::new(source);
        for phase in 0..4 {
            let turns = if phase == 1 { source.len().div_ceil(3).max(1) } else { 1 };
            for _ in 0..turns {
                let d = publication.demands(source, 3).unwrap();
                if phase == 3 {
                    let mut short = d;
                    short.capacity_bytes -= 1;
                    assert_eq!(funded(short, |effect| publication.step(source,effect)).unwrap(), Default::default());
                    assert!(!publication.ready());
                }
                funded(d, |effect| publication.step(source,effect)).unwrap();
            }
        }
        assert!(publication.ready(), "the session must publish when its own exact quote is funded");
        let mut owner = publication.take_shared().unwrap();
        let mut elected = None;
        funded(owner.retire_into_demand().unwrap(), |effect| owner.elect_native_into(&mut elected,effect)).result.unwrap();
        let mut elected = elected.unwrap();
        let mut text = None;
        funded(elected.take_value_demand().unwrap(), |effect| elected.take_value_step(&mut text,effect)).result.unwrap().unwrap();
        assert_eq!(text.as_deref(), Some(source));
        assert_eq!(serde_json::json!(text.unwrap().as_bytes()), case["utf8"]);
        assert!(owner.terminal_is_empty() && elected.terminal_is_empty());
        publication.mark_published();
        assert!(publication.published());
    }
}
