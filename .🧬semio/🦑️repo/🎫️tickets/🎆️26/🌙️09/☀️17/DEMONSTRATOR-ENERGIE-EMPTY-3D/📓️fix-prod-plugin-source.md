# Fix — deployed Energie empty World3d

## Symptom

On `https://v6.demonstrator.entwerfen.mit-bestand.de/#energie`, the `energy.model.3d` viewport stays empty (0 meshes / 0 instances). Console shows benign `no-operator-graph` for energy, plus **404** on:

- `/🔌️plugin-modules/watch`
- `/🧩️extension-modules/watch`

## Root cause

`FrameworkOsShell` (`🏛️ShellHost/🟦️.tsx`) always wired `pluginSource` to **dev-only SSE** watch URLs. Static `dist/site` ships wasm under `/🔌️plugin-modules` and `/🧩️extension-modules`, but **no watch stream**.

The primary plugin (`energy`) still booted via the dedicated boot effect, but **every other registry entry** (declared dependencies and contribution-matched plugins from `expandPluginRegistry`) only installs when `pluginSource.subscribe` delivers a connect-time `snapshot`. Without that snapshot, dependency contributors never load → document/example pipeline and 3D projection stay empty.

Same gap as generator ship (`DEMONSTRATOR-GENERATOR-SHIP-PLUGIN-BOOT/📓️root-cause-and-fix.md`).

## Fix

1. **`ShellHost`**: when `import.meta.env.PROD`, multiplex two `createBundledPluginSource` legs — plugin registry + `extensionRegistryFromCatalog(PLUGIN_CATALOG)` — instead of dev `EventSource` watch URLs.
2. **`createBundledPluginSource`**: snapshot events carry **no** `rebuiltAt`, so a replay does not hot-swap the session-owning plugin the boot effect already installed (`pluginAvailabilityRouteV1`).
3. **`extensionRegistryFromCatalog`**: shared registry builder for extensions (used by `createExtensionSource` and bundled hosts).

## Verify

1. `bun nx run @semio-tech/mit-bestand-demonstrator:build`
2. Serve `dist/site` locally or redeploy v6; open `/#energie`.
3. Expect `window:energy.model.3d` with **9 meshes / 9 instances** (`bestest-600` boot example).
4. Optional probe: `bun …/DEMONSTRATOR-ENERGIE-EMPTY-3D/🐍️deploy-energie-world-probe.mjs`

## Note

Live v6 must be **rebuilt and republished** after this change; code fix alone does not update the hosted bundle.
