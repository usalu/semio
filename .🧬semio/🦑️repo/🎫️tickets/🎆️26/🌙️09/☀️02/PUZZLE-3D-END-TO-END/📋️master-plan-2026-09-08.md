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

## Wave log 01:45

| wave | status |
|---|---|
| W-F | done — document-scale fill capacities, Nakagin regression test; audited literals restored (`📓️2026-09-08-wave-F-fill-capacity.md`) |
| W-R | done — react boot chain already inferred by the Nx plugin; proved materialize→prepare→activate→serve on 6013 with a Sep-7 component; stdio NOT needed; `.claude/launch.json` entries now call the Nx targets directly because `bun ./📜️script.ts dev 3d` dies at Nx plugin loading when spawned via `runCmd` (`📓️2026-09-09-wave-R-react-boot-pipeline.md` §8.1) |
| W-S | done — `render_with_request_context`, `scene.domain_id` bound, `Emit.interaction_writes` + `Interaction` lane, `window_measures_with_request_context`, `context_menu_with_request_context`; lib+tests check 0 errors at 01:35 (`📓️2026-09-09-wave-S-selection-render-and-writes.md`) |
| W-G | done — publication-authority audit green for 3 owners (3d: 62), retained-jobs fixtures 3d 62 / 5d 9, `verify interactivity tool-jobs` unblocked; residual puzzle item → W-P (`📓️2026-09-09-wave-G-gates.md`) |
| W-X | running — test binary aborts (registry-less `testkit::new_app`, `resolve_ready` first poll) |
| W-V | running — host vortex-marker picks via generic interaction path, `[DEBUG]` gumball logs |
| W-P | running — session registry keyed by `app_instance_id`, indexed brush collision, host-side mesh-decode engine, `cancelFill` |

Coordinator fixes: puzzle 2d `handle` duplicate `_view_state` + missing `semio-framework-graph` dep;
puzzle 5d schema TS unquoted `"2d"`/`"3d"` keys (broke Vite's scan). Engine wasm + support/session/fonts
targets green (01:29). Component build (`component-dev`, private `target-p3d-wasm`) launched 01:39 but
the framework plugin crate is mid-edit by the ownership peer (new untracked `🪟️window/🫧️transient`
module, `mod window_transient;` without `#[path]`, private `Fault` imports) — poller
`poll-plugin-heal.sh` retries a plugin check every 3 min once the file is 7 min cold.

## Wave log 03:15

| wave | status |
|---|---|
| W-V | done — marker picks/hovers (vortex/targetVolume/reference) via generic `interactionSelect`/`interactionHover` when `scene.domainId` is bound; six `[DEBUG]` traces removed; engine-contract 445/445, renderer tsc clean (`📓️2026-09-09-wave-V-host-vortex-select.md`) |
| W-P | done — per-instance session registry (ABA/try_lock/census), incremental spatial index for the brush lane, `puzzle3d.mesh-decode` Engine in a process-global `EngineCache` (host-side WIT `engine-derive/read` still aspirational → geometry still uploaded once per process, 512-element wire truth), `cancelFillBuild` + progress readout (63 routes); check --tests 0/0; 3 stack-overflow causes fixed (`📓️2026-09-09-wave-P-performance.md`) |
| W-X | running — test binary aborts + triage of failing tests (30 pre-existing precompute/fill failures reported by W-P) |
| W-G2 | running — fail-closed locale resolver in 3d+5d terminology (`puzzle3d-locale-default` clause), `BuiltNode` Clone → `document_tree_cache` memo |

Framework plugin crate healed at 02:16; 3d crate check lib+tests 0 errors / 0 warnings at 03:10;
component build relaunched 03:09 (`component-dev-2.txt`).

## Wave log 03:40

| wave | status |
|---|---|
| W-X | done — test binary runs (registry-backed instance-bound fixture, `settle` host turn, 2 MiB stacks, asserting `Drop` removed); isolated suite 577 tests / 405 pass / 172 fail / 0 abort; 10 production defects P1-P10 named with file:line (`📓️2026-09-09-wave-X-test-suite.md` §5) |
| W-D | running — fixes P1-P10: missing store disposers (cannot close), `host_configuration_mutation` for setActiveTool/Utility (fill tool unselectable), Config lane admitting only `Snapshot` (every scalar-config route publishes nothing), Nakagin operation capacity, inspection UiMap capacity, precompute owner drop, worker-pump transition, canonical-JSON fixture mismatch, `nx test` default features |
| W-G2 | running — fail-closed locale resolver 3d+5d, `BuiltNode::credited_clone` in `🖱️ui/🧬️contract/🏗️builder.rs` (mid-edit broke the engine wasm builds at 03:29) |

Component built 03:17 (`dist/component-dev/semio_s_plugin_puzzle.wasm`, 88 MB wasm-dev) after removing
W-R's unowned staging dir. `dev-puzzle3d-react-dev` needs the Nx daemon watcher (not running under
load) → use `serve-puzzle3d-react-dev`; the serve chain rerun waits on W-G2's builder edit. The Browser
pane refused `preview_start {url}` for 6013; use the attach-only launch entry `puzzle3d-react-attach`.

## Runtime findings 05:00 (React, port 6013, component of 03:17)

- Boot: shard activation alone takes ~9 s with continuous heartbeats (probe: module-fetch 6.4 s,
  module-ready 6.5 s, actor-ready 8.7 s, max ticker gap 1.0 s). First boot logged `shard 0 terminated`
  then `plugin-ui.native-owner-required`; later boots reach the puzzle3d shell (navbar, Example select
  showing Nakagin, Artifact/Catalogue/Inspection tabs, Top + Perspective window tabs) but BOTH window
  bodies are empty: no `[data-slot="window"]`, 0 canvases, dock panels have 0 children, no fault logged
  after the hook was installed. `__semioOsCatalogProbe` at 54 s: `ready:false, programs:[], spawned:[]`.
- The console is flooded by a peer's `[DEBUG] cooperative-maintenance` trace (`🔌️plugin/🦀️.rs:27533`),
  printed one token per line (500-entry buffer overflows in seconds).
- Peers editing shell TS (`🏛️ShellHost`, kernel/manifest TS) trigger Vite full reloads every 1-2 min,
  faster than the ~70 s boot → added `SEMIO_VITE_HMR=0` knob to `⚙️vite.config.ts` and serve Vite
  directly from the staged activation (`serve-only.sh`), since the Nx chain now re-runs `component-dev`
  and fails on a peer's in-flight puzzle 2d config refactor (`🎚️config/🦀️.rs` 04:43, commands 04:57).
- Release-profile component build blocked by the same 2d drift.

## Boot timeline 05:15 (React, wasm-dev component, page-level shard worker — nesting ruled out)

| t (s) | event |
|---|---|
| 5.3 | `activate` posted (worker script eval alone takes ~6 s; first heartbeat at 6.4 s in isolation) |
| 17.4 | `turn instance-open` posted → activation took ~12 s |
| 28.2 | `turn instance-lifecycle-ack` → instance-open took ~11 s in wasm-dev (487 ms in wasm-release on 09-07) |
| 28.2-28.5 | 16 sliced turns (`more-work` continuation) |
| 28.5 | `turn` with 8 × `surface-visible` (first render of both windows + panels) |
| 28.6 | last turn posted; then silence → the render slice exceeds the 3 × 5 s liveness policy → `shard 0 terminated` → `plugin-ui.native-owner-required` |

Conclusion: the dev-profile guest (opt-level 0 for every framework crate; only `semio-s-artifact-puzzle-3d`
is opt-level 2) is ~20× too slow for the interactive-step slicing to keep the watchdog fed on the first
render. The release-profile component is the fix (chain `serve-puzzle3d-react-release`); it is blocked by
a peer's in-flight puzzle 2d fill-runtime refactor (`◻️2d/…/🎚️config/🦀️.rs` uncommitted, 04:43-04:57).
Native opt-level-0 timings are small (Nakagin DSL parse 0.26 s, load 0.7 s), so this is wasm-dev
codegen cost, not an algorithmic loop.
- 05:19: release component build fails in puzzle 2d again — the ownership peer is now migrating 2d's
  window transient/config mutations (`◻️2d/…/✏️editor/🪟️window/🦀️.rs` 05:15: `print_op`/`parse_op`,
  `ProtocolError::Text`, duplicate `EphemeralEmit`). Hot → `release-when-cold.sh` polls every 4 min and
  builds the release component once those files are 8 min cold.
- 05:36: release component built (`dist/component-release/semio_s_plugin_puzzle.wasm`, 21.7 MB vs 88 MB
  wasm-dev) after the 2d peer edits went cold; release serve chain launched 05:37 (`dev-react-release-1.txt`).
  Main-thread long tasks during boot max 2.2 s (total 6 s) → the watchdog silence is the guest, not the page.

## Wave log 06:12

| wave | status |
|---|---|
| W-D | done — P1 P2 P3 P5 P6 P7 P8 P9 fixed + `nx test` now compiles the app tests (579); isolated suite 405→515 ok / 172→64 failed, whole suite 493/86 with 0 aborts; audits green (`📓️2026-09-09-wave-D-production-defects.md`). Not fixed: P4 = `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` vs ~182 one-item edits for a Nakagin example load; P10 = reserved-job pump treats transient pool contention as a hard fault. |
| W-O | running |

Coordinator decisions:
- P4 → whole-document replacement mutation leaf (`replace-fixture`) so an example load is ONE edit and one
  undo step (schema + quintet + Python oracle arm), not a bigger ledger. Wave W-E once the crate compiles.
- P10 → one retry-after-yield on `Submit(Pool(Contended | Saturated))` in the reserved-job loop of
  `🔌️plugin/🦀️.rs` (peer-hot; schedule when cold).
- 8 ms step-budget overruns (`openVortexSuggestions` 14.6 ms, `fillBuildTick` 11.7 ms, `acceptSuggestion`
  11.8 ms at opt-level 0) → finer slicing in `⏳️precompute` (wave W-P2).
