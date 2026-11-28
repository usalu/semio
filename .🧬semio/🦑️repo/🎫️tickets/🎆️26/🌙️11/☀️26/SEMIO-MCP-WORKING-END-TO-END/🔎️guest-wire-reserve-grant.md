# 🔎 Guest Core wire reserve refusal on `artifact_create s.note.note`

## Status

`artifact_create` of `s.note.note` does NOT yet succeed natively. It now ANSWERS with a typed refusal on every run (BUDGET_EXCEEDED, INTERNAL), never hangs. The grant refusal itself is fixed at its root; the remaining blocker is the cost of the cold `Event::InstanceOpen`.

## Root cause of the wire-reserve refusal (measured)

A `[DEBUG]` print of the ledger at the refusal (since removed) showed, after only 2 completed turns:

- Infrastructure account: observed items 130 980 of 131 072, copy 21.5 MB, capacity 3.1 MB, release 1.1 MB, work 5.18 M.
- Semantic account: 244 items. So the Infrastructure account (guest allocator and protocol work, cumulative per operation) is the one that starves.

`check_core_wire_grant` is correct. The authored `infrastructure` grant in `🌉️mcp/🎚️config/💡️host-operation-policy.json` (131 072 items, 32 MiB copy, 10 M work) was simply smaller than what a real note-actor open spends: about 49 k items, 7.8 MB copy and 2 M work per turn. Once less than the close reserve (93 items, 3 288 work) remained, every further crossing, including the cleanup crossing (CloseActor, header 200), was refused, so the guest Source could not be retired ("keeps its unretired guest Source"). The close path draws on the same cumulative Infrastructure stock, so exhaustion also blocks retirement.

At 5 turns the work axis was next (9 998 493 of 10 000 000 spent).

## Fix (grants, no code constant inflated blindly)

- `host-operation-policy.json` infrastructure: items 4 194 304, copy, capacity and release 536 870 912 (copy equals `maximumPreparedActorBytes`, the largest staged actor image), workFuel 2 000 000 000.
- `host-driver-policy.json` operation: operationFuel 40 G (the host burns about 0.5 G of metering fuel per turn and the infra hold reserves 2 G), items 480 M, copy 30 G (measured 2.04 GB operation copy after 7 turns).
- Law test `authored_operation_policy_infrastructure_funds_the_largest_staged_actor_and_every_close_reserve` in `🌉️mcp/🎚️config/🧪️tests/📥️first-response/🦀️.rs`: the infrastructure grant must cover the largest staged actor image on copy, at least 16 times the measured five-turn open cost on items, capacity, release and work, and the sum of all close first-response reserves.
- `.cargo/config.toml`: `opt-level = 3` for the metering and executing crates (interpreter, interpreter-runtime, numeric, imports, framework-job*, framework-value*, framework-actor, host post-turn-relay, wake, guest-runtime, poll). Open throughput rose from about 4 M to about 13 M fuel units per second (211 s: 838 M units before, 2.8 G after).
- No code changes remain in the guest crate (the diagnostic print was removed); `INSTANCE_OPEN_WALL_BUDGET` is back at 210 s (a 1500 s value was only a diagnostic).

## Typed refusal instead of hang

With current sources every failure reaches the MCP caller: `BUDGET_EXCEEDED` (retryable) with the poll stage, loan header and fuel or the currency ledger in the message. The 5400 s hang of the earlier binary did not reproduce; the instance is retired through `discard_instance` before the reply.

## Probe results (create only, `e2e-note-2`)

| Binary state | Reply | Message |
|---|---|---|
| opt 0, old grants | 211 s BUDGET_EXCEEDED | wall budget, 838 M fuel, 0 turns |
| opt 3 jobs, infra 4 M items | 220 s | wall budget after 5 turns |
| infra work 2 G, operationFuel 4 G | 147 s | semantic work exhausted (host fuel) |
| operationFuel 40 G, 210 s | 302 s | wall budget after 5 turns |
| 1500 s budget, operation items 48 M | 225 s | operation items exhausted after 7 turns: spent 31.7 M, held by guest 16.3 M |
| 1500 s budget, operation items 480 M, copy 30 G | 411 s, INTERNAL | after 12 completed turns `guest poll: Refused: None` |

## Sample (top frames, 10 s sample at 100 s into the open, opt 3 build)

