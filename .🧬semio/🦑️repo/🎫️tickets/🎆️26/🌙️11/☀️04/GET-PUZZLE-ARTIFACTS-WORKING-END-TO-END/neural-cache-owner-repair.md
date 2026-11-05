# Neural Cache Cursor Owner Binding

Root request 11101 authoritatively ended with process exit 1 / Cargo 101 before any physical2d test. Its unique sqlite2d-5.log retains E0432 at neural engine cache/read.rs line3: protocol::causal::transition::HistoryFoldIndexRead was unresolved in the compiled surface. Root counted 654 compilation entries before this frontier. This is a compiler failure, not a native IO/JSON result.

## Actual Current API

HistoryFoldIndexRead is a concrete Default/Clone/Copy cursor struct, not a trait. Its definition is in the replication history-index owner's index/read source, and the actual index crate publicly reexports it. Its matched, next_demands and step methods accept the same canonical HistoryFoldIndex type used by NeuralCache entries. The cache's current calls match these methods; no algorithm/receipt/custody change was needed.

The currently read replication aggregation wrapper also publicly reexports the cursor, including its causal transition surface. That source observation does not explain the retained compiler result by itself. No cache, immutable-source, parser-default or historical-wrapper fault is assumed, and no concurrent wrapper exports were changed.

## Exact Narrow Repair

Only two production files changed:

- The cache/read import directly names semio_framework_replication_history_index::HistoryFoldIndexRead.
- The actual neural budgeted evaluation Cargo manifest declares the real first-party semio-framework-replication-history-index path dependency.

The explicit relative endpoint resolves to the canonical history-index manifest, whose package name and lib path were checked; its index root exports the same struct. This is not a renamed alias, duplicated type, compatibility facade or external library. The existing aggregation dependency remains because other current evaluation imports still use it. All cursor fields, comparisons, progress/work accounting, source identity, dictionary/epoch handling, retirement, tests and Source bounds are unchanged. The changed Rust import adds exactly 13 bytes and no line/input to that source; the real next admission remains authoritative for the full unit.

Low independently reviewed the direct endpoint, public export and same concrete cursor identity green. Root then launched its sixth physical2d request, owned session76254, with unique sqlite2d-6.log/backtrace. That request is the sole native lane as last reported by root. No integration compiler or producer is running/queued; the fifth5d unfiltered command remains prepared for a later explicit handoff.

The repair is source-reviewed. Root's sixth actual compiler/test result is still required, and this binding change alone does not establish neural cursor runtime correctness or any Puzzle acceptance result.
