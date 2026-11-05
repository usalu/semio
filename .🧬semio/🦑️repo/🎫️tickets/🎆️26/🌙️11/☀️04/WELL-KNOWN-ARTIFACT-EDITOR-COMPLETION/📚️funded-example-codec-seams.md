# Funded Example Codec Seams

Read-only source inventory; no native execution or product edits.

## Actual contracts

Framework 🔨️modules/🚪️io/📝️text/📸️snapshot/🦀️.rs ArtifactDsl exposes only parse_dsl(&str) and print_dsl()->String. Binary sibling ArtifactPack exposes encode_pack_with/decode_pack_with with Pack options. Neither contract supplies caller controls or resumable turns.

Framework 🔨️modules/🚪️io/📸️snapshot/🫴️native/🦀️.rs ArtifactNativeSnapshot supplies decode_native_snapshot(NativeSnapshotInput::{Text,Binary}, &mut NativeDecodeControl)->Result<Self,ValueError> and encode_native_snapshot(NativeSnapshotEncoding,&mut NativeEncodeControl)->Result<IoPayload,ValueError>. These are genuine controlled synchronous methods, not independently bounded resumable action cursors.

Store 📦️codec/📸️native-snapshot/🦀️.rs decode_native_snapshot_record/encode_native_snapshot_record retain supplied controls, including schema allocation and envelope parsing/writing. Calling them wholesale from one retained action does not establish bounded per-turn work.

## Existing staged Pack receiver

Store 📦️codec/📸️native-snapshot/🫴️receiving/🦀️.rs defines OriginalSnapshotRecordReceiving<T: DslOwnedReceivingRecord + BorrowedDslRecord>.

- admit(source:&[u8], envelope:&'static str, PackLimits, &NativeDecodeControl, grant) returns optional cursor plus actual admission progress.
- It binds source pointer+length and decoder ledger_identity()+retirement_recipient_identity(); later changes refuse.
- demands(source,body), step(source,&mut native,body,grant), is_ready, take_demand, take_into permit separate paid parsing, typed binding, and output transfer.
- Its inherent step returns RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>. Child-posted outcomes remain posted; own effects are posted exactly once through receive_retained_work.
- close_demands and close_step(&mut dyn NativeDecodeCloseControl,grant) preserve partial output, typed binding, generic record, and physical parser until terminal_is_empty.
- The erased OriginalArtifactSnapshotReceiving trait flattens step to Result<progress,ValueError>; prefer the richer inherent work port where truthful work must survive composition.

This cursor consumes Pack, not textual DSL. No analogous staged authored-text receiver was established in this bounded audit.

## Existing staged Pack writer

Adjacent writing.rs OriginalPackWriting owns BorrowedPackDocumentCursor plus its sole final Vec. new(envelope,maximum) initializes only its header; caller must separately admit that header. demand(body), advance<T:FieldProjectionSource>(&T,BorrowedRecordSpec,&mut NativeEncodeControl,grant), ready, take(grant), close_demand, close, empty form the public staged interface. Output transfer occurs only after codec scratch closes. Errors must retain the writer and every emitted prefix for paid closure.

FieldProjectionSource is defined at framework DSL 🧬️schema/🛫️encode/🦀️.rs:11; a DslField blanket implementation exists at :20. The writer must borrow one immutable retained typed source across turns, not rerun cold projection or replace the source while its cursor is active.

## Typed receiving and current owner gaps

Value 🛬️decode/🧳️owned/🦀️.rs NativeReceive<I> provides an allocation-free receiver header. NativeReceiver<I,T> exposes demands(RetainedCloneRef<I>,body), advance(original,&mut NativeDecodeControl,body,grant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>, output_ready, take_demand, take, begin_close, close_demands, close_step, terminal_is_empty. NativeReceivingBatch preserves child work and progress across refusal.

Semio Text Snapshot and its Run/Mark types derive value::NativeReceive and DslRecord (Text snapshot source :28/:43/:59/:73). This is not proof of DslOwnedReceivingRecord: DslOwnedReceiving and DslReceiving are separate proc-macro entry points in DSL schema/✨️derive/🦀️.rs :22–26. No direct matching owned-receiving declaration was found in this owner. DOCX base snapshot has neither NativeReceive nor DslReceiving/OwnedReceiving declarations in the inspected owner tree. Therefore a shared Text+DOCX Pack receiver cannot currently be claimed from existing bounds.

## Viable worker composition

A genuine source-load worker must retain the original caller/source binding and immutable input, admit each cursor header under the actual grant, drive exact demands using the existing StepContext receiving/encoding loans, retain RetainedWorkOutcome work and physical progress on every error, and transfer the result only after separate funded take. Parsing, final Pack writing, publication, and closure need distinct frontiers; cancellation changes to paid closure without replacing the original ledger/recipient. Existing OriginalCallerSourceLoan and OriginalNativeDecodeSource provide real source/decoder custody; they are not byte ownership by themselves. The retained source buffer must remain owned until all cursor aliases close.

Reusable staged Pack machinery is real. Missing typed bindings and staged DSL ingestion are real prerequisites; a cold parse/encode hidden inside a reducer remains unfunded.
