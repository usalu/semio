# 🔭️ Multiview Batch MB-1 (2026-11-24)

Batch: 19 assets whose main finding was a band copy (`🧰️matrix/batches/MB-1.tsv`). Scripts: `🧰️matrix/MB-1/`. Paths as in `📋️multiview-brief.md` (`S`, `T`, `U`).

## Result

- **12 of 19 assets have no failing cell** in `T/📊️matrix.ts --assets <id>` (252 cells each) and are clean in `T/🔎️scan.ts`: bookshelf, curtain-wall-bay, dining-table, duct-fan, external-venetian-blind, hot-water-cylinder, kitchen-extractor-hood, park-bench, reclaimed-steel-column, single-bed, used-crt-television, used-trench-convector.
- **7 assets keep 168 failing cells** (of 1,764), all `derived-view-band-copy` or `derived-view-in-band` on views that are physically the same drawing: etics-brick-wall-12 (35), etics-brick-wall-30 (35), solid-brick-wall-1960 (42), steel-column (21), glulam-beam (7), spiral-stair (14), stair-flight (14). They are in `requests.md` (`## multiview MB-1 — …`). Each of them is better than before in the scan and worse in no other test.
- All 19 asset files changed; no shared code, test, fixture or manifest was touched.

## Rule of the redraws

A band draws in its front and plan the parts that its own side elevation and volume carry, also those another part covers in the rest pose. A finer band then shows more than a coarser one. Nothing was produced by turning, mirroring or flattening another view; every new shape has hand-set coordinates taken from the dimensions of the solid or of the side elevation of the same band.

Plainly: in seven assets the added shapes lie under another part in the rest pose, so the printed sheet of the default pose does not change there (marked "covered" below). The band copy is gone in the matrix because the drawing data differ; a reader of the sheet sees the difference only where the table says "visible".

## Per asset

Findings "before" are those of `🧰️matrix/catalog-scan.ts` on 2026-11-24 before the repair (derived flags counted per band and view; `sync-bone-*` "other shape id" and the moments cue went away with the shared-code change of the other worker and are not listed as repaired). "Cells" is the matrix after the repair.

