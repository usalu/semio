# result-cache-rest: cache coverage for hub, teaching, semio-tech, mit-bestand, leutwiler and the root workspace (2026-10-19)

Executor `cache-rest`. Static edits only: no `bun repo`, no nx graph, no cargo, no activation. Inputs: `brief-1019.md`, `audit-cache-coverage-1019.md`, `result-bzl-man.md` (pattern), the assembly source `🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs` and the Bazel adapter (`taskInputs`, `bazelCacheDecisionV1`).

## Result in numbers (695 explicit targets in 63 manifests; census re-run)

| Census class | Before | After |
|---|---|---|
| deterministic-cacheable | 351 | 515 |
| deterministic-but-cache-off | 77 | 24 |
| unclassified-cache-off | 171 | 60 |
| continuous / side-effecting / live / diagnostic | 40 / 44 / 11 / 1 | unchanged |

Target edits: 165 `cache:false` to `cache:true`; 120 implicit (absent) to explicit `cache:true`; 6 made explicitly `cache:false` where the policy already forced it or the task rewrites committed assets (`test-doctor` was `true` against the policy, `stdio-fuzz`, `directory-live-lanes`, hub-stdio-publication `publish`, projektetage `canonical-architecture`, remodel `regenerate-example`); 246 `outputs` added; 149 `inputs` added; 58 manifests touched. The 84 remaining cache-off deterministic or unclassified targets all appear in the keep-off table.

## How the decisions follow the machinery

- Bazel disables the cache only for `cache:false` or `continuous` (`bazelCacheDecisionV1`); an absent `cache` is cached. nx applies `policy.json` by name (`targetPolicy`): `verify` commands and the cacheable families are cached unless the manifest says `cache:false`. The fix for most targets is therefore removing the explicit `false` and making the declaration explicit.
- `project-assembly` already appends the script import closure and `externalDependencies` to every cached generic target, and replaces the inputs of Cargo-owner targets by `nativeSources`/`nativeTestSources` plus the command closure and lock inputs. Explicit inputs were declared only where a task reads files that neither mechanism sees (fixture-named files, sibling test trees, dependency outputs), and project-level named inputs carry the lists of aggregators. Explicit target inputs on native-assembled targets are ignored by the assembly (open item 2).
- The audit rule "no explicit `cache:false` on a cacheable family" was applied manifest by manifest. A task stays off only when it writes what it reads, re-enters the build tool, drives a browser/hub/Docker, installs or publishes, or its products live in a leased store.

## Changes

### Root workspace `📋️project.json`
- `namedInputs.default` now includes `sharedGlobals` (platform, bun version, `SEMIO_BUILD_MODE`): the root `default` override had shadowed it, the same defect bzl-man fixed in the products.
- 122 `bun ./📜️script.ts verify …` tasks (ownership, contract, oracle and `-native` laws; framework, viewport, pack, retained, window and fem laws): `cache:true`, `outputs:[]`, `inputs:["default"]`. They read the workspace (TS laws plus `runCargo` crates); the root `default` is the whole workspace, so the key is complete (coarse but sound). They now match the siblings `test-window-view-context*`, which were already cached.
- `agents-check`: precise inputs (`📋️project.json` declaration, `.mcp.json`, `.codex/config.toml`, `sharedGlobals`), `outputs:[]`.
- `generate-config-check`, `schema-entries`, `schema-parsers-report`, `schema-parsers-check`, `test-command-entrypoint`, `test-runtime-build-producer`, `cpp-custody`, `cpp-test-toolchain`, `examples`: cached, `outputs:[]`.
- `schema-verify`, `schema-fixture-boundary`: cached with `["default","schemaSources"]` (the narrower `schemaSources` missed the Rust registry entries and the fixture tree they read).
- `test-canvas-native`, `test-canvas-react`: cached with `default` plus the three test-level environment inputs.
- Explicit `cache:true`/`outputs:[]` on 19 test/stdio laws; explicit `cache:false` on `test-doctor` and `stdio-fuzz`.

