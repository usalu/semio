//! 🧰️ Scene kit: vector helpers, the mesh and entity builders, utilisation bindings from the report, layers, legends and the entity-kind registry.

use super::layout::WallPose;
use super::scene::*;
pub(crate) use scene::projection_kit::{self, base_id, loc, v3, clause_of, tone_of, utilisation_words, binding_where as binding_by, binding_of_subjects as binding};
use super::*;
use scene::projection::{Fragment, Part};
use std::collections::BTreeSet;

pub(crate) const LAYER_MASONRY: &str = "masonry";
pub(crate) const LAYER_OPENINGS: &str = "openings";
pub(crate) const LAYER_FLOORS: &str = "floors";
pub(crate) const LAYER_BOND: &str = "bond";
pub(crate) const LAYER_REINFORCEMENT: &str = "reinforcement";
pub(crate) const LAYER_LOADS: &str = "loads";
pub(crate) const LAYER_PATTERNS: &str = "patterns";
pub(crate) const LAYER_SUPPORTS: &str = "supports";
pub(crate) const LAYER_FIRE: &str = "fire";
pub(crate) const LAYER_CASES: &str = "cases";
pub(crate) const LAYER_ANNOTATIONS: &str = "annotations";

const LAYERS: [(&str, &str, &str); 11] = [
    (LAYER_MASONRY, "Masonry", "Mauerwerk"),
    (LAYER_OPENINGS, "Openings and lintels", "Öffnungen und Stürze"),
    (LAYER_FLOORS, "Floors", "Decken"),
    (LAYER_BOND, "Bond pattern", "Mauerverband"),
    (LAYER_REINFORCEMENT, "Reinforcement", "Bewehrung"),
    (LAYER_LOADS, "Loads", "Lasten"),
    (LAYER_PATTERNS, "Load spreading and yield lines", "Lastausbreitung und Bruchlinien"),
    (LAYER_SUPPORTS, "Supports", "Lager"),
    (LAYER_FIRE, "Fire thickness demand", "Brandschutz-Dickenbedarf"),
    (LAYER_CASES, "Design cases", "Bemessungsfälle"),
    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),
];

/// 🏷️ One entity kind of the EN 1996 outliner, interaction domain and insert palette.
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

/// 🏷️ The single registry of EN 1996 entity kinds, in outliner order.
pub const ENTITY_KINDS: &[EntityKindSpec] = &[
    kind("wall", "walls", "Wall", "Wand", "square", EntityKind::Group),
    kind("opening", "openings", "Opening", "Öffnung", "door-open", EntityKind::Plate),
    kind("concentratedLoad", "concentrated", "Concentrated load", "Einzellast", "arrow-down", EntityKind::Load),
    kind("lintel", "lintels", "Lintel", "Sturz", "panel-top", EntityKind::Solid),
    kind("pier", "piers", "Pier", "Pfeiler", "rectangle-vertical", EntityKind::Solid),
    kind("tie", "ties", "Tie or anchor", "Anker", "link", EntityKind::Fastener),
    kind("prestress", "prestress", "Tendon", "Spannglied", "activity", EntityKind::Fastener),
    kind("confined", "confined", "Confining column", "Einfassungsstütze", "columns", EntityKind::Solid),
    kind("flange", "flanges", "Flange wall", "Flanschwand", "panel-left", EntityKind::Solid),
    kind("bar", "bars", "Reinforcing bar", "Bewehrungsstab", "grip-vertical", EntityKind::Fastener),
    kind("chase", "chases", "Chase or recess", "Schlitz oder Aussparung", "square-dashed", EntityKind::Solid),
    kind("movementJoint", "movementJoints", "Movement joint", "Dehnfuge", "separator-vertical", EntityKind::Solid),
    kind("case", "cases", "Design case", "Bemessungsfall", "clipboard-check", EntityKind::Solid),
    kind("placement", "placements", "Placement", "Platzierung", "map-pin", EntityKind::Group),
];

