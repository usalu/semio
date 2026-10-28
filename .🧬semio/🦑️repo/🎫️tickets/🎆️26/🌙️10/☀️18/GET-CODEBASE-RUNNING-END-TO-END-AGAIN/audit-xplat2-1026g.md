# 🧭️ Cross-platform audit 2 of the wave 1026g changes (aud-xplat2-g, read-only)

Auditor: aud-xplat2-g, 18:10-18:40 CET. Read-only: no source file edited, no modifying git command, nothing built, no graph-loading run, no sub-agent. Temporary lists and probes are in `🗑️generated/aud-xplat2-g/` (swept with the ticket). Companion of `audit-xplat-1026g.md` (10:19, same method).

## Scope and method

- **Lanes.** repair-g, gram-g, gram2-g, derive-g, phase-g, preadmit-g, lane2-g, nxret-g, nodeffi-g, wrap-g, walk-g, testmem-g, testmem2-g, nxfix-g, laws-g, ci-g, obs-g, store-g, keys-g, keys2-g, keys3-g, undeclared-g, rev3-g (+ rev3fix-g).
- **Files.** Every source path named in those result files, intersected with the files modified after 10:20 today (`🗑️generated/aud-xplat2-g/lane-map.txt`): 100 files, 61 of them production code. The framework tree holds 477 files changed since 10:20 in total; the rest belong to lanes outside this list, or are staged batches that are not live. A lane file is attributed to every lane whose result file names it, so the owner column is "named by", not exact authorship.
- **Sweep.** 426 hits over the production and test code for: `process.kill`, signals, `/tmp`, `TMPDIR`, separators, `.exe/.cmd/.bat/.ps1`, `PATHEXT`, `spawn*("sh"|"ps"|"rustc"|"git"|"python3")`, symlinks, `realpath`, `rename*`, `statfs`, `linkSync`, `sysctl`, `/proc`, `detached`, `chmod`, `taskkill`. Each production hit read in context; for tests, gating (`skipIf`, `posix`, `available`, `win32`) checked in the same file.
- **Live versus staged.** Only the live tree was read. Staged batches (`walk-g-tools/staged/footprint`, `nodeffi`/`undeclared-g` batches, `testmem2-g/staged`, `ci-g/staged/files`, `rev3-g/staged`, `obs-g` workbench) are named where relevant and are not yet findings against the tree.
- **Probe run.** The POSIX stand-in of the unit cache was generated from the live module (`unitCacheShimSourceV1`) and run under `/bin/dash` (Linux shell) and macOS `/bin/sh` with descriptors 3 and 5 open, only 3, none, and a `fifo:` jobserver. Results identical on both shells: `3,5` only when both descriptors are open, `none` otherwise (`🗑️generated/aud-xplat2-g/shimtest.sh`). This is shell semantics, not a Linux kernel run.

## Status of the morning findings (audit-xplat-1026g.md)

| # | Morning finding | Now |
|---|---|---|
| 1 | unit-cache wrapper is `.cmd` on Windows | RESOLVED: native `rustc-wrapper-<digest>.exe`, `🚪️launch/🟦️.ts:29-30`, `:76` |
| 2 | Windows tree memory never observed | RESOLVED: one PowerShell process table, `📡️sampling/🟦️.ts:194`, `:225-227`; failure announced once |
| 3 | claim release throws on Windows | RESOLVED: `removeClaimFileV1` settles busy files, `🗃️unit-cache/🔒️lock/🟦️.ts:15-16` |
| 4 | `.ps1` installer races on first start | PARTLY: named mutex now guards the Go install, `🚀️launcher/📥️install/🔵️.ps1:99-108`; see F5 |
| 5 | mirror runs `.cmd` without PATHEXT | RESOLVED: `resolveCommandV1` (PATHEXT, `.cmd` refused bilingually), `📥️inputs/🔭️read-oracle/🏃️run/🟦️.ts:16-30` |
| 6 | estimate store rename without retry | RESOLVED: `settleBusyFilesystemSyncV1`, `📐️estimates/🟦️.ts:125` |
| 7 | sampler silent fallback on Windows | RESOLVED: `partial: true` reaches the policy, `📡️sampling/🟦️.ts:109`, `:160` |
| 8 | resident launcher `/tmp` fallback | NOT RE-CHECKED (Go file, not in this scope) |
| 9 | Bazel toolchain file symlink on Windows | NOT RE-CHECKED (outside scope) |
| 10 | cgroup v1 CPU quota ignored | RESOLVED: `parseCfsQuotaCoresV1`, `📡️sampling/🟦️.ts:89`, used at `:99` |
| 11 | SIGTERM no-op on Windows | OPEN, unchanged (read-oracle CLI, runtime-pin, proctor); outside this scope |
| 12 | dedupe silently does nothing without clone | OPEN, unchanged (see F8 for the related silent-zero) |
| 13 | POSIX hooks need `sh` on Windows | OPEN, unchanged |
| 14 | devcontainer exchange unverified | OPEN, unchanged |

