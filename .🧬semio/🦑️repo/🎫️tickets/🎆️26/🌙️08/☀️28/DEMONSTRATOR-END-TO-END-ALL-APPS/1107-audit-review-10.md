# 1107 Audit Review 10 (lane review-10)

Read-only audit of the three ported crates. No edits, no cargo, no git modifying commands, no sub-agents. Nothing was compiled or run; every finding is from reading source.

Crate roots (under `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/`):
- `🧬️snapshot-clone/` (lib `🧬️snapshot-clone/🦀️.rs`, 757 lines; plus `pending`, `custody`, `handoff`, `metadata`, `assembly`, `config-apply`, `law`)
- `🧵️operation-wire/` (lib `🧵️operation-wire/🦀️.rs`, plus `🧳️replacement`, `🧳️original`, `📤️edit`, `📜️history`, `📦️output`, `🎯️common`, `📝️text`)
- `🎮️one-item/📬️publication/` (lib `🦀️.rs`, 766 lines; `🪪️authority`, `📥️lifted/*`)

Scope note: I read in full `snapshot-clone/🦀️.rs`, `one-item/📬️publication/🦀️.rs`, `operation-wire/🧳️replacement/🦀️.rs`, `operation-wire/🧳️original/🦀️.rs`, and the helpers they call (`one-item/🦀️.rs` turn-work helper, `native_source::polled/pay`, job `StepContext` charge functions). Not read beyond grep: `snapshot-clone/{pending,custody,handoff,metadata,assembly,config-apply}`, `operation-wire/{🦀️ root body, edit, output, text, history, common, canonical}`, `one-item/📥️lifted/*`. Findings in those areas are absent, not cleared.

## Top finding

Pending native receive work is charged again on every repoll (double debit), and the replacement receiver never raises a typed refusal for a Pending child. See F1.

## Findings (ordered by severity)

### F1. Replacement receiver re-debits cumulative pending work on every repoll (single-charge violation)
- `🧵️operation-wire/🧳️replacement/🦀️.rs:53` accumulates `pending_work = pending_work.unwrap_or(0).saturating_add(work)` and returns `Ok(default)`.
- `:103` in `advance`: `if let Some(work)=self.pending_work { cx.consume_fuel(work); return Ok(false) }`. The field is only cleared on Complete (`:53`), so each repoll consumes the whole cumulative total again.
- Concrete failure: poll 1 returns `Pending{work:3}` and consumes 3. Poll 2 returns `Pending{work:2}`, so `pending_work=5` and the call consumes 5 again. Total fuel debited is 8 for 5 performed.
- The job already tracks this: `🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs:43` quotes from `owner.state.pending_work()`, and the job test `🔨️modules/🧵️job/📬️outcome/⏸️continuation/🧪️tests/📦️capsule/🦀️.rs:80` is named `actual_pending_work_keeps_its_original_claim_instead_of_charging_a_receiving_turn_every_quantum`. The replacement code contradicts that rule.
- The unit test at `replacement/🦀️.rs:137` asserts the cumulative value (5), so it locks in the bug rather than catching it.

### F2. Replacement receiver debits fuel by item count and by a fabricated minimum (not by performed work)
- `replacement/🦀️.rs:96`: `settled()` does `cx.consume_fuel(step.copied_items as u64)`. Copied items are discrete ownership actions, not work units. Settle already runs `settle_receiving_scope()` (`:94`), which debits retained progress, not fuel. This is a second, unrelated fuel debit unless the native scope already covers it. Not verified either way; treat as a probable extra charge.
- `replacement/🦀️.rs:119`: `cx.consume_fuel(performed.max(1))`. Forces at least one unit even when the effect performed 0. This is the fixed-work pattern the W1/W2 rules forbid. It also debits directly instead of through `receive_original_retained_work`.
- `replacement/🦀️.rs:47`: when an effect made progress with `performed_work()==0`, `pay(effect,1)` fabricates one unit.
- `snapshot-clone/🦀️.rs` `advance` (around the `coordinated` computation): when `reported==0`, `observed==0`, and the step has ownership progress, `original_one_item_turn_work` (`one-item/🦀️.rs:244`) returns `max(coordinated)`, so a turn that performed no work is still charged 1 unit.

