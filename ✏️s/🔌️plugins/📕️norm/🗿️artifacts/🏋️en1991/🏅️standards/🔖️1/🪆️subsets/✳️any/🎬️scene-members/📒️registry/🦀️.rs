//! 📒️ EN 1991 scene registry: the entity-kind rows, the owner of an entity id, the global entity ids, the scalar owners and the routing of every check to the entity or badge that draws it. A pure function of the snapshot and the report, below the scene kit.

extern crate semio_s_artifact_norm_en1991_checks as checks;
extern crate semio_s_artifact_norm_en1991_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

use scene::projection_kit::{self, base_id};
use scene::*;

/// 🧊️ Whether the entry of `row` declares geometry, so the case stands in the scene.
pub fn declared(row: &model::CaseRow) -> bool {
    projection_kit::design_case::forms::declared::<checks::catalogue::En1991Cases>(row).is_some()
}

/// 🏷️ One entity kind of the EN 1991 outliner, interaction domain and insert palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityKindSpec {
    pub id: &'static str,
    pub collection: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub icon: &'static str,
    pub scene_kind: EntityKind,
}

const fn kind(id: &'static str, collection: &'static str, label_en: &'static str, label_de: &'static str, icon: &'static str, scene_kind: EntityKind) -> EntityKindSpec {
    EntityKindSpec { id, collection, label_en, label_de, icon, scene_kind }
}

/// 🗂️ The single registry of EN 1991 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("floor", "floors", "Floor area", "Geschossfläche", "layers", EntityKind::Surface),
    kind("selfWeightElement", "selfWeightElements", "Self-weight element", "Eigengewichtselement", "box", EntityKind::Solid),
    kind("roof", "roofs", "Roof", "Dach", "home", EntityKind::Surface),
    kind("windFace", "windFaces", "Wind pressure zone", "Winddruckzone", "wind", EntityKind::Load),
    kind("accidentalCase", "accidentalCases", "Accidental case", "Außergewöhnlicher Fall", "zap", EntityKind::Load),
    kind("bridgeLane", "bridgeLanes", "Bridge lane", "Fahrstreifen", "route", EntityKind::Surface),
    kind("constructionStage", "constructionStages", "Construction stage", "Bauzustand", "clock", EntityKind::Load),
    kind("roofSurface", "roofSurfaces", "Roof surface", "Dachfläche", "home", EntityKind::Surface),
    kind("specialVehicle", "specialVehicles", "Special vehicle", "Sonderfahrzeug", "route", EntityKind::Surface),
    kind("windBridge", "windBridges", "Bridge wind case", "Brückenwindfall", "wind", EntityKind::Surface),
    kind("dynamicElement", "dynamicElements", "Dynamic element", "Dynamisches Bauteil", "activity", EntityKind::Solid),
    kind("impactAction", "impactActions", "Impact action", "Anpralllast", "zap", EntityKind::Load),
    kind("internalExplosion", "internalExplosions", "Internal explosion", "Innere Explosion", "flame", EntityKind::Solid),
    kind("thermalSection", "thermalSections", "Thermal section", "Temperaturquerschnitt", "thermometer", EntityKind::Solid),
    kind("case", "cases", "Design case", "Bemessungsfall", "clipboard-check", EntityKind::Solid),
    kind("helicopterLanding", "helicopterLandings", "Helicopter landing", "Hubschrauberlandung", "plane", EntityKind::Load),
    kind("tieRun", "tieRuns", "Tie run", "Zugglied", "link", EntityKind::Load),
    kind("constructionLoad", "constructionLoads", "Construction load", "Bauzustandslast", "hammer", EntityKind::Load),
    kind("fallingLoad", "fallingLoads", "Falling load", "Herabfallende Last", "arrow-down", EntityKind::Load),
    kind("executionImperfection", "executionImperfections", "Execution imperfection", "Ausführungsimperfektion", "ruler", EntityKind::Load),
    kind("derailmentCase", "derailmentCases", "Derailment case", "Entgleisungsfall", "train", EntityKind::Load),
    kind("craneHoist", "craneHoists", "Crane hoist", "Kranhubwerk", "anchor", EntityKind::Load),
    kind("footbridge", "footbridges", "Footbridge", "Fußgängerbrücke", "footprints", EntityKind::Surface),
    kind("railDynamic", "railDynamics", "Rail span", "Eisenbahnfeld", "train-track", EntityKind::Surface),
    kind("loadDispersal", "loadDispersals", "Load dispersal", "Lastausbreitung", "scan", EntityKind::Surface),
    kind("bulkSolid", "bulkSolids", "Bulk solid", "Schüttgut", "wheat", EntityKind::Surface),
    kind("rotatingMachine", "rotatingMachines", "Rotating machine", "Rotierende Maschine", "cog", EntityKind::Surface),
    kind("dustExplosion", "dustExplosions", "Dust explosion", "Staubexplosion", "flame", EntityKind::Load),
    kind("railLoadGroup", "railLoadGroups", "Rail load group", "Eisenbahn-Lastgruppe", "layers", EntityKind::Load),
    kind("railDerailment", "railDerailments", "Rail derailment", "Eisenbahn-Entgleisung", "train", EntityKind::Load),
    kind("fatigueModel", "fatigueModels", "Fatigue load model", "Ermüdungslastmodell", "repeat", EntityKind::Surface),
    kind("supportCollision", "supportCollisions", "Support collision", "Stützenanprall", "truck", EntityKind::Load),
    kind("localFire", "localFires", "Localised fire", "Lokaler Brand", "flame", EntityKind::Solid),
    kind("snowGuard", "snowGuards", "Snow guard", "Schneefanggitter", "shield", EntityKind::Load),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
];

