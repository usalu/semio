//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends, bilingual number formatting and the entity-kind registry.

use super::*;
use scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, add, mul, clause_of, tone_of, polyline, polygon3, polygon2, binding_where, binding_of_subject as binding, binding_of_subjects as binding_of};
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

pub const LAYER_ENVELOPE: &str = "envelope";
pub const LAYER_WIND: &str = "wind";
pub const LAYER_SNOW: &str = "snow";
pub const LAYER_SNOW_ALT: &str = "snowAlternatives";
pub const LAYER_IMPOSED: &str = "imposed";
pub const LAYER_SELF_WEIGHT: &str = "selfWeight";
pub const LAYER_THERMAL: &str = "thermal";
pub const LAYER_FIRE: &str = "fire";
pub const LAYER_ACCIDENTAL: &str = "accidental";
pub const LAYER_TRAFFIC: &str = "traffic";
pub const LAYER_TRAFFIC_ALT: &str = "trafficAlternatives";
pub const LAYER_TRAVEL: &str = "trafficTravel";
pub const LAYER_CRANE: &str = "crane";
pub const LAYER_RAIL: &str = "rail";
pub const LAYER_SILO: &str = "silo";
pub const LAYER_STAGES: &str = "constructionStages";
pub const LAYER_ROOF_SURFACES: &str = "roofSurfaces";
pub const LAYER_WIND_BRIDGES: &str = "windBridges";
pub const LAYER_DYNAMIC: &str = "dynamicElements";
pub const LAYER_IMPACT: &str = "impactActions";
pub const LAYER_EXPLOSION: &str = "internalExplosions";
pub const LAYER_THERMAL_SECTIONS: &str = "thermalSections";
pub const LAYER_HELICOPTER: &str = "helicopterLandings";
pub const LAYER_TIES: &str = "tieRuns";
pub const LAYER_CONSTRUCTION_LOADS: &str = "constructionLoads";
pub const LAYER_FALLING: &str = "fallingLoads";
pub const LAYER_IMPERFECTIONS: &str = "executionImperfections";
pub const LAYER_DERAILMENT: &str = "derailmentCases";
pub const LAYER_HOISTS: &str = "craneHoists";
pub const LAYER_FOOTBRIDGES: &str = "footbridges";
pub const LAYER_RAIL_DYNAMICS: &str = "railDynamics";
pub const LAYER_DISPERSALS: &str = "loadDispersals";
pub const LAYER_BULK_SOLIDS: &str = "bulkSolids";
pub const LAYER_MACHINES: &str = "rotatingMachines";
pub const LAYER_FATIGUE: &str = "fatigueModels";
pub const LAYER_COLLISIONS: &str = "supportCollisions";
pub const LAYER_LOCAL_FIRES: &str = "localFires";
pub const LAYER_SNOW_GUARDS: &str = "snowGuards";
pub const LAYER_RAIL_DERAILMENTS: &str = "railDerailments";
pub const LAYER_RAIL_GROUPS: &str = "railLoadGroups";
pub const LAYER_DUST: &str = "dustExplosions";
/// 🧱 Distance [m] of the first row of the footbridge, rail span, dispersal, bulk solid and machine galleries to its parent pose and between two gallery rows.
pub const GALLERY_START: f64 = 14.0;
pub const GALLERY_STEP: f64 = 6.0;
/// 🧱 Side of the bulk solid and machine plates [m] and the track width of a rail span in the scene.
pub const PLATE_SIDE: f64 = 3.0;
pub const TRACK_WIDTH: f64 = 1.5;
/// 🛤️ Standard track gauge [m] and the axle spacing of the LM71 [m] as drawn in the rail rows of the scene and anchored by their grips.
pub const TRACK_GAUGE: f64 = 1.435;
pub const LM71_AXLE_SPACING: f64 = 1.6;
/// 🧱 Row `index` of the gallery of `collection` along -y of its parent pose: the rows of the collections that share a pose follow one another.
pub fn gallery_y(d: &En1991Snapshot, collection: &str, index: usize) -> f64 {
    let before = match collection {
        "railDynamics" => d.footbridges.len(),
        "loadDispersals" => d.footbridges.len() + d.rail_dynamics.len(),
        "fatigueModels" => d.footbridges.len() + d.rail_dynamics.len() + d.load_dispersals.len(),
        "railDerailments" => d.footbridges.len() + d.rail_dynamics.len() + d.load_dispersals.len() + d.fatigue_models.len(),
        "railLoadGroups" => d.footbridges.len() + d.rail_dynamics.len() + d.load_dispersals.len() + d.fatigue_models.len() + d.rail_derailments.len(),
        "rotatingMachines" => d.bulk_solids.len(),
        "dustExplosions" => d.bulk_solids.len() + d.rotating_machines.len(),
        _ => 0,
    };
    -(GALLERY_START + GALLERY_STEP * (before + index) as f64)
}
/// 🪝 Distance [m] between the hoist rows across the runway in the scene and in their grips.
pub const HOIST_SPACING: f64 = 1.5;
pub const LAYER_CASES: &str = "cases";
pub const LAYER_ANNOTATIONS: &str = "annotations";
pub const TANDEM_STOPS: [(f64, &str, &str, &str); 5] = [
    (0.5, "traffic-position-1", "Tandem at mid-span", "Tandem in Feldmitte"),
    (0.1, "traffic-position-2", "Tandem at 10 % of the span", "Tandem bei 10 % der Spannweite"),
    (0.3, "traffic-position-3", "Tandem at 30 % of the span", "Tandem bei 30 % der Spannweite"),
    (0.7, "traffic-position-4", "Tandem at 70 % of the span", "Tandem bei 70 % der Spannweite"),
    (0.9, "traffic-position-5", "Tandem at 90 % of the span", "Tandem bei 90 % der Spannweite"),
];

