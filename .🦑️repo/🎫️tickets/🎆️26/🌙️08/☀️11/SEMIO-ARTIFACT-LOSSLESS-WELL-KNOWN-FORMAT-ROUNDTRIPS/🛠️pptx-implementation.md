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

## Strict Structural Revision

The native-source replay design above was removed. PPTX now retains a `PptxPhysicalState` whose archive is a structured `ZipSnapshot`; ZIP retains decomposed local headers, compressed member payloads, optional descriptors, central headers, EOCD fields, prefix/gaps/trailer, and a semantic fingerprint. Exact unchanged export is emitted by the ordinary structural ZIP writer from those fields. No whole-PPTX/archive byte blob or unchanged-source byte bypass remains.

The PPTX semantic fingerprint covers schema, OPC state, logical XML parts, and presentation state. Unchanged imports take the structured physical writer path; dirty imports fall through to the existing canonical OPC materializer. Snapshot JSON DSL/pack, artifact conversion, sparse diff algebra, and set-snapshot text/binary operation codecs carry the physical state. The concurrent logical XML-node model remains intact.

Static `rustfmt --check` parsing completed without a syntax diagnostic. Nx/Cargo and fixture execution were intentionally left to the central integration run, so this revision makes no runtime-pass claim. ZIP64 physical capture currently rejects rather than canonicalizing silently; the supplied PPTX fixture is classic non-ZIP64.

## Governing Logical-Model Revision

The strict structural revision above is superseded. ZIP, PPTX, and MP4 no longer carry physical-layout, source-byte, lexical-token, semantic-fingerprint, or replay state in snapshots, artifacts, diffs, mutations, facets, or IO. PPTX persists logical OPC parts, parsed XML parts, binary part payloads, and the typed presentation model; export always traverses the normal OPC and ZIP writers. PPTX diff text and binary codecs now contain only OPC, presentation, and XML-part changes, and set-snapshot operation codecs contain only the logical snapshot fields.

ZIP retains entry order and its logical header semantics as first-class entry fields. MP4 retains its typed movie model and deterministic writer. A static forbidden-symbol audit across the three format trees returned no shadow-state symbols, and `rustfmt --check` parsed the edited PPTX Rust files without syntax errors. Nx/Cargo and fixture execution were not run in this lane, so no runtime-pass claim is made.

## Logical-Only ZIP and OPC Revision

The preceding sentence about retained ZIP order/header semantics is superseded. `ZipSnapshot` now contains only the archive semantic comment plus name-keyed entries of `name + decompressed data`. Import normalizes entries by name. Diff additions and mutations no longer contain an index. Compression method, flags, timestamps, versions, attributes, entry comment, extras, compressed bytes, local/central records, and imported order exist only as transient native parser/writer concepts.

OPC no longer carries an entry-order/header manifest. PPTX `opc.parts` contains genuine binary semantic assets only; every XML part, including presentation and slide parts, is represented by parsed `XmlDocument` state in `xml_parts`. Native export serializes this logical XML authority and then applies deterministic Office XML and ZIP materialization policy.

The TypeScript, GraphQL, JSON Schema, Protocol Buffer, text grammar, and binary protocol facets for ZIP snapshot/diff/mutations were aligned with this logical model. The Rust anti-shadow test includes all four cross-language facets for all three schema families and rejects physical setters and fields.

### Scoped Nx Evidence

Command:

```text
CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- __zip_compile_only__
```

Result after the logical ZIP/OPC and facet cleanup: exit 0, 0 tests run, 3,398 skipped, and `NX Successfully ran target test-long for project @semio-tech/stdio-plugin`.

The immediate PPTX exact-original command was:

```text
CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- fixture_survives_logical_io_persistence_diff_and_mutation_pipelines --nocapture
```

It did not reach the PPTX test because a concurrent DWG refactor made the shared crate fail with 64 foreign compile errors, beginning with removed `DwgSnapshot.sections` references at DWG IO line 637 and DWG diff line 145 and removed `section_names`/`sections`/`decode_status` constructor fields at DWG diff lines 175-177. Therefore no new PPTX expected/actual length or first differing byte was produced by this run. The last genuine pre-revision PPTX runtime evidence remains 16,802,890 actual versus 16,341,544 expected, first archive difference at byte 6, with `[Content_Types].xml` 13,111 actual logical bytes versus 11,733 expected; that evidence predates the logical XML authority change and must not be treated as current.

### PPTX Logical Authority Audit

Read-only fixture classification accounts for all 211 OPC members exactly once:

- 1 typed `[Content_Types].xml` table;
- 78 typed relationship-owner lists;
- 84 authoritative `XmlDocument` parts (`.xml` plus the VML drawing);
- 48 genuine semantic binary parts: PNG/JPEG/EMF media and OLE `.bin` embeddings.

