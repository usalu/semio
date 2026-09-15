# 🧺 One turn, every ready patch — removing `UI_TURN_PATCHES_MAXIMUM = 1` (lane `ui-turn-patch-batching`, 2026-09-15)

Opus lane on port **6021** (`📜️serve-generation3d-react-6021.sh`, host TS vite-live, `SEMIO_VITE_HMR=0`)
and **6118** for the wgpu twin. Continues `📓️reactor-reconcile-spin-2026-09-14.md` §7, which named this
cut and could not take it:

> **`UI_TURN_PATCHES_MAXIMUM = 1`** (`🎠️kernel/🦀️.rs:1614`). One patch per crossing is the wire
> contract, so N published surfaces cost at least N crossings. […] Raising that constant is a
> kernel/wire change with a fixed-capacity `UiTurnPatches`, a host accept loop and a wgpu twin behind
> it — a lane of its own, not a line in this one.

That lane is this one. The floor is gone: a turn result now carries **every** patch that is ready,
under a per-turn BYTE budget instead of a count, and the host acknowledges the whole batch in ONE
crossing. What that is worth on this renderer's steady-state hop is measured in §5, and it is much
less than the floor's removal promised — for a reason §7 names with its evidence.

---

## 0. The headline, and the honest half of it

> **The wire contract is fixed and proven: N surfaces that are ready together now converge in TWO
> host round trips (one carrying every patch, one carrying every acknowledgement) instead of N + 1,
> with zero crossings that carry nothing. On the live React door this cashes out as 4 % — because the
> guest rarely HAS more than one surface ready at the same time, and the crossings that dominate a
> hop were never the patch-carrying ones.**

| | before (00:33) | after (01:49) |
|---|---:|---:|
| Rust law: crossings for N ready surfaces | N + 1 | **2** (N = 1, 3, 6, 8) |
| worker crossings per `flowEvalTick` hop | 58.60 | 58.25 |
| of those, `patch-ack` crossings | 238 (11.9/hop) | **191 (9.6/hop)** |
| ui patches carried | 238 | 237 |
| patches per patch-carrying crossing | 1.00 | **1.24** (boot burst: up to **9**) |
| hop wall | 2 770 ms (load 22–26) | 2 642 ms (load 7.7) |
| whole 10-step run | 55.4 s, 10/10 converged | **52.8 s, 10/10 converged** |
| journey | — | **23/23 converged, 16/16 mesh oracles green, 0 red** |

Read the hop wall with its load column, not against it: the two runs straddle a 3× difference in
machine load (§5.3). The load-independent number is the crossing count, and it moved **58.60 → 58.25**.

---

## 1. What the cap actually was

`semio_framework::kernel::UI_TURN_PATCHES_MAXIMUM` was the literal `1`, and three layers enforced it:

| layer | how it capped |
|---|---|
| `🎠️kernel/🦀️.rs` | `UiTurnPatches` held ONE `UiTurnPatchContents`; `try_push_ui_patch` refused the second patch |
| `🎭️actor/🚪️lifetime/🩹️patch/🦀️.rs` + `🟦️.ts` | `validate_pairing`/`validateActorUiPatchPairing` accepted only `(0, None)` and `(1, Some)` |
| `⚛️reactor/📨️pending/🦀️.rs` | ONE `turn_handback` cell; a second `take_one` in the same turn answered `"pending patch turn is already borrowed"` |

and the WIT `turn-result` already declared `ui-patches: list<ui-patch>`. **The wire was never the
constraint — the owners on both ends of it were.** That is why this lane needed no WIT change at all.

The host side had a matching, invisible cap: `PluginRuntime.acceptUiPatches` and the wgpu bridge's
`accept` both looped over `turn.uiPatches.entries()` already, but each iteration submitted its OWN
lifecycle turn carrying ONE `patch-ack`. Batching the guest's half alone would have moved the N round
trips it stopped paying straight onto the inbound side.

---

## 2. The design — the count cap becomes a byte budget

### 2.1 The budget is read off the declared guest memory budget, never chosen

