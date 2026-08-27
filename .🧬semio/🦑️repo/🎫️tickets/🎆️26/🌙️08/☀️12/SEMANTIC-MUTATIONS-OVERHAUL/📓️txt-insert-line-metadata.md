# TXT-INSERT-LINE-METADATA-15 — Canonical Insert Line Metadata

## Leaf Cutover

The direct TXT insert-line leaf now owns the taxonomy primary `🦀️.rs` and descriptor `🔣️.json`. The former `🦀️component.rs` and `🔣️component.json` files were removed without aliases or fallback lookup. The Rust text and binary facets retain their existing owned filenames because no canonical-facet rule was supplied.

`InsertLineMutation` preserves its existing `Clone`, debug, equality, serde, semantic identity, diff, inverse, text codec, and binary codec behavior. It now additionally declares the accepted public metadata derive:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
```

Its leaf-owned test serializes `<InsertLineMutation as protocol::MutationLeaf>::DESCRIPTOR` through `serde_json`, compares it with canonical `🔣️.json`, checks every expected relative provenance locator, checks a nonzero workspace token, and retains the existing behavior/serde/codec tests.

## Neutral Preflight

`🧪️txt-insert-line-metadata/🛂️schema.json` and `🧫️fixtures/🔣️expected.json` freeze the entire fourteen-field descriptor and expected relative provenance paths. The ticket script validates the fixture through Ajv and structurally checks the canonical filenames, removed legacy names, derive attribute, lower-trait test, serde oracle, nonzero-token assertion, and descriptor equality.

The deliberate pre-cutover result is retained in `🧪️txt-insert-line-metadata/🧪️precutover-structural-red.log`: 7 of 7 canonicality assertions failed before the rename/derive. The post-cutover script result is retained in `🧪️txt-insert-line-metadata/🧪️canonical-ast-green-10.log`: 10 assertions, 0 failures.

## Root Handoff

The sole required shared mount change is the insert-line module's primary path:

```rust
#[path = "📥️insert-line/🦀️.rs"]
pub mod insert_line;
```

This packet did not edit the mount or any shared glue. Root must apply that one-line mount update before its registered STDIO Rust compiler/runtime command. Runtime evidence is therefore pending root integration; no Cargo command was started by this packet.

## Scope

Production writes are confined to `.../🧬️mutations/📥️insert-line/**`. No `compose/**` path was read or changed.
