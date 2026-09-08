# 🧩️ Puzzle 3d end to end — master plan (session 2026-09-08, reopened)

Goal as given: every puzzle 3d window works, every tool (fill, brush, gumball, select, suggestions,
examples, …) works end to end from a user perspective; when the app hits performance limits, optimize
the architecture. Coordination in this session (Fable 5.1), execution by Opus 5 agents, read-only
audits by Sonnet 5 agents, maximum parallel fleet. Other sessions edit the same files; keep going.

## Interpretation of "all windows"

Edit mode opens two instances of one window kind (`puzzle3d-main-top`, `puzzle3d-main-perspective`)
plus the settings / inspection / artifact / catalogue panels. The active example is document-global.
"All windows work" = both views render the document, respond to their per-window options (grid, LOD,
projection, select, sun, vortex), and every panel shows live content and dispatches. Both examples
(concrete-forest, nakagin-capsule-tower) must load and switch. Two views of two *different* examples
at once is a design change and is out of scope unless the dev asks (flagged 2026-09-05, unanswered).

## Constraints measured at session start (23:35)

| signal | value |
|---|---|
| load average | 60 (1 min), 65, 77 |
| swap | 40.9 GB of 42 GB used, 1 GB free; RAM 32 GB |
| repo `.rs` files modified in last 30 min | 703 (stdio 268, flow/os-modules 90) |
| puzzle3d editor last modified | 23:04 (peer: CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS, moved locale/terminology to OS `ViewModel`, `render`/`window_engagements` now take `view_state`) |
| puzzle 3d crate | since today its own crate `semio-s-artifact-puzzle-3d` (`🗿️artifacts/🧊️3d/📦️packages/🦀️rust`), feature `component-app-assembly` |
| staged puzzle component wasm | 10:56 today (39.7 MB), browser wasm 12:11 — both predate the crate split and the ViewModel change |
| dev ports 6013 / 6113 | free, no dev server running |
| repo MCP | CONNECT_TIMEOUT — ticket bookkeeping on disk |

Consequences: every Rust build must go to a private, lane-qualified `CARGO_TARGET_DIR` outside the repo,
with `RUSTC_WRAPPER=""`, `CARGO_PROFILE_WASM_DEV_DEBUG=false`, run under `nohup` with a scratchpad log,
never from a Monitor; build-bound lanes run alone. Peers hot in stdio/flow: poll, do not fix while
their files are < 15 min old.

## Phases

0. Audit fleet (Sonnet, read-only, running): build chain, dev boot path, feature inventory, frontend +
   test harness, performance architecture, peers + gates. Reports land as `📓️2026-09-08-*.md` here.
1. Compile truth: one private `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`
   (smallest unit that covers the editor), then the plugin crate, then the wasm targets the dev boot needs.
2. Fix waves (Opus, foreground builds, disjoint write-locks per file):
   - W-A compile breaks in puzzle3d after the peer's ViewModel/locale move (labels from `view_state`).
   - W-B dead/stubbed features from the inventory (transformBegin/End gumball session, setFixtureJson
     chunked wire, extent bounds on Nakagin, any `_ => {}` reducer arms).
   - W-C frontend hooks needed for user-level verification + `[DEBUG]` cleanup.
3. Boot + browser verification (this session): react renderer on 6013, every step of
   `📓️runtime-verification-plan.md`, both examples, fill/brush/gumball/select/suggestions, both windows,
   every panel. Findings go to `📓️2026-09-08-runtime-verification.md`.
4. Performance waves (Opus) driven by what step 3 measures against the audit's ranked bottlenecks.
5. Gates + close: interactivity audit, publication-authority audit, dependency truth gate, puzzle
   tests, launch seed regen; remove `[DEBUG]`, delete `🗑️generated`, clear `📌️important`, close ticket.

## Findings digest (after the audit fleet, 2026-09-09 00:00)

Reports: `📓️2026-09-08-build-chain-audit.md`, `📓️2026-09-08-feature-inventory.md`,
`📓️2026-09-08-frontend-and-test-harness.md`, `📓️2026-09-08-performance-architecture-audit.md`,
`📓️2026-09-08-peers-and-gates.md` (+ `📓️2026-09-08-dev-boot-path.md`, `📓️2026-09-08-selection-hover-mechanism.md` when they land).

- Build chain: every 09-05 blocker is fixed in source; `semio-framework-plugin` and os-kernel are hot
  (peer ownership-levels fleet). A seeded private `cargo check -p semio-s-artifact-puzzle-3d
  --features component-app-assembly` is running (scratchpad `target-p3d`, log `check-p3d-1.txt`).
- Actions: 60/63 Migrated and real. Held back: `transformBegin`/`transformEnd` (scratch session can
  never survive because `with_puzzle3d_app_for` rebuilds the app per call; the host sends one absolute
  delta per drag) and `setFixtureJson` (no caller, 129 KB vs 8 KB wire cap). → wave W-T.
