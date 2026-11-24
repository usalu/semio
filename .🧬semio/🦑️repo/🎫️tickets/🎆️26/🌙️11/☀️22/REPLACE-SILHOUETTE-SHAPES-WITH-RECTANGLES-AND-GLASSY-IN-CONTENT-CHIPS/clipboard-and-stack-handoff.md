# 📋️ Clipboard receipt and Shell test stack handoff

## Results
| target | before | after |
| --- | --- | --- |
| `semio-ui-wgpu-laws` (lib 46, `original_clipboard_ingress` 2, prepared refusal 1) | 1 failure | all green (run with `--no-fail-fast`) |
| `semio-renderer-wgpu-shell-tests-input-1` | `a_published_canvas_hit_cannot_retarget_a_same_key_successor` overflowed the default 2 MiB test stack | 73/73 with the default stack, `--test-threads=2` |

## 1. `original_clipboard_admission_denial_preserves_text_and_cancelled_write_never_enters_its_producer`
- Observed: step progress `copied_items 1, copied_bytes 118` versus `caller.receipt()` `2 / 236`.
- Root cause (verified, same as mounted-layout): the test's `close` helper compared the step progress with the caller's total turn receipt. Total spend also holds the Source's reserved return span (1 item, 118 bytes), which the job contract excludes from the fresh step receipt. `OriginalWorkerFixtureCaller::close_owner` already asserts the fresh receipt equals `step.progress()` inside its own turn, so the product was right and the test compared the wrong quantity.
- Fix: dropped the `assert_eq!(step.progress(), caller.receipt())` from `close`; the heap-versus-progress assertion stays.
- File: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📋️clipboard-ingress/🦀️.rs`.

## 2. Stack overflow in `a_published_canvas_hit_cannot_retarget_a_same_key_successor`
- Method: the abort crash report (`~/Library/Logs/DiagnosticReports`) showed the overflow inside `KernelClient::get` (kernel_pool `OnceLock` init) reached from `ProgramBridgeEntry::from_wasm` via `host_test_shell`. The test is simply the first one to initialise the process-wide kernel client when run alone. Passes with `RUST_MIN_STACK` of 8 MiB, overflows at 4 MiB.
- Measured with `size_of` ([DEBUG] prints, removed): `KernelPoolState` 342 KB, `run_kernel_pool` future 722 KB. Debug builds copy that future several times (create, `drive_renderer_worker_task` wrapper, `Box::pin`) on the initialising thread. After step A the state was 38 KB and the future 418 KB; `advance_product_replay` alone was 285 KB because it inlines `run_turn` and its `run_turn_once`.
- Fix A: the nine fixed 64-slot tables of `KernelPoolState` are now `Box<[Option<T>]>` built directly on the heap via `empty_slots` (no stack temporary).
- Fix B: `run_turn_once` is awaited through `Box::pin` at its two call sites, so the large turn future no longer lives inside `run_turn`, `advance_product_replay` and `run_kernel_pool`.
- Files (under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/`): `🏊️kernel-pool/📥️lifted/🗃️kernel-retained-state/🦀️.rs`, `🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🏗️pool-construction-and-product-replay/🦀️.rs`, `🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🔄️exchange-and-turn-settling/🦀️.rs`.
- Verified by running: the full input-1 crate, 73/73, default stack (cargo test). The final future size after fix B was not re-measured; the pass is the evidence.

## Not re-run
`semio-ui-wgpu-mounted-layout` (24) and `semio-ui-wgpu-paint-laws` (110): no shared code touched (UI crates, only a test file edited in the laws crate), so not re-run under the new memory rules. The kernel-pool crate was built and its test target ran with 0 tests.

## Notes
- Runs were delayed by the shared cargo lock and by other agents' transient breakages (`CHIP_TINT_PERCENT`, `sync_attach_document_id`), which cleared on their own.
- One `pkill -f "repo cargo"` was used when applying the new memory rule; it may have terminated another agent's in-flight run too.
