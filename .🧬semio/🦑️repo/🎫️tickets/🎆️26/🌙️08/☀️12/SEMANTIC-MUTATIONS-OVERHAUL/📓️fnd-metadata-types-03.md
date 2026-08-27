# FND-METADATA-TYPES-03

## Scope

This packet adds only the complete static `MutationLeafDescriptor` contract: its exact five typed schema vocabularies, fourteen required serialized fields, non-default validation, facade reexports, neutral fixture/schema, and compiler-ready Rust test region. It does not change `MutationKind`, `CompositeMutationKind`, `SemanticDescriptor`, runtime `MutationDescriptor`, derive behavior, registry propagation, source ownership proof, or any artifact leaf.

## Contract

`MutationLeafDescriptor` serializes with `camelCase` to the existing fourteen-key language-neutral descriptor schema. Its nullable `textOpcode` and `binaryTag` remain present as `null`; it has no `Default`, omitted-field form, partial trait, or compatibility object. Validation checks the schema version, scalar patterns, nullable text opcode, unique non-empty outcome classes, and unique non-empty language surfaces containing Rust. Rust's `Option<u32>` makes a binary tag intrinsically limited to the schema's frozen `0..=4294967295` range.

The retained neutral fixture includes the complete descriptor, every enum wire vocabulary, and the eight promoted `binaryTag` boundary vectors. The ticket harness validates it with Ajv against the dedicated wrapper schema and authoritative descriptor schema, verifies fourteen public descriptor fields/five public enum types in the actual command source, and checks byte-identical promotion of the prior vectors. Passing `--cargo` to that harness invokes the registered kernel route `bun nx run @semio-tech/framework-os-kernel:test -- mutation_leaf_descriptor`; it was prepared but intentionally not run because the coordinator's Rust gate remains in progress.

Owner parity adds Unicode U+2028/U+2029 rejection and multiple-marker cases. Ajv accepts a later qualifying `/🧬️mutations/` marker even when an earlier one has an empty prefix, so the Rust helper scans every marker rather than using the first split. The helper remains runtime validation; a future derive-oriented const validator is intentionally not introduced in this type-only packet.

## Evidence

The executable harness is [metadata-types script](🧪️metadata-types/📜️script.ts) and its executed Bun-only transcript is [green log](🧪️fnd-metadata-types-03-green.log). The language-neutral assets are [fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-leaf-descriptor/🧫️fixtures/🔣️.json) and [schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️mutation-leaf-descriptor/🛂️schema.json).

No Cargo command was started in this packet.
