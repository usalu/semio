# Energie + Statik runtime wiring — the demonstrator-local union activation receipt

2026-09-17. Slice: wire the ENERGY playground (`energy` / pluginId `energy` / `s.energy.model@1/*#editor`)
and FEM 3D (`fem3d` / pluginId `fem` / `s.fem.fem3d@1/*#editor`) into the Entwerfen-mit-Bestand
demonstrator runtime. Brand/page/acceptance files were owned by sibling agents and are NOT touched here.

## 1. The blocker and the design

`readDemonstratorActivation()` used to read ONE framework receipt —
`<os-dev pkg>/dist/runtime/react/dev/generator/activation/🔣️receipt.json` — and demand that its plugin
list equal `demonstratorRuntimeComponentIds()`. That receipt is by construction the closure of the
`demonstrator` pluginId (28 components; its rows come from `dist/sessions/generator/🎮️playground-session`
`PLAYGROUND_SESSION.plugins`). `energy` and `fem` can never appear in it, so the union check could never
pass once the two new panes were added.

Implemented fix, entirely demonstrator-local: **activation lanes + a union receipt the demonstrator owns.**

* A *lane* is a framework `activate-<variant>-react-dev` target whose receipt the demonstrator consumes.
* `demonstratorActivationLanes()` = the primary lane (`generator`) plus one lane per pane runtime variant
  whose pluginId is **not already inside an accumulated closure**. Today that is exactly
  **`generator`, `energy`, `fem3d`** (3 lanes).
* `mergeDemonstratorActivationReceipts()` (pure, exported, unit-tested) unions the lanes' plugin rows,
  refuses a lane whose `profile !== "dev"` or whose `variant !== lane`, refuses a disagreement about one
  plugin's `artifactSha256` as a **stale lane**, keeps the earliest `rebuiltAt` for an agreeing overlap,
  and refuses any set that is not exactly `demonstratorRuntimeComponentIds()` (message lists
  missing/extra ids).
* `publishDemonstratorUnionReceipt(workspace)` publishes the merged receipt with the framework's atomic
  `publishActivationReceipt` into `♻️mit-bestand/🧺️demonstrator/dist/♻️activation/dev`
  (gitignored — confirmed by `git check-ignore -v`, `.gitignore:299 dist`).
* `readDemonstratorActivation()` now returns `{ receiptDirectory (= union dir), extensionsDirectory
  (= generator lane's `extensions`), receipt, laneReceiptDirectories }`.
* A demonstrator-local Vite plugin `demonstratorUnionReceiptVitePlugin({ workspace })` observes **every**
  lane receipt directory and republishes the union on any change; failures are logged, never thrown, so a
  half-written lane cannot kill the dev server. It is registered **before** `semioActivationVitePlugin`,
  which stays pointed at the union `receiptDirectory`.

### Why `generation3d` is deliberately NOT a lane

The slice brief specified lanes as `["generator", ...demonstratorRuntimeBuildVariants("generator")]` while
also stating the result should be `generator, energy, fem3d`. Those two are inconsistent:
`demonstratorRuntimeBuildVariants("generator")` is `["generation3d", "energy", "fem3d"]`, so the literal
formula yields **four** lanes. The stated intent (3 lanes) is the correct one, and measurably so:

* `procedural` is reached through `demonstrator`, so the `generation3d` lane contributes **zero** new
  components to the union.
* The two receipts on disk **already disagree** about all 11 shared plugins' `artifactSha256`
  (`flow`, `flow-extension-*`, `procedural`). Including that lane would make `publishDemonstratorUnionReceipt`
  throw "Stale Demonstrator activation lane" today, and would keep doing so whenever the two independently
  cached `activate-…` targets are replayed from different cache generations.

So the implemented rule is coverage-driven — a pane earns a lane only when its plugin is unreachable from
an already-covered closure — and it is asserted by a unit test.

## 2. Nx graph: a second blocker (framework contract)

