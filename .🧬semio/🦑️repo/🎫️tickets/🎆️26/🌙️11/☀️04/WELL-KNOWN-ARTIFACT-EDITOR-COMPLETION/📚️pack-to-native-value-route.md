# Pack To Native Value Route

Read-only; no native build or execution.

Actual Pack/🌱️value/🛬️decode/🫴️receiving/🦀️.rs RetainedRecordReceiving provides admit_document(source,range,BorrowedRecordSpec,PackLimits,grant), demands(source), step(source,&mut NativeDecodeControl,grant), is_ready(), take_into(&mut Option<RecordValue>,grant), close_demands(body), close_step(grant), terminal_is_empty(). Admission/step/take return RetainedWorkOutcome; exact OriginalWireSourceIdentity remains bound.

Its take_value_into(field,&mut Option<DslValue>,grant), line151, is a genuine zero-clone dynamic payload transfer. It requires exactly one record field, and that field must already be FieldValue::Value. Existing native test original_dynamic_value_moves_without_cloning_or_abandoning_record_backing verifies pointer identity, denied transfer retention, and paid parser/output closure. This is not a generic typed RecordValue conversion.

Current authored Text Pack has declared schema/runs record fields, not a single Value field. Therefore that take_value_into API cannot turn it into the DslValue input expected by its existing NativeReceive implementation. RetainedRecordBodyCursor and RetainedValueCursor are lower token/byte drivers; they do not by themselves supply a complete retained DslValue tree for arbitrary authored record schemas.

Value/🛬️decode/🧳️owned/🌳️value/🦀️.rs:34 implements NativeValueSource for DslValue. The NativeReceiver input is RetainedCloneRef<I>; advance returns RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>. NativeReceiveSlot can retain staged receiving storage/output/retirement, but input ownership and binding must outlive every projected child reference. No existing NativeValueSource implementation for RecordValue or FieldValue was found in the inspected DSL/value source.

Two viable designs require real implementation:
- Add DslOwnedReceiving/DslReceiving to the actual typed schema graph and use OriginalSnapshotRecordReceiving, preserving moving field custody and avoiding a duplicate DslValue object tree.
- Add a correctly schema-aware borrowed RecordValue→NativeValueSource adapter, then feed existing typed NativeReceive. This must map numeric field IDs to authored names and tagged enum/record representations; a cold ToValue conversion is not a substitute. Keeping generic record plus constructing typed output retains both trees, increasing allocation/copy/retirement compared with moving owned DSL field binding.

The existing single-dynamic-value Pack route is immediately reusable only when the committed format genuinely declares that representation. Re-encoding Text examples into a new schema-less bridge format would change the source contract and is not an existing codec reuse.

Text has NativeReceive derives; its owned DSL receiving declarations remain absent in the inspected owner. DOCX native receiving support remains an independent missing schema capability, not solved by the dynamic-value cursor.
