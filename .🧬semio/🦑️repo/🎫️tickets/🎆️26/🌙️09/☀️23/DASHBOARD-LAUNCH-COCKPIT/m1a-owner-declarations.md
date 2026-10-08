# M-1a: Owner Declarations

Slice M-1a of `fleet-plan.md`: everything worth keeping from `.vscode/launch.json`, its seed and
`.claude/launch.json` is declared next to its owner in the registry form of fleet-plan §2.2. No launch file,
generator, root `📜️script.ts`, dashboard Rust source or `AGENTS.md` was touched.

## 1. Result

- **105 hand-written declarations in 19 owner manifests** replace the 1,003 retained rows: 7 global axes,
  73 target declarations (32 with `ready`, 15 with `requires`, 42 with `parameters`), 10 tools, 4 compounds,
  11 groups. The inventory expected about 141.
- **Coverage: 1,003 of 1,003 retained rows covered, 0 uncovered.** 610 resolve exactly, 364 are equivalent
  (spelling, runner noise, repeated owner defaults), 29 are covered with a recorded deviation (section 7).
  The four VS Code compounds and all 79 `.claude/launch.json` entries are mapped.
- **Ajv: 20 of 20 edited manifests are valid** against `#/$defs/ProjectManifest` of
  `🎛️dashboard/🧬️schema/🎮️registry/🔣️.json` (sha256 `7148b84bf41fca4f…`, read at 19:27). The validator was
  proven live with three negative cases (unknown tool key, `ready` without a port, a runner-noise variable).
- **Nothing was run through Nx.** Effective target shapes were read from the manifests, the owners' scripts and
  the cached graph (`.nx/workspace-data/project-graph.json`, computed 19:36, before the edits). The registry
  resolver A-1 is writing has not seen these declarations yet: the coverage file states what it must produce.

Proof and inputs (all in this ticket folder):

| File | Role |
| --- | --- |
| `m1a-declarations.ts` | the hand-written declarations as data, one entry per owner manifest |
| `m1a-apply.ts` | surgical insert (`--write`), idempotent; refuses a manifest whose other members would change |
| `m1a-coverage.ts` | independent §2.2 resolver, row mapping, comparison, Ajv validation; `--check` exits non-zero on any gap |
| `🗑️generated/declarations/coverage.json` | per row `{ id, parameters, extraArgs }`, expected `cmd` / `args` / `env` / `ready` / `requires`, match class, differences; compounds; `.claude` entries; validation result |

Reproduce: `bun <ticket>/m1a-apply.ts` (reports `unchanged` 20 times) and
`bun <ticket>/m1a-coverage.ts --check` (exit 0).

## 2. Declarations Per Owner Manifest

"Rows" is the number of retained launch rows that resolve to the entry.

