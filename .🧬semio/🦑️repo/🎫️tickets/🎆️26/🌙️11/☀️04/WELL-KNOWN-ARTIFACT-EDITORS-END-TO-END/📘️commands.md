# Commands: build, start and test the OS app and the stdio plugin (Steam Deck)

Owner: T20 (haiku-documentation). Scope: command map for the fleet goal "every well-known artifact editor (39 stdio format directories under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*`, see status) works end to end in the OS app `🧰️framework/🛍️products/💻️os`".

Evidence key: **[M]** = read from a checked-in manifest or source file (path given). **[R]** = observed by running a cheap command in this session. **[U]** = unverified (not run). Results of `repo show project` are listed separately in section 9.

## 1. Host facts

- Host: Linux Steam Deck, kernel `7.2.7-valve2-2-neptune`, 15.8 GB RAM, load average 22 to 29 during this session (many fleet workers). [R]
- `bun` is not on PATH. Do not call `bun` directly. `repo` provisions its own bun runtime; observed at `~/.cache/semio/runtimes/bun/<hash>/bun` (seen in the process list). [R] Use `repo tools list --json` / `repo tools verify bun` to inspect the pin [U: `tools verify` was started but moved to background and not read to completion].
- On PATH: `~/.local/bin/repo`, `~/.local/bin/cargo`, `~/.local/bin/node`. [R: `which`]
- Root toolchain pin: `rust-toolchain.toml` channel `nightly-2026-07-20`, targets `wasm32-unknown-unknown` and `wasm32-wasip2`. [M]
- Root `package.json`: `packageManager` `bun@1.4.2`, `engines.node` `24.15.0`, Playwright `^1.57.0` in devDependencies. [M]
- AGENTS rules that matter here: use `repo` for every command, never `repo dashboard`, never a modifying git command, no dev server or full build outside the shared verification queue, `[DEBUG] ` prefix on temporary logs, generated outputs under the ticket `🗑️generated/`.

## 2. Heavy-job warning (verified)

- `repo show projects` (cheap in principle) did not finish within 120 s. Its log showed `[capacity] waiting ... for show: not enough free memory (need 5875 MB ...)` behind other fleet jobs (`repo-mcp:install`, `lock`, `projects`, `semio-s-artifact-stdio-*`). [R]
- `repo show project @semio-tech/stdio-plugin` was queued the same way (`behind command ... semio-s-artifact-stdio-obj ... semio-s-artifact-stdio-zip-editor`). [R]
- Practical rule: run any `repo show`, `repo graph`, `repo tools verify` or `repo doctor` as a background job with a long timeout, write output into the ticket `🗑️generated/`, and check later. Do not start a second copy while one is waiting. Inspect queue state with `repo capacity status` and `repo fleet status` [U: not run].
- The manifests below were read directly from the repository, so the names and targets are known even while `repo show` is queued.

## 3. Projects and targets (stdio plugin)

Name sources: `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🪪️identity/🟦️.ts` (`COMPOSITION_RUST_NAME`, `COMPOSITION_TYPESCRIPT_NAME`) [M].

| Project | Root (cwd) | Key targets | Command behind the target |
| --- | --- | --- | --- |
| `@semio-tech/stdio-plugin` (Rust composition) | `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust` | `test`, `editor-component-check`, `catalog-root`, `native-codec-projection`, `composition-prepare`, `composition-contribution-check`, `home-io-surface-check`, `home-io-surface-native-check`, `test-snapshot-sqlite-<format>` (about 30 targets), `artifact-directory-wiring-generate` / `-check`, `subset-directory-wiring-generate` / `-check`, `mutation-inventory`, `flow-retained-decode-check` | `repo ./📜️script.ts <...>` in that cwd [M: its `📋️project.json`] |
| `@semio-tech/stdio-js` (TypeScript composition) | `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript` | `build`, `check`, `test`, `package-contract`, `package-graph`, `verify-semio-member-store-producer-source` | `repo ./📜️script.ts <...>` [M] |
| `@semio-tech/stdio-<format>` (TypeScript, one per format) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<dir>/📦️packages/🟦️typescript` | `build`, `check`, `test` | `repo ./📜️script.ts <...>`; the `test` script runs the rfc/standards suites through `runArtifactTypeScriptPackageMain` [M: CSV example] |
| `@semio-tech/stdio-<format>-rs` (Rust, one per format) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<dir>/📦️packages/🦀️rust` | per-format `📋️project.json` | [M: CSV dir has `@semio-tech/stdio-csv-rs`] |

