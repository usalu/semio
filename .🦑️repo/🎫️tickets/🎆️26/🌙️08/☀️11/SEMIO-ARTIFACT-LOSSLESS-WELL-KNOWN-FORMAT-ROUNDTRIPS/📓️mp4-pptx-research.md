# MP4 and PPTX Exact-Roundtrip Discovery

## Scope and evidence

This is a read-only implementation and fixture audit for:

- `temp/bauen-mit-bestand.mp4`
- `temp/domai-specific-programmaning-language-for-architects.pptx`

No implementation or test file was edited. No supported file-path import/export command currently exists, so this track did not fabricate a temporary Rust harness and did not claim an executed artifact-level byte roundtrip. The conclusions below are direct consequences of the current encoder code and the exact fixtures' container structure.

Fixture baseline:

| Fixture | Bytes | SHA-256 | Observed structure |
| --- | ---: | --- | --- |
| MP4 | 16,086,051 | `54b0672cca68a474d44c6096abb6579160b4d33b0f637f588e2e0752373e05c7` | `ftyp(32)`, `moov(11272)`, `free(8)`, `mdat(16074739)` |
| PPTX | 16,341,544 | `477900b1746139840890bc4edb653c488f3d18f9da34d231332b5db41d4caa8a` | 211 ZIP entries, 62 slide parts, 44 media parts, 78 relationship parts |

`ffprobe` reports the MP4 as one H.264/`avc1` video track, 1200×1080, timescale 15360, 901 frames, duration 30.033333 seconds, and no audio. The same bytes already occur at `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4`.

The PPTX central directory has 211 members and Office-style physical ZIP layout. `zipinfo -v` reports 520 bytes preceding later entries in the inspected records. Those gaps/padding are outside the current logical `ZipSnapshot` and therefore cannot be reproduced by its normalizing writer.

## MP4 findings

### Current representation and laws

The current `Mp4Snapshot` is at:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:35-116`

It retains a typed `ftyp`, video tracks and sample payload/timing/sync information, codec configuration, plus raw payloads for untyped top-level boxes and non-video `trak` boxes. It does not retain the original complete ISO-BMFF box tree or original file bytes.

The decoder/encoder is:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️component.rs:77-87,296-325,578-623`

The module explicitly documents its contract as a fresh normal form rather than whole-file byte identity. `decode_mp4` discards `mdat` and typed `moov` wire bytes. `encode_mp4` always reconstructs `moov`, concatenates all samples into a new `mdat`, and emits `[ftyp, unknown, mdat, moov]`.

The exact fixture is ordered `[ftyp, moov, free, mdat]`, so byte equality is impossible before considering any field-level loss. The current exporter necessarily moves `moov` and `free` and reconstructs both `mdat` placement and every typed `moov` descendant.

### Information lost or normalized

- Original top-level and recursive box order, complete box bytes, 32-bit versus extended-size headers, and size-zero convention.
- `moov` auxiliary fields, including creation/modification timestamps, matrix, volume and handler name, which the source itself lists as intentionally unmodeled.
- Internal unknown `moov` children such as metadata/edit/user-data trees.
- Exact chunk topology and sample-table representation: `stsc`, `stco` versus `co64`, run partitioning, uniform `stsz`, omitted/present `ctts` and `stss`, and table version/flags.
- Original video sample-entry identity where decoded variants share one semantic codec; AVC output is rebuilt as `avc1`.
- Multiple `stsd` entries and other unmodeled sample-entry bytes.
- Original interleaving. The encoder serializes all samples track-by-track into one new media-data payload.
- Raw non-video tracks are retained as `trak` payloads but their source offsets are not rebased with corresponding media payloads. This exact fixture has no audio, but the mechanism is not generally sound for retained media tracks.
- Offset arithmetic is forced through `u32`, precluding a faithful general `co64`/large-file path.

### Existing tests and extension points

- I/O tests: `.../🚪️io/🦀️component.rs:652-739`. They test sniffing, synthetic snapshot equality, non-AVC retention, and `decode(encode(decode(real))) == decode(real)` for a roughly 43 KB fixture. They deliberately do not assert output bytes equal input bytes.
- Snapshot pack/DSL tests: `.../🧬️schema/📸️snapshot/🦀️component.rs:224-246`.
- Diff field sweep/inverse laws: `.../🧬️schema/🔺️diff/🦀️component.rs:365-395`.
- Mutation/diff/inverse/op-codec laws: `.../🧬️schema/🧬️mutations/🦀️component.rs:161-255`.

The cleanest place for exact-fixture I/O assertions is the existing `codec_tests` module in the MP4 I/O component. Snapshot/diff/mutation laws should be extended in their existing files rather than creating tests elsewhere.

## PPTX findings

### Current representation and export behavior

The current snapshot is:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:1-10,64-130`

It contains an `OpcPackage` and a typed semantic slide/shape view. Typed text boxes/placeholders retain text runs, basic bold/italic/font-size state, and transform. Pictures retain a relationship id and transform. Unrecognized shape kinds retain raw XML.

`OpcPackage` is at:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️component.rs:294-309`

It retains decompressed non-metadata part payloads plus parsed content types and relationships. It does not retain the original archive bytes, raw compressed streams, local/central headers, gaps, or raw `[Content_Types].xml`/`.rels` bytes.

