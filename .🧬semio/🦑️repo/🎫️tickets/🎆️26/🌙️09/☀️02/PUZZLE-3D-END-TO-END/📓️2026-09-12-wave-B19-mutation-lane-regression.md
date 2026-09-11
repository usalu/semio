# Wave B19 — the "#46 → #47 mutation-lane regression" is a probe-shape confound, and the real defect is B14 §6

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B19, 2026-09-12. Assignment: bisect the mutation-lane
regression between B14's #46 proof and wasm #47 (suspects B15 `focusedWindowId`, B16 scheduling, B17 F3,
B10 `dispatch_emit` guard). No git write, ticket not opened/closed, `🗑️generated` written to and never
deleted, **no wasm build run** (none was needed), every command foreground, every `[DEBUG] b19` tap
removed before finishing.

**Headline, and it inverts the brief.** There is **no mutation-lane regression between #46 and #47**. The
lanes the coordinator read as "RED again" are red in the FULL BATTERY on #45b, #46 and #47 *identically*,
verdict for verdict; and B14's PASS was never a battery reading — it was a 3-step `--only=` run, which
**still passes on #47, and passes harder**: `duplicate-selection`, `duplicate-reselects-clone`,
`volume-brush-add-target-volume` and `volume-brush-voxel-dims` all flipped FAIL → **PASS** on #47. None of
B15/B16/B17/B10 needs neutralising, and **no coordinator rebuild (#48) is needed for anything in this
wave**.

What is real is the defect B14 handed over in its own §6 and B16 tried to close: **a mutation's edit is
published one command late**. It is still there on #47 — measured in the browser this pass, with the
continuation counts — and it is what makes every one of those lanes red as soon as the next command is far
away, which is exactly what a 20-minute battery guarantees and a 20-second `--only=` run does not.

---

## 1 The hop table — full battery, three builds, same verdicts

`--battery` (no `--only`), `:6013`, fresh page, FAULTS=0 in all three.

| verdict | #45b `probe-…T14-48-04` | #46 `probe-…T15-13-33` | #47 `probe-…T16-46-55` |
| --- | --- | --- | --- |
| `catalogue-add-object-kind` | FAIL `before=1 after=1` | FAIL `before=1 after=1` | FAIL `before=1 after=1` |
| `catalogue-drag-drop` | FAIL `before=1 after=1` | FAIL `before=1 after=1` | FAIL `before=1 after=1` |
| `duplicate-selection` | FAIL `before=1 after=1` | FAIL `before=1 after=1` | FAIL `before=1 after=1` |
| `delete-selection` | FAIL `before=1 after=1` | FAIL `before=1 after=1` | FAIL `before=1 after=1` |
| `volume-brush-arm` | FAIL `activeUtility=select` | FAIL `activeUtility=select` | FAIL `activeUtility=select` |
| `volume-brush-add-target-volume` | FAIL `before=0 after=0` | FAIL `before=0 after=0` | FAIL `before=0 after=0` |
| `engagement-brush-verb` | FAIL `activeUtility=select` | FAIL `activeUtility=select` | FAIL `activeUtility=select` |
| `inspection-object-fields` | FAIL `id=null` | FAIL `id=null` | FAIL `id=null` |
| `context-menu-selection-precondition` | FAIL `outlinerRows=1 selected=0` | FAIL `outlinerRows=1 selected=0` | FAIL `outlinerRows=0 … surfaceStatus=["1=","1="]` |
| `undo-unwind` | **PASS** | **PASS** | **FAIL** `example=Nakagin` |

Nine of the ten lanes named in the brief are **byte-identical across all three builds**. The tenth
(`undo-unwind`) is the only genuine full-battery flip — §4.

## 2 B14's own proof command, replayed on #47 — better than #46

B14 §5's command verbatim:
`bun 🔍️browser-probe.ts --only=catalogue-panel,volume-brush,selection-keybindings --reload-between-groups --port=6013`
→ `🗑️generated/b19-b14-replay.txt`, `probe-2026-09-11T17-33-35.md`,
`done booted=true faults=0 hard=0 collateral=0 verdicts=34`.

| verdict | B14, #46 (`probe-…T15-50-57`) | **B19, #47** |
| --- | --- | --- |
| `catalogue-drag-drop` | PASS | **PASS** |
| `catalogue-add-object-kind` | FAIL `before=1 after=1` | FAIL `before=1 after=1` |
| `duplicate-selection` | FAIL `before=2 after=2` | **PASS** |
| `duplicate-reselects-clone` | — | **PASS** |
| `delete-selection` | FAIL `before=4 after=4` | FAIL `before=4 after=4` |
| `volume-brush-arm` | PASS | PASS |
| `volume-brush-target-volume-attribute` | PASS | PASS |
| `volume-brush-add-target-volume` | FAIL `before=0 after=0` | **PASS** |
| `volume-brush-voxel-dims` | FAIL `before=null after=null` | **PASS** |
| `focus-selection` | — | FAIL (camera unchanged) |

And an isolated `--only=catalogue-panel --port=6013` on #47 (`🗑️generated/b19-repro-1.txt`,
`probe-2026-09-11T17-16-57.md`) reproduces B14's shape exactly:

