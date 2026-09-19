# 📓️ A2 — the `Event::InstanceOpen` guest trap in the MCP gateway's plugin host

Slice A2 of `OS-HUB-COLLABORATION-AI-END-TO-END`. Owns the single root blocker named in
`📓️a1-mcp-end-to-end.md` §5.1: every plugin guest traps with
`memory write is out of bounds: start=4294409068` when the os MCP gateway opens a plugin instance.
Also owns the `BUDGET_EXCEEDED`-on-first-attempt defect and M3's single remaining red
(`📓️m3-mcp-tests-and-wgpu-agent-panel.md` §3.4, `plugin_artifact_channel_mutation_verbs_…`).

All commands run from `/Users/ueli/Documents/semio`. Everything below was executed; nothing is inferred.

**Bottom line.** A1's framing was wrong in the most useful way: the trap is **not** MCP-specific,
**not** `OwnedRuntime`-specific and **not** recursion. It is a **shadow-stack underflow at call
depth 26** in a component wasm-ld linked with its **default 1 MiB** stack instead of the repo's
declared **8 MiB**, triggered by `UiPatchApplyArena::default()` spending ~654 KiB of that stack to
zero-initialise a 93.5 KiB static. `WasmtimeRuntime` — the runtime the os dev host uses — traps at
the **same guest function on the same component** (§3), which is what ruled the interpreter out.
The 8 MiB flag lived in a TypeScript build plan, not in `.cargo/config.toml`, so every plugin
`.wasm` produced by a plain `cargo build --target wasm32-wasip2` landed in the **same shared target
directory** the MCP gateway reads, with a 1 MiB stack.

**The trap is fixed and proven gone** — at the host (4/4 new laws), in the os-mcp crate, and in the
live e2e gate, where `draw` now opens for real. The chain is **not** green: two further, genuinely
separate defects sit behind it (§6), and the second one is named here with its exact evidence rather
than half-fixed under a fleet.

---

## 1. Inherited state

`git status --short` at start: `🌉️mcp/{🏠️workspace,🔀️dispatch,🗿️artifact,🟦️.ts}` carried A1's and M4's
live edits; `🔌️plugin/{🖥️host,🦀️.rs}` carried peer edits. `🗑️generated` was empty — no A2 captures to
inherit. Components read from `.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/`
(20 plugin `.wasm`); `semio_s_plugin_note.wasm` is the probe, `semio_s_plugin_draw.wasm` the one the
e2e chain drives.

This slice was cut by the account session limit at ~08:15 and resumed at 11:00. All edits survived;
every marker was re-verified against the working tree before continuing.

## 2. Reproduction (permanent test)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs`, registered at
`🔌️plugin/🖥️host/🦀️.rs:1597`. Four laws against the real staged component:

```
cargo test -p semio-framework-plugin-host --lib owned_instance_open -- --test-threads=1
```

Both runtime laws were red **with `fuel: u64::MAX` and a 120 s deadline** — so the trap was never a
budget, a slice yield or instance reuse:

| runtime | before the fix |
| --- | --- |
| `OwnedRuntime` | `Trapped("wasm trap: memory write is out of bounds: start=4294409068 length=4 end=4294409072 bound=13172736")` |
| `WasmtimeRuntime` | `Trapped("memory fault at wasm address 0xfffb9dac in linear memory of size 0xc90000 … out of bounds memory access")` |

## 3. Bisect

| hypothesis (A1 §5.1 / the brief) | verdict | evidence |
| --- | --- | --- |
| runaway recursion in the guest's open path | **no** | the trap is at **26 frames** |
| `OwnedRuntime` interpreter defect | **no** | `WasmtimeRuntime` traps on the same component in the same guest function |
| empty `config`/`assets` envelope | **no** | the trapping frame never reads `config`; `🔬️poll-turn-memory`'s wasmtime law already passes with the same empty envelope |
| reuse of a trapped instance | **real, but secondary** | explains only A1's "−112 per attempt" drift (§5.3), not the first trap |
| 8 ms slice / budget | **no** | reproduced with unbounded fuel and a 120 s deadline |

**The guest stack tape** (temporary `SEMIO_INTERPRETER_TRAP_TRACE` instrumentation, since removed and
replaced by the permanent diagnosis in §5.4):

