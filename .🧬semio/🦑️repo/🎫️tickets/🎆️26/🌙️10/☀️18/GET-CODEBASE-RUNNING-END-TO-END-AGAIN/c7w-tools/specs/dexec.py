from specs.lib import *
F = W + '/🖍️draw/🔧️pipeline/🎬️execution/🦀️.rs'
H = 'impl UiPipelines {'
c = Cut(F, 'dexec.rs').impl(H, [359, 646, 927, 1412])
c.child('🧱', 'pipeline-state-and-world-passes', 'pipeline_state_and_world_passes', [(19, 358)],
        'Pipeline state and world passes: the pipeline set, the grow and frame buffers, the layer batches and scissor helpers, the accessors and the world pass preparation.')
c.child('🌍', 'world-pass-drawing', 'world_pass_drawing', [(359, 645)],
        'World pass drawing: the world pass upload and draw, the instance, silhouette, raster and vector draws and the interleaved layer rendering.')
c.child('🖼', 'atlas-uploads-and-instance-encoding', 'atlas_uploads_and_instance_encoding', [(646, 926)],
        'Atlas uploads and instance encoding: the globals update, the glyph and icon atlas uploads, the prepared scene clear and the ui, vector, shadow and world instance encoders.')
c.child('🎨', 'world-material-and-effect-encoding', 'world_material_and_effect_encoding', [(927, 1411)],
        'World material and effect encoding: the material, textured, grid, line, postprocess, scene clear, blur mip and glass scalar encoders.')
c.child('🪟', 'scene-content-glass-and-composite', 'scene_content_glass_and_composite', [(1412, 1889)],
        'Scene content, glass and composite: the prepared scene blits, the scene content and glass foreground rendering, the swapchain composite, the blur chain and the overlay render.')
SPEC = c.spec()
