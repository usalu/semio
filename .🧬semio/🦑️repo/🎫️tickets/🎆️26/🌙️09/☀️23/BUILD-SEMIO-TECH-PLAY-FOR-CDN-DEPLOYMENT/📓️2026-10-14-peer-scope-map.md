# 📓 Peer Scope Map (2026-10-14)

Read-only audit. No git-modifying command, worktree, ticket MCP call, generated-folder access or source edit was made. Evidence is static (file reads, `grep`/`find`, whole-file line counts, file mtimes at about 16:53 CEST). No compiler, admission run or build was executed.

Peer tickets:
- Fleet A (Cargo/Rust, "crate production boundaries"): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`
- Fleet B (repo build tool, Bazel): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL`

Coverage: about 190 markdown reports across both tickets. Read in full: `crate-production-boundaries.md`, `architecture-audit-current.md`, `build-unit-headroom.md`, `cargo-manifest-replanning.md`, `cache-hard-bound-plan.md`, `plugin-production-current.md`, Bazel `final-report.md`, `integration.md`, `configured-inputs.md`, `bazel-design.md`. Read by headings or keyword hits: `rust-unit-artifact-protocol.md`, `exact-cargo-bounded-entry.md`, `nx-bounded-artifact-admission.md`, `cargo-storage-and-overlap.md`, `final-audit.md`, `build-core.md`, `source-changes.md`, `bazel-validation.md`, `bazel-module-setup.md`. The remaining reports were keyword-scanned only. Their conclusions are not verified here.

## 0. Summary

1. Fleet A splits Rust into real Cargo owners, each with a `rust-source-boundary` metadata block. 54 framework manifests carry that block; about 45 of them are new cuts from 14:26 to 16:49.
2. The global admission ceiling is 4,000 active (cfg-selected) lines, 262,144 bytes and 32 files per workspace compilation unit. It is configured in `⚡️caching/🔣️policy.json:502-507`. Per-crate metadata can only tighten it (`Math.min`).
3. The component build driver (`runNativeBuildDriverV1`) refuses over-ceiling units before any compile, once per pass. A monolith such as `semio-framework-plugin` (612 `.rs` files, about 183k whole-file lines in its tree) would be refused. The peers also report its closure refuses on generated reactor input.
4. The root `workspace:release` script (`📜️script.ts:14866`) calls the driver without `storage` or `sourceBoundary`. The driver's first step (`requireNativeStorageReservationV1`) throws on a missing reservation. Static finding: the release path is broken as staged.
5. Fleet B (Bazel and the neutral build CLI) is now the public interface. Commands are `bun repo run <project>:<task>[:<config>]`, `bun repo --tool=bazel ...`, `bun repo tools setup all`. Launch entries and package scripts no longer call `nx`. Bazel does not own Cargo units; `MODULE.bazel` has no rules_rust.
6. Plugins, renderer, UI and store are NOT free. Fleet A is actively cutting them, and about 100 plugin `.rs` files changed in the last 3 hours.
7. Fleet A's named "not touched" areas are touched in part. Fleet B renamed `.nx-artifact.json` markers in `✏️s/🔌️plugins/🗄️stdio` and `🌎️hub`.
8. Candidate low-churn plugin directories for our fleet: `✏️s/🔌️plugins/` items `✒️writer`, `➗️mathematical`, `🀄️wfc`, `🌿️vcs`, `🎞️animate`, `🎬️sequence`, `🏗️fem`, `💠️lowpoly`, `🔋️energy`, `🧱️block`, `🪐️space` (0 recent edits). Confirm with the user first.
9. Conflict with AGENTS.md: it says `package.json` must call `nx`. Both peer lanes now use `bun repo run` in `package.json`, and 249 package manifests no longer carry Nx metadata. Our lanes should follow the neutral CLI.
10. Report path: this file.

---

## 1. Cargo packages: created, cut, planned

Status key: DONE = manifest and boundary metadata on disk, gate evidence reported (native execution only where noted). PROGRESS = manifest modified or created 16:42 to 16:58, not yet green. PLANNED = named in a peer report, no manifest yet.

### 1a. Cargo owners (on disk, verified by `Cargo.toml` scan)

