from specs.lib import *
F = U + '/🧬️contract/🌱️values/🦀️.rs'
CH = [
    ch('🔢', 'fixed-list', 'fixed_list', [(10, 330)],
       'Fixed list: the `UiFixedList` paged list with its iterators, indexing, serde and value conversions.'),
    ch('🗺', 'fixed-map-and-bytes', 'fixed_map_and_bytes', [(334, 624)],
       'Fixed map and bytes: the `UiFixedMap` keyed by bounded text and the `UiFixedBytes` byte buffer with their serde and value conversions.'),
    ch('🏟', 'collection-arena', 'collection_arena', [(626, 974)],
       'Collection arena: the collection kinds and handles, the page and collection slots, the process-wide value arena and its headroom.'),
    ch('📃', 'ui-list', 'ui_list', [(981, 1161)],
       'Ui list: the arena-backed `UiList` with its cursor, builder and serde.'),
    ch('🗝', 'ui-map', 'ui_map', [(1163, 1400)],
       'Ui map: the arena-backed `UiMap` with its cursor, builder and serde.'),
]
SPEC = dict(orig='values.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='values.rs'))