```
sp 1048576 -> 991408  (delta 57168)  depth=1   semio_owned_poll_v1
...
sp 376944  -> 96384   (delta 280560) depth=19
sp 96352   -> 2832    (delta 93520)  depth=22  UI_PATCH_APPLY_ARENA::{closure#0}
sp 2832    -> -90672  (delta 93504)  depth=23  <UiPatchApplyArena as Default>::default
sp -90672  -> -184176 (delta 93504)  depth=24  core::array::from_fn
sp -184176 -> -464704 (delta 280528) depth=25  core::array::try_from_fn
sp -464704 -> -558240 (delta 93536)  depth=26  core::array::try_from_fn_erased   <-- trap
trap depth=26 globals[0] (__stack_pointer) = -558240  memory_bytes=13172736
```

Two facts fall straight out:

1. **`__stack_pointer` starts at `1048576`** — wasm-ld's default 1 MiB, **not** the 8 MiB the repo
   declares for plugin components. The layout is `--stack-first`, so an overrun addresses below zero
   and surfaces as an ordinary out-of-bounds write at a huge unsigned address, indistinguishable in
   the bare trap text from a wild pointer.
2. **`UiPatchApplyArena::default()` costs ~654 KiB of shadow stack** to build a **93.5 KiB** static
   (`UI_PATCH_APPLY_SLOTS` × `UiPatchApplySlot` = 8 × 11 688 B). At `opt-level = 0`,
   `core::array::from_fn` materialises the array **seven times** — once per frame of the
   `from_fn → try_from_fn → try_from_fn_erased` chain, plus the `LazyLock` closure's return
   temporary. The reactor turn above it had already spent ~390 KiB, so the total lands at ~1.04 MiB:
   just over the 1 MiB tape.

The wasmtime backtrace names the same chain exactly:
`try_from_fn_erased::<…UiPatchApplySlot…>` ← `try_from_fn::<…, 8usize, …>` ← `array::from_fn` ←
`<UiPatchApplyArena as Default>::default` ← `UI_PATCH_APPLY_ARENA::{closure#0}` ← `LazyLock::force` ←
`with_ui_patch_apply_arena::<(), UiPatchApplyArena::retire_one>` ← `close_ui_patch_owner_one` ←
`reactor::turn::retire_until_complete` ← `reactor::turn::poll_kernel_turn`.

### 3.1 Why the os dev host boots the same plugins

Because it does not build them the same way. `🔌️plugin/🏗️build/📋️plan/🟦️.ts:61,85` used to pass the
stack size **per invocation**:

```ts
const PLUGIN_WASM_STACK_BYTES = 8 * 1024 * 1024;
const args = ["rustc", "-p", packageName, "--target", PLUGIN_WASM_TARGET, "--profile", profile,
              "--", "-C", `link-arg=-zstack-size=${PLUGIN_WASM_STACK_BYTES}`];
```

`.cargo/config.toml`'s `[target.wasm32-wasip2]` carried `--max-memory` but **no** `-zstack-size`, so a
plain `cargo build -p semio-s-plugin-note --target wasm32-wasip2 --profile wasm-dev` — the command
every worker and several memory notes recommend for checking a plugin — wrote a 1 MiB-stack component
into **the same shared target directory** the dev host and the MCP gateway both read. The gateway was
not doing anything wrong; it was loading whichever artifact the last builder had left there.
`🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧬️schema/🔣️.json:5` documented the asymmetry as if it were
intentional — `maximumBytes` "set uniformly for `wasm32-wasip2` in `.cargo/config.toml`",
`stackBytes` "passed by the dev plugin build in … `📜️script.ts`". That asymmetry *was* the defect.

## 4. Fixes

