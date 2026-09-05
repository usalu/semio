# Lossless Dynamic Pack Integer Repair Packet

## Decision

Repair the dynamic `Shape::Value` contract as one Rust/TypeScript/schema change.  The
current encoder converts every `DslValue::Number` through `as_f64`; it therefore cannot
round-trip general `u64`/`i64` values.  The repair is to emit and accept the already-defined
`TAG_UINT`/`TAG_INT` forms, and to give the browser a tagged `bigint` carrier.  Do not convert
integers back to `number`, and do not make JSON permissive of the new carrier.

This is a read-only packet.  No source was changed and no build or test was run.

## Exact defect and coherent wire change

[`Number`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:28) is already
the authoritative three-form numeric model: `UInt(u64)`, `Int(i64)`, and `Float(f64)`.
Its [`as_f64`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:34) docs
explicitly call the widening lossy past `2^53`.

The scalar grammar already owns `TAG_INT = 0x03` and `TAG_UINT = 0x04`
([`pack value`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:24)).
It uses them correctly for typed `FieldValue` at
[`:371-380`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:371),
but dynamic `DslValue` instead always writes `TAG_F64` from `as_f64` at
[`:529-561`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:529),
and only decodes `TAG_F64` at
[`pack value:1660-1700`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:1660).
The retained validator independently denies both integer tags in a DSL context at
[`pack value:983-997`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:983).

Land these inseparable Rust edits:

```text
encode_dsl_value(Number::UInt(v)) -> 0x04 + canonical unsigned LEB128(v)
encode_dsl_value(Number::Int(v))  -> 0x03 + canonical zig-zag LEB128(v)
encode_dsl_value(Number::Float(v))-> 0x05 + normalized little-endian f64

decode_dsl_value(0x04) -> Number::UInt(read_varint_u64())
decode_dsl_value(0x03) -> Number::Int(read_varint_i64())
decode_dsl_value(0x05) -> existing Float path

RetainedContext::Dsl accepts 0x03 and 0x04, retaining their existing
Integer/Unsigned scalar roles.
```

`read_varint_i64` already supplies the scalar grammar's zig-zag form.  Do not rewrite the
shared replication `number` varint helpers for this: its TypeScript
[`readVarintU64`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:256)
is intentionally for bounded framing/counts and computes in `number`.

`Number::PartialEq` considers fitting `UInt`/`Int` values equal
([`value`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:68)); that must
not weaken this law.  The wire tags still carry writer form, affect canonical bytes and hashes,
and must be asserted with `matches!`/exact bytes rather than only `assert_eq!` on `DslValue`.

## TypeScript representation and API

Bare `bigint` preserves magnitude but not the positive signed form: a Rust `Int(7)` and
`UInt(7)` would both become `7n`, so a later encode could silently choose a different canonical
tag.  Export a narrow, opaque `PackInteger` along with the dynamic AST:

```ts
export type PackInteger = Readonly<{ readonly kind: "int" | "uint"; readonly value: bigint }>;
export type PackValue = null | boolean | number | string | PackInteger
  | readonly PackValue[] | Readonly<Record<string, PackValue>>;
export function packInt(value: bigint): PackInteger;   // -2^63 <= value <= 2^63-1
export function packUInt(value: bigint): PackInteger;  // 0 <= value <= 2^64-1
export function isPackInteger(value: unknown): value is PackInteger;
```

The constructors must create a module-private branded/frozen carrier and the codec must accept
only that carrier as an integer.  A structural `{kind,value}` test alone would make an ordinary
dynamic map ambiguous.  If a structural-looking unbranded object has `value: bigint`, reject it
rather than silently encoding a map.  `packEncodeValue`, `packDecodeValue`, symbol collection,
and all public pack/base64/document functions in
[`os/🟦️.ts:1064-1379`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1064)
should take/return `PackValue`, not `unknown`.

Implement private Pack-local `read/write U64BigInt` and zig-zag helpers with the same ten-byte
overflow rule as Rust.  Keep offsets, symbol counts, string lengths, field counts and collection
allocation counts as bounded JS `number`s; `PackInteger` must never enter those positions.

The opaque carrier cannot be passed as a raw `structuredClone` value.  Add recursive
`clonePackValue` which reconstructs integer carriers, and use it instead of the three raw packed
descriptor clones at
[`registry:2528`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2528),
[`registry:2593`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2593),
and [`registry:2836`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2836).
Cross-worker/process dynamic data should remain Pack bytes, not a cloned decoded `PackValue`.

The retained scene generic parser is a production consumer, not an optional mirror.  Its token
union only permits a `number` scalar at
[`retained Pack value:8-13`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📦️pack/🧾️value/🟦️.ts:8),
and `#value` rejects tags `0x03`/`0x04` at
[`retained Pack value:158-164`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📦️pack/🧾️value/🟦️.ts:158).
Add `int`/`uint` tokens whose values are `bigint`; retain its eight-byte `#natural` for
collection/index bounds and add a distinct, ten-byte scalar parser.  Do not turn the scalar
back into `number` when rebuilding a dynamic Pack value.

## Consumers that need a boundary, not a cast