| Package (`name`) | Directory (under `🧰️framework/`) | Status | Evidence |
| --- | --- | --- | --- |
| semio-framework-ui-arena | `🔨️modules/🖱️ui/🌳️retained/🏟️arena` | DONE, native green | `crate-production-boundaries.md`: UI arena and four retained native laws passed under fixed 32 MiB backing |
| semio-framework-font-anta-latin-regular (+14 font units) | `🔨️modules/🖼️assets/🔤️fonts/...` | DONE (metadata, source gate) | 15 font units with 1 input each, all `B` (boundary block) |
| semio-framework-os-identity | `🛍️products/💻️os/🔨️modules/🪪️identity` | DONE | Native History identity library compiled in the same report |
| semio-framework-os-history (+schema, codec, identity, record-decode) | `🛍️products/💻️os/🔨️modules/📡️spr/📜️history/...` | DONE; identity native compiled | `crate-production-boundaries.md` |
| semio-framework-os-channel, os-directory, os-directory-client, os-vcs, os-command, os-inference, os-pack, pack-record-token | `os/🔨️modules/📡️spr/🧵️channel`, `📇️directory/...`, `🌿️vcs`, `📡️spr/🎮️command`, `💡️inference`, `🎒️pack/...` | DONE (metadata + gates) | Boundary metadata present, DAG table in `crate-production-boundaries.md` |
| semio-framework-plugin-component-receipt | `os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt` | DONE | Listed in DAG |
| semio-framework-value-retirement-owner | `🔨️modules/🌱️value/♻️retirement/📦️owner` | DONE | Listed in DAG |
| semio-framework-ui-text | `🔨️modules/🖱️ui/🔤️typography` | DONE | Listed in DAG |
| semio-framework-ui-theme | `🔨️modules/🖱️ui/🎨️styling/🎭️theme` | DONE | Listed in DAG |
| semio-framework-ui-action | `🔨️modules/🖱️ui/🎬️action` | DONE | Listed in DAG |
| semio-framework-ui-select-behavior | `🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧭️behavior` | DONE | Listed in DAG |
| semio-framework-ui-retained-tree | `🔨️modules/🖱️ui/🌳️retained/🌳️tree` | DONE | Listed in DAG |
| semio-framework-ui-raster-owner | `🔨️modules/🖱️ui/🖼️raster/🧵️ownership` | DONE metadata; native NOT run | `crate-production-boundaries.md`: assertions not yet executed |
| semio-framework-icon-contract | `🔨️modules/🖼️assets/🔣️icons/🧬️schema` | DONE | Listed in DAG |
| semio-framework-ui-layout, ui-declaration | `🔨️modules/🖱️ui/📜️declaration/📐️layout`, `.../📜️declaration` | DONE | Listed in DAG |
| semio-framework-ui-surface-identity | `🔨️modules/🖱️ui/🪟️surface/🪪️identity` | DONE (16:53 touched) | Created after `architecture-audit-current.md` (16:38) |
| semio-framework-os-store-message-ledger | `os/🔨️modules/🏪️store/📒️ledger/📨️messages` | DONE | Listed in DAG |
| semio-framework-os-store-cursor | `os/🔨️modules/🏪️store/🧭️cursor` | DONE | Listed in DAG |
| semio-framework-os-store-projection | `os/🔨️modules/🏪️store/🎞️projection/🧬️schema` | DONE | Listed in DAG |
| semio-framework-os-store-command | `os/🔨️modules/🏪️store/📬️command` | DONE | Listed in DAG |
| semio-framework-os-store-envelope | `os/🔨️modules/🏪️store/✉️envelope` | DONE | Listed in DAG |
| semio-framework-pack-record-token | `🔨️modules/🎒️pack/🧬️schema/📡️record-token` | DONE | Listed in DAG |
| semio-framework-os-kernel | `🛍️products/💻️os/📦️packages` | IN PROGRESS (kernel Store cuts, 16:15) | `crate-production-boundaries.md`: about 54k selected lines still in kernel Store |

### 1b. In progress (on disk, not yet reported green)

