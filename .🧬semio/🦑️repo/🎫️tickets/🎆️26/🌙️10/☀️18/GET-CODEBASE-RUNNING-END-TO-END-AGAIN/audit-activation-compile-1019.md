# 🧭️ Audit: compile closure of the play and demonstrator activations (2026-10-19)

Auditor `aud-act`, read-only. No builds, no `bun repo run`, no source edits. Temporary extraction scripts and JSON are in `🗑️generated/aud-act-*` (delete with the ticket generated folder).

## 0. Verdict

- Neither target can complete end to end today. Compile evidence covers host `cargo check --lib` only. No batch has ever produced a wasm artifact, and three workspaces that the lanes need do not resolve (`cargo fetch --locked` exits 101).
- Lane closure (path-dependency closure of the 36 hub lane crates plus engine and wasm roots): 845 crates. 43 have a green host lib check at least once. 237 are scheduled but never run. 565 appear in no batch at all.
- Hub lane crates (all 36 roots) are never compiled. Batch 149 (hub lib) fails at manifest resolution.
- Expected cost, if the three resolution blockers are cleared: several hours of serial cargo time (memory: play activation about 3 h cold with wasm rebuilds; demonstrator cold about 41 min). Disk is fine (202 GiB free). RAM is 32 GiB.

## 1. Target trace

| Step | Where | What it does |
|---|---|---|
| play `activate-prototype` | `🏢️semio-tech/🎡️play/📋️project.json:77` (dependsOn L91 `prepare-prototype`, then 35 `@semio-tech/framework-os-dev:activate-<lane>-react-prototype`) | Nx target, `repo:script`, command `bun ./🔨️modules/🧩️runtime/📜️script.ts activate`, env `SEMIO_BUILD_MODE=prototype` |
| play `prepare-prototype` | same file `:19` | 35 `prepare-<lane>-react-prototype` deps, command `… script.ts prepare` (verifies staged modules only) |
| play runtime script | `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📜️script.ts` | `prepare` checks staged plugin modules; `activate` reads `♻️activation` receipt; `serve` runs vite on :6033 |
| demonstrator `activate-prototype` | `♻️mit-bestand/🧺️demonstrator/📋️project.json:155` (dependsOn L169 `prepare-prototype`, L170 `activate-lanes-prototype`) | Only the 3 lanes `energy`, `fem3d`, `generator` are activated (target at L182, 3 deps) |
| demonstrator `prepare-prototype` | same file `:22` | 9 lane prepares (aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen) and 4 wasm producers: `@semio-tech/framework-surface-rs:wasm`, `framework-editor-rs:wasm`, `framework-os-node-graph-rs:wasm`, `puzzle-plugin:wasm` |
| lane target generator | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs` | Generates `activate-<v>-react-<profile>` (L217), `prepare-<v>-react-<profile>` (L225) and the prepare dependsOn list (L229: session, `framework-plugin-web:support`, `semio-framework-os-infinite:fonts`, renderer and engine wasm producers, boot prerequisites). `rendererEngines` at L163. `closurePrerequisites` at L138 adds `@semio-tech/os-plugin-describe-rs:build` and `workspace:deps-wasm-opt` |
| lane command | `playground-targets/🟨️.mjs:223` and `:230` (`bun ./📜️script.ts activate|prepare <variant> react <profile>`, cwd `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript`) | Per-lane prepare/activate. The script itself was not read (see section 7) |
| plugin component pipeline | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts:34` (`PLUGIN_WASM_TARGET = "wasm32-wasip2"`); `🏗️build/🧩️driver-stages/🟦️.ts` | Each component: native cargo build, then `describe`, `transpile` (jco), `optimize` (`wasm-opt -Oz`), `browser` publish, `probe` |
| profile mapping | `🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts:21-23` | `prototype` maps to cargo profile `prototype`; `release` maps to `release`. The profile definition itself was not found in any manifest checked |
| cargo launcher used by the batch runner | `.🧬semio/…/cc-tools/run.mjs:21` | `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- <args>`, waits for any foreign `bun repo cargo` first (`waitForeign`, L20) |

Crate routing: the lane's Cargo manifest declares `[[package.metadata.semio.playground]]` (variant, ports). The hub manifest is the lane crate. All 35 play lanes and the 9 demonstrator lanes map to hub component crates under `🌎️hub/🧩️compositions/…`. Ticket copies under `.🧬semio/…/END-TO-END-OS-HUB-COLLABORATION-MCP/wp-st1|wp-st2/payload/` carry the same variants but are skipped by the graph (`path.includes(".🧬semio")` in the manifest loop of `playgroundPreparationTargets`, playground-targets `🟨️.mjs`), so they are not in the build.

