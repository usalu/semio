# SVG Logical Roundtrip Implementation

## Required architecture

The SVG/XML lane is schema-first and logical-only. Whole-file replay, source byte fields,
physical byte fields, token/lexeme tapes, raw source caches, and trivia retained solely for replay
are prohibited.

The shared logical XML model now represents:

- ordered elements, attributes, and child nodes;
- exact Unicode text, CDATA, comments, and processing instructions;
- the typed XML declaration;
- the doctype construct;
- ordered prolog nodes, including the dvisvgm generator comment in temp/artifacts.svg.

XmlSnapshot and SvgSnapshot persist only schema plus XmlDocument. Their deterministic writer uses
single-quoted attributes, a space before empty-element closure, 120-column attribute wrapping with
four-space depth indentation, and logical whitespace text nodes. SVG↔XML bridges copy the logical
document directly; they do not serialize and reparse or transfer hidden source state.

Diffs, set-snapshot text/binary codecs, mutation inverse/absorb, DSL, and pack codecs include the
logical prolog alongside declaration, doctype, and root. The Semio value bridge includes prolog as
a list of typed XML nodes. Rust, TypeScript, GraphQL, JSON Schema, and protobuf facets were updated
to expose the same logical document shape.

## Fixture evidence

- [DEBUG] xmllint --noout temp/artifacts.svg: exit 0.
- [DEBUG] wc -l -c temp/artifacts.svg: 5141 423414.
- [DEBUG] shasum -a 256 temp/artifacts.svg:
  62a1922aad9e06ba7d4a55fe13c360f286eb720873bcf6d8e6ffd1d52e782fc9.
- The file has no final newline.
- The prolog is the XML 1.0 UTF-8 declaration followed by the dvisvgm 3.0.4 generator comment.

Existing test files were extended in place. Coverage includes direct import/export, artifact pack,
DSL, diff, mutation, inverse, absorb, diff/op text and binary codecs, analyzer raw/pack routes,
composer raw/pack routes, and SVG↔XML bridges. No new test or permanent script file was created.

## Validation evidence

rustfmt --edition 2021 --check parsed all six shared XML/SVG snapshot, diff, and mutation Rust
components; it reported formatting diffs but no syntax error.

[DEBUG] bun JSON parsing accepted the updated XML/SVG snapshot, artifact, and diff JSON Schema
facets.

The focused compile command was:

    bun nx run @semio-tech/stdio-plugin:test-quick -- --no-run

It reached the stdio crate and exposed the complete constructor migration. At that run, the lane
had five stale XML lexical initializers, five stale SVG lexical initializers, two lexical test
accesses, and fourteen missing XmlDocument.prolog constructors. Those lane-local lexical
references were removed, the test was rewritten around logical prolog/projection state, and all
fourteen prolog constructors were updated (eleven in this lane and three PPTX-local callers
coordinated with the PPTX/DWG lane). Remaining reported failures were concurrent DWG, IFC, MP4,
ZIP, and PDF physical-model work.

After a three-minute eighteen-second quiet window, the forbidden-reference audit returned zero
matches and the focused no-run compile was repeated. It reported 31 errors: this lane's three
SvgAnalyzer test errors were corrected by targeting SvgAnalyzerAnalysis directly, and two
duplicate OPC prolog fields were removed. The remaining compiler errors were concurrent
PPTX/PDF/ZIP/DWG/shared work. The focused native roundtrip test has not yet reached execution, so
this report does not claim a passing byte/hash comparison.

On 2026-08-14 the lane was resumed and monitored without editing. The complete XML/SVG subset
trees retained fingerprint
`d95054b51fb780309d1e25519299cc88b509221c644c992e04976d6b519c74be` for five consecutive
minutes while PID 8850 remained sleeping. One final logical-only cleanup then removed the
reintroduced lexical state from XML/SVG snapshots, artifact wrappers, diffs, and mutation
text/binary codecs.

[DEBUG] The immediate, post-build, and final forbidden-reference audits all produced no matches
and exited 1, as expected for `rg` with an empty result. The audited expressions were
`XmlLexical`, `xml_lexical`, `.lexical`, `lexical:`, `ArtifactSource`, semantic Blake3/source-byte
spellings, and raw-source spellings across both complete XML 1.0 and SVG 1.1 `any` subset trees.

