//! 🌡️ `s.bim.model@1/*` → `s.stdio.xml@1.0/*` as a gbXML 7.03 document: the thermal model of the building for energy simulation, written from the `energy-envelope` inference (`energy_envelopes`) and the snapshot. A site is a `Campus` with its `Location`,
//! a building a `Building` with its `BuildingStorey`s and `Space`s (area, volume, people, lighting, equipment and outdoor air per area from the authored conditions), the thermal zones of the model (or one zone per space) carry the heating and cooling set
//! points, every surface of the envelope is a `Surface` (type by the mapping of [`surface_type`], `AdjacentSpaceId` once for an outer surface and twice for a partition shared by two spaces, `RectangularGeometry`, `PlanarGeometry`, `CADObjectId` the id
//! of the envelope surface) holding its windows and doors as `Opening`s, constructions come with their `Layer`s and `Material`s and windows with their `WindowType`.
//! The polygons are turned so +Y points at true north: `Azimuth` is the compass bearing of the outward normal whatever `CADModelAzimuth` a reader applies (it is written as 0). Lengths are metres, areas square metres, temperatures degrees Celsius.
//! 🔖 `IoFidelity::Lossy`: gbXML describes the thermal envelope, not the building (no structure, no profiles, no annotations); only conditioned-model data of spaces whose envelope is inferred is written.
//! The export runs in stages ([`STAGES`]) so a job can report progress and stop between two of them.
//! 📎 https://www.gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html, https://www.iso.org/standard/65708.html

use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::holders::{holder_of, layers_of, Holder};
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::{Boundary, EnvelopeSurface, SurfaceKind};
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::EnvelopeSpace;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::{ModelInference, model_graph::{instance, kinds}};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::OpeningKind;
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gbxml::codec::{attr, number};
use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::gbxml::geometry::{area as polygon_area, offset_in, round9, turned, Frame, PolygonProjection, Rect, P3};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_os_kernel::io::io_mechanism::{ArchiveChildren, IoNativeDirection, IoRunControl, Serializer};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::session::ModelInferenceSession;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use std::collections::{BTreeMap, BTreeSet};

#[path = "📊️tables/🦀️.rs"]
pub mod tables;

#[path = "🧵️xml/🦀️.rs"]
mod xml;
pub use xml::{GbxmlAttribute, GbxmlContent, GbxmlElement, GbxmlText, GbxmlTextCursor, GbxmlTextPart, GbxmlTreeSource};

#[path = "🧵️xml/🌐️view/🦀️.rs"]
mod markup;
use markup::{Markup, element, leaf, quantity};

#[path = "📤️pages/🦀️.rs"]
mod pages;
pub use pages::GbxmlPages;

#[path = "🎟️surfaces/🦀️.rs"]
mod original_surfaces;
pub use original_surfaces::{OriginalSurfaceRow, OriginalSurfaces, SurfaceAddress};

#[path = "🎟️zones/🦀️.rs"]
mod original_zones;
pub use original_zones::{OriginalZone, OriginalZones};

#[path = "🎟️names/🦀️.rs"]
mod original_names;
pub use original_names::{GbxmlNameSource, OriginalNames};

#[path = "🎟️constructions/🦀️.rs"]
mod original_constructions;
pub use original_constructions::OriginalConstructions;

#[path = "🎟️materials/🦀️.rs"]
mod original_materials;
pub use original_materials::{MaterialSource, OriginalMaterials};

#[path = "🎟️openings/🦀️.rs"]
mod original_openings;
pub use original_openings::{OpeningRange, OriginalOpeningRow, OriginalOpenings};

#[path = "🎟️document/🦀️.rs"]
mod original_document;
pub use original_document::OriginalGbxml;

/// 🪪️ The XML 1.0 dialect the gbXML document is written in.
pub const XML_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId::ANY };
/// 🔖️ The namespace of the gbXML schema.
pub const NAMESPACE: &str = "http://www.gbxml.org/schema";
/// 🔖️ The version of the gbXML schema the document follows.
pub const VERSION: &str = "7.03";
const NO_ENVELOPE: &str = "no space states thermal conditions, so there is no envelope to write";

//#region 🔖️Plan
/// 🔑️ The `xs:ID`s of the document: a prefix and the model id with every character outside `[A-Za-z0-9_.-]` turned into `_`, made unique with a numeric suffix.
#[derive(Clone, Debug, Default)]
pub struct Names {
    taken: BTreeSet<String>,
    suffixes: BTreeMap<String, usize>,
}

impl Names {
    /// 🔑️ A fresh id for `raw` under `prefix`.
    pub fn id(&mut self, prefix: &str, raw: &str) -> String {
        let tail: String = raw.chars().map(|letter| if letter.is_ascii_alphanumeric() || matches!(letter, '_' | '.' | '-') { letter } else { '_' }).collect();
        let base = format!("{prefix}-{tail}");
        if self.taken.insert(base.clone()) {
            return base;
        }
        let count = self.suffixes.entry(base.clone()).or_insert(1);
        loop {
            *count += 1;
            let candidate = format!("{base}-{count}");
            if self.taken.insert(candidate.clone()) {
                return candidate;
            }
        }
    }
}

/// 🧱️ A material of a given thickness.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialRow {
    pub xml: String,
    pub name: String,
    pub thickness: f64,
    pub conductivity: f64,
    pub density: f64,
    pub specific_heat: f64,
}

/// 🧱️ A construction: its U-value and the layers from the first to the last of the stack.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionRow {
    pub xml: String,
    pub name: String,
    pub u_value: Option<f64>,
    pub layers: Vec<String>,
}

/// 🪟️ A window type.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowTypeRow {
    pub xml: String,
    pub name: String,
    pub description: String,
    pub u_value: Option<f64>,
    pub g_value: Option<f64>,
}

