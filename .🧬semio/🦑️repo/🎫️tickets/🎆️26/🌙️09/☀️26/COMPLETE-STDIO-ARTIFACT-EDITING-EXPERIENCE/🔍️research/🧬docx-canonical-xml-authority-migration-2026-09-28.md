# DOCX Canonical XML Authority Migration

## Decision

The persisted DOCX snapshot must have one authority for every XML-bearing package part. The next schema cut replaces the persisted `DocxDocument` semantic copy with ordered `DocxXmlPart { path, contentType, document: XmlDocument }` records and keeps only non-XML payloads in `OpcPackage.parts`. `document.xml`, `styles.xml`, headers, footers, numbering, comments, settings, properties, and extension XML all follow the same rule. Semantic document, style, paragraph, run, and table values become bounded projections over the addressed `XmlDocument`; they are never serialized beside it.

This matches the existing PPTX logical-part split while removing PPTX's remaining mistake: a projected semantic tree must not be persisted as another authority. The current OPC content-type and relationship tables remain until the separately owned canonical metadata-XML cut replaces those duplicate tables.

## Fixture-first acceptance boundary

The neutral fixture at `🧫️fixtures/🧬️canonical-xml-authority/🔣️.json` contains a custom main-part path, a custom styles-part path, processing instructions, root extension attributes, interleaved bookmarks and unknown body nodes, a run nested inside a hyperlink, unknown run and paragraph properties, a drawing, a proof marker, table/row/cell extension properties, a final section, style extension children, Unicode, and four semantic edit intents. Its adjacent Draft 2020-12 schema fixes the fixture wire contract.

The implementation gate must build a DOCX package from this fixture, apply each edit separately, save and reopen, and compare the complete logical XML with an independent XML DOM oracle. The oracle must confirm that only the target text, `w:b`, `w:pStyle@w:val`, or addressed table-row sequence changed; every `preservedMarkers` value, node order, namespace, attribute, processing instruction, relationship target, custom part path, and unedited package byte part remains present. A second law must compose all four edits, inverse them, and recover the exact pre-edit snapshot and native package semantics. Cancellation and stale-revision cases must retain the user's draft and original snapshot.

## Coupling ledger

| Area | Current coupling to replace in the same schema cut |
| --- | --- |
| Persisted model | Base snapshot Rust/JSON/TypeScript declares `DocxRun`, `DocxParagraph`, `DocxTable*`, `DocxBlock`, `DocxStyle`, `DocxDocument`, and `DocxSnapshot.document`; base public artifact schemas and Strict/Transitional references expose it. |
| Native text/binary snapshot codecs | The base snapshot record packs `document`; `ArtifactDsl` parse/print and binary replay materialize it. The new record must pack `xmlParts` and non-XML parts exactly once. |
| Import | `document_from_xml` and `styles_from_xml` currently build the persisted semantic copy. They become projection functions returning semantic rows plus canonical XML addresses and node revisions. OPC import must route every XML-bearing part into `xmlParts`. |
| Export | `document_into_part`, `styles_into_part`, `part_already_projects`, and `sync_main_part` currently regenerate known XML and merge it with retained fragments. Export must serialize the authoritative `XmlDocument` directly at its retained part path; no reconciliation remains. |
| Diff | `DocxDocumentDiff`, block/run/table/style diffs, the five `extra*Properties` fields, between/apply/inverse/absorb, JSON Schema, TypeScript guards, and text/binary codecs all assume the semantic copy is persisted. Replace their snapshot-level authority with named XML-part diffs using the XML algebra. Semantic commands may still publish concise addressed XML mutations. |
| Mutations | `SetRunText`, block/style/table mutations, `SetSnapshot`, mutation rosters, fixtures, codecs, and derived mutation tests address `DocxBlockPath`. Each becomes a semantic command that resolves to one or more XML mutations against `{partPath,nodePath,expectedName,revision}` and validates the final XML before publication. |
| Retained preparation | Base `editor/📬️preparation` copies/diffs the typed document for `set-page`. It must prepare and apply addressed XML mutations incrementally, preserve checkpoint ownership, and refuse stale node revisions without discarding the draft. |
| Editors | Base/Strict/Transitional `set-page` handlers and main windows enumerate `document.body`. They must render bounded semantic projections from the canonical main XML, use exact XML addresses, and add formatting/style/table controls only after the authoritative mutation path exists. |
| Viewers and inferences | Three viewers, document page text extraction, and outline inference read `document.body`. They become read-only projections over `xmlParts`; unsupported nodes remain in their exact place and may be represented without being rewritten. |
| Details and schemas | Generic Details currently exposes both OPC bytes and the semantic document. After the cut it exposes each logical XML part once; schema-backed controls edit the `XmlDocument` value and the domain surface emits semantic XML mutations. Snapshot/diff/artifact JSON Schemas and TypeScript facets change atomically. |
| Tests and oracles | Existing before/after DOCX fixtures, set-snapshot fixtures, mutation laws, strict/transitional derived tests, save-part-identity tests, outline tests, editor retained tests, and public TypeScript/Ajv contracts all construct or inspect `document`. They must be updated together, with no legacy decoder or compatibility field. |