- 06:05: the ownership peer split `Puzzle3dConfig` into a 4-field shared config + `Puzzle3dRuntime`
  (`🎚️config/🦀️.rs` rewritten); the 3d crate is uncompilable until their refactor converges
  (176 fallout errors + 5 of P2's mutation half). `poll-p3d-heal.sh` checks every 5 min.

## Runtime findings 06:35-07:00 (release guest, React)

- Fixed (framework, TS): `🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️.ts` value-tag switch lacked the pack
  `TAG_UINT` (0x04, LEB128) / `TAG_INT` (0x03, zig-zag LEB128) phases the Rust encoder emits since the
  09-08 evening pack change → every surface with an integral number was rejected
  (`plugin-ui.intake-rejected:typed-decode:Unknown UI value tag`). Also re-exported
  `artifactFrontierIsEditedForV1`/`artifactFrontierIsGenesisForV1` from `💻️os/🟦️.ts` (peer's ShellHost
  import had no export → module-graph SyntaxError, blank page).
- Browser-pane artifacts: hidden pane ⇒ `innerWidth/innerHeight = 0`, root rect 0×0 ⇒ shell in mobile
  mode (`Mode.mobile = true`) ⇒ no windows; `resize_window 1440×900` fixes the viewport (still hidden).
- Release guest boot: instance-open 1.8 s (dev: 11 s), 5 `surface-visible` at 2.8 s, then no further
  turns; `uiRefreshCacheRef` empty, `Mode.windows = []`; a direct `handle.refreshUi(1, {windows:[{key,
  bodyKey:"puzzle3d.play.composite"}]})` never resolves and posts nothing to the shard ⇒ the stall is
  host-side before the shard (suspects: the peer's new window-config lane read against a pre-split guest,
  instance guard single-flight, backbone store-worker import). Audit
  `📓️2026-09-09-react-refresh-path-audit.md` in flight; the release chain is rebuilding the component
  from the healed tree (06:51).
- Boot example: descriptor lists `nakagin-capsule-tower` before `concrete-forest` for the 3d app ⇒
  boot dispatches `setActiveExample nakagin` ⇒ P4 ledger ceiling. Order fix pending (audit item 4).
- 07:05 W-O done: outliner + catalogue virtualised (bounded page, per-section/row budget, `+N`
  continuation rows, fail-soft); `UiValue` arena derived from contract constants (434 pages) with
  `ui_value_headroom()`; Nakagin and 1200-object fixtures render bounded; ui-contract suite green
  (`📓️2026-09-09-wave-O-outliner-and-arena.md`). Open: inert continuation row (needs a paging cursor
  in config), inspection `ids` list unbounded, one-page-per-turn retirement pump.
- 07:15 root cause of the silent refresh stall (`📓️2026-09-09-react-refresh-path-audit.md` + measurement):
  the host UI-patch intake yields with `setTimeout(0)` every 8 steps (`🔌️PluginRuntime/🟦️.tsx
  yieldPluginUiContinuation`), and this Browser pane is permanently hidden (`document.hidden === true`),
  so Chrome throttles the timer chain to 1/s (1/min after 5 min) — thousands of intake steps became
  minutes of silence, holding the per-actor `command-ingress` FIFO so every later `refreshUi` queued
  with zero shard traffic. Fixed: MessageChannel macrotask yield (never visibility-throttled).
  Also swapped the 3d example order (`🧊️3d/…/✳️any/🦀️.rs`: concrete-forest first, matching
  `default_fixture()`), needs a component rebuild to reach the descriptor.
- 07:16 release chain re-materialized the component from the healed tree (18.25 MB); direct release
  serve restarted (`serve-release-direct-2.txt`).
- 07:33 ✅ puzzle 3d RENDERS on React (release guest, viewport 1440×900): both windows mount
  (`puzzle3d-main-top` 470×836, `puzzle3d-main-perspective` 947×836) with live WebGL, Top shows the
  orthographic grid + concrete-forest footprint, Perspective shows grid, gizmo and the hexagonal object;
  catalogue tree lists Objects/Vortices/Cables/Attractions. Fixes that got here: MessageChannel yield,
  size-proportional intake budget (`pluginUiIntakeBudget`: the wire decoder advances one phase per step,
  so 4 096 steps capped a surface at a few KB), per-step grant 256 items / 64 KiB.
  Remaining: every action fails `missing field locale` — the dispatch `ViewModel` lacked
  `locale`/`terminology` (refresh path injected them, dispatch path did not) → injected at the main
  dispatch site.

## Runtime findings 07:40-08:00 (release guest built 07:20-07:27, React, `📓️2026-09-09-runtime-verification.md`)

- Boot example is Concrete Forest; both windows render; catalogue/outliner/inspection panels fill.
- **Every plugin action turn spins in `more-work`**: with the plugin handle hooked from t=1 s, the first
  call `readLocalInteraction` (t=7.0 s) fails after 23 s with "did not publish its requested UI surfaces
  within 4096 continuations (effects=0, status=more-work)", `refreshUi` (t=7.1 s) succeeds in 18 s,
  `handleAction setActiveExample` fails the same way, and every later `handleAction` (window resize,
  pick, example switch, setActiveTool) never resolves. Host-side toggles (utilities bar, tool tab,
  brush pressed) only look alive because they are shell state. Instrumented the reactor turn
  (`⚛️reactor/🔄️turn/🦀️.rs` `[DEBUG] reactor more-work streak=… typed_operation=… reconcile=…`) and
  triggered a release rebuild (07:59, `heal-then-rebuild.sh`) to name the hot source.
- **Sections never reach the React shell** (measures/tools/engagements) → wave W-M launched 07:48
  (`📓️2026-09-09-wave-M-retained-sections.md`).
- **TS typed-operation lane cap was stale**: `typedOperationResult` rejected `lane > 11` while the Rust
  host emits Interaction 12 (W-S), WindowTransient 13 and WindowConfig 14 (ownership peer, uncommitted) →
  `TYPED_OPERATION_RESULT_LANE_MAX = 14` in `🔌️PluginRuntime/🟦️.tsx`.
- **Applied-edit ledger ceiling** (`ARTIFACT_HISTORY_LEDGER_CAPACITY = 64`, `push_applied` refuses at 64,
  one edit per mutation): fill > 64 placements and a Nakagin load fault at edit 65. Audit
  `📓️2026-09-09-applied-ledger-ceiling-audit.md` launched 07:48 (Sonnet) to choose between one Edit per
  Emit, compaction, or capacity. Note the hostile static law
  `set_active_example_hostile_static_law_rejects_whole_document_reset` forbids a single-step whole-document
  reset in the work loop — the fix must keep the cursorized work and batch the *edit*, not the steps.
- Peer breakage healed by me: `✏️editor/🦀️.rs:2975` kind-weight `step` parameter had been renamed
  `_config` while the body reads `config` (4 × E0425 since 07:28) → renamed back.
- 08:20 W-M done (`📓️2026-09-09-wave-M-retained-sections.md`): `engagements`/`measures`/`tools` are
  retained surfaces `${instance}:framework.section.*` carrying canonical JSON in ≤512-byte text leaves;
  measured 523 B / 10 462 B / 2 596 B for puzzle 3d. PluginRuntime vitest 74/74 after re-pinning the
  typed-operation authority test to lane 15 and the yield test to the production `yieldPluginUiContinuation`
  seam. Rebuild #3 (`heal-then-rebuild-2.txt`) queued behind the 3d crate compile (W-B edits the store).
- Launched 08:05 W-B (`📓️2026-09-09-wave-B-batched-edit-publication.md`, one Edit per Emit) and W-P2
  (`📓️2026-09-09-wave-P2-precompute-step-budget.md`, step slicing + the 30 red precompute tests).
- 08:55 measured root cause of the more-work spin (`📓️2026-09-09-runtime-verification.md` §08:55): on wasm
  the process pool executed 3 interactive job steps in 4096 reactor turns because it is pumped only by the
  cooperative-maintenance cadence. Fix in `🔌️plugin/🦀️.rs` (`drive_typed_operation_worker`) and
  `⚛️reactor/🔄️turn/🦀️.rs` (`pump_process_worker_pool`); rebuild #4 running (`heal-then-rebuild-3.txt`).
- 09:03 W-P2 done (`📓️2026-09-09-wave-P2-precompute-step-budget.md`): precompute 104/28 → 132/0; the fill
  planner had never run (worker-pool pump ≈4000 idle turns per step → caller-side `BatchJobSession`);
  fill projection/replan/close-census fixes; scene sync 39.4 ms → 0.18 ms. Still over 8 ms:
  `openVortexSuggestions` 10.2 ms, `fillBuildTick` publish turn 17.6 ms — four whole-document `Value`
  conversions in `handle_action_impl`'s prologue → wave W-P3 after W-B lands (shared command spine).
- 09:10 W-B done (`📓️2026-09-09-wave-B-batched-edit-publication.md`): one machine — `begin_apply_batch`
  replaces the one-item lane everywhere; 200 mutations → 1 ledger slot / 1 undo step proven;
  `nakagin_example_loads_via_operations` and the setActiveExample swap test green; norm apps' LIFO
  `reverse()` compensation deleted (norm test to run). Deviation accepted: app factories keep the
  per-item trait, the store chains N preparations (≈7 turns/mutation) — faster `Vec<Input>` shape is a
  follow-up.
- 09:05 rebuild #4 (worker pump): the 4096-continuation spin is gone (actions settle in ~1 s) but the
  boot actions now end in `typed-operation cancelled before its next publication unit` (a real job fault
  hidden by the lease cancellation) → fault-body retention added (`terminal_fault` on the mounted
  operation, `ArtifactBoundedToolFault::from_payload`); the served wasm then carried W-B's half state
  (`plugin.internal` traps). Rebuild #5 (`heal-then-rebuild-4.txt`) from the consistent tree.
- 09:15 `cargo test -p semio-s-plugin-norm --lib`: 16/16 ok (the `set_snapshot_dispatches_through_the_tool_job_path`
  name W-B cited does not exist in that lib — 0 matched; the norm artifact crates were not run).
  Launched W-P3 (`📓️2026-09-09-wave-P3-command-prologue.md`, prologue split for the 8 ms law) and W-T
  (`📓️2026-09-09-wave-T-typed-operation-fault-routing.md`, route lane-11 pages to the owning call).
- 09:25 W-T done (`📓️2026-09-09-wave-T-typed-operation-fault-routing.md`): lane-11 pages are routed by
  the wire's `sequence == 0` first reveal to the owning host call; foreign faults are parked (32) or
  surfaced once as `interactive-job.unattributed-result-fault`; vitest 76/76.
- 09:20 rebuild #5: actions settle in ~1 s; the boot actions then trap the actor with
  `plugin.internal.interactive-ceiling` — the cooperative-maintenance clock measured one
  `maintenance_step` at 10.2 ms (ceiling 8 ms) and killed instance 1. Added the per-stage
  `[DEBUG] maintenance stage=… elapsed_us=…` trace; rebuild #6 (`heal-then-rebuild-5.txt`) running.
- 09:40 rebuild #6: offender of the instance-fatal ceiling is `drive_store_replacement_jobs` (stage 14,
  8.4 ms/unit on release wasm) → W-R2 launched (`📓️2026-09-09-wave-R2-store-replacement-step-budget.md`).
- 09:50 W-P3 done (`📓️2026-09-09-wave-P3-command-prologue.md`): `Puzzle3dActionPrologue` (scene / sync /
  dispatch halves), typed scene build, mesh index; `openVortexSuggestions` 17.5 → 1.07 ms/turn,
  `fillBuildTick` 17.0 → 1.03 ms, `setActiveExample` 0.36 ms; precompute 132/132; component 89/44 (was
  58 failed isolated). Open: `Puzzle3dPlaySnapshot::new` default-document on null meta (§6.3),
  `worldRelocate` whole sync per dispatch (§6.6).
- 12:20 W-R2 done (`📓️2026-09-09-wave-R2-store-replacement-step-budget.md`): stage 14 was a stale
  cursor in my trace (idle early return) and every maintenance stage costs ≤43 µs natively; the ceiling
  verdict measures wall time and fires on descheduling (19 µs unit measured 14.6 ms under load). Decision:
  the cooperative-maintenance callback overrun is recorded, not instance-fatal (job-level step contract
  keeps quarantining slow steps). Also: `drive_store_replacement_jobs` never runs for puzzle 3d
  (`build_document_store_initialization_job` not overridden → `pollEnvelopeLoad` can never reach Ready),
  `fillBuildTick` still breaches 25.6 ms on turn 763, `setActiveExample nakagin` → `job-session.terminal-fault`.
- 12:15 audit `📓️2026-09-09-remaining-test-failures-audit.md`: 44 failures = 4 root causes (window-config
  lane never quiesces → per-window settings hang; setActiveTool/Utility missing-owned-reducer; addObjectKind
  no-op; suggestion popup transient dropped). Coordinator fix: host-owned `setActiveTool`/`setActiveUtility`
  with no app mutation dispatch an empty `Emit` (`dispatch_action`). W-D2 launched for the other three
  (`📓️2026-09-09-wave-D2-window-config-lane-and-scratch.md`).
- 12:50 rebuild #7: no trap; the real boot fault is `puzzle command wire payload is malformed` — the JSON
  action route admitted `(verb, args)` tuple JSON as the retained raw wire while the driver decodes the
  `encode_op` map form. Fixed in `dispatch_action`/`dispatch_command` (admit `encode_op(&command)` via
  `admit_command_wire`). Rebuild #8 pending on a peer's `🫧️transient` breakage.
- 13:45 rebuild #8 (wire fix): boot `setActiveExample` succeeds (2.4 s), sections + both scenes render,
  no faults; but the next host action (`noteShellCommand`) hangs: its 5th continuation turn never
  returns from the worker (reactor trace stops at streak=4 with `command_ingress=true`, no shard-lost).
  Added per-phase turn traces (`[DEBUG] turn N begin/phase/end`) and a reserved-job poll trace;
  rebuild #9 queued (`heal-then-rebuild-8.txt`). W-D2 still running.
- 14:25 second wasm pool gap (`📓️2026-09-09-runtime-verification.md` §14:20): framework-reserved routes
  run under `resolve_ready`, whose spin never pumps the cooperative pool → `noteShellCommand` polled
  `Submitted` 4096+ times inside one turn; every undo/redo/clipboard/history route hangs on wasm.
  Fix: `run_framework_reserved_job` pumps the pool after each non-terminal poll on wasm. Rebuild #11
  queued (`heal-then-rebuild-10.txt`).
- 16:00 W-D2 done (`📓️2026-09-09-wave-D2-window-config-lane-and-scratch.md`): window-config lane hang
  root-caused (`🪟️window/🎚️config/🦀️.rs:82` compared the 4 096-byte per-turn grant against the owner's
  65 536 schema ceiling → `Blocked` forever for every owner above 4 KiB) and fixed; `addObjectKind`
  materializes the default catalog; popup transient + `take_ephemeral` for acceptSuggestion; component
  89/44 → 96/38 (22 of 38 are the load-flaky `job-session.terminal-fault`; box load avg 89-95).
  Peer composition wave (`ArtifactCompositionFields` on `Puzzle3dPlaySnapshot`) keeps the workspace
  uncompilable since 15:04; the wasm retry loop (`rebuild-until-ok.txt`) waits it out.
