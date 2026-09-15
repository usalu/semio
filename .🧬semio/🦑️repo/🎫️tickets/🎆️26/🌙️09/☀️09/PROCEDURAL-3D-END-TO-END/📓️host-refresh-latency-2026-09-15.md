# 🚑️ Host refresh latency — the ingress drain, the hot-swap retirement, and the settle that held the actor

Lane `host-refresh-latency`, 2026-09-15, React door of 🧊️generation3d on **:6021**.
Predecessor with this name died to the 14:10 usage limit before landing anything; this is the whole lane.

---

## 0. What this lane owns, and what it landed

| # | item | state |
|---|---|---|
| 0 | the peer's tool-run progress starvation (`settlePluginTurn` owed hold + dropped progress frames) | **fixed, 2 laws, served after-reading on the peer's own probe — §1.3** |
| 1 | `command ingress did not complete within 1024 continuations` | **root-caused, fixed on two layers, 4 laws, served — §5.6** |
| 2 | the hot-swap `no channel` fault | **root-caused, fixed at two sites, 2 laws, served 0/0 — §3, §5.6** |
| 3a | `settlePluginTurn` latency with an empty required set | **measured — already ~1 crossing per settle; no defect** |
| 3b | the owed host-side `refreshUi full` after every pick | **hardcode removed, 1 law; served, the pick's scopes went `full 4, partial 2` → `partial 3, full 1` — §5.6** |
| 4 | crossings/hop ≤ 15, slider → preview ≤ 150 ms | **NOT met, measured and attributed — §5.1, §5.4, §5.6** |

**Served.** The reactor half reached the browser at 16:57 after lane `procedural-rename-fixforward`
completed a peer's rename that had failed nine restage attempts (three of this lane's, six of the
coordinator's — §6). §5.6 is the reading with the whole change in it: **0 `1024 continuations`, 0
`no channel`, 0 `plugin.command-page-unowned`, 0 `command-ingress-unowned`/`-stalled`, 0 page errors** across
three probes and ~13 minutes of session, with every command settling in **0 continuations**. §5.1–§5.4 are
kept as the host-TypeScript-only reading, because the pair is what separates the host half from the whole.

---

## 1. Item 0 — a refresh settle held the actor for a whole tool run, and ate its progress frames

Handed to this lane at 15:30 by the coordinator, from peer session `INTERACTIVE-TOOLS-VISIBLE-PROCESS`
(release :6013, 100-object fill, `🔍️w5-fill-timeline-probe.ts --settle=20000`): progress frames reach
`AppChannelClient.publishOperationProgress` only until +442 ms and never again, the run completes at
~2.1 s, **3 refreshes instead of 12**, one `refresh.turn` lasting **1 281 ms**, and the scene stays at
36–40 of 100 provisional objects **20 s after completion**.

Two independent causes, both in `🔌️PluginRuntime/🟦️.tsx`, both now fixed.

### 1.1 The owed clause had no wall

```ts
// before
function settleYieldsToRefreshV1(results, startedMs, nowMs, acknowledgementsOwed, outstanding, owed = false) {
  return !acknowledgementsOwed && !outstanding && !owed && nowMs - startedMs >= PLUGIN_OPERATION_REFRESH_SLICE_MS && …;
}
```

`owed` is `settleOwesRequestedSurfacesV1`, which answers "a surface I asked for has not published AND the
guest says `more-work`". **It cannot tell "the guest is still reconciling MY surface" from "the guest is in
`more-work` for something else entirely"** — and a live tool run keeps the guest in `more-work` for the
whole run. So `!owed` was false for the run's whole length and the settle never sliced: one refresh owned
the actor's ingress from the first frame to the last, which is exactly the 1 281 ms `refresh.turn` and the
3-instead-of-12 refresh count.

The clause itself is right (it fixed the opposite defect: a settle slicing out one crossing in and leaving
its own patch to a later unrelated drain turn). What it lacked is a **wall**:

```ts
const PLUGIN_OPERATION_OWED_SLICE_MS = DEFAULT_SHARD_BUDGET.wallMs;   // 100 ms
…
  return nowMs - startedMs >= (owed ? PLUGIN_OPERATION_OWED_SLICE_MS : PLUGIN_OPERATION_REFRESH_SLICE_MS) && …;
```

The grace is **derived, not chosen**: `DEFAULT_SHARD_BUDGET.wallMs` is the wall the host grants ONE
crossing — the same wall the worker's own `MoreWork` drive spends before it crosses back — so it is the
longest a guest legitimately takes to reach its next publication opportunity. Past it the refresh lane is
let back in and re-asks, which is what a coalescing lane is for. An acknowledgement still owed, or a
REQUIRED surface that has not published, still suppresses the yield unconditionally (the settle would
otherwise throw).

### 1.2 A refresh settle decoded every progress frame and threw it away

```ts
// 🧰️framework/…/🔌️PluginRuntime/🟦️.tsx, retainedUiRefreshEffects — before
    const bytes = shellFrameBytes(effect, instanceId);
    if (bytes) {
      const frame = decodeAppFrame(bytes);
      if ("Error" in frame) { …throw… }
    } else { …requestedEffects.push… }
```