const LAYERS: [(&str, &str, &str, bool); 48] = [
    (LAYER_ENVELOPE, "Building envelope", "Gebäudehülle", true),
    (LAYER_WIND, "Wind", "Wind", true),
    (LAYER_SNOW, "Snow", "Schnee", true),
    (LAYER_SNOW_ALT, "Snow load arrangements", "Schneelastanordnungen", false),
    (LAYER_IMPOSED, "Imposed floor loads", "Nutzlasten", true),
    (LAYER_SELF_WEIGHT, "Self-weight", "Eigengewicht", true),
    (LAYER_THERMAL, "Thermal actions", "Temperatureinwirkungen", true),
    (LAYER_FIRE, "Fire", "Brand", true),
    (LAYER_ACCIDENTAL, "Accidental actions", "Außergewöhnliche Einwirkungen", true),
    (LAYER_TRAFFIC, "Bridge traffic", "Brückenverkehr", true),
    (LAYER_TRAFFIC_ALT, "Alternative load models LM2 to LM4", "Alternative Lastmodelle LM2 bis LM4", false),
    (LAYER_TRAVEL, "LM1 tandem travel (mid-span moment)", "LM1-Tandemfahrt (Feldmoment)", false),
    (TANDEM_STOPS[0].1, TANDEM_STOPS[0].2, TANDEM_STOPS[0].3, true),
    (TANDEM_STOPS[1].1, TANDEM_STOPS[1].2, TANDEM_STOPS[1].3, true),
    (TANDEM_STOPS[2].1, TANDEM_STOPS[2].2, TANDEM_STOPS[2].3, true),
    (TANDEM_STOPS[3].1, TANDEM_STOPS[3].2, TANDEM_STOPS[3].3, true),
    (TANDEM_STOPS[4].1, TANDEM_STOPS[4].2, TANDEM_STOPS[4].3, true),
    (LAYER_CRANE, "Crane runway", "Kranbahn", true),
    (LAYER_RAIL, "Rail load models LM71, SW/0 and SW/2 (unverified)", "Eisenbahn-Lastmodelle LM71, SW/0 und SW/2 (ungeprüft)", false),
    (LAYER_SILO, "Silo and tank", "Silo und Tank", true),
    (LAYER_STAGES, "Construction stages", "Bauzustände", true),
    (LAYER_ROOF_SURFACES, "Roof surfaces", "Dachflächen", true),
    (LAYER_WIND_BRIDGES, "Wind on bridge decks (unverified coefficients)", "Wind auf Brückenüberbauten (ungeprüfte Beiwerte)", true),
    (LAYER_DYNAMIC, "Slender elements: c_s c_d and vortex shedding", "Schlanke Bauteile: c_s c_d und Wirbelerregung", true),
    (LAYER_IMPACT, "Impacts of vehicles, ships and forklifts", "Anprall von Fahrzeugen, Schiffen und Gabelstaplern", true),
    (LAYER_EXPLOSION, "Internal gas explosions", "Innere Gasexplosionen", true),
    (LAYER_THERMAL_SECTIONS, "Temperature profiles of bridge sections", "Temperaturprofile von Brückenquerschnitten", true),
    (LAYER_HELICOPTER, "Helicopter hard landings", "Hubschrauber-Hartlandungen", true),
    (LAYER_TIES, "Horizontal ties (robustness)", "Horizontale Zugglieder (Robustheit)", true),
    (LAYER_CONSTRUCTION_LOADS, "Construction loads of the stages", "Bauzustandslasten der Bauzustände", true),
    (LAYER_FALLING, "Falling loads during execution", "Herabfallende Lasten im Bauzustand", true),
    (LAYER_IMPERFECTIONS, "Execution imperfections", "Ausführungsimperfektionen", true),
    (LAYER_DERAILMENT, "Derailment (forces entered, unverified)", "Entgleisung (Kräfte eingegeben, ungeprüft)", true),
    (LAYER_HOISTS, "Crane hoists (rated and test loads)", "Kranhubwerke (Hub- und Prüflasten)", true),
    (LAYER_FOOTBRIDGES, "Footbridges (deck and uniform load)", "Fußgängerbrücken (Deck und Gleichlast)", true),
    (LAYER_RAIL_DYNAMICS, "Rail spans (determinant length)", "Eisenbahnfelder (maßgebende Länge)", true),
    (LAYER_DISPERSALS, "Wheel load dispersals (footprints)", "Radlastausbreitungen (Aufstandsflächen)", true),
    (LAYER_BULK_SOLIDS, "Bulk solids (pressure ratio)", "Schüttgüter (Horizontallastverhältnis)", true),
    (LAYER_MACHINES, "Rotating machines (unbalance force)", "Rotierende Maschinen (Unwuchtkraft)", true),
    (LAYER_FATIGUE, "Road fatigue load models (FLM1, FLM3)", "Straßenverkehrs-Ermüdungslastmodelle (FLM1, FLM3)", true),
    (LAYER_COLLISIONS, "Vehicle collisions on supports and soffits", "Fahrzeuganprall an Stützen und Unterseiten", true),
    (LAYER_LOCAL_FIRES, "Localised fires (flame, plume, heat flux)", "Lokale Brände (Flamme, Plume, Wärmestrom)", true),
    (LAYER_SNOW_GUARDS, "Snow guards (line force)", "Schneefanggitter (Linienkraft)", true),
    (LAYER_RAIL_DERAILMENTS, "Rail derailments (line load, unverified)", "Eisenbahn-Entgleisungen (Linienlast, ungeprüft)", true),
    (LAYER_RAIL_GROUPS, "Rail load groups (Table 6.11 components)", "Eisenbahn-Lastgruppen (Komponenten der Tabelle 6.11)", true),
    (LAYER_DUST, "Dust explosions (vent and pressure)", "Staubexplosionen (Entlastung und Druck)", true),
    (LAYER_CASES, "Design cases", "Bemessungsfälle", true),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen", true),
];

