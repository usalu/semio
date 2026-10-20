# Retained Runner Cancellation Results

The release16 manual cancellation exposed a harness defect: its Docker exec transport returned0 after the owned container stop, so the old runner wrote status0 while recording expiredtrue. That immutable historical receipt remains unchanged. It is not a course pass.

## Owned Change

Release changed only the existing `mode==="command"` block in `🚀️run/📜️script.ts`. Runtime independently owns the new resume dispatch/functions in the same file. The command's genuine `bun repo` invocation, quick test level,7200000ms supervisor, Docker stop30second grace and awaited cleanup remain unchanged. No private Source file, product code, builder state or actual managed command was changed by this repair.

The block keeps rawStatus separately from effective status. The first stop reason wins and admits exactly one owned Docker stop and one transport kill. Signal handlers remain installed while cleanup is pending, then are removed in finally. The runner waits for both the actual exec child and Docker cleanup before publishing its terminal receipt. Failed cleanup still refuses after recording its actual status.

| Stop reason | Effective status | cancelled | timedOut |
|---|---:|---|---|
| None | Raw transport status | false | false |
| SIGINT |130|true|false|
| SIGTERM |143|true|false|
| deadline |124|false|true|

New command receipts use explicit stopReason/cancelled/timedOut instead of the old conflated expired field. Normal success and failure statuses are unchanged. The terminal console and managed ledger record both raw and effective statuses. No compatibility interpretation or rewrite of older receipts was introduced.

## Test-First Evidence

The language-neutral input is `🚀️run/🧫️command.json`; the retained `🚀️run/🧪️command/📜️script.ts` extracts and transpiles the actual command block. Fake process, timer, Docker child and cleanup ports drive the real body without signals, Docker operations or builder access. Python SQLite independently computes effective status and reason flags from the fixture rows.

- `generated/2026-10-20-runner-command-red3.log` is the meaningful original regression: SIGINT with raw transport0 and settled cleanup produced effective0 instead of130.
- `generated/2026-10-20-runner-command-red4.log` is the meaningful cleanup-lifetime regression: while cleanup was held, the former early off() left a later signal without the retained handler.
- `generated/2026-10-20-runner-command-green4.log` and matching JSON are terminal0 for all12 cases. They cover normal0/failure7, SIGINT/SIGTERM/deadline with raw0, all three first-reason races, failed cleanup9, held cleanup, held exec exit after cleanup, and repeated signals while cleanup is pending. Exact genuine command/environment/grace arguments, two identical terminal receipts, logger fields, listener removal, timer removal and a Source read fence are asserted.

Initial red1 used an incorrect harness parent depth; red2's uniqueness check also matched an intentional string inside the separately owned resume test. They are retained setup failures, not semantic RED evidence. Green1 found a harness comparison that retained an undefined field while JSON serialization omitted it; the expected serialized object was corrected. Green2/3 were passing earlier cohorts; green4 is the final12-case proof.

## Frozen Handoff And Attribution

Version1 rows are in `generated/2026-10-20-runner-command-handoff.json`.

| Ticket-relative path | Status | SHA256 | Bytes |
|---|---|---|---:|
| `🚀️run/📜️script.ts` | Updated shared runner; only command block owned here |a748035e78cc7d21f8f9c0337f504e848773850f861830e3b1897f06755030d0|50513|
| `🚀️run/🧪️command/📜️script.ts` | Created neutral actual-block harness |bb0bb8f40cd6c17f86106de40c4137df4e3ae2f743fb52d19945eff1cead6b53|7671|
| `🚀️run/🧫️command.json` | Created language-neutral case contract |98b80096bf39de457049111f61778fb175fefe195ad627cd6fa0103ce8d39f99|2337|

The extracted command-body hash is da1f88d615a5c687b390ae275fb3f880ab75f7a7af5f71ebedff9d35c8a41165. Runtime is independently checking unchanged resume/non-command bytes against its settled proof. No actual cancellation or managed success is claimed by these controlled tests; root retains the next actual builder resume/instrumentation/probe lane.

## Independent Final Runner Fence

Runtime final read-only fence accepted the exact a748035e… runner. Reversing only the late-signal finally adjustment in memory recovers its preceding cbbf38a5… whole-file hash, demonstrating unchanged resume bytes from the21-scenario GREEN4 proof. Whole-file transpilation passes. Evidence: generated/2026-10-20-runtime-builder-resume-handoff.json and generated/2026-10-20-runtime-builder-resume-final-fence.log; runtime report2026-10-20-builder-resume.md. This is neutral controlled proof and Source identity validation, not an actual builder start.

Parent accepted the final a748… runner for future actual commands and owns authenticated resume/diagnostic execution. No further worker runner edits, actual signals or managed actions followed the controlled proof.

## Diagnostic17 Actual Cancellation And New Runner Confirmation

Diagnostic release17 was deliberately cancelled by explicit parent instruction after the unmarked quiet interval following call3 body-rejected/final-recheck-after. Exact local runnerPID72398/parent31454/argv was verified before supported SIGINT at2026-10-20T06:53:30.809511Z. Owned94319 settled TERMINAL130. Immutable generated/private-build-20261020T063128-ace24a38.json (14103B/SHA256a066ee11fe1343b70bf861a3215e342a60ae1e5159ab4031c9be81bae6e04e37) records06:31:28.802Z–06:53:32.128Z (1323.326seconds), effective status130/rawStatus0/stopReasonSIGINT/cancelledtrue/timedOutfalse/cleanupStatus0/containerStoppedtrue and unchanged7200000ms. The runner cancellation semantics are now confirmed on the actual supported path; this is neither timeout nor course pass. Actual builder inspection confirms exited/Runningfalse/Pid0/ExitCode143/OOMKilledfalse, finished06:53:31.999712176Z. Log generated/2026-10-20-runtime-course-ts-release17.log is5815404B/SHA256e503866be25064252b65927ce55d15d36deb804ad383023c0dd74285e5fc06d2. All24SDKrecords across3calls preserve the final-after boundary; call3 body rejection cause remains unexposed. Graph/portable complete markers are partial preparation evidence; no collection/assertions/final task result occurred. Private24 plus the sole SDK-reader-stage diagnostic remains retained; no Source/private mutation, retry or additional managed command follows. Parent owns the next failure-observation proposal, guarded restart/readiness and exact diagnostic restoration. Generated course17-cancellation.json retains signal/identity/actual state facts.
