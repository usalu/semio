//! 📍️ Layout: where every spatial entity stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! members run along `+X` left to right with a 1.5 m gap (cantilever posts stand upright); cold-formed sheets follow in a row at `y = -6`, shells at `y = -12`.

use super::kit::{member_frame, v3};
use super::model::{AluminiumMember, En1999Snapshot, SupportCondition};
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;
use super::scene::projection_kit::rotate_yaw as rotate;

pub const MEMBER_GAP: f64 = 1.5;
pub const ROW_SHEETS: f64 = -6.0;
pub const ROW_SHELLS: f64 = -12.0;
pub const SHELL_SLOT: f64 = 3.0;
pub const SHEET_GAP: f64 = 1.0;

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

pub fn is_upright(member: &AluminiumMember) -> bool {
    member.support == SupportCondition::Cantilever
}

impl Layout {
    pub fn of(document: &En1999Snapshot, placements: &[Placement]) -> Self {
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
        let mut sheet_cursor = 0.0;
        for sheet in &document.cold_formed {
            let origin = placed(&sheet.id).map_or_else(|| v3(sheet_cursor, ROW_SHEETS, 0.0), |p| p.origin.clone());
            sheet_cursor += sheet.span.max(0.1) + SHEET_GAP;
            layout.rows.insert(sheet.id.clone(), origin);
        }
        for (slot, shell) in document.shells.iter().enumerate() {
            let origin = placed(&shell.id).map_or_else(|| v3(slot as f64 * SHELL_SLOT, ROW_SHELLS, 0.0), |p| p.origin.clone());
            layout.rows.insert(shell.id.clone(), origin);
        }
        layout
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
