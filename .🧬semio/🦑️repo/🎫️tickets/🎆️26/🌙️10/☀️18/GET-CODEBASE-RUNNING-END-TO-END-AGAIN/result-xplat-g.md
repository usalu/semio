# result-xplat-g (executor xplat-g, wave 1026g)

Tools `C/xplat-g-tools/`, output `C/🗑️generated/xplat-g/`. Input: `C/audit-xplat-1026g.md` (14 findings). Nothing was written to `🧰️framework/**` (measurement window 10:30-11:30, brief addendum 10:10): every edit is STAGED, tested in a scratchpad overlay, and applied after 11:30 in one batch.

## FOR lines
- FOR go-g: finding 4 (`install.ps1` has no lock where `install.sh` has `wait_for_lock`) is yours, I did not touch `📥️install/`. Port: `[System.IO.Directory]::CreateDirectory` as the lock directory, owner pid file, stale check with `Get-Process -Id`, same bounds as `wait_for_lock`. Finding 8 (resident `/tmp` fallback) is in your Go package (`🛰️resident/🐹️.go:68`); see "Finding 8" below for what I staged.
- FOR store-g: finding 14 (devcontainer on Docker Desktop: `RENAME_EXCHANGE` over virtiofs/9p, `lsof` is installed in the Dockerfile) is native storage: one live `repo` native-store check inside the devcontainer before the README row says "supported". Not touched.
- FOR coordinator: apply with `sh C/xplat-g-tools/apply.sh` after 11:30 (see "Apply" below).

## Status (newest first)
- 10:50 DONE (xplat-g). 12 of 14 findings fixed at the source and tested (all STAGED, nothing in the tree yet); 4 and 14 are FOR lines (go-g, store-g). `sh C/xplat-g-tools/apply.sh` (dry run against the CURRENT tree at 10:50: clean, 47 files, 0 rejects) then `sh C/xplat-g-tools/verify.sh`. Type check of the changed modules clean (`tsconfig.xplat-g.json`, run in the overlay).

## Method
- Overlay = scratchpad clone of `🧰️framework/🔨️modules`, the build library and the launcher (`ov`) beside a pristine copy (`ov-base`); `C/xplat-g-tools/snapshot.sh` writes `staged/xplat-g.patch` (`diff -ruN`, apply with `patch -p1` from the repo root after rewriting the two prefixes, see `apply.sh`). New files are in the patch (`-N`).
- Windows/Linux facts are injected (platform argument, command runner, file reader, clock/sleep) and read from fixtures; where the behaviour is portable it runs for real on macOS (a native wrapper is compiled and executed, a real read-only directory gives a real `EACCES`, a real hard link is made).
- Independent oracles: Python echo for the wrapper argument vectors, Python tree sum for the Windows process table. No third-party oracle exists for the PATHEXT lookup (documented `cmd.exe` rule), the cgroup v1 file names (kernel doc) and the hard-link rule; the vectors are authored from the documented rules.

## Findings -> fix -> test

### 1 (High) unit-cache `RUSTC_WRAPPER` was a `.cmd` on Windows
- Cause confirmed: Cargo runs the wrapper directly with quoted arguments (`--cfg feature="std"`); Rust refuses such arguments for batch files (CVE-2024-24576 hardening).
- Fix (`⚡️caching/🦀️cargo/🗃️unit-cache`): on win32 the stand-in is a NATIVE executable built with `bun build --compile` from the new entry `🏁️entry.ts` (about 0.25 s, 64 MB), named by the sha256 of the bundled code (`rustc-wrapper-<digest>.exe`), compiled once, atomically renamed, older executables removed. `📜️script.ts` got `runUnitCacheMainV1` (shared by the script and the executable; a compiled executable has no `argv[1]` the main-module test could match). Decision: a native compile, not "the scheduler binary directly", because the wrapper is TypeScript (cache lookup, claims, slots) and the scheduler is only its inner compiler; a native executable keeps one implementation on every host. If the bundler/compiler fails, `unitCacheEnvironmentV1` returns `RUSTC_WRAPPER=""` (empty overrides the configured wrapper), `SEMIO_UNIT_CACHE=off` and `SEMIO_UNIT_CACHE_REFUSAL` (EN+DE, new schema message `wrapperUnavailable`); `compilerLane` prints it once. `.cmd` support and `unitCacheShimSourceV1`'s platform argument are removed (no compat layer).
- Tests `🧪️tests/🪟️windows/🟦️.ts` (5, pass, 2.2 s): the executable is built FOR REAL and run like Cargo runs it with the fixture argument vectors (quotes, spaces, backslashes, unicode, empty argument) against the inner command; the output equals the independent Python echo; exit statuses 0/1/101 propagate; digest naming (no recompile for unchanged code, old executable removed); failing build gives the bilingual refusal and `RUSTC_WRAPPER=""`. Existing `unit-cache/🧪️tests/🟦️.ts` 23/23 and `🛤️lane/🧪️tests` 3/3 pass in the overlay (one assertion adapted: `.cmd` -> `.exe`).
- Not verified on a real Windows host (compile of the executable and a Cargo run through it): needs one run on native Windows.

