use crate::design::DesignStore;
use crate::diff::DesignDiff;

#[test]
fn diff_between_identical_designs_empty() {
    let a = DesignStore::new("d");
    let b = DesignStore::new("d");
    let da = a.to_full_dto();
    let db = b.to_full_dto();
    let d = DesignDiff::between(&da, &db);
    assert!(d.added_pieces.is_empty());
    assert!(d.removed_pieces.is_empty());
    assert!(d.modified_pieces.is_empty());
    assert!(d.added_connections.is_empty());
    assert!(d.removed_connections.is_empty());
    assert!(d.modified_connections.is_empty());
}
