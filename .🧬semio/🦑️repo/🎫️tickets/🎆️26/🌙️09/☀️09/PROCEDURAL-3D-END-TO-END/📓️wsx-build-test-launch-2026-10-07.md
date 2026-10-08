# 🧭️ Procedural Plugin and Flow B-Rep Extension: Build, Test and Launch Map

Read-only audit, 2026-10-07. No commands were executed for this document; every command below is taken from a `📜️script.ts`, `📋️project.json`, `launch.json` entry or a memory note, and should be treated as unverified until an execution agent runs it.

## 1. Names and fastest commands

### Project and crate names

| Thing | Cargo package | nx project | Source |
|---|---|---|---|
| Procedural generation3d artifact (Rust) | `semio-s-artifact-procedural-generation3d` | `@semio-tech/procedural-generation3d-rs` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml`, `📋️project.json` |
| Procedural generation3d artifact (TS) | n/a | `@semio-tech/procedural-generation3d` | `…/🧊️generation3d/📦️packages/🟦️typescript/📋️project.json` |
| Procedural generation2d artifact (Rust) | `semio-s-artifact-procedural-generation2d` | `@semio-tech/procedural-generation2d-rs` | `…/🌀️generation2d/📦️packages/🦀️rust/📋️project.json` |
| Procedural generation2d artifact (TS) | n/a | `@semio-tech/procedural-generation2d` | `…/🌀️generation2d/📦️packages/🟦️typescript/📋️project.json` |
| Procedural plugin TS test harness | n/a | `@semio-tech/procedural-js` | `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📋️project.json` |
| Procedural mutation bridge | `semio-procedural-mutation-bridge` (own `[workspace]`) | none found | `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.toml` |
| Flow B-Rep extension (Rust) | `semio-s-plugin-flow-extension-brep` | `@semio-tech/flow-extension-brep-rust` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml`, `📋️project.json` |
| Framework 3D kernel (B-Rep, collision, mesh modules) | `semio-framework-3d` | `semio-framework-3d` (used by launch.json) | `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` |
| Framework 3D kernel (TS) | n/a | `@semio-tech/s-3d-js` | `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/package.json` |
| Mesh engine | `semio-framework-mesh-engine` | not confirmed (no `📋️project.json` in the folder; likely Cargo-inferred) | `🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/Cargo.toml` |
| OS kernel (used by generation3d) | `semio-framework-os-kernel` | n/a | referenced by generation3d `Cargo.toml` |

Note: `📦️packages/🦀️rust` under the procedural plugin itself has no `Cargo.toml`, only a `dist/` folder. The procedural Rust crates live under the artifact folders.

### Fastest commands

All commands run from the repo root. Prefer nx, since that is what launch.json and the `📜️script.ts` routers use. Add `--skip-nx-cache --excludeTaskDependencies` when you want an isolated run, as launch.json does.

