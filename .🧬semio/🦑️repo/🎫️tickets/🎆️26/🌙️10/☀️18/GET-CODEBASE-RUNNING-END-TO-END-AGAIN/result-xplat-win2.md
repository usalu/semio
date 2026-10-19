# Result xplat-win2 (2026-10-19)

Scope: Windows adopts the directory partition backend (no VHD, no elevation), shared platform-neutral directory core with swap ports, funded child storage on the directory backend for Linux and Windows, capacity bypass fix (coordinator message), stale tests (`creatorPending`, `nativeImageUsedBytesV1`, retirement test at `:207`). Abbreviations: `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`, `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Nothing ran on Windows; every Windows path is verified through a recorded call table (the FFI call-sequence port) and a real-filesystem simulation of the host on macOS.

## Design (schema-first, one core, two ports)

`NB/📁️directory/` (renamed from `🐧️directory`, importers re-pointed) is the platform-neutral core.

| member | role |
|---|---|
| `📋️contract/` | types, `Protocol` loader, identity observation, `NativeDirectoryPlatformPortV1` (`swap`, `flush`, `pending`, `recover`, `prepareVolume`) |
| `🔀️exchange/` | POSIX port: one `renameat2(RENAME_EXCHANGE)` / `renameatx_np(RENAME_SWAP)` call through `bun:ffi` |
| `🪟️host/` | Windows FFI call table (`MoveFileExW`, `CreateFileW`, `FlushFileBuffers`, `CloseHandle`, `NtSetInformationFile` class 71, `GetDiskFreeSpaceExW`, `RegGetValueW`), `\\?\` extended paths, UTF-16 encoders, NTSTATUS mapping |
| `🪟️journal/` | Windows port: journaled two-phase swap + recovery |
| `🟦️.ts` | core: layout, state, allocate/activate/deactivate, children roster, usage, meter, anchor, actions |
| `📜️script.ts` | unchanged child CLI (closed actions) |
| `🧬️schema/🔣️.json` | `Protocol` (names, kernel symbols, Windows constants/statuses/errors/refusals, swap intent, meter), `Anchor`, `Identity`, `Admission`, `SwapIntent`, `MeterRequest`, `MeterLimits` |

### Windows swap (journaled, under the partition lease)
1. write `swap.json` (canonical intent receipt: names, temporary `m.old`, bigint identities of both directories), `fsync` the file, `FlushFileBuffers` on the partition directory;
2. `MoveFileExW(m -> m.old, MOVEFILE_WRITE_THROUGH)`, `MoveFileExW(v -> m)`, `MoveFileExW(m.old -> v)`, each durable before the next;
3. delete the receipt, flush the directory.

Recovery (`recover`, called by the exclusive opener before any state is read; `observeNativeDirectoryStateV1` refuses while a receipt exists) classifies by directory identity against the receipt: S0 nothing renamed (roll back: delete receipt), S1 and S2 (roll forward), S3 (delete receipt); a torn or empty receipt is accepted only when `m` and `v` both exist and `m.old` does not; any other layout (foreign directory, both names holding one identity, symlinked receipt, wrong temporary name, extra receipt field) is refused with `Unrecognized interrupted directory swap state` and left untouched. A refused rename rolls back (step 1, 2) or leaves the receipt for recovery (step 3).

Why readers cannot observe a missing `m` outside the lease window: `m` is absent only between rename 1 and rename 2 (microseconds, three write-through renames), and every swap runs in `activate`/`deactivate`/retirement under the partition's exclusive lease. Every lease-honoring user (receivers, borrowers, cleanup readers, protected providers) holds the shared lease for as long as it touches the partition, so none exists during the window. Lease-less observers are the protected provider rosters (they run under the parent's exclusive writer, which excludes child opens) and `listNativeDirectoryChildrenV1`, which refuses a pending child (or recovers it when the caller holds the parent exclusively and passes the port). A crash leaves the receipt, so the next opener (fixed open path, `retireNativeFixedPlatformV1`, unpublished-birth retention) recovers before reading.

NTFS: `prepareVolume` sets `FILE_CS_FLAG_CASE_SENSITIVE_DIR` through `NtSetInformationFile(FileCaseSensitiveInformation = 71)` on the empty `v` at allocation (children inherit), then the existing `a`/`A` probe must pass. NTSTATUS rows map to precise refusals: build older than 17107, missing optional feature `Microsoft-Windows-Subsystem-Linux` (with the `dism` remedy), non-empty directory, access denied, probe still folding case. Long paths: every FFI path goes through `nativeWindowsExtendedPathV1` (`\\?\` and `\\?\UNC\`); Node `fs` namespaces by itself.

### Capacity (coordinator message `aud-verify4` I1)
`createNativeDirectoryMeterV1` replaces the 16 MiB gate. Every admission is charged: `reserve` charges its request against `max(last measurement + admitted since, ...)`, re-measures the real tree when 16 MiB were admitted since the last measurement, after 2 s, or before any refusal; `commit` always measures fresh; `recheck` measures when the last measurement is older than 2 s. `NativeStoragePortV1.measure(backing, {mode, requestBytes}, control)` is called at reserve (plain and funded child), commit and `reservation.recheck`.

## Changes (file list)

New: `NB/📁️directory/{📋️contract,🔀️exchange,🪟️host,🪟️journal}/🟦️.ts`, tests `…/🪟️host/🧪️tests`, `…/🪟️journal/🧪️tests` (+ fixtures), `NB/🧱️fixed/🧪️tests/🎚️capacity/🟦️.ts`.
Renamed: `🐧️directory` -> `📁️directory`; `🐧️image` -> `📁️image` (`openNativeDirectoryImageStorageV1`, `retireNativeDirectoryImageStorageV1`, Linux and Windows).
Rewritten for the directory backend: `NB/🧱️fixed/🟦️.ts` (Windows backend, PowerShell commands, handle mechanics, VHD allocation/footer, metadata sync and ordinary-read machinery deleted; `fixedCommandSpec` is one child-process spec for Linux and Windows; funded child, protected providers, retirement guards no longer Linux-only; recovery at open, retirement and birth retention), `NB/📤️publication/🧱️fixed/🟦️.ts`, `NB/📤️publication/🎛️owner/🟦️.ts` (Windows metadata command and nofollow gate gone; mode checks skipped on Windows), `NB/📤️publication/📚️witness/🟦️.ts` (native Windows witness removed; POSIX witness on both), `NB/🌐️platform/🟦️.ts`, `📋️contract/🟦️.ts` (+`nativeImageDirectoryHostV1`, Windows block size), `📋️contract/📂️mount` (Windows directory type/schema), `NB/🟦️.ts` (meter calls), `NB/🔌️port/🟦️.ts` (`measure` signature), `NB/🧾️metadata/🟦️.ts` (Windows mapping/receipt readers deleted, directory flush on Windows), `NB/🧹️cleanup/♻️retirement/💾️durability`, `📋️contract` (Windows census substitute), `NB/🌱️birth/🧾️claim/🟦️.ts` (win32 admitted), schemas (`🧱️fixed/🧬️schema`, `📤️publication/🧬️schema`, `🧱️fixed/📤️…/🧬️schema`, `📂️mount/🧬️schema`, `📤️publication/📚️witness/🧬️schema` -> `PosixEof`, `🧾️metadata/🧬️schema.json`).
Deleted: `NB/🪟️image`, `NB/🧱️fixed/{🗂️system,📦️vhd,📥️receipt,📥️allocation,🎛️kernel}`, `NB/📤️publication/📚️witness/🪟️windows`, `NB/📤️publication/🧱️fixed/💾️metadata`, `NB/🌐️platform/🧾️metadata/🧪️tests`, the facade `…📦️packages/🟦️typescript/…/🧱️fixed/🗂️system` and its two `exports` lines in `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/package.json` (only exports, no dependency or lock change).
Elevation: `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🩺️readiness` (no `elevated` fact; Windows needs journaled-swap and case-sensitivity probes, `LongPathsEnabled` reader moved to `🪟️host`), its schema/fixtures/tests; `LIB/🏗️build/🩺️preflight` (Windows `exchange: true`, case capability probe, `nativeDirectoryHostAvailableBytesV1`); `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1` (elevation refusal removed, BOM kept) and its bootstrap test.
Dead capability removed everywhere: `windowsSystem` (`LIB/⚡️caching/📦️artifacts/📋️native-orchestration/{📥️invocation,📥️caller}`, `LIB/⚡️caching/🚀️bootstrap/{📥️caller,🔌️nx-plugin/📤️arguments}`, `LIB/🏗️build/📥️caller`, four schemas, package export `createNativeWindowsSystemGrantV1`).
Other: `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧬️schema` (reader source list), `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json` (deleted schema id), `LIB/🧼️workspace-cleanup/🛡️protection`, `LIB/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph`.

## Tests (language-agnostic fixtures, third-party cross-checks)

- Shared contract: both platform ports (system exchange, simulated exchange) run one swap contract; Windows port over a recording host with real renames.
- Crash recovery fixtures: 7 on-disk states, 8 refused states, all 10 (host call x before/after) crash points, 4 refused-rename rows plus rollback-refused, flush tolerance rows, reader window, full lifecycle over the journaled port while interrupted.
- Windows host: UTF-16/`\\?\` bytes vs Python `utf-16-le`/`ntpath`, `CreateFileW` flags, `NtSetInformationFile` class 71 payload, NTSTATUS table, `RegGetValueW` rows, `GetDiskFreeSpaceExW`, exclusive probe, census substitute.
- Meter fixtures (9 sequences incl. many small writes, writes below the old 16 MiB gate, one large write over the cap, growth after commit, no spurious refusal after a stale estimate) plus a real-tree variant; port-level source test of reserve/commit/recheck call order.
- Linux end to end (see below): honest commit, outgrown write refused at commit, no journal entry while the bytes remain, reservation succeeds after freeing.
- Re-pointed or repaired: publication fixed tests (rewritten for the directory host), birth retention (real anchors; two new rows: interrupted swap recovered, completed-attached swap detached), fixed command tests, receipt/recheck test (coarse Linux timestamps), `paths` fixture (stale `fixed.img`/`fixed.vhd`), rebudget and parent-source tests (`nativeImageUsedBytesV1`, path typos, first `operation` declarator), retirement test `:207` (I2), witness tests (nativeFacts removed, `PosixEof`), readiness and preflight tests, native-orchestration invocation test, Bazel purpose birth test.

## Verification (exact commands, outcomes)

macOS arm64, Bun 1.3.14 (runner: `win2-tools/t.sh`, which mints the root envelope like tst3c; each file alone, `perl alarm 300`):
- `NB/📁️directory/🧪️tests`: 15 pass; `…/🪟️journal/🧪️tests`: 10 pass; `…/🪟️host/🧪️tests`: 10 pass (Python oracles ran).
- `NB/🧱️fixed/🧪️tests` 4 pass, `…/🌱️birth` 1 pass, `🎛️command/🧪️tests` 4 pass, `…/📥️receipt` 2 pass, `…/📥️paths` 1 pass, `NB/📤️publication/🧱️fixed/🧪️tests` 3 pass, `NB/🧪️tests` 10 pass, `NB/🧪️tests/📉️rebudget` 4 pass, witness `🔤️ordering` 1 pass, witness suite 38 pass (excluding the 55 s Python benchmark, `prepared parent isolated`), cache retirement `-t "parent mapping consumes actual private chain"` pass, budget parent suite `-t "child factories"` and `-t "monotonic physical identity"` pass.
- `LIB` readiness 7 pass, preflight 3 pass, bootstrap `-t "applies the long-path"` pass, Bazel purpose birth 6 pass.
- `tsc` (scratch tsconfig over 26 touched production and test files): 0 errors. `bun build --target=bun 📜️script.ts`: bundles. `bun repo --help` and `bun repo preflight`: complete. `C/scan-bare-imports.ts 📜️script.ts @semio-tech`: no bare import reaches `NB`/`📁️directory`.
Linux arm64 (`oven/bun:1.4.2-debian`, uid 1000, `--cap-drop ALL --security-opt no-new-privileges`, ext4 Docker volume): directory suite 15/15, journal 10/10, host 10/10 (no Python: oracles skipped), fixed birth/command/paths/receipt/tests/publication-fixed/index/rebudget all pass, and `🎚️capacity` passes (real `openNativeFixedPlatformV1`, one process per phase because a refusal closes its Script invocation).

Not run: any code on Windows (FFI, `MoveFileExW`, `FlushFileBuffers` on a directory, `NtSetInformationFile`, `GetDiskFreeSpaceExW`, `RegGetValueW`, PowerShell bootstrap parse); a full `bun repo` CLI birth on Linux after these edits (only the fixed-opener level above); `verify taxonomy`/other laws; the Windows ARM64 path.

Still red, not caused by this work (checked): `NB/🧹️cleanup/{🎛️command,📥️operation}` tests (lease/EX harness, `withOperation<T>` text moved to `🔌️port`), the native-orchestration `Native caller phase` and `Native owner capabilities…` tests (`nxOperationArgumentsV1` binding missing after another agent's change; no native grant in the harness), publication owner/witness tests that need the Native grant, `cleanup/📥️receiver` release test on Linux (macOS-only receiver).

## Open items

1. First real Windows CI run must prove: directory `FlushFileBuffers` (failures 1 and 5 are tolerated, relying on write-through renames and file `fsync`), case-sensitivity flag on the build under test plus the probe (`CreateFile` name matching in a case-sensitive directory), `MoveFileExW` on a directory with libuv handles open (libuv opens with `FILE_SHARE_DELETE`, so the rename itself does not detect open handles), legacy rename semantics, `GetDiskFreeSpaceExW`.
2. Windows descriptor census is a substitute (`requireNativeDirectoryUnusedV1`: exclusive open of the child directory and root), not `lsof +D`; handles to files below are fenced only by the lease protocol. A kernel handle census (`NtQuerySystemInformation`) is not built.
3. NTFS file indices above 2^53 (`ino` as Node `Number`, MFT sequence >= 32): the publication owner refuses before effects ("Representable current receipt identity required"); anchors and stamps use bigint but the receipt, request CAS and `lstat` numbers are doubles.
4. About 15 remaining `statfs` call sites in `LIB` (tooling headroom, rustup, nx tooling, graph/products quotas) still call `fs.statfs`; their Windows support in Bun is unproven. `nativeDirectoryHostAvailableBytesV1`/`nativeImageSpaceV1` are the replacements.
5. Windows publication reads the payload with the existing descriptor library's Windows reader through the POSIX witness; its mode and uid frames are meaningless but consistent. The 90-second Python benchmark test in the witness suite and `prepared parent isolated failures` are slow or macOS-specific and were excluded.
6. `.vscode`, `package.json` dependencies and `bun.lock` untouched; only the two removed `exports` lines in the process package manifest changed.
