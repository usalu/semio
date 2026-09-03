//! 💡️ Cad inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::cad::CadSnapshot;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use super::bounds::{object_count, scene_bounds, vertex_count, CadBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a cad snapshot. Today: object/brep-vertex counts and the 3d
/// bounding box across every pane's object origins and vertex positions (see
/// `📦bounds/🦀️.rs`). A simple whole-snapshot scalar — no `InferredField` caching, a full
/// scan over the document is cheap at cad scale.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad.inference")]
pub struct CadInference {
    #[derived]
    pub object_count: usize,
    #[derived]
    pub vertex_count: usize,
    #[derived]
    pub bounds: Option<CadBounds>,
}

impl protocol::Inference<CadSnapshot> for CadInference {
    fn infer(snapshot: &CadSnapshot) -> Self {
        Self { object_count: object_count(snapshot), vertex_count: vertex_count(snapshot), bounds: scene_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<CadSnapshot> for CadInference {
    fn inference_schema_id() -> &'static str {
        "s.cad.cad.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.objectCount", reads: &["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] },
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.vertexCount", reads: &["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] },
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.bounds", reads: &["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::cad::standards::v1::subsets::any::schema::CadBuilder {
    type Snapshot = CadSnapshot;
    type Inference = CadInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.cad.cad.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `cad_artifact_schema_descriptor`'s registration.
pub fn cad_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.cad.cad.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::cad::{empty_cad_snapshot, testkit::sample_model_child};
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let mut snapshot = empty_cad_snapshot();
        snapshot.shape_model = Some(sample_model_child("inference-law-1"));
        assert_eq!(CadInference::infer(&snapshot), CadInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(CadInference::infer(&empty_cad_snapshot()), CadInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests

//#region 🔄️DeriveTransformation
// 🐛️ Relocated verbatim from the deleted `⚙️engine/🔄️transformation/🦀️.rs` (ticket
// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) -- pure `kernel + [CadObject]` derived
// classification/reclassification (rule 2: pure fn snapshot/objects -> derived objects), not
// stateful app behaviour.
mod derive_transformation {
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadObject;
    #[cfg(test)]
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadPrimitiveSlot;

    use semio_framework_3d::engine::Vec3;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel, GeometryHandle};
    #[cfg(test)]
    use std::collections::HashMap;

    //#region 🔖️ClassifyRules
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DominantAxis {
        X,
        Y,
        Z,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum ZBand {
        Min,
        Max,
        Mid,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ClassifyRule {
        pub role: &'static str,
        pub typology: &'static str,
        pub dominant_axis: Option<DominantAxis>,
        pub min_dominant_normal: Option<f64>,
        pub min_axis_normal: Option<f64>,
        pub z_band: Option<ZBand>,
        pub fallback: bool,
    }

    #[cfg(test)]
    const FROM_GEOMETRY_CLASSIFY_RULES: &[ClassifyRule] = &[
        ClassifyRule { role: "roof", typology: "energy.energy.roof", dominant_axis: Some(DominantAxis::Z), min_dominant_normal: Some(0.75), min_axis_normal: None, z_band: Some(ZBand::Max), fallback: false },
        ClassifyRule { role: "baseplate", typology: "energy.energy.baseplate", dominant_axis: Some(DominantAxis::Z), min_dominant_normal: Some(0.75), min_axis_normal: None, z_band: Some(ZBand::Min), fallback: false },
        ClassifyRule { role: "slab", typology: "energy.energy.hull", dominant_axis: Some(DominantAxis::Z), min_dominant_normal: Some(0.75), min_axis_normal: None, z_band: None, fallback: false },
        ClassifyRule { role: "externalwall", typology: "energy.energy.externalwall", dominant_axis: None, min_dominant_normal: None, min_axis_normal: Some(0.5), z_band: None, fallback: false },
        ClassifyRule { role: "slab", typology: "energy.energy.hull", dominant_axis: None, min_dominant_normal: None, min_axis_normal: None, z_band: None, fallback: true },
    ];

    const ENERGY_TYPOLOGIES: &[&str] = &["energy.energy.hull", "energy.energy.baseplate", "energy.energy.roof", "energy.energy.externalwall", "energy.energy.windows"];
    //#endregion 🔖️ClassifyRules

    //#region 🔖️FaceAnalytics
    fn face_mesh_analytics(kernel: &Brep, face: &GeometryHandle) -> Option<(Vec3, Vec3)> {
        let mesh = kernel.tessellate(face, 0.1).ok()?;
        let mut area_sum = 0.0;
        let mut centroid = [0.0, 0.0, 0.0];
        let mut normal = [0.0, 0.0, 0.0];
        for triangle in mesh.index.chunks_exact(3) {
            let i0 = triangle[0] as usize;
            let i1 = triangle[1] as usize;
            let i2 = triangle[2] as usize;
            let p0 = [mesh.position[i0 * 3] as f64, mesh.position[i0 * 3 + 1] as f64, mesh.position[i0 * 3 + 2] as f64];
            let p1 = [mesh.position[i1 * 3] as f64, mesh.position[i1 * 3 + 1] as f64, mesh.position[i1 * 3 + 2] as f64];
            let p2 = [mesh.position[i2 * 3] as f64, mesh.position[i2 * 3 + 1] as f64, mesh.position[i2 * 3 + 2] as f64];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let cross = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            let area = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() * 0.5;
            if area <= 1e-12 {
                continue;
            }
            area_sum += area;
            for axis in 0..3 {
                centroid[axis] += area * (p0[axis] + p1[axis] + p2[axis]) / 3.0;
                normal[axis] += cross[axis];
            }
        }
        if area_sum <= 1e-12 {
            return None;
        }
        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if len <= 1e-12 {
            return None;
        }
        Some(([centroid[0] / area_sum, centroid[1] / area_sum, centroid[2] / area_sum], [normal[0] / len, normal[1] / len, normal[2] / len]))
    }

    /// @emoji 📍️ Face centroid via tessellated triangle area weighting (premigration `faceCentroid` equivalent).
    pub fn face_centroid_sync(kernel: &Brep, face: &GeometryHandle) -> Option<Vec3> {
        face_mesh_analytics(kernel, face).map(|(centroid, _)| centroid)
    }

    /// @emoji 🧭️ Face outward normal from tessellated triangle winding.
    pub fn face_normal_sync(kernel: &Brep, face: &GeometryHandle) -> Option<Vec3> {
        face_mesh_analytics(kernel, face).map(|(_, normal)| normal)
    }

    /// @emoji 🗂️ Groups coplanar faces by dominant axis, sign, and quantized centroid (premigration `facePlaneGroupKey`).
    pub fn face_plane_group_key(normal: Vec3, centroid: Vec3) -> String {
        let [nx, ny, nz] = normal;
        let abs = [nx.abs(), ny.abs(), nz.abs()];
        let (dominant, sign) = if abs[0] >= abs[1] && abs[0] >= abs[2] {
            ("x", nx.signum())
        } else if abs[1] >= abs[2] {
            ("y", ny.signum())
        } else {
            ("z", nz.signum())
        };
        let q = |v: f64| (v * 1000.0).round() / 1000.0;
        format!("{dominant}:{sign}:{}:{}:{}", q(centroid[0]), q(centroid[1]), q(centroid[2]))
    }

    #[cfg(test)]
    fn dominant_axis_of(normal: Vec3) -> DominantAxis {
        let [nx, ny, nz] = normal;
        let abs = [nx.abs(), ny.abs(), nz.abs()];
        if abs[0] >= abs[1] && abs[0] >= abs[2] {
            DominantAxis::X
        } else if abs[1] >= abs[2] {
            DominantAxis::Y
        } else {
            DominantAxis::Z
        }
    }

    #[cfg(test)]
    fn axis_normal_component(normal: Vec3, axis: DominantAxis) -> f64 {
        match axis {
            DominantAxis::X => normal[0].abs(),
            DominantAxis::Y => normal[1].abs(),
            DominantAxis::Z => normal[2].abs(),
        }
    }

    #[cfg(test)]
    fn classify_rule_matches(rule: &ClassifyRule, normal: Vec3, centroid_z: f64, z_min: f64, z_max: f64, z_tol: f64) -> bool {
        if rule.fallback {
            return true;
        }
        if let Some(min_axis) = rule.min_axis_normal {
            let dominant = dominant_axis_of(normal);
            if axis_normal_component(normal, dominant) < min_axis {
                return false;
            }
            if rule.dominant_axis.is_some() {
                return false;
            }
            return true;
        }
        if let Some(axis) = rule.dominant_axis {
            if dominant_axis_of(normal) != axis {
                return false;
            }
            if let Some(min_dom) = rule.min_dominant_normal {
                if axis_normal_component(normal, axis) < min_dom {
                    return false;
                }
            }
            if let Some(band) = rule.z_band {
                return match band {
                    ZBand::Min => (centroid_z - z_min).abs() <= z_tol,
                    ZBand::Max => (centroid_z - z_max).abs() <= z_tol,
                    ZBand::Mid => {
                        let mid = (z_min + z_max) * 0.5;
                        (centroid_z - mid).abs() <= z_tol
                    }
                };
            }
            return true;
        }
        false
    }
    //#endregion 🔖️FaceAnalytics

    //#region 🔖️SolidConstruction
    /// @emoji 📦️ Builds or reuses a kernel solid for a CAD object.
    pub(crate) fn solid_for_object(kernel: &mut Brep, object: &CadObject) -> Option<GeometryHandle> {
        if let Some(handle) = object.solid_handle.as_ref() {
            if kernel.kind(&GeometryHandle(handle.clone())).is_ok() {
                return Some(GeometryHandle(handle.clone()));
            }
        }
        let [ex, ey, ez] = object.extent.unwrap_or([1.0, 1.0, 1.0]);
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        let is_cylindrical = object.typology.contains("column");
        let handle = if is_cylindrical { kernel.cylinder_prim(width.max(depth) * 0.5, height).ok() } else { kernel.box_prim(width, depth, height).ok() }?;
        Some(handle)
    }

    /// @emoji 📦️ Builds a kernel solid sized from extent without mutating the object.
    pub fn build_solid_for_typology(kernel: &mut Brep, typology: &str, extent: [f64; 3]) -> Option<GeometryHandle> {
        let [ex, ey, ez] = extent;
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        if typology.contains("column") {
            kernel.cylinder_prim(width.max(depth) * 0.5, height).ok()
        } else {
            kernel.box_prim(width, depth, height).ok()
        }
    }

    #[cfg(test)]
    fn fuse_solids(kernel: &mut Brep, solids: &[GeometryHandle]) -> Option<GeometryHandle> {
        if solids.is_empty() {
            return None;
        }
        let mut current = solids[0].clone();
        for solid in solids.iter().skip(1) {
            current = kernel.fuse(&current, solid).ok()?;
        }
        Some(current)
    }
    //#endregion 🔖️SolidConstruction

    //#region 🔖️DeriveEngine
    #[cfg(test)]
    struct FaceMeta {
        handle: GeometryHandle,
        normal: Vec3,
        centroid: Vec3,
    }

    #[cfg(test)]
    fn next_object_id(prefix: &str, index: usize) -> String {
        format!("{prefix}-{index}")
    }

    /// @emoji 🔄️ Derives energy objects from shape-pane solids via fuse + face classification.
    #[cfg(test)]
    pub(crate) fn run_derive_from_geometry(kernel: &mut Brep, source_objects: &[CadObject], id_seed: &str) -> Vec<CadObject> {
        let solids: Vec<GeometryHandle> = source_objects.iter().filter_map(|object| solid_for_object(kernel, object)).collect();
        if solids.is_empty() {
            return Vec::new();
        }
        let hull = match fuse_solids(kernel, &solids) {
            Some(hull) => hull,
            None => return Vec::new(),
        };
        let topology = match kernel.deconstruct(&hull) {
            Ok(topology) => topology,
            Err(_) => return Vec::new(),
        };
        let face_meta: Vec<FaceMeta> = topology
            .faces
            .iter()
            .filter_map(|face| {
                let normal = face_normal_sync(kernel, face)?;
                let centroid = face_centroid_sync(kernel, face)?;
                Some(FaceMeta { handle: face.clone(), normal, centroid })
            })
            .collect();
        if face_meta.is_empty() {
            return vec![CadObject {
                id: next_object_id(id_seed, 0),
                label: "Hull".into(),
                typology: "energy.energy.hull".into(),
                visible: true,
                locked: false,
                origin: [0.0, 0.0, 0.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: None,
                solid_handle: Some(hull.0.clone()),
                primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: hull.0.clone(), kind: "solid".into() }],
            }];
        }
        let z_min = face_meta.iter().map(|face| face.centroid[2]).fold(f64::INFINITY, f64::min);
        let z_max = face_meta.iter().map(|face| face.centroid[2]).fold(f64::NEG_INFINITY, f64::max);
        let z_span = (z_max - z_min).max(0.001);
        let z_tol = (z_span * 0.02).max(0.001);
        let mut objects = Vec::new();
        let hull_id = next_object_id(id_seed, 0);
        objects.push(CadObject {
            id: hull_id,
            label: "Hull".into(),
            typology: "energy.energy.hull".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: Some(hull.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: hull.0.clone(), kind: "solid".into() }],
        });
        let mut grouped: HashMap<String, Vec<&FaceMeta>> = HashMap::new();
        for face in &face_meta {
            let rule = FROM_GEOMETRY_CLASSIFY_RULES.iter().find(|rule| classify_rule_matches(rule, face.normal, face.centroid[2], z_min, z_max, z_tol)).unwrap_or(&FROM_GEOMETRY_CLASSIFY_RULES[FROM_GEOMETRY_CLASSIFY_RULES.len() - 1]);
            if rule.role == "slab" && rule.fallback {
                continue;
            }
            let key = format!("{}:{}", rule.typology, face_plane_group_key(face.normal, face.centroid));
            grouped.entry(key).or_default().push(face);
        }
        let mut index = 1usize;
        for (_key, faces) in grouped {
            let face = faces[0];
            let rule = FROM_GEOMETRY_CLASSIFY_RULES.iter().find(|rule| classify_rule_matches(rule, face.normal, face.centroid[2], z_min, z_max, z_tol)).unwrap_or(&FROM_GEOMETRY_CLASSIFY_RULES[FROM_GEOMETRY_CLASSIFY_RULES.len() - 1]);
            let label = rule.role.replace("externalwall", "External Wall").replace("slab", "Slab");
            objects.push(CadObject {
                id: next_object_id(id_seed, index),
                label,
                typology: rule.typology.into(),
                visible: true,
                locked: false,
                origin: face.centroid,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: None,
                solid_handle: Some(face.handle.0.clone()),
                primitives: vec![CadPrimitiveSlot { slot: "surface".into(), primitive_id: face.handle.0.clone(), kind: "surface".into() }],
            });
            index += 1;
        }
        if !objects.iter().any(|object| object.typology == "energy.energy.windows") {
            objects.push(CadObject {
                id: next_object_id(id_seed, index),
                label: "Windows".into(),
                typology: "energy.energy.windows".into(),
                visible: true,
                locked: false,
                origin: [0.0, 0.0, 0.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: None,
                solid_handle: None,
                primitives: Vec::new(),
            });
        }
        objects
    }

    #[cfg(test)]
    const BUILDING_TO_STRUCTURE: &[(&str, &str)] = &[
        ("building.building.slab", "structure.structure.onewayreinforcedconcreteslab"),
        ("building.building.column", "structure.structure.reinforcedconcretecolumn"),
        ("building.building.beam", "structure.structure.reinforcedconcretebeam"),
        ("building.building.wall", "structure.structure.reinforcedconcreteinternalwall"),
        ("aec.building.slab", "structure.structure.onewayreinforcedconcreteslab"),
        ("aec.building.column", "structure.structure.reinforcedconcretecolumn"),
    ];

    /// @emoji 🔄️ Maps building typologies to structure-classic equivalents (premigration `from_building` applier).
    #[cfg(test)]
    pub(crate) fn apply_from_building(source_objects: &[CadObject], id_seed: &str) -> Vec<CadObject> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        source_objects
            .iter()
            .filter_map(|object| BUILDING_TO_STRUCTURE.iter().find(|(from, _)| *from == object.typology.as_str()).map(|(_, to)| (*to, object)))
            .map(|(mapped, object)| {
                let index = counts.entry(mapped).or_insert(0);
                let object_id = format!("{id_seed}-{mapped}-{index}");
                *index += 1;
                CadObject {
                    id: object_id,
                    label: object.label.clone(),
                    typology: mapped.into(),
                    visible: object.visible,
                    locked: object.locked,
                    origin: object.origin,
                    orientation: object.orientation,
                    scale: object.scale,
                    mesh_url: object.mesh_url.clone(),
                    extent: object.extent,
                    solid_handle: object.solid_handle.clone(),
                    primitives: object.primitives.clone(),
                }
            })
            .collect()
    }

    /// @emoji 🔄️ Filters source objects to whitelisted typologies (premigration `applyTransformationFallback`).
    #[cfg(test)]
    pub(crate) fn apply_typology_fallback(source_objects: &[CadObject], typologies: &[&str], id_seed: &str) -> Vec<CadObject> {
        source_objects
            .iter()
            .enumerate()
            .filter(|(_, object)| typologies.contains(&object.typology.as_str()))
            .map(|(index, object)| CadObject {
                id: format!("{id_seed}-{index}"),
                label: object.label.clone(),
                typology: object.typology.clone(),
                visible: object.visible,
                locked: object.locked,
                origin: object.origin,
                orientation: object.orientation,
                scale: object.scale,
                mesh_url: object.mesh_url.clone(),
                extent: object.extent,
                solid_handle: object.solid_handle.clone(),
                primitives: object.primitives.clone(),
            })
            .collect()
    }

    pub fn energy_typologies() -> &'static [&'static str] {
        ENERGY_TYPOLOGIES
    }
    //#endregion 🔖️DeriveEngine

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn derive_from_geometry_classifies_box() {
            let mut kernel = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::Brep::new();
            let solid = kernel.box_prim(2.0, 2.0, 3.0).expect("box");
            let source = vec![CadObject {
                id: "object-box".into(),
                label: "Box".into(),
                typology: "spatial.shape.primitive.box".into(),
                visible: true,
                locked: false,
                origin: [0.0, 0.0, 0.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([2.0, 2.0, 3.0]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
            }];
            let derived = run_derive_from_geometry(&mut kernel, &source, "energy");
            let typos: Vec<_> = derived.iter().map(|o| o.typology.as_str()).collect();
            assert!(derived.iter().any(|object| object.typology == "energy.energy.hull"), "missing hull in {typos:?}");
            assert!(derived.iter().any(|object| object.typology == "energy.energy.roof" || object.typology == "energy.energy.baseplate"), "missing roof/baseplate in {typos:?}");
            assert!(derived.iter().any(|object| object.typology == "energy.energy.externalwall"), "missing wall in {typos:?}");
            assert!(derived.iter().any(|object| object.typology == "energy.energy.windows"), "missing windows in {typos:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn face_plane_group_key_is_stable() {
            let key = face_plane_group_key([0.0, 0.0, 1.0], [1.0, 2.0, 3.0]);
            assert!(key.starts_with("z:1:"));
        }
    }
}
pub use derive_transformation::*;
//#endregion 🔄️DeriveTransformation

//#region 🔍️ConstructQuery
// 🐛️ Relocated verbatim from the deleted `⚙️engine/🔍️construct/🦀️.rs` -- a read-only
// Jack `QueryableGraph` adapter over one `CadGeometry` pane (rule 2/CAD-map: query -> D4
// inference-shaped derived compute).
mod construct_query {
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadGeometry;
    use graph::dsl::{QueryableEdge, QueryableGraph};
    use graph::manifest::PropertyValue;
    use std::collections::BTreeSet;

    const KIND_VERTEX: &str = "Vertex";
    const KIND_EDGE: &str = "Edge";
    const KIND_WIRE: &str = "Wire";
    const KIND_FACE: &str = "Face";
    const KIND_SHELL: &str = "Shell";
    const KIND_SOLID: &str = "Solid";

    const REL_BOUNDED_BY: &str = "BOUNDED_BY";
    const REL_CONTAINS: &str = "CONTAINS";

    /// @emoji 🕸️ One `CadGeometry` pane (e.g. `scene.shape_geometry`), exposed as a Jack
    /// `QueryableGraph` — read-only, matching `construct.md`'s explicit constraint that direct
    /// graph mutation is unsafe for a B-rep and must go through a validated command layer instead.
    pub struct CadTopologyGraph<'a> {
        geometry: &'a CadGeometry,
    }

    impl<'a> CadTopologyGraph<'a> {
        #[cfg(test)]
        pub(crate) fn new(geometry: &'a CadGeometry) -> Self {
            Self { geometry }
        }
    }

    impl QueryableGraph for CadTopologyGraph<'_> {
        fn manifest(&self) -> Option<&graph::manifest::GraphManifest> {
            None
        }

        fn node_ids(&self) -> Vec<String> {
            let g = self.geometry;
            g.vertices
                .iter()
                .map(|v| v.id.clone())
                .chain(g.edges.iter().map(|e| e.id.clone()))
                .chain(g.wires.iter().map(|w| w.id.clone()))
                .chain(g.faces.iter().map(|f| f.id.clone()))
                .chain(g.shells.iter().map(|s| s.id.clone()))
                .chain(g.solids.iter().map(|s| s.id.clone()))
                .collect()
        }

        fn node_kind(&self, id: &str) -> Option<String> {
            let g = self.geometry;
            if g.vertices.iter().any(|v| v.id == id) {
                return Some(KIND_VERTEX.to_string());
            }
            if g.edges.iter().any(|e| e.id == id) {
                return Some(KIND_EDGE.to_string());
            }
            if g.wires.iter().any(|w| w.id == id) {
                return Some(KIND_WIRE.to_string());
            }
            if g.faces.iter().any(|f| f.id == id) {
                return Some(KIND_FACE.to_string());
            }
            if g.shells.iter().any(|s| s.id == id) {
                return Some(KIND_SHELL.to_string());
            }
            if g.solids.iter().any(|s| s.id == id) {
                return Some(KIND_SOLID.to_string());
            }
            None
        }

        fn node_name(&self, id: &str) -> Option<String> {
            self.node_kind(id).map(|_| id.to_string())
        }

        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let g = self.geometry;
            match key {
                "position" => g.vertices.iter().find(|v| v.id == id).map(|v| PropertyValue::Array(v.position.iter().map(|c| PropertyValue::Number(*c)).collect())),
                "curveKind" => g.edges.iter().find(|e| e.id == id).map(|e| PropertyValue::String(e.curve.kind.clone())),
                "surfaceKind" => g.faces.iter().find(|f| f.id == id).map(|f| PropertyValue::String(f.surface.kind.clone())),
                "normal" => g.faces.iter().find(|f| f.id == id).map(|f| PropertyValue::Array(f.surface.normal.iter().map(|c| PropertyValue::Number(*c)).collect())),
                _ => None,
            }
        }

        fn edges(&self) -> Vec<QueryableEdge> {
            let g = self.geometry;
            let mut out = Vec::new();
            let mut next_id = 0usize;
            let mut push = |kind: &str, source_node_id: String, target_node_id: String| {
                next_id += 1;
                out.push(QueryableEdge { id: format!("{kind}-{next_id}"), kind: kind.to_string(), source_node_id, target_node_id, source_port: None, target_port: None, properties: graph::manifest::PropertyBag::default() });
            };
            for solid in &g.solids {
                for shell_id in &solid.shell_ids {
                    push(REL_BOUNDED_BY, solid.id.clone(), shell_id.clone());
                }
            }
            for shell in &g.shells {
                for face_id in &shell.face_ids {
                    push(REL_BOUNDED_BY, shell.id.clone(), face_id.clone());
                }
            }
            for face in &g.faces {
                for wire_id in &face.wire_ids {
                    push(REL_BOUNDED_BY, face.id.clone(), wire_id.clone());
                }
            }
            for wire in &g.wires {
                for edge_id in &wire.edge_ids {
                    push(REL_CONTAINS, wire.id.clone(), edge_id.clone());
                }
            }
            for edge in &g.edges {
                for vertex_id in &edge.vertex_ids {
                    push(REL_CONTAINS, edge.id.clone(), vertex_id.clone());
                }
            }
            out
        }

        fn subgraph_fixture_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
            None
        }
    }

    /// @emoji 🔍️ Runs a Jack query against one `CadGeometry` pane and returns its JSON result —
    /// the single entry point `cad-ui`/an MCP tool calls for topology queries (`saved selections`,
    /// non-manifold-edge checks, adjacency lookups), reusing `graph::dsl::run_query_json`
    /// unchanged.
    #[cfg(test)]
    pub(crate) fn run_construct_query(geometry: &CadGeometry, source: &str) -> Result<String, graph::dsl::GraphDslError> {
        let graph = CadTopologyGraph::new(geometry);
        graph::dsl::run_query_json(&graph, source)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::{CadEdge, CadEdgeCurve, CadFace, CadPlaneSurface, CadShell, CadSolid, CadVertex, CadWire};

        fn box_geometry() -> CadGeometry {
            let corners: [[f64; 3]; 8] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
            let vertices: Vec<CadVertex> = corners.iter().enumerate().map(|(i, p)| CadVertex { id: format!("v{i}"), position: *p }).collect();
            let edge_pairs: [(usize, usize); 12] = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
            let edges: Vec<CadEdge> = edge_pairs.iter().enumerate().map(|(i, (a, b))| CadEdge { id: format!("e{i}"), vertex_ids: vec![format!("v{a}"), format!("v{b}")], curve: CadEdgeCurve { kind: "line".into() } }).collect();
            let face_wire_edges: [[usize; 4]; 6] = [[0, 1, 2, 3], [4, 5, 6, 7], [0, 9, 4, 8], [2, 11, 6, 10], [3, 8, 7, 11], [1, 10, 5, 9]];
            let wires: Vec<CadWire> = face_wire_edges.iter().enumerate().map(|(i, es)| CadWire { id: format!("w{i}"), edge_ids: es.iter().map(|e| format!("e{e}")).collect() }).collect();
            let faces: Vec<CadFace> = (0..6).map(|i| CadFace { id: format!("f{i}"), wire_ids: vec![format!("w{i}")], surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] } }).collect();
            let shell = CadShell { id: "s0".into(), face_ids: (0..6).map(|i| format!("f{i}")).collect() };
            let solid = CadSolid { id: "sol0".into(), shell_ids: vec!["s0".into()] };
            CadGeometry { anchors: Vec::new(), vertices, edges, wires, faces, shells: vec![shell], solids: vec![solid] }
        }

        #[semio_framework_async_macros::async_test]
        async fn topology_graph_exposes_every_entity_as_a_labeled_node() {
            let geometry = box_geometry();
            let graph = CadTopologyGraph::new(&geometry);
            assert_eq!(graph.node_kind("v0").as_deref(), Some(KIND_VERTEX));
            assert_eq!(graph.node_kind("e0").as_deref(), Some(KIND_EDGE));
            assert_eq!(graph.node_kind("w0").as_deref(), Some(KIND_WIRE));
            assert_eq!(graph.node_kind("f0").as_deref(), Some(KIND_FACE));
            assert_eq!(graph.node_kind("s0").as_deref(), Some(KIND_SHELL));
            assert_eq!(graph.node_kind("sol0").as_deref(), Some(KIND_SOLID));
            assert_eq!(graph.node_kind("nonexistent"), None);
            assert_eq!(graph.node_ids().len(), 8 + 12 + 6 + 6 + 1 + 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn topology_graph_bounded_by_and_contains_edges_traverse_every_dimension() {
            let geometry = box_geometry();
            let graph = CadTopologyGraph::new(&geometry);
            let rel_edges = graph.edges();
            assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "sol0" && e.target_node_id == "s0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "s0" && e.target_node_id == "f0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "f0" && e.target_node_id == "w0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "w0" && e.target_node_id == "e0"));
            assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "e0" && e.target_node_id == "v0"));
        }

        #[semio_framework_async_macros::async_test]
        async fn construct_query_finds_every_face_bounded_by_its_wire() {
            let geometry = box_geometry();
            let json = run_construct_query(&geometry, "MATCH (f:Face)--[:BOUNDED_BY]->(w:Wire) RETURN f.name, w.name").expect("construct query must run");
            let value = protocol::json::parse(&json).expect("valid JSON result");
            let rows = value.get("rows").and_then(protocol::os_pack::json::Value::as_array).expect("rows array");
            assert_eq!(rows.len(), 6, "every one of the 6 faces must match exactly its own wire: {json}");
        }

        #[semio_framework_async_macros::async_test]
        async fn construct_query_filters_edges_by_curve_kind_property() {
            let geometry = box_geometry();
            let json = run_construct_query(&geometry, "MATCH (e:Edge) WHERE e.curveKind = 'line' RETURN e.name").expect("construct query must run");
            let value = protocol::json::parse(&json).expect("valid JSON result");
            let rows = value.get("rows").and_then(protocol::os_pack::json::Value::as_array).expect("rows array");
            assert_eq!(rows.len(), 12, "all 12 box edges are line curves: {json}");
        }

        #[semio_framework_async_macros::async_test]
        async fn construct_query_rejects_malformed_syntax_with_a_real_parse_error() {
            let geometry = box_geometry();
            let error = run_construct_query(&geometry, "NOT A QUERY (((").unwrap_err();
            let _ = error;
        }
    }
}
pub use construct_query::*;
//#endregion 🔍️ConstructQuery

