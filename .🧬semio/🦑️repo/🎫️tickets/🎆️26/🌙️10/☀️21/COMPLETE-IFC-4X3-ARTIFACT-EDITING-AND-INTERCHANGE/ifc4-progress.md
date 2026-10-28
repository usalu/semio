> Continuation evidence and current completion gates: [ifc4-continuation.md](ifc4-continuation.md), [ifc-feature-completion-gates.md](ifc-feature-completion-gates.md). The report below records the earlier batch; new shared geometry/catalog/details work supersedes its current-state counts and gaps.

# IFC4 Feature Completion Progress

This batch implements existing IFC4 behavior while remote agents continue IFC4x3 and the repository build refactor. The goal and shared ticket remain open.

## Implemented

- Exact ordered sparse diffs in Rust and TypeScript, checked central application, diff/mutation inverses, same-id relocation, and strict binary framing.
- Ten executable TypeScript mutation kinds, complex constituent edits through the existing adjustable snapshot details, matching schema facets, full-width integer transports, and omitted optional-field handling.
- Strict STEP framing/header/id/reference validation, shared dependency-free TypeScript Part21 codec, real IFC4 import/export leaf functions, unchanged pinned buildingSMART fixtures, and one operation-wide cancellable progress budget.
- Real representation geometry shared by IFC4 viewer/editor in Rust and executable TypeScript, mapped/local placements, supported tessellations/BReps/extrusions, explicit unsupported/invalid reference diagnostics, real product identity, budgets and original render-context cancellation.
- Native supplementary Unicode fix and cross-language Unicode vectors with an independent codec reference.
- Public IFC4 runtime exports and all six permanent TypeScript suite registrations, preserving concurrent IFC4x3 APIs and suite entries.

## Executed Verification

All 34 TypeScript test groups passed with no diagnostics in the directly checked IFC4 modules: diff 5, mutations 4, STEP I/O 5, geometry 16, Unicode 1, integer precision 3. This includes 1,043 edit pairs, 588 edit triples, 1,075 independent JSON Patch comparisons, all ten mutation undo paths, official fixture checksums/counts/full snapshot roundtrips, 22 malformed/valid STEP boundary cases, exact signed/unsigned integers, binary values, surrogate handling, monotonic operation progress, cancellation/yield and geometry limits. Existing test-only references include fast-json-patch, AJV, GraphQL, protobufjs and iconv-lite.

## Verification Still Required

Both repository-routed IFC Rust and TypeScript package commands fail before tests at native cache allocation: Windows refuses per-directory case sensitivity (access denied). Native source regressions and the ruststep/IfcOpenShell differential suites have been authored but have not executed. The public facade compiler also encounters an unrelated missing @semio-tech/framework-interaction dependency in framework replication. No native build or UI end-to-end success is claimed.

## Remaining Goal Scope

Full EXPRESS/MVD rule compliance is not established. The geometry implementation intentionally refuses unsupported profiles, holes and booleans instead of substituting geometry; prepared-view projection resumption remains incomplete. IFC2x3 real surfaces and broader artifact interchange remain for further work. The broader layout/PDF/InDesign/SVG requirements are outside this active IFC focus and are not claimed complete.

## Detailed Evidence

- [Plan and ownership](ifc4-plan.md)
- [Diff laws](ifc4-diff-algebra.md)
- [Mutations and complex UI](ifc4-complex.md)
- [STEP and official fixtures](ifc4-conformance.md)
- [Geometry and limits](ifc4-geometry.md)
- [Exact integer transports](ifc4-integer-precision.md)
- [Unicode](ifc4-unicode.md)
- [Final read-only audit](ifc4-final-audit.md)

No modifying git command, worktree or AGENTS edit was used. Reusable input scripts and audit reports are retained in this ticket. Temporary transformed modules for this batch are removed separately without deleting concurrent IFC4x3 outputs.

Path audit: all 501 IFC4 source-tree entries fit the 256-character limit; longest absolute path is 189 characters.

Cleanup completed for this fleet: all transformed diff/STEP/geometry/Unicode/integer modules were removed. Concurrent IFC4x3 generated inputs/oracles/results were preserved because the shared ticket is still active. Reusable scripts, fixtures and Markdown audits remain.
