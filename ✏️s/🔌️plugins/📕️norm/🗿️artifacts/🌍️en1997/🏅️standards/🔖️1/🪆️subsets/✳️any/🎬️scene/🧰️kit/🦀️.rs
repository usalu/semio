//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::*;
use scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, mul, entity_ref, clause_of, tone_of, polygon3, polygon2, utilisation_words};
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

pub const LAYER_GROUND: &str = "ground";
pub const LAYER_WATER: &str = "water";
pub const LAYER_FOOTINGS: &str = "footings";
pub const LAYER_MECHANISMS: &str = "mechanisms";
pub const LAYER_WALLS: &str = "walls";
pub const LAYER_SLOPES: &str = "slopes";
pub const LAYER_PILES: &str = "piles";
pub const LAYER_UPLIFT: &str = "uplift";
pub const LAYER_SETTLEMENT: &str = "settlement";
pub const LAYER_LOADS: &str = "loads";
pub const LAYER_CASES: &str = "cases";
pub const LAYER_ANNOTATIONS: &str = "annotations";

pub const STAGE_LAYERS: [(&str, &str, &str); 6] = [
    ("stage0", "Consolidation stage 1", "Konsolidationsstufe 1"),
    ("stage1", "Consolidation stage 2", "Konsolidationsstufe 2"),
    ("stage2", "Consolidation stage 3", "Konsolidationsstufe 3"),
    ("stage3", "Consolidation stage 4", "Konsolidationsstufe 4"),
    ("stage4", "Consolidation stage 5", "Konsolidationsstufe 5"),
    ("stage5", "Consolidation stage 6", "Konsolidationsstufe 6"),
];