There is no `else` after the `Error` branch: **every non-`Error` shell frame a refresh settle produced was
decoded and dropped on the floor.** A long typed operation publishes its process as unsolicited
`Invocation` frames (`in_reply_to: 0`, carrying the dirty scope), and a refresh settle that runs DURING
that operation is precisely where those frames land — so once the first refresh took the actor, the run's
progress never reached the channel again. That is the peer's "+442 ms then never again", exactly.

`retainedUiRefreshEffects` now takes a `publishShellFrames` sink (default no-op) and `ownedUiRefreshResponse`
passes `(frames) => turnOutcomes.push({ instanceId, frames })` — the same broadcast `runQueuedTurn` uses.
A frame is the channel's to route, whichever settle produced it.

### 1.3 After-reading on the peer's own probe, :6013

:6013 was up after all and serves this host module from source (verified: the served
`🔌️PluginRuntime/🟦️.tsx` carries `PLUGIN_OPERATION_OWED_SLICE_MS` and `publishShellFrames`). The peer's
probe was run **read-only** — nothing in their ticket was edited; the probe wrote its own output where it
always writes it, `🗑️generated/W5-mac-react-e2e/timeline-100-1789481207402.txt`:

```
cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS
bun 🔍️w5-fill-timeline-probe.ts --count=100 --settle=20000

count=100 frames=1315 firstRecordAt=+873ms distinctRecordCounts=6 largestJump=13 finalRecords=47
+24ms    records=0  … provisional=0
+873ms   records=5  … provisional=40
+2042ms  records=18 … provisional=64
+3160ms  records=24 … provisional=75
+3789ms  records=36 … provisional=89
+4712ms  records=47 … provisional=100
```

| the peer's measurement | before (their report) | after (this run) |
|---|---|---|
| provisional instances at the end | **36–40 of 100**, 20 s after completion | **100 of 100**, at +4 712 ms |
| refreshes during the run | 3 | **12** — the pre-regression count |
| longest `refresh.turn` | 1 281 ms | **802 ms** |
| refresh stop reasons | — | **10 `sliced`, 2 `idle`** (the settle yields instead of holding) |
| progress updates reaching the renderer | stop at +442 ms | **6 distinct record counts across the whole run** |

One span says the whole thing:

```
17654 refresh.turn 149ms {"instanceId":1,"events":1,"requested":1,"missing":0,"stop":"sliced","continuations":1,"patches":1,"owedAtStop":false}
```

— a refresh that owed a surface now ends `sliced` after **one** continuation instead of holding the actor
for the run. Every settle of that run, by stop reason and crossings spent:

```
2 × stop=idle   continuations=0
3 × stop=sliced continuations=1
1 × stop=sliced continuations=2
1 × stop=sliced continuations=3
2 × stop=sliced continuations=4
1 × stop=sliced continuations=7
2 × stop=sliced continuations=9
owedAtStop: 6 false, 6 true
```

**Six of the twelve settles stopped while still owing a surface** — that is the new grace expiring and
handing the actor back, with the lane re-asking, which is precisely the behaviour the wall was added for.
The 802 ms residue is a different cost (one `patch.install` of 434 ms in the same run) and is not this
lane's.

---

## 2. Item 1 — `command ingress did not complete within 1024 continuations`

### 2.1 Reproduction, from the fleet's own captures

The fault did not reproduce on a fresh :6021 boot (`🗑️generated/host-refresh/boot`, 120 s) nor in any of
this lane's four probe runs. It is fully captured in a peer's console — `🗑️generated/react-gen-wire/flow-wire/console.txt`,
:6026, 2026-09-15 10:30, an `interactionHover` (716 bytes, **1 page**):

```
77188 [DEBUG] command ingress crossed {"actionId":"interactionHover","bytes":716,"pages":1,"ms":223}
77202 [DEBUG] command ingress continuation 32   status=idle turn=idle
77217 [DEBUG] command ingress continuation 64   status=idle turn=idle
…
77540 [DEBUG] command ingress continuation 1024 status=idle turn=idle
77540 [DEBUG] command ingress settled status=idle observed=idle
77540 error   [DEBUG] action failed interactionHover … command ingress did not complete within 1024 continuations (observed statuses: idle)
```

**1 024 host crossings in 352 ms, every one answering `idle`, on an actor that is itself `idle`.** The
cost is not the crossings — it is that the diagnosis is the number 1 024 and the word `idle`, which names
no cause at all.

### 2.2 Root cause A (reactor) — the page-admission chain has no terminal arm

`⚛️reactor/🔄️turn/🦀️.rs`, the `if let Some((cursor, page)) = command_page { … }` block, is an `else if`
chain with **no final `else`**:

| branch | condition |
|---|---|
| Fault | the page fails validation |
| Backpressure | a retained owner holds a DIFFERENT cursor |
| CommandPending | the retained owner is `ReservedPresence`/`PendingPresencePage` |
| presence | `cursor.kind == 28`, with two inner arms of its own |
| generic | `page_index == 0 && retained.is_none()` |
| assembly | `if let Some(GenericAssembly{..}) = retained.take()` |
| — | *(nothing)* |

Three shapes fall out of it and are **silently dropped, with the turn answering `Idle`**:

1. `kind != 28`, `page_index > 0`, no retained assembly — a continuation page of a command whose assembly
   is gone;
2. `kind != 28`, `page_index == 0`, retained is the SAME cursor but not a `GenericAssembly` — and worse,
   `retained.take()` in the last arm **takes the owner unconditionally and drops it when the pattern does
   not match**, so the retained state is destroyed along with the page, which then puts every later page of
   that command into shape 1;
