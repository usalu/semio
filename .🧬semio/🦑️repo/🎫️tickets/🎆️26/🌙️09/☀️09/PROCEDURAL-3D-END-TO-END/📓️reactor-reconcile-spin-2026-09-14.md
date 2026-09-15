# 🔁 The reconcile spin — why 89 of 111 worker crossings per hop carried nothing, and what now carries them (lane `reactor-reconcile-spin`, 2026-09-14/15)

Opus lane on port **6021** (`📜️serve-generation3d-react-6021.sh`, host TS vite-live, `SEMIO_VITE_HMR=0`).
Continues `📓️react-guest-turn-cost-2026-09-14.md` §7, which named this cut and did not build it:

> ~59 of the 74 crossings a hop makes post no events and return no patch, and 81 % of them exist
> because the guest's retained-surface reconcile tracker answered `MoreWork`.

That lane could not measure its own "after" (the `contributions-ingress-ceiling` fault held the only
guest build slot). By the time this lane ran, that ceiling was gone, the guest boots, installs its
272 089-char contributions pack and converges with meshes — so the cut is both measurable and built.

---

## 0. The headline

> **The guest was paying a host round trip to move a patch from one of its own slots into the turn
> result, and a second one to free the slot the host had already acknowledged. Neither is work. Both
> are gone, and the empty crossings that remain are now absorbed inside the worker.**

| | before | after |
|---|---:|---:|
| worker crossings per `flowEvalTick` hop | **110.71** | **79.70** |
| of those, crossings posting NO events and returning NO patch | 89.1 / hop (80.5 %) | 49.8 / hop (62.5 %) |
| hop wall | 24 057 ms | **14 113 ms** |
| whole 10-step run | 409.0 s, 5/10 converged, **0 meshes** | **282.3 s, 7/10 converged, meshes on 9/9 examples** |
| reactor turns armed per 45 s of the same page | 197 | **335** |
| publication ping-pong turns (`:e---p` / `:ea---`) | 27 + 27 of 115 reconcile-armed turns | **0 of 239** |

The last two rows are the clean A/B (same probe, same page, same duration, one guest apart): the guest
now runs **70 % more reactor turns** while the host makes **28 % fewer crossings** — the worker is
pumping it instead of the host. And the state the previous lane named as the spin's signature —
one pending slot alternating `…:e---p` with `…:ea---` while every retained-surface family is empty —
**no longer occurs at all**.

---

## 1. The families, printed first

Per the standing rule (`project-reconcile-tracker-more-work-spin`: *print the retained-surface tracker
families first*), the reactor's own `MORE_WORK_TRACE` was read before anything was changed —
`SEMIO_PROBE_GUEST_DIAGNOSTICS=1 bun 🐍️console-dump-probe.mjs`, 45 s of boot on the pre-change guest
(`🗑️generated/reconcile-spin/blocker-check/console.txt`, 197 armed turns):

| arming source | turns | share |
|---|---:|---:|
| `reconcile` | 115 | **58 %** |
| `command_ingress` | 66 | 34 % |
| `typed_operation` | 14 | 7 % |
| `lifecycle` | 2 | 1 % |
| `executor_deadline` | **0** | **0 %** |

and the 115 `reconcile` turns fall into exactly **two** families.

### 1.1 Family 1 — the publication ping-pong (54 of 115 turns), every family empty

```
ready=[] terminals=[] producer_terminals=[] deferred=[]   pending=[slots=[9:e---p]  handback_empty=false handback_sequence=Some(9)]
ready=[] terminals=[] producer_terminals=[] deferred=[]   pending=[slots=[9:ea---]  handback_empty=true  handback_sequence=None]
```

Twenty-seven turns of each, strictly alternating. **Nothing in the retained-surface tracker is
non-empty.** The whole cost is the pending publication authority holding exactly one slot, in one of
two states, and answering `MoreWork` for each.

**Root cause A — `…:e---p`, `handback_sequence=Some(n)`.** `PendingPatchAuthority::take_one`'s reconcile
arm called `SurfaceReconcileReadyPatch::publish_into(&mut self.turn_handback, …)`, set
`slot.emitted = true` — **and returned `Ok(None)`**. The patch it had just published sat in
`turn_handback` until the NEXT call. So the turn that published a surface returned no patch, answered
`MoreWork` (because `has_unpublished()` sees a non-empty handback), and the host paid a whole round
trip whose only work was `turn_handback.source_mut().take()`. `publish_into` is atomic — it moves the
entire source or nothing — so there was never a reason for the split.

