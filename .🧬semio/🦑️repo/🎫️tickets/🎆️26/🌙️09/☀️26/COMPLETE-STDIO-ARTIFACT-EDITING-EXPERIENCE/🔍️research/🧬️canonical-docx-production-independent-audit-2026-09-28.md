# Independent Canonical DOCX Production Audit

Date: 2026-09-28  
Scope: frozen production DOCX authority, import/export, native edit publication, and natural XML mutation fidelity. This was a source inspection only. No Cargo or full build was run because the coordinated production lane was already compiling and the audit brief prohibited competing builds.

## Mounted publication path

The plugin registers the Base DOCX editor at `✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️.rs:502`. The Base editor declares `bounded_native: true` at `.../📜️docx/.../🧱️base/✏️editor/🦀️.rs:180-186`, but it does not mount a `native_edit_preparation_route`. Consequently, the generic bounded-config preparation factory is live. The stale `.../🧱️base/✏️editor/📬️preparation/🦀️.rs` is not declared by the mounted editor and is not a current production compile failure.

## Findings

### P1 — A small native text edit synchronously copies a whole DOCX snapshot three times

`snapshot_details_editor_support!` constructs the generic fallback when an editor has no route (`📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs:1953-1966`); `BoundedNativeEditingEditor` supplies `None` by default (`📇️registry/🧬️contract/✏️editing/🦀️.rs:797-806`). The live generic preparation admits only the encoded mutation size (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16127-16143`) and executes inverse, diff, and post-snapshot application in one advance (`:16173-16184`).

For a `SetRunText`, the inverse is `SetSnapshot { snapshot: base.clone() }` (`.../🧬️mutations/🦀️.rs:325-326`), the mutation candidate starts as `base.clone()` (`:238-240`), and `DocxDiff::apply` starts as another `base.clone()` (`.../🔺️diff/🦀️.rs:1612-1622`). A 10-byte replacement against an arbitrarily large XML package therefore synchronously retains/materializes three full snapshots, has no XML-part checkpoint, and cannot be cancelled until this work finishes.

Reproduction shape: add a multi-megabyte unedited `customXml/item1.xml` or `word/media/image.bin` to a valid package, then change one short run through Base `set-page`. The mutation remains within the generic admission envelope while the one publication turn clones all parts and all XML trees.

Restore a mounted DOCX-specific route that performs an addressed XML copy/diff incrementally and retains a compact inverse XML diff. A size refusal is only an interim honesty guard; it does not restore normal large-document editing.

### P1 — `set-run-text` leaves split text nodes behind and may lose Word whitespace semantics

`set_run_text_xml` finds only the first direct `w:t`, clears its children, and returns immediately (`.../🧬️mutations/🦀️.rs:186-200`). It does not replace the run's other `w:t` nodes. The semantic projection concatenates every `w:t` (`.../🚪️io/📥️import/🧩️deserializers/🦀️.rs:43-59`).

Narrow reproduction:

```xml
<w:r><w:t>A</w:t><w:tab/><w:t>B</w:t></w:r>
```

Applying `SetRunText(..., "X")` produces the logical run:

```xml
<w:r><w:t>X</w:t><w:tab/><w:t>B</w:t></w:r>
```

Its projected text is `XB`, rather than the requested `X`. The mutation should replace every ordered `w:t` text child while retaining the intervening tab/drawing/unknown children in their positions.

The same branch retains attributes on an existing first `w:t` but does not add `xml:space="preserve"` when a replacement begins or ends with whitespace. For `<w:t>Hello</w:t>` followed by replacement `" leading "`, it emits `<w:t> leading </w:t>`; WordprocessingML text whitespace can then be normalized by consumers. The insertion fallback at `:197-199` does add `xml:space`, so the two paths disagree. The setter must maintain `xml:space` from the resulting text, not merely from whether a text node already existed.

### P1 — Published natural mutations still identify targets by unstable semantic ordinals

The `SetRunText` wire payload contains only `DocxBlockPath`, `run_index`, and `text` (`.../🔤set-run-text/🦀️.rs:10-14`; schema `.../🔤set-run-text/🧬️schema/🔣️.json:7-70`). Its target metadata is empty (`:28-30`). Mutation application selects the Nth projected block and depth-first `w:r` (`.../🧬️mutations/🦀️.rs:130-135`, `:169-200`); the Base editor validates only the run's text revision (`.../✏️editor/🦀️.rs:40-59`).

Narrow reproduction: a paragraph contains two distinct runs with identical text `"same"`. A draft opens for ordinal 1. A concurrent XML edit inserts a third `"same"` run before ordinal 1. The current ordinal-1 text and its text-only revision remain `"same"`, so the draft succeeds but edits the inserted run, not the originally addressed XML node.

Commands need the planned `{ partPath, nodePath, expectedName, revision }` address, with a revision of the addressed XML subtree. Reject a stale address before any mutation; never use a semantic projection ordinal as the persisted identity.

### P1 — Two required canonical semantic edits have no production command

The canonical fixture defines `setParagraphStyle` and `insertTableRow` at `.../🧫️fixtures/🧬️canonical-xml-authority/🔣️.json:15-19`. The production mutation roster has neither; its complete list is `set-snapshot`, block/run/style, and raw-part operations (`.../🧬️mutations/🦀️.rs:102`). The only fixture integration test applies its text and formatting cases (`.../🧬️schema/🧪️tests/🔬️unit/🦀️.rs:410-416`) and does not execute the fixture's style or row cases.

This is a product surface gap, not just a stale fixture: the defined canonical authority acceptance boundary cannot be reached. Implement the two addressed XML commands, include live-style validation for paragraph style, preserve `w:tblPr`/`w:trPr`/`w:tcPr` and nested blocks for row insertion, then run each fixture edit separately plus composition/inverse.

### P2 — Semantic projections accept one namespace prefix rather than the WordprocessingML namespace

Import and mutation helpers compare literal strings such as `"w:document"`, `"w:body"`, `"w:r"`, and `"w:t"` (`.../🚪️io/📥️import/🧩️deserializers/🦀️.rs:141-168`; `.../🧬️mutations/🦀️.rs:149-215`). XML namespace prefixes are aliases, so a valid document using `xmlns:word="http://schemas.openxmlformats.org/wordprocessingml/2006/main"` and `<word:document>` cannot project or be edited, even though authority/export can preserve it. Resolve element and attribute names by namespace URI and local name while retaining the source lexical prefix in the authoritative XML tree.

## Scope notes

The canonical XML-part split itself is present in `DocxSnapshot` (`.../🧬️schema/📸️snapshot/🦀️.rs:127-160`), and export validates authority before serializing logical XML (`.../🚪️io/📤️export/🧵️serializers/🦀️.rs:155-173`). Custom main/styles paths and retained relationships have focused coverage. The issues above are in the live editing and semantic-mutation path.

The generic XML codec's UTF-8-only import/export declaration boundary is being handled in the separate XML repair lane, so it is intentionally not duplicated here.
