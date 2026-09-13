# 🪣️ Interactive tools with visible process — status

Session start 2026-09-13 (Fable 5.1 main chat). Repo MCP failed to connect (`invalid initialize params`), so the
ticket folder is managed on disk; goal association `🎯r2603` mirrors the other September tickets.

## Goal (dev's words, condensed)

- Every tool is interactive; the user sees how the algorithm thinks (steps, percentage, status, intermediate results).
- Puzzle 3d fill: no hidden precompute-then-reveal. Count slider unbounded, arbitrarily settable, default 100. The UI
  builds the solution live: every tried object is shown (danger when colliding, highlight when collision free), locked
  objects appear as they are locked.

## Phase 1 — audits (Sonnet fleet, read-only)

| Report | Scope |
|---|---|
| `📓️audit-fill-pipeline.md` | FillBuilder stages, publications, job bridge, count apply, FILL_COUNT_MAX sites, RNG prefix property |
| `📓️audit-viewport-render-path.md` | Rust render → instances → r3f; per-instance color; brush ghost pattern; reveal cutoff |
| `📓️audit-progress-primitives.md` | WindowMeasure variants, jobs/ticks, other plugins' progress UX, schema-first gap |
| `📓️audit-tests-and-deploy.md` | fill tests, oracle convention, cargo commands, deploy chain to :6013, browser probe |
| `📓️audit-tool-inventory.md` | repo-wide tool classification A–E, top-10 conversion list |

Baseline `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` → `🗑️generated/baseline-check.txt`.

## Phase 2 — plan (this chat) → `📋️master-plan.md`

## Phase 3 — implementation waves (Opus fleet)

## Phase 2 — plan written: `📋️master-plan.md` (decisions §1, binding contract §2, waves §3, gates §4)

Audit takeaways: the planner already steps per candidate and publishes a ghost + counters, but (1) it always plans to a hidden
1000, (2) accepted pieces only become document objects at slider commit via a separate synchronous replan, (3) the ghost
never carries a collision verdict, (4) the framework has no unbounded numeric or progress measure.

## Phase 3 — implementation waves launched 2026-09-13 (Opus 5, parallel)

| Wave | Scope | Report |
|---|---|---|
| A | planner: requested-count target, verdict ring, stall reasons, preview wire + fixture/oracle | `📓️wave-A-planner.md` |
| B1 | session/job bridge: requested count to the worker, locked-chunk API, summary counters | `📓️wave-B1-session.md` |
| B2 | commands/config: setFillCount rewrite, tick commits locked pieces, default 100, ghost tail removed | `📓️wave-B2-commands.md` |
| C | viewport/UI: danger/highlight ghosts, tried ring, HUD, Number/Progress measures, reveal removal | `📓️wave-C-viewport.md` |
| D | puzzle 2d/5d parity: unbounded default 100, 5d progress/cancel | `📓️wave-D-2d-5d.md` |
| F | framework `WindowMeasure::{Number,Progress}` end to end | `📓️wave-F-framework-measures.md` |
| G | brush suggestions stream tested/free/blocked with ghost verdicts | `📓️wave-G-brush.md` |
| E | policy predicates in root `📜️script.ts` (after A/B/C) | `📓️wave-E-policy.md` |
| H | probe step, deploy, runtime evidence (after all) | `📓️wave-H-verification.md` |

## Phase 4 — reopened 2026-09-13 ~22:00 (Opus 5 session ⚪7c7fb230, repo MCP still down: `invalid initialize params`)

New dev requirements on top of phases 1–3:

1. Every tool has a lifecycle: **start → running → complete**, **abort** at any time, **finalize** only when complete.
2. A tool that mutates the artifact runs **inside a transaction**: provisional mutations are visible while running,
   abort rolls them back, finalize commits them as one unit.
3. Every tool shows progress (steps, percentage, status) — phase 3 already added `WindowMeasure::Progress`.
4. Puzzle 3d fill shows **all** tested meshes (not a ring of 12), colored: collision = danger, fitting = success.

