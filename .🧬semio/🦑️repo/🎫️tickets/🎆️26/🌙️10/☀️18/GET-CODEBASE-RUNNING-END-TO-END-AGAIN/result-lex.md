# Result LEX (executor prefix `lex`): `layout-engine-export` tests on the current tool-job design, carved to a companion

Sonnet 5.5, foreground, no sub-agents, no cargo/nx/`bun repo`, no modifying git command (an auto-commit, #694, swept my edits while I worked). Nothing was compiled or run. `X` = `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`, `E` = `X/⚙️engine/📤️export`. Tools: `C/lex-tools/` (`posrun.py` launcher over a copy of `F/c5b-tools/pos-tools` in `~/.cache/lex/pos-tools`, `errs.py`, `census.ts`, `families/lex*.json`). Originals of the replaced test files: `~/.cache/lex/orig/` (outside the repo, path-length rule).

## 0. Verdict

The crate's unresolved test names were not one defect but five layers of drift, none of them "never implemented": every one of the ~70 names was deleted, renamed or moved by a later change. All 41 original laws are preserved (38 in the companion, 2 in the json crate, 1 at the `export-pdf` node). The tests now target the current design, live in a new companion crate, and the export crate dropped from 0.996 (test closure, lines) to **0.676** (lib 0.601); the companion is 0.308. No production behaviour changed; the only production-side edits are visibility widenings behind a `testkit` module and the relocation of the test oracle into its own `testkit`-gated module. Static evidence only (section 4); the compile executor must run section 5.

## 1. Fate of the names (`git log -S`, read-only)

| names the tests used | fate | commit(s) |
|---|---|---|
| `StepOutcome` as the return of `InteractiveJob::step`, `Complete(candidate)`, `Fault(_)`, `CheckpointReady(c)`, `candidate.output.is_empty()` | **replaced**: `step` returns `Result<Option<JobOutcomeBorrow>, OriginalTurnFailure>`; the enum is still exported by `semio_framework_job` but no job produces it; layout export production migrated | borrowed model #679 `5487a5cce52`, #680 `48e47561a9e`; layout export production #682 `03c902fa488` |
| `JobPayloadCloseStep`, `close_step(1, JOB_PAYLOAD_PAGE_BYTES)`, `outcome.close_step`, `session.step()`, `take_outcome()`, `BatchJobSession::try_new`, `BatchJobParams.work_grant` | **deleted**: replaced by `RetainedCloneStep`/`InteractiveJobCloseStep`, `close_step(cx)`, `step_on_original_caller(cx)`, `checkout_outcome`, paid `acknowledge_outcome`, `try_admit_owned`, `BatchDriveConfig.retained` | #679 `5487a5cce52`; sessions `wgpu2` |
| `PluginCloseStep`, `ArtifactReservedJob::close_step(job, items, bytes)` | **deleted**: `ArtifactReservedJob` is a marker over `InteractiveJob`; there is one close ladder | #681 `c44e964518f`, #682 `03c902fa488` |
| `LayoutExportJob::new`, `with_output_chunks`, `job.request.snapshot` | **renamed/reshaped**: `from_admitted(operation, source_identity, request, output_chunks)`, `output_chunks: Option<_>`, `request: Option<_>` | #687 `7c8af563c74` |
| `LayoutExportJobFactory`, `LayoutExportToolPayload`, `LayoutMediaExportJobFactory` | **moved, not deleted**: they live in the editor crate (`X/📤️export-tool-job`, `X/🎥️media-export-job-and-factories`) since the export crate was split out of it; the tests stayed behind and could never resolve them (a dev-dependency back-edge would also give two copies of the crate) | #688 `793f63533f2` |
| `JsonContainer`, `JsonArrayState`, `JSON_INPUT_BYTES_PER_UNIT`, `JSON_OUTPUT_BYTES_PER_UNIT`, `TypedJsonCursor { .. }` literal | **moved** to the `-json` crate, private there; `TypedJsonCursor` also gained a private `drawing` field | #688 `793f63533f2` |
| `LayoutExportJob::pdf_image_matrix` | **reshaped**: free function after the c5c7 file split | #657 `dfe2687f7db`, #694 |
| `export_document_*_headless_batch` consumed by the `layout-engine` scene tests | **cfg(test) pub in a dependency: never visible to a dependent** (second defect, found on the way) | #688 |
| `ArtifactReservedJob` close laws asserting `snapshot-unwitnessed` | message is now `layout snapshot lost its paired original supplier` | #682 |

## 2. Design decisions

* **Where tests belong follows the dependency edges.** Pure export laws and the factory dispatch laws (they need the editor's factories) go to one companion that depends on export (`testkit`), json, editor and the commands crate: no cycle, one copy of every type. The JSON cursor laws moved into the json crate beside the code they test. The one law that needs the editor *app fixtures* moved to the `export-pdf` command node, mounted by the existing commands companion (section 3).
* **One batch driver.** `E/🧪️testing/🦀️.rs` (new, `testkit`-gated) holds `drive_job_batch` (admit with `try_admit_owned`, `step_on_original_caller`, checkout, paid receipt and acknowledgement, `resume`, `close_owner`, `retire`), `LayoutExportObserved` (an owned copy of one borrowed outcome), the grants, and the headless oracles rebuilt on the driver (they were ~100 lines of the same loop inside a production file). It replaces the old `drive_test_job_then`.
* **Allocator witness.** `OriginalJobFixtureCaller::admit` asserts its receipts against `semio_framework_trace::HeapWitness`, so every binary using it needs the global allocator. The old in-crate oracle and the engine scene laws lacked it (they would have failed at runtime). Installed in the companion root and in `layout-engine`'s `cfg(test)` root, with the matching dev-dependencies.
* **Visibility, not re-implementation.** `pub(super)` items the white-box laws need were widened to `pub` inside the private modules (`ChunkRope`+fields, `ExportRect`, `ZipEntry`, `ZipState` fields, `LayoutExportCloseStage`, `OUTPUT_CHUNK_BYTES`, `MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS`, `LayoutExportCommit::from_chunks`, `LayoutExportPublication::{advance,close_step,terminal_is_empty,...}`, 12 `LayoutExportJob` fields, `crc32_update`, `decode_base64`, `pdf_image_matrix`; json: `owned_string`, `JSON_INPUT_BYTES_PER_UNIT` to `pub(super)`), reached through `pub mod testkit` (`feature = "testkit"` or `cfg(test)`). No production code was added: the tool-job design needed none (every production piece, `LayoutExportJob`, `LayoutExportToolJob`, both factories, was already ported).
* **Laws kept, re-based.** The reserved-close laws became close-ladder laws on `InteractiveJob::close_step(cx)` (progress receipts instead of item counts; narrow `CHUNK_SLICE` grant keeps the "bounded release" claim; `begin_close` first, because `close_step` resets the stage otherwise). The one-unit-budget and checkpoint laws step the job under `with_original_work_loan(1, ..)` and read the borrowed outcome immediately. The stale-context law builds a second caller with another generation. `exact_factory_dispatch_is_deterministic_across_real_one_two_four_and_default_worker_pools` ignored its `worker_count` already at HEAD and read the shared queue after the close ladder had drained it (likely comparing empty strings); it is now `..._across_repeated_original_caller_drives`, reads the bytes at the terminal outcome and asserts they are non-empty.

## 3. What changed (file list)

Export crate `E` (all `X/⚙️engine/📤️export/` unless marked):
* `🦀️.rs` (test mount removed; `testkit` module, `test_oracle` mount, cfg widening), `📦️packages/🦀️rust/Cargo.toml` (`testkit = [job/testkit, job-worker/testkit]`, dev-dependencies removed: the editor back-edge is gone).
* `🧪️testing/🦀️.rs` NEW (driver, observed, oracles); `⏭️advance-commit-and-codec/🦀️.rs` (oracle region removed, codec helpers `pub`, checkpoint reader cfg); `📜️export-contract-and-state/🦀️.rs`, `🏗️job-admission-and-close/🦀️.rs`, `📕️pdf-png-and-package-encoding/🦀️.rs` (widenings, `restore`/`LayoutExportCommit` cfg).
* `🔣️json/🦀️.rs`, `🔣️json/🧪️tests/🔬️unit/🦀️.rs` NEW (2 laws moved from the export tests), `🔣️json/✅️json-validation/🦀️.rs`, `🔣️json/🏗️cursor-construction-and-owner-pages/🦀️.rs`, `🔣️json/📦️packages/🦀️rust/Cargo.toml` (dev-dependencies).
* Companion `-unit-tests`: `🧪️tests/🔬️unit/🦀️.rs` (root: imports, allocator, `include!`s), `🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` NEW, `🧪️tests/🔬️unit/📕️pdf-structure/🦀️.rs` NEW (shared pdf oracle), the three existing fragments rewritten/trimmed (`🎟️harness-pdf-and-dispatch-laws`, `🚧️caps-json-checkpoint-pdf-style`; `✒️pdf-drawing-laws` unchanged).

Elsewhere:
* `X/🎮️commands/📕️export-pdf/🧪️tests/🔬️unit/🦀️.rs` NEW (the `exportPdf` download law, moved from the export tests; includes the shared pdf oracle), mounted in `X/🎮️commands/🧪️tests/🥣️companion/🦀️.rs` and `…/📦️packages/🦀️rust/Cargo.toml` (+export `testkit`). It still rides the editor fixture layer, whose plugin typed-operation API (`dispatch_typed` and `settle_*` without `cx`) is not yet ported; that port is shared by every editor/plugin law and was not in scope.
* `X/⚙️engine/🦀️.rs` and `X/⚙️engine/📦️packages/🦀️rust/Cargo.toml` (heap witness; export dev-dependency now `features = ["testkit"]`, `semio-framework-trace` dev-dependency).
* Registration: layout `Cargo.toml` (member), `📋️build-owners.json` (owner row `semio-s-artifact-layout-layout-engine-export-unit-tests`). No `launch.json` route referenced the moved tests (grepped).

## 4. Static verification

| check | result |
|---|---|
| rustfmt parse (`~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustfmt --edition 2021 --config skip_children=true --emit stdout`) of all 18 edited/new `.rs` | all parse |
| typeck/privacy oracle (nightly-2026-07-20, `posrun.py`, families `lex`/`lexb`: export + json + companion against `HEAD` for export and json) | lib: errors 413 = base 413, **NEW 0 / GONE 0**; test: privacy 0, trait-scope 0, resolve 0, other 0, type 1 = `E0204` on `#[derive(Copy)] TestRun` caused by the stub `Generation` (the real `Generation` is `Copy`). First run before widening: 44 privacy and 50 resolve rows, all removed by section 2. Log kept: `C/🗑️generated/lex-lexb-b2-report.txt` |
| caveat of the oracle | everything outside the family is stubbed: the framework calls (`OriginalJobFixtureCaller`, `BatchJobSession`, `OriginalWorkerFixtureCaller::close_owner`, `with_original_work_loan`, plugin `ArtifactOutputChunks`, action-bus wire API) were verified by reading their current signatures, not by rustc (a family with the job crates added is too stub-noisy to separate: 3,816 rows) |
| canonical source gate (`readRustSourceClosureV1`, `C/lex-tools/census.ts`) | export lib 6 files / 123,022 B / 2,405 lines = **0.601**, export test closure 7 / 135,651 / 2,704 = **0.676**; companion test closure 5 / 75,561 / 1,232 = **0.308**; json 0.498 / 0.509; commands companion 0.406; engine 0.400 / 0.594 |
| paths | longest touched path 183 chars |
| tool-job census probe (`F/c5c-tools/tooljob-probe.ts`) | runs; the export module tree is read as one source (135,189 B, 2,700 lines); the totals moved with concurrent peer work, so no before/after claim |

## 5. For the compile executor

First `cargo update --workspace --offline` for `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout` (new member, dev-dependency changes in json/engine/commands companion/export; nothing hand-edited in `Cargo.lock`). Then `bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests`:
* `semio-s-artifact-layout-layout-engine-json` (new in-crate tests)
* `semio-s-artifact-layout-layout-engine-export` (also `--features testkit`, and plain lib)
* `semio-s-artifact-layout-layout-engine-export-unit-tests` (new)
* `semio-s-artifact-layout-layout-engine` (`--tests`: heap witness and export `testkit`)
* `semio-s-artifact-layout-layout-editor-commands-tests` (`--tests`; expected to fail only where the editor/plugin fixture API is still stale, not on the moved law's export-side names)

Tests to run: `cargo test -p semio-s-artifact-layout-layout-engine-export-unit-tests --lib` (38 laws), `-p semio-s-artifact-layout-layout-engine-json --lib` (2), `-p semio-s-artifact-layout-layout-engine --lib scene` (headless export laws through the new driver).
Look first at: the closure shapes in `step_one_unit`/`drive_job_batch` (borrowed loans), `FRU`-free `TypedJsonCursor::default()` pushes, `failure.primary_value()` message match in `close_disposes_every_export_buffer_...`, and the narrow `CHUNK_SLICE` grant (`copy 0, capacity 0`) against `close_demand()`.

## 6. Open / not done

* Not compiled, not run. Runtime assumptions that only a run settles: the stale-generation law steps one job with two callers (publication bound under the second), the `Debug` text of `OriginalTurnFailure` is not used (the law reads `primary_value().message`), `close_releases_the_witness_..` and `close_zero_budget_..` set `close_stage` by hand after `begin_close` and drop jobs that were not closed to terminal-empty (the old laws did the same).
* The editor/plugin fixture layer (`unit_tests::context`, `transform_tool_transaction_tests`, commands companion laws, generation3d and other plugin fixtures: `PluginCloseStep`, `dispatch_typed(.., identity)` without `cx`) is still on the old typed-operation API; not touched.
* Stale tool-job laws listed in `result-wgpu2.md` section 4 (`🔬️tool-job-*`, `⚖️laws/🎯️tool-job-coverage`) not touched.
* `Cargo.lock` of the layout workspace not refreshed (needs cargo).
