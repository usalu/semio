//! 🏘️ `zones`: what the zones and the area schemes of the model add up. A space names its zone; a zone names a purpose and an occupancy density; an area scheme names the
//! rule (a measure, the usages and the zones it counts). Everything here is derived from the take-off of the spaces (`quantities`), nothing is stored.
//!
//! Definitions. Areas in square metres, volumes in cubic metres, occupancy in persons.
//! * a space is a member of a zone when it names it; a member is `resolved` when its room is (`quantities` has a row for it);
//! * zone totals: the gross area (outline), the net floor area (without the columns), the volume, the occupancy `density * net floor area` and the three finish areas of its resolved members;
//! * an area scheme counts a space when its usage list is empty or names the usage of the space, and its zone list is empty or names the zone of the space (an empty zone list counts spaces
//!   in no zone too); its area is the gross area or the net floor area of the counted spaces, its volume and occupancy are summed the same way, the occupancy of a space being the density of its zone times its net floor area.
//!
//! The graph has one `Zone` node per zone and one `Scheme` node per scheme; their parents are the `Quantity` nodes of the spaces they add up.
//!
//! Related: DIN 277 and ISO 9836 area schemes (gross, net, rentable), <https://www.iso.org/standard/32443.html>.

use super::super::element_solids::{dep_object, dep_value};
use super::super::finishes::FinishSurface;
use super::super::quantities::ElementQuantity;
use crate::{AreaMeasure, AreaScheme, ModelSnapshot, Space, Zone};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

/// 🗺️ The snapshot collections the zone and scheme totals read, directly or through the take-off rows of the spaces.
pub const READS: &[&str] = &["spaces", "zones", "area_schemes", "walls", "wall_types", "curtain_walls", "curtain_wall_types", "openings", "window_types", "door_types", "columns", "column_types", "slabs", "slab_types", "ceilings", "ceiling_types", "materials", "storeys", "buildings", "sites"];

//#region 🔖️Values
/// 🏘️ What one zone adds up over the spaces that belong to it.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ZoneTotals {
    pub spaces: u32,
    pub resolved: u32,
    pub area: f64,
    pub net_area: f64,
    pub volume: f64,
    pub occupancy: f64,
    pub floor_finish_area: f64,
    pub wall_finish_area: f64,
    pub ceiling_finish_area: f64,
}

/// 🗃️ What one area scheme adds up over the spaces its rule counts.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SchemeTotals {
    pub spaces: u32,
    pub resolved: u32,
    pub area: f64,
    pub volume: f64,
    pub occupancy: f64,
}

/// 🧺️ A space with the take-off row of its room (absent while the room is unresolved).
pub type Member<'a> = (&'a Space, Option<&'a ElementQuantity>);
//#endregion 🔖️Values

//#region 🔖️Rules
/// 🗃️ Whether the rule of `scheme` counts `space`.
pub fn counts(scheme: &AreaScheme, space: &Space) -> bool {
    (scheme.usages.is_empty() || scheme.usages.contains(&space.usage)) && (scheme.zones.is_empty() || space.zone.as_ref().is_some_and(|zone| scheme.zones.contains(zone)))
}

fn occupancy_of(densities: &BTreeMap<&str, f64>, space: &Space, quantity: &ElementQuantity) -> f64 {
    space.zone.as_deref().and_then(|zone| densities.get(zone)).map_or(0.0, |density| density * quantity.net_area)
}

fn finish_area(quantity: &ElementQuantity, surface: FinishSurface) -> f64 {
    quantity.finishes.iter().filter(|row| row.surface == surface).map(|row| row.area).sum()
}
//#endregion 🔖️Rules

//#region 🔖️Totals
/// 🏘️ The totals of a zone over its members, summed in the order given (space id order).
pub fn zone_totals(zone: &Zone, members: &[Member<'_>]) -> ZoneTotals {
    let mut totals = ZoneTotals { spaces: members.len() as u32, ..ZoneTotals::default() };
    for quantity in members.iter().filter_map(|(_, quantity)| *quantity) {
        totals.resolved += 1;
        totals.area += quantity.gross_area;
        totals.net_area += quantity.net_area;
        totals.volume += quantity.net_volume;
        totals.occupancy += zone.occupancy_density * quantity.net_area;
        totals.floor_finish_area += finish_area(quantity, FinishSurface::Floor);
        totals.wall_finish_area += finish_area(quantity, FinishSurface::Wall);
        totals.ceiling_finish_area += finish_area(quantity, FinishSurface::Ceiling);
    }
    totals
}

/// 🗃️ The totals of an area scheme over the members its rule counts (`densities` maps zone ids to occupancy densities), summed in the order given.
pub fn scheme_totals(scheme: &AreaScheme, densities: &BTreeMap<&str, f64>, members: &[Member<'_>]) -> SchemeTotals {
    let counted: Vec<&Member<'_>> = members.iter().filter(|(space, _)| counts(scheme, space)).collect();
    let mut totals = SchemeTotals { spaces: counted.len() as u32, ..SchemeTotals::default() };
    for (space, quantity) in counted.into_iter().filter_map(|(space, quantity)| quantity.map(|quantity| (space, quantity))) {
        totals.resolved += 1;
        totals.area += match scheme.measure {
            AreaMeasure::Gross => quantity.gross_area,
            AreaMeasure::Net => quantity.net_area,
        };
        totals.volume += quantity.net_volume;
        totals.occupancy += occupancy_of(densities, space, quantity);
    }
    totals
}
//#endregion 🔖️Totals

//#region 🔖️Dependency
/// 🔑️ What `zone_totals` reads of the snapshot besides the take-off rows of the members (which are parents): the density of the zone and which spaces belong to it.
pub fn zone_dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let members: Vec<String> = snapshot.spaces.iter().filter(|(_, space)| space.zone.as_deref() == Some(id)).map(|(space, _)| space.clone()).collect();
    dep_object([("density", dep_value(&snapshot.zones.get(id).map(|zone| zone.occupancy_density))), ("members", dep_value(&members))])
}

/// 🔑️ What `scheme_totals` reads of the snapshot besides the take-off rows of the counted members (which are parents): the rule, the zone and usage of every space and the density of every zone.
pub fn scheme_dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let spaces = DslValue::object(snapshot.spaces.iter().map(|(space, row)| (space.clone(), dep_object([("usage", dep_value(&row.usage)), ("zone", dep_value(&row.zone))]))));
    let densities = DslValue::object(snapshot.zones.iter().map(|(zone, row)| (zone.clone(), dep_value(&row.occupancy_density))));
    let rule = snapshot.area_schemes.get(id).map_or(DslValue::Null, |scheme| dep_object([("measure", dep_value(&scheme.measure)), ("usages", dep_value(&scheme.usages)), ("zones", dep_value(&scheme.zones))]));
    dep_object([("rule", rule), ("spaces", spaces), ("densities", densities)])
}
//#endregion 🔖️Dependency

//#region 🔖️Projection
/// 🧾️ The table the third-party oracle reproduces.
#[derive(value_derive::ToValue)]
struct ZoneTable {
    zones: BTreeMap<String, ZoneTotals>,
    schemes: BTreeMap<String, SchemeTotals>,
}

/// 🧾️ The table the third-party oracle reproduces: the totals of every zone and of every area scheme.
pub fn table_json(zones: &BTreeMap<String, ZoneTotals>, schemes: &BTreeMap<String, SchemeTotals>) -> String {
    semio_framework_pack_json::to_json_string(&ZoneTable { zones: zones.clone(), schemes: schemes.clone() })
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
