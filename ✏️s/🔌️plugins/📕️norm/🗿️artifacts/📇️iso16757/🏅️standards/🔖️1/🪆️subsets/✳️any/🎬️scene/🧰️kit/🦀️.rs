//! 🧰️ Scene kit: the mesh builder, the entity spec, the append-only accumulator, the entity-kind registry and the global scene parts.

use super::layout::Pose;
use super::model::part_2::solid::{Primitive, Shape};
use super::scene::*;
pub(crate) use scene::projection_kit::{self, base_id, loc, v3, binding_of_subjects as binding};
use super::*;
use scene::projection::{Fragment, Part};

pub(crate) const LAYER_BODY: &str = "body";
pub(crate) const LAYER_SPACES: &str = "spaces";
pub(crate) const LAYER_PORTS: &str = "ports";
pub(crate) const LAYER_SURFACES: &str = "surfaces";
pub(crate) const LAYER_ANNOTATIONS: &str = "annotations";

const LAYERS: [(&str, &str, &str); 5] = [
    (LAYER_BODY, "Product body", "Produktkörper"),
    (LAYER_SPACES, "Installation, operation and access spaces", "Einbau-, Bedien- und Zugangsräume"),
    (LAYER_PORTS, "Ports", "Anschlüsse"),
    (LAYER_SURFACES, "Semantic surfaces", "Semantische Flächen"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
];

pub(crate) const SPACE_TAG: &str = "space-";
pub(crate) const PORT_TAG: &str = "port-";
pub(crate) const SURFACE_TAG: &str = "surface-";

/// 🏷️ One entity kind of the ISO 16757 outliner and interaction domain.
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

/// 🏷️ The single registry of ISO 16757 entity kinds: the catalogue tree first, then the geometry objects the scene draws.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("productGroup", "productGroups", "Product group", "Produktgruppe", "folder", EntityKind::Group),
    kind("productClass", "productClasses", "Product class", "Produktklasse", "folder-tree", EntityKind::Group),
    kind("productSeries", "productSeries", "Product series", "Produktserie", "layers", EntityKind::Group),
    kind("product", "products", "Product", "Produkt", "package", EntityKind::Group),
    kind("productIndex", "productIndexes", "Product index", "Produktindex", "search", EntityKind::Group),
    kind("propertyDefinition", "propertyDefinitions", "Property definition", "Merkmalsdefinition", "tag", EntityKind::Group),
    kind("geometryObject", "geometryObjects", "Geometry object", "Geometrieobjekt", "box", EntityKind::Solid),
];

/// 🧭️ The registry row owning a scene entity id: a geometry object or one of its spaces, ports or surfaces.
pub fn entity_kind_of(document: &model::Iso16757Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    document.geometry.objects.contains_key(base_id(id)).then(|| &ENTITY_KINDS[ENTITY_KINDS.len() - 1])
}

/// 🆔️ Every drawn document entity in document order: the geometry objects, whose sub-entities (`object:tag`) are projected with them.
pub fn entities_of(document: &model::Iso16757Snapshot) -> Vec<EntityRef> {
    document.geometry.objects.keys().map(|id| object_ref(id)).collect()
}

/// 🔗️ The geometry objects a projection of `id` reads besides itself: the objects its shape references.
pub fn entity_dependencies(document: &model::Iso16757Snapshot, id: &str) -> Vec<String> {
    let mut out = Vec::new();
    if id.contains(':') {
        return vec![base_id(id).to_owned()];
    }
    if let Some(shape) = document.geometry.objects.get(id).and_then(|o| o.shape.as_ref()) {
        references(shape, &mut out);
    }
    out.retain(|r| r != id);
    out.sort();
    out.dedup();
    out
}

fn references(node: &model::part_2::GeometryNode, out: &mut Vec<String>) {
    match node {
        model::part_2::GeometryNode::Primitive { .. } => {}
        model::part_2::GeometryNode::Transform { child, .. } => references(child, out),
        model::part_2::GeometryNode::Boolean { children, .. } => children.iter().for_each(|c| references(c, out)),
        model::part_2::GeometryNode::Reference { geometry_id } => out.push(geometry_id.clone()),
    }
}

