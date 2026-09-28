# OPC Package Fidelity Design

The office container currently preserves content-part payloads and typed metadata fields, but it discards ZIP headers, cross-category member order, and unrecognized metadata XML. Root inspected 156 `.content_types` references across 36 Rust files and 152 `.relationships` references across 32 Rust files. Most representation coupling is in snapshot validation, handcrafted diff/mutation codecs, strict/transitional composition, and DOCX retained preparation.

## Independent Archive State

Retain archive state separately from semantic XML tables: `entryMetadata` is an ordered map from every physical member path, including `[Content_Types].xml` and relationship members, to the existing ZIP metadata type; `memberOrder` is an explicit sequence of all physical member paths; `commentUtf8` accompanies the existing comment. These are disjoint authored facts, not copies of semantic fields. A new path gets the ZIP format default; deleted members remove their metadata and order identity in the same mutation. Rename carries metadata with the member. Explicit order must be a complete unique permutation before export.

Implementation must update all office schemas and value codecs, text/binary full-snapshot replay, diff/inverse/absorption, Details projection, retained post-copy and retirement together. Do not land Rust struct fields before the coordinated source checkpoint. The existing comment-only replay repair proves why omitted ancillary state is a persistence bug.

## Metadata XML Authority

A second whole XML document beside mutable typed tables creates competing persisted authorities. The long-term shape should retain one complete XML document per metadata member and expose typed content-type/relationship projections and edit operations over that document. Unknown attributes, namespace declarations, comments, processing instructions, extension children, explicit empty relationship members, and interleaving order must remain represented and editable. Typed helpers must modify only their addressed known attributes/children. An explicit user deletion can remove extension content; an unrelated office edit must not.

This requires replacing current tuple-only content type collections and relationship vectors in the persisted package, then changing the 3 family diff/codecs and preparation as one cut. No raw-versus-typed compatibility adapter or hidden fallback should remain. Header retention alone is not evidence that this metadata requirement is satisfied.

## Acceptance Cases

- ZIP fixture with content before metadata, distinct local/central timestamps and versions, stored/deflated members, entry comments/attributes/unknown extra records, Unicode path/comment markers, and CP437 archive comment. Edit document text, save/reopen, and independently inspect each ZIP member using the existing `zip` test dependency.
- Metadata XML fixture with prefixed namespace names, extension root and row attributes, comments/processing instructions before and between known entries, mixed Default/Override order, and explicit empty relationship document. An unrelated document edit preserves all of them; one metadata edit changes only its addressed field. An independent XML parser must confirm the same logical output.
- Text/binary mutation and diff replay preserve all retained fields; inverse restores them exactly, including explicit empty values and archive encoding.
- Cancellation at every preparation/retirement phase returns owned data and allocation credits. Large metadata maps use deterministic incremental iteration.

## Execution State

This is a source-backed design, not a completed implementation or passing test. Archive fields remain held while the generic bounded typed-copy foundation and current native checkpoints settle. Root owns the subsequent OPC codec integration; execution workers own the corresponding schema and retained-copy updates.

## Authored Metadata Publication Guard

Root found that decode rejected duplicate metadata identities while encode accepted equivalent invalid edited snapshots, producing archives its own decoder would reject. Added shared identity validation for case-insensitive Default extensions, exact Override paths, and relationship IDs within each owner. Both directions call these validators. Export also refuses a content part whose cached `contentType` disagrees with the content-type table, preventing silent loss of that edit on reopen. The canonical XML redesign above remains required to remove this duplicate authority.

A new neutral `publication.invalidMetadata` fixture and native `publication_rejects_ambiguous_authored_metadata_before_writing_an_archive` law cover four authored invalid cases, source immutability, and a valid independent ZIP/save-reopen control. Test was authored before the guards; the existing ZIP native run remains queued, so no assertion-red or native-green claim is made. Rustfmt and scoped diff checks passed.
