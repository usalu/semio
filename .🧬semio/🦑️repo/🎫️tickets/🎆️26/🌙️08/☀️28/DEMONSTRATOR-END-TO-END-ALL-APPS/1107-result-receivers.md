STATUS: GREEN for `cargo check` (lib, native) of semio-framework-value-native-owned: W-receivers-110 rc 0, 0 errors (log: 🗑️generated/1105b/W-receivers-110.txt). NOT verified: tests (`--tests`), wasm target, downstream crates using value_codec!/NativeSequenceCollection (paged, etc.), runtime laws.

## Ported to select_advance / advance_nested / advance_local / take(effect,reply) / close Poll
tuple (ledger, now compiles), 🔀️variant (selector + Result), 🔢️unit, 🔄️map mapped + try-mapped (protocol native_mapped_receiver_engine!), 📦️box, 🔢️array, 🌳️value scalar/string/option/sequence, 🗂️map, 🧺️set, 📋️list, 🔗️sealed-shared-receiver.

## Protocol layer (🧬️protocol)
- ordered_shared_admission! now: native_poll_local(child, control, demand, grant, units, max, authority, &mut |effect| Poll). New `native_poll_local` (ordered/cleanup children drive a resident NativeLocalAuthority) and `native_grant_step` (grant-priced child step, owed work recorded once with effect.performed).
- Receivers with ordered children keep `ordered_authority`/`cleanup_authority` (retired through ordered_close_step!/set|map_close_work_quote!).
- Rewrote shared_close_work_quote!/shared_original_close_bridge! for a slot child; removed unused sequence_close_work_quote!.
- NativeRecordMatcher::advance returns Poll (batch fn kept as advance_batch).
- NativeVariantSelector: select_advance/advance_nested/advance_local, settle_step, settlement_work_demand, free begin_close().
- NativeReceiveSlot (🌳️value) relaxed to I:?Sized+'static, re-exported as owned::NativeReceiveSlot; mapped/box/shared/list hold the slot, never T::Receiver.
- Sequence: NativeSequenceCollection/Reservation come from crate native-reservation (new path deps added to native-owned and to native-reservation->schema-engine); NativeSequenceInput stays local (two NativeValueSource trait copies exist: 📖️source crate vs schema-engine::source; reservation's input trait is bound to the other copy).
- Added NativeOwnedStorage::release_work_demand and NativeRecordStorage::release_work_demand (derive emits them); fixed u128 work in the reservation crate's Vec push_step.
- Applied valuefix rules: all effect.reborrow(..expect) replaced by effect.nested(..) / nested_within.

## Decisions / notes
- Nested = control-driven children with their own frame (matcher, sequence storage admission + reservation, builder grant ops via control.receive_allocation, ordered ops via native_poll_local). Local = pure steps, quote == performed.
- 🍃️leaf/🦀️.rs is an unmounted dead copy (old slot, old string/option); not touched, delete by owner (value).
- Known follow-ups: valuefix rule 3/4 (a denied grant should quote/report 0; zero-progress Complete must advance) not audited in my ports; list builder grant ops stay Nested via a frame; tests dirs under owned not compiled.

## Follow-up (coordinator batch)
- Rules 3/4 audit: every post-take "published nothing" and push "kept donor" zero-progress re-entry (value, tuple, array, shared, map, set, list) is now a typed InvariantViolated refusal. A denied grant already reports work 0 at the slot pre-check and via the leading admission test in each advance_local (no performed call); select_advance cannot see the grant, so quotes stay per-phase. Not exhaustively re-audited: selector/matcher internals (schema-engine) and the derive output.
- wasm32-wasip2 check of native-owned: GREEN (W-receivers-111 and, after the audit edits, W-receivers-114 rc 0). Native lib re-check after edits: W-receivers-113 rc 0.
- value-native-owned-try-map ported (slot child, select/nested/local, Poll close): W-receivers-118 rc 0 (lib).
- Test targets: NOT ported. `cargo check -p semio-framework-value-native-owned --tests` (W-receivers-112) fails only in the shared fixture crate value/🧪️testing/📥️admission (uses removed NativeConstructionRecipientIssuer/NativeDecodeWorkIssuer/NativeDecodeConstructionRecipient/NativeDecodeWorkRecipient; needs a rewrite onto OriginalConstructionRecipient + OriginalWorkRecipient/OriginalWorkAuthority). Test closure is also 272 KB > 262,144 (nativetests lane).

## dsl-record-model 📥️owned mapping: BLOCKED on a semantic decision (no code changed)
`OwnedFieldReceiving::step(&mut Option<FieldValue>, native, body, grant)` consumes a MOVED owned source (take_source / source.take(), zero payload copy; ScalarReceiving, TextReceiving, VectorReceiving move Strings/Vecs out). `NativeReceiver<I,T>` reads a BORROWED original (`RetainedCloneRef<'_,I>` = &I + lease; `bind` duplicates a lease; nothing can be taken out of it). A literal one-to-one mapping is impossible without changing meaning. Options (need runtime/coordinator pick):
 A. FieldValue (and RecordValue fields) implement the NativeValueSource trait (schema-engine::source) and every DslOwnedReceivingField type receives through the existing NativeReceive<dyn NativeValueSource> receivers (copy decode like value/decode/owned). Deletes the bespoke module (DslOwnedReceivingField/Record/Variants cursors become NativeReceive); payload becomes copied, not moved; derive for dsl records must emit NativeReceive instead of DslOwnedReceivingField.
 B. Keep a moved source: new source type `FieldCell` (interior `Mutex<Option<FieldValue>>`) as I, receivers take from it inside advance_local under effect (work = performed), bind/binding_work_demand skipped (the cell is the sole owner). Keeps zero-copy; deviates from "original is immutable" and RetainedCloneRef lease semantics (project/bind unusable).
 C. Keep OwnedFieldReceiving as the dsl-private per-field cursor trait but port its internals (grants -> effect, remove receive_retained_work) without NativeReceiver; contradicts "no parallel trait".
Recommendation: A for mechanical files (scalar, text, vector, array, mapping, storage), because the value/decode/owned receivers already exist and are green; verfolgen should not start until runtime picks.
