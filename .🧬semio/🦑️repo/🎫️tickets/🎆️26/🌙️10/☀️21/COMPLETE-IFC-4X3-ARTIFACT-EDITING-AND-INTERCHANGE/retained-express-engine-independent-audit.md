# Independent Retained EXPRESS Engine Audit

## Decision

Keep semantic template admission and publication refused. This independent source checkpoint found no additional confirmed P1/P2 in the reviewed continuation/terminal changes. It is not a complete independent semantic review of the engine and does not authorize enabling its gate. Native compilation and execution remain unproved.

Read-only review on 2026-10-21, coordinated with the retained implementation worker after source edits paused. Read the continuation plan and retained-machine-checkpoint.md, actual machine.rs and machine-expression.rs, relevant machine-operation.rs/machine-statement.rs/machine-population.rs/machine-domain.rs/machine-rules.rs branches, source receiving/retained funding helpers, and test declarations/contracts. This extends the historical foundation audit rather than replacing its boundaries. No production edits, native compiler bypass, lifecycle actions or shared generated cleanup occurred. No independent native test execution is claimed.

## Terminal And Source Authority

The new failed latch is checked by advance/demands/result_scalar. A scalar semantic action error records failure before returning its refusal, so popped failing frames cannot expose a successful partial scalar on a retry. Population initialization/action failures set complete=false and valid=false, append the borrowed incomplete admission code, clear pending tasks, reset flow, and finish on the same call. report additionally computes validity as complete AND population_valid.

Finished advances validate the original environment before their no-debit terminal path and still pass cancellation/yield/funding admission. demands returns a zero-copy fixed demand for completed work; no table growth or work increment occurs on that branch. result_scalar requires original environment identity, initialized state, empty task stack and no failure. These are source observations, not executed receipt proofs.

Environment identity includes overlay, receiver, source object, mutation pointer and mutation length. Overlay authority verifies original source storage identity. This deliberately does not hash mutable source contents; the enclosing operation must preserve its captured immutable source and mutations. The machine accepts a supplied StepContext per transition and relies on the enclosing original operation to supply its initiating authority. No replacement context is constructed by the inspected engine.

## Instruction And Population Coverage

The expression dispatcher has explicit branches for all represented enum variants and delegates to retained frames. Statements have explicit assignment/return/call/body/IF/CASE/REPEAT/ESCAPE/SKIP/NULL flow. Functions stage arguments, parameter binding, local declarations, body and result coercion. Aggregate index assignment reconstructs a retained path rather than rewriting the borrowed source; original entity attribute mutation refuses.

Calls contain explicit branches for the ten canonical builtins, plus bounded canonical function lookup, constructors and named wrappers. Unknown operations/calls refuse instead of silently succeeding. Numeric operations check integer overflow/precision loss and finite real results. Text comparisons/copies and string scalar scans use continuations. Primitive arithmetic and fixed metadata/path traversals remain per-transition bounded source work rather than recursively invoking the cold evaluator.

Population initialization normalizes retained source fields and domain coordinates. The receiver checks actual argument count against released metadata before normalization. Domain lanes check primitives, widths, enum members, entity/select compatibility, aggregate cardinality and nested item domains. WHERE/type/derived/context-bound/inverse/UNIQUE/global lanes have actual scheduling branches, scopes and result records. Static metadata index scheduling uses canonical arrays rather than replacing them with another program.

A suspected contextual bound equality issue was checked and not established: the six current dynamic bound rules apply to derived ARRAY bounds (control points/weights), where actual index equality is appropriate; this is not a counterexample involving LIST minimum/maximum cardinality.

Source coverage counts and exhaustive enum arms do not establish correct EXPRESS semantics. Full matching/aggregate uncertainty, constructed/qualified type behavior, selects/alias propagation, contextual rule provenance, generic coercion and inverse/USEDIN behavior still require complete independent normative and differential verification. In particular the documented scalar QUERY empty-source fallback is not independently approved, and aggregate-indeterminate output membership is an explicit refusal boundary. The implementation report correctly identifies these rather than treating them as conformance proof.

## Funding, Retention And Close

The retained helper checks cancellation, yield and all five loan axes before transitions; debit consumes the supplied context's retained progress and fuel. Machine growth computes old-copy, new-capacity and old-release demands and performs each table growth in a separate transition. Semantic actions use a conservative fixed copy frontier; computed text capacity is separately quoted at TextCopy birth. Stores retain nonrecursive arena vectors and individually owned strings for controlled retirement.

begin_close blocks result publication/continuation. close_demands/close_step use RetainedCloneClose over the actual Store, retaining partial close ownership until empty. Native tests are authored to retire machine before receiver and overlay. Actual allocation-versus-receipt conservation, failure after debit, cancellation prefixes, same-recipient closure and peak heap are not verified by inspecting those calls. Rustfmt parsing is not Rust type/borrow checking.

The four new custody tests target initialization refusal, scalar retry refusal, cancelled population prefixes with five denied axes/foreign source, and terminal mutation-identity/zero receipt delta. Other authored tests cover scalar/function fixtures, populations, template goldens and complex binding. Their bodies remain unexecuted native evidence. AJV/canonical program checks and IfcOpenShell declaration comparisons recorded by the worker concern contracts/metadata, not native semantic output.

## Remaining Gate Requirements

Finish independent semantic review of all engine branches and released program uses; compile through authentic enforcing ingress; run all native scalar/function/population/custody cases; compare genuine native results and named diagnostics with independent EXPRESS/IfcOpenShell outputs; measure actual allocation/copy/release/fuel behavior; and prove real Store revision/history publication. No native-produced result marker or external oracle output was generated during this audit. The explicit template.semantic-admission-unavailable gate remains required.
