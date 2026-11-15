# Result: publish latency of the fleet replica (2026-11-13)

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Contract: `1113-git-state-contract.md` (new section "Change detection", attestation and git configuration rewritten). Operator view: module `README.md` (new section "Finding changes"). Bench scripts: `1113-latency-bench/` (usage at the end). The repo MCP did not connect, so the ticket is not closed.

## Summary

| Case (100 000 files, 52 500 directories, watcher disabled, loaded 10-core Mac) | Before | After |
|---|---|---|
| Agent edit through the hook, or any `touch <path>` | 9.6 s (no tier) | **38 ms** median (11 to 58) |
| New file named by a hint | 9.8 s | **38 ms** (29 to 54) |
| In-place edit of a path the daemon published, applied or was hinted about | 9.6 s | **141 to 171 ms** median (39 to 442) |
| Delete of such a path | 14.2 s | **357 ms** (132 to 498) |
| Create of a file in a cold directory, no hint | 9.8 s | **2.0 s** median (0.3 to 3.2) |
| Atomic-rename save over a cold file, no hint | 13.4 s | **0.96 s** median (0.3 to 2.1) |
| In-place edit of a cold file, no hint, no directory change | 13.8 s (scan 15 s) | **69 to 103 s** (scan 90 s, jittered 67 to 112 s) |
| Idle daemon CPU, continuous scans as in the task (`--scan 15`) | 62 % of a core | 12 % |

The last two rows are the price of the design: a cold in-place edit with no hint is found by the full file sweep only. It waits longer than with a 15 s scan, but 10 times shorter than the 900 s that `🗺️topology/🔣️.json` actually carried (see "Facts that differed"). Every machine that wants that case fast calls `touch` (or runs the hook).

## What was built

- **Hint channel** (`🎣️tiers`, new member): `.cache/f/hints/` (sealed, local, mode 1777). A hint is a file of NUL- or newline-separated absolute or worktree-relative paths, created under a dot name and renamed inside the directory. The daemon polls every 50 ms (`readdir` of the empty directory: 56 µs wall, 61 µs CPU per poll, 0.12 % of a core), reads finished files (at most 1 MiB, 5000 paths per poll), removes them, drops paths outside the worktree or below `.cache/f` (absolute paths are also matched through real paths), and publishes the paths in the next loop round without the debounce. Unfinished dot files older than a minute are removed. Writing one hint costs 0.37 ms, consuming one 0.17 ms.
- **Commands** (`🔁️replica/⌨️command`, texts en and de): `touch <path>…`, `touch --stdin`, `touch --hook` (a Claude Code `PostToolUse` payload on stdin: `tool_input.file_path` and `tool_input.notebook_path`, everything else ignored, exit 0 whatever it reads or finds, also without `machine.json`). `touch` itself exits 1 when no path is left to hand over. The process costs 34 ms wall (`bun script.ts touch --hook`), 301 ms through the `repo` launcher.
- **Tiers** (`runV1`): hint (50 ms), hot set (paths published, applied, hinted or seen by the watcher in the last 10 minutes, newest 20 000, bulk operations above 500 paths note nothing; one pass about every 300 ms re-`lstat`s them, the directories that contain them, the directories that recent sweeps found changed, and the stat signature of `.git/index`, which publishes a stage made with `git add` without any event), directory sweep (an `lstat` of every known shared directory, only directories whose modification time moved are listed again, a recorded file that vanished and every file below a removed directory become deletes), full file sweep (`--scan`, default now 90 s, jittered 75 to 125 %; also at start, after an outage, after sync was switched on, after a `.gitignore` change and after a hint overflow). The OS watcher is an accelerator (`watch: false` disables it in code). All tiers feed one publish step per round (`publishOnceV1(paths, …, flat)` with the new `flat` directory list), which compares size, mtime and hash with the recorded state, so a path found by two tiers is published once. Slices are 6 ms (hot) and 10 ms (sweep) with a 1 ms gap; the pass intervals are duty-limited on CPU time (hot 8 %, sweep 12.5 % of a core, sweep never more often than every 2 s), so the tiers stay within budget when the kernel caches are cold.
- **Cost cuts**: the attestation view (`git status`) runs once at start and afterwards only after a relevant change (publish, apply, hint, sweep finding, stage or `HEAD` change), once quiet for 5 s or waiting 120 s, at most every 120 s; never on a pure timer. The git state (`HEAD`, stage tree) is read from a stat stamp of `.git/HEAD`, its ref, `packed-refs` and `.git/index`, and git is started only when the stamp moved (or after 5 minutes). `normaliseGitConfigV1` sets `core.untrackedCache=true` and `index.version=4` (not `feature.manyFiles`; `core.fsmonitor=false` stays; it also lowercases keys now, a bug the new test found). Per round the sync switch, store reachability, retirement record and repair request are read at most about once a second, the origin list every two seconds; forced garbage collection runs only after activity.
- **Latency** (passive): `publishMs` = time from the modification time of an edited file to the moment its event is written, `applyMs` = time from the wall clock in a remote event to its application, both only for samples younger than 10 minutes, p50 and p95 (nearest rank) over the newest 100 samples, in the attestation (`latency`, required, schema `Latency`), in `status` (`latency` in the report and a line) and as a `fleet` column `publish p50/p95 | apply p50/p95`. It refreshes with the 5 s heartbeat.
- Directory table: kept in memory in the replica's path book next to the file paths, filled by every full walk (`scanSharedV1` got an optional `listed` callback), re-learnt after a restart by the first walk.

