# Coordinator Native Full Suite R61/R71 Review — 2026-08-27

Root read the native executor's reports and actual raw result/debug lines. These are the executor's runs, not additional root Cargo invocations.

Common Kernel message/framing/dialect R2 ran seven tests successfully in0.177s with253skipped: three borrowed SendMessage tests, two fixed-header/split-reader tests, and two real Invocation/Presence dialect tests. This is not live return-source admission, poll signature cutover or input ACK proof.

Full UI contract R71 passed158/158 with0skipped in0.469s. Original existing-component runtime R60 passed2 with107skipped in0.052s, preserving zero allocation on refusal and accounting32768bytes over42copy turns.

Full runtime R61 selected109, with no exclusions. It ran82:81passed,1failed;27were not run after fail-fast, duration0.424s. The actual failure is surface_ownership_inline_fields_do_not_allocate_a_second_owner: tree-item-icon reported1536additionalbytes and reserved-binding3072, where the unchanged fixture expects no second allocation for those inline fields. This is a production census defect, not permission to weaken the fixture or raise the Process limit. Remaining27tests are uncredited.

Reports and raw logs remain [Kernel message R2](📓️kernel-return-message-green-r2-native-2026-08-27.md), [full UI R71](🧪️member-ui-canonical-full-r71-native-2026-08-27.txt), [original runtime R60](🧪️member-runtime-existing-original-r60-native-2026-08-27.txt), and [full runtime R61](🧪️member-runtime-canonical-full-r61-native-2026-08-27.txt).

The native owner continues exact inline accounting, simultaneous old/candidate/output/retired ownership, and paired transaction output admission. Full runtime, fresh Process, native returned-source/ACK ownership, fresh guest execution and all-app timing are not complete. No cleanup or publication occurred.