## 2. Lanes and plugins

Counts:

- play: 35 lanes (35 `prepare`, 35 `activate`), 35 distinct hub component crates.
- demonstrator: 9 prepare lanes but only 4 distinct hub crates (`demonstrator` covers 6 variants, `energy`, `fem` and `procedural`). Activated lanes: 3 (`energy`, `fem3d`, `generator`).
- wasm producers named by the targets: play `flow-wasm`, `surface-wasm`, `editor-wasm`, `graph-wasm`, `puzzle-wasm` (build jobs in `📋️project.json:193-` onward; these are `release` profile jobs, not used by `activate-prototype`); demonstrator `surface-rs`, `editor-rs`, `node-graph-rs`, `puzzle-plugin`. Engine catalog `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json` lists surface, editor, graph, puzzle as the renderer engines.
- 36 distinct hub roots in total (play union demonstrator).

### 2a. Play lanes (35) to hub crate and closure verdict

| variant | hub crate | verdict |
|---|---|---|
| demonstrator | `semio-hub-demonstrator-component` | never scheduled |
| imperative | `semio-hub-imperative-component` | never scheduled |
| stdio-png | `semio-hub-stdio-image-component` | never scheduled |
| lowpoly | `semio-hub-lowpoly-component` | never scheduled |
| remodel | `semio-hub-remodel-component` | never scheduled |
| draw | `semio-hub-draw-component` | never scheduled |
| raster | `semio-hub-raster-component` | never scheduled |
| layout | `semio-hub-layout-component` | never scheduled |
| shooting | `semio-hub-shooting-component` | never scheduled |
| block2d | `semio-hub-block-component` | never scheduled |
| wfc2d | `semio-hub-wfc-component` | never scheduled |
| fem2d | `semio-hub-fem-component` | never scheduled |
| energy | `semio-hub-energy-component` | never scheduled |
| architect | `semio-hub-architect-component` | never scheduled |
| din4108 | `semio-hub-norm-component` | never scheduled |
| note | `semio-hub-note-component` | never scheduled |
| writer | `semio-hub-writer-component` | never scheduled |
| forms | `semio-hub-forms-component` | never scheduled |
| mathematical | `semio-hub-mathematical-component` | never scheduled |
| reasoning-wires | `semio-hub-reasoning-component` | never scheduled |
| dag | `semio-hub-dag-component` | never scheduled |
| trinity-jack | `semio-hub-trinity-component` | never scheduled |
| stdio-mp4 | `semio-hub-stdio-media-component` | never scheduled |
| stdio-step | `semio-hub-stdio-cad-component` | never scheduled |
| bim | `semio-hub-bim-component` | never scheduled |
| stdio-ifc-2x3 | `semio-hub-stdio-bim-component` | never scheduled |
| stdio-gltf | `semio-hub-stdio-mesh-component` | never scheduled |
| stdio-pdf-1-4-a | `semio-hub-stdio-pdf-component` | never scheduled |
| stdio-docx | `semio-hub-stdio-office-component` | never scheduled |
| stdio-semio-brep | `semio-hub-stdio-semio-component` | never scheduled |
| stdio-binary | `semio-hub-stdio-binary-component` | never scheduled |
| animate | `semio-hub-animate-component` | never scheduled |
| sequence | `semio-hub-sequence-component` | never scheduled |
| vcs | `semio-hub-vcs-component` | never scheduled |
| home | `semio-hub-space-component` | never scheduled |

The `s` host lane is excluded from play (memory: `stdio` megacrate `semio-s-plugin-stdio` cannot link, wasm-component-ld 1,000,000-function ceiling). `semio-s-plugin-stdio` is not in the computed closure.

### 2b. Demonstrator (9 prepare + 3 activate lanes)

| lane | hub crate |
|---|---|
| aggregator, aussuchen, bearbeiten, generator, koordinator, verfolgen | `semio-hub-demonstrator-component` (six ports, one crate) |
| energy | `semio-hub-energy-component` |
| fem3d | `semio-hub-fem-component` |
| generation3d | `semio-hub-procedural-component` |

Every one of these hub crates is `never scheduled` in the batches.

## 3. Closure and compile evidence

