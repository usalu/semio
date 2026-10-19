# Audit 1019: cross-platform zero-touch build system (aud-xplat)

Read-only static audit. No `bun repo` run except `--help` (none run), no platform executed, no source edited. Measurements were read-only: `git ls-files`, `bun -e` JSON reads of policy/schema files, `grep`/`perl` line lookups.

Abbreviations (all paths relative to the repo root `/Users/ueli/Documents/semio`):
- `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`
- `BLD` = `LIB/🏗️build/`
- `BST` = `LIB/⚡️caching/🚀️bootstrap/`
- `STO` = `LIB/⚡️caching/💾️storage/`
- `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/` (native image backends; the `📦️packages/🟦️typescript/…` copy is a re-export facade only, verified)
- `NAT` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/`

Requested scope "native image code `🧱️fixed`" lives in `NB/🧱️fixed/`, not under `LIB`; it is included.

## 0. Verdict

- macOS (arm64/x64): zero-touch is plausible. Needs no privilege for `hdiutil`; manual Xcode CLT step only.
- Linux native and devcontainer: broken for a non-root user. Every `bun repo` command (except help/preflight) attaches the `leases` loop image first. That needs CAP_SYS_ADMIN, and nothing in the code uses sudo. CI works only because every step is wrapped in `sudo`. The devcontainer runs as `vscode` (`remoteUser`), so `privileged: true` does not give it the capability.
- Windows native: the first run needs an elevated shell for VHD mounts (warning only, not refused). Long paths are a warning, not enforced. Several operations are explicit "not qualified" refusals. Bazel refuses non-ASCII ancestry.
- CI: one workflow, `workflow_dispatch` only, Linux only (custom Debian 12 x64 self-hosted runner), no macOS, no Windows, no PR gate, no Bazel job. It does use the neutral `bun repo` entry.

## 1. Platform branches (inventory and gaps)

| Concern | darwin | linux | win32 | Gap / evidence |
|---|---|---|---|---|
| Native image backend selector | `NB/🌐️platform/🟦️.ts:22` maps darwin to macos | same file: linux to ext4 loop | else to windows (VHD) | Selector is fine (schema-gated at line 22). |
| Image format | DMG `NB/🍎️image/🟦️.ts:27,35,47,59` (`fixed.dmg`) | ext4 file `fixed.img` `NB/🧱️fixed/🟦️.ts:171` (`fallocate --length`, `mkfs.ext4 -E nodiscard`, `mount -t ext4 -o loop,nodev,nosuid`, `losetup --associated`, `umount`) | VHD `fixed.vhd` via PowerShell `Mount-DiskImage`, `Initialize-Disk`, `Format-Volume`, `Dismount-DiskImage` `NB/🧱️fixed/🟦️.ts:171`; allocation via `nativeFixedAllocationScriptV1` (FileStream based) called at `NB/🧱️fixed/🟦️.ts:173` | Three independent backends, no shared contract tests across them beyond fixtures. |
| Binary paths | absolute `/usr/bin/hdiutil` `NB/🧹️cleanup/🎛️command/🟦️.ts:132`; `hdiutil info` `NB/🍎️image/🟦️.ts:27` | restricted PATH `/usr/sbin:/usr/bin:/sbin:/bin` `NB/🧱️fixed/🟦️.ts:171` | absolute System32 PowerShell from `GetSystemDirectory`-style facts `NB/🧱️fixed/🗂️system/🟦️.ts:27` | Linux `preflight` resolves tools through `PATH` `BLD/🩺️preflight/🟦️.ts:45`, but the fixed backend uses its own list; they can disagree. |
| Privilege | none | CAP_SYS_ADMIN (CapEff bit 21) + `/dev/loop-control` + free loops: `BLD/🩺️preflight/🟦️.ts:33,50`; `BST/🛠️tools/🏗️bazel/🩺️readiness/🟦️.ts:40,41` | elevation probe `fltmc`: `BST/…/🩺️readiness/🟦️.ts:61`; only a warning: `…/🩺️readiness/🟦️.ts:36` | Linux: no privileged path at all (see gap G1). Windows: refusal is downgraded to a warning, but attach then fails natively (HRESULT 0x80070522). |
| Free loop devices | n/a | counts only *existing* unused `/dev/loopN` nodes: `BLD/🩺️preflight/🟦️.ts:51`; `…/🩺️readiness/🟦️.ts:58`; threshold 3 (Bazel schema `PlatformPolicy.linuxMinimumFreeLoops`) or 7 (course `📋️project.json` `minimumFreeLoops`) | n/a | Ignores that `losetup`/`mount -o loop` allocate new nodes via `/dev/loop-control`. Hosts with snap squashfs loops refuse falsely (gap G6). |
| Atomic rename / exchange | `renameSync` for birth retention `NB/🧱️fixed/🟦️.ts:192`; `renameSync` again at `NB/🧱️fixed/🟦️.ts:290` | same `renameSync` | `MoveFileExW(from,to,8)`, non-replacing, path <=256 `NB/🧱️fixed/🗂️system/🟦️.ts:21` | No `renameatx_np`, `renameat2(RENAME_EXCHANGE)` or `RENAME_SWAP` anywhere in scope (grep: 0 hits). Nothing needs an exchange today; the Windows side is atomic only when non-replacing. |
| Symlink vs junction | none for node_modules (Bun isolated linker, `dir` default) | `symlinkSync(…,"dir")` for Nx installation pointer `BST/🛠️tools/📜️script.ts:86` | `junction` `…/🛠️tools/📜️script.ts:86`; Bazel source link `BLD/🔧️tools/🏗️bazel/📜️script.ts:286` (junction on win32) | Nx pointer on win32: unlink then rename, non-atomic window at `…/🛠️tools/📜️script.ts:93-95` (gap G8). |
| Path length | `directory.length>256` `STO/🔒️enforcement/📦️budget/📂️paths/🟦️.ts:9` (all platforms) | Linux: no limit beyond the 256 check | MAX_PATH: no readiness check for `LongPathsEnabled` or `core.longpaths`; bootstrap only warns `NAT/🔵️.ps1:238-246`; `setup git` writes `core.longpaths=true` `LIB/🎛️commands/🔧️setup/🟦️.ts:51-57` but is not invoked by the Windows bootstrap | Tracked-path measurement (section 4): max 226 UTF-16 units. Root must stay <=33 units. Gap G7. |
| Non-ASCII ancestry | none | none | refusal for workspace and the three blocking partitions, warning for `%TEMP%`: `BST/🛠️tools/🏗️bazel/🩺️readiness/🟦️.ts:35` | Only checks ancestry. Source file names with emoji (repo-wide) are not checked. Source label regex accepts U+0080-U+FFFF `BLD/🔧️tools/🏗️bazel/🧬️schema/🔣️.json` (Actions.sources.label). Bazel's acceptance of emoji package/file names is unverified (gap G10). |
| Locale for Bazel | not set | not set | not set | `LANG`/`LC_ALL=en_US.UTF-8` only on darwin: `BLD/🔧️tools/🏗️bazel/📜️script.ts:56,483`. Linux inherits the caller's locale; the devcontainer sets it in `.devcontainer/devcontainer.json:107-108` but the Dockerfile never generates the locale (gap G5). |
| Explicit "not qualified" refusals | none | none | `NB/🌐️platform/🟦️.ts:82` (protected-parent descendant inventory), `NB/📤️publication/🌐️platform/🟦️.ts:32` (protected maintenance inventory), `NB/📤️publication/🌐️platform/🟦️.ts:51` (exact-payload), `STO/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts:113` (graph exact-payload), `…/📤️publication/🟦️.ts:195` (dependency retirement; guard names `tools setup all`) | Windows cannot run protected maintenance/retirement. Whether the `tools setup all` caller reaches line 195 on Windows is unverified (caller not located in `LIB`). Gap G4. |
| Pinned tool matrix | Bazel 8.4.2 darwin arm64/x64; Bun 6+musl; binaryen 6; rustup triple mapping `BST/🛠️tools/🦀️rustup/📥️bootstrap/🟦️.ts:20` (darwin, win32-msvc, linux gnu/musl) | Bazel linux arm64/x64; Bun linux gnu+musl | Bazel win32 arm64/x64 (`.exe`); Bun win32 x64/arm64 | Coverage complete for the 6 combinations (Bazel `BST/🛠️tools/🏗️bazel/🧬️schema/🔣️.json` Release; Bun `…/🛠️tools/🟦️bun`; binaryen `…/🕸️wasm/🧬️schema/🔣️.json`). No gap. |
| Bazel adapter output | `…/📜️script.ts:482` `--output_user_root` under `.cache/b/o` | same | same | Windows output-root depth is not checked anywhere (gap G9). |
| Dependency install recipe | `BST/📦️dependencies/💾️storage/🧬️schema/📥️install.json:1717-1719`: `backend: hardlink`, `linker: isolated`, `lifecycle: ignore-scripts` | same | same | Cache (`.cache/b/d`, separate mount) and workspace are on different filesystems: hardlinks cannot cross devices. Bun's fallback to copy is unverified (gap G11). |
| Browser install | `BST/📦️dependencies/🏗️native/📜️script.ts:47` `playwright install chromium` (no `install-deps`) | same; system libraries needed (CI runs `ci-browser-prepare`) | `NAT/🔵️.ps1:382-388` sets Playwright cache to `node_modules\.cache\ms-playwright` | Windows violates `.devcontainer/README.md:85` (shared `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`). Gap G12. |
| Command shims | n/a | n/a | `.cmd` shim `@bun "%~dp0📜️script.ts" %*` is test-only (`LIB/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts:299`); CMake uses `cmd.exe` + `vcvarsall.bat` `BLD/🛠️cmake/🪟️toolchain/🟦️.ts:23` | Non-ASCII arguments through `cmd.exe` depend on the active code page (unverified, gap G13). |

## 2. First-run sequence from a fresh clone (only bun installed)

Entry for every platform: `bun repo run workspace:setup` (root `package.json:134-135`; devcontainer `devcontainer.json:38` postCreate).

| Step | Code | macOS | Linux native (non-root) | Linux native (root/sudo) | Devcontainer (`vscode`) | Windows native |
|---|---|---|---|---|---|---|
| S1 route | `BLD/📜️script.ts:90` preflight returns early; all else continues | ok | ok | ok | ok | ok |
| S2 leases image attach (every command) | `BLD/📜️script.ts:91` `openRepositoryCachePartitionV1(root,"leases")` | hdiutil attach 64 MiB, no privilege: ok | FAIL: native `mount -o loop` EPERM (no CAP_SYS_ADMIN); the readiness refusal runs only for `tools setup bazel` | ok (mount ns, root-owned mounts) | FAIL: CapEff has no CAP_SYS_ADMIN for uid 1000 | VHD create + `Mount-DiskImage`: FAIL unless elevated (warning only at `…/🩺️readiness/🟦️.ts:36`) |
| S3 dependencies image (8 GiB) + `ensureBuildEnvironment` | `BLD/📜️script.ts:128,130`, `BLD/🚀️bootstrap/📜️script.ts:20-22` | ok | blocked by S2 | ok | blocked by S2 | blocked by S2 unless elevated |
| S4 bun install (frozen lockfile, hardlink, ignore-scripts) | `install.json:1717-1719` | ok (network) | blocked | ok | blocked | ok (esbuild/sharp rely on optional packages; `ignore-scripts` is by design) |
| S5 `tools setup all`: Bazel first, then Nx | `BLD/📜️script.ts:131-144`; Bazel branch `:135-143`; readiness `:137` | ok, no refusals | refused by readiness (CAP, loop count, tools) | ok; opens tools 8 GiB + deps + products 24 GiB, downloads Bazel 8.4.2 | refused (see S2) | non-ASCII ancestry refusal if the checkout path has non-ASCII characters; else proceeds |
| S6 Nx installation pointer | `BST/🛠️tools/📜️script.ts:86-95` | ok | ok (as root) | ok | blocked | junction; non-atomic replace on retry |
| S7 playwright/chromium (`browsers` kind) | `BST/📦️dependencies/🏗️native/📜️script.ts:47` | ok | needs system libs (manual or `ci-browser-prepare`) | same | Dockerfile installs libs: ok | ok (Playwright cache path inconsistent, G12) |

Native bootstrap before S1 (what the user runs first):

- macOS: `NAT/🐚️.sh` `ensure_native_toolchain` (`:192-215`). Xcode CLT is manual (`:195`, "run xcode-select --install"). Homebrew is optional (`:196-199`). rustup is fetched with `curl … | sh` (`:236`, not checksummed). Bun is fetched with `curl https://bun.sh/install | bash` (`:344`, not checksummed). Then `repo_bootstrap` (`:356-367`) runs `bun repo run workspace:setup`.
- Linux native: `run_privileged` (`:139-145`) uses sudo, interactive by default. Then `repo_bootstrap` runs `bun repo run workspace:setup` without privilege (`:364`), so S2 fails for a non-root user. No loop/mount provisioning anywhere in `🐚️.sh` (grep for `usermod|setcap|losetup|mount`: 0 hits).
- Windows: `NAT/🔵️.ps1` winget machine installs with UAC (`:333-355`): Git, Bun (`Oven-sh.Bun`, latest, `:339`), Node LTS (`OpenJS.NodeJS.LTS`, `:340`; engines pins `24.15.0`, `package.json:129-130`), Go (latest, `:343`; devcontainer pins 1.26), VS Build Tools (`:351`, multi-GB, elevation), rustup, .NET 8/9/10. Upgrades to latest with `winget upgrade` (`:138-146`). LongPaths: warning only (`:238-246`). Repo-local Python processes are force-stopped (`:294-321`, `Stop-Process -Force` at `:316`). Playwright cache path inconsistent (`:382-388`). Then `bun repo run workspace:setup` (`:411`).

