# Independent Native EXPRESS Review

## Scope And Evidence

Read the native implementation and integration reports, EXPRESS fixes, evaluator/population/AST source, relevant TypeScript source, and retained IFC4X3_ADD2 EXPRESS declarations. This is a read-only source audit of the frozen implementation. No production files, build enforcement, AGENTS files, or Git state were changed. No native compilation or native execution was attempted: authentic ingress remains blocked, and the fifteen native execution tests remain unrun. TypeScript parity and rustfmt do not establish Rust type correctness or native semantic correctness.

Repository MCP tools are absent from this agent's tool inventory. Existing parent-authorized direct PowerShell/Get-Content/rg source-reading fallbacks were used; no general repository CLI route was claimed. The existing parent ticket and lifecycle were reused. No generated output or dependencies were created.

## P1: Unqualified Effective Derived Fields Are Bound To Missing Markers

`population.rs` initialize stores every effective inherited positional field, including derived `*` converted to Unknown, in `ExpressEntity.fields` (around lines 564–591). `evaluator.rs` scoped copies all those fields into scope, and Identifier returns a scope binding before attempting lazy attribute resolution (around lines 350 and 405). Therefore an unqualified reference to a redeclared inherited derived attribute returns the positional marker instead of its computed value. Explicit `SELF.attribute` takes attribute() and computes it.

Concrete retained-schema path: IfcGeometricRepresentationSubContext redeclares inherited TrueNorth as a derived value, while inherited IfcGeometricRepresentationContext.North2D is `NOT(EXISTS(TrueNorth)) OR (HIINDEX(TrueNorth.DirectionRatios) = 2)`. The native scope binds TrueNorth to Unknown; EXISTS returns false, and the rule short-circuits to TRUE without inspecting its derived direction. This can hide a failed North2D rule for a subcontext whose parent has a three-dimensional TrueNorth. Shared TypeScript population.scope also copies all marker fields, so parity tests alone conceal the same defect.

Fix at the binding boundary: keep local/parameter bindings distinct from SELF attribute resolution, or omit effective derived positional fields from the SELF binding projection. Add a neutral case proving unqualified derived access equals explicit SELF access and a structural-admitted subcontext/parent population proving inherited North2D inspects the derived TrueNorth. Native execution must still wait for authentic ingress.

## P2: Foreign SELF Can Panic Before The Ownership Check

`evaluator.rs` Identifier obtains public scope SELF and directly indexes `self.entities[*index]` while checking declared names. It does not call checked_entity first. Public ExpressScope accepts an ExpressValue obtained from another prepared evaluator. A foreign SELF with an arena index outside the recipient arena panics on indexing instead of returning entity-domain; a coincident index consults an unrelated local record before any later ownership check. Other attribute/equality paths perform checked_entity first.

Reproduction recipe for a native regression test: prepare evaluator A with at least one entity; obtain A.entity_by_id; create fresh evaluator B with an empty arena; insert A's value as B scope SELF; evaluate an unbound attribute identifier such as COORDINATES. Expected: entity-domain error, no panic. Add a second equal-index/different-context case. Check SELF ownership before all arena dereferences in Identifier.

## Other Reviewed Boundaries

Immutable Arc storage, source IDs versus arena handles, checked numeric arithmetic, matching occurrence consumption, source structural admission, incoming USEDIN occurrences, inverse cardinality, UNIQUE instance comparison, controlled final validation checkpoint, and the explicit physical NotAssessed boundary have source implementations consistent with the reports. This audit did not identify a certain Rust type error by inspection. These are source observations only; they do not establish native execution, whole-language completeness, or independent oracle agreement.

Integer-to-REAL conversion refuses inexact values; finite arithmetic and i128 overflow have explicit failures. Numeric-order source handles mixed integral/fractional and boundary values without first rounding the integer. Rule Unknown is retained separately from incomplete and does not automatically make semantic validity false, consistent with the existing TS result model; callers must display the outcome honestly. Conditional aggregate output membership remains explicitly incomplete.

The findings were sent to the parent and native owner promptly. Production is available for the owner to apply fixes; this reviewer has no concurrent production edits.
