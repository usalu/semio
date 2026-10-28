# Retained EXPRESS Machine Source Checkpoint

This checkpoint is ready for independent read-only semantic review. The source already contains retained expression, statement, domain and population continuations; it has not been compiled or executed by the authentic native route. Template publication still explicitly refuses after receiving. No template admission or native execution flag has been enabled.

## Current Representation And Custody

The implementation is under the new IFC4x3 `🧬️schema/⚖️validation` directory. `machine.rs` owns nonrecursive values, linked aggregate edges, lexical scopes/bindings, constructed entity fields, normalized original coordinates, derived memo entries, domain metadata, computed text owners, continuation frames and borrowed rule records. Original source strings remain coordinates in the actual receiver/overlay/source; computed strings are individually retained in the same Store. No copied candidate snapshot, recursive cold evaluator or additional runtime library is introduced.

The environment pins the actual overlay, receiver, snapshot and immutable mutation pointer/length, with the overlay checking original source ownership before each transition. Metadata and canonical AST are immutable borrowed static assets. `program-index.rs` schedules indices of the canonical arrays without reordering them. The full canonical SHA256 is `10f5e2e69fd863a7e26e85f1a24d5f4915e41a3987c99afb449e8729fc8e0b84`.

Each actual continuation checks original cancellation, yield/fuel and item/copy/capacity/release/depth loans. Actual table births/reallocations occur as separate transitions with exact contiguous capacity requests, old backing release and reallocation copy demands. This remains a real allocator frontier requiring an adequate contiguous loan. Semantic work uses retained indices rather than recursive execution. The close path owns the complete Store through the existing controlled retirement interface; the source/receiver/overlay owners outlive that close.

## Explicit Instruction Surface

The following table documents source branches, rather than claiming their semantic correctness has been executed.

| Expression kinds | Source behavior and refusal boundary |
| --- | --- |
| Integer, Real | Parse bounded numeric text; checked i128 integer arithmetic and finite binary64 values; malformed, overflow and precision-losing conversions refuse. |
| String, Logical, Unknown | Borrow static literal text; preserve three logical values and Missing. Logical literal spelling must be recognized. |
| Identifier | Incremental scope/binding and SELF attribute lookup, fixed catalog declaration/enum lookup, PI; unresolved names refuse. |
| Unary | NOT and signed numeric values; unrecognized operators and invalid operands refuse. |
| Binary | Explicit logical, equality/instance equality, relational, membership, constructor concatenation, arithmetic and aggregate branches. Unsupported operators refuse. Unresolved output membership in aggregate union/difference/intersection refuses with `express.aggregate-indeterminate`; no normative behavior is guessed for that gap. |
| Attribute, Qualify | Explicit owner selection, derived memo/cycle handling, inverse routing and normalized source/constructed field access; malformed/cyclic/unbound context refuses or propagates Missing according to the current admitted branch. |
| Call | All ten builtins actually used by the released program have explicit branches: ABS, BLENGTH, EXISTS, HIINDEX, LOINDEX, NVL, SIZEOF, SQRT, TYPEOF and USEDIN. Functions, entity constructors and named type wrappers retain their own cursor. Unknown calls and arity errors refuse. |
| Index, Slice | Explicit aggregate/string positions and UTF-8 scalar scans; out-of-range/missing values propagate Missing; malformed AST refuses. |
| Aggregate | Incremental linked literal entries and checked repetition; negative/noninteger repetition refuses. |
| Query | One source candidate and local predicate continuation at a time; only True predicates enter the result. Current scalar-source fallback follows the cold reference's empty-item behavior; this is specifically an audit boundary, not a newly established normative rule. |
| Interval | Separate lower/middle/upper evaluation and two explicit comparisons joined with three-valued AND. |

All 16 expression enum variants have exhaustive source arms. The canonical program uses 15 of them; Slice is an additional represented AST kind. The executed immutable-program scan counts 5,533 identifiers, 1,722 attributes, 640 qualifiers, 2,868 binary expressions, 1,359 integers, 1,684 calls, 73 aggregate literals, 137 reals, 339 indices, 354 unary expressions, 90 logicals, 122 queries, 455 strings, 145 unknowns and 13 intervals.

| Statement kinds | Source behavior and refusal boundary |
| --- | --- |
| Assign | Incremental binding lookup/coercion and aggregate-path reconstruction; constructed entity field assignment; unsupported target forms, original identity mutation and bad positions refuse. |
| Return, Call | Evaluate once and retain/discard the result through explicit control flow. Function parameters, locals and result coercion have distinct stages. |
| If, Case | Explicit branch predicate or label equality continuations; only True chooses a branch. |
| Repeat | Retained index/start/end/increment, WHILE/UNTIL and iteration continuations, checked index arithmetic and iteration ceiling. |
| Block | Retained body cursor under actual lexical scope. |
| Escape, Skip, Null | Explicit flow/no-op branches. |

All ten statement enum variants have exhaustive source arms. The canonical program uses Assign (208), Return (197), If (137), Repeat (30), Block (43), Escape (2) and Case (4); Skip, statement Call and Null remain explicit represented branches rather than a default fallthrough.