Devcontainer lifecycle: `.devcontainer/Dockerfile` pins bun 1.4.2 and node 24.15.0 with sha256 (`:4-5`, verified in `:80-90`), which is the strongest provisioning in the repo. postCreate fails at S2 (uid `vscode`, `remoteUser` `devcontainer.json:101`). The readiness remedy text (`…/🩺️readiness/🟦️.ts:39`, "rebuild the container") does not fix the remoteUser problem. Every container start also needs a privileged re-mount because mounts live in the container's mount namespace (inference, unverified).

## 3. CI workflows

File: `.github/workflows/architecture-course.yml` (the only workflow in the repo).

| Question | Answer | Evidence |
|---|---|---|
| Triggers | `workflow_dispatch` only (no push, no pull_request) | `:25` |
| Default-branch gate | release jobs run only on default branch | `:48,61,92,132` |
| OS coverage | Linux only. `configuration` and `pages` use `ubuntu-24.04` (`:49,118`); `check`, `site`, `proctor` use `vars.ARCHITECTURE_COURSE_RUNNER` (`:62,93,133`), which must be a Debian 12 x64 self-hosted runner with 72 GiB and 7 loops (course `📋️project.json` `metadata.semio.preflight`, `minimumAvailableBytes` 77309411328, `minimumFreeLoops` 7, `docker`) | `:52-57` |
| macOS / Windows jobs | none | - |
| Bazel job | none | - |
| Neutral entry | yes: `bun repo preflight`, `bun repo dependencies sync --all`, `bun repo run <project>:<target>` | `:78-88,107-111,150-161` |
| Privilege model | every step runs as `sudo --non-interactive env …` (about 20 steps). This is the only Linux path that works, which is the evidence for gap G1 | `:78` onward |
| Secrets/permissions | no stored secrets; OIDC for Pages; `contents: read` default | `:18-19,39-40` |

