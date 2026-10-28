# Typed Structured IFC Edit Admission Plan

Read-only next-gate plan for IFC4/IFC4x3 shared owned IfcValue edits. No production changes, native execution or full EXPRESS conformance claim.

## Existing Reusable Code and Limits

IFC4x3 schema/📚️catalog native and TS already implement structural matching, aliases, enum/select membership, builtin scalar types, string width, binary shape, aggregate cardinality and recursive bounds. Native private matches_value/check_bounds/check_references use hardcoded ifc4x3_catalog and a whole-snapshot entity map; TS validateIfc4x3Snapshot mirrors it. schema/⚖️validation/admission.ts adds controlled structural traversal but couples it to async ExpressEvaluator and whole FILE_SCHEMA/population admission. Do not invoke whole population/EXPRESS evaluation for one control edit or copy its matcher into another Details file.

Extract the declaration-domain core into a common source adjacent to IFC/📚️catalog, parameterized by borrowed profile catalog and reference lookup, with existing4x3 structural validator delegating to it. Source-mount via each version wrapper's catalog aliases to preserve nominal Rust identity; owned IfcValue type is already shared by4and4x3. Keep diagnostics separate from population. Existing core needs hardening while extracted: native permits dangling refs in matcher then separate wholegraph references catches them; complex target validation currently uses primary type only; bounds unique check uses expression.unique but must also enforce aggregateSET; TS REAL matcher only tests tag and needs finite-bit decoding. A strict structured edit cannot blindly reuse those permissive whole-report assumptions.

Use existing Part21 binary predicate, shared catalog alias/subtype/select queries, exact native/TS numeric representations and semantic value comparison. Do not use JSON.stringify as UNIQUE equality: EXPRESS numeric equality must treat integer1 and real1.0 consistently for NUMBER, real negativezero equal positivezero, references by exact ID, enum case-insensitively, named SELECT scalar identity deliberately, ordered LIST/ARRAY recursively, SET/BAG semantics according to declaration. Existing EXPRESS comparison engine can be reused through an extracted small equality interface if it stays version-neutral and controlled; do not start another evaluator. This scope excludes evaluating WHERE functions or population UNIQUE rules.

## Proposed Small Interface

Common schema-first ArgumentDomainPolicy/ArgumentDomainIssue and validate_argument_change(catalog,attribute,effectiveDerived,before,candidate,editedPath,lookup,tick) returning accepted/rejected issue. Inputs borrow own graph/values; callback is original caller tick with no independent context. lookup(id,tick) borrows target constituent names and reports missing/duplicate IDs explicitly. Path includes original owning attribute and nested ordinal steps, not a flattened whole graph. Diagnostics carry stable entity ID, constituent, argument and relative value path plus declaration constraint code.

Checks: markers (Derived only when effectiveDerived, Unset only optional location), primitive exact type and finite numeric domain, enum member, BOOLEAN/LOGICAL T/F/U, fixed/bounded Unicode string width and binary bitwidth, SELECT constructor admitted named member with exactly one payload, positive exact target ID+existence+any compatible complex constituent, recursive aggregate element shape, static LIST/SET/BAG lower/upper and ARRAY exactextent, SET/UNIQUE equality, depth/work limit. Dynamic bound expressions and WHERE rules remain explicitly unevaluated and must not be advertised as checked. Effective derived status comes from declaredBy/name across current complex constituents, matching provider control.

## Repair Semantics

Validate touched leaf/new subtree and all affected ancestor constraints, not unrelated invalid slots or whole-document schema. Existing incomplete unrelated attributes must not block repair. An unchanged invalid leaf may remain a no-op; changing it must satisfy its declared domain. Existing ancestor cardinality violations can remain only when operation does not worsen deficit/excess: inserting into too-short list can repair incrementally, removing from overlong list can repair incrementally. Do not introduce a new uniqueness conflict; changing one duplicate to a distinct admitted value repairs it. Bounds on untouched sibling containers are not reimposed. Whole-argument replacement must satisfy that argument domain; partial repair uses targeted nested edits.

Type-switch templates that intentionally produce required unset placeholders need an explicit draft/template outcome policy, separate from valid typed value edits. Do not silently claim these drafts validate or block later targeted repairs because other template slots remain incomplete. Raw ReplaceSource remains explicit source/import authority with physical parse/shape checks and diagnostic structural validation according to import policy; it must not become a backdoor routed through typed controls. Record this separation in UI/tool contracts rather than adding compatibility behavior.

## Existing Route Integration

Admission before capacity measurement stays cheap: bounded event encoding/value-node/path shape plus recognized typed address, no whole population scans. Definitive validation runs under original custom IfcDetailsWork after its owned argument/entity copy and before pending publication. Generic scalar, enum, whole-value, append/insert/remove/move and typed constructor changes all call the same predicate; reference decimal helpers normalize first but cannot fall through to generic edits on wrong input type. Validate resulting candidate and affected ancestors once, then publish existing SetEntityArg/SetEntityComplex primitives. Preserve pending custody, final fuel/cancel, declared inverse rows and actual sequential Store footprint/inverse authority. Errors own and close displaced/candidate values through current retained close; do not release huge locals on refusal outside receipts.

Default raw/generic artifact editing must remain unchanged; only IFC structured Details profile chooses this domain predicate. IFC4x3 native validator can keep its whole-document diagnostic wrapper while reusing extracted core. IFC2x3 later supplies its own exact Part21 value adapter; no IFC4 conversion.

## Test-First Neutral Cases

1. DerivedCoordinateSpaceDimension wholearg replacement and scalar payload both refuse; unchangedDerived is no-op.
2. IFCWALL PredefinedType STANDARD accepted, BOGUS rejected; IFC4x3-only enum literal remains version-specific. OptionalDescription may unset; requiredName/ref may not unless declared optional (do not infer from label).
3. Missing/wrongtype/zero/numerically injected reference rejected identically to decimal field; u64MAX exact compatible complex target accepted.
4. Units SET replacement[#2,#3]→[#3,#3] rejects; selfreplacement accepted; duplicate repair[#2,#2]→[#2,#3] accepted; ordinary LIST may repeat.
5. InnerCoordIndices inner UNIQUE integer[1,2,3]→[2,2,3] rejects; separate inner rings may share values. CoordList2D inner length2 insertthird rejects; outer LIST append still legal.
6. ARRAY fixed bounds noinsert/remove but admitted replacement/reorder preserveextent; optional element marker follows OF OPTIONAL rather than optional owning attribute.
7. SELECT named primitive constructor retains exact identity/onepayload; wrong member/arity/tag rejects. Unicode width counts scalars, binary unusedbits validated.
8. finite REAL/NUMBER, integer1 vsreal1 UNIQUE collision, +0/−0 collision, NaN/infinity bits reject; no arbitrary epsilon.
9. Existing unrelated requiredUnset and malformed sibling do not block valid Name repair. Too-short list incremental insertion and overlong removal repair; new violation rejects.
10. Every controlled step cancellation/fuel stop leaves graph/history unchanged, pending custody closes, exact byte/row limits hold; run actual custom work and mounted publication tests when native build gate clears.

Independent AJV can certify selected enum/staticbounds/string fixtures; IfcOpenShell compiled schema metadata or existing metadata facts certify declarations; actual EXPRESS equality oracle should validate mixed NUMBER/SET cases. These are scoped oracles, not proof of full WHERE/inverse/population semantics.
