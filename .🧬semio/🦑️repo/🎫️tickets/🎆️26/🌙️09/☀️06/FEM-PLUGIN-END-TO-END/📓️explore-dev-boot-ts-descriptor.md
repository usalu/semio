# Explore: fem plugin — dev boot path, descriptor/registry gates, TS side, IO, storybook, launch.json, playground manifest

Scope: read-only static exploration (no cargo/bun build/dev server run). Repo date at exploration time:
2026-09-06. This reuses the boot-mechanics facts already nailed down for `block`/`raster`
(`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md`,
`📓️w2-block5d-boot.md`, `.../RASTER-PLUGIN-END-TO-END/📓️explore-raster-dev-boot-ts.md`) — same machinery,
different crate. fem-specific facts below were independently re-verified against the current tree.

## 1. Dev boot path

**Command chain** (file:line, current tree):

1. `package.json:86-87` — `"dev:fem:2d": "bun ./📜️script.ts dev fem 2d"`, `"dev:fem:3d": "bun ./📜️script.ts dev fem 3d"`.
2. Root `📜️script.ts:476-515` `DevScript.run(["fem","2d"])` → `resolvePlaygroundDevApp` (`:177-195`) →
   `resolveFrameworkOsPlaygroundPlugin` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2477-2489`)
   matches fem's Cargo-declared `aliases = ["fem 2d"]` against the generated catalog, returns `{ app: "fem2d", rest: [] }`.
3. `runFrameworkOsPlaygroundDev("fem2d", [])` (`📜️script.ts:196-…`) spawns
   `bun nx run @semio-tech/framework-os-dev:dev -- fem2d` with env from `frameworkOsPlaygroundDevEnv`
   (`🟦️.ts:2490-…`): `SEMIO_PLUGIN=fem2d`, **`SEMIO_RENDERER = env.SEMIO_RENDERER ?? "wgpu"`**,
   `S_OS_PORT = env.S_OS_PORT || <catalog default port>`. **Default renderer is wgpu**, identical to
   block/raster — a bare `bun run dev:fem:2d` boots the native `trunk serve` wgpu path, not the react
   `ShellHost`, unless `SEMIO_RENDERER=react` (or the `served` alias) is set.
4. `@semio-tech/framework-os-dev`'s `project.json` `dev` target (`forwardAllArgs: true`) forwards `fem2d`
   into the inner `DevScript` at
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`, which resolves
   `renderer = process.env.SEMIO_RENDERER ?? "react"` — moot for the bare invocation since step 3 already
   fixed `SEMIO_RENDERER="wgpu"`.
   - **wgpu path** (default): probes/serves `http://127.0.0.1:6186/?plugin=fem2d` (fem2d wgpu port) via
     the native trunk-serve target script.
   - **react path** (`SEMIO_RENDERER=react`): registry regenerate → engine wasm build → `buildPlugins`
     builds `{fem, stdio}` → Vite serves `http://127.0.0.1:6086/` (fem2d react port), plugin crates stream
     in afterward.

**Ports** — `Cargo.toml` (`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml:16-26`) and the generated
`🎠️playgrounds.json:599-634`:

| variant | app id | react port | wgpu port |
|---|---|---|---|
| fem2d | `s.fem.fem2d@1/*#editor` | 6086 | 6186 |
| fem3d | `s.fem.fem3d@1/*#editor` | 6087 | 6187 |

**Env vars** — mechanics are byte-identical to block/raster (same TS functions, no fem-specific branching):
`SEMIO_PLUGIN_ONLY` narrows only which crates get cargo-**built** (not which registry entries the browser
tries to load); `SKIP_PLUGIN_BUILD` only meaningful with `SEMIO_RENDERER=react`; `SKIP_ENGINE_BUILD` is a
no-op for wgpu, and for react skips the 3 universal engine builds (`framework_surface`, `framework_editor`,
`flow-core`) which fem's Cargo.toml declares no additional `engines` for; `CARGO_TARGET_DIR` isolates
cargo's own target dir but is not injected by the script — export it yourself; `SEMIO_BUILD_BUDGET_MS`
overrides the default 20-minute (`1_200_000`ms) per-cargo-step budget; `CARGO_PROFILE_WASM_DEV_DEBUG` is a
plain Cargo env override, currently moot since `Cargo.toml:251-253`'s `[profile.wasm-dev]` already inherits
`debug = false` from `[profile.dev]` (committed, per project memory
`project-wasm-dev-profile-debug-off-and-swap-thrash`).

