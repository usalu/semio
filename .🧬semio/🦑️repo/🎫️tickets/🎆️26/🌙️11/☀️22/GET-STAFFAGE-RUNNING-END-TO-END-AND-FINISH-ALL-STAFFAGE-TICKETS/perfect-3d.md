# 🗿 Finisher report: Perfect Handcrafted 3D Models per Scale and Mode (ticket 26/11/14)

Finisher session 2026-11-22 20:26 to 23:10 (restarted by the host at about 22:20; nothing was lost, the sweep had ended at 22:10). `K` = the ticket folder, `S` = the staffage product, `U` = this folder. Scope: the 152 original catalog assets (`K/rework/01.txt` … `19.txt`); the 124 `used-*` / `reclaimed-*` manifest entries and the new humans and plants were not touched and not validated.

Every proof line below was run to its end in this session with the pinned bun, one tool process at a time. "Full proof" = `r3-validate.ts <asset> --catalog-only --volumes`, "check" = `S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>`.

## 1. Assets whose rework was in progress at the cut-off

How they were found: file times in `K` and in the catalog after 15:30, the per-asset reports `K/rework/assets/*.md`, the last files under `K/🗑️generated`, and a comparison of every volumes source `K/volumes/<category>/<slug>.json` with `asset.volumes` of the catalog file (150 of 152 identical at 20:30; the two that differed were `machinery/diesel-generator` and `vegetation/japanese-maple`, both left by the limit stop of 01:34).

