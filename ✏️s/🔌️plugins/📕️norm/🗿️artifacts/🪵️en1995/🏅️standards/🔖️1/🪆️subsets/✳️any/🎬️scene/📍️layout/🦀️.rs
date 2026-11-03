//! 📍️ Layout: where every spatial entity stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! members run along `+X` left to right with a 1.5 m gap (columns upright); connections stand in a row at `y = -4 m`, one fixed slot after the other; the typed entities stand in rows from `y = 12 m` (corner origins); the catalogue cases, all drawn by the shared forms, stand packed by bounding box in shelves from `y = +4 m` (centred poses) around the rectangles of the typed entities, so no two overlap.

use super::kit::{member_frame, v3};
use super::model::{En1995Snapshot, MemberRole, SupportType, TimberConnection, TimberMember};
use super::model::CaseRow;
use super::scene::projection_kit::design_case::forms::Size;
use super::scene::{Frame, Vec3};
use super::schema::checks::timber::En1995Cases;
use std::collections::BTreeMap;

pub use super::scene::projection_kit::design_case::forms::CasePose;
use super::scene::projection_kit::design_case::forms;

pub use super::scene::projection_kit::Placement;
use super::scene::projection_kit::rotate_yaw as rotate;

pub const MEMBER_GAP: f64 = 1.5;
pub const ROW_CONNECTIONS: f64 = -4.0;
pub const CONNECTION_GAP: f64 = 0.6;
pub const ROW_CASES: f64 = 4.0;
pub const CASE_GAP: f64 = 0.8;
pub const ROW_ENTITIES: f64 = 12.0;
pub const ROW_ENTITIES_STEP: f64 = 4.0;
pub const ENTITY_MARGIN: f64 = 0.5;

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
    connections: BTreeMap<String, (Vec3, f64)>,
    cases: BTreeMap<String, CasePose>,
    entities: BTreeMap<String, Vec3>,
    entity_rects: Vec<[f64; 4]>,
}

pub(crate) fn is_upright(member: &TimberMember) -> bool {
    member.role == MemberRole::Column
}

/// 📏️ The drawn axis length: two equal spans for the continuous beam, one span otherwise.
pub fn axis_length(member: &TimberMember) -> f64 {
    let span = member.span_m.max(0.0);
    if member.support == SupportType::ContinuousTwoSpan { 2.0 * span } else { span }
}

/// 📏️ Plan footprint `(length along the grain, width)` of a connection under the documented grid rule (see [`super::facts::grid`]).
pub fn connection_extent(connection: &TimberConnection) -> (f64, f64) {
    let grid = super::facts::grid(connection);
    (grid.length, grid.width)
}

impl Layout {
    pub fn of(document: &En1995Snapshot, placements: &[Placement]) -> Self {
        let placed = |id: &str| placements.iter().find(|p| p.entity_id == id);
        let mut layout = Self::default();
        let mut cursor = 0.0;
        for member in &document.members {
            let axis = if is_upright(member) { Vec3::Z } else { Vec3::X };
            let (origin, direction) = match placed(&member.id) {
                Some(p) => (p.origin.clone(), rotate(&axis, p.yaw)),
                None => (v3(cursor, 0.0, 0.0), axis.clone()),
            };
            let length = axis_length(member);
            cursor += if is_upright(member) { member.b_m.max(0.1) } else { length } + MEMBER_GAP;
            layout.members.insert(member.id.clone(), MemberPose { frame: member_frame(&origin, &direction), length });
        }
        let mut cursor = 0.0;
        for connection in &document.connections {
            let (length, width) = connection_extent(connection);
            let origin = placed(&connection.id).map_or_else(|| v3(cursor, ROW_CONNECTIONS, 0.0), |p| p.origin.clone());
            cursor += length + CONNECTION_GAP;
            layout.connections.insert(connection.id.clone(), (origin, width));
        }
        let mut row = ROW_ENTITIES;
        for table in super::schema::checks::timber::ENTITY_TABLES {
            let mut cursor = 0.0;
            for index in 0..super::schema::checks::timber::entity_ids(document, table).len() {
                let Some((drawn, _)) = super::schema::checks::timber::drawing_row(document, table, index) else { continue };
                let (length, width) = entities::extent(&drawn);
                let origin = placed(&drawn.id).map_or_else(|| v3(cursor, row, 0.0), |p| p.origin.clone());
                layout.entity_rects.push([origin.x - ENTITY_MARGIN, origin.x + length + ENTITY_MARGIN, origin.y - ENTITY_MARGIN, origin.y + width + ENTITY_MARGIN]);
                cursor += length + CASE_GAP;
                layout.entities.insert(drawn.id.clone(), origin);
            }
            row += ROW_ENTITIES_STEP;
        }
        let ext = |case: &CaseRow, size: Size| entities::case_extras(document.annex, case, size);
        for (id, spot) in forms::origins_with::<En1995Cases>(&document.cases, 0.0, ROW_CASES, &layout.entity_rects, &ext) {
            let pose = placed(&id).map_or(CasePose { origin: spot, yaw: 0.0 }, |p| CasePose { origin: p.origin.clone(), yaw: p.yaw });
            layout.cases.insert(id, pose);
        }
        layout
    }

    pub fn entity(&self, id: &str) -> Option<&Vec3> {
        self.entities.get(id)
    }

    /// 📍️ The pose of a catalogue case.
    pub fn case(&self, id: &str) -> Option<&CasePose> {
        self.cases.get(id)
    }

    /// 🔲️ The plan rectangles `[x0, x1, y0, y1]` of the typed entities, the obstacles the default places of the cases avoid.
    pub fn entity_rects(&self) -> &[[f64; 4]] {
        &self.entity_rects
    }

    pub fn member(&self, id: &str) -> Option<&MemberPose> {
        self.members.get(id)
    }

    pub fn connection(&self, id: &str) -> Option<&Vec3> {
        self.connections.get(id).map(|(origin, _)| origin)
    }

    pub fn point_on(&self, member: &str, distance: f64) -> Option<Vec3> {
        self.member(member).map(|m| super::kit::along(&m.frame.origin, &m.frame.x, distance))
    }
}
