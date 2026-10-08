# R2 Slice P-1: One Playground Resolver (the registry)

## Round 2c (newest): `testNativeRuntime` failure diagnosed - not ours

- Failing command (`🧊️native-runtime/🟦️.ts:114`): `node <repo>/node_modules/nx/dist/bin/nx.js run fixture:consume-<profile> --outputStyle=static` inside the test's own temp fixture workspace (its `nx.json` is `{useDaemonProcess:false, cacheDirectory}`, targets are `materialize/prepare/consume-<profile>` running the fixture's own `script.ts`). It resolves no `renderer-wgpu` target and never calls the bootstrap `resolveNxInvocation`, so the removed aliases cannot be on its path.
- Captured output of every failure: `Plugin Worker for ...
x\dist\src\plugins\project-jsonuild-nodes\project-json is exiting as it did not receive a load message within 10 seconds of connecting` -> `NX Failed to load 1 default Nx plugin(s) ... exited unexpectedly`, exit 1. That is Nx's fixed 10 s plugin-worker handshake timing out while the host is saturated (6+ cargo/rustc processes from other fleet slices were running; the very first failure happened while my own `cargo test` ran).
- Evidence it is load-dependent, not code: the same driver (`r2-p1-native-driver.ts`, my edited code, no other change between runs) passed to the end twice (`native-runtime ok`, including all `cycle`/`remove` republish rounds, manifest schema, Cargo.lock/reader build) and failed on 4 other runs (the first plus three more, all with the Plugin Worker message above, at line 119); one further run hit my own 50 s `timeout`. My re-expressed assertions (lines 38-46) execute before the spawn and passed in all runs.
- HEAD comparison: not executed (no git checkout/stash allowed); the evidence is that the failing step shares no path with any file this slice changed. Nothing to fix in our files; the 10 s handshake is Nx's. Re-run the test when the machine is idle.

---

## Round 2b: wgpu renderer aliases dissolved

- Removed Nx targets `serve`, `dev`, `native`, `native-release` of `@semio-tech/framework-renderer-wgpu` (all hard-coded to variant `s`, only meaningful through the bootstrap rewrite) and the package scripts `serve`, `dev`, `native`. `native-scale[-release]` and `native-build[-release]` stay (no variant).
- Bootstrap `resolveNxInvocation`: both branches deleted (`renderer-wgpu:dev|serve` via `SEMIO_PLUGIN`, `renderer-wgpu:native[-release]` incl. its `--scale` forwarding, the variant-from-argv and `--smoke` parsing). The `native-build -- --release` rewrite and the generated-target env derivation (`os-dev:<op>-<v>-<renderer>-<profile>`) stay.
- Tests: `🧊️native-runtime/🟦️.ts` now asserts that the generated `os-dev:{run,smoke}-<v>-native-<profile>` targets resolve to themselves with env `SEMIO_PLUGIN/SEMIO_RENDERER=wgpu/SEMIO_BUILD_MODE` and that the retired `renderer-wgpu:native` forms pass through unrewritten; the `--scale` and `native -- s --release` assertions in it and in `⚡️cache-contracts` are gone.
- Registry: `playground:s --param renderer=wgpu-wasm` -> `bun nx run @semio-tech/framework-os-dev:dev-s-wgpu-dev` (ready 6066); `renderer=wgpu-native` -> `run-s-native-dev` (no ready port, no server); `playground:cad renderer=wgpu-native` resolves. `@semio-tech/framework-renderer-wgpu:native` is now `unknown command`; `native-scale` resolves.
- `semio commands --check --root /c/git/semio`: **20402 commands, 0 problems**.
- `cargo test -p semio-framework-repo-dashboard --lib` (target-fleet-p1, private build dir): **180 passed, 0 failed**. No registry/inventory test needed a change: the `workspace:dev` references in `🎮️registry/…/🦀️.rs:59,327` and `📚️inventory/…/🦀️.rs:158` run against the fixture workspace `🧫️fixtures/🎮️registry/🏗️workspace.json`, which carries its own `dev` target.
- `testNativeRuntime` driven in isolation (`r2-p1-native-driver.ts`): my re-expressed assertions (lines 38-46) pass; the run later fails at `🧊️native-runtime/🟦️.ts:119` (a spawned child exits 1; later environment step, not touched by this slice) - UNVERIFIED beyond that point. `bun build --no-bundle` parses the bootstrap and both tests.

