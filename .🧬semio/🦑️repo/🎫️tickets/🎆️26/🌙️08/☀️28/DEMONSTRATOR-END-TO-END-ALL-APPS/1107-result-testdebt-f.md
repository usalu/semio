STATUS: PARTIAL. value-native-encoding GREEN on every target (testdebt-f-28 rc 0: lib 36, action-work 9, effect-settlement 4, typed-retirement 5 + 1 ignored). value-native-decoding compiles with every test target and its closure fits (261,989 B), but its laws are RED (testdebt-f-29: lib 8/10, native-backing 2/6, native-continuation 4/5, original-work-view 1/1). Clone-source mounted tests NOT started (about 150 errors, testdebt-f-30).

Logs: C/🗑️generated/1105b/testdebt-f-<n>.txt. Tools: C/1105b-tools/testdebt-f/ (chk.sh, errs.py, rep.py, port scripts).

## Encoding (GREEN, testdebt-f-28)
- Shared helper `value/🛫️encode/🧪️tests/🎟️eff/🦀️.rs`; `crate::eff` in the package root and in the action/effect test roots.
- Contract changes the laws now state: forwarded `charge`/`copy_*` refused (UnsupportedOwner) so forwarded laws use `allocate_vec_into`/`reserve_text_into`; performed allocator failure posts zero; admission records requested bytes; owner-custody laws run on Local controls; receipt corruption uses completed>total.
- PRODUCTION FIXES (claimed in 1105b-claims):
  1. `close_retirement_recipient` commits the closed owner's births once to the ledger and to the construction recipient when installed (runtime2 ruling). Law: owned_bytes == retained + births.
  2. `effect_allocated_with_work`, unallocated resume: a producer that already performed is no longer re-run after an unwound post (it replayed and double-performed). port-e informed.
- Ported laws: all allocation/slot/unwind/receipt/lib-root/turn-ceiling/continuation tests; typed-retirement 1-5 (pending allocation now via NativeRetirementGate::into_pending, settlement owners via effect_allocated_with_work with panic-after-reply); action-work 1-7, 9-10 (identity of work/construction authorities, post unwind without replay, wrong authority refused before callbacks, quote/admission unwind never replays, short quote, both owned refusals); effect-settlement: zero backing, borrowed status, depth restore, denied effect.
- NOT ported / open (be exact):
  - typed-retirement `scoped_pause_refuses_the_pending_original_and_recovers_the_same_control`: `#[ignore]` with reason; recorded-Source quotes run through `prepare_source_allocation`, which the law does not drive yet.
  - action-work `issuer_birth_quotes_only_initialized_fields`: removed with the Issuer types (no successor type).
  - action-work `original_effect_visitor_recovers_retained_frame_without_aliasing_or_repost`: not ported (visit_effect_original with a boxed frame).
  - effect-settlement `real_backing_retains_every_independent_refusal`, `scoped_continuation_preserves_original_issuer_and_recipient`, `scoped_encoder_retains_actual_overage...`, `scoped_source_retains_born_backing...`: not ported (scoped-source API: `new_recorded_source` changed signature twice during this run).
  - FINDING (open, production): a quoted effect whose closure never performs work leaves a construction action that neither `cancel_effect_settlement` nor `release_refused_effect` can retire ("original effect cannot cancel its retained state" / "refused effect is not settled"); the `!denied` half of `denied_and_unperformed_effects_do_not_post_work` is therefore not run (documented in its docstring).
  - FINDING: callers of a completed effect must call `state.view().claim_producer_work()` before the state is retirement-ready (tests do).
- Corrected stale expectation: after the births fix, `receipt.owned_bytes()` grows by the cleanup births exactly.

## Decoding (compiles, laws RED: needs runtime2 rulings)
- Done: splits (`[[test]] native-backing`, `native-continuation`; invalid-receipt test stays in lib as `🔁️continuation/🧪️tests/invalid.rs`), `allocate_empty_vec` pub + re-exported, shared helper `🎟️work/🧪️tests/eff.rs` (crate::eff in the lib), ported unwind (incl. both frame tests), shared-work test, continuation inline test, view.rs. Production compile fix: `producer_refusal` initializer in `🏭️construction/🧾️result/🛬️receiving/🦀️.rs`. `🎟️work/view.json` views[1] 3 -> 4 (available units after reserving 3 of 7; the old value belonged to the removed `available_work_units` quirk).
- RED laws (testdebt-f-29): unwind `original_port_panic_retains_actual_verdict_and_backing` (settle after recovered post returns false, panic text from the intended interruption precedes it); shared-work (reported work 4, fixture 2: the fixture row `deniedWork`/`nullWork` predate the u128/performed accounting); native-backing x4 (birth work/null work off by one, issuer retains actions after an early return, none-unwrap in the refused-post law); native-continuation `native_local_original_decode_recipient_survives_detachment` ("original failure birth must precede the general effect": the close needs `close_original_birth_step` before the general effect). These are accounting shifts in decode work/birth reporting and need a runtime2 ruling on each fixture row before assertions change; no assertion was weakened.
- Closure 261,989 B of 262,144 (margin 155 B): any new test line in the lib must go to a `[[test]]` target.

## Clone-source
Not started. `cargo check --tests` on value-clone-source: 150 errors (admission tests 32, custody 26, work tests 23, ♻️close 22, query 16, retain 12, header 12, law-bank 6, demand/owner 2).

## Next
1. runtime2 rulings for the decode RED rows above, then fix fixtures or production.
2. Port clone-source tests with the same helper pattern (eff.rs copy, `step(&mut effect)`).
3. Encoding opens: scoped-source laws, unperformed-closure cleanup.
