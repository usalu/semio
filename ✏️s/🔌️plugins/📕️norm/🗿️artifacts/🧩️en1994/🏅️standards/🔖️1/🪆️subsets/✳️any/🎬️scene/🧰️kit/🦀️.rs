//! 🧰️ Scene kit: vector helpers, the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::*;
use scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, add, mul, entity_ref, polyline, envelope, field_along, clause_of, tone_of, utilisation_words, check_with, binding_of_subject as binding, worst_of_subject as worst};
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

pub const LAYER_MODE_BUCKLING: &str = "mode-buckling";
pub const LAYER_MODE_LTB: &str = "mode-ltb";
pub const LAYER_STEEL: &str = "steel";
pub const LAYER_SLAB: &str = "slab";
pub const LAYER_SHEETING: &str = "sheeting";
pub const LAYER_CONNECTORS: &str = "connectors";
pub const LAYER_REINFORCEMENT: &str = "reinforcement";
pub const LAYER_COLUMNS: &str = "columns";
pub const LAYER_ANALYSIS: &str = "analysis";
pub const LAYER_LOADS: &str = "loads";
pub const LAYER_SUPPORTS: &str = "supports";
pub const LAYER_ANNOTATIONS: &str = "annotations";
pub const LAYER_SLIP: &str = "slip";
pub const LAYER_FIELDS: &str = "fields";
pub const LAYER_CASES: &str = "cases";

const LAYERS: [(&str, &str, &str); 15] = [
    (LAYER_CASES, "Design cases", "Bemessungsfälle"),
    (LAYER_FIELDS, "Temperature and crack-width fields", "Temperatur- und Rissbreitenfelder"),
    (LAYER_SLIP, "Interface slip arrows", "Fugenschlupf-Pfeile"),
    (LAYER_STEEL, "Steel sections", "Stahlprofile"),
    (LAYER_SLAB, "Concrete slabs", "Betongurte"),
    (LAYER_SHEETING, "Profiled sheeting", "Profilbleche"),
    (LAYER_CONNECTORS, "Shear studs", "Kopfbolzendübel"),
    (LAYER_REINFORCEMENT, "Reinforcement", "Bewehrung"),
    (LAYER_COLUMNS, "Composite columns", "Verbundstützen"),
    (LAYER_ANALYSIS, "Neutral axis and effective width", "Nulllinie und mitwirkende Breite"),
    (LAYER_LOADS, "Loads", "Lasten"),
    (LAYER_SUPPORTS, "Supports", "Lager"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
    (LAYER_MODE_BUCKLING, "Column buckling mode", "Stützenknickform"),
    (LAYER_MODE_LTB, "Lateral-torsional buckling mode", "Biegedrillknickform"),
];

/// 🏷️ One entity kind of the EN 1994 outliner, interaction domain and insert palette.
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

/// 🏷️ The single registry of EN 1994 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("beam", "beams", "Composite beam", "Verbundträger", "minus", EntityKind::Member),
    kind("column", "columns", "Composite column", "Verbundstütze", "square", EntityKind::Member),
    kind("slab", "slabs", "Composite slab", "Verbunddecke", "layers", EntityKind::Plate),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
    kind("case", "cases", "Design case", "Bemessungsfall", "clipboard-check", EntityKind::Solid),
];

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::En1994Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    let collection = if document.beams.iter().any(|e| e.id == base) {
        "beams"
    } else if document.columns.iter().any(|e| e.id == base) {
        "columns"
    } else if document.slabs.iter().any(|e| e.id == base) {
        "slabs"
    } else if document.placements.iter().any(|e| e.id == base) {
        "placements"
    } else if document.cases.iter().any(|e| e.id == base && declared(e)) {
        "cases"
    } else {
        return None;
    };
    ENTITY_KINDS.iter().find(|k| k.collection == collection)
}

/// 🧊️ Whether the entry of `row` declares geometry, so the design case stands in the scene.
pub fn declared(row: &model::CaseRow) -> bool {
    projection_kit::design_case::forms::declared::<checks::case_catalogue::En1994Cases>(row).is_some()
}

/// 🧭️ The scene reference of a design case.
pub fn case_ref(case: &str) -> EntityRef {
    entity_ref(EntityKind::Solid, case, "cases")
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1994Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let mut out: BTreeSet<String> = BTreeSet::new();
    if let Some(b) = document.beams.iter().find(|e| e.id == base) {
        out.extend(b.actions.iter().map(|a| a.id.clone()));
        out.extend(document.studs.iter().filter(|s| s.beam_id == b.id).map(|s| s.id.clone()));
    }
    if let Some(c) = document.columns.iter().find(|e| e.id == base) {
        out.extend(c.actions.iter().map(|a| a.id.clone()));
    }
    if let Some(s) = document.slabs.iter().find(|e| e.id == base) {
        out.extend(s.actions.iter().map(|a| a.id.clone()));
    }
    out.remove(base);
    out.into_iter().collect()
}

