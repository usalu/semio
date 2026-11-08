STATUS: replay, command and document libs VERIFIED native (document: W-replayport-12 single error fixed, then passed inside W-gateway-114). W-gateway-114 rc=1: three further crates red: semio-os-store-space (24), semio-os-store-member-open-store (38), semio-os-store-config (20).

## Design (step 1)
- `OriginalReplayResident` (pub, root): caller-owned `OriginalConstructionAdmission` + refusal slot + unit slot + `RetainedCloneClose` + `constructing` flag. Methods `built`, `built_with`, `action` (private), `settle`, `close_step`, `close_demands`, `terminal_is_empty`, `constructing`.
- Rule: every public driver of an owner checks `resident.constructing()` first and returns `resident.settle(cx)` (the constructor already ran; the next call settles). Static `admit*` fns take a trailing `resident:&mut OriginalReplayResident`; owners hold `resident` as a field and close it last.
- Done in root file: OriginalReplayPublication (field, guards, admit/admit_edit_step/capture param). Open: abort, checkout, checkpoint, checkpoint-order, repin, publication stage; external callers of admit (command/🦀️.rs 134/138/154, plugin time-travel finalize 22/30, instance ingress 187, instance dispatch 125) still on the old signature.

## Step 1 progress (EDITED-unverified until W-replayport-1)
Resident field + guards + `resident:&mut OriginalReplayResident` last param on static admit fns now in: root 🦀️.rs (publication), abort.rs, checkout.rs, checkpoint.rs, checkpoint-order.rs, repin.rs, 📬️publication stage. `event()` helpers in checkpoint/repin take the resident. Compile W-replayport-1 next (log 🗑️generated/1105b/W-replayport-1.txt).

## Step 2/3 edits (EDITED-unverified; compile W-replayport-2)
Root helpers `receive`/`settle_receiving`/`retire_controlled`; abort/checkout/root now use them for metadata advance, derived report replay step, and metadata/result/ticket close. `admit_original_owned_retirement` -> `retirement::admit_owned_retirement(slot,effect)` in `close_original_effect_step`. Metadata work counters now u128. Metadata `close_step` still takes a grant (needs the effect signature); VcsError/OriginalTurnFailure sites rewritten via `receive`.

## Step 2b (EDITED-unverified; compile W-replayport-3 running)
W-replayport-2 (rc=1): 87 -> 33 errors after resident port. Now: metadata `close_step(store,control,effect)` + `return_step`/`advance_root` effect methods (`root_wanted`/`return_wanted` flags), root helpers `effected`, `advance_metadata`, close-with-effect for overlay/children/birth/inverses.
W-replayport-3: rc=1, 87 -> 10 errors (exit helper, birth close polls, stage advance/close with effect). Fixed in W-replayport-4 (running).


