# R2 Audit: Launch Dependents (static, read-only)

Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT` · audit date 2026-10-08 · HEAD `2604f70cac1` (2026-10-08 02:26:58) · working tree clean at start except `r2-fleet-brief.md`.
Scope: every tracked file outside `.🧬semio/` and `node_modules`, incl. `.cursor/`, `.claude/`, `.vscode/`, `.codex/`, `.agents/`, `.github/`, `.devcontainer/`, `.gitignore`. No untracked file outside `.🧬semio/` exists.
Method: `git grep -n -i -E` over the pattern set of the brief plus `launch-name`, `launchNamePrefix`, `devLaunchers`, `projectLaunchers`, `playgrounds.json`, `forwardPorts`, `.claude/`, `preview_start` (pattern stored in `🗑️generated/r2-pattern.txt`); a word-level sweep for `launch` (`🗑️generated/r2-path-hits-launch.txt`, `r2-hits-lines.txt`); the doc index of `launch-dependents.md` section 8 checked row by row (`🗑️generated/r2-doc-status.tsv`). Counts from `r2-launch-stats.ts` (bun 1.4.2, JSONC stripped by a small tokenizer; output `🗑️generated/r2-launch-stats.json`).
UNVERIFIED: no test, cargo, bun test or build was run. "Will fail" statements are static readings of the code.

---

## 0. Headline

1. **The three launch files still exist and are now orphans.** The plugin-registry generator no longer writes or reads them (`📽️projection/🟦️.ts` has zero launch hits; `…:check` no longer byte-compares). The frozen-seal ledger already records their deletion (`🧫️frozen-seal-ledger/🔣️.json:150–156`, recordedBy this ticket). Deleting them is still open.
2. **The dashboard no longer reads `.vscode/launch.json` in its runtime** (`🌳️command-tree/🦀️.rs` has no launch hit, the reader functions are gone). **But its unit test still calls the removed helpers**: `🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs:199–237` uses `LAUNCH_CONFIGURATIONS` (lines 203) and `strip_jsonc` (line 237), and neither symbol is defined anywhere in the repo (`git grep -n 'fn strip_jsonc|const LAUNCH_CONFIGURATIONS'` is empty). Static finding: the dashboard crate's test target does not compile (UNVERIFIED by cargo).
3. **The root `📜️script.ts` is the largest remaining code dependent**: `verify interactivity apps` reads both launch files and fails closed when they are missing (`📜️script.ts:8962–8965, 9033–9037`). It must be re-expressed before the files are deleted.
4. **Repo-lib registration tests (doc table A, 54 files) are all clean.** Four doc table-B tests still read the launch files.
5. **Still referencing launch.json in the repo's own rules:** `AGENTS.md:50` (agents may not edit it; owner action).

---

## 1. Launch-data measurements (current HEAD)

| file | bytes | content (measured) |
|---|---|---|
| `.vscode/launch.json` | 3,353,046 | `version` 0.2.0; **4,667 configurations** (all `type: node-terminal`, `request: launch`); **4 compounds**; **82 inputs** (52 `pickString`, 30 `promptString`); 356 with `serverReadyAction`; 2,626 with `env`; 1,049 with a `cwd` other than `${workspaceFolder}`; 155 with `${input:…}`; 1,806 referencing `.🧬semio/🦑️repo/🎫️tickets`; 0 duplicate names. Groups: `4_gate` 3,200 · `3_dev` 635 · `9_gates` 453 · `4_build` 260 · `🧹clean🛡️gates` 29 · none 41 · 11 other groups (sum 49). |
| `.vscode/🧩️launch.seed.jsonc` | 2,768,262 | keys `version, configurations, compounds, inputs, devLaunchers, projectLaunchers`; **3,594 configuration slots** of which **89 are `@generated:` placeholders** (3,505 literal rows); **34 inputs**; **47 devLaunchers**; **7 projectLaunchers**. |
| `.claude/launch.json` | 18,767 | version 0.0.1; **79 entries**: **55 runnable** (`runtimeExecutable` 50 × `bun`, 5 × `bash`) and **24 attach-only** (`url` only, `http://localhost:<port>`); **62 entries carry a `port`**, 17 entries have none; **40 distinct ports**, **17 ports are shared** (e.g. 6061 by 4 entries, 6014 by 4, 6070 by 2, 6013 by 3). **8 entries** point at ticket scripts under `.🧬semio/…/🎫️tickets` (4 `serve-supervisor.sh` supervisors, `gis2d` envelope gate, `map-harness`, `puzzle5d-battery` and `-explore`). Full name/port/command list: `r2-launch-stats.json` (`claudeStats.rows`). |

