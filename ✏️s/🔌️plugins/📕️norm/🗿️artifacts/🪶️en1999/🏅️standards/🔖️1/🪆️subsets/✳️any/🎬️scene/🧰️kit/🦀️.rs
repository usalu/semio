//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::*;
use super::scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, add, mul, unit, along, member_frame, entity_ref, clause_of, tone_of, utilisation_words, polyline, envelope, status as to_scene_status};
use scene::projection::{Fragment, Part};
use std::collections::{BTreeMap, BTreeSet, HashSet};

pub const LAYER_MEMBERS: &str = "members";
pub const LAYER_HAZ: &str = "haz";
pub const LAYER_EFFECTIVE: &str = "effective";
pub const LAYER_CONNECTIONS: &str = "connections";
pub const LAYER_SHEETS: &str = "sheets";
pub const LAYER_SHELLS: &str = "shells";
pub const LAYER_WELDS: &str = "welds";
pub const LAYER_STIFFENERS: &str = "stiffeners";
pub const LAYER_LOADS: &str = "loads";
pub const LAYER_SUPPORTS: &str = "supports";
pub const LAYER_ANNOTATIONS: &str = "annotations";

const LAYERS: [(&str, &str, &str); 14] = [
    (LAYER_MEMBERS, "Extrusions", "Strangpressprofile"),
    (LAYER_HAZ, "Heat-affected zones", "Wärmeeinflusszonen"),
    (LAYER_EFFECTIVE, "Effective sections", "Wirksame Querschnitte"),
    (LAYER_CONNECTIONS, "Connections", "Anschlüsse"),
    (LAYER_WELDS, "Weld runs", "Schweißnähte"),
    (LAYER_STIFFENERS, "Stiffeners", "Steifen"),
    (LAYER_SHEETS, "Cold-formed sheeting", "Kaltprofile"),
    (LAYER_SHELLS, "Shells", "Schalen"),
    (LAYER_LOADS, "Actions and fire", "Einwirkungen und Brand"),
    (LAYER_SUPPORTS, "Supports and restraints", "Lager und Halterungen"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
    (super::tracks::LAYER_MODE_FLEXURAL, "Flexural buckling mode", "Biegeknickform"),
    (super::tracks::LAYER_MODE_LTB, "Lateral-torsional buckling mode", "Biegedrillknickform"),
    (super::tracks::LAYER_MODE_TF, "Torsional-flexural buckling mode", "Drill- und Biegedrillknickform"),
    (super::tracks::LAYER_MODE_SHELL, "Axial shell buckling mode", "Axiale Schalenbeulform"),
];

/// 🏷️ One entity kind of the EN 1999 outliner, interaction domain and insert palette; the last four collections (sections, alloys, outline vertices, placements) are palette and outliner rows that are not drawn as scene entities of their own.
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

/// 🏷️ The single registry of EN 1999 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("member", "members", "Member", "Bauteil", "minus", EntityKind::Member),
    kind("connection", "connections", "Connection", "Anschluss", "link", EntityKind::Group),
    kind("coldFormedSheet", "coldFormed", "Cold-formed sheet", "Kaltprofil", "layers", EntityKind::Member),
    kind("shell", "shells", "Shell", "Schale", "cylinder", EntityKind::Shell),
    kind("fireScenario", "fireScenarios", "Fire scenario", "Brandszenario", "flame", EntityKind::Load),
    kind("fatigueDetail", "fatigueDetails", "Fatigue detail", "Ermüdungsdetail", "repeat", EntityKind::Group),
    kind("weldRun", "weldRuns", "Weld run", "Schweißnaht", "flame", EntityKind::Fastener),
    kind("fatigueCategory", "fatigueCategories", "Fatigue site", "Ermüdungsstelle", "target", EntityKind::Solid),
    kind("patchLoad", "patchLoads", "Web patch load", "Stegeinzellast", "arrow-down", EntityKind::Load),
    kind("ringStiffener", "ringStiffeners", "Ring stiffener", "Ringsteife", "circle", EntityKind::Shell),
    kind("transverseStiffener", "transverseStiffeners", "Transverse stiffener", "Quersteife", "separator-vertical", EntityKind::Member),
    kind("section", "sections", "Section", "Querschnitt", "shapes", EntityKind::Group),
    kind("material", "materials", "Alloy", "Legierung", "palette", EntityKind::Group),
    kind("outlineVertex", "outlineVertices", "Outline vertex", "Umrisspunkt", "pentagon", EntityKind::Group),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
];

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1999Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    let d = document;
    let found: [(&str, bool); 15] = [
        ("members", d.members.iter().any(|e| e.id == base)),
        ("connections", d.connections.iter().any(|e| e.id == base)),
        ("coldFormed", d.cold_formed.iter().any(|e| e.id == base)),
        ("shells", d.shells.iter().any(|e| e.id == base)),
        ("fireScenarios", d.fire_scenarios.iter().any(|e| e.id == base)),
        ("fatigueDetails", d.fatigue_details.iter().any(|e| e.id == base)),
        ("weldRuns", d.weld_runs.iter().any(|e| e.id == base)),
        ("fatigueCategories", d.fatigue_categories.iter().any(|e| e.id == base)),
        ("patchLoads", d.patch_loads.iter().any(|e| e.id == base)),
        ("ringStiffeners", d.ring_stiffeners.iter().any(|e| e.id == base)),
        ("transverseStiffeners", d.transverse_stiffeners.iter().any(|e| e.id == base)),
        ("sections", d.sections.iter().any(|e| e.id == base)),
        ("materials", d.materials.iter().any(|e| e.id == base)),
        ("outlineVertices", d.outline_vertices.iter().any(|e| e.id == base)),
        ("placements", d.placements.iter().any(|e| e.id == base)),
    ];
    found.iter().find(|(_, hit)| *hit).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1999Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let d = document;
    let mut out: BTreeSet<String> = BTreeSet::new();
    if let Some(m) = d.members.iter().find(|e| e.id == base) {
        out.insert(m.section_id.clone());
        out.insert(m.material_id.clone());
        out.extend(m.actions.iter().map(|a| a.id.clone()));
        out.extend(d.connections.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));
        out.extend(d.fire_scenarios.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));
        out.extend(d.fatigue_details.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));
        out.extend(d.weld_runs.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));
        out.extend(d.fatigue_categories.iter().filter(|c| c.member_id == base).map(|c| c.id.clone()));
        out.extend(d.patch_loads.iter().filter(|c| c.subject_id == base).map(|c| c.id.clone()));
        out.extend(d.outline_vertices.iter().filter(|v| v.section_id == m.section_id).map(|v| v.id.clone()));
        out.extend(d.transverse_stiffeners.iter().filter(|s| s.girder_id == base).map(|s| s.id.clone()));
    }
    for run in d.weld_runs.iter().filter(|e| e.id == base) {
        out.insert(run.member_id.clone());
    }
    for connection in d.connections.iter().filter(|e| e.id == base) {
        out.extend([connection.member_id.clone(), connection.material_id.clone()]);
        out.extend(connection.actions.iter().map(|a| a.id.clone()));
    }
    out.extend(d.fatigue_details.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));
    for cat in d.fatigue_categories.iter().filter(|e| e.id == base) {
        out.insert(cat.member_id.clone());
        out.extend(checks::part_1_3::category_blocks(cat, &d.fatigue_details).into_iter().map(|b| b.id.clone()));
    }
    for load in d.patch_loads.iter().filter(|e| e.id == base) {
        out.insert(load.subject_id.clone());
        out.extend(d.members.iter().filter(|m| m.id == load.subject_id).flat_map(|m| [m.section_id.clone(), m.material_id.clone()]));
    }
    out.extend(d.ring_stiffeners.iter().filter(|e| e.id == base).map(|e| e.shell_id.clone()));
    for st in d.transverse_stiffeners.iter().filter(|e| e.id == base) {
        out.insert(st.girder_id.clone());
        out.extend(d.members.iter().filter(|m| m.id == st.girder_id).flat_map(|m| [m.section_id.clone(), m.material_id.clone()]));
        out.extend(d.transverse_stiffeners.iter().filter(|s| s.girder_id == st.girder_id).map(|s| s.id.clone()));
    }
    out.extend(d.fire_scenarios.iter().filter(|e| e.id == base).flat_map(|e| std::iter::once(e.member_id.clone()).chain(d.fire_insulations.iter().filter(|i| i.fire_id == base).map(|i| i.id.clone()))));
    for sheet in d.cold_formed.iter().filter(|e| e.id == base) {
        out.insert(sheet.material_id.clone());
        out.extend(sheet.actions.iter().map(|a| a.id.clone()));
        out.extend(d.sheet_profiles.iter().filter(|p| p.sheet_id == base).map(|p| p.id.clone()));
    }
    for shell in d.shells.iter().filter(|e| e.id == base) {
        out.insert(shell.material_id.clone());
        out.extend(shell.actions.iter().map(|a| a.id.clone()));
    }
    out.remove(base);
    out.into_iter().collect()
}

