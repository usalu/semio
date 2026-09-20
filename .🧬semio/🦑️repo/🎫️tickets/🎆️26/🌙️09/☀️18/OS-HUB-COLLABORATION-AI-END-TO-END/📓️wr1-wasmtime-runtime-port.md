# 📓️ WR1 — porting the headless MCP plugin host from `OwnedRuntime` to `WasmtimeRuntime`

Slice: outcome 4's headless lane. The semio MCP gateway (`semio-framework-os-mcp`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`) is the only caller in the repo that
opens plugin instances through `OwnedRuntime`, the repo-owned **interpreter**. R2 §10.6 measured the
consequence: `🗒️note` ~25 s calm / >120 s under load, `🖍️draw` >240 s — against the MCP client's own
240 000 ms per-call budget (`🌉️mcp/🟦️.ts:258`). The permanent `client-e2e` gate is therefore pinned at
**12/16** with `InstanceOpen … budget.exceeded`.

Spec: R2 §13 (the seam design), A1 §5.1 (where the port was first named), A2 (the host laws and the
`ensure_instance` pump this replaces).

Status: **landed and measured** (2026-09-20 12:04). The headless lane runs on `WasmtimeRuntime`.
`🖍️draw`'s instance open went from **>240 s — past the MCP client's whole per-call budget — to 34 ms**,
and the `client-e2e` gate from *dying* on `tools/call did not answer within 240000ms` to **finishing in
9.7 s** at 12/16. The four red rows are now red for reasons no runtime change can touch (§5.4).

## 1. Inherited state

No predecessor: this slice is new, opened out of R2 §11 gap 2. `git status` shows no prior WR1 edit
and `🗑️generated/` carried no `wr1-*` capture. What was inherited is the measurement, not code:

| reading | source |
| --- | --- |
| `client-e2e` 12/16, three plugin rows red on `InstanceOpen … budget.exceeded` | R2 §10.3, §11 (`🗑️generated/r2-s5c-client-e2e.txt`) |
| `note` open ~25 s calm, >120 s under fleet load | A2 §5.2 / R2 §10.7 |
| `draw` open >240 s — past the MCP client's own per-call budget (`🌉️mcp/🟦️.ts:258`) | R2 §10.6 (`r2-s5d-client-e2e.txt`) |
| the os dev host boots ~20 of the same components in seconds through `WasmtimeRuntime` | A1 §5.1, A2 §3.1 |
| `wasmtime_runtime_instance_open_settles_against_a_real_plugin_component` is green | A2 §5.1 |

## 2. The seam as found

`PluginArtifactChannel` held `Arc<OwnedRuntime>` and drove four **sync inherent** methods:
`compile_component`, `instantiate_actor`, `turn_in_flight`, `execute_actor_turn` (plus `drop_actor`
in `attempt_plugin_activation`). `WasmtimeRuntime` implements the **async trait** `GuestRuntime`
(`🔌️plugin/🖥️host/🦀️.rs:679`): `compile`, `instantiate(compiled, actor, caps, budget)`,
`execute_turn`, `drop_instance` — and has no `turn_in_flight`, because a compiled turn is atomic.
Three structures in the gateway existed ONLY to serve mid-flight interpreter yields:

1. `opening: HashMap<u32, GuestInstance>` + `undelivered: HashMap<u32, Vec<Event>>` — the events an
   open still owed a guest that had refused them while a turn was in flight;
2. the `turn_in_flight` guard in `exchange_one_slice`, which suppressed the next retained command
   page so the cursor would not advance past a turn that had not finished;
3. `exchange_one_real`'s resume loop, which re-entered **on a fault's code string** (`budget.exceeded`),
   because every "call me again" was minted as a budget fault.

(1) and (2) are meaningless under a compiled runtime. (3) is not — a command of N pages still needs
N+1 turns — but it was expressing a real control-flow signal as a fault, which is why R2 could not
tell a paging step from a wedged guest in the transcript.

