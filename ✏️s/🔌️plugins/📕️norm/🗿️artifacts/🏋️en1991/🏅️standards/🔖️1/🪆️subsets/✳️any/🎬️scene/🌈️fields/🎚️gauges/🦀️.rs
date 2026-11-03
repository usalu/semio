//! 🎚️ Gauge tables: one scalar grip per numeric field of a row or of the document that has no geometric grip of its own, listed per owner in one table, with the schema step and the physical range (never a normative one). The drag parameter `s` is the displacement in metres along `+Z` of the gauge anchor, so the field is `value + 20 * step * s`; the tables are the contract checked against the snapshot schema by the grip coverage test and read by the handles of the scene members.

use crate::model::{
    ConstructionLoad, ConstructionStage, DerailmentCase, DynamicElement, ExecutionImperfection, FallingLoad, FloorArea, HelicopterLanding, ImpactAction, InternalExplosion, RoofArea, SelfWeightElement, ThermalSection, TieRun, WindBridge, WindFace,
};

pub const STEPS_PER_METRE: f64 = 20.0;
const STRESS: f64 = 1.0e9;
const FORCE: f64 = 1.0e10;
const OPEN: f64 = 1.0e12;
const CODE: f64 = 255.0;

pub struct Gauge<T> {
    pub tag: &'static str,
    pub field: &'static str,
    pub get: fn(&T) -> f64,
    pub unit: &'static str,
    pub step: f64,
    pub lo: f64,
    pub hi: f64,
    pub int: bool,
    pub en: &'static str,
    pub de: &'static str,
}

macro_rules! gauge {
    ($tag:literal, $field:literal, $get:expr, $unit:literal, $step:expr, $lo:expr, $hi:expr, $en:literal, $de:literal) => {
        Gauge { tag: $tag, field: $field, get: $get, unit: $unit, step: $step, lo: $lo, hi: $hi, int: false, en: $en, de: $de }
    };
}

macro_rules! code {
    ($tag:literal, $field:literal, $get:expr, $en:literal, $de:literal) => {
        Gauge { tag: $tag, field: $field, get: $get, unit: "1", step: 1.0, lo: 0.0, hi: CODE, int: true, en: $en, de: $de }
    };
}

pub const FLOORS: &[Gauge<FloorArea>] = &[
    gauge!("assumed-qk", "assumedQk", |r| r.assumed_qk, "Pa", 50.0, 0.0, STRESS, "Assumed imposed load q_k", "Angesetzte Nutzlast q_k"),
    gauge!("assumed-qk-concentrated", "assumedQkConcentrated", |r| r.assumed_qk_concentrated, "N", 500.0, 0.0, FORCE, "Assumed concentrated load Q_k", "Angesetzte Einzellast Q_k"),
    gauge!("assumed-partitions", "assumedPartitions", |r| r.assumed_partitions, "Pa", 100.0, 0.0, STRESS, "Assumed partition load", "Angesetzte Trennwandlast"),
];

pub const SELF_WEIGHT_ELEMENTS: &[Gauge<SelfWeightElement>] = &[gauge!("assumed-gk", "assumedGk", |r| r.assumed_gk, "Pa", 50.0, 0.0, STRESS, "Assumed permanent load g_k", "Angesetzte ständige Last g_k")];

pub const ROOFS: &[Gauge<RoofArea>] = &[
    gauge!("c-e", "cE", |r| r.c_e, "1", 0.1, 0.0, OPEN, "Exposure coefficient C_e", "Umgebungsbeiwert C_e"),
    gauge!("c-t", "cT", |r| r.c_t, "1", 0.05, 0.0, OPEN, "Thermal coefficient C_t", "Wärmebeiwert C_t"),
    gauge!("assumed-sk", "assumedSk", |r| r.assumed_sk, "Pa", 50.0, 0.0, STRESS, "Assumed roof snow load", "Angesetzte Dachschneelast"),
];

