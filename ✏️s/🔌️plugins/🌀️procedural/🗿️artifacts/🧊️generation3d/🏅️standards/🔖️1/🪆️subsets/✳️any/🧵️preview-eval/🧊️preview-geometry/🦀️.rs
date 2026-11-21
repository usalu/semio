//! 🧊️ Preview geometry of the preview evaluation: the channel items, the mesh roles, the fit, the marker meshes and the retained mesh handles. See [`super`].

use super::*;

//#region 🧊️Geometry
/// ⏱️ Face/edge units one `tessellate` STEP may spend — the granularity at which a cancel can land
/// inside the kernel, never the round trip's own bound.
pub const PREVIEW_TESSELLATE_STEP_BUDGET: u32 = 24;

/// 🕐️ Wall-clock microseconds one `tessellate` ROUND TRIP may spend stepping before it answers.
///
/// ⚖️ This is the bound that actually matters, because a unit is not a unit of time: on this kernel
/// one step measured 29 µs and another 5.9 s. Bounding only the units made five consecutive
/// microsecond-long steps each pay a WHOLE `flowEvalTick` round trip — seconds apiece in a served
/// build — which is most of why `box-fillet-preview` needed eight round trips to paint and never
/// finished inside a patience window (`📓️preview-mesh-delivery-2026-09-12.md`).
pub const PREVIEW_TESSELLATE_STEP_WALL_MICROS: u64 = semio_os_flow::mesh::TESSELLATE_STEP_WALL_MICROS;

/// 🔬️ The deflection one LOD step asks the kernel for — the ONE mapping both surfaces read, so an
/// editor mesh and a viewer mesh of the same handle at the same LOD are byte-identical.
pub fn preview_tolerance(lod_mode: &str) -> f64 {
    match lod_mode {
        "coarse" => 0.15,
        "fine" => 0.02,
        _ => 0.05,
    }
}

/// 🌊️ The flow-domain id of the geometry extension whose kernel tessellates every preview handle.
/// It is NOT an invocation address: the host resolves an extension actor by the CONTRIBUTING
/// PLUGIN's id, which [`semio_os_flow::flow_extension_invocation_address`] — the ONE
/// translation surface — looks up from the live contribution table.
pub const GENERATION_3D_GEOMETRY_EXTENSION_ID: &str = "brep";

/// ➕️ The geometry kernel's invocation address, resolved through the one translation surface every
/// producer and every status projection shares. Both callers own the same typed miss, so an
/// unaddressable kernel reaches a preview window as `phase: "faulted"` with its own English/German
/// label instead of a per-tick `eprintln!` nobody reads.
pub fn geometry_extension_address() -> Result<String, semio_os_flow::FlowExtensionAddressMiss> {
    semio_os_flow::flow_extension_invocation_address(GENERATION_3D_GEOMETRY_EXTENSION_ID)
}

pub fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() {
        return false;
    }
    if handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
    {
        return true;
    }
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

/// 🔌️ Point/vector geometry synthesized without a kernel round-trip, for a math-style output
/// channel that carries `x`/`y`/`z` coordinates instead of a brep handle.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewInlineGeometry<'eval> {
    Point { x: f64, y: f64, z: f64 },
    Vector { x: f64, y: f64, z: f64 },
    Plane { origin: [f64; 3], normal: [f64; 3] },
    Mesh { preview: &'eval str, data: &'eval str },
}

#[derive(PartialEq, Eq, Hash)]
enum InlinePreviewMeshKey<'eval> {
    Point([u64; 3]),
    Vector([u64; 3]),
    Plane([u64; 3], [u64; 3]),
    Mesh(&'eval str, &'eval str),
}

impl<'eval> PreviewInlineGeometry<'eval> {
    fn mesh_key(&self) -> InlinePreviewMeshKey<'eval> {
        let coordinates = |values: [f64; 3]| values.map(|value| if value == 0.0 { 0 } else { value.to_bits() });
        match self {
            Self::Point { x, y, z } => InlinePreviewMeshKey::Point(coordinates([*x, *y, *z])),
            Self::Vector { x, y, z } => InlinePreviewMeshKey::Vector(coordinates([*x, *y, *z])),
            Self::Plane { origin, normal } => InlinePreviewMeshKey::Plane(coordinates(*origin), coordinates(*normal)),
            Self::Mesh { preview, data } => InlinePreviewMeshKey::Mesh(preview, data),
        }
    }
}

struct NativePreviewGeometry<'mesh>(&'mesh MeshData);

fn preview_coordinate_bits(value: f32) -> u32 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

impl PartialEq for NativePreviewGeometry<'_> {
    fn eq(&self, other: &Self) -> bool {
        let equal_coordinates = |left: &[f32], right: &[f32]| {
            left.iter().copied().map(preview_coordinate_bits).eq(right.iter().copied().map(preview_coordinate_bits))
        };
        self.0.indices == other.0.indices
            && equal_coordinates(&self.0.positions, &other.0.positions)
            && equal_coordinates(&self.0.edge_positions, &other.0.edge_positions)
    }
}

impl Eq for NativePreviewGeometry<'_> {}

impl std::hash::Hash for NativePreviewGeometry<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.indices.hash(state);
        for coordinates in [&self.0.positions, &self.0.edge_positions] {
            coordinates.len().hash(state);
            for &coordinate in coordinates { preview_coordinate_bits(coordinate).hash(state); }
        }
    }
}

