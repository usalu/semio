# 🧭️ Bazel State Audit, 2026-10-19 (read-only, aud-bzl)

Scope: how `bun repo --tool=bazel run X:Y` becomes Bazel, what is native, what is cached, provisioning, runtime status, gap list.
Nothing was built, installed, run or edited. Only this file was written.

Abbreviations (relative to `/Users/ueli/Documents/semio`):
- `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`
- `S` = `B/🔧️tools/🏗️bazel/📜️script.ts` (Bazel adapter, 396 lines)
- `BS` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/📜️script.ts` (binary provisioning)

## 0. Verdict

- Bazel is **not** a native Bazel build. It is a planner that emits `repo_task` / `repo_launch` Starlark rules, and each rule action re-enters the repo's `bun` runner. Actions are declared `local`, `no-sandbox`, `no-remote`, and `no-cache` unless the target declares `cache: true`. Bazel's action cache therefore only helps the few `cache: true` targets, and the real cache key is a hand-computed fingerprint file.
- No `--tool=bazel` run has reached the planner or execution on this Mac. `.cache/b/p/m/bazel` (workspaces, outputs, actions) and `.cache/b/d/m/bazel` (module catalog) do not exist.
- Requested target `@teaching/architecture-quiz` no longer exists in HEAD. It was renamed to `@teaching/architecture-course` by commit `bd9e4e02bd0` (2026-10-19 12:21:52).

## 1. How `bun repo --tool=bazel run X:Y` becomes Bazel

| Step | Location | Notes |
|---|---|---|
| Entry | `package.json:134` | `repo` runs `B/📜️script.ts repo` |
| Tool choice | `B/🧭️invocation/🟦️.ts:127`, `:126-136` | `SEMIO_BUILD_TOOL ?? policy.defaultTool`; `--tool` parsed; default `nx` (`B/🔣️policy.json:3`) |
| Env inheritance | `B/📜️script.ts:98`, `:100` | `SEMIO_BUILD_TOOL` exported to children (nested tasks inherit) |
| Dispatch | `B/📜️script.ts:168-172` (nx), `:174-195` (bazel) | bazel: `selectBuildTasksV1` (176); `--dry-run` prints `planBazelTasksV1` (181); watch (191); run `runBazelTasksV1` (195) |
| Meta cmds outside Bazel | `B/📜️script.ts:119-126` (`cargo` in-process), `:127-136` (`tools setup`), `:141` (`reset`) | `cargo`, `dependencies`, `tools`, `reset`, `graph`, `show` never go through Bazel |
| Planner | `S:63-90` `planBazelTasksV1` | Recursive closure over `dependsOn` from requested targets. Lazy at graph level: only the requested closure is emitted. Labels `t_<hash16>` |
| Run | `S:343-386` `runBazelTasksInStorageV1` | prepares binary, module catalog, workspace, then `bazel build` selected, restore snapshots, `bazel run` continuous |

Generated Bazel files (none authored in the source tree; `find` finds only `.bazelrc`, `.bazelversion`, `MODULE.bazel` at root):

| File | Written by | Location |
|---|---|---|
| `MODULE.bazel` (`module(name="semio_tasks")`, no `bazel_dep`) | `S:40`, `S:242` | `.cache/b/p/m/bazel/workspaces/<ns16>/` |
| `tasks.bzl` (`repo_task`, `repo_launch` rules) | `S:213-233`, `S:242-243` | same |
| `BUILD.bazel` (one rule per task) | `S:240-256` (planner child) | same |
| `<label>.json` (manifest: `workspace`, `runtime`, `task`, `implementation`) | `S:240-256` | same |
| `<label>.input` (fingerprint file) | `S:182-192`, `S:240-256` | same |
| `implementation.json` + loader `📜️script.ts` (captured closure) | `S:202-212` | same |
| `planner-input.json`, `planner-output.json`, `runtime.json` | `S:235-239` | same |

The planner runs as a captured `bun` child (`S:235-239`), so BUILD generation itself is inside the repo's own runner.

## 2. Native Bazel or wrapper? (quoted generator)

`S:213-233` `STARLARK_RULES` (abridged):

```
def _task_impl(ctx):
    ...
    requirements = {"local": "1", "no-sandbox": "1", "no-remote": "1"}
    if not ctx.attr.cache:
        requirements["no-cache"] = "1"
    ctx.actions.run(executable = ctx.attr.bun, arguments = args, inputs = depset([manifest, fingerprint, runner, source] + dependencies), outputs = [output], mnemonic = "RepoTask", ...)