Same authority the `contributions-ingress-ceiling` lane used (`🧮️memory/🦀️.rs`):

```rust
pub const UI_TURN_PATCH_OWNER_BYTES: usize = 4_096;
pub const UI_TURN_PATCHES_MAXIMUM: usize = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES;   // 16
pub const UI_TURN_PATCH_BUDGET_BYTES: usize = GUEST_HOST_ANSWER_CEILING_BYTES / 4;                               // 2 097 152
```

* **`UI_TURN_PATCHES_MAXIMUM` is no longer an admission rule** — it is the fixed CAPACITY of the page.
  It is derived because the page is parked BY VALUE in every one of the 64 transport slots and their
  handbacks, so the page's own extent is the single block the guest's `dlmalloc` granularity has to
  fund. `UI_TURN_PATCH_OWNER_BYTES` is the inline extent of one exact patch owner — the number the
  kernel already asserted as a literal `4096` and now names.
* **The admission rule is the per-turn BYTE budget.** It is not a new constant: the host already
  declares it per lane on every `poll` as `Budget::max_patch_bytes`, and already enforces it on the
  way back (`🖥️plugin/🖥️host/📥️ui-patch/🦀️.rs`). `UI_TURN_PATCH_BUDGET_BYTES` is the CEILING every
  lane's declaration must stay under, and `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` the floor it must
  clear; `turn_patch_budget_bytes` clamps the declaration into that window.
* **Each patch is priced by `ui_patch_turn_bytes`** — its wire envelope (32 B), its surface name, one
  operation envelope (16 B) per operation, and the physical backing its operation storage holds. The
  last term makes it an UPPER bound of the host's own `patch_wire_bytes`, so a batch the guest admits
  can never be a batch the host refuses.

### 2.2 The three admission clauses

`⚛️reactor/🔄️turn/🦀️.rs`'s `take_turn_patch_page(budget_bytes)`:

1. the **first** ready patch is always admitted, whatever it costs — otherwise a publication larger
   than the whole budget could never leave the guest and its surface would never converge;
2. every further patch joins only while the page's own `ui_patch_turn_bytes` sum stays inside the
   budget — the one that does not fit is handed straight back to its reserved slot and travels on the
   next turn, in order, still exactly once;
3. the page's fixed capacity stops the loop, and that capacity is derived so the page never becomes a
   contiguous request past the guest's ceiling (28 304 B of 65 536 B at `UI_TURN_PATCHES_MAXIMUM = 16`,
   printed by the law).

### 2.3 One receipt authorizes the whole batch

`ActorUiPatchReceipt::validate_pairing` becomes `(0, None) | (n ≥ 1, Some(valid))`. A patch inside the
batch is addressed by its own `(receipt, surface, revision)` — which is exactly how
`PendingPatchAuthority::apply_issued_ack` already discriminated, so no new addressing was invented.
The batch is therefore **single-instance** by construction: `take_one` only adds to a batch that
already names an instance, because one `ui-patch-receipt` names one lifetime.

The host's `captureInstanceUiPatch` duplicate-sequence guard is widened by exactly that much: the same
receipt may be captured once per patch OF THE SAME TURN (`owner.lastPatchTurn`), and is still refused
on any other turn.

### 2.4 The pending authority's handback becomes a page

`turn_handback`/`turn_handback_sequence` become `turn_handbacks: [TurnPatchCell; UI_TURN_PATCHES_MAXIMUM]`.
Cells are borrowed in ascending index and a refused output is drained in publication order, so **cell
`i` always belongs to the `i`-th patch of the turn** — the pairing is positional and no surface
identity has to be re-derived to return a patch to its own slot. `stage_emission` stages the one
receipt against every borrowed cell positionally and refuses if any borrowed publication is left
unstaged; `commit_emission` commits them all and frees every cell.

### 2.5 The host acknowledges the batch in ONE crossing

