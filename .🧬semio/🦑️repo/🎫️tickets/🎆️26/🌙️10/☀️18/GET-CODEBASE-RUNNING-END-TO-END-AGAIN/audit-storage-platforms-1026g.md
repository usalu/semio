# Native storage per platform: audit 1026g (aud-store-g)

Date: 2026-10-26. Auditor: aud-store-g (read-only). Nothing was built, run, edited or committed. Only this file was written. Read-only probes: `ls`, `stat` on `.cache/b/*/fixed.dmg`, `/sbin/mount`, `rg` and `sed` on sources and tests.

Base for relative paths:
`S` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`
Paths are relative to `S` unless shown absolute. `NB` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build`.

---

## 1. Summary

- Two backends are declared in `S/🔌️backends/🧬️schema/🔣️.json`:
  - `mounted-image`: darwin only. A UDRW/UDIF `fixed.dmg` holding an APFS volume, attached with `hdiutil`. Kernel-enforced size cap. Capabilities: kernel-quota, mounted-volume, volume-events.
  - `directory-partition`: darwin, linux, win32. A plain directory tree swapped in and out atomically. No quota. Capability: declared-envelope.
- Selection (`S/🔌️backends/🧭️selection/🟦️.ts`) prefers `kernel-quota`. So macOS gets the DMG by default, and Linux, the devcontainer and Windows always get directory partitions.
- Linux and devcontainer: no loop device, no mount, no quota. The swap is `renameat2(RENAME_EXCHANGE)` through Bun FFI. The libc path is hardcoded to the Debian/Ubuntu multiarch layout, so Fedora, Arch, NixOS and Alpine refuse. Node needs an external FFI provider.
- Windows: journaled write-through swap through Win32/NT FFI (Bun only). Per-directory case sensitivity needs the optional WSL feature and a restart, which breaks the zero-touch rule. There is no hard cap.
- Disk: the DMGs are fully preallocated. Measured logical size equals allocated size. Total is 75.67 GiB across three cache roots, with 16 `.cache/b/*/m` mounts attached at check time.
- Crash: mounts outlive a killed process. Cleanup is a sweep at most every 300 s of scratch-root and removed-backing images, skipping busy ones. A `noowners` parent or a foreign-owned service directory needs a manual `hdiutil detach`. Real-kernel kill tests around hdiutil do not exist.

---

## 2. Platform matrix

| Platform | Default backend | Swap / attach primitive | Capacity enforcement | Runtime or OS needs | Real-kernel test coverage |
|---|---|---|---|---|---|
| macOS | mounted-image | `/usr/bin/hdiutil attach -nobrowse -noautoopen -owners on`; `hdiutil detach`. Directory partitions: `renameatx_np(RENAME_SWAP=2)` from `/usr/lib/libSystem.B.dylib` | Kernel: APFS volume size equals DMG size | Bun FFI for the swap and the volume-event witness; `codesign` for executables | Yes: real hdiutil in `🍎️image/*/🧪️tests`, `🩺️heal` test, and the conformance suite (both backends) |
| Linux (native) | directory-partition | `renameat2(RENAME_EXCHANGE=2)` via `dlopen("/lib/<triplet>/libc.so.6")` | None in kernel. Declared envelope plus tree walk. Host free space via `statfs` | Bun FFI. Under Node, `SEMIO_RUNTIME_NATIVE_PROVIDER` must name a provider module | Partial: Python ctypes oracle (silently skipped without Python), conformance (needs `python3`), crash enumeration |
| Devcontainer (ubuntu-24.04) | directory-partition (Linux path, `DEVCONTAINER=true`) | Same as Linux, on the `.cache` named volume | Same as Linux | No capabilities, no loop device, no privileged mode | Not in CI. No devcontainer job |
| Windows | directory-partition | Journaled swap: `swap.json` intent plus `m.old`, `MoveFileExW` with write-through. Case flag via `NtSetInformationFile` class 71 | Declared envelope. `GetDiskFreeSpaceExW` for host free space | Bun FFI (kernel32, ntdll, advapi32), a valid `SystemRoot`, WSL optional feature for case sensitivity, PowerShell for boot time | Fake-tree tests, plus CI `windows-latest` only. Crash tests skipped on win32 |
| FreeBSD and others | none | Refused: "Directory partitions require Linux, macOS or Windows" (`S/📁️directory/🟦️.ts:25`) | n/a | n/a | n/a |

