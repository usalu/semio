# Well-Known Artifact Roundtrip Workforce Plan

## Objective

Import and export each supplied artifact without changing a byte while exercising the complete logical artifact lifecycle: native deserialization, snapshot DSL and pack codecs, diff algebra, mutations and inverses, registered analyzer/composer IO, deterministic native materialization, and literal byte comparison.

| Lane | Fixture | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| PDF | `temp/📄️bachelor-thesis.pdf` | 6,346,331 | `83ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3` |
| DWG | `temp/architectural_example.dwg` | 148,638 | `52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7` |
| SVG | `temp/artifacts.svg` | 423,414 | `62a1922aad9e06ba7d4a55fe13c360f286eb720873bcf6d8e6ffd1d52e782fc9` |
| MP4 | `temp/bauen-mit-bestand.mp4` | 16,086,051 | `54b0672cca68a474d44c6096abb6579160b4d33b0f637f588e2e0752373e05c7` |
| PPTX | `temp/domai-specific-programmaning-language-for-architects.pptx` | 16,341,544 | `477900b1746139840890bc4edb653c488f3d18f9da34d231332b5db41d4caa8a` |
| IFC | `temp/wellness-center-sama.ifc` | 21,282,588 | `f4dbc661d555bbf92fb80a40443f6b6b540fa0a833b85d78487930368147b593` |

## Governing design contract

1. Native bytes exist only at the IO boundary. Deserialization must consume them into a logical, standard-domain snapshot.
2. No snapshot, artifact, diff, mutation, schema facet, DSL, pack, cache, provenance record, or helper may retain the complete source bytes.
3. No physical surrogate is allowed: token tapes, lexical documents, original whitespace/comment streams, ZIP record replay, page-layout replay, box-layout replay, xref replay, or equivalent encoding-decision state are forbidden.
4. Native serialization materializes a file solely from the logical snapshot through the ordinary deterministic writer.
5. Genuine logical binary values remain allowed, including media samples, embedded images, fonts, compressed document streams when compression is their semantic value, and unknown standard extension payloads. They may not collectively be used as a disguised source-file replay.
6. Byte-affecting information belongs in the schema only when it is a logical format value exposed to users or required by the standard-domain model. Encoding accidents are not schema state.
7. Snapshot DSL and pack codecs serialize the logical snapshot directly. They may not invoke native export and re-import internally.
8. Diff and mutation operate on logical fields. Empty operations preserve the snapshot, effective operations change it, and inverse/absorb laws restore it without hidden provenance.
9. Native output must be deterministic for a given logical snapshot.
10. Acceptance remains literal byte equality for these fixtures. A mismatch is evidence that the logical model or deterministic writer is incomplete; it is never repaired with source replay, fixture hardcoding, or a physical shadow model.

## Workforce topology

The three user-required parallel implementation lanes remain continuously assigned:

| Agent | Primary lane | Follow-on lane | Ownership |
| --- | --- | --- | --- |
| `dwg_roundtrip` | DWG | PPTX | DWG logical model/writer and PPTX logical OPC model/facets/tests |
| `svg_roundtrip` | SVG/XML | shared XML callers | Logical XML/SVG schema, deterministic XML writer, DSL/pack/diff/mutation/io laws |
| `ifc_roundtrip` | IFC/STEP | PDF | Logical STEP/IFC decimal/header/entity model and logical PDF COS model/writer |
| primary orchestrator | integration | MP4 | Contract enforcement, MP4 logical model/writer, shared compile/test gates, collision reconciliation |

All workers share one tree. Each lane inspects current file contents immediately before every patch, preserves concurrent edits, and reports collisions rather than overwriting an active owner. Rejected physical/source state reintroduced by another workflow is removed only after that workflow has been quiet long enough to make a stable reconciliation pass.

## Dependency graph