| Manifest | Entry | Kind | Rows |
| --- | --- | --- | --- |
| `📋️project.json` (`workspace`) | `cache`, `test-level`, `dependencies`, `build-mode`, `nextest-output`, `cargo-jobs`, `build-budget` | global axes | 518, 439, 102, 102, 115, 41, 17 |
| | `repo-mcp`, `os-mcp-stdio`, `os-mcp-http`, `mcp-inspector-os`, `bun-test`, `native-cargo`, `gemini`, `kiro`, `f3d`, `gitkraken` | tools | 2, 1, 1, 1, 81, 1, 1, 1, 1, 1 |
| | `s-with-hub`, `s-with-os-mcp`, `s-users-with-hub` | compounds | 3 VS Code compounds |
| | `process-extension-catalogs`, `sourcing-extension-catalogs`, `extension-catalogs`, `process-extension-tests`, `sourcing-extension-tests`, `stdio-artifact-tests`, `puzzle-spatial-tests`, `snapshot-sqlite-parent-baselines`, `snapshot-sqlite-native`, `snapshot-sqlite-source`, `dag-actor-wasm` | groups | 12 |
| | `dev-storybook` and its 12 scoped siblings | `ready` 6010 / `STORYBOOK_PORT` | 12 |
| | `dev-mcp`, `dev-mcp-repo` | `ready` 6274 / `CLIENT_PORT` | 2 |
| | `verify` | parameter `check` (text, positional) | 11 |
| `🎛️dashboard/📦️packages/🦀️rust/📋️project.json` | `daemon` | parameter `action` start, stop, status, attach | 2 |
| | `preferences` | parameter `action` show, set | 1 |
| | `run` | parameter `repo-implementation` rust, go | 3 |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json` (`os-hub`) | `dev` | `ready` 8787 / `OS_HUB_PORT` / `/admin` | 1 |
| | `dev-postgres`, `dev-neo4j` | same `ready` + parameter `data` (`OS_HUB_DATA`) | 2 |
| | `dev-secure-suite`, `-native`, `-mcp`, `-admin` | `ready` 8787 / `OS_HUB_PORT` | 4 |
| | `trusted-catalog-preflight`, `trusted-catalog-bootstrap` | parameter `packages` (`--packages`) | 2 |
| `🌎️hub/📦️packages/🟦️typescript/📋️project.json` (`os-hub-ts`) | `two-client-e2e`, `document-growth-e2e` | parameter `backend` sqlite, postgres, neo4j (required) | 6 |
| | `backend-up`, `backend-down`, `backend-status` | parameter `backend` all, postgres, neo4j | 3 |
| | `backend-run` | parameter `backend` postgres, neo4j (required) | 6 |
| | `residency-watch`, `hub-freshness`, `agent-ceiling-check` | `requires` hub + parameter `hub` (+ `locale`) | 3 |
| `🌎️hub/🔨️modules/🛡️admin/…/📋️project.json` | `dev` | `ready` 8790 / `OS_HUB_ADMIN_DEV_PORT`, `requires` hub | 1 |
| `💻️os/🔨️modules/🧑‍💻dev/…/📋️project.json` (`framework-os-dev`) | `program-matrix`, `tool-run-matrix`, `io-matrix` | parameters `serve`, `locale` (+ `roles`) | 6 |
| | `hub-document-sweep`, `two-human` | `requires` hub + parameters `serve`, `hub`, `locale` (+ `users`) | 4 |
| | `connection-budget`, `idle-budget`, `memory-soak`, `interaction-latency` | `requires` shell + parameter `serve` | 4 |
| | `time-travel` | parameter `renderer` react, wgpu | 2 |
| | `s-host-foreign-kind-s`, `s-host-pinch-diagram-contrast-s` | `requires` shell | 2 |
| `💻️os/📦️packages/🦀️rust/📋️project.json` (`framework-os-kernel`) | `wal-writer-fence-live` | parameter `lane` sqlite, postgres, neo4j | 1 |
| | `reopen-storm-check` | parameter `law` all, unit, fs, sqlite, postgres, neo4j | 1 |
| `✏️s/🧑‍💻dev/💡️services/…/📋️project.json` | `user-path-check` | `requires` hub + shell, parameters `hub`, `serve`, `locale` | 2 |
| | `security-check`, `inference-quartet-check` | `requires` hub, parameters `hub` (+ `admin-capability`) | 2 |
| | `live-agent-loop-check` | parameters `serve`, `locale` | 1 |
| `🦑️repo/🔨️modules/🧪️test/📋️project.json` | `acceptance-goal` | parameters `hub`, `serve`, `local-serve`, `users` | 1 |
| `📓️print/…/📋️project.json` | `test-native-grammar` | parameters `phase`, `diagram-family`, `geo-palette-phase`, `geo-planar-phase`, `geo-planar-source` | 66 |
| `🎓️teaching/🛂️proctor/…/📋️project.json` | `dev` | `ready` 8791 / `PROCTOR_PORT` | 1 |
| | `restore`, `erase`, `prune` | parameters `file`; `handle`, `tag`, `learner`, `dry-run`; `older-than`, `dry-run` | 5 |
| `🎓️teaching/🏛️architecture/❓️quiz/…/📋️project.json` | `quiz-with-proctor` | compound | 1 VS Code compound |
| | `dev`, `dev-site` | `ready` 6061 / `TEACHING_ARCHITECTURE_QUIZ_PORT` | 2 |
| `🐾️pets/🎯️targets/⚛️react/…/📋️project.json` | `dev` | `ready` by `PETS_STORIES_PORT`, parameters `port`, `menagerie` | 2 |
| `♻️mit-bestand/…/📅️33.projektetage/…/📋️project.json` | `dev` | `ready` 6050 / `PRAESENTATION_PROJEKTETAGE_PORT` | 1 |
| `♻️mit-bestand/🧺️demonstrator/📋️project.json` | `dev`, `serve` | `ready` 6029 / `MIT_BESTAND_DEMONSTRATOR_PORT` | 1 |
| `🏢️semio-tech/🎡️play/📋️project.json` | `dev`, `serve` | `ready` 6033 / `SEMIO_TECH_PLAY_PORT` | 1 |
| `📺️renderer/…/🧊️wgpu/…/📋️project.json` | `native-release` | parameter `variant` (text, positional) | 1 |
| `🌊️flow/🧩️extensions/📐️brep/…/📋️project.json` | `canonical-architecture` | flag `oracle-only` | 1 |
| `🌀️procedural/🗿️artifacts/🧊️generation3d/…/📋️project.json` | `semantic-wire-check` | flag `native` | 2 |
| `✏️s/🧑‍💻dev/📐️cad/…/📋️project.json` | `test`, `test-quick`, `test-long`, `test-exhaustive` | dead `CAD_JS_RENDERER_PLAY_PORT` removed | - |

The other 665 target rows (413 targets) need no declaration: each is a discovered Nx target plus global axes and
free extra arguments (test filters, files, cargo scopes, `--tag`, `plugin -- size`, `os -- run`, …).

## 3. Global Axes

| Axis | Kind | Values and effect | `appliesTo.verbs` (where the retained rows used it) |
| --- | --- | --- | --- |
| `cache` | choice, default `use` | `use`; `skip-local` = `--skip-nx-cache`; `skip-all` = both skip flags | test, check, build, verify, lint, serve, activate, task |
| `test-level` | choice | `quick`, `long`, `exhaustive` = `SEMIO_TEST_LEVEL` | test, verify, check |
| `dependencies` | flag | `--excludeTaskDependencies` | test, verify, check, task |
| `build-mode` | choice | `ship` = `SEMIO_BUILD_MODE=ship` + both skip flags | test, verify, build, task |
| `nextest-output` | choice | `immediate`, `immediate-final`, `final`, `never` = `NEXTEST_SUCCESS_OUTPUT` | test |
| `cargo-jobs` | text | `CARGO_BUILD_JOBS` (any positive integer; rows used 1 and 2) | test, check, task |
| `build-budget` | text | `SEMIO_BUILD_BUDGET_MS` (any millisecond count, `readBudget`; rows used 1800000 and 3600000) | test, serve, activate, task |

All seven carry `playground: false`: no playground row used one. `task` is the verb of targets whose leading
word is not a launcher verb (`describe`, `component-dev`, `semantic-wire-check`, `wasm`, …).

- **Test level, one spelling.** The owner (`🏃️process/🧪️testing/🎚️budget/🟦️.ts`) knows four levels;
  `fundamental` is what an unset variable means, so the axis has no default and three values. Every one of the
  57 base targets that used the positional spelling or a level-named target reaches `resolveTestLevel`: 28 call
  it in their own script, 28 through `runArtifactRustPackageMain` → `runArtifactRustTests`, one does both. It
  takes the positional level when present and `SEMIO_TEST_LEVEL` otherwise. **No target failed the check.**
  Level-named targets (`test-quick`, `test-long`) pin the level in their own command and stay plain targets.
- `SEMIO_TEST_LEVEL=quick` also sat on six `build` rows of TypeScript plugin packages; their build scripts do not
  read it (checked for forms, energy, dag). `build` is therefore not in the axis and those rows drop the variable.
- `output-style` (29 rows) and `--parallel` (11) are not axes: the dashboard owns output rendering and runner
  settings. `renderer`, `example`, `user-slot`, `app-role` are built into the registry and not declared.
- A global axis reaches "commands Nx runs" only (schema `AppliesTo`). The `bun-test` tool therefore owns a
  `build-mode` parameter of its own (26 rows set `SEMIO_BUILD_MODE=ship`).

## 4. Target Declarations: Where The Facts Come From

**Ready ports** are owner facts, never launch values: `options.env` of the manifest (storybook, quiz,
projektetage, demonstrator, play), a script default (`OS_HUB_PORT` 8787 in `📚️library/🎮️playground`, hub admin
8790, pets 6069, proctor `PROCTOR_DEV_PORT` 8791) or the tool's own default (MCP inspector 6274). In 23 of
the 24 dev-server rows the launch value equals the owner default, so `port` + `portEnv` is declared; the
resolver exports the variable, which reproduces the rows. The exception is pets: a second gallery runs beside
the first on 6074 with another `PETS_MENAGERIE`, so its port is a parameter (default 6069).

**`requires`** was declared where the owner code needs a server it does not start itself, which is fewer rows
than the inventory's 34:

| Rows | Target | Declared | Owner fact |
| --- | --- | --- | --- |
| 4 | `connection-budget`, `idle-budget`, `memory-soak`, `interaction-latency` | `playground:s` | take a serve URL, no fixture |
| 2 | `s-host-*` | `playground:s` | probe scripts take a shell URL |
| 4 | `hub-document-sweep`, `two-human` | `os-hub:dev` | the serve is self-provisioned and joins `--hub`, which must answer |
| 3 | `residency-watch`, `hub-freshness`, `agent-ceiling-check` | `os-hub:dev` | `--hub` is required |
| 4 | `user-path-check`, `security-check`, `inference-quartet-check` | `os-hub:dev` (+ `playground:s`) | "ALREADY RUNNING" in each gate's header |
| 1 | `os-hub-admin:dev` | `os-hub:dev` | proxies `OS_HUB_URL` |
| 8 | `program-matrix`, `tool-run-matrix`, `io-matrix`, `time-travel` | none | `withDevServe` / `ensureDevServe` reuses a serve that answers or starts and stops its own |
| 1 | `acceptance-goal` | none | every option is optional; the gate stands up its own providers |
| 1 | `os-hub:local-hub-owner` | none | it is the owner of port 8787 itself |
| 6 | `s` playground rows | none | `dev s` owns the default hub when `S_HUB_URL` is unset (`ensureDevLocalHub`) |

**Value domains** were read from the owners: `backend` from the usage strings of `os-hub-ts` (`sqlite | postgres
| neo4j`, `postgres | neo4j | all`), `lane` / `law` from `WAL_WRITER_FENCE_LANES` and `REOPEN_STORM_LAWS`,
`action` from the dashboard's `daemon::run` and `preferences::run`, `locale` `en | de` from each gate,
`geo-planar-phase` `guard | admission` from the grammar test, proctor options from the router's usage line.
Three domains are open by nature and stay `text`: the 63 sub-commands of `workspace:verify` (first word as
`check`, the rest as extra arguments), the 56 `PRINT_NATIVE_GRAMMAR_PHASE` names, and the playground variant of
`native-release`. Copying those lists into a manifest would be bulk that drifts with every new check.

**`continuous`:** no flag was added. All 32 targets that now declare `ready` are `continuous: true` in the cached
graph, as are `os-hub:local-hub-owner`, `repo-dashboard-rs:daemon` and `framework-renderer-wgpu:native[-release]`.
Fifteen of them (the 13 `dev-storybook*` and 2 `dev-mcp*` root targets) get the flag from the Nx plugin, not from
the manifest: a registry that reads manifests only would miss it, but each now declares `ready`, which §2.3 also
counts as long-running. Since no flag was added, no dependent task changes how it runs.

## 5. Tools, Compounds, Groups

- **Tools** (root project). `repo-mcp` and `os-mcp-stdio` start the stdio servers directly through the root
  script, because nothing but the protocol may reach stdout; `os-mcp-http` keeps the Nx form of its row.
  `mcp-inspector-os` is the one inspector row without an Nx target; the other two inspector rows are the
  existing targets `workspace:dev-mcp` and `workspace:dev-mcp-repo`. `bun-test` is the single replacement for
  81 `nx exec … bun test <file>` rows (`project` token, required `file`). `native-cargo` is the cargo wrapper
  one row called. `gemini`, `kiro`, `f3d`, `gitkraken` are the external tools.
- **No literal token is stored.** `MCP_PROXY_AUTH_TOKEN=repo-mcp-token` and `MCP_AUTO_OPEN_ENABLED=false` are
  gone; the inspector generates a session token and opens its own tokenised URL.
- **Compounds**, `stop: "together"` as `stopAll: true` was: `s-with-hub` (hub → shell), `s-with-os-mcp`
  (os-mcp http → shell), `s-users-with-hub` (hub → slot 1 → slot 2) on the root; `quiz-with-proctor` (proctor →
  quiz) on the quiz project. Hub and proctor declare `ready`, so the next member waits; os-mcp http declares
  none (its listen line was not verified), so the shell starts at once, as in VS Code.
- **Groups.** Five rows reuse two project sets with two targets, hence five groups. `stdio-artifact-tests` uses
  the Nx patterns `@semio-tech/stdio-*-rs` and `!@semio-tech/stdio-artifact-contract-rs` instead of 37 names.
  The two 48-project snapshot rows became `["*"]` (every project that declares the target, 101 at the snapshot);
  the 18-project "parent baselines" row keeps its list.

## 6. Dropped

- Runner noise, never stored: `NX_DAEMON`, `NX_ISOLATE_PLUGINS`, `NX_CACHE_PROJECT_GRAPH`, `NX_TUI`,
  `FORCE_COLOR`, `NX_PLUGIN_NO_TIMEOUTS`, `--output-style`, `--parallel`, and every flag on `nx exec`.
- Dead knobs: `PUZZLE_3D_PLAY_PORT`, `PUZZLE_5D_PLAY_PORT`, `SHOOTING_PLAY_PORT` (no file mentions them) and
  `CAD_JS_RENDERER_PLAY_PORT`. **The last one was still set to 6041 on four test targets of
  `@semio-tech/cad-js`; nothing reads it, so those four `env` blocks were removed.** This changes the Nx hash of
  those four targets once.
- Repeated owner defaults (21 rows): `OS_HUB_DATA=…/hub-dev`, `OS_HUB_URL`, `SERVER_PORT=6277`, `S_OS_PORT=6066`
  and `SEMIO_PLUGIN=s` on the secure launches, `PROCTOR_PORT=8791` on the quiz, `SEMIO_RENDERER=react` on
  demonstrator and play (the `bun nx` wrapper sets it), `--restarts 1`, and on `time-travel` the serve URL,
  locales and chords.
- `fixture concrete` / `fixture base-icon` after a playground variant (8 seed rows): nothing in os-dev interprets
  a `fixture` segment; the rows are the plain variants.

## 7. Conflicts Between Launch Rows And Owner Facts

1. **Dashboard targets moved.** `@semio-tech/repo-cli-rs:daemon|run|preferences|install` no longer exist; the
   live owner is `@semio-tech/repo-dashboard-rs`. Seven rows are retargeted.
2. **`capsule-dream` rows wait on the wrong port.** They set the dead `PUZZLE_5D_PLAY_PORT=6015/6115`; the server
   binds the catalog port 6014/6114, so their ready action never fired and they collide with the plain puzzle5d
   rows.
3. **Four `.claude` entries named `*-react` start wgpu.** `cad-react`, `s-react`, `dag-react`, `puzzle2d-react`
   run `workspace:dev -- <variant>` without `SEMIO_RENDERER`; the owner default is wgpu
   (`frameworkOsPlaygroundDevEnv`), so the React port they name is never bound. `storybook-framework-os`
   expects 6011 and binds 6010.
4. **`native -- trinity`** names no variant or alias; the row means `trinity-jack`.
5. **Port 6071 is not stale.** `hub-document-sweep` names a free port on purpose: its fixture starts a serve
   there, joined to the hub.
6. **`S_OS_MCP_LIVE_SHELL_URL`:** the gates default to `http://127.0.0.1:6080`, the launch prompt to 6070. The
   declaration takes 6070, the `s` shell it also requires. `live-agent-loop-check` got no default and no
   `requires`: which serve its 6080 means is not stated anywhere.