Method: path-dependency closure (`[dependencies]` and `[build-dependencies]`, not dev) of the 36 hub roots plus the extra roots (surface, editor, node-graph, `semio-hub-puzzle`, plugin-describe, os-infinite, canvas, os-renderer-wgpu), starting from `Cargo.toml` manifests. Batch coverage uses `-p` names in the same workspace (`cc-tools/batches.json`, `.done` markers in `🗑️generated/`). Scripts: `🗑️generated/aud-act-*.mjs`.

Limit: the closure does not follow semio metadata `depends-on`, `extends` or `consumes`. Memory says the demonstrator gate is a 28-component runtime closure (8 plugins, 9 flow, 4 process, 4 cad, 3 sourcing), and each play pane has its own `runtimeComponentClosure`. The real activation set is therefore at least this closure, possibly larger.

| verdict | crates | meaning |
|---|---|---|
| GREEN (host lib check, >= 1 batch exit 0) | 43 | last markers 2026-10-18 04:25 to 2026-10-19 01:40 |
| NOT-RUN (in a batch, no marker) | 237 | queued in `batches.json`, never executed |
| UNCOVERED (no batch names it) | 565 | not in any batch |
| FAILED-only | 0 | none fail without a green batch (see 3b) |

### 3a. By workspace

| workspace | closure | GREEN | NOT-RUN | UNCOVERED |
|---|---|---|---|---|
| `🌎️hub/Cargo.toml` (lane roots) | 74 | 0 | 3 | 71 |
| `🧰️framework/Cargo.toml` | 491 | 37 | 230 | 224 |
| `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model` | 229 | 0 | 0 | 229 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` | 28 | 0 | 0 | 28 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack` | 11 | 0 | 1 | 10 |
| other plugin and single crates (12) | 12 | 6 | 3 | 3 |

The 229 bim crates are real path dependencies: `semio-s-artifact-bim-model` pulls about 190 generated `…/🧬️schema/🧬️mutations/…` crates (`mut-any-N`) through `ramp-runs` and `entities`. This is the largest single cost.

Framework GREEN crates (37) are host lib checks from batches 34, 35, 55, 300, 330 and others. Examples: `semio-framework-job` (330), `semio-framework-ui-prepared` (55, 330), `semio-framework-trace` (35). Full list: `🗑️generated/aud-act-cover-full.json`.

### 3b. Failing batches that touch the closure

| batch | result | closure crates in its `-p` list | error (first line) | blocks activation? |
|---|---|---|---|---|
| 036 | exit 1 | 6 (already green in 035) | `failed to select a version for semio-framework-3d-brep-construction` (locked 0.1.0). Framework workspace does not resolve | **yes**: whole framework workspace |
| 070 | exit 1 | 19 | `cargo fetch --locked` exit 101 on root `Cargo.toml`: `failed to get semio-framework-ui-wgpu-layout` (path dep of `os-renderer-wgpu-shell-async`) | **yes**: root workspace |
| 149 | exit 1 | 0 (its crates are not closure crates) | `cargo fetch --locked` exit 101 on `🌎️hub/Cargo.toml`: `failed to get semio-framework-os-mcp-capabilities` (dep of `os-mcp`) | **yes**: hub workspace, all 36 lane roots |
| 068 | exit 101, 5 errors | 16 | test-companion privacy (`--tests`) | no (test-only) |
| 076 | exit 101, 149 errors | 6 | test-companion privacy and carve (`--tests`) | no (test-only) |
| 085 | exit 101, 7 errors | 0 | `ui_wgpu::wgpu` unresolved (`ui/🎯️targets/🧊️wgpu/🦀️.rs` missing during c6ui move) | no (stdio plugin ws lib; not in lane closure) |
| 901 | exit 1, 3 errors | adhoc | `cargo fetch --locked` on framework (ui-wgpu-prepared-laws manifest) | yes (same root cause as 036) |
| 902 | exit 101, 63 errors | adhoc | mcp-bridge test privacy | no |

The `ui` source gate (`semio-framework-ui` refused, result-cc.md) is not in the lane closure (exact crate absent). 81 `semio-framework-ui-*` crates are in it: 10 GREEN, 71 NOT-RUN. `refused.txt` is empty now, so no batch is being skipped by the gate at present. Batches 077-084 show `skipped` markers from 2026-10-18 10:16 to 10:24, which predates the carve.

### 3c. Evidence age

