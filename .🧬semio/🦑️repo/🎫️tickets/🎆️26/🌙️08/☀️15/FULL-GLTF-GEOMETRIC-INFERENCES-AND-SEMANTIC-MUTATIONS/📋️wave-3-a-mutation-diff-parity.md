# Wave 3-A — Semantic Mutation and Diff Facet Parity

## Frozen mutation contract

The public vocabulary contains 28 operations. Binary tags 0–23 retain their established meaning, and tags 24–27 are append-only additions:

| Tags | Operations |
| --- | --- |
| 0–2 | `NoMutation`, `SetSnapshot`, `SetAsset` |
| 3–5 | `InsertScene`, `RemoveScene`, `SetScene` |
| 6–8 | `InsertNode`, `RemoveNode`, `SetNode` |
| 9–11 | `InsertMesh`, `RemoveMesh`, `SetMesh` |
| 12–14 | `InsertAccessor`, `RemoveAccessor`, `SetAccessor` |
| 15–17 | `InsertMaterial`, `RemoveMaterial`, `SetMaterial` |
| 18–20 | `InsertBuffer`, `RemoveBuffer`, `SetBuffer` |
| 21–23 | `InsertAnimation`, `RemoveAnimation`, `SetAnimation` |
| 24 | `TransformNode(index, matrix?, translation?, rotation?, scale?)` |
| 25 | `ReparentNode(index, parent?, scene?, position)` |
| 26 | `BindNodeMesh(index, mesh?)` |
| 27 | `BindPrimitiveMaterial(mesh, primitive, material?)` |

Canonical text keywords are the corresponding kebab-case names. Optional indices use `-`; optional transform arrays use `-` or comma-separated IEEE-754 bit-pattern integers of exact arity 16, 3, 4, and 3.

`apply_gltf_mutation` is represented as a typed sum: accepted applications carry `GltfDiff`; rejected applications carry `GltfMutationRejection { code, path, detail }`. There is no empty-diff rejection fallback.

## Diff, inverse, and touched regions

`GltfDiff` remains a sparse 21-field state transition with strong per-field diffs and index-keyed removed/modified/added collection triples. Derived contracts expose:

- the accepted forward diff;
- the exact inverse satisfying `inverse.apply(forward.apply(base)) === base`;
- sorted, deduplicated touched paths;
- typed touched regions derived from the top-level path family.

Set-snapshot derives `between(base, replacement)` and its inverse captures the exact base snapshot; it does not introduce a replacement slot. Transform, reparent, node-mesh binding, and primitive-material binding derive sparse node/scene/mesh diffs and inverses from the prior values. Structural collection inverses preserve index transport through removed and added ranks.

## Reference validation and remapping

The cross-language reference-rule contract explicitly covers primitive attributes, primitive indices, every morph-target semantic accessor reference, skin inverse-bind matrices, animation sampler input/output accessors, incoming `InsertNode.children` transported from the pre-insertion namespace, and aligned buffer metadata/payload vectors. Removal refuses live references; insertion/removal remaps all declared reference families.

## Changed files

Exactly 33 existing non-Rust files changed:

- mutation root: `🧬️mutations/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- mutation text: all seven existing leaves under `🧬️mutations/📝️text`
- mutation binary: all five existing leaves under `🧬️mutations/💾️binary`
- set-snapshot: the three existing TypeScript leaves under `🧬️mutations/📄set-snapshot`
- diff root: `🔺️diff/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- diff text: `🔺️diff/📝️text/{🅰️component.g4,🔗️component.graphql,🔣️component.json,🔤️component.ebnf,🛰️component.proto,🟦️component.ts}`
- diff binary: `🔺️diff/💾️binary/{🌶️component.spicy,🔠️component.abnf,🟦️component.ts,🥋️component.ksy}`

All paths above are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema`.

The existing `🔺️diff/📝️text/📖️component.grammar.semio` and `🔺️diff/💾️binary/📡️component.protocol.semio` already described the real 21-field Rust codecs exhaustively and remained unchanged. No Rust, inference, snapshot, glue, or framework file was edited.

## Validation evidence

- Bun TypeScript transpilation for root, text, binary, and set-snapshot leaves: passed.
- Root and text JSON parse plus Draft 2020-12 compilation with resolved mutation/diff references: passed.
- Kaitai YAML parsing for mutation and diff leaves: passed.
- Cross-format parity for all 28 variants across TypeScript, GraphQL, JSON Schema, Proto, EBNF, ANTLR, and Semio grammar: passed.
- Frozen binary tag check for every tag 0–27: passed.
- Reference-rule parity, including morph targets, incoming node children, and buffer alignment: passed.
- Cross-format parity for all 21 diff fields: passed.
- Proto relative-import resolution and set-snapshot TypeScript import resolution: passed.
- `bun nx run '@semio-tech/framework-os-kernel:test-quick'`: 862 tests passed, including handcrafted grammar/protocol conformance and production coverage; one unrelated pre-existing cross-artifact rejection test failed because six unrelated artifacts have no shipped DSL fixtures.
- Native GraphQL, Proto, ANTLR, ABNF, Kaitai, and Spicy compilers are not installed; those formats received the strongest available structural/workspace parser checks above.
