//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::*;
use super::model::{CaseRow, RcMember};
use scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, add, mul, unit, polyline, ring3, status, clause_of, tone_of, utilisation_words, at, binding_of_subject as binding};
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

pub const LAYER_CONCRETE: &str = "concrete";
pub const LAYER_COVER: &str = "cover";
pub const LAYER_REBAR: &str = "reinforcement";
pub const LAYER_PRESTRESS: &str = "prestress";
pub const LAYER_PUNCHING: &str = "punching";
pub const LAYER_ANCHORS: &str = "anchors";
pub const LAYER_LOADS: &str = "loads";
pub const LAYER_SUPPORTS: &str = "supports";
pub const LAYER_INTERACTION: &str = "interaction";
pub const LAYER_CRACKS: &str = "cracks";
pub const LAYER_SHEAR: &str = "shear";
pub const LAYER_FIRE: &str = "fire";
pub const LAYER_AGE: &str = "age";
pub const LAYER_FATIGUE: &str = "fatigue";
pub const LAYER_STRUT_AND_TIE: &str = "strut-and-tie";
pub const LAYER_GRADES: &str = "grades";
pub const LAYER_BOND: &str = "bond";
pub const LAYER_CASES: &str = "cases";
pub const LAYER_ANNOTATIONS: &str = "annotations";

const LAYERS: [(&str, &str, &str); 19] = [
    (LAYER_CONCRETE, "Concrete", "Beton"),
    (LAYER_COVER, "Concrete cover", "Betondeckung"),
    (LAYER_REBAR, "Reinforcement", "Bewehrung"),
    (LAYER_PRESTRESS, "Prestressing", "Vorspannung"),
    (LAYER_PUNCHING, "Punching perimeters", "Durchstanzumfänge"),
    (LAYER_ANCHORS, "Anchors", "Dübel"),
    (LAYER_LOADS, "Loads", "Lasten"),
    (LAYER_SUPPORTS, "Supports", "Lager"),
    (LAYER_INTERACTION, "N-My-Mz interaction surface", "N-My-Mz-Interaktionsfläche"),
    (LAYER_CRACKS, "Crack pattern", "Rissbild"),
    (LAYER_SHEAR, "Critical shear sections", "Kritische Schubschnitte"),
    (LAYER_FIRE, "Fire isotherm", "Brandisotherme"),
    (LAYER_AGE, "Creep and shrinkage", "Kriechen und Schwinden"),
    (LAYER_FATIGUE, "Fatigue stress range", "Ermüdungsspannungsschwingbreite"),
    (LAYER_STRUT_AND_TIE, "Strut-and-tie model", "Stabwerkmodell"),
    (LAYER_GRADES, "Strength grades", "Festigkeitsklassen"),
    (LAYER_BOND, "Bond, anchorage and lap zones", "Verbund-, Verankerungs- und Übergreifungsbereiche"),
    (LAYER_CASES, "Design cases", "Bemessungsfälle"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
];

/// 🏷️ One entity kind of the EN 1992 outliner, interaction domain and insert palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityKindSpec {
    pub id: &'static str,
    pub collection: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub icon: &'static str,
    pub scene_kind: EntityKind,
}

