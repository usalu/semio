from specs.lib import *
TRI = PL + '/⏯️tool-run/🧵️instance'
_TXT = open(os.path.join(T, 'orig', 'tri.rs'), encoding='utf-8').read()
CUTS = [283, 481, 659, 863]
CH = [
    ch('🚀', 'action-dispatch-and-start', 'action_dispatch_and_start', [(42, 283)],
       'Action dispatch and start of the instance tool run turns: the dispatched and applied tool run actions, the selected action, the run start, driver events, the job port drain, the dirty marks and the trace lane injection.'),
    ch('👀', 'generation-watch-and-turn-driver', 'generation_watch_and_turn_driver', [(285, 484)],
       'Generation watch and turn driver of the instance tool run turns: the store and settings generation watch, the per-turn driver, the retirement loop, the selected run drive and the refold.'),
    ch('🎟', 'job-admission-and-finalize-turn', 'job_admission_and_finalize_turn', [(486, 665)],
       'Job admission and finalize turn of the instance tool run turns: the job admission, fault, step and completion, the revalidation completion and the finalize turn.'),
    ch('📤', 'publication-and-member-refresh', 'publication_and_member_refresh', [(667, 872)],
       'Publication and member refresh of the instance tool run turns: the publication rejection, the run publication, the finalize owner and publication close and the composed member refresh, settle and publication.'),
    ch('🧹', 'member-retirement-and-close', 'member_retirement_and_close', [(874, 988)],
       'Member retirement and close of the instance tool run turns: the member retire step and the retire, entry close and close steps of the tool run ledger.'),
    ch('🔌', 'ref-reads-and-extension', 'ref_reads_and_extension', [(990, 1105)],
       'Ref reads and extension of the instance tool run turns: the read-only pending work view, the busy fault and the tool run extension that registers the turns with the instance.'),
]
SPEC = dict(orig='tri.rs', file=TRI + '/🦀️.rs', children=CH, wiring='each', pre=cut_impl(42, CUTS), pre_extra=cut_extra(42, CUTS, _TXT), external=mounted(TRI + '/🦀️.rs', orig='tri.rs'))