### Hub
- `os-hub`: `unit-census-check` (cargo unit graph over all manifests; inputs add every `Cargo.toml`/`Cargo.lock`, toolchain, config, cargo environment), `component-codec-check` and `component-codec-budget-check` (inputs add fixture, schema and `dependentTasksOutputFiles` of the `component` producers whose wasm the laws read from `dist/components`); `test`, `test-all-features` (dependency outputs), `test-inference-client-contract` explicit; `directory-live-lanes` explicit `cache:false`.
- `hub-auth-client-rs`: `check`, `test`, `test-source` (inputs name the credential protocol sources the source law reads).
- `flow-plugin`: new named input `graphGenerateSources` (the former inline list of `graph-generate`, now referenced by it); `preview-generated` cached on it plus the generated tree; `graph-wire-check` cached with dependency outputs.
- `gis-plugin`: `installed-service-source` (adds the `🧪️tests` tree). `draw-plugin`: `guest-instance-check` (dependency outputs of the component).
- `hub-compositions`: `ownership`, `typescript`, `native`; new named input `rustWorkspace` (Rust sources, manifests, lock, toolchain, cargo and test environment).
- `stdio-plugin`: `composition-contribution-check`, `mutation-inventory`, explicit cache/outputs on the snapshot and check tasks. `hub-stdio-test-oracle`: seven tasks cached on new named inputs `stdioSources`/`rustWorkspace`. `puzzle-retained` and `space-testing` `test-ownership`: inputs enumerate the files named by the ownership fixture (computed from the fixture). `space-plugin`: `snapshot-owner-census-source|native` (fixture tree and the SQL files the fixture names). `space-plugin-compute-ownership:test` and `hub-renderer-wgpu:test` (adds the generic wgpu package it imports and executes).
- Tidy: explicit `cache:true` and `outputs:[]` on the remaining implicit tests and checks of the single-plugin manifests; `regenerate-example` made explicitly uncached (rewrites committed PNG/JSON assets).

### Products
- `mit-bestand-demonstrator:activate-lanes-prototype` cached (log-only fan-in, `outputs:[]`). Everything else in course, proctor, play and the other product manifests is as bzl-man left it; no value changed there.

## Keep-off list (reason per task)

