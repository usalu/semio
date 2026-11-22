# Dependency ownership handoff

The m02 and m10 dependency synchronizations completed their build-phase ownership fences and then restarted ownership discovery after taking the exclusive bindings gate. The m02 trace moved from frontier operation 1,371,136 at 465 seconds to list operation 64 at 1.419 seconds. The m10 trace moved from frontier operation 1,373,568 at 1,129 seconds to list operation 64 at 1.071 seconds. Its second discovery reached 445,824 operations at 46 seconds.

The producer now keeps the first published ownership callback alive when it takes the bindings gate. It rechecks that custody before and immediately after acquisition, then derives publication's input projection, opens the current ready receipts, and captures current Source endpoints under the gate. The callback's terminal ownership recheck also completes under the gate. The ownership index is written after the build proof and again after publication, and the independent retirement phase still runs under the gate.

If the first gated membership recheck detects a retryable change, the next attempt derives fresh ownership under the same gate. It skips the completed build phase and resumes publication only after validating the current immutable generations. The existing retry policy refuses cancellation and unresolved Source transitions.

The neutral producer cases assert that a stable synchronization opens ownership once. They also inject a retryable change immediately after gate acquisition and assert that a fresh ownership callback opens under the same gate without additional installer runs. SQLite independently queries discovery, gate, ready-receipt, and publication order. The existing Python oracle independently resolves the resulting binding destinations and hashes their ready receipts.

Isolated native-repo runs passed all 13 producer modes, the full-fence source contract, the incremental wiring contract, the converged-proof and preservation seed checks, and seven workspace-ownership churn cases. The aggregate project target remained queued for capacity during this check. The producer fixtures use small workspaces and simulate gate-time drift. This audit has no fleet timing for the revised path.

All 13 producer modes passed in separate invocations, along with seven ownership churn tests. Final focused cases checked that the original ownership callback closed before the exclusive gate released, including after publication failed. The stable, drift, and publication-failure reruns passed six tests with 300 assertions. The aggregate native target remained capacity-gated.