- All 28 `.done` markers are dated 2026-10-18 04:25 to 2026-10-19 01:50, so all are after 2026-10-16. Peer edits continued after many of them (result-cc.md records ongoing ui/mcp churn; batch 149 at 13:15 failed on that churn). Treat GREEN as "was green then", not "green now".
- No `wasm32` build appears in any batch. `batches.json` has only `pl check`, `pl check -p` and `pl update`. The `--target wasm32-*` checks are batches 153-172, none run.

## 4. Disk, memory and duration

| item | value | source |
|---|---|---|
| disk | 202 GiB free of 926 GiB (7% used) | `df -h /` (2026-10-19) |
| RAM | 32 GiB, 10 cores | `sysctl hw.memsize`, `hw.ncpu` |
| swap | 2 GiB total, 424 MiB used | `vm.swapusage` (now). Memory notes cite 45-69 GB under fleet load |
| disk failure history | ENOSPC on a cold play build at 345 MB free (2026-10-11) | memory `feedback-prune-agent-target-dirs-after-each-agent` |
| wasm-dev rustc peak | 8.6 GB RSS with debug on; 165 MB with `CARGO_PROFILE_WASM_DEV_DEBUG=false` | memory `project-wasm-dev-profile-debug-off-and-swap-thrash` |
| demonstrator cold | about 41 min (cached about 11 min) | memory `project-demonstrator-build-and-stale-artifacts` (2026-09-16) |
| play activation | about 3 h with wasm rebuilds (all-lane activation, cold) | memory `project-external-sweep-breaks-nx-tooling` and `feedback-wasm-mutex-fixed-stamp-slots` (both cite about 3 h) |
| root lib batches | about 15-20 min each (063 at 08:00 to 075 at 10:16, 7 batches) | `.done` mtimes |
| build budget | 20 min per cargo spawn (`BUILD_BUDGET_MS`), `SEMIO_BUILD_BUDGET_MS` overrides | memory `project-semio-build-budget-and-oom`. Not re-checked in the current source |

Estimate for a full serial run once the three resolution blockers are fixed: lib checks of about 845 crates, roughly 2 h cold plus wasm builds of about 40 crates, roughly 1 h. Total roughly 3 h cold, under 1 h if the cargo build dir is warm. Disk: expect tens of GB of new intermediates in the shared build dir (not measured). Keep at least 30 GB free.

## 5. Serialized compile plan

Concurrency rule: all cargo steps run one at a time. `run.mjs` waits for any foreign `bun repo cargo` before each launch, and two steps would fight for the same shared build dir and RAM. Non-cargo work (tests, tsc) may run beside a step. Use `--parallelism=4`, leaving about 6 cores free.

Each step is a single `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- …`. Times are estimates.

| step | command (after `bun repo cargo --store=shared --configuration=prototype --parallelism=4 --`) | scope | est. | gate |
|---|---|---|---|---|
| 0a | fix `🧰️framework` `3d-brep-construction` manifest/version (source edit, owner) | framework resolve | edit | `update --workspace --manifest-path 🧰️framework/Cargo.toml` succeeds |
| 0b | restore `semio-framework-ui-wgpu-layout` path dep target (c6ui/ui carve) | root resolve | edit | `update --workspace --manifest-path Cargo.toml` succeeds |
| 0c | restore `semio-framework-os-mcp-capabilities` (ui -> ui-declaration rewrite) | hub resolve | edit | `update --workspace --manifest-path 🌎️hub/Cargo.toml` succeeds |
| 1 | `update --workspace --manifest-path 🌎️hub/Cargo.toml` then `…Cargo.toml` then `…🧰️framework/Cargo.toml` (online, one at a time) | all three | 5 min | none fail |
| 2 | `check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-surface -p semio-framework-editor -p semio-framework-os-node-graph -p semio-framework-plugin-describe -p semio-framework-os-infinite -p semio-framework-canvas -p semio-framework-os-renderer-wgpu --lib --keep-going --message-format=short` | framework engine and fonts roots | 20-40 min | exit 0 |
| 3 | `build --target wasm32-unknown-unknown --manifest-path 🧰️framework/Cargo.toml -p semio-framework-surface -p semio-framework-editor -p semio-framework-os-node-graph --lib` | wasm engines (framework) | 15-30 min | exit 0 (one at a time, budget per cargo spawn) |
| 4 | `check --manifest-path 🌎️hub/Cargo.toml -p <lane roots 1-18> --lib --keep-going --message-format=short` | hub lane roots, half 1 | 30-45 min | exit 0 |
| 5 | same, `-p <lane roots 19-36>` | hub lane roots, half 2 (warm from step 4) | 20-30 min | exit 0 |
| 6 | `build --target wasm32-unknown-unknown --manifest-path 🌎️hub/Cargo.toml -p semio-hub-puzzle --lib` (puzzle) and framework `semio-framework-os-flow-browser-package` (`-p` in framework ws, check its manifest first, see 7) | wasm engines (hub and flow) | 15-25 min | exit 0 |
| 7 | canary: `build --target wasm32-wasip2 --manifest-path 🌎️hub/Cargo.toml -p semio-hub-lowpoly-component --lib` | one plugin wasm component | 10-20 min | exit 0 before the rest |
| 8 | `build --target wasm32-wasip2 --manifest-path 🌎️hub/Cargo.toml -p <remaining 35 lane roots> --lib` in 2 chunks | plugin wasm components, play and demonstrator | 60-90 min total, two steps of under 60 min | exit 0 |
| 9 | final gate (not run here): `bun repo run @semio-tech/semio-tech-play:activate-prototype --configuration=prototype`, then demonstrator `activate-prototype` | activation itself | 11 min cached to 41 min cold (demonstrator); play 3 h cold | out of scope for this audit |

