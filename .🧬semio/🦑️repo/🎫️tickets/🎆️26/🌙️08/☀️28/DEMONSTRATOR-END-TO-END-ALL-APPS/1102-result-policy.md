STATUS: 57 PATCHES PREPARED, apply-check OK for 57/57; COMPILE UNVERIFIED (freeze). 200 of 363 unwrap/expect/panic matches fixed, 133 remain with a per-match blocker in `C/1102-tools/policy/blockers.md`; async_fn_in_trait patched in dispatch + actor.

# 1102 result: policy

Patches: `C/1102-tools/policy/patches/<crate>.patch` (a/ b/ paths from repo root; apply with `git apply <patch>` from `/Users/ueli/Documents/semio`). Generator: `C/1102-tools/policy/{mk,a,b,c,final}.py` (every edit asserts exactly one match, fail-closed). Apply-check = `git apply --check` against the current tree (tree untouched), run at generation time; peers editing the same files can invalidate it, so re-run `final.py` before the window. `patches/_superseded/` holds earlier per-lane-named duplicates, ignore (could not delete).

## Already fixed in the tree (verified by grep of the audited files)
- 9 `[DEBUG]` eprintln: none left in production source (1101 policyfix). Remaining `[DEBUG]` strings are only inside tests (`🧪️tests/`: fem sparse unit eprintln, raster/norm/en1997 sqlite test scripts, assert messages); test files may be edited now, not patched here.
- `#[allow(clippy::too_many_arguments)]` in job/⏱️context and generated `#![allow(async_fn_in_trait)]` in the hub stdio composition: gone. Pre-existing (not new) `allow(async_fn_in_trait)` remain in `🔀️dispatch/🦀️.rs:294`, `🔀️dispatch/📦️packages/🦀️rust/🦀️.rs:12`, `🎭️actor/🎟️retained-turn/📦️packages/🦀️rust/🦀️.rs:2` and dispatch tests: owner must change the trait shape (return `impl Future + Send` instead of `async fn`); not patched.

## Freeze audit (who removed the [DEBUG] lines and allows)
- Removed by the 1101 policyfix lane before the freeze; no file of this lane's patch set was modified at or after 13:03 Nov 2 (checked with `find -newermt '2026-11-02 13:03'` over all 38 patched files: none). Last edits: 🔁️stall-witness-and-unit-advance/🦀️.rs, 👷️worker/🦀️.rs, 🚪️session/🦀️.rs (all Nov 1 22:29), 🔄️full-operation/🦀️.rs (Nov 2 01:02, 3 `[DEBUG]` lines removed, 5 added lines), all under `🧰️framework/` (`🛍️products/💻️os/🔨️modules/🔌️plugin/...` and `🔨️modules/🧵️job/📬️outcome/`). `⏱️context/🦀️.rs` (Nov 1 13:16) and `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts` (Nov 1 01:36) already have no allow in HEAD.

## async_fn_in_trait (fixed by trait shape, patches `semio-framework-dispatch-macros` and actor crates)
- `#[dyn_enum]` re-emits each trait with every `async fn` declared as `fn .. -> impl ::core::future::Future<Output = T>` (default bodies wrapped in `async move`); the generated `Delegate` trait defaults use the same shape and lose their `#[allow]`. Implementors may still write `async fn`; `.await` call sites and `Send` stay unchanged (R3, structural Send).
- Removed: dispatch packages crate-root allow + R7 doc, 6 dispatch test crate-root allows (test files), actor retained-turn packages allow (its only trait is sync), actor packages allow; `ShardTransport` (4 `async fn`) rewritten to the same RPITIT shape.
- NOT patched (outside dispatch/actor): crate-root allows in `🎒️pack/📦️packages`, `🎒️pack/🌐️http`, `🎒️pack/⏳️async`, `📡️replication/🔐️crypto`, `🧮️math/🎯️sampling/*` (6 files); each needs the same trait-shape rewrite in its own traits once the macro patch is in.
- Risk: proc-macro change is unverified; check `semio_framework_dispatch_macros` tests (`🧪️tests/*`) first, then every `#[dyn_enum]` user.

