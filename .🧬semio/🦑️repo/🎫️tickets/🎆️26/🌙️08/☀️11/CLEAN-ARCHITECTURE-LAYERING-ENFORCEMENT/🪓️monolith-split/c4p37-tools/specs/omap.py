from specs.lib import *

CH = [
    ch('🗂', 'map-core', 'map_core', [(8, 132)],
       'Map core of the retained ordered map: the page capacity, the map record, its default, lookup and iteration implementations and the serde and retirement bridges.'),
    ch('🔁', 'clone-cursor', 'clone_cursor', [(134, 435)],
       'Clone cursor of the retained ordered map: the resumable cursor that copies a map page by page and its retained clone implementation.'),
    ch('🔢', 'bounded-ord', 'bounded_ord', [(437, 597)],
       'Bounded ordering of the retained ordered map: the grant, progress and step records, the bounded ordering trait and cursors for scalars and strings.'),
    ch('🔎', 'lookup-cursor', 'lookup_cursor', [(599, 677)],
       'Lookup cursor of the retained ordered map: the lookup outcome and the resumable key lookup.'),
    ch('➕', 'insert-cursor', 'insert_cursor', [(679, 992)],
       'Insert cursor of the retained ordered map: the insertion step, grant and progress records, the workspace and state and the resumable insertion cursor with its drop guard.'),
]

SPEC = dict(
    orig='omap.rs',
    file=OMAP + '/🦀️.rs',
    children=CH,
    rebase_super=True,
    external=mounted(OMAP + '/🦀️.rs', OMAP + '/📦️packages/🦀️rust/🦀️.rs'),
)