| Asset | State found | What the finisher did | Final proof | Sheets opened and read |
|---|---|---|---|---|
| `animals/domestic-cat` | five scripts (girdle, neck and 1:200 bones, sitting, gait, gait reach) all applied, last write 17:27:22; not validated, no report of the pass | nothing changed; proof, look, report section | full proof `VALID … schema 0, validateAsset 0, checkVolumes 0`; check `✓ domestic-cat (1/1)` (1:500 unavailable by decision) | 1:50 body standing, sitting, walk-phase-3; 1:50 structure; 1:200 body; 2D 1:50 sitting |
| `humans/person-on-crutches` | girdle, forefoot and finger scripts published 17:27:01 (53 bones); not validated, no report of the pass | nothing changed; proof, look, report section | full proof `VALID … checkVolumes 0`; check `✓ person-on-crutches (1/1)` | 1:50 body tripod-stand and body-swing; 1:50 and 1:100 structure; 1:500 body; 2D 1:50 |
| `objects/drinking-fountain` | two scripts (splash beads, tie rods on lugs, drain, supply, riser rib) applied 17:26; not validated, no report at all | nothing changed; proof, look, report written | full proof `VALID … checkVolumes 0`; check `✓ drinking-fountain (1/1)` | 1:50 body; 1:50 body cut; 2D 1:50 |
| `vegetation/small-leaved-lime` | catalog = publication of 13:58, which the author's own run of 14:05 measured `INVALID … checkVolumes 10`; a newer candidate (17:22) was unpublished, its last validation ended after the cut-off with 2 `PRINT` findings (1:500, shape `ln3a` 0.185 mm) | measured the cause (a far-side branch shows a 95 mm wedge beside the stem in the side elevation), redrew one point of that shape (`rework-small-leaved-lime-print500.py`), validated the candidate, published it | candidate and catalog: full proof `VALID vegetation/small-leaved-lime (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check `✓ small-leaved-lime (1/1)`, 1:500 min feature 0.224 mm | candidate 1:50 body, 1:500 body, 2D 1:500 |
| `humans/parent-carrying-toddler` | catalog of 03:50; a candidate of 17:17 (clavicles and scapulae as real bones, hip-bone solids re-bound, jeans ankle taper, 2D redrawn) was valid but unpublished; no report of the pass | validated the candidate, published it; check CLI then failed on a polygon union at `walk-carrying@0.750` (shared-code defect, see section 6); pose `walk-6` swings the far arm 1° instead of 0° (script constant and candidate), published again | full proof `VALID humans/parent-carrying-toddler (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check `✓ parent-carrying-toddler (1/1)` | candidate 1:50 body standing-carry and hoisting; 1:50 structure from behind |
| `vegetation/multi-stem-birch` | "veil" crown, joins and 2D redraw applied until 17:05 and validated by the author at 17:09; report stopped at 15:19 | nothing changed; proof, look, report section | full proof `VALID … checkVolumes 0`; check `✓ multi-stem-birch (1/1)` | 1:50 body |
| `furniture/bookshelf` | `rework-a1-bookshelf-scales.py` applied 16:01 (coarse scales per feature); no report | nothing changed; proof, look, report written | full proof `VALID … checkVolumes 0`; check `✓ bookshelf (1/1)` | 1:50 body; 1:500 body |
| `vegetation/japanese-maple` | rebuilt volumes source of 01:34 (batch 13) never spliced; the catalog still carried the old lists and was `INVALID` (60 `volumes-feature`) | source merged: one finding (1:500 support disc 40 mm) → 75 mm (`rework-finish-500-supports.py`); full run with the source `VALID … checkVolumes 0`; published with `volumes-insert.py` | full proof `VALID vegetation/japanese-maple (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check `✓ japanese-maple (1/1)` | 1:50 body; 2D 1:200 |
| `machinery/diesel-generator` | rebuilt volumes source of 01:34 (batch 10) never spliced; catalog `INVALID` (22 `volumes-feature`) | source: 1:500 silencer bearers 60 → 75 mm (same script). Full run with the source: `INVALID … schema 0, validateAsset 0, checkVolumes 3` (`volumes-silhouette` 1:200 elevation p95 187.5 mm against 60, 1:500 elevation IoU 0.891 against 0.9, `volumes-cross-scale` 1:200 against 1:100). **Not published**: the coarse body lists of the source lack the bottom end rail, bottom castings and doors that the drawings and 1:100 have; that is modelling and drawing work of the rework, not a finishing step | catalog unchanged: `INVALID machinery/diesel-generator (catalog file as it is): schema 0, validateAsset 22` | none |
| `vehicles/city-car` | found by the sweep: `INVALID … validateAsset 8` (12 mm brake discs under the per-scale feature rule) | `rework-city-car-coarse-discs.py`: rear discs 15 mm at 1:100, 30 mm at 1:200, one 75 mm brake body at 1:500; published | full proof `VALID vehicles/city-car (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check `✓ city-car (1/1)` | 1:500 body |

Passes that had ended with a report shortly before the cut-off and were left as they are (source = catalog, cheap validation `VALID` in the sweep; their open lists stand in their reports): `animals/goat` (17:22), `animals/red-squirrel` (17:22), `machinery/oil-pumpjack` (17:13), `humans/standing-adult` (17:10), `vegetation/lombardy-poplar` (17:05), `humans/seated-office-worker`, `humans/walking-commuter` (16:54), `humans/person-with-umbrella` (16:43), `animals/dairy-cow` (16:29).

Two reported passes that are **not** proven:

- `machinery/tower-crane`: the report of 17:16 states two remaining measured findings (1:500 `volumes-silhouette`, elevation IoU 0.794, plan 0.833) that no asset-side change satisfies together with the print rule (section 6). Not re-measured by the finisher (the full run takes 25 minutes); cheap validation `VALID`.
- `vegetation/weeping-willow`: the rebuilt candidate was never valid (11 measured findings, report of 17:14) and its candidate folder was deleted by its author; the catalog still holds the round-3 model of 2026-11-21 (cheap validation `VALID`). The rebuild is in `rework-weeping-willow-model.py` and waits for a next pass.

Assets that were only being looked at when the session was cut off (sheets rendered 17:06 to 17:26, no script, no catalog write): `humans/running-child`, `vehicles/city-bicycle`, `animals/sheep`, `vegetation/apple-tree`, `components/tied-canopy`. Nothing to finish; they count as not started for that pass.

What stays imperfect on the finished assets is written at the end of each section the finisher added to `K/rework/assets/<category>--<slug>.md`. The common points: the finisher read 3 to 7 sheets per asset, not the full set the rework brief asks for (every scale × list, every pose, two instants per clip), and wrote no own reference table; none of these assets has had the independent review of `K/review-brief.md` (`K/review/` is empty).

## 2. Catalog sweep (cheap validation of the 152 originals)

`K/finisher-sweep.sh` (`r3-validate.ts <asset> --catalog-only`: schema, `validateAsset`, print checks; 20:35 to 22:10): **128 VALID, 24 INVALID, none crashed, every file parses.** After the repairs above (`vehicles/city-car`, `vegetation/japanese-maple`): **130 VALID, 22 INVALID.**

None of the remaining 22 was left invalid by the 17:27 cut-off: their catalog files were last written between 2026-11-21 and 2026-11-22 01:36. 20 fail through the per-scale `volumes-feature` rule that went live on 2026-11-22 01:23 (coarse scales that still carry the 1:50 parts list), one through `lateral-ambiguous`, one through a print finding only. They are the unstarted part of the rework queue and were not started now. Codes measured at 22:15 (`--catalog-only --no-print`, agave with print):

| Asset | Findings |
|---|---|
| `components/masonry-wall-with-lintel` | `volumes-feature` × 525 |
| `components/etics-brick-wall-12`, `components/etics-brick-wall-30` | `volumes-feature` × 122 each |
| `components/curtain-wall-bay` | `volumes-feature` × 119 |
| `components/solid-brick-wall-1960` | `volumes-feature` × 112 |
| `components/tied-canopy` | `volumes-feature` × 87 |
| `vehicles/cargo-bike` | `volumes-feature` × 72 |
| `components/sliding-door` | `volumes-feature` × 65 |
| `machinery/solar-thermal-collector` | `volumes-feature` × 34 |
| `components/sectional-garage-door` | `volumes-feature` × 32 |
| `components/tilt-window` | `volumes-feature` × 23 |
| `machinery/diesel-generator` | `volumes-feature` × 22 (rebuilt source exists, see section 1) |
| `components/panel-radiator` | `volumes-feature` × 20 |
| `appliances/hot-water-cylinder` | `volumes-feature` × 18 |
| `vehicles/city-bicycle` | `volumes-feature` × 16 |
| `machinery/scroll-compressor` | `volumes-feature` × 11 |
| `objects/waste-bin` | `volumes-feature` × 10 |
| `furniture/park-bench` | `volumes-feature` × 3 (three 19 mm bolts in the 1:500 structure list) |
| `machinery/heat-pump-outdoor-unit` | `volumes-feature` × 3 (8 mm fan blades at 1:100, 1:200, 1:500) |
| `objects/storage-crate-stack` | `lateral-ambiguous` × 3 (front shapes of the turned crates 3, 4, 5) |
| `components/external-venetian-blind` | `volumes-feature` × 2 (16 mm edge spacer at 1:200, 1:500) |
| `vegetation/agave` | `PRINT range-end-feature` × 2 (two leaf shapes 0.198 mm at 1:150) |

Decision: the finisher repaired `vehicles/city-car` (an asset with a second-round pass, eight findings of one kind) and the two interrupted publications; the small cases at the end of the table (park-bench, heat-pump, venetian blind, agave, crate stack) are quick for an author but each needs the coarse scale looked at and redrawn, so they were left in the queue with the rest.

The measured rules (`--volumes`) were not swept over the catalog (minutes per asset, up to ten for the 17 to 22 MB trees). Proven with `--volumes` in this session: the eight assets marked `VALID` in section 1 and the maple.

## 3. Acceptance of the ticket, from the ledgers and from what was measured

Order of the dev: perfect handcrafted 3D models for all 152 assets, different and consistent per scale and mode, end to end (schema, kernel, authoring, exports, viewer, third-party oracle tests).

| Part | State | Evidence |
|---|---|---|
| Schema (`volumes` per scale with body, thermal, structure, decisions; `sheet`, `slender`, `members`, `envelope`, lateral rest placement of bones) | done according to the ledgers; not re-run | `K/validator.md` "Final state 15:00": `🧊️volume-schema` 115 passed, typecheck 0 errors; `K/rig-lateral-rest.md` |
| Kernel and volume rules | done according to the ledgers; not re-run. One new defect found today: `unionPolygons` on an edge-on extruded shape (section 6) | `K/validator.md`: `🧊️volumes-rules` 128, `🧊️volumes-connected` 101, `🧊️volumes-flood` 29 (third-party voxel flood oracle), `🧊️volumes-thermal` 12, `✅️asset-validation` 83, `🧊️model-frame` 30, `🧊️solid-sweep` 18, `🧊️lateral-spread` 27, `🧿️exposed-normals` 7, all passed at 15:00 |
| Authoring of all 152 assets per scale and mode | **open**. Every asset has authored `volumes` at its authored scales; 130 of 152 pass the cheap validation, 22 do not (section 2). Of the dev's rework verdict ("incredibly sloppy … every model handcrafted"): 95 assets had a per-asset rework report with at least one pass, every one of them ends "partial" or with an open list (the finisher added three reports: drinking-fountain, bookshelf, japanese-maple); 55 assets never got a per-asset pass (section 4); **no asset has been independently reviewed and accepted** | `K/rework/assets/` (98 reports), `K/review/` (empty), sweep of this session |
| Catalog-wide suites over the assets | **open, known red**: `🧊️model-skeleton` 22 of 27 (named: elderly-with-cane, person-on-crutches, tour-guide, excavator, goat, parent-carrying-toddler, parent-with-stroller), `🧊️three-views` 314 of 328 (15 assets named in `K/validator.md`), state of 15:00; both lists predate the afternoon reworks of several named assets (person-on-crutches, goat, parent-carrying-toddler) and must be re-run. `🧪️tests/🌡️catalog-physics`, `⚡️asset-power`, `model-bounds` were never re-run after the night's changes of `areaM2`, hidden regions and wall thicknesses (`K/rework/queue.md`, batches 17 and 19) | `K/validator.md`, `K/rework/queue.md` |
| Exports | not re-verified in this session; the check CLI (evaluation, physics, depiction in seven views and three viewpoints at every authored scale, every pose, nine instants per clip) is `✓` for the nine assets proven in section 1 | check lines above |
| Viewer / site | loading fixed and tested on 2026-11-22 (React target 322 of 323, the one failure being the archive-validity test that needs a valid catalog; site 15 passed, 1 skipped); remaining site points 1 to 6 in `K/site-loading.md` (dev server stalls under heavy replication, page reloads on asset change, off-screen stages). Not re-run. New risk measured today: the reworked trees are 17 to 22 MB per asset file (small-leaved-lime 22.1, lombardy-poplar 21.2, english-oak 19.5, multi-stem-birch 17.3 MB) and take minutes to validate | `K/site-loading.md`; file sizes of the catalog |
| Tests with third-party oracles | rule level: present and green at 15:00 (flood oracle, area and silhouette fixtures). Asset level: the catalog suites above are the acceptance and are red | `K/validator.md` |

## 4. Rework queue: not started (stay unstarted)

55 assets never had a per-asset rework pass (batch number in brackets; several were touched or only judged by their batch author in the night, see `K/rework/<NN>.md`); those marked ✗ are among the 22 invalid ones:

- appliances: ceiling-diffuser, circulation-pump, desktop-monitor, duct-fan, electric-kettle, electricity-meter, gas-boiler, gas-meter, hot-water-cylinder ✗, laptop, mvhr-unit (half done in batch 17: porthole, impeller and air paths wrong), room-thermostat, split-ac-indoor-unit, thermostatic-radiator-valve, toaster
- components: curtain-wall-bay ✗, etics-brick-wall-12 ✗, etics-brick-wall-30 ✗, external-venetian-blind ✗, masonry-wall-with-lintel ✗, panel-radiator ✗, sectional-garage-door ✗, sliding-door ✗ (closed leaf stands mid-opening), solid-brick-wall-1960 ✗, tied-canopy ✗, tilt-window ✗
- furniture: folding-deck-chair, operating-table (not articulated), park-bench ✗, picnic-table
- machinery: diesel-generator ✗ (source rebuilt, three measured findings), evaporative-humidifier, expansion-valve, heat-pump-outdoor-unit ✗, plate-heat-exchanger, scroll-compressor ✗, solar-thermal-collector ✗, sorption-wheel
- objects: bicycle-stand, chalkboard, cooking-pot, retractable-bollard, storage-crate-stack ✗, tealight, waste-bin ✗
- vegetation: agave ✗, apple-tree, date-palm, hornbeam-hedge, hosta-clump, ivy-on-trellis, japanese-maple (batch rebuild now published), miscanthus-grass
- vehicles: cargo-bike ✗, city-bicycle ✗

Also queued and not started: a further pass for each of the 97 assets with a pass (each report ends with its precise open list; `vegetation/weeping-willow` and `machinery/tower-crane` first), the consumer follow-up of the lateral rest placement for humans and animals that still lack girdle bones, and the whole independent review wave (`K/review-brief.md`, 152 assets, none done).

## 5. What the umbrella coordinator must run before the ticket can close

The ticket cannot close on today's state: the authoring acceptance is open (sections 3 and 4). To measure where it stands, one process at a time, on a quiet host:

1. `S/🧪️tests/🧊️model-skeleton` and `🧊️three-views` (expected red: lists in `K/validator.md`, to be re-read after today's reworks).
2. `S/🧪️tests/🧊️model-catalog`, `🧊️catalog-silhouette`, `🌡️catalog-physics`, `⚡️asset-power`, `model-bounds` (never re-run after the rework wave; the site agent's accidental run in the morning saw 323 failures in the library suites, mostly `🧊️model-catalog` 144 and `🧊️catalog-silhouette` 112, on assets then under rework).
3. The rule suites of `K/validator.md` "Final state" again (they were green at 15:00; `🧊️volumes-flood` needs a load below about 10).
4. The measured sweep `r3-validate.ts <asset> --catalog-only --volumes` over the 152 originals (not done by anyone yet; hours).
5. The check CLI without `--asset` (manifest against all assets) and the React target test `📚️catalog-loading` "loads the authored archive as one ready row per manifest entry": both need the 22 invalid assets repaired first.
6. The site build and the site tests.

## 6. Shared-code requests (written to `U/requests.md`, both open)

- `PERFECT-HANDCRAFTED-3D-MODELS — unionPolygons fails on an extruded 2D shape seen exactly edge-on` (`S/🔨️modules/🔺️polygon/🟦️.ts`), measured on `humans/parent-carrying-toddler`; worked around in the asset.
- `PERFECT-HANDCRAFTED-3D-MODELS — volumes-silhouette against the print rule for lattice bars at 1:500` (`machinery/tower-crane`), carried over from the author's report, not re-measured.

Older open points of the ticket's own ledgers that concern shared code and were not carried over one by one: `K/validator.md` "Still open" (weighted solids in `🌓️silhouette` after the rig change, goat's proposal on the drawn-width concession), `K/tool-requests.md` last section (`copyMesh` drops `weights`), `K/site-loading.md` "What remains".

## 7. Clean-up

Before deleting, the finisher listed every file under `K/🗑️generated` written since 20:20: outside `r3-sheet/file`, `r3-2d/file` and `volumes-tools/cache` only its own files were new, and no foreign tool process was writing there.

- Removed: 3825 top-level entries of `K/🗑️generated` (logs, result json, png, candidate documents, among them the candidate folders `rework-small-leaved-lime` and `r3-parent-carrying-toddler` whose documents are now the catalog assets), every category folder under `K/🗑️generated/r3-sheet` and `K/🗑️generated/r3-2d`, and the finisher's own sweep and proof files. 4.5 GB → 2.2 GB.
- Kept on purpose: `K/🗑️generated/r3-sheet/file` and `K/🗑️generated/r3-2d/file` (the wave finishers render there; they also hold eleven sheets of the finisher's two candidates), and `K/🗑️generated/volumes-tools/cache` (2.2 GB tessellation cache that every running `r3-*` tool of the other finishers reads and writes; 20 397 files were written into it since 20:20). The umbrella coordinator removes these three when all finishers have ended.
- Kept as inputs: every script, brief, audit and report at the root of `K`, including the 209 root `*.json` (texts, baselines and before-snapshots that scripts read) and the two `*.txt` notes. No raw log or result file lies at the root of `K`.
- Consequence for the next author: the candidates of `vegetation/weeping-willow` and the looks of earlier passes are gone; they are rebuilt by their round scripts.

## 8. Files created, updated, removed (repo-relative)

`K` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog`.

Created:
- `K/finisher-sweep.sh`, `K/finisher-slot.sh`, `K/finisher-print-probe.ts`, `K/finisher-union-probe.ts`
- `K/rework-small-leaved-lime-print500.py`, `K/rework-05-parent-carrying-toddler-publish.py`, `K/rework-05-parent-carrying-toddler-walk6.py`, `K/rework-city-car-coarse-discs.py`, `K/rework-finish-500-supports.py`
- `K/rework/assets/objects--drinking-fountain.md`, `K/rework/assets/furniture--bookshelf.md`, `K/rework/assets/vegetation--japanese-maple.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/perfect-3d.md`

Updated:
- `C/🌳️vegetation/small-leaved-lime/🔣️.json`, `C/👤️humans/parent-carrying-toddler/🔣️.json`, `C/🚜️vehicles/city-car/🔣️.json`, `C/🌳️vegetation/japanese-maple/🔣️.json`
- `K/volumes/vegetation/small-leaved-lime.json`, `K/volumes/humans/parent-carrying-toddler.json`, `K/volumes/vehicles/city-car.json`, `K/volumes/vegetation/japanese-maple.json`, `K/volumes/machinery/diesel-generator.json` (source only, not published)
- `K/rework-05-parent-carrying-toddler-roots.py` (line 234, `ARM_SWING`)
- `K/rework/assets/animals--domestic-cat.md`, `humans--person-on-crutches.md`, `vegetation--small-leaved-lime.md`, `humans--parent-carrying-toddler.md`, `vegetation--multi-stem-birch.md`, `vehicles--city-car.md` (one section appended each), `K/rework/queue.md` (section "23:00 — umbrella finisher")
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (two sections appended)

Removed: see section 7. No manifest edit, no shared engine, schema, validator, test, fixture, README or site file was touched; no `r3-*.ts` or `volumes-*` tool was changed.
