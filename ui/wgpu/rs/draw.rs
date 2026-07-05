//! 🖌️ Draw list and GPU pipeline for UI quads, vector geometry, and 3D scene passes.

use crate::scene3d::ScenePass3d;
use crate::shaders::{UI_SHADER, VECTOR_SHADER, WORLD3D_SHADER};
use crate::theme::Rgba;
use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::util::DeviceExt;

pub const KIND_SOLID: f32 = 3.0;
pub const KIND_ROUNDED: f32 = 1.0;
pub const KIND_GLYPH: f32 = 2.0;
pub const KIND_TEXTURED: f32 = 4.0;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiGlobals {
    pub screen_size: [f32; 2],
    pub _pad: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub params: [f32; 4],
    pub uv_rect: [f32; 4],
}

impl UiInstance {
    pub fn solid(rect: [f32; 4], color: Rgba) -> Self {
        Self {
            rect,
            color: [color.r, color.g, color.b, color.a],
            params: [0.0, 0.0, KIND_SOLID, 0.0],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        }
    }

    pub fn rounded(rect: [f32; 4], color: Rgba, radius: f32, border: f32, border_color: Rgba) -> Self {
        Self {
            rect,
            color: [color.r, color.g, color.b, color.a],
            params: [radius, border, KIND_ROUNDED, border_color.a],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
        }
    }

    pub fn glyph(rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) -> Self {
        Self {
            rect,
            color: [color.r, color.g, color.b, color.a],
            params: [0.0, 0.0, KIND_GLYPH, 0.0],
            uv_rect,
        }
    }

    pub fn textured(rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) -> Self {
        Self {
            rect,
            color: [1.0, 1.0, 1.0, alpha],
            params: [0.0, 0.0, KIND_TEXTURED, 0.0],
            uv_rect,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct VectorVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl ScissorRect {
    pub fn from_rect(rect: crate::geometry::Rect, _screen_h: f32) -> Self {
        let x = rect.x.max(0.0) as u32;
        let y = rect.y.max(0.0) as u32;
        let w = rect.w.max(0.0) as u32;
        let h = rect.h.max(0.0) as u32;
        Self { x, y, w, h }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let x0 = self.x.max(other.x);
        let y0 = self.y.max(other.y);
        let x1 = (self.x + self.w).min(other.x + other.w);
        let y1 = (self.y + self.h).min(other.y + other.h);
        Self {
            x: x0,
            y: y0,
            w: x1.saturating_sub(x0),
            h: y1.saturating_sub(y0),
        }
    }
}

pub struct DrawLayer {
    pub scissor: Option<ScissorRect>,
    pub ui_instances: Vec<UiInstance>,
    pub vector_vertices: Vec<VectorVertex>,
}

impl Default for DrawLayer {
    fn default() -> Self {
        Self {
            scissor: None,
            ui_instances: Vec::new(),
            vector_vertices: Vec::new(),
        }
    }
}

pub struct DrawList {
    pub scene_passes: Vec<ScenePass3d>,
    pub layers: Vec<DrawLayer>,
    scissor_stack: Vec<ScissorRect>,
    screen_h: f32,
}

impl Default for DrawList {
    fn default() -> Self {
        let mut list = Self {
            scene_passes: Vec::new(),
            layers: Vec::new(),
            scissor_stack: Vec::new(),
            screen_h: 720.0,
        };
        list.layers.push(DrawLayer::default());
        list
    }
}

impl DrawList {
    pub fn set_screen_height(&mut self, height: f32) {
        self.screen_h = height;
    }

    fn active_layer(&mut self) -> &mut DrawLayer {
        if self.layers.is_empty() {
            self.layers.push(DrawLayer::default());
        }
        self.layers.last_mut().expect("layer")
    }

    pub fn clear(&mut self) {
        self.scene_passes.clear();
        self.layers.clear();
        self.layers.push(DrawLayer::default());
        self.scissor_stack.clear();
    }

    pub fn push_scissor(&mut self, rect: crate::geometry::Rect) {
        let mut scissor = ScissorRect::from_rect(rect, self.screen_h);
        if let Some(parent) = self.scissor_stack.last() {
            scissor = parent.intersect(&scissor);
        }
        self.scissor_stack.push(scissor);
        self.layers.push(DrawLayer {
            scissor: Some(scissor),
            ui_instances: Vec::new(),
            vector_vertices: Vec::new(),
        });
    }

    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
        let parent = self.scissor_stack.last().cloned();
        self.layers.push(DrawLayer {
            scissor: parent,
            ui_instances: Vec::new(),
            vector_vertices: Vec::new(),
        });
    }

    pub fn push_scene_pass(&mut self, pass: ScenePass3d) {
        self.scene_passes.push(pass);
    }

    pub fn push_solid(&mut self, rect: [f32; 4], color: Rgba) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::solid(rect, color));
    }