**Root cause B — `…:ea---`, `handback_empty=true`.** The turn retires acknowledged publications ONCE,
near its top, **before** it processes its events. So every `Event::PatchAck` a turn carried left its
slot `acknowledged` with nothing left in that turn to retire it; `has_unpublished()` sees
`slot.acknowledged`, the turn answers `MoreWork`, and the host pays a second round trip whose only
work is a `close_step` the same turn could have run 200 µs later.

Two round trips per published surface, at 6.3 ms of guest plus 3.3 ms of reply each, for zero work.

### 1.2 Family 2 — a ready output the host is blocking (61 of 115 turns)

```
ready=[g22:--r-,g23:--r-,g25:p---,g30:p---]   deferred=[]   pending=[slots=[]]
slots=[… 1:framework.panel.artifact#g22:-J-, 1:framework.panel.catalogue#g23:-J- …]
```

Here the tracker genuinely has work — two reconcile JOBS (`-J-`) are running — and two ready outputs
(`p---`) are waiting. These turns are legitimately `MoreWork`. But two defects made them expensive:

1. **The drive spun.** The reconcile loop's condition was `more = patches.has_publishable_work()`,
   which is true for a ready output *nobody can extract*. When the publication authority has no free
   slot — its release depends on the host's acknowledgement — the loop called `drive_one()` on a
   tracker with nothing to drive for its whole 1 024-opportunity budget or until the turn's wall
   deadline expired. **That is the 6.0 ms mean on an empty crossing**: not compute, a spin.
2. **The generation gate is invisible to the loop.** `next_ready_index` refuses to publish a ready
   output while any slot with a LOWER generation still has a producer or job (ordering, by design), so
   even with free capacity the loop can legitimately publish nothing while it drives.

And on top of both: **in the browser the host round trip IS the guest's pump.** There is no
self-driving loop on the far side of the worker boundary, so every one of these turns — real work or
not — costs a POST, a poll, a structured clone and a main-thread pickup.

---

## 2. The measurement it was built on

`🗑️generated/reconcile-spin/before/` — `🐍️react-hop-cost-probe.mjs`, 10 steps, 409.0 s, 17
`flowEvalTick` hops, **1 882 worker crossings**, 5/10 converged, on the 6021 guest as of 19:46.

| worker crossing (posted event kinds) | count | per hop | total ms | mean ms | ui patches |
|---|---:|---:|---:|---:|---:|
| **(none)** | **1 515** | **89.1** | **10 651** | 7.0 | 148 |
| `patch-ack` | 148 | 8.7 | 594 | 4.0 | 0 |
| `surface-visible` | 44 | 2.6 | 1 262 | 28.7 | 0 |
| `message` | 126 | 7.4 | 115 | 0.9 | 0 |
| everything else | 49 | 2.9 | 502 | — | 0 |

**80.5 % of every crossing this renderer makes posts an empty event list**, and they carry 80 % of the
worker's busy time. `worker.guest` was 773 ms/hop, `worker.reply` 290 ms/hop, `turn.accept`
294 ms/hop, of a 24 057 ms hop wall.

---

## 3. The design — four changes, each at the layer that owns it

### 3.1 The pending slot is a named state machine, not five booleans

`⚛️reactor/📨️pending/🦀️.rs` gains `PendingPatchPhase { Queued, Delivered, Issued, Acknowledged,
Rejecting }`, derived from the slot in one place, and the arming question is answered per phase:

| phase | meaning | arms the turn? |
|---|---|---|
| `Queued` | pushed by the reconcile drive, not yet handed to a turn | **yes** — `has_undelivered()` |
| `Delivered` | published into THIS turn's result, receipt not yet staged | no — it is this turn's output |
| `Issued` | the host holds it, its receipt is committed | **no** — HOST-blocked; the answer arrives as its own event-carrying turn |
| `Acknowledged` | the host acknowledged it; the owner still needs retiring | **yes** — `has_retiring()` |
| `Rejecting` | a rejection is being applied | **yes** — `has_retiring()` |

