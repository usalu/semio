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
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct VectorVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct ScissorRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct DrawList {
    pub scene_passes: Vec<ScenePass3d>,
    pub ui_instances: Vec<UiInstance>,
    pub vector_vertices: Vec<VectorVertex>,
    pub scissor: Option<ScissorRect>,
}

impl Default for DrawList {
    fn default() -> Self {
        Self {
            scene_passes: Vec::new(),
            ui_instances: Vec::new(),
            vector_vertices: Vec::new(),
            scissor: None,
        }
    }
}

impl DrawList {
    pub fn clear(&mut self) {
        self.scene_passes.clear();
        self.ui_instances.clear();
        self.vector_vertices.clear();
        self.scissor = None;
    }

    pub fn push_scene_pass(&mut self, pass: ScenePass3d) {
        self.scene_passes.push(pass);
    }

    pub fn push_solid(&mut self, rect: [f32; 4], color: Rgba) {
        self.ui_instances.push(UiInstance::solid(rect, color));
    }

    pub fn push_rounded(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
        self.ui_instances
            .push(UiInstance::rounded(rect, color, radius, 0.0, color));
    }

    pub fn push_glyph(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
        self.ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
    }

    pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let nx = -dy / len * width * 0.5;
        let ny = dx / len * width * 0.5;
        let c = [color.r, color.g, color.b, color.a];
        self.vector_vertices.extend_from_slice(&[
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
        for tri in 1..points.len() - 1 {
            self.vector_vertices.push(VectorVertex { position: points[0], color: c });
            self.vector_vertices
                .push(VectorVertex { position: points[tri], color: c });
            self.vector_vertices
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

    pub fn ensure_mesh(
        &mut self,
        device: &wgpu::Device,
        key: &str,
        positions: &[f32],
        normals: &[f32],
        indices: &[u32],
    ) {
        if self.meshes.contains_key(key) {
            return;
        }
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
            key.to_string(),
            GpuMeshBuffers {
                vertex_buffer,
                index_buffer,
                index_count: indices.len() as u32,
            },
        );
    }
}

fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}

pub(crate) struct UiPipelines {
    ui_pipeline: wgpu::RenderPipeline,
    vector_pipeline: wgpu::RenderPipeline,
    world_pipeline: wgpu::RenderPipeline,
    globals_bind_group_layout: wgpu::BindGroupLayout,
    world_bind_group_layout: wgpu::BindGroupLayout,
    glyph_bind_group_layout: wgpu::BindGroupLayout,
    quad_vertex_buffer: wgpu::Buffer,
    globals_buffer: wgpu::Buffer,
    world_globals_buffer: wgpu::Buffer,
    world_bind_group: wgpu::BindGroup,
    glyph_texture: wgpu::Texture,
    glyph_view: wgpu::TextureView,
    glyph_sampler: wgpu::Sampler,
    glyph_bind_group: wgpu::BindGroup,
    surface_format: wgpu::TextureFormat,
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
            ],
        });

        let glyph_bind_group_layout = globals_bind_group_layout.clone();

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

        let quad_vertices: &[f32] = &[
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0,
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
        let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui_bind_group"),
            layout: &globals_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&glyph_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&glyph_sampler) },
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
            depth_stencil: None,
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
            depth_stencil: None,
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
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let world_globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("world3d_globals"),
            contents: bytemuck::bytes_of(&World3dGlobals {
                view_proj: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                light_dir: [0.4, 0.6, 0.8, 0.0],
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let world_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout: &world_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: world_globals_buffer.as_entire_binding(),
            }],
        });

        let depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

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
            globals_bind_group_layout,
            world_bind_group_layout,
            glyph_bind_group_layout,
            quad_vertex_buffer,
            globals_buffer,
            world_globals_buffer,
            world_bind_group,
            glyph_texture,
            glyph_view,
            glyph_sampler,
            glyph_bind_group,
            surface_format: format,
        }
    }

    pub fn depth_format(&self) -> wgpu::TextureFormat {
        wgpu::TextureFormat::Depth24Plus
    }

    pub fn update_world_globals(&self, queue: &wgpu::Queue, globals: &World3dGlobals) {
        queue.write_buffer(&self.world_globals_buffer, 0, bytemuck::bytes_of(globals));
    }

    pub fn render_world_passes<'a>(
        &'a self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'a>,
        draw: &DrawList,
        mesh_store: &MeshGpuStore,
        screen_w: f32,
        screen_h: f32,
    ) {
        if draw.scene_passes.is_empty() {
            return;
        }
        pass.set_pipeline(&self.world_pipeline);
        pass.set_bind_group(0, &self.world_bind_group, &[]);
        for scene in &draw.scene_passes {
            let viewport = scene.viewport;
            pass.set_viewport(viewport[0], viewport[1], viewport[2], viewport[3], 0.0, 1.0);
            pass.set_scissor_rect(
                viewport[0] as u32,
                viewport[1] as u32,
                viewport[2] as u32,
                viewport[3] as u32,
            );
            self.update_world_globals(
                queue,
                &World3dGlobals {
                    view_proj: scene.view_proj,
                    light_dir: [
                        scene.light_dir[0],
                        scene.light_dir[1],
                        scene.light_dir[2],
                        0.0,
                    ],
                },
            );
            for draw_call in &scene.draws {
                let Some(mesh) = mesh_store.get(&draw_call.mesh_key) else {
                    continue;
                };
                if draw_call.instances.is_empty() {
                    continue;
                }
                let gpu_instances: Vec<World3dGpuInstance> = draw_call
                    .instances
                    .iter()
                    .map(|instance| {
                        World3dGpuInstance::from_instance(
                            instance.model.to_cols_array(),
                            instance.color,
                            instance.selected,
                            instance.hovered,
                        )
                    })
                    .collect();
                let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("world3d_instances"),
                    contents: bytemuck::cast_slice(&gpu_instances),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                pass.set_vertex_buffer(1, instance_buffer.slice(..));
                pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..mesh.index_count, 0, 0..gpu_instances.len() as u32);
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

    pub fn render<'a>(
        &'a self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        depth_view: Option<&'a wgpu::TextureView>,
        draw: &DrawList,
        mesh_store: &MeshGpuStore,
        width: f32,
        height: f32,
    ) {
        self.update_globals(queue, width, height);
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

        if depth_view.is_some() {
            self.render_world_passes(device, queue, &mut pass, draw, mesh_store, width, height);
        }

        pass.set_pipeline(&self.ui_pipeline);
        pass.set_bind_group(0, &self.glyph_bind_group, &[]);

        if let Some(scissor) = &draw.scissor {
            pass.set_scissor_rect(scissor.x, scissor.y, scissor.w, scissor.h);
        }

        if !draw.ui_instances.is_empty() {
            let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("ui_instances"),
                contents: bytemuck::cast_slice(&draw.ui_instances),
                usage: wgpu::BufferUsages::VERTEX,
            });
            pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, instance_buffer.slice(..));
            pass.draw(0..6, 0..draw.ui_instances.len() as u32);
        }

        if !draw.vector_vertices.is_empty() {
            pass.set_pipeline(&self.vector_pipeline);
            let vector_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vector_vertices"),
                contents: bytemuck::cast_slice(&draw.vector_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            pass.set_vertex_buffer(0, vector_buffer.slice(..));
            pass.draw(0..draw.vector_vertices.len() as u32, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ear_clip_polygon;

    #[test]
    fn ear_clip_produces_triangles() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let tris = ear_clip_polygon(&square);
        assert!(tris.len() >= 3);
    }
}