## 3. The port

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`.

| # | what | where |
| --- | --- | --- |
| 1 | `shared_plugin_runtime()` — ONE `WasmtimeRuntime` per process behind a `OnceLock`, held as `Arc<GuestRuntimes>` | new fn beside the budgets |
| 2 | `PluginArtifactChannel.runtime: Arc<GuestRuntimes>`; `opening` and `undelivered` deleted | struct |
| 3 | `new()` compiles through `GuestRuntime::compile` (on-disk `.cwasm` cache, keyed by the component's own content hash) | `PluginArtifactChannel::new` |
| 4 | `ensure_instance` rewritten: `instantiate(…, &caps, &budget)` → `InstanceOpen` → ack the lifecycle receipt → stop at `Idle` with nothing owed. Local `owed` vector, no queue, no in-flight guard | `ensure_instance` |
| 5 | `exchange_one_slice` → `exchange_one_turn`, returning `CommandTurn::{Settled, MoreWork}` instead of encoding "call me again" as a `budget.exceeded` fault | command lane |
| 6 | a turn cut by its own ceiling now **discards the instance** and answers a terminal `budget.exceeded` — under the JIT the component is mid-call and every later call would answer `cannot enter component instance` | `ensure_instance`, `exchange_one_turn`, new `discard_instance` |
| 7 | the inference lane shares the SAME runtime (`Arc::clone(&self.runtime)`) instead of building a second `OwnedRuntime` — mandatory, not tidiness: a `wasmtime::Component` may only be instantiated by the `Engine` that compiled it | `ensure_inference_route` |
| 8 | `activate_plugin_instance(runtime: &GuestRuntimes, …)` and `attempt_plugin_activation` ported; `drop_actor` → `drop_instance` | activation region |
| 9 | budgets re-stated as **ceilings**, not slices: open 60 s/turn under a 150 s wall, command 5 s/turn under the unchanged 60 s wall, fuel disarmed on all three (a fuel yield is as unresumable as an epoch cut under the JIT, so one readable time ceiling replaces two) | `headless_*_budget` |

What is kept, per R2 §13.6: the lifecycle **acknowledgement** (A2's real fix, runtime-independent);
the guest's own retryable lifecycle-deadline verdict; `pending_exchanges`/`pending_command_closes`/
`rejected_command_builds`, which are host-side command bookkeeping and never touch the runtime.

`semio_framework_async::block_on` is the async bridge, reused from the inference lane
(`🏠️workspace/🦀️.rs` already blocked there) rather than making `ArtifactChannel` async — that trait is
sync, is `Send`, and now has a second production implementor in flight (LB1's `ShellArtifactChannel`),
so widening it was never on the table.

## 4. Other `OwnedRuntime` users (untouched)

Grepped repo-wide; the interpreter stays exactly where it was for everyone else:

| file | use |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1500` | `GuestRuntimes::Owned` for the `run` node host |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:406` | descriptor emission |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6465,9181` | two `#[cfg(test)]` modules |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` | the runtime itself + its own laws |

After this port the MCP gateway is no longer among them, which was A1 §5.1's original observation
("the MCP gateway is the only caller that opens plugin instances through `OwnedRuntime`").

**Dependency truth gate:** not needed — no manifest changed. `semio-framework-plugin-host` was
already `semio-framework-os-mcp`'s `cfg(not(target_arch = "wasm32"))` dependency, and `WasmtimeRuntime`/
`SharedEngineConfig`/`GuestRuntimes` are public types of that same crate (its Cargo.toml comment
already named `WasmtimeRuntime` as the reason the dependency exists). No new external crate enters.

## 5. Measured — before / after

Every number below was produced in this session and captured under `🗑️generated/wr1-*`.

### 5.1 The open, and the two faults the port exposed underneath it

| reading | before (interpreter) | after (compiled) |
| --- | --- | --- |
| `🖍️draw` `Event::InstanceOpen` to `Idle` | >240 s — never observed to settle (R2 §10.6) | **34 ms** (`wr1-host-laws-draw.txt`) |
| `🖍️draw` component compile | n/a (the interpreter compiles nothing) | **4.05 s**, once per build, then served from `~/.semio/cache/wasmtime/<engine>/<hash>.cwasm` |
| the gate's first plugin tool call | `budget.exceeded` after 120 s, then (R2 §10.6, 420 s experiment) a dead tool call | **5.4 s**, answered (`wr1-open-probe.txt`) |
| the whole `client-e2e` gate | ~9 min for two journeys (R2 §10.4), latterly aborting at 240 s | **9.7 s** (`wr1-client-e2e.txt`) |

A2's instance-open laws, extended with a `🖍️draw` row (R2 §13.8), all green on the current tree —
`wr1-host-laws-draw.txt`, `cargo test -p semio-framework-plugin-host --lib owned_instance_open`:

```
owned_runtime_instance_open_settles_against_a_real_plugin_component ... ok
wasmtime_runtime_instance_open_settles_against_a_real_plugin_component ... ok
wasmtime_runtime_instance_open_settles_against_the_draw_component ... ok
    draw: bytes=62432620 compile=4.054688625s open=34.191042ms