[DEBUG] The focused `bun nx run @semio-tech/stdio-plugin:test-quick -- --no-run` reached the stdio
crate and failed with 15 errors, none in XML or SVG. The exact external inventory was: two missing
PPTX length-prefixed byte helpers; one MP4 sample-entry argument; three Semio-presentation PPTX
constructor arguments; one PPTX `Other` field rename; one MP4 type annotation; three BCF ZIP
physical initializers; two ZIP physical `DslField` errors; and one DWG test calling `expect` on a
`Vec<u8>`. The command therefore establishes that the logical-only SVG/XML source compiled in this
shared build, but the crate-wide no-run gate remains blocked before runtime tests can execute.

## Concurrent-writer collision

An external Claude process (PID 8850, child polling shell PID 80504) repeatedly overwrote the
shared XML/SVG files with the rejected XmlLexicalDocument design while this lane removed it. The
logical-only removal was reapplied more than four times across snapshot, artifact wrapper, diff,
and mutation files. Root confirmed that none of the assigned Codex agents owned those edits. This
collision remains a stability risk. During the final 2026-08-14 pass PID 8850 remained alive but
sleeping, the required five-minute quiet window completed, and no overwrite occurred during
cleanup or either focused compile. The final forbidden-reference audit is currently clean.

## Repository infrastructure fallback

Repository MCP startup failed with Broken pipe. The already-open ticket and associated
🎯aioptimizedrepo goal were supplied by the root workflow, so this lane continued in the existing
on-disk ticket without opening, reopening, duplicating, or closing a ticket.
# Governing Logical-State Override (2026-08-14)

The final governing contract removes the experimental XML/SVG lexical token stream and DWG physical-layout shadow state. XML/SVG snapshots again persist only typed declaration, doctype, prolog, ordered root/nodes, and ordered attributes; their deterministic XML writers are the sole materializers. The lexical field and its diff, mutation, DSL/pack, bridge, and constructor propagation were removed, including stale DOCX/XLSX/BCF/Semio initializers.

DWG AC1024 now persists only version/header fields, logical drawing, named logical sections/pages, and derived decode state. DwgPhysicalLayout, its artifact/diff/inference propagation, physical decoder/writer, and physical lifecycle assertions were removed. Native export routes through encode_r2004_canonical(snapshot).

Static validation only: Rustfmt parse-check covers XML, SVG, and DWG snapshot/artifact/diff/mutation/I/O files. No Cargo/Nx runtime pass is claimed.

## Zero-Shadow Logical Model (2026-08-14)

The tightened contract is now enforced in the XML/SVG schema rather than documented only as a
writer convention. `XmlDocument.doctype` is a typed `XmlDoctype` containing its declared name,
typed SYSTEM/PUBLIC external identifier, and typed internal entity declarations. The parser
rejects unsupported DTD declarations instead of storing their source syntax. The deterministic
writer materializes the doctype from those fields. XML/SVG snapshot, diff, mutation, DSL, pack,
and Semio-value bridge codecs carry that typed value; TypeScript, GraphQL, JSON Schema, and Proto
facets no longer expose a string doctype. Stale SVG source-token grammar productions were removed.

Existing XML and SVG test modules now include compile-time facet audits spanning Rust,
TypeScript, GraphQL, JSON Schema, Proto, grammar, and protocol files. They reject source/provenance
fields and a string/raw doctype representation.

Validation:

```text
[DEBUG] rustfmt_parse_xml_svg=pass
[DEBUG] xml_snapshot_json_jq=pass
[DEBUG] xml_svg_forbidden_state_static_matches=0
[DEBUG] scoped_nx_target=.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS/🎯️svg-logical-target
```

The isolated scoped Nx build compiled all XML/SVG/DWG changes without a lane-local error, then
stopped on unrelated PDF missing-type imports/one PDF byte-vector mismatch and MP4's missing
`DiffAlgebra::is_empty`. Consequently the facet and exact native test bodies did not execute and
no byte-equality pass is claimed. The existing SVG lifecycle tests still require direct, pack,
DSL, XML bridge, diff/apply, no-op, mutation/inverse, absorb, analyzer, and composer exports to
equal `temp/artifacts.svg`.
