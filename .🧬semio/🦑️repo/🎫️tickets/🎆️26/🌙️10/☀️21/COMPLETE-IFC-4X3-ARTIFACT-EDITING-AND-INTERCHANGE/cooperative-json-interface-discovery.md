# Cooperative JSON Interface Discovery

Read-only targeted discovery for the GLB JSON follow-up. No implementation or framework/build changes were performed.

## Reusable native interfaces

- [ArtifactCanonicalJsonTree and cursor](C:/git/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs): trait at line 13 exposes scalar/array/object nodes and original child/key ordinals; cursor at 426 exposes constructor_demand/admit/next_demand/advance(output slice, grant)/begin_close/close_step. It retains real root-backed projections and a paged frontier rather than materializing another JSON tree.
- [ArtifactCanonicalJsonTextCursor](C:/git/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🔤️text/🦀️.rs): borrowed/native UTF8 text roles, exact escape-byte helpers and bounded output-slice advance, with original demand/close interfaces. Neutral escaped-text fixtures already compare JSON/UTF8 chunk boundaries.
- [Borrowed encode bridge](C:/git/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🫳️borrowed/🦀️.rs): write_json_source_into takes JsonWriteSource, maximum bytes/depth/items, an output callback and NativeEncodeControl. Existing JsonWriteCursor/borrowed writer tests exercise source identity, cancellation and retirement. This is an existing source projection route, not a reason to build a DSL intermediate first.

These are native interfaces; none is a directly callable production TypeScript encoder. Native source/test existence does not establish executed runtime proof in this task.

## Reusable TypeScript reader

[decodeJsonSyntax and decodeJsonSyntaxSegments](C:/git/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🟦️.ts:143) are production cooperative readers, exported by @semio-tech/pack-json-decode-ts. JsonSyntaxControl supplies explicit maximumBytes/maximumNodes/maximumDepth/chunk, cancellation, progress and async yield. The syntax tree retains number spelling and decoded member names; JsonMemberPolicy.Replace from the existing members package is the candidate for current JSON.parse last-wins semantics. Segmented input avoids joining the complete raw document. Control capture validates own data members and preserves original callbacks.

This is relevant to the reader frontier, not an encoder. Reusing it still requires bounded UTF8 byte-to-text segmentation and a cooperative typed glTF syntax projection; calling parseGltfDocument synchronously on a fully reconstructed object would preserve another whole-tree pass. Verify exact Unicode/BOM behavior, duplicate decoded-key replacement, numeric-role and default behavior against existing physical glTF tests before replacing the current ingress.

## TypeScript writer availability and recommendation

Targeted searches under framework pack/json and stdio runtime found no production TypeScript counterpart of the native tree/text encoder. The pack-json-ts package currently exports members and testing fixtures only; TS files under encode are tests/reference oracles, not a reusable runtime writer. Do not import those tests as production or claim a codec exists because a native cursor has a TS oracle.

The long-term shared owner is the existing domain-neutral pack/json encode tree/text taxonomy: add the TS implementation beside the native contracts and neutral escaped-text/tree fixtures, with a source-node/key/child ordinal interface and bounded UTF8 output. The IFC/glTF extension should provide a schema-owned projection for named records, Binary64 words, integer indices, attribute pairs, extras/extensions and omissions, retaining complete private source until completion. Avoid native(document), complete object/key arrays, whole stringify/encode and a flattened DSL copy. Original IfcIoControl can be bridged to the shared cooperative interface without external runtime dependencies.

Canonical native key ordering may differ from existing JSON.stringify insertion order. GLB JSON objects are semantically unordered, but existing byte expectations, duplicate replacement and generated output provenance must be tested explicitly; do not assume byte-identical output. Keep this interface addition separate from native wallet/gate claims and the already corrected export materialization loops.

## Decoder adaptation details

Source inspection confirms root depth starts at one, and all child values (including scalars) increment depth. Numeric syntax remains raw text; finite Binary64 and role-specific safe unsigned integer validation belong to the glTF projection. Whitespace excludes U+FEFF: raw BOM text refuses, while the current TextDecoder default strips an initial UTF8 BOM before JSON.parse; a byte adapter must preserve the intended existing policy explicitly. Raw unpaired surrogates reject invalid-unicode, whereas escaped Unicode code units are retained via fromCharCode, including escaped unpaired surrogates as JSON.parse currently allows.

The existing reader cooperatively scans characters but still assembles a complete string via parts.join and a number via source.slice, and SegmentedJsonText computes its extent via reduce. These are whole host primitives, so reuse alone does not prove bounded final string assembly. The raw byte/node/depth ceilings provide finite input admission; stronger physical/copy claims require further shared-reader work rather than a new private IFC parser.
