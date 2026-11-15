# Package K: Linux and Windows position (2026-11-13 evening)

## 0. What could NOT be done: no Linux run at all

- g1-m04 attests `platform: linux` (`/Users/Shared/semio-store/v1/origins/g1-m04/attest.json`; the file records no architecture, no distribution, no libc, no bwrap).
- The runtime of the earlier agent (Docker Desktop 29.5.3, arm64 Debian 13 containers) is installed (`/Applications/Docker.app`, `/usr/local/bin/docker`) but its daemon was NOT running. I started it with `open -a Docker`; it stopped at a macOS administrator prompt ("Docker Desktop requires privileged access to configure privileged port mapping", an `osascript ... with administrator privileges` dialog). I cannot and must not enter that password. Nothing else (podman, colima, lima, nerdctl, OrbStack) exists. **Consequence: no bun test file ran on Linux, no two-platform fleet with a real Linux agent, no Rust toolchain in a container.** Everything about Linux below is static review plus tests that run on macOS through the existing platform seams. If the prompt is still on screen, someone with the password can accept it (or quit Docker); the Linux run then is `🧰️scripts/1113-linux2/` (bun 1.4.2 image, mirror copy of the trees) plus `🧰️scripts/K/run-files.sh` adapted to the container paths.
- Left behind by me: a Docker Desktop process tree and the blocked prompt (I did not kill them: not mine to kill after the app takes over).

## 1. Linux: static review and what was fixed

