# PPTX Lossless Native Roundtrip Implementation

## Superseding Physical-Model Design

This section supersedes the earlier `ArtifactSource` design retained below only as implementation
history. PPTX no longer persists or replays one opaque archive byte image and no export branch
returns captured source bytes.

`PptxSnapshot` now persists `PptxPhysicalState { archive: ZipSnapshot, semantic_blake3 }`. The
archive is a real semantic ZIP snapshot whose physical layout is decomposed into explicit records:

- each local entry stores signature, version, flags, method, DOS time/date, CRC, classic sizes,
  original filename/extra bytes, original compressed member payload, explicit data-descriptor
  signature/width/CRC/sizes, and trailing inter-record gap;
- each central entry stores signature, versions, flags, method, time/date, CRC, classic sizes,
  original filename/extra/comment bytes, disk start, attributes, and local offset;
- ZIP64 EOCD and locator values are typed fields, as are classic EOCD disk/count/size/offset and
  comment bytes;
- archive prefix, unknown central-directory extension bytes, and post-comment trailer remain
  isolated raw extension/gap channels rather than an aggregate archive or header blob.

`decode_zip` constructs this model. The ordinary `encode_zip` writer writes every header from
those fields and emits the preserved compressed member payloads/descriptors. It selects this
physical representation only while its semantic ZIP-entry fingerprint is valid; dirty snapshots
fall through to canonical recompression. `encode_pptx` similarly verifies the typed
OPC/PresentationML projection and then invokes the ordinary ZIP writer. A dirty imported PPTX is
regenerated canonically instead of replaying stale bytes or returning the former dirty-import
error.

PPTX ArtifactDsl and ArtifactPack now serialize the persisted snapshot model as JSON text/bytes
inside their Semio envelopes. They no longer serialize a native PPTX archive and parse it back.
PPTX diff and set-snapshot mutation codecs carry the tri-state physical model using deterministic
JSON encoded as hex/length-prefixed bytes. Apply, inverse, absorb, between, and emptiness now use
`physical`, so mutation plus inverse restores both typed state and the exact physical ZIP model.

The existing exact-fixture law now checks the 211-member physical archive, direct export,
snapshot-model DSL/pack, BinarySnapshot bridge, self/no-op diffs, shape mutation canonical export,
mutation inverse exact reconstruction, diff inverse/absorb exact reconstruction, physical-only
diff codecs, and physical-bearing set-snapshot op codecs. The expected fixture remains 16,341,544
bytes with 62 slides and 78 relationship parts.

Read-only validation after this refactor:

- `rustfmt --check` parsed every edited ZIP/PPTX Rust file and reached formatting differences only;
- a direct read-only central-directory audit confirmed this fixture is classic ZIP (not ZIP64),
  has 211 entries, zero data-descriptor entries, zero prefix bytes, zero central trailer bytes, and
  zero bytes after the EOCD comment; the broader model still represents all of those variants;
- no Cargo or Nx command was run, per root coordination;
- one out-of-scope BCF `ZipSnapshot` literal was reported to root for adding `physical: None`.

## Historical ArtifactSource Design (Obsolete)

## Scope

Dedicated implementation for:

/Users/ueli/Documents/semio/temp/domai-specific-programmaning-language-for-architects.pptx

The fixture remains:

- 16,341,544 bytes;
- SHA-256 477900b1746139840890bc4edb653c488f3d18f9da34d231332b5db41d4caa8a;
- 211 ZIP entries;
- 62 slide parts;
- 78 relationship parts.

No shared OPC, ZIP, glue, script, project, launch, catalog, ticket JSON, or git-state file was edited by this lane.

## Implemented Contract

### Persisted source and derived cleanliness

PptxSnapshot and PptxArtifact now persist Option<crate::ArtifactSource>.

Import parses OPC and PresentationML first, constructs the typed snapshot, computes a deterministic source-free semantic projection, and captures the complete native PPTX archive bytes. Export recomputes that projection:

- matching imported projection returns the captured archive bytes verbatim;
- a changed imported projection returns PptxError::UnsupportedDirtyImport before presentation parts or ZIP bytes are regenerated;
- source-free authored snapshots retain the existing canonical authoring writer.

The projection is deterministic across HashMap reconstruction because relationship owners are sorted. OPC part payloads are represented by per-part BLAKE3 digests instead of JSON-expanding every binary byte, while path, content type, part order, content types, relationship order, and typed presentation state remain covered.

### State algebra and codecs

PptxDiff now contains tri-state source state:

- outer None: unchanged;
- Some(None): clear source;
- Some(Some(source)): set source.

