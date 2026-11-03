//! 📍️ Layout: where every spatial entity stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! beams run along `+X` stacked in `+Y` rows one tributary width plus 2 m apart, columns stand in a row at `y = -3 m` every 2 m, slab strips (1 m wide) stack towards `-Y` from `y = -8 m` every 2.5 m.

use super::kit::{add, mul, v3};
use super::checks::case_catalogue::En1994Cases;
use super::model::En1994Snapshot;
use super::scene::projection_kit::design_case::forms;
use super::scene::Vec3;
use std::collections::BTreeMap;

pub use super::scene::projection_kit::design_case::forms::CasePose;
pub use super::scene::projection_kit::Placement;

pub const ROW_GAP: f64 = 2.0;
pub const COLUMN_ROW: f64 = -3.0;
pub const COLUMN_PITCH: f64 = 2.0;
pub const SLAB_ROW: f64 = -8.0;
pub const SLAB_PITCH: f64 = 2.5;
pub const STRIP_WIDTH: f64 = 1.0;
pub const CASE_GAP: f64 = 10.0;

/// 📍️ One resolved pose: origin and yaw about `+Z`; all entity geometry is built in the local frame and mapped through it.
#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub origin: Vec3,
    pub yaw: f64,
}

impl Pose {
    pub fn dir(&self, x: f64, y: f64, z: f64) -> Vec3 {
        let (s, c) = self.yaw.sin_cos();
        v3(x * c - y * s, x * s + y * c, z)
    }

    pub fn point(&self, x: f64, y: f64, z: f64) -> Vec3 {
        add(&self.origin, &self.dir(x, y, z))
    }

    pub fn ex(&self) -> Vec3 {
        self.dir(1.0, 0.0, 0.0)
    }

    pub fn ey(&self) -> Vec3 {
        self.dir(0.0, 1.0, 0.0)
    }

    pub fn shifted(&self, along_x: f64) -> Vec3 {
        add(&self.origin, &mul(&self.ex(), along_x))
    }
}

/// 📍️ All resolved poses of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    poses: BTreeMap<String, Pose>,
    cases: BTreeMap<String, CasePose>,
}

impl Layout {
    pub fn of(document: &En1994Snapshot, placements: &[Placement]) -> Self {
        let placed = |id: &str| placements.iter().find(|p| p.entity_id == id);
        let mut poses = BTreeMap::new();
        let mut cursor = 0.0;
        for beam in &document.beams {
            let default = Pose { origin: v3(0.0, cursor + beam.spacing_m / 2.0, 0.0), yaw: 0.0 };
            cursor += beam.spacing_m + ROW_GAP;
            poses.insert(beam.id.clone(), placed(&beam.id).map_or(default, |p| Pose { origin: p.origin.clone(), yaw: p.yaw }));
        }
        for (slot, column) in document.columns.iter().enumerate() {
            let default = Pose { origin: v3(slot as f64 * COLUMN_PITCH, COLUMN_ROW, 0.0), yaw: 0.0 };
            poses.insert(column.id.clone(), placed(&column.id).map_or(default, |p| Pose { origin: p.origin.clone(), yaw: p.yaw }));
        }
        for (slot, slab) in document.slabs.iter().enumerate() {
            let default = Pose { origin: v3(0.0, SLAB_ROW - slot as f64 * SLAB_PITCH, 0.0), yaw: 0.0 };
            poses.insert(slab.id.clone(), placed(&slab.id).map_or(default, |p| Pose { origin: p.origin.clone(), yaw: p.yaw }));
        }
        let mut cases = BTreeMap::new();
        for (id, spot) in forms::origins::<En1994Cases>(&document.cases, 0.0, cursor + CASE_GAP) {
            let pose = placed(&id).map_or(CasePose { origin: spot, yaw: 0.0 }, |p| CasePose { origin: p.origin.clone(), yaw: p.yaw });
            cases.insert(id, pose);
        }
        Self { poses, cases }
    }

    pub fn case(&self, id: &str) -> Option<&CasePose> {
        self.cases.get(id)
    }

    pub fn pose(&self, id: &str) -> Option<&Pose> {
        self.poses.get(id)
    }
}
