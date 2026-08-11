# W2 — Catalog Injection (invert kernel → OS-product generated-registry import)

## Goal
The generic `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` module imported plugin/playground
catalog types directly from the OS product's generated build output
(`💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️plugins.ts` +
`🟦️playgrounds.ts`). Inverted via catalog injection: kernel now takes a `PluginCatalog` as the first
parameter of every resolver instead of reading a module-level import.

## What changed

### 1. Kernel (`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`)
- Deleted the upward imports (`PLAYGROUND_BUILD_TARGETS`/`PlaygroundBuildTarget`,
  `PLUGIN_BUILD_TARGETS`/`PLUGIN_HOST_CONFIGS`/`EXTENSION_TARGETS`/`pluginModuleUrl`/`extensionModuleUrl`)
  from the OS product's generated registry.
- Added a new `🗂️PluginCatalog` region (placed right after `PluginRegistryEntry`) defining:
  - `PluginCatalogTarget` — mirrors a generated `PluginBuildTarget` row: `{ pluginId, wasmOut, role,
    contributes, consumes }`.
  - `PlaygroundCatalogTarget` — mirrors a generated `PlaygroundBuildTarget` row, only the columns the
    kernel resolvers read: `{ variant, pluginId, app?, aliases }`.
  - `PluginCatalog` interface: `{ plugins: readonly PluginCatalogTarget[]; extensions: readonly
    PluginCatalogTarget[]; hosts: readonly PluginHostConfig[]; playgrounds: readonly
    PlaygroundCatalogTarget[]; moduleUrl(pluginId, wasmOut): string; extensionModuleUrl(pluginId,
    wasmOut): string }`.
- Re-signed every resolver to take `catalog: PluginCatalog` as its first parameter:
  - `findPlaygroundVariant(catalog, playgroundPluginId)` (module-private)
  - `resolvePluginRegistryId(catalog, playgroundPluginId)`
  - `resolvePlaygroundDefaultAppId(catalog, playgroundPluginId)`
  - `resolvePlaygroundBoot(catalog, variant, session?)`
  - `resolvePluginHostConfig(catalog, playgroundPluginId)`
  - `createExtensionSource(catalog)`
- `expandPluginRegistry(plugins, primaryPluginId?, hostMode?)` left untouched — already pure/plugins-as-param, per task instructions.
- `PluginHostConfig` type (bottom of file, `🏠️🧳️PluginHostConfig` region) unchanged — its shape already
  matches the generated `PluginHostConfig`/`PluginHostMetadata` row 1:1, so `catalog.hosts` slots in
  directly.

