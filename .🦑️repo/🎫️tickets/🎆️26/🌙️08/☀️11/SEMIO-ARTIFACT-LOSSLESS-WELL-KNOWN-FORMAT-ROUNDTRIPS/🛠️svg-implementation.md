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

## Concurrent-writer collision

An external Claude process (PID 8850, child polling shell PID 80504) repeatedly overwrote the
shared XML/SVG files with the rejected XmlLexicalDocument design while this lane removed it. The
logical-only removal was reapplied more than four times across snapshot, artifact wrapper, diff,
and mutation files. Root confirmed that none of the assigned Codex agents owned those edits. This
collision remains the primary stability risk and must be stopped before the exact runtime gate.
Immediately after the final quiet-window compile fixes, the writer resumed and reinserted
XmlLexicalDocument into the XML snapshot plus lexical imports/fields/replay into the SVG snapshot.
The final forbidden-reference audit is therefore nonzero in those two snapshot files despite the
earlier zero-match audit. Further reapplication was stopped to avoid continued edit thrashing.

## Repository infrastructure fallback

Repository MCP startup failed with Broken pipe. The already-open ticket and associated
🎯aioptimizedrepo goal were supplied by the root workflow, so this lane continued in the existing
on-disk ticket without opening, reopening, duplicating, or closing a ticket.
