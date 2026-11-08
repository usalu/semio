//! 📍️ Layout: where every entity stands. The documented default rule (golden-tested): rooms stand side by side along `+X` on the line `y = 0` with a gap of [`ROOM_GAP`], every room [`ROOM_DEPTH`] deep and `floor area / depth` wide, [`ROOM_HEIGHT`] high; ventilation systems stand as boxes along `+X` in the row `y = VENT_ROW`, `design airflow / AIRFLOW_PER_METRE` wide.

use super::kit::{member_frame, v3};
use super::model::Din16798Snapshot;
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub const ROOM_GAP: f64 = 2.0;
pub const ROOM_DEPTH: f64 = 10.0;
pub const ROOM_HEIGHT: f64 = 3.0;
pub const MIN_WIDTH: f64 = 2.0;
pub const MAX_WIDTH: f64 = 30.0;
pub const VENT_ROW: f64 = -8.0;
pub const VENT_DEPTH: f64 = 2.0;
pub const VENT_HEIGHT: f64 = 1.5;
pub const VENT_GAP: f64 = 1.5;
pub const AIRFLOW_PER_METRE: f64 = 500.0;
pub const MIN_VENT_WIDTH: f64 = 1.0;
pub const MAX_VENT_WIDTH: f64 = 12.0;

/// 📏️ The width of the room that draws `area_m2`: its area over the fixed depth within the drawable range.
pub fn room_width(area_m2: f64) -> f64 {
    (area_m2.max(0.0) / ROOM_DEPTH).clamp(MIN_WIDTH, MAX_WIDTH)
}

/// 📏️ The width of the box that draws a design airflow of `airflow_m3_h`.
pub fn vent_width(airflow_m3_h: f64) -> f64 {
    (airflow_m3_h.max(0.0) / AIRFLOW_PER_METRE).clamp(MIN_VENT_WIDTH, MAX_VENT_WIDTH)
}

/// 📍️ One resolved pose: frame at the lower corner of the entity and the extent along `+X`.
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
    pub fn of(document: &Din16798Snapshot) -> Self {
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for zone in &document.zones {
            let width = room_width(zone.floor_area_m2);
            layout.poses.insert(zone.id.clone(), Pose { frame: member_frame(&v3(cursor, 0.0, 0.0), &Vec3::X), length: width });
            cursor += width + ROOM_GAP;
        }
        cursor = 0.0;
        for vent in &document.vent_systems {
            let width = vent_width(vent.design_airflow_m3_h);
            layout.poses.insert(vent.id.clone(), Pose { frame: member_frame(&v3(cursor, VENT_ROW, 0.0), &Vec3::X), length: width });
            cursor += width + VENT_GAP;
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