/// 🧩️ Reuses rendered meshes that share an exact retained handle or structured geometric value.
#[derive(Default)]
pub struct PreviewMeshIds<'eval, 'mesh> {
    native: std::collections::HashMap<String, String>,
    inline: std::collections::HashMap<InlinePreviewMeshKey<'eval>, String>,
    native_geometry: std::collections::HashMap<NativePreviewGeometry<'mesh>, String>,
    hover_aliases: std::collections::HashMap<String, String>,
}

impl<'eval, 'mesh> PreviewMeshIds<'eval, 'mesh> {
    /// 🕵️ Resolves the mesh already emitted for a geometry source.
    pub fn get(&self, native: Option<&str>, inline: Option<&PreviewInlineGeometry<'eval>>) -> Option<&String> {
        if let Some(native) = native {
            self.native.get(native)
        } else {
            self.inline.get(&inline?.mesh_key())
        }
    }

    /// 📌️ Records the emitted mesh id against its source.
    pub fn insert(&mut self, native: Option<&str>, inline: Option<&PreviewInlineGeometry<'eval>>, id: String) {
        if let Some(native) = native {
            self.native.insert(native.to_owned(), id);
        } else if let Some(inline) = inline {
            self.inline.insert(inline.mesh_key(), id);
        }
    }
    /// 🪢️ Shares hover across equal native geometry while preserving each mesh's own buffer and identity.
    pub fn track_native_hover(&mut self, mesh: &'mesh MeshData, id: &str) {
        use std::collections::hash_map::Entry;
        match self.native_geometry.entry(NativePreviewGeometry(mesh)) {
            Entry::Occupied(entry) => { self.hover_aliases.insert(id.to_owned(), entry.get().clone()); }
            Entry::Vacant(entry) => { entry.insert(id.to_owned()); }
        }
    }

    /// 📚️ Returns distinct mesh IDs that share an exact pick surface at the preview's identity pose.
    pub fn into_hover_aliases(self) -> std::collections::HashMap<String, String> {
        self.hover_aliases
    }
}

/// 🏆️ One spatial leaf at an evaluated channel's direction and structural position.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewChannelItem<'eval> {
    pub channel: String,
    pub index: usize,
    pub handle: String,
    pub inline: Option<PreviewInlineGeometry<'eval>>,
    channel_side: &'static str,
    value_path: Vec<PreviewValueStep>,
}

impl PreviewChannelItem<'_> {
    /// ⬅️ Whether this geometry is an evaluated input to its widget.
    pub fn is_input(&self) -> bool {
        self.channel_side == "in"
    }

    /// 📪️ Whether the spatial leaf is the channel value itself.
    pub fn is_channel_value(&self) -> bool {
        self.value_path.is_empty()
    }

    /// 🪪️ Keeps visual instances distinct by direction under the graph's shared port target.
    pub fn address<'a>(&'a self, widget: &'a str) -> PreviewChannelAddress<'a> {
        PreviewChannelAddress::new(widget, &self.channel, self.index, self.is_input())
    }
}

/// 🗺️ A world instance and its two ancestors in the graph mark domain.
pub struct PreviewChannelAddress<'a> {
    pub widget: &'a str,
    pub port: String,
    pub instance: String,
    input: bool,
    channel: &'a str,
}

impl<'a> PreviewChannelAddress<'a> {
    /// 🧭️ Constructs the stable world address for one output or input instance.
    pub fn new(widget: &'a str, channel: &'a str, index: usize, input: bool) -> Self {
        let port = semio_s_artifact_procedural_3d_model::graph_port_id(widget, channel, input);
        let instance = semio_s_artifact_procedural_3d_model::graph_instance_id(widget, channel, input, index);
        Self { widget, port, instance, input, channel }
    }

    /// 🏷️ Makes the input direction visible in the world instance label.
    pub fn label(&self) -> String {
        let arrow = if self.input { "← " } else { "" };
        format!("{arrow}{}@{}", self.widget, self.channel)
    }

    /// 🎯️ Accepts a graph mark on the instance's port or widget.
    pub fn marked(&self, targets: &crate::PreviewGraphTargets<'_>) -> bool {
        targets.marks(self.widget, &self.port)
    }

