# 🧩️ PDF plugin as one component per runtime shard (round 7 audit and fixes, 2026-11-25)

Goal: a page that opens the PDF 1.7 editor instantiates and compiles only the `semio-hub-stdio-pdf-1-7-editors` component (7 apps), not the 20-app
`semio_hub_stdio_pdf` component. This round audits what round 6 left, fixes the failing transform and the missing pieces, and states what a wasm rebuild
still needs.

## 1. State found (round 6, end to end)

| Layer | Round 6 result | Verdict |
| --- | --- | --- |
| Descriptor schema | `PackageDescriptor.components` (`{id, apps}` rows, 1..64, id `^[a-z0-9_-]{1,128}$`, apps 1..4096) in `📇️registry/🛂️descriptor-verification/🧬️schema`; Rust `ComponentDeclaration` in `🛂️manifest/🧾️plugin-manifest`, re-exported by `semio_framework`; `CATALOG_DESCRIPTOR_TOP_LEVEL` and `descriptor-pair` verification admit and partition-check it | complete |
| Rust describe side | `PluginBuilder::component(id)` opens a component for the apps registered after it; `component_declarations` (registration) turns the start indices into `{id, apps}` and refuses a leading app, a duplicate id, a bad id and an empty component; `try_build` refuses artifact-projected apps under declared components; `PluginDescriptorExtras.components` flows into `plugin_descriptor`; every other `PackageDescriptor` literal sets `components: Vec::new()` | complete (not compiled here) |
| Kernel | `🧩️package-components` (`readPackageComponents`, `packageComponentForApp`, schema-first cases `🧫️fixtures/🔣️.json` shared with the Rust registration test); `descriptor-admission` admits `components` in `fetchDescriptorManifest` and `fetchPackageDescriptor`; `ActivationRegistry.activate(plugin, actor, reason, appId)` picks the component, remembers it per actor so a resume re-activates the same one | complete |
| Shard client | `activate(..., component)`, wire message `activate.component`, `coreModules` seeds | complete |
| Shard worker | `loadActor(..., component, ..., seeds)` hands the component to the bridge as the last `createActorApi` argument; `ShardCoreModules` keys compiled core modules by served URL | complete |
| Generated bridge | `pluginComponentBridgeSource(componentBase, components)`: per-activation `import("./<base>.js")` for the named component only, `actor-component.unknown` for a name the module does not stage, `componentBase` for an activation naming none | complete |
| Browser loaders | renderer `module-loader` (`createKernelHandle`) and wgpu `plugin-handle` pass `appId` to `registry.activate` and `components` to `registerManifest`; request/extension actors name no app and run on the first component | complete |
| Composition generator | `shards` per owner in `🧩️composition/🔣️.json`; the generator publishes each shard crate (its own `dyn_enum_close!`, its `plugin()` with `builder.component(<crate name>)` and its apps only), its `🧩️component` cdylib crate, and the hub `plugin()` with one `component(...)` call per shard in shard order | complete |
| Production publish (`browser-bundle` commands) | still one wasm per plugin target; the bridge is generated with `monolithicComponentTable`, so every declared id maps to the one factory | by design: ships the single hub component, correct for it |
| Dev staging script | `stage-pdf-component.ts`: one transpile per declared component, `describe()` probe per component compared with the plugin descriptor restricted to its apps, multi-component bridge, staged descriptor, receipt, `.semio-artifact.json` | see 2.3 |

Registry discovery (`generateComponentSourceRegistry`) lists 74 owners with one `stdio-pdf` owner (`semio_hub_stdio_pdf_component.wasm`): the shard
component crates are not registry owners (checked with `probe-component-registry.ts`), so they cannot collide with the hub component's plugin id.

## 2. Defects found and fixed

1. **The failing transform.** `🧑‍💻dev/🚚️distribution/🏗️compiler/🟦️.ts` line 45 closed six nested calls with `))))));` and lost the `}` of the innermost
   `async sink=>{`, since the `withViteCompilerCapacityV1` layer was added (HEAD has the same text; the last commit that parsed ended `})))));`). Fixed to
   `}))))));`. The staging suite collects again (62 passed in the first run after the fix).
2. **Two callers of the old bridge signature.** `pluginComponentBridgeSource(base, wasmFileName)` became `(base, components)` in round 6, but
   `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🏗️staging/🟦️.ts` (GIS staging) and `🌎️hub/🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts` still passed a
   file name string as the component table (a type error, and `Object.entries` of a string at run time). Both call it with the base only now.
   `refresh-staged-glue.ts` of this ticket passed the same string; it now reads the staged descriptor's components and builds the multi-component bridge.
