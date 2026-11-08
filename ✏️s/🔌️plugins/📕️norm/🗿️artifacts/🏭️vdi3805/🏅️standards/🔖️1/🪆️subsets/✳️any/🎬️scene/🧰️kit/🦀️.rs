//! 🧰️ Scene kit: the entity builder, the utilisation bindings from the report, layers, legends and the entity-kind registry of the VDI 3805 scene.

use super::scene::*;
pub(crate) use super::scene::projection_kit::{self, base_id, loc, v3, along, entity_ref, clause_of, tone_of, utilisation_words};
use super::scene::projection::{Fragment, Part};
use super::{model, norm};

pub(crate) const LAYER_PRODUCTS: &str = "products";
pub(crate) const LAYER_GEOMETRY: &str = "geometry";
pub(crate) const LAYER_ANNOTATIONS: &str = "annotations";

const LAYERS: [(&str, &str, &str); 3] = [
    (LAYER_PRODUCTS, "Catalogue products", "Katalogprodukte"),
    (LAYER_GEOMETRY, "Parametric geometry", "Parametrische Geometrie"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
];

/// 🏷️ One entity kind of the VDI 3805 outliner, interaction domain and insert palette.
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

/// 🏷️ The document path of the catalogue products.
pub const PRODUCTS: &str = "catalog.products";
/// 🏷️ The document path of the id-keyed parametric geometries.
pub const GEOMETRY: &str = "geometry";
/// 🏷️ The document path of the id-keyed characteristic curves.
pub const CURVES: &str = "curves";

/// 🏷️ The single registry of VDI 3805 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("product", PRODUCTS, "Catalogue product", "Katalogprodukt", "package", EntityKind::Solid),
    kind("geometry", GEOMETRY, "Parametric geometry", "Parametrische Geometrie", "box", EntityKind::Solid),
];

fn rows_of(d: &model::Vdi3805Snapshot) -> [(&'static str, Vec<&str>); 2] {
    [(PRODUCTS, d.catalog.products.iter().map(|p| p.id.as_str()).collect()), (GEOMETRY, d.geometry.keys().map(String::as_str).collect())]
}

/// 🧭️ The registry row owning an entity id, searching both entity collections of the document.
pub fn entity_kind_of(document: &model::Vdi3805Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    rows_of(document).iter().find(|(_, ids)| ids.contains(&base)).and_then(|(collection, _)| ENTITY_KINDS.iter().find(|k| k.collection == *collection))
}

/// 🆔️ Every document entity in outliner order: the canonical order of the scene.
pub fn entities_of(document: &model::Vdi3805Snapshot) -> Vec<EntityRef> {
    rows_of(document).iter().flat_map(|(collection, ids)| ENTITY_KINDS.iter().find(|k| k.collection == *collection).into_iter().flat_map(move |k| ids.iter().map(move |id| entity_ref(k.scene_kind, id, collection)))).collect()
}

/// 🔗️ The entities a projection of `id` reads besides itself: a product reads the geometry it references, a geometry the products that reference it.
pub fn entity_dependencies(document: &model::Vdi3805Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let mut out: Vec<String> = Vec::new();
    if let Some(product) = document.catalog.products.iter().find(|p| p.id == base) {
        out.extend(product.configuration.geometry_ref.iter().filter(|key| document.geometry.contains_key(*key)).cloned());
    }
    if document.geometry.contains_key(base) {
        out.extend(document.catalog.products.iter().filter(|p| p.configuration.geometry_ref.as_deref() == Some(base)).map(|p| p.id.clone()));
    }
    out.retain(|other| other != base);
    out.sort();
    out.dedup();
    out
}

/// 🧭️ Where a check is drawn: on one document row, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The document row that owns `check`: the product or geometry its subject names; `None` for a catalogue-level check.
pub fn owner_entity(document: &model::Vdi3805Snapshot, check: &norm::CheckResult) -> Option<String> {
    let subject = base_id(&check.subject.entity_id);
    (!subject.is_empty() && entity_kind_of(document, subject).is_some()).then(|| subject.to_owned())
}

/// 🧭️ The route of `check`: its owner row; else the document badge when the check names no row or a characteristic curve (the curves are drawn in the diagrams, the catalogue-level verdicts on the badge); `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::Vdi3805Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    owner_entity(document, check).map(CheckRoute::Entity).or_else(|| {
        let subject = check.subject.entity_id.as_str();
        (subject.is_empty() || document.curves.contains_key(subject)).then_some(CheckRoute::Document)
    })
}

/// 🎯️ The row to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`].
pub fn focus_of_outcome(document: &model::Vdi3805Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    projection_kit::check_with(report, check_id).and_then(|check| owner_entity(document, check))
}

/// 📶️ The governing binding among the checks owned by any of the rows `ids`.
pub fn binding(document: &model::Vdi3805Snapshot, report: &norm::CheckReport, ids: &[&str]) -> Option<UtilisationBinding> {
    projection_kit::binding_where(report, |check| owner_entity(document, check).is_some_and(|owner| ids.contains(&owner.as_str())))
}

/// 🧱️ Everything one scene entity needs before sealing.
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
                channel: if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role },
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

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends and the description.
pub(crate) fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Product groups", "Produktgruppen"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.description = description;
}

fn role_legend(entities: &[SceneEntity]) -> Vec<LegendRole> {
    projection_kit::legends::role_legend(entities, |role| {
        let label = match role {
            MaterialRole::Steel => ("Valves and pumps", "Armaturen und Pumpen"),
            MaterialRole::Neutral => ("Radiators and other products", "Heizkörper und sonstige Produkte"),
            MaterialRole::Masonry => ("Heat generators", "Wärmeerzeuger"),
            MaterialRole::Glass => ("Parametric geometry", "Parametrische Geometrie"),
            _ => return None,
        };
        Some((label, role.clone()))
    })
}
