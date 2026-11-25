# Lost guest effects on React (2026-11-25)

Probe `effect-loss-probe.mjs` (node, one capped headless chromium, served modules rewritten only). Outputs `🗑️generated/effect-loss/{a,b}-log.txt`, law `red.txt` / `green.txt`. Component tested against: procedural staged 2026-11-25 10:04 (`semio_hub_procedural_component.core.wasm`), served live on 6018 at 11:30-11:45.

## Superseded-refresh hypothesis: refuted
Run a (before fix, boot only, 180 s): every refresh pass has `generation == current` (gen 1..3, `superseded=false`). The guest's two `publish-event os.on-demand.request` arrive in the `surface-visible` turn (+2.6 s) and the pass answers `requestedEffects=[]`, `REFRESH-OWED pending=[]`. Nothing superseded, nothing parked. No dispatch-action / toolRunStart / invoke-extension was ever returned in 180 s (phase stays `faulted flow.extension-not-contributed`, installed=none).

## Real cause
`wireEffectToFriendly` in `PluginRuntime/🪟️retained-ui-patch/🟦️.ts` (the React copy, called by `retainedUiRefreshEffects`) has no `publish-event` case; its `default` returned `null`, silently. The actor-owned decoder (`framework-actor/wire-turn`) has the case, and the WGPU bridge uses that one, so the WGPU TypeScript path never had the defect (Rust applying `publishEvent` not examined).

## Fix
`default` now delegates to the actor-owned decoder (one owner, no per-tag copy). Applies exactly once: the effect is converted once per turn result and flows through the existing owed-effects lane. Law (red first): fixture `🔌️PluginRuntime/🧪️tests/🧫️fixtures/📣️guest-published-events.json`, test "carries every guest published event through the refresh settle ..." in `🧪️tests/📬️extension-invocation-wire/🔖️identity-and-wire-decoding/🟦️.tsx`. Red: four `null`; green: plugin-runtime suite 146/146.

## Live after fix (run b, boot alone, no manual ask)
- +2.6 s guest asks; +3.5 s `ONDEMAND-APPLY` x2; installed brep + math at +10 s.
- Preview `faulted` -> `computing` at ~+45 s; guest returns `dispatch-action` turns (+15 s, +43 s, +70 s) and `invoke-extension` x2 (+130 s); units 4/7 at +140 s.
- Not done: still `computing 4/7`, `cancellable=false`, at +170 s; no mesh finish observed. Next: where the extension invocation after +130 s ends (brep component).

## Addendum: the invoke path, the time, cancellable (runs f, g; procedural staged 10:04, brep/math components 24 Nov 15:04-15:38)
Probe `effect-loss-probe.mjs` (traces: EXT-*, GUEST-REFRESH, SETCONTRIB, CROSSING) and `effect-loss-contrib-probe.mjs`; outputs `🗑️generated/effect-loss/{f,g}-{log,net,console}.txt`, `contrib-rows.txt`. Runs c-e were void: my own BUILDPACK trace broke the publisher ("Cannot read properties of null"), removed.

