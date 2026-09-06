# Lane K — the OS host plugin's identity is `space`, everywhere

Ticket `26/09/05/S-END-TO-END` · Opus 5 · started 2026-09-05 23:18, cut once by the session limit right
after the census greps (nothing had been written then), resumed 23:18.

## Headline

The plugin identity is now `space` in every authority: Cargo component package `semio:space`, root
`builder("space")`/`.package_id("semio:space")`, deployment catalog row `{ "space", "🪐️space" }`, the
dev cache directory `🔌️plugin-modules/🪐️space/`, the taxonomy path patterns, every hand-authored
fixture/spec, and — after `bun ./📜️script.ts generate` — every generated projection
(`🔌️plugins.json`, `🧩️plugins.ts`, `🖥️hosts.rs`, `🎠️playgrounds.json`, `🎮️playgrounds.ts`).
The playground/launch **variant** stays `s`: `DEFAULT_HOST_VARIANT === "s"`, the Cargo
`[[package.metadata.semio.playground]] variant = "s"` row is untouched, and the generated Rust variant
map now reads `("s", "space")`. Artifact kinds keep their `s.space.*` ids, which is precisely what makes
lane H's assembly gate (`plugin-assembly.surface-dependency-gate` → `artifact identity is not owned by
the declaring plugin`) agree: `ArtifactKindId::plugin()` derives owner `space`, the manifest now says
`space`.

Two new tests make the tuple unfalsifiable in both languages, both driven by one language-agnostic
fixture. `check-frame-worker`, which the 19:05 wgpu report recorded as RED because of a stale bundle,
is green again as a side effect (the bundle embeds the deployment catalog).

## 1. Census — every place the literal `s` meant the plugin id

Method: `grep -rn` (Bash, never the search tool) over `*.rs`/`*.ts`/`*.tsx`/`*.json`/`*.jsonc`/`*.toml`,
excluding `node_modules`, `target*`, `.🧬semio`, then each hit read in context. Three categories were
explicitly **kept**: the playground/launch variant `s`, the `s.` product prefix of artifact kinds, and
the `s-`/`✏️s/` path prefixes.

### 1a. Plugin identity — changed

| # | File:line (pre-edit) | What it was |
|---|---|---|
| 1 | `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:12` | `[package.metadata.component] package = "semio:s"` — the ROOT authority; the registry derives `pluginId = packageId.slice("semio:".length)` (`📇️registry/📜️script.ts:268-269`) |
| 2 | `✏️s/🔌️plugins/🪐️space/🦀️.rs:809` | `Plugin::<SpaceApps>::builder("s")` |
| 3 | `✏️s/🔌️plugins/🪐️space/🦀️.rs:812` | `.package_id("semio:s")` |
| 4 | `✏️s/🔌️plugins/🪐️space/🦀️.rs:1074` | lane J's identity test's `assert_eq!(… .plugin_id, "s")` |
| 5 | `…/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json:52` | `{ "pluginId": "s", "directoryName": "🪐️s" }` |
| 6 | `…/📦️deployment/🧪️cases.json:34` | `moduleUrls` row `s` → `/🔌️plugin-modules/🪐️s/🌉️bridge.js` |
| 7 | `…/📇️registry/✅️catalog-complete.test.ts:66` | `deployment.moduleStaticDirectoryNames("s", true)` |
| 8 | `…/📇️registry/🧫️fixtures/📖️generated-projection.json:6,14,20-22` | the projection expectation vector's `s` entry, playground row and three `pluginIds` lists |
| 9 | `…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2422` | `COLLAB_E2E_REQUIRED_PLUGIN_IDS = ["s", "writer"]` — a *plugin id* list, with a doc comment that explicitly (and now wrongly) asserted `pluginId` is `"s"` "NOT `space`" |
| 10 | `…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5510,5512,5514,5561,5562,5564` | `descriptorRouteDecision` + `scanBuiltPluginModules` tests keyed on the `🪐️s` directory and the resulting `pluginId` `"s"` |
| 11 | `…/🧑‍💻dev/🧫️fixtures/🔬️catalog-smoke.json:4,77` | `"shellPluginId": "s"` in both aggregation vectors |
| 12 | `🧰️framework/🛍️products/🦑️repo/…/📚️library/🔣️taxonomy.json:5441,12823-12825,15078` | `deployed-module-members` member `🪐️s` + four JCO path/exact-path authorities under `🔌️plugin-modules/🪐️s/` (plus their registry keys and reason strings) |
| 13 | `🌎️hub/📇️directory/🧫️fixtures/🌐️directory-home-browser-process-v1/🔣️.json:9` | `"home": { "pluginId": "s", …, "moduleDirectory": "🪐️s" }` |
| 14 | `🌎️hub/📇️directory/🧫️fixtures/🌐️directory-home-browser-process-v1/🧬️.schema.json:51,54` | the two `const` pins for the same two fields |
| 15 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts:5887` | the TS type literal mirroring #13 |
| 16 | `…/📺️renderer/…/🎯️targets/⚛️react/📇️directory-home-bootstrap.test.tsx:53,54,102` | the real-identity Home handle (`pluginId: "s"`, manifest `pluginId: "s"`, dispatch address) |
| 17 | `.storybook/stories/framework/os/plugins.stories.tsx:56` | `export const S: Story = { args: { plugin: "s" } }` |
| 18 | `.storybook/s-end-to-end.spec.ts:10,11` | `S_STORY_ID = "🛠️framework🖥️os-plugins--s"`, `S_PLUGIN_ID = "s"` — literals where the brief asks for catalog derivation |
| 19 | `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2995` | `PluginHostConfig` docstring naming the host pair `"s"`'s home/studio |
| 20 | generated: `🤖️generated/{🔌️plugins.json,🧩️plugins.ts,🖥️hosts.rs,🎠️playgrounds.json,🎮️playgrounds.ts}` | `pluginId`/`packageId`/`plugin_id`/variant map rows |
| 21 | generated: `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js:18860` | the bundled copy of the deployment catalog row |