Apply, inverse, absorb, between, and is_empty cover this state. Presentation-only mutation constructors intentionally leave it unchanged, so a typed edit makes the semantic projection differ while preserving the imported provenance required to reject destructive export. Mutation followed by inverse restores the original projection and therefore restores the exact-source export path.

The handcrafted codecs now retain source state:

- diff text codec uses source=[0] or source=[1,[bytesHex,semanticBlake3Hex]];
- diff binary codec reserves flag bit 2 and encodes source presence plus length-prefixed bytes;
- set-snapshot text codec extends the snapshot tuple with source;
- set-snapshot binary codec extends the snapshot payload with source presence and length-prefixed source bytes.

The mutations and diff grammar leaves were extended to describe these source forms.

### Native archive bridge versus ZipSnapshot

The format-local taxonomy leaf named artifacts/zip/2.0/any does not convert to or from ZipSnapshot. Its actual public boundary is:

PptxSnapshot ↔ BinarySnapshot

That current bridge is exact for unchanged imports:

- PPTX to BinarySnapshot calls the new exact-source encode_pptx;
- BinarySnapshot to PPTX calls decode_pptx, which captures the full binary payload;
- the exact fixture test covers both directions and asserts native bytes after the bridge.

There is no PptxSnapshot ↔ ZipSnapshot conversion in the format-local code. A future or indirect conversion that first materializes the current structural ZipSnapshot still cannot retain physical archive identity unless shared ZipSnapshot provenance is implemented. Its logical entries do not encode original compressed streams, local and central header bytes, padding/gaps, data descriptors, central-directory layout, or EOCD details. This lane did not edit shared ZIP state under its assigned ownership boundary.

The existing PPTX XML bridge remains a stub and is not a native PPTX roundtrip path.

## Existing Files Changed

Rust state and I/O:

- snapshot, artifact, diff, mutations, and outline Rust components;
- PPTX error, native import, and native export Rust components;
- existing BinarySnapshot bridge import/export components under artifacts/zip/2.0/any.

Schema and codec leaves:

- snapshot TypeScript, GraphQL, JSON Schema, and Protocol Buffer leaves;
- artifact TypeScript, GraphQL, JSON Schema, and Protocol Buffer leaves;
- diff TypeScript, GraphQL, JSON Schema, and Protocol Buffer leaves;
- diff and mutation grammar leaves.

No new test file was created. Tests were extended in the existing schema, diff, and mutation components.

## Acceptance Coverage Added

The exact fixture test covers:

1. direct native import and exact native export;
2. 211-entry ZIP integrity, 62 slides, 78 relationship parts, and internal relationship resolution;
3. ArtifactPack encode/decode followed by exact export;
4. ArtifactDsl print/parse followed by exact export;
5. BinarySnapshot bridge in both directions;
6. self-diff emptiness and apply;
7. no-mutation;
8. representative shape-position mutation and explicit dirty-import export rejection;
9. mutation inverse restoring snapshot and exact export;
10. diff inverse and absorbed forward/inverse restoring exact export;
11. source-only diff text and binary codecs;
12. exact source-bearing set-snapshot text and binary operation codecs.

Synthetic law tests additionally cover source field sweep, source set/clear, apply, inverse, absorb, between, emptiness, and codec roundtrips without depending on fixture size.

## Schema Audit

The new source state is represented consistently in the Rust artifact/snapshot/diff facets and is exposed in the existing TypeScript, GraphQL, JSON Schema, and Protocol Buffer leaves. The handcrafted text and binary source codecs have matching grammar/protocol boundaries.

Pre-existing PPTX non-Rust facet leaves remain broader legacy shells: several still describe entries or bytes_wire rather than the current Rust OpcPackage + PptxPresentation model, and mutation leaves do not enumerate every current typed mutation variant. This implementation did not invent a partial translation of the complete OPC/PresentationML type graph. A separate schema-regeneration wave is still required for full cross-language structural parity; the persisted exact source channel added here is present in each existing facet.

## Validation Performed

- zipinfo/stat/shasum read-only validation confirmed 211 entries, 62 slides, 78 relationship parts, 16,341,544 bytes, and the expected SHA-256.
- All edited JSON Schema files passed jq empty.
- git diff HEAD --check for the PPTX subtree produced no whitespace diagnostics.
- rustfmt --check parsed the edited Rust files and reported only formatting deltas against the repository's existing compact style; it produced no syntax error.

Per root coordination, this lane did not run Nx or Cargo. Central compilation and execution remain owned by the root after all concurrent artifact edits settle; no test-pass claim is made here.