| Package | Directory | Last touched | Note |
| --- | --- | --- | --- |
| semio-framework-ui-draw-contract | `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types` | 16:42 | The `architecture-audit-current.md` recommended next cut (draw types), now started |
| semio-framework-ui-prepared-staging | `🖱️ui/.../🎟️prepared/📤️staging` | 16:42 | Same cut (Prepared upload staging) |
| semio-framework-ui-prepared | `🖱️ui/.../🎟️prepared` | 16:49 | Same cut |
| semio-framework-ui (root UI crate) | `🖱️ui/📦️packages` | 16:53 | Root wide feature aggregate; must shrink |
| semio-framework-plugin-host, plugin-interpreter, plugin-interpreter-runtime, plugin-describe, plugin-guest-contract, os-run, os-mcp, os-renderer-wgpu | `os/🔨️modules/🔌️plugin/...`, `🏃️run`, `🌉️mcp`, `renderer/.../wgpu` | 16:56 to 16:58 | Manifests with NO boundary block; attribution unknown. Plugin area is NOT free |

### 1c. Planned next (named in reports, no owner manifest yet)

| Target | Source of the plan | Blocker stated |
| --- | --- | --- |
| UI draw/prepared chain (remaining files) | `architecture-audit-current.md` "Recommended Next Cut" | Must not be one crate (Prepared 207,982 bytes + draw-types 75,909 bytes > 262,144) |
| UI layout to Widgets metric move (TREE_ROW_HEIGHT, layout line 378) | `architecture-audit-current.md` | Metric must move into Theme first |
| UI mounted-layout (Engine::UiSurfaceToken import) | `architecture-audit-current.md` | Surface identity contract needed (now `surface/identity` exists) |
| UI input / events / paint / engine SCC | `architecture-audit-current.md`, `crate-production-boundaries.md` | Needs explicit worker and painting ports |
| Kernel Store live ledger, replay, codec, composition (~54k lines) | `crate-production-boundaries.md` "Remaining SCC Work" | Needs exact read/retirement ports; "Do not attempt a 54k-line leaf" |
| Plugin crate ownership (`semio-framework-plugin`) | `crate-production-boundaries.md` | "Cannot be claimed until generated reactor/compiler dep-info establishes its unit closure" |
| 45 new packages: native compile for each | `crate-production-boundaries.md` Completion Evidence | Only the units listed in 1a as native green are qualified |

### 1d. Fleet B creates no Cargo crates

Bazel changes are task-level only. `MODULE.bazel` is a 42-byte `module(name = "semio", version = "0.0.0")`. Cargo units stay `repo:script` commands.

---

## 2. Laws: admission ceiling, enforcement, release refusal

### 2a. Policy values (authoritative)