/// 🔍️ The worst applicable check of `entity` whose id starts with `prefix`.
pub fn worst<'a>(report: &'a norm::CheckReport, entity: &str, prefix: &str) -> Option<&'a norm::CheckResult> {
    report.checks.iter().filter(|c| c.subject.entity_id == entity && c.id.starts_with(prefix) && c.status != norm::CheckStatus::NotApplicable && c.utilization.is_finite()).max_by(|a, b| a.utilization.total_cmp(&b.utilization))
}

/// 🔗️ The member id plus every fire and fatigue element that verifies the same member: the subjects whose checks colour the member.
pub fn dependents<'a>(d: &'a model::En1999Snapshot, member: &'a str) -> Vec<&'a str> {
    std::iter::once(member)
        .chain(d.fatigue_details.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.fire_scenarios.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.fatigue_categories.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str()))
        .chain(d.patch_loads.iter().filter(|e| e.subject_id == member).map(|e| e.id.as_str()))
        .chain(d.transverse_stiffeners.iter().filter(|e| e.girder_id == member).map(|e| e.id.as_str()))
        .collect()
}

fn rectangle(width: f64, height: f64, hole: Option<(f64, f64)>) -> Polygon2 {
    let ring = |w: f64, h: f64| Ring2 { points: vec![Vec2::new(-w / 2.0, -h / 2.0), Vec2::new(w / 2.0, -h / 2.0), Vec2::new(w / 2.0, h / 2.0), Vec2::new(-w / 2.0, h / 2.0)] };
    Polygon2 { outer: ring(width, height), holes: hole.into_iter().map(|(w, h)| ring(w, h)).collect() }
}

