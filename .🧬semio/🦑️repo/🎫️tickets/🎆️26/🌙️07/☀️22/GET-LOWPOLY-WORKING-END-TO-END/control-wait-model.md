# Control Wait Neutral Model

## Authored Permanent Corpus

Test-only owner admission/wait-model.ts with adjacent wait-model.json and strict wait-model.schema.json; existing admission/🟦️.ts imports it. No production source change.

Neutral graph has a child with both parent and followed branch sharing a root, plus an expired unreachable node. Graphlib preorder independently identifies the three reachable restrictions and excludes the unrelated deadline.

Each row records monotonic start, requested maximum, epoch deadline and its own cumulative wait counter at birth. Data calls cover:
- open overlapping waits100–400 and250–500, union counted once;
- later open interval starting700, still progressing at750/900;
- that interval ending950, then active time1000→1100;
- epoch ahead of monotonic and monotonic ahead of epoch.

Independent SQLite window/sweep calculation unions clipped intervals, rather than adding overlapping durations. A second SQLite expression computes earliest row epoch/monotonic remaining plus max(0,totalWait−row.waitedAtStart), then the reachable minimum. Actual first-party creditedDeadlineRemainingV1 arithmetic agrees with SQLite and fixture expectations across nine calls. Open wait750→900 preserves250ms; after the wait has ended,1000→1100 spends100ms. The per-row waitedAtStart subtraction prevents old queue time from being recredited to a newly born row.

## Actual Pure-Data Oracle Result

Registered light Bun focused wait-model.ts initially executed the pure-data case: exit0,1pass32assertions. Graphlib, strict Ajv and SQLite are independent test libraries behind the registered repo tool interface. No issued deadline authority or wall clock was fabricated to run this model. This is an arithmetic/oracle proof, not a physical admission queue or live DAG execution measurement.

## Public Instrumented Follow-On

A second permanent public case is authored and awaiting Verify's open-wait fix readiness before execution: genuine unbounded parent plus180ms narrowed control; manually bracketed70msfirst/70msoverlap/70mstail wait intervals. Open wait outlasts raw180ms deadline but must authorize within original180ms active budget. After both intervals end, a real35ms active interval must reduce remaining while cumulative wait counter remains fixed.

This uses public beginAdmissionWaitV1 instrumentation with real elapsed time, not an actual machine capacity queue. It must not be called physical admission evidence. No clock mock, process signal, timeout widening or environment spoofing is involved. Earlier Verify-owned actual open-wait freshness red remains separate until corrected/rerun.

Root and Verify received corpus/ownership details. Verify owns all remaining() production corrections. No Native, server or process termination occurred.