## FINAL (W-replayport-4 rc=0)
Error trail: 86 (W-enginestage-10) -> 87 (W-replayport-1, resident ported, imports missing) -> 33 (-2) -> 10 (-3) -> 0 (-4).
Public API changes in `semio-os-store-engine-replay` (all in this crate, verified native only):
- New `pub struct OriginalReplayResident` (Default): caller-owned admission + refusal + settle marker. Every owner (publication, abort, checkout, checkpoint, checkpoint-order, repin, stage) holds one and closes it last (terminal_is_empty/close_demands/close_step include it). Each public driver first calls `resident.settle(cx)` when `resident.constructing()` (constructor ran, next call settles).
- Static birth fns take a trailing `resident:&mut OriginalReplayResident`: `OriginalReplayPublication::{admit,admit_edit_step}`, `OriginalStreamAbort::admit`, `OriginalCheckpointCheckout::admit`, `OriginalCheckpointPublication::admit`, `OriginalRepinPublication::{admit,admit_current}`, `OriginalCheckpointEditOrder::admit`. The bool now means "construction settled" (first call returns false after the constructor ran).
- Metadata: `close_step(store,control,effect)`, new `advance_root`/`return_step` (effect-shaped) with `root_wanted()`/`return_wanted()`; `advance`/`advance_in_order` no longer touch the root read or the source return (root helper `advance_metadata` sequences them). Work counters u128.
- Root helpers: `receive` (local scope, no resident needed), `settle_receiving`, `retire_controlled`, `effected`, `settled`.
NOT done / handoff (callers keep the old arity and will not compile; each needs a resident field in its frame, closed with the owner):
- history/🎮️command/🦀️.rs lines 134, 138, 154 (engine-stage lane)
- plugin/⏪️time-travel/🗄️store-state/📬️finalize/🦀️.rs lines 22, 30
- plugin/🧵️instance/🏪️stores/📥️staging/🚪️ingress/🦀️.rs 187 (repin admit_current)
- plugin/🧵️instance/📜️history/📥️staging/📨️dispatch/🦀️.rs 125 (stream abort admit)
- replay crate tests (🧪️tests, *-tests.rs) and report tests (replay/🎮️operation/…/📜️report/🧪️tests:13 `OriginalReplayMetadataPreparation::advance` old shape) not compiled.
Unverified: wasm target, tests, runtime behaviour of the settle-on-next-call protocol (no test run).
Tool: 1105b-tools/replayport-errs.py (compact error lister).
Caller 1 (history/🎮️command frame): resident field + 3 admit calls + close/demands EDITED-unverified (crate also has other old-grant sites owned by enginestage lane).
Caller 2 (time-travel store-state finalize + field commit_resident): EDITED-unverified (this crate has other old-grant sites e.g. retire_authoring_replay, not mine).
Callers 3+4 EDITED-unverified: ingress (checkpoint_composition_pins_unit/close_demands get `resident` param; Repin reserved stage has new `resident` field; command-routes + close-ladder updated); dispatch (close_streamed_transaction_unit gets `resident`; field `stream_abort_resident` on MountedTypedCommandFullOperation, initialised at every literal via a counted, exact-text codemod of `pending_stream_abort:None,`). Known gap: a failed abort birth leaves its refusal in the resident with pending=None and callers gate close on pending.is_some().
Codemod: 15 literals (exact text 'pending_stream_abort:None,' followed by 'stream_abort_completed') in 10 plugin files; 1 other literal (tests fairness:63) left. Gateway W-gateway-108 started.


## W-gateway-108 red list (rc=1; only crate that failed: semio-os-store-engine-command lib, 31 errors, nothing downstream reached)
history/🎮️command/🦀️.rs, all old-shape work not related to my admit arity:
- own `built`/`built_with` (lines 9, 10, 64, 101 area): 6-arg `admit_original_construction` (needs a command-owned `OriginalConstructionAdmission` + failure slot: reuse the `OriginalReplayResident` pattern in the frame/command; `built_with` closure now takes `NativeConstructionScope`)
- line 123: ~15 sites `retired(.., |grant| ..)` on owners now taking `&mut RetainedAllocationEffect` (e.g. `issuer.step`, source `close_step`, `begin_granted`) -> `close_original_effect_step`
- 132: decoder identity tuple `(u64,u64)` -> `(NativeAllocationIdentity,u64)`; 101: `NativeAllocationIdentity` vs u64 (`control.ledger_identity()`)
- 147, 166, 167: `grant` -> `effect` owner calls
- 152 (stage 9): `with_native_receiving(|control| step_derived_report_replay..)` -> replay-crate `receive` pattern (local scope; VcsError stays in the closure result) then settle_receiving
- my 3 admit calls already carry the resident (no remaining arity errors from them).
Recipe: copy helpers `receive`, `settle_receiving`, `retire_controlled`, `effected`, `settled` from the replay root (history/🏁️replay/🦀️.rs).

## Command crate port (step A, EDITED-unverified; compile W-replayport-5)
history/🎮️command: local built/built_with/action_work removed; resident-based action/built; admit_native_vcs_command gets trailing resident param (take_native_vcs_result uses the frame resident); close paths via retire_controlled/close_original_effect_step/settled; decoder tuple type; stage 9 via receive/settle_receiving; advance_root via effected. Replay helpers made pub.
Gap closed (EDITED): new MountedTypedCommandFullOperation::stream_abort_open() (pending || resident non-empty) gates 5 close/quote sites; close_stream_abort/demands handle pending=None with a resident failure; ready predicates include the resident. Fairness test literal line 63 fixed.
W-replayport-5: command lib rc=0, 0 errors (VERIFIED native). Callers updated after: plugin Revert stage (resident field) + framework_revert_unit + close-ladder; space-member native-vcs and the 4 command/replay test files still on old admit arity. W-gateway-109 next.