/// 🧱️ The section outline source: I, channel, angle and round tube as parametric profiles, box and rectangular hollow sections as polygons with a hole, anything with unusable wall data as its bounding `Rect`.
pub fn section_shape(s: &model::AluminiumSection) -> SectionShape {
    let (h, b) = (s.height.max(1e-3), s.width.max(1e-3));
    let (tf, tw) = (s.flange_thickness, s.web_thickness);
    let t = tf.max(tw);
    match s.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Chs => {
            let d = s.outer_diameter.max(s.width).max(1e-3);
            let wall = tw.max(tf);
            if wall > 0.0 && 2.0 * wall < d { SectionShape::parametric(ProfileKind::Chs, &[("d", d), ("t", wall)]) } else { SectionShape::parametric(ProfileKind::Circle, &[("d", d)]) }
        }
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs if tf > 0.0 && tw > 0.0 && 2.0 * tf < h && 2.0 * tw < b => SectionShape::Polygon { polygon: rectangle(b, h, Some((b - 2.0 * tw, h - 2.0 * tf))) },
        semio_s_artifact_norm_en1999_model::SectionKind::Angle if t > 0.0 && t < b.min(h) => SectionShape::parametric(ProfileKind::Angle, &[("h", h), ("b", b), ("t", t)]),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel if tw > 0.0 && tf > 0.0 && 2.0 * tf < h && tw < b => SectionShape::parametric(ProfileKind::Channel, &[("h", h), ("b", b), ("tw", tw), ("tf", tf)]),
        semio_s_artifact_norm_en1999_model::SectionKind::ExtrudedI if tw > 0.0 && tf > 0.0 && 2.0 * tf < h && tw < b => SectionShape::parametric(ProfileKind::IShape, &[("h", h), ("b", b), ("tw", tw), ("tf", tf)]),
        _ => SectionShape::parametric(ProfileKind::Rect, &[("h", h), ("b", b)]),
    }
}