| # | file:line | change |
| --- | --- | --- |
| 1 | `.cargo/config.toml:35-45` | `-C link-arg=-zstack-size=8388608` moved into `[target.wasm32-wasip2] rustflags`, beside the `--max-memory` it is carved out of. A component's stack size is part of its ABI, not of one builder's argv. |
| 2 | `🔌️plugin/🏗️build/📋️plan/🟦️.ts:81-89` | `PLUGIN_WASM_STACK_BYTES` and the duplicated link arg deleted from `pluginCargoArgs` (and its dead export); `cargo rustc -- …` is now only appended for `SEMIO_PLUGIN_SYMBOLS=1`. |
| 3 | `🦑️repo/…/📋️native-orchestration/🟦️.ts:33-42` | same duplicate removed from the cached-artifact component build. |
| 4 | `🖱️ui/🧬️contract/🛡️limits/🦀️.rs:493-536` | `UiPatchApplySlot::EMPTY`/`UiPatchApplyArena::EMPTY` consts; `UI_PATCH_APPLY_ARENA` is a `const`-initialised `Mutex`, not a `LazyLock`. Zero frames, zero copies, zero first-touch cost, ~654 KiB of shadow stack no longer spent zeroing a static. |
| 5 | `🔌️plugin/🖥️host/🦀️.rs:1123-1128,1215-1221,1361-1364,1470` | `OwnedInstanceState::poisoned`; `OwnedRuntime::turn_in_flight`; a mid-flight turn **refuses** new events instead of dropping them; a guest trap poisons the instance and every later call refuses, naming re-instantiation. |
| 6 | `🔌️plugin/🖥️host/🦀️.rs:635-641` | `retryable_lifecycle_turn` made `pub` — every host that opens an instance owes the guest that retry, not just the shard loop. |
| 7 | `🌉️mcp/🏠️workspace/🦀️.rs:919-1003` | `ensure_instance` rewritten: pumps slices to settle, **acknowledges the guest's lifecycle receipt**, re-offers undelivered events (`undelivered` map), honours `retryable_lifecycle_turn`, drops a trapped instance, and returns a real `Ok(())` instead of the old "success reported as `budget.exceeded`". |
| 8 | `🌉️mcp/🏠️workspace/🦀️.rs:794-801` | `INSTANCE_OPEN_WALL_BUDGET` (120 s), separate from the 15 s per-command budget: a cold open interprets the guest's whole app assembly (~25 s for `🗒️note`) and is paid once per instance. |
| 9 | `🌉️mcp/🏠️workspace/🦀️.rs:1081-1098` | `exchange_one_slice` does not advance the page cursor on a resume slice. |
| 10 | `🔌️plugin/🧠️interpreter/🦀️.rs:1857-1875` | `CoreInstance::diagnose_fault` — a negative `__stack_pointer` is now reported as *"the guest's shadow stack underflowed at call depth N (stack pointer P); link the component with a larger `-zstack-size`"* instead of a bare address. |
| 11 | `📡️spr/🧵️channel/🦀️.rs:1334-1338,1612,1727,1871-1887` | `PagedAppCommandDecodeState::PureCommandFields` — the retained decoder route `AppCommand::PureCommand` (tag 13) never had. Before this, every mutation was refused by the guest with `plugin.command-route-state-machine-required`. One bounded field per step. |
| 12 | `📡️spr/🧵️channel/🦀️.rs:1118-1183` | `CursorShape` + terminal cursor rule: a terminal status carries `last_page_index + 1`, and `⚛️reactor/🔄️turn/🦀️.rs:1047-1066`'s **single-page fast path** publishes only that terminal (no per-page `PageAccepted`) for every `page_count == 1` command. `CommandComplete` is validated against `page_index + remaining_pages` and releases the one outstanding page. A `Fault` cursor is accepted at either end so a cursor complaint can never replace the guest's own message. |
| 13 | `📡️spr/🧵️channel/🦀️.rs:1147-1178` | `plugin.command-cursor-mismatch` now names the offending field and both values. That opacity cost two full diagnosis rounds. |
| 14 | `🌉️mcp/🗿️artifact/🧪️tests/🔬️quick/🦀️.rs:124,130` | stale test retargeted from the undeclared `test.kind` to the gateway probe kind — A1's new kind validation had made it red. |

### 4.1 Why the link arg, and not just the arena

Both were real, and one rebuild closes both. The arena fix alone would make a 1 MiB component fit
today and break again on the next large static; the config fix alone leaves a 93.5 KiB static costing
654 KiB of stack to zero. Moving `-zstack-size` invalidates every `wasm32-wasip2` fingerprint — a
real cost under a fleet, and unavoidable either way, since `semio-framework-ui-contract` is upstream
of every plugin.

## 5. Proof

### 5.1 Host laws — 4/4 green

```
cargo test -p semio-framework-plugin-host --lib owned_instance_open -- --test-threads=1
test …::a_mid_flight_owned_turn_refuses_new_events_instead_of_dropping_them ... ok
test …::a_trapped_owned_instance_refuses_every_later_turn ... ok
test …::owned_runtime_instance_open_settles_against_a_real_plugin_component ... ok
test …::wasmtime_runtime_instance_open_settles_against_a_real_plugin_component ... ok
test result: ok. 4 passed; 0 failed; … finished in 54.18s
```