**Plugin load closure**: fem's generated registry entry
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json:300-325`):
```json
{ "pluginId": "fem", "packageId": "semio:fem", "contributes": [], "consumes": [], "dependsOn": ["stdio"],
  "activationEvents": ["on-artifact-kind:computation.fem2d", "on-artifact-kind:computation.fem3d"],
  "executionMode": "isolated" }
```
matches `Cargo.toml:41` (`semio-s-plugin-stdio` path dependency). `stdio` has no further plugin-crate
dependency, so the closure is exactly **`{fem, stdio}`** — same two-crate shape as block/raster.
`SEMIO_PLUGIN_ONLY=fem` therefore has the SAME risk block/raster's tickets documented: it only narrows
cargo-build targets, not the browser-side load list; on a cold tree (stdio not yet built with a valid
descriptor) it can cascade both `fem` and `stdio` to `plugin.descriptor-unavailable`.

**Boot-outcome verification convention** (from the block/raster recipes, applies identically to fem): poll
a real HTTP port (`curl`/`fetch http://127.0.0.1:6086/` react, `.../6186/?plugin=fem2d` wgpu), not a log
file; the `ShellHost`'s `data-semio-os-ready`/`data-semio-os-error` beacon on `<html>` is the authoritative
"did fem boot" signal for a react session (same mechanism `s-end-to-end.spec.ts`/`os-plugins.spec.ts`
read); `[DEBUG]`-prefixed console logs are the project's temporary-verification convention.

