STATUS: W4 PLAN CURRENT as of 08:11 Nov 3 (197 1102 patch files; 21 pending, rest applied/superseded/held); act20 ENDED; compile UNVERIFIED (no cargo).

# 1102 Patch Window W4 Plan (lane winplan)

Method as in `1102-window-plan.md` (W3). Tools: `1102-tools/winplan/{w4.py,sandbox.py,w4-notes.json,drift.py}`; regenerate with `python3 1102-tools/winplan/w4.py`. Diffs: `git apply --check` forward and `-R` (applied) plus a '+'/'-' line presence check for drifted ones. Scripts/plans: each python script is run in a copy-on-read sandbox (`sandbox.py`: reads copy the repo file into a scratch dir, writes go to scratch, the repo is never touched); rc 0 = applies now, AssertionError = already applied or drifted. Raw: `🗑️generated/1102/winplan-w4.json`.

## 0. Verdict

- Pending now: 2 runtime-behaviour fixes (`runtime/apply-genesis-pack-batch.py`, `energy/effect-only-action-producer.py`), 1 test-only manifest edit (`energy/editor-test-dev-dependency.py`), the policy W4 set (13 diffs in `policy/patches/w4/`, panic/unwrap removal, no behaviour change intended), 2 fem scripts (`fem3d-test-devdeps.py` test manifests, `model-source-extent.py` detaches tests), 1 build-profile change needing the coordinator's decision (`actinfra/prototype-wasm-debug0.py`) and 2 optional [DEBUG] scripts. Every diff from W3 is in the tree (exactly, or with later edits on top; section 6). The W3 plan `1102-window-plan.md` is closed.
- All pending diffs apply in sequence on a scratch copy (`seq.sh` with `order-w4.txt`: 13 OK / 0 FAIL) and all pending scripts apply in one sandbox tree; the only same-file pair is `debug-genesis-measure` vs `genesis-pack-batch` (excluded, section 3). Policy W4 `draw-drawing-model` needs the new `RetainedOwnedProjection::try_project` from W4 `framework-value`, and `os-flow-host` consumes `try_push_cold` from W4 `artifact-flow-flow`: the order encodes both.
- NOT FOUND in `1102-tools/*/patches/` at the time of writing (named in the coordinator message): stale harness `renew_turn`, testport construction-recipient fix, retained-command test visibility. They are not written yet (or live outside `patches/`); the refresh picks them up as UNCLASSIFIED/PENDING. gis `store-replay-retire-unborn-registry-birth.patch` IS applied (reverse check passes).
- Priority rule used: runtime-behaviour fixes on the browser path first (B1 genesis, B2 energie), then test-only, then build profile, optional debug last.

## 1. Pending patches (the W4 work)

