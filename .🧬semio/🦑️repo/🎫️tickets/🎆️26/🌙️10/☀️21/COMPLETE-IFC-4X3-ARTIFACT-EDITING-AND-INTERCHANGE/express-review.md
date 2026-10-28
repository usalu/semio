# Independent EXPRESS Semantic Review

Scope: read-only TypeScript audit, 2026-10-21. No production edits, native execution, capability promotion, ticket lifecycle changes, or build bypass. Applicable AGENTS at root, s, and stdio were read. Direct read/Bun/Python fallbacks were necessary because the available repo ingress does not expose general execution. Focused tests: Bun 1.4.2, **42 passed, 875 assertions** (`validation.test.ts`). No P1 was established by this review.

## Reproduced P2 Findings

1. **BAG subset ignores multiplicity.** `ExpressEvaluator.binary` aggregate `<=`/`>=` searches the full counterpart for each element without consuming matches. BAG `[1,1] <= [1,2]` returns TRUE; it must be FALSE because the right bag contains only one occurrence of 1. Both directions share this defect. Retained IFC rules currently use aggregates heavily; the standalone semantic API also exposes this behavior. This is a direct branch-level reproducer, not proof that an official IFC fixture is misclassified. A third-party EXPRESS BAG operator corroboration has not yet been obtained; do not claim a differential for this case.

2. **Constructor TYPEOF loses entity and defined-type ancestry.** `TYPEOF(IFCCARTESIANPOINT([0.,0.,0.]))` returns only IFCCARTESIANPOINT, omitting IFCPOINT, IFCGEOMETRICREPRESENTATIONITEM, and IFCREPRESENTATIONITEM. `TYPEOF(IFCPOSITIVELENGTHMEASURE(1.))` omits IFCLENGTHMEASURE. The constructor paths in `call` attach the direct type only; TYPEOF simply reads that list. Snapshot entity initialization and named value conversion already traverse ancestry, so identical semantic values behave differently depending on origin. **Independently corroborated** by IfcOpenShell 0.8.3, Python 3.12: its TYPEOF returns all four entity names and both scalar aliases. Its helper omits primitive REAL/NUMBER, so that difference is not being promoted as a finding.

3. **Aggregate relations collapse indeterminate comparisons into determinate answers.** SET `[?] <= [1]` returns FALSE because its matcher recognizes only TRUE and discards UNKNOWN. Intersection SET `[?] * [1]` produces a determinate empty SET. These branches should preserve the inability to determine membership (or explicitly report unsupported/incomplete) rather than assert absence. This needs normative/stronger independent corroboration before declaring full language conformance; the existing IfcOpenShell standalone oracle has documented indeterminate handling defects. No official-population false positive was established.

## Additional Observations and Limits

- Out-of-bounds slice `[1,2,3][0:1]` returns an empty LIST through JavaScript negative-index slicing. Retain as an untriaged general-expression issue pending explicit language contract and independent oracle.
- Structural admission still invokes synchronous `validateIfc4x3Snapshot`; caller cancellation cannot interrupt it once entered. This matches the implementation report's stated limitation. Owned graph capture precedes mutation; no caller-snapshot ownership violation was established.
- UNIQUE rules in this retained schema use strings/tuples (application identifier, application full name/version, enumeration name, root global ID). This review found no entity-identity UNIQUE defect in the retained four rules.
- Complex normalization accepts a complete single subtype chain and flattens it to its concrete leaf. The parent flagged Part 21 edition 3 section 12.2.5.1's internal-mapping requirement for a single leaf. Physical encoding conformance belongs at the STEP ingress boundary and should not be claimed from semantic normalization alone. This review did not independently prove an invalid wire encoding becomes valid because the semantic API receives a graph, not original encoding.
- All-clause execution with indeterminate inputs and source/hash completeness remain dispatch evidence, not semantic branch completeness. The focused test suite passes despite the reproduced gaps above.
- No comprehensive independent oracle is available for every aggregate/logical branch. No new blanket complete-IFC conformance claim is supported.

## Reproduction

Retained inputs: `review-probes.ts`, `review-oracle.py`. Outputs: `🗑️generated/review-probes.json`, `🗑️generated/review-oracle.json` (parent removes generated outputs on ticket completion). Run Bun on `review-probes.ts` from repository root. Run `review-oracle.py` with `C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe`; it imports the existing ticket-generated IfcOpenShell test dependency without modifying it. Both scripts are independent of production test assertions.

IfcOpenShell source helper inspected: test-only `ifcopenshell/express/rules/IFC4X3_ADD2.py`, `typeof`, lines 85 onward, iterates entity supertypes and defined type aliases. Relevant physical-format reference supplied by parent: https://steptools.com/stds/step/IS_final_p21e3.html . Public library docs establish entity supertype traversal but do not independently validate BAG/indeterminate cases: https://www.steptools.com/docs/javalib/javadoc/com/steptools/stdev/EntityDomain.html .
