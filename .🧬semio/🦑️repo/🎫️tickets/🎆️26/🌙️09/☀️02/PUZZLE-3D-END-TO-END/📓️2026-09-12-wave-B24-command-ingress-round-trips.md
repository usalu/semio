# 🛰️ Wave B24 — the per-command round trips are the guest's own pacing, not the wire's (2026-09-12)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Answers B22 §6.1 (`📓️2026-09-12-wave-B22-brush-mesh-upload.md`):
*"One page still costs ~84 worker round trips and ~0.64 s on an idle actor … this is the next ceiling."*
Related: W-P3 (`📓️2026-09-09-wave-P3-command-prologue.md`), W-F6 (`📓️2026-09-10-wave-F6-command-pages.md`),
W-R (`📓️2026-09-10-wave-R-intake-delta-cost.md`), B2/B8/B14/B16/B21.

No git write, ticket not opened/closed, nothing deleted under `🗑️generated`, every command foreground,
**no wasm build run**. The cut is entirely **guest-side**, so it is proved by native cargo laws here and
rides the next wasm build; §6 says exactly why it could not be re-measured in the browser this wave.

---

## 1 Attribution — where the 84 round trips actually are

One host↔worker round trip **is** one guest turn: `runQueuedTurn` submits exactly one `submitTurn` per
`postMessage`, and the shard answers one `poll_kernel` per message. So the whole census can be taken
natively, with `poll_kernel` calls standing in for worker messages — reproducibly, in seconds, and
without the browser.

### 1.1 The census, measured

