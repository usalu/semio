# Surface wasm and gis3d dev targets

## Status
- `@semio-tech/framework-surface-rs:wasm`: not built. Both attempts failed before any target ran, in the repo planner (graph slice worker), not in the surface crate.
- `@semio-tech/framework-os-dev:dev-gis3d-react-prototype` and `:run-gis3d-native-prototype`: not diagnosed. `show_project` needs a sealed graph, which cannot be produced while the planner is broken.

## Root causes (repo tooling, in-flight edits by another agent)
1. Attempt 1 (59 min): `Export named 'admissionReserveBytesV1' not found` in `framework/modules/process/packages/typescript/artifacts/native-build/admission`. The export landed in the package re-export during the run.
2. Attempt 2 (29 min): `Private Nx engine import missing: ../../remote-execution/sidecar/ready/workers/ts` imported from `repo/modules/library/build/fleet-status/readers/ts`. The target `workers` module does not exist (mid-move of the sidecar/ready tree).
3. Also: graph slice reading is very slow (1500+ s) under memory pressure (host gate floor 2048 MB).

## Next
Re-run `repo run @semio-tech/framework-surface-rs:wasm` once the fleet-status reader import resolves; then show_project on framework-os-dev.

## Retry after coordinator message (01:46 onward)
- The `workers` import and admission export blockers were gone. Attempt 3 (12 s) failed on `Closed cache ledger required: area caps leave less free space than the soft reclaim threshold of bazel-workspaces` (cache ledger `policy.json` and `ts` were being edited at that moment, so treated as a recent half-landed edit and not touched).
- Attempt 4 (2h36m): waited about 2 h for a free graph slice worker (2 of 2 busy, other agents' loads), then the slice worker ended twice with exit 75: the replica did not settle in the declared time because code kept changing underneath. Planning still never reached the target.
- No gis, surface or os-dev file was at fault; nothing was edited. `show_project` for framework-os-dev and the two gis3d targets remain undiagnosed.

## Next
Retry only when the replica is quiet and the slice workers are free (`repo scheduler status`); the ledger partition `products` (bazel-workspaces reclaim, soft 3 GiB) must validate against the machine budget partitions first.
