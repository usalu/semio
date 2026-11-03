//! 📍️ Layout: where every geotechnical entity stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! rows along `+X` by kind (footings `y = 0`, piles `y = -16`, uplift slabs `y = -32`, walls `y = 16`, slopes `y = 40`, embedded walls `y = 64`, pile groups `y = -48`, laboratory specimens `y = -64`); the ground block spans every footprint plus a margin and the deepest layer.

use super::kit::v3;
use super::mechanics as mech;
use super::model::{EmbeddedWall, En1997Snapshot, Pile, PileGroup, RetainingWall, Slope, SpreadFoundation};
use super::checks::catalogue::En1997Cases;
use super::scene::projection_kit::design_case::forms;
use super::scene::{Aabb, Frame, Vec3};
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;

pub const ROW_FOOTINGS: f64 = 0.0;
pub const ROW_PILES: f64 = -16.0;
pub const ROW_UPLIFT: f64 = -32.0;
pub const ROW_WALLS: f64 = 16.0;
pub const ROW_SLOPES: f64 = 40.0;
pub const ROW_EMBEDDED: f64 = 64.0;
pub const ROW_GROUPS: f64 = -48.0;
pub const ROW_LABS: f64 = -64.0;
pub const ROW_CASES: f64 = 88.0;
pub const SLOT_EMBEDDED: f64 = 12.0;
pub const SLOT_GROUP: f64 = 14.0;
pub const SLOT_LAB: f64 = 4.0;
pub const SLOT_FOOTING: f64 = 10.0;
pub const SLOT_PILE: f64 = 14.0;
pub const SLOT_UPLIFT: f64 = 8.0;
pub const SLOT_WALL: f64 = 12.0;
pub const SLOPE_CREST: f64 = 4.0;
pub const SLOPE_TOE: f64 = 4.0;
pub const WALL_LENGTH: f64 = 6.0;
pub const UPLIFT_SIDE: f64 = 4.0;
pub const MARGIN: f64 = 6.0;

/// 🧭️ A resolved pose: origin on the ground surface and yaw about `+Z`.
#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub origin: Vec3,
    pub yaw: f64,
}

impl Pose {
    pub fn dir(&self, v: &Vec3) -> Vec3 {
        let (s, c) = self.yaw.sin_cos();
        v3(v.x * c - v.y * s, v.x * s + v.y * c, v.z)
    }

    pub fn at(&self, x: f64, y: f64, z: f64) -> Vec3 {
        let d = self.dir(&v3(x, y, z));
        v3(self.origin.x + d.x, self.origin.y + d.y, self.origin.z + d.z)
    }

    pub fn frame(&self, local: [f64; 3], x: Vec3, y: Vec3, z: Vec3) -> Frame {
        Frame { origin: self.at(local[0], local[1], local[2]), x: self.dir(&x), y: self.dir(&y), z: self.dir(&z) }
    }

    pub fn upright(&self, local: [f64; 3]) -> Frame {
        self.frame(local, Vec3::X, Vec3::Y, Vec3::Z)
    }

    pub fn section(&self, local: [f64; 3]) -> Frame {
        self.frame(local, Vec3::X, Vec3::Z, v3(0.0, -1.0, 0.0))
    }
}

/// 📍️ All resolved poses of one projection and the ground block they share.
#[derive(Clone, Debug)]
pub struct Layout {
    poses: BTreeMap<String, Pose>,
    pub min: [f64; 2],
    pub max: [f64; 2],
    pub depth: f64,
}

fn footing_rect(f: &SpreadFoundation) -> [f64; 4] {
    let (hx, hy) = (f.width / 2.0 + 1.5 * f.width, f.length / 2.0 + 1.0);
    [-hx, hx, -hy, hy]
}

fn pile_rect(p: &Pile) -> [f64; 4] {
    let r = mech::pile_grid(p).iter().fold(0.0_f64, |m, g| m.max(g[0].abs()).max(g[1].abs())) + p.diameter + 2.0;
    [-r, r, -r, r]
}

pub fn wall_stem_x(w: &RetainingWall) -> f64 {
    0.3 * w.base_width
}

fn wall_rect(w: &RetainingWall) -> [f64; 4] {
    [-0.3 * w.base_width - 1.0, 0.7 * w.base_width + wall_reach(w), -WALL_LENGTH / 2.0, WALL_LENGTH / 2.0]
}

pub fn wall_reach(w: &RetainingWall) -> f64 {
    (2.0 * w.height).max(3.0)
}

fn embedded_rect(w: &EmbeddedWall) -> [f64; 4] {
    [-1.0, (2.0 * w.retained_height).max(3.0) + w.thickness, -WALL_LENGTH / 2.0, WALL_LENGTH / 2.0]
}

fn group_rect(g: &PileGroup) -> [f64; 4] {
    let (ex, ey) = ((f64::from(g.columns) - 1.0) * g.spacing / 2.0 + 2.0, (f64::from(g.rows) - 1.0) * g.spacing / 2.0 + 2.0);
    [-ex, ex, -ey, ey]
}