## Patch table
| crate | files | edits | apply-check |
|---|---|---|---|
| semio-s-artifact-stdio-ifc-2x3-base-details | 2 | 7 | OK |
| semio-s-artifact-stdio-ifc-details-provider | 1 | 1 | OK |
| semio-s-artifact-gis-gismap-gis2d | 1 | 1 | OK |
| semio-s-artifact-puzzle-2d-text-inverse | 1 | 2 | OK |
| semio-framework-os-store-replay | 2 | 12 | OK |
| semio-framework-os-store-replay-operation | 4 | 12 | OK |
| semio-s-plugin-flow-brep | 1 | 1 | OK |
| semio-s-artifact-fem-2d-editor-session | 1 | 4 | OK |
| semio-s-artifact-energy-model-session | 2 | 5 | OK |
| semio-s-artifact-draw-drawing-io | 1 | 1 | OK |
| semio-s-artifact-stdio-ifc-4x3 | 1 | 5 | OK |
| semio-s-artifact-stdio-ifc-details-guidance | 1 | 3 | OK |
| semio-framework-os-store-presence | 2 | 3 | OK |
| semio-framework-os-store-one-item-batch | 1 | 3 | OK |
| semio-framework-os-store-ephemeral | 2 | 3 | OK |
| semio-framework-os-store-engine-command | 1 | 2 | OK |
| semio-framework-os-store-engine-replay | 4 | 9 | OK |
| semio-framework-os-store-engine-branch | 1 | 3 | OK |
| semio-framework-plugin-job-receiving | 3 | 12 | OK |
| semio-framework-plugin | 3 | 14 | OK |
| semio-framework-plugin-reactor-turn | 1 | 2 | OK |
| semio-framework-plugin-reactor-core | 2 | 6 | OK |
| ts-framework-process-wasm-build | 1 | 5 | OK |
| semio-framework-2d-path-flatten | 1 | 1 | OK |
| semio-framework-value-native-storage | 1 | 2 | OK |
| semio-framework-pack-record-retained | 1 | 1 | OK |
| semio-framework-pixels | 1 | 1 | OK |
| semio-framework-job | 2 | 4 | OK |
| semio-framework-artifact-flow-flow-copy | 1 | 1 | OK |
| semio-framework-os-store-codec-registry | 1 | 1 | OK |
| semio-framework-os-store-composition | 1 | 1 | OK |
| semio-framework-plugin-job | 2 | 3 | OK |
| semio-framework-plugin-decl-registry | 1 | 2 | OK |
| semio-framework-plugin-window-config | 1 | 10 | OK |
| semio-framework-dispatch-macros | 8 | 11 | OK |
| semio-framework-actor-retained-turn | 2 | 2 | OK |
| semio-framework-actor | 2 | 2 | OK |
| semio-framework-value-derive | 1 | 1 | OK |
| semio-s-artifact-puzzle-2d-mut-any | 1 | 3 | OK |
| semio-s-artifact-puzzle-2d-snapshot | 1 | 1 | OK |
| semio-framework-value-retained-map | 1 | 1 | OK |
| semio-framework-os-flow-host | 2 | 8 | OK |
| semio-framework-plugin-tool-run | 1 | 4 | OK |
| semio-framework-plugin-decl-emission | 1 | 4 | OK |
| semio-framework-plugin-instance-jobs | 1 | 1 | OK |
| semio-framework-plugin-instance-typed-ops | 1 | 1 | OK |
| semio-framework-plugin-retained-command | 1 | 1 | OK |
| semio-framework-os-store-read | 2 | 2 | OK |
| semio-framework-os-kernel | 2 | 2 | OK |
| semio-framework-job-context | 1 | 2 | OK |
| semio-framework-os-services-native-paths | 1 | 2 | OK |
| semio-framework-os-services-native-io | 1 | 7 | OK |
| semio-s-artifact-draw-drawing-scene-paint | 2 | 4 | OK |
| semio-s-artifact-draw-drawing-scene | 1 | 1 | OK |
| semio-framework-replication-history | 1 | 2 | OK |
| semio-framework-os-store-owned-schema-decode | 1 | 1 | OK |
| semio-framework-os-store-owned-schema-fields | 3 | 4 | OK |

