# Result: Linux verification, PowerShell suite and two-OS convergence (2026-11-13)

Module `MOD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Host: this Mac (arm64), Docker Desktop 29.5.3 (started with `open -a Docker`, answered `docker info` within about a minute, left running). Containers are arm64 Linux (Debian 13, kernel of Docker Desktop), bun 1.3.14, git 2.47.3, python 3.13.5, `ajv@8.20.0` (the version the repo pins) at `/node_modules`.

## Commands

Scripts are in `🧰️scripts/` (kept): `1113-linux/bun.Dockerfile`, `1113-linux/pwsh.Dockerfile` (bun image plus PowerShell 7.7.0 arm64 from the GitHub release tarball; the cached `mcr.microsoft.com/powershell:latest` was a 32-bit arm variant and unusable), `1113-linux/suites-inner.sh` (suite runner inside the container), `1113-linux/suites-mac.sh` (same on macOS), `1113-linux/mirror.sh` (builds a copy of the module next to symlinks into the read-only worktree), `1113-two-os.sh` (convergence driver).

```
cd 🧰️scripts/1113-linux
docker build -t semio-linux-bun -f bun.Dockerfile .
docker build -t semio-linux-pwsh -f pwsh.Dockerfile .
docker run --rm -v /Users/Shared/semio:/w:ro -v "$PWD":/s:ro semio-linux-bun  bash /s/suites-inner.sh          # task 2
docker run --rm -v /Users/Shared/semio:/w:ro -v "$PWD":/s:ro semio-linux-pwsh bash /s/suites-inner.sh pwsh     # task 3
../1113-two-os.sh                                                                                              # task 4
./suites-mac.sh                                                                                                # macOS re-run after the fixes
```

The whole worktree is mounted read-only at `/w`; each suite runs as `cd /w/MOD/<member>/🧪️tests && env -i PATH HOME=/tmp/home TMPDIR=/tmp bun test ./<file> --timeout 30000` (no `SEMIO_*`). Tests write only to the container `/tmp`. The closure outside the module is read straight from the mount (`🧰️framework/🛍️products/🦑️repo/📦️packages/…`, `🔨️modules/🧬️schema/…`, `🔨️modules/🏃️process/…`); only `ajv` comes from the image.

## Linux suites (final run, root in container)

| Suite | pass | fail | skip |
|---|---|---|---|
| `📒️journal/🟦️.ts` | 127 | 0 | 0 |
| `🚧️boundary/🟦️.ts` | 37 | 0 | 2 (root-only guards: `chmod 000` tests are skipped for uid 0) |
| `🌿️git/🟦️.ts` | 9 | 0 | 0 |
| `🩺️attest/🟦️.ts` | 35 | 0 | 0 |
| `🔁️replica/🟦️.ts` | 61 | 0 | 0 |
| `🔁️replica/🏁️race.ts` | 14 | 0 | 0 |
| `🔁️replica/🌿️git.ts` | 10 | 0 | 0 |
| `🔁️replica/🥾️bootstrap/🟦️.ts` | 12 | 0 | 0 |
| `🗺️topology/🟦️.ts` | 56 | 0 | 0 |
| `🗺️topology/🔵️.ts` (image without pwsh) | 0 | 0 | 19 |

Re-run of journal and boundary as uid 1000 (`--user 1000:1000`): journal 127 pass, boundary 39 pass, 0 skip.

## PowerShell suite on Linux pwsh 7.7.0

`🗺️topology/🔵️.ts`: 19 pass, 0 fail, 0 skip. No test needed a real Windows API: the harness `🧪️tests/🔵️.ps1` replaces every Windows cmdlet and executable with a recording function and maps drive letters onto a temporary directory. What this does not prove: real `Register-ScheduledTask`, `mount.exe`, `icacls`, Client for NFS, Windows 10/11 behaviour. Those remain unverified.

## Two-OS convergence run (`🧰️scripts/1113-two-os.sh`)

macOS replica (origin `g1-server`, darwin, bun 1.3.14) and Linux container replica (origin `g1-m01`, linux, `Linux aarch64`) on one store in a fresh temp dir under `$TMPDIR`, bind-mounted into the container at `/store`; macOS worktree and git repo in the same temp dir, Linux worktree `/lw` inside the container. Both daemons run a single copy of the module taken once from the live worktree (so peers editing it cannot make the daemons self-restart) with `--poll 100 --scan 5`. Nothing outside the temp dir is written; the live worktree is mounted read-only; the temp dir, container and macOS daemon are removed at the end.

| Step | Outcome |
|---|---|
| seed (macOS, 3 files, one commit, HEAD `e5e4e81…`), `join` in the container | Linux HEAD equals macOS HEAD at once |
| edit `dir/c.txt` on macOS | present on Linux after about 20 s (virtiofs delivers no inotify, the 5 s scan finds it) |
| create `new.txt` on Linux and `git add new.txt dir/c.txt` | macOS `git write-tree` = Linux `git write-tree` = `baeec934892f7644666346fbfbd6dff8d1a2e93e` after 0.5 s |
| `git commit` on macOS | Linux `git rev-parse HEAD` = `2977f4d4fab117ef7b1acc3ce9d6e7bcb39af0c5` after 5 s; `git status --porcelain` empty on both; Linux log `from macOS`, `initial` |
| `fleet --wait 90` on macOS, `fleet --wait 30` in the container | both exit 0, JSON `ok:true`, `differing:[]` |

```
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 linux 3s 0/0 on c2d15c800191 ok
  g1-server darwin 2s 0/0 on c2d15c800191 ok
