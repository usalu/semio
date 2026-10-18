# 📓 Audit: other user-facing deliverables (os-hub, os dev host, bericht/print, Projektetage, repo dashboard, repo VS Code extension, storybook)

Date: 2026-10-18. Read-only audit. Paths are relative to the repo root `/Users/ueli/Documents/semio`.
Nothing was built, served, tested or committed. Only `ls`, `grep`, `sed` and small read-only Node/Bun scripts over `project.json` files and `.vscode/launch.json` were run.
Method: (1) index of all `project.json` names and targets (outside `.🧬semio`), (2) every launch row `bun repo run <project>:<target>` for the in-scope projects checked against that index, (3) every `dependsOn` (same-project and cross-project) checked for an existing target, (4) every `bun ./…/📜️script.ts` path in target commands checked for existence, (5) router registrations of the `📜️script.ts` files read by hand.

Caveats:
- Rust (`-rs`) projects can get Nx-inferred targets (`build`, `check`, `test`) from `Cargo.toml`. The static check flagged 525 `-rs:` launch rows as `TARGET-MISSING`. These are **not** counted as stale below because the inference was not verified (no `nx show project` was run).
- Ports are taken from `project.json` metadata or `package.json` where stated.

## 1. Ranking by importance

The course (`🎓️teaching`, see `audit-teaching-course.md`) does not depend on any of these deliverables except the root workspace (`workspace:start`, see section 3). Semio-tech play and the mit-bestand demonstrator depend on the os dev host and os-hub. Ranking follows that:

| Rank | Deliverable | Status | Why |
|---|---|---|---|
| 1 | os dev host (`@semio-tech/framework-os-dev`) | **broken** | Demonstrator and play launch chains call targets that the project does not declare. 21 cross-project `dependsOn` in the demonstrator are missing. 342 launch rows are stale. |
| 2 | os-hub (`os-hub`) | **mostly ok, 2 broken dependency chains** | Play needs the hub. `dev` and `dev-*` targets are valid. `test-all-features` and the component codec checks depend on `component` targets that do not exist. The memory note cites a command that no longer exists. |
| 3 | mit-bestand Bericht + print pipeline (`@semio-tech/mit-bestand-bericht`, `@semio-tech/print`) | **ok** | Build chain resolves. Watch rows exist. No one-shot launch row for the Bericht build. Not used by course, play or demonstrator. |
| 4 | root workspace storybook and `workspace:start` | **one broken entry point** | `workspace:start` (the launch row "▶️start") targets a missing target. All storybook rows are valid. |
| 5 | repo dashboard (`@semio-tech/repo-dashboard-rs`) | **ok, no run row** | Targets exist. Only `compile` is registered in launch. The `run` target used by the root `dashboard` npm script has no launch row. |
| 6 | repo VS Code extension (`@semio-tech/repo-vscode`) | **ok** | All registered rows resolve. `dev`, `build`, `test`, `lint` have no launch row. |
| 7 | Projektetage (`@semio-tech/mit-bestand-praesentation-projektetage`, port 6050) | **ok** | Independent of the others. `dev` row valid. |

## 2. Per deliverable

### 2.1 os dev host (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript`, `@semio-tech/framework-os-dev`)

- Launch rows: 342 rows reference `@semio-tech/framework-os-dev:<target>` with a target that is not in `📋️project.json`. Examples: `dev-<app>-react-prototype`, `dev-<app>-wgpu-prototype` (330 rows, including the 🛠️dev rows of cad, dag, flow, puzzle3d, generation3d, mit-bestand aggregator/generator/koordinator/aussuchen/bearbeiten/verfolgen, and fem), `serve-generation3d-react-prototype`, `serve-draw-react-prototype`, `build-s-react-release` (📦️build🖥️s), `build-cad-react-release`, `build-puzzle3d-react-release`, `build-puzzle5d-react-release`, `build-shooting-react-release`, `mcp-gis2d-stdio-{prototype,release}`, `mcp-gis2d-http-{prototype,release}`.
- Project targets that exist: `build`, `test`, `verify`, `bench`, `plugin`, `plugin-size`, `parity`, `index-lint`, `host-handle-lint`, `program-matrix`, `tool-run-matrix`, `two-human`, `connection-budget`, `idle-budget`, `interaction-latency`, `memory-soak`, `catalog-smoke`, `cold-boot-check-s`, `generate-distribution`, `preview-distribution`, `check-distribution`, `generate-playground-session`, `preview-playground-session`, `check-playground-session`, `generate-scale-fixture`, `preview-generated`, `scale-fixture-check`, `bench-plugins-{native,react,wgpu}`, `build-inputs`, `channel-version-*`, `serve-hold`, `verify-home`, `two-human-viewer`, `two-human-cross-undo`, `boot-budget`, `time-travel`, `canonical-architecture`, `parity-journey`, `distribution-output-check`, `source-watch-check`, `test-publication`, `verify-staged`, plus the `canonical-bootstrap-folder-mirror-*`, `closed-browser-component-factory-check`, `browser-host-staging-check` checks.
- The `📜️script.ts` router registers `activate`, `serve`, `prepare`, `serve-hold`, `playground-session`, `publication-check`, `publication-worker`, `cold-boot-check`, `test`, `verify`, `bench`, `generate`, `scale-fixture`, `preview-generated`, `distribution`, `canonical-architecture`, `index-lint`, `host-handle-lint`, `channel-version`, `parity`, `plugin`. **`activate`, `serve`, `prepare` are not declared as targets** in `📋️project.json`. The per-app launch rows and the demonstrator chains depend on them through the undeclared names.
- Broken internal `dependsOn`:
  - `build` depends on `build-s-react` (missing). `framework-os-dev:build` (launch row 80758) fails at graph construction.
  - `cold-boot-check-s` depends on `activate-s-react-prototype` (missing).