CI does not validate the devcontainer, the Windows bootstrap, the Bazel readiness refusals, or any non-Linux path.

## 4. Measurements

- Tracked files: 183,944 (`git ls-files`).
- Tracked relative paths with >=215 UTF-16 units: 34. Maximum: 226 (`.🧬semio/🦑️repo/🎫️tickets/…/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/…`, ticket fixture tree). Windows MAX_PATH 260: the checkout root (plus separator) must be <=33 units, e.g. `C:\Users\<name>\Documents\semio` fits only for short user names without `core.longpaths`/LongPathsEnabled.
- Native cache capacities (`STO/🔒️enforcement/📦️budget/🔣️policy.json`): rust 16 GiB (under the volume `.🧬semio/🦑️repo/⚡️cache/rust/fixed`), products 24 GiB, tools 8 GiB, dependencies 8 GiB, graph 4 GiB, diagnostics 2 GiB, nx 1.5 GiB, leases 64 MiB. Sum about 63.6 GiB; `maximumBytes` 64 GiB. Linux creates each image with `fallocate --length` (`NB/🧱️fixed/🟦️.ts:171`), so the space is allocated physically, not sparse.
- Devcontainer declares `hostRequirements.storage` = `32gb` (`.devcontainer/devcontainer.json:44`). Bazel needs tools+dependencies+products = 40 GiB, so the declaration is too small.
- Devcontainer storage placement: `.cache/b/*` (tools, dependencies, products, graph, nx, diagnostics, leases) sits under the bind mount `..:/workspaces/semio:cached` (`.devcontainer/docker-compose.yml:17`). Only `.🧬semio/🦑️repo/⚡️cache` is a named volume (`devcontainer.json:99`). The rust partition is on the volume but the others are on the host filesystem (gap G2).
- Duplicate native-build tree: `NB` is 141 files; the `📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage` copy is 32 files, each a one-line `export *` facade (verified for `🧱️fixed/🟦️.ts`, `🟦️.ts`, `🌐️platform/🟦️.ts`). Not a duplication defect.
- Root `package.json` runtime dependencies (`next` line 343, `pg-boss` 345, `sharp` 346, `esbuild` 342, `@napi-rs/canvas` 339) conflict with AGENTS.md "MUST NOT create runtime dependencies on external libraries" (governance, flag only; `package.json` owner decision).

