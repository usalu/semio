# Control DAG Final Review

Independent read-only source review and actual public focused execution, 2026-11-05. No process cancellation, Native launch, server, environment fabrication, timeout widening or production edits by this agent.

## Actual Permanent Test Run

Executed:

```powershell
repo tools exec --light bun -- test './🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/dag.ts' './🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/freshness.ts'
```

Exit0: **5pass,0fail,36assertions**,1.51s test body total. Real clock-read counts at depths0/2/4/8 are **3/5/7/11**, within neutral linear bounds4/12/20/36. Original measured baseline was1/7/31/511 and genuinely failed before the fix.

Freshness delay positions1–4 all observed authorized:false,parentExpired:true. Positions5–7 were not reached for a delay (four clock reads) and were healthy, not falsely treated as expiry cases. The test therefore measures the real late-branch/final-clock crossings; it does not claim every configured delay was exercised.

Permanent owner admission/🟦️.ts now imports projection.ts,dag.ts,freshness.ts. Strict neutral schema validation and independent Graphlib reachability remain in the owned DAG test. Ticket tests are thin invokers. Root owns old admission/projection and actual child-suite reruns; no duplicate execution of those suites here.

## Traversal / Freshness Review

Process owner remains routing/invocation/process/🟦️.ts, remaining().

- Every call allocates fresh roots/rows/active/stack; no global control-result cache exists. Iterative postorder processes distinct reachable control rows and followed roots once, preventing recursive shared-parent path explosion.
- Initial root authentication retains the privately issued invocation check and original invocation remaining read. Follows restrictions are not discarded or treated as authority merely because another parent edge exists.
- Local row checks retain liveness, current signal, credited absolute deadline, monotonic requested deadline, progress expiry, maximumOnly behavior, and parent minimum. An active-row check refuses cycles rather than looping.
- Final distinct invocation pass authenticates every reachable invocation root and rereads its actual remaining time. Parent rows share their root invocation by construction; following roots with different invocations are included separately. No new cross-invocation authority is minted.
- Wall time is captured unconditionally BEFORE invoking validation/remaining callbacks for each invocation; a delay inside those callbacks cannot turn a stale remaining duration into a renewed wall deadline.
- Final per-row pass rechecks liveness/signals and computes absolute epoch/monotonic cutoffs with current admission credit. It refreshes progress limits too. The final shared wall/monotonic sample and minima expire a short parent even if another branch consumed real time after that parent was memoized.
- Progress expiry is still checked for maximumOnly; renewable progress remaining is excluded from the numeric maximumOnly minimum as before. Requested nonrenewable maximum and original invocation bounds remain enforced.
- A final live/signal pass runs after the potentially delayed final clock read, including every distinct invocation signal. Cancellation after an earlier healthy traversal cannot use a memo from that earlier call.

The old numeric postorder memo alone could authorize after its short parent expired during another branch; root identified that and Verify recorded a separate actual freshness red before adding the final cutoff refresh. This review inspected the corrected version, not that intermediate version.

## Authority / Limits

No wire envelope, parent ownership, capabilities, admission-credit or policy schema changes are needed for this traversal correction. The inadmissible policy0+finite-wire construction is not used or blessed by these tests: parent envelope is genuinely unbounded, and finite restrictions are issued through public narrowed controls.

The focused cases cover duplicate ancestry, earliest followed deadline, following-only cancellation, parent cancellation after a previous healthy read, and elapsed-time freshness. Source review checks distinct invocation root handling and maximumOnly preservation; those two obligations are not claimed as newly executed exhaustive cases in this five-test corpus.

No actionable new source defect found in this focused review. Complexity is now linear in reachable rows/edges with per-call metadata, rather than exponential path count. This proves the authored public DAG behavior; it still does **not** prove Native95837/40274 slowdown causation or predict speedup of existing old-image processes. Those processes retain their loaded implementation until root deliberately replaces them through ordinary admitted work.
