# 🧭️ Cross-platform audit of the wave 1026g changes (aud-xplat-g, read-only)

Auditor: aud-xplat-g. No source file was edited, no git command that modifies state was run, nothing was built, and no graph-loading command was started. Static audit only: no Windows or Linux host was run.

## Scope and method

- **Changed-file list.** Only about eight of the lane result files have a "changed files" section. The rest name their changes in status text. So the scope is the working-tree list from `git status` (uncommitted, outside the ticket folder): 587 source-like files (`.ts .mjs .mts .py .sh .ps1 .rs .go .json .md`, plus `.devcontainer/*`, `hooks/*`, `nx.json`). Every one of them has a modification time of 2026-10-26. This list also holds edits of other waves and other agents, so it is a superset of the 1026g lanes' files, not an exact one.
- **Sweep.** Grep over all 587 files for: process listing and kernel tools (`ps`, `vm_stat`, `sysctl`, `hdiutil`, `lsof`, `sandbox-exec`, `taskpolicy`, `pmset`), `/proc`, cgroup, signals and process groups (`kill`, `SIGTERM`, `SIGKILL`, `detached`, `-pid`, `taskkill`), temp dirs (`/tmp`, `/var/folders`, `TMPDIR`, `LOCALAPPDATA`), separators (`split("/")`, `path.sep`, drive letters), case folding, symlinks and junctions, hashing tools (`sha256sum`, `shasum`), shell and `.exe` use, `statfs`, `renameat2`, `/dev/null` vs `NUL`.
- **Manual reads** of every flagged module, then a grep of the test files for the same branches. Hits are in `🗑️generated/aud-xplat-g/` (`all.txt`, `hits-*.txt`), to be swept with the ticket.

## Findings, ranked

Severity: **High** = crash or wrong results on a supported platform if the path is on by default; **Medium** = silent wrong or skipped behaviour, or a crash on a reachable branch; **Low** = conditional, or a parity or hardening gap. "Owner" is the lane named in its result file; all those lanes report finished, so the coordinator owns the fix unless a lane is reopened.