### 2. New file — the ONE place allowed to import the generated registry on the kernel's behalf
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️catalog.ts` (new taxonomy
leaf, sits next to the existing `📇️registry/🤖️generated/` folder in the same package):
- Imports `PLUGIN_BUILD_TARGETS`, `EXTENSION_TARGETS`, `PLUGIN_HOST_CONFIGS`, `pluginModuleUrl`,
  `extensionModuleUrl` from `📇️registry/🤖️generated/🟦️plugins.ts` and `PLAYGROUND_BUILD_TARGETS` from
  `🤖️generated/🟦️playgrounds.ts`.
- Exports `buildPluginCatalog(): PluginCatalog` and a ready-built singleton `PLUGIN_CATALOG:
  PluginCatalog` (both, per task's "your judgment" note — most call sites use the constant, the
  function is there for any caller that wants a fresh build).

### 3. Manifest (`🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`)
- Deleted the same upward import at lines 4-5. It was **dead** — grepped the whole file, nothing else
  referenced `PLAYGROUND_BUILD_TARGETS`/`PlaygroundBuildTarget`/`PLUGIN_BUILD_TARGETS`/
  `PLUGIN_HOST_CONFIGS`/`EXTENSION_TARGETS`/`pluginModuleUrl`/`extensionModuleUrl` anywhere in this
  1067-line file. Pure deletion, no re-signing needed here.

### 4. Callers repointed to pass the catalog
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🟦️component.ts` (boot entry): imports `PLUGIN_CATALOG`
  from `../🔌️plugin/📦️packages/🟦️typescript/🟦️catalog.ts`; `resolvePlaygroundBoot(PLUGIN_CATALOG, …)`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🧩️multi.tsx` (multi-shell harness entry): same import;
  `resolvePlaygroundBoot(PLUGIN_CATALOG, pane.variant)`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts`
  (wgpu boot): same import; `resolvePlaygroundBoot(PLUGIN_CATALOG, PLAYGROUND_SESSION.variant, PLAYGROUND_SESSION)`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`:
  imports `PLUGIN_CATALOG`; repointed all 5 call sites — `resolvePluginHostConfig`,
  `resolvePluginRegistryId` (×2, one inside `expandPluginRegistry`'s `primaryPluginId` arg + one for
  `primaryPluginId` memo), `createExtensionSource`, `resolvePlaygroundDefaultAppId`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx`:
  added `catalog: PluginCatalog` param to the module-private `isStudioMode(catalog, pluginFilter?)`
  helper and its `resolvePluginHostConfig(catalog, pluginFilter)` call. **Note**: grepped the whole
  renderer engine tree — `isStudioMode` has NO callers anywhere (dead code, predates this ticket). Left
  it re-signed for consistency/future use rather than deleting it (out of scope — not asked to remove
  dead code, only to re-sign resolvers).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`:
  **no code change needed**. `createExtensionSource`, `expandPluginRegistry`,
  `resolvePlaygroundDefaultAppId`, `resolvePluginHostConfig`, `resolvePluginRegistryId` are imported
  from `@semio-tech/framework` at the top of this file but grepped for every bare-word occurrence —
  none are actually called or re-exported anywhere in the file. They're pre-existing dead imports;
  re-signing the kernel functions doesn't break an unused import, so left as-is.

### 5. Framework tests (`🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`)
- Added `type PluginCatalog` to the kernel import block.
- Replaced the `describe("PlaygroundResolution", …)` block's 3 tests, which asserted against real OS
  product plugin ids (`"s"`, `"puzzle3d"`, `"aggregator"`, `"3d"`, `"sourcing"`) with a local
  `SYNTHETIC_PLUGIN_CATALOG: PluginCatalog` fixture (`alpha`/`beta`/`beta-extension-gamma`, `beta-play`
  variant with aliases `["b", "beta play"]`) and rewrote the assertions against it.
- Rewrote the `createExtensionSource()` call inside the `"multiplexPluginSources() merges list() …"`
  test (in the `PluginSource` describe block) to build its own tiny inline `PluginCatalog` fixture and
  pass it to `createExtensionSource(catalog)`.
  - **Found + fixed a pre-existing bug in passing**: that test referenced a bare `EXTENSION_TARGETS`
    identifier with **no import anywhere in the file** — `export * from kernel` doesn't re-export a
    plain `import`, so this was already an unresolved-reference compile error before this ticket
    touched the file. Now replaced with `catalog.extensions` off the local fixture, so it's gone.

## Final `PluginCatalog` shape
```ts
export type PluginCatalogTarget = {
  readonly pluginId: string;
  readonly wasmOut: string;
  readonly role: "plugin" | "extension";
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
};

export type PlaygroundCatalogTarget = {
  readonly variant: string;
  readonly pluginId: string;
  readonly app?: string;
  readonly aliases: readonly string[];
};

