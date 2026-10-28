# result-testmem-g (wave 1026g, executor testmem-g): one worker policy for every test runner, derived from the admitted grant

Tools `C/testmem-g-tools/` (`tree-rss.py` sampler, `m1.sh`..`m6.sh` scenarios, `vt-cmd.ts`/`vt-run.py` vitest driver, `pw-browsers.mjs`, `tsconfig.testmem-g.json`), raw rows `C/🗑️generated/testmem-g/*.jsonl|json` (tool output, delete with the ticket). No sub-agent, no ticket tool, no git command, no `[DEBUG]` line left. Graph-free throughout (direct suite runs); no official run yet (WAITING was 4 and load 27-37 all morning).

FOR ts-g: `🏗️build/🟦️typescript/🧪️tests/🔁️incremental/🟦️.ts` is unrouted (law `every test file of the process module belongs to exactly one suite` fails): add it to the `compiler-state` suite files in `🧪️testing/🗂️suites/🔣️.json` (+ project target stays).
FOR coordinator: all my edits landed before the 10:30 window; nothing is staged. The official dashboard `test-unit` / `course:test` before/after is OPEN (window rule, WAITING 4, load 41).

## Status (newest first)
- 10:10 FINAL (graph-free): 18 tests green (workers 10 + learning 4 + library 3 + policy 1), vitest course scenarios measured (table). Reading of the vitest rows: wall of the 1-worker row is load, CPU 46-55 s in every row; the policy under one 2048 MB job runs 4 workers at 1274 MB (unbounded 12 workers: 1619 MB, -21 %), under a 1300 MB job 1 worker at 1012 MB (-37 %).
- 10:20 LANDED + GREEN (graph-free): the policy module, its laws, the wiring of all five runners, the end-to-end cap test. Section "Measured" is filled from the scenarios that finished; the official dashboard `test-unit` run is OPEN (needs WAITING < 4).

## What was wrong
- `bun test` / suite runner: `maximumParallelFiles: suite.parallel ?? 1` (declared per suite, never bounded by the admission), 0 suites declare it. The "2 GB per worker" of the RAM census is the build-library-closure suites of keys-g; a typical `bun test` file process peaks at 80-500 MB (17 files measured, p50 150, p90 ~480, max 500 MB; `--smol` saves < 10 %).
- vitest: ts-g's constant `SEMIO_TEST_WORKERS` default 4, independent of grant and learned size.
- cargo/nextest: `assertionThreads = cores - ceil(cores/4)` (7 of 10) and only at level `fundamental`; at `quick` (the dashboard `test-unit`) nextest ran `num-cpus` = 10 test processes regardless of the grant. The command configuration had its own copy of the same formula.
- Playwright: `workers: 1` in the config and a constant `SHARED_WORKERS`; the two topologies (dev + rehearsal, each a proctor + vite site + chromium) ran concurrently whatever the grant carried.