| lane | patch | kind | batch | writes | note |
|---|---|---|---|---|---|
| actinfra | `prototype-wasm-debug0.py` | script | B6 | 2 file(s) | BUILD PROFILE (needs coordinator decision): `prototype-wasm.debug` 1 -> 0 in profile schema + 2 consumption fixture rows (takes the repo root as argv[1]); removes ~330 MB DWARF per component; invalidates every prototype-wasm cache key, so apply only when the next activation starts from the patched tree |
| energy | `editor-test-dev-dependency.py` | script | B2 | 1 file(s) | TEST-ONLY manifest edit: adds `semio-framework-trace` dev-dependency to the energy editor Cargo.toml (lib-test target includes initialization tests using `observe_heap_allocations_on_this_thread`); Cargo.lock refresh by the coordinator (`dependencies lock`) |
| energy | `effect-only-action-producer.py` | script | B2 | 3 file(s) | RUNTIME FIX (energie pane): actions without a native mutation descriptor (setActiveExample, other authored verbs) fall through to the typed `command_from_action` instead of failing; adds law `an_action_without_a_native_descriptor_falls_through_to_its_typed_command`; script uses glob lookups asserting exactly one match; writes 2+ files in the energy editor `🎮️commands/🛬️action` |
| fem | `fem3d-test-devdeps.py` | script | B5 | 3 file(s) | TEST-ONLY manifests (replaces the withdrawn `panels-testing-laws-devdep.py`): `semio-framework-plugin` with feature artifact-app-testing in fem 3d editor panels + interaction dev-deps, `semio-framework-os-kernel` protocol-laws/mutation-testing in mutations dev-deps; coordinator runs `dependencies lock` after |
| fem | `model-source-extent.py` | script | B5 | 1 file(s) | detaches the unit-test module (`#[cfg(test)] mod tests;`) from the fem 3d `🧬️schema/🔺️diff/🦀️.rs`; the unreferenced tests file/dir must then be deleted by hand; scratch-applies; REMOVES TESTS from the build (see risks) |
| policy | `semio-framework-artifact-flow-flow.patch` | diff | B3 | 1 file(s) | policy W4 (panic/unwrap removal): flow retained: 3 expects + 2 panics removed; typed try_push_cold |
| policy | `semio-framework-os-flow-host.patch` | diff | B3 | 1 file(s) | policy W4 (panic/unwrap removal): flow host-close: [DEBUG] eprintln and 5 expects removed; typed try_retire_cold, retire_cold keeps its infallible signature |
| policy | `semio-framework-os-store-replay-operation.patch` | diff | B3 | 1 file(s) | policy W4 (panic/unwrap removal): os-store replay report: 3 unwraps -> VcsError::HistoryReplaying |
| policy | `semio-framework-plugin-instance-docs.patch` | diff | B3 | 1 file(s) | policy W4 (panic/unwrap removal): maintenance ladder: expect -> ValueError (PluginLifecycleFailure: From<ValueError>) |
| policy | `semio-framework-value.patch` | diff | B3 | 1 file(s) | policy W4 (panic/unwrap removal): framework-value retained-clone: RetainedOwnedProjection::try_project (definition for the draw lookup) |
| policy | `semio-s-artifact-draw-drawing-geometry.patch` | diff | B4 | 1 file(s) | policy W4 (panic/unwrap removal): draw picking paint: unreachable -> Err(String) |
| policy | `semio-s-artifact-draw-drawing-model.patch` | diff | B4 | 3 file(s) | policy W4 (panic/unwrap removal): draw lookup: expect -> ValueError::invariant; draw lookup owned: 2 expects + unreachable; uses new RetainedOwnedProjection::try_project; draw validation: expect -> fault(); draw snapshot: retire_decoded_drawing_snapshot expects/panic -> typed try_ core |
| policy | `semio-s-artifact-draw-drawing-scene.patch` | diff | B4 | 1 file(s) | policy W4 (panic/unwrap removal): draw raster scene: matches!+let-else(unreachable) -> if let (unrepresentable) |
| policy | `semio-s-artifact-procedural-3d-editor.patch` | diff | B4 | 1 file(s) | policy W4 (panic/unwrap removal): generation3d command_from_action returns Result<_,Fault>: 3 unwraps -> ? (Fault) |
| policy | `semio-s-artifact-procedural-3d-viewer.patch` | diff | B4 | 1 file(s) | policy W4 (panic/unwrap removal): generation3d command_from_action returns Result<_,Fault>: 3 unwraps -> ? (Fault) |
| policy | `semio-s-artifact-raster-raster-model.patch` | diff | B4 | 1 file(s) | policy W4 (panic/unwrap removal): raster asset-content: close_raster_owner_cold expects -> typed try_close_raster_owner_cold |
| policy | `semio-s-artifact-stdio-ifc-2x3-base-details.patch` | diff | B4 | 1 file(s) | policy W4 (panic/unwrap removal): ifc work.rs:20/1404: unwrap -> ValueError::invariant |
| policy | `semio-s-artifact-stdio-ifc4x3-templates.patch` | diff | B4 | 1 file(s) | policy W4 (panic/unwrap removal): ifc4x3 staging-native: receiver hoisted out of the closure |
| runtime | `apply-debug-admission-trace.py` | script | B7 | 2 file(s) | OPTIONAL [DEBUG]: rate-limited eprintln in job outcome worker `admit_step` and plugin job session `prepare_pending_params`; only if runtime asks, remove before ticket close |
| runtime | `apply-debug-settle-cause.py` | script | B7 | 1 file(s) | OPTIONAL [DEBUG]: eprintln in the laws settlement fn (registered-app-construction file); only if runtime asks, remove before ticket close |
| runtime | `apply-genesis-pack-batch.py` | script | B1 | 1 file(s) | RUNTIME FIX (browser path): shared Store genesis advances the Pack cursor one unit per turn, so every WindowConfigOwner birth (~45) exhausts the 32768-turn fixture/host opportunity; patch spends a bounded funded batch of cursor units per turn. Fail-closed on 2 anchors (`cursor_step(cx,false/true)`). Single file `🌱️initialization/🌱️genesis/🦀️.rs` |

