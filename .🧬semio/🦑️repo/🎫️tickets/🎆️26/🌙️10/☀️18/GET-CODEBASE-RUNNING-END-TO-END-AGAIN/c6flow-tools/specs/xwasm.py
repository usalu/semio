from specs.lib import *
F = FLOW + '/🧩️extensions/🕸️wasm/🦀️.rs'
CH = [
    ch('🔎', 'inference-dependency-and-evaluate-json', 'inference_dependency_and_evaluate_json', [(9, 71)],
       'Inference dependency and evaluate JSON of the wasm extension glue: the canonical geometry inference ancestry and the JSON operator and function evaluation.'),
    ch('🎫', 'evaluate-request-admission-and-drive', 'evaluate_request_admission_and_drive', [(79, 292)],
       'Evaluate request, admission and drive of the wasm extension glue: the evaluate request and external result, the continuation identity, the request admission, the step budget constants and the cold envelope drive.'),
    ch('🧯', 'reply-failure-and-child-demands', 'reply_failure_and_child_demands', [(294, 360)],
       'Reply, failure and child demands of the wasm extension glue: the evaluation reply, the operator input parse, the child demands and close and the evaluation failure with its conversions.'),
    ch('🧰', 'input-preparation-and-envelope-source', 'input_preparation_and_envelope_source', [(362, 569)],
       'Input preparation and envelope source of the wasm extension glue: the retained input preparation and its retirement, the evaluation step, the job capacity and the envelope source projection.'),
    ch('🗃', 'evaluation-output-and-retained-evaluation', 'evaluation_output_and_retained_evaluation', [(571, 866)],
       'Evaluation output and retained evaluation of the wasm extension glue: the evaluation output, the retained evaluation owner with its steps and its retirement cursor.'),
    ch('⏱', 'evaluation-jobs-and-cancellation', 'evaluation_jobs_and_cancellation', [(868, 971)],
       'Evaluation jobs and cancellation of the wasm extension glue: the job registry, dependency identity, cancellation, retirement demands and close step and the progress probe.'),
    ch('🧩', 'extension-resources-topic-and-command', 'extension_resources_topic_and_command', [(973, 1097)],
       'Extension resources, topic and command of the wasm extension glue: the extension evaluation resources and their plugin owner impl, the topic contribution and the command JSON.'),
]
SPEC = dict(orig='xwasm.rs', nest=True, file=F, children=CH, wiring='block', external=mounted(F, orig='xwasm.rs'))
