# 🫧️ The per-turn `poll` task box — reproduction, root cause, fix

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: guest reactor poll entry
(`⚛️reactor/**`, the `wasip2` `__export_poll_cabi` / `wit_bridge::poll`, `🔌️plugin/📦️packages/🦀️rust`
component glue). Written 2026-09-10, appended as the lane ran.

## 1. The failure this lane owns

Boot #9b (`📓️runtime-verification-2026-09-09.md`) died ~60 s in:

```
memory allocation of 189328 bytes failed
  → std::alloc::rust_oom
  → unreachable in alloc::boxed::box_new_uninit
     inside wit_bindgen::rt::async_support::start_task (__export_poll_cabi)
     [async-lift]semio:framework/reactor@1.0.0#poll
```

`📓️guest-memory-2026-09-10.md` §5.2 closed on the hypothesis that every `poll` export boxes a
189 328-byte task future that is **never freed**, exhausting the 512 MiB ceiling in ~2 660 turns.
This lane's job was to test that hypothesis on the real wasm target rather than infer it.

## 2. The instruments

| target | instrument | where |
|---|---|---|
| native | `size_of_val` of the real turn future, built with the SAME generic arguments `wit_bridge::poll` instantiates | `⚛️reactor/🔄️turn/🧪️tests/📏️future-size/🦀️.rs` |
| wasm (Wasmtime) | `GuestInstance::guest_linear_memory_bytes()` — the store's `BudgetLimiter` high-water witness, read after every turn | `🖥️host/🦀️.rs` + `🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs` |

Rust computes a generator's layout in MIR (`StateTransform`), before any optimisation, so the size
is a property of the source and is measurable NATIVELY — the only difference on `wasm32` is the
pointer width. That prediction is what makes the two instruments one measurement:

| reading | value |
|---|---|
| native `size_of_val(&poll_kernel(..))`, 64-bit pointers | **252 272 B** |
| × 0.75 (32-bit pointers) | ≈ 189 204 B |
| boot #9b's failed allocation, `wasm32-wasip2` | **189 328 B** |

The 189 328-byte box `start_task` could not allocate **is** the reactor turn future. It is not a
buffer, not a payload, not a page — it is the Rust generator state of `poll_kernel_turn`.

## 3. Where the 189 KB comes from — measured, not guessed

`cargo rustc -p semio-framework-plugin --lib --profile test -- -Zprint-type-sizes` prints every
coroutine's layout. The turn chain, **before**:

| generator | bytes (native, 64-bit) |
|---|---|
| `reactor::turn::poll_kernel` | **252 272** |
| ↳ `reactor::turn::poll_kernel_output` | 247 728 |
| ↳ `reactor::turn::with_turn_execution<_, poll_kernel_turn<..>>` | **243 184** |
| ↳ `reactor::turn::poll_kernel_turn` | **121 584** |
| ↳ its largest `__awaitee`, `plugin_runtime::plugin_exchange` | **76 848** |
| ↳ its largest `__awaitee`, `PluginApp::handle_action_invocation` | 52 248 |

Three findings fall straight out of that table.

### 3.1 `with_turn_execution` stores the turn future TWICE — 243 184 = 2 × 121 584 + 16

```rust
async fn with_turn_execution<T>(future: impl std::future::Future<Output = T>) -> T {
    …
    let mut future = std::pin::pin!(future);
    std::future::poll_fn(move |context| { … future.as_mut().poll(context) … }).await
}
```

`future` is a **parameter**, so it gets a coroutine slot for the whole lifetime of the generator
(`Unresumed` onward). `pin!(future)` then MOVES it into a NEW local that must live across the
`.await`, and rustc gives that local its own slot — the moved-from parameter slot is never reused.
One line of pinning ergonomics costs an exact **121 600-byte** duplicate of the largest generator in
the guest, on every single `poll`.

### 3.2 `plugin_exchange`'s 76 848-byte future is inlined into the turn generator

`poll_kernel_turn`'s four largest variants (`Suspend11/12/17/18`) each carry
`local .__awaitee: 76 848 bytes` — awaiting an `async fn` embeds the callee's ENTIRE generator in
the caller's. Every other awaitee in the turn is small:

| awaitee | bytes |
|---|---|
| `plugin_exchange` | **76 848** |
| `plugin_admit_reserved_presence` | 4 152 |
| `plugin_push_reserved_presence_page` | 4 128 |
| `plugin_dispatch_response_action` | 1 416 |
| `plugin_dispatch_intents` | 1 128 |
| the other eleven | ≤ 688 |

### 3.3 `semio_framework::kernel::Event` is 4 160 bytes, and the turn holds seven 4 KB pages

`Suspend18`'s own locals, once `plugin_exchange` is discounted: `command_page` 4 168 (upvar) +
4 168 (local) + `retained` 4 184 + `page` 4 098 + `Result<(), (Fault, FixedCommandPage)>` 4 208 +
`Option<Event>` 4 160 × 2. `FixedCommandPage` is `[u8; 4096]` BY VALUE, and `Event` carries one.

## 4. The reproduction — and what it falsifies

`🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs` drives the REAL
`target/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm` (81 264 877 B, the 09:41 stage)
through the crate's own Wasmtime host and reads `GuestInstance::guest_linear_memory_bytes()` after
every turn. Four configurations, all on the same component:

| configuration | instantiate | settled | last | B/turn |
|---|---|---|---|---|
| 64 idle turns, **no instance open** | 12 451 840 | 19 726 336 (turn 16) | 19 791 872 (turn 63) | **1 365** (one page, once) |
| 512 turns, `generation3d#editor` open | 12 451 840 | 37 486 592 (turn 128) | 39 190 528 (turn 511) | **4 437.3** |
| 512 turns, `generation3d#editor` open, `max_frames = 0` | 12 451 840 | 37 486 592 | 39 190 528 | **4 437.3** |
| 512 turns, `generation2d#viewer` open | 12 451 840 | 37 224 448 | 38 928 384 | **4 437.3** |

### 4.1 The `start_task` hypothesis is FALSIFIED

`📓️guest-memory-2026-09-10.md` §5.2 concluded that "every reactor `poll` export boxes a ~189 328-byte
task future and that box is never freed". It is freed. Read
`wit-bindgen-0.57.1/src/rt/async_support.rs`:

- `start_task` (line 489) does `Box::into_raw(Box::new(FutureState::new(Box::pin(task))))` and
  immediately calls `callback(EVENT_NONE, 0, 0)`;
- `callback` (line 514) polls the task, and **`if rc == CallbackCode::Exit { drop(Box::from_raw(state)) }`**;
- `Tasks::poll_next` (`spawn_disabled.rs`) sets `self.future = None` the moment the future is `Ready`,
  which drops the 189 KB box, and `FutureState::callback` then returns `Exit` unless a waitable is
  still registered.

A turn that completes synchronously — which every measured turn does — frees its own task box before
`start_task` even returns. **64 idle turns are flat to within one wasm page.** If the box leaked, the
idle configuration would have grown 189 KB per turn too; it does not grow at all.

### 4.2 What is really there: 4 437 B per turn, framework-owned, wasm-only

The growth appears only once an instance is open, is **identical to the byte** for two different apps
in two different artifacts, survives `max_frames = 0`, and the turn result it produces is EMPTY every
single turn:

```
[DEBUG] turn 0 result: status=MoreWork effects=0 patches=0 presence=0 receipt=false
[DEBUG] turn 192 result: status=MoreWork effects=0 patches=0 presence=0 receipt=false
```

So it is not app work, not rendering, and not the WIT lowering of a turn result (there is nothing to
lower). Natively the same shape retains **0 bytes**: `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`
opens an instance, acknowledges its lifetime, spins 256 settled `poll_kernel` turns and weighs the
process heap through `PLUGIN_HEAP_WITNESS` — `settled=35 916 235 after=35 916 235 per_turn=0 B`,
with `SEMIO_RUNTIME_DIAGNOSTICS=1` showing every turn phase (`enter`/`lifecycle`/`ingress`/
`continuation`/`presence-clock`/`render`) at delta 0 or ±16 B.

**Zero retained Rust bytes natively, 4 437 B of linear memory per turn on wasm, for the same turn.**
That difference is the wasm allocator: `dlmalloc` serving a 189 328-byte transient on every single
turn, interleaved with the turn's own small allocations, cannot always place the next one in the hole
the last one left, and ratchets the heap top instead. 512 MiB / 4 437 B ≈ 118 000 turns — which the
browser reaches in ~60 s precisely because the reactor answers `MoreWork` on every turn (the
`MoreWork` spin `📓️wgpu-ui-turn` and the process-pool pump lane both record), and the 189 328-byte
request is simply the largest single allocation in the guest, hence the first to fail.