pub const WIND_FACES: &[Gauge<WindFace>] = &[
    gauge!("c-pe10", "cPe10", |r| r.c_pe10, "1", 0.05, -OPEN, OPEN, "External pressure coefficient c_pe,10", "Außendruckbeiwert c_pe,10"),
    gauge!("c-pe1", "cPe1", |r| r.c_pe1, "1", 0.05, -OPEN, OPEN, "External pressure coefficient c_pe,1", "Außendruckbeiwert c_pe,1"),
    gauge!("c-pi", "cPi", |r| r.c_pi, "1", 0.05, -OPEN, OPEN, "Internal pressure coefficient c_pi", "Innendruckbeiwert c_pi"),
    gauge!("c-s", "cS", |r| r.c_s, "1", 0.01, 0.0, OPEN, "Size factor c_s", "Baugrößenbeiwert c_s"),
    gauge!("c-d", "cD", |r| r.c_d, "1", 0.01, 0.0, OPEN, "Dynamic factor c_d", "Schwingungsbeiwert c_d"),
    gauge!("assumed-wp", "assumedWp", |r| r.assumed_wp, "Pa", 50.0, 0.0, STRESS, "Assumed wind pressure", "Angesetzter Winddruck"),
];

pub const CONSTRUCTION_STAGES: &[Gauge<ConstructionStage>] = &[gauge!("return-years", "returnYears", |r| r.return_years, "a", 0.5, 0.0, OPEN, "Return period of the stage", "Wiederkehrperiode des Bauzustands")];

pub const WIND_BRIDGES: &[Gauge<WindBridge>] = &[
    code!("parapet-kind", "parapetKind", |r| f64::from(r.parapet_kind), "Parapet kind code of the deck", "Brüstungsart des Überbaus"),
    gauge!("parapet-height", "parapetHeight", |r| r.parapet_height, "m", 0.05, 0.0, 100.0, "Parapet height of the deck", "Brüstungshöhe des Überbaus"),
    code!("traffic-kind", "trafficKind", |r| f64::from(r.traffic_kind), "Traffic kind code of the deck", "Verkehrsart auf dem Überbau"),
    gauge!("assumed-force", "assumedForce", |r| r.assumed_force, "N/m", 100.0, 0.0, FORCE, "Assumed wind force per length on the deck", "Angesetzte Windkraft je Länge auf dem Überbau"),
];

pub const DYNAMIC_ELEMENTS: &[Gauge<DynamicElement>] = &[
    gauge!("first-frequency", "firstFrequency", |r| r.first_frequency, "Hz", 0.05, 0.0, OPEN, "First natural frequency n_1", "Erste Eigenfrequenz n_1"),
    gauge!("strouhal", "strouhal", |r| r.strouhal, "1", 0.01, 0.0, OPEN, "Strouhal number St", "Strouhal-Zahl St"),
    gauge!("assumed-cscd", "assumedCscd", |r| r.assumed_cscd, "1", 0.01, 0.0, OPEN, "Assumed structural factor c_s c_d", "Angesetzter Bauwerksbeiwert c_s c_d"),
];

pub const IMPACT_ACTIONS: &[Gauge<ImpactAction>] = &[
    code!("kind", "kind", |r| f64::from(r.kind), "Impact kind code", "Anprallart"),
    gauge!("mass", "mass", |r| r.mass, "kg", 100.0, 0.0, OPEN, "Impacting mass", "Anprallmasse"),
    gauge!("speed", "speed", |r| r.speed, "m/s", 0.5, 0.0, OPEN, "Impact speed", "Anprallgeschwindigkeit"),
    gauge!("stiffness", "stiffness", |r| r.stiffness, "N/m", 100.0, 0.0, OPEN, "Impact stiffness", "Anprallsteifigkeit"),
];

pub const INTERNAL_EXPLOSIONS: &[Gauge<InternalExplosion>] = &[
    gauge!("vent-pressure", "ventPressure", |r| r.vent_pressure, "Pa", 50.0, 0.0, STRESS, "Vent opening pressure", "Öffnungsdruck der Entlastungsfläche"),
    gauge!("assumed-pressure", "assumedPressure", |r| r.assumed_pressure, "Pa", 50.0, 0.0, STRESS, "Assumed explosion pressure", "Angesetzter Explosionsdruck"),
];