`has_unpublished()` is kept, `#[cfg(test)]`-only, defined independently through
`PendingPatchPhase::guest_owes_turn` — so the partition
`has_unpublished() == has_undelivered() ∪ has_retiring()` is a law, not a comment. The trace line now
prints the phase beside the flags (`26:Issued:e--c-p`), so a future reader does not have to decode
five characters again.

### 3.2 `take_one` publishes and hands out in ONE call

The reconcile and external arms now extract the patch out of `turn_handback` immediately after
`publish_into` succeeds. The handback slot keeps its real job — `hand_back_turn`, which is what a
REFUSED turn output returns through — and `stage_emission`/`commit_emission` are unchanged, because
`publish_into` already set `turn_handback_sequence` before the extraction.

**Removes one host round trip per published surface.**

### 3.3 The turn retires its acknowledgements in the SAME turn

`⚛️reactor/🔄️turn/🦀️.rs` runs a second pending-retirement pass immediately after the event loop
(bounded by the same `PATCH_CLOSE_UNITS_PER_TURN` and the same `retirement_deadline` as the first), so
an `Event::PatchAck` this turn carried is retired before the turn decides its status. A run cut short
by the wall deadline still answers `MoreWork` through `has_retiring()` and finishes next turn.

**Removes the second host round trip per published surface.**

### 3.4 The drive stops when it cannot progress, and the arming law is named

`PatchTracker::has_drivable_work()` is `has_publishable_work()` minus the one term that is not the
guest's to advance — a ready output already marked published, whose extraction needs a free publication
slot and therefore the host. The reconcile loop is lifted out of `poll_kernel` into
`drive_reconcile_within(patches, opportunities, deadline)`, which returns the opportunities it spent
and has three stops:

1. **nothing to do** — neither drivable work nor (a ready output AND a free publication slot);
2. **no progress** — an opportunity that neither drove a producer/job nor extracted a ready page;
3. **the hold** — the turn's own wall deadline, re-read every 64 opportunities (unchanged).

and `reconcile_arms_turn(patches)` states the arming law in three terms:

```rust
patches.has_drivable_work()
  || pending.has_undelivered()
  || pending.has_retiring()
  || (pending.has_capacity() && patches.has_publishable_work())
```

What is deliberately absent is the fourth state: **a ready output with no free publication slot**. That
is host-blocked; arming for it bought one round trip per turn that carried nothing.

