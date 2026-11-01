STATUS: PARTIAL (blocked on stdio pdf). fem engine wasm32-wasip2 GREEN (fem-4.rc 0). No fem crate errors remain in the last 3d wasm run (fem-19), but that run is blocked by foreign stdio crates, so 3d wasm is not proven. Hub component, 3d snapshot tests (incl. round-4 edits) and the mounted Betonwald load test are UNVERIFIED.

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
