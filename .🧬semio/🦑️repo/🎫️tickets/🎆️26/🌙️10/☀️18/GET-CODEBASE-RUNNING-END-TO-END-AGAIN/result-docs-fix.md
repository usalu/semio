# Result docs-fix: every build-doc statement against call sites (2026-10-19)

Executor `docs-fix`. Input: `audit-verify-1019e.md` section 7 (5 of 10 sampled statements not confirmed), `result-docs-build.md`, `result-docs-build2.md`, `result-fixups3.md`. No sub-agent, no modifying git command, no build, no `bun repo run`, no cargo, no dependency sync. Edits were atomic anchored replacements (assert exactly one match, then write) because `cli-fix` edited the same README at the same time; the three `cli-fix` additions (Help bullet, exec environment `SEMIO_PROJECT_*`, `--graph` row, German commands and flags) were checked, not changed.

Abbreviations: `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `BZ` = `B/🔧️tools/🏗️bazel`, `CA` = `…/📚️library/⚡️caching`, `BS` = `CA/🚀️bootstrap`, `BT` = `BS/🛠️tools`, `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`. Line numbers are those of 2026-10-19 ~18:00 to 19:00; the tree moves.

Docs in scope: repo README chapter "Build and run" (EN and DE, `🧰️framework/🛍️products/🦑️repo/README.md`), root `README.md` (Windows Setup, Build, the bootstrap-sources sentence), `.agents/skills/build/SKILL.md`, `.devcontainer/README.md`.

## Outcome

- The table "Fixed" lists 56 statements that were wrong, stale or without a call site (row 1 only confirms a fix of `cli-fix`); "Confirmed" lists the statement groups that have a call site, and "Marked" the statements that stay "not yet verified" or "recorded earlier" because they need a host that was not available.
- All five statements of the audit (3: host path guard call site, 7: Windows long-path refusal, 8: `SEMIO_TASK_RECEIPT`, 9: `workspaceCommand`, 10: Bazel 8.4.2) now have a call site: 3 = `BZ/💾️storage/🟦️.ts:22` (`bazelCacheLayoutV1` calls `requireBazelHostPathsV1`; every storage user builds the layout), 7 = `BT/🏗️bazel/🩺️readiness/🟦️.ts:32`, 8 = `B/🧾️store-receipt/🟨️.mjs:25-29` called from `BZ/📜️script.ts:474` and `BS/🔌️nx-plugin/📤️arguments/🟨️.mjs:18`, 9 = `📚️library/🏃️process/🧭️routing/🧩️contributions/🟦️.ts:20-23`, 10 = `BT/🏗️bazel/🧬️schema/🔣️.json` `Release` (8.4.2, six distributions) and `.bazelversion`.
- Docs laws: `owner-projection-policy` 35 pass 0 fail, `readme-current-source-revision` 33 pass 0 fail after the re-pin (below).

## Fixed (statement, evidence, action)

| # | Statement (before) | Evidence | Action |
|---|---|---|---|
| 1 | `exec` exports `NX_PROJECT_NAME`/`NX_PROJECT_ROOT_PATH`, "a tool-neutral pair is not defined" | `B/⚙️operations/🌱️exec-environment/🟦️.ts:2`, `BT/⚙️operations/🟦️.ts:26-34` (wrapper maps NX_* to `SEMIO_PROJECT_NAME`/`SEMIO_PROJECT_ROOT`), `BZ/📜️script.ts:325` | already corrected by `cli-fix` (EN and DE); confirmed |
| 2 | Cancellation and progress hold for every operation | only declared in `B/⚙️operations/🔣️catalog.json`; no test asserts them | rewritten as "declares", cites `runOwnedBuildInLeasesV1`, marked "not yet verified" (EN, DE) |
| 3 | `tools setup` takes `nx|bazel|all` | `B/📜️script.ts:134` (no argument = selected tool) | row extended (EN, DE) |
| 4 | Store receipt publishers: "the Vite and wasm publisher, the compiler store publisher of every direct Cargo `build`" | `CA/📦️artifacts/🟦️.ts:27`, `CA/📦️artifacts/🏗️native-build/🟦️.ts:48` (skipped for `observationOutput`), `CA/📦️artifacts/🟦️typescript/🔗️binding/🟦️.ts:29` | file references added, "every direct" removed (EN, DE) |
| 5 | `SEMIO_TASK_RECEIPT` only for the declaring task (audit 8) | `B/🧾️store-receipt/🟨️.mjs:25-29`, `BZ/📜️script.ts:474`, `BS/🔌️nx-plugin/📤️arguments/🟨️.mjs:18` | call sites added (EN, DE) |
| 6 | "Not receipted on purpose: Rust builds that aggregate per-unit `compile` tasks, playground orchestrators with receipted children" | no call site found | replaced by the rule `declaredStoreReceiptV1` (a target declares a receipt only if `outputs` lists the path) |
| 7 | prototype/release share one receipt "and so rerun once per switch" | inferred from the path (no mode in it), never observed | marked "expected, inferred" (EN, DE) |
| 8 | Nx executors "run the literal arguments through Nx's own task lifecycle" | `executors.json` descriptions (process authority and Bun shell; argv without shell interpolation) | rewritten (EN, DE) |
| 9 | Pool `n` "only receives graph snapshots and funded native namespaces" | `CA/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` (Nx graph cache store in partition `nx`) | rewritten: 1.5 GiB Nx pool holding the Nx graph cache, not task results (EN, DE) |
| 10 | Rust partition "also holds the Rust outputs Nx caches" | `.cargo/config.toml` (shared target and build dirs in that partition) | rewritten (EN, DE) |
| 11 | "unit and contract tests pass" (Nx) | ran 2026-10-19: 23 tests, 0 failures (tools operations, sdk, probe, selection, invocation) | dated, counts added (EN, DE) |
| 12 | Bazel generated workspace has "a `BUILD` file" | `BZ/📜️script.ts:420` writes `BUILD.bazel` | fixed (EN, DE; also "exact BUILD.bazel text") |
| 13 | pure ordering tasks vanish behind producers | `bazelActionsV1` (`BZ/📜️script.ts:195-212`) keeps selected noops | "unless selected themselves" (EN, DE) |
| 14 | "Every action is tagged `no-remote-exec`" | `STARLARK_RULES` (`BZ/📜️script.ts:356`): noop writes a file, no requirements | "every task and launcher action" (EN, DE) |
| 15 | `.bazelrc` "a test keeps it that way" (no sandbox flags) | test `the repository .bazelrc carries exactly …` (`BZ/💾️storage/🧪️tests/🟦️.ts:55`) pins cache lines and rejects paths, forbids no sandbox flag | rewritten honestly (EN, DE) |
| 16 | Host path guard has no call site (audit 3) | `BZ/💾️storage/🟦️.ts:22` | added (EN, DE) |
| 17 | package boundary only for ASCII directories | `bazelSourceClassesV1` (`BZ/📜️script.ts:228-243`) treats any `BUILD`/`BUILD.bazel` directory as boundary | rewritten, conservative behaviour stated (EN, DE) |
| 18 | emoji label facts and `LANG=C` identity as facts | real-binary test `BZ/🧪️tests/📥️actions/🟦️.ts:145-` (re-run 2026-10-19, passes in 28 s); `LANG=C` only in `result-bzl-label.md`, no test | attributed "observed"; `LANG=C` marked "scratch experiment, no test" (EN, DE) |
| 19 | "Verified": real-binary tests, adapter suite passes file by file | ran 2026-10-19: storage + operations suites 67 pass 4 skip (no binary); with the binary: 20 pass, 2 fail (actions file needs the issued envelope when run alone); aggregate `test bazel` (`B/📜️script.ts:58-65`): 41 pass 34 fail, first failure `Bazel component contracts have one canonical owner with unchanged normalized constraints` (two entries `unchanged: false`), then "Original process admission closed" | paragraph rewritten with the real state (EN, DE); open item 1 |
| 20 | the closure blocker "pulls the `@semio-tech/repo-lib/storage` barrel" | no production file under `B` or `BS` imports `@semio-tech/repo-lib` any more | stated as resolved (EN, DE) |
| 21 | adapter suite via `bun repo test bazel` | `bun repo` = `…/🏗️build/📜️script.ts repo`; the route is the root target `workspace:test-build-tool-bazel` (`📋️project.json` `test-build-tool-bazel`) | command fixed in the Verification table |
| 22 | `semio run --dry-run … --parameter k=v` | `📚️library/…/🎛️dashboard/⌨️usage/🦀️.rs` usage: `--param k=v` | fixed (EN, DE) |
| 23 | "the two former IDE launch entries are gone" | history, no call site; `.vscode/` does not exist (`ls`) | clause removed (EN, DE) |
| 24 | `manifests=1456 workspaceCommands=171 problems=0 at the last run` | no run reproduces it | numbers removed; rule and loader kept with call site (EN, DE) |
| 25 | registry law "passes except one fixture row" | ran 2026-10-19: `dashboard-control` 8 pass 2 fail (fixture row `@semio-tech/repo-dashboard-rs:workflow status` expects `native`, oracle `build`; one other test per run hits its own 5 s or 15 s timeout under load) | exact result stated (EN, DE, Verification table) |
| 26 | owner registry "3674 owners, no findings", "CLI route passed" | `📋️build-owners.json` has 3718 owners; `bun ./📜️script.ts verify build-owners` printed `passed` (1:42), `verify zero-touch` printed `passed` (1:08) on 2026-10-19 | numbers and dated runs (EN, DE, Verification table) |
| 27 | "A directory move or rename is not repaired automatically" | `📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts:119` reconciles after an applied move plan | rewritten (EN, DE) |
| 28 | sync drops rows of vanished modules; fails only on catalog providers and native roots | `…/🧾️registry-consistency/🟦️.ts:115` (drop only if no manifest and git tracks nothing), `🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:197-210` (also fails on tracked-but-absent paths) | rewritten (EN, DE) |
| 29 | `external-package` is "the same refusal as the immutable source capture" | no call site | removed, `missing-entry` kind added (`B/📥️source/🛗️zero-touch/🟦️.ts:23-24`) (EN, DE) |
| 30 | `workspace:setup` runs `tools setup all`, "need nothing else" | `📋️project.json` `setup.dependsOn` lists `deps-build-tools` among 13 other targets | rewritten (EN, DE) |
| 31 | Nx toolset below `.cache/b/t/m/nx/<sha256>` | `BT/📦️sdk/📊️material/🟦️.ts:41` (`basename(realpath(.nx/installation))`, a digest) | `<digest>` (EN, DE) |
| 32 | Linux: "every commit of 16 MiB or more measures the real tree" | `NB/📁️directory/🟦️.ts:90-105` and schema `meter` (every 16 MiB admitted or 2 s, on commit, before refusal) | corrected (EN, DE) |
| 33 | Linux probe "beneath the nearest ancestor of `.cache`" for both checks | `B/🩺️preflight/🟦️.ts` (ancestor of `.cache`), readiness `collectBazelPlatformFactsV1` (ancestor of the tools partition) | split (EN, DE) |
| 34 | Linux "ext4 on a Docker volume and tmpfs were exercised" | recorded in `result-xplat-linux.md` only | marked "when the backend landed, not re-run" (EN, DE) |
| 35 | Devcontainer "unprivileged" | compose file has no `privileged`; `devcontainer.json` adds the `docker-in-docker` feature, whose metadata the CLI applies and which was not examined | reworded in repo README (EN, DE) and `.devcontainer/README.md` |
| 36 | Windows: partitions are `fixed.vhd` via `Mount-DiskImage` | `NB/🌐️platform/🟦️.ts:21-24` (non-macOS = directory image), `NB/📁️image/🟦️.ts:4` (linux and win32), `NB/📁️directory/🟦️.ts:22` (journaled port on win32), schema refusals (build 17107, `Microsoft-Windows-Subsystem-Linux`) | Windows row rewritten (EN, DE); marked provisional, open item 2 |
| 37 | Windows: "nothing user-global" | `…/🔩️native/🥾️bootstrap/🔵️.ps1:98-113` (user `PATH`), `:150-157` and `:575` (user `PLAYWRIGHT_BROWSERS_PATH`) | corrected (EN, DE, root README) |
| 38 | Windows: "several maintenance operations are explicit 'not qualified' refusals" | no such string left in source | removed |
| 39 | pin manifest "25 rows for darwin, linux and win32 on x64 and arm64" | `…/🥾️bootstrap/🧬️schema/🔣️.json`: 24 platform rows plus one `any/any` (dotnet-install) | corrected (EN, DE) |
| 40 | Course `dev`: "proctor on 8791 built through `bun repo cargo`" | course `📜️script.ts:44-48` (`runDevStack`: proctor, then site); the cargo build is not visible there | reworded (EN, DE) |
| 41 | "`build`/`publish` uncached: the site is staged into a Vite store" | manifest only (`cache: false`) | rewritten (EN, DE) |
| 42 | "`preflight` and Docker targets need Debian 12 x64" | `🚀️deploy/🐳️docker-image/🟦️.ts:32` (native linux x64) | corrected (EN, DE) |
| 43 | Play `build` "publishes the pages into the products partition" | manifest: declares receipt `.🧾️store/build.json` | corrected (EN, DE) |
| 44 | `.cache/b/c` "sqlite claims" | `CA/💾️storage/🔒️enforcement/📦️budget/🟦️.ts:22,34`, `…/📥️opening/🟦️.ts:44-46` (lease directory, `maximumControlBytes` 1 MiB) | rewritten (EN, DE) |
| 45 | "HFS+ lacks `RENAME_SWAP`" | no call site, only `ImageProtocol` (UDIF, case-sensitive APFS) | rationale marked "not re-checked" (EN, DE) |
| 46 | "recreating `d` about 16 min, `g` 7 to 9 min" | `HANDOVER.md:84` has the 16 min run; no number for `g` | `d` dated, `g` "minutes, not timed" (EN, DE) |
| 47 | `bun repo cargo` "without `--manifest-path` the root workspace is targeted" | `L/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:48-54,73-75` (working directory's `Cargo.toml`; `--store`, `--configuration`, `--` mandatory) | corrected (EN) |
| 48 | Linux and Windows `hdiutil` remedy | Windows partitions are directories now; `windowsOpenHandle` refusal | Recovery sentence adapted (EN) |
| 49 | Verification table `bun test ./📚️library/…` | not a runnable path from the root | `bun test ./` removed from the first row (EN) |
| 50 | root README: "elevated terminal: the fixed cache images attach through `Mount-DiskImage`" | `🔵️.ps1:513-515` still demands elevation, its stated reason is the retired VHD | reworded, requirement flagged "under review" |
| 51 | root README: "tracked fixture paths reach 210 characters" | `git ls-files` maximum 226 UTF-16 units (2026-10-19) | 226, measurement named |
| 52 | root README: Rust outputs "~246 characters" | only the script warning ("240+", `🔵️.ps1:401`) | "240+ (the script's own warning)" |
| 53 | root README: "checksum-verified archives … installer script from the network" | `🔵️.ps1:260` (`Invoke-WebRequest` of pinned files, no `iex`); rustup is an executable | "downloads"; winget note added (`🔵️.ps1:524-540`) |
| 54 | root README toolchain list "Node.js 24 LTS + npm 11.7, Go 1.26" | `package.json` `engines.node` 24.15.0, pin Go 1.26.9, no npm pin | versions from the pins |
| 55 | root README Caches: "on Windows a fixed VHD that needs an elevated terminal" | see 36 | rewritten, provisional |
| 56 | root README: "Every `bun repo` call first publishes the declared bootstrap sources" at `⚡️caching/🚀️bootstrap/🌱️sources/🔣️.json` | file moved to `B/🚀️bootstrap/🌱️sources/🔣️.json`; `ensureBuildEnvironmentV1` runs for run, run-many, affected, exec, watch, graph, show only (`B/📜️script.ts:129`) | path and scope fixed |
| 57 | `.devcontainer/README.md`: locale "which Bazel and the build tools require" | `Dockerfile:72-78`, `devcontainer.json:108-109`; no code states a requirement | reworded |

## Confirmed (statement group, call site)

- Opening block and tool selection: `--tool` over `SEMIO_BUILD_TOOL` over `defaultTool` (`B/🧭️invocation/🟦️.ts:126-138`, policy `B/🔣️policy.json` `defaultTool: nx`), "Unknown build tool" (`:136`), nested export (`B/📜️script.ts:99-105`), "Conflicting build modes" (`:119`), `:<configuration>` suffix (`:109`), dashboard axis `build-tool` (`📋️project.json` parameters).
- Operations table and flags: catalogue states (`B/⚙️operations/🔣️catalog.json`, enum with `missing` in the schema), selector syntax (`B/🧭️selection/🟦️.ts:39-47`), validation before any tool (`:151-167`, called at `B/📜️script.ts:117-119`), affected rule and changed-file queries (`:53-76`), `--projects` refusal for affected (`:166`; Nx `BT/⚙️operations/🟦️.ts:11`), exec plan `repo-exec` uncached (`:96`, `B/⚙️operations/🟦️.ts:62-65`), Nx translation (`B/🧭️invocation/🟦️.ts:141-158`, `BT/⚙️operations/🟦️.ts:9-42`), reset/cache/graph/show/tools/dependencies/cargo routes (`B/📜️script.ts:121-176`), preflight policy and probes (`B/🩺️preflight/🟦️.ts`, `🔣️policy.json`: 8 GiB, tools, exchange), result shapes (`B/⚙️operations/🟦️.ts:7-59`, `BZ/⚙️operations/🟦️.ts:199-212`).
- Manifests: task fields (`B/🧬️schema/📋️project/🔣️.json` Task, ExecutionConfiguration, ReadyDeclaration), named groups (`B/🔣️policy.json`), input kinds (`BZ/📜️script.ts:165-176,226`), `dependsOn` forms (`:74-100`), cache classes and reasons (`B/🏃️execution/🧬️schema/💾️cache.json`), who attaches which class (`target-policy/🟨️.mjs:24-55`, `project-assembly/🟨️.mjs:46-51,149,222-228`, `playground-targets/🟨️.mjs:62-70,112-118,200-263`), policy order table (`target-policy/🟨️.mjs:47-55`, `CA/🔣️policy.json`), Nx does not read the decision (no `cache` reference in `L/🟨️.mjs`), receipt schema, path, repair and message (`B/🧾️store-receipt/🧬️schema/🔣️.json`, `🟦️.ts:35-45`), `.gitignore:17`.
- Nx: plugin and executors (`L/🟨️.mjs:59,77-99`, `executors.json`, `nx.json`), 13-module boundary and test (`BT/🔌️sdk/🔣️policy.json`, `🧪️tests/🟦️.ts:71-82`), cache directory (`nx.json` `cacheDirectory`, `BT/⚙️operations/🟦️.ts:45`).
- Bazel: workspace layout, label `t_`+16 hex, `s` link (`BZ/📜️script.ts:98,299-304,404-423`), decision order 1 to 9 (`:186-193`), declared outputs and snapshots (`:356-391,445`), requirement tags (`:356-366`, schema `Actions.kinds`), host path guard and message (`BZ/💾️storage/🟦️.ts:15-18`), label regex (`Actions.sources.label` equals the documented pattern), disk cache flags 40 percent / 10 s / 30 d / 3 d (`BZ/🧬️schema/🔣️.json` `CachePolicy`, `.bazelrc`), no `--batch` (`:512-540`), output base and 120 s idle (`BZ/💾️storage/🟦️.ts:38-41`, `Policy`), remote switch names, schema, flags and message (`BZ/💾️storage/🟦️.ts:43-52`, `CacheEnvironment`), cache status/clear/ledger (`BZ/⚙️operations/🟦️.ts:35-238`, `Policy.const`), storage layout (`BZ/💾️storage/🟦️.ts:20-23`), reset namespaces (`BZ/💾️storage/🗑️reset/🧬️schema/🔣️.json`), continuous tasks via `bazel run` (`:524-535`), darwin UTF-8 locale (`:523`).
- Command registry: tools, parameters, axes, verbs (`📋️project.json` `metadata.semio.dashboard`; law `L/🧪️tests/🎛️dashboard-control/🟦️.ts:25-64`), `generation` parameter (root `release`, Play `build`), `workspaceCommand` rule (audit 9), `semio` flags (`⌨️usage/🦀️.rs`), ticket tools file (`🎛️dashboard/🎮️registry/📋️facts/🦀️.rs:87`).
- Owner registry and zero-touch: routes (`✅️verify/🟦️.ts:47-52`), targets (`📋️project.json`), kinds (`🧾️owner-consistency.json`), writers (`crate-tree/🟦️.ts:460`, `🧱️scaffold/🟦️.ts:83`, `🧬️mutation-tree/🟦️.ts:410`), bare `verify` includes both (`🏁️gate-lanes/🟦️.ts:147-149`), nine closures (`🛗️zero-touch/🗂️closures/🔣️.json`).
- Setup: readiness refusals and messages (`BT/🏗️bazel/🩺️readiness/🟦️.ts:27-72`), progress every 10 percent, ready line (`B/📜️script.ts:142`), Bazel 8.4.2 rows (`Release`), bootstrap scripts (`🐚️.sh:304,450,517`, `🔵️.ps1:22,260,405-418,513,569`), `build-system.yml` jobs and steps (read in full), devcontainer files (compose, `devcontainer.json:37-38,41-45,100,108`, Dockerfile `:4-5,72-98`).
- Partitions and recovery: sizes (`CA/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`), `fixed.dmg`/UDIF/APFS (`ImageProtocol`), `.publication`, `-owners on`/`noowners`, messages (grep hits in `NB`), EPERM classifier and one purge per sync (`📥️install.json` `CacheLinkRefusalProtocol`), generation retirement (`BS/📦️dependencies/💾️storage/🟦️.ts:455`), clean protection (`📚️library/🧼️workspace-cleanup/🛡️protection/🟦️.ts:187,203`), `node_modules` symlink (`ls -l`).
- Go layout: `runCanonicalGoTests` and overlay (`L/🧪️testing/🐹️go-plan/🟦️.ts:65,127,223-236`). Dashboard section: ticket tools file, playground metadata in crate manifests.
- `SKILL.md`: every command and rule above; unchanged. `Help` bullet of `cli-fix`: `B/⚙️operations/📖️help/🟦️.ts:23-28`.

## Marked or left as recorded (no host to verify)

Real Nx cold/warm run, any real Bazel product run, store-receipt repair path, `build-system.yml` never run, Windows (everything), Linux and devcontainer beyond partition tests, macOS provenance EPERM end to end, product chains, `semio commands --check`. Statements inferred from earlier results: HFS+ rationale, Linux ext4/tmpfs run, `LANG=C` identity, 16 min sync, `RENAME_SWAP`. Heading anchors with emoji (`#-build-and-run`, `#-recovery`, `…#-daily-work`) cannot be checked without GitHub.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/README.md` (EN and DE; atomic replacements)
- `README.md` (root: Windows Setup, Build caches bullet, bootstrap-sources sentence)
- `.devcontainer/README.md` (two sentences)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`devcontainer-readme-owner-current`: `currentPreimage` and `approvalSha256`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🪪️devcontainer-current-source-approval/🔣️.json` (`currentPreimage`)
- Not changed: `.agents/skills/build/SKILL.md` (all statements confirmed), `AGENTS.md`, `package.json`, `bun.lock`, `.vscode/`, any source.

Re-pin (the documented way, fixups3 item 5): `.devcontainer/README.md` is a current-source owner (`readme-license-owner-leaves-v1.currentSourceRevisions`). It changed (9380 to 9625 bytes), so the approval document got the new sha256 `fe75b9b2e68113edc39a0473b540831cce671bca3e2b202d6e72fc673074faab` and size 9625, and the taxonomy row got the same preimage and the new `approvalSha256` `2e5880aeaedb257392e0597a99842a14fd8ad3ca77f4a6e2d9eddc8b9d133b3f`. The pinned policy and its baseline preimages are untouched. If the file changes again, rerun the same two-file update (document `currentPreimage`, then the taxonomy row).

## Verification (exact)

All from the repo root, explicit files, at most two processes, `SEMIO_TEST_ARTIFACT_DIR` set.
- `bun test --timeout 120000 ./<L>/🧪️tests/🏠️owner-projection-policy/🟦️.ts`: before the re-pin 34 pass 1 fail, after 35 pass 0 fail.
- `…/🔖️readme-current-source-revision/🟦️.ts`: 33 pass 0 fail (before and after). Also green: `🟢️readme-current-source-activation` 10, `🖼️runtime-taxonomy-asset-paths` 1, `👀️readme-reviewed-fixture-inputs` 2, `🗺️testing-readme-coordinates` 14. `🔧️build-surfaces`: 20 pass 1 fail (`native executable producers admit portable project declarations`, no relation to the docs).
- `B/⚙️operations/🧪️tests/🟦️.ts` + `B/🧾️store-receipt/🧪️tests/🟦️.ts`: 24 pass. `BT/⚙️operations`, `BT/🔌️sdk`, `BT/🔁️probe`, `B/🧭️selection`, `B/🧭️invocation` tests: 23 pass. `BZ/💾️storage`, `BZ/⚙️operations`, `B/🕸️graph/🧾️registry-consistency`, `B/📥️source/🛗️zero-touch` tests: 67 pass 4 skip.
- With `SEMIO_TEST_BAZEL_BINARY=.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel`: storage + actions files 20 pass 2 fail (the actions file needs the envelope); the labelled-emoji test alone `-t "the real Bazel declares every labelled emoji"`: 1 pass.
- `bun ./🧰️…/🏗️build/📜️script.ts test bazel`: 41 pass 34 fail (open item 1).
- `bun test --timeout 120000 …/🎛️dashboard-control/🟦️.ts`: 8 pass 2 fail.
- `bun ./📜️script.ts verify build-owners`: passed. `bun ./📜️script.ts verify zero-touch`: passed. `bun repo --help`: prints usage.
- Structure: code fences balanced in all four files, every table has constant column counts, EN and DE section headings correspond, backticked repository paths resolve (checked with a script; the remaining misses are generated or relative by convention).
- Not run: `bun repo run`, `dependencies sync`, cargo, product activations, `semio commands --check`, the Bazel operations test with the real binary (it clears the real products cache).

## Open items

1. The aggregate Bazel suite is red: `B/🔧️tools/🏗️bazel/🧪️tests/📋️contract/🟦️.ts` "unchanged normalized constraints" (two entries differ from the baseline; probably the `HostPaths`/`CacheEnvironment` additions of the sandbox and cache work) and the cascade. Owner: `bzl-pin`/`bzl-sandbox`. Docs state the red result with a date; update after the fix.
2. Windows is provisional: `🔵️.ps1:513-515` and `BT/🏗️bazel/🩺️readiness/🟦️.ts:33` still demand elevation and name the retired VHD as reason; `xplat-win2` should reconcile messages and then the Windows row (repo README EN and DE, root README, `.devcontainer` unaffected). During my run the tree briefly missed `NB/🧱️fixed/🗂️system/🟦️.ts` (imported by the readiness module); it was gone from the importers again a few minutes later.
3. `dashboard-control` law: fixture row `@semio-tech/repo-dashboard-rs:workflow status` (owner: `cmdreg`), two tests with built-in 5 s and 15 s timeouts fail under load, and the test prints `[DEBUG]` lines on purpose (permanent test output, not mine).
4. `.agents/skills/build/SKILL.md` says the dashboard request "supersedes older launch registration guidance in repository instructions"; `AGENTS.md` still requires `launch.json` and the brief says the `.vscode/` decision is pending. Left as is; needs a user decision.
5. Pre-existing, not part of today's docs: repo README "Configuration" link `../.🧬semio/🦑️repo/config.toml` does not resolve; `.devcontainer/README.md` mentions `.github/dependabot.yml` and a `🧪️tests/🔁️lifecycle` path relative to another directory.
6. `Developer Operations` (keybinding `Ctrl+B n`) was not traced to Rust source; it is covered by the dashboard README.