The audit found one real leak: `ppt/drawings/vmlDrawing1.vml` had previously fallen through to `OpcPackage.parts` as opaque bytes because classification only recognized `.xml` and XML-suffixed content types. The classifier now recognizes VML path/content-type semantics, so import parses it into `PptxXmlPart` and rejects invalid UTF-8/XML instead of preserving opaque markup.

Native import sorts XML and binary parts by logical path; `PptxSnapshot::from_parts`, DSL/pack decode, and diff application normalize OPC binary parts, content-type tables, relationship lists, and XML parts by their logical keys. No native ZIP member order survives. ZIP local/central headers and decompression metadata remain decoder-local variables only.

Export now rejects duplicate XML paths, XML stored in `OpcPackage.parts`, and a path with both XML and binary authorities. The exact lifecycle test asserts VML XML authority, disjoint XML/binary paths, the 211-member authority sum, the narrow binary extension set, and the anti-bypass rejection. The PPTX snapshot/artifact/diff/mutation TS/GraphQL/JSON/Proto facets are all included in the anti-shadow test.

The typed `PptxPresentation` is a semantic projection of the authoritative presentation/slide XML. Export derives that projection again: unchanged imported projections serialize the authoritative XML documents, while a typed presentation mutation regenerates presentation-owned XML transiently. Applying the inverse restores equality with the XML projection and returns to exact authoritative serialization without any persisted fingerprint/source state.

The authority audit also explained the earlier `[Content_Types].xml` growth from 11,733 to 13,111 bytes: export had called `OpcPackage::set_part` for every logical XML document, which fabricated per-part overrides even when an existing `Default` already resolved the same content type. Export now materializes such XML parts into the transient OPC payload without changing the logical content-type table; an override is added only when resolution genuinely differs. PPTX logical normalization applies the deterministic PowerPoint content-type order visible in the fixture (alphabetic defaults, presentation/master/numeric slides, notes/properties/theme/table/numeric layouts, remaining theme/docProps) and numeric relationship-id order, never imported ZIP order.

### Post-Authority Exact Gate Attempt

After the DWG lane reported shared compilation coherent, the exact-original PPTX lifecycle command above was retried with the isolated ticket target. It again failed before executing the PPTX test, now on four foreign `E0560` errors in the AC1018 DWG structure-inference tests: stale `DwgSnapshot { bytes, section_names }` fields at lines 57-58 and 74-75. Nx exited 1 through Cargo 101. No current PPTX output length or first differing byte was produced. The PPTX/ZIP lane did not modify DWG-owned files.

### Logical Exactness and ZIP Materialization Evidence

Once the foreign DWG constructors were coherent, the exact-original lifecycle reached runtime. Removing numeric reordering of the typed OPC relationship lists fixed `_rels/.rels`: the fixture's typed order begins `rId3,rId2,rId1,rId4`, while the former normalization had emitted `rId1` first at logical byte 162. XML attribute control characters now use the standard `&#x9;`, `&#xA;`, and `&#xD;` forms; the deterministic PowerPoint extended-properties policy expands the empty `Template` element; and the VML policy deterministically emits Office's namespace line wrapping, shapelayout continuation, and single-quoted style attributes from the parsed XML tree. The resulting diagnostic reports `logical_mismatches=[]`: all 211 decompressed OPC members are byte-identical to the fixture without raw XML or archive state.

The remaining archive-only failure was 16,882,129 actual versus 16,341,544 expected under the internal fixed-Huffman encoder, then 16,034,857 under flate2/libz level 6. The original first `[Content_Types].xml` member is 11,733 logical bytes and 721 compressed bytes, method 8, flags 6. System zlib levels 1 through 9 produce 747, 741, 741, 719, 714, 706, 705, 693, and 693 bytes respectively. Exhausting 3,150 `deflateInit2` combinations across level 0-9, raw window bits 9-15, memory level 1-9, and default/filtered/Huffman/RLE/fixed strategies found no exact stream; six combinations have length 721 but differ at compressed byte zero.

The fixture stores all PNG/JPG/JPEG members with method 0 and flags 0, while XML, relationships, VML, EMF, and embedded BIN members use method 8 and flags 6. That extension-derived deterministic writer policy is now transient ZIP serializer behavior, not schema state. With that store policy, system zlib aggregate archive lengths for levels 1-9 are 16,357,334; 16,353,784; 16,352,069; 16,343,822; 16,340,741; 16,338,675; 16,338,188; 16,337,593; and 16,337,526 bytes.

The isolated Nx backend matrix covered miniz_oxide 0.8.9 levels 0-10, zlib-rs 0.6.3 levels 0-9, and zopfli 0.8.3. No direct FINISH stream is exact. Miniz level 2 has the exact 721-byte length but differs at byte zero; zlib-rs level 3 is 720 bytes; zopfli is 649 bytes. Miniz sync/full followed by finish is the closest structural family: levels 6-10 share the fixture's first 14 compressed bytes, but produce 697/699 bytes rather than 721.

