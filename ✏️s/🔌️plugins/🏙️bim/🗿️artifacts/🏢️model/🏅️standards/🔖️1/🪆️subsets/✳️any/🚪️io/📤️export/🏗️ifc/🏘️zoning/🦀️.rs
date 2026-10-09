//! 🏘️ Zones, area schemes and room finishes in the IFC file. A zone is an `IfcZone` (its `ObjectType` the category) that groups its spaces through `IfcRelAssignsToGroup`, with `Pset_ZoneCommon.Reference`, the
//! authored occupancy density in `Semio_Authoring` and the derived totals as the element quantity `Semio_ZoneTotals`. An area scheme has no IFC entity of its own: it is an `IfcGroup` of `ObjectType`
//! `AreaScheme` whose `Semio_Authoring` rows carry the measure and the counted usages and zones as JSON arrays. The finish of a space is `Pset_SpaceCoveringRequirements` (`FloorCovering`, `WallCovering`,
//! `CeilingCovering` name the materials) plus the ids in `Semio_Authoring`, so the importer finds the material again.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcproductextension/lexical/ifczone.htm

use super::data::{define, label, number, property_set};
use super::writer::{opt_text, refs, rf, text, typed, unset, V};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::zones::ZoneTotals;
use crate::ModelSnapshot;
use std::collections::BTreeMap;

/// 🎨️ The finish rows of one space: the material ids in its authoring data and the covering property set that names the materials.
pub fn finishes(x: &mut Export<'_>, id: &str, entity: u64) {
    let model = x.model;
    let Some(space) = model.spaces.get(id) else { return };
    let surfaces: [(&'static str, &'static str, Option<&str>); 3] = [("FloorFinish", "FloorCovering", space.floor_finish.as_deref()), ("WallFinish", "WallCovering", space.wall_finish.as_deref()), ("CeilingFinish", "CeilingCovering", space.ceiling_finish.as_deref())];
    let authoring: Vec<(&'static str, V)> = surfaces.iter().filter_map(|(row, _, material)| material.map(|material| (*row, label(material)))).collect();
    if authoring.is_empty() {
        return;
    }
    x.links.authoring.push((entity, authoring));
    let covering: Vec<(&str, V)> = surfaces.iter().filter_map(|(_, row, material)| material.map(|material| (*row, label(&model.materials.get(material).map_or_else(|| material.to_string(), |row| row.name.clone()))))).collect();
    let key = format!("{id}:Pset_SpaceCoveringRequirements:finish");
    let set = property_set(x, &key, "Pset_SpaceCoveringRequirements", covering);
    define(x, &key, &[entity], set);
}

fn totals(entity: u64, totals: &ZoneTotals) -> (u64, &'static str, Vec<Quantity>) {
    (entity, "Semio_ZoneTotals", vec![Quantity::Area("GrossFloorArea", totals.area), Quantity::Area("NetFloorArea", totals.net_area), Quantity::Volume("GrossVolume", totals.volume), Quantity::Area("FloorFinishArea", totals.floor_finish_area), Quantity::Area("WallFinishArea", totals.wall_finish_area), Quantity::Area("CeilingFinishArea", totals.ceiling_finish_area)])
}

/// 🏘️ Writes every zone with its membership, then every area scheme as a group.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, zone) in &model.zones {
        let entity = x.ifc.rooted("IFCZONE", id, &zone.name, "", vec![opt_text(&zone.category)]);
        x.links.elements.insert(id.clone(), entity);
        x.links.authoring.push((entity, vec![("Id", label(id)), ("Category", label(&zone.category)), ("OccupancyDensity", number(zone.occupancy_density))]));
        let common = property_set(x, &format!("{id}:Pset_ZoneCommon"), "Pset_ZoneCommon", vec![("Reference", typed("IFCIDENTIFIER", text(id)))]);
        define(x, &format!("{id}:Pset_ZoneCommon"), &[entity], common);
        let members: Vec<u64> = model.spaces.iter().filter(|(_, space)| space.zone.as_deref() == Some(id.as_str())).filter_map(|(space, _)| x.links.elements.get(space).copied()).collect();
        if !members.is_empty() {
            x.ifc.rooted("IFCRELASSIGNSTOGROUP", &format!("{id}:members"), "", "", vec![refs(&members), unset(), rf(entity)]);
        }
        if let Some(row) = x.inferred.zone_totals.get(id) {
            x.links.quantities.push(totals(entity, row));
        }
    }
    for (id, scheme) in &model.area_schemes {
        let entity = x.ifc.rooted("IFCGROUP", id, &scheme.name, "", vec![text("AreaScheme")]);
        x.links.elements.insert(id.clone(), entity);
        x.links.authoring.push((entity, vec![("Id", label(id)), ("Measure", label(&format!("{:?}", scheme.measure))), ("Usages", label(&semio_framework_pack_json::to_json_string(&scheme.usages))), ("Zones", label(&semio_framework_pack_json::to_json_string(&scheme.zones)))]));
    }
}