pub const THERMAL_SECTIONS: &[Gauge<ThermalSection>] = &[
    code!("deck-type", "deckType", |r| f64::from(r.deck_type), "Deck type code of the section", "Überbauart des Querschnitts"),
    code!("surfacing", "surfacing", |r| f64::from(r.surfacing), "Surfacing code of the section", "Belagsart des Querschnitts"),
    gauge!("alpha-micro", "alphaMicro", |r| r.alpha_micro, "1", 0.5, 0.0, OPEN, "Thermal expansion coefficient of the section", "Wärmedehnzahl des Querschnitts"),
    gauge!("assumed-range", "assumedRange", |r| r.assumed_range, "K", 0.5, 0.0, OPEN, "Assumed temperature range of the section", "Angesetzter Temperaturbereich des Querschnitts"),
];

pub const HELICOPTER_LANDINGS: &[Gauge<HelicopterLanding>] = &[gauge!("mass", "mass", |r| r.mass, "kg", 100.0, 0.0, OPEN, "Helicopter mass", "Hubschraubermasse")];

pub const TIE_RUNS: &[Gauge<TieRun>] = &[
    code!("kind", "kind", |r| f64::from(r.kind), "Tie kind code", "Zuggliedart"),
    gauge!("dead-load", "deadLoad", |r| r.dead_load, "Pa", 50.0, 0.0, STRESS, "Permanent load g_k on the tie run", "Ständige Last g_k auf dem Zugglied"),
    gauge!("imposed-load", "imposedLoad", |r| r.imposed_load, "Pa", 50.0, 0.0, STRESS, "Imposed load q_k on the tie run", "Nutzlast q_k auf dem Zugglied"),
    gauge!("psi", "psi", |r| r.psi, "1", 0.05, 0.0, 1.0, "Combination factor of the tie run", "Kombinationsbeiwert des Zuggliedes"),
];

pub const CONSTRUCTION_LOADS: &[Gauge<ConstructionLoad>] = &[code!("kind", "kind", |r| f64::from(r.kind), "Construction load kind code", "Bauzustandslastart")];

pub const FALLING_LOADS: &[Gauge<FallingLoad>] = &[
    gauge!("mass", "mass", |r| r.mass, "kg", 100.0, 0.0, OPEN, "Falling mass", "Fallende Masse"),
    gauge!("stopping-distance", "stoppingDistance", |r| r.stopping_distance, "m", 0.01, 0.0, OPEN, "Stopping distance s", "Bremsweg s"),
];

pub const EXECUTION_IMPERFECTIONS: &[Gauge<ExecutionImperfection>] = &[
    gauge!("vertical-load", "verticalLoad", |r| r.vertical_load, "N", 100.0, 0.0, FORCE, "Vertical load on the member", "Vertikallast auf dem Bauteil"),
    gauge!("y", "y", |r| r.y, "m", 0.5, -OPEN, OPEN, "Member position y", "Bauteilposition y"),
    gauge!("assumed-force", "assumedForce", |r| r.assumed_force, "N", 100.0, 0.0, FORCE, "Assumed horizontal replacement force", "Angesetzte horizontale Ersatzkraft"),
];

pub const DERAILMENT_CASES: &[Gauge<DerailmentCase>] = &[
    gauge!("design-force-x", "designForceX", |r| r.design_force_x, "N", 100.0, 0.0, FORCE, "Design force F_dx along the track", "Bemessungskraft F_dx in Gleisrichtung"),
    gauge!("design-force-y", "designForceY", |r| r.design_force_y, "N", 100.0, 0.0, FORCE, "Design force F_dy across the track", "Bemessungskraft F_dy quer zum Gleis"),
    gauge!("y", "y", |r| r.y, "m", 0.5, -OPEN, OPEN, "Track position y", "Gleisposition y"),
];

pub type Site = crate::model::En1991Snapshot;

