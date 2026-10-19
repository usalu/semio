from specs.lib import *
F = R + '/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs'
c = Cut(F, 'canv2d.rs')
c.child('🪣', 'packet-image-and-blend', 'packet_image_and_blend', [(5, 293), (941, 1045)],
        'Packet, image and blend: the canvas fill mirror and packet records, the image decode and upload queue, the checkerboard, grid and dashed line, the gradient sampling, the blend modes and the shapes region helpers.')
c.child('🔷', 'shape-fills-and-render', 'shape_fills_and_render', [(295, 600)],
        'Shape fills and render: the shape fill and outline pushes, the gradient fills, the packet item render, the framing and `render_canvas_2d`.')
c.child('🖼', 'scene-nodes-and-image-upload', 'scene_nodes_and_image_upload', [(602, 939)],
        'Scene nodes and image upload: the paint order, the scene node render and path fills, the canvas constants, the raster ledger reservation, the image upload and the admission refusal.')
SPEC = c.spec()
