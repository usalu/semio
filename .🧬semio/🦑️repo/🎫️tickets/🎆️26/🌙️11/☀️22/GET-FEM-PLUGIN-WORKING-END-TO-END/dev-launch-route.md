# FEM 2D and FEM 3D dev launch route

Read-only research. Nothing was built, started, stopped or edited. Working tree: `C:\semio`.

## Summary

- Canonical serve targets: `@semio-tech/framework-os-dev:dev-fem2d-react-prototype` (port 6086) and `@semio-tech/framework-os-dev:dev-fem3d-react-prototype` (port 6087).
- Only 6086 is listening now, served by `bun.exe` PID 32260. Nothing listens on 6087.
- The published FEM component that 6086 serves carries provenance `dbb136d0…` (2026-11-20 21:52). The ticket records that this build still refuses startup with `Window owner must declare its original disposer birth`. A later component was linked on Nov 21, but canonical publication stopped at 11:02 UTC, so the 6086 server does not yet serve it.
- The dev targets are not in any committed `project.json`. They are inferred at graph time from playground rows declared in the FEM crate's `Cargo.toml`.

## 1. Ticket commands and ports (codex ticket GET-DEV-FEM-2D-WORKING-END-TO-END)

Ticket folder: `C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️20\GET-DEV-FEM-2D-WORKING-END-TO-END\`

Canonical entry (from `📓️dev-launch.md`):

```
repo run @semio-tech/framework-os-dev:dev-fem2d-react-prototype
```

Run from `C:\semio`. The ticket notes say the sealed graph on this host has no admission for this serve task, so the canonical route can stall in graph inventory (a dry run with `--graph-policy=current` took more than 6 minutes and was cancelled).

Ticket-local fallback, `launch-native.ts`:

- Signature: `launch-native.ts <cwd> <project:target> <script args...>`. `<cwd>` is relative to the workspace root (`C:\semio`, resolved as seven levels up from the script). The args after the selection are passed verbatim to `repo`, so the `run` subcommand must be repeated.
- It sets `SEMIO_BUILD_MODE=prototype`, `SEMIO_PLUGIN=fem2d`, `SEMIO_RENDERER=react`, and `SEMIO_PLUGIN_CATALOG=C:\semio\✏️s\🧑‍💻dev\🧩️catalog\🤖️generated`.
- Example (form inferred from the script; not run here): `<bun> launch-native.ts . @semio-tech/framework-os-dev:dev-fem2d-react-prototype run @semio-tech/framework-os-dev:dev-fem2d-react-prototype`

Ports used in the ticket:

| Port | Use |
|---|---|
| 6086 | FEM 2D React prototype dev server (canonical) |
| 6087 | FEM 3D React prototype dev server |
| 16086 | Temporary owned FEM 2D server for Playwright, with HMR off |
| 6095 | Instrumented preview of a linked FEM 2D guest |
| 6096 | Local FEM 3D linked preview (ticket 20 guest startup) |
| 6186 / 6187 | WGPU ports for FEM 2D / 3D (not used by this route) |

Serve environment (from `🚀️local-hub/🏃️execution/🟦️.ts`, `devServeCommandV1`): `SEMIO_PLUGIN=<variant>`, `SEMIO_RENDERER=react`, `SEMIO_VITE_HMR=0`, `S_OS_PORT=<port>`. The Vite config reads `S_OS_PORT`, so the port is set there and not only by the playground row.

Browser verification commands in the ticket:

- `🧪️browser-smoke.mjs` (Playwright; default URL `http://127.0.0.1:6086/?plugin=fem2d`). Flags: `--probe-paint`, `--probe-edit`, `--probe-results`, `--probe-initial-document`, `--probe-wire`, `--probe-defer-analysis`, `--probe-defer-canvases`, `--probe-defer-results`. Startup-isolation flags require the 6095 preview origin.
- `🧪️browser-guest-preview.mjs` (serves a linked guest on 6095/6096).
- Ready and error markers: `data-semio-os-ready=fem2d`, `data-semio-os-error=fem2d`.
- Native gates (Windows, run through `repo`): `editor_boot` passed on Nov 21, including a `patchNode` round trip that set `n1.x = 1.5`. Native session suite passed 19/19. Mounted results and editor media suites were green after the Nov 21 fixes.

The exact `repo cargo` and test-target invocations for the native suites are in `📓️verification.md` and `📓️mounted-session-tests.md`. They were not re-run.

## 2. Where `dev-fem2d-react-prototype` is defined

It is not in any committed `project.json`. It is generated at graph time:

1. Crate metadata. `🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml` declares two `[[package.metadata.semio.playground]]` rows:
   - `variant = "fem2d"`, `aliases = ["fem 2d"]`, `app = "s.fem.fem2d@1/*#editor"`, `ports = { react = 6086, wgpu = 6186 }`.
   - `variant = "fem3d"`, `aliases = ["fem 3d"]`, `app = "s.fem.fem3d@1/*#editor"`, `ports = { react = 6087, wgpu = 6187 }`, `distDir = "✏️s/🔌️plugins/🏗️fem/dist"`.
   - Both use `devContribution = "✏️s/🧑‍💻dev/🎭️variants/🏗️fem/🧬️schema/🔣️.json"`.