- **cargo check one crate (nx, authoritative):** `bun nx run @semio-tech/procedural-generation3d-rs:check --skip-nx-cache`. The target runs `bun ./📜️script.ts check`, which calls `cargo check --locked --manifest-path <crate>/Cargo.toml` plus `artifactRustCargoArguments`. For generation2d, use `@semio-tech/procedural-generation2d-rs:check`. For the B-Rep extension, the `check` target is not declared, so use `cargo check --locked -p semio-s-plugin-flow-extension-brep`. For the framework 3D kernel, use `cargo check --locked -p semio-framework-3d`.
- **Rust unit tests for one crate:** `bun nx run @semio-tech/procedural-generation3d-rs:test --skip-nx-cache -- quick --offline --lib <filter>`. The level arguments are `fundamental` (15 s budget), `quick` (300 s), `long` (900 s), `exhaustive` (1800 s). The `test` target uses `runArtifactRustTests` with `testFeatures: ["component-app-assembly"]`, so feature-gated tests are included automatically. Launch.json shows the pattern `--args='long --offline --lib -E "test(…)"'`. The B-Rep extension equivalent is `bun nx run @semio-tech/flow-extension-brep-rust:test --skip-nx-cache --args='quick --offline --lib <filter>'`, and the quick/long/exhaustive variants also exist as `test-quick`, `test-long`, `test-exhaustive` targets.
- **Language-agnostic tests (TypeScript runner):** `bun nx run @semio-tech/procedural-js:test`. This runs `bun ./📜️script.ts test`, which calls `bun test` over the fixed case list in `📦️packages/🟦️typescript/📜️script.ts`. Use `bun nx run @semio-tech/procedural-js:test-renderer-contract` for the renderer subset. The artifact-level TS suite is `bun nx run @semio-tech/procedural-generation3d:test --skip-nx-cache`, which uses `runArtifactTypeScriptPackageMain` and the sqlite snapshot suite.
- **Language-agnostic tests (Rust runner):** `bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-widget-inputs`, `:verify-generation3d-document-io`, `:verify-generation3d-preview-window-transient`, `:semantic-wire-check` (add `native` in its args for the cargo half), and `:verify-document-restoration-oracle`. Each one runs the matching TS fixture through `bun test` and `tsc --noEmit --strict`, then the Rust half where it exists.
- **Schema codegen:** no codegen target or script was found for the procedural plugin or the B-Rep extension. Schemas (`🔣️.json`) are hand-authored and checked by independent validation (ajv 8.20.0 is in the root `package.json`; the widget-input self-test prints `independentAjv=true`). Do not look for a regen command. If one is needed, it is not in the repo yet.
- **Restage the guest wasm (generation3d React playground):** `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`. Takes roughly 5 minutes warm. Output goes to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/`. Source: memory note `project-generation3d-restage-and-probes.md`.
- **Serve the react playground:** `bun nx run @semio-tech/framework-os-dev:serve-generation3d-react-dev --skip-nx-cache` (launch.json entry `🛠️serve🌀️procedural🧊️generation3d⚛️react`), port 6018. The serve never rebuilds the guest wasm. Vite picks up new bytes without restart. Viewer role: `?role=viewer` on 6018, or port 6019 via the ticket's `📜️serve-generation3d-viewer.sh`. Memory notes say the ticket folder contains `📜️serve-generation3d-react.sh` under `screen -dmS g3dreact`. I did not confirm that file exists because the listing was truncated.
- **Dev boot (via launch.json, no CLI):** `bun nx run workspace:dev -- generation3d` (launch.json entry `🛠️dev🌀️procedural🧊️generation3d⚛️react`).

### Test-level budgets

From `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts`: `fundamental` 15 s, `quick` 300 s, `long` 900 s, `exhaustive` 1800 s. `SEMIO_TEST_LEVEL` carries the level into child processes. `exhaustive` also sets `SEMIO_COVERAGE=1`.

## 2. How language-agnostic tests are structured

Pattern: a standard lives in `🏅️standards/🔖️1/🪆️subsets/✳️any/`. Each law is a JSON fixture (`🔣️.json`, the schema for that folder, or `🧫️fixtures/*.json`). Rust and TypeScript each have their own runner under `🧪️tests/` that reads the same fixture and asserts the same outcome. Rust tests are `🦀️.rs` files; TypeScript tests are `🟦️.ts` files, run by `bun test`.

Each mutation has a `🔣️.json` with `schemaVersion`, `semanticKind`, `binaryTag`, `invertibility`, `outcomeClasses`, and `requiredLanguageSurfaces: ["rust", "json-schema", "text", "binary"]`. Those fields tell you which surfaces must agree.

### Example A: generation3d mutation

- Metadata: `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-synapse/🔣️.json`
- Payload schema: `…/🧬️mutations/🔗️connect-synapse/🧬️schema/…` via `payloadSchema: "🧬️schema/🔣️.json"`
- Rust tests: `…/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`, `…/🧬️mutations/🧪️tests/🧪️gesture-leaves/🦀️.rs`
- TypeScript tests: `…/🧬️mutations/🧪️tests/📄️document-restoration/🟦️.ts`, run by the Rust-side `verify-document-restoration-oracle`
- Also referenced by `…/🧬️mutations/🧫️fixtures/` (fixtures per mutation)

### Example B: generation3d inference

- Schema: `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json`. It defines the `Generation3dInference` with a `topology` object (nodeCount, edgeCount, topoOrder, depth, cycleFree).
- Subfolder: `…/💡️inferences/🧭topology/` with `🟦️.ts` and `🦀️.rs`.
- Rust unit test: `…/💡️inferences/🧪️tests/🔬️unit/🦀️.rs`

### Example C: independent oracle fixture (not a mutation)

- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🔬️p8yz-b-third-party-oracle-laws.json`. Its `oracle.library` is `serde_json` (test-only). It describes a "move one mounted generation3d widget" scenario with explicit `expected` values.
- `…/🔮️oracles/🔣️.json` registers the cross-language oracle. The id `procedural-3d-python-independent` (ecosystem python) points to `🧪️tests/mutate-procedural-3d-1/🐍️component.py`. There is also a rust entry for `parry3d` (package `parry3d`, scipy/numpy for the python oracle).

Fixture directories used across both packages: `🧫️fixtures/`, `🧪️tests/`, `🔮️oracles/`, `🧬️schema/`, `🧵️preview-eval/`, `🚪️io/`, `👁️viewer/`, `✏️editor/`. Each carries its own `🔣️.json` index.

## 3. Third-party oracle libraries

| Library | Where | Role |
|---|---|---|
| `three` (npm) | `🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️unit/🟦️.ts`; `🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js/🟦️.ts`; procedural `…/✏️editor/🎚️set-widget-input/🧪️tests/🔬️unit/🟦️.ts`, `…/✏️editor/🎯️selection/🧪️tests/🔬️unit/🟦️.ts`, `…/🧬️schema/🧭️transforms/🧪️tests/🔬️unit/🟦️.ts`, `…/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts`; brep `🥽️mesh/🧪️tests/🔬️unit/🟦️.ts`, `🧪️tests/🔬️extension-guest-standalone/🟦️.ts`. Used for Box/Plane/Vector geometry and the three.js Vector comparisons. Not a runtime dependency. |
| `manifold-3d` 3.5.1 (root `package.json`) | `🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️unit/🟦️.ts` (boolean differential oracle) | Test-only. |
| `@gltf-transform/core` | `🧊️3d/📦️packages/🟦️typescript/📜️script.ts` area, glTF import/export round-trip | Test and interchange oracle. |
| `parry3d` 0.17 | `🧊️3d` rust `[dev-dependencies]`; `brep` rust `[dev-dependencies]` | Test-only differential oracle for BVH collision (`🧿️collision`). |
| `serde_json` | `🧊️3d` rust dev-dep, mesh-engine dev-dep, generation3d dev-dep | Test-only oracle proving the first-party `ToValue`/`FromValue` codec matches serde's bytes. |
| `gltf` 1.4.1 | mesh-engine dev-dep | `🔬️gltf-oracle-differential` tests. |
| `@semio-tech/geometry-brep-js` | `🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js/🟦️.ts` | Name suggests brepjs-based comparison; I did not confirm the underlying library. |
| `ajv` 8.20.0 | root `package.json` | Independent JSON-schema validation (`independentAjv=true`). |
| `vitest` ^4.0.17 | root and 3d package | TS test runner for `bun test` cases (some cases import vitest). |

The root `package.json` does not list `brepjs` or OpenCASCADE. The brep oracle is OCCT-free as far as I can see. The memory note `project-brepjs-is-production-in-cad.md` says brepjs is production in CAD and not an independent oracle there. Read that note before trusting any "oracle" claim.

Mesh oracle script: the brep extension's `test-mesh-oracle` target runs `bun test ../../🥽️mesh/🧪️tests/🔬️unit/🟦️.ts`.

## 4. launch.json conventions for procedural

The conventions in `.vscode/launch.json` (the seed `.vscode/🧩️launch.seed.jsonc` mirrors it) use a name built from emoji-prefixed segments:

- Verb: `🛠️dev` (dev server), `🛠️serve` (serve only), `⚖️gate` (test gate), `⚖️verify-…`, `⚖️test-…`, `🧿️semio…`
- Family: `🌀️procedural`
- Artifact: `🧊️generation3d`, `🌀️generation2d`, or `📐️brep` for the B-Rep extension
- Runtime suffix: `⚛️react`, `🧊️wgpu🌐️wasm`, `🧊️wgpu🖥️native`, `🖥️native`
- Language suffix on gates: `🟦️` for TypeScript, `🦀️` for Rust

Existing entries verbatim (exact names as they appear in `.vscode/launch.json`):

- `🛠️dev🌀️procedural🌀️generation2d⚛️react`
- `🛠️dev🌀️procedural🌀️generation2d🧊️wgpu🌐️wasm`
- `🛠️dev🌀️procedural🧊️generation3d⚛️react`
- `🛠️dev🌀️procedural🧊️generation3d🧊️wgpu🌐️wasm`
- `🛠️dev🌀️procedural🌀️generation2d🧊️wgpu🖥️native`
- `🛠️dev🌀️procedural🧊️generation3d🧊️wgpu🖥️native`
- `🛠️serve🌀️procedural🧊️generation3d⚛️react`
- `⚖️gate🧊️generation3d🟦️public`
- `⚖️gate🌀️generation2d🟦️public`
- `⚖️gate🌀️procedural◻️2d🪶️sqlite🦀️native`
- `⚖️gate🌀️procedural🧊️3d🪶️sqlite🦀️native`
- `⚖️gate🏢️semio-tech🎡️play🧊️generation3d-semantic`
- `⚖️gate🏢️semio-tech🎡️play🧊️generation3d-io`
- `⚖️verify-generation2d-window-camera-ownership🟦️`
- `⚖️verify-generation3d-document-io🟦️`
- `⚖️verify-generation3d-preview-window-transient🟦️`
- `⚖️test-intrinsic-number-text🌀️generation2d🦀️`
- `⚖️test-topology-widgets📐️brep🦀️`
- `⚖️test-component-picking📐️brep🦀️` and `⚖️test-component-picking📐️brep🟦️`
- `⚖️test-component-retirement📐️brep🦀️`
- `⚖️test-component-label-selection📐️brep🦀️`
- `⚖️test-measurements📐️brep🦀️`
- `⚖️test-closest-point📐️brep🦀️`
- `⚖️test-selected-controls📐️brep🦀️`
- `⚖️test-live-selected-solid📐️brep🦀️`
- `⚖️test-widget-controls-app🧊️generation3d🦀️`
- `🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react` (and `…🧊️wgpu🌐️wasm`, `🏙️3d👁️viewer⚛️react`, `🏙️3d👁️viewer🧊️wgpu🌐️wasm`)

Drift to note: there are two families for procedural dev entries, `🛠️dev🌀️procedural🧊️generation3d…` and `🛠️dev🔧️procedural🏙️3d…` (the latter runs `workspace:dev -- procedural 3d`). Follow the `🌀️procedural` family for new entries unless the user says otherwise. Also note that `launch.json` has uncommitted changes (git status shows `MM .vscode/launch.json`).

Mirror rule: AGENTS.md requires that every executable command be registered in `launch.json`, in the existing order and grouping. Keep the seed file in step if the seed is the template.

## 5. Known pitfalls

From the memory notes and the ticket status log (`📓️status.md` last 40 lines, 2026-09-15 and the 2026-10-07 session 6 entry):

1. **Build time and wall-clock budget.** Every cargo spawn is capped at `BUILD_BUDGET_MS = 1_200_000` (20 min) in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`. A dev or demonstrator build can exit with only the banner in its log. Memory: `project-semio-build-budget-and-oom.md`.
2. **Restage is ~5 min warm, and the serve does not rebuild wasm.** After any guest Rust change, restage with `activate-generation3d-react-dev` before probing. A stale vite transform can also look like a bug. Touch the file or restart the serve after a restage. Memory: `project-generation3d-restage-and-probes.md`, `feedback-vite-stale-transform-verify-served-module.md`.
3. **fs.watch misses atomic saves on macOS.** Editor saves and `sed -i` invalidate nothing unless the writer uses an in-place write. `touch` works. Ticket note `vite-stale-transform-guard-2026-09-15.md`.
4. **Shared cargo build-dir with fine-grain locking.** Since 2026-09-11 all cargo runs share `.🧬semio/🦑️repo/⚡️cache/cargo/build` with `-Zfine-grain-locking`. Never create a private build-dir. Memory: `project-shared-cargo-build-dir-fine-grain-locking.md`.
5. **Deadlock in `prebuild_lock_exclusive`.** Concurrent wasm-dev builds can sit at 0% CPU for 40–100 min. Diagnose with `sample <pid>`; break it by killing the whole stuck set (not only the wrapper). Memory: `feedback-fine-grain-locking-wasm-deadlock.md`, `feedback-killing-wrapper-orphans-cargo.md`.
6. **cargo test starvation on the uplift lock.** Under fleet load `cargo test` waits on `flock` for 20+ min. Workaround from memory: set `CARGO_TARGET_DIR=<repo>/.🧬semio/🦑️repo/⚡️cache/cargo/target-<slice>` for test runs while leaving `build.build-dir` shared. Memory: `feedback-private-target-dir-ends-cargo-test-starvation.md`.
7. **Disk growth.** Private target `debug/incremental` regrows 10–23 GB per hour under cargo-test waves and can cause ENOSPC mid-link for release builds. Memory: `feedback-incremental-cache-regrows-under-fleet.md`.
8. **OOM and swap under a large agent fleet.** Long Rust builds (for example `semio-s-plugin-stdio`, 30–60 min) die silently under load 20–38 with swap near exhaustion. Memory: `feedback-max-parallel-fleet-starves-builds.md`, `project-watchdog-kernel-panics-under-fleet-load.md`. Use at most a few concurrent build agents.
9. **Native cargo misses wasm-gated code.** Code behind `cfg(wasm32)` never compiles on native; check it with the wasm target. Memory: `feedback-native-cargo-misses-wasm-gated-code.md`.
10. **Feature-gated tests.** The generation3d tests behind `component-app-assembly` only compile with `--features component-app-assembly`. The test runner passes it automatically; ad-hoc `cargo test` does not. Memory and `runArtifactRustTests` docstring.
11. **Nx graph rebuild wait.** "Waiting for graph construction" can take 3.5–6 min per nx run. Setting `NX_FORCE_REUSE_CACHED_GRAPH=true` drops it to about 24 s. Memory: `project-nx-graph-construction-wait-and-cached-graph-reuse.md`.
12. **Nx hasher is blind to gitignored inputs.** Do not rely on nx cache for generated inputs. Use `--skip-nx-cache` when in doubt. Memory: `project-nx-hasher-blind-to-gitignored-inputs.md`.
13. **Peer churn breaks restages.** A peer's half-landed rename (duplicate `artifact_schema`, unresolved `snapshot`/`fixture`) makes the restage fail. Ticket log 2026-09-15 shows this resolved by a "fix forward" lane. Check `cargo check` on the touched crates first.
14. **Stale TS test path.** `📦️packages/🟦️typescript/📜️script.ts` references `…/📚️examples/🍄️hexagonal-mushroom/🧪️tests/🧩️example/🟦️.ts`, but the folder on disk is `🍄️hexagonal-mushroom-column`. The TS test router will fail on that entry. Fix the path before relying on `procedural-js:test`.
15. **Hidden browser pane throttles boot.** A 0×0 hidden pane slows plugin boot. Memory: `project-hidden-browser-pane-throttles-plugin-boot.md`.
16. **macOS binary overwrite.** Copying a new binary over an existing one gets it SIGKILLed silently. Use `rm`, `cp`, then `codesign -s - -f`. Memory: `project-macos-inplace-binary-overwrite-sigkill.md`.
17. **macOS has no `timeout` command.** Do not wrap cargo in `timeout`. Memory: `feedback-macos-has-no-timeout-command.md`.
18. **Tickets with long serves.** Serves started by subagents die when the parent turn ends. Start serves from the main session with `nohup` or `screen`. Memory: `feedback-start-servers-from-main-session.md`, `feedback-preview-start-servers-vanish-use-nohup.md`.
19. **Wedged vite serve.** `curl --max-time 5` returns 000 and `ps -o pcpu` above 70% means recycle by pid. Memory: `project-release-serve-wedges-after-host-edit-bursts.md`.
20. **Cargo test filenames must be ASCII.** Declare `[[test]] name` and `path` explicitly for emoji-named test files. Memory: `project-cargo-test-filenames-must-be-ascii.md`.

## 6. Open items

- `🧪️tests` at the procedural plugin root and `🧫️fixtures` at the root were listed but not read.
- No schema codegen target was found. Confirm with the user before adding one.
- Whether `semio-framework-mesh-engine` has an nx project name of the same string was not confirmed.
- Ticket serve scripts (`📜️serve-generation3d-react.sh`, `📜️serve-generation3d-viewer.sh`) were referenced by memory but not verified on disk.
- No command in this document was executed.
