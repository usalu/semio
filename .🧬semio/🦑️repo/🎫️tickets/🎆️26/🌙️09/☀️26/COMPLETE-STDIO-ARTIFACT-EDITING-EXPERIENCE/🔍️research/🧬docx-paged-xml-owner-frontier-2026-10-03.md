# DOCX Paged XML Owner Frontier — 2026-10-03

## Audit boundary before the retained XML cut

`DocxSnapshot.opc` is retained and DOCX preparation copies it through a bounded cursor. `DocxSnapshot.xml_parts` remains `Vec<DocxXmlPart>`, where each part owns `String` path/content type and a recursive `XmlDocument`. Preparation phase 2 still clones `schema` and the complete `xml_parts` vector in one 4 KiB-accounted turn. `measure_snapshot` caps the graph before that turn, but the measured bytes are not the bytes reported by the turn. `DocxSnapshotRetirement` likewise retires the schema/XML remainder as one 4 KiB item after the retained OPC owner. This is a bounded refusal envelope, not scalable XML ownership.

The shared XML model recursively owns `Vec<XmlNode>`, `Vec<XmlAttr>`, and `String`. Its SQLite retirement avoids recursive stack destruction by allocating an ordinary pending `Vec`, but that cold frontier has neither Store grant accounting nor a surviving cursor. Existing `XmlDocumentView` borrows recursive nodes, so changing only DOCX preparation cannot remove the recursive owner or its whole materialization.

## Canonical owner cut

The next clean authority is a shared flat retained XML document, used directly by DOCX rather than a DOCX-only wrapper around `XmlDocument`:

- `RetainedXmlText` uses `PagedUtf8`; every UTF-8 chunk is at most 1 KiB and every directory/payload page is separately admitted.
- `RetainedXmlNodes`, attributes, child references, prolog references, epilog references, declarations, doctype declarations, and entities use `PagedList` with declared schema maxima.
- Each node is one tagged record. An element stores its name plus ordinal ranges into the attribute and child-reference owners. Leaf variants store paged text fields. Child references are stable node ordinals.
- The document stores optional root/declaration/doctype ordinals and ordered prolog/epilog reference ranges. The flat graph preserves all current XML state, including declaration quote, DTD position/external identifier/entities, CDATA, comments, and processing instructions.
- Construction validates every ordinal/range, unique root ownership, one-parent tree membership, boundary order, and the absence of cycles before publication. Validation uses paged bit owners or a caller-admitted frontier; it does not allocate `HashSet`/`BTreeMap` scratch.

This representation mirrors the existing thirteen-table XML vocabulary and removes recursive drop. It also gives addressed editing a stable traversal target: a `DocxXmlAddress.node_path` resolves child ordinals from the root without materializing an `XmlNode`. Insert/remove can build a new paged child-reference owner under an edit cursor; text/attribute changes replace only the addressed paged text owner. Root owns the address/importer implementation, so this lane does not alter those files.

## Required borrowed traversal boundary

The retained owner must expose one first-party event/visitor cursor rather than materialize a conventional `XmlDocument` for each consumer. The cursor emits declaration/doctype/boundary/node/attribute/child events in authored order and retains its ordinal stack in a `PagedList`. Native XML, SQLite projection, OPC XML encoding, subset checks, and diff use this cursor directly. Each turn reports visited entities, copied bytes, new retained capacity, and depth; cancellation retains the frontier until bounded close reaches terminal emptiness.

Conventional `XmlDocument` conversion is a cold consuming import boundary only until parsers construct the retained graph directly. Export materialization, when a third-party test requires it, must take a cumulative `NativeEncodeControl` and remain explicitly outside Store preparation. No Store mutation/save path may silently materialize the whole recursive document.

## DOCX production phases

1. Change `DocxXmlPart` fields to paged text plus `RetainedXmlDocument`, and change `DocxSnapshot.xml_parts` to `PagedList<DocxXmlPart, ...>`. Keep part order authoritative.
2. Add retained clone and retirement cursors for the shared flat XML owner and the DOCX part list. Generic reserved owner adoption moves completed child owners with one item and zero copied payload bytes.
3. Extend `DocxPreparation` with an XML-parts cursor. Phase 0 copies OPC, phase 1 drains its spent cursor, phase 2 copies XML pages, phase 3 drains that spent cursor, and the domain edit cursor then mutates the exclusive retained post owner. Remove `base.xml_parts.clone()` and the 4 KiB remainder drop.
4. Make `DocxSnapshotRetirement` retire schema, OPC, XML part directory, and each flat XML page through explicit cursors. Every successful close turn reports actual released items/bytes; an indivisible page larger than the complete grant returns deterministic `WorkLimit`.
5. Point DOCX save/native/SQLite/diff/subset traversal at the borrowed retained XML cursor. Direct import construction and retained streaming OPC/ZIP emission follow; until then these boundaries remain named materialization limits.

