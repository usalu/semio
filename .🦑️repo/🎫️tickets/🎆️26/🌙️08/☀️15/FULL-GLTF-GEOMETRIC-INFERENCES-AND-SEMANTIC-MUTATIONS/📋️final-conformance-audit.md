# Final GLTF Conformance Audit

## Result

The current combined tree conforms to the frozen GLTF inference, mutation, diff, text, and binary contracts in every audited dimension. Four concrete facet defects were found and repaired during the audit:

1. The mutation and diff Semio text grammars still described the former six-field `GltfPrimitive` tuple. Both now model the seven-field tuple with `morph-target-list` between `mode` and `extensions`.
2. The GLTF operation grammar law now asserts that mutation and diff primitive tuple productions remain identical and explicitly contain morph targets.
3. The root mutation and diff Proto facets imported nonexistent `snapshot.proto` and `diff.proto` paths. They now resolve to the existing sibling Proto facets.
4. `GltfEntityAddress.scope` was an unconstrained string in GraphQL and Proto. Both root and text GraphQL facets and the root Proto facet now use the same seven-value `GltfEntityScope` taxonomy as Rust, TypeScript, and JSON Schema.

No compatibility seam was introduced.

## Frozen Inference Contract

- `GltfInference` has exactly one root field, `geometry`, in Rust, TypeScript, GraphQL, text GraphQL, JSON Schema, and Proto.
- There is no root `bounds`, `min`, `max`, `vertexCount`, `meshCount`, or `primitiveCount` field. `GltfBounds3.min/max` and `geometry.counts.*` are the intended nested value locations, not legacy roots.
- The taxonomy contains 14 groups and exactly 67 unique indicator fields. Every field is present once in its taxonomy group across Rust, TypeScript, GraphQL, text GraphQL, JSON Schema, and Proto.
- Overall, per-part, and pair records are present together with policy, diagnostics, quality, availability, validity, provenance, units, coordinate spaces, tolerances, counts, and entity addresses.
- Entity scope is closed to `document`, `scene`, `nodeInstance`, `mesh`, `primitive`, `component`, and `surfaceRegion` across the typed facets.
- The text envelope is schema `s.stdio.gltf.inference`, version 2, four LF-terminated headers, UTF-8 payload length, lowercase CRC-32/ISO-HDLC, and RFC 8785 canonical JSON without a trailing LF.
- The binary envelope agrees across Rust, TypeScript, ABNF, Kaitai, Spicy, and Semio protocol: eight-byte magic, format 1.0, inference schema version 2, canonical-JSON flag 1, schema CRC `0x6b257ae0`, `u64` payload length, payload CRC, header CRC, and 40 total fixed bytes.

## Frozen Mutation Contract

- Exactly 28 variants are present in TypeScript, GraphQL, JSON Schema, Proto, EBNF, ANTLR, Semio grammar, binary TypeScript, and Rust.
- Rust and binary TypeScript agree exactly on frozen tags 0–27. Proto oneof ordinals are the corresponding 1–28 field numbers.
- Tags 24–27 are `TransformNode`, `ReparentNode`, `BindNodeMesh`, and `BindPrimitiveMaterial`; their canonical text keywords are `transform-node`, `reparent-node`, `bind-node-mesh`, and `bind-primitive-material`.
- `GltfMutationApplication` is a typed accepted/rejected sum in TypeScript, GraphQL, JSON Schema, and Proto. Accepted carries `GltfDiff`; rejected carries exactly `GltfMutationRejection { code, path, detail }`. There is no empty-diff rejection fallback.
- Derivation exposes mutation, diff, exact inverse, sorted/deduplicated touched paths, typed touched regions, and reference rules.
- Reference modeling includes primitive attributes, indices, every morph-target semantic accessor, skin inverse-bind matrices, animation sampler input/output, incoming `InsertNode.children` in the pre-insertion namespace, and aligned buffer metadata/payload vectors.
- The repaired primitive text AST is `attributes, indices, material, mode, targets, extensions, extras` in both mutation and diff grammars.

## Frozen Diff Contract

- Exactly 21 sparse top-level fields agree across Rust, TypeScript, GraphQL, JSON Schema, Proto, binary TypeScript, ABNF, and Semio protocol.
- Binary order is `asset`, `scene`, `scenes`, `nodes`, `meshes`, `accessors`, `bufferViews`, `buffers`, `bufferBytes`, `materials`, `textures`, `images`, `samplers`, `skins`, `animations`, `cameras`, `extensionsUsed`, `extensionsRequired`, `extensions`, `extras`, `sourceForm`.
- No `snapshot` replacement slot exists.
- Native tests prove between/apply, inverse, absorb, index transport, field sweep, touched paths/regions, and text/binary codec round trips.

