from specs.lib import *

CH = [
    ch('🎲', 'seeded-generators', 'seeded_generators', [(15, 270)],
       'Seeded generators of the protocol laws: the splitmix64 state, the word, text, identifier and timestamp draws and the history log and operation DAG generators.', glob=False),
    ch('📼', 'history-log-laws', 'history_log_laws', [(274, 499)],
       'History log laws of the protocol laws: the log writer and reader helpers, the encode and decode identity, canonical stability, zero-copy, tamper detection, recovery truncation and compaction laws.'),
    ch('🧮', 'operation-and-mutation-laws', 'operation_and_mutation_laws', [(501, 764)],
       'Operation text and mutation laws of the protocol laws: the operation text round trip and the diff absorption, inverse and inverse-sum laws with their cold variants and the diff algebra inverse law.', glob=False),
    ch('🔁', 'dag-convergence-and-frame-laws', 'dag_convergence_and_frame_laws', [(766, 913)],
       'DAG convergence and frame laws of the protocol laws: the shuffle and retire helpers, the operation DAG convergence law, the wire and channel frame samples and the outcome laws.'),
    ch('🏛', 'policy-merge-and-corpus-laws', 'policy_merge_and_corpus_laws', [(917, 1092)],
       'Policy, merge and corpus laws of the protocol laws: the policy matrix, merge convergence, modify-versus-delete, chronological determinism, quarantine, ledger replay, conflict round trip and channel frame corpus laws.'),
]

SPEC = dict(
    orig='plw.rs',
    file=PLW + '/🦀️.rs',
    children=CH,
    external=mounted(M + '/📡️spr/🦀️.rs'),
)
