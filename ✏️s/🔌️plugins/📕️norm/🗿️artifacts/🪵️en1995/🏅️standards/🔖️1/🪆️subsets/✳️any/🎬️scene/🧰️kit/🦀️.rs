//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::*;
use super::scene::*;
pub(crate) use scene::projection_kit::{self, base_id, loc, v3, add, mul, unit, along, member_frame, entity_ref, clause_of, tone_of, utilisation_words, polyline, binding_of_subject, worst_of_subject};
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

pub(crate) use entities::{band, binding_of, rect, tinted, worst, Span2, Spec};
pub(crate) use entities::{LAYER_ANNOTATIONS, LAYER_CONNECTIONS, LAYER_FIRE, LAYER_LOADS, LAYER_MEMBERS, LAYER_SUPPORTS, LAYER_ZONES};

const LAYERS: [(&str, &str, &str); 22] = [
    (LAYER_MEMBERS, "Timber members", "Holzbauteile"),
    (LAYER_CONNECTIONS, "Connections", "Verbindungen"),
    (LAYER_FIRE, "Fire: char and zero-strength layer", "Brand: Abbrand und Nullfestigkeitsschicht"),
    ("fire-15", "Char layer after 15 min", "Abbrandschicht nach 15 min"),
    ("fire-30", "Char layer after 30 min", "Abbrandschicht nach 30 min"),
    ("fire-45", "Char layer after 45 min", "Abbrandschicht nach 45 min"),
    ("fire-60", "Char layer after 60 min", "Abbrandschicht nach 60 min"),
    ("fire-75", "Char layer after 75 min", "Abbrandschicht nach 75 min"),
    ("fire-90", "Char layer after 90 min", "Abbrandschicht nach 90 min"),
    ("fire-105", "Char layer after 105 min", "Abbrandschicht nach 105 min"),
    ("fire-120", "Char layer after 120 min", "Abbrandschicht nach 120 min"),
    (LAYER_ZONES, "Spacing and distance zones", "Abstands- und Randzonen"),
    (LAYER_LOADS, "Loads", "Lasten"),
    (LAYER_SUPPORTS, "Supports and bearings", "Lager und Auflager"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
    ("creep-0", "Floor deflection at loading", "Deckendurchbiegung bei Lastaufbringung"),
    ("creep-1", "Floor deflection after 1 year", "Deckendurchbiegung nach 1 Jahr"),
    ("creep-2", "Floor deflection after 2 years", "Deckendurchbiegung nach 2 Jahren"),
    ("creep-5", "Floor deflection after 5 years", "Deckendurchbiegung nach 5 Jahren"),
    ("creep-10", "Floor deflection after 10 years", "Deckendurchbiegung nach 10 Jahren"),
    ("creep-20", "Floor deflection after 20 years", "Deckendurchbiegung nach 20 Jahren"),
    ("creep-50", "Floor deflection after 50 years", "Deckendurchbiegung nach 50 Jahren"),
];

/// 🏷️ One entity kind of the EN 1995 outliner, interaction domain and insert palette.
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

/// 🗃️ The single registry of EN 1995 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("member", "members", "Timber member", "Holzbauteil", "minus", EntityKind::Member),
    kind("connection", "connections", "Connection", "Verbindung", "link", EntityKind::Group),
    kind("case", "cases", "Catalogue case", "Katalogfall", "calculator", EntityKind::Group),
    kind("taper", "tapers", "Tapered or curved member", "Verjüngtes oder gekrümmtes Bauteil", "triangle", EntityKind::Group),
    kind("cutout", "cutouts", "Notch or hole", "Ausklinkung oder Loch", "square-dashed", EntityKind::Group),
    kind("fastener", "fasteners", "Fastener pattern", "Verbindungsmittelbild", "grid-3x3", EntityKind::Group),
    kind("panel", "panels", "CLT panel", "BSP-Platte", "layers", EntityKind::Group),
    kind("floor", "floors", "Timber floor", "Holzdecke", "rectangle-horizontal", EntityKind::Group),
    kind("diaphragm", "diaphragms", "Diaphragm", "Scheibe", "square-split-horizontal", EntityKind::Group),
    kind("truss", "trusses", "Truss", "Fachwerkträger", "triangle-right", EntityKind::Group),
    kind("bracing", "bracings", "Bracing", "Aussteifung", "x", EntityKind::Group),
    kind("deck", "decks", "Stress-laminated deck", "Vorgespanntes Brettstapeldeck", "rows-3", EntityKind::Group),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
];

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1995Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    let found: [(&str, bool); 13] = [
        ("members", document.members.iter().any(|e| e.id == base)),
        ("connections", document.connections.iter().any(|e| e.id == base)),
        ("cases", document.cases.iter().any(|e| e.id == base)),
        ("tapers", document.tapers.iter().any(|e| e.id == base)),
        ("cutouts", document.cutouts.iter().any(|e| e.id == base)),
        ("fasteners", document.fasteners.iter().any(|e| e.id == base)),
        ("panels", document.panels.iter().any(|e| e.id == base)),
        ("floors", document.floors.iter().any(|e| e.id == base)),
        ("diaphragms", document.diaphragms.iter().any(|e| e.id == base)),
        ("trusses", document.trusses.iter().any(|e| e.id == base)),
        ("bracings", document.bracings.iter().any(|e| e.id == base)),
        ("decks", document.decks.iter().any(|e| e.id == base)),
        ("placements", document.placements.iter().any(|e| e.id == base)),
    ];
    found.iter().find(|(_, hit)| *hit).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1995Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let mut out: BTreeSet<String> = BTreeSet::new();
    out.extend(document.members.iter().filter(|e| e.id == base).flat_map(|m| m.actions.iter().map(|a| a.id.clone())));
    out.extend(document.connections.iter().filter(|e| e.id == base).flat_map(|c| c.actions.iter().map(|a| a.id.clone())));
    out.extend(document.cases.iter().filter(|e| e.id == base).flat_map(|c| c.numbers.iter().map(|n| n.id.clone()).chain(c.texts.iter().map(|t| t.id.clone()))));
    out.extend(document.tapers.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(document.cutouts.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(document.fasteners.iter().filter(|e| e.id == base).flat_map(|e| [e.member_id.clone(), e.connection_id.clone()]));
    out.extend(document.floors.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.extend(document.bracings.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    out.into_iter().collect()
}

/// 🧱️ The rectangular timber section outline (`h` vertical, `b` horizontal).
pub(crate) fn section_shape(h: f64, b: f64) -> SectionShape {
    SectionShape::parametric(ProfileKind::Rect, &[("h", h.max(1e-3)), ("b", b.max(1e-3))])
}

/// 🕳️ A centred rectangle with a centred rectangular hole: the hull between two nested outlines.
pub(crate) fn hull(outer: (f64, f64), inner: (f64, f64)) -> Polygon2 {
    let hole = rect(inner.0, inner.1);
    Polygon2 { outer: rect(outer.0, outer.1).outer, holes: vec![Ring2 { points: hole.outer.points.into_iter().rev().collect() }] }
}

/// 📥️ Append-only scene accumulator of one projection part: entities in document order, owned annotations and tracks, legends and the points that bound the part.
#[derive(Default)]
pub(crate) struct Out {
    pub entities: Vec<SceneEntity>,
    pub annotations: Vec<Annotation>,
    pub tracks: Vec<ParameterTrack>,
    pub legends: Vec<Legend>,
    owners: Vec<Ownership>,
    points: Vec<Vec3>,
    scope: Scope,
    owner: Option<EntityRef>,
    /// 🏷️ The collection of the entity being drawn through the case engine; empty means the document cases.
    pub home: &'static str,
}

#[derive(Clone, Default)]
enum Scope {
    #[default]
    All,
    Entity(String),
    Global,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one member, connection or case, or the global rest, at a time.
    pub(crate) fn scoped(part: Part<'_>) -> Self {
        let scope = match part {
            Part::All => Scope::All,
            Part::Entity(id) => Scope::Entity(id.to_owned()),
            Part::Global => Scope::Global,
        };
        Self { scope, ..Self::default() }
    }

    /// 🏷️ The collection the drawn entity belongs to: `cases` unless a typed entity is drawn as the case row it stands for.
    pub(crate) fn home(&self) -> &'static str {
        if self.home.is_empty() { "cases" } else { self.home }
    }

    fn part(&self) -> Part<'_> {
        match &self.scope {
            Scope::All => Part::All,
            Scope::Entity(id) => Part::Entity(id),
            Scope::Global => Part::Global,
        }
    }

    /// 🎯️ Whether the entity `id` is part of this projection; modules test it before building an entity's geometry.
    pub(crate) fn wants(&self, id: &str) -> bool {
        self.part().wants(id)
    }

    /// 📥️ Pushes an entity built outside the spec builder (the case forms of the projection kit); its document row is the case of its base id.
    pub(crate) fn push_entity(&mut self, entity: SceneEntity, points: &[Vec3]) {
        self.owner = Some(entity_ref(EntityKind::Solid, base_id(&entity.entity.id), "cases"));
        if !self.wants(&entity.entity.id) {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(entity);
    }

    pub(crate) fn push(&mut self, spec: Spec, points: &[Vec3]) {
        self.owner = Some(owner_of(&spec));
        if !self.wants(&spec.id) {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(spec.build());
    }

    fn link(&mut self, kind: OwnedKind, id: &str) -> bool {
        if !self.part().keeps(self.owner.as_ref().map(|o| o.id.as_str())) {
            return false;
        }
        if let Some(owner) = &self.owner {
            self.owners.push(Ownership { kind, id: id.to_owned(), owner: owner.clone() });
        }
        true
    }

    pub(crate) fn annotate(&mut self, annotation: Annotation, anchor: &Vec3) {
        if self.link(OwnedKind::Annotation, annotation.id()) {
            self.points.push(anchor.clone());
            self.annotations.push(annotation);
        }
    }

    pub(crate) fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: self.legends, layers: Vec::new(), tracks: self.tracks, owners: self.owners, bounds }
    }
}

impl entities::Sink for Out {
    fn wants(&self, id: &str) -> bool {
        Out::wants(self, id)
    }

    fn home(&self) -> &'static str {
        Out::home(self)
    }

    fn set_home(&mut self, home: &'static str) {
        self.home = home;
    }

    fn push(&mut self, spec: Spec, points: &[Vec3]) {
        Out::push(self, spec, points);
    }

    fn annotate(&mut self, annotation: Annotation, anchor: &Vec3) {
        Out::annotate(self, annotation, anchor);
    }

    fn legend(&mut self, legend: Legend) {
        self.legends.push(legend);
    }
}