- Broken cross-project `dependsOn` from `@semio-tech/mit-bestand-demonstrator` (`♻️mit-bestand/🧺️demonstrator/📋️project.json`): **21** targets, all `@semio-tech/framework-os-dev:activate-*-react-prototype` and `prepare-*-react-{prototype,release}` for aggregator, aussuchen, bearbeiten, koordinator, generator, verfolgen, energy, fem3d and others.
- Suspect path: `test-publication` runs `bun ./🧰️framework/🛍️products/💻️os/…/📜️script.ts publication-check` from the project directory. The path is workspace-relative, unlike every other target of this project. It only works if the executor's cwd is the repo root. Not verified.
- Stale references: the launch rows listed above, the `build` and `cold-boot-check-s` dependencies, and the 21 demonstrator dependencies.
- Probes and e2e: `two-human`, `two-human-viewer`, `two-human-cross-undo`, `verify-home`, `time-travel`, `boot-budget`, `connection-budget`, `idle-budget`, `interaction-latency`, `memory-soak`, `catalog-smoke`, `parity`, `parity-journey`, `program-matrix`, `tool-run-matrix`, `scale-fixture-check`, `check-playground-session`, `distribution-output-check`, `verify-staged`.
- Ports: not traced here. The serve module (`♻️activation/🌐️serve/🟦️.ts`) takes its port from `devServePortV1`, which was not read.

### 2.2 os-hub (`🌎️hub/📦️packages/🦀️rust`, `os-hub`; admin UI `os-hub-admin`)

- Targets (declared): `build` (depends on `os-hub-admin:build`), `build-dev`, `publish` (depends on `build`), `dev`, `dev-postgres`, `dev-neo4j`, `dev-secure-suite`, `dev-secure-native`, `dev-secure-mcp`, `dev-secure-admin`, `dev-collaboration`, `local-hub-owner`, `local-bootstrap-launch-check`, `secure-local-smoke`, `test`, `test-all-features`, `trusted-catalog-preflight`, `trusted-catalog-bootstrap`, `trusted-stdio-gis-bundle-check` (+ `-browser`, `-process`), `live-sign-in-check` (depends on `build-dev`), and about 80 further gate and check targets.
- Router `📜️script.ts`: all declared targets resolve to registered commands. `dev postgres`, `dev neo4j`, `dev secure-*` are sub-arguments of `dev`. No stale registration found.
- Launch rows: all `os-hub:*` rows resolve. Dev row `os-hub:dev` (line 108971), `dev-postgres`, `dev-neo4j`, `dev-secure-{suite,native,mcp,admin}`, `local-hub-owner`, `dev-collaboration`. Build and publish rows at 85725-85758. Trusted-catalog rows: `os-hub:trusted-catalog-preflight -- --packages all` (85803), `os-hub:trusted-catalog-bootstrap -- --packages all` (85818).
- Port: `OS_HUB_PORT`, default 8787 (`dev-*` targets, `📜️script.ts` metadata).
- Broken dependencies:
  - `os-hub:test-all-features` depends on `@semio-tech/gis-plugin:component`. That target does not exist (`🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/📋️project.json`). Launch row 86011 (`📦️test🗄️os-hub♾️all-features`) fails at graph construction.
  - `os-hub:component-codec-check` depends on `@semio-tech/note-plugin:component`, `@semio-tech/gis-plugin:component`, `@semio-tech/stdio-plugin:component`. `os-hub:component-codec-budget-check` depends on `@semio-tech/gis-plugin:component`. None of these `component` targets exist.
