# TXT-SET-LINE-METADATA-18

## Prepared Scope

The direct TXT set-line leaf now owns the canonical `🦀️.rs` and `🔣️.json` primaries. The former component-named source and descriptor were removed with no fallback. `SetLineMutation` preserves its serde, semantic, diff/inverse, text, binary, and existing behavior tests; it additionally derives `dsl::MutationLeaf` with `contract = ::protocol`. The new leaf-local test serializes the descriptor through `serde_json`, compares canonical JSON, asserts every relative provenance locator, and requires a nonzero workspace token.

The independent runtime agent released this production write after its actual-source runtime gate completed 30/30 tests with stable source fingerprints. No shared mount, codec, semantic behavior, or required language-surface file was changed here.

## Pre-Cutover Gate

```sh
bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️txt-set-line-metadata/📜️script.ts
```

The structural preflight was red before release because it required the absent canonical `🦀️.rs` and `🔣️.json`, removal of the component-named primaries, the public derive, and a provenance test.

The pre-cutover command completed exit1 with all ten deterministic canonicality assertions failing, as expected. The outer workspace router's post-cutover invocation remains blocked before the ticket script by an unrelated shared taxonomy validation error: `generatorContracts["wgpu-frame-worker"].outputRoots[5].path is also declared as an input`. It is not a feature result. The approved direct Nx CLI route bypasses that outer router only and completed the ticket fixture post-cutover with ten assertions and zero failures. `git diff --check` completed exit0. No Cargo command was run.

## Root Handoff

Root owns the shared mount and must apply:

```rust
#[path = "✏️set-line/🦀️.rs"]
pub mod set_line;
```

After the shared taxonomy gate is repaired, rerun the scoped preflight and the exact Rust filter `canonical_leaf_metadata_matches_descriptor_and_provenance`. Runtime behavior is root-owned.
