# PDF 1.7 Logical COS Cleanup

## Outcome

PDF 1.7 contains no `PdfPhysicalRecord`, `PdfPhysicalLayout`, source bytes, lexical token tape,
whitespace/comment replay state, or physical diff/mutation payload. Import materializes a logical
COS object graph, trailer, resolved pages, and document information; export deterministically
serializes logical COS/page state.

The supplied fixture is `/Users/ueli/Documents/semio/temp/📄️bachelor-thesis.pdf`: 6,346,331 bytes,
SHA-256 `83ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3`.

Repo MCP startup failed with `Broken pipe`; work continued inside the existing ticket. A concurrent
writer reintroduced rejected PDF physical fields after the first cleanup; the final sweep removed
them again from implementation, mirrors, facets, and the existing bachelor-thesis test.

## Logical Contract

- `PdfObject::Real(PdfDecimal)` retains sign, coefficient, and scale exactly instead of storing a
  logical number as `f64`. Numeric conversion is limited to resolved page/content analysis.
- `PdfSnapshot` contains schema/version, pages, info, full ordered COS indirect objects, and
  trailer dictionary entries only.
- COS streams retain their genuine decoded logical dictionary and byte content. No record spans,
  offsets, whitespace tokens, xref lexemes, or source fragments are persisted for replay.
- The native writer deterministically emits the object graph and trailer, while authored page
  snapshots use the existing deterministic page writer.
- Snapshot/artifact/diff/mutation codecs and TypeScript, GraphQL, JSON Schema, Protobuf, grammar,
  and binary protocol facets carry only logical fields.
- The existing bachelor-thesis test now exercises logical COS retention, canonical fixed-point
  export, DSL/pack, diff/no-op, mutation, and inverse behavior without a physical-layout bypass.

## Rejected State Audit

The final combined audit searched PDF 1.7 and IFC2X3 for `PdfPhysical`, `Part21Physical`, the word
`physical`, `.physical`, and `physical:` and returned no matches. The PDF 1.7 tree SHA-256 aggregate
was `e335ca0071d2da92420f999e1180bd177a527b59d75b2ab24348d682592f1fe9` after the final
quiet-window audit.

## Validation Evidence

- `[DEBUG] forbidden-state audit`: zero matches across PDF 1.7 and IFC2X3.
- `[DEBUG] schema validation`: `jq empty` accepted all modified PDF JSON facets.
- `[DEBUG] whitespace validation`: `git diff --check` accepted the logical-only edits.
- `[DEBUG] focused no-run`: `CARGO_TERM_COLOR=never CARGO_TARGET_DIR=<ticket>/ifc-target bun nx run
  @semio-tech/stdio-plugin:test-quick -- pdf17 ifc2x3 --no-run` reached compilation and exited `1`
  with 78 shared-tree errors. The retained log is `🧪️pdf-ifc-final-no-run.txt`.
- That run found one PDF-local stale call to removed `semantic_projection()` in the demo snapshot;
  it was fixed immediately by returning the logical decode result. The remaining diagnostics in
  the log are XML/PPTX/ZIP/SVG/DWG. A second build was not started per coordinator instruction.
- Runtime fixture identity is not claimed while the shared crate fails to compile. The logical
  fixed-point and mutation tests are present in the existing test file for the next green shared
  build.

Ticket closure remains with the primary coordinator.

## Direct Original-Byte Writer Iterations

The bachelor-thesis lifecycle now uses the imported fixture itself as the baseline and compares
native output directly to its 6,346,331 original bytes before any DSL/pack/diff/mutation route.
It no longer canonicalizes the fixture first.

- System-zlib level 9 reproduces the fixture's first filtered stream exactly.
- Exact PdfDecimal, original indirect-object order, binary header comment, literal-string
  selection, trailer order, stream Length padding, page dictionaries, and compact Group
  dictionaries advanced the first difference from byte 10 to byte 58,340.
- 🧪️pdf17-exact-original-9.log identifies the byte-58,340 mismatch as a top-level destination
  dictionary emitted inline instead of pdfTeX's multiline form.
- Top-level indirect dictionaries now receive explicit structural writer context; nested
  dictionaries remain inline, /Type /Group dictionaries use the compact adjacent-name form,
  and GoTo action dictionaries retain their distinct compact form.
- 🧪️pdf17-exact-original-11.log did not reach PDF runtime because a concurrent DWG snapshot
  rewrite produced 86 unresolved DWG bridge/type diagnostics. No PDF diagnostic was emitted.
- The PDF diff EBNF and ANTLR facets now describe the implemented sparse structural protocol and
  recursive bracketed typed payloads. The anti-shadow law scans both facets and rejects serde_json,
  source-byte, document-wire, physical, and lexical markers.

