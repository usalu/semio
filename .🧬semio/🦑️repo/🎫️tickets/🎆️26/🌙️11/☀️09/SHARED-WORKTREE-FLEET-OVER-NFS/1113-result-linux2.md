# Result: Linux verification of the day's landings, two-OS run, service definitions (2026-11-13)

Host: this Mac (arm64), Docker Desktop 29.5.3. Containers: arm64 Debian 13, bun **1.4.2** (the repo pin; `semio-linux-bun142`, plus `semio-linux-pwsh142` with PowerShell 7.7.0), git 2.47.3, python 3.13.5, `ajv@8.20.0` (the repo pin), systemd 257 in a privileged container. Scripts (kept): `🧰️scripts/1113-linux2/` (`bun.Dockerfile`, `pwsh.Dockerfile`, `systemd.Dockerfile`, `mirror.sh`, `suites-all.sh` = every test file, `two-os2.sh`, `gen-units.ts`, `systemd-check.sh`, `patch-*.py`, `land.py`, `run-live.sh`, `fallback-solo.sh`, `kill-solo.sh`). Everything ran against copies (mirror in a temp dir, live tree mounted read-only), temporary stores and worktrees.

## 1. Every test file on Linux (root; journal/boundary/access again as uid 1000)

All test files of the module (clash, state, compaction, boot, resilience, chaos, killpoints, race, git, store, relocate, bootstrap, adopt, entry, access, tiers, journal, boundary, attest, topology incl. the PowerShell suite): all pass on 1.4.2, 0 skip as uid 1000. `🌪️harness.ts`/`🌪️sweep.ts` are libraries/CLIs, not suites.

Failures found and what they were:

| Finding | Cause | Action |
|---|---|---|
| `tiers`: drop directory not `1777` | bun 1.3.14 `chmod` drops the sticky bit; bun 1.4.2 is correct | none in the module; the first container image (1.3.14) was wrong, rebuilt on the pinned 1.4.2 |
| `state`: 2 tests, ajv `strictRequired` error | `Decoded.files/queue.items` of `💾️state/🧬️schema` list `required: [path,hlc,operation]` without `properties`; ajv 8.20 (strict) rejects it, the macOS run resolves an old ajv 6 by accident | **fixed**: `properties` added to both item schemas (landed) |
| `resilience`/`replica` once each in the first run | flaked under 1.3.14, passed in 6 later runs on 1.4.2 | none; not reproduced |
| `boundary`: 3 tests | harness only (mirror lacked `.gitignore`) | script fixed |
| `topology/🔵️.ts` 18 failures | harness only (env of `pwsh` dropped by `env -i`) | script fixed; 20 pass |

## 2. Two replicas, macOS + Linux (`two-os2.sh`, final run: `failed=0`)

Linux code is a gitignored copy inside `/lw` (the daemon only seals code that lies inside its worktree). Steps and outcomes: seed + join; edit propagates (~1 s); stage on Linux and macOS `write-tree` equal (<1 s); commit on macOS, Linux HEAD equal (~3 s); case rename of an untracked file `low.txt` -> `LOW.txt` arrives; case clash `Foo.txt` (mac) vs `foo.txt` (Linux) at the same time -> both trees equal, one name wins (`case-collision`), loser bytes in `.cache/f/lost`; file-vs-directory `fd` vs `fd/inner.txt` -> both trees equal (`file-replaces-directory` / `directory-replaces-file`); `kill -9` of the Linux supervisor and daemon during a 1500-file bulk apply, restart -> 1500 = 1500; sealed snapshot exists; module sabotaged on Linux (`tables.dirs.adopt is not a function`), kill, restart -> fleet shows `warning: g1-m01 runs a fallback code (snapshot: the worktree code does not start: exit 1)` (`code-fallback`), the fallback daemon keeps syncing; module restored, restart -> no fallback; final `fleet` exit **0 on both OS**, trees byte-identical.

