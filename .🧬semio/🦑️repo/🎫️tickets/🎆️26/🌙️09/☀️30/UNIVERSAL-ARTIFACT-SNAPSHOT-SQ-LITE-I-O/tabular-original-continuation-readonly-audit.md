# Tabular Original Continuation Read-Only Audit

2026-10-10. Source-only audit. No builds/tests/source changes; this report is authored evidence, not runtime qualification.

## Continuation Placement

CSV `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:212` and TSV corresponding file:233 define `original_native_caller`. Current supplied native observer is borrowed by `new_forwarded`; operation owns only its reborrow. Exact ValueError/IoError retained receipt is compared with original owner progress before `drop(owner)`. An explicit `FnOnce(&mut actual NativeDecodeControl/NativeEncodeControl)` hook immediately after owner drop is a suitable seam: recipient custody is installed on that same native control, and the original result remains available for return. Do not take/destroy result ownership inside hook.

Use caller `Cell<bool>` for observer state so both original observer closure and continuation can share it without exclusive borrow conflicts. Ordinary hooks should be explicit no-ops. A hook signature taking only native control suffices because close grant and armed Cell can be captured by the caller. Native still borrows recipient and allocation closure; direct recipient inspection must wait until native drops, while `native.has_retirement_owner()` is available throughout.

## Actual Record And Envelope Route

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs::decode_sqlite_snapshot_record_native` accepts actual IoPayload, envelope id, RecordSpecProducer, construct closure `(record, &mut Option<T>, native, body)`, SQL control and original NativeSnapshotDecodeOwner. It calls `owner.receive::<(Option<RecordSpec>,Option<RecordValue>,Option<T>),T>` and fills each real intermediate slot before proceeding. It obtains schema using `spec.decode(native)`.

Binary payload path is `semio_format::unwrap_binary_controlled(bytes,envelope_id,Component::Pack,1,native)` followed by `pack::record::decode_document_controlled`. Text path is `split_text_preamble_controlled(text,envelope_id,Component::Dsl,1,native)` followed by `semio_framework_dsl_record::parse_exact_controlled` with Document SourceMode and diagnostic max_bytes equal SQL max_file_bytes. Payload must be genuine enveloped Record text/pack; do not use raw CSV/TSV external text as input to this helper. Existing `csv_native_payload` at CSV test:66 uses actual ArtifactPack/ArtifactDsl and is appropriate.

CSV production native decoder uses `CsvSnapshot::__dsl_spec_producer()`, envelope `stdio.csv`, `admission::admit(record,native,limits)` then `admission::bind(record, snapshot_output,native,body)`. A focused integration law can call the actual generic Record decoder with identical admission/bind and a Cell arm placed after admit, immediately before bind. This separates genuine typed binder cancellation from unrelated spec/parser copies that can have the same copy length. A test that only cancels by `event.total==long_text.len()` may otherwise refuse during physical parsing and never create the typed output.

## Limits And Observable Refusal Ordering

CSV original caller fixture `🧫️fixtures/🛬️native-control/🎟️original.json`: native maximum16MiB; body items65536/copy1MiB/capacity16MiB/release16MiB/depth4096; close items4096/copy1MiB/capacity16MiB/release16MiB/depth4096; all-zero denied close;65536turns. Typed binder admission fixture alone has native maximum262144 and a140000-byte UTF8 field. Full Record parser plus typed copy, spec and return frame can exceed that262144 cumulative ceiling, so use the original caller fixture's real allowance for full integration. No exact full-producer allocation demand was measured.

Both `🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs::close_retirement_recipient` and corresponding encode implementation first return Complete for empty recipient, then Progress(0) for zero maximum_items, then check depth/copy/capacity/release demands, then original observer checkpoint, then charge capacity and execute close_step. Therefore zero denied close succeeds even while observer remains canceled. To prove cancellation refusal, call with funded original close grant and assert Canceled, unchanged cumulative owned_bytes, pending has_owner and zero physical birth/release. Ensure grant is generous enough to reach checkpoint rather than fail earlier with DepthLimit/OwnershipLimit.

Only after that refusal should continuation set the same armed Cell false and let ordinary funded close loop run. Observer composition and receive scope should have unwound by then; this hook must not construct another native control or swap observer. SnapshotOwner progress stays a producer receipt, while close-step progress is a separate funded receipt; do not compare them as one scalar wallet. Native owned_bytes is cumulative accepted allocation and does not refund releases.

## Physical And Lifetime Pitfalls

The recipient holds the triple's parsed spec, parsed Record and typed partial output. Direct Option binder law sees only typed allocations; full decoder law sees parser/spec/wrapper allocations too. Its physical birth total must be compared with complete owning-frame/close accounting rather than assuming typed body receipt equals all decoder birth. Inspect the typed Option inside construct closure before returning the error, where it remains borrowable; after receive refuses it belongs to erased recipient and is not directly accessible through public control.

Arm only after borrowed admit; cancel after an authentic copied UTF8 prefix and retain that same output. A Cell witness can record prefix length/quoted state and assert a Serde literal-prefix oracle while still inside construct. Native parent-stage restoration should be tested through a scoped parent stage; avoid unconditional final advance while observer remains canceled. Rearm before a parent progress checkpoint if testing restoration after refusal.

Any successful returned snapshot lives outside recipient and needs its separately funded ControlledRetirement cleanup; hook-based recipient drain cannot retire success output. Error itself remains returnable after close because custody moved into recipient, not into ValueError.