- Stale launch row: `🛠️component🏢️semio-tech🎡️play🎪️demonstrator` runs `semio-hub-demonstrator-component:component` (line 81051). The project name does not exist. The hub composition project is `@semio-tech/demonstrator-plugin` (`🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust`), which has only a `test` target. `📦️describe🖍️draw🦀️` runs `semio-hub-draw-component:component` with the same problem.
- Stale memory: `/Users/ueli/.claude/projects/-Users-ueli-Documents-semio/memory/project-hub-trusted-catalog-publish-chain.md` documents `bun ./📜️script.ts trusted-stdio-gis-bootstrap` in `🌎️hub/📦️packages/🦀️rust`. That command is **not registered** in the router and **not a target** in `📋️project.json`. The current equivalents are `trusted-catalog-preflight` and `trusted-catalog-bootstrap` (launch rows 85803 and 85818). The memory note must be updated before it is reused.
- Probes and e2e: `live-sign-in-check`, `admin-live-journey-check`, `admin-backend-check`, `admin-relay-check`, `browser-document-open-check`, `open-plan-check`, `open-plan-server-check`, `canonical-pair-check`, `trusted-catalog-opened-root-check` (+ `-native`), `trusted-stdio-gis-bundle-check` (+ `-browser`, `-process`), `space-journey-process-check`, `directory-live-lanes`, `local-bootstrap-launch-check`, `secure-local-smoke`.
- Other `hub` rows in `.vscode/launch.json` are Rust crate rows (`@semio-tech/hub-*-rs:build|check|test`, `semio-hub-trusted-catalog-*`, and so on). The static check flags them as `TARGET-MISSING`, but this is the unverified Rust inference caveat above.

### 2.3 mit-bestand Bericht and print pipeline

- Bericht: `@semio-tech/mit-bestand-bericht` (`♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json`).
  - Targets: `build` (complete, depends on `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht`), `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht`, `watch`, `watch-zwischenbericht`, `watch-forschungsbericht`, `watch-kompaktbericht`, `generate-actor-network`, `akteursnetz-validate`, `check-generated`, `preview-generated`, `test`.
  - Cross-project deps: `@semio-tech/print:fonts`, `:deps-tectonic`, `:deps-tex`, `:generate`. All exist.
  - Script paths: `📄️documents/📜️script.ts`, `👥️actor-network/📜️script.ts`, `🖨️tectonic-template-compilation/📜️script.ts`. All exist.
  - Launch rows: `@semio-tech/mit-bestand-bericht:watch` (line 88140), `watch-zwischenbericht` (109565), `watch-forschungsbericht` (109580), `watch-kompaktbericht` (109595). All valid.
  - Gap: no one-shot launch row for `bericht:build` or `build-*`. Per AGENTS.md every executable command needs a launch row.
- Print: `@semio-tech/print` (`🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json`).
  - Targets: `build`, `watch`, `check`, `generate`, `generate-viz`, `build-viz`, `watch-viz`, `fonts`, `deps-tectonic`, `deps-tex`, `test`, `test-viz`, `test-viz-full`, `test-viz-fixtures`, `test-macro-staging`, `test-native-grammar`, `test-toolchain`, `test-measurement`, `generate-font-metrics`, `preview-generated`.
  - Launch rows (`@semio-tech/print:*`) all resolve. Examples: `print:deps-tectonic` (77628), `print:deps-tex` (77643), `print:watch-viz` (78525), `print:watch` (85251), `print:generate-viz` (86332), `print:generate-font-metrics` (120614), `print:check` (127412), `print:test -- command-boundaries` (127511), `print:test -- api-publication` (127526), `print:test-native-grammar` (many).
  - Note: the `print-tests-rs`, `print-*` rust rows are part of the unverified `-rs` caveat.
- Output: the Bericht output is `dist/documents/zwischenbericht` (via `print`). No Bericht output in the demonstrator.

### 2.4 Projektetage (`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript`, `@semio-tech/mit-bestand-praesentation-projektetage`)

- Targets: `dev`, `build`, `test`, `canonical-architecture` (runs `test long ../../🧪️tests/🎞️react-deck/🟦️.tsx`; the file exists).
- Port: **6050** (`📋️project.json` metadata `ready.port` and env `PRAESENTATION_PROJEKTETAGE_PORT=6050`; `package.json` `dev: 6050`).
- Launch rows: `@semio-tech/mit-bestand-praesentation-projektetage:dev` (line 88119). Valid.
- Gap: no launch row for `build`, `test`, `canonical-architecture`.
- Dependencies: none on course, play or demonstrator.