/// 🔷️ The free outline of a section as a planar polygon with holes, when its vertices form a valid polygon.
pub fn outline_polygon(d: &model::En1999Snapshot, s: &model::AluminiumSection) -> Option<Polygon2> {
    let rings = checks::rings(&d.outline_vertices, &s.id);
    checks::outline_props(&rings)?;
    let ring = |r: &checks::Ring| Ring2 { points: r.points.iter().map(|p| Vec2::new(p.0, p.1)).collect() };
    let outer = rings.iter().find(|r| r.name == "outer")?;
    Some(Polygon2 { outer: ring(outer), holes: rings.iter().filter(|r| r.name != "outer").map(ring).collect() })
}

/// 🧱️ The drawn section: the outline polygon when the section has a valid outline, else the parametric source of [`section_shape`].
pub fn section_shape_of(d: &model::En1999Snapshot, s: &model::AluminiumSection) -> SectionShape {
    outline_polygon(d, s).map_or_else(|| section_shape(s), |polygon| SectionShape::Polygon { polygon })
}

/// 📐️ Overall depth and width of the drawn section (bounding box of the outline when there is one).
pub fn section_extent(d: &model::En1999Snapshot, s: &model::AluminiumSection) -> (f64, f64) {
    match outline_polygon(d, s) {
        Some(p) => {
            let span = |f: fn(&Vec2) -> f64| {
                let v: Vec<f64> = p.outer.points.iter().map(f).collect();
                v.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - v.iter().cloned().fold(f64::INFINITY, f64::min)
            };
            (span(|q| q.y).max(1e-3), span(|q| q.x).max(1e-3))
        }
        None => (section_depth(s), section_width(s)),
    }
}

/// 📐️ The overall height of the drawn section (tube: diameter).
pub fn section_depth(s: &model::AluminiumSection) -> f64 {
    if checks::part_1_1::is_tube(s) { s.outer_diameter.max(s.width).max(1e-3) } else { s.height.max(1e-3) }
}

/// 📐️ The overall width of the drawn section (tube: diameter).
pub fn section_width(s: &model::AluminiumSection) -> f64 {
    if checks::part_1_1::is_tube(s) { s.outer_diameter.max(s.width).max(1e-3) } else { s.width.max(1e-3) }
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
    pub deformation: Option<DeformationOverlay>,
    pub style: Option<(StyleState, f64, Option<Hatch>)>,
}