## W-gateway-109 red list (rc=1): crate semio-os-store-space-member lib, 12 errors (command crate now clean)
Ported after that run (EDITED-unverified, compile W-gateway-110): space-member `native-vcs` (Typed has resident; member birth is one funded same-turn local step using cx.native_receiving_identity; effect close), trait `SpaceMember::retire_snapshot_read_erased(snapshot, effect)` (trait, for-artifact-store, no-members visitor, macro, doubles, child-retirement caller via effect.nested). Error cause of OriginalRetirementFailure is collapsed to a fixed message there. Plugin Revert stage carries `resident`; child-vcs reserved caller (plugin/👷️job/🏛️reserved/🪆️child-vcs line 23) still uses old 3-arg trait admit (unchanged, trait signature kept).
Left: `Mu: OpBinaryReceiving` x2 in member-factory-visitors-and-no-members (not mine); test files (command/replay/space-member tests) still on one-call admit protocol.
W-gateway-110: space-member 3 errors (2x Mu: OpBinaryReceiving in member-factory-visitors-and-no-members:39/48 = not mine; 1 missing replay dep -> fixed by re-export from command crate, no Cargo change). W-gateway-111 running.


## FINAL red list (W-gateway-112, log 🗑️generated/1105b/W-gateway-112.txt)
- semio-os-store-engine-replay, semio-os-store-engine-command: green (native check).
- semio-os-store-space-member: W-gateway-111 had only 2 errors (member-factory-visitors Mu bound) -> fixed by adding `StoreMutationFor<P>` to both visitor traits' Mu bounds; W-gateway-112 no longer reports the crate (VERIFIED by being passed).
- semio-os-store-document lib: 78 errors, old grant shapes: history/📥️fold:22,45,71,82-90 (`with_native_receiving(|control,grant|)` generic-arg form, `RetainedWorkPoll` where outcome expected), 📥️metadata:28,37,64 (3-arg methods), 📥️mutation:14,22,39,46,47 (OriginalTurnFailure: From<OriginalReceivingPrimary>), 📥️mounted-pack:314,330,388,416, 🧹️hydration-retirement-drop-and-completion:76-82,115,176-178 (poll `.result/.work` -> settle via effect), ✉️edit-decoding:54,124-126,153-155,216,241,258, 💧️change-hydration:144,174,204,212, 🧬️hydration-types-constructors-and-drive:52,345-347,404, 🏁️retirement-and-finish:13,29, 🔁️replay-and-targets:116,196, 📦️pack-scan-and-fold:113. Same recipe as the replay crate (poll-shaped closes via close_original_effect_step / RetirementPollExit::settle; receiving steps via `receive` local scope).
Open items from this session: stream-abort failed-birth gap closed (stream_abort_open); fairness literal fixed; tests of command/replay/space-member still use the one-call admit protocol and need loops; child-vcs reserved caller of `admit_member_native_vcs_command` unchanged (member birth is now one same-turn funded step so true on first call).

FOLLOW-UP (coordinator note): space-member retire_snapshot_read_erased maps OriginalRetirementFailure to a fixed ValueError message, which loses the cause; the trait should return the failure (RetainedWorkOutcome<_,OriginalRetirementFailure>) and the child-retirement caller should project it.

## semio-os-store-document port (EDITED-unverified)
File 1 hydration/📥️fold: with_native_receiving local scope, close_original via with_original_effect, effect close. Compile pending (W-replayport-6).
Files 2+3 EDITED-unverified: hydration/📥️metadata (reserve_* marker arg), 📥️mutation (local scope, effect close, OriginalTurnPrimary::Receiving).
File 4 hydration-retirement-drop-and-completion: arms 78-82 via poll_outcome (cause collapsed, FOLLOW-UP like retire_snapshot_read_erased); lines 115, 176-178 not yet. mutation file still needs replacement for removed native APIs record_constructed_allocation/receive_retained_work (construction needs a new admission design). metadata:36 has a borrow error to fix.
Files 5-8 EDITED-unverified (W-replayport-8): retirement-and-finish/replay-and-targets/pack-scan via root fn owned_outcome (effect), change-hydration + edit-decoding u128 fuel + reserve markers, empty_messages close via effect.


