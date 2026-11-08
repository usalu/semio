# 1107 Audit Review: job, store/os, spr port diff

Lane: `aud-review-job`. Read-only review. No builds, no repo runs, no edits outside this folder.
Scope: `git -c core.fsmonitor=false diff HEAD -- 🧰️framework/🔨️modules/🧵️job 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr` (40 files, +342/-292), plus the untracked files the diff does not show (park crate manifest, `🧾️scope`, `🏛️source` clock, `🧪️testkit`, `🪙️effect-provider` test, `paged-one-item` manifest). The `🧪️testkit`, `effect-provider` test and `paged-one-item` manifest were not read in full.
Compile status: NOT compile-checked. Everything below is from reading code.

Base paths used below (abbreviated):
- J = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job`
- JC = J + `/⏱️context`
- S = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules`

## Ranked findings (highest likely runtime impact first)

### 1. Receiving scope spend accumulator is never written (live path, single-charge risk)
- Confidence: wiring HIGH (verified). Double-count impact MEDIUM (depends on each caller's semantic receipt; not verified per caller).
- Evidence: `J/🛬️receiving/🎟️state/🦀️.rs:45` builds `let mut child=Default::default();` and passes `&mut child` as `receiving_spent` into `OriginalJobConstructionRecipient::new / new_recovery`. `J/🛬️receiving/🪙️budget/🦀️.rs:28` (`accept_receipt`) adds each accepted construction progress to `*self.spent` (the physical `StepContext.retained_progress`, via `run_scoped_on_caller`) AND to `*self.receiving_spent` (the local `child`, discarded at return). The new `J/🧾️scope` type's `spent` field (`OriginalReceivingScope`) is therefore never incremented; it is only read (`spent()`) and taken (`settle`).
- Consequences:
  - `receiving_child_progress()` (`JC/🦀️.rs` ~281) is always zero.
  - `settle_receiving_scope()` (`JC/🦀️.rs` ~283) always debits zero, so a scope settled through it is never charged its semantic receipt.
  - `consume_receiving_retained(grant, progress)` (`JC/🦀️.rs` ~285) debits the full caller receipt. If that receipt includes construction spend already added to `retained_progress` through `spent`, the spend is charged twice. Live callers: `S/🌊️flow/🖥️host/📥️evaluation-source/🔣️receive/🦀️.rs:38`, `S/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️mounted/🦀️.rs:116`, `S/🔌️plugin/👷️job/📨️emission/🏛️host/🦀️.rs:104`.
  - The `open()` guard "still owes its enclosing semantic receipt" (`🧾️scope/🦀️.rs`) can never fire, because `spent` is always default.
- Pre-existing at HEAD (the `child` line is unchanged context in the diff). The port moved the dead accumulator into a type where the dead state is visible. Fix: pass `&mut self.receiving.spent` (or its equivalent) into the recipient so the scope records accepted spend, then make `settle` the only debit path.

### 2. Header close drops its receipt when the single-charge guard fails (live path, ledger desync)
- Confidence: HIGH on the code path. Impact only when an invariant breaks, which is why this ranks second.
- Evidence: `J/📬️outcome/⏸️continuation/🎟️source/📤️header/🦀️.rs:802` (`close_original_header_failure`). `with_original_effect` runs `close_header_failure_step`, which mutates publisher state (`failure_close.take()` or `failure_close=Some(..)`). Then:
  - `polled?` returns early on a closure refusal, discarding `performed`.
  - `if performed>turn.work {return Err(..)}` returns before `crate::receive_original_retained_work(cx,turn)`.
  The close receipt (`copied_items`, `copied_bytes`, release) and its work are never posted, but the publisher state already changed. This is a retained-stock desync on the error path.
- Fix: post `turn` (or a refusal carrying its progress) before returning on either error branch.

### 3. Park extraction widened privacy of StepContext turn state (latent, high blast radius)
- Confidence: HIGH on the exposure (verified). Runtime impact today: none, because `park_fields()` is called only from the park crate (grep over `🧰️framework`).
- Evidence: `JC/🦀️.rs` ~line 484-489 adds `pub struct ParkFields<'c>` with `pub` fields `fuel_remaining`, `retained`, `clock`, `payload_page_granted`, `stage`, `preview_sequence`, and `pub fn park_fields(&mut self)` on `StepContext`. Before the port these fields were private to the job-context crate root and only the in-crate park module could touch them. Any crate depending on job-context can now set the turn fuel or the retained grant, reset `payload_page_granted`, or rewrite `stage`. That bypasses the non-renewable wallet and the single-charge law.
- Also widened by the same extraction: `OriginalRetainedLoanIdentity` (`capture`/`matches` now `pub`, `JC/🦀️.rs` ~470-476).
- Fix: keep the mutable view `pub(crate)` by moving the park module back into job-context, or expose one narrow method per park operation instead of a bundle of `&mut` fields.

### 4. Owned-turn admission widened to `pub` with a caller-chosen constructor (latent)
- Confidence: HIGH on visibility. Impact MEDIUM-LOW now (only park calls it).
- Evidence: `J/⏱️context/🎟️source/🦀️.rs:76-77` (`admit_owned_turn`) changed from `pub(crate)` to `pub`, and takes `make: impl FnOnce(OriginalCallerSourceParent,&StepContext,usize,u64,OriginalSourceTurn)->T`. `into_source_loan` also changed from `pub(crate)` to `pub`. The previous code had one constructor (`from_admitted_source`, `pub(crate)`). A downstream caller can now wrap the paid source loan into an arbitrary type `T`. The `OriginalOwnedCallerReturn` Drop assert in park protects only the park type, so the loan can be stranded.
- Fix: keep `admit_owned_turn` `pub(crate)` or make `T` a sealed park-only trait.

### 5. Clock dispatch binding moved from the context to caller arguments (latent)
- Confidence: MEDIUM. Signatures verified; no misuse found in callers.
- Evidence: `J/⏱️context/🎟️source/🦀️.rs` ~line 204 (`recorded_clock`) returns `&OriginalSourceClock`. `J/🕰️clock-identity-and-budget/⏱️recorded/🏛️source/🦀️.rs` exposes `pub` methods that take the dispatch identity as an argument: `take_reading(turn)`, `take_refusal(turn)`, `cancel_unstarted(turn)`, `take_decision(turn)`, `finish_reading`, `finish_decision`, `cancel_unstarted_decision`. The old `recorded_clock_context(cx)` derived the turn from `cx`. Now a holder of the clock can name any turn identity.
- Fix: derive the turn inside each method from a context value, not from an argument.

### 6. Refusals swallowed into `None` / `false` in work recipients (low-medium)
- Confidence: MEDIUM (depends on how `OriginalWorkAuthority` treats a `None` source turn; not read in the value crate).
- Evidence:
  - `J/🛬️decode/🦀️.rs` `ActualWorkContext::work_turn` uses `.ok().flatten()`, and `admit_work` uses `.unwrap_or(false)`.
  - `S/🏪️store/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs` `MemberWorkCaller::work_turn` uses `.ok().flatten()`.
  A failed context lookup becomes "no source turn" or "not admitted" with no refusal recorded.
- Fix: return `Result` from `work_turn` and propagate it to the quote.

### 7. Work charged that the effect never performed (low, single debit still holds)
- Confidence: HIGH.
- Evidence: `J/📬️outcome/⏸️continuation/🎟️source/📤️header/🦀️.rs` `close_header_failure_step` branches `done(Ok(Complete(..)),1)` and `done(Ok(Progress(..)),1)` return work 1 without `effect.performed(1)`. `J/🕰️clock-identity-and-budget/⏱️recorded/🏛️source/🦀️.rs` `close`: `Ok(false)=>fail(refused(..),1)` charges one unit when the release was refused. No double charge: the one debit is `receive_original_retained_work`. The guard `performed<=work` still holds. But the refusal branch debits work that the effect explicitly did not perform.
- Fix: return work 0 on the refusal branch of `close`.

### 8. Escrow held capacity and release accounting edits are in an orphan file (no effect)
- Confidence: HIGH (no `mod` or `#[path]` references `📒️book` in the job tree, at HEAD or now).
- Evidence: `J/📬️outcome/⏸️continuation/🎟️source/📒️book/🦀️.rs` gained `held_release_bytes`, `held_capacity_bytes`, and `OriginalReturnEscrowAmounts{release_bytes:0,capacity_bytes:0}` in `claim_inner`, but nothing writes the new fields except the claim's zeros. The file is not compiled. The live budget `J/📬️outcome/🪙️budget/📦️stock/🦀️.rs:33-38` does track capacity and release on claim.
- Fix: delete the orphan `📒️book` directory (source and tests) or wire it in deliberately. Do not treat these edits as live.

### 9. Readiness predicate in `with_original_effect` is weaker than the receiving path (latent)
- Confidence: HIGH. No runtime impact today: the only consumer is a retirement step.
- Evidence: `JC/🦀️.rs` ~line 565 `ready = retirement || now<deadline` ignores cancellation and `should_yield` for non-retirement callers. `JC/🛬️owned/🦀️.rs` ~line 40 (`original_receiving_outcome`) also checks `!cancel.is_cancelled_now()`.
- Fix: mirror the receiving predicate for non-retirement operations before adding a second consumer.

### 10. Copy-byte asymmetry between work-issuer birth and close (low, pre-existing)
- Confidence: HIGH on code. Impact LOW.
- Evidence: `J/🛬️receiving/🎟️state/🦀️.rs` ~1012 charges `size_of::<OriginalWorkAuthority<..>>()` on birth. The close (~1067) and demand (~1058) use `size_of::<Option<OriginalWorkAuthority<..>>>()`. HEAD had the same pattern with `NativeDecodeWorkIssuer`. Each close over-charges the copy grant by the Option tag, and a tight close grant can keep the close refused.

### 11. Preflight allocation closure no longer adapts the request (compile risk, unverified)
- Confidence: MEDIUM that the type changed; not verified because nothing was compiled.
- Evidence: `S/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs:13` passes `allocation` directly to `NativeEncodeControl::new_forwarded` instead of `|request| allocation(request.bytes)`. This compiles only if the `allocation_stage` closure accepts the forwarded request type.

### 12. RetainedHistoryDecode no longer proves live work actions are empty (low)
- Confidence: MEDIUM.
- Evidence: `S/📡️spr/📜️history/🦀️.rs` removed `work_issuer` and its `has_pending_work()` term from `terminal_is_empty`. The work authority is now owned by the caller (`OriginalWorkAuthority` in the test driver). Safety now depends on that authority's own Drop assert.

## Checklist from the brief

- `with_original_effect` single charge: only one production call site (`close_original_header_failure`). The provider itself debits nothing, because `OriginalJobWorkRecipient` holds an immutable `&u64` snapshot. The single debit is `receive_original_retained_work(cx, turn)`, and `performed<=turn.work` is enforced. No double debit found. Unadmitted quote returns `Ok(None)` and charges nothing. See findings 2 and 7 for the refusal-path gaps.
- Job-park extraction privacy: findings 3, 4, 5.
- `OriginalReceivingScope` extraction: finding 1. Removed `native_receiving_requires_recovery`, `original_source_children_are_returned`, and the `poll_recorded_clock` family have no remaining callers in `🧰️framework` (grep).
- Escrow amounts held capacity and release: finding 8. `🪙️budget/📦️stock` is the live model and does track them.
- Retained stock leaks on close: no new leak in the decode or receiving close paths. The residual risks are findings 2 and 10.
- Refusals dropped: no refusal removed. `ValueError::invariant` is `literal(InvariantViolated, msg)` (`🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs:123`), so the 36 added and 35 removed `InvariantViolated` conversions are equivalent. Swallowed refusals are finding 6.
- `Pending` without progress: no new instances. The existing `Pending{progress:Default}` and `Progress(default),work:0` returns are the "not admitted this turn" convention.
- Stale `[DEBUG]`: none added. One `[DEBUG]` eprintln was removed (`S/🏪️store/⚙️engine/🧳️bundle/🦀️.rs`).
- `unwrap`/`expect` added outside tests: none in production code. One new `u64::try_from(..).unwrap()` in the `🧪️testing/♻️retirement` helper (`native_turn`, spr history). Low; it is a test helper.

## Not covered
- `🧪️testkit/🦀️.rs`, `🪙️effect-provider` test, `paged-one-item` manifest: not read in full.
- No compile, no tests, no repo runs (per brief).
