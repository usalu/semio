# Explore: remodel dev-boot, registry/descriptor, TypeScript, storybook, launch.json

Scope: read-only static exploration, no cargo/bun build/dev/test run (host was at 63.3/64.5 GB swap,
load ~104 at ticket open). This report reuses boot-mechanics already nailed down verbatim by
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md` and
`…/☀️05/RASTER-PLUGIN-END-TO-END/📓️explore-raster-dev-boot-ts.md` (identical script/registry machinery,
independently re-verified line-for-line for remodel below) and only reports what is remodel-specific.

## 1. Dev boot: exact chain, env, ports, engines, plugin load closure

**Command chain** (file:line):
1. `package.json:96` — `"dev:remodel": "bun ./📜️script.ts dev remodel"`.
2. Root `📜️script.ts` `DevScript.run(["remodel"])` → `resolvePlaygroundDevApp` →
   `resolveFrameworkOsPlaygroundPlugin` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2477`)
   matches catalog row `variant === "remodel"` (no `aliases` declared) → `{ app: "remodel", rest: [] }`.
3. `runFrameworkOsPlaygroundDev("remodel", [])` spawns `bun nx run @semio-tech/framework-os-dev:dev --
   remodel` with env from `frameworkOsPlaygroundDevEnv` (`🟦️.ts:2490`): `SEMIO_PLUGIN=remodel`,
   **`SEMIO_RENDERER = env.SEMIO_RENDERER ?? "wgpu"`** — same universal default-renderer-is-wgpu finding
   as block/raster/procedural. A bare `bun run dev:remodel` boots native `trunk serve` wgpu, not react
   `ShellHost`, unless `SEMIO_RENDERER=react` is exported.
4. `@semio-tech/framework-os-dev`'s `project.json` `dev` target forwards to the inner dev-package
   `📜️script.ts` (`forwardAllArgs: true`). Env var mechanics are byte-identical to the BLOCK/RASTER
   findings: `SEMIO_PLUGIN_ONLY` only narrows `resolvePluginBuildTargets` (which crates get
   cargo-built), never the browser-side registry load closure; `SKIP_PLUGIN_BUILD`/`SKIP_ENGINE_BUILD`/
   `SEMIO_BUILD_BUDGET_MS`/`CARGO_TARGET_DIR` all behave exactly as documented in the two sibling
   reports (not re-derived here).

**Ports** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1097-1110`
(source: `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml:17-19`):

| variant | pluginId | react port | wgpu port |
|---|---|---|---|
| remodel | remodel | 6063 | 6163 |

Matches the task brief's `6063`/`6163` exactly. `REMODEL_PLAY_PORT` (both dev launch entries + seed
template) is dead — same vestigial pattern as `RASTER_PLAY_PORT`/`BLOCK_2D_PLAY_PORT`: not read anywhere
in the scripts; only `serverReadyAction`'s own hard-coded `6063`/`6163` regex makes it work. Real
override is `S_OS_PORT`.

**Engines**: remodel's Cargo.toml declares no `[[package.metadata.semio.playground]] engines = […]` row
(unlike raster's redundant `framework_surface` declaration) — `buildEngineWasm` still unconditionally
builds the three universal engines (`framework_surface`/node-graph, `framework_editor`, `flow-core`)
before any react boot; remodel needs nothing beyond those three. Not verified live (no build run); check
`.../🗺️surface/📦️packages/🦀️rust/🕸️bindings/` mtimes before assuming `SKIP_ENGINE_BUILD=1` is safe on a
given day.

**Plugin load closure** — generated registry entry
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json:1165-1183`):
```json
{ "pluginId": "remodel", "packageId": "semio:remodel",
  "cratePath": "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust",
  "capabilities": ["documents.write","ui.dialog"], "contributes": [], "consumes": [],
  "dependsOn": ["stdio"], "activationEvents": ["on-artifact-kind:3d.remodel"],
  "executionMode": "isolated" }
```
matches remodel's sole Cargo dependency on `semio-s-plugin-stdio`
(`✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml:24`). `stdio` has no further plugin-crate
dependency. **Load closure is exactly `{remodel, stdio}`** — same two-crate shape as block/raster.
`SEMIO_PLUGIN_ONLY=remodel` therefore carries the identical cold-boot risk documented in both sibling
reports (cascading `plugin.descriptor-unavailable` for both crates if `stdio` lacks a valid on-disk
build+descriptor) — not re-verified live here, but the mechanism is generic, not remodel-specific.

