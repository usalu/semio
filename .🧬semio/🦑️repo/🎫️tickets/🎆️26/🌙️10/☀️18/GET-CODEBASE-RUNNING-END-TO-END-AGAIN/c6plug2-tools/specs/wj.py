from specs.lib import *
F = FILES['wj']
H = 'impl<T: Topology + Clone> WfcJob<T> {'
B = [696, 974]
def pre(t):
    lines = t.split('\n')
    assert lines[465] == H
    for b in sorted(B, reverse=True):
        assert lines[b - 1].startswith('    ') and lines[b - 2].strip() == ''
        lines[b - 1:b - 1] = ['}', '', H]
    return '\n'.join(lines)
CH = [
    ch('🧭', 'protocol-and-determinism', 'protocol_and_determinism', [(23, 184)],
       'Protocol and determinism: the preview and checkpoint limits, the fault, stage, preview and commit types, the sampler, search pulse and job config and the job random number generator.', glob=False),
    ch('📚', 'persistent-state-and-materialization', 'persistent_state_and_materialization', [(187, 442)],
       'Persistent state and materialization: the removal, decision and entropy records, the init, choice and arc cursors, the search state and the bounded checkpoint and commit materialization.'),
    ch('🧩', 'job-struct-and-preview', 'job_struct_and_preview', [(447, 696)],
       'Job struct and preview: the WFC job, its construction from a model or a checkpoint, the preview, the commit and the single step advance.'),
    ch('🔁', 'job-queues-patterns-and-initialize', 'job_queues_patterns_and_initialize', [(698, 977)],
       'Job queues, patterns and initialize: the propagation queue and entropy heap, the visible cell drain, the pattern removal and addition, the initialization and the choice steps.'),
    ch('🌊', 'job-propagate-backtrack-and-checkpoint', 'job_propagate_backtrack_and_checkpoint', [(979, 1287)],
       'Job propagate, backtrack and checkpoint: the propagation, the contradiction detection, the backtracking, the checkpoint and commit materialization steps and the preview emission.'),
    ch('🔄', 'restore-job-state-and-resume', 'restore_job_state_and_resume', [(1289, 1687)],
       'Restore job state and resume: the run outcome, the restore stage, header and preview and the restore job with its resumption steps.'),
    ch('⏯', 'restore-job-run-and-interactive-job', 'restore_job_run_and_interactive_job', [(1689, 1920)],
       'Restore job run and interactive job: the restore run outcome, the restore job driving and its interactive job implementation.'),
    ch('🚦', 'job-run-and-interactive-job', 'job_run_and_interactive_job', [(1923, 2176)],
       'Job run and interactive job: the bounded search loop run, the interactive job implementation with its close and the remaining job operations.'),
]
SPEC = dict(orig='wj.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='wj.rs'), pre=pre, expect_extra=['}'] * len(B) + [H] * len(B), keep_empty=empties('wj'))
