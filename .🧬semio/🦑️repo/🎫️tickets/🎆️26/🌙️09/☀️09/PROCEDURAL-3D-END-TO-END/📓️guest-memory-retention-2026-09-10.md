# 🧮️ Guest Linear Memory Retention — What Accumulated Per Turn, and the Fix

Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. Lane: *why does wasm linear memory never come back down*.
The upstream trigger (the shell pushing ~398 KB of all-thirteen-plugins contributions through a 4 KiB
string envelope as 99 sequentially awaited guest turns) is a different lane's and is untouched here.

## 1. Answer

The per-turn growth is **not** in the events arrays, **not** in retained UI / `BuiltNode` state, **not**
in the guest's operation/mutation log, and **not** in dlmalloc failing to reuse freed blocks.

It is the **canonical-ABI indirect parameter area of the async-lifted `reactor.poll` export**:

> **4 360 bytes of guest linear memory allocated by the host through `cabi_realloc` on every single
> turn, and never deallocated.**

`wit-bindgen` emits the `GuestDeallocate` that frees a guest export's indirect parameter area only for
the *synchronous* lift. `~/.cargo/registry/src/*/wit-bindgen-core-0.57.1/src/abi.rs` (≈1276–1290):

```rust
// This was dynamically allocated by the caller (or async start
// function) so after it's been read by the guest we need to
// deallocate it.
if let AbiVariant::GuestExport
| AbiVariant::GuestExportAsync
| AbiVariant::GuestExportAsyncStackful = variant
{
    if sig.indirect_params && !async_ {
        …
        self.emit(&Instruction::GuestDeallocate { size, align });
    }
}
```

`poll` is `async func`, so `async_` is true, so the branch never fires. The area is leaked by
construction — one leak per call, for the one call the host makes hundreds of thousands of times per
session.

A parameter area only exists when the flattened parameter count exceeds the canonical ABI's
`MAX_FLAT_PARAMS = 16`. The old `poll` was
`poll(events: list<event>, command-page: option<command-ingress-page>, cold-pair-page: option<cold-document-pair-page>, budget: budget)`,
and `command-ingress-page` embedded `byte-page.page` — a fixed record of 64 blocks × 8 `u64` = 4 096 B
**passed by value on every turn even though it is `None` on virtually all of them**. That single field
is what pushed the call over 16 flats and set the 4 360 B area size.

## 2. Evidence

### 2.1 Direct production evidence (jco transpile of the shipped component)

`…/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.js`

Before:

```js
async function poll$1(arg0, arg1, arg2, arg3) { var ptr0 = await realloc0Async(0, 0, 8, 4360); … }
```

After (same file, restaged 23:00):

```js
async function poll$1(arg0, arg1) { … var result100 = await realloc0Async(0, 0, 8, len100 * 80); … }
```

The fixed 4 360 B allocation is gone. The remaining `realloc` is the `list<event>` body, which the
guest owns and drops.

Controls in the same file, all with ≤16 flats and therefore no parameter area:
`restore(list<u8>)` → `paramCount: 2` (direct), `startJob(…)` → `paramCount: 5` (direct).

### 2.2 Arithmetic

* 4 360 B request + dlmalloc's 8 B chunk header = **4 368 B**; measured **4 437.3 B/turn** — 1.6 %
  apart, the remainder being 64 KiB wasm page quantisation (growth arrived in 131 072 B steps every
  32 turns).
* 512 MiB ÷ 4 437 B ≈ **118 000 turns** to exhaust the budget.
* The failing capture (`probe-b14-longsettle-2026-09-10T19-57-24`) hit `322 371 584 B` at 65.7 s ⇒
  ≈ 69 000 turns in 66 s ≈ **1 000 turns/s**, consistent with `settlePluginTurn`'s 4 096-continuation
  budget across the 225 observed command settles.

### 2.3 Hypotheses falsified by measurement

| Hypothesis | Measurement | Verdict |
| --- | --- | --- |
| dlmalloc loses freed blocks on wasm | Standalone `wasm32-unknown-unknown` cdylib (`🔬️dlmalloc-probe/`) run under local `wasmtime`: `cycle_growth` 4 KiB…1 MiB, `turn_growth` with 40 400 B / 189 328 B transients, `doubling_growth` — **all returned 0 B** of `memory.size` growth after warm-up | rejected |
| The events arrays retain | Growth was identical with 0 or 1 live instance and on turns carrying no events at all | rejected |
| The poll task box (future) retains | Prior lane shrank it 189 328 → 40 400 B; curve changed **by zero** | rejected |
| The MoreWork spin drives it | Only **2 of 512** turns answered MoreWork, at `[0, 1]`; the other 510 leaked identically | rejected |
| It is the export call itself | `checkpoint()` — zero params, therefore no parameter area — measured **0 B/turn** on the same component | **confirms the ABI cause** |

