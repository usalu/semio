//! 🧰️ Scene kit: the entity builder, the utilisation bindings from the report, layers, legends and the entity-kind registry of the DIN EN 16798 scene.

use super::scene::*;
pub(crate) use super::scene::projection_kit::{self, base_id, loc, v3, add, mul, unit, along, member_frame, entity_ref, clause_of, tone_of, utilisation_words, polyline};
use super::scene::projection::{Fragment, Part};
use super::{model, norm};

pub(crate) const LAYER_ROOMS: &str = "rooms";
pub(crate) const LAYER_VENTILATION: &str = "ventilation";
pub(crate) const LAYER_ANNOTATIONS: &str = "annotations";

const LAYERS: [(&str, &str, &str); 3] = [
    (LAYER_ROOMS, "Rooms", "Räume"),
    (LAYER_VENTILATION, "Ventilation systems", "Lüftungsanlagen"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
];

/// 🏷️ One entity kind of the DIN EN 16798 outliner, interaction domain and insert palette.
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

/// 🏷️ The single registry of DIN EN 16798 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("zone", "zones", "Room", "Raum", "home", EntityKind::Solid),
    kind("ventSystem", "ventSystems", "Ventilation system", "Lüftungsanlage", "wind", EntityKind::Solid),
];

fn rows_of(d: &model::Din16798Snapshot) -> [(&'static str, Vec<&str>); 2] {
    [
        ("zones", d.zones.iter().map(|e| e.id.as_str()).collect()),
        ("ventSystems", d.vent_systems.iter().map(|e| e.id.as_str()).collect()),
    ]
}

/// 🧭️ The registry row owning an entity id, searching every collection of the document.
pub fn entity_kind_of(document: &model::Din16798Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    rows_of(document).iter().find(|(_, ids)| ids.contains(&base)).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🆔️ Every document entity in outliner order: the canonical order of the scene.
pub fn entities_of(document: &model::Din16798Snapshot) -> Vec<EntityRef> {
    rows_of(document).iter().flat_map(|(collection, ids)| ENTITY_KINDS.iter().find(|k| k.collection == *collection).into_iter().flat_map(move |k| ids.iter().map(move |id| entity_ref(k.scene_kind, id, collection)))).collect()
}

/// 🔗️ The entities a projection of `id` reads besides itself: a zone reads the ventilation system it is linked to, a ventilation system reads the zones it serves.
pub fn entity_dependencies(document: &model::Din16798Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let mut out: Vec<String> = Vec::new();
    if let Some(zone) = document.zones.iter().find(|z| z.id == base) {
        out.push(zone.vent_system_id.clone());
    }
    if document.vent_systems.iter().any(|v| v.id == base) {
        out.extend(document.zones.iter().filter(|z| z.vent_system_id == base).map(|z| z.id.clone()));
    }
    out.retain(|other| other != base && rows_of(document).iter().any(|(_, ids)| ids.contains(&other.as_str())));
    out.sort();
    out.dedup();
    out
}

/// 🧭️ The check families that belong to the building as a whole (airtightness, infiltration, cellar ventilation, night set-back, the empty-zones verdict).
pub const DOCUMENT_CHECKS: &[&str] = &["din16798-7.", "din16798-5-1.", "din16798.zones.empty"];

/// 🧭️ Where a check is drawn: on one document row, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The document row that owns `check`: the row its subject names; `None` for a document-level check.
pub fn owner_entity(document: &model::Din16798Snapshot, check: &norm::CheckResult) -> Option<String> {
    let subject = base_id(&check.subject.entity_id);
    (!subject.is_empty() && entity_kind_of(document, subject).is_some()).then(|| subject.to_owned())
}

/// 🧭️ The route of `check`: its owner row, else the document when it sits in a document-level family; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::Din16798Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    owner_entity(document, check).map(CheckRoute::Entity).or_else(|| DOCUMENT_CHECKS.iter().any(|p| check.id.starts_with(p)).then_some(CheckRoute::Document))
}

/// 🎯️ The row to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`].
pub fn focus_of_outcome(document: &model::Din16798Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📶️ The governing binding among the checks owned by any of the rows `ids`.
pub fn binding(document: &model::Din16798Snapshot, report: &norm::CheckReport, ids: &[&str]) -> Option<UtilisationBinding> {
    projection_kit::binding_where(report, |check| owner_entity(document, check).is_some_and(|owner| ids.contains(&owner.as_str())))
}

/// 🧱️ Everything one scene entity needs before sealing; `channel` is the colour channel the entity is drawn with.
pub(crate) struct Spec {
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
}

impl Spec {
    pub(crate) fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, self.collection),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: ColourChannel::Role,
                style: if failing { StyleState::Error } else { StyleState::Normal },
                opacity: self.opacity,
                hatch: failing.then_some(Hatch::Diagonal),
                deformation: None,
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

#[derive(Clone, Default)]
enum Scope {
    #[default]
    All,
    Entity(String),
    Global,
}

/// 📥️ Append-only scene accumulator of one projection part: entities in document order, owned annotations and the points that bound the part.
#[derive(Default)]
pub(crate) struct Out {
    pub entities: Vec<SceneEntity>,
    pub annotations: Vec<Annotation>,
    owners: Vec<Ownership>,
    points: Vec<Vec3>,
    scope: Scope,
    owner: Option<EntityRef>,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`.
    pub(crate) fn scoped(part: Part<'_>) -> Self {
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

    pub(crate) fn wants(&self, id: &str) -> bool {
        self.part().wants(id)
    }

    pub(crate) fn push(&mut self, spec: Spec, points: &[Vec3]) {
        self.owner = Some(entity_ref(spec.kind, base_id(&spec.id), spec.collection));
        if !self.wants(&spec.id) {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(spec.build());
    }

    pub(crate) fn annotate(&mut self, annotation: Annotation, anchor: &Vec3) {
        if !self.part().keeps(self.owner.as_ref().map(|o| o.id.as_str())) {
            return;
        }
        if let Some(owner) = &self.owner {
            self.owners.push(Ownership { kind: OwnedKind::Annotation, id: annotation.id().to_owned(), owner: owner.clone() });
        }
        self.points.push(anchor.clone());
        self.annotations.push(annotation);
    }

    pub(crate) fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: Vec::new(), layers: Vec::new(), tracks: Vec::new(), owners: self.owners, bounds }
    }
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and category legends and the description.
pub(crate) fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "categories".into(), title: loc("Indoor environment category", "Kategorie des Innenraumklimas"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.description = description;
}

/// 🎨️ The role a comfort category is drawn with: I glass, II aluminium, III timber, IV masonry; a ventilation system is steel.
pub(crate) fn category_role(category: u8) -> MaterialRole {
    match category {
        1 => MaterialRole::Glass,
        2 => MaterialRole::Aluminium,
        3 => MaterialRole::Timber,
        _ => MaterialRole::Masonry,
    }
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Glass => ("Category I (highest expectation)", "Kategorie I (höchstes Anspruchsniveau)"),
            MaterialRole::Aluminium => ("Category II (normal expectation)", "Kategorie II (normales Anspruchsniveau)"),
            MaterialRole::Timber => ("Category III (moderate expectation)", "Kategorie III (mäßiges Anspruchsniveau)"),
            MaterialRole::Masonry => ("Category IV (outside the categories)", "Kategorie IV (außerhalb der Kategorien)"),
            MaterialRole::Steel => ("Ventilation system", "Lüftungsanlage"),
            _ => return None,
        };
        Some((label, role.clone()))
    })
}
