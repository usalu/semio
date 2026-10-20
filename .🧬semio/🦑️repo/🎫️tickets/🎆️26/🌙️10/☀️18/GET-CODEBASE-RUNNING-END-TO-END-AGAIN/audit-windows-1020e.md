# audit-windows-1020e: Windows hazards in code applied 10-20 (read-only)

Auditor `aud-windows-1020e`. Read-only: no builds, no `bun repo`, no git modifying commands, no sub-agents, no repo-wide grep.

## Scope

- Round 2 and round 3 `touched.json` groups: 307 and 216 unique paths (302 and 216 exist; the 5 missing round-2 paths are deletions or renames).
- cli-zt `plan.json` writes and edits (289 paths), graph-speed `staged/ops.json` and `followup.json`.
- input-precision `staged2`, bzl-hermetic `staged`, conc-tests `staged`, xplat-linux-fix `staged` (not applied).
- Regex scan of 742 live files in scope, then manual reads of each hit that could matter on Windows.
- Paths: repo root `/Users/ueli/Documents/semio`. Line numbers refer to live files unless marked "staged".

## Ranked findings

| # | Severity | Where | Hazard | Fix direction |
|---|---|---|---|---|
| 1 | HIGH | `🛍️…/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts:19` (round 3 `conc-cargo-1`, applied) | `systemAncestryV1` returns `[]` on win32, so `resolveLaneIdV1` gives every Windows agent the default lane. All Windows agents then share one Cargo build dir, which is the lock cycle the lanes exist to remove. | Apply staged xplat `g4` (`build_g4.py`: `Get-CimInstance` listing through `%SystemRoot%` PowerShell). |
| 2 | HIGH | `🛍️…/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🟦️.ts:23` (bzl-hermetic, applied) | `.bin` shims are created with `symlinkSync(target, path)` and no type. Windows file symlinks need Developer Mode or SeCreateSymbolicLinkPrivilege, so this fails with EPERM. Windows gets the `local` strategy (table: `windows: remote,local`), which runs this runner. | On win32 write a copy or a `.cmd` shim instead; use `junction` only for directories (as `link()` at line 18 already does). Fail loud. |
| 3 | HIGH (silent) | `🛍️…/🏗️build/🏃️execution/🔒️hermetic/📦️dependencies/🟦️.ts:21` | `shims()` reads each `.bin` entry with `readlinkSync` inside `try{…}catch{return []}`. Where `.bin` entries are not symlinks (bun on Windows writes shims), `readlinkSync` throws and every shim is dropped without an error. | Use `readdirSync` plus a win32 branch; throw on unexpected failure instead of returning `[]`. |
| 4 | HIGH | `🛍️…/🏗️build/📏️incremental/⌨️command/🟦️.ts:44` (round 2 `bzl-incr`, applied) | Default root on Windows is `${SystemDrive}\sinc`, i.e. `C:\sinc`. Standard users cannot create folders at a drive root, so `repo incremental` and `repo concurrency` fail by default on Windows. POSIX default is the literal `/tmp/sinc`. | Apply staged cli-zt2 item 4 (`os.tmpdir()/sinc`, 100-char refusal). Live help text (`⚙️operations/🔣️catalog.json`) still documents `/tmp/sinc` and `<drive>:\sinc`, so it changes with it. |
| 5 | MEDIUM-HIGH (silent) | `🛍️…/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/💾️storage/🟦️.ts:108` and `:112` | `concurrentFreeFractionV1` returns 1 and `concurrentFreeBytesV1` returns `Number.MAX_SAFE_INTEGER` when `statfsSync` throws. Windows statfs is unproven (xplat result, open item 4), so the free-space admission would be silently off on Windows. | Use the host free-space port already used by `📁️directory` (`nativeDirectoryHostAvailableBytesV1`, GetDiskFreeSpaceExW, live `🟦️.ts:40-44`) and throw on failure. Staged xplat g12 keeps statfs, so it does not fix this. |
| 6 | MEDIUM | `🛍️…/🗃️unit-cache/🧹️gc/🟦️.ts:53` | `catch{}` keeps the default `freeBytes`. Same silent class as row 5. | Same as row 5. |
| 7 | MEDIUM | `…/🏗️compiler/📏️capacity/🟦️.ts:26,41`; `…/🔒️enforcement/🔌️nx/🟦️.ts`; `…/🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts:1,87` | Direct `statfs(path)` from `fs/promises` with no win32 branch. On Windows this depends on unproven statfs support and fails loudly if missing. | Staged xplat g10 and g11 route these through `nativeStatfsV1`; apply them. |
| 8 | MEDIUM | `…/🏃️execution/🧪️tests/🔒️classes/🟦️.ts` (staged `bzl-hermetic-tools/staged/law.ts:138`) | Test calls `spawnSync("chmod",["-R","u+w",…])`. Windows has no chmod, so read-only Bazel outputs stay and `rmSync` fails. Test only. | Walk and use `fs.chmod` or `fs.rm` with a retry. |
| 9 | MEDIUM (tests) | staged conc-tests `sb-test.ts:19,63,68,76`; staged graph-speed `lean/test.ts:11`; input-precision `🧪️tests/🐍️oracle.py` runner | Tests require `python3` and `rg` by exact name (`Bun.which("python3")`, `spawnSync("rg")`). On native Windows these are usually `py`/`python`, and `rg` is not installed by default. Oracles then skip silently. AGENTS requires zero-touch cross-platform. | Resolve `python3`, then `python`, then `py -3`, and `rg` through a single interface; fail loud when an oracle is required and missing. |
| 10 | MEDIUM | `…/⚡️caching/🚀️bootstrap/📜️script.ts:466,474,484` | POSIX group kill `process.kill(-child.pid)` is in a silent `try/catch`. On Windows, `killAll` runs `taskkill /pid … /f` without `/T`, so it only reaches the PowerShell-captured descendants. If the capture fails (it only logs), grandchildren survive. | Use `taskkill /T /F /PID` per root pid and report a failed capture as an error. |
| 11 | MEDIUM (unverified) | `.cache/b/d/…` dependency trees, bzl-hermetic runner store | `.cache/b/d` holds about 947k inodes (xplat result). Nested `node_modules/.bun/<pkg@ver>/node_modules/<pkg>/…` below a checkout of about 150 units can exceed the 256-char AGENTS limit on Windows without long-path support. Not measured. | Measure the maximum depth on a Windows host before the first real run. |
| 12 | LOW | staged graph-speed `🪶️lean-inventory` (live `📥️inputs/🪶️lean-inventory/🟨️.mjs:14`); `🗃️ledger/📒️ledger/🟦️.ts:36,58`; `💾️storage/🟦️.ts:103` | `constants.O_NOFOLLOW ?? 0` removes the no-follow guard on Windows. The lstat check before open remains, so there is a TOCTOU window and no reparse-point protection. | After open, compare fstat identity (dev, ino, size) against the lstat result; refuse when they differ. |
| 13 | LOW | `…/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts:93,207` | `executable: (stat.mode & 0o111) !== 0` is always false on Windows, so the same bytes get different manifests per platform. | Derive from extension or content on win32, and record that rule in the manifest. |
| 14 | LOW | `…/🎫️gate/🟦️.ts:85`; `…/🗂️registry/🟦️.ts:88,125`; `mkdirSync(…,{mode:0o700})` | Mode 0o700 is ignored on Windows. Directories sit under `LOCALAPPDATA`, which is per user, so the risk is small. | Document only. |

