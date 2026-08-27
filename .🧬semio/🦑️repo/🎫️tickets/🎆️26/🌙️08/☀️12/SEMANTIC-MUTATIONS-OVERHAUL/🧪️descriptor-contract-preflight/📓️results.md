# Descriptor Binary-Tag Boundary Preflight

The coordinator tested the frozen JSON/Rust `u32` contract before changing the schema. Exact language-neutral inputs are in `🔣️vectors.json`; the replay harness is `📜️script.ts`.

Initial executed result: one mismatch across8 vectors. Ajv accepted4294967296 while the Rust compiler rejected that `Option<u32>` value. Transcript: `🧪️red.log`.

Correction: added `maximum: 4294967295` to the integer branch of `binaryTag` in the authoritative descriptor schema.

Final executed result:8/8 matching expected outcomes across Ajv, the repository's `validateJsonSchemaSubset`, and Rust's compiler. Null, zero and4294967295 passed; overflow, negative, fractional, string and omitted values failed as expected. Transcript: `🧪️green.log`; generated Rust sources/metadata/compiler diagnostics are retained in the printed fixture run directories.

This is a schema boundary check, not the full `MutationLeafDescriptor` implementation. That type, mandatory trait/derive/registration propagation and registered fixture integration remain open.