**Concrete recipe** (mirroring the converged block/raster pattern):
```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=target-fem
export SEMIO_RENDERER=react
export SEMIO_BUILD_BUDGET_MS=3600000
bun ./📜️script.ts dev fem 2d      # or: dev fem 3d
```
First boot must NOT set `SKIP_PLUGIN_BUILD`/cold `SEMIO_PLUGIN_ONLY=fem` — let it do a full, un-narrowed
build of `{fem, stdio}` at least once (see §2: fem's served wasm is currently badly stale). Poll
`http://127.0.0.1:6086/`. For the bare wgpu default: drop `SEMIO_RENDERER`, poll
`http://127.0.0.1:6186/?plugin=fem2d`.

## 2. Descriptor and registry gates

**Owner-root descriptor**: fem HAS both
`✏️s/🔌️plugins/🏗️fem/🔣️.json` (373,210 bytes) and `✏️s/🔌️plugins/🏗️fem/🛂️.descriptor.semio` (87,549 bytes) —
unlike block/stdio (which never had one). Both were **last committed at `21fbcd3538`, 2026-09-02 12:19:02**
(`git log --date=iso` on both paths returns only that one commit).

**Staleness, verified by hash, not assumed**: fem source has changed substantially since that commit — the
newest real-source mtimes under the crate are `📦️packages/🦀️rust/🦀️.rs` (plugin root, Sep 5 05:32,
127,395 bytes) and `🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` (Sep 5 06:03, 87,632 bytes; git blames this range to
the repo's own auto-commit train ending `3a6a9d6bfc`, 2026-09-05 22:02:04). The **served** plugin-module
build is even more stale:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🏗️fem/semio_s_plugin_fem_component.core.wasm`
  — mtime **2026-08-17 18:10** (its `.js` wrapper refreshed 2026-09-04 17:14, its own `🛂️.descriptor.semio`
  refreshed 2026-09-06 00:26, its `🔣️.json` refreshed 2026-09-04 11:17 — three different staleness
  generations layered on one served directory, exactly the pattern the RASTER ticket found).
- `sha256(served .core.wasm) = 4ee336f00e79471fa2cd7f225c5961560c0f97787503165dd459b706ba0f5fae`, but the
  registry's own recorded hash
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json:320-323`
  `coreWasmSha256`) is `924176ed3c2bd2415f14218d6671a485db3d06931f2b47e67c5170f715661e13` — **mismatch,
  confirmed**. The served fem wasm is a ~3-week-old leftover, not what the current registry believes is live.

**Registry `check`/`generate`** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`)
is the same hard gate documented in the block/raster tickets: `ensurePluginRegistry` runs it before
anything else boots and throws on non-zero exit; it reads `🔣️taxonomy.json` for path-emoji/contract
validation. Not fem-specific, but any concurrent taxonomy break blocks fem's boot the same way.

**Is fem in the playground catalogs?** Yes, both variants:
`🎠️playgrounds.json:599-634` — `fem2d`/`fem3d` rows with `pluginId: "fem"`, `cratePath`,
`app: "s.fem.fem2d@1/*#editor"`/`"s.fem.fem3d@1/*#editor"`, `aliases: ["fem 2d"]`/`["fem 3d"]`,
`ports`, `examples: ["🎬️demo", "🎬️demo-session"]`, `engines: []`.

**Does `registryExampleCatalog` find fem's examples?** Yes, both. fem2d's example tree has two entries:
`🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/` (subset root, `🦀️.rs`+`🟦️.ts`+
`🧪️tests/🟦️.ts`) and `🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/`
(editor-surface level). `registryExampleCatalog`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:8799-8825`) scans exactly these two
levels (plus per-surface-role dirs) per the fix the BLOCK ticket's coordinator landed 2026-09-05 13:30 —
both fem examples surface correctly in the generated catalog, matching raster's already-reconciled case
(the same generation the BLOCK ticket originally found broken for block2d/block3d's `demo-session`).
fem3d mirrors fem2d exactly (own `demo`/`demo-session` pair under the analogous `🧊️3d` tree).

**Shell's example switcher**: per the block/raster tickets' shared finding, `ShellHost` reads
`PluginManifest.examples` (the descriptor's own field), not the registry catalog — worth re-checking once
fem's stale descriptor (above) is regenerated, since the manifest's own `examples` field could be a
different, possibly-empty snapshot (raster hit exactly this: descriptor said `examples: []` while the
registry scan found two).

## 3. TypeScript side

**File census** (79 `.ts`/`.tsx` files total under fem, excluding `node_modules`; full list gathered via
`find`). Breakdown by shape:

- `📦️packages/🟦️typescript/🟦️.ts` (12 lines) — a real, correct facade: `export * as fem2d_schema`/
  `fem2d_snapshot`/`_text`/`_binary`/`_diff`/…/`fem2d_io`, and the same 8 exports for fem3d — 20 named
  re-exports total, all 20 target paths verified to exist on disk.
- `📦️packages/🟦️typescript/📜️script.ts` — real: `TestScript` wires `bun ./📜️script.ts test` →
  `runVitest(this.root, rest, "🧪️tests/🟦️.ts")`.
- `📦️packages/🟦️typescript/🧪️tests/🟦️.ts` — a real vitest config (`defineConfig`), `include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts"]`,
  `passWithNoTests: false` — so it discovers every example's own test file dynamically (4 total: fem2d
  demo, fem2d demo-session, fem3d demo, fem3d demo-session), not a hardcoded pair like raster's
  `script.ts`.
- Every `🧬️schema/**/🟦️.ts` under both artifacts (snapshot/diff/mutations × text/binary, inferences,
  bounds) is an **inert typed twin** — e.g. `🧬️schema/📸️snapshot/📝️text/🟦️.ts` (2 lines):
  `export type Fem2dSnapshotText = string;`. Same for every `✏️editor/🎭️modes/…/🪟️windows/*/🟦️.ts` window
  twin — e.g. `✏️editor/…/🪟️windows/🧱️model/🟦️.ts` (11 lines): an interface + two string-literal const ids,
  no rendering logic — "typed twin of `🦀️.rs`'s `render(doc, camera)` boundary" per its own docstring.
  Matches the BLOCK ticket's "232 TS files are inert typed twins" finding for fem's own TS surface.
- `📚️examples/🎬️demo/🧪️tests/🟦️.ts` and `…/demo-session/🧪️tests/🟦️.ts` (both artifacts) — thin, real
  vitest specs asserting the shipped `🖼️assets/🗣️.dsl.semio` (or `.cmd.semio`) file is non-trivially long
  (>8 bytes) — same thin-but-genuine pattern raster's demo-session test showed.
- `🚪️io/📤️export/🧵️serializers/…/🟦️.ts` and `📥️import/🧩️deserializers/…/🟦️.ts` leaves (csv/md/json/txt/stl/obj,
  ×2 artifacts) — not inspected byte-for-byte here, but re-exported wholesale by the top-level
  `fem2d_io`/`fem3d_io` facade entries; not part of the `🟦️.ts` inert-twin family surveyed above (these are
  the TS side of the IO leaves discussed in §4).

**`package.json` is a near-verbatim `cad-js` copy, same defect class as block/raster**:
`✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/package.json` — `name` correctly renamed to
`@semio-tech/fem-js`, but:
- `description` is **verbatim CAD text**: "📐️ CAD plugin TS: spatial factory runtime/model graph (core), R3F
  renderer, brepjs kernel, construct query language, XState machine adapter, and the runtime composition
  root — folded from the 6 former cad-js-* packages." — none of this applies to fem.
- `scripts.generate`/`scripts.fixture` literally read `"bun nx run @semio-tech/cad-js:generate"` /
  `"...cad-js:fixture"` — invoke **cad's** nx project, not fem's. (`scripts.test` is the one script that
  WAS correctly repointed to `@semio-tech/fem-js:test`.)
- `dependencies` still lists all four CAD-specific extension packages
  (`@semio-tech/cad-js-module-spatial-shape`, `-aec-building`, `-aec-building-energy`,
  `-aec-building-structure`) plus `@semio-tech/s-3d-js`/`infinite-world-r3f`/etc. — none of these are used
  by fem's actual code (the real facade only imports from fem's own `🗿️artifacts/` tree).
