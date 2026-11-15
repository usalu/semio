# Package V: starved farm workers and closure.provider-entry-stale (2026-11-14)

## Verdicts
- Outlier rule: the single highest ratio of a class is left out when it exceeds 4 times (Protocol.outlierBasisPoints 40000) the second highest; several high runs stay (heavy tail); a lone sample is limited by the trust blend. Applied in estimateHostRatioV1 and estimateHostPhaseV1. The incident store (25 times and 1.16 times) gives 3224 MB instead of about 22 380 MB.
- Workers inside the reservation: YES in the admission decision (fixtures), NOT measured live. A request names its holder grant; while the holder ramps, elastic workers are lent whole jobs from its unused part (cost to the host zero, holder charged less by what its children are charged). 16 GB fixtures: show 6553 MB, parent at 1100 MB admits 2 workers (3072 MB lent), parent keeps 2948 MB (below 3 GB); show 4096 MB admits 1. Third worker finds 533 MB and takes the ordinary path. Through the real ledger (16 GB host, 6800 MB available) 2 workers admitted, host books the holder once, EN and DE lines.
- provider-entry-stale cause: workers check a shared provider entry twice (admit, then expand) with no closure pass; a peer adding a file to a hot tool directory in between flips the second check (answer undefined). Store is intact (scan of 42 565 entries: 0 missing layers, 0 corrupt), so CLOSURE_LAYER_LOST is not the cause. Fix: worker project runs inside a closure pass (as the parent's discovery does) and a stale query is re-answered up to 3 times after refreshing providers. Cause inferred from code and the live log, not reproduced.
- Not verified: effect on real workers; any live farm run after the landing; parent footprint below 3 GB.

## Live evidence
- Full run on the live tree before the landing (2202 s): the farm inferred 249 of 4775 projects (9 workers over the run, 8 failed, all provider-entry-stale), the parent the rest. Parent footprint 12-13 GB (peak 19 GB) with the `projects` grant at 16-17.5 GB and exec requests waiting ("479 MB spare above the 4096 MB floor"). So the grant lending alone does not explain or fix the parent footprint; in that run most inference ran in the parent concurrently.
- After the landing: `repo show projects` took 129 s, generation 80 came from a peer ("nothing is left to prepare"), parent peak footprint 304 MB, grant 3474 MB. No inference happened, so no worker/parent split could be measured.

## Tests
decision/estimates/phases/lend/gate/followup/upgrade/grow in the process member and library farm/shrink/main, closure-answer-refresh, closure-store-limits all pass (see landings.md). Red as before: library agents (flaky), project-farm declined and farm-close (other cause than the plan fixtures; I added floorMegabytes to the plans, the tests stay red).

## Know
- Lending only applies while the holder ramps; later workers are charged to the host normally.
- Scripts/copy method: scratchpad V2 (setup.py, runall.sh); nothing left in the worktree.
- Further work: parent footprint (waves, assembly), measuring a stale refresh with workers after the next hot edit.
