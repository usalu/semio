# Cold TYPEOF SELECT Independent Audit

Read-only source review of validation/evaluator.rs TYPEOF branch (1204 onward), constructor-cases.json, execution-tests.rs constructor and foreign-context tests, and native-execution.test.ts. No new confirmed defect in this narrow checkpoint. No production edits or duplicate suite run were performed by this auditor.

## Closure and identity

The updated cold branch uses its local result Vec as a growing queue. It formats runtime seeds, suppresses mixed bare/IFC4X3_ADD2-qualified duplicates, then visits every queued seed against immutable metadata::TYPES SELECT members. New SELECTs are deduplicated before append and themselves visited later. Finite static declarations plus unique append ensure cycle termination. Seed and declaration comparisons strip only the exact current schema prefix; foreign qualified names cannot accidentally match bare catalog members. No primitive-to-defined-alias inference, new cache or container was added.

All arguments still pass check_value_context before the TYPEOF branch; existing execution-tests.rs foreign-source test explicitly calls TYPEOF on a foreign entity-bearing value. The branch reads shared type seeds and produces owned string values without modifying the caller's source/types. Cold result Vec index at is an ordinal, so its traversal survives Vec reallocation. scalar_text borrows the created string payloads; no arena entity dereference is needed during closure.

## Cold budget boundary

Each seed, declaration, member and dedup comparison calls tick. Accepted strings go through make/own_value, which checks and charges owned nodes including text bytes; push_value checks the current aggregate frontier and context; final aggregate construction charges its member count. tick checks original cold cancellation before work and after progress callbacks, and enforces maximum_work. Refusal returns before publishing the local aggregate. Scanning primitive seeds also consumes bounded metadata work, which can increase cold work counts relative to the former ancestry-only branch.

These are the existing cold evaluator controls. They are not retained original-wallet five-axis loans, capacity receipts, resumable transitions or controlled retirement. This audit does not extend the retained-machine custody claim to the cold evaluator.

## Constructor fixtures and evidence limits

The shared expected point set now has eight schema names, including IfcGeometricSetSelect, IfcLayeredItem, IfcPointOrVertexPoint and IfcTrimmingSelect. The positive-length set has ten, including its transitive measure/value SELECTs. Both preserve their original ancestry/alias seeds. The TS test compares a sorted array of schema-qualified values; the cold Rust test compares BTreeSets. The latter verifies membership but would hide duplicated values, so duplicate suppression was also inspected directly in the branch. Both intentionally filter primitive names; neither asserts the full primitive TYPEOF contract.

Parent reports the focused native-execution plus TYPEOF reference selection passed 23 tests / 173 assertions. This is reference TypeScript evidence. Parent is extending the separate NumPy/IfcOpenShell graph oracle to assert the two shared constructor sets directly; its complete 1,312-seed closure evidence is documented by typeof-select-oracle.py and the existing typeof-select-independent-audit.md. This auditor did not rerun that full oracle or treat external helper TYPEOF as a reliable oracle.

Rustfmt parsing does not type-check Rust borrow/API use. The branch's short-lived borrowed string comparisons end before result mutation; no obvious borrow/API mismatch was observed in source, but authentic compiler ingress still refuses before rustc and cold Rust tests remain unexecuted. Native retained admission remains refused; this report approves neither full EXPRESS semantics nor native runtime custody.