pub use super::registry::{binding_of_owner, declared as case_declared, entity_kind_of, family_of, focus_of_outcome, global, inert_reference, is_outcome, owner_entity, owner_of, report_owner, route_of, scalar_owner, scene_report, CheckRoute, EntityKindSpec, Owner, DOCUMENT_CHECKS, DRAWN_FAMILIES, ENTITY_KINDS, OUTCOME_FAMILIES};

/// 🗣️ Number with `prec` decimals in the decimal separator of the language.
pub fn num(value: f64, prec: usize, de: bool) -> String {
    let text = format!("{value:.prec$}");
    if de { text.replace('.', ",") } else { text }
}

/// 🗣️ Both languages of a text from one closure that receives `true` for German.
pub fn bi(text: impl Fn(bool) -> String) -> LocalizedCopy {
    LocalizedCopy::new(text(false), text(true))
}

pub fn unit(a: &Vec3) -> Vec3 {
    let l = a.length();
    if l < 1e-12 { Vec3::Z } else { mul(a, 1.0 / l) }
}

pub fn entity_ref(kind: EntityKind, id: &str, collection: &str) -> EntityRef {
    match collection.strip_prefix('@') {
        Some(path) => EntityRef::new(kind, id, path),
        None => EntityRef::new(kind, id, format!("{collection}[id={}]", base_id(id))),
    }
}

