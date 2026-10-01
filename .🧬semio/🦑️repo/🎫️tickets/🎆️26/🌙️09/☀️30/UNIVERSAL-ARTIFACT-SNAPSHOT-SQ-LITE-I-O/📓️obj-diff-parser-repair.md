# OBJ Diff Parser Repair

The registered OBJ snapshot suite exposed 26 adjacent diff-parser TypeScript diagnostics after source line positions became exact bigint values. Six triple parsers previously passed raw object records into typed geometry slots, two sparse geometry helpers were absent, and omitted vectors produced undefined despite the actual Rust owned vectors defaulting to empty.

The diff module now calls the existing owned vertex, texture-coordinate, normal, face, group and object validators for added geometry. Omitted removed/modified/added collections become empty arrays. Explicit sparse vertex and texture-coordinate parsers preserve optional fields and nullable-weight clearing. The complete owned diff parser preserves bigint source ordinals through the full u64 range.

A language-neutral fixture and two Bun tests cover omitted-vector behavior, malformed nested geometry values and relationships, nullable clearing and maximum unsigned source positions. The permanent existing OBJ package script now registers this focused parser suite, so its public package check reaches the repaired code.

Verified checks: the existing OBJ artifact TypeScript script test passed two tests and seventeen assertions; its build reported three outputs and one public export. Fresh registered bun nx run @semio-tech/stdio-obj:check passed in 3.3 seconds with one suite checked, and bun nx run @semio-tech/stdio-obj:test passed in 5.7 seconds with two tests and seventeen assertions. A previous broader snapshot-suite check encountered an unrelated missing declaration for the framework native-host JavaScript implementation; that is not reported as an OBJ parser failure. The root owns the broader geometry snapshot target.
