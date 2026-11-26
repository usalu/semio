//! 🧪️ Checks physically funded layout caps on a nested clone with Puzzle 2D carrier shapes.
use semio_framework_value_retained_clone::*;
use semio_framework_value_derive::{RetainedClone, RetireOwned};
use semio_framework_value_paged::paged::PagedUtf8;
use semio_framework_value_persistent::list::PagedList;
use semio_framework_value_testing_allocator::observe_retirement_allocations as heap;
use std::mem::size_of;

#[path = "../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
#[path = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🧪️tests/🔗️source-custody/🦀️.rs"]
mod custody;

type Text = PagedUtf8<{ usize::MAX }>;
type Rows<T> = PagedList<T, { usize::MAX }>;

#[derive(Debug, PartialEq, serde::Serialize, RetainedClone, RetireOwned)]
#[value(crate = "semio_framework_value_retained_clone")]
struct Handle {
    id: Text,
    label: Option<Text>,
}

#[derive(Debug, PartialEq, serde::Serialize, RetainedClone, RetireOwned)]
#[value(crate = "semio_framework_value_retained_clone")]
struct Node {
    id: Text,
    text: Option<Text>,
    handles: Rows<Handle>,
    measurement: [f64; 4],
}

fn exact(axis: &str, demand: RetirementDemand) -> RetainedCloneGrant {
    assert!(demand.copy_bytes + demand.capacity_bytes + demand.release_bytes <= 4096, "{axis}: {demand:?}");
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}

fn driven<T: RetainedClone + Sync + PartialEq + std::fmt::Debug + 'static>(source: T, cancel: bool) {
    let (source, _, _) = effect::admit(source);
    let mut cursor = RetainedFieldCursor::<T>::default();
    let mut peak = 0;
    for turn in 0..10_000 {
        if cursor.output_ready() || cancel && turn == 120 {
            break;
        }
        let demand = cursor.advance_demands(source.borrow(), 4096, 4096).unwrap();
        peak = peak.max(demand.copy_bytes + demand.capacity_bytes + demand.release_bytes);
        let grant = exact(&format!("advance {turn}"), demand);
        if grant.maximum_capacity_bytes != 0 {
            let short = RetainedCloneGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes - 1, ..grant };
            let (denied, h) = heap(|| effect::advance(&mut cursor, source.borrow(), 4096, 4096, short));
            assert_eq!(denied.work, 0);
            assert_eq!(denied.result.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!(h, (0, 0));
        }
        let (result, h) = heap(|| effect::advance(&mut cursor, source.borrow(), 4096, 4096, grant));
        let p = result.result.unwrap().progress();
        assert!(p.fits(grant));
        assert_eq!(h, (p.retained_capacity_bytes, p.released_bytes));
    }
    assert_eq!(cursor.output_ready(), !cancel);
    let mut output = if cancel {
        None
    } else {
        let grant = exact("take", cursor.take_demand().unwrap());
        let (turn, h) = heap(|| effect::take(&mut cursor, grant));
        let (value, p) = turn.result.unwrap().unwrap();
        assert_eq!(&value, source.borrow().get());
        assert!(p.fits(grant));
        assert_eq!(h, (p.retained_capacity_bytes, p.released_bytes));
        Some(value)
    };
    let _ = effect::begin_close(&mut cursor);
    for turn in 0..10_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        let demand = cursor.close_demands(4096).unwrap();
        peak = peak.max(demand.copy_bytes + demand.capacity_bytes + demand.release_bytes);
        let grant = exact(&format!("close {turn}"), demand);
        if grant.maximum_release_bytes != 0 {
            let short = RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant };
            let (denied, h) = heap(|| effect::close_step(&mut cursor, short));
            assert_eq!(denied.work, 0);
            assert_eq!(denied.result.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!(h, (0, 0));
        }
        let (result, h) = heap(|| effect::close_step(&mut cursor, grant));
        let p = result.result.unwrap().progress();
        assert!(p.fits(grant));
        assert_eq!(h, (p.retained_capacity_bytes, p.released_bytes));
    }
    assert!(cursor.terminal_is_empty());
    let mut retirement = RetainedCloneClose::default();
    for turn in 0..10_000 {
        if output.is_none() && retirement.is_empty() {
            break;
        }
        let demand = retirement.next_owner_demands::<T>(output.is_some(), &None, 4096).unwrap();
        peak = peak.max(demand.copy_bytes + demand.capacity_bytes + demand.release_bytes);
        let grant = exact(&format!("retire output {turn}"), demand);
        let (result, h) = heap(|| {
            if retirement.is_empty() {
                effect::begin_granted(&mut retirement, &mut output, grant).result.unwrap().unwrap().progress()
            } else {
                effect::step_granted(&mut retirement, grant).result.unwrap().progress()
            }
        });
        assert!(result.fits(grant));
        assert_eq!(h, (result.retained_capacity_bytes, result.released_bytes));
    }
    assert!(output.is_none() && retirement.is_empty());
    eprintln!("[DEBUG] clone peak bytes {peak}, cancelled {cancel}");
    let mut source = source;
    for _ in 0..10_000 {
        if source.terminal_is_empty() {
            break;
        }
        effect::driven(effect::STOCK, |effect| source.close_step(4096, effect)).0.result.unwrap();
    }
    assert!(source.terminal_is_empty());
}