/// 📶️ The governing check whose check id starts with any of `prefixes`, whatever its subject (singleton structures have no row id).
pub fn binding_prefixed(report: &norm::CheckReport, prefixes: &[&str]) -> Option<UtilisationBinding> {
    binding_where(report, |c| prefixes.iter().any(|p| c.id.starts_with(p)))
}

pub fn load(kind: LoadKind) -> MaterialRole { MaterialRole::Load { kind } }

pub fn case(id: &str, kind: LoadKind) -> LoadCaseRef { LoadCaseRef { id: id.to_string(), kind, psi: None } }

pub fn pressure(value: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, value) }

pub fn force(value: f64) -> Quantity { Quantity::new(QuantityKind::Force, value) }

pub fn centroid(points: &[Vec3]) -> Vec3 {
    let n = points.len().max(1) as f64;
    v3(points.iter().map(|p| p.x).sum::<f64>() / n, points.iter().map(|p| p.y).sum::<f64>() / n, points.iter().map(|p| p.z).sum::<f64>() / n)
}

pub fn rect2(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<[f64; 2]> { vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1]] }

fn utilisation_words(binding: &Option<UtilisationBinding>) -> (String, String) {
    binding.as_ref().map_or_else(Default::default, |b| (format!(", utilisation {:.2}", b.ratio), format!(", Ausnutzung {}", num(b.ratio, 2, true))))
}

/// 🧱️ Everything one scene entity needs before sealing.
pub struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub collection: String,
    pub geometry: Geometry,
    pub role: MaterialRole,
    pub binding: Option<UtilisationBinding>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub handles: Vec<String>,
    pub opacity: f64,
    pub channel: Option<ColourChannel>,
    pub hatch: Option<Hatch>,
    pub deformation: Option<DeformationOverlay>,
}

impl Spec {
    pub fn new(kind: EntityKind, id: impl Into<String>, collection: impl Into<String>, geometry: Geometry, role: MaterialRole, name: LocalizedCopy, layer: &'static str) -> Self {
        Self { kind, id: id.into(), collection: collection.into(), geometry, role, binding: None, detail: name.clone(), name, layer, handles: Vec::new(), opacity: 1.0, channel: None, hatch: None, deformation: None }
    }

    pub fn bound(mut self, binding: Option<UtilisationBinding>) -> Self {
        self.binding = binding;
        self
    }

    pub fn detail(mut self, detail: LocalizedCopy) -> Self {
        self.detail = detail;
        self
    }

    pub fn handles(mut self, handles: Vec<String>) -> Self {
        self.handles = handles;
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn deformed(mut self, overlay: Option<DeformationOverlay>) -> Self {
        self.deformation = overlay;
        self
    }

    pub fn channel(mut self, channel: ColourChannel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        let channel = self.channel.unwrap_or(if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role });
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, &self.collection),
            geometry: self.geometry,
            appearance: Appearance { role: self.role, channel, style: if failing { StyleState::Error } else { StyleState::Normal }, opacity: self.opacity, hatch: self.hatch.or(failing.then_some(Hatch::Diagonal)), deformation: self.deformation },
            utilisation: self.binding,
            label: Some(self.name.clone()),
            accessible_name: self.name,
            accessible_detail: LocalizedCopy::new(format!("{}{more_en}", self.detail.en), format!("{}{more_de}", self.detail.de)),
            layer: self.layer.to_string(),
            pick: PickPolicy::Whole,
            handles: self.handles,
            content_hash: String::new(),
        }
        .sealed()
    }
}

impl Ctx<'_> {
    /// 📶️ The utilisation binding of the entity `id` from the checks it owns, see [`binding_of_owner`].
    pub fn binding(&self, id: &str, prefixes: &[&str]) -> Option<UtilisationBinding> {
        binding_of_owner(self.input.document, self.input.report, id, prefixes)
    }
}

/// 🗂️ A document row by collection and index, or a global entity.
#[derive(Clone, Copy)]
enum Which {
    Row(&'static str, usize),
    Global(&'static str),
}

struct Rows<'a> {
    document: &'a model::En1991Snapshot,
    all: Vec<(&'static str, Vec<&'a str>)>,
}

impl<'a> Rows<'a> {
    fn of(document: &'a model::En1991Snapshot) -> Self {
        Self { document, all: rows_of(document) }
    }

