# Server Load Audit (read-only), 2026-11-17 06:33–06:34

Host: g1-server, 10 cores (`hw.ncpu`=10), 32 GB. Uptime 3 days 12:52, load 29–40 (1/5/15 min: 31.1 / 35.4 / 33.9 at 06:34:21).
Memory: 87% free (`memory_pressure`). No cargo/rustc process was running during the audit.

## Method

- 6 `ps -Ao pid,ppid,pcpu,nice,rss,etime,command` samples, 06:33:26 to 06:34:21 (about 11 s apart, not 20 s; a 5 s sleep cap per call made 20 s impossible).
- Caveat: `ps` `%CPU` on macOS is a lifetime average (cputime / elapsed). It overstates short-lived processes (git) and smooths bursts. Therefore a second measurement was taken: a 5.1 s cputime delta (`ps -o time`, instantaneous). Both are reported below.
- Totals of all processes: lifetime sum about 830% (8.3 cores busy); instantaneous 5 s window 609% (6.1 cores).

## CPU by group (average over 6 lifetime samples; instantaneous 5 s window in brackets)

| Group | Avg %CPU (lifetime) | Window %CPU | Procs (typ.) |
|---|---|---|---|
| git | 467 (inflated by short-lived procs) | 138 | 4–6 |
| bun farm/other (walk-farm children, replica supervisor, sidecar) | 99 | 275 (bun-farm 183 + bun-other 93) | 16–20 |
| bun pool (`owner-pool-waves`, nice 10) | 103 | 35 (1 child seen; 4 children at ~80% in sample 6) | 1–5 |
| bun graph (`show projects`, `--eval`, graph-farm-worker) | 31 | 47 | 15 |
| bun test (`bun test`) | 6 | 3 | 1–3 |
| nfsd | 14 | 12 | 1 |
| mediaanalysisd | 9 | 26 | 1 |
| Spotlight (mds, mdworker) | 18 | 2 | 16–18 |
| WindowServer | 26 | 28 | 1 |
| Claude app helpers | 26 | 13 | 13 |
| RustDesk | 0.2 | 0.8 | 2 |
| cargo / rustc | 0 | 0 | 0 |
| other >5% (TextEdit, FSEvents, Finder, Activity Monitor) | 33 (all other) | 30 (all other) | 470+ |

Processes with nice > 0: 26–28 per sample. Lifetime sum 178% average. Instantaneous window: 24 procs, 182% (about 1.8 cores). This does not match the premise of 3–6% of a core; individual pool children measured 35–85%. The 3–6% figure did not reproduce in this audit.

## Git spawns

- Distinct git processes seen across the 6 samples plus the 5 s window: about 22 in about 75 s. This is a lower bound (missed short-lived spawns). Rate: at least about 15 per minute.
- Sub-commands: `status` (most), `ls-files`, `diff`, `write-tree`.
- Parent chains (resolved with `ps -o pid,ppid,command -p`):
  - Most git (status, diff, ls-files): parent 69054 "Claude Helper (utility, node.mojom.NodeService)" then 69013 `/Applications/Claude.app/Contents/MacOS/Claude` then launchd. This is the Claude desktop app's Node service polling the repo.
  - `ls-files`, `write-tree`: parent 8967 (bun replica supervisor `shared-worktree/replica/supervisor.ts --boot-child run`), then 8961, then 8937 `fleet/entry/boot.sh run`, then launchd. This is the fleet replica daemon.
  - A separate Claude Code CLI session (pid 69123 under Claude.app, which runs a zsh shell) also runs bun work: pid 6767 at 76–90% CPU, nice 0, grandparent Claude Code.

## Other findings

- Orphaned walk-farm children at nice 10 (pids 55454, 56506, 61614, ppid 1): about 45% each in the window (about 137% combined).
- Pool children (`owner-pool-waves`, nice 10): up to 4 at 79–85% in sample 6.
- nfsd (pid 717): 12% in the window, one process; the repo replica is served over NFS.
- FSEvents (pid 551): up to 15% in one sample. Consistent with heavy file churn in the repo.
- mediaanalysisd (pid 20254): 26% in the window, a background photo or media analysis.

## Spotlight

- `mdutil -s /`: Indexing enabled. The repo lives under `/Users/ueli/Documents`, so it is covered by this index.
- `mdutil -s /Users/Shared`: "unknown indexing state" (normal for a firmlinked path; not an error for the repo).
- No `.metadata_never_index` file in the repo root.

## Recommendations (not executed)

1. Claude desktop git polling (pid 69054): close or detach the semio project in the Claude app, or quit idle Claude Code sessions. This is the largest git source.
2. Fleet replica daemon (pid 8967): reduce the poll interval for `ls-files` / `write-tree`, or cache the index.
3. Kill or reap the orphaned walk-farm children (pids 55454, 56506, 61614). The user decides; this audit did not signal them.
4. Cap pool concurrency (`owner-pool-waves`) to 4 or fewer, or run pool and farm at nice 10 with a concurrency limit.
5. Spotlight: add `.metadata_never_index` (or exclude `.cache` and `.🧬semio`) if indexing the repo is unwanted.
6. mediaanalysisd: pause Photos or Visual Look Up analysis in System Settings if it is not needed.
7. TextEdit and Activity Monitor: close; trivial (about 9%).
