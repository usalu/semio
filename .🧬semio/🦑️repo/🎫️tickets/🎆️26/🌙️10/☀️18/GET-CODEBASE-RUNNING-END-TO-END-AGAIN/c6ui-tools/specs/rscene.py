from specs.lib import *
F = U + '/🖌️render/🎬️scene/🦀️.rs'
CH = [
    ch('🔲', 'rect-quad-glass-and-stencil-primitives', 'rect_quad_glass_and_stencil_primitives', [(22, 251)],
       'Rect, quad, glass and stencil primitives: the layout, scissor and clip rects, the quad instance kinds, the vector vertex, the glass style, region and instance and the stencil policy.'),
    ch('🎭', 'surface-draws-and-pipeline-kinds', 'surface_draws_and_pipeline_kinds', [(257, 390)],
       'Surface draws and pipeline kinds: the mesh, line and textured draws, the lighting, material and shadow, the surface pass and the pipeline kind.'),
    ch('🏗', 'scene-builder', 'scene_builder', [(398, 680)],
       'Scene builder: the scene layer and the `SceneBuilder` with its lifecycle, scissor, silhouette clip, surface, quad, vector and glass recording.'),
    ch('🧮', 'scene-finish-and-render-packet', 'scene_finish_and_render_packet', [(686, 1045)],
       'Scene finish and render packet: the layer state, draw batch, scene errors, `Scene::finish` with its validate, snap, order, batch and hash passes and the `RenderPacket`.'),
]
SPEC = dict(orig='rscene.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='rscene.rs'))