| Field | Value | Where |
| --- | --- | --- |
| maxActiveLines | 4000 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` lines 502 to 507 (`"sourceBoundary"`) |
| maxSourceBytes | 262144 | same, line 504 |
| maxSourceFiles | 32 | same, line 505 |

Read by `repositoryStrictStorageV1` in `⚡️caching/💾️storage/🔒️enforcement/🟦️.ts` (line 12: `boundary=policy.storage.sourceBoundary`), returned as `sourceBoundary` (line 14), and passed to every store (`🏗️compiler/🟦️.ts` line 31: `sourceBoundary:declaration.sourceBoundary`).

Per-crate override is only a tightening, not relaxation. Example: `semio-framework-plugin` declares `maxSourceBytes = 1048576`, `maxSourceFiles = 128`, `maxActiveLines = 12000` in `Cargo.toml`, but the enforced values are the minimum with the global policy, so its declaration is inert.

### 2b. Enforcement code

| Step | File (under `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/`) | Line | What it does |
| --- | --- | --- | --- |
| Admission function | `📥️source/🎟️admission/🟦️.ts` | 14 to 16 | `admitCargoSourceBoundariesV1(graph, workspaceRoot, policy, control, port)` |
| Per-unit limit | same | 15 | `unit.boundary ? {maxSourceBytes: Math.min(policy..., unit.boundary...), ...} : policy` |
| Skip non-workspace units | same | 15 | `if(unit.provenance.kind!=="workspace")continue;` |
| Test kind | same | (none) | No test-unit exemption; `port.cfg` passes `--test` for test units |
| Mid-read refusal | same | 15 | `if(seen.size>limits.maxSourceFiles||total>limits.maxSourceBytes) throw Error("Rust crate Source boundary exceeded before compiler: ...")` |
| Final refusal | same | 16 | `if(inputs.length>limits.maxSourceFiles || bytes>limits.maxSourceBytes || lines>limits.maxActiveLines) throw Error(...)` |
| Line counting | `📥️source/🟦️.ts` | 20 | `activeLines` = cfg-selected physical lines (`selected.split("\n").length`); embedded `include_str!` inputs count bytes but no lines |
| Policy type | `📥️source/🎟️admission/🟦️.ts` | 9 | `CargoSourceBoundaryPolicyV1 {maxSourceBytes, maxSourceFiles, maxActiveLines}` |
| Generated input refusal | `📥️source/🟦️.ts` | 24 | `if (reference.base || reference.expansion) throw Error("Generated Rust input requires compiler ownership: ...")` |

Call sites of `admitCargoSourceBoundariesV1` (all verified by grep):

| Caller | File | Line |
| --- | --- | --- |
| Component/release driver, once per pass | `🧩️component-pass/🟦️.ts` | 284 |
| Ordinary build scheduler | `🎟️admission/🟦️.ts` | 304 |
| Cargo artifact build (uses `policy.cwd`) | `🟦️.ts` | 245 |
| Cargo relay | `🌊️relay/🟦️.ts` | 28 |
| Exact Cargo laws | `🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` | 266 |
| Repo Cargo admission | `🛍️products/🦑️repo/.../📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts` | 37 |
| Repo Cargo unit selection | `🛍️products/🦑️repo/.../⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🟦️.ts` | 30 |
| Framework rust package script | `🧰️framework/📦️packages/🦀️rust/📜️script.ts` | 43 |

Separate unit-route check (not via admission): `🛍️products/🦑️repo/.../⚡️caching/🦀️cargo/📦️units/📜️script.ts` line 27 applies its own per-file `row.size>boundary.maxSourceBytes`, total, and `activeLines` checks before `compileRustUnitV1`. `📥️descriptor/🟦️.ts` line 17 refuses a unit unless its package metadata has exactly `maxActiveLines`, `maxSourceBytes`, `maxSourceFiles`.

Provenance classification (`🕸️units/🟦️.ts`): `kind = registry | git | external-path (manifest outside workspaceRoot) | workspace`. Only `workspace` is admitted.

### 2c. Does the play release path refuse over-ceiling units?

Yes for the component driver, by static reading:
- `runNativeBuildDriverV1` (`🧩️component-pass/🟦️.ts` line 281) calls `captureCargoCompilationGraphV1` then `admitCargoSourceBoundariesV1(graph, options.cwd, options.sourceBoundary, ...)` for every pass (line 284). Any refusal throws before the pass compiles.
- Dev and component paths reach it with the correct arguments: `🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` lines 75 and 93 pass `storage: store.storage` and `sourceBoundary: store.sourceBoundary`.
- Pass plan: `componentPassPlanV1` (`🛍️products/🦑️repo/.../⚡️caching/🦀️cargo/🟦️.ts` line 65) emits one pass per component Cargo workspace. The planning graph uses `workspaceRoot = repoRoot` (driver `cwd`), so every in-repo crate is `workspace` provenance and is admitted.

Broken release call (verified):
- Root `📜️script.ts` line 14866, class `ReleaseScript`, calls `runNativeBuildDriverV1({ plan, compilerConfiguration, cwd, cacheRoot, shared, generation, leaseDirectory, environment, control, stages, jobEnvironment })`.
- It omits `storage` and `sourceBoundary`. The options type (`🧩️component-pass/🟦️.ts` lines 227 to 229) requires both.
- The driver's first call, `requireNativeStorageReservationV1(options.storage, ...)`, throws on a missing reservation (`💾️storage/🟦️.ts` line 64: `if(!reservation||...) throw`).
- Git state: the driver file and root script are staged as `M ` (index modified). HEAD's driver had no storage or boundary contract (0 matches), so the mismatch comes from the staged driver change, not from HEAD's script.
- Effect: `bun repo run workspace:release` (root `project.json` target `release` → `bun ./📜️script.ts release`) should fail before any compile. Not executed.

Monolith test (`semio-framework-plugin`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust`):
- Its library is a 1,891-byte `🦀️.rs` that mounts `🔌️plugin/🦀️.rs` through `#[path="../../🦀️.rs"] pub mod component;`.
- Whole-file measure of the mount tree: 612 `.rs` files, 182,698 lines (tests included). Largest non-test files: `🧵️instance/🦀️.rs` 10,884 lines, `🖥️host/🦀️.rs` 7,295 lines, `🌐host/…` (host mount at line 380).
- Ceiling is 32 files, 4,000 active lines, 262,144 bytes. Even cfg-selected, the closure exceeds each limit by an order of magnitude. A file-level violation would come from `instance/🦀️.rs` alone if it is fully selected.
- Peer evidence: `crate-production-boundaries.md` says the plugin closure "refuses a generated local-macro reactor input requiring actual compiler ownership", which triggers earlier at `📥️source/🟦️.ts` line 24.
- Conclusion: the release build refuses semio-framework-plugin (and any plugin component workspace that pulls it in). The `rust-source-boundary` block in its `Cargo.toml` has no effect.