/// 🏷️ The single registry of EN 1992 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    EntityKindSpec { id: "member", collection: "members", label_en: "RC member", label_de: "Stahlbetonbauteil", icon: "minus", scene_kind: EntityKind::Member },
    EntityKindSpec { id: "anchor", collection: "anchors", label_en: "Anchor", label_de: "Dübel", icon: "anchor", scene_kind: EntityKind::Fastener },
    EntityKindSpec { id: "case", collection: "cases", label_en: "Design case", label_de: "Bemessungsfall", icon: "clipboard-check", scene_kind: EntityKind::Solid },
    EntityKindSpec { id: "concreteGrade", collection: "concreteGrades", label_en: "Concrete grade", label_de: "Betonklasse", icon: "layers", scene_kind: EntityKind::Group },
    EntityKindSpec { id: "reinforcementGrade", collection: "reinforcementGrades", label_en: "Reinforcement grade", label_de: "Bewehrungsklasse", icon: "grip-lines", scene_kind: EntityKind::Group },
    EntityKindSpec { id: "prestressSteel", collection: "prestressSteels", label_en: "Prestressing steel", label_de: "Spannstahl", icon: "cable", scene_kind: EntityKind::Group },
    EntityKindSpec { id: "placement", collection: "placements", label_en: "Placement", label_de: "Platzierung", icon: "map-pin", scene_kind: EntityKind::Group },
];

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &En1992Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let mut out: BTreeSet<String> = BTreeSet::new();
    if let Some(m) = document.members.iter().find(|m| m.id == base) {
        out.extend([m.concrete_grade_id.clone(), m.reinforcement_grade_id.clone(), m.prestress_steel_id.clone()].into_iter().filter(|s| !s.is_empty()));
    }
    out.remove(base);
    out.into_iter().collect()
}

pub fn rect_section(b: f64, h: f64) -> SectionShape {
    SectionShape::parametric(ProfileKind::Rect, &[("h", h.max(1e-3)), ("b", b.max(1e-3))])
}

/// 🔷️ Section shape of a member: the explicit outline polygon moved to its centroid when given, else the rectangle b × h.
pub fn member_section(m: &RcMember, b: f64, h: f64) -> SectionShape {
    if m.section_points.len() >= 3 {
        SectionShape::Polygon { polygon: Polygon2 { outer: Ring2 { points: m.outline().into_iter().map(|p| Vec2::new(p.0, p.1)).collect() }, holes: Vec::new() } }
    } else {
        rect_section(b, h)
    }
}

pub fn entity_ref(kind: EntityKind, id: &str, path: &str) -> EntityRef {
    EntityRef::new(kind, id, path)
}

/// 🧱️ Everything one scene entity needs before sealing.
pub struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub path: String,
    pub geometry: Geometry,
    pub role: MaterialRole,
    pub binding: Option<UtilisationBinding>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub handles: Vec<String>,
    pub deformation: Option<DeformationOverlay>,
    pub opacity: f64,
    pub ghost: bool,
    pub channel: Option<ColourChannel>,
}

impl Spec {
    pub fn new(kind: EntityKind, id: impl Into<String>, path: impl Into<String>, geometry: Geometry, role: MaterialRole, layer: &'static str, name: LocalizedCopy, detail: LocalizedCopy) -> Self {
        Self { kind, id: id.into(), path: path.into(), geometry, role, binding: None, name, detail, layer, handles: Vec::new(), deformation: None, opacity: 1.0, ghost: false, channel: None }
    }

    pub fn bound(mut self, binding: Option<UtilisationBinding>) -> Self {
        self.binding = binding;
        self
    }

    pub fn translucent(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn ghosted(mut self) -> Self {
        self.ghost = true;
        self
    }

    pub fn channel(mut self, channel: ColourChannel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn with_handles(mut self, handles: Vec<String>) -> Self {
        self.handles = handles;
        self
    }

    pub fn deformed(mut self, overlay: Option<DeformationOverlay>) -> Self {
        self.deformation = overlay;
        self
    }

    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, &self.path),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: self.channel.unwrap_or(if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role }),
                style: if failing { StyleState::Error } else if self.ghost { StyleState::Ghost } else { StyleState::Normal },
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
    pub handles: Vec<Handle>,
    owners: Vec<Ownership>,
    points: Vec<Vec3>,
    scope: Scope,
    owner: Option<EntityRef>,
}

#[derive(Clone, Default)]
enum Scope {
    #[default]
    All,
    Entity(String),
    Global,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one member or anchor, or the global rest, at a time.
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