a_trapped_owned_instance_refuses_every_later_turn ... ok
a_mid_flight_owned_turn_refuses_new_events_instead_of_dropping_them ... ok
test result: ok. 5 passed; 0 failed; … finished in 173.20s
```

### 5.2 The package hash was a 64-char hex STRING, and the JIT made that fatal

`framework_hash::hash_bytes` returns a `String`; both gateway call sites did
`hash_bytes(&bytes).into_bytes().try_into().unwrap_or([0u8; 32])`, and a 64-byte `Vec` never converts
to `[u8; 32]` — so **every plugin in the process used an all-zero package hash**. Under the
interpreter that was inert bookkeeping. `WasmtimeRuntime::compile` keys its on-disk `.cwasm` cache on
exactly that hash, so the placeholder made every plugin collide on one cache entry and replay
whichever component compiled first. Measured, not inferred: a single **130 MB
`0000000000…0000.cwasm`** in the cache root, rewritten by each run (2026-09-20 09:30).

The same defect, one line, is in `🏃️run/🦀️.rs:1730` — `WasmtimeNodeHost`'s own component loader, which
writes into the same shared cache directory. Fixed there too (the `run` **lib** checks green; that
crate's `🏗️bootstrap` bin is red on a peer's unrelated `store::Media` serde break).

### 5.3 The real reason the gate died: dropping a component, not opening one

The gate's first plugin call produced NO answer at all, not even a slow one, so it was `sample`d
while stuck (`wr1-open-probe.mjs`, the row driven with no client budget). The main thread was 100 %
inside:

```
Store<ActorHostState> as Drop → ComponentInstance → Component → EngineCode → CodeMemory
  → UnwindRegistration::drop      (__deregister_frame over a 136 MB code image)
```

Releasing the **last** `Arc<Component>` deregisters every unwind frame in the compiled image, and for
a `wasm-dev` plugin that is minutes of single-threaded work. The gateway opened a channel, served
the call, and then paid that teardown inside the same tool call — the open was 34 ms and the
*cleanup* ate the client's budget. Fixed by `shared_compiled_component`: one `CompiledHandle` per
content hash for the life of the process, so a channel's drop releases its `Store` and stops at an
`Arc` decrement. That also means each component compiles at most once per process.

### 5.4 The gate — `🗑️generated/wr1-client-e2e.txt`, **12/16 in 9.7 s**

The count is R2's, the shape is not: **no row times out any more, and every plugin row fails one
layer further along.**

| red row | R2 (interpreter) | WR1 (compiled) |
| --- | --- | --- |
| capability catalog health | 30 diagnostics | **58** — a peer's descriptor regression (`animate` missing `windowKindId`; `dag`/`demonstrator`/`forms`/`trinity`/`vcs`/… missing `artifactSchema`), nothing to do with this slice, and worse than A1 left it |
| `artifact_create` (plugin kind) | `InstanceOpen … budget.exceeded`, then a dead 240 s tool call | `the guest acknowledged command seq 1 and then went idle without publishing a response for it` |
| `artifact_export` | cascade | cascade (`PLUGIN_UNAVAILABLE`: the create never landed) |
| `action_prepare` | `InstanceOpen … budget.exceeded` | same new fault as `artifact_create` |

**The new blocker, root-caused as far as this slice's wire reaches.** The guest takes the retained
command pages, drives its ingress to `CommandBatchProgress::Complete` (so the cursor/owner machinery
A2 and R2 fixed is working end to end), and then goes `Idle` with `next_wake == None` having published
**no `Effect::Respond` at all**. The obvious suspect was a request-id mismatch — every
`store::AppCommand` this gateway builds carries `seq: 0` while the envelope carries `seq: 1..` — so
`await_response` was instrumented to report any response under *another* id. It reported none:
**the guest answers no request whatsoever**, so the mismatch hypothesis is measured and dead. The
remaining candidate is that `ReadDocument` against an app with no loaded document is simply a no-op
in the guest, which is guest/protocol work (R2's and M5br's wire), not runtime work.

This is the first time that fault has been *reachable*: A1 stopped at a guest trap, A2 at
`plugin.command-cursor-mismatch`, R2 at the interpreter's open. The mutation chain is still not
green, and the reason is now one named, fast, reproducible fault instead of a timeout.

## 6. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | the port (§3): `shared_plugin_runtime`, `shared_compiled_component`, `ensure_instance` rewritten, `exchange_one_slice` → `exchange_one_turn` + `CommandTurn`, `await_response`, `discard_instance`, budgets re-stated as ceilings, the package-hash fix, `activate_plugin_instance`/`attempt_plugin_activation` ported |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` | **2 lines only** — `channel_binding: None` in the two `GatewayRuntime` initializers a peer (LB1) had left missing, which made the whole crate refuse to compile for everyone. The rest of that file's diff is theirs. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs` | §5.2's package-hash fix at `:1730` (same defect, same shared cache) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs` | A2's laws extended: `package_ref`/`open_to_settle` parameterised, new `wasmtime_runtime_instance_open_settles_against_the_draw_component` with the compile/open split printed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️app-router/🦀️.rs` | **1 line** — `actions: Vec::new()` in a fixture, after a peer added a mandatory `AppDefinition.actions`; the whole `semio-framework-plugin-host` test binary would not build without it |

