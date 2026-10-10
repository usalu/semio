//! 📊️ The table the gbXML oracle compares: what the file must show, derived from the plan the writer builds (never read back from the committed file). The lxml + shapely oracle reads the same table off the file alone: element counts, the spaces with their
//! areas, volumes, conditions and set points, every surface by its `CADObjectId` with type, adjacent spaces, orientation, size, polygon area, construction U-value and openings, the constructions with their layers, the window types, and the polygon area per surface type.
//! 📎 https://www.gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html

use super::{ConstructionRow, OpeningRow, Plan, SpaceRow, SurfaceRow};
use crate::ModelSnapshot;
use std::collections::BTreeMap;

#[derive(value_derive::ToValue)]
struct Counts {
    campuses: usize,
    buildings: usize,
    storeys: usize,
    spaces: usize,
    zones: usize,
    surfaces: usize,
    openings: usize,
    constructions: usize,
    layers: usize,
    materials: usize,
    window_types: usize,
}

#[derive(value_derive::ToValue)]
struct SpaceTable {
    name: String,
    storey: String,
    zone: String,
    condition: String,
    area: f64,
    volume: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    heating: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    cooling: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    people: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    lighting: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    equipment: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    outdoor_air: Option<f64>,
}

#[derive(value_derive::ToValue)]
struct OpeningTable {
    kind: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    window_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    construction: Option<String>,
    area: f64,
    width: f64,
    height: f64,
    offset: Vec<f64>,
}

#[derive(value_derive::ToValue)]
struct SurfaceTable {
    cad: Vec<String>,
    kind: String,
    exposed: bool,
    spaces: Vec<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    construction: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    u_value: Option<f64>,
    azimuth: f64,
    tilt: f64,
    width: f64,
    height: f64,
    area: f64,
    openings: BTreeMap<String, OpeningTable>,
}

#[derive(value_derive::ToValue)]
struct ConstructionTable {
    u_value: f64,
    layers: Vec<String>,
}

#[derive(value_derive::ToValue)]
struct WindowTypeTable {
    #[value(default, skip_serializing_if = "Option::is_none")]
    u_value: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    g_value: Option<f64>,
    description: String,
}

#[derive(value_derive::ToValue)]
struct Totals {
    by_type: BTreeMap<String, f64>,
    opening_area: f64,
}

#[derive(value_derive::ToValue)]
struct GbxmlTable {
    counts: Counts,
    spaces: BTreeMap<String, SpaceTable>,
    surfaces: BTreeMap<String, SurfaceTable>,
    constructions: BTreeMap<String, ConstructionTable>,
    window_types: BTreeMap<String, WindowTypeTable>,
    totals: Totals,
}

fn opening_table(row: &OpeningRow, plan: &Plan) -> OpeningTable {
    let window_type = row.window_type.as_ref().and_then(|xml| plan.window_types.values().find(|kind| kind.xml == *xml)).map(|kind| kind.name.clone());
    let construction = row.construction.as_ref().and_then(|xml| plan.door_constructions.values().find(|kind| kind.xml == *xml)).map(|kind| kind.name.clone());
    OpeningTable { kind: row.kind.to_string(), window_type, construction, area: row.area, width: row.rect.width, height: row.rect.height, offset: row.offset.to_vec() }
}

fn surface_table(row: &SurfaceRow, plan: &Plan) -> SurfaceTable {
    let space_cad = |xml: &String| plan.spaces.iter().find(|space| space.xml == *xml).map(|space| space.cad.clone()).unwrap_or_default();
    let construction = row.construction.as_ref().and_then(|xml| plan.constructions.iter().find(|kind| kind.xml == *xml)).map(|kind| kind.name.clone());
    SurfaceTable {
        cad: row.cad.clone(),
        kind: row.kind.to_string(),
        exposed: row.exposed,
        spaces: row.spaces.iter().map(space_cad).collect(),
        construction,
        u_value: row.construction.as_ref().and_then(|xml| plan.constructions.iter().find(|kind| kind.xml == *xml)).map(|kind| kind.u_value),
        azimuth: row.azimuth,
        tilt: row.tilt,
        width: row.rect.width,
        height: row.rect.height,
        area: row.area,
        openings: row.openings.iter().map(|opening| (opening.cad.clone(), opening_table(opening, plan))).collect(),
    }
}

fn space_table(row: &SpaceRow, plan: &Plan, model: &ModelSnapshot) -> SpaceTable {
    let zone = plan.zones.iter().find(|zone| zone.xml == row.zone);
    SpaceTable {
        name: row.name.clone(),
        storey: model.storeys.get(&row.storey).map(|storey| storey.name.clone()).unwrap_or_default(),
        zone: zone.map(|zone| zone.name.clone()).unwrap_or_default(),
        condition: row.condition.to_string(),
        area: row.area,
        volume: row.volume,
        heating: zone.and_then(|zone| zone.heating),
        cooling: zone.and_then(|zone| zone.cooling),
        people: row.people,
        lighting: row.lighting,
        equipment: row.equipment,
        outdoor_air: row.outdoor_air,
    }
}

fn construction_table(row: &ConstructionRow, plan: &Plan) -> ConstructionTable {
    let material = |layer: &String| {
        let source = plan.layers.iter().find(|(_, own)| *own == layer).map(|(material, _)| material.clone());
        source.and_then(|xml| plan.materials.values().find(|row| row.xml == xml)).map(|row| row.name.clone()).unwrap_or_default()
    };
    ConstructionTable { u_value: row.u_value, layers: row.layers.iter().map(material).collect() }
}

/// 📊️ The canonical JSON table of a plan; the oracle reads the same table off the written file.
pub fn table_json(plan: &Plan, model: &ModelSnapshot) -> String {
    let mut by_type: BTreeMap<String, f64> = BTreeMap::new();
    for row in &plan.surfaces {
        *by_type.entry(row.kind.to_string()).or_default() += row.area;
    }
    let table = GbxmlTable {
        counts: Counts {
            campuses: plan.sites.len(),
            buildings: plan.buildings.len(),
            storeys: plan.storeys.len(),
            spaces: plan.spaces.len(),
            zones: plan.zones.len(),
            surfaces: plan.surfaces.len(),
            openings: plan.surfaces.iter().map(|row| row.openings.len()).sum(),
            constructions: plan.constructions.len() + plan.door_constructions.len(),
            layers: plan.layers.len(),
            materials: plan.layers.len(),
            window_types: plan.window_types.len(),
        },
        spaces: plan.spaces.iter().map(|row| (row.cad.clone(), space_table(row, plan, model))).collect(),
        surfaces: plan.surfaces.iter().map(|row| (row.cad[0].clone(), surface_table(row, plan))).collect(),
        constructions: plan.constructions.iter().chain(plan.door_constructions.values()).map(|row| (row.name.clone(), construction_table(row, plan))).collect(),
        window_types: plan.window_types.values().map(|row| (row.name.clone(), WindowTypeTable { u_value: row.u_value, g_value: row.g_value, description: row.description.clone() })).collect(),
        totals: Totals { by_type, opening_area: plan.surfaces.iter().flat_map(|row| row.openings.iter()).map(|opening| opening.area).sum() },
    };
    semio_framework_pack_json::to_json_string(&table)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