/// 🌡️ A thermal zone: the model zone (or one space) with the set points of its spaces.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneRow {
    pub xml: String,
    pub cad: String,
    pub name: String,
    pub heating: Option<f64>,
    pub cooling: Option<f64>,
    pub outdoor_air: Option<f64>,
}

/// 🏠️ A space.
#[derive(Clone, Debug, PartialEq)]
pub struct SpaceRow {
    pub cad: String,
    pub xml: String,
    pub name: String,
    pub description: String,
    pub storey: String,
    pub building: String,
    pub zone: String,
    pub condition: &'static str,
    pub area: f64,
    pub volume: f64,
    pub people: Option<f64>,
    pub lighting: Option<f64>,
    pub equipment: Option<f64>,
    pub outdoor_air: Option<f64>,
}

/// 🪟️ A window or door in a surface.
#[derive(Clone, Debug, PartialEq)]
pub struct OpeningRow {
    pub xml: String,
    pub cad: String,
    pub kind: &'static str,
    pub window_type: Option<String>,
    pub construction: Option<String>,
    pub u_value: Option<f64>,
    pub offset: [f64; 2],
    pub rect: Rect,
    pub polygon: Vec<P3>,
    pub area: f64,
}

/// 🧱️ A surface of the envelope: one side of a partition or two.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceRow {
    pub xml: String,
    pub cad: Vec<String>,
    pub site: String,
    pub kind: &'static str,
    pub exposed: bool,
    pub construction: String,
    pub u_value: Option<f64>,
    pub spaces: Vec<String>,
    pub azimuth: f64,
    pub tilt: f64,
    pub rect: Rect,
    pub polygon: Vec<P3>,
    pub area: f64,
    pub openings: Vec<OpeningRow>,
}

/// 🗂️ Everything the stages gather before the document is built.
#[derive(Clone, Debug, Default)]
pub struct Plan {
    pub names: Names,
    pub materials: BTreeMap<(String, i64), MaterialRow>,
    pub layers: BTreeMap<String, String>,
    pub constructions: Vec<ConstructionRow>,
    pub construction_of: BTreeMap<String, String>,
    pub window_types: BTreeMap<String, WindowTypeRow>,
    pub door_constructions: BTreeMap<String, ConstructionRow>,
    pub zones: Vec<ZoneRow>,
    pub spaces: Vec<SpaceRow>,
    pub surfaces: Vec<SurfaceRow>,
    pub space_boundaries: BTreeMap<String, Vec<(String, Vec<P3>)>>,
    pub zone_of: BTreeMap<String, String>,
    pub storeys: BTreeMap<String, String>,
    pub buildings: BTreeMap<String, String>,
    pub sites: BTreeMap<String, String>,
    pub notes: Vec<String>,
}
//#endregion 🔖️Plan

//#region 🔖️Mapping
/// 🧱️ The gbXML `surfaceType` of a surface: walls are exterior, underground or interior by what lies behind them (a partition between two spaces, adiabatic or not, is interior), a floor is slab on grade on the ground, exposed over the outdoors and
/// interior otherwise, a ceiling is a roof under the outdoors and a ceiling otherwise.
pub fn surface_type(surface: &EnvelopeSurface) -> &'static str {
    match (surface.kind, surface.boundary) {
        (SurfaceKind::Wall | SurfaceKind::CurtainWall | SurfaceKind::Window | SurfaceKind::Door, Boundary::Exterior) => "ExteriorWall",
        (SurfaceKind::Wall | SurfaceKind::CurtainWall | SurfaceKind::Window | SurfaceKind::Door, Boundary::Ground) => "UndergroundWall",
        (SurfaceKind::Wall | SurfaceKind::CurtainWall | SurfaceKind::Window | SurfaceKind::Door, _) => "InteriorWall",
        (SurfaceKind::Floor, Boundary::Ground) => "SlabOnGrade",
        (SurfaceKind::Floor, Boundary::Exterior) => "ExposedFloor",
        (SurfaceKind::Floor, _) => "InteriorFloor",
        (SurfaceKind::Ceiling, Boundary::Exterior) => "Roof",
        (SurfaceKind::Ceiling, _) => "Ceiling",
    }
}

/// ☀️ Whether a surface of this type sees the sun.
pub fn exposed_to_sun(kind: &str) -> bool {
    matches!(kind, "ExteriorWall" | "Roof" | "ExposedFloor")
}

fn opening_type(kind: SurfaceKind) -> &'static str {
    if kind == SurfaceKind::Window {
        "FixedWindow"
    } else {
        "NonSlidingDoor"
    }
}

fn condition_type(heating: Option<f64>, cooling: Option<f64>) -> &'static str {
    match (heating.is_some(), cooling.is_some()) {
        (true, true) => "HeatedAndCooled",
        (true, false) => "Heated",
        (false, true) => "Cooled",
        (false, false) => "Unconditioned",
    }
}

fn bearing(model: &ModelSnapshot, storey: &str) -> f64 {
    let building = model.storeys.get(storey).and_then(|storey| model.buildings.get(&storey.building));
    building.map_or(0.0, |building| model.sites.get(&building.site).map_or(0.0, |site| site.true_north) - building.rotation)
}
//#endregion 🔖️Mapping

//#region 🔖️Stages
/// 🏗️ The export under construction: the model, its inference and the plan.
pub struct Build<'a> {
    pub model: &'a ModelSnapshot,
    pub inferred: &'a ModelInference,
    pub plan: Plan,
}

/// 🪜️ One stage: the name of what it gathers and the function that gathers it.
pub type Stage = (&'static str, fn(&mut Build<'_>));

