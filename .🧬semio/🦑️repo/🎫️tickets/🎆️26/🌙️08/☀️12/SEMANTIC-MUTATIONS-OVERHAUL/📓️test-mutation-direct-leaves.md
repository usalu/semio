# TestMutation Direct Leaves

## Scope

The test-only document mutation owner now has two direct leaves: `SetCount` at `📝️set-test-count` and `SetLabel` at `🏷️set-label`. `TestMutation` is a transparent `dsl::Mutations` aggregate. Its text and binary codecs retain the previous generic `DslVariants` and `variants_binary` representation, including `set-count`/tag `0` and `set-label`/tag `1`.

The main test application imports the aggregate and its payload types from the mounted fixture root. Every remaining main-source `TestMutation::SetCount` and `TestMutation::SetLabel` construction/pattern is the tuple form; the static scan found no retired inline enum, manual `Mutation`, manual `SemanticMutation`, retired payload types, or braced enum payload construction.

## Evidence

The schema-first Ajv controller first ran RED before the canonical leaves and mount were present:

- `🧪️test-mutation-direct-leaves/🧫️run-c1NZse`: 22/30, exit 1.

It then ran GREEN through Bun and scoped Nx:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️test-mutation-direct-leaves/📜️script.ts
```

- `🧪️test-mutation-direct-leaves/🧫️run-XfYCCq`: 40/40, exit 0.
- The controller validates both full 14-field descriptors and valid/invalid actual payloads with Ajv 2020, plus text reference values, generic binary ordinals, canonical source/mount presence, and input-hash stability.
- Native Rust tests were authored but not run: `direct_leaves_preserve_generic_document_codecs_and_laws`, `descriptor_has_set_count_identity`, and `descriptor_has_set_label_identity`. Cargo and rustc were intentionally not invoked.

## Source Boundary

Changed production/test-fixture paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-mutations/🧬️document/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`

The final SHA-256 values are:

- `🦀️component.rs`: `55c1ffef6ee1006be88705cf8e6526adc5d6ef3658720d410990458e20e641eb`
- `🧪️tests/🧬️test-app-mutations/🦀️.rs`: `8ab7c2f4166f27cc65afe7b0f4ea99acf56493369be39695bf77971a8e8510da`
- `🧬️document/🦀️.rs`: `1560fb6a3c209343fd6bf24a43cc6a44abe99d7daa018f53c8e6f9b0523e17ec`
- `🧬️document/🧬️mutations/🦀️.rs`: `a1a0d8d87cac48edb47619f7f3d642b40a471a786da838871d8adaea29d0eefa`
- `📝️set-test-count/🦀️.rs`: `135e7f522cb00af8def206c78b01f591ca0e6cc07753fe8cc4c84d01b1f8a641`
- `🏷️set-label/🦀️.rs`: `2a757c8f12fb6151d29db229b2be3efe2680abf2e191d807c2f8dfa2ce005e0f`

The only source changes are the main fixture import/tuple joins, the mounted test-app fixture root, the document aggregate/root, both direct leaves and their descriptors/schema, and this packet's retained neutral evidence. Config selection, Interaction, lifecycle/reactor, builder, contributed wire, channel fixtures, launch, and seed were not edited.

## Root Runtime Filter

Once the source is admitted to the serialized native slot, run the three authored tests by their exact names:

```text
direct_leaves_preserve_generic_document_codecs_and_laws|descriptor_has_set_count_identity|descriptor_has_set_label_identity
```
