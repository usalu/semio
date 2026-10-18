from specs.lib import *
F = FLOW + '/🕸️wasm/🎬️actions-a/🦀️.rs'
CH = [
    ch('🗒', 'catalogue-ports-and-evaluation', 'catalogue_ports_and_evaluation', [(15, 442)],
       'Catalogue, ports and evaluation actions 2504 to 2513: the host catalogue, kind infos, input and output port edits, port connection, the compiled wire literal and the evaluation outputs.'),
    ch('🖱', 'computing-hover-and-preview', 'computing_hover_and_preview', [(445, 758)],
       'Computing, hover and preview actions 2516 to 2529: the dag input failure mapping, the computing widget ids, the preview text, hover and hover channel and the preview off set.'),
    ch('🗜', 'preview-toggle-clusters-and-export', 'preview_toggle_clusters_and_export', [(760, 997)],
       'Preview toggle, cluster and export actions 2530 to 2535: the preview toggle, cluster collapse and explode and the pending export click and payload.'),
    ch('📝', 'slider-and-note-editing', 'slider_and_note_editing', [(999, 1435)],
       'Slider and note editing actions 2536 to 2545: the slider value and overlay and the note text, caret and edit session.'),
    ch('🖼', 'image-schemas-and-variables', 'image_schemas_and_variables', [(1437, 1609)],
       'Image, schema and variable actions 2546 to 2549: the image source, the schema list and the variable name and schema.'),
    ch('🔗', 'widget-graph-editing', 'widget_graph_editing', [(1611, 2044)],
       'Widget graph editing actions 2550 to 2560: adding, ghosting, removing and moving widgets, insert between, make space, neuron params, connect, disconnect and undo.'),
]
OLD = 'pub(super) fn dag_input_failure('
NEW = 'pub(crate) fn dag_input_failure('
LINE = 'pub(crate) fn dag_input_failure(error:semio_framework_value::ValueError)->FlowFailure{'
SPEC = dict(orig='acta.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='acta.rs'),
            patch={'🖱️computing-hover-and-preview': [(OLD, NEW)]}, expect_extra=[LINE],
            expect_missing=['pub(super) fn dag_input_failure(error:semio_framework_value::ValueError)->FlowFailure{'])
