# `🖍️draw` plugin — build, boot, descriptor & test infrastructure

Scope: `✏️s/🔌️plugins/🖍️draw` (crate `semio-s-plugin-draw`, nx `@semio-tech/draw-plugin`, TS package
`@semio-tech/draw-js` at `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript`). Method: read-only static
exploration (no cargo/bun build/dev server run), cross-checked against
`BLOCK-PLUGIN-END-TO-END/📓️explore-build-test-infra.md` + `📓️explore-dev-boot-path.md` (generic
path, verified still current) and `S-END-TO-END/📋️plan.md` + its `📓️explore-catalog-build-state.md`
/ `📓️explore-per-plugin-blockers.md` (draw row). Repo date at exploration time: 2026-09-05/06;
`S-END-TO-END` coordinator log runs live through 2026-09-06 00:17.

**Headline finding, not in either upstream report**: `semio-s-plugin-draw` does not compile today, for
two independent, purely-local naming-drift reasons (§3). Neither is part of the active `space`/`stdio`
churn — both predate this week's rename sweeps. Fixing them is a same-day, single-plugin task with no
cross-plugin dependency, unlike almost everything else blocking the wider `s` shell boot.

---

## 1. Launch config, alias resolution, load closure

**The three launch.json entries** (`.vscode/launch.json:6570-6622`) and the seed
(`.vscode/🧩️launch.seed.jsonc:7359-7382`, key `"draw"`) all call `bun nx run
@semio-tech/framework-os-dev:dev` **directly** — they bypass the root `📜️script.ts`/`package.json`
alias entirely (unlike e.g. `dag`'s seed entry, which does go through `bun ./📜️script.ts dev dag`).
Env for each:

| launch entry | `S_OS_PORT` | `SEMIO_PLUGIN` | `SEMIO_RENDERER` |
|---|---|---|---|
| `🛠️dev✏️draw⚛️react` | 6064 | draw | react |
| `🛠️dev✏️draw🧊️wgpu🌐️wasm` | 6164 | draw | wgpu |
| `🛠️dev✏️draw🧊️wgpu🖥️native` | — | — | `bun .../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native draw` |

`package.json:105` — `"dev:draw": "bun ./📜️script.ts dev draw"`; `package.json:118` —
`"build:draw": "bun nx run @semio-tech/framework-os-dev:build -- draw"`. The root-script path (used by
`bun run dev:draw`, not by the launch.json entries above) is the same chain the block report already
traced: `📜️script.ts:474-514` `DevScript.run(["draw"])` → `resolvePlaygroundDevApp(["draw"])` → catalog
match on variant `"draw"` (single variant, no dimension suffix, unlike `block`'s `2d`/`3d`/`5d`) →
`runFrameworkOsPlaygroundDev("draw", [])`. Both paths converge on the same inner `DevScript` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`, so the block
report's env-var table (`SEMIO_PLUGIN_ONLY`, `SEMIO_RENDERER` default-wgpu-unless-explicit,
`SKIP_ENGINE_BUILD` react-only no-op for wgpu, `SEMIO_BUILD_BUDGET_MS` default 1,200,000 ms,
`CARGO_TARGET_DIR` read-but-not-set-for-you) applies unchanged — verified nothing plugin-specific
overrides it for draw.

**Is `draw` in the generated playground catalog? Yes**, and cleanly:
- `🤖️generated/🎠️playgrounds.json:306-321` — `variant: "draw"`, `pluginId: "draw"`, `cratePath:
  "✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust"`, `ports: { react: 6064, wgpu: 6164 }`, `examples: ["🎬️demo",
  "🎬️demo-session"]`, `engines: []`, `assets: []`. No `aliases`, and — unlike `block`/`norm`/`gis` — **no
  `app` field**: draw declares only one app pair and doesn't need the catalog to disambiguate it.
- `🤖️generated/🎮️playgrounds.ts:36` — same row, TS form.
- `🤖️generated/🔌️plugins.json:246-272` — full plugin registry row (see §3).
- Source of truth: `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml:16-18` —
  `[[package.metadata.semio.playground]] variant = "draw"` — no `[package.metadata.semio.storybook]`
  table at all (see §4).

**Load closure for `SEMIO_PLUGIN=draw`**: `draw` + `stdio`, plus one harmless phantom. Cargo.toml's only
plugin-crate path dependency is `semio-s-plugin-stdio` (`Cargo.toml:23`) → `dependsOn: ["stdio"]`. But
Cargo.toml *also* has a path dependency on a **local helper crate that happens to be named like a
plugin**: `fsm = { path = "../../🗿️artifacts/🖍️drawing/.../🔄️fsm/📦️packages/🦀️rust", package =
"semio-s-plugin-draw-fsm" }` (`Cargo.toml:22`). `parseCargoPluginDependencyIds`
(`🔌️plugin/📇️registry/📜️script.ts:405-415`) pattern-matches *any* `package = "semio-s-plugin-<id>"` or
`semio-s-plugin-<id> =` in the manifest text — it has no way to distinguish "a real sibling plugin" from
"an internal sub-crate that merely follows the `semio-s-plugin-*` naming convention for its own reasons".
Confirmed in the generated catalog: `🤖️generated/🔌️plugins.json:258-261` —
`"dependsOn": ["draw-fsm", "stdio"]`. There is **no** `pluginId: "draw-fsm"` row anywhere in
`🔌️plugins.json` (grep confirms zero matches), so `findPluginCargoPathsForIds`
(`📇️registry/📜️script.ts:635-641`) silently drops it when resolving the dev-session's crate set —
`resolveRegistryPluginIdsForFilter("draw")` (`:609-633`) adds `"draw-fsm"` to its working `Set` but
nothing in `findPluginCargoFiles` ever has that pluginId, so it resolves to zero paths and vanishes. Net
effect: **harmless today**, but it is dead weight in the generated `dependsOn` array and would misfire if
a future refactor ever gives `resolveRegistryPluginIdsForFilter` a "no such dependency" assertion. Low
priority; flagged so nobody spends time chasing a phantom "why does draw depend on a plugin called
draw-fsm" question later.

**Profile / build command**: identical mechanism to block — `cargo rustc -p semio-s-plugin-draw --target
wasm32-wasip2 --profile <wasm-dev|wasm-release>` via `buildPluginCargo`
(`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:963-972`), profile chosen by `selectComponentWasmProfile`
(`📚️library/📦️packages/🟦️typescript/🟦️.ts:3222`, overridable by `SEMIO_PLUGIN_PROFILE`). No draw-specific
override anywhere.

**Engines**: `engines: []` in the catalog row — draw needs no extra engine wasm beyond the three universal
ones (`framework_surface`, `framework_editor`, `flow-core`) that `buildEngineWasm` builds unconditionally
for every react session. All three are fresh as of this exploration (`🕸️bindings` mtimes 2026-09-04
21:25 – 2026-09-05 13:38), so `SKIP_ENGINE_BUILD=1` is currently a valid shortcut — re-check before
relying on it, per the block report's own caveat (other sessions rebuild these).

---

## 2. Descriptor: owner pair, staging, and a real content divergence

`✏️s/🔌️plugins/🖍️draw/🔣️.json` and `🛂️.descriptor.semio` **both exist** at owner root (mtimes
2026-09-04 11:17 / 2026-08-18 20:38) — draw is **not** in the "missing owner pair" class that `block`,
`stdio`, `playbook`, `trinity`, and 15 extensions are in (per `S-END-TO-END/📓️explore-catalog-build-state.md`
§1). But "present" is not the same as "valid":

- **Missing `packageId` (schema-stale)**: parsing `🔣️.json` directly shows no `packageId` key at all.
  This is exactly the class `S-END-TO-END/📓️opus-descriptor-producer.md:121,131-134` identifies as **32
  owner pairs that predate `packageId`'s addition to `PackageDescriptor`** — not a hand-authored mismatch,
  a schema-vintage gap. Under the now-fail-closed `check` (`opus-descriptor-producer.md:48` —
  `validateCatalogDescriptorPair` now checks packageId/pluginId/role/host match and rejects a half-present
  or divergent pair as an **error**, not a warning), draw's pair will fail `check` today even though both
  files exist.
- **App-id divergence — a real content bug, found by direct comparison, not covered by either upstream
  report**: `🔣️.json`'s `manifest.apps` are `"s.draw.draw@1/*#editor"` /
  `"s.draw.draw@1/*#viewer"` (dialect `artifactKind: "s.draw.draw"`). Current source is unanimous and
  consistent that the canonical id is **`s.draw.drawing`**, not `s.draw.draw`:
  `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs:476` (`DRAWING_DIALECT`, `artifact_kind:
  "s.draw.drawing"`), `:534` (`ArtifactKindId::parse("s.draw.drawing")`), the editor's own `controller:
  "s.draw.drawing@1/*#editor"` (`.../✏️editor/🦀️.rs:998`), the TS twins
  (`.../✳️any/✏️editor/🟦️.ts:5`, `.../👁️viewer/🟦️.ts:5`: `artifactKind: "s.draw.drawing"`), every proto
  package name (`semio.s.draw.drawing.*`), every schema/definition capability row in
  `definition()` (`🖍️drawing/🦀️.rs:487-508`). The committed descriptor is stale against all of this —
  it needs a fresh `describe` run once the crate compiles (§3 blocks that today).

**`describe`** (`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📜️script.ts`, `DescribeScript`) is wired
identically to block's fixed convention: `describePluginComponent(this.repoRoot,
"semio-s-plugin-draw", join(this.root, "..", ".."))`, writing into the owner root, not
`🤖️generated/`. Nx targets confirmed real (`📋️project.json`, §4).

**Staging / directory name**: `stagePluginDescriptor` (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:820-833`)
copies the owner-root pair into `🔌️plugin-modules/<moduleDirectoryName(pluginId)>/🔣️.json`.
`moduleDirectoryName("draw")` resolves through the **hand-authored deployment catalog**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts:64-68` +
`🗺️catalog.json:14`): `{ "pluginId": "draw", "directoryName": "🖍️draw" }` — already registered, no
naming decision pending (unlike the `s`/`space` saga this ticket's sibling has been fighting all day —
see §6). Confirmed **no** `🖍️draw` (or any draw-named) directory exists yet under
`🔌️plugin-modules/` — only `🎨️flow-extension-draw` (a *different* row, flow's own extension for the
draw file-format bridge, unrelated to this plugin) is present there. So once draw's crate builds, its
cache directory will land at `🔌️plugin-modules/🖍️draw/` with no further wiring needed.

**Comparator: a built sibling** (`🗒️note`, `💠️lowpoly`) has exactly `.core.wasm`, component JS,
`🔣️.json`, `🛂️.descriptor.semio` under its `🔌️plugin-modules/<dir>/` — draw has none of the four yet.

---

## 3. Why the draw crate does not compile today (the actual gate before everything else)

Two independent, purely-local reference bugs. Both are long-standing (predate this week's rename sweeps
— confirmed by `git log`), not damage from the active `space`/`stdio` churn this ticket's sibling is
fighting.

### 3a. Owner-root `plugin()` references a module/type namespace that does not exist

`✏️s/🔌️plugins/🖍️draw/🦀️.rs:37-46` (the file `📦️packages/🦀️rust/🦀️.rs:698` mounts as `mod plugin` via
`#[path = "../../🦀️.rs"]`, exactly the block-report's "shared kernel" pattern):

```rust
Plugin::<DrawApps>::builder("draw")
    .declare_artifact(crate::artifacts::draw::artifact())
    .editor_mutation_roster::<crate::editor::draw::DrawPlayApp>()
    .viewer_mutation_roster::<crate::viewer::draw::DrawViewer>()
    .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::draw::artifact_kind().id })
```

`crate::artifacts::draw`, `crate::editor::draw`, `crate::viewer::draw` **do not exist**. The crate entry
(`📦️packages/🦀️rust/🦀️.rs:33-35,555-557,671-673`) declares `pub mod artifacts { pub mod drawing {…} }`,
`pub mod editor { pub mod drawing {…} }`, `pub mod viewer { pub mod drawing {…} }` — the module is named
`drawing` everywhere, never `draw`. Likewise `DrawPlayApp`/`DrawViewer` are not defined anywhere in the
tree (grep confirms zero hits outside this one file) — the real structs are `DrawingPlayApp`
(`🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:919`) and `DrawingViewer`
(`.../👁️viewer/🦀️.rs:39`). This is `E0433`/`E0412`-class: unresolved module + unresolved type, at
5 call sites (`🦀️.rs:37,41,42,43,44` plus the two `#[cfg(test)]` surface tests at `:57,62`).

**Fix** (mechanical, no design decision): replace `draw` with `drawing` in the four module paths and
`DrawPlayApp`/`DrawViewer` with `DrawingPlayApp`/`DrawingViewer` in `✏️s/🔌️plugins/🖍️draw/🦀️.rs` —
7 identifier occurrences total.

### 3b. `🚪️io/🦀️.rs`'s stdio import uses stdio's OLD (pre-shortening) type names

`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:15-18`:

```rust
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawingCanvas as SemioDrawCanvas, DrawingLayer as SemioDrawLayer, DrawingNode as SemioDrawNode,
    DrawingStyle as SemioDrawStyle, PathSegment as SemioPathSegment, SemioDrawingSnapshot,
    STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
```

stdio's actual schema module
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🦀️.rs`)
defines `DrawStyle` (`:89`), `DrawLayer` (`:109`), `DrawCanvas` (`:121`), `DrawNode` (`:52`) — **no
`Drawing`-prefixed names** — plus `PathSegment` (`:16`, matches), `SemioDrawingSnapshot` (`:143`,
matches) and `STDIO_SEMIODRAWING_DOCUMENT_SCHEMA` (`:136`, matches). Four of the seven imported names
(`DrawingCanvas`, `DrawingLayer`, `DrawingNode`, `DrawingStyle`) don't exist under those names — `E0432`,
unresolved import. `git log` on this stdio file shows it was last touched **2026-09-05 19:04**
("Normalize semantic emoji path identities…") — draw's `🚪️io/🦀️.rs` was last touched **2026-09-03
12:49** (an earlier normalization pass that explicitly did *not* mention draw in its own commit
message). The two files drifted apart when stdio's own names were normalized to the short `Draw*` form
and draw's consumer import was never updated to match.

**Fix** (mechanical): drop the `ing` from the four import names (`DrawCanvas as SemioDrawCanvas`,
`DrawLayer as SemioDrawLayer`, `DrawNode as SemioDrawNode`, `DrawStyle as SemioDrawStyle`) — every
downstream use in this file goes through the `Semio*` alias already, so nothing past the import list
needs to change.

Neither bug is what `S-END-TO-END/📓️explore-per-plugin-blockers.md`'s draw row describes ("absent stdio
drawing snapshot names… `🚪️io/🦀️.rs:16-19`") — that report correctly spotted 3b but not 3a, and did not
name the specific wrong/right identifier pairs. 3a would fail the build independently of 3b even if 3b
were already fixed.

---

## 4. Identity consistency (once 3a/3b are fixed, everything else already lines up)

| authority | value | file:line |
|---|---|---|
| Cargo component package | `semio:draw` | `📦️packages/🦀️rust/Cargo.toml:13` |
| `Plugin::builder(...)` | `"draw"` | `🖍️draw/🦀️.rs:37` |
| `.package_id(...)` | `"semio:draw"` | `🖍️draw/🦀️.rs:40` |
| Registry row `pluginId`/`packageId` | `draw` / `semio:draw` | `🤖️generated/🔌️plugins.json:247-248` |
| Deployment catalog row | `{ pluginId: "draw", directoryName: "🖍️draw" }` | `📇️registry/📦️deployment/🗺️catalog.json:14` |
| Descriptor `manifest.pluginId` | `draw` | `✏️s/🔌️plugins/🖍️draw/🔣️.json` |
| Artifact kind grammar | `s.draw.drawing` (owned by plugin `draw`, kind `drawing`) | `🗿️artifacts/🖍️drawing/🦀️.rs:534` |
| Activation event (legacy short form) | `2d.drawing` | `🖍️drawing/🦀️.rs:452`, `🔌️plugins.json:263` |

Every one of these agrees on the plugin identity `draw`/`semio:draw`. Unlike the `s`/`space` saga this
week (Cargo said `s`/`semio:s` while a stray auto-commit briefly said `space`/`semio:space` in two
places, only reconciled as of the 2026-09-06 00:17 coordinator log entry), **draw has no identity split
to resolve** — lane H's new assembly gate ("artifact identity is not owned by the declaring plugin",
derived from the canonical `s.<plugin>.<kind>` grammar) would pass draw cleanly: plugin id `draw` owns
kind prefix `s.draw.*`. The only open item is the descriptor CONTENT divergence in §2 (stale app id,
missing `packageId`), not an identity-authority conflict.

---

## 5. Tests, nx targets, oracle, launch entries, storybook

**Rust side** (`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📋️project.json`) — `test`, `test-quick`,
`test-long`, `test-exhaustive`, `describe` all present, wired byte-for-byte like block's (only the crate
name differs): `bun ./📜️script.ts test [quick|long|exhaustive]` → `TestScript.run()` →
`runCargoTestBudgeted(["semio-s-plugin-draw"], this.repoRoot)` (the level segment is accepted by the nx
target but not read by `TestScript.run`; `runCargoTestBudgeted` reads `SEMIO_TEST_LEVEL` from the
environment instead — same as every other plugin, not a draw-specific gap). `describe` →
`describePluginComponent(...)` (§2). This crate's own `📜️script.ts` is fully correct; nothing to fix
here.

**TS side** (`✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/`):
- `📋️project.json` — one `test` target: `bun ./📜️script.ts test`. Correct and real.
- `📜️script.ts` (10 lines) — runs Node's own test runner (`process.execPath test <file> <file>`)
  against two literal files: `.../📚️examples/🎬️demo/🧪️tests/🟦️.ts` and
  `.../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` — **both files exist** (confirmed on disk,
  409 B / 417 B, mtime 2026-09-05 05:51).
- `🟦️.ts` — re-exports `drawing_schema`/`drawing_io` from the two artifact-level TS twins
  (`.../✳️any/🧬️schema/🟦️.ts`, `.../✳️any/🚪️io/🟦️.ts`) — **both exist** (2031 B / 1052 B, 2026-09-02
  17:21), and its own doc comment records that the pre-migration flat `🪓️decomposer` path this file
  used to point at "never existed in the current tree" — already cleaned up, nothing to do.
- **`package.json` is an unfixed, byte-for-byte `cad-js` copy** — same defect class the block report
  flagged for `puzzle`/`procedural`/`dag`/`demonstrator`. Confirmed: `description` is cad boilerplate
  ("📐️ CAD plugin TS: spatial factory runtime/model graph…"), `scripts.test/generate/fixture` all point
  at `@semio-tech/cad-js:*`, `dependencies` list four `cad-js-module-*` packages plus `@semio-tech/s-3d-js`
  /`infinite-world-r3f`/`ui-react`/`ui-styling` that draw has no use for. Only `name` and
  `repository.directory` are correctly customized.
  **Correction to the task's framing**: `🗒️note`'s `package.json` is *also* an unfixed `cad-js` copy
  (verified byte-for-byte identical to draw's except `name`/`repository.directory`) — it is **not** a
  usable oracle. The only correct oracle is `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/package.json`
  (fixed 2026-09-05): own description, `scripts: { "test": "bun nx run @semio-tech/block-js:test" }`
  only, `dependencies: {}`. Draw's `package.json` needs: a draw-specific one-line `description`; drop the
  three `cad-js:*` scripts (rewrite `test` to `@semio-tech/draw-js:test`, drop `generate`/`fixture` unless
  something actually calls them — nothing under draw's TS tree does); replace the 8 leftover
  `dependencies` with whatever draw's TS layer actually needs (its `🟦️.ts` only re-exports two local
  files and imports nothing from those packages — likely `dependencies: {}`, matching block, pending a
  real check of the two twin files' own imports).

**Draw's own `🧪️oracle/🔣️.json`** (`✏️s/🔌️plugins/🖍️draw/🧪️oracle/🔣️.json`): registers **zero**
`oracles`/`noOracleDecisions`/`comparisonProfiles` at plugin-owner scope (the per-subset manifests under
`🗿️artifacts/**/🪆️subsets/*/🔮️oracle/🔣️.json` carry the actual `noOracleDecision` rows — draw is a
semio-native document format, no third-party reader/writer to reconcile against). What it DOES declare
is `oracleHostPackages: [{ implementation: "rust", package: "semio-s-plugin-stdio-test-oracle", path:
"✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust" }]` — consumed by `oracleHostPackagesFor`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:971`), which walks up
from a test case's owner to find which native host package the repo's generated-test-host harness must
link when assembling a cache-local crate for draw's own `mutate-drawing-1-any-*` fixture cases —
identical mechanism to block's `mutate-block-*` adapters (block report §1/§3). Not draw-specific
infrastructure, just draw's row in a shared table.

**Launch entries**:
- `🧪️test🖍️draw📚️examples` (`.vscode/launch.json:7776-7782`) → `bun nx run @semio-tech/draw-js:test` —
  **real**, exercises the TS example tests above.
- `🧹clean🧩️taxonomy🧪️draw-destination-observation` (`.vscode/launch.json:6952-6962`) → `bun nx run
  @semio-tech/repo-lib:test-draw-destination-observation`. **Correction to the task's framing**: this is
  **unrelated to the `🖍️draw` plugin** — it's a `repo-lib` taxonomy-normalization regression test from
  ticket `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/`), whose
  name uses "draw" as a verb ("observe a drawn destination"), not as the plugin id. Coincidental name
  collision only; no action needed for the draw-plugin ticket.

**Storybook**: `.storybook/scopes.ts` has zero literal `draw` references (same as block). Unlike block,
draw's `Cargo.toml` has **no** `[package.metadata.semio.storybook]` table at all, so the generated opt-in
mechanism (`buildGeneratedScopes`) produces **no scope for draw whatsoever** — not even a config-only
empty one. What DOES cover draw: the shared per-plugin matrix story
`.storybook/stories/framework/os/plugins.stories.tsx:35` — `export const Draw: Story = { args: { plugin:
"draw" } }`, one line among 33, feeding the generic `OsBootHost` component under the
`🛠️framework🖥️os/Plugins` title (this file is the one lane A fixed this week — its import is confirmed
resolved, see §6). `.storybook/os-plugins.spec.ts` drives this dynamically over
`PLUGIN_BUILD_TARGETS` (imported from `🤖️generated/🧩️plugins.ts`, not hardcoded per-plugin) — for each
target it HEAD-probes the module URL (`pluginArtifactAvailable`, `:19-24`) and, when the artifact is
missing, asserts the **"plugin artifact missing" panel** appears rather than expecting a real boot
(`:29-32`). **For draw today this spec passes vacuously** — it proves the missing-artifact panel shows
up, not that a canvas renders — because draw has zero build output (§1/§2). It will only start proving a
real boot once §3's fixes land and a build produces `.core.wasm` + `🔣️.json` in the cache.
`.storybook/s-end-to-end.spec.ts` has zero literal `draw` references either (it's the broader
studio/session spec, not a per-plugin one) — not further explored, out of scope for a single-plugin
report.

**`framework-os-dev verify catalog`** (`runCatalogSmokeVerify`,
`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2043-2236`, confirmed functionally complete on disk per
`S-END-TO-END/📋️plan.md`'s 13:35 and 14:50 log entries): boots one live `s` session, reads the shell's
own dev probe (`window.__semioOsCatalogProbe`) for the full program list — never hardcoded — and spawns
each via the command palette (`spawn.<pluginId>`), asserting a new window appears. `catalogSmokeExitCode`
(`:2124-2126`) fails the whole run if **any** program fails, **any** plugin is left `failed`/`crashed`, or
nothing passes at all. Since draw cannot currently load (no build, no valid descriptor), it either
appears as a `failed`/`crashed` row or its `spawn.draw` command never appears in the palette at all —
either way, **this catalog-wide gate cannot go green while draw is unbuilt**, which is exactly
`S-END-TO-END/📋️plan.md`'s Definition-of-done item 2.

---

## 6. What's fixed on disk today vs still open (from the sibling `S-END-TO-END` ticket, as it bears on draw)

| item | state as of this exploration | evidence |
|---|---|---|
| Shard worker route (`/plugin-modules/_shard/…` ASCII bug) | **fixed** | `🔌️plugin-modules/🧵️shard/🟨️shard-worker.js` exists at the correct emoji path; plan log 15:35/19:20 |
| Router excludes demonstrator instead of dying | **fixed** | plan log 21:45, lane H done, assembly gate landed |
| `space` plugin identity (`s` vs `space` split) | **fixed** | Cargo `semio:space` (`🪐️space/Cargo.toml:12`), `builder("space")`/`.package_id("semio:space")` (`🪐️space/🦀️.rs:809,812`), registry row `pluginId: "space"` (`🔌️plugins.json:1336`), deployment row `{space, 🪐️space}` — all verified directly, matches plan log 00:17 (2026-09-06) |
| `stdio` compiles on `wasm32-wasip2` | **fixed** (per plan log 18:25, 86 min clean check) — not independently re-verified here (no cargo run) | plan log only |
| Full catalog rebuild (`draw`/`layout`/`energy` get fresh cores) | **not yet reached** — Wave 2, gated on the `space`/`s` host rebuild currently in flight (rebuild 7, `SEMIO_PLUGIN_ONLY=space`, log `plugin-build-space-7.txt`) | plan log 00:17 |
| `draw`'s own compile bugs (§3) | **open, untouched by any lane** — not mentioned anywhere in the `S-END-TO-END` coordinator log; nobody is currently working on it | this report |
| `draw`'s descriptor content (§2) | **open** — needs a `describe` re-run after §3 lands | this report |
| `draw`'s TS package.json (§5) | **open** | this report |

Draw is a clean, isolable slice: it does not depend on the `space` identity work, the `stdio`
mount/BREP churn, the router/shard fixes (those are shared shell plumbing draw benefits from once it
loads, but draw's own blockers are entirely local to its own two source files). It does depend on the
Wave-2-style full/partial rebuild machinery to actually get its wasm into the cache, and on the shared
shell being up (shard worker + router fixes, both already landed) to render a window once it's built.

---

## Readiness checklist

| item | state today | evidence | owner lane |
|---|---|---|---|
| Crate compiles (`cargo check -p semio-s-plugin-draw`) | **RED** — 2 independent naming-drift bugs (§3a, §3b) | `🖍️draw/🦀️.rs:37-46`; `🗿️artifacts/🖍️drawing/…/🚪️io/🦀️.rs:15-18` | unowned — isolable, single-plugin fix |
| Owner descriptor pair present | present but schema-stale + content-divergent | `✏️s/🔌️plugins/🖍️draw/🔣️.json` (no `packageId`, wrong `s.draw.draw` app id) | needs `describe` re-run after compile fix |
| Plugin identity consistent across authorities | **green** | §4 table | none needed |
| Registered in generated playground/registry catalog | **green** | `🎠️playgrounds.json:306-321`, `🔌️plugins.json:246-272` | none needed |
| Deployment catalog directory name assigned | **green** (`🖍️draw`) | `📦️deployment/🗺️catalog.json:14` | none needed |
| `🔌️plugin-modules/🖍️draw/` cache dir | **absent** (zero build output — one of only 2 of 59 rows with none) | `S-END-TO-END/📓️explore-catalog-build-state.md` §1; confirmed by directory listing | Wave 2 rebuild, gated on §3 |
| Rust nx targets (`test`/`test-quick`/`test-long`/`test-exhaustive`/`describe`) | **green**, correctly wired | `📦️packages/🦀️rust/📋️project.json`, `📜️script.ts` | none needed |
| TS nx target (`test`) | **green**, correctly wired | `📦️packages/🟦️typescript/📋️project.json`, `📜️script.ts` | none needed |
| TS `package.json` content | **RED** — unfixed `cad-js` copy | `📦️packages/🟦️typescript/package.json` | draw-plugin ticket, follow block's fixed shape |
| Storybook scope | none exists (no `[storybook]` metadata); generic per-plugin matrix story covers it vacuously | `.storybook/stories/framework/os/plugins.stories.tsx:35`; `Cargo.toml` (no storybook table) | optional — only needed if a dedicated draw story is desired |
| `os-plugins.spec.ts` for draw | passes vacuously (artifact-missing panel), not a real boot proof | `.storybook/os-plugins.spec.ts:19-32` | resolves itself once §3 + a build land |
| `verify catalog` catalog-wide gate | **blocked** by draw (and `layout`) being unbuilt | `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2043-2236` | Wave 2 rebuild |
| Shared shell plumbing (shard route, router) | **green**, landed this ticket-cycle | §6 table | `S-END-TO-END` lanes G/H (done) |

---

## Boot recipe (copy-pasteable, once §3's two fixes are applied)

```bash
cd /Users/ueli/Documents/semio

# (a) Build only draw's registry-filtered closure ({draw, stdio}) into a private target dir.
#     Isolate from any other session's shared target/ lock; raise the budget past the 20-min default;
#     keep the wasm-dev profile off the memory wall.
CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-draw-e2e-wasm \
SEMIO_BUILD_BUDGET_MS=3600000 \
CARGO_PROFILE_WASM_DEV_DEBUG=false \
bun nx run @semio-tech/framework-os-dev:plugin -- draw

# (b) Regenerate the owner descriptor pair (re-emits 🔣️.json + 🛂️.descriptor.semio at the plugin
#     owner root with the fixed packageId and the correct s.draw.drawing app ids).
cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust
bun ./📜️script.ts describe
cd /Users/ueli/Documents/semio

# (c) Boot the served react shell for draw. Engines are fresh (checked this exploration) so
#     SKIP_ENGINE_BUILD=1 is safe — re-check mtimes under 🕸️bindings/ before trusting this on a
#     later run. This is the same nx target the launch.json entry 🛠️dev✏️draw⚛️react calls.
S_OS_PORT=6064 \
SEMIO_PLUGIN=draw \
SEMIO_RENDERER=react \
SKIP_ENGINE_BUILD=1 \
CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-draw-e2e-wasm \
SEMIO_BUILD_BUDGET_MS=3600000 \
bun nx run @semio-tech/framework-os-dev:dev -- draw
# poll http://127.0.0.1:6064/ for the readiness beacon rather than trusting log output

# (d) Scoped smoke once (c) is up: either the full catalog gate (draw is one row in it)
bun nx run @semio-tech/framework-os-dev:verify -- catalog
# or just draw's own os-plugins.spec.ts case
bunx playwright test .storybook/os-plugins.spec.ts -g "draw"
```

**Warnings** (generic, carried over from the block report, all verified to still apply): the default
`BUILD_BUDGET_MS` is 1,200,000 ms (20 min) and a build queued behind another session's shared
`target/debug/.cargo-lock` will be SIGKILL'd — always isolate `CARGO_TARGET_DIR` and raise
`SEMIO_BUILD_BUDGET_MS` for a cold build under concurrent load; `CARGO_PROFILE_WASM_DEV_DEBUG=false`
avoids the wasm-dev profile's memory wall (`codegen-units=1` + debug info drives stdio's own rustc child
to 8+ GB per the project memory `wasm-dev-profile-debug-off-and-swap-thrash`) — draw itself is a small
crate, but it pulls in `stdio` as a build-order dependency, which is the crate that wall actually bites.
