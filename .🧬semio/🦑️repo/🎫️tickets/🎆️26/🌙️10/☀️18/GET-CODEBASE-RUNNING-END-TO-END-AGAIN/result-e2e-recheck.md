# result-e2e-recheck: static end-to-end re-check after the splits, router-law lifts and renames (2026-10-20 ~01:30)

Executor `e2e-recheck`. Static plus short loads only: no activation, serve, cargo, `show projects`, `dependencies sync`, download, Playwright browser launch. HARD FREEZE respected: nothing under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, no root `📜️script.ts`, no `package.json`/`bun.lock`, no `🔣️taxonomy.json` edited. No sub-agent, no modifying git command.

## 1. Verdict

| Product | Static chain | Config load | Specs list | Open before the first run |
| --- | --- | --- | --- | --- |
| course `@teaching/architecture-course` | green | Vite resolves (runner and native, build and serve) | Playwright `--list`: 76 tests, 16 files | Chromium 1248 download; proctor cargo build (blocked by value-family Rust, see result-course-img) |
| proctor `@teaching/proctor` | scripts build, all subcommands registered | n/a | n/a | cargo |
| play `@semio-tech/semio-tech-play` | green | **was broken (runner loader), fixed**; resolves with `native` | 157 tests, 1 file | Chromium 1248; activation (35 lanes, hours of cargo) |
| demonstrator `@semio-tech/mit-bestand-demonstrator` | green | **was broken (runner loader), fixed**; resolves with `native` | 12 tests, 1 file | Chromium 1248; activation (3 lanes) |

The one real defect found (F1) would have killed `serve` and the site `build` of play and demonstrator at config load, before any pane booted.

## 2. Findings

### F1 (FIXED) Play and demonstrator Vite configs cannot load under Vite's default `runner` config loader
- Symptom (reproduced with `resolveConfig`, Vite 7.3.7 under Bun, the exact call `serveVite` makes): `failed to load config from …/🏗️builder/🌐️vite/🟦️.ts` -> `Error: Captured inventory Source module load failed` (from `…/📚️library/🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs`, which does `import(URL.createObjectURL(blob))`; the module runner cannot import a blob URL).
- Cause: the configs pull the inventory/storage closure of the build tool. Play: `runtime/🟦️.ts` -> `@semio-tech/repo-lib/inventory/tooling` -> `📇️inventory/🟨️.mjs` -> observation -> digest read -> `📥️load/🟨️.mjs`; also `📦️packages/🟦️typescript/💾️storage/🟦️.ts` line 17 re-exports `runOwnedNxInvocationV1` from the nx bootstrap script, which reaches `🏗️build/🕸️graph`. Demonstrator: same `runtime/🟦️.ts` route. The course config does not reach it (resolves under both loaders).
- Precedent: the os dev builder, the wgpu server and the os distribution build already pass `configLoader: "native"` (the loader module documents that only `native` admits opaque dynamic imports). Play and demonstrator did not.
- Fix at the call site (root cause on the product side; the frozen library stays untouched): `configLoader: "native"` added next to `config: join(root, "🏗️builder/🌐️vite/🟦️.ts")` in
  - `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📜️script.ts` (serve, serve-test)
  - `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts` (serve, serve-test)
  - `🏢️semio-tech/🎡️play/🔨️modules/📦️site/🟦️.ts` (site build)
  - `♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/🟦️.ts` (site build)
  `ViteService` and `ViteArtifactBuild` both declare `configLoader`; the build path forwards it as `--configLoader native`. Each file still builds (`bun build --target=bun --packages=external`).
- Proof: `resolve-vite-config.mjs <config> build runner` -> FAILED for play and demonstrator; `… native` -> RESOLVED (play: 63 plugins, port 6033; demonstrator: 64 plugins, port 6029). Course: RESOLVED under both loaders, build and serve.
- Better long-term (frozen, queue for after the lift): cut the storage barrel's re-export of the nx bootstrap (`💾️storage/🟦️.ts:17`) and give `runtimeComponentClosure` a light entry, so product configs no longer inherit the build tool's closure. The call-site fix stays correct either way.

### F2 (FROZEN, report) `✂️node-native-typescript` law is stale: 54 vs 2 native entries
- `bun test ./🧰️framework/…/📚️library/🧪️tests/✂️node-native-typescript/🟦️.ts` -> 2 pass, 1 fail ("the packages vite/vitest configs externalize are exactly the declared native surface": declared `framework` + `framework-os`; received 54 entries, `framework-os` no longer among them). The router-law lifts moved many imports from relative to bare workspace specifiers, so the externalized surface grew.
- Consequence for the products: none under the `runner`/`native` loaders now used (F1). It matters for vitest/`bundle` configs: Node's own stripper refuses 6 files that these 54 entries reach (parameter properties): `🌐️browser-bundle/🌐️wasi/🟦️.ts` (via `🩹️preview2-shim-patches`), `📚️library/⚡️caching/🚀️bootstrap/📜️script.ts`, `…/⚡️caching/🧹️pruning/🟦️.ts`, `…/🧹️pruning/⏱️measurement/🟦️.ts`, `…/🔍️discovery/🔗️schema-reads/🟦️.ts`, `…/🧹️normalization/🏛️repository-authority/🟦️.ts`. `check-native-surface.ts` reproduces it (4 product Vite configs, 953 closure files).
- Owner: the library owner after the lift: either regenerate `nativeEntries` in `🧫️fixtures/✂️node-native-typescript/🔣️.json` and make the six files erasable, or move the products to loaders that do not leave bare entries to Node (done for play/demonstrator by F1).