## Measurements

Method: synthetic worktree `gen.ts` (100 000 files, 52 500 directories, cold mtimes), seeded with `seed.ts` (22.8 s), the module in-process through `runV1` with `watch:false` (the "before" copy is the live module with a one-line env switch that disables the watcher), edits and probes by `latency.ts` (head of the own journal polled every 2 ms), 5 samples per case for "after", 3 for "before". Host load average 7 to 33 during the runs (other agents building), 10 cores, `fseventsd` at 99 % CPU. Numbers are milliseconds unless stated.

Publish latency, before (`--scan 15`, as the task states): edit of a published file 8056, 9604, 11675; create 9845, 10078, 7567; delete 15627, 11356, 14150; atomic save 11611, 16389, 13361; cold in-place edit 10567, 13847, 15446.

Publish latency, after (defaults: scan 90, hint 50, hot 300, sweep 2000):

| Case | Samples | Median |
|---|---|---|
| hint, in-place edit of a cold file | 58, 53, 11, 38, 11 | 38 |
| hint, new file | 36, 54, 39, 38, 29 | 38 |
| hot: edit in place after a hint | 141, 229, 51, 442, 73 | 141 |
| hot: edit of a file published earlier | 39, 171, 313, 69, 284 | 171 |
| hot: delete of a published file | 132, 350, 357, 498, 360 | 357 |
| directory sweep: create in a cold directory | 3234, 1743, 2048, 299, 2086 | 2048 |
| directory sweep: atomic save over a cold file | 964, 512, 2030, 291, 2067 | 964 |
| full sweep: in-place edit of a cold file | 97 888, 68 988, 102 533 | 97 888 |

Directory sweep cost on this host: `lstat` of 52 604 directories takes 105 to 125 ms (2 µs each) with warm kernel caches and 0.75 to 1.2 s CPU (15 to 20 µs each) after the kernel dropped its metadata (3 to 4 s of idle was enough, the host has little free memory). On the 155 593 directories of the live tree a warm pass costs 0.35 to 0.5 s CPU (0.35 to 0.76 s wall, measured read-only), so the 12.5 % duty gives a pass about every 3 s, and a cold pass up to 3 times that. Async `lstat` with 16 to 64 in flight cost 2 to 4 times the CPU and was not reliably faster, so the sweep uses synchronous `lstat` in slices.

Idle daemon cost (same tree, 30 to 180 s windows after a warm-up, whole process, `idle.ts`):