## Console leftovers
- wasm-build `console.log` x3 (framework/process): patched `ts-framework-process-wasm-build` (optional `report?:(line)=>void` on `WasmPackWebBuildOptions`, callers opt in).
- Not patched, classified as CLI edge output (stdout is the command's contract): repo `dependencies/truth` testkit gate, `commands/schema` compile report, native `port` `log` (docstring: the standard log), stdio `pack-tables.ts` / `pack-all.ts` (`bun` generator mains).

## Audit rework (C/1102-audit-patchset.md rows 1-8)
The 8 flagged `.unwrap()` were pre-existing unwraps on lines my edits touched. All are removed: engine-branch (`branch` hoisted before `retired`), engine-replay repin (`key` hoisted, Copy), store-replay supersession (`take_rejection().ok_or_else(invalid)?` inside the funded closure; `owner` hoisted before `close_original_retained_step`), job-receiving pack (`let Some(admission)=.. else {return Err(invalid())}` inside the closure, since `self.admission.take()` forbids an outer borrow), reactor-core metadata (`receiver` hoisted out of both closures, disjoint from `self.phase`/`self.view`), ifc2x3 field_edit (`collection.get_mut(index).ok_or_else(invalid)?`, `output.ok_or_else(invalid)?`). `C/1102-tools/policy/post.py` re-scans every changed line of every patched file after applying the edits: 0 `.unwrap()/.expect(/panic!/unreachable!` introduced or left on a changed line.

## Additional designs in this round
- Existing `RetainedCloneRef::try_project` (fallible projection, already in framework-value) now used for puzzle delete-node inverse and snapshot lookup.
- `RetainedWorkOutcome{result:Err(ValueError::invariant(..)),work:0}` returned via let-else for store presence/read/job-context/origin close steps (the outcome's own error channel replaces the unwrap).
- `drop(self.cursor.take().unwrap())` is `drop(self.cursor.take())` (no invariant needed).
- Tool-run tick, follow, capture, typed-ops, window-config pack load, transient publication, flow host: Option owners propagate `ValueError` via the file's `invalid(..)`.

## Apply ordering (for lane winplan)
All 57 patches are independent per crate (each applies to the current tree on its own, `git apply --check` OK; do not reorder files inside one patch). Cross-crate dependencies:
1. `semio-framework-dispatch-macros` first (macro now emits `fn .. -> impl Future`), then the dispatch test-crate edits (same patch), then actor (`semio-framework-actor`, `-actor-retained-turn`; `ShardTransport` RPITIT) in any order.
2. `semio-framework-value-*` crates before their callers is not required: no patch adds or changes a definition; every patch uses existing APIs (`ValueError::invariant`, `RetainedCloneRef::try_project`, `SchemaDecodeDiagnostic::with_native`, the file's own `invalid(..)`/`refusal(..)`).
3. Window order: framework-value/job/os-store patches, then plugin patches, then `✏️s` artifact patches, then the TS patch. After applying a group run the narrow wasm `--lib` check of its crates.

## Round 4 additions (this reply)
- Phase machines with an error channel now return typed errors instead of `unreachable!()`: native-io rows/read (13 arms, `ValueError::invariant` -> `OriginalTurnFailure`), draw scene paint/prepare/placement (`DocumentSceneError::literal`), actor source, replication fold cleanup, plugin job-receiving pack/transaction.
- Owned-schema decoders: no new `From<ValueError>` was needed or added; `SchemaDecodeDiagnostic::with_native(ValueError)` already converts and the in-scope `diagnostic` is used (5 sites); a blanket `From` would need `Path: Default`, which `OwnedSchemaPath` does not guarantee.
- Flow host close steps: receivers hoisted out of the close closures (8 sites).

## Remaining 133 matches: not done, with the reason (not "ripple" in general)
Per match: `C/1102-tools/policy/blockers.md`. By class:
```
29 closure/outcome signature: needs fallible variant of the API (ref-returning closure or RetainedWorkOutcome)
24 test-support module (move under 🧪️tests/ or cfg(test))
12 embedded-resource/drop-path panic: Result at construction
11 phase/state machine: typed enum makes arm unrepresentable
4 arithmetic/layout bound: saturating or typed bound error
3 proc-macro: emit syn::Error
```
Why I stopped short of the coordinator's list:
- (a) integer/string phase machines: the 29 are down to the sites with no error channel (text-splice getters, value list/splice/persistent/build/ordered, source admission, payload, derived refusal, trace): each needs an `enum Phase` whose field is assigned across a whole minified file, and the owners return `RetainedWorkOutcome`/`&T`/`Node<V>`. I will not guess these without a compile check.
- (b) test-support (24): they are separate dev-only crates (`semio-s-artifact-stdio-contract-testing`, `…os-io testing`, fem `cold-laws`, puzzle jobs). Moving them under `🧪️tests/` changes crate paths and the repo-generated workspace membership; that must go through `repo` tooling, and I was not able to run it under the freeze. No direct tree edit was made.
- (c) done as described above (no blanket `From`).
- (d) Option owners in closures: 35 more would need hoisting out of closures that also mutate the same field (`take()` after use), or widening non-Result signatures (value-ordered plan fns, payload/page-transfer, history fold `Poll`, wgpu source). Those are the sites left.
- (e) embedded resources (12): compile-time validation of a decoded pack needs const decoding; a `Result` accessor ripples through every `ifc4x3_*()`/`embedded_*()` caller in the stdio ifc crates. Not patched.
- Arithmetic (4+6): needs `RetainedCloneProgress::saturating_add` in framework-value first (definition), then the five `checked_add(..).expect(..)` closures in native-owned; not patched.

## Verification needed in the window
Narrow wasm `--lib` checks per patched crate. Highest compile risk: dispatch macros (`without_async_fn`, `parse_quote!` forms), energy `run-job`, fem2d session, ifc2x3 `field_edit`, os-store-replay, hoisted receivers (reactor-core metadata, engine-branch, flow host: rely on disjoint-field closure captures), the `RetainedWorkOutcome` let-else returns (assume default error type `ValueError`), and native-io (state is left `Finished` when an arm now errors).

## Patch window status (updated by lane policy)
- B9 (TS wasm-build report callback): applied, `bun test` of `🧪️tests/🟦️.ts` 6 pass, targeted `tsc --noEmit` exit 0 (`🗑️generated/1102/polB9-tsc.txt`); marker `win-B9-done`.
- B1 (29 patches): applied in plan order; checks `W-b1-3` (hub native, 27 crates) rc 0, `W-b1-w` (hub wasm32-wasip2: plugin, plugin-job, plugin-job-receiving, value-native-storage, job) rc 0, `W-b1-f2` (root `Cargo.toml`: `semio-framework-os-services-native-io`, `-native-paths`, which are not in the hub workspace) rc 0; marker `win-B1-done`. No manifest changed by my patches. The job-context `semio_framework_value::` concern was already resolved in the tree (origin file uses the imported `ValueError`).
- B2 (9 patches): applied; `W-b2-1` failed on one real error (presence `🚫️rejection/🦀️.rs:115` returned a `RetainedWorkOutcome` from a `Result` closure; now `return Err(ValueError::invariant(..))`, source `g.py` updated); `W-b2-3` (10 crates, hub native) rc 0; marker `win-B2-done`.
- B5 (10 plugin patches) + the gen/mutpolicy scripts: DEFERRED by the coordinator to the window after act19; not applied. Re-verified 10/10 policy B5 patches `git apply --check` OK on the current tree at the time of writing (no drift; nothing refreshed).
- Correction for `os-store-owned-schema-fields`: my `field-decoder-registry-and-empty-array-authority` edit used `diagnostic`, which in that file is a `|code|` closure of another fn, not in scope in `close_step` (the other three owned-schema files do have `diagnostic` in scope: verified at lines 274/266/…). uii18n's replacement `self.diagnostic("artifact-envelope.history-factory-absent", 0).with_native(ValueError::invariant("history value factory is absent"))` is correct: `self.diagnostic(code, offset)` is the inherent builder already used by sibling methods in that impl (`artifact-envelope.history-entry-*`), and the code string follows the same family. `policy/patches/semio-framework-os-store-owned-schema-fields.patch` is therefore stale for that one hunk but already applied; do not re-apply or regenerate it.

## Window W3: B5 (lane policy)
Applied: 10 policy patches + gen `brep-boolean-funded-phases.patch` + `gen-host-displaced.py` + `raster-model.py`; the energy manifest one-liner was already in the tree. Checks (wasm32-wasip2): hub `W-b5-2` and flow workspace `W-b5-4` (rc 0). Fixed in-window: `aabb-math-and-stage` still called `step_unbudgeted` (now `step_budgeted`); flow-brep's `BrepBooleanStep` match lacked the new `Failed` arm (returns `ValueError::literal(InvalidValue, ..)`). Not verifiable: `draw-drawing-{io,scene,scene-paint}` because `semio-s-artifact-draw-drawing-model` fails with 56 errors from the B1P `NativeDefault` change (phase must port `🛬️receive/🦀️.rs` and `📸️snapshot/*`). Marker `win-B5-done`; details `🗑️generated/1102/win-B5-note.txt`.

## W4 (act20 running: patches only, `C/1102-tools/policy/patches/w4/`, generator `w4.py`) - reworked, no wrappers
13 patches, all `git apply --check` OK against the current tree; compile UNVERIFIED. Added lines carry no `unwrap/expect/panic!/unreachable!/eprintln/[DEBUG]`.

| crate | files | edits | apply-check |
|---|---|---|---|
| semio-framework-os-flow-host | 1 | 1 | OK |
| semio-framework-artifact-flow-flow | 1 | 1 | OK |
| semio-framework-os-store-replay-operation | 1 | 3 | OK |
| semio-framework-plugin-instance-docs | 1 | 1 | OK |
| semio-s-artifact-procedural-3d-editor | 1 | 1 | OK |
| semio-s-artifact-procedural-3d-viewer | 1 | 1 | OK |
| semio-s-artifact-stdio-ifc-2x3-base-details | 1 | 1 | OK |
| semio-s-artifact-stdio-ifc4x3-templates | 1 | 1 | OK |
| semio-s-artifact-raster-raster-model | 1 | 2 | OK |
| semio-s-artifact-draw-drawing-scene | 1 | 1 | OK |
| semio-s-artifact-draw-drawing-geometry | 1 | 1 | OK |
| semio-s-artifact-draw-drawing-model | 3 | 4 | OK |
| semio-framework-value | 1 | 1 | OK |

Apply order: `semio-framework-value` (adds `RetainedOwnedProjection::try_project`), then everything else. The three Result-returning definitions (rows `semio-framework-os-flow-host`, `semio-framework-artifact-flow-flow`, `semio-s-artifact-raster-raster-model`) break their call sites until the compiler-driven rewrite ran, so they are one atomic step: apply those three patches, then run `C/1102-tools/policy/ripple.py` in a loop (check with `--message-format=json` via `chk.sh W-ripple-N ...`, then `python3 ripple.py <chk output txt> <epoch of the compile start>`) until `rewritten 0`; `<out>.manual` lists what it could not decide (an E0277 `?` in a fn that does not return a Result must be widened by hand).

Decision per API (the earlier infallible `retire_cold` wrappers that discarded the Result are removed):
| API | call sites | option | what |
|---|---|---|---|
| `FlowRetirement::push_cold` | 13 (def 1, `🔺️diff` `handoff` 9, host-authoring 2, test 1) | (a) | now `-> Result<(),ValueError>`; the two host-authoring callers return `Result<(),FlowCoreError>` (`From<ValueError>` exists at `⚠️errors:175`) so they get `?`; `FlowDelta::handoff` must return `Result` and `MutationDiff::retire_cold(self)` (trait method, infallible signature, implemented per mutation diff) is the blocker below |
| `FlowHost::retire_cold` | about 690 textual `.retire_cold()` hits in 42 flow files, mostly tests, receiver type not decidable by text (Widget/Gui/Tree/Host all share the name) | (a), compiler-driven | now `-> Result<(),ValueError>`, `[DEBUG]` and 5 expects gone; call sites via `ripple.py` |
| `close_raster_owner_cold` / `retire_raster_value_cold` | 4 / about 39 (production: panels inspection 1, sqlite snapshot 1; the rest tests) | (a), compiler-driven | both `-> Result<(),ValueError>`; the `panic!` and `assert!` in them are errors now |
| `retire_decoded_drawing_snapshot` | 5 (derive hook + sqlite snapshot) | OPEN, not patched | it is the `retire_with` hook expanded into `fn retire_decoded(self) { .. }` in `✨️derive/⚙️expansion/🎯️expand-entries:551`; an infallible trait signature, so option (a) needs the trait + derive + every `retire_with` user, option (b) needs a framework drain that cannot fail; I removed my wrapper patch and left the original code (1 of the 18 audited hits stays) |
| `MutationDiff::retire_cold(self)` / `retire_projection` (FlowDiff impl uses `push_cold`) | trait-mandated, implementors not counted (a repo-wide `git grep` timed out) | OPEN | same as above: the trait signature must become `Result` first (definition-first, all implementors) |

Other hits in the same 18, unchanged from the first W4 report (typed errors with `?`, unrepresentable restructures, `ValueError::invariant`, `Fault`, `VcsError::HistoryReplaying`), and the limits comparison (no raise; only `PREVIEW_FUEL` is new and caps a previously unbounded loop). `generation3d_retire_displaced_cold` is test-gated.
