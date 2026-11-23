# 🚀️ Sourcing Launch Path (2026-11)

Read-only map of how the sourcing curation editor (`s.sourcing.curation@1/*#editor`, variant `sourcing`, react 6081 / wgpu 6181) is built and served today.

## Dev contribution is required
- `ServeScript` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:52-65`) takes its contribution from `loadDevContribution`, which returns `undefined` without `devContribution`/`brand` (`🧩️contribution/📥️loading/🟦️.ts:18-21`).
- Fallback is the generic `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:228` with `/🚀️entry/🟦️.ts` and an empty inventory, which cannot boot sourcing. The generator (`🎮️playground-targets/🟨️.mjs:213-220`) uses the same fallback.
- Fixed this ticket: variant folder `✏️s/🧑‍💻dev/🎭️variants/🪵️sourcing/` and `devContribution` on the playground row in `🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/📦️packages/🦀️rust/Cargo.toml`. The generated `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` must be regenerated (`repo run @semio-tech/s-dev-composition:generate`).

## Serve (react, 6081)
- `repo run @semio-tech/framework-os-dev:serve-sourcing-react-prototype` (generated at `🎮️playground-targets/🟨️.mjs:244`, runs `repo ./📜️script.ts serve sourcing react prototype`).
- Managed owner: `repo ./📜️script.ts serve-hold --serve http://127.0.0.1:6081/ --variant sourcing` spawns `repo run @semio-tech/framework-os-dev:dev-sourcing-react-prototype` (`🚀️local-hub/🏃️execution/🟦️.ts:31`).
- Only `prototype` and `release` profiles are generated; the old `serve-sourcing-react-dev` target is gone.

## Plugin wasm reaching the browser
- Component `semio-hub-sourcing-component` (`semio:sourcing`, deployment dir `🪵️sourcing`).
- Built by `buildComponentClosureV1` (`🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`): `cargo build -p semio-hub-sourcing-component --lib --target wasm32-wasip2 --profile prototype`, then `semio-native-transpile` in `transpilePluginComponentModulesAsync` (`🌐️browser-bundle/🏗️materialization/🟦️.ts:43`).
- Staged to `.cache/b/p/m/browser/<mode>/modules/` (`♻️activation/📂️paths/🟦️.ts:7`), served at `/🔌️plugin-modules/` (`📦️deployment/🛣️routes.json:2`).
- On 2026-11-23 the staged prototype modules held only `wfc`, `shard`, `vendor`: no sourcing wasm was staged; activation builds it.

## Related tickets (Oct–Nov)
- `🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS`: sourcing wasm size audit (core wasm 152.5 MB) and module rebuild.
- `🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END`: curation document contract rows, sourcing Cargo.lock cleanup; stale descriptor channel excludes plugins from the catalog silently.
- `🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION`: curation standards in the roster.
- `🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/audit-launch.md`: sourcing extension describe routes.
