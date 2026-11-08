# 🔎️ 1107 Audit review-12: semio-os-store-replay-operation and semio-os-store-paged-one-item

Read-only audit, lane `review-12`. No builds, no tests run, no git, no edits. Sources read: `1107-brief.md`, `1107-result-store3.md`, `1107-result-paged.md`, and the crate sources below.

Base `B` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`
- replay-operation (`Cargo.toml` name `semio-os-store-replay-operation`): crate root `B/🔁️replay/🎮️operation/🦀️.rs`, plus `🧬️native/🦀️.rs`, `🧬️native/🔌️drivers/🦀️.rs`, `🌱️derived/🦀️.rs`, `🧬️native/🔗️input/🦀️.rs`, `🧬️native/🔗️history/🦀️.rs`.
- paged-one-item (`semio-os-store-paged-one-item`): `B/📬️paged-one-item/🦀️.rs`, `❄️cold-apply/🦀️.rs`.
- Shared helper read: `B/🎮️one-item/🦀️.rs:244-247`.

## Top finding

`🧬️native/🔌️drivers/🦀️.rs:74` (`SemanticEdit::advance` for the cloned edit) commits `receiving.charge(d.capacity_bytes)` before `probe_effect` runs. When the probe returns `None` (quote not admitted, fuel below 8), the function returns `Progress(default)` and nothing refunds the charge. Every retried, not-admitted turn adds `d.capacity_bytes` to the ownership ledger. Eventually `decode/🦀️.rs:173` (`charge`, `next <= maximum_bytes`) refuses with `OwnershipLimit` on a small, legitimate operation. The sibling path at `🧬️native/🦀️.rs:129-133` does the same charge inside the probe closure, so it is correct; only the drivers path is wrong.

## Findings, ranked

### 1. Phantom ownership charge on not-admitted turns (high, drivers.rs:74)
- Line 74 order: `retained_clone_grant_admits` check, then `receiving.admit_turn_capacity(...)`, then `receiving.charge(d.capacity_bytes)`, then `probe_effect(...)`, then `let Some((result,work))=turn else {return Ok(Progress(default))}`.
- Failure: a fuel-starved turn with `d.capacity_bytes != 0` leaves the ledger charged with no allocation. Repeated Blocked turns accumulate the charge until `OwnershipLimit`.
- Fix: move the admit/charge into the probe closure, as `🧬️native/🦀️.rs:~143-150` already does.

### 2. Panic on a reachable placement refusal (high, drivers.rs:24)
- `take()` runs `let placed=turn.result.unwrap_or_else(|_|panic!("sealed original projection placement changed its admitted demand"))`.
- Failure: `output.place(...)` can return `Err` for allocation or ownership refusal, or for a placement that differs from the quote. That panics instead of returning a typed `ValueError`. Reached via native `Stage::TakeProjection` (`🧬️native/🦀️.rs`, `advance_effect`).
- Fix: `turn.result?`, which the surrounding `pay(effect,turn.work)?` already matches.

### 3. Close liveness: unfunded closes report Progress and stay Pending (W4 violation)
- `🎮️operation/🦀️.rs:38` (`OperationMessageSettlement::close_step`): `grant.maximum_*` or `next_*_demand` not admitted returns `Progress(idle)`. No Blocked, no refusal.
- `🎮️operation/🦀️.rs:221` (`ArtifactReplayPreparationRetirement::close_step`): unfunded returns `Progress(empty)`. Line 220 passes `grant.maximum_copy_bytes` as the `body` argument to `next_demand(body)`. That is a semantic mismatch: the close quote is sized by a byte budget, not a body length. Unverified whether it matters, but if a close demand scales with `body`, a close can be permanently unfunded.
- `🌱️derived/🦀️.rs:116` (`step_task`) and `step_turn` (`!funded(grant,demand)` returns `Progress(idle)`). `Exit::settle` then maps `Pending` to no progress. A close whose demand never fits the grant stays Pending forever, with no typed refusal.
- `🎮️operation/🦀️.rs:48` (`close_cold`): `while !terminal_is_empty() { close_step(...).expect(...) }`. The loop has no progress guard. If a pending child returns Progress with zero progress, this spins forever; any `Err` panics.
- Inconsistency: `🧬️native/🦀️.rs:129-133` returns `Blocked` for the same not-admitted quote that `drivers.rs:74` returns as `Progress(default)`.

### 4. Probe ceiling: bounded, but has a livelock shape and a hard-failure edge
- Bounded: `🧬️native/🦀️.rs:76-82` walks 2^20, 2^18, ..., 8 (about 11 quotes), then returns `None`. Paged: `📬️paged-one-item/🦀️.rs:874-895`, with `PAGED_TURN_WORK_PROBE`/`FLOOR` at lines 43-44 and an `asked==Some(work)` guard at 881. Neither loops.
- Undercharge: no. Both charge only `performed` from the receipt, and `run_effect_turn` (paged 553-559) pays a minimum of 1 for progress with zero performed work.
- Livelock shape (latent): `probe_effect` backs off only on `None`. If the closure itself returns `Blocked` at the 2^20 ceiling (because `advance_effect` recomputes `demands(body, maximum_work_units)` at `🧬️native/🦀️.rs:~146`), the probe returns `Some(Blocked)` and never tries a smaller ceiling. The only ceiling-dependent demand is `SemanticProjection::demands(.., maximum_work_units)`: `drivers.rs:20` and `:39` pass it through (`PagedProjection`, `ClonedProjection`), and `cold-apply` ignores it at `❄️cold-apply/🦀️.rs:402`. A work-sized projection would livelock. Unverified for the snapshot projections behind `drivers.rs:20/39`.
- Hard-failure edge: a single indivisible action whose performed work exceeds the admitted ceiling fails with `WorkLimit` (`one()`/`pay()`) instead of backing off. Examples: `drivers.rs:24` `pay(effect, turn.work)`, and `🧬️native/🦀️.rs` `InverseBirth` `pay(effect, turn.work)`. Not a silent loss, but a legitimately large single step is a hard failure. Probe restarts at 2^20 every turn, so a fuel-starved job pays about 11 quotes per turn.

### 5. Typed-refusal stub on a reachable path (likely, not verified)
- `🧬️native/🦀️.rs` `advance_effect`, `Stage::Edit` arm: `post_read.prepare(...).result?` returning `None` becomes `invalid("semantic edit stage runs on its own original actions")`. `None` is how `with_effect`/`prepare` signal "not admitted" elsewhere, so this should be `Blocked`. Reached when `post_read.demand(..)` is `Some` (so the probe path is taken) and the funded prepare is not admitted. Verify `OriginalPostRead::prepare` `None` semantics in `value` before relying on this.

### 6. Work reconciliation on the direct path (risk, unverified)
- Paged `advance` (`📬️paged-one-item/🦀️.rs:~897-905`): `reported = last_work + coordinated` (`record()` counts 1 per progress, plus sealer/edit work), `observed` = `receiving.actual_work()` delta across `advance_direct`. `B/🎮️one-item/🦀️.rs:245-246` errors `InvariantViolated` when `reported != 0 && observed > reported`.
- Risk: `Phase::Sealing` (`phase_work_demand` returns `None`, paged ~line 793) runs `advance_sealer` (paged 523), and `sealer.last_turn_work()` may not cover every nested probe. If it does not, a legitimate direct turn fails. Not established either way.

### 7. Panics and expects on reachable paths (AGENTS violations, mostly invariants)
- `🎮️operation/🦀️.rs:22` and `:26`: `expect` in `OperationMessageSettlement::step` and `take`, reachable if called after the drain is consumed.
- `🎮️operation/🦀️.rs:50` (`Drop` for `OperationMessageSettlement`) and `:245-248` (`Drop` for `ArtifactReplayPreparationRetirement`): `assert!` on drop of an unsettled owner, guarded only by `panicking()`. Any early drop on a non-panic path aborts.
- `🧬️native/🦀️.rs` `advance_edit`/`advance_effect`: `self.source.as_ref().unwrap()`, `self.input.as_ref().unwrap()`, `self.messages.as_mut().unwrap()[0]`, `self.refusal.unwrap()`, `self.mutations.as_ref().unwrap()`. Invariants, but unwraps.
- `📬️paged-one-item/🦀️.rs:511`: `checked_add(1).expect(...)`. Overflow unreachable in practice.

### 8. Accessor that always reports zero (low)
- `🎮️operation/🦀️.rs:199, 206, 211`: `ArtifactReplayPreparationRetirement::last_work` is never assigned; `last_turn_work()` is always 0 even though `close_step` (214-237) does real work via `effect`. No caller found in the audited crates. Any future caller that charges it would undercharge.

### 9. Double or missing charges: none found in the audited paths
- Derived non-task path: outer `probe_effect` (`🌱️derived/🦀️.rs:110`) wraps `step_turn`. Task path (`step_task`, line 117) bypasses it, so there is no nesting. Native `advance` and `advance_edit` do not wrap an outer effect around the edit's own probe.
- Replay cancel and owned-state `charged(effect, owner.last_turn_work(), ..)` (`🛑️cancel/🦀️.rs:118/125`, `⏩️replay-owned-state-operations/🦀️.rs:68`): these are outside the two crates; the grant-shaped `OperationMessageSettlement::close_step` is charged there, and its `last_work` is set on every branch.
- `phase_work_demand` adds `+2` "parent" work to every quote (paged ~line 795). Reported work is `performed`, so the quote and the charge differ by design. Not a double charge.

### 10. Quadratic replay over history length (not established)
- Each operation does `ledger.retained_read(key)` / `retained_read_demands(key)` (`🧬️native/🔗️history/🦀️.rs:19-27`), which is per key, not a scan. `🌱️derived/🧩️store/🦀️.rs:26` and `:35` clone `source.registry` per operation; its size scales with the registry, not verified. Ledger internals were not read, so a quadratic cost cannot be ruled in or out.

## Skipped or unsure
- No cargo, no tests, no `repo` commands, no git (per brief).
- Not read: `value` crate `with_effect`, `OriginalPostRead::prepare`, `RetainedAllocationEffect` semantics (findings 1, 4 and 5 depend on them), the ledger internals behind `retained_read`, `cold-apply`'s clone and mutation logic beyond the projection, `mutation-apply`, `space-history`, test files, and the `Drop` at `🧬️native/🦀️.rs:272`.
- Line numbers marked `~` are from context, not grep-pinned.
- The brief's "work ceiling probe" is bounded and does not undercharge. Its real problems are the livelock shape (4), the hard `WorkLimit` edge (4), and the probe-adjacent phantom charge (1).