7. **`stdio-docx-rs` has no `test-snapshot-sqlite-source` target**; the row maps to `test-snapshot-sqlite` with
   the extra argument `source`.
8. **`s-host-foreign-kind-s` and `s-host-pinch-diagram-contrast-s` run scripts inside a ticket folder**
   (`26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`). They are ticket commands wearing an Nx target; whoever owns
   that ticket should move them to its `🎮️commands.json`.
9. **Per-backend hub data roots exist only in the launch file.** The hub script defaults every variant to
   `hub-dev`; the rows gave postgres and neo4j their own folder. Declared as the `data` parameter default until
   the hub derives it.

## 8. Playground Registry

Verified against the live catalog (155 variants): all 41 variants behind the 45 `wgpu-native` rows exist (once
`trinity` is read as `trinity-jack`);
user slots 1 and 2 of `s` equal `user_ports` (6072/6073, 6067/6068); `hexagonal-mushroom-column` and
`capsule-dream` are catalogued examples (as `🍄️…` / `🌙️…`, the launch rows use the bare id); the alias
`procedural 3d` resolves. Nothing could be moved as a plain data addition: `🎮️playground/🔎️discovery` reads a
fixed set of keys, so each fact below needs a key and a reader. **Facts that exist only in the launch seed:**

| Fact | Where it was | Needed for |
| --- | --- | --- |
| `S_DATA_DIR` per user slot (`.🧬semio/🔗space/s-user{N}`) and for the base shell (`…/s-dev`) | `devLaunchers.s.users.env`, `devLaunchers.s.env` | without it a shell keeps its document list in memory only |
| `S_HUB_URL=http://127.0.0.1:8787` on the `s` rows | same | joins a hub someone else runs instead of owning the default one |
| `S_LOCAL_ONLY=1` | one seed row | local-only shell |
| `PLAYGROUND_LOCKED_EXAMPLE_ID` | two seed rows | locks a playground to one example |
| ready suffix `/?plugin=generation3d[&role=viewer]` | two seed rows | opens the viewer role directly |

