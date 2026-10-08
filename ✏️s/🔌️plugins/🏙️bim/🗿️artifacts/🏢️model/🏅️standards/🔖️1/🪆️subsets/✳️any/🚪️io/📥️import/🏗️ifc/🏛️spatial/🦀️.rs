//! 🏛️ Project, site, building and storeys of an IFC file. A storey's level comes from the `Level` it was exported with, else from the order of the storey elevations (the lowest at or above
//! zero is level 0); its height from `GrossHeight`, else from the distance to the storey above.

use super::reader::{real, reals, text, Doc};
use super::Import;
use crate::{Building, Point2, Site, Storey};
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::BTreeMap;

/// 🧭️ Degrees of an `IfcCompoundPlaneAngleMeasure` (degrees, minutes, seconds, millionths of a second; the sign sits on every part).
pub fn degrees(parts: &[f64]) -> f64 {
    let sign = parts.iter().find(|part| **part != 0.0).map_or(1.0, |part| part.signum());
    let at = |index: usize| parts.get(index).copied().unwrap_or(0.0).abs();
    sign * (at(0) + at(1) / 60.0 + at(2) / 3600.0 + at(3) / 3_600_000_000.0)
}

/// 🏷️ The `Semio_Authoring` rows of an instance: the authored parameters IFC has no slot for.
pub fn authoring_of(doc: &Doc<'_>, ifc: u64) -> BTreeMap<String, Part21Value> {
    let mut found = BTreeMap::new();
    for definition in doc.index.definitions.get(&ifc).into_iter().flatten() {
        if let Some(args) = doc.args(*definition, "IFCPROPERTYSET") {
            if text(args, 2) == "Semio_Authoring" {
                found.extend(single_values(doc, args));
            }
        }
    }
    found
}

/// 🏷️ The `name → nominal value` rows of a property set's single-value properties.
pub fn single_values(doc: &Doc<'_>, set: &[Part21Value]) -> BTreeMap<String, Part21Value> {
    set.get(4)
        .and_then(Part21Value::as_list)
        .unwrap_or_default()
        .iter()
        .filter_map(|item| doc.follow_args(item, "IFCPROPERTYSINGLEVALUE"))
        .map(|args| (text(args, 0), args[2].clone()))
        .collect()
}

/// 🔢️ The number inside a typed value.
pub fn number_of(value: &Part21Value) -> Option<f64> {
    value.as_typed().and_then(|(_, items)| items.first()).and_then(Part21Value::as_real).or_else(|| value.as_real())
}

/// 🔤️ The string inside a typed value.
pub fn string_of(value: &Part21Value) -> Option<String> {
    value.as_typed().and_then(|(_, items)| items.first()).and_then(Part21Value::as_str).or_else(|| value.as_str()).map(str::to_string)
}

fn quantity(doc: &Doc<'_>, ifc: u64, name: &str) -> Option<f64> {
    doc.index.definitions.get(&ifc)?.iter().filter_map(|definition| doc.args(*definition, "IFCELEMENTQUANTITY")).flat_map(|set| set[5].as_list().unwrap_or_default().iter()).filter_map(|item| doc.follow_args(item, "IFCQUANTITYLENGTH")).find(|args| text(args, 0) == name).and_then(|args| real(args, 3))
}

fn project(i: &mut Import<'_>) -> f64 {
    let Some((_, args)) = i.doc.rows("IFCPROJECT").into_iter().next() else {
        i.skip("IFCPROJECT", "-", "the file has no project");
        return 0.0;
    };
    i.model.project.name = text(args, 2);
    i.model.project.description = text(args, 3);
    i.model.project.phase_names = Some(text(args, 6)).filter(|phases| !phases.is_empty()).map(|phases| phases.split(';').map(str::to_string).collect()).unwrap_or_default();
    if let Some(history) = i.doc.follow_args(&args[1], "IFCOWNERHISTORY") {
        if let Some(account) = i.doc.follow_args(&history[0], "IFCPERSONANDORGANIZATION") {
            i.model.project.author = i.doc.follow_args(&account[0], "IFCPERSON").map(|person| text(person, 1)).unwrap_or_default();
            i.model.project.organization = i.doc.follow_args(&account[1], "IFCORGANIZATION").map(|organization| text(organization, 1)).unwrap_or_default();
        }
    }
    let context = args[7].as_list().and_then(|items| items.first()).and_then(|item| i.doc.follow_args(item, "IFCGEOMETRICREPRESENTATIONCONTEXT"));
    context.and_then(|context| context.get(5)).and_then(|north| i.doc.direction(north)).map_or(0.0, |north| super::frames::snap((-north[0]).atan2(north[1])))
}

fn sites(i: &mut Import<'_>, true_north: f64) {
    for (index, (instance, args)) in i.doc.rows("IFCSITE").into_iter().enumerate() {
        let id = Doc::identity(args, "IFCSITE").unwrap_or_else(|| format!("site-{}", index + 1));
        let boundary = i.doc.footprint(args).map(|ring| ring.into_iter().map(|(point, _)| Point2 { x: point[0], y: point[1] }).collect()).unwrap_or_default();
        i.model.sites.insert(id.clone(), Site { name: text(args, 2), latitude: degrees(&reals(args, 9)), longitude: degrees(&reals(args, 10)), elevation: real(args, 11).unwrap_or(0.0), true_north, boundary });
        i.site_world.insert(id.clone(), i.doc.world(&args[5]));
        i.ids.insert(instance.id, id);
    }
    if i.model.sites.is_empty() {
        i.model.sites.insert("site-1".into(), Site { name: String::new(), latitude: 0.0, longitude: 0.0, elevation: 0.0, true_north, boundary: Vec::new() });
        i.site_world.insert("site-1".into(), super::frames::Rigid::IDENTITY);
    }
}