    /// 🔌️ Gives this instance the interaction ID of its directional port.
    pub fn interaction_id(&self) -> String {
        semio_s_artifact_procedural_3d_model::GraphTargetKind::Handle.target(&self.port)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PreviewValueStep {
    Index(usize),
    Field(String),
}

/// 🔎️ A `$schema: "list"` dictionary's entries in index order (`"0"`, `"1"`, …) — the wire form
/// flow dictionary lists actually take (an object with numeric-string keys, not a JSON array), so
/// ordering has to be recovered by parsing the keys rather than trusting map iteration.
fn preview_channel_list_entries(map: &semio_framework_pack_json::Object) -> Vec<(usize, &semio_framework_pack_json::Value)> {
    let mut entries: Vec<_> = map.iter()
        .filter(|(key, _)| key.as_bytes().iter().all(u8::is_ascii_digit) && (key.len() == 1 || !key.starts_with('0')))
        .filter_map(|(key, value)| key.parse::<usize>().ok().map(|index| (index, value)))
        .collect();
    entries.sort_by_key(|(index, _)| *index);
    entries
}

fn preview_schema(value: &semio_framework_pack_json::Value) -> Option<&str> {
    value.get("$schema").and_then(semio_framework_pack_json::Value::as_str)
}

/// 🧮️ Reads finite coordinates for an admitted point, vector or plane member.
fn preview_xyz(value: &semio_framework_pack_json::Value) -> Option<[f64; 3]> {
    let axis = |key| value.get(key).and_then(semio_framework_pack_json::Value::as_f64).filter(|value| value.is_finite());
    Some([axis("x")?, axis("y")?, axis("z")?])
}

enum PreviewSpatialLeaf<'eval> {
    Native(&'eval str),
    Inline(PreviewInlineGeometry<'eval>),
    Invalid,
}

/// 📐️ Identifies typed geometric leaves before a walk enters their fields.
fn preview_spatial_leaf(value: &semio_framework_pack_json::Value) -> Option<PreviewSpatialLeaf<'_>> {
    use semio_framework_pack_json::Value;

    let schema = preview_schema(value)?;
    let inline = match schema {
        "geometry" | "vertex" | "edge" | "face" | "shell" => {
            let handle = value.get("handle").and_then(Value::as_str).filter(|handle| is_brep_geometry_handle(handle))?;
            return Some(PreviewSpatialLeaf::Native(handle));
        }
        "point" | "vector" => {
            let [x, y, z] = preview_xyz(value)?;
            if schema == "point" { PreviewInlineGeometry::Point { x, y, z } } else { PreviewInlineGeometry::Vector { x, y, z } }
        }
        "plane" => {
            let origin = value.get("origin").and_then(preview_xyz);
            let normal = value.get("normal").and_then(preview_xyz);
            let (Some(origin), Some(normal)) = (origin, normal) else { return Some(PreviewSpatialLeaf::Invalid); };
            PreviewInlineGeometry::Plane { origin, normal }
        }
        "mesh" => {
            let preview = value.get("preview").and_then(Value::as_str);
            let data = value.get("data").and_then(Value::as_str);
            let (Some(preview), Some(data)) = (preview, data) else { return Some(PreviewSpatialLeaf::Invalid); };
            PreviewInlineGeometry::Mesh { preview, data }
        }
        _ => return None,
    };
    Some(PreviewSpatialLeaf::Inline(inline))
}

/// 🕳️ Emits one preview item per typed spatial leaf while descending through structured values.
fn collect_preview_channel_items<'eval>(source: (&str, &'static str), value: &'eval semio_framework_pack_json::Value, path: &mut Vec<PreviewValueStep>, index: &mut usize, items: &mut Vec<PreviewChannelItem<'eval>>) {
    let (channel, channel_side) = source;
    match value {
        semio_framework_pack_json::Value::Object(map) => {
            if let Some(leaf) = preview_spatial_leaf(value) {
                let (handle, inline) = match leaf {
                    PreviewSpatialLeaf::Native(handle) => (handle.into(), None),
                    PreviewSpatialLeaf::Inline(inline) => (String::new(), Some(inline)),
                    PreviewSpatialLeaf::Invalid => return,
                };
                items.push(PreviewChannelItem { channel: channel.into(), index: *index, handle, inline, channel_side, value_path: path.clone() });
                *index += 1;
                return;
            }
            if preview_schema(value) == Some("list") {
                for (slot, entry) in preview_channel_list_entries(map) {
                    path.push(PreviewValueStep::Index(slot));
                    collect_preview_channel_items(source, entry, path, index, items);
                    path.pop();
                }
                return;
            }
            let mut fields: Vec<_> = map.iter().filter(|(_, field)| field.as_array().is_some() || field.as_object().is_some()).collect();
            fields.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (name, field) in fields {
                path.push(PreviewValueStep::Field(name.to_owned()));
                collect_preview_channel_items(source, field, path, index, items);
                path.pop();
            }
        }
        semio_framework_pack_json::Value::Array(list) => {
            for (slot, entry) in list.iter().enumerate() {
                path.push(PreviewValueStep::Index(slot));
                collect_preview_channel_items(source, entry, path, index, items);
                path.pop();
            }
        }
        _ => {}
    }
}

/// 🪜️ Reads the evaluated output, or the input selection of a preview sink.
fn preview_channels<'a>(eval: &'a semio_framework_pack_json::Value, widget: &str) -> Option<(&'static str, &'a semio_framework_pack_json::Object)> {
    let widget = eval.get(widget)?;
    let channels = |key| widget.get(key).and_then(semio_framework_pack_json::Value::as_object);
    channels("out").filter(|ports| ports.iter().next().is_some()).map(|ports| ("out", ports))
        .or_else(|| channels("in").map(|ports| ("in", ports)))
}

/// 🗺️ Follows the exact evaluated location of one geometry leaf across previews.
fn preview_leaf_at<'a>(mut value: &'a semio_framework_pack_json::Value, path: &[PreviewValueStep]) -> Option<&'a semio_framework_pack_json::Value> {
    for step in path {
        if preview_spatial_leaf(value).is_some() { return None; }
        let keyed_list = preview_schema(value) == Some("list");
        value = match step {
            PreviewValueStep::Index(index) if keyed_list => value.get(&index.to_string()),
            PreviewValueStep::Index(index) => value.as_array().and_then(|values| values.get(*index)),
            PreviewValueStep::Field(name) if !keyed_list => value.get(name),
            PreviewValueStep::Field(_) => None,
        }?;
    }
    Some(value)
}

fn preview_history_handle<'a>(eval: &'a semio_framework_pack_json::Value, widget: &str, item: &PreviewChannelItem<'_>) -> Option<&'a str> {
    let value = eval.get(widget)?.get(item.channel_side)?.get(&item.channel)?;
    match preview_spatial_leaf(preview_leaf_at(value, &item.value_path)?)? {
        PreviewSpatialLeaf::Native(handle) => Some(handle),
        _ => None,
    }
}