pub(crate) const OPENING_TAG: &str = "opening-";
pub(crate) const LINTEL_TAG: &str = "lintel-";
pub(crate) const CONCENTRATED_TAG: &str = "conc-";
pub(crate) const PIER_TAG: &str = "pier-";
pub(crate) const TIE_TAG: &str = "tie-";
pub(crate) const TENDON_TAG: &str = "tendon-";
pub(crate) const COLUMN_TAG: &str = "column-";
pub(crate) const BAR_TAG: &str = "bar-";
pub(crate) const WEDGE_TAG: &str = "wedge-";
pub(crate) const BEARING_TAG: &str = "bearing-";
pub(crate) const ISOTHERM_TAG: &str = "isotherm-zone";
pub(crate) const PHI_TAG: &str = "phi-field";
pub(crate) const YIELD_TAG: &str = "yield-lines";
pub(crate) const ARCH_TAG: &str = "arch-thrust";
pub(crate) const ANNEX_E_TAG: &str = "annex-e";
pub(crate) const CHASE_TAG: &str = "chase-";
pub(crate) const JOINT_TAG: &str = "joint-";
pub(crate) const DIAPHRAGM_TAG: &str = "diaphragm-";

/// 🧭️ The registry row owning an entity id: a wall, one of its openings (`wall:opening-<id>`) or one of its concentrated loads (`wall:conc-<load case>-<id>`).
pub fn entity_kind_of(document: &model::En1996Snapshot, id: &str) -> Option<&'static EntityKindSpec> {
    let base = base_id(id);
    if document.cases.iter().any(|c| c.id == base && super::cases::declared(c)) {
        return ENTITY_KINDS.iter().find(|k| k.collection == "cases");
    }
    if !document.walls.iter().any(|w| w.id == base) {
        return document.placements.iter().any(|p| p.id == base).then(|| ENTITY_KINDS.iter().find(|k| k.collection == "placements")).flatten();
    }
    let tag = id.split_once(':').map_or("", |(_, tag)| tag);
    let collection = [(OPENING_TAG, "openings"), (LINTEL_TAG, "lintels"), (PIER_TAG, "piers"), (TIE_TAG, "ties"), (TENDON_TAG, "prestress"), (COLUMN_TAG, "confined"), (super::lateral::FLANGE_TAG, "flanges"), (BAR_TAG, "bars"), (CONCENTRATED_TAG, "concentrated"), (WEDGE_TAG, "concentrated"), (BEARING_TAG, "concentrated"), (CHASE_TAG, "chases"), (JOINT_TAG, "movementJoints")]
        .into_iter()
        .find(|(prefix, _)| tag.starts_with(prefix))
        .map_or("walls", |(_, collection)| collection);
    ENTITY_KINDS.iter().find(|k| k.collection == collection)
}

/// 🆔️ Every document entity in document order: the walls, whose sub-entities (`wall:tag`) are projected with them.
pub fn entities_of(document: &model::En1996Snapshot) -> Vec<EntityRef> {
    document.walls.iter().map(|w| wall_ref(&w.id)).chain(document.cases.iter().filter(|c| super::cases::declared(c)).map(|c| case_ref(&c.id))).collect()
}

