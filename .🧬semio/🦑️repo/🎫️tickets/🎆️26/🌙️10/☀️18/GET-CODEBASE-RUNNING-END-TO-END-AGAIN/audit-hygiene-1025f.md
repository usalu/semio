# 🧹 Hygiene Audit 1025f (hyg-f, read-only)

Snapshot 2026-10-25 19:10-19:17 CET (Haiku audit, read-only: no kills, no edits, no `bun repo`, no modifying git). Brief: `brief-wave-1025f.md`. Baseline: `audit-hygiene-1024d.md`.

## Summary

- **Product `[DEBUG]` is down to one genuine runtime item:** the vendored nx patch, 6 lines (7 occurrences), all env-gated (`NX_VERBOSE_LOGGING`, `SEMIO_BUILD_PROFILE_INPUTS`) but still tagged. Owner nx-d / nx3-d (deferred by dbg-d, see 1a).
- **Course, pets, `🖱️ui`, framework packages, print, presentation, bericht, projektetage, `🏃️process`, and the repo Rust crates: 0 product `[DEBUG]` lines.** The 1024d baseline had 13 course, 3 print and 76 dashboard lines; all gone. The remaining course hits are a deploy checker string and a test fixture (both intentional).
- **Processes: no leftover from a dead lane.** Nothing dev-related is older than 19:09:32. The 8 processes started 19:09:32-19:09:46 are wave-1025f lanes (print-f, ext-f, nx-f, presentation test), still running. Memory is fine (61 % free); the heaviest bun child is 2.5 GB RSS.
- **Disk: `/` 62 GiB free (was 86 GiB at 1024d).** `.cache/b` holds about 152 GiB (`d` 68 G, `p` 56 G, `t` 15 G). **`C/🗑️generated` is 53.4 GiB**, with 300 MB+ `deps4-sync*.log` files; it must be deleted at close.
- **Stray files:** 23 untracked entries, 19 of them this ticket's own lane outputs and 4 live peer source dirs. Root has 15 bun lockfile backups (`.0e32256` … `.ed864b2`, 9 MiB), `temp/` (62 python scripts, git-ignored), an empty untracked `undefined/`, and `debug-storybook.log`. The index holds 6,698 staged A/D/R entries from someone else's in-flight staging; I did not touch it.

## 1. `[DEBUG]` in tracked sources of the three closures

Method: `git grep -n -F '[DEBUG]' -- . ':!.🧬semio'` (tracked files, tickets excluded), then per-closure counts.

### 1a. Genuine product `[DEBUG]` (runtime text, shipped)

