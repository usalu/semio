//! 📏️ Grid lines as one `IfcGrid` per building. Both axis directions are required, so lines closer to the y axis are U axes, the others V axes; when one direction is empty the first axis of the other is listed in both in IFC 2x3, and in IFC4 (an axis is part
//! of exactly one list) repeated as a second `IfcGridAxis` that the importer leaves out because the id list names each line once. Each `IfcGridAxis` carries the line label and a two-point polyline.

use super::writer::{flag, opt_text, refs, rf, unset, Schema};
use super::Export;

/// 📏️ Writes the grid of every building that has grid lines.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (building_id, building) in &x.buildings.clone() {
        let (mut vertical, mut horizontal) = (Vec::new(), Vec::new());
        let mut drawn: std::collections::BTreeMap<u64, Vec<super::writer::V>> = std::collections::BTreeMap::new();
        for (id, line) in model.grids.iter().filter(|(_, line)| line.building == *building_id) {
            let (from, to) = (x.ifc.point2([line.start.x, line.start.y]), x.ifc.point2([line.end.x, line.end.y]));
            let curve = x.ifc.add("IFCPOLYLINE", vec![refs(&[from, to])]);
            let args = vec![opt_text(&line.label), rf(curve), flag(true)];
            let axis = x.ifc.add("IFCGRIDAXIS", args.clone());
            drawn.insert(axis, args);
            let target = if (line.end.x - line.start.x).abs() >= (line.end.y - line.start.y).abs() { &mut horizontal } else { &mut vertical };
            target.push((id.clone(), axis));
        }
        if vertical.is_empty() && horizontal.is_empty() {
            continue;
        }
        let ids = vertical.iter().chain(&horizontal).map(|(id, _)| id.as_str()).collect::<Vec<_>>().join(",");
        let again = |x: &mut Export<'_>, id: &str, axis: u64| -> (String, u64) {
            match (x.schema(), drawn.get(&axis)) {
                (Schema::Ifc4, Some(args)) => (id.to_string(), x.ifc.add("IFCGRIDAXIS", args.clone())),
                _ => (id.to_string(), axis),
            }
        };
        if vertical.is_empty() {
            let (id, axis) = horizontal[0].clone();
            vertical.push(again(x, &id, axis));
        }
        if horizontal.is_empty() {
            let (id, axis) = vertical[0].clone();
            horizontal.push(again(x, &id, axis));
        }
        let placement = x.ifc.place(Some(building.placement), x.ifc.origin);
        let axes = |rows: &[(String, u64)]| refs(&rows.iter().map(|(_, axis)| *axis).collect::<Vec<u64>>());
        let mut tail = vec![opt_text(&ids), rf(placement), unset(), axes(&vertical), axes(&horizontal), unset()];
        tail.extend(x.by(Vec::new(), vec![unset()]));
        let grid = x.ifc.rooted("IFCGRID", building_id, "", "", tail);
        x.links.elements.insert(format!("{building_id}:grid"), grid);
        x.links.in_building.entry(building_id.clone()).or_default().push(grid);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