### F3 (OPEN, not in the product closure) browser-bundle codegen pins jco 1.34.0 / jco-transpile 0.14.0 / preview2-shim 0.25.0
- Root `package.json` (deps owner) now pins jco 1.39.0 and preview2-shim 0.30.0; `jco-transpile` is only a transitive dependency of jco and is not resolvable from the script (`import.meta.resolve("@bytecodealliance/jco-transpile/component")` fails at `🔌️plugin/🌐️browser-bundle/📜️script.ts:319,372`). The schema, 5 fixtures and tests of `browser-bundle` still say 1.34.0 and assert the lock row versions.
- Reached only through `framework-os-dev:closed-browser-component-factory-check` and the `@semio-tech/browser-actor-import-test` project (dynamic import in a test command), not through `activate`, `prepare`, `serve` or `support` of any product. The activation transpile stage uses the workspace `jco` binary and is unaffected.

### F4 (observation) retired facade layer still on disk
`🧰️framework/📦️packages/🟦️typescript/🛍️products/**` holds 25 tracked one-line re-export entries (public package entries, e.g. `…/🕸️engines/🟦️.ts` re-exporting the real module through a 10-level `../`). The play Vite config and several scripts import through them. They resolve, so nothing breaks, but they are a compatibility-shaped layer; the old stale-import class of e2e-prep F1/F3 is gone: `rg -F` of the retired `…/🛍️products/🦑️repo/` prefix finds 0 files.

### F5 (my incident) `bun test` filter scan
I ran `bun test "<path>"` once without the leading `./`; bun treated it as a name filter and searched 7,478,077 files (199 s). That window coincided with a system-wide ENFILE (the Bash tool itself could not open its output files for a few minutes; `kern.num_files` was back to 8k afterwards, limit 245,760). Not repeated; every later run used `bun test ./<file>` (27 s). Lesson for the brief: `./` is mandatory.

### F6 (unchanged) Playwright browsers
Required by `playwright-core@1.64.1`: chromium 1248, chromium-headless-shell 1248, ffmpeg 1013. The repo cache holds 1234/1234/1011. Not installed (needs the download go-ahead). `run-e2e.sh <p> --check` reports exactly the three MISSING directories (exit 4).

### F7 note
Playwright must be started with Bun, as the product scripts do (`Bun.spawn([process.execPath, …/playwright/cli.js …])`). `node …/playwright/cli.js test --list` on the play and demonstrator configs fails with `ERR_UNSUPPORTED_ESM_URL_SCHEME`; under Bun both list cleanly. Do not use `node` in any ad-hoc command.

## 3. Checks (what was re-run, outcome)

