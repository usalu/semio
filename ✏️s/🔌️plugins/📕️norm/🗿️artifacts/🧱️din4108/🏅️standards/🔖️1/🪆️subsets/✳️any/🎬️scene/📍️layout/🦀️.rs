//! 📍️ Layout: where every entity stands. The documented default rule (golden-tested): envelope elements stand as upright panels along `+X` with a 1 m gap on the line `y = 0`, thermal zones as floor plates in the row `y = ZONE_ROW`, thermal bridges as lines in the row `y = BRIDGE_ROW`.

use super::kit::{member_frame, v3};
use super::model::Din4108Snapshot;
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub const PANEL_GAP: f64 = 1.0;
pub const ZONE_ROW: f64 = 8.0;
pub const BRIDGE_ROW: f64 = -4.0;
pub const MIN_SIDE: f64 = 1.0;
pub const MAX_SIDE: f64 = 10.0;

/// 📏️ The side of the square panel that draws `area_m2`: its square root within the drawable range.
pub fn side_of(area_m2: f64) -> f64 {
    area_m2.max(0.0).sqrt().clamp(MIN_SIDE, MAX_SIDE)
}

/// 📍️ One resolved pose: frame at the start of the entity and the extent along `+X`.
#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub frame: Frame,
    pub length: f64,
}

/// 📍️ All resolved poses of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    poses: BTreeMap<String, Pose>,
}

impl Layout {
    pub fn of(document: &Din4108Snapshot) -> Self {
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for element in &document.elements {
            let side = side_of(element.area_m2);
            layout.poses.insert(element.id.clone(), Pose { frame: member_frame(&v3(cursor, 0.0, 0.0), &Vec3::X), length: side });
            cursor += side + PANEL_GAP;
        }
        cursor = 0.0;
        for zone in &document.zones {
            let side = side_of(zone.floor_area_m2);
            layout.poses.insert(zone.id.clone(), Pose { frame: member_frame(&v3(cursor, ZONE_ROW, 0.0), &Vec3::X), length: side });
            cursor += side + PANEL_GAP;
        }
        cursor = 0.0;
        for bridge in &document.thermal_bridges {
            let length = bridge.length_m.clamp(MIN_SIDE, MAX_SIDE);
            layout.poses.insert(bridge.id.clone(), Pose { frame: member_frame(&v3(cursor, BRIDGE_ROW, 0.0), &Vec3::X), length });
            cursor += length + PANEL_GAP;
        }
        layout
    }

    pub fn pose(&self, id: &str) -> Option<&Pose> {
        self.poses.get(id)
    }

    pub fn poses(&self) -> impl Iterator<Item = (&String, &Pose)> {
        self.poses.iter()
    }
}