**Why dropping that term cannot stall**, stated as the argument rather than left to the measurement: if
the authority has no capacity, some slot is occupied, and every phase a slot can be in is covered. If
any of them is `Queued`, `Acknowledged` or `Rejecting`, the turn arms anyway through the first two
terms. If ALL of them are `Issued`, then by definition the host holds those patches and owes their
acknowledgements — `settlePluginTurn` posts them on its next continuation, the guest applies them,
retires them in the same turn (§3.3), and the fourth term's condition (`has_capacity() &&
has_publishable_work()`) becomes true again. The one pathological case it does NOT cover — an
acknowledgement the host drops for a still-live instance — used to present as an infinite `MoreWork`
spin and now presents as a silent stall; `activate_close_instance` already clears `issued` for a
closing instance, so a real teardown still recovers. No stall was observed (7/10 converged against
5/10, 23/23 journey steps, 20/20 mesh oracles), but the changed failure MODE is named here rather than
discovered later.

### 3.5 The worker pumps the guest instead of the host — the silent-turn hold

`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`'s `shardWorkerSource()` gains, on the `turn` path:

```js
const holdDeadline = hopEpochNow() + SILENT_HOLD_MS;   // 8
let holdPolls = 0;
while (
  shardTurnStatusTag(result) === "more-work" && shardTurnCarriesNothing(result) &&
  holdPolls < SILENT_HOLD_POLLS && hopEpochNow() < holdDeadline &&
  actors.get(actorId) === actor && actor.activationGeneration === msg.activationGeneration
) { result = await actor.api.poll([], undefined, undefined, msg.budget); holdPolls += 1; }
```

**Why it is lossless, by construction:** a result is only discarded when `shardTurnCarriesNothing`
admits it, and that predicate names EVERY field of WIT `turn-result` a host reads — `uiPatches`,
`effects`, `presence`, `nextWake`, `lifecycleReceipt`, `uiPatchReceipt`, and both ingress lanes, which
must be `idle`. `fuelUsed` is the one field with no host reader, and that omission is stated rather
than implied. A carrier the predicate forgot would be silently dropped, so the list is asserted
exhaustive in the law.

**Why 8 ms:** its owner is the reactor's own executor hold —
`run_until_deadline(64, 256 KiB, now + 8 ms)`. The guest already promises to hand control back within
that slice, so a worker that re-enters it for at most the same 8 ms adds no new latency class, and a
message queued behind the hold waits at most one hold. `SHARD_TURN_SILENT_HOLD_POLLS = 512` caps a
guest whose turns cost microseconds. Cancellation: the loop re-reads the actor registry and the
activation generation every lap.

The law itself lives outside the generated text, in `🎭️actor/🖼️wire-turn/🟦️.ts`
(`shardTurnCarriesNothingV1`, `driveShardTurnSilentHoldV1` with injected `poll`/`now`/`live`), and the
suite asserts the GENERATED source carries the same loop with the same stop conditions and the same
interpolated constants — so the twin cannot drift silently.

---

## 4. Files changed

| file | change |
|---|---|
| `🧰️framework/…/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs` | `PendingPatchPhase` + `PendingPatchSlot::phase`; `take_one` publishes and hands out in one call; `has_undelivered`/`has_retiring`; `has_unpublished` as the `#[cfg(test)]` partition witness; phase in `debug_state` |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` | `PatchTracker::has_drivable_work()` |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `drive_reconcile_within` (lifted out of `poll_kernel`, with its three stops); `reconcile_arms_turn`; the second post-event pending-retirement pass |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🦀️.rs` | registers the new law module |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🧫️fixtures/🔁️reconcile-spin.json` | **new** — the language-agnostic crossing budget |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-spin/🦀️.rs` | **new law** (6) |
| `🧰️framework/…/🔌️plugin/⚛️reactor/📨️pending/🧪️tests/🩹️receipt/🦀️.rs` | the one-call publication shape |
| `🧰️framework/…/🔌️plugin/⚛️reactor/📨️pending/🧪️tests/🔬️instance-lifetime-patch-close/🦀️.rs` | same |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | same |
| `🧰️framework/…/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` | `SHARD_TURN_SILENT_HOLD_MS`/`_POLLS`; the generated worker's `shardTurnStatusTag`, `shardTurnCarriesNothing` and the hold loop; `timings.holdPolls` |
| `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | `ShardTurnCarriers`, `shardTurnCarriesNothingV1`, `driveShardTurnSilentHoldV1` |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🤫️silent-turn-hold/🟦️.ts` | **new law** (21) |
| `🧰️framework/…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers the new suite |

---

## 5. Laws, with output

### 5.1 Rust — `cargo test -p semio-framework-plugin --lib reconcile_spin` (foreground)

```
running 6 tests
[DEBUG] reconcile-spin surfaces=1 crossings=2 patch=1 events=1 idle=0
[DEBUG] reconcile-spin surfaces=3 crossings=4 patch=3 events=3 idle=0
[DEBUG] reconcile-spin surfaces=8 crossings=9 patch=8 events=8 idle=0
[DEBUG] reconcile-spin host-blocked spent=0 arms=false tracker=slots=[52:surface-0#g1:--R:ack0/rev1:outNone] ready=[g1:p---] …
[DEBUG] reconcile-spin expired-hold spent=64 stride=64
[DEBUG] reconcile-spin cancellation quiesced in 1 turns
test result: ok. 6 passed; 0 failed; 574 filtered out; finished in 0.05s
```

- **`n_pending_retained_surfaces_converge_in_bounded_crossings_that_each_carry_something`** — the
  headline law. N retained surfaces converge in **N + 1** crossings, every one of which carries a patch
  or an acknowledgement, and **`idle = 0`**: not one crossing carries nothing. The budget (2 crossings
  per surface, 0 idle) is declared in `🧫️fixtures/🔁️reconcile-spin.json`, and the loop it drives is
  the production one — `drive_reconcile_within`, `take_one`, `apply_issued_ack`, both retirement
  passes, `reconcile_arms_turn` — not a model of it.
- **`a_published_patch_leaves_the_guest_on_the_turn_that_published_it`** — §3.2, asserted as one call.
- **`a_host_blocked_ready_output_spends_no_opportunity_and_arms_nothing`** — with every publication slot
  `Issued`, a ready output spends **0** opportunities and arms **nothing**. This is the law that fails
  if §3.4's fourth term ever comes back.
- **`the_reconcile_drive_respects_the_turns_wall_hold`** — an expired hold stops the drive inside one
  64-opportunity stride (not at the 1 024 ceiling), and the work resumes on the turns that follow with
  0 idle crossings.
- **`a_cancellation_mid_drive_stops_the_publication_and_still_quiesces`** — a close activated mid-drive
  quiesces in 1 turn and leaves the turn unarmed.
- **`the_pending_publication_phases_partition_what_arms_a_turn`** — walks one publication through
  `Queued → Issued → Acknowledged → ∅` and asserts the partition at every step, including that an
  `Issued` slot arms nothing.

### 5.2 TypeScript twin — `bun ./📜️script.ts test long "silent-turn-hold"` (foreground)

```
 Test Files  1 passed (1)
      Tests  21 passed (21)
```

Nine carrier refusals (one per `turn-result` field), the malformed/absent cases, the six stop
conditions of the drive (`carried`, `idle`, `hold`, `polls`, `closed`, and "never polls a turn that
already carried something"), the kebab/camel status spelling, and the three source-level twin
assertions against the generated worker.

### 5.3 The corpora

| gate | result |
|---|---|
| React engine `test long` | **1 207 passed, 4 failed** (46 files) — the same four pre-existing, peer-owned reds the previous lane recorded: `🎟️resident-refresh-budget`'s fixture census and three `🔬️engine-contract` items. None in a file this lane touched. |
| `cargo test -p semio-framework-plugin --lib reactor:: -- --test-threads=1` | **161 passed, 5 failed**. Run again with `--skip reconcile_spin_tests`: **155 passed, the SAME 5 failed** — this lane's six laws add six passes and no reds. |
| whole `--lib` | aborts in `plugin_builder_contract_tests` and shows 61 reds. Identical with `--skip reconcile_spin_tests`: **peer-owned**, driven by an in-flight `interactive-job-classification` change (`unclassified interactive command 'main:resize'`). |
| `@semio-tech/framework-renderer-wgpu:generate-frame-worker` | green (22.9 s, 4 tasks) |

The five reactor reds, each checked in isolation and each in a file this lane did not touch:
`async_actor_poll_awaits_exchange_and_render_work` (a source-TEXT scan for
`plugin_exchange(runtime, cursor.instance, None).await`, which a peer boxed at `🔄️turn/🦀️.rs:424` and
no longer exists anywhere in the file), `patches::tests::mounted_document_tree_publishes_nested_interactive_rows`
(`SURFACE_RECONCILE_AGGREGATE_BYTES` is 52 559 872 against a fixture that still declares 33 554 432),
`executor::tests::cancel_of_a_parked_task_…`, `m1_m2_reactor_tests::revision_guard_…`, and
`turn_execution_tests::guest_turn_execution_resets_for_every_turn` (order-flaky; green alone).

The one red this lane DID cause and fixed: three suites asserted the old two-call publication shape
(`assert!(pending.take_one(…).unwrap().is_none())`), which is exactly the round trip §3.2 removes.

---

## 6. Before / after, measured

Restage (foreground, no gate):
`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`
→ `🗑️generated/reconcile-spin/restage.txt`, **exit 0**, 10 m 50 s, 34 tasks, "11 completed components
(changed)". The served worker was then verified to carry the change rather than assumed
(`feedback-vite-stale-transform-verify-served-module`):

```
$ curl -s http://127.0.0.1:6021/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js | grep -a "const SILENT_HOLD"
const SILENT_HOLD_MS = 8;
const SILENT_HOLD_POLLS = 512;
```

### 6.1 Crossings and wall

`SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d SEMIO_PROBE_OUT=reconcile-spin/{before,after} bun 🐍️react-hop-cost-probe.mjs`

| | before (`…/before/`) | after (`…/after/`) |
|---|---:|---:|
| **worker crossings per hop** | **110.71** | **79.70** |
| hop wall | 24 057 ms | **14 113 ms** |
| run | 409.0 s, 17 hops, 1 882 crossings | 282.3 s, 20 hops, 1 594 crossings |
| converged | 5/10 | **7/10** |
| `(none)` crossings | 1 515 (89.1/hop), 7.0 ms each | 996 (**49.8/hop**), 12.0 ms each |
| `worker.guest` | 773 ms/hop, 7.0 ms mean | 985 ms/hop, **12.4 ms mean** |
| `worker.reply` | 290 ms/hop | 412 ms/hop |
| `turn.accept` | 294 ms/hop | 556 ms/hop |
| `RecalcStyleDuration` | 0.23 % of wall | 0.80 % of wall (gate: under 5 %) |

Read the `worker.guest` mean the right way round: it **rose** from 7.0 to 12.4 ms per crossing because
each crossing now contains SEVERAL guest turns — that is the hold doing its job — while the number of
crossings fell. The per-hop totals that matter both fell hard: 110.71 → 79.70 crossings and
24.1 s → 14.1 s of hop wall.

### 6.2 Per example, seconds

| step | before s | after s | before meshes | after meshes |
|---|---:|---:|---:|---:|
| boot | 111.0 | **21.7** | 0 | 3 |
| edit:No example | 3.1 | 4.1 | 0 | 0 |
| edit:Hexagonal Mushroom Column | 70.4 | 70.6 | 0 | 3 |
| edit:Rectangle Extrude Volume | 3.1 | 12.2 | 0 | 1 |
| edit:Sphere Cut With Torus | 70.3 | 70.6 | 0 | 1 |
| edit:Box Fillet Preview | 3.1 | 11.5 | 0 | 1 |
| edit:Sphere Box Fuse | 70.5 | **6.2** | 0 | 1 |
| edit:Face Sweep Extrude | 4.1 | 71.1 | 0 | 1 |
| edit:Rectangle Wire Preview | 70.4 | **4.2** | 0 | 1 |
| edit:Box Shell Preview | 3.1 | 10.2 | 0 | 1 |
| **total** | **409.0 s** | **282.3 s** | — | — |

**This per-example table is the weakest evidence in this report and is presented as such.** The "before"
run delivered **zero meshes on every step** — several of its 3.1 s rows are fast because nothing was
rendered, and its 70 s rows are timeouts. Three other lanes landed between the two runs. The honest
reading of §6.2 is the total (−31 %) and the mesh column, not any individual row.

### 6.3 The clean A/B

Because §6.1/§6.2 straddle other lanes' work, the attributable evidence is the guest's own trace under
the identical probe, page and duration — 45 s of boot, `SEMIO_PROBE_GUEST_DIAGNOSTICS=1`:

| | before (19:52, `…/blocker-check/`) | after (23:33, `…/after-morework/`) |
|---|---:|---:|
| reactor turns armed in 45 s | 197 | **335** |
| `reconcile`-armed | 115 | 239 |
| turns with a `:e---p` publication handback | **27** | **0** |
| turns with an `:ea---` acknowledged-awaiting-retirement slot | **27** | **0** |
| `reconcile`-armed turns with an EMPTY publication authority | 61 | 194 |
| `executor_deadline`-armed | 0 | 0 |

The publication ping-pong — 47 % of all reconcile-armed turns, and the entire "every family empty"
population the previous lane named — **is gone**. Every remaining reconcile-armed turn is family 2: a
tracker with real running reconcile jobs and an empty publication authority. The guest ran 70 % more
turns in the same 45 s while the host made fewer crossings, which is the hold.

### 6.4 The journey

`SEMIO_PROBE_URL=… bun 🐍️journey-probe.mjs` → `🗑️generated/reconcile-spin/journey/`:

```
DONE steps 23 meshSteps 20
```

**23/23 steps ran and every mesh oracle is green** — all 20 mesh-bearing rows report `meshOk=true`
with exact triangle counts and bounds inside their fixtures (`eval-extrude@solid#0`,
`eval-brep_bool_cut_5@solid#0`, `eval-fillet@solid#0`, `eval-fuse@solid#0`, `eval-shell@solid#0`,
`eval-rect@wire#0`).