...
def _launch_impl(ctx):  # continuous tasks
    ... execution_requirements = {"local": "1", "no-sandbox": "1", "no-remote": "1"}
```

Consequences:

| Property | Status | Evidence |
|---|---|---|
| Real actions with declared inputs | Partial | Inputs are manifest, fingerprint file, runner, source closure, dependency snapshots. Project source files are **not** declared; they are only hashed into the `.input` file |
| Action cache | Only `cache: true` targets | `S:217-219` `no-cache` unless `cache` attr; `S:248` and `S:251` set `cache=True` only if `task.definition.cache && !skipCache` |
| Sandboxing | None | `no-sandbox` on every action and launcher (`S:217`, `S:227`) |
| Remote cache / exec | Disabled | `no-remote` on every action (`S:217`, `S:227`); `skipRemoteCache` declared at `S:32` but never read |
| Minimal rebuild of consumers | Coarse | Dependency outputs enter as whole snapshot directories (`S:255`, codex `bazel-2026-10-19.md` line 52) |
| Outputs | Copied back into the source tree | `restoreBazelSnapshotV1` `S:277-294`, called for all non-continuous tasks at `S:366` |
| Continuous tasks | Never cached | `bazel run //:label` per task with its own `--output_base` (`S:361`) |
| Cache key includes namespace | Yes, a defect | Manifest embeds `runtime = <workspace>/runtime.json` (`S:240-256`), and workspace is `hash(selected+args+configuration)`. The same task under another selection gets a different manifest, so it misses the action cache |
| Own-source hashing | Bypassed for object inputs | `taskInputs` `S:154-166` defaults to `{projectRoot}/**/*`, but object inputs (`{dependentTasksOutputFiles,...}`) are not strings, so `taskFingerprint` `S:182-186` drops them (`patterns = inputs.filter(string)`) |
| Bazel-run every task | Only at outer level | `proctor:build` imports cargo in-process (`🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts:29`); nested cargo runs inside one action, outside Bazel's graph |

Verdict on Q2: it is a wrapper around `bun` with a Bazel action graph and disk cache. It is not the sandboxed, declared-input model the goal requires.

## 3. Caches configured

| Cache | Setting | Location | Source |
|---|---|---|---|
| Binary | pinned, sha256-verified | `.cache/b/t/m/bazel/bin/<sha16>/bazel` | `BS` (prepareBazel), schema `⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🧬️schema/🔣️.json` |
| Install base | `--install_base` | `.cache/b/t/m/bazel/install/8.4.2/<platform-arch>` | `S:348` |
| Output user root | `--output_user_root` | `.cache/b/p/m/bazel/outputs/<hash16>` | `S:348` |
| Action (disk) cache | `--disk_cache` | `.cache/b/p/m/bazel/actions` | `S:348`, `S:352` (stripped for `info`) |
| Repository cache | `--repository_cache`, `--repository_disable_download` | `.cache/b/d/m/bazel/modules/<id16>/repositories` | `S:44-50`, `S:349` |
| Remote cache | none | none | `S:32` (dead field) |
| Per-task run base | `--output_base` | `.cache/b/p/m/bazel/outputs/launch/<ns>/<label>` | `S:361` |
| Layout | `bazelCacheLayoutV1` | `B/🔧️tools/🏗️bazel/💾️storage/🟦️.ts:14` | |

