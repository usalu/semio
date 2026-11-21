# Record Model Source Admission

The descriptor emitter refused `semio-framework-dsl-record-model` at 265,731 bytes across 26 files. The same scanner reproduced that total for the production closure. The final admitted input in the captured log was the borrowed authored schema. The bytes that could be removed from the production closure belonged to three Rust receiving-projection tests declared inline in a production source file.

The existing `cfg(test)` declaration now points to a dedicated test file. All three test bodies stayed byte-identical after one-level dedenting. The production scanner excludes that file and admits 261,849 bytes across the same 26 files in both native macOS and wasm32 configurations. The 262,144-byte policy and all production logic and APIs stayed intact.

## Verification

- The repository's production source scanner passed both configurations after reproducing the prior failure.
- The pinned Tree-sitter Rust grammar parsed both Rust files with zero syntax errors. The extracted test body matched the prior body.
- All 12 scoped job failure fixture tests passed, including the source projection, independent nested allocation, and receipt checks over the shared receiving fixture.
- The adjacent DSL field receiving suite passed its three independent data checks. Its schema validation case failed against the existing schema and corpus. Neither was edited.
- Scoped whitespace checks passed.

## Adjacent Field Receiving Schema

The HEAD corpus validated against the live schema. Validating each live row separately isolated the two new cases at `🔣️.json` lines 29 and 35. Both use `kind: "value-type"`. The current test schema permits only `set`, `map`, `value`, `block`, and `statements`. The new Rust fixture at `🦀️.rs` line 95 explicitly dispatches `ValueType::receiving_cursor()` on both new rows. Their encodings exercise a nested list of schema references and the primitive integer type.

Pending changes also touch the DSL type binding and owned mapping implementations. The mismatch is part of the live ValueType receiving work and its remaining test scope includes a schema branch for the canonical ValueType tree and independent coverage of that branch. The peer corpus, schema, and Rust files were left as observed while the descriptor and renderer publications were running.