impl Spec {
    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        let (style, opacity, hatch) = self.style.unwrap_or(if failing { (StyleState::Error, 1.0, Some(Hatch::Diagonal)) } else { (StyleState::Normal, 1.0, None) });
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, self.collection),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role },
                style,
                opacity,
                hatch,
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
    router: Option<Router>,
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

    /// 🧭️ The same accumulator with the check routing of the report it projects, built once per fragment so every utilisation binding is a lookup.
    pub fn routed(mut self, router: Router) -> Self {
        self.router = Some(router);
        self
    }

    /// 📶️ The governing binding among the checks routed to any of the rows `ids`, see [`route_of`].
    pub fn binding(&self, input: &SceneInput, ids: &[&str]) -> Option<UtilisationBinding> {
        self.binding_with(input, ids, &[])
    }

    /// 📶️ [`Self::binding`] restricted to the check families whose id starts with any of `prefixes`.
    pub fn binding_with(&self, input: &SceneInput, ids: &[&str], prefixes: &[&str]) -> Option<UtilisationBinding> {
        match &self.router {
            Some(router) => router.binding(input.report, ids, prefixes),
            None => Router::of(input.document, input.report).binding(input.report, ids, prefixes),
        }
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

    /// 🌈️ [`Self::push`] with a scalar-field colour channel (re-sealed after the channel is set).
    pub fn push_with_channel(&mut self, spec: Spec, channel: ColourChannel, points: &[Vec3]) {
        if !self.active {
            return;
        }
        self.points.extend_from_slice(points);
        let mut entity = spec.build();
        entity.appearance.channel = channel;
        self.entities.push(entity.sealed());
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
    entity_ref(kind, id, collection)
}

/// 🗂️ The ids of every collection of the document, in outliner order.
fn rows_of(d: &model::En1999Snapshot) -> [(&'static str, Vec<&str>); 11] {
    [
        ("members", d.members.iter().map(|e| e.id.as_str()).collect()),
        ("connections", d.connections.iter().map(|e| e.id.as_str()).collect()),
        ("coldFormed", d.cold_formed.iter().map(|e| e.id.as_str()).collect()),
        ("shells", d.shells.iter().map(|e| e.id.as_str()).collect()),
        ("fireScenarios", d.fire_scenarios.iter().map(|e| e.id.as_str()).collect()),
        ("fatigueDetails", d.fatigue_details.iter().map(|e| e.id.as_str()).collect()),
        ("weldRuns", d.weld_runs.iter().map(|e| e.id.as_str()).collect()),
        ("fatigueCategories", d.fatigue_categories.iter().map(|e| e.id.as_str()).collect()),
        ("patchLoads", d.patch_loads.iter().map(|e| e.id.as_str()).collect()),
        ("ringStiffeners", d.ring_stiffeners.iter().map(|e| e.id.as_str()).collect()),
        ("transverseStiffeners", d.transverse_stiffeners.iter().map(|e| e.id.as_str()).collect()),
    ]
}

/// 🧭️ The drawn rows of a document with the lookups that route a check to its owner: a hash set of the row ids, so a check is routed without scanning the collections.
pub struct Rows<'a> {
    document: &'a model::En1999Snapshot,
    drawn: HashSet<&'a str>,
}

impl<'a> Rows<'a> {
    pub fn of(document: &'a model::En1999Snapshot) -> Self {
        Self { document, drawn: rows_of(document).into_iter().flat_map(|(_, ids)| ids).collect() }
    }

    /// 🧭️ The drawn row that owns `check`: the row its subject names (the filed report), else the member, connection, sheet or shell that references the subject section, alloy or outline vertex, else the longest row id the check id embeds as a dot-separated segment (the report is not filed yet), else the owner of the action row the subject names; `None` for a document-level check.
    pub fn owner(&self, check: &norm::CheckResult) -> Option<String> {
        let subject = base_id(&check.subject.entity_id);
        if self.drawn.contains(subject) {
            return Some(subject.to_owned());
        }
        self.referenced_by(subject).or_else(|| self.embedded(&check.id)).or_else(|| self.action_owner(subject))
    }

    fn referenced_by(&self, id: &str) -> Option<String> {
        if id.is_empty() {
            return None;
        }
        let d = self.document;
        let section = d.outline_vertices.iter().find(|v| v.id == id).map_or(id, |v| v.section_id.as_str());
        d.sheet_profiles.iter().find(|p| p.id == id).map(|p| p.sheet_id.clone()).or_else(|| d.fire_insulations.iter().find(|i| i.id == id).map(|i| i.fire_id.clone())).or_else(|| d.members
            .iter()
            .find(|m| m.section_id == section || m.material_id == id)
            .map(|m| m.id.clone())
            .or_else(|| d.connections.iter().find(|c| c.material_id == id).map(|c| c.id.clone()))
            .or_else(|| d.cold_formed.iter().find(|c| c.material_id == id).map(|c| c.id.clone()))
            .or_else(|| d.shells.iter().find(|c| c.material_id == id).map(|c| c.id.clone())))
    }

    fn embedded(&self, check_id: &str) -> Option<String> {
        check_id.split('.').filter(|segment| self.drawn.contains(segment)).fold(None::<&str>, |best, segment| if best.is_none_or(|b| segment.len() > b.len()) { Some(segment) } else { best }).map(str::to_owned)
    }

    fn action_owner(&self, id: &str) -> Option<String> {
        if id.is_empty() {
            return None;
        }
        let d = self.document;
        d.members
            .iter()
            .find(|o| o.actions.iter().any(|a| a.id == id))
            .map(|o| o.id.clone())
            .or_else(|| d.connections.iter().find(|o| o.actions.iter().any(|a| a.id == id)).map(|o| o.id.clone()))
            .or_else(|| d.cold_formed.iter().find(|o| o.actions.iter().any(|a| a.id == id)).map(|o| o.id.clone()))
            .or_else(|| d.shells.iter().find(|o| o.actions.iter().any(|a| a.id == id)).map(|o| o.id.clone()))
    }

    fn is_registry_row(&self, id: &str) -> bool {
        let d = self.document;
        !id.is_empty() && (d.sections.iter().any(|e| e.id == id) || d.materials.iter().any(|e| e.id == id) || d.outline_vertices.iter().any(|e| e.id == id))
    }
}

/// 🧭️ The checks of one report grouped by the row that owns them, so the utilisation of a row is a lookup instead of a scan of the report.
pub struct Router {
    owned: BTreeMap<String, Vec<usize>>,
}

impl Router {
    pub fn of(document: &model::En1999Snapshot, report: &norm::CheckReport) -> Self {
        let rows = Rows::of(document);
        let mut owned: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (index, check) in report.checks.iter().enumerate() {
            if let Some(owner) = rows.owner(check) {
                owned.entry(owner).or_default().push(index);
            }
        }
        Self { owned }
    }

    /// 📶️ The governing binding among the applicable checks routed to any of `ids` whose id starts with any of `prefixes` (every one when empty).
    pub fn binding(&self, report: &norm::CheckReport, ids: &[&str], prefixes: &[&str]) -> Option<UtilisationBinding> {
        ids.iter()
            .filter_map(|id| self.owned.get(*id))
            .flatten()
            .map(|index| &report.checks[*index])
            .filter(|c| c.status != norm::CheckStatus::NotApplicable && c.utilization.is_finite() && (prefixes.is_empty() || prefixes.iter().any(|p| c.id.starts_with(p))))
            .max_by(|a, b| a.utilization.total_cmp(&b.utilization))
            .map(projection_kit::binding_of_check)
    }
}

/// 🧭️ The drawn row that owns `check`, see [`Rows::owner`].
pub fn owner_entity(document: &model::En1999Snapshot, check: &norm::CheckResult) -> Option<String> {
    Rows::of(document).owner(check)
}

/// 📄️ Whether `check_id` belongs to the document as a whole (missing references, empty verdicts, integrity, the outline reference rules); the list is the document-level set of the shared check ownership of the EN 1999 units, with no second copy.
pub fn is_document_check(check_id: &str) -> bool {
    <checks::En1999Units as norm::incremental_evaluation::UnitSource>::OWNERS.is_document_level(check_id)
}

/// 🧭️ Where a check is drawn: on one drawn row, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The route of `check`: its owner row; else the document for a document-level family or a section, alloy or outline vertex no row uses; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::En1999Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    let rows = Rows::of(document);
    rows.owner(check).map(CheckRoute::Entity).or_else(|| (is_document_check(&check.id) || rows.is_registry_row(base_id(&check.subject.entity_id))).then_some(CheckRoute::Document))
}