State found: waves A, B1, B2, C, D, G reported; wave F code landed (`WindowMeasure::{Number,Progress}` in wgpu component,
projection, manifest, `🎚️measure-controls`) without a report; waves E and H never ran. B2's "tick commits locked pieces
straight into the document" contradicts requirement 2 and is re-planned in phase 4.

Phase 4a audits (Sonnet, read-only, parallel):

| Report | Scope |
|---|---|
| `📓️audit-p4-transaction-primitives.md` | artifact mutation/history/event-sourcing primitives usable for a tool transaction |
| `📓️audit-p4-tool-lifecycle.md` | tool/job/cancel/adopt primitives, wave F landed state, gap to start/abort/finalize |
| `📓️audit-p4-fill-state.md` | puzzle 3d fill after phase 3: compile/test state, tried-set rendering, commit path |
| `📓️audit-p4-tool-inventory.md` | every algorithmic/mutating tool repo-wide against the lifecycle + transaction contract |

Phase 4a landed (22:15–22:40): `📓️audit-p4-transaction-primitives.md` (no primitive aborts an open coalesced edit; `Emit::amend`/`Emit::commit`/`UtilityPreviewContract` closest; `StateClass` names all four state classes), `📓️audit-p4-tool-lifecycle.md` (`ToolDefinition` static, no run state; wave F React-complete, wgpu measure widgets have no caller; proposal `ToolRunState` + start/abort/finalize), `📓️audit-p4-tool-inventory.md` (34 plugins; puzzle 2d/3d fill commit per tick with no rollback; wave plan 0–3). Fill-state audit still running.

Phase 4b: Opus architect decides the transaction primitive and writes `📋️tool-run-contract.md` (lifecycle, transaction, visible-process rendering, disjoint implementation lanes).
- 23:0x fill-state audit landed → `📓️audit-p4-fill-state.md`: puzzle-3d crate 0 errors/96 warnings; fill tests 95/100 (4 real reds: `fill_job_identity()` None while pieces already locked → cancel vanishes; `fillBuildTick` per-tick document diff 5.09 ms > 2 ms budget); locking = committing every tick; tried ring 12 / 16 KiB full-snapshot wire cannot carry all tested candidates; `success` token exists but unused for fill. Architect told about it.
- 23:3x architect landed → `📋️tool-run-contract.md` (binding): framework-owned `ToolRunLedger` holding provisional `A::Mutation`s as an overlay the renderer reads; abort never touches the store; finalize (only from `complete`) rebases/revalidates and publishes ONE `Edit` (group `toolRun:<runId>`); delta trace pages into a `toolRunTrace` scene lane; generic reserved actions start/pause/resume/step/abort/finalize/dismiss; `Component::Progress`. Rejected `Emit::amend` (per-tick durable + full-snapshot broadcast).

## Phase 4c — wave 0 implementation (Opus 5)

