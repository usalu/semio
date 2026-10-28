# IFC4x3 EXPRESS Implementation

The owned TypeScript frontend and interpreter execute the retained IFC4X3_ADD2 expressions and statements. This establishes executable schema infrastructure and specific tested semantics; it does not establish complete IFC4x3 conformance. The catalog capability flags remain unchanged.

## Source and canonical program

The retained official EXPRESS source is the catalog's IFC4X3_ADD2.exp, SHA256 `f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860`. Its canonical URL is https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp and the downloaded official buildingSMART mirror is https://raw.githubusercontent.com/buildingSMART/Standards-builds/main/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp.

The new `🧬️schema/⚖️validation` domain contains `ast.schema.json`, `ast.ts`, `parser.ts`, `compiler.ts`, `evaluator.ts`, `population.ts`, and `index.ts`. Native ast.rs/program.rs and their generator are owned by the parent worker. No AST shape changed after that native generation.

| Retained declaration lane | Compiled count | Execution evidence |
|---|---:|---|
| Entity declarations | 876 | Named runtime environments for every entity |
| Named types | 436 | Complete catalog plus type WHERE traversal |
| Entity/type WHERE clauses | 777 | Every clause executes with explicit indeterminate input; applicable clauses on official/adversarial models |
| Derived attributes | 60 | Lazy evaluation; every applicable derived attribute is also evaluated explicitly |
| Functions | 48 | Every function executes with indeterminate parameters; meaningful geometry/dimension/array paths in fixture validation |
| Global RULE declarations | 2 | Both execute after population bindings and local statements |
| Inverse declarations | 165 | Incoming-reference index, subtype/role filtering, declared cardinalities, scalar exactly-one |
| Dynamic aggregate bounds | 6 | All six executed for rational curve/surface ARRAY fixtures |
| UNIQUE clauses | 4 | Named tuple comparison; authored duplicate application and root identifiers |

The full program passes the owned canonical AST JSON Schema through existing Ajv. Compilation reports no unsupported source fragments. Canonical AST parsing covers qualifiers, qualified enum literals, queries, aggregate repetition, intervals, exact integer literals, function calls, assignments, IF/CASE/BEGIN/REPEAT/RETURN/ESCAPE/SKIP and LOCAL declarations. No external runtime library is introduced.

## Execution behavior

The interpreter owns its values: bigint INTEGER; binary64-backed JavaScript numeric arithmetic inside the semantic evaluator; distinct indeterminate values and three-valued LOGICAL; string/binary/enumeration; LIST/SET/BAG/ARRAY with lower indices; identity-bearing entity values and composed constructor fields. Snapshot REAL leaves remain the existing Binary64 graph values. Integer-to-real equality checks the represented real's exact integral value, including representable values above 2^53. Finite numeric overflow produces an explicit incomplete evaluation error.

Aggregate assignment and function arguments copy aggregate values recursively while entity identities remain references. Local declared ARRAY types control assignment indices; unbounded ARRAY return types preserve existing lower indices. Qualified attributes read current composed fields after assignment. Schema-qualified enumeration literals resolve enum domain members; they are not treated as missing entity attributes.

The population entry first captures and admits an owned graph with the existing controlled core, normalizes valid complex constituents forming one complete concrete IFC subtype chain, and checks structural domains. It then evaluates inherited entity/type WHERE clauses, derived values, inverse cardinalities, all applicable dynamic bounds, UNIQUE tuples and both global rules. Unknown LOGICAL outcomes remain explicit named results. FALSE produces a rule diagnostic; unsupported constructs, numeric limitations, cyclic derived evaluation and exhausted budgets produce incomplete diagnostics and cannot yield valid:true. Required scalar inverses reject both zero and multiple referring relationships.

