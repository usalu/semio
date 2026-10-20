use semio_framework_value_control_contract::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value_paged_list::list::PagedList;
use semio_s_fem_sparse_lab::*;
use std::collections::BTreeMap;

fn lcg(state: &mut u64) -> u64 { *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); *state >> 33 }

#[test]
fn multi_page_random_triplets_match_sequence_ordered_accumulation() {
    for (n, count, seed) in [(300usize, 2500usize, 7u64), (1, 40, 3), (17, 600, 11), (1100, 5000, 5)] {
        let mut state = seed;
        let mut raw = Vec::new();
        for sequence in 0..count as u64 {
            let row = (lcg(&mut state) % n as u64) as u32;
            let col = (lcg(&mut state) % n as u64) as u32;
            let value = (lcg(&mut state) % 2001) as f64 - 1000.0 + 0.125 * (lcg(&mut state) % 8) as f64;
            raw.push(SparseTriplet { sequence, row, col, value });
        }
        let mut shuffled = raw.clone();
        for i in (1..shuffled.len()).rev() { let j = (lcg(&mut state) % (i as u64 + 1)) as usize; shuffled.swap(i, j); }
        let mut entries = PagedList::default();
        for item in &shuffled {
            while !entries.has_reserved_slot() {
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: entries.next_reserve_copy_byte_demand().unwrap(), maximum_capacity_bytes: entries.next_allocation_bytes().unwrap(), maximum_release_bytes: 0, maximum_depth: entries.next_reserve_depth_demand().unwrap() };
                entries.reserve_one_funded(grant).unwrap();
            }
            entries.push_reserved(*item).unwrap();
        }
        let mut expected: BTreeMap<(u32, u32), f64> = BTreeMap::new();
        for item in &raw { *expected.entry((item.row, item.col)).or_insert(0.0) += item.value; }
        let mut build = CsrTripletBuild::new(n, entries);
        let mut steps = 0usize;
        let mut previous = build.allocated_bytes();
        loop {
            let demand = build.next_demands().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 0, maximum_depth: demand.depth };
            let (done, progress) = build.step_one_funded(grant).unwrap();
            assert!(progress.fits(grant));
            assert_eq!(progress, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: build.allocated_bytes() - previous, released_bytes: 0 });
            assert!(build.allocated_bytes() >= previous);
            previous = build.allocated_bytes();
            steps += 1;
            if done { break; }
            assert!(steps < 5_000_000);
        }
        let demand = build.take_demands().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: demand.depth };
        let (mut matrix, _) = build.take_complete(grant).unwrap();
        assert!(matrix.ownership_is_valid());
        assert_eq!(matrix.entries_len(), expected.len());
        let mut offset = 0usize;
        for row in 0..n as u32 {
            let end = matrix.indptr(row as usize + 1).unwrap() as usize;
            let mut last = None;
            while offset < end {
                let col = matrix.index(offset).unwrap();
                assert!(last.map_or(true, |previous| col > previous), "columns strictly ascend inside a row");
                assert_eq!(matrix.value(offset).unwrap(), expected[&(row, col)], "n={n} ({row},{col})");
                last = Some(col);
                offset += 1;
            }
        }
        assert_eq!(offset, expected.len());
        let mut closed = false;
        for _ in 0..1_000_000 { if build.close_step(4096).0 { closed = true; break; } }
        assert!(closed);
        let mut closed = false;
        for _ in 0..1_000_000 { if matrix.close_step(4096).0 { closed = true; break; } }
        assert!(closed && matrix.terminal_is_empty());
        println!("n={n} entries={count} unique={} steps={steps}", expected.len());
    }
}