All 2 origins with sync on are fresh (within 60 s), settled and identical.
```

Code digest `c2d15c800191` is equal on both OS (same module bytes). `fleet` agreement covers manifest, HEAD commit and stage tree.

## Fixes (all found by the Linux runs, none in the harness of this task)

1. `🗺️topology/🥾️provision/🐚️.sh`: `xml_escape` and `plist_with_environment` break on bash 5.2 and newer (Debian 13, Ubuntu 24.04+): `patsub_replacement` makes `&` in `${value//&/&amp;}` and in `${plist/$marker/$marker$extra}` expand to the matched text, so the LaunchDaemon plist got corrupted (test "a refused launchctl setenv stays a warning and the LaunchDaemon plist carries the fleet environment" failed, 55 pass 1 fail). Fix: `shopt -u patsub_replacement 2>/dev/null || :` as the first line of both functions (a no-op error on bash 3.2/4). Topology 56 pass on Linux and macOS.
2. `📒️journal/🟦️.ts` `writeBlobV1`: stage the blob with `copyFile` (keeps the source mode, git objects are `0444`) followed by `chmod`/reopen. On a store where such a `0444` temp file cannot be chmodded or reopened for writing (seen as `EACCES` on the Docker Desktop virtiofs `fakeowner` mount; plausible on squashed NFS or SMB too) the replica reported `blocked ... EACCES` forever for every `.git/objects/**` file, so a stage created on Linux reached macOS only as index entries without objects (`fleet` showed the macOS origin `lagging (not settled)`, incoming 2). Now the blob is created `wx` with mode `0644`, streamed, hashed on the way, `fchmod`ed and fsynced on the same handle, and the hash is checked before the rename. Unused imports `chmod`, `copyFile` removed. `🧪️tests/🟦️.ts` "a read-only source still publishes" additionally asserts the blob mode `0644`. Journal 127 pass on macOS and Linux (root and uid 1000), replica 61, race 14, git 10, bootstrap 12, attest 35 pass on both.
3. `🗺️topology/🧪️tests/🔵️.ps1` (PowerShell harness): the provisioning script now runs `repo <script> <command>` from the worktree root (`🔵️.ps1`, 00:36), while the harness recorded the directory of the current location and the TS test maps that leaf to `topology`/`replica`, so four tests failed with `repo undefined …` (a mismatch between concurrent edits, not Linux specific). The harness now records the directory of the script argument. 19 pass on Linux pwsh. Not re-run on macOS (no `pwsh` there).

## Not done or not verified

- Windows APIs, and `🔵️.ts` on macOS (no pwsh on this Mac).
- NFS: the store here is a virtiofs bind mount, not NFS; attribute-cache and `rename` behaviour over NFS are covered by `linux-nfs-report.md`, not by this run.
- Latency numbers on the bind mount are not meaningful (the 20 s edit propagation comes from missing inotify plus the 5 s scan).
- Concurrent commits on two machines were not exercised (single committer).
- Docker pulled `oven/bun:1.3.14` (the repo pins 1.4.2 in `package.json`; the runtime-pin preload from `bunfig.toml` only applies with the repository root as cwd, so the driver and runner start from the worktree directories).
