# TXT Facade Parity Repair Proposal

## Decision

Freeze one canonical facade domain for the five existing TXT leaves. It is a transport
admission domain, not a document-state domain:

| Field | Canonical facade domain | State-dependent rule |
| --- | --- | --- |
| `index` | `u32`, `0..=4_294_967_295` | Insert clamps; remove/set can be no-ops; index validity is evaluated against the snapshot only after facade decode. |
| `value` for line ending | exactly `lf` or `crLf` | `crLf` still needs a visible separator; this remains a mutation invariant. |
| `value` for trailing newline | boolean | The snapshot may reject an invisible terminator change. |
| `text` | any Unicode string, including empty and newline-containing strings | `native_text_error` and `native_shape_error` decide whether that text can be represented in the current snapshot. |
| phase | exactly one of the existing five leaf kinds | There are no TXT witness fields. The phase is the aggregate branch/opcode/tag. |

The state checks must not move into GraphQL/protobuf/text/binary decoding: no payload
carries the target snapshot. All representations first construct the same canonical
mutation, then `diff`/`inverse` report the existing semantic error/no-op outcomes.

## Current Audit

Rust uses `usize` for `insert-line`, `remove-line`, and `set-line`. JSON Schema declares
only non-negative integers. TypeScript declares unrestricted `number`. GraphQL exposes
signed `Int!`; proto exposes `uint64`; text/binary frame a JSON serde payload. Thus no
surface currently has the same index domain. In particular, the current GraphQL domain
ends at signed 32-bit, while native Rust/text/binary can admit platform-width integers
and proto can encode `u64` values.

The line-ending JSON Schema and TypeScript literal union are exact, but GraphQL and proto
currently admit any string. All leaf JSON serde types deny unknown fields and require
their fields. Proto3 scalars lack required-field presence by default, and generated/native
protobuf decoding can accept out-of-range unsigned values by wrapping them or accepts
unknown/duplicate wire fields unless an owned boundary rejects them.

The root JSON aggregate already uses a disjoint `oneOf`; text opcodes and binary tags
already select one leaf. The GraphQL root instead permits an independent tag plus zero or
many optional branch fields. The proto root has a oneof but leaf scalar presence remains
ambiguous. Neither TXT payload has a witness shape; phase selection is the only union
constraint.

## Proposed Surface Contract

Keep all current canonical paths, owner names, variants, tags, and opcodes. Change only
payload domain declarations and their decoder boundaries.

- Rust: make the three public payload indices `u32`; convert with checked
  `usize::try_from(index)` only
  at line-vector access/clamping. This makes serde JSON, text, and binary reject negative,
  fractional, and above-`u32` indices before a mutation exists.
- JSON Schema: add `maximum: 4294967295` to all three index fields. Existing required
  fields, `additionalProperties: false`, and the two-value line-ending enum remain.
- TypeScript: preserve `number` at the type boundary but introduce no unsafe cast; emitted
  client values enter the same owned facade conversion, which verifies finite integral
  `u32`. Keep the literal line-ending union and required readonly fields.
- GraphQL: the aggregate declares the generic `TxtMutationUInt32V1` scalar; the
  production-owned primitive API performs the finite integral `u32`
  check for both variables and literals. Replace `String!` with
  `TxtLineEndingV1! { LF CR_LF }`, mapping explicitly to `lf`/`crLf`. Replace the root
  tag-plus-optionals form with `input TxtMutationInput @oneOf` containing exactly the five
  existing branch fields. `@oneOf` belongs only on this semantically disjoint root union.
- Proto: use `optional uint32 index`, `optional string text`, `optional bool value`, and
  `optional TxtLineEndingV1 value` as appropriate; define `TxtLineEndingV1` as exactly
  `LF = 0` and `CR_LF = 1`. Presence distinguishes missing from valid `LF`; there is no
  invented unspecified value. Preserve the existing root oneof/tags.
  Every leaf conversion requires all of its optional fields to be present.
- Text/binary: retain their existing opcode/tag framing and serde payload encoding. Their
  entrypoints call the same owned payload validator after serde decode, before yielding a
  `TxtMutation`; binary additionally keeps the current one-byte tag boundary.

## Repository-Owned Conversion Boundary

Reuse the existing `🧬️schema/🔨️modules/🧬️mutation-support` module for the small generic
primitive layer only: checked `u32`/`usize` conversion, GraphQL UInt32 variable/literal
coercion, strict Unicode scalar validation, and bounded protobuf wire primitives. The
aggregate remains a typed roster with mechanical dispatch; it has no per-mutation
behavior table. Each leaf owns its JSON/GraphQL/protobuf conversion and its direct
protobuf-byte decoder.

```rust
pub fn txt_u32_to_usize(value: u32) -> Result<usize, String>;
pub fn txt_usize_to_u32(value: usize) -> Result<u32, String>;
pub fn coerce_txt_graphql_u32_variable(value: f64) -> Result<u32, String>;
pub fn coerce_txt_graphql_u32_literal(kind: &str, value: &str) -> Result<u32, String>;
```

Leaf-owned protobuf byte decoders construct their canonical payload directly; no
production native/generated or third-party protobuf decoder participates. Each uses the
bounded support primitives and rejects unknown tags, duplicate singular fields,
malformed/non-minimal varints, truncated nested lengths, invalid UTF-8, and trailing
bytes. The GraphQL test adapter delegates only scalar coercion to the production-owned
primitive APIs and provides an `IntValue` AST kind; parser acceptance alone is recorded
separately from semantic decode. Leading BOM is preserved. JavaScript adapters reject
unpaired UTF-16 surrogates and raw wire strings require strict UTF-8.

## Precise Future Production Write Set

1. The three indexed leaf `🦀️.rs` files and their call sites that index vectors.
2. Their three payload `🧬️schema/🔣️.json`, `🟦️.ts`, `🔗️.graphql`, and `🛰️.proto` files.
3. `🔚️set-line-ending` GraphQL/proto and any generated facade types for the two-value enum.
4. Aggregate `🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, and `🛰️.proto` only for generic scalar
   declaration, typed roster, and mechanical union framing; no new mutation-root child.
5. Existing generic text/binary Rust decoders and their direct TXT tests only.

No STDIO glue, canonical mount paths, descriptors, taxonomy, Cargo metadata, framework
crates, or unrelated artifacts belong in this repair.

## Neutral Matrix And Evidence Separation

`🧪️txt-facade-parity/🛂️facade-parity.schema.json` is the schema-first neutral matrix
contract; `🧫️fixtures/🔣️facade-parity.json` contains 19 valid, malformed, range,
presence, phase, unknown-field, strict-proto, snapshot-dependent newline, leading-BOM,
and unpaired-surrogate vectors. The scoped Bun/Nx validation passed against current
source GraphQL/proto/JSON facets and leaf-owned TypeScript decoders:

```text
[DEBUG] TXT facade parity parser and owned-decode probes passed vectors=19
```

Each vector separates GraphQL syntax from scalar/input coercion, protobuf syntax from
protobufjs verification, repository-owned decode, text/binary decode, and snapshot state
validation. For example, protobufjs accepts `-1` and `4294967296` for a prospective
`uint32` during native verification/encoding; the owned decode must reject both. This is
evidence for the strict boundary, not a claim that native protobuf validation is enough.
The same oracle parses the planned GraphQL scalar/enum/root-`@oneOf` SDL and verifies its
five root branches. It does not claim that an SDL parser supplies the required UInt32
scalar resolver: scalar coercion remains an owned conversion test.