3. **`stage-pdf-component.ts` could not take the described pair.** It verified the pair with `verifySemanticDescriptorPairBytesV1`, which refuses a JSON
   or pack above the 4 MiB catalog bound. The described `stdio-pdf` pair is 4,496,151 JSON bytes (`🧱️pdf-descriptor-and-retirement.md`: cross-app action
   duplication), so the stage would have stopped before staging anything. It now checks the pair as the staging needs it: `finalizePluginDescriptor`
   (identity, canonical pack) plus deep equality of the JSON with the pack form, then the components. `check-described-components.ts` runs this check and
   `packageComponentForApp` for every app against the real described pair: 20 apps resolve to the 1.4 shard (6), the 1.7 editors (7) and the 1.7 viewers
   (7); no app resolves to the first component (1.4).
4. **Stale expectations of the dev staging suite.** `🛂️routes-and-bridge` still expected `import(versionedUrl("./plugin_component.js"))` and
   `component.instantiate(...)`; it expects the factory form now (`import(versionedUrl("./" + base + ".js"))`, `factory.instantiate`, the empty
   `COMPONENT_BASES` table, base fallback). A new case covers a two-component table and `actor-component.unknown`.
5. **Docstring of the bridge generator.** The long `pluginComponentBridgeSource` docstring sat above `monolithicComponentTable`; moved to its function.

## 3. Tests (schema-first, language-agnostic case file shared by Rust and TS)

The cases are `🧰️framework/🔨️modules/🎠️kernel/🧩️package-components/🧪️tests/🧫️fixtures/🔣️.json` (schema `🧬️schema/🔣️.json`; Ajv validates the file, a
third-party check of the repository's own subset validator on the descriptor schema's `components` member). Layers and results:

| Layer | Suite | Result |
| --- | --- | --- |
| Case file + admission + registry activation + suspend/resume + served descriptor (both loaders) | kernel vitest `🧩️package-components/🧪️tests` (`run-kernel-vitest.sh`) | **6/6** |
| Rust registration law over the same cases | `builder/📜️registration/🧪️tests/🦀️.rs` `component_registrations_follow_the_language_neutral_package_component_cases` | not run (no cargo) |
| Generated bridge selects only the named component's factory; refuses unknown; monolith table maps all ids to one | bun `🌐️browser-bundle/🏗️materialization/🧪️tests/🧩️components` test 1 | pass |
| **New:** the bridge asks only for the core modules of the named component; two instances share one compiled module per core (2 compilations for 2 instances, 4 after the 1.4 component, none for the viewers); the last holder releases | same file, test 2 (`ShardCoreModules` + generated bridge) | pass |
| **New:** a generated shard worker in a real worker thread, `activate` with `component`: editors load fetches/compiles the editors cores only (2), a second editor instance fetches nothing and posts its holds, the 1.4 component fetches its own 2, an activation seeded with one viewer core fetches only the other | same file, test 3 | pass |
| **New:** every shard of an owner is one component: hub registers `component("<crate>")` in shard order, shard crate registers the same name, `🧩️component` crate is `<crate>-component` as a cdylib over `<crate>::plugin` | bun `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🧪️tests/🧱️production-preparation` | pass (3/3) |
| Composition `🪪️owner-pairs` + `🧱️production-preparation` | bun | 7 + 1 pass |
| Plugin actor transport (generated worker traffic, codec replies, segmented download) | os vitest `🎭️actor-transport` | **183/183** |
| Dev browser-host staging suite | dev vitest `ticket-owned-browser-host-staging` | 68 pass, 8 fail, 2 skipped; the 7 bridge/vendor cases touched by this work pass |

Remaining failures of the dev staging suite, none related to components (verified against their errors):

* 3 `pluginComponentBridgeSource` cases and `rewriteJcoComponentFactory`: the test-kit's fake guest (`retainedGuestSource`) has no `reactor.stageActorContextBudget`
  and answers the pre-staged-budget protocol (`poll(events, grant)`); adding the stage export only moves the failure into `originalReceiptFromWit`. The fake needs
  a rewrite to the current retained-turn protocol (the `🔁️succession` stand-in is the model). Another session's change; left alone.
* `scaleFixtureArtifacts`: `🧪️testing/⚖️scale/🤖️generated/📇️registry/🔣️.json` missing.
* `WASI component profile policy` (`SEMIO_SCRIPT_PROVIDERS` unset in this runner), `componentCargoV1` (`CARGO_RESOLVER_FEATURE_UNIFICATION` `selected` vs `package`),
  `deployed vendor transport` (`cli.js` byte equality of the vendored shim).

