# 🔎 1107 Audit review-13 (read-only)

Scope: `git diff HEAD` of the work-accounting, retirement, close, pack JSON, fold and JPEG/image changes listed in `1107-brief.md`, `1107-result-history2.md`, `1107-result-pixels.md`. No cargo, no edits, no modifying git commands. Nothing was executed or compiled; all findings are from reading the diff and the working tree. Paths are relative to the repo root `/Users/ueli/Documents/semio`, and `VAL` below means `🧰️framework/🔨️modules/🌱️value`.

## Findings (ordered by severity)

1. **`ControlledRetirement::abandon()` can panic instead of releasing, and it is callable by anyone.** `VAL/♻️retirement/🎮️controlled/🦀️.rs:100`. `abandon` drops `cursors` through `ManuallyDrop::drop`, which runs each `Box<dyn RetirementCursor>` destructor. `RetirementRecord` has its own `Drop` that asserts `panicking || terminal_is_empty()` (`VAL/♻️retirement/🧱️record/🦀️.rs:40`). A stalled close with a non-terminal record cursor therefore panics inside `abandon`, which is exactly the path the image lane uses to convert a stall into `Invalid("Decode job close stalled")` (`🔲️pixels/🖼️image/📥️decode/🦀️.rs:103`). The doc comment says "never leaks", but the drop bypasses only the top-level assert, not the nested ones. Also, `pub fn abandon` and `#[doc(hidden)] pub fn retire_funded` have no capability gate, so any crate can drop a live owner without its close protocol. Unverified by run.

2. **Protocol refusal can never drain, so `has_pending_close` can stay true forever.** `take_failure_protocol_refusal` was deleted from `VAL/🛬️decode/🫴️recipient/🦀️.rs` with no replacement and no caller left at HEAD or in the tree. Its setter is `VAL/🛬️decode/♻️retirement/🦀️.rs:50` (`recipient.failure_protocol_refusal=Some(error)` when `retain_protocol_refusal` fails). The flag is still read by `pending_failure()`, which feeds `has_pending_close()` (`recipient/🦀️.rs:26`), `terminal_is_empty()` (line 85), and `demands()`/`step_result()` in `🛍️products/💻️os/.../📡️spr/📜️history/🦀️.rs` (lines 331, 350, 391). Once the flag is set, the close never reaches terminal. This is a W4 liveness hole. The removal in this lane made it visible (HEAD had the same untaken state, but nothing else depended on it). Unverified by run.

3. **Debug print left in a hot path.** `VAL/♻️retirement/🧱️record/🦀️.rs:30`: `next_demand` calls `eprintln!("[DEBUG] record demand ...")` through `inspect_err`. It violates the AGENTS rule that temporary logs are removed before finishing, and it runs on every demand query.

4. **Fold admission can be funded from copy it does not hold.** `🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs:303`: `next_step_grant` sets `maximum_copy_bytes = max(demand, logical).max(pending_original_copy)`. `pending_original_copy` is a reservation, while admission (`:200`) gates on `original_copy >= copy`. The lent grant can exceed the physical copy that admission needs, so the step can spend it and leave admission `Pending` forever, or it can over-lend. Reasoned only, not run.

5. **Fold drop-path panic.** `🔁️fold/🦀️.rs:110-114`: `retire()` runs `self.charged(...)` and panics with "admitted fold owner has no lent effect" if the slot is not lent. `HistoryFoldAdmission::drop` (`:216`) calls `retire(value)`. A pending admission dropped outside a lent turn, for example on cancellation between turns, panics in `Drop`. Medium confidence: this depends on whether `with_effect` returns `Err` when unlent, which the brief says it does, but I did not confirm the cancellation path.

6. **JPEG work is charged after the work has already run.** `🔲️pixels/📸️jpeg/📥️decode/🦀️.rs:44`: `advance` sums per-action work and calls `effect.performed(charged)` once at the end of the turn. The loop only checks `charged>=budget` before each action, so a single action (for example a long entropy segment) can overrun the authority before the refusal. `unwrap_or(false)` also turns an invariant error from `performed` into `WorkLimit`. The `take_result` path has the same `unwrap_or` pattern.

7. **Pack JSON cursor: the wallet stock depends on input length, and the break condition is unproven.** `🎒️pack/🔤️json/🧮️parser-and-grammar-cursor/🦀️.rs`: the stock is `nodes*64` work with `nodes = len+16` (around line 56 onward). If the grammar does more than 64 units per input unit, a valid parse fails with a spurious `WorkLimit`. The `if effect.actual().is_some(){break;}` at line 140 is a sound per-attempt guard. `charged()` (line 56) skips the ledger under a forwarded control, which is correct only if the forwarded control always runs inside a wallet or effect; I did not confirm that for every caller.

8. **Derive enum wrapper: no double count found by reading.** The enum wrapper forwards `next_birth_bytes`, `next_close_byte_demand`, `next_depth_demand`, `next_work_byte_demand`, `next_selected_work_byte_demand` and `allows_admitted_narrow_work` to the active variant (derive `🧬️retained-clone/🦀️.rs`, forwarding hunk). The child is now lent `effect.nested(...)` or `nested_within(..)`, and `nested` measures from its own `work_start` (`VAL/🎟️control/🧬️contract/📦️effect/🦀️.rs`), so the child's work is recorded once on the shared receipt. Parent code also sums `turn.work` locally, and the outcome returns that sum. This is consistent only if the children's `turn.work` equals their receipt delta. Constants such as `work+=3` in the derive bypass the receipt, so the returned work and the receipt can differ. The brief's single-charge rule (fuel debited == `turn.work`) still holds. I did not trace every derived variant.

9. **`ControlledRetirement` cursor grant widened.** `VAL/♻️retirement/🎮️controlled/🦀️.rs`: the child grant changed from `maximum_items: 1` to `maximum_items: grant.maximum_items`. Children may now copy more items per turn than before. Intentional per the W3 "runs" ruling, but not stated in the brief.

10. **Pack/JSON and fold `charged`/`owe` pairs.** `fold/🦀️.rs` `charged()` records `reported.max(done)` on the shared counter and owes only `reported-done`. This is correct only if `done` (receipt delta) is never larger than `reported` by design. I did not verify the `close_one` pop path (`owe(1)`) for a lent effect.

## What I checked and found OK
- `RetainedAllocationEffect::nested` / `nested_within` do not widen authority (`restricted_by`, min with remaining work).
- `ValueRetirement` forwards the new trait methods to the active variant; no duplicate forwarding found.
- `has_pending_close` excludes idle spare failure custody, as its doc says.
- `ControlledRetirement` `Drop` still asserts terminal-empty for ordinary drops; `abandon` is the only bypass.

## Not done
- No cargo, no tests, no runtime checks. Every law the lanes claim is unverified.
- Did not trace the 31 other derived owners or the `#[doc(hidden)]` callers of `retire_funded`.
- Did not read the full pack JSON cursor or the fold `close_one` caller graph.
- Searches ran against the working tree; one broad `git grep` was stopped (bd3ot7o8f) because it was too slow.

Top finding: `ControlledRetirement::abandon()` (`VAL/♻️retirement/🎮️controlled/🦀️.rs:100`) panics through nested `RetirementRecord::drop` asserts (`VAL/♻️retirement/🧱️record/🦀️.rs:40`) on the stalled-close path it was added to fix.
