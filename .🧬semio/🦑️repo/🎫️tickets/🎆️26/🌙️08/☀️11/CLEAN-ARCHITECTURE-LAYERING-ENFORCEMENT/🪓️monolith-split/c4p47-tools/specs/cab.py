from specs.lib import *

CH = [
    ch('🧬', 'operation-schema', 'operation_schema', [(5, 93)],
       'Operation schema of the codec ABI: the schema document and limit constants, the operation and error codes, failure and format records and the format resolver.'),
    ch('📊', 'progress-records', 'progress_records', [(97, 147)],
       'Progress records of the codec ABI: phases, step states, steps, the cancel outcome, the progress counter and the pending input page.'),
    ch('🔤', 'utf8-cursor', 'utf8_cursor', [(149, 203)],
       'UTF-8 cursor of the codec ABI: the incremental UTF-8 validity cursor the structural cursors share.', glob=False),
    ch('🔎', 'filter-kinds-cursor', 'filter_kinds_cursor', [(205, 388)],
       'Filter kinds cursor of the codec ABI: the structural cursor that filters accepted media kinds one byte at a time.'),
    ch('🪪', 'normalize-kind-cursor', 'normalize_kind_cursor', [(390, 476)],
       'Normalize kind cursor of the codec ABI: the structural cursor that normalizes one stdio format kind one byte at a time.'),
    ch('🎟', 'codec-session', 'codec_session', [(478, 818)],
       'Codec session of the codec ABI: the cursor input union and the per-request session with its page, step, seal and close protocol.'),
    ch('🛰', 'retained-service', 'retained_service', [(820, 950)],
       'Retained service of the codec ABI: budget and handle validation and the retained codec service over a format resolver.'),
    ch('🚪', 'direct-service', 'direct_service', [(952, 1007)],
       'Direct service of the codec ABI: the public codec service over the registered resolver and the request metadata decoder.'),
]

SPEC = dict(
    orig='cab.rs',
    file=CAB + '/🦀️.rs',
    children=CH,
    external=mounted(CAB + '/🦀️.rs', orig='cab.rs') + [O + '/🖥️host/🧪️tests/🔬️codec-abi-unit/🦀️.rs'],
)