### 1. High (conditional): unit-cache compiler wrapper is a `.cmd` file on Windows
- **Where:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts:28-35` (`unitCacheShimPathV1` returns `rustc-wrapper.cmd` on win32, `unitCacheShimSourceV1` writes `@"runtime" "script" %*`) and `:48` (`RUSTC_WRAPPER: ensureUnitCacheShimV1(...)`).
- **Platform:** native Windows.
- **Effect:** Cargo runs `RUSTC_WRAPPER` as `<wrapper> <rustc> <args>`. Cargo's rustc arguments contain quotes (`--cfg feature="std"`). Rust's std refuses to pass such arguments to a batch file (the CVE-2024-24576 hardening, Rust 1.77.2 and later), so every compile would fail. Expected effect: crash on every compile. **Not verified on Windows**; the mechanism is from Rust's documented behaviour.
- **Test:** `🗃️unit-cache/🧪️tests/🟦️.ts:279` checks only the text of the shim. No test runs Cargo through it on win32.
- **Fix:** ship a real `rustc-wrapper.exe` (a copy of the runtime launcher or a small native shim, as the bun runner already does for `.exe` targets) instead of a `.cmd`; until then, refuse the unit cache on win32 with a bilingual message.
- **Owner:** lane-g (finished), so coordinator.

### 2. Medium: Windows tree memory is never observed, so the gate learns nothing there
- **Where:** `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts:185` (`observeTreeMegabytesV1` returns `null` for `win32`); `🎫️gate/🟦️.ts:183-185` (heartbeat publishes `observedMegabytes` only when the observation is not null); `🎫️gate/🟦️.ts:207` (`learn` only when `peak>0`).
- **Platform:** native Windows only (Linux and macOS read the tree through `/proc` and `ps`).
- **Effect:** silent. On Windows `observedMegabytes` stays `null`, the peak stays 0, no estimate sample is recorded, and every class keeps its default ratio. Admission still runs on declared requests. Nothing fails, and `capacity status` does not say it is blind. This is the "wrong result, no signal" kind.
- **Test:** the null branch is tested (`🧪️tests/📡️sampling/🟦️.ts:44`). No gate test asserts what happens on win32 (no observed memory, no learning, and a visible reason).
- **Fix:** read the Windows tree in the same PowerShell probe (`Get-CimInstance Win32_Process` with `ParentProcessId` and `WorkingSetSize`), or report `observed: unavailable` in `capacity status` and in the `[capacity]` line.
- **Owner:** ram-g (finished), so coordinator.

### 3. Medium: claim release can throw on Windows inside `finally`
- **Where:** `🗃️unit-cache/🔒️lock/🟦️.ts:39` (`if(holderOf(path)?.token===token)rmSync(path,{force:true})`, no catch). `removeStale` has the same unguarded `rmSync(grave,...)`. Callers in `finally`: `🗃️unit-cache/📜️script.ts:120`, `🏗️buildscript/🟦️.ts:78`, `🧵️lane/🟦️.ts:95`, `🧹️gc/🟦️.ts:131`.
- **Platform:** Windows. `rmSync` with `force` still throws `EPERM`/`EBUSY` when another process holds the file open (a concurrent `holderOf` read, an antivirus scan, an indexer).
- **Effect:** crash. The thrown error replaces the compile or GC result, so a successful compile reports a failure, and a stale claim may stay until it ages out.
- **Test:** none on Windows.
- **Fix:** retry `EPERM`, `EBUSY` and `EACCES` with a short backoff, as `🧹️cache-gc/🏃️runner/🟦️.ts:224` already does; `release()` should never throw.
- **Owner:** lane-g, so coordinator.

### 4. Medium: install script parity gap on Windows (concurrent first install)
- **Where:** `🚀️launcher/📥️install/🔵️.ps1:98-128` (`New-Item`, `Remove-Item -Recurse`, `Move-Item` of the Go toolchain with no lock). The POSIX twin `🚀️launcher/📥️install/🐚️.sh:84-92` has `wait_for_lock` (a `mkdir` lock with a pid owner and stale recovery). The `.sh` side changed today; the `.ps1` did not.
- **Platform:** Windows. Also a parity rule in AGENTS.md (zero-touch, all platforms).
- **Effect:** crash or corrupt toolchain directory when two first starts run at once (one `Remove-Item` can delete the other's `$Target` during its `Move-Item`). Silent on success, fatal on the race.
- **Test:** the Go launcher tests cover one download for four concurrent starts, not the install scripts; no Windows run of `🔵️.ps1`.
- **Fix:** port `wait_for_lock` to the `.ps1` (`[System.IO.Directory]::CreateDirectory` as the lock, owner pid file, stale check with `Get-Process -Id`).
- **Owner:** launcher (finished), so coordinator.

### 5. Medium: read-oracle `mirror` runs plan commands without PATHEXT resolution on Windows
- **Where:** `📥️inputs/🔭️read-oracle/🏃️run/🟦️.ts:12` (`spawn(command[0]!, ..., {detached: !windows})` with no shell). On Windows the mirror backend is the only one (`🔭️read-oracle/🟦️.ts:27`: `ORDER` has darwin and linux only).
- **Platform:** Windows.
- **Effect:** a plan command that is a `.cmd` or `.bat` shim (`npm`, `npx`, `yarn`, `pnpm`) fails with `ENOENT`, which `captureV1` maps to exit 127. The oracle then reports a mismatch or a refusal that is not a real difference. Wrong result.
- **Test:** the mirror tests run on POSIX fixtures only.
- **Fix:** resolve `command[0]` through `PATHEXT` on win32 (or refuse a non-`.exe` command with a bilingual message).
- **Owner:** keys-g / keys2-g (finished), so coordinator.

### 6. Medium-low: estimate store rename over a file another run may hold open
- **Where:** `🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts:121` (`writeFileSync(temporary,...)` then `renameSync(temporary, storePath)`, no retry).
- **Platform:** Windows (`MoveFileEx` over an open target fails with `EPERM`).
- **Effect:** silent. The `learn` port swallows the error by design, so the sample is dropped and the temporary file stays behind.
- **Test:** no EPERM or locked-target case in `🧪️tests/📐️estimates`.
- **Fix:** three retries with backoff, then remove the temporary file and log one bilingual notice.
- **Owner:** ram-g, so coordinator.

### 7. Medium-low: sampler falls back silently when the Windows probe fails
- **Where:** `📡️sampling/🟦️.ts` `readHostCountersV1`, win32 branch: `probe.run("powershell.exe", ...).catch(()=>null)` and then `freemem()` with `pressureLevel: null`, `swapUsedBytes: null`.
- **Platform:** Windows (a missing or slow PowerShell, or a 3 s bound hit).
- **Effect:** silent. Pressure and swap are unknown for that sample and the policy treats the host as normal.
- **Test:** the fallback is tested (`🧪️tests/📡️sampling/🟦️.ts:52`) but not its consequence for admission.
- **Fix:** mark the sample `counters: "partial"` in `capacity status` and in the `[capacity]` line; do not treat unknown pressure as normal.
- **Owner:** ram-g, so coordinator.

### 8. Low-medium: resident launcher falls back to a fixed `/tmp` directory
- **Where:** `🚀️launcher/📦️packages/🐹️go/🛰️resident/🐹️.go:68` (`filepath.Join("/tmp", "semio-repo-"+owner, id)` when the endpoint is over 96 bytes).
- **Platform:** Linux and macOS (not Windows).
- **Effect:** a second local user can create `/tmp/semio-repo-<uid>` first and then read or replace the key and endpoint files. Security rather than crash. The directory's owner and mode are not checked.
- **Test:** `🧪️tests/🛰️resident-contract/🐹️.go:74-79` checks the path choice only.
- **Fix:** `Lstat` the directory and require owner equal to the uid and mode `0700` (and no symlink) before use; otherwise fail with a bilingual refusal.
- **Owner:** go-g (finished), so coordinator.

### 9. Low: Bazel toolchain link uses a file symlink (Windows EPERM)
- **Where:** `🔧️tools/🏗️bazel/🏃️execution/🟦️.ts:177` (`await symlink(target, path)` with no type, for files).
- **Platform:** Windows without Developer Mode (file symlinks need the privilege).
- **Effect:** crash in the workspace generation, but only if a hermetic action reaches this point on Windows. The README says the runner refuses hermetic actions there, and `🏗️bazel/📜️script.ts:484` uses the platform profile to do it. The refusal is not a silent skip, so this is conditional.
- **Test:** the link is tested on POSIX only (`🏃️execution/🧪️tests/🟦️.ts:142-144`).
- **Fix:** on win32 copy the file as the bun runner's `shim` does, or keep the refusal and say so before any link.
- **Owner:** bzsrv-g / hermetic work (finished), so coordinator.

### 10. Low: Linux cgroup v1 CPU quota is ignored
- **Where:** `📡️sampling/🟦️.ts` `readHostCoresV1`: returns the logical core count unless `/proc/self/cgroup` has a `0::` (v2) row.
- **Platform:** Linux hosts or containers on cgroup v1 with a CPU quota (older Docker, some CI runners).
- **Effect:** wrong result, not a crash: the scheduler sizes parallel jobs for all host cores and oversubscribes the quota (slow, more memory pressure).
- **Test:** `parseCpuQuotaCoresV1` is unit-tested for v2 rows only.
- **Fix:** read `cpu.cfs_quota_us` and `cpu.cfs_period_us` from the v1 `cpu,cpuacct` controller path.
- **Owner:** ram-g, so coordinator.

### 11. Low: cancellation by SIGTERM is a no-op on Windows
- **Where:** `🏃️process/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts:33-34`, `🚀️launcher/📌️runtime-pin/⌨️command/🟦️.ts:90`, `🧷️format-stamps/🔄️reexec/🟨️.mjs` (handlers for `SIGINT` and `SIGTERM`).
- **Platform:** Windows. Node delivers only `SIGINT` (Ctrl+C) and forced termination there, so an external `SIGTERM` (for example from a supervisor) never reaches the handler.
- **Effect:** the child is not stopped gracefully. No crash; the ports stay bound until the child exits.
- **Fix:** document it, and let the supervisor use a `taskkill /T` (already used for the tree kills) when it cancels on win32.
- **Owner:** coordinator.

### 12. Low: dedupe silently does nothing where copy-on-write clones are not available
- **Where:** `🧹️cache-gc/👯️dedupe/🟦️.ts`, `cloneFileV1` (`COPYFILE_FICLONE_FORCE`, returns `false` on ext4 and NTFS, and the caller counts the file as skipped).
- **Platform:** Linux (ext4, xfs without reflink) and Windows.
- **Effect:** silent: zero replacements, no warning that the disk budget is not being helped.
- **Test:** `🧪️tests/🟦️.ts:95-100` covers the no-clone volume.
- **Fix:** print the reason once per run (`dedupe: volume cannot clone, nothing to do`), in both languages.
- **Owner:** disk-g (finished), so coordinator.

### 13. Low: POSIX hooks depend on `sh` on the Windows PATH
- **Where:** `🪝️hooks/post-checkout`, `post-commit`, `post-merge`, `post-rewrite`, `prepare-commit-msg` (`#!/bin/sh`, `sed`, `command -v`, `rm -f`), and the generated commit-hook text in `🎛️commands/📝️commit/🪝️hooks/🟦️.ts`.
- **Platform:** Windows without Git for Windows' `sh.exe` on PATH (an IDE or a cmd session that runs hooks directly).
- **Effect:** the hook fails silently (`2>/dev/null ... || true` swallows errors).
- **Test:** none on Windows.
- **Fix:** keep them, and let `repo setup` on win32 check that `sh` resolves (one bilingual line), or write the hook body as a call to the `repo` binary.
- **Owner:** coordinator.