| message kind | count for one 5.7 KB `registerBrushMesh` | bytes it carries | who sets the count | required by the protocol? |
| --- | ---: | --- | --- | --- |
| **command page hand-off** (`turn` + `commandPage`) | **2** (`ceil(5 725 / 4 096)`) | one `FixedCommandPage` ≤ 4 096 B + a 44 B cursor | the wire: `poll_kernel(command_page: Option<…>)` takes **at most one page per turn** | **yes** — one per page |
| **ingress decode moves** (`turn`, no page, no events) | **4** | empty envelope, ~0 B | `plugin_exchange` ran `PluginCommandIngress::step()` **once** and returned `Pending` | **no** — pure pacing |
| **typed-operation units** (`turn`, no page, carrying the previous turn's ACKs) | **≈ 78** | the acknowledgement envelopes of the prior turn, ~0–200 B | `plugin_continue_typed_operations` advanced **exactly one** publication unit per call | **no** — except one per *result page* |
| **patch-ACK lifecycle turn** (`turn` + `issued-ui-ack`) | 1 per published patch (0 for a mesh page) | one `OwnedUiPatchAcknowledgement` token | `acceptUiPatches` | **yes** — the guest's publication capacity is released by its ACK |
| **quiesce tail** | 0 observed, **≤ 128 latent** | empty | `PLUGIN_UI_QUIESCENT_CONTINUATIONS = ceil(1 048 576 / 65 536) × 8` | only while the guest answers `MoreWork` and publishes nothing |

Baselines, both native, both quoted verbatim in §4:

```
a_long_command_stream_never_pins_the_retained_ingress_authority
  → 320 commands, 1282 turns              = 4.006 turns per ONE-page AppCommand::CommandText
retained_operation_continues_after_command_admission_until_publication_and_retirement
  → admitted command published and retired after 48 turns with 3 exact receipts
```

`4 + 48 = 52` for a trivial one-page edit; B22 measured `84.2` for a two-page `registerBrushMesh` whose
staged work (`Puzzle3dPrecomputeCommandWork`, ~23 scan items + the W-P3 prologue stages) runs more units.
**The shape is identical and the dominant term is the same one.**

### 1.2 Why each non-required message exists

**(a) The decode ladder.** `plugin_exchange`'s first statement was `match ingress.step()`, and a `Pending`
answer **returned immediately** with `retry_command` set (`🔌️plugin/🦀️.rs`, the `Some((envelope_seq, ingress))`
arm). Every move of the state machine therefore cost a whole turn:

| move | state |
| --- | --- |
| 1 | `Encoded` → `Decoding(cursor)` |
| 2 | `Decoding` → header byte + seq varint consumed |
| 3 | `Decoding` → `CommandPayload` read (`AppCommand::Command` only) |
| 4 | `Decoding` → `CommandView` read → `Decoded(owner)` |
| 5 | `Decoded` → `Ready` → dispatch |

4 for a `CommandText` (no `CommandView`), 5 for an `AppCommand::Command` — matching the measured 4.006.
Every move is a bounded `read_bounded_bytes` or a bounded page release; **nothing about them needs a turn
boundary.** A cancelled command walked the same ladder one `close_step` (one 4 KiB page) at a time.

**(b) The continuation.** `plugin_continue_typed_operations` called `advance_typed_operation_output`
**once** and returned; the reactor calls it once per turn (`⚛️reactor/🔄️turn/🦀️.rs:1057`). So a staged work
of N units cost N host round trips. The protocol needs a round trip only where the guest hands up a
`TypedOperationResultPage`, because that page is released by the host's own
`plugin_acknowledge_typed_operation_result` — 3 of the 48 above.

**(c) The host adds nothing.** `runQueuedTurn`'s continuation loop and `settleAcknowledgedPluginTurns` →
`settlePluginTurn` submit exactly one turn per guest `MoreWork` (`settlePluginTurn`'s loop condition is
`acknowledgements.length > 0 || status === "more-work"`). It never polls a busy guest, never re-sends an
accepted page, and issues no refresh per continuation. **That is now a law** (§3.3) rather than a reading.

---

## 2 The design

Two changes, both guest-side, both "let one turn drive many units under the grant the turn already
declares" — the B2 pattern, applied to the two places that had not adopted it.

### 2.1 A command assembles, decodes and dispatches inside the turn that carried its last page

`🔌️plugin/🦀️.rs`, region around `PluginCommandIngress`:

* new `PluginCommandIngress::advance(moves)` — loops `step()` while it answers `Pending`, returning the
  first `Ready`/`TerminalFault`, or `Pending` when the grant is spent.
* new `COMMAND_INGRESS_MOVES_PER_TURN = COMMAND_MAXIMUM_PAGES + 8` (72) — **the whole close ladder**:
  64 page releases plus the three decoded field stages plus the header. So a cancelled command also
  retires inside one turn instead of 64.
* `plugin_exchange` calls `advance(COMMAND_INGRESS_MOVES_PER_TURN)` instead of `step()`.

The bound is derived, not chosen: it is the largest number of bounded moves the ingress state machine can
make for any admissible command. Nothing else changed — the same states, the same faults, the same
`retry_command` hand-back when the instance authority is busy.

### 2.2 One turn drives typed-operation units until a bound the protocol itself sets

`🔌️plugin/🦀️.rs`, region `🔁️TypedOperationContinuation`:

```rust
pub struct TypedOperationGrant { units, max_frames, max_effects, deadline }
```

* `TypedOperationGrant::turn(budget)` — the production grant: `units = 256`,
  `max_frames`/`max_effects` from the turn's own `semio_framework::kernel::Budget`, and a deadline of
  `min(budget.deadline_ms, TYPED_OPERATION_SLICE_MS = 8)` ms — **the same 8 ms slice
  `REACTOR_EXECUTOR.run_until_deadline` already gives the executor phase**, so the continuation phase
  costs an interactive turn no more than the executor does.
* `TypedOperationGrant::UNIT` — one unit, for the two laws that step the continuation deliberately.
* `plugin_continue_typed_operations(runtime, grant)` now loops, merging each unit's frames/effects/events
  into one output, and stops at the first of:
  1. **a `typed_operation_result` page** — the host must ACK it before the operation may continue, so
     this is the protocol's own floor and the reason the cut is "one turn per receipt", not "one turn";
  2. the collected output **reaching the turn's declared frame or effect capacity** (the browser stamps
     `maxFrames: 8`, `maxEffects: 64` — `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`'s `poll`);
  3. the **8 ms deadline** (never before the first unit, so progress is guaranteed even with no clock);
  4. the **unit ceiling**, or a runnable unit belonging to a *different* instance — the round-robin
     `typed_continuation_cursor` stays the fairness authority and one turn's output keeps one receiver.

`TypedOperationGrant::spent` deliberately does **not** treat an EMPTY collection as having reached a
zero-frame budget; otherwise a budget declaring `max_frames: 0` would silently restore the exact
one-unit-per-turn pacing this grant exists to remove.

`⚛️reactor/🔄️turn/🦀️.rs:1057` passes `TypedOperationGrant::turn(budget)`.

### 2.3 What was deliberately NOT done, and why

* **Batching a command's pages into one message.** The wire is `poll_kernel(command_page: Option<…>)`;
  making it a list means changing the WIT, the generated component wrapper, `🟨️shard-worker.js`, the
  generated `🌉️bridge.js` and the wgpu `🐚️plugin-bridge.ts`. With no wasm build permitted this wave, the
  host half would ship a `commandPages` the live **#48 guest ignores** — every command would stop
  assembling on `:6013`. Named in §6.
* **Two-deep page pipelining.** It hides one round trip's *latency* per extra page but removes no
  message, and B22's peak-in-flight law is about the mesh **command** queue, not pages. Not worth the
  ordering risk for zero round trips.
* **Batching per-patch `issued-ui-ack` turns.** One turn carries one patch in every path measured here,
  so the batch size would be 1.
* **`PLUGIN_UI_QUIESCENT_CONTINUATIONS = 128`.** It is a latent tail, not a cost paid per command (§1.1):
  it only runs while the guest answers `MoreWork` and publishes nothing, which §2.2 now makes rare.
  Left alone rather than weakened — it is B14's stall attribution.

---

## 3 Changes

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `COMMAND_INGRESS_MOVES_PER_TURN`; `PluginCommandIngress::advance`; `plugin_exchange` uses it; `TYPED_OPERATION_SLICE_MS`; `TYPED_OPERATION_UNITS_PER_TURN`; `TypedOperationGrant` (`turn`/`UNIT`/`spent`/`expired`); `plugin_continue_typed_operations` takes a grant and drives many units |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | the continuation call site passes `TypedOperationGrant::turn(budget)` |
| `…/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs` | `INGRESS_PAGE_SETS`, `INGRESS_TURN_SLACK`, `command_page_authority_driver`, `drive_one_page_set`, and the new law `a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` |
| `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | `TYPED_OPERATION_CONTINUATION_SLACK`, `reactor_native_budget()`, the new continuation-turn bound inside `retained_operation_continues_after_command_admission_until_publication_and_retirement`, and three grant-carrying call sites |
| `…/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs` | one grant-carrying call site (`UNIT` — that law steps deliberately) |
| `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | the new vitest law *spends one worker turn per command page plus the guest's own continuations and none of its own* |

### 3.1 Native law — the page set

`a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` drives 1-, 2-, 4- and 8-page
commands through the real `poll_kernel` with the real `CommandBatchDriver` as the host owner (hand its
next page, `observe` the status that page produced, repeat — the shard's own shape), and asserts
`turns <= pages + INGRESS_TURN_SLACK`. `INGRESS_TURN_SLACK = 1` covers a single instance-authority
contention retry (`plugin_exchange` hands the owner back on `try_lock` `WouldBlock`); the same
non-determinism shows as 320→320..323 turns in the sibling stream law, i.e. under one retry per command.

**It is discriminating**, verified by re-running it against the old pacing (`advance(1)`):

```
a 1-page command spent 4 turns reaching its terminal ingress status (ceiling 2)
  — the ingress is stepping a per-move ladder the host has to drive from outside
```

### 3.2 Native law — the continuation

`retained_operation_continues_after_command_admission_until_publication_and_retirement` now counts the
continuation turns it spends and asserts `spent <= receipts + TYPED_OPERATION_CONTINUATION_SLACK` (slack 1
= the terminating turn on which the scan finally answers "nothing runnable"). Its budget is
`reactor_native_budget()`, which is `DEFAULT_SHARD_BUDGET` lifted exactly as the browser lifts it
(`maxFrames: 8`) rather than a generous test budget.

### 3.3 TypeScript law — the host adds no round trip

*spends one worker turn per command page plus the guest's own continuations and none of its own* drives a
real ~8 KiB `registerBrushMesh` invocation through `loadPluginModule` → `handleAction` →
`AppChannelClient` → `runQueuedTurn` against a real `ShardClient` over a scripted `ShardWorkerLike`,
classifying every `postMessage` of kind `turn` and asserting the sequence is exactly
`[command-page#0 … command-page#(P-1), settle-continuation × K]` for a guest that answers K `MoreWork`s.
It fixes the host's half of the census so a host-side regression cannot hide inside the guest's number.

---

## 4 Laws and their output — foreground, tails quoted

### 4.1 Before / after, measured natively on the same two laws

```
BEFORE  (one-unit pacing restored in place: `advance(1)` + `TypedOperationGrant::UNIT`)
  a_long_command_stream…                 → 320 commands, 1282 turns   (4.006 turns / 1-page command)
  retained_operation_continues…          → published and retired after 48 turns with 3 exact receipts

AFTER
  a_long_command_stream…                 → 320 commands,  320-323 turns over five runs (1.00-1.01 / command)
  a_command_page_set…                    → command page set turns: 1→1 2→2 4→4 8→8
  retained_operation_continues…          → published and retired after  3 turns with 3 exact receipts
```

| | before | after | factor |
| --- | ---: | ---: | ---: |
| ingress turns, one-page `CommandText` | 4.006 | 1.00-1.01 | **4.0×** |
| ingress turns, 8-page command | 11 (derived: 8 pages + the same 3-move ladder) | **8, measured** | 1.4× — the 8 are the wire's own pages |
| continuation turns, 3-receipt staged operation | 48 | 3 | **16×** |
| **turns for one command, ingress + settle** | **52** | **4** | **13×** |
| projected for B22's 5.7 KB `registerBrushMesh` (2 pages + 4 decode + ~78 units) | **≈ 84** | **2 + receipts** | **an order of magnitude** |

### 4.2 The related law set — all green

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 --nocapture \
  a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns \
  a_nakagin_scale_world_publication_reconciles_and_retires_within_a_handful_of_reactor_turns \
  every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable \
  a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot \
  a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work \
  a_status_only_host_call_finishes_every_typed_operation_it_admitted \
  a_deferred_surface_awaiting_the_hosts_acknowledgement_does_not_hold_more_work \
  typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id \
  a_command_page_authority_reserves_only_the_pages_its_command_declares \
  a_saturated_command_page_authority_refuses_without_panicking \
  a_long_command_stream_never_pins_the_retained_ingress_authority \
  a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page \
  retained_operation_continues_after_command_admission_until_publication_and_retirement

  …a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page … [DEBUG] command page set turns: 1→1 2→2 4→4 8→8
  …a_long_command_stream_never_pins_the_retained_ingress_authority … [DEBUG] command page authority: 320 commands, 320 turns, 2 faulted, peak ingress occupancy 0
  …a_mounted_typed_operation_never_parks_a_turn… [DEBUG] … runnable through all 45 continuations of the call that admitted it
  …a_status_only_host_call_finishes_every_typed_operation_it_admitted … [DEBUG] … in 47 continuations, with 1 drain continuations after it
  …every_admitted_typed_operation_slot_is_released… [DEBUG] actual 200 storm actions each released their slot within 512 host continuations, peak occupancy 0
  …retained_operation_continues… [DEBUG] admitted command published and retired after 3 turns with 3 exact receipts
  …a_nakagin_scale_mixed_surface_turn… [DEBUG] mixed-surface retirement completed in 5 turns, against 1092 turns … at the pre-W-B2 one-owner-per-turn pacing
  …a_nakagin_scale_world_publication… [DEBUG] nakagin publication reconciled in 3 turns (2255 steps) and retires in 5 turns / 1092 units
  …a_deferred_surface_awaiting_the_hosts_acknowledgement_does_not_hold_more_work ... ok
→ test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 650 filtered out; finished in 4.49s
```

🧭️ The "45 / 47 continuations" in two of those lines are **app publication units**, not host round trips —
`drive_host_call_with_deferred_acks` drives `PluginApp` directly and never goes through
`plugin_continue_typed_operations`. They are unchanged by this wave by construction.

### 4.3 Whole `semio-framework-plugin` lib suite — zero new failures

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1
  BEFORE (one-unit pacing restored) → FAILED. 518 passed; 144 failed; 0 ignored; finished in 51.02s
  AFTER                             → FAILED. 519 passed; 144 failed; 0 ignored; finished in 47.10s
  set difference (before → after)   → EMPTY, in both directions
```

Both failure lists are in `🗑️generated/b24-plugin-lib-failures-{before,after}.txt`. The 144 are
**peer-owned and pre-existing**, two families, neither touched here:
`app-definition.interactive-job-classification: unclassified interactive command …` and
`app-definition.invalid: app id testkit-dummy must be a canonical surface id`.
The +1 passing is this wave's new law.

### 4.4 B22's own mesh laws — still green

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib \
  -- --test-threads=1 <the 7 B22/W-M2/W-H mesh filters>
  → test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 706 filtered out; finished in 0.67s
```

### 4.5 TypeScript

```
cd …/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bun x vitest run -t "spends one worker turn per command page" --reporter=verbose
  → [DEBUG] command ingress census: 7985B/2p +0cont → 2 worker turns | 7985B/2p +1cont → 3 | 7985B/2p +4cont → 6
  → Test Files 1 passed | 23 skipped (24) / Tests 1 passed | 893 skipped (894)

cd …/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bun x vitest run
  with the new law        → Test Files 3 failed | 21 passed (24) / Tests 9 failed | 885 passed (894)
  with the new law SKIPPED→ Test Files 3 failed | 21 passed (24) / Tests 9 failed | 884 passed | 1 skipped
  set difference          → EMPTY, in both directions
```

The 9 are peer-owned and listed in `🗑️generated/b24-renderer-react-vitest-failures.txt`: 6 in
`🧩️package-integration` (the pinned-Bun / generated-worker toolchain gate), 1 `noteShellCommand` in
`🔬️engine-contract` (B22 §4.3 recorded the same one), and 2 in `🔌️PluginRuntime` that fail identically
with this wave's law skipped. **This wave wrote no production TypeScript at all.**

```
cd …/🎯️targets/⚛️react && bun x tsc --noEmit
  → 852 errors repo-wide. ZERO in 🧪️tests/🔌️plugin-runtime/🟦️.tsx (the only TS file this wave wrote);
    the 2 reported in 🔌️PluginRuntime/🟦️.tsx are lines 2473 and 2644, a file this wave never touched.
```

### 4.6 Cargo gates

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
  → Finished `dev` profile [unoptimized] target(s) in 24.84s        (0 errors; 88 pre-existing warnings)

cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
  → Finished `dev` profile [unoptimized] target(s) in 11.19s        (0 errors)
```

🐛️ At 21:42 the wasm-target check failed with `error[E0382]: borrow of moved value: brush_preview` in
`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:660` — a live peer's in-flight edit (file mtime one minute
old, two statements in the wrong order). Not touched here; the peer landed the fix at 21:48 and the
re-poll above is green. Recorded so a later gate reader does not attribute it to this wave.

---

## 5 Before / after in the browser

**Not re-measured, and the reason is structural, not a skipped step.**

1. The cut is entirely guest-side (§2). `:6013` serves wasm **#48**, which predates both changes, so a
   probe run there would reproduce B22's ~84 and nothing else — the "after" reading needs #49+.
2. This wave's **host** half is one new vitest law and zero production TypeScript, so the host's
   contribution to the census is provably unchanged (§3.3, §4.5): `pages + guestContinuations`, before
   and after.
3. `:6013` is held by the coordinator's 18-lane battery (`🔍️browser-probe.ts --only=… --port=6013`
   inside an `until pgrep` loop writing `🗑️generated/lanes-2026-09-12-48.txt`). Taking the lane would
   have raced that loop's own guard for a reading known in advance to be the B22 baseline.