pub const SITE: &[Gauge<Site>] = &[
    gauge!("en-sk", "enSk", |r| r.en_sk, "Pa", 100.0, 0.0, STRESS, "Characteristic ground snow load s_k", "Charakteristische Schneelast s_k am Boden"),
    gauge!("en-vb", "enVb", |r| r.en_vb, "m/s", 0.5, 0.0, OPEN, "Basic wind velocity v_b", "Basiswindgeschwindigkeit v_b"),
    code!("mixed-terrain-upwind", "mixedTerrainUpwind", |r| f64::from(r.mixed_terrain_upwind), "Upwind terrain category of the mixed terrain", "Luvseitige Geländekategorie des gemischten Geländes"),
    gauge!("mixed-terrain-distance", "mixedTerrainDistance", |r| r.mixed_terrain_distance, "m", 10.0, 0.0, OPEN, "Distance of the mixed terrain", "Abstand des gemischten Geländes"),
    gauge!("orography-factor", "orographyFactor", |r| r.orography_factor, "1", 0.01, 0.0, OPEN, "Orography factor c_o", "Geländefaktor c_o"),
    gauge!("air-density", "airDensity", |r| r.air_density, "kg/m3", 0.01, 0.0, OPEN, "Air density", "Luftdichte"),
    gauge!("assumed-construction-qk", "assumedConstructionQk", |r| r.assumed_construction_qk, "Pa", 100.0, 0.0, STRESS, "Assumed construction imposed load", "Angenommene Bauzustandslast"),
];

pub const THERMAL: &[Gauge<Site>] = &[
    gauge!("t-max", "tMax", |r| r.t_max, "degC", 0.5, -OPEN, OPEN, "Maximum shade air temperature", "Maximale Lufttemperatur im Schatten"),
    gauge!("t-min", "tMin", |r| r.t_min, "degC", 0.5, -OPEN, OPEN, "Minimum shade air temperature", "Minimale Lufttemperatur im Schatten"),
    gauge!("t0", "t0", |r| r.t_0, "degC", 0.5, -OPEN, OPEN, "Initial temperature T_0", "Ausgangstemperatur T_0"),
    code!("bridge-type", "thermalBridgeType", |r| f64::from(r.thermal_bridge_type), "Bridge deck type code", "Brückenüberbauart"),
    gauge!("delta-t-m", "deltaTM", |r| r.delta_t_m, "K", 0.5, -OPEN, OPEN, "Linear temperature difference ΔT_M", "Lineare Temperaturdifferenz ΔT_M"),
    gauge!("assumed-delta-t", "assumedDeltaT", |r| r.assumed_delta_t, "K", 0.5, 0.0, OPEN, "Assumed temperature difference", "Angenommene Temperaturdifferenz"),
];

pub const FIRE: &[Gauge<Site>] = &[
    gauge!("duration", "fireDuration", |r| r.fire_duration, "s", 60.0, 0.0, OPEN, "Fire duration", "Branddauer"),
    gauge!("assumed-gas-temperature", "assumedGasTemperature", |r| r.assumed_gas_temperature, "K", 10.0, 0.0, OPEN, "Assumed gas temperature θ_g", "Angenommene Gastemperatur θ_g"),
    gauge!("assumed-h-net", "assumedHNet", |r| r.assumed_h_net, "W/m2", 1000.0, 0.0, OPEN, "Assumed net heat flux", "Angenommener Nettowärmestrom"),
    gauge!("compartment-area", "fireCompartmentArea", |r| r.fire_compartment_area, "m2", 1.0, 0.0, OPEN, "Fire compartment floor area", "Brandabschnittsfläche"),
    gauge!("load-density-qf", "fireLoadDensityQf", |r| r.fire_load_density_qf, "J/m2", 1.0e6, 0.0, OPEN, "Specified fire load density q_f,k", "Vorgegebene Brandlastdichte q_f,k"),
    gauge!("assumed-qf-d", "assumedQfD", |r| r.assumed_qf_d, "J/m2", 1.0e6, 0.0, OPEN, "Assumed design fire load q_f,d", "Angenommene Bemessungsbrandlastdichte q_f,d"),
];

