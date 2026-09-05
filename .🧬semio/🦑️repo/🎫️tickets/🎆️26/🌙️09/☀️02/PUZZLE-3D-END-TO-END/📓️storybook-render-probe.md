# Storybook render probe (no Rust build) — agent T, 2026-09-05

## Question
Can puzzle3d's UI be rendered/verified today through Storybook, without touching the blocked
Rust plugin build?

## Verdict
**No.** `stories/puzzle/3d/World.stories.tsx` — the only puzzle/3d story in the tree — fails to
build. I ran the real build (Vite, no cargo) and it dies with a hard module-resolution error
before a single puzzle3d pixel can render. This is not a Rust-build problem; it's stale/broken
import paths left by an in-flight taxonomy refactor. Fixing it does **not** require unblocking
cargo — it requires correcting import paths in a `.storybook/` file — but nobody should report
puzzle3d Storybook coverage as "green" while this file is broken.

## 1. What the no-wasm / wasm host specs actually cover

`.storybook/framework-hosts-no-wasm.spec.ts` (header, lines 3-4) covers exactly seven `framework/hosts`-scope
stories: `TableHost`, `BlockListHost`, `GraphTimelineHost`, `IconRenderHost`, `InkCanvasHost`,
`Canvas2dHost`, `UiInterpreter`. Every `test()` in the file targets a story id prefixed
`🛠️framework🔌️hosts-...` (e.g. line 53 `🛠️framework🔌️hosts-tablehost--sortable-with-actions`).
**Zero of these are puzzle3d.** They belong to the `framework/hosts` `StoryScope`
(`.storybook/scopes.ts:109-123`), not `puzzle/3d` (`scopes.ts:94-98`).

`.storybook/framework-hosts-wasm.spec.ts` (header, lines 3-9) covers the *other* half of the same
`framework/hosts` scope: `NodeGraphHost`, `TextEditorHost`, `Paint2dHost`, `TiledMapHost`,
`World3dHost` (two variants), `WorldTerrainLayer` — all boot real prebuilt Rust/WASM engines.
Interesting near-miss: line 90-92, `World3dHost minimal-viewport: renders the pure r3f viewport
(no WASM engine)` — but this is `stories/framework/hosts/World3dHost.stories.tsx`, a **generic**
renderer-host smoke story, not puzzle3d's `stories/puzzle/3d/World.stories.tsx`. Different file,
different fixture, different scope. It proves `World3dHost` itself *can* mount with no engine —
it says nothing about puzzle3d.

**Conclusion for item 1: neither `framework-hosts-*.spec.ts` file touches puzzle3d at all.** The
"no-wasm path" they describe belongs to the `framework/hosts` scope's generic host smoke tests.

## 2. What actually covers puzzle3d, and what it needs to render

