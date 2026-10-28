# EXPRESS Finite Control and Quantum Review

Read-only policy/source checkpoint while root edits validation controls. No production source was changed or imported by the census probe; validation files remain free of this agent's live Bun mappings. New policy tests have not been executed here against in-progress code.

## Existing policy evidence

ExpressControl is optional signal,batchSize,maximumWork,maximumDepth,maximumIterations,progress. Existing tests assert work-budget refusal at10, TYPEOF work-budget at4, actual structural cancellation and returned full-validation incomplete/invalid on genuine structural cancellation. They do not define zero budgets or invalid nonfinite/fractional controls. Keep caller-thrown same-class errors distinct from generated cancellation; genuine cancellation need not be changed into an identical endpoint behavior merely to preserve callback failures.

Before refactor, compiler defaults batch32/work20000000; evaluator defaults batch2048/work10000000/depth256/iterations1000000; structural domain depth defaults64. Supplied finite values must be captured once. Default only undefined; null must not silently select defaults.

Recommended closed domains: batch positive safe integer with effective cap4096; depth nonnegative integer up to256; work/iterations nonnegative safe integers. NaN/±Infinity/fractional/negative values are invalid. The exact reject-versus-clamp treatment above the batch cap must be explicit in root's schema rather than inferred from a helper in another domain. Zero work means first charged work cannot proceed. Zero iterations means the first REPEAT cannot iterate, while an expression requiring no iterations may succeed. Zero depth denies expression entry; it is a budget denial, not necessarily malformed control. Compiler partial-program inspection versus complete compilation remains its explicit API contract.

Capture all scalar fields and original signal/callback once before caller code. Freeze or privately retain the captured record, including when exposing evaluator.control: readonly TypeScript fields alone do not prevent runtime alias mutation. Keep the original AbortSignal object, whose legitimate aborted state can change; do not snapshot its boolean or substitute a caller's later signal. Batch checks must not become modulo0/NaN yielding no callbacks.

## Depth propagation concern

Before root's change, compiler uses supplied depth for rule/function/global ExpressParser construction but invokes parseExpressExpression with no supplied limit for derived, unique, bound and inverse expressions. parser.ts154 constructs a default-depth256 parser. Ensure the new captured depth reaches every parser path before claiming a universal compile depth cap. This is a concrete source propagation gap, reported to root while its fix remains active, not a finding against an announced stable patch.

## Released job census and bounded claim

Authored express-compiler-job-census.ts reads only the existing immutable catalog and emits lengths/counts, never source copies. Actual released catalog census:1062jobs,172607source code units,876entities,436types,maximum21 inherited explicit attributes. Jobs:752entity WHERE,25type WHERE,60derived,165inverse,4unique,6contextual bounds,48functions,2global rules. Largest source job is IfcShapeRepresentationTypes7001code units, then IfcCorrectDimensions4340 and IfcDimensionsForSIUnit3385. Exact metadata is generated/express-compiler-job-census.json.

Current compiler builds all job descriptors synchronously before the first hook. Each job's lexer/parser is synchronous; lexer's generic token ceiling is2000000, not an original per-token turn law. For this pinned schema, a truthful bound is at most7001 source code units per released parser job plus bounded static descriptor work. It is not proof that arbitrary public parser input yields per token, that caller cancellation interrupts a running job, or that TypeScript allocations have native wallet receipts. The census would need regeneration if the pinned schema changes.

## Population-sized remaining work

Entity/value conversion has evaluator ticks at rows and recursive values; simple all-attribute loops have a released bound21. Complex normalization caps constituent count below876 and has fixed declaration chains, but uses synchronous small metadata maps. Source-sized global population spread/filter and inverse scalar-cardinality spread/filter/map/Set remain cold passes; incoming-edge dedup includes linear includes inside one edge tick. They are separate continuation frontiers, not resolved by replacing yield's host scheduler or validating finite limits. The validation report currently has no separately observable final-report phase. Keep bounded-control correctness, source ownership and full semantic/native completeness as distinct claims.