//#region 🖥️SceneCompute
// 🐛️ Relocated from the deleted `⚙️engine/🦀️.rs`'s `🔖️Compute`/`🔖️Register` regions
// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) -- pure derived compute over
// `CadSnapshot`/`CadObject`/`CadCamera` (rule 2/3), never io bytes, never an app type. The
// io-bound half of that file (native solid export, file-payload import, foreign-format bridges)
// moved to `🚪️io/🦀️.rs` instead; the interaction statechart moved to the app's own
// `⚙️engine` (D5 behavioural).
mod scene_compute {
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::{
        centroid_from_fixture_primitives, objects_from_fixture_model, parse_geometry, semio_model_snapshot_from_objects, tessellate_object_mesh, tessellate_object_mesh_from_fixture, CadGeometry, CadObject, CadPrimitiveSlot,
    };
    use crate::artifacts::cad::{cad_model_child_handle, CadCamera, CadModelChild, CadNode, CadPaneId, CadProjectionDsl, CadReference, CadSnapshot, CadWorkingScene, CAD_PLAY_DOCUMENT_SCHEMA};
    use semio_framework::parse_contributions;
    use semio_framework_3d::engine::MeshTransfer;
    use semio_framework_plugin::{mesh_from_kind, MeshData, WorldProjectionConfig};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::mesh_data_from_mesh_transfer;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel, GeometryHandle};
    use std::collections::HashSet;
    use std::sync::{Arc, OnceLock};

    pub const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";

    pub const CAD_DEFAULT_TYPOLOGY_EXTENT: [f64; 3] = [1.0, 1.0, 1.0];

    /// @emoji 🗂️ Indices into the quad play fixture's `models[]` array — one model definition per pane.
    const CAD_MODEL_INDEX_SHAPE: usize = 0;

    const CAD_MODEL_INDEX_BUILDING: usize = 1;

    const CAD_MODEL_INDEX_ENERGY: usize = 2;

    const CAD_MODEL_INDEX_STRUCTURE_CLASSIC: usize = 3;

    const FOREST_LEFT_MODEL_JSON: &str = include_str!("../../📚️examples/🖼️assets/🎮️play/🔣️.json");

    pub const CAD_MODEL_DEFINITION_SHAPE: &str = "spatial.shape";

    pub const CAD_MODEL_DEFINITION_BUILDING: &str = "aec.building";

    pub const CAD_MODEL_DEFINITION_ENERGY: &str = "aec.building.energy";

    pub const CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC: &str = "aec.building.structure.classic";

    const CAD_CONCRETE_FOREST_REFERENCE_URL: &str = "/cad-fixture/🖼️concrete-forest-reference.png";

    pub const CAD_FOREST_REFERENCE_WIDTH_WORLD: f64 = 28.6;

    pub const CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX: f64 = 1430.0;

    pub const CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX: f64 = 692.0;

    const CAD_FOREST_REFERENCE_BASE_ORIGIN_XY: [f64; 2] = [-24.0, -18.0];

    pub const CAD_FOREST_REFERENCE_PLANE_Z: f64 = 0.01;

    pub const CAD_FOREST_REFERENCE_Y_OFFSET_RATIO: f64 = 0.2;

    /// 🌱 Fresh, doctrine-tier-(d) brep kernel: a `Brep::new()` local to the caller, never a
    /// process-global session (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    /// wave G4 — this replaces the deleted `static HOST: OnceLock<BrepEngineHost>`, which was the
    /// exact ambient-reach anti-pattern the ticket exists to remove even though it was write-once).
    /// Every call site already builds, uses and drops its handles within the one call that owns
    /// this kernel, so no cross-call registry was ever load-bearing.
    pub fn cad_brep_kernel() -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::Brep {
        semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::Brep::new()
    }

    /// @emoji 📐️ Tessellates a typology's primitive sized from authored geometry (or a universal
    /// fallback extent when no geometry was captured), instead of hardcoded per-typology constants.
    fn typology_brep_mesh(typology: &str, extent: Option<[f64; 3]>, solid_handle: Option<&str>, centroid: Option<[f64; 3]>) -> MeshData {
        let mut kernel = cad_brep_kernel();
        if let Some(handle_id) = solid_handle {
            let handle = GeometryHandle(handle_id.into());
            if let Ok(mesh) = kernel.tessellate(&handle, 0.1) {
                return mesh_data_from_mesh_transfer(&mesh);
            }
        }
        let [ex, ey, ez] = extent.unwrap_or(CAD_DEFAULT_TYPOLOGY_EXTENT);
        let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
        let is_cylindrical = typology_mesh_kind(typology) == "cylinder";
        let handle = if is_cylindrical { kernel.cylinder_prim(width.max(depth) * 0.5, height) } else { kernel.box_prim(width, depth, height) };
        let Ok(handle) = handle else {
            return mesh_from_kind(typology_mesh_kind(typology));
        };
        let mesh: MeshTransfer = match kernel.tessellate(&handle, 0.1) {
            Ok(mesh) => mesh,
            Err(_) => {
                let _ = kernel.dispose(&handle);
                return mesh_from_kind(typology_mesh_kind(typology));
            }
        };
        let _ = kernel.dispose(&handle);
        let mut mesh_data = mesh_data_from_mesh_transfer(&mesh);
        if let Some(center) = centroid {
            translate_mesh_positions(&mut mesh_data, [center[0] as f32, center[1] as f32, center[2] as f32]);
        }
        mesh_data
    }

    fn mesh_centroid(mesh: &MeshData) -> Option<[f32; 3]> {
        if mesh.positions.is_empty() {
            return None;
        }
        let count = mesh.positions.len() / 3;
        let mut sum = [0.0f32; 3];
        for vertex in mesh.positions.as_chunks::<3>().0 {
            sum[0] += vertex[0];
            sum[1] += vertex[1];
            sum[2] += vertex[2];
        }
        let n = count as f32;
        Some([sum[0] / n, sum[1] / n, sum[2] / n])
    }

    /// @emoji 📐️ Shifts a tessellated mesh onto the authored fixture primitive centroid when kernel output drifts.
    pub(crate) fn align_mesh_to_fixture_centroid(mesh: &mut MeshData, geometry: &CadGeometry, primitives: &[CadPrimitiveSlot]) {
        let Some(target) = centroid_from_fixture_primitives(geometry, primitives) else {
            return;
        };
        let Some(current) = mesh_centroid(mesh) else {
            return;
        };
        let delta = [(target[0] as f32) - current[0], (target[1] as f32) - current[1], (target[2] as f32) - current[2]];
        if delta[0].abs() + delta[1].abs() + delta[2].abs() > 0.05 {
            translate_mesh_positions(mesh, delta);
        }
    }

    /// @emoji 🖼️ Centers the concrete-forest reference and moves it forward from the authored base corner.
    fn forest_reference_origin(reference_z: f64) -> [f64; 3] {
        let height_world = CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX;
        [CAD_FOREST_REFERENCE_BASE_ORIGIN_XY[0] + CAD_FOREST_REFERENCE_WIDTH_WORLD * 0.5, CAD_FOREST_REFERENCE_BASE_ORIGIN_XY[1] + height_world * (0.5 + CAD_FOREST_REFERENCE_Y_OFFSET_RATIO), reference_z]
    }

    fn translate_mesh_positions(mesh: &mut MeshData, offset: [f32; 3]) {
        for vertex in mesh.positions.as_chunks_mut::<3>().0 {
            vertex[0] += offset[0];
            vertex[1] += offset[1];
            vertex[2] += offset[2];
        }
        for segment in mesh.edge_positions.as_chunks_mut::<6>().0 {
            segment[0] += offset[0];
            segment[1] += offset[1];
            segment[2] += offset[2];
            segment[3] += offset[0];
            segment[4] += offset[1];
            segment[5] += offset[2];
        }
    }

    /// @emoji 🗃️ Reads one pane's objects and geometry from the shared quad fixture.
    pub(crate) fn cad_document_pane_bundle(source_json: &str, model_index: usize) -> (Vec<CadObject>, CadGeometry) {
        let Ok(root) = protocol::json::parse(source_json) else {
            return (Vec::new(), CadGeometry::default());
        };
        let geometry_value = root.pointer(&format!("/models/{model_index}/model/geometry")).map(protocol::json::to_dsl_value);
        let geometry = parse_geometry(geometry_value.as_ref());
        let Some(objects_value) = root.pointer(&format!("/models/{model_index}/model/objects")).and_then(|value| value.as_array()) else {
            return (Vec::new(), geometry);
        };
        let objects_value: Vec<protocol::DslValue> = objects_value.iter().map(protocol::json::to_dsl_value).collect();
        let mut kernel = cad_brep_kernel();
        let objects = objects_from_fixture_model(&mut kernel, &objects_value, &geometry);
        (objects, geometry)
    }

    /// @emoji 🌲️ `cad_document_pane_bundle`, scoped to the Concrete Forest Left fixture and keyed by
    /// `CadPaneId` rather than a raw fixture index — the real, non-stub object+geometry source
    /// `crate::editor::cad::forest_working_scene` (the app layer's `CadWorkingScene` test/render
    /// fixture) builds each pane from.
    pub(crate) fn forest_pane_bundle(pane: CadPaneId) -> (Vec<CadObject>, CadGeometry) {
        let model_index = match pane {
            CadPaneId::Shape => CAD_MODEL_INDEX_SHAPE,
            CadPaneId::Building => CAD_MODEL_INDEX_BUILDING,
            CadPaneId::Energy => CAD_MODEL_INDEX_ENERGY,
            CadPaneId::StructureClassic => CAD_MODEL_INDEX_STRUCTURE_CLASSIC,
        };
        cad_document_pane_bundle(FOREST_LEFT_MODEL_JSON, model_index)
    }

    fn forest_references_for_model_definitions(reference_z: f64) -> std::collections::BTreeMap<String, Vec<CadReference>> {
        CadPaneId::all()
            .into_iter()
            .map(|pane| {
                (
                    pane.model_definition_id().into(),
                    vec![CadReference {
                        id: "ref-concrete-forest".into(),
                        source_url: CAD_CONCRETE_FOREST_REFERENCE_URL.into(),
                        media_kind: "image".into(),
                        origin: forest_reference_origin(reference_z),
                        orientation: None,
                        scale: None,
                        width_world: CAD_FOREST_REFERENCE_WIDTH_WORLD,
                        hidden: false,
                        locked: true,
                        opacity: Some(1.0),
                    }],
                )
            })
            .collect()
    }

    pub fn typology_mesh_kind(typology: &str) -> &'static str {
        match typology {
            "building.building.column" | "structure.structure.reinforcedconcretecolumn" | "aec.building.column" => "cylinder",
            _ => "box",
        }
    }

    /// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the single-box placeholder
    /// object this used to inline directly is gone — `CadSnapshot` no longer carries inline object
    /// data, only composed `s.stdio.semio.model` child HANDLES (minted by the host, out of a pure
    /// function's reach). The default document now starts with no model children set; a caller that
    /// wants the placeholder box back mints a `SemioModelSnapshot` child (via `model_element_from_solid_handle`-
    /// style construction) and dispatches `create-shape-model` against the result. Documented gap,
    /// not silently dropped.
    pub fn default_document() -> CadSnapshot {
        CadSnapshot {
            schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
            id: "cad".into(),
            shape_model: None,
            building_model: None,
            energy_model: None,
            structure_classic_model: None,
            drawings: Vec::new(),
            nodes: vec![CadNode { id: "node-root".into(), label: "Model".into(), kind: "group".into() }, CadNode { id: "node-box".into(), label: "Box".into(), kind: "solid".into() }],
            references_by_model_definition_id: std::collections::BTreeMap::new(),
            active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
        }
    }

    /// 🌉️ Mints pane `pane`'s composed `s.stdio.semio.model` child handle from its working
    /// objects, content-addressed exactly like `scene_from_spatial_payload`/`cad_document_from_dwg`,
    /// then attaches `scene` as the handle's local-only materialization via
    /// `ArtifactChild::with_local_owner` — the same seam `flow`/`dag`/`jack`/`wires`/`sequence`
    /// already use to keep in-process content beside a content-addressed handle when no host-level
    /// child resolver has materialized it yet. An empty pane mints no child, matching this file's
    /// existing "no fabricated child" rule.
    fn cad_model_child_for_pane(pane: CadPaneId, objects: &[CadObject], scene: Arc<CadWorkingScene>) -> Option<CadModelChild> {
        if objects.is_empty() {
            return None;
        }
        let content_json = protocol::json::to_json_string(&semio_model_snapshot_from_objects(objects));
        Some(cad_model_child_handle(pane, &content_json).with_local_owner(scene))
    }

    /// @emoji 📟️ Builds the quad play document: shape/building/energy/structure-classic panes each
    /// sourced from their own model definition inside the shared fixture JSON via
    /// `cad_document_pane_bundle` — the real importer, never a parallel one. Empty panes stay empty —
    /// never collapse to `default_document` (that single-box placeholder was the cut-concrete bug).
    /// Each non-empty pane's objects are minted into a real `shape_model`/`building_model`/
    /// `energy_model`/`structure_classic_model` child (`cad_model_child_for_pane`), carrying the
    /// full `CadWorkingScene` (objects AND the fixture's raw wire/vertex `CadGeometry` — lost by
    /// `SemioModelSnapshot`'s own schema) as its local-only materialization so
    /// `build_world_scene_for_pane` can read real geometry back out at render time.
    fn forest_play_document(source_json: &str, id: &str) -> CadSnapshot {
        let (shape_objects, shape_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_SHAPE);
        let (building_objects, building_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_BUILDING);
        let (energy_objects, energy_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_ENERGY);
        let (structure_classic_objects, structure_classic_geometry) = cad_document_pane_bundle(source_json, CAD_MODEL_INDEX_STRUCTURE_CLASSIC);
        let scene = Arc::new(CadWorkingScene {
            objects: shape_objects.clone(),
            geometry: Some(shape_geometry),
            building_objects: building_objects.clone(),
            building_geometry: Some(building_geometry),
            energy_objects: energy_objects.clone(),
            energy_geometry: Some(energy_geometry),
            structure_classic_objects: structure_classic_objects.clone(),
            structure_classic_geometry: Some(structure_classic_geometry),
        });
        CadSnapshot {
            schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
            id: id.into(),
            shape_model: cad_model_child_for_pane(CadPaneId::Shape, &shape_objects, scene.clone()),
            building_model: cad_model_child_for_pane(CadPaneId::Building, &building_objects, scene.clone()),
            energy_model: cad_model_child_for_pane(CadPaneId::Energy, &energy_objects, scene.clone()),
            structure_classic_model: cad_model_child_for_pane(CadPaneId::StructureClassic, &structure_classic_objects, scene.clone()),
            drawings: Vec::new(),
            nodes: vec![CadNode { id: "node-root".into(), label: "Concrete Forest Left".into(), kind: "group".into() }],
            references_by_model_definition_id: forest_references_for_model_definitions(CAD_FOREST_REFERENCE_PLANE_Z),
            active_model_definition_id: CAD_MODEL_DEFINITION_SHAPE.into(),
        }
    }

    /// @emoji 🌲️ The Concrete Forest Left example projection — a bare `CadSnapshot` (no runtime/history),
    /// wrapped into a `ArtifactStore` by `VcsArtifactApp` when spawned. Cached so manifest registration,
    /// `initial_snapshot`, and `setActiveExample` share one BREP import instead of rebuilding thrice.
    pub fn forest_play_scene() -> CadSnapshot {
        static FOREST_PLAY_SCENE: OnceLock<CadSnapshot> = OnceLock::new();
        FOREST_PLAY_SCENE.get_or_init(|| forest_play_document(FOREST_LEFT_MODEL_JSON, CAD_EXAMPLE_FOREST_LEFT)).clone()
    }

    pub fn next_cad_id(prefix: &str) -> String {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let next = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("{prefix}-{next}")
    }

    /// 🌲️ The initial per-pane camera for the Concrete Forest Left example — session-only runtime state
    /// now (camera moved off `CadSnapshot`), matching the pose the document used to carry before the
    /// camera-as-View-action refactor.
    pub fn forest_play_camera() -> CadCamera {
        CadCamera { position: [12.0, -12.0, 8.0], target: [5.4, 2.34, 1.5], zoom: 1.0, fov: 50.0, projection: CadProjectionDsl::default() }
    }

    /// 📐️ Converts `camera.projection`'s local DSL twin into the shared taxonomy config — field-for-field,
    /// since `CadProjectionDsl` mirrors `WorldProjectionConfig` exactly (see its doc comment in `cad/rs`).
    pub fn cad_camera_projection_config(camera: &CadCamera) -> WorldProjectionConfig {
        let p = &camera.projection;
        WorldProjectionConfig {
            kind: p.kind.clone(),
            orthographic_view: p.orthographic_view.clone(),
            axonometric_variant: p.axonometric_variant.clone(),
            axonometric_angle_a: p.axonometric_angle_a,
            axonometric_angle_b: p.axonometric_angle_b,
            axonometric_quadrant: p.axonometric_quadrant.clone(),
            oblique_variant: p.oblique_variant.clone(),
            oblique_angle: p.oblique_angle,
            oblique_depth: p.oblique_depth,
            one_point_axis: p.one_point_axis.clone(),
            fov: p.fov,
            two_point_shift: p.two_point_shift,
            curvilinear_fov: p.curvilinear_fov,
            curvilinear_strength: p.curvilinear_strength,
            curvilinear_mapping: p.curvilinear_mapping.clone(),
        }
    }

    /// 📐️ Writes a taxonomy config back into `camera.projection`'s local DSL twin slot.
    pub fn cad_camera_set_projection_config(camera: &mut CadCamera, config: &WorldProjectionConfig) {
        camera.projection = CadProjectionDsl {
            kind: config.kind.clone(),
            orthographic_view: config.orthographic_view.clone(),
            axonometric_variant: config.axonometric_variant.clone(),
            axonometric_angle_a: config.axonometric_angle_a,
            axonometric_angle_b: config.axonometric_angle_b,
            axonometric_quadrant: config.axonometric_quadrant.clone(),
            oblique_variant: config.oblique_variant.clone(),
            oblique_angle: config.oblique_angle,
            oblique_depth: config.oblique_depth,
            one_point_axis: config.one_point_axis.clone(),
            fov: config.fov,
            two_point_shift: config.two_point_shift,
            curvilinear_fov: config.curvilinear_fov,
            curvilinear_strength: config.curvilinear_strength,
            curvilinear_mapping: config.curvilinear_mapping.clone(),
        };
    }

    /// 📐️ Distance from `camera.position` to `camera.target`, defaulting to the historic orbit radius when degenerate.
    pub fn cad_camera_distance(camera: &CadCamera) -> f64 {
        let [dx, dy, dz] = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        if distance > 1e-3 {
            distance
        } else {
            20.0
        }
    }

    pub(crate) fn ensure_object_solid_handle(kernel: &mut Brep, object: &mut CadObject) {
        if object.solid_handle.is_some() {
            return;
        }
        if let Some(handle) = super::solid_for_object(kernel, object) {
            let primitive_id = handle.0;
            object.solid_handle = Some(primitive_id.clone());
            if object.primitives.is_empty() {
                object.primitives.push(CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() });
            }
        }
    }

    pub(crate) fn resolve_object_mesh_url(object: &CadObject) -> Option<String> {
        object.mesh_url.as_ref().filter(|url| !url.is_empty()).cloned()
    }

    pub(crate) fn primary_primitive_kind(object: &CadObject) -> &str {
        object.primitives.first().map_or("solid", |primitive| primitive.kind.as_str())
    }

    pub(crate) fn object_mesh_data(object: &CadObject, geometry: Option<&CadGeometry>) -> MeshData {
        let kind = primary_primitive_kind(object);
        {
            let mut kernel = cad_brep_kernel();
            let mesh = geometry.filter(|_| !object.primitives.is_empty()).and_then(|geometry| tessellate_object_mesh_from_fixture(&mut kernel, object, geometry)).or_else(|| tessellate_object_mesh(&mut kernel, object, kind));
            if let Some(mut mesh) = mesh {
                if let Some(geometry) = geometry {
                    align_mesh_to_fixture_centroid(&mut mesh, geometry, &object.primitives);
                }
                return mesh;
            }
        }
        let centroid = geometry.and_then(|geometry| centroid_from_fixture_primitives(geometry, &object.primitives));
        typology_brep_mesh(&object.typology, object.extent, object.solid_handle.as_deref(), centroid)
    }

    pub(crate) fn collect_mesh_urls(objects: &[CadObject]) -> Vec<String> {
        let mut urls = HashSet::new();
        for object in objects {
            if let Some(url) = resolve_object_mesh_url(object) {
                urls.insert(url);
            }
        }
        urls.into_iter().collect()
    }

    pub(crate) fn object_scale_json(object: &CadObject) -> [f64; 3] {
        object.scale.unwrap_or([1.0, 1.0, 1.0])
    }

    /// @emoji 🧵️ Tessellates a representative mesh for the OS mesh-exporter boundary — the document's
    /// first object across panes, or the default box typology for an empty scene (no runtime selection
    /// exists at this boundary).
    /// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: this used to scan the
    /// document's inline object list for a representative mesh. `CadSnapshot` no longer carries
    /// inline objects (only composed model child HANDLES, unresolved at this boundary) — falls back
    /// to the default box typology unconditionally. Documented reduced-fidelity gap, not silently
    /// wrong: `document`'s model-child handles are available via `crate::artifacts::cad::
    /// cad_pane_model` for a caller that has ALSO resolved the child content and wants to do better.
    pub fn export_mesh_from_scene(document: &CadSnapshot) -> MeshData {
        let _ = document;
        typology_brep_mesh("spatial.shape.primitive.box", None, None, None)
    }

    //#region 🧩️Contributions
    const CAD_COMPUTER_TOPIC: &str = "cad.computer";

    /// 🗂️ `cad.computer` topic payload shape (`TopicContribution` counterpart, ex `Contribution::CadComputer`).
    #[derive(semio_framework_value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct CadComputerTopicPayload {
        app_id: String,
        module_id: String,
        computers_json: String,
    }

    /// 🗂️ Reads the open `TopicContribution` (`"cad.computer"` topic) shape per entry.
    fn cad_computer_fields(entry: &semio_framework::ProgramContributionEntry) -> Option<(String, String, String)> {
        let topic_contribution = entry.topic_contribution.as_ref()?;
        if topic_contribution.topic != CAD_COMPUTER_TOPIC {
            return None;
        }
        let payload = topic_contribution.decode::<CadComputerTopicPayload>().ok()?;
        Some((payload.app_id, payload.module_id, payload.computers_json))
    }

    /// 🧩️ Validates host-pushed `CadComputer` contributions for `cad-play` (implementations register in cad-js).
    pub fn validate_cad_computer_contributions(contributions_json: &str) {
        for entry in parse_contributions(contributions_json) {
            let Some((app_id, module_id, computers_json)) = cad_computer_fields(&entry) else {
                continue;
            };
            if app_id != "cad-play" {
                continue;
            }
            let _ = (module_id, computers_json);
        }
    }
    //#endregion 🧩️Contributions
}
pub use scene_compute::*;
//#endregion 🖥️SceneCompute
