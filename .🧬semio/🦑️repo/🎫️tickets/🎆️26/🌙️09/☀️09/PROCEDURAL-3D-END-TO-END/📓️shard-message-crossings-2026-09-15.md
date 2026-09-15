# 📮️ The crossing census — what a `flowEvalTick` hop's 36 worker round trips actually carry

Lane `shard-message-crossings`, 2026-09-15, React door of 🧊️generation3d on **:6021**
(`📜️serve-generation3d-react-6021.sh`, host TS vite-live, `SEMIO_VITE_HMR=0`).

Continues `📓️host-refresh-latency-2026-09-15.md` §5.6, which named the two remaining terms and claimed
neither:

> `patch-ack` at 10.9/hop (the patch-batching lane's `UI_TURN_PATCHES_MAXIMUM` floor) and `message` at
> 9.6/hop, **172 crossings carrying nothing at all**, one whole round trip per shell `send-message`
> instead of riding the next turn's event list.

---

## 0. The headline

> **`message` was never "shell messages carrying nothing". Every single one of the 200 measured
> `message` crossings is a typed-operation RESULT PAGE acknowledgement, and every single one was
> exactly ONE ack wide — because the guest handed the host exactly one page per turn, whatever the
> host asked for. The page's ACK is per page by protocol; the CROSSING that carries it never had to
> be. `Event::Message` is a list on the wire and the reactor's poll acknowledges every entry of it.**

> **`patch-ack` is NOT sent per surface — that half of the hypothesis is falsified by measurement:
> 44 of 204 ack crossings carried 2–9 acks, one carried 9. What is serialized is READINESS, and it is
> no longer `next_ready_index`'s generation gate (already widened per allocation by the batching
> lane). It is the workload: 10.2 patch-PUBLISHING turns per hop publishing 13.5 patches between
> them. Named with its owner, not claimed.**

| | before (`🗑️generated/shard-msg/before2`) | after |
|---|---:|---:|
| worker crossings per `flowEvalTick` hop | **36.30** | see §5 |
| `message` (= typed-operation page acks) per hop | **10.0**, width **1×200** | see §5 |
| `patch-ack` per hop | 10.2, width 1×160 2×31 3×10 4×2 9×1 | see §5 |

---

## 1. The instrument — a HOST-side crossing census