## Semantic command schema

Every command carries `partPath`, `nodePath` as decoded XML child ordinals, `expectedName`, and a revision of the addressed logical subtree. `expectedName` plus revision prevents an ordinal shifted by a concurrent insert from mutating a different node. Commands never accept an arbitrary reconstructed paragraph or table when a smaller XML mutation expresses the edit.

- `setRunText`: address one `w:r`; replace its ordered `w:t` text nodes while retaining hyperlinks, run attributes, drawings, breaks, tabs, bookmarks, and unknown children. Empty text is valid.
- `setRunFormatting`: address one `w:r`; set or remove only the named `w:rPr` child (`w:b`, `w:i`, or `w:u`) and preserve all other properties and their order.
- `setParagraphStyle`: address one `w:p`; set, clear, or insert only `w:pPr/w:pStyle`, validating the referenced style id against the projected styles part.
- `insertBlock`, `removeBlock`, `moveBlock`, `insertRun`, `removeRun`, and `moveRun`: operate on exact XML child slots and preserve interleaved unknown nodes. A structural command must declare its insertion anchor and parent revision.
- `insertTableRow`, `removeTableRow`, `moveTableRow`, `insertTableColumn`, `removeTableColumn`, and `moveTableColumn`: address `w:tbl`/`w:tr`/`w:tc`, use schema-valid templates, and retain `w:tblPr`, `w:trPr`, `w:tcPr`, merge/grid metadata, and nested block content.

The first implemented surface slice should be `setRunText`, `setRunFormatting`, `setParagraphStyle`, and table row insertion/removal because the fixture proves all four against difficult content and the current UI already exposes run drafts. Its EN/DE controls are Text/Text, Bold/Fett, Italic/Kursiv, Underline/Unterstrichen, Paragraph style/Absatzformat, Add row/Zeile hinzufügen, and Remove row/Zeile entfernen. Style values come from the live styles projection. All controls are explicit, keyboard reachable, revision-bound, and use retained command jobs with undo/redo.

## Other Office surfaces after the authority cut

PPTX needs the same single-authority rule before adding a slide/shape editor: `xmlParts` stays authoritative and the persisted `presentation` copy is removed. The projected surface then provides slide Add/Remove/Move/Duplicate, shape Add/Remove/Move, exact EMU position and size fields, text-run Bold/Italic/Font size, relationship-backed picture selection, and schema-backed placeholder type. `Other` shapes remain fully present and editable through their canonical XML.

XLSX must split every XML part into one logical authority before direct workbook structure expands. The projected surface then provides sheet Add/Remove/Move/Rename, sparse row/column insertion/removal, cell type selection, formula plus cached value, Boolean toggle, shared-string selection/editing, and style selection. Honest style controls require a typed projection of `xl/styles.xml` plus the cell's style index; the present `XlsxCell` has neither, so style controls cannot be declared until that projection and its addressed XML mutation exist.

No persisted-model migration is authored in this checkpoint. Root's concurrent DOCX diff/OPC work and queued native jobs remain untouched.

## 2026-09-28 canonical addressed editing implementation checkpoint

The persisted DOCX cut now has one XML authority: `DocxSnapshot` stores the non-XML `OpcPackage` plus ordered `DocxXmlPart` values, while `DocxDocument` is a projection used by import/export and tests. Snapshot and diff text/binary codecs, JSON Schemas, TypeScript facets, package import/export, viewers, inference, and Details follow that shape. This section supersedes the earlier planning statement that no persisted field had changed.

The first natural editing slice is canonical `setRunText`:

- `DocxXmlAddress { partPath, nodePath, expectedName, revision }` resolves directly against `xmlParts` with namespace URI/local-name identity. The revision binds the complete target subtree and the shallow structure of every ancestor and sibling along its path. Inserting an identical run before a selected identical run therefore invalidates the old address instead of editing the replacement occupant.
- Empty-namespace elements resolve as `{}` plus local name, inherited default namespaces resolve to their URI, and explicit unbound prefixes are refused. The neutral `xml-address-namespaces` fixture covers all three with a Quick XML parse oracle.
- `prepare_addressed_xml_mutation` returns a compact one-part XML diff, an exact `ReplaceXmlNode` inverse, and `changed: false` for an identical natural-text edit. `apply_addressed_xml_mutation_in_place` validates the same address and mutates only the selected XML part.
- Run text replacement consumes all direct `w:t` contributions while preserving tabs, drawings, bookmarks, unknown siblings, run attributes, and node order. It maintains `xml:space="preserve"`, creates `w:t` only when absent, and restores the exact prior node through its compact inverse. The language-neutral `run-text-fidelity` fixture covers split text and a run with no text child; Quick XML is the independent structure oracle.
- The binary mutation roster retains the established tags 0–11 and assigns tag 12 to `replace-xml-node`. Text, binary, JSON, TypeScript, GraphQL, Proto, leaf manifests, and aggregate mutation schemas carry the canonical address.
- Base, Strict, and Transitional editor windows enumerate canonical top-level runs without `project_document()`, render the complete static address through `EditableDocumentPage::with_arguments`, and require the exact nested `{address,text}` action payload. Strict and Transitional handlers, command codecs, reducers, and unit sources now emit addressed `SetRunText` mutations.