## Acceptance laws

The neutral fixture must contain more than 1 MiB of XML state across multiple parts, one text node larger than a Store turn, Unicode crossing chunk boundaries, empty text/attributes, comments, CDATA, processing instructions, declaration quote state, DTD entities, and a deep flat tree. Its schema declares exact node/text/part counts and maximum page sizes.

The language-neutral law validates the fixture and structural-clones the retained JSON projection. A third-party XML library parses the emitted XML and independently compares node kinds, order, literal text, attributes, declaration, and DTD state. Native laws require:

- clone completion under one-item/64-byte copy turns without successful zero-progress loops;
- cancellation after an interior text chunk and bounded terminal retirement;
- addressed text/attribute/insert/remove edits that preserve untouched page identities;
- save/reopen equivalence through a third-party DOCX/ZIP/XML oracle;
- registered Store cancellation, stale replacement, error, publication, undo/redo, save, reopen, and retirement;
- deterministic typed refusal when a physical page demand exceeds the caller's complete grant.

## Sequencing constraint at the audit checkpoint

The shared explicit `OpcRelationshipOwners` cutover is currently in flight under `UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O`. DOCX retained OPC and Store runtime proof resumes when that conventional package boundary compiles. The XML owner cut must follow the retained OPC proof and coordinate its address/importer changes with Root; implementing a second DOCX-only XML tree during that transition would create another temporary authority and is rejected.


## Implemented foundation checkpoint before the production mount

The shared additive `RetainedXmlDocument` now exists beside the canonical XML snapshot. It uses paged text and flat paged owners for nodes, attributes, boundary nodes, DTD declarations, and entities; validates ordinals and ownership; materializes only under `NativeEncodeControl`; and derives the generic retained clone and explicit retirement cursors. The fixture preserves declaration quote, doctype/entity, comments, CDATA, processing instructions, attributes, text, prolog, and epilog.

`🗑️generated/retained-xml-native-2.log` records the fresh uncached acceptance law: Nextest run `71d5f26a-67c3-417e-be9d-a0bbac0e12b9`, 1/1 passed with 95 skipped. The independent oracle is `quick_xml`; the native half exercises the one-item/64-byte copy grant, interrupted close, controlled materialization, and final retirement.

At this checkpoint the production cut was still pending: `DocxXmlPart` owned a recursive `XmlDocument`, `DocxSnapshot.xml_parts` remained a `Vec`, and the preparation/save/native/SQLite paths had not adopted the flat owner. The production authority checkpoint below supersedes those first two conditions. The cold conversion still uses an ordinary iterative `Vec<NodeFrame>` frontier; it avoids recursive construction but is not a resumable admitted import cursor.

## Production authority checkpoint

The production cut is now mounted. `DocxSnapshot.xml_parts` and `DocxArtifact.xml_parts` are paged owners, and every `DocxXmlPart.document` is the shared `RetainedXmlDocument`. The DOCX preparation cursor incrementally copies and explicitly retires the retained XML part directory and every retained XML field. It no longer clones or releases the XML remainder as one fixed-accounting turn.

The same authority is used by DOCX import/export, native and SQLite projection/reconstruction, subset validation, strict and transitional conformance, base diff, canonical XML-address resolution, addressed mutations, and demo construction. Address resolution materializes the selected part once and returns an owned resolved node, while mutation application materializes and rebuilds only the affected part. `PagedList` now supplies paged borrowed iteration, mutable iteration, indexing, removal, retention, reversal, and unstable sorting needed by those consumers without converting the part directory to `Vec`.

