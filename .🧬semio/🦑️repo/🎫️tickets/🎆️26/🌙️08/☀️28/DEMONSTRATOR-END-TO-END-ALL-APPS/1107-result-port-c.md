STATUS: GREEN (lib checks) for ui-locale, ui-contract-text, ui-raster-owner, ui-contract-identity (via derive), replication-retained-field. UNVERIFIED: locale tests; wgpu caller not ported.

Proofs (logs in `C/🗑️generated/1105b/`):
- port-c-2 native rc 0, port-c-3 wasm32-wasip2 rc 0: semio-ui-locale, semio-ui-contract-text, semio-ui-raster-owner.
- port-c-5 native: semio-ui-contract-identity no longer errors (fixed in `✨️derive/🛬️receive`: transparent newtype now emits `from_native`; enum NativeDefault emits `default_cursor()`).
- port-c-8 wasm32-wasip2 rc 0: semio-framework-replication-retained-field (56 errors -> 0). Preparation/Change/Indexed/Record take sources + `effect`; `close_step(effect)->OriginalRetirementPoll`; u128 work settled once via `field::settle`/`settle_exit` (owed remainder recorded on the effect, refusals merged, never dropped); unwraps removed. `RetainedRecordChangeField` trait changed accordingly: downstream mutation crates must adapt.

Open:
- wgpu `🛫️render-upload-and-eviction/🦀️.rs:129` (`SceneRasterLease::close_step(effect)`): not ported. semio-ui-prepared is unreachable (port-c-4/5: blocked by pack-json, io-control, async reds) and its whole close chain (job, packet, input, mailbox, staging, atlas pages) is grant-based; needs a dedicated slice.
- locale tests: `♻️localized-label-retirement` ported (compiles, uses `owner/🧪️work/effect.rs` harness). `🌐️axis-value` test (full receive driver on the old advance/take API) not ported, so `-p semio-ui-locale --lib` tests stay red (port-c-9).

## Update (later in wave)
GREEN, wasm32-wasip2 rc 0 (port-c-17) and native rc 0 (port-c-16): semio-framework-dsl-record-ownership (was 41 errors, fleet blocker), semio-framework-replication-error, semio-ui-scene-mesh (port-c-12 native).
- dsl-record-ownership: `#[derive(RetireOwned)]` on Shape/FieldSpec/RecordSpec/FieldValue/RecordValue/ExprValue (manual Box<dyn> cursors removed), leaf for RecordLayout/ExprOp/&'static UnitSpec (dsl-token gained a path dep on semio-framework-value; no cycle), RecordFields via artifact_retire_struct!, RecordPlanningRetirement on effect, native-source `query_scalar`/`recover_scalar`, `reserve_one_cold`.
- scene-mesh: `authored_step` runs controlled retirement under an explicit finite `AuthoringWallet` (mesh authority closes outside a job turn); public mesh3d_* grant API unchanged.
- derive/🛬️receive: transparent + enum-default generators emit the new API (contract-identity fixed).
Open (unchanged, edits pending a compile frontier): ui-prepared grant-based close chain + wgpu caller; replication-retained framework layer (root, set-field, patch, diagnostic, diff) and plugin crates using the changed RetainedRecordChangeField API; dsl 🛫️encoding:292 / 🛫️encode:159 still call RetireOwned::retirement. Frontier last seen: semio-framework-diagnostic (aggregator/jobsplit), replication-ids (port-f).

## Update 3
- FIXED-verified (no error in port-c-19): dsl `🛫️encoding` RetainedRecordWriter (children admitted with `admit_retirement_cursor` -> ChildProgress, planning buffers as plain Vec retirement; RecordPlanningRetirement deleted) and `🛫️encode` RetainedFieldProjection/FieldProjectionFrame (owned-field structs + `artifact_retire_struct!`).
- TRULY BLOCKED: semio-framework-dsl-record-model, 66 errors (port-c-19): `🫴️receiving/📥️owned/*` (intrinsic 16, persistent 11, wrapping 6, storage 5, contract 4, text 3, vector 2, mapping 1), `🛫️encode/🧮️expression` 11, `🪆️binding` 2, `🛫️encode/🛬️receiving` 1. All sit on the io-control `NativeSnapshot{Decode,Encode}EffectQuote`/`NativeSnapshotDecodeOwner` protocol rework (ledger decision Option 1, see `1107-result-koordinator.md`: ~20 producer closures move to `|physical,effect|`, quotes re-expressed on `effect_allocated`). That is a multi-hour compiler-in-the-loop slice for the io-control consumers (dsl token/receiving, io-sqlite-snapshot, spr, store native-*); porting it blind would race koordinator/ledger.
- NOT STARTED (behind dsl-record-model, replication-ids, diagnostic): ui-prepared close chain + wgpu caller; replication-retained framework layer and plugin users of the changed RetainedRecordChangeField API.

## Update 4 (replication-mutation, coordinator order)
GREEN native + wasm32-wasip2: semio-framework-replication-mutation (port-c-20, wasm in port-c-30) and semio-framework-replication-edit (port-c-28, wasm port-c-30).
- mutation: MapDelta/MapEntryDelta/MapEntryOperation derive RetireOwned (+ MapPresence leaf); foreign ErasedSnapshotRetirement impls on effect with nested child polls and settled work.
- edit: Edit/MutationOrigin derive RetireOwned; canonical identity/authority/seal cursors `advance`/`close_step(effect)->OriginalRetirementPoll` (`settle_exit` in 🧵️canonical). Public grant forms removed: external callers (pack-record encode etc.) must adapt (handed to port-d).
- Next frontier on my path: semio-framework-replication-mutation-messages (54 errors, port-c-29): `♻️retirement` 25 + `🧳️prepared/♻️retirement` 11 + accumulate/retire/publication/copy/clamp/compose/concat — needs the factory-retirement protocol (`FactorySharedRetirement::admit` removed, `FactoryProducerReply`) and is claimed but not started. Also red: semio-framework-job-context (13). ui-prepared close chain, wgpu caller and replication-retained layer remain behind these.