### F3. Pending child reported as zero-progress success; Pending work discarded
- `one-item/📬️publication/🦀️.rs` does not define this; the helper is `one-item/📬️publication/📥️lifted/🧵️canonical-edit/🧳️source/🦀️.rs:26`: `polled()` maps `RetainedWorkPoll::Pending{..}` to `Ok(default)` and drops the `work`.
- Used in: `operation-wire/🧳️original/🦀️.rs:80` (advance, phase 3), `:111`, `:112` (close_step), and `replacement/🦀️.rs:118` (`close`, top-level).
- `snapshot-clone/🦀️.rs:519-523` `settled_step()`: `Pending => Ok(Progress(default))`. Used in `build_sealer` stage 1 (around the `settled_step(source.close_step(...))` call): `Ok(true)` is returned with zero progress, so the caller sees `Progress` while the close is still pending.
- `snapshot-clone/🦀️.rs` `build_sealer` stage 0 (`take_authority`): `Pending{..}` maps to `Pending(Default)` and the work is discarded with `..`. The turn returns `Ok(true)` with zero progress.
- Mitigation not verified: these are only benign if `effect.nested(...)` records the child's Pending work into the parent frame, so `effect.performed_work()` still sees it. I did not confirm this in the value crate.
- The runtime2 ruling says Pending on native receiving is "recovery-only (host recipients are synchronous): map to a typed refusal, never loop" (`C/1107-brief.md`, runtime2 section). `replacement/🦀️.rs:40` and `:52-53` instead treat Pending as normal idle retry.

### F4. Unfunded or unadmitted closes idle forever (W4 close liveness)
A close whose demand never fits the grant returns `Progress(default)` on every call, with no refusal and no `Blocked`. The job's own `close_original_admission` (`job/♻️retirement/🎟️context/🦀️.rs:~37-40`) refuses with WorkLimit when the turn is untouched and the demand exceeds the whole grant, so the one-item crates should do the same.
- `one-item/📬️publication/🦀️.rs`, `close_original_step` default body: `Ok(false) => settle(Progress(default))`.
- `one-item/📬️publication/🦀️.rs:~695-698`, `ArtifactStoreBatchSourceOf::close_ladder`: `demand.copy_bytes > grant.maximum_copy_bytes || ... => Progress(default)`.
- `one-item/📬️publication/🦀️.rs:~711`: `begin` returns `None` and the caller returns `Progress(default)`.
- `snapshot-clone/🦀️.rs` `close_ladder`: `if grant.maximum_items==0 { Progress(empty) }`, and the `copy_bytes` checks in `copied`, `inverse`, `authority` steps return `Progress(empty)` when the grant is below the size.
- `operation-wire/🧳️original/🦀️.rs:109`: `close_step` returns `Progress(default)` when the close demand is not admitted.
- `operation-wire/🧳️replacement/🦀️.rs:82`: `close_native` returns `Progress(default)` when `!permits(grant,d)`.
- Net effect: a grant that can never fit a single close step freezes the close with no visible cause.

### F5. Preparation refuses or blocks in ways that stall or kill publication
- `snapshot-clone/🦀️.rs` Clone phase (`advance_local`): a zero-progress, not-complete step is returned as `OwnershipLimit` "retained-clone.step-grant-too-small". The `advance` error branch then sets `self.cancelled=true` (`:~ "Err(error)=>{self.cancelled=true;..."`). A merely small grant therefore cancels the whole publication instead of waiting for a larger turn.
- `snapshot-clone` CloseClone and CloseEdit return `Blocked` when progress is zero and the step is not complete. `Blocked` is retried by the caller with the same grant, so this is the same idle-forever risk as F4.
- Chunk rather than cap: the clone and edit phases quote `work = 1` (`advance`: `let demand = if Edit {...} else {1}`), so each Clone turn gets 1 work unit. This is honest only if each turn does exactly one action. The ruling says chunk, not cap.

