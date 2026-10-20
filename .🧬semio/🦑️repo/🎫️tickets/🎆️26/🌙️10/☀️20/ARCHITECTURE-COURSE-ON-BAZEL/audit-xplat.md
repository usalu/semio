# Audit: `bun repo --tool=bazel run` for @teaching/architecture-course on four environments

Read-only static audit by aud-xplat. No source edited, no builds, no `bun repo` run, no sub-agents. Paths are relative to the checkout root `/Users/ueli/Documents/semio/` unless absolute. Shorthand: `BZ` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`, `BS` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel`, `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`. Generated data: `T/🗑️generated/aud-xplat/` (`tracked.json`, `closure-packages.json`, `path-budget.json`).

## 0. Verdict

- The Bazel backend is structurally platform-complete (per-OS profile, host-path guards, directory-backed storage on Linux and Windows). Most 1019 blockers are gone in source.
- Blocking gap on every OS with a checkout under `/Users/ueli/Documents/semio`-like paths: the course closure produces Bazel source paths of 250 to 262 UTF-16 units. AGENTS.md caps paths at 256. Windows native additionally needs machine-wide `LongPathsEnabled` (an elevated, system-wide change), which violates zero-touch and is refused by the adapter today.
- Single highest-leverage fix: stop encoding the repo path into Bazel source labels. Map each closure source to a short generated name (for example `s/<hash>`), keep the real path in the task manifest. This removes the path problem on all four environments at once.
- CI already runs `bun repo --tool=bazel run @teaching/architecture-course:test` on ubuntu, macos and windows (`.github/workflows/build-system.yml:279`). Only the `test` target is covered; no other course target runs through Bazel in CI.

## 1. Status of the peer fixes (grep-verified in source)

| Peer item (source) | Status | Evidence |
|---|---|---|
| Nx `workspace:*` pins dropped (result-bzl-xplat finding 1) | LIVE | `BS/../🛠️tools/🟦️.ts:122,181` call `nxExternalPinsV1`; defined at `BS/../🛠️tools/📦️sdk/📥️source/📐️projection/🟦️.ts:4` |
| Nx setup ordering / workspace links (finding 2) | OPEN | no code change; `.github/workflows/build-system.yml:255` runs `tools setup all` before `dependencies sync` |
| Linux guard on `install` base (finding 3) | LIVE | `BZ/🧬️schema/🔣️.json` `HostPaths.platforms.linux.guarded = ["install"]`; enforced by `BZ/💾️storage/🟦️.ts:18-19` |
| Locale env darwin-only (finding 4) | LIVE for Bazel | `BZ/🧬️schema/🔣️.json` `PlatformRules.platforms.darwin.environment`; applied at `BZ/📜️script.ts` (`prepareBazelModulesV1`, `bazelPlatformRulesV1(...).environment`) |
| Linux sandbox absent in unprivileged container (finding 5) | DOCUMENTED, not fixed | course tasks are `no-sandbox` (see section 3.4), so the effect is limited to hermetic tasks |
| Runner regex accepts `processwrapper-sandbox` (finding 8) | LIVE | `BZ/🧪️tests/📥️actions/🟦️.ts:165` |
| `--nowindows_enable_symlinks` as startup flag (finding 6) | LIVE | `PlatformRules.platforms.win32.startup` in `BZ/🧬️schema/🔣️.json` |
| `--output_user_root` removed (finding 7) | LIVE | absent from `BZ/📜️script.ts`; asserted absent at `BZ/🧪️tests/📥️platform/🟦️.ts:52` |
| Platform profile module and fixtures | LIVE | `BZ/🌐️platform/🟦️.ts` (29 lines, `bazelPlatformProfileV1` at :18-28); tests `BZ/🧪️tests/📥️platform/` |
| 1019 VHD mount privilege (bazel-2026-10-19.md, audit-cross-platform G3) | SUPERSEDED in source | `NB/🌐️platform/🟦️.ts:24` routes linux and win32 to `openNativeDirectoryImageStorageV1` (`NB/📁️image/🟦️.ts:4`: `directoryHost` linux or win32). No production `Mount-DiskImage`, `losetup` or `fixed.vhd` code remains (only tests and `🛍️products/🦑️repo/README.md`, which is stale). |
| 1019 Linux loop-device and CAP_SYS_ADMIN requirement (G1, G2, G6) | SUPERSEDED in source, verified by 1020 container run | Directory backend on Linux; `bun repo tools setup bazel` rc 0 as non-root `vscode` in the unprivileged container (result-bzl-xplat "Linux arm64"). |
| 1019 devcontainer storage 32 GiB and locale (G2, G5) | CLOSED | `.devcontainer/devcontainer.json:44` `storage: "128gb"`; `.devcontainer/Dockerfile:68-78` generates `en_US.UTF-8` and sets `ENV LANG` |
| Unicode workspace/install ancestry (bazel-2026-10-19, Linux and Windows) | LIVE guard | `🛍️products/🦑️repo/.../⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🩺️readiness/🟦️.ts:19-23` (`bazelNonAsciiV1`), refusal at :30 for workspace and tools/dependencies/products roots on win32; darwin/linux install guard in storage (see above) |