Raw-DEFLATE `Z_BLOCK` tracing shows the fixture has three blocks: its data block consumes compressed bytes 0-714 and produces all 11,733 logical bytes with four pending bits and a non-final flag; an empty block ends at byte 719; and a final empty block ends at byte 721. This is consistent with a miniz-style deterministic compressor followed by sync/full and finish, but not with current stock backend defaults. A deterministic probe-count × greedy × filtered miniz grid is implemented in the existing deflate codec test; its Nx execution was paused when the DWG lane reported ten new shared compile errors.

### Office DEFLATE Tokenizer Identification

The complete deterministic miniz probe grid (probe count 1-4095, lazy/greedy, filtered/unfiltered, nondeterministic initialization excluded) found no exact stream. Its best prefix was 14 bytes at probe count 100 with a 697-byte result; 87 variants produced the expected 721-byte length but none matched.

Token-level decoding compared the fixture's dynamic-Huffman literal/length-distance sequence with miniz and zlib candidates on four logical XML members. Miniz first diverges by suppressing Office short matches: `[Content_Types].xml` output 649 uses length 4/distance 123, `presentation.xml` output 587 uses length 5/distance 126, and both sampled slides at output 344 use length 5/distance 15. Stock zlib level 4 reproduces all 696 tokens of `slide1.xml`, identifying the Office compressor as the zlib slow/lazy parser family.

An exhaustive `deflateTune` parameter grid established the XML policy: level 4, `good_length=1`, `max_lazy=4`, `nice_length=258`, `max_chain=1024`, `Z_SYNC_FLUSH`, then `Z_FINISH`. It reproduces both complete tokens and complete compressed bytes for `[Content_Types].xml` (525 tokens, 721 bytes), `presentation.xml` (1,382 tokens, 1,675 bytes), `slide1.xml` (696 tokens, 746 bytes), and `slide39.xml` (2,370 tokens, 2,962 bytes).

The same token method established a vector-media policy over all three EMFs: level 4, `good_length=4`, `max_lazy=4`, `nice_length=258`, `max_chain=4096`, sync then finish. It reproduces `image9.emf` 685/685 bytes, `image10.emf` 570/570, and `image11.emf` 556/556. Both policies are transient serializer choices behind the internal deflate interface; no token stream or compressed bytes are persisted.

After these policies, the full exact gate reached `oleObject1.bin`. All 211 logical payloads and all 211 member positions remain exact. OLE lineage differs: expected size is 768,040 bytes, equal to stock zlib level-7 finish-only output, but the first token divergence is token 63 at logical output 524, where Office emits length 4/distance 4 and zlib/miniz emit a literal. A focused finish-only lazy/token grid is ready but its execution was blocked by concurrent foreign DWG unresolved-symbol compile errors; no final archive pass is claimed.

## Final Exact Logical Lifecycle Result

This section supersedes the preceding blocked-state paragraph. The deterministic Office ZIP policy is complete: XML/default entries use tuned zlib level 4 with sync+finish; EMF uses the high-search level-4 profile; OLE `.bin` uses the compact mem-level-7 high-search profile; PNG/JPEG media are stored. Together with the semantic PowerPoint member ordering, local A220 growth hints, flags, versions, DOS epoch, and typed XML materialization policies, `encode_pptx(decode_pptx(fixture))` is byte-identical to the original 16,341,544-byte fixture without source, physical, compressed, lexical, or imported-order state.

The exact fixture test now covers direct native IO, structured DSL, binary pack, binary artifact bridge, native analyzer, native composer, self-diff, no-op mutation, mutation plus inverse, diff inverse/absorb, XML-parts text/binary diff, and set-snapshot text/binary operations. Every route compares its export directly with the original fixture bytes.

Final isolated Nx command:

```text
CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- fixture_survives_logical_io_persistence_diff_and_mutation_pipelines --nocapture
```

Result: exit 0; 1 passed, 0 failed, 3,377 skipped; test runtime 17.53 seconds; Nx reported success.

The ZIP/OPC lifecycle was also driven directly from the same native PPTX bytes. `ZipSnapshot` remained equal through structured DSL, pack, text/binary self-diff, text/binary set-snapshot operation, native analyzer, native composer, and decode-after-deterministic-rematerialization. `decode_opc(encode_opc(decode_opc(fixture)))` retained the complete logical package. The generic `encode_opc` documentation now defines its deterministic path-sorted logical normal form and delegates format-specific ordering to the shared semantic path-order policy rather than claiming imported-byte preservation.

ZIP/OPC isolated Nx command:

```text
CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- deterministic_logical_round_trip --nocapture
```

Result: exit 0; 1 passed, 0 failed, 3,377 skipped; test runtime 9.56 seconds; Nx reported success.