/// 🪜️ The stages in the order they run; each reads the plan the stages before it left.
pub const STAGES: &[Stage] = &[("zones", zones), ("spaces", spaces), ("constructions", constructions), ("windows", windows), ("surfaces", surfaces)];

pub fn zones(b: &mut Build<'_>) {
    let model = b.model;
    let mut groups: BTreeMap<(String, [Option<u64>; 3]), Vec<&str>> = BTreeMap::new();
    for space in b.inferred.energy_envelopes.keys() {
        let conditions = model.space_conditions.get(space);
        let key = |value: Option<f64>| value.map(f64::to_bits);
        let authored = model.spaces.get(space).and_then(|row| row.zone.clone()).filter(|zone| model.zones.contains_key(zone));
        let owner = authored.unwrap_or_else(|| format!(":{space}"));
        let settings = conditions.map_or([None; 3], |row| [row.heating_setpoint, row.cooling_setpoint, row.ventilation_rate].map(key));
        groups.entry((owner, settings)).or_default().push(space.as_str());
    }
    let mut variants: BTreeMap<&str, usize> = BTreeMap::new();
    for (owner, _) in groups.keys() {
        *variants.entry(owner.as_str()).or_default() += 1;
    }
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    let mut rows = Vec::new();
    for ((owner, [heating, cooling, outdoor_air]), members) in &groups {
        let index = seen.entry(owner.as_str()).or_default();
        *index += 1;
        let base = match model.zones.get(owner) {
            Some(zone) => zone.name.clone(),
            None => model.spaces.get(members[0]).map_or_else(|| members[0].to_string(), |space| format!("{} {}", space.number, space.name).trim().to_string()),
        };
        let name = if variants[owner.as_str()] > 1 && model.zones.contains_key(owner) { format!("{base} ({index})") } else { base };
        let cad = if variants[owner.as_str()] > 1 && model.zones.contains_key(owner) { format!("{owner}#{index}") } else { owner.trim_start_matches(':').to_string() };
        let xml = b.plan.names.id("zone", &cad);
        rows.push((members.clone(), ZoneRow { xml, cad, name, heating: heating.map(f64::from_bits), cooling: cooling.map(f64::from_bits), outdoor_air: outdoor_air.map(f64::from_bits) }));
    }
    for (members, row) in rows {
        for member in members {
            b.plan.zone_of.insert(member.to_string(), row.xml.clone());
        }
        b.plan.zones.push(row);
    }
}

pub fn spaces(b: &mut Build<'_>) {
    let (model, inferred) = (b.model, b.inferred);
    for id in model.sites.keys() {
        let xml = b.plan.names.id("campus", id);
        b.plan.sites.insert(id.clone(), xml);
    }
    for id in model.buildings.keys() {
        let xml = b.plan.names.id("building", id);
        b.plan.buildings.insert(id.clone(), xml);
    }
    for id in model.storeys.keys() {
        let xml = b.plan.names.id("storey", id);
        b.plan.storeys.insert(id.clone(), xml);
    }
    for (id, envelope) in &inferred.energy_envelopes {
        let (Some(space), Some(storey)) = (model.spaces.get(id), model.storeys.get(&envelope.storey)) else {
            b.plan.notes.push(format!("space {id}: its storey is missing"));
            continue;
        };
        let conditions = model.space_conditions.get(id);
        let (heating, cooling) = (conditions.and_then(|row| row.heating_setpoint), conditions.and_then(|row| row.cooling_setpoint));
        let zone_density = space.zone.as_ref().and_then(|zone| model.zones.get(zone)).map(|zone| zone.occupancy_density).filter(|density| *density > 0.0);
        let density = conditions.and_then(|row| row.occupancy_density).or(zone_density);
        let mut description = Vec::new();
        if let Some(occupancy) = conditions.and_then(|row| row.occupancy.as_ref()) {
            description.push(format!("occupancy: {occupancy}"));
        }
        if let Some(schedule) = conditions.and_then(|row| row.schedule.as_ref()) {
            description.push(format!("schedule: {schedule}"));
        }
        let xml = b.plan.names.id("space", id);
        b.plan.spaces.push(SpaceRow {
            cad: id.clone(),
            xml,
            name: format!("{} {}", space.number, space.name).trim().to_string(),
            description: description.join("; "),
            storey: envelope.storey.clone(),
            building: storey.building.clone(),
            zone: b.plan.zone_of.get(id).cloned().unwrap_or_default(),
            condition: condition_type(heating, cooling),
            area: round9(envelope.floor_area),
            volume: round9(envelope.volume),
            people: density.map(|density| round9(density * envelope.floor_area)),
            lighting: conditions.and_then(|row| row.lighting_power_density),
            equipment: conditions.and_then(|row| row.equipment_power_density),
            outdoor_air: conditions.and_then(|row| row.ventilation_rate),
        });
    }
}

pub fn material(plan: &mut Plan, model: &ModelSnapshot, id: &str, thickness: f64) -> Option<String> {
    let key = (id.to_string(), (thickness * 1e6).round() as i64);
    if let Some(row) = plan.materials.get(&key) {
        return Some(row.xml.clone());
    }
    let row = model.materials.get(id)?;
    let xml = plan.names.id("mat", &format!("{id}-{}", key.1));
    plan.materials.insert(key, MaterialRow { xml: xml.clone(), name: format!("{} {} mm", row.name, number(round9(thickness * 1000.0))), thickness, conductivity: row.conductivity, density: row.density, specific_heat: row.specific_heat });
    Some(xml)
}

fn layer(plan: &mut Plan, material_xml: &str) -> String {
    if let Some(layer) = plan.layers.get(material_xml) {
        return layer.clone();
    }
    let xml = plan.names.id("layer", material_xml);
    plan.layers.insert(material_xml.to_string(), xml.clone());
    xml
}

