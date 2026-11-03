STATUS: AUDIT DONE for act15-17 (history reconstructed from logs; act17 cancelled, so its outcome is partial). act18 live at 27/77 at 13:14, no failures yet. No edits, no runs.

# 1102 Audit: Activation History of `@semio-tech/mit-bestand-demonstrator:activate-prototype`

Read-only audit by aud-act. Sources: `C/🗑️generated/1101/act15.log`, `act16.log`, `act17.log` (+ `.status`), `C/🗑️generated/1102/act18.log` (+ `.status`), Bazel build graph `.cache/b/p/m/bazel/workspaces/s2/0000000000000000/BUILD.bazel` (Nov 1 23:55, the graph act17 ran against), and the Bazel failure tails under `workspaces/*/failures/`.

## Runs

| Run | Start | Exit | Wall time | Bazel critical path | Max progress | Failed actions | Outcome |
|---|---|---|---|---|---|---|---|
| act15 | 2026-11-01 12:51:49 | 1 (14:59:50) | 5964 s | 2611.5 s | 77/77 "no actions running" | 3 | Build failed (keep-going); 27 passed, 3 failed, 45 not reached |
| act16 | 2026-11-01 15:04:06 | 1 (19:47:41) | 14539 s | 4113.6 s | 77/77 "no actions running" | 3 | Build failed (keep-going); 27 passed, 3 failed, 45 not reached; long capacity and deadline waits |
| act17 | 2026-11-01 23:41:09 | 130 (01:02:04, SIGINT) | about 81 min | not reported | 22/77 | 2 | Cancelled by operator after the slabs and hub-energy failures; the rest is unknown |
| act18 | 2026-11-02 13:03:04 | running | about 11 min at 13:14 | not yet | 27/77 at 13:14:25 (last log write) | 0 so far | In progress, no ERROR lines yet |

Pass inference: Bazel reached 77/77 with "no actions running" in act15 and act16, so every action that was not failed or gated by a failure completed. The 27 passed per run is therefore a count, not a guess. Actions gated by a failure are "not reached". Bazel prints no per-action pass line for cache hits, so passes are inferred this way. act17 was cancelled, so its completions cannot be confirmed.

## Action table (76 repo_task nodes in the graph, 75 actions plus the top request)

Gates = number of actions that transitively depend on this one (its blast radius). Level = longest chain of actions below it (0 = root). The brief says 77 actions; the BUILD graph has 76 repo_task nodes including the top request, so one action is not a repo_task (likely a launch or noop). It is not visible in the logs.

