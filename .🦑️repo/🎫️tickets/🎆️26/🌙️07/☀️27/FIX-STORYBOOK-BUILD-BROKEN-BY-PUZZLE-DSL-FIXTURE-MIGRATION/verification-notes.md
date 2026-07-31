# Verification notes

## What was fixed
- `puzzle/2d/rs/lib.rs`, `puzzle/3d/rs/lib.rs`, `puzzle/5d/rs/lib.rs`: added a free `#[wasm_bindgen]`
  export per crate (`puzzle2dParseDslJson` / `puzzle3dParseDslJson` / `puzzle5dParseDslJson`) wrapping
  the existing `<Projection as vcs::DocumentDsl>::parse_dsl` + `serde_json::to_string` bridge.
- `puzzle/5d/rs/lib.rs`: fixed a separate, pre-existing latent bug found while rebuilding the wasm
  package — `Puzzle5dPrecomputeSession::set_scene`/`apply_brush_placement_rust`/`apply_fill_count_rust`
  unconditionally called `puzzle_3d::Puzzle3dPrecomputeSession`'s native/p2-only (`_rust`-suffixed,
  `Puzzle3dError`-typed) methods, which don't exist on that struct's plain-wasm32 (non-p2) variant
  (`_json`/short-named, `JsValue`-typed methods instead) — the crate's own `wasm-pack build --target web`
  had apparently never successfully compiled before. Split into two cfg-gated impls mirroring
  `puzzle_3d::Puzzle3dPrecomputeSession`'s own existing split.
- `.storybook/stories/puzzle/2d/Fixtures.stories.tsx`, `.../3d/World.stories.tsx`, `.../5d/Timeline.stories.tsx`:
  replaced the deleted `*.2d.json`/`*.3d.json`/`*.5d.json` imports with `?raw` imports of the real
  `*.puzzle2d`/`*.puzzle3d`/`*.puzzle5d` DSL-text fixtures, parsed at story-mount time via the new wasm
  exports (module-promise-cached, same pattern as `framework/renderer/react/index.tsx`'s
  `createEngineSession`). Host components now load fixtures async (loading placeholder + `useEffect`).
- `.storybook/scopes.ts`: fixed an unrelated stale one-line alias (`@semio-tech/compose-algorithm` pointed
  at `compose/dev/algorithm/index.ts`, should be `compose/dev/algorithm/js/index.ts`) — found while trying
  to get a scoped Storybook build to verify the fix; this was blocking any Storybook build (full or
  scoped) regardless of the puzzle fix.

## Verification performed
- `bun nx run @semio-tech/puzzle-2d-rs:wasm`, `:puzzle-3d-rs:wasm`, `:puzzle-5d-rs:wasm` all build clean,
  new exports confirmed present in the generated `pkg/*.d.ts` files.
- Wrote a standalone verification page (`verify-wasm-parse.html`, this folder) served via a plain
  `python3 -m http.server` from the repo root (bypassing Vite/Storybook entirely) that imports the three
  rebuilt wasm packages directly, fetches the real `.puzzle2d`/`.puzzle3d`/`.puzzle5d` example fixtures,
  and calls the new `puzzle{2,3,5}dParseDslJson` exports. Confirmed in a real browser (Claude Browser pane),
  zero console errors:
  - concrete-forest fixtures: camelCase JSON matching the pre-migration fixture shape exactly
    (`nodeKind`, `handleKind`, `kindCompatibility{bidirectional,specificity,source,target}` for 2d;
    `camera{position,target,zoom}`, `objects[]{id,label,objectKind,origin,orientation,meshUrl,vortices,...}`
    for 3d; `camera3d`, `parts[]{id,partKind,"2d","3d",grips}` with `"3d"{origin,meshUrl,orientation,label}`
    for 5d) — field-for-field matching the three story files' `StoryPuzzle2dFixture`/`StoryWorld3dFixture`/
    `StoryPuzzle5dFixture` TS types.
  - nakagin-capsule-tower fixtures: confirmed counts nodes2d=180, edges2d=179, objects3d=180, parts5d=180,
    matching the story files' own docstrings ("180 nodes / 179 edges", "180 objects", "180 parts").
- `bun nx run workspace:build-storybook` (full, unscoped): after the fixes above, Vite got past ALL
  puzzle-related imports with zero puzzle-related errors (200 modules transformed) before failing on an
  **unrelated, pre-existing** issue: `coda/client/ui/desktop/js/renderer.tsx` fails to resolve
  `@semio-tech/framework-platform-core`. Confirmed via `git status` that `coda/client/ui/desktop/js/renderer.tsx`
  is independently modified (not by this ticket) — looks like other concurrent work mid-refactor.
- Scoped build (`STORYBOOK_SCOPE=puzzle/2d,puzzle/3d,puzzle/5d,compose`) hit a second unrelated pre-existing
  bug: `compose/dev/algorithm/js/index.ts` itself can't resolve `../../../repo/lib/js/index.ts` (wrong
  relative depth) — also not touched by this ticket, left alone (different technology, likely also
  mid-refactor by someone else).
- Storybook **dev** server (`bun ./📜️script.ts dev storybook`, plain `bunx storybook dev`) crashes on
  startup independent of any story/scope selection, during "Building manager": esbuild can't bundle a
  `.node` native file (`node_modules/rollup/node_modules/fsevents/fsevents.node`) pulled in transitively
  by `@storybook/addon-vitest`'s playwright/chromium-bidi dependency graph. Reproduced twice, unrelated to
  puzzle DSL work, not fixed (Storybook addon/tooling config, out of this ticket's scope).

## Net result
The originally-reported bug (three puzzle Storybook story files broken by the DSL fixture migration,
blocking the full Storybook build) is fixed and directly verified. The full `build-storybook` /
Playwright-spec verification path (`styling.spec.ts`, `os-plugins.spec.ts`) is currently still blocked,
but by unrelated pre-existing breakage in `coda/` and `compose/dev/algorithm/` (both independently
modified outside this ticket) plus an unrelated Storybook addon-vitest/esbuild/fsevents dev-server crash
— none of which trace back to the puzzle DSL fixture migration this ticket targets.