### 5.2 The staged component

`cargo rustc -p semio-s-plugin-note --target wasm32-wasip2 --profile wasm-dev` (1 m 46 s) and the same
for `semio-s-plugin-draw` (2 m 14 s), both with `CARGO_PROFILE_WASM_DEV_DEBUG=false`. After the
rebuild the owned `InstanceOpen` stopped trapping and started answering **real guest verdicts** —
first `plugin.internal: unknown app` (my test's placeholder app id; the guest's real app-routing
refusal, and itself proof the open now reaches app routing), then, with
`s.note.note@1/*#editor`, a settling `TurnStatus::Idle`.

### 5.3 The "−112 per attempt" drift, explained and closed

`resume_owned_operation_observed`'s `Fault` arm took `state.pending` and never restored it, so the
next call started a **new** guest call on an instance whose `__stack_pointer` was still wherever the
trap left it — one more 112-byte frame, one more trap, 112 bytes lower. That is the whole of A1's
"decreases by exactly 112 per attempt". Fix 5 poisons the instance instead; law
`a_trapped_owned_instance_refuses_every_later_turn` holds it, and asserts the converse too — a guest
that answers with a `Fault` is healthy and must **not** be poisoned.

### 5.4 e2e gate — `🗑️generated/a2-client-e2e.txt`

`bun nx run @semio-tech/framework-os-mcp:client-e2e`, against both real `.mcp.json` servers:

```
PASS  os: initialize — server=semio-os-mcp@0.1.0 protocol=2025-06-18
PASS  os: tools/list — 27 tools, all 9 required present
PASS  os: capabilities_search — hits=20 first=block.s.block.block2d@1/*#editor.edit
FAIL  os: capability catalog health — 30 diagnostic(s) (A1 §3: descriptor regeneration, not this slice)
PASS  os: artifact_create — artifactId=mcp-client-e2e-mu8643ms revision=1
PASS  os: artifact_open — kind=os.agent.probe/v1 sizeBytes=424
PASS  os: capabilities_describe — draw.s.draw.drawing@1/*#editor.setSnapshot
FAIL  os: artifact_create (a real plugin artifact kind) — `draw` refused ReadArtifact:
      command acknowledgement: plugin.command-cursor-mismatch
FAIL  os: artifact_export — no artifact→plugin binding (the create above never landed)
FAIL  os: action_prepare — command acknowledgement: plugin.command-cursor-mismatch
PASS  repo: initialize / resources/list (8) / resources/read repo://goals (9222 bytes)
```

**The measured movement is the point.** The same three rows failed for A1 with
`InstanceOpen: guest trapped: wasm trap: memory write is out of bounds: start=4294409068`. They now
fail *past* the open, inside the retained command wire: `draw`'s guest comes up, is routed to, and
answers. An intermediate run (before the `draw` rebuild) printed the new diagnosis verbatim —
`… start=4294409068 … — the guest's shadow stack underflowed at call depth 26 (stack pointer
-558240); link the component with a larger -zstack-size` — which is both the proof that the new host
code is live and a demonstration of fix 10 on an un-rebuilt component.

## 6. What is still red, and why it is not this slice

### 6.1 A stale retained command owner in the guest (the current blocker)

`plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired`, with fix 13's named
mismatch:

```
plugin.command-cursor-mismatch: ingress status does not identify the exact retained host owner:
owner is 1, the retained owner's is 554
```

The host's `CommandBatchDriver` owner is the command `seq` (`🏠️workspace/🦀️.rs` passes `seq` to
`CommandBatchDriver::new`), currently 554. The guest answers for owner **1** — the first command this
channel ever sent. `⚛️reactor/🔄️turn/🦀️.rs:943-958`'s `if let Some(owner) = retained.take()` block runs
**before** the current page is looked at, so a `CommandIngressOwner::Generic` retained from an early
`output.retry_command` that never completes answers every later turn with its own stale cursor, and
the host correctly refuses it. Every mutation verb is behind this. It is a guest retained-command
lifecycle defect, in `⚛️reactor`, not in the plugin host or the gateway — adjacent to M5's declared
protocol-conformance scope, and I did not start a redesign of that wire under a live fleet on
inference alone.

