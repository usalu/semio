# Launch-File Dependents: Complete Removal Map

Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT` · read-only audit · 2026-10-07 · nothing was built, tested or executed (static reads and one-off `python3`/`bun` parsers only). Every claim below is a static finding; "intent" statements are my reading of the code, not a run result.

**Scope.** All tracked files plus untracked-but-not-ignored files (133,971 listed, 35 of them untracked), minus `node_modules`, `.git`, `temp`, `target`, `dist`, `.cursor/plans` and `.🧬semio/🦑️repo/🎫️tickets`. Method: (1) full-text scan of every listed file for 17 spellings (`launch.json`, `launch.seed`, `.claude/launch`, `launchSeed`, `LAUNCH_OUTPUT_REL_PATH`/`SEED_REL_PATH`, `🚀️launch`, `devLaunchers`/`projectLaunchers`, `serverReadyAction`, `node-terminal`, group ids `3_dev`/`4_gate`…, "launch row/entry/configuration/catalog", `preview_start`, "Run and Debug", `${input:`, `compounds`, fixtures key names `launchName/launchCommand/launchOrder/launchGroup/launchPath/launchCatalogs/launchSeed`), (2) a second scan for editor/agent surfaces (`.vscode`, `.claude`, `.cursor`, `.codex`, `.agents`, `.devcontainer`, `.github`, `.config`, `.storybook`, `.mcp.json`), (3) manual reading of every distinct code shape. False positives that I checked and excluded: PDF `/Launch` actions (`✏️s/…/📖️pdf`, `🗄️stdio` oracles), projectile "launch" in `🐾️pets` and `📓️print`, "ToolRun panel" (`⏯️tool-run`, matches "Run panel"), `chromium.launch` in program-matrix, `gitkraken-launch.sh` (retired), `launch axes` / `launcher` in the dashboard (its own wizard vocabulary), and `preview-starts-nothing` in two `⏯️preview-eval-run.json` fixtures. Scan artefacts: `🗑️generated/launch-dependents/` (`scan.json`, `index.md`, `table-A.md`, `table-C.md`, `extents.json`); helper scripts `launch-dependents-*.py` and `launch-dependents-stats-{1,2,3}.ts` (row statistics of section 1) next to this file.

**Path legend.** `R/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry` · `OS/` = `🧰️framework/🛍️products/💻️os/🔨️modules` · `📚️library/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` · `DASH/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`. `file:N` is a 1-based line; `a–b` a region.

---

## 0. Headline findings

1. **Three data files go:** `.vscode/launch.json` (3,283,742 B, 4,586 rows, 4 compounds, 82 inputs, 356 rows with `serverReadyAction`; every row `node-terminal`), `.vscode/🧩️launch.seed.jsonc` (2,699,338 B; 3,514 row slots = 3,425 literal rows + 89 `@generated:` placeholders; 34 inputs; `devLaunchers` for 47 playground variants; `projectLaunchers` policy) and `.claude/launch.json` (18,767 B; 79 Claude-preview servers, 55 runnable + 24 attach-only). Both `.vscode` files are being rewritten right now by a peer (`MM` in git status, mtime 2026-10-07 19:10).
2. **209 files outside tickets depend on the two `.vscode` files** (index in section 8; groups: 16 generator-module files, 8 registry-wiring files, 2 taxonomy/authority-ledger files, 1 root `📜️script.ts`, 8 dashboard files, 7 `project.json` input lists, 75 tests, 2 other scripts, 58 fixtures/schemas, 12 docs, 17 comment/message files, 3 editor/ignore files). Plus `.claude/launch.json` (only documented consumer: `🎓️teaching/🏛️architecture/🐾️pets/README.md:358`).
3. **The launch files are a SECOND OUTPUT of the plugin-registry generator**, so they are wired into the build critical path: `@semio-tech/plugin-registry:generate` (root `prepare`, every dev serve, wgpu, hub stdio, dashboard `🔌️plugin-registry`) writes `.vscode/launch.json`; `…:check` (inside `verify gate`) byte-compares it. Deleting the files before the generator is edited breaks `prepare`.
4. **54 registry/gate tests in `@semio-tech/repo-lib` + ~21 tests elsewhere assert "a launch row exists"** (name, command, `presentation.group = 4_gate`, `order`, sometimes `env`). They are the AGENTS.md law "register all executable commands in launch.json" turned into tests; every one has fixtures carrying `launchName/launchCommand/launchGroup/launchOrder`.
5. **Hidden hard couplings found beyond the obvious list:** (a) `taxonomy.json` declares the launch files as `generatorContracts["plugin-registry"]` input/output (`inputPatterns`/`outputRoots`) and validates them in `check`; (b) the nested-cargo-package projection asset pins occurrence counts in both launch files and is sha-sealed (`taxonomy.json:15961`, `🧫️frozen-seal-ledger/🔣️.json:77`) so editing it needs a reseal; (c) `registryCatalogInputPaths` treats every `📋️project.json` as a catalog input solely because the launch projection renders each declared target; (d) the root `verify interactivity [apps]` gate fails closed on a missing launch file; (e) the devcontainer `forwardPorts` law is derived from launch rows; (f) several tests encode execution policy that lives ONLY in launch rows (`SEMIO_TEST_LEVEL=quick`, private `CARGO_TARGET_DIR`, `-- quick --no-fail-fast`, ordering).
6. **Dashboard currently reads `.vscode/launch.json` as a command source** (`DASH/🌳️command-tree/🦀️.rs`, uncommitted work of this ticket): 43,913 commands on the real workspace, of which the launch file contributes the hand-registered ones that no Nx target or script derives (see R2).
7. **Only one AGENTS.md line mentions launch.json** (`AGENTS.md:50`); no nested AGENTS.md does.

---

## 1. The launch data (what actually disappears)

| file | size | content | producer |
|---|---|---|---|
| `.vscode/launch.json` | 3,283,742 B | 4,586 `node-terminal` configurations (groups: `4_gate` 3,119 · `3_dev` 635 · `9_gates` 453 · `4_build` 260 · `🧹clean🛡️gates` 29 · none 41 · others 49), 4 compounds, 82 `inputs` (48 `projectTarget.*` pick-lists) | generated by `generateLaunchJson` from seed + playground catalog + `declaredProjectTargets` |
| `.vscode/🧩️launch.seed.jsonc` | 2,699,338 B | same skeleton, 3,425 literal rows, 89 `"@generated:<variant>:<renderer>"` placeholders, `devLaunchers` (47 variants), `projectLaunchers` policy (classes, emojis, orders) | hand + agents (AGENTS.md law); `reconcile-launch-seed` moves hand rows from launch.json into it |
| `.claude/launch.json` | 18,767 B | 79 Claude Code preview configs (`runtimeExecutable`/`runtimeArgs`/`port`, 24 attach-only `url`) | hand-maintained, **no generator, no gate** (9 rows point at ticket scripts) |

Classification of the 4,586 generated rows (static parse): 1,342 plain `bun nx run P:T` without env; 976 `bun nx run P:T -- args`; 335 `workspace:dev -- <variant>` playground rows; 114 plain target with env; 48 project pickers; 1,066 + 660 `bun nx exec --projects=workspace … bun test <file>` / `NX_DAEMON=false …` forms; 22 + 18 direct script rows; 1 each `gemini`, `kiro-cli`, `f3d`, `gitkraken`, `bun ./📜️script.ts dev mcp`. **2,546 rows set `env`** (top keys: `NX_DAEMON` 1,845 · `NX_ISOLATE_PLUGINS` 1,055 · `NX_WORKSPACE_ROOT_PATH` 867 · `SEMIO_TEST_LEVEL` 506 · `SEMIO_TEST_ARTIFACT_DIR` 359 · `SEMIO_RENDERER` 333 · `S_OS_PORT` 322), **1,038 use a `cwd` other than `${workspaceFolder}`**, **155 use `${input:…}`**, **1,785 reference a path under `.🧬semio/🦑️repo/🎫️tickets`**, 356 carry `serverReadyAction`.

---

## 2. Generation, check and verb map (who writes / verifies launch data)

| Nx target / script verb | implementation | launch role | reached from |
|---|---|---|---|
| `@semio-tech/plugin-registry:generate` (`R/📋️project.json:13–27`; verb `bun ./📜️script.ts generate`, default command `R/📜️script.ts:97`) | `GenerateScript` `R/📽️projection/🟦️.ts:351–376` | **writes** `.vscode/launch.json` (`:367–373`; written last so a seed problem cannot leave the catalog itself unwritten) | root `prepare` (`📋️project.json:1920`); `OS/📺️renderer/…/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json:467,500,818`; `OS/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:400`; `R/🔁️rebuild/🔣️.json:8`; `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1232,1253`; `DASH/🔌️plugin-registry/🦀️.rs:23`; registry `package.json:13` |
| `…:check` (`R/📋️project.json:61–78`; verb `check`) | `CheckScript` `R/📽️projection/🟦️.ts:475–548` | byte-compares launch.json (`:488–498`), reports render failures as violations (`:490–497,504`), prints "…launch is fresh" (`:548`); also runs `validateGeneratorContractsAgainstWorkspace` (`:478`) which reads the taxonomy launch entries | root `verify gate` (`📜️script.ts:8377`, target `verify-gate`); `R/🔁️rebuild/🔣️.json:9`; registry `package.json:14` |
| `…:check-generated` (`:36–42`) | `CheckGeneratedScript` `:436–451` | compares launch bytes (`:439,445–446,448`) | taxonomy `checkTarget` (`taxonomy.json:26967`); hub stdio script `:1239`; dev `📋️project.json:423` |
| `…:preview-generated` (`:28–35`) | `PreviewGeneratedScript` `:379–434` | emits a `.vscode/launch.json` node + stale-removal list (`:419,425`) for the generator-ownership protocol | taxonomy `previewTarget` (`taxonomy.json:26966`); `📚️library/⚡️caching/🧪️tests/🧬️generator-ownership` |
| `…:reconcile-launch-seed` (`:274–290`), `…:check-launch-seed` (`:264–273`) | `ReconcileLaunchSeedScript` `:453–472` (verb `reconcile-launch-seed`, `R/📜️script.ts:77`) | reads launch.json, rewrites the seed | no automated caller (manual; rows `🛠️reconcile-launch-seed📇️registry` exist in launch.json itself) |
| `…:test-launch-name-contract` (`:190–199`), `…:test-launch-placement-contract` (`:240–251`), `…:test-launch-seed-reconcile` (`:252–263`) | `R/📜️script.ts:13–18, 20–28, 30–37` (+ router `:91–93`) | generator unit tests | none wired from root; `registry:test` additionally globs `**/🧪️tests/**/🟦️.ts` (`R/🧪️tests/🎚️config/🟦️.ts`) so it also runs `R/🚀️launch/🧪️tests/*` and `R/🧪️tests/🚀️launch/🟦️.ts` |
| root `verify interactivity [apps [--actions] \| tool-jobs]` (root targets `verify`, `verify-interactivity` `📋️project.json`; dispatcher `📜️script.ts:7273–7318`) | `runInteractivityAudit` `📜️script.ts:8075–8116`, `runInteractivityApps` `:8119–8140` | **reads** both launch files (`interactivityAllAppDiscovery` `:9007–9057`) | `bun nx run workspace:verify -- interactivity …`; the gate's own required rows |
| root `workspace:verify` (`📋️project.json:1393–1402`, `bun ./📜️script.ts verify`, `forwardAllArgs`), `verify-gate` (`:1403–1411`, `verify gate`), `verify-interactivity` (`:1470–1476`), `prepare` (`:1911–1925`, `dependsOn` `plugin-registry:generate` at `:1920`) | `VerifyScript` `📜️script.ts:7209`; bare `verify` and `verify gate` both end in `runGate` (`:7839`, body `:8367`), whose `plugin-registry:check` call (`:8377`) byte-compares launch.json; `verify interactivity …` per the row above | transitive: `check` byte-compare, launch reads | `bun nx run workspace:verify -- …`; the 7 required gate rows |
| `@semio-tech/repo-lib:test-<name>` (54 targets, table A) | script verb `bun ./📜️script.ts test <name>` (`📚️library/📦️packages/🟦️typescript/📜️script.ts`) | each asserts one registration | `package.json` script `test-<name>` (89 scripts there), launch row itself |

`package.json` scripts that reach these verbs: registry `package.json:13` (`generate`), `:14` (`check`); repo-lib `package.json` `test-*`; root `package.json` has none (its `dashboard*` scripts `:33–36,162–163` are the replacement entry points).

---

## 3. Dependents and the change each needs

### 3.1 Generator module — delete as a whole (16 files)

`R/🚀️launch/🟦️.ts` (629 l.: `readSeed`, `appendLaunchEntries`, `renderEntry`, `generateLaunchJson` `:350–430`, `launchContainerRanges` `:446`, `reconcileLaunchSeed` `:602–627`, `declaredProjectTargets`, `playgroundDevCommand` `:64`, `playgroundDevEnv` `:71`), `R/🚀️launch/🏷️name-prefix/🟦️.ts` (183), `…/🏷️name-prefix/🧪️tests/🟦️.ts`, `…/🏷️name-prefix/🧬️schema/🟦️.ts`, `R/🚀️launch/🧱️placement/🟦️.ts`, `R/🚀️launch/🧬️schema/🧱️placement/🔣️.json`, `R/🚀️launch/🧪️tests/🏭️generate/🟦️.ts`, `…/🧪️seed-reconcile/🟦️.ts`, `…/🧱️placement/🟦️.ts`, `R/🚀️launch/🧫️fixtures/🏭️generate/🔣️.json`, `…/🧫️seed-reconcile/🔣️.json` (608 l.), `…/🧱️placement/🔣️.json`, `R/🧪️tests/🚀️launch/🟦️.ts` (274 l.: name-prefix laws `:10–60`, `playgroundDevCommand` parity `:62–95`, rendered-launch laws `:183–273`, "byte-identical to the committed launch.json" `:266–273`), `R/🧫️fixtures/🚀️launch/🔣️.json`, `R/🧫️fixtures/🚀️launch/🏷️name-prefix/🔣️.json`, `R/🧬️schema/🚀️launch/🔣️.json`.

Intents lost with them (decide whether to re-home): (i) the `playgroundDevEnv` env shape (`S_OS_PORT`, `SEMIO_PLUGIN`, `SEMIO_RENDERER`, `SEMIO_APP`) — the dashboard already owns a Rust twin (`crate::env_contract::build_dev_env`, `DASH/🌳️command-tree/🦀️.rs:357`), so only parity coverage is at stake; (ii) "every declared project target is runnable by a dev" (`R/🧪️tests/🚀️launch/🟦️.ts:227–262`) → re-express as "every Nx project target is a leaf of the dashboard command tree" (compare `nx show projects`/project.json targets with `command-tree --dump-tree`, row `🛠️dev🎛️dashboard🌳️command-tree`); (iii) "launch names are unique/injective per playground variant" — dies with names.

**Name-prefix chain (launch-only vocabulary):** Cargo key `launch-name-prefix` in `🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:55,61,70,79,88,97,106` (7 playground blocks); parsed in `R/🎮️playground/🔎️discovery/🟦️.ts:5 (import), 39 (PlaygroundEntry.launchNamePrefix), 74, 89`; re-emitted by `R/📽️projection/🟦️.ts:128,135,148` into the generated, git-ignored catalog (`🚀️playgrounds.json`, `🎮️playgrounds/🟦️.ts`). No Rust/Go code consumes `launchNamePrefix` (checked). Delete all of it together.

### 3.2 Registry wiring and authority ledgers

| file | lines | what | change |
|---|---|---|---|
| `R/📽️projection/🟦️.ts` | `:8` import; `:128,135,148` `launchNamePrefix`; `:295–296` doc; `:300,306,313,314,354,416,439,463` `launchPlaygrounds`; `:367–373` write; `:419,425` preview node; `:439,445–446,448` check-generated; `:453–472` `ReconcileLaunchSeedScript`; `:488–498,504,548` `CheckScript` | generate/preview/check/reconcile of launch.json | delete the launch statements; delete class `:453–472`; see decision D4 for `launchPlaygrounds` |
| `R/📜️script.ts` | `:13–18, 20–28, 30–37` classes; `:77, 91, 92, 93` registrations | launch tests + reconcile verb | delete regions |
| `R/📋️project.json` | `:190–199, 240–251, 252–263, 264–273, 274–290` targets | launch tests + reconcile/check-seed | delete targets |
| `R/🛂️descriptor-verification/🟦️.ts` | `:8–12` docstring | says check also compares launch.json | reword |
| `R/🎮️playground/🔎️discovery/🟦️.ts` | `:5,39,74,89,210` | `declaredLaunchNamePrefix`; `generateWithheldPlaygroundRegistry` doc "launch rows stay stable" | delete prefix plumbing; reword/decide withheld rows (D4) |
| `R/🔎️discovery/🧪️tests/🟦️.ts` `:108–120`, `🚀️source-examples.feature:6`, `🎮️session-catalog.feature:3` | "launch catalog" wording | this "launch catalog" is the *playground catalog*, not launch.json | rename wording only |
| `📚️library/🔣️taxonomy.json` | `:9213–9222` owner kind `os-registry-launch` (member `🚀️launch`); `:14758–14766` `members-of-os-registry-launch` (member `🏷️name-prefix`); `:26969` `inputPatterns` entry `.vscode/🧩️launch.seed.jsonc`; `:27015–27018` `outputRoots` entry `.vscode/launch.json` (`tracked`) | generator-ownership contract of `plugin-registry` (validated at `R/📽️projection/🟦️.ts:478` and `📚️library/🔍️discovery/🟦️.ts:1334, 6174–6217`) | delete the 4 entries; the workspace Nx plugin `📚️library/🟨️.mjs:640,1322,1522` reads `generatorContracts`, and `📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:942–961` requires each generator target's Nx `inputs`/`outputs` to equal the contract; the registry `project.json` declares neither itself, so deleting the two entries updates the target (no hand edit of `generate`), and every cached `generate`/`check` run is invalidated once |
| `📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json` | `:726–742` two `referenceConsumers` (launch.json `occurrenceCount 65`, generated, `generatorOwnerPaths` incl. the stale `📇️registry/🖥️launch.ts`; seed `occurrenceCount 42`, authored) | package-rename reference authority pins occurrence counts in both files | delete the two objects **and reseal**: new sha in `taxonomy.json:15961`, new `reseals[]` entry in `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json` (current seal at `:77`, schema `📚️library/🧬️schema/🔣️frozen-seal-ledger/🔣️.json`) — see R3 |
| `📚️library/🔍️discovery/🟦️.ts` | `:10106–10109` docstring ("every Nx `📋️project.json` the scan meets is a content input, because the registry's launch projection renders each declared target"); `:11554, 11566, 11585` `catalogExists(…"📋️project.json")` witnesses | catalog input set is inflated by the launch projection | after removal re-check whether these witnesses are still needed; shrinking them changes the generator-inputs receipt (perf win) |
| `R/🔁️rebuild/🔣️.json:8–9` | `generate`/`check` stages | not launch-specific (they call the targets above) | none |
| `OS/🧑‍💻dev/🔌️vite-plugins/🟦️.ts:999–1003`, `OS/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json:3–4,11,26` | `.vscode` in `UNWATCHED_REPOSITORY_SEGMENTS`; `:11` sample path `.vscode/launch.json` | watcher policy vector classifies `.vscode/launch.json` as unwatched | keep `.vscode` segment while `.vscode/settings.json` etc. exist; replace sample at `:11` and reword comment `:999–1001` |
| `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts:78`, `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts:69` | `ignored: [..., "**/.vscode/launch.json"]` | watcher ignore | delete the glob (redundant after removal) |

### 3.3 Root `📜️script.ts` (25,798 lines; only the interactivity all-app discovery touches launch data)

| lines | symbol | what it does with launch data | callers | verbs / targets | change |
|---|---|---|---|---|---|
| `137–139, 144` | comment in `dev … served` doc | "stays reachable from `launch.json` … carries no env field" / "react launch row" | — | `dev s served` | reword (keep the behavioural statement, drop launch.json) |
| `6705–6706` | comment on `POLICY_BREACH_GATES` | cites launch.json `⚖️gate…` entries | — | `verify policy-breach <rule>` | reword |
| `8699` | `INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY = 512` | fixed bound for launch-row parsing (**already exceeded by reality: 4,586 rows**) | `:8964–8965`; export `:25742`; test `🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery:48–49` | — | delete |
| `8707–8714` | `INTERACTIVITY_ALL_APP_BROWSER_DEV_COMMAND`, `…_NATIVE_DEV_COMMAND` | restate the dev command shapes the launch rows must carry (46 `…🧊️wgpu🖥️native` rows) | `:8932–8933` | — | delete (dashboard `inject_playground_dev` is the new author: `DASH/🌳️command-tree/🦀️.rs:352–370`) |
| `8715–8716` | `INTERACTIVITY_ALL_APP_LAUNCH_FILE`, `…_LAUNCH_SEED_FILE` | the two paths | `:8960–8987,9031–9035,8923` | — | delete |
| `8717` | `INTERACTIVITY_ALL_APP_PLAYGROUND_FILE` | generated playground catalog `R/🤖️generated/🎮️playgrounds/🟦️.ts` | `:8876–8893,9036` | — | **keep** (not launch data) |
| `8718–8739` | `INTERACTIVITY_ALL_APP_REQUIRED_GATES` (7 rows; comment says "six") | requires exactly one `4_gate` row per gate `bun nx run workspace:verify -- <interactivity \| interactivity tool-jobs \| interactivity apps \| interactivity apps --actions \| dependencies \| dependencies literal-external \| composed-child-refs>`, quoting AGENTS.md ("a verification verb that no launch row runs is a verb no dev can run") | `:8972–8975`; export `:25733`; self-test `…/🔬️interactivity-all-app-discovery:8,43` | `verify interactivity apps` | delete; **intent: every verification verb is reachable by a dev**. Re-express as: the dashboard registry exposes a leaf for each of these 7 argument forms. `workspace:verify` is ONE Nx target (`forwardAllArgs`), so target discovery cannot enumerate its sub-verbs — the dashboard needs an explicit verb catalogue (D2) and the gate should assert against it |
| `8743`, `8752–8756` | `InteractivityAllAppLaunch` type; `launchCoveredAppCount`, `launchMissingAppCount`, `launches`, `launchOnlyProducts` fields | report shape | `:8079,8123,9050–9054` | — | delete fields; `surfaceCount` (`:9046`) = apps only |
| `8896–8915` | `interactivityAllAppPlaygroundLaunchNames` | derives `🛠️dev<prefix>⚛️react/🧊️wgpu🌐️wasm/🧊️wgpu🖥️native` names from seed `devLaunchers.namePrefix` | `:9040`; export `:25744`; self-test `:65–67` | — | delete |
| `8917–8951` | `interactivityAllAppLaunchCoverageFailures` | **intent: every owner-qualified app context has a playground variant with React, WGPU-Wasm and WGPU-native dev surfaces** (editor/viewer role-neutral) | `:9038`; export `:25743`; self-test `:58–64` | `verify interactivity apps` | rewrite to read the generated playground catalog (`INTERACTIVITY_ALL_APP_PLAYGROUND_FILE`) and assert variant coverage per descriptor app; the three renderer leaves per variant are guaranteed by `inject_playground_dev` (`react`, `wgpu-wasm`, `wgpu-native`) |
| `8953–8989` | `interactivityAllAppLaunchesFromSource` | parses launch JSONC, duplicate-name check, capacity, required-gate check, collects `🛠️dev` rows | `:9031,9034`; export `:25741`; self-test `:41–49` | `verify interactivity apps` | delete (the "launch-only product surfaces" = `🛠️dev` rows not derived from playgrounds — hub, storybook, quiz — need a new source: dashboard `dev` leaves that are not playground leaves, D2) |
| `9006–9057` | `interactivityAllAppDiscovery` | lines `9031–9041,9046,9050–9055` are launch-specific; the descriptor/app/action parts stay | `:8077,8120` | `verify interactivity`, `verify interactivity apps` | delete the launch lines |
| `8075–8116`, `8119–8140` | `runInteractivityAudit`, `runInteractivityApps` | messages/counters using the launch fields (`:8079,8114,8123,8125–8126`) | verb dispatcher `:7273–7318` | `verify interactivity`, `verify interactivity apps` | reword/drop counters |
| `12` | import `interactivityAllAppDiscoverySelfTests` | — | — | — | keep (function stays; its launch laws go) |
| `25733–25744` | `export { … }` | exports 5 launch symbols for the self-test | `🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery:1` | — | delete the 5 names |
| `17404` | `POLICY_MCP_CONFIG_PATHS` includes `.vscode/mcp.json` | MCP config policy, **not launch** | — | `verify policy …` | keep |

Failure mode today: `policyReadFileSafe` returns empty text for a missing launch file ⇒ `"invalid JSONC"`/`"configurations is missing"` failures ⇒ `verify interactivity apps` throws (`:8126`); bare `verify interactivity` is WARN-mode only while `INTERACTIVITY_AUDIT_SEVERITY==="warn"` (`:8109`) but the apps ledger lines still print failures. Everything else in this 25.8k-line file is launch-file-free (precise regex over the whole file; the remaining `launch` words are `lifecycle.launchGitKraken` `:236`, "launching" log text `:312`, `.mcp.json launches` `:479` and the MCP config path list `:17404`).

Related verbs in the same dispatcher that do **not** touch launch data but are named in required-gate rows: `verify dependencies`, `verify dependencies literal-external`, `verify composed-child-refs`.

### 3.4 Dashboard (the only runtime reader) — `DASH/`

| file | lines | what | change |
|---|---|---|---|
| `🌳️command-tree/🦀️.rs` | `1` doc; `25` `CommandLeaf::Compound`; `48` `LAUNCH_CONFIGURATIONS = ".vscode/launch.json"`; `62, 72` call sites in `discover_with_manifests`/`seed`; `224–256` `collect_launch_configurations` (files rows as `<verb>/launch/<name>`); `257–276` `resolve_launch_variables` (`${workspaceFolder}`, `${env:}`, defaulted `${input:}`); `278–287` `shell_spec` (only user: the launch collector); `289–309` `strip_jsonc` (only user: same); `885` compound JSON projection | reads the launch file; **only source of compounds and of every hand-registered command** | rewrite to a dashboard-native registry (D1/D2); delete `LAUNCH_CONFIGURATIONS`, `collect_launch_configurations`, `resolve_launch_variables`, `strip_jsonc`; keep `shell_spec` only if the native registry stores shell commands; keep `Compound` only if native compounds are adopted |
| `🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs` | `199–234` test `launch_configurations_targets_and_scripts_are_filed_under_their_verb`; `236–238` `launch_documents_tolerate_comments_and_trailing_commas`; imports of `LAUNCH_CONFIGURATIONS`, `strip_jsonc`; `:22` reads `🚀️playgrounds.json` (keep) | tests of the reader | rewrite against the native registry fixture |
| `🧫️fixtures/🚀️launch-configurations/🔣️.json` (5,092 B), `🥒️.feature` (2,112 B) | whole | the frozen vector: JSONC with comments/trailing commas, `${input:}` defaults, compounds, omitted rows, verb/search expectations | replace by a native-registry vector (keep the verb/search/omitted expectations; they pin dashboard behaviour, not launch.json) |
| `🧪️tests/🌀️control-plane/🟦️.ts` | `56–82` independent `jsonc-parser` projection of the fixture; `72` compounds loop | third-party oracle for the reader (AGENTS.md rule) | rewrite oracle to the new registry format |
| `🌀️daemon/🧪️tests/🔬️unit/🦀️.rs` | `38–73` test `actual_cli_launcher_receives_keys_and_runs_a_launch_configuration` writes a fixture `.vscode/launch.json` and drives the real TUI (`#[ignore]`, needs `SEMIO_TEST_CLI`) | native PTY smoke | write the fixture in the new registry format |
| `📚️inventory/🦀️.rs` | `17` `SNAPSHOT_VERSION = 2`; `124` test string `"test / launch / x"` | snapshot cache shape includes `launch` segment | bump `SNAPSHOT_VERSION` (tree shape changes); update label |
| `🧬️schema/🔣️.json` | `15` ("workspace scripts or launch configurations"), `22,24–33` `CompoundLeaf` | schema | reword; drop `CompoundLeaf` only if compounds go |
| `README.md` | `9` ("or the Dashboard launch entry"), `21–31, 34–35` launcher paragraph, `103` ("No editor launch configuration is required"), `121, 132` | docs | rewrite |
| `🔮️oracles/🔣️.json:11` | wording "no launcher" | harmless | none |

Name collisions that are NOT launch.json and must survive: `DASH/🧫️fixtures/🗣️launch-axes/*`, `DASH/🧫️fixtures/🔎️launcher/*`, `⚙️preferences/🦀️.rs:10,67`, `🖥️terminal/🦀️.rs` launcher windows, `🛝️playground-session`, and the generated `R/🤖️generated/🚀️playgrounds.json` (used by `DASH/📦️packages/🦀️rust/🦀️.rs:176,181`, `DASH/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs:22`, `DASH/🧪️tests/🔬️unit/🦀️.rs:78`, `📚️library/🎮️playground/🟦️.ts:17,23`).

### 3.5 Nx `inputs` lists that name the launch files (27 lines in 7 project.json files)

Nx tolerates a missing input file, but the tests of section 3.6/3.7 pin these exact lists, so list and test change together.

| file | lines | named input / target | target runs |
|---|---|---|---|
| `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/📋️project.json` | `40–41` | target `test-oracle-source` | `@semio-tech/norm-js:test-oracle-source` (`bun ./📜️script.ts oracle-source`) |
| `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/📋️project.json` | `32–33` (`stdioArtifactContract`), `65–66` (`stdioArtifactGraph`) | named inputs | stdio artifact contract/graph tests |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `39–40` (`hubSocketGrantCommandSources`), `68` (`hubFoundationSources`, brace glob `.vscode/{🧩️launch.seed.jsonc,launch.json}`) | named inputs | `os-hub:socket-grant-command-source-check`, `os-hub:foundation-source-check` |
| `OS/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json` | `85–86` | target `test-browser-ownership` | `semio-framework-os-flow-core:test-browser-ownership` |
| `📚️library/⚡️caching/📋️project.json` | `243–244` | `cacheCommandSources` | `repo:test-cache-command-source` |
| `📚️library/📦️packages/🟦️typescript/📋️project.json` | `16–17` (`workspacePublicationSources`), `21` (also lists `R/🧪️tests/🚀️launch/🟦️.ts` — a file that will not exist), `55–56` (`wasmPackageWrapperSources`), `60–61` (`cargoTransactionCommandSources`), `575–576` (`test-vitest-configuration-ownership`), `604–605` (`test-tool-configuration-ownership`), `1151` (`test-os-dev-composition-ownership`, launch.json only) | repo-lib targets | `@semio-tech/repo-lib:test-*` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json` | `399–400` | `commandCompositionSources` | `@semio-tech/repo-test-domain:test-command-composition-source` |

Mirrors of these lists inside fixtures (must change in the same step): `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json:379`, `…/🧱️socket-grant-command-source/🔣️.json:245–246`, `OS/🌊️flow/…/🏷️ownership/🧫️fixtures/🔣️.json:372–373`, `✏️s/🧑‍💻dev/🗄️stdio/🧫️fixtures/🏃️command-ownership/🔣️.json:180–181,213–214`, `📚️library/⚡️caching/🧫️fixtures/🧱️command-source`, `📚️library/🧫️fixtures/🧱️wasm-package-wrappers/🔣️.json:103–104`, `📚️library/🧫️fixtures/🧱️workspace-publication-source/🔣️.json:134–135,139`, `🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json`.

### 3.6 Registry/gate tests in `@semio-tech/repo-lib` — table A (54 files)

Common shape: `for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) { parse JSONC; expect exactly one row with the registration's name/command[/presentation {group:"4_gate", order}][/env] }`. Each such test also asserts the other three registration legs (Nx target in `📚️library/📦️packages/🟦️typescript/📋️project.json`, `package.json` script, router verb) — those stay. **Underlying intent: "this gate is a registered, reachable developer command".** Change: delete the launch block (the bold range) plus the fixture keys of the same registration (table C); optionally replace all 54 per-test assertions by ONE shared law — "every `test-*`/`lint-*` target of `@semio-tech/repo-lib` is a leaf of the dashboard command tree under `test`/`gate`/`lint`" — evaluated against the project graph (cheap) or `command-tree --dump-tree` (needs the built binary). Ordering (`launchOrder` 4xx.xxx) is a VS Code Run-panel concept; the dashboard sorts alphabetically, so those numbers are dead data.

| # | test file (under `📚️library/`) | launch-reading block(s) | test title (line) | Nx target / script verb | note |
|---|---|---|---|---|---|
| 1 | `↪️rust-divergence-callback/🟦️.ts` | **110, 149-153** | l.108 closed divergence callback contract preserves candidate-only and physical-proof separation | `@semio-tech/repo-lib:test-rust-divergence-callback-source`; -native; -syn → `bun ./📜️script.ts test rust-divergence-callback` | three rows (source/native/syn) via contract.launchPaths |
| 2 | `♻️taxonomy-pattern-compiler-reuse/🟦️.ts` | **320-326**; also 298 | l.294 registers pattern compiler reuse through its closed canonical route | `@semio-tech/repo-lib:test-taxonomy-pattern-compiler-reuse` → `bun ./📜️script.ts test taxonomy-pattern-compiler-reuse` | exact row for registration.launchName |
| 3 | `⚙️root-script-compiler/🟦️.ts` | **120-125** | l.116 registers the root compiler gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-root-script-compiler` → `bun ./📜️script.ts test root-script-compiler` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 4 | `✍️rust-writable-path-authority/🟦️.ts` | **56-65** | l.44 exact writable route, package, and both launch registrations preserve the canonical semant | `@semio-tech/repo-lib:test-rust-writable-path-authority` → `bun ./📜️script.ts test rust-writable-path-authority` | row via registration.launchPaths |
| 5 | `🌐️registry-import-language/🟦️.ts` | **173-178**; also 12-17 | l.166 registers the language-neutral compiler gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-registry-import-language` → `bun ./📜️script.ts test registry-import-language` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 6 | `🌳️kind-only-basename/🟦️.ts` | **130-133** | l.124 derives every focused implementation command from the launch seed authority | `@semio-tech/repo-lib:test-kind-only-basename` → `bun ./📜️script.ts test kind-only-basename` | also pins rows for workspace:verify-taxonomy-implementation-{report,enforce} |
| 7 | `🍃️artifact-support-leaf-authority/🟦️.ts` | **231-237**; also 19 | l.227 registers the canonical test through Nx and the launch catalog | (fixture execution.target) → `bun ./📜️script.ts test artifact-support-leaf-authority` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 8 | `🎚️tool-configuration-ownership/🟦️.ts` | **229** | l.209 retires fixed-name exemptions and registers one Bun and Nx ownership route | `@semio-tech/repo-lib:test-tool-configuration-ownership` → `bun ./📜️script.ts test tool-configuration-ownership` | seed+launch must both contain the registration command |
| 9 | `🎚️vitest-configuration-ownership/🟦️.ts` | **279-282** | l.273 registers the gate through package, Nx and both launch projections | `@semio-tech/repo-lib:test-vitest-configuration-ownership` → `bun ./📜️script.ts test vitest-configuration-ownership` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 10 | `🎟️reference-coverage-selection/🟦️.ts` | **123-129** | l.106 reference coverage gate is registered through its exact default-budget route | `@semio-tech/repo-lib:test-reference-coverage-selection` → `bun ./📜️script.ts test reference-coverage-selection` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 11 | `🎯️cargo-target-discovery-skip/🟦️.ts` | **53-58**; also 11-17 | l.49 registers the Cargo target discovery skip gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-cargo-target-discovery-skip` → `bun ./📜️script.ts test cargo-target-discovery-skip` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 12 | `🏺️historical-package-owner-identity/🟦️.ts` | **109-115** | l.103 historical owner identity gate is registered in Nx and both launch catalogs | `@semio-tech/repo-lib:test-historical-package-owner-identity` → `bun ./📜️script.ts test historical-package-owner-identity` | uses execution.launchCatalogs from fixture |
| 13 | `👀️readme-reviewed-fixture-inputs/🟦️.ts` | **335-339** | l.320 reviewed fixture gate registration matches its package route and both launch catalogs | `@semio-tech/repo-lib:test-readme-reviewed-fixture-inputs` → `bun ./📜️script.ts test readme-reviewed-fixture-inputs` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 14 | `💠️inventory-artifact-shards/🟦️.ts` | **182-187** | l.179 inventory shard consistency gate is registered through Nx and both launch catalogs | `@semio-tech/repo-lib:test-inventory-artifact-shards` → `bun ./📜️script.ts test inventory-artifact-shards` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 15 | `💥️nested-cargo-collision-authority/🟦️.ts` | **167-172** | l.164 collision gate is registered through Nx and both ordered launch catalogs | `@semio-tech/repo-lib:test-nested-cargo-collision-authority` → `bun ./📜️script.ts test nested-cargo-collision-authority` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 16 | `📈️reference-coordinate-progress/🟦️.ts` | **192-198** | l.187 registration only: coordinate progress has one exact route package and launch owner | `@semio-tech/repo-lib:test-reference-coordinate-progress` → `bun ./📜️script.ts test reference-coordinate-progress` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 17 | `📣️plugin-publication-source-ownership/🟦️.ts` | **173-176** | l.165 registers the gate in package, Nx, and launch authorities | `@semio-tech/repo-lib:test-plugin-publication-source-ownership` → `bun ./📜️script.ts test plugin-publication-source-ownership` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 18 | `📱️app-verification-source-ownership/🟦️.ts` | **139-142** | l.131 registers the portable gate through package, Nx, and both launch projections | `@semio-tech/repo-lib:test-app-verification-source-ownership` → `bun ./📜️script.ts test app-verification-source-ownership` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 19 | `🔎️json-reference-owner-lookup/🟦️.ts` | **111-116** | l.104 registers the JSON owner lookup gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-json-reference-owner-lookup` → `bun ./📜️script.ts test json-reference-owner-lookup` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 20 | `🔖️readme-current-source-revision/🟦️.ts` | **287-292** | l.272 revision gate registration matches the declared Nx route and both launch catalogs | `@semio-tech/repo-lib:test-readme-current-source-revision` → `bun ./📜️script.ts test readme-current-source-revision` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 21 | `🔤️taxonomy-leading-grapheme/🟦️.ts` | **176-181**; also 155 | l.151 registers leading grapheme through its closed canonical route | `@semio-tech/repo-lib:test-taxonomy-leading-grapheme` → `bun ./📜️script.ts test taxonomy-leading-grapheme` | exact row for registration.launchName |
| 22 | `🔬️workspace-contract/🟦️.ts` | **526-535, 675-682, 4695, 4709, 5269-5275**; also 4302-4309, 4702, 4733, 5248-5276, 5466 | l.474 resident native metadata binds the exact semantic owner and one Cargo identity | (see 3.7)  | special: see section 3.7 (draw-source launch seed scenario, expected.launch rows, registry inputPatterns) |
| 23 | `🕰️historical-json-source-encoding/🟦️.ts` | **225-228** | l.220 both historical source gates are mounted through Nx and exact launch registrations | `@semio-tech/repo-lib:test-historical-json-source-encoding` → `bun ./📜️script.ts test historical-json-source-encoding` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 24 | `🗺️testing-readme-coordinates/🟦️.ts` | **153-158** | l.138 documentation gate registration matches the declared Nx route and both launch catalogs | `@semio-tech/repo-lib:test-testing-readme-coordinates` → `bun ./📜️script.ts test testing-readme-coordinates` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 25 | `🚚️readme-move-source-authority/🟦️.ts` | **356-359**; also 360-366 | l.341 move source authority gate registration matches the package Nx router and both launch cata | `@semio-tech/repo-lib:test-readme-move-source-authority` → `bun ./📜️script.ts test readme-move-source-authority` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 26 | `🚧️cargo-discovery-exclusions/🟦️.ts` | **94-99**; also 14 | l.90 registers the Cargo exclusion gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-cargo-discovery-exclusions` → `bun ./📜️script.ts test cargo-discovery-exclusions` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 27 | `🚪️artifact-io-ownership/🟦️.ts` | **281-284** | l.266 artifact IO contracts and command registration parse independently | `@semio-tech/repo-lib:test-artifact-io-ownership`; `@semio-tech/repo-lib:lint-artifact-io-ownership` → `bun ./📜️script.ts test artifact-io-ownership` | two targets (test- and lint-artifact-io-ownership) each need one row with --skip-nx-cache |
| 28 | `🛑️taxonomy-cli-cancellation/🟦️.ts` | **115-120** | l.111 registers the cancellation gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-taxonomy-cli-cancellation` → `bun ./📜️script.ts test taxonomy-cli-cancellation` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 29 | `🛟️transaction-recovery-authority/🟦️.ts` | **96-97** | l.89 recovery authority is mounted through its exact Nx and launch registrations | `@semio-tech/repo-lib:test-transaction-recovery-authority` → `bun ./📜️script.ts test transaction-recovery-authority` | launch.json only (no seed); exact row incl. presentation {4_gate, order} |
| 30 | `🛤️typescript-path-collection/🟦️.ts` | **210-217** | l.204 registers the kind-only canonical test through the package router and both launch catalogs | `@semio-tech/repo-lib:test-typescript-path-collection` → `bun ./📜️script.ts test typescript-path-collection` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 31 | `🛫️preflight-reference-basis/🟦️.ts` | **392-399** | l.385 the dedicated preflight gate is registered through Nx and both ordered launch catalogs | `@semio-tech/repo-lib:test-preflight-reference-basis` → `bun ./📜️script.ts test preflight-reference-basis` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 32 | `🟢️readme-current-source-activation/🟦️.ts` | **450-454**; also 224 | l.435 activation gate registration matches the package router and both launch catalogs | `@semio-tech/repo-lib:test-readme-current-source-activation` → `bun ./📜️script.ts test readme-current-source-activation` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 33 | `🥒️gherkin-description-inline-code/🟦️.ts` | **127-130** | l.121 registers the gherkin description inline-code gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-gherkin-description-inline-code` → `bun ./📜️script.ts test gherkin-description-inline-code` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 34 | `🥤️rust-finite-target-consumption/🟦️.ts` | **228-236** | l.218 exact finite consumer route and launch registration preserve the canonical semantic leaf | `@semio-tech/repo-lib:test-rust-finite-target-consumption` → `bun ./📜️script.ts test rust-finite-target-consumption` | launch.json only (no seed); exact row incl. env SEMIO_TEST_ARTIFACT_DIR with ${input:...} |
| 35 | `🦑️repo-source-ownership/🟦️.ts` | **125-128** | l.122 registers the portable check in both editor launch authorities | `@semio-tech/repo-lib:test-repo-source-ownership` → `bun ./📜️script.ts test repo-source-ownership` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 36 | `🧑‍💻os-dev-composition-ownership/🟦️.ts` | **154-156**; also 20, 31 | l.141 registers one Bun/Nx owner route through declared project target launch contributions | `@semio-tech/repo-lib:test-os-dev-composition-ownership` → `bun ./📜️script.ts test os-dev-composition-ownership` | launch.json only (no seed): `registration.launchContribution == "declaredProjectTargets"` and exactly one row with the command (`registration.derivedLaunchPath`); this is the only test that names the generator's `declaredProjectTargets` contribution
| 37 | `🧰️framework-root-source-topology/🟦️.ts` | **79-82** | l.76 registers the portable gate in both editor launch authorities | `@semio-tech/repo-lib:test-framework-root-source-topology` → `bun ./📜️script.ts test framework-root-source-topology` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 38 | `🧱️cargo-transaction-command-source/🟦️.ts` | **241-247** | l.225 registers exact Bun, Nx, cache, package, and launch closure | `@semio-tech/repo-lib:test-cargo-transaction-command-source` → `bun ./📜️script.ts test cargo-transaction-command-source` | launch.json only (no seed); four routes incl. goProjection/goDispatch/transaction |
| 39 | `🧱️framework-source-topology/🟦️.ts` | **97** | l.93 declares the portable gate and its generated editor entry | `@semio-tech/repo-lib:test-framework-source-topology` → `bun ./📜️script.ts test framework-source-topology` | launch.json only (no seed); one row by command |
| 40 | `🧱️manifestless-source-closure/🟦️.ts` | **98-101** | l.95 registers the closure gate in both editor launch authorities | `@semio-tech/repo-lib:test-manifestless-source-closure` → `bun ./📜️script.ts test manifestless-source-closure` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 41 | `🧱️root-artifact-dependency-source/🟦️.ts` | **126** | l.122 registers one Bun/Nx/editor route | `@semio-tech/repo-lib:test-root-artifact-dependency-source` → `bun ./📜️script.ts test root-artifact-dependency-source` | launch.json only (no seed); row by name+command |
| 42 | `🧱️root-artifact-schema-law-source/🟦️.ts` | **332-336**; also 35-37 | l.326 registers one Bun Nx and seed-derived launch route | `@semio-tech/repo-lib:test-root-artifact-schema-law-source` → `bun ./📜️script.ts test root-artifact-schema-law-source` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 43 | `🧱️root-clean-scaffold-source/🟦️.ts` | **200-204**; also 34-39 | l.195 registers one Bun Nx and seed-derived launch route | `@semio-tech/repo-lib:test-root-clean-scaffold-source` → `bun ./📜️script.ts test root-clean-scaffold-source` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 44 | `🧱️root-inference-law-source/🟦️.ts` | **288-292**; also 35-37 | l.280 retains native source data and registers one Bun Nx launch route | `@semio-tech/repo-lib:test-root-inference-law-source` → `bun ./📜️script.ts test root-inference-law-source` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 45 | `🧱️root-schema-field-source/🟦️.ts` | **116-120**; also 34-36 | l.111 registers one Bun Nx and launch route | `@semio-tech/repo-lib:test-root-schema-field-source` → `bun ./📜️script.ts test root-schema-field-source` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 46 | `🧱️root-surface-abstraction-law-source/🟦️.ts` | **202-206**; also 34-36 | l.197 registers one Bun Nx and seed-derived launch route | `@semio-tech/repo-lib:test-root-surface-abstraction-law-source` → `bun ./📜️script.ts test root-surface-abstraction-law-source` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 47 | `🧱️root-taxonomy-workflow-source/🟦️.ts` | **60-63** | l.55 registers one Bun Nx and launch route | `@semio-tech/repo-lib:test-root-taxonomy-workflow-source` → `bun ./📜️script.ts test root-taxonomy-workflow-source` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 48 | `🧱️wasm-package-wrappers/🟦️.ts` | **39, 129-132** | l.37 validates the exact language-agnostic wrapper contract | `@semio-tech/repo-lib:test-wasm-package-wrappers` → `bun ./📜️script.ts test wasm-package-wrappers` | uses registration.launchSeedPath/launchPath from fixture |
| 49 | `🧱️workspace-publication-source/🟦️.ts` | **-**; also 36, 287-295, 326-332 | - | `@semio-tech/repo-lib:test-workspace-publication-source` → `bun ./📜️script.ts test workspace-publication-source` | special: asserts launch files are Nx inputs (l.287-295) and three routes + retirement row (l.326-332) |
| 50 | `🧲️rust-physical-reference-context/🟦️.ts` | **136-141** | l.132 registers the physical Rust reference gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-rust-physical-reference-context` → `bun ./📜️script.ts test rust-physical-reference-context` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 51 | `🧾️registry-catalog-gitlink-boundary/🟦️.ts` | **111-116**; also 10-18 | l.107 registers the registry catalog gitlink boundary gate through Nx and both launch catalogs | `@semio-tech/repo-lib:test-registry-catalog-gitlink-boundary` → `bun ./📜️script.ts test registry-catalog-gitlink-boundary` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 52 | `🪟️windows-command-paths/🟦️.ts` | **207-211** | l.206 every launch row runs unchanged in PowerShell, cmd.exe and POSIX shells (only VS Code ${… | `@semio-tech/repo-lib:test-windows-command-paths` → `bun ./📜️script.ts test windows-command-paths` | scans EVERY row of both files for POSIX-only shell syntax (cross-platform law, not presence) |
| 53 | `🪶️artifact-empty-facet-authoring/🟦️.ts` | **453-469**; also 399-405 | l.393 registers empty-facet authoring through its closed canonical route | `@semio-tech/repo-lib:test-artifact-empty-facet-authoring` → `bun ./📜️script.ts test artifact-empty-facet-authoring` | registration presence (name/command[/group/order/env]) in seed + launch.json |
| 54 | `🫙️artifact-empty-facet-authority/🟦️.ts` | **121-127** | l.98 registers the empty-facet authority through its closed canonical route | `@semio-tech/repo-lib:test-artifact-empty-facet-authority` → `bun ./📜️script.ts test artifact-empty-facet-authority` | registration presence (name/command[/group/order/env]) in seed + launch.json |


### 3.7 Other tests and scripts that read or require launch data — table B (21 test files + 2 scripts)

"Runs under" names the Nx target or verb as far as I traced it statically; where I did not trace the exact target I say so.

| file | lines | what it asserts / reads | runs under | change; intent to preserve |
|---|---|---|---|---|
| `📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts` | `1` imports; `8–9` gate/`launch()` builders; `40–49` launchesFromSource laws; `50–67` coverage laws and generated-name classification; `68` `return 29` | self-tests of the gate in 3.3: valid launch file, TypeScript-oracle parity, missing required gate, capacity+1, complete/missing/wrong-command/wrong-renderer React+Wasm+native coverage | root `verify interactivity`, `verify interactivity apps` (`📜️script.ts:8078,8121`) | delete `40–49`, `52–67` with the implementation; fix the returned count; keep descriptor/action laws `10–39` |
| `📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts` (+ fixture `…/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json:67–76`) | `19–22` `LaunchRow`; `55–66`; `96` | each forwarded launch row names exactly one `*_PORT` env; `forwardPorts` and `portsAttributes` equal those ports plus the tool ports; fixture `forwardedLaunchRows` = `🛠️dev🗄️os-hub`, `🛠️dev🪐️space⚛️react`, `…👤️1/2⚛️react`, `…🧊️wgpu🌐️wasm`, `…👤️1/2🧊️wgpu🌐️wasm`, `🛠️dev📖️storybook`; devcontainer law at `.devcontainer/README.md:11,59` | `repo` caching `⚡️cache-contracts` test, which calls `testContainerRuntimeBootstrap` (`📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:24,64`) | **intent: the devcontainer forwards exactly the dev-server ports.** Re-express from the playground catalog (`🎠️playgrounds.json` `ports.react/wgpu`, `userPorts`) + the hub port + the fixture's tool ports (6010 Storybook, 6274/6277 MCP Inspector) |
| `📚️library/⚡️caching/🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts` | `37` | seed and launch each carry exactly one row `bun nx run <fixture.project>:<fixture.target>` | `repo:ci-baseline` area (not traced) | delete the line; intent "CI baseline command is reachable" stays covered by the adjacent Nx assertions |
| `📚️library/⚡️caching/🧪️tests/🌎️hub/🟦️.ts` | `25–27` | seed has `bun nx run <hub>:build` and its prerequisite ("Missing editor build command") | `repo` caching tests (not traced) | delete; intent "hub build targets are reachable" → Nx target existence |
| `📚️library/⚡️caching/🧪️tests/🐍️styling-python-outputs/🟦️.ts`; `…/🔒️trunk-lockfile/🟦️.ts`; `…/🧊️wasm-outputs/🟦️.ts` | `19–22`; `74–78`; `26` | both files contain a row for each fixture `project:target` pair | `repo` caching tests (not traced) | delete the loops; the Nx target checks beside them remain |
| `📚️library/⚡️caching/🧪️tests/🧱️command-source/🟦️.ts` (+ fixture `📚️library/⚡️caching/🧫️fixtures/🧱️command-source/🔣️.json:318–319`) | `31`; `224–239` | both files are in the `cacheCommandSources` inputs (`232–233`) AND contain `route.launchName` exactly once and `route.launchCommand` (`236–239`) | `repo:test-cache-command-source` | delete `232–233` and `236–239`; pair with `⚡️caching/📋️project.json:243–244` |
| `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` (+ fixture `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json:349–352,379`) | `24–31`; `373–378`; `408–419`; `441–450` | type with `route.launch*`; native credential population `launch` field; comment naming launch.json (`419–421`); exact row (name+command+group+order) in both files | `os-hub:foundation-source-check` | delete `441–450`, the `launch` population field (`:377`) and the `route.launch*` fixture keys; reword the comment; `inputs` `🌎️hub/📦️packages/🦀️rust/📋️project.json:68` |
| `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts` (+ fixture `…/🔣️.json:214–217,245–246,252–255`) | `15–21`; `200–208` | type; rows `source` and `native` exact in both files | `os-hub:socket-grant-command-source-check`, `os-hub:socket-grant-check` | delete `200–208`, fixture keys, and `🌎️hub/📦️packages/🦀️rust/📋️project.json:39–40` |
| `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts` | `4–5`; `114, 180`; `186–204` | population types carry `launch`; `source.launch.includes("os-hub:dev-secure-native")` / `("os-hub:dev-secure-mcp")`; populations read the seed | proofs called from `🌎️hub/📦️packages/🦀️rust/📜️script.ts:12963,13194,13198,13308–13309` and `🧱️foundation-source:381,396` | **intent: the secure native/MCP dev entries exist and run through the Bun/Nx owner chain.** Replace the `launch.includes` clauses by checks that Nx targets `os-hub:dev-secure-native` / `dev-secure-mcp` exist (`🌎️hub/📦️packages/🦀️rust/📋️project.json`); drop the `launch` field from both population types |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `2352, 2355`; `4653–4668` | asserts the seed contains `os-hub:dev-secure-admin` ("admin relay dev/launch ownership drift"); `proveHeadlessStdioLaunchIsolation` parses three seed rows and requires `SEMIO_TEST_ARTIFACT_DIR` and `CARGO_TARGET_DIR === <artifact>/cargo-target` in each (callers `:4914,4981`) | admin-relay proof; `os-hub:native-catalog-selection-check`, `…:native-openable-catalog-provider-check`, stdio `--stdio-only` | **content-bearing**: the isolation policy lives only in the launch-row env. Move it to Nx target `options.env`, then assert there; replace `:2355` by an Nx target check |
| `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts` (+ fixture `✏️s/🧑‍💻dev/🗄️stdio/🧫️fixtures/🏃️command-ownership/🔣️.json:251–252`) | `15`; `98, 107` | type; `launch.includes('"command": "<invocation>"')` for every composition command | `@semio-tech/stdio-js` package-graph contract (`✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏃️artifact-commands/🟦️.ts:2`) | delete `98,107` and the fixture keys; `inputs` `…/📦️packages/🟦️typescript/📋️project.json:32–33,65–66` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | `26–34` | two tests: the fifteen owning SQLite commands and fifteen source/build/check commands exist exactly once, with `presentation.order` and `env.SEMIO_TEST_LEVEL === "quick"` | wfc bitmap snapshot sqlite gates (targets from `fixture.owningLaunchCommands`) | **content-bearing** (order + quick level): move the quick level to the Nx targets, delete the order check |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪪️capability/🟦️.ts` | `10–12` | nine commands once each, `order = 408.623 + n/1000`, `SEMIO_TEST_LEVEL=quick`, the `test` command contains `-- quick --no-fail-fast` | `@semio-tech/energy-model[-rs]:test-snapshot-sqlite*`, `build`, `check`, `test` | **content-bearing**: move `quick --no-fail-fast` and the level into the Nx target |
| `✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts` | `107–110` | one row `bun nx run @semio-tech/norm-js:test-oracle-source` in both files | `@semio-tech/norm-js:test-oracle-source` | delete loop; `inputs` `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/📋️project.json:40–41` |
| `OS/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts` (+ fixture `…/🏷️ownership/🧫️fixtures/🔣️.json:372–383`) | `21`; `93–99` | type; package scripts equal `registration.launchCommand`/`previewLaunchCommand` and both files contain them | `semio-framework-os-flow-core:test-browser-ownership` | delete the launch half of `93–99` and fixture keys; `inputs` `OS/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json:85–86` |
| `OS/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts` | `89–93` | exactly one seed row for `test-space-history-sqlite-source` / `-native` with `--skip-nx-cache`, plus the project command | `semio-framework-os-kernel:test-space-history-sqlite-*` | delete the seed-row half (keep the project command check) |
| `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts` | `72–82` | row for `framework-renderer-wgpu:test-worker-cell --skip-nx-cache` with `presentation.order 900.05816` and `env.SEMIO_TEST_ARTIFACT_DIR = …/portable-launch-artifacts` | `@semio-tech/framework-renderer-wgpu:test-worker-cell` | **content-bearing** (caller-owned lifecycle output): move the env to the Nx target, assert there |
| `OS/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts` | `21–25` | a row exists for each `cases.completion.launches` target and none for `build --…` | production browser artifacts (not traced) | delete the loop |
| `📚️library/🧪️tests/🪟️windows-command-paths/🟦️.ts` | `206–214` | all rows of both files are free of POSIX-only syntax (env prefixes, `&&`, `;`, `$(`, `nohup`, `sleep`, …; only `${workspaceFolder}`, `${input:…}`, `${env:…}` allowed) | `repo-lib:test-windows-command-paths` | **intent: every developer-registered command runs on PowerShell, cmd and POSIX.** Re-express over the dashboard's command sources (Nx target commands + root scripts + declared tools); the dashboard already branches `cfg!(windows)` in `shell_spec` (`DASH/🌳️command-tree/🦀️.rs:279–287`) |
| `📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` | `526–535`, `675–682`; `4304–4311`; `4693–4720`; `4733–4735`, `5466–5467`; `5248–5276` | rows `expected.launch` and `expected.existingTs.launch` (from fixtures `🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json:117–150` and `…/🖥️ui-host-package/🔣️.json:380–425`); `DrawSourceScenario.launchSeed`; test "authored Draw source launch seed matches its host…" calls `generateLaunchJson`; the seed treated as an authored declared producer input of the registry contract; assertArtifactProjectionSingleCaseRoute row `4_gate` | `repo-lib` workspace-contract suite | delete the five blocks; fixture keys `🖍️draw-source-scenario/🔣️.json:44–48`, `…/🛣️commit-route/🔣️.json:11–12`, `…/📨️submitted-proof/🔣️.json:42–43`; drop the `launch` keys of the two handoff fixtures |
| `📚️library/🧪️tests/🧱️workspace-publication-source/🟦️.ts` (+ fixture `…/🔣️.json:108–121,134–139,250`) | `36`; `287–295`; `326–332` | route type; both launch files and `R/🧪️tests/🚀️launch/🟦️.ts` listed among source authorities (`292–294`); three routes + retirement row `📦️preview🤖️ticket-important-fem-handoff` | `repo-lib:test-workspace-publication-source` | remove `292–294`, the route launch asserts, fixture keys; `inputs` `📚️library/📦️packages/🟦️typescript/📋️project.json:16–17,21` |
| `📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts` | `802–805` | a row exists in both files | normalization suite (not traced) | delete the loop |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts` (+ fixture `🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json:332–333`) | `219–236` | inputs list (`227–228`) and a loop over both files (`233–236`) | `@semio-tech/repo-test-domain:test-command-composition-source` | delete the two inputs and the loop; `🧪️test/📋️project.json:399–400` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | `33–39, 43` | doc comments ("launch.json entries … inherit it") | puzzle3d rust script `test` verbs | reword |

### 3.8 Fixtures and schemas carrying launch keys — table C (55 files; false positives already removed)

Keys per registration: `launchName`, `launchCommand`, `launchGroup`, `launchOrder` (some also `launchPath`, `launchSeedPath`, `launchCatalogs`, `derivedLaunchPath`, `launchContribution`, `forwardedLaunchRows`). Schema `📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json:13–16,31,34,37,40` declares the four keys as required ⇒ remove from `required`/`properties` together with the fixture and its test. `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json:3594–3595` and its schema mirror `…/🧬️schema/🔣️.json:3682–3683` carry normative prose ("generated launch seed", "generated launch row for the existing Nx target") → reword.

| fixture / schema file | lines | keys carrying launch data |
|---|---|---|
| `✏️s/🧑‍💻dev/🗄️stdio/🧫️fixtures/🏃️command-ownership/🔣️.json` | 180, 181, 213, 214, 251, 252 | launch.json, launch.seed, launchPath, launchSeed |
| `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json` | 349, 350, 351, 352, 379 | launch.json, launch.seed, launchCommand, launchGroup, launchName, launchOrder |
| `🌎️hub/🧫️fixtures/🧱️socket-grant-command-source/🔣️.json` | 214, 215, 216, 217, 245, 246, 252, 253, 254, 255 | launch.json, launch.seed, launchCommand, launchGroup, launchName, launchOrder |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json` | 3683 | "launch" |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧫️fixtures/🔣️.json` | 372, 373, 380, 381, 382, 383 | launch.json, launch.seed, launchCommand, launchPath, launchSeed, previewLaunchCommand |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🧫️fixtures/🏭️generate/🔣️.json` | 6, 7 | launch.json, launch.seed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🧫️fixtures/🧱️placement/🔣️.json` | 12, 75, 106 | "launch" |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🧬️schema/🧱️placement/🔣️.json` | 39 | "launch" |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json` | 11 | launch.json |
| `📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json` | 67 | forwardedLaunchRows |
| `📚️library/⚡️caching/🧫️fixtures/🧱️command-source/🔣️.json` | 318, 319 | launchCommand, launchName |
| `📚️library/🧫️fixtures/↪️rust-divergence-callback/🔣️.json` | 947, 948, 949, 957, 961, 962, 969, 973, 974, 981, 985, 986 | launch.json, launch.seed, launchCommand, launchName, launchOrder, launchPath |
| `📚️library/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json` | 9, 10, 11 | launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/⚙️root-script-compiler/🔣️.json` | 83, 84, 85, 86 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/✍️rust-writable-path-authority/🔣️.json` | 227, 228, 229, 231, 232, 233 | launch.json, launch.seed, launchCommand, launchName, launchOrder, launchPath |
| `📚️library/🧫️fixtures/🌐️registry-import-language/🔣️.json` | 100, 101, 102, 103 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json` | 20, 21, 22, 23 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🎟️reference-coverage-selection/🔣️.json` | 7, 8 | launchName, launchOrder |
| `📚️library/🧫️fixtures/🎯️cargo-target-discovery-skip/🔣️.json` | 33, 34, 35, 36 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🏺️historical-package-owner-identity/🔣️.json` | 8, 9, 10 | launch.json, launch.seed, launchCatalogs, launchCommand, launchName |
| `📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json` | 9, 10, 11, 12 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json` | 11, 12, 13, 14, 36, 37 | launch.json, launch.seed, launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/💠️inventory-artifact-shards/🔣️.json` | 62, 63 | launchCommand, launchName |
| `📚️library/🧫️fixtures/💥️nested-cargo-collision-authority/🔣️.json` | 45, 46 | launchCommand, launchName |
| `📚️library/🧫️fixtures/📈️reference-coordinate-progress/🔣️.json` | 23 | launchName |
| `📚️library/🧫️fixtures/🔎️json-reference-owner-lookup/🔣️.json` | 160, 166, 167, 168, 169 | launch.seed, launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json` | 13, 14, 15, 16 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json` | 9, 10, 11 | launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json` | 42, 43 | launchName, launchOrder |
| `📚️library/🧫️fixtures/🖍️draw-source-scenario/🔣️.json` | 44, 45 | launch.seed, launchSeed |
| `📚️library/🧫️fixtures/🖍️draw-source-scenario/🛣️commit-route/🔣️.json` | 11, 12 | launchName, launchOrder |
| `📚️library/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json` | 9, 10, 11, 12 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🚚️readme-move-source-authority/🔣️.json` | 13, 14, 15, 16 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🚧️cargo-discovery-exclusions/🔣️.json` | 66, 67, 68, 69 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json` | 11, 12, 13, 14 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🛤️typescript-path-collection/🔣️.json` | 503, 504, 505, 506 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🛫️preflight-reference-basis/🔣️.json` | 65, 66, 67, 68 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json` | 12, 13, 14, 15 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json` | 452, 453, 454 | launchCommand, launchName, launchOrder |
| `📚️library/🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json` | 437, 438 | derivedLaunchPath, launch.json, launchContribution |
| `📚️library/🧫️fixtures/🧱️cargo-transaction-command-source/🔣️.json` | 302, 303, 308, 309, 314, 315, 320, 321, 326, 327 | launchCommand, launchName |
| `📚️library/🧫️fixtures/🧱️root-artifact-schema-law-source/🔣️.json` | 147, 148 | launchCommand, launchName |
| `📚️library/🧫️fixtures/🧱️root-clean-scaffold-source/🔣️.json` | 330, 331 | launchCommand, launchName |
| `📚️library/🧫️fixtures/🧱️root-inference-law-source/🔣️.json` | 183, 184 | launchCommand, launchName |
| `📚️library/🧫️fixtures/🧱️root-schema-field-source/🔣️.json` | 107, 108 | launchCommand, launchName |
| `📚️library/🧫️fixtures/🧱️root-surface-abstraction-law-source/🔣️.json` | 162, 163 | launchCommand, launchName |
| `📚️library/🧫️fixtures/🧱️root-taxonomy-workflow-source/🔣️.json` | 288, 289 | launchCommand, launchName |
| `📚️library/🧫️fixtures/🧱️wasm-package-wrappers/🔣️.json` | 74, 75, 76, 77, 103, 104 | launch.json, launch.seed, launchCommand, launchName, launchPath, launchSeed |
| `📚️library/🧫️fixtures/🧱️workspace-publication-source/🔣️.json` | 108, 109, 114, 115, 120, 121, 134, 135, 250 | "launch", launch.json, launch.seed, launchCommand, launchName |
| `📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json` | 342, 343, 344, 345 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🧾️registry-catalog-gitlink-boundary/🔣️.json` | 15, 16, 17, 18 | launchCommand, launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json` | 9, 10, 11 | launchGroup, launchName, launchOrder |
| `📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json` | 9, 10, 11 | launchGroup, launchName, launchOrder |
| `📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json` | 13, 14, 15, 16, 31, 34, 37, 40 | launchCommand, launchGroup, launchName, launchOrder |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json` | 332, 333 | launchCommand, launchName |


### 3.9 Documentation, comments and messages that mention launch rows/files

| file | lines | text / role | change |
|---|---|---|---|
| `README.md` | `658–672`, `863` | section "The four golden paths" (a table of launch rows) and the claim that each canonical root command "has a `.vscode/launch.json` row in the `3_dev` group … Run panel" | rewrite to the dashboard (`bun run dashboard`) |
| `.devcontainer/README.md` | `11, 59` | forwarded-ports law ("the ports the launch rows start") | reword to catalog/dashboard-derived ports |
| `🌎️hub/README.md` | `213` | "(launch rows …" | reword |
| `✏️s/🔌️plugins/🌀️procedural/README.md` | `12` | names launch entry `🛠️dev🔧️procedural🏙️3d🥽️mesh-workbe…` | reword |
| `🎓️teaching/README.md`, `🎓️teaching/🏛️architecture/README.md`, `🎓️teaching/🏛️architecture/❓️quiz/README.md`, `…/❓️quiz/🧱️stack/🟦️.ts` | `63`; `11`; `44` ("Run everything from `.vscode/launch.json` (groups `3_dev` and `4_gate`)"), `159`; `20` | runbooks naming launch rows | reword |
| `🎓️teaching/🏛️architecture/🐾️pets/README.md` | `358–359` | table naming both `.claude/launch.json` entry `architecture-pets-stories` and the VS Code row | reword |
| `🎓️teaching/🛂️proctor/README.md` | `285, 288–292, 295` | seven command-table rows "(launch row …)" | reword |
| `🧰️framework/🔨️modules/🖼️assets/README.md` | `9` | "assets build launch configuration" — **generated** from `🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts:326` | change the template, regenerate |
| `🧰️framework/🛍️products/🎤️presentation/README.md`, `🧰️framework/🛍️products/📓️print/README.md` | `22`, `22` | "… `.vscode/🧩️launch.seed.jsonc` as launch configurations" | reword |
| `DASH/README.md` | see 3.4 | | |
| error messages / comments | `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:105,121,142`; `🌎️hub/🤝️integration-harness/🟦️.ts:190` (error "launch row 🛠️dev🗄️os-hub publishes the development catalog"); `OS/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts:9`; `OS/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts:24`; `OS/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts:817` (error "launch row 🛠️dev🪐️space⚛️react🔒local-only"); `✏️s/🧑‍💻dev/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:297` (error); `📚️library/🎮️playground/🔒️preferences/🟦️.ts:13`; `📚️library/🟦️.ts:2002`; `…/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:266`; `…/⚡️epjson/🔖️25.2/✳️any/🧪️tests/🔬️unit/🦀️.rs:182`; `…/🏛️export-epjson-runs-in-energyplus/🐍️.py:80`; `🧰️framework/🛍️products/📓️print/🔮️oracles/🔣️.json:81` (lists the stale path `R/🚀️launch.test.ts`) | text that tells a dev which launch row to start | replace with the dashboard path ("dev / <plugin> / <variant> / react") or the Nx command |
| `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` | `1262–1263`, `2876, 4969, 7980, 9737, 12794` | prompt history log: the decision to leave VS Code, plus historical requests to put things in launch.json | leave (history); report to the owner |

Historical material that mentions launch.json and should be left alone: 2,050 files under `.🧬semio/🦑️repo/🎫️tickets`, 116 files under `.cursor/plans` (e.g. `dashboard_tui_workforce_775ac26d.plan.md:6,30,46,121,125` planned "add dashboard entries to the launch seed"; `dashboard_wizard_windows_f834d098.plan.md:48` "launch.json plus the seed stay untouched in this ticket"), `.codex/plans`, agent memory in `/Users/ueli/.claude/projects/-Users-ueli-Documents-semio/memory` (9 files cite `.claude/launch.json`, `launch.json`, `preview_start`).

### 3.10 Ignore / repository configuration

`.gitignore:589–593` whitelists `!.vscode/settings.json`, `!.vscode/tasks.json`, `!.vscode/launch.json`, `!.vscode/extensions.json`, `!.vscode/*.code-workspace` — only `:591` is launch-specific (no `.vscode` ignore rule exists above it, so the exceptions are redundant). `.gitignore:3` (`*.local.*`) already ignores `.claude/settings.local.json`; `.claude/launch.json` is tracked.

---

## 4. Editor / agent integration surface (what else starts things) and how it behaves with only the dashboard

| surface | launch/run dependency | after the change |
|---|---|---|
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | the data | deleted; VS Code "Run and Debug" panel and `serverReadyAction` browser-open disappear. Devs use `bun run dashboard` (`package.json:33`; daemon controls `:34–36`, install `:162`, preferences `:163`). The 356 `serverReadyAction` rows have **no dashboard equivalent** (control-plane-completion.md "Not covered") |
| `.vscode/settings.json` | none for launch. `remote.autoForwardPorts:true` + `remote.portsAttributes` (`:52–66`, ports 6010 Storybook, 6274/6277 MCP Inspector) keep working because dashboard tasks run in VS Code's remote terminals | unchanged; ports labelled there are the "tool ports" of the devcontainer law |
| `.vscode/extensions.json`, `.vscode/mcp.json` | none (`mcp.json` starts `bun ./📜️script.ts dev mcp stdio copilot` / `… os`, VS Code-spawned) | unchanged. `.vscode/settings.json`/`extensions.json` are also policed by statutes (`📜️statutes/…/🐹️.go:2330–2440`, "VSCode settings must be inside devcontainer.json") — unrelated, listed so nobody deletes `.vscode/` wholesale by mistake |
| `.vscode/tasks.json` | does not exist (prompt `🐙️ueli.md:2876` merged tasks into launch.json) | nothing to do |
| `.claude/launch.json` | Claude Code `preview_start <name>` resolves a server by name (79 entries, e.g. `s-react`, `cad-react`, `architecture-pets-stories`, the `*-attach` url entries); referenced by `🎓️teaching/🏛️architecture/🐾️pets/README.md:358` and by agent memory notes | delete; agents lose named preview servers. Replacement for agents: `bun nx run …` via Bash/background, or the dashboard daemon (`bun run dashboard:start/status/stop` exist; **no headless "run label" verb is documented**) — D3 |
| `.claude/settings.json` | no launch; hooks block is commented out (`:11–57`); `deny` list includes `Bash(git checkout)` (`:82`) | unchanged |
| `.claude/settings.local.json` | ignored by `*.local.*`; contains ticket Bash permissions only | unchanged |
| `.cursor/hooks.json`, `.cursor/mcp.json` | none (hooks disabled; MCP `bun ./📜️script.ts dev mcp stdio cursor`) | unchanged |
| `.codex/config.toml`, `.codex/plans/declarative-actions.md` | none | unchanged |
| `.agents/skills/{clean,commit,merging,micro-commit}` | none | unchanged |
| `.mcp.json` | none (`stdio client`, `os`) | unchanged; its path list `📜️script.ts:17404` stays |
| `.devcontainer/devcontainer.json` | `forwardPorts` `:48` = `[8787, 6070, 6072, 6073, 6066, 6067, 6068, 6010, 6274, 6277]`, `portsAttributes` `:49–90` labelled `os-hub`, `s React`, `s React user 1/2`, `s wgpu`, `s wgpu user 1/2`, `Storybook`, `MCP Inspector` — equality with launch rows is a TEST (`runtime-bootstrap`, table B) and a README law; lifecycle commands `postCreate/Start/Attach` (`:40–42`) run `workspace:setup` and `📜️script.ts setup devcontainer …` (no launch) | ports stay; law re-derived from the playground catalog |
| `.devcontainer/README.md:11, 59`, `Dockerfile`, `docker-compose.yml` | text only | reword |
| `.github/workflows/architecture-quiz.yml` | runs `bun nx run @teaching/architecture-quiz:test|publish|docker-image-*` (`:62,64,108,115`) — Nx only; `.github/hooks/*.json`, `dependabot.yml` none | unchanged (CI never used launch rows) |
| `.config/nextest.toml`, `.storybook/main.ts`, `go.work`, `CMakePresets.json`, `Monorepo.sln` | none | unchanged |
| root `package.json` scripts `dev:*`, `build:*`, `test:*`, `check:*`, `publish:*`… (106 of 134 scripts) | duplicate many launch rows as `bun nx run …`; **discovered by the dashboard as workspace scripts** (`collect_workspace_scripts`) | unchanged; they become the manual-registration surface (D1) |

---

## 5. AGENTS.md and prompt files that instruct use of launch.json (report to the repo owner; agents must not edit)

| file | line | text |
|---|---|---|
| `AGENTS.md` | `50` | `- All devs are using \`launch.json\` and never use the cli.` |
| `AGENTS.md` | `51` | `  - You MUST register all executable commands there by following the existing order, grouping and naming.` |
| `AGENTS.md` (root) | other | no other mention; the 48 nested `AGENTS.md` files (`✏️s/…`, `🧰️framework/…`, 1 per plugin) contain **no** launch/vscode/claude/preview text (only `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/AGENTS.md:4,6`, which names a *vscode bundle*, unrelated) |
| `📜️script.ts` | `8720–8721` | comment quotes the same law (“All devs are using `launch.json` and never use the cli” — a verification verb that no launch row runs is a verb no dev can run) |
| `README.md` | `660–661` | “Every one of these is a `.vscode/launch.json` row — no terminal needed (AGENTS.md: devs use `launch.json`, never the CLI).” |
| `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` | `1262–1263` | the decision to leave VS Code (“We are moving away to a self-contained dashboard tui. dashboard is becoming the new way for devs to start dev, build, test, etc”) — the prompts directory also holds `🐳️kinan.md`, `🦢️niloufar.md` (no launch text) |

Also tell the owner: the AGENTS.md law is *enforced* by 75 tests + `verify interactivity apps` (section 3), so the AGENTS.md line must change in the same release or agents will keep re-adding rows that no generator honours.

---

## 6. Risk notes, ordered by blast radius

**R1 — registry generator on the build critical path (whole workspace).** `generate` reads the seed first thing (`R/🚀️launch/🟦️.ts:79–80` `readFileSync(seedPath)`); deleting seed/launch.json before editing `R/📽️projection/🟦️.ts` makes `plugin-registry:generate`, `:check`, `:check-generated`, `:preview-generated` throw. Callers: root `prepare`, `verify gate`, `framework-os-dev` serve/dev chains, wgpu builds, hub stdio script, dashboard `plugin-registry` refresh. Blocking order: stop generator → then delete.

**R2 — loss of ≈2,900 hand-registered commands (product regression, no test fails).** The dashboard's 43,913 commands (control-plane-completion.md) include everything in launch.json. Only the 1,342 plain `bun nx run P:T` rows and the 335 `workspace:dev` playground rows are re-derivable from Nx targets / the playground catalog; the other 4,586 − 1,342 − 335 = 2,909 rows carry information that target discovery does not reproduce, among them: 1,785 rows pointing at ticket scripts, 2,546 with env policy (`NX_DAEMON=false`, `NX_ISOLATE_PLUGINS`, `SEMIO_TEST_LEVEL`, private artifact dirs), 1,038 with other `cwd`, 155 with pick-list inputs, 4 compounds (`🧭️compound🖥️s⚛️react🌉️os-mcp`, `…⚛️react🗄️os-hub`, `…👥️users🗄️os-hub`, `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor`), 356 browser-open actions. How many to migrate is a policy decision: `launch-inventory.md` §1 (peer audit, same folder) classifies 3,581 of 4,584 rows as droppable (1,867 ticket-scoped rows in 8 still-open tickets, 663 dead rows, redundant plain/playground rows) and 1,003 as intent-bearing (≈141 registry entries). Decision needed before deleting (D1).

**R3 — sha-sealed authority edits.** The projection asset (`:726–742`) is pinned by `taxonomy.json:15961` and the frozen seal ledger (`:77`, reseals list). Editing it without a reseal fails the frozen-ledger / `historical-json-source-encoding` family; leaving the two launch consumers in place makes the asset describe files that no longer exist. Needs the reseal protocol (new seal + `recordedBy` ticket + evidence revision).

**R4 — taxonomy generator contract.** `check` runs `validateGeneratorContractsAgainstWorkspace` (`R/📽️projection/🟦️.ts:478`); a declared `tracked` output root that does not exist on disk is reported as a contract problem (`📚️library/🔍️discovery/🟦️.ts:6217`, `exactOwnerGeneratorPrestate`/`nestedCargoGeneratedPrestate` aside); `inputPatterns` are not existence-checked there, but `cache-contracts` requires every pattern to be a target input. Edit `taxonomy.json:26969, 27015–27018` in the same change as the projection edit. The `os-registry-launch` kinds (`:9213`, `:14758`) must go with the folder (artifact-name registry gate: member names vs disk).

**R5 — root `verify interactivity apps` (and `--actions`) fails closed** once the files vanish: `policyReadFileSafe` returns `""` for a missing file (`📚️library/🔍️discovery/📖️source-access/🟦️.ts:97–103`), `Bun.JSONC.parse("")` throws, so `interactivityAllAppLaunchesFromSource` returns `"invalid JSONC"` and `:8126` throws. Edit 3.3 before deleting. Statically the gate is already red today: `INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY = 512` (`:8699`) is far below the real 4,586 rows, so `:8964` pushes a capacity failure and the required-gate scan (`:8965–8975`) only sees the first 512 rows (not run; static reading).

**R6 — 75 tests + 55 fixtures fail with ENOENT** the moment the files are deleted (each `readFileSync(".vscode/…")` is unguarded). Their targets: `@semio-tech/repo-lib:test-*` (54), `os-hub:foundation-source-check`, `os-hub:socket-grant-command-source-check`, hub `dev secure-*`/native-catalog gates, `repo:test-cache-command-source`, `repo:ci-baseline`, `@semio-tech/stdio-js` graph/contract, `@semio-tech/norm-js:test-oracle-source`, `semio-framework-os-flow-core:test-browser-ownership`, `semio-framework-os-kernel:test-space-history-sqlite-*`, `framework-renderer-wgpu:test-worker-cell`, `repo-test-domain:test-command-composition-source`, energy/wfc snapshot gates, devcontainer bootstrap test.

**R7 — policy that lives only in launch-row env/args (silent loss).** `SEMIO_TEST_LEVEL=quick` (506 rows; wfc, energy gates), `-- quick --no-fail-fast` (energy), private `SEMIO_TEST_ARTIFACT_DIR`/`CARGO_TARGET_DIR` isolation (hub headless-stdio, worker-cell), `NX_DAEMON=false`/`NX_ISOLATE_PLUGINS` wrappers. Deleting the assertions without moving the policy into Nx target `options.env`/configurations means the dashboard (which runs targets "as declared") runs them without it.

**R8 — frozen name-collision with the dashboard's own playground catalog.** `R/🤖️generated/🚀️playgrounds.json` and `🎮️playgrounds/🟦️.ts` are rendered from `launchPlaygrounds` = playgrounds ∪ withheld stale-channel rows (`R/📽️projection/🟦️.ts:300,313–314`; `generateWithheldPlaygroundRegistry` `R/🎮️playground/🔎️discovery/🟦️.ts:210–217`, tested `R/🔎️discovery/🧪️tests/🟦️.ts:108–120`). The only stated reason for the withheld rows is "launch rows stay stable". The dashboard (`DASH/📦️packages/🦀️rust/🦀️.rs:176,181`), `📚️library/🎮️playground/🟦️.ts:17,23` and the root all-app gate (`📜️script.ts:8717`) consume that file. Do not delete the file; decide whether withheld rows stay (D4).

**R9 — concurrent churn.** `.vscode/launch.json` and the seed are modified by peers while this audit ran (git `MM`, mtime 19:10). Any agent running `plugin-registry:generate` or adding a row after removal re-creates/reintroduces launch data. Announce the freeze; remove the generator first; add nothing new to the seed.

**R10 — documentation drift only:** READMEs, error messages, comments (3.9). No target fails.

**R11 — agents lose named preview servers** (`.claude/launch.json`, 79 entries, 24 attach-only). No gate; breaks agent boot recipes (memory notes) and `preview_start`.

**R12 — dashboard cache shape.** Removing the `launch` segment changes the tree shape: bump `SNAPSHOT_VERSION` (`DASH/📚️inventory/🦀️.rs:17`), update label test `:124`.

**R13 — `registryCatalogInputPaths` side effect.** Removing the projection shrinks the catalog input set (every `📋️project.json` currently included); unexpected interaction with `repo:generator-inputs` receipts and the `🧪️tests/🚀️launch` test "declares every launch-projected project manifest as a registry catalog content input". Verify once after the edit.

### Design decisions the owner must make before dispatch (referenced above as D1–D4)

- **D1 — native registry for hand-registered commands and compounds.** Where do ad-hoc commands (ticket scripts, env-bearing rows, pick-list rows) and the 4 compounds live once launch.json is gone? Options: a dashboard-owned JSONC/event journal (`.🧬semio/🦑️repo/🎛️dashboard/…`, mirrors the preferences journal), or Nx target definitions. The existing fixture `DASH/🧫️fixtures/🚀️launch-configurations` is the behavioural spec to keep.
- **D2 — verb catalogue** for argument forms of one Nx target (`workspace:verify -- interactivity apps --actions`, `workspace:dev -- <variant>`, `verify dependencies literal-external`, `bun ./📜️script.ts dev mcp stdio …`): the dashboard needs explicit leaves, and the replacement of the 7 required gates asserts against them.
- **D3 — headless/agent entry** (replacement for `.claude/launch.json` `preview_start`): a documented `dashboard run <label>`-style verb, or accept `bun nx run` via Bash.
- **D4 — withheld playground rows** (R8) and renaming `launchPlaygrounds` → `catalogPlaygrounds`.

---

### Alignment with `fleet-plan.md` (coordinator document in the same folder)

`fleet-plan.md` §2 (declarations in `metadata.semio.dashboard`, ticket `🎮️commands.json`, parameters, `semio run`) answers D1, D3 and the env part of R7. Items of this report that §2 does **not** mention and that the L-1 slice must still own: (a) taxonomy generator-contract entries and the frozen-seal reseal (R3, R4); (b) `registryCatalogInputPaths` / `catalogExists` side effect (R13); (c) the interactivity all-app gate rewrite and the 7 required argument forms (3.3, D2) — `workspace:verify -- interactivity apps --actions` etc. need declared leaves (`tools`/`groups`) or Nx target configurations; (d) per-gate isolation env that contains artifact paths (`SEMIO_TEST_ARTIFACT_DIR`, `CARGO_TARGET_DIR`; §2 says paths are workspace-relative and never stored absolute) — hub headless-stdio and worker-cell need a durable owner such as Nx target `options.env`; (e) the devcontainer port law and its test (3.7); (f) `R/🤖️generated/🚀️playgrounds.json` must survive (R8); (g) `.claude/launch.json` consumers other than `preview_start` (README `🐾️pets/README.md:358`, agent memory); (h) `serverReadyAction`: §2 `ready` covers readiness, but opening the browser is only a statement in `control-plane-completion.md` "Not covered".

---

## 7. Proposed removal order (dependency-safe checklist)

Phases are ordered so no gate is red between steps if a phase lands atomically; within a phase items are independent (suitable for parallel agents — suggested slices in brackets).

**Phase 0 — freeze and decide (no file operations)**
- [ ] Announce freeze: no new seed rows, no `plugin-registry:generate` by peers until Phase 5 completes (R9).
- [ ] Owner decides D1–D4; owner edits `AGENTS.md:50–51` (agents must not).

**Phase 1 — dashboard stops depending on launch.json [slice S1: 8 files]**
- [ ] `DASH/🌳️command-tree/🦀️.rs`: delete `:48`, `:224–256`, `:257–276`, `:289–309` (and `:278–287` if unused), change call sites `:62,72`; implement D1/D2 source.
- [ ] `DASH/…/🧪️tests/🔬️unit/🦀️.rs:199–238`, `DASH/🧪️tests/🌀️control-plane/🟦️.ts:56–82`, `DASH/🌀️daemon/🧪️tests/🔬️unit/🦀️.rs:38–73`, fixture dir `DASH/🧫️fixtures/🚀️launch-configurations/*`, `DASH/🧬️schema/🔣️.json:15,22–33`, `DASH/📚️inventory/🦀️.rs:17,124`, `DASH/README.md:9,21–35,103,121,132`.

**Phase 2 — re-home content-bearing policy (R7) [slice S5]**
- [ ] Move `SEMIO_TEST_LEVEL`, `quick --no-fail-fast`, private artifact/cargo-target env into the Nx targets for: wfc sqlite gates, energy capability gates, hub headless-stdio gates, `framework-renderer-wgpu:test-worker-cell`.
- [ ] Re-express: devcontainer ports (from playground catalog), windows-command-paths (over dashboard sources), credential-source-order (`os-hub:dev-secure-*` targets).

**Phase 3 — remove assertions, fixtures, inputs [slices S4a/S4b/S5]**
- [ ] Table A: delete the 54 launch blocks; Table C: delete the launch keys of 55 fixtures/schemas; section 3.7 rows (24 rows: 21 test files + 2 scripts + the puzzle comment).
- [ ] 3.5: delete the 27 `inputs` lines in 7 `project.json` files and their fixture mirrors, in the same edit as their tests.
- [ ] Replace per-test assertions by one shared "every target is a dashboard leaf" law (optional, D2).

**Phase 4 — stop the generator and delete the module [slice S2]**
- [ ] `R/📽️projection/🟦️.ts` (3.2), `R/📜️script.ts` (3.2), `R/📋️project.json` targets (3.2), `R/🛂️descriptor-verification/🟦️.ts:8–12`, `R/🎮️playground/🔎️discovery/🟦️.ts` prefix plumbing, `🌎️hub/🧩️compositions/🎪️demonstrator/…/Cargo.toml` (7 keys).
- [ ] `📚️library/🔣️taxonomy.json:9213–9222, 14758–14766, 26969, 27015–27018`; nested-cargo projection asset `:726–742` + reseal (`taxonomy.json:15961`, `🧫️frozen-seal-ledger`); `📚️library/🔍️discovery/🟦️.ts:10106–10109, 11554, 11566, 11585` review; fixtures `📣️plugin-publication-source-ownership/🔣️.json:206–211,235–243`, `🖥️os-source-topology/🔣️.json:154–164`.
- [ ] Delete the 16 files of 3.1 (`R/🚀️launch/**`, `R/🧪️tests/🚀️launch/**`, `R/🧫️fixtures/🚀️launch/**`, `R/🧬️schema/🚀️launch/**`).

**Phase 5 — root gate [slice S3]**
- [ ] `📜️script.ts` regions of 3.3 (+ exports `:25733–25744`), `📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts` per 3.7; re-express required gates (D2) and variant coverage over the playground catalog.

**Phase 6 — delete data and config**
- [ ] Plain file removal (no git commands; the auto-commit daemon records it): `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json`; `.gitignore:591` (and optionally the other redundant `.vscode` exceptions `:589–593`).
- [ ] Verify nothing references `.claude/launch` (only `🎓️teaching/🏛️architecture/🐾️pets/README.md:358`).

**Phase 7 — docs, comments, messages [slice S6]**
- [ ] 3.9 list (README, teaching, proctor, hub, procedural, assets template + regenerate README, presentation/print, devcontainer README, 17 comment/message files, scalar contract JSON + schema mirror).
- [ ] `.devcontainer/README.md:11,59` law text.

**Phase 8 — verification for the execution agents (I ran none of these)**
- [ ] `bun nx run @semio-tech/plugin-registry:check` and `:check-generated`, `:test`
- [ ] `bun nx run workspace:verify -- interactivity apps` and `-- gate`
- [ ] `bun nx run-many -t test -p @semio-tech/repo-lib` (or the 54 `test-*` targets), plus the non-library targets of table B
- [ ] `bun nx run @semio-tech/repo-dashboard-rs:test` (+ `SEMIO_TEST_CLI` native smoke)
- [ ] `grep -rn 'launch\.json\|launch\.seed\|🚀️launch' ` outside tickets/.cursor/plans/prompts returns nothing (use `/usr/bin/grep`; the shell alias misses emoji paths)

Suggested parallel slices (≈ file counts): S1 dashboard (8) · S2 registry+taxonomy+ledger (8+2+16 deletions) · S3 root script + interactivity self-tests (2) · S4a library tests A rows 1–27 + fixtures · S4b rows 28–54 + fixtures · S5 other tests/policy re-homing/project.json inputs (≈25) · S6 docs/comments (≈30) · S7 devcontainer/claude/gitignore (4). S2 must land before Phase 6; S1/S3/S4/S5 must land before or with S2.

---

## 8. Complete file index (every dependent outside tickets; 209 files + 3 data files)

Category key: **G** generator module · **W** registry wiring · **L** taxonomy/authority ledger · **S** root script · **D** dashboard · **N** Nx inputs · **T** test · **T2** other script · **F** fixture/schema/contract · **H** docs · **C** comment/message · **E** editor/ignore/agent instruction. Anchor lines are where the file mentions launch data (first 16 shown); exact semantics are in sections 3.1–3.10. Data files: `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json`.

| cat | file | anchor lines | action |
|---|---|---|---|
| G | `R/🚀️launch/🏷️name-prefix/🟦️.ts` | 73, 81 | delete file |
| G | `R/🚀️launch/🏷️name-prefix/🧪️tests/🟦️.ts` | 3, 6, 19, 25 | delete file |
| G | `R/🚀️launch/🏷️name-prefix/🧬️schema/🟦️.ts` | 1, 14 | delete file |
| G | `R/🚀️launch/🟦️.ts` | 2, 4, 5, 20, 26, 28, 31, 36, 45, 48, 58, 78, 79, 80, 81, 85 (+63 more) | delete file |
| G | `R/🚀️launch/🧪️tests/🏭️generate/🟦️.ts` | 24, 26, 32 | delete file |
| G | `R/🚀️launch/🧪️tests/🧪️seed-reconcile/🟦️.ts` | 1, 8, 11, 23, 59, 60, 72, 83, 84, 93, 106, 107 | delete file |
| G | `R/🚀️launch/🧪️tests/🧱️placement/🟦️.ts` | 8, 17, 19, 20, 25, 27, 28 | delete file |
| G | `R/🚀️launch/🧫️fixtures/🏭️generate/🔣️.json` | 6, 7 | delete file |
| G | `R/🚀️launch/🧫️fixtures/🧫️seed-reconcile/🔣️.json` | 41, 42, 43, 44, 276 | delete file |
| G | `R/🚀️launch/🧫️fixtures/🧱️placement/🔣️.json` | 11, 74, 104 | delete file |
| G | `R/🚀️launch/🧬️schema/🧱️placement/🔣️.json` | 1 | delete file |
| G | `R/🚀️launch/🧱️placement/🟦️.ts` | 9, 17 | delete file |
| G | `R/🧪️tests/🚀️launch/🟦️.ts` | 14, 16, 62, 63, 65, 79, 82, 183, 189, 191, 196, 198, 215, 218, 227, 266 (+2 more) | delete file |
| G | `R/🧫️fixtures/🚀️launch/🏷️name-prefix/🔣️.json` | 77, 83, 89 | delete file |
| G | `R/🧫️fixtures/🚀️launch/🔣️.json` | 1 | delete file |
| G | `R/🧬️schema/🚀️launch/🔣️.json` | 1 | delete file |
| W | `R/🎮️playground/🔎️discovery/🟦️.ts` | 5, 39, 74, 89, 210 | delete region(s) / rename (see 3.2) |
| W | `R/📋️project.json` | 190, 195, 240, 245, 252, 257, 264, 269, 274, 279 | delete region(s) / rename (see 3.2) |
| W | `R/📜️script.ts` | 14, 16, 23, 25, 26, 30, 31, 33, 35, 77, 91, 92, 93 | delete region(s) / rename (see 3.2) |
| W | `R/📽️projection/🟦️.ts` | 8, 128, 135, 148, 295, 296, 300, 306, 313, 314, 354, 368, 370, 371, 372, 373 (+19 more) | delete region(s) / rename (see 3.2) |
| W | `R/🔎️discovery/🧪️tests/🎮️session-catalog.feature` | 3 | delete region(s) / rename (see 3.2) |
| W | `R/🔎️discovery/🧪️tests/🚀️source-examples.feature` | 6 | delete region(s) / rename (see 3.2) |
| W | `R/🔎️discovery/🧪️tests/🟦️.ts` | 108 | delete region(s) / rename (see 3.2) |
| W | `R/🛂️descriptor-verification/🟦️.ts` | 10 | delete region(s) / rename (see 3.2) |
| L | `📚️library/🔣️taxonomy.json` | 9213, 14758, 26969, 27016 | taxonomy / authority ledger: delete entries (see 3.2, risk R3) |
| L | `📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json` | 727, 732, 738 | taxonomy / authority ledger: delete entries (see 3.2, risk R3) |
| S | `📜️script.ts` | 137, 144, 6705, 8079, 8114, 8699, 8708, 8709, 8711, 8713, 8714, 8715, 8716, 8718, 8720, 8721 (+33 more) | delete regions (see 3.3) |
| D | `DASH/README.md` | 9, 22, 27, 29, 31, 103, 121, 132 | rewrite: stop reading launch.json (see 3.4) |
| D | `DASH/🌀️daemon/🧪️tests/🔬️unit/🦀️.rs` | 44, 46, 47, 73 | rewrite: stop reading launch.json (see 3.4) |
| D | `DASH/🌳️command-tree/🦀️.rs` | 1, 25, 48, 62, 72, 224, 225, 226, 239, 253, 885 | rewrite: stop reading launch.json (see 3.4) |
| D | `DASH/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs` | 200, 203, 222 | rewrite: stop reading launch.json (see 3.4) |
| D | `DASH/🧪️tests/🌀️control-plane/🟦️.ts` | 56, 58, 65 | rewrite: stop reading launch.json (see 3.4) |
| D | `DASH/🧫️fixtures/🚀️launch-configurations/🔣️.json` | 1 | rewrite: stop reading launch.json (see 3.4) |
| D | `DASH/🧫️fixtures/🚀️launch-configurations/🥒️.feature` | 1, 2, 21 | rewrite: stop reading launch.json (see 3.4) |
| D | `DASH/🧬️schema/🔣️.json` | 15, 22, 24 | rewrite: stop reading launch.json (see 3.4) |
| N | `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/📋️project.json` | 40, 41 | delete the launch-file input lines |
| N | `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/📋️project.json` | 32, 33, 65, 66 | delete the launch-file input lines |
| N | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | 39, 40, 68 | delete the launch-file input lines |
| N | `OS/🌊️flow/🫀️core/📦️packages/🦀️rust/📋️project.json` | 85, 86 | delete the launch-file input lines |
| N | `📚️library/⚡️caching/📋️project.json` | 243, 244 | delete the launch-file input lines |
| N | `📚️library/📦️packages/🟦️typescript/📋️project.json` | 16, 17, 21, 55, 56, 60, 61, 575, 576, 604, 605, 1151 | delete the launch-file input lines |
| N | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json` | 399, 400 | delete the launch-file input lines |
| T | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 26, 27, 33 | delete launch block (tables A/B) or re-express |
| T | `✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts` | 107 | delete launch block (tables A/B) or re-express |
| T | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪪️capability/🟦️.ts` | 10, 12 | delete launch block (tables A/B) or re-express |
| T | `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts` | 15, 72, 98, 107 | delete launch block (tables A/B) or re-express |
| T | `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts` | 190, 203 | delete launch block (tables A/B) or re-express |
| T | `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` | 28, 377, 419, 422, 441, 446 | delete launch block (tables A/B) or re-express |
| T | `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts` | 19, 200, 203, 204, 205, 206 | delete launch block (tables A/B) or re-express |
| T | `OS/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts` | 21, 94, 95, 96, 98, 99 | delete launch block (tables A/B) or re-express |
| T | `OS/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts` | 89 | delete launch block (tables A/B) or re-express |
| T | `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts` | 73, 76 | delete launch block (tables A/B) or re-express |
| T | `OS/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts` | 21 | delete launch block (tables A/B) or re-express |
| T | `📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts` | 19, 55, 57, 59, 65, 96 | delete launch block (tables A/B) or re-express |
| T | `📚️library/⚡️caching/🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts` | 37 | delete launch block (tables A/B) or re-express |
| T | `📚️library/⚡️caching/🧪️tests/🌎️hub/🟦️.ts` | 25 | delete launch block (tables A/B) or re-express |
| T | `📚️library/⚡️caching/🧪️tests/🐍️styling-python-outputs/🟦️.ts` | 19 | delete launch block (tables A/B) or re-express |
| T | `📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts` | 74 | delete launch block (tables A/B) or re-express |
| T | `📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/🟦️.ts` | 26 | delete launch block (tables A/B) or re-express |
| T | `📚️library/⚡️caching/🧪️tests/🧱️command-source/🟦️.ts` | 31, 232, 233, 236, 238, 239 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts` | 1, 8, 41, 43, 46, 47, 48, 49, 52, 58, 59, 60, 61, 62, 64, 65 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts` | 110, 149, 150, 152 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts` | 298, 320, 323, 324, 325 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/⚙️root-script-compiler/🟦️.ts` | 116, 120, 121, 123, 124 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/✍️rust-writable-path-authority/🟦️.ts` | 44, 56, 57, 59, 65 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🌐️registry-import-language/🟦️.ts` | 16, 166, 173, 174, 176, 177 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts` | 124, 130 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts` | 19, 227, 231, 233, 235, 236 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts` | 229 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` | 279 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🎟️reference-coverage-selection/🟦️.ts` | 123, 126, 128 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🟦️.ts` | 16, 49, 53, 54, 56, 57 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts` | 103, 109, 111, 113 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts` | 320, 335, 337, 339 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/💠️inventory-artifact-shards/🟦️.ts` | 179, 182, 183, 185 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/💥️nested-cargo-collision-authority/🟦️.ts` | 164, 167, 168, 170 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/📈️reference-coordinate-progress/🟦️.ts` | 192, 198 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts` | 173 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/📱️app-verification-source-ownership/🟦️.ts` | 139 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🔎️json-reference-owner-lookup/🟦️.ts` | 104, 111, 112, 114, 115 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🔖️readme-current-source-revision/🟦️.ts` | 272, 287, 290, 291 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🟦️.ts` | 155, 176, 179, 180 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` | 526, 675, 4309, 4693, 4695, 4702, 4704, 4706, 4708, 4709, 4715, 4716, 4733, 5248, 5269, 5272 (+2 more) | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts` | 220, 225, 227 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts` | 138, 153, 156, 157 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🚚️readme-move-source-authority/🟦️.ts` | 341, 356, 358, 365 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🟦️.ts` | 14, 90, 94, 95, 97, 98 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🚪️artifact-io-ownership/🟦️.ts` | 281 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts` | 111, 115, 116, 118, 119 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🛟️transaction-recovery-authority/🟦️.ts` | 89, 96, 97 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🛤️typescript-path-collection/🟦️.ts` | 204, 210, 212, 214, 215, 216 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🛫️preflight-reference-basis/🟦️.ts` | 385, 392, 394, 396, 397, 398 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🟢️readme-current-source-activation/🟦️.ts` | 224, 435, 450, 452, 454 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🥒️gherkin-description-inline-code/🟦️.ts` | 121, 127 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts` | 218, 228, 230, 236 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts` | 125 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts` | 20, 31, 154, 155, 156 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧰️framework-root-source-topology/🟦️.ts` | 79 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️cargo-transaction-command-source/🟦️.ts` | 241, 244, 245 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts` | 97 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️manifestless-source-closure/🟦️.ts` | 98 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️root-artifact-dependency-source/🟦️.ts` | 126 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts` | 31, 35, 36, 326, 332, 334, 335 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts` | 30, 34, 35, 195, 200, 202, 203 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️root-inference-law-source/🟦️.ts` | 31, 35, 36, 280, 288, 290, 291 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️root-schema-field-source/🟦️.ts` | 30, 34, 35, 111, 116, 118, 119 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️root-surface-abstraction-law-source/🟦️.ts` | 30, 34, 35, 197, 202, 204, 205 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️root-taxonomy-workflow-source/🟦️.ts` | 55, 60, 61, 62, 63 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️wasm-package-wrappers/🟦️.ts` | 39, 121, 129, 131 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧱️workspace-publication-source/🟦️.ts` | 36, 292, 293, 294, 312, 327, 330, 332 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts` | 132, 136, 137, 139, 140 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🟦️.ts` | 17, 107, 111, 112, 114, 115 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🪟️windows-command-paths/🟦️.ts` | 206, 207 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts` | 404, 453, 457, 460, 461, 465, 468 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts` | 121, 124, 125, 126 | delete launch block (tables A/B) or re-express |
| T | `📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts` | 802 | delete launch block (tables A/B) or re-express |
| T | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts` | 213, 227, 228, 233, 235, 236 | delete launch block (tables A/B) or re-express |
| T2 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | 33, 43 | delete launch read / re-express (table B) |
| T2 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | 2352, 4654 | delete launch read / re-express (table B) |
| F | `✏️s/🧑‍💻dev/🗄️stdio/🧫️fixtures/🏃️command-ownership/🔣️.json` | 180, 181, 213, 214, 251, 252 | delete launch keys / rows |
| F | `🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` | 55, 61, 70, 79, 88, 97, 106 | delete launch keys / rows |
| F | `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json` | 349, 350, 351, 352, 379 | delete launch keys / rows |
| F | `🌎️hub/🧫️fixtures/🧱️socket-grant-command-source/🔣️.json` | 214, 215, 216, 217, 245, 246, 252, 253, 254, 255 | delete launch keys / rows |
| F | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json` | 3594, 3595 | delete launch keys / rows |
| F | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json` | 3682, 3683 | delete launch keys / rows |
| F | `OS/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧫️fixtures/🔣️.json` | 372, 373, 380, 381, 382, 383 | delete launch keys / rows |
| F | `OS/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json` | 11 | delete launch keys / rows |
| F | `📚️library/⚡️caching/📦️artifacts/🐳️containers/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json` | 67 | delete launch keys / rows |
| F | `📚️library/⚡️caching/🧫️fixtures/🧱️command-source/🔣️.json` | 318, 319 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/↪️rust-divergence-callback/🔣️.json` | 947, 948, 949, 957, 961, 962, 969, 973, 974, 981, 985, 986 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json` | 9, 10, 11 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/⚙️root-script-compiler/🔣️.json` | 83, 84, 85, 86 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/✍️rust-writable-path-authority/🔣️.json` | 227, 228, 229, 231, 232, 233 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🌐️registry-import-language/🔣️.json` | 100, 101, 102, 103 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json` | 20, 21, 22, 23 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🎟️reference-coverage-selection/🔣️.json` | 7, 8 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🎯️cargo-target-discovery-skip/🔣️.json` | 33, 34, 35, 36 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🏺️historical-package-owner-identity/🔣️.json` | 8, 9, 10 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json` | 9, 10, 11, 12 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json` | 11, 12, 13, 14, 36, 37 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/💠️inventory-artifact-shards/🔣️.json` | 62, 63 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/💥️nested-cargo-collision-authority/🔣️.json` | 45, 46 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/📈️reference-coordinate-progress/🔣️.json` | 23 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/📣️plugin-publication-source-ownership/🔣️.json` | 208, 238 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🔎️json-reference-owner-lookup/🔣️.json` | 160, 166, 167, 168, 169 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json` | 13, 14, 15, 16 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json` | 9, 10, 11 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json` | 42, 43 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🖍️draw-source-scenario/🔣️.json` | 44, 45, 47 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🖍️draw-source-scenario/🛣️commit-route/🔣️.json` | 11, 12 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🖥️os-source-topology/🔣️.json` | 157, 159 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json` | 9, 10, 11, 12 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🚚️readme-move-source-authority/🔣️.json` | 13, 14, 15, 16 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🚧️cargo-discovery-exclusions/🔣️.json` | 66, 67, 68, 69 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json` | 11, 12, 13, 14 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🛤️typescript-path-collection/🔣️.json` | 503, 504, 505, 506 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🛫️preflight-reference-basis/🔣️.json` | 65, 66, 67, 68 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json` | 12, 13, 14, 15 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json` | 117, 120, 131, 143, 145 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json` | 380, 383, 394, 405, 416 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json` | 452, 453, 454 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json` | 437, 438 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️cargo-transaction-command-source/🔣️.json` | 302, 303, 308, 309, 314, 315, 320, 321, 326, 327 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️root-artifact-schema-law-source/🔣️.json` | 147, 148 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️root-clean-scaffold-source/🔣️.json` | 330, 331 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️root-inference-law-source/🔣️.json` | 183, 184 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️root-schema-field-source/🔣️.json` | 107, 108 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️root-surface-abstraction-law-source/🔣️.json` | 162, 163 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️root-taxonomy-workflow-source/🔣️.json` | 288, 289 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️wasm-package-wrappers/🔣️.json` | 74, 75, 76, 77, 103, 104 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧱️workspace-publication-source/🔣️.json` | 108, 109, 114, 115, 120, 121, 134, 135, 139, 250 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json` | 342, 343, 344, 345 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🧾️registry-catalog-gitlink-boundary/🔣️.json` | 15, 16, 17, 18 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json` | 9, 10, 11 | delete launch keys / rows |
| F | `📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json` | 9, 10, 11 | delete launch keys / rows |
| F | `📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json` | 13, 14, 15, 16, 31, 34, 37, 40 | delete launch keys / rows |
| F | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json` | 332, 333 | delete launch keys / rows |
| H | `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` | 2876, 4969, 7980, 9737, 12794 | reword docs |
| H | `README.md` | 660, 661, 663, 863 | reword docs |
| H | `✏️s/🔌️plugins/🌀️procedural/README.md` | 12 | reword docs |
| H | `🌎️hub/README.md` | 213 | reword docs |
| H | `🎓️teaching/README.md` | 63 | reword docs |
| H | `🎓️teaching/🏛️architecture/README.md` | 11 | reword docs |
| H | `🎓️teaching/🏛️architecture/❓️quiz/README.md` | 44, 159 | reword docs |
| H | `🎓️teaching/🏛️architecture/🐾️pets/README.md` | 358, 359 | reword docs |
| H | `🎓️teaching/🛂️proctor/README.md` | 285, 288, 289, 290, 291, 292, 295 | reword docs |
| H | `🧰️framework/🔨️modules/🖼️assets/README.md` | 9 | reword docs |
| H | `🧰️framework/🛍️products/🎤️presentation/README.md` | 22 | reword docs |
| H | `🧰️framework/🛍️products/📓️print/README.md` | 22 | reword docs |
| C | `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts` | 78 | reword comment/message or delete read |
| C | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs` | 266 | reword comment/message or delete read |
| C | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🧪️tests/🔬️unit/🦀️.rs` | 182 | reword comment/message or delete read |
| C | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️export-epjson-runs-in-energyplus/🐍️.py` | 80 | reword comment/message or delete read |
| C | `✏️s/🧑‍💻dev/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts` | 297 | reword comment/message or delete read |
| C | `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` | 105, 121, 142 | reword comment/message or delete read |
| C | `🌎️hub/🤝️integration-harness/🟦️.ts` | 190 | reword comment/message or delete read |
| C | `🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts` | 20 | reword comment/message or delete read |
| C | `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts` | 69 | reword comment/message or delete read |
| C | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts` | 326 | reword comment/message or delete read |
| C | `OS/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts` | 24 | reword comment/message or delete read |
| C | `OS/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts` | 9 | reword comment/message or delete read |
| C | `OS/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts` | 817 | reword comment/message or delete read |
| C | `🧰️framework/🛍️products/📓️print/🔮️oracles/🔣️.json` | 81 | reword comment/message or delete read |
| C | `📚️library/🎮️playground/🔒️preferences/🟦️.ts` | 13 | reword comment/message or delete read |
| C | `📚️library/🔍️discovery/🟦️.ts` | 10109 | reword comment/message or delete read |
| C | `📚️library/🟦️.ts` | 2002 | reword comment/message or delete read |
| E | `.devcontainer/README.md` | 11, 59 | see section 4 |
| E | `.gitignore` | 591 | see section 4 |
| E | `AGENTS.md` | 50 | see section 4 |