export interface PluginCatalog {
  readonly plugins: readonly PluginCatalogTarget[];
  readonly extensions: readonly PluginCatalogTarget[];
  readonly hosts: readonly PluginHostConfig[];
  readonly playgrounds: readonly PlaygroundCatalogTarget[];
  moduleUrl(pluginId: string, wasmOut: string): string;
  extensionModuleUrl(pluginId: string, wasmOut: string): string;
}
```
(`PluginHostConfig` unchanged, defined further down in kernel's `🏠️🧳️PluginHostConfig` region:
`{ pluginId, landingAppId, hostAppId }` — identical shape to the generated row, assigned straight
through in `🟦️catalog.ts`.)

## BLOCKING — out-of-ownership caller not updated
`♻️mit-bestand/🧺️demonstrator/📦️index.tsx:407` calls `resolvePlaygroundBoot(bootVariant)` — the OLD
single-arg signature. This file is **not** in my file ownership list for this wave, so I did not touch
it. After this change it will fail to typecheck (`Expected 2-3 arguments, but got 1`) until a follow-up
either repoints it to pass a catalog (likely this demonstrator's own OS-product catalog, or
`PLUGIN_CATALOG` from the new `🟦️catalog.ts` if it depends on the same OS product) or someone with
ownership of `♻️mit-bestand/🧺️demonstrator/` fixes it. `🟦️brand.ts:747` only mentions
`resolvePlaygroundBoot(variant)` in a comment — no code change needed there, but worth updating the
comment once the real call site is fixed.

## Verification
No dedicated `typecheck`/`check` nx target exists for `@semio-tech/framework` or
`@semio-tech/framework-os-dev` (checked both `project.json`s — only
`test`/`test-quick`/`test-long`/`test-exhaustive`/`build`/`dev`/`verify`/`plugin`/`parity`, none of
which run `tsc`), nor for the react-renderer/wgpu-target packages (`test`/`test-quick`/…/`lint` only).

Ran `bunx tsc --noEmit -p tsconfig.json` (repo root, no `paths` override — `@semio-tech/*` resolves
through the bun-workspace `node_modules` symlinks) as the closest equivalent, both with and without
`--incremental false`. **Found this invocation unreliable for this repo**: it silently stops emitting
semantic diagnostics partway through the ~10k-file program (confirmed via `--listFilesOnly`, which
shows the full set including every file I touched) and exits without ever printing a `Found N errors`
summary — 19 lines of pure syntax errors from three unrelated WIP files, then nothing, even though a
scoped check of the same files (below) finds hundreds more real diagnostics including files later in
that same run. This looks like tsc crashing/bailing quietly on this codebase's scale+emoji-path mix
rather than a real "0 errors" result, so I did not trust it and did not use it as the verification gate.

Instead built a scoped tsconfig (`extends` the root one) with `"include"` set to just the 9 files I
directly edited (kernel, catalog.ts, manifest, os-dev component.ts, multi.tsx, ShellHost, ShellHelpers,
wgpu boot.ts, react index.tsx, glue.ts) and ran `tsc --noEmit --incremental false
--allowImportingTsExtensions true` against it (the last flag suppresses a repo-wide, pre-existing
TS5097 noise category — every `import … from "./x.ts"` explicit-extension import in this codebase,
which `bun`/`vite` accept natively but bare `tsc` rejects without that flag; unrelated to this ticket).
tsc still pulls in and diagnoses the full transitive dependency graph from those 9 roots (over 1000
diagnostic lines), which is the real signal:

- **Zero errors in the new `🟦️catalog.ts`** — no lines at all reported against it.
- **Zero errors at or near any of the lines I edited** in `🎠️kernel/🟦️component.ts`, `🛂️manifest/🟦️component.ts`,
  `ShellHost/🟦️component.tsx`, `ShellHelpers/🟦️component.tsx`, `🧑️‍💻️dev/🟦️component.ts`, `🧩️multi.tsx`,
  `wgpu/🟦️typescript/🟦️boot.ts`, or `glue.ts`.
- **Zero `TS2554`/`TS2555`/`TS2556`/`TS2557` (argument-count) errors anywhere** tied to
  `resolvePluginRegistryId`/`resolvePlaygroundDefaultAppId`/`resolvePluginHostConfig`/
  `resolvePlaygroundBoot`/`createExtensionSource` — the one arg-count hit in the whole scoped run is
  unrelated (`TextEditor/🟦️component.tsx(454,43)`, not a resolver call, not a file I touched) — **except**
  the one confirmed, expected, out-of-ownership regression below.
- Every other diagnostic in the scoped run is pre-existing debt unconnected to this ticket's edit:
  repo-wide latent "used without local import, only ever re-exported via `export *`" gaps that predate
  this ticket (e.g. `glue.ts` itself uses `PluginRegistryEntry`/`PluginSourceEvent`/`ephemeralBox` in its
  own vitest block with no explicit import — `export * from kernel` doesn't bind those names into
  `glue.ts`'s own scope; `🛂️manifest/🟦️component.ts` uses `UiMenuRef` at 10+ call sites with no import,
  that type lives in `🔺️mesh/🟦️component.ts`), a pre-existing `kernel/🟦️component.ts(578)`
  `instanceof`/`ArrayBuffer` looseness in the unrelated `PluginWorkerClient.attachWorker` message
  handler, `documentJson`/`TutorialTracks` "document"-field mismatches in `ShellHost`/mit-bestand's
  `🟦️brand.ts` matching the concurrent cross-session refactor I was told to ignore, and a handful of
  other unrelated `WindowKindDefinition`/`FrameworkSyncUtilityLeaf`/`BlobPart`/`LocalizedLabel`/
  `ImportMeta.env` type mismatches in code this ticket never touches. None of these are new — they exist
  independent of whether the `PluginCatalog` injection lands, and none are actionable within this
  ticket's ownership boundary. This absence of a working `tsc`/`vite-env` project setup is also *why*
  no nx `typecheck` target exists: the repo's real pipeline is `bun`(runtime, strips types)
  + `vitest`(esbuild-transpiles, no type-checking) + `vite`(build, same) — nobody runs strict `tsc`
  over this tree today, so this latent debt has never surfaced.

**The one real, confirmed regression** — see the BLOCKING section above: verified directly by scoping a
single-file check to `♻️mit-bestand/🧺️demonstrator/📦️index.tsx`, which reproduces exactly
`TS2554: Expected 2-3 arguments, but got 1` at its `resolvePlaygroundBoot(bootVariant)` call (line 407).
Confirms the signature change is doing its job — every caller I own passes the catalog; the one caller
I don't own needs a follow-up.
