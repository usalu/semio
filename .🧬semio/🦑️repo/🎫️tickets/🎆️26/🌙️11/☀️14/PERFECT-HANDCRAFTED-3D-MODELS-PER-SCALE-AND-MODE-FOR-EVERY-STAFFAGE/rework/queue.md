# Rework coordination (2026-11-22)

Concurrent subagent limit is 20. Authors 01–19 launched 00:20 CET with `T/rework-brief.md`; batches over ~5 assets proved too large (author 16 did a shallow thermal-only pass in 18 min).

## Lessons

- Give an author at most four assets and say that four is the whole budget.
- Name the previous shallow result explicitly so it is not repeated.

## Queue (launch when a slot frees)

- 16b: appliances/vacuum-cleaner, server-rack, tower-server, electric-fan-heater (full rebuild; previous pass thermal-only; rack `--volumes` never finished)
- 16c: appliances/gas-hob, kitchen-extractor-hood, pendant-lamp, shower-head (three left untouched by 16)
- Any author returning a partial batch: re-split its remaining assets in fours.
- After each author: independent reviewer with `T/review-brief.md` → `T/review/<NN>.md`.

## Running

- 16a: appliances/toaster, electric-kettle, laptop, desktop-monitor
- 01–15, 17–19: original batches

## Other threads

- Site dev server `repo run @teaching/staffage-site:dev` (port 6071) to check the real 3D viewer; native Vite start is refused by the cache guard.
- Disk low (~11 GiB): `🗑️generated/volumes-tools/cache` (1.5 GB) could not be removed by the agent.

## 00:50 CET — one asset per agent (workflow `staffage-handcraft`, run wf_3ad81b64-3ea)

