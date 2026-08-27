# Dependency-First Small Direct 01

Date: 2026-08-27. Owner lane: TERRA-SMALL-SCHEMA-DIRECT continuation.

## Scope and Result

| Exact mutation root | Direct leaves | Enum / catalog kinds / catalog vectors | Scoped structural findings |
| --- | ---: | --- | ---: |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 2 | 2 / 2 / 2 | 0 across 17 classes |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 3 | 3 / 3 / 3 | 0 across 17 classes |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 1 | 1 / 1 / 1 | 0 across 17 classes |

Every leaf owns `🦀️component.rs`, `🔣️component.json`, `🔣️payload.schema.json`, `🟦️component.ts`, `🔗️component.graphql`, `🛰️component.proto`, `📝️text/🦀️component.rs`, and `💾️binary/🦀️component.rs`: 48 direct files across the six semantic owners. Existing diff, inverse, and language-neutral scenario fixtures remain beside those owners. The six empty nested implementation directories were removed after moving their Rust/TypeScript files; no payload content was discarded.

The roots now contain direct aggregate wrappers, reexports, and structural correspondence tests. Behavior bridges, store aliases, catalog constants, and cross-kind laws moved to each schema's sibling `⚙️operations/🦀️component.rs`. Each direct Rust owner still visibly implements `MutationKind` and owns `SEMANTICS`.

## Exact Consumer Write Set

- The three plugins' `📦️packages/🦀️rust/📦️glue.rs` mount direct components, leaf text/binary facets, and schema operations.
- GIS terrain's schema root binary test imports and nearest editor imports in `✏️editor/🦀️component.rs` and `✏️editor/🎮️commands/🏔️exaggeration/🦀️component.rs` now use direct payload paths.
- Curate IO `🚪️io/🧬️mutations/📝️text/🦀️component.rs` received only the nine necessary deleted-module-segment corrections in its existing constructions. IO wire ownership and behavior were not refactored.
- GIS/Home existing codec roots and Curate schema codec registries visibly assemble leaf-owned identities. Curate's existing IO encoder/decoder remains in place.
- The six direct TypeScript payloads and three aggregate TypeScript roots now carry the actual payload fields. GraphQL/protobuf/payload-schema counterparts match those fields.

## Executed Gates

1. `bun -e` imported `policyMutationStructuralBreaches` from repository `./📜️script.ts` and called `policyMutationStructuralBreaches(process.cwd(), [root])` for each exact existing root. The first correctly scoped pass reported GIS 2, Curate 12, Home 2, all codec-registry parity. After direct registries were connected, the existence-checked rerun reported 0/17 for all three.
2. `bun -e` imported Ajv plus repository `validateJsonSchemaSubset`. For each leaf it checked the descriptor, rejected empty outcomes, rejected unclassified invertibility, accepted the committed fixture payload, rejected a missing required field, and rejected an extra field. Both implementations agreed on all 36 cases: 12 valid and 24 invalid. No external runtime dependency was added.
3. The same Bun run compared direct owner, aggregate wrapped type, descriptor, catalog kind, catalog directory, and committed vector identities. Exact counts were GIS 2/2/2/2, Curate 3/3/3/3, Home 1/1/1/1. Each owner had all eight direct files.
4. `rustc +nightly --edition 2021 -Zunpretty=ast-tree <file>` parsed the 6 direct owners, 3 aggregate roots, and 3 operations files: 12/12. The independently emitted AST agreed with the repository lexical inspector on all 6 aggregate variants and all 6 MutationKind/SEMANTICS implementations.
5. `new Bun.Transpiler({loader:"ts"}).transformSync(source)` parsed all 9 direct/aggregate TypeScript surfaces. Existing `graphql` and `protobufjs` test libraries were unavailable; no dependency was installed.
6. `find <the three mutation roots and their three operations siblings> -maxdepth 3 -type f -name '🦀️component.rs' -print0 | xargs -0 rustfmt +nightly --edition 2021 --check` passed. The three glue files passed `rustfmt +nightly --edition 2021 --config skip_children=true --check`.
7. Scoped `git diff --check -- <the exact mutation/operations/consumer/glue paths>` passed. Scoped `rg` found no `[DEBUG]` source logs, singular `::mutation::` route, nested implementation Rust mount, or deleted nested TypeScript import.

The inline validation probe printed:

```text
[DEBUG] direct-schema oracle agreement {"owners":6,"agreements":36,"valid":12,"invalid":24}
{"current":{"nightlyAstParsed":12,"aggregateVariantsAgreed":6,"mutationKindImplFacts":6}}
```

The debug probe was inline only; no temporary runtime log was left in source. These are executed static/schema checks, not a claim that mutation application runtime passed.

## Prior Batch Regression Closure

The coordinator's existence-checked acceptance run found one Imperative codec ownership breach and eight Space schema parity breaches. Those findings were reproduced before repair.

- Writer, Imperative, and S Space root structural tests now check the singular `::mutation::` segment rather than the prefix `::mutation`, which incorrectly also matched valid `::mutations::` imports.
- S Space's missing root `🔣️component.json` is restored as an `SSpaceMutation` aggregate with explicit references to all four existing direct payload schemas. Those leaf schemas were already present and unchanged.
- Imperative's four flattened wire records and their domain conversions now live in direct `🌱create-step/📝️text`, `🗑️delete-step/📝️text`, `🔀reorder-steps/📝️text`, and `🔧edit-step-params/📝️text` components. Its root binary component is a wrapped wire aggregate plus generic framing and ordered converter registries, with zero match arms. Declaration order and binary tags remain 0/1/2/3.
- Added a wire correspondence test checking keyword order, field order, and binary tags; existing round-trip fixture tests remain.
- The five changed Imperative codec sources passed pinned nightly AST parsing (5/5), and the root inspector reports four wrapped wire variants and zero match arms. Scoped rustfmt passed.
- Final existence-checked scoped policy: Writer 0/17, Imperative 0/17, S Space 0/17.

## Runtime Boundary

No Cargo build was started in this lane. The coordinator required serialized runtime validation because the shared STDIO test compilation was active and later timed out in the Demonstrator target. Runtime/law/codec assertions remain pending for the registered Nx targets; static/schema/AST acceptance does not substitute for that runtime proof.

No real `compose/**` path was read, listed, traversed, or changed. No modifying git command was used. Demonstrator/Playground remained untouched.