## Update 5
GREEN native: semio-framework-replication-mutation-messages (port-c-34). wasm UNVERIFIED: port-c-35 hit a peer's in-flight edit in value-native-decoding `🏭️construction` (`SealedSharedBirth` undeclared), not mine.
- Replay factory retirement on effect: `ArtifactReplayRetirementFactory::{snapshot,mutations}(&mut Option<..>,&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect)`; snapshot via `FactorySharedRetirement::admit_original`, owned values via `factory::factory_retire_owned`, mutations frame allocated under the effect marker; `ReplayRetirement::{close_step,close_step_controlled,close_original_step}` and `ReplayMutationsRetirement::close_step` return `OriginalRetirementPoll` with nested child effects and settled u128 work (`settle_exit`, `erased_close` in `♻️retirement`).
- accumulate/clamp/publication/`prepared_retirement::close`: PagedList reserve/release on the effect; `OperationMessageDrain::step`, `ReplayMessageAccumulator::{append,close_step}`, `FinalMessageSelection::step`, `FinalMessageSource::release`, publication `select` take the effect. External callers (store replay, conflict) must adapt.
- Next: semio-framework-replication-retained (162 errors, port-c-36; claimed, not started): root `🦀️.rs`, `🎛️set-field`, `🩹️patch`, `⚠️diagnostic`, `🔺️diff`, `🔑️key`, `✂️span` are on grant+actual and need the same effect port (the `RetainedMutationLeaf::decide/inverse` trait is implemented by many plugin crates). Then ui-prepared (also red upstream: semio-framework-job-native-encoding 5, jobsplit/others).

## Handoff (final)
Released: replication-retained -> lane replretained (a32d3e02acb32f364); ui-prepared close chain + wgpu caller (unstarted) -> a future lane.

### replication-retained breakdown (port-c-36, 162 errors)
🔺️diff/🪢️sequence 49, 🎛️set-field 39, ⚠️diagnostic 27, root 🦀️.rs 20, 🔺️diff/🦀️.rs 12, 🩹️patch/🚚️apply 8, ✂️span 6, 🩹️patch 1. Also red upstream of it: job-native-encoding (5), and ui-prepared needs job-context (jobsplit).

### Mapping notes (proved in replication-retained-field, mutation, edit, messages)
- Signature: replace `(g:RetainedCloneGrant, actual:&mut Option<usize>)` by `effect:&mut RetainedAllocationEffect<'_>`; `let g=effect.grant(); let base=effect.performed_work(); let mut work:u128=0;` at entry; `effect.actual()` is the allocation marker.
- Child calls: `effect.nested(child_grant)`; accumulate `work+=turn.work` (child totals); end with a settle that records only the owed remainder `work-(effect.performed_work()-base)` via `effect.performed(owed)` (refusals merged with `OriginalRetirementFailure::from_owned_parts(..)`, Pending kept via `RetirementPollExit::finish`). Copies exist in `field/🦀️.rs` (`owe/settle/settle_exit`), mutation/foreign, canonical `settle_exit`, messages `♻️retirement`.
- Closures must be annotated `(||->Result<RetainedCloneStep,RetirementPollExit>{..})`; bare `return Err(refusal(..))` needs `.into()`.
- Poll children: `RetirementPollExit::settled(poll)?` ; other typed polls (take_authority, factory_retire_owned): match Pending -> `return Err(RetirementPollExit::Pending(work))`.
- `RetainedFieldPreparation`: `demand/advance_work_demand/advance` now take the source `RetainedCloneRef`; `take(grant)`/`install` unchanged; `begin_close_paid(effect)`; `close_step(effect)->Poll`.
- RetainedFieldChange: `decision_demand/decision_work_demand(authored,original,body,max)`, `decide(authored,original,body,max,effect)`, `inverse_demand/inverse_work_demand(original,..)`, `inverse(original,body,max,effect)`, `close_step(effect)->Poll`; take_change/take_inverse still take a grant.
- RetainedRecordChangeField trait: `demand/work_demand(a,o,body,max)`, `advance(a,o,p,body,max,effect)`, `close_step(effect)->Poll`; `RetainedRecordChange::field(index,work,grant,actual)` and `boxed_record_field` unchanged (forward to equality_box with `effect.grant()/effect.actual()`). RetainedRecordChangeOwner and RetainedIndexedReplacement: `demand/work_demand/advance` take (a,o) refs; `advance/close_step` on effect; unwraps removed.
- Ripple: every implementor of `RetainedMutationLeaf::decide/inverse` (trait in replication-retained root, impls in layout/draw/idml/wfc/remodel plugins and set-field) must change `g,actual` to `effect` and call the field/record APIs above; their `*_demand` quotes gain the source refs.
- PagedList: `reserve_one(effect)`, `release_empty_page(effect)` -> `RetainedWorkOutcome<PagedListReleaseOutcome,PagedListReleaseFailure>` (Denied/Ready), cold grant-only `reserve_one_cold(bytes)`.
- Derives: prefer `#[derive(RetireOwned)]` / `artifact_retire_struct!` / `artifact_retire_leaf!` over manual `Box<dyn RetirementCursor>` impls (done for Edit, MutationOrigin, MapDelta*, DSL value types); manual cursors implement `RetireOwned` with `type RetirementCursor`+`retirement_seed`.
- Erased cursors in physical-only paths: wrap the inherent grant step with `crate::replay_retirement::erased_close(effect,|grant|..)` style helpers.