### 14. Info: devcontainer and native directory exchange
- **Where:** `.devcontainer/Dockerfile:24` (installs `lsof`, used by `💾️cache/💾️volume/🟦️.ts:45` on linux too, so consistent) and `📁️directory/🔀️exchange/🟦️.ts:33,45` (a kernel `RENAME_EXCHANGE` that is refused when the library or errno does not offer it).
- **Platform:** devcontainer on Docker Desktop (macOS and Windows hosts, bind mounts through virtiofs or 9p), where the atomic exchange may be unsupported.
- **Effect:** a refusal naming the cause, not a silent wrong result. The README marks the devcontainer row as not yet verified.
- **Fix:** run one live `repo` native-store check inside the devcontainer on Docker Desktop before calling it supported.
- **Owner:** store-g (finished), so coordinator.

## Checked and clean

- **macOS-only tools are gated.** `seatbelt` is in the read-oracle ORDER table for darwin only (`🔭️read-oracle/🟦️.ts:27`), the `sandbox-exec` probe is darwin-only, `hdiutil` is used only inside the macOS image modules (`🍎️image/🛠️hdiutil`), the `vm_stat` and `sysctl` calls sit in the darwin branch of `readHostCountersV1`, and `lsof` runs only on darwin or linux (`💾️volume/🟦️.ts:45`).
- **`/proc` is read only on Linux.** The sampler, `observeTreeMegabytesV1` (linux branch), and the Rust admission `resident_bytes` (`#[cfg(all(unix, not(target_os = "macos")))]`, macOS uses `proc_pid_rusage`).
- **Process trees.** Every POSIX group kill (`process.kill(-pid)`) that I read is paired with a `detached` spawn: bench runner, read-oracle, remote-execution worker, and the Bun runner (worker mode). Windows uses `taskkill /T /F` in each of them. The cargo command's tree kill (`🗂️workspaces/🦀️cargo/🏃️command`) was not read.
- **Launcher build tags.** `windows-host`, `resident-windows`, `resident-unix`, and `unix-host` are split by `//go:build`, and `syscall.Exec` is only in the unix host.
- **Install script hashing.** `install.sh` falls back from `sha256sum` to `shasum -a 256`; the `.ps1` twin uses `Get-FileHash` (line 111).
- **Bazel server stop** removes the pid file (portable), not a signal.
- **Path separators.** The `split("/")` calls in changed code (for example `🏛️sealed/📤️promotion/🟦️.ts:158`) split sealed or project-relative paths, which are `/`-separated by contract; no backslash hazard found.
- **Case.** The ledger key `portable()` (`🧾️ledger/🟦️.ts:42`, NFC plus lower case) folds case on purpose, so it is conservative on case-sensitive Linux too.
- **Hard links** in the unit-cache lock (`linkSync` of the lock grave, same directory) fail safely (`try`/`catch`, the grave is removed).
- **Remote-execution entry** handles `SIGTERM` (`🏃️entry/🟦️.ts:5`), so the non-detached `child.kill("SIGTERM")` in `⌨️command/⚙️runtime/🟦️.ts:58` is fine.

## Not verified / follow-ups

- Items 1, 2, 3, 5 and 9 are static findings. Each needs one run on native Windows (a `RUSTC_WRAPPER` compile for item 1, a gate run with a fixture tree for item 2) before a fix is merged.
- The sweep covered all 587 files by pattern, but only the flagged modules were read in full. Unflagged changed files (for example the Go test-runner) were not read line by line.
- Item 4 needs the same Windows check as item 1.