- Only `repository.directory` was correctly updated.

**Is this a live break?** No — same conclusion as raster: nx reads `📋️project.json`'s `targets`, not
`package.json`'s `scripts`, and fem's own `📋️project.json`
(`✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📋️project.json:1-14`) correctly wires exactly one target,
`test` → `bun ./📜️script.ts test` in fem's own directory. So `bun nx run @semio-tech/fem-js:test` genuinely
runs fem's own tests; the wrong `package.json` only misleads a human reading it directly or a raw
`bun run <script>` invoked outside nx. Recommend fixing description/scripts/dependencies regardless.

## 4. IO — fem is on the OLD `ComposerEntry` channel, self-documented as an honest gap

fem2d/fem3d have **not** migrated to the newer `io_mechanism`/`IoDeclaration.entries` registry that `note`
(and block's target state) use — this is explicitly self-flagged in two places, matching the exact defect
class the BLOCK ticket found ("io unregistered on the io mechanism in all subsets"):

1. **Subset root** `🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🦀️.rs:11-26` (module doc, verbatim):
   > "⚠️ DEVIATION from the `🗒️note`/`🖍️draw` template… fem2d's own `🚪️io/🦀️.rs` is still on the OLD
   > `ComposerEntry`/`io_registry` channel (six composer rows: `fem2d`, `csv`, `md`, `json`, `stl`, `obj`)…
   > `io_declaration()` below is the same `IoDeclaration` shape, built here instead: `native` is real
   > […] but `entries: &[]` — the foreign-format hops stay UNREGISTERED on the new `io_mechanism` channel
   > (an honest gap, not an oversight…)."
   Its `io_declaration()` (`:49-61`) wires only `native` (snapshot/diff/mutations text+binary via
   `pilot_languages()`, all real); `entries: &[]`. fem3d's own subset root carries the identical doc
   comment and shape.
2. **Old-channel file** `🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🦀️.rs:1-14` (module doc):
   registration "flows through 🎹️composer::register (called once from ⚙️engine::register), not per-leaf
   register()" — this is the **deprecated** composer-entry mechanism note's own docstring says was
   "replaced outright" by `io_mechanism`. Confirmed dead-end: repo-wide grep found **no** call site for
   `composer::register`/`engine::register`/`io_registry::entries` anywhere in the fem crate outside this
   file's own comment and a matching bug-tracking comment in the artifact root (`◻️2d/🦀️.rs:317`,
   `🧊️3d/🦀️.rs:236`, both "`io_registry::entries()` registers SEVEN composer rows, not five/six" — a
   D2-capability-claim-repairs bookkeeping note, not a live registration call). fem's own `⚙️engine/` dir
   (`🖥️app-surface/🦀️.rs`, the only file in it) does not call this either — repo-wide grep for
   `composer::register`/`register_composer_entries`/`io_registry::entries` calls found zero hits in fem.
   **Net effect: the old `ComposerEntry` table (`entries()` in this file) is dead code — nothing calls
   it — and the new `io_mechanism` channel has zero registered foreign-format entries for fem2d/fem3d.**