/// 🧭️ The registry entity a scene entity belongs to: its document row, whatever sub-entity of it is drawn.
fn owner_of(spec: &Spec) -> EntityRef {
    let id = base_id(&spec.id);
    ENTITY_KINDS.iter().find(|k| k.collection == spec.collection).map_or_else(|| entity_ref(spec.kind, id, spec.collection), |k| entity_ref(k.scene_kind, id, spec.collection))
}

/// 🔗️ One check of the spec trace bound to the scene: the check id, the collection of the entity that carries it (its 3D body is coloured by the merged utilisation of the entity's checks, members and connections add one failure-mode overlay per check prefix) and the key of the diagram whose marker is bound to the check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckBinding {
    pub check: &'static str,
    pub collection: &'static str,
    pub diagram: &'static str,
}

const fn bound(check: &'static str, collection: &'static str, diagram: &'static str) -> CheckBinding {
    CheckBinding { check, collection, diagram }
}

/// 🔗️ The single registry of the checks of the EN 1995 spec trace and where each shows up: its owning entity collection and diagram. Every check id of the trace has a row (law test), and the diagram of a row is offered for every entity of its collection that carries the check.
pub const CHECK_BINDINGS: &[CheckBinding] = &[
    bound("en1995.material", "members", "kmod"),
    bound("en1995.6.1.6.bending", "members", "kcrit"),
    bound("en1995.6.1.7.shear", "members", "member-results"),
    bound("en1995.6.3.2.compression", "members", "kc"),
    bound("en1995.7.2.winst", "members", "deflection"),
    bound("en1995.7.2.wfin", "members", "deflection"),
    bound("en1995.7.3.f1", "members", "frequency"),
    bound("en1995.7.3.stiffness", "members", "member-results"),
    bound("en1995.7.3.velocity", "members", "velocity"),
    bound("en1995.1-2.4.fire", "members", "fire"),
    bound("en1995.2.a.fatigue", "members", "fatigue"),
    bound("en1995.2.b.avert", "members", "frequency"),
    bound("en1995.8.2.2.johansen", "connections", "johansen"),
    bound("en1995.8.2.3.johansen", "connections", "johansen"),
    bound("en1995.8.spacing.a1", "connections", "spacing"),
    bound("en1995.t-18.net_section", "cutouts", "case-hole-net"),
    bound("en1995.t-30.", "tapers", "case-taper-length"),
    bound("en1995.t-31.apex_bending", "tapers", "case-apex"),
    bound("en1995.t-32.", "tapers", "case-apex-tension"),
    bound("en1995.t-33.", "tapers", "case-apex-reinforcement"),
    bound("en1995.t-46.", "cutouts", "case-notch-rods"),
    bound("en1995.t-94.", "diaphragms", "case-results"),
    bound("en1995.t-95.", "trusses", "case-truss-chord"),
    bound("en1995.t-96.", "bracings", "case-bracing"),
    bound("en1995.t-97.rolling_shear", "panels", "case-layup"),
    bound("en1995.t-122.", "decks", "case-wheel-spread"),
];