/// 📺️ Channel-by-channel enumeration of one widget's preview-bearing values: sorted `"out"` channel
/// keys (falling back to `"in"` for a sink with no output channels), each walked depth-first
/// into its geometry-bearing leaves.
pub fn preview_channel_items_for_widget<'eval>(eval: &'eval semio_framework_pack_json::Value, widget_id: &str) -> Vec<PreviewChannelItem<'eval>> {
    let Some((side, map)) = preview_channels(eval, widget_id) else {
        return Vec::new();
    };
    let mut items = Vec::new();
    collect_preview_channel_side(side, map, &mut items);
    items
}

fn collect_preview_channel_side<'a>(side: &'static str, map: &'a semio_framework_pack_json::Object, items: &mut Vec<PreviewChannelItem<'a>>) {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|(left, _), (right, _)| left.cmp(right));
    let mut path = Vec::new();
    for (channel, value) in entries {
        let mut index = 0;
        collect_preview_channel_items((channel, side), value, &mut path, &mut index, items);
    }
}

/// 🖼️ Borrows every spatial channel carried by a preview-enabled widget, in output then input order.
pub fn preview_viewport_items_for_widget<'a>(eval: &'a semio_framework_pack_json::Value, widget: &str) -> Vec<PreviewChannelItem<'a>> {
    let mut items = Vec::new();
    if let Some(value) = eval.get(widget) {
        for side in ["out", "in"] {
            if let Some(map) = value.get(side).and_then(semio_framework_pack_json::Value::as_object) {
                collect_preview_channel_side(side, map, &mut items);
            }
        }
    }
    items
}

/// 📚️ The exact directional targets behind hover references carried by one viewport publication.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PreviewHoverGroups(BTreeMap<String, Vec<String>>);

impl PreviewHoverGroups {
    /// 🎯️ Resolves one viewport reference against the publication that produced it.
    pub fn targets(&self, id: &str) -> Option<&[String]> {
        self.0.get(id).map(Vec::as_slice)
    }
}

/// 🪟️ Keeps the current exact viewport publications owned by one artifact instance.
#[derive(Default)]
pub struct PreviewHoverPublications {
    windows: BTreeMap<String, PreviewHoverGroups>,
}

impl PreviewHoverPublications {
    /// 📌️ Replaces the viewport publication after assembly succeeds.
    pub fn publish(&mut self, window: &str, groups: PreviewHoverGroups) {
        if groups.0.is_empty() { self.windows.remove(window); }
        else { self.windows.insert(window.to_owned(), groups); }
    }

    /// 🧹️ Releases the publications of windows that are no longer mounted.
    pub fn retain_windows(&mut self, mut mounted: impl FnMut(&str) -> bool) {
        self.windows.retain(|window, _| mounted(window));
    }

    /// 🔐️ Resolves only references published by the window that sent the pointer action.
    pub fn targets(&self, window: &str, reference: &str) -> Option<&[String]> {
        self.windows.get(window)?.targets(reference)
    }
}

/// 🎯️ Publishes every distinct port with the same pick geometry at the preview's common identity pose.
pub fn attach_preview_hover_targets(
    instances: &mut [semio_framework_pack_json::Value],
    granularity: &str,
    aliases: &std::collections::HashMap<String, String>,
    revision: u64,
) -> PreviewHoverGroups {
    use semio_framework_pack_json::{Object, Value, to_string};
    use std::collections::{BTreeMap, BTreeSet};

    let mut ports_by_mesh = BTreeMap::<&str, BTreeSet<&str>>::new();
    for instance in instances.iter() {
        if let (Some(mesh), Some(port)) = (
            instance.get("meshId").and_then(Value::as_str),
            instance.get("interactionId").and_then(Value::as_str),
        ) {
            let geometry = aliases.get(mesh).map_or(mesh, String::as_str);
            ports_by_mesh.entry(geometry).or_default().insert(port);
        }
    }
    let mut referenced = BTreeMap::new();
    let groups: BTreeMap<_, _> = ports_by_mesh.into_iter().enumerate().filter_map(|(index, (mesh, ports))| {
        if ports.len() < 2 || ports.iter().any(|port| port.is_empty() || port.len() > 256) { return None; }
        let target = |granularity: &str, id: String| {
            let mut target = Object::new();
            target.insert("granularity", Value::String(granularity.to_owned()));
            target.insert("id", Value::String(id));
            Value::Object(target)
        };
        let targets = Value::Array(ports.iter().map(|port| target(granularity, (*port).to_owned())).collect());
        if to_string(&targets).len() <= 4096 { return Some((mesh.to_owned(), targets)); }
        let id = format!("preview-hover:{revision}:{index}");
        referenced.insert(id.clone(), ports.into_iter().map(str::to_owned).collect());
        Some((mesh.to_owned(), Value::Array(vec![target(semio_plugin::INTERACTION_HOVER_GROUP_GRANULARITY, id)])))
    }).collect();
    if groups.is_empty() { return PreviewHoverGroups::default(); }
    for instance in instances {
        let Some(mesh) = instance.get("meshId").and_then(Value::as_str) else { continue; };
        let geometry = aliases.get(mesh).map_or(mesh, String::as_str);
        let Some(targets) = groups.get(geometry) else { continue; };
        if let Some(object) = instance.as_object_mut() { object.insert("hoverTargets", targets.clone()); }
    }
    PreviewHoverGroups(referenced)
}

