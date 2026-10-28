# Native EXPRESS Host Integration

## Owned Controlled Query API

The package already mounts public `validation`. The new explicit module reexports are:

- `prepare_ifc4x3_express(snapshot, control) -> ExpressResult<ExpressEvaluator>`
- `validate_ifc4x3_express(snapshot, control) -> IfcExpressValidation`
- `validate_ifc4x3_express_with_program(snapshot, program, control)`
- `ExpressEvaluator`, `ExpressControl`, `ExpressProgress`, `ExpressResult`, `ExpressEvaluationError`, `ExpressScope`
- `ExpressValue`, `ExpressData`, `ExpressLogical`, `ExpressAggregateKind`
- `IfcExpressValidation`, `ExpressDiagnostic`, `ExpressRuleResult`, `ExpressRuleKind`, `ExpressRuleOutcome`, `ExpressPhysicalConformance`

`prepare_ifc4x3_express` performs structural admission and owns the converted graph/index once. It does not run the whole semantic rule population. Its returned evaluator supports `entity_by_id(id)` and `attribute_by_id(id,name)` for read-only explicit, derived and inverse fields. General `attribute(value,name)` can follow returned temporary computed entities in the same context. Context-owned source entities remain immutable. Canonical temporary constructors can assign only their own arena records.

`ExpressValue` is immutable and uses standard-library Arc ownership. `entity_id()` identifies a persisted source reference; temporary constructed entities return None. `types()`, `data()`, `items()`, `integer()`, `lower()` and `aggregate_kind()` support rendering. `scalar_text()` borrows existing strings and only formats small scalar numbers. Private arena indexes are transient handles, never persisted IDs. Cross-context entity use is rejected by the originating context token.

Full validation owns its diagnostics/results and does not retain borrowed caller graph payloads. Diagnostics carry source entity ID, owner/rule or structural attribute where available. AST `offset` values identify the retained declaration/expression fragment, not a physical STEP byte offset. Bounds/uniqueness results retain owner and label even when no AST byte offset is needed.

## Host Scheduling And Cache Boundary

Create and retain a prepared context in a background job/worker for one immutable graph revision. Cache derived/inverse query results by revision, source entity ID and attribute. Avoid preparing a context or running full semantic validation during a details-panel paint. A graph revision change invalidates that context and all computed results; construct the next context from the newly admitted graph.

Bind caller-owned cancellation and progress to the host's existing job interface. The synchronous CPU interpreter checkpoints real construction/evaluation work; the host schedules that work away from interactive rendering. Callback-bearing contexts can be created on their owning worker, while immutable value/report payloads can be transferred independently. Budget counters are cumulative for that prepared context. Choose explicit worker/job budgets; do not reset evidence or publish partial results after a refusal.

When the job completes, the host must recheck its graph revision and cancellation before publishing a report/computed-field update through existing events. The validator itself publishes no host state. This keeps stale revision results and cancelled preparation out of the details projection without introducing a new host framework or persistence format.

Render computed values read-only. Their source identity links to existing graph details; generated entity values remain contextual computed results. Derived/inverse fields do not become serialized positional attributes or mutation targets. The existing edit authority remains responsible for commands, inverse/history and shared publication.

## Conformance And Evidence

Expose semantic completeness and semantic validity separately. An execution/domain/control failure sets both false. A completed FALSE rule produces complete true, valid false. Indeterminate LOGICAL results retain their own rule outcome instead of being silently coerced to true or false.

The result explicitly reports `physical_conformance: NotAssessed`. A normalized legal EXPRESS subtype chain does not establish that its original STEP complex encoding satisfies Part21 internal/external mapping. The physical source validator and the schema/MVD validator have separate evidence boundaries. No MVD, physical STEP or geometry conformance is implied by this semantic result.

Native implementation source is present; native execution is unverified because authentic compiler ingress is blocked before compilation. Keep `IFC4X3_NATIVE_EXPRESS_EXECUTION` false until native build/tests and the independent oracle comparison actually run. The report native-express-implementation.md lists the exact available evidence and all native limits. No shared host framework or details implementation was changed in this task.
## Reviewed Binding And Handle Boundary

Entity scopes contain SELF only. Unqualified effective attributes resolve lazily through that entity after normal lexical local/parameter/QUERY bindings. The inherited derived TrueNorth/North2D case is now checked against actual full IfcOpenShell validation; direct and qualified reads agree. Native values containing entity references retain their context dependency even inside aggregates. Queries or constructors receiving a foreign handle return entity-domain before arena access. Primitive-only aggregates remain portable. Public raw Entity(index) reconstruction is refused; preserve immutable value handles with their owning prepared context/revision, or request the source identity from the new context. See native-express-review-fixes.md for test-first and oracle evidence. The 17 native execution tests remain unrun under blocked compiler ingress; current relevant TypeScript suites pass 89 tests/1,107 assertions and the 47 official corpus tests/462 assertions remain green.