`activate-dev.dependsOn` could not simply grow. `testDemonstratorRuntime` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:210`
pins it to **exactly** `["prepare-dev", pipeline.development.activationTarget]` — two entries, one
activation target.

Resolved demonstrator-locally: a new aggregation target `activate-lanes-dev` (`nx:noop`) whose `dependsOn`
lists the three framework activation targets, and `🧫️pipeline.json`'s
`development.activationTarget` now points at it. `activate-dev.dependsOn` stays two entries. That string is
consumed by nothing else in the repo (grep: only this assertion + the demonstrator schema).

## 3. One unavoidable framework edit (please review)

`testDemonstratorRuntime` hardcoded the demonstrator's root set:

```ts
const layout = runtime.demonstratorRuntimeModuleLayout(["demonstrator", "procedural"]);
```

and then asserted, against the SAME `🧫️cases.json` arrays, both that layout (28 modules) and
`demonstratorRuntimeAssetSources().length === pluginIds.length + extensionIds.length + 3` (now 30 + 3).
With the union at 30 those two assertions are unsatisfiable by any fixture value, so the hardcoded root
list had to follow the catalog:

```ts
const layout = runtime.demonstratorRuntimeModuleLayout([...new Set<string>(runtime.DEMONSTRATOR_RUNTIME_TARGETS.map((row: any) => row.pluginId))]);
```

That is the only line changed under `🧰️framework/…`; it uses the same `runtime.DEMONSTRATOR_RUNTIME_TARGETS`
accessor the function already uses five lines earlier, and the file had no peer edits pending
(`git status` clean for it at edit time).

While making the contract pass, a **pre-existing** drift in `🧫️cases.json` was also repaired: its
`extensionIds` was missing `cad-extension-aec-building`, `cad-extension-aec-building-energy`,
`cad-extension-aec-building-structure`, `cad-extension-spatial-shape` and the three `sourcing-module-*`
entries. That drift predates this slice (the cad/sourcing extensions were added to the closure earlier);
`testDemonstratorRuntime` was already red because of it.

## 4. Files changed

Demonstrator (`♻️mit-bestand/🧺️demonstrator/`):

| File | Change |
| --- | --- |
| `🔨️modules/🧩️runtime/🔣️.json` | `+ {variant: energy, runtimeVariant: energy}`, `+ {variant: fem3d, runtimeVariant: fem3d}` after `verfolgen` |
| `🔨️modules/🧩️runtime/🧫️pipeline.json` | `variants` += `energy`, `fem3d` (alphabetical); `development.activationTarget` → `@semio-tech/mit-bestand-demonstrator:activate-lanes-dev` |
| `🔨️modules/🧩️runtime/🧫️cases.json` | `additionalBuildVariants` → `["generation3d","energy","fem3d"]`; `pluginIds` += `energy`, `fem`; `extensionIds` repaired (pre-existing drift, §3) |
| `🔨️modules/🧩️runtime/🟦️.ts` | exports `demonstratorRuntimePluginId(variant)` (wraps the private resolver) for lane selection |
| `🔨️modules/🧩️runtime/♻️activation/🟦️.ts` | rewritten: lanes, `mergeDemonstratorActivationReceipts`, `publishDemonstratorUnionReceipt`, new `readDemonstratorActivation`; `demonstratorActivationComponents` unchanged; registers its unit tests |
| `🔨️modules/🧩️runtime/♻️activation/🌐️vite/🟦️.ts` | **new** — `demonstratorUnionReceiptVitePlugin`, node-only, multi-lane observer + republish |
| `🔨️modules/🧩️runtime/📜️script.ts` | activation log now `… 30 completed components from 3 lanes` |
| `🏗️builder/🌐️vite/🟦️.ts` | registers the union plugin before `semioActivationVitePlugin`; "six panes" comment → "eight panes" |
| `📋️project.json` | `prepare-dev`/`prepare-release` += energy + fem3d react targets; new `activate-lanes-dev` (`nx:noop`); `activate-dev.dependsOn` → `["prepare-dev", "@semio-tech/mit-bestand-demonstrator:activate-lanes-dev"]` |
| `🧪️tests/🎚️config/🟦️.ts` | `includeSource` += `./🔨️modules/🧩️runtime/♻️activation/🟦️.ts` |
| `🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts` | expectation → `["generation3d","energy","fem3d"]`, title → "three additional artifacts for eight pane runtime variants" |
| `🧪️tests/🧪️demonstratorunionreceipt/🟦️.ts` | **new** — 8 cases over lanes + merge |

Framework: one line in `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` (§3).

NOT touched (sibling-owned): `🪧️brand.ts`, `🟦️.tsx`, `🧪️tests/🎭️acceptance/🟦️.ts`,
`🔨️modules/🧪️e2e/🎚️config/🟦️.ts`, os-dev brand catalog.

## 5. Verification (all foreground, on this machine)

```
$ cd ♻️mit-bestand/🧺️demonstrator && bun ./📜️script.ts test quick
 Test Files  3 passed (3)
      Tests  13 passed (13)