/// 🪪️ Preview-enabled widgets and typed channel relays in declaration order.
pub fn preview_widget_ids(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> Vec<String> {
    use semio_framework_artifact_flow_flow::Widget;
    use semio_s_artifact_procedural_3d_model::{widget_id, widget_previews};
    host_snapshot.widgets.iter().filter(|widget| {
        widget_previews(widget) || matches!(widget, Widget::Variable { .. })
    }).map(|widget| widget_id(widget).to_owned()).collect()
}

pub fn mesh_has_preview_geometry(data: &MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

/// 🏷️ Every role a published preview mesh may declare, in the order a reader ranks them: a body
/// with triangles, a body that is only a polyline, the axis cross of a point channel, the arrow of
/// a vector channel.
pub const PREVIEW_MESH_ROLES: [&str; 4] = ["solid", "wire", "point", "vector"];

/// 📢️ What ONE published preview mesh is, stamped into the payload beside its geometry so a reader
/// never has to re-derive it from the arrays.
///
/// ⚖️ Geometry on either side of a preview-enabled node gets a world instance. Instances sharing
/// a retained source handle use the same mesh buffer, and the role describes that buffer.
///
/// 🎨️ The role is what the payload CARRIES, not what the channel was named: `apply_show_mode_mesh`
/// strips triangles in `wireframe` mode, and a solid published without them really is a wire on
/// screen. The inline markers keep their own role because their arrays are indistinguishable from a
/// tessellated polyline.
pub fn preview_mesh_role(inline: Option<&PreviewInlineGeometry<'_>>, data: &MeshData) -> &'static str {
    match inline {
        Some(PreviewInlineGeometry::Point { .. }) => "point",
        Some(PreviewInlineGeometry::Vector { .. }) => "vector",
        Some(PreviewInlineGeometry::Plane { .. }) => "wire",
        _ if !data.indices.is_empty() => "solid",
        _ if !data.edge_positions.is_empty() => "wire",
        _ => "point",
    }
}

/// 📦️ The finite world-space limits of the visible buffer lanes.
pub type PreviewBounds = ([f64; 3], [f64; 3]);

/// 📏️ Accumulates fit bounds directly from the same vertex and edge buffers the viewport receives.
pub struct PreviewMeshBounds {
    minimum: [f32; 3],
    maximum: [f32; 3],
}

impl Default for PreviewMeshBounds {
    fn default() -> Self {
        Self { minimum: [f32::INFINITY; 3], maximum: [f32::NEG_INFINITY; 3] }
    }
}

impl PreviewMeshBounds {
    pub fn include(&mut self, mesh: &MeshData) {
        for point in mesh.positions.chunks_exact(3).chain(mesh.edge_positions.chunks_exact(3)) {
            if point.iter().all(|value| value.is_finite()) {
                for (axis, value) in point.iter().copied().enumerate() {
                    self.minimum[axis] = self.minimum[axis].min(value);
                    self.maximum[axis] = self.maximum[axis].max(value);
                }
            }
        }
    }

    pub fn finish(self) -> Option<PreviewBounds> {
        self.minimum[0].is_finite().then(|| (self.minimum.map(f64::from), self.maximum.map(f64::from)))
    }
}

/// 🎯️ Headroom the preview leaves around the delivered bounds — the render hosts' own margin rides
/// on top of the exact fitting distance, so this padding only has to say "do not hug the box".
pub const PREVIEW_FIT_PADDING: f64 = 1.12;

/// 📄️ Which DOCUMENT the preview is framing, as a number the render hosts key their one-shot fit on.
///
/// ⚖️ Hashed over the graph's TOPOLOGY (widget ids, their kinds, the synapse wiring) and never over
/// parameter values: switching example replaces the whole graph, while dragging a slider keeps it,
/// so this is exactly the line between "frame the example the user just opened" and "yank the camera
/// out from under an edit". Every re-evaluation of one example answers the same revision, which is
/// what makes the hosts' framing one-shot at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
pub fn preview_fit_revision(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> u32 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    let mut eat = |text: &str| {
        for byte in text.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    };
    eat(&host_snapshot.schema);
    for widget in &host_snapshot.widgets {
        eat(semio_s_artifact_procedural_3d_model::widget_id(widget));
        eat(widget_kind_tag(widget));
    }
    for synapse in &host_snapshot.synapses {
        eat(&synapse.from);
        eat(&synapse.from_port);
        eat(&synapse.to);
        eat(&synapse.to_port);
    }
    (hash >> 32) as u32
}

/// 🔑️ The kind tag [`preview_fit_revision`] hashes per widget — what the node IS, never what it is
/// currently set to.
fn widget_kind_tag(widget: &semio_framework_artifact_flow_flow::Widget) -> &str {
    use semio_framework_artifact_flow_flow::Widget;
    match widget {
        Widget::Neuron { neuron_kind, .. } => neuron_kind.as_str(),
        Widget::InputSlider { .. } => "input.slider",
        Widget::OutputPreview { .. } => "output.preview",
        _ => "widget",
    }
}

/// 🌍️ The `World3dScene.fit_json` every generation3d preview window publishes: frame the delivered
/// bounds once per document, never again while that document is on screen.
pub fn preview_fit_json(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, bounds: Option<PreviewBounds>) -> String {
    semio_ui_declaration::ui::world3d_fit_json(preview_fit_revision(host_snapshot), PREVIEW_FIT_PADDING, bounds)
}

/// ✏️ Half-extent (world units) of the axis cross drawn for a [`PreviewInlineGeometry::Point`].
const PREVIEW_POINT_MARKER_HALF_EXTENT: f64 = 0.05;

/// 🌀️ A small axis cross at `(x, y, z)` — the point-channel preview marker, built without a kernel
/// round-trip when the coordinates fit in the renderer's vertex format. Carries both `positions`
/// (so `"points"` show mode still has something to draw once
/// [`apply_show_mode_mesh`] strips `edge_positions`) and the cross itself as edges.
pub fn point_marker_mesh(x: f64, y: f64, z: f64) -> MeshData {
    let [x, y, z] = [x, y, z].map(|value| value as f32);
    if ![x, y, z].into_iter().all(f32::is_finite) { return MeshData::default(); }
    let e = PREVIEW_POINT_MARKER_HALF_EXTENT as f32;
    MeshData { positions: vec![x, y, z], edge_positions: vec![x - e, y, z, x + e, y, z, x, y - e, z, x, y + e, z, x, y, z - e, x, y, z + e], ..Default::default() }
}

/// 🧭️ Returns orthonormal tangent axes for a unit direction.
fn marker_tangent_basis(direction: [f64; 3]) -> [[f64; 3]; 2] {
    let [a, b, c] = direction;
    let side = if a.abs() > c.abs() { [-b, a, 0.0] } else { [0.0, -c, b] };
    let norm = side[0].hypot(side[1]).hypot(side[2]);
    let side = side.map(|v| v / norm);
    let [i, j, k] = side;
    let up = [b * k - c * j, c * i - a * k, a * j - b * i];
    [side, up]
}

/// ➡️ Draws a shaft and four fins that meet at the spatial vector's tip, or a cross at the origin
/// when the tip rounds to zero in the renderer's vertex format.
pub fn vector_marker_mesh(x: f64, y: f64, z: f64) -> MeshData {
    let tip = [x, y, z];
    let rendered_tip = tip.map(|value| value as f32);
    if !rendered_tip.into_iter().all(f32::is_finite) { return MeshData::default(); }
    if rendered_tip == [0.0; 3] { return point_marker_mesh(0.0, 0.0, 0.0); }
    let length = x.hypot(y).hypot(z);
    let direction = tip.map(|v| v / length);
    let [side, up] = marker_tangent_basis(direction);
    let mut positions = vec![0.0, 0.0, 0.0];
    positions.extend(rendered_tip);
    let mut edge_positions = positions.clone();
    for axis in [side, up] {
        for sign in [-1.0, 1.0] {
            let base: [f32; 3] = std::array::from_fn(|i| (0.8 * tip[i] + sign * 0.07 * length * axis[i]) as f32);
            edge_positions.extend(rendered_tip);
            edge_positions.extend(base);
            positions.extend(base);
        }
    }
    MeshData { positions, edge_positions, ..Default::default() }
}

/// 📐️ Draws a finite outline and normal for a spatial plane channel.
fn plane_marker_mesh(origin: [f64; 3], normal: [f64; 3]) -> Option<MeshData> {
    if origin.iter().any(|value| !(*value as f32).is_finite()) || normal.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let component = normal.iter().map(|value| value.abs()).fold(0.0_f64, f64::max);
    if component == 0.0 { return None; }
    let scaled = normal.map(|value| value / component);
    let length = scaled[0].hypot(scaled[1]).hypot(scaled[2]);
    let direction = scaled.map(|value| value / length);
    let [side, up] = marker_tangent_basis(direction);
    let corners: [[f32; 3]; 4] = [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)].map(|(u, v)|
        std::array::from_fn(|axis| (origin[axis] + u * side[axis] + v * up[axis]) as f32));
    let [nx, ny, nz] = direction.map(|value| 0.35 * value);
    let normal = vector_marker_mesh(nx, ny, nz);
    let mut positions = Vec::with_capacity(12 + normal.positions.len());
    let mut edge_positions = Vec::with_capacity(24 + normal.edge_positions.len());
    for (corner, next) in corners.iter().zip(corners.iter().cycle().skip(1)) {
        positions.extend(corner);
        edge_positions.extend(corner);
        edge_positions.extend(next);
    }
    let shift = |(i, coordinate): (usize, f32)| (f64::from(coordinate) + origin[i % 3]) as f32;
    positions.extend(normal.positions.into_iter().enumerate().map(shift));
    edge_positions.extend(normal.edge_positions.into_iter().enumerate().map(shift));
    Some(MeshData { positions, edge_positions, ..Default::default() })
}

/// 🧷️ Produces the viewport representation of a structured spatial value.
pub fn inline_preview_mesh(inline: &PreviewInlineGeometry<'_>) -> Option<MeshData> {
    match inline {
        PreviewInlineGeometry::Point { x, y, z } => Some(point_marker_mesh(*x, *y, *z)),
        PreviewInlineGeometry::Vector { x, y, z } => Some(vector_marker_mesh(*x, *y, *z)),
        PreviewInlineGeometry::Plane { origin, normal } => plane_marker_mesh(*origin, *normal),
        PreviewInlineGeometry::Mesh { preview, .. } => decode_preview_mesh_pack(preview),
    }
}

/// 👁️ The shading mode decides which mesh channels survive into the payload at all, so
/// wireframe/points really are cheaper.
pub fn apply_show_mode_mesh(mut data: MeshData, show_mode: &str) -> MeshData {
    let show_mode = match show_mode {
        "solid" | "shaded" | "shaded+edges" | "wireframe" | "points" => show_mode,
        _ => "shaded",
    };
    if matches!(show_mode, "wireframe" | "shaded+edges") && data.edge_positions.is_empty() {
        let mut edges = std::collections::BTreeSet::new();
        for triangle in data.indices.chunks_exact(3) {
            for (a, b) in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])] {
                if !edges.insert((a.min(b), a.max(b))) { continue; }
                let point = |index: u32| (index as usize).checked_mul(3).and_then(|start| start.checked_add(3).and_then(|end| data.positions.get(start..end)));
                if let (Some(a), Some(b)) = (point(a), point(b)) {
                    data.edge_positions.extend_from_slice(a);
                    data.edge_positions.extend_from_slice(b);
                }
            }
        }
    }
    if !matches!(show_mode, "wireframe" | "points") { return data; }
    let points_only = data.edge_positions.is_empty() && data.indices.is_empty();
    let wireframe = show_mode == "wireframe" && !points_only;
    if wireframe {
        data.component_references.retain_domains(|kind| kind == "edge");
        return MeshData {
            edge_positions: data.edge_positions, edge_ids: data.edge_ids,
            edge_uvs: data.edge_uvs, edge_is_seam: data.edge_is_seam,
            component_references: data.component_references, ..Default::default()
        };
    }
    if data.positions.is_empty() { data.positions = data.edge_positions; }
    data.component_references.retain_domains(|kind| kind == "vertex");
    MeshData {
        positions: data.positions, vertex_ids: data.vertex_ids,
        component_references: data.component_references, ..Default::default()
    }
}

