# 🧠️ Sourcing guest memory: why 322 MB, why `176 bytes failed`, what was fixed

Ticket `26/09/01/SOURCING-END-TO-END`, 2026-11-23. Component `semio-hub-sourcing-component` (core wasm 223 MB, prototype profile, memory `min 354 pages = 23.2 MB`, `max 8192 pages = 536 870 912 B`).

## 1. Verdict

The guest does not hit 512 MB because the Demo document is big (pack 1 125 B + spr 252 B). It hits it because the guest **spins**: the cold-pair document load never reaches a terminal state, the actor answers `MoreWork` on every turn (`sources=["document_load"]`, 118 947 turns in one 70 s probe), and **every turn leaks guest linear memory** through the component-model async-export parameter area. Linear memory only grows, so a spin that lasts long enough walks to the `maximum` and the next `cabi_realloc`/`alloc` of any size (176 B here) is refused by `memory.grow` and aborts.

Chain, each link measured:

1. `setContributions` stalls (other agent) and the Demo load behind it never settles. The shell shows `Document load: Loading document: 121 of 121` forever (probe `aria-en-US.yml`).
2. The reactor answers `MoreWork` on every turn with `document_load` as the only source (115 000+ consecutive turns, no ceiling anywhere).
3. Each turn leaks 335 B to 1.4 KB of linear memory (section 3).
4. At the `maximum`, `memory.grow` returns -1 and the std allocator aborts: `memory allocation of 176 bytes failed` then `unreachable`, then every later input traps. The `322174976 B` line is only the last diagnostic printed before the abort: it prints at the first turn end past 60 %, and the guest then keeps growing inside one long turn.

## 2. Answers to the three questions

### 2.1 Where the budget and the 60 % ceiling live, and whether the host refuses grow

- The only limit is the **module's own declared memory `maximum`**: `.cargo/config.toml` `[target.wasm32-wasip2]` `--max-memory=536870912`, mirrored by `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs` and its schema `🧬️schema/🔣️.json`. Verified in the staged core wasm: `memory min 354 max 8192`.
- No TS host (browser plugin instance, shard worker, `ShellHost`) and no wasmtime host declares or enforces a second ceiling. `grep` of `WebAssembly.Memory` / `536870912` under `🛍️products/💻️os` finds only flow-preview fixtures. jco instantiates whatever the module declares.
- The failure at 176 B is therefore the real `maximum` (a refused `memory.grow`), not a host-side refusal and not an allocator cap. dlmalloc grows in 64 KiB units, so a 176 B request fails only when the whole memory is exhausted.

### 2.2 Is the 60 % install-peak ceiling correctly designed, does it refuse grow after install?

It never refuses anything, by design and in code. `GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT = 60` is (a) the ceiling a native law compares a boot + install peak against, and (b) a **one-shot `eprintln`** in `trace_guest_memory_pressure` (`⚛️reactor/🔄️turn/🔦️more-work-sources-and-tracing/🦀️.rs`) on the first turn end past it. It is a diagnostic, so as a guard it was incomplete: nothing in the guest turned "near the maximum" into a typed refusal, which is why the only visible outcome was the abort. Fixed in section 4.

### 2.3 Boot baseline (eager allocation)

Measured on the live guest with `SEMIO_RUNTIME_DIAGNOSTICS=1` (localStorage, `probe-sourcing-memory.ts`):

| turn | linear memory | note |
|---|---|---|
| instantiate | 23.2 MB | static image |
| 1 | 88.5 MB | |
| 4 | 120.7 MB | +26.5 MB in one turn |
| 107 | 124.9 MB | steady state after boot |
| 10 000 | 128.3 MB | spin begins to show |
| 118 946 | 165.1 MB | +37 MB of pure leak in 70 s |

Boot is 120 MB of 512 MB (23 %). Native heap witness for boot of the same app (`HeapWitness`, curation lib): retained 44.9 MB. Allocation-size histogram of boot (backtraced with `backtrace_symbols_fd`):

- `MutationDagFixedSlots<MutationEnvelope>` boxes: 734 x 27 136 B = 19.9 MB (`semio_framework_replication_causal`).
- `RegistryBacking<T>` exact reserves (`semio_plugin_ownership::bounded_ownership::registry_backing`, `Vec<MaybeUninit<T>>`): 64 x 136 064 B (document media store) + 32 x 175 776 B (envelope) + 8 x 172 800 B (peer roster publication) + 4 x 220 928 B (command admission) + presence peers etc., about 17 MB.
- `deflate_encoder` 128 KiB hash tables (transient, 18 at boot).