Repo-level totals: 185 tracked files hit the pattern set = 3 launch files + 64 other files outside `.cursor/plans` + 118 `.cursor/plans/*.plan.md`.

---

## 2. Doc-vs-current status (`launch-dependents.md`, 209 indexed rows, section 8)

| doc section | doc claim | current state (static) | status |
|---|---|---|---|
| 0.1 three data files | `.vscode/launch.json`, seed, `.claude/launch.json` exist | all three exist, counts above | **REMAINS** |
| 0.3 generator writes/gates launch.json | `generate` writes, `check` byte-compares | projection and registry `📜️script.ts` contain no launch hits; `check` no longer compares; `📋️project.json` keeps `generate`/`check`/`check-generated`/`preview-generated` (no launch) | **RESOLVED in code** (files orphaned) |
| 0.4 54 repo-lib registration tests | assert a launch row exists | table A (54 rows of doc §3.6) all clean | **RESOLVED** |
| 0.5a taxonomy `generatorContracts` lists launch files | taxonomy input/output | `🔣️taxonomy.json` has no launch hit (index row L, CLEAN) | **RESOLVED** |
| 0.5b nested-cargo projection asset pins launch occurrence counts | `…📽️nested-cargo-package-projection/🔣️.json` | index row CLEAN | **RESOLVED** (reseal ownership UNVERIFIED) |
| 0.5c `registryCatalogInputPaths` treats `📋️project.json` as catalog input because of launch rows | registry code | no launch hit; the logic itself not re-read | **UNVERIFIED** |
| 0.5d `verify interactivity [apps]` fails closed on a missing launch file | `📜️script.ts` 8962–9037 | **still present** | **REMAINS** |
| 0.5e devcontainer forwardPorts law derived from launch rows | runtime-bootstrap test | **still present** (test reads `.vscode/launch.json`) | **REMAINS** |
| 0.5f tests encode execution policy only in launch rows | e.g. `SEMIO_TEST_LEVEL=quick` | not re-checked | **UNVERIFIED** |
| 0.6 dashboard reads `.vscode/launch.json` as command source | `🌳️command-tree/🦀️.rs` | reader removed; dangling test refs (section 3-B1) | **PARTIAL, broken** |
| 0.7 only AGENTS.md:50 names launch.json | AGENTS.md | still there | **REMAINS (owner)** |
| 3.1 generator module, 16 files | `📇️registry/🚀️launch/…`, `🧪️tests/🚀️launch/…` | all 16 paths missing | **RESOLVED** |
| 3.2 registry wiring + ledgers, 8 rows | projection, script, project.json, discovery, descriptor-verification, features | all 8 clean | **RESOLVED** |
| 3.3 root `📜️script.ts` launch regions | 8 regions | 18 hit lines still (section 3-A) | **REMAINS** |
| 3.4 dashboard `DASH/` rows, 8 rows | reader, tests, fixtures, schema, README | reader and its functions gone; 7 rows still hit, one test broken | **PARTIAL** |
| 3.5 Nx `inputs` lists, 7 project.json files | input lists | 5 files clean (norm, stdio, hub-rust, flow-core, caching); **repo-lib `📋️project.json` (6 lines) and `test/📋️project.json` (2 lines) remain** | **PARTIAL** |
| 3.6 table A, 54 repo-lib tests | registration assertions | 54 clean; none of the 54 in the hit list | **RESOLVED** |
| 3.7 table B, 21 test files + 2 scripts | other tests/scripts | 4 test files remain (runtime-bootstrap, interactivity discovery, package-boundary, command-composition); 1 of 2 scripts (`✏️s/…🧊️3d/…📜️script.ts:33,43` comments) remains | **PARTIAL** |
| 3.8 table C, fixtures/schemas (55) | launch keys | 53 clean; 5 remain (scalar contract + schema, watch-policy, reviewed-expectations, command-composition fixture) | **PARTIAL** |
| 3.9 docs/comments/messages | 29 rows | 22 rows still hit; `README.md` root, dashboard README, quiz/pets/proctor READMEs, presentation/print READMEs, devcontainer README | **REMAINS** |
| 3.10 ignore config | `.gitignore:591` | still there | **REMAINS** |
| 4 editor/agent surface (`.claude`, `.vscode`) | consumers | `.vscode/mcp.json`, `settings.json`, `extensions.json`, `.claude/settings*.json`, `.cursor/*` non-plan, `.codex`, `.agents`, `.github`: **no hits** | **CLEAN** (except the 3 files to delete) |
| 5 AGENTS.md | line 50 | still there | **REMAINS (owner)** |
| 8 index | 209 rows | 16 MISSING (generator) · 142 CLEAN · 51 still HIT | **PARTIAL** |

