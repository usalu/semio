# 1108 jctests (job-context test ports)
- Run: `1108-jctests-run.sh W-jctests-<n> --test <name> [--no-run]` (per target; cargo stops at the first failing target; no --keep-going). Targets: original-payload-authority-work, original-source-return-place, original-native-receiving-context, original-source-close-work, original-return-and-retirement-contracts, original-native-parent-retirement, original_source_dispatch, original-native-targeted-retirement, original-native-pair-slot, original-effect-provider, original-park-state, original_source_resident_birth.
- Compiled green at start of lane (W-jctests-11..21): dispatch, targeted-retirement, pair-slot.
## Ported (edited)
- work (builder/🎟️work/🧪️tests): u128 work asserts, `close_original_retained_effect_step` + `close_step(effect)`, `JobPayloadAuthority::admit(.., &mut actual)`, denied close via `cx.with_original_effect`. Compiles (W-jctests-3). Runtime W-jctests-4: 6 red, see below.
- job-testing-caller: appended `bank_run`, `bank_step`, `bank_source_parent` (EffectBank step helpers, Source parent birth via OriginalCallerSourceAdmission); FIX production-adjacent fault: `retire_step` owner-less metadata removal reported work 1 with performed 0 (EffectBank law `reported == performed`) -> now `effect.performed(1)` first.
- return-place, native parent-retirement, effect-provider, park-state: removed per-test `#[global_allocator]` HeapWitness (value-testing-allocator is the single global allocator; trace observe works under it).
- resident_birth: `retirement_work_demand()` of CancelTokenRetirement is `u64` now.
- close-work: rewritten on EffectBank + `bank_source_parent` (OriginalCallerSourceParent::admit is gone).
- contracts (capsule 45 errors): `owning.rs` now `pub use semio_framework_job::*` (InteractiveJob, InteractiveJobStep, OriginalJobReturn* live in the job crate).
- receiving (30 errors): helpers `settle`/`polled`/`effect_step`/`retirement_effect_step` over `NativeDecodeControl::receive_effect`; copy_native_text and retire_original_text on reply-shaped receivers (`advance_local`, `take`, `begin_close_into`, `close_step`) and `ControlledRetirement`; `with_native_receiving(&mut outcome, ..)`.
## Open
- Runtime of work test: payload admit success costs work 2 (law JSON successWork 1) -> law to be re-derived after the full run; other 5 failures were the fixture retire_step reported/performed mismatch (fixed).
- Contracts / receiving compile unverified (queued W-jctests-33).
STATUS: IN PROGRESS
## Priority fix: effects from StepContext carry a producer invocation
- job-context `with_original_effect_at` and `OriginalRetirementTurn::with_effect` run their closure via `OriginalInvocationCall::drive` (a one-shot `OriginalInvocationProducer` over `OriginalEffectAuthority::drive`), so `effect.invocation()` is Some (native/JSON/sealed/field retirements no longer refuse). Lib native W-jctests-41 rc0; law `every_context_and_pump_effect_carries_its_original_producer_invocation` (effect-provider) W-jctests-42 5/5.
## Test status W-jctests-40 (before fix): ok effect-provider, park-state, resident-birth?, dispatch; red: pair-slot 3, parent-retirement 4, targeted 1, receiving 6, contracts 19, close-work 1, work 1 (success work 2 vs law 1). Dominant causes: "original clock Source or action changed" (7), "decoder Source absent" (6) = driver sourced-bootstrap; invocation (2, fixed); fixture "root closure exhausted" 1 (open).
STATUS: PARTIAL (compiles all 12 targets W-jctests-37; runtime reds open)
## Rerun W-jctests-50 (after driver clock resume), pass counts per target
- ok: lib unit 1/1, effect-provider 5/5, park-state 1/1, dispatch 1/1, resident_birth 1/1.
- red: payload-authority-work 5/6 (law fixed afterwards: successWork 1->2), contracts 5/22, return-place 2/10, close-work 1/2, pair-slot 0/3, parent-retirement 0/4, targeted-retirement 0/1, receiving 0/6.
- Admit cost 2 vs 1: HEAD `admit_shared_allocation` returns work 2 on success (1 allocation attempt + 1 value install) and 1 on a refused attempt; that is the committed contract, so the law json `successWork` is now 2 (boxedWork.successWork stays 1, passing). Rerun needed (W-jctests-52).
- "exhausted its initiating authority": only in source_escrow_birth_refuses_insufficient_total_and_yields_for_a_small_turn; same birth/driver cause as below (fixture retire never completes after a refused birth); not fixed.
- Dominant remaining runtime causes (driver/native-turn, not test-port faults): "original Source declined construction extent" x16 (native-turn grant_construction errors when the turn grant is narrower than the demand; tests expect a yield, i.e. needs a pending/yield outcome or total-grant check like close_original_admission), "decoder Source absent" x6, "original Source deadline is already bound" x2 (fixture retire_step), "retirement cursor lacks a work quote" x1.
- Escrow law a_thousand_receiving_turns...: lives in 🧪️tests/📊️native-progress which is NOT a [[test]] target of job-context (W-jctests-51 filtered out, 0 run). Needs a [[test]] entry (LOCK NEEDED, not yet added). zero_work_loan not run.
STATUS: PARTIAL
## Step: a_thousand target + work law
- Added `[[test]] original-native-progress` (path ../../🧪️tests/📊️native-progress/🦀️.rs, testkit) to job-context Cargo.toml; no new deps.
- W-jctests-60: work law 6/6 PASS with successWork 2. native-progress 0/3 (a_thousand_receiving_turns..., scoped observers, cancel at 256): all fail at the fixture `with_native_receiving` step with "decoder Source absent" (driver2 native-turn fault), before reaching the escrow table.
- zero_work_loan (os-store-replay-operation original_zero_work_loan): W-jctests-62 queued/pending at time of writing.
- Rerun after driver2: `1108-jctests-run.sh W-jctests-<n>` with --test for each of the 13 targets (or W-jctests-50 command).
STATUS: PARTIAL
## Double-charge check (fem W-fem-t11)
- No double charge: `close_original_effect_step` debits once via the closure's returned work; the effect receipt is never debited by the provider. Law `a_one_fuel_turn_closes_through_its_effect_charging_each_unit_once` + invocation law: effect-provider 6/6 (W-jctests-71). fem sparse --lib (W-jctests-72): the "work loan callback replaced its initiating context" signature is gone (0 hits); 5 pass / 6 fail on fem's own Admission/publication faults.
- "exhausted its initiating authority" root cause: a test left the shared fixture clock past its deadline; the fixture cannot close its birth then. Fixed in the effect-provider test (reset clock before retire); the escrow-birth test in parent-retirement may have the same cause.
STATUS: PARTIAL