| file:line | what | owner / lane |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch:173` | `[DEBUG] Owned Nx daemon exited`, `NX_VERBOSE_LOGGING` gated | nx-c / nx-d (added); deferred by dbg-d (`result-dbg-d.md` "Live") |
| same, `:1047`, `:1048` | `[DEBUG] Original selected graph publication…`, `[DEBUG] Selected graph publisher sealed=` | nx-d |
| same, `:1545` (two occurrences) | `[DEBUG] filesystem-journal worker/query refused` | nx-d |
| same, `:1932` | `[DEBUG] Nx configuration globs`, `SEMIO_BUILD_PROFILE_INPUTS=1` gated | nx-d |
| same, `:2024` | `[DEBUG] Immutable graph authorizer refused:` | nx-d |

Delta vs 1024d: 7 hunks there (173, 1047, 1048, 1097, 1552, 1939, 2031); now 6 lines (1097 gone). Line numbers moved, so the lane must re-check by text, not by line. dbg-d refused the rewrite because editing the patch re-keys the Nx toolkit identity and invalidates graph caches while nx3-d runs were in flight. The rename `[DEBUG] ` to `[nx-semio] ` needs a cache-aware run.

### 1b. Tag appears intentionally (not a product log)

| file:line | why it is not a debug log |
|---|---|
| `…/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:320` (matches `[DEBUG] Fixed UDRW image `) | parser for recorded producer logs; the tag is the protocol key, pinned by `producerLogSha256`. The emitter is not in tracked sources. Rewriting it needs a re-pin. |
| `…/📦️graph/🧪️tests/🔣️.json` (39 lines, `Fixed UDRW image` rows) | recorded producer-log fixture |
| `…/📦️graph/🧪️tests/🎛️operation/🟦️.ts:58, :61` | test harness prints `[DEBUG] graph-root` protocol lines to the parent process (same class as the producer-key rename in dbg-d) |
| `…/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts:39` | assertion `not.toContain("[DEBUG]")` |
| `…/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:152-163` | the census law itself |
| `…/🧪️test/🎯️acceptance/🧪️tests/🧫️fixtures/🧮️source-census/🔣️.json:29` | fixture |
| `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts:101` | deploy checker list (fails the build on leftover `[DEBUG]`) |
| `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts:51` | fixture input |

### 1c. Closures with 0 hits

- Course: `🎓️teaching` 2 hits, both 1b. `📚️course`, `🐾️pets` and `🖱️ui`: 0. Framework packages `📦️packages/🟦️typescript/🏃️process` and `🖱️ui`: 0.
- Print `📓️print`, presentation `🎤️presentation`, bericht `♻️mit-bestand/📋️bericht`, projektetage `♻️mit-bestand/🎤️präsentation/📅️33.projektetage`: 0.
- `🏃️process` modules: 0.
- Repo Rust crates (`*.rs` under the repo product and framework): 0 product hits. The `.rs` hits in the tree are in plugins (`✏️s`, out of scope) and `🌎️hub`.
- Repo product total: 56 lines in 7 files, all 1a or 1b.

## 2. Leftover processes

`ps -axo pid,ppid,lstart,pcpu,rss,etime,command` (dev tools: bun, bazel, cargo, rustc, nx, playwright, chromium, vite, esbuild, python3, detach). 42 processes match, all started 19:08 or later (`Sun Oct 25 19:08-19:14`). Nothing from 10-24 or earlier. **No dead-lane leftovers older than 19:10.**

Started before 19:10 (all 1025f lanes launched by the wave start at 19:08-19:09, still running; the brief expects them to be live, not leftovers):

| pid | ppid | started | cpu | rss | command (short) | lane |
|---|---|---|---|---|---|---|
| 12059 | 12026 | 19:09:32 | 0.0 | 4.9 MB | `bun repo --tool=bazel run-many --projects=@semio-tech/print-rs --targets=test` | print-f |
| 12060 | 12059 | 19:09:32 | 0.7 | 63.7 MB | bazel-run bun child (print-rs) | print-f |
| 12112 | 12060 | 19:09:36 | 0.9 | 2.5 GB | `bun --eval` helper under print-rs test | print-f |
| 12294 | 1 | 19:09:45 | 0.0 | 1.2 MB | `sh -c nice -n 10 bun …/ext-d-tools/truth.ts` | ext-f |
| 12298 | 12296 | 19:09:45 | 51.4 | 200 MB | `bun …/ext-d-tools/truth.ts` | ext-f |
| 12311 | 1 | 19:09:46 | 0.0 | 1.2 MB | `sh …/nx-f-tools/chain…` | nx-f |
| 12318/12319 | 1 | 19:09:46 | 0.0 | 4.9 MB | `bun repo run @semio-tech/presentation:test` | print-f |
| 12320 | 12319 | 19:09:46 | 0.1 | 67 MB | bazel-run bun child (presentation) | print-f |

Heaviest others at 19:15: bun `--eval` helpers 24835 (1.9 GB RSS, started 19:12:04), 29663 (1.6 GB, 19:13:14), 14475 (250 MB); bazel server 14492 (362 MB, 19:11:18). Concurrency is about 9 build lanes, within the brief's ≤ 10.

Memory: `vm.swapusage` 780 MB used of 2048 MB; memory free 61 %.

## 3. Disk and caches

- `df -h /` (system volume) reports 62 GiB available (18 % used). The Data volume `/System/Volumes/Data`, where the repo lives, is 825 GiB used of 926 GiB (94 %), 62 GiB available, 4 % inodes. Threshold is 30 GiB, so launches are allowed.
- `.cache/b/*` (top-level, `du -sk`): `d` 68 G, `p` 56 G, `t` 15 G, `g` 7.9 G, `o` 3.0 G, `n` 2.3 G, `l` 68 M, `c` 4.1 M. Total ≈ 152 GiB. `.cache/b/d` and `.cache/b/p` are the candidates for the next cleanup.
- `C/🗑️generated`: **56,009,648 KiB = 53.4 GiB.** Largest direct files: `deps4-sync60.log` 308 MB, `deps4-sync50.log` 301 MB, `deps4-sync31.log` 167 MB, `deps4-sync23.log` 119 MB, `deps4-sync14.log` 116 MB, `deps4-sync16.log` 90 MB, `deps4-sync15.log` 42 MB, `rmod-editor-{pre,mut-cur,post}-cur-lib-final.err` 32 MB each, `deps4-lock.log` 21 MB. These are tool-generated logs that AGENTS says must be deleted at ticket close. Subdirectory sizes are not walked.
- `~/.sinc`: `du -sk` still running at 19:16 (it is a ~60 GB image; the walk is slow). Result is in the scratchpad `du-sinc.txt` and is filled in at the re-check below.

## 4. Stray files outside ticket folders

`git --no-optional-locks status --porcelain` at 19:12: 13,063 entries (6,339 M, 4,327 D, 2,261 A, 110 R, 3 MM, 1 M, 23 ??). The index has 6,698 staged A/D/R entries: 1,080 under `🧰️framework` and 29 under `✏️s`, all from someone else's in-flight staging. Not touched.

Untracked (`??`, 23):
- 19 are this ticket's own files: `brief-wave-1025f.md`, `result-*-f.md`, `report-e2e-orch-e2e-1025f-*.md`, `*-f-tools/` dirs, and `status-1025f.md`. Keep.
- 4 are live peer source (new, not scratch): `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🔁️observation/` (`🟦️.ts`), its `🧪️tests/🔁️observation/` (`🐍️.py`, `🟦️.ts`), `🧫️fixtures/…` and `🧬️schema/…`. Written 19:12-19:14. Not stray; not yet `git add`ed.

Root (ignored unless noted; none tracked):
- `temp/`: 62 python scratch scripts (`dump-*.py`, `check-four.py`, …), 2.7 MB, Oct 1-25. Git-ignored, but scratch. Delete candidate for its owner.
- 15 bun lockfile backups `.0e32256` … `.ed864b2` (lockfileVersion 1, 9 MiB total, Oct 19-25). Bun temp output; delete candidate.
- `debug-storybook.log` (Oct 23, 1 KB), `.Synth_check.log` (Aug 12, 81 KB), `.tmp` (0 B, Oct 16). Scratch.
- `undefined/`: **empty, untracked, not ignored**, created Oct 20 22:17. git status does not list it because it is empty. Remove it (almost certainly a path-join bug).
- `bazel-out/` (Oct 15, untracked, not ignored, no files: only empty dirs). Harmless.
- `micro_msg.txt` (tracked, Oct 9): a tracked stray; out of scope, flagged only.
- `node_modules` is a symlink into `.cache/b/d/m/j/…`, expected.

## Re-check (2026-10-25 21:12-21:19 CET, ~2 h after pass 1)

Summary of the diff:
- **Product `[DEBUG]` lines in the nx patch are gone.** `nx@23.2.0.patch` has 0 `[DEBUG]` now (was 6). The repo product is 50 lines in 6 files (was 56 in 7): all test fixtures, census laws or protocol keys (1b). Genuine runtime `[DEBUG]` in the three closures: **0**. Someone edited the patch after 19:17, which re-keys the Nx toolkit identity; that is expected for the nx lane but should be known to the graph-cache owners.
- **Teaching: unchanged** (2 lines, both intentional). Print, presentation, bericht, projektetage, pets, `🖱️ui`, `🏃️process`, course: still 0.
- **Swap pressure is the main new risk.** `vm.swapusage` used 12.0 GB of 13.3 GB at 21:14 (free 1.3 GB), up from 0.78 GB of 2 GB at 19:15. System memory free 46 %. Largest RSS: bun `--eval` helper 64323 (2.8 GB, 40 min), bun 14623 (1.9 GB, 12 min), bun 26395 (0.9 GB). Non-semio: GitKraken renderer 1.2 GB, a transient git 1.7 GB.
- **Processes older than 19:10 (still alive, both idle at 0 % CPU for about 2 h):**
  - 12026 (`sh` wrapper, ppid 1, started 19:09:30) → 78842 `bun repo --tool=bazel run-many --projects=@semio-tech/presentation,@semio-tech/presentation-react --targets=test` (1 h 27 min, RSS 4.5 MB). Print lane; presentation test never finished. Possibly stalled on a lock.
  - 12311 (`sh` chain wrapper, ppid 1, started 19:09:46) → 43892 Python 3.9 (1 h 54 min, 19 MB, 0.3 % CPU). nx-f chain.
  - Neither is from the previous fleet, and the lanes are not reported as dead. Not killed. Owners should check whether they are waiting on a mutex (print-f, nx-f).
  - The 19:09 list from pass 1 is otherwise gone: 19:09 pids 12059/12060/12112/12294/12298/12318-12320 have exited.
- **Disk: `/` 48 GiB free (was 62 GiB at 19:12). Data volume 827 GiB used, 95 %.** Above the 30 GiB threshold.
- **Caches (`.cache/b/*`, KiB converted, GiB):** `d` 64.1 (was 68 G), `p` 56.1, `t` 14.7 (was 15), `g` 8.1 (was 7.9), `o` 3.0, `n` 2.2, `l` 0.07, `c` 0.004. Total ≈ 148 GiB.
- **`C/🗑️generated`: 56,309,348 KiB = 53.7 GiB** (was 53.4). Still to be deleted at close.
- **`~/.sinc`: 130,842,216 KiB = 124.8 GiB.** The brief says "~60 GB image". This is more than double. Flag for the owner. Pass 1 did not finish this walk, so growth is not measured.
- **Git index: cleaned.** `git --no-optional-locks status --porcelain` is now about 162 entries (89 staged M, 42 R, 21 staged A, 2 staged D, 6 worktree-only M, 1 MM, 3 ??). The 13,063-entry staged mess from pass 1 has been committed or restaged.
- **Untracked (new):** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧪️tests/` (new peer test dir, live source, not scratch). In the ticket: `graph-f-tools/`, `purity-f-tools/tsconfig.products.json` (lane outputs, keep). The `🔁️observation/` source from pass 1 is now tracked (3 files in git ls-files, 1 + 2 in the two dirs).
- **Root scratch:** `temp/` now 61 entries (was 62), 15 bun lockfile backups `.0e32256` … `.ed864b2` still present, `undefined/` still empty and present.