pub(crate) fn object_ref(id: &str) -> EntityRef {
    EntityRef::new(EntityKind::Solid, id, format!("geometry.objects.{id}"))
}

pub(crate) fn sub_ref(kind: EntityKind, id: &str, path: String) -> EntityRef {
    EntityRef::new(kind, id, path)
}

/// 🧱️ Accumulating triangle mesh in world coordinates.
#[derive(Default)]
pub(crate) struct MeshBuilder {
    vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
}

impl MeshBuilder {
    pub(crate) fn cuboid(&mut self, pose: &Pose, min: [f64; 3], max: [f64; 3]) {
        self.cuboid_with(&|q| pose.at(q[0], q[1], q[2]), min, max);
    }

    fn cuboid_with(&mut self, at: &dyn Fn([f64; 3]) -> Vec3, min: [f64; 3], max: [f64; 3]) {
        const QUADS: [[u32; 4]; 6] = [[0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [1, 2, 6, 5], [2, 3, 7, 6], [3, 0, 4, 7]];
        let base = self.vertices.len() as u32;
        for z in [min[2], max[2]] {
            for (x, y) in [(min[0], min[1]), (max[0], min[1]), (max[0], max[1]), (min[0], max[1])] {
                self.vertices.push(at([x, y, z]));
            }
        }
        for q in QUADS {
            self.triangles.push(Triangle { a: base + q[0], b: base + q[1], c: base + q[2] });
            self.triangles.push(Triangle { a: base + q[0], b: base + q[2], c: base + q[3] });
        }
    }

    /// 🧱️ One primitive in its world frame (rotation about the local origin, then translation), drawn with the shared axis convention of [`model::part_2::solid`].
    pub(crate) fn primitive(&mut self, pose: &Pose, p: &Primitive) {
        let at = |q: [f64; 3]| {
            let w = [0, 1, 2].map(|i| p.rot[i][0] * q[0] + p.rot[i][1] * q[1] + p.rot[i][2] * q[2] + p.origin[i]);
            pose.at(w[0], w[1], w[2])
        };
        match p.shape {
            Shape::Box(size) => self.cuboid_with(&at, [0.0; 3], size),
            Shape::Cylinder(radius, height) => self.cylinder_with(&|u, v, level| at([u, level, 2.0 * radius - v]), [radius, radius], radius, (0.0, height)),
            Shape::Sphere(radius) => self.sphere_with(&at, [radius; 3], radius),
        }
    }

    fn cylinder_with(&mut self, at: &dyn Fn(f64, f64, f64) -> Vec3, centre: [f64; 2], radius: f64, z: (f64, f64)) {
        const SIDES: u32 = 16;
        let base = self.vertices.len() as u32;
        for level in [z.0, z.1] {
            for k in 0..SIDES {
                let a = std::f64::consts::TAU * f64::from(k) / f64::from(SIDES);
                self.vertices.push(at(centre[0] + radius * a.cos(), centre[1] + radius * a.sin(), level));
            }
        }
        for k in 0..SIDES {
            let n = (k + 1) % SIDES;
            self.triangles.push(Triangle { a: base + k, b: base + n, c: base + SIDES + n });
            self.triangles.push(Triangle { a: base + k, b: base + SIDES + n, c: base + SIDES + k });
        }
        for k in 1..SIDES - 1 {
            self.triangles.push(Triangle { a: base, b: base + k + 1, c: base + k });
            self.triangles.push(Triangle { a: base + SIDES, b: base + SIDES + k, c: base + SIDES + k + 1 });
        }
    }

    fn sphere_with(&mut self, at: &dyn Fn([f64; 3]) -> Vec3, centre: [f64; 3], radius: f64) {
        const RINGS: u32 = 6;
        const SIDES: u32 = 12;
        let base = self.vertices.len() as u32;
        for r in 0..=RINGS {
            let phi = std::f64::consts::PI * f64::from(r) / f64::from(RINGS);
            for k in 0..SIDES {
                let a = std::f64::consts::TAU * f64::from(k) / f64::from(SIDES);
                self.vertices.push(at([centre[0] + radius * phi.sin() * a.cos(), centre[1] + radius * phi.sin() * a.sin(), centre[2] + radius * phi.cos()]));
            }
        }
        for r in 0..RINGS {
            for k in 0..SIDES {
                let n = (k + 1) % SIDES;
                let (a, b, c, d) = (base + r * SIDES + k, base + r * SIDES + n, base + (r + 1) * SIDES + n, base + (r + 1) * SIDES + k);
                self.triangles.push(Triangle { a, b, c });
                self.triangles.push(Triangle { a, b: c, c: d });
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }

    pub(crate) fn points(&self) -> &[Vec3] {
        &self.vertices
    }

    pub(crate) fn solid(self) -> Geometry {
        Geometry::Solid(SolidGeometry { shape: SolidShape::Mesh { vertices: self.vertices, triangles: self.triangles } })
    }
}

/// 🧱️ Everything one scene entity needs before sealing.
pub(crate) struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub path: String,
    pub geometry: Geometry,
    pub role: MaterialRole,
    pub binding: Option<UtilisationBinding>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub opacity: f64,
}

impl Spec {
    pub(crate) fn new(kind: EntityKind, id: impl Into<String>, path: String, geometry: Geometry, role: MaterialRole, layer: &'static str, name: LocalizedCopy) -> Self {
        Self { kind, id: id.into(), path, geometry, role, binding: None, detail: name.clone(), name, layer, opacity: 1.0 }
    }

    pub(crate) fn build(self) -> SceneEntity {
        let (more_en, more_de) = projection_kit::utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        SceneEntity {
            entity: sub_ref(self.kind, &self.id, self.path),
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
            handles: Vec::new(),
            content_hash: String::new(),
        }
        .sealed()
    }
}

/// 📥️ Append-only scene accumulator of one projection part.
pub(crate) struct Out {
    pub entities: Vec<SceneEntity>,
    pub annotations: Vec<Annotation>,
    owners: Vec<Ownership>,
    points: Vec<Vec3>,
    part: OwnedPart,
    owner: Option<EntityRef>,
}

enum OwnedPart {
    All,
    Entity(String),
    Global,
}

impl Out {
    pub(crate) fn scoped(part: Part<'_>) -> Self {
        let part = match part {
            Part::All => OwnedPart::All,
            Part::Entity(id) => OwnedPart::Entity(id.to_owned()),
            Part::Global => OwnedPart::Global,
        };
        Self { entities: Vec::new(), annotations: Vec::new(), owners: Vec::new(), points: Vec::new(), part, owner: None }
    }

    fn part(&self) -> Part<'_> {
        match &self.part {
            OwnedPart::All => Part::All,
            OwnedPart::Entity(id) => Part::Entity(id),
            OwnedPart::Global => Part::Global,
        }
    }

    pub(crate) fn wants(&self, id: &str) -> bool {
        self.part().wants(id)
    }

    pub(crate) fn push(&mut self, spec: Spec, points: &[Vec3]) {
        self.owner = Some(object_ref(base_id(&spec.id)));
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
    let roles = projection_kit::legends::role_legend(&scene.entities, |role| {
        let label = match role {
            MaterialRole::Neutral => ("Product body", "Produktkörper"),
            MaterialRole::Support => ("Spaces", "Räume"),
            MaterialRole::Steel => ("Ports", "Anschlüsse"),
            MaterialRole::Glass => ("Surfaces", "Flächen"),
            _ => return None,
        };
        Some((label, role.clone()))
    });
    let fields = std::mem::take(&mut scene.legends);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[]);
    scene.legends = vec![projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")), Legend { id: "roles".into(), title: loc("Materials", "Werkstoffe"), scale: LegendScale::Roles { entries: roles } }];
    scene.legends.extend(fields);
    scene.description = description;
}
