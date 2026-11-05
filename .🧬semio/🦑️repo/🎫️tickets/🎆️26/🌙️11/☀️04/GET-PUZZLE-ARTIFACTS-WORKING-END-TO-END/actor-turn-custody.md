# Actual Actor Turn Receipt And Resume API

GuestRuntime Owned adapter forwards execute_turn to OwnedRuntime.execute_actor_turn (owned-runtime118). Freshcall encodes events under suppliedactualidentity; pendingcall rejectsnonemptynew events. ForFuelExhausted/DeadlineExceeded, resumeloop retainsstate.pending and execute_actor_turn pausesOriginalOperationReceiver into state.original_receipt. Sameinstance+sameboundidentity mustresume with[]; do not re-submit Event. Originalactorinput/checkedloan remainhelduntilactualreceipt, not replacedwithnewoperation. FinalTurnResult.fuel_used is cumulative acrosspendinginterpreterexecution.

Actorcontextadmit/close use actor_context_call111: onlyfreshpendingnone encodesoriginalinput+retainedpolicy, thenresumeoperation. CompletionreadsfixedOriginalActorFixedReceipt; admission/close publicreplycontainsactualinput/spent/work. Samependingoperationmustfinishbeforeanotheroperation; nobudgetconstructorcalledOriginalexists.

ActualkernelBudgetmustbeformedfromRetainedTurnInput::borrow_original(cx) total andOriginalActorTurnLoan::borrow_original(cx) boundedsemanticview; currentOriginalActorTurnLoanhasvalidate_for/remaining, notreceive. ForactorkernelreturnedRetainedTurnReceipt, validate_for(issuedinput), checkloan.remaining(receipt.spent,receipt.work_units_spent), postACTUALwork withcx.consume_fuel andACTUALretainedwithcx.consume_retained, keepingrawreceiptuntilpostingreconciles. ProductionShardinsteadusesOriginalActorInput.receive_guest/return_actor_turn→receive_original_event→schedulercomplete; no fabricatedremaining/spend required.

MaterialAPIlimit: GuestRuntime.execute_turn returnsResult<TurnResult,TurnFault>; FuelExhausted/DeadlineExceededfaultprovidesneitherexecutionreceipt noractualpartialspentmeasurement. OwnedRuntimependingfuelprivate; existinginternalobservedresumefunctiontakescallbackbutexecute_turndoesnotexposeit. JobcrossingOriginalGuestExecutionReceiptAPIisadifferentdomainanddoesnotmagicallyapplytoactorcalls. ThereforeanactualCPUconservation/resumetestmustuseanactualownedmeasurementAPIornarrowrealAPIextension, notguessfullgrantedfuelorignorefailedCPU. CurrentproductionShardexecute_turn_for genericfaultbranchretires/unregistersactorforFuel/Deadline; do notclaimitsalreadyimplementedresumepolicy.

HistoryPatch kernel/history369 containscursor/upserts/can_undo/can_redo/active_alternative_id/current_checkpoint_id (pluscurrentdeclaredfields), noalternative displayname. Assertactualchangedactive_alternative_id andmatchingHistoryEntry/upsertsemanticfields; do notinventpatch.name. Allsourceauditonly; no compiler/testrequested.


## Staged Interaction Module Bounded Review

ActualinteractionusesrealInstanceOpenCaptured/Ack,pagedCommandBatchDriver+actualCommandIngressPage/acksanddecodedReadHistory/ArtifactCommandresponses. Historyassertionusesactualactive_alternative_idchangeandHistoryEntry.action_id=createAlternative(upserts),notnonexistentname. Postcancelmustackmatchingseqandpublishmatchingresponse;before/afterhistorysamerealinstance,thenactualInstanceCloseRetired/Ackbeforecontextclose. Strictpoll/contextfaultsabort,nounreceiptedFuel/Deadline retryorCPUguess. Actualreceiptpostedwithininitiatingcx; semanticvalidate_forandfuel_used<=heldfuelchecksremain.

Events/effects/command/historysourcesusepaidretirement;UIPatchcloseobservesreleasedbytes,presenceisempty+capacityzeroasserted. SourceprofiletotalstockvsperturngrantremainsBirthfrontier,nofundingadequacy/runtimeclaim. Noconcretewirefield/typeerroridentifiedinboundedread; actualnativequalificationrequired.