fn buildings(i: &mut Import<'_>) {
    let first_site = i.model.sites.keys().next().cloned().unwrap_or_default();
    for (index, (instance, args)) in i.doc.rows("IFCBUILDING").into_iter().enumerate() {
        let id = Doc::identity(args, "IFCBUILDING").unwrap_or_else(|| format!("bldg-{}", index + 1));
        let site = i.doc.index.whole.get(&instance.id).and_then(|whole| i.ids.get(whole)).cloned().unwrap_or_else(|| first_site.clone());
        let world = i.doc.world(&args[5]);
        let relative = world.relative_to(i.site_world.get(&site).unwrap_or(&super::frames::Rigid::IDENTITY));
        let site_elevation = i.model.sites.get(&site).map_or(0.0, |row| row.elevation);
        let elevation = real(args, 9).map_or(relative.origin[2], |absolute| absolute - site_elevation);
        i.model.buildings.insert(id.clone(), Building { site, name: text(args, 2), origin: Point2 { x: relative.origin[0], y: relative.origin[1] }, rotation: relative.heading(), elevation });
        i.building_world.insert(id.clone(), world);
        i.ids.insert(instance.id, id);
    }
    if i.model.buildings.is_empty() {
        i.model.buildings.insert("bldg-1".into(), Building { site: first_site, name: String::new(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 });
        i.building_world.insert("bldg-1".into(), super::frames::Rigid::IDENTITY);
    }
}

struct Found {
    elevation: f64,
    id: String,
    ifc: u64,
    name: String,
    level: Option<i32>,
    height: Option<f64>,
}

fn storeys(i: &mut Import<'_>) {
    let first_building = i.model.buildings.keys().next().cloned().unwrap_or_default();
    let mut found: BTreeMap<String, Vec<Found>> = BTreeMap::new();
    for (index, (instance, args)) in i.doc.rows("IFCBUILDINGSTOREY").into_iter().enumerate() {
        let id = Doc::identity(args, "IFCBUILDINGSTOREY").unwrap_or_else(|| format!("st-{}", index + 1));
        let building = i.doc.index.whole.get(&instance.id).and_then(|whole| i.ids.get(whole)).cloned().unwrap_or_else(|| first_building.clone());
        let level = authoring_of(&i.doc, instance.id).get("Level").and_then(number_of).map(|level| level.round() as i32);
        let elevation = real(args, 9).unwrap_or(0.0);
        found.entry(building).or_default().push(Found { elevation, id, ifc: instance.id, name: text(args, 2), level, height: quantity(&i.doc, instance.id, "GrossHeight") });
    }
    for (building, mut rows) in found {
        rows.sort_by(|a, b| a.elevation.total_cmp(&b.elevation));
        let datum = rows.iter().position(|row| row.elevation >= -1e-9).unwrap_or(rows.len());
        let above: Vec<f64> = rows.iter().map(|row| row.elevation).skip(1).collect();
        for (index, row) in rows.into_iter().enumerate() {
            let level = row.level.unwrap_or(index as i32 - datum as i32);
            let height = row.height.or_else(|| above.get(index).map(|next| next - row.elevation)).unwrap_or(3.0);
            i.model.storeys.insert(row.id.clone(), Storey { building: building.clone(), name: row.name, level, height, cut_height: None });
            i.storey_ids.insert(row.ifc, row.id.clone());
            i.ids.insert(row.ifc, row.id);
        }
    }
}

/// 🏛️ Reads project, sites, buildings and storeys into the model.
pub fn read(i: &mut Import<'_>) {
    let true_north = project(i);
    sites(i, true_north);
    buildings(i);
    storeys(i);
}

/// 📝️ One note per product class that was present but not imported, with its count.
pub fn report_unsupported(i: &mut Import<'_>) {
    const CLASSES: [&str; 22] = [
        "IFCWALL", "IFCWALLSTANDARDCASE", "IFCSLAB", "IFCCOLUMN", "IFCBEAM", "IFCROOF", "IFCSTAIR", "IFCSTAIRFLIGHT", "IFCRAILING", "IFCCURTAINWALL", "IFCMEMBER", "IFCPLATE", "IFCWINDOW", "IFCDOOR", "IFCOPENINGELEMENT", "IFCSPACE", "IFCBUILDINGELEMENTPROXY", "IFCFURNISHINGELEMENT", "IFCCOVERING", "IFCRAMP", "IFCFOOTING", "IFCDISTRIBUTIONELEMENT",
    ];
    for class in CLASSES {
        let missing = i.doc.rows(class).into_iter().filter(|(instance, _)| !i.ids.contains_key(&instance.id)).count();
        if missing > 0 {
            i.notes.push(format!("{class}: {missing} not imported (no parametric equivalent or no swept outline)"));
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