/// 🎒️ Decodes a base64 `pack` mesh body the extension shipped back — the single decode seam every
/// preview reader goes through.
pub fn decode_preview_mesh_pack(base64_body: &str) -> Option<MeshData> {
    let bytes = semio_os_flow::mesh::decode_base64(base64_body).ok()?;
    semio_os_flow::mesh::decode_mesh_pack(&bytes).ok()
}

/// 🧵️ Borrows the canonical mesh completed by the retained preview chain.
pub fn session_preview_mesh<'a>(handle: &str, session: &'a FlowEvalSession) -> Option<&'a MeshData> {
    session.preview_mesh(handle).filter(|mesh| mesh_has_preview_geometry(mesh))
}

/// 🧭️ Resolves resident geometry by its source handle and its evaluated channel.
pub struct PreviewMeshLookup<'a> {
    session: &'a FlowEvalSession,
    source: PreviewEvaluationSource,
    converged: [std::cell::OnceCell<Option<semio_framework_pack_json::Value>>; 2],
}

impl<'a> PreviewMeshLookup<'a> {
    pub fn new(session: &'a FlowEvalSession) -> Self {
        Self::for_source(session, PreviewEvaluationSource::WithFallback)
    }

    pub(super) fn for_source(session: &'a FlowEvalSession, source: PreviewEvaluationSource) -> Self {
        Self { session, source, converged: Default::default() }
    }