const LAYERS: [(&str, &str, &str); 12] = [
    (LAYER_GROUND, "Ground strata", "Baugrundschichten"),
    (LAYER_WATER, "Groundwater", "Grundwasser"),
    (LAYER_FOOTINGS, "Spread footings", "Flächengründungen"),
    (LAYER_MECHANISMS, "Failure mechanisms", "Bruchmechanismen"),
    (LAYER_WALLS, "Retaining walls", "Stützbauwerke"),
    (LAYER_SLOPES, "Slopes", "Böschungen"),
    (LAYER_PILES, "Piles", "Pfähle"),
    (LAYER_UPLIFT, "Uplift and heave", "Auftrieb und Hebung"),
    (LAYER_SETTLEMENT, "Settlement", "Setzung"),
    (LAYER_LOADS, "Loads", "Lasten"),
    (LAYER_CASES, "Design cases", "Bemessungsfälle"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
];

/// 🏷️ One entity kind of the EN 1997 outliner, interaction domain and insert palette.
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

/// 🗂️ The single registry of EN 1997 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("layer", "layers", "Soil layer", "Bodenschicht", "layers", EntityKind::Solid),
    kind("footing", "footings", "Spread footing", "Flächengründung", "square", EntityKind::Solid),
    kind("pile", "piles", "Pile", "Pfahl", "arrow-down", EntityKind::Group),
    kind("retainingWall", "retainingWalls", "Retaining wall", "Stützwand", "separator-vertical", EntityKind::Solid),
    kind("slope", "slopes", "Slope", "Böschung", "triangle", EntityKind::Solid),
    kind("upliftCase", "upliftCases", "Uplift case", "Auftriebsfall", "droplets", EntityKind::Plate),
    kind("embeddedWall", "embeddedWalls", "Embedded wall", "Eingespannte Wand", "separator-vertical", EntityKind::Solid),
    kind("anchor", "anchors", "Ground anchor", "Verpressanker", "move-down-right", EntityKind::Group),
    kind("pileGroup", "pileGroups", "Pile group", "Pfahlgruppe", "grip", EntityKind::Group),
    kind("labTest", "labTests", "Laboratory series", "Laborversuchsreihe", "flask-conical", EntityKind::Solid),
    kind("layerWater", "layerWaters", "Layer water regime", "Wasserverhältnisse der Schicht", "waves", EntityKind::Surface),
    kind("investigation", "investigations", "Ground investigation", "Baugrunderkundung", "drill", EntityKind::Solid),
    kind("characteristicBasis", "characteristicBases", "Characteristic value basis", "Grundlage des charakteristischen Werts", "scale", EntityKind::Solid),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
    kind("case", "cases", "Design case", "Bemessungsfall", "clipboard-check", EntityKind::Solid),
];

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1997Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    let d = document;
    let found: [(&str, bool); 15] = [
        ("layers", d.layers.iter().any(|e| e.id == base)),
        ("footings", d.footings.iter().any(|e| e.id == base)),
        ("piles", d.piles.iter().any(|e| e.id == base)),
        ("retainingWalls", d.retaining_walls.iter().any(|e| e.id == base)),
        ("slopes", d.slopes.iter().any(|e| e.id == base)),
        ("upliftCases", d.uplift_cases.iter().any(|e| e.id == base)),
        ("embeddedWalls", d.embedded_walls.iter().any(|e| e.id == base)),
        ("anchors", d.anchors.iter().any(|e| e.id == base)),
        ("pileGroups", d.pile_groups.iter().any(|e| e.id == base)),
        ("labTests", d.lab_tests.iter().any(|e| e.id == base)),
        ("layerWaters", d.layer_waters.iter().any(|e| e.id == base)),
        ("investigations", d.investigations.iter().any(|e| e.id == base)),
        ("characteristicBases", d.characteristic_bases.iter().any(|e| e.id == base)),
        ("placements", d.placements.iter().any(|e| e.id == base)),
        ("cases", d.cases.iter().any(|e| e.id == base)),
    ];
    found.iter().find(|(_, hit)| *hit).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1997Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let d = document;
    let mut out: BTreeSet<String> = BTreeSet::new();
    let ground = || d.layers.iter().map(|l| l.id.clone());
    if d.footings.iter().any(|e| e.id == base) || d.piles.iter().any(|e| e.id == base) {
        out.extend(ground());
    }
    if d.footings.iter().any(|e| e.id == base) || d.investigations.iter().any(|e| e.id == base) {
        out.extend(d.investigations.iter().map(|i| i.id.clone()));
    }
    if d.investigations.iter().any(|e| e.id == base) {
        out.extend(ground());
    }
    if let Some(slope) = d.slopes.iter().find(|e| e.id == base) {
        out.insert(slope.governing_layer_id.clone());
    }
    if let Some(anchor) = d.anchors.iter().find(|e| e.id == base) {
        out.insert(anchor.wall_id.clone());
    }
    if let Some(group) = d.pile_groups.iter().find(|e| e.id == base) {
        out.insert(group.pile_id.clone());
    }
    if d.lab_tests.iter().any(|e| e.id == base) {
        out.extend(ground());
    }
    if let Some(basis) = d.characteristic_bases.iter().find(|e| e.id == base) {
        out.insert(basis.layer_id.clone());
    }
    if let Some(water) = d.layer_waters.iter().find(|e| e.id == base) {
        out.insert(water.layer_id.clone());
    }
    if d.embedded_walls.iter().any(|e| e.id == base) {
        out.extend(d.anchors.iter().filter(|a| a.wall_id == base).map(|a| a.id.clone()));
    }
    if d.piles.iter().any(|e| e.id == base) {
        out.extend(d.pile_groups.iter().filter(|g| g.pile_id == base).map(|g| g.id.clone()));
    }
    if d.layers.iter().any(|e| e.id == base) {
        out.extend(d.lab_tests.iter().filter(|t| t.layer_id == base).map(|t| t.id.clone()));
        out.extend(d.investigations.iter().map(|i| i.id.clone()));
        out.extend(d.layer_waters.iter().filter(|w| w.layer_id == base).map(|w| w.id.clone()));
        out.extend(d.characteristic_bases.iter().filter(|b| b.layer_id == base).map(|b| b.id.clone()));
        out.extend(d.footings.iter().map(|f| f.id.clone()));
        out.extend(d.slopes.iter().filter(|s| s.governing_layer_id == base).map(|s| s.id.clone()));
    }
    out.remove(base);
    out.into_iter().collect()
}

pub fn soil(class: &str) -> MaterialRole {
    MaterialRole::Soil { class: if class.trim().is_empty() { "soil".to_string() } else { class.to_string() } }
}

