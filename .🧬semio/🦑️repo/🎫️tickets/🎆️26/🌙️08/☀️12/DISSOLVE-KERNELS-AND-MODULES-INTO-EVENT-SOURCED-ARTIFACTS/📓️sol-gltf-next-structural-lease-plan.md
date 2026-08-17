# glTF Next Structural Lease Plan

## Scope Evidence

The deterministic census path filter records thirteen immediately actionable structural errors:

- 8 `collection-authored-behavior` errors at the inference and mutation collection roots;
- 1 `collection-manifest-missing` error at `🏅️standards`;
- 2 errors at `🔖️2.0` (`manifest-child-missing`, `member-component-leaf-missing`);
- 2 errors at `✳️any` (`manifest-child-missing`, `member-component-leaf-missing`).

The CLI scoped report currently prints zero errors for `s.stdio.gltf` because it filters unregistered collection-path problems out when selecting component IDs. It is not a valid release signal while the census path filter has errors. Correcting that central report-scope bug is a separate Sol ownership item; no scope exception is permissible.

`ROOT` below denotes `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any`.

## Graph-colored Lease A: Standards and Subset Manifests — 5 Errors

This is the smallest immediately safe Terra lease. It is manifest-only and has no source, generator, or registrar edit.

| Writable path | Required responsibility |
| --- | --- |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔣️component.json` | New canonical collection manifest for the `🔖️2.0` standard child. |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🔣️component.json` | New canonical collection manifest for `🪆️subsets`. |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/🔣️component.json` | Replace the legacy `artifact`/`standard`/`subsets` object with the canonical `x-semio` collection declaration for `✳️any`. |
| `ROOT/🔣️component.json` | New canonical collection manifest for `📚️examples`, `🔨️modules`, `🚪️io`, and `🧬️schema`, using only allowed collection assembly declarations. |

The owner must classify each direct child according to the collection grammar rather than inventing a wrapper component or a wildcard. No language leaf, package glue, generated output, or source contract belongs to this lease. Validation is the deterministic census plus the path-filtered glTF report; the central report scope bug is recorded separately.

## Graph-colored Lease B: Inference Collection Assembly — 4 Reported Errors

This is independent of mutations but is not manifest-only. Its atomic source boundary is the aggregate geometric-analysis inference and its direct consumers.

| Writable source/contract path | Required disposition |
| --- | --- |
| `ROOT/🧬️schema/💡️inferences/🦀️component.rs` | Move the aggregate `GltfInference`, inference descriptor, invalidation mapping, and other non-mechanical behavior to the existing `🧮️geometric-analysis` inference; leave only generated/mechanical assembly or remove the root language leaf. |
| `ROOT/🧬️schema/💡️inferences/🟦️component.ts` | Move the authored aggregate `GltfInference` contract to `🧮️geometric-analysis`; retain only mechanical collection exports if still needed. |
| `ROOT/🧬️schema/💡️inferences/🔗️component.graphql` | Move the aggregate GraphQL contract to the same specific inference representation. |
| `ROOT/🧬️schema/💡️inferences/🛰️component.proto` | Move the aggregate protobuf contract to the same specific inference representation. |
| `ROOT/🧬️schema/💡️inferences/🔣️component.json` | Move the authored JSON-schema aggregate contract with `GltfInference` to `🧮️geometric-analysis/🔣️component.json`; retain only the canonical collection `x-semio` manifest at the root. |

The five files are one semantic move: the JSON manifest is not currently reported as behavior but contains the same authored aggregate schema and therefore must move atomically. No generator provenance is declared for these files and no generated header or generator input was found; they are authored source, so no direct generated-file edit is allowed.

Direct public referrer and central registrar ownership:

- Central-only registrar request: `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`, glTF inference root mount at lines 2245–2247. It must remove the root `component` mount only after every consumer imports the specific `geometric_analysis` component directly. The existing `geometric_analysis` mount remains the single owner.
- Source owner must update all glTF Rust/TypeScript/schema consumers of the root `GltfInference` contract in the same lease; no forwarding re-export or old path alias is allowed.
- The source owner owns local schemas, protocol mirrors, fixtures, and tests. The coordinator owns only the requested glue edit after the owner supplies an exact post-referrer-sweep request.

## Graph-colored Lease C: Mutation Collection Assembly — 4 Reported Errors, Coupled to the 59-Error Mutation Tree

Do not assign this as a small independent lease. The following root files contain the closed `GltfMutation` union, dispatch, contracts, and fixtures:

- `ROOT/🧬️schema/🧬️mutations/🦀️component.rs`
- `ROOT/🧬️schema/🧬️mutations/🟦️component.ts`
- `ROOT/🧬️schema/🧬️mutations/🔗️component.graphql`
- `ROOT/🧬️schema/🧬️mutations/🛰️component.proto`
- `ROOT/🧬️schema/🧬️mutations/🔣️component.json` (the unreported but authored schema contract)

Every concrete mutation's Rust facets and many TypeScript inverse facets import the root `GltfMutation` contract. The current root Rust mount is `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` lines 2253–2255; the root itself is a dependency hub for the 31 missing-manifest-child and 28 missing-component-leaf paths. It therefore shares its direct consumer set and dependency cycle with the entire mutation tree. The legal lease is the future full mutation-collection refactor: move non-mechanical union/dispatch/validation behavior to specifically owned components, regenerate only mechanical collection assembly, update all direct mutation facets, codecs, schema mirrors, fixtures, and then request the one central glue replacement. It must not be split from the 59-error mutation-tree lease or patched with a wrapper alias.

No generator provenance is declared for the root mutation contract. Existing `🧭️planning`, text, and binary leaves are source-owned but do not prove that the root union can simply be moved there; the future owner must classify each responsibility before moving it.

## Recommended Assignment Order

1. Assign Lease A immediately to Terra; it is conflict-free and reduces five errors without registrar coordination.
2. Assign Lease B to a separate Terra only after Lease A releases its parent manifests; queue its exact Rust glue request to the Sol coordinator.
3. Schedule Lease C together with the 59-error mutation-tree queue, not as a small follow-up. It is one dependency SCC for practical lease purposes.
4. Sol fixes report-scope selection so `verify taxonomy report --scope s.stdio.gltf` includes collection-path findings under the selected semantic owner before any owner can graduate to clean.