```

The default (`fundamental`) level also passes warm (~10 s) but was **killed once at 15 s on a cold
transform cache** by the level budget; the third `includeSource` entry pulls the generated plugin catalog
into the program. Worth watching — if it flakes in CI, level-gate that `includeSource` entry.

```
$ bun -e '… publishDemonstratorUnionReceipt(…) …'
lanes: generator, energy, fem3d
dir: …/♻️mit-bestand/🧺️demonstrator/dist/♻️activation/dev
plugins: 30
ids: cad cad-extension-aec-building cad-extension-aec-building-energy cad-extension-aec-building-structure
     cad-extension-spatial-shape demonstrator energy fem flow flow-extension-bim flow-extension-brep
     flow-extension-dictionary flow-extension-draw flow-extension-list flow-extension-logic
     flow-extension-math flow-extension-primitive flow-extension-text gis procedural process
     process-extension-concrete process-extension-metal process-extension-robotic process-extension-wood
     puzzle sourcing sourcing-module-beams sourcing-module-slabs sourcing-module-windows
```

28 → **30** exactly as predicted.

Missing-lane error text (driven against a throwaway workspace carrying only the generator + energy lanes):

```
Missing Demonstrator activation lane receipt: fem3d (run bun nx run @semio-tech/framework-os-dev:activate-fem3d-react-dev)
```

Runtime scripts:

```
$ bun ./🔨️modules/🧩️runtime/📜️script.ts prepare dev
Prepared Demonstrator dev: 9 runtime variants          # was 7
$ bun ./🔨️modules/🧩️runtime/📜️script.ts activate
Activated Demonstrator dev: 30 completed components from 3 lanes
```

Framework contract:

```
$ bun <scratch>/contract.ts        # calls testDemonstratorRuntime(workspace)
[DEBUG] Demonstrator runtime catalog, full component union and 11-file pure import boundary PASS
CONTRACT PASS
```

Type-check (repo-style options — `allowImportingTsExtensions`, `types: ["node"]`, `strict`) over the
touched demonstrator files:

```
♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts(39,29): error TS2769: No overload matches this call.
♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts(81,45): error TS2339: Property 'dir' does not exist on type 'ImportMeta'.
```

Both **pre-existing and untouched by this slice**: the first is the framework's `OwnedBuildPlugin` vs
Vite's `PluginOption` mismatch coming out of `staticDirVitePlugin`/`browserArtifactVitePlugin`; the second
is the Bun-only `import.meta.dir` in an unmodified script. The new/edited modules
(`♻️activation/🟦️.ts`, `♻️activation/🌐️vite/🟦️.ts`, both test files, `🎚️config`, `🧩️runtime/🟦️.ts`)
report **zero** errors — the new registration site uses `import.meta.dirname` rather than
`import.meta.dir` on purpose.

## 6. On-disk state (for the coordinator)

Lane activation receipts under
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/<lane>/activation/🔣️receipt.json`:

| lane | present | variant / profile / rows |
| --- | --- | --- |
| `generator` | yes | generator / dev / 28 |
| `energy` | yes | energy / dev / 1 (`energy`) |
| `fem3d` | yes | fem3d / dev / 1 (`fem`) |
| `generation3d` | yes, but **stale vs generator** — unused by design (§1) | generation3d / dev / 11 |

Playground sessions: `…/🔌️plugin/📇️registry/dist/sessions/` contains **both** `energy` and `fem3d`, so
`PreparationScript` passes today without the coordinator running `plugin-registry:session-*`.

Staged modules: `…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/` contains `🔋️energy` and
`🏗️fem`.

Union receipt: published at `♻️mit-bestand/🧺️demonstrator/dist/♻️activation/dev/🔣️receipt.json`
(`semio.dev.activation/v1 generator dev 30`), gitignored.

## 7. Not verified here (out of slice)

* No dev server was started and no `activate-dev` / `prepare-dev` Nx target was run (coordinator owns that).
  `bun nx show project` did not return within 10 minutes on this machine — the same was true **before** these
  edits, so the Nx project-graph read was not used as a gate; `📋️project.json` is valid JSON and satisfies
  every `testDemonstratorRuntime` target assertion.
* The Playwright acceptance drift guard (`🧪️tests/🎭️acceptance/🟦️.ts`, `PANE_CASES`) is a sibling's file and
  is not part of the vitest suite run above; it will need its `energie`/`statik` cases before `test-e2e`
  is green.
* Neither pane was booted in a browser; nothing here proves the ENERGY or FEM-3D editors actually mount.
