//! 🏗️ The energy model of a BIM model, built stage by stage from the thermal envelope the model graph inferred (`energy_envelopes`) and the authored records the envelope does not carry (layer stacks and their materials,
//! the conditions of a space, the site). Nothing of the geometry is recomputed: a surface keeps the polygon, boundary, neighbour, azimuth-bearing area and transmittance the inference gave it. Every stage reads the model
//! and the inference afresh and keeps no borrow, so a job runs one stage per step and stops between two.
//!
//! The mapping, and what it cannot say:
//! * a space is a zone (set points are per space); a zone of the BIM model groups its zones in a thermal enclosure;
//! * wall, floor and ceiling become surfaces of the class and boundary their inferred boundary names; the layers of their type are the construction, listed outside first (the stack of a wall is the type's list read from the
//!   interior face when the room lies on the left of the wall axis, else mirrored; a floor reads its slab from the top, a ceiling its slab or roof from the top). A partner of an `Adjacent` surface is an `Interzone` boundary, an
//!   adiabatic one stays adiabatic; a surface without a partner is adiabatic;
//! * windows and doors in an exterior surface are fenestrations (the SHGC of a window is its g-value times the glazed share, a door is opaque to the sun); one behind a partition is merged into the wall, because the engine
//!   exposes every fenestration to the weather; a curtain wall is a host surface with a massless construction carrying one fenestration of its whole area (U, g-value and mullion share of its type), one behind a partition and surfaces without thermal data are not written;
//! * the loads and set points of the conditions become people, lighting, equipment, a thermostat and an ideal loads system; a schedule name becomes a constant or a weekday window.

use super::library::{Library, Profile, ScheduleBook};
use super::pairs::{self, Face, Part};
use super::target::{AdjacencyPair, EnergyModel, EquipmentGain, Fenestration, IdealLoadsSystem, LightingGain, OutsideBoundary, PeopleGain, Site, Space, Surface, SurfaceClass, ThermalEnclosure, Thermostat, Zone};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::thermal::{self, FarSide, HeatFlow};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::{neighbour_storey, Boundary, EnvelopeSpace, EnvelopeSurface, SurfaceKind};
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::Vec3;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::{Axis, Layer, ModelSnapshot};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;
use std::collections::BTreeMap;

/// 🌡️ The heating set point of a space that is not heated: no temperature the zone reaches calls for heat.
pub const NO_HEATING_C: f64 = -100.0;
/// 🌡️ The cooling set point of a space that is not cooled.
pub const NO_COOLING_C: f64 = 100.0;
/// 👤️ The metabolic rate in watts per person the engine assumes for people gains (office work), which the activity schedule states.
pub const ACTIVITY_W: f64 = 100.0;
/// 👤️ The share of the heat of a person that is sensible, the share that is latent follows, and the radiant share of the sensible heat.
pub const PEOPLE_SENSIBLE: f64 = 0.58;
pub const PEOPLE_RADIANT: f64 = 0.3;
/// 💡️ The radiant and visible shares of lighting power.
pub const LIGHTING_RADIANT: f64 = 0.7;
pub const LIGHTING_VISIBLE: f64 = 0.2;
/// 🔌️ The radiant share of equipment power.
pub const EQUIPMENT_RADIANT: f64 = 0.3;
/// ❄️ The supply air limits of the ideal loads system, in degrees Celsius.
pub const SUPPLY_HEATING_C: f64 = 50.0;
pub const SUPPLY_COOLING_C: f64 = 13.0;
/// 🏷️ The version string of a model this bridge writes.
pub const VERSION: &str = "semio-bim-1";

/// 🔗️ A surface written, as the stages after it need it.
#[derive(Clone, Debug)]
struct Placed {
    index: usize,
    boundary: Boundary,
}