### 2d. Gaps and caveats

| # | Gap | Impact |
| --- | --- | --- |
| G1 | Units whose manifest lies outside `workspaceRoot` are `external-path` and skipped. The driver uses repo root (full coverage). `buildCargoArtifacts`, relay and exact-Cargo use caller `policy.cwd`, so path deps outside that crate's workspace escape admission | Non-driver builds (renderer wasm, exact laws, relay) can miss oversized sibling crates |
| G2 | Per-unit boundary can only tighten (`Math.min`) | Crate metadata cannot raise the ceiling; plugin's 12,000 is inert |
| G3 | Line count is cfg-selected physical lines including blanks and comments | Ceiling is slightly stricter than "logical" lines |
| G4 | `🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` is a 561-byte re-export shim of the real driver | Callers import the shim; a second copy would need its own check |
| G5 | Peer gate checks only the entry file of boundary rows, not the whole selected closure (`architecture-audit-current.md`) | Mounted children can add forbidden edges unseen |
| G6 | Root `release` script omits `storage`/`sourceBoundary` (2c) | Release path does not run at all |

---

## 3. Fleet B Bazel lane: interfaces our lanes must respect

### 3a. Developer interface (use exactly these)

| Form | Meaning |
| --- | --- |
| `bun repo run <project>:<task>[:<configuration>]` | Canonical run (Nx default) |
| `bun repo --tool=bazel run <project>:<task>` | Explicit Bazel backend |
| `bun repo tools setup all` | Provision pinned Bazel 8.4.2 and Nx 23.2.0 (no graph discovery) |
| `bun repo graph`, `bun repo show projects` | Inspection |
| Env `SEMIO_BUILD_TOOL` | Inherited through nested orchestration |
| Env `SEMIO_BUILD_MODE` | prototype or release |
| Env `SEMIO_WORKSPACE_ROOT` | Published by bootstrap |
| Env `SEMIO_BUILD_TERMINAL`, `SEMIO_BUILD_DYNAMIC_OUTPUT`, `SEMIO_BUILD_INVOCATION_PID` | Shared task environment (replaces `NX_*` identity) |

Argument rule: leaf arguments use the canonical `--` boundary. `--skip-nx-cache` is rejected by neutral selections.

### 3b. Where the neutral interface lives

