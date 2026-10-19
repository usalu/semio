from specs.lib import *
F = W + '/🖍️draw/🎮️execution/🦀️.rs'
H = 'impl GpuContext {'
c = Cut(F, 'dexe2.rs').impl(H, [511, 765])
c.child('🎚', 'present-cursor-and-helpers', 'present_cursor_and_helpers', [(16, 281)],
        'Present cursor and helpers: the atlas upload cursor, the present phase, the opportunity budget, the glass and clip command helpers and the prepared present cursor with its drop.')
c.child('🧠', 'gpu-context-and-meshes', 'gpu_context_and_meshes', [(288, 510)],
        'Gpu context and meshes: the session roots, the `GpuContext` state, the constructors, the readback, the target and depth management and the mesh upload steps.')
c.child('🧷', 'prepared-upload-and-present-steps', 'prepared_upload_and_present_steps', [(511, 764)],
        'Prepared upload and present steps: the prepared upload step, the begin present and the prepared present advance.')
c.child('🖍', 'draw-scalar-encoding-and-raster-delegates', 'draw_scalar_encoding_and_raster_delegates', [(765, 1077)],
        'Draw scalar encoding and raster delegates: the prepared draw scalar encoder, the atlas uploads, the raster texture delegates and the size accessors.')
SPEC = c.spec()
