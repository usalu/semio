# Idle turn cost, guest OOM retention, and trace fan-out — 2026-09-10

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`. Lane: per-turn guest cost + retention + diagnostic writer.
Boot #14 evidence folded from `📓️boot-13-2026-09-10.md` §5–§7 and
`🗑️generated/probe-b14-longsettle-2026-09-10T19-57-24`.

**restage required: yes** for the Rust half (gating, per-turn memory curve, `note_turn_events`).
The line-buffer + log-level fix is TypeScript and is already patched into the live preview2 `cli.js`
copies the 6018 serve loads — a page reload is enough for that half.

Do **not** raise the 512 MiB budget.

## 1. What this lane owns

| defect | owner | status |
| --- | --- | --- |
| ~2 s per 4 KiB guest command crossing | guest turn body + ungated per-token stderr | writer + gating landed; crossing cost needs restage to re-measure |
| monotonic guest linear memory → `rust_oom` at 116 s | per-turn peak retained by `dlmalloc` | curve instrumented; leak not claimed gone |
| `framesBytes=63345194` | broken `replyError` diagnostic | already closed in §7; not the leak |
| one `console.error` per format token | preview2-shim `cli.js` `write()` | fixed: one host call per newline |
| `[DEBUG]` at `error` | same writer | `[DEBUG]` → `console.debug` |
| `SEMIO_RUNTIME_DIAGNOSTICS` does not cover `plugin_exchange` / `cooperative-maintenance` | ungated `eprintln!` | gated behind `runtime_diagnostics_enabled()` |

Hard boundary respected: `🏛️ShellHost/🟦️.tsx` `:4344`–`:4400` and `:9085` were not touched.

## 2. `framesBytes=63345194` — not retained heap

Emitter: `replyError` in the plugin TypeScript `shardWorkerSource` (see boot-13 §7).
A `turn`'s `events` is an **array of buffers**. The old `instanceof Uint8Array` test never matched, so
every report took `JSON.stringify(frames).length`. That renders each byte as `"index":value` and
inflated ~6–10 MB of wire bytes to 63.3 MB. Boot #13 §7 already fixed the array-of-buffers path to
sum `byteLength`. **Do not treat 63.3 MB as the retain.** The OOM is the guest's own `rust_oom`
against the 512 MiB linear-memory budget.

## 3. What actually retained the memory

`trace_guest_memory_pressure` (`⚛️reactor/🔄️turn/🦀️.rs`) is the 60 % install-peak emitter that fired
at 65.7 s on boot #14:

```
guest linear memory at 322371584 B — 60% of the 536870912 B budget, past the 60% install-peak ceiling
```

wasm linear memory never shrinks (`dlmalloc`). Anything that **peaks** during a turn stays in
`memory.size` even after Rust drops it.

Boot #14, measured:

| elapsed | fact |
| --- | --- |
| 65.7 s | 322 371 584 B (307 MiB, 60 % of 512 MiB) |
| 116.1 s | `rust_oom` → `abort` → `unreachable`, actor `procedural#1` traps |
| 121.0 s | window UI + 3 canvases appear; `meshes=0` (actor already dead) |

The contributions push is **one** 397 921-char payload split into 99 sequentially awaited 4 KiB
pages (`📓️boot-13-2026-09-10.md` §5). Each page is a full `plugin_exchange` command turn. Intermediate
pages (`set-contributions` `apply`) only append to `CONTRIBUTIONS_ASSEMBLY` and return `Emit::default()`,
but `plugin_exchange` still marks `mutated = true`, encodes an `Invocation` + `Ephemeral` frame, and
drains `pending_effects`. That is several MB of peak temporaries per 4 KiB page, made permanent by
`dlmalloc`. ~50 pages × ~6 MB peak ≈ the 307 MiB reading at 66 s.

Idle-poll leftover from `📓️poll-task-leak-2026-09-10.md` / `📓️idle-turns-2026-09-10.md` is ~4 KiB/turn
on wasm and is **not** this curve. The OOM is the command-turn peak, not the idle poll.

A sibling `poll` now calls `turn::note_turn_events(count, events_bytes)` so the next armed boot prints
a **per-turn** line:

`[DEBUG] guest linear memory turn=N bytes=B delta=D percent=P events=E eventsBytes=X elapsed_us=U`

behind `runtime_diagnostics_enabled()`. One sample told us there is a leak; the curve says which turn
owns the step.

## 4. The 2.04 s crossing