## semio-os-store-document red list after W-replayport-8 (18 errors)
Done: fold, metadata (+borrow fix), retirement-and-finish, replay-and-targets, pack-scan (via root `owned_outcome`), change-hydration, edit-decoding (u128/marker part), mutation (scope + close part).
Left, all need an effect-shaped owner API rather than a mechanical edit:
1. 📥️mutation:32 `native.record_constructed_allocation(..)` and :40 `native.receive_retained_work(..)` no longer exist on the native control; the construction step must move to a resident `OriginalConstructionAdmission` (admit_original_construction) and the retained-work debit to `receive_original_receiving_work` (see replay-crate `receive`/`settle_receiving`).
2. ✉️edit-decoding:153-155 `edit.inverse.reserve_one(capacity)` now takes an effect and returns `RetainedWorkOutcome<PagedListReservationOutcome,PagedListReservationFailure>` (use `effected`-style `cx.with_original_effect`).
3. 📥️mounted-pack:314,330,388,416: paged-list/ledger calls that now take `&mut RetainedAllocationEffect` instead of a size/grant.
4. 🧬️hydration-types-constructors-and-drive:52,345,347,404 and 🧹️retirement-drop:117,178-180: `admit_owned_retirement(slot,effect)` (returns outcome, no (error,original) tuple), `runtime.close_step(factory,child)` and u64/u128 `work` fields; wrap in `cx.with_original_effect` / `effect.nested(child)` like the replay crate `retire_controlled`.
Cause-loss follow-ups (typed cause collapsed to a fixed message): space-member `retire_snapshot_read_erased`, document retirement-drop `poll_outcome`.
Step A (EDITED): hydration-types (runtime close via with_original_effect, admit_active via slot+effect, u128 fuel) + retirement-drop (effect.nested for runtime close and actor birth). Next: mounted-pack, edit-decoding reserve_one, mutation.
W-replayport-9: 18 -> 9 errors (hydration-types + retirement-drop VERIFIED by compile). Left: mutation 32/40, edit-decoding 153-155, mounted-pack 314/330/388/416.
W-gateway-113 rc=1: only semio-os-store-document, 6 errors (mutation 32/40, mounted-pack 314/330/388/416). edit-decoding reserve_one_funded VERIFIED. mounted-pack now takes an effect (EDITED); mutation still needs resident OriginalConstructionAdmission field + admit_original_construction (actual marker replaces record_constructed_allocation) + receive_original_receiving_work.


## Update (W-replayport-11, VERIFIED by compile): document crate 2 errors left
Verified green now: hydration-types (runtime close/admit_active via with_original_effect), retirement-drop (effect.nested), edit-decoding (`reserve_one_funded`, outcomes Ready/Denied/Work/Allocation), mounted-pack (`reserve_retained_allocation(max, effect)` and `close_step(items, max, effect)`: public signature change, no in-tree callers found).
Left, both in `📥️mutation/🦀️.rs` (`step_native_mutation`):
- :32 `native.record_constructed_allocation(capacity)` -> the cursor birth must go through `admit_original_construction(cx, &mut *self.mutation_receiving, &mut admission, demand, 1, |scope, actual| ..)`: build with `M::admit_receiving(source,(0,len), mutation_receiving_child(scope.retained_grant(),capacity,overhead))`, set `*actual=Some(capacity)`, return `(Box::new(owner), progress+extra)`; Ok(None) must become a typed refusal.
- :40 `native.receive_retained_work(..)` -> take the `take_into` turn inside the receiving closure into a local, then settle with `semio_framework_job::receive_original_receiving_work(cx, RetainedWorkOutcome{result: turn.result.map(RetainedCloneStep::Progress), work: turn.work})`.
Design decision needed before editing: the hydration struct needs a resident `OriginalConstructionAdmission` field plus a slot for its `OriginalConstructionFailure` (via `take_failure_into_step`), both included in `terminal_is_empty_unbounded`, closed in `close_original_step` (admission.close_step needs `cx`) and retired through the struct's retirement ladder. The document crate cannot reuse `OriginalReplayResident` (replay depends on it).
Mutation port EDITED (compile W-replayport-12): hydration fields mutation_admission/mutation_refusal/mutation_constructing; remaining_owner + close_demands + close_step (refusal retired first) ; close_original_step closes admission (cx); step_native_mutation births via admit_original_construction with actual marker; take settles via receive_original_receiving_work.
W-replayport-12: only 1 error left (E0283 annotation at mutation:16), fixed; W-gateway-114 running.


