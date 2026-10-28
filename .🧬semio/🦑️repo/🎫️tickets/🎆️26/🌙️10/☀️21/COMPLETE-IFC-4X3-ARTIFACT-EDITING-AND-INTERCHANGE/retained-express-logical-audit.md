# Retained EXPRESS Logical Independent Audit

Read-only targeted source checkpoint of evaluator.ts, parser.ts, population.ts and native machine-expression.rs, machine-operation.rs, machine-statement.rs, machine-rules.rs. No new confirmed canonical-program P1/P2 was established in this pass. This is a bounded checkpoint, not approval of the retained engine; the native admission gate remains refusal.

## Actual program and probes

The ticket input retained-logical-probe.ts compiles the current pinned program and records its expression/statement surface in generated/retained-logical-program.json. Compilation reports complete. The actual tree contains 122 QUERY nodes, 30 REPEAT statements, two ESCAPE statements, 747 OR, 396 AND, eight XOR, 676 value-equality and 26 identity comparison nodes. Thus the audit addresses used operators rather than an unused generic language inventory.

The same script records 34 actual reference TypeScript results in generated/retained-logical-probe.json: all 27 AND/OR/XOR truth-table combinations plus logical UNKNOWN equality, indeterminate equality, UNKNOWN and numeric QUERY predicates, doubled-apostrophe literal escaping, aggregate repetition and an exact large-integer/mixed-real relation. UNKNOWN = UNKNOWN produces TRUE whereas ? = ? produces UNKNOWN; QUERY retains only TRUE predicates; the three-element repeated initializer has size three. These runs establish current reference behavior, not independent normative proof and not native execution. Node 24.14.1 required its ordinary experimental TypeScript transform option; no source copy or compiler-ingress bypass was used.

## Source findings

Both implementations distinguish a valid logical UNKNOWN value from an indeterminate/missing value. Native logical enum ordering is False, Unknown, True and reference ordering uses the same sequence. AND/OR preserve their decisive FALSE/TRUE cases and otherwise propagate uncertainty; XOR propagates uncertainty. Logical short-circuit conditions agree. Equality/containment and unordered matching retain uncertainty rather than treating it as a positive match. Native numeric ordering explicitly compares integral and fractional components instead of rounding the integer operand through floating point.

QUERY creates a child binding and retains only a TRUE predicate; native stages retain cursor/result ownership across turns. Aggregate repetition rejects negative/nonintegral counts and preserves indeterminate repeat input. Parser doubled apostrophes are decoded before string evaluation. REPEAT checks WHILE before the body, processes RETURN/ESCAPE before UNTIL, and advances its index after the body/UNTIL boundary. Native continuations use the common original-authority demand/debit/cancellation path; this source reading cannot verify runtime receipts.

Type-WHERE propagation walks aggregate items, evaluates named defined-type rules, then follows their underlying expression. For a SELECT attribute it uses the actual runtime type seed. No SELECT declaration with its own WHERE clause was found in the pinned EXPRESS roster, so the selection replacement does not demonstrate a skipped canonical SELECT rule. This does not prove arbitrary imported type tags or all alias chains correct.

## Explicit unresolved interfaces

Both QUERY implementations currently turn an ordinary nonaggregate scalar population into an empty LIST; native does so explicitly in machine-expression.rs stage 1, and reference uses items(). This remains a permissive cold-reference compatibility behavior, not evidence of conforming scalar QUERY semantics. No legal pinned IFC expression plus admitted model establishing a wrong result from this behavior was isolated here.

Enumeration relational comparison is textual in both implementations. Declaration-order requirements and a reachable canonical-program counterexample were not independently authenticated in this pass; it is an open generic EXPRESS boundary, not a confirmed IFC finding. Likewise this checkpoint does not settle ARRAY QUERY result classification, SKIP with UNTIL, arbitrary enum-domain incompatibility or non-BMP string relational ordering.

The official [IfcLogical definition](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcLogical.htm) establishes TRUE/FALSE/UNKNOWN as logical members. It does not independently establish every operator proposition above. Earlier translated ISO text remains secondary evidence and is not elevated to authenticated normative authority here. Full language conformance still needs authenticated propositions and independent outputs for the unresolved interfaces.

Native source was not type-checked or executed: authentic compiler ingress refuses before rustc. No synthetic runtime claim, broad gate approval or duplicate full-suite run is made.
