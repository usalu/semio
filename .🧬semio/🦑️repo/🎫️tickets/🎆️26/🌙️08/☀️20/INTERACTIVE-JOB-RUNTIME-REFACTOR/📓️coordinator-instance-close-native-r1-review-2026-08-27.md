# Exact Instance Close Native R1 Review

## Latest Original-Two R6 Remains Red

The corrected fresh-process R6 selects exactly the original two tests. It compiled in 25.57 seconds, then reported generation1 outer faults at 8,519 and 19,965 microseconds and SIGABRTed; no passing footer exists. The coordinator read the retained output directly. In the first case, phase5 is87 and phase6 is8515, placing the large interval after an outcome/session close step and before the pump guard is released. In the second case, phases8–14 are zero: the job body was not entered, yet driver return phase3 is19951. These do not establish one common cause.

R5 selected three tests because its initial skip name was incorrect; it is not an original-two cold proof. Its actual result was1PASS/2FAIL. The 22,004-microsecond case spent about21,955 microseconds after app close and before typed job return. The 23,483-microsecond case spent about23,430 microseconds after typed job return and before driver return. Exact evidence: `🧪️member-instance-lifetime-close-cold-r5-native-2026-08-27.txt` and `🧪️member-instance-lifetime-close-cold-r6-native-2026-08-27.txt`.

Source-confirmed candidates include allocation/initialization for a four-byte checkpoint payload, blocking per-site watchdog telemetry, blocking trace-event publication, and outcome/session ownership restoration. Deterministic held-mutex telemetry laws and finer ownership handoff diagnostics are assigned. Optional telemetry must not block a callback; making it lossy cannot silently lose the exact per-job overrun verdict. No prewarming, threshold increase or test serialization is accepted as the fix.

The new deterministic terminal/optional-clock/exact-owner cohort separately passes4/4 with452filtered in0.02seconds. The coordinator read the actual four test names and result and the changed optional-clock source. Equality at8,000microseconds now faults through the shared strict predicate; missing/backward clocks reject or fault without optimistic COMPLETE. This is not a cold timing pass.

## Cold Original-Two R4 Is Red

The requested fresh-process rerun excluded all three new deterministic laws. Both original tests faulted again and the process subsequently SIGABRTed during strict Store Drop. The coordinator read `🧪️member-instance-lifetime-close-cold-r4-native-2026-08-27.txt` directly; there is no successful two-test footer.

Generation 1 outer watchdog measurements were **155,839 microseconds** and **26,938 microseconds**. The fixed phase arrays were `[0,0,0,155826,155837,0,155839,155839]` and `[0,0,0,26922,26937,0,26938,26938]`. The large interval is inside BatchJobSession.step, before outcome checkout. Neither reported an inner fault or structural stall. This confirms that the combined five-test pass is not sufficient cold-start timing proof. It does not yet prove whether app cleanup, tracing, allocation, lock contention or scheduling dominates that interval.

Source tracing reaches try_step_on_caller, drive_worker_job_authority and drive_step_with_payload_ledger. Potential subphases include the job's stage trace, maintenance close, actual app close, watchdog sample recording and checkpoint tracing. The trace implementation has lazy bounded-ring allocation and blocking shared registry locks; these are investigation candidates, not established causes. Finer bounded phase evidence is assigned. Prewarming or serializing the test is not accepted as the complete fix.

The exact plan requires callback/step duration strictly below 8 ms. Equality at 8,000 microseconds is currently admitted by the prior trace/close predicates and fixture; explicit 7,999/8,000/8,001 boundary corrections are assigned. The numeric threshold is not increased.

## R3 Ownership Correction Result

The coordinator read the complete three new native law bodies, the corrected inner/outer worker source and the actual runtime footers. Deterministic terminal-order/fault-precedence/exact-owner retention passes **3/3**, 452 filtered, 0.02 seconds after 30.11 seconds compilation. Combined R3 passes **5/5**, 450 filtered, 0.32 seconds after 27.28 seconds compilation, including both formerly failing original lifetime tests. Logs: `🧪️member-instance-lifetime-terminal-green-r1-native-2026-08-27.txt` and `🧪️member-instance-lifetime-close-r3-native-2026-08-27.txt`.