pub fn constructions(b: &mut Build<'_>) {
    let (model, inferred) = (b.model, b.inferred);
    let mut by_signature: BTreeMap<String, String> = BTreeMap::new();
    for (space, envelope) in &inferred.energy_envelopes {
        for surface in envelope.surfaces.iter().filter(|surface| !matches!(surface.kind, SurfaceKind::Window | SurfaceKind::Door | SurfaceKind::CurtainWall)) {
            let Some(u_value) = surface.u_value else { continue };
            let holder = holder_of(model, surface);
            let located = holder.and_then(|holder| layers_of(model, surface, holder));
            let signature = format!("{}|{:016x}", located.map_or("-", |(type_id, _)| type_id), u_value.to_bits());
            let signature = format!("{}|{}", holder.map_or("-", |holder| match holder {
                Holder::Wall(_) => "wall",
                Holder::CurtainWall(_) => "curtain-wall",
                Holder::Slab(_) => "slab",
                Holder::Roof(_) => "roof",
                Holder::Ceiling(_) => "ceiling",
                Holder::Window(_) | Holder::Door(_) => "opening",
            }), signature);
            let xml = match by_signature.get(&signature) {
                Some(xml) => xml.clone(),
                None => {
                    let mut layers: Vec<String> = Vec::new();
                    for row in located.map(|(_, layers)| layers).unwrap_or_default() {
                        if let Some(material_xml) = material(&mut b.plan, model, &row.material, row.thickness) {
                            layers.push(layer(&mut b.plan, &material_xml));
                        }
                    }
                    let label = match (holder, located) {
                        (Some(Holder::Wall(_)), Some((id, _))) => model.wall_types.get(id).map(|kind| kind.name.clone()),
                        (Some(Holder::Slab(_)), Some((id, _))) => model.slab_types.get(id).map(|kind| kind.name.clone()),
                        (Some(Holder::Roof(_)), Some((id, _))) => model.roof_types.get(id).map(|kind| kind.name.clone()),
                        (Some(Holder::Ceiling(_)), Some((id, _))) => model.ceiling_types.get(id).map(|kind| kind.name.clone()),
                        _ => None,
                    }
                    .unwrap_or_else(|| format!("{} surface", surface.kind.name()));
                    let rounded = round9(u_value);
                    let xml = b.plan.names.id("construction", &format!("{}-{}", located.map_or("none", |(id, _)| id), number(rounded)));
                    b.plan.constructions.push(ConstructionRow { xml: xml.clone(), name: format!("{label} U={}", number(rounded)), u_value: Some(rounded), layers });
                    by_signature.insert(signature, xml.clone());
                    xml
                }
            };
            b.plan.construction_of.insert(format!("{space}|{}", surface.id), xml);
        }
    }
}

fn windows(b: &mut Build<'_>) {
    let (model, inferred) = (b.model, b.inferred);
    let used: BTreeSet<&str> = inferred.energy_envelopes.values().flat_map(|envelope| envelope.surfaces.iter()).filter(|surface| matches!(surface.kind, SurfaceKind::Window | SurfaceKind::Door)).map(|surface| surface.element.as_str()).collect();
    let facades: BTreeSet<&str> = inferred.energy_envelopes.values().flat_map(|envelope| envelope.surfaces.iter()).filter(|surface| surface.kind == SurfaceKind::CurtainWall).map(|surface| surface.construction.as_str()).collect();
    for type_id in facades {
        let key = format!("curtain:{type_id}");
        let Some(row) = model.curtain_wall_types.get(type_id).filter(|_| !b.plan.window_types.contains_key(&key)) else { continue };
        let xml = b.plan.names.id("windowtype", &key);
        let description = row.frame_fraction.map(|fraction| format!("curtain wall, frame fraction {}", number(round9(fraction)))).unwrap_or_else(|| "curtain wall".to_string());
        b.plan.window_types.insert(key, WindowTypeRow { xml, name: row.name.clone(), description, u_value: row.u_value.map(round9), g_value: row.g_value.map(round9) });
    }
    for element in used {
        match model.openings.get(element).map(|opening| &opening.kind) {
            Some(OpeningKind::Window { window_type }) if !b.plan.window_types.contains_key(window_type) => {
                let Some(row) = model.window_types.get(window_type) else { continue };
                let xml = b.plan.names.id("windowtype", window_type);
                let description = row.frame_fraction.map(|fraction| format!("frame fraction {}", number(round9(fraction)))).unwrap_or_default();
                b.plan.window_types.insert(window_type.clone(), WindowTypeRow { xml, name: row.name.clone(), description, u_value: row.u_value.map(round9), g_value: row.g_value.map(round9) });
            }
            Some(OpeningKind::Door { door_type }) if !b.plan.door_constructions.contains_key(door_type) => {
                let Some(u_value) = model.door_types.get(door_type).and_then(|row| row.u_value) else { continue };
                let xml = b.plan.names.id("construction", &format!("door-{door_type}"));
                let name = model.door_types.get(door_type).map_or_else(|| door_type.clone(), |row| row.name.clone());
                b.plan.door_constructions.insert(door_type.clone(), ConstructionRow { xml, name: format!("{name} U={}", number(round9(u_value))), u_value: Some(round9(u_value)), layers: Vec::new() });
            }
            _ => {}
        }
    }
}

fn polygon_of(surface: &EnvelopeSurface, bearing: f64) -> Vec<P3> {
    surface.polygon.iter().map(|point| turned(point, bearing)).collect()
}

