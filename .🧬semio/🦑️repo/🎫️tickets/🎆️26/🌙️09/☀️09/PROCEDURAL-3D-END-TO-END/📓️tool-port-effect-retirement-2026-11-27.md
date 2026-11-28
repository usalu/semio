# Tool port queued final effects: paid per-effect retirement (2026-11-27)

## Root cause

`ToolRunJobPort::original_close_demands` and `OriginalToolRunJobPortRetirement::demands` refused with `UnsupportedOwner: original tool port ... effects require native retirement` whenever the sole lease of a port still held queued host `Effect`s (an undelivered `flowEvalTick` or `invokeExtension` of the predecessor `previewEval` run job). `switchToPluginApp` retires the predecessor instance, the retirement ladder hit the refusal, the instance never retired, and the later `batched publication ... stale` and `typed command requires its complete original source` refusals followed. The port law pinned the refusal.

## Design

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/⏯️tool-run/🦀️.rs`.

- `original_close_demands` (lease capture) no longer refuses a non-empty sole queue; it keeps the weak-backing and unavailable-queue refusals. The quote stays `copy 0, depth 1`.
- `OriginalToolRunJobPortRetirement` gains `staged: Option<Effect>`. Ladder per cursor: (1) release the lease and capture the native queue, (2) while the queue or the staged slot is non-empty, retire the front `Effect` as a separately funded child through `admit_retirement_cursor` (its `RetireOwned` cursor, returned as `RetirementStep::ChildProgress`), (3) release the empty queue backing.
- `demands()` quotes exactly the next move up front: with a staged or queued effect it is `retirement_cursor_birth_demand` of that effect (copy, capacity, depth), without any pop, allocation or release. `next_birth_bytes` now reports the quote's capacity bytes.
- `close_step` checks the quote against the grant and work units before it pops the front effect into `staged`; admission refusal returns `BudgetExhausted` with the effect retained (staged), failure keeps it too, so every step is resumable and cancellation at any boundary leaves queue, staged slot and quote consistent. Refusal now only means genuine budget exhaustion.
- `terminal_is_empty` and the drop assertion include `staged`.

## Laws (replacing the pinned refusal)

Tests in `⏯️tool-run/📮️port/♻️retirement/🧪️tests/🦀️.rs` (package `semio-plugin-tool-run`):

- `original_tool_run_port_preserves_atomic_final_queue_and_full_grants`: the former `is_err` pins now assert a queued sole port is quoted and admitted (weak backing still refused).
- `original_tool_run_port_retires_every_queued_final_effect_through_the_paid_cursor` (a).
- `original_tool_run_port_quotes_effect_demands_exactly_before_any_move` (b): quote equals the effect's cursor birth demand, quoting allocates nothing, each denied axis (items, depth, capacity, copy) moves nothing.
- `original_tool_run_port_cancellation_mid_retirement_leaves_a_resumable_state` (c).

Language-agnostic law: fixture `🧫️fixtures/🔣️.json` case `strong 1, weak 0, queued 1` is now `allowed: true`; `🟦️.ts` admission SQL is `strong>1 OR (strong=1 AND weak=0)` and asserts the refusal text is gone from the source.

## Test results (superseded below)

The two failures listed in the first run were not caused by the retirement change; root cause and fix follow.

## Root cause of the two failing heap laws (macOS-only hidden allocation)

`ToolRunJobPortState.effects` was a `std::sync::Mutex<VecDeque<Effect>>`. On unix targets other than Linux/Android/BSD-futex (macOS here) `std::sync::Mutex` is the pthread implementation, which lazily boxes a 64-byte `pthread_mutex_t` (`OnceBox`) on the FIRST `lock()`/`try_lock()` and frees it when the state drops. That allocation is invisible to every quote:

- `admit_original_dispatch` (first call that reaches `try_lock`, here the capacity-0 denial in turn 0) observed 64 requested bytes on a call that must not allocate.
- the state's drop released 64 more bytes than `shared_retirement_allocation_bytes::<ToolRunJobPortState>()` (136 instead of 72) in the leading empty-queue loop.

The laws are right (they pin that a refusal allocates nothing and that release equals the quoted Arc extent); the production type allocated behind the quote on macOS only (Linux/Windows use futex mutexes, which is why the laws passed there). Not related to `staged` or the bank (the bank is not involved in the first failure).

Fix (production, platform independent): the queue is now `std::sync::RwLock<VecDeque<Effect>>` used only through `write()`/`try_write()`. std's unix RwLock is the allocation-free queue/futex implementation (the uncontended path never allocates), so lock use and state drop allocate or release nothing beyond the Arc extent on every platform. Files:

- `...plugin/📜️declaration/⏯️tool-run/🦀️.rs` (`effects` field type, `try_lock`->`try_write`, `lock`->`write`, docstring on the state).
- `...plugin/⏯️tool-run/📮️port/♻️retirement/🧪️tests/🦀️.rs` (`.lock().unwrap()`->`.write().unwrap()` in the leading loop).
- `...plugin/⏯️tool-run/📮️port/🎟️admission/🧪️tests/🦀️.rs`: new law `original_tool_run_port_first_lock_and_drop_allocate_nothing` (fresh port: quote, `has_effects`, `pop_effect` allocate 0/0; drop releases exactly the Arc extent).
- `...plugin/⏯️tool-run/📮️port/♻️retirement/🧪️tests/🟦️.ts`: Ajv2020 -> Ajv (draft-07), because the schema declares `$schema` draft-07 and Ajv2020 refuses it (`no schema with key or ref "http://json-schema.org/draft-07/schema#"`); same as the sibling admission law.

## Test results

- `repo cargo test -p semio-plugin-tool-run --lib -- --test-threads=1` (prototype, shared): 18 passed, 0 failed, including both formerly failing laws, the three new retirement laws and the new no-allocation law.
- bun laws: `⏯️tool-run/📮️port/🎟️admission/🧪️tests/🟦️.ts` 1 pass (9 expects); `⏯️tool-run/📮️port/♻️retirement/🧪️tests/🟦️.ts` 1 pass (51 expects) after the Ajv fix. Intermediate runs failed with "2 errors building ...process/📦️artifacts/💾️cache/💾️volume/🟦️.ts", an unrelated concurrent edit of another agent that settled by itself.
