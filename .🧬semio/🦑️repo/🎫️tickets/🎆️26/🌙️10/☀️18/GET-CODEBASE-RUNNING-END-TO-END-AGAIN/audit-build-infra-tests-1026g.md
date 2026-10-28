# Build-infra test coverage audit, wave 1026g

Auditor `aud-tests-g`, read-only. Scope: the build-system areas this wave changes. Task file: `brief-wave-1026g.md`. No test was run, nothing was built, no source file was edited, and no git command was run. All numbers come from static analysis of the checkout as of 2026-10-26 (scripts in the audit scratch directory, `/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/`).

## Summary

- **Two closed route tables route every entry they cover.** `⚡️caching/🗂️suites/🔣️.json` (98 suites, 297 test entries) and `🏃️process/🧪️testing/🗂️suites/🔣️.json` (63 suites, 225 entries) show 0 unrouted, 0 duplicated, 0 missing and 0 stale-helper rows when the discovery and coverage rules of `🧪️testing/🗂️suites/🟦️.ts` are re-implemented in Python. Both tables also have an independent Python oracle (`🗂️suites/🧪️tests/🐍️oracle/🐍️.py` and `🧪️testing/🗂️suites/🧪️tests/🐍️oracle/🐍️.py`). The re-implementation was not executed; the bun test that enforces the same rule was not run.
- **The build library has no closed table.** Of its 728 non-caching TS test entries (files under `🧪️tests` that import the test API), 329 are reached through hand-written routes (literal route paths, the build script list, and imports from routed files); the rest are not reached at all. The routes are: the `test-workspace-build` target (root `📋️project.json`, `🏗️build/📜️script.ts test`, 29 explicit files plus their import closure), the `test bazel` and `bazel-bootstrap` targets (phase name patterns over `🔧️tools/🏗️bazel/🧪️tests/🟦️.ts`), and the 168 `test-*` targets of `📦️packages/🟦️typescript/📋️project.json` that dispatch to literal paths in `🧪️tests/🔬️typescript-package-commands/*-routes`. Static reachability (literal route paths, relative imports, route-to-test closure) leaves **399 TS test entries unreached** (appendix). Route-only literal references were used; references inside JSON ledgers, fixtures and schemas were not counted as routes.
- **Unrouted tests that matter most for this wave:** the generation coverage, pin, load and fact tests under `🏗️build/🕸️graph/🕰️generations/` (their coverage oracle `🔭️coverage/🧪️tests/🐍️oracle` is unreached); the SLO and process-tree memory bench `🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts`; the memory-governor model `🏗️build/🧠️memory/🧪️tests/🟦️.ts` (Python oracle `🐍️.py`); the Bazel session concurrency test `🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🔬️concurrency/🟦️.ts` (the only Bazel test that measures process-tree RSS, via `ps -eo rss`); and `🏗️authoring/📦️crate-tree/🧪️tests/*` (21 files, the `test crate-tree` target runs a different file).
- **Gaps by requested type:**
  - *No memory-bound test (measured ceiling):* none in `leases`, `churn-observation`, `script-inputs-closure`; the memory-based evidence elsewhere is either synthetic (`host-capacity` fixtures use `totalMegabytes`) or a byte budget rather than RSS. The one real RSS measurement (Bazel concurrency) is unrouted. The static search found no routed test that asserts the 2 GB builder RSS ceiling for graph generations, which the brief names as acceptance.
  - *No multi-process concurrency test (static co-occurrence of a spawn with `Promise.all`/`race`):* every area has such entries, so no area is empty at this level. The strongest true multi-process case, the Bazel session concurrency test, is unrouted. The rest of the `Promise.all` uses appear to stay in one process (for example leases `🏎️overlap`, the `🧹️collector` racers, graph `🗂️generation-files` workers); the scan cannot prove separate OS processes.
  - *No cross-platform branch test:* `leases` has one `win32` reference (a skip in the overlap test) and no Windows lock branch; `churn-observation` has one `win32` reference (a pure function called with the string `"win32"`); `script-inputs-closure` has three. The other areas have 6 to 21 win32 references each.
- **Test entries outside the library tree:** 9 TS test entries (5 under `🔨️modules/🚀️launcher/🧪️tests`, 4 under `🔨️modules/🖥️server/🎛️coordinator/🏗️builder`) are not named by any route-like source. Their only mentions are JSON ledgers, fixtures and caller lists. They are listed in the appendix as candidate orphans, unverified because those modules were outside this audit's roots.

