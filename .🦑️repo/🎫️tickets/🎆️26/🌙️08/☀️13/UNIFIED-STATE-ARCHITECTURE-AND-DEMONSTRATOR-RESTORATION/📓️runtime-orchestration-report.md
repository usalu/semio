# Runtime Orchestration, Launch, and State-Probe Report

Date: 2026-08-14

## Scope

This lane inventories and verifies the registered app launch surfaces, canonical `bun`/`nx` routing, plugin playground catalog, build/start behavior, and state-transition E2E harness. It does not change shared state, renderer, kernel, plugin command-model, or plugin business logic.

## Canonical execution architecture

- Root router: `📜️script.ts`.
- Plugin catalog source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🔣️playgrounds.json`.
- Framework dev runner: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`.
- Canonical development command: `bun nx run @semio-tech/framework-os-dev:dev -- <variant>`.
- Canonical build command: `bun nx run @semio-tech/framework-os-dev:build -- <variant>`.
- Canonical state/parity command: `bun nx run @semio-tech/framework-os-dev:parity -- probe <variant> state` or `... parity -- verify <variant...>`.
- Authoritative VS Code launch manifest: `.vscode/launch.json`, generated from `.vscode/🧩️launch.seed.jsonc`.
- Secondary launch manifest: `.claude/launch.json`.

The registry contains 59 plugin crates, 58 runnable playground variants, and 24 framework packages. The generated catalog is the source of variant aliases, plugin IDs, application IDs, renderer ports, assets, examples, and engines.

## Launch manifest inventory

`.vscode/launch.json` parses as JSONC and contains 232 configurations:

| Group | Count | Entries |
|---|---:|---|
| Dev | 177 | Repository/framework tools, all catalog app variants and renderer launch surfaces, Storybook, hub/dashboard/coda/compose/print, and demonstrator launch surfaces |
| Gate | 11 | Repository/framework/plugin verification gates |
| Build | 30 | Framework, renderer, plugin, demonstrator, FEM, and publication builds |
| Publish | 5 | Registered publication/release surfaces |
| Other | 9 | Generation, maintenance, and orchestration surfaces |

`.claude/launch.json` contains 42 configurations: 37 app/dev launch entries and 5 parity/triage/sweep or framework support entries. Six stale secondary-launch port bindings were corrected to the catalog/primary-launch values:

| Entry | Corrected port |
|---|---:|
| `shooting-react-dev` | 6019 |
| `shooting-wgpu-dev` | 6119 |
| `forms-react-dev` | 6058 |
| `playbook-react-dev` | 6085 |
| `remodel-react-dev` | 6063 |
| `remodel-wgpu-dev` | 6163 |

## Generic state-management probe

The existing framework-dev parity harness now has a catalog-default `stateTransition` probe. It:

1. obtains independent React DOM and WGPU introspection dumps;
2. intersects explicit, enabled, visible, non-zero-size app-declared `#id` controls;
3. prioritizes toggle, select, slider, button, and activatable stack controls;
4. invokes the same semantic path independently in both renderers using each renderer's coordinates;
5. captures before/after semantic digests, node counts, and exact changed paths;
6. passes only when both renderers expose a change in topology, text, visibility, disabled, or selected state;
7. reports `SKIP`, rather than a false pass, when the app exposes no common declared action surface;
8. reports `FAIL` when controls exist but no attempted action produces observable state on both renderers.

This replaces the framework-command-palette shell probe as the catalog default. The shell probe remains opt-in because command-palette chrome is outside `data-ui-path`/WGPU app introspection and cannot prove app state management.

The JSON parity report contains full evidence per variant. The Markdown report adds State, Action, React delta count, and WGPU delta count columns.

Harness verification after the final prebuild and isolated-target changes: `bun nx run @semio-tech/framework-os-dev:test-quick` passed 2 test files and 16 tests in 4.88 seconds. The added cases verify common explicit-control selection/priority and semantic change evidence independent of focus-only changes.

## Sweep orchestration

The parity runner now performs one explicit plugin prebuild per variant before either renderer starts, then launches React and WGPU with `SKIP_PLUGIN_BUILD=1`. This removes the previous duplicate plugin builds, avoids advertising a renderer port before its plugin is staged, and retains the real current-source artifact. The build artifact lookup follows `CARGO_TARGET_DIR`, so an isolated target does not accidentally stage an artifact from the repository-global target.

The 58 variants are split across four shards. All shards use the single shared ticket-local target `🎯️target-parity`, retaining Cargo cache reuse while isolating the sweep from unrelated repository builds. Variant failures are caught and emitted as `SERVER-FAIL` evidence so each shard continues rather than stopping on its first failure. Per-variant prebuild, React boot, WGPU boot, JSON, and Markdown evidence remains in the four `parity-shard-*` ticket directories.

The WGPU launch surface had several orchestration/taxonomy defects, all repaired in the existing runner and manifests:

- the Cargo package selector now matches `semio-framework-os-renderer-wgpu`;
- the serve path resolves the asset server from the selected program instead of referencing an undefined `plugin` variable;
- Boot TypeScript/JavaScript, native dev-package, Rust glue, output, watch, plugin-module, and asset paths now use their canonical taxonomy locations;
- Trunk publishes plugin modules under the requested ASCII `plugin-modules` URL instead of retaining the emoji source-directory basename;
- parity launches suppress warning floods only in the parity child, preventing Trunk's piped Cargo process from failing after hundreds of thousands of warning bytes while preserving warnings for ordinary developer launches;
- WGPU browser boot is resolved solely from the canonical `?plugin=` URL and generated catalog, with host `s` as the no-query fallback. It no longer imports the concurrently overwritten generated session, so every Bun build emits variant-stable boot bytes;
- parity Trunk ignores changes under the shared plugin-module output after its initial copy. Concurrent variant prebuilds can therefore materialize plugins without forcing every live WGPU server into a rebuild loop; ordinary developer launches retain normal watch behavior.