**The fix is therefore the same either way: the per-turn transient must not be a quarter of a
megabyte.**

## 5. The fix

### 5.1 `with_turn_execution` takes the turn future PINNED, by pointer

`⚛️reactor/🔄️turn/🦀️.rs:169-176` — the parameter is now
`std::pin::Pin<&mut (impl Future<Output = T> + ?Sized)>`, and `poll_kernel_output`
(`🔄️turn/🦀️.rs:241`) does the pinning it already conceptually owned:

```rust
with_turn_execution(std::pin::pin!(poll_kernel_turn(runtime, events, command_page, cold_pair_page, budget, prepare, publish))).await
```

No allocation, no behavioural change, one duplicate generator gone.

### 5.2 `plugin_exchange` is awaited through a box

`⚛️reactor/🔄️turn/🦀️.rs:246-253` adds `plugin_exchange_boxed`, the ONE place the boxing lives (all
four call sites go through it, so the repetition stays in one function as the taxonomy rule asks):

```rust
fn plugin_exchange_boxed<'a, PA: crate::app::PluginApp>(…) -> std::pin::Pin<Box<dyn Future<Output = Result<PluginExchangeOutput, Fault>> + 'a>> {
    Box::pin(crate::plugin_runtime::plugin_exchange(runtime, instance_id, command))
}
```

The turn generator now holds an 8-byte pointer where it held 76 848 bytes, and the exchange state is
allocated only on the turns that actually run a command — most turns never touch it.

### 5.3 The turn chain, after

| generator | before | after | |
|---|---|---|---|
| `poll_kernel` | 252 272 | **53 872** | −78.6 % |
| `poll_kernel_output` | 247 728 | 49 376 | |
| `with_turn_execution` | 243 184 | **24** | |
| `poll_kernel_turn` | 121 584 | 44 784 | |

× 0.75 for 32-bit pointers: the wasm task box `start_task` allocates per turn goes from
**189 328 B to ≈ 40 400 B**.

What remains in `poll_kernel_turn` is exactly seven 4 KB command pages carried BY VALUE
(`command_page` upvar 4 168 + local 4 168, `retained` 4 184, `page` 4 098,
`Option<CommandIngressOwner>` 4 184, `Result<(), (Fault, FixedCommandPage)>` 4 208) plus 18 143 B of
slot padding around them. Getting under a few KB means `FixedCommandPage` stops crossing the turn
frames by value — see §8.

## 6. The measurement AFTER the fix — and what it proves about ownership

The fixed reactor was rebuilt and restaged
(`activate-generation3d-react-dev`, 22 m 55 s, "11 completed components (changed)",
`target/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm` 81 300 657 B at **10:54**), and the
same harness re-run against it:

| configuration | per-turn task box | B/turn |
|---|---|---|
| before (09:41 stage) | 189 328 B | **4 437.3** |
| after (10:54 stage) | ≈ 40 400 B | **4 437.3** |
| after, THREE instances open | ≈ 40 400 B | 4 949.3 |

**Byte-identical.** Shrinking the per-turn transient 4.7× changed the growth by nothing at all, and
opening three instances instead of one changed it by 11 %, not 300 %. So:

- it is not the task box (§4.1 — and now proved twice, by size independence);
- it is not `dlmalloc` fragmentation behind the task box (a 4.7× smaller transient fragments
  differently; the curve did not move by one byte);
- it is not per-instance state (3× the instances, 1.1× the growth);
- it is **per TURN**, and it needs at least one live instance to start.

And the reactor's own turn is not the owner: `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`
drives 256 settled `poll_kernel` turns against a live, acknowledged instance of `TestRuntimeApps` and
weighs the real heap — **0 B retained per turn**, every phase at delta 0.