- Disk cache is unbounded inside a fixed 24 GiB products image (`.cache/b/p`, `fixed.dmg` 25,768,755,200 bytes). No `--experimental_disk_cache_gc_*` flag is set. Verify flag names against 8.4.2 before use.
- Bazel `--keep_going` is set unless `bail` (`S:350`).
- Jobs default `min(8, cores)` (`S:348` area).
- Current disk state (12:47): `.cache/b/p/m` mounted; `.cache/b/p/m/bazel` absent; `.cache/b/d/m/bazel` absent; `.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel` present (58,602,460 bytes, 2026-10-18 11:35). No `.publication` pending in `.cache/b/{p,t,d}`.
- Cache storage requires fixed native images to mount first (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:167-171`: Linux `mkfs.ext4` and `mount -t ext4 -o loop`; Windows `Get-DiskImage`/`Mount-DiskImage`, observed to need admin, HRESULT 0x80070522, per codex report).

## 4. Provisioning (zero-touch?)

| Item | Status | Evidence |
|---|---|---|
| `bun repo tools setup bazel` | Fetches and verifies the binary only | `B/📜️script.ts:135-136` calls `adapter.prepareBazel` only. Module catalog is built lazily at run time (`S:44-50`, from `runBazelTasksInStorageV1`) |
| Version | Bazel 8.4.2, in `.bazelversion` and the BS schema | `.bazelversion`; BS schema `version: "8.4.2"` (schema lines 67-68) |
| Platforms pinned | darwin arm64/x64, linux arm64/x64, win32 arm64/x64 with sha256 and bytes | BS schema lines 71-109 |
| Local status | darwin/arm64 binary present and checksum-matched per ticket audit | `audit-build-system-bazel.md` section 2 |
| Module catalog | Not built on this Mac (`.cache/b/d/m/bazel` absent) | |
| Windows | Bazel native launcher failed `ClassNotFound` in ticket-ancestry (Unicode) path; fixed VHD mount needs admin | codex `…/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/bazel-2026-10-19.md`, section "Isolated Pinned Bazel Attempt" |
| Linux / devcontainer | Not run. `mount -o loop` needs privileges | `…/🧱️fixed/🟦️.ts:171`; ticket C `audit-build-system-bazel.md` section 3.3 |
| `.bazelrc` | `enable_bzlmod`, `enable_workspace=false`, `color=no`, `curses=no`, `symlink_prefix=/` | `/Users/ueli/Documents/semio/.bazelrc` |
| Stray output | `./bazel-out/` at repo root (3 files, dated 2026-10-15), not tracked | `git ls-files -- bazel-out` empty |

## 5. Runtime status and fleet activity (as of 2026-10-19 12:47)

- Last known Bazel runtime evidence:
  - Mac, 2026-10-18: `tools setup bazel` OK (binary only). `result-bzl.md` runtime phase blocked on `node_modules` and the dependency install. No dry-run reached the planner.
  - Windows (codex, `bazel-2026-10-19.md`): `tools setup bazel` failed with ENOENT on `.cache/b/l/image.json` before Bazel started. Cold/warm and cancellation not reached. The isolated pinned Bazel 8.4.2 smoke failed on `bazel version` with `ClassNotFound` (Unicode ancestry).
- No successful Bazel execution, action-cache hit, or disk-cache restore is recorded anywhere.
- Processes at 12:47: no `bazel` or `bazelisk` processes. A `bun repo dependencies lock --all` (pid 32781, started about 12:44) is running from the ticket C chain (`chain-tools/chain.sh`).
- Chain log `🗑️generated/chain/chain.log` (12:38): `dependencies lock --all` failed with "Queued cache maintenance must begin outside its reader scope" (`⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts:124`, called from `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:72`).
- `node_modules` now resolves (`node_modules/nx/package.json` present at 12:49, symlink to `.cache/b/d/m/j/4151d2262cb4a4e7a25a/node_modules`). `HANDOVER.md` section 2.10 and `result-bzl.md` ("still dangling") are stale for this point. Re-verify.
- Codex fleet: Bazel reports and `bazel-smoke/` are stamped 2026-10-19 12:29 (batch mtime). No codex Bazel process is visible. Treat the fleet as idle on Bazel, not active.

## 6. Requested products: current Bazel path

| Requested | Resolves in HEAD? | Project file (line) | cache | continuous | outputs | Bazel path gap |
|---|---|---|---|---|---|---|
| `@teaching/architecture-quiz:test` | **No.** Renamed to `@teaching/architecture-course` (`bd9e4e02bd0`) | `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json` (name line 2) | `test` true (line 209) | - | `[]` | Name. Once renamed: cacheable, but outputs `[]` and no-sandbox |
| `@teaching/architecture-quiz:publish` (`…course:publish`) | same | same, lines 186-201 | false (197) | - | `{projectRoot}/dist/release/pages/course` | Never cached by design, which is correct. Snapshot copied to source tree every run |
| `@teaching/architecture-quiz:dev` (`…course:dev`) | same | same, lines 83-135 | false (132-133) | true | - | `bazel run` path, uncached by design |
| `@teaching/proctor:build` | Yes | `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json` lines 21-28 | false (28) | - | none | Always re-executes. Cargo runs in-process inside the action, outside Bazel's graph. No declared Rust inputs or outputs |
| `@semio-tech/semio-tech-play:activate-prototype` | Yes | `🏢️semio-tech/🎡️play/📋️project.json` lines 67-~126 | true (69) | - | `{projectRoot}/dist/♻️activation/prototype` | `inputs` only `dependentTasksOutputFiles` (object, dropped by fingerprint). Own sources not hashed. Validate against `activate` source. Depends on inferred `prepare-prototype` |
| `@semio-tech/semio-tech-play:serve` | Yes | same, lines 158-181 | false (175) | true (176) | - | Readiness via `metadata.semio.dashboard.ready` port 6033 (lines ~160-168); static path only, not run |
| `@semio-tech/mit-bestand-demonstrator:activate-prototype` | Yes | `♻️mit-bestand/🧺️demonstrator/📋️project.json` lines 142-~167 | true (144) | - | `{projectRoot}/dist/♻️activation/prototype` | Same object-only inputs (lines ~150-156). Depends on inferred `activate-lanes-prototype` |
| `@semio-tech/mit-bestand-demonstrator:serve` | Yes | same, lines 193-217 | false (210) | true (211) | - | `bazel run` path |
| `@semio-tech/mit-bestand-demonstrator:test-e2e` | Yes | same, lines 250-262 | false (252) | - | none | Uncached by design (Playwright). Sandboxing and network need review |

Also: `test` in the demonstrator (lines 133-141) has no `cache` key, so schema default `false` applies. 35 `prepare-<variant>` and 35 `activate-<variant>` framework-os-dev targets are graph-inferred, not in the tracked manifest (`audit-bazel-targets.md`).

## 7. Prioritized gap list

### P0: blocks any run

| # | Gap | Evidence | Fix |
|---|---|---|---|
| G1 | `@teaching/architecture-quiz` renamed to `@teaching/architecture-course` in HEAD; `result-bzl.md` "all eight names exist" likely relied on the stale graph `🎓️teaching/.1b8e820` (2026-10-18 17:22) | `git cat-file` on quiz project.json at HEAD: absent. Commit `bd9e4e02bd0` renames `❓️quiz/*` to `📚️course/*` (`R0xx` entries) | Decide with user: use `@teaching/architecture-course:{test,publish,dev}`. Then update `result-bzl.md` and the ticket target list |
| G2 | Graph load and dependency install not green. Chain step 1 failing on queued cache maintenance (`…/📥️opening/🏪️store/🟦️.ts:124`) | chain.log 12:38; pid 32781 running now | Deps owner: freeze manifests, get `dependencies lock --all` and `sync --all` green. Then `bun repo show projects` must load. Do not run concurrent `bun repo` commands |
| G3 | Bazel closure cap: 527 files vs 512 (raised to 2048 in ticket C). Not re-measured after install | `result-bzl.md` runtime section | Re-measure with `🗑️generated/bzl-closure/measure.ts`, then keep the cap |
| G4 | Native images (`tools`, `dependencies`, `products`) must mount. Linux needs root-level `mount -o loop`, Windows needs admin | `…/🧱️fixed/🟦️.ts:167-171` | Platform owner: provide a non-privileged path or document the prerequisite for devcontainer and Windows |

### P1: caching semantics (the core goal)

| # | Gap | Evidence | Fix |
|---|---|---|---|
| G5 | Default `no-cache` plus `local`, `no-sandbox`, `no-remote` on every action | `S:217-227` | Drop `no-remote` and `no-sandbox`. Make cache the default for non-continuous tasks. Keep `no-cache` only for `publish`, `e2e`, and other side-effecting targets, with a reason |
| G6 | Cache key is a hand-rolled fingerprint file, not declared inputs | `S:182-192` (`.input`), `S:213-233` | Declare the closure of source files as action inputs (`sourceFiles` `S:139`), so Bazel's own action key does the work. Keep fingerprint only as a derived declared input |
| G7 | Object inputs (`dependentTasksOutputFiles`) silently dropped from the fingerprint | `S:154-166`, `S:183` (`filter(string)`) | Expand object inputs in the Bazel path. Add project `{projectRoot}/**/*` to `activate-prototype` for play and demonstrator. Validate against `activate` source |
| G8 | Namespace-specific `runtime` path in manifest breaks cross-selection action reuse | `S:240-256` (`runtime=join(workspace,"runtime.json")`) | Move live runtime data out of the manifest into a non-key channel (as the codex fingerprint note already intends). Keep the manifest key content-stable |
| G9 | Outputs copied back into source tree; consumers depend on whole dependency snapshots | `S:277-294`, `S:366`; `S:255` | Declare outputs as Bazel outputs. Consumers depend on the declared subset only. Restore to the source tree only at the outer boundary |
| G10 | `proctor:build`: `cache: false`, no inputs or outputs, cargo in-process | `🎓️teaching/🛂️proctor/…/📋️project.json:21-28` | Declare Rust inputs (sources, `Cargo.lock`, `Cargo.toml`) and outputs (built binary). Set `cache: true`. Make nested cargo a Bazel-visible action, or accept the boundary and say so |
| G11 | No remote cache option. `skipRemoteCache` is dead | `S:32` | Add a `.bazelrc`-driven remote cache switch (env-gated), and remove the `no-remote` tag |
| G12 | Disk cache unbounded inside fixed 24 GiB products image | `S:348`; budget policy | Configure Bazel disk-cache GC (verify flag names for 8.4.2) and align with the products budget |

### P2: platform, zero-touch, performance

| # | Gap | Evidence | Fix |
|---|---|---|---|
| G13 | `tools setup bazel` does not build the module catalog, so first run needs network and privileged path work | `B/📜️script.ts:135-136`; `S:44-50` | Make `tools setup bazel` run `prepareBazelModulesV1` too, so zero-touch verifies the whole chain |
| G14 | Windows: Bazel 8.4.2 native launcher `ClassNotFound` under Unicode ancestry; fixed VHD needs admin | codex `bazel-2026-10-19.md` | Test under an ASCII junction or subst root; record the failure mode. Add a preflight that refuses non-ASCII paths on Windows |
| G15 | Linux and devcontainer never run (`mkfs`, `mount -o loop`, Docker Desktop admin) | ticket C audit section 3.3; `…/🧱️fixed/🟦️.ts:171` | Linux runtime owner: run in a privileged devcontainer, or add a non-mount fallback store with the same contracts |
| G16 | Each continuous task starts its own Bazel server (`--output_base` per label) | `S:361` | Share one server (`--max_idle_secs`) and one output base for launch, if correctness holds |
| G17 | Stray `./bazel-out/` at repo root | `ls` | Owner to review and remove. Do not touch from audit |

## 8. Suggested disjoint work packages (file ownership)

Run after G2 is green. No package edits a file owned by another package. Each executor uses foreground runs only, and no git modifying commands.

| WP | Owns (write) | Goal | Depends on |
|---|---|---|---|
| WP0 deps owner (not Sonnet) | `package.json`, `bun.lock`, `B/…/🚀️bootstrap/` | G2: green lock, sync, graph load | none |
| WP1 target identity | `ticket C result-bzl.md`, `launch.json` target entries for quiz to course rename | G1: confirm mapping, update requested target list | user decision |
| WP2 manifest semantics | the four project files: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`, `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`, `🏢️semio-tech/🎡️play/📋️project.json`, `♻️mit-bestand/🧺️demonstrator/📋️project.json` | G7, G10: inputs, outputs, cache flags for the 8 requested targets | WP1 mapping |
| WP3 adapter action semantics | `S` only (`B/🔧️tools/🏗️bazel/📜️script.ts`) and its `🧬️schema/🔣️.json` | G5, G6, G8, G9, G11: real inputs, outputs, drop tags, cache default, manifest key | none. Serialize with WP4 |
| WP4 cache config and GC | `B/🔧️tools/🏗️bazel/💾️storage/🟦️.ts`, `.bazelrc`, `🧬️schema/📦️budget` (budget policy only), `B/🔧️tools/🏗️bazel/🧪️tests/📦️snapshot/` | G12, G11 config: disk GC, remote switch | WP3 flags |
| WP5 zero-touch | `B/📜️script.ts` (only `tools` branch 127-136), `BS` | G13, G14 preflight | none |
| WP6 verification (no source edits) | `…/☀️18/GET-…/🗑️generated/` only | Run the codex `bazel-smoke` generator, then `bun repo --tool=bazel run <target> --dry-run` for each of the 8 targets, then cold and warm runs of `architecture-course:test`, then `activate-prototype` cold/warm/unrelated/selected/disk-deletion | WP0, WP1, WP3 |

Recommended order: WP0 then WP1 (decision), then WP2 and WP3 in parallel (disjoint files), then WP4 and WP5, then WP6.

## 9. Open items to validate (not verified here)

- Whether play and demonstrator `activate` read their own sources (decides G7 severity).
- Whether `architecture-course:publish` is required as a Bazel action or only as a documented outer command.
- Bazel 8.4.2 flag names for disk-cache GC.
- Whether `bun repo show projects` loads now that `node_modules` resolves (not run; read-only audit).