- Correction to the inventory's item 7: the tightened `extent()` bounds ARE in current source
  (`worldRelocate` = 3N+3+A+2V etc., verified at editor `🦀️.rs:4202`); Nakagin passes them.
- Dominant defect: no selection/hover reaches `render`/`context_menu` (`InteractionView` missing),
  so gumball never shows, selection overlay empty, inspection panel static, select-same-kind no-op,
  vortex "Selected" show mode dead, brush placement picker dead, presence lacks selection. → after
  the selection-mechanism report.
- Performance: (0) fill planner hard-capped at 32 objects by `FIXED_OWNER_SLOTS` → Fill faults on
  Nakagin → wave W-F; (1) `with_puzzle3d_app_for` rebuilds the app per call and defeats the
  geometry/document-tree/mesh caches → wave W-P; (2) interactive brush suggestion cache is O(N²×C)
  unindexed; (3) no user-facing fill cancellation although `cancel_fill_job` exists → W-P;
  (4) `registerBrushMesh` cap promises 196,608 elements over an 8 KB wire.
- Frontend: React host draws selection/gumball from the app's `selectionJson`/`gumballActive`; DOM
  hooks for panes, example select, fill slider, panels and context menu are documented. `[DEBUG]`
  gumball logs remain in `World3dHost/🟦️.tsx:4718-4740` → final cleanup wave.
- Peers: ownership-levels fleet live in `🔌️plugin`, `📡️spr`, `📺️renderer`; schema-contracts fleet
  relocating mutation leaves incl. puzzle. Gate order for close is in the peers report §4.

## Wave log

| wave | model | scope | status |
|---|---|---|---|
| W-T | Opus | transformBegin/End honest HostOnly brackets, dead scratch session deleted, setFixtureJson removed; 62/62 Migrated | done 00:28; native check: editor clean (1 unused-var warning handed to W-S) — `📓️2026-09-08-wave-T-transform-and-fixture-json.md` |
| W-F | Opus | fill planner 32-slot capacity → document-scale capacities | running (check shows its mid-edit E0308 at `🪣️fill/🦀️.rs:2614`) |
| W-R | Opus | react dev boot pipeline (materialize/support/prepare/activate/serve) for playground variants + `.claude/launch.json` entries | running |
| W-S | Opus | `render_with_request_context` selection/hover → gumball, selection JSON, brush picker, inspection panel, vortex Selected mode; `scene.domain_id` bound; `Emit.interaction_writes` (framework, additive) for select-same-kind/duplicate/accept re-select | running |
| W-G | Opus | puzzle-js publication-authority audit oracle (locale anchors), retained-jobs fixtures 3d/5d (2 vs 62 ids), `verify interactivity` + repo gates | running |
| audits | Sonnet | session registry + brush index + mesh wire + fill cancel design; wgpu renderer coverage for puzzle3d UI | running |

Builds: `target-p3d` native check of the 3d crate passes in ~4 min (00:03, 0 errors). A wasip2
dependency pre-build of `semio-s-plugin-puzzle` (`--profile wasm-dev`) runs in `target-p3d-wasm`
(log `prebuild-wasm-1.txt`) so the boot build after the Rust waves only compiles puzzle crates.
WIT drift confirmed: `surface-visible-event` gained `body-key` + `view-state` at 22:10 → every staged
component (puzzle 10:56, stdio Aug 18) must be rebuilt before any boot.

## Decisions 00:30

- Verification renderer: React (port 6013). The wgpu retained renderer has real world-3d + native chrome
  but no 3D transform gumball anywhere in Rust and no puzzle3d-specific wiring
  (`📓️2026-09-09-wgpu-coverage-audit.md`). wgpu stays a follow-up once React passes.
- Peer fallout fixed by the coordinator (cold files, needed for the plugin wasm): puzzle 2d editor
  `handle` had `_view_state` bound twice (peer editor-cleanup), and `◻️2d` Cargo.toml lacked
  `semio-framework-graph` after the crate split. wasip2 build of `semio-s-plugin-puzzle` relaunched
  (`prebuild-wasm-2.txt`); the first run compiled the full chain in 11 min with only those 2d errors.
- W-P (after W-S + W-F land, same files): per `📓️2026-09-08-session-index-mesh-cancel-design.md` —
  process-global slot registry keyed by `app_instance_id` (already threaded via
  `ArtifactOwnedToolJobRequest`/`ArtifactView::operation()`, unread) holding geometry/document-tree
  caches, registered meshes and a persistent `CollisionSpatialIndex` for the brush lane; incremental
  index sync from the mutation stream; host-side content-addressed `puzzle3d.mesh-decode` Engine instead
  of pushing GLB bytes over the 8 KB wire; `cancelFill` action → `Effect::CancelJob` (energy plugin
  pattern) + cancel affordance in the fill tool panel. ~55 literal substrings are string-matched by
  `interactivityPuzzleFillEnvelopeSelfTests` — waves must re-run `verify interactivity`.