    pub fn push_rounded(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::rounded(rect, color, radius, 0.0, color));
    }

    pub fn push_glyph(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
        self.active_layer()
            .ui_instances
            .push(UiInstance::textured(rect, uv_rect, alpha));
    }

    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let nx = -dy / len * width * 0.5;
        let ny = dx / len * width * 0.5;
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        layer.vector_vertices.extend_from_slice(&[
            VectorVertex { position: [x0 + nx, y0 + ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            VectorVertex { position: [x1 + nx, y1 + ny], color: c },
            VectorVertex { position: [x1 - nx, y1 - ny], color: c },
            VectorVertex { position: [x0 - nx, y0 - ny], color: c },
        ]);
    }

    pub fn push_triangle_fan(&mut self, points: &[[f32; 2]], color: Rgba) {
        if points.len() < 3 {
            return;
        }
        let c = [color.r, color.g, color.b, color.a];
        let layer = self.active_layer();
        for tri in 1..points.len() - 1 {
            layer.vector_vertices.push(VectorVertex { position: points[0], color: c });
            layer.vector_vertices
                .push(VectorVertex { position: points[tri], color: c });
            layer.vector_vertices
                .push(VectorVertex { position: points[tri + 1], color: c });
        }
    }
}

pub fn ear_clip_polygon(points: &[[f32; 2]]) -> Vec<[f32; 2]> {
    if points.len() < 3 {
        return Vec::new();
    }
    let mut indices: Vec<usize> = (0..points.len()).collect();
    let mut triangles = Vec::new();
    let mut guard = 0usize;
    while indices.len() > 3 && guard < points.len() * points.len() {
        guard += 1;
        let mut ear_found = false;
        for i in 0..indices.len() {
            let prev = indices[(i + indices.len() - 1) % indices.len()];
            let curr = indices[i];
            let next = indices[(i + 1) % indices.len()];
            let a = points[prev];
            let b = points[curr];
            let c = points[next];
            let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross <= 0.0 {
                continue;
            }
            let mut contains = false;
            for &idx in &indices {
                if idx == prev || idx == curr || idx == next {
                    continue;
                }
                let p = points[idx];
                if point_in_triangle(p, a, b, c) {
                    contains = true;
                    break;
                }
            }
            if contains {
                continue;
            }
            triangles.push(a);
            triangles.push(b);
            triangles.push(c);
            indices.remove(i);
            ear_found = true;
            break;
        }
        if !ear_found {
            break;
        }
    }
    if indices.len() == 3 {
        triangles.push(points[indices[0]]);
        triangles.push(points[indices[1]]);
        triangles.push(points[indices[2]]);
    }
    triangles
}

fn point_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGlobals {
    pub view_proj: [f32; 16],
    pub light_dir: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color: [f32; 4],
    pub flags: [f32; 4],
}

impl World3dGpuInstance {
    pub fn from_instance(model: [f32; 16], color: [f32; 4], selected: bool, hovered: bool) -> Self {
        Self {
            model0: [model[0], model[4], model[8], model[12]],
            model1: [model[1], model[5], model[9], model[13]],
            model2: [model[2], model[6], model[10], model[14]],
            model3: [model[3], model[7], model[11], model[15]],
            color,
            flags: [
                if selected { 1.0 } else { 0.0 },
                if hovered { 1.0 } else { 0.0 },
                0.0,
                0.0,
            ],
        }
    }
}

pub struct GpuMeshBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

pub struct MeshGpuStore {
    meshes: std::collections::HashMap<String, GpuMeshBuffers>,
}

