from specs.lib import *
F = MOD + '/⚙️engine/🦀️.rs'
H = 'impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {'
CUTS = [
    ('pub fn pointer_down(&mut self, x: f64', H),
    ('pub fn render_snapshot(&self)', H),
    ('fn begin_draw_edge_from_handle(&mut self', H),
]


def pre(t):
    t, _ = cut_impls(t, CUTS)
    return t


CH = [
    ch('🧷', 'point-retirement-and-engine-aliases', 'point_retirement_and_engine_aliases', [(29, 86)],
       'Point retirement and engine aliases of the board engine: the retained point, the point buffer and its retirement cursor and the undirected and directed engine type aliases.'),
    ch('🧱', 'style-kinds-and-data-records', 'style_kinds_and_data_records', [(89, 258)],
       'Style kinds and data records of the board engine: the port mode, element style kinds, node, wire, edge and kind definitions, selection options, gesture stage and interaction.'),
    ch('🖌', 'canvas-palette', 'canvas_palette', [(260, 472), (970, 974)],
       'Canvas palette of the board engine: the palette colours and strokes with their derivation and defaults.'),
    ch('🎟', 'icon-paint-cache', 'icon_paint_cache', [(474, 967)],
       'Icon paint cache of the board engine: the cached icon bodies, tokens, slots, registry, lease and the retirement of the cache.'),
    ch('📜', 'descriptors-and-snapshot-json', 'descriptors_and_snapshot_json', [(977, 1241)],
       'Descriptors and snapshot JSON of the board engine: the edge, wire, region, scene and handle descriptors, the scene and snapshot codecs, the graph extension and the stroke scene encoder.'),
    ch('📷', 'camera-events-and-hit-model', 'camera_events_and_hit_model', [(1243, 1379)],
       'Camera, events and hit model of the board engine: the camera, board events, render snapshot, hit objects, pick targets, interaction modes, handle geometry and the port model trait.'),
    ch('🔲', 'selection-marquee', 'selection_marquee', [(1381, 1596)],
       'Selection marquee of the board engine: the selection thresholds, merge modes, drag shapes, preselection and containment tests.'),
    ch('🧭', 'transform-gumball', 'transform_gumball', [(1598, 1679)],
       'Transform gumball of the board engine: the ring constants, gumball flags and the pivot, rotation and snap helpers.'),
    ch('🗺', 'target-regions', 'target_regions', [(1681, 1832)],
       'Target regions of the board engine: the region constants, region data, bounds, grips, grip hit tests, grip drags and the scalar snap.'),
    ch('🏗', 'engine-state-and-close', 'engine_state_and_close', [(1837, 2026)],
       'Engine state and close of the board engine: the engine selection options, the graph engine struct, its backing retirement step, default and the close step.'),
    ch('🪛', 'engine-mutation', 'engine_mutation', [(2028, 2232)],
       'Engine mutation of the board engine: constructing, camera, node, handle and edge edits, selection options, area cancel, select all and delete selection.'),
    ch('🖱', 'engine-pointer', 'engine_pointer', [(2234, 2624)],
       'Engine pointer of the board engine: pointer down, move and up in world and screen space with selection, drag and draw edge handling.'),
    ch('🔎', 'engine-query-and-area-select', 'engine_query_and_area_select', [(2626, 2959)],
       'Engine query and area select of the board engine: the render snapshot, edge curves, wire snapping, pick application and the area selection and preselect helpers.'),
    ch('🔗', 'engine-connection-and-hit-test', 'engine_connection_and_hit_test', [(2961, 3271)],
       'Engine connection and hit test of the board engine: drawing an edge from a handle, connection validity, type refusal, node drag proximity, cycle checks and hit testing.'),
    ch('📐', 'layout-entry-points', 'layout_entry_points', [(3274, 3313)],
       'Layout entry points of the board engine: the force, ported force, hierarchical and redraw layout and the edge handle snap functions.'),
]
SPEC = dict(orig='eng.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='eng.rs'), pre=pre, pre_extra=['}', H] * len(CUTS))
