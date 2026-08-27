# glTF Node Name Facade Repair Contract

## Scope

This contract repairs the TypeScript, GraphQL, protobuf, and leaf-local Rust decoder facets for the parser-proven gaps in `✏️🔘️change-node-name`. It does not edit aggregate sources, shared traits, GraphQL server infrastructure, protobuf runtime, or mounts. The canonical JSON Schema and Rust operation remain the authority: apply is `{ node: u32, value: Option<String> }`; restore is `{ node: u32, before: Option<String>, after: Option<String> }`; all nullable fields are present even when null; exactly one operation phase is selected.

## Existing Conventions

- `✏️s/🔌️plugins/🎬️sequence/…/🧬️mutations/🔗️component.graphql` and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/…/🧬️mutations/🔗️component.graphql` use input `@oneOf` for a tagged operation union.
- glTF inference GraphQL facets declare named unsigned scalars such as `GltfHolesUInt64V1`; the leaf can therefore own a named `GltfChangeNodeNameUInt32V1` scalar without borrowing a signed GraphQL `Int`.
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/📡️transport/🦀️component.rs` validates discriminators, values, malformed integers, and trailing bytes at its actual decoder boundary. It demonstrates the required native-parser-plus-owned-validation split.
- The current Rust leaf already uses `deny_unknown_fields` and a required nullable serde deserializer. The repaired GraphQL/protobuf conversion must reach the same owned operation only after equivalent validation.

## Canonical GraphQL Facet

The replacement leaf SDL is:

```graphql
scalar GltfChangeNodeNameUInt32V1

input GltfChangeNodeNameOptionalStringV1 @oneOf {
  present: String
  absent: Boolean
}

input GltfChangeNodeNameApplyV1 {
  node: GltfChangeNodeNameUInt32V1!
  value: GltfChangeNodeNameOptionalStringV1!
}

input GltfChangeNodeNameRestoreV1 {
  node: GltfChangeNodeNameUInt32V1!
  before: GltfChangeNodeNameOptionalStringV1!
  after: GltfChangeNodeNameOptionalStringV1!
}

input GltfChangeNodeNameMutationInputV1 @oneOf {
  apply: GltfChangeNodeNameApplyV1
  restore: GltfChangeNodeNameRestoreV1
}
```

The branch name is the sole phase discriminator; the old independent `phase` enum is removed. The leaf exports `coerceGltfChangeNodeNameUInt32Variable` and `coerceGltfChangeNodeNameUInt32Literal`, both repository-owned primitive coercions with no GraphQL runtime dependency. The real scalar adapter calls those exports. Variables accept only a finite integral JavaScript number in `0..=4294967295`; literals additionally require `IntValue` and canonical non-negative integer text. They reject strings, floats, negative values, and values above the range.

`@oneOf` is useful native validation, but it is not enough to assert the operation contract: the repository-owned TypeScript `decodeGltfChangeNodeNameGraphql(unknown)` rejects an absent selected branch, both branches, unknown properties, a selected `present: null`, a carrier with no state, both carrier states, and `absent != true`. It returns only canonical `GltfChangeNodeNameMutation`, never a partially decoded facade object. The future GraphQL resolver calls this converter immediately after GraphQL input coercion and before mutation dispatch.

## Canonical Protobuf Facet

The replacement leaf proto keeps its existing package and message identity but changes only the local payload shape:

```proto
syntax = "proto3";
package stdio.gltf.mutation;

message GltfChangeNodeNameAbsentV1 {}

message GltfChangeNodeNameOptionalStringV1 {
  oneof state {
    string present = 1;
    GltfChangeNodeNameAbsentV1 absent = 2;
  }
}

message GltfChangeNodeNameApplyV1 {
  optional uint32 node = 1;
  GltfChangeNodeNameOptionalStringV1 value = 2;
}

message GltfChangeNodeNameRestoreV1 {
  optional uint32 node = 1;
  GltfChangeNodeNameOptionalStringV1 before = 2;
  GltfChangeNodeNameOptionalStringV1 after = 3;
}

message GltfChangeNodeNameMutationV1 {
  oneof phase {
    GltfChangeNodeNameApplyV1 apply = 1;
    GltfChangeNodeNameRestoreV1 restore = 2;
  }
}
```

The empty `AbsentV1` message makes absence a selected oneof branch, so there is no `absent: false` value. `optional uint32 node` preserves zero while allowing the conversion to distinguish omitted node from zero. Message-field presence and the `phase` oneof are required by the conversion even though proto3 decoding permits them to be absent.

