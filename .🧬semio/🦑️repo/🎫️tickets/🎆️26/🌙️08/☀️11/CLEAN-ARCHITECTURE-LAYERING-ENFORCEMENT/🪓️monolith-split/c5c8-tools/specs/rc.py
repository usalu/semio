from specs.lib import *
RC = OD + '/🔌️plugin/🧵️retained-command'
CH = [
    ch('🔖', 'checkpoint-and-work-contract', 'checkpoint_and_work_contract', [(39, 276)],
       'Checkpoint and work contract of the retained command: the checkpoint size caps and codec, the reducer and extent types, the work step, inputs and retained capacity, the command work trait and its bounded implementation.'),
    ch('🧵', 'retained-command-job', 'retained_command_job', [(280, 930)],
       'Retained command job: the retained command inputs and payload, the phase and publication enums, the job record, its admission, close, checkpoint, restore and reducer advance and the interactive job implementation.'),
    ch('🧯', 'raw-allocation-close-oracle', 'raw_allocation_close_oracle', [(933, 1010)],
       'Raw allocation close oracle of the retained command job: the testing function that closes a raw allocation through the original close ladder.',
       cfg_mod='#[cfg(any(test, feature = "artifact-app-testing"))]'),
]
SPEC = dict(orig='rc.rs', file=RC + '/🦀️.rs', children=CH, external=mounted(RC + '/🦀️.rs', orig='rc.rs'))
