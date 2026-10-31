# 1030 audit: wit (read-only)

STATUS: DONE (read-only, no edits, no builds). Rust guest exports and the native hosts match the reactor WIT. The TS browser hosts do not: they lower the WIT wrongly (admit/close args, retained-turn-input.work, poll budget, jobs args) and the document browser actor path calls the raw jco component with a stale budget. Expect the browser pane to fail on its first guest turn (not run here).

Paths below: P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, M = `🧰️framework/🛍️products/💻️os/🔨️modules`.

## 1. WIT surface (reactor and world actor)

- `P/🧬️schema/📜️reactor.wit` removed reserve/write/close-poll-input and the records `actor-poll-input-call`, `poll-input-page`. Repo-wide search (excluding tickets, dist, node_modules, target) for every spelling finds no remaining reference. The only `poll_input` hits left are the live owned ABI (`P/📣️macros/⚛️wp4-reactor/🦀️.rs:259` `take_actor_poll_input`, `P/🧬️schema/🔁️persistent-local/🦀️.rs:187`), which is `semio_owned_poll_v1` and not part of the removal.
- `world actor` (`P/🧬️schema/📜️.wit:18-26`) exports reactor, jobs, checkpoint, describe, codec. Its imports are `pure` and `host-async`.
- Consistent (no mismatch found):
  - reactor: stage-command-page, stage-cold-pair-page, admit-actor-context, close-actor-context, poll, channel-version. Rust `__semio_actor_exports!` (`P/📣️macros/⚛️wp4-reactor/🦀️.rs:12-37`) implements the same six. TS descriptor fixture `P/🌐️browser-bundle/🛂️descriptor/🧪️tests/🧫️fixtures/🔣️.json` `expected.reactor` lists the same six.
  - jobs (7), checkpoint (2), describe (1), codec (12): names match between `.wit`, the Rust macro (`📣️macros/⚛️wp4-reactor/🦀️.rs:40-214`) and the fixture.
- The other `.wit` files (`jcoprobe` `world.wit`, `browser-bundle/🧪️testing/🌊️actor-import` `world.wit`) are standalone probe worlds with no reactor export. Not affected.

## 2. One event per poll (Rust guests)

- `P/⚛️reactor/🧬️wit-bridge/🪢️bridge/🦀️.rs:84-90`: `poll` refuses a second event with `a reactor turn admits at most one event`. `poll_wit` (`P/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs:43-46`) delegates to this guard. `poll_owned` (same file, line ~53+) uses `single_kernel_event` (line 24). Both entry points are guarded.
- Duplicate stage guards exist: `stage_command_page` (`🧬️wit-bridge/🪢️bridge/🦀️.rs:48-58`) and `stage_cold_pair_page` (line 62+) fault on a second staged page.
- Native hosts, verified:
  - wasmtime `execute_turn` (`P/🖥️host/🎭️guest-runtime/🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs:198-211`): stage command, stage cold pair, then `call_poll(wit_events, wit_budget)`. The budget includes retained, retained_policy, work_fuel, deadline_us. Matches WIT.
  - native shard loop (`P/🖥️host/🧵️shard/🎬️shard-loop-frame-turns-and-job-control/🦀️.rs:96`): `from_ref(event)` on the first turn, `&[]` afterwards.
- No mismatch found on the Rust side.

## 3. Mismatches: TS browser host

### 3.1 Admit/close lowering (HIGH, runtime-breaking)

