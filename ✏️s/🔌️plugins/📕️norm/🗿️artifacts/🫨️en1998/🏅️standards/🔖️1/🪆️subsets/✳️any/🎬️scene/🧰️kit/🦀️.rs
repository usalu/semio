//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::*;
use scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, add, mul, unit, along, member_frame, world_frame, polyline, clause_of, tone_of, utilisation_words, binding_where};
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

/// 🧱️ Thickness of a storey slab [m].
pub const SLAB_THICKNESS: f64 = 0.25;
pub const LAYER_STOREYS: &str = "storeys";
pub const LAYER_COLUMNS: &str = "columns";
pub const LAYER_JOINTS: &str = "joints";
pub const LAYER_MASSES: &str = "masses";
pub const LAYER_FORCES: &str = "forces";
pub const LAYER_DRIFT: &str = "drift";
pub const LAYER_VESSELS: &str = "vessels";
pub const LAYER_TOWERS: &str = "towers";
pub const LAYER_GROUND: &str = "ground";
pub const LAYER_BRIDGES: &str = "bridges";
pub const LAYER_SEISMIC_JOINTS: &str = "seismicJoints";
pub const LAYER_MODE_X1: &str = "modeX1";
pub const LAYER_MODE_X2: &str = "modeX2";
pub const LAYER_MODE_X3: &str = "modeX3";
pub const LAYER_MODE_Y1: &str = "modeY1";
pub const LAYER_MODE_Y2: &str = "modeY2";
pub const LAYER_MODE_Y3: &str = "modeY3";
pub const MODE_LAYERS: [(&str, &str, &str); 6] = [
    (LAYER_MODE_X1, "Mode 1 (x)", "Eigenform 1 (x)"),
    (LAYER_MODE_X2, "Mode 2 (x)", "Eigenform 2 (x)"),
    (LAYER_MODE_X3, "Mode 3 (x)", "Eigenform 3 (x)"),
    (LAYER_MODE_Y1, "Mode 1 (y)", "Eigenform 1 (y)"),
    (LAYER_MODE_Y2, "Mode 2 (y)", "Eigenform 2 (y)"),
    (LAYER_MODE_Y3, "Mode 3 (y)", "Eigenform 3 (y)"),
];
/// 🦶 Thickness of the footing body below the ground level.
pub const FOOTING_DEPTH: f64 = 0.6;
/// 🌉 Width of the bridge deck and the pier positions along it, shared by the scene and its grips.
pub const DECK_WIDTH: f64 = 10.0;
pub const PIER_X: [(&str, f64); 2] = [("a", 8.0), ("b", 22.0)];
/// 🏛 Default pier height and section side of a pier without entered data.
pub const PIER_HEIGHT: f64 = 8.0;
pub const PIER_SIZE: f64 = 1.2;
pub const LAYER_DRIFT_MAP: &str = "driftMap";
pub const LAYER_ASSESSED: &str = "assessedMembers";
pub const LAYER_REGIONS: &str = "criticalRegions";
pub const LAYER_ISOLATION: &str = "isolation";
pub const LAYER_ANNOTATIONS: &str = "annotations";
pub const LAYER_DIAPHRAGMS: &str = "diaphragms";
pub const LAYER_DIAPHRAGM_FLOW: &str = "diaphragmFlow";
pub const LAYER_BASE_SHEAR: &str = "baseShear";
pub const LAYER_SPECTRUM_SURFACE: &str = "spectrumSurface";
pub const LAYER_OVERTURNING: &str = "overturning";
pub const LAYER_ACCELEROGRAMS: &str = "accelerograms";

/// 🌊️ The mode layer showing the n-th tower mode: the first three modes use the x-direction mode layers.
pub fn tower_mode_layer(n: u32) -> &'static str {
    match n {
        1 => LAYER_MODE_X1,
        2 => LAYER_MODE_X2,
        _ => LAYER_MODE_X3,
    }
}