## 9. Schema Gaps And Questions For A-1

Nothing ad hoc was added; each item names the closest form used.

| # | Gap | Rows | Used instead | Proposed additive field |
| --- | --- | --- | --- | --- |
| 1 | a `requires` entry and a `ready` cannot pin parameters | time-travel wgpu, all `playground:s` requirements | plain reference, default renderer | `requires: [{ run, parameters }]` like a compound member |
| 2 | a playground member cannot carry environment (hub URL, data directory) | 3 compounds, 6 user-slot rows | members without it | playground catalog fields (section 8) |
| 3 | no fixed environment on a target | `dev-postgres`, `dev-neo4j` | text parameter with a default | `TargetDeclaration.env` |
| 4 | a group fixes one target and has no exclusion or parallelism | 5 groups for 2 project sets | one group per target; `!pattern` | `targets: []` |
| 5 | global axes do not reach tools | 3 `bun-test` rows lose `NEXTEST_SUCCESS_OUTPUT` | tool-owned `build-mode` | `appliesTo.tools` |
| 6 | a launch accepts extra arguments but no extra environment | 2 rows with one-off diagnostic switches | dropped, recorded | free `KEY=value` at launch |
| 7 | the ready URL is match + `path`; a printed query string is lost | MCP inspector | the inspector opens its own URL | `ready.url: "printed"` |

