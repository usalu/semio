//! 🧰️ Scene kit: the entity builder, utilisation bindings from the report, layers, legends and the entity-kind registry of DIN V 18599.

use super::*;
use scene::*;
pub use scene::projection_kit::{self, base_id, loc, v3, entity_ref, check_with, utilisation_words, binding_of_subject as binding};
use scene::projection::{Fragment, Part};

pub const LAYER_ZONES: &str = "zones";
pub const LAYER_OPAQUE: &str = "opaque";
pub const LAYER_GLAZING: &str = "glazing";

const LAYERS: [(&str, &str, &str); 3] = [
    (LAYER_ZONES, "Thermal zones", "Thermische Zonen"),
    (LAYER_OPAQUE, "Opaque envelope", "Opake Hülle"),
    (LAYER_GLAZING, "Windows and doors", "Fenster und Türen"),
];

/// 🏷️ One entity kind of the DIN V 18599 outliner, interaction domain and insert palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityKindSpec {
    pub id: &'static str,
    pub collection: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub icon: &'static str,
    pub scene_kind: EntityKind,
}

/// 🏷️ The single registry of DIN V 18599 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    EntityKindSpec { id: "zone", collection: "zones", label_en: "Thermal zone", label_de: "Thermische Zone", icon: "home", scene_kind: EntityKind::Solid },
    EntityKindSpec { id: "element", collection: "elements", label_en: "Envelope element", label_de: "Hüllbauteil", icon: "square", scene_kind: EntityKind::Solid },
];

/// 🧭️ The registry row owning an entity id.
pub fn entity_kind_of(document: &model::Din18599Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    let collection = if document.zones.iter().any(|e| e.id == base) {
        "zones"
    } else if document.elements.iter().any(|e| e.id == base) {
        "elements"
    } else {
        return None;
    };
    ENTITY_KINDS.iter().find(|k| k.collection == collection)
}

/// 🔗️ The entities a projection of `id` reads besides itself: an element stands on its zone.
pub fn entity_dependencies(document: &model::Din18599Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    document.elements.iter().find(|e| e.id == base).map(|e| e.zone_id.clone()).filter(|zone| zone != base).into_iter().collect()
}

/// 🆔️ Every document entity in outliner order: the zones, then the envelope elements.
pub fn entities_of(document: &model::Din18599Snapshot) -> Vec<EntityRef> {
    let zones = document.zones.iter().map(|e| entity_ref(EntityKind::Solid, &e.id, "zones"));
    let elements = document.elements.iter().map(|e| entity_ref(EntityKind::Solid, &e.id, "elements"));
    zones.chain(elements).collect()
}

/// 🧱️ Everything one scene entity needs before sealing.
pub struct Spec {
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
    pub fn new(id: String, collection: &'static str, geometry: Geometry, role: MaterialRole, name: LocalizedCopy, layer: &'static str) -> Self {
        Self { id, collection, geometry, role, binding: None, detail: name.clone(), name, layer, handles: Vec::new(), opacity: 1.0 }
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

    pub fn translucent(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        SceneEntity {
            entity: entity_ref(EntityKind::Solid, &self.id, self.collection),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role },
                style: if failing { StyleState::Error } else { StyleState::Normal },
                opacity: self.opacity,
                hatch: if failing { Some(Hatch::Diagonal) } else { None },
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

/// 📥️ Append-only scene accumulator of one projection part: entities in document order and the points that bound the part.
#[derive(Default)]
pub struct Out {
    pub entities: Vec<SceneEntity>,
    points: Vec<Vec3>,
    part_id: Option<String>,
    global: bool,
}

impl Out {
    /// 🎯️ An accumulator that keeps only what belongs to `part`.
    pub fn scoped(part: Part<'_>) -> Self {
        match part {
            Part::All => Self::default(),
            Part::Entity(id) => Self { part_id: Some(id.to_owned()), ..Self::default() },
            Part::Global => Self { global: true, ..Self::default() },
        }
    }

    /// 🎯️ Whether the entity `id` is part of this projection.
    pub fn wants(&self, id: &str) -> bool {
        match (&self.part_id, self.global) {
            (_, true) => false,
            (Some(only), _) => base_id(id) == only,
            _ => true,
        }
    }

    pub fn push(&mut self, spec: Spec, points: &[Vec3]) {
        if self.wants(&spec.id) {
            self.points.extend_from_slice(points);
            self.entities.push(spec.build());
        }
    }

    pub fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: Vec::new(), legends: Vec::new(), layers: Vec::new(), tracks: Vec::new(), owners: Vec::new(), bounds }
    }
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends and the description.
pub fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ZONES);
    let roles = projection_kit::legends::role_legend(&scene.entities, |role| {
        let label = match role {
            MaterialRole::Masonry => ("Walls", "Wände"),
            MaterialRole::Concrete => ("Roofs and floors", "Dächer und Böden"),
            MaterialRole::Glass => ("Windows", "Fenster"),
            MaterialRole::Timber => ("Doors", "Türen"),
            MaterialRole::Neutral => ("Thermal zones", "Thermische Zonen"),
            _ => return None,
        };
        Some((label, role.clone()))
    });
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation of the energy checks", "Ausnutzung der Energienachweise")),
        Legend { id: "roles".into(), title: loc("Building parts", "Bauteile"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.description = description;
}
