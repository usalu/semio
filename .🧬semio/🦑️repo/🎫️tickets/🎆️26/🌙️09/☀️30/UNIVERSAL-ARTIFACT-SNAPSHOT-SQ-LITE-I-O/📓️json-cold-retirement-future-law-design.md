# JSON Cold Retirement Future Law Design

Read-only refreshed source inventory,2026-10-03. No implementation or tests mounted; no Cargo. Root's authentic Native20 baseline remains the current task. Its shallow allocation fixture proves neither allocator-refused retirement nor deep partial-cancellation lifecycle.

## Actual Representation and Defect

JsonValue is Null, Bool, Number{lexeme:String}, String{value:String}, Array{items:Vec<JsonValue>}, Object{members:Vec<JsonMember>}; JsonMember contains key:String and value:JsonValue. Exact ordered duplicate object keys are owned strings and must release alongside child values. No model fields need to be added for retirement.

Current pack decoding `retire_value` creates `vec![value]`, then extends that frontier with child arrays or object member values. The initial Vec is an actual allocation; extending can reallocate. Items/Members/Values partial reconstruction guards call this function. It avoids recursive Rust drop for normal deep state, but allocator refusal during cancellation cleanup can abort before the guard finishes. This is source evidence, not a claimed executed JSON lifecycle failure.

## Existing Authority and Fit Constraints

Mounted CommonMark uses popped inactive node slots for linked continuation headers, compile-time size/alignment fit guards, and no callbacks/reserves during retirement. Its actual narrow64KiB,depth8192,width4096 test checks zero attempted allocations, zero remaining tracked owned bytes, and strict linear visits. HTML has its own neutral lifecycle corpus and isolated public retirement law with the same allocator-refusal intent and visits bounded between node count and3xnode count. Those actual owner proofs cannot be transferred to JSON20 by analogy.

JSON can descend directly from the root's Array or Object vector without allocating a root frontier; scalar roots simply release their leaf fields. Each popped array JsonValue or object JsonMember creates an inactive spare slot in its original vector. A continuation retains that exact sibling vector and the previous link until the active child is fully drained. Popped object keys drop before child descent; leaf number/string payloads drop through their real variants.

A naive frame containing a discriminated array/member Vec plus pointer may exceed the smaller JsonValue slot. Do not assume ABI fit or weaken a fit assertion. Candidate alternatives are a union of equal-sized Vec headers plus a separately proven tagged link identifying its header kind, or split overlays using existing spare slots. Any pointer-tag implementation must prove required alignment, retain pointer provenance, clear tags before reads, and assert frame size/alignment against both JsonValue and JsonMember on actual targets. This is a design candidate, not proven portable implementation. Active work may be an ordinary stack enum; it must not add allocated or persistent schema fields.

## Required Closed Neutral Lifecycle Corpus

Create an adjacent future fixture and schema declaring bounded depth8192,width4096,stack65536, Unicode/NUL schema/key/leaf literals and explicit linear visit multiplier. Include array-only deep chain, object-only deep chain with duplicate literal sibling keys, alternating array/object with retained siblings and empty child containers, wide arrays and objects, root leaves and empty containers. Reject extra properties, missing fields, excessive bounds, and weakened visit allowance. Source validation should independently validate schema and compute authored node/key/byte census; Source proof does not exercise native allocator behavior.

## Required Native Laws

An isolated child controls allocator tracking/refusal only after the entire fixture is built. On a narrow stack invoke actual public retire_sqlite_snapshot and verify zero new allocation attempts, zero remaining tracked owned bytes including schema/object keys/number lexemes, and strict linear visited nodes. Cover vectors with zero capacity and spare capacity separately. Avoid equality/debugging of deep fixtures while refusal is active.

A separate deep partial controlled binding/reconstruction cancellation law must reach an interior owned frontier after children have been materialized, then prove guarded cleanup and complete allocation release. Its allocator refusal flag should activate at the actual cleanup boundary rather than prevent the error diagnostic from being represented; otherwise diagnostic allocation can confound lifecycle authority. A final complete-snapshot cancellation law should retain the owner before the final callback, then retire without recursion/allocation. Preserve exact Canceled identity and callback evidence. None of these future laws is present in or inferred from current Native20 shallow ownership proof.