### (1) invoke-extension, first missing hop (boot alone, performance.now ms, run f)
- 129.9 s: two `invoke-extension` effects returned (math, brep), both resolved to a loaded program. Download is not a hop: module+bridge+wasm requested only at first invoke (math 130.05 s, 43.8 MB, 0.1 s; brep 144.1 s, 60.6 MB, 0.14 s; instance birth < 0.5 s).
- math: invoke starts 129.9 s, guest answers a FAULT in 0.45 s: `invariantViolated registry admission requires its original source slot and caller` (source: neural registry admission, in the math component).
- brep: invoke only starts 144.0 s (serialized per requester actor behind math's completion, see below), then the request actor answers nothing in 64 of 64 turns (`extension.request-unanswered`, 0.3-4.4 s). So the first missing hop is the extension's `respond`, in both components; nothing is missing before it (door, manifest, download, birth, call all happen).
- Swallowed rejection: `applyInvokeExtensionEffect` caught every non-retired error with no log. It now `console.error`s (fault outcomes never throw there, so none was swallowed in these runs).
- Rival (stale 24 Nov components fault): NOT refuted, not provable here. Both extension components predate the procedural 10:04 build and no newer brep/math component exists to compare; the math fault text is a guest-internal invariant. Needs a rebuilt math+brep component (cargo, not run). Evidence to decide: after rebuild, same boot, does `EXT-INVOKE-FAULT` vanish.

### (2) time, per segment (run f; g within 1 s)
| s | segment | cost |
|---|---|---|
| 0-1.8 | module load, 197 MB procedural wasm fetch (cached) | 0.55 s fetch |
| 2.5-3.4 | first refresh; guest asks | 0.36 s |
| 3.4-4.3 | activation (manifests 740+125 KB) + refresh 2 | 0.04 s + 0.8 s |
| 4.3-7.1 | `setContributions` 425 209 B | 2.88 s; the manifests carry two rows per extension (apps `flow-play` and `procedural3d-play`), so half the pack (about 1.4 s) is rows for a sibling app (not a bug, the kernel cut is topic-only on purpose) |
| 7.1-42.6 | refresh 3 settle holds the actor 38.3 s; `toolRunStart` returned at 14.4 s, applied at 42.7 s | 28 s hold |
| 42.7-129.9 | refreshes 4..9 of 0.1-56 s (flowEvalTick turns), guest `more-work` slices of 65 ms | ~100% of wall inside refresh settles (sum 220 s of 230 s, 16 of 20 refreshes > 1 s) |
| 131-144 | math completion delivery waits behind guest refresh | 13.3 s, again 13.3 s for brep |
Census: 1147 empty `(none)` crossings (about 75 s of 65 ms guest slices), 375 command-page crossings, 23 surface-visible.
Largest cost: guest compute in 65 ms slices before the first `invoke-extension` (about 75 s of the 130 s): Rust, not TypeScript. Largest TypeScript-side cost: the refresh settle drains the guest to quiescence (`drainOperations`), so a returned `toolRunStart` waits 28 s and each extension completion 13 s. Not changed: yielding the settle on a returned host effect would not shorten guest compute and changes the settle law; it needs its own fixture.
Side finding fixed: my decoder delegation made the refresh warn on every host-internal `typed-operation-*` pseudo effect; they now drop silently (fixture row `hostInternalTags`).

### (3) cancellable=false while computing
A requirement bug. `cancellable` is true only when the guest holds a `ToolRunView` of the preview tool in Starting/Running/Paused and the chain is unsettled (`📈️status-json/🦀️.rs: preview_progress_status_json_for`). Live: after `toolRunStart` was applied (42.7 s) the guest ran `flowEvalTick`s, yet every 10 s sample for 190 s read `cancellable=false` (units 4/7). So either no run view reaches the status, or it is not preview-tool Starting/Running. Rust, not decidable from TypeScript; the `[DEBUG]` procedural rebuild is not staged yet (component still 10:04), so the guard that returns none is unnamed.

## Addendum 2: stale-deadline candidate for brep (source + g-log crossing timings, no browser)
- Deadline owner: `OriginalOperationSource` (`🎭️actor/🎟️retained-turn/🟦️.ts`): `deadlineUs = clock() + admission.maximumElapsedUs`, fixed at construction (one per actor activation, `component-bridge-source` `createActorApi`), never refreshed per turn (`issue()` copies it; `succeed()` keeps "clock and deadline"). Every `poll` hands the guest `deadlineUs: turn.deadlineUs`; fuel is the Source's remaining fuel; grant is its remaining retained grant. `msg.budget.wallMs` only bounds the fuel clamp, not the deadline.
- Value: plugin actors are admitted from `🖼️canvas/🧵️renderer-work/🔣️stock.json`: `maximumElapsedUs = 31536000000000` (365 days). A past deadline is also not silent: `requireStock` throws `plugin.original-stock-exhausted (maximumElapsedUs)` before every poll.
- Timing of the queue: the 14 s wait for math is in `serializePerActor`, before `ensureRequestActor`, so brep's actor (and Source) is created at the start of its own invoke (144.1 s), 0.2 s before its turns. Stale is impossible.
- `INBOUND_REQUEST_TURN_BUDGET` (64) counts submitted turns that return `more-work`, with or without work: yes, an idle turn counts (`🎭️actor/🖼️wire-turn/📥️inbound-request`).
- g-log crossings of brep (144.601 request, then 64 empty turns): 144.606-144.610 about 1 ms each, one 49 ms turn, then 12 turns in 7 ms (0-1 ms each), total 0.39 s. Procedural slices are 65 ms of real work. So near-zero crossings are real, but they come from a guest that answers `more-work` at once with no effect and no patch (`eff=0`, no `spawn-job`), not from a host deadline.
- Verdict: stale deadline REFUTED. Supported: the brep guest parks the request (or its handler is gated on something the host never supplies) and spins `more-work` without work. Next probe: log per-poll `driveStopped`/`drivePolls` for the brep actor (worker timings) and the guest fault text of the request actor. No fix made; judging "unanswered" by worked turns would only change the message, not the answer.

## Addendum 3: extension refresh owner, run h, per-actor drive input
- Run h (boot only, memory > 30%): 6018 serves extensions from `prototype/runtime/react/generation3d/extensions/*` (installed 24 Nov 15:49), NOT from `prototype/modules/*` where math was staged 14:36. The math fault `registry admission requires its original source slot and caller` therefore repeated: the old component ran (served size 43 848 983 B, new 43 604 537 B). Preview: faulted -> computing at +50 s, 4/7 at +151 s, cancellable=false throughout.
- Owner: dev activation `♻️activation/📥️installation` (`publishActivatedExtension`) writes the install record with `packageHash = artifactSha256` (content identity of the staged module files) and stages it atomically (`stageRepositoryArtifacts`); the activation receipt only runs on `activate <variant> <renderer> <profile>`. So it is the missing dev activation step, not a defect: the copy is refreshed by content digest whenever that step runs; nothing refreshes it without the step by design (no watcher). Command: `repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype`. The graph-free twin `wgpu-native-activate.ts` only publishes the receipt, it does not copy extensions. The canonical run needs more than 7 min just to read the graph slice (first attempt cut at 420 s); started again with a 1700 s cap, log `🗑️generated/effect-loss/activate-react2.txt`.
- Worker drive input (source finding verified): the shard worker serves procedural#1 AND the extension request actors (`flow-extension-*#request`) in one worker; `hostInputSeq` was bumped by every message for any actor, so brep's drive stopped with `input` after one pump whenever procedural traffic (surface-visible, patch-ack, typed-op-ack) arrived, h log: brep polls=1 stopped=input, math (no concurrent traffic) 511 polls. Fix: `//#region drive-input-scope` in `🎬️actor-state-and-lanes`: per-actor counters plus a shared counter for unaddressed messages; the drive reads `hostInputMoved(inputMark, actorId)`; the step ceiling (`drivePolls + 1 >= driveSteps`) is untouched, so no actor starves, and each drive still yields a macrotask per step. Law: fixture `⚛️reactor/🧪️tests/🧫️fixtures/🚦️drive-input-scope.json` read by `📺️renderer/🧑‍🎨engine/🧪️tests/🚚️more-work-drive` (red 3 failures, green 33/33).
- Rebuild: the served worker is `prototype/modules/🧵️shard/🟨️shard-worker.js` (04:18, 44 KB pure JS generated from `shardWorkerSource()`), so the shard `:support` bundle must be regenerated; it is a TypeScript materialization, I expect no cargo, main to confirm the target. Live proof pending that regeneration (serve still reads the 04:18 file).
