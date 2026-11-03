//! 📍️ Layout: where every spatial entity stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! members run along `+X` left to right with a 1.5 m gap (columns and braces upright); satellite rows sit at fixed `y` offsets.

use super::kit::{member_frame, v3};
use super::model::{En1993Snapshot, SteelMember, SteelMemberType};
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;
use super::scene::projection_kit::rotate_yaw as rotate;

pub const MEMBER_GAP: f64 = 1.5;
pub const ROW_PANELS: f64 = -6.0;
pub const ROW_PILES: f64 = -12.0;
pub const ROW_COLD_FORMED: f64 = -18.0;
pub const ROW_PARAMETRIC: f64 = -24.0;
pub const ROW_FRAME: f64 = -30.0;
pub const ROW_DECKS: f64 = -36.0;
pub const ROW_MATERIALS: f64 = -42.0;
pub const ROW_CASES: f64 = -48.0;
pub const ROW_SECTIONS: f64 = -54.0;
pub const ROW_TENSION: f64 = 5.0;
pub const ROW_SILOS: f64 = 12.0;
pub const SLOT: f64 = 6.0;

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
    rows: BTreeMap<String, Vec3>,
}

pub fn is_upright(member: &SteelMember) -> bool {
    member.member_type == SteelMemberType::Column
}

impl Layout {
    pub fn of(document: &En1993Snapshot, placements: &[Placement]) -> Self {
        let placed = |id: &str| placements.iter().find(|p| p.entity_id == id);
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for member in &document.members {
            let axis = if is_upright(member) { Vec3::Z } else { Vec3::X };
            let (origin, direction) = match placed(&member.id) {
                Some(p) => (p.origin.clone(), rotate(&axis, p.yaw)),
                None => (v3(cursor, 0.0, 0.0), axis.clone()),
            };
            let advance = if is_upright(member) { 1.0 } else { member.length };
            cursor += advance + MEMBER_GAP;
            layout.members.insert(member.id.clone(), MemberPose { frame: member_frame(&origin, &direction), length: member.length });
        }
        let rows: [(f64, Vec<&str>); 10] = [
            (ROW_PANELS, document.plated_panels.iter().map(|e| e.id.as_str()).collect()),
            (ROW_PILES, document.piles.iter().map(|e| e.id.as_str()).collect()),
            (ROW_COLD_FORMED, document.cold_formed_members.iter().map(|e| e.id.as_str()).collect()),
            (ROW_TENSION, document.tension_components.iter().map(|e| e.id.as_str()).collect()),
            (ROW_SILOS, document.silo_shells.iter().map(|e| e.id.as_str()).collect()),
            (ROW_PARAMETRIC, document.parametric_sections.iter().map(|e| e.id.as_str()).collect()),
            (ROW_DECKS, document.bridge_fatigue.iter().map(|e| e.id.as_str()).collect()),
            (ROW_MATERIALS, document.materials.iter().map(|e| e.id.as_str()).collect()),
            (ROW_CASES, document.load_cases.iter().map(|e| e.id.as_str()).collect()),
            (ROW_SECTIONS, document.sections.iter().map(|e| e.id.as_str()).collect()),
        ];
        for (y, ids) in rows {
            for (slot, id) in ids.iter().enumerate() {
                let origin = placed(id).map_or_else(|| v3(slot as f64 * SLOT, y, 0.0), |p| p.origin.clone());
                layout.rows.insert((*id).to_string(), origin);
            }
        }
        layout
    }

    /// 🕸️ The world point of a frame node with coordinates x and z: the frame stands in the plane `y = ROW_FRAME`.
    pub fn frame_point(&self, x: f64, z: f64) -> Vec3 {
        v3(x, ROW_FRAME, z)
    }

    pub fn member(&self, id: &str) -> Option<&MemberPose> {
        self.members.get(id)
    }

    pub fn origin(&self, id: &str) -> Vec3 {
        self.rows.get(id).cloned().unwrap_or(Vec3::ZERO)
    }

    pub fn point_on(&self, member: &str, distance: f64) -> Option<Vec3> {
        self.member(member).map(|m| super::kit::along(&m.frame.origin, &m.frame.x, distance))
    }
}
