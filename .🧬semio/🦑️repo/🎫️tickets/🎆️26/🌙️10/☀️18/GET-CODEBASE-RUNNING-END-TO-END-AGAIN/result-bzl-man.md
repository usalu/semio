# result-bzl-man: manifest cacheability for course, proctor, play, demonstrator (2026-10-19)

Executor `bzl-man`. Static only: no builds, no `bun repo run`, no cargo, no activation. Inputs: `audit-bazel-state-1019.md` (G7, G10, WP2), `audit-products-1019.md` section 1, `brief-1019.md`.

## Files changed

| File | Change |
|---|---|
| `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json` | `default` named input completed; `test` explicit inputs; `typecheck` cached |
| `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json` | `cargoSources`, `cargoTests` named inputs; `build` and `test` cached |
| `🏢️semio-tech/🎡️play/📋️project.json` | `activationSources` named input on `prepare-prototype`, `activate-prototype`; `test` made explicitly uncached |
| `♻️mit-bestand/🧺️demonstrator/📋️project.json` | same as play; `build` uncached; `test`, `canonical-architecture` explicit |
| `📋️build-owners.json` | row `@teaching/architecture-quiz` replaced by `@teaching/architecture-course` (root and manifest under `📚️course/📦️packages/🟦️typescript`), same list position |

Nothing else was touched. The codex course ticket's changes (dashboard metadata, preflight, compounds, launch-related target metadata) are untouched; the diff against HEAD of the course file is only the additions below.

## Findings that shaped the decisions