**`activationEvents: ["on-artifact-kind:3d.remodel"]` is STALE** — see §2, this is the pre-rename id, not
`3d.remodeling` (source truth, confirmed in §2).

## 2. Registry/descriptor: stale pre-rename ids confirmed, examples confirmed reachable but NOT surfaced

**Generated projections are gitignored** (`.gitignore:91` `**/🤖️generated/`, confirmed with
`git check-ignore -v` on both `🔌️plugins.json` and `🎠️playgrounds.json` — exit 0, both match) — they are
a local snapshot from whenever some peer session last ran `registry generate`, not source of truth.
`registryEntry` parsing (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:264-330`,
`readDescriptorJson` at `:237-248`) reads `capabilities`/`contributes`/`activationEvents`/`hashes`
**directly from `<cratePath>/../../🔣️.json`** (the owner-root descriptor) when present — so the generated
JSON's staleness is entirely downstream of the owner-root descriptor's own staleness, not a separate bug.

**Owner-root `🔣️.json` (557,512 bytes) and `🛂️.descriptor.semio` (122,617 bytes) — both tracked in git**,
last touched at commit `21fbcd3538` (2026-09-02 12:19:02, the rename commit itself). `git status
--porcelain` on the whole `✏️s/🔌️plugins/📸️remodel/` tree is currently **clean** (0 changed paths) — so
these are the actual committed HEAD state, not an in-flight edit.

**Confirmed stale pre-rename ids inside the committed owner-root descriptor** (`grep -o` over the raw
JSON, not assumed):
```
"3d.remodel"                    ×3   (should be "3d.remodeling" — artifact_kind().id, see below)
"s.remodel.remodel@1/*#editor"  ×9
"s.remodel.remodel@1/*#viewer"  ×7
```
Current source truth (`🦀️.rs:24-27`): `artifact_kind().id = "3d.remodeling"`; dialect at `🦀️.rs:180`:
`"s.remodel.remodeling.remodeling"`. This source file was last touched at commit `fe7c8a8f8b`
(2026-09-05 03:53:30) — three days after the descriptor was last regenerated. `describe` must be re-run
(`bun nx run @semio-tech/remodel-plugin:describe`) or a full un-narrowed dev boot taken (self-heals via
`materializePlugin`/`describeBuiltPlugin`/`stagePluginDescriptor`, per the BLOCK report) before trusting
either file.

**Descriptor's own `manifest.apps[].examples` field is `[]`** (only one `"examples"` key in the whole
557 KB file, `🔣️.json:16500`, nested per-app, value `[]`) — same shape RASTER's report found. This
matters because **`ShellHost`'s example switcher reads the descriptor's `examples`, not the registry
scan**: confirmed live —
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6418`
`return (activePluginManifest?.examples ?? [])…`. So even though both real examples are discoverable
on disk (below) and the generated playground catalog lists them, **remodel's react example picker will
show zero examples until the descriptor is regenerated** — this is a concrete, verified DoD-4 blocker
("examples load and switch"), not a hypothetical.

**Examples on disk — both found by the fixed `registryExampleCatalog` scan.** Generated
`🎠️playgrounds.json:1104-1107` lists `["🎬️demo","🎬️demo-session"]`, matching two real, distinct
locations:
- `🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/` (subset root)
- `🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/` (editor-surface level)

This is exactly the pattern `registryExampleCatalog`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:8799-8825`) was extended to scan
per the BLOCK ticket's 2026-09-05 fix (subset root AND `<subset>/<surfaceRole>/📚️examples`) — remodel is
a clean beneficiary of that fix, not a new gap. Both directories contain `🟦️.ts` (id/label/icon) +
`🦀️.rs` + `🧪️tests/` (§3); `demo` ships `🖼️assets/🗣️.dsl.semio`, `demo-session` ships `🖼️assets/🎮️.cmd.semio`
(confirmed only via its passing test, not independently re-opened).

## 3. TypeScript package — inert typed twins confirmed, zero real codecs

**`📦️packages/🟦️typescript/`** (unlike raster, this package.json is NOT a stale cross-plugin copy —
`description`/`scripts`/`dependencies` are all remodel-specific and consistent):
- `🟦️.ts` (2 lines) — real facade: `export * as remodeling_schema from
  "../../🗿️artifacts/📸️remodeling/…/🧬️schema/🟦️.ts"` + `remodeling_io` from the `🚪️io/🟦️.ts` barrel.
  Both paths exist.
- `📜️script.ts` (9 lines) — `TestScript` wraps `runVitest(root, rest, "🧪️tests/🟦️.ts")`.
- `🧪️tests/🟦️.ts` is a **vitest config**, not a test file: `include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts"]`,
  `passWithNoTests: false`. So `bun nx run @semio-tech/remodel-js:test`
  (`📋️project.json:1-14`, correctly wired, `cwd`'d to this package) runs exactly the example test files
  under `🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts` — i.e. the two files below.

**The only two tests that exist, both trivial "file is non-empty" checks** (verified content, not
assumed):
```ts
// 📚️examples/🎬️demo/🧪️tests/🟦️.ts
expect(readFileSync(join(here, "../🖼️assets/🗣️.dsl.semio"), "utf8").length).toBeGreaterThan(8);
// ✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts
expect(readFileSync(join(here, "../🖼️assets/🎮️.cmd.semio"), "utf8").length).toBeGreaterThan(8);
```
No parsing, no schema validation, no round-trip — identical thinness to raster's own example tests.

**46 `🟦️.ts` files under the artifact tree** (`find … -iname 🟦️.ts | wc -l` = 46, confirmed). Of these,
12 are literal one-to-two-line `export {};` stubs: the `🚪️io/🟦️.ts` barrel itself
(`/** 🚪️ IO facet barrel — WASM facades land in W7. */ export {};`), one editor-window options leaf
(`✏️editor/🎭️modes/🧊️model/…/☑️options/👁️layers/🟦️.ts`), and 10 of the per-stdio-kind serializer/
deserializer leaves under `🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/*/…/🟦️.ts`
(las/png/json/txt/stl/dwg/obj/gltf/ply). The remaining 34 files are non-stub but still pure type
declarations (`interface`/`type` only — schema, diff, mutations, snapshot, inference mirrors). **Net:
zero of the 46 files contain any executable logic** — `grep -l "function\|=>"` across all 46 returns
**only the two example test files** cited above; every other file is declarations only, no encode/decode
function anywhere.
- **Schema mirrors are real but shallow**: e.g. `🧬️schema/🟦️.ts` (68 lines) declares `RemodelingArtifact`
  with typed top-level fields but leaves `MediaStream`/`ImageAsset`/`CalibrationState`/
  `ReconstructionParams`/`GroundControlPoint`/`ReconstructionJob`/`ReconstructionResults` all as
  `{ [key: string]: unknown }` — untyped escape hatches, not full schema fidelity. The binary-facet files
  (`💾️binary/🟦️.ts` under `📸️snapshot`/`🔺️diff`/`🧬️mutations`/`💡️inferences`) are each 2 lines:
  `export type Remodeling<X>Binary = Uint8Array;` — a type alias, not a codec.

**Fixture-oracle gap (DoD-2 "validated against a third-party oracle").** Block's W3 pattern
(`.🧬semio/…/BLOCK-PLUGIN-END-TO-END/📓️w3-io.md:98-120`) needs: (a) TS leaf functions that actually
encode/decode, (b) a shared `🧪️tests/🧫️fixtures/*.json` asserted byte-for-byte from both a `bun test`
and a Rust `include_str!` unit test, (c) real Rust `Serializer`/`Deserializer` impls on the `io_mechanism`
channel. Remodel has none of the three: `🚪️io/🟦️.ts` is an explicit stub deferred to "W7", no
`🧫️fixtures/` directory exists anywhere under `🗿️artifacts/📸️remodeling/`, and the Rust io channel itself
is out of this report's scope (owned by the mutations/schema exploration agent).

## 4. Storybook — generic matrix coverage only, no dedicated scope or component fixture

- `.storybook/stories/framework/os/plugins.stories.tsx:53` — `export const Remodel: Story = { args: {
  plugin: "remodel" } };` — one row in the generated `PLUGIN_BUILD_TARGETS` whole-shell matrix (same
  mechanism raster documented), confirmed present. This boots `FrameworkOsShell` for `remodel` via
  `OsBootHost` inside Storybook.
- **No dedicated scope**: `.storybook/scopes.ts` has no `remodel` entry, and there is no
  `.storybook/stories/remodel/` directory at all (`find` returns nothing) — remodel has neither block's
  "scope resolves but empty" state nor raster's "component-level `Paint2dHost.stories.tsx` fixture"
  state. It is strictly behind raster: **zero remodel-specific stories of any kind**, only the generic
  matrix row.
- **`.storybook/s-end-to-end.spec.ts`** does not mention `remodel` (grep confirms) — this spec is
  `s`/`space`-specific only (`PLUGIN_HOST_CONFIGS`), matching the RASTER report's own generalization; it
  was never expected to cover remodel.
- **`.storybook/os-plugins.spec.ts`** does not name `remodel` literally either, but it iterates the
  generated `PLUGIN_BUILD_TARGETS` from `🧩️plugins.ts`
  (`.storybook/os-plugins.spec.ts:10,48`), and remodel IS present in that generated file
  (`🧩️plugins.ts:70`, confirmed) — so remodel gets the same automatic "reaches a deterministic boot
  outcome, zero unexpected console.error" coverage every other plugin gets, without needing a literal
  mention.
- **Gap vs BLOCK's W5** (13 stories for a `block` scope with real DSL fixtures, story-local reducers,
  mesh-catalog-resolved meshes — `📓️w5-storybook.md`): remodel has no equivalent. Its three windowKinds
  (`remodel-main`→world-3d, `remodel-frames`→canvas-2d, `remodel-report`→table, `remodel-view-model`→world-3d)
  all resolve to real, already-implemented Interpreter hosts (`🗣️Interpreter/🟦️.tsx:291-298`:
  `case "world-3d": World3dHost`, `case "table": TableHost`, `case "canvas-2d": Canvas2dHost`) — no
  missing host class, just nothing exercising them at the component level with a remodel-shaped fixture.

## 5. launch.json — three dev entries confirmed, TS-test entry missing (same gap as raster)

Confirmed at `.vscode/launch.json`:
- `:6734` `🛠️dev🏺️remodel⚛️react` — `bun run dev:remodel`, env `{REMODEL_PLAY_PORT: "6063", SEMIO_RENDERER:
  "react"}`, `serverReadyAction` pattern hard-codes `6063`.
- `:6754` `🛠️dev🏺️remodel🧊️wgpu🌐️wasm` — same command, env `{REMODEL_PLAY_PORT: "6163", SEMIO_RENDERER:
  "wgpu"}`, pattern hard-codes `6163`.
- `:6774` `🛠️dev🏺️remodel🧊️wgpu🖥️native` — `bun
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts
  native remodel`, no env/serverReadyAction (matches the native-target shape for every other plugin).

Both `REMODEL_PLAY_PORT` occurrences are dead (§1) — cosmetic only, `serverReadyAction`'s literal regex
does the real work. `.vscode/🧩️launch.seed.jsonc:5286-5297,7853-7869` is the generator source for these
three (`@generated:remodel:react`/`@generated:remodel:wgpu` template rows plus the native block and the
`"remodel": {...}` template object) — the seed and the generated file agree.

**Missing vs. other plugins**: a `🧪️test🏺️remodel📚️examples` (`bun nx run @semio-tech/remodel-js:test`)
launch entry does not exist for either raster or remodel, while it DOES exist for seven others with a
TS-example test target: `draw` (`:7785`), `shooting` (`:7792`), `note` (`:7799`), `procedural` (`:7806`),
`block` (`:7813`), `mathematical` (`:7820`), `trinity` (`:7827`) — all `"🧪️test<emoji><id>📚️examples"` →
`bun nx run @semio-tech/<id>-js:test`. No `describe`-specific launch entry exists for any plugin's TS side
(that nx target is Rust-only, wired per-crate) — not a gap, matches every other plugin.

## Prioritized gaps (dev-boot/registry/TS/storybook/launch surface only)

1. **[HIGH, verified]** Owner-root `🔣️.json`/`🛂️.descriptor.semio` are stale relative to source by 3 days
   (rename commit 2026-09-02 vs. source touch 2026-09-05): contain `3d.remodel`/`s.remodel.remodel@1/*#…`
   instead of current `3d.remodeling`. Directly blocks DoD-3 ("registry check accepts remodel" — not
   verified here whether `registry check` actually rejects the mismatch, but the ids plainly disagree)
   and DoD-4's example-switching (next point). Fix: `bun nx run
   @semio-tech/remodel-plugin:describe` (mirroring raster's dedicated fix path) or a full un-narrowed dev
   boot.
2. **[HIGH, verified]** `ShellHost`'s example picker reads `PluginManifest.examples`
   (`🏛️ShellHost/🟦️.tsx:6418`), which is `[]` in the current descriptor — remodel's react playground will
   show **zero** switchable examples today even though both `demo`/`demo-session` are real and
   registry-discoverable. Same fix as #1 resolves this if `describe`/regeneration also repopulates
   `manifest.apps[].examples` (not independently confirmed — worth a follow-up check after regeneration).
3. **[MEDIUM, verified]** TypeScript side is entirely type-only: 44 of 46 `🟦️.ts` files under the artifact
   tree contain no function; the `🚪️io/🟦️.ts` barrel is an explicit `export {}` stub deferred to "W7". No
   fixture oracle infrastructure (`🧫️fixtures/`) exists. Needed for DoD-2's "validated against a
   third-party oracle" — modeled on block's W3 (`📓️w3-io.md`).
4. **[LOW, verified]** No dedicated storybook scope/stories for remodel (raster at least has one
   component-level fixture; remodel has neither). Not required for DoD but closes the same gap block's W5
   closed.
5. **[LOW, verified]** `🧪️test🏺️remodel📚️examples` launch.json entry missing (shared gap with raster,
   not remodel-specific).
6. **[COSMETIC, verified]** `REMODEL_PLAY_PORT` env var in both dev launch entries and the seed template
   is dead code — harmless today only because `serverReadyAction`'s regex hard-codes the real ports.

## Unverified (out of scope / blocked by read-only constraint)

- Whether `registry check` actually fails on remodel's id mismatch, or whether a live `dev remodel` boot
  404s/degrades gracefully given the stale descriptor — not run (no build allowed; host swap/load state).
- Rust-side io channel real-vs-stub status (`✏️editor`/`👁️viewer` impls) — owned by the mutations/schema
  and editor/dispatch exploration agents, not re-derived here.
- `demo-session`'s `🖼️assets/` contents beyond the one file its test reads (`🎮️.cmd.semio`, length > 8) —
  not independently opened.

## Files referenced (key files; full set is inline above with file:line)

- `/Users/ueli/Documents/semio/package.json`, `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/.vscode/launch.json`, `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (+ its `🤖️generated/` outputs)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🔣️.json`, `🛂️.descriptor.semio`, `🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/` (package.json, project.json, script.ts, 🟦️.ts, tests)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/` (🚪️io, 🧬️schema, both examples)
- `.storybook/stories/framework/os/plugins.stories.tsx`, `.storybook/os-plugins.spec.ts`, `.storybook/s-end-to-end.spec.ts`, `.storybook/scopes.ts`
- Sibling reports: BLOCK's `📓️explore-dev-boot-path.md` / `📓️w3-io.md` / `📓️w5-storybook.md`, RASTER's `📓️explore-raster-dev-boot-ts.md` (all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/`)