## 5. Prioritized gap list with disjoint work packages

Priority: P0 blocks zero-touch on a platform; P1 causes false refusals or silent wrong behavior; P2 hygiene, supply chain, or unverified risk.

| ID | Pri | Gap | Evidence | Work package (owner files) |
|---|---|---|---|---|
| G1 | P0 | Linux non-root (native and devcontainer): no privileged path; every `bun repo` command except help/preflight fails at the leases image attach. Only CI works, via sudo. | `BLD/📜️script.ts:91`; `NB/🧱️fixed/🟦️.ts:171,231`; `BLD/🩺️preflight/🟦️.ts:33`; `…/readiness/🟦️.ts:40`; CI `:78` | WP-A (Linux privilege). Decision needed from the user: a narrow sudoers-allowlisted helper for `losetup/mount/umount/fallocate/mkfs.ext4` (recommended) vs root-only. No FUSE path exists (ruled out earlier). |
| G2 | P0 | Devcontainer: `remoteUser: vscode` never gets CAP_SYS_ADMIN, so postCreate fails. Remedy text says "rebuild the container", which does not help. `.cache/b` is on the bind mount, not the volume. Declared storage 32 GiB < 40 GiB Bazel images. `en_US.UTF-8` not generated. | `.devcontainer/docker-compose.yml:13,17`; `devcontainer.json:44,99,101,107`; `Dockerfile:96`; `…/readiness/🟦️.ts:39` | WP-B (devcontainer). Options: `containerUser`/`remoteUser` root (no), helper from WP-A, move native partitions into the volume, raise storage, add `locale-gen`. |
| G3 | P0 | Windows elevation: VHD mount needs an elevated shell (HRESULT 0x80070522), but readiness only warns, so the first command fails natively. Images are detached at reboot (inference). | `…/readiness/🟦️.ts:36`; `NB/🧱️fixed/🟦️.ts:171` (`Mount-DiskImage`, `Initialize-Disk`) | WP-C (Windows image backend): refuse early with a precise remedy, or self-elevate in one step. Coordinate `readiness/🟦️.ts` with WP-A (WP-A owns that file). |
| G4 | P1 | Windows protected maintenance, protected-parent inventory, exact-payload and dependency retirement are explicit refusals; `tools setup all` may reach line 195. | `NB/🌐️platform/🟦️.ts:82`; `NB/📤️publication/🌐️platform/🟦️.ts:32,51`; `STO/…/📤️publication/🟦️.ts:113,195` | WP-C (Windows image backend) for the implementation; the caller of line 195 must be located first (not in `LIB`). |
| G5 | P1 | Locale: Bazel `LANG`/`LC_ALL` forced only on darwin; devcontainer declares en_US.UTF-8 without generating it. | `BLD/🔧️tools/🏗️bazel/📜️script.ts:56,483`; `devcontainer.json:107-108`; `Dockerfile` (no locale) | WP-D (Bazel adapter + Dockerfile). |
| G6 | P1 | Linux loop preflight counts only existing unused `/dev/loopN` nodes; `losetup`/`mount` allocate new nodes through loop-control. Hosts with snap loops refuse falsely. Thresholds 3 (Bazel) and 7 (course). | `BLD/🩺️preflight/🟦️.ts:50-51`; `…/readiness/🟦️.ts:58` | WP-A (preflight/readiness): probe `/dev/loop-control` allocatable count, not existing nodes. |
| G7 | P1 | Windows long paths: no readiness check for LongPathsEnabled or `core.longpaths`; 34 tracked paths reach 215-226 units; bootstrap warns only; `setup git` sets `core.longpaths` but is not part of the Windows bootstrap. | `NAT/🔵️.ps1:238-246`; `LIB/🎛️commands/🔧️setup/🟦️.ts:51-57` | WP-A adds a readiness fact; WP-F wires `core.longpaths` into the Windows bootstrap (`NAT/🔵️.ps1`). |
| G8 | P1 | Windows Nx installation pointer replace is not atomic (unlink, then rename, with a window where the pointer is missing). | `BST/🛠️tools/📜️script.ts:86-95` | WP-E (Nx installation). |
| G9 | P2 | Bazel `output_user_root` depth on Windows is not checked. | `BLD/🔧️tools/🏗️bazel/📜️script.ts:482` | WP-D (Bazel adapter); verify on Windows. |
| G10 | P2 | Emoji in Bazel inputs: readiness checks only ancestry; label regex admits non-ASCII; Bazel's own acceptance is unverified. | `BST/🛠️tools/🏗️bazel/🩺️readiness/🟦️.ts:35`; Bazel `Actions.sources.label` | WP-D: verify with a Bazel run on Windows and macOS with emoji source paths. |
| G11 | P2 | `hardlink` backend with cache on a separate mount: cross-device hardlinks cannot succeed; fallback behavior unverified. | `install.json:1717`; partition layout `STO/…/🔣️policy.json` | WP-G (dependency install recipe; deps owner). |
| G12 | P1 | Windows Playwright cache at `node_modules\.cache\ms-playwright` violates the repo rule `.devcontainer/README.md:85`. | `NAT/🔵️.ps1:382-388` | WP-F. |
| G13 | P2 | `cmd.exe`/`.cmd` argument handling with non-ASCII arguments (test shims and CMake `vcvarsall`). | `LIB/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts:299`; `BLD/🛠️cmake/🪟️toolchain/🟦️.ts:23` | WP-E/WP-C; low priority. |
| G14 | P2 | Supply chain: `curl | sh` (rustup, `NAT/🐚️.sh:236`), `curl | bash` (bun, `:344`), dotnet installer (`:321`), not checksummed; Dockerfile verifies sha256. | `NAT/🐚️.sh` | WP-F: pin and verify, as the Dockerfile does. |
| G15 | P2 | Windows bootstrap installs unpinned latest tools (Bun, Node LTS, Go, `winget upgrade`), so Bun/Node/Go differ from the pins (Bun 1.4.2, Node 24.15.0, Go 1.26). | `NAT/🔵️.ps1:138-146,339-343` | WP-F. |
| G16 | P2 | Windows bootstrap force-stops repo-local Python processes without confirmation. | `NAT/🔵️.ps1:294-321` (`Stop-Process -Force` at 316) | WP-F: confirm or scope narrower. |
| G17 | P2 | Permanent `[DEBUG] ` log in preflight (AGENTS: debug logs removed before finishing). | `BLD/🩺️preflight/🟦️.ts:54` | WP-A (one line). |
| G18 | P2 | macOS readiness is a no-op (no hdiutil or APFS check). Low risk. | `…/readiness/🟦️.ts:34,38` (darwin not handled) | WP-A, optional. |
| G19 | P2 | `.bazelrc` `build --symlink_prefix=/` may need symlink privilege on Windows. | `.bazelrc:5` | WP-D; verify. |
| G20 | P2 | Dependency sync `browsers` runs without system library install on native Linux. | `BST/📦️dependencies/🏗️native/📜️script.ts:47` | WP-F documentation or `install-deps` step. |
| G21 | P1 | CI: Linux only, manual trigger only, no PR gate, no macOS/Windows, no Bazel, no devcontainer or bootstrap check; every step under sudo (root-owned caches). | `.github/workflows/architecture-course.yml:25,49,62,78` | WP-G2 (CI). Needs user decisions on runners and cost. |