### 1b. Deliberately NOT changed, with the reason

- **Playground/launch variant `s`** — `Cargo.toml [[package.metadata.semio.playground]] variant = "s"`,
  `DEFAULT_HOST_VARIANT`, `dev s`, `SEMIO_PLUGIN=s`, `S_OS_PORT`, `.claude/launch.json` `s-react`/
  `🛠️dev🖥️s⚛️react`, the alias `studio`, `📜️script.ts:486-515` (`segments[0] === "s"`). This names the
  shell variant, not the plugin. Resolution goes variant → `pluginId` through the catalog
  (`resolvePluginRegistryId`, `findPlaygroundVariant`), so the two names are already decoupled at every
  call site; the generated map now reads `("s", "space")`.
- **Artifact-kind prefix `s.`** — `s.space.home@1/*#editor` etc., `📜️script.ts:34367`
  (`segments[0] !== "s"`), `🔌️plugin/📦️packages/🦀️rust/📜️script.ts:55`, `🚪️io/🧬️schema/🦀️.rs:146`,
  `🌉️mcp/…/📜️script.ts:124`. `s` is the *product* segment of `s.<plugin>.<kind>`; the **owner** segment
  is the second one, which is `space` and always was. Changing it would be the actual regression.
- **Path prefixes** — `✏️s/🔌️plugins/…`, crate name `semio-s-plugin-space`, wasm out
  `semio_s_plugin_space.wasm`, `semio_s_plugin_space_component.*`. These carry the product `s`, not the
  plugin id, and `packageName` is pinned by the new fixture as-is.