The real puzzle3d coverage is `.storybook/puzzle-3d-5d-infinite.spec.ts`, lines 53-66:
tests `🧩️puzzle🧊️3d--concrete-forest` and `🧩️puzzle🧊️3d--nakagin-capsule-tower` (the story ids
exported by `World.stories.tsx`'s `ConcreteForest` / `NakaginCapsuleTower`, `World.stories.tsx:338-349`).
Its header note (line 5) says it isn't yet wired into `playwright.config.ts`'s `testMatch` — that's
now stale: the current `playwright.config.ts:28` is `testMatch: ["*.spec.ts"]`, so it already
picks up every spec including this one.

`stories/puzzle/3d/World.stories.tsx` is exactly the fixture-driven pattern the ticket is hoping
for — **not** a live plugin boot:
- It raw-imports real `.puzzle3d` DSL-text fixture files via Vite `?raw` (lines 15-17).
- It parses that DSL text at runtime through a WASM module: `@semio-tech/puzzle-3d-ui-rs/pkg/puzzle_3d.js`
  (line 111), calling its `puzzle3dParseDslJson` export (lines 106-122).
- It hand-builds a `UiComponentSceneNode` (lines 218-272) and mounts `World3dHost` directly
  (line 314) — no OS boot host, no plugin-fleet materialize step, no `/plugin-modules` route.
- GLB meshes are deliberately skipped (header note, line 5) so `World3dHost` falls back to its
  placeholder box — no asset pipeline needed either.

So in principle this is a real no-cargo render path. In practice it is currently broken on two
independent counts, both pre-existing/stale, not caused by the cargo lockout:

**(a) The fixture DSL import paths are stale.** `World.stories.tsx:16` imports
`"../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio?raw"`.
The real file lives three directory levels deeper and under the new kind-only-filename convention:
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🧪️tower/🗣️.dsl.semio`
(confirmed on disk; the shallow `🗿️artifacts/🧊️3d/📚️examples` path the story uses doesn't exist at
all — `ls` fails). The same is true of the `concrete-forest` import on line 15 (real file:
`.../📚️examples/🌲️concrete-forest/🖼️assets/🧪️forest/🗣️.dsl.semio`). The third fixture
(`CapsuleDream`, line 17) points at
`../../../../.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️09/PUZZLE-DESIGN-PARITY/🌙️capsule-dream-out/🗣️dream.3d.dsl.semio`
— `.🦑️repo` at repo root is a build cache dir, not the ticket store (`.🧬semio/🦑️repo/...`), and
that `🌙️capsule-dream-out` directory/file doesn't exist anywhere under the real
`PUZZLE-DESIGN-PARITY` ticket folder either. All three fixture imports are dead paths.

**(b) `@semio-tech/puzzle-3d-ui-rs` does not exist anywhere in this repo.** Repo-wide grep (excluding
`node_modules`) for `puzzle-3d-ui-rs` returns exactly two hits: `World.stories.tsx` itself, and a
migration-ticket JSON (`PUZZLE-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`). It's not a
workspace package (`node_modules/@semio-tech/` has 68 entries, no `puzzle-3d-ui-rs` or `ui-rs`
anything), not declared in any `package.json`, and not aliased in `.storybook/main.ts` or
`scopes.ts`'s `puzzle/3d` scope (`scopes.ts:94-98` has no `aliases` key at all). The
`puzzle3dParseDslJson` wasm-bindgen export the story calls does genuinely exist — at
`✏️s/.../✏️editor/🌉️wasm/🦀️.rs:133-134` (`#[wasm_bindgen(js_name = puzzle3dParseDslJson)]`) — and
is already compiled into a **prebuilt, current** wasm-pack output, but under a different package
name entirely: `@semio-tech/puzzle-wasm` at
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js` (built 2026-09-01 12:17, i.e. after
the Aug 30 Rust source — not stale, `grep -c puzzle3dParseDslJson` on it returns 2 matches). Nobody
wired `puzzle-3d-ui-rs` → `puzzle-wasm`. This looks like an in-flight rename from the same
migration ticket referenced above, not touched yet in the story file.

## 3. The real run

Ran a Storybook production build scoped to just `puzzle/3d` (Vite only — no cargo, no `bun run
dev:puzzle:3d`, nothing that touches the blocked plugin build):

```
cd /Users/ueli/Documents/semio
STORYBOOK_SCOPE=puzzle/3d bunx storybook build -c .storybook --output-dir <scratch>/storybook-static-puzzle3d
```

Real output (full, unedited):

```
┌  Building storybook v10.5.6
│
◇  Cleaning outputDir: ...
◇  Loading presets
◇  Building manager..
●  Building open services..
●  Building preview..
│  Vite vite v7.3.6 building client environment for production...
│  Vite transforming...
▲  Vite [plugin vite:react-docgen-typescript] .storybook/preview.tsx:
│  Skipping docgen for "/Users/ueli/Documents/semio/.storybook/preview.tsx" because it is not
│  included in the active TypeScript project.
│  Vite ✓ 32 modules transformed.
■  Vite ✗ Build failed in 18.95s
■  Failed to build the preview
■  Could not resolve
│  "../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio?raw"
│  from ".storybook/stories/puzzle/3d/World.stories.tsx"
│  file: ./.storybook/stories/puzzle/3d/World.stories.tsx
│  at getRollupError (file://./node_modules/rollup/dist/es/shared/parseAst.js:317:41)
│  at error (file://./node_modules/rollup/dist/es/shared/parseAst.js:313:42)
│  at ModuleLoader.handleInvalidResolvedId (file://./node_modules/rollup/dist/es/shared/node-entry.js:21928:24)
│  at file://./node_modules/rollup/dist/es/shared/node-entry.js:21888:26
└  Storybook exited with an error
```

The build dies on the very first bad fixture import (`nakaginCapsuleTowerFixtureDsl`) before even
reaching the `concreteForestFixtureDsl` or the `@semio-tech/puzzle-3d-ui-rs` wasm import — this one
file failing to bundle takes down **all three** exported stories (`ConcreteForest`,
`NakaginCapsuleTower`, `CapsuleDream`), since Vite/Rollup can't build the preview chunk that
contains the story module at all. This is a genuine, observed failure, not a guess — no cargo, no
plugin fleet involved; pure Vite module resolution.

I did not attempt to patch the paths and re-run, since fixing content in
`World.stories.tsx`/fixtures is outside this probe's scope (find-out-only task) and I wasn't asked
to fix it.

## Is this a usable verification channel today?

**Not as-is.** The no-cargo fixture-DSL-render pattern is architecturally sound and exactly what's
needed to verify puzzle3d UI without the blocked Rust build — but the specific file that
implements it (`World.stories.tsx`) currently has three dead fixture import paths and one
dangling wasm package import, all from an in-progress taxonomy/package rename that hasn't reached
this file yet. Until those four references are corrected (real fixture paths under
`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/.../🗣️.dsl.semio`, and either an alias or a corrected
import for `@semio-tech/puzzle-wasm` in place of the nonexistent `puzzle-3d-ui-rs`), Storybook
cannot render any puzzle3d story, and `puzzle-3d-5d-infinite.spec.ts`'s two puzzle3d tests cannot
pass. This is worth flagging to whoever owns `World.stories.tsx` / the
`PUZZLE-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION` ticket, since it's a small,
Rust-build-independent fix.

## Files referenced
- `.storybook/main.ts`
- `.storybook/scopes.ts`
- `.storybook/playwright.config.ts`
- `.storybook/framework-hosts-no-wasm.spec.ts`
- `.storybook/framework-hosts-wasm.spec.ts`
- `.storybook/puzzle-3d-5d-infinite.spec.ts`
- `.storybook/stories/puzzle/3d/World.stories.tsx`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js` (+ `package.json`, name `@semio-tech/puzzle-wasm`)
