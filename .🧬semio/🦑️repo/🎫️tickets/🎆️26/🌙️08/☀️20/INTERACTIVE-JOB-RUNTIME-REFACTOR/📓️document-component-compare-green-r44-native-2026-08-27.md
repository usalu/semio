# Document Component Comparison R44 Native GREEN

Canonical UI contract selector `retained_document_component_compare_` completed **3 passed, 137 skipped**, 0.111 seconds. The strict source schema/Node Buffer oracle completed 43 assertions. This is an isolated existing-document read/comparison primitive, not active old-record reconciliation or full resident accounting.

The new owner binds an exact immutable document lease, ordinal, expected node ID, and incoming component. It shares the private typed comparison engine, not `read_node_page` or its whole-record clone. Native admission requires **15,224 bytes**, including all 256 comparison frames, fixed owner initialization, and exact lease/component moves; allocation delta is zero. Insufficient admission and incorrect ordinals return the same incoming Surface allocation pointer.

All 18 variants run at 1/64/4096 comparison-byte grants. Seven cancellation frontiers run under all three close grants, and the other document lease remains readable after comparison close. Document-arena contention preserves the owner without waiting. Nonfinal read release and final-root retirement decision occur under the same arena lock; general document lease close semantics are unchanged.

Current runtime still retains its prior record map. Canonical document-root/id-ordinal replacement, separately admitted document placement, runtime old-record use, complete resident owner census, and Process acceptance remain pending. No duplicate current tree has been introduced into runtime.

Actual output:

```text

[DEBUG] fixed-list-page-oracle checks=43
warning: ignoring --test-threads because --no-capture is specified
────────────
 Nextest run ID b1ca40c9-519b-421f-8af5-837c9b91ee60 with nextest profile: fundamental
    Starting 3 tests across 1 binary (137 tests skipped)
       START [         ] (1/3) semio-framework-ui-contract document::document_component_compare_tests::retained_document_component_compare_cancel_and_contention_keep_live_document_and_incoming_root

running 1 test
[DEBUG] document-component-cancel frontiers=7 grants=1,64,4096 contention-retains=true live-document-wait=false
test document::document_component_compare_tests::retained_document_component_compare_cancel_and_contention_keep_live_document_and_incoming_root ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 139 filtered out; finished in 0.02s

        PASS [   0.050s] (1/3) semio-framework-ui-contract document::document_component_compare_tests::retained_document_component_compare_cancel_and_contention_keep_live_document_and_incoming_root
       START [         ] (2/3) semio-framework-ui-contract document::document_component_compare_tests::retained_document_component_compare_reads_exact_lease_without_copy_and_preserves_wire_order

running 1 test
[DEBUG] document-component-read variants=18 grants=1,64,4096 old-root-copy=false wire-order=41,9 exact-close=true
test document::document_component_compare_tests::retained_document_component_compare_reads_exact_lease_without_copy_and_preserves_wire_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 139 filtered out; finished in 0.02s

        PASS [   0.039s] (2/3) semio-framework-ui-contract document::document_component_compare_tests::retained_document_component_compare_reads_exact_lease_without_copy_and_preserves_wire_order
       START [         ] (3/3) semio-framework-ui-contract document::document_component_compare_tests::retained_document_component_compare_rejects_exact_owners_before_admission_or_foreign_ordinal

running 1 test
[DEBUG] document-component-admission required=15224 rejected-root-pointer-exact=true foreign-ordinal-denied=true
test document::document_component_compare_tests::retained_document_component_compare_rejects_exact_owners_before_admission_or_foreign_ordinal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 139 filtered out; finished in 0.00s

        PASS [   0.015s] (3/3) semio-framework-ui-contract document::document_component_compare_tests::retained_document_component_compare_rejects_exact_owners_before_admission_or_foreign_ordinal
────────────
     Summary [   0.111s] 3 tests run: 3 passed, 137 skipped
[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-OZTlv9



 NX   Successfully ran target test for project @semio-tech/ui-contract-rs
```