Three readings the declarations rely on, to confirm: `{workspace}` is expanded in a text parameter's `default`
that goes to `valueEnv` (hub `data`); `ready: { portEnv }` without `port` finds the port in the resolved
environment (pets); `groups[].projects` passes Nx patterns (`*`, `!name`) through unchanged.

## 10. Coverage

| Family | Rows | Exact | Equivalent | With deviation |
| --- | --- | --- | --- | --- |
| test axis variants | 280 | 261 | 19 | 0 |
| test filter | 268 | 85 | 181 | 2 |
| `bun test <file>` | 81 | 0 | 78 | 3 |
| axis variants (non-test) | 66 | 49 | 16 | 1 |
| print native phases | 65 | 65 | 0 | 0 |
| target-specific arguments | 62 | 45 | 16 | 1 |
| wgpu native | 45 | 45 | 0 | 0 |
| vitest files | 29 | 14 | 15 | 0 |
| dev-server ports | 24 | 17 | 6 | 1 |
| prompted arguments | 21 | 18 | 3 | 0 |
| cargo scope | 14 | 4 | 10 | 0 |
| playground seed variants | 12 | 1 | 0 | 11 |
| `nx run-many` | 12 | 0 | 9 | 3 |
| workspace dev mcp | 9 | 1 | 5 | 3 |
| external tools, other exec, user slots, bun script | 15 | 5 | 6 | 4 |
| **total** | **1,003** | **610** | **364** | **29** |