const LAYERS: [(&str, &str, &str); 28] = [
    (LAYER_STOREYS, "Storeys", "Geschosse"),
    (LAYER_COLUMNS, "Columns", "Stützen"),
    (LAYER_JOINTS, "Joints", "Knoten"),
    (LAYER_MASSES, "Masses and centres", "Massen und Mittelpunkte"),
    (LAYER_FORCES, "Lateral forces", "Horizontalkräfte"),
    (LAYER_DRIFT, "Storey drift", "Stockwerksverschiebung"),
    (LAYER_VESSELS, "Silos and tanks", "Silos und Tanks"),
    (LAYER_TOWERS, "Towers and chimneys", "Türme und Schornsteine"),
    (LAYER_GROUND, "Foundations and walls", "Gründungen und Wände"),
    (LAYER_BRIDGES, "Bridges", "Brücken"),
    (LAYER_SEISMIC_JOINTS, "Seismic joints", "Erdbebenfugen"),
    (LAYER_MODE_X1, "Mode 1 (x)", "Eigenform 1 (x)"),
    (LAYER_MODE_X2, "Mode 2 (x)", "Eigenform 2 (x)"),
    (LAYER_MODE_X3, "Mode 3 (x)", "Eigenform 3 (x)"),
    (LAYER_MODE_Y1, "Mode 1 (y)", "Eigenform 1 (y)"),
    (LAYER_MODE_Y2, "Mode 2 (y)", "Eigenform 2 (y)"),
    (LAYER_MODE_Y3, "Mode 3 (y)", "Eigenform 3 (y)"),
    (LAYER_DRIFT_MAP, "Storey drift heat map", "Stockwerksverschiebung als Heatmap"),
    (LAYER_ASSESSED, "Assessed members", "Bewertete Bauteile"),
    (LAYER_REGIONS, "Critical regions", "Kritische Bereiche"),
    (LAYER_ISOLATION, "Isolation and foundation springs", "Isolation und Fundamentfedern"),
    (LAYER_DIAPHRAGMS, "Diaphragms", "Scheiben"),
    (LAYER_DIAPHRAGM_FLOW, "Diaphragm force flow", "Kraftfluss in Scheiben"),
    (LAYER_BASE_SHEAR, "Base shear distribution", "Gesamtquerkraftverteilung"),
    (LAYER_SPECTRUM_SURFACE, "Response spectrum surface", "Antwortspektrum-Fläche"),
    (LAYER_OVERTURNING, "Overturning stability", "Kippsicherheit"),
    (LAYER_ACCELEROGRAMS, "Accelerograms", "Akzelerogramme"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
];

/// 🏷️ One entity kind of the EN 1998 outliner, interaction domain and insert palette.
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

/// 🏷️ The single registry of EN 1998 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("building", "buildings", "Building", "Gebäude", "building", EntityKind::Group),
    kind("bridge", "bridges", "Bridge", "Brücke", "git-commit", EntityKind::Plate),
    kind("assessment", "assessments", "Assessment", "Bewertung", "clipboard-check", EntityKind::Group),
    kind("criticalRegion", "criticalRegions", "Critical region", "Kritischer Bereich", "ruler", EntityKind::Member),
    kind("silo", "silos", "Silo", "Silo", "cylinder", EntityKind::Shell),
    kind("tank", "tanks", "Tank", "Tank", "database", EntityKind::Shell),
    kind("foundation", "foundations", "Foundation", "Gründung", "square", EntityKind::Solid),
    kind("retainingWall", "retainingWalls", "Retaining wall", "Stützwand", "brick-wall", EntityKind::Solid),
    kind("tower", "towers", "Tower", "Turm", "radio-tower", EntityKind::Member),
    kind("soilLayer", "soilLayers", "Soil layer", "Bodenschicht", "layers", EntityKind::Solid),
    kind("diaphragm", "diaphragms", "Diaphragm", "Scheibe", "panel-top", EntityKind::Plate),
    kind("accelerogram", "accelerograms", "Accelerogram", "Akzelerogramm", "audio-waveform", EntityKind::Surface),
    kind("pier", "piers", "Pier", "Pfeiler", "land-plot", EntityKind::Member),
    kind("bearing", "bearings", "Bearing", "Lager", "circle-dot", EntityKind::Support),
    kind("joint", "joints", "Joint", "Knoten", "link", EntityKind::Solid),
    kind("tieBeam", "tieBeams", "Tie beam", "Fundamentbalken", "minus", EntityKind::Member),
    kind("slope", "slopes", "Slope", "Böschung", "mountain", EntityKind::Solid),
    kind("isolationSystem", "isolationSystems", "Isolation system", "Isolationssystem", "move-horizontal", EntityKind::Support),
    kind("pierBase", "pierBases", "Pier base", "Pfeilerfuß", "arrow-down-to-line", EntityKind::Support),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
];

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1998Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    let d = document;
    let found: [(&str, bool); 20] = [
        ("buildings", d.buildings.iter().any(|e| e.id == base)),
        ("bridges", d.bridges.iter().any(|e| e.id == base)),
        ("assessments", d.assessments.iter().any(|e| e.id == base)),
        ("criticalRegions", d.critical_regions.iter().any(|e| e.id == base)),
        ("silos", d.silos.iter().any(|e| e.id == base)),
        ("tanks", d.tanks.iter().any(|e| e.id == base)),
        ("foundations", d.foundations.iter().any(|e| e.id == base)),
        ("retainingWalls", d.retaining_walls.iter().any(|e| e.id == base)),
        ("towers", d.towers.iter().any(|e| e.id == base)),
        ("soilLayers", d.soil_layers.iter().any(|e| e.id == base)),
        ("diaphragms", d.diaphragms.iter().any(|e| e.id == base)),
        ("accelerograms", d.accelerograms.iter().any(|e| e.id == base)),
        ("piers", d.piers.iter().any(|e| e.id == base)),
        ("bearings", d.bearings.iter().any(|e| e.id == base)),
        ("joints", d.joints.iter().any(|e| e.id == base)),
        ("tieBeams", d.tie_beams.iter().any(|e| e.id == base)),
        ("slopes", d.slopes.iter().any(|e| e.id == base)),
        ("isolationSystems", d.isolation_systems.iter().any(|e| e.id == base)),
        ("pierBases", d.pier_bases.iter().any(|e| e.id == base)),
        ("placements", d.placements.iter().any(|e| e.id == base)),
    ];
    found.iter().find(|(_, hit)| *hit).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1998Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let d = document;
    let mut out: BTreeSet<String> = BTreeSet::new();
    out.extend(d.foundations.iter().filter(|e| e.id == base).map(|e| e.supported_building_id.clone()));
    out.extend(d.assessments.iter().filter(|e| e.id == base).map(|e| e.supported_building_id.clone()));
    out.extend(d.foundations.iter().filter(|e| e.supported_building_id == base).map(|e| e.id.clone()));
    out.extend(d.assessments.iter().filter(|e| e.supported_building_id == base).map(|e| e.id.clone()));
    out.extend(d.critical_regions.iter().filter(|e| e.id == base).map(|e| e.building_id.clone()));
    out.extend(d.critical_regions.iter().filter(|e| e.building_id == base).map(|e| e.id.clone()));
    out.extend(d.isolation_systems.iter().filter(|e| e.id == base).flat_map(|e| [e.bridge_id.clone(), e.building_id.clone()]).filter(|id| !id.is_empty()));
    out.extend(d.isolation_systems.iter().filter(|e| e.bridge_id == base || e.building_id == base).map(|e| e.id.clone()));
    out.extend(d.pier_bases.iter().filter(|e| e.id == base).flat_map(|e| [e.bridge_id.clone(), e.pier_id.clone()]));
    out.extend(d.pier_bases.iter().filter(|e| e.bridge_id == base || e.pier_id == base).map(|e| e.id.clone()));
    out.extend(d.soil_layers.iter().filter(|e| e.id == base).map(|e| e.foundation_id.clone()));
    out.extend(d.soil_layers.iter().filter(|e| e.foundation_id == base).map(|e| e.id.clone()));
    out.extend(d.diaphragms.iter().filter(|e| e.id == base).map(|e| e.building_id.clone()));
    out.extend(d.accelerograms.iter().filter(|e| e.id == base).map(|e| e.building_id.clone()));
    out.extend(d.joints.iter().filter(|e| e.id == base).map(|e| e.building_id.clone()));
    out.extend(d.joints.iter().filter(|e| e.building_id == base).map(|e| e.id.clone()));
    for t in d.tie_beams.iter().filter(|e| e.id == base) {
        out.extend([t.foundation_a_id.clone(), t.foundation_b_id.clone()]);
    }
    out.extend(d.tie_beams.iter().filter(|t| t.foundation_a_id == base || t.foundation_b_id == base).map(|t| t.id.clone()));
    let bridge = d.piers.iter().filter(|e| e.id == base).map(|e| e.bridge_id.as_str()).chain(d.bearings.iter().filter(|e| e.id == base).map(|e| e.bridge_id.as_str())).chain(d.bridges.iter().filter(|e| e.id == base).map(|e| e.id.as_str())).next();
    if let Some(bridge) = bridge {
        out.insert(bridge.to_owned());
        out.extend(d.piers.iter().filter(|e| e.bridge_id == bridge).map(|e| e.id.clone()));
        out.extend(d.bearings.iter().filter(|e| e.bridge_id == bridge).map(|e| e.id.clone()));
    }
    if d.buildings.iter().any(|e| e.id == base) {
        out.extend(d.buildings.iter().filter(|e| e.id != base).map(|e| e.id.clone()));
    }
    out.remove(base);
    out.into_iter().collect()
}