New since the 10-07 doc (not in its index): `🧫️frozen-seal-ledger/🔣️.json:154–155` (ledger deletion records; keep as evidence until the owner decides about the seal); `📓️print/🔮️oracles/🔣️.json:79` (stale entry naming `📇️registry/🚀️launch.test.ts`, which does not exist); `🌳️command-tree/🧪️tests` (test now references removed helpers); `🧫️fixtures/🎮️registry` and `🎮️registry/🦀️.rs` hits are native `compounds` only (not launch dependents).

---

## 3. Dependents by group (current line numbers)

Action vocabulary: **DELETE** the lines/files · **RE-EXPRESS** against the dashboard registry or declarations (`metadata.semio.dashboard`, `🎮️registry`) · **REWRITE** prose to point at the dashboard · **KEEP** (not a launch dependent).

### A. Code that reads, generates or gates launch files

| file:line | what it does | action |
|---|---|---|
| `📜️script.ts:8717–8718` | constants `INTERACTIVITY_ALL_APP_LAUNCH_FILE = ".vscode/launch.json"` and `…_SEED_FILE` | DELETE with the reader |
| `📜️script.ts:8720–8745` (hits 8720, 8722, 8723, 8732, 8733) | `INTERACTIVITY_ALL_APP_REQUIRED_GATES`: six gate rows that must be registered "exactly once, in the `4_gate` group" in the launch file | RE-EXPRESS: the six gate commands become declared dashboard commands; the law checks the registry, not a file |
| `📜️script.ts:8899–8914` `interactivityAllAppPlaygroundLaunchNames` (reads seed `devLaunchers` at 8906) | derives generated playground launch names from the seed | DELETE (only consumers: 9042 and the test in B6) |
| `📜️script.ts:8920–8935` `interactivityAllAppLaunchCoverageFailures` (seed read 8927; `Bun.JSONC.parse(seedSource)` 8921) | every app context has React/Wasm/native dev surfaces | RE-EXPRESS against dashboard command-tree leaves |
| `📜️script.ts:8956–8990` `interactivityAllAppLaunchesFromSource` (failures 8962, 8965, 8966, 8973, 8976, 8986, 8989) | parses JSONC `configurations`, checks names, required gates, command/cwd | DELETE; replace with a registry reader |
| `📜️script.ts:9033–9056` `interactivityAllAppDiscovery` (9033, 9035–9037, 9040, 9042–9043, 9047–9056) | reads both launch files, computes `launchOnlyProducts` and `surfaceCount` | RE-EXPRESS: surface count from registry leaves; drop `launchOnlyProducts` |
| `📜️script.ts:8075–8140` `runInteractivityAudit` / `runInteractivityApps` (log lines 8081, 8125) | prints launch-derived counts; `verify interactivity apps` throws when the file is missing (static reading) | RE-EXPRESS output on registry counts |
| `📜️script.ts:26178–26189` exports `INTERACTIVITY_ALL_APP_REQUIRED_GATES`, `interactivityAllAppLaunchesFromSource`, `INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY`, `interactivityAllAppPlaygroundLaunchNames` | test imports | DELETE exports with their functions |
| `📜️script.ts:8379` `plugin-registry:check` | runs the registry check inside `verify gate` | KEEP (no launch dependency any more) |
| `🧰️…/📇️registry/📽️projection/🟦️.ts` (no hits) | generator | KEEP; confirms the generator is clean |
| `🧰️…/🎛️dashboard/🌳️command-tree/🦀️.rs` (no hits) | runtime reader removed | KEEP; see B1 for the broken test |
| `🧰️…/📚️library/⚡️caching/…/🚀️runtime-bootstrap/🟦️.ts` | test code, see B5 | — |

### B. Tests and fixtures

