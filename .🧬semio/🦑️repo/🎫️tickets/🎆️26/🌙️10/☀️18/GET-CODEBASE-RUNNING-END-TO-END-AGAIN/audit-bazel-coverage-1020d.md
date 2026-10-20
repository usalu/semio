# audit-bazel-coverage-1020d: is Bazel fully behind `bun repo`?

Auditor `aud-bzlcov-1020d`, 2026-10-20. Read-only: no builds, no bazel/cargo/bun repo runs, no git commands, no sub-agents. Searches were limited to the build library subtree and named files.

Abbreviations used below:
- `L` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`
- `B` = `L/🏗️build`
- `BZ` = `B/🔧️tools/🏗️bazel`
- `C` = the ticket folder `GET-CODEBASE-RUNNING-END-TO-END-AGAIN`
- `VERIFIED-READ` = I read the cited live line in this session. `INFERRED` = derived from code structure or from a result document, not re-run.

## 1. Verdict

1. Routing is fully live. `--tool=bazel` and `SEMIO_BUILD_TOOL` inheritance reach Bazel for run, run-many, affected, exec, watch, test, reset, cache status|clear, cache remote status, and tools setup bazel|all. Nested tasks inherit the tool through the process environment (VERIFIED-READ `B/📜️script.ts:101-103`, `B/🧭️invocation/🟦️.ts:133`). There is no fallback from the Bazel path to Nx. Nx is only reached when `selected.tool==="nx"` (`B/📜️script.ts:193`).
2. Tool-independent commands never reach either tool: graph, show, preflight, dependencies, cargo, the native dashboard and native recovery (`B/📜️script.ts:86-96`, `:110`, `:120-131`, `:170-190`). `graph`/`show` read the portable graph and ignore `--tool` (`B/🕸️graph/🟦️.ts:242`: "without installing or invoking Nx").
3. Bazel-native mechanisms that are live in the source: Bazel disk action cache with GC (`BZ/💾️storage/🟦️.ts:46`), a remote cache rendered as Bazel flags but off by default (`BZ/🌐️remote-cache/🟦️.ts:19`, `B/🔣️policy.json:18` endpoint null), persistent-worker strategies per mnemonic (`BZ/👷️workers/🟦️.ts:15`), `bazel test` with cached test results for test-family tasks (`BZ/📜️script.ts:392`, `:415`, `:551`; `BZ/🧭️commands/🟦️.ts:25-37`), `rdeps` for the real `affected` answer (`BZ/📜️script.ts:726-737`), and a per-selection generated package (`BZ/📜️script.ts:588-589`).
4. Not Bazel-native, or only partly: every non-hermetic action is a `repo_task` that re-enters bun, tagged `no-remote-exec` and `no-sandbox` (`BZ/📜️script.ts:515`, `:523`). Remote execution is only reachable for actions that carry a hermetic declaration, and the one measurement recorded says zero (`C/result-bzl-coverage.md:7`). No `cquery` string occurs in `BZ` (grep, VERIFIED-READ for that directory). Lazy closure comes from the planner, not from Bazel query.
5. Staged-only items that the brief lists are mostly applied (`C/result-integrate.md:5-17`, groups A-G). The remote-execution service `bzl-rbe` is not applied (`C/result-bzl-rbe.md:5`). Verification of the applied groups was by their own test runs, which I did not repeat.
6. The default is Nx, declared in one authored place (`B/🔣️policy.json:3`). My recommendation: keep Nx as the default for now. Flip only after the gates in section 4 pass. The flip itself is a one-line policy change plus one help string.

## 2. Command matrix (`--tool=bazel`)

| command | route with `--tool=bazel` | Bazel-native mechanism in use | status | evidence |
|---|---|---|---|---|
| `run <p:t>` | Bazel branch, `B/📜️script.ts:208-230` | Bazel `build` or `test` on the batch; `--keep_going` unless `--bail`; disk cache; workers; remote cache only if the endpoint env is set | live (source) | VERIFIED-READ `BZ/📜️script.ts:752-800`, `BZ/🧭️commands/🟦️.ts:25-37` |
| `run-many` | same branch as `run` | same | live | VERIFIED-READ `B/📜️script.ts:120-121`, `:208` |
| `affected` (real run) | `bazelAffectedProjectsV1` | `bazel query` with `rdeps(//:all, set(...))` over a generated package | live | VERIFIED-READ `BZ/📜️script.ts:726-737`, `BZ/🎯️affected/🟦️.ts:28` |
| `affected --dry-run` | portable neutral rule, not Bazel | none | live, but plan can diverge from the real run | VERIFIED-READ `B/📜️script.ts:208-210` (`dryRun?selection.affectedBuildProjectsV1`); divergence documented `C/result-bzl-lazy.md:11` (lockfile-only change) |
| `exec` | same branch; `exec --dry-run` prints the Bazel plan, not Nx's exec plan | Bazel batch | live | VERIFIED-READ `B/📜️script.ts:211-216`; live baseline before integrate, 12 pass incl. dry-run and exec parity with Nx, `C/result-bzl-lazy.md:132` |
| `watch` | `watchBuildInvocationV1` plus `runBazelTasksV1` | `bazel run //:label` per continuous task | live | VERIFIED-READ `B/📜️script.ts:221-226`, `BZ/📜️script.ts:774` |
| `test` (`workspace:test`, `repo-test-domain:test-*`) | rewritten to `run`, then the Bazel branch | `bazel test` only for test-family tasks with at most 10,000 inputs and no consuming task | live, bounded | VERIFIED-READ `B/🧭️invocation/🟦️.ts:80-87`, `BZ/🧭️commands/🟦️.ts:22`, `BZ/🧭️commands/🧬️schema/🔣️.json:12` (`testInputLimit: 10000`) |
| `test bazel` | Bazel self-test suites, not a build | none | live | VERIFIED-READ `B/📜️script.ts:58-66` |
| `graph`, `show` | ignores `--tool` | none (portable graph) | live | VERIFIED-READ `B/📜️script.ts:170-190` |
| `cache status` | Bazel operation: disk census plus build-event metrics | BEP metrics | live | VERIFIED-READ `B/📜️script.ts:164-168`; catalog note `B/⚙️operations/🔣️catalog.json:535` |
| `cache clear` | stops persistent servers, removes output bases, empties disk cache | Bazel server and disk cache | live | catalog note `B/⚙️operations/🔣️catalog.json:535`, VERIFIED-READ dispatch `B/📜️script.ts:164-168` |
| `cache remote status` | renders the contract as Bazel flags and probes `/ac/000...` | `--remote_cache`, `--remote_instance_name`, credential helper | live, off until endpoint set | VERIFIED-READ `BZ/🌐️remote-cache/🟦️.ts:29-31`, `B/🔣️policy.json:18` |
| `cache serve` | serves Bazel HTTP and Nx API together, tool-neutral | n/a | live | VERIFIED-READ `B/📜️script.ts:133-141`; catalog `B/⚙️operations/🔣️catalog.json:537-538` |
| `tools setup bazel` | prepares pinned Bazel, modules, repository cache, readiness report | `prepareBazel`, `prepareBazelModulesV1`, `describeBazelReadinessV1` | live | VERIFIED-READ `B/📜️script.ts:142-154` |
| `tools setup` (no arg) | provisions only `selected.tool`, i.e. Nx by default | n/a | live | VERIFIED-READ `B/📜️script.ts:144` |
| `tools setup all` | runs Bazel, then Nx `--version` | Nx side is broken, see gap 5 | partly broken | VERIFIED-READ `B/📜️script.ts:155`; `C/result-bzl-xplat.md:12` |
| `reset` | `resetBazelBuildStateV1` | Bazel state reset | live | VERIFIED-READ `B/📜️script.ts:158-161` |
| `remote-execution` | no CLI command exists | env only: `SEMIO_REMOTE_EXECUTION_ENDPOINT|PLATFORM|PROPERTIES|JOBS` | env live, service staged | VERIFIED-READ `BZ/🏃️execution/🧬️schema/🔣️.json:105-108`, `:335-349`; `BZ/🏃️execution/🟦️.ts:48-64`; catalog has 0 entries (grep); service `C/result-bzl-rbe.md:5` |
| `--workers on|off` | Bazel worker strategies (`--strategy=...=worker,sandboxed,local`) | `--worker_max_instances`, multiplex, cancellation, memory limit | live | VERIFIED-READ `BZ/👷️workers/🟦️.ts:15-37`, `BZ/📜️script.ts:752-800` (`bazelWorkerSettingsV1(workers)`) |
| `preflight`, `dependencies`, `cargo` | tool-independent | n/a; cargo runs directly, not via Bazel | live | VERIFIED-READ `B/📜️script.ts:91`, `:124`, `:128-131` |
| `dashboard` (`repo-dashboard-rs`) and native recovery | dispatched before the tool branch | n/a | live | VERIFIED-READ `B/📜️script.ts:93`, `:110-119` |

## 3. Tool selection and SEMIO_BUILD_TOOL inheritance

- Precedence: `--tool` / `--tool=`, then `SEMIO_BUILD_TOOL`, then `policy.defaultTool` (VERIFIED-READ `B/🧭️invocation/🟦️.ts:133`, `:138-139`). `--` ends parsing, so `--tool=nx` after `--` reaches the child untouched (VERIFIED-READ `B/🧭️invocation/🟦️.ts:137`; test `B/🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts:25`).
- Invalid values throw (`B/🧭️invocation/🟦️.ts:144`). An empty `SEMIO_BUILD_TOOL=""` is not treated as unset, so it throws instead of using the default (INFERRED from `??` at `:133`, low impact).
- Inheritance: the child environment is patched with `SEMIO_BUILD_TOOL: selected.tool` (VERIFIED-READ `B/📜️script.ts:101-103`). Watch callbacks and nested build children cannot change the tool (VERIFIED-READ `B/📥️caller/📤️child/🟦️.ts:27`, `B/📥️caller/🟦️.ts:79`: "Undeclared original Repo build selection").
- Consequence: a nested `repo` call inside a Bazel action must use the same tool. A mismatched nested tool is refused, not silently run on Nx.

## 4. Default tool: where it is declared, and whether Bazel should become default

Declarations of the default (VERIFIED-READ):
- `B/🔣️policy.json:3`: `"defaultTool": "nx"`. This is the single authored source.
- `B/🧭️invocation/🟦️.ts:133`: fallback chain `SEMIO_BUILD_TOOL ?? policy.defaultTool`.
- `B/🧬️schema/🔣️.json:544-561`: `BuildTool` enum is `nx|bazel`, `defaultTool` is required in the policy schema. The schema does not fix a value.
- `B/⚙️operations/🔣️catalog.json:23`: help text says "(nx)". This is hand-written and will drift if the default changes.
- `B/🕸️graph/🦀️rust/🦀️.rs:75`: the native reader only validates that the value is a tool name. It does not choose a default.
- The MCP `repo` server instructions in the session say "Nx remains the default; select Bazel with --tool=bazel". That text is external to the repo and is not authored here (INFERRED: consistent with policy.json:3).

Recommendation: do not flip the default in this wave. Flip it (change `B/🔣️policy.json:3` and the catalog string at `B/⚙️operations/🔣️catalog.json:23`) only when all of these hold:
1. One real-body (not stub-body) run under `--tool=bazel` of `@teaching/architecture-course:{build,test,typecheck}`, `@semio-tech/semio-tech-play:serve` (with `activate-prototype`) and `@semio-tech/mit-bestand-demonstrator:{build,serve}`. Current evidence uses a stub action body (`C/result-bzl-lazy.md:139`), and the production entry was blocked by a dependency lease (`C/result-bzl-real.md:9`). `C/result-bzl-lazy.md:28-35` has the per-project timings from the stub matrix only.
2. Fresh-clone `tools setup nx` and `tools setup all` are green (see gap 5).
3. A decision on remote execution: either accept that it is env-only and unused, or apply `bzl-rbe` (gap 3).
4. A decision on sandboxing, or an accepted risk list for undeclared inputs (gap 1).
5. The 244 whole-repository inputs are fixed or accepted (gap 4).

## 5. Live versus staged

| set | status | evidence |
|---|---|---|
| Groups A-G of `result-integrate` (cache-fix, rcache, rexec, bworkers, bzl-coverage, bzl-rust, bzl-xplat adapter parts, zt-e2e-fix part, tax6, vit4, fnsplit4b, ingress-perf, uncached-owners, bzl-lazy, graph-perf) | applied 14:56-15:25 per the document | `C/result-integrate.md:5-17`. Source markers confirmed live: `repo_test` at `BZ/📜️script.ts:392`, `optionalOutputs` at `:624-627`, `bazelWorkerSettingsV1` in `BZ/👷️workers/🟦️.ts:15`, `bazelRemoteExecutionV1` at `BZ/🏃️execution/🟦️.ts:48`, `bazelRemoteCacheV1` at `BZ/🌐️remote-cache/🟦️.ts:19`, `bazelCommandPlanV1` at `BZ/🧭️commands/🟦️.ts:25`. Runtime results of these sets were not repeated. |
| `bzl-rbe` (remote-execution service, Buildbarn compose) | PREPARED, not applied | `C/result-bzl-rbe.md:5`; no `🐳️service` directory in `B/🏃️execution/`; no `remote-execution` catalog entry |
| `zt-e2e-fix` frozen part (remove `@semio-tech/framework-filesystem: workspace:*`) | NOT applied | `C/result-integrate.md:52`; still present at `L/⚡️caching/🚀️bootstrap/🛠️tools/package.json:45,54` (VERIFIED-READ) |
| `hub-bzl`, `demo-bzl`, `play-bzl`, `docs-build`, `orphan-child`, `own-output`, `testfix-defect` | not in the seven integrate groups, so staged or pending | INFERRED from `C/result-integrate.md:5-17` and the `play-bzl` note in `C/coordination.md:365` |
| `bzl-sandbox` (darwin-sandbox fix) | no sign it was applied | INFERRED: not in integrate groups; the live policy still runs `sandboxing: false` (`B/🔣️policy.json:141`) |

## 6. Gaps ranked by impact

1. Correctness, high. Non-hermetic actions are `no-sandbox` (`BZ/📜️script.ts:523`) and `no-remote-exec` (`:515`). Bazel's action cache therefore trusts the declared fingerprint and source labels, not a sandbox observation. An undeclared input that changes will not invalidate a cached action. The workers also run unsandboxed (`B/🔣️policy.json:141`; `C/result-bworkers.md:20`, open item 3). Evidence to check: a sample with an undeclared file read. Not run here (INFERRED risk).
2. End-to-end evidence, high. The Bazel timings and correctness checks use a stub action body (`C/result-bzl-lazy.md:139`). The real-body path was measured only on a scratch host (`C/result-bzl-real.md:9`). `C/result-bzl-lazy.md:132` covers the existing adapter suites, not the three target products. `play-bzl` and `demo-bzl` are staged and waiting for a GO (`C/coordination.md:365`, `:394`).
3. Remote execution, high for the "readiness" requirement. The routing exists (`BZ/🏃️execution/🟦️.ts:60-68`, `--remote_executor` and `--noremote_local_fallback`), but only hermetic actions use it, and a hermetic declaration was present in 0 of 8,027 tasks at measurement (`C/result-bzl-coverage.md:7`). The service that would make it testable is unapplied (`C/result-bzl-rbe.md:5`). No CLI operation exposes readiness (catalog has no `remote-execution`).
4. Cache coverage and cost, high for performance. 2,870 of 8,027 tasks are `no-cache` (`C/result-bzl-coverage.md:7`). 18 producers have no declared outputs and no generator contract (`C/result-cache-fix.md:14`). 244 of 3,845 cacheable tasks declare the whole repository as input, 240 of them in the root `workspace` project, costing 5-17 s analysis and about 12 s hashing on a hit (`C/result-bzl-lazy.md:8`). `os-hub-ts:typecheck` (134k configured Bazel targets) took 19.9 s cold, unsharded, in the stub matrix (`C/result-bzl-lazy.md:53`).
5. Zero-touch setup, high. `ensureBuildEnvironmentV1` runs for every command except `tools`, `reset` and `cache` (`B/📜️script.ts:132`) and installs all JavaScript dependencies (`B/🚀️bootstrap/🟦️.ts:47-49`), so a Bazel run still needs the full Nx-bearing install. The Nx setup is broken on fresh clones because of the `framework-filesystem` workspace link (`C/result-bzl-xplat.md:12`, unapplied per gap in section 5). A Bazel run also waits on the dependency storage lease (`C/result-bzl-real.md:9`).
6. Test routing, medium. `bazel test` covers only test-family tasks with at most 10,000 inputs (`BZ/🧭️commands/🟦️.ts:22`, schema `:12`) that no other task consumes. Larger tests run as build actions with no test-result cache. The two staged reports do not agree on the count: `C/result-bzl-lazy.md:10` says 1,696 of 1,817 cacheable test-family tasks; `C/result-bzl-coverage.md:9` says 1,789 of 8,027 tasks. Not reconciled.
7. Affected plan, medium. `affected --dry-run` uses the neutral rule (`B/📜️script.ts:210`), while the real run uses Bazel `rdeps`. They diverge for lockfile-only changes (`C/result-bzl-lazy.md:11`). A user reading the plan gets a different answer than the run.
8. Per-action overhead, medium. Every task re-enters bun through `📜️script.ts birth`. The worker measurement shows 2.50 to 1.46 CPU-seconds per action after workers (`C/result-bworkers.md:8`), but the graph store is reopened on every warm request (about 6.2 s, `C/result-bworkers.md:72`, open item 1). The per-worker module cache and in-process leaf execution are built but not wired (`C/result-bworkers.md:79`, open item 4).
9. Cross-platform, medium. Only macOS arm64 and a Linux arm64 container were run, both on staged copies. Windows was not run (`C/result-bzl-xplat.md:7`). A non-ASCII install base crashes Bazel on Linux (exit 37, `C/result-bzl-xplat.md:13`). The schema guards non-ASCII only for `install`, `outputs` and `server` on darwin (`BZ/🧬️schema/🔣️.json`, `HostPaths`).
10. Documentation mismatch, low. `C/result-bzl-lazy.md` states "No `--experimental_*` flag is used anywhere", but live code uses `--experimental_disk_cache_gc_max_size` (`BZ/💾️storage/🟦️.ts:46`), `--experimental_worker_*` and `--experimental_total_worker_memory_limit_mb` (`BZ/👷️workers/🟦️.ts:40-41`), and `.bazelrc` lines 6-7 set experimental GC flags. Treat the claim as wrong.
11. Bypass by design, low. `repo cargo` runs cargo directly, so Rust outside tasks never goes through Bazel (`B/📜️script.ts:128-131`). Native dashboard and recovery bypass the tool (`B/📜️script.ts:93`, `:110`). Document these, do not route them.
12. Empty tool value, low. `SEMIO_BUILD_TOOL=""` throws instead of defaulting (`B/🧭️invocation/🟦️.ts:133`, `:144`).

## 7. Checked and not a gap

- No fallback from the Bazel path to Nx exists in the dispatch (VERIFIED-READ `B/📜️script.ts:193-230`).
- Lazy closure is real: the generated BUILD contains only the planned selection (VERIFIED-READ `BZ/📜️script.ts:588-589`), and the staged measurement shows Bazel executes only the closure (`C/result-bzl-lazy.md:7`, verdict 1; the closure claim itself was measured on a stub body).
- Disk cache is bounded by a policy share of the products partition: `productsShareBasisPoints: 4000` in `BZ/🧬️schema/🔣️.json` `CachePolicy`, applied at `BZ/💾️storage/🟦️.ts:35-37` (VERIFIED-READ).