| Asset | Before | After (scan) | Cells: fail | Change |
|---|---|---|---|---|
| bookshelf | band copy plan 50≡100, 200≡500 | clean | 0 | Plan: back panel and plinth at 1:50, plinth at 1:100 and 1:200 (covered). |
| dining-table | band copy plan 50≡100, 200≡500 | clean | 0 | Plan: four legs at 1:50, 1:100, 1:200; aprons at 1:50 (covered). |
| park-bench | band copy plan 200≈500 (0.0022); `anchor-landmark` 12 (contacts without z, 135 mm off in the front) | clean | 0 | Plan 1:200: front and rear foot pad on each foot strip (visible). Contacts split per pad: `contact-left-front/-rear`, `contact-right-front/-rear` at z -0.268 and 0.245 (D45); pose and clip contacts, `structure.supports` and the support solids of the structure volumes follow. |
| single-bed | band copy plan 50≈100 (0.0047); `anchor-landmark` 25 (10 off every shape, 15 on a descendant) | clean | 0 | Centre leg in every plan (covered) and in the front of 1:500, the only one that lacked it (visible); cross beam in the plan at 1:50 (covered). `foot-head` split into `foot-head-l/-r` at z ±0.475 (D45), references and support solids follow. |
| duct-fan | band copy plan 100≈200; in-band plan 50 = front turned 90° | clean | 0 | Rotor seen through the housing, as 1:100 and 1:200 already draw it in the front: shaft and one blade in front and plan at 1:50, the blade in the plan at 1:100 (visible). The blade that crosses the plan's axis plane lies further out than the one of the front, so the views are no quarter turn. |
| kitchen-extractor-hood | band copy plan 50≈100 (0.0047); matrix: `language-geometry` 42 cells in the plan (not seen by the scan) | clean | 0 | Plan 1:50: spindle of the backdraught damper down the open duct (visible). Section label of the side elevation `A-A` in both languages at 1:50 and 1:100: it was a sentence that differs per language, so the cut-line marker of the plan had other geometry in German. |
| used-crt-television | band copy plan 200≡500; `sync-forward` 16 (pose `carried`: 20 mm) | clean | 0 | Plan: the round swivel stand in all four bands (visible: 20 mm proud of the bezel; this was the synchrony finding); picture tube behind the bezel at 1:200 (covered). |
| hot-water-cylinder | band copy plan 200≡500; moments cue (shared code) | clean | 0 | Plan: base ring under the jacket at 1:50, 1:100, 1:200 (covered). |
| curtain-wall-bay | band copy front and plan 100≈200 | clean | 0 | 1:100 plan section: mullions cut with their glazing rebates, glass 15 mm into them, as the 1:100 volume is built (visible). 1:100 front section: slab cleat and head of the hanging bolt beyond the cut (visible). |
| external-venetian-blind | band copy plan 100≈200≡500 | clean | 0 | Plan: guide rails under the box (three bands), window frame and glass under the wall head (glass shorter at 1:500 as in its side elevation), sill board at 1:100 (all covered). |
| reclaimed-steel-column | band copy plan 50≈100 (0.0042) | clean | 0 | Plan 1:50: the web solid between the flanges, the one the front already places (covered). |
| used-trench-convector | band copy front 100≈200≡500 | clean | 0 | Front: levelling brackets beside the trough at 1:100 (visible); end of the fin pack at 1:100 and 1:200, pipe tails at 1:200 (covered). |
| glulam-beam | band copy plan 100≡500; in-band plan 500≈elevation (0.0288) | in-band plan 500 (0.0288) | 7 | Plan: bearing plates at 1:50, 1:100, 1:200 (covered), the two finger joints of the top lamella also at 1:100 and its grain at 1:50 (visible). Open: 1:500, see requests. |
| steel-column | band copy plan 200≡500; in-band front≈elevation at 100, 200, 500 | in-band front at 100 (0.0252), 200, 500 (0.0067) | 21 | Plan 1:500 one block without the cap plate, as its side and front (visible). Open: the two side views of an HEB 200, see requests. |
| stair-flight | band copy plan 50≡100; front 100≈200 (0.0038) | front 100≈200 (0.0038) | 14 | Plan 1:50: base plate and head plate of the stringer (head plate visible at the top edge). Open: front. |
| spiral-stair | band copy front and plan 100≡200 | front 100≡200 (0) | 14 | Plan 1:100: inner edge of the stiffening flange under the landing, which only the 1:100 volume has (covered). Open: front. |
| etics-brick-wall-12 | band copy front in four bands; in-band plan = elevation at 100, 200, 500 | front in four bands; in-band plan 500 | 35 | Side elevation (the section): bed joints of the twelve courses at 1:100; six bond bands and the joint of the two insulation courses at 1:200 (visible). The in-band copy at 1:100 and 1:200 is gone. |
| etics-brick-wall-30 | as above | as above | 35 | As above. |
| solid-brick-wall-1960 | as above, and band copy plan 200≡500 | front in four bands; plan 200≡500; in-band plan 500 | 42 | Bed joints at 1:100, bond bands at 1:200 in the side elevation (visible). |

## Proof (each run to the end in this session, on the final files)

- `T/🔎️scan.ts --language en` over the 19: 12 clean, 7 with a hard finding, 49 hard findings (all derived-view flags of the seven assets above).
- `T/📊️matrix.ts --language en --jobs 1 --assets …`: 19 records, outcome `done`; 4,788 cells; 168 fail in the seven assets above, 0 in the other twelve (statuses there: ok, `group-drift`, `unavailable`).
- `validateAsset`, original against repaired file (`validate.sh`): 0 issues before and after for all 19.
- Check CLI `🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>`: `✓` for all 19.
- `🧊️catalog-silhouette` with the name filter over the 19: passed (21 tests; park-bench first failed at 1:200 plan with IoU 0.918 when the pads replaced the foot strip, so the strip stayed and the pads are clipped to it; rerun passed). kitchen-extractor-hood rerun after its last change: passed.
- `🧊️three-views` 42 passed, `🌡️catalog-physics` 42 passed (both with the name filter over the 19); `🫁️breathing-catalog` has no test for these assets; `✅️asset-validation` 83 of 83 and `⚡️asset-power` 24 of 24 by describe name.
- `🔭️multiview-coverage`, the case that names bookshelf, glulam-beam, steel-column and single-bed (thermal legend): passed.
- `fix.py all` a second time: every asset "already repaired, nothing to write".
- Sheets: product sheets (`look.sh`) rendered for the 14 changed scales with a visible change; they are too small to judge a detail, so the views were also drawn large from the evaluated shapes (`zoom.sh`). Looked at: duct-fan 1:50, curtain-wall-bay 1:100, used-crt-television 1:200, park-bench 1:200, stair-flight 1:50, used-trench-convector 1:100, etics-brick-wall-12 1:200. Not opened one by one: the scales whose added shapes are covered, and kitchen-extractor-hood 1:50, glulam-beam 1:50 and 1:100, steel-column 1:500, the two other walls.