```
[11.5s] verdict catalogue-add-object-kind FAIL before=1 after=1
[15.1s] catalogue drag-drop {"ran":true,…} before={"count":1,"ids":["seed-left-001"]} after={"count":2,"ids":["seed-left-001","puzzle3d.object.c05b8dbd798d5454"]}
[15.1s] verdict catalogue-drag-drop PASS
```

The click's add lands during the DROP step's window — B14 §6.3's reading, unchanged. So the comparison in
the brief ("B14's #46 short run" vs "#47 full battery") was measuring the probe shape, not the build.

## 3 The real defect, measured — the admitting call hands back nothing

Temporary taps (added, run, **removed**; `rg -a "b19"` over `🔌️PluginRuntime/🟦️.tsx` → no match): one
`[DEBUG] b19 settled` per `settlePluginTurn` (call label, continuations, quiesced, ms, status, patches,
effects) and a `[DEBUG] b19 dispatch settled` per `performInvocation`. From
`probe-2026-09-11T17-16-57.md`, in order, one `addObjectKind` click:

```
performInvocation {"actionId":"addObjectKind"}
command ingress lane {"actionId":"addObjectKind","seq":32,"lane":"Interactive"}
b19 settled {"call":"puzzle#1:command#1","continuations":23,"quiesced":false,"ms":443,"status":"idle","patches":0,"effects":5}
b19 dispatch settled {"actionId":"addObjectKind","ms":929}
performInvocation settled {"actionId":"addObjectKind","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyUpserts":0,"historyCanUndo":null}
…
b19 settled {"call":"puzzle#1:command#1","continuations":70,"quiesced":false,"ms":2816,"status":"idle","patches":0,"effects":4}
history patch applied {"replace":false,"currentCursor":3,"patchCursor":4,"upserts":1,"labels":["create-object object { id=puzzle3d.object.c05b8dbd798d5454 …"]}
b19 dispatch settled {"actionId":"registerBrushMesh","ms":12469}
```

The add's OWN settle runs 23 continuations, reaches `status=idle`, emits 5 effects and hands the caller
`["Invocation","Ephemeral"]` with `historyUpserts:0` — **no `OperationCompleted`, no history row, no world
delta**. The `create-object` row arrives inside the NEXT command's settle (a `registerBrushMesh`, 70
continuations later). That is B14 §6 verbatim, on #47, *after* B16.

Settle census for the whole run (35 settles): `command#1` 18× (0–73 continuations, 2 of them quiesced at
132), `refresh-ui#1` 9× (10–22), `job-completion#1` 3×, `operation-drain#1` 1×. Identical to B14's #46
census ("18 command settles 0–71 continuations, 10 refresh-ui settles 10–23, 3 job-completion settles 2–4,
exactly 2 quiesced at 128/132") — **B16 changed neither the continuation budget nor the quiescence rate**,
so hypothesis (b) ("`more-work` forever" / "`idle` too early") is measured out as a *count* effect; what
survives is that the ladder's terminal step lands on a later call.

### 3.1 Why the battery is red where the short run is green

`registerBrushMesh` dispatches take **5.6 s / 7.8 s / 12.5 s / 13.4 s / 14.7 s** on a fresh page
(`b19 dispatch settled`), 18 of them per session, and `client.command` is serialised per actor. The probe
gives each mutation verdict a 3.5 s window. On a fresh page the following command arrives inside that
window often enough for `catalogue-drag-drop` / `duplicate-selection` to pass; 880 s into a battery the
queue is deep enough that it never does. Measured on the #47 battery's own console tail (last 1 203
lines): **74 invocations started, 59 settled, and of those settles 54 name `interactionHover` while only
one hover was even ingressed in that window** — i.e. the settles draining there belong to calls started
long before. `undo` ×24, `engagementAbort` ×23, `duplicateSelection` ×2, `deleteSelection` ×2 and
`addObjectKind` ×1 started in that window and **none of them settled before the run ended**. On the #46
battery's tail the same counters are balanced (34 started / 35 settled, matching by action id).

So the lanes are not binary-broken; they are **latency-gated by the one-command lag**, and the battery is
the load that exposes it. This is the thing to fix, and it is guest-side.

### 3.2 Suspects, individually cleared

