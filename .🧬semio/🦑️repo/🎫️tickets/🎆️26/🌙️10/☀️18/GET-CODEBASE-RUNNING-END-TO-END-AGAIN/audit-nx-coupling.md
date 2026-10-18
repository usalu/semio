# 🧭️ Nx Coupling Audit Outside The Nx Adapter

Date: 2026-10-18. Read-only static audit of git-tracked files. No builds, tests, `bun repo` invocations or edits were run.

Excluded from the adapter scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/` and `…/📚️library/🏗️build/🔧️tools/`.
Ticket folders under `.🧬semio/` are counted separately (class T), not as product code.

Classes: **A** production or test path bypassing the neutral interface, **B** project.json executor Bazel cannot run, **C** stale docs or strings, **T** ticket artifacts only.

Path prefixes used below: `L/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `FW/` = `🧰️framework/🔨️modules/`, `OS/` = `🧰️framework/🛍️products/💻️os/🔨️modules/`, `PL/` = `✏️s/🔌️plugins/`, `DM/` = `♻️mit-bestand/🧺️demonstrator/🔨️modules/`.

## Summary

| Area | Count |
|---|---|
| Product `project.json` executors (non-ticket) | `repo:script` 3,375 occurrences, `repo:noop` 1, `nx:run-commands` 0, `@nx/*` executors 0 |
| Ticket-only `nx:run-commands` (class T and B-ticket) | 934 occurrences in 40 tracked `.🧬semio/` files |
| Direct `bun nx` / `bunx nx` / `npx nx` / `nx run` in product code | 1 comment (`.storybook/🧪️tests/🧪️browser-runner/🟦️.ts:4`) |
| `require("nx")` / `from "nx"` / `@nx/` imports in code | 0 (only `package.json` deps, `nx.json`, `🔒️dependencies.json`) |
| `nx.json` readers (production + test support) | 1 production, ~11 test sites |
| Direct Nx CLI invocation in tests | 3 sites |
| `.vscode/launch.json` `bun nx` / `nx:` commands | 0 |
| `.vscode/launch.json` commands naming `test-nx-*` targets | 17 (launch.json), 14 (seed) |
| `.vscode/launch.json` `bun repo tools setup nx` | 2 (launch.json), 2 (seed) |
| `.vscode/launch.json` `bun repo run workspace:dev…` | 351 (launch.json), 33 (seed) |

## B. Bazel Coverage Gap

**Product code: no gap.** The neutral graph accepts only `repo:script` and `repo:noop` and throws on any other executor:
- `L/🏗️build/🕸️graph/🟦️.ts:81` (`Unsupported project executor`).
- The Bazel tool dispatches on `task.definition.execution.kind` (`L/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:279-285`), so both neutral kinds are covered.

