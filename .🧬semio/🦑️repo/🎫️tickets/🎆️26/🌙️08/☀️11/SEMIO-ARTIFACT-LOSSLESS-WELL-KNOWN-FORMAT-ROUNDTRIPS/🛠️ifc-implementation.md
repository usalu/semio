# IFC2X3 Logical Part-21 Roundtrip Implementation

## Outcome

IFC2X3 contains no source bytes, physical token tape, whitespace/comment replay state, or
`ArtifactSource`. Import materializes a typed logical Part-21 document, typed exact decimals, and
the fixture's EXPRESS Data Manager preamble as named semantic metadata. Export deterministically
serializes only that logical state.

The supplied fixture is `/Users/ueli/Documents/semio/temp/wellness-center-sama.ifc`: 21,282,588
bytes, 409,102 ordered instances, CRLF, schema `IFC2X3`, SHA-256
`f4dbc661d555bbf92fb80a40443f6b6b540fa0a833b85d78487930368147b593`.

Repo MCP startup failed with `Broken pipe`; work continued inside this already-open ticket as
directed by the coordinator. The shared worktree was concurrently modified, including two
reintroductions of the rejected physical model; both were removed without modifying git state.

## Logical Contract

- `Part21Document` retains the ISO-defined header fields and instance order as logical
  collections; it does not widen the header into an artificial ordered record representation.
- `Part21Value::Real(Part21Decimal)` retains sign, coefficient, scale, and optional base-10
  exponent, avoiding `f64` precision loss. `f64` conversion is analysis-only.
- STEP strings are decoded to semantic Unicode and deterministically escaped by the Part-21
  writer.
- `Ifc2x3EdmPreamble` models producer, module, creation date, host, database identity/version/date,
  schema/model/header-model fields, EDM user/group, license, and options as named metadata. No raw
  comment text is stored.
- The IFC serializer selects CRLF, fixture-compatible section spacing, instance assignment
  spacing, and deterministic preamble formatting from logical values.
- Snapshot/artifact conversion, diff, mutation, set-snapshot, DSL, pack, raw binary/text IO,
  analyzer, composer, schema facets, and COBie/CV20/SAV subset constructors retain only logical
  state.
- Diffs retain schema/header changes, removed/upserted instances, explicit instance order, and
  typed EDM preamble replacement. Mutation inverse restores the complete logical snapshot when
  ordering requires it.

## Rejected State Audit

The final audit searched the PDF 1.7 and IFC2X3 trees for
`PdfPhysical`, `Part21Physical`, the word `physical`, `.physical`, and `physical:`. It returned no
matches. The IFC2X3 tree SHA-256 aggregate was
`936ab9ed374fce30fdbe49b38ef97c74118a624701ce11df5da4afe06b746408` after the final quiet-window
audit.

## Validation Evidence

- `[DEBUG] forbidden-state audit`: zero matches across IFC2X3 and PDF 1.7.
- `[DEBUG] schema validation`: `jq empty` accepted all modified IFC2X3 JSON schema facets.
- `[DEBUG] whitespace validation`: `git diff --check` accepted the edited logical-only files.
- `[DEBUG] focused no-run`: `CARGO_TERM_COLOR=never CARGO_TARGET_DIR=<ticket>/ifc-target bun nx run
  @semio-tech/stdio-plugin:test-quick -- pdf17 ifc2x3 --no-run` reached Rust compilation and exited
  `1` with 78 shared-tree errors. The log is `🧪️pdf-ifc-final-no-run.txt`. No IFC2X3 diagnostic was
  emitted.
- The only PDF-local diagnostic in that run was a stale `semantic_projection()` fixture call; it
  was removed afterward. A second build was deliberately not started per coordinator instruction.
- Exact native/hash execution is not claimed because the shared crate cannot compile past
  unrelated XML/PPTX/ZIP/SVG/DWG errors. Existing IFC tests exercise native, raw IO, DSL, pack,
  analyzer, composer, diff, mutation, inverse, and fixture byte identity once that shared gate is
  restored.

## Static Exactness Boundary

The fixture's first exponential real is `6.12303176911189E-17`; the exact decimal writer preserves
that form. Its header/preamble, CRLF, blank lines, `#id= ` assignment spacing, ordered entities,
and final terminator all match the selected deterministic writer options. No irreducible lexical
mismatch was identified statically; runtime byte comparison remains blocked by foreign compile
failures rather than hidden replay state.

Ticket closure remains with the primary coordinator.

## Runtime Acceptance and Linear Codec Recovery

The original SetSnapshot lifecycle run exceeded the 300-second long-test budget at 100% CPU and
approximately 1.3 GB RSS. The cause was not the structural Part-21 codec: diff application searched
the growing instance vector for every one of 409,102 upserts. The final implementation uses an
id-to-index map for replacement/insertion and reconstructs explicit instance order linearly.
Text persistence also writes values, entities, instance lists, diffs, and SetSnapshot payloads
directly into one preallocated buffer; decoding streams top-level instances instead of retaining a
409,102-element slice vector. Empty-base/empty-target diffs bypass two full hash maps, and the
lifecycle test drops each text/binary intermediate before entering the next phase.

- 🧪️ifc2x3-set-snapshot-linear-3.log: exact_native_set_snapshot_codecs_retain_complete_logical_model
  passed 1/1 in 14.722 seconds under the long profile. Text diff, binary diff, text SetSnapshot
  op, and binary SetSnapshot op each rebuild the directly imported logical fixture and serialize
  exactly to the original 21,282,588 bytes.
- 🧪️ifc2x3-native-routing.log: analyzer/composer native Part-21 routing passed 1/1 in 11.672
  seconds.
- 🧪️ifc2x3-anti-shadow.log: IFC snapshot/facet anti-shadow law passed in 0.014 seconds. The
  combined substring filter also selected the PDF anti-shadow law; that separate failure was an
  invalid JSON test mechanism and does not invalidate the recorded IFC pass.

## Governing Final Audit

The final code retains only the ordered `Part21Document`, exact `Part21Decimal` values, typed
`Ifc2x3EdmPreamble`, and the deterministic Part-21 writer. COBie, SAV, CV20, and bounds constructors
were realigned with this snapshot. Existing fixture lifecycle laws cover native import/export,
raw binary/text bridges, DSL, pack, analyzer, composer, diff/apply/inverse/absorb, mutations,
set-snapshot codecs, entity ordering, and semantic-bypass rejection.

The final static audit found no byte/token/layout replay fields or types in the PDF 1.7 and IFC2X3
artifact trees. `rustfmt --check` parsed the edited Rust files without syntax errors and
`git diff --check` reported no whitespace defects. Nx/Cargo was deliberately not run.