### 2 (Medium) Windows tree memory was never observed
- Fix (`🏃️process/🏗️build/🌡️host-capacity/📡️sampling`): `observeTreeMegabytesV1` on win32 runs one PowerShell process table (`WINDOWS_PROCESS_TABLE_COMMAND`: `Get-CimInstance Win32_Process`, `pid ppid working-set-KB`, the shape of `ps -A -o pid=,ppid=,rss=`) through the same parser and tree sum as macOS. If PowerShell cannot answer it returns null AND says so once per process in EN+DE (new messages `treeUnobserved`); the gate then learns nothing from the run (existing rule) and keeps the declared sizes.
- Tests `🧪️tests/🪟️windows/🟦️.ts`: fixture process table with CRLF and blank lines, tree sums for 5 roots equal the Python oracle, the exact command line is asserted, failure gives two lines (EN/DE) once even over 3 heartbeats, and the REAL gate (`withHostCapacityV1`) with the win32 observer learns peak 75 MB from the table and learns nothing (2 notice lines) when the probe fails.

### 3 (Medium) claim `release()` could throw on Windows in `finally`
- Fix: `settleBusyFilesystemSyncV1` / `filesystemBusyErrorV1` in `🔁️churn/🟦️.ts` (the churn module of churn-g): busy = the closed race table's `busy` kind for the platform (EBUSY, EPERM, EACCES on win32), retried on the table's `read` pauses (10, 40, 160, 640, 2560 ms, 6 attempts), then returned as `settled:false` and reported once in EN+DE (the churn exhaustion text). Never throws. `🔒️lock/🟦️.ts` got `removeClaimFileV1` and uses it in `release()`, `removeStale` (grave) and both ticket removals of `acquireSlotV1`; the callers in `finally` (`📜️script.ts`, `🏗️buildscript`, `🧵️lane`, `🧹️gc`) are covered because `release()` no longer throws.
- Tests: `🔁️churn/🧪️tests/🔒️busy` (5): code classification per platform, retries on the declared pauses, exhaustion report (one line per language), a non-busy error is final, and a REAL `EACCES` from a read-only directory is final under POSIX rules and retried under injected win32 rules. `unit-cache/🧪️tests/🪟️windows`: `claim.release()` on a read-only directory does not throw (it threw before), retry pauses equal the protocol, the report names the file.

### 4 install.ps1 lock: go-g (FOR line above).

### 5 (Medium) read-oracle mirror: no PATHEXT on Windows
- Fix (`📥️inputs/🔭️read-oracle/🏃️run`): `resolveCommandV1` resolves the command like the shell (every `PATH` directory, case-insensitive key, each `PATHEXT` extension, first hit wins). `.exe`/`.com` start directly; a `.cmd`/`.bat` shim (`npm`, `yarn`, `pnpm`) cannot take arbitrary arguments safely without a shell, so `captureV1` ends with exit 127 and a bilingual reason (new message `batchCommand`, schema updated) instead of an `ENOENT` that reads like a difference between the two runs. Other hosts: unchanged.
- Tests `🧪️tests/🪟️windows` (3): 7 fixture vectors (shim refused, exe direct, first directory decides, PATHEXT order, explicit extension, absolute name, unknown name), POSIX hosts unchanged, `captureV1` refusal path. Whole read-oracle suite 12/12 in the overlay.