## What stays open

1. Seven assets, 168 cells, as listed; no truthful redraw exists for a rendered wall face, a prismatic beam or column and the two stair fronts under the current thresholds (numbers in `requests.md`).
2. duct-fan draws shaft and blade of the rotor on the closed housing in front and plan. That is the convention the asset already had at 1:100 and 1:200; it was extended to 1:50, not judged.
3. The manifest was not edited; park-bench and single-bed have new anchor ids.
4. `group-drift` (reported tier) remains in dining-table, duct-fan, hot-water-cylinder, kitchen-extractor-hood, spiral-stair.

## Follow-up: allowlist entries (coordinator ruling, D47 of the suite)

`🧰️matrix/MB-1/allowlist.json` holds the entries for the owner of the fixture: 33 under `bandCopyAllowlist` (asset, view, the two bands, reason `{en, de}`), 5 under `derivedViewAllowlist` (asset, scale, view, against, reason) and 2 under `open`. No asset file changed in this follow-up, so the proofs above stand.

- **Open assets, truthfully the same view:** the front of the three walls (all six band pairs each), their plan against the section at 1:500, the plan of solid-brick-wall-1960 at 1:200 and 1:500, steel-column front against side at 1:200 and 1:500, the fronts of spiral-stair and stair-flight at 1:100 and 1:200.
- **Stay open, not truthfully the same:** steel-column front against side at 1:100 (0.02522; the two views draw the stiffeners and anchors differently, the threshold is 90 mm) and glulam-beam plan against side at 1:500 (0.02875; 160 against 620 mm wide, the threshold is 240 mm). Both are a limit of the measure on slender bodies, not a copied drawing; 14 cells.
- **Assets cleared with covered shapes only:** no visible redraw was added. For each the view is truthfully the same in the two bands and has an entry instead: bookshelf plan (50/100, 200/500), dining-table plan (50/100, 200/500), single-bed plan (50/100), hot-water-cylinder plan (200/500), external-venetian-blind plan (100/200/500), reclaimed-steel-column plan (50/100), spiral-stair plan (100/200), used-trench-convector front (200/500). The covered parts stay in the files. These views pass the matrix today only because of those covered parts; the entries are what justifies them.
- **glulam-beam** needs no band-copy entry: its bands differ visibly (grain at 1:50, finger joints at 1:50 and 1:100, axis at 1:200); the bearing plates are covered. **used-trench-convector** 1:100 differs visibly by the levelling brackets.
- Not searched in this follow-up: finer visible details (edge radii, fixings, texture) for the eight views above; none is in their volumes at the finer band.

## Files

Updated (repo-relative, under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`, each `<id>/🔣️.json`):

- `🔌️appliances/`: duct-fan, hot-water-cylinder, kitchen-extractor-hood, used-crt-television
- `🧱️components/`: curtain-wall-bay, etics-brick-wall-12, etics-brick-wall-30, external-venetian-blind, glulam-beam, reclaimed-steel-column, solid-brick-wall-1960, spiral-stair, stair-flight, steel-column, used-trench-convector
- `🛋️furniture/`: bookshelf, dining-table, park-bench, single-bed

Updated in the ticket: `requests.md` (one section appended).

Created in the ticket (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`): `multiview-MB-1.md`; in `🧰️matrix/MB-1/`: `fix.py` (the repairs, idempotent, runs after the silhouette scripts of SAM, SC and SF), `mb.py`, `env.sh`, `show.py`, `cmp.py`, `ext.ts`, `scan.sh`, `scan-lines.py`, `validate.ts`, `validate.sh`, `vt.sh`, `check.sh`, `look.sh`, `zoom.ts`, `zoom.sh`, `cells.py`, `allowlist.json`.

Removed: `🗑️generated/matrix/MB-1` (tool output, originals of the 19 files included).