1. **Target closure** (`e2e-tools/target-script-closure2.py all`): 202 tasks over the 4 products (play, demonstrator, course, proctor), follows `dependsOn` (object and `^` forms), maps the 89 inferred `framework-os-dev` tasks to the os-dev script and the 43 inferred `plugin-registry:session-*` to the registry script, adds the support/fonts/describe/wasm-opt/browsers dependencies. 28 distinct command scripts, 0 unresolved projects, 0 missing explicit targets, 0 `^` edges. `check-variants.ts`: all 43 playground variants the targets need are declared in Cargo manifests (156 declared).
2. **`bun build --target=bun --packages=external`** of 89 entries (the 28 closure scripts, registry, os-dev distribution + wgpu native-entrypoint, cargo workspace script, root `📜️script.ts`, all 46 hub `🦀️rust/📜️script.ts`, the 4 Vite configs, 3 Playwright configs, play/demonstrator specs, course gate + learner spec): **89 ok, 0 errors** (`bun build` demonstrably fails on a deliberately broken relative import; checked).
3. **Bare + relative specifier walk** (`e2e-tools/check-closure.ts`, Bun resolver, regex scanner because `Bun.Transpiler.scanImports` mangles non-ASCII specifiers): 2,573 first-party files from the 89 entries, 66 unresolved strings, all false positives (law fixtures, generated worker source text, test fixtures `removed/index.ts`, `external`, ticket strings); the single non-test real hit is F3. The 16 course spec directories + pet-kit + config: 57 files, all resolve.
4. **Module evaluation**: `bun -e 'await import(<script>)'` for the 78 script entries: 30 load cleanly; 48 stop at their unguarded top-level `await receiveScriptProcessInvocation` with "Original parent Script process invocation required" (expected without an envelope; every import before it evaluated).
5. **Subcommands** (`e2e-tools/check-subcommands.py`): 70 `bun <script> <command>` project.json commands across the closure: every command is registered in its script's router closure (strict `register("…")`/`registerLazy("…")`), 0 missing. Also read by hand: `prepare`/`activate`/`serve`/`session`/`build`/`complete`/`prepare-test`/`serve-test`/`test`/`test-enduser`/`test-release*`.
6. **Vite configs** (F1): real `resolveConfig` of 4 configs x loader x command (table in F1). The demonstrator shell config (`🐚️shell`) loads up to the generated session/activation receipt it needs (`dist/sessions/s/…`, `runtime/react/s/activation/🔣️receipt.json`), which exist only after activation: expected.
7. **Playwright**: configs import and evaluate (`PLAYWRIGHT_BASE_URL` dummy); `bun node_modules/playwright/cli.js test --list`: play 157 tests / 1 file, demonstrator 12 tests / 1 file, course 76 tests / 16 files (projects boot, desktop, phone, layout, presence, shortage, away, faults, pets). `testDir`/`testMatch` paths exist, 16 course `testMatch` directories each hold `🟦️.ts`.
8. **Selectors/routes** (`e2e-tools/check-selectors.ts`, 48,783 files): play 58 tokens / 47 found, demonstrator 38 / 35, course 81 / 81 - identical to e2e-prep; every miss is explained as before (template placeholders, fixture-internal ids, `${slot}-open-chip` composed by `OverviewCardOpenChip` with `slot="demonstrator-pane-card"`, `procedural-play-inspector` prefix composed in the generation3d inspection panel Rust, `[data-host-snapshot-json]` is the CSS form of a found attribute). Routes are hash deep links (`./#<variant>`, `/#<pane>`), unchanged. Logo assets of the demonstrator brand exist under `🖼️asset/🪧️logos/*`.
9. **Ports** consistent: play 6033 (Vite config, project.json env, dashboard ready), demonstrator 6029 (Vite config, `🧫️pipeline.json`, project.json), course dev 6061 (project.json, dashboard fixture, package.json), proctor 8791.
10. **`run-e2e.sh` chain targets** exist: play/demonstrator `activate-prototype`, `serve` (continuous), `prepare-e2e`, `serve-e2e` (continuous), `test-e2e`; course `test-e2e`, `dev` (continuous); `@teaching/proctor:build`; `workspace:deps-browsers` (command `sync browsers` -> `playwright install chromium`, still pinned to `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`). Course gate flags `--topology --serial --keep --proctor` match the gate's parser.
11. Not run: `bun repo run … --dry-run` (needs the graph the coordinator serializes; load average was 216 and `bun repo show projects` was running), any activation, serve, cargo, browser launch, Docker.

## 4. Changes

Product files (4, one token each): see F1.

Ticket inputs (`C/e2e-tools/`, not permanent tooling):
- `run-e2e.sh`: new `--serial` (course gate), new preflight `check_vite_config` (play/demonstrator: loads the Vite config with the loader the runtime script names, aborts with exit 3 on failure, ~5 s). Targets unchanged, so no chain edits. `bash -n` clean; `play --check` and `demonstrator --check` print `RESOLVED …` then stop at the browsers gate (exit 4) as intended.
- `target-script-closure2.py` (full closure incl. generated targets), `check-closure.ts` (specifier walk; `CHAIN=1`, `TRACE=<substring>`, `SKIP=<substring>`), `check-subcommands.py`, `check-variants.ts`, `check-native-surface.ts`, `resolve-vite-config.mjs`.
- Temporary outputs (`C/🗑️generated/e2erecheck-tmp`) deleted. Unchanged from e2e-prep: `check-selectors.ts`, `stale-imports.ts`, `target-script-closure.py`.

## 5. Open items (owner)

1. After the freeze lift (library owner): F1 long-term (storage barrel line 17, light inventory entry) and F2 (native-surface law fixture + 6 parameter-property files).
2. Deps/plugin owner: F3 (jco pin drift: either pin root back to 1.34.0 or re-qualify browser-bundle policy, fixtures and the lock-row check for 1.39.0 / jco-transpile 0.20.0 / preview2-shim 0.30.0, and resolve `jco-transpile` through jco's own location).
3. Permanent regression for F1 (test owner): a language-agnostic law that resolves each product Vite config through its declared loader (`resolveConfig`, `build`) with a dummy bounded cache; `e2e-tools/resolve-vite-config.mjs` is the working oracle.
4. Coordinator: browser download go-ahead (`bun repo run workspace:deps-browsers`), then `run-e2e.sh demonstrator`, `play`, `course --topology=dev --warm-proctor` as in result-e2e-prep section 5.
5. Optional: add `workspace:deps-browsers` to course `test-e2e` `dependsOn` (bzl-man owns the manifest).
