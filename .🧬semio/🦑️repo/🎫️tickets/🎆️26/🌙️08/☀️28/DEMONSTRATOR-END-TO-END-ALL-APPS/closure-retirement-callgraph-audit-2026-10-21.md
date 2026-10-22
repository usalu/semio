# Closure Retirement Call-Graph Review

2026-10-21T13:17:28.906Z — finite source-only review of closures/🟦️.ts and its owned Protocol constants. No runtime action or timing measurement.

## Full Capture Pass Counts

For one new nonempty optional group, successful terminal settlement and then an immediate satisfied/no-more-groups exit requires six full captureClosureProof calls: initial discovery; post-rename authentication inside invalidateGroup; next-loop committed-group deletion proof; next-loop terminal discovery; nested terminal-absence authentication; final-loop exit proof. Existing committed intent with surviving members requires four: deletion proof; terminal discovery; nested terminal authentication; final exit proof. If the caller stops at member deletion rather than complete intent settlement, these are three and one respectively. Additional groups, prepared/renamed-but-uncommitted recovery, or a final condition that remains unsatisfied add passes. Each capture authenticates current full inventory through bounded read children and ends in full recheck. maximumClosurePasses=2 is present in Protocol but is not referenced by this owner as a pass counter.

## Concrete Scaling Finding

Current retireCommittedGroupMembersV1 (line100) calls full recheck(proof) twice per leaf. That function (line56) walks every current ancestor, directory, proof row and recorded missing path; physicalRow invokes named/check for each row. Directory removal adds one full recheck per directory (line102), plus a final full recheck (line103). maximumRetirementBatch=64 gates image recheck/progress/yield windows, not the full leaf rechecks. Per-leaf parentAfter also enumerates and compares the remaining parent names. Thus deletion is not bounded to a single leaf plus one full proof check per batch in this source.

The same custody shares recoveryBudget across all captures and deletions. check increments that shared operations counter, limited to16,777,216; bytes also remain cumulative, capped at17,179,869,184. A conditional example: if136,000 proof rows remain at the start of deletion, two full checks per deleted leaf alone exceed16million row checks after roughly62 leaves, before accounting for captures/directories/other checks. Historical136k is not a measurement of current Source13310. A single group also has a65,536-member limit;136k total inventory must not be assumed to be one group. If a large unrelated inventory persists, the row-check cost persists even as one group shrinks. This is a source-level operation-budget concern, not a runtime-failure claim.

## Deadline Boundary

This owner creates no thirty-minute deadline and resets no deadline between captures, groups or yield windows. It reuses custody.control, checking it throughout metadata operations, worker capture and continuation. A caller deadline therefore covers the cumulative work performed with that control; whether queue time is included and the actual duration value belong to its external issuer and are not established by this requested owner-only trace. Source alone cannot establish completion within30minutes, an actual timeout, current file count or observed throughput. No bypass or guard change is proposed; prepare only upon actual runtime evidence remains the parent decision.
