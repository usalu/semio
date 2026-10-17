from specs.lib import *

CH = [
    ch('🔑', 'dep-hash-and-failures', 'dep_hash_and_failures', [(30, 168)],
       'Dependency hash and failures of the inference cache: the merkle dependency hash, the inference step and fault records, the work failure with its domain projections and the inference error.', glob=False),
    ch('🎯', 'inferred-field-and-config', 'inferred_field_and_config', [(170, 282)],
       'Inferred field contract of the inference cache: the pending and compute step enums, the inferred field trait, the cache configuration and the persistence mode.'),
    ch('🗄', 'cache-and-session', 'cache_and_session', [(286, 528)],
       'Cache and session of the inference cache: the owned value erasure, the content-addressed cache with its statistics and the per-session entry tables.'),
    ch('🔄', 'field-update', 'field_update', [(532, 709)],
       'Field update of the inference cache: the field change and delta records, the lane tracks and the incremental field update.'),
    ch('🧮', 'cursor', 'cursor', [(713, 835)],
       'Resumable inference cursor of the inference cache: the original scope, the cursor with its phases and the step report.'),
    ch('🏁', 'driver', 'driver', [(839, 1090)],
       'Driver of the inference cache: the step, whole-field, update and diff entry points and the event, commit and drive helpers behind them.'),
]

SPEC = dict(
    orig='inf.rs',
    file=INF + '/🦀️.rs',
    children=CH,
    external=mounted(INF + '/🦀️.rs'),
)