fn state() -> Rows<Node> {
    let text = "Grüße 🧩".repeat(1024);
    [Node {
        id: text.as_str().into(),
        text: Some(text.as_str().into()),
        handles: [Handle {id: text.as_str().into(), label: Some(text.as_str().into())}].into_iter().collect(),
        measurement: [1.0, 2.0, 3.0, 4.0],
    }].into_iter().collect()
}

#[test]
fn cloned_and_cancelled_nested_payload_uses_exact_turns() {
    eprintln!("[DEBUG] cursor sizes nodes={} node={} handle={} text={} optional_text={} string={}",
        size_of::<<Rows<Node> as RetainedClone>::Cursor>(), size_of::<<Node as RetainedClone>::Cursor>(), size_of::<<Handle as RetainedClone>::Cursor>(), size_of::<<Text as RetainedClone>::Cursor>(),
        size_of::<<Option<Text> as RetainedClone>::Cursor>(), size_of::<<String as RetainedClone>::Cursor>());
    driven(state(), false);
    driven(state(), true);
    driven(Some(Text::from("optional 🧩".repeat(1024))), false);
    driven(None::<Text>, false);
}

#[test]
fn puzzle_2d_custody_keeps_the_original_work_source_through_paged_cancel() {
    let input = state();
    let mut source = custody::admit();
    let mut cursor = RetainedFieldCursor::<Rows<Node>>::default();
    for turn in 0..120 {
        let original = source.borrow(&input);
        let grant = exact(&format!("advance {turn}"), cursor.advance_demands(original, 4096, 4096).unwrap());
        let step = custody::run(grant, |effect| {
            let mut reply = None;
            cursor.advance_into(original, 4096, 4096, effect, OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap();
            reply.unwrap().unwrap()
        });
        assert!(step.progress().fits(grant));
        assert!(!cursor.output_ready());
    }
    custody::begin_close(&mut cursor);
    custody::close_cursor(
        &mut cursor,
        10_000,
        |cursor| {
            let demand = cursor.close_demands(0).unwrap();
            let capacity = cursor.close_demands(demand.copy_bytes).unwrap().capacity_bytes;
            (demand.copy_bytes, capacity, demand.release_bytes, demand.depth)
        },
        |cursor, effect| cursor.close_selected(effect.grant().maximum_copy_bytes, effect),
        |cursor| cursor.terminal_is_empty(),
    );
    custody::close(&mut source);
}