| Item | Path |
| --- | --- |
| Neutral CLI | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` (root `package.json` line 109: `"repo": "bun ./…/🏗️build/📜️script.ts repo"`) |
| Project schema | `…/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json` (executors: `repo:script`, `repo:noop`) |
| Nx private adapter (moved) | `…/📚️library/🔌️nx-plugin/` → `…/📚️library/🏗️build/` (graph inference at `🏗️build/🕸️graph/📥️inference/`) |
| Publication adapter (moved) | `…/📇️registry/📣️publication/🔌️nx/` → `…/📇️registry/📣️publication/🏗️build/` |
| Artifact marker | `.nx-artifact.json` → `.semio-artifact.json` (about 128 old paths removed) |
| Bazel root files | `MODULE.bazel` (42 bytes), `.bazelrc` (117 bytes), `.bazelversion` |

### 3c. Declarations our lanes must follow

- Root `📋️project.json`: 306 targets, each `executor: "repo:script"`, with `options.command` of the form `bun ./🧰️framework/.../📜️script.ts <cmd>`. Root target `release` runs `bun ./📜️script.ts release`; `components` runs `bun ./📜️script.ts components`.
- Play project `🏢️semio-tech/🎡️play/📋️project.json`: name `@semio-tech/semio-tech-play`, 13 targets (`prepare-prototype`, `activate-prototype`, `dev`, `serve`, `build`, `test`, `prepare-e2e`, `serve-e2e`, `test-release-routing`, `test-release`, `test-e2e-enduser`, `test-e2e`, `release-verify`). `dependsOn` uses `<project>:<target>`. Its `$schema` points at the neutral project schema.
- Play `package.json` scripts: `"build": "bun repo run @semio-tech/semio-tech-play:build"` (same pattern for test and e2e).
- Release pipeline metadata lives in root `📋️project.json` `targets.release.metadata.semio.release` (prerequisite `workspace:deps-wasm-opt`, jobs `describe-tool`, `font-assets`, `fonts`, `stageAfter`).
- Package manifests: 249 authored manifests express identity via the neutral project authority. Nx package metadata is absent. This conflicts with AGENTS.md ("`package.json` MUST call nx"). Follow the neutral CLI and flag the conflict to the user.

### 3d. Launch registry

- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`: 0 `bun nx` lines. 448 commands were rewritten to `bun repo run ...`.
- Release entries: `📦️build🦑️workspace🚀️release` (`bun repo run workspace:release`, group `4_build`, order 11.011) and `…🆕️fresh` with `SEMIO_RELEASE_GENERATION=fresh`.
- Register new commands in both files, following the existing group and order.

### 3e. Bazel limits

- Bazel 8.4.2 tasks restore cached product snapshots; it has no Cargo unit model.
- Open configured-input gap (`configured-inputs.md`): a configured command or cwd can select a script whose source is absent from inferred inputs. Our configurations must not depend on it until fixed.
- Native Windows and Linux were not run by fleet B.

---

## 4. Areas not touched by peers (evidence and verdict)

Recent-edit counts use mtime under 3 hours, excluding `node_modules`, `🗑️generated`, `target`. Peer mentions are substring hits in both ticket report sets.

| Area | Recent files (3 h, all file types) | Peer mentions (MODULAR / ABSTRACT) | Verdict |
| --- | --- | --- | --- |
| `🧰️framework/.../💻️os/🔨️modules/🔌️plugin` (framework plugin) | 102 | 13 MODULAR / 5 ABSTRACT | NOT FREE. Fleet A edits reactor and tests (`clipboard-original-ingress.md`); manifests changed 16:56 to 16:58 |
| `🧰️framework/.../🏪️store` | 394 | 2 / 1 | NOT FREE. Active kernel Store cut |
| `🧰️framework/.../📺️renderer` | 16 | 9 / 2 | NOT FREE. `semio-framework-os-renderer-wgpu` touched 16:58 |
| `🧰️framework/🔨️modules/🖱️ui` | 13 to 20 across subdirs | 9 / 2 | NOT FREE. Draw and Prepared cut in progress |
| `🧰️framework/🔨️modules/📡️spr/📜️history` | 43 | (OS history cut) | NOT FREE |
| `✏️s/🔌️plugins/🗄️stdio` (Cargo + artifacts) | 12 | 8 / 3 | Mixed. Fleet B deleted and renamed `.nx-artifact.json` markers (about 70 paths). Cargo/generated code not touched in the 3 hours |
| `🌎️hub` | 6 | 8 / 4 | NOT FREE. Trusted-catalog work (`🗿️artifact-authority/🔏️trusted-catalog`, `trusted-catalog-receipts` ticket active). `🌎️hub/🧩️compositions/🗄️stdio` lease-root finding (`build-cohort-audit.md` P1). Fleet B renamed hub admin marker |
| `✏️s/🔌️plugins/🌀️procedural` (generation3d) | 31 | 2 / 1 | Active (not peer scope) |
| `✏️s/🔌️plugins/🏙️bim` | 1,109 `.rs` files, all under `🗿️artifacts/🏢️model/🏅️standards` | 0 / 0 | Mass mtime change in 3 h (likely codemod or checkout). Attribution unknown; avoid until checked |
| `✏️s/🔌️plugins/🔱️trinity`, `🖍️draw`, `🗒️note`, `📖️playbook`, `🌍️gis` | 1 to 3 each | 37 to 44 | Light activity |

