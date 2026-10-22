# Result rust-reds-a (executor rust-reds-a, wave 1022a)

Tools: `C/rust-reds-a-tools/run.sh` (one detached `bun repo cargo --store=shared --configuration=prototype -- test --manifest-path <ws> --no-fail-fast --message-format=short`, retried on capacity/lease/import-capture refusals). Logs: `C/🗑️generated/rust-reds-a-logs/`.

## 1. os-history `retained_native_history_fold_obeys_the_neutral_law`
Diagnosis (instrumented turn trace, [DEBUG] removed afterwards): not a missing wake (`FoldWake` is a no-op, the job polls its coroutine itself); `completed` froze at 2 from turn 4 to turn 100000 at work grant 1 / byte grant 1. Root cause: the original index text quote `text_slice_demand` (`fold/🏗️birth/🗂️index/🦀️.rs`) priced `2 * min(remaining, body / 2)` compared bytes; with a 1-byte page `body / 2 == 0`, so the quote covered only the comparison metadata while `HistoryFoldIndexInsert::compare_original_text` needs `metadata + 2` to advance one byte (`grant - metadata < 2` -> no progress). Every holder of that quote (FoldIndexRead / FoldIndexInsert) was admitted by its own gate and then made zero progress forever.
Fix: quote at least one compared byte: `2 * remaining.min((body / 2).max(1))`.

## 2. os-store-engine `original_derived_semantic_task_admits_copy_and_capacity_as_independent_currencies`
Cancellation tail failed with `original retirement requires its exact original recipient` because the test's `NativeDecodeControl::new` carried no retirement recipient. Fix: test installs an empty `NativeDecodeRetirementRecipient` (`install_retirement_recipient`) on the receiving control before the first step.

## Verification
- `sh C/rust-reds-a-tools/run.sh Cargo.toml C/🗑️generated/rust-reds-a-logs/h3.log -p semio-framework-os-history --lib -- retained_native_history_fold_obeys --nocapture` -> `test ... ok` (1 passed). Before the fix (h1.log) `completed` froze at 2 from turn 4 to turn 100000. The run included the [DEBUG] turn trace in the test; the trace is removed afterwards (the edit that removes it is a pure revert to the previous line, not re-run).
- NOT re-run: the rest of the os-history lib (57 other tests were green in rust-rebase `hist4`) and the replication fold crate tests after the quote change (`(body/2).max(1)` only raises the quote for body < 2).
- os-store-engine derived-operation test: edit WRITTEN BUT UNVERIFIED (no cargo run, wound down by the coordinator because of the 90 % usage quota). Next: `bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-os-store-engine --lib --message-format=short`.

## Changed files
- `🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🏗️birth/🗂️index/🦀️.rs` (`text_slice_demand`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧪️tests/🦀️.rs` (recipient installed)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️tests/🔬️unit/⏳️native-fold-law/🦀️.rs` (temporary trace added and removed; the pre-existing [DEBUG] demand/grant suffix of the assertion was also already absent in the working tree)
No process of mine is left running. Infra note: a peer's `cargo-workspace` provider produced `Uncaptured implementation import refused` for ~1 h (retried automatically, cleared by itself).