## Mechanical Audit

The ticket-local deterministic audit is `🧪️conformance-audit.ts`. It performs:

- normalized 67-field set equality across six inference representations;
- geometry-only root assertions;
- all 28 mutation variants, text keywords, Rust tags, binary tags, and Proto ordinals;
- typed accepted/rejection and rejection-field equality;
- reference-rule equality;
- all 21 diff fields and binary order across eight representations;
- morph-target primitive grammar equality;
- actual Proto import resolution;
- TypeScript syntactic transpilation for all 13 facet files;
- JSON parsing and Draft 2020-12 AJV compilation for all six schemas;
- YAML parsing for all three Kaitai facets;
- inference envelope constant parity.

```text
bun '.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️15/FULL-GLTF-GEOMETRIC-INFERENCES-AND-SEMANTIC-MUTATIONS/🧪️conformance-audit.ts'
PASS indicators=67 groups=14 inference-roots=geometry-only mutations=28 tags=0..27 diff-fields=21 proto-imports=resolved morph-target-grammar=7-fields ts=13 json=6 ksy=3 accepted-rejection=typed inference-envelope=parity
```

## Native Validation

```text
cargo test -p semio-s-plugin-stdio 'artifacts::gltf::standards::v2_0::subsets::any::io::component::tests::conformance_laws' --lib
6 passed; 0 failed
```

This covers committed grammar/protocol parsing, snapshot grammar, mutation grammar, diff grammar, protocol walking, and fixture honesty. The repaired `ops_grammar_conformance_law` is included and passes.

```text
cargo test -p semio-s-plugin-stdio 'artifacts::gltf::standards::v2_0::subsets::any::schema::mutations::component::tests' --lib
10 passed; 0 failed
```

This covers all mutation text/binary round trips, unknown/trailing input rejection, all-variant mutation/diff and inverse laws, structural index transport, semantic operations, stable regions, referenced-removal rejection, pre-insertion node children, morph-target accessor transport, and buffer metadata/payload alignment.

```text
cargo test -p semio-s-plugin-stdio 'artifacts::gltf::standards::v2_0::subsets::any::schema::diff::component::tests' --lib
9 passed; 0 failed
```

```text
cargo test -p semio-s-plugin-stdio 'artifacts::gltf::standards::v2_0::subsets::any::schema::inferences' --lib
23 passed; 0 failed
```

The inference group covers deterministic/default/dependency laws, all current analytic geometry fixtures, exact/estimated quality behavior, invalid/open/non-manifold cases, text canonicalization and corruption rejection, and exact binary framing/corruption rejection.

```text
cargo test -p semio-s-plugin-stdio 'artifacts::gltf::standards::v2_0::subsets::any::schema::diff::component::handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law' --lib -- --exact
1 passed; 0 failed
```

```text
cargo test -p semio-s-plugin-stdio inference_wire_echoes_revision_and_uses_frozen_binary_codec --lib
1 passed; 0 failed
```

```text
bun nx show project '@semio-tech/stdio-plugin'
passed
```

## Broader Gate Evidence

```text
bun nx run '@semio-tech/stdio-plugin:test-quick'
failed outside GLTF after 43 passes
```

The quick gate compiled the combined tree and started 3,448 tests. It stopped at the pre-existing BCF `fixture_honesty_law`, where regenerated ZIP bytes differ from the shipped BCF fixture; 3,404 tests were consequently not run. Every focused GLTF group above passes.

An initially broad name filter also ran 53 cross-artifact `diff_codec_text_binary_roundtrip_law` tests: the GLTF test passed, while the unrelated PPTX test failed on its `font_size` tri-state. The exact GLTF invocation above then passed 1/1.

## Files Changed by This Audit

1. `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
2. `🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`
3. `🚪️io/🦀️component.rs` — existing inline conformance test only
4. `🧬️schema/🧬️mutations/🛰️component.proto`
5. `🧬️schema/🔺️diff/🛰️component.proto`
6. `🧬️schema/💡️inferences/🔗️component.graphql`
7. `🧬️schema/💡️inferences/📝️text/🔗️component.graphql`
8. `🧬️schema/💡️inferences/🛰️component.proto`

The schema paths above are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any`.

## Residual Gaps

- No GLTF contract or runtime gap remains in the requested audit dimensions.
- Native GraphQL, Proto, ANTLR, ABNF, Kaitai code-generation, and Spicy compilers are not installed. Those facets received deterministic structural parity checks, real import resolution, TypeScript/AJV/YAML parsing where applicable, and the repository's native Semio grammar/protocol parser and execution laws.
- The unrelated BCF fixture and PPTX tri-state failures remain outside this ticket and were not modified.