    pub fn resolve<'item>(&self, widget: &str, item: &'item PreviewChannelItem<'_>) -> Option<(std::borrow::Cow<'item, str>, &'a MeshData)> {
        if item.handle.is_empty() { return None; }
        if let Some(mesh) = session_preview_mesh(&item.handle, self.session) {
            return Some((std::borrow::Cow::Borrowed(&item.handle), mesh));
        }
        if self.source == PreviewEvaluationSource::Current || self.session.preview_diagnostics(&item.handle).is_some() { return None; }
        let history = [self.session.converged_eval_json(), self.session.previous_converged_eval_json()];
        for (slot, text) in self.converged.iter().zip(history) {
            let Some(eval) = slot.get_or_init(|| {
                semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()
            }).as_ref() else { continue };
            if let Some(handle) = preview_history_handle(eval, widget, item) {
                if let Some(mesh) = session_preview_mesh(handle, self.session) {
                    return Some((std::borrow::Cow::Owned(handle.to_owned()), mesh));
                }
            }
        }
        None
    }
}

fn visit_retained_preview_items<'a>(
    eval: &'a semio_framework_pack_json::Value,
    host: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    mut visit: impl FnMut(&str, PreviewChannelItem<'a>),
) {
    let widgets = preview_widget_ids(host);
    let visible: HashSet<_> = widgets.iter().map(String::as_str).collect();
    for id in &widgets {
        for item in preview_viewport_items_for_widget(eval, id) {
            visit(id, item);
        }
    }
    for (id, channel) in semio_s_artifact_procedural_3d_mesh_bridge::retained_geometry_channels(host) {
        if visible.contains(id.as_str()) { continue; }
        for item in preview_channel_items_for_widget(eval, &id).into_iter().filter(|item| channel.as_ref().is_none_or(|channel| *channel == item.channel)) {
            visit(&id, item);
        }
    }
}

/// 🖌️ What the preview would paint from `eval` given the session's current mesh packs — changes when
/// `flowTessellateResolve` lands even though the evaluation text did not move one byte.
fn preview_mesh_residency_digest(session: &FlowEvalSession, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, eval: &semio_framework_pack_json::Value, source: PreviewEvaluationSource) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    session.preview_mesh_epoch().hash(&mut hasher);
    let lookup = PreviewMeshLookup::for_source(session, source);
    visit_retained_preview_items(eval, host_snapshot, |id, item| {
        if !item.handle.is_empty() {
            lookup.resolve(id, &item).map(|(handle, _)| handle).hash(&mut hasher);
        }
    });
    hasher.finish()
}

/// 📤️ Whether the addressed preview window owes a fresh publication. The painted evaluation text is
/// the primary signal; mesh residency is the second — tessellation can finish without moving eval
/// json, and a signature that looked only at bytes pinned the first (empty or stale) table forever
/// (`📓️slider-mesh-supersession-fallback-2026-09-15.md`, viewer `preview_mesh_signature`).
pub fn preview_eval_publication_for(session: &mut FlowEvalSession, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, retained_eval: Option<&str>, source: PreviewEvaluationSource) -> FlowEvalPublication {
    let painted = source.text(session);
    let digest = if painted.is_empty() {
        0
    } else {
        semio_framework_pack_json::parse(painted, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|eval| preview_mesh_residency_digest(session, host_snapshot, &eval, source)).unwrap_or(0)
    };
    let evaluated = (!painted.is_empty()).then_some(painted);
    let publication = match source {
        PreviewEvaluationSource::WithFallback => session.eval_publication_for(retained_eval),
        PreviewEvaluationSource::Current if evaluated == retained_eval => FlowEvalPublication::Retained,
        PreviewEvaluationSource::Current => FlowEvalPublication::Changed(evaluated.map(str::to_owned)),
    };
    if matches!(publication, FlowEvalPublication::Retained) {
        if session.published_preview_mesh_digest() == digest {
            return FlowEvalPublication::Retained;
        }
        let snapshot = evaluated.map(str::to_owned);
        session.note_published_preview_mesh_digest(digest);
        return FlowEvalPublication::Changed(snapshot);
    }
    session.note_published_preview_mesh_digest(digest);
    publication
}

