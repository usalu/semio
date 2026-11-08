# 1107 Audit Review 6: semio-os-history, semio-os-dsl, semio-os-spr, semio-os-flow-drawing, semio-os-inference

Read-only audit. No cargo run, no edits, no git modifying commands. Source reviewed by reading, not compiled or executed.

Base path used below: `B` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules`
Framework path used below: `F` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules`

Crates located by Cargo.toml name:
- semio-os-flow-drawing: `B/🌊️flow/🖍️drawing/📦️packages/🦀️rust/Cargo.toml`
- semio-os-spr and semio-os-history: `B/📡️spr/📦️packages/🦀️rust/Cargo.toml`, `B/📡️spr/📜️history/📦️packages/🦀️rust/Cargo.toml`
- semio-os-inference: `B/💡️inference/📦️packages/🦀️rust/Cargo.toml`
- semio-os-dsl: `B/🗣️dsl/📦️packages/🦀️rust/Cargo.toml`

Scope limits: the five crates total roughly 38k lines including tests. I read the work-accounting core in full (dsl receiving, dsl derive retained, inference driver, cursor, plan, values, the history metadata decoder, history fold, drawing store, drawing boolean retirement) and grepped every production loop, Pending/Blocked return, Drop impl and unwrap/panic site across all five crates. I did not read the history file/edit/conflict/transition decoders, the channel command-cursor (436 lines), the json-bridge and svg/pdf export, or the inference cache-key and dep-hash code line by line. Where a finding depends on a caller I could not see, I say so.

## Findings, most severe first

### 1. Ordinary exhaustion or error panics inside receive_to_completion (high confidence)

`B/🗣️dsl/🫴️receiving/🦀️.rs:189-205`

`receive_to_completion` returns `Err` when fuel runs out or a step fails, but the cursor is still live at that point. Its Drop runs at function end (line 205) with a non-terminal cursor. The owned fields assert on drop:
- `OwnedSlot<T>` Drop asserts `terminal_is_empty`: `F/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🧬️contract/🦀️.rs:102`
- `RetainedRecordReceiving` Drop asserts the same: `F/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🦀️.rs:180`

Concrete failure: a document whose operation needs more work than the caller's `ReceivingStock.fuel`. Once the decoder has entered the record or generic phase (phases 3 to 5, where owned state exists), `while !cursor.is_ready()` stops at line 193 with `stalled("exhausted its work")`. The close loop at 201-202 then gets `quantum_of` with `maximum_items == 0`, the wallet refuses, and it breaks at 202. The cursor is still non-terminal at 203. Line 205 drops it and the Drop assertion panics. The expected result is `Err(WorkLimit)`. Phases 0 to 2 hold nothing owned, so the panic needs the decoder to have reached phase 3 or later. The same happens for a step error that is followed by a close that cannot finish.

Fix shape: close or move out the owned fields before returning an error, or wrap the cursor so its Drop does not assert on the error path.

### 2. history fold drops a live ControlledRetirement on every error path (high confidence)

`B/📡️spr/📜️history/🧬️schema/🦀️.rs:225-241`, inside `HistoryLog::fold`

`retirement` is a `ControlledRetirement` (Drop asserts terminal-empty, `F/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:268-270`). The `while !retirement.terminal_is_empty()` loop at line 229 has many `?` and `return Err` exits: the quote at 230-231, `ok_or_else` on admission at 232, the `Pending` arm at 233 (which is documented as impossible but still returns Err with the retirement live), the failure arm at 234, and `wallet.post(...)?`. Each one drops `retirement` non-terminal and panics. Reachable triggers:
- A history large enough that the total retirement work exceeds the wallet stock `1 << 24` (line 228). The wallet refuses, so `run` returns `Ok(None)`, the `ok_or_else` at 232 fires, and the function panics instead of returning `ProtocolError::Malformed`.
- A per-step copy demand above the fixed `maximum_copy_bytes: 4096` at line 230.

Brief rule breach in the same block: `stock` at 226 uses `maximum_capacity_bytes: usize::MAX` and `maximum_release_bytes: usize::MAX`. These are admit-all constants, which the brief forbids ("never `u64::MAX`/`1<<40`-style admit-all"). `1 << 24` is a bounded constant, which is allowed.

