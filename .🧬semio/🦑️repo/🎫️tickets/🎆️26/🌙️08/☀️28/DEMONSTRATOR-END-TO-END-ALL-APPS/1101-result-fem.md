STATUS: PARTIAL. fem engine, 2d/3d editors and `semio-hub-fem` + `semio-hub-fem-component` GREEN on wasm32-wasip2 (`fem-hub-3`); 3d snapshot tests and Betonwald mounted test NOT run. fem engine wasm32-wasip2 GREEN (fem-4.rc 0). No fem crate errors remain in the last 3d wasm run (fem-19), but that run is blocked by foreign stdio crates, so 3d wasm is not proven. Hub component, 3d snapshot tests (incl. round-4 edits) and the mounted Betonwald load test are UNVERIFIED.

## GREEN (proven)
- Engine wasm check (`semio-s-fem-{analyses,analyses-assembly,sparse,sparse-ldlt,sparse-pcg,sparse-subspace,mesh,static,model,payload-turns,elements3d}`), log `C/🗑️generated/1101/fem-4.txt`, rc 0.

## Edits this wave
- `analyses/📦️packages/🦀️rust/Cargo.toml`: added `semio-s-fem-payload-turns` (the unresolved import).
- `analyses/🦀️.rs`: `FemJobGraph` no longer calls the removed `StepContext::payload_from_bytes`; it publishes checkpoint, preview and commit output as staged `NumericalBytesPublication` turns (same pattern as `assembly/🔶️csr-build-and-run`), with `publication` included in `close_frontier`, `terminal_is_empty` and `close_granted`.
- fem 2d (shared by the 3d closure): `...◻️2d/.../🚪️io/📝️text/📸️snapshot/🦀️.rs` had a self-multiplied `structure::{...}` import (E0252), reduced to the 7 names. 2d and 3d `🚪️io/💾️binary/📸️snapshot/🦀️.rs`: `admit_document`, `take_into`, `step` read through `.result`, and the `OriginalReceivingPrimary` error is mapped to a `ValueError`. 2d and 3d `🧬️schema/📸️snapshot/native.rs`: `OriginalReceivingPrimary` import moved to `semio_framework_value_native_owned`, `close_retirement_recipient(..).result?`, staged admission no longer uses the removed `ready()`.
- Note: the duplicated import suggests a generator that is not idempotent; not investigated.

## Later edits (fem-14..19)
- `fem-2d-editor-transient`: `begin_projection` returns `RetainedWorkOutcome<(Projection, Progress)>`.
- 2d `editor-session` (`mounted-state-step`) and 3d `editor-visual-job` (`staged-step/*`): child jobs are stepped with `NumericalStep::step_plain` (the claim-taking `InteractiveJob::step` needs an `OriginalJobReturnClaim`); both crates gained a path dependency on `semio-s-fem-payload-turns`.
- 3d `visual-job` `staged-step`: `close_original_retained_step` callback returns `RetainedWorkOutcome{result, work: 1}`.

## State at hand-off
- fem-19 (3d wasm, `C/🗑️generated/1101/fem-19.txt`, rc 1): no fem crate reports an error any more. The only failures are foreign stdio crates `semio-s-artifact-stdio-deflate-zlib` (7) and `-gltf-model-snapshot-root` (6), which the 3d closure pulls in, so the remaining 3d crates were not compiled. fem-3d wasm is UNVERIFIED until those compile.
- fem-10 (native `--tests` for 3d snapshot-tests, `fem-10.txt`) was blocked earlier by OS plugin-framework crates (`time-travel-turns`, `instance-typed-ops`, `tool-run-turns`); not re-run since they changed.
- 2d editor-session unit test `🧪️tests/🔬️unit/🦀️.rs:71` still calls `job.step(&mut context)` with `child_end`; will need `step_plain` when tests are compiled.

## Not run
`semio-hub-fem-component` wasm check, 3d snapshot tests (incl. round-4 edits), mounted Betonwald load test. Next: rerun `1101-tools/fem/wasm3d.sh <label>` once stdio deflate/gltf compile, then native snapshot tests, then hub component.

## Resume after desc-w7 (fem-20..22): BLOCKED by stdio pdf
- fem-20 (3d wasm), fem-21 (`semio-hub-fem` + `semio-hub-fem-component` wasm), fem-22 (3d snapshot-tests native `--tests`) all stop in the same foreign crates, with no fem error: `semio-s-artifact-stdio-pdf-schema-{4,5}` (E0432 `crate::editor::pdfNN::modes::edit::windows` is configured out in `pdf/.../🧬️schema/.../🪞️mirror/🦀️.rs`, called from `✏️editor/🎭️modes/✏️edit/🦀️.rs:5`) and `stdio-pdf-mut-family-15`. Logs: `C/🗑️generated/1101/fem-20|21|22.txt`.
- The fem closure (3d editor and the hub descriptor) pulls in stdio pdf, so no fem crate after that point can be verified until stdio pdf compiles. Owner: the stdio lane.
- Still UNVERIFIED: 3d wasm, hub fem component, 3d snapshot tests incl. round 4, mounted Betonwald load test.