PDF exact equality is therefore still in active deterministic-writer iteration and is not claimed
by this report.

## Illustrator Flate Materialization

The first embedded Illustrator Form stream exposed a second deterministic producer policy. A
workspace-backend matrix and RFC 1951 block trace, retained in `📓️pdf-illustrator-deflate-research.md`,
proved that the stream was created with zlib window 12, level 6, memory level 5, Default strategy,
`Z_PARTIAL_FLUSH`, then `Z_FINISH`. System libz reproduces all 3,362 fixture bytes; miniz_oxide,
zlib-rs, and Zopfli do not.

The shared RFC 1950 implementation now encapsulates libz-sys behind an internal byte interface.
PDF chooses the policy only from the logical `/PieceInfo << /Illustrator ... >>` dictionary. It
retains decoded stream data and the typed Flate pipeline only; no compressed bytes or producer
state entered snapshot, diff, mutation, DSL, pack, or facets. `libz-sys` is pinned at workspace
version `1.1.29` and remains invisible in public APIs.

`illustrator_partial_flush_materialization_matches_fixture_stream` passed 1/1 in 0.020s and
requires decoded logical data to regenerate the fixture's exact 3,362-byte stream. The complete
direct-original lifecycle then advanced past the body and derived `/Length` to byte 199,341,
where named Illustrator font arrays required the standard padded `/Widths` and `/FontBBox`
serialization form. That rule is implemented. The next run, `🧪️pdf17-exact-original-19.log`, was
blocked before PDF runtime by ten concurrent DWG AC1024 signature errors; no PDF failure was
emitted in that run.

## Governing Logical-Only Finalization

The superseding contract removes byte/token/layout replay state from PDF. PDF 1.7 retains its
ordered COS graph, exact `PdfDecimal` values, object and dictionary order, trailer/xref concepts,
and deterministic writer. Snapshot DSL/pack, diff, mutations, inverse, absorb, analyzer, and
composer operate exclusively on that logical model. The bachelor-thesis lifecycle test now checks
that every intermediate representation returns the same deterministic native bytes.

Static verification found no shadow-state symbols in the PDF 1.7 or IFC2X3 artifact trees.
`rustfmt --check` parsed all edited Rust files without syntax errors, and `git diff --check`
reported no whitespace defects. Nx/Cargo was not run per coordinator instruction.

## PDF 1.7 Final Exact-Original Acceptance

The deterministic logical writer now reconstructs
`temp/📄️bachelor-thesis.pdf` exactly from the imported ordered COS graph. The original is
6,346,331 bytes with SHA-256
`83ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3`; the lifecycle test uses
that fixture directly as its baseline and never substitutes a canonicalized intermediate.

The final writer derives every materialization choice from named PDF concepts and graph structure:
typed filter pipelines over decoded stream values; pdfTeX and Illustrator dictionary/array rules;
Illustrator `PieceInfo` graph membership; typed Type3 widths; PDF name trees and page labels; and the
xref free-object chain computed from absent logical object numbers. No compressed source bytes,
layout flags, lexical strings, native file payload, `ArtifactSource`, or physical/token state is
retained by snapshot, diff, mutation, DSL, pack, or facets.

Final ticket-local Nx evidence:

- `[DEBUG] bachelor_thesis_logical_lifecycle_preserves_original_native_bytes=pass` (1/1,
  16.847s), log `🧪️pdf17-exact-original-final.log`.
- The passing lifecycle covers direct native export, structural snapshot DSL and pack, diff text and
  binary codecs, diff apply/inverse/absorb with exact native re-export, mutation text and binary
  codecs, mutation inverse with exact native re-export, analyzer native routing, composer native
  routing, and exact original-byte equality after every restoration route.
- `[DEBUG] pdf_snapshot_and_facets_forbid_native_shadow_state=pass` (1/1, 0.015s), log
  `🧪️pdf17-anti-shadow-final.log`.
- `[DEBUG] illustrator_partial_flush_materialization_matches_fixture_stream=pass` (1/1, 0.020s),
  log `🧪️pdf17-illustrator-compressor.log`.
- `[DEBUG] full_zero_selection_stdio_compile=pass` (0 run, 3,371 skipped), log
  `🧪️pdf-ifc-zero-selection-full-compile.log`.

The PDF analyzer now distinguishes native `%PDF-` input from Semio structural DSL/pack input and
invokes native deserialization only at the import boundary. Composer routing is symmetric. The
complete PDF route is therefore accepted under the logical-only exact-roundtrip contract.
