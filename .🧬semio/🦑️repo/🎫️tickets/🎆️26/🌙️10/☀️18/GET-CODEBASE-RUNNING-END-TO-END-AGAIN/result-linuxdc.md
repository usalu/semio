# result-linuxdc (lane linuxdc, wave 1027b, mirror `lxdc` dropped): Linux / devcontainer review

`C` = this ticket folder. Tools `C/linuxdc-tools/`. Nothing live edited, no git command that modifies, no official run, no sub-agent. Evidence is static plus macOS-side Linux checks; nothing ran on Linux.

## FOR coordinator (two staged batches, independent, base = live files of 15:03, `stage.py check` says all `base`)

- batch `xdgroot` (5 files: lease registry root, agent session registry root, authorizer memo base + 2 test files)
  - `python3 -I C/linuxdc-tools/staged/xdgroot/stage.py check|apply|revert [--root R]`
  - laws: `sh C/linuxdc-tools/staged/xdgroot/verify.sh <MIRROR root with the batch applied>` (mirror: `python3 -I C/mirror-tools/mirror.py create <name> --batch "python3 -I C/linuxdc-tools/staged/xdgroot/stage.py apply --root {root}"`)
  - mirror result: new law 2/0 (registry roots vs a Python `os.path.isabs` reading; red on the base sources: `Received "semio/lease-registry"`), memo base 1/0, agent sessions 14 pass 1 skip, host capacity gate 24/0, negative control bites. `stage.py` mode law OK, `scan.ts` parse check 5 good 0 bad.
  - landing touches process-module sources: expect one re-inference/replan (no product run needed to prove it; the gate's next course/print run is enough).
- batch `tmpdirep` (2 files: resident session module `🟨️.mjs` + its service test)
  - `python3 -I C/linuxdc-tools/staged/tmpdirep/stage.py check|apply|revert [--root R]` · `sh C/linuxdc-tools/staged/tmpdirep/verify.sh <MIRROR root>`
  - mirror result: new law 1/0 (red on base: `Received "/private/tmp/claude-501/vft/semio-repo-1000/..."`), whole service test file 118 pass 0 fail; resident service test file: the same 3 failures with and without (base run had one more flaky failure), resident command 23/0, protocol 4/0, journal directory 4/0. `scan.ts` 2 good.
  - the resident service is a long-lived process: after landing, restart it as for clonejournal.
- rebuild a batch on the current live bases (hot files): `python3 -I C/linuxdc-tools/mkbatch.py <batch> <mirror-root-with-the-edits> <live-root>`.

## Container runtime

`/usr/local/bin/docker` (Docker Desktop 29.5.3) is installed, the daemon is NOT running (`Cannot connect to the Docker daemon at unix:///Users/ueli/.docker/run/docker.sock`); no colima, podman, lima, `container`. I did not start Docker Desktop (a VM). So the devcontainer image was not built and no suite ran on Linux; this is the second wave in a row with that gap (`C/result-xplat-linux-fix.md` ships `run-linux-suites.sh` for the first moment a daemon is up; it is the right first step, plus the exact CI sequence: `repo preflight`, `repo tools setup all`, `repo $CI_SUITES run --must-run --tool bazel --out .ci-suites`).

## What was checked (all against the live files of today)

1. Linux-target compile: the Go launcher cut like `launcher/📥️install/🐚️.sh` (flattened package) `go vet` + `go build -trimpath` for linux/amd64, linux/arm64, darwin/arm64, windows/amd64: clean (`C/linuxdc-tools/go-cross.sh <root> <outdir>`). The Rust watcher (inotify backend) `rustc --crate-name watcher --target x86_64|aarch64-unknown-linux-gnu --emit=metadata` clean, `--test` for linux clean (one dead-code warning of the macOS-gated module). Inotify backend re-read: poll `nfds_t` is u64, event parse is correct, `ENOSPC` is `lost("watch-limit")`, symlinks not followed, excluded directories are never watched. Only note: an unbroken event stream in watched directories postpones the batch (the debounce restarts at every wake-up, no maximum wait); harmless for builds (excluded `target`/`.cache` are not watched), not changed.
2. Linux laws that run on macOS through injected platforms, in the mirror: Linux host-capacity sampling (cgroup chain, `/proc` tables) 6 pass 1 skip; lease registry root 2/0; directory-exchange port (renameat2 table, syscall fallback numbers) 4/0; process-tree termination 2/0; boot clock 2 pass 1 skip (the real-Linux row is the skip). The suites platform-table law could not run in the mirror (it reads a runtime test outside the mirror's closure; a mirror artifact, not a defect).
3. Constants verified by reading: `renameat2` syscall 316 (x86_64) / 276 (aarch64, riscv64), `RENAME_EXCHANGE` 2, `AT_FDCWD` -100, unsupported errnos 95/38/22, `flock` contention errno 11, `LOCK_EX|LOCK_NB` 6, `ps -axo pid=,ppid=,pgid=` (procps-ng accepts it; `procps` is in the Dockerfile).
4. Pins, against the upstream sources (HTTP, read only): Node 24.15.0 linux x64/arm64 sha256 equal to nodejs.org `SHASUMS256.txt` (Dockerfile and bootstrap pins agree); Go 1.26.9, rustup 1.29.0, uv 0.11.6 linux x64/arm64 equal go.dev / `.sha256` files; Bun 1.4.2 (`linux-x64-baseline`, `linux-aarch64`), Bazel 8.4.2, Binaryen 130 linux sizes and sha256 equal the GitHub release asset digests. Every pinned tool has a linux arm64 row, so a Docker Desktop on Apple silicon is covered.
5. Dockerfile packages on Ubuntu 24.04: `libappindicator3-1` (universe), `libu2f-udev`, `dnsutils`, `cloc`, `libasound2t64`, `libgbm1`, `libvulkan1`, `libjpeg-turbo-progs` exist (Launchpad API). `libatk1.0-0`, `libatk-bridge2.0-0`, `libcups2`, `libatspi2.0-0` have no binary of that name on noble: they are t64 packages and apt installs them through `Provides` (it prints a "selecting ... instead" note); working, cosmetic only. All 12 devcontainer features resolve on ghcr (`dotnet:2`, `docker-in-docker:2`).
6. NOT a defect (checked and withdrawn): `"additionalVersions": "9.0 10.0"` is not comma separated, the documented format of the dotnet feature, but its `install.sh` word-splits the output of `split_csv` (`for v in $(split_csv ...)`), so both SDKs install. I had staged a fix and then dropped it. Repo projects target net8 with `rollForward: disable`, so the two extra SDKs (about 1.2 GB on a 14 GB runner disk) are unused: owner decision to drop the line.

## Findings and what happened

| # | finding | state |
|---|---|---|
| L1 | `leaseRegistryRootV1` and `agentSessionRegistryRootV1` used `XDG_CACHE_HOME ?? ~/.cache`: an EMPTY or RELATIVE value (`XDG_CACHE_HOME=` is common in CI images) gave the root `semio/lease-registry` relative to the working directory, i.e. a lease registry per cwd while the capacity gate (already `isAbsolute`) used `~/.cache`: mutual exclusion of leases lost across directories. The memo base accepted a relative value too (`||`). | FIXED, batch `xdgroot` |
| L2 | `endpointsV1` (resident service socket/key directory) used Node's `tmpdir()`, which on POSIX reads `TMPDIR`, `TMP`, `TEMP`, and the process environment instead of the injected one; the Go launcher computes the same directory with `os.TempDir` (`TMPDIR` only). With `TMP`/`TEMP` set and `TMPDIR` unset (some WSL/CI/container environments) the launcher and the service disagree about the socket directory. | FIXED, batch `tmpdirep` |
| L3 | CI: the last five `build-system.yml` runs on `🐙ueli/⛳wip` (10-22 to 10-26, commits 701-703) failed in 0 s with zero jobs ("likely failed because of a workflow file issue"); `gh api .../jobs` total 0. The current workflow and all five local actions validate against the schemastore schemas (0 errors), every `uses: ./` exists, so the cause is semantic and not found offline. The devcontainer jobs never ran. | OPEN: read the annotation of run 40301732436 in the Actions UI (needs a logged-in view; `gh run view --log` has none) |
| L4 | the devcontainer pulls `docker-in-docker` which adds `privileged: true`, while the compose file and law g9 state the container is unprivileged; the law scans only the compose/devcontainer text, not feature metadata. Functionally fine on a runner. | OPEN (doc/law owner decision; drop the feature if nothing needs Docker inside) |
| L5 | `devcontainer-suites` timeout is 350 min while `SEMIO_BUILD_BUDGET_MS=21600000` (360 min): the job is killed before the budget ends. Cosmetic. | OPEN |
| L6 | bootstrap `package_declares_workspace` is an awk JSON reader; Ubuntu's awk is mawk, the tests never run it under mawk (only BSD awk here, no gawk/mawk to run). Constructs read: POSIX classes, `sprintf("%c")`, recursion with locals, `exit a ? 0 : 1`: all supported by mawk 1.3.4. | UNVERIFIED: first container run decides |
| L7 | the GitHub runner disk: the build image (3 .NET SDKs, Go, Python, Rust, Bazel, caches) against about 14 GB free on `/`; the `repo preflight` floor is 8 GiB. No free-disk step in the workflow. | OPEN: watch the first run's `df` |

## Not verified
Anything that needs a Linux kernel: the devcontainer build, `repo tools setup all` in it, the five must-run suites there (leases, descriptor-lifetime, process-tree-termination, native-storage, native-storage-platform; `mustRun.linux` is empty by policy), inotify at run time, `renameat2` on overlayfs vs the `.cache` volume, flock on the Docker Desktop bind mount, mawk.

## Files
`C/linuxdc-tools/`: `go-cross.sh`, `mkbatch.py`, `staged/xdgroot/`, `staged/tmpdirep/` (manifest, files, base, stage.py from `xplat5-tools/stage.template.py`, verify.sh). Outputs under `C/🗑️generated/linuxdc/` removed.
