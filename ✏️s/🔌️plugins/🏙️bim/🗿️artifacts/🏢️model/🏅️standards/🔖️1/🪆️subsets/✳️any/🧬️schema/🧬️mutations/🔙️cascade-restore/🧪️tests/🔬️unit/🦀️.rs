//! 🧪️ The restoring list of the aggregate covers every collection the removal closure can name: for each cascading delete case the rows the aggregate states equal the rows the closure counts.

use crate::standards::v1::subsets::any::schema::mutations::cascade::{closure, RemovalInverse};
use crate::standards::v1::subsets::any::schema::mutations::{kit, ModelMutation};

#[test]
fn restores_exactly_the_rows_the_removal_counts() {
    let mut checked = 0;
    for case in kit::cases().iter().filter(|case| case.dir.contains("cascades")) {
        let base = kit::before(case);
        for root in base.sites.keys().chain(base.buildings.keys()).chain(base.storeys.keys()) {
            if let Ok(removal) = closure(&base, std::slice::from_ref(root)) {
                let rows = <ModelMutation as RemovalInverse>::removal_inverse(&removal, &base).len();
                assert_eq!(rows, removal.rows(&base), "{}: restoring rows of {root}", case.dir);
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "no cascading case was checked");
}