Six rows nonetheless report `converged=false`, and all six fail on **exactly one** clause of the
probe's own predicate: the preview publishes `phase:"idle"`, `ratio:1`, the right meshes and the right
graph — and `computing:true` in the SAME status blob. That is an internally inconsistent status the
guest app emits; a lost or delayed patch would show the OLD blob entirely (`phase:"computing"`), not
this mixture, so it is not a delivery fault. It is also not this lane's: it first appears in
`🗑️generated/flow-inline/journey/results.json` at **23:06**, on the PREVIOUS guest — this lane's
procedural component only landed at 23:21 — and its count varies run to run (2/23 → 4/21 → 6/23) while
every journey before 23:00 shows 0/23. Named, evidenced, not fixed here.

---

## 7. What is left, named with its evidence

**49.8 of the 79.7 crossings per hop still carry nothing.** They are family 2 — a guest with running
reconcile jobs — and three things bound how far this lane could go:

1. **`UI_TURN_PATCHES_MAXIMUM = 1`** (`🎠️kernel/🦀️.rs:1614`). One patch per crossing is the wire
   contract, so N published surfaces cost at least N crossings. The Rust law measures the floor
   exactly: **N surfaces → N + 1 crossings**. The task's ≤ 5 per hop is **below that floor** whenever a
   hop republishes more than about four surfaces, and this renderer publishes ~6. Raising that constant
   is a kernel/wire change with a fixed-capacity `UiTurnPatches`, a host accept loop and a wgpu twin
   behind it — a lane of its own, not a line in this one.
