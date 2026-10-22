---
name: Centralized Styling Tokens
overview: Make ui/styling/tokens.json the single source for all visual and metric constants (colors, themes, strokes, radii, opacities, layout metrics, fonts), generate a real styling library per ecosystem (rs, js, net, py) from it, and remove every hardcoded color/stroke/metric from the Rust render libraries.
todos:
 - id: tokens-schema
   content: Extend ui/styling/tokens.json with themes(light/dark), strokes, radii, opacities, metrics, canvasFonts using existing concrete defaults from the inventory
   status: completed
 - id: generator
   content: Rewrite ui/styling/script.ts into a unified multi-target generator (JS, C#, Rust, Python); delete the two *.inc.rs and update adapters.manifest.json
   status: completed
 - id: rs-crate
   content: Create framework-neutral ui/styling/rs crate (Cargo.toml, lib.rs, generated.rs, project.json); add to root Cargo workspace members
   status: completed
 - id: py-lib
   content: Create ui/styling/py library (generated module + script.ts + project.json)
   status: completed
 - id: net-extend
   content: Extend net generator output (Palette.g.cs) with strokes/metrics/themes
   status: completed
 - id: js-bridge
   content: Extend tokens.generated.ts and rewrite serializeGraphVelloThemePaletteJson()/ui.css to derive from token themes
   status: completed
 - id: migrate-vello
   content: Migrate infinite/canvas/vello (clear color, camera limits, label metrics, Typst sizes, insets, fonts, BLACK/WHITE) to ui_styling
   status: completed
 - id: migrate-graph
   content: Migrate mathematical/graph port/directed (VelloThemePalette->Theme), normal, dag, and graph lib stroke/radius/opacity constants to ui_styling
   status: completed
 - id: migrate-map
   content: Migrate gis/map/rs to ui_styling map paints + tokenized scales/bands; remove build.rs color include
   status: completed
 - id: migrate-flow-puzzle
   content: Migrate flow/core and puzzle/2d/rs; remove board .inc.rs include from puzzle build.rs
   status: completed
 - id: wiring
   content: Wire nx generate target for all ecosystems and register commands in launch.json
   status: completed
 - id: validate
   content: Generate, build/test all affected crates+packages, extend existing tests, validate runtime light/dark via [DEBUG] logs
   status: completed
isProject: false
---

# Truly Centralized Styling From Tokens

## Goal

`ui/styling/tokens.json` becomes the single source of truth for **all** visual + metric constants. Each ecosystem gets a real generated styling library (`ui/styling/{rs,js,net,py}`). No Rust render library hardcodes any color, stroke, radius, opacity, or layout metric. Light/dark themes live in tokens; the runtime JS->WASM bridge is derived from the same tokens.

This is a single repo ticket (open via repo MCP `ticket_open`, associate with the most appropriate goal from `repo://goals`). Greenfield: no back-compat, delete the old `.inc.rs` + per-consumer `build.rs` color codegen entirely.

## 1. Redesign `tokens.json` schema

Extend [ui/styling/tokens.json](ui/styling/tokens.json) (currently only `colors`, `spacing`, `fontStacks`, `fontFaces`) with new top-level sections:

- `themes`: `{ "light": {...}, "dark": {...} }`. Each theme maps every **semantic paint** to a `{ token | hex, alpha }` ref. The semantic set is the union of:
  - `VelloThemePalette` fields (38 paints) from [mathematical/graph/port/directed/lib.rs](mathematical/graph/port/directed/lib.rs) lines 400-440 / 696-739.
  - Map paints (`SURFACE_CLEAR`, `LAND_FILL`, `LABEL_FILL`, ...) from [ui/styling/rs/map_vello_build.inc.rs](ui/styling/rs/map_vello_build.inc.rs).
  - Canvas paints: `rasterClear`, icon `fg`/`bg` defaults, label fill/halo from [infinite/canvas/vello/lib.rs](infinite/canvas/vello/lib.rs) + [infinite/canvas/vello/theme.rs](infinite/canvas/vello/theme.rs).
- `strokes`: named widths/mults/dash patterns (edge base `2.0`, selected `1.35`, hovered `1.2`, dashes, grid px bands, handle, wire highlight `2.85`, arrow, selection preview, map road/boundary/coastline/route scales).
- `radii`: node radius, handle radius, position-marker radius.
- `opacities`: dim/disabled alphas (`110/120/140`), grid alpha, fill alphas.
- `metrics`: camera zoom min/max + factor, LOD label px bands, label-layout multipliers (`pad 0.35`, `char 0.62`, `height 1.6`, clamps), icon insets (`fit 0.76`, `clip 0.88`), Typst page/margin/font sizes, hit-test tolerances, layout spacing defaults, declutter, tile bleed.
- `canvasFonts`: family refs for map-label sans + Noto Color Emoji (replace hardcoded `"Noto Color Emoji"` / `"sans-serif"`).

Use the existing concrete defaults from the inventory as the `light` theme values; pick dark equivalents from existing dark palette keys (`dark`, `dark-6-7`, etc.).

## 2. Unified generator: `ui/styling/script.ts`

Rewrite [ui/styling/script.ts](ui/styling/script.ts) so `generate` emits **all** targets from tokens:

- JS: `js/palette.css`, `js/tokens.generated.ts` (add `STYLING_STROKES`, `STYLING_RADII`, `STYLING_METRICS`, `STYLING_THEMES`).
- C#: extend `Palette.g.cs` with strokes/metrics/theme constants.
- Rust: emit a checked-in `ui/styling/rs/src/generated.rs` (replaces the build-time `.inc.rs` include flow). Reuse the existing sRGB->linear conversion (`srgb_byte_to_linear_u8`).
- Python: emit `ui/styling/py/styling/generated.py`.

Delete [ui/styling/rs/board_vello_build.inc.rs](ui/styling/rs/board_vello_build.inc.rs) and [ui/styling/rs/map_vello_build.inc.rs](ui/styling/rs/map_vello_build.inc.rs); update [ui/styling/adapters.manifest.json](ui/styling/adapters.manifest.json).

## 3. New `ui/styling/rs` crate (the canonical model)

Create a real framework-neutral crate (no vello/peniko dependency, per styling-core "framework-neutral" note):

- `ui/styling/rs/Cargo.toml` (crate `ui_styling`), `ui/styling/rs/lib.rs`, generated `ui/styling/rs/src/generated.rs`, `ui/styling/rs/project.json`.
- Expose colors as linear-sRGB `[f32; 4]` consts + numeric `f64` strokes/radii/opacities/metrics, plus `Theme` structs `LIGHT`/`DARK` covering all semantic paints.
- Add `ui/styling/rs` to `[workspace].members` in [Cargo.toml](Cargo.toml).
- Consumers map to their own color type, e.g. `Color::new(ui_styling::theme::LIGHT.node_fill)`.

## 4. New `ui/styling/py` library

Create `ui/styling/py/` with `script.ts` (delegates to the shared generator) + `project.json` + generated `styling/generated.py` exposing tokens/themes/metrics as Python constants/dataclasses. This satisfies "a library for every ecosystem".

## 5. JS bridge from tokens (tokens-only theming)

Rewrite `serializeGraphVelloThemePaletteJson()` in [ui/styling/js/resolve.ts](ui/styling/js/resolve.ts) to build the WASM theme JSON from `STYLING_THEMES[light|dark]` instead of hand-written `color-mix` CSS. CSS semantic vars in `js/ui.css` are generated from the same token themes. Light/dark switch selects the token theme set.

## 6. Migrate Rust render libraries (remove all hardcoded values)

- [infinite/canvas/vello/lib.rs](infinite/canvas/vello/lib.rs) + [theme.rs](infinite/canvas/vello/theme.rs): add `ui_styling` dep; replace `default_raster_clear`, camera zoom limits (`camera` mod), label-layout multipliers + clamps (`text` mod), Typst page/margin/font sizes (`icon_codec`), icon insets, `Color::BLACK/WHITE` icon defaults, and font families with token lookups.
- [mathematical/graph/port/directed/lib.rs](mathematical/graph/port/directed/lib.rs): replace `VelloThemePalette::default()` (lines 696-739) with `ui_styling::theme::LIGHT`/`DARK`; keep `merge_from_json` for runtime override.
- [mathematical/graph/port/directed/normal/lib.rs](mathematical/graph/port/directed/normal/lib.rs) and [.../dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs): replace stroke widths, radii, opacity alphas, grid px bands, hit tolerances, layout defaults with `ui_styling` consts.
- [mathematical/graph/lib.rs](mathematical/graph/lib.rs): replace handle/proximity/selection px + `Color::WHITE` debug.
- [gis/map/rs/lib.rs](gis/map/rs/lib.rs): use `ui_styling::map` paints; tokenize road/boundary/coastline scales, label px bands, declutter, marker radii, tile bleed. Delete [gis/map/rs/build.rs](gis/map/rs/build.rs) color include.
- [flow/core/lib.rs](flow/core/lib.rs), [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs): consume `ui_styling`; remove the board `.inc.rs` include from [puzzle/2d/rs/build.rs](puzzle/2d/rs/build.rs) (keep its icon codegen).

## 7. Wiring

- Extend `@semio-tech/ui-styling-tokens:generate` in [ui/styling/project.json](ui/styling/project.json) to run the full multi-target generation; add nx projects for `ui/styling/rs` and `ui/styling/py`.
- Register the styling generate/test commands in `launch.json` following existing order/grouping.

## 8. Validation

- Run `bun nx run @semio-tech/ui-styling-tokens:generate`, then `cargo build`/`bun test` across affected crates/packages.
- Extend existing tests only (no new test files): the `resolve.ts` vitest block, Rust `#[cfg(test)]` palette tests in the styling crate + graph crate, and a guard test asserting no raw `Color::from_rgba8(`/literal strokes remain in migrated render libs.
- Confirm runtime light/dark switching with `[DEBUG]` logs before removing them.

## Notes / decisions

- `ui/styling/rs` stays framework-neutral (raw `[f32;4]` + numbers); render crates convert to `peniko::Color`. This avoids coupling styling to vello and honors the "no direct external-lib dependency" rule.
- Rust color generation moves from per-consumer `build.rs` includes to a single checked-in generated file, mirroring how JS/C# are generated and checked in.