| file:line | what it asserts or reads | action |
|---|---|---|
| **B1** `🎛️dashboard/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs:199–237` | test `launch_configurations_targets_and_scripts_are_filed_under_their_verb` (uses `LAUNCH_CONFIGURATIONS` at 203, fixture at 200) and test `launch_documents_tolerate_comments_and_trailing_commas` (uses `strip_jsonc` at 237). **Both helpers are undefined in the repo → does not compile (static).** | DELETE both tests; add a registry test on `🎮️registry` fixtures |
| B2 `🎛️dashboard/🧫️fixtures/🚀️launch-configurations/🔣️.json:2` | frozen JSONC vector with compounds, `${input:}` defaults | DELETE; replace by a registry fixture |
| B3 `🎛️dashboard/🧫️fixtures/🚀️launch-configurations/🥒️.feature:1, 2, 21` | Gherkin "Every launch configuration … is offered under its verb" | REWRITE scenario to "declared command" wording, then delete the file with B2 |
| B4 `🎛️dashboard/🧪️tests/🌀️control-plane/🟦️.ts:56–82` | third-party `jsonc-parser` oracle of the B2 fixture (58 read, 65 `node-terminal`, 72 compounds loop) | RE-EXPRESS: keep an external-library oracle, but over the registry format |
| B5 `🎛️dashboard/🌀️daemon/🧪️tests/🔬️unit/🦀️.rs:38–73` (`#[ignore]`, needs native CLI) | writes `.vscode/launch.json` fixture (43–47), drives the TUI, prints (73) | RE-EXPRESS: write a registry declaration fixture instead |
| B6 `📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts:15, 19, 55, 57, 59, 65` | devcontainer `forwardPorts` must equal the ports of launch rows read from `.vscode/launch.json` | RE-EXPRESS: forwarded ports derive from dashboard declarations (see D6) |
| B7 `📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts:1, 52, 65–67` | imports `INTERACTIVITY_ALL_APP_REQUIRED_GATES`/launch types; seed fixture `devLaunchers` (52); asserts launch-name classification (65–67) | DELETE launch cases; keep discovery cases |
| B8 `📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts:802` | loops over both launch files | DELETE the two paths |
| B9 `🧰️…/🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts:213, 227–228, 233, 235–236` | asserts `launchName` appears once in both launch files and contains `launchCommand` | RE-EXPRESS against dashboard declaration, or DELETE the launch half |
| B10 `🧰️…/🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json:332–333` | `launchName`, `launchCommand` keys | DELETE keys (with B9) |
| B11 `📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json:9–12` | `launchName`, `launchCommand`, `launchGroup`, `launchOrder` (410.192) | DELETE keys; consumer not identified by grep (UNVERIFIED) |
| B12 `💻️os/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json:11` | watch list contains `.vscode/launch.json` | DELETE entry |
| B13 `📓️print/🔮️oracles/🔣️.json:79` | lists `📇️registry/🚀️launch.test.ts`, which does not exist | DELETE entry (stale) |
| B14 `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json:154–155` | `consumerDeletions` rows recording the removal of both files (recordedBy this ticket) | KEEP as evidence; owner decides whether to re-seal after deletion (seal rules UNVERIFIED) |
| B15 `🎛️dashboard/🧫️fixtures/🎮️registry/🏗️workspace.json:89, 348` | native `compounds` | KEEP (not a launch dependent) |

### C. Docs, READMEs, comments and messages