| Manifest | Tasks | Reason |
|---|---|---|
| root | `agents-write` | rewrites the MCP client files from the root declaration in place |
| root | `generate-config` | rewrites tracked scope nextest files from the root nextest file in place (reads what it writes) |
| root | `schema-parsers-apply`, `schema-parsers-restate` | projects parsers into tracked sources (writes what it reads) |
| root | `verify-canonical-architecture` | orchestrator: re-enters the build tool (`run-many -t canonical-architecture --skip-cache`) over every project |
| root | `build`, `release`, `components` | orchestrator or release pipeline; products live in leased stores (plugin modules, vite store) |
| root | `build-storybook` | storybook static build, leased vite store output, port env |
| root | `build-canvas-font-assets` | cargo build whose artifacts are captured in the shared compiler store |
| root | `cpp`, `cpp-setup`, `cpp-configure`, `cpp-build`, `cpp-test` | CMake chain: toolchain install, build tree in the native store consumed downstream, host compiler probes |
| root | `cargo-prepare` | cargo workspace preparation writes generated manifests and store observations |
| root | `runtime-fixture-graph` | builds a production fixture graph in a private cargo store (compiler captures) |
| root | `test-build-tool-bazel`, `test-build-tool-bazel-bootstrap` | runs real Bazel actions and bootstraps the pinned Bazel binary (download, output_user_root) |
| root | `format`, `clean-*`, `new-taxonomy-mutation` | policy-pinned: `clean*`, `format`, `new` are forced uncached by the workspace policy (`clean-taxonomy-inventory|plan|verify` look read-only, see open item 8) |
| root | `bench-plugins`, `os`, `semio` | benchmark or launches the OS shell |
| root | `setup*`, `deps-*`, `publish` | installs or publishes: mutates host, lockfiles or network |
| root | `dev-*` (storybook, mcp) | continuous |
| hub `os-hub` | `runtime-fixture-graph` | private cargo store, compiler captures |
| hub `os-hub` | `setup`, `publish` | cargo fetch (network); publishes the hub distribution |
| hub `os-hub` | `live-sign-in-check`, `admin-live-journey-check`, `directory-live-lanes` | live: boots a real hub child, signs in over HTTP, or needs live backends |
| hub `os-hub` | `browser-actor-child-worker-containment-check`, `browser-actor-gis-describe-check`, `hub-document-sweep`, `io-matrix` | drive Chromium against a loopback server and write timestamped acceptance records |
| hub `os-hub` | `trusted-catalog-preflight` | reads built plugin components from the products store, outside declared inputs |
| hub `os-hub` | `trusted-catalog-bootstrap`, `secure-local-smoke`, `dev-collaboration` | publishes a catalog generation or runs against a live local hub |
| hub `os-hub` | `hostile-input-check` | publishes a timestamped acceptance record, seeded generative laws |
| hub `os-hub`, `puzzle-plugin`, demonstrator, projektetage | `canonical-architecture` | `policy.targetDefaults` pins it uncached |
| hub `os-hub-ts` | `two-client-e2e`, `document-growth-e2e` | e2e: boots the real hub binary with two clients |
| hub `os-hub-ts` | `backend-up|down|status|run`, `docker-image-build|check` | Docker daemon side effects |
| hub `os-hub-ts` | `backup-restore-drill`, `shutdown-drill` | operational drill against a built local hub with timing assertions |
| hub `os-hub-ts` | `residency-watch`, `boot-watch`, `hub-freshness`, `agent-ceiling-check` | need a running hub URL (live observation) |
| hub `gis-plugin` | `hub-edit-durability-check` | restarts a hub process to observe durability |
| hub `stdio-plugin` | `artifact-directory-wiring-generate`, `subset-directory-wiring-generate` | rewrite tracked wiring sources in place (the `-check` twins are cached) |
| hub `stdio-plugin` | `native-codec-projection` | read-modify-write of the tracked factory, definition and catalog files it also reads |
| hub `stdio-plugin` | `composition-prepare` | rewrites tracked manifests and generated sources in place |
| hub `stdio-plugin` | `catalog-root`, `editor-component-check` | caller-owned empty build root plus cargo release build; links wasm into the shared cargo target |
| hub `stdio-plugin` | `parent-removal-check`, `parent-removal-test-check` | need a copied workspace snapshot argument and write proofs inside it |
| hub `puzzle-retained`, `space-testing` | `test-native` | orchestrators re-entering the build tool with `--skip-cache` for each listed test project |
| hub `hub-renderer-wgpu` | `journey-check`, `creation-check`, `collaboration-check`, `native-guest-journey-check`, `collaboration-acceptance` | live: boots a hub, mounts the staged native runtime, acceptance records |
| hub, play, course, proctor | `*-e2e`, `prepare-e2e`, `serve-*`, `dev-*` | e2e or continuous (browser, ports) |
| demonstrator | `build` | vite site lands in the leased products store, downloads map tiles (see bzl-man) |
| demonstrator, play | `test`, `test-runtime-catalog`, `test-release`, `test-release-routing`, `release-verify` | read repository-wide files or release pages of the products store (see bzl-man) |
| play | `build` | orchestrator publishing pages into the leased store |
| course | `preflight`, `ci-browser-prepare`, `build`, `publish`, `check`, `docker-*`, `deploy-check` | host introspection or install; vite store output (test-pinned); builds and launches the proctor; Docker daemon |
| proctor | `check`, `rebuild`, `health`, `backup`, `restore`, `erase`, `prune`, `capacity` | operator verbs on the dev data directory; `capacity` is a timing gate |
| leutwiler | `deps` | installs the Lean toolchain |
| remodel | `regenerate-example` | rewrites committed example assets |

## Verification

