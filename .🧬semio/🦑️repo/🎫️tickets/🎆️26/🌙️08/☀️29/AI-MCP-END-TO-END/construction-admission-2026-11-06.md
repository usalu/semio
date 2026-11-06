# Original Construction Admission

The former constructor had no allocator gate: it ran a callback with full NativeDecodeControl and inferred allocation afterward from reported progress. The canonical boundary now retains one real NativeReceivingEffect, complete Source result and complete native child before any post can fail.

## Actual API

`admit_original_construction(cx, output, admission, demand, work, constructor) -> Result<OriginalConstructionStatus, OriginalTurnFailure>`.

The constructor receives `(&mut NativeConstructionScope, &mut Option<usize>)` and returns the original owned candidate plus actual progress/work. The scope exposes checkpoint, actual held-work view, readonly issuer identity, retained grant and narrowing only. It exposes no new allocation, charge, grant, prepare or commit operation.

The existing allocation gate was factored to lend the same control internally. Its public constructor callback gets the restricted scope. One original allocation/work/effect frame owns quote, actual primitive marker, child, callback refusal and settlement. Starting a new frame while another work action is pending is refused before quote.

Source metadata and candidate placement/work are separately quoted and withheld. The primitive sees its narrowed body grant and work, and writes the original allocator marker directly. The candidate is moved once to the caller's actual slot before gate or Source posting. Posthoc record_constructed_allocation was removed from this constructor.

Status is Pending, Ready or Failed. Failed retains complete causes, including independent Source post refusal; it is never an invented summary. `take_failure_into_step` transfers `OriginalConstructionFailure {source,child}` to an actual empty destination under Source funding. Its domain projection borrows all original native/operation/context/receipt/Source-post causes. Candidate ownership remains with its original caller.

`reset_step` is explicit and funded before another construction. Caller control phases must wait for Ready before advancing, and must reconcile a retained attempt before reusing its state. A callback must not prematurely advance an outer control phase. `close_step` retires settled metadata without semantic work; unresolved callbacks and unreceived failures remain retained.

## Evidence and limits

AJV + independent SQLite admission arithmetic and held-scope contracts pass: 2 tests, 14 assertions. Cleanup-only SQL law passes: 1 test, 6 assertions. All four prior Rust constructor laws are retained and migrated: every physical axis, distinct authentic roots, over-quote preserving actual candidate/heap, and allocator-null work. They now use the actual marker, complete failure and funded cleanup gate. Rustfmt parses the actual owner, Source glue, gate and test files (exit 1 is formatting differences).

No Rust compilation or real artifact/server readiness is claimed. Parent holds new Cargo snapshots while the mandatory retirement/projection graph is changing.

The one completed caller census found 123 Job/OS files and 245 constructor calls. Most production callers still require the new retained admission and actual marker/work propagation. Existing full-control wrapper cases are recorded in generated/construction-control-census.json; four genuinely nested/raw-charge families need their own direct gate, not a full-control escape from the constructor scope.

Actual Job facade Source remains approximately 21 files / 100KB, within unchanged caps. Source Context is approximately 25 files / 259KB after removing the obsolete manual semantic receiving-receipt fields/protocol. AppFrame receiver remains unregistered until its state/cleanup integration is coherent.

## Financial scope awaiting owner decision

The complete same-turn reserve is exposed by original_construction_reserve<O>. Body release is currently an operation demand. Lifetime release for newly owned O requires an actual held cleanup stock or a genuine bound output release hold; checking spare release alone is insufficient. Parent was asked for the existing hold owner before this is claimed safe. No implicit hold, policy increase or fabricated release receipt was added.
