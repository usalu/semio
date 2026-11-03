//! 📍️ Layout: where every spatial entity stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! members run along `+X` left to right with a 1.5 m gap (columns and walls upright, slabs flat at their support level); interaction surfaces sit in a row at `y = +5`, anchors in a row at `y = -6`.

use super::checks::En1992Cases;
use super::kit::{unit, v3};
use super::model::{En1992Snapshot, MemberKind};
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;
pub use super::scene::projection_kit::design_case::forms::CasePose;
use super::scene::projection_kit::design_case::forms;
use super::scene::projection_kit::rotate_yaw as rotate;

pub const MEMBER_GAP: f64 = 1.5;
pub const ROW_INTERACTION: f64 = 5.0;
pub const ROW_ANCHORS: f64 = -6.0;
pub const SLOT: f64 = 6.0;
pub const ROW_CASES: f64 = 25.0;
pub const COLUMN_UNDER_SLAB: f64 = 2.5;

/// 🧭️ How a member stands in space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Beam,
    Upright,
    Slab,
}

/// 📍️ One resolved member pose: frame at the start of the axis through the section centroid, axis length and posture class.
#[derive(Clone, Debug, PartialEq)]
pub struct MemberPose {
    pub frame: Frame,
    pub length: f64,
    pub class: Class,
    pub interaction: Vec3,
}

/// 📍️ All resolved poses of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    members: BTreeMap<String, MemberPose>,
    anchors: BTreeMap<String, Vec3>,
    cases: BTreeMap<String, CasePose>,
}

pub fn class_of(kind: MemberKind) -> Class {
    match kind {
        MemberKind::Column | MemberKind::Wall => Class::Upright,
        MemberKind::Slab | MemberKind::FlatSlab | MemberKind::RibbedSlab => Class::Slab,
        _ => Class::Beam,
    }
}

pub fn frame_of(origin: &Vec3, axis: &Vec3) -> Frame {
    let x = unit(axis);
    let reference = if x.cross(&Vec3::Z).length() < 1e-9 { Vec3::X } else { Vec3::Z };
    let y = unit(&reference.cross(&x));
    let z = x.cross(&y);
    Frame { origin: origin.clone(), x, y, z }
}

impl Layout {
    pub fn of(document: &En1992Snapshot, placements: &[Placement]) -> Self {
        let placed = |id: &str| placements.iter().find(|p| p.entity_id == id);
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for member in &document.members {
            let class = class_of(member.kind);
            let footprint = match class {
                Class::Upright => member.width.max(member.height),
                _ => member.span,
            };
            let level = match (class, member.kind) {
                (Class::Beam, _) => member.height / 2.0,
                (Class::Upright, _) => 0.0,
                (Class::Slab, MemberKind::FlatSlab) => COLUMN_UNDER_SLAB + member.height / 2.0,
                (Class::Slab, _) => member.height / 2.0,
            };
            let axis = if class == Class::Upright { Vec3::Z } else { Vec3::X };
            let (origin, yaw) = placed(&member.id).map(|p| (p.origin.clone(), p.yaw)).unwrap_or_else(|| (v3(cursor, 0.0, level), 0.0));
            let frame = frame_of(&origin, &rotate(&axis, yaw));
            let interaction = v3(origin.x + footprint / 2.0, ROW_INTERACTION, 0.0);
            cursor += footprint + MEMBER_GAP;
            layout.members.insert(member.id.clone(), MemberPose { frame, length: member.span, class, interaction });
        }
        for (slot, anchor) in document.anchors.iter().enumerate() {
            let origin = placed(&anchor.id).map(|p| p.origin.clone()).unwrap_or_else(|| v3(slot as f64 * SLOT, ROW_ANCHORS, 0.0));
            layout.anchors.insert(anchor.id.clone(), origin);
        }
        for (id, spot) in forms::origins::<En1992Cases>(&document.cases, 0.0, ROW_CASES) {
            let pose = placed(&id).map_or(CasePose { origin: spot, yaw: 0.0 }, |p| CasePose { origin: p.origin.clone(), yaw: p.yaw });
            layout.cases.insert(id, pose);
        }
        layout
    }

    pub fn case(&self, id: &str) -> Option<&CasePose> {
        self.cases.get(id)
    }

    pub fn member(&self, id: &str) -> Option<&MemberPose> {
        self.members.get(id)
    }

    pub fn anchor(&self, id: &str) -> Option<&Vec3> {
        self.anchors.get(id)
    }
}
