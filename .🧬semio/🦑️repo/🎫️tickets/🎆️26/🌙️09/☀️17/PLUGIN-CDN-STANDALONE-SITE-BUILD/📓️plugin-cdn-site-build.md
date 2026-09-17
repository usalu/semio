# Plugin CDN Standalone Site Build

## Objective

Every plugin with a `[[package.metadata.semio.playground]]` row must publish a CDN-deployable static site under its own `dist/` tree via a single `build` command (e.g. `energy.semio-tech.com`, `3d.fem.semio-tech.com`).

## Mechanism

| Layer | Command / target | Output |
|-------|-------------------|--------|
| React ship bundle | `@semio-tech/framework-os-dev:build-<variant>-react-release` | Vite distribution (existing) |
| Plugin convenience | `@semio-tech/<plugin>:build` or `bun ./📜️script.ts build` | Same tree as react-release |
| Per-variant alias | `@semio-tech/<plugin>:build-<variant>-site` | Same |

### Default `distDir`

When Cargo.toml omits `distDir`:

- One playground per `pluginId` → `✏️s/🔌️plugins/<plugin>/dist`
- Several playgrounds per `pluginId` → `✏️s/🔌️plugins/<plugin>/dist/<variant>` (unless overridden, e.g. `fem3d` → shared `🏗️fem/dist`)

Resolved in registry generation (`generatePlaygroundRegistry`), Nx `playgroundPreparationTargets`, and `playgroundReactReleaseOutputPath`.

### Deploy surface

Sites inherit `semioEmojiIndexHtmlVitePlugin` (`index.html`, `404.html`) and favicon aliases from the shared Vite builder (see DEPLOYED-WEBSITES-OUT-OF-BOX).

## Examples

```bash
bun nx run @semio-tech/energy-plugin:build
bun nx run @semio-tech/fem-plugin:build-fem3d-site
bun nx run @semio-tech/framework-os-dev:build-energy-react-release
NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:build-all-playground-cdn-sites
```

`build-all-playground-cdn-sites` depends on every `build-<variant>-react-release` whose `distDir` lives under `✏️s/🔌️plugins/` (all 61 variants).

Dist roots (after build):

- `✏️s/🔌️plugins/🔋️energy/dist`
- `✏️s/🔌️plugins/🏗️fem/dist` (fem3d explicit `distDir`)

## Verification

- `testPlaygroundSiteDistDefaults` + `testPluginSiteNxTargets` + `assertCdnDeploySurface` in repo-lib cache-contracts suite
- `testRuntimeComponents` (same suite) asserts each `build-<variant>-react-release` outputs `{workspaceRoot}/<distDir>` and depends on `@semio-tech/assets:build`
- Full production build per plugin: run `build` and confirm deploy surface via `assertCdnDeploySurface(distRoot)`

### Energy smoke (2026-09-17)

`NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:build-energy-react-release` — **green** (~26m). Output: `✏️s/🔌️plugins/🔋️energy/dist` contains `index.html`, `404.html`, `favicon.ico`, `🧶️bundles/`, `🔌️plugin-modules/`.
