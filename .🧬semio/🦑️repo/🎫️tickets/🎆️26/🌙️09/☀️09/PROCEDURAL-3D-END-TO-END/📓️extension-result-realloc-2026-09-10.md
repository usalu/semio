# 🧨️ `cabi_realloc` aborts while lowering the extension result — reproduction, root cause, fix

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: extension-result delivery path (`Event::Completed`
lowering into the guest `poll`, `⚛️reactor/🔄️turn` registry resolve, `📥️imports`, the brep/math
extension evaluate outputs) and its memory footprint. Written 2026-09-10, appended as the lane ran.

## 1. The failure this lane owns

Boot #12 (`📓️runtime-verification-2026-09-09.md`, 11:35, on the 10:54 restage): contributions
install, the shell resolves the first extension, and EVERY delivery of the result traps the guest:

```
[DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-math, capability: evaluate, req: 15n,
  error: unreachable at semio_s_plugin_procedural.wasm.abort
    ← std::sys::pal::wasi::abort_internal
    ← cabi_realloc
    ← poll}
PluginRuntime: actor procedural#1 trapped [handler/turn]   → actor restored → req: 16n traps identically
```

Preview stays `faulted`, meshes `[]`, forever.

## 2. Reproduction — headless, on the real staged `wasm32-wasip2` module

`🐍️realloc-probe.mjs` (this ticket). It patches `WebAssembly.instantiate` BEFORE importing the
staged component (jco captures it once at module scope as `instantiateCore`), hands back a plain
`{ exports }` whose `cabi_realloc` is a logging shim over the real export, then drives the same
`poll` turns `ShardClient` drives and reads `memory.buffer.byteLength` around every turn.

```
node --experimental-wasm-jspi 🐍️realloc-probe.mjs \
  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🌀️procedural <bytes> <turns>
```

Raw logs: `🗑️generated/realloc-probe-1.txt` … `realloc-probe-4.txt`.

### 2.1 The signature reproduces EXACTLY

Delivering one `Event::Completed` whose `ok` pack the guest allocator cannot serve
(`🗑️generated/realloc-probe-2.txt`):

```
[probe   2707ms] completed 600000000B: RuntimeError: unreachable
    at semio_s_plugin_procedural.wasm.abort (wasm-function[143714])
    at semio_s_plugin_procedural.wasm._RNvNtNtNtCsgjBd1RpiWTb_3std3sys3pal4wasi14abort_internal (wasm-function[143619])
    at semio_s_plugin_procedural.wasm.cabi_realloc (wasm-function[143688])
    at Object.poll$1 [as poll] (…/semio_s_plugin_procedural_component.js:7446:31)
```

Frame for frame the browser's line. `poll` is jco's `poll$1` — the JS wrapper — so the trap happens
in the **lowering of the poll INPUT**, before one instruction of guest Rust runs. The guest was at
**35.44 MB, 6.9 % of the 512 MiB ceiling**, so it is NOT heap exhaustion: it is ONE `cabi_realloc`
for a block the allocator refuses, and `cabi_realloc` answers that with `handle_alloc_error` →
`abort_internal` → `unreachable`, i.e. an unrecoverable actor trap with no fault and no diagnosis.

### 2.2 Payload / realloc / memory table (before the fix)

Fresh boot, `instance-open` + 2 idle turns, then one `Event::Completed` per row
(`🗑️generated/realloc-probe-3.txt`):