| Configuration | CPU |
|---|---|
| before, `--scan 900` (what the topology file said) | 0.69 % (earlier runs 1.04 %, 1.27 %) |
| before, `--scan 15` (the task's description) | **62.1 %** (user 11.3 s, sys 63.3 s per 120 s) |
| after, defaults (scan 90, sweep, hot) | **12.3 %** (user 4.5 s, sys 17.6 s per 180 s) |
| after, tiers only (scan 3600) | 12.3 % (the sweep at its duty cap) |
| after, sweep and scan off (per-round cost only) | 0.88 % |

The live daemon of g1-server after the restart (272 000 files): 22.6 %, 18.2 % and 6.4 % of a core in three 25 s windows, plus a `git status` child (about 19 s) after each relevant change at most every 120 s (it ran every 60 s before); machines other than this one were not measured.

Per-round cost, local store, one other origin (`rounds.ts`, CPU µs including thread pool, 200 calls each): sync switch read 54, store stat 38, retirement read 29, repair-request `lstat` 32, rebase-request `lstat` 33, origin list 30, head of one origin 31, a whole idle follow 74 to 98, `gitStateV1` on a git worktree before 12 358 µs wall (two `git` processes) and after 112 µs (four `stat`s). Per second at 4 rounds: 20 + 4·N file operations before (N other origins), 4 + 4·N after (N = 5: 40 to 24; each of them an NFS call on a worker). Whole loop (poll 25 ms against 250 ms, earlier build): 0.97 ms per round before, 0.6 ms after. The saving is NFS round trips and process spawns, not local CPU: at 4 rounds per second the fixed work is below 1 % of a core both before and after.

`git status` on the live repo, read-only (index copies in a scratch directory, `--no-optional-locks` against the live index), 272 319 entries, 160 241 directories:

| Variant | Wall | User + sys |
|---|---|---|
| the daemon's command on the live index (defaults) | 18.1 s, 15.8 s | 3.6 + 19.3 s, 3.6 + 19.7 s |
| index copy, defaults | 13.9 s, 15.1 s | 3.5 + 19.5 s, 3.6 + 21.3 s |
| index copy, `core.untrackedCache=true` `index.version=4`, `--untracked-files=all` | 12.4 s, 16.1 s, 14.9 s | 3.7 + 17.7 s, 3.6 + 13.3 s, 3.5 + 13.3 s |
| the same with `--untracked-files=normal` (run 1, 2, 3) | 14.3 s, 5.2 s, 5.3 s | untracked part 12.0 s, 2.85 s, 3.0 s |
| `--untracked-files=no` | 2.3 to 3.0 s | |
| `git ls-files -o --exclude-standard` | 13.2 to 13.9 s | never writes the cache |

Trace2 shows where it goes: tracked refresh 2.2 s (preload `lstat` of 272 319 entries), `read_directory` for untracked files 13.2 s (160 241 directories, 434 673 paths). **The view's `--untracked-files=all` never uses the untracked cache and no configuration makes it cheaper; `--no-optional-locks` also means the daemon's status can never write the cache.** Plain `git status` (normal mode) drops from 14 s to 5 s with the cache. I kept `--untracked-files=all` (the view contract, per-file lines) and cut the cost by running it far less often. A cheaper view needs a contract change (directory rows with `normal`, plus a private index copy so status may write) and was not done.

## Hook registration and per-client status

Checked in: `.claude/settings.json` (existing keys kept):

```json
"hooks": {"PostToolUse": [{"matcher": "Write|Edit|MultiEdit|NotebookEdit", "hooks": [{"type": "command",
  "command": "repo \"$CLAUDE_PROJECT_DIR/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📜️script.ts\" touch --hook",
  "shell": "bash", "async": true, "timeout": 10}]}]}
```

The keys `async`, `shell` and `timeout` of a command hook were confirmed in the schema of the installed Claude Code 2.1.304 binary; `$CLAUDE_PROJECT_DIR` is the documented variable. `shell: bash` forces Git Bash on Windows, where Claude Code needs it anyway; on a Windows machine without Git Bash the hook fails silently (async, never blocks). PowerShell variant for a per-machine `.claude/settings.local.json` (gitignored), not checked in because one shared file has one shell: `{"type":"command","command":"repo \"$env:CLAUDE_PROJECT_DIR/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📜️script.ts\" touch --hook","shell":"powershell","async":true,"timeout":10}`. Verified live on g1-server with the exact command through `/usr/local/bin/repo`: exit 0, 301 ms wall, the hint file was consumed by the running daemon within 133 ms.

| Client | Status |
|---|---|
| claude-code | wired (above) |
| codex | not wired: `.codex/config.toml` configures MCP servers only; no per-edit hook is configured or was verified |
| cursor-chat, windsurf-chat, copilot-chat, antigravity-chat, kiro-cli, droid | not wired: the repository has no configuration directory for them (`.agents/` holds skills only); I did not invent hook files. `touch <path>` or a dropped hint file works from any of them, and a client whose hook payload carries `tool_input.file_path` can run `touch --hook` unchanged (not verified for any of them). |

## Files changed

Module (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`): `🎣️tiers/🟦️.ts` (new), `🎣️tiers/🧬️schema/🔣️.json`, `🎣️tiers/🧪️tests/{🟦️.ts,🐍️.py,🧫️fixtures/🔣️.json}` (new); `🔁️replica/🟦️.ts` (`runV1` rewritten, `observe` with `flat` directories and one batched pass for removed trees, `publishOnceV1(…, flat)`, `followOnceV1(…, reuse)`, folds `inside()`, latency and hot-set hooks, status latency), `🔁️replica/⌨️command/🟦️.ts`, `📜️script.ts` (stdin reader), `📖️texts/🔣️.json`, `🧬️schema/🔣️.json` (`Latency`, `Status.latency`); `🚧️boundary/🟦️.ts` (`listed` callback); `🌿️git/🟦️.ts` (stat-stamp cache of `gitStateV1`, `indexSignatureV1`, config keys); `🩺️attest/🟦️.ts` (view scheduling, `latency`, fleet row), `🩺️attest/🧬️schema/🔣️.json`, attest fixture and oracle; `🗺️topology/🔣️.json` (`scanSeconds` 900 to 90) with its fixture, oracle and test; `README.md`; tests: `🔁️replica/🧪️tests/{🟦️.ts,🌿️git.ts}`, `🌿️git/🧪️tests/🟦️.ts`, `🩺️attest/🧪️tests/🟦️.ts`. Outside the module: `.claude/settings.json`, `1113-git-state-contract.md`, `1113-latency-bench/` (seven scripts), this file.

## Tests (macOS, all green; run in the temp copy, then `🎣️tiers`, `🚧️boundary`, `🌿️git`, `🩺️attest` and `🔁️replica/🟦️.ts` again against the live module after the copy)

journal 127, boundary 39, git 10, tiers 53 (new: schema, Python oracle with numpy 2.0.2 and jsonschema over 45 vectors, path, hint text, hook and percentile vectors, latency sampler, hot set, directory table, hint drop), attest 59 (was 58: the view test rewritten for the new schedule, one latency test), replica `🟦️.ts` 73 (was 64: hint within one loop round with no watcher and the same edit without a hint not published, hot set in-place edit and a cold path left to the full sweep, directory sweep create, delete, atomic save, directory rename, new tree and removed tree, full sweep on a cold edit, no double publish with every tier on, latency in attestation and status, `touch` arguments and stdin, `touch --hook` well-formed, malformed, missing path, outside path, no `machine.json`, throwing stdin, hook to running daemon), `🏁️race.ts` 14, `🌿️git.ts` 14 (was 13: stage, new file and commit replicate with the watcher disabled), bootstrap 12, topology 56. The PowerShell topology suite did not run (no `pwsh`).

## Facts that differed from the task

- The topology file said `scanSeconds` 900 and the launchd job passes no `--scan`, so the live daemon scanned every 15 minutes, not every 15 s; the 9 to 10 s came from the watcher. The new default of 90 s therefore walks the tree 10 times more often than the file said (about 8 % of a core on the server).
- The directory sweep is the expensive tier on a host with cold kernel caches; its interval stretches with measured CPU time instead of being fixed.

## Not verified, risks

- Linux and Windows were not run (hint directory mode, `lstatSync` on Windows, the PowerShell hook, `repo` on the hook's PATH on workers). No physical machine other than g1-server was exercised; g1-m01..m04 restarted by themselves within two minutes and attest the same code digest and view digest (`bba2f5a3cd20`, `8f902a39`), g1-m05 still attests the old code with sync off.
- The watcher was emulated as absent in all latency numbers; the live fleet has a partly working one, which only makes it faster.
- No TypeScript compiler exists in this environment: the code is verified by tests, not by `tsc`.
- The view lags up to 120 s behind a change, so `fleet` can show `diverged view.digest` for up to two minutes after edits on a busy fleet (it was one minute plus quiet time before).
- `index.version=4` is set in the live `.git/config` (the index is still version 2 and converts at the next index write); a git GUI that cannot read index v4 would fail. Revert with `git config --local --unset index.version` if one does.
- The first attestation samples after a restart show the start-up catch-up (`publishMs` p50 39 s on g1-server), not steady state; the window is 100 samples.
- Another agent is changing `📒️journal`, the relay and `prepare` in `🔁️replica/🟦️.ts` in a separate copy (store IO); I deployed first and left their test files untouched, so their deploy must be merged onto the new `🔁️replica/🟦️.ts`, `🔁️replica/⌨️command/🟦️.ts` and `🩺️attest/🟦️.ts`, not copied over them.
- Ticket bookkeeping is manual (repo MCP unavailable); nothing was closed.

## Reproduce

`bun gen.ts <dir> 100 25 20 2`, `bun seed.ts <module> <dir> <store> b1`, then `BENCH_NOWATCH=1 bun latency.ts <module> <dir> <store> '{"scanSeconds":15}' "edit-published,create,delete,rename-save,edit-cold" 3` (before copy) and `bun latency.ts <module> <dir> <store> '{"scanSeconds":90}' "hint-edit-cold,hint-create,edit-hot,edit-published,create,delete,rename-save" 5`; `idle.ts <module> <dir> <store> '<options>' <seconds>`, `rounds.ts <module> <dir> <store> 200` (`GITWT=<git worktree>` adds the git cases), `sweepbench.ts <module> <dir>`, `hintcost.ts <module>`. Git: `GIT_INDEX_FILE=<copy of .git/index> git -c core.untrackedCache=true -c index.version=4 status --porcelain=v1 -z --untracked-files=all --no-renames` with `GIT_TRACE2_PERF`.
