# Artifact Replay Numeric Shape and Precision Audit

## Finding

`g3OtOQ` exposed a real dynamic-value codec behavior, not a WAL replay defect.  The new
restart assertion is correctly scoped: it proves that replay preserves the *already-normalized
live query value* and complete frontier.  It does not prove that the submitted JSON integer was
preserved by the schema-less wire codec.

`DB_PATHMAP_SCHEMA` reaches the generic `Shape::Value` bridge on every write and read:

1. [`db/🗿️artifact/🦀️.rs:155`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:155) converts the JSON pathmap to `DslValue`; [`:165`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:165) turns it back into JSON.
2. [`:137`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:137)-[`144`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:144) call `pack_rt::encode_wire_value` / `decode_wire_value`.
3. The bridge is explicitly a one-field `Shape::Value` record at [`store/🦀️.rs:4958`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4958)-[`4999`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4999).

Ingress itself is exact.  `DslValue::from(&serde_json::Value)` chooses `UInt`, then `Int`, then
`Float` at [`🌱️value/🦀️.rs:247`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:247)-[`259`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:259), and the reverse JSON bridge writes those integer variants without widening at [`:218`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:218)-[`225`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:225).  The loss is solely in the dynamic pack shape.

## Exact codec behavior

`DslValue::Number` has distinct `UInt(u64)`, `Int(i64)`, and `Float(f64)` variants
([`🌱️value/🦀️.rs:28`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:28)-[`64`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:64)); its `as_f64` documentation expressly says that widening 64-bit integers is lossy beyond
`2^53` ([`:35`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:35)-[`40`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:40)).

Despite the scalar pack grammar having `TAG_INT` (`0x03`) and `TAG_UINT` (`0x04`)
([`pack/🌱️value/🦀️.rs:21`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:21)-[`26`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:26)), the `Shape::Value` arm unconditionally emits `TAG_F64` from `n.as_f64()`
([`:531`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:531)-[`539`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:539)) and its decoder always creates `Number::Float`
([`:1664`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:1664)-[`1671`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:1664)).  Its retained grammar also rejects `TAG_INT`/`TAG_UINT` when the context is `Dsl`
([`:983`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:983)-[`997`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:987)).

Therefore:

- `2` becomes a decoded JSON `2.0`: a deliberate *representation normalization* of safe integers, explaining the failed hard-coded assertion.
- The encoding is not injective for general `u64`/`i64`: `9007199254740993_u64` becomes the `f64` value `9007199254740992.0`; `-9007199254740993_i64` analogously rounds to `-9007199254740992.0`.  `u64::MAX` can round to `2^64`, which is not even an unsigned 64-bit value.  Some larger powers/multiples remain representable, but the map is still non-injective and cannot guarantee exact 64-bit preservation.
- This is intentionally frozen current protocol behavior, not an accidental test conversion: the value test generator documents `TAG_F64`-only dynamic values pending a matching TS `PackValueCodec` update at [`pack/🧪️testkit/🦀️.rs:307`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️testkit/🦀️.rs:307)-[`312`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️testkit/🦀️.rs:312).  The browser codec deliberately exposes only the same subset and writes every JS number as F64 at [`🟦️.ts:1087`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1087)-[`1095`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1087), [`:1205`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1205)-[`1208`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1208).

This affects production-shaped schema-less payloads (pathmaps, UI tree diffs, host effects,
events, and manifests), not only the `g3OtOQ` fixture: the TS bridge describes precisely those
uses at [`🟦️.ts:1064`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1064)-[`1089`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1089).

## Assessment of the revised replay law

The current law now takes a live query result, checks semantic equivalence to literal `2`, captures
that value/frontier before owner retirement, and requires exact value/frontier equality after open
([`db/🗿️artifact/🦀️.rs:4473`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4473)-[`4502`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4502)).  That is the right durable-replay assertion; it neither hides a restart drift nor incorrectly makes a lexical `2` requirement of the F64 wire.

`json_values_equal` is deliberately only a comparison boundary.  It converts exact, fractionless
floats with absolute value **strictly below** `2^53` to integral variants
([`store/🦀️.rs:5064`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:5064)-[`5089`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:5077)); it cannot restore the omitted bit at `2^53 + 1`, and appropriately does not declare that unsafe result equal.

One test-quality gap remains: `dsl_value_numeric_insensitive_eq` says it is numeric-insensitive but
delegates number comparison to `Number::PartialEq` ([`store/🦀️.rs:26492`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26492)-[`26498`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26492)).  That equality has no `UInt`/`Float` or `Int`/`Float` arm ([`🌱️value/🦀️.rs:68`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:68)-[`76`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs:76)), although the corpus supplies integer values ([`store/🦀️.rs:26468`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26468)-[`26539`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:26468)).  Static inspection therefore shows that it is not an exact or a semantic numeric assertion for this codec; do not use its current success/failure as a 64-bit-fidelity signal.

## Bounded next packet

Keep the revised replay law.  Add a separate native codec corpus before any schema-less value is
advertised as 64-bit-safe:

1. `UInt(2)` and `Int(-2)` characterize the current F64 normal form, and prove that replay retains that form exactly.
2. `UInt(2^53)` and `UInt(2^53 + 1)`, plus `Int(-(2^53 + 1))`, require input-to-decoded equality.  The second and third are expected RED on the current contract and make the loss explicit rather than treating it as replay behavior.
3. `u64::MAX`, `i64::MAX`, `i64::MIN`, and adjacent extrema test no aliasing/cross-sign conversion through `encode_pathmap_json` -> `decode_pathmap_json`, then through submit/query/open.
4. A `json_values_equal` boundary law proves it accepts `2`/`2.0` but rejects `2^53 + 1` versus its rounded float.

The only correctness repair is a coordinated protocol change: encode dynamic `Number::UInt` as
`TAG_UINT` and `Number::Int` as `TAG_INT`; decode those tags back to their exact variants; admit
them in the retained `Dsl` grammar; and extend the TS codec with an explicit lossless integer
representation (not JS `number`) plus byte fixtures.  Merely calling the existing renormalizer
after decode repairs the safe `2` presentation but cannot recover values beyond the F64 exact
range.

No build was run and no product source was edited for this audit.