Freshsame-readerHubguestIOroot/cfgtest/Hubmanifest30942bytes/5inputs/294active,physicalprobesidentical. Retainedguest-io-source-census.ts/generatedguest-io-extents.json,unchangedcaps. Unexecutedsource-reviewedonly.

LatesttighteningrequiresnovisibleUIpublication/receiptandactualpatchclosecomplete; newterminaldriverretirementobservesemptyVecDequespinereleases. Freshsame-reader31767/5/304,probesidentical. Auditfrontier: retire_driverdestructivelydropsbeforegrantprecheckthenpostsactualheapspend; insufficientgrantwouldbeobservedaftercustodyreleased. Rootnotifiedtoestablishpaidstatic/conservativeemptyspinepreadmissionoractualownerinterfacewithoutboundincrease; no runtimefailureclaimed.

Terminalspineprepayfrontiercorrected: factoryobservesactualsuccessfulCommandEnvelopeSet/try_push/BatchDriverallocationorigin, carriesrequestedbytesceiling; retire_driverchecksitem/depth/releaseceilingandretirementyieldBEFOREtake/drop,thenmeasuresrelease<=ceilingandpostsactualdelta. Pageackpreadmitsfixedphysical4096pagebeforeobserve; actualCommandBatchEntryscalarsdonotaddownedheaprelease. No privateconstantguess/boundraise. Freshsame-reader32456/5/308,probesidentical. Boundedsource-reviewgreen; actualbackend/totalstock/nativegatesremainpending.

ExplicitoperationSourcepreimplementationreview: finitefixtureauthorizationdistinctfromperturnpolicyisvaliddesign,currentmountedpreparation/maintenance/closequotesunchanged. Proposed64000items/copy48MiB/capacity+release70MiB/depth4096arithmeticconsistent,notmeasuredsufficiency/productiondefault. Original2000user-visiblefuelquanta/120s/10000turnsboundsretained. Actual1024node/2MiBrequest/result/4MiBnativeworking/16MiBVM/2MiBscaffoldceilingsmustbeclosedneutralcaseandasserted; no creditrenewalafterrefusal. Reviewdesignonly,implementation/nativeproofpending.


## Cancellation terminal oracle audit

The current guest-IO Actor.drain(job, false) is a false-positive acceptance risk: it rejects Done but discards every actual output, and returns upon source_closed without proving a cancellation terminal result. An arbitrary semantic Failed outcome can therefore satisfy the cancellation check. Running is permitted while paid cancellation cleanup progresses, but is not cancellation completion.

The actual reactor jobs owner first signals cancellation, closes the original body/outcome/source, and then creates ValueError::literal(ValueRefusalKind::Canceled, original-job cancellation message), wrapped in RetainedExtensionFaultReply::new_terminal_cancellation. The reply is a pack fault, not JSON: its projection gives origin=framework, code=ValueRefusalKind::Canceled.as_str(), severity=error and retryable=false. ValueRefusalKind::as_str defines the exact code as canceled. AdmissionFailedJob emits those bytes with JobStep::Failed. Ordinary unrelated causes use the same Failed state with different fault codes.

The necessary assertion is an observed Failed outcome whose actual fully-read/acknowledged pack fault decodes successfully and whose code equals ValueRefusalKind::Canceled.as_str(); optionally also assert framework origin and error severity. Do not match prose or invent a separate protocol state. Keep rejecting Done, allow transient Running, retain a local observed-cancellation flag, and require that flag before accepting source_closed. Canonical existing decode usage is store::pack_rt::decode_wire_value followed by semio_framework_value::FromValue into semio_framework_diagnostic::Fault (plugin host root decode_fault_pack demonstrates it; that private convenience function itself is not a public helper). Its fallback behavior should not be copied: the test must fail a malformed fault. Decoder ownership/retirement must use the actual retained/controlled variant if the chosen typed Fault owner requires it.

Sources: plugin reactor/jobs around cancellation_fault_started and Refused dispatch; plugin extension/invoke/failure RetainedExtensionFaultReply and OriginalCauseLoan::Value projection; value/control/contract ValueRefusalKind::as_str; plugin/host encode_fault_pack/decode_fault_pack. Source review only, no runtime verdict.