| # | Label | Kind | Lvl | Gates | act15 | act16 | act17 | act18 (13:14) |
|---|---|---|---|---|---|---|---|---|
| 1 | `@semio-tech/dsl-derive-rs:generate-schema-index:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 2 | `@semio-tech/flow-plugin:graph-generate:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 3 | `@semio-tech/framework-schema:generate:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 4 | `@semio-tech/puzzle-2d-rs:graph-generate:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 5 | `@semio-tech/puzzle-3d-rs:graph-generate:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 6 | `@semio-tech/puzzle-5d-rs:graph-generate:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 7 | `@semio-tech/ui-rs:generate:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 8 | `@semio-tech/ui-styling-tokens:generate:prototype` | generate | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 9 | `repo:generator-inputs:prototype` | generator inputs | 0 | 0 | passed | passed | cancelled, unknown | ok (dependents seen) |
| 10 | `semio-framework-canvas-font-assets:build:prototype` | build | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 11 | `workspace:deps-trunk:prototype` | deps workspace | 0 | 0 | passed | passed | cancelled, unknown | ok (dependents seen) |
| 12 | `workspace:deps-wasm-opt:prototype` | deps workspace | 0 | 0 | passed | passed | cancelled, unknown | not yet |
| 13 | `@semio-tech/framework-editor-rs:wasm:prototype` | wasm | 1 | 1 | passed | passed | cancelled, unknown | not yet |
| 14 | `@semio-tech/framework-os-node-graph-rs:wasm:prototype` | wasm | 1 | 2 | FAIL | passed | cancelled, unknown | not yet |
| 15 | `@semio-tech/framework-plugin-web:support:prototype` | support | 1 | 1 | passed | passed | cancelled, unknown | not yet |
| 16 | `@semio-tech/framework-surface-rs:wasm:prototype` | wasm | 1 | 1 | passed | passed | cancelled, unknown | running/queued |
| 17 | `@semio-tech/os-plugin-describe-rs:build:prototype` | describe | 1 | 2 | FAIL | FAIL | cancelled, unknown | running/queued |
| 18 | `@semio-tech/s-dev-composition:generate:prototype` | generate | 1 | 2 | passed | passed | cancelled, unknown | running/queued |
| 19 | `semio-framework-os-infinite:fonts:prototype` | fonts | 1 | 1 | passed | FAIL | cancelled, unknown | not yet |
| 20 | `workspace:deps-wasm:prototype` | deps workspace | 1 | 2 | passed | passed | cancelled, unknown | ok (dependents seen) |
| 21 | `@semio-tech/plugin-registry:session-aggregator:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 22 | `@semio-tech/plugin-registry:session-aussuchen:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 23 | `@semio-tech/plugin-registry:session-bearbeiten:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 24 | `@semio-tech/plugin-registry:session-energy:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 25 | `@semio-tech/plugin-registry:session-fem3d:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 26 | `@semio-tech/plugin-registry:session-generation3d:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 27 | `@semio-tech/plugin-registry:session-generator:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 28 | `@semio-tech/plugin-registry:session-koordinator:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 29 | `@semio-tech/plugin-registry:session-verfolgen:prototype` | lane receipt (session) | 2 | 3 | passed | passed | cancelled, unknown | not yet |
| 30 | `@semio-tech/puzzle-plugin:wasm:prototype` | wasm | 2 | 9 | FAIL | FAIL | cancelled, unknown | running/queued |
| 31 | `semio-hub-cad-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 32 | `semio-hub-demonstrator-component:component:prototype` | component | 2 | 8 | not reached | not reached | cancelled, unknown | not yet |
| 33 | `semio-hub-energy-component:component:prototype` | component | 2 | 7 | not reached | not reached | FAIL | not yet |
| 34 | `semio-hub-fem-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 35 | `semio-hub-flow-component:component:prototype` | component | 2 | 8 | not reached | not reached | cancelled, unknown | not yet |
| 36 | `semio-hub-gis-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 37 | `semio-hub-playbook-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 38 | `semio-hub-procedural-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 39 | `semio-hub-process-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 40 | `semio-hub-puzzle-component:component:prototype` | component | 2 | 10 | not reached | not reached | cancelled, unknown | not yet |
| 41 | `semio-hub-sourcing-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 42 | `semio-s-plugin-cad-aec-building-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 43 | `semio-s-plugin-cad-aec-energy-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 44 | `semio-s-plugin-cad-aec-structure-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 45 | `semio-s-plugin-cad-spatial-shape-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 46 | `semio-s-plugin-flow-bim-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 47 | `semio-s-plugin-flow-brep-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 48 | `semio-s-plugin-flow-dictionary-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 49 | `semio-s-plugin-flow-draw-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 50 | `semio-s-plugin-flow-list-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 51 | `semio-s-plugin-flow-logic-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 52 | `semio-s-plugin-flow-math-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 53 | `semio-s-plugin-flow-primitive-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 54 | `semio-s-plugin-flow-text-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 55 | `semio-s-plugin-playbook-procedural-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 56 | `semio-s-plugin-process-concrete-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 57 | `semio-s-plugin-process-metal-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 58 | `semio-s-plugin-process-robotic-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 59 | `semio-s-plugin-process-wood-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 60 | `semio-s-plugin-sourcing-beams-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 61 | `semio-s-plugin-sourcing-slabs-component:component:prototype` | component | 2 | 7 | not reached | not reached | FAIL | not yet |
| 62 | `semio-s-plugin-sourcing-windows-component:component:prototype` | component | 2 | 7 | not reached | not reached | cancelled, unknown | not yet |
| 63 | `@semio-tech/framework-os-dev:prepare-aggregator-react-prototype:prototype` | lane prepare (react) | 3 | 52 | not reached | not reached | not reached | not yet |
| 64 | `@semio-tech/framework-os-dev:prepare-aussuchen-react-prototype:prototype` | lane prepare (react) | 3 | 52 | not reached | not reached | not reached | not yet |
| 65 | `@semio-tech/framework-os-dev:prepare-bearbeiten-react-prototype:prototype` | lane prepare (react) | 3 | 52 | not reached | not reached | not reached | not yet |
| 66 | `@semio-tech/framework-os-dev:prepare-energy-react-prototype:prototype` | lane prepare (react) | 3 | 16 | not reached | not reached | not reached | not yet |
| 67 | `@semio-tech/framework-os-dev:prepare-fem3d-react-prototype:prototype` | lane prepare (react) | 3 | 16 | not reached | not reached | cancelled, unknown | not yet |
| 68 | `@semio-tech/framework-os-dev:prepare-generation3d-react-prototype:prototype` | lane prepare (react) | 3 | 29 | not reached | not reached | cancelled, unknown | not yet |
| 69 | `@semio-tech/framework-os-dev:prepare-generator-react-prototype:prototype` | lane prepare (react) | 3 | 52 | not reached | not reached | not reached | not yet |
| 70 | `@semio-tech/framework-os-dev:prepare-koordinator-react-prototype:prototype` | lane prepare (react) | 3 | 52 | not reached | not reached | not reached | not yet |
| 71 | `@semio-tech/framework-os-dev:prepare-verfolgen-react-prototype:prototype` | lane prepare (react) | 3 | 52 | not reached | not reached | not reached | not yet |
| 72 | `@semio-tech/framework-os-dev:activate-energy-react-prototype:prototype` | lane activate (react) | 4 | 17 | not reached | not reached | not reached | not yet |
| 73 | `@semio-tech/framework-os-dev:activate-fem3d-react-prototype:prototype` | lane activate (react) | 4 | 17 | not reached | not reached | cancelled, unknown | not yet |
| 74 | `@semio-tech/framework-os-dev:activate-generator-react-prototype:prototype` | lane activate (react) | 4 | 53 | not reached | not reached | not reached | not yet |
| 75 | `@semio-tech/mit-bestand-demonstrator:prepare-prototype:prototype` | prepare (demonstrator) | 4 | 71 | not reached | not reached | not reached | not yet |

## Failures and class

| Run | Action | Class | Evidence |
|---|---|---|---|
| act15 | `@semio-tech/os-plugin-describe-rs:build:prototype` | unclassified (bun RepoBuild exit 1) | BUILD.bazel:1296233 ERROR at 8998 after a 2150 s heartbeat wait in source.served; the stderr is not in the retained lines |
| act15 | `@semio-tech/framework-os-node-graph-rs:wasm:prototype` | compile + source churn | E0277 `Mutation: ColdMutation` and `P: Clone` bounds in `semio-framework-os-store-replay` (5 errors); the same line 541 of `step_prepared_operation` appears in both the old and new text, so the source changed mid-compile (act15.log ~15107-15320) |
| act15 | `@semio-tech/puzzle-plugin:wasm:prototype` | compile + source churn | `error[E0433]: cannot find export in io`, `stdio-pdf-schema-5` lib fails (5 errors); `Completed Cargo Source differs from admitted artifact plan` (act15.log 18060) |
| act16 | `@semio-tech/os-plugin-describe-rs:build:prototype` | unclassified (bun RepoBuild exit 1) | ERROR at 12075; no compiler line near it |
| act16 | `semio-framework-os-infinite:fonts:prototype` | deadline/capacity | `Original resource loan deadline exhausted` (act16.log 15789) |
| act16 | `@semio-tech/puzzle-plugin:wasm:prototype` | capacity wait (cause not confirmed) | `cargo:build remains active on semio.compiler.host-capacity` for 400+ s (act16.log ~16900); ERROR at 16950 |
| act16 | `@semio-tech/framework-os-node-graph-rs:wasm:prototype` | source churn | `Completed Cargo Source differs from admitted artifact plan` (act16.log 22016) |
| act17 | `semio-s-plugin-sourcing-slabs-component:component:prototype` | compile-driver regression | `Distinct Cargo target flag views require distinct compiler invocation scopes` (act17.log 10583); the brief names this as the cause |
| act17 | `semio-hub-energy-component:component:prototype` | compile-driver regression | ERROR at 10759, same message family (act17.log 10581-10583) |
| act17 | run cancelled (SIGINT, exit 130 at 01:02) | operator cancel | not a failure; ran 81 min with progress-deadline "idle windows renewed" messages |
| Out of window | `repo_generator-inputs_prototype-dc14ef6d485f.tail.log` in `workspaces/s1` (Nov 1 03:52) | unclassified | `process:owner-command failed (1)`; cause not in the 5 KB tail; the run is not act15-17 |
| Out of window | `_semio-tech_trinity-jack-shell_build_prototype` in `workspaces/d1` (Oct 23) | compile | `error[E0053]` incompatible trait for `step` and `close_step` in `semio-s-artifact-trinity-jack-schema` (Oct 23, out of the Nov 1-2 scope) |
| Missing | `workspaces/s2/0000000000000000/failures/` (slabs tail Nov 2 00:52, hub-energy tail Nov 2 00:57) | n/a | The folder no longer exists (s2 directory was rewritten by act18 at 13:07). The tails could not be read. The act17.log lines above cover them. |

## Never passed (act18 discovery risks)

47 of the 75 actions never passed in act15 or act16 (each either failed or was gated by a failure). act17 was cancelled, so it gives no confirmed pass. Each of these is a discovery risk for act18, because no run has proven it. The list is dominated by the component actions (all `component` kind) and the 12 framework-os-dev prepare/activate lanes, which are only reached after the failing puzzle-plugin and describe-rs actions have run.

- `@semio-tech/framework-os-dev:activate-energy-react-prototype:prototype` (lane activate (react), gates 17)
- `@semio-tech/framework-os-dev:activate-fem3d-react-prototype:prototype` (lane activate (react), gates 17)
- `@semio-tech/framework-os-dev:activate-generator-react-prototype:prototype` (lane activate (react), gates 53)
- `@semio-tech/framework-os-dev:prepare-aggregator-react-prototype:prototype` (lane prepare (react), gates 52)
- `@semio-tech/framework-os-dev:prepare-aussuchen-react-prototype:prototype` (lane prepare (react), gates 52)
- `@semio-tech/framework-os-dev:prepare-bearbeiten-react-prototype:prototype` (lane prepare (react), gates 52)
- `@semio-tech/framework-os-dev:prepare-energy-react-prototype:prototype` (lane prepare (react), gates 16)
- `@semio-tech/framework-os-dev:prepare-fem3d-react-prototype:prototype` (lane prepare (react), gates 16)
- `@semio-tech/framework-os-dev:prepare-generation3d-react-prototype:prototype` (lane prepare (react), gates 29)
- `@semio-tech/framework-os-dev:prepare-generator-react-prototype:prototype` (lane prepare (react), gates 52)
- `@semio-tech/framework-os-dev:prepare-koordinator-react-prototype:prototype` (lane prepare (react), gates 52)
- `@semio-tech/framework-os-dev:prepare-verfolgen-react-prototype:prototype` (lane prepare (react), gates 52)
- `@semio-tech/mit-bestand-demonstrator:prepare-prototype:prototype` (prepare (demonstrator), gates 71)
- `@semio-tech/os-plugin-describe-rs:build:prototype` (describe, gates 2)
- `@semio-tech/puzzle-plugin:wasm:prototype` (wasm, gates 9)
- `semio-hub-cad-component:component:prototype` (component, gates 7)
- `semio-hub-demonstrator-component:component:prototype` (component, gates 8)
- `semio-hub-energy-component:component:prototype` (component, gates 7)
- `semio-hub-fem-component:component:prototype` (component, gates 7)
- `semio-hub-flow-component:component:prototype` (component, gates 8)
- `semio-hub-gis-component:component:prototype` (component, gates 7)
- `semio-hub-playbook-component:component:prototype` (component, gates 7)
- `semio-hub-procedural-component:component:prototype` (component, gates 7)
- `semio-hub-process-component:component:prototype` (component, gates 7)
- `semio-hub-puzzle-component:component:prototype` (component, gates 10)
- `semio-hub-sourcing-component:component:prototype` (component, gates 7)
- `semio-s-plugin-cad-aec-building-component:component:prototype` (component, gates 7)
- `semio-s-plugin-cad-aec-energy-component:component:prototype` (component, gates 7)
- `semio-s-plugin-cad-aec-structure-component:component:prototype` (component, gates 7)
- `semio-s-plugin-cad-spatial-shape-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-bim-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-brep-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-dictionary-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-draw-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-list-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-logic-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-math-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-primitive-component:component:prototype` (component, gates 7)
- `semio-s-plugin-flow-text-component:component:prototype` (component, gates 7)
- `semio-s-plugin-playbook-procedural-component:component:prototype` (component, gates 7)
- `semio-s-plugin-process-concrete-component:component:prototype` (component, gates 7)
- `semio-s-plugin-process-metal-component:component:prototype` (component, gates 7)
- `semio-s-plugin-process-robotic-component:component:prototype` (component, gates 7)
- `semio-s-plugin-process-wood-component:component:prototype` (component, gates 7)
- `semio-s-plugin-sourcing-beams-component:component:prototype` (component, gates 7)
- `semio-s-plugin-sourcing-slabs-component:component:prototype` (component, gates 7)
- `semio-s-plugin-sourcing-windows-component:component:prototype` (component, gates 7)

Note: `os-plugin-describe-rs` passed in act17 (heartbeats to 763 s with dependents progressing), but act17 was cancelled before Bazel confirmed it. Keep it on the risk list.

## Dependency order and critical path

Longest chain to the top request (from the BUILD graph):

1. `repo:generator-inputs:prototype` (RepoGenerate, level 0)
2. `@semio-tech/s-dev-composition:generate:prototype` (RepoGenerate)
3. `@semio-tech/plugin-registry:session-fem3d:prototype` (RepoScript)
4. `@semio-tech/framework-os-dev:prepare-fem3d-react-prototype:prototype` (RepoBuild)
5. `@semio-tech/mit-bestand-demonstrator:prepare-prototype:prototype` (RepoBuild)
6. `@semio-tech/mit-bestand-demonstrator:activate-prototype:prototype` (top request)

Gating hubs (a failure here skips the most work, by transitive dependents): `ui-rs:generate` (57 dependents), `ui-styling-tokens:generate` (51), `dsl-derive-rs:generate-schema-index` (49), `workspace:deps-wasm-opt` (48), `framework-schema:generate` (47), `os-plugin-describe-rs:build` (46), `framework-plugin-web:support` (46). Most of the 75 actions depend on these roots.

Measured critical path: act15 2611.5 s, act16 4113.6 s (Bazel "Critical Path"). act15 wall clock 5964 s, act16 14539 s. The gap is queueing for capacity and deadlines, not compile time.

## act18 compared with act15-17 (as of 13:14:25)

- act18 reached 27/77 at 13:14 (about 11 min after start). act15 and act16 last progress markers were 62/77 before they finished at 77/77. act17 reached 22/77 and was cancelled after 81 min. The log has no per-line timestamps, so a time-aligned comparison with act15-17 is not possible from these logs.
- act18 is at the same two gating actions that caused the act15/16 delays: `os-plugin-describe-rs:build` (queued, Sched 70 s) and `puzzle-plugin:wasm` (running, 339 s). Both failed in act15 and act16, so watch them first.
- act18 has no ERROR lines yet (checked at 13:14).
- Freeze check: no act18 failures so far, so the freeze policy is not yet tested.

## Expected act18 duration if nothing fails

- Floor (critical path only, warm caches): about 2600 s, or 44 min (act15 measured).
- Expected with the act15 pattern (queue and capacity waits included): about 100 min (act15 wall 5964 s).
- Upper bound if capacity or deadline waits recur as in act16: about 4 h (act16 wall 14539 s).

## Data gaps

- Per-action durations are heartbeat values only (max seen elapsed). Bazel prints no per-action finish time, so exact durations are not available.
- The 77-action total cannot be matched exactly. The graph has 76 nodes (75 actions plus the top request), so one action is missing from the logs and the BUILD graph.
- The s2 failure tails were removed when act18 rewrote the workspace, so the per-tail stderr for slabs and hub-energy could not be checked.
- describe-rs failures (act15, act16) have no compiler line in the retained log, so their cause stays unclassified.