/// 🔗️ The entities a projection of `id` reads besides itself, so a change of any of them re-projects `id`.
pub fn entity_dependencies(document: &model::En1996Snapshot, id: &str) -> Vec<String> {
    let base = base_id(id);
    let mut out: BTreeSet<String> = BTreeSet::new();
    if let Some(wall) = document.walls.iter().find(|w| w.id == base) {
        let tag = id.split_once(':').map(|(_, tag)| tag);
        match tag {
            None => {
                out.extend(wall.openings.iter().map(|o| o.id.clone()));
                out.extend(wall.lintels.iter().map(|x| x.id.clone()).chain(wall.piers.iter().map(|x| x.id.clone())).chain(wall.ties.iter().map(|x| x.id.clone())).chain(wall.prestress.iter().map(|x| x.id.clone())).chain(wall.confined.iter().map(|x| x.id.clone())).chain(wall.flanges.iter().map(|x| x.id.clone())).chain(wall.bars.iter().map(|x| x.id.clone())));
                out.extend(wall.load_cases.iter().flat_map(|lc| std::iter::once(lc.id.clone()).chain(lc.concentrated.iter().map(|c| c.id.clone()))));
                out.extend(document.chases.iter().filter(|c| c.wall_id == wall.id).map(|c| c.id.clone()));
                out.extend(document.movement_joints.iter().filter(|j| j.wall_id == wall.id).map(|j| j.id.clone()));
            }
            Some(tag) if tag.starts_with(DIAPHRAGM_TAG) => out.extend(document.walls.iter().map(|w| w.id.clone())),
            Some(tag) if tag.starts_with(CONCENTRATED_TAG) || tag.starts_with(WEDGE_TAG) || tag.starts_with(BEARING_TAG) => {
                let key = tag.strip_prefix(CONCENTRATED_TAG).or_else(|| tag.strip_prefix(WEDGE_TAG)).or_else(|| tag.strip_prefix(BEARING_TAG)).unwrap_or(tag);
                out.extend(wall.load_cases.iter().filter(|lc| lc.concentrated.iter().any(|c| key == format!("{}-{}", lc.id, c.id))).map(|lc| lc.id.clone()));
                out.insert(wall.id.clone());
            }
            Some(_) => {
                out.insert(wall.id.clone());
            }
        }
    }
    out.remove(id);
    out.into_iter().collect()
}

pub(crate) fn case_ref(case: &str) -> EntityRef {
    EntityRef::new(EntityKind::Solid, case, format!("cases[id={case}]"))
}

pub(crate) fn wall_ref(wall: &str) -> EntityRef {
    EntityRef::new(EntityKind::Group, wall, format!("walls[id={wall}]"))
}

pub(crate) fn entity_ref(kind: EntityKind, id: &str, path: &str) -> EntityRef {
    EntityRef::new(kind, id, path)
}

/// 📶️ The binding of the check with exactly this id.
pub(crate) fn binding_of(report: &norm::CheckReport, check_id: &str) -> Option<UtilisationBinding> {
    binding_by(report, |c| c.id == check_id)
}

/// 📶️ A binding for a ratio computed in the scene: status from the ratio, check id from the report.
pub(crate) fn computed_binding(check_id: String, ratio: f64) -> UtilisationBinding {
    let status = scene::utilisation_status(ratio, 0.9);
    UtilisationBinding { check_id, ratio: ratio.max(0.0), status, per_station: None, stale: false }
}

pub(crate) fn ring(points: &[Vec3]) -> Polygon3 {
    Polygon3 { outer: Ring3 { points: points.to_vec() }, holes: Vec::new() }
}

pub(crate) fn polygon2(points: &[(f64, f64)]) -> Polygon2 {
    Polygon2 { outer: Ring2 { points: points.iter().map(|p| Vec2::new(p.0, p.1)).collect() }, holes: Vec::new() }
}

pub(crate) fn surface(points: &[Vec3]) -> Geometry {
    Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: ring(points) } })
}

/// 🧱️ Accumulating triangle mesh of cuboids in wall coordinates, transformed by the wall pose.
#[derive(Default)]
pub(crate) struct MeshBuilder {
    vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
}

impl MeshBuilder {
    pub(crate) fn cuboid(&mut self, pose: &WallPose, x: (f64, f64), y: (f64, f64), z: (f64, f64)) {
        const QUADS: [[u32; 4]; 6] = [[0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [1, 2, 6, 5], [2, 3, 7, 6], [3, 0, 4, 7]];
        let base = self.vertices.len() as u32;
        for zi in [z.0, z.1] {
            for (xi, yi) in [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)] {
                self.vertices.push(pose.at(xi, yi, zi));
            }
        }
        for q in QUADS {
            self.triangles.push(Triangle { a: base + q[0], b: base + q[1], c: base + q[2] });
            self.triangles.push(Triangle { a: base + q[0], b: base + q[2], c: base + q[3] });
        }
    }