---

Date 2026-10-08. Every playground-variant resolution outside the Rust registry is gone. `semio run playground:<variant> [--param renderer=react|wgpu-wasm|wgpu-native]` is the only way to resolve a playground.

## Removed

| Where | What |
| --- | --- |
| root `📜️script.ts` | `ensureFrameworkOsPlaygroundCatalog`, `resolvePlaygroundDevApp`, `runFrameworkOsPlaygroundDev` (+ the `served` doc block), the `dev s` / `dev <variant>` / bare `dev` branches. `DevScript` keeps `storybook`, `storybook-static`, `mcp`; anything else exits 1 and names `bun run dashboard run playground:<variant>`. `dev mcp http os` removed (tool `os-mcp-http` now owns it); `dev mcp <unknown>` (was: silently the inspector) exits 1 |
| bootstrap `⚡️caching/🚀️bootstrap/📜️script.ts` | whole `workspace:dev` / `@semio-tech/framework-os-dev:dev` branch (mcp rewrite, storybook rewrite, `multi`, catalog + `served` + renderer/profile resolution) and `nxRoutingServices` |
| library `📚️library/🟦️.ts` | `frameworkOsPlaygroundDefaultPort`, `resolveFrameworkOsPlaygroundPlugin`, `frameworkOsPlaygroundDevEnv`, the `loadFrameworkOsPlaygroundSelections` re-export, the `PlaygroundVariant` type import; module `🎮️playground/🧭️selection/` deleted (authored-manifest scan, no caller left) |
| root `📋️project.json` | target `dev` (the `-- <variant>` forwarder), target `dev-mcp-engine` (an alias of `dev-mcp`, no caller) |
| os-dev `📦️packages/🟦️typescript` | Nx target `dev` (alias the bootstrap rewrote; the router never had a `dev` command) and the `dev` package script |
| tests | `⚡️cache-contracts/🟦️.ts`: selection-loader block, the `workspace:dev` mcp/storybook/variant/renderer assertions, the `workspace.targets.dev` policy lines; new assertions that `workspace:dev`, `workspace:dev -- mcp http os` and `os-dev:dev` pass through unrewritten, `os-dev` has no `dev` target, and the resolver source names no playground selection/catalog. `workspace-contract/🟦️.ts`: three tests (`resolveFrameworkOsPlaygroundPlugin`, two `frameworkOsPlaygroundDevEnv`) and their imports. Fixture `⚡️caching/🧫️fixtures/nx-contract/🔣️.json`: `playgroundSelections` vector |

## Kept, addressable through the registry

