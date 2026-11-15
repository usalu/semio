# 🗺️ Exploration: mit-bestand demonstrator chain (2026-11-13)

## Chain
`repo run @semio-tech/mit-bestand-demonstrator:activate-prototype`, then `:serve` (Vite, `127.0.0.1:6029`).

Closure (dependencies first): `framework-plugin-web:support`, `workspace:deps-wasm-opt`, `workspace:deps-browser-wasm`, `os-plugin-describe-rs:build`, canvas font assets, `@semio-tech/assets:build`, `semio-os-infinite:fonts`, four wasm producers (surface, editor, node graph, puzzle), `plugin-registry:generate` and `session-<variant>` (x9), about 32 plugin `component` crates, `framework-os-dev:prepare-<variant>-react-prototype` (x9: aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen), `activate-<variant>-react-prototype` (energy, fem3d, generator), demonstrator `prepare-prototype`, `activate-prototype`.

Scripts cannot be started standalone: every `📜️script.ts` requires the build-issued `SEMIO_SCRIPT_PROCESS_INVOCATION` envelope, so the chain only runs through `repo run`.

## State found
- `.cache/b/p/m/browser/` did not exist: no lane output, no receipts, no `dist/♻️activation`.
- Dependencies: `@semio-tech/puzzle-2d`, `s-dev-catalog-ts`, `gis-gismap-js` are linked in the `♻️mit-bestand` scope (`♻️mit-bestand/node_modules`), not in the root scope. Resolution is fine after `repo dependencies ensure`.
- `wasm-pack`, `wasm-bindgen`, `trunk` are located on PATH only and are absent on this host; `wasm-opt` is provisioned by `workspace:deps-wasm-opt`.

## Wasm tools (checked, no blocker)
- `wasm-pack` 0.15.0, `wasm-bindgen` 0.2.129 and `trunk` 0.21.14 are cargo-installed into `.cache/b/r/m/cargo/home/bin`; `wasm-opt` 130 is under `.cache/b/t/m/binaryen/`. The wasm build prepends these to PATH, so their absence from the shell PATH is irrelevant.
- Rust targets `wasm32-unknown-unknown` and `wasm32-wasip2` are installed.
- The producers' `wasm` targets declare no deps target in `📋️project.json`, but graph inference adds `workspace:deps-wasm` to every `wasm` target (`🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs:150`), so a fresh cache is covered too.

## Pane boot contract (acceptance spec)
- Routes: `/` landing page, `/#<paneId>` for generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik.
- Ready: `[data-shell-id="<id>"]` carries `data-shell-ready` (failure: `data-shell-error`, `data-shell-not-found`); `html` and shell have `lang` `en` or `de`.
- Language: `localStorage["ui.chrome.locale"]`, else `navigator.languages`, else the picker `[data-slot="demonstrator-locale-picker"]`.
- Any `console.error` or page error fails a case, except the exact 404 resource message.

## Inconsistencies noted
- README lists a `complete` subcommand of `🔨️modules/🧩️runtime/📜️script.ts` that is not registered.
- Stale comment in `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml` about a moved `generator` row.
- `prepare` of the demonstrator reports a raw ENOENT for a missing `.semio-artifact.json` marker.

## Blocker 1: project graph planning
See `🕸️graph-slice-slowness.md`.
