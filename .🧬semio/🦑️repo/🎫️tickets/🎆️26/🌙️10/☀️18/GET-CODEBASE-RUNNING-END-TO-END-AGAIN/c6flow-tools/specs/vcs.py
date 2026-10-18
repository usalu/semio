from specs.lib import *
F = FLOW + '/🌿️vcs/🦀️.rs'
H = 'impl FlowRetainedVcs {'
CUTS = [
    ('pub fn poll(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant)', H),
    ('pub fn next_close_operation_demands', H),
    ('pub fn preflight', H),
    ('fn step_action_cursor', H),
]


def pre(t):
    t, _ = cut_impls(t, CUTS)
    return t


CH = [
    ch('🔤', 'vocabulary-census-and-digest', 'vocabulary_census_and_digest', [(37, 224), (1787, 1833)],
       'Vocabulary, census and digest of the Flow VCS: the declared limits, credits, resource fingerprint, census, source, authority, handle, grant, close phase, fault, page and poll types plus the fixture, widget and synapse census fns and the host snapshot scalar digest.'),
    ch('🪟', 'surface-owner-action-and-cursor', 'surface_owner_action_and_cursor', [(226, 426)],
       'Surface owner, action and cursor of the Flow VCS: the surface owner, the action and stage enums and the history cursor with its phase and kind.'),
    ch('🎛', 'retained-vcs-state-and-begin', 'retained_vcs_state_and_begin', [(428, 706)],
       'Retained VCS state and begin: the operation slot, the fixed owners, the document record, the retained VCS owner and the constructors, accessors and `begin_*` entry points of every authoring operation.'),
    ch('📄', 'poll-pages-cancel-and-fault', 'poll_pages_cancel_and_fault', [(708, 851)],
       'Poll, pages, cancel and fault of the Flow VCS: polling, page take, resume, retry and acknowledge, cancellation, fault injection and handle rediscovery.'),
    ch('🏁', 'close-protocol', 'close_protocol', [(853, 1020)],
       'Close protocol of the Flow VCS: the operation and retired owner close demands and steps, the close phase and the terminal state.'),
    ch('🎟', 'admission-and-slots', 'admission_and_slots', [(1022, 1110)],
       'Admission and slots of the Flow VCS: preflight, admission, credit charge and hand back and the operation slot lookup.'),
    ch('🪜', 'action-cursor-impl', 'action_cursor_impl', [(1113, 1339)],
       'Action cursor impl of the Flow VCS: stepping the action cursor through the history load, transfer, visibility and page publication phases and the cursor helper fns.'),
    ch('🧮', 'cursor-steps', 'cursor_steps', [(1341, 1783)],
       'Cursor steps of the Flow VCS: the scan, shift, mutation, document replacement and rollback steps of the action cursor.'),
]
SPEC = dict(orig='vcs.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='vcs.rs'), pre=pre, pre_extra=['}', H] * len(CUTS))