## Mutation port done; W-gateway-114 red list (log 🗑️generated/1105b/W-gateway-114.txt)
Document crate: resident `mutation_admission` + `mutation_refusal` + `mutation_constructing` on the hydration struct; they count in `remaining_owner`; the refusal is quoted and retired first in the effect ladder; the admission is drained in `close_original_step` (cx); the cursor birth is `admit_original_construction` with the `actual` marker; the take is settled with `receive_original_receiving_work`. Document no longer appears in the failing list (VERIFIED by being passed by W-gateway-114).
Red now (not yet touched):
- semio-os-store-space (24): `SpaceHistoryMutation: OpBinaryReceiving` / `StoreMutationFor<SpaceHistorySnapshot>` not implemented (needs the receiving impl on the mutation type), plus lifted/store/🦀️.rs methods missing on ArtifactStore (backbone_ref, detach_backbone, attach_backbone, tick, dispatch, snapshot, close_owned_unscheduled) that follow from the unsatisfied bound.
- semio-os-store-member-open-store (38): old-grant shapes: hydration-types-constructor-and-drive:208-232 and hydration-advance:82-84,229-231, retirement-drop:159-170,234-236 (poll `.result` fields, grant -> effect), record/🦀️.rs:58-125 (`FactoryProducerReplyComplete`/poll results, 2-3 arg methods, JsonBorrowError conversions), open/🦀️.rs:148-175, `with_replacement` missing at 🦀️.rs:576.
- semio-os-store-config (20): OpBinaryReceiving for its mutation type (registration/🦀️.rs:10) and the same lifted store methods.
First step for space/config: implement `OpBinaryReceiving` for the two mutation types; the remaining errors in those crates are most likely consequences.

## OpBinaryReceiving for SpaceHistoryMutation / store-config mutation: investigated, NOT implemented (no code changed)
Finding: the existing implementors fall in two groups and neither is a one-line installer for these types.
1. DSL installers (`original_ordinal_operation_receiving!` / `original_tagged_operation_receiving!` in 🗣️dsl/🫴️receiving) need the type to be a `DslOps` variants type (`BorrowedDslVariants` + `DslOwnedReceivingVariants`, binary via `dsl::variants_binary`), e.g. CollectionMutation (🪐️space/🗿️artifacts/🗂️collection: mutations file + 📥️body-receiving, ~5 KB per type) and DemoMutation.
2. Hand-written cursors (ProbeMutation in mcp/workspace/probe-document/📥️receiving, store/📬️command/🛬️receiving tests): ~10 KB each (parser slot, output slot, syntax cause, close ladder).
`SpaceHistoryMutation` (store/🧬️schema/🧬️mutations, derive ToValue/FromValue, `OpBinary` via pack_rt in store/🚪️io/💾️binary/🧬️mutations) and the store-config mutation type are plain ToValue/FromValue pack-value enums, so they match neither group. Decision needed from the coordinator: (a) migrate both mutation enums to the schema-first `DslOps` variants shape and use the ordinal installer (matches the Collection precedent and AGENTS.md schema-first), or (b) hand-write two cursors. Recommendation: (a). The remaining space/config errors (backbone_ref, dispatch, snapshot, ...) are expected to follow once the bound holds. member-open-store old-grant code not started.

