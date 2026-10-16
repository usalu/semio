"""Board crate set: id, merged bands (bottom to top), folder relative to the infinite root."""
B = '🎲️board'
NR = B + '/🔌️ports/➡️directed/➕️normal'
DR = B + '/🔌️ports/➡️directed/🕸️dag'
CRATES = [
 ('schema',          ['schema', 'infer', 'iotext'],                                       B + '/🧬️schema'),
 ('engine',          ['engine', 'ports'],                                                 B + '/⚙️engine'),
 ('normal-model',    ['n-types', 'n-style', 'n-fill', 'n-events', 'n-scene', 'n-pointer'], NR + '/🗃️model'),
 ('normal-scene',    ['b-scene', 'b-style'],                                              NR + '/🖼️scene'),
 ('normal-focus',    ['b-focus', 'b-outbox'],                                             NR + '/🎯️focus'),
 ('normal-gestures', ['b-brush', 'b-gestures', 'b-commit', 'b-closing'],                 NR + '/🕹️gestures'),
 ('normal-fill',     ['b-fill'],                                                          NR + '/🪣️fill'),
 ('normal-host',     ['b-host'],                                                          NR + '/🧭️host'),
 ('dag-model',       ['dm-root', 'dm-input', 'dm-sel', 'dm-frame', 'dm-mutate', 'dm-paint', 'dm-wasm'], DR + '/🗃️model'),
 ('dag-graph',       ['d-graph', 'd-viewport'],                                           DR + '/🕸️graph'),
 ('dag-host',        ['d-pointer', 'd-paint', 'd-minimap', 'd-notes', 'd-host'],          DR + '/🧭️host'),
]
TITLES = {
 'schema': ('🧬️', 'Board schema: layout contracts, palette, dag input records, layout inference algorithms and the text io codecs.'),
 'engine': ('⚙️', 'Board engine: the retained graph engine with camera, selection and hit testing, plus the directed port descriptors it exposes.'),
 'normal-model': ('🗃️', 'Normal board model: events, plans, kinds, styles-neutral records and constants shared by the board host components.'),
 'normal-scene': ('🖼️', 'Normal board scene and style components with the scene painting, style resolution and viewport functions over them.'),
 'normal-focus': ('🎯️', 'Normal board focus and outbox components: selection, hover, picking, descriptor sync and event emission.'),
 'normal-gestures': ('🕹️', 'Normal board brush, gesture, commit and closing components with their gesture and commit functions.'),
 'normal-fill': ('🪣️', 'Normal board fill: snapshot capture and ingress, the fill job state machine and the commit encoder.'),
 'normal-host': ('🧭️', 'Normal board host: the composed `BoardHost` owner with pointer and render orchestration.'),
 'dag-model': ('🗃️', 'Dag board model: geometry, LOD, preview records, errors, edits and the host snapshot loaders.'),
 'dag-graph': ('🕸️', 'Dag board graph and viewport components with the engine sync, selection and projection functions.'),
 'dag-host': ('🧭️', 'Dag board host: the composed `DagHost` owner with pointer, paint, minimap and note orchestration.'),
}
PREFIX = 'semio_framework_os_infinite_board_'
def crate_ident(cid): return PREFIX + cid.replace('-', '_')
def crate_pkg(cid): return 'semio-framework-os-infinite-board-' + cid