### 3. Drawing Boolean: early error return drops a live retirement, plus an admit-all wallet (high confidence on the panic)

Source: `B/🌊️flow/🖍️drawing/🧱️kernel-impl/🦀️.rs:182-197`

`let (mut retired, _) = job.into_retirement();` returns `PathBooleanRetirement`. Its `physical_work_retirement!` macro wraps a `ControlledRetirement` (`F/◻️2d/🧹️retire/🦀️.rs:15-27`), so Drop asserts terminal-empty. Inside `while !retired.terminal_is_empty()` (line 187), each `?` (lines 188, 190, 191, and 195) returns while `retired` is still live. Reachable trigger: a Boolean whose retirement step needs more than `STOCK` allows (copy above `1 << 28` or capacity above `1 << 25`, both fixed in `STOCK` at line 161), or whose step work exceeds the `1 << 24` per-step bound at line 190. The `run_settled` quote then returns `None`, `ok_or_else` at 190 fires, and the thread panics.

Brief rule breach: `AuthoringWallet::new(STOCK, 1 << 48)` (line 186) is a 1<<48 admit-all work stock. The brief explicitly names `1<<40`-style constants as forbidden. The retirement also sits in a plain function with no turn, so the wallet is the only funded driver here; the stock must be a bounded size-derived constant, not `1 << 48`.

Also, the job driver at lines 171-181 calls `job.advance(4096, ...)` in an unbounded synchronous `loop` with `max_work: 100000000`. It has no progress reporting and no cancellation. This is outside the strict work-accounting faults but breaks the interaction rule.

### 4. Inference: the plain driver does no fuel or cancellation accounting, and the documented "slice" API is not a slice (high confidence)

`B/💡️inference/🏁️driver/🦀️.rs:99-184` (`drive`), `:206-214` (`update_field_cold`), `:203-205` (its doc)

- `drive` checks `cursor.cancelled` only once at entry (100-103). Its `while *next < plan.len()` loop (110) never consults fuel, yield, or cancellation. It calls `F::compute` (151) for every entity in one go and never charges any work.
- `update_field_cold` documents itself as walking "with the same fuel and cancellation as `infer_field_step`" (203-205). That is false. It takes no `cx` and no wallet, so the whole plan is computed in one call with zero recorded work.
- The `ComputeStep::Working` arm at 157-161 is unreachable: `outcome` is hard-wired to `Done` at line 151. The `pending`/`progress` path in the plain driver is dead.

Brief breach: no turn and no wallet (rule W1 requires an `AuthoringWallet` with an explicit stock on a no-turn path) means "unrecorded work" in the sense the audit asks about.

Related doc drift: `infer_field_after_diff` (`:235-237`) documents a tier-1 gate that returns the previous result when `diff.touches()` does not intersect `F::reads()`. The body (`:238-245`) never checks it and always calls `infer_field_delta`, so every diff walks the full plan. That is a performance regression against the contract, not a correctness fault.

### 5. Inference original path: unbounded panic-free stalls and zero-work returns that never admit (medium-high confidence)

Each item below returns "not yet" with zero or constant work when the grant is smaller than the requirement. No typed refusal is produced, so the caller sees `done == false` forever, which is the W4 close-liveness breach. The caller's loop in `infer_field_step` (`driver.rs:16-19`) breaks only on `maximum_items == 0`, not on a grant that never fits.