---

## 3. macOS-only call sites

No `clonefile` call exists in `S`. An explicit APFS clone path does not exist in this module (see the reflink note at the end of this section).

| # | Location | What it does |
|---|---|---|
| 1 | `S/🍎️image/🛠️hdiutil/🟦️.ts:2` | Single source of the program path, `/usr/bin/hdiutil` |
| 2 | `S/🍎️image/🟦️.ts:33` | `hdiutil info -plist` (image roster), via `captureOwnedCommandV1` |
| 3 | `S/🍎️image/🟦️.ts:41, 53, 65` | darwin gate: "macOS fixed image enforcement is unavailable on this platform" (retire, retire-with-writer, create) |
| 4 | `S/🍎️image/🟦️.ts:46, 56` | `hdiutil detach <root>` (no `-force`) |
| 5 | `S/🍎️image/🟦️.ts:66-67` | `statfs` and device identity of the mounted volume |
| 6 | `S/🍎️image/👤️owners/🟦️.ts:5` | Attach arguments: `-nobrowse -noautoopen -owners on -mountpoint` (`attachArguments` in `S/🧬️schema/🔣️.json`) |
| 7 | `S/🍎️image/👤️owners/🟦️.ts:18` | Refusal text telling the user to run `/usr/bin/hdiutil detach` by hand |
| 8 | `S/🍎️image/🤫️silence/🟦️.ts:69-72` | `hdiutil create -size <N>m -format UDRW -nospotlight -fs "Case-sensitive APFS" -volname semio -srcfolder <seed> <image>` |
| 9 | `S/🍎️image/🤫️silence/🟦️.ts:47` | Refusal text: foreign-owned service directory, detach by hand |
| 10 | `S/🍎️image/🤫️silence/🟦️.ts` (seed) | Seed markers `.fseventsd/no_log` and `.metadata_never_index` (`S/🍎️image/🤫️silence/🧬️schema/🔣️.json`) |
| 11 | `S/🍎️image/📏️capacity/🟦️.ts:5-` | APFS container overhead model (predicts statfs bytes of an empty volume) |
| 12 | `S/🔌️backends/🍎️mounted-image/📐️geometry/🟦️.ts:13-18` | `statfs` figures; line 18 runs `captureOwnedCommandV1("hdiutil", ...)` with a bare name (PATH lookup, see G5) |
| 13 | `S/🔌️backends/🍎️mounted-image/✍️sealing/🟦️.ts:6` | `spawnSync("codesign", ["--force","--sign","-", binary])`, bare name, untested (see G5) |
| 14 | `NB/🟦️.ts:74-76, 85-88` | Ad-hoc signing of every distribution copy through `sealMacExecutableV1` (no-op off darwin) |
| 15 | `S/🧹️cleanup/🎛️command/🟦️.ts:75` | Create, activate and inspect bodies: `nativeImageHdiutilProgramV1` and `mountProgram` `/sbin/mount` for the mount table |
| 16 | `S/🧹️cleanup/🎛️command/🟦️.ts:83` | `spawn(nativeImageHdiutilProgramV1, args, {env: PATH=/usr/bin:/bin:/usr/sbin:/sbin})` |
| 17 | `S/🧹️cleanup/🎛️command/🟦️.ts:138-140` | `readNativeBootSessionV1` and `nativeBootSessionV1()` for receipts (`kern.bootsessionuuid`) |
| 18 | `S/🧹️cleanup/🎛️command/🟦️.ts:171, 224, 231` | darwin gates for persistent native records and protected parents |
| 19 | `S/📤️publication/🎛️owner/🟦️.ts:73-76` | Detach, attach and info commands spawned with the hdiutil spec; `:80, 94, 97` darwin checks |
| 20 | `S/🌐️platform/🥾️boot/🟦️.ts:17` | darwin: `/usr/sbin/sysctl -n kern.bootsessionuuid`. linux: `/proc/sys/kernel/random/boot_id`. win32: throws (latent, see G12) |
| 21 | `S/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🟦️.ts:25` | darwin: `/usr/sbin/sysctl -n kern.boottime` |
| 22 | `S/🌐️platform/🥾️boot/🩹️recovery/📋️plan/🟦️.ts:105` | darwin boot epoch parse |
| 23 | `S/🌐️platform/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts:58` | Kernel roster read: `hdiutil info -plist` |
| 24 | `S/🌐️platform/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts:90` | `hdiutil detach <device>` (sweep of leaked images) |
| 25 | `S/🌐️platform/🥾️boot/🩹️recovery/🧬️schema/🔣️.json` `leakedImages` | Scratch roots `/private/tmp`, `/private/var/folders`, `/var/tmp`, `/tmp`, plus `$TMPDIR` and `SEMIO_TEST_ARTIFACT_DIR`; sweep interval 300000 ms |
| 26 | `S/📤️publication/📚️witness/🍎️events/🟦️.ts:14, 19, 32, 41` | darwin-fsevents volume-event witness, Bun FFI only (`🍞️bun/🟦️.ts:3`) |
| 27 | `S/🧹️whole/🟦️.ts:67, 73` | darwin-only ancestry binding and evidence names. Other platforms use the anchor file |
| 28 | `S/🧹️cleanup/♻️retirement/📋️contract/🟦️.ts:50, 56` | darwin-only path bounds (256 units) and terminal descriptor phases |
| 29 | `S/🧹️cleanup/📋️state/🟦️.ts:84` | `refuse-platform` for any non-darwin native image |

