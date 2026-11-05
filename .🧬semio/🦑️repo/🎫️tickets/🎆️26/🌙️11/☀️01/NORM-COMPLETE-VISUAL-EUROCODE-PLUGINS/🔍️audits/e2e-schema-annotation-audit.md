# Norm Schema Annotation Integrity Audit

Date: 2026-11-04. Read-only source traversal of 2,884 JSON Schema documents. No validator mass compilation.

## Unsatisfiable Patch Schemas

EN1993 `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json` has two definitions, `En1993JointPlatePatch` and `En1993FoundationBlockPatch`, whose required list contains `x`, `y`, `z`, `yaw` while none of those fields exists in properties. Both reject additional properties. Therefore every instance is invalid: omitting those fields violates required and including them violates additionalProperties.

EN1995 at the same diff suffix has `En1995DeckPatch`, which requires ten absent fields: `kind`, `memberId`, `stabilisedCount`, `spacingM`, `lengthM`, `braceWidthM`, `braceDepthM`, `compressionForceN`, `resistanceN`, `stiffnessNPerM`. It likewise rejects additional properties and is unsatisfiable. The retained required fields appear copied from another record; reconcile the native patch model and remove spurious required metadata before regenerating twins.

## Keyword Census

Top-level standard JSON Schema keyword checking found only the oracle step vocabulary file `🔮️oracles/🥒️steps/🧬️schema/🔣️.json` mixing schema and instance data: unknown schema keywords `schemaVersion` and `families` hold the actual vocabulary. This is incompatible with strict unknown-keyword admission unless the consumer deliberately strips them. Determine whether to split the instance from its schema or explicitly document its intentional custom admission. No other top-level unexpected keyword, or misplaced array of property names, was found.

Recursive required-to-properties checking found only the three patch definitions above. Recursive constant-property checking across artifact schemas found no discriminant property with a const value omitted from its enclosing required array. This does not establish codec correctness for aliases, singleton enums, or non-constant discriminants.
