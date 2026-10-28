# Original Admission Grant Laws

`admission.json` now states the five grant axes, the real 63-byte partial loan against a 64-byte codec demand, owner text extent and native execution flag. `admission_tests.rs` independently exercises items, copy, capacity, release, depth and fuel over every actual admission frontier. It checks original event/path/text addresses, inline traversal state, wire state/chunk bytes, next demand, receipts, fuel and actual heap observations around the operation itself.

The admission traversal and canonical codec have zero capacity and release demand. Zero grants for those axes correctly permit inline progress; no artificial nonzero requirement was added. Items, copy, depth and fuel tests underfund real requirements and assert no state/custody/receipt/heap changes. A separate canceled-owner test reaches a genuine nonzero release demand, denies it by one byte, checks no heap or receipt change, then drains the same retained owner under funded close. Existing every-prefix cancellation tests remain.

The shared tests compile in both actual IFC editors. IFC4 now has the trace test dependency, JSON fixture test dependency and a cfg(test) HeapWitness allocator, matching IFC4x3's existing instrumentation. Without a real instrumented allocator, observing zero heap events would be vacuous.

Executed: Ajv2020/UTF-8 event oracle passes six event fixtures and the oversized-event refusal; rustfmt parses the added tests. A focused `cargo test --lib actual_admission_grant_axes` still fails at the enforcing compiler's `-vV` ingress before type checking. Native test, original caller execution, heap observations and physical cleanup assertions are authored and **not executed**. No native pass is inferred from the independent oracle.

The canceled original-owner test additionally denies its real nonzero retirement-factory capacity birth before moving the event, asserting original event/path/text pointer custody and zero heap/receipt changes. The later release test exercises the same retained owner's actual release demand.

