# Typed Receiving Derive Prerequisites

Read-only actual generator review; no Rust compilation or edit.

## Text graph

Adding a receiving derive only to SemioTextSnapshot is insufficient. DSL schema/✨️derive/📥️owned/🦀️.rs fields_cursor generates OwnedReceivingSlot<field_ty> for each field. OwnedReceivingSlot<T> requires T:DslOwnedReceivingField. The Vec<T> implementation in schema/🫴️receiving/📥️owned/📋️vector/🦀️.rs:5 requires its element to implement the same trait.

The actual dependency chain is Snapshot(schema:String,runs:Vec<SemioTextRun>) → Run(language:String,content:String,marks:Vec<SemioTextMark>) → Mark(kind:SemioTextMarkKind,href:String). Existing value::NativeReceive derives do not satisfy these DSL owned-receiving bounds.

Use DslReceiving on all four Text types for the synchronous plus staged owned contracts. Its record branch appends emit_owned_receiving at receiving-and-projection/🦀️.rs:46. Do not also add DslOwnedReceiving on those records: that duplicates generated owned implementations. An owned-only record alternative is DslOwnedReceiving on Snapshot/Run/Mark, with DslReceiving on the unit enum MarkKind for its dedicated scalar cursor.

MarkKind currently has DslScalar. Actual emit_scalar in 🚦️emitters/🦀️.rs:95–146 emits DslField and borrowed scalar metadata only; it does not emit receiving. The scalar DslOwnedReceivingField implementation at receiving-and-projection:31 belongs to the DslReceiving enum branch. Existing DslScalar is not enough.

String and generic Vec containers already have owned receiving support. BorrowedDslRecord and schema metadata remain separate requirements of OriginalSnapshotRecordReceiving; retain current DslRecord derives.

## DOCX difference

DOCX base snapshot is not a small Text-like record graph. Actual DocxSnapshot at schema/📸️snapshot/🦀️.rs:254 has schema:String, opc:RetainedOpcPackage, xml_parts:DocxXmlParts. The authoritative XML part contains RetainedXmlDocument. Semantic DocxDocument/Paragraph/Run are projections, not the authoritative root stored by this snapshot.

Its binary snapshot ArtifactPack implementation at io/💾️binary/📸️snapshot/🦀️.rs:18 delegates to native::encode_standalone/decode_binary with SqliteSnapshotControl and authored pack limits. It is not a DslRecord-derived root codec. ToValue/FromValue derives on the snapshot do not supply owned DSL receiving or native original construction.

Consequently adding Text's four derives does not establish a DOCX loader. DOCX needs actual receiver support for retained OPC/XML owners and its current wire layout, or a different genuine bounded domain-native import path. Deriving over projected paragraphs would omit authoritative raw XML/package custody.
