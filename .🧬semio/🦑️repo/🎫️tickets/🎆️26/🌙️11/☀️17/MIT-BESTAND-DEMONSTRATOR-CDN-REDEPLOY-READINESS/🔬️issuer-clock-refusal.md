# 🔬️ Issuer clock/deadline refusal at `poll_wit` (generator and demonstrator boot)

## Symptom

`[DEBUG] poll_wit authority refusal epoch=N kind=InvariantViolated ... "original actor context changed its issuer clock/deadline or renewed work"`, then `setContributions command failed procedural original-turn.native-return-refused: authority` (procedural) and `interactive-job.publication-stalled` (demonstrator). Reproduced deterministically in the browser pane on the built site (procedural `#generator`, always epoch 2429).

## Method

A patched copy of the procedural `🌉️bridge.js` (`issuer-clock-probe-bridge.js`, served through `issuer-clock-overlay-serve-k.mjs`, no builds) logged every issue/receive and each poll's `fuelUsed`, receipt work and `remainingFuel`, and let the receipt work be inflated for a chosen epoch.

## Findings

1. Clock binding and absolute deadline are constant on the host: every issued turn had `cb=1` and the same `d=...` (`OriginalOperationSource.issue`, `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts:58`). The refusal is the third arm of `validate_currency` (`🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs:48`): the next epoch's `work.fuel` exceeded `self.work.fuel - fresh_work()`, i.e. the issuer "renewed work".
2. The host subtracts exactly the receipt's `work_units_spent` (`🟦️.ts:59`, fed from `result.retainedReceipt.workUnitsSpent` at `🌉️component-bridge-source/🟦️.ts:237`). The refused poll (2429) followed poll 2427 whose receipt said 27 (`fuelUsed` 28). Reporting `rw+1` still failed, `rw+2` passed, and `+2` on epoch 2427 alone made the whole boot pass the refusal. So the guest's own context counted exactly 2 more work units for epoch 2427 than its receipt.
3. Source of the 2 units: `StepContext` has a business view `original_work_units_spent()` (= `work_units_spent - birth_work`, `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:518`) used for every receipt (`dispatch_step`, `poll_kernel_turn`), because `exclude_original_birth` (`🦀️.rs:520`, called by the native receiving scope admission at `🧵️job/⏱️context/🛬️owned/🦀️.rs:134`) moves framework scope-admission work out of it. The `Drop` of `StepContext` (`🦀️.rs:134`) nevertheless posted the raw `work_units_spent` (birth work included) into the external work recipient, which is the actor context's `work_units_spent` that `fresh_work()` reads. Receipt and context diverged by the excluded work, and the next epoch's fuel check refused. The poll after the 47 KB command page admits the receiving scope, which is why it hits epoch 2427 and not the cheap earlier polls. `birth_work` and `exclude_original_birth` arrived with the latest commit (`508f457ff1e`).

The host (TS) is correct; no TS change.

## Fix

`🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` `Drop for StepContext`: post `self.original_work_units_spent()` into the external work recipient instead of the raw `self.work_units_spent`.

## Test

`work_recipient_posts_the_business_view_without_resident_birth_work` in `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/retained-charge.rs:230` (binary `original-native-progress`). It binds the physical, work and reserved recipients, spends 3 business plus 2 birth-excluded units and expects the recipient to gain 3.

- Red before the fix: `left: 10, right: 8` (recipient gained 5).
- Green after: `repo cargo --store=shared --configuration=prototype -- test -j 2 --manifest-path '🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/Cargo.toml' -p semio-framework-job-context --test original-native-progress work_recipient_posts` -> 1 passed.
- Caveat: the other three fixture-closing tests of that binary (`a_thousand_receiving_turns...`, `actual_original_progress_observer...`, `retained_charge_matches_original_quote...`) currently fail on the heap-witness assertion (`(0,0)` observed), independent of this change: the binary has no installed `HeapWitness` allocator while `🌱️value/🧪️testing` is being edited by another session. The new test therefore does not close its fixture (`mem::forget`), so it does not depend on that harness. No language-agnostic fixture row exists for this law; none added.

## Rebuilds

`semio-framework-job-context` is linked into every plugin guest. Required: `procedural` (generator) and `demonstrator` (koordinator, verfolgen) guest components, and every other hub that ships this crate (cad, fem, playbook, sourcing, puzzle, gis, flow, process, energy). The shard worker and the site need no TS rebuild; a site refresh only to pick up the rebuilt modules (the overlay server serves them from the staged modules directory).

The probe bridge and overlay server are kept as `issuer-clock-probe-bridge.js` and `issuer-clock-overlay-serve-k.mjs` (the latter reads a `epoch:K` line from `KFILE`).

## Follow-up: `original-turn.native-return-refused: semantic` after the job-context fix

### Cause

