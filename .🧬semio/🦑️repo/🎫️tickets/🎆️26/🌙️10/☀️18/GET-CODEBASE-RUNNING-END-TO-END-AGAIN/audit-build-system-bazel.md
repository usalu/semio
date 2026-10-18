# 🧭️ Build System And Bazel Audit (Read-Only)

Scope: neutral build interface, Bazel adapter, cache image storage, and the `.publication` state. Nothing was built, started, installed, deleted or modified. Source paths are relative to `/Users/ueli/Documents/semio`. The cache tree was changing during the audit (see section 4).

## 1. Architecture Of The Neutral Interface

- Root `package.json:134`: `repo` runs `bun …/🏗️build/📜️script.ts repo`.
- `🏗️build/📜️script.ts`:
  - `RepoScript.run` at l.80-82 calls `runOwnedBuildInvocationV1` (l.85-88), which opens the `leases` partition and the `diagnostics` and `dependencies` stores (l.87), then calls `runOwnedBuildInLeasesV1` (l.90-197).
  - Tool selection is read from `🔣️policy.json` (`"defaultTool": "nx"`) through the invocation reader `🧭️invocation/🟦️.ts` (per the ticket audit, l.122). `SEMIO_BUILD_TOOL` is exported at l.94-96.
  - Early subcommands: `tools setup` (l.127-134), `dependencies` (l.119-122), `cargo` (l.123-126), `reset` (l.135-140; bazel calls `resetBazelBuildStateV1`, nx calls `invalidateBuildGraphV1` plus `nx reset`).
  - Common path: `ensureBuildEnvironmentV1` (l.141-142), `graph`/`show` (l.143-159), `prepareBuildToolStorageV1` (l.160), graph load (l.161-163).
  - Nx branch: `runOwnedNxInvocationV1(root, nxBuildArgumentsV1(args))` (l.166-172).
  - Bazel branch: `selectBuildTasksV1` (l.174). `--dry-run`/`--graph` prints `planBazelTasksV1` output (l.176-184). Watch uses `watchBuildInvocationV1` (l.185-192). Execution is `adapter.runBazelTasksV1` (l.193). The adapter is `🔧️tools/🏗️bazel/📜️script.ts`.
- Bazel rules and BUILD files:
  - Authored BUILD or BUILD.bazel files in the source tree: **0** (`find` with `.cache`, `.🧬semio`, `node_modules`, `bazel-*` and `.git` pruned).
  - All BUILD files are generated at run time. `🔧️tools/🏗️bazel/📜️script.ts:236` writes `BUILD.bazel` from `declarations` into a planner workspace under the tools store. Line 225 writes `MODULE.bazel` and `tasks.bzl` (from constants `BAZEL_MODULE` and `STARLARK_RULES`). Line 46 writes a setup `BUILD.bazel` into the fixed embedded module catalog. Line 342 runs each task as `bazel run //:<label>`.
  - Under `.🧬semio/🦑️repo/⚡️cache/` there are 7,060 `BUILD`/`BUILD.bazel` files. These are Bazel output-base and external repository files, not authored code.
- Pinned toolchain: `⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🧬️schema/🔣️.json` pins version `8.4.2`, release `https://releases.bazel.build/8.4.2/release`, and per-platform sha256 values. Root `.bazelversion` is `8.4.2` (matches). `.bazelrc` sets `enable_bzlmod`, `enable_workspace=false`, `color=no`, `curses=no`, `symlink_prefix=/`. `MODULE.bazel` is only `module(name="semio", version="0.0.0")`. The adapter passes `--bazelrc`, `--lockfile_mode=error`, `--disk_cache` and `--output_user_root` (adapter l.331-332).

## 2. Bazel And Nx Installation