/// 🏛️ Fixed end of the assessed member glyph of the `index`-th assessment of a building: beside the plan, 3 m apart per member.
pub fn assessed_origin(building_origin: &Vec3, building: &model::En1998Building, index: usize) -> Vec3 {
    v3(building_origin.x + building.plan_width_m + 3.0, building_origin.y + 1.0 + 3.0 * index as f64, building_origin.z + 1.0)
}

/// 🔩 Fixed end of the member glyph of the `index`-th critical region of a building: south of the plan, 2.5 m apart per region.
pub fn region_origin(building_origin: &Vec3, index: usize) -> Vec3 {
    v3(building_origin.x, building_origin.y - 3.0 - 2.5 * index as f64, building_origin.z + 1.0)
}

/// 🔩 Whether a critical region has the section and the length the scene draws.
pub fn region_drawn(region: &model::En1998CriticalRegion) -> bool {
    region.section_width_m > 0.0 && region.section_depth_m > 0.0 && region.clear_length_m > 0.0
}

/// 🦶 Centre of the footing of a foundation at ground level: under the centre of its building, or south of the first building while it is not linked.
pub fn foundation_centre(document: &model::En1998Snapshot, layout: &super::Layout, foundation: &model::En1998Foundation) -> Vec3 {
    document.buildings.iter().find(|b| b.id == foundation.supported_building_id).map_or_else(|| v3(0.0, -10.0, 0.0), |b| {
        let o = layout.origin(&b.id);
        v3(o.x + b.plan_width_m / 2.0, o.y + b.plan_length_m / 2.0, o.z)
    })
}