3. `kind == 28` (presence), `page_index > 0`, retained is not `Presence`.

A dropped page is indistinguishable from a page still being worked on: both are `Idle`. That is the
`observed statuses: idle` the host printed 1 024 times.

**Fixed:**

- the assembly arm is taken only when it IS the assembly this page belongs to, so no other owner shape is
  taken-and-dropped;
- a terminal `else` raises a NAMED fault, `plugin.command-page-unowned`;
- one total rule, `command_ingress_while_owned(status, owner_cursor)`: **a turn that still owns an ingress
  never answers `Idle`** — an owner that advanced nothing this turn answers `CommandPending(cursor)`.
  Without that rule the `Presence` publication drain legitimately reports `Idle` for many turns, which is
  what made `Idle` ambiguous in the first place.

After this, `idle` from the host's side means exactly one thing: **no owner holds this command.**

### 2.3 Root cause B (host) — a magic 1 024 with no progress rule, and a terminal read past

Two defects in the drain in `🔌️PluginRuntime/🟦️.tsx`:

- **the bound**: a literal `1_024`, derived from nothing. A maximal 67-page command happens to land near it
  (`67 × 2 × 8 = 1 072`), which is why it looked adequate; a 1-page hover got the same 1 024.
- **the terminal read**: the loop read `continued.commandIngress?.tag` — the turn IT submitted — while
  `acceptTurn` submits turns of its own (the batched `issued-ui-acks` lifecycle turn reshaped in
  `700854a9ab`). A `command-complete` landing on one of those supplemental turns was **read straight past**,
  after which every further continuation reports `idle` forever. This is a second, independent way to reach
  the same 1 024-crossing spin, and it is the one whose onset matches the fault's 09:53 first appearance.

**Fixed:**

```ts
export function commandIngressContinuationCeilingV1(pages: number): number {
  return Math.max(1, pages) * COMMAND_INGRESS_TURNS_PER_PAGE * PLUGIN_UI_CONTINUATION_BATCH_SIZE;
}
export function commandIngressUnownedV1(ingressTag: string | undefined, turnStatusTag: string | undefined): boolean {
  return (ingressTag ?? "idle") === "idle" && turnStatusTag !== "more-work";
}
```

- `COMMAND_INGRESS_TURNS_PER_PAGE = 2` is read off the reactor's own chain: the page is admitted on the turn
  it arrives (`page-accepted`) and the assembled command is exchanged on the next (`command-pending` →
  `command-complete`); one whole `PLUGIN_UI_CONTINUATION_BATCH_SIZE` of slack per page covers the worker
  drive's cadence. 1 page → **16**, 67 pages → **1 072**.
- the ceiling is the **liveness backstop**; the **progress guarantee** is `commandIngressUnownedV1`, and it
  costs **zero further crossings** where the old shape cost 1 024.
