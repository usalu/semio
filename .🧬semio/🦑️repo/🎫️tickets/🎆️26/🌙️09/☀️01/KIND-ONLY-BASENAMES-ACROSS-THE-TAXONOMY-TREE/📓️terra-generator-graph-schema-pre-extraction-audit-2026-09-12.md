# Generator Graph and Schema — Pre-Extraction Acceptance Audit

**Scope:** read-only preparation for the graph catalog and schema entity-catalog extraction. This review covers the graph’s nine-manifest input/output protocol, the hand-written Rust value bridge, and the 58-kind schema projection with its code and source-data consumers. UI and actor are outside this note. No generator, preview, native, or Nx route was run here.

## Graph Catalog Protocol

The current graph package router is [`📜️script.ts`](../../../../../../../🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts). It discovers these exact nine manifest IDs:

`drawing-layers`, `flow-dag`, `nakagin`, `puzzle2d-default`, `puzzle3d-default`, `puzzle5d-default`, `rewrite-lhs`, `wires`, and `writer-languages`.

[`📇️outputs.json`](../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json) is the language-neutral authority. It binds each ID to paired Rust and TypeScript leaves, and binds the shared Rust registry, TypeScript index, and TypeScript types. The parser rejects unknown fields, duplicate/unknown IDs, divergent Rust/TypeScript owners, unsafe paths, and mismatch with discovered IDs. One rendered byte plan feeds generate, preview, and freshness check. Preview reports exact file/directory nodes and stale removals; the writer inventories the output with `lstat` and rejects a symlink or other unsupported output entry.

The retained portable suite at [`🟦️.ts`](../../../../../../../🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts) already proves:

- Ajv and the owned catalog parser agree on the output catalog and hostile catalog paths.
- Writing produces exactly declared nested paths, prunes stale files, rejects Windows/UNC escapes, and refuses an output symlink without touching its target.
- The actual generated Rust registry and TypeScript index resolve every declared current manifest.

The prerequisite report records a direct `check-generated` success: nine manifests, three Bun tests and 70 assertions. That is earlier direct-package evidence, not a route executed by this audit.

### Graph Controls Still Required

Input discovery currently uses `statSync` after filtering, and silently continues on its errors. It can follow a source-manifest symlink and can omit an unreadable entry without evidence. Add portable input-admission vectors that prove:

1. a manifest leaf symlink and a directory symlink are never followed;
2. an unreadable admitted root, directory, or manifest reports a controlled failure/evidence instead of reducing the catalog;
3. duplicate IDs across physical manifests, malformed JSON, a non-`manifest` document, and an output-catalog identity mismatch cannot produce a partial plan;
4. generated input is skipped, while all nine admitted source manifests are represented exactly once;
5. preview, check, and write derive the same structured output plan, including stale-removal membership.

## Hand-Written Rust Value Bridge

[`🌉️generated-value-bridge.rs`](../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌉️generated-value-bridge.rs) is authored Rust, despite its basename. [`🛂️manifest/🦀️.rs:15`](../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:15) includes it directly. Its responsibility is explicit `dsl_core::ToValue`/`FromValue` wire mapping for enums emitted from the nine manifest sources; its mapped strings are not generated implementation text.

The existing native manifest unit module loads named manifests and exercises a nested `PropertyValue` round trip. It does not establish bridge-wide equivalence between every generated enum variant and every manifest wire string. Before moving the bridge to a semantic manifest/value-conversion owner, add a native control derived from structured manifest input that, for every admitted generated enum variant:

- checks `ToValue` emits the exact source wire string;
- checks `FromValue` round-trips it;
- rejects an unknown wire string and a non-string value; and
- proves every expected mapping occurs once.

This must compare semantic manifest data and runtime conversions, not bridge source text or a frozen body hash. Rebase the one `include!` edge and retain the native manifest crate test registration.

## Entity-Kind Catalog

The schema generator reads `🧬️schema/🔣️entity-kinds.json` with the owned TypeScript parser from `🧬️schema/🟦️.ts`, then projects a 58-entry ordered catalog to TypeScript, Rust, and Go. It has one canonical source and one common byte plan for preview, generate, and check.

The current generated Rust output is [`🤖️generated.rs`](../../../../../../../🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs). It must move to the neutral entity-catalog projection owner, with every producer, include, source-data reference, test, taxonomy output root, and launch/target input rebased. It is genuine generated output. This is distinct from the hand-written graph bridge.

The observable closure to retain is:

| Role | Current consumer |
| --- | --- |
| Rust code inclusion | [`⚛️component/🦀️.rs:69`](../../../../../../../🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs:69) `include!` |
| Rust source-as-data | [`⚛️component/🦀️.rs:864`](../../../../../../../🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs:864) `include_str!` |
| Source document | [`⚛️component/🦀️.rs:874`](../../../../../../../🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs:874) `include_str!` |
| Rust semantic proof | [`component-unit/🦀️.rs:611`](../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs:611) validates the document, all five entry fields, uniqueness, and FIRST-WINS |
| TypeScript portable proof | [`entity-kinds/🟦️.ts`](../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts) uses Ajv plus the owned parser, malformed vectors, all three provenance headers, and FIRST-WINS |
| Go source-data and runtime proof | [`component/🐹️.go:7763`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go:7763) compares source ordering to the Go projection; [`component/🐹️.go:1019`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component/🐹️.go:1019) consumes it at runtime |

The retained portable test has strong coverage: Ajv and the owned parser agree on malformed records, 58 entries, duplicate IDs, first-wins handling for the two intentionally shadowed emoji, and provenance for every projection. The Rust test independently compares every source entry field to the generated Rust entry and uses the owned native validator. The Go test compares its projection in declaration order with the source document.

After moving the Rust output, retain these controls and add a real TypeScript module-resolution assertion from the relocated generated TypeScript leaf to its `EntityKind` type owner. An earlier producer emitted the wrong relative type import; byte freshness alone preserved it. Runtime import success and installed TypeScript resolution must both hold after the path changes.

## Registered Producer Inputs

Both producers are cached. Their taxonomy generator registrations enumerate broad source patterns, but that metadata alone is not proof that Nx cache keys include those sources.

The graph project’s local `namedInputs.default` currently covers graph Rust and its package project root, while its generator reads plugin manifest JSON, [`📇️outputs.json`](../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json), taxonomy, and the discovery implementation outside that root. The schema producer reads its catalog JSON and parser/type source outside its package project root. Their actual target inputs must explicitly cover every live source/data owner and, after extraction, every relocated implementation owner. Add a focused cache-input/ownership control that checks those paths rather than inferring them from the taxonomy generator registry.

Preserve the registered `generate`, `preview-generated`, and `check` targets in each package, the workspace generation aggregate, and the seed/generated launch preview routes. Use ticket-private preview roots for mutation-capable proof; ordinary generation must be evaluated only after the read-only preview and check evidence agrees.

## Acceptance Boundary

Acceptance needs a direct, acyclic owner graph with anonymous implementation leaves; real producer input closure; exact source-data/include edges; the native bridge mapping control; semantic 58-kind equality and FIRST-WINS across all three projections; and current registered preview/check/native evidence. The existing prerequisite results are helpful baseline evidence but do not accept a relocated source tree.

**Status:** pre-extraction audit complete. The static producer-input closure and graph source-admission controls require repair/proof before acceptance. No product file changed.
