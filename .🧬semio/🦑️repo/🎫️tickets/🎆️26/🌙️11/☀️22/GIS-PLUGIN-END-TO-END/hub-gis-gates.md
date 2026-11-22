# Hub GIS gates

Status: all five targets NOT RUN. The repo planner failed before any target started; no source files were edited.

Targets: os-hub:component-codec-check, os-hub:gis-map-proposal-process-check, os-hub:trusted-stdio-gis-bundle-process-check, os-hub:trusted-stdio-gis-bundle-browser-check, @semio-tech/s-dev-composition:typecheck.

## Blockers (repo tooling, edited concurrently by another agent)
1. fleet-status readers imported the missing `sidecar/ready/workers` module. Landed after waiting.
2. Cache ledger refused deterministically: "Closed cache ledger required: area caps leave less free space than the soft reclaim threshold of bazel-workspaces" (ledger/🟦️.ts readCacheLedgerV1; files under caching/storage/ledger and enforcement/budget were being edited). Disk has 32 GiB free.
3. Latest run (2h36m, mostly graph slice reading): slice worker exit 75, "Private Nx engine import missing: ./🧊️stamps/🟦️.ts" from caching/bootstrap/tools/sdk/source/derivative/index/🟦️.ts (not yet landed).

show_project os-hub also failed: no sealed whole-workspace graph on this machine.
Next: retry component-codec-check once the repo tooling edits settle.
