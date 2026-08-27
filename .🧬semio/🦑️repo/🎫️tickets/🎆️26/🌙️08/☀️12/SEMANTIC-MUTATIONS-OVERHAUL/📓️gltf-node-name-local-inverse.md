# glTF Node Name Local Inverse

## Boundary

Only ✏️🔘️change-node-name and its approved glTF TypeScript, protobuf, and JSON-schema aggregate references changed. Root updated the single shared Rust mount at ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:2472. The Rust aggregate is still only a reexport and enum variant. Aggregate text and binary codecs remain unmodified test consumers.

## Contract

The public mutation envelope is phase apply or restore. Apply carries node u32 and value string-or-null; restore carries node u32 plus before and after string-or-null. Every nullable payload field uses a required serde deserializer, so an omitted property is rejected while an explicit JSON null is retained as Option None.

The restore guard requires the current node name to equal after, rejects an equal before/after no-op, and writes only before. Its inverse is another guarded restore with before and after swapped. No public variant can transport GltfDiff. The internal outcome delta remains GltfDiff.

Public node identity is u32; conversion to usize is checked before the existing bounds check. Targets derive as document/nodes/{node}/name.

## Canonical Surfaces

The leaf now owns canonical 🦀️.rs, 🟦️.ts, 🔗️.graphql, 🛰️.proto, 🔣️.json, and 🧬️schema/🔣️.json. The previous component-named primaries and old payload schema were removed. The descriptor retains the existing aggregate framing ownership: textOpcode and binaryTag are null.

## Evidence

- 🧪️gltf-node-name/🧪️schema-matrix-final.log: scoped Bun/Nx plus Ajv 2020 completed 13 language-neutral wire laws. It independently applies the name-value reference rules, including guarded restore redo, and verifies apply/restore envelopes, both nullable directions, no-op/missing/stale semantic vectors, omitted witness fields, wrong field types, and the uint32 maximum.
- 🧪️gltf-node-name/🧪️rustfmt-green-final.log: current canonical Rust source parses and formats cleanly.
- git diff --check completed after the post-review source change.

The first registered STDIO compilation later exposed three test-only `FaultCode` versus `&str` comparisons and an incorrectly sequenced redo witness. They are corrected in the leaf test: codes compare as `protocol::FaultCode`, redo derives from the guarded restore against its pre-restore Pivot state, asserts a swapped guarded restore, and then applies against restored Root. `🧪️gltf-node-name/🧪️rustfmt-redo-green.log` retains the formatting check; registered Rust runtime is pending root's retry and is not claimed here.

The first neutral red capture was produced before the cutover and showed Restore(GltfDiff), but its ticket fixture directory was removed externally during the root mount handoff. It is not presented as a retained file. The recreated retained fixture is the current schema-first law source.

No genuine stdio rlib/rmeta pair was available at the time of this report, so no standalone actual-source Rust compiler/runtime result is claimed. The current fresh kernel/schema artifacts are insufficient to compile an unmocked glTF client; root will provide the stdio pair or run the registered test binary.

## Facade Contract Follow-Up

`📓️gltf-node-name-facade-contract-audit.md` records the pre-runtime cross-language representation review. Root later supplied isolated test-only GraphQL 16.11.0 and protobufjs 7.5.4 oracles. `🧪️gltf-node-name/🧪️facade-parser-green.log` retained the scoped Bun/Nx run: 13 core Ajv laws and 11 actual GraphQL AST/coercion plus protobuf parsing/verification vectors completed, exposing nine current representation gaps. The earlier `facade-contract-matrix.log` was a pre-oracle inferred matrix and is retained only as chronological evidence; parser results supersede it. Production GraphQL/protobuf facets were held unchanged.

## Runtime Filter

Once the registered stdio test binary is available, run the direct_leaf_tests module under change_node_name. The direct test block covers descriptor/provenance, JSON required-nullable decoding, absent-name normalization, no-op/missing/stale classification, guarded inverse/undo/redo, node-specific targets, and actual aggregate text and binary codec round trips.