- `values.rs:39-40`: `reserve_parents_step` reserves the entire parent vector in one allocation (`count * size_of::<V>()`). If that exceeds `grant.maximum_capacity_bytes` it returns `Ok(false)` each time. A single entity with enough parents therefore never completes. The brief says "chunk rather than cap", and this does not chunk.
- `cursor.rs:76-77`: `close_original_pending_step` returns `Ok(false)` whenever `grant.maximum_release_bytes < bytes` (the pending box's release bytes). Nothing else bounds this, so a pending box whose release exceeds any grant the job can admit is never closed. Same for `plan.rs:166`, where `close` returns `Ok(())` forever when `grant.maximum_release_bytes < bytes`.
- Plan and values `close`/`metadata` helpers return `Ok(false)` on every short grant and never refuse. Liveness depends on the job's grant growing, which I could not verify.

### 6. Work performed by the effect is dropped on several early-return branches (medium confidence; reachability depends on the reserve and release functions)

- `plan.rs:150-154` (`append`): `Ok(PagedListReservationOutcome::Ready(receipt))` returns at line 152 when `receipt.copied_items == 0`, before `cx.consume_fuel(work)` (line 152, second half) and before `cx.consume_retained(receipt)`. `Denied` (line 153) returns without charging `turn.work` at all. If `reserve_capacity_one_funded` can perform work and still return zero copied items or `Denied`, that work is never debited.
- `values.rs:97-99` (`initialize_step`): the same pattern. `copied_items == 0` returns at line 98 before `cx.consume_fuel(work)`, and `Denied` at 99 charges nothing.
- `plan.rs:170` (`close`): `if released.progressed { consume_fuel(work); ... }`. When the release did work but reports `progressed == false`, the work is dropped.

Brief rule: "work charged twice or not at all". I did not confirm that these branches can carry non-zero `turn.work`. Check `reserve_capacity_one_funded` and `release_empty_page`.

### 7. Quadratic and cubic plan and driver loops (high confidence on the code shape; cost depends on graph size)

`B/💡️inference/🧭️plan/🦀️.rs`
- Phase 9 to 10 (topological decrement, `:114-128`): for every popped queue node (phase 8), the loop walks every child `0..N` (phase 9, `:115`), and for each child it scans the whole edge list (`:116-118`) looking for `(from, child)`. That is O(N² · E) metadata turns. The fix is an outgoing-edge list per node.
- Phase 11 (cyclic remainder, `:124-128`): O(N · order) scans.
- Phase 4 (edge dedupe, `:93-97`): every link scans every edge already recorded, O(L · E).
- Phase 0 (node dedupe, `:73-78`): every source scans every earlier node with byte-by-byte metered compares, O(N²) comparisons.

`B/💡️inference/🏁️driver/🦀️.rs:55-58` (CountParents) and `:66-68` (CopyParents): for each entity the driver walks every edge in the plan (O(E) metadata turns per entity), so the original field run costs O(N · E) turns. Each turn is charged, so this is not an unrecorded-work fault, but it is a quadratic cost on large fields.

Example: N = 500 entities and E = 1000 edges gives on the order of 2.5 × 10^8 metadata turns in phase 9 alone.

### 8. Zero-work "Progress" returns that loop forever under a caller that does not bound re-entry (medium; depends on caller)

These return `Progress` with zero work when the demand does not fit or the quote is larger than what is available. A caller that retries until terminal and has no fuel bound spins forever.
- `B/🗣️dsl/🫴️receiving/🦀️.rs:117-118` (`OriginalOperationReceiving::close`): `if !funded(grant,demand){return settle(effect,Ok(RetainedCloneStep::Progress(default)))}`. No Pending, no refusal, no work. `OperationRetirement::close_step` (line 148) exposes this directly to a `RetirementCursor` driver. `receive_to_completion` bounds it with `closes`, but other callers may not.
- `B/🗣️dsl/✨️derive/🧳️retained/🦀️.rs:52` (generated `decide`): `if selected==0||selected>maximum_work_units {return Progress(default)}` with `work` still 0. A leaf whose birth work exceeds the turn's maximum never gets selected, and every call returns the same zero-work progress.
- `B/🗣️dsl/✨️derive/🧳️retained/🦀️.rs:79` (generated `close_step`): same, when the close demand does not fit.
- `B/📡️spr/🧵️channel/🎞️app-frame/📥️receiving/🔗️causal/📦️payload/🦀️.rs:20` (stage 4): `if maximum<work {return Turn{result:Ok(Progress(default)),work:0}}`. If a single push needs more native work than one turn's `available_work_units()`, it never completes. This is the "chunk, do not cap" rule (W2) breached for a single push.

### 9. Drawing store: a live handle can lose its data to the engine cache (medium-high confidence)

`B/🌊️flow/🖍️drawing/🗄️store-and-geometry/🦀️.rs:38-44, 64-71, 77-83`
`F/◻️2d/🧮️compute/🦀️.rs:117-160`

`DrawingStore::derive_node` inserts the handle into `live` and stores the node bytes only in the engine cache. `EngineCache::derive` evicts LRU entries once the byte budget is exceeded (`ensure_budget`, lines around 146-158), with no regard for `live`. `DrawingStore::entry` (line 64-71) checks `live` and then reads the cache, so the handle still looks valid but the read fails with `EngineFault::Evicted`. There is no path to re-derive the node, because the StoredNode is discarded after derive. Result: a document node that is still listed as live becomes permanently unreadable, and every Group above it fails on export (`flatten_handle`).

`dispose_sync` and `retain_sync` (`:77-83`) remove only from `live` and never release engine entries, so memory is bounded by the cache budget, not leaked. That part is not a leak.

### 10. Drawing flatten: shared-child DAG expansion is exponential and uncharged (medium-high confidence)

`B/🌊️flow/🖍️drawing/🗄️store-and-geometry/🦀️.rs:98-109`

`flatten_handle` expands every Group child recursively with no memoization, no work accounting, and no cancellation. Groups are content-addressed and `group()` accepts any existing handles (`B/🌊️flow/🖍️drawing/🧱️kernel-impl/🦀️.rs:66-71`), so a group can reference the same child several times. A chain of `d` groups each containing the previous one twice produces `2^d` leaf scene nodes from about `2d` store entries. Recursion depth also equals group depth, with no guard, so a deep chain can overflow the stack.

Also `group()` (`kernel-impl:66-71`) does not check that its children are live. A group naming a missing or evicted handle is accepted and fails later at export. This is dropped validation, at lower severity.

### 11. Possible leak in the operation receiving close (unsure)

`B/🗣️dsl/🫴️receiving/🦀️.rs:120`: `std::mem::forget(self.binding.take())`. If the cursor type `M::Cursor` owns heap allocations beyond its terminal-empty fields, forgetting it leaks them. I did not check the generated cursor type, so treat this as a candidate, not a confirmed leak.

### 12. Charging the fuel directly instead of through the effect (unsure)

`B/💡️inference/🧮️cursor/🦀️.rs:78, 84-85`, `B/💡️inference/🏁️driver/🦀️.rs:25, 44, 56-79`, `B/💡️inference/🧭️plan/🦀️.rs:140, 170`, `B/💡️inference/🧺️values/🦀️.rs:44, 46, 98, 105, 168`: these call `cx.consume_fuel(...)` and `cx.consume_retained(...)` directly rather than through `with_original_effect` or `receive_original_retained_work`. The brief says the single debit must be the receipt. Whether these charge the same ledger that `original_work_units_spent()` reads (driver line 17-18, which breaks the loop when that counter does not move) I could not confirm. If it is a different ledger, the loop either stops after one event or the counter never reflects the charge.

### 13. Panics that may be reachable (unsure)

- `B/🗣️dsl/✨️derive/🧳️retained/🦀️.rs:57, 68`: `take_decision` and `take_inverse` panic on an `Unselected` leaf. Reachable only if a caller reads the outcome before `decide` admits a leaf.
- `B/📡️spr/📜️history/🦀️.rs:846-850` and `:901-915`: `resolve_ready` panics (`F/⏳️async/🔂️poll/🦀️.rs:10`) if a future is pending. The edit iterators use it on the frame and payload futures. Reachable only if the source is not fully cold.
- `B/🌊️flow/🖍️drawing/🌉️json-bridge` and `kernel-impl`: `unwrap_or_else(|_| unreachable!(...))` on `control.track` in `B/📡️spr/📜️history/🔁️fold/🦀️.rs:19-52`. Not checked whether the schema guarantees the branch.

## What I did not find

- No `Pending{work:0}` loop in the history, channel or drawing code that is driven by a `loop` with no admission bound, other than the zero-work returns in finding 8.
- The history metadata decoder (`B/📡️spr/📜️history/🛬️decode/🧾️metadata/🦀️.rs`) charges each state transition through `publish(metadata())`, and `close_step_body` (line 138) returns Blocked, not Pending forever, while the partial is still held by the parent. The parent edit decoder does call `take_partial_into` (edit decoder line 84), so I found no stuck close there.
- The dsl retained derive `close_step` (line 33) charges its terminal arm with a constant `work=1` for exactly one action, which matches the rule.

## Top finding

`B/🗣️dsl/🫴️receiving/🦀️.rs:192-205`: `receive_to_completion` returns an error with a live owned cursor, so the cursor's Drop assertions panic instead of returning `Err` when the stock runs out mid-operation.