Reflink note. `NB/🗃️stage-cache/🟦️.ts:89` uses `copyFile(..., COPYFILE_FICLONE)`. Under libuv this is best-effort clone (on APFS, `copyfile` with `COPYFILE_CLONE`; on Linux, FICLONE; on NTFS, a silent copy). It is not in `S`. `NB/🟦️.ts:86` is a plain `copyFileSync`. The README's reflink claim is therefore implemented in stage-cache and the library store, not in the storage layer.

RENAME_SWAP note. There is no RENAME_SWAP on HFS+ or exFAT. `renamex` then fails with errno 45 (ENOTSUP) or 22. The code refuses with "Atomic directory exchange is unsupported", with no fallback (`S/📁️directory/🔀️exchange/🟦️.ts:28`, errno list in `S/📁️directory/🧬️schema/🔣️.json` `exchange.darwin.unsupported`).

---

## 4. What Linux, Windows and the devcontainer do instead

### Linux (native) and devcontainer

- Port selection: `S/📁️directory/🟦️.ts:23` (darwin or linux get the exchange port).
- Swap: `S/📁️directory/🔀️exchange/🟦️.ts:16-19`. The library is chosen per arch: `/lib/x86_64-linux-gnu/libc.so.6` or `/lib/aarch64-linux-gnu/libc.so.6`, symbol `renameat2`, flag 2 (RENAME_EXCHANGE), errno 95, 38, 22 map to "unsupported". Bun FFI through `runtimeNativeSystemV1()`.
- Layout: `fixed.dir` is an anchor JSON file (identity of the placeholder directory). `v` is the volume and `m` the root (`S/📁️directory/🧬️schema/🔣️.json`). Attach is one exchange (`S/📁️directory/🟦️.ts`, `activateNativeDirectoryV1`).
- Capacity: no kernel quota. The meter (`S/📁️directory/🟦️.ts`, `createNativeDirectoryMeterV1`) re-measures the tree every 16 MiB of admissions or 2 s, and on commit or recheck. The walk is `S/📁️directory/📏️usage/🟦️.ts`: lstat, concurrency 16, batch 8, at most 1,048,576 entries, depth 64.
- Host free space: `S/📁️directory/🟦️.ts:43` (statfs). Declared envelope as capacity: `S/🔌️backends/📁️directory-partition/📐️geometry/🟦️.ts`. The code states it is advisory: "An ordinary directory has no portable aggregate physical quota" (`S/🟦️.ts:50`).
- Child environment: `S/📁️directory/🟦️.ts:36`, fixed PATH `/usr/bin:/bin`, LANG=C.
- Boot time: `S/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🟦️.ts:25` reads `/proc/stat` btime and `/proc/1/stat` for the PID-namespace offset. This is relevant in containers.
- No mount, so no leaked-image sweep. `healNativeLeakedImagesV1` only runs when `nativeStorageHostMountsVolumesV1()` is true (`S/🌐️platform/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts`).
- Devcontainer: `.devcontainer/docker-compose.yml` mounts the checkout bind-mounted, but the cache goes to the named volume `semio-native-cache-${devcontainerId}` at `.cache` (`.devcontainer/devcontainer.json:151-152`). The compose file states: no capability, no loop device, no mount. `repo preflight` refuses a case-insensitive host mount. `hostRequirements.storage` is 160 GB.

