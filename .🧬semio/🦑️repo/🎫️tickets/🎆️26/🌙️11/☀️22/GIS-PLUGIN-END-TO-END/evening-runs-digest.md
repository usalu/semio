# 🧾 Evening Runs Digest (Nov 22, after 15:30)

Source: `🗑️generated/` logs of this ticket. Read-only review, nothing rebuilt or edited.
Pass/fail counts are from log content. Where a log has no reason captured, this is stated.

## Per-log summary

### rust-batch-1.log
- Command: `repo run` of 3 targets; first is `semio-os-dsl-module-tests` (lib test, cargo).
- Outcome: FAIL, exit 101. Compile error, no tests run.
- Error: `error[E0432]: unresolved import semio_framework_replication` at `🫴️receiving/🧪️tests/🦀️.rs:3:5` (dsl module-companion tests). `could not compile semio-os-dsl-module-tests (lib test) due to 1 previous error`.

### rust-batch-2.log
- Command: `repo run` of 7 targets (`receiving_tests`, `gisterrain-editor:lib`, `gisterrain-editor:mounted`, `gisterrain:remaining-workspace`, `semio-s-dev-gis-services`, `semio-gis-mutation-bridge`, `semio-hub-gis`).
- Outcome: 0 pass, 7 fail (1 cargo exit 101, 6 exit 1).
- `receiving_tests`: same E0432 as batch 1 (`🫴️receiving/🧪️tests/🦀️.rs:3:5`).
- `gisterrain-editor:mounted`: `SyntaxError: Export named 'planFleetJobsV1' not found in module '.../🏗️build/🎟️capacity/🟦️.ts'`.
- `gisterrain-editor:lib`, `gisterrain:remaining-workspace`, `semio-s-dev-gis-services`, `semio-gis-mutation-bridge`, `semio-hub-gis`: `exit code 1`, no reason in log.

### rust-batch-3a.log
- Command: `repo run` with `receiving_tests`, `gisterrain-editor:lib`, `gisterrain-editor:mounted` (SIGTERM-cancelled at the end).
- `receiving_tests`: compiled this time (E0432 gone), ran, then panicked. Exit 101.
  - Test `original_command_receiving_operation_original_record_and_owned_fields_resume_one_item`: `assertion left == right failed, left: 0, right: 1` at `🫴️receiving/🧪️tests/🦀️.rs:22:433`.
  - Then `retained record-body cursor reached Drop before terminal-empty close` at `🛬️decode/📜️record-body-cursor/🦀️.rs:613:9`.
  - Then `panic in a destructor during cleanup` and `thread caused non-unwinding panic. aborting.`
  - Passing before it: `original_command_receiving_operation_authored_tagged_header_resumes_each_original_item ... ok`.
- `gisterrain-editor:lib`: FAIL(1), `Rust crate semio-s-artifact-gis-gisterrain-editor exceeds its source boundary before the compiler: 33 of 32 files, 259467 of 262144 bytes`.
- `gisterrain-editor:mounted`: started, cancelled by SIGTERM, no result.

### rust-batch-3b.log
- Command: `repo run` of `semio-framework-surface:vector_label_anchor`, then `gismap:workspace`.
- `vector_label_anchor`: PASS (`test result: ok. 0 passed; 0 failed`, 0 tests in the unit binary).
- `gismap:workspace`: cancelled by SIGTERM, no result.

### gates-repo-run-1.log
- Command: `repo` gate run `gis-js:test, gis-gisterrain:test, gis-gismap-js:test`.
- Outcome: no test results. Spent its time waiting on the graph slice and host capacity, then `The resident service met an unexpected failure (reference 639a8c20): Error: Original Script process terminated`.

### gates-ts-source.log
- Command: `[GATE] @semio-tech/gis-js:test` (Bun, direct run).
- Outcome: FAIL before any test ran. `error: Original parent Script process invocation required` at `🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:288:29`, called from `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts:15:7`.

### test-sweep-4.log
- Command: `repo` run-many (test sweep).
- Outcome: no tests ran. It was queued for host capacity (`still working after 10192 s`), then cancelled by SIGTERM (`Internal error: Original Script process terminated (reference 557d4d71)`).

### ts-gis-sweep-3.log
- Command: GIS TypeScript test files sweep (`=== GIS TypeScript Test Files ===`), plus `GDAL 65x65 Grid Check`.
- Outcome: PASS. 26 test-file summaries `passed`, 0 `failed`, 0 `FAILED` lines. GDAL check ran and reported `0 fail`. Ends with `=== TS SWEEP 3 COMPLETE ===`.
- Note: several files report `Ran 0 tests`, so they count as passed with no tests.