//#region 🔖️Report
/// 🏘️ What the subject says one zone is in the file: its name, category (object type), reference (`Pset_ZoneCommon`), authored density, member spaces and the derived totals of `Semio_ZoneTotals`.
#[derive(value_derive::ToValue)]
struct ZoneReport {
    name: String,
    category: String,
    reference: String,
    occupancy_density: f64,
    members: Vec<String>,
    totals: BTreeMap<String, f64>,
}

/// 🗃️ What the subject says one area scheme is in the file: a group of type `AreaScheme` with its rule.
#[derive(value_derive::ToValue)]
struct SchemeReport {
    name: String,
    measure: String,
    usages: Vec<String>,
    zones: Vec<String>,
}

#[derive(value_derive::ToValue)]
struct Report {
    zones: BTreeMap<String, ZoneReport>,
    schemes: BTreeMap<String, SchemeReport>,
    coverings: BTreeMap<String, BTreeMap<String, String>>,
}

fn totals_of(totals: &ZoneTotals) -> BTreeMap<String, f64> {
    [("GrossFloorArea", totals.area), ("NetFloorArea", totals.net_area), ("GrossVolume", totals.volume), ("FloorFinishArea", totals.floor_finish_area), ("WallFinishArea", totals.wall_finish_area), ("CeilingFinishArea", totals.ceiling_finish_area)].into_iter().map(|(name, value)| (name.to_string(), value)).collect()
}

/// 🧾️ The table the third-party IfcOpenShell oracle reads back from the exported file: every zone with its members and totals, every area scheme with its rule and the covering names of every finished space.
pub fn report_json(model: &ModelSnapshot) -> Result<String, String> {
    let zone_totals = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::try_with_inference(None, model, |inferred| inferred.zone_totals.clone()).map_err(|error| error.to_string())?;
    let zones = model
        .zones
        .iter()
        .map(|(id, zone)| {
            let members = model.spaces.iter().filter(|(_, space)| space.zone.as_deref() == Some(id.as_str())).map(|(space, _)| space.clone()).collect();
            let totals = zone_totals.get(id).map(totals_of).unwrap_or_default();
            (id.clone(), ZoneReport { name: zone.name.clone(), category: zone.category.clone(), reference: id.clone(), occupancy_density: zone.occupancy_density, members, totals })
        })
        .collect();
    let schemes = model.area_schemes.iter().map(|(id, scheme)| (id.clone(), SchemeReport { name: scheme.name.clone(), measure: format!("{:?}", scheme.measure), usages: scheme.usages.clone(), zones: scheme.zones.clone() })).collect();
    let name = |material: &str| model.materials.get(material).map_or_else(|| material.to_string(), |row| row.name.clone());
    let coverings = model
        .spaces
        .iter()
        .filter_map(|(id, space)| {
            let rows: BTreeMap<String, String> = [("FloorCovering", &space.floor_finish), ("WallCovering", &space.wall_finish), ("CeilingCovering", &space.ceiling_finish)].into_iter().filter_map(|(row, material)| material.as_deref().map(|material| (row.to_string(), name(material)))).collect();
            (!rows.is_empty()).then(|| (id.clone(), rows))
        })
        .collect();
    Ok(semio_framework_pack_json::to_json_string(&Report { zones, schemes, coverings }))
}
//#endregion 🔖️Report

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
