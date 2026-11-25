FINAL

# 🧾️ Audit of Every Staffage Ticket (lane s3-audit, Linux worker g1-m04, 2026-11-25 09:05 to 09:40)

Read-only lane. Nothing was changed except this file. No suite, matrix, check CLI or browser was run: every "verified" line below is a static measurement of the replicated tree at `/home/deck/semio` (file exists, counts over the 375 asset documents, manifest rows, fixture entries, test names, ticket json). The repo MCP server `semio` did not connect in this session; ticket states are read from the `🎫️ticket.json` files.

- `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`
- `TK` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26`, `U` = this folder, `K` = `TK/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`
- **Verified** = measured by me now. **Claimed** = stated in a ticket file or close summary, not re-measured here.
- The Mac's `U/🗑️generated` (scan and matrix records, proof logs) is not replicated to this host (here it holds only `linux-s1`, `linux-s2`), so no matrix record or proof log could be read.

## 1. Result in Short

- 22 tickets are about staffage. **20 are closed, 2 are open** (`K` and the umbrella `U`), and one folder (`🌙️11/☀️05/GET-STAFFAGE-WEBSITE-RUNNING-END-TO-END`) has no `🎫️ticket.json` at all.
- **One closed ticket has a part of its acceptance that is measurably unmet in the tree today**: `🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-…` (and with it the three `USED-*` collection tickets and the reclaimed one, whose descriptions demand volumes at 1:500). 27 `used-*` assets declare `volumes["500"].status = "unavailable"` (one also at 1:200). The ticket's own guard `S/🧪️tests/🧊️model-catalog/🟦️.ts:42` ("ships all four digital scales for every selected asset") asserts all four scales for every asset, so that case is red by construction. The close summary of 2026-11-24 does not list `model-catalog` among its measured suites and does not name this gap. Section 4.
- **Every green of 2026-11-24 is stale for 181 of 375 assets**: 181 asset documents were written after the 3D ticket was closed (2026-11-24 12:18) by the multiview fixers (humans 53, vegetation 36, objects 22, components 21, furniture 18, appliances 17, machinery 8, vehicles 6). The closes of the 3D ticket, the views ticket and the waves rest on runs made before those rewrites. Nothing is known to be broken; nothing is proven either until step 4 of section 5 has run.
- Four closed tickets were closed with a stated remainder that no later ticket carries by name (flow view: playgrounds and compare controls; engine-neutral 3D: Python consumer not re-measured; archive: video export not re-measured; course clips: no narrated full rebuild). Section 3.
- `K` cannot close yet. Beyond the batch work it needs eight things, the first of which no batch covers: **145 assets were never in a fixer batch and have never had a matrix run** (the scan that called them clean visits poses only). Section 5.

## 2. All Staffage Tickets

Status and dates are verified from `🎫️ticket.json`. "Acceptance" compares the ticket description with evidence.

| # | Ticket | Status (last interaction) | Acceptance met? | Left open |
|---|---|---|---|---|
| 1 | `🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE` | closed 2026-11-23 12:38 | Met for what is static: 375 assets, 9 categories, 2,109 poses, 930 clips, every asset with `thermal`, `structure`, `volumes` (verified). Suites and site: claimed (`U/older-tickets.md` section 2, `U/browser-check.md`). | Video export (MP4/WebM, `🎞️frame-timing`, `🗜️zip-reopen`) not re-measured since 2026-10-30/11-05 (stated at close). Playground only seen loading, nothing placed. |
| 2 | `🌙️10/☀️30/STAFFAGE-ACTIVE-CHROME` | closed 2026-10-31 | Claimed, UI detail, no evidence file beyond the json. | Nothing stated. |
| 3 | `🌙️10/☀️30/STAFFAGE-CATALOG-SUITABILITY-PASS` | closed 2026-10-30 | Claimed for 107 assets; superseded by the check CLI 375/375 of 2026-11-24 (claimed). | Nothing. |
| 4 | `🌙️10/☀️30/STAFFAGE-DESKTOP-GRID-SCROLL` | closed 2026-10-30 (reopened once) | Claimed (`live-verification.md`). | Nothing. |
| 5 | `🌙️10/☀️30/STAFFAGE-REACT-RENDER-LAG` | closed 2026-10-30 | Claimed (`render-lag.md`); no before/after measurement named in the close summary. | Nothing stated. |
| 6 | `🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS` | closed 2026-11-05 10:59 (three rounds) | Static part verified: cast manifest `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` has 98 casts over 53 assets, every asset exists in today's catalog, every cast has `viewpoint`; guard `🧪️tests/🐍️staffage_only.py` exists. Test counts: claimed. | "No narrated full-course rebuild or non-Windows execution claimed" (stated at close). The rebuild is carried by the open course tickets (section 6), not by a staffage ticket. 91 ticket files name `C:/git/semio`; 2 `🗑️generated*` folders left in the ticket. |
| 7 | `🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE` | closed 2026-11-23 15:24 | Static part verified: 1,416 authored representations, each with non-empty `elevation`, `front`, `plan`; 84 unavailable, each with `cause`; 152 symbols, each with three views; no `presentation.viewpoints`. Guard `🧊️three-views` 775/775 on 2026-11-24: claimed (`U/🤝️handoff.md`), and stale for the 181 rewritten assets. | The close itself says the full suite was not run again after the last redraw; the 775/775 run of the next day covers that (claimed). 112 ticket files name `C:/git/semio`. |
| 8 | `🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE` (`K`) | **open** (reopened 2026-11-02 23:30) | Not met. Section 5. | Matrix without a `fail` cell, proof sheets, example exports, handoff (still headed "SKELETON"), 169 files naming `C:/git/semio`. |
| 9 | `🌙️11/☀️05/GET-STAFFAGE-WEBSITE-RUNNING-END-TO-END` | **no `🎫️ticket.json`**; one file `📝️research.md` (2026-11-05/07) | Never met in its own folder: three start attempts ended in capacity admission, "No application behavior or tests have yet been verified". | Superseded in substance by #12 (release build, 23 Chromium checks on 2026-11-21, 152 assets) and `U/browser-check.md` (375 assets on the dev server). The folder is an orphan: the MCP listed a ticket but wrote no local json. It needs either a json written by `ticket_open`/`ticket_close` with a pointer to #12 and to lane s1, or removal by the dev. Lane `linux-s1-site.md` is the run that would satisfy it on Linux. |
| 10 | `🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE` | closed 2026-11-23 12:38 | DWG: `S/🔨️modules/📤️export/🖊️dwg/staffage_dwg.wasm` exists (verified, 186,229 bytes); suites claimed. Every asset has `volumes` (verified). | Stated at close: Python consumer against real manim not re-measurable (course Python environment not provisioned on the Mac), `repo run @teaching/staffage:test` did not reach a test in 300 s. Both rest on evidence of 2026-11-12 only. |
| 11 | `🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE` | closed 2026-11-24 12:18 | **Partly unmet, section 4.** Suite counts (silhouette 385/385, check 375/375, three-views 775/775, physics 840/840, …): claimed, and stale for 181 assets. | Stated debt at close: `bamboo-clump` raster-fitted squares, slab-stack leaves (agave, hosta-clump, date-palm), people at the limit, `weeping-willow` 9.5 MB (verified larger files today: `small-leaved-lime` 22.5 MB, `lombardy-poplar` 21.0 MB, `bamboo-clump` 19.9 MB). Not stated: `model-catalog`, the 27 assets without a 1:500 volume, the independent review of `review-brief.md` (`perfect-3d.md`: "`K/review/` is empty"). 41 log/txt files left in the ticket folder. |
| 12 | `🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS` | closed 2026-11-21 10:28 | Met for the catalog of 152 (claimed). | Not re-verified for 375 assets (617 MB of asset JSON today, verified): release build, chunk sizes and the 23 browser checks are of 2026-11-21. No `S/🌐️site/dist` in this checkout (verified). |
| 13 | `🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET` | closed 2026-11-18 | Met for 152 at that date (claimed); superseded by the stricter guard and 385/385 of 2026-11-24 (claimed). | Results exactly at the limit (stated). |
| 14 | `🌙️11/☀️20/OPTIMIZE-STAFFAGE-AND-ARCHITECTURE-COURSE-FOR-PHONES` | closed 2026-11-20 | Claimed (Chromium and WebKit). | "Staffage's separately tracked authored geometry issues remain" (stated); those were the silhouette and three-views lists, closed since (claimed). Mobile width was not looked at in either browser check of the umbrella. |
| 15 | `🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION` | closed 2026-11-23 13:53 | 22 of 40: collection `reclaimed-components` has 22 members (verified). Scope cut to started rows by the dev (claimed, order of 16:29). | Rows 23 to 40 never started. Not every sheet opened. |
| 16 | `🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE` | closed 2026-11-23 15:24 | 75 humans in catalog and manifest (verified); the seven never-started people are absent from the catalog (verified). | Closed while `catalog-silhouette` was red for 22 of 47, handed to #11, which reports 385/385 (claimed). Generators not all re-run after repair ops (stated). |
| 17 | `🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE` | closed 2026-11-23 15:24 | 67 vegetation assets, eight plant collections (verified). | Closed with measured volume-rule findings on 18 of 27 plants and the silhouette guard "not measured to the end"; the silhouette part is covered by 385/385 (claimed), the cross-scale crown findings are carried by nobody by name. |
| 18 | `🌙️11/☀️22/STAFFAGE-THERMODYNAMIC-FLOW-VIEW-INSTEAD-OF-AIRFLOW-CONES` | closed 2026-11-23 15:24 (reopened once) | **Closed with unmet acceptance, stated**: the reopening asked for flow everywhere, full control of wind and every export. | "Playgrounds carry no flow, comparison panes have no air controls of their own, the model-batch suite did not finish within 10 minutes." Today `🧪️playground` has 3 files and `⚖️compare` 1 file that name flow (verified by grep only; whether that is the feature is not checked). No later ticket carries these three points. |
| 19 | `🌙️11/☀️22/USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION` | closed 2026-11-23 12:38 | 35 of 40 members (verified). | Rows 36 to 40 never started; reference tables not re-researched; 1:500 volumes, section 4. |
| 20 | `🌙️11/☀️22/USED-HVAC-COMPONENTS-STAFFAGE-COLLECTION` | closed 2026-11-22 17:09 | 18 of 40 members (verified); scope cut by the dev (claimed). | 1:500 volumes, section 4. |
| 21 | `🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION` | closed 2026-11-23 13:53 | 21 of 40 members (verified). | 19 rows never started; 1:500 volumes, section 4. |
| 22 | `🌙️11/☀️22/USED-PARK-OBJECTS-STAFFAGE-COLLECTION` | closed 2026-11-23 12:38 | 29 of 40 members (verified). | Rows 30 to 40 never started. |
| 23 | `🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS` (`U`) | **open** | Not met while `K` is open. | Section 5, step 8. `U/📌️important/📝️.md` is empty (0 bytes, verified); the stray `S/📦️packages/🟦️typescript/humansb-A-findings.md` still exists (verified). |

Verified catalog facts used above: 375 asset folders, 375 manifest entries, no id missing on either side, all `completion: "authored"`; humans 75, vegetation 67, objects 58, components 58, appliances 39, furniture 39, machinery 16, animals 14, vehicles 9; 23 collections (`used-hvac-components` 18, `used-household-objects` 35, `used-park-objects` 29, `reclaimed-components` 22, `used-instruments` 21); manifest `heightM`/`widthM` equal the asset `dimensions` within 1 mm for all 375 and no manifest pose or clip id is missing in its asset (manifest last written 2026-11-24 05:37, so the multiview rewrites changed no declared dimension so far).

## 3. Closed With a Remainder: Who Carries It

| Remainder | From | Carried by |
|---|---|---|
| 27 assets without a 1:500 volume, `model-catalog` | #11, #19, #20, #21 | nobody (section 4) |
| Flow in playgrounds, air controls in compare, `model-batch` to its end | #18 | nobody |
| Python consumer against real manim; `repo run @teaching/staffage:test` to its end | #10 | nobody by name; lane s1 touches `repo run` on Linux |
| MP4/WebM export re-measurement | #1 | nobody (`K` names video export among its consumers: section 5, step 6) |
| Narrated full-course rebuild | #6 | open course tickets, section 6 |
| Release build and browser journeys on 375 assets | #12, #9 | lane `linux-s1-site.md` |
| Cross-scale crown findings of 18 plants | #17 | nobody |
| Never-started rows: 18 reclaimed, 5 household, 22 HVAC, 19 instruments, 11 park, 7 people | #15, #16, #19 to #22 | dropped by the dev's order of 2026-11-22 16:29 (claimed in `U/📋️brief.md`); not a defect of the closes |
| The requests in `U/requests.md` (verified: 88 section headings, of which 8 name `unionPolygons` or a polygon union, 8 `strokesOf`, 5 the structure solver) | all waves | lane `linux-s2-requests.md` (triage, proposals only) |

## 4. The One Measurable Gap of a Closed Ticket

Measured over the 375 asset documents:

- `volumes` by scale and status: 1:50 375 authored; 1:100 375 authored; 1:200 374 authored, 1 unavailable (`used-microwave-oven`); **1:500 348 authored, 27 unavailable**.
- The 27: `used-accordion`, `used-bedside-table`, `used-brine-water-heat-pump`, `used-cast-iron-sectional-boiler`, `used-dishwasher`, `used-djembe-and-cajon`, `used-dry-rotor-inline-pump`, `used-electric-cooker`, `used-electronic-organ`, `used-harmonium`, `used-high-chair`, `used-insulated-copper-coils`, `used-microwave-oven`, `used-moving-boxes`, `used-oil-filled-radiator`, `used-panel-radiator-pallet`, `used-sewing-machine`, `used-spring-rocker`, `used-steel-string-acoustic-guitar`, `used-towel-radiator`, `used-trumpet`, `used-tumble-dryer`, `used-viola`, `used-violin`, `used-wall-hung-gas-boiler`, `used-washbasin-vanity`, `used-wc-pan-cistern`. The same list as `U/older-tickets.md` 5.5 measured on 2026-11-23: nothing changed since.
- The guard: `S/🧪️tests/🧊️model-catalog/🟦️.ts:42-51` expects `authoredScales(asset.volumes)` to equal `[50, 100, 200, 500]` for every asset; `authoredScales` (`S/🔨️modules/🔗️contracts/🟦️.ts:316`) keeps only `status === "authored"`. So the case fails at the first of the 27. Test file unchanged since 2026-11-20.
- Contrast: 56 scale entries of older assets have an unavailable 2D representation and an authored volume at the same scale (for example `hedgehog` 1:200 and 1:500, `room-thermostat`, `thermostatic-radiator-valve`); those assets satisfy the guard. The 27 are the only ones whose volume is declared unavailable.

Two ways to make the close true, both outside this lane: author the 28 missing volume lists (27 at 1:500, one at 1:200), which is what the descriptions of #11, #19, #20 and #21 ask for; or the owner of the guard decides that a volume may be unavailable where the object is below the printable feature and changes the test with that rule written into the README (a decision of the dev, since a test may not be weakened to pass). Until then `model-catalog` is red and its per-asset cases (363 of 375 never reached on 2026-11-23, `older-tickets.md` 5.5) are unmeasured.

## 5. `K`: Acceptance and the Exact Remaining Path

Acceptance, from the description, `K/📋️brief.md` and `K/📦️handoff.md`: every asset correct in plan, front and side over every pose, clip, transition, scale band, alternative, seven modes and every consumer; one evaluated state per instant; a missing combination is a named failure; **coverage matrix, proof sheets, regression tests**; handoff written.

What holds today (verified statically): the declarations (section 2, row 7); the generator `T/📊️matrix.ts` is resumable per asset, shardable (`--shard i/n`), mergeable (`--merge-only --from`), names crashes and timeouts as failures, and treats a record as stale when its asset document changed (header of the file, written 2026-11-25 04:45); `T/🔎️scan.ts` and `T/🖨️proofs.ts` exist (`🖨️proofs.ts` written 09:03 today, so someone is editing it); the suite has about 40 `it` cases and 22 scenarios; the fixture holds 13 in-band and 48 band-copy allowlist entries (61, as the handoff says).

State of the batches at 09:10 (claimed, first lines of `U/multiview-*.md`): no batch report is `FINAL`. MB-1, MB-2, MS, MR, MF and MO ended on 2026-11-24 with leftovers; MA, MH-1 to MH-6, MC, ML-1, MV-2, MV-3 are `IN WORK`; MV-1 has no report file; `multiview-memory.md` is a draft with three sections pending. MA: all 13 animals untouched, 4,477 hard findings.

The path beyond the batch work, in order:

1. **Measure the 145 assets no batch owns.** 230 assets are in a batch file; 145 are not (components 37, objects 31, vegetation 22, appliances 21, furniture 20, machinery 8, vehicles 3, humans 2, animals 1; verified against `🧰️matrix/batches/M*.tsv`). All 145 are clean in `🧰️matrix/scan-after-shared.json` (verified: 169 clean, 206 hard, 11,660 hard findings; none of the 145 hard), but the scan visits poses only. MF showed what that is worth: 20 of 20 clean in the scan, 4 of 20 with failing cells in the matrix, all in clip instants or transitions. Expect a later round for some tens of these assets. Run `T/📊️matrix.ts --assets <the 145>` first, before the full run, because it is cheap (rigid assets, 0.26 GB each) and it sizes the last fixer round.
2. **Decide the matrix questions that no fixer may decide** (each is a rule of `T` or of the engine, owned by the coordinator):
   - `steel-column` 1:100 front and `glulam-beam` 1:500 plan: neither has an allowlist entry in the fixture (verified); the handoff says `steel-column` front was refused.
   - `used-harmonium`, 21 `paper-height` cells: a tie at the tolerance decided by rounding (`requests.md`, "multiview ML-1" and "multiview MO").
   - `kneeling-gardener`, 30 `blank-shapes` cells on a symbol drawn as an unfilled outline ("multiview MH-2").
   - `storage-crate-stack`, 28 `language-geometry` cells ("multiview MB-2").
   - `rowan` and `saucer-magnolia` 1:500 front: no allowlist entry (verified), they sit in MS and MV-3.
   - The seven places where `🧊️catalog-silhouette` and the multiview gate ask for opposite things ("multiview MH-4"), the six people of MH-6 and the three walkers of MH-2 that keep hard findings inside the 1:50 guard: without a rule decision these stay `fail` whatever the fixers do.
   - `evaluate` throwing `PolygonUnionError` at single clip instants (named in `requests.md`: `pregnant-person`, `tour-guide`, `parent-with-stroller`, `sleeping-person`, `herring-gull`, `person-pushing-bicycle`; whether each still throws was not measured): in the matrix such an instant is a failing cell or an `asset-crashed` record. This is the one engine repair the clean matrix depends on; lane s2 triages it.
3. **Merge and sync**: read the new `🧰️matrix/<BATCH>/allowlist.json` entries, `python3 🧰️matrix/merge-allowlists.py --write`, `🧰️matrix/allowlist-check.ts`, then `🧰️silhouette/sync-manifest.py --write` and `manifest-check.ts`. (Manifest is in step with the assets right now, verified.)
4. **Freeze the catalog, then verify everything again**, because 181 documents changed since the last greens: check CLI over all categories (`check-categories.sh`), `🧊️catalog-silhouette` (about 3.2 h), `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power`, `🧬️schema-conformance`, `🧍️plan-projection`, `✂️section-views`, `📑️multiview-sheet`, `🧊️chain-state` (2 red cases during the repairs, claimed), `🧊️model-bounds` (3 heater cases, MF), and `🔭️multiview-coverage` itself. A red asset goes back to its batch owner; only then step 5, because every asset rewrite makes its matrix record stale.
5. **Full matrix on the frozen catalog**: `bun T/📊️matrix.ts --language en --out <run> --jobs <n>` through `repo capacity run`, sharded by `--shard i/n` over machines and merged with `--merge-only --from`. Universe 94,038 cells on 2026-11-24 (claimed). Acceptance line: `summary.json` with 0 `fail`, which includes 0 `asset-crashed`, 0 `asset-timeout`, 0 `cell-missing`. `saguaro-cactus` did not end in 25 minutes on 2026-11-24; the default limit is 7,200 s per asset. This Linux worker could take shards (12 cores) only for rigid categories: 7 GB free RAM, 4 GB free disk, a tree process holds 1.2 GB.
6. **Proof sheets and example exports**: `bun T/🖨️proofs.ts --out <dir>` (contact sheet per category, print sheet per scale and category), opened and read; one example per export format the description names (SVG, PDF, DXF, DWG, frames, MP4/WebM), which also settles the video remainder of #1: run `🎞️frame-timing` and `🗜️zip-reopen`.
7. **Consumers**: the consumer findings in `K/📦️handoff.md` ("Consumer audits") are all still marked open and were static readings of 2026-11-02. Two spot checks of today: `Frieze` in `🏠️overview/🟦️.tsx` no longer hard-codes `viewpoint="elevation"` (no match); `🔬️detail/🧬️model/🟦️.tsx:160` still reads `joint.momentNm` beside `mxNm`/`myNm` (may be a legitimate joint field now). The list needs one pass against the tree, each item marked done or fixed, plus the browser pass lane s1 is doing (compare, playground, German, phone width were never looked at).
8. **Write `K/📦️handoff.md`** as a result (it is headed "Status: SKELETON", its matrix table is all TODO): matrix summary per category, sampled against exhaustive, proof sheet paths, suite lines with command and exit code. W1: 169 ticket files still name `C:/git/semio` (verified by grep; the handoff counts 162 scripts); they are records of single edits and cannot be re-run, so state that in the close summary instead of rewriting them.
9. **Close `K`** (`ticket_close`, needs the repo MCP, which did not connect here), then `U`: delete the Mac's `U/🗑️generated`, move or delete `S/📦️packages/🟦️typescript/humansb-A-findings.md`, fill `U/📌️important/📝️.md`, decide section 4 and the orphan folder of row 9, close `U`.

## 6. Neighbouring Open Tickets That Consume Staffage (not staffage tickets)

Listed so that nobody counts them as finished by the umbrella; none was audited.

- `🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS`: open since 2026-11-03, 37 files name staffage. A second folder with a damaged month name (`🌩…11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS`, one file `parallel-ahead.md`, no json) lies beside `🌙️11`.
- `🌙️11/☀️11/DECLARATIVE-TEACHING-MORPHS-AND-FAST-PREVIEWS`: open; its description names "canonical staffage LoD" (`🧊️cast-lod` exists in `S/🧪️tests`).
- `🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS`: last interaction is a reopen of 2026-11-14 15:35 (F27), status open.
- `🌙️11/☀️12/CREATE-HEATING-AND-COOLING-FINGER-BUNG-ASSIGNMENT`: closed; its close of 2026-11-24 05:13 says "the cover illustration with staffage runs in a separate session", for which no ticket was found.
- `🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS`: open (UI frame, touches the staffage React target).

## 7. Housekeeping Found on the Way

- Ticket folders that still hold tool output (CLAUDE.md asks for its removal at close): #11 41 log/txt files (227 MB folder), #1 35, #6 18 and two `🗑️generated*` folders, #16 9, `U` 19.
- All 22 staffage tickets have an empty `📌️important` (0 bytes) or none.
- Closes without a file list (`files: 0` in the json): #1, #10, #11, #13, #15, #18 (second close), #19, #21, #22; the umbrella's handoff notes that `ticket_close` wants a file outside the tickets folder.

## 8. What This Audit Did Not Do

No test, scan, matrix, check CLI, build or browser run; no reading of the Mac's `🗑️generated`; no reading of the 4,783 files of #11 beyond its acceptance file and the finisher report; the closes of #2 to #5 (October UI tickets) were taken from their summaries; suite counts of 2026-11-24 are quoted, not reproduced.