```text
G0 immutable fixture hashes + logical-only contract
 ├─ L-DWG logical decode/write/schema laws
 ├─ L-SVG logical XML decode/write/schema laws
 ├─ L-IFC logical STEP decode/write/schema laws
 ├─ L-PDF logical COS decode/write/schema laws
 ├─ L-MP4 logical box/track/sample decode/write/schema laws
 └─ L-PPTX logical OPC/package decode/write/schema laws

L-* + zero forbidden-state audit
 └─ I1 Rust and facet compilation
     ├─ I2 direct analyzer/composer paths
     ├─ I3 DSL and pack roundtrips
     ├─ I4 diff, inverse and absorb laws
     ├─ I5 mutation and inverse laws
     └─ I6 deterministic native writer
         └─ E1 literal length/cmp/SHA-256 matrix
             └─ E2 aggregate Bun/Nx gate + ticket evidence
```

No lane is complete before its public end-to-end flow produces the fixture's original byte sequence.

## Schema-first format work

### PDF

- Parse ordered logical COS objects, dictionaries, arrays, names, strings, references, streams and trailer values.
- Represent decimal values exactly rather than through `f64`.
- Derive object offsets, xref entries, `startxref`, and EOF syntax during serialization.
- Preserve embedded/stream payloads as logical object values; never preserve the original PDF record layout.
- Expand the deterministic writer until the supplied document materializes identically.

### DWG

- Parse the supported AC1024 sections into typed drawing entities, handles, tables, geometry, metadata and logical opaque extension records.
- Derive compression, page directory, offsets, checksums, padding and encrypted headers during serialization.
- Keep cross-domain drawing/mesh bridges on the logical drawing model; never decode through a retained source buffer.
- Expand typed section coverage and deterministic compression until the supplied drawing materializes identically.

### SVG/XML

- Model declaration values, doctype semantics, ordered nodes, ordered attributes, namespaces, processing instructions, comments, CDATA/text values and entities logically.
- Do not retain quotes, delimiter spelling, whitespace tokens, source substrings, or a lexical document.
- Materialize through one deterministic XML writer and expand its choices and logical fields until the supplied SVG is identical.

### MP4

- Model ordered logical boxes, movie/track/media headers, edit lists, sample descriptions, timing, chunk grouping, offsets, metadata, samples and extension payloads.
- Treat media samples and genuine extension payloads as logical bytes; do not retain original box headers, offsets, padding or a physical box tree.
- Recompute sizes, chunk offsets and table encodings through deterministic multi-pass serialization.
- Expand logical metadata and writer ordering until the supplied movie is identical.

### PPTX

- Model OPC content types, relationships, logical XML parts, presentation/slide/theme/media structures and genuine binary package parts.
- Materialize XML using the logical XML writer and ZIP using the deterministic package writer.
- Do not retain local/central ZIP headers, compressed member bitstreams, timestamps as replay metadata, gaps, descriptors or original XML text.
- Expand logical OPC/package fields and deterministic writer choices until the supplied deck is identical.

### IFC/STEP

- Model standard headers, typed EDM preamble values, ordered entities, references, enums, exact decimal values, strings, lists and schema extensions.
- Do not retain STEP token streams, comments, whitespace, line-ending records or original numeric/string spelling.
- Materialize through a deterministic Part 21 writer.
- Expand typed logical header/entity coverage and writer choices until the supplied model is identical.

## Lifecycle laws

Every format must execute these public paths against the exact fixture:

1. Native bytes → registered analyzer/deserializer → logical snapshot.
2. Logical snapshot → registered composer/serializer → native bytes.
3. Logical snapshot → DSL text → logical snapshot → native bytes.
4. Logical snapshot → binary pack → logical snapshot → native bytes.
5. `between(base, base)` → apply empty diff → native bytes.
6. Effective logical diff → apply → verify changed snapshot/output → inverse or absorb → restore base → native bytes.
7. No mutation → native bytes.
8. Effective logical mutation → verify changed snapshot/output → inverse → restore base → native bytes.
9. Set-snapshot operation through text and binary operation codecs → native bytes.
10. Two serializations of one restored snapshot → identical native byte arrays.