/// 🏗️ The state of an energy model being built: what the stages before have written and the ids they gave.
#[derive(Default)]
pub struct Build {
    pub notes: Vec<String>,
    next: u32,
    model: Option<EnergyModel>,
    library: Library,
    book: ScheduleBook,
    spaces: Vec<String>,
    zone_ids: BTreeMap<String, u32>,
    surface_ids: BTreeMap<String, u32>,
    placed: Vec<Placed>,
    faces: Vec<(usize, Face)>,
}

/// 🪜️ One stage of the build: the name of what it writes and the function that writes it.
pub type Stage = (&'static str, fn(&mut Build, &ModelSnapshot, &ModelInference) -> Result<(), String>);

/// 🪜️ The stages in the order they run.
pub const STAGES: &[Stage] = &[("site", site), ("zones", zones), ("surfaces", surfaces), ("openings", openings), ("partners", partners), ("loads", loads), ("model", seal)];

impl Build {
    fn id(&mut self) -> u32 {
        self.next += 1;
        self.next
    }

    fn model(&mut self) -> Result<&mut EnergyModel, String> {
        self.model.as_mut().ok_or_else(|| "the site stage has not run".to_string())
    }

    /// 🏁️ The model and the notes once every stage has run.
    pub fn finish(&mut self) -> Result<(EnergyModel, Vec<String>), String> {
        let model = self.model.take().ok_or_else(|| "the model was already taken".to_string())?;
        Ok((model, std::mem::take(&mut self.notes)))
    }
}

fn building_of<'a>(snapshot: &'a ModelSnapshot, storey: &str) -> Option<(&'a str, &'a crate::Building)> {
    let storey = snapshot.storeys.get(storey)?;
    snapshot.buildings.get_key_value(&storey.building).map(|(id, building)| (id.as_str(), building))
}

fn bearing_of(snapshot: &ModelSnapshot, storey: &str) -> Option<(String, f64)> {
    let (_, building) = building_of(snapshot, storey)?;
    let site = snapshot.sites.get(&building.site)?;
    Some((building.site.clone(), site.true_north - building.rotation))
}

fn written(envelope: &EnvelopeSpace) -> bool {
    envelope.floor_area > 0.0 && envelope.volume > 0.0 && !envelope.surfaces.is_empty()
}

//#region 🔖️Site
/// 🧭️ Which spaces are written and where they stand: the spaces with a room and a thermal envelope of the first building's site and orientation (the energy model has one site and one north).
pub struct Selection {
    pub site: String,
    pub bearing: f64,
    pub spaces: Vec<String>,
    pub notes: Vec<String>,
}

