# Gates and Acceptance Prerequisites (read-only audit)

Date: 2026-11-22. Scope: read-only. No source edited, no builds, tests or servers run.

## 1. Declared targets per project

Each project.json was located by its `"name"` and parsed. All five names matched exactly.

| Project | project.json | Requested targets declared | Requested targets not declared |
|---|---|---|---|
| `@semio-tech/s-gis-services-native` | `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust/📋️project.json` | `native-build`, `mcp-build`, `user-path-check`, `security-check` | `native-codec-check`, `component-codec-check`, `gis-map-proposal-process-check`, `trusted-stdio-gis-bundle-process-check`, `trusted-stdio-gis-bundle-browser-check`, `typecheck`, `wasm` |
| `@semio-tech/gis-plugin` | `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/📋️project.json` | `native-codec-check` | `native-build`, `mcp-build`, `user-path-check`, `security-check`, `component-codec-check`, `gis-map-proposal-process-check`, `trusted-stdio-gis-bundle-process-check`, `trusted-stdio-gis-bundle-browser-check`, `typecheck`, `wasm` |
| `os-hub` | `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json` | `component-codec-check`, `gis-map-proposal-process-check`, `trusted-stdio-gis-bundle-process-check`, `trusted-stdio-gis-bundle-browser-check` | `native-build`, `mcp-build`, `user-path-check`, `security-check`, `native-codec-check`, `typecheck`, `wasm` |
| `@semio-tech/s-dev-composition` | `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/📋️project.json` | `typecheck` | `native-build`, `mcp-build`, `user-path-check`, `security-check`, `native-codec-check`, `component-codec-check`, `gis-map-proposal-process-check`, `trusted-stdio-gis-bundle-process-check`, `trusted-stdio-gis-bundle-browser-check`, `wasm` |
| `@semio-tech/framework-surface-rs` | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📋️project.json` | `wasm` | `native-build`, `mcp-build`, `user-path-check`, `security-check`, `native-codec-check`, `component-codec-check`, `gis-map-proposal-process-check`, `trusted-stdio-gis-bundle-process-check`, `trusted-stdio-gis-bundle-browser-check`, `typecheck` |

Caveats:
- The project.json files reference plugin schemas (for example `.../🧬️schema/📋️project/🔣️.json`). Those plugins may infer extra targets that are not written in the file. I did not run `repo show-targets` to confirm inferred targets, so an undeclared name here may still exist at run time.
- `@semio-tech/framework-surface-rs` declares `test`, `test-source`, `test-scene-wire-native`, `test-scene-wire-source` and `wasm`. It is the only project with `wasm`.
- An existing ticket command already uses the combined form `run-many --lang=en --parallel=2 --output=static --targets=native-build,mcp-build,user-path-check,security-check,native-codec-check,component-codec-check,gis-map-proposal-process-...`. It names targets across projects that do not declare all of them, so do not reuse it as-is.

Minimal commands. `run-many` applies its targets to every listed project, so each target group is paired with the project(s) that declare it:

```
repo run-many --lang=en --targets=native-build,mcp-build,user-path-check,security-check --projects=@semio-tech/s-gis-services-native
repo run-many --lang=en --targets=native-codec-check --projects=@semio-tech/gis-plugin
repo run-many --lang=en --targets=component-codec-check,gis-map-proposal-process-check,trusted-stdio-gis-bundle-process-check,trusted-stdio-gis-bundle-browser-check --projects=os-hub
repo run-many --lang=en --targets=typecheck --projects=@semio-tech/s-dev-composition
repo run-many --lang=en --targets=wasm --projects=@semio-tech/framework-surface-rs
```

## 2. Browser acceptance scripts

Both scripts are in `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/`.

Playwright resolution:
- Both scripts `import { chromium } from "playwright"`. The ticket folder sits inside the repo, so Node resolves this by walking up to the repo root.
- The root `package.json` declares `"playwright": "^1.57.0"` (line 284) and `"@playwright/test": "^1.57.0"` (line 226).
- `/Users/ueli/Documents/semio/node_modules/playwright` and `playwright-core` are present, so `playwright` is resolvable.

Chrome: `/Applications/Google Chrome.app` is installed. Both scripts launch with `channel: "chrome"`, so they use this install.

Commands:
```
repo tools exec --light bun -- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/verify-s-shell-terrain.mjs http://127.0.0.1:6070/
repo tools exec --light bun -- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/verify-terrain-maximum-grid-browser.mjs "http://127.0.0.1:6083/?example=demo"
```

What each needs running:
- `verify-s-shell-terrain.mjs`: the full s shell at `http://127.0.0.1:6070/` (default URL). It first fetches the landing URL and fails with "Full s shell unavailable" if the shell is down. The URL may be `/` or `/spaces/<id>` or `/spaces/<id>/studio`, with no query or hash. It uses the broker path `/_semio/dev/local-session`. The scripts do not reference 8787 directly. Port 8787 is the dev local-hub default (`DEV_LOCAL_HUB_DEFAULT_URL = "http://127.0.0.1:8787"` in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`), so the shell is expected to need it. This dependency is inferred, not confirmed by a run.
- `verify-terrain-maximum-grid-browser.mjs`: the gis3d react playground at `http://127.0.0.1:6083/?example=demo` (default URL). It launches with `--use-angle=swiftshader --enable-unsafe-swiftshader`. It does not use 6070 or 8787.
- Port 6183 (gis3d wgpu) is not used by either script.

## 3. Target names and ports for gis2d and gis3d

Port table: `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts`. It is marked `@generated by framework/plugin/registry/script.ts`.

| Variant | Renderer key | Port | Target |
|---|---|---|---|
| `gis2d` | react | 6040 | `dev-gis2d-react-prototype` |
| `gis2d` | wgpu | 6140 | `dev-gis2d-wgpu-prototype` |
| `gis3d` | react | 6083 | `dev-gis3d-react-prototype` |
| `gis3d` | wgpu | 6183 | `dev-gis3d-wgpu-prototype` |

Generator chain:
- The port table is written by the projection module `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts` (around lines 141-144). It emits `ports: { react, wgpu }` per variant.
- Target names are built as `@semio-tech/framework-os-dev:dev-${variant}-${renderer}-${profile}` in `.../🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts:470`. The `renderer` value is `react` or `wgpu`, and the profile is `prototype`.
- The dashboard resolver reads the same renderer-to-target mapping: `react` gives `dev-<variant>-react-prototype`, and `wgpu-native` gives `run-<variant>-native-prototype`. That file is under `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/`, and the matching lines are in its test `🧪️tests/🎮️registry/🟦️.ts:350`.
- The `dev-*` targets are not written literally in `@semio-tech/framework-os-dev`'s project.json. The grep found only the cold-boot target `activate-s-react-prototype`. The `dev-<variant>-<renderer>-prototype` names appear to be composed at run time from the registry. I did not confirm that by running `repo show-targets`.

The `os-hub` project's `dev` target also declares a ready port of 8787 (`OS_HUB_PORT`, path `/admin`). That is a separate local hub, not the gis playgrounds.

Commands for the four ports, if needed:
```
repo run @semio-tech/framework-os-dev:dev-gis2d-react-prototype
repo run @semio-tech/framework-os-dev:dev-gis2d-wgpu-prototype
repo run @semio-tech/framework-os-dev:dev-gis3d-react-prototype
repo run @semio-tech/framework-os-dev:dev-gis3d-wgpu-prototype
```

Remaining gaps: no run was performed, so inferred targets, generated names and the 8787 dependency of the s shell are unconfirmed.