pub const BRIDGE: &[Gauge<Site>] = &[
    gauge!("curve-radius", "bridgeCurveRadius", |r| r.bridge_curve_radius, "m", 5.0, 0.0, OPEN, "Horizontal deck radius (0 = straight deck)", "Horizontaler Deckradius (0 = gerades Deck)"),
    gauge!("assumed-tandem", "assumedBridgeTandem", |r| r.assumed_bridge_tandem, "N", 1000.0, 0.0, FORCE, "Assumed LM1 tandem axle", "Angenommenes LM1-Tandem"),
    gauge!("assumed-udl", "assumedBridgeUdl", |r| r.assumed_bridge_udl, "Pa", 50.0, 0.0, STRESS, "Assumed LM1 uniform load", "Angenommene LM1-Gleichlast"),
    gauge!("assumed-lm2", "assumedBridgeLm2", |r| r.assumed_bridge_lm2, "N", 1000.0, 0.0, FORCE, "Assumed LM2 axle", "Angenommene LM2-Achse"),
    gauge!("assumed-footway", "assumedBridgeFootway", |r| r.assumed_bridge_footway, "Pa", 50.0, 0.0, STRESS, "Assumed footway load", "Angenommene Gehweglast"),
    gauge!("assumed-lm3", "assumedBridgeLm3", |r| r.assumed_bridge_lm3, "N", 1000.0, 0.0, FORCE, "Assumed LM3 special vehicle axle", "Angenommene LM3-Sonderfahrzeugachse"),
    gauge!("assumed-lm4", "assumedBridgeLm4", |r| r.assumed_bridge_lm4, "Pa", 50.0, 0.0, STRESS, "Assumed LM4 crowd load", "Angenommene LM4-Menschenlast"),
    gauge!("assumed-rail-udl", "assumedRailLm71Udl", |r| r.assumed_rail_lm71_udl, "N/m", 1000.0, 0.0, FORCE, "Assumed LM71 distributed load", "Angesetzte LM71-Streckenlast"),
    gauge!("assumed-rail-sw0", "assumedRailSw0", |r| r.assumed_rail_sw0, "N/m", 1000.0, 0.0, FORCE, "Assumed SW/0 load", "Angesetzte SW/0-Last"),
    gauge!("assumed-rail-sw2", "assumedRailSw2", |r| r.assumed_rail_sw2, "N/m", 1000.0, 0.0, FORCE, "Assumed SW/2 load", "Angesetzte SW/2-Last"),
];

pub const CRANE: &[Gauge<Site>] = &[
    gauge!("assumed-wheel", "assumedCraneWheel", |r| r.assumed_crane_wheel, "N", 1000.0, 0.0, FORCE, "Assumed crane wheel load", "Angenommene Kranradlast"),
    gauge!("assumed-horizontal", "assumedCraneHorizontal", |r| r.assumed_crane_horizontal, "N", 1000.0, 0.0, FORCE, "Assumed crane horizontal force", "Angenommene Kranhorizontalkraft"),
];

pub const SILO: &[Gauge<Site>] = &[
    gauge!("bulk-density", "siloBulkDensity", |r| r.silo_bulk_density, "N/m3", 100.0, 0.0, OPEN, "Bulk unit weight γ", "Wichte γ des Schüttguts"),
    gauge!("hydraulic-radius", "siloHydraulicRadius", |r| r.silo_hydraulic_radius, "m", 0.05, 0.0, OPEN, "Hydraulic radius a", "Hydraulischer Radius a"),
    gauge!("mu", "siloMu", |r| r.silo_mu, "1", 0.01, 0.0, OPEN, "Wall friction μ", "Wandreibung μ"),
    gauge!("k", "siloK", |r| r.silo_k, "1", 0.01, 0.0, OPEN, "Lateral pressure ratio k", "Seitendruckverhältnis k"),
    gauge!("assumed-pressure", "assumedSiloPressure", |r| r.assumed_silo_pressure, "Pa", 100.0, 0.0, STRESS, "Assumed silo or tank pressure", "Angenommener Silo- oder Tankdruck"),
    gauge!("assumed-patch", "assumedSiloPatch", |r| r.assumed_silo_patch, "Pa", 100.0, 0.0, STRESS, "Assumed patch pressure", "Angenommener Patchdruck"),
    gauge!("assumed-wall-friction", "assumedSiloWallFriction", |r| r.assumed_silo_wall_friction, "Pa", 100.0, 0.0, STRESS, "Assumed wall friction traction", "Angenommene Wandreibung"),
];

/// 🚫 The numeric schema fields no viewport grip writes, with the reason: a placement is moved in the plane by the translate grips and has no vertical or rotation drag.
pub const EXEMPT: &[(&str, &str, &str)] = &[("placements", "z", "a placement is dragged in the plane only"), ("placements", "yaw", "a placement has no rotation drag")];

/// 🎚️ One row of the contract: the collection (empty for the document scalars), the schema field, its step and whether it is an integer code.
pub type Contract = (&'static str, &'static str, f64, bool);

