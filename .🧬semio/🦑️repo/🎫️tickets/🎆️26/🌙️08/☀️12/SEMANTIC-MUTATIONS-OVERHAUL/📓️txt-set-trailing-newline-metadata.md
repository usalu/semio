# TXT-SET-TRAILING-NEWLINE-METADATA-20

The direct leaf now owns canonical `🦀️.rs` and `🔣️.json`; component-named primaries were removed without fallback. It preserves serde, semantics, codecs, facets, and required surfaces while adding `dsl::MutationLeaf(contract = ::protocol)` and a serde descriptor/full relative-provenance/nonzero-token test.

The direct Nx CLI preflight was red before cutover with all ten canonicality assertions failing, then green after cutover with ten assertions and zero failures. `git diff --check` completed exit0. No Cargo command or shared mount write occurred.

Root-owned mount required:

```rust
#[path = "✏️set-trailing-newline/🦀️.rs"]
pub mod set_trailing_newline;
```

Run the exact filter `canonical_leaf_metadata_matches_descriptor_and_provenance` after mounting. Production scope is only `✏️set-trailing-newline/**`; ticket assets are under `🧪️txt-set-trailing-newline-metadata/`. No `compose/**` path was accessed.
