from specs.lib import *
F = U + '/🧠️runtime/⏭️cursor/🦀️.rs'
H = 'impl SurfaceReconcileCursor {'
B = [476, 974]
CH = [
    ch('🪞', 'surface-shadow-and-record-read', 'surface_shadow_and_record_read', [(23, 329)],
       'Surface shadow and record read: the `SurfaceShadow` document mirror with its reconcile, snapshot, transaction reader, seal, rejection and retirement, and the borrowed `SurfaceRecordRead`.'),
    ch('🧭', 'cursor-state-and-construction', 'cursor_state_and_construction', [(335, 476)],
       'Cursor state and construction: the reconcile stage and step, the `SurfaceReconcileCursor` state and its construction, usage and pending accessors.'),
    ch('🚶', 'cursor-step', 'cursor_step', [(478, 977)],
       'Cursor step: the single resumable `SurfaceReconcileCursor::step` that advances the reconcile one admitted unit at a time.'),
    ch('🧩', 'component-advance-and-retirement', 'component_advance_and_retirement', [(979, 1392)],
       'Component advance and retirement: the credit admission, fault handling, the existing and fresh component advance, the record assembly, the pending patch, the native resource close and the retirement step.'),
    ch('🔮', 'reconcile-oracle', 'reconcile_oracle', [(1398, 1582)],
       'Reconcile oracle: the testkit `SurfaceReconcileOracle` that replays a reconcile against an independent shadow.'),
]
SPEC = dict(orig='cursor.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='cursor.rs'))