### Windows

- Port selection: `S/📁️directory/🟦️.ts:24` gives `createNativeDirectoryJournaledPortV1(nativeWindowsSystemHostPortV1())`.
- Journal: `S/📁️directory/🪟️journal/🟦️.ts`. Intent `swap.json`, temp `m.old`. `:16` refuses a failed move with "Directory rename refused by Windows". `:21` tolerates flush codes 1 and 5. `:53` aggregates rollback failures. Interrupted swaps refuse with `interrupted` until the lease holder calls `recoverNativeDirectoryV1`.
- Win32 surface: `S/📁️directory/🪟️host/🟦️.ts:68-69` (FFI guard and kernel32 `MoveFileExW`, `CreateFileW`), `:89` (advapi32 registry for `LongPathsEnabled`). The extended path prefix `\\?\` and a 32767-unit bound are in the schema.
- Case sensitivity: `S/📁️directory/🪟️host/🟦️.ts` and the refusals in `S/📁️directory/🧬️schema/🔣️.json`. `caseBuild` needs build 17107 or newer. `caseFeature` says to enable the optional feature Microsoft-Windows-Subsystem-Linux with dism and restart. `caseAccess` and `caseUnverified` also exist.
- Free space: `S/📁️directory/🟦️.ts:44` (GetDiskFreeSpaceExW).
- Boot time: `S/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🟦️.ts:25` runs `powershell.exe -Command (Get-CimInstance Win32_OperatingSystem).LastBootUpTime...`, a bare name.
- Sharing: a sharing violation (Win32 error 32) becomes the refusal `windowsInUse`, which asks the user to close every handle. `requireNativeDirectoryUnusedV1` probes with share mode 0.
- Crash-enumeration tests skip win32 (`S/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/💥️crash/🟦️.ts:31`).

---

## 5. Gaps and recommendations

Each gap has one recommendation.

| ID | Platform | Gap (evidence) | Recommendation |
|---|---|---|---|
| G1 | Linux (Fedora, RHEL, Arch, NixOS, Alpine) | libc path is hardcoded to `/lib/<triplet>/libc.so.6` (`S/📁️directory/🧬️schema/🔣️.json` `exchange.linux`; `S/📁️directory/🔀️exchange/🟦️.ts:17`). Those distros have no such path, Alpine has no glibc at all, and any non-x64/arm64 arch falls into the x64 path with the misleading message "lacks renameat2" | `dlopen("libc.so.6")` by soname so the loader searches, map unsupported arch to an explicit refusal, and add a per-distro test table |
| G2 | Linux and Windows under Node | No FFI in Node. `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🟦️.ts` (repo-relative) needs `SEMIO_RUNTIME_NATIVE_PROVIDER`. No default, and CI does not set it | Refuse at storage entry with one tested message, or declare Bun the only storage runtime and check it in `repo preflight` |
| G3 | Windows (zero-touch) | Per-directory case sensitivity needs the WSL optional feature, a restart and NTFS. Refusal `caseFeature` appears mid-build. AGENTS.md requires zero-touch | Detect it in `repo preflight` once with exact steps, or place the cache on a case-sensitive location automatically; never refuse mid-build |
| G4 | Linux and Windows | No hard cap. The envelope is advisory: overshoot is up to 16 MiB plus 2 s plus any writer outside the admission path, and the host disk can fill | Add a lock-aware GC threshold at about 90 % of the envelope, and a test that races an out-of-band writer |
| G5 | macOS | Bare-name calls: `S/🔌️backends/🍎️mounted-image/📐️geometry/🟦️.ts:18` runs `"hdiutil"` and `S/🔌️backends/🍎️mounted-image/✍️sealing/🟦️.ts:6` runs `"codesign"`, so both depend on PATH. The rest uses `/usr/bin/hdiutil`. `sealMacExecutableV1` has no test | Route both through absolute constants (`nativeImageHdiutilProgramV1`, `/usr/bin/codesign`) and add a darwin test for sealing |
| G6 | Windows | Boot time spawns `powershell.exe` (bare name, Get-CimInstance) on every process start. Cost not measured | Use an absolute System32 path and measure; or read boot time through the existing Win32 host without a spawn |
| G7 | Windows | Antivirus or indexer handles cause `windowsInUse` as a hard refusal with no retry | Add bounded backoff retry with EN and DE progress before refusing, and confirm on `windows-latest` (unverified) |
| G8 | macOS (directory-partition) | Needs a case-sensitive host. Default APFS is case-insensitive, so the probe refuses. The conformance suite builds a case-sensitive DMG to test it (`S/🔌️backends/🧪️tests/✅️conformance/🟦️.ts`) | Document it and have `repo preflight` say so |
| G9 | Linux, Windows, macOS | No fallback when RENAME_EXCHANGE is missing: vfat, exFAT, SMB, FUSE, 9p or virtiofs bind mounts, HFS+. Refusal by design (errno 22, 38, 45, 95) | Keep the refusal, but have storage setup probe the `.cache` location once and print the remedy (named volume or local path, as the devcontainer does) |
| G10 | macOS (crash, reboot) | A `noowners` parent or a foreign-owned service directory blocks nested attach until a human runs `hdiutil detach` (`S/🍎️image/👤️owners/🟦️.ts:18`, `S/🍎️image/🤫️silence/🟦️.ts:47`) | Let heal detach an unowned parent automatically when its lease is free, keeping the message only as last resort |
| G11 | All | Test gaps: (a) `S/📁️directory/🪟️host/🧪️tests/🟦️.ts:100` returns early off win32, so the host test is vacuous on Linux and macOS; (b) `S/📁️directory/🧪️tests/🟦️.ts:55-56` silently skips the Linux exchange oracle when `testPythonV1()` returns null (`🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts:49`, repo-relative); (c) conformance hard-requires `python3 -I` for the byte walk, which fails on Windows without `python3`; (d) crash enumeration skipped on win32; (e) `sealMacExecutableV1` untested; (f) no test sets `SEMIO_RUNTIME_NATIVE_PROVIDER`; (g) no real kill injection around `hdiutil create`, `attach` or `detach` | Replace Python with a Bun-only walker, use `test.skipIf` with a visible reason instead of silent returns, add a darwin CI crash job, and test sealing and the provider path |
| G12 | Windows (latent) | `S/🌐️platform/🥾️boot/🟦️.ts:17` throws on win32 by design. Only the persistent darwin path calls `nativeBootSessionV1()` (`S/🧹️cleanup/🎛️command/🟦️.ts:138-140`) | Keep it and add a test asserting that no win32 path reads the boot session |
| G13 | All (CI) | No storage-specific CI job. Storage tests run only if `repo affected` selects them per OS (`.github/workflows/build-system.yml`, jobs smoke and affected). No devcontainer job | Add a storage job on linux, macos and windows running the storage projects, and a devcontainer job |
| G14 | macOS (disk) | DMGs are fully preallocated (UDRW), and each cache root duplicates the set (section 6) | Switch to sparse bundles (UDSB) or size each image from its declared need, and put all roots under one budgeted cache |
| G15 | macOS (RAM) | RAM cost of N attached images (APFS containers, `hdiutil info` polling) was not measured | Measure footprint with 16 attached images on the 16 GB target before adding more images |

---

## 6. Disk and memory cost of the image model (measured)

Method: `stat -f '%z %b'` on each `fixed.dmg` (block unit 512 bytes). Logical size equals allocated size in every case, so the images are preallocated. `ls -ls` showed the same.

| Root | Partition | Logical bytes | Allocated bytes |
|---|---|---|---|
| repo `.cache/b` | p | 25,768,755,200 | 25,768,755,200 |
| repo `.cache/b` | d | 8,588,886,016 | 8,588,886,016 |
| repo `.cache/b` | t | 8,588,886,016 | 8,588,886,016 |
| repo `.cache/b` | g | 6,441,402,368 | 6,441,402,368 |
| repo `.cache/b` | o | 2,146,435,072 | 2,146,435,072 |
| repo `.cache/b` | n | 1,609,564,160 | 1,609,564,160 |
| repo `.cache/b` | l | 66,060,288 | 66,060,288 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/.cache/b` (library) | l, o | 2,212,495,360 | 2,212,495,360 |
| `~/.sinc/.cache/b` | d, g, l, o, t | 25,831,669,760 | 25,831,669,760 |
| Total | 3 roots, 14 images | 81,254,154,240 (75.67 GiB) | same |

