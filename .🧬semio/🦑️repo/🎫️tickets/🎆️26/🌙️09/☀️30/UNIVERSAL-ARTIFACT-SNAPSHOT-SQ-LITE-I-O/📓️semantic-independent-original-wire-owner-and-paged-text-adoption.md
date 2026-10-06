# Original 8194/4096 Owner Identity and Paged Text Adoption

Read-only inspection of original law, current first-party shapes and held decoder/PagedText. No production mutation or gates. No exemption, activation or scope shrink inferred.

## Exact Original Owner

[Original full loop](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs:1228) reads neutral wire string, to_vec, pads that one Vec, stores ops:vec![wire] in ChildEmit, plus schema/labels/slot/child_id. After zero-item retain assertion, it calls child.close_one(1,TYPED_OPERATION_RESULT_PAGE_BYTES) for wire_bytes+128 iterations, rejects Blocked/AwaitingInput, requires Complete, released bytes>=wire_bytes and all nested fields empty. Current complete assertion is1258 (earlier authority1257 shifted by one).

That literal law owns encoded/rejected ChildEmit wire and wrappers. It neither calls PDF decode_op_span nor directly holds a returned PdfMutation.text. Direct paged PDF encoder proof cannot solve original literal Vec ChildEmit construction by itself. ChildEmit.ops and its exact callers must adopt retained first-party paged source ownership without Vec adapter. The original completion grant and literal8194 payload remain unchanged.

## Three Different Owner Edges

1. Encoded input/source: OwnedOperationBytes owns physical pages; ByteSpan borrows immutable octets. Decoder may inspect/canonical-compare but cannot copy frame or retire borrowed source.
2. Rejected partial/scaffold decoder candidate: explicit NativeDecodeRetirementRecipient receives actual DirectReadRetirement or SnapshotPatchReadCursor.into_retirement. Every success/refusal after creation transfers those actual owners to caller drain.
3. Successful returned semantic PdfMutation/PdfSnapshot/PageDoc: owns its actual typed fields independently of source pages and empty scaffold recipient. No source-page terminal receipt proves those fields retire.

[Held decoder law](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/pdf14-original-paged-source/decoder-law.rs:20) checks returned decoded.encode_op bytes but its close helper only closes OwnedOperationBytes source. It does not explicitly retire successful returned typed mutation; scope ends ordinarily. Therefore this held law currently provides no1/4096 successful semantic-text retirement evidence. The broader ownership objective still requires genuine final typed-owner/caller evidence rather than assuming an exemption.

[Held decoder](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/pdf14-original-paged-source/decoder-provider.rs) creates String-backed candidate fields with try_reserve_exact(length), then incrementally fills them. DirectReadRetirement::release_text only frees entire actual String.capacity if it fits grant. Thus canceled/refused8194 allocation remains retained forever at4096. Returning it explicitly solves ownership transfer, not bounded physical release. Raising grant/refunding/prefixDrop cannot qualify it.

## Concrete PagedText Adoption Closure

The [candidate primitive](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/native-paged-text-owner/provider.rs) supplies separate paid PagedList byte/metadata allocations, full validation and borrowed bytes/chars, retaining partial pages on cancellation. It has no flatten method. Primitive laws do not by themselves adopt any PDF field or public caller.

Necessary field roots:
- [PageDoc.text and PdfSnapshot.schema](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:50), presently String. PageDoc derives Clone/Debug/ToValue/FromValue/DslRecord; PdfSnapshot additionally ArtifactSchema/PartialEq. Defaults/new initialize Strings.
- [ReplacePageText.text](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/♻️replace-page-text/🦀️.rs:15), presently String. Its diff and inverse clone field ownership.
- [PdfPageDiff.text](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:36), Option<String>; apply/merge/diff carry same semantic ownership. InsertPage and SetSnapshot embed PageDoc/PdfSnapshot; PatchSnapshot carries intrinsic patch text owners through real parser.

Candidate PagedText lacks required Clone/Debug/PartialEq/Default, ToValue/FromValue, DslField and field projection hooks. A mere field type replacement therefore cannot compile or preserve current public shapes. Each needed controlled construction/clone/read capability must be first-party and account exact pages; retain actual owners on failure rather than implicitly dropping them.

Current [DslValue](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:141) String(String) and [FieldValue](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:405) Text(String) are contiguous owner representations. Existing derive-controlled String readers copy_text into String, and ordinary value conversion/as_str expects contiguous text. No PagedText→String/Vec adapter may masquerade as controlled ownership; provide first-party borrowed text projection/parser capability and appropriate retained semantic representation or explicit owner-specific codecs rather than flattening.

Concrete codec consumers:
- [ordinary mutation binary helper](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:31) put_text(&str) and Reader.text()->String currently flatten/copy; update genuine field encode/decode to stream bytes and own pages while preserving original tags/u64length/UTF8octets.
- [direct borrowed encoder](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/pdf14-original-paged-source/provider.rs:10) text(&str) uses len/as_bytes; consume complete TextReadSpan byte_len/bytes directly under original control.
- Held direct borrowed decoder text(&mut String) must construct real PagedText in actual candidate, transfer every partial owner to recipient, and canonical-compare original ByteSpan via streamed encoder.
- [actual PDF IO](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🦀️.rs:329) shown_text()->String and literal_string(&page.text), is_empty and encode_pdf must consume/produce genuine semantic paged text without changing shown-text interpretation, Unicode bytes or PDF1.4 page geometry.
- Snapshot native DSL/binary hooks, mutation text codecs, diff/inverse, editor/page consumers and typed app initialization need the same real field capability. Editor1.4 aliases currently join1.7 types (retained prior audit); that existing mismatch must not be hidden behind a paged adapter.

Seven-frame family remains InsertPage,RemovePage,MovePage,ResizePage,ReplacePageText,SetSnapshot,PatchSnapshot. Text-bearing direct cases0/4/5 need actual paged fields; patch6 needs real typed patch intrinsic text owner and parser retirement, not DirectReadRetirement substitute. Scalar cases1/2/3 retain ordinary literal semantics. Preserve exact file/segment/item/depth/canonical limits and both caller controls.

No fixed8194 or guessed8192 capacity may become general field policy merely from one specimen. Paged capacity/metadata must be admitted through actual caller/declared ownership bounds. Retain empty complete text semantics, Unicode scalar validation and exact physical allocation layout. Final successful typed owner retirement and partial refusal retirement each need actual caller laws at unchanged1/4096; encoded-source retirement is a third distinct law.

## Roster Correction

Updated existing [60-owner roster](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📓️universal-concrete-owner-completion-roster-current.md): normal hooks now mounted and actual selected Flow10/10,Run18/18,frameworkDAG10/10 qualification supersedes stale absent-hook text. Other unknown publication contracts, Collection leaky qualifier and intentional explicit SpaceHistory boundary remain separate.