The authority refusal is gone; the next failure is a guest semantic fault. `🗑️generated/gen-2208.log` shows, right before the first `[DEBUG] semantic ...`, `[DEBUG] bundle reported 2223 observed 2243 coord 1` (someone else's print in `🏪️store/⚙️engine/🧳️bundle/🦀️.rs` `advance_apply_batch`). `advance_apply_batch` ends with `original_one_item_turn_work(reported, observed, coordinated)` (`🏪️store/🎮️one-item/🦀️.rs:249`), whose law is "reported work covers the entire call"; `observed > reported` is refused with `InvariantViolated` ("one-item work report omitted observed native effects"), which the poll surfaces as the semantic fault (the host only sees `semantic`).

The batch preparation branch (uncommitted multi-step loop by another session) set `publication.turn_work` to a sum of `max(owner.last_turn_work(), per-step effect delta)`, which missed 20 units of native receiving work (2223 vs 2243) done between the per-step measurements.

### Fix

Same branch of `advance_apply_batch`: take `receiving.actual_work()` before the loop and report `work.max(delta over the whole loop)`, so the reported work covers the observed effects (two lines, `call_effects_before`). No TS change; the probe bridge was not needed because the guest's own debug line identifies the refusal.

### Tests

`cargo check -p semio-os-store-engine-bundle` only (see below); no focused unit test: reproducing observed > reported needs a batched publication with receiving control and the loop is another session's uncommitted code. The existing law test is `🏪️store/🧪️tests/🌈️bin-03/🔬️unit/📚️batch-publication-and-stamped-clock-laws`. After the rebuild the boot itself is the test: `[DEBUG] bundle reported X observed Y` must show X >= Y.

### Rebuilds

Every guest linking `semio-os-store-engine-bundle`: procedural again, plus demonstrator and the other hubs. Remove the other session's `[DEBUG] bundle reported` print when they finish.

## Follow-up 2: Generator stalls on `setActiveExample` / `flowEvalTick` publication

Status: cause NOT yet established; narrowed, one `[DEBUG]` probe added to source for the next rebuild.

### Print owners

- `[DEBUG] admits REFUSED ...`: `🧵️job/🛬️receiving/🎟️work/🦀️.rs` `OriginalJobWorkDriver::admits` (uncommitted one-line edit by another session). `ready=false` is the only failing term (`fuel=64`, `additional=64` fit).
- `[DEBUG] recv refuse tag=N`: `recv_dbg` in `🧵️job/⏱️context/🛬️owned/🦀️.rs` (another session debugging receiving admission; cooperate there, no duplicate edits).
- `[DEBUG] rel`, `[DEBUG] bundle reported`: other sessions.

### Evidence (probe bridge `issuer-clock-probe2-bridge.js` on the staged procedural module, overlay only)

- Not slow polls: 44 of 28,500 polls took more than 6 ms; the stall spans thousands of fast polls. Contract step window is 7,500 us (`bounded_first_step(.., 7_500)`).
- Not the clock: guest clock reads never go backwards (`back=0`), jumps above 7.5 ms occur only at boot between polls; about 64 reads per poll during the stall.
- `ready` (`🛬️owned/🦀️.rs:164`) is `!cancel.is_cancelled_now() && now < deadline` (non-retirement). With clock and window excluded, the remaining term is a cancelled operation token.

### Working hypothesis

`🧵️retained-command/🔄️full-operation/🦀️.rs` ~364: after a worker continuation fault the code cancels the operation lease, but the fault is then published through a non-retirement receiving scope whose `ready` is false for a cancelled token, so the ladder never moves and the stall verdict masks the first fault. The first fault itself is unknown; the host sees only the stall. I added `[DEBUG] worker continuation cause: {cause}` there; one rebuilt procedural boot shows the real fault. If the lease cancel precedes fault capture in a non-retirement scope, fault capture should run under `with_original_retirement` (ready = true), with a law test on that.

### Rebuilds

procedural (and only procedural to read the new print); no TS/site change.

## Follow-up 3: one-item publication preparation refused, retried forever

Path: `🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`.

- Work accounting on this path is already covered: it settles `publication.last_turn_work()` via `cx.consume_fuel`, and `advance_apply_batch` (bundle) now reports at least the observed receiving work, so `original_one_item_turn_work` no longer refuses here.
- The silent retry: `PreparationRefused` and `Blocked` both end the unit with `Ok(())` (HEAD behaviour; the prints are another session's). The store has set `publication.fault`, `begin_close()` and phase `Closing`/`Fault`; `advance_apply_batch` then answers `Blocked` (Closing) or `PreparationRefused` (Fault) on every later turn. The only place a refusal is surfaced is `retained_structural_refusal` (`🧵️retained-command/🔄️full-operation/🦀️.rs:255`), which reports only `ValueRefusalKind::UnsupportedOwner`; any other kind stays pending until the ladder stall verdict, which hides it.
- Not fixed yet: the refusal kind is unknown. Surfacing all kinds would also surface retryable `WorkLimit`/`OwnershipLimit` refusals. I extended the other session's print to show the refusal: `[DEBUG] one-item publication preparation refused: {refusal:?}`. After the next demonstrator rebuild the kind decides: a non-retryable kind (InvariantViolated, InvalidValue) should be added to `retained_structural_refusal` with a law test; a limit kind means a grant/quote bug upstream (the receiving quote of this unit, `receiving_quote`/`receiving_work`, refused by `ready=false`, see follow-up 2).
- Rebuild: demonstrator and procedural (one print).