What the two measurements differ in is the APP. The reactor turn is clean; the procedural app's
per-turn work is not, and the harness shows it running on every single turn:
`status=MoreWork effects=0 patches=0` for 512 turns straight — the actor claims more work forever and
produces nothing, which is the same `MoreWork` spin `📓️wgpu-ui-turn-2026-09-10.md` and the process-pool
lane record. `📓️guest-memory-2026-09-10.md` §4 already measured that same app natively at
**4 082 B per settled tick+render cycle** and dismissed it by extrapolating to 600 cycles. At the
`MoreWork` cadence the browser actually runs, 512 MiB / 4 437 B ≈ **118 000 turns** — reached in the
~60 s boot #9b survived. The 189 328-byte allocation was never the leak; it was simply the largest
single allocation in the guest, and therefore the first one to fail.

## 7. Tests

| law | file | result |
|---|---|---|
| `the_reactor_turn_future_fits_the_one_page_ceiling` | `⚛️reactor/🔄️turn/🧪️tests/📏️future-size/🦀️.rs` | **passes** — 53 888 B against a 65 536 B ceiling (was 252 272 B) |
| `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` | same file | **passes** — 0 B retained per settled turn over 256 turns |
| `the_poll_export_keeps_guest_linear_memory_flat_across_turns` | `🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs` | **RED, deliberately** — 4 437.3 B/turn on the real component; this is §6's open defect stated as a law |

