# 😴️ What generation3d does when nothing happens — the idle turn

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: idle-turn behaviour of generation3d/2d and the
per-turn retention they cause (`⚛️reactor/🔄️turn`, `🔌️plugin`'s typed-operation continuation,
`🖥️host/🧪️tests/🔬️poll-turn-memory`). Written 2026-09-10, appended as the lane ran.

**restage required: yes** — the fixes are in `semio-framework-plugin` (`⚛️reactor/🔄️turn`, the
plugin runtime's typed-operation scan), which every staged plugin component links.

## 1. The two defects this lane owns

`📓️poll-task-leak-2026-09-10.md` §8 handed over a 40-second reproduction with two symptoms measured
against the real `wasm32-wasip2` procedural component:

| symptom | §8's reading |
|---|---|
| the actor never settles | `status=MoreWork` on every one of 512 turns, `effects=0 patches=0` |
| the guest grows per turn | **4 437 B/turn**, 1 instance; 4 949 B/turn, 3 instances |

Both reproduced. Both had a different owner than §8 supposed, and the second is NOT what §8 (or
`📓️guest-memory-2026-09-10.md` §4) had in its sights.

## 2. Instruments added

| instrument | where | what it answers |
|---|---|---|
| `reactor::TurnMoreWorkSources` + `reactor::last_turn_more_work_sources()` | `⚛️reactor/🔄️turn/🦀️.rs` | WHICH of the nine sources armed `MoreWork` on the last turn — one `Cell` store per turn, always recorded |
| `plugin_runtime::TypedOperationScan` | `🔌️plugin/🦀️.rs` | splits "an instance holds runnable typed work" from "the instance lock was busy", which used to be one `bool` |
| `😴️idle-turns` `[[test]]` | `✏️s/🔌️plugins/🌀️procedural/🧪️tests/😴️idle-turns/🦀️.rs` | the native law: boot the REAL generation3d/2d editor through the guest reactor, load the bundled example, render its preview, then 512 idle turns under a `HeapWitness` global allocator |
| `SEMIO_POLL_TURN_LEAK_EXPORT=checkpoint` | `🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs` | drives a DIFFERENT async-lifted export the same number of times, separating the shared component-async machinery from `reactor.poll` itself |
| `guest-heap-witness` feature | `✏️s/🔌️plugins/🌀️procedural` | installs `HeapWitness` as the COMPONENT's `#[global_allocator]` and arms `set_runtime_diagnostics`, so a wasm build can attribute retention to a turn phase |
| `trace_guest_line` | `⚛️reactor/🔄️turn/🦀️.rs` | routes the reactor's diagnostic lines through the actor world's `log` import on wasm — a `wasm32-wasip2` guest's `stderr` goes to whatever `WasiCtx` the host built, which in the Wasmtime harness and in the browser is nothing |

## 3. Defect 1 — `MoreWork` on an idle turn

### 3.1 The 512-turn `MoreWork` streak was the harness, not the app

`🔬️poll-turn-memory`'s `drive_idle_poll_turns` opened an instance and **never acknowledged the
`Captured` receipt**. An unacknowledged open is retained lifecycle state the guest re-offers until
the ACK arrives, so `NativeLifecycleRegistry::has_work()` stays true forever and the turn folds it
into `MoreWork` — correctly. Reproduced natively, exactly, in the new law
`an_unacknowledged_open_retains_nothing_per_turn`:

```
[DEBUG] unacknowledged open, 512 turns: sources {"lifecycle": 512}
```

With the ACK fed back — what a real host does, and what the harness now does — the same component
answers `Idle`:

```
[DEBUG] turn 0 result: status=MoreWork … receipt=true
[DEBUG] turn 1 result: status=MoreWork … receipt=false
[DEBUG] turn 2 result: status=Idle …
[DEBUG] 9 of 512 turns answered MoreWork
```

So the "actor never settles" reading was an artefact of the instrument. What is left is the 9.

### 3.2 The real source: a busy instance lock reported as runnable work

`🔌️plugin/🦀️.rs:31599 pending_typed_operation_instance` scanned every live instance and returned
one `bool` that meant EITHER "this instance has a runnable typed operation" OR "`try_lock` said
`WouldBlock`". `⚛️reactor/🔄️turn/🦀️.rs`'s fold turned both into `TurnStatus::MoreWork`.

The native law with the sources split shows what that costs an idle app:

| app | idle turns | `MoreWork` | source |
|---|---|---|---|
| generation3d | 512 | 7 – 12 (run to run) | `typed_operation_contended`, never `typed_operation` |
| generation2d | 512 | 2 – 5 | `typed_operation_contended`, never `typed_operation` |

Not one of those turns had a runnable typed operation, produced an effect, a patch or a presence
update. Every one cost the host a turn round trip.

A busy lock is not an answer of "there is work" — it is "another owner is mid-step and I could not
look". The guest is single-threaded, so on `wasm32-wasip2` it can only ever be a reader inside the
very same turn; natively it is a worker-pool thread touching the instance. And every owner that can
hold that lock is itself a reactor executor task (`executor_pending`), a task resume (`resumes`), an
ingress command (`command_ingress`) or a lifecycle step (`lifecycle`) — each of which already arms
the turn on its own account. `executor_pending` was FALSE on every one of those hot turns, i.e.
nothing was registered as in flight.

**Fix**: `typed_operation_scan.contended` is no longer folded into `more_work`
(`⚛️reactor/🔄️turn/🦀️.rs`, the `let more_work = …` fold). It survives as an OBSERVATION on
`TurnMoreWorkSources::typed_operation_contended`, deliberately outside `any()`/`names()`, so a law
can still see it without the reactor spinning on it.

## 4. Defect 2 — the per-turn guest growth

### 4.1 It is per `poll` CALL, not per instance and not per app

§8 recorded "it needs at least one live instance; 64 turns with none: flat to one page". That was a
slicing artefact of a 64-turn run. At 512 turns, with `SEMIO_POLL_TURN_LEAK_APP=` (no instance ever
opened, every turn `Idle`, zero app code reached):

| run | instance | `MoreWork` turns | growth |
|---|---|---|---|
| open, never acked | 1 | 512 / 512 | 4 608 B/turn |
| open + ACK | 1 | 9 / 512 | **4 437 B/turn** |
| never opened | 0 | **0 / 512** | **4 437 B/turn** |

Identical to four significant digits with and without an instance, and with every turn answering
`Idle`. The growth is a property of the `poll` call itself.

### 4.2 It is `reactor.poll`, not the component-async export machinery

512 calls of the `checkpoint` export — the same async-lifted WIT shape, the same `start_task` task
box, the same `ComponentPersistentLocal` runtime access:

```
[DEBUG] drove 512 checkpoint exports instead of poll turns
[DEBUG] guest linear memory: instantiate=12451840 B, turn 128=18546688 B, turn 511=18546688 B, 0.0 B/turn
```

**0 B/turn.** So `wit_bindgen`'s `start_task`/`FutureState` box is released (its `callback` drops it
on `CallbackCode::Exit`), and the leak is inside `reactor::poll`'s own body.

### 4.3 The shape of the growth names its size

`memory.size` readings, sampled every 32 turns, step by 131 072 B (two 64 KiB `dlmalloc`
granularity units) per 32 turns, with an occasional 196 608 B step:

```
turn   0: 19333120 B      turn 256: … 
turn  32: +196608         turn 288: +131072
turn  64: +131072         turn 320: +196608
turn  96: +131072         turn 352: +131072
turn 128: +131072         …
```

131 072 / 32 = **4 096 B per turn, exactly one wasm page's worth of net demand** — i.e. ONE ~4 KiB
block retained per `poll`, not fragmentation noise and not the 40 400 B task box.

### 4.4 Natively the same turn retains nothing

The same `poll_kernel`, driven 512 times under `HeapWitness` in the new native law:

| case | retained over 512 turns | per turn |
|---|---|---|
| generation3d, example loaded, preview rendered | −72 B | 0 B |
| generation2d | +54 401 B | 106 B (one `Vec` growth, not linear) |
| open never acked | −32 391 B | −63 B |

A 4 096 B/turn retention would be 2.1 MB over the same window; it is not there. The owner is
therefore `cfg`-gated code that a native build never compiles — the wasm-gate is the discriminator.