/// 🎚️ Every gauge as `(collection, field, step, integer)`: the contract of the grip coverage test, `""` standing for the top-level scalars of the snapshot.
pub fn coverage() -> Vec<Contract> {
    fn rows<T>(collection: &'static str, table: &[Gauge<T>]) -> Vec<Contract> {
        table.iter().map(|g| (collection, g.field, g.step, g.int)).collect()
    }
    [
        rows("floors", FLOORS),
        rows("selfWeightElements", SELF_WEIGHT_ELEMENTS),
        rows("roofs", ROOFS),
        rows("windFaces", WIND_FACES),
        rows("constructionStages", CONSTRUCTION_STAGES),
        rows("windBridges", WIND_BRIDGES),
        rows("dynamicElements", DYNAMIC_ELEMENTS),
        rows("impactActions", IMPACT_ACTIONS),
        rows("internalExplosions", INTERNAL_EXPLOSIONS),
        rows("thermalSections", THERMAL_SECTIONS),
        rows("helicopterLandings", HELICOPTER_LANDINGS),
        rows("tieRuns", TIE_RUNS),
        rows("constructionLoads", CONSTRUCTION_LOADS),
        rows("fallingLoads", FALLING_LOADS),
        rows("executionImperfections", EXECUTION_IMPERFECTIONS),
        rows("derailmentCases", DERAILMENT_CASES),
        rows("", SITE),
        rows("", THERMAL),
        rows("", FIRE),
        rows("", BRIDGE),
        rows("", CRANE),
        rows("", SILO),
    ]
    .concat()
}

/// 🎚️ The current value of the field a gauge path names, `None` for a path no gauge writes: how the tests read the document value a gauge starts from.
pub fn value_at(d: &Site, path: &str) -> Option<f64> {
    fn row<T>(table: &[Gauge<T>], row: Option<&T>, field: &str) -> Option<f64> {
        let g = table.iter().find(|g| g.field == field)?;
        row.map(g.get)
    }
    let (head, field) = path.rsplit_once('.').map_or(("", path), |(h, f)| (h, f));
    if head.is_empty() {
        return [SITE, THERMAL, FIRE, BRIDGE, CRANE, SILO].iter().find_map(|table| row(table, Some(d), field));
    }
    let (collection, rest) = head.split_once("[id=")?;
    let id = rest.strip_suffix(']')?;
    match collection {
        "floors" => row(FLOORS, d.floors.iter().find(|r| r.id == id), field),
        "selfWeightElements" => row(SELF_WEIGHT_ELEMENTS, d.self_weight_elements.iter().find(|r| r.id == id), field),
        "roofs" => row(ROOFS, d.roofs.iter().find(|r| r.id == id), field),
        "windFaces" => row(WIND_FACES, d.wind_faces.iter().find(|r| r.id == id), field),
        "constructionStages" => row(CONSTRUCTION_STAGES, d.construction_stages.iter().find(|r| r.id == id), field),
        "windBridges" => row(WIND_BRIDGES, d.wind_bridges.iter().find(|r| r.id == id), field),
        "dynamicElements" => row(DYNAMIC_ELEMENTS, d.dynamic_elements.iter().find(|r| r.id == id), field),
        "impactActions" => row(IMPACT_ACTIONS, d.impact_actions.iter().find(|r| r.id == id), field),
        "internalExplosions" => row(INTERNAL_EXPLOSIONS, d.internal_explosions.iter().find(|r| r.id == id), field),
        "thermalSections" => row(THERMAL_SECTIONS, d.thermal_sections.iter().find(|r| r.id == id), field),
        "helicopterLandings" => row(HELICOPTER_LANDINGS, d.helicopter_landings.iter().find(|r| r.id == id), field),
        "tieRuns" => row(TIE_RUNS, d.tie_runs.iter().find(|r| r.id == id), field),
        "constructionLoads" => row(CONSTRUCTION_LOADS, d.construction_loads.iter().find(|r| r.id == id), field),
        "fallingLoads" => row(FALLING_LOADS, d.falling_loads.iter().find(|r| r.id == id), field),
        "executionImperfections" => row(EXECUTION_IMPERFECTIONS, d.execution_imperfections.iter().find(|r| r.id == id), field),
        "derailmentCases" => row(DERAILMENT_CASES, d.derailment_cases.iter().find(|r| r.id == id), field),
        _ => None,
    }
}
