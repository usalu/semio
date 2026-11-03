//! 📍️ Layout: where every spatial entity stands. Persisted placements win; otherwise the documented default rule applies (golden-tested):
//! buildings run along `+X` from the origin with a 12 m gap and a footprint of `Lx` by `Ly`; satellite rows sit at fixed `y` offsets with a 30 m slot.

use super::kit::v3;
use super::model::En1998Snapshot;
use super::scene::Vec3;
use std::collections::BTreeMap;

pub use super::scene::projection_kit::Placement;

pub const BUILDING_GAP: f64 = 12.0;
pub const ROW_WALLS: f64 = -30.0;
pub const ROW_BRIDGES: f64 = -70.0;
pub const ROW_SILOS: f64 = 50.0;
pub const ROW_TANKS: f64 = 90.0;
pub const ROW_TOWERS: f64 = 130.0;
pub const ROW_SLOPES: f64 = 170.0;
pub const SLOT: f64 = 30.0;

/// 📍️ All resolved origins of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    origins: BTreeMap<String, Vec3>,
}

impl Layout {
    pub fn of(document: &En1998Snapshot, placements: &[Placement]) -> Self {
        let placed = |id: &str| placements.iter().find(|p| p.entity_id == id).map(|p| p.origin.clone());
        let mut origins = BTreeMap::new();
        let mut cursor = 0.0;
        for building in &document.buildings {
            let origin = placed(&building.id).unwrap_or_else(|| v3(cursor, 0.0, 0.0));
            cursor += building.plan_width_m.max(1.0) + if building.joint_gap_m > 0.0 { building.joint_gap_m } else { BUILDING_GAP };
            origins.insert(building.id.clone(), origin);
        }
        let rows: [(f64, Vec<&str>); 6] = [
            (ROW_WALLS, document.retaining_walls.iter().map(|e| e.id.as_str()).collect()),
            (ROW_BRIDGES, document.bridges.iter().map(|e| e.id.as_str()).collect()),
            (ROW_SILOS, document.silos.iter().map(|e| e.id.as_str()).collect()),
            (ROW_TANKS, document.tanks.iter().map(|e| e.id.as_str()).collect()),
            (ROW_TOWERS, document.towers.iter().map(|e| e.id.as_str()).collect()),
            (ROW_SLOPES, document.slopes.iter().map(|e| e.id.as_str()).collect()),
        ];
        for (y, ids) in rows {
            for (slot, id) in ids.iter().enumerate() {
                origins.insert((*id).to_string(), placed(id).unwrap_or_else(|| v3(slot as f64 * SLOT, y, 0.0)));
            }
        }
        Self { origins }
    }

    pub fn origin(&self, id: &str) -> Vec3 {
        self.origins.get(id).cloned().unwrap_or(Vec3::ZERO)
    }
}
