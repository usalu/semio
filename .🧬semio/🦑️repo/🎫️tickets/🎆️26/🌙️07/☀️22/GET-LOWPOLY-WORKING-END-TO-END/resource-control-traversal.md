# Resource Control Traversal

## Source Finding

Process owner `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:260–261` remaining() walks parentControl chain and recursively calls remaining(row.follows). It has no per-call visited set/memo. Shared ancestor restrictions are reevaluated for every incoming path.

Public withRequestedIssuedResourceControlV1 at294 allows a following control equal to its actual parent. This forms a legal DAG, not a cycle. For repeated parent/following diamonds of depthn, unique scopes=n+1, while evaluated bounded rows satisfy T(0)=1,T(n)=2T(n−1)+1=2^(n+1)−1.

requireIssuedResourceControlV1 invokes full remaining before returning. Refinement at270 authenticates both controls through that full traversal before its own Set-based linear ancestry check. Publishing/yielding each check remaining before/after; child environment projection checks full/maximum-only/final authority separately. Thus the cost can be amplified even though the visible workspace selection work is unchanged.

Actual Cargo compiler-store reuse `repo library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:65` calls withRequestedIssuedResourceControlV1(retained.control,0,...,control.signal,control): retained parent and current following control share ancestry. This proves this DAG shape is reachable in production structure; it does not measure the exact active depth/path count of Native95837/40274.

## Actual Public TDD Red

Root authorized an isolated public API probe. Inputs control-dag.json, strict control-dag.schema.json and control-dag.test.ts are retained in this ticket. The test creates a genuine public ScriptProcessEnvelope with an unbounded parent and ordinary60,000ms narrowed controls; no parent environment is fabricated.

At deepest callback only, Date.now is temporarily wrapped to count reads and return the original actual clock value, then restored in finally. No time is advanced/frozen and no deadline budget authority is changed. Independent Graphlib multigraph preorder counts unique reachable nodes from neutral depths0,2,4,8. Expected public bound is≤4clockreads per unique node, allowing constant implementation overhead without asserting private layout.

Registered command:
`repo tools exec --light bun -- test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️22/GET-LOWPOLY-WORKING-END-TO-END/control-dag.test.ts'`

Actual initial red exit1,0pass1fail16assertions; clockreads1,7,31,511 against unique1,3,5,9 and allowed4,12,20,36. Failure31>20. This directly demonstrates exponential public restriction-check work on the authored legal DAG, not a source-text mirror or instrumentation export.

Added semantic guards and re-executed unchanged source:3pass1fail26assertions. Earliest following120ms deadline survives min-selection, then genuinely expires after170ms active wait; following-only cancellation is rechecked after a prior healthy call; duplicate shared ancestry still honors parent cancellation. Count regression remains red with the same1/7/31/511 measurements.

Verify owns production remaining traversal correction and permanent test integration; this agent edits no process production source. Root owns old admission/actual-child reruns.

## Required Semantics / Limits

Use memo/visited scope per one traversal, preserving every distinct parent/following restriction, current cancellation, live issuance, progress deadlines and maximumOnly meaning. Do not globally memoize across calls: time, cancellation, liveness and renewal are dynamic. Retain earliest deadline and admission-credit handling; do not widen timeouts, suppress follows, fake environment or cache results permanently. Shared-node checking once must not skip a different branch's restriction.

The public count reproduction establishes a real algorithmic issue. It does **not** prove this caused Native's70minute selection label, quantify its contribution, or establish deadlock. Existing telemetry already showed the selection label can persist during held-body rechecks. Live CPU and source-level DAG reachability are insufficient causal profiling evidence. No Native retry/kill/server/setup was performed here.


## Final Independent Green

Permanent dag.ts+freshness.ts independently ran exit0,5pass36assertions; measured3/5/7/11clockreads for1/3/5/9unique nodes. Final absolute-cutoff/liveness/cancellation freshness correction reviewed. See control-dag-review.md for exact source obligations and limits.