| Route | Registry command |
| --- | --- |
| `dev storybook [scope…]` | Nx targets `workspace:dev-storybook`, `dev-storybook-<scope>` (existing; `--help`-less, they start Storybook) |
| `dev storybook-static` | tool `workspace/storybook-static`, now `bun ./📜️script.ts dev storybook-static` (was `nx run workspace:dev -- storybook-static`); ready 6010 |
| `dev mcp` / `dev mcp repo` (inspector) | Nx targets `workspace:dev-mcp`, `workspace:dev-mcp-repo` (ready 6274, printed) |
| `dev mcp stdio [client-profile]`, `dev mcp stdio os …` | tools `workspace/repo-mcp`, `workspace/os-mcp-stdio`; also what `.mcp.json`, `.cursor/mcp.json`, `.vscode/mcp.json`, `.codex/config.toml` launch (C-1a owns those; unchanged) |
| Streamable HTTP of the os gateway | tool `workspace/os-mcp-http` now `bun nx run @semio-tech/framework-os-mcp-rs:dev -- http --port 6300`, `ready {port 6300}` (target verified in that project's `📋️project.json`, `dev` depends on `build`, script `dev` execs the binary with the forwarded args). The project graph cache in `.nx/workspace-data` is stale (old project names) so it was not used as evidence |

## Defect found and fixed

`🌎️hub/📦️packages/🦀️rust/📜️script.ts` spawned the os-dev script with `dev served` (`startGisMapShellPeerV1`, ~12257) and `dev s` (secure-suite, ~13277). The os-dev router has no `dev` command (the `dev` target was an alias that only the removed bootstrap branch understood), so both exited 1 with `unknown command`. They now run `serve s react dev`, the form the collaboration harness already uses (`dev-collaboration/🟦️.ts`).

## Docs

Root `README.md` (MCP table row; `workspace:dev` out of the root command list; "Only `dev` stays live" sentence; `dev s` rows -> `playground:s`), `🌉️mcp/README.md` (run block: http via the tool; `dev s` -> `playground:s`), runtime refusal text in `🌉️mcp/🖥️ui/🦀️.rs:86` now says `bun run dashboard run playground:s` (the test only matches "no shell is attached"). Historic `.cursor/plans/**` and ticket archives untouched. Remaining prose `dev s` in code comments denotes the running playground session and was left.

## Verification (what was run)

- Debug build into `target-fleet-p1` (private build dir): OK, 52 s.
- `semio commands --check --root /c/git/semio`: **20406 commands, 0 problems** (was 20409: `workspace:dev`, `workspace:dev-mcp-engine`, `os-dev:dev` gone).
- `semio run playground:<v> --dry-run`: s, dag, cad, draw, note, gis2d, generation3d, trinity-jack, animate resolve to `@semio-tech/framework-os-dev:dev-<v>-react-dev` with the catalog ports (6070, 6017, 6020, 6064, 6080, 6040, 6018, 6054, 6051); `s` with `renderer=wgpu-wasm` -> `dev-s-wgpu-dev`, `renderer=wgpu-native` -> `run-s-native-dev`; `cad renderer=react` ok. `playground:block` is not a variant (unknown command, expected).
- Dry runs: `tool:workspace/os-mcp-http`, `storybook-static`, `os-mcp-stdio`, `repo-mcp`, `workspace:dev-mcp`, `dev-mcp-repo`, `dev-storybook` resolve; `workspace:dev` is `unknown command`.
- `T/r2-p1-verify.ts` (bun): bootstrap passes the three retired forms through unrewritten; root and os-dev have no `dev` target; `bun ./📜️script.ts dev`, `dev s`, `dev mcp http os`, `dev mcp engine` each exit 1 with the new message. Servers were not started (`dev mcp stdio … --help` is not supported by the route: it would exec the MCP binary).
- `bun test …/workspace-contract/🟦️.ts -t "playground static sites"`: 4 pass. `bun build --no-bundle` parses all five edited TS sources.
- UNVERIFIED: the full cache-contracts suite. It aborts in `testCommandInputs` -> `testWasmToolFingerprint` (`🔏️tool-fingerprint/🟦️.ts:28`, `policy.toolchains.wasm.commands` path check on Windows) before it reaches the assertions I edited; the earlier failures I caused (`workspace.targets.dev`) were fixed and re-run past. Whole workspace-contract file and `windows-command-paths` also have failures unrelated to this slice (missing `🧬️schema/🔐️owner-only`, `bunx` in the hub script line 2674, process-table schema, mutation-authority).
- UNVERIFIED: Rust tests in `🎮️registry`/`📚️inventory` that reference `workspace:dev` (`registry/…/🦀️.rs:59,327`, `inventory/…/🦀️.rs:158`) and fixture `🏗️workspace.json:12`; the fixture workspace carries its own `dev` target, so they should be independent of the real root, but the A-1 owner should run `cargo test -p semio-framework-repo-dashboard --lib`.

## Left for others (flagged)

- (done in round 2b) the wgpu renderer aliases.
- `D/README.md:117` text about the retired `semio dev <variant>` is A-1's.
