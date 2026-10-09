//! 🎯️ Which nodes a diff can have moved: the per-key twin of [`READS`](super::READS). `touched` says whether the regions a diff wrote (`<collection>/<id>[/<field>]`) intersect the snapshot rows the dependency of a node reads
//! (`compute::dependency`, the honesty contract of `dep_input`), so an incremental update evaluates the dependency of those nodes only. Rows are named by the snapshot after the diff: a changed reference means the referring row
//! was written. Granularity is the row; an element kind that reads aggregates (rooms, plans, annotations, diagnostics, schedules, zones, views, phases) answers yes, and so does every kind this table does not know.

use super::ModelNode;
use super::super::super::element_solids::{SolidFamily, SolidKey};
use super::plan::element_storey;
use crate::{ModelSnapshot, OpeningKind};
use protocol::TouchedPaths;

struct Regions<'a>(&'a TouchedPaths);

impl Regions<'_> {
    fn row(&self, collection: &str, id: &str) -> bool {
        self.0.intersects_parts(&[collection, id])
    }

    fn collection(&self, collection: &str) -> bool {
        self.0.intersects_parts(&[collection])
    }

    fn kinds(&self) -> bool {
        ["wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "curtain_wall_types"].into_iter().any(|collection| self.collection(collection))
    }

    fn placements(&self) -> bool {
        self.collection("storeys") || self.collection("buildings")
    }
}

fn type_of<'a>(snapshot: &'a ModelSnapshot, id: &str) -> Option<(&'static str, &'a str)> {
    snapshot
        .walls
        .get(id)
        .map(|row| ("wall_types", row.wall_type.as_str()))
        .or_else(|| snapshot.curtain_walls.get(id).map(|row| ("curtain_wall_types", row.curtain_wall_type.as_str())))
        .or_else(|| snapshot.slabs.get(id).map(|row| ("slab_types", row.slab_type.as_str())))
        .or_else(|| snapshot.ceilings.get(id).map(|row| ("ceiling_types", row.ceiling_type.as_str())))
        .or_else(|| snapshot.roofs.get(id).map(|row| ("roof_types", row.roof_type.as_str())))
        .or_else(|| snapshot.columns.get(id).map(|row| ("column_types", row.column_type.as_str())))
        .or_else(|| snapshot.beams.get(id).map(|row| ("beam_types", row.beam_type.as_str())))
        .or_else(|| {
            snapshot.openings.get(id).and_then(|row| match &row.kind {
                OpeningKind::Window { window_type } => Some(("window_types", window_type.as_str())),
                OpeningKind::Door { door_type } => Some(("door_types", door_type.as_str())),
                OpeningKind::Void { .. } => None,
            })
        })
}

fn home_of(snapshot: &ModelSnapshot, id: &str) -> Option<&'static str> {
    if snapshot.walls.contains_key(id) {
        return Some("walls");
    }
    if snapshot.openings.contains_key(id) {
        return Some("openings");
    }
    [
        ("curtain_walls", snapshot.curtain_walls.contains_key(id)),
        ("columns", snapshot.columns.contains_key(id)),
        ("beams", snapshot.beams.contains_key(id)),
        ("slabs", snapshot.slabs.contains_key(id)),
        ("ceilings", snapshot.ceilings.contains_key(id)),
        ("roofs", snapshot.roofs.contains_key(id)),
        ("stairs", snapshot.stairs.contains_key(id)),
        ("ramps", snapshot.ramps.contains_key(id)),
        ("railings", snapshot.railings.contains_key(id)),
        ("spaces", snapshot.spaces.contains_key(id)),
    ]
    .into_iter()
    .find_map(|(collection, present)| present.then_some(collection))
}

fn element(regions: &Regions<'_>, snapshot: &ModelSnapshot, id: &str) -> bool {
    let Some(home) = home_of(snapshot, id) else { return true };
    regions.row(home, id) || (regions.kinds() && type_of(snapshot, id).is_some_and(|(collection, kind)| regions.row(collection, kind)))
}

const HOLDERS: &[&str] = &["sites", "buildings", "storeys", "walls", "curtain_walls", "columns", "beams", "slabs", "ceilings", "roofs", "openings", "stairs", "ramps", "railings", "spaces", "zones", "area_schemes", "views", "wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types"];