- `command -v bazel bazelisk nx` finds nothing on PATH.
- Repo-managed Bazel is present: `.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel` (58,602,460 bytes). Its sha256 is `45e9388abf21d1107e146ea366ad080eb93cb6a5f3a4a3b048f78de0bc3faffa`, which equals the pinned `darwin/arm64` checksum exactly. The install base `.cache/bazel/install/8.4.2/darwin-arm64/build-label.txt` reads `8.4.2`.
- Nx is reachable through `node_modules` (symlink to `.cache/b/d/m/j/e53350b0bed685d50b82/node_modules`). `node_modules/.bin/nx` and `node_modules/nx/dist/bin/nx.js` exist. Both depend on the dependencies image being mounted.

## 3. What The Ticket Lists As Remaining

Sources: `final-report.md` (l.107, l.49, l.59), `2026-10-18-progress-feedback.md`, `2026-10-17-coordination.md` (tail), `2026-10-18-neutral-task-interface-coverage.md`, `2026-10-18-current-public-build-interface-audit.md`.

1. **Dependencies integration**: run 46185 refused a 257-unit absolute path against the unchanged 256-unit limit. Dependency ownership, provider, v2 and Store-consumer integration remain. Retained recovery of `j/0b06bf268e0b61667f62` has no disposition decision yet.
2. **Windows qualification**: constructor, journal, terminal and PUB integration in progress. Full shared-traversal gate and any physical Windows acceptance are not claimed.
3. **Linux runtime**: Docker Desktop needs administrator setup, so no Linux conditional-file runtime run has happened.
4. **Public Nx and Bazel matrix**: not passed. Remaining: nine Bazel phases plus installed-Nx parity, authored ALL, WATCH and RESET, Cargo/stdin, renderer/runtime prototype and release behavior, workspace/graph/toolkit/sdk/partition checks.
5. **Authenticated SDK**: active SDK metadata diagnostic 72013 was refused by pending publication before metadata. No SDK authentication has been accepted.
6. **Execution coverage**: 3,515 authored target entries (3,372 `repo:script`, one `repo:noop`, 142 with omitted executors that have explicit `options.command`) were audited for metadata and plan shape only. Full inferred ALL graph and task execution, and every installed Nx and Bazel runtime phase, are unexecuted. Selector last-wins handling for the 142 omitted-executor targets is being addressed separately by the Interfaces agent.
7. **Pending publications and fuses**: Tools 72495 hit the 262,144-entry fuse. Tools 13831, Dependencies 20897 and Products 863 were terminated by SIGKILL with unknown cause (Products 12101 later passed canonical recovery). Selected Tools retention, the canonical declared-asset and nested-owner lifecycle, the parent/nested maintenance bridge, and the nested inactive Leases image are open.
8. **Dashboard**: live daemon, Cargo and public task acceptance pending.
9. **Closure capacity**: reader 176, runtime 256 and compiler 138 closures are at or near the limits.
10. **Cosmetic**: `🛠️cmake/📜️script.ts:11` prints "[cpp] Nx completed" on the Bazel route.
11. **Canonical cleanup and ticket closure** remain.

## 4. Native Image Storage And `.publication`

- **Layout**: `📦️budget/🔣️policy.json` (l.3-41) sets `maximumBytes` 64 GiB, control bytes 1 MiB, and partitions:
  - `rust` (`.🧬semio/🦑️repo/⚡️cache/rust/fixed`, 16 GiB)
  - `tools` (`.cache/b/t`, 8 GiB)
  - `graph` (`.cache/b/g`, 4 GiB)
  - `dependencies` (`.cache/b/d`, 8 GiB)
  - `products` (`.cache/b/p`, 24 GiB)
  - `diagnostics` (`.cache/b/o`, 2 GiB)
  - `leases` (`.cache/b/l`, 64 MiB)