This is the bounded-ownership design: all capacity is reserved at boot so no allocation can fail later. It is not a defect by itself, and 120 MB leaves 390 MB. Shrinking it means retuning slot counts per app (design work, not done). It matters only because the leak below spends the remainder.

## 3. Root cause of the leak: async-export indirect parameter area

Shipped core wasm exports (parsed from `native-stage/🪵️sourcing/transpiled/semio_hub_sourcing_component.core.wasm`):

```
[async-lift]semio:framework/reactor@1.0.0#poll                  (i32) -> i32
[async-lift]semio:framework/reactor@1.0.0#admit-actor-context   (i32) -> i32
[async-lift]semio:framework/reactor@1.0.0#admit-actor-input     (i32) -> i32
[async-lift]semio:framework/reactor@1.0.0#close-actor-context   (i32) -> i32
[async-lift]semio:framework/reactor@1.0.0#close-actor-output    (i32) -> i32
[async-lift]semio:framework/reactor@1.0.0#stage-cold-pair-page  (i32) -> i32
[async-lift]semio:framework/jobs@1.0.0#start-job / step-job / read-job-output / ...   (i32) -> i32
[async-lift]semio:framework/{checkpoint,describe,codec}@1.0.0#*                       (i32) -> i32
[async-lift]semio:framework/reactor@1.0.0#stage-command-page    (13 flats) -> i32      <- direct, leak-free
```

A single `i32` param means the canonical ABI passes the parameters **indirectly**: the host allocates a block in guest memory with the guest's `cabi_realloc`, writes the params, and the callee must free it. `wit-bindgen` frees it only for sync exports (`wit-bindgen-core` `abi.rs`: `if sig.indirect_params && !async_`). Upstream bug, open: bytecodealliance/wit-bindgen#1757 "Async exports never free the block of indirect parameters", reproduced against 0.57.1 (the version in the registry here); no released fix as of 0.63.0.

This is the same defect ticket `26/09/09/PROCEDURAL-3D-END-TO-END` (`📓️guest-memory-retention-2026-09-10.md`) fixed for `poll` by moving bulk payload off its signature (`stage-command-page` is still direct, 13 flats). It regressed with the retained-turn authority rework: `reactor.wit` `actor-context-budget` is now `retained-turn-input` (11 flats) + `retained-policy` (5) + `fuel` + `deadline-us` + `cancelled` = **19 flat values > 16**, and `job-budget` is the same shape. So every per-turn `admit-actor-context`, `admit-actor-input` (22 flats with the wire), `poll` and every job step leaks a 152 to 168 B block. Matches the live curve:

- `document_load`-only spin turns (2 indirect calls: `admit-actor-input` 168 B + `poll` 152 B): 3.3 MB per 10 000 turns = 335 B/turn.
- `typed_operation` + `command_ingress` turns (job steps and page staging in addition): one 64 KiB page per 46 turns = 1.4 KB/turn.

Native tests cannot see it: it is a component-model ABI artifact of the guest build, not Rust heap state. The native Demo load measured here (`+50 MB` retained) is a **measurement artifact**: a peer's `[DEBUG] post27136` `Backtrace::force_capture()` in `🧵️job/⏱️context/🦀️.rs:317` makes std symbolize and cache the whole debug info (the 24/48 MiB doubling `Vec` in the histogram is `gimli` `Object::parse`). Without it the native load allocates under 10 MB. That debug line should be removed by its owner.

### Fix options for the leak (not applied: cross-cutting, see section 6)

