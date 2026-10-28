# Core Native Shape Fixes

## Result

The two final review shape gaps are resolved comprehensively. A command without `setEntityName.name` is refused before planning or history publication. A present `fileDescription:null` is refused instead of silently ignored. Native snapshot, mutation and sparse diff admission now derive their required fields, discriminants, object/array containers, allowed properties and coordinate scalar domains from the checked-in IFC4x3 JSON Schemas.

The new dependency-free `native-shape.ts` interpreter reads only these owned schemas. Its lazy generator keeps one iterator per container, validates owned captures before normalization/planning, and cooperates with the existing cancellation/progress driver. Existing semantic admission remains responsible for identifier/binary content, reference integrity, duplicate/overlapping targets, finite binary64 words and resource budgets. It is an internal schema subset interpreter, not a general JSON Schema implementation or an exported third-party validator interface.

Optional native fields with `undefined` retain their existing omission semantics; present null and wrong containers/types are refused. Required undefined is refused. Unknown properties are refused even when undefined. Arrays reject holes and undefined items. The shared optional entity `complex` field remains optional in the schema and normalizes to an empty owned collection only after shape admission. Coordinate refusals preserve the `invalidIndex` error contract.

The graph identity check now uses `IFC4X3_GRAPH_SCHEMA = stdio.ifc`. The host envelope identity is independently `IFC4X3_DOCUMENT_SCHEMA = s.stdio.ifc@4x3/*`; the new regression checks both identities. No root facade/API change is needed for the shape interpreter.

## Test-First Evidence

The language-neutral editing feature and `shape-cases.json` fixtures were added before the shape implementation. The initial new suite failed all three differential suites, including malformed history publication at revision 1 and accepted null/object containers.

The new tests derive mutations from every declared property of all eleven command branches, every header/entity/list diff lane, and snapshot/header/entity/value structures. For each field they generate missing, undefined, null and wrong-type cases; they also generate unknown discriminants, undeclared properties (including undefined), holes, undefined array items and invalid/nonfinite coordinates. Tests compare synchronous and asynchronous command planning/application/inverse/history, diff absorption/application and snapshot/history creation against Ajv using the actual checked-in schemas. Native bigint leaves are independently represented by a test-only Ajv bigint-domain keyword; unchanged wire schemas independently verify every accepted recorded command after decimal JSON serialization. Refused history commands retain revision 0, empty events and the original graph. All tested sources remain unchanged.

The existing fast-json-patch differential output and inverse/composition laws remain in the focused suite.

## Verification

- Combined editing suite: **35 tests pass, 5,126 assertions**, 0 failures.
- Focused strict TypeScript check of both production editing modules, native shape checker and both editing test files passes.
- `core-final-repro.ts` passes: required command name and null array lane are refused; progress phase alias probes are safely refused or return valid owned data.
- `core-ownership-repro.ts` passes: four ownership/domain refusals, revision 0 after refusal and consistent recorded-command replay.
- `core-repro.ts` passes 1,864 composition/inverse cases; numeric binary refused; maxValues 3 refuses a 100,000-child aggregate with 0 child reads; result mutation does not change source.
- Historical attack-only `core-reaudit-repro.ts` now exits on an uncaught `invalidValue` refusal during owned decimal-leaf admission. It previously demonstrated acceptance; this is the intended rejection and is explicitly covered by the passing ownership tests/repro above.

Focused commands use Bun and strict `bunx tsc --noEmit --strict --target ES2022 --module ESNext --moduleResolution Bundler --allowImportingTsExtensions --skipLibCheck --types bun`. As previously recorded, the cached repo MCP binary offers no general script execution route, and the old CLI cannot execute these scripts. These necessary focused fallbacks do not alter build infrastructure. No whole build was attempted; the independent framework dependency/build ingress limitations remain outside this fix.

## Files

All production and feature/test files are under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any`:

- `🧬️schema/native-shape.ts` (new internal native schema shape authority)
- `🧬️schema/🔺️diff/🟦️.ts` (owned shape admission; graph identity constant)
- `🧬️schema/🧬️mutations/🟦️.ts` (owned required command admission)
- `🧪️tests/🧬️editing/shape.test.ts` (new executable differential test suite)
- `🧪️tests/🧬️editing/shape-cases.json` (new language-neutral fixtures)
- `🧪️tests/🧬️editing/🥒️.feature` (language-neutral acceptance scenarios)

Ticket scripts `core-shape-tests.py` and `core-shape-runtime.py` retain the test-first construction steps. No generated output files were created. Parent retains ownership of the shared ticket lifecycle and root facade integration.