1. `CoreInstanceOpening::advance_scalar` closure (the guest wasm instruction loop): 858
2. `OriginalReturnBudgetSnapshot::remaining` (job_payload return escrow): 294
3. `OriginalReturnBudgetSnapshot::advance_work_exact`: 220
4. `StepContext::fuel_remaining`: 201
5. `OriginalReturnBudget::remaining`: 198
6. timer `atomic_load`: 172
7. `ClockStride::read`: 149
8. `_platform_memmove`: 146
9. `OriginalReturnBudget::record_work_exact`: 133
10. `StepContext::deadline_exceeded`: 103

The budget and clock bookkeeping around each guest step is as hot as the guest instruction loop itself.

## Conclusion and open leads

The cold open of the note actor needs at least 12 turns and about 3.5 GB of operation copy, 177 MB of `memory.grow`, 941 k host-import results and 22 MB of checkpoint image (ledger from the 7-turn refusal), and takes about 34 s per turn. That is far above the 210 s open wall budget. Remaining leads, in order:

1. Per-turn copy of about 290 MB: the poll stage 0 outside the interpreter spent 843 k items and 760 MB copy in 7 turns; the frozen capture and escrow apparently re-copy guest memory every turn.
2. Per-instruction budget bookkeeping (`remaining`, `advance_work_exact`, `fuel_remaining`, `ClockStride::read`, `deadline_exceeded`): batch it per quantum instead of per scalar step.
3. The guest does heavy work at open (2.8 G instructions in 5 turns); a guest-side profile through the interpreter trace observation is needed to find avoidable initialisation.
4. After 12 turns the poll answers `Refused: None` (status Refused with neither failure nor cleanup failure retained); that path needs its own typed cause.

The ticket's labels migration (other agent) changed the descriptor decoder and the component-models layout (`component-models/<digest>/<generation>`); the probe used a patched scratch component root under `🗑️generated/note-root3`.

## Cold-open performance

### Findings (static, from the sources)

1. **The 290 MB per turn is a ledger figure, not a memmove.** Guest linear memory is resident: `CoreInstance.memories[..].bytes` lives in the retained `original_core` of the guest instance across turns, and `OriginalMemoryWork` (`🧠️interpreter/🏗️instance/▶️invocation/💾️memory`) grows, fills and copies it in place in 1 MiB chunks (one work unit per 64 bytes). The `copy` axis counts bytes the metering reports as moved; the 10 s sample shows `_platform_memmove` at 146 of about 3 500 frames (about 4 %). No per-turn snapshot or clone of guest memory exists, so there was nothing to make resident. The 22 MB checkpoint image is the prepared actor image (`imageBytes` 21 263 738), loaded once.
2. **The wall time is the instruction loop plus its per-instruction bookkeeping.** `CoreInstanceOpening::advance_scalar` ran, for EVERY wasm instruction: `is_cancelled` (atomic load), `deadline_exceeded` (clock stride), `should_yield` (`fuel_remaining` walks `OriginalReturnBudgetSnapshot::remaining`, then another `deadline_exceeded`), a second `deadline_exceeded`, and `consume_fuel(1)` (clock `note_work`, `record_work_exact` building and checking a whole snapshot). Together that was as many frames as the instruction decode itself.
3. **`Refused: None`** came from `GuestPollSession::advance_native`: the interpreter-side `GuestPollStatus::Refused` was returned to the caller without ever retaining a failure (only `open_step` retained `poll.refusal_cause()`), so `poll_fault` found neither `failure` nor `cleanup_failure`.
4. **`authored_mcp_policies_fund_actual_owner_first_response_without_expanding_runtime_grants` (3286 vs 3052) is legitimate fixture drift, not a source defect.** `OriginalEscrowPublisher::publication_demand` and `core_wire_reserve` agree on 94 work units per row (`132 + 94 * 32`, `maximum_reserve_dominates_actual` proves it dominates the actual cost); the fixture's `coreWire` 2 950 is the older 88 per row (`132 + 88 * 32 + 2`), and it omitted the table (35) and publisher (9) birth work that `first_response_reserve` has always added. Items (93) are unchanged. The fixture is regenerated: `coreWire` 3 142, `tableBirth` 35, `publisherBirth` 9, the remaining owners as before minus the unexplained `pendingInputWire` work 2, total 3 288 (3 286 for the non-maximal headers the test checks). The root `turnFuel` 262 144 still dwarfs it.

### Changes

