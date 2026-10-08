//! 🔗️ `set-wall-location` / `follows-by-inference`: the `wall-layout` inference follows the location line. The face offsets always add up to
//! the thickness, the interior line puts the body to the right of the axis and the exterior line to the left, and the extent never changes.

use super::SetWallLocation;
use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::testing::{close, decode, layout};
use crate::{LocationLine, ModelMutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🦚️set-wall-location/✅️moves-to-the-exterior-face/📸️snapshot/⬅️before/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_face_offsets_follow_the_location_line() {
    let before = decode(BEFORE);
    let expected = [(LocationLine::Center, 0.15, 0.15), (LocationLine::Interior, 0.0, 0.3), (LocationLine::Exterior, 0.3, 0.0), (LocationLine::CoreCenter, 0.15, 0.15)];
    for (location, left, right) in expected {
        let after = apply_model_mutation(&before, &ModelMutation::SetWallLocation(SetWallLocation { id: "w-south".into(), location })).expect("the location edit applies");
        assert_eq!(after.walls["w-south"].location, location);
        let (old, new) = (layout(&before, "w-south"), layout(&after, "w-south"));
        assert!(close(new.offset_left, left) && close(new.offset_right, right), "{location:?}");
        assert!(close(new.offset_left + new.offset_right, new.thickness));
        assert!(close(new.length, old.length) && close(new.height, old.height) && close(new.thickness, old.thickness), "{location:?}: the extent never changes");
    }
}
