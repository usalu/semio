# Small Schema Direct 01

## Scope

- Writer 1 Any schema: `rename-writer`, `change-uri`, `change-language`, `edit-text`.
- Imperative 1 Any schema: `create-step`, `delete-step`, `reorder-steps`, `edit-step-params`.
- S Space 1 Any schema: `create-artifact`, `delete-artifact`, `rename-artifact`, `touch-artifact`.
- Writer IO and Space Home were immutable exclusions for this batch.

## Result

- Twelve nested payload owners moved to authoritative direct `🦀️component.rs` leaves.
- Twelve completed language-neutral descriptors point to distinct direct `🔣️payload.schema.json` files.
- Writer and Imperative each expose eight required files per leaf: Rust, descriptor, payload schema, TypeScript, GraphQL, protobuf, text, and binary.
- S Space exposes six required files per leaf: Rust, descriptor, payload schema, TypeScript, text, and binary; its root did not previously declare GraphQL or protobuf surfaces.
- Behavior, codec bridges, shared path/store helpers, and behavior tests moved from mutation aggregates to sibling schema `⚙️operations` owners. Roots retain only direct reexports, aggregate enums, derived registry behavior, and structural correspondence tests.
- Catalog parity is exact: `4` direct owners, `4` descriptors, `4` catalog kinds, and `4` vectors for each root.

## Executed Evidence

- Scoped `policyMutationStructuralBreaches` through root `📜️script.ts`: Writer `0`, Imperative `0`, S Space `0` across all 17 registered policy classes.
- Ajv versus repository-owned Draft-07 subset validator: `48/48` agreement cases (`24` valid and `24` deliberately invalid), zero disagreements.
- Pinned nightly `rustc -Zunpretty=ast-tree`: `18/18` parsed (`12` direct owners, `3` aggregates, `3` operations owners).
- Static catalog/surface oracle: Writer `4/4/4/4` with 8 surfaces, Imperative `4/4/4/4` with 8 surfaces, S Space `4/4/4/4` with 6 surfaces.
- `rustfmt +nightly --edition 2021 --check` passed for direct owners, optional Rust facets, aggregate roots, and operations owners. Glue passed with `skip_children=true` so unrelated mounted sources were not treated as this batch's formatting scope.
- Scoped scans found no `unclassified`, empty outcome classifications, legacy payload-owner mounts, singular `::mutation` routes, or `[DEBUG]` markers.

## Deferred Runtime

No Cargo or Nx Rust target was launched because the coordinator kept the shared STDIO gate active. Therefore this batch makes no compile/runtime-pass claim. The registered plugin targets must be rerun after that shared dependency gate clears, with a temporary `[DEBUG]` runtime probe removed before final global closure.
