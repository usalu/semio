//! 📄️ Publishes whole viewport instances in bounded retained scene pages.

use super::{world3d_scene_with_geometry, LoweredViewport};
use semio_framework_pack_json::{parse, to_string, JsonMemberPolicy, Value};
use semio_plugin::{scene_surface, ui_assembly_error, BuiltNode, UiAssemblyResult, World3dSceneLane};
use semio_ui_contract::{ActionBinding, SurfaceKind, UiText, UI_BUILT_CHILDREN_MAX, UI_DOCUMENT_NODES, UI_FIXED_BYTES, UI_FIXED_LIST_ITEMS, UI_RESIDENT_SURFACE_BYTES, UI_RESIDENT_SURFACE_ITEMS, UI_TEXT_MAX_BYTES};
use std::{collections::{HashMap, HashSet}, mem::size_of, ops::Range};

#[path = "🕸️mesh/🦀️.rs"]
mod mesh;

struct Instance {
    json: String,
    mesh: usize,
    owner: String,
}

struct Mesh {
    id: String,
    source: usize,
    vertices: Option<Vec<usize>>,
    json: String,
}

#[derive(Default)]
struct Page {
    instances: Range<usize>,
    meshes: HashSet<usize>,
    mesh_bytes: usize,
    instance_bytes: usize,
}

#[derive(Clone, Copy)]
struct Footprint {
    nodes: usize,
    items: usize,
    bytes: usize,
}

impl Footprint {
    fn plus(self, other: Self) -> Self {
        Self { nodes: self.nodes.saturating_add(other.nodes), items: self.items.saturating_add(other.items), bytes: self.bytes.saturating_add(other.bytes) }
    }

    fn fits(self, maximum: Self) -> bool {
        self.nodes <= maximum.nodes && self.items <= maximum.items && self.bytes <= maximum.bytes
    }
}

const SURFACE_LIMIT: Footprint = Footprint { nodes: UI_DOCUMENT_NODES, items: UI_RESIDENT_SURFACE_ITEMS, bytes: UI_RESIDENT_SURFACE_BYTES };
const OWNER_COPY_FACTOR: usize = 3;
const NODE_BACKING_BYTES: usize = OWNER_COPY_FACTOR * size_of::<BuiltNode>();
const BRANCH_BACKING_BYTES: usize = OWNER_COPY_FACTOR * UI_BUILT_CHILDREN_MAX * size_of::<Option<Box<BuiltNode>>>();
const CHROME_NODE_BYTES: usize = NODE_BACKING_BYTES + BRANCH_BACKING_BYTES + OWNER_COPY_FACTOR * 2 * size_of::<ActionBinding>();
const ATTRIBUTE_BACKING_BYTES: usize = OWNER_COPY_FACTOR * UI_FIXED_LIST_ITEMS * size_of::<(UiText, UiText)>();
const FIXED_CHROME_NODE_ITEMS: usize = 32;

/// 🧾️ Bounds the retained text pieces by their shortest possible UTF-8 chunk, then prices each packed leaf's value, attribute entries and backing.
fn carrier_footprint(bytes: usize) -> Footprint {
    let slice_bytes = UI_TEXT_MAX_BYTES - (char::MAX.len_utf8() - 1);
    let slices = bytes.div_ceil(slice_bytes).max(1);
    let leaves = slices.div_ceil(UI_FIXED_LIST_ITEMS + 1);
    let attributed_leaves = leaves - usize::from((slices - 1) % (UI_FIXED_LIST_ITEMS + 1) == 0);
    let mut width = leaves;
    let mut branches = 1;
    while width > UI_BUILT_CHILDREN_MAX {
        width = width.div_ceil(UI_BUILT_CHILDREN_MAX);
        branches += width;
    }
    let nodes = leaves + branches;
    Footprint {
        nodes,
        items: branches * 18 + leaves * 17 + attributed_leaves * 7 + (slices - leaves) * 3,
        bytes: nodes * NODE_BACKING_BYTES + branches * BRANCH_BACKING_BYTES + attributed_leaves * ATTRIBUTE_BACKING_BYTES,
    }
}

fn geometry_fits(fixed: Footprint, mesh_bytes: usize, instance_bytes: usize) -> bool {
    fixed.plus(carrier_footprint(2 + mesh_bytes)).plus(carrier_footprint(2 + instance_bytes)).fits(SURFACE_LIMIT)
}

impl Page {
    fn admit(&mut self, instance: &Instance, meshes: &[Mesh], fixed: Footprint) -> bool {
        let mesh_bytes = self.mesh_bytes + if self.meshes.contains(&instance.mesh) { 0 } else { meshes[instance.mesh].json.len() + 1 };
        let instance_bytes = self.instance_bytes + instance.json.len() + 1;
        if !geometry_fits(fixed, mesh_bytes, instance_bytes) {
            return false;
        }
        self.meshes.insert(instance.mesh);
        self.mesh_bytes = mesh_bytes;
        self.instance_bytes = instance_bytes;
        self.instances.end += 1;
        true
    }
}

fn plan_pages(instances: &[Instance], meshes: &[Mesh], fixed: Footprint) -> UiAssemblyResult<Vec<Page>> {
    let mut pages = vec![Page::default()];
    for (index, instance) in instances.iter().enumerate() {
        if pages.last_mut().is_some_and(|page| page.admit(instance, meshes, fixed)) {
            continue;
        }
        let mut page = Page { instances: index..index, ..Page::default() };
        if !page.admit(instance, meshes, fixed) {
            return Err(ui_assembly_error("scene.viewport.geometry-page"));
        }
        pages.push(page);
    }
    Ok(pages)
}

