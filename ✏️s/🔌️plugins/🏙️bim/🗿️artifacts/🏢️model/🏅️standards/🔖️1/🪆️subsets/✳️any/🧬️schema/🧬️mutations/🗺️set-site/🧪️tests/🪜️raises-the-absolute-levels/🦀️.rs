//! 🪜️ `set-site` / `raises-the-absolute-levels`: inference level. The site elevation is a datum of the parametric chain: changing it moves the
//! absolute elevation of every storey it carries by the same amount (`storey-levels`) and leaves the building-relative elevations alone.

use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::standards::v1::subsets::any::schema::mutations::kit::{self, Case};
use protocol::Mutation;

const CASE: Case = Case {
    dir: "🗺️set-site/✅️moves-the-site",
    before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️set-site/✅️moves-the-site/📸️snapshot/⬅️before/🔣️.json"),
    after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️set-site/✅️moves-the-site/📸️snapshot/➡️after/🔣️.json"),
    mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️set-site/✅️moves-the-site/🦠️mutation/🔣️.json"),
    diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️set-site/✅️moves-the-site/🔺️diff/🔣️.json"),
    outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️set-site/✅️moves-the-site/🎯️outcome/🔣️.json"),
};

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

#[semio_framework_async_macros::async_test]
async fn the_site_elevation_moves_every_absolute_storey_elevation() {
    let before = kit::before(&CASE);
    let (diff, _) = kit::mutation(&CASE).diff(&before).into_parts();
    let after = protocol::apply_diff(&diff, &before).expect("the diff applies");
    let raised = after.sites["site-1"].elevation - before.sites["site-1"].elevation;
    assert!(raised.abs() > 0.0, "the case changes the site elevation");
    let (low, high) = (compute_storey_levels(&before), compute_storey_levels(&after));
    assert!(!low.is_empty() && low.len() == high.len(), "the scene carries storeys");
    for (id, level) in &low {
        let next = high[id];
        assert!(close(next.absolute_elevation - level.absolute_elevation, raised), "{id}: absolute elevation follows the site elevation");
        assert!(close(next.absolute_top_elevation - level.absolute_top_elevation, raised), "{id}: absolute top elevation follows the site elevation");
        assert_eq!(next.elevation, level.elevation, "{id}: the building-relative elevation is untouched");
        assert_eq!(next.top_elevation, level.top_elevation, "{id}: the building-relative top elevation is untouched");
    }
}
