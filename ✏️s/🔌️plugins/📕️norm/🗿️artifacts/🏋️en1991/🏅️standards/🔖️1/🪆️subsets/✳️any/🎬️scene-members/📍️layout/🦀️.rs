//! 📍️ Layout: where every structure of the EN 1991 subject stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! the building footprint spans `x ∈ [0, depth]`, `y ∈ [0, width]` with the wind blowing along `+X`; the bridge deck lies `GAP` south of it, the crane runway a further `GAP` south,
//! the silo `GAP` east, the self-weight build-up west and every further roof one `GAP` north of the previous one.

use super::kit::v3;
use super::model::En1991Snapshot;
use super::checks::catalogue::En1991Cases;
use super::scene::projection_kit::design_case::forms;
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;
pub use super::scene::projection_kit::design_case::forms::CasePose;

pub const GAP: f64 = 15.0;
pub const SELF_WEIGHT_X: f64 = -12.0;
pub const DECK_Z: f64 = 5.0;
pub const DECK_THICKNESS: f64 = 0.6;
pub const FOOTWAY: f64 = 1.5;
pub const RUNWAY_HEIGHT: f64 = 7.0;
pub const CRANE_BAY: f64 = 4.0;
pub const MIN_SILO_RADIUS: f64 = 0.5;

/// 🧭️ A resolved pose: origin on the ground and yaw about `+Z`.
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

    pub fn upright(&self, local: [f64; 3]) -> Frame {
        Frame { origin: self.at(local[0], local[1], local[2]), x: self.dir(&Vec3::X), y: self.dir(&Vec3::Y), z: Vec3::Z }
    }
}

/// 🌉 Width of the bridge deck: the carriageway plus a footway on each side.
pub fn deck_width(document: &En1991Snapshot) -> f64 {
    document.bridge_lane_width.max(0.0) + 2.0 * FOOTWAY
}

/// 🫙 Radius of the circular silo cross-section from its diameter `d_c` (hydraulic radius `A/U = d_c/4`).
pub fn silo_radius(document: &En1991Snapshot) -> f64 {
    (document.silo_diameter / 2.0).max(MIN_SILO_RADIUS)
}

/// 📍️ All resolved poses of one projection.
#[derive(Clone, Debug)]
pub struct Layout {
    pub building: Pose,
    pub bridge: Pose,
    pub crane: Pose,
    pub silo: Pose,
    pub self_weight: Pose,
    pub roofs: Vec<Pose>,
    pub cases: BTreeMap<String, CasePose>,
}

impl Layout {
    pub fn of(document: &En1991Snapshot, placements: &[Placement]) -> Self {
        let placed = |id: &str, default: Vec3| placements.iter().find(|p| p.entity_id == id).map_or(Pose { origin: default, yaw: 0.0 }, |p| Pose { origin: p.origin.clone(), yaw: p.yaw });
        let building = placed("building", Vec3::ZERO);
        let deck = deck_width(document);
        let roofs = document
            .roofs
            .iter()
            .enumerate()
            .map(|(i, roof)| if i == 0 { building.clone() } else { placed(&roof.id, v3(0.0, (document.width + GAP) * i as f64, 0.0)) })
            .collect();
        let north = (document.width + GAP) * document.roofs.len().max(1) as f64 + GAP;
        let cases = forms::origins::<En1991Cases>(&document.cases, 0.0, north)
            .into_iter()
            .map(|(id, spot)| {
                let pose = placements.iter().find(|p| p.entity_id == id).map_or(CasePose { origin: spot, yaw: 0.0 }, |p| CasePose { origin: p.origin.clone(), yaw: p.yaw });
                (id, pose)
            })
            .collect();
        Self {
            cases,
            bridge: placed("bridge", v3(0.0, -GAP - deck, 0.0)),
            crane: placed("crane", v3(0.0, -2.0 * GAP - deck - CRANE_BAY, 0.0)),
            silo: placed("silo", v3(document.depth + GAP + silo_radius(document), document.width / 2.0, 0.0)),
            self_weight: placed("selfWeight", v3(SELF_WEIGHT_X, 0.0, 0.0)),
            building,
            roofs,
        }
    }

    pub fn case(&self, id: &str) -> Option<&CasePose> {
        self.cases.get(id)
    }

    pub fn roof(&self, index: usize) -> &Pose {
        self.roofs.get(index).unwrap_or(&self.building)
    }
}

pub const STRIP_Z: f64 = 0.15;
pub const STACK_STEP: f64 = 0.4;

/// 🧱️ Height of the construction load stack above the stage strip: the loads of one stage are layered in list order.
pub fn stack_level(d: &En1991Snapshot, index: usize) -> f64 {
    let stage = &d.construction_loads[index].stage;
    STRIP_Z + STACK_STEP * d.construction_loads[..index].iter().filter(|l| &l.stage == stage).count() as f64
}

/// 🛣️ One lane of the deck: its LM1 number and its band across the deck, derived from Table 4.1 or authored as a `bridgeLanes` row.
pub struct Band {
    pub number: u8,
    pub id: String,
    pub derived: bool,
    pub y0: f64,
    pub y1: f64,
}

impl Band {
    /// 🆔️ Id of a part of the lane: `bridge:<stem>-<number><tail>` for derived lanes and `<row id>:<stem><tail>` for authored rows.
    pub fn sub(&self, stem: &str, tail: &str) -> String {
        if self.derived { format!("bridge:{stem}-{}{tail}", self.number) } else { format!("{}:{stem}{tail}", self.id) }
    }

    pub fn label_id(&self) -> String {
        if self.derived { self.sub("lane", ":label") } else { format!("{}:label", self.id) }
    }

    pub fn name(&self, de: bool) -> String {
        if self.derived { format!("{} {}", if de { "Fahrstreifen" } else { "Lane" }, self.number) } else { self.id.clone() }
    }
}

/// 🛣️ The lane bands of the deck in the deck frame, the width left over beside them and where it starts: the authored rows of `bridgeLanes`, else the notional lanes of Table 4.1.
pub fn bands_of(d: &En1991Snapshot) -> (Vec<Band>, f64, f64) {
    let carriageway = d.bridge_lane_width.max(0.0);
    if d.bridge_lanes.is_empty() {
        let (count, lane_width, remaining) = super::checks::part_2::notional_lanes(carriageway);
        let bands = (1..=count).map(|k| { let y0 = FOOTWAY + f64::from(k - 1) * lane_width; Band { number: k, id: format!("bridge:lane-{k}"), derived: true, y0, y1: y0 + lane_width } }).collect();
        return (bands, remaining, FOOTWAY + f64::from(count) * lane_width);
    }
    let bands: Vec<Band> = d.bridge_lanes.iter().map(|l| { let centre = FOOTWAY + carriageway / 2.0 + l.offset; Band { number: l.lm1_lane, id: l.id.clone(), derived: false, y0: centre - l.width / 2.0, y1: centre + l.width / 2.0 } }).collect();
    let used = d.bridge_lanes.iter().map(|l| l.width).sum::<f64>();
    let edge = bands.iter().map(|b| b.y1).fold(FOOTWAY, f64::max);
    (bands, (carriageway - used).max(0.0), edge)
}
