//! 📍️ Layout: where every member stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! members run along `+X` left to right with a 1.5 m gap, all on the ground line `y = 0`.

use super::checks::catalogue::En1990Cases;
use super::kit::{member_frame, v3};
use super::model::En1990Snapshot;
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;
pub use super::scene::projection_kit::design_case::forms::CasePose;
use super::scene::projection_kit::design_case::forms;
use super::scene::projection_kit::rotate_yaw as rotate;

pub const MEMBER_GAP: f64 = 1.5;

/// 📍️ One resolved member pose: frame at the start node and axis length.
#[derive(Clone, Debug, PartialEq)]
pub struct MemberPose {
    pub frame: Frame,
    pub length: f64,
}

/// 📍️ All resolved poses of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    members: BTreeMap<String, MemberPose>,
    cases: BTreeMap<String, CasePose>,
}

impl Layout {
    pub fn of(document: &En1990Snapshot, placements: &[Placement]) -> Self {
        let placed = |id: &str| placements.iter().find(|p| p.entity_id == id);
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for member in &document.members {
            let (origin, direction) = match placed(&member.id) {
                Some(p) => (p.origin.clone(), rotate(&Vec3::X, p.yaw)),
                None => (v3(cursor, 0.0, 0.0), Vec3::X),
            };
            cursor += member.span + MEMBER_GAP;
            layout.members.insert(member.id.clone(), MemberPose { frame: member_frame(&origin, &direction), length: member.span });
        }
        for (id, spot) in forms::origins::<En1990Cases>(&document.cases, 0.0, super::cases::ROW_CASES) {
            let pose = placed(&id).map_or(CasePose { origin: spot, yaw: 0.0 }, |p| CasePose { origin: p.origin.clone(), yaw: p.yaw });
            layout.cases.insert(id, pose);
        }
        layout
    }

    pub fn case(&self, id: &str) -> Option<&CasePose> {
        self.cases.get(id)
    }

    pub fn case_poses(&self) -> impl Iterator<Item = (&String, &CasePose)> {
        self.cases.iter()
    }

    pub fn member(&self, id: &str) -> Option<&MemberPose> {
        self.members.get(id)
    }

    pub fn point_on(&self, member: &str, distance: f64) -> Option<Vec3> {
        self.member(member).map(|m| super::kit::along(&m.frame.origin, &m.frame.x, distance))
    }

    pub fn members(&self) -> impl Iterator<Item = (&String, &MemberPose)> {
        self.members.iter()
    }
}