| Population lane | Source behavior |
| --- | --- |
| Header and normalization | Incremental value/header checks, complex-chain coordinates and inherited field normalization; malformed names/counts/chains/derived markers refuse. |
| Domain and aliases/selects | Primitive/string-width/binary-width, entity references, declared enum membership, nested aggregates, fixed bounds, optional positions and alias/select traversal. Domain failure produces terminal incomplete admission, never valid output. |
| WHERE and type rules | Inherited entity owner indices, alias/selected-type scopes and nested aggregate element lanes. |
| Derived and inverse | Memoized derived attribute continuation with explicit cycle refusal; original population reference scan, role filtering and inverse cardinality checks. |
| Contextual bounds | Explicit attribute/path collection, evaluated bound and per-value result continuations. |
| UNIQUE | Per-rule population candidate and tuple comparison history; instance comparisons retain original identity. |
| Globals | Actual per-type populations, locals, statement bodies and global WHERE continuations. |

Canonical counts remain 777 WHERE, 60 derived, 165 inverse, six contextual bounds, four UNIQUE declarations, 48 functions and two globals. All lanes are scheduled by source; completeness/correctness of their results still needs independent native execution and full differential review.

## Terminal-State Correction

The continuation review confirmed two terminal-state defects. A failed population initialization recorded an incomplete result but left initialization false, so a caller saw pending and re-entered initialization. A scalar action refusal had no terminal latch, so a caller could continue reducing its remaining frames after the original refusal.

Test-first neutral `machine-terminal.json` and native custody tests now cover these paths. `machine.rs` records a failed semantic state before returning scalar errors; later scalar advances refuse and scalar output is unavailable. A population semantic failure returns a terminal incomplete/invalid report on that same turn, preserving exactly one borrowed admission diagnostic and retaining all Store owners for controlled close. Successful and incomplete terminal advances still check original source authority/cancellation/funding but do not grow tables, increment work or debit the original receipt again.

The result contract `machine.json` now rejects `complete=false, valid=true`, and rejects any incomplete record in either results or diagnostics when complete is true. A borrowed diagnostic or partial receiving/result state cannot authorize publication.

## Authored Native Tests

`machine-custody-tests.rs` is a child test module of the real machine, so its assertions can inspect the actual retained owners without adding production inspection API. Its four tests cover terminal initialization refusal; scalar refusal/retry; every cancelled population prefix, denied item/copy/capacity/release/depth loans and foreign source identity; and a canonical positive direction population with an actual original argument replacement, rejected equivalent foreign mutation allocation, stable borrowed records, zero terminal receipt delta and cancellation after completion.

Every observed transition compares native requested/released heap bytes with the same original receipt. Controlled machine close deliberately denies inadequate release loans before granting actual close; receiver and overlay retire afterwards. Existing `machine-tests.rs` adds five scalar/function/cancellation/grant/contract tests, and `machine-model-tests.rs` adds five official/inherited-derived/named-failure/template/complex tests. These 14 retained-machine native tests are authored source only, not executed evidence.

## Executed Verification

- `bun test native-program.test.ts`: **2 pass, 0 fail, 22 assertions**. This executes canonical AST/digest checking and Ajv 2020 result-contract validation, including rejection of inconsistent completeness/validity in either record lane.
- `retained-program-index.ts --check`: complete canonical digest, 584 direct owners and 876 inherited entity index rows match exactly, with canonical arrays unchanged.
- `retained-coverage.ts`: complete canonical instruction/builtin scan succeeds; generated source-audit result remains under `🗑️generated/retained-express/surface.json`.
- `retained-oracle.py` using bundled Python 3.12 and IfcOpenShell 0.8.3: all 876 entities, 436 types and 6,004 inherited attribute names/optionality/derived positions match; maximum released hierarchy depth is nine; the neutral retained foundation schema validates. This reruns declaration/fixture evidence, not native semantic output.
- Rustfmt parsed/formatted the retained machine files and checked the final machine/test syntax. This does not type check Rust or execute receipts.
- Authentic `cargo test --manifest-path <4x3 rust package>/Cargo.toml --lib machine` refuses before rustc during its compiler version probe: **Original compiler ingress requires the repository launcher and authentic enforcing store**. No guard, alternate compiler, synthetic store or build configuration was changed.

## Audit And Admission Boundaries

Native compiler/type/borrow proof, actual heap/copy/fuel/cancellation measurements, scheduler fairness, complete rule semantics and genuine registered Store/revision/history publication remain unproved. Unknown aggregate output membership remains explicitly incomplete. Existing cold-reference permissive fallbacks (such as scalar QUERY) and full original/constructed/qualified type semantics require independent semantic review rather than being inferred correct from shared code shape. Runtime semantic-error diagnostics currently retain a borrowed admission code with no originating owner/rule/offset; ordinary evaluated rule records preserve their owners/rules/source offsets.

`admission.rs` remains receiving followed by explicit `template.semantic-admission-unavailable`. `IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION`, `NATIVE_TEMPLATE_SEMANTIC_ADMISSION` and native execution flags remain false. All template planners, TS AST/program arrays, factory chooser/exchange changes, AGENTS files, Git state and enforcing infrastructure were preserved.

This continuation changed `machine.rs`, added `machine-custody-tests.rs` and `machine-terminal.json`, tightened `machine.json`, extended registered `native-program.test.ts`, and retained this report plus `retained-machine-continuation-plan.md`. Original machine files were rustfmt-parsed; no cold evaluator rewrite or duplication occurred. Shared generated dependencies/output are intentionally retained for the fleet. The active ticket/full-spec objective remains open; no lifecycle action was taken.