- `P/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts:154`: `reactor[method](originalTurnWit(turn.input))` passes one argument. WIT `admit-actor-context` and `close-actor-context` take `(input, retained-policy)` (`reactor.wit`, the two `async func` lines after `channel-version`'s neighbours). The retained-policy argument is missing.
- `M/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts:60` (`originalTurnWit`): emits `{operation, generation, epoch, grant}`. WIT `retained-turn-input` also requires `work: original-actor-turn-work {clock-binding, absolute-deadline-us, fuel}`. The `work` record is never produced. The same function is used for `poll` at bridge line 196, so the omission reaches every turn.
- Note: `🌉️component-bridge-source/🟦️.ts:155`: `receiveOriginal(call, 0n)` ignores `call.workUnitsSpent` from the admit/close records. WIT provides `work-units-spent` on both. MEDIUM.

### 3.2 Poll budget in the bridge (HIGH)

- `🌉️component-bridge-source/🟦️.ts:196`: poll budget is `{retained, fuel, deadlineUs, maxEffects, maxPatchBytes, maxFrames}`. WIT `budget` (`reactor.wit`) also requires `retained-policy` and `work-fuel`. Both are missing.

### 3.3 Jobs lowering in the bridge (HIGH)

- `🌉️component-bridge-source/🟦️.ts:213`: `startJob(job, kind, input)` calls `jobs.startJob(job, kind, input)`. WIT `start-job(source: option<job-source>, budget: job-budget)`. Two args, wrong shape: `job-source` not built and budget missing.
- `:215`: `cancelJob(job)` calls `jobs.cancelJob(job)`. WIT `cancel-job(job, budget)`. Budget missing.
- `:214`: `stepJob(job, budget)` forwards whatever budget arrives. Upstream it is `{fuel, deadlineMs}` (see 4.2), not WIT `job-budget {input, retained-policy, fuel, deadline-us, cancelled}`.

### 3.4 Document browser actor path calls the raw jco component (HIGH)

- `M/🏪️store/👷️worker/🧷️actor-reservation/🛤️poll-lanes/🟦️.ts:24`: `child.invoke(["reactor","poll"], [events, browserActorTurnBudget()])`. Stage calls at lines 22-23 use the same route.
- `M/🏪️store/👷️worker/🧩️actor-wire-helpers/🟦️.ts:182-184`: `browserActorTurnBudget()` returns `{fuel, deadlineMs, maxEffects, maxPatchBytes, maxFrames}`. Uses the pre-WIT names (`deadlineMs`, not `deadline-us`) and has no `retained`, `retained-policy`, `work-fuel`. Required by WIT `budget`.
- Route: `reserveBrowserActorChild` (`P/🌐️browser-bundle/🧵️child/🟦️.ts:16-25`) → child worker `invoke` → `P/🌐️browser-bundle/📜️script.ts:683-697` (`invoke`) → `owner[name](...args)` on the jco `component` from `instantiateFreshComponent` (`script.ts:655`). So `reactor.poll(events, budget)` is the raw jco export. It bypasses the bridge's custody (`originalSource`, `admitContext`, producer-held checks) and the admit/close steps.
- Consequence: the raw lift of `budget` fails on the missing fields. Also the document path never calls `admitActorContext`. Verify at runtime whether the guest needs admission first (the bridge does it; the guest's `poll_wit` admits internally, so this may be benign).
- Jobs on this route: `M/🏪️store/👷️worker/🧷️actor-reservation/🛤️poll-lanes/🟦️.ts:52-53`: `startJob` passes `[job, kind, input]`; `stepJob` passes `{fuel, deadlineMs}`. Same WIT mismatch as 3.3. `invokeJobs` (line 36) calls `child.invoke(["jobs", name], args)`, so `cancelJob` also lacks budget there.

### 3.5 Shard worker frame path (MEDIUM)

- `P/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/📨️message-handler/🟦️.ts:96,134`: `actor.api.poll` calls are shaped correctly for the bridge (4-arg). The frame case (lines 231-234) now returns an array of turn results for multi-envelope frames.
- `M/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:155` (`ShardClient.grant`) returns `Promise<unknown>`. The single-vs-array reply contract is not typed or tested anywhere (`grep` finds no production consumer). LOW-MEDIUM.

## 4. Stale or wrong TS consumers

### 4.1 Generated jco glue (gitignored, stale)

- `M/🧑‍💻dev/🔌️plugin-modules/**` and `M/🧑‍💻dev/🧩️extension-modules/**`: 60 `interfaces/semio-framework-reactor.d.ts` have `Budget {fuel, deadlineMs, maxEffects, maxPatchBytes, maxFrames}` and no `admitActorContext`, `closeActorContext`, `channelVersion`. 82 `🌉️bridge.js` copies use `deadlineMs: budget.wallMs` and the old no-custody poll (e.g. `plugin-modules/🧮️flow-extension-math/🌉️bridge.js:147-150`). Owner: the materialization build that writes these (not hand-edited). Action: regenerate; do not edit by hand.

### 4.2 Test fakes and tests asserting old behaviour

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/🪤️cold-pair-worker-fake/🟦️.ts:116`: on a panel refresh the fake expects `events` to equal `[...panelVisible, ...sectionVisible]` in one poll. Line 141 expects `events.slice(1)` to be a multi-event batch. Both contradict the one-event law and the new `renderSurface`/`refreshDocumentSurfaces` (one poll per surface event). Owner: GIS variant tests / actor-reservation surface-rendering. Test will fail.
- `M/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧬️jco-rewrites/🟦️.ts:25`: passes the old `{fuel, wallMs, ...}` budget into `api.poll`. Harmless for the bridge (it reads only maxEffects/maxPatchBytes/maxFrames) but stale. LOW.
- `M/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🛂️routes-and-bridge/🟦️.ts:79-91`: the rewritten staging tests check ordering and the one-event refusal, but the fake guest does not check budget fields. This is why 3.1-3.3 pass there. Coverage gap.

## 5. Verified consistent (no action)

- No TS host calls a removed export (reserve/write/close-poll-input). Confirmed by repo-wide search.
- Event batching on TS hosts, one event per poll:
  - `M/🏪️store/👷️worker/🧷️actor-reservation/🛤️poll-lanes/🟦️.ts:17-18`: `invokeWithPages` throws on `events.length > 1` before the child call.
  - surface-rendering `renderSurface` (`🖼️surface-rendering/🟦️.ts:~110`): `owed.splice(0,1)` then wake.
  - `refreshDocumentSurfaces` (line ~157): `[event]` per poll.
  - ui-patch-reconciliation: `owed.splice(0,1)` (lines 106, 143); wake-only poll at line 174.
  - document-backbone, guest-activation, guest-retirement, cold-pair-transfer, action-dispatch, linked-read: each literal array has one element.
  - shard worker `turn` path (`message-handler/🟦️.ts:96`) is one poll, and the bridge refuses >1.
  - `TurnScheduler` / `BoundedMailbox` `coalesce` is latest-wins replacement, not batching (`M/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts:120-128`). It does not create multi-event turns.
- Jobs `takeSegmentedDownloadChunk` and `checkpoint`/`restore`/`describe` in the bridge match WIT.
- The browser test `🧪️ticket-owned-browser-host-staging/🛂️routes-and-bridge/🟦️.ts` asserts stage-before-poll order and the one-event refusal (correct), but none of the budget or lowering mismatches above.

## 6. Gaps

- No language-agnostic fixture test that a 2-event `poll` is refused by both guest and host (1029 follow-up still open).
- No test asserts the WIT budget or retained-turn-input shape at the TS lowering boundary (the source of 3.1-3.2).

## Owners (suggested)

- `component-bridge-source/🟦️.ts` (3.1-3.3, 4.2 bridge tests): browser materialization owner.
- `originalTurnWit` (3.1): actor retained-turn owner (`M/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`).
- `poll-lanes`, `actor-wire-helpers` (3.4): store/worker actor-reservation owner.
- `script.ts` `invoke` route (3.4): browser-bundle owner.
- GIS cold-pair fake (4.2): GIS variant test owner.
- jco glue (4.1): materialization build (regenerate).