**Formats declared** (`import_stdio_kinds()`/`export_stdio_kinds()`, both artifacts' `🚪️io/🦀️.rs:9-13`):
```
import: stdio.csv, stdio.json, stdio.md, stdio.txt
export: stdio.csv, stdio.json, stdio.md, stdio.obj, stdio.stl, stdio.txt
```
`stl`/`obj` are **export-only, real geometry** (triangulated/extruded `FemRegion` footprints via
`engine::meshing::build_semio_mesh_snapshot`, not fabricated bytes) — the module doc explains there is no
honest IMPORT direction since an arbitrary mesh carries no `FemMaterial`/`FemSection`/`FemSupport`/
`FemLoadCase` to reconstruct a snapshot from. `stdio.zip`/`stdio.png` were **deleted outright** in both
directions (module doc references "ticket w5a--report.md's `stdio_gaps`/rationale" — the same
zip/png-retirement rationale the BLOCK ticket's own io report independently found for block).

**Multiple "subset" directories are NOT real registered subsets.** `🪆️subsets/` under both fem2d and fem3d
contains, besides `🌐️any`, sibling directories `🏋️load`, `🧱️material`, `🕸️mesh`, `🛡️boundary`,
`📈️analysis`. These are **not** compiled Rust modules — a repo-wide grep found no `#[path]` mount or
`subsets::{load,material,mesh,boundary,analysis}` module reference anywhere in the crate; only `🌐️any` is
wired (the `🌐️any/🦀️.rs` subset root's own doc comment says outright: "the only subset this artifact
has"). Their contents are exclusively cross-implementation oracle/fixture/test material (`.py`, `.feature`,
`fixtures/*.json`, `🔮️oracle/🔣️.json`) organized by mutation-category name, matching test dirs of the same
name that live inside `🌐️any/🧪️tests/` (e.g. `🌐️any/🧪️tests/🏋️mutate-fem2d-1-any-load/🦀️.rs`). Not a
registry-discovery gap — just non-Rust oracle scaffolding, correctly outside the compiled module tree.

## 5. Storybook

`.storybook/stories/framework/os/plugins.stories.tsx:37,82` — `export const Fem: Story = { args: { plugin:
"fem" } satisfies OsBootHostProps };`, included in the generic `PLUGIN_BUILD_TARGETS`-matrix export list.
This is the same whole-shell `OsBootHost` coverage every registered plugin crate automatically gets (per
the raster ticket's finding of the same story file) — no fem-specific edit was needed for this to exist,
and no fem-specific scope/story was added beyond it.

**No dedicated fem scope or component-level story exists** — `fem` has no `[package.metadata.semio.storybook]`
section in its `Cargo.toml` (grepped: zero hits), so `.storybook/scopes.ts`'s generated-scope mechanism
(the one the BLOCK ticket's W5 used, keyed on that Cargo metadata) produces no `fem`-specific scope, and
there is no `.storybook/stories/fem/` directory. Unlike raster (which got a bespoke `Paint2dHost.stories.tsx`
covering its shared wasm session engine), fem's editor/viewer windows are plain typed-twin TS surfaces
(§3) with no `ComponentSceneHost`-style host mounting them yet — so there is nothing analogous to wire a
component-level fixture against without first identifying which renderer host, if any, fem's windowKinds
resolve to (not established in this exploration; would need the `resolveComponentSceneHost` switch in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` checked against
fem's descriptor's windowKinds/surfaceKinds).

**What BLOCK's W5 did that fem would need** (from `📓️w5-storybook.md`): add a
`[package.metadata.semio.storybook]` opt-in to `Cargo.toml` (`id`/`titlePrefix`/`sourceRoots`), then author
real story files under `.storybook/stories/<id>/**` that parse the shipped example DSL via a story-local
reader (`?raw` import), project it into the host's expected scene-node shape, and — per block's documented
"honest limitation" — check whether `.storybook/preview.tsx`'s `WASM_LOADERS` map has an entry for
whichever `surfaceKind` fem's editor/viewer windows declare; if not, the canvas half renders empty and the
story must fall back to a text/debug panel of the window's own `render()` output (block's `board2d`
precedent) until a session-factory is registered.

## 6. launch.json

fem has exactly the same shape of entries block/raster have — 6 dev entries, no test/describe entries:

| Entry name | Command |
|---|---|
| `🛠️dev🏗️fem🩻️2d⚛️react` | `bun run dev:fem:2d` (line 7588-7591) |
| `🛠️dev🏗️fem🩻️2d🧊️wgpu🌐️wasm` | `bun run dev:fem:2d` (line 7608-7611) |
| `🛠️dev🏗️fem🩻️2d🧊️wgpu🖥️native` | `bun .../🎯️targets/🧊️wgpu/.../📜️script.ts native fem2d` (line 7628-7631) |
| `🛠️dev🏗️fem🏙️3d⚛️react` | `bun run dev:fem:3d` (line 7639-7642) |
| `🛠️dev🏗️fem🏙️3d🧊️wgpu🌐️wasm` | `bun run dev:fem:3d` (line 7659-7662) |
| `🛠️dev🏗️fem🏙️3d🧊️wgpu🖥️native` | `bun .../🎯️targets/🧊️wgpu/.../📜️script.ts native fem3d` (line 7679-7682) |

(react vs wgpu-wasm entries both run the identical `bun run dev:fem:2d`/`:3d` command — the two configs
differ only in which env vars VS Code's launch config sets around it, e.g. `SEMIO_RENDERER`, mirroring the
same pattern block/raster/every other plugin uses.) There is also an unrelated hit,
`📦️preview🤖️ticket-important-fem-handoff` (line 8253) — a different, ticket-handoff preview task, not a
dev/test/describe entry for the plugin itself.

**No `describe`/`test` launch.json entries** — checked whether this is a fem-specific gap: a repo-wide
grep for `"describe"` in `.vscode/launch.json` returns exactly **one** hit total
(`@semio-tech/space-plugin:describe`, line 5437) across the entire file. So the absence of fem
test/describe launch entries is the norm for essentially every plugin (block/raster also have none), not
something specific to fem. fem's own `📦️packages/🦀️rust/📋️project.json` DOES wire real nx targets for all
of these (`test`, `test-quick`, `test-long`, `test-exhaustive`, `describe` — all `cwd`'d correctly to fem's
rust package dir, invoking fem's own `📜️script.ts`), so `bun nx run @semio-tech/fem-plugin:describe` etc.
work fine even though nothing in `launch.json` runs them for a developer via the debugger UI.

## 7. Playground manifest vs. artifact kind ids

`Cargo.toml`'s `[[package.metadata.semio.playground]]` rows (`:16-26`):
```
variant = "fem2d"  app = "s.fem.fem2d@1/*#editor"  aliases = ["fem 2d"]  ports = { react = 6086, wgpu = 6186 }
variant = "fem3d"  app = "s.fem.fem3d@1/*#editor"  aliases = ["fem 3d"]  ports = { react = 6087, wgpu = 6187 }
```

**App id derivation matches**, following the same `surface_app_id(dialect, role)` formula the BLOCK ticket
traced (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`): fem2d's `Dialect { artifact_kind: "s.fem.fem2d",
standard: StandardId("1"), subset: SubsetId::ANY }` (seen in the subset root's `FEM2D_DIALECT` const,
`◻️2d/…/🚪️io/🦀️.rs:21` `derived_composition` mirror; the canonical const lives at
`crate::artifacts::fem2d::FEM2D_DIALECT`) → coordinate `"s.fem.fem2d@1/*"`, role `editor` →
`"s.fem.fem2d@1/*#editor"` — exactly the Cargo.toml string. Registration:
`✏️s/🔌️plugins/🏗️fem/🦀️.rs:31-45`'s `plugin()` calls `.declare_artifact(crate::artifacts::fem2d::artifact())`
+ `.declare_artifact(crate::artifacts::fem3d::artifact())`, `.editor_mutation_roster::<Fem2dPlayApp>()` /
`Fem3dPlayApp`, and `.activation(ActivationEvent::OnArtifactKind { kind:
crate::artifacts::fem2d::computation_artifact_kind().id })` / fem3d's counterpart.

**Two different id namespaces, both correct, not conflated** (same two-id pattern the BLOCK ticket found
for block): the app-id/Dialect coordinate (`s.fem.fem2d`, used for `#editor`/`#viewer` surface routing)
is a **different string** from the `ArtifactKindSpec.id` used for activation
(`computation_artifact_kind()` in `◻️2d/🦀️.rs` returns `id: "computation.fem2d".into()`, `dimension:
"computation"`, `media_type: {class: Computation, form: Value}` — this is the OS-catalog-level "computed
FEM results" resource kind, deliberately distinct from the raw Dialect coordinate). This exactly matches
the generated registry's `activationEvents: ["on-artifact-kind:computation.fem2d",
"on-artifact-kind:computation.fem3d"]` (`🔌️plugins.json:314-317`) — both ids read live from the crate's
own functions, never hardcoded, consistent with the plugin root's own doc comment.

**`Plugin::builder`/component-package identity check** (per project memory
`project-plugin-id-drift-builder-vs-component`): `Plugin::<FemApps>::builder("fem")` (`🦀️.rs:33`) +
`.package_id("semio:fem")` (`:35`) vs. `Cargo.toml`'s `[package.metadata.component] package = "semio:fem"`
(`:9-10`) — **match, no drift**.

## Gaps to fix (prioritized)

1. **Served fem plugin wasm is stale and hash-mismatched against the registry** (`§2`) — sha256 of the
   on-disk `.core.wasm` under `🔌️plugin-modules/🏗️fem/` does not match the registry's recorded
   `coreWasmSha256`, and the wasm mtime (2026-08-17) predates ~3 weeks of source changes (up to
   2026-09-05). **First boot of any dev session on fem must be a full, un-narrowed build** (no
   `SKIP_PLUGIN_BUILD`, no cold `SEMIO_PLUGIN_ONLY=fem`) — do not trust a `SKIP_PLUGIN_BUILD=1` serve-only
   boot until this is rebuilt.
2. **io_mechanism migration incomplete, self-documented** (`§4`) — fem2d/fem3d's foreign-format hops
   (csv/json/md/txt/stl/obj) are unregistered on the new `io_mechanism` channel; the old `ComposerEntry`
   table in `🚪️io/🦀️.rs` is dead code (nothing calls it). Native codec (snapshot/diff/mutations) IS real
   and wired. Closing this means hand-authoring six typed `Serializer`/`Deserializer<Fem2dSnapshot>` (and
   the fem3d equivalents) impls per the `note`/block-target template — matches the exact remediation shape
   block's own W3 packet used.
3. **`📦️packages/🟦️typescript/package.json` is a misleading verbatim `cad-js` copy** (`§3`) —
   description/`generate`/`fixture` scripts/four CAD-specific dependencies are all wrong for fem, though
   harmless in practice since nx dispatches via `project.json` not `package.json`. Cheap, low-risk fix:
   rewrite description, repoint `generate`/`fixture` to `@semio-tech/fem-js` (or drop them if fem has no
   such targets), and drop the CAD dependency list.
4. **No dedicated fem Storybook coverage** (`§5`) — only the generic whole-shell `OsBootHost` matrix entry
   exists; no `[package.metadata.semio.storybook]` opt-in, no `.storybook/stories/fem/` scope, no
   component-level host fixture (unlike raster's `Paint2dHost.stories.tsx`). Before authoring one, first
   determine which `surfaceKind`/`ComponentSceneHost` fem's windowKinds resolve to (not established here)
   and whether `.storybook/preview.tsx`'s `WASM_LOADERS` has an entry for it — if not, expect the same
   "canvas renders empty, fall back to a text/debug render() panel" limitation block's W5 hit for board-2d.
5. **Descriptor pair (`🔣️.json`/`🛂️.descriptor.semio`) not regenerated since 2026-09-02**, ~3 days behind
   current source — low-risk to fix directly: `bun nx run @semio-tech/fem-plugin:describe` (wired in
   `📦️packages/🦀️rust/📋️project.json`) regenerates both without booting the whole dev server, the same
   fix path raster's ticket used.

## Files referenced (absolute paths)

- `/Users/ueli/Documents/semio/package.json`
- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/.vscode/launch.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🏗️fem/` (served build, stale)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🛂️.descriptor.semio`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📋️project.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/package.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📋️project.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📜️script.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🟦️.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🧪️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` (comparison source, new `io_mechanism` pattern)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json` (comparison source)
- `/Users/ueli/Documents/semio/.storybook/stories/framework/os/plugins.stories.tsx`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md` (sibling reference)
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️w2-block5d-boot.md` (sibling reference)
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️w5-storybook.md` (sibling reference)
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️explore-raster-dev-boot-ts.md` (sibling reference)
