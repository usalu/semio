# R2 Slice L-1: Launch File Removal (code and tests)

Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT` · 2026-10-08 · executor L-1. Tests were run on Windows with bun 1.4.2; `PYTHONIOENCODING=utf8` was set where Python children print emoji paths.

## Result

`.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` and `.claude/launch.json` are deleted. The final tracked-file grep for `launch\.json|launch\.seed` outside `.🧬semio` and `.cursor/plans` returns only:

- `AGENTS.md:50` (owner action),
- `🎛️dashboard/README.md:22` (slice L-2),
- `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json:154,155` (deletion evidence, kept on purpose).

No code, test, config or fixture consumer remains. `node-terminal`, `serverReadyAction`, `devLaunchers`, `projectLaunchers` have no tracked hits outside `.🧬semio`, `.cursor` and docs.

## Per item

| Item | Change | Test and result |
| --- | --- | --- |
| A: root `📜️script.ts` | Deleted the launch constants, `INTERACTIVITY_ALL_APP_LAUNCH_*`, `…PlaygroundLaunchNames`, `…LaunchCoverageFailures`, `…LaunchesFromSource`, the `launches` / `launchOnlyProducts` report fields and their exports. Added `interactivityAllAppGateFailures` (the seven `INTERACTIVITY_ALL_APP_REQUIRED_GATES` are now `workspace:verify` with free arguments: the root `📋️project.json` target `verify` must forward all args to `📜️script.ts verify` and declare exactly one positional text parameter in `metadata.semio.dashboard`; the schema `🧬️schema/🎮️registry/🔣️.json` must offer `text` and `valuePositional`), `interactivityAllAppPlaygroundsFromSource` (reads the generated `🚀️playgrounds.json`, the file the Rust registry reads), `interactivityAllAppRenderersFromSource` (reads the `RENDERERS` axis out of `🎮️registry/🦀️.rs`, so the renderer list has one owner), `interactivityAllAppPlaygroundSurfaces` and `interactivityAllAppSurfaceCoverageFailures` (per app context: a playground of the owning plugin whose React port and WGPU port both exist, so React, Wasm and native all start). Descriptor walk now covers `✏️s/🔌️plugins` and `🌎️hub/🧩️compositions` (plugin descriptors moved there; before the walk found 0 apps). Comments at former lines 137, 144, 6707 reworded. | `bun 📜️script.ts verify interactivity apps`: `descriptors=69 apps=147 playgrounds=155 surfaces=465 surfaceCoveredApps=147 surfaceMissingApps=0 gates=7 failures=46 selfTests=45`. The 46 failures are all non-launch (see Open issues); before this slice the same command reported 43 failures with `apps=0`. |
| B2, B3 | Deleted `🎛️dashboard/🧫️fixtures/🚀️launch-configurations/` (JSONC vector and feature). The still-valid scenarios (verb filing, published project graph, graph rebuild) moved into the new `🧫️fixtures/🎮️registry/🥒️.feature`, reworded for declared commands (tool, compound with ordered members, required parameter refused, schema problem reported, valid commands stay runnable). Claims checked against `🎮️registry/🦀️.rs` (`required` refusal at ~1327, `problems` at ~947). | feature file: no runner (Gherkin adapter is slice A-1 / V-1). |
| B4 | `🧪️tests/🌀️control-plane/🟦️.ts`: replaced the `jsonc-parser` launch oracle by an Ajv 2020 oracle that validates every `📋️project.json` and `🎮️commands.json` of `🧫️fixtures/🎮️registry/🏗️workspace.json` and the real workspace root `📋️project.json` against `#/$defs/ProjectManifest` / `TicketCommands`, plus two hostile declarations that must be rejected. | `bun test ./🟦️.ts` in that folder: 10 pass, 0 fail. |
| B6 | `…/🚀️runtime-bootstrap/🟦️.ts` and its fixture: `forwardedLaunchRows` replaced by `forwardedCommands`. Each forwarded port is read from a dashboard declaration (`os-hub` target `dev` ready port, root `dev-storybook` ready port, tool `mcp-inspector-os` ready port) or from the generated playground catalog (`s` react/wgpu and user slots), cross-checked with lodash `get`/`find`, and the label in `portsAttributes` must equal the command label. `devcontainer.json` unchanged (values 8787 6070 6072 6073 6066 6067 6068 6010 6274 6277). 6277 (MCP Inspector proxy, the inspector's own default) is declared nowhere, so it stays as `forwardedUndeclaredPorts` in the fixture. | `testContainerRuntimeBootstrap("C:/git/semio")` run through a throwaway script: PASS. |
| B7 | `📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts` rewritten with a counting `expect`: descriptor cases kept, launch cases replaced by gate, renderer axis, catalog, surface coverage cases with TypeScript parser oracles. | runs inside `verify interactivity apps`: `selfTests=45` (was 29). |
| B8 | `📦️package-boundary-classification` test: removed the launch-file loop and the unused `command`. | 119 pass, 1 fail (`go-bootstrap-delegation`, a 30 s timeout unrelated to launch). |
| B9, B10 | `🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts`: dropped the two launch inputs and the launch-name assertions; fixture keys `launchName`, `launchCommand` removed. | `bun ./📜️script.ts test command-composition-source` in `🧪️test`: 11 pass, 0 fail. |
| B11 | NOT changed. The file is hash-pinned (`sha256 e821d683…`, 12355 bytes, `🔣️taxonomy.json:740` and the reviewed manifest). I edited it, saw the pin, restored it byte-exact (sha re-verified). Its `launch*` keys are historical content, not a launch.json dependent. | n/a |
| B12 | `👁️watch-policy.json`: `.vscode/launch.json` replaced by `.vscode/settings.json` (still under the unwatched `.vscode` segment). | matcher probe: true for slash and backslash forms. The vitest suite `🧹️config` is UNVERIFIED (bun cannot run it, no vitest config found). |
| B13 | `📓️print/🔮️oracles/🔣️.json`: removed the stale `📇️registry/🚀️launch.test.ts` entry. | JSON valid; no consumer test run (UNVERIFIED). |
| F: Nx inputs | Removed 6 lines in repo-lib `📋️project.json` and 2 in `🧪️test/📋️project.json`. | JSON valid; registry-wide test above passes. |
| F: vite | Removed `**/.vscode/launch.json` from the ignored globs of `♻️mit-bestand/🧺️demonstrator/…/🌐️vite/🟦️.ts` and `🏢️semio-tech/🎡️play/…/🌐️vite/🟦️.ts`. | not run. |
| F: contract prose | Scalar contract `🔣️.json` and its `🧬️schema/🔣️.json`: launch-row sentence now says "started as a dashboard command". Dashboard `🧬️schema/🔣️.json:15` description now says "declared tools". | JSON valid; typedwire vitest UNVERIFIED. |
| `.gitignore:591` | Removed `!.vscode/launch.json`. | n/a |
| Deletions | The three launch files removed last. | `git status` shows `D` for all three. |

## Open issues (not launch, not fixed)

1. `verify interactivity apps` still exits 1 with 46 failures in 10 descriptors: `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🔣️.json` duplicates apps instead of delegating, and the nine `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/*/🔣️.json` descriptors are `role: "plugin"` packages at extension coordinates (no `ExtensionBundle::new`, no `.extends`). This disagrees with the existing law ("extension descriptors must be role extension", also asserted by the self-test); the owner of the stdio composition must decide which side moves.
2. `📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts:61` references the undefined `manifestSchemaPath` (committed that way); `test readme-reviewed-fixture-inputs` fails before it reads any fixture.
3. `launchNamePrefix` remains in the (untracked) generated `🚀️playgrounds.json`; tracked generator code no longer emits it.

## Requests

- Slice A-2: B5 `🌀️daemon/🧪️tests/🔬️unit/🦀️.rs` no longer writes `.vscode/launch.json` (A-2 changed it meanwhile; the final grep has no hit). The test name still contains "launch configuration" (11 `launch` words remain in that file); reword if it should say declared command.
- Slice L-2: `🎛️dashboard/README.md:22` (and the fixture sentence near 128: the `🚀️launch-configurations` fixture is deleted, `🧫️fixtures/🎮️registry` replaces it).
- Owner: `AGENTS.md:50`; frozen-seal ledger rows 154/155 (evidence, kept); the hash-pinned B11 fixture.