## 2. Path length budget (the question asked in item 3)

### 2.1 Method

- Closure approximated as the 51 workspace packages reachable from `@teaching/architecture-course` (`T/🗑️generated/aud-xplat/closure-packages.json`), all their tracked files: 3,458 files (`tracked.json`). This is an upper bound on the real Bazel source set (the real set is `bazelTaskInputsV1` over `namedInputs.default`, which I did not execute). Ticket folders excluded.
- Length metric: JavaScript `.length` (UTF-16 code units). This is the repo's own convention (`BZ/💾️storage/🟦️.ts:24,42`, `STO/📥️opening/📂️paths/🟦️.ts:9` all use `.length > 256`). Code points and UTF-8 bytes are reported too.
- Bazel layout per `BZ/💾️storage/🟦️.ts:22-42` and `STO/.../📂️paths/🟦️.ts:9`: partition directories are `.cache/b/<t|d|p|l>/m`, output base is `<products>/bazel/outputs/server/<16 hex>`, source labels are `execroot/_main/s/<repo path>` (`BZ/📜️script.ts` `linkBazelSourcesV1`, `actions.sources.link = "s"`).
- Measured: the existing install base `.cache/b/t/m/bazel/install/8.4.2/darwin-arm64` has 589 entries, longest 151 characters below the checkout root (178 absolute).

### 2.2 Results (UTF-16 units, the repo metric)

| Platform (checkout root example) | Output base | Execroot prefix to `s/` | Longest source path (closure upper bound) | Limit 256 | Windows MAX_PATH 259 | Output file (`bazel-out/.../t_*.snapshot/files/<32>`) |
|---|---|---|---|---|---|---|
| macOS arm64 `/Users/ueli/Documents/semio` | 78 | 96 | **260** (cp 248, utf8 316) | EXCEEDED | n/a (PATH_MAX 1024 bytes, 316 bytes OK) | 197 |
| Linux devcontainer `/workspaces/semio` | 68 | 86 | **250** (cp 238, utf8 306) | ok | n/a | 177 |
| Linux native `/home/ueli/Documents/semio` | 77 | 95 | **259** (cp 247, utf8 315) | EXCEEDED | n/a (PATH_MAX 4096) | 186 |
| Windows `C:\Users\ueli\Documents\semio` | 80 | 98 | **262** (cp 250, utf8 318) | EXCEEDED | EXCEEDED without LongPathsEnabled | 198 |

The longest closure file is `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📦️package/🌱️environment/📖️references/🧪️testing/🔣️json/🟦️.ts` (relative 164 units).