## Findings, ranked (new in this audit)

Severity as in the morning audit: **High** = crash or wrong result on a supported platform by default; **Medium** = silent wrong or skipped behaviour, or a red test on a platform the CI does not block on; **Low** = conditional or coverage gap.

### F1. Medium (test red on Windows, hidden by advisory CI): witness test writes to hard-coded `/tmp`
- **Where:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧪️tests/🟦️.ts:79` (`writeFileSync("/tmp/semio-witness-outside-"+process.pid,...)`), `:79` (`new Database("/tmp/semio-witness-db-"...)`), `:88` (`rmSync("/tmp/...")`).
- **Platform:** native Windows (and any host where `/tmp` is absent). On Windows `/tmp` resolves to `C:\tmp`.
- **Effect:** the sneaky child throws ENOENT before it reports; the expectations for "wrote outside the declared places" and "opened a database outside" fail. Test-only, no production effect.
- **Coverage:** the test is not gated; no Windows run of it is blocking (see F3).
- **Fix:** use `join(tmpdir(), ...)` for both paths (the same root the test already creates for `base`).
- **Owner:** witness lane (named in `result-laws-g.md`, `result-nodeffi-g.md`); coordinator (lane finished).

### F2. Medium (coverage gap, silent skip): the Node native provider has no Windows test
- **Where:** `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🟦️.ts:14` (`const posix = process.platform !== "win32"`), `:48` (`const available = posix && (darwin || linux)`), and every runtime test behind `test.skipIf(!available)` (`:95`-`:173`). Production: `🧲️native/🦀️.rs:16` (`raw-dylib` against `node.exe`), `:24`, `:47`, `:103`, `:145`, `:188` (`#[cfg(windows)]` paths), `🏗️addon/🟦️.ts:79` (`win32` `.exe` compiler name), and the cache root and targets in `🧬️schema/🔣️.json` (`cacheRoots.win32` = `LOCALAPPDATA`, the `pc-windows-msvc` targets).
- **Platform:** native Windows.
- **Effect:** silent skip. The only Windows coverage is the compile-only test at `:68` and `:77`; the cdylib built for `x86_64-pc-windows-msvc` is never loaded, the `raw-dylib` import of `node.exe` symbols and `LOCALAPPDATA` cache placement are never exercised. A broken Windows core would pass CI as "skipped".
- **Fix:** add platform-parameterised pure tests (cache root and plan for `win32` with an injected host, the compile arguments for both Windows targets), and a Windows load test that fails instead of skipping when the Node core is the designated one. Until then, say "unverified on Windows" in the README.
- **Owner:** nodeffi-g (finished); coordinator.

### F3. Medium (CI): Windows suite rows are advisory, so Windows failures never block
- **Where:** `.github/workflows/build-system.yml:360` (`continue-on-error: ${{ matrix.advisory }}`), rows `:379`-`:385` (`advisory: true`, all seven Windows rows of the `suites` job: process 1-2, caching 1-3, build 1-2). The `host` job (`:241`-`:295`) is blocking on Windows.
- **Platform:** CI on `windows-latest`.
- **Effect:** silent for the 220 suites behind those rows: a red Windows suite leaves the workflow green. The reason is documented in `result-ci-g.md:55` (POSIX-only suites carry no `platforms` tag yet).
- **Coverage:** the tag mechanism exists in the report; no suite carries it yet.
- **Fix:** tag the POSIX-only suites with `platforms` now (the report lists them), then set `advisory: false` on the Windows rows so only the tagged set is excluded. Owner decision: coordinator.

