# 📊 RAM census 2 (1026g): repo processes, 12:56 to 14:50, against the morning census

Auditor `aud-ram2-g`, read-only. Window 12:56:32 to 14:49:50 CET, 2026-10-26, 24 samples at ~290 s spacing (plus one 5 s CPU window per sample). Machine: 32 GB RAM, 10 cores, load average about 47 at the start. The morning census is `audit-ram-census-1026g.md` (07:24 to 09:17, 24 samples). Its raw samples were deleted with its folder, so the comparison uses its published tables.

## Method
- Same tool as the morning census: `aud-ram-g-tools/tick.py` (stdlib, `/usr/bin/python3 -I`, read-only `ps`, `vm_stat`, `sysctl vm.swapusage`). Same role and lane heuristics, same summary scripts (`summarize.py`, `extra.py`). The driver loop is `aud-ram2-g-tools/run.sh` (sleeps only, starts no builds, kills nothing).
- My own driver shell appears as lane `aud-ram2-g` (role lane-script, negligible RSS).
- Caveat: RSS per process double-counts pages shared between bun processes, so the sums overstate the footprint. Use them for ranking and budgeting only.
- Roles are inferred from the command line. `bun test` is classified as `bun-other` (its argument is `test`), so `bun-other` is in practice the test workers. Graph builders are not a separate role: the morning `graph-child` role and the `nx` role (which holds graph builders and the nx daemon) are compared as published.

## Machine state over the window
- Repo RSS (sum): median 11.8 GB (morning 12.3), peak 16.6 GB at 13:55:39 (morning 18.7 GB). Samples above 12 GB: 12 of 24 (morning 12). Samples above 16 GB: 1 (morning 6).
- Repo processes per sample: median 47.5 (morning 51), max 87 (morning 94).
- Concurrent runs (lane and root groups): median 15 (morning 15), min 7 (morning 6), max 22 (morning 24). The morning census recommended capping active runs at 2. That cap is not visible.
- Idle share (under 1 % CPU): 31 % pooled (morning 20 %). Median idle RSS 3.8 GB (morning 2.4 GB), max 8.2 GB (morning 5.8 GB).
- Compressor: median 3.0 GB, min 1.3 GB, max 12.3 GB (morning median 6.9 GB, range 3.5 to 16.8 GB). The 12.3 GB spikes came at 14:25 and 14:44 with memory free at 49 %.
- Swap used: median 4.0 GB, min 2.5 GB, peak 9.5 GB at 14:49 (morning median 5.3 GB, peak 36.7 GB in tick 1 while the swap file was growing).
- Memory free: 49 to 84 % (morning 35 to 77 %).

## Per role, morning against now
Avg/peak count, avg/peak total RSS GB, idle share. Morning values from its table.

| role | morning | now |
|---|---|---|
| loader (`bun --eval`) | 8.3 / 12, 5.53 / 11.67 GB, 21 % | 2.7 / 4, 2.70 / 8.80 GB, 39 % |
| loader, per process | median 0.13 GB, peak 9.0 GB; 17.5 % of samples above 1 GB | median 0.54 GB, peak 6.6 GB; 25 % above 1 GB (16 of 65) |
| bazel-server (JVM) | 1.1 / 2, 0.23 / 1.08 GB, 80 % | 1.5 / 3, 0.31 / 0.99 GB, 73 %; largest single server 0.75 GB |
| nx role (daemon and graph builders) | 1.9 / 5, 0.62 / 4.01 GB, 20 % | 2.3 / 6, 2.64 / 6.56 GB, 34 % |
| graph-child role | 0.7 / 3, 0.07 / 0.38 GB, 73 % | 0.4 / 2, 0.06 / 0.95 GB, 63 % |
| bun test workers (in `bun-other`) | dashboard `bun test` about 2.0 to 2.1 GB each, two at 4.2 GB | 1.5 / 4, 2.12 / 5.11 GB, 1 %; one gram2-g worker 1.7 to 5.1 GB for 40 min |
| test-runner (playwright) | 0.4 / 2, 0.10 / 0.63 GB, 24 % | 0.5 / 2, 0.11 / 0.56 GB, 22 % |
| browser (chromium) | 0.8 / 4, 0.20 / 1.14 GB, 7 % | 1.0 / 4, 0.24 / 1.23 GB, 5 % |
| script (bun entry) | 13.5 / 27, 2.33 / 7.08 GB, 41 % | 13.0 / 21, 2.26 / 5.21 GB, 62 % |
| action (under Bazel) | 3.6 / 17, 0.57 / 3.39 GB, 52 % | 4.2 / 23, 0.57 / 3.33 GB, 63 % |
| vite | 1.7 / 10, 0.42 / 4.57 GB, 38 % | 1.1 / 5, 0.21 / 1.03 GB, 0 % |
| cargo-rustc | 0.4 / 2, 0.13 / 2.56 GB, 7 % | 0.2 / 2, 0.06 / 0.66 GB, 1 % |
| wrapper, lane shells, Bazel client, scan tools | each under 0.05 GB | unchanged, under 0.05 GB |