The wasm law is opt-in: with `SEMIO_POLL_TURN_LEAK_COMPONENT` unset it returns immediately (the same
shape `🖨️describe`'s `SEMIO_OWNED_DIFFERENTIAL_FIXTURES` law uses), because a
`wasm32-wasip2 --profile wasm-dev` artifact is not a `cargo test` prerequisite. `SEMIO_POLL_TURN_LEAK_TURNS`,
`…_APP`, `…_FRAMES` and `…_INSTANCES` are the knobs §4 and §6 varied. It is red on purpose: the
defect it names is real, measured, and unowned by this lane — see §8.

Three `launch.json` entries were added through the seed
(`.vscode/🧩️launch.seed.jsonc`, rendered by `@semio-tech/plugin-registry:generate`):
`⚖️gate🔌️plugin📏️turn-future-size`, `⚖️gate🔌️plugin🧮️turn-retention`,
`⚖️gate🖥️host📈️poll-turn-memory`.

### Gate tails

| gate | result |
|---|---|
| `cargo test -p semio-framework-plugin --lib the_reactor_turn_future_fits` | **1 passed, 0 failed** (`53888 B`) |
| `cargo test -p semio-framework-plugin --lib a_settled_reactor_turn_retains_` | **1 passed, 0 failed** (0 B/turn) |
| `cargo check -p semio-s-plugin-procedural --keep-going` (native) | `Finished dev profile in 4m 53s`, 0 errors |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going` | `Finished wasm-dev profile in 3m 39s`, 0 errors |
| `bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev` | `Successfully ran … and 38 tasks`, 22 m 55 s, "11 completed components (changed)" |

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib` does
NOT compile right now: `👁️viewer/🧪️tests/🔬️unit/🦀️.rs:143` calls `app.window_measures(..)` as a method
on `Generation3dViewerFixture` while `ArtifactViewer::window_measures` is an associated function
(E0599 + two E0277 follow-ons). That is a peer's in-flight `window_measures` refactor, in a file this
lane never touched, and it is why §6's native cross-check against the REAL app could not be re-run
here — `📓️guest-memory-2026-09-10.md` §4's own 4 082 B/cycle reading stands in for it.

## 8. What is still open, and its byte accounting

### 8.1 The 4 437 B/turn is real, unowned by this lane, and now measurable in 40 seconds

Everything in §4 and §6 is reproducible from one command (the component compiles once and Wasmtime
caches it, so re-runs are ~40 s):

```
CARGO_TARGET_DIR=<private> RUSTC_WRAPPER="" \
SEMIO_POLL_TURN_LEAK_COMPONENT=<repo>/target/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm \
SEMIO_POLL_TURN_LEAK_TURNS=512 \
cargo test -p semio-framework-plugin-host --lib the_poll_export_keeps_guest_linear_memory_flat_across_turns -- --nocapture --test-threads=1
```

What the next lane inherits, already narrowed:

| fact | evidence |
|---|---|
| it is per TURN, not per instance | 1 instance 4 437.3 B/turn, 3 instances 4 949.3 B/turn |
| it needs at least one live instance | 64 turns with none: flat to one page |
| it is not the reactor's own turn | `TestRuntimeApps`, 256 acknowledged settled turns, **0 B** retained, every phase delta 0 |
| it is not the render path | `max_frames = 0` changes nothing |
| it is not the task box, nor fragmentation behind it | the box shrank 189 328 → 40 400 B and the curve did not move one byte |
| it is not the WIT lowering of the turn result | `effects=0 patches=0 presence=0 receipt=false`, all 512 turns |
| the actor never settles | `status=MoreWork` on every one of 512 turns, producing nothing |

The remaining suspect is the procedural app's own per-turn work, which the `MoreWork` spin runs on
every turn — the same object `📓️guest-memory-2026-09-10.md` §4 measured natively at 4 082 B per
settled tick+render cycle and set aside by extrapolating to 600 cycles instead of the ~118 000 turns
the `MoreWork` cadence actually reaches. Two lanes already own that spin
(`📓️wgpu-ui-turn-2026-09-10.md`, the process-pool pump) and one owns the app
(`📓️flow-eval-tick-address-2026-09-10.md`). `trace_turn_phase_retention` is in place for whoever
takes it: `SEMIO_RUNTIME_DIAGNOSTICS=1` prints the retained-heap delta of every turn phase in any
binary that installs `semio_framework_trace::HeapWitness`.

### 8.2 The turn future's last 44 KB is seven command pages

To go from 53 888 B to the few-KB shape, `semio_framework::kernel::FixedCommandPage` (`[u8; 4096]`)
must stop crossing the turn frames by value. The exact remaining accounting, from
`-Zprint-type-sizes` on `poll_kernel_turn`'s largest variant (`Suspend18`, 44 783 B):

| slot | bytes |
|---|---|
| `upvar .command_page` (`Option<(CommandPageCursor, FixedCommandPage)>`) | 4 168 |
| `local .command_page` | 4 168 |
| `local .retained` (`Option<CommandIngressOwner>`) | 4 184 |
| `..coroutine_field85` (`Option<CommandIngressOwner>`) | 4 184 |
| `..coroutine_field87` (`Result<(), (Fault, FixedCommandPage)>`) | 4 208 |
| `local .page` (`FixedCommandPage`) | 4 098 |
| slot padding around them | 18 143 |

Three changes cover it: box the page payloads of `CommandIngressOwner::ReservedPresence` /
`PendingPresencePage` (reactor-local); return the page-return error through a box; and change the
`command_page` parameter of `poll_kernel`/`poll_kernel_output`/`poll_kernel_turn` to
`Option<Box<(CommandPageCursor, FixedCommandPage)>>` — a 4 KB page has no business being memcpy'd
through four async frames — which touches `wit_bridge::poll`, the owned-ABI `PollInput` bridge in
`🔌️plugin/🦀️.rs:122`, and `plugin_exports!` at `🔌️plugin/🦀️.rs:32767`.

That work was deliberately NOT landed here: §6 proves shrinking the future further has **zero**
measured effect on the defect, the change spans files two other lanes are editing, and validating it
costs a 23-minute wasm rebuild it cannot be shipped without. The one-page ceiling law locks in the
4.7× that IS measured and blocks the two shapes that produced the quarter megabyte.

## 9. Files

| file | change |
|---|---|
| `⚛️reactor/🔄️turn/🦀️.rs` | `with_turn_execution` takes `Pin<&mut …>`; `poll_kernel_output` pins; `plugin_exchange_boxed`; `trace_turn_phase_retention` + its `TURN_PHASE_RETENTION` cell and six phase probes |
| `⚛️reactor/🔄️turn/🧪️tests/📏️future-size/🦀️.rs` | new — the one-page future ceiling law and the settled-turn retention law |
| `⚛️reactor/🔄️turn/🧪️tests/⏱️execution/🦀️.rs` | pins the driven future for the new `with_turn_execution` signature |
| `🔌️plugin/🦀️.rs` | `#[cfg(test)] #[global_allocator] PLUGIN_HEAP_WITNESS`; includes the new test module |
| `🖥️host/🦀️.rs` | `BudgetLimiter::peak_memory_bytes` + its `memory_growing` witness; `GuestInstance::guest_linear_memory_bytes()`; includes the new test module |
| `🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs` | new — the wasm turn-growth law and harness |
| `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` | three gate entries |