Step-level notes:

- The 36 hub root names for steps 4-8 are in `🗑️generated/aud-act-roots.json` (`hubRoots`, names as printed in section 2a/2b).
- Steps 4-8 `-p` lists are hub-only. Cargo compiles the transitive path closure of each root, so one `-p` per root is enough. The 229 bim crates and 28 stdio semio crates come in through the hub roots.
- Path deps across workspaces may trigger "believes it's in a workspace when it's not" (result-cc item 5). If it happens, run the affected plugin workspace on its own with `check --manifest-path <ws>/Cargo.toml -p <crate> --lib`.
- `--keep-going` is required, so one failing unit does not hide reachable errors (memory `feedback-cargo-keep-going-reachability-masking`).
- Wasm `build` steps are not verified against the real pipeline. The activation runs describe/transpile per component. Confirm steps 7-8 with a single crate first. Never link `semio-s-plugin-stdio` standalone (memory: unlinkable).

## 6. Blockers in order

1. Resolution: framework (36), root (70), hub (149) fail `cargo fetch --locked`. These are manifest and dependency edits by peers, not lock timing. Without them no step can run.
2. Hub lane roots have never been compiled (all 36 `never scheduled`). Batches 149-172 are the hub set; 149 fails.
3. No wasm artifact has ever been built for any lane (no `build --target wasm32-*` batch exists).
4. Runtime closure not computed (metadata `depends-on`, `consumes`, per-pane closure). The real set may be larger than 845.

## 7. Open items and unverified

- The per-lane command `framework-os-dev` `📜️script.ts prepare|activate <variant> react <profile>` was not read. The exact cargo argv for component builds and the `prototype` profile definition were not located. Both need reading before step 7-8 is trusted.
- `BUILD_BUDGET_MS` and `SEMIO_SCRIPT_POLICY` (`maximumElapsedMilliseconds` 14400000 in `run.mjs:19`) were not re-checked in source.
- Disk and RAM footprint of the closure is estimated. Not measured.
- The demonstrator `prepare` dependency check (HANDOVER S3) is stale: `prepare-prototype` now depends on lanes, not `prepare-<variant>-react` without profile. Not verified against the current source.
- Demonstrator: 6 of 9 lanes are prepared but not activated (only energy, fem3d, generator reach `activate-lanes-prototype`). Intended, but the activation gate reads receipts for all of them.
- Debug output: `cc-b401.log` contains `[DEBUG]` lines from the launcher (retirement progress). Remove before the ticket closes.

## 8. Files

- Audit: this file.
- Inputs: `🏢️semio-tech/🎡️play/📋️project.json`, `♻️mit-bestand/🧺️demonstrator/📋️project.json`, `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📋️project.json` (lane targets are generated, not listed), `…/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`, `…/🔌️plugin/🏗️build/📋️plan/🟦️.ts`, `…/🏃️process/🏗️build/🎚️mode/🟦️.ts`.
- Compile evidence: `C/cc-tools/batches.json`, `C/🗑️generated/cc-b*.{done,errs,log}`, `C/result-cc.md`, `C/HANDOVER.md` section 2.6.
- Temporary (delete with the generated folder): `C/🗑️generated/aud-act-*.{mjs,json,txt,md}`.
