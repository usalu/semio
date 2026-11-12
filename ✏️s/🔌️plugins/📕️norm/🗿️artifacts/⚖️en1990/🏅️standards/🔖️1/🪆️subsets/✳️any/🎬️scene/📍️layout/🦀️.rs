//! 📍️ Layout: where every member stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! members run along `+X` left to right with a 1.5 m gap, all on the ground line `y = 0`.

extern crate semio_s_artifact_norm_en1990_checks as checks;
extern crate semio_s_artifact_norm_en1990_model as model;
extern crate semio_s_plugin_norm_structural_scene as scene;

use checks::catalogue::En1990Cases;
use model::En1990Snapshot;
use scene::projection_kit::design_case::forms::{self, CasePose};
use scene::projection_kit::rotate_yaw as rotate;
use scene::projection_kit::{member_frame, v3, Placement};
use scene::{Frame, Vec3};
use std::collections::BTreeMap;

/// 📍️ The first row of cases lies behind the analysis lanes.
pub const ROW_CASES: f64 = 20.0;

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
        for (id, spot) in forms::origins::<En1990Cases>(&document.cases, 0.0, ROW_CASES) {
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
        self.member(member).map(|m| scene::projection_kit::along(&m.frame.origin, &m.frame.x, distance))
    }

    pub fn members(&self) -> impl Iterator<Item = (&String, &MemberPose)> {
        self.members.iter()
    }
}