Source-only checks completed at this checkpoint: every touched Rust source parses through `rustfmt`; every new JSON fixture/schema parses through Bun; scoped `git diff --check` reports no whitespace errors. No new Cargo/Nx job was launched because the root coordinator owns the queued DOCX/full-catalog lanes. The first native proof of these sources is therefore still pending.

The remaining DOCX work is explicit:

- The shared canonical retained preparation route is mounted in Base, Strict, and Transitional. It recognizes only `SetRunText` and `ReplaceXmlNode`, uses the addressed compact plan, and refuses noncanonical or over-envelope owners without falling back to the synchronous generic reducer. The route and base editor tests still need their current-source native proof.
- Address revision computation is proportional to path depth plus the shallow attribute/child footprint of every ancestor sibling set and the target subtree. Retained admission/checkpoint work must count and page that traversal before claiming bounded execution.
- `SetRunFormatting`, `InsertBlock`, `RemoveBlock`, and `SetBlockContent` still carry `DocxBlockPath` ordinals. They must become canonical XML addresses with compact inverses. `setParagraphStyle` and table-row insertion/removal from the neutral canonical-authority fixture are not yet implemented.
- The editor currently materializes a `Vec<EditableDocumentPage>` for every top-level run after direct XML traversal. A complete large-document surface still needs window-driven run enumeration so render cost follows the requested page rather than total run count.
- Native DOCX replay/diff/inverse/save/reopen tests, Strict/Transitional action invocation, full catalog production compilation, and browser Apply/Undo/Redo remain unverified for this source fingerprint. Native7 and full18 are the next coordinated gates.

## 2026-09-28 expanded canonical mutation checkpoint

The canonical mutation layer now implements the remaining first-slice operations without reintroducing a semantic shadow document:

- `setRunFormatting {address,bold,italic,underline}` rewrites only the addressed `w:r`, preserves unknown run properties and their order, and omits an empty `w:rPr`.
- `setParagraphStyle {address,styleId}` rewrites only the addressed `w:p`, validates a non-null style id against the canonical styles XML and its paragraph style type, and clears the style without disturbing other paragraph properties.
- `insertTableRow {address,index,cells}` and `removeTableRow {address,index}` resolve the addressed `w:tbl`, map logical row ordinals to its direct `w:tr` children, and lower to exact compact XML child diffs. Insert requires at least one cell. Removal refuses the last row so it cannot author a structurally empty table through this domain control.
- `insertXmlNode {parent,index,node}` and `removeXmlNode {parent,index,expectedName,revision}` are the canonical structural primitives. Removal binds both expanded XML name and subtree revision. Their compact inverses preserve the original child index and exact authored node.

All eight addressed operations share `prepare_addressed_xml_mutation` and `apply_addressed_xml_mutation_in_place`. They return one canonical part diff, a compact exact inverse, the replacement node, and an explicit no-op flag. Direct reducers and retained preparation therefore share revision, namespace, and semantic validation. Binary tags 13–17, text grammar, Proto, GraphQL, leaf manifests, Draft 2020-12 schemas, aggregate schema, and TypeScript union/guards use the same payloads. The standalone `DocxXmlAddress` schema remains the single address contract.

The neutral canonical-authority fixture now drives text, formatting, paragraph-style, and table-row edits in sequence. The Rust law checks a compact non-`SetSnapshot` inverse for every operation, composes the inverses to recover the exact authored snapshot, saves/reopens the package, and uses Quick XML to independently count the inserted row. Additional laws refuse an unknown paragraph style and last-row removal atomically. `demo_mutation_cases` exercises text and binary replay plus diff/inverse for every new variant, including the generic structural pair.

The retained-route owner has consumed these exact variants in the base preparation seam and reports that all eight are recognized, capacity-metered, and encoded through the borrowed canonical carrier. Base, Strict, and Transitional now share the same early `SetRunText` preparation and retained text-copy cursor rather than resolving outside the base envelope.

Source checks at this checkpoint: `rustfmt` accepted every touched Rust file; Bun parsed all 45 mutation JSON files; scoped `git diff --check` is clean. No Cargo/Nx/native/browser job was started in this lane. DOCX7/full18 and retained lifecycle results remain owned by the root coordinator and must be treated as pending for this exact source fingerprint.

Remaining product work after the native gate includes moving the older ordinal block/style mutations onto the canonical structural primitives, adding move/column operations, exposing revision-bound formatting/style/table controls in every subset, and replacing the eager all-run page vector with window-driven canonical run enumeration. Address-revision cost still includes shallow traversal of ancestor sibling sets and must remain accounted by retained admission/checkpoint work.