### F6. Hard-coded per-turn caps (chunk rather than cap; per-call 64-unit constants)
- `operation-wire/📝️text/🦀️.rs:51, 64, 92, 105`: `.min(64)` on every turn.
- `operation-wire/📦️output/🦀️.rs:42, 48, 74, 115`: `.min(64)`.
- `operation-wire/🦀️.rs:133, 179, 210`: `.min(64)`.
- `operation-wire/🎯️common/🦀️.rs:26`: `.min(64)` per segment fragment.
- `operation-wire/🧳️original/🦀️.rs:53`: `copy_bytes: cursor.next_minimum_copy_bytes().max(64)` and `depth: 64` hard-coded in `demands`.
- `snapshot-clone/🪪️metadata/🦀️.rs:25`: `.min(256)` on the actor copy quote, a lowered quote.
- Effect: a payload of N bytes needs about N/64 turns. Brief W2 forbids this shape.

### F7. Panics, asserts and unwrap/expect on reachable paths (AGENTS.md forbids unwrap/expect outside tests)
Asserts in Drop or on reachable calls:
- `one-item/📬️publication/🦀️.rs:339`: `record_outcome_messages` uses `assert!` on a caller-reachable path (double record or unready owner panics).
- Drop asserts: `operation-wire/🧳️replacement/🦀️.rs:127`, `operation-wire/🧳️original/🦀️.rs:119`, `snapshot-clone/📦️assembly/🦀️.rs:104`, `snapshot-clone/♻️custody/🦀️.rs:33`, `snapshot-clone/🚚️handoff/🦀️.rs:64`. A value dropped before close panics. Reachable on cancel or error paths unless every owner is always closed first; not verified.

Expect/unreachable/unwrap on paths that look reachable:
- `one-item/📬️publication/🦀️.rs:658-667` (`begin_next`): `expect("observed original batch item authority")`, `expect("observed original batch item input")`, and `unwrap_or_else(|| unreachable!(...))` on `checked_add` of domain-returned `copied_bytes`. An overflow from a domain-reported progress value panics. `:682` and `:718` also `expect` in `retirement_demands` and `close_ladder`.
- `snapshot-clone/🦀️.rs:491`: `self.edit_cursor.as_mut().unwrap().close_step(...)` in CloseEdit.
- `snapshot-clone/🦀️.rs` `seal_turn`: `self.assembly.sealer.as_ref().unwrap()` twice.
- `snapshot-clone/🦀️.rs:664, 679, 732, 743` (`close_ladder`): `take().unwrap()` after is_some checks; safe as long as the guards hold.
- `snapshot-clone/📦️assembly/🦀️.rs:51-54` and `snapshot-clone/🪪️metadata/🦀️.rs:51-80`: many `as_mut().unwrap()`.
- `snapshot-clone/♻️custody/🦀️.rs:8-9, 30`: `expect` in accessors.
- `snapshot-clone/♻️pending/🦀️.rs:24`: `ControlledRetirement::new(value.take().unwrap())`.
- `snapshot-clone/🚚️handoff/🦀️.rs:31`: `expect("validated retained clone cursor...")`.
- `operation-wire/🧳️replacement/🦀️.rs:30`: `receiver.as_ref().unwrap()` after a None check.
- `operation-wire/🧳️replacement/🦀️.rs:103`, `🧵️operation-wire/📦️output/🦀️.rs:185` and others: unwraps in production bodies (not all checked individually).
- `one-item/🪪️authority/🦀️.rs:19`: `unwrap_or_else(|_| unreachable!(...))` on `ControlledRetirement::new` of a tuple. Safe only while `controlled_retirement_supported()` is true, which it is for this type.

Authority bypass:
- `one-item/📬️publication/🦀️.rs` `ArtifactStoreOneItemLiveAuthority::mint(...)` is `pub` with no witness. Only `mint_for_store` takes a `StoreMintWitness`, and the doc says a type that implements it elsewhere "fabricates a publication authority and is a review finding". Any domain can call the unwitnessed `mint`.