The WGPU Cargo manifest also separates native-only kernel `sync` from the common wasm dependency and enables `getrandom`'s `wasm_js` target feature. `cargo tree --target wasm32-unknown-unknown -i mio` then returned no dependency path. The shared renderer subsequently passed an isolated `cargo check --tests` before the stable-revision sweep.

## Runtime and build evidence

### Registry check

`bun nx run @semio-tech/plugin-registry:check` was rerun after the `cad` and `koordinator` asset-root repair. It passes the former asset-root phase, then exits 1 on the systemic plugin taxonomy-tree validator. The current output spans many plugins and reports missing required taxonomy directories, Rust components not declared by `📦️glue.rs`, declared paths whose files do not exist, and missing root command components. Representative failures include `writer`, `space`, and `sourcing`. These plugin taxonomy/business-layout failures are outside this orchestration-only lane and were not changed.

### Representative clean catalog build

`bun nx run @semio-tech/framework-os-dev:build -- note` originally exposed the common command-model bridge failures. The app-conformance lane repaired that bridge and the Puzzle 5D nested action ownership. The state-infrastructure lane then removed obsolete flat `AppDefinition.actions` host literals and validated the canonical isolated kernel and host Nx checks. The common compiler blockers are therefore cleared. The earlier representative Note build was interrupted with exit 130 while waiting on a repository-global Cargo link and is not claimed as a completed build; the live catalog sweep below is the replacement current-source evidence and uses the isolated ticket target.

### React server smoke

`SKIP_PLUGIN_BUILD=1 SEMIO_RENDERER=react S_OS_PORT=7398 bun nx run @semio-tech/framework-os-dev:dev -- note` reached Vite ready in 1.916 seconds. HTTP returned 200, the browser title was `semio·os`, `#root` contained 150 descendants, and there were zero page errors. It rendered framework chrome only and had zero `data-ui-path` nodes because current plugin artifacts were deliberately skipped; this proves runner/server health only and is not counted as app E2E.

### Demonstrator runtime

The demonstrator on port 6029 eventually mounts. Browser evidence shows its guided-tour state transition `1/3 -> 2/3` succeeds. Initialization first logs timeouts for `procedural` and `demonstrator`, then both workers eventually become live. Typed command dispatch still warns that `setContributions` and `setAppRegistrations` pushes are skipped because they are absent from the typed command vocabulary/reserved actions. The demonstrator is therefore runtime-visible with one proven UI state transition, but is not architecturally clean or fully passing.

## Baseline catalog matrix before shared repair

Status definitions:

- `PASS`: clean current-source build, boot, and concrete state transition confirmed.
- `FAIL`: build/boot reached the app but a runtime or state assertion failed.
- `NOT-STARTABLE`: a shared dependency or required manifest validation prevents a current-source app launch.
- `PENDING-RERUN`: the common blocker is actively being repaired and the full per-variant browser sweep has not yet been able to execute.

This table preserves the initial sweep baseline: all 58 variants were `NOT-STARTABLE` through the then-current shared compiler blocker. That blocker is now repaired. The active four-shard ticket-isolated sweep is replacing these rows with exact live build, boot, and state evidence; no row below is a claim about post-repair status.

| # | Variant | Build/boot | State evidence | Current blocker |
|---:|---|---|---|---|
| 1 | aggregator | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 2 | animate | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 3 | architect | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 4 | aussuchen | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 5 | bearbeiten | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 6 | block2d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 7 | block3d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 8 | block5d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 9 | cad | NOT-STARTABLE | PENDING-RERUN | shared bridge + missing static root |
| 10 | dag | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 11 | din16798 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 12 | din18599 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 13 | din4108 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 14 | draw | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 15 | en1990 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 16 | en1991 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 17 | en1992 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 18 | en1993 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 19 | en1994 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 20 | en1995 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 21 | en1996 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 22 | en1997 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 23 | en1998 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 24 | en1999 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 25 | fem2d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 26 | fem3d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 27 | flow | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 28 | forms | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 29 | generator | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 30 | gis2d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 31 | gis3d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 32 | imperative | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 33 | iso16757 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 34 | koordinator | NOT-STARTABLE | PENDING-RERUN | shared bridge + missing static root |
| 35 | layout | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 36 | lowpoly | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 37 | mathematical | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 38 | note | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 39 | playbook | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 40 | procedural2d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 41 | procedural3d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 42 | process3d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 43 | puzzle2d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 44 | puzzle3d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 45 | puzzle5d | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 46 | raster | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 47 | reasoning-wires | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 48 | remodel | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 49 | s | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 50 | sequence | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 51 | shooting | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 52 | sourcing | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 53 | trinity-jack | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 54 | trinity-rewrite | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 55 | vcs | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 56 | vdi3805 | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 57 | verfolgen | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |
| 58 | writer | NOT-STARTABLE | PENDING-RERUN | shared command-model bridge |

## Active post-repair rerun

1. Registry check rerun: former asset-root failures cleared; systemic taxonomy validator still fails.
2. Shared compiler repairs validated by the responsible lanes; old global-target representative build is not counted.
3. Four-shard 58-variant sweep active with a shared ticket-local Cargo target.
4. Generated JSON/Markdown reports and per-renderer boot logs remain in this ticket.
5. Final rows require per-variant boot plus `PASS`, `FAIL`, or explicit `SKIP` state evidence; framework chrome alone is never counted.
