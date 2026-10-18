from specs.lib import *
F = U + '/🎬️scene/🎬️scenes/🖼️plane/🦀️.rs'
CH = [
    ch('🔲', 'canvas2d-scene', 'canvas2d_scene', [(23, 202), (1122, 1134)],
       'Canvas2d scene: the `Canvas2dScene` payload with its pack wire, scene doc, value conversions and lanes, and the canvas framing value conversions.'),
    ch('🎨', 'paint2d-scene', 'paint2d_scene', [(206, 393)],
       'Paint2d scene: the `Paint2dScene` payload with its scene doc, lanes and value conversions.'),
    ch('🗺', 'tiled-map-scene', 'tiled_map_scene', [(397, 649)],
       'Tiled map scene: the `TiledMapScene` payload with its scene doc, lanes, serde defaults and value conversions.'),
    ch('🎲', 'board2d-scene', 'board2d_scene', [(653, 1019)],
       'Board2d scene: the `Board2dScene` payload with its pack wire, scene doc, lanes, serde defaults and value conversions.'),
    ch('🖋', 'ink-canvas-scene', 'ink_canvas_scene', [(1023, 1119)],
       'Ink canvas scene: the ink canvas interaction domain and the `InkCanvasScene` payload with its scene doc and value conversions.'),
]
SPEC = dict(orig='plane.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='plane.rs'))