The leaf-local TypeScript `decodeGltfChangeNodeNameProtobuf(Uint8Array)` runs a bounded strict protobuf field walker for this four-message tree and directly constructs canonical `GltfChangeNodeNameMutation`; it has no runtime parser dependency. It rejects truncated or non-minimal varints, invalid wire kinds, duplicate singular fields, repeated oneof selections, every unknown field tag, malformed nested-length boundary, invalid UTF-8, and trailing/overread bytes. It uses a fatal UTF-8 decoder configured to preserve an initial U+FEFF and validates UTF-16 pairs in object boundaries, rejecting isolated high or low surrogates. `decodeGltfChangeNodeNameProto(unknown)` separately applies the exact-presence and oneof conversion rules to a decoded-object boundary. Generated/native protobuf parsing is test-only because it can preserve or discard unknown fields and represents absent proto3 fields with defaults.

The strict walker and its `GltfChangeNodeNameDecodeError` are leaf-local, repository-owned TypeScript interfaces; they introduce no parser dependency or shared trait. A successful native decode without a successful strict walker is rejected.

## Required Validation Order

1. Parse GraphQL AST/coerce input or parse protobuf wire.
2. Reject malformed syntax/wire and unknown fields at that representation's decoder boundary.
3. Run the representation-specific exact-presence, oneof, scalar, and discriminator checks above.
4. Construct canonical TypeScript `GltfChangeNodeNameMutation`.
5. Construct the actual Rust `ChangeNodeNameMutation` from the leaf-owned `decode_gltf_change_node_name_graphql`, `decode_gltf_change_node_name_proto`, or `decode_gltf_change_node_name_protobuf` boundary. The object decoders accept only the repository-owned `dsl::DslValue`; its ordered `Object(Vec<(String, DslValue)>)` representation is checked for duplicate, unknown, and missing keys before conversion. Finite integral `DslValue::Number(f64)` values cover all `u32` values exactly. The Rust wire reader is equally strict and uses `String::from_utf8`, which preserves U+FEFF.
6. Reuse existing TypeScript or Rust validation for document-state checks (missing target, no-op, stale inverse). Facade conversion must not collapse those semantic outcomes into malformed-input errors.

## Law Matrix And Oracle Strategy

`🧪️gltf-node-name/🧫️fixtures/🔣️facade-boundary-vectors.json` and schema define decoded-object vectors; `protobuf-wire-vectors.json` and schema retain seven malformed raw protobuf cases. They cover six valid operations with zero/maximum u32, both null directions, and an initial U+FEFF; invalid negative/over-u32/missing node; missing/both/false optional states; missing restore witness; no/multiple phase; unknown field; isolated high/low surrogate; nonminimal tag; wrong wire; truncation; duplicate; and invalid UTF-8. The canonical JSON payload schema now separately rejects isolated UTF-16 surrogates while accepting paired astral characters. Every invalid vector fails before a canonical mutation exists.

`🧪️gltf-node-name/📜️script.ts` uses isolated ticket parsers at `🧪️language-surface-oracles/node_modules`: GraphQL 16.11.0 parses the repaired SDL and exercises the real scalar adapter through INT AST literals and finite integral variables only; protobufjs 7.5.4 parses the repaired proto and encodes permitted messages. The scoped Bun/Nx result in `🧪️gltf-node-name/🧪️facade-dslvalue-green.log` verified 16 canonical JSON-schema laws, six identical canonical facade outputs, and 19 malformed boundary rejections. The two stale direct imports exposed by the initial exact leaf import were corrected only in the leaf consumer closure: structure-geometry now references existing top-level support, and top-level references the actual schema snapshot. The retained scoped actual-source Rust run in `🧪️gltf-node-name/🧫️actual-source-run-dVvKUg` compiled the current full GLTF glue-derived aggregate roster (255 tests) against coherent genuine artifacts and ran each of the six direct node-name tests separately: all passed, with source and artifact SHA-256 fingerprints unchanged before and after. This is compiler/runtime evidence for current actual sources, distinct from root's registered Cargo selection gate.

Native parser acceptance is evidence only of grammar/coercion or protobuf structural decoding. It is not evidence of the required scalar range, `absent == true`, required proto3 presence, strict unknown-field rejection, or phase/payload coupling; those are the repository-owned conversion assertions above.
