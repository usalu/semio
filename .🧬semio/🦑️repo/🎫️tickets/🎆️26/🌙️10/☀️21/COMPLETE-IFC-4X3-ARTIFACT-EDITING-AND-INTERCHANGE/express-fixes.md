# EXPRESS Review Fixes

The two reproduced P2 defects are fixed: BAG subset/superset consumes occurrences, and constructor TYPEOF includes entity supertypes and defined type aliases. Partial aggregate matching no longer asserts FALSE or an empty result merely because membership comparisons are indeterminate. Full semantic structural admission now runs controlled domain checks instead of calling the synchronous catalog validator.

## Decisions and implementation

BAG and mixed SET/BAG subset matching uses bounded bipartite matching. A complete matching through definite edges proves TRUE; only a possible matching through UNKNOWN edges produces UNKNOWN; no possible complete matching proves FALSE. The same matching authority handles unordered aggregate equality, preventing multiple elements from reusing one possible counterpart. Entity membership, aggregate arithmetic and UNIQUE tuples use instance comparison; explicit value equality remains distinct.

Constructor ancestry follows entity supertypes and defined type aliases. LOGICAL UNKNOWN is a concrete value, so UNKNOWN=UNKNOWN is TRUE; ordering is FALSE<UNKNOWN<TRUE. An indeterminate operand remains different from LOGICAL UNKNOWN. Pratt precedence now places unary operators above exponentiation and places AND/OR at the standard arithmetic precedence levels. Out-of-range/reversed slices return indeterminate values; controlled string scanning avoids whole-string Array.from allocation.

Normative basis: ISO 10303-11:2004 clauses 12.6.5–12.6.6 specify occurrence counts for BAG subset/superset. Clause 12.2.2 defaults entity comparisons to instance identity; 12.2.3 uses that identity for membership. Clause 12.2.1.3 defines LOGICAL ordering. Table 10 places unary operators above exponentiation. The cited text is the [identical GOST adoption of ISO 10303-11:2004](https://internet-law.ru/documents/prod/gost-r_gosudarstvennyj-standart/44/gost_26329.html), whose preface identifies the adoption as identical. [ISO identifies the underlying standard](https://committee.iso.org/standard/38047.html?browse=ics). This is a primary standards text reproduced on a third-party host; it is not presented as an official ISO web edition.

A remaining explicit limitation concerns partial aggregate **output** membership: union/difference/intersection throw `aggregate-indeterminate` when an UNKNOWN comparison would determine whether a member is emitted. The current semantic value representation cannot describe conditional output membership. This refusal prevents false conformance but is not a completed general-language feature. SET/BAG simple domains do not ordinarily permit unset elements; probes with such partial values are conservative engine tests, not proof of valid schema-domain instances. No blanket conformance flags were promoted.

## Controlled structural admission

New `admission.ts` exposes `validateIfc4x3Domains` over owned shape-admitted graphs. It performs actual asynchronous type/select/alias traversal, cardinality checks, instance uniqueness, reference checks, width scans and binary validation, with cancellation/depth/work controls. Progress uses the structure phase and measures those operations; no synchronous catalog validation call remains in population.ts. Existing graph capture still provides bounded shape admission and ownership before this domain pass.

Implicit SET uniqueness is checked, as well as explicit UNIQUE aggregate declarations. References compare entity identity; defined scalar values compare their represented values, including positive and negative zero. String equality and width scans use bounded checkpoints. Semantic complex normalization now checkpoints its traversal, bounds constituent counts by the complete schema, and refuses a literal in an effectively derived inherited position before normalization can erase it.

Public facade adds the owned `validateIfc4x3Domains` API. Full `validateIfc4x3Express` remains the entry that captures/admits input and aggregates named semantic diagnostics. Domain, normalization and evaluation budgets remain phase-specific; the work field reports semantic evaluation work, not a combined lifetime instruction count.

## Evidence

Test-first reprobes initially failed for the reviewed matching/TYPEOF behavior, logical comparison, precedence, indeterminate operands and out-of-bounds slices. Final focused suite: **81 tests passed, 944 assertions**. Strict tsc of the semantic index passed. Canonical AST compilation still covers 777 WHERE clauses, 60 derived attributes, 48 functions, 2 global rules, 165 inverses, 6 dynamic bounds and 4 UNIQUE declarations without diagnostics. The parent regenerated the native canonical program after the precedence change; the official corpus AST hash was unchanged.

Language-neutral contracts now include 36 scalar cases, nine matching cases, four structural domain cases, five imperative statement cases, the 15-entity adversarial model, and the existing nine shared structural cases. Tests explicitly cancel during a large type traversal and cancel through the full validator's real structural phase. They also reject erased complex derived literals and compare identity uniqueness against distinct equal entities.

Independent checks:

- Python multiset 3.2.0 matches all five determinate BAG/mixed SET subset/superset vectors.
- IfcOpenShell 0.8.3 TYPEOF independently returns all four point entity type names and both length-measure aliases.
- For all four new uniqueness domain vectors, IfcOpenShell independently parses the actual values and multiset independently counts their represented identities/scalars; expected uniqueness decisions agree. IfcOpenShell's own structural validator emits no diagnostics for the three duplicate vectors. That validator omission is retained explicitly rather than described as validation agreement.
- Existing official fixtures retain zero semantic diagnostics; the 15-entity adversarial model retains the same 13 named failures.
- Parent reran the permanent 46-file official ADD2 corpus after the admission rewrite: 47 tests/462 assertions passed; 45 valid files and the same single structural error remain unchanged. See official-corpus-results.md for exact pinned inputs and independent whole-corpus results.

Retained inputs: express-fix-probes.ts, express-fix-oracle.py, express-domain-oracle.py; neutral fixture files and tests are production assets in the validation domain. Outputs and test-only dependencies remain in ticket/🗑️generated for final ticket cleanup. Direct Bun/Python/tsc fallbacks remain necessary because the available repo binary exposes no general execution route. No AGENTS, Git state, old IFC4 implementation or build infrastructure was changed.

Native parity was communicated to ifc_binary. Native compilation/execution remains subject to authentic enforcing-store ingress; this report makes no native execution claim. Stronger independent evidence for conditional aggregate outputs and full semantic branch coverage remains required before any full-conformance capability claim.
