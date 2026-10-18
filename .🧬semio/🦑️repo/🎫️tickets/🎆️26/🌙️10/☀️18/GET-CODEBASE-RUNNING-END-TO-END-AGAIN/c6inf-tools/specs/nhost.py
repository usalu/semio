from specs.lib import *
F = NRM + '/🧭️host/🦀️.rs'
H = 'impl BoardHost {'
CUTS = [('pub fn pointer_leave_screen', H)]


def pre(t):
    t, _ = cut_impls(t, CUTS)
    return t


CH = [
    ch('🎨', 'brush-session-and-event-authority', 'brush_session_and_event_authority', [(22, 314)],
       'Brush session and event authority of the normal board host: the brush session mirror, active utility, suggestion offset, brush kind weights and slots and the event authority steps.'),
    ch('🪝', 'pointer-commit-steps', 'pointer_commit_steps', [(316, 771)],
       'Pointer commit steps of the normal board host: the pointer commit, selection preview, selection and brush pointer commit steps.'),
    ch('🧹', 'host-default-and-close', 'host_default_and_close', [(773, 1195)],
       'Host default and close of the normal board host: the default host, the non-opaque close step and the terminal emptiness check.'),
    ch('🖱', 'pointer-screen-events', 'pointer_screen_events', [(1197, 1614)],
       'Pointer screen events of the normal board host: pointer down, move and up in screen space.'),
    ch('🛑', 'cancel-and-transform-gumball', 'cancel_and_transform_gumball', [(1616, 1825)],
       'Cancel and transform gumball of the normal board host: pointer leave and cancel, area select cancel and the transform gumball geometry, JSON, drag and paint.'),
    ch('🕰', 'legacy-pointer-differential', 'legacy_pointer_differential', [(1827, 1973)],
       'Legacy pointer differential of the normal board host: the testkit differential that replays a pointer commit against the legacy path.', cfg_mod='#[cfg(any(test, feature = "testkit"))]'),
    ch('🏠', 'host-struct-and-retirement', 'host_struct_and_retirement', [(1975, 2045)],
       'Host struct and retirement of the normal board host: the canvas content impl, the host struct, its retirement owner and the node move byte count.'),
    ch('🌐', 'world-content-and-pointer-planning', 'world_content_and_pointer_planning', [(2047, 2336)],
       'World content and pointer planning of the normal board host: descriptor sync, cached world content, the vector scene, hover and the pointer plan, commit, cancel and close steps.'),
    ch('🗑', 'delete-planning', 'delete_planning', [(2338, 2497)],
       'Delete planning of the normal board host: the delete selection plan, the pending delete step and the board delete selection entry point.'),
    ch('🗺', 'pointer-planners', 'pointer_planners', [(2499, 2798)],
       'Pointer planners of the normal board host: the selection, link, hover, brush and drag pointer planners and the pointer commit and cancel begin entry points.'),
]
SPEC = dict(orig='nhost.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='nhost.rs'), pre=pre, pre_extra=['}', H] * len(CUTS), inert_regions=True)
