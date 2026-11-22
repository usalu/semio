# 📋️ Brief: Staffage Plant Collection Wave

Shared brief for every plant worker (`PW-01` … `PW-17`). Coordinator: Opus 5.5 main session.

## Paths

- `R` = `/Users/ueli/Documents/semio` (follow `R/CLAUDE.md` and `R/AGENTS.md` strictly).
- `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage` (read `S/README.md` fully first).
- `T` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE` (this ticket).
- `A` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE` (the archive ticket: authoring rules and the generators of the existing plants).
- `V` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE` (3D volume authoring).
- `M` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE` (top/front/side coverage).

## Read first (binding)

1. `S/README.md`, `S/🧬️schema/🟦️.ts` (v2 types), `S/🧬️schema/🔣️.json`.
2. `A/authoring-v2.md`, `A/authoring.md`, `A/content-worker-brief.md`, `A/content-vegetation.md`, `A/core-v2-status.md` (check / preview CLIs), `A/content-v2-NW-*.md` reports on vegetation.
3. `V/authoring-vegetation.md`, `V/volume-drafting.md`, `V/volume-agreement.md`; `M/📝️sonnet-vegetation.md`.
4. The closest existing plant(s) in `S/🗂️catalog/🌳️vegetation/<id>/🔣️.json` and their generators in `A/🧰️scripts/vegetation/`, `A/🧰️scripts/NW-V1/`, `A/🧰️scripts/NW-V2/` (and any newer generator that the git log of the vegetation folder points to). Reuse the existing generator libraries by importing them from their place — do not copy them.

## Your job

Author your assigned new plants as complete `semio.staffage/v2` assets of category `vegetation` at the same quality level as the best existing plants:

- `S/🗂️catalog/🌳️vegetation/<id>/🔣️.json`, one folder per asset. Work ONLY in your own asset folders. Others work on other plants, the engine and the manifest at the same time; never edit another asset, the engine, the schema or the manifest.
- Species-true dimensions, rig (fixed base support), masses (green wood density, foliage authored-estimate), wind poses (calm, windy, storm; snow/ice/fruit/flowering poses where physically sensible), clips (`wind-sway` looping and periodic, `gust`, `diurnal-transpiration`), leaf thermal regions with transpiration emitters, structure (sections, loads per EN 1991-1-4), every parameter tagged with `source` and `reference`.
- Representations `500`, `200`, `100`, `50` individually designed per scale, plus the authored 3D volume (the same quality the existing plants have; see `V/`), top/front/side coverage (see `M/`).
- All texts `{ en, de }` (English first, German second, natural German, no default language).
- Total power 0 W (passive) unless the asset has a powered element (none expected).

## Generators

Your generator scripts and their design inputs go to `T/🧰️scripts/<worker-id>/` (keep them; they are inputs). Generated previews and logs go to `T/🗑️generated/<worker-id>/` (delete raw logs at the end; keep only evidence PNGs you cite). Shipped JSON contains only schema shapes; never hand-edit the shipped JSON without updating the generator.

## Check and look

Run the staffage check CLI (see `A/core-v2-status.md` and `S/🔨️modules/📤️export/`; tools only through `repo tools exec …` or the repo MCP) until it prints ✓ for every asset you own: all poses, clip instants, thermal, structure, all 7 representations, the 3D volume. Render the default pose at 1:50, 1:100, 1:200, 1:500 plus plan / front / side and READ the PNGs; iterate until each reads well at print size and looks like the species. Run the existing staffage tests that cover every catalog asset (asset-validation, asset-power, catalog-silhouette, multiview-coverage, volume tests, …) through `repo test` / `repo test_file` scoped to the staffage project and fix every failure caused by your assets.

## Manifest

Do NOT edit `S/🗂️catalog/🔣️.json`. Write one manifest entry per asset (exactly the shape of the existing `entries` rows, values derived from your evaluated asset: `heightM`, `widthM`, `powerW`, poses, clips, representations, symbols, analytics, exports, …) to `T/📥️manifest-rows/<id>.json`. The coordinator merges all rows and collections at once.

## Report

Write `T/📝️<worker-id>.md`: per asset the species reference data with sources, rig, poses, clips, representation decisions per scale, 3D volume, check output, tests run, preview PNGs inspected, remaining doubts. Final message: short summary plus the report path.

## Rules recap

No git commands that modify anything, no worktrees, no IDEs, no `repo dashboard`. `[DEBUG] ` prefix on temporary logs. Files change under you (replica daemon): re-read before editing. Keep every path under 256 chars. Do not stop halfway; do not ask questions; be thorough.
