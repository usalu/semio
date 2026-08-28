# Plugin Declaration Channel Fixture Review 48

## Scope and Evidence

Read-only review of the ticket packet `🧪️plugin-declaration-fixtures-43`: its report, controller, the three direct `SetValue` leaves, transparent aggregates, JSON schemas/descriptors, and the mounted native tests under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️declaration-channels`. No production, schema, controller, or Cargo change was made.

The controller was replayed without changing its inputs. It retained a source/schema/neutral GREEN at `484/484` in `🧪️plugin-declaration-fixtures-43/🧫️run-9wydRv/🔣️result.json`. That replay does not execute Rust code.

## Confirmed Direct Ownership

Each of `1️⃣standard-1/🌐️any`, `1️⃣standard-1/🔒️strict`, and `2️⃣standard-2/🌐️any` has a distinct direct `🧬️mutations/📝️set-value/🦀️.rs`, descriptor, payload schema, and one-newtype-variant aggregate. The leaves hold `i32`, emit their owner diff with `Some(value)`, and retain the pre-state explicit inverse. The aggregate serde representation is externally tagged `SetValue`, while the retained macro owns the JSON-text / UTF-8-JSON-binary codec implementation. No arithmetic can overflow in that replacement operation; the existing boundary vectors include both i32 endpoints and out-of-range/fractional payload negatives.

Current aggregate schema hashes observed during review:

| Owner | SHA-256 |
| --- | --- |
| Std1Any | `6101258ad53717c1322b4fa8b2eb81178f3846666f902b0c5de8a37ce465551a` |
| Std1Strict | `958a2707c40af56c303ee94cabc99a950e9f5404281d5b700046750c6f4b49f5` |
| Std2Any | `6489736cdd9a55ed8ad74dad215965c7b71b1162f960ce13c2ece5527e9981f3` |

All currently close their envelopes with `additionalProperties: false`; the test vector extra-envelope case is therefore covered by the current Ajv replay.

## Review Findings

1. The controller never reads or compiles the authoritative mutation-descriptor schema at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json`. It checks selected descriptor fields manually, but that is not a full descriptor-contract proof. The authored Rust test calls `L::DESCRIPTOR.validate()`, yet it remains unexecuted. Add authoritative-schema validation for all three descriptor JSON files before treating the neutral gate as descriptor-complete.

2. The wire vectors and authored Rust test cover malformed UTF-8 (`[0xff]`), empty bytes, malformed JSON, unknown fields, i32 bounds, and fractions. They do not cover raw JSON lexical or duplicate-key behavior: for example `{"SetValue":{"value":1e0}}`, `{"SetValue":{"value":-0}}`, repeated `value`, or repeated `SetValue`. Ajv receives parsed values and cannot establish serde_json's lexical/duplicate-key behavior. This is a missing codec-characterization test, not evidence that the current codec is wrong.

3. The neutral controller's third-party JSONC oracle uses parsed integer values and JSON-tree replacement. It adequately checks the simple ordered replacement diff, but it cannot establish actual serde codec behavior or derive-generated descriptor roster behavior; only the mounted Rust tests and serialized Plugin cfg(test) gate can prove those joins.

4. `Std1Strict` correctly leaves negative values admitted by the mutation leaf and moves non-negative enforcement to the existing IO conformance callback. The controller/natives cover that intended separation. No overflow or ownership defect was found in the direct leaf's diff/inverse semantics.

## Required Native Follow-up

Run the existing ten leaf/shared native tests under the serialized Plugin cfg(test) gate, then add raw text/binary lexical and duplicate-key characterization assertions with the behavior explicitly selected by the owned codec contract. No recommendation here changes the frozen source packet.
