from specs.lib import *
JW = M + '/🧵️job/👷️worker'
CH = [
    ch('🎫', 'worker-job-vocabulary', 'worker_job_vocabulary', [(39, 147)],
       'Worker job vocabulary: the worker job ticket, contention, demand, submit, poll, take and close phase records, the batch session and the mounted session pump fault.'),
    ch('🧰', 'mounted-and-batch-sessions', 'mounted_and_batch_sessions', [(149, 466)],
       'Mounted and batch sessions: the mounted worker job session and the batch job session operations over the interactive job.'),
    ch('🕳', 'retirement-slots-and-nodes', 'retirement_slots_and_nodes', [(468, 537), (649, 687)],
       'Retirement slots and nodes: the session state constants, the worker job retirement slots, wake and header, the retirement pump and the retirement node phase and pump.'),
    ch('🛂', 'session-inner-and-admission', 'session_inner_and_admission', [(539, 647), (689, 769)],
       'Session inner and admission: the shared session inner custody, the session handle, the admission rejection and the submission records.'),
    ch('🧵', 'worker-job-session', 'worker_job_session', [(771, 1117)],
       'Worker job session: the begin close and close step helpers and the worker job session operations with their drop.'),
    ch('🏁', 'outcome-and-rejection', 'outcome_and_rejection', [(1119, 1238)],
       'Outcome and rejection: the worker job outcome and rejection records with their drops and the rejection code and kind mapping.'),
]
SPEC = dict(orig='jw.rs', file=JW + '/🦀️.rs', children=CH, wiring='block', external=mounted(JW + '/🦀️.rs', orig='jw.rs'), crate_files=[])
