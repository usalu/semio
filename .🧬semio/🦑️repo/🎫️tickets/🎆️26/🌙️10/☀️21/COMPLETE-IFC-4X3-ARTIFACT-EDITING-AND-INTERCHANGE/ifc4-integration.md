# IFC4 Integration State

## Current Ownership

Coordinator owns ordered diff algebra in Rust/TypeScript, wire/schema alignment and IFC TypeScript suite registration. High execution agent owns ten mutation runtimes, complex-edit details, native mutation framing and schema facets. High I/O agent owns strict STEP boundaries and shared format-neutral TypeScript Part21 parsing. High geometry agent owns representation projection in both languages and viewer/editor render cancellation. Remote agents own IFC4x3 and build refactoring; their concurrent source and test-suite entries are preserved.

## Outstanding Validation

- Compile and run native IFC4/model/mutation/viewer/editor and independent ruststep/IfcOpenShell suites after build storage is repaired.
- Execute TypeScript Part21 official fixture/malformed framing tests once the shared parser lands.
- Execute TypeScript geometry projections against all neutral fixtures and official models once the new adapter lands.
- Check strict compiler types and all source include paths; retain official fixture checksums unchanged.
- Register each permanent TypeScript suite in the existing IFC package router, preserving all remote IFC4x3 suites.

## Remaining Goal Scope

Full EXPRESS/MVD rule compliance is not established by graph validation or official sample roundtrips. Real geometry remains a declared subset; unsupported operations must produce diagnostics instead of substitute geometry. Prepared-view projection resumption, richer visible geometry status, full IFC2x3 real geometry and broader artifact interchange remain to be assessed before claiming full IFC experience. Full layout/PDF/InDesign/SVG work remains outside the active IFC focus.

## Public TypeScript API

The IFC facade now explicitly exposes IFC4 apply/inverse/absorb, mutation diff/apply/inverse, and parseIfc4Step/writeIfc4Step. Existing IFC4x3 exports and SQLite APIs remain present. Source test suite entries are added through the existing IFC package suite router.

## Public Facade Compiler Check

The public IFC artifact facade was checked after exporting IFC4 runtimes. Its only diagnostic was a missing pre-existing workspace module @semio-tech/framework-interaction from framework replication/wire/local-interaction, outside this task scope. The IFC4 diff/mutation/STEP/geometry modules each have separate strict checks with zero diagnostics. Root package compilation is not claimed to pass while that workspace dependency is absent.
