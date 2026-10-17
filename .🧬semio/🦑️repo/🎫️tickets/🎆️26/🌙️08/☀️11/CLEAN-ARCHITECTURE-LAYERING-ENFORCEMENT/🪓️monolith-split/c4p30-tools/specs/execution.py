from . import _ext
F = '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs'
SPEC = dict(
    orig='jack-execution.rs.txt',
    file=F,
    wiring='each',
    children=[
        dict(folder='⚖️owned-bytes', mod='owned_bytes',
             ranges=[(13, 165)],
             doc='//! ⚖️ Jack query owned bytes: the result accounting limits, the per-value owned byte measures and the closing owner of the query graph metadata.'),
        dict(folder='🧱️execution-preparation', mod='execution_preparation',
             ranges=[(167, 322)],
             doc='//! 🧱️ Jack query preparation: the bounded preparation steps and the `QueryExecutionPreparation` state machine.'),
        dict(folder='🔁️pattern-delete-return', mod='pattern_delete_return',
             ranges=[(324, 585)],
             doc='//! 🔁️ Jack query clause executors: the pattern match, delete and return phases of one query execution.'),
        dict(folder='▶️query-execution', mod='query_execution',
             ranges=[(587, 1005)],
             doc='//! ▶️ Jack query execution: the `QueryExecution` driver that runs the clauses of a prepared query under the step budget.'),
    ],
    external=_ext.mounted(F),
)
