from specs.lib import *
RC = OD + '/🔌️plugin/🧵️retained-command'
CH = [
    ch('🔖', 'checkpoint-and-work-contract', 'checkpoint_and_work_contract', [(39, 276)],
       'Checkpoint and work contract of the retained command: the checkpoint size caps and codec, the reducer and extent types, the work step, inputs and retained capacity, the command work trait and its bounded implementation.'),
    ch('🧳', 'payload-phase-and-job-record', 'payload_phase_and_job_record', [(280, 387)],
       'Payload, phase and job record of the retained command: the retained command inputs and payload, the phase and publication enums and the retained command job record.'),
    ch('🏁', 'job-admission-close-and-checkpoint', 'job_admission_close_and_checkpoint', [(389, 659)],
       'Job admission, close and checkpoint of the retained command job: the controlled close step, the close demands, the admission from the wire and payload, the receive step, the publication retirement, the checkpoint and the restore.'),
    ch('🩺', 'reducer-faults-and-advance', 'reducer_faults_and_advance', [(661, 853)],
       'Reducer faults and advance of the retained command job: the reducer fault detail and the advance of one original command turn.'),
    ch('⏯', 'interactive-step-and-allocation-close', 'interactive_step_and_allocation_close', [(855, 1010)],
       'Interactive step and allocation close of the retained command job: the interactive job implementation with its step, outcome and close ladder and the raw allocation close oracle.'),
]
SPEC = dict(orig='rc.rs', file=RC + '/🦀️.rs', children=CH, external=mounted(RC + '/🦀️.rs', orig='rc.rs'))
