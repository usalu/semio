STATUS: plugin-abi and store-testing-doubles (test profile) VERIFIED native (W-memberstore-16, -21 rc=0) plus earlier crates; W-gateway-119 rc=1: plugin-original-job-ownership (8), plugin-operation (32), plugin-decl-tool-contract (14), plugin-interaction (20), plugin-window-config (259) red
config retained: drop-file actor admit + types file (drive_active/admit_input/close_source via Self::authored wallet) EDITED-unverified; next advance:82,229-231, then member-open-store
config retained advance file EDITED-unverified (payload admit, inverse reserve_one via authored)
W-memberstore-1: config 20 -> 1 error (runtime.close_step effect), fixed line 172 EDITED; W-memberstore-2 config recheck running
member-open-store root 🦀️.rs EDITED-unverified: validate_source_context Ok(?), snapshot birth via owners.retire_initial_snapshot_owned, with_replacement dropped (document hydration has none; replacement staleness stays cx.replacement_is_stale). Left: record/🦀️.rs advance_original, lifted/🚪️open 148-175
member-open-store lifted/🚪️open close_step EDITED-unverified (runtime nested effect, initial via retire_initial_snapshot_owned, history via admit_typed slot)
member-open-store record/🦀️.rs EDITED-unverified: helpers funded/closed/received, parse_step/receive_step/take_step methods, receive_request field keeps Pending request identity, erased close ladder via RetirementPollExit::resume. Compile W-memberstore-3 next
W-memberstore-3: member-open-store 38 -> 6 errors; fixed (demands 2-arg, ControlledRetirement::step via resume, Err.into) EDITED; W-memberstore-4 running
W-memberstore-4: 1 error left (request.close_step ? conversion), fixed EDITED; W-memberstore-5 running

## W-gateway-116 red list (log 🗑️generated/1105b/W-gateway-116.txt, rc=1)
Only `semio-os-store-test-support` lib fails (11 errors, store/🧰️test-support/📦️packages/🦀️rust/../../🦀️.rs); config and member-open-store no longer appear (VERIFIED by being passed). Nothing downstream of test-support was reached, so testing-doubles/fixtures/laws are untested.
- 571/574/577: `impl SnapshotRetirementFactory<P> for RoundTripValueRetirementFactory` misses `retire_demands`, `retire_work_demand`, `recover_*` and `retire(&self, Option<Arc<P>>, effect, reply)`; work is u64 where u128 is required.
- 604/607/609: `ArtifactOwnedValueRetirementFactory<T>` impl misses `retire_owned_*` items; `retire_owned` must take `(&self, &mut Option<T>, &mut RetainedAllocationEffect, FactoryProducerReply)`; model: store/🏗️initialization/🏠️owners/🦀️.rs:735-744 and `factory_retire_framed`.
- 619: `owner.close_step(grant).result?` must take an effect (`semio_os_store_read::settle_close_poll` or `RetirementPollExit::resume`).
- 815: `<S as SubsetRoundtripSpec>::Mutation: OpBinaryReceiving` not implemented (3 errors), needs the bound added to `SubsetRoundtripSpec::Mutation` (or the schema installers: `original_pack_value_operation_receiving!` for pack-value mutations).