fn properties(regions: &Regions<'_>, id: &str) -> bool {
    regions.row("properties", id) || regions.collection("property_templates") || HOLDERS.iter().any(|collection| regions.row(collection, id))
}

fn placement(regions: &Regions<'_>, snapshot: &ModelSnapshot, storey: &str) -> bool {
    regions.placements() && (regions.row("storeys", storey) || snapshot.storeys.get(storey).is_some_and(|row| regions.row("buildings", &row.building)))
}

fn host(regions: &Regions<'_>, snapshot: &ModelSnapshot, id: &str) -> bool {
    let Some(row) = snapshot.openings.get(id) else { return false };
    regions.row("walls", &row.host) || regions.row("curtain_walls", &row.host)
}

fn railing_host(regions: &Regions<'_>, snapshot: &ModelSnapshot, id: &str) -> bool {
    snapshot.railings.get(id).and_then(|row| row.host.as_ref()).is_some_and(|spec| regions.row("ramps", &spec.element) || regions.row("slabs", &spec.element))
}

fn solid(regions: &Regions<'_>, snapshot: &ModelSnapshot, key: &SolidKey) -> bool {
    let id = key.id.as_str();
    element(regions, snapshot, id) || host(regions, snapshot, id) || (key.family == SolidFamily::CurtainWall && (regions.collection("curtain_panel_overrides") || regions.collection("door_types") || regions.collection("window_types"))) || (key.family == SolidFamily::Beam && (regions.collection("columns") || regions.collection("column_types"))) || (regions.placements() && placement(regions, snapshot, &element_storey(snapshot, id).unwrap_or_default())) || (key.family == SolidFamily::Railing && (regions.collection("materials") || railing_host(regions, snapshot, id)))
}

fn quantity(regions: &Regions<'_>, snapshot: &ModelSnapshot, id: &str) -> bool {
    snapshot.spaces.contains_key(id) || element(regions, snapshot, id) || host(regions, snapshot, id) || railing_host(regions, snapshot, id) || regions.collection("materials")
}

fn storey(regions: &Regions<'_>, snapshot: &ModelSnapshot, id: &str) -> bool {
    regions.row("storeys", id) || snapshot.storeys.get(id).is_some_and(|row| regions.row("buildings", &row.building) || snapshot.buildings.get(&row.building).is_some_and(|building| regions.row("sites", &building.site)))
}

fn hosting(regions: &Regions<'_>, snapshot: &ModelSnapshot, id: &str) -> bool {
    regions.row("walls", id) || regions.row("curtain_walls", id) || snapshot.walls.get(id).is_some_and(|row| placement(regions, snapshot, &row.storey)) || snapshot.curtain_walls.get(id).is_some_and(|row| placement(regions, snapshot, &row.storey) || regions.row("curtain_wall_types", &row.curtain_wall_type))
}

/// 🎯️ Whether the regions `touched` may change the dependency of `key` in `snapshot` (the snapshot after the diff).
pub fn touched(snapshot: &ModelSnapshot, key: &ModelNode, touched: &TouchedPaths) -> bool {
    let regions = Regions(touched);
    match key {
        ModelNode::Storey(id) => storey(&regions, snapshot, id),
        ModelNode::Band(id) | ModelNode::WallLayout(id) => element(&regions, snapshot, id),
        ModelNode::Cut(id) | ModelNode::OpeningFrame(id) => element(&regions, snapshot, id),
        ModelNode::CurtainLayout(id) => element(&regions, snapshot, id) || regions.collection("curtain_panel_overrides"),
        ModelNode::Host(id) => hosting(&regions, snapshot, id),
        ModelNode::StairRun(id) => regions.row("stairs", id),
        ModelNode::Properties(id) => properties(&regions, id),
        ModelNode::RampRun(id) => regions.row("ramps", id),
        ModelNode::Solid(solid_key) => solid(&regions, snapshot, solid_key),
        ModelNode::Quantity(id) => quantity(&regions, snapshot, id),
        ModelNode::Family(id) => regions.row("families", id) || regions.collection("family_parameters") || regions.collection("family_solids") || regions.collection("materials"),
        ModelNode::Totals(_) | ModelNode::DiagnosticIndex => false,
        ModelNode::Sheet(id) => regions.row("sheets", id) || regions.collection("viewports") || regions.collection("sheet_revisions") || regions.collection("views"),
        _ => true,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