- 16:45 checklist audit written (`📓️2026-09-09-user-feature-checklist.md`, 25 sections; the agent stalled
  after writing it) → W-D4 launched for its source-read defects (zoomToSelection unregistered, outliner
  hide/lock never toggles off, Add Object dialog kind selector static, engagement placeholder grammar,
  silent engine failures). Peer composition wave requires `ArtifactCompositionFields` on the puzzle play
  snapshots → implemented for 2d/3d/5d (3d delegates to the typed `Puzzle3dSnapshot` derive) so the wasm
  rebuild loop can proceed. W-Q and W-D3 running.
- 17:20 rebuild #11 verified (`📓️2026-09-09-runtime-verification.md` §17:10): boot actions succeed; the
  async completion gap (seq-0 completion frames dropped, no drain polling, history carrier bug) → W-A
  launched; W-M2 (paged mesh upload) launched at 17:12; rebuild #12 (trace retune) running.
- 17:56 the Claude Code process restarted: the session scratchpad was wiped (seeded cargo targets,
  scripts, build logs gone; the release server died) and waves W-Q/W-D3/W-M2/W-A/W-D4 were killed
  mid-flight (W-Q report complete; W-D3 report §4/§5 empty; W-M2/W-A/W-D4 partial edits only).
  Re-seeded `target-p3d` from the repo `target/` (debug 75 GB + wasm-release 1.2 GB), recreated the
  serve/rebuild scripts, relaunched W-A, W-M2, W-D3, W-D4 as continuation waves (they wait for the
  `.seeded` marker), queued the wasm rebuild loop behind the seed.

