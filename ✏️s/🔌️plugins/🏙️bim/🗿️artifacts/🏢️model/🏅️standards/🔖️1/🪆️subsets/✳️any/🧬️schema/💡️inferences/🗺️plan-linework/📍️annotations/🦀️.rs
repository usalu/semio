//! 📍️ Annotations in the plan: spaces (outline and tag anchor with number, name and area) and the grid lines of the building with their bubbles.

use super::{Context, PlanKind, PlanStyle, PlanVertex, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{bulged as corners, point};
use crate::SpaceBoundary;
use semio_framework_geometry::Point;

/// ⭕️ Radius of a grid bubble in metres: the plan convention.
pub const GRID_BUBBLE_RADIUS: f64 = 0.3;

/// 📏️ Draws the grid lines of the building of the storey: the line, a bubble beyond each end and the label in each bubble.
pub fn draw_grids(sheet: &mut Sheet, cx: &Context<'_>) {
    let Some(building) = cx.snapshot.storeys.get(cx.storey).map(|storey| &storey.building) else { return };
    for (id, grid) in cx.snapshot.grids.iter().filter(|(_, grid)| &grid.building == building) {
        let (start, end) = (point(&grid.start), point(&grid.end));
        let along = end - start;
        let length = along.hypot();
        if length <= 1e-9 {
            continue;
        }
        sheet.path(id, PlanKind::GridLine, PlanStyle::Annotation, &[start, end]);
        let unit = Point::new(along.x / length, along.y / length);
        for (tip, sign) in [(start, -1.0), (end, 1.0)] {
            let centre = Point::new(tip.x + sign * unit.x * GRID_BUBBLE_RADIUS, tip.y + sign * unit.y * GRID_BUBBLE_RADIUS);
            let round = vec![PlanVertex { x: centre.x + GRID_BUBBLE_RADIUS, y: centre.y, bulge: 1.0 }, PlanVertex { x: centre.x - GRID_BUBBLE_RADIUS, y: centre.y, bulge: 1.0 }];
            sheet.polyline(id, PlanKind::GridBubble, PlanStyle::Annotation, true, round);
            sheet.text(id, PlanKind::GridLabel, PlanStyle::Annotation, centre, 0.0, &grid.label, "", None);
        }
    }
}

/// 🏠️ Draws the spaces of the storey from their inferred rooms: the outline and its islands, and a tag with number, name and area at an interior point (at the seed while the room is unknown).
pub fn draw_spaces(sheet: &mut Sheet, cx: &Context<'_>) {
    let rooms = cx.inputs.rooms;
    for (id, space) in cx.snapshot.spaces.iter().filter(|(_, space)| space.storey == cx.storey) {
        let room = rooms.and_then(|rooms| rooms.get(id)).filter(|room| room.outline.len() >= 3);
        let (anchor, area) = match (room, &space.boundary) {
            (Some(room), _) => {
                sheet.loop_polyline(id, PlanKind::SpaceOutline, PlanStyle::Annotation, &corners(&room.outline));
                room.holes.iter().for_each(|hole| sheet.loop_polyline(id, PlanKind::SpaceOutline, PlanStyle::Annotation, &corners(hole)));
                (point(&room.point), Some(room.area))
            }
            (None, SpaceBoundary::Bounded { seed }) => (point(seed), None),
            (None, SpaceBoundary::Explicit { .. }) => continue,
        };
        sheet.text(id, PlanKind::SpaceTag, PlanStyle::Annotation, anchor, 0.0, &space.number, &space.name, area);
    }
}