pub fn mesh_content_version(positions: &[f32], normals: &[f32], indices: &[u32]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for value in positions.iter().chain(normals.iter()) {
        hash ^= value.to_bits() as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for value in indices {
        hash ^= *value as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

impl Default for MeshGpuStore {
    fn default() -> Self {
        Self {
            meshes: std::collections::HashMap::new(),
        }
    }
}

impl MeshGpuStore {
    pub fn get(&self, key: &str) -> Option<&GpuMeshBuffers> {
        self.meshes.get(key)
    }

    pub fn lookup_key(mesh_key: &str, version: u64) -> String {
        format!("{mesh_key}:{version}")
    }

    pub fn get_versioned(&self, mesh_key: &str, version: u64) -> Option<&GpuMeshBuffers> {
        self.get(&Self::lookup_key(mesh_key, version))
    }

    pub fn ensure_mesh(
        &mut self,
        device: &wgpu::Device,
        key: &str,
        version: u64,
        positions: &[f32],
        normals: &[f32],
        indices: &[u32],
    ) {
        let store_key = format!("{key}:{version}");
        if self.meshes.contains_key(&store_key) {
            return;
        }
        let prefix = format!("{key}:");
        self.meshes.retain(|existing, _| !existing.starts_with(&prefix) || existing == &store_key);
        let mut vertices = Vec::with_capacity(positions.len() / 3);
        for index in 0..positions.len() / 3 {
            vertices.push(World3dVertex {
                position: [
                    positions[index * 3],
                    positions[index * 3 + 1],
                    positions[index * 3 + 2],
                ],
                normal: [
                    normals.get(index * 3).copied().unwrap_or(0.0),
                    normals.get(index * 3 + 1).copied().unwrap_or(1.0),
                    normals.get(index * 3 + 2).copied().unwrap_or(0.0),
                ],
            });
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("world3d_vertices"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("world3d_indices"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.meshes.insert(
            store_key,
            GpuMeshBuffers {
                vertex_buffer,
                index_buffer,
                index_count: indices.len() as u32,
            },
        );
    }
}

pub const WORLD_GLOBALS_SLOT_SIZE: u64 = 256;

pub struct GrowBuffer {
    buffer: Option<wgpu::Buffer>,
    capacity: usize,
}

impl Default for GrowBuffer {
    fn default() -> Self {
        Self {
            buffer: None,
            capacity: 0,
        }
    }
}

impl GrowBuffer {
    pub fn slice(&self) -> Option<wgpu::BufferSlice<'_>> {
        self.buffer.as_ref().map(|buffer| buffer.slice(..))
    }

    pub fn upload<T: Pod>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[T],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> Option<wgpu::BufferSlice<'_>> {
        if data.is_empty() {
            return None;
        }
        let bytes = bytemuck::cast_slice(data);
        let required = bytes.len();
        if self.capacity < required {
            self.capacity = required.next_power_of_two().max(256);
            self.buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: self.capacity as u64,
                usage,
                mapped_at_creation: false,
            }));
        }
        let buffer = self.buffer.as_ref()?;
        queue.write_buffer(buffer, 0, bytes);
        Some(buffer.slice(..))
    }
}

pub struct FrameBuffers {
    pub world_instances: GrowBuffer,
    pub ui_instances: GrowBuffer,
    pub vector_vertices: GrowBuffer,
}

impl Default for FrameBuffers {
    fn default() -> Self {
        Self {
            world_instances: GrowBuffer::default(),
            ui_instances: GrowBuffer::default(),
            vector_vertices: GrowBuffer::default(),
        }
    }
}

struct WorldDrawRange {
    mesh_key: String,
    mesh_version: u64,
    instance_offset: u32,
    instance_count: u32,
}

struct PreparedWorldPass {
    globals: World3dGlobals,
    viewport: [f32; 4],
    draws: Vec<WorldDrawRange>,
}

struct WorldGlobalsRing {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    slot_stride: u32,
    capacity_slots: u32,
}

impl WorldGlobalsRing {
    fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, initial_slots: u32) -> Self {
        let slot_stride = WORLD_GLOBALS_SLOT_SIZE as u32;
        let capacity_slots = initial_slots.max(1);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("world3d_globals_ring"),
            size: slot_stride as u64 * capacity_slots as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(mem::size_of::<World3dGlobals>() as u64),
                }),
            }],
        });
        Self {
            buffer,
            bind_group,
            slot_stride,
            capacity_slots,
        }
    }

    fn ensure_slots(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, slots: u32) {
        if slots <= self.capacity_slots {
            return;
        }
        self.capacity_slots = slots.next_power_of_two().max(4);
        self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("world3d_globals_ring"),
            size: self.slot_stride as u64 * self.capacity_slots as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &self.buffer,
                    offset: 0,
                    size: std::num::NonZeroU64::new(mem::size_of::<World3dGlobals>() as u64),
                }),
            }],
        });
    }

    fn write_passes(&self, queue: &wgpu::Queue, passes: &[World3dGlobals]) {
        for (index, globals) in passes.iter().enumerate() {
            let offset = (index as u64) * self.slot_stride as u64;
            queue.write_buffer(&self.buffer, offset, bytemuck::bytes_of(globals));
        }
    }

    fn offset_for_slot(&self, slot: u32) -> u32 {
        slot * self.slot_stride
    }
}

fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}

pub struct IconAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    entries: std::collections::HashMap<String, [f32; 4]>,
}

impl Default for IconAtlas {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            pixels: vec![0, 0, 0, 0],
            entries: std::collections::HashMap::new(),
        }
    }
}

impl IconAtlas {
    pub fn from_packed(width: u32, height: u32, pixels: Vec<u8>, entries: Vec<(String, [f32; 4])>) -> Self {
        Self {
            width,
            height,
            pixels,
            entries: entries.into_iter().collect(),
        }
    }

    pub fn icon_uv(&self, icon_id: &str) -> Option<[f32; 4]> {
        self.entries.get(icon_id).copied()
    }
}

pub struct RasterTexture {
    pub texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

pub struct RasterTextureStore {
    textures: std::collections::HashMap<String, RasterTexture>,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

impl RasterTextureStore {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("raster_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Self {
            textures: std::collections::HashMap::new(),
            layout: layout.clone(),
            sampler,
        }
    }

    pub fn ensure_raster(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        globals_buffer: &wgpu::Buffer,
        glyph_view: &wgpu::TextureView,
        glyph_sampler: &wgpu::Sampler,
        icon_view: &wgpu::TextureView,
        icon_sampler: &wgpu::Sampler,
        key: &str,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) {
        if self.textures.contains_key(key) {
            return;
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("raster_texture"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raster_bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        self.textures.insert(
            key.to_string(),
            RasterTexture {
                texture,
                bind_group,
                width,
                height,
            },
        );
    }

    pub fn get(&self, key: &str) -> Option<&RasterTexture> {
        self.textures.get(key)
    }
}

pub(crate) struct UiPipelines {
    ui_pipeline: wgpu::RenderPipeline,
    vector_pipeline: wgpu::RenderPipeline,
    world_pipeline: wgpu::RenderPipeline,
    quad_vertex_buffer: wgpu::Buffer,
    globals_buffer: wgpu::Buffer,
    world_globals_ring: WorldGlobalsRing,
    world_bind_group_layout: wgpu::BindGroupLayout,
    glyph_texture: wgpu::Texture,
    glyph_sampler: wgpu::Sampler,
    icon_texture: wgpu::Texture,
    icon_sampler: wgpu::Sampler,
    glyph_bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

struct LayerBatch {
    scissor: Option<ScissorRect>,
    ui_start: u32,
    ui_count: u32,
    vec_start: u32,
    vec_count: u32,
}

fn build_layer_batches(draw: &DrawList) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
    let mut all_ui = Vec::new();
    let mut all_vec = Vec::new();
    let mut batches = Vec::new();
    for layer in &draw.layers {
        if layer.ui_instances.is_empty() && layer.vector_vertices.is_empty() {
            continue;
        }
        let ui_start = all_ui.len() as u32;
        all_ui.extend_from_slice(&layer.ui_instances);
        let vec_start = all_vec.len() as u32;
        all_vec.extend_from_slice(&layer.vector_vertices);
        batches.push(LayerBatch {
            scissor: layer.scissor,
            ui_start,
            ui_count: layer.ui_instances.len() as u32,
            vec_start,
            vec_count: layer.vector_vertices.len() as u32,
        });
    }
    (all_ui, all_vec, batches)
}

fn set_pass_scissor(pass: &mut wgpu::RenderPass<'_>, scissor: Option<ScissorRect>, width: f32, height: f32) {
    if let Some(scissor) = scissor {
        pass.set_scissor_rect(scissor.x, scissor.y, scissor.w, scissor.h);
    } else {
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
    }
}

impl UiPipelines {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let globals_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ui_globals_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let glyph_bind_group_layout = globals_bind_group_layout.clone();
        let _ = glyph_bind_group_layout;

        let ui_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ui_shader"),
            source: wgpu::ShaderSource::Wgsl(UI_SHADER.into()),
        });
        let vector_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vector_shader"),
            source: wgpu::ShaderSource::Wgsl(VECTOR_SHADER.into()),
        });
        let world_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("world3d_shader"),
            source: wgpu::ShaderSource::Wgsl(WORLD3D_SHADER.into()),
        });

        let depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        let overlay_depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::Always,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

        let quad_vertices: &[f32] = &[
            0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0,
        ];
        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui_quad_vertices"),
            contents: bytemuck::cast_slice(quad_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui_globals"),
            contents: bytemuck::bytes_of(&UiGlobals {
                screen_size: [1.0, 1.0],
                _pad: [0.0, 0.0],
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let glyph_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph_atlas"),
            size: wgpu::Extent3d { width: 2048, height: 2048, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let glyph_view = glyph_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let glyph_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("glyph_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let icon_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("icon_atlas"),
            size: wgpu::Extent3d { width: 2048, height: 2048, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let icon_view = icon_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let icon_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("icon_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui_bind_group"),
            layout: &globals_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&glyph_sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&icon_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&icon_sampler) },
            ],
        });
        let ui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ui_pipeline_layout"),
            bind_group_layouts: &[&globals_bind_group_layout],
            push_constant_ranges: &[],
        });
        let ui_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ui_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 8,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<UiInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &ui_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: overlay_depth_state.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vector_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("vector_pipeline_layout"),
            bind_group_layouts: &[&globals_bind_group_layout],
            push_constant_ranges: &[],
        });
        let vector_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("vector_pipeline"),
            layout: Some(&vector_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vector_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<VectorVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 },
                        wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &vector_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: overlay_depth_state,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let world_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("world3d_globals_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: std::num::NonZeroU64::new(mem::size_of::<World3dGlobals>() as u64),
                },
                count: None,
            }],
        });

        let world_globals_ring = WorldGlobalsRing::new(device, &world_bind_group_layout, 8);

        let world_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("world3d_pipeline_layout"),
            bind_group_layouts: &[&world_bind_group_layout],
            push_constant_ranges: &[],
        });
        let world_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world3d_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &world_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<World3dVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: 12,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<World3dGpuInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute { offset: 0, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 16, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 32, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 48, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 64, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                            wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: depth_state.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let _ = queue;
        Self {
            ui_pipeline,
            vector_pipeline,
            world_pipeline,
            quad_vertex_buffer,
            globals_buffer,
            world_globals_ring,
            world_bind_group_layout,
            glyph_texture,
            glyph_sampler,
            icon_texture,
            icon_sampler,
            glyph_bind_group,
            bind_group_layout: globals_bind_group_layout,
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn globals_buffer(&self) -> &wgpu::Buffer {
        &self.globals_buffer
    }

    pub fn glyph_view(&self) -> wgpu::TextureView {
        self.glyph_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn glyph_sampler(&self) -> &wgpu::Sampler {
        &self.glyph_sampler
    }

    pub fn icon_view(&self) -> wgpu::TextureView {
        self.icon_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn icon_sampler(&self) -> &wgpu::Sampler {
        &self.icon_sampler
    }

    pub fn depth_format(&self) -> wgpu::TextureFormat {
        wgpu::TextureFormat::Depth24Plus
    }

    fn prepare_world_passes(draw: &DrawList) -> (Vec<PreparedWorldPass>, Vec<World3dGpuInstance>) {
        let mut prepared = Vec::new();
        let mut all_instances = Vec::new();
        for scene in &draw.scene_passes {
            let mut pass_draws = Vec::new();
            for draw_call in &scene.draws {
                if draw_call.instances.is_empty() {
                    continue;
                }
                let instance_offset = all_instances.len() as u32;
                let instance_count = draw_call.instances.len() as u32;
                for instance in &draw_call.instances {
                    all_instances.push(World3dGpuInstance::from_instance(
                        instance.model.to_cols_array(),
                        instance.color,
                        instance.selected,
                        instance.hovered,
                    ));
                }
                pass_draws.push(WorldDrawRange {
                    mesh_key: draw_call.mesh_key.clone(),
                    mesh_version: draw_call.mesh_version,
                    instance_offset,
                    instance_count,
                });
            }
            prepared.push(PreparedWorldPass {
                globals: World3dGlobals {
                    view_proj: scene.view_proj,
                    light_dir: [
                        scene.light_dir[0],
                        scene.light_dir[1],
                        scene.light_dir[2],
                        0.0,
                    ],
                },
                viewport: scene.viewport,
                draws: pass_draws,
            });
        }
        (prepared, all_instances)
    }

    fn upload_world_passes(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        draw: &DrawList,
        frame_buffers: &mut FrameBuffers,
    ) -> Option<Vec<PreparedWorldPass>> {
        if draw.scene_passes.is_empty() {
            return None;
        }
        let (prepared, all_instances) = Self::prepare_world_passes(draw);
        self.world_globals_ring.ensure_slots(
            device,
            &self.world_bind_group_layout,
            prepared.len() as u32,
        );
        let globals: Vec<World3dGlobals> = prepared.iter().map(|pass| pass.globals).collect();
        self.world_globals_ring.write_passes(queue, &globals);
        frame_buffers.world_instances.upload(
            device,
            queue,
            &all_instances,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            "world3d_instances",
        )?;
        Some(prepared)
    }

    fn draw_world_passes<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        mesh_store: &MeshGpuStore,
        prepared: &[PreparedWorldPass],
        instance_buffer: wgpu::BufferSlice<'a>,
        screen_w: f32,
        screen_h: f32,
    ) {
        let instance_stride = mem::size_of::<World3dGpuInstance>() as u64;
        pass.set_pipeline(&self.world_pipeline);
        for (slot, scene) in prepared.iter().enumerate() {
            let viewport = scene.viewport;
            pass.set_viewport(viewport[0], viewport[1], viewport[2], viewport[3], 0.0, 1.0);
            pass.set_scissor_rect(
                viewport[0] as u32,
                viewport[1] as u32,
                viewport[2] as u32,
                viewport[3] as u32,
            );
            pass.set_bind_group(
                0,
                &self.world_globals_ring.bind_group,
                &[self.world_globals_ring.offset_for_slot(slot as u32)],
            );
            for draw_call in &scene.draws {
                let store_key = MeshGpuStore::lookup_key(&draw_call.mesh_key, draw_call.mesh_version);
                let Some(mesh) = mesh_store.get(&store_key) else {
                    continue;
                };
                let byte_offset = draw_call.instance_offset as u64 * instance_stride;
                pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                pass.set_vertex_buffer(
                    1,
                    instance_buffer.slice(byte_offset..byte_offset + draw_call.instance_count as u64 * instance_stride),
                );
                pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..mesh.index_count, 0, 0..draw_call.instance_count);
            }
        }
        pass.set_viewport(0.0, 0.0, screen_w, screen_h, 0.0, 1.0);
        pass.set_scissor_rect(0, 0, screen_w as u32, screen_h as u32);
    }

    pub fn update_globals(&self, queue: &wgpu::Queue, width: f32, height: f32) {
        queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::bytes_of(&UiGlobals {
                screen_size: [width, height],
                _pad: [0.0, 0.0],
            }),
        );
    }

    pub fn upload_glyph_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.glyph_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    pub fn upload_icon_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.icon_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }

    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        overlay: Option<&DrawList>,
        mesh_store: &MeshGpuStore,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        self.update_globals(queue, width, height);
        let world_prepared = if depth_view.is_some() {
            self.upload_world_passes(device, queue, draw, frame_buffers)
        } else {
            None
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ui_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.06,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if let (Some(prepared), Some(instance_buffer)) =
            (world_prepared, frame_buffers.world_instances.slice())
        {
            self.draw_world_passes(&mut pass, mesh_store, &prepared, instance_buffer, width, height);
        }

        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        pass.set_scissor_rect(0, 0, width as u32, height as u32);

        let (all_ui, all_vec, batches) = build_layer_batches(draw);
        let ui_buffer = if all_ui.is_empty() {
            None
        } else {
            frame_buffers.ui_instances.upload(
                device,
                queue,
                &all_ui,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "ui_instances",
            )
        };
        let vector_buffer = if all_vec.is_empty() {
            None
        } else {
            frame_buffers.vector_vertices.upload(
                device,
                queue,
                &all_vec,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "vector_vertices",
            )
        };

        for batch in &batches {
            set_pass_scissor(&mut pass, batch.scissor, width, height);
            if batch.ui_count > 0 {
                if let Some(instance_buffer) = &ui_buffer {
                    pass.set_pipeline(&self.ui_pipeline);
                    pass.set_bind_group(0, &self.glyph_bind_group, &[]);
                    pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                    pass.set_vertex_buffer(1, instance_buffer.clone());
                    pass.draw(
                        0..6,
                        batch.ui_start..batch.ui_start + batch.ui_count,
                    );
                }
            }
            if batch.vec_count > 0 {
                if let Some(vector_buffer) = &vector_buffer {
                    pass.set_pipeline(&self.vector_pipeline);
                    pass.set_vertex_buffer(0, vector_buffer.clone());
                    pass.draw(
                        batch.vec_start..batch.vec_start + batch.vec_count,
                        0..1,
                    );
                }
            }
        }
        pass.set_scissor_rect(0, 0, width as u32, height as u32);
        drop(pass);
        if let Some(overlay) = overlay {
            let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_overlay_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_overlay(
                device,
                queue,
                &mut overlay_pass,
                overlay,
                frame_buffers,
                width,
                height,
            );
        }
    }

    pub fn render_overlay<'a>(
        &'a self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'a>,
        overlay: &DrawList,
        frame_buffers: &mut FrameBuffers,
        width: f32,
        height: f32,
    ) {
        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);

        let (all_ui, all_vec, batches) = build_layer_batches(overlay);
        let ui_buffer = if all_ui.is_empty() {
            None
        } else {
            frame_buffers.ui_instances.upload(
                device,
                queue,
                &all_ui,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "overlay_ui_instances",
            )
        };
        let vector_buffer = if all_vec.is_empty() {
            None
        } else {
            frame_buffers.vector_vertices.upload(
                device,
                queue,
                &all_vec,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                "overlay_vector_vertices",
            )
        };

        for batch in &batches {
            set_pass_scissor(pass, batch.scissor, width, height);
            if batch.ui_count > 0 {
                if let Some(instance_buffer) = &ui_buffer {
                    pass.set_pipeline(&self.ui_pipeline);
                    pass.set_bind_group(0, &self.glyph_bind_group, &[]);
                    pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                    pass.set_vertex_buffer(1, instance_buffer.clone());
                    pass.draw(
                        0..6,
                        batch.ui_start..batch.ui_start + batch.ui_count,
                    );
                }
            }
            if batch.vec_count > 0 {
                if let Some(vector_buffer) = &vector_buffer {
                    pass.set_pipeline(&self.vector_pipeline);
                    pass.set_vertex_buffer(0, vector_buffer.clone());
                    pass.draw(
                        batch.vec_start..batch.vec_start + batch.vec_count,
                        0..1,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ear_clip_polygon, mesh_content_version, DrawList, ScissorRect, WORLD_GLOBALS_SLOT_SIZE};
    use crate::geometry::Rect;
    use crate::theme::Rgba;

    #[test]
    fn scissor_intersects_child() {
        let a = ScissorRect { x: 0, y: 0, w: 100, h: 100 };
        let b = ScissorRect { x: 50, y: 50, w: 100, h: 100 };
        let c = a.intersect(&b);
        assert_eq!(c.w, 50);
        assert_eq!(c.h, 50);
    }

    #[test]
    fn scissor_from_rect_uses_top_left_origin() {
        let scissor = ScissorRect::from_rect(Rect::new(10.0, 20.0, 80.0, 60.0), 720.0);
        assert_eq!(scissor.x, 10);
        assert_eq!(scissor.y, 20);
        assert_eq!(scissor.w, 80);
        assert_eq!(scissor.h, 60);
    }

    #[test]
    fn draw_list_push_scissor_splits_layers() {
        let mut draw = DrawList::default();
        draw.set_screen_height(200.0);
        draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
        draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
        draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
        draw.pop_scissor();
        assert!(draw.layers.len() >= 3);
    }

    #[test]
    fn ear_clip_produces_triangles() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let tris = ear_clip_polygon(&square);
        assert!(tris.len() >= 3);
    }

    #[test]
    fn world_globals_slot_size_is_aligned() {
        assert!(WORLD_GLOBALS_SLOT_SIZE >= 80);
        assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
    }

    #[test]
    fn mesh_content_version_changes_with_indices() {
        let v0 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
        let v1 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
        assert_ne!(v0, v1);
    }
}