Findings of the run that are not module bugs but traps for whoever repeats it:
- The daemon is now started through the installed boot program (`~/.local/share/semio/fleet/entry/boot.mjs`); a `kill -9` by command-line pattern must match `boot.mjs` too (earlier iterations "killed" nothing and hit `Another replica daemon already runs`).
- A tracked file renamed only by case (`dir/c.txt` -> `dir/C.txt`) makes `fleet` say `diverged` (git status view differs: on the case-insensitive volume git sees no deletion). Expected capability difference, listed here, not changed.
- A Docker container without `--init` leaves the killed daemon as a zombie; the lock check (`kill(pid,0)`) treats a zombie as alive. Only possible when PID 1 does not reap (not under systemd); not changed.
- One run took 159 s to deliver 1500 small files macOS -> Linux over the bind mount (others: seconds); virtiofs, not reproduced.
- One scenario run wrote to the live `.cache/f/boot/ready.json` because my first mirror symlinked `.cache` into the live tree (mac daemon of the harness). Restored to the live value (`{"pid":99888,"waiting":false}`); mirror now links only `.gitignore`. `beat.json` is rewritten every second by the live daemon.

## 3. Service definitions: systemd, launchd, Windows

Measured in a systemd 257 container with the generated units (`systemd-analyze verify` clean for replica, cache, relay):

- Before: default start limit (5 starts / 10 s) with `RestartSec=2` sits exactly on the boundary; with `RestartSec=100ms` (or any load that compresses restarts) a crash-looping unit ended `failed` and stayed dead after 5 restarts. A TERM-ignoring daemon took **90 s** to stop (default `TimeoutStopSec`).
- After: `StartLimitIntervalSec=0` on replica, cache and relay: the same crash loop ran 60 restarts in 15 s and stayed `activating (auto-restart)`. Replica: `KillMode=mixed`, `TimeoutStopSec=30`: TERM goes to the supervisor only; a blocked child is killed by the supervisor after 20 s (measured stop 20 s), a supervisor that ignores TERM is SIGKILLed at 30 s (measured 30 s); `kill -9` of the main process is restarted (NRestarts=1).
- With the new entry shim (`boot.sh`, dash): internal crash loop keeps the unit `active` (no systemd restarts), stop 2 s; hung child stop 30 s. Observation for the shim owner: a TERM during its back-off `sleep` (up to 60 s) is handled only after the sleep, so stop then ends at the 30 s SIGKILL (bounded, unit marked `failed (timeout)`); `sleep "$delay" & wait` would make it immediate.
- launchd (static review): `KeepAlive` true, `ThrottleInterval` 5 (launchd has no start limit), added `ExitTimeOut` 30 (default 20 s equals the supervisor grace and could SIGKILL the supervisor during its own child kill).
- Windows task XML (static review): `RestartOnFailure` PT1M x 255, `ExecutionTimeLimit` PT0S, `MultipleInstancesPolicy` IgnoreNew, logon trigger plus a time trigger every minute (revives after count exhaustion or exit 0). No change needed; asserted in the test now. Real `Register-ScheduledTask` behaviour not verified (no Windows).

Landing (`1113-landings.md`, 10:5x): `🗺️topology/🟦️.ts` (constants `UNLIMITED_STARTS`, `STOP_BOUND`, `ExitTimeOut`; conflict with the entry-shim landing resolved by hand), its test (assertions for all three platforms and the three server units), `🔁️replica/💾️state/🧬️schema/🔣️.json`, README bullet. Merge by three-way merge, all test files green on the live tree afterwards (macOS) and topology/state/entry again in Linux.

## Not verified

Real Windows; NFS (bind mount only); the real `boot.mjs` under systemd (stand-in runtime used for signal timing; the real supervisor's TERM/kill timing is covered by `🛟️boot.ts`, which passes on Linux); `fleet` with three or more OS at once; uid-1000 run of the replica/boot suites (root only).
