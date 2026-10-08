//! 📊️ Native JSON inference table projection.
use std::collections::BTreeMap;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::spaces::{SpaceRoom, SpaceStatus};

/// 🧾️ One row of the table the third-party oracle reproduces.
#[derive(value_derive::ToValue)]
struct SpaceRow {
    status: SpaceStatus,
    area: f64,
    perimeter: f64,
    net_floor_area: f64,
    clear_height: f64,
    volume: f64,
    hole_count: u32,
    ceiling_slab: String,
    bounding_walls: Vec<String>,
}

/// 🧾️ The table the third-party oracle reproduces: status, areas, clear height, volume, island count, ceiling slab and bounding walls of every space.
pub fn table_json(rooms: &BTreeMap<String, SpaceRoom>) -> String {
    let rows: BTreeMap<String, SpaceRow> = rooms
        .iter()
        .map(|(id, room)| {
            let row = SpaceRow { status: room.status, area: room.area, perimeter: room.perimeter, net_floor_area: room.net_floor_area, clear_height: room.clear_height, volume: room.volume, hole_count: room.holes.len() as u32, ceiling_slab: room.ceiling_slab.clone(), bounding_walls: room.bounding_walls.clone() };
            (id.clone(), row)
        })
        .collect();
    semio_framework_pack_json::to_json_string(&rows)
}