## Edits made (all compile-checked by W-memberstore-2 / -5)
- config/📥️retained: types file `authored` (AuthoringWallet outside any turn, config hydration has no StepContext), `drive_active`/`admit_input`/`close_source`; advance file payload admit and `inverse.reserve_one`; drop file actor admit and `runtime.close_step` with nested effect.
- member-open operation root: `validate_source_context`, snapshot birth through `owners.retire_initial_snapshot_owned`, `with_replacement` removed (document hydration carries none; staleness stays `cx.replacement_is_stale`).
- lifted/🚪️open close ladder: runtime nested effect, initial and history via slot-based admits.
- record/🦀️.rs: helpers `funded`/`closed`/`received`, methods `parse_step`/`receive_step`/`take_step`, new field `receive_request` (a Pending receiver must be re-advanced with the identical (body, units) request), close ladder via `RetirementPollExit::resume`.
Unverified: wasm target, tests, runtime behaviour (receiving nested inside `with_original_effect`, work charged with `consume_fuel` on top of publish's items+bytes charge).
test-support: RoundTripValueRetirementFactory ported to framed factory shape, drive_retirement via wallet (test helper, no turn), SubsetRoundtripSpec::Mutation += StoreMutationFor EDITED-unverified; W-memberstore-6 running

## Wallet scope note (coordinator question)
Config hydration (`RetainedConfigStoreHydration`, config/📥️retained) has no `StepContext` anywhere in its crate. Its only users in the tree are the store prelude re-export (store/🦀️.rs:89-90, 🧪️testing/🧭️prelude) and store tests (supersede-replay fold-site laws, replica-identity laws); the plugin window config hits only a `.ts` string mention. The `AuthoringWallet` therefore runs outside any job turn (pre-existing at the pending_snapshot branch, now also in `authored`). Static grep only, not a runtime proof.
W-memberstore-6 rc=0: semio-os-store-test-support VERIFIED native. W-gateway-117 running

## W-gateway-117 red list (log 🗑️generated/1105b/W-gateway-117.txt, rc=1)
test-support, config, member-open-store, document, replay, command, space, space-member no longer fail (passed in this run). Red now:
- `semio-os-store-testing-prelude` lib (58 errors) and `semio-os-kernel` lib (113 errors; the store facade 🏪️store/🦀️.rs lines 26-554 and the `codec/🎮️operation` file). Almost all are E0432/E0433 "unresolved import / cannot find crate `semio_framework_os_store_*`" (child, space_member, engine, engine_owners, composition, document, config, member_open_store, test_support, canonical_json, backbone, initialization_runtime/replay, space_schema, retirement, io, presence, ...), plus `semio_framework_os_semio` and `semio_framework_os_io`. The crates exist under the `semio_os_store_*` names (e.g. `semio_os_store_config`, `semio_os_store_document` as used in the lanes I ported), so this is a half-landed crate-name rename in the store facade/prelude, not an old-grant port. Owner: whoever renamed the packages; I did not touch it (names, imports, and Cargo deps may both be involved, so likely LOCK NEEDED).
- Real port errors behind the name noise: store/📦️codec/🎮️operation/🦀️.rs:92-93 (`close_step(..).result` on a poll, `RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>`), and unresolved items `canonical_edit::ARTIFACT_CANONICAL_JSON_CHUNK_*`, `store_schema::ArtifactBackboneRef`/`AcceptedArtifactProjection` (probably consequences of the same unresolved crates).

## Rename fix (EDITED-unverified, not compiled; waiting for the coordinator's lock)
- Sources store/🦀️.rs, store/🧪️testing/🧭️prelude/🦀️.rs, store/📦️codec/🎮️operation/🦀️.rs: every `semio_framework_os_io`, `semio_framework_os_semio`, `semio_framework_os_store_*` path now `semio_os_*` (looked up each by Cargo name and [lib] name: all 34 crates resolve to `semio_os_<same suffix>`, none overridden). No alias or re-export.
- Manifests: the two remaining alias dependencies (`semio-framework-os-store-member-open`, `semio-framework-os-store-one-item-batch`, both `package = "semio-os-store-..."`) renamed to the direct package names in the kernel and in the testing-prelude manifest. LOCK NEEDED logged in 1105b-claims.md for: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml and 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🧭️prelude/📦️packages/🦀️rust/Cargo.toml.
- codec/🎮️operation: `RetainedArtifactCodecCatalog::close_step` now takes `&mut RetainedAllocationEffect` (grant = effect.grant(); children via `effect.nested(child)`; box close through `settle_close_poll`; `close_catalog` takes the effect). The unit test calls it through a new local `close_with` helper (AuthoringWallet, test only, no turn). The facade re-export is unchanged.
- Next: after the lock, compile `-p semio-os-store-testing-prelude -p semio-os-kernel` (W-memberstore-7), then rerun the gateway W-gateway-118.
lock-117/sync-117 done; W-memberstore-7 running (testing-prelude, kernel)
W-memberstore-7: 2 errors (kernel codec admit_constructor -> admit_constructor_cold; prelude FixtureStoreConstruction -> OriginalConstructionRecipient/Authority) EDITED; W-memberstore-8 running
W-memberstore-8: kernel passed (no errors); prelude 4 errors from misplaced and_then, fixed EDITED; W-memberstore-9 running
W-memberstore-9: prelude 3 errors left (retire fixture needs OriginalWorkAuthority loan; expect needs Debug) EDITED with FixtureStoreWork; W-memberstore-10 running. Note FixtureStoreReceiving::retire callers keep their closure shape (OriginalRetirementCloseLoan)
W-memberstore-10 rc=0: testing-prelude and kernel VERIFIED native. W-gateway-118 running

## W-gateway-118 red list (log 🗑️generated/1105b/W-gateway-118.txt, rc=1)
Store facade, testing-prelude and kernel now pass (W-memberstore-10 rc=0 native lib check). Prelude changes beyond the rename: `FixtureStoreConstruction` ported to `OriginalConstructionRecipient` + `OriginalConstructionAuthority::with_loan`; new `FixtureStoreWork` (`OriginalWorkRecipient`, admits every quote) feeding `NativeDetachedRetirementDecodeContinuation::retire`, so `FixtureStoreReceiving::retire` keeps its `OriginalRetirementCloseLoan` closure shape for callers; kernel codec catalog admits its constructor through `admit_constructor_cold` (outside any Job turn, grant-supplied).
Red now (each is a separate crate, none touched by me):
- `semio-plugin-abi` lib: 2 errors.
- `semio-plugin-ownership` lib: 96 errors.
- `semio-plugin-tool-run-operations` lib: 59 errors (e.g. 🫧️transient/🧵️publication/🧬️native:67, 🏭️preparation:341, 📖️source:29).
- `semio-os-store-testing-doubles` lib TEST: 482 errors, dominated by 🗄️store/🦀️.rs lines 378-402, 744 and 57 (old grant/one-item shapes in the doubles' store fixture), i.e. test-profile code only.
Unverified: wasm, runtime behaviour of the fixture's work/construction recipients.
plugin-abi: genesis Sources<P> handwritten RetireOwned (old retirement(self) API) replaced by #[derive(RetireOwned)] EDITED; compile W-memberstore-11 (-p semio-plugin-abi)
W-memberstore-11 (semio-plugin-abi, rc=1): genesis E0407 gone (derive worked). The 2 remaining errors are one root cause: persistent-local/🦀️.rs:90 uses `semio_framework_kernel_identity::PollRequestLimits` (crate semio-framework-kernel-identity, defined framework/🔨️modules/🎠️kernel/🪪️identity) but abi's manifest did not depend on it; the E0367 Drop error is a cascade. EDITED the manifest (dependency added). LOCK NEEDED logged for 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/📦️packages/🦀️rust/Cargo.toml; abi recompile waits for the lock. Next while waiting: testing-doubles test profile.
testing-doubles store/🦀️.rs: StoreMutationFor<P|DemoSnapshot> added to 6 explicit Mutation bounds (lines 42,118,156,485,493,499) EDITED; compile W-memberstore-12 --tests -p semio-os-store-testing-doubles
W-memberstore-12: testing-doubles test 482 -> 272 after StoreMutationFor bounds. Remaining E0277 OpBinaryReceiving for Severity/Validated/TimestampedMutation: mirrored DemoMutation (NativeReceive + DslReceiving derives on enums and all leaves, original_ordinal_operation_receiving!, artifact_persisted_binary_operation!) in testing/🧬️mutations/{timestamped,severity,validated} EDITED; compile W-memberstore-13
W-memberstore-13: 272 -> 178. Added StoreMutationFor to ReferenceSpaceMember impl bound (store/🦀️.rs:~311) and envelopes owned_test_envelope EDITED. Remaining are old-API impls in doubles: preparation/🦀️.rs (52,71,239,368), retirement/🦀️.rs (28-223: factory retire/retire_owned 3->4 params, close_state_step), store/🦀️.rs:702-757 (advance/close_step). W-memberstore-14 running
W-memberstore-14: 178 -> 87 errors. doubles ♻️retirement/🦀️.rs factories ported (Demo/Exact snapshot, DemoInitialSnapshot, DemoMutation) + ExactDemoSnapshotRetirement close via factory_retire_owned/resume EDITED; W-memberstore-15 running. abi still waits on lock for kernel-identity dep

## State after W-memberstore-15 (log 🗑️generated/1105b/W-memberstore-15.txt)
- semio-plugin-abi: genesis `Sources<P>` now `#[derive(RetireOwned)]` (E0407 gone). Remaining 2 errors are one root cause, `semio_framework_kernel_identity` not a dependency of abi (persistent-local/🦀️.rs:90). Manifest edited (dependency on framework/🔨️modules/🎠️kernel/🪪️identity added), LOCK NEEDED already logged in 1105b-claims.md. NOT recompiled: needs the coordinator's lock first (then `chk.sh W-memberstore-<n> Cargo.toml native -p semio-plugin-abi`).
- semio-os-store-testing-doubles test profile: the StoreMutationFor<P> recipe cleared most (482 -> 272 -> 178 -> 87 -> 67): bounds added in doubles store/🦀️.rs and envelopes; Severity/Validated/Timestamped mutation vocabularies got the Demo shape (NativeReceive + DslReceiving derives on enum and all leaves, `original_ordinal_operation_receiving!`, `artifact_persisted_binary_operation!`); ♻️retirement factories ported.
- 67 errors left, all old-API owner impls (not bounds): 🏭️preparation/🦀️.rs (lines 52, 71, 88-111, 238-239 trait now needs `has_pending_receiving_recovery`/`pending_receiving_demand`/`pending_receiving_recovery`/`finish_receiving_recovery` and a new `advance` signature, 308, 341 (u128 work, `Option<(NativeAllocationIdentity,u64)>`), 368-392 close_step + factory `retire_owned` calls), 🗄️store/🦀️.rs (561, 702-709, 744 (same as preparation 341), 757-821), ♻️retirement/🦀️.rs (158, 217-228 `prepare_close_state_step`/`close_state_step` signatures, 276, 280), 🧾️callers/🦀️.rs (121, 201: 3 -> 4 args). Recipe: the replay port in 1107-result-replayport.md plus the document mutation port (receive_original_receiving_work, resident OriginalConstructionAdmission).
- Gateway W-gateway-119 not run (doubles still red; abi waits on lock).
sync-118 done; W-memberstore-16 (plugin-abi) running
W-memberstore-16 rc=0: semio-plugin-abi VERIFIED native lib check. Next: doubles preparation/store/retirement/callers (67)
doubles 🏭️preparation: FactoryPayloadRetirement impl ported (prepare_poll, close_result via effect/resume/settle) EDITED; viewing remaining lines 230-400
doubles 🏭️preparation: DemoOneItemPreparation advance error type, recovery methods, close_step(effect) via inherent close_result_value + settled_close/born helpers EDITED; compile W-memberstore-17
W-memberstore-18: 78 -> 37. doubles preparation fixed (inverse reserve via receiving.with_effect, u128 work); store/🦀️.rs DemoMemberWirePreparation ported like preparation (advance Primary error, recovery forwarders, close_step(effect) via close_result_value), closed_with wallet helper for test close loops EDITED; W-memberstore-19
W-memberstore-19: 37 -> 16. Fixed callers JobPayloadAuthority::admit (+&mut None marker), retirement DemoPublishedRootWitness factory impl and test close helpers (close_granted wallet), preparation .0, store primary_value/closed_with EDITED; W-memberstore-20
W-memberstore-20: 16 -> 3. closed_with now takes Result<Step,RetirementPollExit> and settles; callers OriginalCallerSourceParent::admit +&mut None EDITED; W-memberstore-21
W-memberstore-21 rc=0: semio-os-store-testing-doubles test profile VERIFIED native (482 -> 0). W-gateway-119 running

## W-gateway-119 red list (log 🗑️generated/1105b/W-gateway-119.txt, rc=1)
semio-plugin-abi, semio-plugin-ownership, semio-plugin-tool-run-operations, semio-os-store-testing-doubles (lib test) no longer fail. Red now, none touched by me:
- `semio-plugin-original-job-ownership` lib: 8 errors
- `semio-plugin-operation` lib: 32
- `semio-plugin-decl-tool-contract` lib: 14
- `semio-plugin-interaction` lib: 20
- `semio-plugin-window-config` lib: 259 (note: config hydration `.ts` test mention only; unrelated to store config hydration)
Doubles port summary (all VERIFIED by W-memberstore-21 native `--tests` compile, no test run): preparation `FactoryPayloadRetirement` + `ArtifactStoreOneItemPreparation` (Primary error, recovery methods returning no pending frame, close via `close_result_value` + `settled_close`/`born`, inverse reservation through `receiving.with_effect`), `DemoMemberWirePreparation` (recovery forwarded to inner), retirement factories/`close_granted` (wallet, test helper only), callers (`admit(..., &mut None)` marker). Unverified: wasm, runtime behaviour, that forwarding `&mut None` as the construction marker matches what the tests intend.
