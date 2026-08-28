# Plugin Declaration Channel Codec Follow-up 48

## Scope

This source-only correction adds descriptor-schema coverage and raw JSON codec characterization for the three existing declaration-channel `SetValue` leaves. It does not change mutation behavior, the JSON text/binary implementation, Store/common compiled inputs, runtime code, or production leaves. No Cargo, rustc, or native test runner was used.

## Test-first evidence

The controller first required a schema-declared raw wire vector section before that section existed. The retained actual-file RED is [run-o5E1OU](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/🧫️run-o5E1OU/📓️result.md): `484/485`; only `wire vectors: schema-declared raw serde cases` failed.

The source packet then added five raw cases to both the ticket controller input and the actual mounted fixture, a fixture schema, actual native raw text/binary assertions, authoritative descriptor-schema validation, and a raw-property/token `jsonc-parser` reference. The retained Bun/Nx GREEN is [run-ff9nZl](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/🧫️run-ff9nZl/📓️result.md): `570/570`.

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-declaration-fixtures-43/📜️script.ts
```

## New source ownership

- Actual raw vectors: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️declaration-channels/🔣️cases.json`
- Actual vector schema: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️declaration-channels/🧬️schema/🔣️.json`
- Actual shared native consumer: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️declaration-channels/🧪️tests/🦀️.rs`
- Controller and duplicate neutral fixture: `🧪️plugin-declaration-fixtures-43/{📜️script.ts,🧫️vectors.json}`

The shared native helper runs every raw case through actual `OpText::parse_op` and `OpBinary::decode_op`, so each direct leaf's existing `source_json_codecs_and_i32_boundaries` test exercises the exact same raw strings.

## Characterized current codec contract

The frozen Plugin fixture macro supplies `serde_json::from_str(line)` and `serde_json::from_slice(bytes)`. Each owned leaf is `#[serde(deny_unknown_fields)]` with `value: i32`. The reference preserves property multiplicity and the raw numeric token with `jsonc-parser.parseTree`; it does not use parsed JavaScript-number equality to collapse the distinction.

The vectors therefore declare the current intended serde boundary:

- `1e0` is rejected for an `i32` field.
- `-0` is accepted and decodes to `0`.
- repeated nested `value` and repeated external `SetValue` are rejected.
- a valid value followed by a second JSON value is rejected.

The controller's i32 lexical model is explicitly tied to the actual `serde_json` codec call sites and leaf type/serde attributes. The mounted native tests are the pending runtime oracle for those exact expectations; the controller has not executed Rust.

## Descriptor and capture coverage

The controller now reads, hashes, compiles, and applies the authoritative draft-07 schema `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json` to all three actual descriptor files. It captures the controller before work, canonical-nofollow checks the workspace root and every input path, first-hashes every controller/consumer/schema/descriptor input, and rereads every input before writing the result. The final receipt lists the complete stable input map, including authoritative schema hash `db1c30ab7f19ab9a0f46539c71a427ba6ce51789c5c7904ea4d93dd9ea488aee`.

## Native status

The ten authored native tests remain unexecuted. The green result is source/schema/reference evidence only and does not claim compiled or runtime Plugin acceptance.
