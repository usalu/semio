# Result xplat-linux: unprivileged directory-backed Linux partitions (WP-A, G1/G2/G6/G17/G18)

Date 2026-10-19. No git-modifying command, no sub-agent, no cargo build. Abbreviations: `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`, `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `BST` = `LIB/⚡️caching/🚀️bootstrap`, `BLD` = `LIB/🏗️build`.

## Design (schema-first)

A Linux partition `.cache/b/<p>` is now a directory, no image, no mount:

```
fixed.dir   anchor file (JSON: kind, filesystemBytes, maximumPhysicalBytes, underlying placeholder identity)
image.json  receipt (unchanged schema, platform "linux", dev/ino/size of the anchor)
m           the root: the empty placeholder while inactive, the volume while attached
v           the volume while inactive, the placeholder while attached
```

- Attach and detach are one `renameat2(RENAME_EXCHANGE)` of `m` and `v` through `bun:ffi` on libc (macOS uses the already documented `renameatx_np(RENAME_SWAP)`). State is derived from which name still holds the placeholder inode recorded in the anchor, so the old mounted/inactive state machine (publication, retained inactive births, cleanup) keeps its meaning. EINVAL/ENOSYS/ENOTSUP/EOPNOTSUPP produce the precise refusal "Atomic directory exchange is unsupported by this kernel or filesystem".
- The image tools (`fallocate`, `mkfs.ext4`, `mount`, `losetup`, `umount`) are replaced one to one by closed actions of a first-party child process (`NB/🐧️directory/📜️script.ts`: `allocate`, `activate`, `inactive`, `deactivate`), spawned through the same owned-command, cancellation and cleanup paths as before. Observation is in-process (stat, no `/proc/self/mountinfo`, no sysfs).
- Capacity is accounting: the journal admits reservations against the declared envelope (as before); every commit of an allocation of 16 MiB or more first measures the real tree (`measureNativeDirectoryUsageV1`: allocated blocks, links not followed, hard links once, bounded) and refuses above the envelope (`NativeStoragePortV1.measure`, optional, implemented only by directory partitions). Space queries that used `statfs` (rebudget, Nx graph headroom, products layout, graph headroom) go through `nativeImageSpaceV1` / `nativeImageUsedBytesV1`, which are envelope-bound on Linux and unchanged elsewhere.
- Case sensitivity is probed at allocation (`a`/`A` pair) and by preflight. Ownership, leases, borrowers, receipts and cleanup protocols are the existing ones; nothing was weakened for macOS or Windows.
- The loop branch is deleted: `nativeLinuxLoopMountV1`, `withOwnedLinuxChildrenV1`, `readFixedKernelText`, the `fallocate/mkfs/mount/losetup/umount` command table, the `CapEff` check, the loop mapping schema. No compatibility alias. Grep of `nativeLinuxLoopMountV1|readFixedKernelText` over the tracked tree is empty.

## Changes

New module `NB/🐧️directory/`: `🟦️.ts` (core), `📜️script.ts` (child CLI), `🧬️schema/🔣️.json` (protocol: names, exchange symbols per OS/arch, refusals, anchor, admission), `🧪️tests/🟦️.ts` + `🧪️tests/🧫️fixtures/🔣️.json`.

Linux branch and ripple (all `NB/`):
- `🧱️fixed/🟦️.ts`: new `linux` backend (`members`, `directory` flags on `NativeFixedBackendV1`), `fixedCommandSpec` linux branch, protected parent/maintenance rosters (`listNativeDirectoryChildrenV1`), disposable/funded retirement (removes `v`), failed-birth cleanup, receiver facts and physical rechecks (`nativeImageAttachedV1`, `nativeImageFilesystemBytesV1`), port `measure`. Windows branch only got `members`/`directory` fields; Windows expressions kept their original shape.
- `🌐️platform/📋️contract/🟦️.ts`: shared helpers `nativeImageBackingFilenameV1`, `nativeImageAttachedV1`, `nativeImageFilesystemBytesV1`, `nativeImageUsedBytesV1`, `nativeImageSpaceV1` (non-Linux behaviour identical to the previous inline code).
- `🟦️.ts` (storage index: mounted-child check, rebudget usage, commit measurement), `🔌️port/🟦️.ts` (optional `measure`), `📤️publication/🧱️fixed/🟦️.ts` + its schema (`DirectoryMapping` replaces `LinuxMapping`; `kernel()` reader removed), `🧹️cleanup/♻️retirement/📋️contract/🟦️.ts` (Linux census uses `lsof +D`), literal `fixed.img` -> `fixed.dir` in publication `🌐️platform` and `🎛️owner`, fixed `🧬️schema` (`MaterializedPaths`).
- `LIB/⚡️caching/💾️storage/🔒️enforcement/`: `🔌️nx/📦️graph/🟦️.ts`, `📦️budget/🕸️graph/📏️headroom/🟦️.ts`, `📦️budget/📥️opening/📐️layout/🟦️.ts`, `📦️budget/📥️opening/📤️publication/🟦️.ts` (statfs -> helpers, filename).
- `LIB/🧼️workspace-cleanup/🛡️protection/🟦️.ts`: backing name set.

Preflight (coordinator request): `BLD/🩺️preflight/🟦️.ts`, `🔣️policy.json` (per platform: architectures, 8 GiB, tools, exchange probe), `🧬️schema`, fixture, test. `bun repo preflight` takes no scope, runs on linux/darwin/win32, requires no `CAP_SYS_ADMIN`, loop device, distribution or Docker. On Linux it probes atomic exchange and case sensitivity beneath the nearest existing ancestor of `.cache` and prints the probe error. Removed the permanent `[DEBUG]` (G17; the file was rewritten, grep for `DEBUG` is empty). Related one-line edits: `BLD/📜️script.ts` help text (`preflight`), `BLD/📥️caller/📤️child/🟦️.ts` (uses `readBuildPreflightArgumentsV1`), course `📦️packages/🟦️typescript/📜️script.ts` (calls `["preflight"]`).

Bazel readiness (`BST/🛠️tools/🏗️bazel/🩺️readiness/`, schema, fixtures, test): Linux facts are now `exchange` (probed) instead of capability/loop/mount-tool facts; macOS checks `/usr/bin/hdiutil` (G18); the Windows branch only lost the dead `mountedPartitions` condition (`!elevated` refuses as before).

Docs: `🧰️framework/🛍️products/🦑️repo/README.md` (Linux row in both languages, partition paragraph).

Existing tests re-pointed (loop parser cases removed, directory cases added): `🧱️fixed/🧪️tests/🟦️.ts` + fixture, `🧱️fixed/🎛️command/🧪️tests/🟦️.ts`, `🧱️fixed/🧪️tests/🌱️birth/🟦️.ts` (platform mock gets `members`), `📤️publication/🧱️fixed/🧪️tests/🟦️.ts` + fixture (mapping, observation and command-plan cases for the directory backend; the kernel-reader test is gone with `kernel()`), `📦️budget/🧪️tests/📥️parent/🟦️.ts` (Linux provider bindings), `🔌️nx/📦️graph/🧪️tests/🟦️.ts` (loop-mount test removed; this was the import break reported by store-out).

## Verification

macOS arm64 (Bun 1.3.14):
- `SEMIO_TEST_ARTIFACT_DIR=C/🗑️generated/xplat-linux-tests bun test` of the directory, preflight, readiness and fixed-command suites: 26 pass, 0 fail (directory suite 12 tests: capacity admission vs SQLite oracle and Ajv, anchor canonical form, exchange refusal table with fake ports, real kernel exchange vs `stat` and an independent python/ctypes exchange, concurrent readers never see a missing or duplicated name while a child exchanges 400 times, lifecycle, allocation refusals, case probe vs `sh`, usage vs `du -sk`, nested roster, child-process actions, probe cleanliness).
- `bun repo preflight` (real CLI, no scope): complete.
- Through a temporary invocation harness (births the Script envelope, runs `bun test`): `🧱️fixed/🧪️tests/🟦️.ts` 26 pass, `📤️publication/🧱️fixed/🧪️tests/🟦️.ts` 3 pass.
- `bun build --target=bun 📜️script.ts` (root): bundles.

Linux arm64 (`oven/bun:1.4.2-debian`, uid 1000, no capabilities, repo on an ext4 Docker volume, tmpfs work dir):
- Directory suite 12/12 on tmpfs and on ext4; preflight 3/3; readiness 7/7; publication-fixed 3/3 (harness).
- Real CLI, unprivileged: `bun repo preflight` passes (with stub `cc`/`pkg-config`, the image has no compiler). `bun repo cache status` on an empty tree births `l` (64 MiB), `o` (2 GiB) and `d` (8 GiB) partitions (anchor, receipt, journal, `diagnostics.json`), reopens them on later runs, survives 4 concurrent invocations on a populated tree and 4 concurrent cold births, and recovers after `kill -9` during birth. `bun repo tools setup nx` birthed the `t` partition and then stopped at an unrelated recipe error (below).
- A bind mount from the macOS host into the container (virtiofs, case-insensitive) is refused by preflight and allocation, which is intended and is what WP-B must avoid for `.cache/b`.

## Not run / open items

1. Publication of images (`publishRepositoryCacheImageV1`, tooling and dependency generations), funded Nx namespaces and the disposable/funded child retirement on Linux are adapted and unit-covered through the extracted-function tests, but never executed end to end on Linux: `tools setup` fails earlier for an unrelated reason. `bun repo tools setup nx` fails with "Closed pinned dependency recipe required" because `BST/🛠️tools/package.json` became a workspace package (`exports`, no `packageManager`/`semio.toolPatches`/`dependencies`) while `captureNxToolingInputsV1` still reads it as the install recipe. This breaks macOS too (owner: deps/lw10a work). First thing to run once fixed: `bun repo tools setup all` and `dependencies sync --all` in the Linux container.
2. Existing tests that need a real `repo test` birth fail when run through my harness for reasons independent of this change: `📦️budget/🧪️tests/📥️parent/🟦️.ts` (path typos in the test itself: `../../../../🧱️fixed/…` and the `🍎️image` join, present in HEAD) and `🔌️nx/📦️graph/🧪️tests/🟦️.ts` ("Exact carried graph purpose receipt required"). The Linux provider test I re-pointed (`protectedProviderSourceV1`) was only exercised individually, not green as a whole. Python-dependent Windows receipt tests fail on Linux containers without python3.
3. `lsof +D` census (cleanup contract) is the only guard against open descriptors when detaching a child: an exchange succeeds with open files, unlike `umount`. It requires `lsof` (already required before).
4. Commit-time measurement walks the whole partition for allocations of 16 MiB or more; fine for the current stores, revisit if one commit pattern becomes frequent on very large trees (`measureCommitBytes` in the directory schema).
5. Not touched, for the owners: `.devcontainer/**` still has `privileged: true` and loop-device remedies (WP-B), `.devcontainer/README.md` and course README still describe loops (WP-B/F), CI `architecture-course.yml` sudo wrappers (WP-G2), Windows VHD refusal (WP-C). Receipt field names (`mountDirectory`) keep their image-era names; renaming them is a schema-wide change.
6. Docker Desktop could not bind-mount the repo (`~/Documents` not shared), so Linux runs used a rsync copy of the tracked framework tree into a throw-away volume; the volume, copies and `oven/bun:1.4.2-debian` pull (the one allowed image) were the only Docker effects. Scratch copies and the volume were removed.
