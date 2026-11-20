# Norm boot: `admitActorContext` refuses with `invariant-violated`

Status: root cause found by source reasoning and fixed in Rust; NOT compiled, NOT run (no cargo build on the busy 16 GiB machine).

## Which check fires

1. `admit_wit` -> `OriginalActorGuestEntry::admit_plugin_step`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs`).
2. Its business closure ran, in this order: `admit_original_core_wire_delivery`, `admit_original_return_table`,
   `admit_original_escrow_publisher`, UI arena, `ensure`.
3. Call 1: the table admission sets `birth_requested`, returns `Ok(false)` -> the `table_false` probe with
   (Absent, Absent, Absent, false, false). The publisher is not even asked for yet, because the closure only reaches it
   once the table admission returned `true`.
4. Call 2: `borrow_context` (`🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs`) sees `return_birth_requested()` and runs
   `birth.service`. The chain (`🧵️job/🛬️receiving/🎟️native-turn/🌱️birth/🦀️.rs`, `chain`) has `publisher_wanted == false`,
   so it births only the table. The table is Live and its accepted proof is retired at the end of the chain.
5. Call 3: the table is present, the closure reaches `admit_original_escrow_publisher`, which sets
   `publisher_requested = true` and returns `Ok(false)`.
6. Call 4: `chain` now has `table_needed == false`, `publisher_needed == true`, so `table_proof = None` is passed to
   `filled(.., Stage::Publisher, (&origin, None, ..))`. `birth_action` (`🧵️job/📬️outcome/🪙️budget/🌱️birth/🦀️.rs`,
   `Stage::Publisher`) does `table.ok_or_else(missing)?` with
   `missing = refused("original return birth lacks its accepted proof")`; `advance_publisher_birth` additionally needs
   `table_with_post(table_proof)`. A table born in an earlier chain cannot supply that proof, so the chain fails.
7. `advance_inner` cancels any building stage and returns the error. `borrow_context` wraps EVERY `birth.service`
   failure as `ValueError::new(ValueRefusalKind::InvariantViolated, "original return birth failed: ...")`;
   `admit_wit` ships only the kind. That is the `invariant-violated` the bridge logs. The message text with the real
   cause stays in the guest, so the host cannot see it.

The probe line from the report is only call 1; the later calls print nothing because the publisher path has no probe.

Evidence for the diagnosis:
- Other agents already hit the same message in the native test `birth_chain` ("original return birth lacks its accepted
  proof", `job-crates-tests.md`); that test requests the publisher before the table (`request_return_publisher`), which is
  the only supported order and why it later went green.
- The receipt path (`📤️receipt/🦀️.rs`) calls `admit_original_escrow_publisher` directly, without a table call first.
  `admit_original_escrow_publisher` itself calls `admit_original_return_table` and therefore already births the table in
  the same chain as the publisher. The explicit `admit_original_return_table()` in `admit_plugin_step` was the only
  caller that forced the split.
- `original_plugin_publisher_receipt_failure_retains_complete_cause` pre-births the table in its own turns and only
  asserts `is_err()`, so it passed on this very error and masked the bug.

## Framework-wide or norm-specific

Framework-wide. The failure sits in the plugin bootstrap admission that every wasm plugin component runs before `ensure`
(the norm-specific plugin init, apps, editor/viewer config and presence only run in `ensure`, the last step, which is
never reached). Nothing in norm's declaration triggers it. Only norm is staged in the browser prototype, so no other
plugin could be compared at runtime; the same macro (`📣️macros/⚛️wp4-reactor`) and the same `admit_plugin_step` back all
of them.

Recent edits: no uncommitted diff in the job crates; `admit_wit`/`admit_plugin_step` date from 11-11 / 11-12. The
trigger is the 11-10..11-12 "Source birth became paid" change (paid, single-chain birth with accepted proofs) combined
with the unchanged table-then-publisher call order. The 11-16 edits in `🧵️job` (`debug_probe`, origin/source probes) are
instrumentation only.

## Fix

`admit_plugin_step` business closure: drop the explicit `admit_original_return_table` call so the publisher request
precedes the table birth and one chain births table and publisher together (same shape as `birth_chain`):

```
if !cx.admit_original_core_wire_delivery()?{return Ok(false)}if !cx.admit_original_escrow_publisher()?{return Ok(false)}
```

Regression test added (unrun): `original_plugin_bootstrap_births_table_and_publisher_in_one_chain_under_the_whole_grant`
in `🧪️tests/🤝️admission/🦀️.rs`; it drives `admit_plugin_step` from a fresh entry with the whole remaining grant as the
per-turn policy (exactly what the TS bridge issues) until `ready`, requires no refusal, then closes.

Known follow-ups (not done):
- `original_plugin_publisher_receipt_failure_retains_complete_cause` still pre-births the table separately; it should
  drop that table loop so it exercises the real publisher birth failure.
- Latent: `borrow_context` collapses the kind of any birth failure to `InvariantViolated`, and `admit_wit` drops the
  message. Preserving the primary kind (or exposing the cause) would have made this a one-minute diagnosis.
- If the combined chain needs more than the whole remaining grant in one turn, `grant_admits` returns `Narrow` and the
  bridge will report `original-turn.context-held` instead; then the table-plus-publisher quote must be compared with the
  TS grant.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs` (closure in `admit_plugin_step`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🧪️tests/🤝️admission/🦀️.rs` (new test)

## What must be rebuilt

Rust guest code changed, so a component rebuild is required for every plugin component (norm and all others) and the
host-side bundles that embed them; a TS bridge restage or patching the staged `bridge.js` cannot fix this. Host
`cargo test` of the reactor context crate (`admission` tests) verifies the fix natively before the wasm rebuild; it was
not run here.

## Follow-up (08:21 restage): `original-turn.source-not-ready` at `admitWire`

Source reading only; the first failing call is not visible in the pasted console, so this part is NOT root-caused.

1. The Rust fix worked. The stack is `poll` (bridge.js:804) -> `admitWire` (746), and `poll` runs `await admitContext()` first,
   so `admitActorContext` returned `ready` and the table plus publisher chain completed. The `table_false` line with
   `publisher_wanted=true` is only call 1 (the call that requests the combined birth, `Ok(false)` by design) and is printed once
   because the probe de-duplicates identical details. It is not a stuck admission, and neither the grant nor a missing
   bridge loop is involved: `admitContext` loops until `ready` and throws `context-held` on zero progress, which did not happen.
2. `OriginalOperationSource.issue()` (`🎭️actor/🎟️retained-turn/🟦️.ts:58`, bridge.js:100) throws whenever the source is closed,
   has an outstanding turn, or holds a receipt, producer or native refusal. It cannot fail on a fresh source, so a PREVIOUS
   call left the source blocked and this is a later call (replay or next poll). Holders in the bridge: the `poll` catch
   (bridge.js ~812: any failure thrown by `reactor.poll`, including a retained semantic plugin fault, is held with
   `holdNativeRefusal`), the `admitWire` catch, `receiveOriginal` on a refused receipt, the job funds, and a dangling
   `issue()` when a wasm call throws before `receive`.
3. Most likely the first poll (the first lifecycle turn that now runs norm's real init, because `ensure` is finally reached)
   failed, was classified retryable (`plugin.reactor-turn-deadline`, logged with `console.log`, so absent from an
   error-only paste) or reported elsewhere, and the replay then hit the held refusal. If it is the retryable deadline, the
   bridge bug is that the `poll` catch holds a refusal for a failure the shard worker replays; if it is a semantic plugin
   fault, the fault itself (norm init) is the real problem and the held refusal is only the follow-on.

To get the exact first failure without a rebuild, patch the staged bridge (adds `[DEBUG]` logs of the blocked source state and
of the first `poll` failure; applies cleanly and parses, tested on a copy, not applied to the staged file):

```
python3 "<ticket>/norm-bridge-debug-patch.py" ".cache/b/p/m/browser/prototype/modules/📕️norm/🌉️bridge.js"
```

Please send the full console (all levels, including `shard worker: retryable lifecycle deadline ...`) or the `[DEBUG] poll failure`
line; that names the real failing check. Rebuild impact so far: none for this step (no further source change). Once the first
failure is known, a Rust fix needs a component rebuild; a bridge fix needs a restage (or the same one-line patch of `bridge.js`).

Files added: `norm-bridge-debug-patch.py` (ticket folder).

## Follow-up 2: first `reactor.poll` returns `semantic` after 9 items / 4096 B / 9 work

Facts from the debug patch: epoch 11, failure tag `semantic`, spent 9 items + 4096 copied bytes, 9 work units, no deadline.
`semantic` means `dispatch_value_step` stored a complete `OriginalTurnFailure` in `value_fault` (guest custody); the WIT result
carries only the tag, and `publish_actor_failure_wit` is a stub, so the host can never decode the cause. No TS patch can show it.

Most likely cause (source reasoning, unverified): the 9 items / 4096 B / 9 work are the per-turn origin refresh that
`borrow_context` pays (`birth_requested` stays true after the table admission, so `birth.service` runs a refresh chain before every
turn's business). That spend is posted before `poll_kernel_turn` starts, and its first statement is `admit_turn_entry`
(`⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement/🦀️.rs`), which required
`budget.retained.grant == cx.retained_grant()`. The TS bridge issues the whole remaining grant, but `cx.retained_grant()` is already
reduced by the refresh, so the check fails with "original reactor retained input does not match its borrowed context"
(`reactor_close_fault`, retained as the semantic failure). The refresh cost equals the observed spend and nothing else was spent.

Fix (Rust): `admit_turn_entry` now compares against `turn_entry_grant(cx)` = current grant plus what this call already posted
(`original_turn_progress(reserved, retained_progress)`), i.e. the grant the turn was admitted with.

Diagnostic (temporary, `[DEBUG]`): `poll_wit` now logs the complete retained failure (`{failure:?}`) through the host `log` import
(`[actor] [DEBUG] poll semantic failure: ...` in the console) whenever poll ends semantic. If the entry check was not the cause,
this line names the real one. Remove once boot works.

Risks to check after the rebuild: the kernel `retained_receipt` of the poll covers only the poll's own spend (captured after the
refresh), while `bridge.js` `receiveOriginal(result.retainedReceipt)` is the TS accounting; if the guest's aggregate and the TS
remaining diverge by the refresh cost, the next epoch's `validate_currency` refuses ("renewed its physical source authority").

Files changed: `⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement/🦀️.rs` (fix), `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`
(`[DEBUG]` log). Rebuild: Rust guest, so every plugin component must be rebuilt and restaged; no bridge change.

## Follow-up 3: the entry check still failed after the "current grant + posted" adjustment

Confirmed culprit: `admit_turn_entry`. My adjustment was wrong because `cx.retained_progress()` is the context's CUMULATIVE
`fresh_spent` (`borrow_context` hands `&mut self.fresh_spent` to the Source context), not this call's spend; adding it over-counted
every earlier epoch. This call's spend is only the delta since the bind point, which `StepContext` does not expose.

Exact original comparison (`🪜️entry-lifecycle-and-retirement/🦀️.rs`): `budget.retained.validate()`, `operation`, `generation`
(both equal to the context's) and the whole `RetainedCloneGrant` (items, copy, capacity, release, depth) equal to
`cx.retained_grant()`. Epoch, fuel, deadline and clock binding are not compared here (`borrow_context` already enforces them).

Why equality can never hold after a refresh: `cx.retained_grant()` is the minimum of (initial grant - cumulative spend, total
remaining - this turn's spend, policy - this turn's spend, and, once the return budget is bound, the return budget's own remaining
stock). The origin refresh lowers the first three by its spend and the bound return stock can lower it further by reserved return
currency, so the turn-start grant the bridge issued is strictly larger than `cx.retained_grant()` whenever birth ran first.

Correct invariant: the borrowed context must name the same operation and generation, and everything it may still spend (its
remaining grant) must lie INSIDE the input grant (each axis <=, depth <=). The kernel receipt still returns against the full input.

Fix: `admit_turn_entry` now checks operation, generation and `held <= input` on all five axes (the temporary `turn_entry_grant`
helper was removed). On mismatch it logs both sides through the host `log` import:
`[actor] [DEBUG] turn entry mismatch: operation a/b generation a/b input {..} held {..}` (temporary, remove after boot works).
The `[DEBUG] poll semantic failure` log in `poll_wit` stays for the same reason.

Files: `⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement/🦀️.rs`. Rebuild: Rust guest, all plugin components; no bridge change.

## Follow-up 4: first poll now fails with `authority / invariant-violated`, zero spend (epoch 13)

Entry check is fixed (no mismatch log). The new failure is NOT root-caused; what source reading established:

- `poll_wit` returns `failure(.., Some(kind))` (authority) for: lifting the admitted wire (`take_poll`), `native_input`/`step_budget`,
  `outputs_held` (busy), a failed `with_actor_operation_authority`, or an `Authority` result of `dispatch_value_step`.
  The last one wraps `validate_next_input`, the body-budget subset check, `begin_dispatch` (-> `validate_currency`: grant must be a
  subset of the guest's `remaining_grant()` and `work.fuel <= prev fuel - fresh work`) and `borrow_context` (pre-birth checks, the
  total/policy binds, then `birth.service`, whose failure is rewrapped as `InvariantViolated` "original return birth failed: ...").
- Host-shim refusals (`operation_begin/progress/finish`) cannot be the source: they map to codes 1/3/6, i.e. Canceled / WorkLimit /
  UnsupportedOwner, never `InvariantViolated` (`🚪️io/🛂️authority/🦀️.rs:19`).
- Zero spend means the failure is before the refresh chain pays, or the failing birth posted nothing.
- Possible mismatch worth checking in the output: the TS source charges the admitted wire (`admit_actor_input`, epoch 12: 295 B
  here) that the guest context never records, so the TS grant is smaller than the guest's remaining (allowed by the subset
  check); the opposite direction would refuse in `validate_currency`.

Diagnostic added (temporary): in `poll_wit` the authority arm now logs
`[actor] [DEBUG] poll authority failure: kind .. spent .. work .. error <ValueError Debug>`, which carries the real message
(the birth rewrap formats the whole `OriginalTurnFailure`, the checks above each have a distinct literal message).
File: `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`. Rebuild: Rust guest (all plugin components). Next step: read that line.

## Follow-up 5: "original actor context renewed its physical source authority"

Raised only in `OriginalActorContext::validate_currency` (`🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs`), reached from `begin_dispatch` -> `begin`
-> `validate`. Rule for a new epoch: `subset(retained.grant, self.remaining_grant())`, with `remaining_grant() = initial_grant - fresh_spent`
(all five axes, `<=`). The message appears when ANY axis of the bridge-issued grant is LARGER than the guest's own ledger, i.e. the
guest has recorded physical spend that the bridge's receipts never debited. Epoch, fuel, deadline and clock binding are checked by
different branches with a different literal ("changed its issuer clock/deadline or renewed work"), so they are not the failing part.

What I could and could not establish from source:
- Bridge-only charges (the admitted wire at `admit_actor_input`: 1 item + ~147-295 B copy and capacity, two of them this run) make the
  bridge grant SMALLER than the guest's remaining, which this check allows; they are not the cause (my earlier note was the harmless direction).
- Receipts to the bridge are the per-call delta posted when the Source context drops (`StepContext::drop`: aggregate delta incl. reserved
  returns) while `fresh_spent` is lowered to the fresh part afterwards; both conserve in that direction too.
- Therefore the offending axis is a guest-side spend that is recorded in `fresh_spent` and not returned in any call's receipt. The
  admission and poll receipts, `admit_wit` and `poll_wit` all return `call.receipt_spent`, and the only candidates left are the axis-specific
  ones (copy, capacity, release or items), which cannot be told apart without the two ledgers.

No root fix is possible from source alone without guessing the axis; a guess that loosens the check would hide an accounting bug and let later
receipts drift. So this round adds the self-explaining diagnostic instead:
`OriginalActorContext::debug_currency` (+ slot and entry delegates) and an extra `[DEBUG] poll currency: ..` line printed next to
`poll authority failure`, containing input epoch/grant/work and the context's epoch, `initial_grant`, `fresh_spent`, `remaining_grant`,
`issued_grant`, work, `work_units_spent`, `reserved_work`, `reserved_spent`. The axis where grant > remaining names the unreported spend.

Files: `🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs`, `⚛️reactor/🎟️context/🌉️call/🦀️.rs`, `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`.
Rebuild: Rust guest (`norm`, and every plugin component); no bridge change or restage needed.

## Follow-up 6: unreported spend = the per-turn origin refresh of a SUCCESSFUL poll

Ledger output: guest context epoch 11 `remaining` (items 1043607, copy 14675277) vs bridge epoch-13 grant (1043614, 14678896); fuel 49995029
(epoch-11 issue) -> 49995018 at epoch 13, while the guest counted `work_units_spent 20` for epoch 11. So the guest's epoch 11 spent
9 items / ~3.9 KB / 9 work more than the bridge was told; epochs 11 and 12 here are a poll that SUCCEEDED (entry fix) and the admitted wire
of the next poll (12, charged only on the bridge side, harmless), then poll 13 is the one refused.

Root cause: `poll_kernel_turn` (`⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs`) captured its receipt baseline
(`original_progress = cx.retained_progress()`, `original_work = cx.original_work_units_spent()`) AFTER `borrow_context` had already
paid the origin refresh chain (9 items, ~4 KB, 9 work) into the same call. The kernel `retained_receipt` that the bridge debits
(`bridge.js receiveOriginal(result.retainedReceipt)`) therefore covered only the poll's own work. The guest ledger (`fresh_spent`,
`work_units_spent`) kept the refresh, so the next epoch's grant looked "renewed" (`validate_currency`) and the fuel too.
(`poll_wit`'s `return_original(spent, work)` uses the full recipient and passes, which is why only the bridge side drifted.)

Fix (Rust only):
- `StepContext::original_call_start_progress()` (`🔨️modules/🧵️job/⏱️context/🦀️.rs`): the aggregate progress when the call's external
  recipient was bound, i.e. before any birth or refresh paid for the call.
- `poll_kernel_turn` now baselines on it (and on work 0, the fresh per-call counter), so the turn receipt includes the refresh.
  Reserved return spend is still subtracted exactly as before (`kernel_work_receipt` keeps the fresh conservation).

Separate bridge bug seen on the way (not needed for this fix): the `poll` catch in `bridge.js` tests `failure.input && failure.spent`, but
jco throws an `Error` carrying `payload`, so a failed poll's receipt is never debited either (it should read `failure.payload`).

Rebuild: Rust guest (`norm` and every plugin component, the job crate changed too); no bridge restage required for this fix.
The `table_false` probe belongs to another agent and was left in place.

## Follow-up 7: "lifecycle backing unavailable" at the first instance open

Raised in `NativeLifecycleRegistry::admit` (`⚛️reactor/🚪️lifetime/🦀️.rs`) when `slots.allocation_admitted` is false, i.e. the eager
`ReactorFixedSlots::new()` reservation of 1024 slots made when the `PluginRuntime` was built (inside `ensure`) was refused and the
refusal was stored silently. (`admit_lifecycle_focus` itself does not raise it; it is the `Event::InstanceOpen` arm in
`🔄️turn/📨️event-dispatch/🦀️.rs`.) Not norm-specific by construction: the slot type is small and generic, and the same eager
reservation is used by every plugin runtime. The design note in `RuntimeInstanceRegistry::new` says exactly why that pattern is wrong in a
guest ("allocator success is not an admission"; backing is taken on the first admission that needs a slot). I could not determine from
source WHY the reservation was refused (needs the slot size and the guest memory at construction), so the fix is the documented pattern
plus a diagnostic.

Fix (Rust):
- `ReactorFixedSlots::unallocated()` and `admit_backing()` (`⚛️reactor/⏲️tasks/🧮️fixed-slots/🦀️.rs`): empty table, one-shot reservation.
- The three registries touched by an instance open now start unallocated and take backing on first use:
  `NativeLifecycleRegistry` (`admit`), `InstanceMetadataRegistry` (`insert`), `ReactorCloseRegistry` (`insert` and `preflight_reactor_close`).
  Other `ReactorFixedSlots::new()` users (timers, task lanes) are unchanged.
- Diagnostic (temporary): a refused open logs `[actor] [DEBUG] lifecycle admit refused: <reason>; slot bytes N, slots 1024, backing <bool>,
  guest memory Some(bytes)` (`🔄️turn/📨️event-dispatch/🦀️.rs`), so if even the lazy reservation fails the slot size and linear memory show why.

Rebuild: Rust guest (`norm` and every plugin component). Not verified further down the open path (`plugin_open_actor_instance`, assets, UI);
if the next error is another "backing" or allocation refusal, the log line above names the size problem.

## Follow-up 8: the lifecycle slot is 643,680 B, so 1024 inline slots (659 MB) can never fit in 512 MiB

Diagnostic: slot bytes 643680 (norm-energy 643456), 1024 slots, guest memory ~94 MB. The slot is `NativeLifetimeSlot<PA>` (`⚛️reactor/🚪️lifetime/🦀️.rs`):
`cell` (small), `command_origin` (small), `native_created`, `patch_sequence`, and `pending_creation: Option<PendingNativeCreation<PA>>`
(`🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs:21`). The large inline part is the open-time construction state
(`pending_creation`: three `OriginalConstructionAdmission` frames incl. `OriginalStoreBirth`, `Option<PA>`, `RuntimeActorAuthority`, a
`OriginalTurnFailure`, ...); the 224-byte difference between components is `PA`. I did not isolate the single 600 KB member (no compiler);
the structural fix does not depend on it.

Fix: the table now holds one `Box<NativeLifetimeSlot<PA>>` per admitted instance (`ReactorFixedSlots<Box<NativeLifetimeSlot<PA>>>`):
the fixed table is 1024 pointers (~4 KiB, taken lazily on first open via `admit_backing`, follow-up 7), and the 643 KB body is allocated
only for a live instance and freed when its terminal ACK removes it. `get`/`get_mut` deref the box, `admit` boxes the new slot; the
existing field and method accesses (`slot.cell`, `slot.command_origin`, `slot.close_command_origin(..)`, iteration) are unchanged through
auto-deref. Peak cost: ~0.65 MB heap per simultaneously open instance (plus a transient stack copy of the slot at admission, well under the 8 MiB shadow stack).

Checked the other fixed tables: `InstanceMetadata` (u32 + String + `QuotaSchema`), `ReactorCloseState` (key, flags, cursors),
`TimerEntry` and the task-lane record were not oversized in the reported numbers or by their definitions, so they stay inline
(metadata and close registries stay lazy from follow-up 7).

Follow-up worth doing later: box or slim `pending_creation` itself (`Option<Box<PendingNativeCreation<PA>>>` plus constructing it in
`🏃️runtime/📦️instances/🚪️opening/🦀️.rs:26`) so that even one slot is small; not needed for boot.

Not verified: no compiler run. Files: `⚛️reactor/🚪️lifetime/🦀️.rs`. Rebuild: Rust guest, all five norm components (every plugin component). The
`[DEBUG] lifecycle admit refused` log now reports the box size; remove it with the other probes once boot works.

## Follow-up 9: "original total exceeds its persistent source recipient" (epoch 17)

Raised in `StepContext::bind_original_retained_total` (`🔨️modules/🧵️job/⏱️context/🦀️.rs`), called from `OriginalActorContext::borrow_context`
with `total = input.grant - epoch_spent` (the bridge's remaining) before the birth refresh and business run. Rule: `total` must be a subset of
`self.retained_grant()`, the minimum of (initial grant - fresh spend), (total - spend), the policy and, once the return budget is bound,
`OriginalReturnBudget::remaining()`.

Why it fires only now: `remaining()` is `raw_remaining()` MINUS the currency held by live return claims (`held_items`, `held_copy_bytes`, ...),
while the bridge's total is the whole outer remaining, which still contains that held, not yet spent currency. The numbers fit: the bridge's items
at epoch 17 are exactly `1048576 - 5005` (fresh 5001 + reserved 4), i.e. it has no slack above the budget's own ledger, so any hold
(the 4 items / 480 B of reserved return currency show that return claims are live this turn; they did not exist in epochs <= 13) pushes the net-of-holds
stock below the outer total. The currency check (fresh ledger) passes, as you saw; the budget ledger is the one that refuses.

Fix: the total-binding comparison now uses the stock BEFORE holds (`retained_grant_by(false)`, built on `raw_remaining`), because held currency
is still owned by the outer total. Business spending is unchanged: `retained_grant()` still nets the holds (`retained_grant_by(true)`), so
nothing can spend held currency. The refusal message now prints both sides: `total {..} recipient {..} net of holds {..}`.

Same family checked: `narrow_original_work_budget` compares `budget.retained` (already the meet with `retained_grant()`), so it cannot trip on holds;
`admit_turn_entry` compares the held-netted grant against the input (subset), also fine. `renew_turn` (requires no live claims) is not on this path.
Not verified further (no compiler run, not traced through render); `clock_binding 2` is the bridge's operation id in `originalTurnWit` (`clockBinding: input.operation`)
and does not rebind the context.

Rebuild: Rust guest (job crate changed, so every plugin component). Diagnostics kept.

## Follow-up 10: the return budget's own ledger is below both the fresh ledger and the bridge total

Numbers: budget spent = (5010 items, 2115553 B) = fresh (5001, 2113741) + reserved (4, 480) + budget-only (5, 1332); the bridge's total =
fresh + reserved + the admitted wires (0 items, 177 B). So the budget over-records (5 items, 1155 B vs the bridge, capacity and release agree).
I could not attribute those 5 items / 1332 B to one call from source alone (no per-call budget trace exists); what source reading does establish is a
structural gap that makes the budget drift against the per-epoch outer grant:

- `OriginalReturnBudget::commit_origin` (`🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs:189`) adopts the origin's `spent`/`work` only for the FIRST origin
  (`origin_prior.is_none()`). Every later turn pays an origin refresh that replaces the origin with a new `granted` (the bridge's total of that epoch,
  already net of all earlier spend) but keeps `self.spent` cumulative since the first origin, so spend is deducted twice.
  `renew_turn()` is the method that reconciles them, and it is called only from the job `testkit` (fixture caller), never in production.
- The bind check compares the bridge's total with a ledger (the budget) that lags one epoch and does not see TS-only charges (wires), so it is not a valid
  invariant for the outer total; the budget conserves itself at every spend (`record_retained` -> `conserved()`), so spending stays bounded by it anyway.

Fix:
1. `commit_origin` now also adopts the new origin's `spent`/`work` when the budget is `renewable()` (no live claims, holds or failure), i.e. production performs
   the same reconciliation as `renew_turn` on every refresh; a budget pinned by live claims keeps its ledger (the existing "cannot renew" rule).
2. `bind_original_retained_total` compares the bridge's total with the Source's own persistent ledger only (`retained_grant_by(None)`: initial grant minus fresh spend and
   the total/policy terms); spending still uses the net-of-holds stock (`retained_grant()` unchanged). The refusal message prints `total / ledger / stock / net of holds`.

Not verified (no compiler run) and the 5 / 1332 attribution is open: if a later turn is refused by the budget itself (`original return source receipt exceeds spent and held authority`
or a consume refusal) the budget is still over-recording; the next diagnostic would then be a per-call `[DEBUG]` of `budget.snapshot()` deltas.
Files: `🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs`, `🔨️modules/🧵️job/⏱️context/🦀️.rs`. Rebuild: Rust guest, all plugin components. Diagnostics kept.

## Follow-up 11: `WorkLimit "original host operation admission refused the guest producer"` at epoch 87

Not an accounting mismatch: it is the host shim's deadline check. `refusal(code)` (`🚪️io/🛂️authority/🦀️.rs:19`) maps host codes 1 -> Canceled, 3 -> WorkLimit,
6 -> UnsupportedOwner, 4 -> InvariantViolated; the generated shim (`🟨️.js`, `originalCheck`) returns 3 when `operationClockUs() >= originalTurn.deadlineUs`, and
`operationBegin()/operationProgress()/operationAllocation()/operationReserveReturn()/operationFinish()` all call it, so the first guest producer after the deadline is refused
with exactly this message. Nothing else yields code 3 (no fuel cap or operation-table bound in the shim).

Why the deadline is hit: the operation deadline is `now at activation + maximumElapsedUs` (`OriginalOperationSource` constructor), and the guest pins it for the whole
operation (`validate_currency` refuses a changed `absolute_deadline_us`). The browser's actor operations come from `🖼️canvas/🧵️renderer-work/🔣️stock.json` (used by
`rendererOriginalWorkStock()` in the browser host and plugin bridge): operation `maximumElapsedUs = 30000000` (30 s) for the WHOLE activation lifetime, pool
`120000000`. Activation 2 here had deadline 68573099 us, i.e. created at ~38.6 s, expiring at ~68.6 s; activation 1 had 30965200 us. A 30 s lifetime wall for a long-lived actor is
simply wrong, and a hidden/slow tab trips it during boot.

Epochs 18..87 are not a guest loop by themselves: each worker drive poll (`more-work`) is one poll + one admitted wire = 2 epochs, so ~35 polls; the worker yields a
macrotask (`driveYield`) between polls and a hidden pane clamps timers to ~1 s, which is ~30 s for ~35 polls. Whether the guest should need ~35 more-work polls at boot
(document loads, resumes) is a separate question not answered here.

Fix (host config, TS/JSON only, no component rebuild): `renderer-work/🔣️stock.json` now gives an activation operation 3600000000 us (1 h) and the pool 86400000000 us (24 h; the
pool must be >= any operation, `KernelOriginalResourcePool.admit`). Each turn is still bounded by its own fuel and wall budget (`msg.budget`), and the per-crossing drive by
`SHARD_TURN_DRIVE_STEP_CEILING = 512`; the shard worker drive `budget` stop reads the same deadline, so it now stops on input or the step ceiling instead of a 30 s wall.
A real renewable per-turn deadline would need the Rust context to accept a renewed `absolute_deadline_us` (currently refused by design); not done.

Restage required (the stock is compiled into the renderer bundle, not into the component). Quick test without restaging: in each staged `🌉️bridge.js`, in
`OriginalOperationSource`'s constructor, replace `this.deadlineUs = now + admission.maximumElapsedUs;` with `this.deadlineUs = now + 3600000000n;` (the guest reads the deadline from every
epoch's input, so one consistent value is accepted). Diagnostics unchanged.

## Follow-up 12: the first InstanceOpen turn ran 776 s without returning

Static findings (no profiler, no run):

1. A turn is never sliced. `bridge.js poll` hands the guest `originalContextBudgetWit(turn)`, i.e. ALL remaining operation fuel and the ABSOLUTE operation deadline
   (now 1 h after the previous fix), and ignores the shard budget's `wallMs` (100) and `fuel`. The guest only yields through `StepContext::should_yield()` (fuel or deadline exhausted),
   so it can never yield partway. The instance open itself is already resumable (`plugin_advance_actor_opening` does ONE paid step per turn, and `plugin_resume_actor_openings` is
   called once per `poll_kernel_turn`), so slicing makes the worker's drive (`more-work`) interleave steps with host messages and with the hang watchdog.
2. A genuinely host-pending await inside the turn cannot complete in the same wasm call, and `drive_self_waking_ready` (`🏃️runtime/🧰️ops/🛡️wire-limits/🦀️.rs`) just re-polls it
   up to 1,048,576 times with no yield and then panics. 776,499 ms / 1,048,576 polls is ~0.74 ms per poll, which is a plausible cost of one deep async re-poll at opt-level 0/1,
   so a pending-forever future (a host reply awaited inside the turn) fits the number as well as one heavy synchronous step does. Statically I could not identify a specific
   await that waits on the host in the open path (opening itself is sync inside `advance_original_app`; candidates are the document/asset loads, typed operations, render and surface
   awaits), and I did not find a retry loop around the `table_false` birth condition: that birth finishes in admission (the context was `ready` before the poll).
3. Heaviness: `create_din4108_app()` builds the whole app definition (Editor builder, 5 window kinds, layouts, panels, actions) and `advance_original_app` runs it as one step;
   DSL parse, compliance evaluation (`semio_s_plugin_norm_compliance`, `NormHost`), scene and first render run in later steps/turns. I did not estimate their cost.

Changes:
- Bridge generator (`🌉️component-bridge-source/🟦️.ts`): `poll` now narrows the turn budget to `min(operation, shard budget)`: deadline `now + wallMs`, fuel `budget.fuel`. TS only; restage required.
  Staged quick test (applies to the current `bridge.js`, parsed OK on a copy): `python3 <ticket>/norm-bridge-turn-slice-patch.py <staged>/📕️norm/🌉️bridge.js` (and the other four norm bridges).
- Rust diagnostics (temporary, `[DEBUG]`): `[actor] [DEBUG] self-wake spin: ... after N polls` at 1k/10k/100k/500k pending polls (`drive_self_waking_ready`), and
  `[actor] [DEBUG] turn phase <name> at <ms> ms` around `resume-openings`, `dispatch-event` (includes the instance open) and `render-surfaces` (`🔄️turn/🪜️entry-lifecycle-and-retirement`, `🌀️poll-kernel-turn`).
  After a rebuild the last phase line before silence names where the first turn sits, and a spin line says whether it is a pending future (host wait) rather than computation.

Rebuild: Rust (all components) for the diagnostics; bridge restage (or the patch script) for the slice. The slice alone helps only if the open is many short steps; if one step is the
13 minutes, the diagnostics will say which phase, and then `opt-level` overrides for that crate (the profile `packageOverrides`) or splitting the step is the remedy.

## Follow-up 13: `actor-poll.event-count: a reactor turn admits at most one event`

Raised in the generated bridge `poll` (`🌉️component-bridge-source/🟦️.ts:195`, and the TS twin in `🎭️actor/🎟️retained-turn/🟦️.ts:123`); the guest enforces the same rule
(`wit_bridge::poll`: "a reactor turn admits at most one event"; `poll_kernel_turn` takes `&mut Option<Event>`). The protocol is one event per turn.

Who batches: `UiAcknowledgementHost.submitInstanceUiAcknowledgements` (`🎭️actor/📮️shard-client/📥️ui-acknowledgement/🟦️.ts:89`) sends `states.map(patchAckEvent)` through one
`sendInstanceLifecycle` crossing ("the reactor's poll takes a LIST of events, so N patch-acks ride one crossing", an older protocol comment). The shard worker's `turn` case
(`🧵️shard-worker-source/📨️message-handler/🟦️.ts`) hands that whole list to `api.poll`, so any crossing with 2+ patch-acks (the app shell publishes several surfaces) trips the check. The frame
path in the same worker already serves one turn per envelope.

Fix (host TS, keeps the one-crossing/one-result shape of the client): the worker's `turn` case now serves a batch as N sequential guest turns in order. Every turn but the last must carry nothing
the host could act on (`shardTurnCarriesNothing`, the exact predicate the drive already relies on, so nothing is dropped); the last one answers the crossing (with `commandPage`). If an earlier
event does answer with something, the crossing fails loudly with `actor-poll.event-batch-carried` instead of dropping it. The guest and the bridge keep the one-event rule.
Restage required; quick test: `python3 <ticket>/norm-worker-event-batch-patch.py .cache/b/p/m/browser/prototype/modules/🧵️shard/🟨️shard-worker.js` (parsed OK on a copy).

`[DEBUG]` probes removable now (mine): `poll semantic failure`, `poll authority failure`, `poll currency` (+ `debug_currency` in `OriginalActorContext`, slot and entry), `lifecycle admit refused`,
`turn entry mismatch` (`admit_turn_entry`), `turn phase` (+ `debug_phase`), `self-wake spin`; and the bridge-patch logs `[DEBUG] poll failure` / `source-not-ready` (only if you applied
`norm-bridge-debug-patch.py`). The other agent's `table_false`, `claim_false`, `origin_admit_false` (and the worker probe) plus `debug_probe` in `🧵️job/⏱️context/🦀️.rs` are noise and can be stripped with them. Keep the real fixes (see the earlier follow-ups).

## Follow-up 14: turns cycle back-to-back at ~12 items / 4105 B each; the lifetime copy grant is nearly spent

1. Why it keeps polling (source reasoning, most likely a regression of mine): it is not an idle drive. Each turn the Source pays its origin refresh (9 items, ~4 KB, `birth_requested`
   stays true; see follow-up 6) plus ~3 items of phase overhead = the observed constant 12 items / 4105 B, and nothing else, i.e. the instance-open step makes no progress. The
   open advances one paid step per turn behind `if cx.should_yield(){return Ok(false)}` (`plugin_advance_actor_opening`), and `should_yield()` is "fuel or deadline exhausted".
   The bridge slice I added in follow-up 12 gives each poll a deadline of `now + wallMs` = 100 ms, but at prototype opt-level one turn takes ~2 s and the refresh chain alone exceeds 100 ms, so
   the business already sees `should_yield() == true`, returns `Ok(false)` (more-work) without doing its step, and the next turn repeats: starved forever, paying only the refresh. The
   guest asks for more work because the open is genuinely unfinished (`NativeLifecycleRegistry::has_work` is true while a slot is not live).
   Fix (bridge TS, restage): the slice is now `max(wallMs, 3 x the previous poll's measured duration)` (`lastPollUs`), so a turn always gets room for its fixed overhead plus a step and
   self-corrects after one turn; floor stays `wallMs`. Quick test: `python3 <ticket>/norm-bridge-adaptive-slice-patch.py <staged>/🌉️bridge.js` on each norm bridge (replaces the previous slice
   patch; parsed OK on a copy). If turns still cycle after this, the loop is a real more-work source and the `[DEBUG] turn phase` lines will show which phase repeats.
   A real idle actor would park: `settle_turn_work` yields `Idle` only when no source (opening, resumes, typed operations, reconcile, document loads) is runnable; the worker drive stops on idle.

2. Renewal: there is none in the code. The operation (`operation` stock: items, copy, capacity, release, fuel, wall) is admitted once per activation by `KernelOriginalOperationIssuer.admit`
   (`🎭️actor/🎟️retained-turn/🟦️.ts`), the guest context pins operation, generation, clock binding and deadline for its whole life (`validate_currency`), and `returnClosed` hands it back only at
   close. So the retained grants and fuel are LIFETIME budgets, and every turn costs at least the refresh (~4 KB copy), i.e. 16 MiB copy buys ~4,000 turns whatever the app does.
   A real rotation (close the actor context and admit a fresh operation/generation at a quiescent boundary, or a renewable per-epoch grant in `validate_currency`) is a design change; not done.
   Interim fix (config, restage): `renderer-work/🔣️stock.json` operation grant is now items 8388608 and copy 536870912 (512 MiB, still a valid wasm32 `usize`); the pool (items 67108864, copy 4 GiB,
   fuel 1.1e12) covers 5 norm activations x (8M items, 512 MiB copy) = 41.9M items, 2.5 GiB. At ~4 KB per turn that is ~130,000 turns per activation instead of ~4,000.
   Reducing the per-turn overhead itself (refresh only when the turn needs the return budget) would be the structural improvement; not done.

Rebuild/restage: no Rust rebuild; restage the renderer bundle (stock) and the generated bridges, or patch the staged bridges with the script above.

## Follow-up 15: every turn consumes its whole (growing) slice and still reports more work

Observation: slice growth is exponential because each turn runs until `should_yield()`. Static search of the post-dispatch part of `poll_kernel_turn` found loops bounded only by the
deadline in the UI reconcile drive (`drive_reconcile_within`, up to 1,024 opportunities, each `patches.drive_one(cx)` while work is "drivable", re-checking `should_yield` only), plus
the store/document open loops (`while !cx.should_yield()`, which do consume fuel per iteration and look productive). The reconcile loop (the known area of
`📓️reactor-reconcile-spin-2026-09-14.md`) is the best static candidate: at prototype opt-level one `drive_one` of the shell/viewport/panels surfaces can take hundreds of ms, so the
slice is spent inside it; whether it makes progress or spins on an undrainable tracker cannot be told from source. `self-wake spin` not firing rules out `drive_self_waking_ready`
(no future stayed pending 1000 polls), and the turn is not blocked on a host reply.

Diagnostics added (temporary `[DEBUG]`, Rust, rebuild needed): `turn phase after-<step>` after each of extension, cold-pair, command-ingress, original-receiving, patch-close, result-pages,
displaced-surfaces, reconcile, task-resumes, before/after settle (on top of resume/dispatch/render phases), and `reconcile loop: N opportunities, M drive_one of 1024, yield <bool>` at the end of
`drive_reconcile_within`. The phase whose timestamp jumps by the slice names the spinner; the reconcile line says whether it ran many opportunities that each drove something.

Slice cap (bridge TS, restage): `slice = min(max(wallMs, 3 x last poll), 8 s)`, so a spinning turn cannot grow without bound; 8 s rather than 2 s because one prototype turn's fixed overhead
is already ~2 s (a 2 s cap would starve the business step again). Quick test: `python3 <ticket>/norm-bridge-adaptive-slice-capped-patch.py <staged>/🌉️bridge.js` (applies on top of the
previous adaptive patch; parsed OK on a copy). `norm-bridge-adaptive-slice-patch.py` now already includes the cap for fresh bridges.

## Follow-up 16: `shard worker: actor norm#1 already has a turn in flight`

The worker's `message` handler is async and every message runs concurrently: a `turn`, `frame` or `codec` message for an actor awaits the guest for the whole crossing, and a second one
for the same actor that arrives meanwhile hit `if (inFlightTurnActors.has(actorId)) throw ...`. That guard was a defect detector for a client that never overlaps, which held while turns lasted
milliseconds; with prototype turns of seconds (and sequential event batches, drive polls and ui-ack crossings), the host legitimately posts the next crossing while one is still running
(the client only guards its own lifecycle path with `owner.inFlight`, not the captured-activation `turn` lease). Both crossings are valid; the worker must not reject them.

Fix (worker TS, `🧵️shard-worker-source`): one crossing at a time per actor, FIFO. `enterActorTurn(actorId)` (`🎬️actor-state-and-lanes`) chains each admission behind the previous one
(`actorTurnTails`), marks the actor in flight only when it actually runs, and returns a `leaveTurn()` that releases the next one. The three admission sites (`turn`, `codec`, `frame`) now await it, re-check that the
actor is still the registered one after waiting (else `actor-lifecycle.activation-mismatch`), and call `leaveTurn()` in their `finally`. The running turn's drive loop already stops when a new
message arrives (`hostInputSeq` moves), so a queued crossing starts right after the current poll, not after the whole drive; `dispose` still cancels a running turn.
Restage required; quick test: `python3 <ticket>/norm-worker-actor-queue-patch.py .cache/b/p/m/browser/prototype/modules/🧵️shard/🟨️shard-worker.js` (needs the three guard sites present, i.e. the
current generated worker; it applied and parsed on a copy of the staged one).
Not changed: the client could also avoid posting a second crossing, but the worker is the single admission point for every client path, so serializing there fixes all of them.

## Follow-up 17: every turn's last phase line is `after-result-pages`

Correction of the premise: in `poll_kernel_turn` the statement after `plugin_drive_result_pages` (my `after-result-pages` line) is NOT the displaced-surfaces step. The order is
result-pages -> `plugin_drive_pending_effects` -> `plugin_drive_displaced_surfaces` -> `after-displaced-surfaces` -> `now_ms` -> `render-surfaces`. The step that never logs its end is therefore
`plugin_drive_pending_effects` (`🧰️ops/🧾️typed-operations/🦀️.rs:250`), unless displaced-surfaces itself fails.

1. Displaced-surfaces (`🧰️ops/🖥️surface-render/🦀️.rs:50`) is cheap and resumable: per live instance it retires ONE displaced projection (`retire_displaced_step`), returns `true` while any remains, and
   returns `Ok(true)` early on `should_yield()`; it keeps its cursor in the instance, so it makes monotone progress and cannot restart from scratch. It is not the spinner.
   `plugin_drive_pending_effects` polls each armed instance's `app.pending_effects(view, cx)` -> `RenderContext::pending_effects` (`🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs:135`):
   `if cx.should_yield() {return Ok(None)}`, then `self.refresh_cache().await?` (committed-document snapshot/cache rebuild), mounted snapshot-read preparation (`MountedSnapshotReadPreparation::grant`, `Ok(None)`
   when the grant is too small) and the app's own `A::pending_effects` against the document. That is where the first real document work (and the example load) lives.
2. There is no early `return Ok(more_work)` between result-pages and settle in `poll_kernel_turn`: every later statement is either infallible or uses `?`. A turn that skips the remaining phase lines
   therefore left through an `Err` (the poll then ends `semantic` and `[actor] [DEBUG] poll semantic failure ...` should print the full failure, unless that line was filtered out of the report), or inside
   `pending_effects` the turn trapped/never returned. A silent `Ok(None)` inside `pending_effects` would not skip phases, so it is not that.
3. No root fix is possible without the failure text; guessing a progress change in `refresh_cache`/snapshot-read preparation would be speculation.

New temporary diagnostics (Rust rebuild): `turn step pending-effects failed: <full OriginalTurnFailure>` and `turn step displaced-surfaces failed: ...` (via `debug_failed`), plus `turn phase after-pending-effects`.
Next run reading: `after-pending-effects` present -> displaced/render are reached (then my model was wrong and the log tells which); `turn step pending-effects failed` -> the error names the missing precondition
(document cache, snapshot-read grant, ...); neither line and no semantic-failure line -> the turn never returns from `pending_effects` (heavy `refresh_cache`), and per-iteration counters inside `refresh_cache` are the next probe.
Files: `🔄️turn/🌀️poll-kernel-turn/🦀️.rs`, `🔄️turn/🪜️entry-lifecycle-and-retirement/🦀️.rs`.

## Follow-up 18: `plugin_drive_displaced_surfaces` spins for the whole slice

Root cause (found by reading the loop, not the retirement): `RuntimeInstanceRegistry::next_entry_from(start)` (`🗃️instance-registry/🦀️.rs:180`) is a CYCLIC scan: after the last occupied slot it wraps to the first
one. `plugin_drive_displaced_surfaces` (`🧰️ops/🖥️surface-render/🦀️.rs:50`) iterated `while let Some((index,..)) = instances.next_entry_from(cursor) { cursor = index + 1; ... }` with no wrap guard, so with a
single live instance (one norm actor) the scan returns that same instance again and again; each pass does a cheap `retire_displaced_step` (nothing left to retire), and the loop only ends through
`if cx.should_yield() { return Ok(true) }`, i.e. after the entire 8 s slice, then reports `true` (more work), so the turn repeats. My earlier reading of the per-instance retirement was right; the loop around it was not bounded.
Every sibling caller guards the wrap (`pending_effects` has `if index < cursor { break }`, `plugin_drive_result_pages` and the typed-operation scan stop at `first == Some(index)`).

Fix: `if index < cursor { break }` at the top of the loop body, so each live instance is visited once per call and the step returns `pending` (false when nothing is displaced) instead of running to the deadline.
No counter needed. Checked the other `next_entry_from` users: publication and typed-operations guard with `first`; the cleanup-turn cursors take one entry per call.
Rebuild: Rust guest (all plugin components); no bridge/worker change. This also makes the earlier "more-work forever" cadence idle properly once opening and rendering finish.
File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs`.

## Follow-up 19: `UnsupportedOwner "original host operation admission refused the guest producer"` and `setActiveExample` backpressure

1. Which host call: the generated host shim (`🪢️host-shim-source/🟦️.ts`, staged `📕️norm/🟨️.js`) returned code 6, which `refusal()` (`🚪️io/🛂️authority/🦀️.rs:19`) maps to `UnsupportedOwner`. Code 6 has exactly one
   producer in the shim, `unsupportedProducer(phase, fields)`, reached from FOUR calls whenever the guest reports real, non-zero producer work: `operationProgress(completed,total,owned)` ("progress"),
   `operationAllocation(bytes,owned,next,maximum)` ("allocation"), `operationReserveReturn(kind,count)` ("canonical-return", count != 0) and `operationFinish(owned)` ("finish", owned != 0).
   The shim never implemented the receiving side the native host has (`🖥️host/🚪️io/🎛️operation/🐎️wasmtime`): it only admitted EMPTY producers, which was enough while every turn returned nothing. The first
   turn that carries a real result (a UI patch list or effects: `walk_turn` -> `operation_reserve_return(kind, count > 0)`, or identity/document encoding progress) is refused, and the bridge holds the raw
   `{phase, fields}` (`holdProducerRefusal`), which makes the next `issue()` fail with `source-not-ready`. Which of the four fired is in that held record (`originalProducer.phase`); all four are fixed below.
   The turn is long only because it now does real work; length is not the cause.
   Fix (host shim TS, restage or patch): the shim now ADMITS the producer against the turn's own grant instead of refusing it: progress/finish accept `owned <= maximumCapacityBytes`, allocation requires
   `maximum == maximumCapacityBytes` and `next <= capacity - owned`, reserve-return accepts `count <= maximumItems`; otherwise code 2 (OwnershipLimit), 4 for a mismatched allocation ceiling. The
   authoritative debit still happens in the guest's own ledger and the turn receipt, so this is only the host-side admission gate (the browser has no separate native storage to reserve).
   Staged patch (all five norm modules): `python3 <ticket>/norm-shim-producer-admission-patch.py .cache/b/p/m/browser/prototype/modules/<module>/🟨️.js ...` (parsed OK on a copy of the norm shim).

2. `setActiveExample` "command ingress backpressure after serialized submission": thrown by the host command drive (`🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts:370`, twin in the wgpu
   `🐚️plugin-bridge/.../🖼️render-and-turn-drivers/🟦️.ts:197`) when a turn answers `commandIngress.tag === "backpressure"`. In the guest (`⌨️command-ingress-turn/🦀️.rs`) `Backpressure` is the
   NORMAL "ask me again" signal: it is returned when the single fixed ingress owner is still assembling, when the lifecycle loan cannot be admitted this turn, or when `cx.should_yield()` /
   `fuel_remaining()==0` ends the turn mid-command (lines 58, 161, 240, 253, 255; the retained owner "waits as it always did"). With the new per-turn slice and a first open that is still running, the
   command arrives while the turn is busy, so backpressure is routine, and the client turned it into a fatal error. It should be waiting, not refusal: queued behind the running turn by the worker already
   (follow-up 16), and retried by the drive.
   Fix (renderer TS, restage; both drivers): backpressure no longer throws; the existing loop keeps submitting continuation turns up to `commandIngressContinuationCeilingV1`, and only then
   reports `command-ingress-stalled`. No staged-file patch (the renderer bundle is compiled into the shell, not a staged module file). Unverified: a page the guest never accepted is not re-sent by the
   continuation loop; if a command is ever dropped after backpressure, the drive needs to resubmit its pages on `backpressure` before the owner accepted any.
Rebuild: none for Rust. Restage: host shim (+ renderer bundle for item 2); quick patch for the shim above.

## Follow-up 20: `actor-poll.event-batch-carried` and the stalled `setActiveExample`

1. Batched crossing: protocol-correct fix is one event per crossing, with every answer handled; the batch path is removed.
   - `UiAcknowledgementHost.submitInstanceUiAcknowledgements` (`🎭️actor/📮️shard-client/📥️ui-acknowledgement/🟦️.ts`) no longer sends `states.map(patchAckEvent)` as one crossing. Each ack is its own
     `sendInstanceLifecycle` crossing, chained in order; per-entry memoization (`state.submission`) is kept, now per crossing. Its result type is `{ receipts, results[] }` (hub and activation interfaces
     updated; the single-ack wrapper returns `results[0]`). The wgpu intake (`🍽️ui-intake-and-route/🟦️.ts`) acts on EVERY result (`coerceTurnResult` + `accept` for each), so nothing a sub-turn answers with is dropped.
   - The shard worker's `turn` case no longer serves batches (my sequential sub-turn loop is gone): more than one event in a crossing throws `actor-poll.event-count` (a host defect), as the bridge and guest already do.
   Restage required (the client lives in the renderer/shell bundle). Worker staged patch (removes the sub-turn loop, parses on a copy):
   `python3 <ticket>/norm-worker-single-event-patch.py .cache/b/p/m/browser/prototype/modules/🧵️shard/🟨️shard-worker.js`. The client change has no staged file; until restaged the old batch client would now hit the
   one-event error, so restage both together.

2. Command stall (`command-pending`, more-work, `continuations=16/16`, statuses `backpressure, command-pending`): from the guest side this is progress, not a stuck owner. `drive_command_ingress` runs
   AFTER `step_restored_document_loads` in the turn (`🌀️poll-kernel-turn`), so while the example load / evaluation triggered by `setActiveExample` is being worked off, the early part of a turn can spend the
   slice and the ingress step sees `should_yield()` and answers `Backpressure` (`⌨️command-ingress-turn` lines 240/253); the next turn it resumes (`command-pending`). A slow prototype therefore needs many slices.
   The fixed 16-turn ceiling (`commandIngressContinuationCeilingV1(pages)`, a count) is the wrong bound for a retained owner that is making progress. Fix: in `🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts` the
   continuation loop now runs while `continuations < ceiling` OR the crossing is younger than `COMMAND_CONTINUATION_WALL_MS` (300 s), so the count is a floor and wall time the ceiling; the wgpu driver already allows 1,024
   continuations. A genuinely stuck owner still ends in `command-ingress-stalled` after 5 minutes. I did not add a progress probe on the guest side; if the stall persists after the wall bound, the cause is that
   each continuation turn never reaches the ingress step with slice left (then moving `drive_command_ingress` before the document-load step is the next change) and the `[DEBUG] turn phase after-command-ingress` timestamps show it.
   Restage the renderer bundle (no staged-file patch for the shell code).
Rebuild: no Rust. Restage: renderer/shell bundle (client + drive) and the worker (or the patch above).

## Follow-up 21: more than one event in a crossing from another path, and `original-turn.input-refused`

1. Other multi-event senders. All crossings go through one choke point, `ShardClient.turn(actorId, events, budget, commandPage)` -> `ShardExactActivations.captureActorActivation(...).turn`
   (`🎭️actor/📮️shard-client/🪪️exact-activation/🟦️.ts`); `sendInstanceLifecycle` (open, close, ack, poll) already sends 0 or 1 event, and `submitInstanceUiAcknowledgements` was fixed. The remaining multi-event senders are
   the callers of `ShardClient.turn` that pass a LIST: the wgpu plugin bridge `submitTurn(actorId, drive.takeAcknowledgements(), ...)` (7 call sites in `🖼️render-and-turn-drivers`, `📞️extension-and-request-actors`,
   `🧲️contributions-and-backbone`; typed-operation page acknowledgements queue up and are all taken at once), the PluginRuntime `submitTurn` (`🪢️handle-scope`) used by the command drive and `settleAcknowledgedPluginTurns`
   (`acknowledgements = [...accepted.acknowledgements, ...typedOperationAcknowledgements(...)]`) and the kernel `activation-registry` (`shardClient.turn(actorId, payload.events, budget)`).
   Root fix at the choke point instead of in each caller: the activation lease's turn now sends a multi-event batch as ONE CROSSING PER EVENT, in order (re-checking the activation between them, `commandPage` only on the last),
   and folds the answers into the single result its API returns (`mergeSequentialTurnAnswers`: effects and presence concatenate; last status/wake/lifecycle receipt win; the last non-idle command ingress wins; at most ONE answer may carry
   ui patches + their receipt, otherwise it fails loudly with `actor-poll.event-batch-carried`, since a patch page and its receipt are one unit). The worker and bridge keep rejecting multi-event crossings, so a new sender
   cannot slip through. Restage the shell bundle (no staged-file patch; the client is compiled into the shell).
2. `original-turn.input-refused` is thrown by the bridge `admitWire` (`🌉️component-bridge-source/🟦️.ts:145`) when `admitActorInput` answers with a refusal, `ready=false` or `disposition="returned"` for the poll's input wire (NOT the
   command page: pages travel as `commandPage`, not through `admitWire`). The guest (`🎟️admitted-input/🦀️.rs: admit`) returns the wire unadmitted for exactly four reasons: the turn is cancelled (`Canceled`), the wire exceeds the
   policy (`maximum_items < 1` or length > copy/capacity, `OwnershipLimit`), the single input slot already holds a wire (`InvariantViolated`) or an earlier turn output is still held (`outputs_held`, `InvariantViolated`).
   A source poisoned by the event-count failure is NOT the cause: that check ran in the worker before the bridge, so the source and guest were untouched. The two candidates that fit a long run are an occupied
   input slot (a previous wire admitted by a poll that never consumed it) and a held output (a turn result kept in guest custody after a refused return, as happens after a host refusal such as the earlier shim one);
   a cancelled source (`dispose` while a turn runs calls `originalSource.cancel()`) also yields `Canceled`. I cannot tell which from source.
   Diagnostic fix (bridge TS): the error now carries the reason: `original-turn.input-refused: refusal=<kind or null> ready=<bool> disposition=<kind>`. Quick test: `python3 <ticket>/norm-bridge-input-refused-text-patch.py <staged>/🌉️bridge.js` on each norm bridge (parsed OK).
   Reading it: `Canceled` -> a dispose/cancel race; `OwnershipLimit` -> grant exhaustion; `InvariantViolated` -> occupied slot or held output (the earlier failed turn left custody behind; then the root is that failure's cleanup).
Rebuild: none. Restage: shell bundle (client); bridge patch for the diagnostic.

## Follow-up 22: `original-turn.input-refused: refusal="ownership-limit" ready=false disposition=returned`

Which one: not a fixed "policy too small". The admission policy of `admitActorInput` is NOT a separate constant: the bridge passes `retainedPolicy = originalGrantWit(turn.input.grant)`, i.e. the operation's WHOLE REMAINING
grant of that epoch (`🌉️component-bridge-source`, `originalContextBudgetWit`), and `admit` (`🎟️admitted-input/🦀️.rs:58`) returns `OwnershipLimit` exactly when `policy.maximum_items < 1 || wire_len > policy.maximum_copy_bytes ||
wire_len > policy.maximum_capacity_bytes`. So either the wire is larger than the remaining copy/capacity grant, or the grant is (nearly) drained on one of items/copy/capacity. Everything else (cancel, occupied slot, held output) answers differently.

Candidates, by size of the numbers: items (8,388,608 stock) and copy (512 MiB) are unlikely to be gone after ~20 minutes at ~12 items / ~4 KB per turn (the 12 items / 4 KB refresh was measured). CAPACITY is the weak axis:
the stock still had `maximumCapacityBytes = 33554432` (32 MiB) because my earlier bump only touched items and copy, and capacity is a lifetime currency that only goes down: each admitted input wire is charged
`retained_capacity_bytes = wire length` (`admit`'s `spent`), the tables/documents the guest retains are charged, and the release currency is separate and does not refund capacity. A large example document (the "Compliant ETICS
dwelling" load) plus 20 minutes of wires can plausibly reach 32 MiB or leave less capacity than the next wire needs. I cannot distinguish "grant drained" from "one oversized wire" from source alone.

Changes (no renewal design yet):
- `stock.json`: operation `maximumCapacityBytes` and `maximumReleaseBytes` raised from 32 MiB to 512 MiB (pool 4 GiB covers 5 actors; valid wasm32 `usize`). If the cause was capacity, this removes it for ~16x longer.
- Diagnostics: bridge error text now also carries `wireBytes=<n> grant={items,copy,capacity,release,depth}` of the refused epoch (patch applies after the previous text patch, parses on a copy:
  `python3 <ticket>/norm-bridge-input-refused-grant-patch.py <staged>/🌉️bridge.js`), and the guest logs `[DEBUG] input admission refused: wire N B, policy items .. copy .. capacity .. depth .., cancelled .., outputs held ..`
  (`admit_input_wit`, Rust rebuild). Either line names the axis: wire larger than the remaining capacity/copy means drain (or a huge wire), `items 0` means item drain.
- The stock change only reaches the bridge through the shell bundle: `renderer-work/🔣️stock.json` is compiled into the renderer, so the running server must be restaged; a stale shell keeps issuing the 32 MiB capacity.

Renewal: not implemented. A sound minimal rotation needs both sides: at a quiescent boundary (no outstanding turn, no held output/claims, `closeOriginal` returns `sourceClosed`) the guest context must close (`closeActorContext`
retires the Source and its returns) and the host must admit a fresh operation/generation, then the guest must re-admit a new context under it; with the current guest, `validate_currency` pins operation, generation, clock binding and deadline
for the context's whole life, and the shard worker, bridge and renderer all cache the actor's `originalSource`. That is a cross-layer change (guest slot reuse, bridge, worker, client) and I am not going to guess it before the
drain is confirmed. If the new diagnostics show lifetime drain on any axis even with the larger stock, the smallest sound design is the guest-side per-epoch renewable grant: accept a NEW `RetainedTurnInput.grant`
larger than `remaining_grant()` when the host re-issues it, by re-basing `initial_grant`/`fresh_spent` in `begin` at a quiescent point; say so and I will implement that.
Rebuild: Rust (all components) for the guest log only; restage the shell (stock) and the bridges (or the two patch scripts).

## Follow-up 23: `din4108 has not answered "an interaction" for 141 s` (setActiveExample)

What `setActiveExample` does (guest, `🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:196` and `🧊️reserved-routes/🦀️.rs:422`): `A::catalogue_example_document(example_id)` returns the example DSL text,
`catalogue_snapshot_from_document` parses it, `snapshot.encode_pack()` packs it, and the command emits ONE effect, `Effect::LoadDocument { pack, spr }`, through `dispatch_emit`; the document replacement itself runs afterwards
through the host (cold-pair pages) and the guest's `step_restored_document_loads` / store replacement, i.e. in later turns. `dispatch_command` returns `Ok(None)` (command stays pending) while
`self.host_events.resumes.in_flight()` is true or while an earlier ingress/typing/emission owner is still being driven, which is the `command-pending` the shell reports.

Stuck or progressing: not decidable from source, but the evidence points to heavy inline work, not a dead owner: the command owner has no wait-for-host step of its own (its single effect is emitted, not awaited),
the earlier statuses alternated `backpressure` / `command-pending` (slice-bound, not a dead loop), and the new document is evaluated by compliance inline on wasm. The only unbounded-per-turn piece I found is that evaluation:
`Worker::spawn` on wasm32 (`⚖️compliance/⚡️incremental-evaluation/🧵️session/🦀️.rs`) runs the WHOLE `commit_background` plan inside one call, and the paint path (`live_report!`'s `responsive(document, PAINT_BUDGET = 16 ms)`,
used by `live_within`/`paint_report_of`) therefore evaluates every slow (T2) unit synchronously in the first paint of the new document, regardless of its 16 ms budget; `report`/`report_for` (the `live_report` default of the editors) also settles synchronously.

Fix (norm compliance, Rust): cooperative evaluation instead of one synchronous run. `SharedEvaluation::step_background(document, budget)` keeps the plan, cursor and serial (`Stepping`) between calls and advances unit by unit until `budget` has
elapsed (a unit is the granule); a superseding document restarts it, a settled or already-complete session returns immediately. `live_report!`'s `responsive` calls it on wasm32 (native keeps the worker thread), so each paint advances the
evaluation by its own budget and the report is the partially settled one meanwhile (`report_now`, pending units keep their previous results). Not done: (a) re-painting until settled: a non-thread target needs something to dirty the surface
while `settled()` is false (the transient `progress()` lane or a `more-work` signal), otherwise the report stays partial until the next unrelated paint; (b) the synchronous `report_for` / `live_report` readers (context menu, checks windows)
still settle in one go; (c) a single unit that is itself slow (the `live_fold_source!` fallback is ONE unit) is not split.

Diagnostics (temporary, Rust rebuild): `[DEBUG] more-work inputs: events_pending .. close_cleanup .. typed_runnable .. result_pages .. displaced .. typed_contended .. pending_effects .. reconcile .. resumes_remain .. document_load ..
cold_pair_loading .. command_ingress <status> lifecycle_work ..` once per turn before settle. A command that advances shows `document_load`/`resumes_remain` true with the status moving between `CommandPending`/`PageAccepted` and finally
`CommandComplete`; a stuck owner shows `command_ingress CommandPending` with every other source false (nothing left to drive it).
Rebuild: Rust (all norm components; compliance crate changed). The renderer crash at the 20-minute mark is unexplained (memory growth is the usual suspect: the guest memory budget is 512 MiB).

## Follow-up 24: `CommandPending` page 0/1 forever, and the copy grant falling from 512 MiB to 59 bytes

1. Not a missing page. `page_index 0, page_count 1, item_count 0` is the normal cursor of a single-page generic command AFTER it was accepted: `continue_generic_assembly` (`⌨️command-ingress-turn/🦀️.rs`) takes the page,
   converts the pages into `CommandIngressOwner::Generic { command: Some(Encoded(..)), exchange: Some(empty), .. }` and calls `exchange_generic`, which sets `status = CommandPending(cursor)` and then runs
   `plugin_exchange_turn` on every later turn until `state.applied`; only then it routes the output and the status becomes terminal. So the owner is waiting on the EXCHANGE (decode, dispatch of `setActiveExample` and its application), not on host pages,
   and a re-sent page would be refused as `plugin.command-page-order`/superseded. Why the exchange does not advance is not visible from source: `dispatch_command` (`🧷️command-routes/📥️staging/🚦️actions`) returns `Ok(None)` (pending) while
   `self.host_events.resumes.in_flight()` (a typed-source delivery started or closing, `🎟️typed/🔁️resumes`), while a typing/emission/host-configuration owner is active, or while `stage_action_source`/`loan_source` is not ready; none of those is reported by the
   `more-work` sources (they all showed false), which is exactly the stuck shape you found.
2. The ~500 MiB of copy is the per-turn overhead, not a re-copied page or pack: ~4,105 B and 12 items per turn (the origin refresh, follow-up 6) x ~125,000 turns = ~512 MiB and ~1.5M items, matching both numbers
   (items fell ~1.5M of 8.4M, copy fell to 59 B, capacity is barely touched). Those 125k turns come from my continuation change: the host drive re-asked a command that could not advance at millisecond cadence for the 5-minute
   wall bound (follow-up 20) while every turn charged the lifetime grant. The page and the `encode_pack` result are built once, in the first command turn, and are not re-copied by retries.
   Fix (TS, restage): both command drivers now pause 25 ms between continuation turns (`COMMAND_CONTINUATION_PAUSE_MS`), 100-1000x less spend for a stuck command and no cost for one that progresses (the guest, not the host, paces a
   progressing command). The structural cost is still the 4 KB/12 items of refresh per turn even for turns that need no return budget; the clean fix is to skip the origin refresh unless the turn's business needs the table
   (`birth_requested` is never cleared), which changes when claims make progress (one extra turn each time) and I did not do it blind.
3. Diagnostic (temporary, Rust rebuild): per turn `[DEBUG] exchange_generic: seq N command <encoded|decoded|failed-decoded|none> output <bool> applied <Some(bool)>`. `encoded` forever means the exchange never starts
   (a gate in `dispatch_command` returns pending: look for `resumes` in flight or an unfinished earlier owner); `decoded, applied Some(false)` forever means the dispatch runs but never applies (a wait inside the command/emit path); `applied Some(true)` then
   a status change means it completes and the stall was the stuck host drive. Next step, once that line is read: probe the named gate in `dispatch_command`.
Rebuild: Rust (all components) for the diagnostic; restage the shell (drivers). No staged-file patch for the shell code.

## Follow-up 25: `command decoded output true applied None` forever

Reading the sequence against `exchange_generic` (`⌨️command-ingress-turn/🦀️.rs`): `output true` means the command already ran and its semantic output exists, so `plugin_exchange_turn` is skipped; `applied None` means the exchange state is
already GONE (`close_exchange_owner` took and retired it; the debug line reads `exchange.as_ref().map(|s| s.applied)`). The function is therefore in its RETIREMENT tail, and nothing about the effect, host acknowledgements or a cold-pair load is awaited yet:
the tail is, in order, (1) `close_exchange_owner(exchange, exchange_close)`, (2) `OriginalDecodedPluginCommand::close_original_step` for the retained decoded command (its wire, receiving owners, media receivers, `🏃️runtime/🎞️command/📥️custody/🛂️manifest`),
(3) `command.take()` behind a grant check, and only then (4) `output.take()` -> `terminal_command_ingress` -> `route_exchange_output`, which is what finally emits `Effect::LoadDocument` and completes the command.
Steps (1) and (2) are ONE paid step per call (`close_original_effect_step` / `close_original_step` return `false` after each step, even when the step made progress), and the function returns `Ok(())` as soon as one reports `false`, so a retained
decoded command whose wire holds the example pack (many pages/frames) needs one TURN per retirement step; each turn costs the ~4 KB / 12 items refresh plus the host's per-turn latency (now 25 ms plus the drive), so the stall notice (287 s)
arrives long before routing. The `cold_pair_loading`/`document_load` flags are false because the load has not been requested yet: the effect is emitted in (4).
Which of (1)-(3) is the slow one is not visible from the log; (3) can also stay false when `retained_grant()` is below its small thresholds, which is not the case here (grant is large).

Fix (Rust, command ingress): a retirement tail that makes physical progress now repeats within the turn's own slice. `step_until_slice(cx, step)` runs the step again while it returns `false`, the slice has not ended
(`cx.should_yield()`, deadline-aware) and the previous step spent something (`cx.retained_progress()` changed); it stops on completion, slice end or a no-progress step (so a genuinely blocked step still yields at once instead of spinning).
Steps (1) and (2) use it. A multi-frame command now retires over a few turns bounded by the slice rather than one step per turn.
Diagnostic (temporary): `[DEBUG] exchange_generic: input retired, routing the output` right before step (4); if the existing `exchange_generic ... output true applied None` lines continue without it, the block is in (1), (2) or (3) and
the next probe is the return value of each (add `closed` booleans).
Not changed: the per-turn 4 KB refresh and the host cadence. Rebuild: Rust (all components). No restage needed for this fix (the 25 ms pause from follow-up 24 stays).

## Follow-up 26: seq 7 (`setActiveExample`) logs `encoded` then `decoded` and then nothing

Reading `exchange_generic` again: the debug line sits AFTER the `FailedDecoded` branch, so a command that has FAILED logs nothing on later turns. The silence after `decoded` is exactly that shape: the dispatch inside `plugin_exchange_turn`
returned `Err(failure)`, and the `Err` arm of `exchange_generic` swallows it for a Decoded command: `retain_decoded_exchange_failure(command, Some(failure), ...)` (`🏃️runtime/🔄️exchange/🔀️plugin-exchange`) re-labels the command `FailedDecoded`
and returns `Ok(())`; every later turn then takes the early `FailedDecoded` branch (retire the exchange owner, `retire_failed_step_on_original`, `return Ok(())`) without a log line, and only when that retirement completes is the failure routed to the host as the
command fault. So seq 7 is most likely a REFUSED command being retired step by step, not an apply waiting for a wake; the cause of the refusal is not visible anywhere yet.
(The example itself is small: `🎬️demo/🗣️.dsl.semio` is 3,982 bytes, so parse and `encode_pack` are not heavy.)
New temporary probes (Rust rebuild): `[DEBUG] exchange_generic: command seq N refused: <full OriginalTurnFailure>` in the `Err` arm, and `[DEBUG] exchange_generic: seq N failed-decoded, retiring` once per turn in the failed-command branch.
`boot-probe-phases.mjs` (ticket folder) is `boot-probe.mjs` with the regex extended by `turn phase|more-work inputs|reconcile loop|input admission|[DEBUG] poll`; `exchange_generic` already matches the two new lines.

Probe run on the currently staged norm (`boot-probe-phases.mjs`, 420 s, log in `🗑️generated/probe-phases.log`, last 90 console lines only): turns now complete in ~1-3 ms with every phase present, no refusals/panics, and every turn ends with
`more-work inputs: ... displaced true ...` (all other sources false). `plugin_drive_displaced_surfaces` therefore reports `true` forever: `SurfaceContexts::retire_displaced_step` (`⚛️reactor/🪟️surfaces/native.rs:80`) returns `Ok(false)` while
`self.cached_capture.is_some()`, i.e. a surface projection captured for a patch that was never released; that capture is released when the host acknowledges/consumes the published UI patch, so a missing patch acknowledgement (or a
render whose patch the shell never accepted) keeps the guest in permanent "more work" at ~1 ms turns, each charging the ~4 KB/12-item refresh to the lifetime grant. This is a second, independent stall candidate next to the failed-command retirement
above; the tail of the log did not contain `exchange_generic` lines, so the seq-7 state could not be confirmed from it. Next probe after the rebuild: `refused:`/`failed-decoded` lines (command) and a `[DEBUG]` of `cached_capture` presence in `retire_displaced_step`
if the command lines are clean.

## Follow-up 27: displaced-surfaces probes (added before the norm build)

Crate `semio-plugin-reactor-surfaces` (`⚛️reactor/🪟️surfaces`, already depends on `semio-plugin-abi`); nothing in the window-config crate.
- `[DEBUG] displaced: cached_capture <Option<usize>> displaced <n> retiring <bool> retirement_empty <bool> closing_body <bool>` from `retire_displaced_step` (`native.rs`), logged only when that tuple CHANGES (thread-local last state), not per turn.
- `[DEBUG] surface capture taken by reader 0x.. selection ..` where a cached view reader takes the registry capture (`read.rs`, phase 0) and `[DEBUG] surface capture released by reader close` where it releases it (`read.rs`, `locked` close step) - the release path of `cached_capture`.
Reading: `cached_capture Some` forever with a `taken` line and no `released` line = the reader that took it is never closed/finished (its owner, the render or `OriginalSurfaceRefresh` driver, is stuck or abandoned it), which blocks both `retire_displaced_step` and
`OriginalSurfaceRefresh::advance` (both return `Ok(false)` while `cached_capture.is_some()`), and would explain the permanent `displaced true` and a command that waits on its refresh. No fix yet: the capture is guest-internal (released by the reader's own close step), not by a host
acknowledgement, so a missing host ack is not the mechanism; who should close the reader is only visible once the probe names the stuck reader.

## Follow-up 28: seq 7 fails without a `refused:` line; its failed-command retirement never ends

(1) Where the failure comes from: `FailedDecoded` is built from FOUR call sites of `retain_decoded_exchange_failure` (`exchange_generic`'s `Err` arm, which did not fire, plus three inside `plugin_exchange`, `🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs:157/259`):
the one that fits seq 7 is line 157: `owner.advance_manifest_step(...)` returned `Err(failure)` in the SAME turn that logged `decoded`, the exchange retained it (`retain_decoded_exchange_failure(command, Some(failure), ..)`) and returned `Ok(default)`, so the next turn
sees `failed-decoded`. `advance_manifest_step` (`🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs:217`) has several literal refusals for a decoded command; the ones reachable by `setActiveExample` are: "original ArtifactCommand action is not owned by this command
channel" (only the six history verbs `DOCUMENT_COMMAND_ACTION_IDS` pass the artifact route), "original action plugin owner is foreign" (`program.manifest.plugin_id != address.plugin_id`), "original command mode is inactive", metadata/limit refusals
(`max_total_alloc`, `max_depth`) and anything `handle_action_invocation` returns. Seqs 2, 4, 5, 6 were other commands, so seq 7 is the first `setActiveExample` through this route; I cannot tell which refusal it is without the text.
Probe added (turn crate, no new dependency): in the failed-command branch of `exchange_generic` the first turn of a failed seq logs `[DEBUG] exchange_generic: seq N FAILED with <Option<OriginalTurnFailure> Debug>` (the real cause) and every turn logs
`[DEBUG] failed-decoded state: identity .. cx (op,gen) .. owner_terminal_empty .. close_failure .. compound .. raw_media .. yield .. fuel .. grant .. exchange_open ..`.
(2) Why the retirement never ends: `PluginCommandIngress::advance_original` for `FailedDecoded` (`🧰️ops/📥️ingress/🦀️.rs:213`) returns `Pending` WITHOUT progress in exactly two cases that depend on state the turn cannot change:
`identity != (cx.operation(), cx.generation())` (the identity captured with the failure no longer equals the borrowed context's) or `cx.retirement_should_yield()` (fuel exhausted); it also stays `Pending` forever if `owner.terminal_is_empty()` never becomes true
(`close_original_step` of the decoded command, the step that returned `false` per call), and it returns `Pending` when the grant has `maximum_items == 0 || maximum_depth == 0`. The state line above prints every one of these conditions per turn, so the next run names which gate holds.
I did not change the retirement logic blind: the first two conditions are guards against retiring under a foreign context and loosening them would be unsound; if `identity` differs it is a bug in what `retain_decoded_exchange_failure` captured and the fix is there.
Rebuild: Rust (all components; nothing else changed in this round).

## Follow-up 29: seq 7 pending forever after the deadline slice was removed

Noted: the adaptive wall slice changed `slice.deadlineUs` per turn while `OriginalManifestCommandReceiving::validate` (`🛂️manifest/🦀️.rs:55`) and six other guest `validate()` sites pin the deadline as part of the operation identity, so a per-turn deadline can never be used;
fuel bounds the turn. I will not re-add a deadline-changing slice. (My earlier note that the slice was safe because the guest accepted "one consistent value" was wrong for any value that varies per turn.)

Static review of `OriginalManifestCommandReceiving::advance_step` and the path after `decoded` (every `Ok(false)` that does not advance under a constant grant, fuel and deadline):
- `loan.is_none()`: `admit_loan` returns false until the caller-source loan is funded (copy/items of the loan demand); constant grant that is below the loan demand never advances. Needs `maximum_items>0`, copy >= the loan size, depth>0.
- `cx.should_yield()` (fuel exhausted or deadline passed): constant-false only if fuel and deadline are untouched; it is the guard on every phase.
- `raw_failure`/`raw_place` set: `admit_raw_failure` returns `Ok(None)` (-> `Ok(false)`) until its place is ready; it is paid step by step and finishes, or errors.
- Every `paid(cx, bytes)` (phases 0 end, 1, 2, 3, 4-6, address build, 12, 13 and the action fields): `Ok(false)` forever when `grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 || should_yield()`.
- Phase 8 (argument vector): needs `grant.maximum_capacity_bytes >= (n+1)*size_of::<(String,DslValue)>()` and items/depth non-zero; phases 10/11: capacity >= text length and a copy grant > 0 for the chunked copy; the copy loop returns `Ok(false)` when `copied == 0`.
- `advance_manifest_step` after `advance_step`: `OriginalWindowViewBinding::admit`/`projection.advance`, `OriginalSurfaceRefresh::advance` (returns `Ok(false)` while `registry.cached_capture.is_some()`, see follow-up 27) and the metadata `advance(actor, ..)` each also gate the next phase.
None of these can loop on a constant grant with budget left; all of them stall permanently only if (a) a grant axis is exhausted (items, depth, copy below the size of a `DslValue`/loan) or (b) fuel/deadline is exhausted, or (c) the surface refresh waits on `cached_capture`. I found no phase that fails to advance when funded, so I made no logic change: the missing fact is which gate holds,
and the exhausted-grant theory is checkable from the new line.
New probe (temporary, runtime-ops crate which already depends on `semio-plugin-abi`; logged only when the tuple changes): `[DEBUG] manifest advance_step: phase N kind K yield <b> items0 <b> depth0 <b> loan_none <b> raw_failure <b> grant {..} fuel F deadline_us D parser .. args .. action .. metadata ..`.
Reading it: `loan_none true` with items0/depth0 -> loan funding; `yield true` -> fuel/deadline; `phase` stuck at a number -> that phase's `paid`/capacity gate (compare `grant` with the sizes above: `DslValue`, `String`, `ActionAddress`, loan); no line at all after the 9th `decoded` means `advance_step` is not being called, i.e. the turn never reaches `exchange_generic` and the gate is in `resume()`/`lend` of
`drive_command_ingress` (the lend of the actor return slot pays `size_of` bytes with the same `items/copy/depth` rule).
Rebuild: Rust (norm components). Nothing in the window-config crate.

## Follow-up 30: `advance_step` never reached for seq 7 (gate probes)

Trace of the path between `decoded` and `advance_step`: `drive_command_ingress` -> `resume()` (settle_step of the live ingress slot, lend of the actor return slot: both return `false` without progress when the grant/yield rule of `pay` fails or an earlier owner is still being settled) -> `admit` of the source's command origin
-> `exchange_generic` -> `plugin_exchange` (1. `store_constructor_is_pending` returns an empty output, 2. `advance_original` of the encoded command: `Pending` returns an empty output, 3. for a decoded command `owner.command_seq.is_some()` -> `advance_manifest_step`).
In `advance_manifest_step` the FIRST call only admits the receiving owner (`OriginalManifestCommandReceiving::admit`) and returns `Ok(None)`; `admit` returns `Ok(None)` again, and the next turn repeats it, whenever its funded step cannot run (`funded(cx, d, ..)` skips the closure when the grant does not cover
`d` = the retained pack parser body + a `Vec`, or `should_yield()`); `created` stays `None`, `receiving` stays `None`, `advance_step` (and its probe) is never called. That is the best static fit for "decoded 9 times, advance_step 0 times"; the other candidates are the gates listed first.
I cannot see the grant at that point, so no logic change: one probe per gate, each logging only when its state changes (new helper `debug_state(point, detail)` in the turn crate; `gate <point>:` lines):
- `gate ingress.resume: ready <b> page <b> grant {..} yield <b> fuel <n>` (every `resume()` outcome, `drive_command_ingress`),
- `gate ingress.command_origin_admit: admitted <b> grant {..}`,
- `gate exchange.produced: seq N applied <b> frames/effects/events <n> constructor/preparation/retirement <b> typed_pages <n>` (what `plugin_exchange` returned: all zero/false = it returned an empty output at an early gate),
- `gate manifest.admit: receiving admitted <b> grant {..} yield <b> fuel <n>` (logged when the admission result changes; `admitted false` = the funded step of `OriginalManifestCommandReceiving::admit` cannot run).
Reading: `ingress.resume ready false` -> the gate is the ingress slot/lend; `exchange.produced` all-empty with `manifest.admit admitted false` -> the receiving admission is unfunded (compare `grant` with the parser body size, `size_of::<RetainedIntrinsicBody>`); `exchange.produced` empty and no `manifest.admit` line ->
the exchange returned before reaching it (`constructing` or `command_seq` none).
Rebuild: Rust (norm). Nothing in the window-config crate.

## Follow-up 31: `gate ingress.resume ready false page false` under a large grant, every turn

Reading `resume()` with the data (`⌨️command-ingress-turn/🦀️.rs`): it calls `settle_step` of the live ingress slot (`OriginalActorReturnSlot::settle_step`, `🎭️actor/🎟️retained-turn/🎟️context/📦️return/🦀️.rs`) and returns `false` BEFORE the lend. The page-less continuation turns therefore never reach `exchange_generic`
(that is why `decoded` is logged only ~9 times in ~9,981 turns). `settle_step` returns `Ok(false)` without spending in exactly these cases, none of which depends on the grant (items 8.2M, copy 498 MB are ample; each turn only pays the 12-item refresh):
(a) `self.target.borrowed`: the owner was lent and not yet restored (restore runs in `CommandIngressTurn::finish` / `Drop`);
(b) `settle_place(home, ..)` or `settle_place(donor, ..)` is false: `OriginalCallerReturnPlace::settle_return` (`⏱️context/📦️return/🎟️place/🦀️.rs:52`) issues the return witness and calls `receive_original_return_witness`, and it returns `Ok(false)` until the claim's credit is `terminal_is_empty()` i.e. until the
return-escrow row of that claim is fully settled; the escrow table has only `ORIGINAL_RETURN_ESCROW_CELLS = 32` rows and a row becomes reclaimable only after it is ACKNOWLEDGED (`mark_local_received` for local witnesses, or a host hand-back/`release_resident_receipt` of the published core-wire header for `CoreWireSnapshot` delivery, which the plugin bootstrap selects in `admit_plugin_step`);
(c) the final `pay` (grant, which is ample).
The component (jco) host of the browser never hands core-wire headers back, so a row claimed per lend/return would stay unacknowledged; with several rows per ingress return, 9 returned turns plus the earlier commands (seq 2..6) fit the 32-row table's exhaustion pattern, after which every claim/settle waits forever.
That is a hypothesis from structure and the "9" - the fix it points to (acknowledge settled rows locally on the WIT path, or select local-witness delivery there) changes the return protocol of every component and I will not do it without the rows' state; the probes below decide it.
New probes (turn crate, `debug_state`, logged once per change): `gate ingress.settle_step: ready <b> slot_terminal_empty <b> owner_present <b> grant {..}` (owner_present false with a non-terminal slot = (a) or (b); true = the owner is home and settled),
`gate ingress.peek: allow_empty .. owner_present ..`, `gate ingress.lend: lent <b> grant {..}` (reached only when settle_step is ready).
Next step after the log: `ready false` with `slot_terminal_empty false` and `owner_present false` -> add a probe of `OriginalReturnBudget` table rows (states/acknowledged) at the claim that waits; if rows are all `state 3` and unacknowledged the 32-row exhaustion is confirmed and I implement the local acknowledgement for the WIT path (`acknowledged |= settled rows` when the host cannot hand headers back).
Rebuild: Rust (norm). Nothing in the window-config crate.

## Follow-up 32: escrow row probe and the local-acknowledgement fix (same build)

Confidence the 32-row escrow exhaustion is the stall: ~60-65 % (structural fit: `CoreWireSnapshot` delivery chosen in `admit_plugin_step`, rows acknowledged only by `mark_local_received` (LocalWitness) or a host header hand-back that the jco host never performs, 32 cells, ~2 rows per ingress lend/return, stall at the 9th turn of seq 7 after seq 2-6). Because one build costs ~30 min, the fix ships with the probes and is isolated so it can be reverted by deleting one call.
Probes (once per change, `debug_state`): `gate ingress.escrow_table: ready <b> states [..] acknowledged 0b.. guard 0b.. free <n> stage .. borrowed ..` at every `settle_step` outcome and `gate ingress.escrow_table_after_lend: lent <b> <same>` after the lend
(`OriginalReturnEscrowTable::debug_summary`, `StepContext::return_table_summary`); together with `ingress.settle_step` / `ingress.peek` / `ingress.lend` of follow-up 31. All rows `3` with acknowledged 0 and `free 0` confirms the exhaustion; `free > 0` refutes it.
Fix (guest, job crate + reactor context): the return budget gets `local_acknowledgement` (`OriginalReturnBudget`, default false). `admit_plugin_step` (the component/WIT admission) sets it once per call (`StepContext::acknowledge_return_rows_locally`). While it is set, `claim_original_sized_return_inner`
(`🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs`) first runs `OriginalReturnEscrowTable::acknowledge_settled` (`🪙️return-escrow`): every row in state 3 (settled) without an issued header guard, not staged and not borrowed gets its acknowledged bit, so the existing `reclaim_one` path frees it when the table is full. Rows with a published header (`guard_issued`) keep waiting for a real
host hand-back, so the owned/native ABI behaviour is unchanged (the flag is only set on the component path). If the probe refutes the theory the flag is harmless (it only acknowledges rows nothing will ever acknowledge on that path).
Rebuild: Rust (norm and every component; job crate and reactor changed). Nothing in the window-config crate.

## Follow-up 33: the lend is the gate (`lent false`) with 1 free row, rows 0-2 in state 2

Reading the code against the table dump: the "needs more rows than are free" deadlock is NOT possible. `OriginalReturnEscrowTable::reserve` (`🪙️return-escrow:177`) takes ONE free row (`position(state==0)`); the lend claims two places (home, donor) through `claim_original_sized_return`; with 1 free row the first claim succeeds, and the second sees `!has_capacity()` -> the existing `full` branch
reclaims one acknowledged state-3 row (`reclaim_one`) and the claim proceeds on the next call. With rows 4-31 acknowledged and `guard 0` they are reclaimable, so the table is not what blocks. Rows 0-2 in state 2 are LIVE claims (reserved, not settled): the home/donor claims of the current lend attempt plus an older one; rows 0-3 cycling 2/0/3 is those claims being reserved, cancelled (`cancel_preissued_capture`) or settled turn after turn.
What the lend (`OriginalActorReturnSlot::lend_step`) needs, in order, besides the table: `pay(size_of binding + usize)`, `pay(usize)`, for home and donor `prepare` = `admit_sized` (claim: `cx.retained_grant()` items>0/depth>0, `retained_recipient_is_admitted`, THEN against the RETURN BUDGET'S OWN STOCK
`available = budget.remaining()`: `items >= 3`, `copy >= copy_bytes + overhead + row_copy` and `fuel >= 3`, where `copy_bytes` is the size of the DESTINATION (`ActorReturnTarget<RetainedCommandIngress>`, which embeds the whole command ingress owner, possibly large) and holds of the live claims are subtracted),
`preissue_capture` (2 items, a few bytes, 2 work), and a final `pay(size_of target + size_of Option<T> + bool)`. The cx grant shown (486 MB) covers the `pay`s; the return budget stock is a DIFFERENT, smaller ledger (and it cannot renew while claims are live, `renewable()` false, so it only drains), and it is the only check in this chain that can fail under the printed grant.
Probe added (same build): `gate ingress.return_budget_after_lend: lent <b> remaining Ok((items,copy,fuel)) raw Ok((..)) held_items .. held_copy .. held_fuel .. claims .. serial .. core_wire <b> renewable <b> yield .. fuel ..` (`OriginalReturnBudget::debug_summary`, `StepContext::return_budget_summary`). Compare `remaining` with `3 items / copy_bytes+overhead+row_copy / 3 fuel`: `remaining` below them (or an `Err`) is the gate;
if it is ample the gate is `retained_recipient_is_admitted` or the `pay`/`preissue` of the lend. The existing `ingress.escrow_table*` probes stay.
I did not change reclaim logic: eager reclaim would not change this outcome and the claim path already reclaims when no row is free. Rebuild: Rust (norm).

## Follow-up 34: prepared (NOT applied) fix for "return budget stock exhausted while claims are live"

Sources are frozen during the norm build, so this is a script only: `prepared-fix-return-budget-renew-with-claims.py` in the ticket folder (`--check` verifies its two targets exist (they do, checked now), `--apply` edits `🧵️job/📬️outcome/🪙️budget/🦀️.rs`).

How the stock is born and renewed on the component path (source reading):
- Birth: `admit_plugin_step` -> table/publisher birth -> `birth_origin_action` -> `OriginalReturnBudget::stage_initial_origin(input)` with `input = OriginalReturnSourceInput::capture(cx)` (`⏸️continuation/🎟️origin`/`🎟️source`): `granted = cx.original_total_retained` (the bridge's remaining grant of that epoch), `fuel = original_total_fuel`,
  `spent`/`work` = what this epoch already spent. `commit_origin` stores it and, for the FIRST origin, adopts `spent`/`work`.
- Every later turn pays an origin refresh (`refresh_turn` + `commit_origin`, 9 items / ~4 KB / 9 work, part of the 12-item per-turn cost): the new origin carries the NEW epoch's `granted` (the bridge's total, already net of all earlier spend) and `spent`; `commit_origin` adopts `spent`/`work` only if `origin_prior.is_none() || renewable()`,
  and `renewable()` requires no live claim and no held currency. `renew_turn()` is the same operation and is only called by the testkit.
- Consequence while a claim is live (the ingress owner's home/donor return claims are live on every lend attempt, `claims > 0`): `granted` is re-based every turn but `spent` keeps ACCUMULATING from the last renewal, so every turn deducts the whole history again from a total that already excludes it; `remaining() = granted - spent - held` falls faster than the real grant, and the
  per-claim holds (`copy_bytes` = size of the destination `ActorReturnTarget<RetainedCommandIngress>`, 2 items, work 2, plus overhead) are subtracted on top. Once `remaining` is below `items>=3 / copy>=destination+overhead+row_copy / fuel>=3` the claim in `claim_original_sized_return` returns `Ok(false)` and the lend never completes (the gate of follow-up 33).
- The per-turn refresh is not the culprit by itself (it is the 4 KB/12 items every turn, charged exactly once to the real grant); the double deduction is.
Prepared fix: re-base the ledger on every refreshed origin whenever the currency held by live claims still fits that origin (`holds_fit_origin`: `spent + held <= granted` on items, copy, release, capacity and `work + held_fuel <= fuel`). Held currency stays owned by the outer total, so conservation holds, claims keep their serial/rows and later redeem against the re-based ledger; if the holds do not fit the new origin the ledger is left as it is (current behaviour).
Apply when the `gate ingress.return_budget_after_lend` line shows `renewable false` with `claims > 0` and `remaining` below the lend's needs (or `raw` much smaller than the bridge's grant): then run `python3 <ticket>/prepared-fix-return-budget-renew-with-claims.py --apply` and rebuild norm. If the line shows an ample `remaining`, this is NOT the gate and the fix should not be applied.
Alternative if the probe shows the destination size itself is the problem (`held_copy` huge): shrink the claim, i.e. box the command-ingress owner inside the slot (`ActorReturnTarget<Box<RetainedCommandIngress>>`) so the sized claim is a pointer, not the owner; larger change, not prepared.

## Follow-up 35: every early return of the lend chain tagged (`gate ingress.lend_step`)

Data read: return budget ample (`remaining` 8.2M items / 495 MB / 49.5M fuel, `core_wire true`, 3 claims, held 11 items / 66,378 B, serial +1 per turn). The serial rising each turn is also the poll's own result claim, so it does not prove the lend reserves and drops a claim; but rows cycling 2/0/3 with `claims 3` is consistent with home/donor claims being settled by `settle_step` and
re-admitted by the next lend. The prepared renewal patch is NOT applied (budget is not the gate).
Instrumentation (one line, once per change): `gate ingress.lend_step: lent <b> reason <tag> claim_reason <tag> cancelled <b> should_yield <b> retirement_active <b> grant {..} fuel <n>`.
`reason` (set by `OriginalActorReturnSlot::lend_step`, `🎭️actor/🎟️retained-turn/🎟️context/📦️return`) is one of: `settle_step_false`; `pay_address:{cancelled|should_yield|items0|copy|depth0|fuel0|retirement_should_yield}`; `pay_donor:..`; `home:admit_sized_false` / `home:preissue_capture_false`; `donor:admit_sized_false` / `donor:preissue_capture_false`; `final_pay:..` (same sub-tags); `lent`.
`claim_reason` (set by `claim_original_sized_return`, `🧵️job/📬️outcome/⏸️continuation/🎟️source`, read through `StepContext::last_claim_reason`) is one of: `cancelled`, `should_yield`, `retirement_should_yield`, `recipient_not_admitted`, `items0`, `depth0` (the first gate), `table_absent_birth_requested`, `table_full_nothing_reclaimable`, `table_full_reclaim_copy`,
`reclaim_one_false`, `reclaimed_one_row`, `stock_items_below_3`, `stock_copy_below_needed`, `stock_fuel_below_3`, `cx_copy_below_row_copy`, `table_reserve_none`. `pay` and `prepare` were split into `pay_why`/`prepare_why` with the same conditions (behaviour unchanged).
What I can see from the code: every `pay` and the claim's first gate also stop on `cx.is_cancelled()` (when no retirement is active), so a cancelled actor token (the shell's "Cancel command", a dispose, or a token cancelled by the earlier failed command) makes the lend, and every other paid step, return `false` forever under any grant; the new line prints `cancelled`
so this is decided by the first tagged turn. Not changed: any logic of the lend.
Rebuild: Rust (norm; job crate + actor retained-turn crate + turn crate). Nothing in the window-config crate.

## Follow-up 36: livelock between reclaim and claim (`donor:admit_sized_false`, `claim_reason reclaimed_one_row`)

Diagnosis confirmed by the tagged line: with the table full of acknowledged rows, `claim_original_sized_return` reclaimed ONE row and then returned `false` ("retry next turn"); between turns another claim (the poll's own result claim or the home re-claim after `settle_step`) took the freed row, so the donor never got one.
Fix (`🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs`, `claim_original_sized_return`):
(1) after a successful `reclaim_one` the claim no longer returns; it falls through and reserves in the same call (the `reclaimed_one_row` tag is gone);
(2) on the local-acknowledgement path (component admission) every claim first acknowledges the settled rows and then eagerly reclaims ALL acknowledged, unguarded state-3 rows (up to 32 per call), each paid like the single reclaim (1 item + `ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES`, 1 fuel) and bounded by grant/fuel, so the table no longer sits full of dead rows.
A concurrent helper `reserve_original_return_rows` (rows-before-claim, uses `table.free_rows()`) appeared in the same file from another editor; I left it untouched. Probes kept (`ingress.lend_step`, `escrow_table`, `return_budget_after_lend`). Rebuild: Rust (norm).

## Follow-up 37: `original total exceeds its persistent source recipient` by 11,740 copy bytes after the lend works

Data: bridge total copy 530,814,008 vs guest ledger copy 530,802,268 (the guest has spent 11,740 B of copy that the bridge never charged), while the bridge charged 136 MORE items and 12,549 fewer capacity bytes than the guest ledger shows.
Conservation of the eager reclaim: it is neutral on both sides. `reclaim_one` only flips a row state (no ledger), and each reclaim posts `consume_retained(1 item, 12 B = ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES = size_of::<u32>() + size_of::<u64>())` through the context, i.e. into the same aggregate that the call's receipt reports, exactly like every other paid step; neither 11,740 nor 136 is a multiple of the reclaim cost (11,740 / 12 is not an integer), so it is not the reclaim.
The drift has the signature of the earlier family (follow-up 6, 10): guest spend recorded in `fresh_spent` that no receipt reported to the bridge, now of a different kind, while the bridge was charged items it did not report as copy. It first appears once the command lend/return and the redeemed return witnesses (reserved returns, charged to the bridge as items but excluded from the fresh receipt) run every turn; candidates I could not separate from source: reserved-return progress that carries copy bytes and is
dropped from the receipt (`kernel_work_receipt` reports `aggregate - reserved`), spend posted after the receipt is computed in `poll_kernel_turn`, and failed-poll receipts that the bridge never debits (`failure.input` vs the jco `payload`, see follow-up 24 note).
Fix (robust, sound): `StepContext::bind_original_retained_total` (`🧵️job/⏱️context/🦀️.rs`) no longer refuses a bridge total that is larger than the persistent ledger; it binds the per-axis MINIMUM of the two (`min(total, ledger)` on items, copy, capacity, release, depth). The guest then never spends more than either authority allows, the epoch receipt still returns against the bridge's own grant, and a bridge that under-charges by a few KB out of 530 MB cannot kill the actor.
The drift itself is NOT fixed (a leak of ~11 KB per ~9 turns would exhaust nothing, but it is an accounting error): next probe if you want it found: log `input.grant - ledger` per axis once per change in `poll_wit` (`debug_currency`) and the reserved-return progress per turn.
Probes kept. Rebuild: Rust (norm).

## Follow-up 38: the failed command's retirement kills the source with its own failure

"staged original metadata is absent" is NOT raised by the retirement: it is the command's own failure (the literal is in `advance_manifest_step`, the action path `receiving.metadata....meta.as_mut().ok_or_else(|| refusal("staged original metadata is absent"))`: the metadata owner exists but the `ActionMeta` it should stage is absent for this `setActiveExample` action, which is very likely
the seq 7 FAILED reason that was truncated). The retirement then completes in the guest's failed-command branch of `exchange_generic`, and its terminal arm `OriginalCommandIngressRetirement::Failed(failure) => return Err(failure)` hands the command's failure back AS THE TURN'S FAILURE, so the actor turn fails semantically (`poll semantic failure`) and the bridge source is dead.
Fix (turn crate, `⌨️command-ingress-turn/🦀️.rs`): after the retirement is complete the failure is converted to a fault and routed to the host as the command's terminal status instead of failing the turn: `Fault(App, "plugin.command-failed", failure.primary.message())` -> `encode_fault_bytes` -> `self.status = terminal_command_ingress(cursor, Some(bytes))`, owner cleared, `Ok(())`. The host's command promise rejects with that fault (shell shows the refusal) and the actor keeps running.
The root of seq 7's failure remains the missing staged `ActionMeta` (the command's `view_state`/window metadata for the outliner window's `setActiveExample`): `OriginalManifestMetadata::advance` completes without producing `meta` when the command carries no staged view state; send me the full `FAILED with ...` line and the `manifest advance_step` phases for seq 7 to decide whether the metadata stager or the host's command encoding is wrong.
Secondary (copy ~270 KB per turn during the retirement): that is the retirement itself progressing, not a leak: `close_exchange_owner` and the decoded command's `close_original_step` now run repeatedly within a turn (`step_until_slice`, follow-up 25) and each step copies up to a retirement page; the copy is charged once to the lifetime grant (it frees `release` the same amount). Probes kept. Rebuild: Rust (norm).

## Follow-up 39: root cause of seq 7: `OriginalManifestMetadata` phase 11 is both "start parsing" and "ready"

`OriginalManifestMetadata` (`🛂️manifest/metadata.rs`) is a phase machine. `admit` starts it in phase 8 when the command carries no staged view state, but in phase 11 ("create the view parser", the arm that continues to phases 0..7, 12) when it does. The terminal state was ALSO phase 11:
`ready() == !closing && phase == 11`, set at the end of phase 10 (`self.phase = 11`). `advance` begins with `if self.ready() { return Ok(true) }`, so a metadata owner created WITH view bytes was "ready" at birth, its parser, view and `ActionMeta` were never built, `meta` stayed `None`, and the next unwrap in `advance_manifest_step`
(`receiving.metadata....meta.as_mut().ok_or_else(|| refusal("staged original metadata is absent"))`) failed. That is exactly the log: kind 5, phases 1..14 of the receiving finish, `metadata false` in the probe tuple (it is not part of the change detector, so later `true` lines were never printed), then "staged original metadata is absent".
Commands 2, 4, 5, 6 carried no view state (phase 8 -> ... -> 11 = ready) and worked; `setActiveExample` from the outliner window carries the window's view state, hence it was the first to hit the collision.
Fix: the terminal state is now its own phase 13: phase 10 ends with `self.phase = 13` and `ready()` is `phase == 13`; phase 11 keeps meaning "create the parser". No defaulting of the `ActionMeta` was needed, it is now built from the staged view state as designed.
The routing fix of follow-up 38 stays (a failed command now answers the host with a fault instead of killing the source). Probes kept. Rebuild: Rust (norm), crate `semio-plugin-runtime-ops`.

## Follow-up 40: the ingress owner is lent by pointer (lifetime copy drain of ~260 KB per turn)

Confirmed by the budget numbers (`held_copy 130,493`, 4 claims): every turn the ingress slot lends and returns `ActorReturnTarget<RetainedCommandIngress>`, whose size is that of the whole inline `CommandIngressOwner` (~130 KB), through two sized escrow claims (home and donor), so the lifetime copy grant (512 MiB) was spent after ~1,000 turns even though the command was progressing.
Fix (turn crate, `🔄️turn/🦀️.rs` and `🔌️extension-invocation-and-command-ingress/🦀️.rs`): the slots now hold `Option<Box<RetainedCommandIngress>>` (`COMMAND_INGRESS: [OriginalActorReturnSlot<Box<RetainedCommandIngress>>; 2]`, `CommandIngressLoan.entry: Option<Box<RetainedCommandIngress>>`, `admit_entry` boxes the new entry and charges `size_of::<Option<Box<..>>>()`
instead of the inline owner). The sized claims are therefore pointer-sized (~16 B each) and the per-turn escrow copy is a few dozen bytes instead of ~260 KB. All accessors (`state`, `state_mut`, `state_slot`, `peek().entry.key`) go through the box unchanged; the retire/park paths use the same loan type.
Other per-turn lends of large owners: the only other `OriginalActorReturnSlot` in the turn path is the retiring lane of the same array (covered); the exchange owner and the surface refresh are held in their own owners and are not lent through a return slot each turn, so they do not pay this claim. Failed-command routing (follow-up 38) and all probes are kept.
Not charged: the one-time heap allocation of the box (130 KB once per command, not per turn). Rebuild: Rust (norm).

## Follow-up 41: "original projection must be driven or returned through its resident frame" (seq 7, first exchange turn)

Where it is raised: `RetainedCloneSourceAdmission::advance` (`🌱️value/🧬️retained-clone/🔗️source/🎟️admission/🦀️.rs:65`) answers exactly this refusal when it is asked to ADVANCE while the admission is in the owned-identity projection phase (`projectionStart`, `projection`, `projectionFailure`, `projectionFailed`): that phase is not a generic step, it must be driven by `project_owned` (one funded step that publishes the sealed owner's address, demand `owned_projection_demand()`), as the JSON decoders do
(`🎒️pack/🔤️json/📥️decode/🪪️native-input`: `if projecting { admission.project_owned(..) } else if ready { take_output } else { advance }`).
Why seq 7: `OriginalManifestMetadata` phase 3 (`🛂️manifest/metadata.rs`) drove its owned `RetainedCloneSourceAdmission<DslValue,..>` only with `advance` / `take_output`. That arm had never run: until follow-up 39 a metadata owner with view bytes was "ready" at birth, and the earlier commands (seq 2, 4, 5, 6) carried no view state and skipped the whole parse-view -> admission path. With the phase fix the path is exercised for the first time and the admission, once past `identity`, reaches `projectionStart`, where `advance` refuses. It is NOT caused by the boxed ingress owner (no address or frame is shared with it) nor by document replacement: the failure happens while the command's view-state metadata is staged, before the example load runs.
Fix: phase 3 now selects `project_owned` when `admission.phase() == "projectionStart"` (demand `owned_projection_demand()`), otherwise `advance`/`take_output` as before.
Afterwards ("stopped without publishing requested UI surfaces ... status=idle faults=[]", displaced 2->0 cycles): separate symptom, most likely a consequence: the outliner window's mount produced the failing `setActiveExample`, the shell then waits on it (later input waits for the command), and the guest has no pending render work, so it idles with the surfaces the shell requested unpublished. I could not confirm this from source; if the surfaces are still missing once `setActiveExample` completes, the surface-publish path needs its own probe (`render_dirty_surfaces` dirty set vs requested surfaces).
Probes and the failed-command routing are kept. Rebuild: Rust (norm), crate `semio-plugin-runtime-ops`.

## Follow-up 42: panic "original source admission retains unpaid custody" (Drop of `RetainedCloneSourceAdmission`)

Cause: `OriginalManifestMetadata` phase 3 dropped the admission as soon as `take_output` had put a Source into `self.source` (`if self.source.is_some() { *self.admission = None; self.phase = 4 }`). `RetainedCloneSourceAdmission::take_output` publishes the live Source fields across TWO bounded turns ("Publishes live Source fields across two bounded turns without relocating inert closing state"), so after the first turn the output exists but the admission still holds the owner/lease custody; dropping it trips the `assert!(panicking || terminal_is_empty())` in its `Drop`.
The JSON decoders (`🎒️pack/🔤️json/📥️decode/🪪️native-input`, phase `Source`) do it in this order: `project_owned` (when `projectionStart`) -> `take_output` repeated (the action tag 9 remembers "taking") -> the admission is `take()`n ONLY when `admission.terminal_is_empty()`.
Fix (`🛂️manifest/metadata.rs`, phase 3): same lifecycle. `taking = self.source.is_some()` marks a take in progress: while taking the demand is `admission.take_output_demand()` and the step is `take_output` again (never `advance`); `*self.admission = None; phase = 4` only when `self.source.is_some() && admission.terminal_is_empty()`. Projection is unchanged (`project_owned` when `projectionStart`).
Probes and failed-command routing kept. Rebuild: Rust (norm), crate `semio-plugin-runtime-ops`.

## Follow-up 43: panic "retained source is closed" (`RetainedCloneSource::borrow`) on the first exchange turn

I could not identify the `borrow()` caller from the panic text alone (the infallible `borrow()` panics on any `try_borrow` error: owner absent, lease absent or lease closed). What the source says: `take_output` publishes the destination Source in two turns and until then it is an "unborrowable publication shell" (`publication_pending()`); it only becomes borrowable when its original live fields have settled, and it is closed when the admission is dropped before that.
Defensive fixes in `OriginalManifestMetadata` (`🛂️manifest/metadata.rs`): (1) phase 3 now moves on (drops the admission, phase 4) only when the destination Source exists, is NOT `publication_pending()` AND the admission is terminal; (2) phase 4 (the first user of the Source, wrapping it into the receiver) waits (`Ok(false)`) while the Source is still publication-pending. Any consumer after phase 4 therefore only ever sees a fully published, open Source.
New probe (temporary, once per change): `[DEBUG] metadata advance: phase N source <b> publication_pending <b> admission <b>`; the last line before the panic names the phase that borrows. If the panic persists, the caller is outside the metadata phases (the apply of the staged `ActionMeta`, `handle_action_invocation`, or the view decode `ActionMeta::admit_view_root`, phase 12) and that line tells which.
Rebuild: Rust (norm), crate `semio-plugin-runtime-ops`.

## Follow-up 44: `retained source is closed` after metadata phase 10 = the surface refresh borrows a half-published Source

`ActionMeta` carries no retained Source: `view_state` is a plain `Arc<ViewModel>` (`ViewLease`, built in metadata phase 12 from the owned view), so nothing in the metadata phases closes something the action later borrows. The first consumer of that `Arc` after the metadata is ready is `OriginalSurfaceRefresh` (`advance_manifest_step`: `receiving.refresh.advance(&mut instance.surface_contexts, &mut meta.view_state, cx)`),
which wraps it as `RetainedCloneSource::<ViewModel, ViewLease>::admission_borrowed(ViewLease(view))`, drives the admission with `self.admission.drive(cx, &mut self.source)` and, in phase 2 (`🪟️surfaces/native.rs:43`), calls the panicking `self.source.as_ref().ok_or_else(invalid)?.borrow()`.
Phase 0 moved on as soon as `self.source.is_some()` - the SAME mistake as follow-ups 42/43: the destination Source exists after the first of the two `take_output` turns but is still `publication_pending()` (an unborrowable shell), and its `borrow()` panics with "retained source is closed".
Fixes (crate `semio-plugin-reactor-surfaces`): `OriginalSurfaceRefresh` phase 0 now advances to phase 1 only when `source` is Some AND not `publication_pending()` (`native.rs`); the sibling cached-view reader (`read.rs` phase 3, `registry.cached_source`) had the identical `is_some()` transition and gets the same condition. While pending, the next turn drives the admission again.
(Observation matching your log: after the metadata phase 10 comes exactly this refresh, the first Source borrowed from the staged `ActionMeta`.) The `[DEBUG] metadata advance` probe is kept; if a panic remains, the last gate/phase line names the next consumer.
Rebuild: Rust (norm), crates `semio-plugin-reactor-surfaces` and `semio-plugin-runtime-ops`.

## Follow-up 45: field child without native terminal lifecycle, refused retirement routing

- Root cause 1: three framework cursors never declared `native_terminal_release` (trait default `None`), so any retained-clone field walk reaching them refused the field close with `UnsupportedOwner "field child has no native terminal lifecycle"`: `RetainedAliasCursor` (contract/alias), `SharedCloneCursor` (issuance-clone) and `SharedUtf8CloneCursor` (shared-utf8/clone; every shared string in the din4108 view model). Each now returns `RetainedCursorRelease::native(|c|c.terminal_is_empty())`, the same pattern as the vec, array, paged and scalar cursors.
- Root cause 2: in `exchange_generic`, a `Refused` failed-decoded retirement returned `Err(failure)`, killing the source as `poll semantic failure`. It now keeps the owner, parks it in the retiring slot (as the supersede path does) and answers the host with terminal fault `plugin.command-failed`; if the retiring slot is busy it retries next turn.
- Rebuild: norm (semio-framework-value retained-clone/shared-utf8, plugin reactor turn crate). All `[DEBUG]` probes kept.

## Follow-up 46: "native projection backing changed its original ownership" during failed-command retirement

- The check is `validate_effect` in `🧬️retained-clone/📦️backing/📦️storage`: a native backing records its work binding (scope, issuer, source turn) at birth and refuses any close effect whose binding differs.
- Root cause: `PluginCommandIngress::advance_original`, FailedDecoded arm, called `owner.close_original_step(cx)` bare. Every sibling close path (`RetiringDecoded`, `ClosingDecoded`, `exchange_generic` Decoded) wraps it in `cx.with_original_retirement(...)`, which switches the context to the original-retirement loan that carries the source-turn identity. Without it the close effect had a different source turn than the birth effect, so the projection/view backing refused. Fixed by wrapping the call.
- "record action must settle before close" (2) and "clone owner has unresolved close action" (3) are most likely consequences: the refused close left a record action pending, the Refused routing then parked the owner, and the reconcile loop met the unsettled action. Not changed independently; if they persist after this rebuild they are a separate retire-order gap in `🧩️record/🌳️source` (`close_dispatch` returns pending while `action.pending()`).
- Rebuild: norm (plugin runtime-ops ingress crate).

## Follow-up 47: audit of the first "native projection backing changed its original ownership" (normal path)

- The wrapper theory from follow-up 46 is weak: `with_original_retirement` only flips `original_retirement`, which changes deadline-yield behaviour, not the work identity. The wrapper on the failed path stays harmless, but it is not the root cause of the first failure. No close in `advance_manifest_step_inner` or the surface refresh is missing a wrapper that would change a binding.
- Evidence: `with_original_effect_at` (used by `funded` and `close_original_effect_step`, hence by every refresh `advance_cursor`/`take_cursor`/`close_cursor`, the `begun`/`retired_poll` steps and the metadata steps) builds a fresh `OriginalWorkAuthority` per effect. `OriginalWorkActionIssuer::new` and `OriginalWork::fresh` draw new issuer and scope ids from static counters, so every effect has a different (scope, issuer). Its source turn is `None` there (only `with_original_source_effect` supplies one).
- `NativeProjectionBacking` records `OriginalWorkBindingIdentity::from_effect` at its first effect and then requires `matches_effect` (scope + issuer + source) on every later effect. The second effect of the same backing therefore always mismatches. That is exactly "right after the surface refresh started": the view-model cursor (maps/vecs use this backing) is born by the first `advance_cursor` and refused at the next.
- The contract already documents the intended predicate: `matches_source` ("same Source dispatch although each effect opens its own Work issuer and scope, so a resident owner captured by one turn's effect retires under a later turn's").
- Patch prepared, NOT applied: `norm-backing-source-binding-patch.py` (dry-run by default, `--apply` writes) switches `validate_effect` to `matches_source` and `validate_work_binding` to compare the source turn. Same hazard exists in the many other `binding.matches_effect(effect)` sites (paged-list engine `validate`, array backing `authenticate`, derive projection close, source-owner, ordered-map cursors, etc.); they are only reached if the next run moves the failure there, so they are not touched yet.

## Follow-up 48: source-dispatch binding applied

- Applied `norm-backing-source-binding-patch.py` (storage backing: `matches_source`, source-turn compare for `validate_work_binding`).
- Proactively switched `matches_effect(effect)` to `matches_source(effect)` in the stored-binding sites on the view-model path: derive projection (`✨️derive/📽️projection`, all six sites), paged-list engine `validate`, array backing `authenticate`, ordered-map clone cursor `effect()`, ordered-map edit workspace `validate`. Left untouched: ordered-map lookup cursor (compares the loan's binding with the effect, a different relation) and the decode/construction/read families.
- Rebuild: norm (semio-framework-value crates and derive).

## Follow-up 49: "original manifest receiving requires its retained command frame" after apply

- Raised only by `command_parts` (manifest/🦀️.rs), which needs `wire.original_command_mut()`. The dispatch capture (`capture_dispatch_step` -> `take_for_dispatch`) moves the `AppCommand` out of the wire before the command runs, so after a successful apply the frame is gone while the receiving still has to close.
- Root cause: `close_original_step` fetched the receiving through `command_parts()` (needing the frame) although it only used the receiving, never the command bytes. It now closes `self.receiving` directly, so the receiving closes after the frame was consumed.
- The `effects 1` (document replacement) is routed by the existing `route_exchange_output`; not changed. Rebuild: norm (plugin runtime crate).

## Follow-up 50: "original outcome acknowledgement requires its live receiving place"

- Sole raise site: `OriginalJobReturnReceiver::acknowledge_outcome` (`🧵️job/📬️outcome/⏸️continuation/🎟️origin/🦀️.rs`), which refused whenever the receiver was closing OR held a continuation, before even looking for an outcome. `receive_outcome` and `receive_continuation` are mutually exclusive, so a receiver holding a continuation has no outcome to acknowledge; callers (cleanup pump, tool-run, preview, worker session) call the acknowledgement whenever their own flag says an outcome may be pending, so the guard fired for a legitimately outcome-less receiver and killed the source.
- Fix: acknowledge now answers `Ok(true)` when there is no outcome (continuation or none), and keeps the refusal only for the genuinely invalid case, a closing receiver that still holds an outcome (close_resources owns that acknowledgement).
- Not confirmed which caller (document-transfer load vs the seq 7 effect) hit it; if the failure moves, the next line to read is the caller's pending flag (`outcome_pending`/`acknowledgement_pending`). Rebuild: norm (semio-framework-job crate).

## Follow-up 51: "checked out cleanup job body was unreachable"

- `checked_out_job_mut()` returns `None` when `OriginalWorkerJobAuthority::job_mut` refuses: `retained_receipt_pending`, an unacknowledged outcome, or a held continuation (`payload.continuation().is_some()`).
- A continuation is not a benign yield. `OriginalJobReturnOrigin::finish` builds it only from `InteractiveJobStep::Continuation(failure)` or from a step whose callback/qualification/receipt check failed, i.e. the cleanup job's step FAILED and the capsule carries the original causes (`view.callback`, `qualification`, `receipt_qualification`). Follow-up 50 made the acknowledgement return `Ok(true)` for such a receiver, so the pump advanced to `checked_out_job_mut()` and reported the generic fault. The real cause was hidden before (guard refusal) and now (unreachable body); the relaxation is correct but exposes that the pump never handled a failed step.
- The typed-operation worker (`🧵️retained-command/🔄️full-operation`, line ~358) already reads the continuation view and turns its cause into a terminal fault. The cleanup pump has no such branch.
- Added a `[DEBUG] cleanup job body unreachable: <cause>` probe at both pump sites (live pump and close pump in `📦️instances/🔁️cleanup-turns`) printing the continuation's callback/qualification/receipt cause. The next run names the actual cleanup step failure; fix that root. I deliberately did not paper over it (waiting with Ok(false) would stall forever because a continuation is only released by closing the receiver).
- Slowness (2307 turns / 197 s for seq 7): not analysed from source this round; the earlier follow-ups' per-turn logs (reconcile loop 1024/751/381/308/151 opportunities, one `step_until_slice` slice per turn in `exchange_generic`) point at the retirement/receiving steps advancing one demand per turn. Candidate: `step_until_slice` around `cx.with_original_retirement(...close_original_step)` in `exchange_generic` and the manifest `advance_step`; needs per-phase counts before changing.

## Follow-up 52: "original native producer omitted observed native work"

- Raised in `OriginalNativeDecodeSource::receive_observed_work` (`🧵️job/⏱️context/🛬️decode`) when the producer's reported work is below the native decode control's observed work (`NativeDecodeControl::actual_work` growth during the step). Not the `[DEBUG]` logs: `pure::log` is a guest host call outside the decode control's work counter.
- The failing job is the document archive load (shell document transfer) in the cleanup/maintenance ladder (`🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder`): it calls `receive_archive(..., observed = decoder.last_turn_native_work())`. The SPR history decoder (`📡️spr/📜️history/🦀️.rs`) reported only `last_work`, which several branches (`refill_failure_storage`, retirement/close branches) never raise although the control observed native work.
- Fix: `RetainedHistorySource::step` and `close_step` now report `last_work.max(last_native_work)`, so the producer's receipt always covers what the control observed (the archive close path then also pays it through `Exit::pay`). Rebuild: norm (spr history crate, then plugin).

## Follow-up 53: "original lifecycle receipt exceeds its exact phase loan"

- Raised in `original_lifecycle_step` / `original_lifecycle_effect_step` (`🏛️ownership/⚠️failure/🦀️.rs`) when `step.progress()` does not fit the phase grant. `original_lifecycle_grant` clamps `maximum_items` to `min(remaining, loan, 1)`: every action owner is handed ONE item. The archive load's history decoder admits its decoder owner in one step and reports a receipt of 6 items (2597 copy bytes, 4832 capacity); the decoder quotes only bytes (`demands()` has no item axis), so the 1-item clamp refuses a receipt the loan (and the remaining turn grant) actually cover. This is independent of the work max change in follow-up 52 (progress is not work).
- Fix: the receipt is now checked against the real ceiling (`min(remaining items, loan items)` with the other axes of the phase grant) while owners still receive the one-item grant; the remainder is debited exactly as before.
- Caveat: only the items axis is known to be over; if copy/capacity also exceed the phase grant the refusal will name the same message and the next probe line shows which axis. Rebuild: norm (plugin ownership crate).

## Follow-up 54: phase-loan refusal made self-describing

- Grep over the whole framework: the message has exactly two raise sites, `original_lifecycle_step` and `original_lifecycle_effect_step` (`🏛️ownership/⚠️failure/🦀️.rs`); there is no other site in the job, lifecycle or ladder crates. So follow-up 53's relaxation was in the right place but only widened the items axis; copy, capacity, release and depth were already `min(remaining, loan)` per axis, i.e. the real ceiling. The receipt (copy 2597, capacity 4832) therefore genuinely exceeds the phase grant on at least one of those axes, or the remaining turn grant was already low when the step began.
- Both sites now build the refusal through `phase_loan_refusal`, whose message prints the receipt per axis, the owner grant, the ceiling, the loan and the remaining turn grant. The next probe names the axis and which currency is short; then the loan/funding of the archive load phase (not the receipt) is what must change.

## Follow-up 55: the one-item loan is the mounted turn's remaining items, not a fixed 1

- Diagnostics: copy and capacity fit (32692 / 16777180 = the mounted policy minus what the turn already spent); only items is short. The loan is minted by `MountedOwnerTurnV1::grant()` (`🏇️mounted-owner`): policy items (32 per interactive-component turn, "32 one-item units batched within the turn") minus the items the turn's earlier steps already posted. At the archive-load step the turn had 1 item left, so the loan was 1 and the 6-item decoder-admission receipt exceeded it (the same check also exists in `advance_lifecycle`'s `actual.fits(grant)`, so widening `original_lifecycle_*` alone could never have helped).
- Fix at the starting condition, not the receipt: the archive-load ladder step (`🔌️plugin/🪜️maintenance-ladder`, `DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS = 16`) now refuses to start (answers an empty progress step, retried next turn) unless the turn's remaining item loan is at least 16. The decoder quotes only bytes, so the item need cannot come from the demand; 16 covers the 6-item admission with headroom and fits the 32-item policy.
- Same hazard elsewhere: other lifecycle steps whose owners post several items in one receipt (decoder/archive/hydration admissions) would fail the same way when started late in a turn; only the archive load is guarded now. Rebuild: norm (plugin maintenance-ladder crate).

## Follow-up 56: "original close demand exceeds the whole grant of its turn"

- Two raise sites, both in `🧵️job/♻️retirement/🎟️context/🦀️.rs` (`close_original_admission` and the retained-turn variant), comparing the close demand with `cx.original_total_retained_grant()` (the turn's whole grant). The axis is not derivable from source: the owner that closes (rejected UI patch page, surface publication or a large document owner) is not named by the message.
- The message now prints the demand (copy/capacity/release/depth), the whole grant and the remaining grant, so the next probe names the axis and the amount. No speculative grant change was made.

## Follow-up 57: din4108 viewport "color buffer cardinality differs from the vertex count"

- The host (`🌐️World3dHost/🎨️mesh-render/🔺️mesh-geometry-and-appearance`) binds `mesh.colors` with width 4 and requires `colors.length == vertices * 4`; the scene schema documents `colors` as "canonical per-vertex RGBA in linear color space". The norm viewport lowering (`📕️norm/…/🧊️viewport3d/🎞️lowering`, `mesh_json`) emitted RGB triples (3 per vertex), so the host refused the bind.
- Fix on the norm side (the host contract is the documented one): `mesh_json` now appends alpha 1.0 to every vertex colour; the unit test chunks RGBA quadruples; the module doc states the RGBA format. Rebuild: norm (viewport3d crate). Reactor/job crates untouched, awaiting the (2) numbers.

## Follow-up 58: lifetime copy exhaustion (a) and the endless document-transfer command (b) — findings, no source edits

Numbers: close demand copy 880 vs whole-grant copy 2 (items 3.4M, capacity 242M, release 234M left). The operation's lifetime copy grant (512 MiB) is exhausted; the host comment in `🔌️PluginRuntime/🚚️handle-turn-drive` says every guest turn pays ~4 KB / 12 items of lifetime grant (origin refresh), so 512 MiB / ~4 KB is about 131k turns: 36,024 continuations of the stuck command plus its idle turns plus ~1100 commands account for it. The grant is consumed by turn COUNT, so (b) is the real burn and (a) only moves the cliff.

### (a) renewal, design constraints
- `KernelOriginalResourcePool.admit` deducts an operation's whole grant once (`🎭️actor/🎟️retained-turn/🟦️.ts`); `OriginalOperationSource.receive` subtracts each receipt's spend from `#remaining`, and the next `issue()` hands exactly `#remaining` to the guest.
- The guest does not trust the host number: the persistent source owns a ledger and `bind_original_retained_total` clamps the bound total to `min(total, ledger)` (follow-up 21/22 era). Raising `#remaining` host-side alone is therefore clamped to the guest ledger and cannot renew anything.
- A new `OriginalOperationSource` (new operation id) is also not possible: command-ingress owners, source recipients, surface caches and retained clones are validated against the admitting operation identity (`validate_source`/`binding.matches`), so they cannot rebind to a new operation.
- Minimal sound design: a guest-visible, ledger-level top-up in the retained-turn custody. Host (`OriginalOperationSource`): `renew(extra: OriginalRetainedGrant)` only between turns (no outstanding turn), withdrawing `extra` from the kernel pool via `admit`, adding it to `#remaining` and to a new `#epochTopUp` carried in the next `OriginalTurnInputWit` (new field `renewal` in the wire record, WIT + schema-first). Guest (`job/⏱️context`, source parent ledger): accepts `renewal` only when it equals what the host pool minted for this operation epoch (the issuer-signed field), adds it to the ledger total and the return-budget origin, once per epoch. That is a WIT/schema change plus guest ledger code, i.e. a separate ticket; I did not start it blind.
- Cheaper interim already in place: `stock.json` operation grant 4 GiB. The rule of thumb is turns x 4 KB, so 4 GiB covers ~1M turns; fixing (b) matters more.

### (b) endless command: cause not provable from source; suspect list
`action=none` means the command is not a manifest action: it is a media-in (document transfer/import), `LoadWindowConfig`, config or pure command handled by the `Some(AppCommand::...)` arms of `plugin_exchange_turn` (`🔀️plugin-exchange`, lines ~200-255). `media_pending` returns an empty output every turn until one of the chained steps reports ready: `prepare_media_descriptor` -> `structured_media_binding`/`accepts_original_structured_media` or `natural_file_binding` -> `prepare_media_payload` -> `prepare_media_metadata` -> `advance_original_media_import` -> `settle_original_media_import` -> `take_media_in`. A step that answers `Ok(false)` without spending anything keeps the host asking (each ask costs the fixed per-turn grant).
- Strong suspect, caused by my follow-up 55: the archive load inside `advance_original_media_import` runs through the maintenance ladder, which now REFUSES to start unless the mounted turn still has >= 16 items. If earlier phases of the same turn already spend more than 16 of the 32 items every turn (the ordering is not mine to verify statically), the load never starts: pending forever, burning lifetime grant at idle cadence. This matches "archive load passes, then endless pending".
- Needed to settle it: the guest `[DEBUG]` lines for that command (`exchange_generic: seq N …`, `manifest pending: …`, `gate exchange.unapplied`) and ideally the line the ladder would log when it defers. Cheap next step I can add on request: a `[DEBUG] archive load deferred: items X` probe in `🔌️plugin/🪜️maintenance-ladder`, plus, if confirmed, replacing the items guard by "start only when the turn has spent nothing yet" (fresh 32 items) or raising the interactive policy items from 32 to 64.

## Follow-up 59: archive-load deferral probe and item-budget headroom (one edit)

- Log reading (`🗑️generated/din4108-probe38.log`): 1219 commands retired (all `applied true frames 2`), 7099 `more-work` turn lines, no guest line for the stuck transfer command (the host's `seq=none` means it never reached the guest). So the log neither confirms nor refutes the archive-load starvation theory, and the burn is ~512 MiB / ~7100 turns, i.e. far above the 4 KB per-turn figure (about 75 KB per turn on average); the per-turn copy cost deserves its own investigation once the stall is understood.
- Edit: `[DEBUG] archive load deferred: items X` logged once per change from the maintenance ladder (new `semio-plugin-abi` dependency in `🪜️maintenance-ladder/📦️packages/🦀️rust/Cargo.toml`, wasm-only log, no-op elsewhere); interactive mounted policy items 32 -> 64 with the >=16 guard kept (`🏇️mounted-owner`), so a turn that already spent up to 48 items can still start the archive load. The cleaner "reserve at the start of the turn" variant needs the ordering of the turn phases and was not attempted.
- The RGBA viewport fix (follow-up 57) is already in the tree. Rebuild: norm (instance-ladders, mounted-owner and viewport3d crates).

## Follow-up 60: the archive-load guard was reading a clamped value (my bug), and the ladder reserves the load's items

- Root cause of "deferred: items 1": the guard in the archive-load step (follow-up 55) compared `grant.maximum_items`, but that grant is `original_lifecycle_grant(..)`, which clamps owner items to `min(.., 1)`. It therefore ALWAYS saw 1 and always deferred: the archive load could never start, regardless of policy items (32 or 64). The policy bump to 64 never mattered.
- Fix: the guard is removed from the archive step. The real mounted loan is only visible in the ladder unit loop (`🧵️instance/📥️staging/🪜️maintenance-ladder`, `maintenance_ladder_unit_step(cx, loan)`): when an archive load is pending (stage 24 quotes a non-empty demand), stage 24 itself is skipped while `loan.maximum_items < 16` (logging `[DEBUG] archive load deferred: items X` once per change) and every OTHER unit is skipped while `loan.maximum_items <= 16`, i.e. the last 16 items of the 64-item mounted loan are reserved for the load. `DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS` and `debug_archive_deferred` are exported from the ladders crate.
- The ~63-item idle consumption per turn: `maintenance_ladder_step` loops units until the loan's items are 0; any unit that reports progress while idle keeps it looping. Stage 18 (tool-cancellation cursor) does exactly that: when its slot scan is complete and idle it posts one item and 8 bytes to advance the cursor, returns Progress, and the loop calls the ladder again, once per item of the loan (about 63x8 B). That explains the item burn but only about 0.5 KB of copy per turn, so it is NOT the ~75 KB per-turn copy drain; that consumer is still unidentified (not in the maintenance ladder). Stage 18's idle spin is left as is, since the reserve rule no longer lets it starve the archive load.
- Rebuild: norm (instance-ladders and instance crates).

## Follow-up 61: eurocode `ui.fixed-capacity` (analysis only, nothing applied)

- Raise sites: `ui_assembly_error(stage)` (`🔌️plugin/🕹️interaction`) and `ui_assembly_error_because(stage, cause)` (`🔌️plugin/🪟️window`); both carry the stage name in the MESSAGE ("fixed UI admission failed at <stage>[: cause]"). The probe logs only the code because `OriginalDomainPrimary`'s `Debug` prints `code` and `..`, so the stage is invisible in `en1990-probe.log`/`en1992-probe.log` (no `turn step ... failed` line precedes it, hence the failure is not from the displaced-surfaces or pending-effects steps; it comes from a path without `debug_failed`, most likely `render_dirty_surfaces`/surface assembly).
- Candidate capacities (schema-owned, `🖱️ui/🧬️contract/📏️budget/🧬️schema/🔣️.json`): documentNodes 128 (one UI document), nodeBindings 32, textMaxBytes 512, fixedListItems 32, valueArgumentEntries 4, valueRowActions 2, valueMaxItems 256, plus the tree window's fan-out cap (`🪟️window-kits/🌳️tree`: "refusing the whole editor with ui.fixed-capacity — the fan-out a document format reaches"). Eurocode apps (many checks/actions/surfaces) most plausibly exceed documentNodes 128 or fixedListItems 32 or a 512-byte text. I cannot pick between them from source; a wrong raise would be a blind cycle.
- Probe prepared, not applied: `norm-domain-primary-message-probe-patch.py` (dry run by default, `--apply`) makes `OriginalDomainPrimary`'s `Debug` print its message, so the existing `poll semantic failure` line names the stage and cause (the cause of `SurfaceEncodeError` already distinguishes pack, payload-capacity and schema overflow). With the stage, the fix is: raise the matching schema constant (budgets are schema-first JSON; the arenas size from it) or page the structure as the tree kit already does. Apply on your word; rebuild: norm (job crate).

## Follow-up 62: probe applied; the blanket UI budget raise was NOT applied (it cannot work)

- Applied `norm-domain-primary-message-probe-patch.py`: `OriginalDomainPrimary`'s `Debug` prints its message, so `poll semantic failure` names the stage and cause.
- Why no 8x raise: the UI budgets are inline fixed arrays and quadratic arenas, not soft limits.
  - `UiText` is `[u8; UI_TEXT_MAX_BYTES]` inline (`🖱️ui/🧬️contract/🔤️text`). `UiNodeRecord` embeds several `UiText`s (key plus accessibility/label fields) and a `UiNodeRecord` is the unit of `UI_RESIDENT_DOCUMENT_BYTES = documentNodes * size_of::<UiNodeRecord>()`. textMaxBytes 512 -> 4096 multiplies every record 8x; documentNodes 128 -> 1024 multiplies the node count 8x: 64x the document baseline, and `UI_RESIDENT_AGGREGATE_BYTES` is 64 slots of that baseline (about 32 MiB now by a 4 KB record estimate, about 2 GiB after), against the 512 MiB wasm linear memory and the fixed `UI_RESIDENT_SURFACE_BYTES = 8 MiB` per surface, which a 1024-node document can no longer fit.
  - `UI_DOCUMENT_VIOLATIONS = documentNodes^2 + 6*documentNodes + 1` is quadratic: 128 -> 16 K entries, 1024 -> over 1 M entries of a `UiContractViolation`, inline in a fixed list.
  - `remove_stack`, `validation_stack`, `validation_visited` (`🛡️limits/🏭️patch-apply-producer`) are `UiFixedList<_, documentNodes>` stack/struct arrays, and the schema constants are also asserted against the host (`🧪️tests/🎟️resident-refresh-budget`, resident refresh fixtures), so both sides must move together.
  - `fixedListItems` backs `UiFixedMap`/`PagedList` rows and every `UiFixedList` default; `nodeBindings` is a per-node inline table.
- Decision: no schema constant changed, because 8x (or even 4x on documentNodes together with textMaxBytes) would exceed the 8 MiB surface and the linear-memory budget and would move the failure from `ui.fixed-capacity` to an out-of-memory or resident-budget refusal. The right fix once the probe names the stage is to PAGE or truncate that one structure (tree-window fan-out paging as in `🪟️window-kits/🌳️tree`, or clip long text with the existing `UiText` clip mark), or to raise exactly the one constant it names by 2x. Rebuild: norm-eurocode (and norm for the job crate).

## Follow-up 63: the archive step's own loan was clamped to 1 item by the unit loop

- After follow-up 60 the load started (`deferred: items 15` once) and failed again because the loan reaching `original_lifecycle_step` was already `{items 1}`: `maintenance_ladder_unit_step` builds `grant = original_lifecycle_grant(cx, loan)` (items = min(.., 1)) and hands that clamped grant down as the step's loan, so the widened ceiling (`min(remaining, loan items)`) was 1 as well.
- Fix (clean variant): for `MaintenanceUnit::Stage(24)` (the document-archive load) the unit loop re-derives the step's loan with `min(remaining turn items, mounted loan items)` instead of 1; owners still receive the one-item owner grant inside the step, and the receipt (6 items for the decoder admission) is checked against the real ceiling. The alternative (spreading the 6 items over 6 steps) would require the decoder to meter items, which it does not. Rebuild: norm (instance crate).
