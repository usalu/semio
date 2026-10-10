//! 📏️ The subject's report of a glTF export, in the shape the `three` `GLTFLoader` oracle measures from the committed file: node, mesh, primitive, triangle and material counts, the
//! element nodes per kind, the elements and triangles per storey, and the world bounds of every vertex in glTF's Y-up frame. Built from the typed document, never from the bytes.

use super::document::{GltfModel, GltfNode};
use std::collections::BTreeMap;

/// 📊️ Elements and triangles of one storey.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoreyCount {
    pub elements: usize,
    pub triangles: usize,
}

/// 📊️ The report of one export.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    pub nodes: usize,
    pub meshes: usize,
    pub primitives: usize,
    pub triangles: usize,
    pub materials: usize,
    pub kinds: BTreeMap<String, usize>,
    pub storeys: BTreeMap<String, StoreyCount>,
    pub volumes: BTreeMap<String, f64>,
    pub min: [f64; 3],
    pub max: [f64; 3],
}

fn rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let (u, w) = ([q[0], q[1], q[2]], q[3]);
    let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let t = cross(u, v);
    let t = [t[0] + w * v[0], t[1] + w * v[1], t[2] + w * v[2]];
    let c = cross(u, t);
    [v[0] + 2.0 * c[0], v[1] + 2.0 * c[1], v[2] + 2.0 * c[2]]
}

/// 📍️ A point through a node's rotation and translation (the order of the spec: scale, rotate, translate).
pub fn through(node: &GltfNode, v: [f64; 3]) -> [f64; 3] {
    let v = node.rotation.map_or(v, |q| rotate(q, v));
    let t = node.translation.unwrap_or_default();
    [v[0] + t[0], v[1] + t[1], v[2] + t[2]]
}

/// 🧊️ The volume a primitive encloses: the sum of the signed tetrahedra over the origin of each triangle, from the 32-bit vertices as stored.
fn volume_of(primitive: &super::document::GltfPrimitive) -> f64 {
    let corner = |index: u32| {
        let at = index as usize * 3;
        [f64::from(primitive.positions[at]), f64::from(primitive.positions[at + 1]), f64::from(primitive.positions[at + 2])]
    };
    primitive
        .indices
        .chunks_exact(3)
        .map(|triangle| {
            let (a, b, c) = (corner(triangle[0]), corner(triangle[1]), corner(triangle[2]));
            (a[0] * (b[1] * c[2] - b[2] * c[1]) + a[1] * (b[2] * c[0] - b[0] * c[2]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0
        })
        .sum()
}

struct Walk<'a> {
    model: &'a GltfModel,
    report: Projection,
}

impl Walk<'_> {
    fn visit(&mut self, index: usize, chain: &mut Vec<usize>) {
        chain.push(index);
        let node = &self.model.nodes[index];
        if let Some(mesh) = node.mesh {
            let kind = node.extras.get("kind").and_then(|kind| kind.as_str()).unwrap_or_default().to_string();
            let storey = node.extras.get("storey").and_then(|storey| storey.as_str()).unwrap_or_default().to_string();
            let mesh = &self.model.meshes[mesh];
            let triangles: usize = mesh.primitives.iter().map(|primitive| primitive.triangle_count()).sum();
            *self.report.kinds.entry(kind).or_default() += 1;
            let count = self.report.storeys.entry(storey).or_default();
            count.elements += 1;
            count.triangles += triangles;
            self.report.meshes += 1;
            self.report.primitives += mesh.primitives.len();
            self.report.triangles += triangles;
            if let Some(id) = node.extras.get("id").and_then(|id| id.as_str()) {
                self.report.volumes.insert(id.to_string(), mesh.primitives.iter().map(volume_of).sum());
            }
            for primitive in &mesh.primitives {
                for point in primitive.positions.chunks_exact(3) {
                    let world = chain.iter().rev().fold([f64::from(point[0]), f64::from(point[1]), f64::from(point[2])], |at, node| through(&self.model.nodes[*node], at));
                    for axis in 0..3 {
                        self.report.min[axis] = self.report.min[axis].min(world[axis]);
                        self.report.max[axis] = self.report.max[axis].max(world[axis]);
                    }
                }
            }
        }
        for child in node.children.clone() {
            self.visit(child, chain);
        }
        chain.pop();
    }
}

/// 📊️ The report of a document.
pub fn project(model: &GltfModel) -> Projection {
    let report = Projection { nodes: model.nodes.len(), meshes: 0, primitives: 0, triangles: 0, materials: model.materials.len(), kinds: BTreeMap::new(), storeys: BTreeMap::new(), volumes: BTreeMap::new(), min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3] };
    let mut walk = Walk { model, report };
    for root in &model.roots {
        walk.visit(*root, &mut Vec::new());
    }
    let mut report = walk.report;
    if report.triangles == 0 {
        report.min = [0.0; 3];
        report.max = [0.0; 3];
    }
    report
}

fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

impl Projection {
    /// 🧾️ The report as compact JSON, the shape the oracle returns.
    pub fn to_json(&self) -> String {
        let kinds = self.kinds.iter().map(|(kind, count)| format!("{}:{count}", quote(kind))).collect::<Vec<_>>().join(",");
        let storeys = self.storeys.iter().map(|(storey, count)| format!("{}:{{\"elements\":{},\"triangles\":{}}}", quote(storey), count.elements, count.triangles)).collect::<Vec<_>>().join(",");
        let volumes = self.volumes.iter().map(|(id, volume)| format!("{}:{volume:?}", quote(id))).collect::<Vec<_>>().join(",");
        let point = |p: [f64; 3]| format!("[{:?},{:?},{:?}]", p[0], p[1], p[2]);
        format!(
            "{{\"nodes\":{},\"meshes\":{},\"primitives\":{},\"triangles\":{},\"materials\":{},\"kinds\":{{{kinds}}},\"storeys\":{{{storeys}}},\"volumes\":{{{volumes}}},\"bounds\":{{\"min\":{},\"max\":{}}}}}",
            self.nodes,
            self.meshes,
            self.primitives,
            self.triangles,
            self.materials,
            point(self.min),
            point(self.max)
        )
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