### F4. Medium-low (Windows runtime): rename over a possibly open file without retry
- **Where:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:789` (`fs.renameSync(temporary, receiptPath)`), `:755` (`fs.renameSync(temporary,cachePath)` inside `withBuildGraphOptionalPublicationV1`); `🏗️build/📥️inputs/🟨️.mjs:114` (`renameSync(temporary, target)`), `:76` (`renameSync(temporary,path)`).
- **Platform:** native Windows. `MoveFileEx` over a file another process has open fails with EPERM (the morning audit's item 6 class).
- **Effect:** crash of the observation or the cache publish on a reader race. Line 755 is inside an optional publication, so the cost is a failed publish; line 789 is a receipt, so the run fails.
- **Coverage:** none on Windows.
- **Fix:** route these renames through `settleBusyFilesystemSyncV1` (the helper `📐️estimates` and the lock already use). Owner: obs/gram lanes (finished); coordinator.

### F5. Low-medium (Windows, unverified): named mutex in the PowerShell installer uses the `Global\` namespace
- **Where:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🔵️.ps1:99` (`[Threading.Mutex]::new($false, "Global\semio-go-$version")`), `:103`, `:106`, `:133`.
- **Platform:** native Windows, a non-administrator account on Windows 10 or 11.
- **Effect (if it fails):** the mutex creation throws, the install stops with a PowerShell error. Creating a `Global\` object from a session other than 0 may need `SeCreateGlobalPrivilege`. Not reproduced here.
- **Fix:** use `Local\semio-go-$version`. The install directory is per user, so a per-session mutex is enough. Verify on a standard account before landing.
- **Owner:** launcher (finished); coordinator.

### F6. Low (Windows test, dev machines without Developer Mode): symlinks and a `libc.so.6` load in ungated tests
- **Where:** `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts:105-106` (`symlinkSync("../a", ...)`, `symlinkSync("nowhere", ...)`), `:267` (`symlinkSync(join(root,"probe.ts"), ...)`), `:280-281` (`process.platform === "darwin" ? "/usr/lib/libSystem.B.dylib" : "libc.so.6"`, gated only by `host !== "bun"`).
- **Platform:** Windows without Developer Mode (symlinks, EPERM); Windows with Bun (the `libc.so.6` name does not exist).
- **Effect:** test red, no production effect. GitHub's Windows runners are administrator and may pass the symlink lines; the FFI line is wrong on every Windows host.
- **Fix:** gate `:105`, `:267` with `symlinkSupportedV1()` (try once, skip on EPERM), and `:280` with `process.platform === "win32" ? skip`.
- **Owner:** runtime test lanes (finished); coordinator.

### F7. Low (silent wrong result on a failed `statfs`): "healthy" fallbacks
- **Where:** `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:543` (`leaseStoreFreeRatioV1`: `catch{return 1;}`, i.e. a store that cannot be measured counts as fully free), `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts:50` (`free()` returns 0 on failure, so `gained` in `runDedupeV1` stays 0 silently; `:59`).
- **Platform:** any platform where `statfsSync` throws or is missing (Bun on Windows is not verified; the call is in ~25 places, `grep statfsSync`).
- **Effect:** admission and reporting act on a number that means "unknown". `leaseStoreFreeRatioV1` = 1 can admit work into a full disk; the dedupe reports zero gain without saying why.
- **Fix:** return `undefined` for "unknown" and let the caller refuse or print `dedupe: free space unknown` (bilingual). Verify `statfsSync` on Bun win32 once with a native probe.
- **Owner:** lease and cache-gc lanes (finished); coordinator.

### F8. Low-medium (coverage, Linux and Windows): the kernel read oracle is macOS-only in practice
- **Where:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🟦️.ts:27` (`ORDER`: darwin `seatbelt, mirror`; linux `landlock, mirror`; win32 `mirror`), live `🐧️linux/🐍️.py` (63 lines, landlock launcher) and `🐧️linux/🟦️.ts`.
- **Platform:** Linux (landlock) and Windows (mirror only).
- **Effect:** silent weaker check, not a wrong result: `undeclared-g` names Linux landlock "not run" and Windows "output equivalence only". A wrong cache hit caused by an undeclared read is therefore caught only on macOS. The fallback Seatbelt to mirror is announced (`fallback` progress, `:81`) and each round records its backend, so it is visible.
- **Coverage:** the corpus replay law and the hybrid law are macOS-only.
- **Fix:** run the landlock law once in the devcontainer (Docker Desktop may refuse Landlock; then the reason is printed and mirror runs). Record the enforced backend in the product report so a mirror result is never read as kernel-enforced.
- **Owner:** undeclared-g (finished, batch staged in `undeclared-g-tools/batch/`); coordinator.