| poll input | `cabi_realloc` calls | bytes asked | largest single block | guest memory before → after | Δ |
|---|---|---|---|---|---|
| `instance-open` | 9 | 4 495 | 4 360 | 11.88 MB → 35.00 MB | +24 248 320 |
| idle turn | 2 | 4 360 | 4 360 | 35.00 → 35.19 MB | +196 608 |
| idle turn (settled) | 2 | 4 360 | 4 360 | 35.19 → 35.19 MB | **0** |
| `completed` 0 B | 3 | 4 440 | 4 360 | 35.19 → 35.19 MB | 0 |
| `completed` 1 KiB | 3 | 5 464 | 4 360 | 35.19 → 35.19 MB | 0 |
| `completed` 64 KiB | 3 | 69 976 | 65 536 | 35.19 → 35.19 MB | 0 |
| `completed` 256 KiB | 3 | 266 584 | 262 144 | 35.19 → 35.44 MB | +262 144 |
| `completed` 1 MiB | 3 | 1 053 016 | 1 048 576 | 35.44 → 36.19 MB | +786 432 |
| `completed` 4 MiB | 3 | 4 198 744 | 4 194 304 | 36.19 → 39.19 MB | +3 145 728 |
| `completed` 16 MiB | 3 | 16 781 656 | 16 777 216 | 39.19 → 51.19 MB | +12 582 912 |
| `completed` 64 MiB | 3 | 67 113 304 | 67 108 864 | 51.19 → 99.19 MB | +50 331 648 |
| `completed` 128 MiB | 3 | 134 222 168 | 134 217 728 | 99.19 → 163.19 MB | +67 108 864 |
| `completed` 256 MiB | 3 | 268 439 896 | 268 435 456 | 163.19 → 291.19 MB | +134 217 728 |
| `completed` 384 MiB | 3 | 402 657 624 | 402 653 184 | 291.19 → **419.19 MB (81.9 %)** | +134 217 728 |
| `completed` 572 MiB | 3 | 600 004 440 | 600 000 000 | 35.44 MB → **trap** | — |

Three readings fall out of that table.

1. **The delivery path has NO bound.** The `pack` in `Event::Completed` is lowered as ONE contiguous
   `cabi_realloc(0, 0, 1, payload.len())`. Whatever the shell hands it, jco asks the guest for it.
2. **Every turn's fixed cost is 4 360 B** — jco's `poll` return area — and it IS released: a settled
   idle turn moves guest memory by 0.
3. **A single oversized delivery is permanent.** `dlmalloc` never shrinks wasm linear memory: after
   the 384 MiB row the guest sits at 419 MB for the rest of its life, so the NEXT delivery of any
   size traps. That is exactly boot #12's "req 15 traps, actor restored, req 16 traps identically"
   — the restore is a `checkpoint`/`restore` inside the SAME instance and the SAME linear memory.