Format directories: 39 found under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`. Coordinator correction: 38 artifact definitions, since `📇️inventory` is a library and not an artifact. Some names in the TS packages are `@semio-tech/stdio-<format>` (e.g. `stdio-csv`, `stdio-png`, `stdio-dxf`), some Rust crates use a `-rs` suffix (e.g. `stdio-pdf-rs`, `stdio-dwg-rs`, `stdio-semio-rs`). [M: grep of `"name"` in `package.json` files]

Root gate targets (project `workspace`, root `📋️project.json`) [M]: `stdio-quick`, `stdio-long` (depends on quick), `stdio-exhaustive` (depends on long), `stdio-schema-parity`, `stdio-standards-coverage`, `stdio-codec`, `stdio-mutation-law`, `stdio-inference`, `stdio-runtime`, `stdio-fuzz`, `stdio-cross-platform`, `stdio-document-contract`, `stdio-document-contract-native`. Each runs `repo ./📜️script.ts stdio <gate>`.

Caveat [M: `🎛️commands/🗄️stdio/🟦️.ts`]: `stdio runtime`, `stdio fuzz` and `stdio cross-platform` run only the `quick` structural gate. They do not exercise runtime, fuzz or cross-platform checks. Do not count them as proof.

`stdio long` = structural gates, then `repo run-many -t test -p @semio-tech/stdio-plugin @semio-tech/stdio-js --parallel <n> -- long`. `stdio exhaustive` = long, then the same with `-- exhaustive`. [M]

Stdio test default level: the stdio composition `TestScript` calls `resolveTestLevel(segments, "long")`, so `test` with no level runs at least `long` (budget 900 s). [M: `📚️editor-catalog-and-command-scripts/🟦️.ts:157`]

Commands:

```bash
cd /home/deck/semio
repo run workspace:stdio-quick
repo run workspace:stdio-long
repo run @semio-tech/stdio-plugin:test -- long            # forwarded arguments go to the test script
repo run @semio-tech/stdio-js:test
repo run @semio-tech/stdio-csv:test                       # per format; replace csv
repo run @semio-tech/stdio-plugin:editor-component-check
repo run @semio-tech/stdio-plugin:catalog-root
```

Those `repo run` forms follow the documented `repo run <project:target[:configuration]>` syntax from `repo --help` [R]. The forms are [U] until `repo show project` returns.

## 4. OS app: build, prepare, start

Projects [M]:

- `@semio-tech/framework-os` (`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`): scripts `generate-wgpu`, `check-wgpu`, `preview-generated`, `test`, `installed-service-source`.
- `@semio-tech/framework-os-dev` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript`): the dev server, verification and playground script. Its `📜️script.ts` routes: `serve`, `serve-hold`, `verify`, `test`, `build`/`distribution`, `playground-session`, `parity`, `plugin`, `cold-boot-check`, `generate`, `preview-generated`. Targets are listed in its `📋️project.json` (for example `verify-home`, `two-human`, `catalog-smoke`, `boot-budget`, `interaction-latency`, `idle-budget`, `time-travel`, `program-matrix`, `tool-run-matrix`, `connection-budget`). [M]
- `os-hub` (`🌎️hub/📦️packages/🦀️rust`): `dev` (dashboard id `os-hub:dev`, port 8787 with `/admin`), `dev-collaboration`, `test`, `test-all-features`, `build-dev`, `dev-secure-*`. [M]
- Root `os` target: `repo ./📜️script.ts os` (`continuous: true`), headless run `os run <bundle>.studio`. [M]
- Root `components` target: `repo ./📜️script.ts components` (depends on `@semio-tech/os-plugin-describe-rs:build` and `@semio-tech/framework-plugin-web:support`). [M]
- Root `prepare` target: `repo:noop` ordering `@semio-tech/framework-schema:generate`, `@semio-tech/ui-styling-tokens:generate`, `@semio-tech/assets:build`, `@semio-tech/framework-graph:generate`, `@semio-tech/plugin-registry:generate`. [M]

Build and prepare (cheap to read, heavy to run; run through the queue):

```bash
cd /home/deck/semio
repo run workspace:prepare                 # generated schema, styling, assets, framework graph, plugin registry
repo run workspace:components describe --all --profile prototype   # compiles every plugin component [heavy]
repo run workspace:components describe <plugin-id> --profile prototype
```