Both forget paths are removed. One cell guard spans terminal preflight, detachment, unwrap and exact shared-Arc restoration. Maintenance terminal ownership is included. The inner turn returns a candidate; the outer elapsed verdict alone publishes status, and fault/cancellation wins over Complete. Fixed test-only phase timestamps remain for diagnosis.

These passes do not establish a causal explanation for the earlier 38 ms callback. The new deterministic tests create BatchJobSession before their own timed boundary and may incidentally warm shared state. The coordinator therefore requested both original tests alone in a fresh process, excluding all three new laws; that run is pending. General all-app timing and full native UI/host/renderer/ACK retirement remain open.

## R2 Diagnostic Result

The coordinator read the actual R2 runtime footer and both DEBUG faults in `🧪️member-instance-lifetime-close-r2-native-2026-08-27.txt`. The coherent microsecond-consumer snapshot compiled in 3 minutes 26 seconds; the two lifetime tests ran and finished **0 passed, 2 failed**, 449 filtered, 0.35 seconds. Unlike R1, this run did not SIGABRT.

Both failures were origin 1, the outer callback wall watchdog: generation 2 measured **9,147 microseconds** and **38,260 microseconds**. Their inner fault flag was false, stalled count was zero, pending count was zero, and the fixed inner diagnostic buffer was empty. This establishes callback overrun, not an inner cleanup failure or a numeric payload-ledger collision. It does not identify the expensive callback phase. Bounded test-only phase timing is assigned; the hard 8 ms ceiling is unchanged.

The transport executor has authored two deterministic RED laws for premature COMPLETE and complete/fault precedence. Production repair must keep exact fault owners reachable, give fault/cancellation precedence over optimistic app completion, and publish the final status only after the outer verdict. Intrinsic native UI descendant retirement and the complete host/renderer/ACK join remain separate unfinished obligations. The earlier R1 abort evidence below is preserved.

## Actual Result

The native `instance_lifetime_close_` cohort compiled in 1 minute 21 seconds and started two tests, then aborted. It is RED, not a two-test pass. The coordinator inspected `🧪️member-instance-lifetime-close-r1-native-2026-08-27.txt` directly.

The primary error is `plugin.internal: runtime close cleanup faulted for instance 7` from the foreign-root/exhaustion test's close pump. Subsequent strict `ArtifactStore` Drop checks panic during unwind and terminate with SIGABRT. The secondary Drop checks must remain strict; masking or leaking their owners would not repair the primary close failure.

## Investigation Scope

Subsequent source review found two additional obligations, not proven causes of R1: `run_runtime_close_turn_inner` can forget a detached cell when restoration re-locking is contended, and can forget a nonterminal app after unwrap. The executor must retain the exact fault/retirement owner instead. The inner turn also publishes COMPLETE before the outer elapsed-time watchdog's verdict; the final status must be published only after both actual emptiness and the callback timing verdict, so an observer cannot accept a transient optimistic terminal state. These exact findings were assigned without relaxing the hard ceiling.

The close witness captures the exact allocation with a Weak reference, then retains the admitted close state and checked close generation. The two laws exercise witness survival across quarantine removal, reused numeric instance IDs, repeated old close, foreign-root rejection and generation exhaustion before ownership detachment. This is app/worker-session lifetime only; reactor, native UI, retained JS UI, ingress, publication and ACK owners still need their exact aggregate join.

Each resumable session owns its own `JobPayloadOperationLedger`. Reused numeric operation/generation IDs alone therefore do not prove a payload-admission collision. Global watchdog history still uses numeric operation/generation identity and requires investigation. The executor is adding bounded test-only diagnostics to distinguish actual callback overrun, watchdog evidence, zero-progress exhaustion, session admission/step failure and app cleanup failure. The coordinator has not established the primary cause yet.

No runtime ceiling is increased and no parallel-runtime test is serialized merely to conceal interference. The publication executor retains the sole fleet Rust lease; the transport executor owns the exact-close implementation and diagnostic repair. Independent actor wire laws already pass, and the executor's later 59-test actor run is separate JavaScript/Node-worker evidence, not a substitute for this native RED result or complete host retirement.