Other rows:
- Sandboxed actions (`<output base>/sandbox/<strategy>/<id>/execroot/_main/s/...`, assumed layout): 287 (macOS), 277 (devcontainer), 286 (Linux native), 290 (Windows). Only relevant if a hermetic task exists. None of the course targets is hermetic (`📋️project.json` has no `hermetic` metadata; `hermetic` appears in the adapter only in `BZ/🏃️execution/...`).
- Install base files: longest estimated 221 (macOS), 211, 220, 223 (Windows). Fine.
- Output base: 78 / 68 / 77 / 80 (the code's own cap of 256 at `BZ/💾️storage/🟦️.ts:42` is far from binding).

### 2.3 What this means

- The path budget in `BZ/🌐️platform/🟦️.ts:24` uses a constant `trackedPathUnits = 256` (`BZ/🧬️schema/🔣️.json` `PlatformRules.const.trackedPathUnits`), not the real closure. The constant makes Windows always refused unless LongPathsEnabled (output base 80 + 18 + 256 = 354 > 259; refusal text at :25). That is correct as a refusal, but it is the reason Windows native is blocked, not the real closure (262 exceeds 259 too, so the conclusion holds).
- For the 256 AGENTS rule, the checkout root must be at most 23 units with this closure (51 + 18 + 164 = 233 + root <= 256, root <= 23 units). For Windows MAX_PATH 259, root <= 26 units. `C:\semio` (8 units) fits both; the profile already suggests it (`BZ/🌐️platform/🟦️.ts:25`).
- The emoji do not add a package-name problem: the generated BUILD has only the root package `//` with ASCII labels `t_<16 hex>` (`BZ/📜️script.ts` `renderBazelBuildV1`, schema `ActionBirth.label` pattern `^t_[a-f0-9]{16}$`). Emoji appear only in file labels (`s/<path>`), which the schema admits (`BZ/🧬️schema/🔣️.json` `actions.sources.label` accepts U+0080 to U+FFFF). Output files are flattened to `files/<32 hex>` (`BZ/📜️script.ts:625-631` `captureBazelSnapshotV1`), so no emoji reach `bazel-out`.

## 3. Per-environment findings

### 3.1 Shell and actions (all OS)

- No `run_shell` or `genrule` anywhere in `BZ` (checked). Actions use `ctx.actions.run` with an absolute `bun` executable (`BZ/📜️script.ts` `_task_impl` around line 537). No shell, no `BAZEL_SH` needed.
- Spawns in the adapter path use `shell: false` (`🛍️products/🦑️repo/.../📚️library/🏃️process/🟦️.ts`, `.../🍞️bun/🟦️.ts`, `.../containers/.../lifecycle/🟦️.ts`).
- Remaining shell usage (not on the course path): `nxExecWrapperV1` in `BS/../⚙️operations/🟦️.ts` uses `shell: process.platform === "win32"` (Nx `exec` only). The remote-cache credential helper is a `#!/bin/sh` script on POSIX and a `.cmd` batch file on Windows (`BZ/🌐️remote-cache/🟦️.ts:13-15`). Only written when a remote cache endpoint is configured.

### 3.2 Windows native

Blockers, in priority order:

- P0 Long paths. Course closure needs 262 units, the adapter needs `LongPathsEnabled` (machine-wide HKLM). Refusal text is at `BS/🩺️readiness/🟦️.ts:31` and instructs an elevated PowerShell. This conflicts with "zero-touch" and with the agent rule against system changes. CI works only because the workflow sets it (`.github/workflows/build-system.yml:65-71`, :202-208).
- P0 Checkout location. Non-ASCII ancestry is refused (`BS/🩺️readiness/🟦️.ts:30`). Any Windows user whose profile path has non-ASCII characters is blocked; the remedy (move the checkout) is correct but manual.
- P1 Emoji source labels on win32 are flagged `emojiLabels: "unverified"` (`BZ/🧬️schema/🔣️.json` `PlatformRules.win32`); warning only. The bazel-2026-10-19 smoke could not start Bazel at all on this host (ClassNotFound, Unicode jar path), so no Windows Bazel run has proven the closure.
- P1 Case sensitivity. The probe (`NB/📁️directory/🟦️.ts:59`, native `setCaseSensitive` in `NB/📁️directory/🪟️host/🟦️.ts:48`) must pass beneath the workspace cache root. The refusal at `BS/🩺️readiness/🟦️.ts:35` tells the user to run `dism` elevated. Whether per-directory case sensitivity truly needs that feature was not verified; if it does not, that refusal text is wrong and should be corrected.
- P2 `.cmd` credential helper (`BZ/🌐️remote-cache/🟦️.ts:14`) is launched by Bazel as an executable. Batch files are not reliably launchable that way; use a `.exe` or route through `cmd /c` if remote cache is ever enabled on Windows.
- P2 Junction source link (`BZ/📜️script.ts` `linkBazelSourcesV1`, `symlink(root, link, "junction")`) and `--nowindows_enable_symlinks` are unverified against the real Windows binary.
- Verified good: `.gitattributes` is `* text=auto eol=lf` (LF BUILD and fingerprints on Windows), `*.cmd` CRLF, `.cache` in `.gitignore`.

### 3.3 Linux native (non-root)

- Privilege: resolved in source (directory backend). Remaining Linux risk is path length (250 to 259 units; over 256 for any checkout root longer than 23 units).
- Sandbox: course tasks are not sandboxed (see 3.4); no strategy is named in the adapter, so the host default is used. `linux-sandbox` needs user namespaces; the adapter does not depend on it for the course.
- Locale: Bazel receives no `LANG`/`LC_ALL` on linux (`PlatformRules.platforms.linux.environment = {}`). Peer experiments found this is fine (all five locales accepted for emoji outputs; only ASCII `install` is required).
- Verified: `ASCII Bazel host path required` guards `install` (`BZ/💾️storage/🟦️.ts:19`).

### 3.4 Sandbox strategy per OS (course closure)

- Every `task` and `launch` action carries `host_state` and therefore `no-sandbox` (`BZ/📜️script.ts` `_requirements` at about line 521; `renderBazelBuildV1` pushes `host_state` for non-noop actions). This is the same on darwin, linux and win32: the course build runs in the real checkout with no Bazel sandbox, so `darwin-sandbox`, `linux-sandbox` and Windows `local` are irrelevant to course correctness. The darwin non-ASCII sandbox bug from `HostPathPolicy` does not apply.
- Consequence: cache correctness depends on the declared fingerprint (`.input` files), not on sandbox isolation. Out of scope here, but the 1020 cache audit should cover it.
- Only `noop` actions are sandboxed (`BZ/🧬️schema/🔣️.json` `actions.kinds.noop.sandbox = true`).

### 3.5 Devcontainer (Linux, `vscode` user)

- `.devcontainer/devcontainer.json:102` `remoteUser: vscode`; `:100` `.cache` is a named volume `semio-native-cache`; `:44` storage 128 GB; `:107` `LANG` set and generated by the Dockerfile. Good.
- `.devcontainer/docker-compose.yml:5` comment says the container is unprivileged by design; consistent with the directory backend.
- Path budget 250 units: fits 256 only with a root of 23 units or less (`/workspaces/semio` is 16, so 250 passes; the remaining risk is only with the flattening not applied and a longer repo tree growing).
- `linux-sandbox` is unavailable unprivileged (peer finding 5); irrelevant for `no-sandbox` tasks.

### 3.6 macOS arm64

- Path: 260 units (316 bytes). Exceeds 256 by 4 units; PATH_MAX is 1024 bytes so the kernel is fine. AGENTS.md rule is violated by the Bazel tree (the checkout's `.cache` is inside the repo).
- Platform profile constant is wrong: `posix` limit is 4095 (`BZ/🧬️schema/🔣️.json` `PlatformRules.const.limits.posix`), but macOS PATH_MAX is 1024 bytes. Give darwin its own limit (1024 bytes) and measure bytes, not units, for darwin.
- Output base guard (`darwin` HostPaths `outputs`, `server`, `install`) keeps the checkout root ASCII.
- DMG cache images were the 1019 model; the macOS storage path (`NB/🍎️image`, `hdiutil`) is still in use on darwin only (`NB/🌐️platform/🟦️.ts:21,24`).

## 4. Prioritized fixes

P0 (blocks the goal on a platform):

1. Flatten Bazel source labels. In `BZ/📜️script.ts`, `bazelSourceLabelV1` (line ~348) and `linkBazelSourcesV1` (line ~446): replace per-file repo paths with generated short names (for example `s/<sha256(path) 16 hex>` symlinks, or one symlink per source under a hash directory). Carry the real path in the task manifest so the runner still reads the real checkout. Expected result on all four OS: longest Bazel path about 96 + 65 bytes, under 256 everywhere. Test: extend `BZ/🧪️tests/📥️sources/` and `📥️platform/` with a fixture of emoji paths asserting the generated label is under 256 units for the closure fixture.
2. Windows: remove the hard dependency on machine-wide LongPathsEnabled. Once (1) is done, the closure fits 259 for checkouts up to 26 units; make the refusal (`BS/🩺️readiness/🟦️.ts:31`) apply only when the computed budget exceeds 259 instead of always. Keep the workflow setting as belt and braces.
3. macOS and Linux path budget constants: replace the global `trackedPathUnits = 256` and posix `4095` with real measured values (`BZ/🌐️platform/🟦️.ts:24`, schema `PlatformRules`), and make darwin 1024 bytes (count UTF-8 bytes there).
4. Run the course closure end to end on windows-latest, macos-latest and ubuntu, with `--tool=bazel` for every course target (see section 5), before claiming done. No Windows run of the closure exists; the peers' Windows smoke failed at the Bazel launcher (Unicode install path), which is a different failure.

P1:

5. Windows case-sensitivity refusal text (`BS/🩺️readiness/🟦️.ts:35`): verify whether non-elevated per-directory case sensitivity works; if yes, delete the `dism` instruction.
6. Windows emoji labels: promote from `unverified` once the Windows closure run passes; add a Windows fixture row.
7. `BZ/🌐️platform/🟦️.ts:27`: keep the emoji warning until (6).
8. Docs: `🧰️framework/🛍️products/🦑️repo/README.md` still describes Mount-DiskImage and `fixed.vhd` (stale since the directory backend). Update to the directory backend.

P2:

9. Windows credential helper as `.cmd` (`BZ/🌐️remote-cache/🟦️.ts:14`): make it an executable or use `cmd /c`; only matters with a remote cache.
10. Junction source link on Windows (`BZ/📜️script.ts`): verify with the real binary.
11. `.bazelrc` and `MODULE.bazel` at the checkout root with a leftover `bazel-out/` directory (`/Users/ueli/Documents/semio/bazel-out`): only relevant if someone runs bazel at the root; delete or ignore.
12. CI: `.github/workflows/architecture-course.yml` (release) runs no Bazel and uses `sudo` for every step; out of scope for this ticket but note that CI never exercises the non-sudo path.

## 5. CI and provisioning

- `.github/workflows/build-system.yml` (tracked, triggered on PR, push to main and manually; this supersedes the 1019 statement that no workflow runs Bazel):
  - smoke job, all three OS: `tools setup all` (line 255-256 products, 120-121 smoke), cache proof for nx and bazel on `@teaching/architecture-course:test` (lines 135-165).
  - products job, all three OS: `bun repo --tool=bazel run "$PROOF_TARGET"` at line 279, with the Bazel disk cache restored (lines 268-273).
  - Windows jobs set `core.longpaths` and `LongPathsEnabled` (lines 65-71, 202-208). The Windows job therefore never exercises the refusal path.
  - Only the `test` target runs under Bazel. The brief's full target list (build, typecheck, check, dev, dev-site, publish, test-e2e, preflight, deploy verbs) is not covered by any CI job.
- `.github/workflows/architecture-course.yml`: no `--tool=bazel`, no matrix, Linux only (`ubuntu-24.04` and a self-hosted runner), every step `sudo --non-interactive` (lines 77-88, 106-111, 149-161).
- Devcontainer provisioning: `.devcontainer/Dockerfile` pins bun 1.4.2 and node 24.15.0 with sha256 (lines 84-98) and generates the locale (68-78). `postCreateCommand` runs `bun repo run workspace:setup` (devcontainer.json:38). Bazel itself is provisioned by `tools setup bazel` (checksum-pinned in `BS/🧬️schema/🔣️.json` `Release`), not by the Dockerfile.

## 6. Not verified (explicit)

- No Bazel run on any OS (read-only audit; the peer's Windows Bazel launcher failure is cited, not re-run).
- Exact Bazel source set of the course: upper bound used (package closure). The real set from `bazelTaskInputsV1` may be smaller, which would lower the 260 and 262 figures but would not change the conclusion for the closure's deepest framework files if they are inputs.
- Sandbox directory layout (`<output base>/sandbox/<strategy>/<id>/execroot`) assumed, not observed; affects only hermetic tasks (none in the course).
- Windows: junction semantics under Bazel, `--nowindows_enable_symlinks` on the real binary, emoji source labels, `.cmd` credential helper launch, per-directory case sensitivity without elevation, whether `bun.exe` is long-path aware.
- Linux non-root end-to-end `bun repo --tool=bazel run` (peers stopped at the build:dependencies stage on 1020 and did not finish a course run).

## 7. Files read or measured

- `BZ/🌐️platform/🟦️.ts`, `BZ/💾️storage/🟦️.ts`, `BZ/📜️script.ts` (lines 17-30, 40-52, 60-75, 254-260, 348, 386-420, 440-530, 537, 600-631, 774), `BZ/🧬️schema/🔣️.json` (`PlatformRules`, `HostPaths`, `StorageLayout`, `CacheFlags`, `Actions`), `BZ/🌐️remote-cache/🟦️.ts`, `BZ/🏃️execution/🟦️.ts`
- `BS/🟦️.ts`-level: `BS/📜️script.ts` (58 lines), `BS/🩺️readiness/🟦️.ts` (lines 19-58), `BS/🧬️schema/🔣️.json` (`Release`), `BS/../🛠️tools/📦️sdk/📥️source/📐️projection/🟦️.ts`
- `NB/🌐️platform/🟦️.ts`, `NB/📁️image/🟦️.ts`, `NB/📁️directory/🟦️.ts`, `NB/📁️directory/🪟️host/🟦️.ts`
- Storage budget `STO/🔒️enforcement/📦️budget/🔣️policy.json` and `.../📥️opening/📂️paths/🟦️.ts`, `.../📥️opening/🟦️.ts`
- Course: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`, `package.json`
- Peers: `GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-bzl-xplat.md`, `audit-cross-platform-1019.md`, `ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/bazel-2026-10-19.md`
- `.devcontainer/devcontainer.json`, `Dockerfile`, `docker-compose.yml`; `.github/workflows/build-system.yml`, `architecture-course.yml`; `.gitattributes`, `.gitignore`, `.bazelrc`, `MODULE.bazel`