pub fn rect_ring(w: f64, l: f64) -> Polygon2 {
    Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(w, 0.0), Vec2::new(w, l), Vec2::new(0.0, l)])
}

pub fn entity_ref(kind: EntityKind, id: &str, path: &str) -> EntityRef {
    EntityRef::new(kind, id, path)
}

pub fn path_of(collection: &str, id: &str) -> String {
    format!("{collection}[id={}]", base_id(id))
}

pub fn binding_prefix(report: &norm::CheckReport, prefix: &str) -> Option<UtilisationBinding> {
    binding_where(report, |c| c.id.starts_with(prefix))
}

pub fn binding_exact(report: &norm::CheckReport, id: &str) -> Option<UtilisationBinding> {
    binding_where(report, |c| c.id == id)
}

/// 🧮️ The demand a check computed, in its own unit (N for forces).
pub fn computed_of(report: &norm::CheckReport, id: &str) -> Option<f64> {
    report.checks.iter().find(|c| c.id == id).map(|c| c.computed.value).filter(|v| v.is_finite())
}

pub fn load_case(id: &str) -> LoadCaseRef {
    LoadCaseRef { id: id.to_string(), kind: LoadKind::Seismic, psi: None }
}

pub fn box_solid(origin: Vec3, size: Vec3) -> Geometry {
    Geometry::Solid(SolidGeometry { shape: projection_kit::solid::box_from_corner(&world_frame(origin), size) })
}

