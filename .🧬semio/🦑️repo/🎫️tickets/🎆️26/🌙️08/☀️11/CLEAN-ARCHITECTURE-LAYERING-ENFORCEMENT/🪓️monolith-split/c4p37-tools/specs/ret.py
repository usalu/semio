from specs.lib import *

CH = [
    ch('📄', 'pages-and-source-state', 'pages_and_source_state', [(15, 118)],
       'Pages and source state of the retained Pack source: the close demands, the page record, the source events, progress, faults and the source cursor record.'),
    ch('🚰', 'source-cursor', 'source_cursor', [(120, 333)],
       'Source cursor of the retained Pack source: the implementation that admits canonical bytes page by page and its drop guard.'),
    ch('🧱', 'segment-and-varint-cursors', 'segment_and_varint_cursors', [(335, 826)],
       'Segment and varint cursors of the retained Pack source: the segment header and events, the varint cursor and the segment cursor with its drop guard.'),
    ch('⚓', 'catalog-and-anchor-cursors', 'catalog_and_anchor_cursors', [(828, 1065)],
       'Catalog and anchor cursors of the retained Pack source: the catalog cause and fault, the paged kind helper and the anchor cursor with its default and drop guard.'),
]

SPEC = dict(
    orig='ret.rs',
    file=RET + '/🦀️.rs',
    children=CH,
    external=mounted(RET + '/🦀️.rs'),
)