| suspect | verdict | evidence |
| --- | --- | --- |
| (a) B15 `focusedWindowId` | **cleared** | The action-dispatch path always stamps `windowId` — `dispatchWindowId = actionWindowId ?? activeWindowIdRef.current` fed through `hostArmedViewContext` (`🏛️ShellHost/🟦️.tsx:5980-5987`) — so the guest's `puzzle3d_addressed_window_id` never reaches its `focused_window_id` hop for a dispatch; the hop only changes PANEL rendering. And the lanes B15 is accused of breaking (`duplicate-selection`, `volume-brush-*`) flipped to **PASS** on #47 (§2). |
| (b) B16 scheduling | **cleared as a regression; not effective as a fix** | Continuation census identical to #46 (§3). No `more-work` spin, no early `idle` beyond the pre-existing one. But the lag B16 targeted is still there, and §5 shows why its laws did not catch it. |
| (c) B17 F3 `reloadPlugin` committed handle | **cleared** | Zero `hot-swap` / `activation.revoked` / `reactor-close-authority` lines in any run this pass (`faults=0 hard=0 collateral=0 guest-death-faults=0` on all four runs). |
| (d) B10 `dispatch_emit` guard | **cleared** | The guard sits inside `if artifact_mutations.is_empty()` (`🔌️plugin/🦀️.rs:21469`) and additionally requires `published_window_config && config_edit_id.is_none() && matches!(kind, ActionKind::View)`. A mutation carries artifact ops and never reaches it, so it can suppress neither a mutation's history row nor its `OperationCompleted`. |

## 4 `undo-unwind` — the one genuine full-battery flip

PASS on #45b (962.9 s) and #46 (1072.5 s), FAIL on #47 (1068.8 s, `example=Nakagin Capsule Tower`). The
#47 console tail shows the cause directly: the step presses undo 12 times, **24 `undo` invocations were
dispatched and not one of them settled** before the run ended (§3.1). This is the same backlog, not a
second defect: `undo-redo`'s PASS on the same run is vacuous (the example was still Nakagin, which is what
that verdict asserts).