- the terminal is now folded over EVERY turn the drain collected, supplemental ones included.
- the fault names a cause: `plugin.command-ingress-unowned` ("the reactor retains no ingress owner for this
  command") or `plugin.command-ingress-stalled` ("still owns it and never completed it"), each carrying the
  action id, the seq, the page count, the continuations spent over the ceiling, and the observed statuses.

**Law, in one sentence:** *a command completes, or faults with a named cause, within a bound derived from
its own page count — and an unowned command is named on the first turn, not on the thousandth.*

### 2.4 Measured on the live door

Over the 55 s gate run (`🗑️generated/host-refresh/after`), **every** command settled in **0** continuations:

```
68 × command ingress settled status=command-complete continuations=0/16
 1 × command ingress settled status=command-complete continuations=0/1072   ← the 67-page setContributions
```

and over the 10.6-minute journey, 124 × `command-complete continuations=0`. The drain's continuation loop
is never entered on the healthy path at all — which is exactly why a bound with no progress rule behind it
went unnoticed for so long.

---

## 3. Item 2 — the hot-swap `no channel`

`adaptPluginHandle` owns a `retiredInstances: Set<number>` **inside its own closure**. `destroyApp` adds to
it and `requireChannel` reads it, so a late gesture against a destroyed instance is a typed
`isPluginInstanceRetiredV1` refusal the shell drops quietly — *for as long as that handle lives*. A hot swap
replaces the handle, and **the replacement's ledger is empty**: every lane still addressed to the destroyed
instance gets an UNMARKED `no channel for instance N`, which `dropForRetiredInstance` cannot recognise.

Measured, :6023, `🗑️generated/react-sweep2/panel-i18n/console.txt`:

```
70602 … at Object.destroyApp (…PluginRuntime…)          ← the swap tears instance 1 down
75181 error [os-shell] remounted window refresh failed  Error: … no actor for instance 1
75198 pageerror                                          Error: … no actor for instance 1     ← unhandled
77715 error [DEBUG] typed-operation completion subscription failed Error: … no channel for instance 1
77720 error [DEBUG] typed-operation progress subscription failed  Error: … no channel for instance 1
```

**Fixed** by moving the ledger to module scope, keyed `pluginId#instanceId` — instance ids are minted per
plugin and never reused, so the plugin-qualified id is the honest key and the answer survives the handle
that gave it. `createApp` un-says a retirement for an id it hands out again. The existing law that
"nothing ever created instance 99 — that is a bug, not a teardown race" is preserved and re-asserted in the
new law. The remounted-window refresh (`ShellHost`, the one that surfaced as an unhandled rejection) now
goes through `dropForRetiredInstance` like its two siblings.

---

## 4. Laws, with output

### 4.1 Rust — `cargo test -p semio-framework-plugin --lib command_ingress_terminal -- --test-threads=1 --nocapture` (foreground)

```
running 5 tests
test …::a_turn_that_still_owns_an_ingress_never_answers_idle ...
[DEBUG] owned ingress turn answers command-pending, unowned answers idle, every stated status is preserved
ok
test …::accepted_final_page_completes_without_a_follow_up_turn ... ok
test …::accepted_final_page_preserves_a_terminal_fault ... ok
test …::async_actor_poll_awaits_exchange_and_render_work ... ok
test …::every_admitted_command_page_leaves_the_turn_with_a_named_status ...
[DEBUG] command page admission: terminal else present, fault named plugin.command-page-unowned, assembly take is shape-guarded
ok

test result: ok. 5 passed; 0 failed; 755 filtered out
```

| law | asserts |
|---|---|
| `a_turn_that_still_owns_an_ingress_never_answers_idle` | **the rule.** `Idle` + an owner ⇒ `CommandPending` at that owner's exact cursor; `Idle` + no owner stays `Idle`; and every already-stated status (`PageAccepted`, `CommandComplete`, `Backpressure`, `Fault`) is preserved untouched. Reads `reactorStatuses.ownedTurnNeverIdle` from the shared fixture. |
| `every_admitted_command_page_leaves_the_turn_with_a_named_status` | the admission chain has a terminal `else`; it names `plugin.command-page-unowned` (the string the TS fixture declares); and the assembly arm no longer takes-and-drops every other owner shape |

`async_actor_poll_awaits_exchange_and_render_work` was **already red at `HEAD`** — a source-text scan for
three call shapes a peer had since boxed and renamed (`plugin_exchange` → `plugin_exchange_boxed`,
`plugin_render` → `plugin_render_surface`). Verified against `git show HEAD:` before touching it, and
repaired in place; it is not a red this lane caused and not one it leaves.

### 4.2 TypeScript — `bun ./📜️script.ts test long "PluginRuntime"` (foreground)

```
 Tests  2 failed | 117 passed (119)
```

Both reds are peer-owned and in files this lane never touched:

- `reads every request-carrying effect out of its nested WIT params record` asserts `documentJson` against a
  runtime that now emits `artifactJson` — the same `document`→`artifact` rename wave as §6. Red since before
  this lane's first edit.
- `keeps the leftover vortex id on an armed brush window` — `ReferenceError: Cannot access
  '__vite_ssr_import_18__' before initialization` at `🛠️ShellHelpers/🟦️.tsx:2102 shellLabel` ←
  `🐚️Shell/🟦️.tsx:78`. `Shell` calls `shellLabel(…)` at module top level and `ShellHelpers` imports `uiI18n`
  back from `Shell`: a cycle, whose evaluation order a peer's 16:29 rework of `ShellHelpers`
  (185 insertions / 199 deletions vs `HEAD`) flipped. It was GREEN in this lane's 15:5x full run and red
  after that edit; the failing frames are module initialisation of two files this lane has not opened.

Six new laws (each also run alone, with its `[DEBUG]` line):

```
[DEBUG] command ingress ceiling: 1 page=16 67 pages=1072 (was a flat 1024 for both)
 ✓ bounds one command's drain by a ceiling derived from its own page count, never by a chosen number

[DEBUG] unowned command ingress: 0 further crossings, ceiling 16 (was 1024 crossings then an unnamed timeout)
 ✓ calls a command unowned on the FIRST idle turn instead of spending a whole ceiling on it

 ✓ remembers a destroyed instance across a hot swap that replaces the handle

[DEBUG] owed settle grace: 100 ms (one crossing wall), unowed slice 16 ms — was unbounded
 ✓ lets an owed settle hold its surface for at most one crossing's wall, then yields to the progress lane

[DEBUG] refresh settle frames: published to the channel, not decoded and discarded
 ✓ hands a refresh settle's unsolicited progress frames to the channel instead of dropping them

[DEBUG] retired instance: handleAction/handleCommand/contextMenu all reject, none throws synchronously
 ✓ refuses a retired instance by REJECTION, never by a synchronous throw past the caller's catch
```

The ceiling and unowned laws are driven by the language-agnostic fixture
`🔌️PluginRuntime/🧫️fixtures/command-ingress-drain.json`, which the Rust laws read too — five ceiling rows
(1, 0, 2, 64 and the live 67-page `setContributions`) each asserted against both the expected number AND
the derivation, and eight `unowned` rows covering every status/actor pair.

### 4.3 TypeScript — `bun ./📜️script.ts test long "engine-contract" -t "owed effects"` (foreground)

```
 Test Files  1 passed (1)
      Tests  1 passed | 625 skipped (626)
[DEBUG] owed-pass scope: pending effects earn nothing, the lane carries the pass's own scope
```

`applies a pass's owed effects under that pass's own scope, never a hardcoded full one` — proves that a
pass's owed `pending_effects` (`dispatchAction flowEvalTick` / `toolRunStart`) earn **no scope of their
own** through `hostEffectRefreshScopeV1`, so `full` was the caller's hardcode and not a derivation; then
reads `ShellHost` and asserts the lane carries `owedEffects.scope` and names no scope of its own.

**The suite would not load at all** before this: it imported
`🛂️manifest/🧫️fixtures/🎞️tutorial-document-track.json`, which a peer had renamed to
`🎞️tutorial-artifact-track.json` — the whole 626-law engine-contract gate was failing to resolve. Repaired
(one import path); every other relative import in that file resolves.

### 4.4 Typecheck

`bun nx run @semio-tech/framework-renderer-react:typecheck` — **826 pre-existing errors** across the tree
(overwhelmingly the in-flight `document`→`artifact` rename). Error line numbers in the two files this lane
edits are `1155 2059 2937 5647 5757 6937 6942 6955 6957 6959 6961 6963 6967 6972 7000` — **not one of them
is a line this lane touched**. No new type error.

---

## 5. Measurements, under load

Fleet load through the whole session: peers rebuilding wasm and running `cargo check` continuously; three
of this lane's own restages ran against it.

> **§5.1–§5.4 below are the host-TypeScript-only reading (guest of 13:04). §5.6 is the SERVED reading, taken
> on the 16:57 guest with the reactor half in it.** Both are kept: the pair is what separates the host half
> from the whole change.

### 5.1 The gate — `🐍️react-hop-cost-probe.mjs` (`🗑️generated/host-refresh/after`, 55.2 s, 10/10 converged)

```
worker crossings per hop: 35.00; hop wall: 2760 ms
RecalcStyleDuration over the run: 840 ms = 1.52% of wall (gate: under 5%)
worker MoreWork drive: 1394 guest turns absorbed over 700 crossings (1.99 per crossing); stops carried=449 idle=215 input=27 budget=9
```

**Target ≤ 15 crossings per hop: NOT met.** 35.00 including boot (170 crossings for 2 hops); 29.4 excluding
boot. The worker-more-work-drive lane's own last reading was 36.2, so this lane's changes moved the
steady-state crossing count essentially not at all — which is correct: they remove FAULT paths, not
crossings. Where the 700 crossings go, attributed:

| posted event kinds | count | per hop | total ms | ui patches |
|---|---:|---:|---:|---:|
| `patch-ack` | 198 | 9.9 | 7 851 | 160 |
| `message` | 182 | 9.1 | 171 | 0 |
| `(none)` | 162 | 8.1 | 1 666 | 42 |
| `surface-visible` | 79 | 4.0 | 5 425 | 53 |
| `request` | 38 | 1.9 | 1 189 | 0 |
| `completed` | 38 | 1.9 | 160 | 0 |

**The 15-crossing target is unreachable without cutting `patch-ack` (9.9/hop) and `message` (9.1/hop).**
`message` is 182 crossings costing 0.9 ms each and carrying **nothing** — one whole crossing per shell
`send-message` instead of riding the next turn's event list. That is the single biggest remaining cut on
this path and it belongs to the shard-client / turn-event layer, not here; this lane names it and does not
claim it. `patch-ack` is the patch-batching lane's floor (`UI_TURN_PATCHES_MAXIMUM`).

### 5.2 Settles with an empty required set — item 3a, no defect

`🐍️host-ingress-latency-probe.mjs` reads the `refresh.turn` span detail `settlePluginTurn` reports:

```
pick: settleStops {"idle":4}  settleContinuations 4  settleSamples 4
```

**Four settles, four continuations — one crossing each, and every one stopped `idle`.** A turn that owes
nothing already settles in one crossing; the 128-continuation quiescent tail
(`PLUGIN_UI_QUIESCENT_CONTINUATIONS`) is never reached on this door. No change made, because there is
nothing here to fix.

### 5.3 The owed `refreshUi full` — item 3b, hardcode removed, reading unchanged

Before (`host-refresh/before`, 14:37) and after (`host-refresh/after-host`, 15:07) on the same door:

| | before | after |
|---|---:|---:|
| `refreshUi lane` `decision:"owed"` | 21 | 21 |
| …of which `{"kind":"full"}` | 5 | 5 |
| …of which `partial` | 16 | 16 |
| pick worker crossings | 54 | 36 |

The hardcode IS gone and the law proves it, but **the live number did not move**, and the reason is worth
stating precisely rather than dressing up: in these runs the passes that owed effects were themselves
`{kind:"full"}` passes, so carrying the pass's own scope is still `full` — correctly so. The five full owed
requests are boot passes (`passes: 0`–`2`).

What actually keeps the pick's passes full is a **different** owner: `resolveUiDirtyScope(undefined) →
{kind:"full"}`. A guest response that declares no `uiScope` at all refreshes everything, and the pick-time
`applyHostEffects refresh {"declared":{"kind":"full"},…,"viewStateSame":true}` bursts (five at one
millisecond) are exactly those. Narrowing that default would be under-refreshing; the fix is a guest that
declares its scope. **Named, not claimed** — it is the same seam
`📓️interaction-scope-narrowing-2026-09-15.md` §5.3 could not find an owner for, and this lane has found the
owner but not the fix.

### 5.4 Slider → preview — item 4, host is not the bottleneck

`🗑️generated/host-refresh/after-host`: the node-graph inline sliders are 29 px knobs, so a pointer drag
across one lands as a hover; the `role="slider"` keyboard contract is the reliable value change and
dispatches the same command. Six `ArrowRight` presses moved `Column Height` **3 → 3.6** and

```
drag {"found":1,"valueBefore":"3","valueAfter":"3.6","firstUpdateMs":null,"releaseToIdleMs":3}
```

**no preview update at all** within 5 s — `firstUpdateMs: null`. The command dispatches, the host settles it
in one crossing, and nothing re-evaluates. That is the guest-side coalescing lane
`slider-preview-update` owns; §1's dropped-progress-frame fix is very plausibly part of the same symptom
(the coordinator's own hypothesis) but this lane cannot show that until the run re-evaluates at all.

Six `pageerror ReferenceError: sliderLane is not defined` fired during that interaction — a peer's
in-flight edit on the served tree, not this lane's, and gone from the two later runs.

### 5.5 The 10-minute session gate — `🐍️journey-probe.mjs` (`🗑️generated/host-refresh/journey`, 638 s)

```
DONE steps 23 meshSteps 20
```

| counted over the whole 10.6-minute session | |
|---|---:|
| `did not complete within … continuations` | **0** |
| `no channel` | **0** |
| `plugin.command-ingress-unowned` / `-stalled` | **0** |
| page errors | **0** |
| steps with `meshOk: false` | **0** |
| steps with a non-empty `meshReasons` | **0** |
| steps with a non-empty `faults` | **0** |
| `command ingress settled status=command-complete continuations=0` | **124** |

**Every mesh oracle green, 0 faults, 0 of either named fault in ten minutes.**

One thing the journey does NOT show green and this lane did not cause: **10 of 23 rows miss the probe's own
`converged` predicate** (`phase==="idle" && ratio===1 && computing!==true`) inside its 60 s window — every
edit-role row, each with the right meshes and an empty `meshReasons`. The viewer rows converge in 3 s. The
blocking term is `computing`, which never clears in the edit role, and it is **present in this lane's
pre-change 14:37 capture** (`"computing":true` on `window:procedural-preview` with `phase:"idle"` and
`ratio:1`) — i.e. it predates every edit here. It is the same family as the user's 13:03 directive and is
worth a lane of its own.

### 5.6 THE SERVED READING — the 16:57 guest, with the reactor half in it

Lane `procedural-rename-fixforward` completed the peer's rename and restaged; the staged guest
(`…/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`) went 13:04 →
**16:57**. `:6021` recycled by pid, `screen -S g3dreact6021 -X quit`, `node_modules/.vite-temp` removed, the
script restarted; the served host modules were then re-fetched over `/@fs` and confirmed to carry
`commandIngressUnownedV1` (4), `PLUGIN_OPERATION_OWED_SLICE_MS`/`publishShellFrames` (5) and
`owedEffects.scope` (1) before a single reading was taken.

#### Faults — three probes, ~13 minutes of session

| counted in `served`, `served-hop` and `served-journey2` | |
|---|---:|
| `command ingress did not complete within … continuations` | **0** |
| `no channel` | **0** |
| `plugin.command-page-unowned` | **0** |
| `plugin.command-ingress-unowned` | **0** |
| `plugin.command-ingress-stalled` | **0** |
| page errors | **0** |

`plugin.command-page-unowned` is the reactor's new named fault and it fires **zero** times on a healthy
door — which is the point: it exists so that the shape which used to cost 1 024 silent crossings is named
the moment it happens.

Ingress, served, over the two instrumented runs: **105 × `command-complete continuations=0/16`** and
**2 × `command-complete continuations=0/1072`** (the 67-page `setContributions`, once per boot). Every
command completes on the turn that carried its page; the derived ceiling is never approached and the
continuation loop is never entered.

#### Boot and convergence

`🗑️generated/host-refresh/served`: **hex example converges, 0 page errors**, `bootConverged: true`.
`🗑️generated/host-refresh/served-hop`: **10/10 steps converged**, every example.

#### Crossings per hop — `🐍️react-hop-cost-probe.mjs` (`served-hop`, 61.8 s)

```
totals: 10 steps, 61.8 s, 18 hops, 682 worker crossings, converged 10/10
worker crossings per hop: 37.89; hop wall: 3431 ms
RecalcStyleDuration over the run: 931 ms = 1.51% of wall (gate: under 5%)
worker MoreWork drive: 1365 guest turns absorbed over 682 crossings (2.00 per crossing); stops carried=435 idle=209 input=27 budget=11
```

| posted event kinds | count | per hop | ui patches |
|---|---:|---:|---:|
| `patch-ack` | 196 | 10.9 | 161 |
| `message` | 172 | 9.6 | **0** |
| `(none)` | 159 | 8.8 | 46 |
| `surface-visible` | 76 | 4.2 | 48 |
| `request` | 38 | 2.1 | 0 |
| `completed` | 38 | 2.1 | 0 |

**37.89 per hop (31.7 excluding the boot step) against a target of ≤ 15 — still not met**, and served, it is
the same shape as the host-only reading (35.0 / 29.4): the reactor half removes a fault path, not crossings.
The two terms that would have to fall are unchanged and unclaimed — `patch-ack` at 10.9/hop (the patch-batching
lane's `UI_TURN_PATCHES_MAXIMUM` floor) and `message` at 9.6/hop, **172 crossings carrying nothing at all**,
one whole round trip per shell `send-message` instead of riding the next turn's event list.

#### The refresh scopes moved, served

`🗑️generated/host-refresh/served` vs the two earlier runs, same probe, same door:

| | before (13:04 guest, pre-change) | host-only | **served** |
|---|---:|---:|---:|
| pick `refreshScopes` | `full 4, partial 2` | `full 4` | **`partial 3, full 1`** |
| `refreshUi lane` owed decisions, partial | 16 | 16 | **18** |
| …owed decisions, full | 5 | 5 | 7 |
| pick worker crossings | 54 | 36 | 40 |

The pick's passes are now predominantly **partial** where before they were predominantly full. The residual
full is still `resolveUiDirtyScope(undefined) → {kind:"full"}` for guest responses that declare no scope
(§5.3) — named, not claimed.

#### Settles that owe nothing, served

```
settleStops {"idle":4}   settleContinuations 5   settleSamples 4
```

Four settles, five crossings — still ~one crossing each, and no settle hit the new 100 ms owed grace on this
door (the grace bites on a long tool run, which is what §1.3 measures on :6013).

#### The slider, served

```
drag {"found":3,"valueBefore":"6","valueAfter":"9","firstUpdateMs":null,"releaseToIdleMs":6}
```

`Column Height` moves 6 → 9 and **the preview still does not update** — unchanged by the reactor half, as
expected: it is guest-side coalescing, lane `slider-preview-update`.

#### The 10-minute session gate, served — `🐍️journey-probe.mjs`, run twice

| | `served-journey` (252 s) | `served-journey2` (633 s) |
|---|---:|---:|
| steps / mesh steps | 23 / 20 | 23 / 20 |
| `meshOk: false` | **0** | **0** |
| non-empty `meshReasons` | **0** | **0** |
| non-empty `faults` | **0** | **0** |
| `did not complete within … continuations` | **0** | **0** |
| `no channel` | 2 → *see below* | **0** |
| `plugin.command-page-unowned` | **0** | **0** |
| `plugin.command-ingress-unowned` / `-stalled` | **0** | **0** |
| page errors | **0** | **0** |
| `shell fault` | 1 | **0** |
| `command-complete continuations=0` | 129 | — |

**The 10.6-minute session gate is met on the served guest**: `served-journey2` is 633 s with **zero** of
every named fault, zero page errors, zero shell faults, and every mesh oracle green.

##### The served run found a third `no channel` site — fixed, with a law

`served-journey` still showed two `no channel` lines, and the pair is the whole story:

```
147520 warning [DEBUG] dropped typed-operation progress subscription for retired instance procedural#1
147561 error   [DEBUG] shell fault surface-node-graph  Error: … no channel for instance 1
                  at requireChannel (…/🔌️PluginRuntime/🟦️.tsx)
                  at Object.handleAction (…/🔌️PluginRuntime/🟦️.tsx)
                  at Object.<anonymous> (…/🏛️ShellHost/🟦️.tsx)
```

The first line is §3's ledger working — a lane that used to log an error now drops. The second is a
DIFFERENT path: `adaptPluginHandle.handleAction` called `requireChannel` **synchronously**, before the
promise existed, so `ShellHost`'s `.catch(actionError => …)` never saw it — the typed retirement escaped
into React's nearest error boundary and tore the node-graph surface down. A refusal is a rejection:
`handleAction`, `handleCommand` and `contextMenu` are now `async`, and a new law pins the shape the caller
actually uses (build the promise, attach `.catch`, never wrap in `try`):

```
[DEBUG] retired instance: handleAction/handleCommand/contextMenu all reject, none throws synchronously
 ✓ refuses a retired instance by REJECTION, never by a synchronous throw past the caller's catch
```

`served-journey2`, taken after that fix on a recycled `:6021` (served module verified to carry
`handleAction: async`), has **0 `no channel` and 0 `shell fault`**.

##### Convergence is load-flaky, and the mesh oracles are not

`served-journey` converged **21 of 23** rows by the probe's own predicate (`phase idle && ratio 1 &&
computing !== true`) where the 13:04 guest converged 13; `served-journey2`, under heavier load, converged 13
again. Both runs published exactly the right geometry on every row (`meshOk: false` 0, `meshReasons` 0).
So the `computing` term (§5.5) is **flaky under load, not fixed and not broken by this lane** — it swung
13 → 21 → 13 across three runs with no change to it in between.

---

---

## 6. The blocker, and how it cleared

`⚛️reactor/🔄️turn/🦀️.rs` compiles into the plugin guest component, so its change reaches the browser only
through `activate-generation3d-react-dev`. **Three foreground restages, all failed, none on this lane's
code:**

| # | duration | failed on |
|---|---:|---|
| 1 | 6 m 30 s | `🌊️flow/🖥️host/🦀️.rs` — `FlowHostRetirementState` has `fixture`, the code writes `host_document`; `DagHost` has `host_document`, the code reads `.fixture` (a half-applied rename inside one file) |
| 2 | 4 m 20 s | `🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` — `cannot find value fixture` / `host_snapshot` |
| 3 | 4 m 56 s | `✏️s/🔌️plugins/🌊️flow/…/🧬️schema/📸️snapshot/🦀️.rs` — the same rename, one layer further out |

Each attempt was made only after `cargo check -p …` on the previously-breaking crate came back **0 errors**
— the wave simply moves faster than a restage takes. The lane then stopped launching its own restages
rather than keep burning the shared build lock.

**A central restage then failed all six of its own attempts, on the same wave.** The coordinator ran
`📜️restage-nocache-s5.sh` (6 attempts, 120 s apart) under `screen -S restage` from 15:58;
`🗑️generated/react-s5/restage.txt` records `EXIT=130` on every one, each on the peer's half-landed rename in
the procedural artifact crates and none on this lane's code:

```
error[E0062]: field `artifact_schema` specified more than once
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/…/🧬️schema/💡️inferences/🦀️.rs:44:9
error[E0425]: cannot find value `snapshot` in this scope
  --> ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/🧬️schema/🦀️.rs:230:15
error[E0425]: cannot find value `fixture` in this scope    (×6)
```

This lane has modified **zero** files under `✏️s/` (`git status`, checked). Lane
`procedural-rename-fixforward` is completing the rename forward and owns the restage; this lane watches the
staged guest (`🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`,
still 13:04) and re-takes §5 the moment it is rewritten.

**Cleared at 16:57.** Lane `procedural-rename-fixforward` completed the rename forward and restaged; the
staged guest went 13:04 → **16:57**, `:6021` was recycled onto it, and **§5.6 is the served reading with the
reactor half in it** — 0 `plugin.command-page-unowned`, 0 `1024 continuations`, 0 `no channel`, 0 page
errors, every command settling in 0 continuations.

§5.1–§5.4 are kept as the host-TypeScript-only reading (13:04 guest) precisely because the pair is what
separates the host half of the change from the whole of it, and the pair says the same thing: the reactor
half removes a fault path, not crossings.

Note also, unrelated to this lane but on the same tree: the same rename wave broke the store worker's
`coldDocumentPairCursorEquals` import for ~13 minutes at 14:22, which is why this lane's first boot probe
returned a `SyntaxError` and three lines of console.

---

## 7. Files

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `command_ingress_while_owned` (the never-idle-while-owned rule) + `command_ingress_owner_cursor`; shape-guarded assembly take; terminal `else` raising `plugin.command-page-unowned` |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🧪️tests/🔬️command-ingress-terminal/🦀️.rs` | **2 new laws**; repaired the pre-existing red source-text scan (3 renamed call shapes) |
| `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `COMMAND_INGRESS_TURNS_PER_PAGE`, `commandIngressContinuationCeilingV1`, `commandIngressUnownedV1`; the drain folds the terminal over every collected turn and faults by name; `PLUGIN_OPERATION_OWED_SLICE_MS` + the walled `settleYieldsToRefreshV1`; `retainedUiRefreshEffects` publishes shell frames instead of dropping them; module-level `retiredPluginInstances` ledger (`markPluginInstanceRetiredForPluginV1` / `pluginInstanceWasRetiredV1` / `forgetPluginInstanceRetirementV1`); `handleAction`/`handleCommand`/`contextMenu` refuse by rejection, not by a synchronous throw |
| `🧰️framework/…/🔌️PluginRuntime/🧫️fixtures/command-ingress-drain.json` | **new** — the language-agnostic drain contract both twins read |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | **6 new laws** |
| `🧰️framework/…/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | the owed-effects slot records the pass's scope and the lane applies it (no hardcoded `{kind:"full"}`); the remounted-window refresh drops for a retired instance |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | **1 new law**; repaired the renamed fixture import that was failing the whole 626-law suite to load |
| `<ticket>/🐍️host-ingress-latency-probe.mjs` | **new** — boot → pick → slider → example switch → hover on one boot, counting both named faults, the owed-refresh scopes, the settle stop reasons and the crossing attribution |

---

## 8. Not claimed

- **Crossings per hop ≤ 15 is not met**, host-only 35.0 (29.4 ex-boot) and **served 37.89 (31.7 ex-boot)**. The two cuts that would reach it —
  `message` crossings that carry nothing (9.1/hop host-only, **9.6/hop served, 172 crossings, 0 ui patches**)
  and `patch-ack` (9.9 → **10.9**/hop) — are named and attributed in §5.1 and §5.6 and belong to the
  shard-client turn-event layer and the patch-batching lane.
- **Slider → preview ≤ 150 ms is not met**: there is no preview update at all, host-only (§5.4) and served
  (§5.6) alike. Guest-side, lane `slider-preview-update`.
- **The owed-refresh narrowing moved on the served guest but not on the 13:04 one** — pick scopes
  `full 4, partial 2` → `full 4` (host-only) → **`partial 3, full 1`** (served), §5.3 and §5.6. The residual
  full is `resolveUiDirtyScope(undefined) → full` for guest responses that declare no scope, which this lane
  identifies and does not fix.
- **`computing` never clearing in the edit role** (§5.5) is named, shown to predate this lane's first edit,
  and shown to be **load-flaky** across three runs (13 → 21 → 13 of 23 converged rows, with the mesh oracles
  green in all three). Neither fixed nor broken here; left to its owner.
- **Two reds in the React `PluginRuntime` suite are peer-owned**, both in files this lane never opened
  (§4.2): the `documentJson`/`artifactJson` fixture drift, and a `Shell` ⇄ `ShellHelpers` module-init cycle
  a peer's 16:29 rework flipped.
- The `1024` fault **did not reproduce live** in any of this lane's seven runs (four host-only, three
  served); the root cause is established
  from the reactor source and the fleet's captured console, and the fix is proven by law and by the live
  `continuations=0/16` readings — not by a before/after of the fault itself.