2. **The hold is 8 ms and a guest turn now costs 12.4 ms.** The hold absorbs one extra poll where a
   turn is expensive and hundreds where it is cheap. Raising it is a latency trade against the worker's
   message queue and was deliberately not taken: 8 ms is the reactor's OWN slice, and matching it is
   what makes the hold provably free of a new latency class.
3. **The `heartbeat()` / `heartbeat("turn-step")` pair** the worker posts per crossing is still two
   extra main-thread messages per crossing on a main thread the measurement shows is the binding
   constraint. Unchanged, for the same reason the previous lane gave: it is the shard liveness
   contract, with its own laws and a wgpu twin, and no lane owns it.

---

## 8. The wgpu shell

The wgpu shell's frame Worker runs the same generated shard worker and the same reactor, so both
changes apply to it unchanged. `@semio-tech/framework-renderer-wgpu:generate-frame-worker` is green
(22.9 s, 4 tasks), and `…:activate-generation3d-wgpu-dev` plus
`bun 🐍️wgpu-battery.mjs --only=frame-loop,examples` on **6118** was run behind the standing gate
(`until [ -z "$(pgrep -f 'wgpu-batter[y]|wgpu-.*-pro[b]e')" ]`).

<!--WGPU-RESULT-->

---

## 9. Not claimed