## Verified safe (checked in code)

- Symlinks with a Windows fallback: `🧪️deploy/🛑️cancellation-and-staging/🟦️.ts:76` (junction on win32), `🔁️verification/📋️orchestration/🟦️.ts:82` (junction on win32), bzl runner `link()` at `🏭️runner/🟦️.ts:18` (junction for directories).
- `ps` and `/proc` callers: `🌡️host-capacity/📡️sampling/🟦️.ts:116-122` returns null on win32; `📈️telemetry` falls back to `freemem()` for any other platform; `🧩️component-pass/🟦️.ts:318` returns RSS on win32; `🏃️process…/🩹️recovery` boot epoch has a win32 branch (`📋️plan/🟦️.ts:106`).
- `hdiutil`, `vm_stat`, `sysctl`, `/usr/bin/*`: darwin-only at the callers that matter (`🍎️image`, `🩺️heal`, `sb-images.ts:7`, sandbox tests skip unless darwin). Build-script shim is gated by `policy.buildScript.platforms: ["darwin","linux"]` (`🗃️unit-cache/🧬️schema/🔣️.json:34`).
- Shell usage in product code: bootstrap uses PowerShell with `Get-CimInstance` on win32 (`📜️script.ts:458`). `🐳️stack/compose.yaml` `sh -c` runs inside Linux containers only.
- PATH separator: bzl runner uses `;` on win32 (`🏭️runner/🟦️.ts:39`). `path.delimiter` and `split(":")` hits are not PATH splits.
- Tool env: `LOCALAPPDATA` fallback for lease and host-capacity roots (`🎫️gate/🟦️.ts:17`, `🧭️order/🗂️registry/🟦️.ts:21`).
- Line endings: `.gitattributes` sets `* text=auto eol=lf`, and `**/🧫️fixtures/** -text`. CRLF is not a fixture risk. Parsers that split on `\n` are fed by POSIX tools only (PowerShell output is compressed JSON).
- Case: `probeNativeDirectoryCaseSensitivityV1` exists (`📁️directory/🟦️.ts`), and `testPathInsideV1` folds case on win32 and darwin (`sb-layout.ts:33`).
- Separators: the input-precision engine works on `/` strings only (host must convert). A mismatch fails closed with "cannot bound", not silently. bzl dependencies use `sep`; crate-tree normalizes with `replaceAll("\\","/")`.
- Sandbox root: `maximumRootLength: 90` (`sb-protocol.json`), enforced at `sb-root.ts:48`.

## Path lengths (UTF-16 code units; a variation selector counts as one unit)

- 742 live files in scope: longest relative path 154 units (`…/📦️budget/📥️opening/🏪️store/🧪️tests/📥️phase/📋️callers.json`). With the prefix `C:\Users\ueli\Documents\semio\` (32 units) that is 184, under 256. Longest UTF-8 byte length 210.
- xplat staged new files: longest 147 units (177 with prefix).
- Sandbox root `C:\Users\ueli\AppData\Local\Temp\st\<8 hex>` is about 44 units, under the 90 cap. The part below it, `artifacts/<route>`, is not capped.
- `C:\sinc` plus the longest repo-relative path is 161 units. The 100-char `sinc` refusal is not reached.
- Not measurable here: nested `.cache/b/d` store paths (row 11).

## Not verified

- No Windows host and Docker is down, so nothing ran on Windows or Linux. Rows 1-5 rest on static reading.
- Bun `statfs` on Windows, and whether bun writes `.bin` as symlinks or shims on Windows (rows 3 and 5).
- Out of ledger scope: `🛍️…/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts:397` runs `sh -c "git archive … | tar"` with single-quoted paths. It is not in any round 2 or 3 group. Note only: it needs Git Bash on Windows.
- Reserved port range 49152-65000 overlaps the Linux ephemeral range (xplat result, recorded risk). Not Windows-specific.
