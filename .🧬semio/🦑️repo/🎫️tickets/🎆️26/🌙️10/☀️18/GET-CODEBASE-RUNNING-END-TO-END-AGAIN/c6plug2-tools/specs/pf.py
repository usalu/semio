from specs.lib import *
F = FILES['pf']
CH = [
    ch('📏', 'limits-vocabulary-and-placement', 'limits_vocabulary_and_placement', [(28, 395)],
       'Limits, vocabulary and placement: the fill run limits, the run stage, counter and reason vocabulary, the checkpoint and the placement helpers.'),
    ch('🔬', 'capture-cursor', 'capture_cursor', [(399, 568)],
       'Capture cursor: the artifact capture stages and fields and the fill capture cursor with its fault codes.'),
    ch('🧲', 'fill-capture', 'fill_capture', [(571, 1036)],
       'Fill capture: the fill capture and its streaming of the document into the board fill ingress.'),
    ch('⏯', 'run-job-state-and-search', 'run_job_state_and_search', [(1040, 1428)],
       'Run job state and search: the fill search and owed state, the run owners and replay, the fill run job and its search stepping.'),
    ch('🏃', 'run-job-stepping-and-finalize', 'run_job_stepping_and_finalize', [(1430, 1632)],
       'Run job stepping and finalize: the remaining operations of the fill run job and its interactive job implementation.'),
    ch('🔍', 'revalidate-job', 'revalidate_job', [(1636, 1870)],
       'Revalidate job: the revalidate owners and job that re-test the provisional placements against the head.'),
]
SPEC = dict(orig='pf.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='pf.rs'), keep_empty=('\U0001F9EA️Tests',))
