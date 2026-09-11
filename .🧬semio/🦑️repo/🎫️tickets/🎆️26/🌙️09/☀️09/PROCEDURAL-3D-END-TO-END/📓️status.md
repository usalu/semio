# Procedural 3d End To End — status

Coordinator: Fable 5.1 (session ⚪9f5f6952). Started 2026-09-09 ~14:30 at HEAD 9b605a4550.
Repo MCP timed out at session start; ticket bookkeeping is done on disk.

## Waves

- **W0 audits (Sonnet, read-only, parallel ×6)** — boot path, feature inventory, build chain, test harness, kernel+preview, peers+gates. Reports: `📓️*-audit-2026-09-09.md`, `📓️feature-inventory-2026-09-09.md`.
- **W1 build + boot (Opus)** — get the procedural plugin compiling natively + wasm32-wasip2 and the playground serving (react first, then wgpu wasm), with a private target dir.
- **W2 feature waves (Opus, parallel per lane)** — windows, examples + preview, hover/selection, editor actions, tests/oracles. Scoped after W0 reports land.
- **W3 performance** — profile the runtime, optimize architecture at the limits.
- **W4 gates + runtime verification + close.**

## Log

- 14:30 ticket opened on disk; W0 dispatched.
- 14:18 boot lane started: `serve-generation3d-react-dev` (Nx-inferred from `[[package.metadata.semio.playground]] variant="generation3d"`, react port 6018) under nohup with private `CARGO_TARGET_DIR=$S/target-boot` seeded by rsync from `target/wasm32-wasip2/wasm-dev` (5.6 GB, ~3 MB/s under load 80 / swap 45 of 46 GB). Log `$S/boot-react.log`.
- 14:20 W0 partial findings (editor + examples + mutations, from inventory sub-audits):
  - All 14 mutations real (diff/inverse/7-fn tests/5 fixtures each). All 27 editor actions `Migrated`, `factory_type` set. No todo!/FIXME.
  - **Dead continuations**: `✅️flow-eval-resolve` and `🔺️flow-tessellate-resolve` are implemented but not in `Generation3dCommand` nor retained tool ids → `Effect::InvokeExtension{brep, evaluate|tessellate}` results have no dispatch path back. Prime suspect for "no 3d preview".
  - `context_menu` hardcodes empty selection (`✏️editor/🦀️.rs:1217`). `world-pointer-down`/`graph-pointer-down` are no-op stubs. `👥️set-contributions/` dir is empty. Viewer has no transient hover/selection; viewer command enum is `Noop` only.
  - Examples: only `.dsl.semio` assets are real; `.op/.spr/.pack.semio` are empty envelopes; example tests only assert asset non-empty (no geometry assertions).
  - Brep ops run in plugin `semio-s-plugin-flow-extension-brep` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep`) over kernel `semio-framework-3d` (`🧰️framework/🔨️modules/🧊️3d`); generation3d itself has no kernel dep.
  - Assembly editor/viewer authored but not mounted (plugin root `🦀️.rs:91-98`).
- 14:21 boot lane restarted on the shared default target dir (rsync seed abandoned).
- 14:40 W0 landed: `📓️boot-path-audit-2026-09-09.md` (react via `serve-generation3d-react-dev`, 11-crate closure procedural+forms+9 flow-extensions, URL `?plugin=generation3d`), `📓️peers-and-gates-audit-2026-09-09.md` (launch.json generated from seed; gates: `verify dependencies literal-external`, `verify interactivity`, `verify taxonomy`, `verify rust-warnings`, `plugin-registry:generate/check`). Build-chain auditor lost its result (backgrounded monitor); its native `cargo check -p semio-s-plugin-procedural` keeps running in `$S/target-buildaudit`, log `$S/native-check-plugin.log`.
- 14:40 W1 dispatched (Opus ×3): boot-lane owner → `📓️wave1-boot-report-2026-09-09.md`; extension continuation wiring (flowEvalResolve/flowTessellateResolve) → `📓️extension-continuation-2026-09-09.md`; launch seed repair → `📓️launch-entries-2026-09-09.md`.
- 14:55 W0 complete except test-harness audit (pending): `📓️feature-inventory-2026-09-09.md` (IO codecs silently wrong for 7/9 formats; demo-session example unreachable; context menu empty selection), `📓️kernel-and-preview-audit-2026-09-09.md` (kernel = BREP subset of `semio-s-artifact-stdio-semio`; booleans have 15 failing tests hitting sphere-cut-with-torus + sphere-box-fuse; tessellation uncancellable; picking is client-side r3f raycast; no validate gate).
- 14:55 W2 dispatched (Opus ×3): boolean kernel fix → `📓️boolean-kernel-2026-09-09.md`; example geometry tests + oracle → `📓️example-geometry-tests-2026-09-09.md`; real IO codecs → `📓️io-codecs-2026-09-09.md`. Deferred until continuation agent lands (shared `✏️editor/🦀️.rs`): context menu selection, pointer-down stubs, perf/progress/cancel of tessellation.
- 14:53 W1 launch seed repaired → `📓️launch-entries-2026-09-09.md` (procedural 3d/2d dev rows env-fixed, hexagonal column rows rewired to VITE_SEMIO_DEFAULT_EXAMPLE, .claude/launch.json got procedural3d-react-attach + procedural3d-wgpu). Test-harness audit landed → `📓️test-harness-audit-2026-09-09.md` (TS examples 11/11 pass; python oracle lane blocked by unbuilt PyO3 host; no procedural3d E2E exists).
- 16:35 W1 continuation wiring landed → `📓️extension-continuation-2026-09-09.md`: **root cause of "no 3d preview" found** — (A) `Effect::InvokeExtension` with hand-minted `RequestId` never resolves (only registry-minted `req` via `Host::invoke_extension` is answered by `Event::Completed`), so every brep evaluate/tessellate result is discarded repo-wide (gen3d 104/105, gen2d 101, flow 106); (B) generation3d builds a throwaway `FlowEvalSession` per dispatch (no `ArtifactInstanceOperationOwner`). flowEvalResolve/flowTessellateResolve now first-class routes (29 rows), tests written but blocked from running by peer churn (`ArtifactCompositionFields`, `♻️retirement`, `set_sun`, `Generation3dViewCamera`).
- 16:35 boot lane attempt 1 failed at 15:29 on peer breakage (`semio-framework-os-kernel` OwnerRef derives — since repaired by peer at 15:13; plugin-host `ArtifactCompositionFields` bound churn); boot owner retrying.
- 16:36 W2b dispatched (Opus): SDK-level fix for (A) + retained owner for (B), migrating gen3d/gen2d/flow producers → `📓️extension-round-trip-2026-09-09.md`.
- 17:10 W2 IO codecs landed → `📓️io-codecs-2026-09-09.md`: real STL/OBJ/PLY/glTF/LAS/DWG export via stdio codecs, imports plant brep.io.import* neurons (LAS/PNG typed refusals), txt = full DSL round trip; 34/34 round-trip tests + python oracle lane green; gen3d crate checks with 0 errors/0 own warnings.
- 17:40 machine rebooted (uptime reset); all in-flight lanes died, scratchpad `/private/tmp/...` wiped (private target dirs gone). Reports on disk: boolean-kernel (through §4; remaining box∪/−cylinder, the two example booleans, offset, cone orientation, quality tags), example-geometry-tests (harness+fixtures done, §7 runs empty), viewer (complete: 7 Migrated viewer actions, config/presence/transient real, gates 0 errors), io-codecs (complete). Round-trip (Defect A/B) lane left partial edits, no report.
- 17:57 boot lane restarted (`$S/boot-react.sh`, shared target). 18:00 relaunched Opus lanes: boot owner → `📓️wave1-boot-report-2026-09-09.md`; round-trip resume → `📓️extension-round-trip-2026-09-09.md`; boolean resume → `📓️boolean-kernel-2026-09-09.md` §5; example-geometry resume → §7/§8 of its report. Private targets now seeded by APFS clone `cp -Rc target/debug`.
- 19:26 W2b round trip landed → `📓️extension-round-trip-2026-09-09.md`: `Emit::extension_invocations` + SDK-minted req + response_action dispatch; retained FlowEvalSession owner in gen3d/gen2d; gen2d flowEvalTick Migrated; 8/8 SDK, 5/5 gen3d, 7/7 gen2d tests; native + wasm32-wasip2 checks clean. Owed: descriptor regen (needs served wasm).
- 19:28 W3 dispatched: editor gaps (context menu selection, pointer-down routes, config leaves) → `📓️editor-gaps-2026-09-09.md`; resumable/cancellable tessellation + pack mesh transfer + validate gate → `📓️tessellation-jobs-2026-09-09.md`; Sonnet runtime verification checklist → `📓️runtime-verification-checklist-2026-09-09.md`. Boot lane at flow-plugin materialize-dev, 0 errors so far.
- 19:46 boot lane SERVES 6018 (`📓️wave1-boot-report-2026-09-09.md`, fresh wasm). Runtime boot #1: shell chrome OK, plugin actor traps → `📓️runtime-verification-2026-09-09.md` (retirement panic in with_scratch_session; 'surface context exceeds its wire bound').
- 19:47 boolean kernel resumed lane done (`📓️boolean-kernel-2026-09-09.md` §5): both example booleans exact (χ=2, validate silent, volume vs oracle), boolean 14/0, engine 27/0, offset 12/1 (Minkowski bomb → blend rewrite). Dispatched: runtime-defects lane (retirement panic + surface wire bound) → `📓️runtime-defects-2026-09-09.md`; analytic fillet/chamfer blend lane → `📓️blend-kernel-2026-09-09.md`.
- 19:51 example geometry harness ran (`📓️example-geometry-tests-2026-09-09.md` §7): 2/8 pass (wire, box-shell); extrude trio blocked on sweep lateral-face flip (`➡️sweep/🧮️core/🦀️.rs:271`), fillet unstitched (blend lane), booleans failed in harness (re-check vs boolean fixes). TS twin 35/35, python+repo-test oracle 8/8 green (oracle 🔣️.json malformed packages array fixed; PyO3 diagnosis was wrong). Dispatched sweep-kernel lane → `📓️sweep-kernel-2026-09-09.md`.
- 20:05 runtime verification checklist ready (37 steps) → `📓️runtime-verification-checklist-2026-09-09.md`; to be executed after the runtime-defects lane + a procedural component rebuild.
- 20:51 editor gaps landed → `📓️editor-gaps-2026-09-09.md` (selection-aware context menu via context_menu_with_request_context; dead pointer-down routes deleted; 7 real config mutation leaves; native+wasm checks 0 errors). Follow-up dispatched: interactionSelect stack overflow → `📓️selection-overflow-2026-09-09.md`.
- 21:02 tessellation jobs landed → `📓️tessellation-jobs-2026-09-09.md`: step-budgeted resumable tessellation with progress measure + cancelPreviewEval action (en/de), pack record-body mesh wire (2.7-2.9× smaller, chunked), LOD reuse, validate() gate → typed diagnostic; 33 tests green, native+wasm checks 0 errors.
- 21:09 sweep kernel landed → `📓️sweep-kernel-2026-09-09.md`: extrude lateral orientation fixed at the root; example harness 7/8 pass (only box-fillet-preview blocked on the blend lane); extrude-orientation 12/12, example booleans 2/2, kernel suite 434/0; native+wasm 0 errors/0 warnings.
- 21:59 runtime defects landed → `📓️runtime-defects-2026-09-09.md` (FlowHost fixture-clone retirement via retire_cold/with_host; surface view-context bound now schema-derived 808 640 B instead of the 256 KiB public-action cap; 45 fail-closed Drop asserts guarded; gen3d lib suite runs: 127/168 of 295). Dispatched: rebuild+restage+describe lane → `📓️rebuild-2026-09-09.md`; unit-suite debt lane → `📓️unit-suite-2026-09-09.md`.
- 22:03 blend lane stalled mid-write (partial 1382-line `🎨️blend/🦀️.rs` on disk, compile state unknown); relaunched with bounded-edit rule. Rebuild lane may be blocked by it until it compiles.
- 22:40 selection overflow landed → `📓️selection-overflow-2026-09-09.md`: not a cycle — the reserved-route dispatcher's generator frame reserved 1.89 MB (19 separate awaits) + close_step 692 KB; collapsed to one job await (221 KB) and extracted close steps (470 KB); 2 MiB-thread guard test; all plugin crates 0 errors native+wasm.
- 22:51 blend kernel landed → `📓️blend-kernel-2026-09-09.md`: analytic cylinder/sphere/torus/cone blends, iso-line pcurves; blend 10/0, offset 13/0 (Minkowski in 37 s), engine 27/0, analytic-blend 7/0; **example-geometry 8/8**; fillet 1 ms, +tessellation 61 ms; native+wasm 0 errors/0 warnings.
- 23:21 unit-suite debt lane landed → `📓️unit-suite-2026-09-09.md` (3d 127/168 → 244/51; 2d abort → 179/46; fixture template retirement, stale fixtures regenerated, MutationDiff::apply leak fixed, viewer actions dispatchable, kernels installed in suite). **New blocker found**: node-graph surface embeds the whole operator catalogue (111 KB vs 32 KiB UI_FIXED_BYTES) → flow window cannot render live; `AppActionRegistry::from_definition` ignores top-level actions. Dispatched: catalogue-surface lane → `📓️catalogue-surface-2026-09-09.md`; 3d remaining failures → `📓️unit-suite-3d-2026-09-09.md`; 2d remaining + BatchOnly migration → `📓️unit-suite-2d-2026-09-09.md`. Rebuild lane still retrying (gen2d viewer churn).
- 00:06 (09-10) rebuild landed → `📓️rebuild-2026-09-09.md` (served wasm fresh, descriptor 44 editor actions/4 commands, viewer 26). Serve restarted from main session (`/serve-react.sh`; agent-started serves die with the agent). Boot #2: no trap, no wire-bound fault; remaining: flow window intake-budget (74 736 B, catalogue lane) and every typed operation failing the store's batched fold contract → dispatched `📓️fold-contract-2026-09-10.md` lane. Also owed: plugin-registry:check taxonomy phase red repo-wide (639 procedural entries).
- 00:21 taxonomy audit → `📓️taxonomy-violations-audit-2026-09-10.md` (640 procedural violations: 573 leaf-not-reachable-from-Cargo, 42 window children, 12 surface schemas, 3×4 assembly facets, 1 plugin-root commands; `verify taxonomy` abort has a pathExclusions remedy). Fix lane deferred until the 3d/2d suite lanes release the test trees.
- 00:34 (09-10) all four Opus lanes (catalogue surface, fold contract, 3d suite, 2d suite) killed by the session rate limit at ~00:20 (reset 00:30) mid-edit, no reports; relaunched as resume lanes at 00:35. Serve restarted.
- 01:46 (09-10) all four resumed lanes stalled at ~01:00-01:20 (stream watchdog, 600 s); their edits were auto-committed at 01:31 (6ad7b0e7bc). Relaunched all four with report-first + bounded-edit rules: catalogue surface, fold contract, 3d suite, 2d suite.
- 02:20 catalogue surface landed → `📓️catalogue-surface-2026-09-09.md`: procedural.play.main 111 031 → 15 447 B (862 B at 500 operators), catalogue panel paginated, AppActionRegistry top-level actions fixed; 3d suite 287/16, 2d 222/5; renderer TS 756 pass; native+wasm 0 errors. Owed: flow artifact's 13 BatchOnlyPendingRewrite actions; registry-less assert_two_instances_converge.
- 02:51 restage with catalogue fix done (activate-1, EXIT=0); serve restarted with SEMIO_VITE_HMR=0 (HMR reload loops from concurrent edits). Boot #3: both edit windows render chrome+bodies, flow scene 7 nodes/6 edges, eval stalls at first extension node (extrusion-axis computing), preview meshes []; flow canvases stuck 300×150; empty-action invocation fault. Dispatched: eval-continuation runtime lane → `📓️eval-continuation-runtime-2026-09-10.md` (will restage); renderer lane → `📓️renderer-fixes-2026-09-10.md`.
- 02:59 generation2d suite lane landed → `📓️unit-suite-2d-2026-09-09.md`: 179/46 → 228/2 (default features 169/0); six product defects fixed (retire_cold, port slot decode, nesting bounds, dictionary frame, move-widget slot, resumable importer); native+wasm 0 errors. Remaining 2 are framework: `advance_artifact_envelope_load` reports Pending decode as Fault + decode worker never pumped; fail-closed module.vcs remote merge.
- 03:04 wgpu audit → `📓️wgpu-path-audit-2026-09-10.md` (World3d snapshot bridge missing, catalogue fetch React-only, closure stale). Dispatched wgpu renderer lane → `📓️wgpu-renderer-2026-09-10.md`; envelope-load lane → `📓️envelope-load-2026-09-10.md`.
- 03:34 fold-contract lane landed → `📓️fold-contract-2026-09-10.md` (one-item footprint counted rows: invertible mutation costs 2, declared 1 → every durable gesture fail-closed; shared footprint for 29 routes; 13 laws). Needs restage. Boot #4: renderer sizing live; actions fail with `coerceWireBytes: unsupported payload {tag:none}` — `commandIngressFaultDisplay` crashed on an option-none fault envelope (patched in PluginRuntime); underlying failure = fold contract (Rust, pending restage). Node-graph wasm engine attach still not happening (2D contexts, 0 px).
- 04:07 eval-continuation lane landed → `📓️eval-continuation-runtime-2026-09-10.md` (flow registry registered extensions under manifest id math/brep instead of the owning plugin id → extension.missing forever); restaged 04:02 (includes fold-contract fix). Serve restarted for boot #5.
- 04:12 boot #5 (04:02 restage + node-graph attach fix + option-none unwrap in wireEffectToFriendly paramPack/packField): no action decode failures; node graph attaches and draws (dag draw lod=normal/detail); typed operations now fail with 'retained command exceeds semantic work capacity' (interactionSelect; example load likely too); eval still stuck at extrusion-axis.
- 04:14 node-graph attach lane landed → `📓️node-graph-attach-2026-09-10.md` (attach required a WebGPU device it never used; hidden-tab size collapse); flow canvases now paint. Dispatched: work-capacity lane (retained command Preflight extent vs 2-row footprint) → `📓️work-capacity-2026-09-10.md` (will restage); Sonnet runtime hot-path audit → `📓️runtime-hotpath-audit-2026-09-10.md`.
- 04:26 hot-path audit → `📓️runtime-hotpath-audit-2026-09-10.md`; dispatched hot-path optimization lane (remove stale probes, gate maintenance logs, hash-gated eval publication as pack, tessellation cache across renders, no eval in render) → `📓️hotpath-optimization-2026-09-10.md`.
- 04:32 generation3d suite lane landed → `📓️unit-suite-3d-2026-09-09.md` (244/51 → 301/10 on its tree; fixed FlowDiff retire abort on live graph mutations, spr load projection drop, mesh pack chunk 48 KiB vs 8 KiB wire bound that stalled every tessellation). Handovers → dispatched FlowHost ownership + cursor law + flow-operators convergence lane → `📓️flow-host-ownership-2026-09-10.md`.
- 05:10 envelope-load lane landed → `📓️envelope-load-2026-09-10.md` (short-circuited if → replacement poll returned Fault on turn 1; decode never pumped by the reactor; owner abandoned on ?; close_step dropping live history; nested authorities inheriting the parent terminal flag). Downstream: gen3d `mutation-ingress-malformed @ $.edits` (FlowHost/cursor lane), gen2d publication lease contention.
- 05:16 wgpu renderer lane landed → `📓️wgpu-renderer-2026-09-10.md` (mesh-wire→World3dScene.snapshot bridge, selection/hover/camera/sun, graph-domain picking, catalogue fetch); new blocker: node-graph host never constructed in production on wgpu.
- 05:25 work-capacity lane landed → `📓️work-capacity-2026-09-10.md` (one ArtifactRetainedWorkCapacity per route; Preflight refusal vs capacity; restaged 05:22). Remaining runtime blocker: flowEvalTick self-dispatch has no window address → preview-window gate refuses → dispatched `📓️flow-eval-tick-address-2026-09-10.md` lane (will restage). Serve restarted for boot #6.
- 05:28 boot #6 on the 05:22 restage: plugin traps on first-step with plugin.reactor-turn-deadline (retryable) → No plugins loaded. Dispatched first-step-deadline lane → `📓️first-step-deadline-2026-09-10.md`.
- 05:30 boot #6b (reload): plugin booted this time (first-step deadline is load-flaky); windows replaced by 'Unterminated string in JSON at position 512' from ShellHost render + typed-operation completion effects → a 512-byte truncated JSON string reaches JSON.parse.
- 05:31 dispatched JSON-512 truncation lane → `📓️json-512-truncation-2026-09-10.md`. In flight: flow-eval-tick address (restage), hot-path optimization, FlowHost ownership, wgpu node-graph, first-step deadline, JSON-512.
- 05:57 hot-path optimization landed → `📓️hotpath-optimization-2026-09-10.md`: eval tick 26.2→6.9 ms mean (worst 35.9→7.4 ms, under the 8 ms ceiling), dispatch+settle 110→10 ms, hot-path console lines 15+→0 (SEMIO_RUNTIME_DIAGNOSTICS flag), eval publications 3→1 per boot, zero tessellation on hover re-render; dominant cost was the 108 KB catalogue JSON re-serialized per tick (now typed Arc map). gen3d lib 315/5, gen2d 228/2.
- 06:07 FlowHost ownership lane landed → `📓️flow-host-ownership-2026-09-10.md`: gen3d 289/23 → 315/5, gen2d 228/2; remaining reds: window-transient close ladder, first-tick budget, envelope Poll::Fault, module.vcs merge.
- 06:14 wgpu node-graph lane landed → `📓️wgpu-node-graph-2026-09-10.md` (engine surfaces were cfg(test)-only in production; Flow window now live on wgpu); dispatched wgpu engine-surfaces lane (World3d/TiledMap/Board2d) → `📓️wgpu-engine-surfaces-2026-09-10.md`.
- 06:15 first-step deadline lane landed → `📓️first-step-deadline-2026-09-10.md` (5 s wall ceiling now prices executing µs; browser shard replays retryable lifecycle deadlines instead of trapping). New: generation3d never reaches Retired on InstanceClose → dispatched close-ladder lane → `📓️close-ladder-2026-09-10.md`.
- 06:22 JSON-512 lane landed → `📓️json-512-truncation-2026-09-10.md` (shell reader concatenated only slice 0 of 33 packed text slices; typed section-payload fault; TS-only, no restage). Disk hit 100% at 05:38 from 14 private target clones; removed finished lanes' dirs → 180 GB free.
- 06:25 boot #7 on 06:19 restage: action chain healthy, tick loop runs; blocker = producers emit manifest id 'brep' vs registry plugin-id addressing → dispatched `📓️extension-addressing-2026-09-10.md` lane (restages, gates TS chatter).
- 07:11 wgpu engine surfaces landed → `📓️wgpu-engine-surfaces-2026-09-10.md` (World3d/TiledMap/Board2d engines in production; two World3d interaction bugs fixed). Started wgpu wasm dev lane from main session: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f5f6952-6c25-4056-a743-773b3668812e/scratchpad/wgpu-dev.sh` (private target, port 6118), log `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f5f6952-6c25-4056-a743-773b3668812e/scratchpad/wgpu-dev.log`.
- 07:17 extension-addressing lane landed → `📓️extension-addressing-2026-09-10.md` (one translation surface; miss → typed faulted preview status; TS chatter gated; restaged 07:14). **Root blocker identified**: the served plugin's flow extension registry is empty — no setContributions on procedural, channel 8 KiB vs 294 KB payload → dispatched contributions-delivery lane → `📓️contributions-delivery-2026-09-10.md` (paged 512 KiB tool factory, restage).
- 07:21 boot #8 on 07:14 restage: typed 'Geometry extension unavailable' preview fault surfaces correctly; wgpu dev lane first run died on a stale 🎞️frame-worker.js (regenerated via framework-renderer-wgpu:generate-frame-worker), relaunched with a readiness waiter.
- 07:26 wgpu dev lane: plugin closure builds 11/11 in the private target, but the bundle gate 'frame-worker.js is stale' fails even right after a deterministic regenerate → dispatched wgpu frame-worker lane → `📓️wgpu-frame-worker-2026-09-10.md`. In flight: contributions delivery (restage), close ladder.
- 07:48 wgpu frame-worker lane landed → `📓️wgpu-frame-worker-2026-09-10.md` (cwd-relative bundle banners + inlined plugin hashes made the worker context-dependent; stable-artifact sync assumed hashed names); trunk build succeeded. Relaunching wgpu serve.
- 07:56 wgpu boot #1: flow plugin traps on catalog-authority (13+ unmigrated generated actions), wgpu shell 'native-owner-required' worker-boot fault. Dispatching flow-catalog lane + wgpu-shell-boot lane.
- 08:33 wgpu shell boot lane landed → `📓️wgpu-shell-boot-2026-09-10.md` (boot opened plugins.first() = flow's app; per-plugin fault isolation; destroyApp guard removed); bundle rebuilt. Restarting trunk serve.
- 08:37 contributions delivery landed → `📓️contributions-delivery-2026-09-10.md` (paged public-invocation contract, 73 pages, per-app contributions job factories; native law: extrude ok, 3 meshes; restaged 08:35 with setContributions ×3). Serve restarted for boot #9.
- 08:46 boot #9: setContributions needed locale (shell patched); #9b: guest OOM (189 KB alloc) after paged install → dispatched guest-memory lane → `📓️guest-memory-2026-09-10.md` (restage). Flow catalog lane landed → `📓️flow-catalog-authority-2026-09-10.md` (34 migrated; restage needed in both closures).
- 08:48 wgpu boot #2: fatal ui-turn-overrun (5 ms progress hook) at shell-construct → dispatched wgpu UI-turn lane → `📓️wgpu-ui-turn-2026-09-10.md`.
- 09:23 wgpu UI-turn lane landed → `📓️wgpu-ui-turn-2026-09-10.md` (executing-span UiTurnClock + degrade-only ledger; ui-turn-overrun deleted); bundle rebuilt. Restarting wgpu dev (rebuilds closure incl. flow catalog fix).
- 09:34 close-ladder lane landed → `📓️close-ladder-2026-09-10.md` (reactor close witness; store envelope retirement progress; runtime close yields on Blocked; Retired in 2052 turns; restage needed). wgpu boot #3: worker-boot-step-overrun (plugin-graph 8.3 ms vs 8 ms) → dispatched wgpu worker-boot lane → `📓️wgpu-worker-boot-2026-09-10.md`.
- 09:35 dispatched taxonomy fix lane (640 procedural violations, assembly facets, pathExclusions) → `📓️taxonomy-fix-2026-09-10.md`. In flight: guest memory (restage), wgpu worker boot (bundle), taxonomy.
- 09:45 guest-memory lane landed → `📓️guest-memory-2026-09-10.md` (peak 32.6 MB of a 512 MiB ceiling; install payload retained once; real leak = 189 KB poll-task future box per turn on wasm → OOM after ~2 660 turns; restaged 09:41). Serve on 6018 was killed by a peer; restarting.
- 09:48 boot #10 on 09:41 restage: shard 0 terminated silently after plugin hot-swaps; boot cleanup throws plugin-ui.native-owner-required. Investigating.
- 10:25 shard-termination lane landed → `📓️shard-termination-2026-09-10.md` (watchdog killed a healthy worker during a 3 s synchronous first poll; first-turn ladder 30 s; cleanup no longer masks). Boot #11 boots but preview stays faulted → dispatched contributions re-arm lane → `📓️contributions-rearm-2026-09-10.md`.
- 10:27 boot #11: example picker → label changes to Box Shell Preview but the flow graph/preview stay on the hexagonal example (same as puzzle3d 09-09) → dispatched example-switch lane → `📓️example-switch-2026-09-10.md`.
- 10:44 wgpu worker-boot lane landed (served); wgpu boot #4 → intake-budget-exhausted → dispatched wgpu intake lane → `📓️wgpu-intake-budget-2026-09-10.md`. Tree-action audit → `📓️tree-action-dispatch-audit-2026-09-10.md` (window kinds declare no actions → shell gate drops them) → dispatched window-kind-actions lane → `📓️window-kind-actions-2026-09-10.md`.
- 11:21 example-switch lane landed → `📓️example-switch-2026-09-10.md` (switch published with effects=0 so preview recovery hung off a later refresh-ui; now re-arms every attached preview's flowEvalTick; picker dispatch testable). Restage required (poll-leak lane's restage in flight).
- 11:29 poll-leak lane landed → `📓️poll-task-leak-2026-09-10.md` (task box not leaked; turn future 252→54 KB; real driver = 4.4 KB/turn retention from the app's per-turn empty MoreWork result; restaged 10:54). Dispatched idle-turns lane → `📓️idle-turns-2026-09-10.md`. Serve restarted for boot #12.
- 11:36 boot #12: contributions install, evaluate dispatched to flow-extension-math, but each result delivery aborts in cabi_realloc → dispatched extension-result-realloc lane → `📓️extension-result-realloc-2026-09-10.md`. wgpu intake lane landed → `📓️wgpu-intake-budget-2026-09-10.md` (served).
- 11:39 wgpu boot #5: worker-boot-timeout at renderer-runtime (76 MB wasm compile > 60 s watchdog) → dispatched wgpu boot-watchdog lane → `📓️wgpu-boot-watchdog-2026-09-10.md`.
- 11:47 taxonomy lane landed → `📓️taxonomy-fix-2026-09-10.md`: procedural 650 → 22 (18 assembly codec bounds, 4 generated test-host adapters), repo-wide 19 464 → 2 123 (two detector bugs fixed with the mounts oracle 9/9; 42 dirs moved, 14 #[path] mounts rewritten); gen3d 328/4, gen2d 228/2; verify taxonomy abort cleared. Restage needed (moves).
- 11:48 dispatched assembly mount lane (five codec bounds, io/examples, fixtures, mount, window-kind actions) → `📓️assembly-mount-2026-09-10.md`. In flight: contributions re-arm, window-kind actions, idle turns, extension-result realloc, wgpu boot watchdog, assembly.
- 12:10 contributions re-arm lane landed → `📓️contributions-rearm-2026-09-10.md` (setContributions published nothing; now invalidates the retained session on registry generation and re-arms flowEvalTick per attached preview; 0 → 3 meshes natively). Restage required. gen3d 334/2.
- 12:18 wgpu boot-watchdog lane landed → `📓️wgpu-boot-watchdog-2026-09-10.md` (declare-phase liveness protocol, streaming compile progress, IndexedDB module cache; served 12:14). wgpu boot #6 in progress (wasm-compile 100 %, 15 % overall after 130 s).
- 12:25 wgpu boot #6: no watchdog kill; stalls silently at shell-boot 86 % (≥5 min, no fault/console) → dispatched wgpu shell-boot silence lane → `📓️wgpu-shell-boot-silence-2026-09-10.md`. Killed the taxonomy lane's leftover `verify taxonomy report` (1h52m). In flight: window-kind actions, idle turns, extension-result realloc, assembly, wgpu shell-boot silence.
- 12:44 window-kind actions lane landed → `📓️window-kind-actions-2026-09-10.md` (per-window owned action refs; audit's gate hypothesis refuted — all 44 actions were declared; found+fixed an unretired FlowFixture abort in generate_preview::render; stable DOM ids). Restage pending on idle-turns + realloc lanes.

## Session 2 — Opus 5 High (cursor-chat), resumed 2026-09-10 21:18

Coordinator handover: session 1 (Fable 5.1, claude-code, ⚪9f5f6952) went idle at 12:44 with five lanes
mid-flight and no restage since 10:54. Its process is still alive but has not acted for eight hours.
This session takes over the same ticket at HEAD f39d4b0db3. Repo MCP is again unavailable
(no `repo` namespace in this client's dynamic tool catalog), so ticket bookkeeping stays on disk.
Fleet for this session: Opus 5 High coordinating, Cursor Grok 4.6 Extra High for execution lanes,
Composer 2.5 for read-only exploration and audits, maximum parallelism.

- 21:20 reclaimed 145 GB of dead-lane cargo target clones from session 1's scratchpad
  (`target-asm`, `target-idle`, `target-realloc`, `target-realloc-trace`, `target-wgpu`; no process
  held them). Kept `target-wgpu-boot`, which session 1's still-live `trunk serve` on 6118 uses, and
  kept the puzzle3d session's dirs untouched. Disk was at 91 %.
- 21:21 restage started for boot #13 — the first one that carries the realloc paging, contributions
  re-arm, example-switch, window-kind-actions, idle-turns and taxonomy-move work:
  `CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" SEMIO_BUILD_BUDGET_MS=14400000
  SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react bun nx run
  @semio-tech/framework-os-dev:activate-generation3d-react-dev` → `🗑️generated/s13-restage.txt`.
  It doubles as this session's compile gate over session 1's mid-flight edits.
- 21:24 procedural react serve restored on **6018** (`📜️serve-generation3d-react.sh`, this ticket).
  Session 1's note that "agent-started serves die with the agent" is a process-group teardown:
  `nohup … &` from a tool shell is killed when the shell session ends. `screen -dmS g3dreact` with
  the script redirecting its own stdout survives. Ports in use by OTHER sessions: 6013 and 6014 are
  the puzzle3d agent's react serves, 6118 is session 1's wgpu trunk serve.
- 21:25 wave 1 dispatched — one execution lane (Grok 4.6 xhigh) and three read-only audits
  (Composer 2.5), all forbidden from running cargo/nx while the restage holds the shared target:
  browser probe harness → `📓️browser-probe-harness-2026-09-10.md`; user-facing gap inventory →
  `📓️gap-inventory-2026-09-10.md`; wgpu React-vs-wgpu boot divergence (resuming session 1's
  truncated shell-boot-silence report) → `📓️wgpu-boot-divergence-2026-09-10.md`; open-debt
  punchlist harvested from all ~60 lane reports → `📓️open-debt-punchlist-2026-09-10.md`.
- 21:29 restage GREEN in 7 m 45 s — `Successfully ran … and 38 tasks`, `Activated generation3d react
  dev: 11 completed components (changed)`, 2/39 cache hits, no errors. **Session 1's five mid-flight
  lanes all compile**, so the working tree needed no repair: the tree is healthy at HEAD f39d4b0db3
  plus session 1's uncommitted edits.
- 21:29 **staging-root correction, important for reading older reports.** There are two
  `🔌️plugin-modules/` trees and only one of them is served:
  | tree | state | role |
  |---|---|---|
  | `💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌀️procedural/` | stale, frozen at 09:30, 81 219 866 B | NOT served |
  | `💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/` | fresh 21:28, 82 273 400 B | **what vite serves** |
  `⚙️vite.config.ts:32` resolves `pluginModulesDir` to the `🔌️plugin/…/dist/<profile>/` tree; the
  `🧑‍💻dev/` tree is a leftover. Verified by hash: the descriptor fetched over HTTP from 6018
  (200 880 B, sha256 `1dec7265…`) is the fresh one, not the 09:30 copy (`0139b8e8…`).
  Consequence for prior art: `📓️extension-result-realloc-2026-09-10.md` §2 ran `🐍️realloc-probe.mjs`
  against the STALE `🧑‍💻dev/` tree, so its measurements came off a 09:30 module. Its conclusions are
  structural (jco lowers a `pack` as one `cabi_realloc`) so they still hold, but any future probe
  must point at the `dist/dev/` tree.
- 21:33 dispatched the fault-arm symmetry lane (the `STILL OPEN` item from
  `📓️extension-result-realloc-2026-09-10.md` §4.5, confirmed first-hand: `🏛️ShellHost/🟦️.tsx:1663`
  writes `encodePackValue(fault)` while `🔌️plugin/🌐host/🦀️.rs:48` reads `dsl::decode_fault_bytes`,
  and the same `decode_fault_bytes` appears on ~15 other host-effect error arms) →
  `📓️fault-arm-symmetry-2026-09-10.md`.
- 21:55 **boot #13 observed** on the 21:28 restage → `📓️boot-13-2026-09-10.md` (headless Chromium via
  Playwright, which is already a devDependency; traces armed with
  `localStorage.SEMIO_RUNTIME_DIAGNOSTICS=1`). This retires the class of failure the last twenty
  lanes were chasing and replaces it with a narrower, further-upstream one:
  - **no trap, no fault, no page error, no failed request.** Boot #12's `cabi_realloc` abort is gone
    and nothing replaced it. Chrome is finished work: navbar, all four panels, example picker reading
    `Hexagonal Mushroom Column`, Edit/Generate mode buttons, Flow + Preview dock panes, full footer;
    viewer state live (`show-mode=shaded`, `preview-camera { position=@4,-4,3 fov=45 }`, real sun).
  - **no `canvas` element exists on the page at all** — `[]` after a 60 s settle, after clicking the
    Preview tab, after clicking the Flow tab, and after clicking Generate. Both window bodies are
    blank rectangles. A regression against boot #5, where `📓️node-graph-attach-2026-09-10.md` had the
    flow canvas drawing.
  - **generate mode opens nothing**: the DOM id set is byte-identical after clicking
    `#playground.navbar.modes.generate`, though the artifact declares `👁️preview`/`📝️form`/`🗂️generations`
    and the descriptor carries their labels. Generate mode has never been exercised at runtime in
    this ticket — every boot only ever looked at the two edit-mode windows.
  - **`flowEvalTick` 184×, `invokeExtension` 0×.** Not invoked-and-faulted like boot #12 —
    never invoked. All nine extensions hot-swap cleanly incl. `flow-extension-brep v0.3.0`.
  - **idle invocation storm**: `setContributions` (an install action with a 73-page ~294 KB payload)
    93× in 90 s, `plugin_exchange … branch=command-invocation` 949×, full-scope `applyHostEffects
    refresh` 314× → `guest linear memory at 322 240 512 B — 60 % of the 536 870 912 B budget, past the
    60 % install-peak ceiling`, monotonic, while displaying two empty rectangles.
  - **the guest trace writer emits one `console.error` per token** (5 750 raw lines ⇒ 1 452 logical
    messages, 4× fan-out; 7 775 lines armed), and emits routine `[DEBUG]` chatter at `error` level.
    `📓️hotpath-optimization-2026-09-10.md`'s "hot-path console lines 15+→0" does not hold for these
    two traces with the flag off.
- 21:57 dispatched the two decisive lanes off boot #13, file-disjoint:
  blank window bodies + generate-mode windows + `flowEvalTick`→`invokeExtension`
  → `📓️window-bodies-and-eval-dispatch-2026-09-10.md`; idle invocation storm + trace fan-out and
  level → `📓️idle-storm-and-trace-fanout-2026-09-10.md`.
  In flight: browser probe harness, gap inventory, wgpu divergence, open-debt punchlist, fault-arm
  symmetry, window bodies, idle storm.
- 21:40 wave 1 landed, four of seven cleanly:
  - **probe harness** → `🔍️browser-probe.ts` + `📓️browser-probe-harness-2026-09-10.md`. Playwright with
    swiftshader/WebGPU flags; `--url`/`--port`/`--plugin`, `--mode=boot|interact`,
    `--steps=example,hover,select,orbit`, `--settle`, `--label`, and a
    `semio.procedural3d.browser-probe/1` JSON summary. Runtime verification is now one command
    instead of hand-driving a browser, and it works against 6118 too.
    Its baseline run adds two facts boot #13 did not have: the example picker opens with **9 options**
    and selecting one DOES relabel and settle a second `setActiveExample` with `effects: 2` — so the
    picker and switch path are alive — while `hover`, `select` and `orbit` all fail with
    **`no preview canvas`**. This also **refutes** `📓️gap-inventory-2026-09-10.md`'s Lane G
    ("demo-session unreachable, 9th example"): all nine options are in the picker.
  - **gap inventory** → `📓️gap-inventory-2026-09-10.md`: ten file-disjoint lanes; edit mode judged
    complete in source, with generate mode, viewer, wgpu and assembly independently broken.
  - **wgpu divergence** → `📓️wgpu-boot-divergence-2026-09-10.md`: **refutes** the extension-await
    hypothesis for the 86 % stall — it is retained-UI intake across `create_app` plus multi-surface
    `refresh_ui` (`renderDocument` per window/panel/catalogue, ~163 M steps in 4096-step slices)
    inside one declared phase. But it confirms wgpu drops `InvokeExtension` in `queue_host_effects`
    (`_ => {}`), never sends `setContributions`, never flushes `deferred_actions` in `settle_boot`,
    and does not import `GUEST_HOST_ANSWER_CEILING_BYTES` — so adding dispatch without paging would
    re-create boot #12's abort. Ordered five-step fix plan with file ownership.
  - **open-debt punchlist** → `📓️open-debt-punchlist-2026-09-10.md`: 91 items — 38 still open
    (12 blocker, 14 degraded), 47 confirmed fixed later, 6 unclear — grouped into ten lanes.
  Three lanes (fault-arm symmetry, window bodies, idle storm) exited after ~2 minutes with no report
  and no source edits; re-dispatched as resumes with the probe's contract and its new evidence.
- 21:42 wave 2 dispatched, six lanes, file-disjoint:
  resumed fault-arm symmetry, resumed window bodies + generate mode + eval dispatch, resumed idle
  storm + trace fan-out; new wgpu extension dispatch + boot instrumentation (executing the divergence
  audit's plan, owns `🎯️targets/🧊️wgpu/**` and manages the 6118 serve itself) →
  `📓️wgpu-extension-dispatch-2026-09-10.md`; new assembly artifact mount (five codec bounds, mount,
  taxonomy, real content; owns `🗿️artifacts/🧩️assembly/**` + the mount block) →
  `📓️assembly-artifact-mount-2026-09-10.md`; new viewer preview round trip (owns
  `✳️any/👁️viewer/**`, proves itself natively since the shared no-canvas defect belongs to the window
  bodies lane) → `📓️viewer-preview-pipeline-2026-09-10.md`.
- 21:45 the assembly lane exited after ~3 minutes having only noted that the repo MCP is unavailable.
  **Lane-hygiene note for whoever coordinates next:** five execution lanes have now ended their turn
  early with a mid-thought status line as their final answer. Two causes seen so far — reaching for
  ticket bookkeeping (the repo MCP is unavailable to every client in this session; bookkeeping is the
  coordinator's job, and saying so explicitly in the brief fixes it), and simply stopping after
  orientation. Verify every lane against disk (`git status` plus the expected report path) instead of
  trusting a completion signal; four of the five had produced no edits at all.
- 21:48 coordinator traced the blank body end to end while the lanes ran, and handed the chain to the
  window-bodies lane → `📓️boot-13-2026-09-10.md` §4. Summary: the tabpanel has **zero children**, so
  this is not a canvas that failed to size. `🎨️Canvas/🟦️.tsx:1229` renders `<Window>` only when
  `activeDescriptor` is truthy; `:1194` looks it up in `windowsById`; `:1364` builds that map from the
  `windows` prop; and `:1176` feeds the TAB BAR from the layout node instead — which is exactly why
  correct `Flow`/`Preview` tabs sit above blank bodies. `windows` is `modeWindows`
  (`🏛️ShellHost/🟦️.tsx:9344`), and `:9085` returns `[]` whenever `windowUiByWindowId` is empty, which
  it is (`:1948`, from `shellState.windowUi`) because **the guest delivers no window UI at all** —
  very likely the same root cause as `flowEvalTick` 184× / `invokeExtension` 0×. Second, separable
  defect: that early return converts "no UI yet" into "no windows at all", defeating the
  `PENDING_WINDOW_UI_NODE` fallback the same function already uses at `:9113`/`:9159`, which is why
  the failure is silent. Owed law: a window whose UI has not arrived renders its pending node, never
  an empty body.

### Root cause closed by the coordinator — the contributions push (2026-09-10 ~22:00)

Wave 2 produced nothing on disk: six execution lanes dispatched, zero reports, zero source edits,
no cargo/vitest/playwright ever running. Rather than dispatch a seventh, I traced the defect myself
from the boot #13 console. It is closed, and written up in `📓️boot-13-2026-09-10.md` §5.

`🏛️ShellHost/🟦️.tsx` hands the procedural plugin the **aggregated contributions of all thirteen
plugins** — 397 921 chars, 98 % of it unaddressable by procedural — through the 4 KiB
public-invocation string envelope as **99 sequentially awaited pages** at ≈ **2.04 s each**, so
≈ **202 s**. The **only writer of `windowUiByWindowId` anywhere in the codebase** is the dispatch at
`:4386`, which sits *after* that loop in the same function. Hence `{}` for the first ~3.4 minutes of
every boot, hence `:9085`'s `return []`, hence `🎨️Canvas:1229` rendering no `<Window>` while
`:1176` still draws correct tabs from the layout node. Correct tabs over empty bodies is the exact
signature, and it matches boot #13's DOM. `flowEvalTick`'s 184 firings with `invokeExtension` at 0
are the same defect, not a second one: contributions never landed.

Every settle used in this ticket — 60 s, 90 s — expires less than halfway through the push. That is
why ~20 lanes chased traps and faults: **there was never anything wrong at the point they were
looking**, and the window in which they looked was inside the blockade.

Two of my own earlier readings were wrong and are superseded:
- §2.4's "93 `setContributions` invocations / invocation storm" — it is **one** push of 99 pages.
- §4's "the guest delivers no window UI" — one link short; the guest is not at fault at all, and
  `app.windowKinds` is fully populated (all five editor kinds present in the staged manifest).

Re-scoped ownership to keep two lanes off the same lines:
- **window-bodies lane** — sole owner of `:4344`–`:4400` and `:9085`. Three ordered parts: unblock
  the first paint (pending nodes must not sit downstream of a data push; drop the `:9085` early
  return so the `?? PENDING_WINDOW_UI_NODE` fallback works), scope the payload by flow-graph
  reachability, then move contributions onto the `pack` path — one crossing, not 99.
- **turn-cost lane** (was idle-storm) — the 2.04 s *per 4 KiB page*. A 4 KiB command cannot cost two
  seconds; whatever each guest turn drags behind it is worth more than the paging, because cutting
  99 crossings to 1 still leaves every interaction unusable at 2 s a crossing.

Also settled in passing: the staged manifest declares apps for `generation2d` and `generation3d`
only — there is **no assembly app at all**, confirming the assembly lane's premise independently.

### Lane hygiene — delegation is not holding in this client

Six of six wave-2 execution dispatches ended their turn on a mid-thought plan with nothing on disk.
Two causes identified: reaching for ticket bookkeeping via the unavailable repo MCP, and simply
stopping after orientation. Every resume now states that bookkeeping is the coordinator's job, that
the MCP is unavailable to everyone by design, and that the turn may not end until the report exists
on disk and `git status` shows edits. Read-only Composer audit lanes have not shown this failure —
all four produced substantial reports. **Verify every lane against disk, never against its
completion signal.**

### Viewer lane parked, not restarted (2026-09-10 ~21:58)

Seventh consecutive execution-lane early exit ("I'll keep gathering the viewer preview window,
ViewEmit, and the editor's drain/tessellate helpers…", nothing on disk). Deliberately **not**
restarted: the viewer app is `s.procedural.generation3d@1/*#viewer` with a single window kind, so it
sits behind exactly the same contributions blockade as the editor. Its preview cannot render until
the window-bodies lane lands part 1. Restarting it now would burn a lane on work that cannot be
verified. Re-dispatch after boot #14 confirms bodies fill.

### Example envelopes are generated, not hand-authored — todo de-scoped

The empty 171 B / 170 B / 73 B `🎒️.pack.semio`, `📡️*.spr.semio` and `🔧️*.op.semio` beside each
example are **not** assets to handcraft. Two independent confirmations:

- Each example's `🦀️.rs` references only the DSL: `PRIMARY_TEXT =
  include_str!("🖼️assets/…/🗣️.dsl.semio")`, passed to `ExampleSource::new(ID, label(),
  PRIMARY_TEXT, ICON)`. The other three envelopes are never read by the example at all, so they
  cannot be blocking example loading, preview, hover or selection.
- The assembly subset's mutation tests state the rule outright: "the
  `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are **derived from it
  by `fixtures generate`**".

So they are stale generator output, and the fix is to run `fixtures generate` for the generation3d
subset and check that it fills them — a codec round-trip fixture concern, not a user-facing one.
Moved off the critical path.

### Long-settle probe — the falsification test for §5

Running detached as `screen -S b14probe`, label `b14-longsettle`, **settle 260 s**, deliberately
longer than the ≈ 202 s projected push. §5 predicts bodies stay at 0 for ~200 s and then fill. If
they fill, the diagnosis is confirmed at runtime rather than by reading; if they never fill, §5.2's
ordering claim is wrong even though §5.1's measurements stand, and the blockade is only half the
story. Either outcome is worth the 4.5 minutes.

Note for anyone launching long jobs here: `nohup … &` from an agent tool shell **does not survive** —
process-group teardown kills it (this killed the first attempt at this very probe, and earlier
killed the react serve). Use `screen -dmS <name> zsh -c '… > log 2>&1'`; this screen build is
4.00.03 and does not support `-Logfile`.

### Boot #14 (260 s settle) — the terminal failure is a guest OOM (2026-09-10 ~22:05)

The falsification test came back and it both confirmed §5 and overturned boot #13's headline.
Written up in `📓️boot-13-2026-09-10.md` §6.

Bodies and canvases stayed at 0 for 114 s exactly as §5.2 predicts, then at 121 s ten window UI
nodes and **three canvases** appeared. So "no canvas anywhere" and "the guest delivers no window UI"
were both artefacts of probing inside the blockade. The ordering defect is a two-minute *delay*, not
the terminal failure.

The terminal failure is that the guest is already dead by then: at 116.1 s the actor aborts with
**`rust_oom` → `process::abort` → `abort_internal` → `unreachable`**. Guest linear memory was at
**322 371 584 B (307 MiB) of a 536 870 912 B (512 MiB) budget at 65.7 s** — the codebase's own
install-peak diagnostic flagged it — and exhausted the rest by 116 s. The worker error carries
**`framesBytes=63345194`** (63.3 MB of retained frames) at the moment of death. `meshes=0` is
therefore unrecoverable on this boot: the actor died five seconds before its windows existed.

Boot #13 reported "no trap, no fault, no page error" only because its window was 60 s. **Every probe
in this ticket, across both sessions, expired before the OOM.** That is why ~20 lanes hunted a trap
and found nothing: the trap is real, it is an OOM, and it is caused by the contributions push.
Session 1's realloc/abort instinct had the mechanism right and the trigger wrong. All probes from
here use `--settle=260`.

One root cause, two amplifiers, and the two live lanes are aimed at exactly one each — now confirmed
rather than guessed:
- **payload size** (window-bodies lane): 398 KB, 98 % unaddressable, 99 sequential turns → scope it
  and move it to the `pack` path so it crosses once.
- **per-turn retention** (turn-cost lane): linear memory never shrinks, so ~3 MB left behind per
  turn is permanent. Handed them `framesBytes` and the existing install-peak sampler as leads.

Raising the 512 MiB budget is not on the table; it only moves the abort later.

### `framesBytes` was lying by 10×, and one fix landed from the coordinator (2026-09-10 ~22:10)

The turn-cost lane came back with an empty response — my second interrupt of it almost certainly
destroyed its context. **Lesson: stop interrupting working lanes.** Deliver corrections at their
natural completion instead. That is eight execution dispatches with nothing on disk.

So I chased the memory number myself and it turned out to be wrong. `replyError` in
`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` sized the turn's `events` with `JSON.stringify(frames).length`
whenever `frames` was not itself a buffer — and a turn's `events` is always an **array** of buffers,
so every report took that path. `JSON.stringify` renders a `Uint8Array` as `{"0":12,…}`, measured at
7.9× per byte at 1 KB, 9.9× at 100 KB, 11.8× for large values. So `framesBytes=63345194` meant
roughly **5.8–10.6 MB of real wire bytes**, not 63 MB. §6's OOM conclusion is unaffected — that
comes from the guest's own `rust_oom` stack and the 512 MiB budget line — but the payload figure I
handed the turn-cost lane was off by an order of magnitude. Corrected in `📓️boot-13…` §7.

**Fixed it myself** (file unowned by any lane): an array whose members are all buffers is now summed
by `byteLength`; a lone buffer still reports `byteLength`; anything else still falls back to
stringify. Validated on six cases — lone buffer, array of three, empty array, `undefined`, non-buffer
object, and a mixed array that must still fall back — all six pass, and `bun build` bundles the
module (95 modules). The comment now records why the array path is the only correct one, because the
old comment asserted the exact property the code did not have.

Method note worth keeping: **a diagnostic never checked against a known input is not evidence.**
This one had a docstring claiming it avoided stringify, and it sent the memory hunt off by 10×.

Retention work re-dispatched as a **fresh Opus lane** rather than a resume, briefed with the
corrected numbers, the turn plumbing I traced (`runQueuedTurn` at `🔌️PluginRuntime:2064`,
`typedOperationAcknowledgements` at `:537`, the lease deliver at `📮️shard-client:1584`–`:1593`), the
`rg -a` note for PluginRuntime's intentional NUL separators, and a hard boundary off ShellHost
`:4344`–`:4400`/`:9085`. Fresh dispatch chosen deliberately: every resume so far has died, while the
one hardened resume that survived (wgpu) is the only Grok lane with real edits on disk.

Also confirmed: the **wgpu lane is genuinely working** — unstaged edits present in
`🌐️browser-worker/🦀️.rs`, `🎞️frame-worker/🟦️.ts`, `🐚️plugin-bridge.ts`,
`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`, `🩺️window-fault/🟦️.ts`, `🎠️kernel/🟦️.ts`, `🏃️run/🦀️.rs`.

### Fault-arm lane landed and I verified it against disk (2026-09-10 ~22:22)

**First execution lane of session 2 to actually deliver.** Report:
`📓️fault-arm-symmetry-2026-09-10.md` (5.3 KB). I checked every claim against the logs in
`🗑️generated/fault-*.txt` rather than trusting the summary, and all seven hold:

| Claim | Log evidence |
| --- | --- |
| `cargo check` plugin | `Finished dev profile … in 36.53s`, no errors |
| `cargo check` plugin-host | `Finished … 1m 08s` |
| `cargo check` procedural native | `Finished … 2m 11s` |
| `cargo check` procedural `wasm32-wasip2` | `Finished … 2m 33s` |
| `a_packed_host_fault_round_trips_through_outcome_to_result` | `test result: ok. 1 passed; 0 failed` |
| `a_faulted_invocation` | `test result: ok. 1 passed; 0 failed` |
| vitest `packs completion-result.fault` | `Tests 1 passed \| 92 skipped (93)` |

Edits confirmed present in `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`, `🔌️plugin/🌐host/🦀️.rs`,
`🔌️plugin/🦀️.rs`, `⚛️reactor/🧪️tests/🔬️extension-continuation/🦀️.rs`, plus a new fixture
`⚛️reactor/🧫️fixtures/extension-result-fault-pack.json`. The extension-result fault arm is now
`pack` on both sides, matching the OK arm; live-path mismatch 1 → 0 and AppFrame summaries 2 → 0.

The lane correctly flagged that it did **not** restage and did **not** probe Chrome, so the on-screen
error surface is unverified. Worth being precise about what that means: the shell's TypeScript side
was already `encodePackValue` from a session-1 lane, so the currently served guest (built 21:28)
decodes JSON while the host encodes pack. **That mismatch predates this lane** — this lane is the fix
for it, not the cause — and it only affects fault paths.

**Restage deliberately deferred.** Not restaging now, for two reasons: the window-bodies and
guest-memory lanes are mid-flight and a restage would bake their partial state into the module their
own probes read, and a restage costs ~7m45s of shared cargo time. The guest-memory lane restages
itself after Rust changes per its brief, which picks this up for free. One clean restage before
boot #15.

### wgpu lane landed — verified, with one attribution corrected (2026-09-10 ~22:25)

Report `📓️wgpu-extension-dispatch-2026-09-10.md` (10 KB). Verified against disk, and in one case by
re-running the tests myself rather than reading its log:

| Claim | How I checked it |
| --- | --- |
| `cargo check` wgpu renderer, native + wasm32, 0 errors | `Finished dev profile … 15.68s` / `0.93s`, zero `^error` lines |
| new suite registered in three places | present at react `vitest.config.ts:24`, wgpu `vitest.config.ts:8`, wgpu `📜️script.ts:527` |
| "4/4 extension-dispatch tests passed" | **re-ran it myself**: `Test Files 1 passed (1)`, `Tests 4 passed (4)`, 1.48 s — matches its claimed 1.43 s |
| `shell-boot` finished in 2.6 s (was pinned at 86 % indefinitely) | `wgpu-boot-14` probe capture present |

One scare that was not this lane's doing: `🗑️generated/wgpu-world-suite.txt` records
`test result: FAILED. 105 passed; 10 failed`. Its mtime is **04:30:32**, so it is session 1's log,
not this lane's run. The ten `world::tests::*` failures (marquee publish/cursor, object registry
capacity/ABA, saturation FIFO, terrain tile sync, prepared-resource dedup, live-renderer generation
wake) are **pre-existing debt nobody has touched**, and their current status is unknown. Added to the
punchlist rather than attributed here.

**Correction to the lane's conclusion.** It reported empty windows with `invokeExtension` 0× and
called it "the same shared blank-body reason as React". It is not. I checked its own capture:
`wgpu-boot-14-console.jsonl` has **24 records total** (React's boot has 895), **zero mentions of
`contributions`**, and 29 × `effects=0`. So on wgpu the contributions push **never happens at all** —
the registry is empty because nothing was ever sent, not because a send is slow. Same symptom,
different defect:

| | React | wgpu |
| --- | --- | --- |
| contributions push | happens once, 99 pages, ≈ 202 s | **never happens** |
| windows | appear at 121 s | never appear |
| guest memory | OOM at 116 s | no pressure — 2.6 s boot, nothing pushed |

The consequence matters for sequencing: **the React fix will not fix wgpu.** The window-bodies lane's
payload scoping and pack transport change a push that wgpu does not make. This lane built the
*receiving* end (paged `setContributions` in the wgpu Shell Rust) while the *sending* end does not
exist there. Resumed the lane with exactly that.

### Window bodies render — first user-visible win (2026-09-10 ~22:40)

`📓️window-bodies-and-eval-dispatch-2026-09-10.md`. Part 1 of three landed. Verified from the lane's
own capture `🗑️generated/probe-bodies-1-2026-09-10T20-09-14` rather than from its summary:

| | before | after |
| --- | --- | --- |
| tabpanels with content | 0 of 1 (`role="tabpanel"></div>`) | **2 of 2, zero empty** |
| canvases | 0 | **3** |
| time to window UI | 121 s (boot #14), never inside a 90 s settle | **9.7 s** |
| window UI nodes | none | all ten (`proceduralMain`, `proceduralPreview`, controls/measures/engagement/utilityBar) |

The `:9085` early return is gone from `🏛️ShellHost/🟦️.tsx`, confirmed by grep. Pending window UI now
dispatches ahead of the contributions push instead of behind it. TS-only, so no restage was needed.

Parts 2 and 3 did not land, and the lane was straight about it rather than rounding up:
- **Part 2 (scoping) is on disk but inert.** `reachableKinds: []` every time, so the fallback restores
  the full payload and the push is still `397921 chars / 99 pages / ≈ 202 s`. Cause: the cached
  `BuiltNode` is lane-split and carries no `neuronKind` at push time. A greenfield repo cannot keep a
  path that always falls back, so the resume tells it to either resolve reachability from the
  document's flow graph (where the graph actually lives) or remove the scoping and defer it honestly.
  Hazard noted: a broken scoped run briefly pushed `chars: 2` / 1 page, so an empty scoped payload
  must fail loudly rather than install nothing.
- **Part 3 (one `pack` crossing) not started.** This is now the critical path for the whole ticket:
  it takes the push from ≈ 202 s to roughly one crossing, which puts `brep` in front of the first
  eval tick. The preview's current failure is exactly `flow.extension-not-contributed / brep`.

New gap the fix exposed, and now its own lane: **the Flow window paints a placeholder instead of the
example's 7-node graph.** It mounts correctly at 966×836, so this is content, not mount. Dispatched
as a fresh Opus lane against
`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/`, file-disjoint from everything else in flight, with
hover and selection in scope since the objective requires them. Generate mode does mount its three
bodies (Generations / Form / Preview).

Still true and unchanged: bodies now appear at 9.7 s but the guest still aborts with `rust_oom` at
~116 s, so the user-visible state is "windows appear, then the plugin dies". Both remaining
amplifiers are owned.

### Turn-cost lane landed; a transient tree break found and cleared (2026-09-10 ~22:47)

`📓️idle-storm-and-trace-fanout-2026-09-10.md` (11.5 KB), plus capture
`🗑️generated/probe-turncost-after-2026-09-10T20-43-49`. The lane I thought my interrupt had
destroyed came back with real work — **the interrupt was survivable after all**, so the earlier empty
response was transient rather than fatal. Worth remembering before I avoid interrupts on principle.

It independently reached the same `framesBytes` conclusion I did, from the other direction, and named
the actual retain: **each 4 KiB `setContributions` page runs a full `plugin_exchange`, allocating
Invocation + Ephemeral even when `Emit` is empty.** Those peak temporaries stick in `dlmalloc`, and
since wasm linear memory never shrinks the guest only climbs — 322 MB at 65.7 s, `rust_oom` at
116.1 s. So the 99-crossing payload and the retention are not two independent defects: **the crossing
count is the multiplier on a per-exchange allocation peak.** That makes the other lane's single-pack
crossing the fix for both, and this lane's honesty is worth noting — it did **not** claim the OOM was
gone, and it is not.

Verified claims: plugin `cargo check` and procedural native `cargo check` both `Finished`, 0 errors;
`storm-plugin-debug-law.txt` shows `test result: ok. 1 passed`. What it cut are the guest trace
amplifiers, not the allocation: per-token `console.error` fan-out (8 writes → 1 sink emit on the law
fixture), one host call per newline, `[DEBUG]` to `console.debug`, and the ungated
`plugin_exchange` / `cooperative-maintenance` traces that produced boot #13's 5 750 console lines.
Restage needed before its per-turn curve prints.

**Transient tree break, found and now clear.** Its `storm-procedural-wasm-check2.txt` recorded
`could not compile semio-framework-plugin (lib) due to 6 previous errors` on `wasm32-wasip2` — two
concurrent lanes colliding in `⚛️reactor/`: `note_turn_events` defined twice (lines 136 and 181 of
`🔄️turn/🦀️.rs`, one instrumentation function added by each lane) and `wit::CompletionResult`
unresolved in `⚛️reactor/🦀️.rs`. **A broken wasm build blocks every restage, and a restage blocks
every lane's runtime verification, so this was the highest-priority thing in the ticket.** Both have
since healed on their own: one definition at `:138`, no `wit::` uses left. I re-ran it myself rather
than trusting the stale log — `cargo check --target wasm32-wasip2` **Finished in 1m 10s, 0 errors**.
The tree is restageable.

New gate debt this exposed: the wasm build carries **`warning: function encode_fault_pack is never
used`** (from the fault-arm lane) plus an unused `new` in the `framework_reserved_job!` macro. The
repo has a `verify rust-warnings` gate, so these fail it. Added to the gates item, not patched here.

Restage still deliberately deferred: the guest-memory lane's probe (`screen memchk4`) is live against
6018 and a restage would swap the module under it. One clean restage once the in-flight probes land.

### One crossing landed; two sizing decisions reversed (2026-09-10 ~23:05)

The pack crossing is in: **one `setContributions` `handleCommand` instead of 99 JSON pages.** That is
the structural change the ticket needed, and removing the always-firing scoping fallback was right.
Two follow-on decisions were backwards and I have reversed them.

**Reverted: the ingress ceiling raise.** The lane raised `SHARD_COMMAND_MAXIMUM_PAGES`
(`📮️shard-client/🟦️.ts:112`) and `COMMAND_MAXIMUM_PAGES` (`📡️spr/🧵️channel/🦀️.rs:57`) from 64 to
128 so the 397 921-char pack would fit. Wrong direction on three compounding counts: it doubles the
permitted burst into a guest that is already dying of memory exhaustion; it leaves the payload
O(number of plugins), so plugin fourteen breaks it again and the ceiling gets raised again; and the
objective is to optimise the architecture, not widen limits until today's data fits. A 4 KiB envelope
was the wrong shape for bulk data and a 256 KiB one is the same mistake larger.

**Its own capture shows the damage the raise did:** `probe-bodies-2` has
`command ingress exceeds 64 pages` twice and then **43 `v102_1` faults**. Host raised to 128, staged
guest still enforcing 64, guest refuses, everything downstream faults. Which also means the lane's
"no `rust_oom` in the 9.9 s probe" is **not evidence about the OOM** — the probe died at 9.9 s and
the abort is at 116 s. Recorded so nobody carries it forward as a positive result.

**Direction given instead: scope the payload, which needs no new machinery.** The lane kept exactly
the right pieces in `🎠️kernel/🟦️.ts` — `reachableKindsFromUnknown` (`:271`), `scopeContributionsJson`
(`:288`) and a `describe("scopeContributionsJson")` suite (`:3070`). The helpers were never the
problem; **their input was.** Fed the cached lane-split `BuiltNode`, they always saw `[]`. The kinds
must come from the open document, whose graph is the example's seven nodes in its `🗣️.dsl.semio`.
Scoped that way the payload is kilobytes and fits inside the original 64-page cap comfortably.

I checked whether a manifest declaration could substitute for the graph and it cannot: procedural
declares `activationEvents` for its two artifact kinds and one `documents.write` capability request,
and nothing that names brep. The document graph is the only honest source.

**Recorded as the end state, not built now:** push-based contributions are O(what exists) however
well scoped. The design that stays O(what is used) is **demand-driven** — the guest asks for a kind's
contribution the first time it meets one it does not know. Graph scoping gets a working app; pull is
what stops this recurring as plugins are added.

### The OOM is gone — root cause was the async-lift parameter area (2026-09-10 ~23:20)

`📓️guest-memory-retention-2026-09-10.md` (14 KB). This is the biggest result of the session and I
verified it hard, because it is the headline claim of the ticket.

**What actually retained the memory** — and it is none of the four things we suspected. Not the
events arrays, not retained UI/`BuiltNode` state, not the guest's operation log, and **not dlmalloc
reuse**: a standalone allocator probe measured 0 bytes of growth for every alloc/free pattern
thrown at it. It was the canonical-ABI **indirect parameter area of the async-lifted `reactor.poll`
export** — 4 360 B allocated in guest memory by the host through `cabi_realloc` every turn and never
freed, because wit-bindgen 0.57.1 emits the `GuestDeallocate` for a guest export's parameter area
only when the lift is *not* async (`wit-bindgen-core` `abi::call`, `if sig.indirect_params &&
!async_`). The area existed only because `option<command-ingress-page>` carried a fixed 4 096-byte
64-block record **by value on every turn**, `None` on virtually all of them, pushing the call past
the canonical ABI's 16-flat-parameter limit.

**This supersedes the turn-cost lane's diagnosis.** That lane said the retain was `plugin_exchange`
allocating Invocation + Ephemeral per page with the peak sticking in `dlmalloc`. The allocator probe
disproves it. Two lanes, two confident diagnoses, and the one with a controlled experiment won.

**Verified independently, not taken on trust:**

| Claim | How I checked it |
| --- | --- |
| ~4 368 B/turn leaked before | `🗑️generated/leak-wasm-idle-turns-before.txt`: 19 529 728 B at turn 0 → 19 791 872 B at turn 60 = **4 369 B/turn**, which is 4 360 B area + dlmalloc's 8 B header **exactly** |
| 0 B/turn after | report's 512-turn curve, flat at 36 438 016 B from turn 0 through 480, same `2 of 512` MoreWork answers before and after so it is not the guest doing less work |
| `rust_oom` gone | `probe-mem-1` console: **zero** matches for `rust_oom`, `process::abort`, `unreachable` or the 60 % install-peak line |
| boot settles at 9.9 s | `probe-mem-1/probe.log`: settled 9.9 s, ten window nodes, 3 canvases (baseline aborted at 116 s and only rendered at 121 s) |
| staging path works at runtime | `probe-mem-2-interact`: 146.7 s, orbit step ok, 10 windows, **67 `command-complete` settlements** |

**The fix is a schema change, not a workaround.** Bulk payload comes off `poll`, which now flattens
to 7 values and is passed directly; pages move to their own `stage-command-page(cursor, bytes:
list<u8>)` and `stage-cold-pair-page` exports, staged one per turn. The 4 KiB record and the reactor's
`use byte-page` are deleted outright — no compatibility layer — and the 512 MiB budget is untouched,
which is what I asked for. The WIT now carries a law ("exactly one staged page per `poll`") and
documents the wit-bindgen internal that caused it. `byte-page` still exists for `return-page`, which
is correct.

Two things it flagged. It fixed guest-trap classification, which had been reading only the first
line of the wasmtime error and discarding the trap code — that is why session 1's turn-0 trap was
undiagnosable. And the actor package's vitest has **12 pre-existing failures**, all ajv failing to
resolve a `NonZeroU64` schema `$ref`; unrelated, added to the gates item.

Remaining for geometry: `meshes=0`, still gated on `brep` reaching the guest, which is the
contributions lane's scoping work. With the OOM gone and bodies rendering, the viewer lane is no
longer blocked on infrastructure and has been re-dispatched.
