STATUS: parent lib VERIFIED (W-durablegroup-107 rc 0, native). Companion tests NOT run: W-durablegroup-110 (136 errors) is red in the test closure upstream in store history/replay (checkpoint-order.rs, abort.rs, publication.rs OpBinaryReceiving bound, checkout.rs), none in durable-group yet seen; wire testing-helper close_step ported to AuthoringWallet (unverified). wasm not run.

## Part 1 (parent durable-group ported to effect-shaped wire/stage owners)
- Root helpers `effect_turn` / `close_turn` (durable-group/🦀️.rs): one original effect from the caller's `NativeDecodeControl` receiving turn (`receiving.with_effect(grant.retained_grant(), u64::MAX, ..)`, `None` = not admitted).
- map-assembly-open: `close_assembly_publication` x3 and `retire_originals_step(.., grant, receiving)`.
- record-and-snapshot-capture `retire_originals_step`: decision_identity/decision_encoding `close(effect)`, `ControlledRetirement::step(effect)` + `settle_close_poll`.
- commit-steps: `close_journal_owner(effect)`, decision/output retirement `step(effect)`; `receiving` threaded through coordinator `advance`, `advance_recovery`, `advance_with_sink`, commit operation `advance`, MapCommitHost / MapRecoveryOperation / MapRecoveryHost `advance(grant, receiving)`.
- recovery-completion `advance_binding` phase 14: `native.with_effect` around identity/encoding `close(effect)`.
- New dependency: parent Cargo `semio-os-store-read` (for `settle_close_poll`). LOCK NEEDED logged.
- Tests (companion-run unit fixtures): `receive_turn` helper; 28 `host|coordinator.advance` sites; fake-journal `observe_close_journal` (3 sites); 3 `retire_originals_step` sites. Not compiled yet.

## Part 2 (mint ruling ii)
- publication `ArtifactStoreOneItemLiveAuthority`: new named `from_recovered_outcome` (recovery verification of decoded data); `mint_for_store` builds the struct directly (live events keep the log witness); generic `mint` gated `#[cfg(any(test, feature = "testkit"))]`.
- Switched to `from_recovered_outcome`: stage durable-group 🦀️.rs :487 and :617, wire outcome-codecs :131.
- `testkit` dev-dep/feature on publication added to: stage, paged-one-item, one-item-owners, snapshot-clone (+ feature `law`), durable-group, durable-group companion, wire feature `durable-group-testing`. LOCK NEEDED logged.

## Rerun commands (after engine-bundle line 148 is fixed)
- native lib: C/1105b-tools/chk.sh W-durablegroup-105 "<durable-group>/📦️packages/🦀️rust/Cargo.toml" native -p semio-os-store-durable-group
- wasm: same with `wasm` and label W-durablegroup-106
- tests (companion crate): C/1105b-tools/runtime/tst.sh W-durablegroup-107 "<durable-group>/🧪️tests/🥧️companion/📦️packages/🦀️rust/Cargo.toml" native
- Then check each crate that gained `testkit` dev-dep (stage, paged-one-item, one-item-owners, snapshot-clone, wire with durable-group-testing) still compiles its tests.
- Risk: companion tests assert exact heap counts around retire_originals_step and journal close; the fixture receiving turn (FixtureStoreReceiving) may need adjusting at runtime.

## Update (OpBinaryReceiving wave)
- Cause: `semio_os_spr::StoreMutationFor<P>` (blanket trait in spr/📏️store-bounds) now carries `OpBinaryReceiving`; `ArtifactStore<P,M>` requires it. Fix at source: appended `+ semio_os_spr::StoreMutationFor<XP>` to all 96 Parent/Drawing/ValueMutation where-bounds in the parent crate (+ `captured_store_snapshot`). 345 errors -> 6 -> 0.
- `with_encoding_receiver` now needs caller frame/result custody: new `encoding_turn` helper (durable-group/🦀️.rs) used by assembly Binding.
- Same bound fix may be needed in store history/replay (`M: OpBinaryReceiving` errors at 🕰️history/🏁️replay publication.rs) and stage/wire if they hand-spell the mutation bound.
- Rerun: tst.sh W-durablegroup-111 <companion Cargo.toml> native --lib (after history/replay is green); chk.sh wasm for the parent; wire check.

## Update (history/replay)
- engine-replay is CLAIMED by enginestage (claims line 1676), so I only logged the `StoreMutationFor<P>` bound fix for them in 1105b-claims.md and edited nothing there. Companion tests stay blocked until semio-os-store-engine-replay is green; rerun: tst.sh W-durablegroup-111 <companion Cargo.toml> native --lib.