Probe: `🐍️wr1-open-probe.mjs` (drives the one hanging gate row with no client budget and prints the
server pid, so the stuck process can be `sample`d — the gate's own client kills the evidence at
240 s). Captures: `wr1-build-mcp.txt`, `wr1-host-laws.txt`, `wr1-host-laws-draw.txt`,
`wr1-open-probe.txt`, `wr1-client-e2e.txt`.

Staged binary: `🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`, 161 246 208 B, built through
`bun ./📜️script.ts build` (the underlying verb, not the nx target — R2 §10.3: nx caches `build` on
inputs that exclude the framework crates it links), which stages through a fresh directory + rename
and re-signs; `codesign -v` clean.

## 7. Honest gaps

1. **16/16 was not reached; the gate is 12/16.** Three of the four red rows are §5.4's single new
   fault (the guest acknowledges a command and answers no request), which is on the retained
   command/response wire, not the runtime. The fourth is a peer's descriptor regression that got
   *worse* during this session (30 → 58 diagnostics) and is A1 §3's item.
2. **`headless_command_budget`'s 5 s ceiling is still not exercised.** No command has yet reached it —
   every one now settles its ingress in milliseconds and dies on the missing response instead. R2's
   gap 1 therefore stands, with a different reason.
3. **`await_response`'s wall is never reached either**, because the `Idle`-with-no-wake stop fires
   first. The wall is the belt to that braces; it has not been observed firing.
4. **The compiled-component cache is unbounded.** One `CompiledHandle` per distinct component the
   process ever opens, held to process exit — ~136 MB of code image each. That is the right shape for
   a gateway serving one agent (and the alternative is §5.3's minutes-long teardown), but a
   long-running gateway that opens all 20 plugins will hold ~2 GB of code images. Not measured, not
   bounded, and worth an eviction policy the day someone runs one that long.
5. **Only `🖍️draw` and `🗒️note` are proven.** The other 18 staged components are not opened by any law;
   A2's gap 5 (13 components still linked with a 1 MiB stack) is unchanged and is now less relevant
   to this lane, since the compiled runtime uses the host stack, but it is still unmeasured.
6. **The inference lane was re-pointed at the shared runtime but not re-measured.** It had to move —
   a `wasmtime::Component` cannot be instantiated by a second `Engine` — and it type-checks, but no
   `inference_run` was driven through it in this session.
7. **`🏃️run`'s package-hash fix is compile-verified only** (`cargo check -p semio-framework-os-run
   --lib`). That crate's binary does not build for an unrelated peer reason, so no `run` host was
   started to confirm the cache now keys per plugin there.

> 🤝 **LB1 (2026-09-20 ~12:4x), `🏠️workspace/🦀️.rs`:** I changed exactly two functions in your file —
> `create_plugin_artifact` and `export_artifact_media` — so each calls the new
> `open_session_artifact_channel` instead of `open_artifact_channel` (plus a `shell_route: OnceLock`
> field, its `bind_shell_route` setter, and an `ArtifactChannels::ShellDirect` variant). Nothing in
> `OwnedRuntime`/`ensure_instance`/`PluginArtifactChannel` was touched; `open_artifact_channel`
> itself is unchanged and still the headless path. `cargo check -p semio-framework-os-mcp` rc=0
> after the edit. See `📓️lb1-live-bridge-action-routing.md` §10.1.