### 2.4 CQRS / event-sourcing check (asked explicitly)

The event stream is **not** replayed or re-sent per turn. `typedOperationAcknowledgements(result)`
builds each turn's events from the *previous* turn's result only, and the guest-side per-turn events
figure (§4) shows turns with `eventsBytes=0` growing exactly as much as turns carrying payload. The
defect was in the ABI shape of one export, not in the event-sourcing shape. No architectural change to
the command/event model was needed or made.

## 3. The fix

Take **all** bulk payload off the async `poll` signature so its flattened parameter count drops to 7
(≤16), which makes the canonical ABI pass parameters directly and removes the leaked area entirely.
Pages get their own staging exports.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`:

```wit
stage-command-page: async func(cursor: command-page-cursor, bytes: list<u8>) -> result<_, plugin-error>;
stage-cold-pair-page: async func(page: cold-document-pair-page) -> result<_, plugin-error>;
poll: async func(events: list<event>, budget: budget) -> result<turn-result, plugin-error>;
```

* `record command-ingress-page` and `use byte-page.{block, page}` are **deleted** from `interface
  reactor`. The page crosses as a `list<u8>` the guest owns and drops, not as a 4 KiB fixed record.
* `stage-command-page` is 13 flats ⇒ also passed directly, so it allocates no area either.
* `stage-cold-pair-page` is indirect, but it is paid once per cold page rather than once per turn.
* Staging is a one-slot authority per turn: staging twice without an intervening `poll` is a host
  defect and **faults** (`plugin.turn-page-staging`) rather than silently replacing an unread page.
* The guest holds staged pages **boxed** (`Option<(CommandPageCursor, Box<FixedCommandPage>)>`). A
  4 KiB page by value made every `.with()` take memcpy it on the deepest frame of the turn, which
  overran the shadow stack — measured as `memory fault at wasm address 0xffff3798` (a stack-pointer
  underflow) on turn 0.

Greenfield: no compatibility layer, no second poll arity, no deprecation. The 512 MiB budget is
unchanged.

### 3.1 Second defect found and fixed on the way

`execute_turn` classified and reported guest traps off `trap.to_string()`, which is only a wasmtime
error's **first line** ("error while executing at wasm backtrace: …"). The trap code itself — "out of
fuel", "wasm trap: unreachable", the `rust_oom` abort this ticket chased — lives in the source chain
and was being thrown away, so every guest trap arrived as an unattributed backtrace and the
`fuel`/`epoch` classification could never match. Now formatted with `Debug`, which renders the chain.
That change is what turned an undiagnosable turn-0 trap into `memory fault at wasm address
0xffff3798 … out of bounds memory access` in one run.

### 3.2 Build note (not a code defect, but it cost a cycle)

`.cargo/config.toml` sets `-zstack-size` for `wasm32-unknown-unknown` only. Plugin components get
their 8 MiB shadow stack from `PLUGIN_WASM_STACK_BYTES` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`. A raw
`cargo build -p semio-s-plugin-procedural --target wasm32-wasip2` links with wasm-ld's 1 MiB default
and traps on turn 0 inside `ColdDocumentPairIngressRegistry<1024>::new`. Always build the harness
component with `cargo rustc … -- -C link-arg=-zstack-size=8388608`.

## 4. Instrumentation added (behind the existing diagnostics gate)

`⚛️reactor/🔄️turn/🦀️.rs`, under `semio_framework_trace::runtime_diagnostics_enabled()`, all
`[DEBUG] `-prefixed:

```
[DEBUG] guest linear memory turn={turn} bytes={bytes} delta={delta} percent={…} events={…} eventsBytes={…} elapsed_us={…}
```

`eventsBytes` is computed at the component boundary by `wit_events_payload_bytes` — every `pack` /
`list<u8>` / `string` field of every `event` variant, which is all a turn can retain that scales with
its input. Fixed-shape variants (`timer`, `wake`, `surface-resized`, …) contribute nothing by
construction, so `eventsBytes=0` genuinely means the turn carried no payload.

## 5. Before / after — measured