| file:line | what it says | action |
|---|---|---|
| `README.md:660, 661, 663, 863` | "Every one of these is a `.vscode/launch.json` row" + table "what / launch row(s) / nx target" + "Each one also has a `.vscode…`" | REWRITE to dashboard entries |
| `🎛️dashboard/README.md:9, 22, 27, 29, 31, 103, 128, 143` | launcher paragraph, "configuration and compound of `.vscode/launch.json`", "No editor launch configuration is required", fixture `🚀️launch-configurations`, "runs the selected launch configuration" | REWRITE (dashboard is the only entry point) |
| `🎓️teaching/README.md:63` | "Validate with the launch rows of the site" | REWRITE |
| `🎓️teaching/🏛️architecture/README.md:11` | "via the launch rows in [its README]" | REWRITE |
| `🎓️teaching/🏛️architecture/❓️quiz/README.md:44, 46, 159, 201` | "Run everything from `.vscode/launch.json` (groups `3_dev` and `4_gate`)"; table of launch rows with ports; Claude preview row; "Launch rows: `🚚️publish…`" | REWRITE tables to dashboard command paths |
| `🎓️teaching/🏛️architecture/🐾️pets/README.md:358, 359` | "Claude preview (`.claude/launch.json`)" and "VS Code (`.vscode/launch.json`, group `3_dev`)" | REWRITE (the only documented consumer of `.claude/launch.json`) |
| `🎓️teaching/🛂️proctor/README.md:285, 288, 289, 290, 291, 292, 295` | command table with "(launch row …)" annotations | REWRITE annotations to dashboard paths |
| `🎓️teaching/🏛️architecture/❓️quiz/🧱️stack/🟦️.ts:20` | `@see ../README.md — the launch rows` | REWRITE |
| `🌎️hub/README.md:213` | "(launch rows …)" | REWRITE |
| `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:105` | comment "whichever launch row reaches a clean `hub-dev` root" | REWRITE |
| `🌎️hub/🤝️integration-harness/🟦️.ts:190` | error message "launch row 🛠️dev🗄️os-hub publishes…" | REWRITE message |
| `✏️s/🧑‍💻dev/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:297` | error message "Start one first (launch row …)" | REWRITE message |
| `✏️s/🧑‍💻dev/💡️services/🧪️tests/💬️agent-reply/🟦️.ts:143` and `…/🤖️live-agent-loop/🟦️.ts:225` | "the gate drives the shipped launch line" | REWRITE wording (optional) |
| `🧰️…/🛍️products/🎤️presentation/README.md:22` and `📓️print/README.md:22` | "`.vscode/🧩️launch.seed.jsonc` as launch configurations" | REWRITE |
| `🧰️…/🔨️modules/🖼️assets/README.md:9` and its generator `🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts:326` | "run the assets build launch configuration" | REWRITE the generator text, then regenerate the README |
| `✏️s/🔌️plugins/🌀️procedural/README.md:12` | "`🛠️dev🔧️procedural…⚛️react` launch configuration" | REWRITE |
| `✏️s/🔌️plugins/🔋️energy/…/🐍️.py:80` | "(or the `…setup` launch entry)" in a message | REWRITE message |
| `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:266` | "run it through the `…🔮️compare` launch entry" | REWRITE comment |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts:33, 43` | comments about "the launch.json entries that call them" | REWRITE comments |
| `🧰️…/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts:24` | "used by every other os-dev variant/launch.json entry" | REWRITE comment |
| `🧰️…/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts:9` | "Which launch row reaches a clean data root first" | REWRITE comment |
| `🧰️…/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts:817` | message "launch row 🛠️dev🪐️space⚛️react🔒local-only" | REWRITE message |
| `📚️library/🎮️playground/🔒️preferences/🟦️.ts:13` | "no artifact's read-only surface was reachable from any launch…" | REWRITE comment |
| `📚️library/🟦️.ts:2002` | "`SEMIO_RENDERER` (launch row, `extra`)" | REWRITE comment |
| `📜️script.ts:137, 144, 6707, 8710, 8711, 8715, 8722, 8723, 8732` | comments on launch.json reachability, rows, the six gates | REWRITE/DELETE with the reader |
| `.devcontainer/README.md:11, 59` | "forwards exactly the ports the launch rows start"; "Forwarded ports MUST equal the ports the launch rows start" | REWRITE |

### D. Editor, agent, ignore and devcontainer configs

| file:line | what it is | action |
|---|---|---|
| `.vscode/launch.json` | 4,667 `node-terminal` configs, 4 compounds, 82 inputs (section 1) | **DELETE** |
| `.vscode/🧩️launch.seed.jsonc` | authored seed: 3,594 slots (89 placeholders), 34 inputs, 47 devLaunchers, 7 projectLaunchers | **DELETE** |
| `.claude/launch.json` | 79 Claude preview entries (55 runnable, 24 attach-only) | **DELETE** (its consumers are in C: pets README 358, quiz README 159) |
| `.gitignore:591` | `!.vscode/launch.json` un-ignore rule | **DELETE** line after the file is gone |
| `.devcontainer/devcontainer.json:48` | `forwardPorts: [8787, 6070, 6072, 6073, 6066, 6067, 6068, 6010, 6274, 6277]` | KEEP the value; make the list derive from dashboard declarations (B6) |
| `.devcontainer/README.md:11, 59` | prose tying forwarded ports to launch rows | REWRITE |
| `AGENTS.md:50` | "All devs are using `launch.json` and never use the cli." | **OWNER ACTION** (agents may not edit AGENTS.md) |
| `.vscode/settings.json`, `.vscode/mcp.json`, `.vscode/extensions.json`, `.claude/settings.json`, `.claude/settings.local.json`, `.cursor/hooks.json`, `.cursor/mcp.json`, `.codex/config.toml`, `.agents/**`, `.github/**` | scanned | CLEAN, no action |

Outside scope but noted: `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` lines 2876, 4969, 12794 mention `launch.json` (prompt file inside the excluded ticket tree; owner decides).

### E. `.cursor/plans/*.plan.md` (historical plans, count only)

- 429 plan files in the tree; **118 files contain a hit**; 238 hit lines (about 269 individual terms: 269 `launch.json`, 11 `launch.seed`, 6 `launch entry`, 3 `launch configuration`, 2 `serverReadyAction`, 1 `runtimeExecutable`).
- Action: none. Historical planning records; do not edit. Full file list: `🗑️generated/r2-cursor-plans-hit.txt` (appendix below).

### F. Other (build config, contracts, schemas, name-only)

| file:line | what it is | action |
|---|---|---|
| `🧰️…/📚️library/📦️packages/🟦️typescript/📋️project.json:55, 56, 570, 571, 599, 600` | Nx `inputs` naming both launch files (targets `test-*` in repo-lib) | DELETE the 6 input lines |
| `🧰️…/🧪️test/📋️project.json:427, 428` | Nx input list naming both launch files | DELETE |
| `🧰️…/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json:3595` and `…/🧬️schema/🔣️.json:3683` | norm prose "taxonomy-owned seed and generated launch row" | REWRITE prose (contract text; reseal rules UNVERIFIED) |
| `🧰️…/🎛️dashboard/🧬️schema/🔣️.json:15` | description "workspace scripts or launch configurations" | REWRITE |
| `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts:78` and `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts:69` | Vite watch `ignored` globs contain `**/.vscode/launch.json` | DELETE the glob entry |
| `✏️s/🔌️plugins/🔱️trinity/…/🧠️lsp/📦️packages/🦀️rust/🦀️.rs:2` | crate doc "compatibility shim for existing launch targets" (LSP, not launch.json) | name-only; flag to owner (AGENTS forbids compatibility layers; separate ticket) |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json:1077` `local-bootstrap-launch-check` | Nx target name only | name-only, no action |
| `📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:760–911` `componentLaunchers` | variable name, plugin components | name-only, no action |
| `🎛️dashboard/🎮️registry/🦀️.rs:366, 370, 917, 962, 1003–1005, 1046, 1216, 1347` and `🧬️schema/🎮️registry/🔣️.json:202, 206, 213, 217` | native declaration `compounds` (dashboard registry) | KEEP |
| root `📋️project.json:342` and `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📋️project.json:6` | `metadata.semio.dashboard.compounds` (verified by JSON walk) | KEEP |