1. **Activation reads (G7 open question answered).** Play `activate` (`🔨️modules/🧩️runtime/♻️activation/🟦️.ts`, `readPlayActivation`) reads the per-lane receipts under `.cache/b/p/m/browser/prototype/runtime/react/<lane>/activation`, hashes the installed plugin-modules root and the infinite font asset (all declared outputs of the `framework-os-dev:*` and `infinite:fonts` dependencies, reached by `dependentTasksOutputFiles` transitive), imports play's own `🟦️.ts` pane layout and the workspace packages `framework-os-dev`, `s-dev-catalog-ts`, `plugin-registry`, `repo-lib`, `framework-plugin`, `framework-canvas-ts` (all devDependencies of play), and writes the union receipt to `{projectRoot}/dist/♻️activation/prototype` (matches the declared output). Demonstrator `activate` is identical in shape (`DEMONSTRATOR_UNION_RECEIPT_DIRECTORY`). Both targets had ONLY `dependentTasksOutputFiles` as inputs, so project sources and imported workspace code were never in the key. Fixed with `activationSources` = `production` + `^production` (workspace dependency sources, test-free) minus `dist`/`test-results`, plus the dependency outputs.
2. **Vite builds do not write `{projectRoot}/dist`.** `buildViteArtifact` runs `vite build --outDir <tmp>` and stages into the immutable products store `.cache/b/p/m/vite/<mode>/<sha256(owner)[:16]>/out` (`viteStorageLayoutV1`). `demonstrator:site` = `5dc1b077e86de2aa`, `architecture-course:site` = `e483dc668ec8bf18`. So the demonstrator `build` declared output `{projectRoot}/dist/{mode}/site` is never written, and restoring into a lease-protected store from outside is unsafe. Consistent with the framework's own `build-<variant>-react-release` (`cache:false`, orchestrator) and with the course (`build` pinned `cache:false, outputs:[]` by `🧪️tests/🧪️deploy/🟦️.ts:146`). Demonstrator `build` is therefore `cache:false` (the pinned output string stays because `🧩️runtime/🧪️tests/🟦️.ts:43` pins it).
3. **Rust output boundary (G10).** `proctor:build` runs `buildAdmittedCargoArtifactsV1`; artifacts are captured in the shared compiler store (`<store>/cargo/<mode>/captures`, `repositoryCompilerLayoutV1`), not under the project. Nothing consumes the `build` target's files (dev/check/e2e rebuild through `bun repo cargo` and pin their own copy). Boundary accepted: `cache:true`, `outputs:[]` = the cache records "this exact source closure built", the binary lives in the compiler store. Inputs are exact: workspace `cargo` group + `Cargo.lock` + proctor `.rs`/`Cargo.toml` + `^cargo` (the dependency-edge inference reads Cargo path dependencies, `🕸️dependency-edges/🟨️.mjs:187-193`, so the 73-crate path closure is reached without listing it; computed closure verified with a Bun TOML walk).
4. **Test hermeticity.** Play and demonstrator `test` read repository-wide files (plugin descriptors, other crates' `Cargo.toml`, sibling `📋️project.json`, a ticket `🗑️generated` folder in `playpages`), so project-scoped inputs cannot be sound: explicit `cache:false`. Course `test` was `cache:true` with a large `default` that missed files its tests actually read (`.github/workflows/architecture-course.yml`, root `package.json`, both READMEs, `Cargo.toml`s used by `proctorSourceDirectories`, dashboard registry schema, `.vscode` existence checks): added.
5. **`default` shadowing.** Each product's `default` replaced the workspace `default`, silently dropping `sharedGlobals` (platform, bun version, `SEMIO_BUILD_MODE`). Re-added. Bazel adapter uses `{projectRoot}/**/*` when `inputs` is absent while nx uses `default`; every cacheable target now has explicit `inputs`.

## Before / after per target

Legend: `cache` c; `inputs`: D = project `default`, AS = `activationSources`, DTO = `{dependentTasksOutputFiles:"**/*",transitive:true}`.

### `@teaching/architecture-course`

| Target | Before | After | dependsOn |
|---|---|---|---|
| preflight | c=false, outputs [] | unchanged (host introspection) | none |
| ci-browser-prepare | c=false | unchanged (installs system packages) | none |
| dev, dev-site | c=false, continuous | unchanged | assets:build, ui-styling-tokens:generate |
| build | c=false, inputs D + env PROCTOR_URL, outputs [] | unchanged, test-pinned; D now complete (see below) | assets:build, tokens:generate |
| publish | c=false, outputs `{projectRoot}/dist/release/pages/course` | unchanged (side effecting stage for CDN) | assets:build, tokens:generate |
| test | c=true, no inputs (= D), outputs [] | c=true, inputs [D], outputs [] | none |
| test-e2e | c=false, outputs [] | unchanged (browser, ports, processes) | assets:build, tokens:generate |
| typecheck | c=false, outputs [] | c=true, inputs [D, DTO non-transitive], outputs [] (tsbuildinfo is in `node_modules/.cache`, not an output) | framework-rs:generate, ui-contract-rs:generate, ui-rs:generate, framework-actor-rs:typegen, tokens:generate, assets:build (unchanged) |
| check | c=false | unchanged (builds and launches the proctor from the store) | none |
| docker-image-build/-check/-publish, docker-stack-check, docker-stack-bundle, deploy-check | c=false | unchanged (Docker daemon side effects; bundle keeps `dist/proctor` output) | deploy-check: assets, tokens, typecheck |

Course `default` additions: `!{projectRoot}/dist/**`, `sharedGlobals`, `externalDependencies` (`@noble/hashes ajv jsonc-parser picomatch resolve.exports tar-stream typescript vite vitest yaml`, all present in `bun.lock`), the two READMEs, `.github/workflows/architecture-course.yml`, root `package.json` and `tsconfig.json`, root, proctor, framework-course and framework-server `Cargo.toml`, dashboard registry schema, `.vscode/launch.json|tasks.json` (existence checked by the dashboard test).

### `@teaching/proctor`

| Target | Before | After |
|---|---|---|
| build | c=false, no inputs/outputs | c=true, inputs [`cargoSources`], outputs [] |
| test | no cache flag (false), no inputs | c=true, inputs [`cargoTests`], outputs [] |
| dev | c=false, continuous, ready 8791 | unchanged |
| check, rebuild, health, backup, restore, erase, prune, capacity | c=false | unchanged (operator verbs on the dev data directory; capacity is a timing gate) |

`cargoSources` = `cargo`, `^cargo`, `sharedGlobals`, `Cargo.lock`, proctor `**/*.rs` and `**/Cargo.toml`, `🏗️bootstrap/🟦️.ts`, `📜️script.ts`, minus `target`. `cargoTests` = `cargoSources`, `default`, framework course `🧫️fixtures` and `🧬️schema` (proctor tests `include_str!` two course fixtures).

### `@semio-tech/semio-tech-play` (:6033)

| Target | Before | After | dependsOn |
|---|---|---|---|
| prepare-prototype | c=true, inputs [DTO], outputs [] | c=true, inputs [AS, DTO], outputs [] | 35 `framework-os-dev:prepare-<lane>-react-prototype` |
| activate-prototype | c=true, inputs [DTO], outputs `dist/♻️activation/prototype` | c=true, inputs [AS, DTO], same output | prepare-prototype + 35 `activate-<lane>-react-prototype` |
| dev, serve | c=false, continuous | unchanged | activate-prototype |
| build | c=false, orchestrator, output `.cache/b/p/m/play/{mode}/pages` | unchanged (pinned `cache:false` + orchestrator by `🧪️playfreshbuild`) | none (job graph inside metadata) |
| test | unset | c=false, outputs [] (explicit) | none |
| prepare-e2e, serve-e2e, test-e2e, test-e2e-enduser, test-release, test-release-routing, release-verify | c=false | unchanged | chain intact: activate-prototype -> prepare-e2e -> serve-e2e (continuous) -> test-e2e / test-e2e-enduser; deps-browsers on prepare-e2e and test-release |

### `@semio-tech/mit-bestand-demonstrator` (:6029)

| Target | Before | After | dependsOn |
|---|---|---|---|
| prepare-prototype, prepare-release | c=true, inputs [DTO], outputs [] | c=true, inputs [AS, DTO] | `framework-os-dev:prepare-<variant>-react-<profile>` + 4 wasm |
| activate-lanes-prototype | c=false | unchanged (log-only fan-in) | 3 `activate-*-react-prototype` |
| activate-prototype | c=true, inputs [DTO], output `dist/♻️activation/prototype` | c=true, inputs [AS, DTO], same output | prepare-prototype, activate-lanes-prototype |
| dev, serve | c=false, continuous | unchanged | activate-prototype |
| build | c=true, inputs unset (= D, no dependency outputs), output `dist/{mode}/site` | c=false (see finding 2); output string kept (test-pinned) | prepare-release + 4 wasm |
| test, canonical-architecture | unset | c=false, outputs [] (explicit) | none |
| test-runtime-catalog, prepare-e2e, serve-e2e, test-e2e | c=false | unchanged | chain intact: activate-prototype -> prepare-e2e -> serve-e2e -> test-e2e |

Both play and demonstrator: `default` gained `!{projectRoot}/dist/**`, `!{projectRoot}/test-results/**`, `sharedGlobals`.

## Verification

| Check | Command | Result |
|---|---|---|
| Neutral schema, repo validator | scratch Bun script calling `validateJsonSchemaSubset` (`🔨️modules/🧬️schema/✅️validator`) with `🏗️build/🧬️schema/📋️project/🔣️.json` on all four files | 0 errors each |
| Neutral schema, third-party | same script, Ajv draft-07 (`allErrors`) | valid each |
| JSON syntax of owners file | `python3 -c json.load` | valid; only the one row changed |
| Pinned manifest assertions (demonstrator) | `SEMIO_WORKSPACE_ROOT=. bun test "./♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts"` | all project.json asserts (build deps/outputs/command, activation deps/outputs, serve/dev, e2e chain, prepare cache true) pass; test stops later at line 75 `41 !== 35` (runtime asset source count vs fixture), which does not read the manifest and is not related to these edits (not fixed here) |
| Input expansion (Bazel adapter semantics) | scratch replica of `taskInputs` + `git ls-files` match | course test/typecheck 3,238 files; proctor build 44 (own tree, `^cargo` excluded in the replica), proctor test 194; play prepare/activate 301; demonstrator prepare/activate 817; exclusions and groups resolve without cycles |
| externalDependencies names | `bun.lock` lookup | all 10 present |
| Not run | `bun repo show projects`, nx graph load, any `--tool=bazel` dry-run, vitest suites of course/play, cargo | not run per brief |

Scratch scripts lived in `🗑️generated/bzl-man-tools` and were removed.

## Open items (for the coordinator / other owners)

1. **Adapter (WP3).** Bazel `taskInputs` drops `dependentTasksOutputFiles` objects (G7); until it expands them, the activation/prepare/typecheck keys in Bazel lack dependency outputs (Bazel does pass dependency snapshots as action inputs, so correctness holds but with coarse granularity). `^group` expands over direct edges only, nx over the transitive closure: `proctor` `^cargo` and play/demo `^production` under-approximate in Bazel. Negation patterns inside named groups work in the adapter (`excluded` filter).
2. **Store-aware outputs.** Vite builds (course build/publish, demonstrator build, play pages) and cargo captures live in leased stores; a Bazel/nx restore into them is unsafe, so those stay `cache:false`/`outputs:[]`. Correct fix: publish to a declared output tree first and let the store ingest it, or let the adapter treat the store as the output.
3. **Demonstrator `build` output string** `{projectRoot}/dist/{mode}/site` is wrong (never written) but pinned by `🧩️runtime/🧪️tests/🟦️.ts:43`; change both together when item 2 is decided.
4. **Map tile prefetch** (`📦️site/🗺️map-tiles`) downloads into `.🧬semio/🗺️map/**` (ignored, network): not hashable. Lifting it into its own target (play already has the `map-tiles` job) would make demonstrator `build` cacheable.
5. **Tests with repo-wide reads** (play, demonstrator): need fixture extraction or a declared repository-read input set before `cache:true`.
6. **Release chain decision:** play `release-verify` and `test-release` read the pages `build` publishes but have no `dependsOn` on it (`release-verify` is explicitly `[]`). Adding `build` makes `release-verify` start a full release build; left as is.
7. **Owner registry:** `📋️build-owners.json` still has stale framework rows `@semio-tech/quiz`, `quiz-react`, `quiz-rs`, `quiz-*-tests-rs` at `🧰️framework/🛍️products/❓️quiz/**` and no rows for `@semio-tech/course`, `course-react`, `course-rs` (only `course-*-tests-rs` / `course-schema-rs` exist). Not mine; a registry refresh by the framework course owner is needed (`❓️quiz/📦️packages/🟦️typescript` holds only `node_modules`).
8. **Peer conflict (not mine):** the course dashboard test asserts `.vscode/launch.json` and `tasks.json` do not exist while the codex ticket's readiness report claims restored launch entries.
9. **Play/demo `package.json` scripts** use `bun repo run`; course uses `bun nx run`; AGENTS says `nx` (decision pending, deps owner).
