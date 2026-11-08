STATUS: GREEN native: io-control (port-f-3, wasm port-f-4 also rc 0), SnapshotProjection derive (port-f-6), pack-json-native-input (port-f-9), io-sqlite-snapshot (port-f-13). wasm32-wasip2 for the last two UNVERIFIED: port-f-14 stopped in semio-framework-value-retained-clone (dynamic/🦀️.rs:49, a peer's in-flight edit, not port-f). Crate tests UNVERIFIED.

Decisions
- Snapshot effect closures keep the `performed` flag: the decode side has no public performed receipt. The encode side binds it to effect.performed(1) once inside effect_allocated (single charge).
- Frame-level physical quotes use NativeEncodeControl::construction_grant/submit_constructed_progress, re-added in value/🛫️encode/🏭️construction (copy of the decode version).
- Retirement of producer inputs runs inside effect_allocated (encode) or receive_effect (decode).
- value facade re-exports record_retirement_work.

API changes for consumers
- NativeSnapshotEffectProgress = {state, failure}, Deref to NativeEncodeEffectState.
- NativeSnapshotEncodeAllocation = {state, failure}.
- NativeSnapshotEncodeEffectQuote removed.
- begin_original_settlement drops work_quote and returns bool.
- record_effect takes 1 arg.
- resolve_original_quote -> recover_original_quote; resolve_original_admission/post removed.
- copy_text_encoded returns RetainedWorkOutcome.
- Pending replies map to an InvariantViolated refusal (non-resumable).

Not done: io-control tests; consumers (io-sqlite-snapshot, dsl, store native-*, space-history native) not rechecked.

## SnapshotProjection derive
- value/✨️derive/📽️projection/🦀️.rs: close_step work is u128; child effects use effect.nested; bind(effect); begin_granted replaces begin_from (the duplicate bitwise copy is forgotten); polls from step_granted and close_retained_binding are settled, a Pending maps to a zero-progress step; added the missing rank arm.
- value/📽️projection/📋️field and 🦀️.rs: reborrow replaced by nested/nested_within.
- energie informed.

## Pending reply
runtime ruled A (guard only): host recipients are synchronous, Pending exists only on the recovery arm after an unwind or cancellation. The InvariantViolated refusal stays; the marker state stays retirable. A law for this is still to be written.

## Done after the first report
- pack-json-native-input: advance(control,body,max_work,g) is a Poll over resident NativeReceivingChild+NativeLocalAuthority; take(control,output,effect); close_step(control,effect)->Poll; demands(body,max_work).
- io-sqlite-snapshot: receive macro on the new receive(output,op) shape; Port::effect encode arm via the new NativeSnapshotBodyWallet::effect_encoded; work receipts u128 (io-sqlite-model too); schema-validation page reserve through AuthoringWallet; paged_text::copy_text added; Result alias with a default error for the RetireOwned derive.
- verfolgen got the io-control mapping.

## Open (consumer slice, mine)
- dsl token/receiving, spr archive decode, store native-{encoding,decoding,retirement}, space-history native: not started (mapping sent to verfolgen).
- Old open notes below are superseded for pack-json-native-input and io-sqlite-snapshot.
- pack-json-native-input: public driver (advance/take/close_step with control and grant) needs a redesign onto effects: NativeReceivingOriginal::advance is a Poll, receive_retained_work is gone, RetainedCloneSourceAdmission and JsonGrammarCursor are effect-shaped. It touches the callers in pack-json decode, so it needs a coordinated slice with aggregator/valuefix.
- io-sqlite-snapshot: 71 errors in the consumers of the io-control closures and carriers (copy_text, NativeSnapshotEncodeEffectQuote import, Poll shapes).
- replication-ids stays with port-a.

## io-ports and pack-format-retained
- io-ports: vector prepare `advance(read, effect)` and `close_step(effect)` take the caller's effect (bind and close_one run on `effect.nested`). No in-tree callers. GREEN native.
- pack-format-retained: source-cursor reserve_page and close_step use AuthoringWallet for the PagedList reserve/release. GREEN native (port-f-15 covers both).

## Consumer slice (dsl, spr, store codec, space-history)
Packages: semio-framework-dsl-record, semio-os-channel, semio-os-store-codec-registry, semio-os-store-space-history; manifest is the repo-root Cargo.toml.
UNVERIFIED: the run port-f-17 stops before them in two crates:
- semio-framework-dsl-record-model (55 errors, verfolgen).
- semio-framework-pack-format-catalog (30 errors, unclaimed): same PagedList-needs-effect class as pack-format-retained (symbols/storage, symbols/decode, symbol-table, catalog-cursor). The AuthoringWallet recipe from pack-format-retained applies. Next for port-f.

## Schema validation wallet (runtime2 ruling)
SchemaValidationStorage::parse keeps AuthoringWallet (import_sqlite_database_controlled is a no-turn entry). The stock is sized from the control's remaining allocation and the performed work goes to control.record_performed_work. Native green (port-f-21). Open: ProjectionStorage::initialize reaches construct_database_into from a control-only entry; the hosts sweep must confirm no snapshot decode turn calls it, otherwise it needs owner.native().receive_effect.
