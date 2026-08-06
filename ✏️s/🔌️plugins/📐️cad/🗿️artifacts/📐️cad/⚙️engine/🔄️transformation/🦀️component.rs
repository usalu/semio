//! 🔄️ CAD derive-transformation engine — ports premigration `runDeriveTransformation` onto `kernel_3d_brepkit`.

use crate::artifacts::cad::{CadObject, CadPrimitiveSlot};

use semio_s_3d::brep::engine::{BrepKernel, GeometryHandle, Vec3};
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
/// @emoji 📍️ Face centroid via surface midpoint sampling (premigration `faceCentroid` equivalent).
pub fn face_centroid_sync(kernel: &dyn BrepKernel, face: &GeometryHandle) -> Option<Vec3> {
    semio_s_3d::brep::engine::block_on(kernel.surface_point(face, 0.5, 0.5)).ok()
}

/// @emoji 🧭️ Face outward normal at the surface midpoint.
pub fn face_normal_sync(kernel: &dyn BrepKernel, face: &GeometryHandle) -> Option<Vec3> {
    semio_s_3d::brep::engine::block_on(kernel.surface_normal(face, 0.5, 0.5)).ok()
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

fn axis_normal_component(normal: Vec3, axis: DominantAxis) -> f64 {
    match axis {
        DominantAxis::X => normal[0].abs(),
        DominantAxis::Y => normal[1].abs(),
        DominantAxis::Z => normal[2].abs(),
    }
}

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
pub fn solid_for_object(kernel: &mut dyn BrepKernel, object: &CadObject) -> Option<GeometryHandle> {
    if let Some(handle) = object.solid_handle.as_ref() {
        if semio_s_3d::brep::engine::block_on(kernel.kind(&GeometryHandle(handle.clone()))).is_ok() {
            return Some(GeometryHandle(handle.clone()));
        }
    }
    let [ex, ey, ez] = object.extent.unwrap_or([1.0, 1.0, 1.0]);
    let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
    let is_cylindrical = object.typology.contains("column");
    let handle = if is_cylindrical { semio_s_3d::brep::engine::block_on(kernel.cylinder_prim(width.max(depth) * 0.5, height)).ok() } else { semio_s_3d::brep::engine::block_on(kernel.box_prim(width, depth, height)).ok() }?;
    Some(handle)
}

/// @emoji 📦️ Builds a kernel solid sized from extent without mutating the object.
pub fn build_solid_for_typology(kernel: &mut dyn BrepKernel, typology: &str, extent: [f64; 3]) -> Option<GeometryHandle> {
    let [ex, ey, ez] = extent;
    let (width, depth, height) = (ex.max(0.05), ey.max(0.05), ez.max(0.05));
    if typology.contains("column") {
        semio_s_3d::brep::engine::block_on(kernel.cylinder_prim(width.max(depth) * 0.5, height)).ok()
    } else {
        semio_s_3d::brep::engine::block_on(kernel.box_prim(width, depth, height)).ok()
    }
}

fn fuse_solids(kernel: &mut dyn BrepKernel, solids: &[GeometryHandle]) -> Option<GeometryHandle> {
    if solids.is_empty() {
        return None;
    }
    let mut current = solids[0].clone();
    for solid in solids.iter().skip(1) {
        current = semio_s_3d::brep::engine::block_on(kernel.fuse(&current, solid)).ok()?;
    }
    Some(current)
}
//#endregion 🔖️SolidConstruction

//#region 🔖️DeriveEngine
struct FaceMeta {
    handle: GeometryHandle,
    normal: Vec3,
    centroid: Vec3,
}

fn next_object_id(prefix: &str, index: usize) -> String {
    format!("{prefix}-{index}")
}

/// @emoji 🔄️ Derives energy objects from shape-pane solids via fuse + face classification.
pub fn run_derive_from_geometry(kernel: &mut dyn BrepKernel, source_objects: &[CadObject], id_seed: &str) -> Vec<CadObject> {
    let solids: Vec<GeometryHandle> = source_objects.iter().filter_map(|object| solid_for_object(kernel, object)).collect();
    if solids.is_empty() {
        return Vec::new();
    }
    let hull = match fuse_solids(kernel, &solids) {
        Some(hull) => hull,
        None => return Vec::new(),
    };
    let topology = match semio_s_3d::brep::engine::block_on(kernel.deconstruct(&hull)) {
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

const BUILDING_TO_STRUCTURE: &[(&str, &str)] = &[
    ("building.building.slab", "structure.structure.onewayreinforcedconcreteslab"),
    ("building.building.column", "structure.structure.reinforcedconcretecolumn"),
    ("building.building.beam", "structure.structure.reinforcedconcretebeam"),
    ("building.building.wall", "structure.structure.reinforcedconcreteinternalwall"),
    ("aec.building.slab", "structure.structure.onewayreinforcedconcreteslab"),
    ("aec.building.column", "structure.structure.reinforcedconcretecolumn"),
];

/// @emoji 🔄️ Maps building typologies to structure-classic equivalents (premigration `from_building` applier).
pub fn apply_from_building(source_objects: &[CadObject], id_seed: &str) -> Vec<CadObject> {
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
pub fn apply_typology_fallback(source_objects: &[CadObject], typologies: &[&str], id_seed: &str) -> Vec<CadObject> {
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
    use semio_s_3d::brep::kernel::BrepkitKernel;

    #[test]
    fn derive_from_geometry_classifies_box() {
        let mut kernel = BrepkitKernel::new();
        let solid = semio_s_3d::brep::engine::block_on(kernel.box_prim(2.0, 2.0, 3.0)).expect("box");
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
        assert!(derived.iter().any(|object| object.typology == "energy.energy.hull"));
        assert!(derived.iter().any(|object| object.typology == "energy.energy.roof" || object.typology == "energy.energy.baseplate"));
        assert!(derived.iter().any(|object| object.typology == "energy.energy.externalwall"));
        assert!(derived.iter().any(|object| object.typology == "energy.energy.windows"));
    }

    #[test]
    fn face_plane_group_key_is_stable() {
        let key = face_plane_group_key([0.0, 0.0, 1.0], [1.0, 2.0, 3.0]);
        assert!(key.starts_with("z:1:"));
    }
}