pub fn load_case(id: &str, kind: LoadKind) -> LoadCaseRef {
    LoadCaseRef { id: id.to_string(), kind, psi: None }
}

pub fn force(value: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, value)
}

/// 🧱️ Everything one scene entity needs before sealing.
pub struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub collection: &'static str,
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
}

impl Spec {
    pub fn new(kind: EntityKind, id: impl Into<String>, collection: &'static str, geometry: Geometry, role: MaterialRole, name: LocalizedCopy, layer: &'static str) -> Self {
        Self { kind, id: id.into(), collection, geometry, role, binding: None, detail: name.clone(), name, layer, handles: Vec::new(), opacity: 1.0, channel: None, hatch: None }
    }

    pub fn bound(mut self, binding: Option<UtilisationBinding>) -> Self {
        self.binding = binding;
        self
    }

    pub fn detail(mut self, en: String, de: String) -> Self {
        self.detail = LocalizedCopy::new(en, de);
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

    pub fn hatched(mut self, hatch: Hatch) -> Self {
        self.hatch = Some(hatch);
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
            entity: match self.collection.strip_prefix('@') {
                Some(path) => EntityRef::new(self.kind, &self.id, path),
                None => entity_ref(self.kind, &self.id, self.collection),
            },
            geometry: self.geometry,
            appearance: Appearance { role: self.role, channel, style: if failing { StyleState::Error } else { StyleState::Normal }, opacity: self.opacity, hatch: self.hatch.or(failing.then_some(Hatch::Diagonal)), deformation: None },
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

    /// 📥️ Pushes a scene entity built outside the spec builder (the case forms of the projection kit) into the current owner.
    pub fn push_entity(&mut self, entity: SceneEntity, points: &[Vec3]) {
        if !self.active {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(entity);
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

    pub fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: self.legends, layers: Vec::new(), tracks: self.tracks, owners: self.owners, bounds }
    }
}

/// 🧭️ The reference of a document entity of `collection`, with the scene kind of its registry row; the pseudo entities of the `units` collection are groups.
fn owner_ref(collection: &str, id: &str) -> EntityRef {
    let kind = ENTITY_KINDS.iter().find(|k| k.collection == collection).map_or(EntityKind::Group, |k| k.scene_kind);
    EntityRef::new(kind, id, format!("{collection}[id={id}]"))
}

/// 🧩️ The pseudo entities that own what belongs to no document row: the groundwater table and the settlement field over all footings.
pub const UNIT_WATER: &str = "unit-groundwater";
pub const UNIT_SETTLEMENT: &str = "unit-settlement";

/// 🗂️ The ids of every drawn collection of the document, in outliner order; characteristic bases are outliner and palette rows that steer the ground profile diagrams, not entities of their own.
fn rows_of(d: &model::En1997Snapshot) -> [(&'static str, Vec<&str>); 14] {
    [
        ("layers", d.layers.iter().map(|e| e.id.as_str()).collect()),
        ("footings", d.footings.iter().map(|e| e.id.as_str()).collect()),
        ("piles", d.piles.iter().map(|e| e.id.as_str()).collect()),
        ("retainingWalls", d.retaining_walls.iter().map(|e| e.id.as_str()).collect()),
        ("slopes", d.slopes.iter().map(|e| e.id.as_str()).collect()),
        ("upliftCases", d.uplift_cases.iter().map(|e| e.id.as_str()).collect()),
        ("embeddedWalls", d.embedded_walls.iter().map(|e| e.id.as_str()).collect()),
        ("anchors", d.anchors.iter().map(|e| e.id.as_str()).collect()),
        ("pileGroups", d.pile_groups.iter().map(|e| e.id.as_str()).collect()),
        ("labTests", d.lab_tests.iter().map(|e| e.id.as_str()).collect()),
        ("layerWaters", d.layer_waters.iter().map(|e| e.id.as_str()).collect()),
        ("investigations", d.investigations.iter().map(|e| e.id.as_str()).collect()),
        ("characteristicBases", d.characteristic_bases.iter().map(|e| e.id.as_str()).collect()),
        ("cases", d.cases.iter().filter(|e| super::cases::declared(e)).map(|e| e.id.as_str()).collect()),
    ]
}

/// 🆔️ Every scene entity owner in document order: the rows of every collection, then the groundwater and settlement units.
pub fn entities_of(document: &model::En1997Snapshot) -> Vec<EntityRef> {
    let rows = rows_of(document).into_iter().flat_map(|(collection, ids)| ids.into_iter().map(move |id| owner_ref(collection, id)));
    rows.chain([owner_ref("units", UNIT_WATER), owner_ref("units", UNIT_SETTLEMENT)]).collect()
}

/// 🔗️ The rows a unit reads: the settlement field reads every footing.
pub fn unit_dependencies(document: &model::En1997Snapshot, id: &str) -> Vec<String> {
    if id == UNIT_SETTLEMENT { document.footings.iter().map(|f| f.id.clone()).collect() } else { Vec::new() }
}

/// 📍️ What an entity's projection derives from other entities: its pose (default slots follow the row index), the site bounds the soil layers, waters and the groundwater plane span, and the layer top (the depth of all layers above).
pub fn layout_stamps(document: &model::En1997Snapshot, layout: &Layout) -> std::collections::BTreeMap<String, String> {
    let bounds = format!("{:?}|{:?}|{}", layout.min, layout.max, layout.depth);
    let mut cursor = 0.0_f64;
    let layers = document.layers.iter().map(|l| {
        let top = cursor;
        cursor = l.depth_bottom.max(top + 1e-3);
        (l.id.clone(), format!("{top}|{bounds}"))
    });
    let posed = rows_of(document).into_iter().skip(1).flat_map(|(_, ids)| ids).map(|id| (id.to_owned(), format!("{:?}", layout.pose(id))));
    let waters = document.layer_waters.iter().map(|w| (w.id.clone(), bounds.clone()));
    let bases = document.characteristic_bases.iter().enumerate().map(|(i, b)| (b.id.clone(), format!("{bounds}|{i}")));
    let logs = document.lab_tests.iter().map(|t| (t.id.clone(), format!("{:?}|{}", layout.pose(&t.id), document.investigation_depth)));
    let poses = document.footings.iter().map(|f| format!("{:?}", layout.pose(&f.id))).collect::<Vec<_>>().join("|");
    layers.chain(posed).chain(logs).chain(waters).chain(bases).chain([(UNIT_WATER.to_owned(), bounds.clone()), (UNIT_SETTLEMENT.to_owned(), poses)]).collect()
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments, the site bounds and the description.
pub fn finish(scene: &mut StructuralScene, layout: &Layout, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let fields = std::mem::take(&mut scene.legends);
    let site = layout.aabb();
    let content = !scene.entities.is_empty() || !scene.annotations.is_empty();
    scene.bounds = if content { Aabb::of_points(&[scene.bounds.min.clone(), scene.bounds.max.clone(), site.min.clone(), site.max.clone()]).unwrap_or(site) } else { site };
    let table: Vec<(&str, &str, &str)> = LAYERS.iter().chain(STAGE_LAYERS.iter()).copied().collect();
    scene.layers = projection_kit::legends::layers(&table, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Materials", "Werkstoffe"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(fields);
    scene.description = description;
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Soil { .. } => ("Ground", "Baugrund"),
            MaterialRole::Concrete => ("Concrete", "Beton"),
            MaterialRole::Support => ("Supports", "Lager"),
            MaterialRole::Load { .. } => ("Loads", "Lasten"),
            _ => return None,
        };
        let key = match role {
            MaterialRole::Load { .. } => MaterialRole::Load { kind: LoadKind::Other },
            MaterialRole::Soil { .. } => MaterialRole::Soil { class: "ground".to_string() },
            other => other.clone(),
        };
        Some((label, key))
    })
}

// BEGIN check-owners (generated by check_owners.py)
/// 🧭️ The collection whose entity draws each check family, from the `entity` of the spec-trace item that lists the family; longer prefixes first.
pub const CHECK_OWNERS: &[(&str, &str)] = &[
    ("en1997.6.5.bearing.undrained.", "footings"),
    ("en1997.2.4.2.investigation.", "footings"),
    ("en1997.pile-04.deflection.", "piles"),
    ("en1997.7.6.2.compression.", "piles"),
    ("en1997.2.characteristic.", "cases"),
    ("en1997.pile-07.buckling.", "piles"),
    ("en1997.9.earthPressure.", "retainingWalls"),
    ("en1997.6.6.settlement.", "footings"),
    ("en1997.6.5.3.sliding.", "footings"),
    ("en1997.7.6.3.tension.", "piles"),
    ("en1997.9.overturning.", "retainingWalls"),
    ("en1997.6.5.bearing.", "footings"),
    ("en1997.6.6.angular.", "footings"),
    ("en1997.6.6.elastic.", "footings"),
    ("en1997.layer.model", "layers"),
    ("en1997.11.bishop.", "slopes"),
    ("en1997.9.sliding.", "retainingWalls"),
    ("en1997.6.6.tilt.", "footings"),
    ("en1997.slope-02.", "slopes"),
    ("en1997.slope-03.", "slopes"),
    ("en1997.slope-04.", "slopes"),
    ("en1997.geo-f03.", "footings"),
    ("en1997.pile-05.", "piles"),
    ("en1997.pile-06.", "pileGroups"),
    ("en1997.pile-08.", "piles"),
    ("en1997.pile-09.", "piles"),
    ("en1997.10.hyd.", "upliftCases"),
    ("en1997.10.upl.", "upliftCases"),
    ("en1997.anc-01.", "anchors"),
    ("en1997.anc-02.", "anchors"),
    ("en1997.anc-03.", "anchors"),
    ("en1997.anc-04.", "anchors"),
    ("en1997.dew-01.", "layerWaters"),
    ("en1997.dew-02.", "layerWaters"),
    ("en1997.dew-03.", "embeddedWalls"),
    ("en1997.hyd-02.", "embeddedWalls"),
    ("en1997.hyd-03.", "embeddedWalls"),
    ("en1997.sls-02.", "labTests"),
    ("en1997.sls-03.", "labTests"),
    ("en1997.gi-01.", "footings"),
    ("en1997.gi-02.", "cases"),
    ("en1997.gi-03.", "cases"),
    ("en1997.gi-04.", "cases"),
    ("en1997.gi-05.", "cases"),
    ("en1997.gi-06.", "cases"),
    ("en1997.gi-07.", "cases"),
    ("en1997.gi-08.", "cases"),
    ("en1997.gi-09.", "cases"),
    ("en1997.gi-10.", "labTests"),
    ("en1997.gi-11.", "labTests"),
    ("en1997.gi-12.", "labTests"),
    ("en1997.gi-13.", "labTests"),
    ("en1997.gi-14.", "cases"),
    ("en1997.gi-15.", "cases"),
    ("en1997.gi-16.", "cases"),
    ("en1997.gi-17.", "labTests"),
    ("en1997.rw-02.", "retainingWalls"),
    ("en1997.rw-03.", "embeddedWalls"),
    ("en1997.rw-04.", "retainingWalls"),
    ("en1997.rw-05.", "retainingWalls"),
    ("en1997.rw-07.", "embeddedWalls"),
    ("en1997.rw-08.", "retainingWalls"),
    ("en1997.rw-09.", "retainingWalls"),
    ("en1997.rw-10.", "retainingWalls"),
    ("en1997.g-02.", "cases"),
];

/// 🧭️ The collection that owns the check `check_id` (longest family prefix), `None` for a check outside the spec trace.
pub fn check_owner(check_id: &str) -> Option<&'static str> {
    CHECK_OWNERS.iter().find(|(prefix, _)| check_id.starts_with(prefix)).map(|(_, collection)| *collection)
}
// END check-owners

/// 🧭️ The document row that owns `check`: the row its subject names, else the only row of the collection [`check_owner`] files the family under; `None` for a document-level check.
pub fn owner_entity(document: &model::En1997Snapshot, check: &norm::CheckResult) -> Option<String> {
    let subject = base_id(&check.subject.entity_id);
    if !subject.is_empty() && entity_kind_of(document, subject).is_some() {
        return Some(subject.to_owned());
    }
    let collection = check_owner(&check.id)?;
    let rows = rows_of(document).into_iter().find(|(name, _)| *name == collection)?.1;
    (rows.len() == 1).then(|| rows[0].to_owned())
}

/// 📈️ The check families a dedicated diagram draws (profiles, factors, pressures, curves, sections and plans of their entity).
pub const DRAWN_FAMILIES: &[&str] = &[
    "en1997.layer.model", "en1997.g-02.", "en1997.2.characteristic.", "en1997.2.4.2.investigation.", "en1997.gi-01.", "en1997.gi-02.", "en1997.gi-03.", "en1997.gi-04.", "en1997.gi-05.", "en1997.gi-06.", "en1997.gi-07.", "en1997.gi-08.", "en1997.gi-09.", "en1997.gi-14.", "en1997.gi-15.", "en1997.gi-16.",
    "en1997.6.5.bearing.", "en1997.geo-f03.", "en1997.6.5.3.sliding.", "en1997.6.6.settlement.", "en1997.6.6.elastic.", "en1997.6.6.angular.", "en1997.6.6.tilt.",
    "en1997.9.earthPressure.", "en1997.9.overturning.", "en1997.9.sliding.", "en1997.rw-02.", "en1997.rw-04.", "en1997.rw-05.", "en1997.rw-10.",
    "en1997.rw-03.", "en1997.rw-07.", "en1997.hyd-02.", "en1997.hyd-03.", "en1997.10.upl.", "en1997.10.hyd.",
    "en1997.11.bishop.", "en1997.anc-01.", "en1997.anc-02.", "en1997.anc-03.", "en1997.anc-04.",
    "en1997.sls-02.", "en1997.sls-03.", "en1997.gi-11.", "en1997.gi-12.",
    "en1997.pile-04.deflection.", "en1997.pile-06.", "en1997.pile-09.",
];

/// 📊️ The check families without a dedicated diagram: the outcome diagram of their entity draws every check of them as a utilisation bar against the limit line.
pub const OUTCOME_FAMILIES: &[&str] = &[
    "en1997.7.6.2.compression.", "en1997.7.6.3.tension.", "en1997.pile-05.", "en1997.pile-07.buckling.", "en1997.pile-08.",
    "en1997.rw-08.", "en1997.rw-09.", "en1997.dew-01.", "en1997.dew-02.", "en1997.dew-03.",
    "en1997.slope-02.", "en1997.slope-03.", "en1997.slope-04.", "en1997.gi-10.", "en1997.gi-13.", "en1997.gi-17.",
];

/// 📄️ The check families that belong to the document as a whole (design situation, governing combination, integrity and link rules, missing-entity notes); they select the global badge.
pub const DOCUMENT_CHECKS: &[&str] = &["en1997.design.", "en1997.governing.", "en1997.integrity.", "en1997.gk.", "en1997.link.", "en1997.footing.", "en1997.gi-01.points", "en1997.pile.na", "en1997.slope.na", "en1997.upl.na", "en1997.wall.na"];

/// 🧭️ Where a check is drawn: on one document row, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The route of `check`: its owner row, else the document when it has no subject and sits in a document-level family; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::En1997Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    owner_entity(document, check).map(CheckRoute::Entity).or_else(|| (check.subject.entity_id.is_empty() && DOCUMENT_CHECKS.iter().any(|p| check.id.starts_with(p))).then_some(CheckRoute::Document))
}

/// 🎯️ The row to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`].
pub fn focus_of_outcome(document: &model::En1997Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📶️ The governing binding among the checks owned by the row `id` whose id starts with any of `prefixes` (every owned check when empty).
pub fn binding_of_owner(document: &model::En1997Snapshot, report: &norm::CheckReport, id: &str, prefixes: &[&str]) -> Option<UtilisationBinding> {
    projection_kit::binding_where(report, |check| owner_entity(document, check).as_deref() == Some(id) && (prefixes.is_empty() || prefixes.iter().any(|p| check.id.starts_with(p))))
}

impl Ctx<'_> {
    /// 📶️ The utilisation binding of the row `id` from the checks it owns, see [`binding_of_owner`].
    pub fn binding(&self, id: &str, prefixes: &[&str]) -> Option<UtilisationBinding> {
        binding_of_owner(self.input.document, self.input.report, id, prefixes)
    }
}
