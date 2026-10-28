# GLB JSON Writer Independent Audit

Read-only mounted TypeScript writer checkpoint: io/gltf-json.ts and writeIfcGltfGlb framing/copy integration. No new confirmed valid-schema semantic defect in the reviewed source. Reader JSON parsing remains a separate synchronous frontier. Native execution/gates remain unchanged.

## Exact old endpoint comparison

Before integration, gltf-json-endpoint-baseline.ts captured thirty-five actual old endpoint cases using the genuine loaded Khronos Box snapshot. After integration the unchanged script ran --compare: all thirty-five padded JSON strings, header/chunk fields, BIN SHA256 and complete GLB SHA256 match exactly. generated/gltf-json-endpoint-comparison.json reports zero mismatches. Baseline output remains immutable; it was not regenerated from the new writer.

Independent gltf-json-three-subset.ts reconstructed eleven meaningful old physical GLB variants from those exact JSON chunks plus original physical BIN, reparsed through the actual endpoint and emitted with the new writer, then loaded old/new physical GLBs independently with Three.GLTFLoader. All eleven full world-vertex/index signatures, mesh/vertex/triangle counts match. Cases include Unicode/escapes, numeric extrema metadata, dangerous/integer-key/namespace extras, attribute duplicate order, defaults and member insertion order. Three warns it ignores primitive numeric extras for the subnormal/max-finite cases; geometry evidence does not replace the separate exact JSON-byte evidence for that metadata. generated/gltf-json-three-subset.json retains results.

## Semantics and source control

The schema-aware conversion uses null-prototype objects plus defineProperty for arbitrary data keys, preserving Object.fromEntries last-wins behavior and ordinary numeric-key ordering without __proto__ mutation. Extras/extensions use their own closed JSON namespace, so ordinary members named attributes/targets/bits are not dispatched as glTF record roles. Generic record empty-array/undefined omission remains distinct from extras empty-array/object retention. Safe BigInt and Binary64 conversion remain exact before bounded scalar number formatting; malformed ordinary nonfinite numeric input now refuses explicitly rather than inheriting JSON.stringify null conversion, outside the valid-schema baseline contract.

Quoted output matches the inspected JSON.stringify rules: controls, quote/backslash, lowercase Unicode escaping for lone surrogates, intact valid surrogate pairs and literal other Unicode. A long string/key fragment contains at most max(batch,6) UTF16 code units (six for one escape; intact pair may exceed batch=1 by one unit), and UTF8 encoding runs only on that bounded fragment. Finite numeric scalar formatting is intrinsically bounded. No complete native map/stringify/JSON text/TextEncoder document copy remains.

Conversion advances one node/member step at a time under original captured limits; encode and final JSON-byte copy use captured control and completion checkpoints. Source is already privately admitted before entry. The public helper itself is a projection/codec over caller-supplied document data, not an independent capture API; actual endpoint private-source ownership is the reviewed guarantee. Cancellation rejects before public GLB return, and final JSON/BIN copies check original control. JSON space padding is at most three bytes; BIN allocation zero initialization supplies independent zero padding.

## Remaining physical and limit boundaries

Conversion builds a complete second schema-aware native-value tree cooperatively; this is not a zero-copy projection or native original-wallet implementation. JavaScript property enumeration and output-array allocation remain host primitives without native capacity receipts. Ordinary emitted fragments are bounded; value/member/string work limits are distinct, and maxValues now also constrains total framed GLB output, stricter than the previous binary-only egress check. Reader decode/JSON.parse/typed projection is unchanged.

One source-level limit consideration was sent to root: encoder's interim JSON ceiling is maxValues. If a caller explicitly sets a ceiling above unsigned32, JSON fragments could accumulate beyond a frameable GLB before final framing refusal. The default ten-million ceiling is far below that limit. No multi-gigabyte probe was performed; this is a conservative early-admission recommendation rather than an executed overflow counterexample.

Parent owns neutral cancellation/strict/full-suite results for the mounted writer. This auditor ran only the immutable thirty-five-case comparison and independent eleven-case Three subset. No production source changes, synthetic native run or renderer lifecycle approval is included.

## Framing-cap follow-up

Root corrected the interim admission observation: encodeIfcGltfJson now requires an explicit finite maximumBytes no greater than unsigned32, and the endpoint computes aligned JSON capacity from min(original maxValues,unsigned32) minus the 28-byte framing and aligned BIN before invoking the codec. Parent added an actual narrow 64-byte JSON framing capacity case under a Number.MAX_SAFE_INTEGER caller ceiling. This closes the reported early-admission recommendation in source; native execution remains unchanged.