1. Keep every async export at <= 16 flat params. Cleanest within the schema: a **sync** `stage-actor-context-budget` / `stage-job-budget` export (sync exports free their block) and `poll()`, `admit-actor-context()`, `admit-actor-input(wire)`, `close-*()`, job exports taking no budget; the guest reads the staged budget at entry. Same precedent as `stage-command-page`. Touches `reactor.wit`, `jobs.wit`, `checkpoint.wit`, `codec.wit`, `describe.wit`, the guest macros in `📣️macros/⚛️wp4-reactor`, `⚛️reactor/🎟️context/🌉️call/🎭️exports`, the wasmtime host calls in `🖥️host/🎭️guest-runtime`, and the TS bridge `🌉️component-bridge-source` plus the shard-client/retained-turn TS copies.
2. Patch the generator: free the block between future creation and the first `.await` (the fix proposed in wit-bindgen#1757). One change, fixes every guest, but vendors a patched `wit-bindgen-rust` and rebuilds the whole workspace.
3. Do not try a guest `cabi_realloc` override: it cannot tell a parameter block from a payload buffer.

## 4. What was changed (typed refusal instead of spin and abort)

All guest-side, schema-first, no compatibility layer.

1. **Refusal ceiling in the neutral memory budget.**
   - `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧬️schema/🔣️.json`: new `refusalPercent` const 85 (required).
   - `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs`: `GUEST_LINEAR_MEMORY_REFUSAL_PERCENT`, `guest_linear_memory_refusal_ceiling_bytes()`, `guest_linear_memory_over_refusal_ceiling()` (always `false` off wasm).
   - `🧰️framework/🔨️modules/⏱️trace/🦀️.rs`: re-exports.
   - `🧪️tests/🔬️memory/🦀️.rs`: schema pin, plus law `the_refusal_ceiling_sits_between_the_install_peak_and_the_budget` (above 60 %, leaves at least 4 x 8 MiB for a refusal to allocate its own fault).
2. **Cold-pair document load never spins unbounded and never allocates into an abort.**
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🦀️.rs`: `COLD_PAIR_LOAD_IDLE_POLL_CEILING = 4096`, `ColdPairDocumentLoad::observe_progress(completed, total)`.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧊️cold-pair-loads-and-process-pool/🦀️.rs`: `step_cold_pair_document_load` now refuses with a typed fault through `refuse_cold_pair_document_load`, cancelling the admitted archive operation (the previous document stays): `plugin.cold-pair.guest-memory-ceiling` when over the 85 % ceiling (checked before admission and every poll), `plugin.cold-pair.load-stalled` when the archive status `(completed, total)` is unmoved for 4096 consecutive polls. The host sees `cold-pair` terminal `Fault`, the shell shows `load-failed`, the actor goes idle, memory stops growing.
   - Law `a_cold_pair_load_that_never_moves_is_declared_stalled_at_exactly_the_idle_poll_ceiling` in `📥️cold-pair/🧪️tests/📥️cold-pair/🦀️.rs`.

Validated natively: `semio-framework-trace` laws (2 passed), `semio-plugin-reactor-surfaces` law (1 passed), `semio-plugin-reactor-turn` type-check.

The 4096-poll ceiling bounds the leak of a wedged load to about 1.4 MB (4096 x 335 B), against 160+ MB unbounded. It does not fix why the load wedges: that is the `setContributions` / publication Blocked family owned by the other agents (`foreign_step_presence` never recorded for hand-written preparers, `🧵️job-stall-root-cause.md`). Once that lands the load completes in a few turns and neither guard trips.

## 5. Evidence kept in the ticket

- `probe-sourcing-memory.ts`: the probe with `SEMIO_RUNTIME_DIAGNOSTICS=1` armed; output `🗑️generated/probe-memory/` (the 159 MB `console-en-US.txt` was deleted after analysis).
- `heap-measure-demo-load.rs.txt`: the native `HeapWitness` + allocation-size-histogram + `backtrace_symbols_fd` measuring test (removed from the curation unit file; paste back to re-measure).
- `native-cargo-mem.sh` / `native-cargo-root.sh`: copies of `native-cargo.sh` on APFS-cloned target dirs (`🗑️generated/native-mem`, `🗑️generated/native-root`) so measuring runs do not queue behind peers' cargo lock.

## 6. Requests to the coordinator

- **Wasm rebuild needed**: yes. Guest code changed in `semio-plugin-reactor-surfaces` and `semio-plugin-reactor-turn` (and the trace crate), which every plugin component links. `ONLY_HUB=1 ONLY=semio-hub-sourcing-component` is enough to verify sourcing; the three extension components relink the same reactor.
- After restage and server restart, expected probe result with the stall still unfixed: no `memory allocation of ... failed`; after about 4096 idle polls the load ends with `load-failed` and `plugin.cold-pair.load-stalled`; `setActiveExample` / `noteShellCommand` keep working. With the stall fixed: no fault at all.
- Decision for the ABI leak (section 3, option 1 or 2): it is the only thing still converting idle time into memory growth (335 B per MoreWork turn, 1.4 KB per job-heavy turn; 512 MB lasts about 1.5 M turns, minutes under a spin, hours of normal use). I did not start it because it rewrites five WIT files and three host implementations that peers edit concurrently and needs a full 20-minute rebuild cycle to validate. A language-agnostic guard for it would be a law that counts flat values of every `async func` in `🧬️schema/*.wit` and requires <= 16.
- Owner of `🧵️job/⏱️context/🦀️.rs:317`: remove the `[DEBUG] post27136` `Backtrace::force_capture()`; it inflates every native heap measurement by about 50 MB and, in a wasm build with symbols, would cost the same.
