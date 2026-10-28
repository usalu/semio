# 📊 RAM census 1026g: repo processes over 2 h on a 32 GB dev Mac (target 16 GB)

Auditor `aud-ram-g`, read-only. Window 07:24 to 09:17 CET, 2026-10-26, 24 samples at ~5 min (07:24:20 to 09:17:20). Machine: 32 GB RAM (the brief's 16 GB target is the design budget), 10 cores, swap total 5 GB at start (dynamic, grew in one sample).

## Method
- Tool: `aud-ram-g-tools/tick.py` (stdlib, `/usr/bin/python3 -I`, read-only: `ps`, `vm_stat`, `sysctl vm.swapusage`, `memory_pressure -Q`). Each tick takes two `ps` snapshots 5 s apart; CPU % = delta of cumulative CPU time over the 5 s window (so "idle" means under 1 % CPU during that window).
- Selection: any process whose command contains the repo path or `/.cache/b/`, or whose binary is bun, bazel, cargo, rustc, rust-analyzer, java, vite, nx, playwright, proctor, esbuild, tsc, or a lane shell (`sh`/`bash` with `-tools/` in the path). Claude's own shells and the sampler are excluded.
- Roles from the command line: `bun repo`/`bun exec` = wrapper; `bun --eval` = loader; `bun <entry>` = script; bun with `graph` = graph child; `nx`/`/nx/` = nx; `bazel(...)` = Bazel server, `bazel` = client; `java` = Bazel JVM; any process under a Bazel server = action; `playwright` node and `ms-playwright` chromium = test-runner and browser; `proctor` = test server; `rg/grep/bfs/du/find/tee` = scan tool.
- Lane: the nearest ancestor shell whose path contains `<prefix>-tools/`, mapped to `result-<prefix>.md` (the `-g` result files now exist for most lanes). Processes under the Bazel server are attributed to the lane of the Bazel client alive in the same sample, else `bazel-shared`. Run = lane plus its topmost repo ancestor.
- Raw per-sample JSON (`🗑️generated/aud-ram-g/samples.jsonl`) is deleted with its folder at the end, per AGENTS.md; the tables below are the retained result. Scripts: `aud-ram-g-tools/tick.py`, `summarize.py`, `extra.py`, `run-rest.sh`.

Caveat: RSS per process double-counts pages shared between bun processes (the bun binary and JIT pages are shared), so sums overstate the true footprint. Use the sums for ranking and budgeting, not as exact totals. Idle bun script or loader processes show a median RSS of 53 MB each, which is the practical base cost of one extra bun process.

## Machine state over the window
- Repo RSS (sum): median 12.3 GB, peak 18.7 GB (tick 13, 08:28). Six samples were above 16 GB; twelve were above 12 GB.
- Repo processes per sample: median 51, max 94. Concurrent runs: median 15, min 6, max 24.
- Idle share (under 1 % CPU) of repo RSS: 20 % pooled; median idle RSS 2.4 GB, max 5.8 GB.
- `vm_stat`: compressor occupied 3.5 to 16.8 GB (median about 6.9 GB); swap used median 5.3 GB, min 2.7 GB, peak 36.7 GB in tick 1 (the swap total grew); memory_pressure free 35 to 77 %.
- Lanes seen: e2e, e2e-f, nx2-f, ver-f, orphan (ppid 1 daemons), bazel-shared, and wave-g lanes asm, bzsrv, chain, churn, cutoff, disk, gram, hold, integ, keys, lease, noop, nxperf, ram, route, rust, ts.

## Per role (over 24 samples; per-sample role totals)
| role | samples seen | avg count | peak count | avg total RSS GB | peak total RSS GB | idle share |
|---|---|---|---|---|---|---|
| loader (bun --eval) | 24/24 | 8.3 | 12 | 5.53 | 11.67 | 21 % |
| script (bun entry) | 24/24 | 13.5 | 27 | 2.33 | 7.08 | 41 % |
| bun-other | 20/24 | 3.5 | 18 | 1.79 | 8.07 | 12 % |
| nx (daemon, graph) | 18/24 | 1.9 | 5 | 0.62 | 4.01 | 20 % |
| action (under Bazel) | 11/24 | 3.6 | 17 | 0.57 | 3.39 | 52 % |
| vite | 12/24 | 1.7 | 10 | 0.42 | 4.57 | 38 % |
| node (non-test) | 6/24 | 0.4 | 2 | 0.36 | 2.11 | 0 % |
| bazel-server | 19/24 | 1.1 | 2 | 0.23 | 1.08 | 80 % |
| browser (chromium) | 5/24 | 0.8 | 4 | 0.20 | 1.14 | 7 % |
| cargo-rustc | 6/24 | 0.4 | 2 | 0.13 | 2.56 | 7 % |
| test-runner (playwright) | 5/24 | 0.4 | 2 | 0.10 | 0.63 | 24 % |
| graph-child | 13/24 | 0.7 | 3 | 0.07 | 0.38 | 73 % |
| other (tee, misc) | 21/24 | 4.4 | 12 | 0.05 | 0.19 | 68 % |
| wrapper (bun repo) | 24/24 | 7.8 | 13 | 0.02 | 0.04 | 100 % |
| lane-script (sh) | 24/24 | 7.8 | 11 | 0.01 | 0.01 | 99 % |
| bazel-client | 12/24 | 0.5 | 1 | 0.01 | 0.01 | 100 % |
| scan-tool (rg, grep, du) | 22/24 | 1.2 | 2 | 0.00 | 0.01 | 52 % |

Per-process view (process-samples): loader median 0.13 GB, peak 9.0 GB; 35 of 200 loader samples above 1 GB, 11 above 3 GB; 11 loader samples were idle and older than 30 min with more than 100 MB (stale, 2.8 GB summed). Script median 0.05 GB, peak 4.6 GB. Action median 0.07 GB, peak 3.1 GB, 46 % idle. Test workers `bun test` for the dashboard (keys-g lane) run at about 2.0 to 2.1 GB each.

## Per run (lane and root; tree = root plus repo descendants)
| run (lane, root) | samples | tree size median (peak) | RSS median GB (peak) | note |
|---|---|---|---|---|
| e2e, sh lane script | 16 | 8 (10) | 3.10 (9.17) | architecture course e2e, Bazel |
| e2e, Bazel server | 4 | 15 (15) | 2.48 (5.10) | server CPU 193 % median |
| nx2-f, sh lane script | 14 | 16 (16) | 1.22 (4.74) | dashboard Rust test chain |
| noop-g, sh lane script | 4 | 5 (7) | 0.24 (4.81) | |
| cutoff-g, bun --preload | 17 lane samples | 2 | 1.42 lane median (6.67 peak) | single runs up to 4.8 GB |
| keys-g, Xcode python root | 12 lane samples | 2 to 12 | 0.24 lane median (4.60 peak) | two `bun test` workers at 2 GB each |
| rust-g, sh lane script | 2 | 6 (7) | 3.46 (4.19) | cargo/rustc inside |
| gram-g, sh lane script | 8 runs | 2 | 2 to 4.5 each | graph generation |
| orphan, nx daemon 64500 | 10 | 1 | 0.85 (0.89 at 782 s age) | nx daemon, ppid 1, 2 % CPU |
| unattributed Bazel server 83675 | 1 | 6 | 3.84 | 152 % CPU |

Per lane (median RSS, peak): e2e 3.14 GB / 9.17; cutoff-g 1.42 / 6.67; orphan 0.70 / 5.19; noop-g 0.78 / 4.81; nx2-f 1.05 / 4.74; keys-g 0.24 / 4.60; gram-g 0.45 / 4.55; rust-g 0.77 / 4.19; ts-g 0.86 / 1.66; nxperf-g 0.28 / 2.35; bazel-shared 0.17 / 0.43.

## Which roles a 16 GB machine cannot afford
1. Concurrent run trees. The median is 15 runs per sample and 51 repo processes. Two largest runs alone median 7.1 GB out of 12.3 GB, and the single largest run median is 3.8 GB. At 16 GB, more than about 2 full run trees (2 to 4 GB each, 5 to 16 processes) cannot coexist with the OS and the browser.
2. Bun loader and entry processes, which are the biggest RSS consumers: loader and script, together about 8 GB of the median 12.3 GB, with spikes of 9 to 12 GB per role. Each run carries 5 to 7 bun processes at about 50 MB base cost each, and the eval heaps spike.
3. Test workers: `bun test` dashboard workers at about 2 GB each, and chromium at up to 1.1 GB with four instances at peak. Two concurrent dashboard test runs are already 4 GB.
4. Nx daemon and graph child: nx median 0.35 GB, peak 4.0 GB, and an orphaned daemon idle for hours.
5. Bazel JVM: peaks at 1.1 GB with 80 % idle in the sampled windows. Affordable as one shared server only with a capped heap.

Affordable at any count: lane shells, `bun repo` wrappers, the Bazel client, and scan tools (each under 0.01 GB).

## Top 5 savings (estimates from these samples)
1. Cap concurrent active runs to 2 in the capacity governor. Median repo RSS falls from 12.3 GB to about 7.1 GB (saves about 5 GB median), and the peak falls from 18.7 GB to about 9 to 10 GB. This is the largest saving.
2. Reap idle loader and script processes once their run ends or after a grace period of about 5 minutes. Idle RSS is 2.4 GB median (5.8 GB peak). Stale idle loaders older than 30 min held 2.8 GB summed across the window.
3. Merge the loader and entry script into one bun process per run, or reuse one bun across the wrapper, script and graph child. This removes about 8 bun base costs per sample (8.3 loaders times about 53 MB, roughly 0.45 GB median) and the eval heap spikes up to 9 GB.
4. Run one Nx daemon per workspace and cap its heap. Nx averages 0.62 GB (peak 4.0 GB); the orphaned daemon held 0.9 GB at 2 % CPU. Saves about 0.6 GB average and up to 3 GB at peak.
5. Serialize dashboard `bun test` workers to one at a time (about 2 GB each, two at peak 4.2 GB) and cap the Bazel JVM heap to 1 GB (currently `-Xmx2048m`, peak 1.1 GB, 80 % idle). Saves about 2 GB peak per extra test worker plus 0.5 to 1 GB on the server.

## Limits
- One 2-hour window on a busy machine shared by about 20 agents. Not a controlled benchmark; repeat under a fixed scenario before setting budgets.
- Role and lane attribution are heuristic (command line and ancestry). Bazel actions are attributed to the Bazel client alive in the same sample, which can misattribute when several clients run.
- The 5-second CPU window can miss short bursts; the sampling grid is 5 minutes.
