# Note create: why the guest refused and why nobody saw it (2026-11-25)

Symptom: `artifact_create s.note.note` ran 69 to 75 s and failed `INTERNAL guest poll: Refused: None`, then cleanup logged `InvariantViolated: Poll still owns its original actor failure reference`.

## Cause

The guest did not trap and the interpreter did not misexecute. The ordinary Poll export (`semio_owned_poll_v1`) calls the pure import `operation-begin`; the host drives that export through `advance_original_scalar_step`, which carries no `OriginalOperationReceiving`. The interpreter import path (`🚪️io/📞️imports`, `Request::PureBegin`) then replies the refusal `original operation begin has no actual receiving loan`, and the guest ends its turn with a semantic failure (receipt `Failure { authority: None }`, SAF1 Signal). `advance_original_scalar_step_receiving` existed and had no caller.

## Why nothing reported it

1. `GuestPollSession::acknowledge` returned `Refused` for a retained actor failure reference without writing the session failure slot, so `poll_fault` read `failure()` and `cleanup_failure()` as `None` (`Refused: None`).
2. The failure service that would recover guest text is unwired in production: `failure_active` is never set, and `select_failure_plan_step`, `receive_failure_budget_step`, `begin_failure_scalar_step`, `drive_failure_scalar_step` have no callers (dead code). Cleanup therefore always stops at `Poll still owns its original actor failure reference` and the guest Source stays unretired until process end. This is not fixed here.
3. Even when the service is driven (Bun, below) the published bytes of an Input failure carry only `outcomeValue.retainedProgress`; kind and message are not in the projection.

## Evidence

Real server, binary built from the tree, one run each (`🗑️generated/r1..r6`, deleted at the end):

| Run | Result |
| --- | --- |
| before | 69.1 s, `guest poll: Refused: None` |
| with the surfaced cause | `Refused: InvalidValue: guest actor refused its ordinary turn: phase ReleaseHeader; last guest receipt Failure { authority: None }, 3190 guest work units; guest actor failure reference retained without a failure-service crossing; execution fuel spent=733733584; <ledger>`; fuel 733,733,586 and 733,733,584 in two runs (deterministic) |
| receipt trace (temporary `[DEBUG]`, removed) | Boot 792 work / 116 items / 26,861 copy / 13,272 capacity, then 6,176 / 4,943 / 2,008,141 / 1,982,016; Input 11,492 / 6,479 / 1,244,693 / 557,714 / 556,544, then 3,265 / 152 / 19,776 / 908 / 1,484 (ready and returned); Poll `Failure{None}` 3,190 / 76 / 12,619 |
| import trace (temporary, removed) | exactly one `operation-begin receiver_present=false`, during Poll |
| after the fix | no refusal and no cleanup failure; the turn runs on and ends `BUDGET_EXCEEDED guest poll reached its wall budget before it settled` after 151.8 s (`INSTANCE_OPEN_WALL_BUDGET` = 150 s; unoptimized `prototype` interpreter) |

Independent engine (Bun / JavaScriptCore, `note-poll-bun.ts`, a copy of `owned-input-jit.ts` with `--tuple-limits`, current component `55e0fb94…`): Boot receipts are identical to the server's (792/116/26,861/13,272 and 6,176/4,943/2,008,141/1,982,016); the second Input receipt is identical (3,265/152/19,776/908/1,484); the first Input differs only through the wire (11,170/6,238/1,244,531/557,552/556,463 for the 437-byte fixture wire). With the pure operation imports answered by a ledger, `poll` returns `SAC3 kind 5 flags 2` (TurnOutput) in 38.7 ms; without them it is the failure above. The same harness shows that the object-form limits (no `--tuple-limits`) make the guest refuse the Input itself (`kind 7`), unrelated to the server.

## Interpreter conformance (task item 2)

Not the cause. About 650 M prepare instructions plus the Boot, Input and Poll receipts reproduce an independent engine's guest-reported effects byte for byte, so the paid interpreter executed the guest faithfully through the pool, label type-vector recycling, branch result moves and bulk moves. Static review: locals are born with `ValueType::zero` per signature type (the `Value::I32(0)` wipe only precedes `clear()` when a vector enters the pool); `memory.copy` chunks go backward exactly when `destination == source && to > from && to < from + length`; branch result moves copy forward to a lower height; label vectors are recycled only after their frame left the control stack. A Wasmtime-oracle regression vector was added anyway (below). The ticket's captured component and transcript are gone and the cached native-host dependency set the replay driver links against is gone, so the byte-exact replay was replaced by the Bun comparison.

## Changes

- `🔁️poll/🦀️.rs`: `refusal_cause` (phase, scalar or input failure, last guest receipt and work units, retained actor reference), field `fixed_seen`, field `operation`; the Poll export now runs through `advance_original_scalar_step_receiving` with a `GuestOperationLedger` sized by the issued semantic capacity, dropped when the export is no longer pending.
- `🔁️poll/🎛️session/🦀️.rs`: an ordinary refusal now fills the session failure slot (cause, execution fuel spent, interpreter ledger) unless a failure is already held.
- `⚠️failure/🧮️operation-ledger/🦀️.rs` (new, exported as `GuestOperationLedger`): the reservation ledger; it mirrors the codec crate's private `Reservations` (follow-up: share one implementation). It lives in the failure crate because the poll crate's test closure is within 1.6 KB of its 262,144-byte boundary and a new crate edge would change the graph. Its two unit tests cannot run: that crate is not a workspace member for tests (same as its existing fetch test).
- `🌉️mcp/…/🔁️native-poll/🦀️.rs`: `poll_fault` reports kind and text of any retained value failure instead of `Debug` of the slot.
- Tests: `🧪️tests/🔬️metering/🦀️.rs` gains `nested_branches_calls_and_overlapping_copies_match_wasmtime_at_every_pacing` (nested blocks, loops, `br_if`, `br_table` moving results over extra operands, recursion depth 40, locals of i32/i64/f32/f64 reusing pooled vectors, overlapping `memory.copy` both directions; result, instruction count, receipt and ledger equal Wasmtime at turn fuel none, 2, 3, 7, 64) and the close loop of `measure` allows 20,000 steps (deep recursion pools more than 200 vectors); the poll tests assert `refusal_cause`, and two stale exact-message assertions (the coordinator's ledger suffix) became prefix checks.

## Verification

- `test -p semio-plugin-interpreter-core-laws --lib`: 22 passed, 0 failed (includes the new vector).
- `test -p semio-plugin-host-poll --lib`: 17 passed, 1 failed. The failing `actual_poll_refuses_and_retires_an_unfundable_returned_frame` never reaches its refusal; it predates this ticket (frame pool makes recycling free, interpreter-metering Update 3 lists it as stale); not touched.
- Real server: see the table; last binary run noted at the end of the summary.

## Open

- The first note create still cannot finish inside the 150 s wall budget on the unoptimized `prototype` interpreter; the Poll export instruction count is not yet measured (Bun: 38.7 ms).
- Wiring the failure service (so cleanup retires the Source and clients get guest text) remains; until then any actor failure leaves the Source unretired and the Fault carries only the host-side facts above.
