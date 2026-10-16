use super::*;

fn retire(index: &mut RetainedDictionaryIndex, grant: usize) -> usize {
    let empty = RetainedCloneProgress::default();
    let backing = index.pages[index.allocated.saturating_sub(1)].as_ref().map(|page| page.as_ptr());
    let pages_before = index.allocated_pages();
    let attempt = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: grant, maximum_depth: 1, ..Default::default() };
    let (step, requested, released) = crate::test_allocation::observe_backing(|| index.close_step(attempt).unwrap());
    assert_eq!(requested, 0);
    if pages_before != 0 && grant < PAGE_BYTES {
        assert_eq!(step, RetainedCloneStep::Progress(empty));
        assert_eq!(released, 0);
        assert_eq!(index.allocated_pages(), pages_before);
        assert_eq!(index.pages[index.allocated - 1].as_ref().map(|page| page.as_ptr()), backing);
    } else { assert_eq!(released, step.progress().released_bytes); }
    let mut bytes = step.progress().released_bytes;
    for _ in 0..MAXIMUM_PAGES + 1 {
        let demand = index.next_release_byte_demand().unwrap();
        let (step, requested, released) = crate::test_allocation::observe_backing(|| index.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand, maximum_depth: 1, ..Default::default() }).unwrap());
        assert_eq!(requested, 0);
        assert_eq!(released, step.progress().released_bytes);
        assert!(step.progress().copied_items <= 1);
        bytes += released;
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(index.terminal_is_empty()); return bytes; }
    }
    panic!("whole-page dictionary retirement failed to converge");
}

#[test]
fn retained_dictionary_range_index_never_publishes_partial_delta_and_retires_two_pages() {
    assert_eq!(size_of::<[DictionaryRange; PAGE_ENTRIES]>(), PAGE_BYTES);
    for grant in [1, 7, 4096] {
        let mut index = RetainedDictionaryIndex::new(10_000, 8192, 1048576).unwrap();
        index.begin_delta(0, 65).unwrap();
        for entry in 0..65 {
            index.append(DictionaryRange { offset: 32 + entry, length: 1 }).unwrap();
            assert_eq!(index.visible_entries(), 0);
            assert_eq!(index.lookup(entry as usize), Err(DictionaryIndexError::Malformed));
        }
        assert_eq!(index.allocated_pages(), 2);
        index.publish_delta().unwrap();
        assert_eq!(index.visible_entries(), 65);
        assert_eq!(index.lookup(64), Ok(DictionaryRange { offset: 96, length: 1 }));
        assert_eq!(index.dictionary_bytes(), 65);
        index.begin_delta(65, 2).unwrap();
        index.append(DictionaryRange { offset: 100, length: 1 }).unwrap();
        index.reject_record();
        assert_eq!(index.visible_entries(), 65);
        assert_eq!(index.lookup(0), Err(DictionaryIndexError::Malformed));
        assert_eq!(retire(&mut index, grant), 2048);
    }
}

#[test]
fn retained_dictionary_range_index_caps_all_deltas_and_retains_late_rejections() {
    for grant in [1, 7, 4096] {
        let mut index = RetainedDictionaryIndex::new(100, 3, 4).unwrap();
        index.begin_delta(0, 1).unwrap();
        index.append(DictionaryRange { offset: 32, length: 3 }).unwrap();
        index.publish_delta().unwrap();
        index.begin_delta(1, 2).unwrap();
        index.append(DictionaryRange { offset: 35, length: 1 }).unwrap();
        assert_eq!(index.append(DictionaryRange { offset: 36, length: 1 }), Err(DictionaryIndexError::Capacity));
        assert_eq!(index.visible_entries(), 1);
        assert_eq!(index.dictionary_bytes(), 4);
        assert_eq!(retire(&mut index, grant), 1024);
        for range in [DictionaryRange { offset: 99, length: 2 }, DictionaryRange { offset: u64::MAX, length: 1 }] {
            let mut index = RetainedDictionaryIndex::new(100, 1, 1048576).unwrap();
            index.begin_delta(0, 1).unwrap();
            assert_eq!(index.append(range), Err(DictionaryIndexError::Malformed));
            assert_eq!(retire(&mut index, grant), 0);
        }
        let mut index = RetainedDictionaryIndex::new(100, 1, 1048576).unwrap();
        assert_eq!(index.begin_delta(1, 1), Err(DictionaryIndexError::Malformed));
        assert_eq!(retire(&mut index, grant), 0);
    }
}

#[test]
fn dictionary_range_denied_item_depth_and_release_grants_preserve_original_backing() {
    let mut index = RetainedDictionaryIndex::new(100, 1, 100).unwrap();
    index.begin_delta(0, 1).unwrap();
    index.append(DictionaryRange { offset: 1, length: 3 }).unwrap();
    let pointer = index.pages[0].as_ref().unwrap().as_ptr();
    for grant in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: PAGE_BYTES, ..Default::default() }, RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, maximum_release_bytes: PAGE_BYTES - 1, ..Default::default() }] {
        let (result, requested, released) = crate::test_allocation::observe_backing(|| index.close_step(grant));
        if grant.maximum_items > 0 && grant.maximum_depth == 0 { assert_eq!(result.unwrap_err().kind, ValueRefusalKind::DepthLimit); }
        else { assert_eq!(result.unwrap().progress(), RetainedCloneProgress::default()); }
        assert_eq!((requested, released), (0, 0));
        assert_eq!(index.pages[0].as_ref().unwrap().as_ptr(), pointer);
        assert_eq!(index.allocated_pages(), 1);
        assert!(!index.closing);
    }
    assert_eq!(retire(&mut index, PAGE_BYTES), PAGE_BYTES);
    eprintln!("[DEBUG] dictionary index retains original pointer on all refusals and physically releases each whole page exactly");
}