    pub(crate) fn prism(&mut self, pose: &WallPose, bottom: (f64, f64), top: (f64, f64), y: (f64, f64), z: (f64, f64)) {
        const QUADS: [[u32; 4]; 6] = [[0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [1, 2, 6, 5], [2, 3, 7, 6], [3, 0, 4, 7]];
        let base = self.vertices.len() as u32;
        for (zi, x) in [(z.0, bottom), (z.1, top)] {
            for (xi, yi) in [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)] {
                self.vertices.push(pose.at(xi, yi, zi));
            }
        }
        for q in QUADS {
            self.triangles.push(Triangle { a: base + q[0], b: base + q[1], c: base + q[2] });
            self.triangles.push(Triangle { a: base + q[0], b: base + q[2], c: base + q[3] });
        }
    }

    /// 🧱️ A hexahedron over the x range between two z levels whose y range changes from `bottom` to `top` (a trapezoid in the y–z plane extruded along x).
    pub(crate) fn frustum(&mut self, pose: &WallPose, x: (f64, f64), bottom: (f64, f64), top: (f64, f64), z: (f64, f64)) {
        const QUADS: [[u32; 4]; 6] = [[0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [1, 2, 6, 5], [2, 3, 7, 6], [3, 0, 4, 7]];
        let base = self.vertices.len() as u32;
        for (zi, y) in [(z.0, bottom), (z.1, top)] {
            for (xi, yi) in [(x.0, y.0.min(y.1)), (x.1, y.0.min(y.1)), (x.1, y.0.max(y.1)), (x.0, y.0.max(y.1))] {
                self.vertices.push(pose.at(xi, yi, zi));
            }
        }
        for q in QUADS {
            self.triangles.push(Triangle { a: base + q[0], b: base + q[1], c: base + q[2] });
            self.triangles.push(Triangle { a: base + q[0], b: base + q[2], c: base + q[3] });
        }
    }

    /// 📏️ A square-section rod of half width `half` between two points `(x, y, z)` in wall coordinates.
    pub(crate) fn rod(&mut self, pose: &WallPose, a: [f64; 3], b: [f64; 3], half: f64) {
        const QUADS: [[u32; 4]; 6] = [[0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [1, 2, 6, 5], [2, 3, 7, 6], [3, 0, 4, 7]];
        let cross = |p: [f64; 3], q: [f64; 3]| [p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]];
        let unit = |p: [f64; 3]| {
            let n = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            [p[0] / n, p[1] / n, p[2] / n]
        };
        let along = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        if along.iter().map(|c| c * c).sum::<f64>() < 1e-12 {
            return;
        }
        let d = unit(along);
        let seed = if d[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let u = unit(cross(d, seed));
        let v = cross(d, u);
        let base = self.vertices.len() as u32;
        for p in [a, b] {
            for (su, sv) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                let at = |k: usize| p[k] + half * (su * u[k] + sv * v[k]);
                self.vertices.push(pose.at(at(0), at(1), at(2)));
            }
        }
        for q in QUADS {
            self.triangles.push(Triangle { a: base + q[0], b: base + q[1], c: base + q[2] });
            self.triangles.push(Triangle { a: base + q[0], b: base + q[2], c: base + q[3] });
        }
    }

    /// 📏️ A thin sheet over a convex polygon of points `(x, y, z)` in wall coordinates, visible from both sides.
    pub(crate) fn sheet(&mut self, pose: &WallPose, points: &[[f64; 3]]) {
        let base = self.vertices.len() as u32;
        self.vertices.extend(points.iter().map(|p| pose.at(p[0], p[1], p[2])));
        for k in 1..points.len().saturating_sub(1) as u32 {
            self.triangles.push(Triangle { a: base, b: base + k, c: base + k + 1 });
            self.triangles.push(Triangle { a: base, b: base + k + 1, c: base + k });
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

/// 🧱️ Everything one scene entity needs before sealing; `nested` is the path below the wall row (`.openings[id=win-1]`), `owner_path` the path of an entity whose document row is not below a wall (`chases[id=c1]`).
pub(crate) struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub nested: String,
    pub owner_path: Option<String>,
    pub geometry: Geometry,
    pub role: MaterialRole,
    pub channel: Option<ColourChannel>,
    pub binding: Option<UtilisationBinding>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub handles: Vec<String>,
    pub opacity: f64,
    pub hatch: Option<Hatch>,
    pub error: bool,
    pub deformation: Option<DeformationOverlay>,
}

impl Spec {
    pub(crate) fn new(kind: EntityKind, id: impl Into<String>, geometry: Geometry, role: MaterialRole, layer: &'static str, name: LocalizedCopy) -> Self {
        Self { kind, id: id.into(), nested: String::new(), owner_path: None, geometry, role, channel: None, binding: None, detail: name.clone(), name, layer, handles: Vec::new(), opacity: 1.0, hatch: None, error: false, deformation: None }
    }

    pub(crate) fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.error || self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        let path = self.owner_path.clone().unwrap_or_else(|| format!("walls[id={}]{}", base_id(&self.id), self.nested));
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, &path),
            geometry: self.geometry,
            appearance: Appearance {
                role: self.role,
                channel: self.channel.unwrap_or(if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role }),
                style: if failing { StyleState::Error } else { StyleState::Normal },
                opacity: self.opacity,
                hatch: self.hatch.or(failing.then_some(Hatch::Diagonal)),
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
pub(crate) struct Out {
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
    /// 🎯️ An accumulator that keeps only what belongs to `part`: the live scene projects one wall, or the global rest, at a time.
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

    /// 🎯️ Whether the entity `id` is part of this projection; modules test it before building an entity's geometry.
    pub(crate) fn wants(&self, id: &str) -> bool {
        self.part().wants(id)
    }

    /// 🔗️ From now on annotations and tracks belong to no entity.
    pub(crate) fn global(&mut self) {
        self.owner = None;
    }

    /// 📥️ Pushes an entity built outside the spec builder (the case forms of the projection kit); its document row is the case of its base id.
    pub(crate) fn push_entity(&mut self, entity: SceneEntity, points: &[Vec3]) {
        self.owner = Some(case_ref(base_id(&entity.entity.id)));
        if !self.wants(&entity.entity.id) {
            return;
        }
        self.points.extend_from_slice(points);
        self.entities.push(entity);
    }

    pub(crate) fn push(&mut self, spec: Spec, points: &[Vec3]) {
        self.owner = Some(wall_ref(base_id(&spec.id)));
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

    pub(crate) fn track(&mut self, track: ParameterTrack) {
        if self.link(OwnedKind::Track, &track.id) {
            self.tracks.push(track);
        }
    }

    pub(crate) fn into_fragment(self) -> Fragment {
        let bounds = Aabb::of_points(&self.points);
        Fragment { entities: self.entities, annotations: self.annotations, legends: self.legends, layers: Vec::new(), tracks: self.tracks, owners: self.owners, bounds }
    }
}

/// 🏁️ The global parts of the merged scene: used layers, the utilisation and role legends in front of the field legends of the fragments, and the description.
pub(crate) fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    let used = projection_kit::legends::used_layers(&scene.entities, &scene.annotations, LAYER_ANNOTATIONS);
    let roles = role_legend(&scene.entities);
    let fields = std::mem::take(&mut scene.legends);
    scene.layers = projection_kit::legends::layers(&LAYERS, &used, &[LAYER_BOND]);
    scene.legends = vec![
        projection_kit::legends::utilisation_legend(loc("Utilisation", "Ausnutzung")),
        Legend { id: "roles".into(), title: loc("Materials", "Werkstoffe"), scale: LegendScale::Roles { entries: roles } },
    ];
    scene.legends.extend(fields);
    scene.description = description;
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
            MaterialRole::Masonry => ("Masonry", "Mauerwerk"),
            MaterialRole::Concrete => ("Concrete lintels and floors", "Beton: Stürze und Decken"),
            MaterialRole::Rebar => ("Reinforcement", "Bewehrung"),
            MaterialRole::Glass => ("Windows", "Fenster"),
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

const MEMBER_TAGS: [(&str, &str); 7] = [("piers", PIER_TAG), ("lintels", LINTEL_TAG), ("ties", TIE_TAG), ("prestress", TENDON_TAG), ("confined", COLUMN_TAG), ("flanges", super::lateral::FLANGE_TAG), ("bars", BAR_TAG)];

fn member_ids<'a>(wall: &'a model::MasonryWall, collection: &str) -> Vec<&'a str> {
    match collection {
        "piers" => wall.piers.iter().map(|x| x.id.as_str()).collect(),
        "lintels" => wall.lintels.iter().map(|x| x.id.as_str()).collect(),
        "ties" => wall.ties.iter().map(|x| x.id.as_str()).collect(),
        "prestress" => wall.prestress.iter().map(|x| x.id.as_str()).collect(),
        "confined" => wall.confined.iter().map(|x| x.id.as_str()).collect(),
        "flanges" => wall.flanges.iter().map(|x| x.id.as_str()).collect(),
        "bars" => wall.bars.iter().map(|x| x.id.as_str()).collect(),
        _ => Vec::new(),
    }
}

fn after_wall<'a>(check_id: &'a str, wall: &str) -> &'a str {
    let key = format!(".{wall}.");
    check_id.find(&key).map_or("", |at| &check_id[at + key.len()..])
}