    fn ids(&self, collection: &str) -> &[&'a str] {
        self.all.iter().find(|(name, _)| *name == collection).map_or(&[][..], |(_, ids)| ids.as_slice())
    }

    fn which(&self, id: &str) -> Option<Which> {
        global::ALL.iter().copied().find(|g| *g == id).map(Which::Global).or_else(|| self.all.iter().find_map(|(collection, ids)| ids.iter().position(|row| *row == id).map(|i| Which::Row(*collection, i))))
    }
}

/// 🌬️ Whether wind face `index` is the first of its zone letter, the face that owns the zone entity `wind-zone-<letter>`.
pub fn first_of_zone(d: &model::En1991Snapshot, index: usize) -> bool {
    d.wind_faces.iter().position(|f| f.zone.eq_ignore_ascii_case(&d.wind_faces[index].zone)) == Some(index)
}

/// 🏠 What the wind zones, the envelope label and the thermal anchor read of the first roof: its form, pitch and parapet. Stamped instead of declared as a dependency, so the roof row's own dependencies (snow site) do not leak into them.
fn roof_form(d: &model::En1991Snapshot) -> String {
    d.roofs.first().map_or_else(String::new, |r| format!("{}|{:?}|{}|{:?}", r.roof_type, r.pitch_deg, r.has_parapet, r.parapet_height))
}

fn dependencies_of(rows: &Rows<'_>, which: Which) -> Vec<String> {
    let d = rows.document;
    let mut out: BTreeSet<&str> = BTreeSet::new();
    match which {
        Which::Row(collection, index) => match collection {
            "floors" => out.extend([global::ANNEX, global::BUILDING]),
            "selfWeightElements" => out.extend([global::ANNEX]),
            "roofs" => out.extend([global::BUILDING, global::SNOW_SITE, global::ANNEX]),
            "windFaces" if first_of_zone(d, index) => {
                out.extend([global::WIND_SITE, global::ANNEX, global::BUILDING]);
                out.extend(d.wind_faces.iter().enumerate().filter(|(i, f)| *i != index && f.zone.eq_ignore_ascii_case(&d.wind_faces[index].zone)).map(|(_, f)| f.id.as_str()));
            }
            "accidentalCases" | "constructionStages" | "roofSurfaces" | "impactActions" | "internalExplosions" | "helicopterLandings" | "tieRuns" | "fallingLoads" | "executionImperfections" => out.extend([global::BUILDING]),
            "constructionLoads" => {
                out.extend([global::BUILDING]);
                out.extend(rows.ids("constructionStages").iter().copied());
            }
            "derailmentCases" => out.extend([global::BUILDING]),
            "craneHoists" => out.extend([global::CRANE]),
            "footbridges" | "railDynamics" | "loadDispersals" | "supportCollisions" => out.extend([global::BRIDGE]),
            "fatigueModels" | "railDerailments" | "railLoadGroups" => out.extend([global::BRIDGE, global::ANNEX]),
            "dustExplosions" => out.extend([global::SILO, global::ANNEX]),
            "localFires" => out.extend([global::BUILDING]),
            "snowGuards" => {
                out.extend([global::BUILDING, global::SNOW_SITE, global::ANNEX]);
                out.extend(rows.ids("roofs").iter().copied());
            }
            "bulkSolids" | "rotatingMachines" => out.extend([global::SILO]),
            "bridgeLanes" => out.extend([global::BRIDGE, global::ANNEX]),
            "specialVehicles" | "thermalSections" => out.extend([global::BRIDGE]),
            "windBridges" => out.extend([global::BRIDGE, global::WIND_SITE, global::ANNEX]),
            "dynamicElements" => out.extend([global::BUILDING, global::WIND_SITE, global::ANNEX]),
            _ => {}
        },
        Which::Global(name) => match name {
            global::ENVELOPE => out.extend([global::BUILDING, global::WIND_SITE, global::SNOW_SITE]),
            global::WIND_ZONES => out.extend([global::WIND_SITE, global::ANNEX, global::BUILDING]),
            global::WIND_QP => {
                out.extend([global::WIND_SITE, global::ANNEX, global::BUILDING]);
                out.extend(rows.ids("windFaces").iter().copied());
            }
            global::BRIDGE_DECK => {
                out.extend([global::BRIDGE, global::ANNEX]);
                out.extend(rows.ids("bridgeLanes").iter().chain(rows.ids("specialVehicles")).copied());
            }
            global::THERMAL => {
                out.extend([global::BRIDGE, global::BUILDING]);
                out.extend(rows.ids("thermalSections").iter().copied());
            }
            global::FIRE => out.extend([global::ANNEX, global::BUILDING]),
            global::CRANE | global::SILO => out.extend([global::ANNEX]),
            global::SELF_WEIGHT => out.extend(rows.ids("selfWeightElements").iter().copied()),
            global::DEFLECTION => {
                out.extend([global::BUILDING, global::WIND_SITE, global::SNOW_SITE, global::ANNEX]);
                out.extend(["roofs", "windFaces", "roofSurfaces", "windBridges", "dynamicElements"].iter().flat_map(|c| rows.ids(c)).copied());
            }
            _ => {}
        },
    }
    out.into_iter().map(str::to_owned).collect()
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`: the row entity or global entity named by `id` (a derived scene id resolves to its base). The graph is acyclic: leaf scalar groups, then rows, then the aggregates that read many rows.
pub fn entity_dependencies(document: &model::En1991Snapshot, id: &str) -> Vec<String> {
    let rows = Rows::of(document);
    rows.which(base_id(id)).map_or_else(Vec::new, |which| dependencies_of(&rows, which))
}

fn extra(d: &model::En1991Snapshot, layout: &Layout, slots: &[(f64, f64)], which: Which) -> String {
    match which {
        Which::Row(collection, i) => match collection {
            "floors" | "thermalSections" => i.to_string(),
            "windFaces" if first_of_zone(d, i) => roof_form(d),
            "selfWeightElements" => format!("{:?}|{:?}", d.self_weight_elements[..i].iter().map(|e| e.thickness.max(0.01)).sum::<f64>(), layout.self_weight),
            "roofs" => format!("{i}|{:?}", layout.roof(i)),
            "accidentalCases" => format!("{i}/{}", d.accidental_cases.len()),
            "constructionStages" => format!("{:?}", slots[i]),
            "roofSurfaces" => format!("{:?}", super::collections::surface_span(d, i)),
            "dynamicElements" => format!("{:?}", super::wave3::element_origin(d, i)),
            "internalExplosions" => format!("{:?}", super::wave3::room_origin(d, i)),
            "cases" => d.cases.iter().filter(|c| case_declared(c)).nth(i).and_then(|c| layout.case(&c.id)).map_or_else(String::new, |pose| format!("{pose:?}")),
            _ => String::new(),
        },
        Which::Global(global::ENVELOPE | global::WIND_QP | global::THERMAL) => roof_form(d),
        Which::Global(global::SELF_WEIGHT) => format!("{:?}", layout.self_weight),
        Which::Global(global::CRANE) => format!("{:?}", layout.crane),
        Which::Global(global::SILO) => format!("{:?}", layout.silo),
        Which::Global(global::WIND_ZONES) => {
            let mut letters: Vec<String> = d.wind_faces.iter().map(|f| f.zone.to_ascii_uppercase()).collect();
            letters.sort();
            letters.dedup();
            format!("{}|{}", letters.join(","), roof_form(d))
        }
        Which::Global(_) => String::new(),
    }
}

/// 📍️ What an entity's projection derives from its position among other rows and from poses other than its own row: the index, running totals, derived poses and the ids of the rows it depends on, so adding or removing a row re-fingerprints exactly the entities that list it.
pub fn stamps(document: &model::En1991Snapshot, layout: &Layout) -> std::collections::BTreeMap<String, String> {
    let rows = Rows::of(document);
    let slots = super::collections::stage_slots(document);
    let mut out = std::collections::BTreeMap::new();
    let mut put = |id: &str, which: Which| {
        out.insert(id.to_owned(), format!("{}|{}", extra(document, layout, &slots, which), dependencies_of(&rows, which).join(",")));
    };
    for (collection, ids) in &rows.all {
        for (i, id) in ids.iter().enumerate() {
            put(id, Which::Row(*collection, i));
        }
    }
    for name in global::ALL {
        put(name, Which::Global(name));
    }
    out
}
/// 🗂️ The ids of every collection of the document, in outliner order.
fn rows_of(d: &model::En1991Snapshot) -> Vec<(&'static str, Vec<&str>)> {
    vec![
        ("floors", d.floors.iter().map(|e| e.id.as_str()).collect()),
        ("selfWeightElements", d.self_weight_elements.iter().map(|e| e.id.as_str()).collect()),
        ("roofs", d.roofs.iter().map(|e| e.id.as_str()).collect()),
        ("windFaces", d.wind_faces.iter().map(|e| e.id.as_str()).collect()),
        ("accidentalCases", d.accidental_cases.iter().map(|e| e.id.as_str()).collect()),
        ("bridgeLanes", d.bridge_lanes.iter().map(|e| e.id.as_str()).collect()),
        ("constructionStages", d.construction_stages.iter().map(|e| e.id.as_str()).collect()),
        ("roofSurfaces", d.roof_surfaces.iter().map(|e| e.id.as_str()).collect()),
        ("specialVehicles", d.special_vehicles.iter().map(|e| e.id.as_str()).collect()),
        ("windBridges", d.wind_bridges.iter().map(|e| e.id.as_str()).collect()),
        ("dynamicElements", d.dynamic_elements.iter().map(|e| e.id.as_str()).collect()),
        ("impactActions", d.impact_actions.iter().map(|e| e.id.as_str()).collect()),
        ("internalExplosions", d.internal_explosions.iter().map(|e| e.id.as_str()).collect()),
        ("thermalSections", d.thermal_sections.iter().map(|e| e.id.as_str()).collect()),
        ("helicopterLandings", d.helicopter_landings.iter().map(|e| e.id.as_str()).collect()),
        ("tieRuns", d.tie_runs.iter().map(|e| e.id.as_str()).collect()),
        ("constructionLoads", d.construction_loads.iter().map(|e| e.id.as_str()).collect()),
        ("fallingLoads", d.falling_loads.iter().map(|e| e.id.as_str()).collect()),
        ("executionImperfections", d.execution_imperfections.iter().map(|e| e.id.as_str()).collect()),
        ("derailmentCases", d.derailment_cases.iter().map(|e| e.id.as_str()).collect()),
        ("craneHoists", d.crane_hoists.iter().map(|e| e.id.as_str()).collect()),
        ("footbridges", d.footbridges.iter().map(|e| e.id.as_str()).collect()),
        ("railDynamics", d.rail_dynamics.iter().map(|e| e.id.as_str()).collect()),
        ("loadDispersals", d.load_dispersals.iter().map(|e| e.id.as_str()).collect()),
        ("bulkSolids", d.bulk_solids.iter().map(|e| e.id.as_str()).collect()),
        ("rotatingMachines", d.rotating_machines.iter().map(|e| e.id.as_str()).collect()),
        ("fatigueModels", d.fatigue_models.iter().map(|e| e.id.as_str()).collect()),
        ("supportCollisions", d.support_collisions.iter().map(|e| e.id.as_str()).collect()),
        ("localFires", d.local_fires.iter().map(|e| e.id.as_str()).collect()),
        ("snowGuards", d.snow_guards.iter().map(|e| e.id.as_str()).collect()),
        ("railDerailments", d.rail_derailments.iter().map(|e| e.id.as_str()).collect()),
        ("railLoadGroups", d.rail_load_groups.iter().map(|e| e.id.as_str()).collect()),
        ("dustExplosions", d.dust_explosions.iter().map(|e| e.id.as_str()).collect()),
        ("cases", d.cases.iter().filter(|e| case_declared(e)).map(|e| e.id.as_str()).collect()),
    ]
}

/// 🆔️ Every scene entity owner in scene order: the global entities, then the document rows of every collection.
pub fn entities_of(document: &model::En1991Snapshot) -> Vec<EntityRef> {
    let rows = rows_of(document);
    let kinds = rows.iter().flat_map(|(collection, ids)| ENTITY_KINDS.iter().find(|k| k.collection == *collection).into_iter().flat_map(move |k| ids.iter().map(move |id| entity_ref(k.scene_kind, id, collection))));
    global::ALL.iter().map(|id| entity_ref(EntityKind::Group, id, "@site")).chain(kinds).collect()
}

/// 📥️ Append-only scene accumulator of one projection part: the entities, owned annotations and tracks, legends and bounding points of the entities the part wants.
#[derive(Default)]
pub struct Out {
    pub entities: Vec<SceneEntity>,
    pub annotations: Vec<Annotation>,
    pub tracks: Vec<ParameterTrack>,
    pub legends: Vec<Legend>,
    owners: Vec<Ownership>,
    points: Vec<Vec3>,
    scope: Scope,
    owner: Option<EntityRef>,
    active: bool,
}

#[derive(Clone, Default)]
enum Scope {
    #[default]
    All,
    Entity(String),
    Global,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one entity, or the global rest, at a time.
    pub fn scoped(part: Part<'_>) -> Self {
        let scope = match part {
            Part::All => Scope::All,
            Part::Entity(id) => Scope::Entity(id.to_owned()),
            Part::Global => Scope::Global,
        };
        Self { scope, ..Self::default() }
    }

    fn part(&self) -> Part<'_> {
        match &self.scope {
            Scope::All => Part::All,
            Scope::Entity(id) => Part::Entity(id),
            Scope::Global => Part::Global,
        }
    }

    /// 🎯️ Whether this part wants the entity `id`, so a module can skip work no wanted entity needs.
    pub fn wants(&self, id: &str) -> bool {
        self.part().wants(id)
    }

    /// 🎯️ Starts the entity `owner`: everything pushed until the next call belongs to it; returns whether this part wants it, so the caller can skip the geometry.
    pub fn own(&mut self, owner: &str) -> bool {
        self.active = self.part().wants(owner);
        if self.active {
            self.owner = Some(entity_ref(EntityKind::Group, owner, "@site"));
        }
        self.active
    }

    /// 📥️ Pushes an entity built outside the spec builder (the case forms of the projection kit) into the entity that was started with [`Out::own`].
    pub fn push_entity(&mut self, entity: SceneEntity, points: &[Vec3]) {
        if !self.active {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(entity);
    }

    pub fn push(&mut self, spec: Spec, points: &[Vec3]) {
        if !self.active {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(spec.build());
    }

    fn link(&mut self, kind: OwnedKind, id: &str) -> bool {
        if !self.active {
            return false;
        }
        if let Some(owner) = &self.owner {
            self.owners.push(Ownership { kind, id: id.to_owned(), owner: owner.clone() });
        }
        true
    }

    pub fn annotate(&mut self, annotation: Annotation, anchor: &Vec3) {
        if self.link(OwnedKind::Annotation, annotation.id()) {
            self.points.push(anchor.clone());
            self.annotations.push(annotation);
        }
    }

    pub fn track(&mut self, track: ParameterTrack) {
        if self.link(OwnedKind::Track, &track.id) {
            self.tracks.push(track);
        }
    }

    pub fn label(&mut self, id: String, anchor: Vec3, text: LocalizedCopy, tone: Tone) {
        self.annotate(Annotation::Label { id, anchor: anchor.clone(), text, tone }, &anchor);
    }

    pub fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: self.legends, layers: Vec::new(), tracks: self.tracks, owners: self.owners, bounds }
    }
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments, and the description.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let fields = std::mem::take(&mut scene.legends);
    scene.layers = LAYERS.iter().filter(|(id, ..)| used.contains(id)).map(|(id, en, de, visible)| Layer { id: id.to_string(), name: loc(en, de), visible: *visible, locked: false }).collect();
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Materials and actions", "Werkstoffe und Einwirkungen"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(fields);
    if scene.entities.iter().any(|e| e.layer == LAYER_SELF_WEIGHT) {
        scene.legends.push(super::gravity::weight_legend());
    }
    scene.tracks.extend(super::dynamics::traffic(&scene.entities));
    scene.tracks.extend(super::fields::travel::track(&scene.entities));
    scene.description = description;
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Concrete => ("Concrete", "Beton"),
            MaterialRole::Steel => ("Steel", "Stahl"),
            MaterialRole::Neutral => ("Envelope", "Hülle"),
            MaterialRole::Support => ("Supports", "Lager"),
            MaterialRole::Load { .. } => ("Actions", "Einwirkungen"),
            _ => return None,
        };
        let key = match role {
            MaterialRole::Load { .. } => MaterialRole::Load { kind: LoadKind::Other },
            other => other.clone(),
        };
        Some((label, key))
    })
}
