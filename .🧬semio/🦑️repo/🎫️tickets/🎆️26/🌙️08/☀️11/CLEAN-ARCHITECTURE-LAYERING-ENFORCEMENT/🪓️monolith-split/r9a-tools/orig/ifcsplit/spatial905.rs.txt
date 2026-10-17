//! 🏛️ The spatial structure: `IfcProject` with units and context, `IfcSite` with its compound-angle position, `IfcBuilding`, `IfcBuildingStorey`
//! and the `IfcRelAggregates` chain between them.

use super::writer::{en, int, list, opt_text, real, refs, rf, typed, unset, V};
use super::{BuildingRef, Export, Quantity, StoreyRef};

/// 🧭️ `IfcCompoundPlaneAngleMeasure` of an angle in degrees: sign on every component, millionths of a second last.
pub fn compound_angle(degrees: f64) -> V {
    let sign = if degrees < 0.0 { -1 } else { 1 };
    let micro = (degrees.abs() * 3600e6).round() as i64;
    let (whole, rest) = (micro / 3_600_000_000, micro % 3_600_000_000);
    let (minutes, rest) = (rest / 60_000_000, rest % 60_000_000);
    list(vec![int(sign * whole), int(sign * minutes), int(sign * (rest / 1_000_000)), int(sign * (rest % 1_000_000))])
}

/// 🏛️ Writes project, sites, buildings and storeys.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    let units = x.ifc.units();
    let phases = model.project.phase_names.join(";");
    let project = x.ifc.rooted("IFCPROJECT", "project", &model.project.name, &model.project.description, vec![unset(), unset(), opt_text(&phases), refs(&[x.ifc.context]), rf(units)]);
    x.links.elements.insert(":project".into(), project);
    let mut site_entities = std::collections::BTreeMap::new();
    let mut site_placements = std::collections::BTreeMap::new();
    for (id, site) in &model.sites {
        let origin = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
        let placement = x.ifc.place(None, origin);
        let footprint = if site.boundary.len() >= 3 {
            let vertices: Vec<([f64; 2], f64)> = site.boundary.iter().map(|point| ([point.x, point.y], 0.0)).collect();
            let curve = x.ifc.loop_curve(&vertices);
            let shape = x.ifc.shape(x.ifc.footprint, "FootPrint", "Curve2D", &[curve]);
            rf(x.ifc.definition(&[shape]))
        } else {
            unset()
        };
        let entity = x.ifc.rooted("IFCSITE", id, &site.name, "", vec![opt_text(id), rf(placement), footprint, unset(), en("ELEMENT"), compound_angle(site.latitude), compound_angle(site.longitude), real(site.elevation), unset(), unset()]);
        site_placements.insert(id.clone(), placement);
        site_entities.insert(id.clone(), entity);
        x.links.elements.insert(id.clone(), entity);
    }
    x.links.aggregated.insert(project, site_entities.values().copied().collect());
    for (id, building) in &model.buildings {
        let Some(site_placement) = site_placements.get(&building.site).copied() else {
            x.skip("building", id, "its site is missing");
            continue;
        };
        let direction = (building.rotation.abs() > 1e-12).then(|| [building.rotation.cos(), building.rotation.sin(), 0.0]);
        let axis = x.ifc.axis3([building.origin.x, building.origin.y, building.elevation], None, direction);
        let placement = x.ifc.place(Some(site_placement), axis);
        let datum = model.storeys.iter().filter(|(_, storey)| storey.building == *id).find_map(|(storey, _)| x.inferred.storey_levels.get(storey)).map(|level| level.absolute_elevation - level.elevation);
        let absolute = datum.unwrap_or(building.elevation + model.sites.get(&building.site).map_or(0.0, |site| site.elevation));
        let entity = x.ifc.rooted("IFCBUILDING", id, &building.name, "", vec![opt_text(id), rf(placement), unset(), unset(), en("ELEMENT"), real(absolute), unset(), unset()]);
        x.buildings.insert(id.clone(), BuildingRef { ifc: entity, placement });
        x.links.elements.insert(id.clone(), entity);
        x.links.aggregated.entry(site_entities[&building.site]).or_default().push(entity);
    }
    for (id, storey) in &model.storeys {
        let (Some(building), Some(level)) = (x.buildings.get(&storey.building).copied(), x.inferred.storey_levels.get(id).copied()) else {
            x.skip("storey", id, "its building is missing");
            continue;
        };
        let axis = x.ifc.axis3([0.0, 0.0, level.elevation], None, None);
        let placement = x.ifc.place(Some(building.placement), axis);
        let entity = x.ifc.rooted("IFCBUILDINGSTOREY", id, &storey.name, "", vec![opt_text(id), rf(placement), unset(), unset(), en("ELEMENT"), real(level.elevation)]);
        x.storeys.insert(id.clone(), StoreyRef { ifc: entity, placement, elevation: level.elevation });
        x.links.aggregated.entry(building.ifc).or_default().push(entity);
        x.links.elements.insert(id.clone(), entity);
        x.links.quantities.push((entity, "Qto_BuildingStoreyBaseQuantities", vec![Quantity::Length("GrossHeight", storey.height), Quantity::Length("NetHeight", storey.height)]));
        x.links.authoring.push((entity, vec![("Level", typed("IFCINTEGER", int(i64::from(storey.level))))]));
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