fn embeds_wall(check_id: &str, wall: &str) -> bool {
    check_id.contains(&format!(".{wall}.")) || check_id.ends_with(&format!(".{wall}"))
}

/// 🧩️ The scene tag below its wall that draws the outcome of `check_id`: the concentrated load, movement joint, opening, bar or member the id names; `None` when the wall itself is the owner.
fn sub_tag(document: &model::En1996Snapshot, wall: &model::MasonryWall, check_id: &str) -> Option<String> {
    let rest = after_wall(check_id, &wall.id);
    if check_id.starts_with("en1996.6.1.3.") {
        return wall.load_cases.iter().flat_map(|lc| lc.concentrated.iter().map(move |c| (lc, c))).find(|(lc, c)| rest == format!("{}.{}", lc.id, c.id)).map(|(lc, c)| format!("{CONCENTRATED_TAG}{}-{}", lc.id, c.id));
    }
    if check_id.starts_with("en1996.8.5.joint-position.") {
        return document.movement_joints.iter().find(|j| j.wall_id == wall.id && check_id.ends_with(&format!(".{}", j.id))).map(|j| format!("{JOINT_TAG}{}", j.id));
    }
    if check_id.starts_with("en1996.8.5.joint-spacing.") {
        return document.movement_joints.iter().find(|j| j.wall_id == wall.id).map(|j| format!("{JOINT_TAG}{}", j.id));
    }
    if check_id.starts_with("en1996.5.4.opening.") || check_id.starts_with("en1996.5.5.lintel-cover.") {
        return wall.openings.iter().find(|o| rest == o.id).map(|o| format!("{OPENING_TAG}{}", o.id));
    }
    if check_id.starts_with("en1996.8.2.bar-layout.") {
        return wall.bars.iter().find(|b| rest == b.id).map(|b| format!("{BAR_TAG}{}", b.id));
    }
    let parts: Vec<&str> = rest.split('.').collect();
    MEMBER_TAGS.iter().find_map(|(collection, tag)| parts.windows(2).find(|pair| pair[0] == *collection && member_ids(wall, collection).contains(&pair[1])).map(|pair| format!("{tag}{}", pair[1])))
}

