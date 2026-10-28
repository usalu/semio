# Physical Length Resolver Contract Peer Review

Read-only review of current 📐️geometry/📏️units/🧬️schema/🔣️.json and ifc-physical-length-units.md; no implementation yet reviewed.

## Normative factor wrapper admission

Pinned IFC4 and IFC2x3 IfcMeasureValue SELECT explicitly contain IfcLengthMeasure and IfcPositiveLengthMeasure. The latter aliases IfcLengthMeasure with SELF>0. Accept both wrappers with one finite positive numeric scalar. IFC4 additionally contains IfcNonNegativeLengthMeasure (length>=0); a positive instance is dimensionally valid. IFC2x3 does not define that wrapper. If the borrowed GeometryGraph has no schema identity, document dimensional alias resolution separately from complete version validation, or pass an explicit version/profile. Do not infer version from entity spelling. Raw untyped REAL, IFCAREAMEASURE, IFCRATIOMEASURE, malformed multiple items and nested arbitrary wrappers should fail this bounded length-factor contract.

## Assignment

IfcUnitAssignment.Units is SET[1:?] OF IfcUnit; duplicate exact references violate SET. IfcCorrectUnitAssignment also prohibits multiple non-USERDEFINED named units of the same UnitType. Thus distinct length units with equal scale are still ambiguous/invalid, not mergeable. A length-only resolver can explicitly limit validation to referenced entries and length ambiguity rather than claiming all assignment WHERE rules (unrelated pressure/monetary uniqueness).

The selected project is explicit: do not select the first of multiple projects. Reject nonproject ID, missing assignment, empty aggregate, scalar in place of aggregate, duplicate IDs anywhere in index and duplicate refs in assignment. Unused global length units must not affect selected project's resolution.

## Cursor ownership and limits

A native cursor borrowing &'a G can bind the immutable graph statically; an advance(graph) API instead needs original source identity and cannot accept a different graph between turns. TS readonly types are not immutability: capture a stable immutable graph owner or state that caller mutation invalidates cursor, with identity/version guard. Do not clone the entire graph in cursor constructor.

Separate per-advance fuel exhaustion (pending, resumable) from operation-wide maxEntities/maxChain/maxWork refusal (limited, terminal). Keep partial index/duplicate state and exact next ordinal across turns. Every assignment entry, conversion link, dimensional exponent and typed payload element consumes bounded original work. Cancellation remains observable even when grant0. Conversion stack must detect cycles by exact IDs and retain bounded memory; no recursive unbounded call chain.

Float product multiplication order can overflow/underflow despite a mathematically representable final scale (e.g. alternating extreme factors). Either explicitly refuse intermediate nonfinite/nonpositive products as bounded numerical policy, or retain exponent/mantissa product; do not claim exact arbitrary-chain resolution. Current report correctly states each product must remain finitepositive. Zero offset includes negativezero; finite nonzero offset fails, NaN cannot compare equalzero. Offset entity unsupported by2x3 must not accidentally become valid version2 data.

## Output contract gaps

Current JSON schema permits metresPerUnit on pending/invalid/cancelled because only resolved requires it; consider status-specific exclusion so consumers cannot use stale scale. IDs pattern permits aboveu64MAX and entity0/leadingzero; enforce actual exact ID range in implementation and neutral tests. Pending/cancelled reason semantics should be explicit. JSON alone cannot represent Infinity/NaN but native/TS objects can; runtime finite check remains mandatory despite exclusiveMinimum.

## Focused neutral tests

1. All16 prefixes plus absentprefix, unknown YOCTO/RONNA and malformed prefix.
2. Same chain using IFCLENGTHMEASURE and IFCPOSITIVELENGTHMEASURE; IFC4 nonnegative alias policy explicit.
3. Official inch andmm fixture projections from ifc-physical-units-audit.md, including unused global metre.
4. Two distinct equal-scale assigned LENGTHUNIT entries; repeated same ref; multiple projects selected explicitly.
5. Valid chained conversion, cycle, missing target, wrong exponent in each of7slots, fractional exponent rejected.
6. Factors0,-1,NaN,Infinity, overflow/underflow and finite verysmallscale; zero/nonzero offset.
7. Advance1 across index+assignment+chain matches one-shot; grant0 unchangedfrontier; cancelatboundary; sourcechange refusal; exact entity/chainlimit.

Validate numeric results with Decimal.js only as test oracle, and validate each neutral output through AJV. These prove scoped physical resolution, not complete EXPRESS or every IfcUnit kind.