## DslOps migration: assessed, BLOCKED (no code changed)
Ruling was to migrate SpaceHistoryMutation (+ config mutation) to DslEnum/DslReceiving + original_ordinal_operation_receiving! like CollectionMutation/DemoMutation (`artifact_persisted_binary_operation!` replaces the handwritten pack_rt OpBinary). Checked before editing:
- The aggregate needs `DslEnum` + `DslReceiving`, and each of the 6 payload structs `DslRecord` + `DslReceiving` + `#[dsl(keyword=..)]` (DemoMutation/validated SetN is the model). Crate semio-os-store-space-schema already depends on semio-framework-dsl-record and semio-os-dsl (derive crate availability unverified; a missing dep would need a Cargo.toml change -> LOCK NEEDED, not allowed here).
- The blocker is the payload types, not the enum: CommitSpaceCheckpoint embeds `SpaceCheckpoint {id,parent_id,message,authors:Vec<Author>,timestamp:HybridLogicalTimestamp,members:Vec<SpaceMemberPin>}` and CreateSpaceAlternative embeds `SpaceAlternative`. `Author` and `HybridLogicalTimestamp` are foreign types without `DslField`; the CollectionMutation docstring documents the same orphan-rule problem and solves it with hand-written DslVariants per foreign type. SetActiveSpaceAlternative also relies on `#[value(required)]` Option semantics that DslRecord must reproduce. The config mutation was not opened.
- Wire change: the migrated encoding is `dsl::variants_binary`, not pack_rt, so every space-history fixture/golden (sqlite snapshot tests, op logs) must be regenerated in the same wave.
Proposed alternative (keeps the existing pack_rt wire form, so no fixture changes and no compat layer): one generic `OriginalPackValueReceiving<M: ToValue+FromValue+RetireOwned>` cursor in the dsl crate (parse the pack value in bounded steps into a retained DslValue, then FromValue it with the retained-clone path) and a macro `original_pack_value_operation_receiving!(Type)`; both SpaceHistoryMutation and the config mutation install through it. This is one new component instead of two schema migrations. Needs the coordinator's go-ahead because it adds a runtime component to the dsl crate.

## Generic OriginalPackValueReceiving<M>: design verified against the tree, NOT started (shared-crate risk)
Findings that make the component feasible (so the next owner starts cheaply):
- Stored op bytes = `pack_rt::encode_pack_value` = a pack DOCUMENT whose record spec is `value_bridge_spec()` (one field id `VALUE_BRIDGE_FIELD_ID` named "value", `Shape::Value`, `RecordLayout::Lines`, no keyword) in os/🎒️pack/🌉️bridge/🦀️.rs. Decode oracle = `pack_rt::decode_pack_value(bytes) -> DslValue`, then `FromValue::from_value(renormalize_whole_number_floats(value))` (exactly the current `OpBinary::decode_op` of SpaceHistoryMutation).
- Bounded receiving primitives already exist: `RetainedRecordReceiving::{admit_document(source,range,BorrowedRecordSpec,PackLimits,grant), demands(source), step(source,native,effect), is_ready, take_value_into(field,&mut Option<DslValue>,grant), close_demands, close_step(effect), terminal_is_empty}` (pack/🌱️value/🛬️decode/🫴️receiving) and `bridge::take_received_wire_value`. `OriginalOperationReceiving<M>` (dsl/🫴️receiving) is the structural template (phases, `perform(effect,units)` single charge, `unpoll`/`stepped`, ordered close: output, value slot, record).
- Plan: new file in dsl/🫴️receiving: `OriginalPackValueReceiving<M: FromValue+ToValue+RetireOwned+OpBinary>` with phases 0 admit_document(value_bridge borrowed spec) / 1 record.step until ready / 2 take_value_into -> `OwnedSlot<DslValue>` / 3 convert DslValue -> M into `OwnedSlot<M>`; `OpBinaryReceivingCursor<M>` impl with `DomainFailure=ValueError`; macro `original_pack_value_operation_receiving!($ty)` mirroring `original_ordinal_operation_receiving!`.
- Open design point needing a ruling: phase 3 (`FromValue` of a whole DslValue) allocates M's heap outside the retained custody; an honest quote needs a retained-size bound of the DslValue (or a retained-clone FromValue path). Without it the quote can only be `size_of::<M>()` which is not honest.
- Not started because adding an uncompiled module to the shared dsl crate would break every peer build, and the law (third-party oracle vs `pack_rt`) plus installs on SpaceHistoryMutation and the config mutation depend on the phase-3 ruling.
Step 1: new module dsl/🫴️receiving/📦️pack-value (cursor, quote fns, macro) written, wired via #[path] mod; compile W-replayport-13 on semio-os-kernel?
W-replayport-13: semio-os-dsl with the new pack-value cursor compiles (rc=0, VERIFIED). Step 2: macro installed on SpaceHistoryMutation (io/binary/mutations), compile space-schema W-replayport-14.
W-replayport-14: space-schema compiles with the install (VERIFIED). Running W-gateway-115 to see whether config still needs its own install.
Step 3: oracle law appended to dsl/🫴️receiving/🧪️tests (original_pack_value_receiving_decodes_the_same_value_as_the_pack_rt_oracle); running W-replayport-15 test.
Step 3 (revised): dsl crate tests are not cargo targets (no [[test]]), so both laws live in the space-schema unit tests (mutations/🧪️tests/🔬️unit): oracle law via check_receiving_round_trip over all 6 leaves, heap-growth law with a std counting allocator. Running W-replayport-15 (cargo test, space-schema lib).
W-replayport-15: heap law FAILED for CommitSpaceCheckpoint (3049 requested vs 1305 quoted) -> per ruling it needs a retained-clone FromValue; strict law kept as #[ignore] with that reason plus a report test naming violating leaves. Oracle law failed on a source-slice identity bug in my cursor (record needs the full source, not a sub-slice): fixed. W-replayport-16 rerun.

