# Animate present-deck DSL + OpText (Wave 4)

## Step 0 — dead TS mirror deletion

Verified before deleting:
- `animate/present/core/js/index.ts` region `//#region 🔖️DocumentVcs` (was lines 1987-2175) declared
  `PresentationEditOperation` union with field `operation:` but every literal inside
  `backwardsPresentationEditOperation`/`applyPresentationEditOperation` used `operator:` (mismatched
  field name from an old rename) — confirmed type-broken.
- `materializeProjection`'s replay loop referenced a free variable `operator` that was never bound
  (`for (const operation of edit.forwards) deck = applyPresentationEditOperation(deck, operator);`) —
  would ReferenceError at runtime even if types were fixed. Definitely dead.
- The dead region also re-declared `FigureTileSource`/`FigureTileDraft` as `export type` aliases that
  collide (duplicate identifier) with the REAL, live `export interface FigureTileSource`/
  `FigureTileDraft` declared earlier in the same file (~line 630/642) and used by
  `populateTileDraftsFromGrid`/`buildTileMorphPrompt` (mirrors the Rust `FigureTileSource`/
  `FigureTileDraft`). So the file could not even type-check as written.
- Grepped for every exported symbol from the dead region (`PresentationEditOperation`,
  `backwardsPresentationEditOperation`, `applyPresentationEditOperation`,
  `diffPresentationEditOperation`, `PresentationDeck`, `PresentationDeckVcsEnvelope`,
  `createPresentationAppVcsHandler`, `createPresentEnvelope`, `clampTileCrop`,
  `PRESENTATION_DECK_EMPTY`) across the whole repo (excluding the file itself). Zero live consumers.
  The only other hits were: (a) a same-named but unrelated React component `PresentationDeck` in
  `animate/present/renderer/react/index.tsx` (a UI component, not importing this type), (b) stale
  one-off migration scripts under `.repo/🎫️/26/07/03/APP-ISOLATION-ENFORCED-BOUNDARIES/*.ts` that only
  contain string literals for a DIFFERENT, now-nonexistent technology
  (`framework/product/presentation`), (c) stale `.claude/worktrees/*` copies from old agent sessions.
  None of these `import` anything from `animate/present/core/js/index.ts`.
- Deleted the whole region plus its own dedicated test block
  (`describe("createPresentationAppVcsHandler", ...)` at the end of the file, which only exercised the
  dead handler). Left the unrelated `describe("tile play", ...)` block alone (it exercises the live
  `FigureTileSource`/`FigureTileDraft` interfaces and functions, not the dead region).
- Ran `bunx nx run @semio-tech/animate-present-core:test` after deletion: 2 test files, 110 tests, all
  passed.

## DSL + OpText design

`PresentDeck { schema: String, source: FigureTileSource, tiles: Vec<FigureTileDraft> }` — real Rust
types in `animate/present/rs/lib.rs` (not opaque JSON), so the grammar is a typed, hand-rolled
recursive-descent parser/printer (`mod present_text`), closely mirroring the established
`imperative_text` pattern in `imperative/core/rs/lib.rs` (same `Word`/`Str`/`LBrace`/`RBrace`/`Eof`
lexer, same `key=`/`key=value` collapsed-token convention, same `wrap_body` pretty/compact helper).

### `.present` DSL (`EXTENSION = "present"`)

```
present schema="animate.present.deck"
source src="/bauteilbörse.png" kind="figure" frame=0.127,0.1,0.746,0.75 aspect=1.3639508928571428 pdfPage=-
tiles {
  tile id="tile-r0-c0" name="tile-r0-c0" crop=0,0,0.373,0.375
  tile id="tile-r0-c1" name="tile-r0-c1" crop=0.373,0,0.373,0.375
}
```

- `frame=`/`crop=` are single whitespace-free `x,y,width,height` tokens (round-trips via Rust's
  `f64::to_string()`/`str::parse::<f64>()`, which are exact inverses).
- `aspect`/`pdfPage` use vcs's own `-` sentinel for `None`.
- `tiles { }` is always printed (even empty) for a deterministic, always-present section.

### Op-text (one line per `PresentOperation` variant)

```
tiles-add index=0 tile id="t1" name="A" crop=0.1,0.1,0.2,0.2
tiles-remove id="t1"
tiles-move id="t1" to=2
tiles-patch id="t1" name="Renamed" crop=-
tiles-patch id="t1" name=- crop=0.3,0.3,0.4,0.4
set-source src="/x.png" kind="figure" frame=0.1,0.1,0.5,0.5 aspect=1.5 pdfPage=-
set-tiles { tile id="t1" name="A" crop=0,0,0.5,0.5 }
set-deck schema="animate.present.deck" source src="..." kind="figure" frame=... aspect=- pdfPage=- tiles { }
```

`set-deck`/`present` share the same `parse_deck_body`/`print_deck_body` (pretty=multi-line for DSL,
compact=single-line for op-text), same as `imperative_text::parse_document`/`print_document` reusing
`print_step` with a `pretty` flag.

## Files touched (all under animate/)
- `animate/present/core/js/index.ts` — deleted dead `🔖️DocumentVcs` region + its dead test block.
- `animate/present/rs/lib.rs` — added `mod present_text` (`🔖️Dsl`/`🔖️OpText` regions), extended
  `🧪️Tests` with `🔖️DslTests`/`🔖️OpTextTests`/`🔖️DocumentTextTests`.
- `animate/plugin/rs/lib.rs` — `.example("demo", ...)` now uses `default_present_deck().print_dsl()`
  instead of `serde_json::to_string(&default_present_deck())`; added `vcs::DocumentDsl` to the `use vcs::{...}` import.

## Verification status
See bottom of this file / final report for cargo test results (filled in after the run completes).
