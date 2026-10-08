# Launch inventory: `.vscode/launch.json`, its seed and `.claude/launch.json`

Read-only audit for ticket `DASHBOARD-LAUNCH-COCKPIT`. Every number below is computed by `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/launch-inventory.ts`; the machine-readable result is `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🗑️generated/launch-inventory/inventory.json` (one record per configuration plus all summaries).

Reproduce (repository root): `bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/launch-inventory.ts" --verify-generator` (reads the snapshot; `--source=live` reads the live files; `--snapshot-live` refreshes the snapshot; `--skip-env-scan` skips the 8 s repository scan for environment readers).

| Input | Bytes | sha256 | Content |
| --- | --- | --- | --- |
| `.vscode/launch.json` | 3280714 | bdf3492dd814fe33… | 4584 configurations, 4 compounds, 82 inputs |
| `.vscode/🧩️launch.seed.jsonc` | 2696310 | 4cf47373214c946a… | 3512 configuration items (3423 objects, 89 placeholders), 4 compounds, 34 inputs, 47 devLaunchers, projectLaunchers policy |
| `.claude/launch.json` | 18767 | 2131b25690c6e589… | 79 preview-server entries |
| `…/📇️registry/🤖️generated/🚀️playgrounds.json` | 158521 | fb01b1d1aece915a… | 155 playground variants (the generator's second input) |

Source used for this report: **snapshot** (copies taken at the start of the audit under `<generated>/snapshot/`; other developers keep regenerating the live files, so the live counts drift by a few rows). Live files identical to the used source: launch no, seed no, claude yes, playgrounds yes (the coordinator's 4583/3511 were measured on an earlier revision of the same files).

JSONC: all three files parse with `Bun.JSONC.parse`; the scan found launch: 0 line comments, 0 block comments, 0 trailing commas; seed: 0 line comments, 0 block comments, 0 trailing commas; claude: 0 line comments, 0 block comments, 0 trailing commas; playgrounds: 0 line comments, 0 block comments, 0 trailing commas. The seed is JSONC by contract (the generator parses it with `Bun.JSONC.parse`) but contains no comment today; the launch file is plain JSON.

## 1. Result in numbers

- **4584 configurations**, all `node-terminal`/`launch` (99.1% carry `presentation`, 41 do not), 4 compounds, 82 inputs; names are unique (true); 12 groups of rows share command+env (25 rows, i.e. 13 surplus rows).
- **1867 rows (40.7%) are ticket-scoped**, all of them hand-written seed rows, all referencing only 8 tickets, **all of which are still `open`** (0 closed). One ticket (`CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`) owns 1451 of them.
- **663 rows are dead** (8 outside tickets, 655 ticket-scoped): 500 rows run a ticket-local Nx workspace whose directory was deleted, 6 non-ticket rows name an Nx target that no longer exists, 2 non-ticket rows name a moved/missing file. No row names a missing Nx *project*.
- **3423 rows are seed-authored, 314 are generated from the playground registry, 847 are generated from Nx project targets** (exact attribution in section 2; regenerating from seed + registry + project walk reproduces the launch file byte for byte: verified in this run).
- **1396 plain/picker rows are redundant with Nx target discovery** and **310 playground rows are redundant with the playground catalog**; together with the ticket and dead rows **3581 of 4584 rows (78.1%) are dropped**, 1003 rows carry intent worth keeping.
- Those 1003 rows reduce to a canonical registry of about **141 hand-maintained entries**: 101 declared presets (70 distinct base targets), 16 compounds (4 VS Code compounds + 12 `nx run-many` target groups), 12 global axes and 12 input prompts (section 11).

### Classes (mutually exclusive, first match wins)

| Class | Rows | Share | Definition |
| --- | --- | --- | --- |
| `ticket-scoped` | 1867 | 40.7% | Command, cwd, env or a used input default points into `🎫️tickets/…`, into a `🗑️generated` folder, or names a ticket-named Nx project. |
| `plain-nx` | 1353 | 29.5% | `bun nx run <project>:<target>` with fixed project and no semantic env, no flags, no arguments, no input, no ready action (noise env allowed). |
| `nx-preset` | 877 | 19.1% | `nx run P:T` with Nx flags, passthrough arguments, semantic env, an input or a ready action. |
| `playground-dev` | 326 | 7.1% | `workspace:dev -- <playground variant>` with a ready action (generated from the playground registry or a seed variant). |
| `nx-exec-bun-test` | 81 | 1.8% | `nx exec --projects=P -- bun test <file>`. |
| `project-picker-family` | 48 | 1.0% | Generated `nx run ${input:projectTarget.<target>}:<target>` row (one per target name declared by at least 3 projects). |
| `tool-launcher` | 27 | 0.6% | Not an `nx run`: `nx run-many`, a `bun` script, an external tool, `workspace:dev -- mcp …`. |
| `nx-exec-other` | 5 | 0.1% | `nx exec` running something other than `bun test`. |


### Surprises worth knowing before migrating

- Every ticket row is a **seed** row; the generator never produces one. `reconcile-launch-seed` keeps adopting anything a developer adds to `launch.json`, so the seed grows by whole ticket test-matrices (the dominant ticket alone contributes 1451 rows, 500 of them pointing at a workspace that no longer exists).
- 554 seed rows are plain `nx run P:T` rows the project-launcher walk would generate anyway; the generator even skips its own row when a seed row already runs that target.
- The Nx/VS Code workaround env (`NX_DAEMON`, `NX_ISOLATE_PLUGINS`, `NX_CACHE_PROJECT_GRAPH`, `NX_TUI`, `FORCE_COLOR`) sits on 1882 rows and is already defaulted by the `bun nx` wrapper (section 6); the ticket sandbox variables (`NX_WORKSPACE_ROOT_PATH`, …) sit on 1036 rows, **none** outside tickets.
- The cross-cutting axes have several spellings each: test level 3 (env var, positional after `--`, target-name suffix), cache policy 4 (two env vars, two Nx flags); section 5.1 lists them all.
- 34 non-ticket rows only work while another row's server runs (hub on 8787, `s` shell on 6070/6071) — they are compounds in disguise (section 5.4).
- All 155 playground ports match between registry, generated rows and `.claude/launch.json` (0 mismatches); the 24 `attach` entries there carry no command at all.

The coordinator's rough classes map as follows: *1788 ticket* -> `ticket-scoped` is 1867 here because 80+ rows reach a ticket only through an `${input:…Artifacts}` whose default is a ticket path (reasons: ticket-path+generated-dir 1119, ticket-path 512, ticket-path+ticket-named-project+generated-dir 153, ticket-input 80, generated-dir 2, ticket-path+ticket-input 1); *1447 plain* -> `plain-nx` 1353 plus 48 project-picker rows (ticket rows and rows with noise-only env are classified before/with the plain ones here); *318 playground launchers* -> `playground-dev` 326 (314 generated + 12 seed variants; 318 rows carry `SEMIO_PLUGIN`); *71 nx exec bun test* -> `nx-exec-bun-test` 81 (non-ticket; the 1470 ticket-scoped `nx exec` rows are counted as tickets).

## 2. The seed and the generator

Pipeline (`📇️registry/🚀️launch/🟦️.ts`, run by `generate` in `📇️registry/📽️projection/🟦️.ts`; `reconcile-launch-seed` and `check-launch-seed` are the other two entry points):

1. `readSeed` parses the seed with `Bun.JSONC.parse`, validates the container shapes with `🧱️placement` (configurations may only be rows or `@generated:<variant>:<kind>` strings, inputs only input objects; schema `🧬️schema/🧱️placement/🔣️.json`), splits off `devLaunchers` and `projectLaunchers`, and keeps the rest as the **skeleton**.
2. **devLaunchers** (variant -> `{namePrefix, order, wgpuOrder?, env?, users?}`): for each entry the `@generated:<variant>:react|wgpu|users` placeholder inside the skeleton is replaced by a row whose command (`bun nx run workspace:dev -- <variant>`), env (`S_OS_PORT`, `SEMIO_PLUGIN`, `SEMIO_RENDERER`, `SEMIO_APP`) and `serverReadyAction` come **only** from the playground registry entry; the seed contributes the display name prefix, the `order`, optional extra env and the multi-user template (`users`: one row per registry `userPorts` slot, tokens `{N}`, `{PORT}`, `{EMAIL}`). A seed row can never drift from the variant it launches.
3. `refreshDevLaunchNames` rewrites `🛠️dev…` names so their emoji match the plugin/artifact taxonomy folders (`🏷️name-prefix`: `<plugin folder><artifact folder>[<standard folder>]<subset folder>` + `⚛️react` | `🧊️wgpu🌐️wasm`; injective over variants, `launch-name-prefix` in a crate manifest overrides).
4. **Synthesis**: every registry variant that has no row of that name yet gets a `⚛️react` and a `🧊️wgpu🌐️wasm` row appended (order `420 + index*0.01` unless curated). Then `refreshDevLaunchNames` again.
5. **projectLaunchers** (policy in the seed; `declaredProjectTargets` walks every `📋️project.json`, skipping hidden dirs and `skipDirectories`): a target name declared by >= `familyMinimumProjects` projects becomes one **family row** `<class emoji><target>📋️` with a `${input:projectTarget.<target>}` pickString input (options = the owning projects); every other declared (project, target) not already run by some `nx run P:T` row gets a **target row** `<class emoji><target><project label>`. The class (`dev` 🛠️ / `build` 📦️ / `gate` ⚖️ / fallback `run` ▶️) is the first target-name token listed by a class; the label is the shortest unique trailing run of the project's path segments (language folders shortened to 🦀️/🟦️/🐍️).
6. The result is re-serialised with `JSON.stringify(…, null, 2)`. `reconcile-launch-seed` moves rows that exist only in `launch.json` into the seed as the bytes they were written in (this is how hand-added rows, including every ticket row, end up in the seed).

**Verification.** Re-running the real generator on the audited seed + playground registry + current project tree yields 4584 configurations and is **byte-identical** to the audited launch file; without project launchers it yields 3737 rows, exactly the boundary used below (generator walk: 564 projects, 3593 declared targets). The attribution below is structural (seed order + placeholder expansion + appended synthesized rows + appended project rows) and does not depend on that run.

| Provenance | Rows | How it is produced |
| --- | --- | --- |
| seed-authored | 3423 | 3423 object rows in the seed `configurations`, copied verbatim (names are rewritten only for `🛠️dev…` rows that map to a playground) |
| playground-seed-placeholder | 92 | 47 `@generated:<v>:react` + 41 `:wgpu` + 4 rows of 1 `:users` placeholder (template `🖥️s👤️{N}`, `user{N}@semio.dev`, env `S_HUB_URL` + `S_DATA_DIR`) |
| playground-synthesized | 222 | 108 registry variants without a seed entry x 2 renderers + 6 curated variants that lack a `wgpuOrder` (react-only seed entry): appended after the skeleton |
| project-launcher-family | 48 | 48 target names declared by >= 3 projects (covering 2249 of the 3593 declared (project, target) pairs) -> 48 generated `projectTarget.*` inputs |
| project-launcher-target | 799 | 3593 declared pairs - 2249 in family targets - 545 already run by an earlier row = 799 pending pairs |
| **total** | 4584 | seed items 3512 -> 3737 rows before project launchers -> 4584 |

Provenance x class (rows):

| Provenance | plain-nx | tool-launcher | nx-exec-bun-test | ticket-scoped | nx-preset | playground-dev | nx-exec-other | project-picker-family |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| seed-authored | 554 | 27 | 81 | 1867 | 877 | 12 | 5 |  |
| project-launcher-target | 799 |  |  |  |  |  |  |  |
| playground-synthesized |  |  |  |  |  | 222 |  |  |
| playground-seed-placeholder |  |  |  |  |  | 92 |  |  |
| project-launcher-family |  |  |  |  |  |  |  | 48 |

Duplicates: names are unique; 12 groups of rows share command+env (25 rows, 13 surplus):

| Command | Rows (names) | Classes |
| --- | --- | --- |
| `bun "nx" "exec" "--projects=workspace" "--excludeTaskDependencies" "--skip-nx-c…` | `🧪️test🏢️ifc📜️view-intent🟦️source` = `🧪️test🏢️ifc📜️native-view🟦️source` | nx-exec-bun-test |
| `bun nx run @semio-tech/framework-renderer-wgpu:native -- cad` | `🛠️dev📐️cad🧊️wgpu🖥️native` = `🛠️dev📐️cad🧩️concrete🌲️forest🧊️wg…` | nx-preset |
| `bun nx run @semio-tech/repo-coordinator:dev` | `▶️repo coordinator` = `🛠️dev🧰️repo🖥️coordinator🟦️typescr…` | plain-nx |
| `bun nx run @semio-tech/framework-plugin:test --skip-nx-cache --args="quick chil…` | `♻️ Typed Source · Original Owning Phy…` = `♻️ Typed Source · Original Source And…` | ticket-scoped |
| `bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d` | `🛠️dev🧩️puzzle🧊️3d🧊️wgpu🖥️native` = `🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️for…` | nx-preset |
| `bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d` | `🛠️dev🧩️puzzle🖐️5d🧊️wgpu🖥️native` = `🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️for…` | nx-preset |
| `bun nx run @semio-tech/framework-renderer-wgpu:native -- shooting` | `🛠️dev🎥️shooting🧊️wgpu🖥️native` = `🛠️dev📸️shooting🎛️base🧊️wgpu🖥️nat…` | nx-preset |
| `bun nx exec --projects=workspace --excludeTaskDependencies --skip-nx-cache -- b…` | `🧪️test🧩️puzzle2d📦️paged🟦️oracle` = `🧪️test🧩️puzzle2d🗑️cascade🔮️oracle` = `🧪️test🧩️puzzle2d🗑️cascade🔮️full-o…` | ticket-scoped |
| `bun nx exec --projects=workspace --excludeTaskDependencies --skip-nx-cache -- b…` | `🧪️test📜️history🦀️non-stdio-assembl…` = `🧪️test📜️history🎯️non-stdio🦀️curre…` | ticket-scoped |
| `bun nx run @semio-tech/framework-plugin:test --skip-nx-cache -- long history_ed…` | `🧪️test📜️history🧮️array🦀️controls` = `🧪️test📜️history🧺️array🦀️current` | nx-preset |
| `bun nx exec --projects=workspace --excludeTaskDependencies --skip-nx-cache -- b…` | `🧪️test🧩️puzzle2d🧬️inline🟦️current…` = `🧪️test🧩️puzzle2d🧬️inline🌳️current…` | nx-exec-bun-test |
| `bun nx exec --projects=workspace --excludeTaskDependencies --skip-nx-cache -- b…` | `🧪️test🧩️puzzle2d🌳️root📄️txt🦀️syn…` = `🧪️test🧩️puzzle2d🏞️region🚩️🦀️synt…` | ticket-scoped |

What the seed's metadata encodes:

- `devLaunchers`: 47 entries (of 155 registry variants; 108 variants have none), field frequency namePrefix 47, order 47, wgpuOrder 41, env 1, users 1; `order` 10..392.3; `env` extras on 1 entry, `users` on 1 entry (variant `s`). Per entry the seed owns only presentation (name prefix, order) — everything operational is registry-owned.
- `projectLaunchers`: 4 classes (dev 🛠️ -> 3_dev, base 900, 11 tokens; build 📦️ -> 4_build, base 900, 28 tokens; gate ⚖️ -> 4_gate, base 900, 26 tokens; run ▶️ -> 3_dev, base 950, 1 tokens), fallback `run`, `familyMinimumProjects` 3, family emoji 📋️, transparent segment `📦️packages`, language shortening `🦀️rust->🦀️, 🟦️typescript->🟦️, 🐍️python->🐍️`, skipDirectories `node_modules, dist, target, temp, pkg, storybook-static, 🤖️generated, 🗑️generated, 🎫️tickets`. Output rows land in groups 4_gate 644, 4_build 107, 3_dev 96.
- The seed's 3423 object rows are by class: ticket-scoped 1867, nx-preset 877, plain-nx 554, nx-exec-bun-test 81, tool-launcher 27, playground-dev 12, nx-exec-other 5; by `presentation.group`: 4_gate 2473, 9_gates 453, 3_dev 225, 4_build 153, (none) 41, 🧹clean🛡️gates 29, 🧿️ Semio Snapshot SQLite 18, 4_test 6, 🪶️ Snapshot SQLite 5, 0_dev 4, 2_mouse 4, 9_goal 3, repo-gate 3, 🪶️ Artifact Snapshot SQLite 2, 1_keyboard 2, 2_build 1, 9_clean_architecture 1. In particular **554 seed rows are plain `nx run P:T`** that the project-launcher walk would otherwise generate itself (they win through the `covered` check and keep hand-chosen names/orders), and all 1867 ticket rows live here.
- Project-launcher naming is fully derivable: recomputing `<class emoji><target><label>` from (project path, target) reproduces 799/799 target-row names, 799/799 orders and 799/799 groups, and 48/48 family names, 48/48 family orders.
- **Side finding:** `skipDirectories` contains `🎫️tickets`, which also matches the repository's own module folder `🔨️modules/🎫️tickets`; 2 real Nx projects (`@semio-tech/repo-tickets-go`, `@semio-tech/repo-tickets-rs`) therefore get no project-launcher row (the walk finds 564 projects, my manifest walk finds 566).

## 3. Do the commands still exist?

Method: every `nx run P:T[:cfg]` and `nx exec --projects=P` row is parsed with a shell tokenizer (quotes, inline `VAR=value` prefixes, `--` passthrough) and checked against three sources — (a) the declared manifests (walk of all `📋️project.json`/`project.json`: 568 files, 568 named projects with 3602 declared targets, plus 144 Cargo-only packages that the inference plugin turns into projects; walk of 82899 directories), (b) the cached Nx graph (`.nx/workspace-data/project-graph.json`, computed 2026-10-07T17:12:59.083Z, 1271 nodes; the older copy `.nx/workspace-data/project-graph.json` is from 2026-10-07T17:12:59.083Z but has identical node and target sets), (c) re-implemented inference rules of `📚️library/🟨️.mjs` (root `router.register` commands, Cargo `build|check|test`, component targets, leveled `test-quick|long|exhaustive`, print documents, playground-derived targets). Declared manifests explain every one of the graph's declared targets (0 declared targets missing from the graph) and rules + manifests explain all but 10 graph-only targets (`nx-release-publish` and two `test:*` colon targets), so the verdicts below do not depend on running Nx.

| Verdict (workspace) | Rows |
| --- | --- |
| ok-declared (main) | 3400 |
| ok-declared (ticket-local) | 538 |
| ticket-workspace-missing (ticket-local) | 500 |
| input-selected (main) | 48 |
| ok-inferred-rule (main) | 43 |
| missing-target (main) | 7 |

Rule-explained rows: leveled-test 24, playground-target 12, component-target 4, cargo-target 2, root-router-register 1. Ticket-local workspaces (`NX_WORKSPACE_ROOT_PATH` or a ticket cwd): 82 distinct roots, 82 still exist.

Path references: cwd, `NX_WORKSPACE_ROOT_PATH`, script and test-file arguments are *required* paths; cache/artifact/data env values are *outputs* and need not exist. Relative `nx run P:T -- ../x.ts` arguments are resolved against the project root first (that is Nx's `cwd`), then the workspace root. Missing required paths: inside a deleted 🗑️generated folder 1367, missing file 3.

**Dead rows: 663** = 8 non-ticket + 655 ticket-scoped (ticket-workspace-missing 500, missing-path 156, missing-target 7).

Non-ticket dead rows (8):

| # | Name | Command | Why dead |
| --- | --- | --- | --- |
| 446 | 📦️test🏃️process🪓️tree-termination | bun nx run @semio-tech/repo-lib:test-process-tree-termination | missing-target: @semio-tech/repo-lib:test-process-tree-termination |
| 611 | 🛠️dev🔺️trinity🃏️jack🦀️shell | bun nx run @semio-tech/trinity-jack-shell:run -- trinity/fixture/nakagin-capsule-tower.trinity.json "MATCH (a… | missing-path: arg:✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/trinity/fixture/nakagin-capsule-tower.trinity.json |
| 1127 | 🧹clean🧩️taxonomy🎯️draw-destination-observation | bun nx run @semio-tech/repo-lib:test-draw-destination-observation --skip-nx-cache | missing-target: @semio-tech/repo-lib:test-draw-destination-observation |
| 1393 | ⚖️test-engagement-status🚧️react🟦️ | bun nx run @semio-tech/ui-react:test -- 🧪️tests/📣️engagement-status/🟦️.tsx | missing-path: arg:🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🧪️tests/📣️engagement-status/🟦️.tsx |
| 1934 | ⚖️test-artifact-kind🧰️framework🦀️ | bun nx run @semio-tech/framework-rs:test-artifact-kind | missing-target: @semio-tech/framework-rs:test-artifact-kind |
| 1950 | 🧰️framework modules 🛍️product dependency direction | bun nx run @semio-tech/repo-lib:lint-framework-module-product-direction | missing-target: @semio-tech/repo-lib:lint-framework-module-product-direction |
| 2476 | ⚖️test-fixture-ownership🧰️framework🦀️ | bun nx run @semio-tech/framework-rs:test-fixture-ownership | missing-target: @semio-tech/framework-rs:test-fixture-ownership |
| 2481 | ⚖️test-snapshot-sqlite-io🧰️framework🦀️ | bun nx run @semio-tech/framework-rs:test-snapshot-sqlite-io | missing-target: @semio-tech/framework-rs:test-snapshot-sqlite-io |

Reading of the non-ticket causes: `@semio-tech/framework-rs` and `@semio-tech/repo-lib` renamed or removed their per-test targets (`test-fixture-ownership` -> `test-fixture-ownership-source`, `test-snapshot-sqlite-io` -> `test-snapshot-sqlite`, …), `framework-io-schema-rs:test-binding` no longer exists, the `ui-react` engagement-status test moved to `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📣️engagement-status/🟦️.tsx`, and the trinity shell row points to a fixture file that is not tracked anywhere. These eight go away with the rows; nothing needs migrating.

Ticket-scoped dead rows: 500 rows share one deleted ticket-local workspace (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/fixed-slot-fixture-owners/nx-publication-3` x500), the remaining 155 reference files that sit inside `🗑️generated` folders deleted at ticket cleanup (1367 required references) or a missing script (3).

## 4. Ticket-scoped rows (pollution)

1867 rows; reasons: ticket-path+generated-dir 1119, ticket-path 512, ticket-path+ticket-named-project+generated-dir 153, ticket-input 80, generated-dir 2, ticket-path+ticket-input 1. By provenance: seed-authored 1867 (every one is a hand-written seed row — `reconcile-launch-seed` adopts whatever a developer adds to `launch.json`). By shape: ticket/nx-exec-ticket-script 1464, ticket/nx-run-preset 367, ticket/bun-script 25, ticket/nx-exec 6, ticket/nx-other 5. By Nx entry point: direct-nx-js 1038, bun-nx 802, bun-script 25, bun-x-nx 2 (the 1038 `nx.js` rows bypass the `bun nx` wrapper, which is why they repeat `NX_DAEMON`/`NX_ISOLATE_PLUGINS`).

| Ticket | Title | Status | Rows | Rows whose required paths all exist | Rows with missing paths | Rows with deleted Nx workspace | Required refs (missing) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` | Clean Architecture Layering Enforcement | open | 1451 | 800 | 651 | 500 | 4962 (1365) |
| `26/05/30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY` | Assets Fixtures Separation | open | 154 | 154 | 0 | 0 | 308 (0) |
| `26/09/30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O` | Universal Artifact Snapshot SQLite I/O | open | 128 | 127 | 1 | 0 | 250 (1) |
| `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING` | Non-Destructive History Editing | open | 96 | 94 | 2 | 0 | 526 (2) |
| `26/09/23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT` | Build Semio Tech Play For CDN Deployment | open | 15 | 15 | 0 | 0 | 32 (0) |
| `26/06/08/PROCEDURAL-FEATURE-COMPLETE` | Procedural Feature Complete | open | 10 | 10 | 0 | 0 | 10 (0) |
| `26/09/17/WGPU-RENDERER-REACT-PARITY` | Wgpu Renderer React Parity | open | 7 | 7 | 0 | 0 | 7 (0) |
| `26/09/26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE` | Complete Stdio Artifact Editing Experience | open | 4 | 4 | 0 | 0 | 4 (0) |

Ticket status is read from each `🎫️ticket.json`: open 8; rows by roll-up status open 1865, no-ticket-reference 2. **No referenced ticket is closed**, so "closed ticket" cannot be used as the drop criterion here — the rows are still dropped because they are one-off verification probes bound to a ticket folder, not developer entry points (the open tickets can keep their commands in their own folder).

Rows flagged ticket-scoped without any ticket reference (they only run into `🗑️generated`): #412 `⚖️check🪶️hub-count-authenticated-lease🦀️`, #1189 `⚖️check🧫️host-count-component🦀️`.

Top shapes of the dominant ticket (examples): `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`: `🛠️os-common-successor-laws🧬️`, `🛠️os-common-successor-prepare🧬️`, `🛠️os-common-successor-metadata🧬️`; `26/05/30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY`: `🧪️test🦑️repo🧪️test🧫️fixture-isolation`, `🧫️fixtures-testing-only-schema-catalog-boundary🧪️`, `🧫️fixtures-testing-only-kernel-example-oracles🧪️`; `26/09/30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O`: `🧿️semio🪶️sqlite Paid Copy Cancellation Witness`, `🧿️semio🪶️sqlite Drawing Successful Empty Workspace Release Preparation`, `🧿️semio🪶️sqlite Drawing Successful Empty Workspace Release`.

## 5. Families and axes (non-ticket rows)

Rows that are neither plain nor generated pickers were grouped by *base target* (`project:target[:configuration]`, or the `nx exec`/tool entry point) and then by the dimensions along which rows of one base target differ. A dimension is **cross-cutting** when it appears on at least 3 distinct base targets, otherwise **target-specific**.

### 5.1 Cross-cutting axes

| Axis | Rows | Base targets | Owner projects | Expressed as | Value domain | Scope |
| --- | --- | --- | --- | --- | --- | --- |
| `cache-policy` | 705 | 369 | 144 | flag:--skip-nx-cache (676), env:NX_SKIP_NX_CACHE (128), env:NX_SKIP_REMOTE_CACHE (128), flag:--skip-remote-cache (99) | skip-local (577), skip-local+skip-remote (128) | global |
| `test-level` | 453 | 280 | 126 | env:SEMIO_TEST_LEVEL (251), positional (212), target-name (9) | quick (250), long (203) | global |
| `port` | 360 | 29 | 9 | env:S_OS_PORT (322), env:STORYBOOK_PORT (12), env:OS_HUB_PORT (7), env:PROCTOR_PORT (4), env:PUZZLE_5D_PLAY_PORT (4), env:CAD_JS_RENDERER_PLAY_PORT (2), env:PETS_STORIES_PORT (2), env:PUZZLE_3D_PLAY_PORT (2), env:SHOOTING_PLAY_PORT (2), env:TEACHING_ARCHITECTURE_QUIZ_PORT (2), env:CLIENT_PORT (1), env:MIT_BESTAND_DEMONSTRATOR_PORT (1), env:PRAESENTATION_PROJEKTETAGE_PORT (1), env:SEMIO_TECH_PLAY_PORT (1), env:SERVER_PORT (1) | 325 distinct values | global |
| `renderer` | 333 | 4 | 4 | env:SEMIO_RENDERER (333) | react (169), wgpu (164) | global |
| `test-filter` | 284 | 67 | 64 | passthru (284) | 274 distinct values | global |
| `task-dependencies` | 184 | 44 | 30 | flag:--excludeTaskDependencies (184) | excluded (184) | global |
| `test-scope` | 174 | 43 | 43 | passthru (174) | 10 distinct values | global |
| `build-mode` | 128 | 39 | 21 | env:SEMIO_BUILD_MODE (128) | ship (128) | global |
| `nextest-output` | 118 | 38 | 36 | env:NEXTEST_SUCCESS_OUTPUT (118) | immediate (118) | global |
| `print-phase` | 65 | 1 | 1 | env:PRINT_NATIVE_GRAMMAR_PHASE (65), env:PRINT_NATIVE_DIAGRAM_FAMILY (3), env:PRINT_NATIVE_GEO_PALETTE_PHASE (3), env:PRINT_NATIVE_GEO_PLANAR_PHASE (2), env:PRINT_NATIVE_GEO_PLANAR_SOURCE (1) | actual-mark-controls (1), biofabric-matrix-controls (1), critical-path-controls (1), density-controls (1), diagram-controls (1) | target-specific |
| `cargo-jobs` | 41 | 18 | 16 | env:CARGO_BUILD_JOBS (41) | 2 (30), 1 (11) | global |
| `test-files` | 30 | 7 | 6 | passthru (30) | 22 distinct values | global |
| `output-style` | 29 | 9 | 8 | flag:--output-style (27), flag:--outputStyle (2) | stream (24), static (5) | global |
| `build-budget` | 18 | 12 | 10 | env:SEMIO_BUILD_BUDGET_MS (18) | 1800000 (12), 3600000 (6) | global |
| `example` | 4 | 1 | 1 | env:SEMIO_DEFAULT_EXAMPLE (4), env:PLAYGROUND_LOCKED_EXAMPLE_ID (2) | capsule-dream (2), hexagonal-mushroom-column (2) | target-specific |
| `app-role` | 2 | 1 | 1 | env:SEMIO_APP_ROLE (2) | viewer (2) | target-specific |

What each axis means and where it should live:

- `cache-policy` — Skip local / remote Nx cache. Expressed as `--skip-nx-cache`, `--skip-remote-cache` and the env pair `NX_SKIP_NX_CACHE`/`NX_SKIP_REMOTE_CACHE` (the three forms are synonyms).
- `test-level` — quick | long | exhaustive. Expressed as `SEMIO_TEST_LEVEL`, a first positional after `--`, or the `test-<level>` target name.
- `port` — Dev-server port. Registry-owned for playgrounds (`S_OS_PORT`); owner-declared per service otherwise (`*_PORT`). Belongs next to the ready action, not in a row.
- `renderer` — react | wgpu (`SEMIO_RENDERER`), plus `wgpu-native` via the `native` target.
- `test-filter` — Free-text test selection (`--filter-expr`, `-E`, bare names, `--testNamePattern`). One-off by nature: a prompt, never stored.
- `task-dependencies` — `--excludeTaskDependencies`: run only the target, not its `dependsOn` chain.
- `test-scope` — cargo scope (`--lib`, `--test X`, `--features f`, …).
- `build-mode` — `SEMIO_BUILD_MODE=ship`; always together with the cache-policy pair.
- `nextest-output` — `NEXTEST_SUCCESS_OUTPUT` immediate|final (cargo-nextest's own variable).
- `print-phase` — `PRINT_NATIVE_*` phase/family selectors of one test target.
- `cargo-jobs` — `CARGO_BUILD_JOBS` cap (1|2), a load guard.
- `test-files` — Explicit test file paths (vitest).
- `output-style` — `--output-style=stream|static` / `--outputStyle=stream`.
- `build-budget` — `SEMIO_BUILD_BUDGET_MS` wall-clock budget (30 or 60 min).
- `example` — `SEMIO_DEFAULT_EXAMPLE` / `PLAYGROUND_LOCKED_EXAMPLE_ID`: initial or locked example of a playground.
- `app-role` — `SEMIO_APP_ROLE=viewer`.

The same axes also steer the ticket rows (ticket-row axis usage: task-dependencies 1484, cache-policy 421, artifact-dir 366, test-level 272, output-style 155, nextest-output 66, test-filter 18, test-scope 11). Two expressions per axis coexist today (test level has 3 forms, cache policy 4); a registry should store one canonical spelling and let each target map it (env var, flag or positional).

### 5.2 Families

| Family | Rows | Base targets | Owner projects | Varying dimensions | Constant | Ready action |
| --- | --- | --- | --- | --- | --- | --- |
| `playground-dev / playground-dev/registry-generated` | 310 | 1 | `workspace` | port[310], renderer[2] | project `workspace` | (http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT}) -> %s |
| `nx-preset / test-axis-variant` | 281 | 278 | 121 projects (`@semio-tech/framework-plugin:test`, `@semio-tech/framework-dsl-record-…`, …) | cache-policy[2], test-level[2], task-dependencies[1], build-mode[1], nextest-output[1], output-style[1] | - | - |
| `nx-preset / test-selection/filter` | 268 | 65 | 62 projects (`@semio-tech/framework-os-kernel:t…`, `@semio-tech/framework-plugin:test`, …) | test-filter[258], cache-policy[2], test-level[2], test-scope[6], nextest-output[1], task-dependencies[1] | - | - |
| `nx-exec-bun-test / bun-test-file` | 81 | 10 | 10 projects (`exec[workspace]:bun test`, `exec[@semio-tech/framework-os-ker…`, …) | task-dependencies[1], cache-policy[2], build-mode[1], nextest-output[1] | - | - |
| `nx-preset / axis-variant` | 66 | 66 | 45 projects (`@semio-tech/repo-cli-rs:install`, `@semio-tech/cad-extension-aec-bui…`, …) | cache-policy[2], test-level[1], build-mode[1], build-budget[2], task-dependencies[1], output-style[1] | - | - |
| `nx-preset / print-native-phase` | 65 | 1 | `@semio-tech/print` | print-phase[65], cache-policy[1] | project `@semio-tech/print` | - |
| `nx-preset / target-specific-arguments` | 63 | 37 | 13 projects (`workspace:verify`, `os-hub-ts:backend-run`, …) | port[2], task-dependencies[1], cache-policy[1], output-style[1], cargo-jobs[1] | - | - |
| `nx-preset / wgpu-native-launch` | 45 | 2 | `@semio-tech/framework-renderer-wgpu` | args[10+] | project `@semio-tech/framework-renderer-wgpu` | - |
| `nx-preset / test-selection/vitest-files` | 30 | 7 | 6 projects (`@semio-tech/framework-renderer-re…`, `@semio-tech/framework-os:test-qui…`, …) | test-files[22], test-level[2], test-filter[16], cache-policy[2], task-dependencies[1], build-mode[1] | - | - |
| `nx-preset / dev-server-port` | 24 | 23 | 9 projects (`@semio-tech/pets-react:dev`, `@semio-tech/framework-os-dev:serv…`, …) | port[9], renderer[1], cache-policy[1] | - | (http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT}) -> %s |
| `nx-preset / prompted-argument` | 21 | 16 | 5 projects (`@teaching/proctor:erase`, `@teaching/proctor:prune`, …) | args[10+] | - | - |
| `nx-preset / test-selection/scope` | 14 | 12 | 12 projects (`@semio-tech/stdio-plugin:test`, `@semio-tech/framework-dsl-record-…`, …) | test-scope[6], cache-policy[2], test-level[1], build-mode[1], task-dependencies[1], nextest-output[1] | - | - |
| `tool-launcher / nx-run-many` | 12 | 1 | `nx run-many (project set)` | cache-policy[2], build-mode[1], output-style[2], test-level[1], cargo-jobs[1] | - | - |
| `playground-dev / playground-dev/seed-variant` | 12 | 1 | `workspace` | port[11], renderer[2], example[1] | project `workspace` | (http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT}) -> %s |
| `tool-launcher / dev-tool/workspace-dev-mcp` | 9 | 1 | `workspace` | port[3], renderer[2], example[1], app-role[1] | project `workspace` | (http://[^\s]+:{PORT}) -> %s |
| `tool-launcher / external-tool` | 5 | 5 | 5 projects (`external:gemini`, `external:f3d`, …) | - | - | - |
| `nx-exec-other / nx-exec-other` | 5 | 3 | `@semio-tech/cad-cad-rs`, `workspace` | cache-policy[2], task-dependencies[1], build-mode[1] | - | - |
| `playground-dev / playground-dev/user-slot` | 4 | 1 | `workspace` | port[4], renderer[2] | project `workspace` | (http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT}) -> %s |
| `tool-launcher / bun-script` | 1 | 1 | `./📜️script.ts` | - | - | - |

#### `playground-dev / playground-dev/registry-generated` — 310 rows, 1 base target

Generated `workspace:dev -- <variant>` rows: two per registry variant (react, wgpu).

- Owners: `workspace`.
- Varying axes: port on 310 rows (env:S_OS_PORT; 310 values); renderer on 310 rows (env:SEMIO_RENDERER; react | wgpu).
- Argument = the playground variant id (155 distinct variants; extra tokens after it: none).
- Constant on every row: env none; args none.
- Target-specific env keys: S_DATA_DIR 2, S_HUB_URL 2.
- Ready action: `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s`.
- Examples: `🛠️dev📐️cad⚛️react`, `🛠️dev📐️cad🧊️wgpu🌐️wasm`, `🛠️dev🕸️dag⚛️react`.

#### `nx-preset / test-axis-variant` — 281 rows, 278 base targets

A `test*` target run with only cross-cutting axes (level, nextest output, cargo jobs, cache policy, ship build, task dependencies, output style). No selection.

- Owners: `@semio-tech/architect-program-rs`, `@semio-tech/block-2d-rs`, `@semio-tech/block-3d-rs`, `@semio-tech/block-5d-rs`, `@semio-tech/cad-cad-rs`, `@semio-tech/cad-extension-aec-building-rust`, +115 more.
- Varying axes: cache-policy on 241 rows (flag:--skip-nx-cache, env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE, flag:--skip-remote-cache; skip-local | skip-local+skip-remote); test-level on 205 rows (env:SEMIO_TEST_LEVEL, positional, target-name; quick | long); task-dependencies on 7 rows (flag:--excludeTaskDependencies; excluded); build-mode on 6 rows (env:SEMIO_BUILD_MODE; ship); nextest-output on 6 rows (env:NEXTEST_SUCCESS_OUTPUT; immediate); output-style on 3 rows (flag:--output-style; static); cargo-jobs on 1 row (env:CARGO_BUILD_JOBS; 1).
- Target-specific arguments (3 distinct sets): `--families-only` x1, `--filter-expr test(snapshot_io_prerequisite_flow_gesture_journal)` x1, `quick --lib part21_cohort_` x1.
- Constant on every row: env none; args none.
- Largest base targets: `@semio-tech/framework-plugin:test` x2, `@semio-tech/framework-dsl-record-rs:test-native` x2, `@semio-tech/framework-dsl-record-derive-rs:test-nat…` x2, `@semio-tech/cad-extension-aec-building-rust:test` x1, `@semio-tech/framework-process:test-cargo-driver` x1.
- Examples: `⚖️gate🏢️semio-tech🎡️play🏢️aec-building`, `⚖️gate🏢️semio-tech🎡️play📡️nextest-driver`, `⚖️gate🏢️semio-tech🎡️play🧬️child-history-source`.

#### `nx-preset / test-selection/filter` — 268 rows, 65 base targets

A `test*` target with a test-name filter (`--filter-expr`/`-E`/bare names/`--testNamePattern`): one-off reproduction of a single test or test group, plus the axes above.

- Owners: `@semio-tech/architect-program-rs`, `@semio-tech/cad-cad-rs`, `@semio-tech/draw-drawing-rs`, `@semio-tech/draw-js`, `@semio-tech/flow-extension-brep-rust`, `@semio-tech/flow-flow-rs`, +56 more.
- Varying axes: test-filter on 268 rows (passthru; 258 values); cache-policy on 229 rows (flag:--skip-nx-cache, env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE, flag:--skip-remote-cache; skip-local | skip-local+skip-remote); test-level on 193 rows (positional, env:SEMIO_TEST_LEVEL, target-name; long | quick); test-scope on 160 rows (passthru; 6 values); nextest-output on 106 rows (env:NEXTEST_SUCCESS_OUTPUT; immediate); task-dependencies on 78 rows (flag:--excludeTaskDependencies; excluded); build-mode on 72 rows (env:SEMIO_BUILD_MODE; ship); cargo-jobs on 37 rows (env:CARGO_BUILD_JOBS; 2 | 1).
- Target-specific arguments (3 distinct sets): `--disableConsoleIntercept` x1, `--exact` x1, `--test-name-pattern` x1.
- Constant on every row: env none; args none.
- Target-specific env keys: SEMIO_CARGO_PREPARATION_TIMING 1, SEMIO_DEBUG_CLOSE_PHASE 1.
- Largest base targets: `@semio-tech/framework-os-kernel:test` x39, `@semio-tech/framework-plugin:test` x30, `@semio-tech/puzzle-2d-rs:test` x30, `@semio-tech/draw-drawing-rs:test` x14, `semio-framework-os-infinite:test` x13.
- Examples: `⚖️test🎛️dashboard🧊️execution`, `⚖️gate🏢️semio-tech🎡️play🔤️descriptor-canonical`, `⚖️gate🏢️semio-tech🎡️play🏪️retained-genesis`.

#### `nx-exec-bun-test / bun-test-file` — 81 rows, 10 base targets

`nx exec --projects=P --excludeTaskDependencies --skip-nx-cache -- bun test <file>`: run one TypeScript test file with Nx's environment.

- Owners: `@semio-tech/framework-job-rs`, `@semio-tech/framework-os-kernel`, `@semio-tech/framework-plugin`, `@semio-tech/framework-value`, `@semio-tech/process-process3d-rs`, `@semio-tech/repo-lib`, +4 more.
- Varying axes: task-dependencies on 81 rows (flag:--excludeTaskDependencies; excluded); cache-policy on 81 rows (flag:--skip-nx-cache, env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE; skip-local | skip-local+skip-remote); build-mode on 26 rows (env:SEMIO_BUILD_MODE; ship); nextest-output on 3 rows (env:NEXTEST_SUCCESS_OUTPUT; immediate).
- Constant on every row: env none; args `--excludeTaskDependencies`.
- Largest base targets: `exec[workspace]:bun test` x56, `exec[@semio-tech/framework-os-kernel]:bun test` x10, `exec[@semio-tech/semio-tech-play]:bun test` x6, `exec[@semio-tech/framework-value]:bun test` x2, `exec[@semio-tech/framework-plugin]:bun test` x2.
- Examples: `🧪️test🏢️ifc📜️view-intent🟦️source`, `🧪️test📜️history🎯️ready-candidate🟦️source`, `🧪️test🔣️json🗝️member-key🟦️source`.

#### `nx-preset / axis-variant` — 66 rows, 66 base targets

A non-test target (build/describe/check/verify) run with only cross-cutting axes; mostly the `ship` build + both cache skips pair.

- Owners: `@semio-tech/architect-program-rs`, `@semio-tech/block-5d-rs`, `@semio-tech/cad-cad-rs`, `@semio-tech/cad-extension-aec-building-rust`, `@semio-tech/dag-dag`, `@semio-tech/demonstrator-playground`, +39 more.
- Varying axes: cache-policy on 58 rows (flag:--skip-nx-cache, env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE, flag:--skip-remote-cache; skip-local | skip-local+skip-remote); test-level on 23 rows (env:SEMIO_TEST_LEVEL; quick); build-mode on 8 rows (env:SEMIO_BUILD_MODE; ship); build-budget on 5 rows (env:SEMIO_BUILD_BUDGET_MS; 1800000 | 3600000); task-dependencies on 3 rows (flag:--excludeTaskDependencies; excluded); output-style on 2 rows (flag:--output-style, flag:--outputStyle; stream); port on 2 rows (env:PROCTOR_PORT; 1 values).
- Constant on every row: env none; args none.
- Largest base targets: `@semio-tech/repo-cli-rs:install` x1, `@semio-tech/cad-extension-aec-building-rust:describe` x1, `@semio-tech/demonstrator-plugin:component-dev` x1, `@semio-tech/semio-tech-play:build` x1, `@semio-tech/sourcing-curation-rs:verify-curation-do…` x1.
- Examples: `🛠️dev🎛️dashboard📦️install`, `📇️catalog🏢️semio-tech🎡️play🏢️aec-building`, `🛠️component🏢️semio-tech🎡️play🎪️demonstrator`.

#### `nx-preset / print-native-phase` — 65 rows, 1 base target

`@semio-tech/print:test-native-grammar` with one `PRINT_NATIVE_*` selector per row (and `--families-only` on some).

- Owners: `@semio-tech/print`.
- Varying axes: print-phase on 65 rows (env:PRINT_NATIVE_GRAMMAR_PHASE, env:PRINT_NATIVE_DIAGRAM_FAMILY, env:PRINT_NATIVE_GEO_PALETTE_PHASE, env:PRINT_NATIVE_GEO_PLANAR_PHASE, env:PRINT_NATIVE_GEO_PLANAR_SOURCE; actual-mark-controls | biofabric-matrix-controls | critical-path-controls | density-controls); cache-policy on 61 rows (flag:--skip-nx-cache; skip-local).
- Constant on every row: env none; args none.
- Examples: `⚖️test📓️print🎛️family-customization`, `⚖️test📓️print✨️scale-options`, `⚖️test📓️print🎨️geo-palette`.

#### `nx-preset / target-specific-arguments` — 63 rows, 37 base targets

Genuinely target-specific arguments: sub-commands (`daemon start|attach`), backend names (`sqlite|postgres|neo4j`), serve URLs for acceptance matrices, `--hub`/`--locale`/`--tag` options.

- Owners: `@semio-tech/draw-drawing-rs`, `@semio-tech/flow-extension-brep-rust`, `@semio-tech/framework-os-dev`, `@semio-tech/framework-os-kernel`, `@semio-tech/framework-renderer-wgpu`, `@semio-tech/procedural-generation3d-rs`, +7 more.
- Varying axes: port on 4 rows (env:OS_HUB_PORT, env:S_OS_PORT; 2 values); task-dependencies on 2 rows (flag:--excludeTaskDependencies; excluded); cache-policy on 2 rows (flag:--skip-nx-cache; skip-local); output-style on 1 row (flag:--outputStyle; stream); cargo-jobs on 1 row (env:CARGO_BUILD_JOBS; 1).
- Target-specific arguments (10+ distinct sets): `all` x4, `sqlite` x3, `--help` x2, `--packages all` x2, `neo4j` x2.
- Constant on every row: env none; args none.
- Target-specific env keys: OS_HUB_DATA 5, GOWORK 1, S_HUB_URL 1, S_OS_MCP_LIVE_LOCALE 1, SEMIO_REPO_IMPLEMENTATION 1.
- Largest base targets: `workspace:verify` x11, `os-hub-ts:backend-run` x6, `@semio-tech/repo-cli-rs:run` x3, `os-hub-ts:two-client-e2e` x3, `os-hub-ts:document-growth-e2e` x3.
- Examples: `🛠️dev🎛️dashboard🌀daemon▶️start`, `🛠️dev🎛️dashboard🌀daemon📎attach`, `🛠️dev🎛️dashboard🌳️command-tree`.

#### `nx-preset / wgpu-native-launch` — 45 rows, 2 base targets

`@semio-tech/framework-renderer-wgpu:native[-release] -- <playground variant>`: the wgpu-native renderer of a playground.

- Owners: `@semio-tech/framework-renderer-wgpu`.
- Argument = the playground variant id (40 distinct variants; extra tokens after it: none).
- Constant on every row: env none; args none.
- Largest base targets: `@semio-tech/framework-renderer-wgpu:native` x44, `@semio-tech/framework-renderer-wgpu:native-release` x1.
- Examples: `🛠️dev📐️cad🧊️wgpu🖥️native`, `🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🖥️native`, `🛠️dev🧊️wgpu🖥️native🚢️release`.

#### `nx-preset / test-selection/vitest-files` — 30 rows, 7 base targets

A `test*` target of a TypeScript package with explicit test file paths (vitest `--run <file>`), plus axes.

- Owners: `@semio-tech/framework-os`, `@semio-tech/framework-renderer-react`, `@semio-tech/plugin-registry`, `@semio-tech/s-2d-js`, `@semio-tech/semio-tech-play`, `@semio-tech/ui-react`.
- Varying axes: test-files on 30 rows (passthru; 22 values); test-level on 20 rows (positional, target-name; long | quick); test-filter on 16 rows (passthru; 16 values); cache-policy on 5 rows (flag:--skip-nx-cache, env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE, flag:--skip-remote-cache; skip-local | skip-local+skip-remote); task-dependencies on 3 rows (flag:--excludeTaskDependencies; excluded); build-mode on 1 row (env:SEMIO_BUILD_MODE; ship).
- Target-specific arguments (1 distinct sets): `--disableConsoleIntercept` x7.
- Constant on every row: env none; args none.
- Largest base targets: `@semio-tech/framework-renderer-react:test` x18, `@semio-tech/framework-os:test-quick` x4, `@semio-tech/ui-react:test` x3, `@semio-tech/framework-renderer-react:test-long` x2, `@semio-tech/semio-tech-play:test` x1.
- Examples: `🧪️test📜️history📈️replay-steps⚛️band`, `⚖️gate🏢️semio-tech🎡️play🆕️fresh-contract`, `🧪️test🖍️draw🧰️actions⚛️react`.

#### `nx-preset / dev-server-port` — 24 rows, 23 base targets

A dev server with a `*_PORT` env and a `serverReadyAction` that opens the printed URL.

- Owners: `@semio-tech/framework-os-dev`, `@semio-tech/mit-bestand-demonstrator`, `@semio-tech/mit-bestand-praesentation-proje…`, `@semio-tech/pets-react`, `@semio-tech/semio-tech-play`, `@teaching/architecture-quiz`, +3 more.
- Varying axes: port on 23 rows (env:STORYBOOK_PORT, env:OS_HUB_PORT, env:PETS_STORIES_PORT, env:PROCTOR_PORT, env:TEACHING_ARCHITECTURE_QUIZ_PORT, env:MIT_BESTAND_DEMONSTRATOR_PORT, env:PRAESENTATION_PROJEKTETAGE_PORT, env:S_OS_PORT, env:SEMIO_TECH_PLAY_PORT; 9 values); renderer on 3 rows (env:SEMIO_RENDERER; react); cache-policy on 1 row (flag:--skip-nx-cache; skip-local).
- Constant on every row: env none; args none.
- Target-specific env keys: OS_HUB_DATA 3, OS_HUB_URL 1, PETS_MENAGERIE 1.
- Ready action: `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s`; `(http://(?:127\.0\.0\.1|localhost):{PORT}) -> %s`; `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s/admin`.
- Largest base targets: `@semio-tech/pets-react:dev` x2, `@semio-tech/framework-os-dev:serve-generation3d-rea…` x1, `@semio-tech/mit-bestand-praesentation-projektetage:…` x1, `@semio-tech/mit-bestand-demonstrator:dev` x1, `@semio-tech/semio-tech-play:dev` x1.
- Examples: `🛠️serve🌀️procedural🧊️generation3d⚛️react`, `🛠️dev📽️projektetage`, `🛠️dev♻️mit-bestand🧺️demonstrator`.

#### `nx-preset / prompted-argument` — 21 rows, 16 base targets

Arguments that are `${input:…}` prompts (acceptance URLs, proctor handle/age/backup file).

- Owners: `@semio-tech/framework-os-dev`, `@semio-tech/repo-test-domain`, `@semio-tech/s-services-native`, `@teaching/proctor`, `os-hub-ts`.
- Target-specific arguments (10+ distinct sets): `${input:acceptanceServeUrl}` x4, `--hub ${input:acceptanceHubUrl}` x2, `--handle ${input:proctorEraseHandle}` x1, `--handle ${input:proctorEraseHandle} --dry-run` x1, `--hub ${input:acceptanceHubUrl} --locale en` x1.
- Constant on every row: env none; args none.
- Target-specific env keys: OS_MCP_HUB_ORIGIN 4, S_OS_MCP_LIVE_LOCALE 2, S_OS_MCP_LIVE_SHELL_URL 2, OS_HUB_ADMIN_CAPABILITY_FILE 1.
- Largest base targets: `@teaching/proctor:erase` x2, `@teaching/proctor:prune` x2, `@semio-tech/framework-os-dev:hub-document-sweep` x2, `@semio-tech/framework-os-dev:two-human` x2, `@semio-tech/s-services-native:user-path-check` x2.
- Examples: `♻️restore🎓️teaching🛂️proctor`, `🧨️erase🎓️teaching🛂️proctor🔍️dry-run`, `🧨️erase🎓️teaching🛂️proctor`.

#### `nx-preset / test-selection/scope` — 14 rows, 12 base targets

A `test*` target with a cargo scope (`--lib`, `--test X`, `--features f`), no filter, plus axes.

- Owners: `@semio-tech/cad-cad-rs`, `@semio-tech/flow-flow-rs`, `@semio-tech/framework-dsl-record-rs`, `@semio-tech/gis-gismap-rs`, `@semio-tech/process-process3d-rs`, `@semio-tech/sourcing-curation-rs`, +6 more.
- Varying axes: test-scope on 14 rows (passthru; 6 values); cache-policy on 11 rows (flag:--skip-nx-cache, env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE, flag:--skip-remote-cache; skip-local+skip-remote | skip-local); test-level on 11 rows (positional, env:SEMIO_TEST_LEVEL; long); build-mode on 7 rows (env:SEMIO_BUILD_MODE; ship); task-dependencies on 5 rows (flag:--excludeTaskDependencies; excluded); nextest-output on 3 rows (env:NEXTEST_SUCCESS_OUTPUT; immediate); cargo-jobs on 1 row (env:CARGO_BUILD_JOBS; 2).
- Constant on every row: env none; args none.
- Largest base targets: `@semio-tech/stdio-plugin:test` x2, `@semio-tech/framework-dsl-record-rs:test-native` x2, `@semio-tech/value-derive-rs:test` x1, `@semio-tech/cad-cad-rs:test` x1, `@semio-tech/sourcing-curation-rs:test` x1.
- Examples: `⚖️gate🏢️semio-tech🎡️play🧬️empty-enum`, `⚖️gate🏢️semio-tech🎡️play📐️cad-native`, `⚖️gate🏢️semio-tech🎡️play🗂️curation-native`.

#### `tool-launcher / nx-run-many` — 12 rows, 1 base target

`nx run-many -t <target> -p <projects>` or `--target=<t> --args=…`: one target over a project set.

- Owners: `nx run-many (project set)`.
- Varying axes: cache-policy on 11 rows (flag:--skip-nx-cache, env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE, flag:--skip-remote-cache; skip-local | skip-local+skip-remote); build-mode on 5 rows (env:SEMIO_BUILD_MODE; ship); output-style on 4 rows (flag:--output-style; static | stream); test-level on 1 row (env:SEMIO_TEST_LEVEL; quick); cargo-jobs on 1 row (env:CARGO_BUILD_JOBS; 2).
- Target-specific arguments (9 distinct sets): `-t describe` x3, `--target=test --args=--lib --features component-app-assembly ordinary…` x1, `--target=test --args=the_transform_chart_obeys_the_shared_phase_fixtu…` x1, `--target=test --features component-app-assembly --lib editor` x1, `--target=test-snapshot-sqlite-native` x1.
- Constant on every row: env none; args none.
- Examples: `📇️catalog🏢️semio-tech🎡️play🏭️extensions`, `📇️catalog🏢️semio-tech🎡️play🪵️extensions`, `📇️catalog🏢️semio-tech🎡️play🌐️extensions`.

#### `playground-dev / playground-dev/seed-variant` — 12 rows, 1 base target

Seed variants of a playground: fixture argument, locked example, viewer role, per-app legacy port variable.

- Owners: `workspace`.
- Varying axes: port on 12 rows (env:PUZZLE_5D_PLAY_PORT, env:CAD_JS_RENDERER_PLAY_PORT, env:PUZZLE_3D_PLAY_PORT, env:S_OS_PORT, env:SHOOTING_PLAY_PORT; 11 values); renderer on 12 rows (env:SEMIO_RENDERER; react | wgpu); example on 2 rows (env:PLAYGROUND_LOCKED_EXAMPLE_ID, env:SEMIO_DEFAULT_EXAMPLE; capsule-dream).
- Argument = the playground variant id (5 distinct variants; extra tokens after it: `fixture concrete`, `fixture base-icon`, `served`).
- Constant on every row: env none; args none.
- Target-specific env keys: S_LOCAL_ONLY 1.
- Ready action: `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s`; `(http://(?:127\.0\.0\.1|localhost):{PORT}) -> %s`.
- Examples: `🛠️dev📐️cad🧩️concrete🌲️forest⚛️react`, `🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🌐️wasm`, `🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest⚛️react`.

#### `tool-launcher / dev-tool/workspace-dev-mcp` — 9 rows, 1 base target

`workspace:dev -- mcp …` (MCP inspector, repo MCP proxy, os-mcp stdio/http).

- Owners: `workspace`.
- Varying axes: port on 5 rows (env:S_OS_PORT, env:CLIENT_PORT, env:SERVER_PORT; 3 values); renderer on 4 rows (env:SEMIO_RENDERER; react | wgpu); example on 2 rows (env:SEMIO_DEFAULT_EXAMPLE; hexagonal-mushroom-column); app-role on 2 rows (env:SEMIO_APP_ROLE; viewer).
- Target-specific arguments (6 distinct sets): `procedural 3d` x4, `mcp` x1, `mcp http os` x1, `mcp repo` x1, `mcp stdio cursor` x1.
- Constant on every row: env none; args none.
- Target-specific env keys: MCP_AUTO_OPEN_ENABLED 1, MCP_PROXY_AUTH_TOKEN 1.
- Ready action: `(http://[^\s]+:{PORT}) -> %s`; `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s`; `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s/?plugin=generation3d`; `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s/?plugin=generation3d&role=vie…`.
- Examples: `🖱️mcpinspector`, `🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️r…`, `🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️…`.

#### `tool-launcher / external-tool` — 5 rows, 5 base targets

External programs: gemini, kiro-cli, f3d, gitkraken, MCP inspector.

- Owners: `@modelcontextprotocol/inspector`, `f3d`, `gemini`, `gitkraken`, `kiro-cli`.
- Constant on every row: env none; args none.
- Largest base targets: `external:gemini` x1, `external:f3d` x1, `external:kiro-cli` x1, `external:gitkraken` x1, `bun-x:@modelcontextprotocol/inspector` x1.
- Examples: `⌨️gemini`, `🖱️f3d`, `⌨️kiro`.

#### `nx-exec-other / nx-exec-other` — 5 rows, 3 base targets

`nx exec` around another command (ship+cache axes on a bun script).

- Owners: `@semio-tech/cad-cad-rs`, `workspace`.
- Varying axes: cache-policy on 5 rows (env:NX_SKIP_NX_CACHE, env:NX_SKIP_REMOTE_CACHE, flag:--skip-nx-cache; skip-local+skip-remote | skip-local); task-dependencies on 5 rows (flag:--excludeTaskDependencies; excluded); build-mode on 3 rows (env:SEMIO_BUILD_MODE; ship).
- Constant on every row: env none; args `--excludeTaskDependencies`.
- Largest base targets: `exec[@semio-tech/cad-cad-rs]:bun ${workspaceFolder}…` x3, `exec[workspace]:bun ${workspaceFolder}/✏️s/🔌️plugi…` x1, `exec[workspace]:bun ${workspaceFolder}/🧰️framework…` x1.
- Examples: `⚖️gate🏢️semio-tech🎡️play📐️cad-sqlite-source-body`, `⚖️gate🏢️semio-tech🎡️play📐️cad-document-source-body`, `⚖️gate🏢️semio-tech🎡️play📐️cad-sqlite-types-source-body`.

#### `playground-dev / playground-dev/user-slot` — 4 rows, 1 base target

Multi-user rows of the `s` shell: slot N of the registry `userPorts` with `S_HUB_URL` and `S_DATA_DIR`.

- Owners: `workspace`.
- Varying axes: port on 4 rows (env:S_OS_PORT; 4 values); renderer on 4 rows (env:SEMIO_RENDERER; react | wgpu).
- Argument = the playground variant id (1 distinct variants; extra tokens after it: none).
- Constant on every row: env `SEMIO_PLUGIN=s`, `S_HUB_URL=http://127.0.0.1:8787`; args `s`.
- Target-specific env keys: S_DATA_DIR 4, S_HUB_URL 4.
- Ready action: `(http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):{PORT}) -> %s`.
- Examples: `🛠️dev🪐️space👤️1⚛️react`, `🛠️dev🪐️space👤️2⚛️react`, `🛠️dev🪐️space👤️1🧊️wgpu🌐️wasm`.

#### `tool-launcher / bun-script` — 1 rows, 1 base target

A `bun ./📜️script.ts dev mcp stdio client` row.

- Owners: `./📜️script.ts`.
- Constant on every row: env none; args none.
- Examples: `🦑️mcp dev`.

### 5.3 Genuinely target-specific argument sets (base targets with more than one argument set)

| Base target | Rows | Argument sets (domain) |
| --- | --- | --- |
| `@semio-tech/framework-renderer-wgpu:native` | 45 | `cad`, `puzzle3d`, `puzzle5d`, `shooting`, `animate`, `architect`, … |
| `workspace:verify` | 11 | `composed-child-refs`, `dependencies`, `dependencies literal-external`, `docstrings emoji-unique`, `history-closure`, `interactivity`, … |
| `os-hub-ts:backend-run` | 6 | `neo4j -- bun nx run @semio-tech/framework-os-kern…`, `neo4j -- bun nx run @semio-tech/framework-os-kern…`, `neo4j -- bun nx run os-hub:directory-live-lanes -…`, `postgres -- bun nx run @semio-tech/framework-os-k…`, `postgres -- bun nx run @semio-tech/framework-os-k…`, `postgres -- bun nx run os-hub:directory-live-lane…` |
| `@semio-tech/repo-cli-rs:run` | 3 | `--help`, `command-tree --dump-tree` |
| `os-hub-ts:two-client-e2e` | 3 | `neo4j`, `postgres`, `sqlite` |
| `os-hub-ts:document-growth-e2e` | 3 | `neo4j`, `postgres`, `sqlite` |
| `@semio-tech/repo-cli-rs:daemon` | 2 | `attach`, `start` |
| `@teaching/proctor:erase` | 2 | `--handle ${input:proctorEraseHandle}`, `--handle ${input:proctorEraseHandle} --dry-run` |
| `@teaching/proctor:prune` | 2 | `--older-than ${input:proctorPruneAge}`, `--older-than ${input:proctorPruneAge} --dry-run` |
| `@semio-tech/framework-os-dev:program-matrix` | 2 | `--serve http://127.0.0.1:6070/ --tag launch-de --…`, `--serve http://127.0.0.1:6070/ --tag launch-en --…` |
| `@semio-tech/framework-os-dev:tool-run-matrix` | 2 | `--serve http://127.0.0.1:6070/ --tag launch-de --…`, `--serve http://127.0.0.1:6070/ --tag launch-en --…` |
| `@semio-tech/framework-os-dev:hub-document-sweep` | 2 | `--serve http://127.0.0.1:6071/ --hub ${input:acce…`, `--serve http://127.0.0.1:6071/ --hub ${input:acce…` |
| `@semio-tech/framework-os-dev:io-matrix` | 2 | `--serve http://127.0.0.1:6070/ --locale de`, `--serve http://127.0.0.1:6070/ --locale en` |
| `@semio-tech/framework-os-dev:two-human` | 2 | `--hub ${input:acceptanceHubUrl} --serve ${input:a…`, `--hub ${input:acceptanceHubUrl} --serve ${input:a…` |
| `@semio-tech/framework-os-dev:time-travel` | 2 | `--serve http://127.0.0.1:6012/ --renderer react -…`, `--serve http://127.0.0.1:6112/ --renderer wgpu --…` |

Examples of the pattern the coordinator named: `@semio-tech/repo-cli-rs:run` -> `--help`|`command-tree --dump-tree`; `@semio-tech/repo-cli-rs:daemon` -> `attach`|`start`; `@semio-tech/repo-cli-rs:test` -> ; `@semio-tech/repo-cli-rs:install` -> ; `@semio-tech/repo-cli-rs:preferences` -> `show`; `@semio-tech/repo-cli-rs:repo` -> `--help`.

### 5.4 Rows that depend on a running server

34 non-ticket rows contain a `http://127.0.0.1:<port>` (command, env or input default) that is **not** the port they serve themselves: 8787 (served by 🛠️dev🗄️os-hub +2) x13; 6070 (served by 🛠️dev🪐️space⚛️react +2) x12; 8787 (served by 🛠️dev🗄️os-hub +2) + 6070 (served by 🛠️dev🪐️space⚛️react +2) x5; 6071 (served by no launch row) + 8787 (served by 🛠️dev🗄️os-hub +2) x2; 6012 (served by 🛠️dev🧩️puzzle◻️2d⚛️react) x1; 6112 (served by 🛠️dev🧩️puzzle◻️2d🧊️wgpu🌐️…) x1. Examples: `🛠️dev🪐️space⚛️react`, `🛠️dev🪐️space🧊️wgpu🌐️wasm`, `🛠️dev🪐️space👤️1⚛️react`, `🛠️dev🪐️space👤️2⚛️react`. These are the rows that need compound semantics (start a serve, wait for its ready URL, then run): the hub on 8787 and the `s` React shell on 6070/6071 are the hard dependencies.

## 6. Environment variables: noise versus meaning

75 distinct keys (2562 rows carry env; inline `VAR=value` command prefixes are counted too). Kinds: port 14, service-config 12, target-config 12, global-axis 8, output-location 6, runner-noise 6, toolchain 6, unread-probe-knob 4, playground-binding 3, ticket-sandbox 3, secret-like 1. Rows carrying only Nx/VS Code noise env: 132 (78 outside tickets). Rows carrying a ticket sandbox (`NX_WORKSPACE_ROOT_PATH`/`NX_WORKSPACE_DATA_DIRECTORY`/`NX_CACHE_DIRECTORY`): 1036 (0 outside tickets). Readers were searched in 73770 tracked code/config files (`git ls-files`, excluding tickets, `.vscode`, `.claude`, `🗑️generated`) with read-site patterns (`process.env.X`, `env::var("X")`, …); Nx-owned variables were additionally searched in the installed Nx (`.nx/installation/node_modules/nx/dist`). The wrapper behind `bun nx` (root `package.json` script -> `⚡️caching/🚀️bootstrap/📜️script.ts`) already defaults `NX_DAEMON=false` (`nxChildEnvironment`), `NX_ISOLATE_PLUGINS=false` and `NX_WORKSPACE_DATA_DIRECTORY=.nx/workspace-data` (`run`), and `devToolingEnv()` (`🏃️process/🌿️environment/🟦️.ts`) defaults `NX_TUI=false`; `repoToolCacheEnv()` defaults `PLAYWRIGHT_BROWSERS_PATH`.

| Variable | Rows (non-ticket / ticket) | .claude | Top values | Judgement | Read by (files reading / mentioning) |
| --- | --- | --- | --- | --- | --- |
| `NX_DAEMON` | 1861 (515 / 1346) | 3 | false (1861) | **runner-noise** — Nx daemon off; the `bun nx` wrapper already defaults it to "false" (bootstrap `nxChildEnvironment`), only the rows that call `nx.js` directly need it. Drop. | Nx: bin/nx.js; 3 / 33: ⚡️caching/🚀️bootstrap/📜️script.ts |
| `NX_ISOLATE_PLUGINS` | 1055 (234 / 821) | 2 | false (1055) | **runner-noise** — Wrapper sets `NX_ISOLATE_PLUGINS ?? "false"` itself (bootstrap `run`). Drop. | Nx: src/command-line/init/init-v2.js; 2 / 8: ⚡️caching/🚀️bootstrap/📜️script.ts |
| `NX_WORKSPACE_ROOT_PATH` | 885 (0 / 885) |  | ${workspaceFolder}/<ticket-path> (885) | **ticket-sandbox** — Points Nx at a ticket-local workspace (own nx.json). Exists only to run ticket probes; drops with the ticket rows. | Nx: src/utils/workspace-root.js; 0 / 15 |
| `NX_WORKSPACE_DATA_DIRECTORY` | 531 (0 / 531) |  | ${workspaceFolder}/<ticket-path> (531) | **ticket-sandbox** — Private Nx graph/daemon state per ticket run. Drops with the ticket rows; a global `private nx state` axis is the dashboard's job (it already isolates graph state). | Nx: src/utils/cache-directory.js; 7 / 33: ⚡️caching/🚀️bootstrap/📜️script.ts |
| `SEMIO_TEST_LEVEL` | 506 (251 / 255) | 4 | quick (319); long (179) | **global-axis** — Test level quick\|long\|exhaustive; read by the shared test harness (`🧪️tests/🎚️config` files). Same axis as the positional `quick\|long\|exhaustive` after `--` and the `test-quick\|test-long\|test-exhaustive` targets. | 22 / 37: 🧪️tests/🎚️config/🟦️.ts |
| `NX_CACHE_DIRECTORY` | 382 (0 / 382) |  | ${workspaceFolder}/<ticket-path> (382) | **ticket-sandbox** — Private Nx cache per ticket run. Drops with the ticket rows. | Nx: src/utils/cache-directory.js; 1 / 19: 📦️site/🆕️fresh-build/🟦️.ts |
| `SEMIO_TEST_ARTIFACT_DIR` | 359 (0 / 359) | 2 | ${workspaceFolder}/<ticket-path> (288); ${workspaceFolder}/${input:processContractA… | **output-location** — Where a test writes its receipts (read by the shared test harness). Every row that sets it is ticket-scoped: a literal ticket path, or an `${input:…Artifacts}` whose default is a ticket path. An output-location concern for the dashboard's run directory, not a per-row string. | 194 / 207: 🧪️tests/🗿️artifact-surface/🦀️.rs |
| `SEMIO_RENDERER` | 333 (333 / 0) | 14 | react (169); wgpu (164) | **global-axis** — Renderer react\|wgpu; read by the dev server/builder. Registry-bound for playgrounds. | 13 / 42: 📦️packages/🦀️rust/📜️script.ts |
| `S_OS_PORT` | 322 (322 / 0) | 14 | 6018 (4); 6070 (3) | **playground-binding** — The only port variable any playground dev server binds (generator `playgroundDevEnv`); value = registry `ports.react\|wgpu` or `userPorts`. | 7 / 29: 📦️packages/🦀️rust/📜️script.ts |
| `SEMIO_PLUGIN` | 318 (318 / 0) |  | s (9); generation3d (3) | **playground-binding** — Playground variant id; fully determined by the playground registry entry (generator `playgroundDevEnv`). | 14 / 46: 📦️packages/🦀️rust/📜️script.ts |
| `NX_CACHE_PROJECT_GRAPH` | 302 (55 / 247) | 1 | false (300); true (2) | **runner-noise** — Nx graph-cache switch; the repo never sets or reads it, only Nx does. Rows pin it to false (twice to true). Drop; the dashboard owns its graph policy (`NX_FORCE_REUSE_CACHED_GRAPH`, see build-wait-investigation.md). | Nx: src/utils/plugin-cache-utils.js; 0 / 0 |
| `SEMIO_APP` | 289 (289 / 0) |  | s.procedural.generation3d@1/*#editor (5); s.cad.cad@1/*#editor (4) | **playground-binding** — Pinned app coordinate; fully determined by the playground registry entry. | 0 / 1 |
| `NEXTEST_SUCCESS_OUTPUT` | 184 (118 / 66) |  | immediate (163); final (21) | **global-axis** — cargo-nextest success output (`immediate\|final`); read by nextest itself, not by repo code. | 0 / 2 |
| `FORCE_COLOR` | 153 (0 / 153) |  | 0 (153) | **runner-noise** — Colour off for the VS Code terminal / captured logs; read by Nx and Node. A dashboard that renders ANSI does not need it. Drop. | Nx: bin/nx.js; 1 / 8: 🧪️tests/🔒️trunk-lockfile/🟦️.ts |
| `NX_TUI` | 139 (128 / 11) |  | false (139) | **runner-noise** — Nx terminal UI off; `devToolingEnv()` already does `NX_TUI ??= "false"`. Drop. | Nx: src/command-line/release/publish.js; 1 / 16: 🏃️process/🌿️environment/🟦️.ts |
| `NX_SKIP_NX_CACHE` | 136 (128 / 8) |  | true (136) | **global-axis** — Cache policy axis (`skip local cache`); same meaning as `--skip-nx-cache`. Read by Nx (`command-line-utils`). | Nx: src/utils/command-line-utils.js; 2 / 8: 🧪️tests/📦️native-dependencies/🟦️.ts |
| `NX_SKIP_REMOTE_CACHE` | 136 (128 / 8) |  | true (136) | **global-axis** — Cache policy axis (`skip remote cache`); same meaning as `--skip-remote-cache`. | Nx: src/utils/command-line-utils.js; 0 / 5 |
| `SEMIO_BUILD_MODE` | 136 (128 / 8) |  | ship (136) | **global-axis** — Build mode `ship` (release-like wasm/site builds); read by the wgpu server config and the wasm build scripts. Always set together with both cache-skip variables. | 7 / 21: 🌐️server/🎚️config/🟦️.ts |
| `PRINT_NATIVE_GRAMMAR_PHASE` | 65 (65 / 0) |  | diagram-controls (4); geo-palette (4) | **target-config** — Phase selector of the native chart-grammar test (`@semio-tech/print:test-native-grammar`); the values are the owner's own phase list. | 1 / 1: 🧪️tests/🧬️native-chart-grammar/🟦️.ts |
| `CARGO_BUILD_JOBS` | 44 (41 / 3) |  | 2 (32); 1 (12) | **global-axis** — Cargo parallelism cap (1\|2) used as a machine-load guard; read by cargo. | 1 / 10: ⚖️parity/🌐️server-pool/🟦️.ts |
| `SEMIO_TEST_ARTIFACTS_DIR` | 26 (0 / 26) |  | ${workspaceFolder}/<ticket-path> (22); ${workspaceFolder}/${input:graphContractArt… | **output-location** — Plural spelling of the same variable (a few files read it, only ticket rows set it): inconsistent name. | 4 / 4: 🧪️tests/🚦️cohort/🦀️.rs |
| `SEMIO_BUILD_BUDGET_MS` | 18 (18 / 0) |  | 1800000 (12); 3600000 (6) | **global-axis** — Per-build wall-clock budget (30 min / 60 min); read by the build scripts. | 11 / 17: 📦️packages/🦀️rust/📜️script.ts |
| `SEMIO_EARLY_SEED_CONTROL` | 12 (0 / 12) |  | red (12) | **unread-probe-knob** — Ticket probe knob; no reader outside the ticket scripts. | 0 / 0 |
| `STORYBOOK_PORT` | 12 (12 / 0) |  | 6010 (12) | **port** — Storybook dev server port (6010/6011); read by the storybook runner. | 2 / 6: 🧪️tests/🧪️browser-runner/🟦️.ts |
| `OS_HUB_DATA` | 8 (8 / 0) |  | ${workspaceFolder}/.🧬semio/🌐hub/hub-dev/ …; ${workspaceFolder}/.🧬semio/🌐hub/hub-dev (… | **service-config** — Hub data directory (per backend); read by the hub bootstrap. | 6 / 24: 🌎️hub/🏗️bootstrap/🦀️.rs |
| `OS_HUB_PORT` | 7 (7 / 0) |  | 8787 (7) | **port** — Hub port 8787; read by the hub bootstrap (Rust). | 1 / 9: 🌎️hub/🏗️bootstrap/🦀️.rs |
| `S_HUB_URL` | 7 (7 / 0) |  | http://127.0.0.1:8787 (7) | **service-config** — Hub URL the `s` shell joins; part of the hub+shell compounds and the multi-user rows. | 8 / 25: 📦️packages/🦀️rust/📜️script.ts |
| `S_DATA_DIR` | 6 (6 / 0) |  | ${workspaceFolder}/.🧬semio/🔗space/s-dev (…; ${workspaceFolder}/.🧬semio/🔗space/s-user1… | **service-config** — Per-user local data directory of the `s` shell (user slots). | 3 / 15: 📇️directory/🪪️identity/🦀️.rs |
| `SEMIO_EARLY_SEED_RED_ATTEMPT` | 6 (0 / 6) |  | 2 (6) | **unread-probe-knob** — Ticket probe knob; no reader outside the ticket scripts. | 0 / 0 |
| `CARGO_BUILD_BUILD_DIR` | 4 (0 / 4) |  | ${workspaceFolder}/<ticket-path> (4) | **toolchain** — Private cargo build dir (ticket probes only). | 6 / 29: 🧪️tests/🧩️neutral-owner/🟦️.ts |
| `CARGO_TARGET_DIR` | 4 (0 / 4) |  | ${workspaceFolder}/<ticket-path> (4) | **toolchain** — Private cargo target dir (ticket probes only); read by cargo and by repo scripts. | 12 / 55: 📦️packages/🦀️rust/📜️script.ts |
| `OS_MCP_HUB_ORIGIN` | 4 (4 / 0) |  | ${input:acceptanceHubUrl} (4) | **service-config** — Hub origin for the os-mcp acceptance gates (bound to `${input:acceptanceHubUrl}`). | 5 / 8: 🧪️tests/💼️inference-quartet/🟦️.ts |
| `PROCTOR_PORT` | 4 (4 / 0) | 6 | 8791 (4) | **port** — Proctor port 8791 (8793 for the second instance). | 3 / 15: 🏗️builder/🌐️vite/🟦️.ts |
| `PUZZLE_5D_PLAY_PORT` | 4 (4 / 0) |  | 6014 (1); 6015 (1) | **port** — Legacy per-app port variable (not read in tracked code): dead knob. | 0 / 0 |
| `SEMIO_ALIAS_PLAN_CONTROL` | 4 (0 / 4) |  | red (4) | **unread-probe-knob** — Ticket probe knob; no reader outside the ticket scripts. | 0 / 0 |
| `SEMIO_DEFAULT_EXAMPLE` | 4 (4 / 0) |  | capsule-dream (2); hexagonal-mushroom-column (2) | **target-config** — Initial example of a playground (`hexagonal-mushroom-column`, `capsule-dream`); read by the wgpu server. Axis `example` of a playground. | 1 / 6: 🧊️wgpu/🌐️server/🟦️.ts |
| `SEMIO_RUNTIME_SOURCE_CONTROL` | 4 (0 / 4) |  | red (4) | **unread-probe-knob** — Ticket probe knob; no reader outside the ticket scripts. | 0 / 0 |
| `SEMIO_TICKET_DIR` | 4 (0 / 4) |  | ${workspaceFolder}/<ticket-path> (4) | **output-location** — Ticket directory handed to ticket-owned e2e scripts; ticket-scoped by nature. | 12 / 13: 🔨️modules/📦️site/📜️script.ts |
| `PLAYWRIGHT_BROWSERS_PATH` | 3 (0 / 3) |  | ${workspaceFolder}/.🧬semio/🦑️repo/⚡️cache… | **toolchain** — Shared Playwright cache; `repoToolCacheEnv()` already defaults it (`PLAYWRIGHT_BROWSERS_PATH ??=`). Drop. | 7 / 17: 📦️packages/🦀️rust/📜️script.ts |
| `PRINT_NATIVE_DIAGRAM_FAMILY` | 3 (3 / 0) |  | concept-shape (1); logic-tree (1) | **target-config** — Diagram family selector of the same chart-grammar test. | 1 / 1: 🧪️tests/🧬️native-chart-grammar/🟦️.ts |
| `PRINT_NATIVE_GEO_PALETTE_PHASE` | 3 (3 / 0) |  | flow-admission (1); vector-admission (1) | **target-config** — Sub-phase of the same chart-grammar test. | 1 / 1: 🧪️tests/🧬️native-chart-grammar/🟦️.ts |
| `S_OS_MCP_LIVE_LOCALE` | 3 (3 / 0) |  | de (2); en (1) | **service-config** — Locale `en\|de` of the os-mcp acceptance gates. | 2 / 5: 🧪️tests/🚶️user-path/🟦️.ts |
| `SEMIO_TEST_ARTIFACTS_ROOT` | 3 (0 / 3) |  | ${workspaceFolder}/<ticket-path> (3) | **output-location** — Third spelling, not read in tracked code (ticket rows only): dead knob. | 0 / 0 |
| `CAD_JS_RENDERER_PLAY_PORT` | 2 (2 / 0) |  | 6020 (1); 6120 (1) | **port** — Legacy per-app port variable: the cad package manifest still sets 6041 while the two seed rows set 6020/6120 and the dev server binds `S_OS_PORT`: stale knob (conflicting values). | 0 / 1 |
| `PETS_STORIES_PORT` | 2 (2 / 0) | 2 | 6069 (1); 6074 (1) | **port** — Pets stories port. | 1 / 3: 🏗️builder/🌐️vite/🟦️.ts |
| `PLAYGROUND_LOCKED_EXAMPLE_ID` | 2 (2 / 0) |  | capsule-dream (2) | **target-config** — Locks a playground to one example; read by the playground host. Axis `example` (locked). | 2 / 11: 📚️library/🎮️playground/🟦️.ts |
| `PRINT_NATIVE_GEO_PLANAR_PHASE` | 2 (2 / 0) |  | admission (1); guard (1) | **target-config** — Sub-phase of the same chart-grammar test. | 1 / 1: 🧪️tests/🧬️native-chart-grammar/🟦️.ts |
| `PUZZLE_3D_PLAY_PORT` | 2 (2 / 0) |  | 6013 (1); 6113 (1) | **port** — Legacy per-app port variable (not read in tracked code): dead knob. | 0 / 0 |
| `RUST_BACKTRACE` | 2 (0 / 2) |  | 1 (2) | **toolchain** — Backtraces on; read by Rust. | 0 / 2 |
| `S_OS_MCP_LIVE_SHELL_URL` | 2 (2 / 0) |  | ${input:acceptanceServeUrl} (2) | **service-config** — Shell URL for the os-mcp acceptance gates (bound to `${input:acceptanceServeUrl}`). | 3 / 6: 🧪️tests/💬️agent-reply/🟦️.ts |
| `SEMIO_APP_ROLE` | 2 (2 / 0) |  | viewer (2) | **target-config** — Opens the playground as `viewer` instead of editor. Axis `app role` of a playground. | 1 / 11: 🧊️wgpu/🌐️server/🟦️.ts |
| `SEMIO_TEST_RETAIN_ARTIFACTS` | 2 (0 / 2) |  | true (2) | **output-location** — Keep artifacts after the run; not read in tracked code (ticket probes). | 0 / 0 |
| `SHOOTING_PLAY_PORT` | 2 (2 / 0) |  | 6019 (1); 6119 (1) | **port** — Legacy per-app port variable (not read in tracked code): dead knob. | 0 / 0 |
| `TEACHING_ARCHITECTURE_QUIZ_PORT` | 2 (2 / 0) | 5 | 6061 (2) | **port** — Architecture quiz port; the owner's `📋️project.json` options.env and `package.json` already declare 6061 for `dev` and `dev-site`, the rows only repeat it. | 0 / 3 |
| `CARGO_INCREMENTAL` | 1 (0 / 1) |  | 0 (1) | **toolchain** — Incremental compilation off for one probe; read by cargo. | 1 / 15: 🔨️modules/🌉️mcp/🟦️.ts |
| `CLIENT_PORT` | 1 (1 / 0) |  | 6274 (1) | **port** — MCP inspector client port (6274); read by the inspector, not by the repo. | 0 / 0 |
| `GOWORK` | 1 (1 / 0) |  | ${workspaceFolder}/go.work (1) | **toolchain** — Go workspace file for the Go repo client; read by the go toolchain. | 0 / 23 |
| `MCP_AUTO_OPEN_ENABLED` | 1 (1 / 0) |  | false (1) | **service-config** — MCP inspector auto-open off; read by the inspector, not by the repo. | 0 / 0 |
| `MCP_PROXY_AUTH_TOKEN` | 1 (1 / 0) |  | repo-mcp-token (1) | **secret-like** — Literal dev token `repo-mcp-token` committed in the launch file; must not be copied into the canonical registry as a value. | 0 / 0 |
| `MIT_BESTAND_DEMONSTRATOR_PORT` | 1 (1 / 0) |  | 6029 (1) | **port** — Demonstrator port. | 2 / 4: 🏗️builder/🌐️vite/🟦️.ts |
| `OS_HUB_ADMIN_CAPABILITY_FILE` | 1 (1 / 0) |  | ${input:acceptanceHubAdminCapability} (1) | **service-config** — Hub admin capability file (bound to an input). | 1 / 4: 🧪️tests/🛡️security/🟦️.ts |
| `OS_HUB_URL` | 1 (1 / 0) |  | http://127.0.0.1:8787 (1) | **service-config** — Hub URL for the admin UI. | 1 / 1: 📦️packages/🟦️typescript/📜️script.ts |
| `PETS_MENAGERIE` | 1 (1 / 0) | 1 | 🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts … | **target-config** — Which pets menagerie the stories server loads. | 1 / 4: 🏗️builder/🌐️vite/🟦️.ts |
| `PRAESENTATION_PROJEKTETAGE_PORT` | 1 (1 / 0) |  | 6050 (1) | **port** — Port variable named by the package's own `package.json` (`"env": "PRAESENTATION_PROJEKTETAGE_PORT"`, default 6050): the owner already declares it, the launch row only repeats the value. | 0 / 3 |
| `PRINT_NATIVE_GEO_PLANAR_SOURCE` | 1 (1 / 0) |  | rust (1) | **target-config** — Source language selector of the same chart-grammar test. | 1 / 1: 🧪️tests/🧬️native-chart-grammar/🟦️.ts |
| `S_LOCAL_ONLY` | 1 (1 / 0) |  | 1 (1) | **service-config** — Local-only mode of the `s` shell (no hub). | 2 / 5: 📜️script.ts |
| `SEMIO_CARGO_PREPARATION_TIMING` | 1 (1 / 0) |  | 1 (1) | **target-config** — Prints cargo preparation timings; read by the cargo preparation scripts. | 3 / 3: 🦀️cargo/🛠️preparation/📜️script.ts |
| `SEMIO_DEBUG_CLOSE_PHASE` | 1 (1 / 0) |  | 1 (1) | **target-config** — Debug switch of one process3d test; read by the plugin crate. | 1 / 1: 🔨️modules/🔌️plugin/🦀️.rs |
| `SEMIO_REPO_IMPLEMENTATION` | 1 (1 / 0) |  | go (1) | **target-config** — Selects the `go` implementation of the repo client (read by the dashboard command-tree and the container lifecycle). | 2 / 7: 🎛️dashboard/🌳️command-tree/🦀️.rs |
| `SEMIO_TECH_PLAY_PORT` | 1 (1 / 0) |  | 6033 (1) | **port** — semio-tech play site port. | 2 / 3: 🏗️builder/🌐️vite/🟦️.ts |
| `SEMIO_TEST_OUTPUT_SCOPE` | 1 (0 / 1) |  | root/rewriting46-native (1) | **output-location** — Scopes test output below a root; read by the test plugin (`🧪️test/🟦️.ts`). | 1 / 3: 🔨️modules/🧪️test/🟦️.ts |
| `SERVER_PORT` | 1 (1 / 0) |  | 6277 (1) | **port** — MCP inspector proxy port (6277); read by the inspector, not by the repo. | 0 / 0 |
| `NX_PLUGIN_NO_TIMEOUTS` | 0 (0 / 0) | 2 |  | **runner-noise** — Only in .claude/launch.json (Nx plugin timeouts off for a slow cold graph). Drop. | Nx: src/project-graph/plugins/isolation/isolated-plugin…; 0 / 0 |
| `PROCTOR_DATA` | 0 (0 / 0) | 1 |  | **service-config** — Proctor data directory (.claude only). | 1 / 12: 🛂️proctor/🏗️bootstrap/🟦️.ts |
| `TEACHING_ARCHITECTURE_QUIZ_WATCH` | 0 (0 / 0) | 1 |  | **service-config** — Quiz watch switch (.claude only). | 2 / 4: 🏗️builder/🌐️vite/🟦️.ts |

Judgement summary: **noise** (drop, never store) = `NX_DAEMON`, `NX_ISOLATE_PLUGINS`, `NX_CACHE_PROJECT_GRAPH`, `NX_TUI`, `FORCE_COLOR`, `NX_PLUGIN_NO_TIMEOUTS`; **ticket sandbox** (drops with the ticket rows) = `NX_WORKSPACE_ROOT_PATH`, `NX_WORKSPACE_DATA_DIRECTORY`, `NX_CACHE_DIRECTORY`; **global axes** = `SEMIO_TEST_LEVEL`, `NEXTEST_SUCCESS_OUTPUT`, `CARGO_BUILD_JOBS`, `SEMIO_BUILD_MODE` + `NX_SKIP_NX_CACHE` + `NX_SKIP_REMOTE_CACHE`, `SEMIO_BUILD_BUDGET_MS`, `SEMIO_RENDERER`; **registry-owned** = `SEMIO_PLUGIN`, `SEMIO_APP`, `S_OS_PORT`. Port variables: playground servers bind `S_OS_PORT` only; services such as the quiz, projektetage and demonstrator declare their own port variable in their manifests (`options.env` or a `package.json` `"env"` key, which the code-read count does not see, hence the mention column), so those rows merely repeat the owner's default. `PUZZLE_3D_PLAY_PORT`, `PUZZLE_5D_PLAY_PORT` and `SHOOTING_PLAY_PORT` are not mentioned anywhere in tracked files and `CAD_JS_RENDERER_PLAY_PORT` conflicts with its owner's value: dead or stale knobs. `MCP_PROXY_AUTH_TOKEN=repo-mcp-token` is a literal token committed in the launch file.

## 7. Compounds and inputs

### 7.1 Compounds (4; all in the seed, none generated)

| Compound | Members (in start order) | Ready actions | Needs | Group / order |
| --- | --- | --- | --- | --- |
| `🧭️compound🖥️s⚛️react🌉️os-mcp` | `🛠️dev🌉️os-mcp🌐️http` -> `🛠️dev🪐️space⚛️react` [S_OS_PORT=6070] | 1/2 | hub URL in 1 member(s) (S_HUB_URL, S_DATA_DIR); all members found: true | 3_dev / 386.16 |
| `🧭️compound🖥️s⚛️react🗄️os-hub` | `🛠️dev🗄️os-hub` [OS_HUB_PORT=8787] -> `🛠️dev🪐️space⚛️react` [S_OS_PORT=6070] | 2/2 | hub URL in 2 member(s) (OS_HUB_PORT, OS_HUB_DATA, S_HUB_URL, S_DATA_DIR); all members found: true | 3_dev / 386.15 |
| `🧭️compound🖥️s👥️users🗄️os-hub` | `🛠️dev🗄️os-hub` [OS_HUB_PORT=8787] -> `🛠️dev🪐️space👤️1⚛️react` [S_OS_PORT=6072] -> `🛠️dev🪐️space👤️2⚛️react` [S_OS_PORT=6073] | 3/3 | hub URL in 3 member(s) (OS_HUB_PORT, OS_HUB_DATA, S_HUB_URL, S_DATA_DIR); all members found: true | 3_dev / 386.16 |
| `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor` | `🛠️dev🎓️teaching🛂️proctor` [PROCTOR_PORT=8791] -> `🛠️dev🎓️teaching🏛️architecture❓️quiz` [TEACHING_ARCHITECTURE_QUIZ_PORT=6061,PROCTOR_PORT=8791] | 1/2 | all members found: true | 3_dev / 213.62 |

Compound keys used: name, configurations, stopAll, presentation (no `preLaunchTask`, no `folder`). What the dashboard must reproduce: (1) members are launched **in array order** (hub before shell: the shell's `S_HUB_URL=http://127.0.0.1:8787` needs the hub up), (2) VS Code starts them in parallel without waiting, so a dashboard should **wait for each member's ready URL** (its `serverReadyAction` pattern) before starting a dependent member, (3) `stopAll: true` — stopping one stops all, (4) every member is an existing playground/hub row, so a compound is simply `[command refs]` + order + stop policy; 1 compound uses the multi-user rows (user slots 1 and 2 with separate `S_DATA_DIR`).

### 7.2 Inputs (82: pickString 52, promptString 30; 48 generated `projectTarget.*` pickStrings, 34 seed inputs)

Generated pickers: 48 checked, 0 with an option that does not declare the target. Dangling `${input:…}` references: 0. Declared but unused: `nativeScaleRegistry`, `gisVerification`, `nxCacheTicket`, `artifactPackageProject`, `artifactPackageTarget`, `catalogFreshBuildRoot`. Ticket-scoped (default/description references a ticket): 12 (`wgpuEmbeddedConfiguration`, `wgpuEmbeddedArtifacts`, `wgpuDockArtifacts`, `wgpuMediaArtifacts`, `rendererParityJourneyArtifacts`, `stdioRemovalArtifacts`, `stdioRemovalSnapshot`, `graphContractArtifacts`, `svgVideoArtifacts`, `processContractArtifacts`, `identityContractArtifacts`, `standaloneNativeOsPlan`).

| Input | Type | Default / options | Used by (non-ticket / ticket rows) | Ticket-scoped | Meaning |
| --- | --- | --- | --- | --- | --- |
| `wgpuEmbeddedConfiguration` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARI…` | 0 / 1 | yes | Ticket input configuration for actual embedded renderer acceptance |
| `wgpuEmbeddedArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARI…` | 0 / 1 | yes | Ticket directory for embedded browser receipts |
| `wgpuDockReactServe` | promptString | `http://127.0.0.1:7300/` | 0 / 3 | no | React renderer host URL |
| `wgpuDockServe` | promptString | `http://127.0.0.1:7301/?plugin=puzzle3d` | 0 / 3 | no | WGPU renderer host URL |
| `wgpuDockArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARI…` | 0 / 4 | yes | Ticket directory for Dock acceptance receipts |
| `wgpuMediaServe` | promptString | `http://127.0.0.1:7303/?plugin=stdio-wav` | 0 / 1 | no | WGPU stdio media viewer host URL |
| `wgpuMediaLocale` | pickString | options: en, de; default `en` | 0 / 2 | no | Explicit acceptance language |
| `wgpuMediaArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARI…` | 0 / 1 | yes | Ticket directory for media acceptance receipts |
| `rendererParityRenderer` | pickString | options: paired, react, wgpu; default `paired` | 0 / 1 | no | Renderer comparison |
| `rendererParityJourneyArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARI…` | 0 / 1 | yes | Ticket directory for shell interaction receipts |
| `nativeScaleRegistry` | promptString | `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🤖️generated/📇️re…` | 0 / 0 | no | Native scale registry JSON path |
| `gisVerification` | pickString | options: imports, watcher, map, graph; default `map` | 0 / 0 | no | GIS verification |
| `nxCacheTicket` | promptString | (no default) | 0 / 0 | no | Active ticket directory for Nx diagnostics |
| `artifactPackageProject` | promptString | (no default) | 0 / 0 | no | Artifact package Nx project, for example @semio-tech/stdio-pdf-rs |
| `artifactPackageTarget` | pickString | options: build, check, test | 0 / 0 | no | Artifact package target |
| `catalogFreshBuildRoot` | promptString | (no default) | 0 / 0 | no | Absolute fresh plugin catalog build root |
| `acceptanceHubUrl` | promptString | `http://127.0.0.1:8787` | 12 / 0 | no | Hub under acceptance (origin) |
| `historyUniversalVariant` | promptString | (no default) | 0 / 2 | no | Declared history playground variant / Deklarierte Verlauf-Playground-Variante |
| `historyUniversalServeUrl` | promptString | (no default) | 0 / 2 | no | Declared variant renderer URL / Renderer-URL der deklarierten Variante |
| `acceptanceServeUrl` | promptString | `http://127.0.0.1:6070/` | 9 / 0 | no | s React serve joined to that hub |
| `acceptanceLocalServeUrl` | promptString | `http://127.0.0.1:6070/` | 1 / 0 | no | Local-only s React serve (every plugin loaded; launch row 🛠️dev🪐️space⚛️react🔒local-only) |
| `acceptanceHubAdminCapability` | promptString | `` | 1 / 0 | no | The hub launcher's admin-capability.json (0600) for gates that read the hub's connection census |
| `acceptanceUsers` | promptString | `` | 3 / 0 | no | Optional JSON file {"users":[{"email","password"}…]} with the hub's test users (empty: each gate's … |
| `stdioRemovalArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYER…` | 0 / 5 | yes | Ticket output directory containing the retained copied workspace |
| `stdioRemovalSnapshot` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYER…` | 0 / 4 | yes | Retained workspace copy with only AVI absent, relative to the workspace |
| `graphContractArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYER…` | 0 / 2 | yes | Ticket-owned graph contract artifacts directory relative to workspace |
| `svgVideoArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYER…` | 0 / 2 | yes | Ticket-owned SVG video artifacts directory relative to workspace |
| `processContractArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYER…` | 0 / 63 | yes | Ticket-owned framework process contract output directory relative to workspace |
| `historyVerificationJob` | promptString | `message-clamp-native-20261007-a` | 0 / 3 | no | Exact ticket-owned verification cohort name; use a fresh name to start another cohort |
| `identityContractArtifacts` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYER…` | 0 / 1 | yes | Ticket-owned identity contract output directory relative to workspace |
| `proctorBackupFile` | promptString | (no default) | 1 / 0 | no | Proctor backup file to put in place of the dev database (stop the dev proctor first) |
| `proctorEraseHandle` | promptString | (no default) | 2 / 0 | no | Pseudonym or name of the learner to erase from the dev database (stop the dev proctor first) |
| `proctorPruneAge` | promptString | `7d` | 2 / 0 | no | Age past which a registration nobody played under is pruned from the dev database: 90m, 36h, 7d, 2w… |
| `standaloneNativeOsPlan` | promptString | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYER…` | 0 / 1 | yes | Admitted whole OS GREEN source plan containing the standalone transport owner |

The 48 `projectTarget.<target>` inputs are pickStrings whose options are the owning projects (generator[36], generator-carrier[3], generator-carrier-manifests[3], generator-encryption[4], generator-encryption-manifests[4], generator-manifests[32], build[247], canonical-architecture[33], +40 more); the dashboard already lists every project target, so they are redundant. Input kinds a dashboard needs: free text with optional default (30) and fixed choice (4 non-generated pickStrings); no `command` inputs and no `password` flags are used.

## 8. serverReadyAction

356 rows (356 non-ticket, 0 ticket-scoped); all use `action: openExternally`; extra keys: (none) 356. By class: playground-dev 326, nx-preset 24, tool-launcher 6; by provenance playground-synthesized 222, playground-seed-placeholder 92, seed-authored 42. In 354 of 356 rows the port inside the pattern equals a port env variable of the same row; exceptions: #589 `🛠️dev🧰️repo🤖️mcp` (pattern port 6274, env none); #665 `🛠️dev🗄️os-hub🛡️admin` (pattern port 8790, env none).

| Pattern (port normalised to {PORT}) | uriFormat | Rows | Examples |
| --- | --- | --- | --- |
| `(http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT})` | `%s` | 340 | `🛠️dev📐️cad⚛️react`, `🛠️dev📐️cad🧊️wgpu🌐️wasm`, `🛠️dev🕸️dag⚛️react` |
| `(http://(?:127\.0\.0\.1\|localhost):{PORT})` | `%s` | 9 | `🛠️dev📽️projektetage`, `🛠️dev♻️mit-bestand🧺️demonstrator`, `🛠️dev🏢️semio-tech🎡️play` |
| `(http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT})` | `%s/admin` | 3 | `🛠️dev🗄️os-hub`, `🛠️dev🗄️os-hub🐘️postgres`, `🛠️dev🗄️os-hub🕸️neo4j` |
| `(http://[^\s]+:{PORT})` | `%s` | 1 | `🖱️mcpinspector` |
| `(http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT})` | `%s/?plugin=generation3d` | 1 | `🛠️dev🔧️procedural🏙️3d👁️viewer…` |
| `(http://(?:127\.0\.0\.1\|localhost\|0\.0\.0\.0):{PORT})` | `%s/?plugin=generation3d&role=viewer` | 1 | `🛠️dev🔧️procedural🏙️3d👁️viewer…` |
| `(http://[^\s]+:{PORT}/\?MCP_PROXY_AUTH_TOKEN=[^\s]+)` | `%s&transport=stdio&serverCommand=cargo&serverArgs=run&serverArgs=--release&serverArgs=-p&serverArgs=semio-framework-repo-cli&serverArgs=--&serverArgs=mcp&MCP_PROXY_FULL_ADDRESS=http://127.0.0.1:6277` | 1 | `🛠️dev🧰️repo🤖️mcp` |

What a dashboard needs to reproduce: watch the task's output for the first match of the regex (a localhost/127.0.0.1/0.0.0.0 `http://host:port`; the generator uses the host alternation `127.0.0.1|localhost|0.0.0.0` because a devcontainer prints `0.0.0.0`), take capture group 1 (the whole URL) and open `uriFormat` with `%s` replaced by it (`%s`, `%s/admin`, `%s/?plugin=…`, or the MCP inspector URL with auth token and server arguments); `openExternally` = default browser. Because the port equals the row's own port variable, a single declaration **ready: {port, path-suffix}** reproduces every pattern except the MCP inspector one (token query string) and the two `?plugin=…` suffix rows — the pattern itself is derivable. For playgrounds the port, URL suffix and host are registry facts (`ports.react`/`wgpu`/`userPorts`).

## 9. Presentation: groups, order and the name grammar

| `presentation.group` | Rows | Order range | Distinct orders | Leading name keys | Provenance |
| --- | --- | --- | --- | --- | --- |
| `(none)` | 41 | - | 0 | 🛠️editor 9, 🛠️os 9, 🛠️general 8, 🛠️unified 4 | seed-authored 41 |
| `0_dev` | 4 | 121.4 .. 392.002 | 4 | 🛠️dev 4 | seed-authored 4 |
| `1_keyboard` | 2 | 10 .. 20 | 2 | ⌨️gemini 1, ⌨️kiro 1 | seed-authored 2 |
| `2_build` | 1 | 206.061 .. 206.061 | 1 | 🪧️Logo 1 | seed-authored 1 |
| `2_mouse` | 4 | 10 .. 31 | 4 | 🖱️mcpinspector 2, 🖱️f 1, 🖱️gitkraken 1 | seed-authored 4 |
| `3_dev` | 635 | -50 .. 950.007 | 609 | 🛠️dev 450, 🧪️test 40, 📥️deps 14, ▶️framework 11 | seed-authored 225, playground-synthesized 222, playground-seed-placeholder 92, project-launcher-target 92, project-launcher-family 4 |
| `4_build` | 260 | 9 .. 900.0089 | 251 | 🧿️semio 65, 📦️build 30, 📦️🏭️generator 28, 📦️check 19 | seed-authored 153, project-launcher-target 90, project-launcher-family 17 |
| `4_gate` | 3117 | -11.5 .. 902.207001 | 2654 | ⚖️gate 580, ⚖️test 473, (no leading emoji) 227, 🧫️fixtures 153 | seed-authored 2473, project-launcher-target 617, project-launcher-family 27 |
| `4_test` | 6 | 1.4 .. 900.0385 | 4 | 🧪️test 3, 🧪️source 2, ⚖️test 1 | seed-authored 6 |
| `9_clean_architecture` | 1 | 901.05756 .. 901.05756 | 1 | ⚖️check 1 | seed-authored 1 |
| `9_gates` | 453 | 408.781 .. 900.0586999999999 | 445 | 🧪️test 348, ⚖️gate 28, (no leading emoji) 27, 🦑️Repo 14 | seed-authored 453 |
| `9_goal` | 3 | 900.05837 .. 900.05848 | 3 | ⚖️gate 3 | seed-authored 3 |
| `repo-gate` | 3 | 900.056975 .. 900.056977 | 3 | 🧪️ 1, 🧰️ 1, 🪪️ 1 | seed-authored 3 |
| `🧹clean🛡️gates` | 29 | 900.05693 .. 900.05808 | 29 | 🧰️framework 9, 🗄️stdio 4, 🦑️Repo 4, 🦑️repo 3 | seed-authored 29 |
| `🧿️ Semio Snapshot SQLite` | 18 | 206.1806 .. 206.18073 | 14 | ♻️ 10, 🧿️ 6, 🌊️ 1, 🔢️ 1 | seed-authored 18 |
| `🪶️ Artifact Snapshot SQLite` | 2 | 206.19 .. 206.1906 | 2 | 🪶️ 2 | seed-authored 2 |
| `🪶️ Snapshot SQLite` | 5 | 206.1901 .. 206.1905 | 5 | 🪶️ 5 | seed-authored 5 |

- `group` is VS Code's sort bucket (lexicographic: `0_dev`, `1_keyboard`, `2_mouse`, `3_dev`, `4_build`, `4_gate`, `4_test`, `9_gates`, then ad-hoc names); `order` sorts inside a group. 4031 distinct (group, order) pairs for 4543 rows, 880 rows share an order with another row. **Neither carries information that is not already in (verb, project, target)**: generated rows use `orderBase + index/10000` in label-then-target order; seed numbers are insertion positions (`386.15`, `900.0586999…`).
- Groups map to verbs: `3_dev` = dev servers and workspace commands (class `dev` + fallback `run`), `4_gate` = checks/tests/verification (class `gate`), `4_build` = build/generate/format/clean (class `build`), `9_gates` = the hand-written `🧪️test…` single-file probes, `1_keyboard`/`2_mouse` = tool shortcuts (gemini, kiro / f3d, gitkraken, MCP inspector), the rest are one-off labels (`🧿️ Semio Snapshot SQLite`, `repo-gate`, `🧹clean🛡️gates`, …). 41 rows have no `presentation` at all (they sort last).

Leading name keys (emoji + first lowercase word):

| Key | Rows | Groups | Meaning |
| --- | --- | --- | --- |
| `⚖️gate` | 611 | 4_gate 580, 9_gates 28, 9_goal 3 | gate: check/test/verify run (seed hand name, or `⚖️gate` + domain emojis) |
| `⚖️test` | 476 | 4_gate 473, 9_gates 2, 4_test 1 | gate named after the target (`⚖️test-<target><label>`; generated or hand-made) |
| `🛠️dev` | 456 | 3_dev 450, 0_dev 4, (none) 1, 4_build 1 | dev server / dev tool (playground launcher or `🛠️dev<domain>`) |
| `🧪️test` | 393 | 9_gates 348, 3_dev 40, 4_test 3, 4_gate 2 | single-file or probe test (`9_gates`) |
| `(no leading emoji)` | 254 | 4_gate 227, 9_gates 27 | free-form (`🧰️ Framework …` style with space) |
| `🧫️fixtures` | 153 | 4_gate 153 | fixture-isolation rows of the FIXTURES ticket |
| `🧱️` | 150 | 4_gate 150 | ticket probe rows |
| `⚖️verify` | 83 | 4_gate 83 | verification gates |
| `📥️native` | 78 | 4_gate 78 | ticket probe rows |
| `🧿️semio` | 65 | 4_build 65 | snapshot SQLite rows |
| `🛠️general` | 57 | 4_gate 49, (none) 8 |  |
| `⚖️check` | 50 | 4_gate 48, 4_build 1, 9_clean_architecture 1 |  |
| `🛠️cargo` | 50 | 4_gate 50 |  |
| `🛠️block` | 47 | 4_gate 47 |  |
| `🧹clean` | 42 | 4_gate 32, 4_build 7, 🧹clean🛡️gates 3 | clean/guard rows |
| `📥️ui` | 32 | 4_gate 32 |  |
| `⚖️stage` | 31 | 4_gate 31 |  |
| `📦️build` | 31 | 4_build 30, 3_dev 1 | build |
| `📦️🏭️generator` | 28 | 4_build 28 | generator build |
| `🛠️surface` | 28 | 4_gate 25, (none) 3 |  |
| `⚖️borrowed` | 27 | 4_gate 27 |  |
| `📥️renderer` | 26 | 4_gate 26 |  |

Grammar, as far as it is mechanical:

- Generated project rows: `<verb emoji><target><project label>`; verb emoji from the target-name token (⚖️ gate: test/check/verify/lint/…, 📦️ build, 🛠️ dev, ▶️ run), label = shortest unique trailing path segments with 🦀️/🟦️/🐍️ (example `⚖️test-artifact-kind🧰️framework🦀️`). Family rows end in `📋️`. **Fully derivable** (section 2).
- Playground rows: `🛠️dev<plugin folder><artifact folder>[<standard folder>]<subset folder><renderer suffix>` with suffix ⚛️react 162, 🧊️wgpu🌐️wasm 162, (other) 2; user slot rows insert `👤️<N>` before the suffix (4 rows). Derivable from registry + taxonomy folders (needs the folder walk of `🏷️name-prefix`), or simply `<variant> <renderer>`.
- Compounds: `🧭️compound<member shorthand>` (🧭️compound🖥️s⚛️react🌉️os-mcp; 🧭️compound🖥️s⚛️react🗄️os-hub; 🧭️compound🖥️s👥️users🗄️os-hub; 🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor).
- Seed-authored names are free-form hand labels (e.g. `⚖️gate🏢️semio-tech🎡️play🌊️flow-fixtures`, `🧪️test📜️history🎮️operation…`): 254 names have no leading emoji at all, 622 contain spaces. They carry no information beyond the underlying command and are not derivable; a registry should derive the label from (verb, target, owner label) and let an owner override it only when needed.

## 10. `.claude/launch.json` (Claude Code preview servers)

79 entries; keys used: name 79, port 62, runtimeArgs 55, runtimeExecutable 55, env 28, url 24. 55 start a process (`runtimeExecutable` bun/bash + `runtimeArgs`), 24 only declare `port`/`url` (attach to a running server). 28 entries have env (noise keys: NX_DAEMON 3, NX_ISOLATE_PLUGINS 2, NX_CACHE_PROJECT_GRAPH 1). By kind: attach-only 24, playground-dev 19, gate 13, service-dev 8, ticket-script 8, dev-tool 4, script 2, playground-native 1. 10 entries run exactly the same command+env as a `.vscode/launch.json` row. All playground ports match the playground registry (0 mismatches). Nx verification: 1 entry dead (`@semio-tech/framework-rs:test-snapshot-sqlite-io`); 8 entries run ticket-local scripts; 2 entries point at script files that no longer exist (`terra-jco-spike-static`, `map-harness`).

| Kind | Entries | Corresponds to |
| --- | --- | --- |
| `attach-only` | 24 | no command: `{name, port\|url}` — attach the preview to a server that is already running |
| `playground-dev` | 19 | playground dev server (`workspace:dev -- <variant>`, `framework-os-dev:serve-<v>-react-dev`, `framework-os-dev:dev -- <v>`): canonical = playground catalog entry + renderer; port equals the registry port |
| `gate` | 13 | an `nx run P:test…` gate: canonical = the Nx target (+ level axis) |
| `service-dev` | 8 | service dev server (`P:dev`) with a port: canonical = Nx target `dev` + declared port/ready |
| `ticket-script` | 8 | ticket-owned script (bash/bun file under `🎫️tickets`): no canonical command |
| `dev-tool` | 4 | `workspace:dev -- storybook …\|gis 2d`: workspace dev command |
| `script` | 2 | a bun script outside Nx |
| `playground-native` | 1 | wgpu native renderer of a playground (`framework-renderer-wgpu:native -- <v>`) |

| Name | Kind | Canonical command | Port (registry) | Env (semantic) | Same as launch.json row | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| `⚖️gate🎒️pack🫳️borrowed-prefligh…` | gate | @semio-tech/framework-pack-rs:test-borrowed-preflight-n… | - | SEMIO_TEST_LEVEL | - | ok |
| `⚖️gate🎒️pack🫳️borrowed-prefligh…` | gate | @semio-tech/framework-pack-rs:test-borrowed-preflight-s… | - | SEMIO_TEST_LEVEL | - | ok |
| `cad-react` | playground-dev | playground cad (react) | 6020 (6020 ok) | - | - | ok |
| `s-react` | playground-dev | playground s (react) | 6070 (6070 ok) | - | - | ok |
| `s-react-served` | playground-dev | playground s (react) args: served | 6070 (6070 ok) | - | - | ok |
| `procedural3d-react` | playground-dev | playground generation3d (react) | 6018 (6018 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `procedural3d-wgpu` | playground-dev | playground generation3d (wgpu) | 6118 (6118 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `puzzle3d-react` | playground-dev | playground puzzle3d (react) | 6013 (6013 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `puzzle3d-wgpu` | playground-dev | playground puzzle3d (wgpu) | 6113 (6113 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `puzzle5d-react` | playground-dev | playground puzzle5d (react) | 6014 (6014 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `puzzle5d-wgpu` | playground-dev | playground puzzle5d (wgpu) | 6114 (6114 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `puzzle5d-native` | playground-native | playground puzzle5d (wgpu native) | - | - | `🛠️dev🧩️puzzle🖐️5d🧊️wgpu🖥…` | ok |
| `dag-react` | playground-dev | playground dag (react) | 6017 (6017 ok) | - | - | ok |
| `terra-jco-spike-static` | script | bun 🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcopro… | 8846 | - | - | script missing |
| `mit-bestand-demonstrator` | service-dev | @semio-tech/mit-bestand-demonstrator:dev | 6029 | - | - | ok |
| `mit-bestand-demonstrator-supervis…` | ticket-script | (ticket-local script) | 6029 | - | - | ok |
| `map-harness` | ticket-script | (ticket-local script) | 6210 | - | - | script missing |
| `storybook-framework-hosts` | dev-tool | workspace:dev -- storybook framework hosts | 6010 | - | - | ok |
| `gis2d-wgpu` | dev-tool | workspace:dev -- gis 2d | 6140 | - | - | ok |
| `verify-gis2d-envelope-gates` | ticket-script | (ticket-local script) | - | - | - | ok |
| `puzzle2d-react` | playground-dev | playground puzzle2d (react) | 6012 (6012 ok) | - | - | ok |
| `storybook-static` | dev-tool | workspace:dev -- storybook-static | 6010 | - | - | ok |
| `storybook-framework-os` | dev-tool | workspace:dev -- storybook framework os | 6011 | - | - | ok |
| `puzzle3d-react-attach` | attach-only | attach http://localhost:6013 | 6013 | - | - | ok |
| `puzzle3d-react-release-e2e-attach` | attach-only | attach http://localhost:6014 | 6014 | - | - | ok |
| `puzzle5d-react-attach` | attach-only | attach http://localhost:6014 | 6014 | - | - | ok |
| `puzzle5d-react-supervised` | ticket-script | (ticket-local script) | 6014 | - | - | ok |
| `puzzle3d-react-supervised` | ticket-script | (ticket-local script) | 6013 | - | - | ok |
| `puzzle5d-battery` | ticket-script | (ticket-local script) | - | - | - | ok |
| `puzzle5d-battery-explore` | ticket-script | (ticket-local script) | - | - | - | ok |
| `puzzle2d-react-supervised` | ticket-script | (ticket-local script) | 6012 | - | - | ok |
| `procedural3d-react-attach` | attach-only | attach http://localhost:6018 | 6018 | - | - | ok |
| `process3d-react` | playground-dev | playground process3d (react) | 6022 (6022 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `process3d-react-attach` | attach-only | attach http://localhost:6022 | 6022 | - | - | ok |
| `process3d-react-lane-attach` | attach-only | attach http://localhost:6222 | 6222 | - | - | ok |
| `fem2d-react` | playground-dev | playground fem2d (react) | 6086 (6086 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `fem2d-react-attach` | attach-only | attach http://localhost:6086 | 6086 | - | - | ok |
| `fem3d-react` | playground-dev | playground fem3d (react) | 6087 (6087 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `fem3d-react-attach` | attach-only | attach http://localhost:6087 | 6087 | - | - | ok |
| `energy-react` | playground-dev | playground energy (react) | 6106 (6106 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `energy-react-attach` | attach-only | attach http://localhost:6106 | 6106 | - | - | ok |
| `forms-react-attach` | attach-only | attach http://localhost:6058 | 6058 | - | - | ok |
| `raster-react-attach` | attach-only | attach http://localhost:6060 | 6060 | - | - | ok |
| `shooting-react-attach` | attach-only | attach http://localhost:6019 | 6019 | - | - | ok |
| `remodel-react-attach` | attach-only | attach http://localhost:6063 | 6063 | - | - | ok |
| `layout-react-attach` | attach-only | attach http://localhost:6079 | 6079 | - | - | ok |
| `note-react-attach` | attach-only | attach http://localhost:6080 | 6080 | - | - | ok |
| `wfc-bitmap-react-attach` | attach-only | attach http://localhost:6041 | 6041 | - | - | ok |
| `wfc-grid2d-react-attach` | attach-only | attach http://localhost:6042 | 6042 | - | - | ok |
| `wfc-wfc2d-react-attach` | attach-only | attach http://localhost:6043 | 6043 | - | - | ok |
| `wfc-grid3d-react-attach` | attach-only | attach http://localhost:6044 | 6044 | - | - | ok |
| `wfc-wfc3d-react-attach` | attach-only | attach http://localhost:6045 | 6045 | - | - | ok |
| `reasoning-react` | playground-dev | playground reasoning-wires (react) | 6015 (6015 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `reasoning-react-attach` | attach-only | attach http://localhost:6015 | 6015 | - | - | ok |
| `imperative-react` | playground-dev | playground imperative (react) | 6076 (6076 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `imperative-react-attach` | attach-only | attach http://localhost:6076 | 6076 | - | - | ok |
| `playbook-react` | playground-dev | playground playbook (react) | 6085 (6085 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `playbook-react-attach` | attach-only | attach http://localhost:6085 | 6085 | - | - | ok |
| `norm-react` | playground-dev | playground din4108 (react) | 6091 (6091 ok) | SEMIO_RENDERER,S_OS_PORT | - | ok |
| `norm-react-attach` | attach-only | attach http://localhost:6091 | 6091 | - | - | ok |
| `teaching-proctor` | service-dev | @teaching/proctor:dev | 8791 | PROCTOR_PORT | `🛠️dev🎓️teaching🛂️proctor` | ok |
| `architecture-quiz` | service-dev | @teaching/architecture-quiz:dev | 6061 | TEACHING_ARCHITECTURE_QUIZ_PORT,PROCTOR_PORT | `🛠️dev🎓️teaching🏛️architect…` | ok |
| `host-count-component-sqlite` | gate | @semio-tech/framework-plugin-host:count-component-check | - | SEMIO_TEST_ARTIFACT_DIR | `⚖️check🧫️host-count-componen…` | ok |
| `architecture-quiz-site` | service-dev | @teaching/architecture-quiz:dev-site | 6061 | TEACHING_ARCHITECTURE_QUIZ_PORT,PROCTOR_PORT | `🛠️dev🎓️teaching🏛️architect…` | ok |
| `architektur-und-technologie-quizze` | service-dev | @teaching/architecture-quiz:dev | 6061 | TEACHING_ARCHITECTURE_QUIZ_PORT,PROCTOR_PORT | `🛠️dev🎓️teaching🏛️architect…` | ok |
| `architektur-und-technologie-quizz…` | service-dev | @teaching/architecture-quiz:dev | 6061 | TEACHING_ARCHITECTURE_QUIZ_PORT,PROCTOR_PORT,TEACHING_ARCHITECTURE_QUIZ_WATCH | - | ok |
| `architektur-und-technologie-quizz…` | script | bun 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️t… | 6063 | TEACHING_ARCHITECTURE_QUIZ_PORT,PROCTOR_PORT,PROCTOR_DATA | - | ok |
| `framework-snapshot-sqlite` | gate | @semio-tech/framework-rs:test-snapshot-sqlite | - | - | - | ok |
| `framework-deflate-encoding` | gate | @semio-tech/framework-rs:test-deflate-encoding | - | - | `⚖️test-deflate-encoding🧰️fra…` | ok |
| `framework-snapshot-sqlite-native` | gate | @semio-tech/framework-rs:test-snapshot-sqlite-native | - | - | - | ok |
| `framework-snapshot-sqlite-source` | gate | @semio-tech/framework-rs:test-snapshot-sqlite-source | - | - | - | ok |
| `framework-snapshot-sqlite-io` | gate | @semio-tech/framework-rs:test-snapshot-sqlite-io | - | - | `⚖️test-snapshot-sqlite-io🧰️f…` | dead (@semio-tech/framework-rs:test-snapshot-sqlite-io) |
| `value-borrowed-key-index-native` | gate | @semio-tech/framework-os-kernel:test-borrowed-key-index… | - | SEMIO_TEST_LEVEL | - | ok |
| `value-intrinsic-retirement-native` | gate | @semio-tech/framework-os-kernel:test-intrinsic-retireme… | - | SEMIO_TEST_LEVEL | - | ok |
| `value-controlled-construction` | gate | @semio-tech/value-rs:test-controlled-construction | - | - | `⚖️test-controlled-constructio…` | ok |
| `value-controlled-construction-sou…` | gate | @semio-tech/framework-value:test-controlled-constructio… | - | - | `⚖️test-controlled-constructio…` | ok |
| `pptx-outline-ownership-source` | gate | @semio-tech/stdio-pptx-rs:test-outline-ownership | - | SEMIO_TEST_ARTIFACT_DIR | - | ok |
| `pets-stories` | service-dev | @semio-tech/pets-react:dev | 6069 | PETS_STORIES_PORT,NX_PLUGIN_NO_TIMEOUTS | - | ok |
| `architecture-pets-stories` | service-dev | @semio-tech/pets-react:dev | 6074 | PETS_STORIES_PORT,PETS_MENAGERIE,NX_PLUGIN_NO_TIMEOUTS | - | ok |

Recommendation: the preview entries are a *second* registry for the same playground servers. Every `playground-dev`, `playground-native`, `service-dev`, `gate` and `dev-tool` entry is a view of a canonical command the dashboard already discovers (+ port); the `attach-only` entries become a dashboard-side **attach** (port/url of a running dev server, registry-derived) and the ticket scripts (supervisors, probes) are ticket-scoped and drop. The only data worth keeping is the port, which the playground registry already owns; the file can be generated from the dashboard's catalog (name, port, command) if Claude Code's preview tool still needs it.

## 11. Recommendation per class and family

A ticket-scoped or dead row takes that recommendation first (ticket status and deadness are properties of the row, not of its family); the families then decide the rest. "Playground catalog discovery" is the dashboard's existing discovery of playground leaves from the generated playground catalog (the same registry the generator consumes); it is the second kind of "redundant with discovery" and is listed separately from plain Nx targets.

| Class / family | Rows | Recommendation | Why |
| --- | --- | --- | --- |
| `ticket-scoped / ticket/nx-exec-ticket-script` | 1464 | drop (ticket-scoped, open ticket) | One-off ticket probe; belongs in the ticket folder. |
| `plain-nx / plain-nx` | 1348 | drop (redundant with Nx target discovery) | The dashboard discovers every Nx target. |
| `plain-nx / plain-nx` | 5 | drop (dead) | The Nx target or file no longer exists. |
| `ticket-scoped / ticket/nx-run-preset` | 365 | drop (ticket-scoped, open ticket) | One-off ticket probe; belongs in the ticket folder. |
| `ticket-scoped / ticket/nx-run-preset` | 2 | drop (ticket-scoped, no ticket reference) | One-off ticket probe; belongs in the ticket folder. |
| `playground-dev / playground-dev/registry-generated` | 310 | drop (redundant with playground catalog discovery) | Generated from the playground registry that the dashboard discovers; renderer is an axis. |
| `nx-preset / test-axis-variant` | 280 | express as a global axis | Only cross-cutting axes on a discovered target. |
| `nx-preset / test-axis-variant` | 1 | drop (dead) | The Nx target or file no longer exists. |
| `nx-preset / test-selection/filter` | 268 | needs an input prompt | Axes + a free-text test selection (prompt). |
| `nx-exec-bun-test / bun-test-file` | 81 | needs an input prompt | One generic `bun test <file>` command with a file prompt. |
| `nx-preset / axis-variant` | 66 | express as a global axis | Only cross-cutting axes on a discovered target. |
| `nx-preset / print-native-phase` | 65 | keep as declared command preset in the owner's manifest | One preset with a phase parameter owned by `@semio-tech/print`. |
| `nx-preset / target-specific-arguments` | 62 | keep as declared command preset in the owner's manifest | Sub-commands / backend names / serve URLs are target facts; declare in the owner's manifest. |
| `nx-preset / target-specific-arguments` | 1 | drop (dead) | The Nx target or file no longer exists. |
| `project-picker-family / project-picker` | 48 | drop (redundant with Nx target discovery) | One row per target name with a project pick list = the dashboard's own target list. |
| `nx-preset / wgpu-native-launch` | 45 | express as a global axis | Renderer axis value `wgpu-native` of a catalogued playground. |
| `nx-preset / test-selection/vitest-files` | 29 | needs an input prompt | Axes + test file prompt. |
| `nx-preset / test-selection/vitest-files` | 1 | drop (dead) | The Nx target or file no longer exists. |
| `ticket-scoped / ticket/bun-script` | 25 | drop (ticket-scoped, open ticket) | One-off ticket probe; belongs in the ticket folder. |
| `nx-preset / dev-server-port` | 24 | keep as declared command preset in the owner's manifest | Port + ready pattern belong in the owner's manifest. |
| `nx-preset / prompted-argument` | 21 | needs an input prompt | Already an input prompt (acceptance URLs, proctor arguments). |
| `nx-preset / test-selection/scope` | 14 | needs an input prompt | Axes + cargo scope prompt. |
| `tool-launcher / nx-run-many` | 12 | express as a compound | A named project set for one target. |
| `playground-dev / playground-dev/seed-variant` | 12 | express as a global axis | Example / role / fixture argument are playground axes. |
| `tool-launcher / dev-tool/workspace-dev-mcp` | 9 | keep as declared command preset in the owner's manifest | Repo-level MCP tools. |
| `ticket-scoped / ticket/nx-exec` | 6 | drop (ticket-scoped, open ticket) | One-off ticket probe; belongs in the ticket folder. |
| `tool-launcher / external-tool` | 5 | keep as declared command preset in the owner's manifest | External developer tools (not Nx). |
| `nx-exec-other / nx-exec-other` | 5 | keep as declared command preset in the owner's manifest |  |
| `ticket-scoped / ticket/nx-other` | 5 | drop (ticket-scoped, open ticket) | One-off ticket probe; belongs in the ticket folder. |
| `playground-dev / playground-dev/user-slot` | 4 | express as a global axis | User slot is an axis of a playground (registry `userPorts`). |
| `tool-launcher / bun-script` | 1 | keep as declared command preset in the owner's manifest |  |

### Totals

| Recommendation | Rows | Share |
| --- | --- | --- |
| drop (dead) | 8 | 0.2% |
| drop (redundant with Nx target discovery) | 1396 | 30.5% |
| drop (redundant with playground catalog discovery) | 310 | 6.8% |
| drop (ticket-scoped, open ticket) | 1865 | 40.7% |
| drop (ticket-scoped, no ticket reference) | 2 | 0.0% |
| keep as declared command preset in the owner's manifest | 171 | 3.7% |
| express as a global axis | 407 | 8.9% |
| express as a compound | 12 | 0.3% |
| needs an input prompt | 413 | 9.0% |
| **total** | 4584 | 100% |

Dropped: **3581** (78.1%) — 8 dead, 1396 redundant with Nx discovery, 310 redundant with the playground catalog, 1867 ticket-scoped (0 of them with a closed ticket). Retained in some form: **1003**.

### How small is the hand-maintained canonical registry?

| Registry element | Entries | Covers | Content |
| --- | --- | --- | --- |
| Global axes | 12 | 1186 non-ticket rows spell at least one axis | cache-policy, test-level, renderer, task-dependencies, build-mode, nextest-output, cargo-jobs, output-style, build-budget, user-slot, example, app-role |
| Declared presets (owner manifest) | 101 | 171 rows | 70 distinct base targets; target-specific-arguments 62, dev-server-port 24, dev-tool/workspace-dev-mcp 6, external-tool 5, nx-exec-other 2, bun-script 1, print-native-phase 1 |
| Compounds | 16 | 12 rows + 4 VS Code compounds | 4 VS Code compounds (hub+shell, multi-user hub, quiz+proctor) + 12 `nx run-many` target groups |
| Input prompts | 12 | 413 rows | 8 seed inputs used outside tickets (proctorBackupFile, proctorEraseHandle, proctorPruneAge, acceptanceHubUrl, acceptanceServeUrl, acceptanceLocalServeUrl, acceptanceUsers, acceptanceHubAdminCapability) + generic prompts bun-test-file, test-files, test-filter, test-scope |
| **Total hand-maintained** | 141 | 1003 retained rows from 4584 | everything else is discovered (Nx targets, playground catalog) or dropped |

Upper bound: the 101 presets still contain families that parameterise further (the `two-client-e2e`/`document-growth-e2e`/`backend-*` rows differ only by backend sqlite|postgres|neo4j, `daemon` only by start|attach, 12 storybook dev servers only by their target); with a per-target `args: [...]` domain the preset count approaches the 70 distinct base targets. The 12 global axes replace 1186 non-ticket rows that spell an axis, and the user-slot/example/role axes replace 16 playground rows (4 generated user-slot rows + 12 seed variants).

### Order of migration (suggested)

1. Delete the 1867 ticket rows with their 12 ticket-path inputs (and the 6 unused inputs); a dashboard that never reads a launch file also stops `reconcile-launch-seed` from re-adopting new ones. 2. Drop the 8 dead rows and the 1353+48+310 discovered-anyway rows. 3. Hand-migrate the remaining rows in this order: playground axes (user slot, example, role, wgpu-native), the test axes (level, nextest output, cargo jobs, ship build + cache policy, task dependencies, output style) with the filter/file prompts, then the 101 declared presets in their owners' manifests (ports + ready patterns of the 24 dev servers first), then the 16 compounds, then the remaining prompts. 4. Retire `.claude/launch.json` last (79 entries; derivable from the same catalog).

## 12. Caveats and open points

- Ticket scoping is evidence-based (paths, ticket-named Nx projects, `🗑️generated`, ticket input defaults); a legitimate non-ticket row that merely writes its output below `🗑️generated` would be counted as ticket-scoped (2 rows are in that situation: #412, #1189; both are `bun nx run os-hub:test --skip-nx-cache -…`-style gates that hand their receipts to a root-level `🗑️generated`).
- Existence is verified against declared manifests, inference rules and the cached graph without running Nx; a target created by an inference plugin that neither the rules nor the graph know would be reported `missing-target`. The graph copy used was computed 2026-10-07T17:12:59.083Z; 10 of its targets are not explained by manifests or rules (`nx-release-publish` and two `test:*`).
- Relative file arguments after `--` are checked against the project root and the workspace root only; a program that resolves them elsewhere would be reported missing (the two non-ticket missing-path rows were confirmed by hand: one file moved, one is untracked).
- `readers` of an env variable are found by regex over tracked code; a variable read only through a computed key is reported with 0 readers (the `unread-probe-knob` and "dead knob" judgements say "no tracked reader found", not "proved unused").
- The live `.vscode/launch.json` and seed are rewritten by other developers while this audit ran (live differs from the audited snapshot: launch, seed); re-run with `--source=live` for current numbers. The inventory is deterministic for a given snapshot.
- Nothing outside the ticket folder was written; no `nx`, `cargo` or build was run. `--verify-generator` imports the repository's own generator in memory (read-only).
