# Off-Facet Declaration Inspector Boundary

## Captured Input Boundary

`inventoryMutationTaxonomy` begins at root `📜️script.ts:20925`. Its `before` value is directly a `MutationTaxonomySourceIndex`, created by `mutationTaxonomySourceSnapshot` at `20842`. It has `admission`, `roots`, `files`, `bytes`, `contents`, `directories`, captured schemas, a source roster, and a digest. It does **not** have a `sourceIndex` property. The approved next census may consume that snapshot's already-captured `files` and `contents`; it must not add a second source-root list, a new filesystem walk, or a content read.

## Exported Inspector Evidence

Discovery owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`.

- `inspectRustStructure` (`5667`) returns `RustStructuralFacts`, including generic `RustImplFact { traitPath, selfType, methods, associatedConstants }`, modules, enums, inline payloads, match arms, constants, includes, and test modules.
- `inspectRustMutationMetadataFacts` (`5834`) returns declaration/derive/conditional facts, `extern`/`use`/reexport aliases, and `manualMutationLeafImpls`. The latter is a literal final-`::`-segment filter for `MutationLeaf`; its current output excludes `MutationLeaf<T>`.
- `inspectRustModuleGraphFacts` (`5996`) and `inspectRustModuleGraph` (`5945`) establish source-module and use edges. They do not resolve a trait path to a crate definition.
- The searched exported TypeScript declaration-inspector set is absent. The discovery exports that begin with `inspectRust` are Rust-specific (`inspectRustStructure`, `inspectRustMutationMetadataFacts`, `inspectRustModuleGraphFacts`, `inspectRustPublicTypeNames`, `inspectRustMutationAggregateSpan`, `inspectRustVirtualSources`). Root's current TS consumer path is only `mutationTaxonomyTsSpecs` (`📜️script.ts:20971`), which extracts import/reexport specifiers rather than object/interface/type-alias/union declaration facts.

Consequently, `impl Mutation<T>`, `impl MutationKind<T>`, `impl CompositeMutationKind<T>`, and a `use ... as ...` spelling are candidate evidence only. A local `trait Mutation<T>` produces the same structural `traitPath` as an intended external trait, so neither is a validated semantic mutation until a later resolver supplies crate/module evidence. An empty enum is only a structurally observed empty roster, not an implicit operation.

## Neutral Inspector Packet

New packet: `🧪️off-facet-declaration-census-56/`.

It validates eight language-neutral vector cases through strict Ajv and the actual exported Rust inspectors: manual `Mutation`, `MutationKind`, `CompositeMutationKind`; a valid direct `dsl::MutationLeaf` declaration; nested conditional empty enum; use alias; same-named local trait; and unconditional empty enum. The expected evidence state deliberately separates declaration/empty-roster observations from unresolved trait identity.

Executed command:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-declaration-census-56/📜️script.ts'
```

Observed result: `[DEBUG] off-facet declaration inspector 8/8 passed`, retained at `🧪️off-facet-declaration-census-56/🧫️run-mtc5672r/`. This validates inspector output against the neutral expectations; it is not a global mutation census and does not validate trait identity.

Input hashes captured by the run:

- discovery source: `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423`
- neutral schema: `ec431960851ef1a9903e5511b9ff5ae3fb3a3a7e799f77b03e1ae40f68805ee5`
- neutral vectors: `b2c6d48bbeae584aba7a4daa2454065cfc23ef3377da3a095dce38322dde848e`
- controller: `df4a8804566acb71abf44e1c330eae9540b803d46a1c1f88fd602841e8cfca4f`

## Bounded Follow-Up

An admission-derived census can classify only records whose source path is already present in `MutationTaxonomySourceIndex.admission` and whose Rust inspector facts are accompanied by mounted graph/context evidence. It must emit unresolved records for alias/local-trait/manual/generic gaps; it must not turn lexical names into a completed off-facet worklist. A separate exported TS declaration inspector is required before any TS object/union candidates can be structurally validated.

## Provenance-Hardened Rerun

The first retained run, `🧫️run-mtc5672r`, is preserved as an observed 8/8 example result only. Its controller statically imported discovery and captured hashes only after inspection, so it is not stable source-attribution evidence.

The controller now rejects opaque/escaping paths lexically before filesystem reads, checks the workspace-to-input directory ancestry with `lstat` and rejects symlinks, uses `O_NOFOLLOW` for the final regular-file read where available, captures all four inputs before work and again afterward, dynamically imports the captured discovery source only after the first capture, and creates a unique `mkdtemp` receipt directory. Any thrown path records both attempted captures and a full error receipt.

The one bounded rerun used the same eight expected inspector facts unchanged:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️off-facet-declaration-census-56/📜️script.ts'
```

Observed result: `[DEBUG] off-facet declaration inspector 8/8 passed`, retained in `🧪️off-facet-declaration-census-56/🧫️run-s7E0xv/`. Before and after hashes were identical:

- discovery: `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423`
- schema: `ec431960851ef1a9903e5511b9ff5ae3fb3a3a7e799f77b03e1ae40f68805ee5`
- vectors: `b2c6d48bbeae584aba7a4daa2454065cfc23ef3377da3a095dce38322dde848e`
- hardened controller: `95f722b73cee5332f56e500056ce21e96f0b4f0cd42848913f585447a34d5466`

The vector `evidence` value is a golden annotation supplied by the neutral data. It is not an inspector-resolved proof of trait origin, reachability, leaf ownership, or mutation semantics.
