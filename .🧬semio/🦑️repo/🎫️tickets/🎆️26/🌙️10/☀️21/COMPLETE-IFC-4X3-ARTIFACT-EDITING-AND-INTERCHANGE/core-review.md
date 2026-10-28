# IFC4x3 Core and Shared BINARY Review

Read-only audit of the new IFC4x3 TypeScript graph editing/catalog surface and shared BINARY integration, 2026-10-21. No production sources, AGENTS, Git state, compiler/store enforcement or build infrastructure changed. Direct read/Bun tools were necessary under the coordinator's recorded unavailable `repo` execution route. `core-repro.ts` is retained as a small reproducible input; no generated output files were created.

## Actionable Findings

### P1 — Value budgets and cooperative cancellation do not bound wide aggregate preparation

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts:26`, `:32`, `:36`, `:56`.

Validation gathers all root values, builds one stack entry per root, and pushes every aggregate child before checking subsequent values. A single entity can therefore allocate/traverse millions of children within one generator step, even with a tiny `maxValues` limit. Async scheduling outside `steps.next()` cannot interrupt that work. Reproduction uses a 100,000-item aggregate with a Proxy counting numeric child reads: `applyIfcDiff(graph, {}, {maxValues:3})` reads **all 100,000 children** before returning `valueLimit`. A cursor frame per aggregate with bounded child advancement fixes both budget admission and yielding; copying the complete children list into another stack does not. Header-root expansion and per-entity args/complex loops require the same treatment. Eager `structuredClone` calls at `:15`, `:72`, `:91`, `:94` also execute outside bounded traversal and can allocate a large proposed replacement before result validation rejects it.

### P1 — Public command/history operations cannot report progress or cancel expensive work

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts:71`, `:76`, `:80`, `:83`, `:86`, `:90`.

`IfcEditHistory` construction, execute, undo and redo only call synchronous graph application and offer no signal/progress/options argument. Execute performs a complete forward application through `inverseIfcDiff`, then applies the same diff again through `#publish`, in addition to cloning event payloads. On allowed million-entity inputs these public editing actions block the UI for multiple complete traversals. `applyIfcMutation` is likewise synchronous only. Add async command/history operations using the same bounded authority; validate the expected revision again immediately before publishing after an await, and update snapshot/events/undo/redo only on successful completion. Cancellation must leave all four unchanged.

### P2 — Mutable result aliases the caller's original graph

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts:77`, `:89`, `:95`.

Unmodified entities, argument arrays, complex constituents and header roots are returned by reference. Public graph interfaces are mutable, so callers can unknowingly modify the original snapshot through the returned result. Repro: `const after=applyIfcDiff(before,{}); after.entities[0].args[0].value=99n` changes `before` too. Decide explicitly whether this graph API guarantees owned results or immutable persistent graphs; enforce that contract with bounded cloning or immutable types/objects and tests. The current history class defensively clones its public getter, so the observation specifically concerns the direct applier.

### Corrected during review — Native scalar coercion

The coordinator corrected the initially reproduced acceptance of `{kind:'binary',value:0}` by adding a string guard, and hardened EXPRESS identifier checks against array/object coercion. Rerunning the retained repro now produces `invalidValue` for the numeric binary input. This finding is resolved in current shared files.

## Positive Evidence and Limits

- `core-repro.ts` checks **1,864** sequential argument combinations against direct application, including removals, multiple insertions, second-step removal/modification and inverse restoration. All pass; no coordinate/composition bug was found in these combinations.
- History publication computes its next snapshot and cloned event before changing internal state; ordinary validation failures do not consume undo/redo entries. Async revisions need a fresh check if async APIs are added.
- Current transport references use IFC4x3 snapshot `$defs`, preserve decimal bigint transport, and keep runtime domain validation distinct from wire validation. No mismatch was found in the inspected definitions.
- The retained EXPRESS catalog and declared limitations were inspected. No additional concrete catalog declaration mismatch was established. Pairwise aggregate uniqueness plus JSON serialization is expensive and its deep/native-value safety is already identified in `schema-implementation.md`; no claim of complete semantic rule validation is made here.
- Shared BINARY inspection confirms distinct scalar arms and canonical padding checks in Part21 lexer/native decode/writer, IFC/STEP projections, IFC4 SQLite native projection/reconstruction, semantic admission and backing census. The Rust canonical-tree kind table uses index 9 consistently. STEP references use a wildcard scalar arm, so that inspected consumer remains exhaustive with the new variant.
- Rust compilation **remains unverified**, because the enforcing compiler ingress is blocked as recorded by the coordinator. No attempt was made to bypass it. An exploratory source-only match scanner (`core-rust-scan.py`) was interrupted because the repository-wide scan was too broad; it is not compile evidence and yields no exhaustive-match conclusion. No additional compile defect was proven in this audit.

## Reproduce

Run `bun` with the explicit ticket-relative `core-repro.ts` path from the workspace root. Output is prefixed `[DEBUG]`; no output file is needed. The script currently confirms the corrected binary refusal, 1,864 passing law combinations, eager 100,000-child reads under `maxValues:3`, and result/base aliasing.