Batch authors run out of budget after ~75 tool calls (07: three quadrupeds patched, four not started; 16: thermal-only). A workflow now gives every asset its own author (up to four passes until it reports complete), then an independent reviewer who fixes, then up to two repair rounds. 148 assets (all but 16a's four); untouched batch tails first, batch heads last. Per-asset reports: `rework/assets/<category>--<slug>.md`, reviews: `review/<category>--<slug>.md`. Queue entries 16b/16c above are covered by the workflow.

Fundamental issues reported so far: README loft `rx`/`ry` description swapped for limb rotation `[0.5,0.5,0.5,0.5]` (07); `r3-project2d.ts` outlines at 1:500 can fail `volumes-silhouette` (07).

## Batch 19 back (partial): six components patched at 1:50, coarse scales still carry the 1:50 parts list; sliding door "closed" leaf stands mid-opening (concept wrong). All nine are in the workflow.

Fundamental (19): `volumes-feature` lets under-feature solids through at coarse scales, so deleted-list coarse models stay VALID → needs a root fix with fixture and test (dedicated agent once a slot is free). ETICS walls now 530 / 710 mm: `model-bounds` test that pins the layer spans not yet rerun.

## 01:20 CET — real viewer

`repo run @teaching/staffage-site:dev` sat >30 min in "Reading the project graph slice" (expected 3600 s, 4.4 GB) → stopped. Native start works: `.claude/launch.json` config `staffage-site-native` (Vite with `SEMIO_BUILD_MODE=prototype`, `SEMIO_VITE_CACHE_DIRECTORY=<repo>/.cache/b/p/m/staffage-preview-vite`, `--mode prototype`, port 6072). Asset route: `http://localhost:6072/#/<category>/<slug>`.

Fundamental (site, to fix with a dedicated agent):
- one asset page (`#/animals/domestic-cat`) requests a dozen-plus other asset modules (kick-scooter child, cow, bar stool, bicycle stand, cargo bike, courier, bookshelf, chalkboard, goat, hedgehog …) and the dev plugin `🎯️targets/⚛️react/🌐️vite/🟦️.ts` runs `validateAsset` (~20 s each) synchronously per module, so the page stays on "Loading…" for minutes;
- while loading, an authored asset shows the notice "Planned, not yet authored".
- `validateAsset` itself takes ~22 s for a 1.4 MB human asset.

## Batches 14 and 08 back (partial); agents launched 01:35 CET

- 14 furniture: six structure/floating fixes; six unchanged; open: bed head bones point away from solids, operating table not articulated, sofa centre foot, bench third support.
- 08 animals: labrador sitting pose and pigeon feather-tract wing rebuilt; cat, squirrel, hedgehog, gull only reviewed; mallard not started (folded wing is one rigid plate, `wing-stretch` lifts the plate); pigeon 2D in opened poses still shows the old stick wing.
- Validator agent running: `volumes-connected` + tightened coarse `volumes-feature` → `T/validator-connectivity.md`.
- Site agent running: asset page loading → `T/site-loading.md`.

Fundamental (08), not yet assigned: `volumes-feature` forces non-extrude sheets (feathers) to 7.5 mm thickness; `r3-sheet.ts` has no `--file`, so rig/pose candidates must be written to the live catalog to be seen.

## Batch 17 back (partial), 02:10 CET

- Internals/thermal on boiler, cylinder, pump, TRV, thermostat; MVHR half done (front porthole with sealed supply impeller and air paths wrong → needs layout decision + rig/2D/thermal/clips redesign together); six only reviewed.
- Root changes not yet test-run: new hidden thermal regions (cylinder, pump, TRV, thermostat), two new materials, changed `areaM2` (boiler, MVHR) → run `🧪️tests/🌡️catalog-physics`, `⚡️asset-power`, `model-bounds` in the final test sweep.
- Fundamental (17), to hand to the validator agent: thermal-area rule counts inner faces of hollow shells, forcing solid lumps on pump flanges and the TRV sensor cap.

## Batch 11 back, 02:35 CET (best batch so far)

- Rebuilt + VALID `--volumes`: heat pump (fan bone, refrigerant circuit, four scales), scroll compressor (crank/orbiter bones, real scroll set; 1:500 keeps 1:50 wall sizes), expansion valve (catalog was INVALID before), plate heat exchanger (real plate stack). Four judged sound from one 1:50 sheet each, not signed off.
- Fundamental (11, five in `rework/11.md`): `r3-sheet.ts --file` (already with validator agent); very small assets fail the silhouette rule on raster and pen width; feature rule cannot express thin vessel walls (same family as the feather-sheet finding).

## Batch 06 back (partial), 03:20 CET

- Worker, courier: shoulder line at four scales + thermal shells + 2D front, hip bones and scapulae as plates; cyclist: hip bones/scapulae only; cargo bike: floating saddle and pedals fixed at coarse scales; city bicycle measured correct, unchanged. Rigs unchanged.
- Tightened `volumes-feature` is live: worker 47, cargo bike 72, city bicycle 14 findings → INVALID on coarse lists (thinned 1:50 lists). Expected; workflow authors fix by modelling per scale.
- Fundamental (06): the rig's rest pose is planar (sagittal), so clavicle / shoulder-girdle bones that run laterally cannot be authored. Wait for author 01's standing-adult result; if confirmed, needs a schema/rig extension (rest-pose lateral direction) with fixtures and tests before the human rigs can be right.

## Batch 04 back, 03:45 CET

- `humans/elderly-with-cane` fully rebuilt at four scales (rig, body, thermal, structure, 2D), VALID `--volumes`. Crutches, rollator, white cane, wheelchair not started (in workflow).
- Rig limitation confirmed by 04 and 06 (arm cannot hang on a lateral clavicle) → rig agent launched: full 3D rest placement per bone, authoring note `T/rig-lateral-rest.md`. When it lands, human/animal authors and reviewers must add shoulder girdle / pelvis halves and re-parent limbs (follow-up wave).
- Ruling sent to validator agent: `slender` long members keep true section at every scale; tightened feature rule targets compact under-feature solids; must agree with `🧪️tests/🧊️model-skeleton`; no more back-and-forth rule changes, announce in `T/validator.md`.

## 01:50 CET — Fable limit hit (resets 03:00 Europe/Berlin)

All Fable subagents stopped with HTTP 429: authors 01, 02, 03, 05, 09, 10, 12, 13, 15, 16a, 18, validator, site, rig. Workflow wf_3ad81b64-3ea stopped by me so it does not run through items with dead agents (resume with `resumeFromRunId` after 03:00). The dev asked for Fable subagents, so no silent switch to another model: resume each agent by SendMessage at 03:02 (they keep context), then resume the workflow.

Catalog state at 01:50: all 152 parse; 13 sources differ from the catalog (in-progress edits): japanese-maple, diesel-generator, excavator, forklift, oil-pumpjack, scissor-lift, runner-with-blade-prosthesis, seated-office-worker, skateboarder, sleeping-person, standing-adult, spiral-stair, stair-flight. Full published-state sweep running (`catalog-sweep.sh` → `🗑️generated/sweep/`).

## 01:55 CET — new orchestrator session takes over

The previous session is gone (its agents and workflow wf_3ad81b64-3ea cannot be resumed from here). Fable subagents answer again. Relaunched fresh, all on Fable 5.2 medium:

- Root-fix agents (continue the half-done work found on disk): rig lateral rest → `T/rig-lateral-rest.md`; validator rules and tools → `T/validator.md`; site asset-page loading → `T/site-loading.md`.
- Workflow `staffage-handcraft-objects`: one author (up to three passes) then an independent reviewer (up to two repair rounds) per non-living asset, 8 at a time. Reports `rework/assets/`, reviews `review/`.
- Humans and animals follow in a second workflow as soon as the rig agent reports (they need shoulder girdle / pelvis halves on the lateral rest placement).

## 12:10 CET — resumed after the second Fable limit (hit ~03:25, 19–25 agents exhaust it in about 90 minutes)

State at the stop: 34 of 152 assets with an author report, 21 author passes returned (all "partial"), no review, none accepted. Rig lateral rest landed (`T/rig-lateral-rest.md`); its consumer verification, the validator fixes and the site fix were cut off mid-work.

Resumed: validator agent (with the authors' overnight rule findings), site agent, rig-verification agent, and both pipelines from their journals (`wf_b3cf0787-c1e` non-living, `wf_17306803-0d6` living; finished passes come from cache, dead ones rerun). A third pipeline could not be launched (launch hook timeout); six direct tail authors died with the limit and are covered by the pipeline.

## 14:50 CET — validator agent done; third (weekly) limit at ~13:50, pipelines resumed 14:22

Validator: all ten original items, the authors' overnight findings and the type errors closed; rule suites green; `model-skeleton` settled towards the feature rule. Open: no `--volumes` sweep over the catalog yet; `model-skeleton` 22/27 and `three-views` 314/328 are asset work (lists in `T/validator.md`). Morning sweep: 92 VALID / 60 INVALID (57 through `volumes-feature`, i.e. coarse scales that are thinned 1:50 lists). Rig verification and site loading done. Airflow preview handled in its own session/ticket (26/11/22/STAFFAGE-THERMODYNAMIC-FLOW-VIEW-INSTEAD-OF-AIRFLOW-CONES).

## 23:00 CET — umbrella finisher (ticket 26/11/22 GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS)

All sessions were cut off at 17:27. Finished and proven (`--catalog-only --volumes` `VALID`, check CLI `✓`, sheets read): domestic-cat, person-on-crutches, drinking-fountain, small-leaved-lime (candidate fixed and published), parent-carrying-toddler (candidate published, `walk-6` arm 1°), multi-stem-birch, bookshelf, japanese-maple (source of 01:34 published), city-car (coarse brake discs). Not published: diesel-generator source (three measured findings). Catalog sweep of the 152 originals: 130 VALID, 22 INVALID (21 through `volumes-feature` / `lateral-ambiguous`, agave through print). No pipeline is running; nothing was started that had not been started. State, queue and what must be run before closing: `perfect-3d.md` in the umbrella ticket. `🗑️generated` is emptied except `r3-sheet/file`, `r3-2d/file` and `volumes-tools/cache`.