Notes:
- Disk: a DMG costs its full declared size from creation. Each cache root duplicates the partition set (`l`, `o`, `d` and others), so the same owner pays in every root.
- The repo-root total is about 49.6 GiB. The brief's "`.cache/b` about 150 GiB" includes non-image content. That content was not measured here, because `du` over `.cache/b` timed out.
- Attached at check time: `/sbin/mount` lists 16 `.cache/b/*/m` mounts across the three roots.
- RAM: not measured (G15). Attached DMGs add APFS containers. The roster is shared per process through `observeSharedNativeImageInventoryV1`.
- Directory partitions: no preallocation. Disk cost equals used bytes. Enforcement cost is a tree walk (up to 1,048,576 entries, concurrency 16) every 16 MiB of admissions or every 2 s. CPU and I/O not measured.

---

## 7. Mounted images on crash and reboot (what the code does)

- Crash of semio during a run: `hdiutil attach` is a child process, and the kernel mount outlives its parent.
- Next process start: `healNativeProcessBootV1` (`S/🌐️platform/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts:113`) runs once per process. `healNativeToolStateBootV1` runs once per boot, with its ledger in `.cache/boot-recovery`.
- `healNativeLeakedImagesV1` runs at most every 300 s (stamp file). It detaches only images whose backing lies under a scratch root, or whose backing file is gone. It requires the image lease probe to be free. It calls `hdiutil detach <device>` without `-force`. A busy image is left for the next run (`S/🌐️platform/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts:90`).
- Persistent partitions (`.cache/b/<x>`) are meant to stay attached. They are reattached on open with the attach arguments (`S/🍎️image/👤️owners/🟦️.ts:5`).
- Reboot: no code path detaches at shutdown. The code assumes mounts are gone after boot, and the per-boot ledger reruns. Whether macOS remounts DMGs at login is UNVERIFIED. I expect it does not.
- A kill during `create` or `attach` is covered only by heal test scaffolds (`S/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🩺️heal/🟦️.ts:80-87`) and the generic crash enumeration of journal scenarios. Neither kills real hdiutil in the middle of an operation.
- Linux: no mount, so nothing leaks. `renameat2` cannot tear, and state is derived from identity (`S/📁️directory/🟦️.ts`, `observeNativeDirectoryStateV1`). Data written before a crash stays, since there is no quota.
- Windows: a pending journal makes state `interrupted` until the lease holder runs `recoverNativeDirectoryV1`. Crash enumeration does not run on Windows.