The worker's own `worker.turn` span detail names the posted event KINDS
(`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`, `eventKinds`), which cannot say what a
`message` IS, cannot say how WIDE a batch was, and books a command-ingress page crossing as `(none)`.
So this lane added a census on the one seam every crossing passes through — the turn scheduler's
`runTurn` in `🔌️PluginRuntime/🟦️.tsx` — classifying each round trip by the events it carried,
splitting `message` by source tag and page magic, and keeping a per-crossing WIDTH histogram.
`🐍️react-hop-cost-probe.mjs` reads it off `globalThis.__semioCrossingCensus` /
`__semioTypedOpAckCensus` and prints two extra tables. It is a lazy accessor (a `get`/`set` pair, so
a stale module's assignment can never throw) and costs two counter increments per crossing.

---

## 2. The taxonomy, BEFORE — `🗑️generated/shard-msg/before2`, 54 s, 20 hops, 10/10 converged, load 15.8 → 20.8

```
totals: 10 steps, 54.3 s, 20 hops, 726 worker crossings, converged 10/10
worker crossings per hop: 36.30; hop wall: 2880 ms
```

| host crossing census (carried events) | crossings | per hop | events | reply patches | reply effects | widths |
|---|---:|---:|---:|---:|---:|---|
| `patch-ack` | 204 | 10.2 | 269 | 168 | 0 | 1×160 2×31 3×10 4×2 9×1 |
| **`message.typed-op-ack`** | **200** | **10.0** | **200** | 0 | 314 | **1×200** |
| `command-page` | 128 | 6.4 | 128 | 0 | 221 | 1×128 |
| `surface-visible` | 81 | 4.0 | 792 | 55 | 0 | 2×11 8×1 11×68 14×1 |
| `(none)` | 49 | 2.5 | 0 | 46 | 11 | 0×49 |
| `request` | 38 | 1.9 | 38 | 0 | 38 | 1×38 |
| `completed` | 38 | 1.9 | 38 | 0 | 121 | 1×38 |
| `job-completed` / `lifecycle.open` / `lifecycle.receipt-ack` | 4 | 0.2 | 4 | 0 | 4 | 1×4 |

```
typed-operation ack ladder: 200 acks over 69 operations (2.9 pages per operation);
  widest — op704:5acks seq0..4, op896:5acks seq0..4, op1216:5acks seq0..4, op1728:5acks seq0..4, …
worker MoreWork drive: 1389 guest turns absorbed over 726 crossings (1.91 per crossing);
  stops carried=471 idle=218 input=27 budget=10
```

### 2.1 `message` — classified, and the answer is ONE kind

Of the five kinds the lane was asked to look for — turn-event notifications, heartbeat residue,
session admission, `surface-visible`, `request`/`completed` — **`message` contains none of them.**

| classified | count |
|---|---:|
| `message.typed-op-ack` (a `TypedOperationResultPage` acknowledgement, `Shell{instance}` + `semio.typed-operation-ack.v1\0`) | **200 / 200** |
| `message.backbone` (document backbone data) | 0 |
| `message.shell` (document backbone control) | 0 |
| turn-event notifications / heartbeat residue / session admission | **0** |

`surface-visible`, `request` and `completed` are their OWN crossing classes, and two of the three are
already well batched (`surface-visible`: 792 events over 81 crossings, 68 of them 11 wide). The
heartbeat residue the 09-14 lane named is gone for good — the worker's beat rides the reply
(`📓️worker-more-work-drive-2026-09-15.md` §0, 3 main-thread messages per crossing → 1).

**`command-page` is a re-attribution, not a new term**: 128 of the worker's 171 `(none)` crossings
actually carry a command-ingress page, which the worker's `eventKinds` string cannot see because
`msg.events` is empty. Nearly all of them are the 67-page `setContributions` of the two boots
(`📓️host-refresh-latency-2026-09-15.md` §5.6: `2 × command-complete continuations=0/1072`), so it is a
BOOT term, not a steady-state one. Named for the ingress-paging owner; not this lane's.

### 2.2 The defect, in the guest's own words

`🔌️plugin/🦀️.rs`, `plugin_continue_typed_operations`:

> 📮️ Publishes admitted operations without requiring another user command — as many units of ONE
> instance as `TypedOperationGrant` allows, **so a staged work costs the host one round trip per
> acknowledgeable result page** instead of one per unit.

and the two lines that make that literally true:

```rust
let mut output = PluginExchangeOutput { typed_operation_result: app.take_typed_operation_result_page(instance), .. };   // ONE page
fn spent(&self, output: &PluginExchangeOutput) -> bool { output.typed_operation_result.is_some() || … }                  // stop at the first
```

`take_typed_operation_result_page` rotates a cursor over `ARTIFACT_LIVE_OUTPUT_SLOTS` and returns the
first operation with a presentable page — so even with four live operations each holding one, three
waited a whole round trip each. The census proves it end to end: **200 acks, 69 operations, every
crossing exactly one ack wide.**

The native fixture path had already found and fixed the same shape for its own drain —
`while let Some(page) = app.take_typed_operation_result_page(receiver)` with the comment "EVERY
presented page, not one per turn … a command that publishes an artifact, a config and an app transient
reported two of the three (ticket 26/09/09/PROCEDURAL-3D-END-TO-END)". The WIRE path was the one still
handing out one.

### 2.3 `patch-ack` — which of the two named causes? Measured.

The brief named two candidates. The census settles both:

1. **"acks are sent per surface anyway" — FALSIFIED.** The batching lane's change is live and works:
   the width histogram is `1×160 2×31 3×10 4×2 9×1` — 44 of 204 crossings carried 2 to 9 acks in one
   crossing, 269 acks over 204 crossings.
2. **"the readiness is serialized (the `next_ready_index` generation gate)" — the gate is ALREADY
   widened.** `⚛️reactor/🩹️patches/🦀️.rs::ready_output_is_admissible` compares a ready output only
   against ITS OWN allocation's earlier work; the global minimum-generation refusal the batching
   lane's §7.1 named is gone from the source.

What is left is neither. Three independent readings say the guest is not being held back by a wall or
a wire cap, it simply has ~1.3 surfaces ready at a time on this workload:

* `UI_TURN_PATCHES_MAXIMUM` = `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES` =
  **16** patches per turn — and the widest turn observed carried **9**. The wire is not the cap.
* `drive_reconcile_within` is given the turn's OWN `budget.deadline_ms` (100 ms), not the reactor's
  8 ms executor slice, and extracts every ready output a free publication slot will take.
* The worker drive census says the guest is not wall-bound either: `budget=10` and `steps=0` out of
  726 crossings. **471 crossings stopped `carried`** — the drive crossed back because the turn had
  produced something, which for a patch-carrying turn it MUST: `uiPatchReceipt`'s `patchSequence`
  pairing is a per-turn authority (`captureInstanceUiPatch`, `validateActorUiPatchPairing`), so two
  guest turns' patches can never be merged into one reply.

So **`patch-ack` per hop == patch-PUBLISHING turns per hop**, by wire law: 10.2 crossings publishing
13.5 patches. Cutting it means making one turn's reconcile finish more surfaces at once, which is the
reconcile drive's readiness — `📓️ui-turn-patch-batching-2026-09-15.md` §7.1's own words, "widening the
batch means widening READINESS, which is the reconcile drive's budget, not the wire's capacity".
**Named with its owner; not claimed here** (§7).

---

## 3. The design — a turn carries every page it can, the host retires them in one crossing

One declaration bounds it, on the guest side, where the pages are:

```rust
/// 📄️ Result pages ONE turn may hand the host, and therefore the widest acknowledgement batch the
/// shell posts back in one crossing.
pub const TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM: usize = 8;
```

* `PluginExchangeOutput.typed_operation_result: Option<Page>` → **`typed_operation_results: Vec<Page>`**.
* `advance_typed_operation_output` drains `take_typed_operation_result_page` up to that bound instead of
  taking one — the same `while let` shape the native fixture path already proved.
* `TypedOperationGrant::spent` ends the drive at the BOUND, not at the first page, so one turn keeps
  driving units while pages accumulate.
* `plugin_continue_typed_operations` **extends** the collected output instead of overwriting it.
* `⚛️reactor/🔄️turn/🦀️.rs::route_exchange_output` pushes one `Effect::SendMessage{Shell}` per page.

Nothing on the host needed changing, and that is the point: the host has been batch-ready the whole
time. `typedOperationAcknowledgements(turn)` already `flatMap`s every page of a turn into a list, and
`settlePluginTurn`'s `acknowledge()` already posts that whole list as ONE `submitPluginTurn` event
array. The wgpu bridge's `WgpuTypedOperationDrive` already queues `scan.acknowledgements` and hands
them over in one `takeAcknowledgements()`. Both are now pinned by law against the shape that is
actually on the wire (§4).

**One latent drop fixed forward on the way**: the intent-frame exit of `plugin_exchange` built its
`PluginExchangeOutput` with `typed_operation_result: None` while the `advance_typed_operation_output`
it had just called may have taken a page — and `take_result_page` marks the page `presented`, so the
operation would have waited for an ACK that could never be sent. It now carries
`typed.typed_operation_results`.

---

## 4. Laws, with their output

### 4.1 Rust — `cargo test -p semio-framework-plugin --lib -- concurrent_typed_operations_hand_every_presentable_page_to_one_turn --test-threads=1 --nocapture`

`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`, new law: four operations admitted on
one instance, driven through the production `plugin_continue_typed_operations` with the production
browser budget.

```
[DEBUG] typed-operation slots instance=7 live=4/64 peak=4
[DEBUG] typed-operation slots instance=7 live=0/64 peak=4
[DEBUG] actual 10 typed-operation result pages retired in 4 host crossings, widest batch 4
test …::concurrent_typed_operations_hand_every_presentable_page_to_one_turn ... ok
test result: ok. 1 passed; 0 failed; 761 filtered out
```

**10 pages, 4 crossings, widest batch 4** — before this change that is 10 crossings by construction.
The law asserts `widest >= 2` and `crossings < pages`, and is deliberately lane-agnostic: four
simultaneous edits of one immutable document root make three of them rebase refusals, and a refusal is
published as its own acknowledgeable `Fault` page on exactly the wire the law measures.

### 4.2 TypeScript twin (the shard client / settle lane) — `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`

```
[DEBUG] typed-operation ack batch: 4 result pages retired in 1 crossing(s)
 ✓ …🔌️PluginRuntime/🟦️.tsx > submitPluginTurn … > retires every result page one turn carried in a single acknowledgement crossing
 Test Files  11 failed | 1 passed | 39 skipped (51)
      Tests  1 passed | 1174 skipped (1175)
```

A turn whose effects carry four distinct operations' pages yields four acknowledgements
(`typedOperationAcknowledgements(carried)` has length 4) and `settlePluginTurn` spends **exactly one**
crossing carrying `["message","message","message","message"]`.

The 11 failed FILES are pre-existing and peer-owned: every one of them dies at import with
`ReferenceError: Cannot access '__vite_ssr_import_18__' before initialization` in
`🧱️elements/🛠️ShellHelpers/🟦️.tsx:2102` / `🐚️Shell/🟦️.tsx:78` — files this lane never opened, and
**zero** tests run inside them.

### 4.3 The wgpu bridge's decode, by law (wgpu is on hold — no 6118 battery was run)

`bun vitest run --config 🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts 🗞️wgpu-typed-operation-reply`

```
 ✓ 🗞️ wgpu multi-frame reply … > splits one interactive job's reply into decodable frames and owed acknowledgements
 ✓ … reports the exact framing failure a missing discrimination produces
 ✓ … never observes the same turn twice, so one page is acknowledged exactly once
 ✓ … agrees with the shared scanner about the fixture's own declared stream
 ✓ … projects a bigint-bearing reply onto the JSON the program bridge parses
 ✓ … declares finite settle, drain and retained-effect authorities
 Test Files  1 passed (1)   Tests  6 passed (6)
```

The second of those is precisely the new wire shape: ONE turn carrying **three** pages plus an app
frame splits into one decodable frame and **three** owed acknowledgements handed over in ONE
`takeAcknowledgements()`. The wgpu bridge shares the reactor and its decode stays correct under a
multi-page turn — proven, not assumed.

### 4.4 Type-check

`cargo check -p semio-framework-plugin --all-targets --keep-going` — **0 errors**, `Finished`, and
286 warnings emitted (the pre-existing tree-wide set), which is the proof expansion actually ran.

---

## 5. The served reading

*(filled below after the restage)*

---

## 6. An incident this lane caused and closed

At 18:25 the coordinator relayed a peer's :6021 console fault:
`Cannot set property __semioTypedOpAckCensus of #<Window> which has only a getter`, failing
`setActiveExample`/`setContributions` for `procedural#1`. It was this lane's census: the first version
ASSIGNED the global on the hot path, the second replaced the assignment with a getter-only
`defineProperty`, and a browser holding the older served transform then assigned into the newer
descriptor.

Fixed within minutes: both census globals are now defined with a `get`/`set` pair, so an assignment
from any module generation is inert instead of throwing. `:6021` was recycled by pid, the served
`/@fs` module was confirmed to carry the pair, and a 45 s `🐍️console-dump-probe.mjs` run
(`🗑️generated/shard-msg/census-fault-check`) reads **0 `only a getter` lines and 0 page errors**, with
`window:procedural-main` all-ok and `window:procedural-preview` idle at ratio 1.

**An instrument must never break dispatch.** It did for roughly twenty minutes; it is named here
rather than quietly repaired.

---

## 7. Not claimed

- **`patch-ack` is measured and attributed, not fixed** (§2.3). Both causes the brief named are ruled
  out by measurement; the residual is one crossing per patch-PUBLISHING turn — a wire law, given the
  per-turn `uiPatchReceipt` authority — times 10.2 publishing turns per hop. Cutting it means
  widening the reconcile drive's per-turn readiness, which is `📓️ui-turn-patch-batching-2026-09-15.md`
  §7.1's declared next owner and was not opened here.
- **`command-page` (6.4/hop) is re-attributed, not claimed.** It is a boot term (the 67-page
  `setContributions`) and belongs to the command-ingress paging owner.
- **`surface-visible` (4.0/hop) needed nothing**: 792 events over 81 crossings, already 11 wide.
- **The ≤ 15 crossings-per-hop target** — see §5 for whether it was met; the two terms this lane did
  not own (`patch-ack` 10.2 and `command-page` 6.4) together exceed it on their own.
- **No millisecond is attributed to this change.** The before and after runs straddle a load swing
  from 15.8 to well past 70 (peers rebuilding wasm throughout). Only the crossing counts are
  load-independent and only they are claimed.
- **The 11 red React test FILES are pre-existing and peer-owned** (§4.2), each dying at import in
  `🛠️ShellHelpers`/`🐚️Shell`, neither of which this lane touched.
- **No wgpu 6118 battery was run** — wgpu is on hold; the bridge's decode is held by §4.3's law only.
- **`TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM = 8` is a declared bound, not a measured optimum.** Each
  operation still retains exactly ONE unacknowledged page (`queue_page`), so the bound is over
  DISTINCT live operations; one operation's own 0..4 sequence stays strictly serial and no change here
  touches that.
