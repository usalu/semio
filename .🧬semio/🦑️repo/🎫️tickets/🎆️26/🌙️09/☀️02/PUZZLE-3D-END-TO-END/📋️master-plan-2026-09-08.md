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
