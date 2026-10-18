from specs.lib import *
SC = LY + '/⚙️engine/🎬️scene'
CH = [
    ch('🎨', 'display-list-and-hit-testing', 'display_list_and_hit_testing', [(30, 216)],
       'Display list and hit testing of the layout scene: the display colour, glyph, rectangle, guide, text run, image and stroke records, the display list, the stacking and paint order helpers, the page frame hit test and the margin guides.'),
    ch('🔤', 'text-fonts', 'text_fonts', [(220, 360)],
       'Text fonts of the layout scene: the layout sans font, the installed font set and its install errors, the byte and native font installers, the font asset names and declarations and the package font set.'),
    ch('⚙', 'layout-engine-and-views', 'layout_engine_and_views', [(362, 480)],
       'Layout engine and views of the layout scene: the layout engine state, its story layout, the paragraph defaults, the interactive and per page display list builders and the interactive view and layer visibility checks.'),
    ch('✍', 'drawing-labels-and-rasters', 'drawing_labels_and_rasters', [(483, 664)],
       'Drawing labels and rasters of the layout scene: the drawing label collection, fitting and shaping, the background scene, the drawing raster collection with its png data urls and the raster placement.'),
    ch('🖌', 'drawing-strokes-and-paths', 'drawing_strokes_and_paths', [(666, 941)],
       'Drawing strokes and paths of the layout scene: the raw drawing strokes, their kinds and placement in a frame, the stroke paint plan, the draw node collection, the curve point helpers and the display stroke painting.'),
    ch('🧩', 'assemble-display-list', 'display_list_assembly', [(943, 1105)],
       'Assemble display list of the layout scene: the assembly of one page display list from frames, stories, drawings, guides and layers.'),
    ch('🔠', 'style-spans-and-preview-marks', 'style_spans_and_preview_marks', [(1108, 1288)],
       'Style spans and preview marks of the layout scene: the story run glyphs, the style spans and span style resolution, the placed glyphs and the painted preview marks.'),
    ch('🎬', 'scene-painting-and-queries', 'scene_painting_and_queries', [(1290, 1410)],
       'Scene painting and queries of the layout scene: the drop preview, the display list to scene painter, the scene query record and the document JSON scene, hit test and screen to world queries.'),
]
SPEC = dict(orig='sc.rs', file=SC + '/🦀️.rs', children=CH, external=mounted(SC + '/🦀️.rs', orig='sc.rs'))