## 4. `stage-pdf-component.ts` against what the host expects

Checked line by line:

* Component id → jco base: `id.replaceAll("-","_") + "_component"`; the cargo package `semio-hub-stdio-pdf-<x>-component` produces
  `semio_hub_stdio_pdf_<x>_component.wasm`; the bridge table maps `id → base` with the first declared component as fallback (`componentBase`).
* Per component it copies `<base>.js`, `<base>.d.ts` and every `<base>.core<N>.wasm` into the module directory, relocates preview2 imports, copies `interfaces/` once, and
  rewrites `.semio-artifact.json` so the file list holds exactly those (old `semio_hub_stdio_pdf_component*` files are removed first).
  The core URLs the bridge requests are `./<name>` relative to the bridge: unique per component, so `ShardCoreModules` never confuses components.
* It writes `🌉️bridge.js` (multi-component), `🟨️.js`, the shard worker, the descriptor pair (`🛂️.descriptor.semio`, `🔣️.json`, finalized from the described pack) and sets the
  receipt's `wasmSha256`/`wasmBytes` to the first component and `semanticDescriptorSha256` to the pack. The receipt names one component only; nothing in the host
  reads it for per-component identity.
* Each component's own `describe()` must equal the plugin descriptor restricted to its apps and its single component row (`components`, `manifest.apps`, identity
  fields), so a shard cannot ship an app the plugin descriptor does not declare.
* `bun verify-staged` (`🧑‍💻dev/🔎️verification/🧾️convergence`) compares the staged receipt hash with the hub wasm for the `stdio-pdf` row and will report DIFF for
  a per-component stage; it is a dev report, not a gate, and was not changed.
* Not caught by the script: it takes `<wasm directory>/<base>.wasm` for every declared id and fails with the missing file's name if one component was not built.

## 5. What remains before a wasm rebuild and staging

1. Run the committed-descriptor test (`run-committed-descriptor.sh <label>`): it fails by design on the 4 MiB bound but writes the described pair
   (`🗑️generated/native-target/lane-*/t/tmp/stdio-pdf-described`); the one currently there (15:39) already carries the three components, so a re-run is only
   needed if the Rust sources changed. The committed `🧩️component/🔣️.json` is still the stale 3.66 MB pair without `components`; it cannot be regenerated until the
   descriptor shrinks below the bound (shared action tables, see `🧱️pdf-descriptor-and-retirement.md`). Staging does not need it.
2. `sh rebuild-pdf-wasm.sh` (builds the three `semio-hub-stdio-pdf-*-component` cdylibs in one cargo run, then runs `stage-pdf-component.ts`). Unverified by cargo: the
   Rust side of round 6 (`component`, `into_parts(app_ids)`, `ComponentDeclaration`, shard crates, all `PackageDescriptor` literals) has never been compiled; expect to
   fix compile errors there first.
3. Browser run: `probe-lazy-component.ts` (host on 6260) expects boot to fetch only the 1.7 editors component and the 1.4 open to fetch the 1.4 component;
   `pdf-memory-budget.ts` for the memory budget (compile once per component, one live memory).
4. Open points not addressed here: the production `browser-bundle` publish still ships the single hub component through the monolith table, and the kernel's
   `PluginCatalog` row type (`registerCatalog`, used by tests only) carries no `components`; both need the per-component deliverable model if the production
   path should split the PDF plugin too.

## 6. Files changed this round

