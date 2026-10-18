from specs.lib import *
F = FLOW + '/🖥️host/⏱️eval-session/🦀️.rs'
H = 'impl FlowEvalSession {'
CUTS = [
    ('pub fn next_invocation_origin_capacity_bytes', H),
    ('pub fn retain_tick_host', H),
    ('pub fn next_window_tick_latch_copy_byte_demand', H),
    ('pub fn flow_extension_generation', H),
    ('pub fn note_pending_tessellate', H),
    ('pub fn begin_close', H),
]


def pre(t):
    t, _ = cut_impls(t, CUTS)
    return t


CH = [
    ch('🔌', 'construction-and-geometry-port', 'construction_and_geometry_port', [(5, 73)],
       'Construction and geometry port of the evaluation session: the operator registry and geometry authority builders and the session constructor.'),
    ch('📨', 'invocation-origin-reply-and-snapshot-intake', 'invocation_origin_reply_and_snapshot_intake', [(75, 169)],
       'Invocation origin, reply and snapshot intake of the evaluation session: the retained invocation origins and replies, the neural cache and the tick snapshot intake steps.'),
    ch('🌡', 'baseline-tick-sync-and-eval-json', 'baseline_tick_sync_and_eval_json', [(171, 394)],
       'Baseline, tick, sync and eval JSON of the evaluation session: the tick host, baseline capture, tick evaluation and display steps, repaint, the eval JSON lease and publication and the pending flag.'),
    ch('🔒', 'window-tick-latch', 'window_tick_latch', [(397, 601)],
       'Window tick latch of the evaluation session: the latch demands, arming, owed ticks, extension bookkeeping, continuation admission and latch retirement.'),
    ch('🧩', 'extension-generation-and-preview-retention', 'extension_generation_and_preview_retention', [(604, 774)],
       'Extension generation and preview retention of the evaluation session: the flow extension generation and invalidation, evaluate faults, the node cache seed and the retained preview meshes.'),
    ch('🔭', 'preview-tessellation-cancellation-and-status', 'preview_tessellation_cancellation_and_status', [(776, 979)],
       'Preview tessellation, cancellation and status of the evaluation session: pending tessellation, preview cancellation steps, the preview evaluation and chain status and the tessellate resolution.'),
    ch('🏁', 'close-and-retirement', 'close_and_retirement', [(981, 1090)],
       'Close and retirement of the evaluation session: closing, the session source, the close and retirement demands and steps and the terminal state.'),
]
SPEC = dict(orig='evs.rs', nest=True, file=F, children=CH, wiring='block', external=mounted(F, orig='evs.rs'), pre=pre, pre_extra=['}', H] * len(CUTS))