/// 🧭️ What a scene entity id belongs to: a document row, one of the singleton structures of the snapshot, or the wind-zone map.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Owner<'a> {
    Building,
    Bridge,
    Crane,
    Silo,
    Fire,
    Thermal,
    Floor(&'a model::FloorArea),
    SelfWeight(&'a model::SelfWeightElement),
    Roof(usize, &'a model::RoofArea),
    Face(&'a model::WindFace),
    Accidental(&'a model::AccidentalCase),
    Lane(&'a model::BridgeLane),
    Stage(&'a model::ConstructionStage),
    Surface(&'a model::RoofSurface),
    Vehicle(&'a model::SpecialVehicle),
    BridgeWind(&'a model::WindBridge),
    Dynamic(&'a model::DynamicElement),
    Impact(&'a model::ImpactAction),
    Explosion(&'a model::InternalExplosion),
    Section(&'a model::ThermalSection),
    Helicopter(&'a model::HelicopterLanding),
    Tie(&'a model::TieRun),
    ConstructionLoad(&'a model::ConstructionLoad),
    Falling(&'a model::FallingLoad),
    Imperfection(&'a model::ExecutionImperfection),
    Derailment(&'a model::DerailmentCase),
    CraneHoist(&'a model::CraneHoist),
    Footbridge(&'a model::Footbridge),
    RailSpan(&'a model::RailDynamic),
    Dispersal(&'a model::LoadDispersal),
    BulkSolid(&'a model::BulkSolid),
    Machine(&'a model::RotatingMachine),
    FatigueModel(&'a model::FatigueModel),
    Collision(&'a model::SupportCollision),
    LocalFire(&'a model::LocalFire),
    SnowGuard(&'a model::SnowGuard),
    RailDerailment(&'a model::RailDerailment),
    RailGroup(&'a model::RailLoadGroup),
    Dust(&'a model::DustExplosion),
}

pub fn owner_of<'a>(d: &'a model::En1991Snapshot, id: &str) -> Option<Owner<'a>> {
    let base = base_id(id);
    match base {
        "building" | "wind-qp" | "site" => return Some(Owner::Building),
        "bridge" => return Some(Owner::Bridge),
        "crane" => return Some(Owner::Crane),
        "silo" => return Some(Owner::Silo),
        "fire" => return Some(Owner::Fire),
        "thermal" => return Some(Owner::Thermal),
        _ => {}
    }
    if let Some(letter) = base.strip_prefix("wind-zone-") {
        return Some(d.wind_faces.iter().find(|f| f.zone.eq_ignore_ascii_case(letter)).map_or(Owner::Building, Owner::Face));
    }
    d.floors.iter().find(|e| e.id == base).map(Owner::Floor)
        .or_else(|| d.self_weight_elements.iter().find(|e| e.id == base).map(Owner::SelfWeight))
        .or_else(|| d.roofs.iter().enumerate().find(|(_, e)| e.id == base).map(|(i, e)| Owner::Roof(i, e)))
        .or_else(|| d.wind_faces.iter().find(|e| e.id == base).map(Owner::Face))
        .or_else(|| d.accidental_cases.iter().find(|e| e.id == base).map(Owner::Accidental))
        .or_else(|| d.bridge_lanes.iter().find(|e| e.id == base).map(Owner::Lane))
        .or_else(|| d.construction_stages.iter().find(|e| e.id == base).map(Owner::Stage))
        .or_else(|| d.roof_surfaces.iter().find(|e| e.id == base).map(Owner::Surface))
        .or_else(|| d.special_vehicles.iter().find(|e| e.id == base).map(Owner::Vehicle))
        .or_else(|| d.wind_bridges.iter().find(|e| e.id == base).map(Owner::BridgeWind))
        .or_else(|| d.dynamic_elements.iter().find(|e| e.id == base).map(Owner::Dynamic))
        .or_else(|| d.impact_actions.iter().find(|e| e.id == base).map(Owner::Impact))
        .or_else(|| d.internal_explosions.iter().find(|e| e.id == base).map(Owner::Explosion))
        .or_else(|| d.thermal_sections.iter().find(|e| e.id == base).map(Owner::Section))
        .or_else(|| d.helicopter_landings.iter().find(|e| e.id == base).map(Owner::Helicopter))
        .or_else(|| d.tie_runs.iter().find(|e| e.id == base).map(Owner::Tie))
        .or_else(|| d.construction_loads.iter().find(|e| e.id == base).map(Owner::ConstructionLoad))
        .or_else(|| d.falling_loads.iter().find(|e| e.id == base).map(Owner::Falling))
        .or_else(|| d.execution_imperfections.iter().find(|e| e.id == base).map(Owner::Imperfection))
        .or_else(|| d.derailment_cases.iter().find(|e| e.id == base).map(Owner::Derailment))
        .or_else(|| d.footbridges.iter().find(|e| e.id == base).map(Owner::Footbridge))
        .or_else(|| d.rail_dynamics.iter().find(|e| e.id == base).map(Owner::RailSpan))
        .or_else(|| d.load_dispersals.iter().find(|e| e.id == base).map(Owner::Dispersal))
        .or_else(|| d.bulk_solids.iter().find(|e| e.id == base).map(Owner::BulkSolid))
        .or_else(|| d.rotating_machines.iter().find(|e| e.id == base).map(Owner::Machine))
        .or_else(|| d.fatigue_models.iter().find(|e| e.id == base).map(Owner::FatigueModel))
        .or_else(|| d.support_collisions.iter().find(|e| e.id == base).map(Owner::Collision))
        .or_else(|| d.local_fires.iter().find(|e| e.id == base).map(Owner::LocalFire))
        .or_else(|| d.snow_guards.iter().find(|e| e.id == base).map(Owner::SnowGuard))
        .or_else(|| d.rail_derailments.iter().find(|e| e.id == base).map(Owner::RailDerailment))
        .or_else(|| d.rail_load_groups.iter().find(|e| e.id == base).map(Owner::RailGroup))
        .or_else(|| d.dust_explosions.iter().find(|e| e.id == base).map(Owner::Dust))
        .or_else(|| d.crane_hoists.iter().find(|e| e.id == base).map(Owner::CraneHoist))
}

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1991Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    if document.cases.iter().any(|c| c.id == base_id(id) && declared(c)) {
        return ENTITY_KINDS.iter().find(|k| k.collection == "cases");
    }
    let placed = document.placements.iter().any(|p| p.id == base_id(id));
    let Some(owner) = owner_of(document, id) else {
        return placed.then(|| ENTITY_KINDS.iter().find(|k| k.collection == "placements")).flatten();
    };
    let collection = match owner {
        Owner::Floor(_) => "floors",
        Owner::SelfWeight(_) => "selfWeightElements",
        Owner::Roof(..) => "roofs",
        Owner::Face(_) => "windFaces",
        Owner::Accidental(_) => "accidentalCases",
        Owner::Lane(_) => "bridgeLanes",
        Owner::Stage(_) => "constructionStages",
        Owner::Surface(_) => "roofSurfaces",
        Owner::Vehicle(_) => "specialVehicles",
        Owner::BridgeWind(_) => "windBridges",
        Owner::Dynamic(_) => "dynamicElements",
        Owner::Impact(_) => "impactActions",
        Owner::Explosion(_) => "internalExplosions",
        Owner::Section(_) => "thermalSections",
        Owner::Helicopter(_) => "helicopterLandings",
        Owner::Tie(_) => "tieRuns",
        Owner::ConstructionLoad(_) => "constructionLoads",
        Owner::Falling(_) => "fallingLoads",
        Owner::Imperfection(_) => "executionImperfections",
        Owner::Derailment(_) => "derailmentCases",
        Owner::CraneHoist(_) => "craneHoists",
        Owner::Footbridge(_) => "footbridges",
        Owner::RailSpan(_) => "railDynamics",
        Owner::Dispersal(_) => "loadDispersals",
        Owner::BulkSolid(_) => "bulkSolids",
        Owner::Machine(_) => "rotatingMachines",
        Owner::FatigueModel(_) => "fatigueModels",
        Owner::Collision(_) => "supportCollisions",
        Owner::LocalFire(_) => "localFires",
        Owner::SnowGuard(_) => "snowGuards",
        Owner::RailDerailment(_) => "railDerailments",
        Owner::RailGroup(_) => "railLoadGroups",
        Owner::Dust(_) => "dustExplosions",
        _ if placed => "placements",
        _ => return None,
    };
    ENTITY_KINDS.iter().find(|k| k.collection == collection)
}

/// 🌐️ The ids of the global scene entities: leaf scalar groups that own document scalars, and aggregates that draw or read many rows.
pub mod global {
    pub const ANNEX: &str = "annex";
    pub const WIND_SITE: &str = "wind-site";
    pub const SNOW_SITE: &str = "snow-site";
    pub const BUILDING: &str = "building";
    pub const BRIDGE: &str = "bridge";
    pub const REPORT: &str = "report";
    pub const ENVELOPE: &str = "envelope";
    pub const BRIDGE_DECK: &str = "bridge-deck";
    pub const WIND_ZONES: &str = "wind-zones";
    pub const WIND_QP: &str = "wind-qp";
    pub const THERMAL: &str = "thermal";
    pub const FIRE: &str = "fire";
    pub const CRANE: &str = "crane";
    pub const SILO: &str = "silo";
    pub const SELF_WEIGHT: &str = "selfWeight";
    pub const DEFLECTION: &str = "deflection";
    pub const ALL: [&str; 16] = [ANNEX, WIND_SITE, SNOW_SITE, BUILDING, BRIDGE, REPORT, ENVELOPE, BRIDGE_DECK, WIND_ZONES, WIND_QP, THERMAL, FIRE, CRANE, SILO, SELF_WEIGHT, DEFLECTION];
}

/// 🗂️ The top-level scalars of the snapshot by the global entity that owns them, field names lower-cased without underscores; `report` holds the scalars no projection reads. A unit test enumerates the snapshot, so a new scalar must be listed.
const SCALARS: &[(&str, &[&str])] = &[
    (global::ANNEX, &["annex"]),
    (global::WIND_SITE, &["windzone", "envb", "terraincategory", "mixedterrainupwind", "mixedterraindistance", "orographyfactor", "coastorisland", "airdensity"]),
    (global::SNOW_SITE, &["snowzone", "altitude", "ensk"]),
    (global::BUILDING, &["depth", "width", "height", "storeycount"]),
    (global::BRIDGE, &["structurekind", "bridgespan", "bridgelanewidth", "bridgetandemx"]),
    (global::BRIDGE_DECK, &["bridgeloadgroup", "assumedraillm71", "assumedraillm71udl", "assumedrailsw0", "assumedrailsw2"]),
    (global::THERMAL, &["tmax", "tmin", "t0", "thermalelementtype", "thermalbridgetype", "deltatm"]),
    (global::FIRE, &["firemode", "firecurve", "fireduration", "firecompartmentarea", "firecompartmentheight", "fireopeningfactor", "firethermalinertia", "fireoccupancy", "fireloadbasis", "fireloaddensityqf"]),
    (global::CRANE, &["craneclaimed", "craneclass", "hoistclass", "hoistingspeed", "cranex", "cranerunwaylength", "cranewheelbase"]),
    (global::SILO, &["siloclaimed", "silokind", "silobulkdensity", "siloheight", "silohydraulicradius", "silomu", "silok", "silodiameter", "siloeccentricity", "silohopperangledeg"]),
    (
        global::REPORT,
        &[
            "northgermanlowlandsnow",
            "assumeddeltat",
            "assumedgastemperature",
            "assumedhnet",
            "assumedqfd",
            "constructionactivity",
            "assumedconstructionqk",
            "bridgelane",
            "assumedbridgetandem",
            "assumedbridgeudl",
            "assumedbridgelm2",
            "assumedbridgefootway",
            "assumedbridgelm3",
            "assumedbridgelm4",
            "railalpha",
            "assumedcranewheel",
            "assumedcranehorizontal",
            "assumedsilopressure",
            "assumedsilopatch",
            "assumedsilowallfriction",
        ],
    ),
];

fn normal(name: &str) -> String {
    name.chars().filter(|c| *c != '_').flat_map(char::to_lowercase).collect()
}

/// 🗂️ The global entity that owns the top-level scalar `field` of the snapshot, `None` for a field no group lists.
pub fn scalar_owner(field: &str) -> Option<&'static str> {
    let key = normal(field);
    SCALARS.iter().find(|(_, fields)| fields.contains(&key.as_str())).map(|(owner, _)| *owner)
}

/// 🔇️ Row fields that only point at another entity without either projection reading it: a roof surface names its wind face.
pub fn inert_reference(collection: &str, field: &str) -> bool {
    (normal(collection).as_str(), normal(field).as_str()) == ("roofsurfaces", "face")
}

/// 🧭️ The scene entity that reads a check row without a subject row: the aggregate drawn from that check family, else the `report` sink.
pub fn report_owner(check_id: &str) -> &'static str {
    const FAMILIES: [(&str, &str); 7] = [
        ("en1991.1-2.", global::FIRE),
        ("en1991.1-3.", global::DEFLECTION),
        ("en1991.1-4.", global::DEFLECTION),
        ("en1991.1-5.", global::THERMAL),
        ("en1991.2.", global::BRIDGE_DECK),
        ("en1991.3.", global::CRANE),
        ("en1991.4.", global::SILO),
    ];
    FAMILIES.iter().find(|(prefix, _)| check_id.starts_with(prefix)).map_or(global::REPORT, |(_, owner)| *owner)
}

/// 📈️ The check families a dedicated diagram draws (curves, profiles, pressure maps, timelines and plans of their entity), as id prefixes: the longest prefix of a check id names its family.
pub const DRAWN_FAMILIES: &[&str] = &[
    "en1991.1-1.imposed", "en1991.1-1.self-weight", "en1991.1-2.gas-temperature", "en1991.1-2.local-flux",
    "en1991.1-3.snow", "en1991.1-3.roof-surface-mu", "en1991.1-3.snow-guard",
    "en1991.1-4.wind", "en1991.1-4.cpe10", "en1991.1-4.bridge-wind", "en1991.1-4.dyn-cscd", "en1991.1-4.vortex-shedding",
    "en1991.1-5.section-range",
    "en1991.1-6.stage-return-period", "en1991.1-6.stage-combination", "en1991.1-6.falling-load", "en1991.1-6.execution-imperfection",
    "en1991.1-7.impact-force", "en1991.1-7.gas-explosion", "en1991.1-7.impact", "en1991.1-7.explosion", "en1991.1-7.derailment-force", "en1991.1-7.helicopter-force", "en1991.1-7.tie-force",
    "en1991.2.lm1-tandem", "en1991.2.rail.lm71-axle", "en1991.2.lm1-lane", "en1991.2.lm1-lane-udl", "en1991.2.fatigue-axle", "en1991.2.collision-x", "en1991.2.footbridge-load", "en1991.2.rail-dynamic", "en1991.2.dispersal-footprint", "en1991.2.lm3-vehicle",
    "en1991.3.hoist-release", "en1991.3.machine-force",
    "en1991.4.silo-pressure", "en1991.4.tank", "en1991.4.bulk-k-mean",
    "en1991.2.derail-line-load", "en1991.2.rail-group-axle", "en1991.4.dust-vent-area",
    "en1991.e",
];

/// 📊️ The check families without a dedicated diagram: the outcome diagram of their entity draws every check of them as a utilisation bar against the limit line.
pub const OUTCOME_FAMILIES: &[&str] = &[
    "en1991.1-1.concentrated", "en1991.1-1.floor-force", "en1991.1-1.partitions",
    "en1991.1-2.fire-load", "en1991.1-2.h-net", "en1991.1-2.opening-factor", "en1991.1-2.validity", "en1991.1-2.local-flame", "en1991.1-2.local-plume",
    "en1991.1-3.roof-parapet-drift", "en1991.1-3.roof-step-drift", "en1991.1-3.snow-exceptional",
    "en1991.1-4.cpe1", "en1991.1-4.roof-surface-cpe", "en1991.1-4.roof-surface-extent", "en1991.1-4.dyn-frequency", "en1991.1-4.vortex-reynolds", "en1991.1-4.vortex-scruton", "en1991.1-4.bridge-wind-longitudinal", "en1991.1-4.bridge-wind-reaction", "en1991.1-4.bridge-wind-vertical",
    "en1991.1-5.combination", "en1991.1-5.isotherm", "en1991.1-5.thermal", "en1991.1-5.section-combination", "en1991.1-5.section-curvature", "en1991.1-5.section-expansion", "en1991.1-5.section-gradient", "en1991.1-5.section-rotation",
    "en1991.1-6.stage-load", "en1991.1-6.stage-wind",
    "en1991.1-7.impact-energy", "en1991.1-7.impact-lateral", "en1991.1-7.gas-explosion-domain",
    "en1991.2.braking", "en1991.2.centrifugal", "en1991.2.collision-y", "en1991.2.collision-height", "en1991.2.fatigue-udl", "en1991.2.fatigue-vehicle", "en1991.2.footbridge-horizontal", "en1991.2.footbridge-walking", "en1991.2.footway", "en1991.2.group.selected",
    "en1991.2.lane", "en1991.2.lane-inside", "en1991.2.lane-width", "en1991.2.lm1-remaining", "en1991.2.lm1-tandem-position", "en1991.2.lm1-udl", "en1991.2.lm2", "en1991.2.lm3", "en1991.2.lm4",
    "en1991.2.lm3-vehicle-lateral", "en1991.2.lm3-vehicle-length", "en1991.2.lm3-vehicle-position", "en1991.2.notional-lanes", "en1991.2.rail.lm71-udl", "en1991.2.rail.sw0", "en1991.2.rail.sw2",
    "en1991.3.crane-horizontal", "en1991.3.crane-position", "en1991.3.crane-runway", "en1991.3.crane-vertical", "en1991.3.hoist-test-static", "en1991.3.hoist-test-dynamic",
    "en1991.4.silo-class", "en1991.4.silo-discharge", "en1991.4.silo-discharge-friction", "en1991.4.silo-eccentricity", "en1991.4.silo-hopper", "en1991.4.silo-hydraulic-radius", "en1991.4.silo-patch", "en1991.4.silo-wall-friction",
    "en1991.2.derail-length", "en1991.2.derail-offset", "en1991.2.rail-group-udl", "en1991.2.rail-group-longitudinal", "en1991.2.rail-group-lateral", "en1991.2.rail-group-combination", "en1991.2.footbridge-service", "en1991.4.dust-pressure",
    "en1991.input", "en1991.integrity",
];

/// 📄️ The check families that belong to the document as a whole (invalid-input and integrity notes of the scalars, and the `none` notes of a structure the document does not claim); they have no 3D owner and select the global badge.
pub const DOCUMENT_CHECKS: &[&str] = &[
    "en1991.input.", "en1991.integrity.", "en1991.1-1.imposed.none", "en1991.1-1.self-weight.none", "en1991.1-2.fire.none", "en1991.1-3.snow.none", "en1991.1-4.wind.none", "en1991.1-6.construction", "en1991.1-7.accidental.none", "en1991.2.bridge.none", "en1991.3.crane.none", "en1991.4.silo.none",
];

/// 🧭️ The family of a check id: its longest prefix among [`DRAWN_FAMILIES`] and [`OUTCOME_FAMILIES`].
pub fn family_of(check_id: &str) -> Option<&'static str> {
    DRAWN_FAMILIES.iter().chain(OUTCOME_FAMILIES).copied().filter(|prefix| check_id.starts_with(prefix)).max_by_key(|prefix| prefix.len())
}

/// 📊️ Whether the outcome diagram of an entity draws `check`: its family has no dedicated diagram and it carries a finite utilisation of an applicable, non-informative check.
pub fn is_outcome(check: &norm::CheckResult) -> bool {
    check.utilization.is_finite() && !matches!(check.status, norm::CheckStatus::NotApplicable | norm::CheckStatus::Info) && family_of(&check.id).is_some_and(|family| OUTCOME_FAMILIES.contains(&family))
}

/// 🧭️ Where a check is drawn: on one scene entity, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

fn is_document_check(id: &str) -> bool {
    DOCUMENT_CHECKS.iter().any(|prefix| id.starts_with(prefix))
}

fn unfiled(check: &norm::CheckResult) -> bool {
    let subject = base_id(&check.subject.entity_id);
    subject.is_empty() || subject == global::REPORT
}

fn drawn_entity(document: &model::En1991Snapshot, filed: &str) -> Option<String> {
    match filed {
        global::BRIDGE_DECK | global::BRIDGE => Some(global::BRIDGE.to_owned()),
        global::ENVELOPE | global::WIND_ZONES | global::WIND_QP | global::DEFLECTION | global::BUILDING => Some(global::BUILDING.to_owned()),
        global::CRANE | global::SILO | global::FIRE | global::THERMAL => Some(filed.to_owned()),
        global::ANNEX | global::WIND_SITE | global::SNOW_SITE | global::SELF_WEIGHT | global::REPORT | "" => None,
        row => entity_kind_of(document, row).map(|_| row.to_owned()),
    }
}

/// 🧭️ The scene entity that draws `check`: the row its subject names, else the aggregate entity [`report_owner`] files its family under (the deck of a bridge, the building for the pressure and deflection families); `None` for a document-level check. The tone of that entity is the worst check it owns.
pub fn owner_entity(document: &model::En1991Snapshot, check: &norm::CheckResult) -> Option<String> {
    let subject = base_id(&check.subject.entity_id);
    drawn_entity(document, if subject.is_empty() { report_owner(&check.id) } else { subject })
}

/// 🧭️ The route of `check`: a document-level family without a row subject goes to the global badge, every other check to its owner entity; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::En1991Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    if unfiled(check) && is_document_check(&check.id) {
        return Some(CheckRoute::Document);
    }
    owner_entity(document, check).map(CheckRoute::Entity)
}

/// 🎯️ The entity to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`].
pub fn focus_of_outcome(document: &model::En1991Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📶️ The governing binding among the checks owned by the entity `id` whose id starts with any of `prefixes` (every owned check when empty).
pub fn binding_of_owner(document: &model::En1991Snapshot, report: &norm::CheckReport, id: &str, prefixes: &[&str]) -> Option<UtilisationBinding> {
    projection_kit::binding_where(report, |check| owner_entity(document, check).as_deref() == Some(id) && (prefixes.is_empty() || prefixes.iter().any(|p| check.id.starts_with(p))))
}

/// 🧭️ The report as the live scene reads it: every check row without a subject row is filed under the scene entity that reads it, so an edit of one scalar group changes the check slice of that group only; the document-level families stay unfiled, so the global badge sums them up.
pub fn scene_report(report: &norm::CheckReport) -> norm::CheckReport {
    let mut scoped = report.clone();
    scoped.checks.iter_mut().filter(|c| c.subject.entity_id.is_empty() && !is_document_check(&c.id)).for_each(|c| c.subject.entity_id = report_owner(&c.id).to_owned());
    scoped
}