Disjoint work packages (file ownership):
- WP-A (Linux privilege and readiness): `BLD/🩺️preflight/🟦️.ts`, `BST/🛠️tools/🏗️bazel/🩺️readiness/🟦️.ts`, the Linux branch of `NB/🧱️fixed/🟦️.ts`, and the gate at `BLD/📜️script.ts:91`.
- WP-B (devcontainer): `.devcontainer/**`, and the partition directories in `STO/🔒️enforcement/📦️budget/🔣️policy.json` (only after the decision).
- WP-C (Windows image backend): `NB/🧱️fixed/🗂️system/🟦️.ts`, `NB/🪟️image/🟦️.ts`, the Windows branches of `NB/🧱️fixed/🟦️.ts`, and the Windows refusals in `NB/🌐️platform/🟦️.ts` and `NB/📤️publication/🌐️platform/🟦️.ts`.
- WP-D (Bazel adapter): `BLD/🔧️tools/🏗️bazel/📜️script.ts`, `.bazelrc`, Bazel schema.
- WP-E (Nx installation): `BST/🛠️tools/📜️script.ts` (lines 86-95 and the Windows branch).
- WP-F (native bootstrap and setup): `NAT/🐚️.sh`, `NAT/🔵️.ps1`, `LIB/🎛️commands/🔧️setup/🟦️.ts`.
- WP-G (dependencies): `BST/📦️dependencies/**` (`install.json` recipe; deps owner).
- WP-G2 (CI): `.github/workflows/**`.
- Not owned by any package: root `package.json` (runtime dependencies, G-governance), needs the deps owner.

