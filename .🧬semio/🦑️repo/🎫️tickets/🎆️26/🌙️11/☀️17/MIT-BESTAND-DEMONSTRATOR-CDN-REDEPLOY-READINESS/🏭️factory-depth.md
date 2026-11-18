# Factory quote vs. legacy close (original_turn_tests stall)

Static analysis only (no build allowed); patch: `apply-factory-depth-fix.py` (idempotent, not yet applied).

## Finding: depth is consistent, the copy quote is not executed
- Queue quote: `owner.next_demand(body)` with `depth += queue.len()`; queue step passes the owner `depth - len`. Owner `demands` already includes its child level (`+1`) in every state except the two native leaf stages. Both sides agree.
- `FactoryOwnedRetirement` has no `resident_reply`, so `close_owner` always takes legacy `close_step`.
- A fresh frame is `native_stage==0`, original present, `unique` empty. `demands` returns the native setup quote (copy 1 byte = `native_setup_bytes`, depth 1, work 1). That matches budget 1 and the work gate.
- Legacy `close_step` passes the gate, but never runs stage 0: it calls `produce()`, which needs `retire_owned_demands` (a full payload copy). With 1 byte it is denied: progress zero, work zero. Stage 0 exists only in `native_produce_into`.

## Which side is wrong
Execution (legacy `close_step`): the quote is the contract (the queue test, factory tests and the native path all rely on exact quotes). The quote must not be widened for stage 0, because native callers would then be over-charged.

## Fix
1. `close_step`: at stage 0 with original and no `unique`/`production`, record 1 work, set `native_stage=1`, report `copied_items:1, copied_bytes:demand.copy_bytes` (mirrors `native_produce_into`).
2. `produce()` completion resets `native_stage=0` (legacy never reaches native stage 2; `terminal_is_empty` requires stage 0).
3. Pin: `📋️queue/🧪️tests/🏭️legacy-owner.rs` (queue crate owns the effect bank; factory crate tests lack it, and `📦️owned/🧪️tests/🦀️.rs` uses an older API). Asserts quote (1,1), then every exactly quoted turn performs work until terminal.

## Risks
Unverified by execution. Stage-1 quote adds `native_reply_bytes` (over-quote, harmless). Run factory, queue and retirement suites with `--store=isolated` after applying.