## Rerun after replay/pdf fixes (fem-23, fem-24)
- fem-24 (3d wasm, `C/🗑️generated/1101/fem-24.txt`, rc 1): `stdio-pdf-schema-4/-5` and `-mut-family-15` now compile. The only remaining error is `semio-s-artifact-stdio-pdf` (lib): 12 x E0433 `cannot find module or crate semio_s_artifact_stdio_pdf_testkit` at `pdf/🦀️.rs:99,143` (`pub use semio_s_artifact_stdio_pdf_testkit::...` in non-test code). In `pdf/📦️packages/🦀️rust/Cargo.toml` the crate is declared only under `[dev-dependencies]` (line 142), not `[dependencies]`. Owner: the pdf lane (moving it may create a cycle, so I did not touch it). No fem crate errors.
- No pdf source file changed in the last 3+ minutes at 18:03, so I stopped polling. Rerun `1101-tools/fem/wasm3d.sh <label>`, `chk.sh` for `semio-hub-fem-component` and the native `semio-s-artifact-fem-3d-snapshot-tests --tests` once `semio-s-artifact-stdio-pdf` compiles.

## fem-27 (3d wasm, after pdf fixes) and activation freeze
- fem-27 (`C/🗑️generated/1101/fem-27.txt`, rc 1): stdio/pdf now compile. Two fem errors surface: 3d `editor-panels` (10, snapshot-port fallout: SealedShared<String> comparisons, `&FemNode` vs `&FemNodeValue` row helpers) and 2d `editor` (1, `FemGumballTransientMutation: ArtifactCanonicalJsonTree` missing).
- ACTIVATION FREEZE (Addendum 19:50) is in force: no cargo/repo/test, no source edits. Nothing of mine is running. Proposed patches are in `C/1101-tools/fem/patches/PLAN.md`; apply them after `act-green` or `freeze-lifted`, then rerun 3d wasm, `semio-hub-fem-component`, 3d snapshot tests, and the Betonwald mounted test.

## Freeze exception: 2d editor + 3d editor-panels GREEN (wasm32-wasip2 `--lib`)
- fem-31 (`semio-s-artifact-fem-2d-editor-transient`, `-2d-editor`, rc 0) and fem-33 (`-3d-editor-panels`, rc 0), logs `C/🗑️generated/1101/fem-31|33.txt`.
- 2d transient: `CanonicalJsonTree` derive (owner `semio_framework_pack_json`) on state, gesture, entry; hand `ArtifactCanonicalJsonTree` impl for the macro-generated `FemGumballTransientMutation` (`{"kind":"snapshot","transient":..}`); `semio-framework-pack-json` moved from dev- to normal dependencies.
- 3d panels: label and row helpers take the persistent `FemNode`/`FemMaterial`; id comparisons via `as_str()`; the windowed outliner sections collect the persistent lists into `Vec<&_>` because `window_section_or_placeholder` takes a slice (O(n) per build; a list-window API in the tree kit would avoid it).
- No other edits; freeze otherwise respected. Remaining after the lift: 3d wasm full list, `semio-hub-fem-component`, 3d snapshot tests, Betonwald mounted test.

## Hub loop (fem-hub-1..3): fem GREEN
- `3d editor` fixes: Cargo.toml gained `semio-framework-diagnostic`, `semio-framework-value-native-owned`, and `semio-s-artifact-fem-2d-results-state` moved from dev- to normal dependencies; `store-preparation` `advance` takes the new `_receiving: &mut NativeDecodeControl` and imports `ColdMutation`; `play-app` uses `materials.iter().next()` and `as_str().to_owned()` for the SealedShared id.
- `semio-hub-fem` Cargo.toml gained `semio-framework` (needed by the `dyn_enum_close!` expansion), same path form as the flow hub.
- `C/🗑️generated/1101/fem-hub-3.txt` (rc 1): no fem crate red; `Checking semio-hub-fem` and `-component` ran without error. Remaining red crates are foreign (puzzle, procedural, gis).
- Not run (freeze exception allows no native tests): 3d snapshot tests incl. round 4, Betonwald mounted test.

## 3d snapshot tests (native, `1101-tools/fem/test3d.sh`) - fem-t1..t4
- fem-t1: `--tests` compile failed on 7 `RetainedWorkOutcome` call sites in `3d/.../🧪️tests` (`close_step`, `close_retirement_recipient`): fixed with `.result`.
- fem-t2: 9 passed / 10 failed. 9 failures = tests build `NativeDecodeControl` without the original caller work loan the framework now requires ("native decode work requires its original caller loan", UnsupportedOwner). Fix in tests only: a `Loan` work recipient and `native.with_work_recipient(&mut Loan, ..)` around every `NativeSnapshotDecodeOwner` use (`native-tests.rs`, `loads/receiver-tests.rs`). One failure (`denied_capture_and_physical_frontier_preserve_original_owner`) assumed the first frontier needs capacity; the staged source admission's first step is copy-only now, so the test advances to the first capacity frontier before denying capacity.
- fem-t3 (`C/🗑️generated/1101/fem-t3.txt`): 18 passed / 1 failed. Round-4 laws pass: copy-denial law, cancel inside large material name (Canceled wins), pack receiving, load receivers, identity index. FAILED: `actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work`: `OwnershipLimit "snapshot cursor original grant cannot admit its frontier"`, retained_capacity 536869632 = the whole 512 MiB test grant.
- fem-t4 (temporary `[DEBUG]` probe in `native-tests.rs`, to be removed): receive_authoring cost is LINEAR but huge: ~72 KB retained capacity charged-and-released per node (250 nodes: 18.0 MB; 1000: 71.9 MB; 4000: 287.3 MB; copied_items ~340/node). 10000 nodes need ~718 MB > the 512 MiB grant. Before the work-accounting/staged-admission port the same test passed (17/17 at fem-2), so a per-node transient capacity of ~70 KB crept in (production fault, perf: should be a few hundred bytes). Not fixed (production source is frozen); to diagnose: which allocation per node (candidates: per-item `SealedShared<String>` shared allocation grant sizing, `NativePersistentListReceiver::take`, page reservation per push).