For every restoration scenario, assert snapshot equality before asserting output equality. Effective operations must prove that the writer did not return stale native bytes.

## Acceptance matrix

| Gate | PDF | DWG | SVG | MP4 | PPTX | IFC |
| --- | --- | --- | --- | --- | --- | --- |
| Public native import succeeds | required | required | required | required | required | required |
| Forbidden-state audit is zero | required | required | required | required | required | required |
| Direct logical export identical | required | required | required | required | required | required |
| DSL logical roundtrip identical | required | required | required | required | required | required |
| Pack logical roundtrip identical | required | required | required | required | required | required |
| Empty diff export identical | required | required | required | required | required | required |
| Effective diff changes output | required | required | required | required | required | required |
| Diff inverse/absorb identical | required | required | required | required | required | required |
| No mutation export identical | required | required | required | required | required | required |
| Effective mutation changes output | required | required | required | required | required | required |
| Mutation inverse identical | required | required | required | required | required | required |
| Repeat export deterministic | required | required | required | required | required | required |
| Length, `cmp`, SHA-256 identical | required | required | required | required | required | required |

## Forbidden-state audit

Before compilation and again before final evidence, inspect every Rust and schema/facet surface for:

- `ArtifactSource`, `source.bytes`, `sourceBytes`, source hashes used for replay;
- `Physical`, `physical`, `lexical`, token tapes, raw whole-file/document/package snapshots;
- encode branches that bypass the deterministic writer for an unchanged snapshot;
- decode branches that populate a second export-authoritative representation;
- mutation/diff fields that merely carry source or encoding-decision state.

Matches are reviewed semantically because ordinary words such as a PDF physical page size may be legitimate domain terminology. The audit result is recorded per format with exact file/line evidence.

## Verification workflow

### V0 — static integrity

- Recompute all six input sizes and hashes.
- Validate every changed JSON facet with `jq empty`.
- Parse changed Rust with `rustfmt --check`; formatting output is not a passing compile claim.
- Run `git diff --check` without modifying the index.
- Confirm no new permanent script or separate test file was created.

### V1 — compilation

- Run the smallest existing Bun/Nx target that compiles the stdio plugin.
- Resolve errors by ownership lane and rerun only after the shared tree is stable.
- Record exact command, exit code and first diagnostic. Never infer green status from compilation progress.

### V2 — focused lifecycle tests

- Run exact fixture filters for each format through existing tests.
- Capture analyzer/composer, DSL, pack, diff, mutation, inverse/absorb and deterministic writer assertions.
- A failed byte comparison becomes the next implementation input: locate the first offset and classify the missing logical value or writer rule.

### V3 — literal equality evidence

For each scenario record input/output length, first differing offset if any, `cmp` status and SHA-256. Output files and logs remain inside this ticket. Inputs under `temp/` are immutable.

### V4 — aggregate gate

- Run the relevant stdio project tests through `bun nx`.
- Run existing policy/type/facet targets exposed by the project's `📜️script.ts`.
- Recompute fixture hashes after all commands and record ticket-owned regressions only.

## Collision protocol

1. Identify the active writer and exact colliding files with read-only process and mtime checks.
2. Do not kill another user's process or use modifying Git commands.
3. Continue non-colliding schema, facet, report and test work.
4. Wait for a stable quiet interval, then make one logical-only reconciliation pass.
5. Hash or timestamp reconciled files, run the forbidden-state audit, and compile immediately.
6. If rejected state is reintroduced, record the collision and repeat only after another quiet interval; never claim the tree is stable while it is being overwritten.

## Completion criteria

The ticket closes only when all six fixtures satisfy every lifecycle law and produce byte-for-byte identical native exports through logical snapshots alone; all focused and aggregate Bun/Nx gates pass; facet audits are clean; runtime evidence is stored in the ticket; and the existing ticket is closed through repo MCP when that transport is available.

Current state: implementation is active. No fixture or aggregate green claim has been made. Repo MCP startup failed with a broken stdio transport, so work continues in the already-open ticket and closure must retry the required MCP path.
