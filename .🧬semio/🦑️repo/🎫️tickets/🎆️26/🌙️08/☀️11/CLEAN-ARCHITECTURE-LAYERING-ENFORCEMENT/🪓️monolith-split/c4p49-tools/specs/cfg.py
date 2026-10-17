from specs.lib import *

CH = [
    ch('⏱', 'schedules', 'schedules', [(6, 190)],
       'Schedules of the sampling configuration: the schedule input, the schedule curves with their evaluation and the schedule progress.'),
    ch('🎛', 'sampling-method-and-processor-specs', 'sampling_method_and_processor_specs', [(194, 425)],
       'Sampling method and spec vocabulary: the tie break, the multinomial strategy, the sampling method, the penalty scope, the processor and constraint specs and the stop, forced, logprobs and diagnostics specs.'),
    ch('⚙', 'sampling-config', 'sampling_config', [(427, 636)],
       'Sampling config: the configuration record, its defaults and its validation, accessors and update methods.'),
    ch('🧾', 'enum-json-and-spec-validation', 'enum_json_and_spec_validation', [(638, 755)],
       'Enum JSON and spec validation: the processor spec validation and the JSON forms of the sampling method, tie break, multinomial strategy and penalty scope.'),
    ch('🧬', 'processor-spec-json', 'processor_spec_json', [(757, 946)],
       'Processor spec JSON: the JSON form of every processor spec and its parser.'),
    ch('🏗', 'sampling-config-builder', 'sampling_config_builder', [(948, 1040)],
       'Sampling config builder: the builder that assembles and validates a sampling config.'),
    ch('📊', 'step-results', 'step_results', [(1044, 1087)],
       'Step results of a sampling step: the candidate, the top logprobs, the step diagnostics and the sampling result.'),
]

SPEC = dict(
    orig='cfg.rs',
    file=CFG + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[CFG + '/🧪️tests/🔬️unit/🦀️.rs'],
)
