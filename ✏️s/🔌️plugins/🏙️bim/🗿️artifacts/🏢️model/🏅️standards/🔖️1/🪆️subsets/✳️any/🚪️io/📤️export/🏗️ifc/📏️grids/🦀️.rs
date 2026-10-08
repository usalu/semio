//! 📏️ Grid lines as one `IfcGrid` per building. IFC 2x3 requires both axis directions, so lines closer to the y axis are U axes, the others V axes; when one direction is empty the first axis of the other is
//! listed in both. Each `IfcGridAxis` carries the line's label and a two-point polyline.

use super::writer::{flag, opt_text, refs, rf, unset};
use super::Export;

/// 📏️ Writes the grid of every building that has grid lines.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (building_id, building) in &x.buildings.clone() {
        let (mut vertical, mut horizontal) = (Vec::new(), Vec::new());
        for (id, line) in model.grids.iter().filter(|(_, line)| line.building == *building_id) {
            let (from, to) = (x.ifc.point2([line.start.x, line.start.y]), x.ifc.point2([line.end.x, line.end.y]));
            let curve = x.ifc.add("IFCPOLYLINE", vec![refs(&[from, to])]);
            let axis = x.ifc.add("IFCGRIDAXIS", vec![opt_text(&line.label), rf(curve), flag(true)]);
            let target = if (line.end.x - line.start.x).abs() >= (line.end.y - line.start.y).abs() { &mut horizontal } else { &mut vertical };
            target.push((id.clone(), axis));
        }
        if vertical.is_empty() && horizontal.is_empty() {
            continue;
        }
        let ids = vertical.iter().chain(&horizontal).map(|(id, _)| id.as_str()).collect::<Vec<_>>().join(",");
        if vertical.is_empty() {
            vertical.push(horizontal[0].clone());
        }
        if horizontal.is_empty() {
            horizontal.push(vertical[0].clone());
        }
        let placement = x.ifc.place(Some(building.placement), x.ifc.origin);
        let axes = |rows: &[(String, u64)]| refs(&rows.iter().map(|(_, axis)| *axis).collect::<Vec<u64>>());
        let grid = x.ifc.rooted("IFCGRID", building_id, "", "", vec![opt_text(&ids), rf(placement), unset(), axes(&vertical), axes(&horizontal), unset()]);
        x.links.elements.insert(format!("{building_id}:grid"), grid);
        x.links.in_building.entry(building_id.clone()).or_default().push(grid);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
