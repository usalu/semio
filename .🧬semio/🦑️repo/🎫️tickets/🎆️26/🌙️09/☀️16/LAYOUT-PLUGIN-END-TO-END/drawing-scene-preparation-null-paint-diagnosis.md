# Preparation Null Paint and Type Diagnosis

Both actual runs are terminal and all151 inputs are unchanged. Owning28000:22pass/2fail/947724; existingScene18 allpass. Both failures stop in canonical parseDrawingArtifact on the original final preparation fixture's reference-parent attributes.fill:null before its job is constructed. This is not a geometry tolerance/credit failure. Strict71000 has exactly one missing relocated first-party type import: DrawingPathSegment at defining owned preparation line32. No suppression/cast or body change is needed for that import.

## Defining evidence

- Canonical TypeScript DrawingAttributes currently declares optional nonnullable fill/stroke. Its ownedAttributes parser sends every defined value, including null, to object validation.
- The untouched original seventeenth preparation row (boolean parent reference preserves independent result movement) contains explicit fill:null and stroke:null on reference-parent and expects no paint; the former numeric producer intentionally treated these as no paint.
- Native DrawingAttributes has Option<FillStyle>/Option<StrokeStyle>, which admits no-paint semantics.
- The first-party fill editor's none variant returns null, and DrawingDiff's fill/stroke value interfaces and closed schema explicitly allow null.
- Canonical artifact JSON Schema is inconsistent: attributes declares nonnullable stroke and fillRule but omits fill entirely, although TypeScript/parser/actual producers implement it. The diff currently contains the full nullable DrawingFillValue union.

## Proposed coherent correction

Preserve explicit null through canonical TypeScript DrawingAttributes and ownedAttributes for both fields (no fixture stripping/coercion, no omission rewrite). Retain existing full object validation for every nonnull value; bad scalars/arrays remain refused. Add a schema-first genuine parser law distinguishing omitted versus explicit-null fields and hostile scalar/array values, including the untouched original row and independent Ajv projection. Add only the erased first-party DrawingPathSegment import for strict.

For the artifact schema, move the existing genuine fill value union and its color/stop definitions to canonical artifact FillStyle rather than adding an unvalidated object allowance or a dependency from artifact to diff. Attribute fill/stroke reference canonical value definitions with explicit null alternatives; diff DrawingFillValue then references canonical artifact FillStyle (diff already has artifact schema dependencies). Preserve all numeric/style limits and bilingual UI metadata. This is a defining schema/type/parser consistency fix, not an accepted rendering-threshold/fixture/stock change. Root should review the exact bounded schema movement before execution; current151 remains held until release.
