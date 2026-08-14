# PDF 1.7 Ordered Physical Roundtrip Implementation

## Scope

PDF 1.7 now combines the exact `PdfDecimal` COS model with `PdfPhysicalLayout`, an ordered sequence
of typed lexical/physical records. There is no whole-file source byte field and anti-bypass tests
reject any single record equal to the complete fixture.

Repo MCP startup failed with `Broken pipe`; work continued in the already-open ticket supplied by
the parent. The shared tree was concurrently edited throughout this lane.

## Physical and logical materialization

- `PdfObject::Real` uses `PdfDecimal { negative, coefficient, scale }`, avoiding `f64` loss in the
  logical COS model. Numeric conversion to `f64` occurs only for resolved page/content analysis.
- Native import parses the full logical COS graph and tokenizes every byte into ordered header,
  whitespace, comment, delimiter, name, string, number, stream-data, xref, trailer, `startxref`,
  EOF, keyword, or unknown records.
- `semanticBlake3` binds records to `PdfSnapshot::semantic_projection()`. Normal export rebuilds
  bytes only by concatenating records, reparses the result, and requires exact semantic equality.
  A forged record tape with a stale fingerprint is rejected.
- Dirty page/info edits use the authored page writer; dirty COS/trailer edits use the logical COS
  writer. Simultaneous page-model and COS-model edits are rejected until an explicit
  materialization strategy is supplied. Mutations retain the imported COS and physical state so
  mutation and diff inverses restore the exact original snapshot.
- Snapshot/artifact conversion, set-snapshot, diff apply/between/inverse/absorb, mutation text and
  binary codecs, snapshot DSL/pack, raw binary IO, and all Rust/TypeScript/GraphQL/JSON/Proto/text/
  binary facets carry the physical layout.
- Snapshot, diff, and mutation binary decoders validate format markers and trailing bytes.

## Exact fixture coverage

The existing bachelor-thesis test uses `/Users/ueli/Documents/semio/temp/📄️bachelor-thesis.pdf`
(6,346,331 bytes, SHA-256 `83ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3`) and asserts:

1. exact native import/export byte identity;
2. more than 1,000 ordered records, no whole-file record, and complete byte coverage;
3. stream-data, xref, and trailer record classification;
4. exact DSL and pack roundtrips followed by exact native export;
5. self-diff and no-op exact export;
6. dirty mutation export, mutation/diff codec roundtrip, inverse restoration, and exact export;
7. semantic anti-bypass rejection after changing a `/MediaBox` physical number while retaining the
   old semantic fingerprint.

## Files changed

- PDF 1.7 snapshot, artifact, diff, mutation, outline, and IO Rust components.
- PDF 1.7 snapshot/artifact/diff TypeScript, GraphQL, Protobuf, JSON, text grammar, and binary
  protocol facets.
- Existing bachelor-thesis PDF test under the 1.4 example tree, which exercises the 1.7 engine.

## Validation

- `[DEBUG] anti-whole-file audit`: physical state consists only of ordered typed records; fixture
  assertions forbid a record spanning or equaling the entire input.
- `[DEBUG] schema validation`: `jq empty` accepted the snapshot, artifact, and diff JSON facets.
- `[DEBUG] whitespace validation`: `git diff --check` accepted all PDF-local changes.
- `[DEBUG] parse-oriented validation`: `rustfmt --check` parsed every changed PDF Rust file and
  reported formatting-only differences, with no syntax error.
- Cargo and Nx were intentionally not run per parent coordination; runtime passing is not claimed.
