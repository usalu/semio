//! 🧪 Flat bytes, original source selection, and paid ownership fixtures.

use std::mem::size_of;

use norm_group_probe::{Mutation, Original, descriptors, select};
use paid::{Budget, Refusal};

const INTERLEAVED: &str = include_str!("../cases.tsv");

#[test]
fn canonical_bytes_and_both_ordinal_spaces() {
    for (native, row) in INTERLEAVED.lines().enumerate() {
        let fields: Vec<_> = row.split('\t').collect();
        let [tag, wire, number, canonical] = fields[..] else { panic!("{row}") };
        let raw = number.parse::<u16>().unwrap();
        assert_eq!(descriptors()[native].tag, tag);
        assert_eq!(descriptors()[native].number, wire.parse::<u16>().unwrap());
        let authored = [(tag, number)];
        for original in [Original::Direct(tag, raw), Original::Carrier(&authored)] {
            let budget = Budget::new(3, 32);
            let selection = select(original, &budget).unwrap();
            let pending = selection.receive().unwrap();
            let slot = pending.publish().unwrap();
            let value = *slot.value();
            assert_eq!(value.write_json().as_bytes(), canonical.as_bytes());
            assert_eq!(value.inverse(raw - 1).inverse(raw), value);
            assert_eq!(budget.observed().3, 1);
            assert_eq!(budget.observed().4, 1);
            drop(slot);
            assert_eq!(budget.observed().1, 0);
            assert_eq!(budget.observed().3, 0);
            assert_eq!(budget.observed().0, 0);
        }
    }
}

#[test]
fn cancelled_and_refused_slots_return_their_budget() {
    let original = || Original::Direct("ChangeDeckSpan", 17);
    let leaf = size_of::<deck::Leaf>();
    let published = size_of::<Mutation>();

    let budget = Budget::new(3, leaf + published);
    {
        let _selection = select(original(), &budget).unwrap();
        assert_eq!(budget.observed(), (2, 0, 0, 0, 1));
    }
    assert_eq!(budget.observed(), (2, 0, 0, 0, 1));
    let pending = select(original(), &budget).unwrap().receive().unwrap();
    assert_eq!(budget.observed(), (0, leaf, leaf, 1, 2));
    drop(pending);
    assert_eq!(budget.observed(), (0, 0, leaf, 0, 2));

    let budget = Budget::new(3, leaf + published - 1);
    let pending = select(original(), &budget).unwrap().receive().unwrap();
    assert_eq!(pending.publish().err(), Some(Refusal::Capacity));
    assert_eq!(budget.observed(), (0, 0, leaf, 0, 1));

    let budget = Budget::new(2, leaf + published);
    let pending = select(original(), &budget).unwrap().receive().unwrap();
    assert_eq!(pending.publish().err(), Some(Refusal::Work));
    assert_eq!(budget.observed(), (0, 0, leaf, 0, 1));

    let budget = Budget::new(3, leaf + published);
    let slot = select(original(), &budget).unwrap().receive().unwrap().publish().unwrap();
    assert_eq!(budget.observed(), (0, published, leaf + published, 1, 1));
    drop(slot);
    assert_eq!(budget.observed(), (0, 0, leaf + published, 0, 1));
}

#[test]
fn malformed_originals_never_publish() {
    let budget = Budget::new(4, 32);
    let raw = [("ChangeDeckSpan", "17"), ("ChangeDeckSpan", "18")];
    assert_eq!(select(Original::Carrier(&raw), &budget).err(), Some(Refusal::Shape));
    assert_eq!(select(Original::Direct("Deck", 17), &budget).err(), Some(Refusal::UnknownTag));
    let malformed = [("ChangeDeckSpan", "wrong")];
    assert_eq!(
        select(Original::Carrier(&malformed), &budget).unwrap().receive().err(),
        Some(Refusal::Payload),
    );
    assert_eq!(budget.observed(), (0, 0, size_of::<deck::Leaf>(), 0, 3));
}