`ShardInstanceLifecycleLease.submitUiAcknowledgements(entries, budget)` sends N `patch-ack` events in
one `sendInstanceLifecycle` — `reactor::poll` takes a LIST of events, and the guest retires all N in
the same turn. The one-patch `submitUiAcknowledgement` is now literally a batch of one, so both paths
admit, cross and memoize through the same code.

`PluginRuntime.acceptUiPatches` and the wgpu bridge's `WgpuOwnedUi.accept` were reshaped the same way:
drive every patch's intake to its acknowledgement token FIRST, submit one `issued-ui-acks` lifecycle
turn, then finish every intake against its own submission receipt.

### 2.6 The readers that assumed "index 0"

`captureBrowserActorUiPatchV1` (the browser store worker's document actor) read `patches[0]` and
demanded it name the reader's own surface. A turn that publishes every ready surface would have made
it throw on any batch whose first patch belonged to someone else, so it now SELECTS the patch its
surface owns out of the batch and still refuses a foreign instance.

The native host bridge (`wit_ui_patches_to_kernel`) moved the whole emitted-first batch instead of
`…next()`. Its fixture's `both-channels` row was the count cap standing in for a real invariant — a
component may not both import and return patches — so that invariant is now stated on its own and the
`maximum-plus-one` row moved to `UI_TURN_PATCHES_MAXIMUM + 1`.

---

## 3. Files changed

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` | `UI_TURN_PATCH_OWNER_BYTES`, derived `UI_TURN_PATCHES_MAXIMUM`, `UI_TURN_PATCH_BUDGET_BYTES`, `ui_patch_turn_bytes`; `UiTurnPatches` becomes an ordered fixed page (`entries`/`length`/`cursor`) with per-entry retirement, order-preserving `try_transfer_one`, a draining `IntoIterator`, and a `size_of` law against the contiguous ceiling |
| `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🦀️.rs` + `🟦️.ts` + `🧫️fixtures/🔣️.json` | the pairing law becomes `(0, None) \| (n ≥ 1, Some)` in both twins and in the shared fixture |
| `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` | `OwnedUiPatchAcknowledgementEntry`, `submitUiAcknowledgements`, `admitInstanceUiAcknowledgement`/`patchAckEvent` shared by both paths, per-turn duplicate-sequence guard |
| `🧰️framework/…/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs` | `TurnPatchCell` page, batched `take_one`/`hand_back_turn`/`stage_emission`/`commit_emission`, `close_instance_step` over the page, `borrowed_sequences`, new `debug_state` |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `turn_patch_budget_bytes`, `take_turn_patch_page`, `fill_turn_patch_page`; the refusal path returns the WHOLE page |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🦀️.rs` | registers the new law module |
| `🧰️framework/…/🔌️plugin/🖥️host/📥️ui-patch/🦀️.rs` (+ fixture, + unit law) | admits the whole batch; the imported/returned channel invariant is stated on its own |
| `🧰️framework/…/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts` | the browser reader selects ITS surface out of the batch |
| `🧰️framework/…/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `issued-ui-acks` lifecycle work; intake-then-one-crossing-then-finish |
| `🧰️framework/…/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` | the same reshape for the wgpu frame worker's bridge |
| `🧰️framework/…/⚛️reactor/🧫️fixtures/🧺️turn-patch-batch.json` | **new** — the language-agnostic batch budget |
| `🧰️framework/…/⚛️reactor/🧪️tests/🧺️turn-patch-batch/🦀️.rs` | **new law** (5) |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🧺️turn-patch-batch/🟦️.ts` + `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | **new TS twin** (4) and its registration |
| `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️ui-turn-patch/🦀️.rs` | capacity/derivation/order laws for the page |
| `⚛️reactor/🧪️tests/🔬️reconcile-spin/🦀️.rs`, `📨️pending/🧪️tests/{🩹️receipt,🔬️instance-lifetime-patch-close}`, `🩹️patches/🧪️tests/🔬️unit` | adapted to the batched `stage_emission`/page shape |
| `📜️script.ts` | the interactivity gate's turn-patch strings follow the new shape, and the audit's `reactor` corpus gains `⚛️reactor/🔄️turn/🦀️.rs` — the turn patch page is assembled THERE, so two clauses that named it were reading a file it does not live in and measured nothing |
| `<ticket>/🐍️turn-patch-gate-check.mjs` | **new** — evaluates that gate block's clauses against the working tree and prints every miss, so §4.3's claim is a reading rather than an eyeball |

---

## 4. Laws, with output

### 4.1 Rust — `cargo test -p semio-framework-plugin --lib turn_patch_batch -- --test-threads=1 --nocapture`

```
[DEBUG] turn-patch-batch surfaces=1 crossings=2 patch=1 events=1 idle=0
[DEBUG] turn-patch-batch surfaces=3 crossings=2 patch=1 events=1 idle=0
[DEBUG] turn-patch-batch surfaces=6 crossings=2 patch=1 events=1 idle=0
[DEBUG] turn-patch-batch surfaces=8 crossings=2 patch=1 events=1 idle=0
[DEBUG] turn-patch-batch handback count=5 order=[("79:surface-0", 1), … ("79:surface-4", 5)]
[DEBUG] turn-patch-batch split count=5 fit=2 unit=44B delivered=5
[DEBUG] turn-patch-batch first-always-admitted len=1
[DEBUG] turn-patch-batch capacity=16 page=28304B ceiling=65536B budget=2097152B
test result: ok. 5 passed; 0 failed; 743 filtered out
```

| law | asserts |
|---|---|
| `n_ready_surfaces_converge_in_two_crossings_when_their_patches_fit_the_budget` | **THE law.** 1, 3, 6 and 8 mounted surfaces each converge in **2** crossings — ONE patch-carrying, ONE acknowledgement-carrying — with `idle = 0`, driving the production ladder (`drive_reconcile_within`, `take_turn_patch_page`, `stage_emission`, `commit_emission`, `apply_issued_ack`, `reconcile_arms_turn`), not a model of it. The previous lane's law measured N + 1 for the same shape. |
| `a_refused_turn_page_returns_every_patch_to_its_own_slot_in_publication_order` | the page carries the queued publications in order; a page refused patch-by-patch returns every one to its own reserved slot and the next page reads back in the SAME order |
| `a_patch_over_the_turn_byte_budget_travels_on_the_next_turn_and_still_applies_exactly` | with the budget cut to exactly two patches' measured cost, each turn admits ≤ 2, every turn makes progress, and the concatenation of the pages is the publication order, exactly once |
| `the_first_ready_patch_is_admitted_whatever_the_budget_says` | a 1-byte budget still admits the OLDEST queued publication, and the rest follow in order |
| `a_turn_patch_page_never_asks_the_guest_for_more_than_one_contiguous_ceiling` | the capacity is `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES`, `size_of::<UiTurnPatches>() ≤ 65 536`, `UI_TURN_PATCH_BUDGET_BYTES = GUEST_HOST_ANSWER_CEILING_BYTES / 4`, every declared lane budget sits inside the window, and the clamp lifts/cuts outside it |

### 4.2 TypeScript twin — `bun ./📜️script.ts test long "turn-patch-batch"`

```
 Test Files  1 passed (1)
      Tests  4 passed (4)
```

The host decoder's half: the pairing law over counts 1…64 and every malformed count; **every** patch of
one turn captured as its own private authority in publication order under one receipt, with the same
receipt still refused on any OTHER turn; the whole batch acknowledged in **ONE** crossing carrying one
`patch-ack` per surface in order (`sent.length - before === 1`), memoized so a second submission posts
nothing; and the empty-batch / foreign-token refusals.

### 4.3 The corpora

| gate | result |
|---|---|
| `cargo test -p semio-framework --lib ui_turn_patch -- --test-threads=1` | **18 passed, 0 failed.** This suite parks the retire arena's mutex on purpose, so it is `--test-threads=1`-only; in parallel its own fixtures starve each other's `try_lock` (pre-existing, unrelated to this lane). |
| `cargo test -p semio-framework-actor --lib ui_patch -- --test-threads=1` | 3 passed, 0 failed (the shared pairing fixture) |
| `cargo test -p semio-framework-plugin --lib reactor:: -- --test-threads=1` | **166 passed, 5 failed** — the SAME five the previous lane recorded, each in a file this lane did not touch: `async_actor_poll_awaits_exchange_and_render_work` (a source-TEXT scan for a line a peer boxed), `patches::tests::mounted_document_tree_publishes_nested_interactive_rows` (`SURFACE_RECONCILE_AGGREGATE_BYTES` 52 559 872 vs a fixture's 33 554 432), `executor::tests::cancel_of_a_parked_task_…`, `m1_m2_reactor_tests::revision_guard_…`, `turn_execution_tests::guest_turn_execution_resets_for_every_turn` (order-flaky). This lane adds 5 passes and no reds. |
| `cargo test -p semio-framework-plugin-host --lib ui_patch::tests -- --test-threads=1` | 3 passed, 0 failed |
| `cargo test -p semio-framework-plugin-host --lib -- --test-threads=1` | 214 passed, 8 failed — four `shard::tests` envelope-authority items, three `ui_patch_component_tests` that need a staged scale component (`registered scale component path`), and `schema_parity::every_actor_export_is_async`. All pre-existing and in files this lane did not touch. |
| React engine `bun ./📜️script.ts test long` | **1 241 passed, 4 failed** (48 files) — the same four pre-existing peer-owned reds the previous lane recorded (`🎟️resident-refresh-budget`'s census + three `🔬️engine-contract` items). Before this lane the same corpus was 1 236 passed / 5 failed; the fifth was this lane's own (`schedules captured lifecycle UI ACKs …`, which asserted the one-ack-per-crossing shape) and is fixed. |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | green (warnings only) |
| `@semio-tech/framework-renderer-wgpu:generate-frame-worker` | green (27.5 s, 4 tasks); the regenerated worker carries `submitUiAcknowledgements` and the new pairing rule |
| `@semio-tech/framework-renderer-wgpu:wasm` | green (2 m 28 s, 8 tasks) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib retained -- --test-threads=1` | 13 passed, 1 failed — `command_batch_ninth_document_is_retained_…` pops `UI_DOCUMENT_LEASE_SLOTS` (now 64) owners from a 9-element fixture list. Pre-existing and peer-owned: `UI_RESIDENT_SLOTS` moved 8 → 64 in the `react-example-switch-regression` lane, and this fixture still assumes the old number. |
| `bun 🐍️turn-patch-gate-check.mjs` | the interactivity audit's turn-patch block: **0 of this lane's clauses miss**. Three clauses still miss, all pre-existing peer drift this lane did not touch — `close_ui_turn_patch_transport_one() -> bool` (the function now returns `Result<UiTurnPatchTransportProgress, &'static str>`), `close_ui_turn_patch_owner_one()` (no longer in any audited file), and a `take_ready()` line-break the formatter moved. The same block had **seven** misses against the pre-change tree. |

---

## 5. Before / after, measured on 6021

Restage (foreground, no cache): `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true
bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev` →
`🗑️generated/patch-batch/restage.txt`, **exit 0**, 8 m 40 s, 34 tasks, "11 completed components
(changed)". The served bridge was then verified to carry the change rather than assumed
(`feedback-vite-stale-transform-verify-served-module`):

```
$ curl -s http://127.0.0.1:6021/🔌️plugin-modules/🌀️procedural/🌉️bridge.js | grep -ao "patchCount[^;]\{0,120\}"
patchCount, receipt) {
patchCount) || patchCount < 0 || patchCount > 0 !== (receipt != null))
```

### 5.1 Crossings and wall

`SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d SEMIO_PROBE_OUT=patch-batch/{before,after2} bun 🐍️react-hop-cost-probe.mjs`

| | before (`…/before/`, 00:33) | after (`…/after2/`, 01:49) |
|---|---:|---:|
| **worker crossings per hop** | **58.60** | **58.25** |
| run | 55.4 s, 20 hops, 1 172 crossings | **52.8 s**, 20 hops, 1 165 crossings |
| converged | 10/10 | 10/10 |
| meshes | 13 | 13 |
| hop wall | 2 770 ms | 2 642 ms |
| `patch-ack` crossings | 238 (11.9/hop) | **191 (9.6/hop)** |
| `(none)` crossings | 590 (29.5/hop), 14.0 ms each | 626 (31.3/hop), 13.5 ms each |
| `message` crossings | 196 (9.8/hop) | 200 (10.0/hop) |
| ui patches carried | 238 | 237 |
| `worker.guest` | 750 ms/hop, 12.8 ms mean | 746 ms/hop, 12.8 ms mean |
| `turn.accept` | 511 ms/hop | 495 ms/hop |
| `RecalcStyleDuration` | 1.56 % of wall | 1.59 % of wall (gate: under 5 %) |

**The `patch-ack` row is the whole attributable effect**: 238 → 191 crossings for the same 237 patches,
i.e. 1.00 → 1.24 patches per patch-carrying crossing, and 47 crossings of the run saved on each side of
the boundary. Everything else is inside run-to-run noise.

### 5.2 How big the batches actually are

A batch is only as wide as the number of surfaces the guest has READY at the same instant. Measured
directly, with one temporary `[DEBUG]` line in `acceptUiPatches` over 50 s of boot
(`🗑️generated/patch-batch/batchsize/`):

| patches in one turn result | turns |
|---:|---:|
| 1 | 18 |
| 2 | 4 |
| 4 | 2 |
| **9** | **1** |

43 publications in 25 crossings where the old contract would have needed 43, and 25 acknowledgement
crossings where it would have needed 43 — **36 crossings saved across one boot**. The 9-patch turn is
the contributions-install burst, where the whole shell's panel set becomes ready together; the long
tail of ones is the steady state, where surfaces reconcile one at a time.

### 5.3 The load caveat, stated rather than buried

The two runs straddle a 3× difference in machine load (`uptime`): **22–26** for the before run,
**7.7** for the after run, on a machine carrying ~12 lanes. The hop wall (2 770 → 2 642 ms) and every
millisecond column in §5.1 are therefore NOT attributable to this change. The crossing counts are, and
they are what §5.1 is read for.

### 5.4 The journey

`SEMIO_PROBE_URL=… bun 🐍️journey-probe.mjs` → `🗑️generated/patch-batch/journey/`:

```
DONE steps 23 meshSteps 20
```

**23/23 steps converged (`converged=false` count: 0)**, 20 steps carry meshes, and **all 16 rows that
declare a mesh oracle report `meshOk=true`** with exact triangle counts and bounds inside their
fixtures (`eval-extrude@solid#0`, `eval-brep_bool_cut_5@solid#0`, `eval-fillet@solid#0`,
`eval-fuse@solid#0`, `eval-shell@solid#0`, `eval-rect@wire#0`); `meshOk=false` count: 0. The previous
lane's run on the same probe had six `converged=false` rows; this one has none.

### 5.5 The operational consequence: a serve older than this change refuses a batched turn

The FIRST probe run after the restage measured 0/10 converged, 0 meshes and a storm of
`actor-ui-patch.pairing` — while `curl` showed the served module already carrying the new rule. Three
runs later against the same untouched build (`console-dump`, `page-error-stack`, `console2`) the
shell boots, `window:procedural-preview` holds 3 meshes at `phase:"idle"`, `ratio:1`, and the pairing
error does not occur at all.

The mechanism is not a race but a **version skew across the boundary**, and the coordinator observed
it independently at 10:10 on the other React serves: **a host serve started BEFORE this change still
runs the old `validateActorUiPatchPairing`, which refuses any turn carrying more than one patch.** The
guest is staged once and shared; the host half is served by whichever long-lived vite server a lane is
pointed at, and with `SEMIO_VITE_HMR=0` a server that has already transformed
`🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts` keeps serving what it transformed. So:

> After this change lands, **every React serve must be recycled**; a serve older than it will fail
> every batched turn with `actor-ui-patch.pairing`, which looks exactly like a broken guest.

6021 recovered on its own once its server re-transformed the module; the idle serves (6018, 6022–6025,
6027, 6028) were recycled by the coordinator. **Every number in §5.1 and §5.4 is from a run on a page
that booted clean**; the first run's numbers are published in `🗑️generated/patch-batch/after/` as the
artefact of that skew, not as a measurement.

---

## 6. The wgpu twin

The wgpu shell's frame Worker runs the SAME reactor and the same generated shard worker, so the guest
half of this change reaches it unmodified. Its host half does not: the frame worker carries its own
bundled copy of the shard client and its own `WgpuOwnedUi.accept`, and both were reshaped exactly like
the React ones (§2.5) — drive every patch's intake to its token, submit ONE `issued-ui-acks` crossing,
finish every intake against its own receipt.

| gate | result |
|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | green (warnings only). The wgpu renderer's own patch admission (`🧊️renderer/🦀️.rs`) already drained the turn owner with `try_transfer_one` until `Empty`, so it needed no change to carry a batch — only the count cap above it had to move. |
| `@semio-tech/framework-renderer-wgpu:generate-frame-worker` | green (27.5 s, 4 tasks). The regenerated `🎞️frame-worker/🤖️generated/🟨️.js` carries `submitUiAcknowledgements` (2 sites) and the new pairing rule — verified by reading the emitted file, not assumed. |
| `@semio-tech/framework-renderer-wgpu:wasm` | green (2 m 28 s, 8 tasks) |
| `…:activate-generation3d-wgpu-dev` (foreground, `NX_SKIP_NX_CACHE=true`) | **exit 0**, 1 m 20 s, 39 tasks, "11 completed components (changed)" → `🗑️generated/patch-batch/restage-wgpu.txt` |
| `bun 🐍️wgpu-battery.mjs --only=frame-loop,examples` on 6118 | **not run as its own invocation** — 6118's standing gate (`until [ -z "$(pgrep -f 'wgpu-batter[y]\|wgpu-.*-pro[b]e')" ]`) was held continuously from 09:21 to past 11:00 by another lane's broader battery. Both of those two steps were read instead out of THAT battery's own scoreboard, on this lane's build — §6.1. |

### 6.1 What the standing battery already read on this build

A broader battery was already running against 6118 when this lane's wgpu restage landed (09:25) and
kept going until 10:28, so its later steps are evidence on exactly this build
(`🗑️generated/wgpu-verify/scoreboard.json`, `pageerrors: 0` over the whole run):

| step | ran | verdict |
|---|---|---|
| **`frame-loop`** | ~10:26 (post-restage) | **green** — 20 gestures, **0 quarantines**, 6 954 batches / 6 953 frames, **0 faults**, 16 actions over 368 admits and 25 sweeps. The frame loop does not regress. |
| **`examples`** | 09:22–09:45 (straddles) | **green** — all sixteen editor and viewer rows publish exactly their committed meshes with the right ids and boxes (`eval-extrude@solid#0`, `eval-fillet@solid#0`, `eval-fuse@solid#0`, `eval-shell@solid#0`, `eval-rect@wire#0`, `eval-brep_bool_cut_5@solid#0`) |
| `generate-add`, `generation-roster` | post-restage | green |
| `spawn-job` | post-restage | red on `s1_hover` only (`pumps: 0`), with `no dropped spawn-job effect` and `no panic, no dispatch failure` both green |
| `world3d-editor` / `world3d-viewer` | post-restage | red on the selection round-trip (`h1`/`h3`/`h4`/`h6`) and on two boot-camera framing rows — **both are other lanes' open items**, named in `📓️status.md` as `wgpu-selection-roundtrip` and `boot-camera-framing`'s `reportCamera` origin fallback. Every `no authority fault, no panic, no dropped effect` row is green, and every example still publishes exactly its committed meshes. |
| `boot`, `no-example`, `deferred-commit` | 09:21–09:24 (pre/straddling) | red; not read here — `no-example` is green in that battery's own `battery-2.txt` and red in `battery-1.txt`, on builds that predate this lane entirely |

**No red on this build is a patch-delivery red**: every one of them is a hover/selection/camera or
pixel-diff verdict, and the two rows that would catch a broken patch batch — "publishes exactly its
committed meshes" and "no authority fault, no panic, no dropped effect" — are green on every example
in both roles.

Also restaged and re-verified after the wgpu wave: the React door on 6021 still boots clean, with
`window:procedural-preview` holding 3 meshes at `phase:"idle"` and zero console errors
(`🗑️generated/patch-batch/postwgpu/`), so §5's numbers are not left behind by the wgpu build.

---

## 7. What is left, named with its evidence

**The cap was real and is gone; it was not what a hop is made of.** 58.25 crossings per hop decompose as:

| crossing | per hop | what it is |
|---|---:|---|
| `(none)` | 31.3 | the host's continuation loop asking a `MoreWork` guest again |
| `message` | 10.0 | shell↔guest messages |
| `patch-ack` | 9.6 | the batched acknowledgements — **this lane's term**, 11.9 before |
| `surface-visible` | 3.5 | |
| `request` / `completed` | 3.8 | |

1. **The batch is as wide as the guest's readiness, and the guest is rarely ready in parallel.** §5.2
   measures it: 18 of 25 boot turns had exactly one surface ready. The reconcile drive is bounded by
   the turn's own 8 ms wall hold, so per turn only a few reconcile jobs finish and only a few outputs
   become extractable — and `next_ready_index`'s generation gate serializes even those. Widening the
   batch means widening READINESS, which is the reconcile drive's budget, not the wire's capacity.
2. **31.3 empty crossings per hop are still the dominant term** and none of them is a publication:
   they are the `settlePluginTurn` continuation loop polling a guest that answers `MoreWork` with
   nothing. That is the same family the previous lane cut in half (89 → 50 → 29.5 per hop across three
   lanes) and it is still first.
3. **The ≤ 10 crossings-per-hop target is NOT met** — 58.60 → 58.25. What IS met is the law the target
   rests on: N ready surfaces cost **2** crossings, not N + 1, proven against the production ladder for
   N = 1, 3, 6, 8 with zero idle crossings, and observed in the browser at up to 9 patches in one turn.

---

## 8. Not claimed

- **The crossings-per-hop target is not met** (§7.3). The floor `📓️reactor-reconcile-spin-2026-09-14.md`
  §7 named is removed and proven removed; the hop cost barely moved because the floor was not binding
  on this workload.
- **No millisecond in §5.1 is attributed to this change.** The before and after runs straddle a 3×
  load difference (§5.3). Only the crossing counts are load-independent and only they are claimed.
- **`patches per patch-carrying crossing` (1.00 → 1.24) is the honest size of the effect on the probe's
  workload**; the 9-patch turn in §5.2 is a boot burst, not a steady-state figure.
- **The 4 React reds, the 5 reactor reds and the 8 plugin-host reds are pre-existing and peer-owned**,
  each checked with this lane's tests present and each in a file this lane did not touch (§4.3).
- **`🔬️ui-turn-patch` is a `--test-threads=1` suite** — its own fixtures hold the retire arena's mutex
  on purpose, so in parallel they starve each other's `try_lock`. That is pre-existing; this lane
  neither introduced nor fixed it.
- **Nothing about the reconcile drive's readiness budget was changed** (§7.1). Widening the batch is
  now a matter of widening readiness, and that is the next lane's, not this one's.
- **The wgpu `--only=frame-loop,examples` battery was not run as its own invocation** (§6). 6118's
  standing gate was held by another lane's battery for the entire window; both steps were read from
  that battery's scoreboard on this build instead, and `examples` there STRADDLES the restage by
  ~3 minutes (§6.1) while `frame-loop` does not. A dedicated run is the one number this lane owes.
- **This change is not backward compatible with a running host serve, by design.** §5.5 states the
  skew and the recycle it requires; it is a wire cardinality change in a greenfield repo, so no
  compatibility path was built and none should be.