| Check | Command | Result |
|---|---|---|
| Census before/after | `bun census.ts`, `bun census-after.ts`, `bun diff.ts`, `bun delta.ts` in `cache-rest-tools` (copy of the audit census) | table above |
| JSON syntax | each edit re-parsed in `apply.ts` before writing | valid |
| Neutral schema, repo validator | `bun validate.ts`: `validateJsonSchemaSubset` with `🏗️build/🧬️schema/📋️project/🔣️.json` over all 123 manifests of the scope | 0 invalid |
| Neutral schema, third party | same script, Ajv draft-07 `allErrors` | 0 invalid |
| Concrete input paths | `apply.ts` checks every non-glob `{workspaceRoot}/…` path or glob base it adds | 0 missing |
| Idempotence | second `apply.ts` run | 0 edits |
| Pinned policy tests | grep of the caching, build, commands, agent-clients and root test trees for the changed task names | no pin; `⚡️cache-contracts` asserts root `build` cache true (already false before this task, untouched) |
| Not run | `bun repo show projects`, nx graph, any `--tool=bazel` run, cargo, the changed tasks | not run per brief; assembled inputs are derived from source reading, not observed |

## Open items

1. **Observed graph.** Confirm the assembled inputs of the flipped tasks with the coordinator's next graph build (`unit-census-check`, `component-codec-*`, `preview-generated`, root verifiers).
2. **Native-assembled targets ignore explicit target inputs** (`nativeTarget` replaces `policy.inputs`). For `hub-auth-client-rs:test-source`, `snapshot-owner-census-*`, `component-codec-*` the extra paths only take effect via the project `namedInputs.default`/`nativeTestSources`. If the observed graph shows them dropped, move the lists into the project named inputs.
3. **nx dependency outputs.** `component` producers are uncached; `dependentTasksOutputFiles` is declared on their consumers but dropped for native-assembled ones (item 2). Bazel is unaffected (dependency snapshots are action inputs).
4. **Root verifiers hash the whole workspace.** Sound but invalidated by any edit. Narrowing needs per-law lists (test root plus the Rust path closure of each `-p` crate); a generator from the verify router and the Cargo path graph would produce them.
5. **Existing cached tasks that need live services** (untouched): hub `*-postgres-check`, `*-neo4j-check`, `*-process-check`, `directory-home-browser-process-check`, `local-bootstrap-launch-check` are `cache:true` although they start processes or need Docker backends.
6. **`verify gate`, `verify layering`** (root, implicit cache) re-enter the build tool with nested runs; they behave like orchestrators.
7. **Nested `--skip-cache` orchestrators** (`puzzle-retained:test-native`, `space-testing:test-native`) would be graph-native as `dependsOn` of the listed projects' `test` targets; the project list lives in a fixture, so the edges need generating from it.
8. **Policy-pinned names** (`canonical-architecture`, `clean-taxonomy-inventory|plan|verify`) cannot be cached from a manifest; they look read-only and deterministic, so the policy owner (cache-fw) may relax them.
9. **Moved crate in flight.** The census saw a tracked but absent `Cargo.toml` at `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust` (a peer move); the after-census tolerates it.

## Files

Tools (kept) in `cache-rest-tools/`: `census.ts`, `census-after.ts`, `patch.ts` (offset-based JSON editor), `plan.ts` (decisions and reasons), `apply.ts`, `validate.ts`, `delta.ts`, `diff.ts`, `scope.ts`, `list.ts`, `by.ts`, `cmds.ts`, `absent.ts`, `susp.ts`, `mismatch.ts`, `work.ts`, `show.ts`, `rt.ts`, `cnt.ts`, `fixpaths.ts`. Scratch outputs under `🗑️generated/cache-rest-*` were removed.

Edited manifests (58): root `📋️project.json`; hub `📦️packages/🦀️rust`, `📦️packages/🟦️typescript`, `🔨️modules/🛡️admin/📦️packages/🟦️typescript`, `🔐️auth/🔌️client/📦️packages/🦀️rust`, `🧩️compositions`, the composition plugins (`🌊️flow`, `🌍️gis`, `🖍️draw`, `🗄️stdio` incl. `🔮️oracles`, `📇️publication`, nine `🧩️extensions`, `🧩️puzzle` incl. `🧵️retained`, `🪐️space` incl. `🧪️testing` and `🏛️ownership/🧮️compute`, `📕️norm`, `📸️remodel` and the single-plugin test-tidy manifests), `🧪️tests/📺️renderer/🧊️wgpu`; `♻️mit-bestand/🧺️demonstrator` and `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/…`.