### 6 (Medium-low) estimate store rename over an open file
- Fix (`📐️estimates/🟦️.ts`): the rename goes through `settleBusyFilesystemSyncV1` (platform/sleep/rename ports for tests); a store that stays busy drops the sample, REMOVES the temporary file and reports once in EN+DE.
- Test in `🧪️tests/📐️estimates/🟦️.ts` (14/14): two EPERM then success on the declared pauses; permanent EBUSY returns false, leaves no `.tmp`, one line per language, store untouched; on POSIX a permission refusal is final.

### 7 (Medium-low) sampler fell back silently when the Windows probe failed
- Fix: the fallback counters carry `partial:true`; `reduceHostSampleV1` marks the sample `counters:"partial"` (optional property of the closed `Host` definition, absent on complete samples so every stored fixture is unchanged) and raises the pressure to at least `warn` (unknown pressure is never normal); a one-time EN+DE line (`countersPartial`); `repo capacity status` prints the same line in the selected language, `--json` carries the marker.
- Tests: host-capacity `🪟️windows` (partial counters, `warn`, schema valid, `counters:"full"` invalid, a complete sample has no `counters` key) and library `host-capacity/🧪️tests/🪟️partial` (status line EN/DE, JSON marker, complete sample prints one line). Existing sampling 9/9, gate, root host-capacity 59/59.

### 8 (Low-medium) resident `/tmp` fallback
- Reality check: the Go client already refused a leaf directory that is not owned by the caller or not private (`residentTrusted`); the gap is the PARENT `semio-repo-<uid>`, created by whoever comes first under the shared temporary directory, which could swap the private leaf after the check. Fix (`🚀️launcher/📦️packages/🐹️go/🛰️resident-unix`): the parent must be a real directory of the caller that nobody else can write (`perm & 0o022 == 0`, deliberately not 0o077 so a service that creates it 0755 keeps working); a link anywhere is refused.
- Test `TestResidentTrustedDirectoryAndParent` (private, read-only-for-others parent, world/group-writable parent, open leaf, link as parent, link as leaf, missing). Verified with `go vet` + `go test` + `gofmt -l` in a standalone module holding the unix member and the test (the launcher package needs the whole go.work); run the launcher's Go test target after the apply. RISK for go-g: if the service creates the parent group-writable (umask 002 with a mode-less mkdir) the client now says "down"; the service should create it 0700.