So the numbers that carry are §4.1's, and they are native, reproducible in seconds, and discriminating
(§3.1 shows the law failing against the old pacing). The browser "after" is the first item in §6.

---

## 6 Residual — named, not hidden

1. **The browser re-measure rides the next wasm build.** On #49+, B22's probe
   (`🔍️b22-mesh-probe.ts`, `workerTurnsPerMeshCommand`) should read `2 + receipts` per
   `registerBrushMesh` instead of 84.2, and the uncontended per-command p50 should fall from 0.643 s
   toward the page hand-off plus the staged work's own microseconds. **Nothing else in this wave is
   unverified** — run it and quote it; if it does not move, the guest half did not ship.
2. **One page per turn is still the wire.** `poll_kernel(command_page: Option<…>)` caps a command at one
   page per round trip, so a 64-page command still costs 64. Lifting it to a page *set* is a WIT +
   generated-wrapper + `🟨️shard-worker.js` + `🌉️bridge.js` + `🐚️plugin-bridge.ts` change that CANNOT be
   half-shipped: a host sending `commandPages` to a guest that reads `commandPage` never assembles a
   command. Do it in a wave that owns a wasm build.
3. **The quiesce tail is still 128.** `PLUGIN_UI_QUIESCENT_CONTINUATIONS` is only paid while the guest
   answers `MoreWork` while publishing nothing; §2.2 makes that rare but does not remove the bound.
   B14 owns its attribution — weaken it only with a measurement that says which guest still needs it.
4. **B22 §6.2 is still open.** `Puzzle3dPrecomputeCommandWork` claims ~23 work items to walk a 5 464-char
   base64 string 512 characters at a time and the reducer then decodes the same string again
   (`✏️editor/🦀️.rs:6747-6794`). After this wave those items no longer cost a round trip each — they cost
   part of one turn's 8 ms slice — so the ceiling is far lower, but the double decode is still there.
   `✏️editor/🦀️.rs` was under live peer rewrite throughout this wave.
5. **The pull lane (B22 §6.3) is untouched**: `Effect::HttpRequest` + the parked-future
   `RequestRegistry::request` remains the only host→guest path with no 8 KiB cap, and still needs an
   `http-request` case in `wireEffectToFriendly` plus a guest `AsyncTask`.
6. **144 peer-owned Rust failures and 9 peer-owned vitest failures** (§4.3, §4.5) are listed so #49's
   gate reader does not attribute them here.
