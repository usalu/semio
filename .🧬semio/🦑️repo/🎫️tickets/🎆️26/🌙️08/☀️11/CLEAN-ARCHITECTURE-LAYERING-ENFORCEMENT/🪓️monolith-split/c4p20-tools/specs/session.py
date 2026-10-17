from specs.lib import *

SPEC = dict(
    orig='session.rs.txt',
    file=E + '/🧵️reconstruction-session/🦀️.rs',
    nest=False,
    external=[orig('sessionunit')],
    children=[
        ch('🧭', 'run-contract', 'run_contract', [(61, 386)],
           'Run contract of the reconstruction session: the job kinds and trace lanes, the stage, counter and reason vocabulary and the tool-run definition.'),
        ch('🔑', 'trace-checkpoint-and-ingestion', 'trace_checkpoint_and_ingestion', [(390, 530)],
           'Trace keys, checkpoint and ingestion of the reconstruction session: the per-stage trace keys, the run checkpoint, the run inputs and the bounded frame ingestion with its sharpness.'),
        ch('🎁', 'products-and-outbox', 'products_and_outbox', [(534, 919)],
           'Products and outbox of the reconstruction session: the content leaves, the resumable product preparation and the outbox that publishes the outcome in turns.'),
        ch('🏃', 'run-job', 'run_job', [(923, 1484)],
           'Run job of the reconstruction session: the run phases and remnant and the interactive reconstruction run job that advances the engine under the budget.'),
        ch('🔍', 'revalidate-job', 'revalidate_job', [(1488, 1606)],
           'Revalidate job of the reconstruction session: the interactive job that recomputes the quality report of a stored reconstruction.'),
    ],
)