| Consumer | Required change |
| --- | --- |
| [`decodeMutationEnvelopesPack`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1373) and artifact byte parsers | Use one `isPackByteVector`: each item must be a finite, safe, integral `number` in `0..255`; reject `PackInteger` and floats. |
| Backbone worker request/response decode at [`:695-740`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:695) | Keep the decoded envelope as `PackValue` until each declared field is parsed.  Do not retain `as Record<string, unknown>` as a trust boundary. |
| Execution-target descriptor admission at [`backbone-worker:688-724`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:688) | `descriptorVersion !== 1` will reject a decoded Pack `UInt(1)`.  Require `asPackUInt32(descriptorVersion, "descriptorVersion") === 1`, while retaining the raw value for canonical re-encode. |
| Backbone event `sequenceNumber` at [`backbone-worker:1046`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1046) | Replace `Number(...)` and fallback with a required `asPackUIntSafe` check (or a schema-defined absent default); a received out-of-range integer must reject, not round or become an implicit sequence. |
| App-channel report/merge/conflict decoders at [`:2259-2295`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:2259) | Replace direct casts with report-specific parsers.  Each declared count/index/timestamp gets a range and exact safe-number narrowing; dynamic payload slots retain `PackValue`. |
| WGPU/plugin action boundaries (including `ActionWire.args` at [`:1351-1359`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1351)) | Change generic args/view state/history payloads to `PackValue`; action-specific decoders must accept only the documented `number` or appropriately ranged `PackInteger`, never `Number(value)`. |
| Native UI/actor numerical consumers | Reuse the existing exact pattern in [`shard-client:1765-1769`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1765): range-check bigint, then convert only within `Number.MAX_SAFE_INTEGER`.  Apply a signed/unsigned expectation per schema field. |

The descriptor pipeline is deliberately JSON-only and **must not** learn to accept `PackInteger`.
For the paired descriptor form, canonical-check/hash the raw `PackValue`, then project it through
`packValueToExactJson`: recursively permit only integer carriers whose `bigint` fits exactly in a
safe JS integer, yielding a number; reject the rest.  Use that projection only for JSON
schema/pair comparison and JSON writing.  The affected current sites are
[`verifyDescriptorPairBytesV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2507),
[`validateCatalogDescriptorValue`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2569),
[`finalizePluginDescriptor`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:314),
and trusted rotation [`hub script:4697-4704`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4697).
`isStrictJsonValue` is correctly strict today; do not modify it to accept `bigint` or a wrapper.

## Schema-first corpus and gates

Add one shared, bounded corpus:

```text
🧰️framework/🛍️products/💻️os/🧫️fixtures/🎒️pack-dynamic-integer-v1/
  🧬️schema/🔣️.json
  🔣️.json
```

JSON expresses integer alternatives as tagged decimal strings, never JSON numbers:

```json
{
  "schema": "semio.pack.dynamic-integer.v1",
  "version": 1,
  "accept": [
    {"id":"uint-safe-plus-one", "value":{"uint":"9007199254740993"}, "wireHex":"…"},
    {"id":"uint-max", "value":{"uint":"18446744073709551615"}, "wireHex":"…"},
    {"id":"int-positive", "value":{"int":"7"}, "wireHex":"…"},
    {"id":"int-min", "value":{"int":"-9223372036854775808"}, "wireHex":"…"},
    {"id":"float-two", "value":{"f64LeHex":"0000000000000040"}, "wireHex":"…"},
    {"id":"nested", "value":{"map":[["u",{"uint":"9007199254740993"}],["i",{"int":"-7"}],["f",{"f64LeHex":"0000000000000040"}]]}, "wireHex":"…"}
  ],
  "reject": ["truncated-u64", "u64-overflow", "nonminimal-u64", "nonminimal-zigzag-i64"]
}
```

Bound it to six accept rows, depth three, and 256 expected wire bytes per row.  The schema must
anchor decimal strings to the exact signed/unsigned ranges and require exact accept/reject keys.
The two nonminimal rows characterize raw decoder versus canonical boundary separately: existing
generic `read_varint_u64` permits some redundant encodings, while a caller that requires
canonical bytes must reject them by `encode(decode(bytes)) != bytes`.  Do not silently change that
shared acceptance rule inside this numeric repair.

Add these gates (selectors are proposed; none was run here):

```text
# Add a narrow registered native task in packages/rust/📜️script.ts and project.json:
bunx nx run @semio-tech/framework-os-kernel:pack-dynamic-integer-check -- --native
# exact native law run by that task:
pack_wire_value_preserves_integer_variants_at_u64_i64_boundaries

# Existing browser target and exact test scope:
bunx nx run @semio-tech/framework-os:test-quick -- --testNamePattern='@semio-tech/framework-os PackValueCodec'
```

The native law must assert tags and `Number` variants for all accepted rows, including positive
`Int(7)`, not only `DslValue` equality.  The browser law must compare exact bytes and `PackInteger`
kind/value, then prove nested int/uint values survive Pack decode/re-encode.  Extend the retained
scene test with an actual native packed field containing tags `0x03` and `0x04`; it must emit
bigint scalar tokens, close all owners, and reject malformed/truncated scalar varints.

The existing fixture comments name the non-existent old package
`semio-framework-os-kernel-store` at
[`store:26470`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26470).
The active package is `semio-framework-os-kernel`
([`Cargo.toml`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:2)); correct the comment in the same fixture update.

## Ordered implementation boundary

1. Add the schema corpus and native exact-variant law; it is RED against the present F64-only
   dynamic codec.
2. Change Rust immediate and retained dynamic grammar together.
3. Add `PackInteger`, local BigInt LEB128, and the focused browser byte corpus.
4. Upgrade retained scene decoding and byte/typed public boundaries before allowing browser
   component execution to claim dynamic Pack integer support.
5. Convert descriptor JSON sites through the exact safe projection and clone raw Pack values with
   `clonePackValue`; JSON remains a strict, safe-integer-only projection.

This has no compatibility path: one canonical dynamic Pack grammar is emitted and accepted by
both hosts after the coordinated landing.
