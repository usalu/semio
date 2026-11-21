# Paid JSON UTF-8 validation batches

The actual 480-byte Note `instanceOpen` Actor Input fixture completed in a pinned optimized native relink before and after batching. The test admits the same typed Note app and actor on the bounded and larger construction windows, checks the one-byte-short resident quote, retains the same dispatch, and checks physical allocation and release against the caller's receipt.

| Metric across the fixture's bounded and larger windows | One scalar per action | Batched validation |
| --- | ---: | ---: |
| Parser actions | 3,180 | 2,266 |
| Parser work units | 3,180 | 3,180 |
| Parser actions ending at grammar byte position zero | 964 | 50 |
| Parser work units ending at grammar byte position zero | 964 | 964 |

The batch removed 914 parser actions, or 28.74% of the measured parser actions. The byte-position-zero count includes initial grammar setup after the 480-byte UTF-8 scan. The scanner advances its own validation position without advancing the separate grammar position.

The grammar cursor quotes a guaranteed UTF-8 prefix using the unvalidated byte extent and the four-byte maximum scalar width. This needs no extra read or scan. The receiving turn caps the quote at remaining work, remaining items and 256 scalars. The original parser loop checks ownership, cancellation and progress for each scalar and reports actual work and physical progress through the original Source action. The quote stops at or before the end of UTF-8 validation, so it authorizes the same depth-one frontier and a single receiver metadata update. Grammar construction remains one frontier per action. An in-flight parser keeps its exact quote and bound grant, and a produced value advances to binding only when the original action has settled.

The work and item receipts still charge each scalar. The optimization reduces original action and metadata handling. VM fuel and the live Note and WFC caller budgets require validation in the separate WASI lane.

The probe derives the exact dependencies from the existing Actor Input fingerprint. It uses atomic counters inside instrumented native parser actions because allocation in the measurement would perturb the physical receipt test. The malformed UTF-8 fixture uses the multilingual shared JSON case, inserts one invalid byte after four valid bytes, and checks the serde JSON refusal, actual work and progress, syntax custody and paid retirement.

All nine independent Ajv Actor Input cases passed. The optimized native unique JSON owner suite passed all four tests with the 8 MiB stack, including the invalid UTF-8 batch and cleanup. The invalid batch reported the four completed validation units plus the refused unit. The original malformed or structurally invalid cases and every configured cancellation prefix also passed.