/// 🧱️ Box whose corner sits at `origin` and whose length runs along the plan direction `yaw` [rad] from the plan x axis, thickness to its left, height up.
pub fn yaw_box_solid(origin: Vec3, yaw: f64, size: Vec3) -> Geometry {
    let (s, c) = yaw.sin_cos();
    Geometry::Solid(SolidGeometry { shape: projection_kit::solid::box_from_corner(&Frame { origin, x: v3(c, s, 0.0), y: v3(-s, c, 0.0), z: Vec3::Z }, size) })
}

/// 🧱️ Everything one scene entity needs before sealing.
pub struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub path: String,
    pub geometry: Geometry,
    pub role: MaterialRole,
    pub binding: Option<UtilisationBinding>,
    pub style: Option<StyleState>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub handles: Vec<String>,
    pub deformation: Option<DeformationOverlay>,
    pub opacity: f64,
    pub channel: Option<ColourChannel>,
}

impl Spec {
    pub fn new(kind: EntityKind, id: impl Into<String>, path: impl Into<String>, geometry: Geometry, role: MaterialRole, name: LocalizedCopy, layer: &'static str) -> Self {
        Self { kind, id: id.into(), path: path.into(), geometry, role, binding: None, style: None, detail: name.clone(), name, layer, handles: Vec::new(), deformation: None, opacity: 1.0, channel: None }
    }

    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail) || self.style == Some(StyleState::Error);
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, &self.path),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: self.channel.unwrap_or(if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role }),
                style: if failing { StyleState::Error } else { self.style.unwrap_or(StyleState::Normal) },
                opacity: self.opacity,
                hatch: failing.then_some(Hatch::Diagonal),
                deformation: self.deformation,
            },
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

/// 📥️ Append-only scene accumulator of one projection part: entities in document order, owned annotations and tracks, legends and the points that bound the part.
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
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one document entity, or the global rest, at a time.
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

    /// 🎯️ Starts the document entity `id` of `collection`: everything pushed until the next call belongs to it; returns whether this part wants it, so the caller can skip its row.
    pub fn own(&mut self, collection: &str, id: &str) -> bool {
        self.owner = Some(owner_ref(collection, id));
        self.active = self.part().wants(id);
        self.active
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

    pub fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: self.legends, layers: Vec::new(), tracks: self.tracks, owners: self.owners, bounds }
    }
}

/// 🧭️ The reference of a document entity of `collection`, with the scene kind of its registry row.
fn owner_ref(collection: &str, id: &str) -> EntityRef {
    let kind = ENTITY_KINDS.iter().find(|k| k.collection == collection).map_or(EntityKind::Group, |k| k.scene_kind);
    EntityRef::new(kind, id, path_of(collection, id))
}

