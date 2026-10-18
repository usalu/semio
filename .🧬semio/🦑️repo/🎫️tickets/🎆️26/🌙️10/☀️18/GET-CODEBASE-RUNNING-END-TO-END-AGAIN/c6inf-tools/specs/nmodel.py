from specs.lib import *
F = NRM + '/🗃️model/🦀️.rs'
CH = [
    ch('📸', 'session-json-gestures-and-catalog-parsing', 'session_json_gestures_and_catalog_parsing', [(20, 111)],
       'Session JSON, gestures and catalog parsing of the board model: the brush preview snapshot from session JSON, gesture release events, drag records, pick targets, the draw level of detail label and the catalog parsers.'),
    ch('🎨', 'css-color-parsing', 'css_color_parsing', [(113, 240)],
       'CSS color parsing of the board model: hex, hsl and general CSS colors with the hue, saturation, alpha and byte conversions.'),
    ch('🖌', 'style-resolution-and-edge-tips', 'style_resolution_and_edge_tips', [(242, 448)],
       'Style resolution and edge tips of the board model: explicit style kinds, locked dimming, chrome passes, node, handle and edge strokes, colour interpolation, edge tips on curves and wire strokes.'),
    ch('🔌', 'port-shapes-and-brush-geometry', 'port_shapes_and_brush_geometry', [(450, 526)],
       'Port shapes and brush geometry of the board model: pair compatibility, port shapes and families, link gesture rules, brush handle alignment, brush template positions and region paint bounds.'),
    ch('⚠', 'port-error', 'port_error', [(528, 606)],
       'Port error of the board model: the normal port error with its display, error and from-JSON conversions.', glob=False),
    ch('🎚', 'constants-lod-and-naming', 'constants_lod_and_naming', [(608, 755)],
       'Constants, level of detail and naming of the board model: the hit tolerance and link constants, the draw level of detail, its scale JSON and the board and placement naming.'),
    ch('🎛', 'chrome-passes-and-brush-candidates', 'chrome_passes_and_brush_candidates', [(757, 877)],
       'Chrome passes and brush candidates of the board model: the style chrome pass, the handle paint layer, brush candidates, the candidate page and the brush preview snapshot.'),
    ch('🪣', 'fill-stage-and-drag-records', 'fill_stage_and_drag_records', [(879, 999)],
       'Fill stage and drag records of the board model: the fill stage, the transform, region and paint drag records, the drop preview snapshot and the pick target JSON.'),
    ch('📣', 'event-kinds-and-payload-builder', 'event_kinds_and_payload_builder', [(1001, 1263)],
       'Event kinds and payload builder of the board model: the event capacities, event kinds, pointer offsets, gesture motion, event faults, owned events, the payload builder and the event reservations.'),
    ch('📬', 'owned-event-impl', 'owned_event_impl', [(1265, 1743)],
       'Owned event impl of the board model: the large owned event method block.'),
    ch('🗂', 'event-queue-and-world-cache', 'event_queue_and_world_cache', [(1745, 2063)],
       'Event queue and world cache of the board model: the event queue, node icon spans, the world content cache, property retirement credits, pointer and descriptor capacities and the descriptor admission.'),
]
SPEC = dict(orig='nmodel.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='nmodel.rs'))
