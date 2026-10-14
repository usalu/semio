# Current Canonical Original Turn Failure Audit

Read-only current source, 2026-10-13. Supersedes historical absent-type audit: external authors now defined the actual type; no builds/edits here.

Actual owner `🧰️framework/🔨️modules/🧵️job/⚠️failure/🦀️.rs`, publicly reexported Job root line62. OriginalTurnFailure owns primary ValueError plus optional receipt_refusal ValueError and derives RetireOwned. `From<ValueError>` moves primary with receipt_refusal None; no cloning, diagnostic prose conversion or heap allocation. Schema/corpus define primary and receipt refusal as separate complete owners; current schema root describes its law fixture/cases rather than just the runtime struct.

## Exact Debit Boundary

`debit_original(primary,cx)` calls only `cx.consume_retained(primary.retained_progress())`, captures error as receipt_refusal and retains original primary. It does not debit fuel. Actual tests assert zero heap, preserved original owned-message pointer, recipient equals original failed child effects and fuel_remaining unchanged. StepContext::consume_retained updates actual recipient before reporting overdraw; hence receipt_refusal is evidence of that already-recorded original debit.

Use debit_original exactly once at boundary of a lower failed child whose error receipt has not yet been charged to current StepContext. Do not debit a failure propagated from cx.consume_retained again, and do not re-debit wrapper at outer Actor/provider boundary. `From` is appropriate for failures already accounted or carrying no newly unrecorded child effects. Keep caller scheduling/fuel pricing independent: existing Plugin jobs close path consumes fuel1 before owner.step, then on Err calls wrapper debit; success consumes step.retained receipt. That path does not need wrapper fuel debit.

Current GuestEntry still has local debit_original_error charging fuel copied_items plus retained and separate close_fault/debit_fault ValueError slots. Adopting canonical wrapper must replace its accounting role, not call both helper and wrapper. If that caller's existing policy prices failure work by copied_items, debit fuel once explicitly at its original boundary and retain canonical wrapper's both causes. Do not silently switch between fuel1 and copied_items because wrappers change.

## Custody And Admission

Wrapper itself is inline and introduces no new dynamic backing: original primary owns its existing message, receipt refusal is current static literal overdraw cause. An Option<OriginalTurnFailure> destination must be preborn inside the already admitted Entry or retained frame; enlarge any actual whole-entry header/copy quote from sizeof current owner rather than stale fixed byte constants. ControlledRetirement<OriginalTurnFailure> must use the wrapper's actual declared retirement birth/demand; separate primary-only cursor would lose receipt-refusal custody. No new permission, external dependency, Any box or synthetic controller needed.

Current GuestEntry declaration has preborn inline ValueError slots and cursors, while canonical Actor job provider cancel/step now returns OriginalTurnFailure. High's migration must store whole wrapper in actual preborn slot and expose `.primary` / `.receipt_refusal` borrows without taking/dropping either cause. A second semantic producer must remain fenced while either primary or receipt refusal is retained/closing. Error ownership cannot be flattened to primary merely to compile.

No concrete unexplained heap allocation in canonical wrapper construction was found. New Entry size/copy/frame admission quotes require actual callsite verification during migration; this report does not infer those quotes from comments. Current code is concurrent, so wrapper adoption/source/runtime qualification await owning tests.
