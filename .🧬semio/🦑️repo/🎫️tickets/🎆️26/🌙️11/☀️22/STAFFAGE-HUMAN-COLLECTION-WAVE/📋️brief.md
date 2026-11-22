# 📋️ Brief: Staffage Human Collection Wave

Shared brief for every human worker (`HW-01` … `HW-18`). Coordinator: Opus 5.5 main session. A parallel plant wave (`☀️22/STAFFAGE-PLANT-COLLECTION-WAVE`) works on `🌳️vegetation` at the same time: never touch its files.

## Paths

- `R` = `/Users/ueli/Documents/semio` (follow `R/CLAUDE.md` and `R/AGENTS.md` strictly).
- `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage` (read `S/README.md` fully first).
- `T` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE` (this ticket).
- `A` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE` (authoring rules and the generators of the existing humans).
- `V` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE` (3D volume authoring).
- `M` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE` (top/front/side coverage).
- `G` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET` (silhouette guard fixes of the humans; validate-some helpers).
- `P` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE` (per-scale, per-mode 3D volumes).

## Read first (binding)

1. `S/README.md`, `S/🧬️schema/🟦️.ts` (v2 types), `S/🧬️schema/🔣️.json`.
2. `A/authoring-v2.md`, `A/authoring.md`, `A/content-worker-brief.md`, `A/content-humans-a.md`, `A/content-humans-b.md`, `A/HUMANS-PRINT.md`, `A/refine-humans-animals.md`, `A/core-v2-status.md` (check / preview CLIs), `A/content-v2-CV-H1.md` … `CV-H3.md`, `A/content-v2-NW-H2.md`, `A/content-v2-NW-H3.md`.
3. `V/authoring-humans-a.md`, `V/authoring-humans-b.md`, `V/volume-drafting.md`, `V/volume-agreement.md`, `V/lateral-ambiguous.md`; `M/📝️sonnet-humans.md`; `G/humansb-common-causes.md`, `G/silhouette-humans-a.md`, `G/silhouette-humans-b.md`; the human notes in `P/`.
4. The closest existing human(s) in `S/🗂️catalog/👤️humans/<id>/🔣️.json` and their generators in `A/🧰️scripts/` (`CV-H1`, `CV-H2`, `CV-H3`, `CVF-H1`, `CVF-H2`, `NW-H1`, `NW-H2`, `NW-H3`, `HUM-FIX`) and `G/humans*`. Reuse the existing generator libraries by importing them from their place — do not copy them. Deriving rig, outlines and volumes from the closest existing figure (as `standing-adult` was derived from `standing-phone-user`) is the expected route; the result must still be a distinct, handcrafted person.

## Your job

Author your assigned new people as complete `semio.staffage/v2` assets of category `humans` at the same quality level as the best existing humans:

- `S/🗂️catalog/👤️humans/<id>/🔣️.json`, one folder per asset. Work ONLY in your own asset folders. Others work on other people, plants, the engine and the manifest at the same time; never edit another asset, the engine, the schema, tests, fixtures or the manifest.
- Anthropometry true to the person (de Leva / DIN 33402-2 / ANSUR II; children by age; state stature, mass, sex-neutral or specified build), the human rig with joint limits, segment masses, centre of mass; clothing and carried objects as materials with emissivity and absorptivity; at least 3 poses that fit the activity and at least 2 clips (one looping activity clip, plus `idle-breathing` or a gait), activity `met` per pose (ISO 7730 / ISO 8996 / Ainsworth compendium), thermal body, respiration and clothing regions (ISO 9920 clo), structure (supports, joint loads), every parameter tagged with `source` and `reference`.
- Representations `500`, `200`, `100`, `50` individually designed per scale, plus the authored 3D volumes per scale (body, thermal, structure lists; see `V/`, `P/`), top/front/side coverage (see `M/`).
- All texts `{ en, de }` (English first, natural German second, no default language). Use gender-neutral German where the existing humans do.
- Total power: metabolic heat of the body plus the nameplate of any powered carried device (see README "Power").

## Generators

Your generator scripts and their design inputs go to `T/🧰️scripts/<worker-id>/` (keep them; they are inputs). Generated previews and logs go to `T/🗑️generated/<worker-id>/` (delete raw logs at the end; keep only evidence PNGs you cite). Shipped JSON contains only schema shapes; never hand-edit the shipped JSON without updating the generator. Keep every path under 256 chars.

## Check and look

Run `validateAsset` on your assets (pattern: `G/machinery-validate-some.ts`, adapted to `👤️humans` and your slugs in your own scripts folder, run with the repo's pinned bun as in `G/humansb-A2-env.sh`, or `repo tools exec bun -- …`) and the staffage check CLI (see `A/core-v2-status.md`) until it reports zero issues for every asset you own: all poses, clip instants, thermal, structure, all 7 representations, the 3D volumes. Render the default pose at 1:50, 1:100, 1:200, 1:500 plus plan / front / side and READ the PNGs; iterate until each reads well at print size and looks like that person. Run the existing staffage tests that cover every catalog asset (asset-validation, asset-power, catalog-silhouette, multiview-coverage, breathing-catalog, catalog-physics, volume tests, …) through `repo test-file` scoped to your assets where the test allows it, and fix every failure caused by your assets.

## Manifest

Do NOT edit `S/🗂️catalog/🔣️.json`. Write one manifest entry per asset (exactly the shape of the existing `entries` rows, values derived from your evaluated asset: `heightM`, `widthM`, `powerW`, poses, clips, representations, symbols, analytics, exports, distinction, completion) to `T/📥️manifest-rows/<id>.json`. The coordinator merges all rows and collections at once.

## Report

Write `T/📝️<worker-id>.md`: per asset the anthropometric and activity reference data with sources, rig, poses, clips, representation decisions per scale, 3D volumes, check output, tests run, preview PNGs inspected, remaining doubts. Final message: short summary plus the report path.

## Rules recap

No git commands that modify anything, no worktrees, no IDEs, no `repo dashboard`. `[DEBUG] ` prefix on temporary logs. Files change under you (replica daemon, other workers): re-read before editing. Do not stop halfway; do not ask questions; be thorough.