### 9 (Low) Bazel toolchain link used a file symlink
- Fix (`🔧️tools/🏗️bazel/🏃️execution/🟦️.ts`): on win32 `linkBazelToolchainsV1` makes a HARD LINK (no privilege, same volume) and copies when the volumes differ; an entry that is the same inode or an up-to-date copy is kept; a symlink of an older run is replaced. POSIX unchanged.
- Test `🏃️execution/🧪️tests/🪟️toolchain-link` (3): real hard link/copy under injected win32, current entry kept, stale copy refreshed, POSIX symlink. NOTE: runs here only through an extracted copy of the function (the execution module's import closure needs a repo path under 256 characters, the overlay path is longer); run it in the tree after the apply.

### 10 (Low) cgroup v1 CPU quota ignored
- Fix: `cgroupV1CpuFilesV1` + `parseCfsQuotaCoresV1` read `cpu.cfs_quota_us` / `cpu.cfs_period_us` along the cgroup chain (`/sys/fs/cgroup/cpu,cpuacct` or `/cpu`), `-1` is unlimited, the tightest quota in whole cores bounds `readHostCoresV1`; v2 path unchanged.
- Test (host-capacity `🪟️windows`): 5 fixture hosts (2 cores nested, ceil 1.5 -> 2, unlimited, quota above host cores, no cpu row) and the file-name chain.

### 11 (Low) SIGTERM is a no-op on Windows
- Fix: `terminationSignalsV1` / `onTerminationV1` in `⚙️runtime/🏁️main` (facade re-exports updated): win32 listens to `SIGINT`, `SIGTERM`, `SIGBREAK`, `SIGHUP` (the console signals Node delivers; `taskkill` without `/F` posts the close event). Used by the read-oracle CLI, the runtime-pin `runInheritedV1` (launcher) and the format-stamps re-exec (`.mjs`, inline list). A supervisor still cancels with `taskkill /T` where it needs the tree (already used).
- Tests `⚙️runtime/🧪️tests/🛑️termination` (3): platform sets, all four Windows signals once and the disposer, a REAL child stopped by SIGTERM through the handler (POSIX).

### 12 (Low) dedupe on volumes without clones
- Reality check: a per-file bilingual `dedupe.unsupported` line existed, but only after the full hashing census and once per file. Fix: `volumeClonesV1` probes the volume once before the census; no clone support prints one bilingual line (`dedupe.noClone`, says the disk budget then rests on garbage collection alone) and returns an empty outcome without hashing. Dry runs are unchanged.
- Test `👯️dedupe/🧪️tests/🟦️.ts` (8/8): the old "cannot clone" test now asserts one line, no census progress, EN and DE, no probe file left, tree untouched.

### 13 (Low) POSIX hooks depend on `sh` on the Windows PATH
- Fix (`🎛️commands/📝️commit/🪝️hooks`, called by `repo setup` on every run): `resolveHookShellV1` finds `sh.exe` (PATH, case-insensitive key, then Git for Windows in Program Files, Program Files (x86), the per-user install); `reportHookShellV1` says nothing when it is on PATH, an info line when only Git for Windows' own shell exists (git runs the hooks, an IDE or `cmd` session that starts the files directly skips them silently because the hooks swallow errors), and a warning when none exists. EN+DE through the commit catalog (`hooks.shell-bundled`, `hooks.shell-missing`). The hook bodies are unchanged (the shebang is enough on POSIX).
- Test `🪝️hooks/🧪️tests` (3): 4 Windows host vectors (fixture), POSIX hosts need nothing, report levels, both messages exist in both languages. Verified through an extracted copy of the two functions (the catalog import chain needs a repo path under 256 characters); the real test runs in the tree after the apply.

### 14 (Info) devcontainer and native directory exchange: store-g (FOR line above). No change.

## Apply and verify (coordinator, after 11:30)
1. `sh C/xplat-g-tools/apply.sh` (refuses inside 10:30-11:30; dry run first; all or nothing; patch is relative to the repo root, `patch -p1`). A peer edit on a patched line makes the dry run fail with the file named: re-run `snapshot.sh` is not possible after the scratchpad is wiped, so then apply the other hunks by hand from the patch.
2. `sh C/xplat-g-tools/verify.sh`: 17 single-file graph-free `bun test` runs (pass/FAIL per file, logs in `C/🗑️generated/xplat-g/`, delete with the ticket sweep). In the overlay 14 of them passed; 4 cannot load there (the overlay path is longer than the 256-character source path bound of the revision reader, so `bazel/execution` tests, hooks test, library host-capacity root test and the `setup` module were checked another way: extracted functions / type check): `bazel toolchain-link` 3/3 and `hooks` 3/3 through extracted copies, library host-capacity root 15/16 with the one failure being a path-length fault in the overlay only.
3. The edit set is compile-atomic by construction: the new pieces (`🏁️entry.ts`, churn `settleBusyFilesystemSyncV1`, `terminationSignalsV1`, the schema messages) arrive in the same patch as their call sites. Schema edits: host-capacity `Host.counters` (optional) and two messages, unit-cache `wrapperUnavailable`, read-oracle `batchCommand`, commit `hooks.shell-*`, ledger `dedupe.noClone`. Graph effect: the patch touches ~25 library/process/launcher files, so the next graph generation re-infers as after any build-library edit.

## Not verified / honest limits
- Nothing ran on a real Windows host: finding 1 (the compiled executable run by Cargo), 2 (PowerShell `Win32_Process` output shape: I assumed one `pid ppid KB` line per process exactly as the command formats it), 3, 5, 9 need one native Windows run each; the logic is proven with injected platforms and real POSIX equivalents.
- The unit-cache wrapper executable is ~64 MB per code version (old ones are deleted); bundling costs ~50 ms per Cargo start on macOS (not measured on Windows).
- Finding 11 is a mitigation (console signals), a Windows supervisor that needs the whole tree still uses `taskkill /T`.
- No performance claim is made; no official or graph-loading run was started; no process of mine is running; overlay and baseline are in the session scratchpad (`ov`, `ov-base`; removable, the patch is the deliverable).
