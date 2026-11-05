# Bounded Named Example Routing

The nine missing named-tool owners require real loading work, not factory registration alone: DOCX, IDML, PPTX, XLSX, PDF, EPW, binary, deflate, ZIP. No production routing changes were made during this review.

## Actual paths

DOCX base uses bounded_native_editing_editor with set-page/set-run-formatting, its command parser and preparation route. ZIP base has a handwritten BoundedNativeEditingEditor with set-node and ArchiveTextWork. Neither accepts setActiveExample. Adding a tool label cannot teach either command or worker to load a source.

The framework catalogue fallback is cold: editor-app example_action_document calls reference_birth/print_dsl for blank sources; reserved-routes catalogue_snapshot_from_document parses the document and encode_pack constructs its pack before dispatch_emit. The No-example UI fix deliberately does not broaden this path.

Existing registered Text is also insufficient. Its semio_text_editor_retained_reduce calls parse_example_snapshot, which performs S::parse_dsl plus a full print_dsl canonical comparison. It then calls load_example_effect, which performs encode_pack and resolves empty_document_spr synchronously. BoundedArtifactCommandWork (retained-command/checkpoint-and-work-contract) declares only depth1, has zero capacity/copy/release demands, reports_progress=false and ignores cx in its single reducer step. A bounded_first_step contract and retained producer request do not finance these nested allocations or make their traversal resumable. This is direct Text evidence, not an unsupported claim about every registered owner.

ExampleSource currently carries Inline(String) or Deferred {produce:fn()->String, source_suffix, source:static bytes}. document_json clones or invokes produce. Deferred descriptor identity is useful but is not a retained runtime decode cursor or typed snapshot binding.

## Reusable infrastructure and missing prerequisites

ArtifactNativeSnapshot decode_native_snapshot/encode_native_snapshot are controlled synchronous APIs. They preserve the caller control when used correctly, but do not provide bounded resumable work by themselves.

OriginalSnapshotRecordReceiving plus OriginalPackWriting are staged Pack infrastructure. Their typed receiver needs DslOwnedReceivingRecord and BorrowedDslRecord. DslRecord alone does not imply that receiver. Text has NativeReceive tree support but no verified owned record receiver binding; DOCX lacks that binding too. Text would need recursive receiving for Snapshot, Run, Mark and MarkKind; DslReceiving on records already emits owned receiving, while DslScalar alone supplies no receiving cursor. DOCX is the retained OPC plus XML snapshot, so a paragraph projection or Text-only derive cannot establish its typed loader. See 📚️typed-receiving-derive-prerequisites.md. There is no verified RecordValue-to-NativeValueSource adapter for arbitrary field/name/tag layouts. See the independent reports 📚️funded-example-codec-seams.md and 📚️pack-to-native-value-route.md.

## Clean production route

Implement one shared retained named-source work owner in the stdio editing layer, with explicit per-editor typed source bindings. It must resolve the exact declared source ID without materializing document_json, retain immutable authored bytes, stage decoding and schema validation through the real original caller/decoder, then stage Pack writing and the empty-history owner before emitting LoadDocument exactly once. Preserve partial typed values, output buffers and owning errors until paid close. Unknown IDs and malformed or mismatched source schemas fault; there is no default snapshot substitution. The actual source parser/writer must expose resumable control before this owner can honestly promise the current one-item interactive stock.

First supply the missing typed staged receiver bindings (or a genuine bounded DSL receiver) and prove them with Text and DOCX. A cold ToValue conversion, new local NativeControl, increased item ceiling, or whole synchronous parse hidden in a reducer is not an acceptable implementation. Canonical Pack assets could only be an alternative after real current typed encoder evidence and complete typed staged receiver support; present assets are not such proof.

## TDD gates before porting nine owners

Create a schema-first neutral corpus with named source identity, exact document schema, initial caller stock, per-turn currency denials, cancellation at each stage, malformed input, wrong schema, duplicate delivery, and explicit expected terminal custody. Validate event ordering and counts independently with SQLite and source ID/schema projection with an existing JSON schema oracle.

Native tests must use one genuine OriginalJobFixtureCaller and persistent decoder, HeapWitness on every advance including errors, and actual retained/work receipts. With all five currencies denied independently, no unquoted heap or synthetic progress is allowed. Resume under the same authority; prove exact source content, one LoadDocument, no replay after accepted publication, and paid retirement of partial source/output/errors on cancellation. Compare final typed content and Pack bytes with actual canonical codec and independent format oracle where applicable. Then bind DOCX and ZIP via the shared factory and retain exact unknown/malformed selector tests. Port the other seven only after this shared path is physically green.

No native execution, descriptor publication or real browser journey is claimed by this research.