- `🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs` (`advance_scalar`): cancellation, deadline and fuel checks run once per `SCALAR_WINDOW` (1 024) loop iterations instead of per instruction; pure scalar instructions and linear-memory loads and stores are admitted against a fuel window computed once from `min(execution fuel left, ledger fuel left, 1024)` and their fuel is posted to the Source ledger in ONE `consume_fuel(owed)` before any other ledger interaction (every control, call, host, bulk-memory, frame-return and growth path settles first) and again when the call returns. Total posted fuel is identical to the per-instruction path; only the check cadence changes (cancellation is observed at most 1 024 iterations late, a deadline likewise plus the existing clock stride). A call with a test trace keeps the window at 1 so the category accounting tests stay exact. `admit_memory_instruction` no longer pays work itself.
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs`: `OriginalReturnBudget::remaining_fuel()` answers the CPU part of `remaining()` in O(1) without building a snapshot or a `ValueError`; `StepContext::fuel_remaining` (`⏱️context/🦀️.rs`) uses it. `OriginalReturnBudgetSnapshot::{conserved, retained_conserved, work_conserved, advance_work_exact}` are `#[inline]` across the crate boundary (`📦️builder/🪙️return-escrow/🦀️.rs`).
- Typed refusal: `GuestPollSession::advance_native` (`🖥️host/🔁️poll/🎛️session/🦀️.rs`) retains `guest actor refused its ordinary turn: <poll.refusal_cause()>; execution fuel spent=..; <ledger>` when the VM answers `Refused` without an input failure; `poll_fault` (`🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🦀️.rs`) now answers every guest refusal as fault code `guest.refused` (new `GUEST_REFUSED_FAULT_CODE` and `GUEST_REFUSED_REMEDY` (en, de) in `🌉️mcp/🔀️dispatch/🔌️channel-port/🦀️.rs`, mapped by `map_fault` to `PluginUnavailable` with `details.remedy.{en,de}`), carrying the poll stage diagnostic even when no failure was retained.
- Fixture: `🌉️mcp/🎚️config/🧪️tests/📥️first-response/🔣️.json` (see finding 4).

### Tests added

- `scalar_window_posts_exactly_the_fuel_of_the_admitted_instructions` and `scalar_window_stays_within_its_declared_bound` (`🧠️interpreter/🏗️instance/▶️invocation/🧪️tests/🦀️.rs`): a 600-instruction straight-line module run with 1 fuel per call and with a whole window per call posts the same total fuel, and both agree with Wasmtime (900).
- `remaining_fuel_agrees_with_the_snapshot_remaining_in_every_state` (`🪙️budget/🧪️tests/🦀️.rs`).
- `guest_refusal_maps_to_a_typed_plugin_fault_with_an_en_and_de_remedy` (`🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs`).

### Probe result (binary built with the window, remaining-fuel and typed-refusal changes; copied to `🗑️generated/semio-os-mcp`)

`artifact_create s.note.note` does NOT yet succeed, but it now answers in 177.6 s (previous binary: 411 s, i.e. about 2.3 times faster; about 15 s per guest turn instead of 34 s, about 31 M fuel units per second instead of 13 M) and with a typed refusal inside the 210 s `INSTANCE_OPEN_WALL_BUDGET`: `PLUGIN_UNAVAILABLE`, `details.faultCode` `guest.refused`, `details.remedy.{en,de}`, message `guest poll: Refused: InvalidValue: guest actor refused its ordinary turn: phase Fault; guest execution Err(... Core original memory instruction exceeds its retained extent ...)` after 12 completed turns and 5.39 G fuel.

So the former `Refused: None` is a GUEST TRAP: the interpreter refuses an out-of-bounds linear-memory access by the note guest, deterministically in turn 12. It is not a budget. The guest also executes about 5.4 G instructions (401 k host-import calls, 245 `memory.grow` to 177 MB) before it traps, which is why the open is slow at all: no interpreter speed-up can make 5.4 G instructions take a few seconds, so the next lead is the guest itself (a runaway loop that ends in an out-of-bounds access, or a shadow-stack overflow of the 8 MiB `-zstack-size` in the prototype component). To localise it, the interpreter's out-of-bounds refusal now names `opcode`, `address`, `width`, `extent`, `function` index and `pc` (source change in `plan_memory_address`; the probe binary above predates that message, a rebuild is needed and was blocked by host memory admission: `semio_os_mcp_plugin_runtime` needs 6.1 GB while other agents held the host). The component carries no `name` section, so a function index needs mapping through the component's exports or a debug build.

Not compiled after the last edit: the out-of-bounds message in `plan_memory_address`. The law tests added above were not run either (the host had no room for `cargo test`); only `cargo build -p semio-os-mcp` of the sources before that message was verified.