/// 🗂️ The ids of every drawn collection of the document, in outliner order; placements are outliner and palette rows keyed by the entity they move, not entities of their own.
fn rows_of(d: &model::En1998Snapshot) -> [(&'static str, Vec<&str>); 18] {
    [
        ("buildings", d.buildings.iter().map(|e| e.id.as_str()).collect()),
        ("bridges", d.bridges.iter().map(|e| e.id.as_str()).collect()),
        ("assessments", d.assessments.iter().map(|e| e.id.as_str()).collect()),
        ("silos", d.silos.iter().map(|e| e.id.as_str()).collect()),
        ("tanks", d.tanks.iter().map(|e| e.id.as_str()).collect()),
        ("foundations", d.foundations.iter().map(|e| e.id.as_str()).collect()),
        ("retainingWalls", d.retaining_walls.iter().map(|e| e.id.as_str()).collect()),
        ("towers", d.towers.iter().map(|e| e.id.as_str()).collect()),
        ("soilLayers", d.soil_layers.iter().map(|e| e.id.as_str()).collect()),
        ("diaphragms", d.diaphragms.iter().map(|e| e.id.as_str()).collect()),
        ("accelerograms", d.accelerograms.iter().map(|e| e.id.as_str()).collect()),
        ("piers", d.piers.iter().map(|e| e.id.as_str()).collect()),
        ("bearings", d.bearings.iter().map(|e| e.id.as_str()).collect()),
        ("joints", d.joints.iter().map(|e| e.id.as_str()).collect()),
        ("tieBeams", d.tie_beams.iter().map(|e| e.id.as_str()).collect()),
        ("slopes", d.slopes.iter().map(|e| e.id.as_str()).collect()),
        ("isolationSystems", d.isolation_systems.iter().map(|e| e.id.as_str()).collect()),
        ("pierBases", d.pier_bases.iter().map(|e| e.id.as_str()).collect()),
    ]
}

/// 🆔️ Every document entity in outliner order: the canonical order of the scene.
pub fn entities_of(document: &model::En1998Snapshot) -> Vec<EntityRef> {
    rows_of(document).into_iter().flat_map(|(collection, ids)| ids.into_iter().map(move |id| owner_ref(collection, id))).collect()
}

/// 📍️ What an entity's pose derives from other entities: its origin (default slots follow the row index and the widths of the buildings before it) and, for a foundation, the footing centre under its building.
pub fn layout_stamps(document: &model::En1998Snapshot, layout: &super::Layout) -> std::collections::BTreeMap<String, String> {
    let foundations = document.foundations.iter().map(|f| (f.id.clone(), format!("{:?}", foundation_centre(document, layout, f))));
    let origins = rows_of(document).into_iter().filter(|(collection, _)| !matches!(*collection, "foundations" | "assessments" | "soilLayers" | "diaphragms" | "accelerograms" | "piers" | "bearings" | "joints" | "tieBeams" | "isolationSystems" | "pierBases")).flat_map(|(_, ids)| ids).map(|id| (id.to_owned(), format!("{:?}", layout.origin(id))));
    foundations.chain(origins).collect()
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments, the description and the mode and phase tracks.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let fields = std::mem::take(&mut scene.legends);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Materials and actions", "Werkstoffe und Einwirkungen"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(fields);
    scene.description = description;
    super::tracks::finish(scene);
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Steel => ("Steel", "Stahl"),
            MaterialRole::Concrete => ("Concrete", "Beton"),
            MaterialRole::Timber => ("Timber", "Holz"),
            MaterialRole::Masonry => ("Masonry", "Mauerwerk"),
            MaterialRole::Support => ("Centres of stiffness and supports", "Steifigkeitsmittelpunkte und Lager"),
            MaterialRole::Load { .. } => ("Seismic masses, centres of mass and forces", "Erdbebenmassen, Massenmittelpunkte und Kräfte"),
            MaterialRole::Soil { .. } => ("Content and ground", "Füllgut und Baugrund"),
            _ => return None,
        };
        let key = match role {
            MaterialRole::Load { .. } => MaterialRole::Load { kind: LoadKind::Seismic },
            MaterialRole::Soil { .. } => MaterialRole::Soil { class: "content".into() },
            other => other.clone(),
        };
        Some((label, key))
    })
}