## Staged cancellation gate follow-through

The current drain now retains an observed-cancellation flag, rejects Done, requires actual Failed payload to decode as strict pack and typed Fault, checks canceled/framework/error/nonretryable, and only accepts source_closed after that flag. Malformed or unrelated semantic Failed output now fails rather than satisfying cancellation. Actual output paging and acknowledgment occur before fault classification; transient Running remains permitted. The closed neutral fixture/schema now author failed/pack/canceled/framework/error/false. All drain call sites pass that same law.

Fault has a real explicit artifact_retire_struct owner covering all fields in diagnostic/retirement. The helper closes it through ControlledRetirement, selected_demand, close_original_retained_step and actual last_turn_work under actor original retirement turns. No ordinary live-Fault drop is substituted. The overall law still ends with assert!(interior), so a completed-before-cancel fallback cannot pass positive execute cancellation acceptance. Native/producer execution remains pending.

Fresh actual guest-io Source capture after this staged change: 36242 bytes / 5 inputs / 321 active lines, physical probes exactly equal. No bounds changed.


## Returned page ownership qualification

No demonstrated unpaid guest-page retirement gap was found in Actor.output. OwnedRuntime.read_actor_job_output documents and implements a borrowed guest-memory page copied into a host Vec and handed back in the same crossing. It reserves handback, reads the real guest output receipt, BorrowJobOutput copies the borrowed region into invocation.output, then AcknowledgeJobOutputChunk runs with the remaining original budget and combines actual read+ack receipts before returning the host Vec. In the reactor, the Owned path keeps the paid page in the producer; chunk acknowledgement ends only its exact loan, and the next paid output step retires that same producer body. The final output acknowledgement closes the guest source separately.

Thus the returned chunk.bytes consumed by Actor.output is a host-owned caller result distinct from the producer retained page; copying it into the test oracle Vec and dropping that host result does not itself demonstrate missing producer release. Caller-side oracle buffers are test analysis, not proof of guest paid closure. The existing read/output/ack/source_closed and final jobs/context/VM closure assertions must remain. Do not add a second charged guest release for the host copy by analogy. This is actual API/custody source trace, not measured runtime proof.


## Cumulative VM execution authorization finding

Current actor Source is finite for retained resources and semantic work, but does not enforce the declared 2000-quanta lifetime against all VM instructions. Actor.call constructs execution.fuel from cx.fuel_remaining each turn, validates the backend receipt and records spent_fuel in observations, then debits only the separate semantic work_units_spent through OriginalJobTurnLoan.receive. No separate remaining VM stock is reduced. Interaction.poll likewise bounds result.fuel_used by its current held turn without cumulative debit. A 10000-turn ceiling is finite, but can authorize five times the stated 2000 full execution quanta. The 120s monotonic deadline is an additional wall bound, not an instruction receipt.

Startup advance has an available actual OwnedActorStartupCall.execution receipt which the test currently ignores. Core advance_scalar separately charges physical instruction/copy work into StepContext, so some startup instructions already reduce semantic funds; this does not replace accounting its explicit VM domain. Kernel execute_actor_turn overwrites result.fuel_used with actual invocation.fuel_used, so that result is available for exact separate debit. Job APIs already return OriginalGuestExecutionReceipt and can be reconciled exactly without changing semantic work units.

A concrete API limitation remains for context admit/close: OwnedRuntime.actor_context_call resumes the interpreter, obtains invocation.fuel_used, then returns only OriginalActorFixedReceipt; GuestRuntime admit_actor_context/close_actor_context expose only input/spent/work_units_spent and readiness/closure. The Wasmtime context implementation likewise arms per-call fuel but returns only the semantic reply. No existing exposed actual context VM receipt was found. Ordinary instance paid close operates physical resources through StepContext rather than a guest export.

