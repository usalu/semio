//! 📍️ Layout: where every wall stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! walls run along `+X` left to right with a 3 m gap, thickness towards `+Y`, openings stand at their own positions.

extern crate semio_s_artifact_norm_en1996_model as model;
extern crate semio_s_artifact_norm_en1996_schema as schema;
extern crate semio_s_plugin_norm_structural_scene as scene;

use model::{En1996Snapshot, MasonryWall};
use scene::projection_kit::design_case::forms::{self, CasePose};
use scene::projection_kit::{v3, Placement};
use scene::Vec3;
use schema::checks::masonry::En1996Cases;
use std::collections::BTreeMap;

pub const WALL_GAP: f64 = 3.0;
pub const ROW_CASES: f64 = 15.0;
pub const EPS: f64 = 1e-9;

/// 📍️ One resolved wall pose: the local origin and the yaw about `+Z`; wall coordinates run `x` along the length, `y` through the thickness and `z` up.
#[derive(Clone, Debug, PartialEq)]
pub struct WallPose {
    pub origin: Vec3,
    pub yaw: f64,
}

impl WallPose {
    pub fn at(&self, x: f64, y: f64, z: f64) -> Vec3 {
        let (s, c) = self.yaw.sin_cos();
        v3(self.origin.x + x * c - y * s, self.origin.y + x * s + y * c, self.origin.z + z)
    }

    pub fn dir(&self, x: f64, y: f64, z: f64) -> Vec3 {
        let (s, c) = self.yaw.sin_cos();
        v3(x * c - y * s, x * s + y * c, z)
    }

    pub fn axis_x(&self) -> Vec3 {
        self.dir(1.0, 0.0, 0.0)
    }

    pub fn axis_y(&self) -> Vec3 {
        self.dir(0.0, 1.0, 0.0)
    }
}

/// 🪟️ One opening resolved into wall coordinates: horizontal span and vertical span, both clamped to the wall.
#[derive(Clone, Debug, PartialEq)]
pub struct OpeningSpan {
    pub id: String,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

/// 🪟️ The openings of a wall in document order, placed at their own positions along the wall and clamped to it.
pub fn openings(wall: &MasonryWall) -> Vec<OpeningSpan> {
    wall.openings
        .iter()
        .map(|o| {
            let x0 = o.position_m.clamp(0.0, wall.length_m);
            OpeningSpan { id: o.id.clone(), x0, x1: (x0 + o.width_m.max(0.0)).min(wall.length_m), z0: o.sill_height_m.clamp(0.0, wall.height_m), z1: (o.sill_height_m + o.height_m).clamp(0.0, wall.height_m) }
        })
        .filter(|o| o.x1 - o.x0 > EPS && o.z1 - o.z0 > EPS)
        .collect()
}

/// 🧱️ The solid rectangles `(x0, x1, z0, z1)` of the wall between the heights `z0` and `z1` once every opening is cut out.
pub fn solid_rects(wall: &MasonryWall, z0: f64, z1: f64) -> Vec<(f64, f64, f64, f64)> {
    let spans = openings(wall);
    let mut zs = vec![z0, z1];
    zs.extend(spans.iter().flat_map(|o| [o.z0, o.z1]).filter(|z| *z > z0 + EPS && *z < z1 - EPS));
    zs.sort_by(f64::total_cmp);
    zs.dedup_by(|a, b| (*a - *b).abs() < EPS);
    let mut out = Vec::new();
    for pair in zs.windows(2) {
        let mid = 0.5 * (pair[0] + pair[1]);
        let mut cuts: Vec<(f64, f64)> = spans.iter().filter(|o| o.z0 < mid && mid < o.z1).map(|o| (o.x0, o.x1)).collect();
        cuts.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut x = 0.0;
        for (a, b) in cuts {
            if a > x + EPS {
                out.push((x, a, pair[0], pair[1]));
            }
            x = x.max(b);
        }
        if wall.length_m > x + EPS {
            out.push((x, wall.length_m, pair[0], pair[1]));
        }
    }
    out
}

/// 📍️ All resolved wall poses of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    walls: BTreeMap<String, WallPose>,
    cases: BTreeMap<String, CasePose>,
}

impl Layout {
    pub fn of(document: &En1996Snapshot, placements: &[Placement]) -> Self {
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for wall in &document.walls {
            let pose = match placements.iter().find(|p| p.entity_id == wall.id) {
                Some(p) => WallPose { origin: p.origin.clone(), yaw: p.yaw },
                None => WallPose { origin: v3(cursor, 0.0, 0.0), yaw: 0.0 },
            };
            cursor += wall.length_m.max(0.5) + WALL_GAP;
            layout.walls.insert(wall.id.clone(), pose);
        }
        for (id, spot) in forms::origins::<En1996Cases>(&document.cases, 0.0, ROW_CASES) {
            let pose = placements.iter().find(|p| p.entity_id == id).map_or(CasePose { origin: spot, yaw: 0.0 }, |p| CasePose { origin: p.origin.clone(), yaw: p.yaw });
            layout.cases.insert(id, pose);
        }
        layout
    }

    pub fn case(&self, id: &str) -> Option<&CasePose> {
        self.cases.get(id)
    }

    pub fn pose(&self, wall: &str) -> Option<&WallPose> {
        self.walls.get(wall)
    }
}