Fresh source command:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/docx-retained-check-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/stdio-docx-rs:check --excludeTaskDependencies --skip-nx-cache
```

`🗑️generated/docx-retained-xml-check-3.log` records a fresh uncached success after the production migration. Cargo finished the DOCX check in 4.14 seconds and Nx completed the task in 12.8 seconds.

The registered Store lifecycle fixture now carries both a 1.25 MiB retained binary OPC owner and a 1.0625 MiB retained XML text owner, each larger than `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES`. Its law enters through the registered editor owner catalog, checks cancelled publication close, publishes an addressed edit, saves both owners, validates both ZIP entries with third-party `zip::ZipArchive`, reopens the retained snapshot, explicitly retires the reopened owner catalog, and closes the live Store.

The first focused native build, `🗑️generated/docx-retained-opc-xml-store-lifecycle-1.log`, found test-only `ValueError` conversion and owned-node move fallout after the typed XML-address cut. Those DOCX consumers are repaired. The second focused build, `🗑️generated/docx-retained-opc-xml-store-lifecycle-2.log`, stopped on a concurrent mesh compile error. The third run reached runtime and found that `DocxPreparation::close_step` propagated a retained child cursor's `Complete` before checking and draining the other live fields. The close route now validates that child's terminal witness and falls through to the next owner; outer `Complete` is returned only when the complete preparation is terminal-empty.

`🗑️generated/docx-retained-opc-xml-store-lifecycle-7.log` is the fresh uncached clean receipt after that fix and the concurrent `ArtifactCommandWork::step` context migration. Nextest run `7324ae71-af40-44d3-bde6-ad7f83c692e2` passed 1/1 with 157 skipped in 4.818 seconds. The language-neutral fixture caps each lifecycle stage at 16,384 turns; the law fails if copy, cancellation close, publication, acknowledgement close, reopened-owner retirement, or final Store disposal does not terminate inside that bound. Nx explicitly reports the cache was skipped.

The exact focused command is:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/docx-retained-check-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/stdio-docx-rs:test --excludeTaskDependencies --skip-nx-cache -- --features component-app-assembly --lib retained_opc_and_xml_store_route_cancels_publishes_saves_and_retires_large_owners -- --nocapture
```

### Remaining scalable-path limits

1. The DOCX parser still builds a conventional `XmlDocument` before controlled conversion into the retained graph. Direct resumable retained parsing remains required for bounded import construction.
2. DOCX save, native, SQLite, subset checks, diff, and addressed mutation materialize one whole conventional XML part under explicit ownership controls. They no longer materialize the whole XML-part directory, but they are not borrowed retained traversal or streaming encode paths.
3. Addressed mutation rebuilds the complete affected retained document. Stable direct retained-node editing with untouched-page identity preservation remains required.
4. The retained XML cold constructor uses an ordinary iterative `Vec<NodeFrame>` frontier. It avoids recursive construction and recursive drop but is not a resumable admitted cursor.
5. OPC and XML package save still materialize conventional OPC/ZIP output. Controlled refusal, progress, and cancellation do not make that boundary retained streaming.


## Full production validation checkpoint

The earlier sixteen DOCX failures are cleared without widening limits or weakening history assertions. Schema UI metadata now has truthful English and German labels, the set-snapshot neutral fixture uses the canonical tuple representation, mutation history includes its required seed event, and the retained XML SQLite paths settle their exact controlled allocations. The deep flat fixture is constructed child-first, matching retained node ordinal ownership.

Native encode cancellation exposed one remaining recursive error path: `write` materialized a conventional `XmlDocument`, then returned from `writer.document` before the explicit retirement call. The temporary document now lives in an owner whose `Drop` calls iterative `retire_xml_document`; it covers success, refusal, and cancellation. `DocxSnapshot` derives typed retirement and `retire_sqlite_snapshot` drains that cursor with a256-item grant and at least the cursor's next physical byte demand, avoiding both recursive drop and an under-granted zero-progress loop.

Focused command:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/docx-retained-check-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/stdio-docx-rs:test --excludeTaskDependencies --skip-nx-cache --args="--features component-app-assembly --lib sqlite_snapshot_docx_deep_erased_native_input_output_are_interior_cancellable -- --exact --nocapture"
```

`🗑️generated/docx-retained-deep-green-2.log` records Nextest `e36f461f-056e-4acc-828f-736269021656`:1/1 passed,157 filtered,5.359s runtime.

Full command:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/docx-retained-check-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/stdio-docx-rs:test --excludeTaskDependencies --skip-nx-cache --args="--features component-app-assembly --no-fail-fast --lib -- --nocapture"
```

`🗑️generated/docx-native-current-21-retained-xml-green.log` records Nextest `afb8ba3b-add8-4f1f-92a8-dc32b9be149b`:158/158 passed, zero skipped,14.071s runtime. Nx reports26.7s and confirms cache skipping.

The passing suite establishes the current mounted DOCX retained authority, not a fully streaming package path. Direct retained XML parsing, borrowed retained traversal for native/SQLite/save, page-preserving XML mutation, resumable cold construction, and streaming OPC/ZIP output remain open. Current boundaries materialize one conventional XML part under explicit allocation control and retire it iteratively.
