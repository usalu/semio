# GLB Writer JSON Semantic Matrix

Read-only source-derived matrix for current io/gltf.ts native()/jsonValue() → JSON.stringify → UTF8 behavior. The replacement should compare actual complete output against this existing endpoint before removing it. These are accepted schema-value semantics; malformed plain objects are a separate explicit policy decision. No production codec or native execution is claimed.

| Area | Neutral inputs | Required current behavior/check |
|---|---|---|
| ASCII escapes | quote, backslash, slash | quote/backslash escaped; slash remains literal |
| Control escapes | NUL, backspace, tab, LF, formfeed, CR, U+001F | short named escapes where JSON.stringify uses them; other controls lowercase hex Unicode escapes |
| Unicode | Latin/German, combining sequence, CJK, emoji | preserve code points and normalization; genuine UTF8, no BOM |
| Surrogate boundaries | valid pair split at every fragment boundary; isolated high/low; high followed by ordinary letter | paired scalar becomes four UTF8 bytes; lone code unit is escaped in JSON, never UTF8 replacement character |
| Other separators | U+2028/U+2029, DEL, U+FEFF inside string | literal string characters, not dropped as document BOM/whitespace |
| Empty values | empty name, false, zero, null | retained; falsiness is not omission |
| Generic empty arrays | children, weights, skins, animations, extensionsUsed, empty targets | omitted by native record filtering |
| Extras empty values | extras array/object, nested empty array/object | retained by jsonValue; generic empty-array omission must not leak into JSON-value namespace |
| Undefined fields | omitted optional record property; undefined array element | generic record member omitted; JSON array position is null if such malformed/optional input remains admitted |
| Attribute pairs | POSITION plus custom attributes, duplicate custom name | object members, final value wins; duplicate does not move a non-index key from its first insertion position |
| Morph target pairs | multiple target objects, duplicate names within target | same attributes-role conversion per target, not an array of pairs in emitted JSON |
| Extras object duplicates | same decoded key two/three times | last value wins; preserve first creation position for non-index keys |
| Object key order | keys 10,2,01,-1,4294967294,4294967295,x | array-index keys numeric ascending, then remaining strings by creation order; 01/-1/4294967295 are not array indices |
| Dangerous names | __proto__, constructor, prototype, toJSON as ordinary data keys | retain safe own data; no prototype mutation and no callable invocation |
| Namespace isolation | extras members named attributes, targets, bits, extras, extensions | treat as ordinary JSON member names; do not run glTF record-role dispatch inside extras |
| Exact ordinals | 0n, 9007199254740991n, 9007199254740992n, u64 max | safe range converts to numeric JSON; unsafe BigInt refuses gltf.index rather than rounding |
| Negative ordinal input | -1n | current native conversion accepts safe signed integers; whether role validation should additionally reject is an explicit malformed-source policy, not an assumed byte-preserving change |
| Binary64 zero | +0/-0 words in TRS, accessor bounds and extras number | JSON text is 0 for either; do not emit -0 or preserve a spelling that old stringify did not |
| Finite Binary64 | smallest subnormal, smallest normal, max finite, 0.1, values around 1e-6/1e21 | decode exact word then match ECMAScript number text, including exponent sign/threshold; no arbitrary decimal formatter |
| Nonfinite words | ±Infinity and NaN words | existing numeric/ifcNumber path refuses before JSON; do not silently emit null |
| Ordinary malformed nonfinite number | NaN/Infinity in a plain numeric record field | JSON.stringify itself maps to null; distinguish invalid schema input from finite word contract rather than accidentally changing accepted valid cases |
| Record member order | asset/scene/scenes/... custom permitted metadata order | preserve current own-property order if claiming byte equality; schema order/canonical sorted order is a deliberate output change |
| Defaults | empty versus omitted arrays, explicit numeric zero, empty strings, normalized:false | preserve old per-field inclusion; no blanket default-value omission |
| JSON length/padding | UTF8 byte lengths mod4 = 0/1/2/3, non-ASCII | JSON chunk length counts bytes, pads spaces; no giant padding fill; BIN pads zeros independently |
| Fragment limits | long string/key, deeply nested extras, many small records, duplicate at late prefix | all string/key/escape/UTF8/final-copy work remains original-control cooperative; no complete stringify/encode/set hidden behind a preflight |
| Cancellation | interior and completion of projection/string/UTF8/finalcopy | original signal/callback retained; no partial public GLB; caller source unchanged after completed capture |

Safe own-data construction must use an own-data definition mechanism for arbitrary keys. Replacing Object.fromEntries with ordinary assignment to {} changes __proto__ behavior. Replacing all objects with Maps and iterating insertion order changes numeric-key order. Duplicate replacement must not reorder non-index keys. Bounded numeric scalar formatting may use the existing finite number conversion; long strings/keys must not use JSON.stringify as a whole-fragment shortcut.

Primary references: [ECMAScript JSON serialization and QuoteJSONString](https://tc39.es/ecma262/multipage/structured-data.html#sec-json.stringify), [ordinary own-key ordering](https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinaryownpropertykeys), and [WHATWG TextEncoder](https://encoding.spec.whatwg.org/#interface-textencoder). These establish language/encoding semantics; the glTF-specific role/omission rows above are observations of the actual repository conversion, not invented normative glTF requirements.
