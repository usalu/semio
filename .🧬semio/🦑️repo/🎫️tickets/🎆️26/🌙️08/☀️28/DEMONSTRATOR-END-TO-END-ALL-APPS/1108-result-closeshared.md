# Lane closeshared result

## Root cause
A Pending close records its route with the whole first-turn grant and body; the recover turn compared `effect.grant()` and `body` for equality. Turn grants shrink per turn, so a Pending shared close (media slot, render snapshot) failed with an invariant error on recovery.

## Fix (quote-based recovery, Pending keeps state)
- `🌱️value/♻️retirement/🧬️contract/🔗️arc-custody` (`shared_controlled_owner!`, expanded by the shared owner and cursor): `SharedCloseRoute` records the exact selected `RetirementDemand` quote (not the grant). The route also keeps the action boundary. Recover admits `retained_clone_grant_admits(grant, quote)` and `remaining_work_units >= route.work - performed_since_action` from whatever grant the turn has; otherwise `reply.pending()` without any charge. The stored `body` is reused. `next_demand` while pending returns the stored quote.
- `🌱️value/♻️retirement/🎮️controlled` (`ControlledRetirement::step_reply`): `close_grant` removed; recover compares only invocation + original binding, reuses the stored body, admits the stored `close_selected` quote against the turn grant and the remaining Work of the selected action, else stays Pending uncharged. The pending child is lent `stored_grant.restricted_by(effect.grant())` (never widened). `next_demand` while pending ignores the caller's body.

## Law
- `🌱️value/♻️retirement/🔗️arc-custody/🧪️tests/shared-recover-grant.rs` (target `value-shared-recover-grant` of semio-framework-value-tests): `pending_shared_close_recovers_from_a_smaller_sufficient_grant` (recovers on exactly the recorded quote, smaller than the wide first-turn grant, one cursor recovery) and `pending_shared_close_stays_pending_without_charge_under_an_insufficient_grant` (depth below the quote: Pending, performed work 0, no cursor recovery; the next sufficient turn completes).
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-closeshared-tN "🧰️framework/🔨️modules/🌱️value/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-value-tests --test value-shared-recover-grant`

## Verification
- W-closeshared-1 native / W-closeshared-6 wasm32-wasip2 (`🌎️hub/Cargo.toml`, semio-framework-value-retirement + semio-framework-value): rc0 (W-6 on the final code).
- W-closeshared-4 native and W-closeshared-5 wasm32-unknown-unknown (media-slots crate manifest, semio-renderer-wgpu-media-slots): rc0.
- W-closeshared-t6 native: both new laws PASS on the final code.
- W-closeshared-t5 `shared_original_loan` (decode tests): the shared Pending close loop with identical retries still passes (saw_pending); the test then fails later in `exercise_resident_ack_refusal` (NativeResidentSuccessAck, post_error / advance_resident_success), unrelated to shared close; not investigated. An earlier run (t3) spun because the first cut compared the full route Work instead of Work minus already performed (fixed with the stored action boundary).

## Follow-up wave (coordinator: remaining owners), HANDOVER
Edited, compile unverified until W-closeshared-8 (native hub: control-contract, read, snapshot-projection, edit, native-owned, value, retirement) reports rc0; W-closeshared-7 had only one error (index.rs NamedRetirement has no close_route; fixed).
- `🎟️control/🧬️contract/📦️effect/🦀️.rs`: new `RetainedAllocationEffect::admits_recovery(quote,work,boundary)` (quote fits this grant, unspent Work fits remaining).
- `🔗️read` registry (`🦀️.rs`), `🪪️lease`, `🪪️authority`, `🧬️clone/♻️cursor`: route tuple field 2 is now the exact quote `RetirementDemand` (selected at the first turn via `selected/work` or `next_demand/next_computational_work`), recover drops grant/work/body equality, admits via `admits_recovery` else `reply.pending()`, uses the stored body; `next_demand`/`next_computational_work` with a route ignore the caller's body.
- `📽️projection/*` (owned, array, captured, field, root) and `✨️derive/📽️projection`: close recovery no longer compares grant or body (stored body reused). These routes keep no quote (state-derived), so they do not gate; the pending child gates itself (ControlledRetirement now does). The stale grant field in their key tuple is unused, not removed.
- `📋️list/🔺️edit` (edit, named, named index x2, transfer, commit close): recovery uses the stored body. The commit advance route still compares `maximum` Work (not a close route).
- `🛬️decode/🧳️owned/🔗️sealed-shared-receiver`: `RelayRoute` keeps the quote it already had plus the action boundary instead of the grant; recovery admits via `admits_recovery`; `progress.fits(effect.grant())`.
TODO: confirm W-closeshared-8 rc0, then wasm32-wasip2 (`chk.sh W-closeshared-9 🌎️hub/Cargo.toml wasm` same -p list), law rerun (value-shared-recover-grant) and `shared_original_loan`, media-slots native/wasm32-unknown-unknown; investigate `exercise_resident_ack_refusal` (fails at original-loan.rs:116 col 730: `advance_resident_success` after `post_error` returns Ready, test expects Pending; `NativeReceivingEffectState::Ready` is returned when `actor.metadata.child_work()` is Some in `🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs:175`; the test and function are unchanged vs HEAD, so a dependency change moved the semantic).

## Debts
- The same grant-equality pattern remains in the read registry family (`🔗️read/🦀️.rs` RegistryRetirement, `🪪️lease`, `🪪️authority`, `🧬️clone/♻️cursor`), projection family (`📽️projection/*`, `✨️derive/📽️projection`), list-edit routes and the sealed shared receiver (`route.grant!=effect.grant()`). Their routes store the first-turn grant as the quote and hand the raw effect to children, so each needs the same quote-recording port; not touched.
- `shared_original_loan` resident-ack-refusal failure (above).

STATUS: FIXED-verified W-closeshared-t6 (law native), W-closeshared-6 (wasm32-wasip2 value retirement), W-closeshared-4/5 (media-slots native + wasm32-unknown-unknown); read/projection route families still grant-equality