- **What it is**: each partition is a fixed-capacity sparse disk image on macOS. The backing file is `fixed.dmg` (`fixed.vhd` on Windows, `fixed.img` on Linux, chosen at `🔌️nx/📦️graph/🟦️.ts:53`), with `image.json` recording image and mount-directory identity. The mount is `<partition>/m`. The Bazel binary lives in `t/m`, and `node_modules` lives in `d/m`.
- **Why the CLI needs it**: `runOwnedBuildInLeasesV1` opens the leases, diagnostics and dependencies stores before any task (l.87). Tool storage is prepared at l.160, graph storage at l.161-163. This is my reading of the code: the image caps cache size per partition and gives a single attach-and-verify point for each store. Without the mounts, the Nx and Bazel binaries and `node_modules` are unreachable.
- **`.publication`**: `native-image-current-publication-design.md` (l.46-48) and `pending-publication-owner-readiness.md` (l.15-17) define one canonical `.publication` namespace per partition. It holds exactly `intent.json` and `receipt.json` (each at most 4,096 bytes, directory mode 0700). It is created before the first detach or receipt rename. While it exists, ordinary admission refuses (`…/storage/🌐️platform/📋️contract/🟦️.ts:63`; `…/storage/🍎️image/🟦️.ts:35–41`).
- **Documented recovery**: there is **no one-line recovery command** in the ticket. The docs say:
  - "no public clear-pending or old-token-resume bypass" (`final-report.md` l.35)
  - "No manual pending/image deletion" (coordination tail)
  - Removing pending state requires private provider-owned proof of current receipt durability, exact reattachment, and matching payload and journal witnesses (`native-image-current-publication-design.md` l.48)
  - A fresh genuine publisher "may discard and rebuild only that schema-owned metadata roster," and "unknown entries or unsafe identities refuse" (l.46)
  - The sequence is `asset-census/📜️script.ts --run`, then `asset-publication/📜️script.ts --plan <fresh-basename>`, then the canonical retire or publish commands (`pending-publication-owner-readiness.md` l.25-28). Only actual completed settlement plus a current ordinary reopen unblocks SDK qualification.
  - `reset` is not a documented recovery path. A grep of `reset/🟦️.ts` found no `.publication` reference.
- **Observed state**: at about 00:39 (`.cache/b/d`) and 00:58 (`.cache/b/t`) `.publication/intent.json` (341 bytes, mode 0600) existed. By 01:14 both directories were gone, and `fixed.dmg` and `image.json` had been rewritten at 01:13-01:14. Another agent was publishing or settling during the audit. I did not read the intent contents. Current state needs to be re-checked before any decision.

## 5. Assessment

**Nx default (`bun repo run <project>:<target>`)**
1. Re-check `.cache/b/{t,d,g,p,o,l}/.publication` read-only. If it exists, the only sanctioned route is the canonical publisher sequence in section 4. Manual deletion is prohibited by the ticket.
2. With no pending state, the mounts must be attached. `bun repo show projects` exercises graph admission (l.143-159) and is the cheapest check.
3. `bun repo tools setup nx` (l.132) validates the Nx binary. Then `bun repo run <project>:<target>`.
4. Blocker only if `node_modules` must be reinstalled: the 257-unit path refusal in Dependencies run 46185.

**Bazel (`--tool=bazel`) for the three targets**
- Mapping from the project files: the mit-bestand demonstrator is `@semio-tech/mit-bestand-demonstrator` (`♻️mit-bestand/🧺️demonstrator/📋️project.json`). The semio-tech play is `@semio-tech/semio-tech-play` (`🏢️semio-tech/🎡️play/📋️project.json`). The teaching course is most likely `@teaching/architecture-quiz` (`🎓️teaching/🏛️architecture/❓️quiz/…/📋️project.json`). The owner should confirm this, since "course" is not in any project name.
1. `bun repo --tool=bazel tools setup bazel`. The pinned binary already exists with a matching checksum, so this should only verify. `prepareBazelModulesV1` (adapter l.43-47) then builds the module catalog in the tools partition, which must not be pending.
2. For each target, `bun repo --tool=bazel run <project>:<target> --dry-run` (l.176-184). This prints the plan JSON without executing, and validates selection and BUILD generation.
3. Execute the target with `bun repo --tool=bazel run <project>:<target>` (adapter l.323-346).
4. Blockers are the same tools-partition pending check, plus the uncompleted Bazel phase matrix. Nothing was executed, so runtime success is unverified.

All statements above come from reading source and ticket documents. No build, install, server or Bazel runtime command was run.
