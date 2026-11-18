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