Reviewed (grep of every POSIX call in the packages' non-test code, then reading the hits): `/proc` and PSI sampling, signals and process groups, `bwrap`, `tar`, reflink, hard links, modes, case sensitivity, `/bin/sh`.

| Area | Finding | State |
|---|---|---|
| `/proc/meminfo`, `/proc/pressure/memory`, cgroup v1/v2 memory and CPU quota, `file-nr` | already implemented and fixture-tested in `🏃️process/🏗️build/🌡️host-capacity/📡️sampling` (`parseLinuxCountersV1`, `cgroupMemoryFilesV1`, `parseCpuQuotaCoresV1`, `cgroupV1CpuFilesV1`); not touched by tonight's code, which only calls `evaluateHostCapacityV1` | ok by reading, not run |
| process-tree kill | worker spawns `detached` (own process group) and kills `-pid` with SIGKILL, `taskkill /T /F` on Windows; bubblewrap has `--die-with-parent --unshare-pid` | ok by reading, not run |
| **bwrap availability probe** | the probe ran `bwrap --unshare-net --ro-bind / / /bin/true`; the real sandbox also needs `--unshare-pid --unshare-ipc --dev /dev --proc /proc --new-session`. In a container where the bare form works but a new `/proc` or a PID namespace is refused (typical Docker without `--privileged`/with masked `/proc`), the worker would have chosen `linux-sandbox`, claimed `confinement: full` and then failed EVERY action. **Fixed**: `remoteExecutionBubblewrapProbeArgumentsV1(root)` derives the probe from the real command line (single source), so such a host falls back to `isolated-root` and says `UNCONFINED` / `network unenforced`. Test: `🔒️sandbox/🧪️tests` "the Linux availability probe runs the real sandbox command line" (4 pass). Container vs real host: in a container bwrap needs user namespaces (`--security-opt seccomp=unconfined` or `--privileged`); on a real host with unprivileged user namespaces enabled (Ubuntu 24.04 restricts them through AppArmor: `kernel.apparmor_restrict_unprivileged_userns=1` makes bwrap fail until a profile exists) the probe passes or fails honestly. Real bwrap behaviour NOT verified. |
| `tar` | `-cf - --no-xattrs -C` / `-xf - --no-same-owner -C` is valid GNU tar (>=1.28) and bsdtar; busybox tar (Alpine) has no `--no-xattrs`. Not run on GNU tar. |
| reflink | every use is `COPYFILE_FICLONE` without FORCE: falls back to a plain copy on ext4/xfs-without-reflink; no hard-link-across-volume assumption except `linkSync` for locks inside one directory | ok by reading |
| file modes of the credential copy | `0600` via `writeFile mode` + `chmod`; reading needs no mode check on the client; the NFS export is `all_squash` on Linux (A): by reasoning only | not verified |
| case-sensitive paths | all new member paths are exact, no case-insensitive lookup was assumed in the new code | ok by reading |
| offload action | `/bin/sh -c` script is POSIX (dash-safe: `eval`, `case`, `set --`), macOS-only linker flag is guarded by `uname -s` = Darwin; sandbox `readable` is a no-op on Linux (host bound read-only) | ok by reading |
| **native CPU identity** | for `-C target-cpu=native` the key used `os.cpus()[0].model`, which is `unknown` for every ARM Linux CPU, so two different ARM CPUs (Graviton vs Neoverse-N1, different feature sets) would share code compiled for one. **Fixed**: `cpuIdentityV1(model, cpuinfo)` falls back to `CPU implementer/architecture/variant/part` plus a digest of `Features` from `/proc/cpuinfo`. Test in `🪪️identity/🧪️tests` (8 pass). |
| **libc is not in the key (NOT fixed)** | on Linux the `linker` part of the platform key is empty (only Darwin has the OS major release). Two Linux machines with different glibc could share a linked unit (bin, proc-macro, cdylib) built against a newer glibc. Harmless today (one Linux machine); must be added (glibc version from `ldd --version` or `process.report`) before a second Linux machine joins. I did not add it blind because the probe could not be run on Linux. |

## 2. Two-platform fleet in miniature (macOS only, Linux half simulated)

No Linux agent was available, so the platform separation is proven with workers that CLAIM the other platform's properties (`OSFamily`, `Arch`, `semio-toolchain`) against a real scheduler and cache (processes on scratch ports, scratch stores):

- `🌐️remote-execution/🧪️tests/🌐️fleet` "two platforms in miniature": 3 workers (`macos` + toolchain A, `linux` + toolchain A, `linux` without any compiler). 4 macOS-bound actions all ran on the macOS worker, 4 Linux-bound all on the Linux worker, never on the other or on the compiler-less one; an action for Windows, and one for an unknown compiler build, are refused (status not 0; the offload client then compiles locally). 9 pass in that file.
- `🛰️fleet/🧪️tests` "a requester whose compiler build differs from the fleet's" (real cargo, real cache server, three scratch machines at different paths): machine 1 builds and uploads (`built` units), machine 2 with another `SEMIO_UNIT_CACHE_TOOLCHAIN` gets 0 remote hits, compiles every unit and uploads under its own keys (server `puts` = 2 x built), machine 3 with the fleet's build compiles nothing (all remote hits). Platform key moves with OS/arch/host triple/rustc digest/launcher toolchain (unit tests of package B, 7 pass in `🪪️identity`); the same mechanism, the manifest platform check in `fetchFleetUnitV1`, would also refuse a forged foreign entry.
- Platform-NEUTRAL entries (by code, not by test across OSes): pinned downloads (key = sha256 of the bytes, `fleet-fetch/🟦️.ts`), crate files (key = sha256 of `Cargo.lock`, `🦀️crates`), the download mirror. Platform-BOUND: Rust units, install packs (`fleetInstallKeyV1` includes platform + arch + Bun version), Nx task slots (always `<os>-<arch>`; the Nx cache key itself is the Nx hash, shared only where the hash is equal; G found almost every hash carries the platform), graph state and remote-exchange (platform in the key).
- NOT done: a real Linux agent under the scheduler taking a trivial Linux action; the second container checkout of a tiny crate through the wrapper (needs the container).

## 3. Windows, statically

Fixed (plain portability bugs, each with a test through a platform parameter):

1. **Install packs** (`fleet-fetch/📦️install`): a Windows junction reads back as `C:\store\...` or `\\?\C:\...`; the pack compared with `startsWith(store + "/")`, so links were never rewritten and the fingerprint contained the path. New `fleetInstallLinkTextV1`, used by fingerprint and rewrite; the rewrite creates a `junction` on Windows. Test "Windows junction targets ..." (11 pass).
2. **Receipt content links** (`fleet-content`): `symlink(relative, path)` without a type creates a file link on Windows (wrong for a directory) and needs a privilege. New `storeReceiptLinkPlanV1`: POSIX relative link, Windows directory junction to the absolute target. Test (10 pass).
3. **Fleet cache facts copy** (`remote-cache/🛰️fleet`): the 30 s refresh renames over a file that every concurrent compiler wrapper has open; Windows answers `EPERM/EBUSY/EACCES`. New `renameOverReadersV1` retries five times (20..100 ms) on Windows only. Test (11 pass).

What a Windows worker does and does not get tonight (from the code; nothing ran on Windows):

| Feature | Windows |
|---|---|
| rustc wrapper through `repo` | native-executable shim (`ensureUnitCacheShimV1` Windows seam, `🪟️windows` 7 pass), unchanged by tonight |
| Rust unit cache, fleet tier | works in code; platform key `windows`; hits only from another Windows machine with the same compiler build (none exists), so it uploads for nobody and downloads nothing for now. Uploader is a detached process; spool renames use unique names |
| Rust offload | excluded by an explicit check (`offload/🟦️.ts`: `not-portable`/`windows`), the unit compiles locally, counted as declined |
| Windows as offload worker | the agent registers (`OSFamily windows`), sandbox `isolated-root` only (no bubblewrap/sandbox-exec), `confinement none`; no action is ever generated for it (the only action generator excludes Windows and the key would not match any other machine); the `/bin/sh` action script could not run there anyway |
| Sidecar supervision | `proc.kill("SIGTERM")` is a hard terminate on Windows: the agent does not get its graceful abort, running action trees are not killed by the worker (orphans possible). Not changed (needs a Windows job object) |
| Pinned downloads, crates, mirror | work (HTTP + sha256); `tar` for install packs is the Windows 10+ bsdtar (`--no-xattrs` accepted by libarchive >= 3.2); packs made by POSIX machines are keyed by platform so a Windows machine never restores one |
| Nx single-flight front, claims, graph state | Node http / bun:sqlite / path normalisation by `sep`; graph state key includes `win32-x64`; none exercised |
| Credential file mode | `0600` is ignored by Windows; the copy is under the user profile, the group server (macOS) writes the facts, so the "mode must be 0600" re-publication check never runs on Windows |
| Long paths | no `\\?\` handling in the new code (the existing `@argfile` mechanism covers command lines over 32 767 characters); offload paths are virtual and POSIX |
| Symlinks without Developer Mode | the three places above now use junctions; `fleet-fetch` crate/blob placement uses files only |

## 4. Toolchain identity across machines

- A machine without the pinned `rustc` : `readRemoteExecutionToolchainV1` returns `null/null`; the platform key ends in `-none`; `remoteExecutionHostPropertiesV1` then omits `semio-toolchain` and `semio-target`, so the worker does not advertise a compiler. A toolchain-bound action never goes there: proven in the new miniature test (`linux` without compiler took none of the 8 bound actions) and by D's "an action goes only to workers of the requester's toolchain build".
- A requester whose compiler differs: no hits, own keys (test above); the offload action carries `semio-toolchain`, the scheduler refuses it when nobody has that build (status not 0 -> compile locally; D's test: failed back at once).
- Visible reason (NEW): `repo fleet status` now adds, under the machine list, `{origin} shares its platform key with no other machine (other keys: ...)` and `{origin} reports no Rust compiler (...)`, English and German (`fleet.apart`, `fleet.norustc`), only when more than one machine reports. Test "the fleet section says why a machine shares nothing" (13 pass). Real `repo fleet status --lang en` runs through it (single machine today, so no extra lines).

## 5. Tests run (macOS, bun 1.4.2, fleet env unset; script `🧰️scripts/K/run-files.sh`)

sandbox 4, fleet-status 13, install 11, fleet-fetch 8 + crates 4 + mirror 2, fleet-content 10, store-receipt 17, task-slot 2, remote-cache `🛰️fleet` 11, scheduler 7, placement 6, sidecar 2, remote-execution main 16, `🌐️fleet` 9, offload 17 + cluster 8, unit-cache main 26, `🪟️windows` 7, key 8, identity 8, unit-cache fleet "compiler build differs" 1 (35 s).
Failing, not mine, not changed by me: `⌨️command/🧪️tests` "serve starts the store, the service and a worker process ..." times out at 180 s (6 pass 1 fail, reproduced twice under load average 9 to 14; D and E reported the same file red at their landings; probably the local worker's admission/headroom, not examined).

## 6. Unverified / what the others must know

- Nothing ran on Linux or Windows. The full Linux file list of the brief (remote-cache, slots, remote-execution, graph-state, host-capacity, fleet-status, unit cache, bootstrap, Nx front) is unrun there.
- bwrap on a real host and in a container; GNU tar flags; NFS `all_squash` and the credential read; PSI sampling; glibc in the key; Windows junction creation and bsdtar with junctions; hard process termination of sidecars on Windows.
- To run Linux: someone accepts the Docker Desktop prompt (or starts another runtime), then reuses `1113-linux2/bun.Dockerfile` and `suites-all.sh`, mounting a COPY of the trees.