/// 🎯️ The row to select and frame for the check `check_id` of `report`; `None` for a document-level check.
pub fn focus_of_outcome(document: &model::En1999Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📈️ The check families a dedicated diagram draws as a formula curve, a plan or a table with the design point bound to the check; the id prefixes of the spec trace, see [`is_drawn_family`].
pub const DRAWN_FAMILIES: &[&str] = &[
    "en1999.3.2.alloy.", "en1999.6.1.4.class.", "en1999.6.1.6.haz.", "en1999.6.1.outline.", "en1999.6.2.5.m.", "en1999.6.2.9.nm.", "en1999.6.3.1.fb.", "en1999.6.3.1.tb.", "en1999.6.3.1.haz.", "en1999.6.3.2.ltb.", "en1999.6.7.5.patch.", "en1999.6.7.6.",
    "en1999.8.5.bolt.", "en1999.8.5.tension.", "en1999.8.6.weld.", "en1999.1-2.fire.", "en1999.1-3.fat.", "en1999.1-4.local.", "en1999.1-4.axial.", "en1999.1-4.bend.", "en1999.1-4.shear.", "en1999.1-4.nm.", "en1999.1-5.buckle.",
];

/// 📊️ The carve-outs of [`DRAWN_FAMILIES`]: reference and placement checks of a drawn family that its diagram does not plot; they are bars of the outcome diagram of their owner.
pub const OUTCOME_FAMILIES: &[&str] = &["en1999.1-3.fat.category.", "en1999.1-3.fat.site-", "en1999.6.7.6.web-ref.", "en1999.6.7.6.web-position.", "en1999.6.7.6.web-section.", "en1999.6.7.6.web-alloy."];

/// 📊️ Whether a dedicated diagram draws the check `check_id`; every other check of a row is a bar of its outcome diagram `check-outcomes:<row>`.
pub fn is_drawn_family(check_id: &str) -> bool {
    DRAWN_FAMILIES.iter().any(|prefix| check_id.starts_with(prefix)) && !OUTCOME_FAMILIES.iter().any(|prefix| check_id.starts_with(prefix))
}

/// 🆔️ Every document entity in outliner order: the canonical order of the scene.
pub fn entities_of(document: &model::En1999Snapshot) -> Vec<EntityRef> {
    rows_of(document).into_iter().flat_map(|(collection, ids)| ids.into_iter().map(move |id| owner_ref(collection, id))).collect()
}

/// 📍️ What an entity's pose derives from beyond its own row: the member pose and the default slot of a sheet or shell (sheets stand behind the spans of the sheets before them).
pub fn layout_stamps(document: &model::En1999Snapshot, layout: &Layout) -> std::collections::BTreeMap<String, String> {
    let members = document.members.iter().map(|m| (m.id.clone(), format!("{:?}", layout.member(&m.id))));
    let rows = document.cold_formed.iter().map(|e| &e.id).chain(document.shells.iter().map(|e| &e.id)).map(|id| (id.clone(), format!("{:?}", layout.origin(id))));
    members.chain(rows).collect()
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends, the description and the buckling-mode track.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    let carried: Vec<Legend> = std::mem::take(&mut scene.legends).into_iter().filter(|l| l.id == super::fire_and_fatigue::FIRE_LEGEND).collect();
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Materials", "Werkstoffe"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(carried);
    scene.description = description;
    super::tracks::finish(scene);
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Aluminium => ("Aluminium", "Aluminium"),
            MaterialRole::Bolt => ("Bolts and rivets", "Schrauben und Niete"),
            MaterialRole::Weld => ("Welds and heat-affected zones", "Schweißnähte und Wärmeeinflusszonen"),
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
