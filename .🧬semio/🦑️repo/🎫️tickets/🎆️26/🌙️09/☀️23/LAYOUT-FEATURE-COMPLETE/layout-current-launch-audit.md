# Current Layout Launch Graph Audit

The ordinary Specific Layout graph target is **not established as resolvable by this audit**. The supported sealed-only project query did not reach graph inspection: host capacity admission reported swap above its ceiling, with a 2,529 MB requirement, and waited behind the host. Two bounded attempts were cancelled with exit 130, including the documented live-source query below. No inference, graph construction, compilation, Vite, or browser was started by those queries. This is a present capacity blocker, not a new proof that the graph is unsealed.

The prior completed live-source query documented in [Current Graph Gate](layout-current-graph-gate.md) returned a typed cold-generation refusal: no usable sealed whole-workspace generation existed on that host at that earlier checkpoint. It proved the applied current-project query avoided the former graph-facts writer queue, but cannot establish the graph's present state. Today's capacity refusal means the exact present generation or any missing closure cannot be determined; do not report the old cold refusal as a fresh observation.

The safe supported inspection, when capacity admits it, is:

```sh
SEMIO_REPO_SEAL=live repo show project @semio-tech/framework-os-dev --graph-policy=current --json
```

The `show --help` contract says current returns the newest sealed graph without waiting for graph rebuild or checking freshness. Require successful JSON naming the project and both inferred targets `dev-layout-wgpu-prototype` and `serve-layout-wgpu-prototype`; inspect their dependency metadata for activation and preparation. A sealed older graph can be stale, so even a target listing is not proof that the live source generation is current. A typed missing-generation or closure refusal identifies the relevant sealed-data blocker if returned. Bound and cancel the inspection if it waits at admission.

[Launch Entry Point](layout-launch-entry-point.md) identifies the ordinary integrated launch as `repo run @semio-tech/framework-os-dev:dev-layout-wgpu-prototype`; the generated `serve-layout-wgpu-prototype` target also depends on Layout activation, which depends on preparation. Preparation covers the registry session, browser support, fonts, WGPU renderer, boots, worker and components. These launch commands execute work and can contend with the current canonical 6179 listener and guest owner; they were not run. `repo run --help` documents `--dry-run` as no task execution, but it does not promise no graph construction, so it is outside this read-only no-build audit. `--exclude-dependencies` admits only the named owner and is not an integrated prerequisite proof; these particular targets are inferred, so manifest-only resolution is not established.

The direct ticket stage wrappers and the live listener remain the separate owned path; their existence does not prove an integrated graph run. `repo service host status` was also attempted as a status query and cancelled as soon as it entered build-source admission; it produced no graph evidence.