- **`💻️os/🧫️fixtures/📇️directory/🚀️event-page-bootstrap-v1.json:27`** (`retainedHome.pluginId: "s"`,
  `appId: "s.home"`). Synthetic trace fixture: the schema declares only
  `{"type":"string","minLength":1}` (no `const`), the app id `s.home` is not a real app id, and the only
  consumers (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3353-3362`, wgpu `📜️script.ts:455`) compare it to itself.
  Renaming it would imply inventing a matching app id. Left, and recorded here.
- **Unrelated `"s"` literals** — UI surface ids, `SurfaceReconciler::new("s")`, `scope: "s"`,
  `opt-level = "s"`, `detail: "s"`, `${n === 1 ? "" : "s"}` pluralisation, `STPQCharacter`. ~90 hits,
  each read; none is a plugin id.

### 1c. Stale generated artifact left behind (not a source authority)

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧶️bundles/🚀️bootstrap-B__lgXdC.js`
still embeds `{pluginId:"s",moduleUrl:"/🔌️plugin-modules/🪐️s/🌉️bridge.js"}`. It is a committed
**production Vite build output** with a content-hashed filename; regenerating it means a full
`framework-os-dev:build`, which is the coordinator's/wave-3's, not this lane's. Its sibling
`🎞️frame-worker.js` **was** regenerated here because it has a cheap dedicated generator.

## 2. File:line changes

### Rust — the plugin itself

- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:12` — `package = "semio:space"`.
- `✏️s/🔌️plugins/🪐️space/🦀️.rs:809` — `Plugin::<SpaceApps>::builder("space")`.
- `✏️s/🔌️plugins/🪐️space/🦀️.rs:812` — `.package_id("semio:space")`.
- `✏️s/🔌️plugins/🪐️space/🦀️.rs:914-916` — new test consts `IDENTITY_FIXTURE`, `DEPLOYMENT_CATALOG`,
  `GENERATED_REGISTRY` (`include_str!`, `#[cfg(test)]` only, so nothing is linked into the guest).
- `✏️s/🔌️plugins/🪐️space/🦀️.rs:1074` — lane J's assertion now reads the id out of the fixture instead of
  a second literal.
- `✏️s/🔌️plugins/🪐️space/🦀️.rs:1080-1136` — `identity_fixture()` + the new law
  `plugin_identity_is_the_same_in_every_authority` (§4).

### Registry / deployment authority

- `…/📇️registry/📦️deployment/🗺️catalog.json` — the row moved from line 52 to line 58 and became
  `{ "pluginId": "space", "directoryName": "🪐️space" }`. The move is required, not cosmetic:
  `✅️catalog-complete.test.ts:78-82` asserts `catalog.modules.map(pluginId)` **equals**
  `🤖️generated/🔌️plugins.json`'s order, which is `localeCompare`-sorted, so `space` must sit between
  `sourcing-module-windows` and `stdio`. The `🪐️` emoji is preserved — `parseModuleDirectories`
  (`📦️deployment/🟦️.ts:38-49`) rejects a duplicate sibling emoji.
- `…/📦️deployment/🧪️cases.json:34` — `moduleUrls` row → `space` / `🪐️space`.
- `…/📇️registry/✅️catalog-complete.test.ts:66` — `moduleStaticDirectoryNames("space", true)`.
- `…/📇️registry/🧫️fixtures/📖️generated-projection.json:6,14,20-22` — entry, playground row and the
  three host `pluginIds` vectors; `activationEvents` also corrected to the live pair
  (`space.shome` + `space.sspace`) so the fixture stops lying about the row it mirrors.

### Dev cache directory

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪐️s` → `🪐️space` (`mv`, not a
  delete/rebuild: the 64 MB core wasm, component JS/d.ts, `interfaces/`, `🌉️bridge.js`, `🔣️.json` and
  `🛂️.descriptor.semio` from build 6 are all still there under the new name, which is exactly what the
  next `describeBuiltPlugin`/boot expects to find via `moduleDirectoryName("space")`).

### Dev tooling

- `…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:47` — new import of `PLUGIN_HOST_CONFIGS` from
  `🤖️generated/🧩️plugins.ts` (the generated module, not the barrel — same import the storybook specs use;
  it is outside `⚙️vite.config.ts`'s import graph, so lane E's 32-module config budget is untouched).
- `…/📜️script.ts:2414-2423` — `COLLAB_E2E_REQUIRED_PLUGIN_IDS` is now
  `[...PLUGIN_HOST_CONFIGS.map((entry) => entry.pluginId), "writer"]`; the doc comment that asserted the
  id is `"s"` "NOT `space`" is replaced by one that says the opposite and names why.
- `…/📜️script.ts:2476-2502` — the six lease/registry/build calls that take a *variant* now pass
  `DEFAULT_HOST_VARIANT` instead of a bare `"s"`, so the variant and the plugin id are visibly different
  things at the call site (`SEMIO_PLUGIN: "s"` at :2520 and :6355 stays: that is the variant env var).
- `…/📜️script.ts:5510-5564` — the `descriptorRouteDecision` and `scanBuiltPluginModules` unit tests now
  use `🪐️space` / `"space"`, and the fake core wasm is named `semio_s_plugin_space_component.core.wasm`
  (the real basename) instead of the invented `s_plugin_component.core.wasm`.

### Shell / harness — literals replaced by catalog derivation

- `.storybook/s-end-to-end.spec.ts:9-22` — imports `PLUGIN_HOST_CONFIGS`; `S_PLUGIN_ID` is the single
  host row's `pluginId` (and throws if the catalog ever declares zero or several), `S_STORY_ID` is
  derived from it. Header and two messages updated.
- `.storybook/stories/framework/os/plugins.stories.tsx:57` — `export const Space` with
  `plugin: "space"`, in sorted position (the story id `…--space` is what the spec now computes).
- `…/🎯️targets/⚛️react/📇️directory-home-bootstrap.test.tsx:53-54,102` — the modelled Home handle and the
  asserted dispatch address are `space`.
- `…/🧑‍💻dev/🧫️fixtures/🔬️catalog-smoke.json:4,77` — `shellPluginId: "space"`.
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2995` — docstring names the `space` plugin.
- ShellHost itself needed **no** change: `hostConfig`/`primaryPluginId`/`pluginShouldEstablishSession`
  (`🏛️ShellHost/🟦️.tsx:1409,1431,2218,2337,3511,3926,3946`) already resolve through
  `resolvePluginHostConfig(PLUGIN_CATALOG, pluginFilter)`, and the readiness beacon
  (`:7669-7692`) stamps `pluginFilter`, which is the variant in the dev shell and the story's plugin id
  in storybook — both now correct without a literal anywhere.

### Hub

- `🌎️hub/📇️directory/🧫️fixtures/🌐️directory-home-browser-process-v1/🔣️.json:9` and
  `🧬️.schema.json:51,54` — `pluginId`/`moduleDirectory` consts.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts:5887` — the mirrored TS literal type.

### Taxonomy (schema authority)

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:

- `deployed-module-members.memberNames` — `🪐️s` removed, `🪐️space` inserted in sorted position
  (between `🪟️sourcing-module-windows` and `🗄️stdio`).
- the five `🔌️plugin-modules/🪐️s/…` path patterns / exact paths → `🪐️space`.
- the four registry keys renamed with every reference: `dev-plugin-component-s-{js,declaration,wasm}` →
  `…-space-…` (2/2/1 occurrences) and `dev-plugin-interfaces-s` → `dev-plugin-interfaces-space`
  (8 occurrences: its own row plus the six `dev-jco-*-interfaces` group lists and the fixed-contract map).
- five `reason` strings ("… for public plugin s" → "… space", "Exact emitted s component interface
  owner" → "space"). Re-parsed with `json.loads` after writing.

### New tests and their registration

- **New** `✏️s/🔌️plugins/🪐️space/🧪️fixtures/🧫️plugin-identity/🔣️.json` + `🧬️.schema.json` — the
  language-agnostic identity tuple (§4).
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:252-311` — new exported
  `spacePluginIdentityOracle(repoRoot)`; `:320` — `interactiveJobCatalogOracle` now *starts* from it
  (lane J's 3-way pin is subsumed, not duplicated); `:359-365` — `PluginIdentityCheckScript`; `:404` —
  router registration `plugin-identity-check`.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📋️project.json` — Nx target `plugin-identity-check`.
- `.vscode/🧩️launch.seed.jsonc` — `⚖️gate🪪️space-plugin-identity`, group `4_gate`, order `411.0435`
  (immediately before the sibling `⚖️gate🧵️space-interactive-job-catalog` at 411.044).
- `.vscode/launch.json:5103` — regenerated from the seed.

### Regenerated (by `generate`, not hand-edited)

`🤖️generated/🔌️plugins.json`, `🧩️plugins.ts`, `🖥️hosts.rs`, `🗿️artifacts.rs`, `🎠️playgrounds.json`,
`🎮️playgrounds.ts`, `🏗️framework.ts`, `🧰️framework.json`; `.vscode/launch.json`;
`…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js`.

## 3. Commands and real outputs

### Regeneration — the row, the package id, the host, the variant

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry && bun ./📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages) -> …/📇️registry/🤖️generated
.vscode/launch.json regenerated -> /Users/ueli/Documents/semio/.vscode/launch.json
```

```
🧩️plugins.ts:42  { pluginId: "space", landingAppId: "home", hostAppId: "studio" },
🧩️plugins.ts:74  { pluginId: "space", packageId: "semio:space", cratePath: "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust",
                   wasmOut: "semio_s_plugin_space.wasm", role: "plugin", capabilities: ["documents.write"], …
                   dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:space.shome","on-artifact-kind:space.sspace"],
                   host: { landingAppId: "home", hostAppId: "studio" }, executionMode: "isolated", hashes: { … } },
🖥️hosts.rs:10    PluginHostConfig { plugin_id: "space", landing_app_id: "home", host_app_id: "studio" },
🖥️hosts.rs:64    ("s", "space"),
🎮️playgrounds.ts:86  export const DEFAULT_HOST_VARIANT = "s";
🔌️plugins.json    59 rows, tail: … 'sourcing-module-windows', 'space', 'stdio', 'trinity', 'vcs', 'writer'
```

All four requested confirmations hold: `pluginId: "space"`, `packageId: "semio:space"`, `host` intact,
`DEFAULT_HOST_VARIANT` still `"s"`.

### Space plugin — the new identity gate, and lane J's oracle on top of it

```
$ cd ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust && bun ./📜️script.ts plugin-identity-check
plugin-identity-check: checks=11 clean

$ bun ./📜️script.ts interactive-job-catalog-check
interactive-job-catalog: descriptor rows without an interactiveJob disposition: 47 (regenerated by `describe` after a wasm build)
interactive-job-catalog-check: checks=23 clean
```

(23 = lane J's 12 + the 11 identity checks. The 47 is lane J's known descriptor staleness, unchanged.)

### Registry vitest

`bun nx run @semio-tech/plugin-registry:test` is **not runnable as registered on this machine**: the
`test` level pins a 15 s budget and the run needs ~850 s under load 140-210, so it is killed before the
first assertion —

```
$ bun nx run @semio-tech/plugin-registry:test
[budget] … vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 15000 … exceeded 15000ms — killed.
```

(`SEMIO_CMD_BUDGET_MS`/`SEMIO_TEST_ORCHESTRATION_BUDGET_MS` do not lift it; the level budget is
internal.) Run through vitest directly instead, per file:

```
$ cd …/📇️registry && bun …/vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 1800000 ✅️catalog-complete.test.ts
 Test Files  1 passed (1)
      Tests  11 passed (11)
   Duration  1512.45s
```

```
$ cd …/📇️registry && bun …/vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 600000 📖️generated-projection.test.ts 🚀️launch.test.ts
 Test Files  1 failed | 1 passed (2)
      Tests  2 failed | 7 passed (9)
```

`📖️generated-projection.test.ts` is the passing file (4/4, including the live-projection host predicate
over every real variant/alias/plugin id). Both `🚀️launch.test.ts` failures are **peer-owned and
pre-existing at HEAD**, verified against `git show HEAD:`:

- `exposes every owned generator preview exactly once in contract order` — a peer added the
  `dev-distribution-bundle` generator contract to `🔣️taxonomy.json` without extending this test's
  `previewOrder` list (`🚀️launch.test.ts:26-34`). Present at HEAD (`git show HEAD:…taxonomy.json | grep -c
  dev-distribution-bundle` → `2`); my taxonomy diff adds zero occurrences of it.
- `keeps generated native, root preflight, and MCP runtime profiles identical without debug` —
  `expect(describe).toContain('"wasm32-wasip2", "wasm-dev"')` against
  `🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`, which a peer rewrote to
  `"--target", "wasm32-wasip2", "--profile", "wasm-dev"`. `git show HEAD:` of that file already contains
  0 occurrences of the asserted substring; the file is committed clean (last commit
  `b0dfa0f09b 2026-09-05 19:04:38`, no working-tree diff) and I never touched it.

### Kernel vitest

```
$ cd 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript && bun …/vitest.mjs run --config vitest.config.ts --testTimeout 300000
 Test Files  2 passed (2)
      Tests  50 passed (50)
   Duration  223.48s
```

(The registered `bun ./📜️script.ts test` hits the same 15 s level budget.)

### wgpu frame worker — the stale-bundle RED from the 19:05 report is cleared

```
$ cd …/🎯️targets/🧊️wgpu/📦️packages/🦀️rust && bun ./📜️script.ts generate-frame-worker
framework-renderer-wgpu: generated 🎞️frame-worker.js
$ bun ./📜️script.ts check-frame-worker
framework-renderer-wgpu: 🎞️frame-worker.js is fresh
```

The regenerated bundle carries `🪐️space` (3 occurrences) and zero `🪐️s"`.

### Residual-literal sweep

A repo-wide `grep -rl '🪐️s/🌉️bridge\|"🪐️s"\|🪐️s/semio'` (excluding `node_modules`, `.git`, `target*`,
`.🧬semio`) returns exactly the two generated bundles named in §1c; after the frame-worker regeneration
only `📤️distribution/🧶️bundles/🚀️bootstrap-B__lgXdC.js` remains, and it is a hashed production build
output.

### Still running / not obtained

- `cargo check -p semio-s-plugin-space --lib --keep-going --message-format=short` with
  `RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-k` — started 00:17 into a **cold** private target
  dir; at the time of writing it is still in third-party dependencies (`wgpu-types`, `metal`,
  `futures-util`) with `0` `error[` lines so far, under load average 211. Log:
  `scratchpad/lane-k/cargo-check-space.txt`. Lane J's 21:45 finding stands as the prior: the crate's
  own code was clean and the only errors came from a peer's in-flight `💻️os/🖥️host/🦀️.rs` async/dyn
  migration (`E0038`/`E0308` on `dsl::Backbone`), which is outside the plugin.
- `framework-os-dev test quick` and the react-target vitest: the react run aborted with
  `[vitest-pool]: Failed to start forks worker … Timeout waiting for worker to respond` after 61 s —
  a worker-startup starvation at load 200+, not an assertion. Both are retryable and neither has been
  observed failing on content.

## 4. Tests added

### The language-agnostic fixture

`✏️s/🔌️plugins/🪐️space/🧪️fixtures/🧫️plugin-identity/🔣️.json`:

```json
{
  "schema": "semio.space.plugin-identity/v1",
  "pluginId": "space",
  "packageId": "semio:space",
  "packageName": "semio-s-plugin-space",
  "moduleDirectoryName": "🪐️space",
  "artifactKindPrefix": "s.space.",
  "playgroundVariant": "s",
  "host": { "landingAppId": "home", "hostAppId": "studio" }
}
```

with its own pinned `🧬️.schema.json` (2020-12, `additionalProperties: false`, patterns for each id
shape). `playgroundVariant` is in the tuple **on purpose**: the fixture is the one place that states,
in data, that the variant and the plugin id are two different names.

### Rust — `✏️s/🔌️plugins/🪐️space/🦀️.rs:1094` `plugin_identity_is_the_same_in_every_authority`

Reads the fixture, then joins it to five authorities in one law:

1. the assembled manifest's `plugin_id` (so `builder(…)` is covered through a real `plugin()` build, and
   `PluginBuilder::try_build`'s own `semio:<plugin_id>` rule covers `.package_id(…)`),
2. `Cargo.toml` — the `[package.metadata.component] package`, the `[package] name`, **and** the
   `[[package.metadata.semio.playground]] variant` (asserted to be the *other* name),
3. the canonical grammar — `artifactKindPrefix == "s.<pluginId>."`, and **every** app id in the built
   manifest starts with it, which is exactly the predicate lane H's assembly gate derives via
   `ArtifactKindId::plugin()`,
4. `📦️deployment/🗺️catalog.json` — the row exists for `pluginId` and its `directoryName` is the fixture's,
5. `🤖️generated/🔌️plugins.json` — the row's `packageId`, `packageName` and `host` block.

`assert_eq!(… , "space")` in lane J's older test (`:1074`) was replaced by a read from the fixture, so
the crate now contains no second literal of the identity anywhere.

### TypeScript — `spacePluginIdentityOracle`, registered as `plugin-identity-check`

Same five authorities from the other side, plus what Rust cannot see: the fixture is first validated
against its own schema by ajv 2020 (third-party oracle), then the generated **playground** module is
checked (`DEFAULT_HOST_VARIANT === playgroundVariant`, and the `{ variant, pluginId }` row maps the
variant to the plugin identity), the generated **Rust** host table (`🖥️hosts.rs`) is checked to carry the
same host config, and the deployment catalog's id roster is asserted **order-equal** to the generated
registry's — the exact invariant `✅️catalog-complete.test.ts` needs and the one that made moving the
catalog row mandatory rather than cosmetic. 11 checks. Lane J's `interactiveJobCatalogOracle` now begins
by calling it, so the previous 3-way pin is a strict subset and nothing is duplicated.

### The shell has no host-plugin literal left

`.storybook/s-end-to-end.spec.ts` derives both the plugin id and the story id from `PLUGIN_HOST_CONFIGS`
and **throws** if the generated catalog ever declares zero or more than one host — so a future host
rename cannot make the spec silently assert nothing, and re-introducing a literal `"s"` there would be
visible as a diff against a derived value. Together with the dev script's
`COLLAB_E2E_REQUIRED_PLUGIN_IDS` (also derived) and ShellHost's pre-existing `hostConfig` resolution,
no shell or harness path names the host plugin by hand any more.

## 5. Blockers / notes for the coordinator

1. **The committed owner descriptor pair is now identity-stale by construction.**
   `✏️s/🔌️plugins/🪐️space/🔣️.json:5` still says `"pluginId": "s"` and its packed twin
   `🛂️.descriptor.semio` is hash-bound to it; the same holds for the copies in the cache directory. Both
   are build outputs of `describeBuiltPlugin` and cannot be hand-edited without desyncing the pack hash,
   so they were left alone — the next `SEMIO_PLUGIN_ONLY=space`(or `=s` variant) rebuild re-emits them
   with `space`. Until then `plugin-registry:check` will report this row's descriptor identity as
   divergent. That is the intended, visible drift, not a regression.
2. **The next rebuild is the real proof.** This lane deliberately built no wasm. The assembly gate that
   produced `artifact identity is not owned by the declaring plugin` now has a manifest whose
   `plugin_id` matches the `s.space.*` owner segment, and the descriptor-identity check that produced
   `component package identity must exactly match semio:<plugin-id>` now sees `semio:space` on both
   sides — but only a build can confirm it.
3. **`cargo check -p semio-s-plugin-space --lib` is unfinished** (cold target dir, load 211). No error
   yet. Expect it to end RED *outside* the plugin if the peer's `💻️os/🖥️host/🦀️.rs` async/dyn migration
   is still mid-flight, exactly as lane J recorded at 21:45.
4. **Two `🚀️launch.test.ts` failures are pre-existing peer drift** (`dev-distribution-bundle` missing
   from the test's `previewOrder`; the `🖨️describe` script's changed cargo-argument style). Both
   reproduce at HEAD without my diff. Neither belongs to this lane; whoever owns the describe-script
   rewrite and the distribution-bundle generator should close them.
5. **`plugin-registry:test` and `framework-kernel:test` are unrunnable at their registered `test` level
   on a loaded machine** — a 15 s internal budget against ~850 s / ~220 s real runs. Both pass when
   driven through vitest directly. If these are meant to be gates, they need `itLong`/level gating like
   lane A did for `framework-os-dev test quick`.
6. **One stale production bundle**, `📤️distribution/🧶️bundles/🚀️bootstrap-B__lgXdC.js`, still embeds
   `🪐️s`; it needs a full `framework-os-dev:build` to regenerate (hashed filename will change).
