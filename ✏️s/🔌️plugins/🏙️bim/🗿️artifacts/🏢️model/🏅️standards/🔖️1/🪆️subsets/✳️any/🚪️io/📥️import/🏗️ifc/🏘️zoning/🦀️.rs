//! 🏘️ Zones, area schemes and room finishes of an IFC file: every `IfcZone` becomes a zone (its category, and the occupancy density it was exported with), the spaces assigned to it through
//! `IfcRelAssignsToGroup` join it, every `IfcGroup` of `ObjectType` `AreaScheme` becomes an area scheme (the measure and the counted usages and zones are read from `Semio_Authoring`), and the finish of a
//! space is the material its `Semio_Authoring` names, else the material of the same name that `Pset_SpaceCoveringRequirements` names. A file that was not exported by this model brings no
//! density, so its zones start without occupancy; a covering that names no known material is reported, not invented.

use super::data::{label, number};
use super::reader::{opt_text, refs, text};
use super::spatial::{authoring_of, single_values};
use super::Import;
use crate::{AreaMeasure, AreaScheme, Zone};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::BTreeMap;

/// 🪪️ The set whose rows the importer maps to finishes instead of to user properties.
pub const COVERING_SET: &str = "Pset_SpaceCoveringRequirements";

fn covering_of(i: &Import<'_>, ifc: u64) -> BTreeMap<String, Part21Value> {
    let mut found = BTreeMap::new();
    for definition in i.doc.index.definitions.get(&ifc).into_iter().flatten() {
        if let Some(args) = i.doc.args(*definition, "IFCPROPERTYSET") {
            if text(args, 2) == COVERING_SET {
                found.extend(single_values(&i.doc, args));
            }
        }
    }
    found
}

fn material_named(i: &Import<'_>, name: &str) -> Option<String> {
    i.model.materials.iter().find(|(_, material)| material.name == name).map(|(id, _)| id.clone())
}

fn finish(i: &Import<'_>, authoring: &BTreeMap<String, Part21Value>, covering: &BTreeMap<String, Part21Value>, authored: &str, standard: &str) -> Option<String> {
    label(authoring, authored).filter(|material| i.model.materials.contains_key(material)).or_else(|| label(covering, standard).and_then(|name| material_named(i, &name)))
}

fn finishes(i: &mut Import<'_>) {
    let spaces: Vec<(u64, String)> = i.ids.iter().filter(|(_, id)| i.model.spaces.contains_key(*id)).map(|(ifc, id)| (*ifc, id.clone())).collect();
    for (ifc, id) in spaces {
        let (authoring, covering) = (authoring_of(&i.doc, ifc), covering_of(i, ifc));
        let found = (finish(i, &authoring, &covering, "FloorFinish", "FloorCovering"), finish(i, &authoring, &covering, "WallFinish", "WallCovering"), finish(i, &authoring, &covering, "CeilingFinish", "CeilingCovering"));
        for (row, material) in [("FloorCovering", &found.0), ("WallCovering", &found.1), ("CeilingCovering", &found.2)] {
            if material.is_none() && label(&covering, row).is_some() {
                i.skip("IFCSPACE", &id, &format!("{row} names no material of the model"));
            }
        }
        if let Some(space) = i.model.spaces.get_mut(&id) {
            (space.floor_finish, space.wall_finish, space.ceiling_finish) = found;
        }
    }
}

fn free(i: &Import<'_>, candidate: &str) -> bool {
    crate::mutations::elements::taken(&i.model, candidate).is_none()
}

fn zones(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCZONE") {
        let rows = authoring_of(&i.doc, instance.id);
        let name = text(args, 2);
        let id = label(&rows, "Id").filter(|candidate| free(i, candidate)).unwrap_or_else(|| Import::unused(&format!("z-{}", Import::slug(&name)), |candidate| !free(i, candidate)));
        let category = label(&rows, "Category").or_else(|| opt_text(args, 4)).unwrap_or_default();
        i.model.zones.insert(id.clone(), Zone { name, category, occupancy_density: number(&rows, "OccupancyDensity").filter(|density| density.is_finite() && *density >= 0.0).unwrap_or(0.0) });
        i.ids.insert(instance.id, id);
    }
    for (_, args) in i.doc.rows("IFCRELASSIGNSTOGROUP") {
        let Some(zone) = args.get(6).and_then(Part21Value::as_ref_id).and_then(|group| i.ids.get(&group)).filter(|zone| i.model.zones.contains_key(*zone)).cloned() else { continue };
        for object in refs(args, 4) {
            if let Some(space) = i.ids.get(&object).cloned().and_then(|id| i.model.spaces.get_mut(&id)) {
                space.zone = Some(zone.clone());
            }
        }
    }
}

fn list(rows: &BTreeMap<String, Part21Value>, name: &str) -> Vec<String> {
    label(rows, name).and_then(|text| from_json_str::<Vec<String>>(&text, JsonMemberPolicy::Reject).ok()).unwrap_or_default()
}

fn schemes(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCGROUP").into_iter().filter(|(_, args)| text(args, 4) == "AreaScheme") {
        let rows = authoring_of(&i.doc, instance.id);
        let name = text(args, 2);
        let id = label(&rows, "Id").filter(|candidate| free(i, candidate)).unwrap_or_else(|| Import::unused(&format!("as-{}", Import::slug(&name)), |candidate| !free(i, candidate)));
        let measure = if label(&rows, "Measure").as_deref() == Some("Gross") { AreaMeasure::Gross } else { AreaMeasure::Net };
        let zones: Vec<String> = list(&rows, "Zones").into_iter().filter(|zone| i.model.zones.contains_key(zone)).collect();
        i.model.area_schemes.insert(id.clone(), AreaScheme { name, measure, usages: list(&rows, "Usages").into_iter().filter(|usage| !usage.trim().is_empty()).collect(), zones });
        i.ids.insert(instance.id, id);
    }
}

/// 🏘️ Reads the finishes of the spaces, then the zones with their members, then the area schemes.
pub fn read(i: &mut Import<'_>) {
    finishes(i);
    zones(i);
    schemes(i);
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