fn partition_geometry(lowered: &LoweredViewport, fixed: Footprint) -> UiAssemblyResult<(Vec<Mesh>, Vec<Instance>)> {
    let id_of = |value: &Value| value.get("id").and_then(Value::as_str).map(str::to_owned)
        .ok_or_else(|| ui_assembly_error("scene.viewport.mesh-identity"));
    let mut meshes = lowered.meshes.iter().enumerate().map(|(source, mesh)| {
        Ok(Mesh { id: id_of(mesh)?, source, vertices: None, json: to_string(mesh) })
    }).collect::<UiAssemblyResult<Vec<_>>>()?;
    let by_id: HashMap<_, _> = meshes.iter().map(|mesh| (mesh.id.clone(), mesh.source)).collect();
    let mut fragments = HashMap::<usize, Vec<usize>>::new();
    let mut instances = Vec::with_capacity(lowered.instances.len());
    for instance in &lowered.instances {
        let source = instance.get("meshId").and_then(Value::as_str).and_then(|id| by_id.get(id)).copied()
            .ok_or_else(|| ui_assembly_error("scene.viewport.mesh-reference"))?;
        let owner = instance.get("interactionId").and_then(Value::as_str).map(str::to_owned).map_or_else(|| id_of(instance), Ok)?;
        let record = Instance { json: to_string(instance), mesh: source, owner };
        if geometry_fits(fixed, meshes[source].json.len() + 1, record.json.len() + 1) {
            instances.push(record);
            continue;
        }
        if let std::collections::hash_map::Entry::Vacant(cached) = fragments.entry(source) {
            let begin = meshes.len();
            let pieces = mesh::split(&lowered.meshes[source], instance, &record.owner, &meshes[source], fixed)?;
            meshes.extend(pieces);
            cached.insert((begin..meshes.len()).collect());
        }
        for &index in &fragments[&source] {
            let json = mesh::instance_json(instance, &record.owner, &meshes[index].id)?;
            instances.push(Instance { json, mesh: index, owner: record.owner.clone() });
        }
    }
    Ok((meshes, instances))
}

fn json_array<'a>(records: impl Iterator<Item = &'a str>) -> String {
    let mut output = String::from("[");
    for record in records {
        if output.len() > 1 {
            output.push(',');
        }
        output.push_str(record);
    }
    output.push(']');
    output
}

/// 📄️ Returns the retained World3d at a clamped page index, with room for its surrounding viewport controls.
pub fn render_viewport_page(body_key: &str, lowered: &LoweredViewport, domain_id: &str, sun: bool, requested_page: usize, additional_nodes: usize) -> UiAssemblyResult<(BuiltNode, usize, usize)> {
    let mut scene = world3d_scene_with_geometry(lowered, domain_id, sun, "[]".into(), "[]".into());
    let mut fixed_lanes = scene.clone();
    let fixed = Footprint { nodes: 1 + additional_nodes, items: 1 + (1 + additional_nodes) * FIXED_CHROME_NODE_ITEMS,
        bytes: additional_nodes * CHROME_NODE_BYTES + NODE_BACKING_BYTES + BRANCH_BACKING_BYTES
            + OWNER_COPY_FACTOR * (UI_FIXED_BYTES + UI_FIXED_LIST_ITEMS * size_of::<ActionBinding>()) };
    let fixed = World3dSceneLane::ALL.into_iter()
        .filter(|lane| !matches!(lane, World3dSceneLane::Meshes | World3dSceneLane::Instances))
        .filter_map(|lane| lane.take(&mut fixed_lanes))
        .map(|payload| carrier_footprint(payload.len())).fold(fixed, Footprint::plus);
    if !geometry_fits(fixed, 0, 0) {
        return Err(ui_assembly_error("scene.viewport.fixed-page"));
    }
    let (meshes, instances) = partition_geometry(lowered, fixed)?;
    let pages = plan_pages(&instances, &meshes, fixed)?;
    let current = requested_page.min(pages.len() - 1);
    let page = &pages[current];
    scene.meshes_json = json_array(meshes.iter().enumerate().filter_map(|(index, mesh)| page.meshes.contains(&index).then_some(mesh.json.as_str())));
    scene.instances_json = json_array(instances[page.instances.clone()].iter().map(|instance| instance.json.as_str()));
    if pages.len() > 1 {
        let visible: HashSet<_> = instances[page.instances.clone()].iter().map(|instance| instance.owner.as_str()).collect();
        if let Some(payload) = scene.pick_targets_json.take() {
            let Ok(Value::Array(mut targets)) = parse(&payload, JsonMemberPolicy::Reject) else {
                return Err(ui_assembly_error("scene.viewport.pick-page"));
            };
            targets.retain(|target| target.get("id").and_then(Value::as_str)
                .and_then(|id| id.rsplit_once(':')).is_some_and(|(owner, _)| visible.contains(owner)));
            scene.pick_targets_json = (!targets.is_empty()).then(|| to_string(&Value::Array(targets)));
        }
    }
    if let Some(mut field) = scene.scalar_field.take() {
        if let Some(index) = page.meshes.iter().copied()
            .filter(|&index| meshes[meshes[index].source].id == field.mesh_id).min() {
            let mesh = &meshes[index];
            if let Some(vertices) = &mesh.vertices {
                field.values = vertices.iter().map(|&vertex| field.values.get(vertex).copied().flatten()).collect();
                field.mesh_id = mesh.id.clone();
            }
            scene.scalar_field = Some(field);
        }
    }
    let surface = scene_surface(body_key, SurfaceKind::World3d, &scene)?;
    Ok((surface, current, pages.len()))
}
