# Explore: development infrastructure for non-destructive history editing

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit of the practical dev infrastructure (crates, TS packages, fixtures, launch.json, commands, taxonomy). No source file edited. Written 2026-09-30 ~02:40.

Sibling reports (same ticket): `📓️explore-event-sourcing-core.md`, `📓️explore-puzzle2d.md`, `📓️explore-tools-transactions.md`, `📓️explore-history-ui.md`, `📓️explore-alternatives-vcs.md`.

## 0. Method and what I actually ran

- Read every cited file/region directly. Paths are repo-relative to `/Users/ueli/Documents/semio`. Aliases used below:
  - `FW` = `🧰️framework/🔨️modules`
  - `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`
  - `PZ` = `✏️s/🔌️plugins/🧩️puzzle`
  - `PZ2D` = `PZ/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` (the puzzle 2d subset owner)
  - `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`
  - `TESTDOM` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`
- Ran (all read-only):
  - `cargo metadata --no-deps --format-version 1 --offline` (0.46 s) -> 275 workspace packages; table in section 1 is derived from it.
  - `bun ./📜️script.ts verify taxonomy report --scope <dir>` on 7 module scopes (results in section 6). 5.7 s for `⏯️tool-run`; 2 min 09 s and a crash for the puzzle 2d artifact scope.
  - `jq` over `.nx/workspace-data/project-graph.json` (fresh, 2026-09-30 02:05) for exact nx project names/targets. This is the fastest way to answer "what is the nx project called": `jq -r '.nodes | keys[]' .nx/workspace-data/project-graph.json | /usr/bin/grep <word>`.
- Did NOT run any cargo build/test, wasm build, dev serve or browser session (fleet rule 9, machine load 17-29, swap 3.9 of 5.1 GB used). Every "fast command" in section 5 is therefore statically derived from `📜️script.ts`/`project.json`/`Cargo.toml`; the one command a peer report proves by execution is marked (peer-verified).
- Machine state at audit time: nothing listens on 6012/6112/6070/6071/6033/6010; `dist/runtime/react/dev/` has NO `puzzle2d` directory (puzzle3d and puzzle5d are there), so puzzle 2d is not activated on this checkout.

## 1. Rust workspace layout

### 1.1 The mounting model (read this before creating any Rust file)

- Root `Cargo.toml` has an explicit `[workspace] members = [...]` list (822 lines, heavily contended shared file) and a `[workspace.dependencies]` table that maps each crate name to a path. 275 packages are workspace members. Plugin/artifact crates are members too (`semio-s-*`, 174 of them).
- Crates are thin **package glue**. The domain code lives in owner files `<owner>/🦀️.rs` and is mounted into a crate with `#[path = "../../🦀️.rs"] mod component; pub use component::*;`. The crate entry is always `<owner>/📦️packages/🦀️rust/🦀️.rs` next to `Cargo.toml`, `📋️project.json`, `📜️script.ts` (and sometimes `package.json`).
- Many owner directories have NO crate of their own; they are `#[path]`-mounted by a carrier crate. There is no module auto-discovery: **every new `.rs` file (including test files) must be mounted by a `#[path = "..."]` line in its parent `🦀️.rs`**, or it is never compiled. `#[path]` resolution is cumulative (`taxonomy.rustEntryPathRules`: `leaf-prefixed` and `once-reset` conventions, `#[path = "."]` keeps the enclosing base). A file that no crate mounts reads as green everywhere.
- Consequence for verification: a crate that dies at module expansion (`couldn't read ...`) reports zero type errors. Require a type-dependent lint or a real error as proof of type-check (memory: require-warnings-as-proof-of-typecheck).
- Line budget for glue: `taxonomy.libWiringLineBudget = 150`; glue may only be declaration/registration/bootstrap/thin-delegation (`packageGlueGrammar`, max 32 delegation statements for Rust/TS).

### 1.2 Framework modules (`FW/*`) -> crate

| Directory | Crate (Cargo `name`) | Kind | Notes |
|---|---|---|---|
| `📡️replication` | `semio-framework-replication` | lib | **`[lib] name = "protocol"`**. Owns `🎮️mutation`, `🔗️causal` (+ `🔀️transition`), `🧾️wire`, `📡️wire`, `🚰️source`, `🚧️apply-refusal` (new, peer, staged), `🌱️value`, `⚠️diagnostic`. Crate glue `📡️replication/📦️packages/🦀️rust/🦀️.rs`. Also a TS twin `📡️replication/🟦️.ts` (1854 lines) |
| `🔄️machine` | `semio-framework-machine` (+ `-derive` proc-macro) | cdylib+rlib | statechart kernel, actors, hosts; wasm-bindgen `WasmHost` only on `all(wasm32, not(p2))`. TS twin `🔄️machine/🟦️.ts` (package `@semio-tech/machine`) |
| `⏯️tool-run` | `semio-framework-tool-run` | lib (`lib name semio_framework_tool_run`) | pure/target-neutral; compiles native, `wasm32-unknown-unknown`, `wasm32-wasip2`. TS twin `⏯️tool-run/🟦️.ts` (61 KB). **Best template for a new framework module** (section 6.4) |
| `🖱️ui` | `semio-framework-ui` (+ `-contract`, `-scene`, `-runtime`, `-viewport`, `-styling`, `-render`, `-host`, 4 GPU backends) | lib | sub-crates under `🖱️ui/<facet>/📦️packages/🦀️rust`; `-contract` is `lib,test`. Elements (`🧱️elements/🎚️Slider`, `🪜️Stepper`, `🕰️HistoryTable`, `🔘️Button`...) are TSX components with `🎯️targets` and `📖️stories` |
| `🧬️schema` | `semio-framework-schema` (+ `-registry`, `-derive`) | lib (+test) | `🧬️schema/📦️packages/🦀️rust`; also TS (`🧬️schema/🟦️.ts`) and `🔣️.json` |
| `🌱️value` | `semio-framework-value-derive` (proc-macro+test), `semio-framework-value-resident` | | The value model `🌱️value/🦀️.rs` has NO crate: it is mounted by replication as `protocol::value`. `ToValue/FromValue` derives come from `value-derive` |
| `🕹️interaction` | (none) | mounted | mounted by `semio-framework` (`🧰️framework/📦️packages/🦀️rust/🦀️.rs`, `pub mod interaction`); pure hover/selection state machine actually lives in replication `📡️wire/🦀️.rs` `Interaction` region |
| `✍️editor` | `semio-framework-editor` | cdylib+rlib | canvas-agnostic editor state machine (selection, transforms, undo); wasm-pack target `wasm`; depends on `os-kernel` |
| `🛂️manifest`, `🎯️action-bus`, `🌉️abi`, `🖥️platform`, `🚪️io` | (none) | mounted | all mounted into **`semio-framework`** (`🧰️framework/📦️packages/🦀️rust`, lib) via `#[path]` |
| `🎠️kernel` | (none) | mounted | mounted by `🛂️manifest/🦀️.rs:5348`; TS package `@semio-tech/framework-kernel` |
| `🧵️job`, `⏳️async`, `⏱️trace`, `🎭️actor`, `🎒️pack`, `🔏️hash`, `🗜️deflate` | `semio-framework-job`, `-async`(+macros), `-trace`, `-actor`(cdylib), `-pack`, `-hash`, `-deflate` | lib | supporting crates |
| `🔀️dispatch` | `semio-framework-dispatch-macros` | proc-macro+test | `dyn_enum_close!` etc.; tests are `[[test]]` files |
| `📐️geometry`, `🧮️math`, `🕸️graph`, `🧊️3d`, `◻️2d`, `🗺️surface`, `🖌️raster`, ... | `semio-framework-geometry` ... | | not relevant to history editing |

### 1.3 OS modules (`OS/*`) -> crate

