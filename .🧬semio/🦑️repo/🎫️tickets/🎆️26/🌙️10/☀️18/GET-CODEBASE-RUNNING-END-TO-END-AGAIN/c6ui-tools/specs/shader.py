from specs.lib import *
F = U + '/🖌️render/✨️shader-contract/🦀️.rs'
CH = [
    ch('🧱', 'pipeline-spec-types', 'pipeline_spec_types', [(18, 228)],
       'Pipeline spec types: the vertex formats and buffers, the shader stage visibility, the bind groups, the blend, colour write, depth and stencil specs, the primitive topology and the pipeline, shader variant and family records.'),
    ch('🧵', 'ui-and-vector-families', 'ui_and_vector_families', [(234, 527)],
       'Ui and vector families: the ui shader with its globals bind group, vertex buffers and mask and content pipelines, and the vector shader with its pipeline and family.'),
    ch('🌐', 'world3d-mesh-family', 'world3d_mesh_family', [(533, 910)],
       'World3d mesh family: the world bind groups and stencil, the world3d shader, the mesh vertex buffers and the opaque and translucent pipelines.'),
    ch('📏', 'world3d-lines-grid-and-textured-family', 'world3d_lines_grid_and_textured_family', [(912, 1223)],
       'World3d lines, grid and textured family: the lines, grid and textured shaders with their pipelines and the world3d family.'),
    ch('🌫', 'blur-glass-families-and-registry', 'blur_glass_families_and_registry', [(1229, 1471)],
       'Blur and glass families and the registry: the blur downsample and scene blit shaders and pipelines, the glass shader and pipeline and the `ALL_SHADERS` registry.'),
]
SPEC = dict(orig='shader.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='shader.rs'))
