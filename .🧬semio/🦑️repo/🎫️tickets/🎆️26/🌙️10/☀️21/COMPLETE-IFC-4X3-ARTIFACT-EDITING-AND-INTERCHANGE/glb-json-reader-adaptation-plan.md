# GLB JSON Reader Adaptation Plan

Read-only source/primary-spec review, 2026-10-21. No production edits, new parser implementation or runtime proof.

## Existing boundaries

IFC ANY io/gltf.ts:98 captures original control and private GLB bytes. At110 it synchronously performs TextDecoder(fatal UTF8) → JSON.parse → parseGltfDocument. Chunk framing and private source capture should remain. JSON decode, parsing and schema projection currently contain no interior original checkpoint. The later BIN copy is already controlled; its at-most-three-byte padding check is bounded.

Existing public reusable decoder: `C:/git/semio/🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🟦️.ts:143 decodeJsonSyntax` and150 `decodeJsonSyntaxSegments`, exported by @semio-tech/pack-json-decode-ts. Its JsonSyntaxNode preserves raw numeric spelling and decoded UTF16 string code units. JsonMemberPolicy.Replace replaces the last decoded duplicate in the original first-slot position, using Map and no prototype lookup. Finite explicit maxBytes/maxNodes/maxDepth/chunk<=4096 and original callbacks are required. Root depth is1; every scalar child also increments depth. Nodes count syntactically parsed values including overwritten duplicate subtrees, not just retained results.

The glTF schema-owned reader is `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts`. It checks closed declared members, defaults absent arrays/scalars, converts safe nonnegative integral native numbers to bigint indices, and wraps finite native numbers as Binary64. It uses recursive map/Object.entries/flatMap; calling it once after cooperative syntax decode leaves a second synchronous whole-document frontier.

## Exact parity map

| Aspect | Current endpoint | Required adaptation |
|---|---|---|
| Raw source | Private original captured bytes | Borrow only private JSON chunk; never reread caller buffer after yields |
| UTF8 | Fatal TextDecoder; malformed/truncated/overlong/surrogate UTF8 throws | Streaming fatal TextDecoder on bounded private byte slices, final flush; preserve sequence across slices |
| BOM | Default ignoreBOM=false strips one initial UTF8 BOM | Keep one decoder for the chunk, not one per slice; preserve current initial BOM admission. Literal FEFF elsewhere is not JSON whitespace |
| Duplicates | JSON.parse last decoded name wins | Replace policy, including escaped-equivalent names; do not use Reject |
| Prototype names | JSON.parse creates own __proto__ data | Syntax records or null-prototype objects/safe defineProperty; never assignment into Object.prototype-backed object |
| Member order | Object.entries sorts canonical array-index keys first, then other strings in insertion order | Syntax members alone preserve source first-slot order, so direct GltfJson/map conversion must emulate integer-key enumeration order or explicitly change contract with tests |
| Numeric words | JSON.parse binary64 conversion; glTF role reader then finite/safe/integral tests | Convert final retained token with Number(text); preserve -0/underflow/rounding. Do not upgrade index acceptance to arbitrary exact BigInt merely because token is integral |
| Overflow | Finite glTF roles/extras refuse Infinity | Refuse final retained overflow; an overwritten overflow member must not cause semantic refusal if its last value is finite |
| Strings | Escaped unpaired surrogate code units preserved | Keep syntax decoded units; raw invalid UTF8 remains refused. No Unicode normalization |
| Defaults | Missing arrays/default scalars inserted by glTF member tables | Preserve missing versus explicit null/wrong-kind; extras/extensions are open JSON, schema records remain closed |
| Cancellation | Only outer GLB checkpoints today | Original cancel before/after every bounded decode/syntax/projection phase and completion; no partially returned snapshot |

Primary support: [Khronos glTF 2.0](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#glb-file-format-specification) requires first JSON chunk, aligned space padding and second BIN with zero padding. [ECMAScript JSON.parse](https://tc39.es/ecma262/multipage/structured-data.html#sec-json.parse) specifies duplicate overwrite and JSON-specific __proto__ behavior. [WHATWG TextDecoder](https://encoding.spec.whatwg.org/#interface-textdecoder) specifies fatal streaming decoding and initial BOM handling. Existing endpoint acceptance is not by itself a statement that every tolerated BOM/padding form is normative glTF.

## Feasible implementation

Use the existing decoder with Replace; do not create another JSON grammar. Feed bounded decoded text segments without joining the complete source. Map original IFC control once into the exact finite JsonSyntaxControl with io.cancelled/error translation. Count original raw bytes independently: the syntax reader's UTF8 recount excludes a stripped three-byte BOM and must not silently redefine the original byte ceiling. Apply exact original node/depth meanings explicitly; choose separate documented syntax and schema projection limits instead of assuming maxValues means every kind of count.

Preferred schema path: add cooperative syntax→typed glTF projection beside the existing glTF-owned wire tables, preserving declared role/default semantics and avoiding an intermediate generic object tree. Alternatively a bounded syntax→safe native value adapter can reuse the current parser only as an explicitly incomplete intermediate milestone: it does not eliminate the synchronous schema projection. Do not advertise fully cooperative receiving in that state.

The existing decoder itself has physical limits: #string builds character parts then joins whole string; #number source.slice and SegmentedJsonText.slice join a whole token; segmented constructor reduce and capture descriptor arrays are synchronous. Thus reusing it establishes scheduling of token consumption, not bounded final token assembly. Plan a shared decoder refinement if claiming bounded allocation/copy for long strings/numeric tokens. Also Number(very-long-token) remains a synchronous conversion requiring explicit token extent policy or documented scalar frontier. Do not silently truncate accepted numbers.

## Neutral tests and official fixtures

Capture old endpoint outputs/refusals before mount using actual Box GLB; compare typed snapshot words/defaults/extras order after mount. Cases: duplicate asset/nested/escaped names; __proto__/constructor/prototype; keys 0,00,01,2,4294967294,4294967295; last duplicate changes overflow→finite and wrong-type→valid; -0,1e0,1.0,small underflow,MAX_SAFE and above-safe index; long exponent; surrogate pair/lone escapes; UTF8 multibyte split at every boundary; BOM split in1/2/3byte slices; malformed/truncated UTF8; space/CR/LF/tab JSON trailing whitespace versus NUL; depth/node/byte exact and one-short; huge ignored duplicate subtree still bounded syntactically.

Add all new phases interior+completion cancellation under original control, source mutation after capture, and exact final no-partial-publication. Keep explicit framing failures for duplicate/misplaced chunks, unknown chunks before/after BIN, missing/URI buffer and padding. Use existing official Khronos Box, NegativeScale, interleaved and quantized fixture roster where admitted; run existing independent Three geometry comparison and Khronos validator on meaningful valid variants. JSON.parse/TextDecoder can be independent development oracles behind ticket test interfaces; no external runtime library dependency is needed.
