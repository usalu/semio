STATUS: PARTIAL. Items 1-5 GREEN (paged-text all 28 pass, lawfix-160; equality 17 pass in 5 s, lawfix-149); native-encoding and native-decoding test suites are unported to the Original* protocol (hundreds of errors) and are NOT green.

Logs: `C/🗑️generated/1105b/lawfix-<n>.txt`.

## 1 original-source-marker: GREEN (lawfix-104)
Production faults: `RetainedCloneSourceAdmission::advance` reported phase work it never put on the effect receipt (wrapper now records the owed remainder once); sealed `Cursor::step_selected` alias-retire/take_value reported 1 unrecorded (now `record_retirement_work`). Quotes alias/payload/publication tightened to real work (3/1/5 incl. issuance). marker.json/ts updated to real pricing [4,3,1,4,5,1]; the old `clear-*` actions no longer exist.

## 2 retirement-queue turn law: GREEN (lawfix-112, 7 passed)
Factory bug: `OwnedValueRetirementFactory::retire_owned_demands` quoted the cursor birth (copy 56/cap 32) but `admit_owned_retirement` allocates the ControlledRetirement frame (copy 184/cap 160). Now `controlled_retirement_birth_demand`. Also factory construction `admit` (Prepared) and `prepare_step` capture reported work 1 without recording it.

## 3 value-read: GREEN (lawfix-113)
Stale expectation: a refused source allocation = issuance (1) + allocation attempt (1) = `SealedShared::birth_work_demand` 2. Law moved into case.json `refusedSourceAttemptWork` (+schema).

## 4 value-equality: GREEN (lawfix-120, 17 passed; one test takes 310 s)
Production: equality family reported work never recorded; `settle_work`/`settle_poll` in equal root now record every reported unit (sequence, record, tuple, intrinsic, string, owner). Stale tests: slice zero-body asserts neutral default quote (matches TS oracle/fixture), sequence law calls the inherent slice advance (adapter adds 2), intrinsic corpus `maximumWorkUnits` 64->1024 (text birth is priced 157 work: 9+1+2*72 layout bytes, never admissible at 64 = the stall). Open: `retained_typed_equality_matches_portable_original_fields` is very slow (310 s), not diagnosed.

## 5 paged-text
- close-work: GREEN (lawfix-127). PagedText byte pops now recorded; `next_demand` of the empty-backing release quoted no copy/depth (denied forever); test setup used next_reserve_* quotes for an exact reservation.
- Native frames: RED (lawfix-137). Fixed so far: `SealedShared::try_duplicate` records its 3 duplicate units; decode publication closure records its unit; Bind prepares `binding_work_demand` (3) not 1; encode effect gained `effect_allocated_with_work`/`begin_effect_settlement_with_work`/`NativeEncodeEffectRecovery::begin_with_work` (additive; old forms quote 1) and the paged encoder uses it for Bind. Remaining (staged-owner 13 red, decode/encode paged tests): decoder still returns Denied at publication with grant copy 0/cap 392 (decode page reservation path), encoder context error "original work requires its original issuer and resolved quote" (reserved != additional on resumed prepare, root not yet located), recorded-reply interruption tests. Rig panics now print the failure cause.

## Not done
- native-decoding tests split: closure 279,349 B; moving `🏭️construction/📦️backing/🧪️tests` (15.5 KB) + `🫴️recipient/🔁️continuation/🧪️tests` (12.6 KB) into [[test]] targets gets 251 KB, but they use private `super::*` items and the removed NativeDecodeWorkIssuer family, so each needs a public test API plus a port to Original* authorities.
- encode effect tests (4 uses): not started.
- Coordination: testdebt-c told debug prints removed; testdebt-d told about its transient controlled/ build break (fixed by them).

## Update (round 2)
- Decoder Denied at publication: root = page reservation quoted copy 0/depth 1 while the list's exact reservation demands next_capacity_copy/depth (and reserve work); quotes now come from the list and are stored in the reservation frame (encoder and decoder).
- Encoder issuer/quote error: root = no-allocation encode effect ran `settle_closure_work` twice (inside the closure and again at the shared settle step); the second call hit a finished action. `settle_closure_work` is idempotent once the work phase is Completed. Bind is quoted at 3 work (`effect_allocated_with_work`).
- Remaining red (5): staged-owner `interrupted_source_error...`, `overrun_retains_original_error...`, `producer_final_refusal...`, `encoder_settles_recorded_byte...` (the source never reaches its injected panic: tests expect recorded-reply interruption at a source read, but the first turn is now Bind; test sequencing needs re-derivation) and decoding `paged_semantic_text_keeps_actual8194...` (expects OwnershipLimit, gets InvariantViolated).
- Equality 310 s: cost was O(depth^3) per turn (nested work re-gating in `demand` at every level; depth-64 chain = 192 cursors, 48 ms/turn). `RetainedEqualityCursor::demand_admitted` (additive default) removes the re-gate: 5 s.

## Update (round 3)
- paged-text: all 28 pass (lawfix-160). Last fixes: encode no-allocation effect resumed after an unwind settles the claimed partial receipt before replaying the recorded reply and reports the carried work (`carried`); decode page reservation cancels its unperformed work on quote refusal (OwnershipLimit, was an invariant).
- try_duplicate regression check: retained-clone alias-work, original-scalar-public, tuple-work green (lawfix-152). clone-source lib tests do not compile (mounted custody/query/close-demand tests still on the removed `retirement()`/`close_step` API; testdebt-a/c territory), so no verdict there.
- native-encoding tests: the crate's test closure exceeded the 262,144 B source budget (lawfix-161). The effect tests (22.7 KB) now live in `[[test]] effect-settlement` (`🧪️tests/🎟️effect`), which fits the budget, but the whole suite still fails to compile (lawfix-162: 202 errors; allocation tests 76, slot 26, action 24, unwind 20, retirement 16, lib root 10) because it uses the removed `NativeEncode{Work,Allocation,Construction}Issuer` family and old `admits/receive/recover` signatures. Needs a real port to `OriginalWorkRecipient`/`OriginalConstructionRecipient`/`NativeAllocationAuthorityPort`, like the paged-text rig (`🧪️tests/native-rig.rs`) which is the working template.
- native-decoding tests: same situation (old `NativeDecodeWorkIssuer` family); the two files to move are named above; not started.
