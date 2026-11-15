# Quiet Grants Reserve Their Footprint, Not 2048 MB

## Root cause
- Service-planned `repo run` clients (and bazel forecast runs) are admitted as quiet grants sized by `forecastHostQuietV1`: learned ratio x `estimates.quietReferenceMegabytes`.
- A quiet class that never recorded a run has ratio 100 %, so the size was the reference itself: 2048 MB (`🔣️policy.json` quietReferenceMegabytes). A run that only waits on a nested graph-plan/script door holds ~47 MB, yet kept 2048 MB for minutes; two such runs starved cargo ("spare -463 MB").
- The growth path is already live: an executor's `requestHostCapacityUpgradeV1` or a tree outgrowing the quiet reservation grows the grant in the fair queue (tests in `🧪️tests/🤫️quiet`). Nested invocations inherit the quiet grant and book nothing, so the parent/child double booking was only the oversized quiet block.

## Change
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`: `estimates.quietReferenceMegabytes` 2048 -> 512 (unlearned quiet block; learned sizes are 30 %..200 % of it, i.e. 154..1024 MB; 256 was too tight for ~500 MB hot no-ops, which would grow to full), `quietMaximumMegabytes` 4096 -> 1024 (a class that needs more than that is not forecast quiet).
- Test (product followup suite): unlearned quiet request books exactly the declared reference (<= 512 MB, one job, quiet), a learned 47 MB quiet run stays within it and the quiet maximum.

## Not changed
- Graph-plan 2048 MB: that is the slice worker prior `sliceUnlearnedMegabytes` (resident policy, real worker limit, host test requires <= 2048); lowering it risks killing workers, so left.
- Light cargo subcommands: cargo sizing by tool alone is covered by `cargo-admission-scaling.md`.

## Restart
Client-side admission (`repo run` process) reads the policy per invocation: no restart. The resident service host holds the policy it started with and the existing live grants/estimate store (old quiet samples are ratios, they re-clamp to the 30 % floor): a restart is needed for it to use the new values; I did not restart it.

## Second symptom: free memory above the floor idle while a request fits
Findings: the decision has no per-session budget; the session only orders waiters (least held per weight) and gives an idle session its first job. The "budget" in `repo capacity status` is `share - committed`, and a ramping grant commits its whole reservation for its class's ramp time (60 s up to 30 min). Quiet grants were ramping grants, so each 2048 MB quiet reservation of the coordinator's subagents was committed in full against the budget while it observed ~47 MB: observed free 4885 MB, budget room negative, request waits though it fits the real free memory.

Change (`host-capacity/📏️decision/🟦️.ts`, both `decideHostCapacityV1` and `hostBlockersV1`, and the three Python oracles upgrade, blockers, lend): a quiet grant never ramps for accounting (`ramping` is false for `quiet`); it commits and leaves unrealized only what its tree observes. Growth beyond the quiet reservation still goes through the existing upgrade path, which flips the row to non-quiet (then it ramps and commits normally). Regenerated fixtures with the oracles (upgrade, grow, blockers, lend, framework followup, library quiet).

Tests: framework quiet suite gained "Quiet grants that hold next to nothing never commit their reservation against the memory budget" (four 2048 MB quiet grants of one session, 47 MB observed, 9000 MB free: binding is `available`, a 2048 MB request is admitted); oracle-backed decision fixtures cover the semantic. Adjusted library tests: runs that observe 100 MB (not 1100 MB, which is above the quiet size and rightly counts as executed) are learned quiet; the g29 "only what is free" bound is spare plus what the run already holds.

Run: `SEMIO_HOST_CAPACITY=off repo tools exec bun -- test ./<file>` (the `repo tools exec` door itself queued 4+ minutes at 2048 MB behind the same symptom). Passing: framework quiet/upgrade/grow/blockers/lend/followup (45+36), library quiet, followup, planned-grant, cargo-scaling (24+3). A combined all-files run under the off switch shows failures in handover, restart, marks, windows-probe, label, arguments (needs SEMIO_TEST_PYTHON_BINARY) tests that depend on the gate being on or on the managed python binding, not on this change; not re-verified with the gate on.

Restart: the resident service host must be restarted to take the new decision code and policy.