Start the React `s` shell (the "real OS app" dev server). [M: `🚀️local-hub/🏃️execution/🟦️.ts` `DEV_SERVE_REACT_SCRIPT`, `devServeCommandV1`; `♻️activation/🌐️serve/🟦️.ts`]

```bash
cd "/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
repo ./📜️script.ts serve s react prototype
```

- `serve <variant> react <prototype|release> [server options]`. Variant `s` is the full OS shell. The serve activates the variant first if its activation receipt is missing. [M]
- Default port 6070 (`S_OS_PORT`). Env set by the serve owner: `SEMIO_PLUGIN=<variant>`, `SEMIO_RENDERER=react`, `SEMIO_VITE_HMR=0`, `S_OS_PORT=<port>`, `S_LOCAL_ONLY=1` (local-only) or `S_HUB_URL=<url>`. [M]
- Supervised variant (holds until SIGINT, stops only what it started): `repo ./📜️script.ts serve-hold --serve http://127.0.0.1:6070 [--hub <url>] [--variant s]`. [M]
- Reuse: if a serve already answers on the port, the owner reuses it and never stops it. Port 7800 is refused (hub). [M]
- Dashboard equivalents (documentation only; do not use the dashboard): `playground:s` (6070), `playground:s renderer=wgpu-wasm` (6066), `os-hub:dev` (8787), `compound:workspace/s-users-with-hub`. [M: root README table]
- Storybook scope for the OS framework: `repo run workspace:dev-storybook-framework-os` (`STORYBOOK_PORT=6010`). [M]
- MCP: `repo run workspace:dev-mcp`, `repo ./📜️script.ts dev mcp os` (the os MCP gateway over stdio; binary must be staged by `repo run repo-mcp:build` / `@semio-tech/framework-os-mcp-rs:build`). [M]

Headless run (no browser): `repo ./📜️script.ts os run <bundle>.studio [--node <id>] [--watch] [--dry]`. It refuses to run when a built plugin component is missing under `dist/components` and names the `repo run workspace:components describe <plugin-id>` command to fix it. The registry it checks is `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json`. [M: `🎛️commands/🧩️os/🟦️.ts`]

## 5. Catalog generation

- Plugin registry (feeds `🔌️plugins.json` and the OS dev routes): `repo run @semio-tech/plugin-registry:generate`; check with `repo run @semio-tech/plugin-registry:check`. Part of `workspace:prepare`. Project dir `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`. [M]
- Stdio catalog root and codec projection: `repo run @semio-tech/stdio-plugin:catalog-root`, `repo run @semio-tech/stdio-plugin:native-codec-projection`. The latter writes `🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json`, `🌎️hub/🧩️compositions/🗄️stdio/📇️publication/📜️native-catalog.json` and each `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/📜️artifact-definition.json`. [M]
- Stdio directory wiring: `repo run @semio-tech/stdio-plugin:artifact-directory-wiring-generate` (selects the `jpg` artifact directory); `repo run @semio-tech/stdio-plugin:subset-directory-wiring-generate` (`semio mesh`). Both have `-check` variants. [M]
- Derived config: `repo ./📜️script.ts generate config` (`--check` verifies only) and `repo ./📜️script.ts generate taxonomy census|duplicates`. Root targets `generate-config`, `generate-config-check`, `generate-taxonomy-census`, `generate-taxonomy-duplicates`. [M]
- OS catalog smoke (renders each program of the catalog in the served shell): `repo ./📜️script.ts verify catalog --out <dir>` in `framework-os-dev`, target `catalog-smoke`. Needs a served shell at `S_STUDIO_URL` (default `http://127.0.0.1:$S_OS_PORT/`). [M]

## 6. Browser e2e harness (Playwright)

