# 📌 Handover: GET-CODEBASE-RUNNING-END-TO-END-AGAIN

Snapshot of the ticket state as of 2026-10-18, from `coordination.md` (last entry 22:00 plus a 22:45 note), the `result-*.md` and `audit-*.md` reports, and `compile-plan.md`. Nothing was compiled, built, or run to produce this file. Every "DONE" below means the executor reported it, usually as static evidence only. Re-verify before relying on it.

Ticket folder, referred to as C: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`
Repo root: `/Users/ueli/Documents/semio`
Short paths like `result-cc.md` are relative to C.

Status legend: DONE (reported complete), DONE-static (edits and checks done without compiling or running), PARTIAL, BLOCKED, OPEN, NO-RESULT (tool folder exists, no report file).

---

## 1. Goal and acceptance

Goal of this ticket: make the codebase run end to end again, and make the build tool-neutral and zero-touch on devcontainer, native Windows, native macOS and native Linux.

Concrete products (acceptance = the target reaches task execution and succeeds):

| Product | Targets | Source of truth |
|---|---|---|
| Architecture course | `bun repo run @teaching/architecture-quiz:{dev,test,test-e2e,publish,build}`, `@teaching/proctor:build`, CI (`.github/workflows/architecture-quiz.yml`) | `audit-teaching-course.md` |
| Semio-tech play | `bun repo run @semio-tech/semio-tech-play:{activate-prototype,serve}` plus its E2E | `audit-semio-tech-play.md`, `result-play.md` |
| MIT-Bestand demonstrator | `bun repo run @semio-tech/mit-bestand-demonstrator:{activate-prototype,serve,test-e2e}` | `audit-mit-bestand-demonstrator.md`, `result-demo.md` |

Supporting goals:

- Untangling: every hand-authored file under 1,000 lines. Every Rust crate closure inside the source gate (4,000 active lines, 32 files, 262,144 bytes, per `readRustSourceClosureV1`). Evidence: `audit-monolith-census-6.md`, `audit-monolith-census-7.md`, `audit-untangling-state.md`.
- Layering: dependency-cruiser violations driven down (`audit-layering.md`, `result-lay0.md`).
- Tests: TypeScript, vitest, Go, and the source-text "law" suites green (`result-tst1.md`, `result-tst2*.md`, `result-law1..5.md`).
- Bazel route: `bun repo --tool=bazel run <target>` for the products above (`result-bzl.md`, `audit-bazel-targets.md`).
- Tool-neutral build: no runtime dependency on a specific host state, zero-touch bootstrap, 256-character path cap for authored files (`AGENTS.md`).

---

## 2. Current state per area

### 2.1 Build CLI (`bun repo`)

State: boots, but the full graph and run chain is not yet green.

- Fixed 01:20 to 02:00 (`coordination.md`): pending publication intents from killed runs (moved to `stale-publication-intents/`), receipts without `mountDirectory` rebuilt from the covered directory inode (originals kept as `*-image.json.orig`).
- Lock-order bug in `🏗️build/📜️script.ts` fixed. `cargo` now runs before the dependency bootstrap and pin (05:00). `ensureBuildEnvironmentV1` inspects read-only against a shared dependencies store (11:40).
- Zero-touch: 379-file CLI import closure rewritten to relative imports (`relativize-bare-imports.ts`, plus 13 more re-entering at 22:45). `bun repo --help` boots without `node_modules` (1.4 s, `result-catfix.md`, `result-jsi.md`).
- Zero-touch gap, unverified on a clean tree: `openRepositoryCachePartitionV1` in `budget/📥️opening/🟦️.ts` now creates the parent `⚡️cache/rust` recursively (`result-cc.md`, Phase 1 item 2).
- Launcher fix: `rustup/📥️installation/🟦️.ts` `acquireArchive` now lstat-probes before hashing (`result-cc.md`).
- Tracked-operation trap found, not changed: `settle()` rethrows the secondary "admission closed" error and hides the root cause (`result-cc.md`).
- Raw `cargo` and `RUSTC` are refused by the compiler-ingress gate (`.cargo/config.toml` `[build] rustc`). Every compile goes through `bun repo cargo`.

### 2.2 Native cache images (APFS, recovery)

State: APFS adopted for the store images. Recovery paths exist. Cause of one deletion is unproven.

- `native-build/💾️storage/🧬️schema/🔣️.json`: `ImageProtocol.filesystem` changed from "Case-sensitive HFS+" to "Case-sensitive APFS" (01:45). HFS+ returns ENOTSUP for `renameatx_np(RENAME_SWAP)`, which publication and binding use.
- Dependencies image recreated as APFS, 8 GiB fixed (`.cache/b/d`). Tools image `.cache/b/t` is APFS. Other partitions may still be HFS until they first need a swap. Old HFS image moved to `.cache/b-d-hfs-retired` (delete after green sync, see section 5).
- The `fixed.dmg` images of `.cache/b/*` vanished at 02:08 (external sweep). `result-sweep.md`: neither `clean` nor `cache-prune` explains it in its current form. Trigger not proven; needs root `fs_usage`.
- `.fseventsd` filled a 64 MiB lease image. Fix: `no_log` written at attach (`cleanup/🎛️command`) and applied manually to mounted images (`result-cc.md`).
- Dependencies image hit ENOSPC because `m/tmp` accumulated 4.9 GiB of installer leftovers. Emptied manually. Product follow-up: clear `m/tmp` at sync start (not done, `result-deps.md`).
- Leaked test images: 34 detached (`result-leak.md`). 21 `ex-*` / `case-candidates-*` images under the shared scratchpad `ta/` remain mounted, owned by other runs.
- Nx namespace images: now under a top-level partition `.cache/b/n`, 1.5 GiB APFS, funded children (`result-nxpart.md`). See 2.5.
- Disk: 238 GB free (cc state, 13:25). Re-check before any large build.

### 2.3 Dependency sync (`bun repo dependencies sync --all`)

State: GREEN at 18:21 for generation `4151d2262cb4a4e7a25a` (`result-deps.md`, line 2). Stale again after 19:01. Not confirmed green since.

- Earlier green: generation `441fd20cf4b20f6dcafe` (12:15, `result-deps.md` Update 12:15).
- Stale cause at 19:01: peer edits to `✏️s/🔌️plugins/**/📦️packages/🟦️typescript/package.json` (procedural, fem, layout, raster, stdio registry, zip, jpg). `bun repo show projects` at 19:04 then failed with "Current Source-owner execution changed during consumption" (`result-deps.md`, Final state).
- Latest coordination entry: 20:30 "deps: batch 2 synced, final sync running" (`coordination.md`). No GREEN line is recorded after 18:21 in `result-deps.md`. Treat the current state as unknown until the chain in section 3 succeeds.
- Manifest batches: batch 1 added 124 undeclared dependency pairs in 26 `package.json` files (root 51, ✏️s 19, 🌎️hub 8, play 7, plugin registry 7, and others). Batch 2 (brepjs, d3-geo, katex, `@dnd-kit/*`, react-resizable-panels, `@types/*`) done 19:40. `bun.lock` regenerated by `lock --all`.
- Manifest freeze: "MANIFEST FREEZE" at 13:00, "STRICT FREEZE" at 18:30 (package.json, bun.lock, storage and revision modules). Deps was resumed 19:30 for batch 2. No explicit lift recorded.
- Pending manifest work, all waiting on the deps owner:
  - Exports entries from the layering WPs (section 2.8).
  - 2 laws that need manifest edits (`result-inv.md`).
  - 6 dangling `./products/repo/modules/...` export keys in `🧰️framework/📦️packages/🟦️typescript/package.json` (`result-inv.md`).
  - `tree-sitter-wasms` 0.1.13 in the actor TS devDependencies (`result-deps.md`).
  - `@types/node` ^22.19.6 in ✏️s, ♻️mit-bestand, 🎓️teaching, 🏢️semio-tech, 🌎️hub.
  - Playwright `chromium-1248` vs installed `chromium-1234` (ruling pending, see section 4).
- Design choices made by the deps executor (`result-deps.md`):
  - Installed-path cap raised from 256 to 1024 (npm content under `.bun/` reached 291 chars). Open decision.
  - Parent-identity fence compares dev, ino and mode, not mtime. Open decision.
  - Versionless workspace providers count as `0.0.0`.
  - Unready generations are resumable when their staged inputs match. Otherwise discarded.
  - Scripts excluded from the install material.
- Performance: warm sync about 16 min (18:21 run). Full-fence interval 32 bindings. Obsolete ready generations are not garbage-collected (18+ generations, image about 50% used). Product follow-up.
- Tests in `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts`: the new ones pass when run directly. The rest need the repo-run envelope (`Original parent Script process invocation required`).
- Test-dependency list for suites: `result-tst2-deps-and-vitest.md`, the input for the deps owner.

### 2.4 Graph load (`bun repo show projects`)

State: PARTIAL. Cold inference cut from hours to 7 to 9 min. Publish tail not completed.

- `result-gperf.md`: cold 4 h+ (extrapolated) became 7 to 9 min for 3,656 projects. Warm 70 s. Ownership fence about 11 s per walk, down to 1.2 s. The graph was never published in one complete run (tail blocked by peer edits and storage image verification).
- Root causes (already fixed): per-project cache records about 1.4 MB, 97% duplicated, against a 1 GiB optional-cache quota (thrashed every run). Progress and liveness checks on every call. Per-project revalidation of shared facts. Authority self-checks re-hashing source on every call.
- `result-lq.md`: nonwaiting maintenance on ordinary commands is now queued (30 min bound, cancellable, progress reported). First-time pool provisioning creates the parent directory.
- Verification still open: a full `bun repo show projects` and `bun repo show project @teaching/architecture-quiz` (`result-gperf.md`, section 5).
- `show projects` needs about 40 min on this machine. Every manifest edit during that window invalidates it.

### 2.5 Nx partition

State: DONE-static. Acceptance not demonstrated.

- `result-nxpart.md`: the Nx cache pool is a top-level partition `nx` at `.cache/b/n` (budget policy `📦️budget/🔣️policy.json`, capacity 1.5 GiB, `maximumArtifactBytes` 512 MiB store charge). Namespaces are funded children at `<n/m>/<tool20><key20>/native`. Live: `.cache/b/n` provisioned (APFS). Old `g/m/nx` removed.
- Acceptance (`bun repo run @teaching/architecture-quiz:check` reaching task execution) NOT demonstrated: the run aborts in the dependencies bootstrap ("Store dependency sync incomplete") before Nx code runs.
- `result-own.md`: nested attach (`-owners on`) works on the real graph image.
- `result-nxneutral.md`: root lookup made neutral. AGENTS.md line 7 conflict reported to the user (section 4).
- Nx coupling audit (`audit-nx-coupling.md`): 3,375 `repo:script` executor uses in product `project.json` files, 0 `nx:run-commands` there, 934 `nx:run-commands` in 40 tracked ticket files, 0 `@nx/*` imports, 1 comment mentioning `bun nx`.
- User memory note (not verified here): `NX_FORCE_REUSE_CACHED_GRAPH=true` reportedly cut Nx graph construction from minutes to about 24 s. Evaluate before relying on it.

### 2.6 Cargo compile (`result-cc.md`, `compile-plan.md`, `cc-tools/`)

State: launcher GREEN. Batches partly green. Several batches failing. Many not yet run.

Launcher: `bun repo cargo --store=shared --configuration=prototype -- check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-geometry-primitives --lib` exits 0. A call without `--manifest-path` targets the root workspace.

Runner: `cc-tools/run.mjs` (`--parallelism=4`, `--keep-going`), wrapper `cc-tools/rb.sh <idx>`. `<idx>` is the 0-based array index into `cc-tools/batches.json` (400 entries; section labels `2. Offl`, `3.1`..`3.34`, `4. Feat`, `uicarve root/framework`, `plug …`, `adhoc`). Logs and markers: `🗑️generated/cc-bNNN.{log,errs,skipped,done}`.

Batch state from the `.done` markers in `🗑️generated/`:

| Idx | Scope | Result |
|---|---|---|
| 034, 035 | framework lib 1, 2 | GREEN (exit 0) |
| 036 | framework lib 3 | exit 1, 3 errors. Peer manifest churn (3d-brep-construction-unit-tests `testkit`; `ui/tests/targets-wgpu-prepared-laws`) |
| 063, 065, 067, 069, 071, 073, 075 | root lib | GREEN (exit 0). 1 to 13 crates skipped behind `semio-framework-ui` at the time |
| 064, 066 | root test | GREEN (exit 0; 064 skipped 11) |
| 068 | root test | exit 101, 5 errors. Test-companion privacy (`tco`) |
| 070 | root test | exit 1, 3 errors |
| 076 | root test | exit 101, 149 errors. Test-companion privacy and library carve defects (`result-tco.md`) |
| 077-084 | root lib | SKIPPED: every crate depends on `semio-framework-ui`. Re-run now that uicarve has carved it |
| 085 | stdio plugin workspace lib | exit 101, 7 errors (`ui_wgpu::wgpu` missing in the middle of the c6ui move) |
| 147, 148 | teaching lib and tests | GREEN (exit 0) |
| 149 | hub lib | exit 1, 3 errors (peer rewrite of `os-mcp-capabilities/Cargo.toml`) |
| 300 | uicarve framework | GREEN (exit 0) |
| 330 | list root / cstep priority | GREEN (exit 0) |
| 900, 910, 911 | ad hoc (indices 900+, `CC_CMD`, not in batches.json) | 900 GREEN; 910 and 911 exit 0 |
| 901, 902 | ad hoc | 901 exit 1 (3 errors), 902 exit 101 (63 errors) |

Not yet run: 086-146 (plugin workspaces, lib and tests), 150-172 (hub tests, feature and wasm lanes), 301-317 (uicarve consumers and ui tests), 331, 332-365 (plugin workspaces), 366 (`semio-framework-job` `original-close-context` test), and the `compile-plan.md` section 4 lanes.

Known compile risks:
- `compile-plan.md` section 2 (offline lock refresh) conflicts with `result-cc.md`: `update --workspace --offline` fails for `zip`. Framework and root locks were refreshed online instead. Use online refresh, or add `zip` to the offline index first.
- `compile-plan.md` section 0 gates B1 and B2 (receipt `mountDirectory`, headroom 4,262,842,368 B against 4,294,967,296 B) were not re-probed.
- `cstep` changed the close API across about 213 plugin files (grant-based, 75 refusals) and queued 73 crates for cc (`result-cstep.md`). The WFC headless driver was rewritten in `wfcj` (`result-wfcj.md`).
- Compiles since 2026-10-16 are absent in the untangling audit (`audit-untangling-state.md`). All crate evidence after that is static.

### 2.7 Untangling

State: file-level DONE-static. Crate gate DONE-static for the known outliers, one accepted ceiling.

File level (`audit-monolith-census-6.md`, `audit-monolith-census-7.md`):

- Census 6: 295 files over 1,000 lines. Split by c6 executors: c6db1 (15), c6db2 (18), c6flow (11), c6inf (13), c6fin (11), c6fin2 (11 of 14, 3 reported not split), c6fw (15), c6plug (8), c6plug2 (16), c6repo (P73-P78, 474 importers re-pointed), c6repo2, c6store (27 to 146 children), c6ts, c6tsfw (16 of 18; ShardClient and DocumentBrowserActorReservation redesigned by `cls`), c6ui (31 to 202 dirs). All DONE-static.
- Census 7: 44 hand-authored files over 1,000 lines. `c7w` split 37 Rust and 4 TS files. Excluded: 2 `jcoprobe.js` bundles (`@generated` header). Left unsplit: `values/🦀️.rs` (generated). Largest produced file 945 lines. Some single functions remain over 600 lines (`paint` 841, `fact` 945, `shact` 774, `cdisp` 694, `winit` 613) because they are single indivisible functions.
- Post-split, `coordination.md` (20:30): "file-level untangling complete (all hand-authored files < 1,000 lines except generated)".
- Go: `gocomp` (46,930 lines to 134 dirs), `gocomptest` (23,637 to 68 test dirs), `gocli`, `gogql`, `gotix`, `gotree`, `gocoord` split. `gocanon` converted 332 Go files into member directories. `glob` removed 89 `pub use child::*` glob re-exports (`result-glob.md`). `gocomp` documents 2 over-1,000 exceptions.
- Structural deviation: `result-gocomp.md` uses directories instead of sibling files. Go layout ruling: one `🐹️.go` per registered member dir.
- Audit findings that were fixed later: `audit-split-verification.md` (02:27) recorded 77 glob re-exports and one `export type` compatibility re-export. `glob` removed the globs. `pai` was refused at 02:27 and reported done at 03:30 (coordination).

Crate gate (`readRustSourceClosureV1`, canonical reader `meas.ts`):

- `semio-framework-ui` all-features closure: 258 files / 56,932 lines (3.5 MB) before. After `uicarve` (14:05): 2 files plus 32 tui and wgpu crates, each at or below 0.892. Gate: `Math.min(policy, crate metadata)`, so the 12,000-line override never applied. This was the blocker for 257 of 549 root members (`result-cc.md`).
- `hdr3` (05:1x, 3,320 crates): 0 lib and 0 test over the gate. Near-gate: `semio-framework-plugin-reactor-turn` 0.909.
- `hdr4` (20:30): 7 of 8 crates at or below 0.90. `semio-framework-plugin-instance-app` 0.988 down to 0.950, accepted as ceiling pending redesign (`VcsArtifactApp` has about 120 inherent methods; the orphan rule forces them into the type's crate).
- `hdr4b`: compatibility re-exports removed.
- `cg1` (job, process3d-host, brep-boolean): all at or below 0.90.
- `cg2`: flow-flow at or below 0.875 (the 0.906 editor-leaves crate fixed). bim-model applied, every crate at or below 0.854 in the sandbox census.
- `cg3`: bim unmirror completed. Generator 18 GB blow-up fixed.
- `lex`: layout export gate 0.996 to 0.676.
- `cls`: `ShardClient` 1,655 to 219 lines and `DocumentBrowserActorReservation` redesigned. Actor-transport 179 of 179 (`result-cls.md`).

Caveat: no crate has been compiled since 2026-10-16 (`audit-untangling-state.md`). Every split is verified only by lossless line oracles, `rustc --emit=metadata` differentials, and scans.

### 2.8 Layering (dependency-cruiser)

State: policy fixed. Total violations roughly halved in the measured WPs. Most remaining edges need new `exports` entries, which are blocked by the manifest freeze.

Baselines and totals:

- Whole repo: 11,025 error violations at start (`audit-layering.md`, 19,808 cruised modules). `lay0` policy fixes brought it to 9,696, then 9,693 (`result-lay0.md`).
- Policy fixes (no code changes): `no-core-path` exemption for the `base` subset folder (taxonomy), builtin and `bun:*` handling, `node_modules`/`target`/`pkg` exemptions, installed-package realpaths for the renderer allow-list. `audit-layering.md` estimated about 387 config false positives in the 11,025 baseline. `result-lay0.md` reports the drop to 9,696 (9,693 final) from these policy fixes and code fixes together.
- `no-circular` in the OS: 346 to 219 whole OS, 217 of which are in the renderer (untouched).
- Cross-plugin violations: 0. All 92 plugin cycles lie inside one plugin each.

WP status (WP ids from `audit-layering.md`):

| WP | Scope | Before to after | Status | Evidence |
|---|---|---|---|---|
| WP1 | plugins to framework | 1,157 (audit) | NO-RESULT. `lw1-tools/` and `lay1-tools/` present (`lay1-tools/touched*.txt`, 45 files) | none |
| WP2 | plugins to os | 333 (audit) | NO-RESULT. `lw2-tools/` present | none |
| WP3 | plugins to plugins | 989 (audit) | NOT addressed. 92 intra-plugin cycles remain; `base` subset exemption done in policy | `result-lay0.md` |
| WP4 | hub to framework | 713 to 563 | PARTIAL. 150 fixed, 563 need `exports` | `result-lw4.md` |
| WP5 | s-dev, s-tests, mit-bestand | 300 to 256 | PARTIAL. 154 of 256 need `exports`; 3 not code-fixable; leftovers fixed in `lw5b` | `result-lw5.md`, `result-lw5b.md` |
| WP6 | os (non-renderer) to os | 442 to 267 (no-circular 129 to 2) | PARTIAL. 255 remaining need `exports` | `result-lay6.md` |
| WP7 | os (non-renderer) to repo | 696 to 470 | PARTIAL. 127 `exports` entries needed | `result-lay7.md` |
| WP8 | os (non-renderer) to framework/ui/other | 464 to 285 | PARTIAL. 69 edges need `exports` | `result-lw8.md` |
| WP9 | renderer | 2,114 (audit) | Exports spec only (see below). Renderer code work not done | `lay9-tools/` |
| WP10a | repo product to `framework/modules/process` deep imports | 1,203 to 428 | DONE-static for part A | `result-lw10a.md` |
| WP10b | repo product, remaining | 1,725 deep imports (audit) | NO-RESULT. `lw10b-tools/` present | none |
| WP11 | other products to repo | 112 (audit) | NO-RESULT | none |
| WP12 | framework modules, packages, ui | 1,178 (audit) | NO-RESULT. `lay12-tools/` has 4 files | none |

Exports spec for the deps batch:

- Per-WP specs: `lay6-tools/exports-needed.json`, `lay7-tools/exports-needed.json` (127 entries), `lw4-tools/exports-needed.json`, `lw5-tools/exports-needed.json`, `lw5b-tools/exports-needed.json`, `lw8-tools/exports-needed.json` (69 edges), `lw10a-tools/exports-needed.json`, `lw10b-tools/exports-needed.json`.
- Consolidated spec, the one to apply: `lay9-tools/exports-needed.json`, written 22:51. Summary keys `summary`, `byPackage` (29 packages), `uiContract` (63 entries). Totals: 337 new export entries, 638 importer edges. Generated by `lay9-tools/exports-spec.ts`, with `lay9-tools/manifest-gaps.ts` for gaps. No `result-lay9.md` exists.
- `repoapi-tools/exports.ts` is an analyzer that reads exports. No output spec named `repoapi` was found in the ticket. Treat `lay9-tools/exports-needed.json` as the consolidated exports spec.
- Applying it needs the manifest freeze lifted (section 2.3). Apply through the deps owner as one manifest batch, then one green sync.

Other layering notes:

- `inv` (framework to product imports): 196 to 0 via a provider loader and ports. 2 laws need manifest edits (`result-inv.md`).
- `pin`: build-graph inputs now include script provider designations and framework provider loader closure (`result-pin.md`). Follow-up from the coordinator: declare provider modules as inputs of the 15 `cache:true` framework targets (`coordination.md`, 15:30).
- `lay6`: 85 exports entries recorded as `lay6-tools/exports-needed.json`.

### 2.9 Tests

State: partial green everywhere. Rust law suites static only.

TypeScript:

- `tst1` (course and repo-lib): `quiz` vitest 254 of 254, `quiz` tsc 0 errors, repo-lib tsc 180 down to 5, `checkQuizCatalog` through the launcher (`result-tst1.md`).
- `tst2` bun suites (`result-tst2-suites.md`, header): before 1,064 of 1,465 green (11,025 pass / 1,054 fail tests); after 1,261 of 1,465 green (12,848 pass / 707 fail). Remaining failures by class: source-law drift 151, missing dependency 19, harness environment 13, needs native or browser 12.
- Law suites, outside `🔨️modules` and inside: `law1` 34 suites green (`result-law1.md`, 35 still red: 4 missing or wrong packages, 7 harness). `law2` 40 of 63 green (767 pass / 65 fail; `repo` scope 408 of 664 suites green, 256 red, of which about 110 are launcher-gated). `law3` 21 of 25 green (`result-law3.md`, rows 55 to 79). `law4` production and structure fixes; locks split without cargo (`result-law4.md`, no count). `law5` laws 34 and 46 redesigned; durable journal fixtures fixed (`result-law5.md`).
- `repot` and `repot2` (`result-repot.md`, `result-repot2.md`): `🦑️repo` red suites groups 1 to 4 of 108. Before 0 of 100 green (1,146 pass / 307 fail) to 17 green (1,205 pass / 250 fail). Repo tests 2,284 to 2,488 passing (`coordination.md`, 20:30).
- `vit` (`result-vit.md`): 27 plain configs, 3 green before, 7 green after (animate, quiz-react, pets-react, jack lsp, root, styling, plugin-registry). 11 green with the harness that supplies missing packages and the Playwright browser.
- `tsc` (`result-tsc.md`): 23 `tsconfig.json` under framework, teaching and hub; 13 at zero errors after. Remaining errors: stale wasm-bindgen output in flow core (`semioSetRuntimeAssets`), missing packages.
- `tscp` (`result-tscp.md`): the whole `✏️s/` run (14,038 files, Check 501 s) 70 errors, 11 fixed. The rest are missing packages (brepjs, d3-geo, katex, `@dnd-kit/*`, etc.) and build outputs (`@semio-tech/stdio-docx` has no `dist`).

Go:

- `gofin`, `gofin3`, `gofix`, `gofix2`, `gogql`, `gotix`, `gotree`, `gocli`, `gocomp`, `gocomptest`, `gocoord`, `gocanon`: Go short suites green in a throwaway copy (`result-gofin.md`: model 770, tree 170, graphql 66, tickets 31, cli 363 pass with `-short`).
- `gofix2`: short suite 74 down to 3 failures, all TS-toolchain (`@semio-tech/compiler-syntax-ecma-ts` missing). Also a stale 6 MB `breaches/testing.json` cache.
- Pre-existing failure: `TestSystemPolicy/the_checked-in_devcontainer_block_equals_its_derivation` (`.vscode/settings.json` drift). Partly fixed in `gofix` (`result-gofix.md`).
- Remaining client issue: about 65 non-short `TestExhaustive*` tests still read the real checkout (`result-gofin.md`, `result-gofin3.md`).
- INCIDENT (`coordination.md`, 05:50): `gofix2` non-short tests rewrote about 3,450 ticket JSON and 204 goal JSON files in the real checkout. Restored from HEAD content. Uncommitted ticket edits by peers could have been reset. The tests now use a throwaway repo.

Rust:

- Compiles: see 2.6. Law suites in `🧪️tests` are static only.

### 2.10 Bazel

State: static phase DONE. Runtime phase BLOCKED.

- `result-bzl.md`: target names verified. `tools setup bazel` OK, readiness 18 of 18 (13:00 in `coordination.md`). Closure cap 512 to 2048. Three graph-load import fixes done.
- Blocked on `node_modules`: still a dangling symlink after about 2 h of polling. Deps `sync --all` attempts 9 to 14 failed on fence and profile checks. `bun repo show projects` cannot load without it (`Cannot find module '@semio-tech/process/wasm-build'`).
- Not run: `quiz:test` dry-run, the other seven dry-runs, real `quiz:test`, `proctor:build`, Bazel suite, Nx `readyWhen` check.
- Requested target list from `result-bzl.md`: `bun repo --tool=bazel run <target>` for `@teaching/architecture-quiz:{test,publish,dev}`, `@teaching/proctor:build`, `@semio-tech/semio-tech-play:{activate-prototype,serve}`, `@semio-tech/mit-bestand-demonstrator:{activate-prototype,serve}`.
- `audit-bazel-targets.md`: 35 `prepare-<variant>` and 35 `activate-<variant>` framework-os-dev targets are not in the tracked manifest. They are synthesized by graph inference. The 257-unit dependency path refusal blocks Bazel as much as Nx. Dry-run flag: `--dry-run`.
- `audit-build-system-bazel.md`: partitions are fixed-capacity sparse images (`fixed.dmg` on macOS, `fixed.vhd` on Windows, `fixed.img` on Linux). No one-line recovery command is documented.

### 2.11 Products

Each product has a static (done) layer and a runtime layer (blocked).

Architecture course (`audit-teaching-course.md`, `result-tst1.md`, `result-quizdock.md`):

- Static findings from the audit (`audit-teaching-course.md` 5c):
  1. Proctor image build broken since 2026-10-14 (`.cargo/config.toml` `rustc` ingress script inside the Docker context). Quizdock fix: `.cargo` kept out of the Docker context, builder config at `🚀️deploy/Dockerfile.cargo.toml`, dev stack builds the proctor through `bun repo cargo`. Image build unverified (needs Docker daemon).
  2. Dev stack spawns raw `cargo` (`PROCTOR🏗️bootstrap/🟦️.ts` L226). Works only under the launcher env.
  3. `bun repo` entry point was broken at audit time. Every course entry point goes through it.
  4. `site.legal` empty. Bun version split (`bun.lock` vs pinned 1.3.14). 17 to 18 taxonomy errors on teaching root files. `pet-walk` e2e rehearsal failures.
  5. Working-tree edits uncommitted: `.github/workflows/architecture-quiz.yml`, `.vscode/launch.json` (MM), `.vscode/🧩️launch.seed.jsonc`. `QUIZ🧪️tests/🧪️deploy/🟦️.ts` staged.
- Docker daemon not running (`/var/run/docker.sock` missing, 2026-10-18).
- Blocking for runtime: build-graph and deps chain (section 3); the Docker daemon for the image; the `bun repo` entry point.

Semio-tech play (`audit-semio-tech-play.md`, `result-play.md`):

- DONE-static: describe scripts removed; route is now `<pkg>:component`; tool-neutral wording; 35 lanes hard-coded in `project.json` pinned to `playActivationLanes()` by the `playactivation` test.
- Nothing staged now: `:6033` has no listener, `dist` contains only `reports/`. `serve` needs `activate-prototype` first.
- Memory notes are stale (`activate-dev`, `dist/♻️activation/dev`, 26 or 28 lanes). Ticket scripts `describe-*.sh` call `activate-dev` and `@semio-tech/<p>-plugin:describe` (gone).
- Open: `package.json` scripts call `bun --cwd "../.." repo run` where AGENTS.md says `nx` (decision). `components:complete` has no definition found. `SEMIO_RENDERER=react` in the launch entry is not read.
- Blocking for runtime: deps chain; `bun repo run` through the graph.

MIT-Bestand demonstrator (`audit-mit-bestand-demonstrator.md`, `result-demo.md`):

- DONE-static: 335 launch routes now `framework-os-dev dev-<variant>-<renderer>-prototype`; ship renamed to release; demonstrator prepare deps added.
- Still STALE in `♻️mit-bestand/🧺️demonstrator/📋️project.json`:
  - S3: `prepare` target calls `prepare-<variant>-react` without a profile (likely broken).
  - S4: `🔨️modules/🧩️runtime/📜️script.ts` L23 points at `plugin/.../dist/<profile>/🔌️plugin-modules` (real root is `browserProductRootsV1(…).modules`).
  - S5: L30 points at `plugin/📇️registry/dist/sessions/...` (real root is `.sessions`, a cache).
  - S1: about 333 launch entries use `workspace:dev -- <plugin>`. Root project has no `dev` target.
- `workspace:dev-mcp-engine` (launch "🛠️dev-mcp-engine") points to a nonexistent target (`result-demo.md`, `audit-other-products.md`).
- Acceptance E2E: Playwright (`🔨️modules/🧪️e2e/🎚️config/🟦️.ts`), 7 `test(` blocks covering 8 panes. Needs chromium 1248 vs installed 1234, and a browser download (decision).
- Blocking for runtime: deps chain; playwright decision; fix S3 and S4/S5.

Other products (`audit-other-products.md`, `result-osdev.md`):

- `workspace:start` (main repo entry, launch "▶️start") points to a target missing from the root `project.json`. Broken.
- `@semio-tech/framework-os-dev` CLI fails on `@semio-tech/process/wasm-build` (`🌊️flow/🕸️wasm/🌐️browser/📦️publication`). Blocks any target. Owner must restore the package or import path.
- Launch rows that do not resolve: `@semio-tech/repo-cli-rs:{run,daemon,install,preferences,workflow}` (dashboard moved to `repo-dashboard-rs`).
- Launch audit (`audit-launch.md`, 19:08 snapshot): 462 routes name projects not in manifests, 49 undeclared targets, seed and launch drift. Verdicts (1) PASS, (2) FAIL, (3) FAIL, (4) PARTIAL. Reconcile against the real graph after `show projects`.

---

## 3. Critical path: next commands in order

Prerequisite: confirm no peer is editing `package.json`, `bun.lock` or the storage and revision modules. Freeze status is in section 2.3. Check for running `bun repo` processes first (`ps`), since a concurrent run causes "Quiescent pre-pin cache storage admission required" refusals.

Step 1. Deps chain (repo root). Lock only if manifests changed since the last green generation (`4151d2262cb4a4e7a25a`, 18:21):

```sh
cd /Users/ueli/Documents/semio && bun repo dependencies lock --all
cd /Users/ueli/Documents/semio && bun repo dependencies sync --all    # resumable; repeat until exit 0; warm about 5-16 min
cd /Users/ueli/Documents/semio && bun repo show projects              # cold inference about 40 min; expect exit 0
cd /Users/ueli/Documents/semio && bun repo tools setup nx
```

The chain script `scratchpad/run-chain.sh` (in the session scratchpad, which is wiped on process restart) runs lock, sync, show and nx in that order. Recreate it if missing.

Step 2. Acceptance for the course test target (first real Nx run, `result-nxpart.md`):

```sh
cd /Users/ueli/Documents/semio && bun repo run @teaching/architecture-quiz:test
cd /Users/ueli/Documents/semio && bun repo run @teaching/architecture-quiz:check
```

If it aborts at "Store dependency sync incomplete", return to step 1.

Step 3. TypeScript suites, once step 1 is green:

```sh
cd /Users/ueli/Documents/semio && bun repo run @teaching/architecture-quiz:test
cd /Users/ueli/Documents/semio && bun repo run repo:test-javascript-storage
```

Then the tst2 runner (`tst2-tools/`) for the 1,465 bun suites, and the vitest configs (`vit-tools/runvt.py`). Expect the 13 zero-error configs in `result-tsc.md` to remain green.

Step 4. Cargo, via the launcher only. Take the fleet mutex per `compile-plan.md` section 0 if other executors are running. Probe first:

```sh
pl() { cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- "$@"; }
pl check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml --message-format=short
```

`perl alarm` is used because macOS has no `timeout` command (`MEMORY` note). Then the refresh (online, not offline; see section 2.6) for framework and root:

```sh
pl update --workspace --manifest-path 🧰️framework/Cargo.toml
pl update --workspace --manifest-path Cargo.toml
```

Then batches, in this order:

1. Re-run the skipped root lib batches now that uicarve carved the ui closure: `zsh C/cc-tools/rb.sh 77` through `zsh C/cc-tools/rb.sh 84` (use `rb.sh <idx> wait` to wait for a run in progress).
2. Failing batches, fix then re-run: 036 (framework 3, after a quiet window), 068 and 076 (test-companion privacy, executor `tco` notes), 070, 085 (`ui_wgpu::wgpu` missing; check `ui/🎯️targets/🧊️wgpu/🦀️.rs`), 149 (hub `os-mcp-capabilities` manifest), 901 and 902 (adhoc).
3. Never-run batches: 086-146, 150-172, 301-317 (uicarve consumers and ui tests), 331, 332-365, 366.
4. `compile-plan.md` section 4 feature and wasm lanes, one `pl` call each.
5. Crate lists from the c6 and cstep reports: `result-c6flow.md` section 6, `result-c6inf.md`, `result-cstep.md` (73 queued crates), `result-lfix.md` and `result-lfix2.md`, `result-wgpu2.md`.

Step 5. Layering exports (after the manifest freeze is lifted and step 1 is green):

Apply `lay9-tools/exports-needed.json` as one manifest batch through the deps owner, then:

```sh
cd /Users/ueli/Documents/semio && node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err
```

Expect the count to drop from roughly 9,693 toward the per-WP totals in section 2.8. Then run the `verify` dependency-lint gate, which uses the same roots with `--output-type err` (`audit-layering.md`, section 1).

Step 6. Products, after step 1:

```sh
cd /Users/ueli/Documents/semio && bun repo run @teaching/architecture-quiz:build
cd /Users/ueli/Documents/semio && bun repo run @teaching/architecture-quiz:dev            # interactive
cd /Users/ueli/Documents/semio && bun repo run @semio-tech/semio-tech-play:activate-prototype
cd /Users/ueli/Documents/semio && bun repo run @semio-tech/semio-tech-play:serve            # port 6033
cd /Users/ueli/Documents/semio && bun repo run @semio-tech/mit-bestand-demonstrator:activate-prototype
cd /Users/ueli/Documents/semio && bun repo run @semio-tech/mit-bestand-demonstrator:serve
```

Then `:test-e2e` for the course and the demonstrator (Playwright; see the decision in section 4). Do not use `workspace:start` or `workspace:dev` (both broken, section 2.11).

Step 7. Docker (course image). Start the Docker daemon, then the image checks through `bun repo run @teaching/architecture-quiz:docker-image-build` and `docker-image-check`.

Step 8. Bazel, after step 1:

```sh
cd /Users/ueli/Documents/semio && bun repo --tool=bazel run @teaching/architecture-quiz:test --dry-run
```

Repeat for the other seven targets in `result-bzl.md`. Then re-measure the closure with `🗑️generated/bzl-closure/measure.ts`.

Step 9. Zero-touch check on a clean tree (not yet done): verify the `rust` cache parent creation (section 2.1) and `bun repo --help` without `node_modules`.

---

## 4. Open decisions for the user

1. AGENTS.md line 7 vs. repo practice. AGENTS.md says `package.json` MUST call `nx`. The repo calls `bun repo run` from play and teaching `package.json` scripts, and 3,375 `repo:script` executors exist. `nxneutral` reported the conflict. Needs a ruling before step 1.
2. Git-tracked generated files and stale ticket copies. `coordination.md` (22:00): 37 generated files and about 370 stale ticket copies are tracked in git. Needs `git rm --cached`, which agents must not run. This ticket folder itself has 2,583 git-tracked files.
3. Staged index leftovers. `zz-gocomp-stage/*.go` (AD) from `gocomp`, staged at 04:00. Staged deletions of the publication-owner and receipt-conflict laws, origin unknown (21:30). `.3a0f1e4` staged at the start. Owner to decide. Do not commit or stash.
4. Playwright chromium: 1234 installed, 1248 expected by the registry (21:30). Decide whether `tools setup` or deps owns the browser download. Downloading browsers needs explicit permission.
5. Docker daemon not running. Course image build, check and publish unverified. Quizdock fix untested.
6. Instance-app 0.950 ceiling. `semio-framework-plugin-instance-app` accepted at 0.950 (`hdr4`) pending a redesign of `VcsArtifactApp`.
7. Installed-path cap 1024 (npm content) against the 256 authored-file rule. Deps decision (`result-deps.md`).
8. Parent-stamp semantics: identity (dev, ino, mode) instead of strict mtime. Deps decision (`result-deps.md`).
9. Compile-plan offline refresh vs online refresh (section 2.6). Online chosen for framework and root.
10. Test-companion privacy: widen items to `pub`, or move tests back (`result-tco.md`, `result-cc.md` open item 3).
11. WP1, WP2, WP3, WP9, WP10b, WP11, WP12: no result reports. Assign owners or accept as out of scope for this ticket.
12. Fresh-clone behavior of the zero-touch `rust` parent creation (untested).
13. Remaining `[DEBUG]` logs. `result-deps.md` reports none left. Run `grep -rn "\[DEBUG\] "` over repo source before closing.

---

## 5. Close-out checklist

Do these only after the critical path is verified.

- Delete `orig/` source snapshots in tool folders. 13 directories, 978 files (examples: `fnsplit-tools/orig`, `c6store-tools/orig`, `uicarve-tools/orig`, `jsi-tools/orig`, `c6db2-tools/orig`). `coordination.md` (18:30) counted 494 paths over the Windows limit. Measured today from the repo root, no path outside `🗑️generated` exceeds 256 characters; 147 paths inside `🗑️generated` do. AGENTS.md forbids copying checked-in sources into the ticket.
- Delete `🗑️generated/` (1,306 entries, 3.2 GB). Keep only what the close requires. Do not delete `cc-tools/`, `*-tools/` inputs, or any report.
- Delete tool-generated outputs in the ticket: logs, `*.out`, `*.errs`, `*.skipped`, `*.done`, dated scratch files. Keep scripts, specs, configs and reports (AGENTS.md).
- Scratch `node_modules`: none inside C (0 found). Check the session scratchpad, which is outside the repo, and remove copies.
- `.cache/b-d-hfs-retired`: delete after the green sync (`coordination.md`, 01:28). Also remove `*-image.json.orig` backups once no receipt needs rollback.
- `stale-publication-intents/` backups: keep until the publication owner confirms they are not needed.
- Lift the freeze: the strict freeze from 18:30 is still recorded as in effect. Tell all executors and the deps owner explicitly.
- Remove `[DEBUG] ` logs (see section 4, item 13).
- Ticket close: `ticket_close` through the repo MCP with an explicit path (`MEMORY` note). The repo MCP was not connected during `lq`, and `ticket_open` returned a malformed result (`coordination.md`, 00:00). Verify the connection first. Summary and file list must cover created, updated and removed files (AGENTS.md).
- Commit nothing and stash nothing (AGENTS.md). Staged leftovers stay for their owners (section 4, item 3).

---

## 6. Per-executor index

Prefix to report, status, open items. "static" means the executor reported static evidence only. Reports listed as NO-RESULT have a tool folder but no report file.

### 6.1 Build, dependencies, cache, Nx, Bazel

| Prefix | Report | Status | Open items |
|---|---|---|---|
| deps | `result-deps.md` | PARTIAL: GREEN 18:21, stale 19:01, batch 2 synced 20:30 | Re-verify; freeze; GC generations; `m/tmp` at sync start; path-cap and fence decisions |
| gperf | `result-gperf.md` | PARTIAL (inference 7-9 min cold) | Publish tail; full `show projects` and `show project` unverified |
| nxpart | `result-nxpart.md` | DONE-static | Acceptance blocked by deps |
| nxneutral | `result-nxneutral.md` | DONE | AGENTS.md line 7 decision |
| own | `result-own.md` | DONE | Next blocker was the Nx pool (fixed by nxpart) |
| lq | `result-lq.md` | DONE | Real-CLI proof blocked |
| pin | `result-pin.md` | DONE | Follow-up: declare provider modules as target inputs (15 `cache:true` targets) |
| sweep | `result-sweep.md` | DONE (root cause of `fixed.dmg` deletion unproven) | Needs root `fs_usage` |
| leak | `result-leak.md` | DONE | 21 images under scratchpad `ta/` owned by other runs |
| bzl | `result-bzl.md` | PARTIAL: static DONE, runtime BLOCKED | Needs working `node_modules` |
| osdev | `result-osdev.md` | DONE-static; CLI blocked by `@semio-tech/process/wasm-build` | Owner must restore the package |
| loc | `result-loc.md` | DONE | Locale and Terminology have no `Default` (ruling); env on launch routes |
| cc | `result-cc.md` | PARTIAL: launcher GREEN, batches partly green | Section 2.6 |
| cstep | `result-cstep.md` | DONE-static (159 files; 73 crates queued) | Crate compile |
| wfcj | `result-wfcj.md` | DONE-static | WFC headless driver on original-caller API |
| wfcj2 | `result-wfcj2.md` | DONE-static | Fem2d coverage check fixed |
| cls | `result-cls.md` | DONE-static | ShardClient and Reservation redesigned |
| tsc | `result-tsc.md` | DONE-static (13 of 23 zero-error) | Stale flow-core wasm bindings; missing packages |
| tscp | `result-tscp.md` | DONE-static (70 errors, 11 fixed) | Missing packages and build outputs |
| vit | `result-vit.md` | DONE-static (7 of 27 plain green, 11 with harness) | Playwright browser |
| tst1 | `result-tst1.md` | DONE (course quiz 254/254; repo-lib tsc 5) | Requires `bun repo run` |
| tst2 | `result-tst2.md` | DONE (1,261 of 1,465 suites) | Remaining classes in `result-tst2-suites.md` |
| tst2-suites | `result-tst2-suites.md` | Per-suite table (reference) | Source-law drift 151, missing dep 19, harness 13, native/browser 12 |
| tst2-deps-and-vitest | `result-tst2-deps-and-vitest.md` | Input list for deps owner | Missing packages per suite |
| repot | `result-repot.md` | DONE-static | Repo suites |
| repot2 | `result-repot2.md` | DONE-static (17 more green, groups 1-4) | Groups 5+ of 108 |
| tco | `result-tco.md` | PARTIAL (companions; 68, 76 still failing) | Library carve defects in batch 76 |
| law1 | `result-law1.md` | DONE (34 suites green) | 35 still red (missing packages, harness) |
| law2 | `result-law2.md` | DONE (40 of 63 green) | Repo scope 256 red, about 110 launcher-gated |
| law3 | `result-law3.md` | DONE (21 of 25) | Remaining 4 rows |
| law4 | `result-law4.md` | DONE-static | Refresh 171 stale consumer locks; `BIM_BLESS=1` fixtures; `cargo metadata` energy and epw locks; tree-sitter-rust wasm async closures (deps) |
| law5 | `result-law5.md` | DONE-static (laws 34 and 46 redesigned) | `wasm-cleanup` release-failure asymmetry; 21 leaked images |
| inv | `result-inv.md` | DONE-static (framework to product 196 to 0) | 2 laws need manifest edits; 6 dangling export keys |
| lex | `result-lex.md` | DONE-static (layout export gate 0.996 to 0.676) | Open items in section 6 of the report |
| hdr3 | `result-hdr3.md` | DONE-static (0 over gate) | `plugin-reactor-turn` 0.909 |
| hdr4 | `result-hdr4.md` | DONE-static (7 of 8 at or below 0.90) | `instance-app` 0.950 ceiling |
| hdr4b | `result-hdr4b.md` | DONE-static | Compatibility re-exports removed |
| cg1 | `result-cg1.md` | DONE-static | 179 `Cargo.lock` files need offline refresh |
| cg2 | `result-cg2.md` | DONE-static (flow-flow; bim-model applied) | Compile |
| cg3 | `result-cg3.md` | DONE-static (bim unmirror complete) | Generator hardened |
| uicarve | `result-uicarve.md` | DONE-static (ui closure 56,932 to 2 files plus 32 crates) | Compile |
| wgpu | `result-wgpu.md` | DONE-static | Compile |
| wgpu2 | `result-wgpu2.md` | DONE-static | Compile |
| plugfix | `result-plugfix.md` | DONE-static (14 crates normalized) | Compile |
| lfix | `result-lfix.md` | DONE-static (fixture API port, 35 plugins) | Compile |
| lfix2 | `result-lfix2.md` | DONE-static | Compile |
| c7w | `result-c7w.md` | DONE-static (37 Rust, 4 TS) | Compile |
| flowcore | `result-flowcore.md` | DONE-static (rename `🌊️flow/🫀️core` to `🎁️browser-package`) | `manifest-batch.py` written, not run; `kind-only-basename` shims |

### 6.2 Untangling, census, file splits

| Prefix | Report | Status | Open items |
|---|---|---|---|
| c6db1 | `result-c6db1.md` | DONE-static (15 files) | Compile list in section 6 |
| c6db2 | `result-c6db2.md` | DONE-static (18 files) | Compile |
| c6flow | `result-c6flow.md` | DONE-static (11 files) | Compile list section 6 |
| c6inf | `result-c6inf.md` | DONE-static (13 files) | Compile `--lib --tests` and `--features testkit` |
| c6fin | `result-c6fin.md` | DONE-static (11 files; 62 folders) | Compile |
| c6fin2 | `result-c6fin2.md` | DONE-static (11 of 14; 3 not split) | Reported; 3 files left |
| c6fw | `result-c6fw.md` | DONE-static (15 files; largest root 471) | Taxonomy schema defect |
| c6plug | `result-c6plug.md` | DONE-static (8 files) | Compile |
| c6plug2 | `result-c6plug2.md` | DONE-static (16 files; one 1,108-line function) | Compile |
| c6repo | `result-c6repo.md` | DONE (P73-P78; 474 importers) | Library root barrel removed |
| c6repo2 | `result-c6repo2.md` | DONE-static (TypeScript only) | Test helpers split |
| c6store | `result-c6store.md` | DONE-static (27 files to 146 children) | Compile |
| c6ts | `result-c6ts.md` | DONE-static | PDF schema as JSON import |
| c6tsfw | `result-c6tsfw.md` | DONE-static (16 of 18) | `wgpu-frame-worker` taxonomy inputs |
| c6ui | `result-c6ui.md` | DONE-static (31 files to 202 dirs) | `semio-framework-ui` carve (now in uicarve) |
| fnsplit | `result-fnsplit.md` | DONE-static (tjc, srs, brep register) | Trace-verified |
| glob | `result-glob.md` | DONE (89 globs removed) | Verify no re-introduced globs |
| jsi | `result-jsi.md` | DONE-static (2,120 lines to 151-line root) | Browser reload semantics |
| led2 | `result-led2.md` | DONE-static (O-05, O-07, O-01, O-10 closed) | One unmounted test not attributed |
| spr | `result-spr.md` | DONE-static (granted multi-record History decoder) | Compile; decoder law |
| gocanon | `result-gocanon.md` | DONE-static (332 Go files) | `fixtures/🐹️sample.go` shared fixture; vestigial test-layout rule |
| gocomp | `result-gocomp.md` | DONE-static (46,930 lines to 134 dirs) | 2 documented over-1,000 exceptions; staged `zz-gocomp-stage` |
| gocomptest | `result-gocomptest.md` | DONE (23,637 lines to 68 dirs) | Verdict DONE; test execution not re-checked here |
| gocli | `result-gocli.md` | DONE (cli and move split) | Pre-existing failures to `gofix` |
| gogql | `result-gogql.md` | DONE-static | `applyAutofixes` no production caller (product decision) |
| gotix | `result-gotix.md` | DONE-static (tickets split) | Open items in report |
| gotree | `result-gotree.md` | DONE (hooks and tree split) | 3 tree tests failing (missing g1 fixture) |
| gocoord | `result-gocoord.md` | DONE (9 files split; declarations moved verbatim) | Test execution not re-checked here |
| gofix | `result-gofix.md` | DONE (10 Go tests fixed; applyAutofixes decomposed) | Devcontainer drift |
| gofix2 | `result-gofix2.md` | DONE (short suite 74 to 3; TS-toolchain failures) | Incident: ticket JSON rewrite, restored |
| gofin | `result-gofin.md` | DONE (short suites green in copy) | About 65 non-short exhaustive tests read real checkout |
| gofin3 | `result-gofin3.md` | DONE-static | Client duplicates module stack |
| catfix | `result-catfix.md` | DONE (taxonomy catalog load error fixed) | Folders created after last sweep; leaked `TEST-GOAL-*` dirs |
| vtax | `result-vtax.md` | DONE (validateTaxonomy to 20 validators; 5,803-variant differential) | None recorded |
| pfx | none (tool folder `pfx-tools/` present) | NO-RESULT | Unknown |
| lw1 | none (`lw1-tools/`) | NO-RESULT | WP1 |
| lw2 | none (`lw2-tools/`) | NO-RESULT | WP2 |
| lw7 | none (no tool folder) | NO-RESULT | Launched per `coordination.md` 22:00; no output found |
| lw10b | none (`lw10b-tools/`, exports spec `exports-needed.json`) | NO-RESULT | WP10b |
| tjcov | none (`tjcov-tools/`) | NO-RESULT | Unknown; tool folder modified 22:24 |
| osplay | none (`osplay-tools/`) | NO-RESULT | Play-related; unknown |

### 6.3 Layering

| Prefix | Report | Status | Open items |
|---|---|---|---|
| lay0 | `result-lay0.md` | DONE (11,025 to 9,693) | 92 intra-plugin cycles (WP3) |
| lay1 | none (`lay1-tools/`) | NO-RESULT | Tool folder only (45 files: renames, touched sets, cycle scan); no report |
| lay6 | `result-lay6.md` | DONE (WP6 442 to 267) | 255 need `exports` |
| lay7 | `result-lay7.md` | DONE (WP7 696 to 470) | 127 `exports` entries |
| lay9 | none (`lay9-tools/`) | Consolidated exports spec exists (`exports-needed.json`, 337 entries) | Manifest freeze; renderer code work not done |
| lay12 | none (`lay12-tools/`) | NO-RESULT | WP12 |
| lw4 | `result-lw4.md` | DONE (WP4 713 to 563) | 563 need `exports` |
| lw5 | `result-lw5.md` | DONE (WP5 300 to 256) | 154 need `exports` |
| lw5b | `result-lw5b.md` | DONE (policy false positives, stdio-docx, glb ownership) | `docx` and `inventory-ts` need `dist` builds |
| lw8 | `result-lw8.md` | DONE (WP8 464 to 285) | 69 `exports` entries |
| lw10a | `result-lw10a.md` | DONE (WP10a 1,203 to 428) | Part B (lw10b) |

### 6.4 Audits (read-only, Haiku)

| Audit | Covers | Key output |
|---|---|---|
| `audit-layering.md` | Dependency-cruiser inventory | 11,025 violations; WP1-WP12 breakdown |
| `audit-monolith-census-6.md` | Files over 1,000 lines (census 6) | 295 files |
| `audit-monolith-census-7.md` | Files over 1,000 lines (census 7) | 50,284 files in scope; 44 over 1,000 |
| `audit-untangling-state.md` | Monolith split and crate gate state | Gate 4,000 lines / 32 files / 262,144 bytes; compile ingress gate |
| `audit-split-verification.md` | Split losslessness | Content PASS; 77 glob re-exports FAIL (fixed by `glob`); `pai` refused (later done) |
| `audit-go-split-canonical.md` | Go split vs canonical build and gate | Go toolchain compiles every `*.go` directory; resolved by `gocanon` |
| `audit-nx-coupling.md` | Nx dependence | 3,375 `repo:script`, 934 ticket `nx:run-commands` |
| `audit-launch.md` | `.vscode/launch.json` and seed | 462 routes unmatched, 49 undeclared targets; verdict (2) FAIL, (3) FAIL |
| `audit-other-products.md` | Other products | `workspace:start` and `workspace:dev-mcp-engine` broken; `osdev` fix |
| `audit-teaching-course.md` | Course | Docker and ingress, `bun repo` entry point, working-tree edits |
| `audit-semio-tech-play.md` | Play | Stale memory notes; nothing staged; `serve` needs `activate-prototype` |
| `audit-mit-bestand-demonstrator.md` | Demonstrator | S1 to S6 stale references |
| `audit-bazel-targets.md` | Bazel targets for 7 requested targets | Synthesized framework-os-dev targets; 257-unit refusal |
| `audit-build-system-bazel.md` | Build system and Bazel | Partition layout; no documented one-line recovery |

Briefs present but not summarized here: `brief-executor.md`, `brief-go-split.md`, `brief-layering.md`, `brief-split-executor.md`, `followup-law1-script-product-imports.md` (not read for this handover; check before closing).

Supporting files: `compile-plan.md` (784 crates, 34 workspaces, 119 batches, `pl` form in section 1); `coordination.md` (chronological log); `🎫️ticket.json`; `stale-publication-intents/`; `demo-rewrite-launch.ts`, `osdev-targets.ts`, `relativize-bare-imports.ts`, `scan-bare-imports.ts` (top-level tool scripts).