### 6.2 Capability catalog — 30 skips

Unchanged and untouched: A1 §3 root-caused these to descriptor regeneration (~6 min per plugin, and
the one sampled regeneration still failed in the emitter). Not this slice.

### 6.3 Peer reds observed in passing

`semio-framework-os-kernel` is red independently of my hunks (which are additive — verified by
`git diff --stat`): `paged_generic_decoder_admits_…` now needs more than its 4 decode steps for
`LoadDocumentArchive`, and five `os_store::component::tests::*` dispatch laws fail. Both sit in files
a peer is editing (`🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`). Reported, not touched.

## 7. Honest gaps

1. **The mutation chain is not green.** `action_prepare`/`action_invoke` → snapshot → undo → redo →
   rollback → export are all behind §6.1. Named with exact evidence; not fixed.
2. **Fix 12 (the terminal cursor rule) is unit-green but not proven end-to-end.** It is derived from
   `⚛️reactor/🔄️turn/🦀️.rs:1047-1066`'s single-page fast path and is necessary, but §6.1's `owner`
   check fires first, so no run has yet exercised the `pageIndex` branch against a live guest.
3. **Fix 11 (the `PureCommand` decoder) is proven only to the next layer.** It moved the guest's
   answer from `plugin.command-route-state-machine-required` to a real `CommandComplete`; the command
   has not yet been observed to apply a mutation.
4. **The capability probe (`🐍️a1-capability-probe.ts`) across cad/draw/note/raster/layout/architect
   was not rerun.** Each plugin needs a fresh ~2–3 min wasm rebuild to leave the 1 MiB stack behind,
   and every one would stop at §6.1 rather than at the trap — the same information the gate above
   already gives, at six times the build cost. The architect `BUDGET_EXCEEDED` half of that item **is**
   addressed structurally by fix 7 (the gateway now pumps to settle instead of surfacing the first
   8 ms slice yield), but it is unmeasured against architect itself.
5. **Only `note` and `draw` have been rebuilt** with the corrected link args. The other 18 staged
   components still carry a 1 MiB stack and will trap until rebuilt; after fix 4 they need far less
   stack, but they have not been measured.
6. **`retire_until_complete`'s cost was not investigated.** The open spends ~25 s of interpretation
   for `🗒️note`; that is why fix 8 exists. Whether that is inherent to the interpreter or a guest-side
   pathology is unmeasured.

## 8. Files changed

| file | change |
| --- | --- |
| `.cargo/config.toml` | `-zstack-size=8388608` for `[target.wasm32-wasip2]`; the `wasm32-unknown-unknown` comment corrected to point at it |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs` | const arena + const-initialised `Mutex` static, `LazyLock` import dropped |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` | `poisoned`, `turn_in_flight`, mid-flight refusal, trap poisoning, `pub retryable_lifecycle_turn`, new test module registration |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs` | new: four laws (owned open, wasmtime open, poisoning, mid-flight admission) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs` | `diagnose_fault` — shadow-stack underflow named for what it is |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` | `PureCommandFields` retained decode route, `CursorShape` + terminal cursor rule and its page release, field-naming mismatch message |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | `ensure_instance` pump + receipt ACK + `undelivered` + trapped-instance drop, `INSTANCE_OPEN_WALL_BUDGET`, resume-slice cursor guard, caller loop collapsed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗿️artifact/🧪️tests/🔬️quick/🦀️.rs` | stale `test.kind` retargeted to the gateway probe kind |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts` | duplicated stack link arg + dead constant/export removed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts` | same duplicate removed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` | both `pluginCargoArgs` laws updated; the second now asserts the flag lives in `.cargo/config.toml` |
| `…/🎫️tickets/…/📓️a2-mcp-plugin-host-instance-open.md` | this report |

Captures (delete with `🗑️generated`): `a2-build-note.txt`, `a2-build-draw.txt`,
`a2-mcp-channel-test.txt`, `a2-os-mcp-suite.txt`, `a2-kernel-suite.txt`, `a2-host-laws.txt`,
`a2-client-e2e.txt`.

**Disk:** `⚡️cache/cargo/build` hit ENOSPC mid-slice; pruned stale incremental sessions
(`-mindepth 1 -maxdepth 1 -mmin +120` under the two `incremental` roots), freeing 7.4 GiB at the time
and leaving 79 GiB free at the end. No peer artifact was removed.
