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

(filled in below as it landed)