## Official routes (what runs what)

| Route | Where | Covers | Verdict |
|---|---|---|---|
| Caching suite table | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🔣️.json` (+ `🧪️tests/🟦️.ts`) | Caching module, 98 suites, `test-<id>` targets in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json` | Closed: 297 entries, 0 unrouted. Each entry is its own process; oracle present |
| Process suite table | `🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🔣️.json` | Process module (`🏃️process`), 63 suites incl. native storage, leases, host capacity, cargo workspace | Closed: 225 entries, 0 unrouted. Oracle `🧪️tests/🐍️oracle/🐍️.py` |
| Build test script | root `📋️project.json` `test-workspace-build` (L1572-1620) runs `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts test` | Graph, observation, preflight, inference, scheduling (caching) and parts of storage | Explicit list: 29 files, closure 867 files incl. helpers. One `bun test` process for the list |
| Bazel phases | root `📋️project.json` L5820-5836 (`test-build-tool-bazel-bootstrap` and its sibling) run `build/📜️script.ts test bazel` | `🔧️tools/🏗️bazel/🧪️tests/🟦️.ts`, selected by `--test-name-pattern` per phase (cache, nested, uncached, birth, services, refusals, cancellation, configuration, offline) | Umbrella file imports sessions, storage, cargo, workers, and more. The concurrency test is not imported |
| TS package routes | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` (168 `test-*` targets) → `📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/*-routes` | Library-wide laws and build routes such as `build-mode`, `build-surfaces`, `build-source-demand`, `crate-tree` | Hand-listed literal paths; `crate-tree` routes `🧪️tests/📦️crate-tree/🟦️.ts` only |
| Other build targets | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/{📖️storybook/💾️storage, 📊️reporting, 📥️inputs/💾️storage, 📥️inputs/📂️membership, 🧭️selection/📥️closure}/📋️project.json` | Small sub-trees | Not analysed beyond existence |

## Area map

Columns: TS entries = files under `🧪️tests` that import the test API. Routed = by process table, caching table, or library route. Unrouted = not reached statically, including the 9 entries outside the library tree (no route-like source names them). Spawn = the file starts child processes. Co-spawn = spawns and uses `Promise.all`/`race`/`Worker` in the same file. Mem = real measurement constructs (RSS, `ps -eo rss`, GiB, MEGABYTE, totalMegabytes, process memory). Python = `🐍️` oracle files in the area and entries that call them.

| Area | TS entries | Process table | Caching table | Library route | Unrouted | Python oracle files | Entries calling Python | Spawn | Co-spawn | Mem | Platform (any) | win32 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Graph inference, generations, coverage | 128 | 0 | 45 | 30 | 53 | 39 | 73 | 72 | 18 | 3 | 13 | 9 |
| Leases | 27 | 24 | 0 | 1 | 2 | 7 | 19 | 18 | 11 | 0 | 2 | 1 |
| Host capacity | 34 | 13 | 14 | 2 | 5 | 6 | 19 | 24 | 6 | 8 | 14 | 6 |
| Bazel adapter (sessions, scopes) | 62 | 0 | 3 | 32 | 27 | 8 | 45 | 50 | 10 | 1 | 26 | 19 |
| Cargo custody and launcher | 95 | 14 | 40 | 13 | 28 | 8 | 50 | 58 | 9 | 3 | 17 | 14 |
| Churn and observation | 18 | 2 | 0 | 5 | 11 | 5 | 10 | 10 | 3 | 0 | 1 | 1 |
| Native storage | 132 | 51 | 60 | 13 | 8 | 8 | 85 | 61 | 19 | 1 | 47 | 21 |
| Cache GC | 40 | 3 | 21 | 3 | 13 | 3 | 20 | 22 | 3 | 3 | 9 | 6 |
| Script inputs, router, closure | 56 | 4 | 7 | 18 | 27 | 8 | 29 | 37 | 6 | 0 | 3 | 3 |

Notes on the columns: `win32` counts entries whose code names the `win32` platform (a branch or a platform argument); `Platform (any)` counts entries naming any of process.platform, win32, darwin or skipIf on a platform (includes fixture fields); `Mem` counts the strict measurement pattern, which includes `totalMegabytes` fixtures in host-capacity, so host-capacity Mem=8 is synthetic sampling data, not measured RSS.

### Per-area detail

**Graph inference, generations, coverage**

- Oracles are present for generation protocol, pin, coverage queue, fact codec, model and inference tests (Python files under `🕰️generations`, `🔭️coverage`, `🧷️pin`, `🧮️fact-index`). They are not reached by any official route, so the verified coverage stamp (graph-f) has no gate in the route tables. No route file names `🔭️coverage`; it appears only in source modules and the README.
- Routed: the caching `🕸️graph` tree (45 entries via the caching table) and the build-script list for inference and target policy (`🕸️graph/📥️inference/*`).
- Concurrency: 13 entries combine spawn and `Promise.all`, for example `🏊️project-farm`, `🔭️coverage/🧪️tests/🤝️queue`, `📥️scope`. The brief's builder RSS ceiling (below 2 GB) and the "only affected owners re-inferred" check are not asserted by any routed test. The memory-bound constructs found are in `🧾️closures` and `🏁️startup/🚏️admission`.
- Platform: 9 entries have win32 branches; several are unrouted (for example `🏊️project-farm` and `🕰️generations/🧪️tests/🟦️.ts`).

**Leases**

- Route: 24 of 27 entries are in the process table (`🔒️leases` tree). Two unrouted entries remain.
- Multi-process: 18 spawn entries, 10 with `Promise.all`/`race`. The overlap test `🏎️overlap` drives concurrent compiles in one process; the custody and sweep tests spawn child processes as well (not verified beyond the spawn-token scan).
- Memory: no measured ceiling. Platform: the overlap test skips Python on win32; the descriptor test skips unless darwin or linux. No Windows lock branch is tested.

**Host capacity**

- Route: 13 process-table and 14 caching-table entries; 5 unrouted.
- Memory: fixtures use `totalMegabytes:32768` in the host helpers, and 16384 or 16 GB fixtures appear in 5 host and capacity test files. This is governor logic on synthetic samples. The static search found no routed test that compares a real host sample with the 16 GB policy.
- Platform: 14 entries name darwin, linux or win32 (mostly sampler fixture fields, not branches).

**Bazel adapter (sessions, scopes)**

- Route: 32 library-route entries (build script or bazel umbrella), 3 caching table, 27 unrouted.
- Unrouted and important: `🧵️sessions/🧪️tests/🔬️concurrency/🟦️.ts` (spawns processes, uses `Promise.all`, measures server RSS with `ps -eo rss`, no Python oracle). It is imported by no route. The sessions umbrella (`🧵️sessions/🧪️tests/🟦️.ts`), which the Bazel umbrella imports, does not import it.
- Platform: the highest win32 count after native storage (19 entries). Oracles: 8 Python files; 45 entries call Python.

**Cargo custody and launcher**

- Route: 40 caching table, 14 process table, 13 library-route, 28 unrouted (4 of them outside the library tree). Non-TS files: go 10, feature 13, Python 8, rs 2; most go and feature files sit under `🚀️launcher`.
- Memory: 3 entries mention memory-like budgets (`⏱️timing`, `🧹️gc/📈️fill`, `🗃️unit-cache`), not measured RSS. Multi-process: 58 spawn entries; 9 co-spawn.
- Platform: 14 win32 branches (launcher pin and install contracts, cargo execution).

**Churn and observation**

- Route: 2 process table, 5 library route, 11 unrouted, among them the `📏️incremental/🤝️concurrent` test.
- Concurrency: `⚡️incremental` runs `Promise.all` over builder runs (the process model was not checked); the `🤝️concurrent` test drives its runner through callbacks. Rename-mid-run, exit-75 and ENOENT assertions appear in observation tests (`👁️observation/🧪️tests/🟦️.ts`, `🪪️presence`, `📦️projects`) and in `📏️incremental/🏃️runner`; their route status is in the appendix.
- Memory: 0 entries match the measurement pattern. Platform: one win32 branch (`defaultJobserverStyleV1("win32")`).

**Native storage**

- Route: 60 caching, 51 process, 13 library, 8 unrouted (2 of them outside the library tree). Most entries sit in the closed tables.
- Multi-process: 61 spawn entries and 16 co-spawn, including the ledger gate `💾️storage/🧪️tests/📒️ledger` (a child process holds the ledger behind a gate file, `gate-ready` and `gate-stop`).
- Memory: 1 entry matches the strict measurement pattern (the ledger file, not verified). Platform: 21 win32 branches, the most of any area. Oracles: 8 Python files.

**Cache GC**

- Route: 21 caching, 3 process, 3 library, 13 unrouted. Four build-level `🧹️cache-gc/🧪️tests` files (directory, verify, launch, root) are unrouted.
- Concurrency: the collector test races 12 `acquireRemoteCacheLockV1` calls with `Promise.all`. Memory: byte budgets (`maximumBytes`) in the collector and physical-retention tests; no RSS ceiling.
- Platform: 6 win32 branches. Oracles: 3 Python files.

**Script inputs, router, closure**

- Route: 7 caching, 4 process, 18 library, 27 unrouted (2 of them outside the library tree). The router closure test (`🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts`) is unrouted.
- Memory: none measured. Concurrency: 5 spawn-plus-Promise entries (revision, continuations, closure). Platform: 3 win32 references (revision, lean inventory, stable file).
- Oracles: 8 Python files; 29 entries call Python.

## Coverage gaps by requested type

- **No memory-bound test (measured ceiling):** `leases`, `churn-observation`, `script-inputs-closure` (0). Weak: `graph-generations`, `cargo-custody`, `cache-gc` (3 each, budget or size constants, not RSS). The real RSS measurement (`🧵️sessions/🔬️concurrency`) and the process-tree memory bench (`📏️incremental/🏎️bench`) are both unrouted. The memory governor model `🏗️build/🧠️memory/🧪️tests/🟦️.ts` is unrouted too.
- **No multi-process concurrency test that is routed and proves separate OS processes:** the Bazel concurrency test is unrouted; churn-observation has the thinnest multi-process coverage (3 co-spawn entries; the `🤝️concurrent` test is unrouted). The other areas have routed co-spawn entries, but these were classified by static co-occurrence only, not run.
- **No cross-platform branch test:** `leases` (no Windows branch), `churn-observation` (one pure win32 function), `script-inputs-closure` (3 references, none for a Windows lock or path case). Linux and darwin are split in `host-capacity`, `native-storage` and `cache-gc`.

## Orphans: test entries not reached by any official route

Method: a TS entry is reached when a route-like source (TS or `📋️project.json`, not JSON fixtures, ledgers or schemas) names its path literally, when a route-reached file imports it (relative imports followed transitively), or when the build script list or a repository-level route names it. Process and caching entries are covered by their closed tables and are not listed. Dynamic path construction is not analysed, so each entry below is a candidate until a runtime check confirms it.

Library tree: 399 TS entries, plus 9 TS entries outside the library tree (5 under `🔨️modules/🚀️launcher/🧪️tests`, 4 under `🔨️modules/🖥️server/🎛️coordinator/🏗️builder`) that no route-like source names. Non-TS test files (Python oracles, feature, Go and Rust files) are not listed.

### graph-generations (53)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🚦️governed-dispatch/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/📐️declared-inputs/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🔗️late-closures/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🔗️workspace-closure/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🖨️print-documents/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🗜️interned-closures/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🙈️presentation-resources/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🦀️compiled-closure/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧩️component-cache/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧮️covered-inputs/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌊️json-stream/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌍️environment/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🚧️withheld-dependency/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧪️tests/🤝️queue/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/♻️refine/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️stream/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌱️unborn-root/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/📋️summary/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🧮️fact-index/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🧱️stable-directories/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🧶️shared-strings/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🧹️retire/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧷️pin/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧷️pin/🧪️tests/🤝️nested/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧷️pin/🧪️tests/🤝️plan/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🗂️generation-files/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/🪢️trim-paths/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/📤️portable/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🔓️sealed-reuse/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🔬️analysis-providers/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🕊️graph-release/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🖼️canvas-runtime-assets/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🪪️generation-identity/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧭️file-owner/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️collect-errors/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧲️owner-claims/🟦️.ts
```

### leases (2)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🕊️release/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔒️leases/⌨️command/🧪️tests/🟦️.ts
```

### host-capacity (5)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/📖️texts/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔁️restart/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🧪️tests/🟦️.ts
```

### bazel-adapter (27)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🔧️bazel/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🔧️bazel/🧭️flags/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🧱️workspace/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/📥️strategies/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️shim/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🐹️go/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👁️evidence/♨️warm/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👁️evidence/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🔒️planned/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔁️probe/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🚦️test-classes/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🚦️test-classes/🧾️witness/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️closure/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️keys/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️mode/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧰️toolchains/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🔬️concurrency/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧾️test-results/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🧪️tests/🔁️churn/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🧪️tests/🟦️.ts
```

### cargo-custody (24)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/⏱️timing/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🧪️tests/🍎️image/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🧪️tests/🎟️phase/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏭️inventory/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/📥️capture/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️entrypoint/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🏃️command/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📖️toml/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📤️arguments/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🔁️replan/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/📥️original-control/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/📦️identity/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🕰️queued-preparation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🪆️nested-owners/🟦️.ts
```

### churn-observation (11)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/⏱️analysis-stamp/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📒️registry/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📥️reuse/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧊️verified-blocks/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧠️retention/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧭️resolution/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧱️block-cost/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧹️superseded-blocks/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧾️record-header/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🤝️concurrent/🧪️tests/🟦️.ts
```

### native-storage (6)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/💾️storage/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🛑️primary/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🪟️retirement-window/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🪪️identity/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/📥️consumption/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/🟦️.ts
```

### cache-gc (8)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/📁️directory/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🔬️verify/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🚀️launch/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️test-level-budgets/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧮️path-budget/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧮️tracked-path-budget/🟦️.ts
```

### script-inputs-closure (25)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🛗️zero-touch-closure/🧪️tests/🛗️closure/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🦀️exporters/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📂️membership/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/📌️received-pin/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🖼️captured-image/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🧊️own-stamp/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🧓️stale-reader/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/✂️closure-chunks/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/📏️windows/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🔭️trace/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🚫️cache-root-refusal/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧱️static/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/📥️closure/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🔗️relative-closure/🧪️tests/🟦️.ts
```

### outside the nine areas (238)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/📇️indexed-generated-output/🧪️tests/🗂️indexed-generated-output/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/📋️build-owner-registry/🧪️tests/📋️registry/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🔎️scan/🧪️tests/🔎️scan/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗂️files/🧪️tests/🗂️files/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🌱️database-lifecycle/🧪️tests/🌱️lifecycle/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🧭️all-app/🧪️tests/🧭️all-app/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🧪️tests/🗣️corpus/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🚷️raw-process-spawn/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🛰️service-invocation/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation/🧪️tests/✍️handcrafted/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/📐️merge-policy-law/🧪️tests/📐️merge-policy/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🧪️diff-only-law/🧪️tests/🧪️diff-only/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🧪️tests/🎯️outcome/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🧪️tests/🏗️shape/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏛️foundation/🧪️tests/🏛️foundation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/📋️allowlists/🧪️tests/📋️allowlists/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/📐️schema-overhaul-pc/🧪️tests/📐️overhaul/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🧪️tests/🔌️architecture/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🧪️tests/🏷️naming/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/📚️examples/🧪️tests/📚️examples/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🧪️tests/🗺️shape/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🧱️lib-and-barrel-shape/🧪️tests/🧱️wiring/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🧪️tests/🚧️bans/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚪️artifact-io/📋️owner-table/🧪️tests/📋️owners/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🔁️dialects-and-round-trip/🧪️tests/🔁️dialects/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🔌️io-matrix/🧪️tests/🔌️matrix/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🗂️stdio-catalog/🧪️tests/🗂️catalog/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🧾️standards/🧪️tests/🧾️standards/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧊️dissolved-kernels/🧪️tests/🧊️kernels/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2/📇️stdio-entries/🧪️tests/📇️entries/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2/🚫️machinery-bans/🧪️tests/🚫️machinery/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2/🧪️tests/🧬️overhaul-s2/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2/🧱️grammar-honesty/🧪️tests/🧱️honesty/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2/🪞️facet-mirror-drift/🧪️tests/🪞️drift/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🧪️tests/🧭️wiring/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧮️completeness/🧪️tests/🧮️completeness/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧱️app-coupling/🧪️tests/🧱️coupling/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🧪️tests/🧼️mechanism/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🪆️subset-conformance/🧪️tests/🪆️conformance/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧪️self-test-lanes/🧪️tests/🧪️lanes/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧰️direct-tool-declarations/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧰️direct-tool-spawn/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/📖️texts/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🏷️labels/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📅️dates/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📈️attribution/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🎨️format/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🔀️deltas/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🔢️uloc/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🛤️path-rules/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🧮️summary/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📋️prepare-commands/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📖️texts/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📤️emit/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🔖️tags/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🔢️counter/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🔸️bullets/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🧭️path-prefixes/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🪜️steps/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎨️styling/🏗️builder/🐍️python/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🚪️entrypoint/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌲️inline-mounts/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📏️measure-union/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🏷️naming/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧭️module-chains/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧰️tree-helpers/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🌳️use-tree/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🧰️support-hoist/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧰️testkit/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧱️decl-tree/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🏃️run/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/📝️manifest/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔁️cycle-memo/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🧵️use-tree/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🗂️collection-catalog/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⏱️profile/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔤️options/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🛗️requires/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🔬️action-result/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧹️collector/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/⌨️command/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/📡️wire/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🔒️sandbox/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🗓️scheduler/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🧪️tests/🔗️shims/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧪️tests/🎯️coverage/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/⌨️command/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏊️delegate/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏗️provision/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏛️service/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏭️executor/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/📨️protocol/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔭️watch/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🕰️graph-hooks/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🗃️state/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🚪️client/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📊️reporting/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📐️conformance/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/🌐️globals/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/🚪️door/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔌️backends/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/📨️envelope/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🪵️failure-tail/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🚦️startup/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/📚️libraries/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/🧩️units/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧩️units/🧪️tests/⌨️command/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧩️units/🧪️tests/🌐️languages/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧩️units/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/📌️pinned-reader/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🧊️isolated-plan/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🧊️taxonomy/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-registry/🧪️tests/🔎️exec-command/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⌨️command/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/🧪️tests/📮️dispatch/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧵️workers/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧪️tests/🔁️implementation-edit/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧪️tests/🔁️stale-codes/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🧪️tests/⌨️command/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🧪️tests/🔎️probes/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️agent-sessions/⌨️command/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📇️catalog/📣️publication/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📇️catalog/🚚️deployment/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📋️policy/⏯️tool-run/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📖️texts/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/⚙️configured/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🏺️vendored-exclusion/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🔑️sdk-foundation-admission/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🚫️resolved-boundary-classification/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/📖️texts/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests/🏘️execution/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🧪️tests/🦀️native/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🧪️tests/🦀️native/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📁️data-directories/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📄️manifest-projection/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📣️publication/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📦️payload/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧬️metadata/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🔒️allowlist/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🖥️cli/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🧭️entrypoints/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧪️tests/🧫️probe/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🎚️levels/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🎛️repository-command/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🗄️lcov/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️taxonomy-scaling-laws/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/☂️frozen-coordinate-wildcard-coverage/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️vite-config-loader/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏠️owner-projection-policy/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️executable-owners/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️executable-owners/🧭️ownership/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️micro-commit-staged-index/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🏪️managed-storage-protection/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🧾️owned-preview-refusal-reason/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔷️typescript-project/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖼️runtime-taxonomy-asset-paths/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗂️mutation-leaf-case-table/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚨️transaction-sentinel-policy/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛂️mutation-owner-descriptor-roots/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️mutation-fixture-coverage/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧊️fixture-payload/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️script-async-runners/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-scenario-identities/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧰️toolchain-pins/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️dependency-selectors/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️package-implementation-admission/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🔗️participation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🧾️attributes/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🪵️root/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️reference-reach/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧵️cad-bindings/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧷️member-slug-pattern/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧹️clean-ticket-runs/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪜️lift-kids/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪜️port-layering/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪞️unmirror/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪡️planner-blockers/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🏗️source-services/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📥️checked-inputs/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-implementation-destination/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🔣️taxonomy-input/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission-io/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️candidate-discovery/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️empty-folders/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️marker-only-folders/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🗑️removal/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🛡️protection/🧪️tests/🟦️.ts
```

### Outside the library tree (9)

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/💾️storage/🧪️tests/🔌️nx/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/💾️storage/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🔌️nx/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🧪️tests/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📥️install-contract/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🗂️registry-schema/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🧱️cache-isolation/🟦️.ts
🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🧱️cache-isolation/🧮️runner/🟦️.ts
```

## Caveats

- Static only: no test was executed, no build run, no graph loaded. The route-table emulation reproduces `discoverProcessTestFilesV1`, `routeProcessTestFilesV1` and `processTestCoverageV1`, but the authoritative check is the bun test in `🗂️suites/🧪️tests/🟦️.ts` (caching) and the process module test, which were not run.
- "Unrouted" means no route-like literal reference and no import reaches the file. A test could still run through a directory walk that is not visible in source; the repository-wide search found none for the listed files.
- Co-spawn and memory counts are regex evidence (spawn tokens, `Promise.all`, RSS and MEGABYTE tokens) and may include helpers or comments. Use them to rank, not to prove a property.