Pattern false positives (checked, excluded): PDF `/S /Launch` actions and `CODE_LAUNCH` (`✏️s/🔌️plugins/🗄️stdio/…`), `chromium.launch` / playwright, `launch-date` in the architect composition, pets "launch" projectile, `RecordCompoundStep`/`compound_value` (DSL), BRep "compounds" (`🧊️3d`), `…r3f` "compounds", `.claude/plans/*.md` references in Rust/Go docs (Claude plan files, not launch.json), MCP "launch" process wording.

---

## 4. Required removal order (dependency-safe)

1. Re-express the root gate (A, 8717–9056) against the dashboard registry; keep the six gate commands as declared dashboard commands.
2. Re-express or delete B1–B13; delete the dangling command-tree tests first so the dashboard test target compiles again.
3. Replace the devcontainer port law (B6) with a declaration-derived check; keep `forwardPorts` values unchanged until then.
4. Remove the F-list Nx inputs, the vite watch globs and the D `.gitignore` line.
5. Rewrite the C docs (incl. generated assets README via its builder).
6. Delete `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json` last, then re-run `verify interactivity` and the root gate (not run; UNVERIFIED).
7. Owner: `AGENTS.md:50`, the frozen-seal ledger entries (B14) and the scalar contract prose (F).

---

## Appendix: `.cursor/plans` files with hits (118, group E)

