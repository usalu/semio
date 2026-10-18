from specs.lib import *
RP = PL + '/⚛️reactor/🩹️patches'
H = 'impl PatchTracker {'
B = [526, 772, 911]
CH = [
    ch('🎰', 'surface-slots-and-reconcile-grant', 'surface_slots_and_reconcile_grant', [(11, 340)],
       'Surface slots and reconcile grant: the ready patch capacity, the surface, terminal, rejected, ready and unadmitted slots, the mounted reconcile grant and owner, the closing instance, the tracker state and the patch tracker struct.'),
    ch('🚪', 'tracker-admission-and-reservation', 'tracker_admission_and_reservation', [(342, 526)],
       'Tracker admission and reservation: the patch tracker constructor, the begin, defer and retain admission, the pending render and deferred ready takes and the mounted output reservation with its refusals.'),
    ch('🏃', 'tracker-drive-and-work-predicates', 'tracker_drive_and_work_predicates', [(528, 775), (1252, 1312), (1318, 1354)],
       'Tracker drive and work predicates: the drive of one surface, the debug state, the publishable, drivable and general work predicates, the render fault take, the job drive helper and the generation commit.'),
    ch('📬', 'ready-patches-and-publication-acks', 'ready_patches_and_publication_acks', [(777, 917), (1180, 1222)],
       'Ready patches and publication acks: the ready patch keys and takes, the rejection marks, the published native resource loans, the publication ack and the oldest admissible ready output.'),
    ch('🔚', 'instance-close-and-terminals', 'instance_close_and_terminals', [(919, 1178), (1224, 1250)],
       'Instance close and terminals: the close instance preflight, reservation, activation and release, the close step, the terminal and rejected takes and resumes and the stranded output closes.'),
]
SPEC = dict(orig='fpa.rs', file=RP + '/🦀️.rs', children=CH, wiring='block', external=mounted(RP + '/🦀️.rs', orig='fpa.rs'),
            pre=split_impl(H, B), expect_extra=split_extra(H, B))
