# Composable Artifact Packages

## Implementation

Artifacts now have independent package declarations while their Rust, TypeScript and browser implementations remain in the domain taxonomy. The production inventory is 99 Rust artifacts plus two shared artifact contracts, with 40 TypeScript artifact packages. The stdio plugin composes 36 independently selectable format packages. Its shared registry contract does not depend on the artifact catalog; Semio format conversion dependencies are selected through explicit features.

The seven framework artifact owners are Workflow Run, Workflow, Playbook, Flow, Infinite DAG, Space and Collection. Plugin and host consumers use their packages directly. Nested Draw FSM and Sequence browser implementations were also moved out of package directories. The former Sequence TypeScript parent is a private integration harness; its public browser API belongs to the Sequence artifact.

Nx recognizes the package-neutral implementation sources, direct package prerequisites and emitted outputs. Package commands route through the existing Bun script mechanism. The TypeScript builder preserves referenced JSON and handwritten declaration sidecars, and its downstream consumer check reports unresolved declarations instead of suppressing them. Launch workflows are owned by the persistent launch seed.

## Validation In Progress

This report is a live acceptance ledger. The goal is still active; remaining native and aggregate gates must finish before ticket closure.

| Gate | Observed result |
| --- | --- |
| Stdio standalone package compile | All 36 format packages compiled in the earlier isolated check. |
| Norm package compile | All 15 standard artifacts and both direct FEM feature integrations compiled. |
| Framework native runtime | Workflow Run 19, Workflow 37, Playbook 14, Flow 37, Space 4, Collection 4 tests passed in completed gates. |
| DAG runtime | Existing 55-test gate and focused retirement regression passed. |
| PDF package output/cache | Own-source edit invalidated cache; sibling JPG edit preserved PDF reuse; cached JS, declaration and referenced JSON restored byte-identically. |
| Sequence declaration cache | A handwritten browser declaration edit caused an Nx cache miss; exact source restoration recovered the baseline task hash, and a local cache hit restored all three outputs. |
| Sequence public package | Typed consumer and runtime import passed. Normal Nx prerequisite plus full browser, protocol oracle and both examples passed. |
| GIS source oracles | Region-group 26 checks and durable three-store source/oracle checks passed; literal include audit found zero unresolved references after correction. |
| Norm source oracles | Config and thirty-app surface inventories passed with independent AJV validation. |
| Space source laws | Projection persistence 11, interactive job catalog 23, event page ownership 27, and identity rows 54 checks passed. |
| Fresh static audit | 99 artifacts plus 2 contracts, 40 TS packages, declaration-only artifact directories, dependency direction and static Nx/launch wiring confirmed. Identified persistent law routes have been corrected. |
| TypeScript aggregate | All 40 artifact package Nx test targets passed. |
| Repository package contract | Final current-source rerun passed after stdio parent mount cleanup: AJV positive/negative cases, 99 Rust artifacts, 40 TypeScript packages, complete Cargo/Nx ownership and acyclic dependencies. |
| Launch generation | Normal generator prerequisite and plugin registry generation passed; contract, artifact selector and Sequence harness entries occur exactly once, with no stale stdio Cargo target. |
| Native Nx prerequisites | Normal Forms graph/schema/UI/styling generator chain completed and reached its Cargo check; native check remains queued. |
| Remaining native/aggregate work | Current multi-artifact/parent/host checks, GIS and Norm integration tests, native single-artifact gates and final Nx native prerequisites remain pending. |

## Evidence And Limits

Detailed commands, failures, corrections and ownership lists are retained in the adjacent ticket Markdown reports, especially `📓️nx-execution.md`, `📓️multi-artifact-execution.md`, `📓️sequence-browser-package.md`, `📓️block-gis-composition.md`, `📓️norm-artifact-package-extraction.md` and `📓️final-boundary-audit.md`.

The cache and dependency-selection results establish composability and incremental task isolation. They are not a comparable before/after cold compilation benchmark. Concurrent compilation, shared source changes and a generated-output removal affected elapsed times and some raw evidence; those incidents and any required recapture are documented in `📓️integration-review.md`.
