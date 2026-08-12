//! 📦 `bounds` — one named inference: 3d bounding box across every pane's object origins and
//! brep vertex positions, plus object/vertex counts. All four panes (shape/building/energy/
//! structure-classic) contribute uniformly — cad's four model definitions share one document.
//! Simple whole-snapshot scalar: no `InferredField` caching, a full O(objects+vertices) pass is
//! cheap at cad-document scale.

use crate::artifacts::cad::{CadGeometry, CadObject, CadSnapshot};
use serde::{Deserialize, Serialize};

//#region 📦Bounds
/// 📦 Axis-aligned 3d bounding box.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

fn grow(bounds: Option<CadBounds>, point: [f64; 3]) -> CadBounds {
    match bounds {
        Some(bounds) => CadBounds {
            min: [bounds.min[0].min(point[0]), bounds.min[1].min(point[1]), bounds.min[2].min(point[2])],
            max: [bounds.max[0].max(point[0]), bounds.max[1].max(point[1]), bounds.max[2].max(point[2])],
        },
        None => CadBounds { min: point, max: point },
    }
}

/// 🗂️ The four (objects, geometry) panes cad's document carries, in schema-declaration order.
fn panes(snapshot: &CadSnapshot) -> [(&[CadObject], &Option<CadGeometry>); 4] {
    [
        (snapshot.objects.as_slice(), &snapshot.shape_geometry),
        (snapshot.building_objects.as_slice(), &snapshot.building_geometry),
        (snapshot.energy_objects.as_slice(), &snapshot.energy_geometry),
        (snapshot.structure_classic_objects.as_slice(), &snapshot.structure_classic_geometry),
    ]
}

/// 📦 3d bounding box across every pane's object origins and brep vertex positions, or `None` when
/// the document carries neither.
pub(crate) fn scene_bounds(snapshot: &CadSnapshot) -> Option<CadBounds> {
    let mut bounds = None;
    for (objects, geometry) in panes(snapshot) {
        for object in objects {
            bounds = Some(grow(bounds, object.origin));
        }
        if let Some(geometry) = geometry {
            for vertex in &geometry.vertices {
                bounds = Some(grow(bounds, vertex.position));
            }
        }
    }
    bounds
}

/// 📦 Total object count across every pane.
pub(crate) fn object_count(snapshot: &CadSnapshot) -> usize {
    panes(snapshot).into_iter().map(|(objects, _)| objects.len()).sum()
}

/// 📦 Total brep vertex count across every pane's geometry.
pub(crate) fn vertex_count(snapshot: &CadSnapshot) -> usize {
    panes(snapshot).into_iter().map(|(_, geometry)| geometry.as_ref().map_or(0, |geometry| geometry.vertices.len())).sum()
}
//#endregion 📦Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::cad::{empty_cad_snapshot, CadVertex};

    #[test]
    fn empty_scene_has_no_bounds() {
        let snapshot = empty_cad_snapshot();
        assert!(scene_bounds(&snapshot).is_none());
        assert_eq!(object_count(&snapshot), 0);
        assert_eq!(vertex_count(&snapshot), 0);
    }

    #[test]
    fn object_origins_and_vertex_positions_both_grow_the_box() {
        let mut snapshot = empty_cad_snapshot();
        snapshot.objects.push(CadObject {
            id: "o1".into(),
            label: "O1".into(),
            typology: "generic".into(),
            visible: true,
            locked: false,
            origin: [1.0, 2.0, 3.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: None,
            primitives: Vec::new(),
        });
        snapshot.shape_geometry = Some(CadGeometry {
            anchors: Vec::new(),
            vertices: vec![CadVertex { id: "v1".into(), position: [-1.0, 0.0, 5.0] }],
            edges: Vec::new(),
            wires: Vec::new(),
            faces: Vec::new(),
            shells: Vec::new(),
            solids: Vec::new(),
        });
        let bounds = scene_bounds(&snapshot).expect("bounds present");
        assert_eq!(bounds, CadBounds { min: [-1.0, 0.0, 3.0], max: [1.0, 2.0, 5.0] });
        assert_eq!(object_count(&snapshot), 1);
        assert_eq!(vertex_count(&snapshot), 1);
    }
}
//#endregion 🧪️Tests
