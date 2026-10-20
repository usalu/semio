use semio_framework_value_control_contract::RetainedCloneGrant;
use semio_framework_value_paged_list::list::PagedList;
use semio_s_fem_sparse_lab::*;

#[test]
fn numpy_tridiagonal_order_step_count() {
    let n = 513usize;
    let mut entries = PagedList::default();
    let mut sequence = 0u64;
    for row in 0..n {
        for column in row.saturating_sub(1)..=(row + 1).min(n - 1) {
            let value = if row == column { if row == 0 || row + 1 == n { 1.0 } else { 2.0 } } else { -1.0 };
            while !entries.has_reserved_slot() { entries.reserve_one(4096).unwrap(); }
            entries.push_reserved(SparseTriplet { sequence, row: row as u32, col: column as u32, value }).unwrap();
            sequence += 1;
        }
    }
    let mut build = CsrTripletBuild::new(n, entries);
    let mut steps = 0usize;
    let mut by_stage = std::collections::BTreeMap::<String, usize>::new();
    loop {
        let demand = build.next_demands().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 0, maximum_depth: demand.depth };
        *by_stage.entry(format!("{:?}", build.stage)).or_default() += 1;
        let (done, _) = build.step_one_funded(grant).unwrap();
        steps += 1;
        if done { break; }
    }
    println!("steps={steps} {by_stage:?}");
    let (matrix, _) = build.take_complete(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 }).unwrap();
    assert_eq!(matrix.entries_len(), 1537);
}
