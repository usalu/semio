# 🧵 What a guest turn actually costs — the shard worker, instrumented from the inside (lane `react-guest-turn-cost`, 2026-09-14)

Opus lane on port **6021** (`📜️serve-generation3d-react-6021.sh`, host TS vite-live, `SEMIO_VITE_HMR=0`).
Continues `📓️react-hop-latency-2026-09-14.md`, whose §10 left two things unmeasured and one cut unbuilt:
the split of `channel` between message-passing and guest compute, and the ingress-generation gate of §4.
Both are answered here.

---

## 0. The headline, before anything else

> **A `flowEvalTick` hop is not three guest turns. It is 74 worker round trips.**

The previous lane could see `channel` (442 ms) and `refresh.turn` (420 ms) and correctly concluded that
a hop is three serialized guest turns. What it could not see — the CDP `Performance` domain answers
nothing on a shard-worker target — is that **each of those turns is itself ~16 to 20 host↔worker round
trips**, 79 % of which post NO events at all and 90 % of which come back with NO ui patch. Measured:
**3 786 worker crossings across 51 hops**, at ~6.1 ms of guest poll plus ~3.3 ms of reply crossing
plus ~0.7 ms of post crossing each.

That is what the ~420 ms per turn is made of, and it is why widening a turn (the previous lane's
mounted-window narrowing) moved nothing: the per-turn cost is **round-trip count × crossing price**,
and neither factor cares how many window bodies the turn renders.

---

## 1. The instrument

### 1.1 The worker half, and the clock problem it had to solve

