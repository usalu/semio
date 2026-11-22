# 🚀 Wave 1122 Brief: Finish the Build System End to End

Coordinator: main chat (Opus). Executors: Sonnet lanes `E*`. Auditors: Haiku lanes `H*`. Started 2026-11-22 00:40 CET.

`T` = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`
`W` = `T/wave-1122/` (this folder). `B` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `C` = `…/📚️library/⚡️caching/`, `HC` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/`.
`S` = `/Users/Shared/semio-store/v1/` (fleet store: `S/origins/<origin>/ready.json`, `ready-log-*.txt`, `S/fleet/cache.json`). Worker readiness ticket: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/`.

## Goal of the wave
The build system works end to end on the whole fleet: every live worker (16 GB Macs, Windows, Linux) reaches Ready, the fleet shares one hot cache (nothing computed twice, outputs identical across machines of one platform), a machine under pressure offloads to idle machines, and nothing fails or hangs when a remote part is down.

## Live state at 00:38 (measured)
- 0 of 13 workers Ready. 10 macOS workers sit in stage `workspace` reading the graph slice for hours ("taken N of 600"). m05 Windows in `dependencies`. m04 Linux (`tools`, 24 h stale), m06 (29 h stale), m09 absent.
- Remote cache 6094: 113,507 entries, 147 MB; **1000 AC divergences** (same action key, different output sha from different origins); peers tier 0 hits / 704 misses / 20 errors; nx namespace 0 puts.
- Fleet unit counters: cacheHit 131k, cacheMiss 251k, cacheSkipped ~100k with empty `cacheSkipReasons`.
- Remote execution 8980/8981: 11 workers registered, completed 1, failed 1, offloaded in total 3. Six machines refuse with `no-headroom` while m07, m10, m13, g1-server, m05 are idle with headroom.
- Coordinator host (this Mac, 32 GB): swap 20.0 of 21.5 GB used, load 17-35, 54 GB free disk, `fseventsd` at 9.6 GB RSS.
- Wave 1121 results (read the ones that touch your lane): `T/wave-1121/result-*.md`, `census-*.md`, `audit-R2.md`; landing log `T/landings.md`.

## Hard rules (binding)
1. Read `T/design.md` (decisions + binding landing rules) and the census/result files of your lane first. Verify in code anything you rely on.
2. No git command that changes the repo or the working tree; no worktrees; never `pkill`/`killall`; kill only PIDs you started; no sudo; macOS has no `timeout`; `cd` explicitly in every shell call; do not edit `AGENTS.md`; do not open/close/reopen tickets or goals; no sub-agents; run everything in the foreground and leave no background process, timer or monitor behind (a background child dies with your turn anyway).
3. Do not delete or sweep any `🗑️generated` folder you did not create. Your generated output goes to `W/🗑️generated/<lane>/` and you delete it when done; input scripts you keep go to `T/🧰️scripts/<lane>/`.
4. Working copies OUTSIDE the worktree under `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-kfaIfm/ccee8eb5-3363-4a84-af9e-53e38d236627/scratchpad/<lane>/`. No symlink from a scratch dir into the worktree; no `mv`, `rsync --delete` or `rm -r` on a path whose `realpath` starts with `/Users/ueli/Documents/semio` except your own `W/🗑️generated/<lane>/`. Scratch < 500 MB.
5. Landing (the tree replicates to 13 machines in about one second and their supervisors restart on code change): one landing at a time in the build system: `mkdir T/landing.lock` and write your lane id + time into `T/landing.lock/owner`; remove the folder when done; a lock older than 40 minutes may be taken over. Re-read every live file right before editing it (others edit the same files; last writer wins); land by anchored edits, never whole-file restores from an older copy. Before releasing the lock run the test files of every member you touched and of its direct dependants. Record the landing in `T/landings.md` (append a section `## Wave 1122 <lane>: <title>` with time, files with sha256 prefix before -> after, tests run with pass/fail counts). Compile-atomic: never leave the tree in a state that fails to load between two edits.
6. A broken cache, scheduler, network or peer must never fail or hang a build: every remote step has a timeout and falls back to local.
7. Never start a long real build of the whole repo. Prove mechanisms on small slices (one crate, one project). If the 1-minute load average is above 40, wait (check every 60 s) before starting builds or test runs heavier than one file.
8. Never print, log or write a token or credential value.
9. Code style (CLAUDE.md): concise, no comments inside definitions, every docstring starts with a unique emoji, user-visible texts in English and German, schema-first (JSON schema before code for every new format/wire message), no compatibility layers, no migrations, no deprecations, no new runtime dependency on external libraries (tests may use them), test-driven with at least one language-agnostic test and a cross-check against an independent implementation where one exists. Greenfield: no legacy support.
10. Tests: `cd <tests dir> && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun test ./<file> --timeout 180000` (always `./` before the file: a bare path is a filter that scans the whole checkout). Some members use another runner: read their `🧪️tests` folder and `project.json`.
11. Live fleet: you may read anything in `S/`. Do not write into `S/` and do not restart services on other machines; code you land reaches them through the replica and their supervisors restart themselves. Prefer repo MCP tools / `repo` CLI for builds and status (`repo fleet status --json`, `repo remote-cache status`, `repo remote-execution status`, `repo doctor --json`, `repo capacity status --json`).
12. Peers (other agents, other people) work in the same files. Stay inside your lane, do not undo changes you did not make, do not stop because others touch the area.