/// 🧭️ The scene entity of a catalogue case: the body solid of its form.
pub(crate) fn case_ref(case: &model::CaseRow) -> EntityRef {
    entity_ref(EntityKind::Solid, &case.id, "cases")
}

/// 🆔️ Every document entity in outliner order: the members, the connections and the catalogue cases; their sub-entities (`id:tag`) are projected with them.
pub fn entities_of(document: &model::En1995Snapshot) -> Vec<EntityRef> {
    let cases = document.cases.iter().map(case_ref).collect::<Vec<_>>();
    let of = |collection: &'static str, ids: Vec<&str>| ENTITY_KINDS.iter().find(|k| k.collection == collection).into_iter().flat_map(|k| ids.into_iter().map(|id| entity_ref(k.scene_kind, id, collection))).collect::<Vec<_>>();
    [of("members", document.members.iter().map(|e| e.id.as_str()).collect()), of("connections", document.connections.iter().map(|e| e.id.as_str()).collect()), cases, of("tapers", document.tapers.iter().map(|e| e.id.as_str()).collect()), of("cutouts", document.cutouts.iter().map(|e| e.id.as_str()).collect()), of("fasteners", document.fasteners.iter().map(|e| e.id.as_str()).collect()), of("panels", document.panels.iter().map(|e| e.id.as_str()).collect()), of("floors", document.floors.iter().map(|e| e.id.as_str()).collect()), of("diaphragms", document.diaphragms.iter().map(|e| e.id.as_str()).collect()), of("trusses", document.trusses.iter().map(|e| e.id.as_str()).collect()), of("bracings", document.bracings.iter().map(|e| e.id.as_str()).collect()), of("decks", document.decks.iter().map(|e| e.id.as_str()).collect())].concat()
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments, the description and the fire track.
pub(crate) fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let fields = std::mem::take(&mut scene.legends);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Materials", "Werkstoffe"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(fields);
    scene.description = description;
    super::tracks::finish(scene);
    super::entities::finish(scene);
}

/// 🌈️ The scalar legend of a utilisation field with the viridis ramp 0 to 1,2, pushed once per field id.
pub(crate) fn field_legend(legends: &mut Vec<Legend>, id: &str, title: LocalizedCopy) {
    if !legends.iter().any(|l| l.id == id) {
        legends.push(Legend { id: id.into(), title, scale: LegendScale::Scalar { field_id: id.into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: 1.2 }, unit: QuantityKind::Dimensionless } });
    }
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Timber => ("Timber", "Holz"),
            MaterialRole::Steel => ("Steel plates", "Stahlbleche"),
            MaterialRole::Bolt => ("Fasteners", "Verbindungsmittel"),
            MaterialRole::Support => ("Supports and bearings", "Lager und Auflager"),
            MaterialRole::Load { .. } => ("Loads", "Lasten"),
            _ => return None,
        };
        let key = match role {
            MaterialRole::Load { .. } => MaterialRole::Load { kind: LoadKind::Other },
            other => other.clone(),
        };
        Some((label, key))
    })
}
