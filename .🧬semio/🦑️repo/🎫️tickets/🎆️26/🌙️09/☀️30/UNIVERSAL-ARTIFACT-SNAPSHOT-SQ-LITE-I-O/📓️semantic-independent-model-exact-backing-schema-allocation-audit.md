# Model Exact Backing Schema Allocation Audit

Read-only current mounted source audit. No command replay or code mutation. Genuine Before29347 supplied by Root reports admitted 132045 versus aggregate requested 707120: a 575075-byte discrepancy. No attempt was made to attribute that exact numerical remainder without an allocation trace. Root After85936 remains authoritative for the newly mounted implementation.

## Concrete uncontrolled route

Model `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:129–130` currently executes check_database followed by **validate_sqlite_database_schema(database, declared_schema, control.limits())**. The second call receives copied limits, not mutable control. This is a definite uncharged successful-operation allocation path.

Framework `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs:70` builds an expected SqliteDatabase with from_schema; that constructs owned table SQL/name metadata. It builds a BTreeSet of lowercase names. Every table validation parses actual and expected SQL, then schema_matches lexes both and parses expected again. Lexer line210 constructs each token String and growing token Vec. Parser line251 clones names, allocates groups/columns, collects declared-type fragments into Vec, joins and uppercases them. Comparison line225 builds a BTreeSet of identifier positions. These temporary allocations are visible to the aggregate request observer despite being freed before return. They are not charged by RowIndex or typed owner admission.

Successful uncontrolled parsing also eagerly constructs discarded diagnostics: table schema byte checked_add(...).ok_or(ownership_limit(...)); expected table lookup .ok_or(invalid(...)); parser close.ok_or(invalid(...)). Those helpers create owned ValueError messages, including format! in this uncontrolled module. These requests occur even when the Option is Some. Failure diagnostic capacity accounting does not excuse diagnostics constructed on a successful operation.

## Paid and allocation-free routes

check_database at framework line169 traverses borrowed tables/rows/cells, checks scalar/value totals and invokes progress. Its successful traversal has no owned metadata copy. Model semantic layout is a scalar limit check. The current reconstruction uses paid RowIndex frontiers, transfer reserve for spatial/element order, parent positions/group indices, and reconstruct_text for typed strings. Empty String/Vec constructors are allocation-free. Final schema at Model line230 is copied through reconstruct_text under the same control and retained inside the guarded snapshot before the final checkpoint and take; it is not an additional uncontrolled schema clone. The original database and its construction are outside the success measurement. Returned owner retirement is outside that success measurement too.

## Existing first-party repair

Use the already exported **validate_sqlite_database_schema_controlled(database, declared_schema, ReconstructSnapshot, control)**, rather than introducing an adapter or allocating an unchecked expected database. Its defining authority is `🔁️transfer/🦀️.rs:211`; package/root exports already exist. Its lexer line67 uses NativeDecodeControl through allocation_stage; table names/columns use copy_text, token/groups/statement growth uses paid reserve/grow, comparison marks use paid reserve, duplicate names use paid indices and cancellable heap_sort. Its current successful validator route has lazy ok_or_else diagnostics, unlike the uncontrolled path. This preserves quote-aware schema authority and cancellation, and charges temporary requests cumulatively; it will increase concrete admitted requirements, so existing exact law must discover the actual required charge rather than increasing allowances or lowering assertions.

A later allocation-free schema authority could use first-party borrowed token spans and static schema descriptors, comparing actual SQL against the canonical declared SQL with the same literal/quoted identifier rules. A runtime cache initialized inside the measurement would still allocate; an unmeasured global warmup would evade the law. Any cache must be explicit retained authority with accounted construction and ownership, not a test setup shortcut. Existing controlled validation is the clean concrete current API solution.

The observer measures cumulative allocation requests, not final live bytes. Repeated parser allocations and growth replacements therefore count separately. Controlled requests should be compared with this cumulative metric, not remaining retained capacity. Failure/cancellation measurements additionally include diagnostics and retirement scaffolds and require balanced release; this does not establish bounded physical retirement for source allocations.

No runtime success or exact residual amount is claimed from this static audit.