Dependency-ordered: W0-A/C/F/G first (independent); W0-B, W0-D, W0-E and W1-A after W0-A (W0-D also after W0-C).
`.vscode/launch.json` registration is done by the coordinator at the integration gate from the lanes' reported commands.
- 23:4x dispatched wave 0a (Opus, rules `📋️lane-rules.md`): W0-A `⏯️tool-run` module → `📓️wave-W0-A.md`; W0-C `Component::Progress` + `WindowMeasure::Progress` removal → `📓️wave-W0-C.md`; W0-F presence tool-run summary → `📓️wave-W0-F.md`; W0-G wgpu measures home → `📓️wave-W0-G.md`. Queued on W0-A: W0-B, W0-E, W1-A; on W0-A + W0-C: W0-D.
- W0-F landed → `📓️wave-W0-F.md`: `PresencePeer.tool_run` (flag bit 10) Rust/TS/schema/fixtures, replication 273 pass, wasm32-wasip2 clean, hub vectors byte-exact via ticket probe; hub Rust test blocked by peer compile error `🌎️hub/🏗️bootstrap/🦀️.rs:31`. Open: W0-D fills `tool_run`; coordinator reruns hub test; local `PresenceToolRunState` duplicates `ToolRunState` names (layering).
- W0-C landed → `📓️wave-W0-C.md`: `Component::Progress` contract + wgpu reconcile/paint/a11y + React Interpreter (96/96), `WindowMeasure::Progress`/`MeasureProgressStep*` removed; puzzle 2d/3d/5d fill + 3d brush no longer compile (expected per §3.6). Pre-existing reds noted: 24 contract + 2 ui-runtime retirement close-loop asserts (container/text rows), 3 wgpu engine tests. Open: W0-B legacy `UiNode` TS `progress` arm; wgpu indeterminate sweep static.
- dispatched W0-C2 (Opus) → `📓️wave-W0-C2.md`: delete plugin progress measures so puzzle crates compile again for concurrent devs.
- W0-C2 landed → `📓️wave-W0-C2.md`: puzzle 3d/2d/5d compile again (96/4/1 warnings); cancel kept as Toggles (5d got a new `fill_cancel` label); 2d fill 5/5, 5d cancel 1/1, the two 3d cancel reds now pass. Pre-existing reds: two 5d "tool factory key already registered", one 3d brush precompute test. Open: React `🎚️window-measure-controls` test Progress fixture (W0-B); rerun `cargo check -p semio-framework-os-renderer-wgpu --tests`.
- W0-A landed → `📓️wave-W0-A.md`: crate `semio-framework-tool-run` 21/21, TS 18/18 (ajv/xstate/fast-check), wasm32-wasip2 + clippy clean, TS↔Rust byte parity. Deviations: reducer over `Option<ToolRunSlot>`, `AbortComplete` event, `toolRun.illegal`, delta `clear`/`next`, reserved reason codes 0xFF00–0xFF03. Depends on os-kernel + ui(wgpu) → ui/ui-scene/ui-contract/os-kernel/job must never depend on it (W0-E carries trace as opaque base64url). Owed launch.json: `@semio-tech/framework-tool-run-rs:test|check`.
- dispatched wave 0b: W0-B manifest+projection, W0-D runtime ledger+driver, W0-E trace lane+renderers, W1-A puzzle 3d fill run job.
- W0-B landed → `📓️wave-W0-B.md`: `run: Option<ToolRunDefinition>` on Tool/UtilityDefinition (crate `semio-framework`), `tool_run_action_definitions` injected only when `run` declared (builder line in P), chord law + fixture (4 Rust + 4 bun ajv), projection `ToolRun*` + `UiProgressNode`, generate→check green, React Progress measure removed (7/7, 9/9). Pause/resume share `mod+alt+enter` bound to pause (router toggles; W0-D told); dismiss has no app-wide key. Pre-existing reds: `semio-framework --lib` 12 (10 ui retirement asserts), os-mcp compile churn, verify interactivity (launch.json capacity/gate registrations, 757+ discovery, puzzle fill envelope). Owed launch.json: `@semio-tech/framework-rs:test-tool-run-actions`, `:check`, renderer react `test-long 🎚️window-measure-controls`.
- W0-E landed → `📓️wave-W0-E.md`: `toolRunTrace` lane on World3dScene (20 lanes) + Canvas2dScene (opaque base64url; `SceneLaneRef`/`scene_lane_hash` renames, 23 Canvas2dScene sites), base64url codec in `🚪️io/🔤️base64`, wgpu `♾️infinite/🌍️world/⏯️tool-run-trace` instanced layer, React 3d + 2d trace layers with verdict tokens/age fade/`data-tool-run-*`, `provisional` mesh style prop. scene 123, base64 6, wgpu trace 5, reconcile/wire 39, React 7 + Interpreter 97. Pre-existing: 4 wgpu image tests (global queue), frame-worker bundle regen owed. Legend/cursor echo/provisional set exposed, not wired.
- dispatched W1-C React host cleanup → `📓️wave-W1-C.md`.
- W0-D landed → `📓️wave-W0-D.md`: `ToolRunLedger<A>` + driver in `🔌️plugin/⏯️tool-run`, 7 actions framework-routed, render/engagements/measures read overlay, panel body, finalize = one grouped Edit + one command row + one Mutations batch; 10/10 tool_run tests incl. abort zero-trace, one-undo, rebase conflict, stale no-op, pause/step, resume/retract, 771-tick bench. Store edits: `begin_outbound_apply_batch` + `flush_published_apply_batch`, per-op mutation ids in batches. Driver steps the job inside `advance_typed_operation_publication` (not ActionBus). `ArtifactApp::build_tool_run_job` hook. Open: tick writer start offset (W0-A), scene lane + presence injection from `tool_run_trace_delta()`/`tool_run_presence()` not wired, per-tick whole-UI dirty. Pre-existing reds: store `artifact_store_batch_commit_refuses_…`, plugin transaction tests (`testkit-txn` app id).
- dispatched W0-H integration (scene lane + presence injection, tick writer offset, tick dirty scope) → `📓️wave-W0-H.md`.
- W1-C landed → `📓️wave-W1-C.md`: World3dHost fill code gone (tried ghosts, fillBuildPreview, caps, diagnostic overlay, tick loop), trace layer mounted per World3d window with `data-tool-run-*`; fixture `⏯️tool-run-trace-mount.json` + three.js oracle 1/1, World3d 42/42, engine-contract 605/605. Provisional style awaits a producer (`provisional: true` on instance records or `ToolRunProvisionalIdsContext`) — told W0-H. Leftover `fillBuildTick` refs in ShellHelpers/ShellHost/scene TS → W1-B. Owed launch.json: `bun ./📜️script.ts test long "🌐️World3dHost/🧪️tests"`.
- W1-D landed → `📓️wave-W1-D.md`: `⏯️ToolRunPolicy` (5 predicates, 28-row required-tool table) + `🪣️PuzzleFillToolRun` in root `📜️script.ts`, new `🔬️interactivity-puzzle-fill-run-job`/`-trace`, 110 self-test cases, 12/12 planted breaks caught. Expected reds: 28 missing definitions, 190 legacy-trace hits (puzzle fill), 4 amend hits (puzzle `fill-count` ×2, remodel per-tick reconstruction commits ×2 — remodel is not class A), 19+11 fill, 1 P4e. `verify interactivity` still aborts on pre-existing foreign self-tests `interactivityLiveReconcileSelfTests`/`interactivityMountedLayoutTextSelfTests`.
- W1-A landed → `📓️wave-W1-A.md`: `FillRunJob` (testing → one verdict per fuel unit, appendOps + entity per placement, CheckpointReady, stall warning → Complete, resume raise/lower) + `FillRevalidateJob`; 8 new tests pass at threads 4 and 1; `-- fill` 106/2 (the two §0.9 reds in W1-B's file); worst drive_step 1.106 ms / 2 270 turns Nakagin; parry3d 204 decisive / 0 disagreements; ≥5 000-candidate delivery complete; native + wasm32-wasip2 0 errors. Deviations: in-memory retarget on resume, typed placements to revalidate, caller-provided mesh lane order (W1-B must publish the same order). Tried ring + 16 KiB cap still referenced by main window / precompute / root script.
- dispatched W1-B puzzle 3d fill editor wiring → `📓️wave-W1-B.md`.
- dispatched W2-A puzzle 2d fill run → `📓️wave-W2-A.md`; W3-1 energy simulation run → `📓️wave-W3-1.md`. Held: W2-B (5d) and W2-C (brush) until W1-B lands (shared 3d session/precompute); remaining wave 3 lanes after W0-H lands (fleet size vs build starvation).
- W0-G landed → `📓️wave-W0-G.md`: measures ride the existing `framework.section.measures` retained surface (new `UiDocumentLease::read_paged_text`/`try_publish`), bridge reads them on both backends, Shell projects Number/Slider/Select/Toggle/Group into a right-aligned Measures overlay (keyboard `setFillCount` + pointer toggle tests), dead stub/field/129-line widget block deleted; contract 3/3, bridge+Shell 5, catalogue 5. Foreign: `mounted_layout` `Control` layout kind (controls had zero height). Unverified: wasm32-unknown-unknown blocked by 7 `InteractiveJob` Send errors in plugin crate from W0-D → handed to W0-H. Red to recheck: `dock_stack_content_fills_full_bounds_through_one_silhouette_clip` + ~10 shell/interpreter + 7 ui tests (no baseline).
