from specs.lib import *
RT = PL + '/⚛️reactor/🔄️turn'
CH = [
    ch('🔌', 'extension-invocation-and-command-ingress', 'extension_invocation_and_command_ingress', [(31, 79), (86, 220), (226, 232)],
       'Extension invocation and command ingress of the reactor turn: the retained extension invocation with its receive and advance steps and the retained command ingress with its owner cursor, retirement, park and close steps.'),
    ch('🔦', 'more-work-sources-and-tracing', 'more_work_sources_and_tracing', [(251, 420)],
       'More-work sources and tracing of the reactor turn: the settled and busy more-work sources, the late UI retirement, the turn event note and the guest memory and phase retention traces.'),
    ch('🎯', 'dirty-poll-surfaces', 'dirty_poll_surfaces', [(422, 523), (530, 570)],
       'Dirty poll surfaces of the reactor turn: the dirty render and intent capacities, the dirty poll owners, the background and document surface collection and the arena-starved redirty.'),
    ch('⏱', 'turn-execution-and-poll-entry', 'turn_execution_and_poll_entry', [(573, 588), (594, 691), (1656, 1704)],
       'Turn execution and poll entry of the reactor turn: the guest turn execution witness, the kernel poll entry points, the boxed plugin exchange and the turn patch budget and page.'),
    ch('🌀', 'poll-kernel-turn', 'poll_turn', [(693, 1654)],
       'Poll kernel turn of the reactor turn: the one reactor turn that polls the kernel, routes events, drives retirement, reconciliation and publication and fills the turn patch.'),
    ch('🧊', 'cold-pair-loads-and-process-pool', 'cold_pair_loads_and_process_pool', [(1706, 1831)],
       'Cold pair loads and process pool of the reactor turn: the cold pair and restored document load steps, the patch page filler and the process worker pool pump.'),
    ch('🧯', 'retirement-drivers-and-turn-limits', 'retirement_drivers_and_turn_limits', [(1847, 1964)],
       'Retirement drivers and turn limits of the reactor turn: the retire-until-complete drivers and the close, patch, executor hold and measured guest turn limits.', glob=False),
    ch('🩹', 'reconcile-drive-and-publication-receipts', 'reconcile_drive_and_publication_receipts', [(1965, 2102), (2105, 2209)],
       'Reconcile drive and publication receipts of the reactor turn: the more-work drive steps and budget, the reconcile drive, the live patch receipt, the original publication receipts and the exchange and app frame routing.'),
    ch('📬', 'task-resumes-and-wire-effects', 'task_resumes_and_wire_effects', [(2211, 2318)],
       'Task resumes and wire effects of the reactor turn: the drained task resumes and the decoded wire effects, replay shell commands, file opens, app events and admitted effects.'),
]
SPEC = dict(orig='rt.rs', file=RT + '/🦀️.rs', children=CH, wiring='each', external=mounted(RT + '/🦀️.rs', orig='rt.rs'))