| Directory | Crate | Kind | Notes |
|---|---|---|---|
| `🏪️store` | mounted in **`semio-framework-os-kernel`** (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`, lines ~279-303 of its `🦀️.rs`; `store` alias) | bin,cdylib,rlib,test | 23.6k-line `🏪️store/🦀️.rs`: artifact store, sync (`🔄️sync`), worker (`👷️worker`), canonical-edit, presence, ephemeral |
| `🌿️vcs` | mounted in `semio-framework-os-kernel` (line ~218, `vcs` alias) | | 1288 lines, group-history visibility |
| `🌐️locale` | mounted in `semio-framework-os-kernel` (line ~213) | | `LocalizedLabel::native(en, de)` lives here; i18n |
| `📡️spr` (`📜️history`, `🎮️command`) | mounted in `semio-framework-os-kernel` (`os_spr`) | | mutation command family, HLC history fold |
| `🗣️dsl`, `🎒️pack`, `🪪️identity`, `📇️directory`, `🧬️semio`, `🧩️extension`, `💡️inference`, `⚙️engine` | mounted in `semio-framework-os-kernel` | | `dsl` derives (`MutationLeaf`, `Mutations`) come from `semio-framework-os-kernel-dsl-derive` (`🗣️dsl/✨️derive`) |
| `🔌️plugin` | **`semio-framework-plugin`** (`🔌️plugin/📦️packages/🦀️rust`, lib; 45k-line `🔌️plugin/🦀️.rs`), `semio-framework-plugin-host` (`🖥️host`, bin+lib), `semio-framework-plugin-describe` (`🖨️describe`, bin+lib) | lib | features `component-guest`, `component-guest-async`, `component-extension-guest`, `artifact-app-testing`. `⏯️tool-run/🦀️.rs` (os-side tool-run runtime, mounted at `🔌️plugin/🦀️.rs:7378`) and `🕹️interaction` live here |
| `🖥️shell` | **`semio-framework-os-shell`** (`🖥️shell/📦️packages/🦀️rust`) | cdylib+rlib | small (379-line `🖥️shell/🦀️.rs`); TS package `@semio-tech/framework-os-shell` (`🖥️shell/📦️packages/🟦️typescript`) |
| `📺️renderer` | **`semio-framework-os-renderer-wgpu`** (`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`) | bin,cdylib,rlib,custom-build | Rust wgpu shell. The React host is TS: `@semio-tech/framework-renderer-react` at `📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript` (entry `🟦️.tsx`, elements in `📺️renderer/🧑‍🎨engine/🧱️elements/*`, e.g. `🖥️Board2dHost`, `🌳️GraphTimelineHost`) |
| `🖥️host` (sibling of `🔨️modules`, `💻️os/🖥️host`) | `semio-framework-os` | cdylib+rlib | wasm host bridge |

Other os crates: `semio-framework-os-mcp` (`🌉️mcp`, bin+lib+test), `semio-framework-os-kernel-db` (`🛢️db`), `semio-framework-os-services`, `-os-run`, `-os-flow` (cdylib), `-os-infinite`, `-os-config`.

### 1.4 Puzzle crates

| Directory | Crate | Kind |
|---|---|---|
| `PZ/📦️packages/🦀️rust` | **`semio-s-plugin-puzzle`** (nx `@semio-tech/puzzle-plugin`) | **cdylib+rlib**, WASM component entry (`plugin-entry` default feature; `[lib] path = "../../🦀️.rs"` = `PZ/🦀️.rs`). Also wasm-pack `wasm` target (`pkg/semio_puzzle_bg.wasm`, 70 MB). `[package.metadata.component] package = "semio:puzzle"`. Playground rows: `puzzle2d` react 6012 / wgpu 6112, `puzzle3d` 6013/6113, `puzzle5d` 6014/6114 |
| `PZ/🗿️artifacts/◻️2d/📦️packages/🦀️rust` | **`semio-s-artifact-puzzle-2d`** (nx `@semio-tech/puzzle-2d-rs`) | lib. `[lib] path = "../../🦀️.rs"` (122 KB owner file). **The whole `editor`/`viewer`/`examples` trees and `standards::v1::...::component` are `#[cfg(feature = "component-app-assembly")]`** (12 gates in `◻️2d/🦀️.rs`, lines 19-23, 597-661, 743-757, 1553, 1571, 1818). The `schema` subtree (snapshot, diff, mutations, inferences) is NOT gated |
| `PZ/🗿️artifacts/🧊️3d/...`, `🖐️5d/...` | `semio-s-artifact-puzzle-3d`, `-5d` | lib |
| `PZ/🏭️bridge` | `semio-puzzle-mutation-bridge` | bin, **standalone `[workspace]`** (not a root member; use `--manifest-path`). Production `list-mutations` bridge the test domain's `inventory` phase calls |
| `PZ/🎯️targets/⚛️5d-react`, `PZ/📦️packages/🟦️typescript` | `@semio-tech/puzzle-5d-react`, `@semio-tech/puzzle-js` | TS |

The puzzle 2d mutation vocabulary is 33 leaf directories under `PZ2D/🧬️schema/🧬️mutations/<emoji><verb>-<noun>/` (each with `🦀️.rs`, `🔣️.json`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🧪️tests/<emoji><case>/🦀️.rs`, `🧬️schema/🔣️.json`). The aggregate is `Puzzle2dMutation` in `🧬️mutations/🦀️.rs` (575 lines) with a TS twin `🧬️mutations/🟦️.ts` (268 lines) and binary/text codecs under `💾️binary`/`📝️text`.

### 1.5 cdylib / wasm summary

- **wasm32-wasip2 plugin components** (`cdylib`): 71 of the 74 `semio-s-plugin-*` crates (the exceptions are `draw-fsm-macros` proc-macro, `wfc-engine` lib, `trinity-jack-shell` bin; puzzle = `semio-s-plugin-puzzle`, the multi-app aggregator = `semio-s-plugin-demonstrator`). Built with `cargo rustc -p <crate> --target wasm32-wasip2 --profile wasm-dev` (stack size now set in `.cargo/config.toml [target.wasm32-wasip2]`), then jco-transpiled into `🧑‍💻dev/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/`.
- **wasm-pack (browser, `wasm32-unknown-unknown`) cdylib**: `semio-framework-editor`, `-machine`, `-surface`, `-actor`, `-math`, `-os`, `-os-flow`, `-os-kernel`, `-os-shell`, `-os-renderer-wgpu`, `-os-scale-fixture`, and `semio-s-plugin-puzzle` (via its `wasm` target).
- Plugin crates depend on `semio-framework-os` for `cfg(not(wasm32))`, so a native `cargo check -p semio-s-plugin-x` compiles the whole plugin host. Prefer `--target wasm32-wasip2` (memory: plugin-native-check-pulls-broken-host). Native cargo never compiles `#[cfg(target_arch = "wasm32")]` code.
- `rust-toolchain.toml`: `nightly-2026-07-07` with targets `wasm32-unknown-unknown`, `wasm32-wasip2`; cargo 1.99 nightly (fine-grain-locking, build-dir-new-layout, checksum-freshness).

## 2. TypeScript packages, tests, React host, Storybook

### 2.1 Layout

- Bun workspaces (root `package.json`, 118 entries, `bunfig.toml` `linker = "hoisted"`). Each TS package is `<owner>/📦️packages/🟦️typescript/` holding `package.json`, `📋️project.json`, `📜️script.ts` and a glue `🟦️.ts`/`🟦️.tsx` that only re-exports the owner leaf: `export * from "../../🟦️.ts";`. The domain TS sits in the owner leaf `<owner>/🟦️.ts` (`.tsx` for React) and is the **TS twin** of `<owner>/🦀️.rs`. TS `strict`, `moduleResolution: bundler`, `allowImportingTsExtensions` (imports name `.ts`/`.tsx` explicitly).
- Package names for the relevant modules:
  - `@semio-tech/framework-replication` (`FW/📡️replication/📦️packages/🟦️typescript`), Rust project `@semio-tech/framework-replication-rs`
  - `@semio-tech/machine` (`FW/🔄️machine/📦️packages/🟦️typescript`), Rust `@semio-tech/framework-machine`
  - Rust `@semio-tech/framework-tool-run-rs`; TS twin has no package of its own (tested via bun test in the Rust package script, imported by os TS)
  - `@semio-tech/framework-kernel` (`FW/🎠️kernel/...`), `@semio-tech/framework` (`🧰️framework/📦️packages/🟦️typescript`), `@semio-tech/framework-os` (`💻️os/📦️packages/🟦️typescript`)
  - `@semio-tech/framework-os-shell` (shell TS), `@semio-tech/framework-plugin-web` (`OS/🔌️plugin/📦️packages/🟦️typescript`, targets `support-dev|release` only), `@semio-tech/plugin-registry` (`OS/🔌️plugin/📇️registry`)
  - `@semio-tech/framework-renderer-react` (React host), `@semio-tech/ui-react` (`FW/🖱️ui/🎯️targets/⚛️react/...`), `@semio-tech/ui-styling`, `@semio-tech/framework-os-dev` (dev host, `OS/🧑‍💻dev/📦️packages/🟦️typescript`)
  - `@semio-tech/puzzle-js` (`PZ/📦️packages/🟦️typescript`), `@semio-tech/puzzle-5d-react`

### 2.2 How they are tested

Two runners coexist:

1. **Vitest 4** (44 package scripts): `📜️script.ts` `TestScript` calls `resolveTestLevel(segments)` then `runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts")`. The config lives in the OWNER's `🧪️tests/🎚️config/🟦️.ts`. `runVitest` (`LIB/🟦️.ts:2618`) runs the workspace's own `node_modules/vitest/vitest.mjs` under bun (node when coverage), with `--config` made absolute, under a wall-clock budget, with the test level exported as `SEMIO_TEST_LEVEL`.
   - **In-source tests**: 109 owner leaves use `if (import.meta.vitest) { ... }` and register test files by `await import("./🧪️tests/<case>/🟦️.ts")` then `registerTestsN(import.meta.vitest, ...)`. Example: `FW/📡️replication/🟦️.ts:1821-1829` registers four cases. `includeSource: ["*.ts"]` is a non-recursive glob on the module root, so a NEW replication test must be registered from that block (or listed in `include`).
   - **Explicit include lists are a silent-skip trap**: `@semio-tech/framework-renderer-react`'s config (`OS/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`) names every suite through `engineSuite("<dir>")`/`elementSuite(...)`/`uiSuite(...)`; a suite not named there never runs while the gate reads green (its own comment says so). The replication config only `include`s `👕️peer-overlay/🧪️tests/**/*.ts` plus the in-source glob.
   - Level gating in tests: `atTestLevel(it, "long")`, `testLevelAtLeast`, `testLevelRank` (`LIB/🟦️.ts`). Levels `fundamental (15 s) < quick (300 s) < long (900 s) < exhaustive (1800 s)`; `SEMIO_TEST_BUDGET_MS` overrides.
2. **`bun:test`** (294 test files import `bun:test`; 6 package scripts run `bun test <file>` through `runTestBudgeted`). Example: `⏯️tool-run` runs `bun test FW/⏯️tool-run/🧪️tests/🧩️conformance/🟦️.ts` from `semio-framework-tool-run`'s Rust package script, then `runCargoTestBudgeted`.

Direct invocations: `cd <pkg>/📦️packages/🟦️typescript && bun ./📜️script.ts test [quick|long|exhaustive] [vitest args]`, or `bun nx run <project>:test|test-quick|test-long|test-exhaustive`.

### 2.3 React host and Storybook

- React host = `@semio-tech/framework-renderer-react` (entry `🟦️.tsx`), targets `test`, `typecheck`, `lint`, and many `*-check` browser checks (`window-scope-check`, `scoped-presence-check`, `surface-switch-check`, `view-state-carriage-check`, `world3d-interaction-check`, ...). The board pane is `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx` (`data-board-*` attributes are what the browser probes read; tests in `🖥️Board2dHost/🧪️tests`).
- Existing history-like UI elements to reuse: `FW/🖱️ui/🧱️elements/🕰️HistoryTable`, `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🌳️GraphTimelineHost`, `🎚️Slider`, `🪜️Stepper` (the mutation-input UI metadata targets: slider/stepper/min/max/snap).
- Existing browser E2E harness for tool runs: `OS/🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/🟦️.ts` (Playwright; drives palette -> tool -> Start/Pause/Resume/Abort/finalize, reads History/Tasks/Tool-runs panels, asserts `↶` History rows and rail Undo). Nx `@semio-tech/framework-os-dev:tool-run-matrix` (`bun ./📜️script.ts verify tool-run`), launch row `⚖️gate⏯️tool-run⚛️react`.
- Storybook 10: root `.storybook/{main.ts,preview.tsx}`; stories are `<component>/📖️stories/🧪️.story.tsx` (file kind `story` = `🧪️` + `.story.tsx`). Scopes are hand-curated in `.storybook/📖️stories/🧭️coordination/🟦️.ts` (`STORY_SCOPES`). Puzzle 2d: `PZ/📖️stories/🎭️2d-board`, `🎭️2d-fixtures`; storybook test `PZ/🧪️tests/◻️storybook-2d/🟦️.ts`. Start: `bun nx run workspace:dev-storybook-puzzle-2d` (port `STORYBOOK_PORT=6010`, launch row `🛠️dev📖️storybook🧩️puzzle◻️2d`, order 340). `FW/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts` is a Playwright boot-health smoke over a HARD-CODED list of story ids (mounted root, zero console errors), so a new element's stories must be added to such a list to be covered by `test-storybook`. Browser runner: `.storybook/🧪️tests/🧪️browser-runner` (Playwright) via `bun nx run workspace:test-storybook`.

## 3. Language-agnostic fixtures and third-party validation

### 3.1 Where fixtures live

- `🧫️fixtures/` (exactly this name) at the OWNER root: immutable inputs shared by all languages (`FW/📡️replication/🧫️fixtures/📡️wire`, `.../🔗️causal/🧫️fixtures/🔀️history-transition-v1/🔣️.json`, `FW/⏯️tool-run/🧫️fixtures/{⚖️lifecycle-law,🎞️ticks,📼️trace-pages}.json`). Per-case private fixtures: `🧪️tests/<case>/🧫️fixtures/`.
- JSON Schemas beside them in `🧬️schema/` (`.../🔗️causal/🧬️schema/🔀️history-transition-v1/🔣️.json`, `FW/⏯️tool-run/🧬️schema/🔣️.json`).
- Test cases: `<owner>/🧪️tests/<emoji><kebab-slug>/` holding `🥒️.feature` (normative contract, tags `@capability-`, `@oracle-`, `@comparison-`; per scenario `@id-`, one `@level-`, one `@mode-`) plus one native adapter per implementation: `🦀️.rs`, `🟦️.ts`, `🐹️.go`, `🐍️.py`, `🔷️.cs` (`TESTDOM/README.md`, `taxonomy.testCaseSlugPattern = ^[a-z0-9]+(?:-[a-z0-9]+)*$`).
- Mutation quintets: `PZ2D/🧫️fixtures/🧬️mutations/<kind>/<case>/{🦠️mutation/🔣️.json, 🎯️outcome/🔣️.json, 📸️snapshot/⬅️before, 📸️snapshot/➡️after, 🔺️diff/🔣️.json}` (rejections commit `🔺️diff/🚫️.absent`). Derived encodings (`🦠️mutation/🔧️component.op.semio`, `📡️component.spr.semio`, `🔺️diff/🩹️component.patch.semio`, ...) are generated by `fixtures generate`, never hand-authored. NOTE: staged git index shows peers renaming reject-vector fixtures right now (`R` rows under `🧫️fixtures/🧬️mutations/💔disconnect-kind-compatibility/...`); the fixture location moved from `<leaf>/🧪️tests/<case>/` to `🧫️fixtures/🧬️mutations/<kind>/<case>/`.
- Oracle manifests: owner `🔮️oracles/🔣️.json` (`PZ2D/🔮️oracles/🔣️.json`: keys `mutationCatalogs, mutationManifests, noOracleDecisions, oracleHostPackages, oracles`). `TESTDOM/📇️registry/🔣️.json` `oracles` array is EMPTY: oracles are declared per owner, never centrally. `verify inventory`/`test contract` demand runtime dispatch == manifest == test catalog EXACTLY, so **every new mutation leaf (or verb) must be added to the owner manifest**.

### 3.2 Concrete example A: the history transition corpus (ticket 26/09/19 EVENT-SOURCED-FRAMEWORK-VERSION-CONTROL-END-TO-END)

- Corpus: `FW/📡️replication/🔗️causal/🧫️fixtures/🔀️history-transition-v1/🔣️.json` (`schema: semio.history.transition.v1`, `diffSchema: semio.history.transition`, 15 cases: `{id, payloadHex, expect:{outcome: accepted|malformed, transition | detail}}`). Schema: `.../🧬️schema/🔀️history-transition-v1/🔣️.json`.
- Generator (independent, non-Rust, ticket-local): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/EVENT-SOURCED-FRAMEWORK-VERSION-CONTROL-END-TO-END/🧪️generate-history-transition-fixture.py` (Python varint/length-prefixed encoder written from the wire grammar).
- Rust consumer: `FW/📡️replication/🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs:296-318` `the_language_agnostic_fixture_matches_the_codec_byte_for_byte` (`include_str!` of the corpus, asserts `encode == payloadHex` and decode round-trip for accepted, error-detail substring for malformed). Mounted via `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` at `🔗️causal/🔀️transition/🦀️.rs:476`.
- TS consumer (third-party validation): `FW/📡️replication/🧪️tests/🧪️history-transition/🟦️.ts` (`registerTests3`) validates the corpus against the JSON Schema with **Ajv 8 (strict)**, re-encodes every accepted case with a TS encoder written from the wire grammar alone and compares to `payloadHex`, and refuses a bogus `merge` kind. Registered from `FW/📡️replication/🟦️.ts:1826-1827` (vitest in-source). Three independent implementations pin the corpus: Python generator, Rust codec, TS encoder + Ajv.
- Caveat: this pattern validates an encoder/schema with Ajv; it is not a third-party *history-editing* engine. A replay/suffix algebra needs its own oracles (section 3.5).

### 3.3 Concrete example B: tool-run lifecycle law (best template for tool state machines)

- Fixtures `FW/⏯️tool-run/🧫️fixtures/{⚖️lifecycle-law,📼️trace-pages,🎞️ticks}.json` + schema `FW/⏯️tool-run/🧬️schema/🔣️.json`.
- Rust: `FW/⏯️tool-run/🧪️tests/🔬️unit/🦀️.rs` (`include_str!` of all three fixtures).
- TS: `FW/⏯️tool-run/🧪️tests/🧩️conformance/🟦️.ts` (`bun:test`) with THREE third-party oracles: **ajv** (schema), **xstate** (a `createMachine` built from the law matrix; compared with the TS reducer), **fast-check** (random event sequences, reducer vs xstate). `xstate`/`fast-check` are installed in root `node_modules` (used here; verify how they are declared with `bun ./📜️script.ts verify dependencies literal-external` before adding a new user).
- Runner: `FW/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts` `test` = `runTestBudgeted(bun test ...conformance)` then `runCargoTestBudgeted(["semio-framework-tool-run"], ...)`; `check` = `cargo check -p` native AND `--target wasm32-wasip2`.

### 3.4 Concrete example C: puzzle 2d mutation cross-language differential

- `PZ2D/🧪️tests/◻️mutate-puzzle-2d-1/{🥒️.feature, 🐍️.py, 🦀️.rs}`: Rust adapter replays the committed quintets; `🐍️.py` is an INDEPENDENT second implementation (declared `cross-semio-implementation`, "a required supplement, never a substitute").
- `PZ2D/🧪️tests/🕸️third-party-puzzle-2d-1/{🥒️.feature,🐍️.py}`: **networkx** (topology; `graphs_equal`), **shapely** (geometry), **jsonschema**, **jsonpatch** on CPython. `PZ2D/🧪️tests/🌐️third-party-puzzle-2d-1/{🥒️.feature,🟦️.ts}`: **graphology 0.26**, npm **jsonschema 1.5**, **fast-json-patch 3.1** in bun. `ajv` is deliberately declined there because `verify dependencies literal-external` counts an oracle also reachable from production as an `oracle-conflict`; use test-only libraries for registered oracles.
- Registered oracles (`PZ2D/🔮️oracles/🔣️.json`): python networkx 3.6.1, shapely 2.1.2, jsonschema 4.26.0, jsonpatch 1.33, deepdiff 9.1.0; typescript graphology 0.26.0, jsonschema 1.5.0, fast-json-patch 3.1.1. Rust side: `json-patch` 4.1.0 exists in `Cargo.lock` (used only by `semio-s-artifact-stdio-contract`).
- Nx case projects: `test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-◻️mutate-puzzle-2d-1` (and `-🌐️third-party-puzzle-2d-1`, `-🕸️third-party-puzzle-2d-1`), targets `lint,test,test-contract,test-oracle,test-parity,test-quick,test-long,test-exhaustive,test-subject`; `test` = `bun ./📜️script.ts run --owner "PZ2D" --case ◻️mutate-puzzle-2d-1` (run in `TESTDOM`; root router also exposes `bun ./📜️script.ts test <phase> --owner ... --case ...`).
- Test-platform phases (`TESTDOM/README.md`): `discover | doctor | contract | oracle | subject | parity | run | report | clean | dependency | inventory | fixture <generate|reproduce|verify|audit> | probe | matrix | gc`. The lifecycle for a new feature is spelled out there (owner -> case -> reference implementation -> scenarios -> contract green -> oracle green -> subject -> other languages -> parity -> dependency -> clean -> delete replaced legacy test).
- Ratchets to respect: `🚚️migration.json` (`unmanagedTests` total 48; a count may only shrink: executable test files outside the canonical owner-root `🧪️tests` tree) and `🔒️dependencies.json` (`verify dependencies` fails on NEW third-party deps; `write-baseline` is contended, coordinator only).

### 3.5 Third-party validation candidates for history editing (all already in the repo)

| Question | Rust | TS | Python |
|---|---|---|---|
| does suffix replay produce the committed after-state / diff | `json-patch` crate (in `Cargo.lock`; would need a dev-dep) | `fast-json-patch` 3.1.1 (registered oracle) | `jsonpatch` 1.33, `deepdiff` 9.1.0 |
| tool state machine (transactions, accept/discard, pause) | own reducer | `xstate` (used by tool-run conformance) + `fast-check` | `pytest` (uv `test` group has pytest 9) |
| payload/wire shape of new mutation input metadata | schema-first JSON Schema | `ajv` (in-source tests) or `jsonschema` (registered oracle) | `jsonschema` 4.26.0 |
| topology cascades on replay | - | `graphology` 0.26.0 | `networkx` 3.6.1 |

## 4. `.vscode/launch.json` and the seed

### 4.1 Structure (verified by parsing with `Bun.JSONC.parse`)

- **`.vscode/launch.json` is GENERATED. Never hand-edit it.** Source of truth: `.vscode/🧩️launch.seed.jsonc` (5235 lines) + the plugin playground registry (`[[package.metadata.semio.playground]]` blocks in plugin `Cargo.toml`) + every nx `📋️project.json` target. Generator: `OS/🔌️plugin/📇️registry/🚀️launch/🟦️.ts` (`generateLaunchJson`); written by `bun nx run @semio-tech/plugin-registry:generate` (= `bun ./📜️script.ts generate` in `OS/🔌️plugin/📇️registry`, which regenerates the catalog and launch.json together); freshness gate `bun nx run @semio-tech/plugin-registry:check-generated` (also `preview-generated`). Stale launch bytes fail the gate.
- Current output: 1450 configurations, 42 inputs, 4 compounds. Groups: `0_dev` (4), `1_keyboard` (2), `2_mouse` (4), `3_dev` (580), `4_gate` (686), `4_build` (173).
- Row shape: `{ name, type: "node-terminal", request: "launch", command: "bun nx run <project>:<target>", cwd: "${workspaceFolder}", [env], presentation: { group, order }, [serverReadyAction] }`.
- Seed keys: `configurations` (477 curated rows and `"@generated:<variant>:react|wgpu|users"` placeholders), `compounds`, `inputs`, `devLaunchers` (47 playground variants -> `namePrefix`, `order`, `wgpuOrder`, `env`, `users`), `projectLaunchers` (policy).
- **Automatic rows for new nx targets** (`projectLaunchers`): every declared target in any `📋️project.json` that no curated row already runs (`nx run <project>:<target>`) gets a row named `<class emoji><target><project label>`; the class is decided by the target name's first token: `🛠️` dev (`3_dev`, base 900; tokens dev/serve/start/watch/activate/open/launch/inspect/demo/playground/attach), `📦️` build (`4_build`, 900; build/package/wasm/publish/release/bundle/generate/generator/typegen/codegen/preview/deps/fonts/prepare/materialize/install/compile/sign/deploy/bootstrap/clean/prune/restage/rebuild/setup/format/fix/regenerate), `⚖️` gate (`4_gate`, 900; test/check/verify/lint/typecheck/oracle/native/e2e/probe/audit/census/bench/parity/law/laws/drill/smoke/conformance/contract/validate/scan/report/doctor/discover/inventory/metrics), `▶️` run (`3_dev`, 950; run; also the fallback). The project label is the shortest unique trailing run of path segments with language folders shortened to `🦀️`/`🟦️`/`🐍️` and `📦️packages` dropped (`📚️library🟦️`, `💻️os🦀️`, `📡️replication🦀️`). A target name declared by >= 3 projects becomes ONE family row with a project picker (`familyEmoji 📋️`, e.g. `⚖️test📋️` -> `bun nx run ${input:projectTarget.test}:test`); the picker's `options` list is regenerated from the declared projects, so a new project with a `test` target appears there automatically.
- Names collide-checked (`generated launch name ... collides with an existing row` throws).
- Ordering convention: curated rows keep hand-picked orders (`puzzle2d` react 220, wgpu wasm 220.1, native 220.2; puzzle3d 230; puzzle5d 250; storybook rows 3xx-4xx); generated rows use `orderBase + 0.0001 * rank`.

### 4.2 Registering a new command (AGENTS.md rule: launch.json only, following existing order/grouping/naming)

1. Implement the subcommand in the owning `📜️script.ts` (extend `ScriptRouter(...).register("<cmd>", <Script>)`; no other script files).
2. Add the nx target to the owner's `📋️project.json`: `{ "executor": "nx:run-commands", "options": { "cwd": "<pkg dir>", "command": "bun ./📜️script.ts <cmd> [sub...]", "forwardAllArgs": true } }` (pattern of `FW/⏯️tool-run/📦️packages/🦀️rust/📋️project.json`: `test`, `test-quick`, `test-long`, `test-exhaustive`, `check`).
3. If the package exposes a script alias, `package.json` `scripts` must call `bun nx run <project>:<target>` (never the script directly).
4. Regenerate: `bun nx run @semio-tech/plugin-registry:generate` (launch row: the family row `📦️generate📋️`, pick `@semio-tech/plugin-registry`; freshness: `⚖️check-generated📋️` with the same pick, or `📦️check🔌️plugin-registry`). If a curated name/order is wanted instead of the auto row, add a row to the seed `configurations` in the matching group with the same emoji grammar and order slot; then regenerate and run `check-generated`. Family rows sit at orders `899.00xx` in `4_gate`/`4_build` (`⚖️check📋️` 899, `⚖️check-generated📋️` 899.0001, `⚖️test📋️` 899.0006, `-exhaustive` .0007, `-long` .0008, `-quick` .0009, `📦️build📋️` 899.0006 in `4_build`).
5. Root-level verbs go through the `workspace` project (`📋️project.json` at repo root): target `command: "bun ./📜️script.ts <verb> ..."`, and root `package.json` `scripts` call `bun nx run workspace:<target>`.

### 4.3 Rows that boot / test puzzle 2d and history-adjacent things

Boot (all `3_dev`):
- `🛠️dev🧩️puzzle◻️2d⚛️react` (order 220): `bun nx run workspace:dev -- puzzle2d`, env `S_OS_PORT=6012 SEMIO_PLUGIN=puzzle2d SEMIO_RENDERER=react SEMIO_APP="s.puzzle.puzzle2d@1/*#editor"`, opens `http://127.0.0.1:6012`.
- `🛠️dev🧩️puzzle◻️2d🧊️wgpu🌐️wasm` (220.1, port 6112); `🛠️dev🧩️puzzle◻️2d🧊️wgpu🖥️native` (220.2, `bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle2d`).
- `🛠️dev📖️storybook🧩️puzzle◻️2d` (340): `bun nx run workspace:dev-storybook-puzzle-2d` (6010).
- No curated launch row exists for `activate-puzzle2d-react-dev`, the serve supervisor, or the browser battery (those are ticket-local; the nx targets `@semio-tech/framework-os-dev:activate-puzzle2d-react-dev|serve-puzzle2d-react-dev|dev-puzzle2d-react-dev` are inferred by `LIB/🟨️.mjs:1099-1116` and would appear as auto family rows).

Chain and boot recipe (from `26/09/06/PUZZLE-2D-END-TO-END`, memory note "Puzzle2d React Boot And Probe"):
- Launch row -> root `📜️script.ts dev puzzle2d` -> `bun nx run @semio-tech/framework-os-dev:dev -- puzzle2d` (env from `frameworkOsPlaygroundDevEnv`, `LIB/🟦️.ts:2519`) -> nx `dev-puzzle2d-react-dev` -> depends on `activate-puzzle2d-react-dev` -> `prepare-...` -> `@semio-tech/plugin-registry:session-puzzle2d` + `framework-plugin-web:support-dev` + fonts + engine wasm (`surface`, `editor`, `os-flow`, `semio-s-plugin-puzzle` wasm-pack) + component `materialize-dev` of the `puzzle` plugin (wasm32-wasip2).
- Split recipe that survives peer churn (memory: dev variants watch all files):
  1. `bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev` (cold: engine wasm-pack 2-6 min + component 1-4 min; budget `SEMIO_BUILD_BUDGET_MS`, default 20 min per cargo spawn).
  2. Serve detached (`nohup` + log; the ticket supervisor `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🔁️serve-supervisor.sh` runs `cd OS/🧑‍💻dev/📦️packages/🟦️typescript && bun ./📜️script.ts serve puzzle2d react dev` and recycles :6012 when `curl` stops answering, since Vite exits silently after headless Playwright runs and after host-edit bursts). Open `http://127.0.0.1:6012/?plugin=puzzle2d` (boots with Nakagin: 180 nodes / 179 edges).
  3. Battery: `bun 🔍️browser-probe.ts --battery|--only=<steps> --port=6012` from that ticket folder (Playwright; reads `Board2dHost`'s `data-board-{nodes,edges,handles,positions-json,selection-json,camera-json,fixture-parsed}`; writes `probe-<stamp>.md/.ndjson` to the ticket's `🗑️generated`). Blank board with `data-board-nodes=180`: read `data-board-fixture-parsed` first.
  Skip flags: `SKIP_PLUGIN_BUILD=1`, `SKIP_ENGINE_BUILD=1`, `PUZZLE_BOARD_SKIP_WASM_BUILD=1`, `SEMIO_PLUGIN_ONLY=<id>`, `CARGO_PROFILE_WASM_DEV_DEBUG=false` (8.6 GB rustc without it).
- Surfaces: mode `edit` = three `Board2d` windows `window:2d-overview|2d-detail|2d-selection` (`PZ2D/✏️editor/🎭️modes/✏️edit`); only overview is interactive.

Gate/test rows relevant to this ticket (names from the generated file):
- `⚖️test📋️` (family picker; options include `@semio-tech/puzzle-2d-rs`, `puzzle-plugin`, `puzzle-js`, all rust/TS projects), `⚖️check📋️`, `📦️build📋️`, `⚖️test-quick📋️`/`-long`/`-exhaustive`.
- Rust: `⚖️test-native💻️os🦀️`, `⚖️gate🔌️plugin🦀️lib` (`bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache`), `⚖️test-local-interaction-native📡️replication🦀️`, `⚖️presence-peer-codec-check📡️replication🦀️`, `⚖️gate⏯️tool-run⚛️react` / `...🌐️de` (`framework-os-dev:tool-run-matrix`), `⚖️fixtures-lint🧩️puzzle🦀️` (`@semio-tech/puzzle-plugin:fixtures-lint`), `⚖️schema-check🖥️shell🦀️`, `⚖️conformance🖱️ui🧬️contract🦀️`.
- Repo gates: `📦️check🗿️taxonomy` / `📦️check🗿️taxonomy🧩️implementation` (`workspace:verify-taxonomy-report|implementation-report`), `⚖️verify-taxonomy-enforce`, `⚖️verify-taxonomy-implementation-enforce`, `📦️check🔒️dependencies📃️literal-external`, `📦️check🧅️layering`, `📦️check🔌️plugin-registry`, `⚖️gate🚧️production-placeholders`, `⚖️gate🎛️command-reachability`, `⚖️gate⚡️interactivity🎯️tool-jobs`, `⚖️gate📦️dependencies`, `⚖️verify-rust-warnings`, `⚖️verify-gate`, `🏛️check🧩️canonical-architecture` (`4_gate`, 899.99), `⚖️rust-taxonomy-mounts-check📇️registry`, `⚖️schema-check|verify|oracle|audit|test`.

### 4.4 `📜️script.ts` -> launch mapping

`launch row -> bun nx run <project>:<target> -> nx run-commands (cwd = package dir) -> bun ./📜️script.ts <cmd> [sub...] -> ScriptRouter.register(<cmd>, <XScript extends BundleScript>)`. Package `package.json` scripts point back at nx. Rust package scripts use `runCargoTestBudgeted(["<cargo name>"], repoRoot, rest)` (nextest when installed, profile per level from `.config/nextest.toml`, `--skip quick:: --skip long:: --skip exhaustive::` for levels above the requested one, `RUST_MIN_STACK=128 MiB`); TS scripts use `runVitest`/`runTestBudgeted`; artifact packages use `runArtifactRustPackageMain(import.meta.dir, "<cargo name>", { testFeatures?, twins? })` (`LIB/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts`). **`@semio-tech/puzzle-2d-rs` passes no `testFeatures`**, so `nx test` on it never enables `component-app-assembly` and therefore never compiles the editor tree (see 5.3).

## 5. Fast commands and pitfalls

### 5.1 Rust (run from repo root; foreground; wait if a lock is held)

| Goal | Command |
|---|---|
| quick type check, framework crates | `cargo check -p semio-framework-replication`, `-p semio-framework-tool-run`, `-p semio-framework-machine`, `-p semio-framework-schema`, `-p semio-framework-ui-contract`, `-p semio-framework-value-derive` |
| target-neutral proof (tool-run pattern) | `cargo check -p semio-framework-tool-run --target wasm32-wasip2` (what `📜️script.ts check` does) |
| unit tests (level `fundamental`) | `cargo test -p semio-framework-replication --lib transition` (peer-verified by `📓️explore-event-sourcing-core.md`: 16 passed, ~6 s warm), `cargo test -p semio-framework-tool-run --lib`, `cargo test -p semio-framework-machine --lib` |
| through the repo runner (levels, nextest, budget) | `bun nx run @semio-tech/framework-replication-rs:test` / `...-tool-run-rs:test` / `@semio-tech/framework-machine:test`; levels: `:test-quick`, `:test-long`, `:test-exhaustive`, or `bun ./📜️script.ts test quick <filter>` in the package dir |
| os-kernel (store, vcs, spr) | `cargo test -p semio-framework-os-kernel --lib <filter>` (huge crate, long cold build); nx `@semio-tech/framework-os-kernel:test` and `:test-native` |
| plugin runtime (`🔌️plugin/🦀️.rs`, tool-run os-side, interaction) | `cargo test -p semio-framework-plugin --lib <filter>` (e.g. `tool_run`), nx `@semio-tech/framework-plugin:test -- <filter>` |
| editor / shell | `cargo check -p semio-framework-editor`, `cargo test -p semio-framework-os-shell --lib`; nx `@semio-tech/framework-editor-rs:test\|wasm`, `@semio-tech/framework-os-shell-rs:test\|schema-check` |
| puzzle 2d schema + mutation vectors (not feature-gated) | `cargo test -p semio-s-artifact-puzzle-2d --lib <filter>` |
| **puzzle 2d editor / app assembly (feature-gated)** | `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly` and `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib <filter>` (the `26/09/19` runner script `🧪️run-final-convergence.sh` in that ticket uses exactly this feature switch for plugin crates) |
| puzzle plugin as shipped (wasm component) | `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2`; build: `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo rustc -p semio-s-plugin-puzzle --target wasm32-wasip2 --profile wasm-dev` (or nx `@semio-tech/puzzle-plugin:component-dev`, `:materialize-dev`) |
| puzzle mutation bridge (standalone workspace) | `cargo run --manifest-path ✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.toml -- list-mutations s.puzzle.2d 1 any` or `bun ./📜️script.ts list-mutations ...` in that dir |
| nx projects/targets | `jq` on `.nx/workspace-data/project-graph.json` (see section 0) |

Test binaries per level: tests live in `mod quick`/`mod long`/`mod exhaustive` submodules of the test module; un-scoped tests are `fundamental`.

### 5.2 TypeScript

| Goal | Command |
|---|---|
| replication TS (in-source vitest incl. history-transition Ajv test) | `bun nx run @semio-tech/framework-replication:test` or `cd FW/📡️replication/📦️packages/🟦️typescript && bun ./📜️script.ts test -t "history transition"` |
| machine TS | `bun nx run @semio-tech/machine:test` |
| tool-run TS conformance (bun:test with ajv + xstate + fast-check) | `bun test FW/⏯️tool-run/🧪️tests/🧩️conformance/🟦️.ts` |
| React host tests / types | `bun nx run @semio-tech/framework-renderer-react:test` and `:typecheck` |
| shell TS | `bun nx run @semio-tech/framework-os-shell:test` |
| puzzle TS | `bun nx run @semio-tech/puzzle-js:test`, `:test-renderer-contract`, `:publication-authority-audit` |
| puzzle mutation case (Rust + Python + TS, phases) | `bun nx run 'test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-◻️mutate-puzzle-2d-1:test-contract'` (also `:test-oracle`, `:test-subject`, `:test-parity`) |
| fixtures lint (puzzle, mutation leaf == variant == test subject) | `bun nx run @semio-tech/puzzle-plugin:fixtures-lint` (**caution**: running `bun ./📜️script.ts fixtures lint` directly in `PZ/📦️packages/🦀️rust` ran 13 min at ~93% CPU and 3.6 GB RSS without printing anything at machine load ~20, so I killed it; it walks every `🧬️mutations` tree; do not run it casually, and expect its `CORE_CASE_FILES` expectation (`<leaf>/🧪️tests/<case>/{🦠️mutation,🔺️diff,🎯️outcome}/🔣️.json`) to disagree with the peers' in-flight move of fixtures to `🧫️fixtures/🧬️mutations/<kind>/<case>/`) |

### 5.3 Pitfalls (each is documented in memory or observed here)

1. **One shared cargo build-dir** (`.cargo/config.toml`: `build-dir` and `target-dir` under `.🧬semio/🦑️repo/⚡️cache/cargo/`, nightly `fine-grain-locking`). Do not create private build dirs or set `RUSTC_WRAPPER`. Under fleet load `cargo test` can starve on the uplift lock for tens of minutes; a private `CARGO_TARGET_DIR` (target only, build-dir stays shared) fixes that (memory: private-target-dir-ends-cargo-test-starvation). `cargo check` never uplifts. A 0% CPU cargo with a live rustc child is working, not hung.
2. **Feature-gated code compiles to nothing by default**: puzzle 2d's whole editor is behind `component-app-assembly`. A plain `cargo test -p semio-s-artifact-puzzle-2d` is green over a fraction of the crate. Same trap class as `cfg(target_arch = "wasm32")` blocks, which native cargo never compiles: after any framework signature change also run `cargo check --target wasm32-wasip2` for the plugin crates and a wasm-pack/`wasm32-unknown-unknown` check for `semio-framework-editor`/`-machine`.
3. **Emoji test filenames**: cargo derives an integration test's crate name from the file stem; emoji stems break discovery silently. Integration-style tests in `<pkg>/🧪️tests/<case>/🦀️.rs` need `[[test]] name = "<ascii_snake>" path = "../../🧪️tests/<emoji><kebab>/🦀️.rs"` (24 Cargo.tomls do this, e.g. `FW/🔀️dispatch/📦️packages/🦀️rust/Cargo.toml`). `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` unit tests need nothing (but must be mounted).
4. **Every new file must be mounted** by `#[path]` (section 1.1); registry `rust-taxonomy-mounts-check` and the taxonomy verifier compare mounts to the tree.
5. **Explicit include lists**: React host vitest config lists every suite; replication tests are registered from `🟦️.ts` in-source. A test that no runner names never runs.
6. **`timeout` does not exist on macOS**; wrapping cargo in `timeout N` never runs it. Use `/usr/bin/grep` (the shell `grep` alias misses matches on emoji paths). `cd /Users/ueli/Documents/semio` explicitly in every Bash call.
7. **20-min budget per cargo spawn** (`SEMIO_BUILD_BUDGET_MS`); swap exhaustion kills builds silently; `CARGO_PROFILE_WASM_DEV_DEBUG=false` for wasm-dev plugin builds.
8. **Dev serve fragility**: Vite exits silently after Playwright runs and host-edit bursts, `dev <variant>` starts `nx watch --all` (peer edits re-run activation). Activate once, serve detached with `nohup` + log, probe with `curl --max-time 15 http://127.0.0.1:6012/` before trusting a browser verdict (`000` with LISTEN = dead). Hidden browser panes throttle plugin boot.
9. **Native `-p semio-s-plugin-*` check pulls the plugin host** (`semio-framework-os`); if it fails only inside `🔌️plugin/🖥️host/**` or `💻️os/🖥️host/**` that is a peer's in-flight breakage: verify with `--target wasm32-wasip2`.
10. **Nx hasher is blind to gitignored files** and unions/negates `inputs` globs globally: a cached target that depends on generated/ignored files needs a runtime digest input.
11. **Concurrent churn right now** (staged/unstaged in `git status`): `FW/📡️replication/🚧️apply-refusal/**` (new module), puzzle reject-vector fixture renames under `PZ2D/🧫️fixtures/🧬️mutations/**` and their `🧪️tests/**/🦀️.rs`, `PZ/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts}`, `PZ/🧑‍💻dev/**` and `PZ/🧪️tests/**` additions, `FW/🔄️machine/✨️derive/.../Cargo.toml`, `.vscode/launch.json` + seed. Re-read before editing; never revert; nx project files of `@semio-tech/puzzle-plugin` changed at 00:39-01:17 today.
12. **`bun ./📜️script.ts verify taxonomy report --scope <dir>` can crash on unrelated churn**: the puzzle 2d scope died after 2 min with `frozen-coordinate-evidence-invalid: ...🧫️fixtures/📐️cad-draw-path-projection/🔣️.json: document digest does not match registered bytes` (not our files). Prefer small scopes (5 s) such as `FW/<module>`.

## 6. Taxonomy SSOT and gates

### 6.1 The SSOT

- `LIB/🔣️taxonomy.json` (1.1 MB, ~250 keys). Key tables: `fileKinds` (87: emoji + extension chains + role), `semanticDirectoryKinds` (676), `semanticCollections`, `fixedDirectoryContracts` (104), `fixedFilenameContracts` (513), `packageBoundaryRules`, `packageGlueGrammar`, `implementationLeafPolicy`, `pathEmojiPolicy`, `collisionPolicy`, `mutation*` (`mutationDirectoryPattern = ^.+️[a-z][a-z0-9]*(?:-[a-z0-9]+)+$`, facet dirs `🦠️mutation`, `🔺️diff`, `↩️inverse`; optional `🧩️plan`, `📝️text`, `💾️binary`, `🧬️schema`), `test*` (`testsDirName 🧪️tests`, `testFixturesDirName 🧫️fixtures`, `testOraclesDirName 🔮️oracles`, `testCaseSlugPattern`, `testLevels`, `testModes` differential|conformance|round-trip|property|error).
- Loader/validator code: `LIB/🧹️normalization/🟦️.ts` (`canonicalDirectory` at ~3150, `verifyTaxonomy`), discovery `LIB/🔍️discovery/🟦️.ts`. Companion docs: `LIB/📓️schema-catalog.md`, `LIB/🔣️schema-catalog.json`.

### 6.2 Naming rules a new file/dir must satisfy

- **Files use kind-only basenames** = `<file-kind emoji>.<registered extension>`: `🦀️.rs`, `🟦️.ts`, `🟦️.tsx`, `🔣️.json`, `🥒️.feature`, `🐍️.py`, `🧪️.story.tsx`, `📝️.md`, `📌️.empty.md`, `⚙️.jsonc`. Never `foo.rs`. Flat `<emoji><slug>.json` files under a `🧫️fixtures/` directory are accepted today (tool-run's `⚖️lifecycle-law.json`, `🎞️ticks.json`, `📼️trace-pages.json` raise no finding). Exceptions are fixed contracts: `📜️script.ts`, `📋️project.json`, `Cargo.toml`, `package.json`, `tsconfig.json`, `🛂️manifest.json`, `🗑️generated`. Fixture case artifacts inside a case use kind names too (`🦠️mutation/🔣️.json`).
- **Directories** = `<one leading emoji grapheme + U+FE0F><kebab-slug>` (NFC, case-fold lower; VS16 comparison ignores the selector but write it). Emoji identities must be UNIQUE among siblings (files and directories share one namespace). Banned name stems: core, common, util(s), helper(s), misc, shared, base, lib, impl. Forbidden path segments: `⚡️implementations`. Max path 240 bytes, no Windows reserved names.
- **Every directory must resolve to a registered kind, by one of two mechanisms** (`LIB/🧹️normalization/🟦️.ts` `canonicalDirectory`, `matchDirectoryKind`):
  1. **Open patterns** (`semanticDirectoryKinds` with a kebab `slugPattern`, no registration needed):
     - `🧪️tests/🧪️<kebab-slug>` = kind `test-case` (emoji MUST be `🧪️`; allowed under `tests` and 80 other parent kinds). Inside a case: `🧫️fixtures/🧪️<member>` (`test-fixture-member`), `🖼️<asset>` (`test-fixture-asset`).
     - `🧫️fixtures/🧫️<kebab-slug>` = `fixture-case` (emoji `🧫️`; parents `fixtures`, `test-tube-fixtures`).
     - `🧬️schema/🔣️<kebab-slug>` = `schema-artifact-subject` (emoji `🔣️`; parent `schema`), e.g. `🧬️schema/🔣️history-edit/🔣️.json`.
     - `📖️stories/🎭️<kebab-slug>` = `story-case`.
  2. **Explicit registered member lists**: `semanticDirectoryMemberKinds.<members-of-X>.memberNames` (`source: "registry"`, persisted INSIDE `LIB/🔣️taxonomy.json`, NFC emoji-leading names): `members-of-tests` (1483 names, e.g. `🔬️unit`, `🔬️host-unit`, `⏯️tool-run-matrix`), `members-of-schema` (1612), `members-of-commands` (836), `members-of-fixtures` (286), `members-of-modules` (145: `⏯️tool-run`, `📡️replication`, `🔄️machine`, `🕹️interaction`, `✍️editor`, ...) plus ~200 domain-specific kinds. A name that matches no open pattern and is not in its parent's list is `directory-kind-unresolved`; new kinds go into `semanticDirectoryKinds` (`{emoji, slugPattern, parentKindIds}`) or a new name into the matching `memberNames` in the same change (the taxonomy file is contended: edit atomically).
  - **Evidence from today's scoped reports (all pre-existing, all `directory-kind-unresolved`)**: `FW/📡️replication/🔗️causal/🔀️transition` (the 26/09/19 history module itself), `.../🔗️causal/🧫️fixtures/🔀️history-transition-v1`, `.../🔗️causal/🧬️schema/🔀️history-transition-v1`, `.../🧪️tests/🗄️durable-collaborative-redo`, `.../🚧️apply-refusal` (peer, new), `🎮️mutation/🗂️map`, `👕️peer-overlay`, `📐️format/🔎️verification`, every `🔬️<name>` test dir not in the registry, `⏯️tool-run/🧪️tests/🧩️conformance` and `🔬️interactivity-tool-run-policy`, `🔄️machine/🧪️tests/🔀️toggle-machine`, `🕹️interaction/👆️gesture`, `✍️editor/🧪️tests/⌨️text-input`. NOT flagged: `📡️replication/🧪️tests/🧪️history-transition` (open `test-case`), `🔬️unit`. Copying the 26/09/19 layout (`🔀️`-emoji fixture/schema dirs, `🗄️` test dirs) adds new findings; use the open patterns above instead.
  - The taxonomy builder (`LIB/🏗️builder/🟦️.ts:40`) throws `Unregistered authoring directory: "<name>"` for a directory that is neither pattern-matched nor listed.
- Leaf placement: authored implementation lives at `<semantic collection>/<specific dir>/<file-kind>.<ext>`; test adapters at `<owner>/🧪️tests/<case>/<kind>.<ext>`; language-neutral assets (schema, fixtures, oracles, examples) at the owner root, never inside `📦️packages`. A package root admits only exact contracts and thin glue (<= 150 lines).
- **Reusable modules require two independent production semantic consumers at their lowest common owner** (`_comment`); shared code is placed at the lowest common owner (`standard -> artifact -> plugin -> ✏️s/🔨️modules -> framework`). A new artifact-agnostic framework module (`FW/<emoji><slug>`) therefore needs at least two consumers on day one (e.g. puzzle 2d plus a second artifact or the plugin runtime).
- Mutations: a mutation directory `🧬️mutations/<emoji><verb>-<noun>/` directly owns ONE `🦀️.rs` (payload + dispatch only); apply/diff/inverse live in `🦠️mutation`, `🔺️diff`, `↩️inverse` facet dirs (`_mutationOwnershipComment`; `mutationDirectLeafForbiddenRegionMarkers` catches inlining). Tests: `🧪️tests/<emoji><case>/🦀️.rs`. Schema per mutation: `🧬️schema/🔣️.json`.

### 6.3 Gates (commands)

- `bun ./📜️script.ts verify taxonomy report|enforce [--scope <path>]`, `verify taxonomy implementation report|enforce` (implementation-leaf policy), launch rows above. Report mode never fails; enforce fails on any error or warning. Today the repo is NOT clean in report mode (replication 24 errors, machine 7, interaction 3, editor 2, tool-run 2, all pre-existing `directory-kind-unresolved`, plus `fixed-source-disposition-unresolved` for `FW/📡️replication/📦️packages/🦀️rust/📜️script.ts`), so add no new findings and expect enforce to be red for unrelated reasons.
- `bun ./📜️script.ts verify dependencies [literal-external]` (ratchet + census, oracle-conflict detection), `verify layering` (clean-architecture direction: `repo-wide` and `framework` may be depended upon by implementations, never the reverse), `verify docstrings`, `verify debug-tags` (`[DEBUG] ` prefix rule), `verify rust-warnings`, `verify mutation-outcome-law`, `verify interface-owners`, `verify interactivity [commands|tool-jobs|apps]`, `verify package-purity`, `verify semantic-vocabulary`, `verify canonical-architecture`, `bun nx run @semio-tech/plugin-registry:check|check-generated|rust-taxonomy-mounts-check|surface-schema-check`, `bun nx run workspace:schema-check|schema-verify|schema-oracle`.
- Docstrings: AGENTS.md requires every docstring to start with a unique fitting emoji (`//! 🔀️ ...`, `/// 🧮️ ...`, TS `/** 🔬️ ... */`); regions use `//#region 🔖️Name` ... `//#endregion 🔖️Name`.

### 6.4 Template for a new framework module (nearest precedent: `⏯️tool-run`, 26/09/13; taxonomy-conforming names)

```
FW/<emoji><slug>/                         name must be appended to semanticDirectoryMemberKinds["members-of-modules"].memberNames
  🦀️.rs                                   owner (pure, target-neutral if it must run in wasm plugins)
  🟦️.ts                                   TS twin
  🧬️schema/🔣️.json                        schema-first (JSON Schema; fixtures reference its $defs)
  🧬️schema/🔣️<slug>/🔣️.json               (optional per-corpus schema; open pattern schema-artifact-subject)
  🧫️fixtures/🧫️<slug>/🔣️.json             language-agnostic corpus (open pattern fixture-case); tool-run keeps flat `🧫️fixtures/<name>.json` files, which are files not directories
  🧪️tests/🔬️unit/🦀️.rs                    in-crate unit tests, mounted `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` (`🔬️unit` is a registered test name)
  🧪️tests/🧪️<case>/{🥒️.feature,🦀️.rs,🟦️.ts,🐍️.py}   conformance case (open pattern test-case); third-party oracles ajv/xstate/fast-check/fast-json-patch in the TS adapter
  📦️packages/🦀️rust/{Cargo.toml, 📋️project.json, 📜️script.ts, 🦀️.rs, package.json}
  📦️packages/🟦️typescript/{package.json, 📋️project.json, 📜️script.ts, 🟦️.ts}   (only if a TS package is needed)
```

Registration checklist for a new Rust module crate:
1. Root `Cargo.toml`: one `members` line and one `[workspace.dependencies]` path entry (see `semio-framework-tool-run` at lines 232 and 366). The file is heavily contended: edit atomically; a crate only ever consumed as a path dependency can instead declare its own empty `[workspace]` (as `PZ/🏭️bridge` does), but never do that to an existing member.
2. `[package.metadata.semio] role = "framework" id = "<slug>"`, `[lints] workspace = true`, `[lib] name/path = "🦀️.rs"`.
3. `LIB/🔣️taxonomy.json`: module name into `members-of-modules`; any non-open directory name into `semanticDirectoryKinds` or the matching `memberNames`.
4. nx `📋️project.json` with `test`, `test-quick`, `test-long`, `test-exhaustive`, `check` (pattern of `FW/⏯️tool-run/📦️packages/🦀️rust/📋️project.json`); `namedInputs.default` must glob the owner tree (`{workspaceRoot}/FW/<module>/**/*`) because the sources sit outside the package dir. Consumers whose `test` inputs enumerate project lists (e.g. `@semio-tech/framework-os-kernel:test`) must list the new project so Nx re-runs dependents.
5. TS package: add to root `package.json` `workspaces`, then `bun install` for the workspace link.
6. Mount every file by `#[path]`; register vitest in-source tests from the owner `🟦️.ts` (`if (import.meta.vitest)` block) or name them in the package's vitest `include`; `[[test]]` with an ASCII name for integration tests.
7. Regenerate launch rows (`bun nx run @semio-tech/plugin-registry:generate`) and run `check-generated`; run `verify taxonomy report --scope FW/<module>`, `verify dependencies literal-external`, `verify layering`.
8. Reusable-module rule: two independent production semantic consumers at the lowest common owner.

## 7. Gaps, decisions the coordinator should know, and open questions

1. **Puzzle 2d is not activated** here (`dist/runtime/react/dev/puzzle2d` absent): a browser gate needs `bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev` first (minutes to tens of minutes under the current load 14-29 and swap 3.9/5.1 GB). Boot one tab, detached, with a supervisor; only one agent should own :6012.
2. **`@semio-tech/puzzle-2d-rs:test` does not enable `component-app-assembly`** (`◻️2d/📦️packages/🦀️rust/📜️script.ts` passes no `testFeatures`), so the editor/app-assembly tests only run with an explicit `--features component-app-assembly`. Any new editor-side test (edit-mutation input metadata, time-travel, tool state machine) will be invisible to `nx test` until that script passes `{ testFeatures: ["component-app-assembly"] }` (a one-line, greenfield fix in that `📜️script.ts`).
3. **Taxonomy is already red for the exact area we will touch** (replication 24, machine 7, interaction 3, editor 2, tool-run 2 findings; `🔀️transition`, `🧪️tests/🗄️durable-collaborative-redo`, `🚧️apply-refusal` unresolved). New directories must use the open patterns of 6.2 or be registered in `LIB/🔣️taxonomy.json`; do not copy the 26/09/19 folder naming. Per AGENTS.md ("refactor inconsistencies") the coordinator may want a separate slice that registers or renames those directories; that touches the contended taxonomy file, so give it one owner.
4. **Test-platform ratchets**: a new executable test outside `🧪️tests/<case>/` may raise `🚚️migration.json` `unmanagedTests` (48 today) and fail `test contract`; new mutations/verbs must be added to `PZ2D/🔮️oracles/🔣️.json` `mutationManifests`/`mutationCatalogs` (production dispatch == manifest == tests exactly), and every leaf needs `🧪️tests/<case>` plus a fixture quintet (`fixtures lint` D1/D6).
5. **Third-party evidence already installed** (no new dependency needed): `ajv`, `graphology`, `jsonschema`, `fast-json-patch`, `xstate`, `fast-check` (JS); networkx/shapely/jsonschema/jsonpatch/deepdiff (Python oracle hosts); `json-patch` (Rust, already in `Cargo.lock`, dev-dep would be new to the consuming crate). `verify dependencies literal-external` rejects a registered oracle that production can reach (`ajv` is production-reachable in five packages, hence the npm `jsonschema` in the puzzle third-party case). No Rust property-testing crate (`proptest`, `quickcheck`) exists in `Cargo.lock`; property laws would need the JS `fast-check` twin or a new dev-dependency.
6. **Language-agnostic test shape to follow for the feature**: (a) a JSON corpus under the module's `🧫️fixtures` validated by a JSON Schema under `🧬️schema` (schema-first); (b) Rust test via `include_str!` in `🧪️tests/🔬️unit/🦀️.rs`; (c) TS test with a third-party oracle (`ajv` + `xstate` + `fast-check`, or `fast-json-patch` for replay diffs) in `🧪️tests/🧪️<case>/🟦️.ts` registered from the owner `🟦️.ts` (vitest in-source) or run by `bun test` from the Rust package script (tool-run pattern); (d) an independent non-Rust generator for the corpus (the 26/09/19 Python generator pattern; the test domain's `fixture generate|reproduce|verify|audit` phases exist for provenance); (e) for artifact-level replay, a `🧪️tests/<emoji><case>/🥒️.feature` case in the puzzle 2d subset with a Python oracle plus a bun oracle, following `◻️mutate-puzzle-2d-1` and `🌐️third-party-puzzle-2d-1`.
7. **Existing runtime surfaces to hook a history-editing UI into** (paths only, no design): History table `FW/🖱️ui/🧱️elements/🕰️HistoryTable/🟦️.tsx`, `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🌳️GraphTimelineHost/🟦️.tsx`, controls `🎚️Slider`/`🪜️Stepper`, host `🖥️Board2dHost`, os-side tool-run runtime `OS/🔌️plugin/⏯️tool-run/🦀️.rs` (mounted at `OS/🔌️plugin/🦀️.rs:7378`), tool-run browser matrix `OS/🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/🟦️.ts`, HLC fold `FW/📡️replication/🔗️causal/🔀️transition/🦀️.rs` (`fold_history`), mutation trait family `FW/📡️replication/🎮️mutation/🦀️.rs`.
8. **Not verified by execution** (state explicitly when reporting): every command in 5.1/5.2 except the peer-verified `cargo test -p semio-framework-replication --lib transition`; the puzzle 2d boot chain; whether `verify taxonomy report --scope` of the new module will be clean; the `test-*` phases of the puzzle case projects; launch row regeneration. No claim in this report should be read as "passes".

## 8. Artifacts of this audit

- Report: this file. Scratch (mine, safe to delete at close): `🗑️generated/cargo-metadata.json` (1.6 MB), `crates.tsv` (275 rows: name, path, kinds), `taxonomy-scope-tool-run.log`, `taxonomy-scopes.log`, `taxonomy-replication.log`, `taxonomy-puzzle2d.log` (crashed run), `puzzle-fixtures-lint.log` (empty, killed run).