Low-churn candidate plugin directories (0 recent `.rs` edits in 3 hours, peer mentions only as generic enumerations in `execution-native.md` and `cargo-target-store-type-diagnostics.md`):
`✒️writer`, `➗️mathematical`, `🀄️wfc`, `🌿️vcs`, `🎞️animate`, `🎥️shooting`, `🎪️demonstrator`, `🎬️sequence`, `🏗️fem`, `🏛️architect`, `💠️lowpoly`, `💡️reasoning`, `📋️forms`, `📏️layout`, `📸️remodel`, `🔋️energy`, `🕸️dag`, `🖨️raster`, `🧩️puzzle`, `🧱️block`, `🪐️space`.

Heavily named in peer reports (avoid without coordination): `📐️cad` (96), `📜️imperative` (110), `🏭️process` (93), `🪵️sourcing` (76), `📕️norm` (38), `🌊️flow` (185 mentions, mostly prose).

---

## 5. Recently modified Rust under `🧰️framework` (last 3 hours, 537 files)

| Module (under `🧰️framework/🛍️products/💻️os/🔨️modules/`) | Files | Peer owner (by report) |
| --- | ---: | --- |
| `📡️spr/📜️history` (incl. decode) | 43 | Fleet A history cut |
| `🏪️store/🧩️composition` | 36 | Fleet A kernel Store cut |
| `🏪️store/🧪️tests` | 32 | Fleet A |
| `🏪️store/🧪️testing` | 20 | Fleet A |
| `🏪️store/📦️codec` | 19 | Fleet A |
| `🏪️store/🧬️schema` | 16 | Fleet A |
| `🖼️assets/🔤️fonts` (framework) | 15 | Fleet A font units |
| `📇️directory/🔌️client` | 13 | Fleet A directory |
| `🏪️store/🧵️canonical-edit` | 13 | Fleet A (store) |
| `🖱️ui/🧪️tests` | 13 | Fleet A UI |
| `🖱️ui/🎯️targets` | 13 | Fleet A (draw/prepared) |
| `🏪️store/🚪️io`, `📜️space-history`, `🧷️assembly`, `🧬️snapshot-clone`, `🔄️sync`, `🔗️read`, `🪆️child`, `📬️command` | 8 to 12 each | Fleet A kernel Store |
| `🎭️actor/🎟️retained-turn` | 12 | Unknown (framework actor) |
| `🔌️plugin/🖥️host` | 11 | Not in peer scope list; active |
| `🔌️plugin/⚛️reactor` | 10 | Active |
| `🔌️plugin/🧠️interpreter` | 6 | Active |
| `🌱️value/🧬️retained-clone` | 8 | Unknown |
| `🗣️dsl/🧬️schema` | 5 | Unknown |
| `🧊️3d/🥽️mesh`, `🧊️3d/📐️brep` | 4, 2 | Unknown |
| `📺️renderer/🧑‍🎨engine` | 5 | Fleet A (renderer wgpu) |
| `🌉️mcp/...` | 2 to 3 | Unknown |
| Others (1 to 3 files each) | ~100 | Mixed |

Most recent Rust sources are mostly Fleet A's `🏪️store`, `📡️spr/📜️history`, `🖱️ui` and `🔌️plugin` trees. Peer ownership is inferred from report text, not from lock files.

---

## 6. Recommendations for our fleet (for the user to confirm)

1. Do not add `nx` to `package.json` or `project.json`. Use `bun repo run <project>:<target>` and the schema-owned `repo:script` executor.
2. Fix or flag the broken root `ReleaseScript` call (2c) before relying on `workspace:release`.
3. Do not create monolithic Cargo crates above 32 files or 4,000 active lines. Plugin components must split or wait for Fleet A's plugin ownership.
4. Take low-churn plugin candidates (section 4) only after confirming with the user. Avoid `🏭️bridge` and `🗿️artifacts` trees that Fleet A enumerates for WASM profiles.
5. Verify G1 (provenance skip) before trusting admission in non-driver builds.

## 7. Limits

- No build, compile, admission or Nx/Bazel run. All "refuses" statements are static.
- Line counts are whole-file, not cfg-selected.
- Attribution of recent edits to a fleet is by peer-report text, not by git authorship.
- Roughly 150 peer reports were keyword-scanned only.