## 6. Verified versus unverified

Verified by reading code or read-only measurement:
- Each refusal, branch, and path cited above (file:line).
- Tracked-path lengths and counts (`git ls-files`).
- Policy capacities (JSON read).
- CI triggers, runners, and sudo wrappers.
- Devcontainer `remoteUser`, `privileged`, mounts, storage, locale settings.

Not run (no execution on any OS, no `bun repo` run, no container):
- Linux non-root failure at S2 (inferred from the code path and the CapEff check; not executed).
- Windows elevation failure (inferred; the warning text cites 0x80070522).
- Bun hardlink behavior across devices (G11).
- Bazel emoji package names (G10) and `symlink_prefix` on Windows (G19).
- Docker Desktop loop devices on the bind mount (G2; not claimed, needs a run).
- Caller of Windows-refused dependency retirement (G4).
- macOS TCC prompts for `~/Documents` (not claimed here; expected only from macOS policy, not verified).

## 7. Open decisions for the user

1. Linux privilege model (G1, G2): sudoers-allowlisted helper for the five image commands (recommended), or root-only execution. No unprivileged mount path exists.
2. Devcontainer storage placement and size (G2): move `.cache/b` into the volume and raise `storage` to at least 48 GB, or keep the bind mount and accept the risk.
3. Windows: refuse early with remedy vs self-elevate for the VHD attach (G3).
4. CI matrix: add macOS and Windows jobs and PR triggers (G21), and their cost.