### F9. Low (test-only, POSIX shell in a Windows-run test): `sh -c` in the native parity test
- **Where:** `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🟦️.ts:31` (`spawnSync("sh", ["-c", ...])`), `:134` (`sh -c 'test -e ...'` case probe), `:123` (`ps -o pgid=`).
- **Platform:** Windows.
- **Effect:** none today (all three sit behind `available`, F2), but the day F2 is fixed these lines fail on Windows. Note it in the same change.
- **Fix:** replace the case probe with `fs.existsSync` on both spellings, and the `ps` call with the already-present native pgid; or keep them behind `posix`.
- **Owner:** nodeffi-g; coordinator.

## Checked and clean (this audit)

- **POSIX stand-in of the unit cache (wrap-g).** `🗃️unit-cache/🚪️launch/🟦️.ts:33-40` runs under `/bin/dash` and macOS `/bin/sh` with the same results (probe above). The `(: <&N) 2>/dev/null` test does not print on either shell, and `exec` keeps the descriptors open for the compiler. The Windows path never sets `SEMIO_UNIT_CACHE_JOBSERVER_FDS`; `📜️script.ts:37-40` leaves stdio as before when the list is empty.
- **Descriptor pass-through (wrap-g).** `📜️script.ts:37` maps only the verified pair; no `fstat` guess left. Known gap, unchanged: build scripts under the build-script cache draw no jobserver tokens (wrap-g says so).
- **Process-group kills.** The only `process.kill(-pid)` in the changed code (`📥️inputs/🔭️read-oracle/🏃️run/🟦️.ts:46`) is paired with `detached: !windows` (`:38`); Windows uses `taskkill /T /F` (`:45`). The launcher's `runInheritedV1` handles `SIGBREAK` on win32 only (`🚀️launcher/📌️runtime-pin/⌨️command/🟦️.ts:90`).
- **macOS-only tools are gated.** `sysctl` via `bun:ffi` `libSystem.B.dylib` (`🚀️launcher/📌️runtime-pin/⌨️command/🟦️.ts:45-49`) sits under the darwin branch; `/proc/<pid>/cmdline` (`:42`) under linux; `sandbox-exec` only under darwin (`🍎️seatbelt/🟦️.ts:41`); `footprint` is in staged walk-g code only.
- **Node native build.** The Xcode-license retry is darwin-only (`🏗️addon/🟦️.ts:96`); `linkArguments` are per target (`dynamic_lookup` on darwin, none on Windows); `.exe` is added on win32 only (`:79`); `path` length is checked against the declared 256 (`🧬️schema` `maximumPathCharacters`).
- **Path length (AGENTS.md 256).** Staging path of the unit-cache lane is checked (`📥️inputs/🟨️.mjs:76`).
- **Separators.** Declared-pattern and sealed-path `split("/")` calls operate on `/`-separated contract paths (for example `🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:154`, the declared-input parser).
- **Test gating.** `witness` and `native` tests were the only changed tests with unguarded POSIX-only effects besides F1, F6; the store-receipt `#!/bin/sh` is fixture content, never executed.

## Not verified

- Every Windows finding is static. F1, F2, F5 need one native Windows run (a `bun test` of the witness file, a `🔵️.ps1` run from a standard account, a Node native build on `x86_64-pc-windows-msvc`).
- F7's Bun-on-Windows `statfsSync` support is not checked here.
- Staged batches (walk-g footprint: `macOS footprint`, Linux `smaps_rollup`, Windows PowerShell private working sets; `ci-g` staged `ci-report`; `obs-g` `repo fleet status`) are not in the live tree and were not audited line by line. Before landing, the footprint batch's Windows command needs the same single-run check as F2.
- The rest of the 100 mapped files were pattern-swept and read where flagged; not every line of the scheduler, graph and gate changes was read.
