# Original Reserved Source Laws Review

2026-10-28. Independent source-only audit. Read contract, evidence implementation, neutral mutations/tests, framework/Puzzle guards, compiler source interface and source-law packet. No code changes, tests or builds.

## Actionable Finding

P2: sourceBinding can falsely accept a missing mounted-owner call. It compares dispatch.indexOf(mount_original_reserved_owners) < dispatch.indexOf(letvalidation=) without first checking that the mount token exists. When mount is absent, -1 compares less than the actual positive validation index. Other sourceBinding terms do not require the mount call. Require nonnegative mount and validation indices before the ordering comparison, and add a deliberate mount-call omission neutral case. This finding was sent to root and mutations writer before report completion. Existing sourceBinding omission changes OriginalToolSourceBinding::borrow and therefore does not cover the missing-mount branch.

## Actual Source Binding and Fourteen Facts

The positive input is actual source read through toolJobPluginInstanceSource, which calls the compiler-owned readSourceFamilySource interface over instance, maintenance-ladder, cancellation, retained-command and job families. The public compiler interface reexports its canonical module-tree owner; readSourceFamilySource reads the listed family files. toolJobProductionSource strips test module spans before evidence extraction. It does not use the former synthetic host fixture for production acceptance.

All fourteen facts consume those source bytes: sourceBinding/cancellation/decoded bound inspect reserved dispatch; sourceValidation inspects validate_original_producer_source; callerParent inspects admit_original_tool_lease; candidateCustody/producerContext inspect drive_original_producer; workerContext/budget/pending/ACK/terminal/cancellation inspect drive_worker_step; publicationFreshness inspects typed_operation_document_is_fresh. outputLimit combines the scoped decoded bound with a whole-family raw output bound search. noReconstructedContext searches the dispatch/lease/producer/worker bodies. No fact is a hardcoded true value. Body extraction requires a unique named fn occurrence, then reads the Rust block through the existing inspector.

These are textual source recognizers, not a Rust control-flow proof. Most facts check conjunctions of exact tokens; candidateCustody and workerContext also check ordering. The missing mount index guard above is a concrete repair within that existing approach. Global output-bound presence is a broader witness than exact owning-function analysis; current neutral mutation removes every matching bound and does establish the intended omission refusal, without proving arbitrary relocation cannot fool it.

## Neutral Cases Are Not Vacuous

The schema requires all fourteen named booleans, each const=true, and forbids additional properties. The actual fixture has one positive source case and one named false case for each of the fourteen gates. The replacement helper asserts the target exists (using its compact spelling only if needed), then actually replaces all matches. Each candidate is re-extracted from modified source; actual gate must be false, schema acceptance false, exact host acceptance false and full framework acceptance false. The positive case requires actual current source acceptance before omissions can pass. A nonexistent mutation target therefore fails the suite rather than silently preserving input.

fast-json-patch independently changes the corresponding reference evidence key to false without mutating the baseline evidence; strict Ajv independently rejects the resulting contract and agrees with the re-extracted actual evidence. This validates evidence contract/omission decisions, not independent Rust execution. Framework import/config identity mutations additionally require full guard refusal. Their replacement calls lack the shared explicit target assertion, but an unchanged candidate would remain positively accepted and fail the expected false assertion, so they still cannot pass vacuously.

## Historical Positive Names and Negative Witnesses

Checked current framework factory and route guard files: run_framework_reserved_job, ensure_reserved_emit_bounded, validate_framework_reserved_commit, FrameworkReservedCommitPermit and framework_reserved_work_items occur zero times. Current positives require original reserved dispatch, original source/parent worker custody, import permit identity and ConfigurationWorker. Registry roster uniqueness, job/factory uniqueness, route lanes and app registration/build seam checks remain.

Both Puzzle host positives now read actual current production source. Synthetic app/job fixtures remain test inputs for the app-owned route/outbox laws. Deliberate negative witnesses remain for app-owned reserved factories, fixed ingress, raw ownership, cancellation, commit preparation/publication/closure, actual host ACK, terminal-empty witness, output bound, canonical freshness and parent loan. The test runs both Puzzle suites and asserts more than30cases, so replacing positive host text did not remove that rejection roster.

## Result Boundary

One missing-token ordering guard needs correction. No other actionable defect identified in the inspected fourteen current cases. Native runtime behavior is outside these static laws, and the actual owned source-law suite result remains the main task responsibility. No success is inferred merely from the earlier source inspection report.

## Writer Correction Verified

During this review the mutations writer corrected sourceBinding to require both mount and validation indices >=0 before ordering. Fresh source read confirms both guards. The neutral fixture now has16cases, including refuse-missing-owner-mount replacing mount_original_reserved_owners( with removed_original_reserved_owners( and requiring sourceBinding=false. The shared target-presence and actual extraction assertions cover this case. The identified false-pass defect is resolved in inspected source; no remaining correction requested. The declared test is queued, and no test result was independently produced by this review.
