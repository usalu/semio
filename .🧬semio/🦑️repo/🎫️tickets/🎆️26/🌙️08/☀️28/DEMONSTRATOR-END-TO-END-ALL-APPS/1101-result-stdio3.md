STATUS: PART 1 GREEN (hub check, `stdio3-hub-1.txt`). PART 2 (native tests csv/json/xlsx/zip/docx/tiff/png) IN PROGRESS, UNVERIFIED, FROZEN at Addendum 19:50: my test run was killed (pids 84242, 5430); no test has passed yet. Resume with `C/1101-tools/stdio3/tests.sh <n>` (set `ONLY=<csv|json|xlsx|zip|docx|tiff|png>` for one workspace) after `act-green`/`freeze-lifted`.

## Edits
- docx base `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` `retire_sqlite_snapshot`: `.result.unwrap_or_else`, `close_step(grant).result` (error -> panic, `.expect` removed).
- tiff `✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs` close_step: closure returns `RetainedWorkOutcome{result, work}` (same as png patch-pixel-region).
- png-tests: `paint-native-region` and `patch-pixel-region` were already ported by a peer at 12:20; not touched.

## Why test/regression crates are in the closure
`stdio-png-tests` and `stdio-tiff-mutation-regressions` are not tests: they are path-preserving carves holding production editor/definition/io-registry code. `stdio-png` and `stdio-tiff` list them as normal `[dependencies]` and re-export from them (`semio_s_artifact_stdio_png_tests::{definition,...}`, `..tiff_mutation_regressions::{io_registry,editor,...}`). Misnomer, no dev-dependency leak; renaming would touch ~65 lockfiles (coordinator-only `dependencies lock`), so left as is.

## Part 2: native test port (round 1 logs `C/🗑️generated/1101/stdio3-t1-*.txt`; one workspace takes ~1h, wrapper deadline ~65 min)
Applied before the freeze (all UNVERIFIED by a rerun):
- `stdio/📇️registry/🧬️contract/🧰️testing` (shared test fixture used by csv): `🧲️native-owners`, `🔀️routed-io` get `.result`; `🔁️registered-job` redesigned: `Session` owns an `OriginalJobReturnOrigin`; `step`/`step_stage` drive via `drive_step`, `origin.admit/receive_outcome`, then `acknowledge_outcome`; `finish` closes decoder then origin; continuation panics.
- json `🧱️base/.../🫴️receiving/🧪️tests`: import `ArtifactSqliteSnapshotHost as _` (the `_receiving`/`validate_*_decoding` methods live on it), `.result` on `close_retirement_recipient`.
- xlsx base mutations: fixtures (`fixture`, `sweep_a/b`, imports) and `opc-layer::with_demo_entries` widened from `cfg(test)` to `cfg(any(test, feature="testkit"))` because `demo_mutation_cases` is compiled under `testkit` without `test` (real E0425 in a lib build).
- zip editor tests: `command_from_action(.., &mut Some(args), &mut None)`; `semio-framework-plugin` feature `artifact-app-testing` now forwards to `semio-framework-plugin-decl-app/artifact-app-testing` (feature was never enabled, so `ArtifactToolCompletion::test_new` was unreachable); `📬️preparation` tests use a `advance` helper with `testing::admit_owned_source` sources, `PagedOneItemPost::Mutable`, `NativeDecodeControl`.
Not yet seen: docx, tiff, png test compile results (round 1 was still compiling docx).

## Rename note for close-out
Misnamed crates: `semio-s-artifact-stdio-png-tests` (`png/.../🧪️tests/🔬️unit`) and `semio-s-artifact-stdio-tiff-mutation-regressions` hold production code (see below). Rename after activation, with `dependencies lock` for the ~65 lockfiles.

## Test run status (post-freeze narrowing, 20:55)
csv (`C/🗑️generated/1101/stdio3-t6-csv.txt`): compiles. Test-only ports applied: `ColdMutation/ColdMutationDiff/DiffBinary/DiffText/OpBinary/OpText` imports, `.result` on `close_retirement_recipient`. Run: ~100 pass, 16 fail, all from three production-level causes (none test-port):
1. 11 failures `original retirement requires its paid caller turn`: `NativeDecodeControl::close_retirement_recipient` goes through `with_retirement` -> `NativeRetirementDecodeControl::begin`, which Errs on a zero-item/zero-depth (denied) grant, while the encode twin and the inner close return `Ok(Progress(0))` for a denied grant and every law fixture (`deniedCloseGrant`) expects that. Patch: `C/1101-tools/stdio3/patches/decode-denied-close-grant.patch` (apply inside `🧰️framework/🔨️modules/🌱️value`). Same fault is testport item 3/5.
2. 4 failures (`browser_archive_reload...`, `the_kit_verb_edits...`, `an_agent_set_cell...`, `history_edits_end_to_end`): `plugin.document-load.history-invalid: document archive parent Pack and SPR hydration was rejected: Stale`. `RetainedPersistedDocumentHydration::from_pack` in `🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs:243` is created with `OperationId(active.operation)` and `Generation(stores.store.generation_now())`, but `check_authority` compares them with the stepping `cx.operation()/cx.generation()`. Needs a live diagnostic (print both pairs) after the freeze; every host-loading editor test of every artifact is exposed.
3. 1 failure `original file decode close exceeds authored turn ceiling` (`os-io 🧰️testing/🦀️.rs:22`): fixture ceiling vs production demand drift.