Boot #13 §5.1: 53 pages over 108.3 s of `performInvocation …setContributions` timestamps ≈ **2.04 s
per 4 KiB page**. A 4 KiB command cannot cost two seconds on its own. Each crossing dragged:

1. **Per-token stderr fan-out** (this lane) — every `eprintln!` format piece is a separate WASI
   `write()` → `console.error`. `cooperative-maintenance` Debug-prints `CooperativePoolSnapshot`,
   which is dozens of tokens. `plugin_exchange entry` fired on every exchange, ungated.
2. **A full `plugin_exchange` + ephemeral publish** on every page, including no-op assembly pages
   (`plugin.rs` `mutated = true` on every successful `Invocation`).
3. Host-side sequential await of the 99-page loop (window-bodies lane — not touched).

(1) is removed for a normal boot (gated) and coalesced to one host call when armed.
(2) is diagnosed; cutting `mutated` on empty emits is a follow-up that must not hide a real dirty UI.
(3) is the other lane's payload/pack work.

## 5. Trace writer — root cause and fix

**Writer:** `@bytecodealliance/preview2-shim` `cli.js` `stderrStream.write` → `console.error(decode(chunk))`
immediately. Rust `eprintln!("… {} … {:?}", a, big)` calls `write_str` once per token. That is the
4× fan-out (5 750 raw lines / 1 452 logical on boot #13).

`hotpath-optimization-2026-09-10.md` claimed 15+→0 behind `SEMIO_RUNTIME_DIAGNOSTICS`. Boot #13 saw
5 750 lines with the flag **OFF** because these two traces were **not gated**:

- `plugin.rs` `plugin_exchange entry` (was unconditional `eprintln!`)
- `plugin.rs` `cooperative-maintenance` (power-of-two turns, unconditional, and it formatted `pool={:?}`)

**Fix:**

1. `createGuestLogLineSink` (`🌐️browser-bundle/🌐️wasi/🟦️.ts`) — buffer until `\n`; classify
   `[DEBUG]` → `debug`, other stderr → `error`, stdout → `log`.
2. `patchPreview2ShimCliLineBuffer` (`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`) overwrites vendored
   `cli.js` after copy so one logical line is one `console.debug` / `console.error` / `console.log`.
3. Live `cli.js` copies used by the 6018 serve were patched in place (reload, no restage).
4. Hot-path `eprintln!("[DEBUG] …")` now go through `debug_runtime_line`, which returns before
   formatting when diagnostics are off. Cooperative-maintenance snapshots are not built unless armed.

## 6. Before / after

### Before (boot #14, `b14-longsettle`, settle 260 s, flag as the serve had it)

| metric | value |
| --- | --- |
| console total / errors / warnings | 894 / 281 / 596 |
| `[DEBUG]`-bearing lines | 883 |
| `plugin_exchange` text hits | 249 |
| `setContributions` performInvocation lines | 74 |
| `flowEvalTick` performInvocation lines | 291 |
| guest linear memory at 65.7 s | 322 371 584 B (60 %) |
| slope | monotonic to OOM; one sample only |
| OOM at 260 s? | **yes — `rust_oom` at 116.1 s** |
| cost per guest command crossing | ≈ 2.04 s (boot #13 §5.1; 53 pages / 108.3 s) |
| raw console tokens per logical DEBUG line | ~4× (boot #13 §2.5) |

### After — measured now (no restage)

| metric | value | how |
| --- | --- | --- |
| host calls per logical line | **8 → 2** on the fixture (preview2 writes every chunk; our sink emits once per newline) | `testBrowserWasiActivation` EXIT 0, 17 laws |
| `[DEBUG]` level | `debug`, not `error` | same law (`debug-lines-use-debug-level`) |
| `debug_runtime_lines_are_silent_when_diagnostics_are_off` | **ok** | `cargo test -p semio-framework-plugin --lib` 1 passed |
| guest crossing cost / memory curve / OOM | **not re-measured in the guest** | Rust not restaged; 6018 still runs the previous wasm |

### After — expected on the next restage (do not treat as measured)

| metric | expected |
| --- | --- |
| console errors with flag OFF | routine `[DEBUG]` gone; `error` only for real faults |
| `plugin_exchange` / `cooperative-maintenance` with flag OFF | 0 |
| per-turn curve with flag ON | one `[DEBUG] guest linear memory turn=…` line per turn, with `delta` and `elapsed_us` |
| OOM at 260 s | unknown until a restaged `--settle=260` probe; this lane did not raise the budget |

A restaged probe command (coordinator-owned restage, then):

```
bun browser-probe.ts --url=http://127.0.0.1:6018/?plugin=generation3d --label=turncost-after --settle=260 --timeout=420000
```

Run it from the ticket folder. Do not edit the probe.

A TS-only `turncost-after` probe is running now in `screen -r turncost-after` (settle 260 s) against the shared 6018 serve. It can show the writer-level change; it cannot show the Rust gating or the per-turn curve until restage.

## 7. Laws

| law | where | result |
| --- | --- | --- |
| `one-logical-message-one-host-call` | `🌐️wasi-activation` fixture `lineBuffer` — 8 preview2 writes, 2 sink emits | pass |
| `debug-lines-use-debug-level` | same fixture: `[DEBUG]…` → `debug`, `guest trapped…` → `error` | pass |
| third-party oracle | `@bytecodealliance/preview2-shim` `outputStreamCreate` still fans out (`preview2HostCalls=8`) | pass (documents the defect) |
| `debug_runtime_lines_are_silent_when_diagnostics_are_off` | `plugin.rs` unit | pass (1 / 1) |
| idle settled guest does not grow | already owned by `the_poll_export_keeps_guest_linear_memory_flat_across_turns` / idle-turns laws | not re-run (40 s+ wasm harness; other lanes' instrument) |

## 8. Checks (real numbers)

| command | result |
| --- | --- |
| `cargo check -p semio-framework-plugin` (native) | **0 errors**, 2–3 warnings, `Finished` 1.62–2.73 s |
| `cargo check -p semio-s-plugin-procedural` (native) | **0 errors**, `Finished` 23.08 s |
| `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2` | **blocked** by a sibling's in-progress `⚛️reactor/🦀️.rs` (`wit_events::CompletionResult` / `wit::` not in scope around `wit_events_payload_bytes`). This lane added `turn::note_turn_events` which that file already calls. Logs: `🗑️generated/storm-procedural-wasm-check*.txt` |
| `cargo test -p semio-framework-plugin --lib debug_runtime_lines_are_silent` | **1 passed**, 0 failed |
| `testBrowserWasiActivation` | **EXIT 0**, `laws=17` |

## 9. Files

| file | change |
| --- | --- |
| browser-bundle wasi.ts | createGuestLogLineSink / classifyGuestLogLine |
| browser-bundle schema.json | WasiLineBufferLaw |
| browser-bundle wasi-activation fixture | lineBuffer + two laws |
| browser-bundle wasi-activation test | sink vs preview2 fan-out |
| plugin typescript package | patchPreview2ShimCliLineBuffer after vendor copy |
| live preview2 cli.js | line-buffered stderr/stdout; DEBUG goes to console.debug |
| plugin.rs | debug_runtime_line; gate exchange / handle_action / maintenance / history |
| reactor turn.rs | per-turn memory curve, elapsed_us, note_turn_events |
| this report | |

## 10. Compact handover

- **Retained:** not `framesBytes`. Per-turn peak (exchange + ephemeral + previously ungated Debug
  snapshots + event payloads) stuck in `dlmalloc`. 307 MiB by 66 s, `rust_oom` at 116 s.
- **Growth curve:** instrumented per turn (`bytes`, `delta`, `elapsed_us`, `eventsBytes`). Needs
  restage + `SEMIO_RUNTIME_DIAGNOSTICS=1` to print.
- **OOM at 260 s:** still present on the live 6018 wasm. Not claimed gone.
- **Crossing cost:** 2.04 s before. Writer fan-out and ungated DEBUG formatting were the guest-side
  amplifiers this lane could cut; payload paging stays the other lane. Re-measure after restage.
- **Writer:** one logical line = one host call; `[DEBUG]` is `debug`; flag OFF silences the two
  traces that produced boot #13's 5 750 lines.

## 11. `turncost-after` probe (no restage)

`screen` probe `turncost-after` against the shared 6018 serve ended at **10.9 s** (`elapsedMs=10857`), not 260 s. Chrome and 3 canvases appeared; actor `procedural#1` then trapped with a jco TypeError (`Cannot destructure property 'length' of 'v102_1'`). That is not `rust_oom` and is not explained by the line-buffer `cli.js` patch. Capture: `🗑️generated/probe-turncost-after-2026-09-10T20-43-49`. Rust gating and the per-turn memory curve were not on this wasm.