**Ticket-only gap (not run by the product graph):** `nx:run-commands`, 934 occurrences in 40 files, for example:
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/headroom/project.json` (about 30 entries)
- `…/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/project.json`
- `…/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/native-cleanup/project.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/wp-st1/payload/✏️s/🔌️plugins/🗄️stdio/…` (9) and `wp-st2/…` (9)
- `…/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/wfc-bridge-inputs/original/✏️s/🔌️plugins/🀄️wfc/…` (5)

Ticket launch entries (`.vscode/launch.json`, `headroom/📜️script.ts nx-*` commands) are class T.

## A. Bypassing The Neutral Interface

### A1. Production code

1. `FW/🧬️schema/📇️registry/🦀️.rs:277-293`: `install_default_schema_leaf_reader` finds the repository root by walking ancestors for `nx.json` (line 290). This is in production code, not `cfg(test)`, and is used by every artifact that serves schema documents.
2. `L/🔌️nx-plugin/` (whole directory, outside the adapter dirs): the Nx plugin implementation lives in the neutral library.
   - `…/🔌️nx-plugin/📤️arguments/🟨️.mjs` has 14 `nx` references.
   - `…/🔌️nx-plugin/📤️arguments/🟨️.mjs:21` enforces the Nx-only env contract `SEMIO_NX_COMMAND_RUNNER` and `SEMIO_NX_COMMAND_DIRECTORY`.
   - Also `…/🔌️nx-plugin/🏃️execution/🟨️.mjs`, `…/🔌️nx-plugin/🌍️environment/🟦️.ts`, `…/🔌️nx-plugin/🧬️schema/`.
3. `--skip-nx-cache` flag, Nx-named, in tests and fixtures only. The flag definition was not located, so this needs verification. Sites:
   - `L/🧪️tests/🔧️build-surfaces/🟦️.ts`
   - `L/🧪️tests/🔬️workspace-contract/🟦️.ts`
   - `L/🏗️build/🧪️tests/🕸️graph/🟦️.ts` (2)
   - `L/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts:173` (asserts the owned routing does not include the flag)
   - `L/⚡️caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json:756,763`
   - `L/🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts` (16 `nx` references)
   - `launch.json` uses `--skip-cache` (the neutral flag), with 0 uses of `--skip-nx-cache`.

### A2. Test support and test code

4. Shared test-support root discovery via `nx.json`:
   - `FW/🧪️test/🧲️fixture-reader/🦀️.rs:48-57, 64, 86, 138` (ancestor search, panic message at line 57)
   - `FW/🧪️test/🧲️fixture-reader/🧪️tests/🦀️.rs:97-118`
5. Other `nx.json` walk-ups in tests:
   - `FW/🗣️dsl/🧪️tests/🧪️carrier-record-lists/🦀️.rs:14-15`
   - `FW/🖱️ui/📱️device/🧪️tests/🔬️unit/🟦️.ts:28-30`
   - `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs:111`
   - `PL/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️retirement/🧪️tests/🟦️.ts:13`
   - `PL/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/📦️member-authority/🌱️birth/🟦️.ts:15`
   - `FW/🎒️pack/🔤️json/🧪️tests/🚮️absence/🟦️.ts:30` and `FW/🗣️dsl/🧪️tests/🚮️absence/🟦️.ts:25` write `nx.json` as a fixture marker.
6. Direct Nx CLI from tests:
   - `FW/🏃️process/🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts:13` runs `node_modules/nx/dist/bin/nx.js run …`.
   - `L/⚡️caching/🧪️tests/📬️task-child/🟦️.ts:25` runs `sdk.resolve("nx/bin/nx.js")`.
   - `L/⚡️caching/🧪️tests/📬️task-child/🟦️.ts:15` creates a `.nx` symlink.
7. Stale test that encodes the Nx executor: `FW/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts:228-230` expects `project.targets.test.executor === "nx:run-commands"`, `entry.nx.includedScripts`, and `--skip-nx-cache`. The fixture target it reads, `FW/🧬️schema/📇️registry/📦️packages/🦀️rust/📋️project.json`, declares `repo:script`, so this assertion is stale and will fail.
8. Demonstrator runtime gate: `DM/🧩️runtime/🧪️e2e/🟦️.ts:13` throws "Demonstrator E2E must run through Nx". The gate itself reads the neutral `SEMIO_BUILD_INVOCATION_PID` (line 12), so only the message is Nx-specific (class C, see below).

### A3. Route naming

9. `workspace:dev` in launch entries: 351 occurrences in `.vscode/launch.json` and 33 in `.vscode/🧩️launch.seed.jsonc`. This is not an Nx route. The root `📋️project.json` (project name `workspace`) defines no `dev` target (only `dev-storybook*` and `dev-mcp*`). Dashboard tests reference `workspace:dev` as a registry id (`FW/🎛️dashboard/…/📚️inventory/🦀️.rs:179`, `…/🎮️registry/🧪️tests/🔬️unit/🦀️.rs:78`, `…/🧪️tests/🎮️registry/🟦️.ts:372`). It is probably dashboard-derived, so it is not a confirmed dead route. Needs a runtime check, which was not run.
10. `bun repo tools setup nx`: `.vscode/launch.json:199890, 200216` and `.vscode/🧩️launch.seed.jsonc:153902, 154228`. Provisioning named after the Nx tool. Minor, verify whether it should be `bun repo tools setup all`.

## C. Stale Docs And Strings

### C1. Comments and user-facing strings in product code

- `DM/📦️site/📜️script.ts:12`, `DM/🧩️runtime/📜️script.ts:17, 20, 38, 56`, `DM/🧪️e2e/📜️script.ts:9`, `DM/🧩️runtime/🧪️e2e/🟦️.ts:10, 13`, `DM/🧩️runtime/🧪️tests/🟦️.ts:45`: "outer Nx graph", "owned by Nx", "must run through Nx".
- `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts:3, 52`: "Nx contracts", "Nx output ownership".
- `OS/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts:90, 109`: error text "Missing Nx native host artifact" and "Missing Nx MCP owner artifact".
- `OS/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/📜️script.ts:9`, `…/⚙️browser-build/🟦️.ts:35`, `…/🌐️server/📜️script.ts:12`, `…/🌐️server/🟦️.ts:36`: comments about "Nx prepares", "Nx-completed", "Nx activation".
- `OS/🏃️run/🏗️bootstrap/🦀️.rs:120`: "produced by the one `plugin-registry:generate` nx target".
- `OS/📡️spr/🧪️tests/⏱️protocol-benchmark/🦀️.rs:7`: doc says `nx run @protocol/testkit-rs:bench`.
- `FW/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:105`: generated stub text "run the matching nx wasm target".
- `FW/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts:17`: "across Nx processes".
- `FW/🖱️ui/🧬️contract/🧬️schema/🟦️projection/🦀️.rs:1221`: generated header "@generated by `bun nx run @semio-tech/ui-contract-rs:generate`". Fix the generator, not the output.
- `FW/🧊️3d/🧪️tests/🎚️config/🟦️.ts:26`: comment "`nx test`".
- 26 plugin Rust `📦️packages/🦀️rust/📜️script.ts` files throw "package writes the Nx-owned deliverable and accepts no output override" (`PL/🌊️flow/…`, `PL/🏭️process/…`, `PL/📐️cad/…`, `PL/📖️playbook/…`, `PL/📜️imperative/…`, `PL/🏗️bim/…`).
- `PL/🔋️energy/🗿️artifacts/🔋️model/…/🧪️tests/🏛️export-epjson-runs-in-energyplus/🐍️.py:24, 80`: skip message "run `nx run @semio-tech/energy-oracle-py:oracle-setup`".
- `PL/🌀️procedural/…/📈️guest-memory/…` and other `nx.json` walk-up comments: see A2.

### C2. Docs and agent instructions

- `FW/🛍️products/🐾️pets/README.md:46`: "nx `@semio-tech/pets-staging-rs`".
- `PL/🎞️animate/AGENTS.md:37`, `PL/🏛️architect/AGENTS.md:21`, `PL/🔋️energy/AGENTS.md:22`: "via nx". Not edited, per the AGENTS.md rules.
- Root `AGENTS.md:7` says `package.json` MUST call `nx`. This contradicts the neutral goal and needs a decision from the user. Not edited.
- `README.md:883`: "Nx remains the default build tool". Accurate policy text, not stale. No action.
- `🌎️hub/Dockerfile:12, 17, 72`: "only the Nx launcher creates", "`nx:run-commands` wrappers". Comment-only, stale.

### C3. Config and tooling

- `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts:4`: comment "`bun nx run workspace:test-storybook`". The target exists as `repo:script` at root `📋️project.json:1998`, so only the prefix is stale.
- `.storybook/main.ts:143` and `tsconfig.json:33`: ignore globs `**/.nx/**`. The Nx cache lives under `.🧬semio/…` (`nx.json:92`), so `.nx` is stale.
- `package.json:145-146, 175, 244` and `🔒️dependencies.json:155, 167`: `@nx/devkit`, `@nx/js`, `nx`, and the `nx@23.2.0` patch. No code imports `@nx/*`, so the `@nx/*` packages may be unused. Verify before removing. `nx` itself is needed by the adapter.
- `.vscode/launch.json` and seed: 17 and 14 commands of the form `bun repo run <project>:test-nx-…`. Neutral CLI, Nx-named targets. Naming only.

### C4. Other

- `.cursor/plans/*.plan.md`: about 100 files and about 430 matching lines (for example `bun_nx_monorepo_setup_756984c2.plan.md`). Stale planning docs. Not counted in the A list.
- Ticket folders under `.🧬semio/` contain the B-ticket and T entries above.

## Verification Gaps

- Static, tracked-files-only search (`git grep`). Untracked files and `node_modules` were not covered.
- `workspace:dev` route resolution was not executed. Dashboard registry code was read, not run.
- The `--skip-nx-cache` flag definition was not located.
- The neutrality test (A2 item 7) is inferred stale from the fixture contents, not run.