fn opening_row(b: &Build<'_>, names: &mut Names, space: &str, child: &EnvelopeSurface, bearing: f64, host: &Frame, corner: P3) -> Option<OpeningRow> {
    let polygon = polygon_of(child, bearing);
    if polygon.len() < 3 {
        return None;
    }
    let (offset, rect) = offset_in(host, corner, &polygon);
    let (window_type, construction) = match b.model.openings.get(&child.element).map(|opening| &opening.kind) {
        Some(OpeningKind::Window { window_type }) => (b.plan.window_types.get(window_type).map(|row| row.xml.clone()), None),
        Some(OpeningKind::Door { door_type }) => (None, b.plan.door_constructions.get(door_type).map(|row| row.xml.clone())),
        _ => (None, None),
    };
    let _ = space;
    Some(OpeningRow {
        xml: names.id("opening", &child.element),
        cad: child.element.clone(),
        kind: opening_type(child.kind),
        window_type,
        construction,
        u_value: child.u_value.map(round9),
        offset,
        rect,
        area: round9(polygon_area(&polygon)),
        polygon,
    })
}

pub fn surfaces(b: &mut Build<'_>) {
    surfaces_checked(b, &mut || Ok::<_, std::convert::Infallible>(()))
        .unwrap_or_else(|never| match never {});
}

fn surfaces_checked<E>(b: &mut Build<'_>, checkpoint: &mut impl FnMut() -> Result<(), E>) -> Result<(), E> {
    let inferred = b.inferred;
    let storey_of: BTreeMap<&str, &str> = inferred.energy_envelopes.iter().map(|(space, envelope)| (space.as_str(), envelope.storey.as_str())).collect();
    let space_xml: BTreeMap<String, String> = b.plan.spaces.iter().map(|row| (row.cad.clone(), row.xml.clone())).collect();
    let mut names = std::mem::take(&mut b.plan.names);
    let mut rows = Vec::new();
    let mut unspecified = None;
    for merged in semio_s_artifact_bim_model_components::standards::v1::subsets::any::io::export::gbxml::pairing::merge_checked(&inferred.energy_envelopes, checkpoint)? {
        checkpoint()?;
        let storey = storey_of.get(merged.space).copied().unwrap_or_default();
        let turn = bearing(b.model, storey);
        let polygon = polygon_of(merged.surface, turn);
        let mut geometry = PolygonProjection::new(polygon.len());
        let projection = loop {
            checkpoint()?;
            if let Some(result) = geometry.advance(64, |index| polygon[index]) { break result; }
        };
        let Some(projection) = projection else {
            b.plan.notes.push(format!("surface {}: its polygon is degenerate", merged.surface.id));
            continue;
        };
        let (frame, rect) = (projection.frame, projection.rect);
        let kind = surface_type(merged.surface);
        let site = b.model.storeys.get(storey).and_then(|row| b.model.buildings.get(&row.building)).map(|row| row.site.clone()).unwrap_or_default();
        let mut cad = vec![merged.surface.id.clone()];
        cad.extend(merged.other.map(|(_, other)| other.id.clone()));
        let key = format!("{}|{}", merged.space, merged.surface.id);
        let mut openings: Vec<OpeningRow> = merged.children.iter().filter_map(|(space, child)| opening_row(b, &mut names, space, child, turn, &frame, rect.corner)).collect();
        if merged.surface.kind == SurfaceKind::CurtainWall {
            if let Some(row) = b.plan.window_types.get(&format!("curtain:{}", merged.surface.construction)) {
                let cad = format!("{}/glass", merged.surface.id);
                openings.push(OpeningRow { xml: names.id("opening", &cad), cad, kind: "FixedWindow", window_type: Some(row.xml.clone()), construction: None, u_value: merged.surface.u_value.map(round9), offset: [0.0, 0.0], rect, polygon: polygon.clone(), area: round9(polygon_area(&polygon)) });
            }
        }
        let spaces: Vec<String> = merged.spaces().iter().filter_map(|space| space_xml.get(*space).cloned()).collect();
        let construction = b.plan.construction_of.get(&key).cloned().unwrap_or_else(|| {
            unspecified.get_or_insert_with(|| {
                let xml = names.id("construction", "unspecified");
                b.plan.constructions.push(ConstructionRow { xml: xml.clone(), name: "Unspecified construction".to_owned(), u_value: None, layers: Vec::new() });
                xml
            }).clone()
        });
        let xml = names.id("surface", &merged.surface.id);
        b.plan.space_boundaries.entry(merged.space.to_owned()).or_default().push((xml.clone(), polygon.clone()));
        if let Some((space, other)) = merged.other {
            b.plan.space_boundaries.entry(space.to_owned()).or_default().push((xml.clone(), polygon_of(other, turn)));
        }
        rows.push(SurfaceRow {
            xml,
            cad,
            site,
            kind,
            exposed: exposed_to_sun(kind),
            construction,
            u_value: merged.surface.u_value.map(round9),
            spaces,
            azimuth: frame.azimuth(),
            tilt: frame.tilt(),
            rect,
            area: round9(projection.area),
            polygon,
            openings,
        });
    }
    b.plan.names = names;
    b.plan.surfaces = rows;
    Ok(())
}
//#endregion 🔖️Stages

//#region 🔖️Document
fn point<'a>(coordinates: &[f64]) -> Markup<'a> {
    element("CartesianPoint", Vec::new(), coordinates.iter().map(|value| leaf("Coordinate", Vec::new(), &number(*value))).collect())
}

fn poly_loop(polygon: &[P3]) -> Markup<'_> {
    element("PlanarGeometry", Vec::new(), vec![Markup::PolyLoop(polygon)])
}

fn rectangular<'a>(azimuth: f64, tilt: f64, corner: &[f64], rect: &Rect) -> Markup<'a> {
    element("RectangularGeometry", Vec::new(), vec![leaf("Azimuth", Vec::new(), &number(azimuth)), point(corner), leaf("Tilt", Vec::new(), &number(tilt)), leaf("Height", Vec::new(), &number(rect.height)), leaf("Width", Vec::new(), &number(rect.width))])
}