- .cursor/plans/2d_references_on_grid_ff798114.plan.md
- .cursor/plans/adaptive_flow_preview_ae973235.plan.md
- .cursor/plans/app_isolation_and_boundaries_b0a20173.plan.md
- .cursor/plans/architect_query_lang_c13595b6.plan.md
- .cursor/plans/architect_technology_11ce537e.plan.md
- .cursor/plans/artifact_io_facet_c8c44e6f.plan.md
- .cursor/plans/artifact_schema_facets_7af7fc96.plan.md
- .cursor/plans/block3d_type_editor_62d3aa52.plan.md
- .cursor/plans/brep_suite_procedural_eaf0eeaa.plan.md
- .cursor/plans/brush_kind_suggestion_weights_7ae14c60.plan.md
- .cursor/plans/budgeted_repo_binary_tooling_0539a411.plan.md
- .cursor/plans/bun_nx_monorepo_setup_756984c2.plan.md
- .cursor/plans/cad-generalize-modeldefs_0dfb6842.plan.md
- .cursor/plans/cad_step_shape_fixtures_9401982e.plan.md
- .cursor/plans/centralized_styling_tokens_b2c59cc8.plan.md
- .cursor/plans/channel_3d_sync_39385f72.plan.md
- .cursor/plans/complete_procedural_3d_54299d2c.plan.md
- .cursor/plans/composable_window_option_trees_afd215c8.plan.md
- .cursor/plans/compose_rust_removal_+_full_testing_98fa0924.plan.md
- .cursor/plans/configured_node_apps_30251998.plan.md
- .cursor/plans/context_menu_keyboard_nav_7363b23b.plan.md
- .cursor/plans/corner_window_chips_f935d2ca.plan.md
- .cursor/plans/dashboard_tui_workforce_775ac26d.plan.md
- .cursor/plans/dashboard_wizard_windows_f834d098.plan.md
- .cursor/plans/decouple_sketchpad_from_metabolism_8e20eed7.plan.md
- .cursor/plans/demonstrator_2x3_grid_b1bf0f7d.plan.md
- .cursor/plans/dev_build_opt_profiles_767e588d.plan.md
- .cursor/plans/display_panel_mechanism_c2f7c2e6.plan.md
- .cursor/plans/dissolve_cores_plugin_contract_57e786f6.plan.md
- .cursor/plans/domain-driven_artifact_specs_66ddc332.plan.md
- .cursor/plans/emoji_unlock_and_repo_11ab8751.plan.md
- .cursor/plans/energy_engine_technology_03fd4ca0.plan.md
- .cursor/plans/enforce_panel_tab_tree_section_f8c26471.plan.md
- .cursor/plans/enforce_styling_by_api_413a1c4d.plan.md
- .cursor/plans/engagement_ui_controls_7c1f3a21.plan.md
- .cursor/plans/example_shape_refactor_adb7b675.plan.md
- .cursor/plans/fill_target_volumes_60b33937.plan.md
- .cursor/plans/fix_brep_geometric_operations_714b1f51.plan.md
- .cursor/plans/fix_layout_document_crash_7a2c626d.plan.md
- .cursor/plans/flow_catalogue_dnd_2ffa4bf9.plan.md
- .cursor/plans/flow_undo_redo_6e12141c.plan.md
- .cursor/plans/folder_structure_migration_2c56af58.plan.md
- .cursor/plans/framework_core_abstraction_9427fd48.plan.md
- .cursor/plans/framework_icon_interface_de631a59.plan.md
- .cursor/plans/full_virtual_file_system_b2678c72.plan.md
- .cursor/plans/generalize_infinite_world_ab04cb15.plan.md
- .cursor/plans/generalize_rich_ui_engines_fb5319c1.plan.md
- .cursor/plans/gis-map-infinite-canvas_74114735.plan.md
- .cursor/plans/gis-map-reuse-pins_a017207e.plan.md
- .cursor/plans/global-ghost-interaction_b1a1cd0d.plan.md
- .cursor/plans/i18n_compile_enforcement_854a780a.plan.md
- .cursor/plans/imperative_and_sequence_technologies_2fa78baa.plan.md
- .cursor/plans/infinite_world_hide_lock_78af5297.plan.md
- .cursor/plans/language_neutral_taxonomy_2a68dcd2.plan.md
- .cursor/plans/layer_graph_crates_d07eda66.plan.md
- .cursor/plans/lowpoly_mesh_kernel_technology_302c4f57.plan.md
- .cursor/plans/map_vector_tile_labels_c099c62c.plan.md
- .cursor/plans/map_vector_tiles_05c372b0.plan.md
- .cursor/plans/mathematical_fuzzy_bundle_4d97a786.plan.md
- .cursor/plans/mit-bestand_demonstrator_generalization_b51de4c8.plan.md
- .cursor/plans/mobile_demonstrator_list_728d9c60.plan.md
- .cursor/plans/morph_ghosts_generalization_f055ab19.plan.md
- .cursor/plans/mutations_refactor_bc66e12b.plan.md
- .cursor/plans/native_brep_kernel_workforce_e2f133ed.plan.md
- .cursor/plans/neural_schemas_and_operators_f692e604.plan.md
- .cursor/plans/note_infinite_canvas_app_3c2e10ae.plan.md
- .cursor/plans/operator-typed_brep_nodes_a382c9bc.plan.md
- .cursor/plans/os_framework_and_composable_apps_857cb11f.plan.md
- .cursor/plans/os_state_authority_bd6b04e2.plan.md
- .cursor/plans/panel-ghost-interaction_dc3e8955.plan.md
- .cursor/plans/per-artifact_grammars_and_protocols_8b0fe9ad.plan.md
- .cursor/plans/play_document_tabs_b3050808.plan.md
- .cursor/plans/playground_technology_registry_9cdd8572.plan.md
- .cursor/plans/pluralize_kind_folders_26a96337.plan.md
- .cursor/plans/presentation_framework_69342862.plan.md
- .cursor/plans/print_latex_technology_c77413ae.plan.md
- .cursor/plans/procedural-3d-hardening_bb9c0925.plan.md
- .cursor/plans/procedural3d-extension-node-discovery_a9c2972c.plan.md
- .cursor/plans/procedural_feature_complete_1603207d.plan.md
- .cursor/plans/procedural_puzzle_selection_128fd91a.plan.md
- .cursor/plans/procedural_schema_component_36a50064.plan.md
- .cursor/plans/projektage_reveal_presentation_44ae3b87.plan.md
- .cursor/plans/puzzle3d_precompute_worker_afb88899.plan.md
- .cursor/plans/puzzle_5d_design_parity_61aab236.plan.md
- .cursor/plans/puzzle_5d_unified_tools_f87040c9.plan.md
- .cursor/plans/puzzle_fill_engagement_695e66a7.plan.md
- .cursor/plans/puzzle_playground_relayer_5d67554e.plan.md
- .cursor/plans/raw_wgpu_renderer_abf93943.plan.md
- .cursor/plans/repo_emoji_path_consistency_21004b8c.plan.md
- .cursor/plans/restore_flow_wgpu_rendering_parity_6d0f4468.plan.md
- .cursor/plans/restore_playgrounds_app_split_aa944cd3.plan.md
- .cursor/plans/running_goal_tree_87b5a8ce.plan.md
- .cursor/plans/runtime_installable_extensions_6562017a.plan.md
- .cursor/plans/rust-only_apps_migration_861de551.plan.md
- .cursor/plans/rust_plugin_framework_migration_a0decb35.plan.md
- .cursor/plans/s_as_collaborative_os_c379b520.plan.md
- .cursor/plans/s_studio_full_parity_port_145ba465.plan.md
- .cursor/plans/semio_format_and_examples_6b665da9.plan.md
- .cursor/plans/semio_root_data_layout_3f4d0861.plan.md
- .cursor/plans/settings_panel_host_parity_66f26970.plan.md
- .cursor/plans/single_side_panel_tree_mechanism_ff8a1f6f.plan.md
- .cursor/plans/split_puzzle_2d_bundles_17a101da.plan.md
- .cursor/plans/stdio_artifacts_io_b8e0ef62.plan.md
- .cursor/plans/strict_side_panel_tree_api_a05d2020.plan.md
- .cursor/plans/subset_conformance_roundtrips_c57a3e1a.plan.md
- .cursor/plans/systematic_pixel_sizing_40a3580d.plan.md
- .cursor/plans/token_color_consistency_b28a212a.plan.md
- .cursor/plans/tree_operator_clusters_6eac66ce.plan.md
- .cursor/plans/trinity_rewrite_6-window_playground_52a31797.plan.md
- .cursor/plans/true_full-loc_s_parity_00cd64f1.plan.md
- .cursor/plans/ui_theme_and_settings_consistency_bf440ed0.plan.md
- .cursor/plans/unified_gumball_ee2b0dff.plan.md
- .cursor/plans/unify_package_names_a0fcf287.plan.md
- .cursor/plans/variadic_flow_ports_14fa63b8.plan.md
- .cursor/plans/wgpu_winit_trunk_migration_e5614dfc.plan.md
- .cursor/plans/wires_kit_relationship_view_e12b40b3.plan.md
- .cursor/plans/wires_normal_graph_03c36798.plan.md
- .cursor/plans/writer_technology_editor_4b5f0141.plan.md