Caller controls include AbortSignal, work/depth/iteration budgets, scheduling batches and progress. Compilation, owned graph capture, graph conversion/indexing, aggregate copy/comparison, statement/function execution and inverse lookups have bounded checkpoints. The reused catalog structural validator is synchronous; this is a remaining responsiveness boundary requiring review before any blanket interactive-control claim. Generic EXPRESS constructs outside the retained ADD2 corpus are not advertised as a complete general-purpose EXPRESS implementation.

## Verification

`validation.test.ts`: **42 tests passed, 875 assertions**, Bun 1.4.2. Strict tsc of the public semantic index passed with ES2023, preserve modules and bundler resolution. Neutral contracts are `validation.feature`, `cases.json` (25 scalar cases), `models.json` (15-entity adversarial STEP envelope/body), and `statements.json` (five imperative function cases). The root package runner registration is owned by the parent.

Independent IfcOpenShell **0.8.3**, bundled Python 3.12, `express_rules=True`:

| Model | Own named failures | Oracle named failures | Difference |
|---|---:|---:|---:|
| Official triangulated, 35 entities | 0 | 0 | 0 |
| Official alignment, 3161 entities | 0 | 0 | 0 |
| Official structural, 188 entities | 0 | 0 | 0 |
| Authored adversarial, 15 entities | 13 | 13 | 0 |

The oracle's inverse diagnostic names the concrete inherited attribute, whereas our canonical identity names its declaring entity; the differential resolves declaration ownership before comparing entity IDs and rule names. Two adversarial oracle failures contain internal constructor/list errors for AxisToRefDirPosition and WeightValuesGreaterZero. We match their failing rule identities, not those implementation errors. Our expected negative geometry/weight outcomes also have direct assertions.

An additional isolated-clause differential constructs every concrete USERDEFINED PredefinedType environment: 317 entities, **291 CorrectPredefinedType failures in both implementations, zero missing/extra**. Other mandatory attributes are deliberately unset, so this checks those named clauses and does not claim those complete models are valid.

Independent standalone EXPRESS compilation has significant limitations in this IfcOpenShell version. Of the initial 24 scalar vectors, 16 match; it differs/errors for UNKNOWN OR TRUE, NOT UNKNOWN, QUERY with UNKNOWN, aggregate repetition, DIV compilation, exponentiation, and Unicode source literals. Inspection shows generated `[1:3,2]` drops the second entry, and `2 ** 10` becomes `210`. Of five imperative fixtures, ARRAYFLOW matches; CONTROLFLOW and WHILEFLOW fail compilation, CASEFLOW returns -1 rather than 4, and UNTILFLOW returns 0 rather than 8. These discrepancies are retained as oracle limitations; our expected semantics were not changed to reproduce them. This oracle cannot establish full three-valued/control-flow conformance.

## Reproduction and remaining work

Retained ticket input scripts: `compile-express.ts`, `evaluate-express.ts`, `adversarial-express.ts`, `express-coverage.ts`, `express-enums.ts`, `express-oracle.py`, `express-differential.py`, `express-scalar-oracle.py`, `express-statements-oracle.py`. Generated programs/results and test-only Python dependencies remain under the ticket's generated directory for the parent to remove at ticket completion. The existing repo binary has no general execution route, so the necessary Bun/Python/tsc fallbacks remain documented rather than pretending repo executed them.

Public facade exports are explicit: compileIfc4x3Express, validateIfc4x3Express, parseExpressExpression, evaluateExpressExpression, ExpressEvaluator, ExpressEvaluationError, ExpressParser, ExpressSyntaxError and owned AST/value/control/result types. Low-level IfcExpressContext is available in its domain module for isolated tests; the public validation entry performs graph/domain admission.

Remaining evidence: independent audit of semantic branches, particularly aggregate multiset/equality corner cases and invalid/indeterminate model behavior; a stronger independent oracle for the standalone language cases; control-aware structural admission; native interpreter integration/testing by the native worker; authentic repository compiler ingress before claiming any native execution. All-clause indeterminate execution proves dispatch coverage, not every branch. No conformance flags or MVD scope metadata were changed by this implementation.