## Report
Write `W/result-<lane>.md`: what changed (files), how each behaviour is proven (test names, measured numbers, live evidence), what is NOT verified, what other lanes must know. Final message to the coordinator: at most 10 lines, pointing to the report and naming everything unverified. If you find a fundamental issue outside your lane, write it to `W/findings-<lane>.md` and mention it in the final message.

## Addendum 00:56: per-area landing locks (replaces the single lock of rule 5)
Two lanes landed without the lock because one global lock serialised 12 executors. From now on the lock is per area: `T/landing.lock.<area>/owner` where `<area>` is the first folder below `B` or `C` that you change (e.g. `graph`, `remote-cache`, `remote-execution`, `host-capacity`, `caching-cargo`, `caching-storage`, `tools`, `replica` for `🤝️shared-worktree`). Take every area lock you touch, always in alphabetical order of area name (no deadlocks), release all at once. A lock older than 40 minutes may be taken over. Landing without the lock is forbidden; when a lock is held, poll every 60 s. The old `T/landing.lock` counts as area `replica` until E6 releases it.

## Addendum 01:20: report files
Several lanes saw the Write tool refuse new report files. If Write is refused, create or append your report with Bash (`cat > W/result-<lane>.md <<'EOF' … EOF`). Report files are mandatory.

## Addendum 01:30: no whole-file writes of shared files
`B/🔣️policy.json` was reverted three times between 01:05 and 01:13 by a stale whole-file write. Never write a whole shared file (policy JSON, schemas, rules catalogues, fixtures shared by several members) from a copy: edit it in place with anchored edits or a script that loads the live file, changes only your keys and writes it back immediately, under your area lock. Re-read right before.

## Addendum 06:05: rule 10 replaced (tests)
10. Tests: `cd /Users/ueli/Documents/semio && repo test-file [--json] [--bail] <path>...` (MCP tool `test_file` after the next MCP rebuild). It routes each file by the suite table of its area (build, caching, process; other files run plain) and runs it as the suite runner would (Original envelope with the suite's caller grant, genuine-birth launcher, managed Python, hermetic sandbox, checkout as working directory) without a graph read, 0.5–15 s per file; exit 0 all passed, 1 a file failed, 2 refused; `--dry-run` shows the routing, `--json` one structured result, Ctrl-C cancels. Do not call `bun test` yourself; the raw form of the old rule 10 is only for the case that `repo` itself does not start.