## The policy (schema first, one place)
`🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/` (public facade `@semio-tech/process/testing/workers`, library facade `@semio-tech/repo-lib/testing/workers`)
- `🧬️schema/🔣️.json` (Policy, Runner, Facts, Plan, Corpus, EN+DE messages) and `🔣️.json` (the authored policy: per runner `override` variable, `baseMegabytes`, `workerMegabytes` (measured peak RSS), `wanted` (integer or `cores`), `maximum`, `oversubscription` (workers one admitted core may carry)).
- Facts of a process: with a grant (`SEMIO_HOST_CAPACITY_PLAN`) its SHARE = one job: `memoryMegabytes/jobs` (fixed operations: `fixedMegabytes`) and `cpu/jobs` cores; without a grant the host share (50 % of RAM, all cores). Why one job: sibling tasks of the same run inherit the same plan, so a runner may count only on its own job; the sum over concurrent tasks can then never exceed the grant.
- Plan: `workers = max(1, min(wanted, memoryCap, coreCap))`, `memoryCap = floor((memory - base') / worker')` with `base', worker' = declared x learned ratio` (ceil), `coreCap = cores x oversubscription`; `bound` names what decided (override, request, wanted, maximum, cores, memory). Override variable (integer 1..maximum, else a refusal naming the variable, EN+DE) is still capped by the admitted cores. A reduced count prints `[tests] Vitest: 2 of 4 worker(s), bounded by the admitted memory (...)` (DE `[tests] Vitest: 2 von 4 Worker(n), begrenzt durch den zugelassenen Speicher (...)`).
- Learning reuses ram-g's store (no second store): class lineage `test-worker-<runner>` <- `test-worker-<runner>:<suite|project>`, outcome = observed peak of the process tree (`observeTreeMegabytesV1`, sampled every 1 s) against the declared size of the plan, recorded through `recordHostOutcomeV1`/`updateHostEstimateStoreV1` (same file `estimates.json`, same lock, same LRU, failed runs teach only when they exceed the estimate). A runner never pools with another runner (first measured defect: a shared root class made a vitest sample shave Cargo's size).
- Wiring (all through the one policy, nothing else computes a count any more):
  - suite runner `🗂️suites/🏃️run` (`bun-test`, `request = suite.parallel ?? 1`, learning per suite id via `workers.learning`; library callers `🏗️build/🗂️suites/📜️script.ts` and `⚡️caching/📜️script.ts` pass the build's store);
  - dashboard `bun test` calls (`TestScript.bun`): planned (request 1) + learned per label;
  - vitest: library `repositoryVitestPolicyV1` (replaces ts-g's `testWorkersV1`/`SEMIO_TEST_WORKERS`; now `SEMIO_VITEST_WORKERS`) and `runVitest` (plan, announce, learn), command configuration (`vitestPolicy.maxWorkers`);
  - Cargo/nextest: `assertionThreads` from the policy at EVERY level in `cargoTestPlanV1` (nextest, plain, llvm-cov; a caller's own `--test-threads` wins and appears exactly once), library `repositoryCargoTestPolicyV1`, command configuration (`assertionThreads: "planned"` replaces the token `available-minus-quarter` in the authored configuration + schema); new `CargoTestAdmissionV1.assertion` hook wraps only the assertion phase so the learned size is the test threads, not rustc;
  - Playwright: workers per topology from the policy (`--workers N` always forwarded unless the caller named it) and the concurrent topologies (`e2e-stacks`, serial when the share carries one stack only) in `🎭️e2e/🟦️.ts`; the stacks are learned around the whole session.
- Docs: `📓️build-guide.md` item 5 (EN + DE).

## Laws and oracles (all green, graph-free, `nice -n 10 bun test ./<file>`)
- `👷️workers/🧪️tests/🟦️.ts` 10 pass: Ajv (third party) admits policy and corpus and agrees with the in-repo validator on 9 refusals; all 885 stored vectors (grid of 3 core counts x 4 memory sizes x 3 ratios x 3 requests x 5 runners, 40-core host, override texts incl. refusals, 160 exact threshold vectors at +-1 MB for 5 ratios) equal the independent Python model `🐍️oracle/🐍️.py` (also run live in the test through `testPythonV1`; `python3 🐍️.py --check`); 7 facts vectors (no grant, 8 GB laptop, 1 job, 4 jobs, 2 cores per job, fixed operation, empty variable); property laws over a 5x7x3 grid with the AUTHORED policy (never above maximum, cores x oversubscription or the memory of the share unless the floor of one; monotone in memory and cores, antitone in the learned size; override never leaves the admitted cores); the 16 GB profile law; Cargo plan law (16 combinations of level x nextest x coverage: threads exactly once, caller's wins); authored command configuration passes the closed schema, the old token is refused; messages EN+DE.
  Negative control: replacing `ceil` by `floor` in the size scaling fails the corpus test (the first corpus had round sizes and did NOT detect it; the policy of the corpus now uses odd sizes and threshold vectors).
- `👷️workers/🧪️tests/📐️learning/🟦️.ts` 4 pass: lineage, closed learning loop (5 runs at 50 % -> ratio 5000-7000 -> the same share carries more workers; sibling class borrows, other runner untouched; failed run cannot shrink, a failed run above the estimate can raise; zero peak teaches nothing), observer peak/time to peak with injected clock, `withTestWorkerLearningV1` records once and rethrows.
- `👷️workers/🧪️tests/🧱️capped-run/🟦️.ts` 1 pass (END TO END): the REAL suite runner launches a fixture suite (6 bun test files, each allocates 120 MB and holds it 1.5 s, a barrier makes every file wait until the allowed number run at once so a slow start cannot hide concurrency) under (a) a small grant (1 job, 2 cores, 800 MB share): exactly 2 files at once, process-tree RSS grew by +290 MB (bound 800), the reduced line printed once; (b) a roomy grant (8 cores, 4096 MB): 6 at once, +828 MB (non-vacuous: the cap is what held (a) back); (c) override 1: 1 at once; (d) override 6 under (a): still 2 (never beyond the admitted cores).
- Library: `🏗️build/🌡️host-capacity/🧪️tests/🧪️workers/🟦️.ts` 3 pass (store = the build's estimate policy + gate directory; 4 observed runs teach `test-worker-vitest:spec` and shrink the next plan; override variable reaches the plan; Cargo policy of an owner: 10 without grant, 3 by override, 4 under one job); `🏃️process/📋️context/🔣️policy/🧪️tests/🟦️.ts` 1 pass (rewritten for the new API); command suite `🧭️routing/🎛️command/🧪️tests/🟦️.ts` 5 pass with `SEMIO_TEST_ARTIFACT_DIR` set.
- `tsc --strict` over the 21 touched/new TS files (`C/testmem-g-tools/tsconfig.testmem-g.json`): 0 errors in my files (2 errors in `🕸️graph/🟦️.ts` are a peer's open edit: `sweepWaitMilliseconds`, `stamps`).

## Measured
Rows in `C/🗑️generated/testmem-g/`; machine load 21-37, so wall is noisy, RSS is the stable column.
| runner | scenario | wall | peak tree RSS | processes |
|---|---|---|---|---|
| bun test (17 files, each one process) | plain | 0.3-19 s | 84-496 MB (p50 150, p90 ~480) | 2-7 |
| bun test dashboard dispatch | plain / `--smol` | 8.1 / 7.0 s | 347 / 314 MB | 3 |
| vitest course suite (697 tests, direct) | before: no bound (12 workers) | 102.8 s | 1619 MB | 11 |
| | before: ts-g constant 4 | 104.1 s | 1325 MB | 10 |
| | w1 / w2 / w8 (m4) | 151.7 / 70.7 / 93.2 s | 1267 / 1548 / 1596 MB | 5 / 6 / 11 |
| | after: policy, no grant (host share, 4 workers) | 59.0 s | 1460 MB | 8 |
| | after: policy, 1 job of 2048 MB (4 workers) | 56.9 s | 1274 MB | 8 |
| | after: policy, 1 job of 1300 MB (1 worker) | 126.1 s | 1012 MB | 5 |
| chromium (headless, one page each) | 0 / 1 / 2 / 3 browsers | | 19 / 415 / 708 / 987 MB | 1 / 5 / 9 / 13 |

Reading: the vitest peak is dominated by the heaviest test (deploy site build + tsc, ~1.2 GB), the marginal worker costs 45-100 MB (declared 100 MB, base 1200 MB); a chromium + page costs ~300-400 MB (declared Playwright worker 600 MB incl. its node worker, stack 1500 MB incl. proctor + site server).

## Files changed
Process module (`🧰️framework/🔨️modules/🏃️process/`): NEW `🧪️testing/👷️workers/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json,📐️learning/🟦️.ts,🧪️tests/{🟦️.ts,🧫️fixtures/🔣️.json,🐍️oracle/🐍️.py,📐️learning/🟦️.ts,🧱️capped-run/🟦️.ts}}`; NEW facade `📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts`; `📦️packages/🟦️typescript/package.json` (export `./testing/workers`); `🧪️testing/🗂️suites/🏃️run/🟦️.ts`; `🧪️testing/🦀️cargo/🟦️.ts`; `🧭️routing/🎛️command/⚙️configuration/{🟦️.ts,🧬️schema/🔣️.json}`; `../../⚙️configuration/🏃️process/🔣️.json` (`framework/⚙️configuration`).
Library (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`): NEW `🏗️build/🌡️host-capacity/🧪️workers/🟦️.ts`, NEW `🏗️build/🌡️host-capacity/🧪️tests/🧪️workers/🟦️.ts`, NEW facade `📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts` + `📦️packages/🟦️typescript/package.json`; `🏃️process/📋️context/🔣️policy/{🟦️.ts,🧪️tests/🟦️.ts}`; `🧪️testing/🧪️vitest/🟦️.ts`; `🏃️process/🦀️cargo/🔣️policy/🟦️.ts`; `🏃️process/🦀️cargo/🎟️admission/🟦️.ts`; `🏗️build/🗂️suites/📜️script.ts`; `⚡️caching/📜️script.ts`.
Dashboard: `🎛️dashboard/🛤️task-routes/🟦️.ts`. Course: `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts`. Docs: `🛍️products/🦑️repo/📓️build-guide.md`.

## Open / next
1. OFFICIAL before/after of the dashboard `test-unit` (cargo/nextest: before = `SEMIO_CARGO_TEST_THREADS=10` equals the old quick-level default, after = derived) and of `course:test` through `bun repo run` when WAITING < 4; the Cargo per-thread size (192 MB declared) is a guess until that run teaches the store (`repo capacity status --json` shows the `test-worker-cargo-test` classes).
2. A run with a single test task still holds `jobs` job shares but may count on one only (siblings are unknown to a child). An orchestrator that sets how many tasks run beside this one (or hands a single-task run its whole grant) would let `bun-test`/`cargo-test` use the idle shares; nothing sets it today.
3. Windows: the tree observer returns nothing there (ram-g's port is Linux/macOS), so nothing is learned on Windows; the declared sizes apply.
4. Other direct runners not routed through the policy yet: `startToolV1("vitest")` in `✅️verify`, the ui scene/surface `bun test` scripts, Go/pytest/dotnet runners (the policy has room for new runner ids; add a row + schema enum entry).