Product:
* `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏗️compiler/🟦️.ts` (parenthesis)
* `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🏗️staging/🟦️.ts`, `🌎️hub/🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts` (bridge call)
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts` (docstring placement)

Tests:
* `…/🏗️materialization/🧪️tests/🧩️components/🟦️.ts` (core-module and worker cases)
* `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🛂️routes-and-bridge/🟦️.ts`
* `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🧪️tests/🧱️production-preparation/🟦️.ts`

Ticket: `stage-pdf-component.ts`, `refresh-staged-glue.ts`, `check-described-components.ts`, `probe-component-registry.ts`, this file; logs `🗑️generated/r7-*.log`.

## 7. Dev staging suite green

Run: `sh run-dev-vitest.sh "🧪️tests/🧪️ticket-owned-browser-host-staging" <absolute log>` (log `🗑️generated/dg-1.log` .. `dg-6.log`). First run in this
session: 64 pass, 12 fail (bun and the script provider map were missing from the runner, which the 8-failure run did not exercise). Final: **75 pass, 1 fail,
2 skipped** (78); the one failure is environmental and not fixable without a wasm build (see 7.10).

| # | Failure | Cause | Fix |
| --- | --- | --- | --- |
| 1 | `scaleFixtureArtifacts` (ENOENT `⚖️scale/🤖️generated/📇️registry/🔣️.json`) | the `🤖️generated` folder is gitignored output of the owned scale-fixture generator and was absent | regenerated through the owner's own `writeScaleFixtureArtifacts(root, 50, 50, 1)` (`regen-scale-fixture.ts`, same call as `repo ./📜️script.ts generate scale-fixture`); the test's own re-render byte comparison passes |
| 2-4 | three `pluginComponentBridgeSource` cases (`stage is not a function`) | the fake guest `retainedGuestSource` answered the pre-staged-budget protocol (`poll(events, grant)`) | the fake now answers the current protocol: `stageActorContextBudget` keeps the budget, `admitActorInput(wire)` hands the pack wire to `__bridgeAdmitted`, `poll()` returns `retainedReceipt` accounted against the staged `budget.input`; the cases assert the admitted wire bytes against `createActorInputWireV1().encode({operation:"poll", events, shape})` (patch ACK/rejection cases, byte-exact) and the decoded `events` tags (lifecycle cases); the stage-page case now expects the order admit, stage, poll |
| 5 | `rewriteJcoComponentFactory` (timeout) | same stale fake (`grant.retained`) | the poll body of the case uses `receipt()` |
| 6-7, 11-12 | `bun child failed`, `spawnSync bun ENOENT`, `backboneDbHandleFor` x2 | bun was not on the runner's `PATH` | `run-dev-vitest.sh` prepends the repository's bun runtime cache (`~/Library/Caches/semio/runtimes/bun/*/`, `~/.cache/semio/runtimes/bun/*/`, `~/.bun/bin`) |
| 8 | `WASI component profile policy` (`SEMIO_SCRIPT_PROVIDERS` unset) | the runner is an issuing parent that never issued the provider designation | `issue-script-providers.ts` writes the designation through `repositoryScriptProvidersV1` + `writeScriptProviderMapV1`; the runner exports it. The case then failed on a stale expectation: the `wasm-component` prototype profile is `opt-level "s"` (`🎚️mode/🦀️profile/🧬️schema/🔣️.json`, `prototype-component`), the test expected 1; now `"s"` |
| 9 | `componentCargoV1` | the native build driver policy const is `featureUnification: "selected"` (schema `🧩️component-pass/🧬️schema/🔣️.json`); the test expected `"package"` | expectation updated to `"selected"` (the native-build tests assert the same value) |
| 10 | `deployed vendor transport` `cli.js` equality | the test derived the expected patched `cli.js` in the vitest process, where `classifyGuestLogLine.toString()` (embedded by the patch) is the transformed source (2 x 4 bytes shorter) while the product is built by Bun | the expected bytes are produced in a Bun child (`underBun`, the suite's own idiom for Bun-hosted code) from the same patch module; the 15 `dev-vendor-*` fixed filename contracts of `🦑️repo/…/📚️library/🔣️taxonomy.json` pointed at the retired `…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🪞️vendor/…` and now state `.cache/b/p/m/browser/*/modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/<file>` (the product root of `browserProductRootsV1`, both modes) |

7.10 Remaining (not fixable here): the same case then counts the materialized components of the prototype product (fixture `🪞️vendor.json`: 60 plugin and 26
extension components) and finds 2 plugin modules (`📘️stdio-pdf`, `🗒️note`) in `.cache/b/p/m/browser/prototype/modules`. It needs the full prototype
activation (about 60 wasm component builds), which this task may not run; once the activation has materialized, the earlier assertions of the case pass as shown.
The 15 `dev-vendor-*` contract rows were the other (unrelated to the other agent's `actionSets` / a11y heading edits) cause; nothing was reverted.

Files: `run-dev-vitest.sh`, `issue-script-providers.ts`, `regen-scale-fixture.ts` (ticket); `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/`
`🧰️test-kit`, `🛂️routes-and-bridge`, `🧬️jco-rewrites`, `🏗️profile-and-catalog-build`, `🚚️vendor-transport` (`🟦️.ts`); `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`;
regenerated (ignored) `⚖️scale/🤖️generated/{📇️registry,🗂️catalog}/🔣️.json`.