/// 🧭️ The scene entity that owns `check`: its declared design case, the chase it measures, else a sub-entity of the wall the id names (concentrated load, movement joint, opening, bar or member row), else the wall. The wall is the subject the shared ownership filing writes, or the wall whose id the check id embeds when the report is not filed yet. `None` for a check no row owns.
pub fn owner_entity(document: &model::En1996Snapshot, check: &norm::CheckResult) -> Option<String> {
    let id = check.id.as_str();
    let subject = base_id(&check.subject.entity_id);
    if document.cases.iter().any(|c| c.id == subject && super::cases::declared(c)) {
        return Some(subject.to_owned());
    }
    if let Some(chase) = id.starts_with("en1996.8.6.").then(|| document.chases.iter().find(|c| id.ends_with(&format!(".{}", c.id)) && document.walls.iter().any(|w| w.id == c.wall_id))).flatten() {
        return Some(format!("{}:{CHASE_TAG}{}", chase.wall_id, chase.id));
    }
    let wall = document.walls.iter().find(|w| w.id == subject).or_else(|| document.walls.iter().filter(|w| embeds_wall(id, &w.id)).max_by_key(|w| w.id.len()))?;
    Some(sub_tag(document, wall, id).map_or_else(|| wall.id.clone(), |tag| format!("{}:{tag}", wall.id)))
}