/// 🧭️ The spaces an export writes.
pub fn selection(snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<Selection, String> {
    let candidates: Vec<&EnvelopeSpace> = inferred.energy_envelopes.values().filter(|envelope| written(envelope)).collect();
    let Some(first) = candidates.first() else { return Err("the model has no space with a thermal envelope: state the conditions of at least one space with a room".to_string()) };
    let (site, bearing) = bearing_of(snapshot, &first.storey).ok_or_else(|| format!("space {} lies in no building on a site", first.space))?;
    let (mut spaces, mut notes) = (Vec::new(), Vec::new());
    for envelope in &candidates {
        match bearing_of(snapshot, &envelope.storey) {
            Some((other, own)) if other == site && (own - bearing).abs() < 1e-9 => spaces.push(envelope.space.clone()),
            _ => notes.push(format!("space {} stands in a building on another site or turned differently and is not written: the energy model has one site and one north", envelope.space)),
        }
    }
    Ok(Selection { site, bearing, spaces, notes })
}

fn site(build: &mut Build, snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<(), String> {
    let chosen = selection(snapshot, inferred)?;
    let row = snapshot.sites.get(&chosen.site).ok_or_else(|| format!("site {} is missing", chosen.site))?;
    let site = Site { latitude_deg: row.latitude, longitude_deg: row.longitude, elevation_m: row.elevation, time_zone_hours: (row.longitude / 15.0).round(), north_axis_deg: chosen.bearing.to_degrees().rem_euclid(360.0) };
    build.spaces = chosen.spaces;
    build.notes.extend(chosen.notes);
    build.model = Some(EnergyModel::new(&snapshot.project.name, VERSION, site));
    Ok(())
}
//#endregion 🔖️Site

//#region 🔖️Zones
fn zones(build: &mut Build, snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<(), String> {
    let mut rows: Vec<(Zone, Space)> = Vec::new();
    for id in build.spaces.clone() {
        let (Some(envelope), Some(space)) = (inferred.energy_envelopes.get(&id), snapshot.spaces.get(&id)) else { continue };
        let zone = build.id();
        build.zone_ids.insert(id.clone(), zone);
        let name = if space.name.trim().is_empty() { id.clone() } else { format!("{id} {}", space.name.trim()) };
        let entry = build.id();
        rows.push((Zone { id: zone, name, volume_m3: envelope.volume, multiplier: 1, conditioned: envelope.conditioned, part_of_total_floor_area: envelope.conditioned }, Space { id: entry, name: space.name.clone(), zone_id: zone, floor_area_m2: envelope.floor_area }));
    }
    let mut enclosures = Vec::new();
    for (zone_id, zone) in &snapshot.zones {
        let members: Vec<u32> = build.spaces.iter().filter(|id| snapshot.spaces.get(*id).and_then(|space| space.zone.as_deref()) == Some(zone_id.as_str())).filter_map(|id| build.zone_ids.get(id).copied()).collect();
        if !members.is_empty() {
            let id = build.id();
            enclosures.push(ThermalEnclosure { id, name: if zone.name.trim().is_empty() { zone_id.clone() } else { zone.name.clone() }, zone_ids: members });
        }
    }
    let model = build.model()?;
    for (zone, space) in rows {
        model.zones.push(zone);
        model.spaces.push(space);
    }
    model.thermal_enclosures = enclosures;
    Ok(())
}
//#endregion 🔖️Zones

//#region 🔖️Surfaces
fn far_side(boundary: Boundary) -> FarSide {
    match boundary {
        Boundary::Exterior => FarSide::Outdoor,
        Boundary::Ground => FarSide::Ground,
        Boundary::Adjacent | Boundary::Adiabatic => FarSide::Room,
    }
}

fn stack(snapshot: &ModelSnapshot, layers: &[Layer]) -> Option<Vec<(f64, f64)>> {
    layers.iter().map(|layer| snapshot.materials.get(&layer.material).map(|material| (layer.thickness, material.conductivity))).collect()
}

fn same(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * left.abs().max(right.abs()).max(1.0)
}

/// 🧱️ The layer stacks a floor (`up` false) or ceiling can lie on, in the order the inference prefers them: the slabs of a storey, the thickest first; for a ceiling the slabs of the storey above and then the roofs of the storey.
fn candidates<'a>(snapshot: &'a ModelSnapshot, storey: &str, up: bool) -> Vec<(&'a str, &'a [Layer])> {
    let slabs = |storey: &str| {
        let mut found: Vec<(&str, &[Layer], f64)> = snapshot.slabs.iter().filter(|(_, slab)| slab.storey == storey).filter_map(|(id, slab)| snapshot.slab_types.get(&slab.slab_type).map(|kind| (kind.name.as_str(), kind.layers.as_slice(), kind.layers.iter().map(|layer| layer.thickness).sum::<f64>()))).collect();
        found.sort_by(|a, b| b.2.total_cmp(&a.2));
        found.into_iter().map(|(name, layers, _)| (name, layers)).collect::<Vec<_>>()
    };
    if !up {
        return slabs(storey);
    }
    let mut found = neighbour_storey(snapshot, storey, true).map(|above| slabs(&above)).unwrap_or_default();
    found.extend(snapshot.roofs.iter().filter(|(_, roof)| roof.storey == storey).filter_map(|(_, roof)| snapshot.roof_types.get(&roof.roof_type).map(|kind| (kind.name.as_str(), kind.layers.as_slice()))));
    found
}

/// 🧭️ Whether the room lies on the left of the axis of the wall a surface is on: the surface runs counter-clockwise around the room, so the room is on the left of the axis where the two run the same way.
fn room_on_left(snapshot: &ModelSnapshot, surface: &EnvelopeSurface) -> bool {
    let (Some(wall), [first, second, ..]) = (snapshot.walls.get(&surface.element), surface.polygon.as_slice()) else { return true };
    let direction = [second.x - first.x, second.y - first.y];
    let middle = Point::new((first.x + second.x) / 2.0, (first.y + second.y) / 2.0);
    let segment = match &wall.axis {
        Axis::Line { start, end } => BulgeSeg::line(Point::new(start.x, start.y), Point::new(end.x, end.y)),
        Axis::Arc { start, end, bulge } => BulgeSeg::new(Point::new(start.x, start.y), Point::new(end.x, end.y), *bulge),
    };
    let tangent = segment.tangent_at(segment.closest(middle).t);
    direction[0] * tangent.x + direction[1] * tangent.y >= 0.0
}

/// 🧱️ The layers a surface's holder gives it, as the holder's type lists them, with whether the list runs from the room outwards in the wall sense (interior face first).
fn layers_of(snapshot: &ModelSnapshot, surface: &EnvelopeSurface) -> Option<(String, Vec<Layer>)> {
    let kind = &surface.construction;
    if snapshot.walls.contains_key(&surface.holder) {
        snapshot.wall_types.get(kind).map(|row| (row.name.clone(), row.layers.clone()))
    } else if snapshot.slabs.contains_key(&surface.holder) {
        snapshot.slab_types.get(kind).map(|row| (row.name.clone(), row.layers.clone()))
    } else if snapshot.roofs.contains_key(&surface.holder) {
        snapshot.roof_types.get(kind).map(|row| (row.name.clone(), row.layers.clone()))
    } else if snapshot.ceilings.contains_key(&surface.holder) {
        snapshot.ceiling_types.get(kind).map(|row| (row.name.clone(), row.layers.clone()))
    } else {
        None
    }
}

/// 🧱️ The construction of a surface, outside first: a wall's stack is read from the interior face when the room lies on the left of the axis, a floor's slab and a ceiling's slab, ceiling or roof from the top.
fn construction_of(build: &mut Build, snapshot: &ModelSnapshot, surface: &EnvelopeSurface) -> Option<u32> {
    let (name, mut layers) = layers_of(snapshot, surface)?;
    let (reverse, mirrored) = match surface.kind {
        SurfaceKind::Wall => {
            let left = room_on_left(snapshot, surface);
            (left, !left)
        }
        SurfaceKind::Floor => (true, false),
        _ => (false, false),
    };
    if reverse {
        layers.reverse();
    }
    let title = if mirrored { format!("{name} (mirrored)") } else { name };
    let Build { library, next, .. } = build;
    library.construction(snapshot, &title, &layers, next)
}

fn class_of(surface: &EnvelopeSurface) -> SurfaceClass {
    match (surface.kind, surface.boundary) {
        (SurfaceKind::Floor, _) => SurfaceClass::Floor,
        (SurfaceKind::Ceiling, Boundary::Exterior) => SurfaceClass::Roof,
        (SurfaceKind::Ceiling, _) => SurfaceClass::Ceiling,
        (_, Boundary::Exterior | Boundary::Ground) => SurfaceClass::ExteriorWall,
        (_, Boundary::Adjacent) => SurfaceClass::Interzone,
        (_, Boundary::Adiabatic) => SurfaceClass::Adiabatic,
    }
}

fn outside_of(boundary: Boundary) -> OutsideBoundary {
    match boundary {
        Boundary::Exterior => OutsideBoundary::OutdoorAir,
        Boundary::Ground => OutsideBoundary::Ground,
        Boundary::Adjacent | Boundary::Adiabatic => OutsideBoundary::Adiabatic,
    }
}

fn polygon(surface: &EnvelopeSurface) -> Vec<[f64; 3]> {
    surface.polygon.iter().map(|vertex| [vertex.x, vertex.y, vertex.z]).collect()
}

fn surfaces(build: &mut Build, snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<(), String> {
    for space in build.spaces.clone() {
        let Some(envelope) = inferred.energy_envelopes.get(&space) else { continue };
        let zone_id = *build.zone_ids.get(&space).ok_or_else(|| format!("space {space} has no zone"))?;
        for surface in &envelope.surfaces {
            if matches!(surface.kind, SurfaceKind::Window | SurfaceKind::Door) {
                continue;
            }
            if surface.kind == SurfaceKind::CurtainWall && surface.boundary != Boundary::Exterior {
                build.notes.push(format!("{}: a curtain wall behind a partition is not written: the engine exposes every fenestration to the weather", surface.id));
                continue;
            }
            let Some(u_value) = surface.u_value else {
                build.notes.push(format!("{}: the {} {} has no thermal data and is not written", surface.id, surface.kind.name(), surface.element));
                continue;
            };
            let curtain = surface.kind == SurfaceKind::CurtainWall;
            let host = if curtain { Some(build.library.curtain_host(&mut build.next)) } else { construction_of(build, snapshot, surface) };
            let Some(construction_id) = host else {
                build.notes.push(format!("{}: its holder {} has no layer stack in the model and it is not written", surface.id, surface.holder));
                continue;
            };
            let id = build.id();
            build.surface_ids.insert(surface.id.clone(), id);
            let exterior = surface.boundary == Boundary::Exterior;
            let vertices = polygon(surface);
            let face = matches!(surface.boundary, Boundary::Adjacent | Boundary::Adiabatic) && !surface.adjacent.is_empty();
            let part = match surface.kind {
                SurfaceKind::Floor => Part::Floor,
                SurfaceKind::Ceiling => Part::Ceiling,
                _ => Part::Wall,
            };
            let centre = pairs::centre_of(&vertices);
            let model = build.model()?;
            model.surfaces.push(Surface { id, name: surface.id.clone(), zone_id, class: class_of(surface), vertices_m: vertices, construction_id, outside_boundary_condition: outside_of(surface.boundary), sun_exposed: exterior, wind_exposed: exterior, multiplier: 1 });
            let index = model.surfaces.len() - 1;
            build.placed.push(Placed { index, boundary: surface.boundary });
            if face {
                build.faces.push((build.placed.len() - 1, Face { space: space.clone(), adjacent: surface.adjacent.clone(), part, element: surface.element.clone(), centre, area: surface.gross_area }));
            }
        }
    }
    Ok(())
}
//#endregion 🔖️Surfaces

//#region 🔖️Openings
fn extent(polygon: &[Vec3]) -> (f64, f64) {
    polygon.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), vertex| (low.min(vertex.z), high.max(vertex.z)))
}

fn openings(build: &mut Build, _snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<(), String> {
    for space in build.spaces.clone() {
        let Some(envelope) = inferred.energy_envelopes.get(&space) else { continue };
        for surface in envelope.surfaces.iter().filter(|surface| matches!(surface.kind, SurfaceKind::Window | SurfaceKind::Door | SurfaceKind::CurtainWall)) {
            let host = if surface.kind == SurfaceKind::CurtainWall { Some(surface) } else { envelope.surfaces.iter().find(|host| host.id == surface.parent) };
            let Some(host_id) = host.filter(|host| host.boundary == Boundary::Exterior).and_then(|host| build.surface_ids.get(&host.id).copied()) else {
                build.notes.push(format!("{}: the {} {} is merged into its wall, because the engine exposes every fenestration to the weather and its wall is not a written exterior surface", surface.id, surface.kind.name(), surface.element));
                continue;
            };
            let Some(u_value) = surface.u_value else {
                build.notes.push(format!("{}: the {} {} has no thermal data and is not written", surface.id, surface.kind.name(), surface.element));
                continue;
            };
            let door = surface.kind == SurfaceKind::Door;
            let sunless = door;
            let shgc = if sunless {
                0.0
            } else {
                if surface.g_value.is_none() {
                    build.notes.push(format!("{}: the window {} has no g-value and lets no sun in", surface.id, surface.element));
                }
                (surface.g_value.unwrap_or(0.0) * (1.0 - surface.frame_fraction)).clamp(0.0, 1.0)
            };
            let (low, high) = extent(&surface.polygon);
            let host_low = host.map_or(low, |host| extent(&host.polygon).0);
            let id = build.id();
            let name = format!("{} {}", surface.kind.name(), surface.id);
            build.model()?.fenestrations.push(Fenestration {
                id,
                name,
                surface_id: host_id,
                u_value_w_m2k: u_value,
                shgc,
                vlt: shgc,
                area_m2: surface.gross_area,
                height_m: high - low,
                sill_height_m: low - host_low,
                frame_conductance_w_k: 0.0,
                divider_conductance_w_k: 0.0,
                overhang_depth_m: 0.0,
                overhang_offset_m: 0.0,
                fin_depth_m: 0.0,
                fin_offset_m: 0.0,
                glazing_construction_id: None,
                vertices_m: polygon(surface),
            });
        }
    }
    Ok(())
}
//#endregion 🔖️Openings

//#region 🔖️Partners
fn partners(build: &mut Build, _snapshot: &ModelSnapshot, _inferred: &ModelInference) -> Result<(), String> {
    let faces: Vec<Face> = build.faces.iter().map(|(_, face)| face.clone()).collect();
    let mut paired = vec![false; faces.len()];
    let mut links: Vec<(usize, usize)> = Vec::new();
    for (low, high) in pairs::pair(&faces) {
        paired[low] = true;
        paired[high] = true;
        links.push((build.faces[low].0, build.faces[high].0));
    }
    let unpaired: Vec<usize> = build.faces.iter().enumerate().filter(|(index, _)| !paired[*index]).map(|(_, (placed, _))| *placed).collect();
    for (a, b) in links {
        let (first, second) = (build.placed[a].index, build.placed[b].index);
        let boundary = build.placed[a].boundary;
        let model = build.model()?;
        let (id_a, id_b) = (model.surfaces[first].id, model.surfaces[second].id);
        if boundary == Boundary::Adjacent {
            model.surfaces[first].outside_boundary_condition = OutsideBoundary::Interzone(id_b);
            model.surfaces[second].outside_boundary_condition = OutsideBoundary::Interzone(id_a);
        }
        model.adjacency_pairs.push(AdjacencyPair { surface_a_id: id_a.min(id_b), surface_b_id: id_a.max(id_b) });
    }
    for placed in unpaired {
        if build.placed[placed].boundary == Boundary::Adjacent {
            let index = build.placed[placed].index;
            let name = build.model()?.surfaces[index].name.clone();
            build.notes.push(format!("{name}: no surface of its neighbour faces it, so it is written adiabatic and the heat through it is lost"));
        }
    }
    Ok(())
}
//#endregion 🔖️Partners

//#region 🔖️Loads
fn loads(build: &mut Build, snapshot: &ModelSnapshot, inferred: &ModelInference) -> Result<(), String> {
    for space in build.spaces.clone() {
        let (Some(envelope), Some(row)) = (inferred.energy_envelopes.get(&space), snapshot.spaces.get(&space)) else { continue };
        let zone_id = *build.zone_ids.get(&space).ok_or_else(|| format!("space {space} has no zone"))?;
        let Some(conditions) = snapshot.space_conditions.get(&space) else {
            build.notes.push(format!("{space}: the space states no conditions and floats freely"));
            continue;
        };
        let (profile, understood) = Profile::of(conditions.schedule.as_deref());
        if !understood {
            build.notes.push(format!("{space}: the schedule '{}' is not a window of hours (for example Office 08-18) and the loads run all year", conditions.schedule.clone().unwrap_or_default()));
        }
        let density = conditions.occupancy_density.or_else(|| row.zone.as_ref().and_then(|zone| snapshot.zones.get(zone)).map(|zone| zone.occupancy_density)).filter(|density| *density > 0.0);
        let (schedule, activity) = {
            let Build { next, book, .. } = &mut *build;
            (book.profile(profile, next), book.constant(ACTIVITY_W, next))
        };
        if let Some(per_area) = density {
            let id = build.id();
            build.model()?.people.push(PeopleGain { id, zone_id, schedule_id: schedule, activity_schedule_id: activity, people_per_area: per_area, sensible_fraction: PEOPLE_SENSIBLE, latent_fraction: 1.0 - PEOPLE_SENSIBLE, radiant_fraction: PEOPLE_RADIANT });
        }
        if let Some(watts) = conditions.lighting_power_density.filter(|power| *power > 0.0) {
            let id = build.id();
            build.model()?.lighting.push(LightingGain { id, zone_id, schedule_id: schedule, watts_per_area: watts, radiant_fraction: LIGHTING_RADIANT, visible_fraction: LIGHTING_VISIBLE, return_air_fraction: 0.0 });
        }
        if let Some(watts) = conditions.equipment_power_density.filter(|power| *power > 0.0) {
            let id = build.id();
            build.model()?.equipment.push(EquipmentGain { id, zone_id, schedule_id: schedule, watts_per_area: watts, radiant_fraction: EQUIPMENT_RADIANT, latent_fraction: 0.0 });
        }
        if envelope.conditioned {
            let (heating, cooling) = {
                let Build { next, book, .. } = &mut *build;
                (book.constant(conditions.heating_setpoint.unwrap_or(NO_HEATING_C), next), book.constant(conditions.cooling_setpoint.unwrap_or(NO_COOLING_C), next))
            };
            let (thermostat, system) = (build.id(), build.id());
            let model = build.model()?;
            model.thermostats.push(Thermostat { id: thermostat, zone_id, heating_setpoint_schedule_id: heating, cooling_setpoint_schedule_id: cooling, heating_throttle_range_k: 0.0, cooling_throttle_range_k: 0.0 });
            model.ideal_loads.push(IdealLoadsSystem {
                id: system,
                zone_id,
                max_heating_supply_air_temp_c: SUPPLY_HEATING_C,
                min_cooling_supply_air_temp_c: SUPPLY_COOLING_C,
                max_heating_capacity_w: None,
                max_cooling_capacity_w: None,
                outdoor_air_per_person_m3_s: 0.0,
                outdoor_air_per_area_m3_s_m2: conditions.ventilation_rate.unwrap_or(0.0).max(0.0) / 1000.0,
            });
        } else if conditions.ventilation_rate.is_some_and(|rate| rate > 0.0) {
            build.notes.push(format!("{space}: the ventilation of a space that is neither heated nor cooled is not written"));
        }
    }
    Ok(())
}
//#endregion 🔖️Loads

fn seal(build: &mut Build, _snapshot: &ModelSnapshot, _inferred: &ModelInference) -> Result<(), String> {
    let (materials, constructions) = (std::mem::take(&mut build.library.materials), std::mem::take(&mut build.library.constructions));
    let schedules = std::mem::take(&mut build.book.schedules);
    let model = build.model()?;
    model.materials = materials;
    model.constructions = constructions;
    model.schedules = schedules;
    Ok(())
}

