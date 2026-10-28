# Native EXPRESS Implementation

## Implemented Source

The native IFC4x3 validation module now mounts `evaluator.rs` and `population.rs`, alongside the retained schema-admitted immutable AST. The normative `ast.rs` and `program.rs` were preserved. The canonical program digest remains `10f5e2e69fd863a7e26e85f1a24d5f4915e41a3987c99afb449e8729fc8e0b84`.

The interpreter covers all 16 expression variants and 10 statement variants retained by the AST: exact checked integer literals/arithmetic, finite REAL arithmetic, strings/binary/enumerations, distinct indeterminate values and LOGICAL UNKNOWN, identifiers/SELF, unary/binary operators, attributes/qualifiers, named calls, indices/slices, aggregate repetition, QUERY and intervals; assignment, return, conditionals, CASE, indexed/WHILE/UNTIL REPEAT with BY, blocks, ESCAPE, SKIP, calls and null statements. Declared ARRAY coercion preserves local lower bounds and result bounds. Immutable aggregate values rebuild assignment paths, while constructed entities retain private arena identity. Admitted persistent graph entities remain read-only during evaluation.

Canonical functions, entity/defined-type constructors and retained builtins execute through the same controlled authority. TYPEOF expands complete entity ancestry and defined-type alias chains. Value equality and instance equality remain distinct. IN, aggregate matching and UNIQUE components compare entity instances. LOGICAL values order FALSE < UNKNOWN < TRUE, and UNKNOWN equals itself; this differs from indeterminate `?`. Definite/possible bipartite matching prevents BAG multiplicity loss and reuse of one uncertain candidate during unordered equality/subset matching. Indeterminate membership affecting an aggregate output is explicitly incomplete (`aggregate-indeterminate`).

The native population authority performs controlled generic and structural admission before semantic validity: graph/header identity, canonical identifiers/BINARY, positive unique source IDs, finite values and reference targets; concrete entities, inherited positional counts, required/derived markers, entity/select/named/primitive domains, aggregate cardinality and implicit SET/LIST UNIQUE instance uniqueness. Typed wrappers compare their underlying scalar values for uniqueness, including positive/negative zero. Complex constituents must form a complete legal single subtype chain; effective target leaf types are established before converting references. Supplied invalid derived literals are refused before normalization can erase them.

The admitted graph receives an owned entity arena and controlled incoming-reference index. Derived and inverse attributes resolve lazily with derived-cycle checks. USEDIN preserves incoming BAG occurrences. Scalar inverses validate exactly one matching source relationship; aggregate inverses enforce their retained bounds. Validation executes entity/type WHERE rules, all derived definitions, contextual bounds (including nested ITEM paths), inverse cardinalities, UNIQUE populations and global RULE bodies/WHERE clauses. Incomplete evaluation always makes both `complete` and `valid` false. Structural errors retain source entity identity, owner and attribute; rule results retain owner/label/kind/entity identity, and evaluation failures preserve retained fragment offsets.

## Controls And Ownership

Work, owned nodes/aggregate slots/text bytes, nesting and iteration controls belong to the caller. Graph construction, conversion, field capture, inverse indexing, strings, aggregate matching, functions and population loops checkpoint real work. Wide aggregates are built incrementally rather than through a full preallocated frontier. Progress callbacks are followed by cancellation checks; validation performs a final controlled checkpoint before returning validity. There is no mutation/publication of the caller's graph.

Values use immutable standard-library Arc storage. Source entity IDs are available through `entity_id()`; private arena indices are guarded by their originating context token. `data()`, `types()`, `items()` and borrowed `scalar_text()` expose read-only results without a large eager string copy. Constructed temporary entities have no persisted source ID. No external runtime library or type dependency was added.

The native integer domain is checked signed i128. Literal, arithmetic and iteration overflow, inexact integer-to-REAL conversion and nonfinite arithmetic produce explicit incomplete errors. Integers are never silently rounded. Numeric tokens are bounded to 128 bytes, names/roles to 1,024 bytes, operators to 16 bytes, and native recursive nesting to the smaller of the caller limit and 256. These are explicit native execution limits, not claims of unbounded EXPRESS arithmetic. Existing one-time normative catalog bootstrap remains the catalog module's fixed synchronous initialization; per-population work does not call the old unbounded structural validator.

## Physical Conformance Boundary

`IfcExpressValidation.physical_conformance` is always `ExpressPhysicalConformance::NotAssessed`. The validator operates on a graph and its normalized schema semantics. It does not certify the original STEP physical encoding, including Part21 12.2.5.1 internal versus external mapping requirements for single-leaf complex chains. Schema semantic validity therefore cannot be presented as physical STEP conformance. See native-express-integration.md.