**Re-proved in isolation on #47** — `--only=example-switch,history-open,undo-unwind,undo-redo --undo
--port=6013` (`🗑️generated/b19-undo-2.txt`, `probe-2026-09-11T17-37-04.md`, `faults=0 hard=0
collateral=0`):

```
[216.1s] example after switch: Nakagin Capsule Tower
[216.1s] verdict example-switch PASS
[237.2s] unwind done: example=Concrete Forest census={"chrome":0,"replay":2}
[237.2s] verdict undo-unwind PASS
[250.5s] redo done: example=Nakagin Capsule Tower
[250.5s] verdict undo-redo PASS
```

Three undo presses unwound the switch in 21 s on a fresh page. So `undo-unwind` is **not broken on #47**
either — it is the same latency gate as §3.1, and it is the tenth lane, not a tenth defect.

## 5 The law — and what it found about B16's two laws

`🔌️plugin/🦀️.rs` `test_typed_operation_lands_its_edit_inside_the_admitting_call`, entry point
`an_admitting_host_call_hands_back_the_terminal_lane_it_earned` in
`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`, beside B16's two. It pins the browser sequence
natively: **the host call that ADMITS a typed command is the call that hands its terminal lane back**, and
a `Terminal` page owes exactly one completion witness in that same call (the witness is what carries the
`history_patch` the renderer turns into an `OperationCompleted` frame).

Written first and run, it immediately reported something worse than expected:

```
the admitting call of 33 continuations (parked at None) handed back no terminal completion for
'compositeEdit'; it left 0 of 64 typed-operation slots live after 33 host continuations []
```

`parked=None`, zero slots live, zero completions. Adding a lane tap to
`drive_host_call_with_deferred_acks` named it:

```
[DEBUG] b19 page c=28 lane=Fault body=test count accepts exactly one scalar mutation
```

> **Wave B16's two laws are green over a ladder that never publishes anything.** The
> `KeyedTestApp`/`compositeEdit` fixture both of them drive is refused at
> `TestCountOneItemPreparationFactory::preflight`
> (`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:169`) and terminates on the `Fault` lane with
> no completion witness at all. `a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work`
> and `a_status_only_host_call_finishes_every_typed_operation_it_admitted` assert only that no continuation
> is PARKED and that nothing is left pending — both of which a refused ladder satisfies trivially. **B16's
> fix has therefore never been proven against a mutation that actually lands**, which is consistent with
> the browser still showing the lag on #47.

The law is written as an equivalence (`completions == (lane == Terminal)`) so it is honest and green today
— it NAMES the refusal in its own `[DEBUG] actual` line instead of hiding it — and becomes the completion
pin the moment the fixture reaches `Terminal`:

```
[DEBUG] actual the admitting call handed back the Fault lane ("test count accepts exactly one scalar
mutation") and 0 completion witness(es) within 35 continuations
test component::plugin_runtime::plugin_builder_contract_tests::an_admitting_host_call_hands_back_the_terminal_lane_it_earned ... ok
```

`drive_host_call_with_deferred_acks` now returns a named `HostCallCensus`
(`completions`/`spent`/`parked`/`terminal`) instead of a 3-tuple; B16's two call sites were updated in
place and neither law's assertions were touched.

**Next hop, and it is the whole fix.** Repair the `KeyedTestApp` `compositeEdit` fixture so its ladder
reaches the `Terminal` lane (the refusal is `description.is_some()` / non-`SetCount` at that `preflight`);
the new law then turns red on exactly the browser defect, B16's two laws become non-vacuous, and the
one-command lag can be fixed and proven without a wasm build. That is guest-crate work in
`🔌️plugin/🦀️.rs` + its contract test — no browser, no deploy.

## 6 Verification (foreground, tails quoted)

| command | tail |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib typed_operation -- --test-threads=1` | `test …::a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work ... ok` / `…::a_status_only_host_call_finishes_every_typed_operation_it_admitted ... ok` / `…::a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot ... ok` / `…::every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable ... ok` / `…::typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id ... ok` — `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 657 filtered out; finished in 0.49s` |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib an_admitting_host_call -- --test-threads=1 --nocapture` | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 661 filtered out; finished in 0.02s` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings` / `Finished dev profile [unoptimized] target(s) in 14.77s` — **0 errors** |
| `cargo check -p semio-framework-plugin --lib` | `warning: semio-framework-plugin (lib) generated 6 warnings` / `Finished dev profile … in 0.29s` — **0 errors** |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/🎯️targets/⚛️react/vitest.config.ts -t "settle"` | `Test Files  4 passed \| 20 skipped (24)` / `Tests  12 passed \| 874 skipped (886)` — identical to B14 §4's post-change reading; the host is back at B14's state, no new failure |
| `bun x tsc --noEmit -p …/🎯️targets/⚛️react/tsconfig.json` | 852 errors repo-wide (peer baseline). In `🔌️PluginRuntime/🟦️.tsx` exactly the same **two pre-existing** errors B14 §7 recorded, at the same lines: `(2473,52) … 'req' does not exist in type 'FaultScope'` and `(2644,111) … Property 'Invocation' does not exist on type 'AppFrameValue'`. **No new error.** |
| probes | `b19-repro-1.txt` (`--only=catalogue-panel`) `battery PASS=7 FAIL=2 FAULTS=0`; `b19-b14-replay.txt` (B14's command) `done booted=true faults=0 hard=0 collateral=0 verdicts=34`; `b19-undo-2.txt` (`--only=example-switch,history-open,undo-unwind,undo-redo --undo`) `done booted=true faults=0 hard=0 collateral=0 verdicts=18`, `undo-unwind PASS` + `undo-redo PASS`; `b19-undo-1.txt` (the same set WITHOUT `--undo` — vacuous, see §8.4). All on `:6013`, each started only with `pgrep -f "browser-probe\|lane-probe\|b19-"` empty. |

No wasm build was run: the deploy budget was not needed, because no guest change was made and the
regression did not exist. **No #48 rebuild is required for this wave.**

## 7 Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `HostCallCensus`;
  `drive_host_call_with_deferred_acks` returns it and records the terminal lane + body;
  `test_typed_operation_lands_its_edit_inside_the_admitting_call`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  — `an_admitting_host_call_hands_back_the_terminal_lane_it_earned`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` —
  temporary `[DEBUG] b19` settle/dispatch taps only; **added, run, removed**, file back to B14's state.
- `🗑️generated/` — `b19-repro-1.txt`, `b19-b14-replay.txt`, `b19-undo-1.txt`, `b19-undo-2.txt` and their
  `probe-2026-09-11T17-{16-57,18-49,33-35}.md/.ndjson`.

## 8 Handover

1. **Do not chase a #46 → #47 mutation regression — there is none.** Any wave descoped on that reading
   should be re-opened. §1 and §2 are the evidence.
2. **A battery verdict on a mutation lane is a latency measurement, not a correctness one**, until the
   one-command lag is closed. Compare like with like: a battery against a battery, an `--only=` run against
   the same `--only=` run.
3. **B16's two laws are vacuous** (§5) and must be repaired before anyone claims the lag is fixed.
4. `--only=` takes STEP names, not verdict names (`catalogue-panel`, `selection-keybindings`,
   `undo-unwind`, `selection-surfaces`), and `example-switch` only switches to Nakagin — and only then
   emits its `example-switch` verdict — when `--undo` is passed. An isolated `--only=…,undo-unwind` run
   without `--undo` is vacuous. Worth hardening (B18's file).
