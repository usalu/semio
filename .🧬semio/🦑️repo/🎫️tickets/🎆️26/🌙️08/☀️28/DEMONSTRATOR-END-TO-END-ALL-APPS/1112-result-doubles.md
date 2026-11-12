# 1112 doubles (lane doubles, 11/12)

## Step 1: semio-os-store-testing-doubles (EDITED, compile W-doubles-2 running)
Root causes (108 errors): (1) `SnapshotRetirementFactory` takes `SharedOriginalStorage<P>` not `Arc<P>`; (2) `close_step/prepare_close_state_step/close_state_step/artifact_retirement_box_close_step/FactoryAuthority::step/FactoryChildTickets::close_step` gained `body`; (3) Sync bounds cascade (`DocumentStoreOwners<P,M>`, `UnsupportedMemberFactoryOpen<M>`); (4) caller doubles built on removed `OriginalCallerSourceParent::admit`.
- retirement: factories on SharedOriginalStorage; ExactDemoSnapshotRetirement elects owner via try_owner/take_value/release_empty; `pump_turn` (OriginalRetirementPump, no AuthoringWallet) replaces every wallet use (close_granted, drive_retirement_terminal, admit_demo_alias_retirement).
- preparation/store/retained-edit: body params.
- callers: OriginalEnvelopeCaller and OriginalMemberCaller are now thin wrappers on the shared `OriginalJobFixtureCaller`; envelope_close_turn uses pump_turn.
- space-member lifted/operation: UnsupportedMember*Open bounds relaxed from Send+Sync to 'static (needed for non-Sync test stores).

## Step 2: semio-plugin-testing-laws lib
VERIFIED W-doubles-7 wasm ("🌎️hub/Cargo.toml") rc0. Edits in plugin/testing/🧪️artifact-app-laws: `fixture_pump_turn` (host pump, replaces AuthoringWallet), effect-shaped close helpers (`close_original_effect_step`), document archive admission + OriginalCommandExchangeState threading, JobPayloadAuthority::admit marker, with_native_receiving out-slot, decoder closed through `close_on_source`, OpBinaryReceiving bound.

## Step 3: gateway --tests VERIFIED W-doubles-11 rc0 (fixtures, doubles, laws). Fixtures config-selection unit test ported to bank_run (dev-dep testing-admission; LOCK NEEDED logged).

## Step 4: tests-12 --tests (NOT green)
- Fixtures: TestMutation + SetCount/SetLabel/SetSlotChildren got NativeReceive + DslReceiving + `dsl::original_ordinal_operation_receiving!` (Demo recipe); removed 222 `OpBinaryReceiving` errors. W-doubles-12 397 -> W-doubles-13 175 errors, rc101.
- Remaining (tests-01 34, tests-02 29, tests-05 109): test-code API drift (grant-shaped close_step, RetainedWorkOutcome iteration, MountedTypedCommandFullOperation.reserved_construction, PendingLatestWinsCommand.retired_key, InteractiveJob::step arity, SharedControlledRetirement::lease, missing semio_plugin_mounted_owner dep import). Not touched.
- Set-label consumer (child-operations laws/owner): not ported; PagedText `read_from_encoding_source`/`encode_with_inline`/`label_into` no longer exist anywhere, so the fixture paths (owner file too) need a design decision on the paged-text decode API.
- Rerun: "$C/1105b-tools/chk.sh" W-doubles-N Cargo.toml native --tests -p semio-plugin-instance-ladders -p semio-plugin-tests-12