fn opening_node(row: &OpeningRow, azimuth: f64, tilt: f64) -> Markup<'_> {
    let mut attrs = vec![attr("id", &row.xml), attr("openingType", row.kind)];
    attrs.extend(row.window_type.iter().map(|id| attr("windowTypeIdRef", id)));
    attrs.extend(row.construction.iter().map(|id| attr("constructionIdRef", id)));
    element("Opening", attrs, vec![leaf("Name", Vec::new(), &row.cad), rectangular(azimuth, tilt, &row.offset, &row.rect), poly_loop(&row.polygon), leaf("CADObjectId", Vec::new(), &row.cad)])
}

fn surface_node(row: &SurfaceRow) -> Markup<'_> {
    let attrs = vec![attr("id", &row.xml), attr("surfaceType", row.kind), attr("constructionIdRef", &row.construction), attr("exposedToSun", row.exposed)];
    let mut children = vec![leaf("Name", Vec::new(), &row.cad[0])];
    children.extend(row.spaces.iter().map(|space| element("AdjacentSpaceId", vec![attr("spaceIdRef", space)], Vec::new())));
    children.push(rectangular(row.azimuth, row.tilt, &row.rect.corner, &row.rect));
    children.push(poly_loop(&row.polygon));
    children.extend(row.openings.iter().map(|opening| opening_node(opening, row.azimuth, row.tilt)));
    children.extend(row.cad.iter().map(|cad| leaf("CADObjectId", Vec::new(), cad)));
    element("Surface", attrs, children)
}

fn space_node<'a, E>(row: &SpaceRow, plan: &'a Plan, checkpoint: &mut impl FnMut() -> Result<(), E>) -> Result<Markup<'a>, E> {
    let mut attrs = vec![attr("id", &row.xml)];
    if !row.zone.is_empty() {
        attrs.push(attr("zoneIdRef", &row.zone));
    }
    attrs.push(attr("conditionType", row.condition));
    attrs.push(attr("buildingStoreyIdRef", plan.storeys.get(&row.storey).map_or("", String::as_str)));
    let mut children = vec![leaf("Name", Vec::new(), &row.name)];
    if !row.description.is_empty() {
        children.push(leaf("Description", Vec::new(), &row.description));
    }
    children.push(quantity("Area", None, row.area));
    children.push(quantity("Volume", None, row.volume));
    children.extend(row.people.map(|value| quantity("PeopleNumber", Some("NumberOfPeople"), value)));
    children.extend(row.lighting.map(|value| quantity("LightPowerPerArea", Some("WattPerSquareMeter"), value)));
    children.extend(row.equipment.map(|value| quantity("EquipPowerPerArea", Some("WattPerSquareMeter"), value)));
    children.push(leaf("CADObjectId", Vec::new(), &row.cad));
    if let Some(boundaries) = plan.space_boundaries.get(&row.cad) {
        for (surface, polygon) in boundaries {
            checkpoint()?;
            children.push(element("SpaceBoundary", vec![attr("surfaceIdRef", surface)], vec![poly_loop(polygon)]));
        }
    }
    Ok(element("Space", attrs, children))
}

fn construction_node<'a>(row: &ConstructionRow) -> Markup<'a> {
    let mut attrs = vec![attr("id", &row.xml)];
    let mut children = vec![leaf("Name", Vec::new(), &row.name)];
    if let Some(value) = row.u_value {
        children.push(quantity("U-value", Some("WPerSquareMeterK"), value));
    } else {
        attrs.push(attr("constructionIsSchematic", true));
    }
    children.extend(row.layers.iter().map(|layer| element("LayerId", vec![attr("layerIdRef", layer)], Vec::new())));
    element("Construction", attrs, children)
}

fn campus_node<'a, E>(plan: &'a Plan, model: &ModelSnapshot, inferred: &ModelInference, site_id: &str, checkpoint: &mut impl FnMut() -> Result<(), E>) -> Result<Option<Markup<'a>>, E> {
    let (Some(site), Some(xml)) = (model.sites.get(site_id), plan.sites.get(site_id)) else { return Ok(None) };
    let location = element(
        "Location",
        Vec::new(),
        vec![leaf("ZipcodeOrPostalCode", Vec::new(), ""), quantity("Longitude", None, site.longitude), quantity("Latitude", None, site.latitude), quantity("Elevation", None, site.elevation), quantity("CADModelAzimuth", None, 0.0), leaf("Name", Vec::new(), &site.name)],
    );
    let mut children = vec![location];
    for (building_id, building) in model.buildings.iter().filter(|(_, building)| building.site == site_id) {
        checkpoint()?;
        let Some(building_xml) = plan.buildings.get(building_id) else { continue };
        let spaces: Vec<&SpaceRow> = plan.spaces.iter().filter(|row| row.building == *building_id).collect();
        let mut inner = vec![leaf("Name", Vec::new(), &building.name), quantity("Area", None, round9(spaces.iter().map(|row| row.area).sum()))];
        let mut storeys: Vec<(&String, &semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Storey)> = model.storeys.iter().filter(|(_, storey)| storey.building == *building_id).collect();
        storeys.sort_by(|a, b| a.1.level.cmp(&b.1.level).then_with(|| a.0.cmp(b.0)));
        for (storey_id, storey) in storeys {
            let level = inferred.storey_levels.get(storey_id).map_or(0.0, |level| level.elevation);
            inner.push(element("BuildingStorey", vec![attr("id", plan.storeys.get(storey_id).map_or("", String::as_str))], vec![leaf("Name", Vec::new(), &storey.name), quantity("Level", None, round9(level))]));
        }
        for row in spaces {
            checkpoint()?;
            inner.push(space_node(row, plan, checkpoint)?);
        }
        children.push(element("Building", vec![attr("id", building_xml), attr("buildingType", "Unknown")], inner));
    }
    for row in plan.surfaces.iter().filter(|row| row.site == site_id) {
        checkpoint()?;
        children.push(surface_node(row));
    }
    Ok(Some(element("Campus", vec![attr("id", xml)], std::iter::once(leaf("Name", Vec::new(), &site.name)).chain(children).collect())))
}