Resolved as: 833 Nx targets (827 declared in a manifest, 6 inferred), 91 tools, 67 playgrounds, 12 groups; 545
distinct command ids. Four rows hold `<prompt:…>` where the launch input had no default (proctor restore and
erase, hub admin capability). The 29 deviations: 12 launch-only facts (section 8 and two diagnostic switches),
10 dead knobs, 5 axes not offered (cache on a playground, nextest on a tool), 3 merged groups, 2 port conflicts,
1 stale variant, 1 literal token (some rows carry several).

`.claude/launch.json`, 79 entries: 24 Nx targets, 21 playgrounds, 24 attach-only (23 resolve to a catalog
playground by port, one ticket lane on 6222 does not), 8 ticket scripts, 2 scripts (one file gone, one second
quiz stack). Entry-only overrides without a declaration: `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` and the second
stack's ports and `PROCTOR_DATA`.

The comparison is strict in both directions on words after `--` and on environment; Nx flags are compared as
sets. It does not prove that A-1's resolver agrees: that is the battle-test slice's assertion, with this file
as its expectation.

## 11. Validation

All 20 manifests: valid JSON after the edit, `m1a-apply.ts` confirmed that no member outside the declarations
changed, and Ajv (repository dependency 8.20.0, draft 2020-12) accepts each against `#/$defs/ProjectManifest`.
All 25 `requires` entries and compound members resolve, and every literally named group project exists.

## 12. Files Touched

Edited (declarations only, plus the four removed `env` blocks in the last one):

- `📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `🌎️hub/📦️packages/🟦️typescript/📋️project.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📋️project.json`
- `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`
- `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📋️project.json`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📋️project.json`
- `♻️mit-bestand/🧺️demonstrator/📋️project.json`
- `🏢️semio-tech/🎡️play/📋️project.json`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/📋️project.json`

Created in the ticket folder: `m1a-declarations.ts`, `m1a-apply.ts`, `m1a-coverage.ts`,
`m1a-owner-declarations.md`, `🗑️generated/declarations/coverage.json`.

## 13. Not Verified

- No Nx command, build or server was started; whether each `ready` line appears as declared was read from the
  owners' scripts (hub, proctor, storybook, Vite servers) and, for the MCP inspector, taken from the launch
  row's own pattern.
- The compounds without `S_HUB_URL` rely on `dev s` signing in through the running hub's session broker; that
  was read in the hub and os-dev scripts, not run.
- The cached graph predates the edits, so it does not yet show the new metadata.