### F8. Recovery trait methods in snapshot-clone (the stub check)
- `snapshot-clone/🦀️.rs:554-557`: `has_pending_receiving_recovery -> false`, `pending_receiving_demand -> None`, `pending_receiving_recovery -> None`, and `finish_receiving_recovery` returns `Err(InvariantViolated)` with `work:0`.
- Correction to the stubs audit (S2): the current file does not return `Ok(Default)`. `finish_receiving_recovery` is an explicit invariant refusal. It is still dead by contract, so the risk is low. The stubs audit's D1 (replacement Pending as `Ok(default)`) is reworked in the current tree (`settle_receive`), but the risk moved to F1.
- Unresolved: the Seal phase (`advance` with `phase==Seal`) runs `seal_turn` without `receiving.with_effect`. Its work is read from `actual_work()` delta and `sealer.last_turn_work()`. Not confirmed that the native seal commit is charged once.

### F9. Silent discards in snapshot-clone
- `snapshot-clone/🦀️.rs` `CaptureForeignStepPresence`: `let _ = cursor.begin_close();` discards the bool that signals whether the close began. A false result is ignored, and the phase advances to CloseEdit.
- `snapshot-clone/🦀️.rs` `handoff_clone_cursor`: `begin_close_paid(...).result` drops the turn's work field.
- `snapshot-clone/🦀️.rs` `terminal_is_empty` lists `pending_base`, `pending_mutation`, `edit`, `pending_mutation_close`, and others twice. Harmless, but it signals a rushed port.

### F10. Feature flags and production manifests
- `snapshot-clone` Cargo.toml: `law = ["semio-framework-job/testkit"]` is an optional feature. Its only dependent in the repo I checked (`plugin/🪟️window/🎚️config/🧬️preparation/🔁️replace/Cargo.toml:29`) enables it under `[dev-dependencies]`. No testkit in `[dependencies]` of the three crates. Clean.
- `one-item` Cargo.toml: `testkit = []` and `durable-group-testing = []` are not default. Test-only seams are gated by `cfg(any(test, feature="testkit"))` (`one-item/📬️publication/🦀️.rs` around lines 286-300 and 330-335).
- `store/🧪️testing/🎭️doubles/Cargo.toml:39` enables `semio-os-store-one-item-publication` with `features = ["testkit"]` under `[dependencies]`. Doubles is a test-support crate, so this is probably intended. Whether any production crate depends on doubles was not verified.
- `snapshot-clone` `pub use law::...` is gated by `feature="law"` (`snapshot-clone/🦀️.rs` lines 26-27). Fine.

### F11. Smaller items
- `snapshot-clone` `advance` returns `Blocked` when `cancelled || closing || !grant.permits_one()` with no refusal. A cancelled or closing preparation returns Blocked forever. Correct only if the caller always proceeds to close.
- `operation-wire/🧳️replacement/🦀️.rs:127` assert in Drop and `:34`, `:35` `Ok(default)` when `maximum_work==0` (unfunded) are both idle paths.
- `operation-wire/🧳️original/🦀️.rs:74`: `self.pending[self.offset..self.length]` panics if the cursor reports `written_bytes > 64`. The cursor's bound was not verified.

## What I did not check

- The value crate's `RetirementPollExit::settle` (does Progress with no work pay 1 unit?) and `effect.nested` (does child Pending work land in the parent frame?). F3 and F4 depend on both.
- `NativeDecodeControl::with_effect` and `receive_effect` signatures, and what `native_scope` already debits (F2).
- `one-item/📥️lifted/*` (canonical-edit 463 lines, receiving binary 166 lines, target), the operation-wire root body, `edit`, `output`, `text`, `history`, `common`, `canonical`, and the snapshot `pending`, `custody`, `handoff`, `metadata`, `assembly`, `config-apply` modules beyond grep hits.
- No test was run, no cargo, no compile check. Nothing here is a proven failure; each item is a source-level claim with the file:line cited.
