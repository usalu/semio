# Job-context source boundary 2026-11-26

Method: `job-context-closure-jobs.py` + `real-closure-probe.mjs` (the repository's own `readRustSourceClosureV1` with the real rustc cfg) on the package shim of each crate. The shim counts (the earlier 262,309 included its 179 bytes). Limits per crate: 262144 bytes, 32 files, 4000 active lines (not raised, no crate lowers them).

## Before
`semio-framework-job-context` lib 16 files, 262,130 (+179 shim) bytes, then 262,271 after a concurrent edit; test unit 265,341 bytes; lines about 1,840 to 1,980. Over the byte limit.

## After (bytes / % of 262144 / files / active lines)
| Crate | lib | lib + testkit | lib test unit |
| --- | --- | --- | --- |
| `semio-framework-job-context-step` | 206,041 / 78.6 % / 12 / 1,572 | 215,439 / 82.2 % / 13 / 1,664 | 218,509 / 83.4 % / 16 / 1,713 |
| `semio-framework-job-context` (facade) | 57,286 / 21.9 % / 8 / 416 | same | same |

All units of both crates are at or under 85 % of the byte limit; files at most 16 of 32; lines at most 43 % of 4000.

## Design
- New crate `semio-framework-job-context-step` (package `🧱️step/📦️packages/🦀️rust`, its own `rust-source-boundary` metadata) owns `StepContext` and every module that touches its private state or is named by it: the root `🦀️.rs`, `🎟️source`, `🛬️owned` decoder, continuation `source`, retirement close context, return place, weak return, failure, binding, and the testkit hooks (`🧪️testkit/🦀️.rs`, now a file mounted only for tests and the `testkit` feature instead of inline). No file moved, so concurrent edits of `🦀️.rs` and `🎟️source/🦀️.rs` stayed valid (a package shim mounts the existing root, as the other packages do).
- The facade keeps the name `semio-framework-job-context` and the shim; its domain root is `🪟️facade/🦀️.rs`, which re-exports the step crate (`pub use ...::*`, so no client import changes except three trait imports below) and mounts, from their existing paths, the modules that need only the step crate's public surface: Source admission, Source close producer, child source, `StepContextOwner`, retirement pump and turn, and the return-origin birth surface.
- A module could leave only if the step crate never calls it. The cycle check showed these cannot: place (the macro-expanded native Source embeds it), retirement close context and continuation claims (called by Source and place), decoder and weak return (fields and impls of `StepContext`), failure (trait impl). That bounds what a vertical split can reach.
- Private state became reachable through hidden, documented accessors in the step crate instead of public fields: `sealed` (hidden module with `SourceRoots` and four helper fns), `SourceRoots::{assemble,into_inputs,payload_identity,cancel,sealing_issuer}`, `OriginalCallerSourceParent::{born,child_witness,original_is_held,borrowed_cancel,lend_borrowed_cancel}` plus three hidden `pub` close methods, `OriginalCallerSourceLoan::retained_identity`, `StepContext::{return_budget_ref,return_budget_mut,source_parent_ref,with_payload_ledger}`, and `StepContext::lend_child_turn` (the child context transplant moved out of the child source so the 20 private fields stay private).
- Return-origin birth was a set of inherent `StepContext` methods used only by the native-turn and headless callers; it is now the trait `StepContextReturnOrigin` (facade), imported in `🛬️receiving/🎟️native-turn/🦀️.rs`, `🛬️receiving/🎟️native-turn/🌱️birth/🦀️.rs` and `🏃️headless/🎟️caller/🦀️.rs`.
- `testkit` in the facade re-exports the step testkit and adds `child_source_status`.

## Files changed
- New: `🧱️step/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📋️project.json}`, `🪟️facade/🦀️.rs`, `🪙️retirement-pump/🦀️.rs`, `🧪️testkit/🦀️.rs` (replaced the unmounted stale copy), all under `🧰️framework/🔨️modules/🧵️job/⏱️context`.
- Edited: `⏱️context/🦀️.rs`, `🎟️source/🦀️.rs`, `🎟️source/{🏭️admission,🏭️close,👶️child,🏭️return-origin}/🦀️.rs`, `📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}`, the three client files above, `🧰️framework/Cargo.toml` (member and workspace dependency), `📋️build-owners.json`, and `Cargo.lock` in all 188 lock files that lock `semio-framework-job-context` (`job-context-lock-sync.py`).
- The facade manifest still lists every former dependency; pruning the ones its modules no longer use is left.

## Verification
See the end of this file.

## Verification (one worker, through `repo capacity run --memory=2663 --cpu=1`, `--offline --locked`)
- `check -p semio-framework-job-context-step`: rc 0 (46 warnings, mostly pre-existing `unused_qualifications`; `🗑️generated/step-check2.log`).
- `check -p semio-framework-job-context`: rc 0, source admission passed (`🗑️generated/facade-check1.log`).
- `check` of step, facade, `semio-framework-job`, `semio-framework-job-native-turn`, `semio-framework-job-testing-caller`, `semio-framework-job-return` together: rc 0 (`🗑️generated/verify-check2.log`). `semio-framework-job-headless` is not selectable from the repository root workspace (`package ID specification ... did not match`); its only change is one trait import, not compiled.
- `test -p semio-framework-job-context -p semio-framework-job-context-step`: NOT RUN. It must run from `🧰️framework` (from the root cargo refuses: not a workspace member). The request (`🗑️generated/verify-test3.log`, pid of my `repo capacity run`) has waited over 60 minutes at the capacity gate with 0 jobs running, behind another agent's 4614 MB command (pid 57648) that cannot fit in 3.5 GB spare; I did not touch it. If it starts it writes its result to that log (final `rc=` line).
- Not yet done: pruning unused dependencies from the facade manifest; clippy-level cleanups of the step's unused imports.

## Follow-ups (explicit re-exports, dependencies, warnings)
- The facade no longer globs the step crate in its public surface. `job-context-reexport-list.py` takes the step root's public names (172) and keeps those that appear as an identifier in any of the 2,386 `.rs` files naming `semio_framework_job` or `semio_framework_job_context` (a safe over-approximation of what clients use): 159 names re-exported explicitly (plus `install_browser_monotonic_clock` behind its wasm cfg) and 28 of the 29 step testkit names (`observe_native_decode_progress` is unused). Never-used names such as `OriginalBirthBaseline`, `JobPayloadPage` and `OriginalNativeBorrow` are no longer exported. The facade's own modules still read the step crate through a private glob import; it is not exported.
- Facade manifest: dependencies cut from 24 to 10 crates (step, job-receiving, job-return-budget, value, value-control-contract, value-native-outcome, value-native-owned, value-native-receiving-frame, value-retained-clone, value-shared-issuance); `job-payload` and `trace` moved to dev-dependencies for its integration tests; the `testkit` feature now forwards to step and job-receiving only. All 189 lock files re-synced.
- Step warnings fixed: unused atomic and task imports in the root, `OriginalReturnSourceInput` in source, `ValueRefusalKind` in place. The remaining step warnings are pre-existing `unused_qualifications` and one deprecated `fetch_update`.

## Result of the queued runs
- `check` of step, facade, job, native-turn, testing-caller and return after the explicit re-exports and dependency cut: rc 0 (`🗑️generated/verify-check3.log`).
- First `test` (`verify-test3.log`): rc 101, the facade test `original-return-and-retirement-contracts` mounts `⚠️failure/🧪️tests/🧰️support/🦀️.rs`, which needs `semio_framework_diagnostic`; my dependency cut had missed that transitive mount. Added it as a facade dev-dependency (all lock files re-synced).
- Second `test` (`verify-test4.log`): rc 101. The test targets before `original-native-receiving-context` pass (effect-provider 6, pair-slot 3, native 4, native-progress 14). In `original-native-receiving-context` (file `🛬️receiving/🧪️tests/🦀️.rs`) 3 of 6 fail: `original_receiving_ready_scope_settles_a_ready_step_and_refuses_a_suspended_one`, `original_context_charges_child_construction_and_enclosing_semantic_once` (progress 3 items / 402 bytes against 2 / 321) and `original_context_retirement_after_cancel_keeps_ledger_deadline_fuel_and_exact_heap`. Cargo stopped there, so the later targets and the step unit tests did not run.
- Cause not established. These tests exercise the decoder and receiving scope code (`🛬️owned`, unchanged against HEAD, and the root), none of which this carve rewrote; the crate could not reach the compiler for tests before, so there is no earlier green run to compare. The child-source transplant I moved is not used by these tests.
- Third run, `test --no-fail-fast` (`verify-test5.log`), is queued to cover every target.