/// 📄️ The gbXML tree of a finished plan.
pub fn document(plan: &Plan, model: &ModelSnapshot, inferred: &ModelInference) -> XmlNode {
    document_checked(plan, model, inferred, &mut || Ok::<_, std::convert::Infallible>(()))
        .unwrap_or_else(|never| match never {})
        .into_node()
}

fn document_checked<'a, E>(plan: &'a Plan, model: &ModelSnapshot, inferred: &ModelInference, checkpoint: &mut impl FnMut() -> Result<(), E>) -> Result<Markup<'a>, E> {
    let mut children = Vec::new();
    for site in model.sites.keys() {
        checkpoint()?;
        children.extend(campus_node(plan, model, inferred, site, checkpoint)?);
    }
    for row in plan.window_types.values() {
        checkpoint()?;
        let mut inner = vec![leaf("Name", Vec::new(), &row.name)];
        if !row.description.is_empty() {
            inner.push(leaf("Description", Vec::new(), &row.description));
        }
        inner.extend(row.u_value.map(|value| quantity("U-value", Some("WPerSquareMeterK"), value)));
        inner.extend(row.g_value.map(|value| leaf("SolarHeatGainCoeff", vec![attr("unit", "Fraction"), attr("solarIncidentAngle", "0")], &number(value))));
        children.push(element("WindowType", vec![attr("id", &row.xml)], inner));
    }
    for row in plan.constructions.iter().chain(plan.door_constructions.values()) {
        checkpoint()?;
        children.push(construction_node(row));
    }
    let by_layer: BTreeMap<&String, &str> = plan.materials.values().filter_map(|row| plan.layers.get(&row.xml).map(|layer| (layer, row.xml.as_str()))).collect();
    for (layer, material) in by_layer {
        checkpoint()?;
        children.push(element("Layer", vec![attr("id", layer)], vec![element("MaterialId", vec![attr("materialIdRef", material)], Vec::new())]));
    }
    for row in plan.materials.values().filter(|row| plan.layers.contains_key(&row.xml)) {
        checkpoint()?;
        children.push(element(
            "Material",
            vec![attr("id", &row.xml)],
            vec![
                leaf("Name", Vec::new(), &row.name),
                quantity("R-value", Some("SquareMeterKPerW"), round9(row.thickness / row.conductivity)),
                quantity("Thickness", Some("Meters"), round9(row.thickness)),
                quantity("Conductivity", Some("WPerMeterK"), row.conductivity),
                quantity("Density", Some("KgPerCubicM"), row.density),
                quantity("SpecificHeat", Some("JPerKgK"), row.specific_heat),
            ],
        ));
    }
    for row in &plan.zones {
        checkpoint()?;
        let mut inner = vec![leaf("Name", Vec::new(), &row.name)];
        inner.extend(row.outdoor_air.map(|value| quantity("OAFlowPerArea", Some("LPerSecPerSquareM"), value)));
        inner.extend(row.heating.map(|value| quantity("DesignHeatT", Some("C"), value)));
        inner.extend(row.cooling.map(|value| quantity("DesignCoolT", Some("C"), value)));
        inner.push(leaf("CADObjectId", Vec::new(), &row.cad));
        children.push(element("Zone", vec![attr("id", &row.xml)], inner));
    }
    Ok(element(
        "gbXML",
        vec![attr("xmlns", NAMESPACE), attr("version", VERSION), attr("useSIUnitsForResults", "true"), attr("temperatureUnit", "C"), attr("lengthUnit", "Meters"), attr("areaUnit", "SquareMeters"), attr("volumeUnit", "CubicMeters")],
        children,
    ))
}
//#endregion 🔖️Document

//#region 🔖️Export
/// 🌱️ A build of `model` that has run no stage.
pub fn begin<'a>(model: &'a ModelSnapshot, inferred: &'a ModelInference) -> Build<'a> {
    Build { model, inferred, plan: Plan::default() }
}

/// 🌡️ The plan of `model` from its inference: every stage run, `Err` when the model states no conditions (the inference has no envelope).
pub fn plan_of(model: &ModelSnapshot, inferred: &ModelInference) -> Result<Plan, String> {
    if inferred.energy_envelopes.is_empty() {
        return Err(NO_ENVELOPE.to_string());
    }
    let mut build = begin(model, inferred);
    for (_, stage) in STAGES {
        stage(&mut build);
    }
    Ok(build.plan)
}

/// 📄️ The gbXML text of `model` from its inference plus a note per item that could not be written.
pub fn inferred_to_gbxml(model: &ModelSnapshot, inferred: &ModelInference) -> Result<(String, Vec<String>), String> {
    let plan = plan_of(model, inferred)?;
    let tree = document_checked(&plan, model, inferred, &mut || Ok::<_, std::convert::Infallible>(()))
        .unwrap_or_else(|never| match never {});
    let text = document_text_checked(&tree, &mut || Ok::<_, std::convert::Infallible>(()))
        .unwrap_or_else(|never| match never {})?;
    drop(tree);
    Ok((text, plan.notes))
}

const GBXML_FIELDS: u64 = kinds::ENERGY | kinds::LEVELS;