    /// 🎯️ Whether the entity `id` is part of this projection; modules test it before building an entity's geometry.
    pub fn wants(&self, id: &str) -> bool {
        self.part().wants(id)
    }

    /// 📥️ Pushes an entity built outside the spec builder (the case forms of the projection kit); its document row is the case of its base id.
    pub fn push_entity(&mut self, entity: SceneEntity, points: &[Vec3]) {
        let id = base_id(&entity.entity.id);
        self.owner = Some(entity_ref(EntityKind::Solid, id, &format!("cases[id={id}]")));
        if !self.wants(&entity.entity.id) {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(entity);
    }

    pub fn push(&mut self, spec: Spec, points: &[Vec3]) {
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

/// 🧭️ The document entity a scene entity belongs to, from the collection its path starts with.
fn owner_of(spec: &Spec) -> EntityRef {
    let id = base_id(&spec.id);
    let collection = spec.path.split('[').next().unwrap_or_default();
    let kind = ENTITY_KINDS.iter().find(|k| k.collection == collection).map_or(spec.kind, |k| k.scene_kind);
    entity_ref(kind, id, &format!("{collection}[id={id}]"))
}

/// 🆔️ Every document entity in outliner order: the members, then the anchors, then the cases `declared` accepts; their sub-entities (`id:tag`) are projected with them.
pub fn entities_where(document: &En1992Snapshot, declared: impl Fn(&CaseRow) -> bool) -> Vec<EntityRef> {
    let of = |collection: &'static str, ids: Vec<&str>| ENTITY_KINDS.iter().find(|k| k.collection == collection).into_iter().flat_map(|k| ids.into_iter().map(|id| entity_ref(k.scene_kind, id, &format!("{collection}[id={id}]")))).collect::<Vec<_>>();
    [of("members", document.members.iter().map(|e| e.id.as_str()).collect()), of("anchors", document.anchors.iter().map(|e| e.id.as_str()).collect()), of("cases", document.cases.iter().filter(|e| declared(e)).map(|e| e.id.as_str()).collect())].concat()
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments and the description; the field tracks are added by the scene root.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let fields = std::mem::take(&mut scene.legends);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[LAYER_GRADES]);
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
            MaterialRole::Concrete => ("Concrete", "Beton"),
            MaterialRole::Rebar => ("Reinforcement", "Bewehrung"),
            MaterialRole::Steel => ("Prestressing steel and anchors", "Spannstahl und Dübel"),
            MaterialRole::Support => ("Supports", "Lager"),
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

/// 🌈️ The scalar legend of a utilisation field with the viridis ramp 0 to 1,2, pushed once per field id.
pub fn field_legend(legends: &mut Vec<Legend>, id: &str, title: LocalizedCopy) {
    if !legends.iter().any(|l| l.id == id) {
        legends.push(Legend { id: id.into(), title, scale: LegendScale::Scalar { field_id: id.into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: 1.2 }, unit: QuantityKind::Dimensionless } });
    }
}

pub fn mm(meters: f64) -> String {
    format!("{:.0}", meters * 1000.0)
}

pub fn kn(newton: f64) -> String {
    format!("{:.1}", newton / 1000.0)
}

pub fn load_kind(kind: semio_s_artifact_norm_en1992_model::ActionKind) -> LoadKind {
    match kind {
        semio_s_artifact_norm_en1992_model::ActionKind::Permanent => LoadKind::Permanent,
        semio_s_artifact_norm_en1992_model::ActionKind::Imposed => LoadKind::Variable,
        semio_s_artifact_norm_en1992_model::ActionKind::Wind => LoadKind::Wind,
        semio_s_artifact_norm_en1992_model::ActionKind::Snow => LoadKind::Snow,
        semio_s_artifact_norm_en1992_model::ActionKind::Accidental => LoadKind::Accidental,
        semio_s_artifact_norm_en1992_model::ActionKind::Prestress => LoadKind::Prestress,
    }
}
