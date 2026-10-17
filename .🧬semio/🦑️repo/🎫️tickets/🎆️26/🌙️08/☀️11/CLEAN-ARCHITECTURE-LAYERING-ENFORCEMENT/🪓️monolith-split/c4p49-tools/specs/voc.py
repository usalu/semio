from specs.lib import *

CH = [
    ch('🆔', 'ids', 'ids', [(8, 85)],
       'Identifiers of the sampling vocabulary: the token id, the sequence id and the step index.', glob=False),
    ch('⚠', 'errors-fallbacks-and-limits', 'errors_fallbacks_and_limits', [(89, 276)],
       'Errors, fallbacks and limits of sampling: the sampling error, the finish reason, the error mode, the fallback action and its resolution and the sampling limits.'),
    ch('🧾', 'json-value-and-parser', 'json_value_and_parser', [(281, 522)],
       'JSON value and parser of the sampling vocabulary: the JSON value and the strict JSON parser.'),
    ch('📤', 'json-writer-and-utf8', 'json_writer_and_utf8', [(524, 647)],
       'JSON writer and UTF-8 of the sampling vocabulary: the JSON writer and the UTF-8 sequence length and status.'),
    ch('🧮', 'numerics', 'numerics', [(652, 824)],
       'Numerics of sampling: the logit sanitation, the Kahan sum, the live softmax, log-sum-exp, entropy, candidate count, truncation mass and distribution health.'),
    ch('🎯', 'selection-and-cumulative-distribution', 'selection_and_cumulative_distribution', [(826, 898), (1362, 1373)],
       'Selection and cumulative distribution of sampling: the partial top-k selection, the descending quickselect, the cumulative distribution binary search and the cumulative sums of probabilities.'),
    ch('🧱', 'token-bitset', 'token_bitset', [(902, 1022)],
       'Token bitset: the bitset over token ids with its set algebra.'),
    ch('🎲', 'random-streams', 'random_streams', [(1026, 1240)],
       'Random streams of sampling: the stream purpose and key, the seeded stream derivation, the random source trait with the counter and xoshiro sources and their closed enum.'),
    ch('📖', 'vocabulary-and-text-adapters', 'vocabulary_and_text_adapters', [(1244, 1359)],
       'Vocabulary and text adapters: the token vocabulary, the token text adapter trait, the slice adapter and the closed adapter enum.'),
]

SPEC = dict(
    orig='voc.rs',
    file=VOC + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[VOC + '/🧪️tests/🔬️unit/🦀️.rs'],
)