2. Generated catalog. `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎠️playgrounds.json` (and the identical-size `🚀️playgrounds.json`) holds the flattened rows. `🔌️plugins.json` holds the plugin row (`pluginId: fem`, `directoryName: 🏗️fem`, `wasmOut: semio_hub_fem_component.wasm`).
3. Target inference. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs` (about lines 202–262) emits, per row and profile: `session-<variant>`, `prepare-<variant>-react-<profile>`, `activate-<variant>-react-<profile>`, `serve-<variant>-react-<profile>` (runs `repo ./📜️script.ts serve <variant> react <profile>`), and `dev-<variant>-react-<profile>` (continuous, depends on `activate-…`). The same file also emits the wgpu and native targets.
4. Owner project. Targets belong to `@semio-tech/framework-os-dev`, whose `project.json` is `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`. The only literal react-prototype target there is `activate-s-react-prototype`.

The alias `fem 2d` is not part of the target name. The target uses the variant `fem2d`.

FEM 3D: yes, the same mechanism. The target is `@semio-tech/framework-os-dev:dev-fem3d-react-prototype`, port 6087 (wgpu 6187). The alias is `fem 3d`.

## 3. Listeners on 6086 and 6087

Command: `netstat -ano | findstr LISTENING | findstr 608`

```
TCP    127.0.0.1:6086         0.0.0.0:0              LISTENING       32260
```

- 6087 is not listening.
- PID 32260 is `bun.exe`, running:
  `C:\semio\.cache\b\d\m\j\420f71600f321d023284\node_modules\vite\bin\vite.js --mode prototype --config C:\semio\✏️s\🧑‍💻dev\🎭️variants\🏗️fem\🏗️builder\🌐️vite\🟦️.ts --configLoader native --host 127.0.0.1 --port 6086 --strictPort --clearScreen false`
- The ticket says this listener was restarted on Nov 21 with the patched Vite optimizer configuration, after which the shell and transformed FEM 2D entry returned HTTP 200. Its HMR setting is not visible from the command line. It was not probed.

## 4. Shared FEM component build and publication

Source:

- Hub composition: `🌎️hub/🧩️compositions/🏗️fem/`.
- Component crate (cdylib, the only wasm): `🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/`. Package `semio-hub-fem-component`, component package `semio:fem`, entry `🌎️hub/🧩️compositions/🏗️fem/🧩️component/🦀️.rs`. It depends on `semio-hub-fem` at `🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust`.
- Tests: `🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust/📜️script.ts test` (target `test` in that folder's `📋️project.json`, project `@semio-tech/fem-plugin`).
- Variant builder: `✏️s/🧑‍💻dev/🎭️variants/🏗️fem/🏗️builder/🌐️vite/🟦️.ts` and entry `✏️s/🧑‍💻dev/🎭️variants/🏗️fem/🚀️entry/🟦️.ts`.

Rebuild path:

- The component is produced by the `prepare-fem2d-react-prototype` dependency chain. Its inputs include `@semio-tech/framework-plugin-web:support`, `semio-os-infinite:fonts`, the boot components, and the session target. The component's own wasm target is `<project>:wasm`. Its project name is probably `semio-hub-fem-component`, but this was not verified.
- The wasm target is `wasm32-wasip2` with nightly `rustc 1.99.0`.
- Notes: the Nov 21 build reached a Cargo lock-selection problem, and the canonical publication was blocked by the descriptor emitter rejecting `--prepare-actor`. The ticket's workaround is the descriptor owner build through the ticket transport.

Published location (current):

- Component modules: `C:\semio\.cache\b\p\m\browser\prototype\modules\🏗️fem\`
  - `semio_hub_fem_component.core.wasm` (176,331,961 bytes, dated 2026-11-20 21:52)
  - `semio_hub_fem_component.js`, `semio_hub_fem_component.d.ts`, `🌉️bridge.js`, `🔣️.json`, `🛂️.descriptor.semio`, `🟨️.js`
  - `🧾️receipt.json`: `wasmSha256 2062ccd3…`, `semanticDescriptorSha256 bfa8de99…`, `provenanceSha256 dbb136d0…`, `sourceTreeDigest 2fb6caaca…`
- Activation: `C:\semio\.cache\b\p\m\browser\prototype\runtime\react\fem2d\activation\🔣️receipt.json` (`variant fem2d`, `artifactSha256 1b1185fa…`, `rebuiltAt 1795207967082`, which is 2026-11-20 21:52 UTC).
- A `runtime\react\fem3d` directory also exists (dated 2026-11-19).
- The ticket's Nov 21 links are only in preview output (`15eea2dc…` at 10:54 UTC, and `8b95b5bb…` for the Nov 20 21:31 guest). They are not the canonical publication.

## Open issues for the edit to calculate to result loop

- The 6086 server serves the Nov 20 receipt `dbb136d0…`, which the ticket says still reproduces the disposer startup refusal. The loop cannot be verified against the current server until a newer component is published.
- Known blockers recorded in the ticket: `setActiveExample` refused at stage 14 (store replacement) with zero copied items; the mounted analysis job reports running and then failed with an empty payload; a `noteShellCommand` note stalls intermittently after panel toggles.
- What is verified: the native `editor_boot` gate covers an inspector edit that changes the stored model and gets a new revision.