---

## 8. Test coverage per platform

| Platform | Real-kernel tests | Pure or fake tests | Skipped or vacuous branches |
|---|---|---|---|
| macOS | hdiutil create, attach, detach, info (`🍎️image/*/🧪️tests`, `🩺️heal`); conformance for both backends; crash enumeration (POSIX) | Selection and dispatch are pure data (`S/🔌️backends/🧪️tests/🧭️selection/🟦️.ts`) | `sealMacExecutableV1` untested |
| Linux | Python ctypes `renameat2` oracle (silently skipped without Python); conformance needs `python3`; crash enumeration; `statfs` | Table of unsupported errnos per platform (`S/📁️directory/🧪️tests/🟦️.ts:40`) | Oracle skipped when Python absent. No libc-path test |
| Windows | CI `windows-latest` only. Real Win32 conditional-retirement tests behind `windowsTest` (`S/🪟️conditional-retirement/🧪️tests/🟦️.ts:62`) | Fake tree (most of the suite) | Crash tests skipped; host test returns early off win32 |
| Devcontainer | None | As Linux | No CI job |

---

## 9. Not checked

- No build, test run or source edit (rule). No `hdiutil info` (an earlier batch timed out), no `du` over `.cache/b`, no run on Fedora, Alpine, Windows or in the devcontainer.
- The libc path and the errno lists are read from the schema, not executed on those hosts.
- Reboot behaviour of DMGs (section 7) is unverified.
- libuv mapping of `COPYFILE_FICLONE` on macOS is from memory of libuv, not checked in source.
- The Python helper returns null when Python is absent (`testPythonV1` is typed `string|null`); this is inferred from the signature.