## 18:55 close-out inventory — temporary traces owned by this ticket (remove before close)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — 7 `[DEBUG]` line(s):
    - 50:    /// 🐞️ `[DEBUG]` turn sequence for the phase trace — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
    - 52:    /// 🐞️ `[DEBUG]` more-work streak trace: (current streak, total more-work turns) — temporary, ticket 26/09/02/PUZZLE-3D-END-TO
    - 129:        eprintln!("[DEBUG] turn {turn_seq} begin events={} page={}", events.len(), command_page.is_some());
    - 848:                "[DEBUG] reactor more-work streak={streak} seen={seen} executor_deadline={} close_cleanup={close_cleanup_work} typed_ope
    - 853:            eprintln!("[DEBUG] reactor more-work streak ended after {} turns (seen={seen})", trace.0);
    - 881:        eprintln!("[DEBUG] turn {turn_seq} end more_work={more_work}");
    - 941:            eprintln!("[DEBUG] pool pumps={pumps} remaining={remaining} before={before:?} after={:?}", pool.try_cooperative_snapshot());
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — 12 `[DEBUG]` line(s):
    - 16364:                "[DEBUG] registered keyed dispatch progress stage={:?} terminal_seen={} pending={} completion={} result={:?}",
    - 16389:            eprintln!("[DEBUG] actual registered keyed dispatch {} preserved exact target supersession, rebased its worker, published 
    - 16447:            eprintln!("[DEBUG] actual registered keyed dispatch {} preserved foreign reservation, retired seven UTF-8 bytes exactly, a
    - 17993:                    eprintln!("[DEBUG] reserved job '{verb}' poll {reserved_polls}: {poll:?}");
    - 22961:    /// 🐞️ `[DEBUG]` last maintenance stage entered — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
    - 22963:    /// 🐞️ `[DEBUG]` typed-operation publication turn counter — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
    - 24003:                    "[DEBUG] typed-operation publication turn={trace_turn} operations={operations:?} latest_wins_empty={} effects={} e
    - 28300:            std::panic::set_hook(Box::new(|panic| eprintln!("[DEBUG] [semio-plugin panic] {panic}")));
    - 28653:                    eprintln!("[DEBUG] maintenance stage={} elapsed_us={} outcome={:?}", crate::app::LAST_MAINTENANCE_STAGE.load(Order
    - 28735:                eprintln!("[DEBUG] cooperative maintenance callback overran the interactive ceiling for instance {} (elapsed {elapsed_
    - 29249:            eprintln!("[DEBUG] native close deadline instance={} generation={} candidate={status:?} elapsed_us={elapsed_us}", state.in
    - 29415:                "[DEBUG] cooperative-maintenance instance={} turn={} generation={} status={}->{} entries={} phase={:?} clock={} pool={
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/🦀️.rs` — 1 `[DEBUG]` line(s):
    - 214:    /// 🐞️ `[DEBUG]` state summary — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🦀️.rs` — 1 `[DEBUG]` line(s):
    - 109:    /// 🐞️ `[DEBUG]` state summary — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` — 1 `[DEBUG]` line(s):
    - 513:                        eprintln!("[DEBUG] puzzle command wire malformed: raw_len={} pages={} scan={} error={error:?} head={:?}", self.r
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — 39 `[DEBUG]` line(s):
    - 267:  console.error(`[DEBUG] PluginRuntime: shard ${shardIndex} lost, restoring actors: ${actorIds.join(", ")}`);
    - 291:    onActorTrap: (actorId, message) => console.error(`[DEBUG] PluginRuntime: actor ${actorId} trapped: ${message}`),
    - 760:    if (!record) throw new Error(`[DEBUG] retained UI surface ${surface.surface} references missing node ${id}`);
    - 789: * here degrades to an honest `[DEBUG]`-logged drop rather than guessing an unverified shape. */
    - 833:      console.warn(`[DEBUG] wireEffectToFriendly: unmapped effect "${effect.tag}" dropped — unverified wasm-boundary conversion (this 
    - 959:      console.warn("[DEBUG] thunk start", _actorId);
    - 962:        console.warn("[DEBUG] thunk done", _actorId);
    - 964:        console.warn("[DEBUG] thunk failed", _actorId, error instanceof Error ? error.message.slice(0, 160) : String(error));
    - 980:    if (backpressure.kind === "rejected") reject(new Error(`[DEBUG] serializePerActor: actor ${actorId}'s queue is full (>${SERIALIZE_PE
    - 1070:    onTurnError: (actorId, error) => console.error(`[DEBUG] PluginRuntime: turn failed for actor ${actorId}`, error),
    - 1127:        reject(new Error(`[DEBUG] PluginRuntime: actor ${actorId}'s turn queue is full — rejected rather than growing unbounded`));
    - 1133:    if (backpressure.kind === "rejected") reject(new Error(`[DEBUG] PluginRuntime: actor ${actorId}'s turn queue is full — rejected r
    - 1252:    if ((continuation + 1) % 512 === 0) console.warn(`[DEBUG] settle ${actorId} continuation ${continuation + 1} status=${wireTurnStatu
    - 1257:      `[DEBUG] PluginRuntime: actor ${actorId} did not publish its requested UI surfaces within ${PLUGIN_UI_CONTINUATION_LIMIT} continu
    - 1265:    throw new Error(`[DEBUG] PluginRuntime: actor ${actorId} stopped without publishing requested UI surfaces (missing=${JSON.stringify
    - 1636:    if (!actorId || closingInstances.has(instanceId)) throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId}
    - 1725:            if (terminal === "fault") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress fault: ${commandIngressFaultDisplay(
    - 1726:            if (terminal === "backpressure") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress backpressure after serialized
    - 1731:            if (continuation % 32 === 31) console.warn(`[DEBUG] command ingress continuation ${continuation + 1} status=${terminal ?? "
    - 1733:          console.warn(`[DEBUG] command ingress settled status=${terminal ?? "missing"} observed=${[...observedStatuses].join(",")}`);
    - 1734:          if (terminal !== "command-complete") throw new Error(`[DEBUG] plugin ${pluginId}: command ingress did not complete within 102
    - 1793:      if (outcome.stopped === "budget") console.warn(`[DEBUG] typed-operation drain for instance ${instanceId} exhausted its ${PLUGIN_O
    - 2066:      console.warn(`[DEBUG] applyRetainedWindowPatches: actor ${actorId} desynced (unrecognized op shape or stale baseRevision) — kee
    - 2103:  throw new Error(`[DEBUG] coerceWireBytes: unsupported payload ${JSON.stringify(raw)?.slice(0, 120)}`);
    - 2206:    if (!client) throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already 
    - 2239:      if (!frame) throw new Error("[DEBUG] readHistory: missing HistorySnapshot frame");
    - 2246:      if (errorFrame) throw new Error(`[DEBUG] applyMutations failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`)
    - 2257:      if (errorFrame) throw new Error(`[DEBUG] readAppDocumentPack failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValu
    - 2264:      if (errorFrame) throw new Error(`[DEBUG] loadAppDocumentPack failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValu
    - 2289:      if (!frame) throw new Error(`[DEBUG] program ${pluginId}: transactionPrepare(${instanceId}): missing transactionPrepared frame`);
    - 2301:      throw new Error(`[DEBUG] program ${pluginId}: transactionCommit(${instanceId}): missing transactionCommitted/Error frame`);
    - 2316:      if (errorFrame) throw new Error(`[DEBUG] program ${pluginId}: setMergePolicy failed: ${faultDisplayMessage(errorFrame.Error.fault
    - 2321:      if (errorFrame) throw new Error(`[DEBUG] program ${pluginId}: resolveConflict failed: ${faultDisplayMessage(errorFrame.Error.faul
    - 2332:      if (errorFrame) throw new Error(`[DEBUG] program ${pluginId}: readConflicts failed: ${faultDisplayMessage(errorFrame.Error.fault,
    - 2652:          console.warn(`[DEBUG] TransactionCoordinator rollback(${member.pluginId}#${member.instanceId}) failed`, error);
    - 2666:          console.warn(`[DEBUG] TransactionCoordinator undo(${member.pluginId}#${member.instanceId}) failed`, error);
    - 2693:          console.warn(`[DEBUG] TransactionCoordinator redo(${member.pluginId}#${member.instanceId}) failed`, error);
    - 2805:      loadFailures.push({ pluginId: entry.pluginId, error: new Error(`[DEBUG] loadPluginModulesInDependencyOrder: ${entry.pluginId} ski
    - 2811:        loadFailures.push({ pluginId: entry.pluginId, error: new Error(`[DEBUG] loadPluginModulesInDependencyOrder: ${entry.pluginId} s
- `debug_state()` helpers (🐞️) on the live/query interaction types — delete with the trace that reads them.

## 19:00 W-A landed

- W-A complete: `AppFrame::OperationCompleted` (tag 25) + TS twin + fixture, Rust `typed_completion_outbox` drained at the Terminal page, `AppChannelClient` pre-correlation routing → `ShellHost` history patch + UI scope, history carrier fixed (`decodePackWire`, no more `[object Object]` rows), bounded drain poller. Report `📓️2026-09-09-wave-A-async-completion-refresh.md`. Not run in browser yet — covered by rebuild #13 attempt 2 (started 18:44, after the edits).

## 19:50 W-J launched — bounded fill job kind

- Browser on the fresh wasm: fill measures render, ticks run, `ready` stays 0, guest OOMs after ~115 ticks. Cause: `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:71` registers `FILL_JOB_KIND` as an async `JobFn`; `⚛️reactor/💼️jobs/🦀️.rs::spawn_job` only executes `register_bounded_job_kind` kinds outside `cfg(test)` (`job.explicit-state-machine-required`). fem 3d/2d and energy already converted. W-J (Opus) converts the fill job to a `BoundedJob`, verifies the React host steps `Isolated` jobs, bounds the tick spawn, adds laws. W-S (settle stall) and W-J share `⏳️precompute/🦀️.rs` — W-J told to keep to the job-kind boundary.
- Serve chain fixed (`🔨️serve-release-direct.sh` PLUGIN_SCRIPT path) — `Activated … (changed)` at 19:26 is the first fresh wasm served since 17:07.
- Peer breakage fixed: `node:fs` import in `🔌️plugin/📇️registry/🟦️.ts` moved to `📇️registry/✅️trusted-stdio-catalog/🟦️.ts`.

## 20:00 W-I landed

- Root cause: the local interaction query was stuck in *closing*, waiting for the Store one-slot-per-step reclaim cursor (1 probe / 8 continuations / 1024 slots) — 8×1024 continuations per lease vs the 4096 drain budget. Fix: query ownership ends at registry acceptance; returned-read pumps moved into `maintenance_step` (fill + drain over all three stores); terminal reply ungated. Read cost 127 → 27 turns, 5 puzzle laws green. Needs rebuild #14 (plugin host Rust). Report `📓️2026-09-09-wave-I-local-interaction-query-termination.md`.
- Vite boot currently broken by a peer: `💻️os/🟦️.ts:2393` imports the not-yet-existing `🧪️tests/🧊️mesh-pack-decode/🟦️.ts` (mesh pack codec, 19:49). Waiting for the peer file before the next browser pass.

## 20:05 settle stall — likely cause found by W-I, fix applied to the testkit

- W-I §6: `dispatch(setActiveExample)` loops to the 1 048 576-turn guard with `operations=[] … ui=0` — the typed **completion** outbox (W-A, `typed_completion_outbox`, counted in `has_pending_typed_operations`) is never drained by the testkit `settle`, which only drains effect/event/ui. Coordinator added `take_typed_operation_completion().await` to `🧪️tests/🔬️testkit/🦀️.rs::settle` (completion `ui_scope` replaces the UI-lane scope). Reproduction run in the scratchpad (`settle-repro.txt`). W-S should treat this as the primary lead.

## 20:10 completion refresh owner fix + mesh-pack test module

- W-A gap: `subscribeOperationCompletions` in `🏛️ShellHost/🟦️.tsx` captured its effect owner with `presentation: null`, so `isCurrentEffectOwner` was always false and the completion full-scope refresh never ran (measured: Nakagin completion arrived, nothing refreshed). Fixed to `captureEffectOwner(target, captureDialogOrigin(target))`; `[DEBUG] completion apply` / `applyHostEffects refresh` traces added (remove at close).
- Peer blocker: wrote `💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts` (five decoder laws) so `💻️os/🟦️.ts:2393` resolves; os TS suite running in the scratchpad (`os-ts-test.txt`).
- W-I lead for W-A: `nakagin_example_loads_via_operations` spins at `608:Publishing:true:true` natively.

## 20:22 status

- os TS suite with the new mesh-pack laws: 5 files / **348 passed** (was 343).
- Rebuild #14 launched 20:18 (W-I Rust + everything landed so far); W-S, W-J, W-D3 still running (W-J is fixing the shard-client job wire `u64` identities — the isolated job stepping path).

## 20:30 W-D3 landed

- Fixed: settle/measured_host_turn drain the completion witness; brush-lane reset moved out of `start_fill_preparation` into `rebuild_queue` (a no-op scene sync wiped the brush candidates on every render); `suggestionsTick` warms the looked-at target; `window_measures`/`tool_measures` read the live precompute session. `missing-owned-reducer` 0 occurrences; module 81/58 → 87/57. Report `📓️2026-09-09-wave-D3-fixture-viewmodel-and-candidates.md`.
- Open, to assign after W-S/W-J land (same files): (a) scene surface hits the 32 KiB cap when the suggestion popup carries content (`meshes_json` 27 673 B of 33 289 B) — scene-payload budget; (b) `puzzle3d_view_session_key` is `None` for a bare `window_measures` call (chrome runs session-less/cold); (c) process-wide fill-worker session leak (W-S/W-J); (d) retained-jobs fixture still lists `setFillCountStep`.

## 20:35 W-S landed — settle stall root cause was worker-session admission leakage

- Not the precompute loop: `WorkerJobSession` admissions come from a process-wide 256-slot array; `Drop` parks a retirement node that only `pump_worker_job_retirements` returns, and no app runtime ever called it → after 256 leaked sessions every operation is refused, walks to `Publishing` with an empty completion slot, and `settle` spins. Fix: maintenance stage 23 pumps one retirement unit (`MAINTENANCE_STAGES` 24), refused admissions publish `interactive-job.admission-capacity` in one turn, reserved routes retry transient pool `Contended` rejections instead of hard-faulting (`interactionHover`/`interactionSelect` coin flip). Suite 516/85 → **581/30**; wasm check clean. Report `📓️2026-09-09-wave-S-settle-stall.md`.
- Residual 30: 14 `fill_worker` cascade (first assertion poisons the group static), 16 fill-planning / render-capacity (`scene-surface.encode … 33527 bytes`) / localisation assertions.
- Escalation: stage 23 is shared by every artifact app — run the other artifacts suites + repo check before close. Machine hit ENOSPC once (disk tight).

## 20:45 W-J landed; rebuild #15

- W-J: `Puzzle3dFillBoundedJob: BoundedJob` replaces the async `fill_job` (registered via `precompute::initialize()` from the plugin root); React host never started/stepped isolated jobs — `spawn-job` was dropped in `wireEffectToFriendly` and nothing called `ShardClient.startJob/stepJob`; fixed with a job driver in `🔌️PluginRuntime/🟦️.tsx` (`driveSpawnedJob`, `deliverJobCompletion`) + shard-client wire `u64` fixes + `cancelJob` route; `fill_faulted` latch stops the per-tick re-preparation growth, one `fill_failed` notice. Laws: 4 Rust + precompute 146/146, shard-client vitest 151, PluginRuntime 82. Open (§5): wgpu host still drops `spawn-job`; `Event::JobProgress` dead on both targets; bounded factories never receive `restore_job` checkpoints (fem/energy); editor testkit `drive_fill_until_ready` bypasses the job runtime. Report `📓️2026-09-09-wave-J-bounded-fill-job.md`.
- Rebuild #15 launched 20:45 (W-J Rust, per-turn reactor traces removed). Browser on #14: Nakagin switch → completion → refresh spins in `reconcile` (see runtime note 20:35).

## 21:15 W-L launched

- Browser interaction defects from the 21:05 pass: duplicate-selection no-op, raw `menu.group.*` labels in React, delete rows unconditional, fill tick over-queue + view ticks dirtying history, brush-mesh uploads triggering 14 refresh/history round trips at boot, empty inspection body. Report `📓️2026-09-09-wave-L-browser-interaction-defects.md`.

## 22:05 reconcile spin root cause found and fixed (coordinator)

- Per-slot tracker trace (rebuild #17) showed the artifact panel reconcile job abandoned by `drive_job_one` (`reconciler.is_some() && !job.is_ready()`) leaving its reserved ready output neither published nor closing → `has_work()` true forever → every drain after a document switch spun to 4096 continuations. Fix: `close_output` before retiring the abandoned job (`⚛️reactor/🩹️patches/🦀️.rs`); law `abandoned_reconcile_job_closes_its_ready_output_so_the_tracker_can_idle` added (plugin-host test target currently blocked by peer test files — run when it compiles). Rebuild #18 launched 21:22 (machine clock).
- Traces added this round (remove at close): `PatchTracker::debug_state`, `PendingPatchAuthority::debug_state`, `patches=[…] pending=[…]` in the more-work streak trace (threshold 2048).

## 22:20 W-K stalled → W-K2 relaunched; rebuild #18 reverted

- Rebuild #18 (close the abandoned job output) broke boot: the first refresh ended `status=idle` with `missing=[inspection, settings, section.engagements, section.tools]`, i.e. the abandon branch also fires at boot for those four surfaces and closing their output loses their first publication. Reverted to the original abandon behaviour plus a `[DEBUG] reconcile abandon …` trace (surface, generation, ready state) — rebuild #19 — to see why a slot has a reconciler while its job is unfinished (the grant cancel/drop path restores `slot.reconciler`; `receive_job_into` returns `Pending` while `current.is_some()`, so a reconciler present during a transfer both blocks the transfer and triggers the abandon — the fix must live in that hand-over, not in closing).
- W-K stalled (watchdog) with §1 written and edits in editor/main window/plugin host/World3dHost/ShellHelpers/testkit; W-K2 launched to attribute the hunks (W-L shares World3dHost/ShellHelpers), finish 1.1–1.3 and run the suite.

## 23:05 reconcile strand — rebuilds #19/#20/#21

- #19 (trace, original behaviour): the abandon branch is the normal post-`Empty`/`Published` job retirement (fires for every surface at boot), so closing there unconditionally (#18) broke boot. #20 (close only when unpublished): boot clean, Nakagin still strands `artifact#g26` with `output_index` intact → the strand comes from another job→terminal path (closing-instance branch), not from abandon. #21: `close_stranded_outputs` sweep in `PatchTracker::close_step` (unpublished + not closing + no live producer/job on its slot → close, clear `output_index`). Law `abandoned_reconcile_job_closes_its_ready_output_so_the_tracker_can_idle` covers the outcome (plugin-host test target still blocked by peer test files).

## 23:30 W-L + W-K2 landed; producer close semantics fixed

- W-L: context-menu rows were the *shell fallback* (guest returned `[]` from seven silent branches; selection arms ran over empty ids; `dispatch_step` abort dropped effects); TS twin of `ribbon_parent_label` from the one `ui.ribbon.parent.*` table (+ `menu.group.more` key); tick in-flight gate now awaits the guest completion (`onAction` settles on `OperationCompleted`, 30 s watchdog — timing change for every scene host); `applyHostEffects` no longer mints a new session per dispatch (killed the 90 `readHistory`); `register_brush_mesh` now sets its `HostOnly` scope; inspection body drop instrumented only. Suite 590/23. Report `📓️2026-09-09-wave-L-browser-interaction-defects.md`.
- W-K2: mesh kinds ride as `{id,kind}` (no tessellation in the scene payload), `live_render_operation` on every host lane, fixtures fixed (5d `toolIds` was missing 5 ids), a transient-read leak in a test. Suite **591/22** (22 = fill_worker/fill_build_tick family + 2). Report `📓️2026-09-09-wave-K-scene-payload-and-residuals.md`.
- Coordinator: after the stranded-output sweep (#21) the Nakagin spin persisted on `producer_terminals=[section.measures:cARV]` + `terminals=[g26:c--]`: `ComponentTreeProducer::close_step` (`🖱️ui/🧠️runtime/…/🎭️present.rs`) only completed when the GLOBAL built-child retire pool was empty (impossible with live surfaces) and nothing else drained that pool. Fixed: producer close completes when its own owners are released; the reactor turn drains one built-node page per turn (`close_built_node_page_one`). Rebuild #22.

## 23:55 terminal drain throughput

- After the producer-close fix (#22) the Nakagin spin narrowed to two job terminals (`g26` artifact, `g33` tools) drained one unit per 64 turns by `PatchTracker::close_step`'s slot cursor. Fixed: the cursor jumps to the first live closing terminal each turn; law `a_closing_terminal_does_not_wait_behind_sixty_three_empty_slots_per_unit` added. Abandon trace removed. Rebuild #23 (launched before the cursor edit but before the host compiled — verify the string `stranded output` and behaviour; else #24).
- ui-contract `close_built_node_page_one` now reports "close queue empty" (live reservations excluded) + `built_node_pages_are_terminal_empty` for shutdown witnesses.

## 00:25 more-work classification

- `reconcile_work` now derives from `PatchTracker::has_publishable_work` (terminal retirement excluded); the turn retires up to 8 terminal units (`PATCH_CLOSE_UNITS_PER_TURN`). Rebuild #24 (cursor fix) in flight; #25 will carry this.

## 00:45 W-N launched

- W-L §5 open items: inspection body empty after pick, partial scopes with empty `panel_bodies`, translate/rotate refusal notices unobservable, wgpu `menu.group.more` label. Report `📓️2026-09-09-wave-N-inspection-scopes-notices.md`.
- Rebuild #24 (cursor fix) still spins on the artifact terminal (expected — classification fix is in #25, compiling).

## 00:58 milestone + W-P

- Reconcile spin after document switch is fixed (rebuild #25). The Nakagin refresh now fails fast on the real cap: `scene-surface.encode … 57281 bytes` → W-P launched (paged scene lanes, schema-first, TS assembler, laws). Report `📓️2026-09-09-wave-P-paged-scene-payload.md`.

## 00:15 (2026-09-10) — Fill unblocked at the host, OOM found, rebuild #26

- Fixed in TS (live via HMR): tool activation shipped a window-less view state → host `unknown action window instance` (see runtime note 23:20 section). Fill now activates; the tick loop runs.
- Fill guest OOM (`memory allocation of 16384 bytes failed` after ~180 ticks, 512 MB cap) → **W-F** (Opus, running): native law with a counting allocator, fix the per-tick retention, also the 2 MiB stack overflow of `fill_and_brush_params…` natively.
- Shell quirks (Fill tab pressed while inactive; Tool category needs two clicks) → **W-G** (Opus, running).
- Read-only audits (Sonnet, running): `📓️2026-09-10-debug-trace-inventory.md` (close-out list of every `[DEBUG]` trace + helper added by this tree) and `📓️2026-09-10-brush-mesh-upload-audit.md` (seven 0.65–2.85 s `registerBrushMesh` uploads with `notify` refusals on window activation).
- Peer landed a fourth refresh section (`catalogue`, `framework.section.catalogue`) in `🛂️manifest/🟦️.ts`; with wasm #25 the shell fails every refresh with `plugin-ui.section-root-mismatch` → browser verification paused until **rebuild #26** (launched 00:12, `rebuild-until-ok.sh`, carries the pool-trace removal + tidy and the peer's Rust half).
- Still running from before: **W-N** (inspection body / partial scopes / refusal notices / wgpu label), **W-P** (paged scene payload lanes).
- Next after #26 serves: re-run pick → inspection body, context menu, gumball, undo/redo, duplicate, copy/paste, marquee, suggestions popup, Nakagin (paged lanes), Fill (after W-F) with the hook + timer shim + `scrollTo(0,0)`.

## W-G — Fill tab pressed state & Tool category reveal (done, 2026-09-10)

Report: `📓️2026-09-09-wave-G-shell-tool-tab.md`. Both handed-over quirks are **one** root cause.

- **RC-1**: tool activation hung off `ShellHost`'s press callback (`buildPanelSelectionProps.onActiveTabPathChange`)
  only. A path arriving any other way — the `DockUiStateStore` arrangement restored on boot (it persists
  per-anchor `visible` **and** `path`), the introduction's `SET_PANEL_PATH`, a program `setActiveTool` —
  left the tab selected over an unarmed tool. `progressPanelTabSelection` then read the user's first press
  as a re-press of the active segment and *collapsed* it, so only the second press armed the tool.
- **RC-2**: `buildToolTree`'s inactive branch rendered `<Toggle id={"tool.<id>"}>` — a **duplicate DOM id**
  with the leaf tab button (both answered to `#tool.fill`, which is also the tutorial's click target). That
  toggle is what dispatched the reported `setActiveTool {toolId:""}`.
- **RC-3** (= "Tool category needs two clicks"): the restored arrangement has bottom-middle already open on
  the Tool category, so press #1 is an active-root re-press → fold (exactly one `handleAction`,
  `shell.panelToggle`), press #2 reopens. On a cleared profile one press has always opened the category.
  The "8 `handleAction`s of increasing latency" are `World3dHost`'s `fillBuildTick` firing in duplicated
  pairs (~2/s) once Fill is armed — the known tick over-firing, not the press.
- **Fix**: one state, one owner. New pure `reconcileToolTabSelection(previous, activeToolId, selectedToolId)`
  in `ShellHelpers` (last-change-wins: tool moved → `select` the leaf; selection moved → `activate`;
  self-healing, non-bouncing) driven by a single `🧭️DockAssembly` effect in `ShellHost` that covers every
  route into the path (desktop anchor + mobile panel), skipped while the Tool category is not the active
  root so an armed tool survives browsing another category. The in-tree activation toggle is deleted;
  `buildToolTabs(tools, toolMeasuresByToolIdRef, onAction)` lost `controllerId`/`activeToolIdRef`.
- **Laws** (engine-contract, 6 new + 1 rewritten): duplicate-id ban (proven failing against the old shape),
  hydrate-arms, collapse-disarms/one-press-rearm, program→tab direction, refusal self-heal, and
  "one press on the Tool category reveals its remembered leaf and arms that tool".
- **Verification**: `typecheck` 821 errors, **0 in any touched file** (all pre-existing peer breakage);
  `test` 1 passed/4 skipped; `test long --run engine-contract` **467/467**; `test long` **751/751** in 19
  files. Browser (`:6013`, seeded profile): Fill now boots **armed with its measures** (`Count 0 ·
  Hexagonal Cut · Concrete Forest · Left 100% · Distribution`) at zero presses, `#tool.fill` resolves to a
  single element, fold/reopen is one press each and never disarms the tool.
- Handed on: duplicated `fillBuildTick` pairs, `plugin-ui.intake-budget-exhausted` on arming,
  `plugin-ui.section-root-mismatch` refresh errors — all other waves' territory.

## 01:30 (2026-09-10) — Relaunch after the usage-limit kill and the shared-target stall

- 00:22 usage limit killed W-P, W-F, W-H mid-work; relaunched as W-P2/W-F2/W-H2 (continuations from `git diff HEAD`), which all stalled at ~01:15 waiting on the ONE private cargo target (dev chain + three waves + the host test build). Now W-F3 (`target-p3d-f`), W-P3 (`target-p3d-p`), W-H3 (`target-p3d`) run on APFS clones (`cp -Rc`, 14 s for both).
- W-G landed: tool tab pressed state / one-press category with engine-contract laws (467/467), browser re-verified by the wave.
- Coordinator: `has_publishable_work` no longer counts a deferred surface waiting on the host's ack (+ law); `bun dev:puzzle:3d` entry fixed twice — `runCmd` now reaches workspace scripts before same-named bins (`workspaceScriptExists`, law in `⏱️process-budgets`), and the caching bootstrap clears a stale `d/disabled` marker and starts the Nx daemon before `nx watch` (`NxScript.ensureDaemon`). The dev chain then reaches the puzzle plugin build, which fails only on in-flight wave edits (duplicate `HashMap` import; `ui_value_text(...)?` after a peer's helper signature change) — rerun `dev3d.sh` after the waves.
- Build #26 verified: boots, both windows; blocked at the Perspective intake stall (W-P3) for everything after window activation.

## W-H3 — Brush-mesh re-announce storm on window activation (done, 2026-09-10)

Report: `📓️2026-09-10-wave-H-brush-mesh-reannounce.md` (audit: `📓️2026-09-10-brush-mesh-upload-audit.md`).
Continuation of W-H (usage-limit kill) and W-H2 (stream stall); most of the mechanism had landed, W-H3
settled the open cost question, verified everything and closed the report.

- **RC-1 — a host belief outliving the fact it described.** `registeredPuzzle3dBrushMeshes` was a
  page-lifetime `Map` in `🛠️ShellHelpers/🟦️.tsx`, written at *enqueue* time; the guest's
  `brush_mesh_store()` is a `static OnceLock` in one wasm instantiation and a restored actor holds
  nothing. Every later Perspective activation re-announced seven identities by `{url, digest}`, each was
  refused into `Effect::Notify { "puzzle3d-register-mesh-digest: <url>" }` — a code no TS line reads — so
  the brush utility silently kept no collision geometry until a full browser reload.
- **Fix A — lifetime scoping without a nonce.** The guest publishes a monotone
  `interactionJson.meshResidency` (`BRUSH_MESH_INSTALLS`, bumped in `derive_brush_mesh`, held outside the
  store's `Mutex` so a contended read cannot forge a zero). `Puzzle3dBrushMeshRegistry` replaces the bare
  `Map`: a residency below the page's high-water mark proves a re-instantiation and voids every claim.
  A claim is now `confirm`ed on a run's LAST page, never at enqueue.
- **Fix B — the refusal is a request for bytes.** The id-only `adopt_shared_mesh` miss records the url in
  the session's bounded `mesh_reupload_requests` and widens the otherwise-`Quiet` scope to
  `puzzle3d_viewport_scope()`, so the world body republishes it as `meshReuploadUrls`. The host claims
  each once per publishing residency and bumps a per-url `revision` that `BrushMeshRegistrar` carries in
  its effect deps — the only thing that can re-announce a `useLoader`-cached GLB. The request retires the
  moment real geometry installs.
- **RC-2 — the 0.65 → 2.85 s per announcement.** The *rise* is FIFO queueing (≈370 ms per position on one
  actor), not per-call growth. The ≈370 ms itself was the prologue's precompute **sync**:
  `"registerBrushMesh"` was a member of `puzzle3d_action_uses_precompute` at `9b605a4550`, so every
  dispatch first seeded a scaled-box fallback for each mesh id the session lacked (in-tree measurement:
  0.6 ms × twelve on the 180-object Nakagin document, each clearing `brush_queue`/`brush_cache` and
  rebuilding the queue), then built and pushed the whole engine scene — 17.6 ms native per W-P3 — for ids
  the arm was about to supply and whose fallbacks its own `register_mesh` then discarded. Self-cancelling
  work, removed; the native law asserts the non-membership with that rationale so it cannot come back.
  Residual, documented not done: `scene_step` still materializes a provably inert scene for this action
  (wants a declared "reads nothing from the scene" class).
- **Laws**: engine-contract — "re-pages every mesh instead of re-announcing when the guest restarted",
  "claims a guest re-upload request once per residency and drops the stale claim" (+ the id/digest law
  rewritten against the registry). Native — `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`
  (refusal emits no `Notify`, widens to the world body, publishes the url, the page run retires it and
  climbs the residency, and the identity is then adopted id-only at zero wire cost).
- **Verification**: `typecheck` **820** errors, **0** in any touched file; `test long --run engine-contract`
  **469/469** (was 467). Native `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly`:
  `--no-run` clean with warning output; `an_id_only_announcement` **1/1**; `mesh` **21 passed / 1 failed**;
  whole crate **607 passed / 23 failed** — all 23 in W-F3's fill lane. Including
  `a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh`, ruled out by a temporary
  `[DEBUG]` census dump: the differing field is `fillBuild.done` (`false → true`), while `meshResidency`
  and `meshReuploadUrls` are byte-identical.
- **Environment note**: `the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity`
  overflows the default test-thread stack and `SIGABRT`s the binary (truncating the run summary);
  `RUST_MIN_STACK=67108864` makes it pass. Not a defect.
- **Not claimed**: no browser re-drive, no wasm `component-release` this wave — the activation path after a
  guest restart still wants one in-browser confirmation.

## W-P4 — The paged scene-lane intake never stalled; both host budgets were priced off the delta (done, 2026-09-10)

Report: `📓️2026-09-10-wave-P4-lane-intake.md`.

- **Inherited claim, disproved.** The Perspective refresh dying with
  `plugin-ui.intake-budget-exhausted:1:puzzle3d-main-perspective:36864` was read as "the intake never
  progresses" (a lane-paging protocol deadlock, or a TS half that does not handle the lane page kind).
  A new law that drives a REAL Nakagin-scale paged lane patch — surface node, one `paged_text_carrier`
  per declared lane, 512-byte leaves, 32-ary pages, sized from the contract fixture's own
  `measuredFaultBytes` — through `OwnedUiPatchIntake` with the renderer's own grant shows the opposite:
  it reaches the acknowledgement, publishes revision 1, and the longest run of steps carrying neither an
  item nor a byte is **4** (the intake rejects at 32). No deadlock, no missing lane kind.
- **RC — publication cost is the DOCUMENT, the budgets priced the DELTA.** The measured publication needs
  **669 403** intake steps for 145 nodes / 57 294 carried bytes = 4 617 per node, because every patch —
  a one-op delta included — revalidates, rehashes, restages and renotifies the whole surface. The
  original `4096 + max(bytes, ops × 4096) × 8` credited a one-op delta 36 864; W-P2's 64 KiB floor
  credited 528 384 — still 21 % short, so the same fault would have come back with a bigger number.
- **Second defect, latent.** W-P2's 4096 same-phase guard in `acceptUiPatches` is a false positive: a
  legitimate Nakagin publication sits in the phase name `validation` for **163 284** consecutive steps.
  It would have turned the budget fault into `plugin-ui.intake-zero-progress:…:validation`.
- **Third defect, throughput.** `yieldUi` used `PLUGIN_UI_CONTINUATION_BATCH_SIZE` = 8 — the cadence for
  guest TURNS — for wire-phase steps of ~3.4 µs: 83 675 macrotask round-trips per refresh.
- **Fix.** `pluginUiIntakeBudget`/`measureRetainedUiPatchBytes` deleted; `pluginUiIntakeStepCeiling(limits)`
  = `limits.maxNodes × PLUGIN_UI_INTAKE_STEPS_PER_NODE (8192)` is a document-priced liveness BACKSTOP,
  with the progress guarantee left where it belongs (the intake's own byte-aware zero-progress reject).
  The same-phase guard is gone; `PLUGIN_UI_INTAKE_YIELD_STRIDE = 1024` (~3.5 ms slice) replaces the turn
  cadence in the intake loop.
- **Laws**: typedwire `OwnedIntake drives a Nakagin-scale paged scene-lane surface patch to
  acknowledgement inside the credited budget` (pins 669 403 ≤ the per-node price, `samePhase > 4096`,
  `idle < 32`, and the close within `PLUGIN_UI_CONTINUATION_LIMIT`); intake in-source
  `prices a surface patch by the retained document, never by the delta that triggered it`.
- **Verification**: `typecheck` **820** errors, **0** in any touched file. `test exhaustive --run
  UiDocumentStore --testNamePattern='Nakagin-scale paged scene-lane'` **1/1** (fails before the ceiling
  fix with `expected 669403 to be less than or equal to 528384`, and before the guard removal with
  `expected 4097 to be less than or equal to 4096`). `test long --run intake + Interpreter`
  **74/74**; `test long --run PluginRuntime + engine-contract` **551/551**.
  `cargo test -p semio-framework-ui-scene` **116 passed / 0 failed** (all seven world-3d lane laws).
- **Instrument note**: BSD `grep` treats `🔌️PluginRuntime/🟦️.tsx` as binary — every repo grep in this
  ticket needs `-a` or it silently reports zero matches. W-P3's "PluginRuntime is missing from the
  `pluginUiIntakeBudget` list" was this artefact, not a missing edit.
- **Not claimed**: no browser re-drive and no `component-release` this wave; the fix is TS-only and rides
  HMR / the next serve. The `did not publish its requested UI surfaces within 4096 continuations` drain
  message was a consequence of the intake throwing, and is expected to clear with it — unconfirmed.

## W-R — The intake now costs the delta: validation is 163 284 → 159 steps on a lane re-publish (done, 2026-09-10)

Report: `📓️2026-09-10-wave-R-intake-delta-cost.md`.

- **W-P4's attribution was wrong where it matters.** It priced the whole 669 403-step publication as
  "revalidate + rehash + restage + renotify the whole document". Split by phase, three of those four are
  not costs at all: `hash` is **2 030** steps (0.3 %, 14/node — the canonical-JSON streamer consumes atoms
  until the 65 536-byte grant is spent), `staging` **1** and `notifications` **5**. Roughly **450 000**
  steps (67 %) are the wire decode of the 145 upserted nodes and **54 624** are `scenes` — both already
  priced by the delta. The one document-scaled cost is `validation`: **163 284**.
- **RC, one level down.** Attributing every step inside `validation` to its sub-operation shows **131 229
  of 163 284 (80 %) are the three `RetainedUiNumericTable.set` calls the DFS makes per node** (enter 49 502,
  on-path 41 176, exit 40 551). A `Table.set` is ~300 steps — `NumericIndex` rebuilds the `ids` AND `order`
  AVL trees through `TreeEdit`'s reservation dance and then drains two retirements — against 10–16 steps
  for a `lookup` on the same index. The visit *marks*, pure patch-local scratch, cost 20× the node reads.
- **Fix — a delta-priced validation program, exact, no fallback hack.** `OwnedUiSurface` only ever
  publishes states that passed validation, so every patch's source is a proven-valid graph. Every
  invariant but `nodeQuota` and `nonFiniteNumber` is decided by structure: node set, child edges, root,
  sibling `key`, section role. So when every operation replaced an EXISTING record keeping its `key`, its
  `children` and its section role, and nothing was removed and the root did not move, reachability, depth,
  cycles, nested sections, orphan children and duplicate sibling keys are already proven and only
  `nonFiniteNumber` on a replaced record can still break.
  - `🩹️operations/🟦️.ts`: `OwnedUiOperationResult.shapePreserving`, certified per operation by a metered
    `sameGraphShape` generator (`upsert` now looks the id up in the source first: +14 steps/op).
  - `🛡️validation/🔬️graph/🟦️.ts`: `retainedUiGraphTouchedValidation` — `nodeQuota` + one lookup and one
    `finite()` per touched id. No marks table, no sibling keys, no dangling sweep, no index writes.
  - `🛡️validation/🟦️.ts`: `OwnedUiValidationCursor(source, root, limits, touched?)`; `null` keeps the
    authoritative whole-graph walk, which stays the ONLY program that mints the exact depth-first
    violation ORDER, so every rejection path and conformance fixture is unchanged.
  - `🖼️surface/🟦️.ts`: `OwnedUiSurfacePatch` accumulates `#shapePreserving` and passes `#touched`.
- **Measured** (145 nodes, 18 lanes, 57 294 carried bytes, grant `{256, 65 536}`): first publication
  669 403 → **671 321** (+0.3 %, the added upsert lookups; still `validation` 163 284 because it re-roots);
  one-lane re-publish of 8 records **28 066** steps with `validation` **159** — from ~194 000 / 29 % of a
  full publication down to **4.18 %**.
- **No wire or Rust change.** `shapePreserving` is derived host-side from records the host already holds;
  the published `hash` is still FNV-1a over the whole canonical document, so the guest's ACK token and
  `🛡️limits.rs`'s `validate_core` are untouched. **The served wasm needs no rebuild for this wave.**
- **Law**: W-P4's typedwire law restructured around a `drive(ops, base, revision)` helper and extended to
  a SECOND publication — a one-lane re-publish at the same key/children/role — asserting revision 2, a
  changed published `hash`, `validation < 40 × touched`, and `steps × 10 < firstSteps`. The first
  publication keeps every W-P4 assertion plus `first.validation > 10 × nodes.length`, which pins the
  whole-graph walk as *reached* rather than merely available.
- **Verification** (all four re-run after the temporary `[DEBUG]` histogram logs were removed):
  `test exhaustive --run UiDocumentStore --testNamePattern='Nakagin-scale'` **1/1**;
  `test long --run intake + Interpreter` **74/74**; `test long --run PluginRuntime + engine-contract`
  **551/551**; `typecheck` **820** errors (byte-identical to W-P4's baseline), **0** in any touched file.
  Whole-file `test exhaustive --run UiDocumentStore` (367 s) **212 passed / 1 failed** — every
  `OwnedOperation`/`OwnedValidation`/`OwnedSurface`/`OwnedIntake` law green; the single failure
  (`OwnedHash streams exact insertion-ordered JSON bytes…`) fails identically ALONE and its import graph
  (`📦️wire/🧾️typed`, `🗂️nodes`, `🔢️hash`, `🔏️owned-hash` fixture, `encodePackValue`) contains no file
  this wave edited. Not W-R's; left to its lane.
  Note: `UiDocumentStore/🟦️.tsx` is not in the `long` include set — `exhaustive` is the only level that
  reaches it.
- **Next lever, not this wave**: the residual 28 066 is 74 % wire decode, and `symbol-edit` (12 904, 1 613
  per record) is the SAME ~300-step persistent-index write, in the pack decoder's symbol table. Staging
  stays O(subscriber cells) at ~14 steps/cell (≈ 2 000 on a 145-cell surface) — a node→cell reverse index
  would remove it, and the measurement does not yet justify one.
- **Not claimed**: no browser re-drive and no `component-release`; the change is TypeScript-only and rides
  HMR / the next serve.

## W-F4 — the fill tool actually plans now (2026-09-10)

Continuation of W-F/W-F2/W-F3. Full record: `📓️2026-09-09-wave-F-fill-oom.md` §7.

- **Two defects, both measured, both fixed in `…/✏️editor/⏳️precompute/🦀️.rs`.**
  1. **The admission census advanced one unit per `fillBuildTick`.** An envelope is only handed to its
     bounded job once `FillBuilderOwnerCensusCursor` returns `Complete`; that walk is 97 units on the
     `app()` fixture and thousands on a real document, so at one unit per 120 ms tick the plan was never
     admitted, **no `Effect::SpawnJob` was ever requested**, and every superseding edit re-admitted a
     fresh envelope on top (320 ticks → 50 admissions, 0 jobs). Now `enqueue_fill_job` spends
     `FILL_ENVELOPE_CENSUS_UNITS_PER_TURN = 4_096` census units per turn — the same per-turn unit budget
     `precompute_step_lane` already runs on the brush lane.
  2. **The fill cancel token lived on the per-call collision engine.** `begin_measurement` clones
     `engine.fill_cancel` into the envelope, but that token is rebuilt fresh by `with_puzzle3d_app_for`
     on every dispatch and `Drop for Puzzle3dPrecomputeSession` cancelled it *before* its `involved`
     early-return — so the very dispatch that admitted a plan cancelled it on the way out,
     `drive_fill_envelope` hit `is_cancelled_now()` on its first slice, and the job "completed" having
     placed nothing (`ready: 0`, `registry_available=0`). It also meant every later
     `supersede_admitted_fill` / `cancel_fill_job_for` cancelled a token nobody listened to. The token
     now travels on `Puzzle3dFillSession` (`take_fill_session` swaps it out, `install_fill_session`
     restores it), and the drop-time cancel happens only after the `involved` check.
- **Law**: `fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances` — 320 host ticks
  through the REAL `reactor::jobs` runtime (`start_job` on the requested effect, then bounded
  `step_job` slices), asserting admissions ≤ 8, registry occupancy ≤ its 4 slots, and slider
  `ready > 0`. **Before: FAILED** (`spawned 0 jobs`, then after fix 1 only `ready: 0`).
  **After: `ok`.** `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job` restated: the
  bounded sync prologue means the spawning tick is not necessarily the first, so it now asserts exactly
  ONE isolated `FILL_JOB_KIND` spawn across 16 ticks and reads the plan through `fill_ready` (the
  slider's own path) instead of through a session-less `with_puzzle3d_app` probe that answers off the
  process-wide registry. Both pass together, `--test-threads=1`.
- **Test isolation**: four process-wide envelope slots. A 320-tick law that walked away from its live
  plan starved every later fill law of an admission, so `drain_fill_envelope_registry_for_test`
  (test-only) drains the registry at both ends of the growth law.
- **Stack (deliverable c) — partial, and the brief's premise was wrong.** Painted peaks, debug profile:
  `create_puzzle3d_app` 404 KiB → **233 KiB** after boxing `EditorBuilder`/`ViewerBuilder`'s `inner`
  (`🧰️framework/…/🔌️plugin/🦀️.rs`: ~250 fluent chain steps each got their own stack slot for a whole
  inline `AppBuilder`; a framework-wide win). `app()` 1 292 KiB, `app()` + one `dispatch("fillBuildTick")`
  **2 299 KiB**, the growth law's body 2 407 KiB. The measures path costs 96 KiB and futures are
  irrelevant (`app()`'s future is 99 680 bytes): the remaining owner is the framework `settle` /
  continuation chain, ~1.9–2.3 MiB deep on its own. Whole-app laws in this crate still need
  `RUST_MIN_STACK` above 2 MiB.
- **Not mine, flagged**: W-F2c's `take_terminal_fill_job` retention (a `Terminal(Complete)` envelope is
  no longer discarded) makes `bounded_fill_job_reaches_done_publishing_its_envelope_token_as_progress_and_checkpoint`
  hang on a slot that never empties and leaks a completed plan into any later law that reads
  `read_fill`'s registry fallback. `fill_build_tick_only_plans_available_slider_range` still ends at
  `ready: 0` when driven by `drive_fill_until_ready` even though the growth law reaches `ready > 0` —
  same retention/close boundary. The `fill_worker_*` family (12 laws) fails identically before and
  after this wave.

## 10:50 — Evidence for W-P5 (coordinator, browser on #28, clean profile)

- Switching Concrete Forest → Nakagin: `setActiveExample` 556 ms **scope `none`**, next `refreshUi` ok, inspection `Objects 180`, world EMPTY (grid only).
- Switching back Nakagin → Concrete Forest: `setActiveExample` 533 ms scope `none`, `refreshUi` 491 ms ok, `Objects 1`, world STILL EMPTY — the table that rendered at boot does not come back. So the scene surface is not republished on an example switch at all; the boot publication is the only one the host ever renders. `setActiveExample` must dirty the world (viewport/full scope) and republish the scene lanes.

## W-F5 — the host's fill-job lifetime cap, and the cancel that reported a failure (2026-09-10)

Report: `📓️2026-09-10-wave-F5-fill-budget-cancel.md`.

- **`PLUGIN_JOB_STEP_LIMIT` is deleted.** `🔌️PluginRuntime/🟦️.tsx`'s isolated-job driver bounded a
  bounded job's LIFETIME at `1 << 16` steps and cancelled whatever was still running there. The native
  host does not: `ShardLoop::pump` grants one `job_budget_from_grant` PER SLICE, walks `running_jobs`
  with no step counter at all, and ends a job only on the guest's own `JobStep::Done`/`Failed`, actor
  loss, or an explicit `Effect::CancelJob`. A `semio.puzzle3d.fill` plan (up to
  `PUZZLE3D_FILL_COUNT_MAX = 1000` placements over a document census) reached 65 536 in ~40 s of
  ticking on build #28, so the cap sat inside the feature's working range. `driveSpawnedJob` now loops
  `while (live())` on the per-slice `jobStepBudget` alone.
- **A deliberate cancel was reported to the user as a failure.** `jobs::cancel_job` calls
  `BoundedJob::cancel` and then DROPS the owner, so no later `step` reaches `drive_fill_envelope` to
  observe the token `Puzzle3dFillBoundedJob::cancel` tripped. The drop released a still-armed
  `FillEnvelopeWorkerFaultGuard`, which requests `FillEnvelopeTerminalReason::Fault`; the intent
  register is `fetch_max` over `Complete=1 < Cancelled=2 < Fault=3 < Closed=4`, so Fault won and every
  cancel raised the `fill_failed` notice — *"Fill planning stopped — the background job failed"*.
  `cancel()` now terminalizes `Cancelled` itself and disarms the guard, in both the `Admitting` and
  `Driving` stages.
- **`Cancel fill` outlived the run it named.** `fill_job_identity` answered `Some` until the envelope's
  close cursor gave the slot back — one retained owner per tick, ~100 ticks (≈12 s) on `app()`. It now
  answers only for a run this session can still advance (no checked-out terminal, authority phase
  `Measuring`/`Admitted`, via `FillEnvelopeRegistry::is_live`); `cancel_fill_job_for`'s guard rides the
  same predicate.
- **The per-tick browser console flood is gone**: the `[DEBUG] fill_build_tick …` `eprintln!` in
  `🎮️commands/🪣️fill-build-tick/🦀️.rs` (W-U3's, ~7 000 lines/min) removed, along with the driver's own
  `[DEBUG] job done` / cap lines.
- **Laws.** Native (`🪣️FillJobLifetime` region): a registered `BoundedJob` driven 70 000 slices under an
  UNCHANGED per-slice budget reaches `Done` on exactly slice 70 000 — length is not a fault and a
  constant budget is not a stall; plus both cancel orders end to end (user `cancelFillBuild` →
  `Effect::CancelJob` → `jobs::cancel_job`, and host `jobs::cancel_job` alone) asserting no
  `fill_failed`, the affordance stops naming the cancelled run, and the slot comes back. Vitest
  (`🧪️tests/🔌️plugin-runtime/🟦️.tsx`): the driver steps all 66 560 slices, never calls `cancelJob`, and
  passes ONE distinct budget every slice — **fails before the fix with `expected 65536 to be 66560`**.
- **Order-dependence repaired**: `fill_build_tick_is_ignored_when_fill_tool_is_inactive` took no fill
  registry guard while `fill_progress_summary` falls back to the process-wide registry, so it read a
  concurrent law's plan as its own progression (W-F4 §7.6 (2)'s leak). It now takes
  `fill_envelope_test_guard()`.
- **Open**: the browser `unreachable` itself is NOT localised — neither cancel order panics natively.
  What is established is that the host had no business issuing that cancel, and that the path it used
  reported a user-visible failure. A recurrence after a wasm rebuild needs a guest backtrace.
- **Test isolation, flagged not fixed**: `fill_envelope_registry()` is four process-wide slots guarded
  in tests by ONE mutex plus a drain on drop — serialization, not isolation. This wave's three extra
  admitting laws share those slots, and W-F4's
  `fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances` now passes alone
  (`--test-threads=1`, **ok, 14.93 s**) while failing in a `--test-threads=2` family run with
  `puzzle3d.fill-job.stale` — a superseded envelope, not a broken plan; the same binary passes both
  ways, so the production path is not what moved. Run single-threaded the whole fill family is green
  including this wave's three laws — **61 passed / 0 failed** (62 s), against W-F4's 59/0 plus the two
  new fill laws. Whole lib on a quiet box: **640 passed / 4 failed** (the other three: two foreign wave
  laws plus the standing `module.vcs` one). The fill family needs a per-law registry.
- **The served wasm must be rebuilt** — the guest changed. The host half (`PluginRuntime`) is TypeScript
  and reaches the browser on a reload.

## W-P5 — the Nakagin lane set DOES become instances; the world was measured before it arrived (2026-09-10)

Report: `📓️2026-09-10-wave-P5-lanes-render.md`.

- **The reported defect does not reproduce on the tree as it stands.** Three read-only headless probes
  against the already-running React dev target at `:6013` (repo `node_modules/playwright`, a separate
  browser process; the coordinator's tab was not touched, no server started) switch Concrete Forest →
  Nakagin and read the scene `World3dHost` actually consumes off its own root element
  (`data-instances-json` / `data-meshes-json`, `🌐️World3dHost/🟦️.tsx:5422`). It goes 266 B / 1 instance
  → **54 254 B / 180 instances**, 14 mesh records, **zero unresolved `meshId`s**, bounds
  x[-23.45, 0] y[-12.55, 0] z[0, 39.58] — and the end-of-probe screenshot
  (`…/scratchpad/wp5-nakagin3.png`) shows the **Capsule Tower fully drawn in both windows**.
- **What the 10:50 evidence actually caught is latency.** The scene lands **15–35 s after the click** on
  every probe run, while `setActiveExample` (556 ms) and the following `refreshUi` (643 ms) both report
  ok inside the first 1.2 s and the inspection panel — a reserved SECTION of a few hundred bytes —
  repaints `Objects 180` almost immediately. The world body is 57 KB across 11 carriers, the largest
  publication in the app, and it is simply still being taken in when the panels are already correct.
- **`scope none` is not the cause.** `puzzle3d_command_scope_class("setActiveExample")` →
  `Puzzle3dScopeClass::Chrome` → `UiDirtyScope::Full` (`✏️editor/🦀️.rs:2185,2204`), and W-X's
  `set_active_example_lands_as_one_edit_and_republishes_the_world_scene` already asserts that emit
  scope. The browser's `scope none` line is reporting something other than the job's Complete emit; the
  republish demonstrably happens.
- **Every other candidate is ruled out with a measurement**, in the report's §4 table: no lane exceeds a
  page count (10 lanes 1 leaf, `instances` 4, all depth 1), no carrier key drifts, no hash rejects,
  `World3dHost` reads the assembled scene, and a packed 33-slice leaf crosses the wire intact
  (`encodePackValue` → `RetainedUiTypedCursor` → `value` 512 B + 32 attributes).
- **The coverage that was missing — three new laws, all green, each with a measured negative run.**
  Nakagin is the first document that PACKS a text leaf (`section_text_chunks`: slice 0 in
  `TextProps.value`, slices 1…32 in `data_attributes`, 16 896 B per leaf) and no suite crossed a packed
  leaf on the production path.
  1. `🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes` — *projects a Nakagin-scale packed window body
     through `builtNodeToSnapshot` into 180 assembled instances*: the real `ShellHost` projection into a
     real `UiDocumentStore`, then the Interpreter's own lane walk; asserts 180 records and that **no
     record names a mesh absent from the same publication**. Fails with `packedTextLeaf`'s attribute
     loop disabled (`JSON Parse error: Unexpected EOF` — the 512-byte-prefix signature).
  2. `📃️UiDocumentStore/🧪️tests/🧪️typedwire` — *`OwnedIntake` publishes a packed scene-lane leaf whose
     data-attribute slices all survive the wire*: 32 147 B packed the way the guest packs it, driven
     through a real `ShardClient` UI-patch authority and read back the way `projectOwnedUiSurface` does.
     Fails with the typed cursor's `dataAttributes` decode nulled.
  3. `✏️editor/🧪️tests/🔬️example-switch` — *the Nakagin switch assembles every object onto a mesh the
     same publication declares*: `[DEBUG] Nakagin switch assembled 180 instances over 14 declared meshes
     (0 unresolved) from a 55154-byte instances lane in 4 leaves`. A dangling `meshId` is the one shape
     that passes every byte-level lane law and still paints an empty world.
- **Gates.** puzzle-3d `example_switch` **5 passed / 0 failed**; puzzle-3d `nakagin` family **24 passed /
  0 failed**; `semio-framework-ui-scene` **116 passed / 0 failed**; react `test long --run 🗣️Interpreter
  📥️intake 🔬️engine-contract` **3 files, 569 passed / 0 failed**; `typecheck` **824 `error TS`, zero in
  any file this wave touched** (the peer baseline moved 820 → 824 during the wave).
- **The served wasm does NOT need a rebuild for this.** The probes ran against the module already served
  (2026-09-09 19:21) and it publishes the packed lanes correctly; the only Rust this wave edited is a
  test module.
- **Open**: the 15–35 s repaint latency is unattributed and is the real residual — a performance wave.
  Also flagged: `scene.fitJson` is never published by the world window, so a fixture swap never refits
  the camera; Nakagin happens to sit inside Concrete Forest's framing.
- **Not closed: the SWITCH BACK.** The 10:50 note also reports Nakagin → Concrete Forest leaving the
  world empty at `Objects 1`. The round-trip probe was written
  (`🔍️wp5-world-lane-probe.ts --round-trip`, kept in the ticket folder) but from ~10:36 the dev target
  at `:6013` stopped answering — the socket still LISTENs (`bun` pid 44038) while both `curl` and
  Playwright's `goto` time out — so two runs were lost. Re-run it against a responsive target.

## 11:15 — Evidence for W-F6 (coordinator, console capture on #29)

- The `unreachable` that follows the `plugin.command-page-allocation` refusals is a guest panic inside the bounded fill job step (`[handler/stepJob]`): `panicked at …/✏️editor/⏳️precompute/📐️geometry/🦀️.rs:263:39: live fixed owner page`. So the command-page authority and the geometry owner pages share (or starve) the same fixed pool: once the 4 Hz tick stream pins the pages, the job step `expect`s a live owner page and aborts the whole guest.
- Boot is currently blocked by an in-flight TS edit: `The requested module … does not provide an export named 'pluginUiIntakeStepCeiling'` (W-S2 area) — the coordinator's browser checks resume when W-S2 lands.

## W-S2 — the 15–35 s between the example click and the world repaint, attributed and fixed

Report: `📓️2026-09-10-wave-S2-scene-latency.md`. Probe: `🔍️ws2-scene-latency-probe.ts` (kept).

- **Attribution, measured, not argued.** The browser probe (a separate headless chromium against the
  live release target; no server started, no interactive tab touched) instruments the gap with a
  `longtask` observer, a `Worker.prototype.postMessage` counter and console capture:
  `gap=24.3s longtaskCpu=5.4s tasks=21 workerPosts=8799`, with the runtime's own
  `[DEBUG] settle puzzle#1 continuation 512/1024/1536 status=more-work acks=0 drain=true` inside it.
  So 78 % of the gap is host↔guest turn ROUND TRIPS (~4 400 of them), not work: the guest keeps
  answering `MoreWork` while publishing nothing. The same shape the 2026-09-09 20:35 live trace
  recorded (`more-work streak=8192 … reconcile=true`, everything else false).
- **The terms.** Guest `split_lanes` + `scene_surface` for the 180-object document: **557 µs**
  (native debug). Guest render of the whole composite body: 99–378 ms. Host intake of the REAL
  (packed, 26-node) lane set: **105 416 steps ≈ 0.36 s** — six times cheaper than W-P4/W-R's
  synthetic 145-node UNPACKED figure of 669 403. Reconcile of the published surface: **2 255 steps =
  3 reactor turns**. Retirement of the very same patch: **1 092 units at 8 units per turn = 137 turns
  = 137 round trips**, per window — and the pending authority and the kernel's turn-patch arena drove
  ONE unit per turn each.
- **The defect.** Every stage of a reactor turn is priced per PAGE (`SURFACE_RECONCILE_PAGE_BYTES`,
  256 items / 64 KiB, 1 024 reconcile opportunities, the turn's own deadline). Retirement was the one
  stage priced per ITEM per TURN — and a turn is a host round trip, because an unretired acknowledged
  publication keeps `PendingPatchAuthority::has_unpublished` (and therefore the whole turn) in
  `MoreWork`.
- **The fix.** Retirement is now bounded work per TURN: `PATCH_CLOSE_UNITS_PER_TURN` 8 → **256**, each
  unit granted `PATCH_RETIREMENT_ITEMS_PER_UNIT` = 1 024 items / `SURFACE_RECONCILE_PAGE_BYTES`, every
  run cut short by the turn's own `budget.deadline_ms` (`retire_until_complete` /
  `retire_while_progress`, clock re-read every 8 units). **Measured: 137 turns → 5** for the same
  publication — 27× fewer round trips, pinned by a law that reads the production constants. The grant is threaded through the four owners that ignored it:
  `close_surface_patch_owner` (ui-runtime), `PendingPatchAuthority::close_step`,
  `PatchTracker::close_step`/`ReadySlot::close_step`, and the kernel's own
  `close_ui_turn_patch_owner_with_grant` — the arena the emitted patch actually lands in.
- **The camera.** `scene.fitJson` was never published by this editor, so a document swap never refit
  the camera (W-P5 §7). New `world3d_fit_json(revision, padding)` in the ui wgpu component module and
  `main::world_fit_revision(fixture)` — FNV over the fixture's IDENTITY (schema, domain, kind
  catalogs), never its geometry, so a document swap refits once and an object edit never moves the
  camera.
- **Gates.** puzzle-3d `example_switch` **6 passed / 0 failed** (includes the new fit law:
  `fit lane revision forest=2383401358 nakagin=1178560245 after-edit=1178560245`); framework-plugin
  `a_nakagin_scale_world_publication…` **1 passed** (`3 turns` reconcile, `5 turns / 1092 units`
  retirement, `137 turns` at the old pacing) and `a_settled_reactor_turn_retains_nothing…`
  **1 passed** at `per_turn=0 B`; react `test long --run 🗣️Interpreter 📥️intake 🔌️PluginRuntime
  🔬️engine-contract` **4 files, 675 passed / 0 failed**; the new TS lane-set law **1 passed**
  (1.86 steps/byte) with both negatives measured; `typecheck` **826 error TS, zero in any file this
  wave touched**. The whole framework-plugin `--lib` run is peer-blocked (54 failures from other
  lanes' in-flight app-definition/classification work, none in this wave's files).
- **The served wasm MUST be rebuilt** for any of this to show in the browser: unlike W-P4, W-R and
  W-P5, this wave's fix is guest Rust (reactor + kernel + the puzzle 3d window). The before-figure is
  24.3 s / 8 799 worker messages; re-run `🔍️ws2-scene-latency-probe.ts` after the rebuild for the
  after-figure.

## W-F6 — the command-page authority's quarter-megabyte per command, and the owner page that trapped instead of refusing

Report: `📓️2026-09-10-wave-F6-command-pages.md`.

- **Who reserves the 64 slots.** One production site: `CommandPageSet::try_new`
  (`📡️spr/🧵️channel/🦀️.rs`), called from exactly one guest place — the reactor's command-ingress
  prologue (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`, the `page_index == 0 && retained.is_none()` arm),
  i.e. ONCE PER COMMAND. `FixedCommandPage` is 4 098 B, so the reservation was
  **64 × 4 098 = 262 272 contiguous bytes for every command**, including a one-page `fillBuildTick`
  of a few hundred bytes, four times a second.
- **Who releases them, and why the fault is not a leak.** The set is transient: pages pop one per
  `close_step`, the deque MOVES into `PagedCommand` → `PagedCommandReader` → the decode cursor and
  dies with the command, and the retained authority is a fixed TWO-slot array. The 320-command
  native law measures **peak retained ingress occupancy 0** and **68 B retained per command**.
  Saturation has its own code (`plugin.command-page-count`) and never fired on #29 — what fired was
  `try_reserve_exact` refusing, i.e. the guest's 512 MiB linear memory could no longer produce a
  262 KiB contiguous block.
- **The `dlmalloc` theory that was checked and rejected.** There is no mmap threshold in
  `dlmalloc-rs`; a freed large chunk IS recycled. What is true: its wasm backend returns `false`
  from `free`/`free_part`/`can_release_part`, so the guest's memory **grows and never shrinks**, and
  its `granularity` is 64 KiB. A quarter-megabyte contiguous request is therefore the FIRST request a
  fragmented, nearly-full guest refuses — exactly the order #29 showed.
- **The trap, localised and fixed.** The coordinator's capture named it:
  `📐️geometry/🦀️.rs:263: live fixed owner page`. `FixedOwnerVec`/`FixedOwnerMap`/`FixedOwnerSet`
  answer a failed `try_reserve_exact` with a page-less owner — but then reported their FULL declared
  capacity and `expect`ed the page on the first push, and every collision-mutation preflight compared
  against that same lie. Now `capacity()` answers 0 without a page, `try_push`/`try_insert` refuse and
  hand the value back, and the one owner the preflight cannot weigh (a brand-new cell member bucket)
  is checked at the site. An exhausted guest answers `Rejected(Capacity(..))` instead of aborting.
- **The fix at the cause.** `CommandPageSet::try_new(declared)` reserves exactly the pages the command
  declares (validated `1..=64` by the ingress cursor, identical across a command's pages), so a
  one-page command now reserves **4 098 B instead of 262 272 B**. New framework budget
  `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES = 65 536` (one guest growth unit), schema-pinned as
  `contiguousRequestCeilingBytes`.
- **Laws.** `📄️command-page-authority/🦀️.rs` (3): exact declared reservation under the ceiling; a
  saturated authority refuses on every path, hands the page back and survives the fault wire without
  panicking; 320 real commands through `poll_kernel` leave the retained ingress authority empty every
  time. Plus `an_owner_whose_page_was_refused_refuses_every_insert_instead_of_trapping` in the puzzle3d
  geometry suite and the trace memory ceiling law.
- **Verified**: plugin laws **3 passed / 0 failed**; `semio-framework-trace` **31/0**;
  `semio-framework-os-kernel` channel **29/0**; `semio-framework` kernel page/batch **10/0**;
  puzzle3d refused-owner law **1/0** and the whole geometry family **45/0**; `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle`
  **EXIT 0 (4m 04s)**.
- **The served wasm MUST be rebuilt** — both halves are guest Rust.
- **Still open**: `plugin_exchange`'s 76 848 B box per command and `wit_bindgen`'s ~189 KB turn-task
  box per poll are the other two over-ceiling contiguous requests on the same guest path (a peer wave
  is on the latter); `CommandEnvelopeSet::try_new` still reserves 64+64 slots on the hosts; and the
  Rust `CommandBatchDriver` rejects the guest's first answer to a one-page generic command with
  `plugin.command-cursor-mismatch` (TS host does not — latent, worth its own ticket).

## W-Z — `[DEBUG]` close-out sweep (2026-09-10)

Full write-up: `📓️2026-09-10-wave-Z-debug-sweep.md`.

- **The 00:00 inventory is not the list.** `📓️2026-09-10-debug-trace-inventory.md` was captured
  against `HEAD=ebbace9b`; HEAD is now `6ad7b0e7` and the repo auto-committed most of what it listed,
  so those traces read as unchanged HEAD context today, not as `+` lines. The sweep re-derived
  everything live with `git diff HEAD -- <path> | grep -a -n '^+.*\[DEBUG\]'` over the scoped areas.
- **Attribution, in order:** peer-owned (adjacent docstring names another ticket slug, or the trace
  text is quoted in another ticket's notes) → untouched; verbatim at HEAD
  (`git show HEAD:<file> | grep`) → untouched even where the working tree shows a `+` because a peer
  only wrapped it in a gate; otherwise ours → removed with its dead helpers.
- **Removed: 11 traces + 3 dead helpers.** Traces: the whole undo/redo diagnosis family the
  coordinator flagged at 11:35 — ShellHost `actor dispatch`, `undo route`, and the five
  `undo funnel*` warns (two of which had grown `if`-blocks that are now back to their one-line
  guards); plugin `🦀️.rs` `group history action=`, `history route action= tail_group_id=` and
  `history route action= benign-collapse` (its match arm restored to one line); PluginRuntime
  `spawn-job routed` (fired once per spawned job, i.e. every fill/suggestions tick). Helpers:
  `LocalInteractionQuery::debug_state` and `LiveLocalInteraction::debug_state` (both docstring-marked
  "temporary, ticket 26/09/02", both orphaned when this tree deleted the
  `typed-operation publication` trace that read them), and the reactor's `TURN_TRACE` thread-local
  with its `let _turn_seq = …` per-turn bump for a phase trace that no longer exists.
- **Kept, ours, deliberately (13 lines).** One-shot `[DEBUG]` summaries at the end of a passing law,
  restating exactly the values the surrounding assertions pin — puzzle 3d `🔬️example-switch` ×3 and
  `📌️panels/🗿️artifact/🔬️unit` ×1, plugin `🔬️plugin-runtime-plugin-builder-contract` ×5 and
  `🔬️reconcile-budget` ×1, Interpreter `🚚️surface-scene-lanes` ×3. Every hot-loop probe named for
  removal (`probe_sp`, `probe_future_sizes`, `probe_stack_sizes`, `probe_fill_build_*`,
  `[DEBUG] tick=` / `FINAL retained=`) was already gone, and puzzle 3d production code now carries
  **zero** `[DEBUG]` lines, including W-U3's `🪣️fill-build-tick` `eprintln!`.
- **Kept, not ours (28 in-scope `+` lines).** Nearly all of `26/09/09/PROCEDURAL-3D-END-TO-END`'s.
  **Two exceptions to the task's own removal list, both deliberate:** ShellHost's
  `applyHostEffects refresh` / `skipped refresh` / `completion apply` and the reactor's
  `more-work streak` pair are verbatim at HEAD — the working tree's `+` is only the peer wrapping
  them in `runtimeDiagnosticsEnabled()` / `runtime_diagnostics_enabled()`. That peer's
  `//#region 🩺️RuntimeDiagnostics` adopts them by name, they are OFF by default, and
  `🧪️tests/🔬️engine-contract/🟦️.ts` pins the arming key; deleting them would gut a live peer
  facility. `PatchTracker::debug_state`, the pending-patch `debug_state` and `LAST_MAINTENANCE_STAGE`
  stay for the same reason — at HEAD, and each still has a live reader.
- **Peer drift repaired to unblock the lane (not part of the sweep).** A peer changed
  `plugin_continue_typed_operations` to return `TypedOperationScan` instead of `bool` but had not
  followed the rename into `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`, so
  `cargo test -p semio-framework-plugin --lib` failed to compile for every wave (4 × E0600/E0308).
  Two call sites now bind `scan` and derive `let more = scan.runnable || scan.contended;`.
- **Verified:** `cargo check -p semio-framework-plugin` **0 errors** (1 pre-existing macro
  `dead_code` warning, and no `debug_state is never used` — the orphans are gone);
  `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` **0 errors / 87
  pre-existing warnings, 1m 44s**; react `typecheck` **826 errors, all pre-existing peer test files**
  (the three in `🏛️ShellHost`/`🔌️PluginRuntime` sit in peer symbols — `undeclaredActionDiagnostic`,
  `pasteActionWithRetainedFragment`, `FaultScope` — that this sweep never touched);
  `bun ./📜️script.ts test long --run '🔌️PluginRuntime' '🔬️engine-contract'` **2 files / 600 tests
  passed, 0 failed**; `cargo test -p semio-framework-plugin --lib` over the nine laws the sweep can
  reach **9 passed / 5 failed**, and all five reds are the peer-blocked class W-S2 already recorded —
  two `interactive-job.missing-factory` on the fixture's `compositeEdit` SETUP (before any undo), one
  `app-definition.invalid: app id testkit-txn must be a canonical surface id`, and two
  `🏪️store/🦀️.rs:17917` Drop-witness panics in a file peers moved by +374/−223 lines in this tree.
  Every law that actually exercises the removed code is green: all seven `local_interaction_live*` /
  `local_interaction_runtime_query_generation_*` (the two deleted `debug_state` helpers hung off
  exactly those types), `a_nakagin_scale_world_publication…` and
  `reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps` (the reactor turn
  the `TURN_TRACE` counter was removed from).
- **Residual count.** `git diff HEAD | grep -a -c '^+.*\[DEBUG\]'` → **420** tree-wide, but ~340 of
  those are ticket-note markdown quoting console output and the rest are out-of-scope peer files. In
  the scoped areas **41 `+` lines remain**: 13 deliberate test one-shots, 27 peer/at-HEAD, 1 comment.
  **Zero remain in this ticket's production code** at the moment of the sweep.
- **Landed during the sweep, deliberately left.** A wave WAS editing (contrary to the brief):
  `🌐️World3dHost/🟦️.tsx` lost its three original traces to someone else mid-sweep and then gained a
  different set at 12:16 — `[DEBUG] vortex marker hover` / `vortex marker click` /
  `world dispatch addBrushObject`, an ACTIVE, unfinished browser diagnosis of the vortex-hover and
  brush-placement defect. Killing a live probe mid-run would destroy the diagnosis, so they stay —
  **but they are ungated `console.log`s on the hover and click paths and must be swept the moment
  that wave closes.** Also new: W-F6's one-shot `[DEBUG] command page authority: … B/command` in
  `📄️command-page-authority/🦀️.rs`, which rule (b) admits (its law asserts the printed figure).