/// 📄️ The check families that belong to the document as a whole (site, integrity, the shared ground and profile checks, the not-applicable notes); they select the global badge.
pub const DOCUMENT_CHECKS: &[&str] = &["en1998.site.", "en1998.integrity.", "en1998.cancelled", "en1998.3.1.", "en1998.3.2.", "en1998.4.3.", "en1998.5.mononobe-okabe", "en1998.5.seismic-slope", "en1998.5.tie-beam", "en1998.5.wall-water"];

/// 🧭️ Where a check is drawn: on one document row, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The id segment a check id names as its owner (`en1998.<part>.<owner>…`).
pub fn owner_segment(check_id: &str) -> Option<&str> {
    check_id.split('.').nth(2)
}

/// 🧭️ The document row that owns `check`: the row its subject names, else the row named by the owner segment of its id; `None` for a document-level check.
pub fn owner_entity(document: &model::En1998Snapshot, check: &norm::CheckResult) -> Option<String> {
    [Some(base_id(&check.subject.entity_id)), owner_segment(&check.id)].into_iter().flatten().find(|id| !id.is_empty() && entity_kind_of(document, id).is_some()).map(str::to_owned)
}

/// 🧭️ The route of `check`: its owner row, else the global badge for a document-level family or a not-applicable note; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::En1998Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    owner_entity(document, check).map(CheckRoute::Entity).or_else(|| (DOCUMENT_CHECKS.iter().any(|p| check.id.starts_with(p)) || check.id.ends_with(".na")).then_some(CheckRoute::Document))
}

/// 🎯️ The row to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`].
pub fn focus_of_outcome(document: &model::En1998Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📶️ The governing binding among the checks owned by the row `id` whose id contains any of `fragments` (every owned check when empty).
pub fn binding_of_owner(document: &model::En1998Snapshot, report: &norm::CheckReport, id: &str, fragments: &[&str]) -> Option<UtilisationBinding> {
    binding_where(report, |check| owner_entity(document, check).as_deref() == Some(id) && (fragments.is_empty() || fragments.iter().any(|f| check.id.contains(f))))
}

/// 📈️ The check families a dedicated physical diagram draws: (id infix, id suffix, diagram id prefix); the diagram id is the prefix plus the owner segment of the check.
pub const DRAWN_FAMILIES: &[(&str, &str, &str)] = &[
    ("", ".pressure", "wall-pressure:"), ("", ".slosh", "wall-pressure:"),
    ("", ".wall", "silo-demand:"), ("", ".anchor", "silo-demand:"),
    (".pier.", ".flexure", "pier-moments:"), (".pier.", ".shear", "pier-capacity:"), (".pier.", ".behaviour", "pier-q:"),
    ("", ".bearing", "bearing-displacement:"), (".bearing.", ".displacement", "bearing-displacement:"),
    (".capacityDesign.joint.", "", "joint-ratios:"),
];

/// 📈️ The diagram id prefix that draws `check_id` and the id of that diagram, `None` for a check the outcome diagram of its entity draws.
pub fn drawn_diagram(check_id: &str) -> Option<String> {
    DRAWN_FAMILIES.iter().find(|(infix, suffix, _)| check_id.contains(infix) && check_id.ends_with(suffix)).and_then(|(_, _, prefix)| owner_segment(check_id).map(|owner| format!("{prefix}{owner}")))
}
