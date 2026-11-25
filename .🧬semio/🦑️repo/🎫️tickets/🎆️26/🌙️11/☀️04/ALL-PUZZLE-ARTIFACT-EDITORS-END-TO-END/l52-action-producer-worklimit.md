# L52: 3D Action-Producer WorkLimit ("original effect performed more work than its ceiling")

This is source analysis only. Nothing was compiled or run, because the brief forbids builds while the native batch runs. The fix is UNVERIFIED.

Input: `🗑️generated/l51-3d-action-producer-failures.txt`. Both tests fault with `interactive-job.worker-continuation`, message `WorkLimit: original effect performed more work than its ceiling`.

## 1. Where the message comes from

The text appears in exactly two places in the framework, both in `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`:
- `StepContext::with_original_effect_at`
- `OriginalRetirementTurn::with_effect`

It fires when `work.commit_for_action(performed, ..)` rejects a receipt whose observed work is larger than the ceiling that `prepare_for_action(maximum)` quoted.

The fault code is `interactive-job.worker-continuation`. That code is assigned only in `🔄️full-operation/🦀️.rs`, from the checked-out continuation of the typed-operation worker job. So the effect that overran is a `cx.with_original_effect(..)` inside the worker step. It is not the store publication. The publication and its wide seal window run in `typed-operation-unit` → `advance_apply_batch` → `NativeDecodeControl::with_effect`, on the main context, and their failures would surface as `VcsError`/`PluginLifecycleFailure`, never as this continuation fault.

## 2. The suspects from the brief are not the cause

### Wide seal window (l43 §4)
The bundle runs the window through `NativeDecodeControl::with_effect(grant, wide_work, ..)`. Every byte is bounded:
- `invoke_original` reborrows the effect to exactly `wide_work`.
- `advance_window` hashes at most `min(effect remaining, window − last_work, copy left)` bytes.
- It starts a tree run only while 256 work units remain in both the window and the effect.
- Tree runs go through `effect.nested(..)`, which is capped at the remaining work.

So performed ≤ quoted, and `canonical_fields_settle` owes nothing extra. The window also never runs in a worker StepContext.

### Other suspects
- `store_unit_run`: each unit is admitted by the receiving scope's own loan.
- l46 two-step publication, l47 `birth_mounted_window` and backbone `Arc::into_inner`, l42 clipboard slot: none of them changes how any `StepContext` effect sets its ceiling.

## 3. Root cause: the effect extent was capped one unit below the fuel that callers admit against

`git diff HEAD` of `⏱️context/🦀️.rs` shows exactly one change. The MIT-BESTAND ticket applied it on 2026-11-24 15:50 PST as an "unverified candidate", described in `🔬️work-limit-return.md`:

```rust
-        let maximum=self.fuel_remaining;let fuel=self.fuel_remaining;…
+        let maximum=self.fuel_remaining.saturating_sub(semio_framework_value_control_contract::ORIGINAL_INVOCATION_WORK);let fuel=self.fuel_remaining;…
```

`maximum` is used twice: as the action ceiling (`prepare_for_action(maximum)`) and as the effect extent (`RetainedAllocationEffect::new(grant, maximum, ..)`). So both became `fuel − 1`.

### Why that overruns
Framework callers admit a step against the full fuel and then run it inside `with_original_effect`. For example:
- `📡️spr/🧵️channel/🧩️command-decode/🎟️original` `close_original_step` and `🧾️command-owner` (the framework-declared action producer's command cursor and command cleanup): `if cx.fuel_remaining()<work {return Ok(false)}`, then `close_controlled` → `cx.with_original_effect(grant, |_, effect| owner.step(..))`.
- Many more sites, such as hydration, `space-members-macro`, `🫴️record` and `🤝️group`, follow the same shape.

Owners record performed work before they validate it. `RetainedAllocationEffect::performed` and `OriginalWorkSink::performed` add the delta first and only then answer whether it fits. So when a turn reaches the unit where `fuel_remaining == work` (always some last unit of a fuel-bounded worker step, `step_fuel = min(fuel, max_work_units_per_step)`), the sequence is:
1. The caller admits the step.
2. The effect extent is `work − 1`.
3. The owner records `work`.
4. `commit_for_action` sees `work > work − 1` and returns the hard `WorkLimit`, which reaches the host as the worker-continuation fault.

Before the cap, extent = ceiling = `fuel_remaining`, so the same unit fit exactly. That explains why `target_volume_action…` passed in l41 and fails now.

### The cap's premise is false
The cap assumed that `with_original_invocation` pays a one-unit handoff on top of the closure's work. It does not:
- `🎟️control/🧬️contract/🏭️producer/🦀️.rs` documents "it charges nothing beyond the call's own work".
- Its `InvocationProducer::receive` only calls the closure.
- The engine's `drive` and `transfer_reply` charge no work.
- `performed` is read from the receipt before `claim`, so it is exactly the closure's work.

The `+ORIGINAL_INVOCATION_WORK` widening exists only on the `NativeDecodeControl` path (`invocation_extent` + `invoke_original`). There the action is opened one unit wider and the operation is narrowed to the caller's quote, so the quote still covers exactly the work performed. `StepContext` has no such handoff, so the subtraction only made its ceiling smaller than its callers' admission.

## 4. Fix (applied)

`🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`, `StepContext::with_original_effect_at`:

```rust
let fuel=self.fuel_remaining;let maximum=fuel;…
```

- The Work ceiling, the effect extent and the fuel every caller admits against are the same number again. A step admitted at `fuel_remaining >= work` therefore always fits its quote exactly.
- This is now consistent with `OriginalRetirementTurn::with_effect` (`maximum=self.fuel`).
- The docstring now states this invariant and links `with_original_invocation`.
- The seal, digest and store-run code is unchanged, so the wide-window throughput gain of l43 is kept.

## 5. Effect on the MIT-BESTAND investigation

Their Koordinator/Bearbeiten `native-return-refused: authority (work-limit)` is still not root-caused. Their own note already marks the cap as unverified. Static reading shows that an effect cannot report `fuel + 1` through `with_original_invocation`, so the cap could not have fixed that overrun. Their decisive next probe, the `first_work_refusal` line in `return-escrow`, still applies. I did not edit their ticket.

## 6. Next run
- `-p semio-s-artifact-puzzle-3d-editor --test editor-action-producer`: both tests.
- `-p semio-framework-job-context`: the context laws.

## Files changed
- `/home/deck/semio/🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` (`with_original_effect_at`: ceiling and extent = remaining fuel; docstring)
- `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/ALL-PUZZLE-ARTIFACT-EDITORS-END-TO-END/l52-action-producer-worklimit.md` (this report)