/// 📄️ Whether `check_id` belongs to the document as a whole (integrity, missing references, the bracing system); the list is the document-level set of the shared check ownership of the EN 1996 units.
pub fn is_document_check(check_id: &str) -> bool {
    <masonry::En1996Units as norm::incremental_evaluation::UnitSource>::OWNERS.is_document_level(check_id)
}

/// 🧭️ Where a check is drawn: on one scene entity, or on the global badge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckRoute {
    Entity(String),
    Document,
}

/// 🧭️ The route of `check`: the document for a document-level family, else its owner entity; `None` is a check no entity or badge draws, which the routing law forbids.
pub fn route_of(document: &model::En1996Snapshot, check: &norm::CheckResult) -> Option<CheckRoute> {
    if is_document_check(&check.id) {
        return Some(CheckRoute::Document);
    }
    owner_entity(document, check).map(CheckRoute::Entity)
}

/// 🎯️ The entity to select and frame for the check `check_id` of `report`, resolved through [`owner_entity`]; `None` for a document-level check.
pub fn focus_of_outcome(document: &model::En1996Snapshot, report: &norm::CheckReport, check_id: &str) -> Option<String> {
    report.checks.iter().find(|c| c.id == check_id).filter(|c| !is_document_check(&c.id)).and_then(|c| owner_entity(document, c))
}

/// 📶️ The governing binding among the checks owned by the scene entity `id` (every owned check when `prefixes` is empty).
pub fn binding_of_owner(document: &model::En1996Snapshot, report: &norm::CheckReport, id: &str, prefixes: &[&str]) -> Option<UtilisationBinding> {
    binding_by(report, |check| !is_document_check(&check.id) && owner_entity(document, check).as_deref() == Some(id) && (prefixes.is_empty() || prefixes.iter().any(|p| check.id.starts_with(p))))
}

/// 📊️ The check families a dedicated diagram draws with a marker bound to the check; every other check of a wall is a bar of its outcome diagram `check-outcomes:<wall>`.
pub const DRAWN_FAMILIES: &[&str] = &[
    "en1996.6.1.2.eccentricity.", "en1996.6.1.2.compression.", "en1996.5.5.1.slenderness.", "en1996.3.1.material.", "en1996.6.2.shear.", "en1996.m-31.", "en1996.m-21.", "en1996.m-44.", "en1996.1-2.fire.", "en1996.8.6.chase-", "en1996.8.5.joint-", "en1996.6.1.3.concentrated.", "en1996.6.1.3.area.",
];

/// 📊️ Whether a dedicated diagram draws the check `check_id`; the others are outcome bars.
pub fn is_drawn_family(check_id: &str) -> bool {
    DRAWN_FAMILIES.iter().any(|prefix| check_id.starts_with(prefix))
}