/// 📤️ The gbXML text of `model` and the items excluded from the thermal envelope.
pub fn export_gbxml(model: &ModelSnapshot) -> Result<(String, Vec<String>), String> {
    instance::probe::<GBXML_FIELDS, _>(None, model, |inferred| inferred_to_gbxml(model, inferred)).map_err(|error| error.to_string())?
}

/// 📊️ The canonical JSON table of the export of `model` (see [`tables`]): the table the lxml, shapely and numpy oracle measures from the written file.
pub fn export_table(model: &ModelSnapshot) -> Result<String, String> {
    instance::probe::<GBXML_FIELDS, _>(None, model, |inferred| plan_of(model, inferred).map(|plan| tables::table_json(&plan, model))).map_err(|error| error.to_string())?
}

/// 🧵️ An export that runs one [`STAGES`] entry per step, so a job can report how far it is and stop between two stages; it keeps the plan and no borrow of the inference.
pub struct StagedGbxml {
    plan: Option<Plan>,
    next: usize,
}

impl StagedGbxml {
    /// 🌱️ An export that has not run a stage yet.
    pub fn new() -> Self {
        Self { plan: Some(Plan::default()), next: 0 }
    }

    /// 📈️ How far the stages are, from zero to one.
    pub fn fraction(&self) -> f32 {
        self.next as f32 / (STAGES.len() + 1) as f32
    }

    /// 🏷️ The name of the stage the next step runs; `document` for the last step, `None` once done.
    pub fn stage(&self) -> Option<&'static str> {
        STAGES.get(self.next).map(|(name, _)| *name).or((self.next == STAGES.len() && self.plan.is_some()).then_some("document"))
    }

    /// ⏩️ Runs the next stage; the text and its notes after the last, `None` before.
    pub fn step(&mut self, model: &ModelSnapshot, inferred: &ModelInference) -> Option<Result<(String, Vec<String>), String>> {
        self.step_checked(model, inferred, &mut || Ok::<_, std::convert::Infallible>(()))
            .unwrap_or_else(|never| match never {})
    }

    fn step_checked<E>(&mut self, model: &ModelSnapshot, inferred: &ModelInference, checkpoint: &mut impl FnMut() -> Result<(), E>) -> Result<Option<Result<(String, Vec<String>), String>>, E> {
        checkpoint()?;
        let Some(plan) = self.plan.take() else { return Ok(None) };
        if self.next == 0 && inferred.energy_envelopes.is_empty() {
            return Ok(Some(Err(NO_ENVELOPE.to_string())));
        }
        let mut build = Build { model, inferred, plan };
        if let Some((name, stage)) = STAGES.get(self.next) {
            if *name == "surfaces" { surfaces_checked(&mut build, checkpoint)?; } else { stage(&mut build); }
            self.next += 1;
            self.plan = Some(build.plan);
            return Ok(None);
        }
        self.next += 1;
        let tree = document_checked(&build.plan, model, inferred, checkpoint)?;
        let text = document_text_checked(&tree, checkpoint)?;
        drop(tree);
        Ok(Some(text.map(|text| (text, build.plan.notes))))
    }
}

fn document_text_checked<E>(tree: &impl GbxmlTreeSource, checkpoint: &mut impl FnMut() -> Result<(), E>) -> Result<Result<String, String>, E> {
    let mut cursor = GbxmlTextCursor::default();
    let mut bytes = Vec::new();
    let mut page = [0; 4096];
    while !cursor.is_complete() {
        checkpoint()?;
        let written = match cursor.write_source(tree, &mut page, 4096) {
            Ok(written) => written,
            Err(message) => return Ok(Err(message.to_owned())),
        };
        bytes.extend_from_slice(&page[..written]);
    }
    Ok(String::from_utf8(bytes).map_err(|error| error.to_string()))
}

impl Default for StagedGbxml {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️Export

//#region 🔖️Serializer
/// 🌡️ The gbXML serializer of the BIM model.
pub struct ModelIntoGbxml;

impl Serializer<ModelSnapshot> for ModelIntoGbxml {
    const INTO: Dialect = XML_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        macro_rules! original {
            ($native:expr) => {
                $native.scoped_stage(|native| serialize_checked(from, |progress| match progress {
                    Progress::Check => native.checkpoint(),
                    Progress::Begin(total) => native.begin_stage(total),
                    Progress::Complete => native.step(),
                }))
            };
        }
        match control.direction() {
            IoNativeDirection::Decode => original!(control.decode()?),
            IoNativeDirection::Encode => original!(control.encode()?),
        }
    }
}

enum Progress {
    Check,
    Begin(usize),
    Complete,
}

fn serialize_checked(model: &ModelSnapshot, mut progress: impl FnMut(Progress) -> Result<(), ValueError>) -> IoResult<IoPayload> {
    let invalid = |message: String| IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, format!("ModelIntoGbxml: {message}")));
    progress(Progress::Check)?;
    let inferred = ModelInferenceSession::default().probe_checked::<GBXML_FIELDS, _>(model, || progress(Progress::Check))?
        .map_err(|error| invalid(error.to_string()))?;
    let mut export = StagedGbxml::new();
    progress(Progress::Begin(STAGES.len() + 1))?;
    let (text, notes) = loop {
        let result = export.step_checked(model, &inferred, &mut || progress(Progress::Check))?.transpose().map_err(invalid)?;
        progress(Progress::Complete)?;
        if let Some(result) = result {
            break result;
        }
    };
    let diagnostics = notes.into_iter()
        .map(|note| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new("bim.gbxml.export.skipped"), severity: semio_framework_diagnostic::Severity::Warning, span: Default::default(), message: note, expected: None, scope: Default::default() })
        .collect();
    Ok(IoOutcome { value: IoPayload::Text(text), diagnostics })
}
//#endregion 🔖️Serializer


