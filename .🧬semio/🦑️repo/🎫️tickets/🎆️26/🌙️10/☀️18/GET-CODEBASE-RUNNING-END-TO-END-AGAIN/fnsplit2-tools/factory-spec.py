import json, sys
V = "️"
F = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🏗️pipeline-resources-construction/🦀️.rs"
def child(e, name, text):
    return {"dir": e + V + name, "doc": "//! %s%s %s See [`super`]." % (e, V, text)}
c1 = child("🔰", "base-resources-and-ui-pipelines", "Base resources and ui pipelines: the globals layout, shaders, atlases, glyph bind group and the ui, vector and mask pipelines.")
c2 = child("🌍", "world-geometry-pipelines", "World geometry pipelines: the world layouts, shadow target and the mesh, section, shadow and line pipelines.")
c3 = child("🎭", "material-pipelines", "Material pipelines: the authored, painted and celebration layouts and the thirteen material pipelines.")
c4 = child("🖼", "textured-grid-and-post-pipelines", "Textured, grid and post processing pipelines: the textured world samplers and pipeline, the grid, the postprocess, blur, blit and glass pipelines.")
def ph(name, doc, a, b, ch, struct=None, struct_doc=None):
    d = {"name": name, "doc": doc, "from": a, "to": b, "child": ch}
    if struct: d.update(struct=struct, struct_doc=struct_doc)
    return d
spec = {
    "file": F, "function": "new",
    "types": {
        "world_encoded_format": "gpu_device::TextureFormat",
        "depth_state": "Option<gpu_device::DepthStencilState>", "overlay_depth_state": "Option<gpu_device::DepthStencilState>",
        "translucent_depth_state": "Option<gpu_device::DepthStencilState>", "world_line_depth_state": "Option<gpu_device::DepthStencilState>",
        "world_globals_ring": "WorldGlobalsRing", "world_shadow_target": "WorldShadowTarget",
        "mask_pipeline": "gpu_device::RenderPipeline",
        "world_material_samplers": "Vec<(crate::kernel_3d_scene::SceneTextureSampler3d, gpu_device::Sampler)>",
    },
    "modes": {"format": "val", "world_encoded_format": "val"},
    "cfg": {"mask_pipeline": '#[cfg(any(test,feature="testkit"))]'},
    "keep": [844, 845],
    "phases": [
        ph("build_base_resources", "🧱 Builds the globals layout, the ui, vector and world shaders, the depth states, the quad and globals buffers, the glyph and icon atlases and the glyph bind group.", 11, 94, c1, "BaseResources", "📦 The globals layout, shaders, depth states, buffers, atlases and glyph bind group that every later pipeline reads."),
        ph("build_ui_pipelines", "🖌 Builds the ui pipeline layout, the silhouette mask pipeline (tests and testkit) and the ui and vector pipelines.", 105, 178, c1, "UiPipelines", "🗂 The ui, vector and mask pipelines."),
        ph("build_world_layouts_and_targets", "🌐 Builds the world and shadow bind group layouts, the globals ring, the shadow target and the world pipeline layouts.", 204, 231, c2, "WorldLayouts", "🧭 The world bind group layouts, the globals ring, the shadow target and the world pipeline layouts."),
        ph("build_world_geometry_pipelines", "🏔 Builds the world mesh, translucent, section stencil, section cap, shadow and line pipelines and their depth states.", 233, 342, c2, "WorldGeometryPipelines", "🏗 The world mesh, section, shadow and line pipelines and the line depth state."),
        ph("build_material_layouts_and_shaders", "🎛 Builds the post processing shaders, the blur, postprocess, scene and authored layouts, the white authored texture and the authored, painted and celebration shaders and pipeline layouts.", 368, 430, c3, "MaterialLayouts", "🎚 The blur, postprocess, scene and authored layouts and the material shaders and pipeline layouts."),
        ph("build_material_pipelines", "🎨 Builds the standard, authored, painted and celebration material pipelines in their opaque and translucent variants.", 431, 591, c3, "MaterialPipelines", "🖍 The thirteen material pipelines."),
        ph("build_textured_resources", "🧵 Builds the textured and grid shaders, the plane buffer and sampler, the material sampler matrix and the textured world pipeline.", 608, 662, c4, "TexturedResources", "🧶 The textured and grid shaders, the plane buffer and sampler, the material samplers and the textured pipeline."),
        ph("build_grid_resources", "🔲 Builds the procedural grid layout, uniform buffer, bind group and pipeline.", 707, 725, c4, "GridResources", "🕸 The procedural grid uniform buffer, bind group and pipeline."),
        ph("build_post_processing_resources", "✨ Builds the blur and postprocess globals buffers and the postprocess, blur downsample, scene blit and glass pipelines.", 753, 811, c4, "PostProcessingResources", "🪄 The blur and postprocess globals buffers and the postprocess, blur, blit and glass pipelines."),
    ],
}
json.dump(spec, open(sys.argv[1], "w", encoding="utf8"), ensure_ascii=False, indent=1)