- **The ≤ 5 crossings-per-hop target is NOT met**: 110.71 → 79.70. §7 names the floor
  (`UI_TURN_PATCHES_MAXIMUM = 1`, so ~6 published surfaces ⇒ ~6 crossings minimum) and why this lane
  did not move it. What IS met is the second half of the target on the publication path: the Rust law
  proves N surfaces cost N + 1 crossings with **zero** that carry nothing, and the browser trace proves
  the two-state ping-pong is gone.
- **§6.1 and §6.2 are not a clean A/B of this change.** Three other lanes landed between 19:46 and
  23:30, and the "before" run delivered no meshes at all. §6.3 is the attributable comparison; the
  per-example seconds are reported because they were asked for, with their caveat attached.
- **The journey's six `converged=false` rows are not claimed as fixed and not claimed as caused here.**
  §6.4 gives the evidence for "pre-existing" (a 23:06 run on the previous guest) and the argument for
  "not a delivery fault" (one status blob, internally inconsistent). 23/23 steps and 20/20 mesh oracles
  ARE verified.
- **The 4 React reds and the 5 reactor reds are pre-existing and peer-owned**, each checked with this
  lane's tests skipped and each in a file this lane did not touch. The whole-`--lib` abort is
  reproducible without this lane's tests.
- **`timings.holdPolls` is emitted but not yet read by the host**, so the number of guest turns the
  hold actually absorbs per crossing is inferred from `worker.guest`'s mean (7.0 → 12.4 ms) and from
  the armed-turn count (197 → 335 per 45 s), not read directly. Publishing it as its own span is a
  one-line follow-up for whoever next measures this door.
- **Nothing about the `commandIngress`/`typed_operation` arming families was changed.** They are 34 %
  and 7 % of armed turns and were out of this lane's scope.