pub fn v2(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

pub fn rect(x0: f64, x1: f64, y0: f64, y1: f64) -> Polygon2 {
    Polygon2::solid(vec![v2(x0, y0), v2(x1, y0), v2(x1, y1), v2(x0, y1)])
}

/// 🔁️ A binding scaled to a share of its governing ratio (the shear-flow indicator of one stud zone).
pub fn scaled(binding: &Option<UtilisationBinding>, share: f64) -> Option<UtilisationBinding> {
    binding.as_ref().map(|b| {
        let ratio = b.ratio * share;
        let status = match b.status {
            scene::CheckStatus::NotApplicable => b.status,
            _ if !scene::norm::within_limit(ratio) => scene::CheckStatus::Fail,
            _ if ratio > 0.8 => scene::CheckStatus::Warning,
            _ => scene::CheckStatus::Pass,
        };
        UtilisationBinding { check_id: b.check_id.clone(), ratio, status, per_station: None, stale: b.stale }
    })
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
    pub opacity: f64,
    pub ghost: bool,
    pub channel: Option<ColourChannel>,
}

impl Spec {
    pub fn new(kind: EntityKind, id: String, collection: &'static str, geometry: Geometry, role: MaterialRole, name: LocalizedCopy, layer: &'static str) -> Self {
        Self { kind, id, collection, geometry, role, binding: None, detail: name.clone(), name, layer, handles: Vec::new(), deformation: None, opacity: 1.0, ghost: false, channel: None }
    }

    pub fn bound(mut self, binding: Option<UtilisationBinding>) -> Self {
        self.binding = binding;
        self
    }

    pub fn detailed(mut self, detail: LocalizedCopy) -> Self {
        self.detail = detail;
        self
    }

    pub fn with_handles(mut self, handles: Vec<String>) -> Self {
        self.handles = handles;
        self
    }

    pub fn channel(mut self, channel: ColourChannel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn translucent(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn ghosted(mut self, opacity: f64) -> Self {
        self.ghost = true;
        self.opacity = opacity;
        self
    }

    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, self.collection),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: self.channel.unwrap_or(if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role }),
                style: if failing { StyleState::Error } else if self.ghost { StyleState::Ghost } else { StyleState::Normal },
                opacity: self.opacity,
                hatch: if failing { Some(Hatch::Diagonal) } else if self.ghost { Some(Hatch::Dots) } else { None },
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
}

#[derive(Clone, Default)]
enum Scope {
    #[default]
    All,
    Entity(String),
    Global,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one beam, column or slab, or the global rest, at a time.
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
        self.owner = Some(case_ref(base_id(&entity.entity.id)));
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

/// 🧭️ The registry entity a scene entity belongs to: its document row, whatever sub-entity of it is drawn.
fn owner_of(spec: &Spec) -> EntityRef {
    let id = base_id(&spec.id);
    ENTITY_KINDS.iter().find(|k| k.collection == spec.collection).map_or_else(|| entity_ref(spec.kind, id, spec.collection), |k| entity_ref(k.scene_kind, id, spec.collection))
}

/// 🆔️ Every document entity in outliner order: the beams, the columns and the slabs; their sub-entities (`id:tag`) are projected with them.
pub fn entities_of(document: &model::En1994Snapshot) -> Vec<EntityRef> {
    let of = |collection: &'static str, ids: Vec<&str>| ENTITY_KINDS.iter().find(|k| k.collection == collection).into_iter().flat_map(|k| ids.into_iter().map(|id| entity_ref(k.scene_kind, id, collection))).collect::<Vec<_>>();
    [of("beams", document.beams.iter().map(|e| e.id.as_str()).collect()), of("columns", document.columns.iter().map(|e| e.id.as_str()).collect()), of("slabs", document.slabs.iter().map(|e| e.id.as_str()).collect()), of("cases", document.cases.iter().filter(|e| declared(e)).map(|e| e.id.as_str()).collect())].concat()
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends, the description and the mode track.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let fields = std::mem::take(&mut scene.legends);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation, shear flow and interface slip", "Ausnutzung, Schubfluss und Fugenschlupf")),
        Legend { id: "roles".into(), title: loc("Materials", "Werkstoffe"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(fields);
    scene.description = description;
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Steel => ("Steel", "Stahl"),
            MaterialRole::Concrete => ("Concrete", "Beton"),
            MaterialRole::Rebar => ("Reinforcement", "Bewehrung"),
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