- Browsers: `repo run workspace:deps-browsers` (root target; runs `repo ".../📦️dependencies/🏗️native/📜️script.ts" sync browsers`; depends on `deps-js-all`). Sets `PLAYWRIGHT_BROWSERS_PATH` to the repo cache `tools/ms-playwright` by default. [M: `repoToolCacheEnv` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts`]
- Studio e2e (Chromium headless, Playwright `chromium.launch`): `repo ./📜️script.ts verify e2e` in the `framework-os-dev` package with the shell served on `S_OS_PORT` (default 6070) or `S_STUDIO_URL`. Source: `🧪️tests/🎬️studio/🟦️.ts` `runStudioE2eVerify`. [M]
- Home e2e: `repo ./📜️script.ts verify home --serve <url> [--hub <url>] [--locale en|de] [--tag <t>] [--out <dir>]`. A `--hub` run needs `OS_HUB_PROBE_EMAIL` and `OS_HUB_PROBE_PASSWORD`; without them it publishes a `blocked` result. It sets `S_DATA_DIR` to a scratch dir under `--out`. [M]
- Other verify routes: `two-human`, `time-travel`, `latency`, `boot`, `idle`, `connections`, `tool-run`, `matrix`. [M]
- Default `repo ./📜️script.ts verify` (no route) runs the whole chain: plugin registry generation, `cargo test --lib` per plugin, vitest for the React renderer, studio e2e, and the plugin capability lint. It is heavy. Do not run it outside the queue. [M]
- Playwright lives in the root `package.json` devDependencies `playwright ^1.57.0`. [M]
- Gap: this inventory found no per-editor Playwright spec for the 39 stdio formats. The closest per-editor coverage is `verify catalog` (catalog smoke) and the stdio `editor-component-check` / `test` targets. A per-editor create/open/edit/save/reopen journey still needs to be identified or written. [U]

Storybook unit runner (not the OS e2e): `repo run workspace:test-storybook-catalog` runs `repo tools exec bun -- x vitest run --config .storybook/🧪️tests/🧪️unit-runner/🟦️.ts`. [M]

## 7. Environment variables (with sources)

| Variable | Meaning and default | Source |
| --- | --- | --- |
| `SEMIO_TEST_LEVEL` | `fundamental` (default) \| `quick` \| `long` \| `exhaustive`. Set by `resolveTestLevel`. `exhaustive` also sets `SEMIO_COVERAGE=1`. | `🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts` |
| `SEMIO_TEST_BUDGET_MS` | Overrides the per-level wall clock budget in ms. Defaults: fundamental 15 000, quick 300 000, long 900 000, exhaustive 1 800 000. Inherited by vitest, cargo, go, pytest, dotnet. The project targets pass it with `{ "env": "SEMIO_TEST_BUDGET_MS" }`. | same file (`testLevelBudgetMs`); `🌎️hub/📦️packages/🦀️rust/📋️project.json` and many others |
| Contention stretch | Level budgets are stretched by a load-aware ratio at launch (`launchContentionRatioV1`). | `🎚️levels/🟦️.ts` |
| `SEMIO_BUILD_BUDGET_MS` | Build budget, default 0 (unbounded). Preparation falls back to a schema default. | `🔨️modules/🏃️process/⏱️budget/🟦️.ts` |
| `SEMIO_CMD_BUDGET_MS`, `SEMIO_ORCHESTRATOR_BUDGET_MS`, `SEMIO_DAEMON_BUDGET_MS` | Command, orchestrator and long-lived process budgets, default 0. | same file |
| `SEMIO_BUILD_PARALLEL` | Parallelism; default `availableParallelism()`. Override with a positive integer. | `🏃️process/🌿️environment/🟦️.ts` (`semioBuildParallel`) |
| `SEMIO_BUILD_TERMINAL`, `SEMIO_BUILD_DYNAMIC_OUTPUT`, `SEMIO_BUILD_VERBOSE` | Set to `false` by `devToolingEnv` unless already set. | same file |
| `SEMIO_BUILD_MODE` | `prototype` or `release`. Any value other than `prototype` selects `release` in component passes (read in code; the dev-profile meaning is [U]). | `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` |
| `SEMIO_BUILD_TOOL` (dashboard parameter `build-tool`) and `repo --tool=nx\|bazel\|semio` | Build backend. Default is the repo default (`nx` with bun). | root `📋️project.json` `metadata.semio.dashboard.parameters`; `repo --help` [R] |
| `SEMIO_BUILD_GRAPH_POLICY` | `fresh` (default) or `current` for `repo graph` / `repo show`. | `repo --help` [R] |
| `CARGO_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR` | Do not set by hand in the shared checkout. Repo default from `.cargo/config.toml`: `target-dir = ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/d/t"`, `build-dir = ".../d/b"`, `rustc-wrapper = repo`. Scripts that need isolation set them explicitly (stdio composition `devToolingEnv`, `catalog-root` target dir). Prefer `repo cargo --store=shared\|isolated --configuration=prototype\|release -- <cargo args>`. | `.cargo/config.toml` [M]; `🦀️cargo/🗂️directories/🟦️.ts`; `🌎️hub/.../🏗️catalog-root-script/🟦️.ts` |
| `SEMIO_WORKSPACE_ROOT`, `SEMIO_REPO_ROOT` | Workspace root for stdio and os-run commands. | `🌎️hub/.../📚️editor-catalog-and-command-scripts/🟦️.ts`; `🧩️os/🟦️.ts` |
| `S_OS_PORT` | Shell port for verify and serve. Default 6070. | `🧪️tests/✅️verification/🟦️.ts` [M] |
| `S_STUDIO_URL` | Studio URL for `verify e2e` / `catalog`. Default `http://127.0.0.1:$S_OS_PORT/`. | same |
| `S_STUDIO_E2E_TIMEOUT_MS` | Studio e2e timeout, default 300 000. | same |
| `S_CATALOG_SMOKE_PROGRAM_MS` | Per-program catalog smoke timeout, default 30 000. | same |
| `S_LOCAL_ONLY`, `S_HUB_URL` | Local-only serve or hub URL for the serve owner. | `♻️activation/🌐️serve/🟦️.ts` [M] |
| `S_DATA_DIR` | Data dir for a serve. Home e2e sets it to a scratch dir under its `--out`. | `🎬️studio/🟦️.ts` [M] |
| `OS_HUB_PROBE_EMAIL`, `OS_HUB_PROBE_PASSWORD` | Credentials for `verify home --hub`. | `🎬️studio/🟦️.ts` [M] |
| `STORYBOOK_PORT` | Storybook port, 6010 for framework-os. | root `📋️project.json` `dev-storybook-framework-os` [M] |
| `PLAYWRIGHT_BROWSERS_PATH` | Playwright browser cache; default `tools/ms-playwright` under the repo cache. | `🏃️process/🌿️environment/🟦️.ts` `repoToolCacheEnv` [M] |
| `GOCACHE` | Go build cache under the repo cache. | same |
| `DEVCONTAINER=true` | MCP inspector binds `0.0.0.0` instead of `127.0.0.1`. | `🎛️commands/🛠️dev/🟦️.ts` [M] |
| `SEMIO_COVERAGE` | `1` enables coverage; set automatically at `exhaustive`. | `🎚️budget/🟦️.ts` [M] |

## 8. Recommended order (once the queue is free)

1. `repo tools verify` (background, with output in `🗑️generated/`).
2. `repo run workspace:prepare`.
3. Stdio: `repo run workspace:stdio-quick`, then the per-format `repo run @semio-tech/stdio-<format>:test`, then `repo run workspace:stdio-long` if budget allows.
4. OS build: `repo run workspace:components describe --all --profile prototype` (heavy, one at a time).
5. OS serve: `serve s react prototype` in the `framework-os-dev` package, port 6070, as a single shared serve (`serve-hold` for a held session).
6. Browser: `repo run workspace:deps-browsers` once, then `repo ./📜️script.ts verify e2e` and `verify catalog --out <ticket>/🗑️generated/...` against that serve.

Each heavy step uses the shared verification queue (one full build, test or browser session at a time).

## 9. Verification status

Passed (cheap, executed):

- `repo --help`: prints the command set above. [R]
- Manifest and source reads listed as [M] above. [M]

Blocked or queued (not verified by execution):

- `repo show projects`: timed out at 120 s while waiting for host capacity. Output file: `🗑️generated/show-projects.txt` (partial: queue log only). [R, blocked]
- `repo show project @semio-tech/stdio-plugin`: queued for host capacity; output will go to `🗑️generated/show-stdio-plugin.txt`. Status at time of writing: still waiting behind other fleet jobs. [R, pending]
- An earlier `repo show project semio-s-artifact-stdio` was started with a guessed name and was stopped by me; that name does not exist in the manifests. [R]
- `repo tools verify`: started, then moved to the background; not finished. [R, pending]

Unverified: every `repo run` line in this document except the help text. The `repo show project` checks for the OS and hub projects (`@semio-tech/framework-os-dev`, `os-hub`, `@semio-tech/plugin-registry`, `@semio-tech/framework-os`) were not queued, to avoid adding load. [U]

## 10. Open items for the coordinator

- Format count resolved by coordinator: 38 artifact definitions (39 directories minus `📇️inventory`).
- Name the per-editor e2e harness: no per-format Playwright journey was found. Either write one (create, open, edit, save, reopen per format, driven through `verify e2e`-style Playwright code) or assign it.
- Decide whether `stdio runtime|fuzz|cross-platform` should run real checks; today they only run the quick structural gate.
- Confirm `repo show project` results for each target above (section 9) when the queue frees.

## Running stdio crate tests

Source: `📦️crate-names.md` (TOML read of every `[package] name`, with the owning workspace per crate). Status: the pattern below is derived from the manifests and the failing logs; no run in this list has passed yet. Items marked [U] are unverified.

### Why `-p` from the repo root fails for many stdio crates

- `cargo -p <name>` only selects packages of the workspace it runs in. Each stdio artifact directory (`🗿️artifacts/<artifact>/`) has its own `[workspace]`. The repo-root `Cargo.toml` and the stdio `Cargo.toml` list those directories as `exclude` (root: `🗿️artifacts/🎒️zip`, `🗿️artifacts/🗽️obj`, and others) or as members of their own workspace.
- Three queued runs failed for this reason, not because the names are wrong: `semio-s-artifact-stdio-zip-editor` (`h-zip-editor-test.log`), `semio-s-artifact-stdio-obj` (`e-obj-editor-test.log`), `semio-s-artifact-stdio-contract-editing` (`e-transform-test.log`). Each log reads `package ID specification ... did not match any packages`.

### Pattern

1. Look up the owner of the crate in `📦️crate-names.md` (column "Owning workspace"). The value is a repo-relative directory.
2. Change to that directory, then run the crate through `repo cargo` (store `shared`, configuration `prototype`, as the batch reports require):

```bash
cd "/home/deck/semio/<owning workspace directory>"
repo cargo --store=shared --configuration=prototype -- test -p <package> --lib <test filter>
```

- Use `--lib` for unit tests (the batch reports use it). Drop `--lib` for integration tests in `🧪️tests` and for crates whose tests are `#[path]`-mounted into a companion crate; for those, run the companion crate (its owner row lists it).
- Example: the zip editor is owned by `🗿️artifacts/🎒️zip` (`stdio/🎒️zip` in the table):

```bash
cd "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip"
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-zip-editor --lib
```

- Example: the obj root is owned by `🗿️artifacts/🗽️obj`:

```bash
cd "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj"
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-obj --lib
```

### Owner directories (from the index)

| Owner directory (repo-relative) | Crates it owns (examples) |
| --- | --- |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>` (one per artifact, 38 artifacts; the owner column of each row gives the exact directory) | the artifact root, its editors, io-registry, snapshot, mutations, schema and test crates |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model` | `semio-s-artifact-energy-model-editor` and its `-editor-*` and `-mutations` crates (energy model) |
| `🌎️hub/🧩️compositions/🗄️stdio` | `semio-hub-stdio` and the nine extension packages (`semio-hub-stdio-{bim,binary,cad,image,media,mesh,office,pdf,semio}`), plus component and catalog crates |
| `.` (repo root workspace) | `semio-framework-os-io`, `semio-framework-os-semio`, `semio-framework-plugin-tests-07`, `semio-framework-plugin-surface-fixture` and the other OS plugin test crates, and `semio-framework-os-renderer-wgpu-shell-tests-docs-2` |

Two crates have no owner (`member: no` in the index): `semio-s-artifact-energy-model-snapshot-bridge` and `semio-semio-v1-base-bridge`. See the last subsection.

### Bridge crates (`--manifest-path` case)

- The two bridge crates are not listed by any ancestor workspace. `cargo -p` from any directory cannot select them.
- Run them with the manifest path, from the crate directory:

```bash
cd "/home/deck/semio/<bridge crate directory>"
repo cargo --store=shared --configuration=prototype -- test --manifest-path "<bridge crate directory>/Cargo.toml" --lib
```

- Known risk [U]: if an ancestor `Cargo.toml` has `[workspace]`, cargo may refuse with `current package believes it's in a workspace when it's not`. The fix is a coordinator decision: add the crate to the nearest owner's `members` (energy model workspace for `energy-model-snapshot-bridge`; the semio v1 workspace for `semio-v1-base-bridge`). Do not edit the members list without that decision.

### Verification of this section

- Ownership and membership: read from `Cargo.toml` by TOML parsing only; not checked with `cargo metadata`. [U]
- `repo cargo` honouring the working directory as the workspace selector: [U]. Confirm once with a cheap `repo cargo ... -- metadata --no-deps --format-version 1` from one owner directory before queueing long runs.
- Heavy runs go through the shared capacity queue (one full build at a time); none of the commands above has completed in this ticket yet.