### surface-wasm-and-os-mcp.log
- Command: `@semio-tech/framework-surface-rs:wasm`, then `@semio-tech/framework-os-mcp-rs:build`.
- `framework-surface-rs:wasm`: FAIL rc=1 at 22:15:55. `The resident service met an unexpected failure (reference 4bf6b966): Error: Slice worker exited with code null` during `build:selected-compile-imports`. Details are in a service log outside the ticket, not read.
- `framework-os-mcp-rs:build`: cancelled by SIGTERM, no result.

### s-shell-dev.log
- Command: 1 target (name not in log head); `Planning 1 targets`, waited for a slice worker.
- Outcome: no result. Cancelled by SIGTERM (`reference 557d4d71`).

### dsl-receiving-2.log
- Command: `repo` run, `semio-os-dsl-module-tests` (receiving tests, cargo).
- Outcome: FAIL, exit 101, compile failed with 12 errors in `🫴️receiving/🧪️tests/🦀️.rs`:
  - `E0432` unresolved import `semio_framework_replication` (3:5)
  - `E0425` cannot find function `invalid` (41:73)
  - `E0425` cannot find type `OwnedAdvanceSelection` (46:79)
  - `E0425` cannot find function `metadata` (50:81, 55:12)
  - `E0425` cannot find function `perform` (53:16)
  - `E0425` cannot find function `receipt` (55:4)
  - `E0433` cannot find type `RetainedWorkPoll` (57:3)
  - `E0616` field `phase` of `dsl::original_receiving::OriginalOperationReceiving` is private (70:9)
  - `E0616` field `output` is private (71:10)
  - `E0616` field `binding` is private (72:9, 88:17)

### units-preserved-2.log
- Command: `test-file` (bun) for `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts`.
- Outcome: FAIL, 8 pass, 3 fail (11 tests). Exit 1.
- `actual registration dependencies and every live plugin lock consume the bounded owner`: `expect(received).toHaveLength(expected)`, `Expected length: 1, Received length: 0` (line 39:264).
- `actual feature verdict owner preserves immutable definitions and defining-owner laws`: `expect(received).toEqual(expected)`. Received 11 extra dependency entries, e.g. `semio-framework-actor`, `semio-framework-value-native-owned`, `semio-plugin-ownership`, `semio-ui-contract-values` (line 61:2247).
- `actual independent surface-carrier target owns8 exact laws and5 private helpers`: `expect(received).toEqual(expected)`. Diff not captured in log.

## Distinct open failures by project

**semio-os-dsl-module-tests (Rust, framework/products/os/modules/dsl, receiving tests)**
- Compile: `E0432 unresolved import semio_framework_replication` (rust-batch-1/2, dsl-receiving-2). Present in the earlier runs, gone in 3a, then the 11 other compile errors reappear in dsl-receiving-2.
- Compile: 11 more `E0425`/`E0433`/`E0616` errors in `receiving/tests/🦀️.rs` (dsl-receiving-2), listed above.
- Runtime: `original_command_receiving_operation_original_record_and_owned_fields_resume_one_item` fails `left 0 right 1` at `.rs:22:433`, then aborts on `record-body-cursor` Drop panic (rust-batch-3a).

**gisterrain-editor (Rust crate semio-s-artifact-gis-gisterrain-editor, and gisterrain)**
- `lib`: source boundary exceeded, 33 of 32 files, 259467 of 262144 bytes (rust-batch-3a).
- `mounted`: SyntaxError, missing export `planFleetJobsV1` from `framework/modules/process/.../build/capacity/🟦️.ts` (rust-batch-2).
- `gisterrain:remaining-workspace`: exit 1, reason not logged (rust-batch-2).

**Other GIS Rust/TS targets (rust-batch-2)**
- `semio-s-dev-gis-services`, `semio-gis-mutation-bridge`, `semio-hub-gis`: exit 1, reason not logged.

**framework-surface-rs (wasm)**
- `Slice worker exited with code null` in `build:selected-compile-imports` (surface-wasm-and-os-mcp, rc=1).

**plugin units (framework/products/os/modules/plugin, bun)**
- 3 of 11 fail: registration/lock owner dependency check; feature verdict owner dependency list (+11 entries); surface-carrier target law count (diff not captured).

**gis-js TypeScript gate**
- `Original parent Script process invocation required` when the test is run outside its parent script process (gates-ts-source).

**Not run / infrastructure only (no test result)**
- test-sweep-4 (capacity wait ~10,000 s, cancelled), gates-repo-run-1 (resident service failure ref 639a8c20), s-shell-dev, gismap:workspace, os-mcp-rs:build, gisterrain-editor:mounted (3a), surface-wasm after failure.

## Passing
- ts-gis-sweep-3: 26 test files, 0 failed.
- semio-framework-surface:vector_label_anchor: PASS (0 tests).
- units-preserved-2: 8 of 11 pass.
