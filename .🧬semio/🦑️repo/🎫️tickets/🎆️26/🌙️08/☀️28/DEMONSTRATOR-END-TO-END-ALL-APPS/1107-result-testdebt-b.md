STATUS: PARTIAL. Every crate in the lane now compiles its tests except native-decoding (not started); remaining reds are listed with their evidence. GREEN only where cited.

Logs: `C/🗑️generated/1105b/testdebt-b-<n>.txt`. Error summarizer: `C/1105b-tools/testdebt-b/errs.py <log>`.

## Shared fixture crate (coordinator priority)
- `value/🧪️testing/📥️admission/🦀️.rs`: `ConstructionLoan`/`WorkLoan`/`receive` ported to `OriginalConstructionRecipient`/`OriginalWorkRecipient` + `OriginalConstructionAuthority`/`OriginalWorkAuthority`. `capture`/`close_owned` run through a scratch `EffectBank`. `Wallet` work is `u128`. Lib check green: testdebt-b-12 (rc 0), `--lib --tests` compiles in testdebt-b-14.
- `tests/original-close-marker.rs` ported (marker now settles inside the one funded action; the independent "clear" action no longer exists): GREEN in testdebt-b-14.
- `marker.rs` (`original-source-marker`): compiles, RED. `RetainedCloneSource::advance(effect)` (retained-clone/🔗️source/🎟️admission) reports work 4 while the effect receipt performed 0 (`SealedShared::admit(.., effect.actual())` never records `effect.performed`). Owner: value / retained-clone lane. `marker.json` actions still describe the old independent clear steps; update it only after the source records its work.

## retirement-queue (GREEN except one)
- Ported all tests to `EffectBank` through `🧪️tests/bank.rs` (+ `bank.json`/`bank.schema.json`); dev-dep `testing-effect-bank` added (no lock needed).
- Real bugs fixed in `📋️queue/🦀️.rs` (claimed in 1105b-claims.md): `admit_owned_demand`/`admit_retirement`/`admit_typed_retirement` under-priced depth (paged `adopt_reserved` needs `height+3`; added `PagedList::adoption_depth_demand`, additive); the empty-backing release quote omitted `next_release_copy_byte_demand` and `next_release_depth_demand`, so the final page release was denied forever.
- Proof: testdebt-b-15 (lib): 6 passed, 1 failed; testdebt-b-9 (resident_admission): ok.
- RED `original_turn_tests::retirement_turn_original_queue_preserves_full_grants_...`: the factory-built frame (`FactoryOwnedRetirement`) denies its own first `close_step` with its own quote (copy 56/cap 32/depth 2): the quote depends on the grant body and production needs more (accepts at copy 200: cap 160). Evidence testdebt-b-17/18 (child poll Progress(default), work 0). Owner: factory/retirement (`runtime`/`value`). Test is left faithful and failing.
- Semantic updates (not loosening): denied admissions are `Ok(None)` inert instead of OwnershipLimit/DepthLimit errors (API change); `transferWork` 2 (allocation + placement, matches resident `admissionWork`); aggregate `copied` is bounded by the quoted copy (headers) instead of 0 because the copy currency now counts header writes.

## value-read
- `🎟️work/🧪️tests` ported (`effect.rs` helper). Real production gap fixed: `ReadLease`, `ErasedReadLease`, `ReadAuthority`, `RegistryRetirement` had no `next_computational_work` (default Err "retirement cursor lacks a work quote"). Added (claimed). Proof testdebt-b-36: `real_read_alias_child_handoff_and_empty_header_have_distinct_quantum_one_work` ok, `canonical_borrowed_source_null_...` FAILED (below).
- Open: `canonical_borrowed_source_null_...` asserts the refused source allocation attempt costs work 1; now 2. Possible double charge in the borrowed source admission advance; unresolved.

## value-equality
- Tests ported (`tests/🦀️.rs`, `actual.rs`, `intrinsic/🧪️tests.rs`, `list/equal tests`, `sequence` mod). Compiles. testdebt-b-23: 14 passed, 2 failed (`inline_sequence_respects_neutral_zero_body_vectors`: first inspection turn reports work 0, law says 3; `retained_slice_equality_matches_portable_bytes_without_allocations`: `demand.copy_bytes > fixed_copy_bytes()` false for body 0). testdebt-b-22: `original_value_equality_matches_neutral_intrinsic_partial_eq_and_physical_cancellation` stalls (16384 turns, no answer) and aborts. All three look like lib-side quote/work gaps of the ledger port (`inspection.inspect` records no `effect.performed`); not changed.

## value-paged-text
- All three targets compile (testdebt-b-28): `🧪️tests/effect.rs` helper mounted at the crate root, `close-work.rs`, `native-rig.rs`, `staged-owner.rs`, encode/decode `📝️paged` tests ported; fixture paths fixed.
- RED (testdebt-b-29/31): close-work stalls (quote vs grant body), and the native encoder/decoder frames refuse "fully funded" turns (`native-rig.rs:40/47`), plus `InvariantViolated` where the law expects `OwnershipLimit`. These are production paged-text encode/decode frame faults, not test staleness.

## Not run / not done
- ordered, native-shared-slots, persistent, native-default, native-shared-field, retained-map, original-slot: compile (testdebt-b-3/19/32 had no errors for them) but their tests were never executed green in this lane (the batch run aborts at the first failing crate). UNVERIFIED.
- native-decoding `♻️retirement/🧪️tests`: NOT STARTED. The lib closure is 279349 B (budget 262144); mounted test files to move into `[[test]]` targets (ASCII names): `🏭️construction/📦️backing/🧪️tests/🦀️.rs` (15463 B), `🫴️recipient/🔁️continuation/🧪️tests/🦀️.rs` (12600 B), plus the retirement tests which are unmounted in this crate. All still use the removed `NativeDecodeWorkIssuer` family.
- encode effect tests (4 uses): not started.