The PPTX exporter is:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs:186-265`

Before writing ZIP bytes, `encode_pptx` unconditionally calls `regenerate_presentation_parts`. That function removes every slide part, every slide relationship set, and `ppt/presentation.xml`; then it recreates sequential slide filenames, ids and relationship ids from the lossy semantic view. Every slide receives only a synthetic slide-layout relationship.

This is semantic loss, not merely compression nondeterminism:

- Existing image/chart/hyperlink/external relationships are removed while `Picture.blip_rel_id` can still point at the removed id.
- Typed `p:sp` and `p:pic` nodes are serialized from the narrow view, losing unmodeled shape XML: nonvisual ids/names, geometry, fills, lines, effects, styles, paragraph/run properties, hyperlinks and extension data.
- Presentation-level information outside the master/slide-id list is discarded, including sizes, properties, notes-master references, default text styles and extension lists.
- Slide filenames, presentation relationship ids and slide ids are renumbered.
- Only shapes represented as `Other` have a raw XML channel.

OPC export then regenerates all metadata XML and builds fresh ZIP entries:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️component.rs:401-430`

Its documentation explicitly says byte identity is not expected. Every entry is freshly deflated; relationship owners are sorted; metadata XML is reserialized; archive comment is emptied; default ZIP entry metadata is used. The underlying `ZipEntry` models decompressed data and selected metadata, not raw compressed bytes or all physical layout:

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:55-117`

Even without PPTX regeneration, recompression, flag normalization, XML serialization, entry ordering, physical gaps and header choices prevent exact byte equality.

### Existing tests and extension points

- Existing PPTX I/O/schema tests are in `.../🧬️schema/🦀️component.rs:400-590`; they use synthetic/minimal packages, assert selected semantic or part retention, and test repeated regeneration order. They do not assert real input bytes equal output bytes.
- Mutation/diff/inverse/codec/field-sweep laws are in `.../🧬️schema/🧬️mutations/🦀️component.rs:791-1095`. `codec_retention_law` uses `build_minimal_pptx`, i.e. a package already produced by the normalizing exporter.
- Diff codec laws are in `.../🧬️schema/🔺️diff/🦀️component.rs:2360-2422`.
- OPC and ZIP codec tests remain relevant for the physical preservation layer, but exact PPTX assertions should be added to the existing PPTX schema/I/O test module and the current law modules.

## Required clean architecture

Exact no-op export must be a first-class invariant, not an accidental property of a canonicalizing encoder.

### Shared invariant

Every binary artifact snapshot needs an immutable wire representation with provenance and a semantic projection. Import records both. If neither the wire structure nor semantic projection has changed, export returns the original bytes verbatim. Diffs and mutations must cover the wire/provenance state or produce a deterministic dirty-part plan; otherwise snapshot equality can pass while exported bytes silently change.

### MP4

Retain a complete ordered recursive ISO-BMFF box representation, including original full box bytes/header form and all untyped children, alongside semantic track/sample views. Semantic entities should reference stable box/table paths. A no-op exports the original byte stream. A mutation rewrites only the affected boxes and recalculates dependent sizes/offset tables; untouched boxes remain byte-identical. This avoids storing mutually authoritative typed and raw states without a synchronization contract.

### PPTX/OPC/ZIP

Retain the entire original ZIP wire image or an equivalent structural snapshot containing raw local headers, raw compressed bytes, descriptors, physical gaps, central records and EOCD. Retain raw metadata XML and raw slide/presentation XML. Treat typed PPTX shapes as projections into those raw parts. A no-op returns the source archive. A semantic mutation patches only targeted XML/relationship parts and preserves every untouched member's bytes and physical metadata. OPC and presentation cannot remain independent authoritative states: current mutations can make them diverge, after which export silently chooses `presentation` and destroys OPC slide state.

For long-term consistency, use one canonical event-sourced structural state with derived semantic projections rather than a boolean `dirty` shortcut. An original-byte fast path is still valuable, but mutation/inverse must restore the exact original structural state and therefore the exact original export.

## Acceptance matrix for these fixtures

The permanent exhaustive suite should add all of the following for both exact fixture paths:

1. Import succeeds and records source length and SHA-256.
2. Immediate export satisfies `assert_eq!(exported, imported)`; hash and length are repeated in failure output.
3. Snapshot text/pack encoding and decoding preserves the wire state, then export remains byte-identical.
4. Empty diff and no-op mutation pipelines preserve exact bytes.
5. Every representative mutation followed by its inverse restores both snapshot equality and the exact original export bytes.
6. A meaningful mutation produces a valid artifact; import of that result matches the intended semantic state; a second no-op export is byte-identical to the first mutated export.
7. MP4 validation checks ISO-BMFF structure plus `ffprobe`-observable stream/timing/frame invariants.
8. PPTX validation checks ZIP integrity, unchanged untouched entry bytes/relationships/media, slide count and relationship resolvability.
9. Failure diagnostics identify the first differing byte, containing MP4 box or ZIP member/header, and before/after hashes.

Run the suite through the existing exhaustive Nx target, not ad-hoc Cargo invocation:

`bun nx run @semio-tech/stdio-plugin:test-exhaustive`

The project routes all four test targets through the required script:

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📋️project.json:7-39`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1-22`

At present `TestScript` ignores the `quick`/`long`/`exhaustive` segment and calls the same budgeted Cargo test command. If fixture-scale tests must be gated by tier, extend this existing `📜️script.ts`; do not add another script. `.vscode/launch.json` currently has only the stdio catalog gate (`⚖️gate🗄️stdio-catalog` at line 3021) and no stdio-plugin exact-roundtrip/test launch. Any new executable command must be registered there in the existing order, per repository policy.

## Baseline conclusion

Both exact fixtures necessarily fail the requested import→export byte equality under the current architecture. MP4 preserves codec substance but intentionally normalizes the container and changes this fixture's top-level order. PPTX first destroys and rebuilds modeled presentation parts, then normalizes the OPC metadata and ZIP container. Snapshot/diff/mutation laws currently prove semantic/model roundtrips, not wire-image roundtrips.
