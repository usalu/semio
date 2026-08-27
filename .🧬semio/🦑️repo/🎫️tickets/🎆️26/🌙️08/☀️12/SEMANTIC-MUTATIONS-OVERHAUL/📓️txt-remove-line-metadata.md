# TXT-REMOVE-LINE-METADATA-17

## Implemented Scope

The direct TXT remove-line leaf now owns only its canonical mutation-primary files: `🦀️.rs` and `🔣️.json`. The former `🦀️component.rs` and `🔣️component.json` were removed with no fallback or alias. `RemoveLineMutation` retains its serde configuration, semantic descriptor, diff and inverse implementation, text and binary facet modules, and behavior tests; its declaration additionally derives `dsl::MutationLeaf` with `contract = ::protocol`.

Its leaf-local metadata test serializes the full lower descriptor with `serde_json`, compares it to canonical `🔣️.json`, asserts all five relative provenance locators, and requires a nonzero workspace token. The neutral fixture and schema at `🧪️txt-remove-line-metadata/` freeze the same fourteen descriptor fields and provenance expectations.

## Verification

Required scoped command:

```sh
bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️txt-remove-line-metadata/📜️script.ts
```

The initial pre-cutover invocation was blocked before the fixture script executed by the then-existing strict-mode Bun parse error in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` (`package` reserved-word binding). It is tooling evidence, not a feature-red result; the canonical-missing condition is instead the deterministic negative state expressed by the fixture script's required canonical-file assertions. After the shared entrypoint correction, the scoped post-cutover command completed exit0 with ten assertions and zero failures. This packet does not claim Rust runtime evidence. `git diff --check` also completed exit0.

## Root Handoff

Root owns the shared glue mount. The required mount replacement is:

```rust
#[path = "🗑️remove-line/🦀️.rs"]
pub mod remove_line;
```

No shared file was modified. Root must apply the mount and run the exact focused filter `canonical_leaf_metadata_matches_descriptor_and_provenance`; no Cargo command was run by this packet.

## Scope

Production writes are confined to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/**`; verification assets are confined to the existing ticket. No `compose/**` path was read or changed.