Correct finite design: preserve separate semantic and VM domains, authorize each actual VM crossing from min(per-turn ceiling, one remaining lifetime execution stock), validate its exact identity/budget receipt and checked-subtract actual measured instructions. Include startup and kernel poll, not only job calls; reject exhaustion before another crossing and retain receipt on error paths. Existing production GuestRelayOriginalRoot is the responsibility model: execution_fuel_remaining, checkout_execution_budget/checkout_actor_execution_budget, receive_execution_receipt and finish_key debit operation_execution_fuel separately from operation_fuel. Do not consume VM instructions as semantic work. For context calls without a public measurement, either expose the real existing invocation measurement through its owning interface or conservatively reserve/nonrefund the admitted execution ceiling in a separately named authorization ledger; that is an upper-bound reservation, not fabricated measured spending. It must be explicit and included before the call, not guessed after it. No larger authored stock or longer limits are justified by this finding.

This is a source-demonstrated test-budget gap; no native execution, patch or policy change was performed.


## Operationwide protocol-turn ceiling qualification

Current driver initializes Self.turns from epoch, not zero: epoch starts1 for the first actual birth caller.turn, and each advance/take crossing increments once. Thus startup counts are carried into maintenance today. However, neither startup advance loop nor handoff loop checks the cumulative count before dispatch; each independently allows maximumTurns iterations. The final close loop increments epoch but neither increments turns nor checks the operationwide ceiling before its direct caller.turn. Maintenance alone prechecks its turns counter.

The precise remaining repair is one actual protocol-crossing counter initialized to the first birth call, checked/incremented before every actual startup advance, take, maintenance and direct instance-close caller.turn, with no reset or larger cap. Bind the monotonic epoch independently or preserve its one-to-one current relationship; do not charge original issuer creation or host test-oracle analysis as protocol calls. Count the actual calls, not loop iterations that exit before dispatch. Existing10000 remains operationwide, including paid instance close. Source-only finding; no source edit.


## Context proposal independent ownership qualification

The genuine phased startup hands off OwnedGuestInstance with actor=None and original_core=Some. Legacy owned_operation_resumes explicitly refuses original_core, so the current actor_context_call path using begin_owned_operation is a source-demonstrated mismatch for the new real actor harness. Reusing the actual funded original_call crossing must preserve that original core/source rather than construct a second actor.

Repository-wide production inventory qualifies the proposal: actor-specific checkout/receive are not globally test-only. Actual OS run/wasmtime-node-host.rs245/251 checks out execution when phased startup requires it and receives each call.execution before finish_actor_step. That is an existing genuine production responsibility model. The missing connection remains specifically current context admission/shard outcome transport; the old context public reply omits actual invocation.fuel_used. No runtime failure is claimed from this static trace.


## Current Stronger Actor Evidence and Portable Status

The current genuine phased OriginalCore instance has a known source contradiction, not merely an unexecuted happy path: phased startup hands off original_core=Some/actor=None, while current legacy context, poll and job crossings go through operation begin/resume paths that reject original_core. The existing original scalar helper is not a general interpreter/host-call engine and cannot be cited as proof those public actor paths are implemented. No actual actor run has qualified a repaired general path. The native borrowed JSON owned-export proposal complements actor acceptance; it does not replace actual model publication, public guest IO, strict cancellation, same-actor post-cancel dispatch or UI/browser gates.

Cumulative VM execution stock and the operationwide protocol-turn ceiling remain open source-demonstrated gaps. Actual job receipt instructions are recorded without a separate lifetime debit; context APIs discard backend VM measurement; startup advance returns an explicit receipt which this test currently ignores. Current test startup count is carried into Self from epoch, but startup loops lack a shared predispatch bound and direct instance close does not debit/check the global counter. Preserve the authored2000 execution quanta/120s/10000 total turns; no increased limit or semantic/VM unit conflation.

Production qualification: actual OS Run wasmtime-node-host.rs245/251 already checks out and receives phased startup execution receipts before finish_actor_step. Do not characterize the actor-specific execution ports as globally test-only. The missing stock transport is specifically current context/shard/public crossing settlement and the acceptance ledger.

Parent-authoritative portable result: cold-launch Vitest actual31/31 passed, terminal log independently read by Root. This proves its current portable controls, not real fresh actor/browser startup. Older browser-readiness request19193 refused before tests at an operand guard; it is not a test pass. No fresh managed WASM publication, native physical IO, actor end-to-end, browser20 or full en/de app journey completion is claimed.
