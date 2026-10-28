# result-docs-g (wave 1026g, executor docs-g): developer docs of the build system as it is now

## Doc files
- NEW `🧰️framework/🛍️products/🦑️repo/📓️build-guide.md` (209 lines, EN first, DE second; follows the `📓️<name>.md` pattern of `📚️library/📓️schema-catalog.md`): run commands, what a run does (door, graph claims + coverage stamp + yield, nested pin, capacity admission with learned estimates), priority classes and `SEMIO_LEASE_PRIORITY`, lease status and holder progress, `SEMIO_LEASE_RECLAIM`, warm-cache invalidation (7 scheduling domains vs output-producing code, sectional digests, derived facts, source memo), editing the build tool, disk budget (`cache status|gc|dedupe|verify`, `SEMIO_CACHE_BUDGET_BYTES`), Rust lanes, test routing tables, troubleshooting table, status.
- EDITED `🧰️framework/🛍️products/🦑️repo/README.md`: a one-line pointer to the guide in EN ("Many agents in one folder") and DE ("Viele Agenten in einem Ordner"); fixed the two stale Admission bullets (EN + DE): floor is 12.5 % clamped 1.5-4 GB (2,048 MB on 16 GB), not 20 % clamped 4-8 GB.

## Verification
Every command, flag and variable in the guide was checked against the catalogue (`🏗️build/⚙️operations/🔣️catalog.json`), policy/schema JSON or source by `rg`: SEMIO_LEASE_PRIORITY, SEMIO_LEASE_RECLAIM, SEMIO_CACHE_BUDGET_BYTES, SEMIO_BUILD_GENERATION_PIN, SEMIO_BUILD_TOOL, SEMIO_BUILD_GRAPH_POLICY, SEMIO_GRAPH_COMPUTERS, SEMIO_GRAPH_PHASES, SEMIO_REPO_SOURCE_MEMO, SEMIO_REPO_PROFILE, SEMIO_REPO_IMPLEMENTATION, SEMIO_STALE_REEXEC, SEMIO_HOST_CAPACITY, SEMIO_TEST_WORKERS, SEMIO_COMPILER_PARALLELISM, SEMIO_CARGO_LANE, SEMIO_UNIT_CACHE_SLOTS. Numbers (floor, estimates, budget weights, stuck thresholds, ramp, yield bounds, exit 75 re-executions) were read from the policy files, not from result prose. `--graph-policy` is catalogued for graph/show only, so the guide uses `SEMIO_BUILD_GRAPH_POLICY=current` for `run`.

## Marked in progress / not shown (as their results say)
- self-g sealed snapshot: `🏗️build/🏛️sealed` exists, default `live`, door does not call it (only the loader canary argument is in the tree): documented as in progress, behaviour documented is the live exit-75 restart.
- lane-g (shared-dir cargo callers): started, no result: in progress.
- integ-g: cycle table empty: stated.
- Targets not met/shown: hot Bazel no-op < 10 s, builder < 2 GB, graph delta < 2 min (constant tests/policy re-discovery).
- Known limit kept visible: review-1026g #3 (stuck ignores watchdog activity).
- Not run: `repo verify taxonomy` for the new `.md` name (graph-loading), Windows/Linux.
