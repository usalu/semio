# print-rs build and test failure

## Findings

- The workspace lint policy (`/Users/Shared/semio/Cargo.toml`, `[workspace.lints.rust|clippy]`) sets every lint to `warn`; nothing is promoted to an error. The thousands of `unnecessary qualification`, `unused import`, `fetch_update` deprecation and similar diagnostics in the r6 logs are warnings and are not why the targets fail.
- The `[script]` progress stream in the r6 logs swallows the cargo error. The real compiler output is kept by the cargo owner under `.cache/b/r/m/u/l/<lane>/semio-cargo-logs/cargo-artifacts-*.log`.
- That file shows the only hard error, the same for build and test (the test target compiles the same dependency chain):

```
error[E0599]: no method named `has_failure` found for struct `OriginalConstructionAdmission`
  .../plugin/declaration/app/lifted/media-export/rs:107 and :116
error: could not compile `semio-plugin-decl-app` (lib) due to 2 previous errors
```

- Cause: `OriginalConstructionAdmission` (`🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs`) no longer has `has_failure`; its failure query is `failure_untaken()` (`is_failed() && !failed_taken`), which is what `genesis` already uses.

## Fix

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🛬️lifted/🎬️media-export/🦀️.rs`: both `self.admission.has_failure()` calls became `self.admission.failure_untaken()`.

## Verification

See the section appended below by the last run.

Status at 04:37: the first post-fix `build` run (2026-11-13 03:51, holding the run lock) is still waiting in the capacity gate for 2574 MB (another agent's `@semio-tech/print:test-viz-fixtures` run has been alive for 1.5 h and starves memory). Result files appear when it ends: `🗑️generated/ps-rs-build.log` and `🗑️generated/ps-rs-build.exit`; the lock is released by `🗑️generated/ps-rs-run.sh`. If `build` still fails, read the real error from `.cache/b/r/m/u/l/*/semio-cargo-logs/cargo-artifacts-*.log` (`grep -aE '^error'`), not from the `[script]` stream. `test` has not been re-run; `ps-rs-run.sh test` runs it.

## Second error (surfaced after the first fix, build run 04:42, exit 1)

`error[E0592]: duplicate definitions with name 'handoff'` in `semio-plugin-instance-jobs`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🏷️reserved/🦀️.rs` had two near-identical `pub fn handoff` (a concurrent edit left both). Removed the first (with `size_of::<tuple>`), kept the second (with `size_of::<Option<tuple>>`, the actual slot written, plus the doc comment). A new build run was queued at ~04:45 behind the run lock; result in `🗑️generated/ps-rs-build.exit` / `ps-rs-build.log`.

## Further build fixes (build exit 0 at 10:38)

- `semio-plugin-instance-docs`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs` passed a `u128` native-work reading to `receive_archive(..., observed:u64)`; both sites now use `u64::try_from(..).unwrap_or(u64::MAX)`.
- `semio-framework-print` (product crate) had not followed the receiving API:
  - `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` and `.../🚦️native/🦀️.rs`: `decode_sqlite_snapshot_native` / `encode_sqlite_snapshot_native` take the output slot and return `SnapshotReceivingOutcome`; native glue uses `decode_/encode_sqlite_snapshot_record_native(..., &mut closure, control, owner, output)` like the collection artifact.
  - `🧰️framework/🛍️products/📓️print/🚪️io/💾️binary/🧬️mutations/🦀️.rs`: `ChangeChartValue` binary wire is now the pack value of its `ToValue` (as `WorkflowMutation`), with `original_pack_value_operation_receiving!` and `artifact_persisted_binary_operation!` (this satisfies `StoreMutationFor`); the old `[1,1]`-framed `encode_op_into` / `decode_op_span` streaming pair had no framework caller and was removed. Note: the framework's pack-value receiving renormalizes whole-number floats to integers (`renormalize_whole_number_floats`), so a chart float such as `1.0` is received as `1`.

## Test fixes (build exit 0, test failing at compile in test crates)

The hidden cause of `test` failures is that the compile error text is lost with `--output=static`; `--output=stream` prints it (`@semio-tech/print-rs: error[...]`). Stream-mode run showed:
- `semio-os-space-testing-sqlite-native-caller` (shared helper, last touched Nov 6) no longer compiles against the funded-effect close API. The replacement crate `semio-framework-test-original-snapshot-owners` exists (`🔨️modules/🧪️test/🛬️original-snapshot-owners`). The print tests were ported to it: `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` and `.../📥️native-fixtures/🦀️.rs` (`encode_native` / `decode_native` / `io_run`; local `with_native_control` removed) and the dev-dependency in `🧰️framework/🛍️products/📓️print/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml`. Other crates (space, collection, dag, workflow, run) still depend on the stale native-caller helper; not touched.
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs`: `paged_chart_original_operation_...` (old streamed `[1,1]` format, `close_one(items,bytes)`) replaced by `chart_operation_binary_wire_keeps_exact_variants_and_path` over `encode_op`/`decode_op`.

Helper scripts: `🗑️generated/ps-rs-run.sh <target>` (static) and `ps-rs-run-stream.sh <target>` (stream, errors visible).