## Test-First And Cross-Implementation Evidence

Native execution test sources and shared parsed expression fixtures were written before the evaluator/population source. Authentic Rust compilation was unavailable at that point, so no native red/green execution result is claimed.

The native test module now contains 17 execution tests. It consumes the same 36 scalar vectors, five statement functions, nine matching vectors and four structural uniqueness vectors as TypeScript. Parsed Rust expression/statement AST fixtures are generated from the existing canonical parser by the retained ticket script `native-expression-cases.ts`; their source digests are independently checked by the TypeScript fixture tests against the owned AST JSON Schema. Unknown-valued slice expectations are mapped explicitly to native indeterminacy rather than the JavaScript word `undefined`.

Native tests also assert the complete retained 777 WHERE rules and 48 functions under explicitly admitted indeterminate contexts, derived zero-based ARRAYs, immutable source reads, scalar inverse duplicates, controls and overflow, complex order and physical metadata. The permanent official corpus is referenced through `official-cases.rs` (include_str references only; no source copies): all 46 files and 8,172 entities are checked for native STEP roundtrip, then 45 semantic successes and the unchanged basin example's #52 IfcPerson.Identification structural rejection.

Independent oracle evidence belongs to the same fixtures: the adversarial model's 13 named failures and the 46-source corpus outcomes were established against IfcOpenShell; constructor hierarchy vectors agree with its schema TYPEOF query. The TypeScript worker's express-fixes.md records third-party multiset checks for determinate BAG matching/domain uniqueness. That report also documents IfcOpenShell 0.8.3 scalar/statement compiler mismatches; those mismatches are not represented as successful oracle agreement here. No native result is claimed to match an oracle until native tests can execute.

## Verification Actually Run

- Combined validation, native program parity and native fixture parity suites: **89 Bun tests pass, 1,107 assertions**.
- Permanent official TypeScript corpus suite: **47 tests pass, 462 assertions**, preserving the one expected unchanged schema-invalid source.
- Focused strict TypeScript checking of the new native fixture test and its imports passes.
- rustfmt parses/formats and its check passes on eight owned native source/fixture/module files. This establishes syntax formatting only.
- `cargo test -p semio-s-artifact-stdio-ifc-4x3 --lib validation::execution_tests --no-default-features` refuses before project compilation: **Original compiler ingress requires the repository launcher and authentic enforcing store**. The build system, ingress enforcement and environment were not bypassed or changed. Native type checking and all 17 Rust execution tests therefore remain unverified.

The mounted `IFC4X3_NATIVE_EXPRESS_IMPLEMENTATION = true` records source presence. `IFC4X3_NATIVE_EXPRESS_EXECUTION = false` and its shared fixture remain unchanged as the execution-evidence status. These flags do not convert source inspection or TypeScript tests into native execution evidence.

As previously recorded, the cached repo MCP binary does not execute general scripts, and the old CLI has no script route. Focused Bun, TypeScript and rustfmt commands were necessary fallbacks. No unrelated build work was performed.

## Files

Production/test files are under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation`:

- `evaluator.rs`, `population.rs` (new native authorities)
- `🦀️.rs` (minimal module mount/reexports/evidence flag)
- `execution-tests.rs`, `expression-cases.rs`, `statement-cases.rs`, `official-cases.rs`, `binding-cases.rs` (new native tests and shared fixture references)
- `native-execution.feature`, `native-execution.test.ts`, `constructor-cases.json` (new neutral/source-parity contracts)

The existing AST/program and shared official sources were preserved. The TypeScript population scope was corrected to bind only SELF, matching the native lazy attribute authority. Ticket input scripts and all reports are retained; no temporary generated output files were created by this native implementation. Parent retains the shared ticket lifecycle and generated-folder cleanup.
## Independent Review Follow-up

The P1 effective derived binding and P2 foreign SELF findings are resolved in native-express-review-fixes.md. Native and TypeScript scopes bind SELF separately from lexical locals; inherited North2D now resolves the SubContext's derived TrueNorth. Actual full IfcOpenShell 0.9.0.post1 validation agrees on both new structurally valid cases. Native foreign context checks precede every identifier arena lookup, propagate through aggregates/constructor payloads, and prevent public raw-index reconstruction. Primitive aggregate portability remains supported. The relevant suites now pass 89 tests/1,107 assertions; all 47 official corpus tests/462 assertions remain green. Native test sources total 17, still unrun because authentic compiler ingress is blocked. Existing arithmetic, work/node/iteration and depth limits and physical NotAssessed status remain unchanged.