fn slope_rect(s: &Slope) -> [f64; 4] {
    [-SLOPE_CREST, mech::slope_run(s.height, s.angle_deg) + SLOPE_TOE, -mech::slope_span(s) / 2.0, mech::slope_span(s) / 2.0]
}

impl Layout {
    /// 💧 Plan position of a dewatering well: one metre inside the east edge on the front edge of the site.
    pub fn well_at(&self) -> [f64; 2] {
        [self.max[0] - 1.0, self.min[1]]
    }

    pub fn of(document: &En1997Snapshot, placements: &[Placement]) -> Self {
        let mut layout = Self { poses: BTreeMap::new(), min: [0.0; 2], max: [0.0; 2], depth: 0.0 };
        let mut corners: Vec<[f64; 2]> = Vec::new();
        let mut put = |id: &str, default: Vec3, rect: [f64; 4], layout: &mut Self| {
            let pose = placements.iter().find(|p| p.entity_id == id).map_or(Pose { origin: default, yaw: 0.0 }, |p| Pose { origin: p.origin.clone(), yaw: p.yaw });
            for (x, y) in [(rect[0], rect[2]), (rect[0], rect[3]), (rect[1], rect[2]), (rect[1], rect[3])] {
                let p = pose.at(x, y, 0.0);
                corners.push([p.x, p.y]);
            }
            layout.poses.insert(id.to_string(), pose);
        };
        for (i, e) in document.footings.iter().enumerate() {
            put(&e.id, v3(i as f64 * SLOT_FOOTING, ROW_FOOTINGS, 0.0), footing_rect(e), &mut layout);
        }
        for (i, e) in document.piles.iter().enumerate() {
            put(&e.id, v3(i as f64 * SLOT_PILE, ROW_PILES, 0.0), pile_rect(e), &mut layout);
        }
        for (i, e) in document.uplift_cases.iter().enumerate() {
            let h = UPLIFT_SIDE / 2.0;
            put(&e.id, v3(i as f64 * SLOT_UPLIFT, ROW_UPLIFT, 0.0), [-h, h, -h, h], &mut layout);
        }
        for (i, e) in document.retaining_walls.iter().enumerate() {
            put(&e.id, v3(i as f64 * SLOT_WALL, ROW_WALLS, 0.0), wall_rect(e), &mut layout);
        }
        for (i, e) in document.embedded_walls.iter().enumerate() {
            put(&e.id, v3(i as f64 * SLOT_EMBEDDED, ROW_EMBEDDED, 0.0), embedded_rect(e), &mut layout);
        }
        for (i, e) in document.pile_groups.iter().enumerate() {
            put(&e.id, v3(i as f64 * SLOT_GROUP, ROW_GROUPS, 0.0), group_rect(e), &mut layout);
        }
        for (i, e) in document.lab_tests.iter().enumerate() {
            put(&e.id, v3(i as f64 * SLOT_LAB, ROW_LABS, 0.0), [-1.0, 1.0, -1.0, 1.0], &mut layout);
        }
        for e in &document.investigations {
            put(&e.id, v3(e.position_x, e.position_y, 0.0), [-1.0, 1.0, -1.0, 1.0], &mut layout);
        }
        for (id, origin) in forms::origins::<En1997Cases>(&document.cases, 0.0, ROW_CASES) {
            if let Some(rect) = document.cases.iter().find(|c| c.id == id).and_then(super::cases::rect) {
                put(&id, origin, rect, &mut layout);
            }
        }
        let mut cursor = 0.0;
        for e in &document.slopes {
            let rect = slope_rect(e);
            put(&e.id, v3(cursor + SLOPE_CREST, ROW_SLOPES, 0.0), rect, &mut layout);
            cursor += rect[1] - rect[0] + 2.0;
        }
        let (mut lo, mut hi) = ([f64::MAX; 2], [f64::MIN; 2]);
        for c in &corners {
            for k in 0..2 {
                lo[k] = lo[k].min(c[k]);
                hi[k] = hi[k].max(c[k]);
            }
        }
        if corners.is_empty() {
            (lo, hi) = ([-5.0; 2], [5.0; 2]);
        }
        layout.min = [lo[0] - MARGIN, lo[1] - MARGIN];
        layout.max = [hi[0] + MARGIN, hi[1] + MARGIN];
        layout.depth = document.layers.iter().map(|l| l.depth_bottom).fold(document.investigation_depth, f64::max).max(1.0);
        layout
    }

    pub fn pose(&self, id: &str) -> Pose {
        self.poses.get(id).cloned().unwrap_or(Pose { origin: Vec3::ZERO, yaw: 0.0 })
    }

    pub fn centre(&self) -> [f64; 2] {
        [(self.min[0] + self.max[0]) / 2.0, (self.min[1] + self.max[1]) / 2.0]
    }

    pub fn aabb(&self) -> Aabb {
        Aabb::new(v3(self.min[0], self.min[1], -self.depth), v3(self.max[0], self.max[1], 0.0))
    }
}