## 2. Held / absent / not patches

- HELD: runtime `NOT-APPLIED-puzzle3d-window-config-pack-receiving.py` + puzzle `window-config-pack-receiving.rs.txt`: needs the pack-receiving traits on the whole puzzle3d window-config tree (author: not compile-atomic). The puzzle3d window-config birth fault stays fixed only for the `initial_state` trio. Owner runtime/puzzle must supply the complete tree patch before it can be batched.
- EXCLUDED: `apply-debug-genesis-measure.py` (conflicts with genesis-pack-batch).
- SUPERSEDED: energy `hydration-binds-job-authority.py` (by stale), actinfra `retention-intermediates.diff` (by norows), `worker-spare-instances.diff` (by worker-demand), rtpanes `laws-publish-...md` (by testkit).
- ABSENT: stale-harness `renew_turn`, testport construction-recipient, retained-command test visibility (see verdict).

## 3. File-overlap matrix (pending x pending, by written file)

| file (tail) | patches | verdict |
|---|---|---|
| none | | |

Declared/observed collisions (sequential sandbox run):

- `runtime/apply-genesis-pack-batch.py` x `runtime/apply-debug-genesis-measure.py`: same file `🌱️initialization/🌱️genesis/🦀️.rs`; sequential sandbox run: measure first -> pack-batch fails its asserts; pack-batch first -> measure fails. MUTUALLY EXCLUSIVE; pack-batch wins.
- `runtime/apply-debug-genesis-measure.py` partially writes (store-owners file written before the second file's assert fails): non-atomic if it is ever run after pack-batch; do not.
- `energy/effect-only-action-producer.py` x `energy/editor-test-dev-dependency.py`: different files (rs vs Cargo.toml); both applied cleanly in one scratch tree together with every other pending script.
- Sequence B1..B5 run in ONE scratch tree: all pending scripts applied except the excluded/held ones (asserts only failed for the excluded debug-genesis-measure).

## 4. Ordered batches (single owner, narrow check)

### B1 runtime: shared Store genesis batches Pack cursor units per turn (owner: `runtime`)
1. `1102-tools/runtime/patches/apply-genesis-pack-batch.py` (run from the repo root)
- Check: wasm `--lib` check of `semio-framework-plugin` (genesis + window-config crates), then the native window-config / genesis lib tests and the runtime lane's ledger test (`settlement_work_ledger_is_finite_and_progresses`, editor-task-resume, editor-action-producer); browser effect: a puzzle3d/energie window command settles instead of re-queueing 32768 turns
- Why here: Shared path used by all ~45 WindowConfigOwner births; every pane's first window command waits on it. Framework layer goes before plugin patches. No limit is raised (bounded page-sized batch, stops on yield/completion).

### B2 energie pane: effect-only action fall-through and test dev-dependency (owner: `energy`)
1. `1102-tools/energy/patches/effect-only-action-producer.py` (repo root; fail-closed globs)
2. `1102-tools/energy/patches/editor-test-dev-dependency.py` (repo root; Cargo.toml edit: tell the coordinator for `dependencies lock`)
- Check: wasm `--lib` check of the energy model editor crate, then native `test -p <energy editor> --lib an_action_without_a_native_descriptor_falls_through_to_its_typed_command` and the energy editor-initialization tests (heap-allocation witness)
- Why here: Production edit first, manifest after (the dev-dep only matters for the lib-test target). After B1 so the energie window command can be exercised end to end.

### B3 policy W4 framework: value, instance docs, store replay-operation, flow, flow host (owner: `policy`)
1. `1102-tools/policy/patches/w4/semio-framework-value.patch`
2. `1102-tools/policy/patches/w4/semio-framework-plugin-instance-docs.patch`
3. `1102-tools/policy/patches/w4/semio-framework-os-store-replay-operation.patch`
4. `1102-tools/policy/patches/w4/semio-framework-artifact-flow-flow.patch`
5. `1102-tools/policy/patches/w4/semio-framework-os-flow-host.patch`
- Check: native `check -p semio-framework-value -p semio-framework-plugin-instance-docs -p semio-framework-os-store-replay-operation -p semio-framework-artifact-flow-flow -p semio-framework-os-flow-host` (hub manifest), wasm `--lib` for the value and flow-flow crates; order matters (definitions first)
- Why here: Definitions before callers: `RetainedOwnedProjection::try_project` (value) is used by the draw model patch in B4; `try_push_cold` (flow-flow) by the flow host. After B1/B2 because those are the runtime behaviour fixes and must be proven alone first.

### B4 policy W4 plugins: draw, generation3d, raster, ifc (owner: `policy`)
1. `1102-tools/policy/patches/w4/semio-s-artifact-draw-drawing-geometry.patch`
2. `1102-tools/policy/patches/w4/semio-s-artifact-draw-drawing-model.patch`
3. `1102-tools/policy/patches/w4/semio-s-artifact-draw-drawing-scene.patch`
4. `1102-tools/policy/patches/w4/semio-s-artifact-procedural-3d-editor.patch`
5. `1102-tools/policy/patches/w4/semio-s-artifact-procedural-3d-viewer.patch`
6. `1102-tools/policy/patches/w4/semio-s-artifact-raster-raster-model.patch`
7. `1102-tools/policy/patches/w4/semio-s-artifact-stdio-ifc-2x3-base-details.patch`
8. `1102-tools/policy/patches/w4/semio-s-artifact-stdio-ifc4x3-templates.patch`
- Check: wasm `--lib` check of: draw drawing geometry/model/scene, procedural 3d editor and viewer, raster raster-model, ifc 2x3 base details, ifc4x3 templates (one `repo cargo` run per few crates)
- Why here: Plugin layer after the framework batch (draw model uses the B3 definition). Independent crates otherwise.

### B5 fem: test dev-dependencies, then detach stale unit-test module (owner: `fem`)
1. `python3 -I 1102-tools/fem/patches/fem3d-test-devdeps.py` (repo root; 3 Cargo.toml dev-dependency edits, tell the coordinator for `dependencies lock`)
2. `python3 -I 1102-tools/fem/patches/model-source-extent.py` (repo root), then delete the now unreferenced `🧪️tests/🔬️unit/🦀️.rs` and the empty `🧪️tests` dir by hand
- Check: `check --tests` of the fem 3d editor panels, interaction and mutations crates (native), wasm `--lib` of the fem 3d model/diff crate
- Why here: Tests-only change; after the production batches so it cannot mask them. See risks: it deletes test coverage instead of fixing the stale tests.

### B6 build profile: prototype-wasm debug 0 (owner: `actinfra`)
1. `python3 -I 1102-tools/actinfra/patches/prototype-wasm-debug0.py /Users/ueli/Documents/semio` (needs the coordinator's go)
- Check: `bun test ./<profile>/🧪️tests` profile tests (11 pass expected) and `./<profile>/🧪️tests/📥️consumption/` (24 pass expected); the arguments/precedence files fail identically with and without the patch (author)
- Why here: Changes the prototype-wasm cache key of every component: apply only right before the activation that should use it, never between B1/B2 and their own wasm checks (they would rebuild with a different profile).

### B7 optional [DEBUG] diagnostics (runtime lane only if asked) (owner: `runtime`)
1. `1102-tools/runtime/patches/apply-debug-admission-trace.py`
2. `1102-tools/runtime/patches/apply-debug-settle-cause.py`
- Check: native check of `semio-framework-job` (outcome worker) and the plugin job session / laws crates; remove the [DEBUG] lines before the ticket closes
- Why here: Diagnostics only; last so they never ride into a compile verdict of B1-B3. Do NOT add `apply-debug-genesis-measure`.

## 5. Risky patches

- B1 genesis-pack-batch (author): changes the shared lifecycle of every window-config birth from one unit per turn to a funded batch per turn; risk = a unit that posts nothing or a yield point mishandled loops the turn (author says it stops on yield, completion, measured length, or a unit that posts nothing). Touches the hottest framework path: run the full window-config lib tests, not only the check.
- B2 effect-only producer: broadens the fallthrough to `command_from_action` for ALL actions without a native descriptor; wrong typed-command mapping would execute an unintended verb instead of refusing (previously a hard refusal). The new law covers setActiveExample only.
- B3 debug0: build-profile change; invalidates all wasm caches; the arguments/precedence fixtures already fail without it (author), so do not read those failures as caused by the patch.
- Held puzzle3d pack-receiving: until a complete patch exists, puzzle3d window-config native-pair birth is unfixed; the browser pane may still fault on the native-pair path.
- fem `model-source-extent.py`: removes the diff crate's unit tests from compilation (and asks for a manual delete) rather than repairing the stale tests; conflicts with the test-driven rule unless the fem lane replaces the tests; owner should confirm.
- policy W4: author-named highest compile risks are none beyond the generic ones (index notes: draw lookup uses a NEW framework API, `try_project`; raster adds `try_close_raster_owner_cold`; flow host adds `try_retire_cold`): each definition ships in the same or an earlier patch of this order.

## 6. Applied / obsolete inventory (do not apply)

| lane | patch | state |
|---|---|---|
| actinfra | `emitter-index-bound.diff` | APPLIED |
| actinfra | `emitter-request-bytes.diff` | APPLIED |
| actinfra | `emitter-request-bytes.test.ts` | APPLIED (target test/module exists) |
| actinfra | `jco-strip-debug.diff` | APPLIED |
| actinfra | `retention-intermediates-norows.diff` | APPLIED |
| actinfra | `retention-intermediates.diff` | SUPERSEDED by retention-intermediates-norows.diff (applied) |
| actinfra | `retention-size-law.test.ts` | APPLIED (target test/module exists) |
| actinfra | `module.ts` | APPLIED (target test/module exists) |
| actinfra | `test.ts` | APPLIED (target test/module exists) |
| actinfra | `worker-demand-caller.diff` | APPLIED |
| actinfra | `worker-demand.diff` | APPLIED |
| actinfra | `worker-demand.test.ts` | APPLIED (target test/module exists) |
| actinfra | `worker-spare-instances.diff` | SUPERSEDED by worker-demand.diff (applied) |
| cad | `mount-puzzle3d-unmounted-tests.md` | APPLIED |
| cadi18n | `cad-editor-callers.patch` | APPLIED |
| cadi18n | `cad-engine-0.patch` | APPLIED |
| cadi18n | `cad-engine-1.patch` | APPLIED |
| cadi18n | `cad-engine-2.patch` | APPLIED |
| cadi18n | `cad-mode-edit.patch` | APPLIED |
| cadi18n | `cad-mode-window-building.patch` | APPLIED |
| cadi18n | `cad-mode-window-energy.patch` | APPLIED |
| cadi18n | `cad-mode-window-shape.patch` | APPLIED |
| cadi18n | `cad-mode-window-structure.patch` | APPLIED |
| cadi18n | `cad-spec-json-1.patch` | APPLIED |
| cadi18n | `cad-spec-rust-0.patch` | APPLIED |
| cadi18n | `cad-spec-rust-1.patch` | APPLIED |
| cadi18n | `cad-test-languages.patch` | APPLIED (new files exist) |
| cadi18n | `cad-tests-1.patch` | APPLIED |
| cadi18n | `cad-tests-2.patch` | APPLIED |
| cadi18n | `cad-tests-3.patch` | APPLIED |
| cadi18n | `cad-tests-4.patch` | APPLIED |
| cadi18n | `name-derive-test.patch` | APPLIED |
| cadi18n | `name-derive.patch` | APPLIED |
| cadi18n | `name-dwg-test.patch` | APPLIED |
| cadi18n | `name-geometry-1.patch` | APPLIED |
| cadi18n | `name-geometry-2.patch` | APPLIED |
| cadi18n | `name-history-test.patch` | APPLIED |
| cadi18n | `name-import.patch` | APPLIED |
| cadi18n | `name-kit.patch` | APPLIED |
| cadi18n | `name-panels-artifact-test.patch` | APPLIED |
| cadi18n | `name-panels-artifact.patch` | APPLIED |
| cadi18n | `name-panels-inspection-test.patch` | APPLIED |
| cadi18n | `name-panels-inspection.patch` | APPLIED |
| cadi18n | `name-picking-test.patch` | APPLIED |
| cadi18n | `name-sample-scene.patch` | APPLIED |
| cadi18n | `name-terminology.patch` | APPLIED |
| cadi18n | `web-acceptance-1.patch` | APPLIED |
| cadi18n | `web-demonstrator-1.patch` | APPLIED |
| cadi18n | `web-shellhost-0.patch` | APPLIED (new files exist) |
| cadi18n | `web-shellhost-1.patch` | APPLIED (new files exist) |
| cadi18n | `web-shellhost-2.patch` | APPLIED |
| capext | `job-value-runtime-dep.py` | APPLIED |
| describe | `step-geometry-dev-dep.md` | APPLIED |
| energy | `hydration-binds-job-authority.py` | SUPERSEDED by stale/hydration-job-authority.py |
| flowext | `directed-dag-facade-exports.py` | APPLIED |
| flowext | `flow-child-group-work-detached.py` | APPLIED |
| flowext | `flow-examples-dsl-fixture.py` | APPLIED (tree identical to the script text) |
| flowext | `flow-retirement-push-cold-depth.py` | APPLIED |
| gen | `step-context-owner-NOTES.md` | NOT A PATCH |
| gen | `step-context-owner-remount.py` | APPLIED |
| gis | `store-replay-retire-unborn-registry-birth.patch` | APPLIED |
| phase | `order.txt` | NOT A PATCH |
| phase | `semio-framework-actor-retained-turn.patch` | APPLIED |
| phase | `semio-framework-job-failure.patch` | APPLIED |
| phase | `semio-framework-job-payload.patch` | APPLIED |
| phase | `semio-framework-job.patch` | APPLIED |
| phase | `semio-framework-os-flow-extensions.patch` | APPLIED, later edited (old lines gone) |
| phase | `semio-framework-os-flow-wasm.patch` | APPLIED |
| phase | `semio-framework-os-history.patch` | APPLIED |
| phase | `semio-framework-os-renderer-wgpu.patch` | APPLIED |
| phase | `semio-framework-os-store-document.patch` | APPLIED |
| phase | `semio-framework-os-store-engine-replay.patch` | APPLIED, later edited (old lines gone) |
| phase | `semio-framework-os-store-initialization-replay.patch` | APPLIED |
| phase | `semio-framework-os-store-replay-operation.patch` | APPLIED |
| phase | `semio-framework-pack-record-retained.patch` | APPLIED |
| phase | `semio-framework-plugin-decl-app.patch` | APPLIED |
| phase | `semio-framework-plugin-decl-emission.patch` | APPLIED |
| phase | `semio-framework-plugin-job-receiving.patch` | APPLIED |
| phase | `semio-framework-plugin-job.patch` | APPLIED |
| phase | `semio-framework-plugin-tool-run-turns.patch` | APPLIED |
| phase | `semio-framework-plugin-window-config.patch` | APPLIED |
| phase | `semio-framework-plugin.patch` | APPLIED |
| phase | `semio-framework-replication-history.patch` | APPLIED |
| phase | `semio-framework-text-splice-native.patch` | APPLIED |
| phase | `semio-framework-value-allocation-return.patch` | APPLIED |
| phase | `semio-framework-value-clone-source.patch` | APPLIED |
| phase | `semio-framework-value-control-contract.patch` | APPLIED |
| phase | `semio-framework-value-derive-expansion.patch` | APPLIED |
| phase | `semio-framework-value-derive.patch` | APPLIED |
| phase | `semio-framework-value-edit.patch` | APPLIED |
| phase | `semio-framework-value-native-owned.patch` | APPLIED, later edited (old lines gone) |
| phase | `semio-framework-value-ordered.patch` | APPLIED |
| phase | `semio-framework-value-paged-list.patch` | APPLIED |
| phase | `semio-framework-value-persistent.patch` | APPLIED |
| phase | `semio-framework-value-retained-map.patch` | APPLIED |
| phase | `semio-s-artifact-draw-drawing-scene.patch` | APPLIED |
| phase | `semio-s-artifact-fem-3d-editor-results-window.patch` | APPLIED |
| phase | `supersedes.txt` | NOT A PATCH |
| policy | `semio-framework-2d-path-flatten.patch` | APPLIED |
| policy | `semio-framework-actor-retained-turn.patch` | APPLIED |
| policy | `semio-framework-actor.patch` | APPLIED |
| policy | `semio-framework-artifact-flow-flow-copy.patch` | APPLIED |
| policy | `semio-framework-dispatch-macros.patch` | APPLIED |
| policy | `semio-framework-job-context.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-job.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-os-flow-host.patch` | APPLIED |
| policy | `semio-framework-os-kernel.patch` | APPLIED |
| policy | `semio-framework-os-services-native-io.patch` | APPLIED |
| policy | `semio-framework-os-services-native-paths.patch` | APPLIED |
| policy | `semio-framework-os-store-codec-registry.patch` | APPLIED |
| policy | `semio-framework-os-store-composition.patch` | APPLIED |
| policy | `semio-framework-os-store-engine-branch.patch` | APPLIED |
| policy | `semio-framework-os-store-engine-command.patch` | APPLIED |
| policy | `semio-framework-os-store-engine-replay.patch` | APPLIED |
| policy | `semio-framework-os-store-ephemeral.patch` | APPLIED |
| policy | `semio-framework-os-store-one-item-batch.patch` | APPLIED |
| policy | `semio-framework-os-store-owned-schema-decode.patch` | APPLIED |
| policy | `semio-framework-os-store-owned-schema-fields.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-os-store-presence.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-os-store-read.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-os-store-replay-operation.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-os-store-replay.patch` | APPLIED |
| policy | `semio-framework-pack-record-retained.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-pixels.patch` | APPLIED |
| policy | `semio-framework-plugin-decl-emission.patch` | APPLIED |
| policy | `semio-framework-plugin-decl-registry.patch` | APPLIED |
| policy | `semio-framework-plugin-instance-jobs.patch` | APPLIED |
| policy | `semio-framework-plugin-instance-typed-ops.patch` | APPLIED |
| policy | `semio-framework-plugin-job-receiving.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-plugin-job.patch` | APPLIED |
| policy | `semio-framework-plugin-reactor-core.patch` | APPLIED |
| policy | `semio-framework-plugin-reactor-turn.patch` | APPLIED |
| policy | `semio-framework-plugin-retained-command.patch` | APPLIED, later edited (old lines gone) |
| policy | `semio-framework-plugin-tool-run.patch` | APPLIED |
| policy | `semio-framework-plugin-window-config.patch` | APPLIED |
| policy | `semio-framework-plugin.patch` | APPLIED |
| policy | `semio-framework-replication-history.patch` | APPLIED |
| policy | `semio-framework-value-derive.patch` | APPLIED |
| policy | `semio-framework-value-native-storage.patch` | APPLIED |
| policy | `semio-framework-value-retained-map.patch` | APPLIED |
| policy | `semio-s-artifact-draw-drawing-io.patch` | APPLIED |
| policy | `semio-s-artifact-draw-drawing-scene-paint.patch` | APPLIED |
| policy | `semio-s-artifact-draw-drawing-scene.patch` | APPLIED |
| policy | `semio-s-artifact-energy-model-session.patch` | APPLIED |
| policy | `semio-s-artifact-fem-2d-editor-session.patch` | APPLIED |
| policy | `semio-s-artifact-gis-gismap-gis2d.patch` | APPLIED |
| policy | `semio-s-artifact-puzzle-2d-mut-any.patch` | APPLIED |
| policy | `semio-s-artifact-puzzle-2d-snapshot.patch` | APPLIED |
| policy | `semio-s-artifact-puzzle-2d-text-inverse.patch` | APPLIED |
| policy | `semio-s-artifact-stdio-ifc-2x3-base-details.patch` | APPLIED |
| policy | `semio-s-artifact-stdio-ifc-4x3.patch` | APPLIED |
| policy | `semio-s-artifact-stdio-ifc-details-guidance.patch` | APPLIED |
| policy | `semio-s-artifact-stdio-ifc-details-provider.patch` | APPLIED |
| policy | `semio-s-plugin-flow-brep.patch` | APPLIED |
| policy | `ts-framework-process-wasm-build.patch` | APPLIED |
| puzzle | `window-config-pack-receiving.rs.txt` | HELD (text of the parked item) |
| rtpanes | `by-example-law-redo.md` | APPLIED |
| rtpanes | `energy-manifest-uses-stdio-semio-edit-mode.md` | APPLIED |
| rtpanes | `laws-publish-framework-schema-documents.md` | SUPERSEDED by testkit patch (applied) |
| runtime | `NOT-APPLIED-puzzle3d-window-config-pack-receiving.py` | HELD (not compile-atomic) |
| runtime | `apply-debug-genesis-measure.py` | EXCLUDED (conflicts with genesis-pack-batch) |
| runtime | `apply-puzzle3d-window-config-birth.py` | APPLIED |
| runtime | `apply-worker-continuation-fault.py` | APPLIED |
| stale | `hydration-job-authority.py` | APPLIED |
| stdio | `decouple-mesh-export-from-pdf.py` | APPLIED |
| testkit | `publish-framework-schemas.patch` | APPLIED |
| testport | `cursor-dev-vcs-testkit.py` | APPLIED |
| testport | `durable-group-test-visibility.py` | APPLIED |
| testport | `member-open-dictionary-public-types.py` | APPLIED |
| testport | `paged-projection-any-output.py` | APPLIED |
| uii18n | `cad-step-labels-1.patch` | APPLIED |
| uii18n | `cad-step-labels-2.patch` | APPLIED, later edited (old lines gone) |
| uii18n | `flow-form-labels-1.patch` | APPLIED |
| uii18n | `flow-form-labels-2.patch` | APPLIED |
| uii18n | `procedural-docstring-1.patch` | APPLIED |
| uii18n | `process-status-steps-1.patch` | APPLIED |

## 7. New / unclassified since last refresh

- none

## 8. Refresh log

- 08:11: regenerated; 197 files, 21 pending, 0 unplaced.
- 08:15: FINAL refresh; act20.status has `exit 130 2026-11-03 08:11:30`.