Per run (median and peak RSS GB; morning in brackets where the same lane exists):
- nx2-f: median 4.4, peak 6.8 (morning 1.05 / 4.74 per lane).
- gram2-g (graph generation and its test lane): median 2.5, peak 5.1.
- lane2-g (new): median 1.3, peak 7.1, one `bun` loader and nx daemon of 1.9 and 2.3 GB at the 13:55 peak.
- derive-g (new): median 3.5, peak 4.0, `bun` processes at 2.6 to 3.9 GB each.
- e2e: median 2.6, peak 3.1 (morning median 3.14, peak 9.17). Its peak fell.
- gate-g: peak 3.9, median 0.1.
- orphan (ppid 1 daemons and detached processes): median 0.33, peak 5.6. Five idle loader samples older than 30 min (2.4 GB summed) and two idle nx samples (1.5 GB summed) are stale.

## Answer to the 16 GB targets
- None of the brief's 16 GB targets holds at the field level in this window.
- Median repo RSS of 11.8 GB is under 16 GB, but there is no headroom: the peak of 16.6 GB at 13:55 is above it, and only the morning peak (18.7 GB) was higher.
- Three-agents-on-16-GB without a swap storm does not hold: concurrent runs are unchanged at a median of 15, swap reaches 9.5 GB, and the compressor reaches 12.3 GB twice.
- Loaders (after chain-g): fewer and smaller in total (2.7 GB average, down from 5.5 GB), but the median loader process is larger (0.54 GB, up from 0.13 GB). The 1.2 GB drop per loader that chain-g reports is not visible in the field medians. The surviving loaders may be longer-lived, and 65 process-samples is a small base.
- Bazel servers (after bzsrv-g): two to three servers now run instead of one or two. Total RSS is flat (0.31 GB average, 0.99 GB peak) and each server stays under 0.8 GB. The change adds processes without adding memory.
- Graph builders (after gram-g): not met. gram-g's own target is a graph child below 2 GB. Field nx-role RSS rose to 2.6 GB average (morning 0.6 GB), and one gram2-g graph-related process reached 5.1 GB.
- Test workers (after testmem-g): not met in the field. Single `bun test` workers reach 5.1 GB (morning 2.0 GB), and testmem-g's worker policy is not visible for `bun test` under gram2-g. Chromium and playwright are unchanged, at a median of 0.1 to 0.2 GB.

## Morning recommendations, status
1. Cap concurrent active runs to 2: not in effect (median 15 runs).
2. Reap idle loader and script processes after a grace period: idle RSS rose to 3.8 GB median; stale loaders still hold 2.4 GB summed.
3. Merge loader and entry script into one bun process per run: not visible in the counts (loader count fell, script count is unchanged at 13).
4. One Nx daemon per workspace with a capped heap: nx RSS rose, not fell.
5. Serialize dashboard `bun test` workers and cap the Bazel JVM heap: the JVM peak fell to 0.75 GB, so that part is met. The `bun test` workers are heavier than before.

## Limits
- One 2-hour window on a shared machine with load about 47. Not a controlled benchmark.
- Role and lane attribution is heuristic. Bazel actions are attributed to the Bazel client alive in the same sample, and `bun test` sits in `bun-other`.
- The 5 s CPU window can miss short bursts, and the sampling grid is about 5 minutes.
- Raw per-sample JSON is deleted with its generated folder at the end, as the morning census did. The tables above are the retained result. Tools kept: `aud-ram-g-tools/` (unchanged) and `aud-ram2-g-tools/run.sh`.