/// 🧹 Every brep handle whose tessellation pack must stay live across a handle supersession.
pub fn preview_mesh_retention_handles(eval: &semio_framework_pack_json::Value, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession) -> HashSet<String> {
    let lookup = PreviewMeshLookup::new(session);
    let mut live: HashSet<String> = session.in_flight_preview_handles().map(str::to_owned).collect();
    for widget in &host_snapshot.widgets {
        let id = semio_s_artifact_procedural_3d_model::widget_id(widget);
        for item in preview_viewport_items_for_widget(eval, id) {
            if item.handle.is_empty() { continue; }
            if let Some((source, _)) = lookup.resolve(id, &item) {
                if source != item.handle.as_str() { live.insert(source.into_owned()); }
            }
            live.insert(item.handle);
        }
    }
    live
}

/// 🤲️ Geometry handles on preview widgets that still need an extension tessellate. Takes the ALREADY
/// PARSED evaluation: its caller parses the same document one line earlier to collect the live
/// handle set, and re-parsing a whole eval session per tick is the cost this path exists to avoid.
pub fn pending_preview_tessellate_handles(eval: &semio_framework_pack_json::Value, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession, tolerance: f64) -> Vec<String> {
    let mut handles = Vec::new();
    visit_retained_preview_items(eval, host_snapshot, |_, item| {
        let handle = item.handle;
        if !handle.is_empty()
            && session.preview_diagnostics(&handle).is_none()
            && session.preview_mesh_for_tolerance(&handle, tolerance).is_none()
            && !session.preview_tessellate_failed(semio_os_flow::preview_tessellate_node_hash(&handle,tolerance.to_bits()))
        {
            handles.push(handle);
        }
    });
    handles
}

/// 📨 Extension invocations that tessellate preview handles inside the owning brep extension kernel.
/// Each one names `flowTessellateResolve` as its response action, so the SDK's request registry
/// mints the `req`, parks the continuation and dispatches the mesh JSON straight back into the
/// addressed surface — a hand-minted `RequestId` owns no registry slot, so every tessellation result
/// would be discarded and the 3d preview could never paint.
pub fn preview_tessellate_invocations(window_id: &str, window_kind_id: &str, session: &mut FlowEvalSession, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, tolerance: f64, source: PreviewEvaluationSource, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<Vec<ExtensionInvocation>, semio_framework_value::ValueError> {
    let Ok(geometry_extension_address) = geometry_extension_address() else {
        return Ok(Vec::new());
    };
    session.note_preview_tessellate_tolerance(tolerance);
    let tolerance_bits = tolerance.to_bits();
    let (live, pending) = {
        let eval_json = source.text(session);
        if eval_json.is_empty() {
            return Ok(Vec::new());
        }
        let eval = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
        let live = preview_mesh_retention_handles(&eval, host_snapshot, session);
        (live, pending_preview_tessellate_handles(&eval, host_snapshot, session, tolerance))
    };
    retain_preview_meshes(session, live, effect)?;
    let mut invocations = Vec::new();
    for handle in pending {
        let node_hash = semio_os_flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone(), tolerance) {
            let mut request_object = semio_framework_pack_json::Object::new();
            request_object.insert("handle", semio_framework_pack_json::Value::String(handle));
            request_object.insert("tolerance", semio_framework_pack_json::Value::from(tolerance));
            request_object.insert("nodeHash", semio_framework_pack_json::Value::from(node_hash));
            request_object.insert("budget", semio_framework_pack_json::Value::from(u64::from(PREVIEW_TESSELLATE_STEP_BUDGET)));
            request_object.insert("wallMicros", semio_framework_pack_json::Value::from(PREVIEW_TESSELLATE_STEP_WALL_MICROS));
            request_object.insert("windowId", semio_framework_pack_json::Value::String(window_id.to_string()));
            request_object.insert("windowKindId", semio_framework_pack_json::Value::String(window_kind_id.to_string()));
            let request_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(request_object));
            invocations.push(ExtensionInvocation::new(geometry_extension_address.clone(), "tessellate", request_json, "flowTessellateResolve"));
        }
    }
    Ok(invocations)
}

/// 💚️ Retains exactly the live roster of preview meshes, one step per loop inside the caller's original effect, and refuses a retention that stops making progress.
pub fn retain_preview_meshes(session: &mut FlowEvalSession, live: HashSet<String>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<(), semio_framework_value::ValueError> {
    session.begin_retain_preview_meshes(live.into_iter().collect()).map_err(|(error, _)| error)?;
    while !session.preview_retention_terminal_is_empty() {
        session.retain_preview_meshes_step(effect)?;
        if session.preview_retention_step_progress() == Default::default() && !session.preview_retention_terminal_is_empty() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "preview mesh retention was granted too little to advance"));
        }
    }
    Ok(())
}
//#endregion 🧊️Geometry