### 2.5 Repo dashboard (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust`, `@semio-tech/repo-dashboard-rs`)

- Targets: `build`, `test`, `test-coverage`, `test-journeys`, `test-load`, `test-smoke`, `run`, `daemon`, `install` (depends on `build`), `preferences`, `launch`, `compile`. All declared.
- Root `package.json` scripts: `dashboard` → `repo-dashboard-rs:run --output=stream`; `dashboard:install` → `:build`. Both valid.
- Launch rows: only `repo-dashboard-rs:compile --configuration=prototype` (80698) and `:compile --configuration=release` (80713). No row for `:run`, `:launch` (outside ticket context), `:daemon`, `:test*`. `:launch -- run ticket:…` rows exist only inside ticket-scoped entries (185095 and following).
- Port: not declared in `project.json`.
- Probes: `test-smoke`, `test-journeys`, `test-load`, `test-coverage`.

### 2.6 Repo VS Code extension (`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript`, `@semio-tech/repo-vscode`)

- Targets: `dev` (depends on `framework-schema:generate` and `generate-graphql-types`), `test`, `build`, `lint`, `build-vsix`, `compile-extension`, `watch-extension`, `generate-graphql-types`, `preview-generated`, `check-graphql-types`, `test-graphql-types`, `test-custody`. All declared; cross-project `framework-schema:generate` exists.
- Launch rows (all valid): `repo-vscode:generate-graphql-types` (132671), `preview-generated` (132686), `check-graphql-types` (132701), `test-graphql-types` (132716), `test-custody --configuration=prototype|release` (175622, 175640), `build-vsix` (175694), `watch-extension --configuration=prototype|release` (175709, 175724), `compile-extension --configuration=prototype|release` (175739, 175754).
- Gap: no launch row for `dev`, `build`, `test`, `lint`.
- Artifact: `🧩️repo.vsix` sits in the project folder (built output, not checked against the current build).

### 2.7 Storybook (root `.storybook/`, `repo-storybook-storage`, root workspace)

- Root `.storybook/main.ts`: every relative import resolves (checked by script). Storybook 10 (`storybook`, `@storybook/react-vite` in root `package.json`). **The root `package.json` has no storybook script.** Entry points are the `workspace` project targets.
- `workspace` (root `📋️project.json`, name `workspace`), storybook targets, all declared: `dev-storybook` (port 6010 via `STORYBOOK_PORT`), `dev-storybook-{animate,cad,framework,framework-hosts,framework-os,infinite,puzzle,puzzle-2d,puzzle-3d,puzzle-5d,styling,ui}` (launch rows 107708-120488 all valid), `build-storybook` (163924), `setup-storybook` (166144), `test-storybook` (166894).
- `repo-lib:ui-storybook-dev` (132977) and `repo-lib:ui-storybook-build` (132992): valid. `repo-lib:test-storybook-discovery` (172031): valid.
- `repo-storybook-storage:test` (launch rows 12573, 12588): valid.
- **Broken:** `workspace:start` (launch row 77115, labelled "▶️start") and `workspace:dev-mcp-engine` (launch row "🛠️dev-mcp-engine") point to targets that are not in the root `📋️project.json`. `workspace:start` is the main entry point of the repo and is therefore the most important finding in this group.

## 3. Cross-cutting findings

1. `workspace:start` is missing from the root `📋️project.json`. The repo's own "start" launch row fails.
2. The os dev host (`framework-os-dev`) is the single largest source of stale references: 342 launch rows, 2 internal dependencies, 21 demonstrator cross-project dependencies.
3. Memory staleness: `project-hub-trusted-catalog-publish-chain.md` cites an unregistered hub command.
4. Gaps against the "every executable command has a launch row" rule: the Bericht build targets, repo dashboard `run`, repo VS Code `dev`/`build`/`test`/`lint`, Projektetage `build`/`test`.

## 4. Not verified (for the next step)

- Whether `bun repo run <project>:<target>` resolves undeclared targets (for example `framework-os-dev:activate`, `:serve`, `:prepare`) through a router or a generator. The static check says no. A dry run would settle it, but it was out of scope for this read-only audit.
- Whether the 525 Rust `-rs:` rows are satisfied by Nx-inferred targets (`nx show project <name> --json` would settle it).
- Ports for the os dev host serve (`devServePortV1`) and the dashboard.