Harness: `🔌️plugin/🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs`, law
`the_poll_export_keeps_guest_linear_memory_flat_across_turns`, `SEMIO_POLL_TURN_LEAK_TURNS=512`,
staged `semio_s_plugin_procedural.wasm`, wasmtime host, 512 idle turns.

| | instantiate | turn 128 | turn 511 | slope |
| --- | --- | --- | --- | --- |
| before | 12 451 840 B | 37 027 840 B | 38 731 776 B | **4 437.3 B/turn** |
| after | 12 451 840 B | 36 438 016 B | 36 438 016 B | **0.0 B/turn** |

After-curve, sampled every 32 turns — completely flat:

```
turn   0: 36438016 B      turn 256: 36438016 B
turn  32: 36438016 B      turn 288: 36438016 B
turn  64: 36438016 B      turn 320: 36438016 B
turn  96: 36438016 B      turn 352: 36438016 B
turn 128: 36438016 B      turn 384: 36438016 B
turn 160: 36438016 B      turn 416: 36438016 B
turn 192: 36438016 B      turn 448: 36438016 B
turn 224: 36438016 B      turn 480: 36438016 B
```

`2 of 512 turns answered MoreWork, at [0, 1]` — unchanged before and after, so the flat curve is not
an artifact of the guest doing less work.

Test result: `1 passed; 0 failed … finished in 118.11s`.

## 6. Runtime verification in the browser

Shared React dev serve on `http://127.0.0.1:6018/?plugin=generation3d` (still running; not restarted).
Component restaged at 23:00 with the new WIT — the transpiled `poll$1` is the 2-argument form above.

**Boot probe** — `--settle=260 --label=mem-1`, capture
`🗑️generated/probe-mem-1-2026-09-10T21-07-46/`:

* settled at **9.9 s** with 10 windows and 3 canvases. The failing baseline reached `rust_oom` at
  116.1 s and only rendered at 121.0 s.
* **zero** matches for `rust_oom`, `process::abort`, `abort_internal`, `memory access out of bounds`
  in `console.jsonl` / `page-faults.jsonl`.
* the unconditional install-peak line (`guest linear memory at … past the 60 % install-peak ceiling`)
  **never printed** — the guest never reached 60 % of the 512 MiB budget. The baseline printed it at
  65.7 s / 322 371 584 B.

**Interact probe** — `--mode=interact --settle=260 --label=mem-2-interact`, capture
`🗑️generated/probe-mem-2-interact-2026-09-10T21-08-45/`:

* **146.8 s** of live page: `example` → `hover` → `select` → `orbit`, every step `ok`.
* **67** `command ingress settled status=command-complete` — the new `stage-command-page` path is
  exercised end to end and completes; this is the runtime confirmation that staging works, not an
  inference from compilation.
* again **zero** `rust_oom` / abort / out-of-bounds / install-peak matches.

Still open and **not** this lane's: `meshes=0`, and the page faults
`fixed typed-operation and segmented-output authorities did not pre-admit the exact operation slot`,
`missing field 'locale'`, and `[DEBUG] command ingress exceeds 64 pages` (the 398 KB contributions
push — the trigger lane).

## 7. Other checks run

* `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev` — **EXIT=0**.
* `cargo check -p semio-framework-plugin-host --all-targets` — **EXIT=0**.
* `cargo rustc -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev -- -C
  link-arg=-zstack-size=8388608` — **EXIT=0**, 82 MB component.
* `vitest run --config 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts` —
  209 passed, 12 failed. All 12 failures are pre-existing and unrelated: ajv cannot resolve
  `https://semio.tech/schema/framework/value/schema.json#/$defs/NonZeroU64` from the actor-return
  schema, failing the `ActorReturn*` / `ActorReturnResponse*` canonical-vector oracles. Nothing in
  `📮️shard-client` or `📃️page` failed.

**Not runtime-verified:** `🧵️backbone-worker.ts`'s document-actor poll lane was updated to the new
staging shape but is not exercised by the React probe, so its behaviour is unconfirmed. The
`🧪️ticket-owned-browser-host-staging` bridge law was rewritten for the new shape but its project was
not run in this session.

## 8. Files changed

Schema
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`

Guest
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

Wasmtime host
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime/🦀️.rs`

Browser host
* `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`
* `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts`

Tests
* `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`

Ticket inputs kept
* `🔬️dlmalloc-probe/` — the standalone allocator probe that falsified §2.3's first hypothesis.
