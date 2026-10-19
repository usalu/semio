from specs.lib import *
F = W + '/🖍️draw/🎨️shaders/🦀️.rs'
c = Cut(F, 'shad.rs')
c.child('🖼', 'ui-vector-glass-and-raster-mip-shaders', 'ui_vector_glass_and_raster_mip_shaders', [(4, 208), (1062, 1138)],
        'Ui, vector, glass and raster mip shaders: the WGSL sources of the ui instance, vector, glass and raster mip passes.')
c.child('🌍', 'world3d-shader-and-edits', 'world3d_shader_and_edits', [(210, 626)],
        'World3d shader and edits: the world3d WGSL source, the painted and authored shader edit tables and their assembling functions.')
c.child('✨', 'world3d-effect-shaders', 'world3d_effect_shaders', [(628, 1060)],
        'World3d effect shaders: the WGSL sources of the world3d celebration, lines, grid, textured, blur downsample, scene blit and postprocess passes.')
SPEC = c.spec()
