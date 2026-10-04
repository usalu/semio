# Presentation Export Fidelity Follow-Up

Root source review found two concrete issues in the current PresentationML serializer. `xml_document_to_pptx_text` replaces VML double-quoted style attributes with single-quoted attributes without escaping apostrophes, making valid style values malformed XML. `order_pptx_paths` contains hardcoded ordering for 62 slides, 44 images, and named fixture-specific relationships. Neither behavior follows canonical package state.

Neutral fixture `🧫️fixtures/🔤️literal-xml-export` covers apostrophes, entities, Unicode, custom VML paths, and extended properties. Independent XML/ZIP save/reopen tests and source repairs are the next root cut; no passing runtime claim yet.

A larger remaining issue is `PptxSnapshot.presentation` duplicating `xml_parts` authority: changing the semantic projection regenerates presentation/slide XML, risking loss of unmodeled properties. Import projection also compares literal prefixes. Canonical XML authority and revision-addressed natural slide edits remain required; do not call PPTX feature complete based on native encoding tests.

## Regression Evidence

The new focused native laws ran against the previous exporter and both failed: QuickXML rejected an unterminated VML style attribute after apostrophe rewriting; independent ZIP member inspection found fixture-specific sorting. After removal of both helpers, green1 ran 2/2 passing with 146 filtered. Canonical checked OPC XML serialization now returns an XML error instead of introducing filename-dependent transformations. Package ordering follows logical part order. The complete semantic/XML authority migration remains assigned to the Office worker; this focused result is not full PPTX editing acceptance.

## Canonical Authority Implementation Checkpoint

`PptxSnapshot` now persists only `schema`, `opc`, and ordered `xmlParts`. `PptxPresentation` is a derived read view. Import resolves the root office-document relationship and canonical XML parts without publishing a second independently editable presentation tree. Export writes the authored OPC order and checked canonical XML documents. Snapshot value, native, SQLite, public facet, diff, and set/patch snapshot paths use the same three-field authority.

Slide and shape editing is now revision-addressed. The schema owns XML addresses and vacancies; slide insert/remove/move, shape insert/remove, text replacement, and position replacement prepare mutations against canonical PresentationML nodes. Unrelated XML, namespace aliases, shape identities, relationship paths, rich text properties, unknown attributes, and untouched package bytes remain in the canonical snapshot. Editors and viewers derive their slide/shape lists from that authority and publish the addressed commands.

The package-level TypeScript validation completed after the migration:

- `bun nx check @semio-tech/stdio-pptx --skip-nx-cache`
- `bun nx test @semio-tech/stdio-pptx --skip-nx-cache`: 14 tests passed with zero failures.

A preview WASM compile found one ambiguous empty fatal target in canonical mutation dispatch; the source now supplies `Vec::<String>::new()`. No fresh native PPTX result is claimed because the first full run was blocked inside concurrently edited shared retained-page ownership before PPTX tests executed. A later shared OPC relationship-owner migration is also active. The next native run must prove the canonical multi-slide/rich/unknown-node save/reopen laws, addressed mutation/inverse laws, strict/transitional editors, SQLite ownership, and the earlier two literal export regressions together.

The shared OPC relationship cut is now an explicit `OpcRelationshipOwners` authority with sorted contiguous groups and named owner operations. PPTX production, subset-policy, mutation, snapshot DSL, fixtures, and tests use `groups`, `groups_mut`, `relationships`, `replace_owner`, and `remove_owner` directly. Minimal-package regeneration removes obsolete slide relationship owners through those operations instead of a map `retain` compatibility surface, and snapshot DSL validation checks `owner_count`. Preview20 also located two stale Semio presentation consumers: import now derives `PptxSnapshot::presentation()` and maps a failed projection to `PackError::Schema`; export builds a canonical package through `build_minimal_pptx` instead of supplying a removed persisted presentation argument to `from_parts`. Fresh uncached `check` runs are green for `@semio-tech/stdio-pptx`, `@semio-tech/stdio-xlsx`, and `@semio-tech/stdio-semio` (22 Semio suites, 4m18s). Preview21 and native execution remain pending.

The pre-existing standalone differential oracle catalog still describes the former index-addressed eight-kind wire and includes a removed persisted `snapshot.presentation` field. It does not describe `patch-snapshot` or the new canonical address/vacancy payloads. The canonical package's new ZIP/Quick-XML laws are current, but that older external oracle host must be rewritten or removed rather than retained as a misleading compatibility surface. It is a recorded remainder; the artifact must not be declared complete while that stale executable catalog remains.
