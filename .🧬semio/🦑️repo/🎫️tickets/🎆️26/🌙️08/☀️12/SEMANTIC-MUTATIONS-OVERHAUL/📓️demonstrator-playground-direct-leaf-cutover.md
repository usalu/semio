# Demonstrator Playground Direct Leaf Cutover

## Scope

- Root: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
- Semantic leaf: `✒️change-schema`
- Removed payload indirection: `✒️change-schema/🦠️mutation/{🦀️component.rs,🟦️component.ts}`

## Result

- `✒️change-schema/🦀️component.rs` is the authoritative payload and `protocol::MutationKind` behavior owner.
- The direct leaf owns its descriptor, payload schema, TypeScript, GraphQL, protobuf, text, and binary files.
- The root Rust, TypeScript, GraphQL, protobuf, JSON Schema, text, and binary surfaces visibly assemble the one semantic identity.
- Root text and binary files contain grammar/framing and direct-owner registries; per-operation matching and payload encoding live in the direct leaf.
- Glue and editor consumers mount or import `change_schema::ChangeSchema` directly.

## Verification

- Structural policy: 0 of 17 breach classes.
- Ajv: descriptor 1, payload schema 1, errors 0.
- Internal descriptor validation: covered by the structural policy query, errors 0.
- Nightly Rust AST parser: 8 files parsed, errors 0.
- Rustfmt: 8 direct/root Rust files clean.
- Ajv aggregate schema: committed mutation fixture accepted and invalid scalar payload rejected.
- TypeScript aggregate import and one-kind roster: clean.
- Scoped `git diff --check`: clean.
- Runtime compilation remains serialized behind the shared STDIO compiler gate.