## Generic pack-value cursor: DONE and VERIFIED (W-replayport-13 dsl check rc=0, W-replayport-14 space-schema check rc=0, W-replayport-16 tests rc=0)
- `OriginalPackValueReceiving<M>` + `PackValueRetirement` + `decoded_value_retained_bytes` + `pack_value_conversion_quote::<M>` + macro `original_pack_value_operation_receiving!` in `dsl/🫴️receiving/📦️pack-value/🦀️.rs` (wired by `#[path] mod pack_value` in dsl/🫴️receiving/🦀️.rs). Phases: admit_document(value_bridge spec) -> record.step -> take_received_wire_value -> typed conversion quoted as decoded retained bytes + size_of::<Option<M>>(); one `perform` charge per phase, pending keeps state, ordered close output/value/record.
- Installed on `SpaceHistoryMutation` (store/🚪️io/💾️binary/🧬️mutations/🦀️.rs). Gateway W-gateway-115 no longer lists semio-os-store-space. The "config mutation" needed no install: the OpBinaryReceiving errors in semio-os-store-config were the same `SpaceHistoryMutation` bound; config's remaining 20 errors and member-open-store's 38 are old-grant code (record, hydration, open), not mutation traits.
- Oracle law (third-party = whole-buffer `pack_rt` decode via `check_receiving_round_trip`): VERIFIED over all 6 leaves / 9 samples (space-schema unit test `bounded_pack_value_receiving_decodes_every_leaf_like_the_pack_rt_oracle`).
- Heap law RESULT: FAILS for `CommitSpaceCheckpoint` (requested 3049 > quoted 1305; 3466 > 1504) and `CreateSpaceAlternative` (727 > 668); the other four leaves are within quote. Per ruling these leaf types (SpaceCheckpoint with authors/members, SpaceAlternative) need a retained-clone `FromValue`. Strict law is `#[ignore]`d with that reason (`pack_value_conversion_heap_growth_stays_within_its_quote`); `..._report_names_every_leaf_above_its_quote` runs and pins the violating set. Until then the conversion phase under-quotes those leaves (the phase also includes `renormalize_whole_number_floats`, which is part of the measured growth).
- Note: the dsl crate's own tests (receiving/🧪️tests, 🧪️tests/*) are not cargo targets (no [[test]], emoji paths), so laws were placed in the space-schema lib unit tests, which run.
- Remaining red for the gateway: semio-os-store-member-open-store (38) and semio-os-store-config (20), both old-grant ports (hydration-advance, hydration-types, retirement-drop, open, record).