## 3. Root cause

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:735-738` — `completed-event`
carries `outcome: completion-result`, whose `ok`/`fault` arms are a bare unbounded `pack`
(`type pack = list<u8>`, line 10) — and

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:2192`
(`captureExtensionCompletion`'s `complete`) submits the WHOLE extension output as that one pack:

```ts
await submitTurn(actorId, [{ kind: "completed", payload: { req,
  outcome: "ok" in outcome ? { tag: "ok", val: Array.from(outcome.ok) } : { tag: "fault", val: Array.from(outcome.fault) } } }], { activation })
```

with no size check anywhere between `runCapturedExtensionEffect`'s `encodePackValue(JSON.parse(outputJson))`
(`🏛️ShellHost/🟦️.tsx:1648`) and jco's `realloc0Async(0, 0, 1, len)`.

The repo ALREADY declares the bound this violates:
`🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs` `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES = 65 536` —
*"Largest CONTIGUOUS block a routine, per-command or per-turn guest path may request … the FIRST
request a fragmented or nearly-full guest refuses"*. The extension-result delivery is a per-turn
guest path and it asks for an arbitrary multiple of that bound. Nothing enforced it, and the failure
mode when the allocator says no is not a fault the actor can report — it is `abort`.

Secondary retainer on the same path:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:733` `extension_response_args`
re-parses the ORIGINAL request body and merges every one of its fields onto the response action.
For the procedural evaluate request (`⏱️flow-eval-tick/🦀️.rs:61-66`) that echoes `inputJson` — the
operator's whole input — straight back INTO the guest on every answer, on top of `outputJson`. The
SDK never needed the body itself, only the app's correlation fields.

## 4. The fix

### 4.1 The budget is declared, in both languages, pinned to one schema

`🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧬️schema/🔣️.json` gains `hostAnswerCeilingBytes: 8388608` —
*"Largest ASSEMBLED answer the host may deliver into a guest for one outstanding request"* — beside
the `contiguousRequestCeilingBytes: 65536` that was already there and already said what the
extension-result path was violating.

| side | file | what it holds |
|---|---|---|
| Rust | `⏱️trace/🧮️memory/🦀️.rs` | `GUEST_HOST_ANSWER_CEILING_BYTES`, `guest_host_answer_pages(bytes)` |
| TS (new) | `⏱️trace/🧮️memory/🟦️.ts` | the same two constants + `guestAnswerPages(answer)`, the cutter |

The TS twin is not a convenience: **the party that lowers a payload into a guest is the only party
that can keep it inside the bound**, because the guest's refusal is an abort, not a fault. It is
re-exported from `@semio-tech/framework` beside `PUBLIC_INVOCATION_STRING_BYTES`, the bound that
already governs the same class of problem on the command channel.

### 4.2 The host pages the answer; the completion carries the terminal page

`🔌️PluginRuntime/🟦️.tsx`'s `captureExtensionCompletion().complete` now refuses an answer past the
host-answer ceiling with `extension.answer-too-large` (a `SemioFaultError`, surfaced through the
`invokeExtension dispatch failed` path the shell already has) and otherwise cuts it with
`guestAnswerPages`, submitting ONE turn:

```
[ http-chunk{req, bytes: page0, done:false} ... http-chunk{req, bytes: pageN} , completed{req, ok: terminal} ]
```

`Event::Completed` stays THE one completion door for a `req` — the pages are strictly prologue — so
the guest's turn generator gains NO second dispatch await site (the size the poll-leak lane shrank,
`📓️poll-task-leak-2026-09-10.md` section 3, is untouched).

### 4.3 The guest accumulates, bounded, and answers with a fault instead of trapping

- `⚛️reactor/📮️requests/🦀️.rs` — `Slot::Continuation` gains `partial: Result<Vec<u8>, Fault>`; new
  `RequestRegistry::append_continuation_chunk` refuses a page over
  `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` or an assembled body over
  `GUEST_HOST_ANSWER_CEILING_BYTES` into the slot's POISON
  (`plugin.request-registry.answer-too-large`) rather than growing the guest;
  `ExtensionContinuation::into_response` joins prologue + terminal, poison winning over both.
- `⚛️reactor/🦀️.rs` — `take_extension_response` takes the outcome BY VALUE and hands it back
  (`Err(unclaimed)`) when the id owns no continuation, so `RequestRegistry::resolve` still gets it
  with no clone; new `append_extension_response_page`.
- `⚛️reactor/🔄️turn/🦀️.rs` — `Event::HttpChunk` offers each page to the continuation table first and
  falls through to the parked-future accumulator otherwise.

### 4.4 The echoed correlation is correlation, not the request body

`extension_response_args` now drops any echoed request field whose string exceeds
`PUBLIC_INVOCATION_STRING_BYTES` — the bound every structurally addressed command argument already
crosses. A string the shell could not have SENT as an argument is not one the SDK hands back. The
procedural evaluate request's `inputJson` (the operator's whole input) stops riding back into the
guest on every answer; `nodeHash`/`windowId`/`handle` — the actual correlation — are untouched.

### 4.5 The answer is a pack on BOTH sides (second defect, found on the way)

The ABI field is `pack` and `📜️.wit:8-10` says so explicitly ("no JSON string ... anywhere on this
ABI's data path"). The shell packs — `outcome = { ok: encodePackValue(JSON.parse(outputJson)) }`,
`🏛️ShellHost/🟦️.tsx:1648` — and the SDK was doing
`DslValue::String(String::from_utf8_lossy(bytes))`, i.e. handing the app the CONTAINER BYTES as
text. Measured with the real `encodePackValue` under `bun`:

```
encodePackValue(JSON.parse('{"channels":{"sum":{"schema":"number","value":3}}}'))  ->  58 B
new TextDecoder().decode(those 58 bytes)  ->  "\u0001\u0006number\u0001\u0001\u0011\u0010\u0001\u0007\bchannels..."
JSON.parse(that text)                     ->  SyntaxError: Unrecognized token
```

So every browser-served answer would have reached `flowEvalResolve` as mojibake and
`seed_node_cache` would have failed even once the trap was gone. The native suite never saw it
because `testkit::settle_extension_invocations` handed the capability's RAW JSON bytes over — a
shape the shell never sends. Both halves are fixed in the two framework-owned places:
`extension_response_args` decodes the pack (`store::pack_rt::decode_wire_value`) and serialises the
app's own value into `outputJson`, faulting `extension.answer-not-a-pack` if the bytes are not one;
`settle_extension_invocations` packs the served answer exactly the way the shell does. No plugin
file changed, and there is now ONE shape.

STILL OPEN, adjacent, NOT fixed here (flagged for the coordinator): the FAULT arm has the same
asymmetry in reverse — the shell sends `encodePackValue(fault)` while `🌐host/🦀️.rs:49` decodes with
`dsl::decode_fault_bytes`. Every `extension.missing` / `extension.invoke-failed` reaching a guest
today decodes from a shape it was not written in.

## 5. Payload sizes, before to after

Same probe, same staged module: `🗑️generated/realloc-probe-4.txt` (after) against
`realloc-probe-3.txt` (before). What matters is the LARGEST SINGLE `cabi_realloc` — the one number
that decides whether the guest aborts.

| answer | before: events / largest block | after: events / largest block |
|---|---|---|
| 1 KiB | 1 / 4 360 (the poll return area) | 1 / 4 360 |
| 64 KiB | 1 / **65 536** | 1 / **65 536** |
| 256 KiB | 1 / **262 144** | 4 / **65 536** |
| 1 MiB | 1 / **1 048 576** | 16 / **65 536** |
| 4 MiB | 1 / **4 194 304** | 64 / **65 536** |
| 16 MiB | 1 / 16 777 216 | refused at the host: `extension.answer-too-large` |
| 572 MiB | 1 / 600 000 000 -> **ABORT** | refused at the host |

Guest linear memory over the same run: 11.88 MB instantiated -> 35.00 MB after `instance-open` ->
35.19 MB settled (a settled idle turn moves it by **0 B**; the per-turn fixed cost is jco's 4 360 B
return area and it IS released), and a 1 MiB paged answer costs 35.31 -> 36.13 MB, i.e. **7.1 % of
the 512 MiB ceiling** — against 82-87 % for a single unpaged 384 MiB answer.

### 5.1 Paging must not move the oversized block into the event LIST

`realloc-probe-4.txt` shows the trap trying to relocate: at 64 MiB the largest block becomes
81 920 B, at 384 MiB it becomes 491 520 B — jco lowers the events list as ONE
`cabi_realloc(0, 0, 8, events * 80)`. The host-answer ceiling closes that door too: a maximal 8 MiB
answer is 129 events, and at a generous 256 B/event that list is 33 024 B, inside the same one-page
bound. Pinned by `a_maximal_answers_own_event_list_still_fits_one_page`.

## 6. Laws

| law | where | verdict |
|---|---|---|
| `the_budget_matches_the_neutral_schema` (extended) | `⏱️trace/🧮️memory/🧪️tests/🔬️memory/🦀️.rs` | PASS |
| `the_host_answer_ceiling_is_a_whole_number_of_pages_inside_the_install_headroom` | same | PASS |
| `a_maximal_answers_own_event_list_still_fits_one_page` | same | PASS |
| `a_paged_answer_reaches_the_response_action_whole` (1 MiB, paged) | `⚛️reactor/🧪️tests/🔬️extension-continuation/🦀️.rs` | PASS |
| `an_over_ceiling_answer_faults_instead_of_growing_the_guest` | same | PASS |
| `the_echoed_correlation_drops_a_request_body_the_shell_could_not_have_sent` | same | PASS |
| the four pre-existing continuation laws | same | PASS (re-pinned on the pack shape) |
