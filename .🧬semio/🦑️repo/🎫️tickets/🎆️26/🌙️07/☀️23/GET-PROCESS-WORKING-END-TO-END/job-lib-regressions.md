# Job Lib Regressions

## Result

`semio-framework-job --features testkit --no-fail-fast` is green on the current tree in a fresh build dir (`🗑️generated/native-j`, script `job-cargo.sh`, 100 tests, every target ok).

- `genuine_original_page_publication_keeps_the_exact_output_before_receipt_refusal`: ok
- `genuine_worker_preparation_uses_the_admitting_ledger_and_one_current_fuel`: ok
- `original-caller-capture`, `original-child-source`: ok (no heap vs receipt mismatch)

I could not reproduce the failures. The birth-ledger settle (`original_lifecycle_step`, `original_lifecycle_effect_step_at`), the receiving-refund and the `NativeReceivingEffect` single-relay changes are all consistent with these tests. A fresh build starts from the current sources, so the earlier red runs came from a stale or interleaved tree (concurrent edits and debug prints that allocated inside observed windows) that has since been cleaned up. No product change was needed and no assertion or stock was touched.

## Debug cleanup

- Grep of the whole `🧵️job` module for `[DEBUG]`, `debug_tag`, `debug_turn`, `crate_dbg`: one leftover, in `retained-charge.rs` (a timing `eprintln!` after the measured loop). Removed it together with the timing code (`Instant`, `elapsed`) it alone used.
- Other `[DEBUG]` prints remain outside the job module, owned by other agents (display tests, artifact-app-laws construction, plugin builder contract replacement fixtures, typed-operation-unit, store replay replacement tests, wgpu layouts dispatch). Not touched.

## Files changed

- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/retained-charge.rs`
- ticket: `job-cargo.sh` (build script), `job-lib-regressions.md`

## Cleanup

Idle test binaries over 20 MB removed from `🗑️generated/native-j`; that dir and `🗑️generated/job-t1.txt` are tool output to delete at ticket close.