`🟨️shard-worker.js` is generated (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`'s
`shardWorkerSource()`), so the spans are generated with it. Five new stages join the declared
vocabulary in `🧰️framework/🔨️modules/⏱️trace/🧫️fixtures/🪃️hop-stages/🔣️.json`:

| stage | owner | covers |
|---|---|---|
| `worker.receive` | `ShardClient.send` → worker `onmessage` | the POST crossing |
| `worker.decode` | worker `onmessage` | command page + event admission before the guest is entered |
| `worker.guest` | worker `actor.api.poll` | the reactor turn itself |
| `worker.reply` | worker reply → `ShardClient.handleMessage` | result clone + the main thread's pickup |
| `worker.turn` | worker `onmessage` | the worker's own busy span, handler entry to reply post |

**The clock**: a Worker's `performance.now()` counts from ITS OWN `timeOrigin`, so a duration measured
inside the worker is comparable across realms and an INSTANT is not — and the two crossings are
exactly the terms that need instants on both sides. `performance.timeOrigin + performance.now()` is
the same Unix-epoch millisecond in both realms at sub-millisecond resolution
(`hopTraceEpochNowMs`/`hopTraceEpochToTimeline`, `⏱️trace/🟦️.ts`, with the worker carrying a verbatim
twin). The host stamps `postedAtEpochMs` on every `turn` message; the worker returns
`{postedAt, receivedAt, guestEntered, guestLeft, repliedAt, events, eventKinds, patches, status}` on
the reply; `ShardClient.handleMessage` replays the five spans onto the PAGE's own hop timeline through
the new `hopTrace.record`, so one probe read returns a breakdown that adds up:

```
worker.receive + worker.decode + worker.guest + worker.reply  ==  the crossing the host observes
worker.turn                                                   ==  the worker's own busy span inside it
```

The worker also carries `previousReplyCloneMs` — the synchronous cost of its own
`postMessage` on the PREVIOUS reply, since a worker cannot amend a message it has already posted.
That is what separates "the structured clone was expensive" from "the main thread was busy".

Accounting only: a malformed or missing timings record publishes no span and changes nothing about
the turn. The worker's own cost is two `performance.now()` pairs per request.

### 1.2 The host half — the term neither side could see

Three more stages close the remaining gap, because the continuation loop between two crossings is
host code that no existing span covered:

| stage | owner | covers |
|---|---|---|
| `turn.accept` | `PluginRuntime.settlePluginTurn` | admitting one continuation's ui patches (incl. the ack turns admission itself submits) |
| `turn.decide` | `PluginRuntime.settlePluginTurn` | the loop's own required-surface and progress bookkeeping |
| `turn.yield` | `PluginRuntime.settlePluginTurn` | the macrotask yield taken every batch |

### 1.3 The probe

`🐍️react-hop-cost-probe.mjs` gained real `worker` columns, a per-step `crossings` count, a
per-crossing breakdown, and — the row that did the attribution — a histogram of crossings **by the
event kinds the host posted**, with the ui patches each kind produced.

```
cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d \
  SEMIO_PROBE_OUT=react-turn/before bun 🐍️react-hop-cost-probe.mjs
```

The old CDP-worker columns are kept and still read `0`; they are now clearly labelled `worker cpu ms`
and are no longer the only thing that could have answered this question.

---

## 2. The worker-side breakdown

`🗑️generated/react-turn/attrib2/` — 10 steps, 71.3 s, 51 `flowEvalTick` hops, 3 796 worker crossings,
10/10 converged, on the 18:12 procedural wasm (lane `flow-tick-coalescing`'s one-wave-per-level guest).

### 2.1 Per stage

| stage | count | total ms | mean ms | **per hop ms** | share of hop wall |
|---|---:|---:|---:|---:|---:|
| `worker.guest` | 3 796 | 22 864 | **6.0** | **448** | 32 % |
| `worker.reply` | 3 796 | 12 396 | **3.3** | **243** | 17 % |
| `turn.accept` | 3 379 | 11 338 | 3.4 | **222** | 16 % |
| `commit` | 116 | 6 017 | 51.9 | 118 | 8 % |
| `worker.receive` | 1 356 † | 1 383 | 1.0 | 27 † | ≥2 % |
| `arm` | 61 | 1 046 | 17.1 | 21 | 1.5 % |
| `worker.decode` | 3 796 | 152 | 0.04 | 3 | — |
| `refresh.project` | 116 | 145 | 1.3 | 3 | — |
| `turn.yield` | 298 | 32 | 0.1 | 1 | — |
| `turn.decide` | 3 136 | 5 | 0.002 | 0 | — |
| `refresh.slots` / `refresh.apply` / `encode` / `decode` / `mesh.decode` | — | ≤6 each | 0.0 | 0 | — |
| `worker.turn` | 3 796 | 23 081 | 6.1 | 453 | (contains decode+guest) |
| — | | | | | |
| `channel` | 74 | 28 418 | 384.0 | 557 | the dispatch, made of the above |
| `refresh.turn` | 116 | 43 773 | 377.4 | 858 | the two refresh passes, made of the above |
| hop wall | 51 | | | **1 397** | |

† `worker.receive` publishes only where the receive instant is provably after the post instant; sub-
resolution and slightly skewed samples are dropped rather than clamped, so 1 356 of 3 796 crossings
contribute and the per-hop figure is a **lower bound**. At the published 1.0 ms mean over 74 crossings
the honest estimate is ~74 ms/hop.

### 2.2 The crossings, attributed by what the host posted

| crossing (posted event kinds) | count | **per hop** | total ms | mean ms | ui patches produced |
|---|---:|---:|---:|---:|---:|
| **(none)** | **3 019** | **59.2** | **18 944** | 6.3 | 318 |
| `surface-visible` | 111 | 2.2 | 2 049 | 18.5 | 0 |
| `request` | 38 | 0.7 | 1 111 | 29.2 | 0 |
| `patch-ack` | 322 | 6.3 | 525 | 1.6 | 0 |
| `message` | 265 | 5.2 | 210 | 0.8 | 0 |
| `instance-open` | 1 | 0.0 | 117 | 117.1 | 0 |
| `completed` | 38 | 0.7 | 84 | 2.2 | 0 |
| `job-completed` | 1 | 0.0 | 37 | 36.9 | 0 |
| `instance-lifecycle-ack` | 1 | 0.0 | 4 | 4.3 | 0 |

**Read that first row.** 79 % of every crossing this renderer makes posts an EMPTY event list. They
exist only because the previous turn answered `MoreWork`, and they carry 87 % of the worker's total
busy time. One in ten returns a patch; nine in ten cost 6.3 ms of guest and 3.3 ms of reply to be told
"not yet".

### 2.3 Why the guest says `MoreWork` — named, not guessed

`🗑️generated/react-turn/morework/console.txt` (`SEMIO_PROBE_GUEST_DIAGNOSTICS=1`, which arms the
reactor's own `LAST_MORE_WORK_SOURCES` trace), 529 more-work turns over a 35 s boot:

| arming source | turns | share |
|---|---:|---:|
| `reconcile` | 428 | **81 %** |
| `command_ingress` | 61 | 12 % |
| `typed_operation` | 38 | 7 % |
| `lifecycle` | 2 | <1 % |
| `executor_deadline` | **0** | **0 %** |

`executor_deadline` — "the reactor executor still had ready work when its 8 ms slice expired"
(`⚛️reactor/🔄️turn/🦀️.rs:1249`) — **never fires**. The guest is not being cut off mid-compute. It
finishes its slice, finds the retained-surface reconcile tracker still holding work
(`has_publishable_work() || has_unpublished()`, `🔄️turn/🦀️.rs:1239`), answers `MoreWork`, and the host
pays a whole round trip to ask again. The longest observed streak is 65 consecutive turns.

This is the same family as `project-reconcile-tracker-more-work-spin` (fixed four times on
2026-09-09/10), one layer further out: the tracker no longer spins *forever*, it spins **~60 times per
hop**, and each spin is priced at a host round trip because in the browser the host round trip IS the
guest's pump — there is no self-driving loop on the other side of the worker boundary.

### 2.4 What this excludes, by measurement

- **Command/reply codec**: `encode` 0.1 ms, `decode` 0.05 ms, `worker.decode` 0.04 ms. Dead.
- **Big-body encoding**: `refresh.project` 1.3 ms mean over 116 passes, `mesh.decode` 0.3 ms. The
  1.3 MB `🔣️.json` interface file and the mesh payloads are not in this path at all. Dead.
- **The reactor's 8 ms hold pump**: never reached (§2.3). Dead as a *cause*; it is the ceiling that
  would matter if the guest were compute-bound here, and it is not.
- **The continuation loop's own bookkeeping**: `turn.decide` is 3 136 × 0.002 ms. The suspected
  quadratic re-scan of accumulated `uiPatches` per continuation is real in shape and free in practice.
  Dead.
- **The macrotask yield**: `turn.yield` is 298 × 0.1 ms = 1 ms/hop. Dead.
- **React**: `refresh.apply` 0.0 ms, `commit` 52 ms mean / 118 ms per hop (8 %), `RecalcStyleDuration`
  2.78 % of wall — still inside the 5 % gate the previous lane armed.

---

## 3. What was cut

### 3.1 The ingress-generation gate — `📓️react-hop-latency` §4, landed

**The mechanism, restated from the measurement**: a converging preview arms one chain per flow window,
so each hop produces two `flowEvalTick` completions and each completion demands a full refresh pass.
The second request arrives a few milliseconds after the first pass has already submitted its own guest
turn — and BOTH completions are host-side callbacks of turns that had already settled, so **nothing
crossed into the guest in between**. The pass in flight is re-rendering exactly the state the second
request wants re-rendered.

**The machinery that was missing**, now built:

- `🔌️PluginRuntime/🟦️.tsx` — `guestIngressGenerationV1(instanceId)`, a per-instance count of
  guest-MUTATING ingress, bumped in `performInvocation` and at the extension-completion publication,
  and **deliberately not** by `refreshUi`'s own turn (a refresh cannot invalidate another refresh).
  Cleared with the instance's maps.
- `🛠️ShellHelpers/🟦️.tsx` — `uiDirtyScopeCoveredByV1(inner, outer)` and
  `uiRefreshAlreadyAnsweredV1(running, next, ingressNow)`. Four ways the join could be wrong, four
  refusals: ingress crossed since the running pass submitted; the request carries a **host-owned**
  render input (view state, armed utility, locale, a newly mounted window); the scope is not covered;
  a body replacement the running pass is not doing. Plus: a different instance, and a pass that has
  not submitted its turn at all.
- `createUiRefreshCoalescerV1` — a fifth lane state. A request the gate admits **joins the running
  pass's waiters** instead of entering the owed slot, and inherits that pass's verdict, rejection
  included. Without a gate supplied the lane behaves exactly as before.
- `🏛️ShellHost/🟦️.tsx` — `UiRefreshLaneRequest.hostInputs`, which **defaults to true**: a request that
  cannot prove it carries no host-owned input is never joined. Only the completion path lowers it, and
  only when it can prove it — `hostEffectsRewriteGuestRenderInputsV1(effects) === false` **and** the
  view state did not change. The pass records the ingress generation *inside* the pass, immediately
  before the guest crossing, because that instant — not the instant the request was made — is what a
  joiner must be compared against.

The direction of every doubt is fixed: a join that should not have happened is a stale pane with no
fault anywhere, so unknown coverage refuses.

### 3.2 The per-step promise in the retained-UI intake drive

`turn.accept` is 222 ms per hop — 16 % — and it is where the host admits the guest's published ui
patches. Its inner loops (`acceptUiPatches`, `closeIntake`, `advanceUiMaintenance`, the read walk)
each ran `await yieldUi(step)` **on every step**, where `yieldUi` was an `async` helper that yields
once per 1 024 steps. So 1 023 of every 1 024 steps allocated a promise and spent two microtask ticks
to be told "no". An intake step is a wire phase costing ~3.4 µs and a world-3d surface publication
takes hundreds of thousands of them.

`uiIntakeOwesYieldV1(step)` is the same stride asked synchronously; the six call sites now read
`if (uiIntakeOwesYieldV1(step)) await yieldPluginUiContinuation();`. The yield **cadence is
unchanged** and is asserted as law. The wgpu shell's frame worker carried the identical defect on its
own decoder drive (`📓️wgpu-edit-convergence-perf-2026-09-14.md`: "allocated a promise per decoder
phase").

---

## 4. Files changed

| file | change |
|---|---|
| `🧰️framework/🔨️modules/⏱️trace/🟦️.ts` | five `worker.*` + three `turn.*` stages; `hopTracer.record`; `hopTraceEpochNowMs`/`hopTraceEpochToTimeline` |
| `🧰️framework/🔨️modules/⏱️trace/🧫️fixtures/🪃️hop-stages/🔣️.json` | the eight new stage declarations + two invariants |
| `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` | `postedAtEpochMs` on every turn post; `ShardWorkerTurnTimings`; `publishShardWorkerTurnSpans` |
| `🧰️framework/…/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` | the generated worker's own `hopEpochNow` + per-turn timings, event kinds, patch count, status and reply-clone carry |
| `🧰️framework/…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `guestIngressGenerationV1`/`noteGuestIngressV1`; `turn.accept`/`turn.decide`/`turn.yield` spans; `uiIntakeOwesYieldV1` replacing the per-step async yield |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `uiDirtyScopeCoveredByV1`, `uiRefreshAlreadyAnsweredV1`, the coalescer's join gate + `answered()` |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🚪️ingress-generation-gate.json` | **new** — the language-agnostic gate declaration |
| `🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` | `hostInputs` on the lane request, the pass's ingress capture, the gate wiring |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🚪️ingress-generation-gate/🟦️.ts` | **new law** |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/⏱️hop-trace/🟦️.ts` | the cross-realm `record` laws |
| `🧰️framework/…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers the new suite |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | `⏱️trace/🟦️.ts` in the wgpu browser profile's `sourceModulePaths` + `inputPatterns` |
| `🧰️framework/…/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` | `⏱️trace/🟦️.ts` in `frameWorkerSources` |
| `<ticket>/🐍️react-hop-cost-probe.mjs` | real worker columns, crossing count, crossing-kind histogram, per-stage totals table |

The two taxonomy/project edits are the same fix-forward the `⏯️tool-run` wave needed: `📮️shard-client`
is in the wgpu browser bundle, so a new module it imports must be declared in both places or
`generate-frame-worker` breaks.

---

## 5. Laws, with output

All through `cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && bun ./📜️script.ts test long <filter>`.

```
$ … test long "ingress-generation-gate"      $ … test long "hop-trace"
 Test Files  1 passed (1)                     Test Files  1 passed (1)
      Tests  25 passed (25)                        Tests  13 passed (13)

$ … test long "mounted-window-fetch"         $ … test long "world3d-mesh-residency"
 Test Files  1 passed (1)                     Test Files  1 passed (1)
      Tests  9 passed (9)                          Tests  17 passed (17)
```

**`🚪️ingress-generation-gate` (new, 25)** asserts both directions, and the refusal direction by name:

- the declared contract — a refresh turn does not bump ingress, a dispatch and an extension-completion
  publication do, host inputs never join, unknown coverage refuses, a joiner inherits the verdict;
- ten `uiDirtyScopeCoveredByV1` cases including the two that must refuse (`full` inner against a
  `partial` outer; a section flag or body the outer does not name);
- nine gate cases: the measured redundancy joins; a dispatch that crossed since, a host-owned render
  input, a wider scope, an unmatched body replacement, a different instance and a pass that has not
  submitted its turn each refuse;
- the lane itself, driven for real: a joined request runs **one** pass and settles with it; ingress
  moving under the pass owes a **second**; a failed pass **rejects** its joiners rather than reporting
  them covered; and with no gate supplied the lane behaves exactly as before;
- the intake yield cadence: exactly one yield per 1 024 steps, from a predicate that is not a promise.

**`⏱️hop-trace` (9 → 13)** gained the cross-realm laws: a recorded span lands where it HAPPENED rather
than where this realm's clock is now, a negative duration clamps instead of publishing a span that
ends before it starts, every `worker.*` stage is declared, and `hopTraceEpochToTimeline` never places
an instant before this realm's origin.

Whole React engine corpus, after every change:

```
$ … test long
 Test Files  2 failed | 43 passed (45)
      Tests  4 failed | 1186 passed (1190)
```

The four reds are **pre-existing and peer-owned**, none in a file or function this lane touched:
`🎟️resident-refresh-budget` (a peer added a law to the JSON fixture), `engine-contract`'s
`CSS.escape` jsdom gap, the open `viewer mints translateSelection` item, and a `shell option locks`
assertion a peer's live `selectionMode`/`showEdges` change moved. The previous lane recorded five;
one of them is now green and this lane added +28 passing (the corpus also grew under peers during the lane).

---

## 6. Before / after

### 6.1 Before, and the hop count per example (coordinator's extra deliverable)

`🗑️generated/react-turn/before/` and `🗑️generated/react-turn/attrib2/`, both on the **18:12** procedural
wasm — i.e. WITH lane `flow-tick-coalescing`'s one-wave-per-dependency-level guest.

| example | hops, `📓️react-hop-latency` "before" (pre-coalescing wasm) | hops, this lane (18:12 wasm) | s |
|---|---:|---:|---:|
| boot | 7 | **6** | 11.4 |
| No example | 2 | 2 | 3.2 |
| Hexagonal Mushroom Column | 3 | 3 | 5.1 |
| Rectangle Extrude Volume | 8 | **6** | 8.3 |
| Sphere Cut With Torus | 7 | **6** | 8.3 |
| Box Fillet Preview | 6 | 6 | 7.2 |
| Sphere Box Fuse | 7 | **6** | 7.2 |
| Face Sweep Extrude | 8 | **7** | 9.3 |
| Rectangle Wire Preview | 3 | 3 | 4.1 |
| Box Shell Preview | 6 | 6 | 7.3 |
| **total** | **57** | **51** | **71.3 s** |

So the coalescing wave cut five examples by one to two hops each (57 → 51 over ten steps, −11 %), and
the wall is 79.3 s → 71.3 s (−10 %). It did **not** reach the expected ≤ 3 hops for five examples: six
of the ten still take 6-7 hops. Reported as measured, not as hoped; that lane owns the gap.

### 6.2 After — NOT MEASURED, and why

The `contributions-ingress-ceiling` fault landed between this lane's attribution runs and its
verification run, and was still open when this lane ended. Three guests were tried:

| guest | verdict |
|---|---|
| **18:12** | the last one that converges — every number in §2 and §6.1 is taken on it |
| **19:05** | `setContributions command failed procedural … command ingress exceeds 64 pages` → `No loaded plugin contributes the flow extension "brep"`; every preview `faulted`, 0 examples converge |
| **19:22** | same door, a different ceiling: `tool factory 's.procedural.generation3d@1/*#editor/setContributions' rejected 273136 raw bytes before decoding; maximum is 262144` (`🗑️generated/react-turn/after-boot-check/console.txt`); preview still `faulted` ("Geometry extension unavailable"), 0/10 steps converge (`🗑️generated/react-turn/after-blocked/`) |

The coordinator's restage retry also failed at 19:14 (`plugin catalog build failed: flow`,
`🗑️generated/restage-retry.txt`). So **no valid post-change hop measurement exists**, and the target
(hop ≤ 500 ms, every example in ≤ half the §3.2 wall) is **neither met nor refuted by this lane**.
`🗑️generated/react-turn/after-blocked/` is deliberately NOT named `after`: it is a non-converging run
kept only as evidence of the blocker.

What IS verified on the current tree, on the faulted guest
(`🗑️generated/react-turn/gate-check/`, `🗑️generated/react-turn/faulted-smoke/`):

- the gate fires at runtime — `[DEBUG] refreshUi lane {"decision":"answered", …}` — with **0 page
  errors** and the lane still passing, owing and merging as before;
- the worker spans publish on the page timeline and the `worker.reply` split reports;
- the whole probe pipeline runs end to end with the new columns.

The one command that settles §6, to run on the first guest that boots without `exceeds 64 pages`:

```
cd <ticket>
SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d SEMIO_PROBE_OUT=react-turn/after bun 🐍️react-hop-cost-probe.mjs
SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d bun 🐍️journey-probe.mjs      # 23/23, every mesh oracle green
```

Expected from the measurement, stated in advance so it can be checked rather than narrated:

- `refresh` passes per hop 2.27 → ~1.3, i.e. one whole guest turn (~20 crossings, ~380 ms) off a hop;
- `turn.accept` below its 222 ms/hop, by the promise-per-step removal;
- `worker.reply` below its 243 ms/hop as a consequence of both, since §2.4 proves it is main-thread
  contention and not payload;
- hop wall ~1 400 ms → ~900-1 000 ms. **That is still above the 500 ms target**, because §7 names what
  the remaining ~59 empty-event crossings per hop cost and that cut is not in this lane.

---

## 7. The cut that is left, named with its evidence

**~59 of the 74 crossings a hop makes post no events and return no patch, and 81 % of them exist
because the guest's retained-surface reconcile tracker answered `MoreWork`.** They cost 18.9 s of the
23.1 s of worker time in a 71 s run.

The reactor's own trace (`🗑️generated/react-turn/morework/console.txt`) narrows it further than "the
tracker is busy":

- `executor_deadline` is **0 of 529** — the guest is never cut off mid-compute, so raising the 8 ms
  reactor hold would buy nothing;
- of the 428 `reconcile`-armed turns, **92 have `ready=[] terminals=[] producer_terminals=[]
  deferred=[]`** — every reconcile family empty — and in every one of those the PENDING authority holds
  exactly one slot, alternating `…:e---p` with `handback_sequence=Some(n)` (published, awaiting the
  host's handback) and `…:ea---` with `handback_empty=true`;
- `427 of 428` carry `effects=0`;
- the host side of that same handshake is visible as **6.3 `patch-ack` crossings per hop** for 6.2
  published patches per hop.

So the structure is: **one published UI patch costs a multi-turn host handshake**, and
`reconcile_work` is folded as `more || has_publishable_work() || pending.has_unpublished()`
(`⚛️reactor/🔄️turn/🦀️.rs:1239`) — read at a point where a patch this very turn reconciled into pending
is still "unpublished". This is the same family as `project-reconcile-tracker-more-work-spin`, one
layer out: the tracker no longer spins for ever, it spins ~60 times per hop, and in the browser every
spin is a host round trip because the host round trip IS the guest's pump.

It was not attempted here for two reasons, both stated rather than implied: it is guest Rust needing a
full restage, and lane `contributions-ingress-ceiling` held the only guest build slot for the whole
window this lane had.

---

## 8. Not claimed

- **The target is not met, and this lane cannot say by how much it missed.** §6.2: no valid post-change
  hop measurement exists, because the only guest that boots is faulted by another lane's regression.
  Every "after" figure in §6.2 is an expectation with its check, not a result.
- **The journey probe was not re-run green.** `🐍️journey-probe.mjs` on the 19:05/19:22 guests cannot converge
  (the preview is `faulted` before any step runs), so the 23/23 invariant is **unverified** for this
  lane's changes. The React engine corpus (1 186 passing, 4 pre-existing reds) and the runtime
  gate-check (0 page errors, the gate deciding `answered`) are what is verified instead.
- **The ~420 ms per turn is attributed, not eliminated.** §7 owns the remainder.
- `worker.receive` is a **lower bound** where the two realms' epoch clocks skew below its resolution:
  samples whose receive instant reads before the post instant are dropped rather than clamped, so the
  published per-hop figure (27-32 ms) understates a true ~74 ms. The `faulted-smoke` run, where all
  450 samples survived, measured 0.1 ms per crossing — so this term is small either way and is not
  load-bearing for any conclusion here.
- The reply-clone figure (0.01 ms per crossing) is carried on the FOLLOWING reply, so it lags one turn.
  Over 450 and 3 796 samples that is immaterial to the conclusion but it is not a per-turn figure.
- **The `heartbeat()` / `heartbeat("turn-step")` pair the worker posts on every request** — two extra
  main-thread messages per crossing, ~7 600 per run, on a main thread §2.4 shows is the binding
  constraint — was measured as a candidate and **not changed**: it is the shard liveness contract, it
  has a wgpu twin and its own laws, and no lane owns it today. Named, not fixed.
- Nothing about the wgpu door was measured or changed; the two declaration edits
  (`🔣️taxonomy.json`, the wgpu `📋️project.json`) only keep its frame-worker bundle building now that
  `📮️shard-client` imports `⏱️trace/🟦️.ts`. That IS verified —
  `bun nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker` is green (32.5 s, 4 tasks) —
  but no wgpu runtime behaviour was exercised.
- The `DuplicateSiblingKey` fault and the `contributions-ingress-ceiling` fault are neither fixed nor
  investigated here.

---

## 9. Incident this lane had to clear

At 18:37 the repo hit **ENOSPC** — 380 MB free on a 926 GB volume — which killed the :6021 vite watcher
mid-edit (`ENOENT … 🟦️.ts.tmp…`, `EXIT=1`) and would have failed every build in the fleet. Freed by
pruning the two cargo incremental trees no live build was using:
`⚡️cache/cargo/build/wasm32-wasip2/{wasm-dev,debug}/incremental` (39 GB + 7.6 GB) → **121 GB free**.
`debug/incremental` (52 GB) and `wasm32-unknown-unknown/debug/incremental` (24 GB) were left alone
because peers were actively compiling into them. This is the third recurrence on this ticket
(`feedback-incremental-cache-regrows-under-fleet`).
