# Record Open Receipt Hole

The [receipt audit](402acb7f-340f-4938-a08f-b19bda9ba752) matches the genuine drawing failure shape: the first capacity-bearing parse step receipts `72280` (`65 * size_of::<Frame>()`, `PackLimits::default().max_depth + 1`) and the heap observer sees an extra align-1 realloc of `365 -> 821`.

That frames return in `pack/value/decode/receiving` is already exact. `try_reserve_exact` on an empty frame stack is one fresh allocation, `add_capacity` books `capacity * size_of::<Frame>()`, and the act returns before the document driver, the cursor, or any byte buffer. Writing `821` and `365` into that progress would receipt a realloc the act does not perform.

`observe_retirement_allocations` only adds layout sizes into a thread-local `Cell`. It does not allocate inside the measured closure, so the pair is a real realloc elsewhere in the same `open.step`.

Debug prints on this path format strings into the observed heap. Removed from the member-open `refused` helper and from the receiving-scope not-ready and mismatch branches. The receive-turn prints were already gone.

The realloc site is still unidentified. Leave the frame booking and `combined()` alone.

`seed-rec6` through `seed-rec8` compiled `semio-os-store-member-open-store` and ran. The foreign-envelope test passed. The three drawing opens failed later, after the fill selector: `open.step` returned `InvalidValue` / "original member snapshot receiving turn refused", and the cause printed before that helper stopped logging was `InvariantViolated` / "pending decoder requires original recovery". A turn had left the decoder's work or construction issuer with a live action, and the next receive refused to resume it.

That refusal is removed from `original_receiving_outcome`. A later receive now loans the same issuer and the resident action poll continues. `seed-rec9` had already left the lock wait when this landed, so that log may still show the old refusal.

`seed-rec9` finished after that: the large drawing still unwraps `InvalidValue` with the generic text "original member snapshot receiving turn refused". That helper now copies `OriginalTurnPrimary::message()` into the `ValueError`, so the next open-store run shows the real cause. Four other cargo processes still hold `native/target`.


## Resume across a live issuer

The pending-decoder refusal is gone from `original_receiving_outcome`, and the five `[DEBUG] rcv` prints on that path are gone again. `refused()` still copies `OriginalTurnPrimary::message()`.

A live receiving action keeps its scope loan. The next turn's quote asked for a fresh half-turn quantum. That quantum does not fit the partly spent loan, and the turn often has no fresh stock left because the loan still holds it, so `OriginalReturnScopePlan::quote` returned `None` and the step made no progress. While `pending_actions` is set, the loaned non-recovery branch now clamps that quantum to the loan's remaining body and resumes. An empty remainder still falls through to a top-up.

`receive_step` used to refuse the resume when the offered grant or work was smaller than the frozen action quote. A resume now keeps the frozen grant and work units so `NativeReceivingChild::poll` sees the same ceiling, and the effect still restricts itself to the scope grant. A first receive still requires the offer to cover its quote.

No open-store binary has compiled this yet. Other cargo processes hold `🗑️generated/native/target`. The next `seed-cargo.sh` of `semio-os-store-member-open-store` is the check.


## Quoted scope depth

The catalogue test `catalogue_lists_workshop_wood_machines` ended on the authored turn bound. Every turn printed a phase-4 host quote and then stopped with no retained progress. `original_receiving_outcome` replaced that quote's depth with `native_scope_quantum()`. When the reservation does not fit inside recorded progress, that helper returns an all-zero grant, the depth becomes 0, and `OriginalReturnScopePlan::quote` returns `None`. The same zero grant then failed the exact-match check after a scope was admitted.

An explicit quote now keeps its own depth when the helper is zero, and otherwise uses the smaller of the two. The exact-match limit uses the turn grant when the helper depth is zero. The pending-decoder refusal is out of `original_receiving_outcome` again.


A finished scope is observed against the turn grant when that same helper is zero. Otherwise the observation refuses to fit and the next parse step cannot clear the loan.


## Editor catalogue binary

`replay-p2q` ran `process3d_panels_catalogue` before the depth fix was compiled: 8 passed, 3 failed. The catalogue test is still the turn bound. `history_edits_end_to_end` refuses `document-archive-replacement.closure-rejected` with `Incomplete` on the create-step accept fixture. `conflict_history_edits_end_to_end` idles on `ClosingChildMember` because the next release demand is 27136 bytes and the caller has 25208 release bytes left.

`replay-p2r` is compiling that same catalogue filter against the current receive path: quoted depth is kept when the scope quantum is zero, the exact-match limit falls back to the turn grant, and a finished scope is observed against that grant.


## Catalogue effect fuel

`replay-p2s` reached the host callback (`hoststep ran=false`, 32742 times) and never entered the JSON parser. The scope claim succeeds, then `with_effect_reported` asks for `receiving_units(fuel)+1` work. The scope body fuel is that quote minus control work, so `OriginalEffectRun::begin` is refused and phase 4 repeats until the turn bound.

`StepContext::with_native_receiving_quoted_work` passes the scope body fuel into the callback. The host sizes the effect to `units-1` and shrinks the physical grant so the invocation widen fits the offered grant. Debug prints on that path are removed.

## Archive roster

`complete_document_archive_genesis` drops archived members the hydrated parent no longer reaches, including members owned only through those, then renumbers ordinals and mints the current projection. A reminted `steps`/`toolSolids` id no longer leaves the previous id in the roster for closure to reject as `Incomplete`.
