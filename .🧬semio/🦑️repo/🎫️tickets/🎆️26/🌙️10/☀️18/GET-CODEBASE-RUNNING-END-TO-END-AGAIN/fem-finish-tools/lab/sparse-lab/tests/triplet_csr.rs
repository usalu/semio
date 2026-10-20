use semio_framework_value_control_contract::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value_paged_list::list::PagedList;
use semio_s_fem_sparse_lab::*;

const FIXTURE: &str = "@REPO@/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧫️fixtures/🧮️triplet-csr/🔣️.json";

#[test]
fn original_triplet_csr_matches_portable_sparse_matrix_and_closes_every_page() {
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(FIXTURE).unwrap()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut entries = PagedList::default();
        for entry in case["entries"].as_array().unwrap() {
            while !entries.has_reserved_slot() {
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: entries.next_reserve_copy_byte_demand().unwrap(), maximum_capacity_bytes: entries.next_allocation_bytes().unwrap(), maximum_release_bytes: 0, maximum_depth: entries.next_reserve_depth_demand().unwrap() };
                entries.reserve_one_funded(grant).unwrap();
            }
            entries.push_reserved(SparseTriplet { sequence: entry["sequence"].as_u64().unwrap(), row: entry["row"].as_u64().unwrap() as u32, col: entry["column"].as_u64().unwrap() as u32, value: entry["value"].as_f64().unwrap() }).unwrap();
        }
        let n = case["order"].as_u64().unwrap() as usize;
        let mut build = CsrTripletBuild::new(n, entries);
        let mut complete = false;
        for _ in 0..4096 {
            let demand = build.next_demands().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 0, maximum_depth: demand.depth };
            let before = (build.stage, build.allocated_bytes(), build.sort);
            if build.stage != CsrTripletBuildStage::Complete {
                for axis in fixture["deniedAxes"].as_array().unwrap() {
                    let axis = axis.as_str().unwrap();
                    if axis == "capacityBytes" && demand.capacity_bytes == 0 { continue; }
                    let mut denied = grant;
                    match axis { "items" => denied.maximum_items = 0, "copyBytes" => denied.maximum_copy_bytes -= 1, "capacityBytes" => denied.maximum_capacity_bytes -= 1, _ => denied.maximum_depth -= 1 }
                    assert_eq!(build.step_one_funded(denied).unwrap(), (false, RetainedCloneProgress::default()));
                    assert_eq!((build.stage, build.allocated_bytes(), build.sort), before);
                }
            }
            let (done, progress) = build.step_one_funded(grant).unwrap();
            assert!(progress.fits(grant));
            assert_eq!(progress.retained_capacity_bytes, build.allocated_bytes() - before.1);
            if done { complete = true; break; }
        }
        assert!(complete, "{} bounded completion", case["id"]);
        let demand = build.take_demands().expect("original matrix output demand");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: demand.depth };
        let before = build.matrix.as_ref().map(|matrix| (matrix as *const Csr, matrix.allocated_bytes()));
        let mut denied = grant;
        denied.maximum_copy_bytes -= 1;
        assert!(build.take_complete(denied).is_none());
        assert_eq!(build.matrix.as_ref().map(|matrix| (matrix as *const Csr, matrix.allocated_bytes())), before);
        let (mut matrix, progress) = build.take_complete(grant).expect("funded original matrix output");
        assert!(progress.fits(grant));
        for row in 0..n {
            for column in 0..n {
                let mut actual = 0.0;
                for offset in matrix.indptr(row).unwrap() as usize..matrix.indptr(row + 1).unwrap() as usize {
                    if matrix.index(offset).unwrap() as usize == column { actual += matrix.value(offset).unwrap(); }
                }
                assert_eq!(actual, case["matrix"][row * n + column].as_f64().unwrap());
            }
        }
        let mut closed = false;
        for _ in 0..4096 { if build.close_step(4096).0 { closed = true; break; } }
        assert!(closed);
        let mut closed = false;
        for _ in 0..4096 { if matrix.close_step(4096).0 { closed = true; break; } }
        assert!(closed && matrix.terminal_is_empty());
    }
}
