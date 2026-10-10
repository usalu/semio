//! 📊️ The table the third-party oracle reproduces from the exported file: for every space that is written the conditioned flag, heated flag, floor area, volume and its surfaces merged by kind, boundary, neighbour and
//! compass sector (area and `sum(U * A)`), and the totals of every scope. The values are the inference's, seen as the export keeps them: a window or door behind a partition is part of its wall, a curtain wall and a
//! surface without thermal data are not written, and the totals are added up again over what is left.
//! 📎 ../../🧾️json/🦀️.rs

use super::build::selection;
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::{totals_of, Boundary, Climate, EnergyScope, EnergyTotals, EnvelopeSpace, EnvelopeSurface, SurfaceKind};
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::ModelSnapshot;
use std::collections::BTreeMap;

/// 🧾️ The area and heat loss `sum(U * A)` of the surfaces of one space that share a kind, a boundary, a neighbour and a sector.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Group {
    pub area: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub loss: Option<f64>,
}

/// 🧾️ One written space.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct Row {
    pub conditioned: bool,
    pub heated: bool,
    pub floor_area: f64,
    pub volume: f64,
    pub groups: BTreeMap<String, Group>,
}

/// 🧾️ The table of an export.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct EnergyTable {
    pub spaces: BTreeMap<String, Row>,
    pub totals: BTreeMap<String, EnergyTotals>,
}

fn key(surface: &EnvelopeSurface) -> String {
    let sector = if surface.vertical() { surface.sector().to_string() } else { "-".to_string() };
    format!("{}|{}|{}|{}", surface.kind.name(), surface.boundary.name(), surface.adjacent, sector)
}

/// 🧱️ The envelope of a written space as the file keeps it.
pub fn written_envelope(envelope: &EnvelopeSpace) -> EnvelopeSpace {
    let mut kept = envelope.clone();
    let exterior = |id: &str| envelope.surfaces.iter().any(|host| host.id == id && host.boundary == Boundary::Exterior);
    let dropped_walls: Vec<String> = envelope.surfaces.iter().filter(|surface| !matches!(surface.kind, SurfaceKind::Window | SurfaceKind::Door) && (surface.u_value.is_none() || (surface.kind == SurfaceKind::CurtainWall && surface.boundary != Boundary::Exterior))).map(|surface| surface.id.clone()).collect();
    kept.surfaces.retain(|surface| !dropped_walls.contains(&surface.id) && !dropped_walls.contains(&surface.parent));
    let merged: Vec<(String, f64)> = kept.surfaces.iter().filter(|surface| matches!(surface.kind, SurfaceKind::Window | SurfaceKind::Door) && !exterior(&surface.parent)).map(|surface| (surface.parent.clone(), surface.gross_area)).collect();
    kept.surfaces.retain(|surface| !(matches!(surface.kind, SurfaceKind::Window | SurfaceKind::Door) && (!exterior(&surface.parent) || surface.u_value.is_none())));
    for (host, area) in merged {
        if let Some(surface) = kept.surfaces.iter_mut().find(|surface| surface.id == host) {
            surface.area += area;
        }
    }
    kept
}

fn scope_of(key: &str) -> Option<EnergyScope> {
    match key.split_once(':') {
        Some(("zone", id)) => Some(EnergyScope::Zone(id.to_string())),
        Some(("building", id)) => Some(EnergyScope::Building(id.to_string())),
        _ if key == "project" => Some(EnergyScope::Project),
        _ => None,
    }
}

/// 📊️ The table of the export of `snapshot` from its inference.
pub fn table_of(snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<EnergyTable, String> {
    let chosen = selection(snapshot, inferred)?;
    let envelopes: Vec<EnvelopeSpace> = chosen.spaces.iter().filter_map(|id| inferred.energy_envelopes.get(id)).map(written_envelope).collect();
    let mut spaces = BTreeMap::new();
    for envelope in &envelopes {
        let mut groups: BTreeMap<String, Group> = BTreeMap::new();
        for surface in &envelope.surfaces {
            let group = groups.entry(key(surface)).or_insert(Group { area: 0.0, loss: Some(0.0) });
            group.area += surface.area;
            group.loss = group.loss.zip(surface.u_value).map(|(loss, u_value)| loss + u_value * surface.area);
        }
        spaces.insert(envelope.space.clone(), Row { conditioned: envelope.conditioned, heated: envelope.heated, floor_area: envelope.floor_area, volume: envelope.volume, groups });
    }
    let climate = Climate::of(snapshot);
    let members: Vec<&EnvelopeSpace> = envelopes.iter().collect();
    let totals = inferred.energy_totals.keys().filter_map(|name| scope_of(name).map(|scope| (name.clone(), totals_of(&scope, &members, &climate)))).collect();
    Ok(EnergyTable { spaces, totals })
}

/// 📊️ The table as canonical JSON text.
pub fn table_json(snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<String, String> {
    table_of(snapshot, inferred).map(|table| semio_framework_pack_json::to_json_string(&table))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
