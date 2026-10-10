//! 🪑️ Components in the plan: the oriented outline of the footprint of the instance (the rectangle of the bounds of its visible solids in the family frame), a short tick at the middle of the front edge (the local `+y`
//! side, the side a hosted component faces away from its wall) and, for a terminal, a cross at the connector. The outline is cut, projected or hidden like every element by the vertical span of the component against the cut plane;
//! the tick and the cross are annotation strokes. The colour of the cross is the colour of the system, `components[id].connector.colour` of the inference.

use super::{classify, outline_style, Context, PlanKind, PlanStyle, PlanVertex, Sheet};
use semio_framework_geometry::Point;

/// 📏️ Length in metres of the tick in front of the outline.
pub const TICK: f64 = 0.1;
/// 📏️ Half the length in metres of each stroke of the connector cross.
pub const CROSS: f64 = 0.1;

/// 🪑️ Draws the components of the storey.
pub fn draw(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, value) in cx.inputs.components.iter().filter(|(_, value)| value.storey == cx.storey && value.solid()) {
        let style = outline_style(classify(value.bounds.min.z, value.bounds.max.z, cx.cut));
        sheet.polyline(id, PlanKind::ComponentOutline, style, true, value.footprint.iter().map(|corner| PlanVertex { x: corner.x, y: corner.y, bulge: 0.0 }).collect());
        let count = value.footprint.len() as f64;
        let centre = value.footprint.iter().fold((0.0, 0.0), |sum, corner| (sum.0 + corner.x / count, sum.1 + corner.y / count));
        let front = value.placement.rotate([0.0, 1.0, 0.0]);
        let half = value.footprint.iter().map(|corner| (corner.x - centre.0) * front[0] + (corner.y - centre.1) * front[1]).fold(0.0, f64::max);
        let from = Point::new(centre.0 + front[0] * half, centre.1 + front[1] * half);
        sheet.path(id, PlanKind::ComponentFront, PlanStyle::Annotation, &[from, Point::new(from.x + front[0] * TICK, from.y + front[1] * TICK)]);
        if let Some(connector) = &value.connector {
            let at = Point::new(connector.position.x, connector.position.y);
            sheet.path(id, PlanKind::ComponentConnector, PlanStyle::Annotation, &[Point::new(at.x - CROSS, at.y), Point::new(at.x + CROSS, at.y)]);
            sheet.path(id, PlanKind::ComponentConnector, PlanStyle::Annotation, &[Point::new(at.x, at.y - CROSS), Point::new(at.x, at.y + CROSS)]);
        }
    }
}
