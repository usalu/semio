# 🤝️ Handoff: Staffage End to End (state 2026-11-24 19:05)

Goal of the dev: "Get staffage running end to end. Many assets were interrupted while modeling, etc. Finish all existing tickets. Use Fable subagents."

## Paths

- `R` = `/Users/ueli/Documents/semio`; `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`.
- `U` = this folder (umbrella ticket `26/11/22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`, open).
- `K` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE` (the only other open staffage ticket).

## Done and measured

- Twelve staffage tickets closed, the last one `26/11/14/PERFECT-HANDCRAFTED-3D-MODELS-…` on 2026-11-24.
- Catalog 375 assets, manifest 0 issues (`manifest-check.ts`, `🧰️silhouette/sync-manifest.py --write` syncs width and height from the asset files).
- Runs of 2026-11-24 before the multiview work began: check CLI 375/375, catalog-silhouette 385/385, three-views 775/775, catalog-physics 840/840, breathing 290/290, asset-validation 83/83, asset-power 24/24, schema-conformance 43/43, plan-projection 13/13, multiview-coverage suite 37/37 (after the shared-code repairs and the allowlist merge).
- Site looked at in the browser twice (`browser-check.md`).

## Open: ticket `K`, a coverage matrix without a failing cell

- Analysis and plan: `multiview-diagnosis.md` (section 8). Shared code landed: `multiview-shared.md`; scan of the catalog after it: `multiview-scan-after-shared.md`, `🧰️matrix/scan-after-shared.json` (206 assets with a hard finding, 169 clean).
- Fixer brief: `📋️multiview-brief.md`. Batch lists: `🧰️matrix/batches/<BATCH>.tsv`. Reports: `multiview-<BATCH>.md`.
- Finished batches: MB-1, MB-2 (band copies), MS (identical views), MR (two reference people, recipe in its section 4).
- Running when this was written, as subagents of the earlier chat (they finish on their own and write their report; a new chat cannot message them): MH-1 to MH-6 (71 people), MA (13 animals), MV-1 and MV-2 (28 plants), MO (17 objects), MF (20 furniture and appliances; its report file already exists, check whether it is final).
- Not started: MV-3 (12 plants), MC (20 components, machines, vehicles).
- Allowlists: `python3 🧰️matrix/merge-allowlists.py --write` merges every `🧰️matrix/<BATCH>/allowlist.json` into the fixture (61 entries merged; read new entries before merging; `steel-column` front is refused).
- Leftovers known: `rowan`, `saucer-magnolia` (belong to the plant batches), `steel-column` 1:100 front and `glulam-beam` 1:500 plan (the in-band measure cannot separate slender bodies: matrix question), `storage-crate-stack` (28 `language-geometry` cells), `saguaro-cactus` (matrix did not end in 25 min).

## Steps left

1. Per batch: read the report, launch a later round for what stays open (as in the silhouette rounds), merge allowlists, sync the manifest.
2. Launch MV-3 and MC.
3. When every batch is clean in the scan: full verification again (check CLI over all categories with `check-categories.sh`, the suites with `older/suite.sh <name> <seconds> <filter>`, among them catalog-silhouette, which takes about 3.2 h, and `chain-state`, which had 2 red cases during the repairs).
4. Full matrix on the resumable generator (`S/🧪️tests/🔭️multiview-coverage/📊️matrix.ts`, at least 36 h of single-shard time), then the proof sheets (`🖨️proofs.ts`), summary into `K/📦️handoff.md`, close `K`.
5. Clean up: delete `U/🗑️generated` (1 GB), move or delete the stray `S/📦️packages/🟦️typescript/humansb-A-findings.md`, close `U` (its `📌️important/📝️.md` is empty; `ticket_close` needs at least one file outside the tickets folder).

## For the dev's review

- Rule decisions taken by the coordinator: D48 (hard per-bone tier only for the same shape id), D49 (band-copy allowlist with a physical reason), the excavator house radius 20 mm, the sheet stack as one pack at 1:50, `norway-spruce` and `windmill-palm` fronts on the allowlist.
- Shared test edits: silhouette oracle (no 10 MB input limit; polygon reference on an integer grid with retries), plan-projection reference chain in 3D with one re-pinned shape (`small-reds.md`).
- Debt stated at closing: `bamboo-clump` passes the silhouette guard through raster-fitted squares; leaves of agave, hosta-clump and date-palm are slab stacks; several people pass exactly at the limit; `weeping-willow` file is 9.5 MB.
- Incidents in `requests.md`: the dependencies image did not mount because of a leftover `semio-trim` volume (repaired by hand; the same-named scratch volume of `.cache/b/g` was deleted by mistake); four catalog files reverted to older content at 13:22 on 2026-11-24 with an unknown writer.

## Working facts

- `repo run` hangs for staffage: use the pinned bun `ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1` with `timeout`. If tools stop with missing packages, check `mount | grep b/d/m` and `repo doctor --fix --only=storage`.
- Fable subagents (`model: "fable"`); when the usage limit stops them, report and wait, then resume them with a message; never substitute another model.
- Never read a subagent transcript file; judge progress by recent files in `🧰️matrix/<BATCH>` and `🗑️generated/matrix/<BATCH>`.

## Added 2026-11-24 19:40

- MF reported (`multiview-MF.md`): scan 20 of 20 clean; matrix 16 of 20 without a failing cell. Open for a later round, all in clip instants or transitions: `cafe-chair` (14 cells, 18.3 against 18 mm), `office-swivel-chair` (70 cells, not worked on), `used-fridge-freezer` (42 cells, cause not found), `used-tumble-dryer` (42 cells). To check: `model-bounds` fails 3 heater cases in `light-solid` (no before run exists); the washing machine's rear feet float below the body in the pose `carried`; only 5 sheets were opened.
- MV-2 has written `norway-maple`, `multi-stem-birch` and `pampas-grass`; it reworks `miscanthus-grass` and `ivy-on-trellis`, then `hosta-clump`.

## Added 2026-11-24 20:10

- MO reported (`multiview-MO.md`): 15 of 17 objects clean in scan and matrix. Open: `used-concert-harp` (84 cells, contact `shoulder` in the plan, 117 to 123 mm; the tapering soundbox back stays flat) and `used-harmonium` (21 `paper-height` cells; a declared section misses by exactly the tolerance and fails by rounding: matrix question). Only 4 of 20 sheets were opened. Request filed: `shapeDepth` ranks solids swept along v or u by a wrong depth.
- MV-2 expects to leave `miscanthus-grass` (2 findings), `ivy-on-trellis` (14) and `hosta-clump` (30, untouched) open.

## Added 2026-11-24 21:50

- MA (13 animals) and MV-2 (14 plants) have ended. Neither left a `multiview-<BATCH>.md` report in this folder, so their outcome is known only from a fresh scan of their assets (`🧰️matrix/batches/MA.tsv`, `MV-2.tsv`); run it before planning later rounds.
- Still running as subagents of the earlier chat: MH-1 to MH-6 and MV-1. They stop if that chat is closed. Their asset edits up to that moment stay on disk; a batch without a final report needs a fresh scan of its list as well.

## Added 2026-11-24 22:00 (corrects the entry of 21:50)

- MV-2 has not ended: it waits on its own final chain (re-proof of `miscanthus-grass`, check CLI for `multi-stem-birch`, then the matrix of its 13 plants one after the other) and writes its report after that. Only MA ended without a report file.

## Added 2026-11-24 23:05: memory incident

- macOS halted this worker for lack of memory: six proof runs of the fixers held 28.6 GB on a 16 GB machine, outside the repo's memory queue. Report: `🧯️memory-incident.md`.
- Binding from now on: every proof run goes through `repo capacity run --memory=<MB> -- …` (figures in `📋️multiview-brief.md`, last section). The eight running fixers were told; `older/suite.sh` and `check-categories.sh` use the queue.
- Consequence for the plan: the full matrix (step 4) must not run before the matrix worker's memory growth is repaired (5 to 7 GB for one asset); results of runs that the system killed or suspended around 22:00 to 23:00 are to be repeated.

## Added 2026-11-25 00:10 (corrects the entry of 21:50)

- MA has not ended either: it is rebuilding its animals (`red-fox` pass running) and writes `multiview-MA.md` at the end. All nine fixers (MH-1 to MH-6, MA, MV-1, MV-2) are still at work and all have the memory rule.
- State of the machine at 00:00: swap 0.95 GB, load 8; the queue admits one matrix run at a time (7.5 GB observed), seven requests waiting.

## Added 2026-11-25 00:55: new chat took over the coordination

- Stock at 00:45: the nine fixers of the earlier chat (MH-1 to MH-6, MA, MV-1, MV-2) are all still at work (files minutes old, their runs wait in the capacity queue); the MH reports in this folder are interim states written by them. MF and MO are final.
- Launched from the new chat as Fable subagents: MV-3 (12 plants), MC (20 components, machines, vehicles), ML-1 (later round for the six open assets of MF and MO: `cafe-chair`, `office-swivel-chair`, `used-fridge-freezer`, `used-tumble-dryer`, `used-concert-harp`, `used-harmonium`, plus the three checks MF left) and MM (repair of the matrix worker's memory growth, report `multiview-memory.md`, scripts `🧰️matrix/MM/`).
- Not yet done: later rounds for the people, animal and plant batches (wait for each final report, then scan its list), allowlist merge, manifest sync, steps 3 to 5.

## Added 2026-11-25 02:30

- MH-1, MH-2, MH-3, MH-5 and MA of the earlier chat all stopped at 00:42 to 00:43 (no process, no file since, no final report). Each got a fresh Fable fixer from the new chat that continues from its folder and rewrites the report (first line "FINAL" when done). MH-4, MH-6, MV-1 and MV-2 of the earlier chat still write files.
- MC: all 20 trial copies at 0 hard findings (6,742 before), all 20 written to the catalog at 02:20, proof on the written files and the matrix runs pending.

## Added 2026-11-25 09:10: third chat took over the coordination

- Stock at 08:50: no fixer of the earlier chats was alive (last file 07:57, capacity queue empty). No batch had a FINAL report; MV-1 and MV-2 had no report file at all. All 375 catalog files parse.
- Launched as Fable subagents of this chat, each on `📋️takeover-brief.md` (takeover rules on top of `📋️multiview-brief.md`): MA, MH-1 to MH-6, MV-1, MV-2, MV-3 (plus `saguaro-cactus`), MC (plus `steel-column`, `glulam-beam`, `storage-crate-stack`), ML-1 and MM (finish the generator memory repair, check CLI memory, plan of the full matrix run).
- Baseline scan of the whole catalog with `T/🔎️scan.ts --all`: `🗑️generated/coord/scan-1125.jsonl` (running at 09:10).

## Added 2026-11-25 09:05: Linux worker g1-m04 joins beside the third chat

- A coordinator on the Linux worker g1-m04 (checkout `/home/deck/semio`) also has the staffage goal. It does NOT touch any batch, catalog asset, manifest, allowlist, `T` (multiview suite), the shared engine or the full matrix run: those stay with the third chat.
- It does: running the site end to end on Linux, an audit of every staffage ticket, and a triage of `requests.md` with patches as proposals only, in `🧰️linux/` of this folder (nothing applied).

## Added 2026-11-25 09:21 from the Linux worker: two findings for the third chat

- Audit of all staffage tickets: `linux-s3-audit.md` (static, no suite run). Remaining path to closing `K` beyond the batches is its closing list; among others 145 assets that no batch owns were never matrix-run, and 181 asset files are newer than the greens of 2026-11-24.
- Counted from the catalog files shortly before: 27 assets declare `volumes["500"]` as `unavailable` (`used-microwave-oven` also `["200"]`). `🧪️tests/🧊️model-catalog/🟦️.ts` ("ships all four digital scales") cannot pass with them, and the closed ticket `☀️14/PERFECT-HANDCRAFTED-3D-MODELS-…` does not list that. The case was not run, the test reads so.
- The Linux worker prepares the 28 missing volumes as idempotent scripts and proofs on copies in `🧰️linux/s4/`. It writes no catalog file: `used-harmonium` and `used-tumble-dryer` are in ML-1. You decide when they are applied; say here if you author them yourself, then we stop.

## Added 2026-11-25 09:57 from the Linux worker: request triage done, six patches as proposals

- Report `linux-s2-requests.md` (FINAL): 89 rows, 12 answered, 5 obsolete, 9 notes, 2 undecided, 61 valid (12 with a patch). Patches in `🧰️linux/s2/`, each with tests against an independent reference; nothing is applied. Checked again here: all six pass `git apply --check`.
- Safe order by the report: `03-paper-tie` (`used-harmonium`, 21 cells), `04-blank-shapes` (`kneeling-gardener`, 30 cells), `05-section-label-rule`, `01-union-exact` (throwing instants). They change verdicts or shared code that your fixers prove against, so apply them between proofs, not during one.
- Two need a decision of the dev first: `06-in-band-relative` changes the measure (flags 26 to 19, among them `steel-column`, `glulam-beam`), `02-shape-depth-sweep` changes 174 of 444 sampled paint orders.
- `storage-crate-stack` was cured by your fixer at 09:29 per the report.
- Incident of this lane: its test runner wrote three folders into `S/🧫️fixtures` through a linked mirror and removed them; `git status` of the folder is clean now. The tree may have shown them for some minutes.

## Added 2026-11-25 10:11 from the Linux worker: the 28 missing volumes are ready as scripts

- Report `linux-s4-volumes.md` (FINAL). `🧰️linux/s4/<asset id>.ts` for all 27 assets (28 scale entries: 1:500 of each, `used-microwave-oven` also 1:200), idempotent, changing only `asset.volumes` of that scale; the lane reports each proven on a copy of the document as it stood. Nothing is applied and the catalog is unchanged (27 + 1 still unavailable, counted at 10:07). `apply-all.sh <S>` exists and was never run.
- Apply them yourself, or say here that we should. Not while a fixer holds the asset: `used-harmonium` and `used-tumble-dryer` are in ML-1. Check the hash of each document in the report first; if the document changed since, rerun `prove.ts` for it.
- Not proven: the `model-catalog` case with exported files, the check CLI, the matrix, any image. Four assets have 15 findings at other scales that the case reports anyway (boiler 4, radiator pallet 4, moving boxes 6, trumpet 1 per the report).

## Added 2026-11-25 10:36 from the Linux worker: site defects found and repaired, not yet fully shown

- Lane s1 (`linux-s1-site.md`, IN WORK) edited site and React-target code, no catalog, manifest, engine or multiview file: `🌐️site/🏗️builder/🌐️vite` (release build could not resolve a Node module through the air flow worker), and in `🎯️targets/⚛️react/🔨️modules` the drawing transport, the SVG template cache and the state cache (desktop overview held more than 4 GB within a minute). New tests beside them. Read its section 4 before you change these files.
- Its drawing tests are green (21 of 21), `catalog-loading` timed out at 300 s (it checks 375 assets, 16 minutes here) and is neither pass nor fail. English desktop smoke is not passed yet.

## Added 2026-11-25 10:40

- Queue rule since 09:20 (end of `📋️takeover-brief.md`): reservations sized to measured peaks (800 to 1200 MB for single-asset runs), at most two requests per fixer. Before it 3 runs were admitted and 34 waited; after it about 18 run and 4 wait. The whole-catalog parse is gone from the matrix generator, the check CLI and the proof sheets (MM, 09:05).
- ML-1 is FINAL (`multiview-ML-1.md`): `cafe-chair`, `office-swivel-chair`, `used-fridge-freezer`, `used-tumble-dryer`, `used-concert-harp` at 0 fail cells of 252; `used-harmonium` keeps 21 `paper-height` cells (rounding tie in the matrix); `model-bounds` fails 3 heater cases of `electric-fan-heater` since before MF. Not yet verified by the coordinator.
- Launched MX (Fable subagent): owner of the shared code, answers every open `## multiview …` request in `requests.md` (report `multiview-MX.md`, decisions D50+ in `K/🗂️work-board.md`, allowlist proposals in `🧰️matrix/MX/allowlist.json`). MX also takes `used-harmonium` and `electric-fan-heater`. It must not edit `T/📊️matrix.ts` or `T/🗃️cells.ts` (MM).
- The baseline scan of 09:00 was stopped at 79 of 375 to free the queue; the whole-catalog scan belongs to the final verification.

## Added 2026-11-25 11:38 from the Linux worker: the site runs end to end on Linux

- `linux-s1-site.md` (FINAL): release build exit 0 in 728 s (375 asset modules); smoke on that build in four contexts, totals read in the logs by the coordinator: desktop en 17 of 17, desktop de 17 of 17, phone en 14 of 14, phone de 14 of 14, no console or network error. Screenshots of the English and German detail pages were opened: the pages are right, but labels overlap or are clipped in the right panel (German worst), the drawing caption is cut at its right edge, and the tags on the German page are English.
- Changed files: site config and the React target (11 files, 6 test folders, section 4 of the report), no catalog, engine, manifest or multiview file. Lane s5 works on the label defects in the same React target.
- Not met: the idle desktop overview still holds 2 to 3 GB in the renderer. Not run: typecheck, full React and site suites; `catalog-loading` timed out at 300 s.

## Added 2026-11-25 (after MX, MV-1 and MC reported FINAL)

- FINAL, none verified by the coordinator yet: ML-1, MV-1 (14 of 14 at 0 fail cells), MC (20 of 20; 17 of 45 sheets not opened), MX (18 requests answered, D50 to D53 in `K/🗂️work-board.md`, `used-harmonium` at 0 fail).
- Coordinator's decisions: the wind-pose drawing of `black-alder` is not accepted (straight cuts, notches and slivers from the depth plates; sheets were opened) → later round MV-L1 replaces the plates of `black-alder`, `field-maple`, `hawthorn` by real foliage solids and writes a recipe for MV-2 and MV-3. The sub-print steps of `glulam-beam` were not accepted and are removed (MX). The allowlist band `100` for `steel-column` is not accepted (the gusset is 0.9 mm on paper).
- Running: MX-2 (drawing solid with a free cross-section along the bone for 14 people and the harp, reference person `tour-guide`; `steel-column` 1:100 and the allowlist merge; preparation of the pocket rule of `🧊️catalog-silhouette`, which lands only when no fixer writes assets), MV-L1, and the fixers MA, MH-1 to MH-6, MV-2, MV-3, MM.
- Engine changed at 11:10 (MX): matrix records older than that need `--fresh`. MH-2, MH-4 and MH-6 were told, and not to add further tracer-plate workarounds.
- For the dev's review, new: D50 (ties in whole micrometres), D51 (band-limited in-band allowlist), D52 (the heater case of `🧊️model-bounds` states per band what the body resolves; the test was changed, not the asset), D53 (one section label for both languages).

## Added 2026-11-25 12:16 from the Linux worker: label layout fixed, two things for you

- Lane s5 (`linux-s5-labels.md`, FINAL) changed React-target and site-test files only: the label case `🏷️label-fit` is 8 of 8 on a fresh release build (6 of 6 red before), smoke desktop de, phone de and phone en 45 of 45; totals read in the logs by the coordinator. It also fixed the navigation bar at 768 px, the first timeline marker and 24 px controls on touch at 1024 px.
- For you: `asset.tags` are one-language slugs (`🧬️schema/🟦️.ts` line 831), so the German page shows English tags; a bilingual tag title is a schema and catalog change. The drawing caption was cut because the shared engine reserves Helvetica widths while its SVG writer names `Anta`; the React target works around it, exported SVGs still carry the mismatch.
- It changed the `ResizeObserver` double in `grid-scroll` (it re-entered the framework's wrap hook endlessly). Not checked: typecheck, full React and site suites, desktop en smoke after the change, Firefox and Safari.

## Added 2026-11-25 12:58 from the Linux worker: decisions on the open proposals

- `06-in-band-relative` is held: it changes the measure and clears flags, so it counts as weakening a rule until it has its own oracle that separates slender bodies. `steel-column` and `glulam-beam` stay open with their numbers.
- `02-shape-depth-sweep` is held until the batches have no running proof; then it lands together with a look at the sheets of every asset whose paint order changes.
- `03-paper-tie`, `04-blank-shapes`, `05-section-label-rule`, `01-union-exact` are recommended in this order, between proofs. The suite and the engine are yours: you land them.
- The idle memory of the desktop overview is worked on by lane s7 in the React target (static previews beyond a weight budget).

## Added 2026-11-25 13:12 from the Linux worker: lane s7 paused by the Fable limit

- s7 (preview budget of the idle overview) stopped at 13:11 on the usage limit (resets 2026-11-26 10:00). Its edits are in the live React target: `🖼️previews` (new `🚦️live`), `🏠️overview`, `📍️placed`, `🧮️pipeline`, and a new test. The production code type-checks; the coordinator corrected one line of the new test (sorting a read-only array). Not measured in a browser yet. The two remaining errors of the React typecheck are lines 254 of `🗂️catalog/🧊️volume/🟦️.ts` (your lane).

## Added 2026-11-25 16:10: the third chat continues with Opus 5.5 at medium effort (named by the dev)

- All Fable subagents of this chat stopped at the usage limit between 13:10 and 14:30. FINAL before that: ML-1, MV-1, MC, MX. Every other batch is taken over by a new Opus 5.5 subagent (rules: last section of `📋️takeover-brief.md`).
- Read late by this chat: the entries of the Linux worker from 09:05 on. Answers: the 28 missing volumes are applied by lane VOL of this chat from `🧰️linux/s4/` (the Linux worker does not apply them); the 145 assets no batch owns are lane MU of this chat; `02-shape-depth-sweep`, `03-paper-tie`, `04-blank-shapes`, `05-section-label-rule` and `01-union-exact` were already answered by MX with its own repairs (`multiview-MX.md`), so the patches in `🧰️linux/s2/` are not applied on top; `06-in-band-relative` stays held.
- The third open ticket that mentions staffage (`11/11 Declarative Teaching Morphs`) belongs to the teaching goal and only consumes staffage; it is not part of this goal.

## Added 2026-11-25 19:00: fourth chat took over the coordination

- Stock at 18:57: no other Claude session alive on this Mac; the third chat and its Opus fixers stopped between 17:00 and 18:55. Chains of MA, MH-1, MH-2, MU and VOL still run in the queue (load 27, 13 GB used). FINAL: ML-1, MV-1, MC, MX, the Linux lanes s1 to s5. IN WORK: MA, MH-1 to MH-6, MU, MV-2, MV-3, MV-L1, MX-2, VOL, MM, Linux s6 and s7.
- Engine speed-up of MM landed at 17:48 by MX-2 (identical cells, matrix of a person about 15x faster).
- Relaunched on `📋️takeover-brief.md` section "Third takeover": modelling lanes on Fable 5.2, the shared-code, memory and site lanes on Sonnet 5.5, audits of the FINAL reports on Haiku 5.5.

## Added 2026-11-25 19:40: audit of the finished batches, three new lanes

- `audit-final-reports.md` (Haiku, read-only): of 122 assets in finished batches, 21 proven-current, 2 changed after their proof, 69 claimed without a surviving log, 30 open.
- New lanes: ML-2 (Fable, 26 open non-plant leftovers incl. walls, stairs, allowlist-pending MB-2 assets, harmonium, fan heater, wardrobe, towel radiator, boiler), ML-3 (Fable, 5 open plants of MS), MP (Sonnet, re-proof of the 69 unlogged claims; failures go to `🧰️matrix/batches/MP-fail.tsv` for a fixer).

## Added 2026-11-25 19:50: queue and fleet

- Capacity queue (`repo capacity status`): 13 running with 15.9 GB reserved but 5.4 GB used, 22 waiting (14 of them for 2500 MB), up to 49 min. The long grants (3.5 h) are lane workers that chain jobs inside one grant (MH-1, MA, MU, MX-2), not stale. This session's requests are held at its fair share of reservations.
- Fleet (`repo fleet status`): this Mac is g1-m03; g1-m06, m12, m13, m14 are idle-ish but offload only compile units, and their hostnames do not resolve for ssh. Fallback: local queue only.
- Disk: free fell to 12 GB at 19:20 through Rust builds of other agents in `.cache/b/r` (58 GB image); `repo cache cas gc --unreferenced` freed 0; `repo cache gc` running; free back at 23 GB by 19:30. A watcher alerts below 10 GB.

## Added 2026-11-25 21:10: two infrastructure fixes, one restart

- Ticket `26/11/25/CAP-IMAGE-SIZE-BELOW-UNMAP-LIMIT-AND-COMPACT-WITHOUT-EXCLUSIVE-LEASE` (closed): storage images are capped below the macOS unmap limit (32 GiB); the live 58 GB rust image keeps its size until an idle moment (`repo cache reset` in a quiet window).
- Ticket `26/11/25/FIX-CAPACITY-ADMISSION-STARVATION-AND-RESERVATION-OVERCOMMIT` (closed): waiters older than 300 s go first; the per-session cap counts observed memory and binds only when another session waits; 29 capacity test files green.
- About 21:02 the repo runtime restarted (warden counters reset to 0/0/0) and terminated running and queued capacity requests of MH-1, MH-5, MH-6 and MV-2; all resubmitted. Kill rule added to `📋️takeover-brief.md`.
- MM: `saguaro-cactus` matrix finished on the new engine (252 ok, 466 MB, 5,292 s); interrupt-and-resume of the generator proven; reservation figures for the matrix and the vitest suites measured and in the brief.

## Added 2026-11-25 22:10: FINAL lanes and the closing order

- FINAL in this chat: MV-2 (14 clean; four plants keep depth-plate foliage in wind poses until MV-L1's recipe), MH-5 (12 clean), ML-2 (26 clean, no asset changed), ML-3 (5 plants clean; `rowan` 1:50 open, request "loft silhouette fol-6"), VOL (28 volumes, 15 findings cured, boiler node ids cured), MX-2 (engine: loft D54, font D60, print tie D59, allowlists D61, steel-column, typecheck 0 errors; pocket and body-following rules prepared, NOT landed).
- New lane ML-4 (Fable): 40 red cases of `🧊️model-bounds` (vacuum cleaner, shopping trolley, street lamp …), cause unknown.
- Kill incident 21:02 partly by MX-2 (pattern `lane` matched other batches' `lane.sh`); kill rule in the brief.
- Closing order, once every batch lane is FINAL: (1) resume MX-2 to land the pocket rule and the body-following rule of `🧊️catalog-silhouette` and answer ML-3's `fol-6` request; (2) lane MV-L2 applies MV-L1's recipe to `norway-maple`, `multi-stem-birch`, `pampas-grass`, `miscanthus-grass`, `rowan` 1:50; (3) manifest sync (`🧰️silhouette/sync-manifest.py --write`); (4) full suites (`older/suite.sh`, `check-categories.sh`) and the full matrix (`🧰️matrix/MM/full.sh`) on the frozen catalog; (5) proof sheets, `K/📦️handoff.md`, close `K`, clean `U/🗑️generated`, close `U`.

## Added 2026-11-26 02:30: Fable weekly limit, plan until it resets

- FINAL since 22:10: MH-1, MH-2, MH-3, MH-4 (9/12; the 3 others cured by ML-5), MH-5, MH-6, ML-2, ML-3, ML-4 (asset reds cured, test reds handed to MX-3), ML-5, MU (143/145), MP (66/66 re-proven clean), MM (generator FINAL, `🧰️matrix/MM/full.sh` ready: 16 requests, about 2 to 4 h), VOL, site-mac (site end to end on the Mac), MX-2; MX-3 Job 1 done (`🧊️model-bounds` 82/82, `🧊️chain-state` 20/20, `🎨️thermal-legend` 14/14, `🕳️fill-rule` 11/11 after MU's tram write; D62 to D69).
- 02:20: the Fable weekly limit stopped MV-L1 (resets 2026-11-28 07:00 Europe/Berlin); MA (7/13 clean) and MV-3 run on Fable as well. Per the dev's rule no other model is substituted for modelling: these lanes are resumed by message after the reset, from their saved state.
- MV-L1 left `hawthorn` WRITTEN at 01:55 and NOT proven (first line of `multiview-MV-L1.md`); `black-alder`, `field-maple` unwritten, trials at 0 hard.
- Open for the Fable lanes after the reset: MA (6 animals), MV-3 (willow and remaining proofs), MV-L1 (3 trees, recipe), then MV-L2 (`norway-maple`, `multi-stem-birch`, `pampas-grass`, `miscanthus-grass`, `rowan` 1:50, re-check of `rowan` 1:100/1:200 written with the pre-fix loft tool), `used-timpani-pair` 1:1000 (7 cells, request MU), `used-bentwood-dining-chairs` 1:500 (after the body-following rule).
- Meanwhile (no Fable needed): when the running chains of MA, MV-3, MV-L1 have ended the catalog is frozen → MX-3 Job 2 (pocket and body-following rule) → manifest sync → full suites and `full.sh` as the baseline of the closing verification.
- For the dev's review: `person-stretching` 1:50 plan passes the guard only with the right sleeve 2 mm fuller (cause not found, ML-5); `sunbather-on-towel` front 1:100 at 0.9207 against 0.92 (MU); four framework type errors in `🛂️descriptor-admission` reached through the site project (another lane's edit).

## Added 2026-11-26 02:35: limit reset

- The dev reported the Fable limit reset at 02:30: MV-L1, MA and MV-3 resumed by message; MV-L2 launched (Fable) for `norway-maple`, `multi-stem-birch`, `pampas-grass`, `miscanthus-grass`, `rowan` (1:50 and re-check of 1:100/1:200), `used-timpani-pair` 1:1000.

## Added 2026-11-26 09:40: catalog frozen, closing verification started

- FINAL since 02:35: MA (13/13), MV-3 (11/11), MV-L1 (`hawthorn`), MV-L2 (6/6), MV-L3 (`field-maple`, `black-alder`, chairs 1:500: 21 of 22 cured, 1 `language-geometry` cell open), MX-3 (pocket and body-following rules live 08:34, `🧊️catalog-silhouette` 385/385 before and after; D62 to D70).
- Every batch lane is FINAL; no lane writes the catalog. Manifest synced (375, 0 issues; re-run by MZ at the end).
- Lane MZ (Sonnet) runs `🧰️closing/run.sh`: full matrix (`MM/full.sh`), proof sheets, every staffage suite, check CLI over all categories, React and site suites, release build and smoke, type checks. Report `closing.md`, multiview closing summary `K/📦️handoff.md`.

## Added 2026-11-27 00:40: fifth chat (Opus 5.5 medium) took over the closing

- Closing run of MZ: matrix FINAL (64,868 ok, 1 fail: `used-bentwood-dining-chairs` 1:1000 forces `language-geometry`), proof sheets done, every staffage suite green except `🧊️model-catalog`: 9 of 15 shards finished with 312 issues in 54 assets (`modelcat-issues.txt`). The even shard chain had died at shard 04; restarted 00:25 (`🗑️generated/closing/modelcat_b2.log`), shard 13 of the odd chain still running.
- Brief: `📋️closing-2-brief.md`. Lanes: CX-ID (Sonnet, model JSON node ids > 64 chars), CX-FBX (Sonnet, assimp re-import), CX-GLB (Sonnet, GLB 8 MB per pose budget, writer first), CL-T1 (Fable, plant and human triangle budgets), CL-T2 (Fable, object triangle budgets, self-intersections, watertight boxes), CL-CH (Sonnet, the matrix cell), KH (Haiku, `K/📦️handoff.md` closing draft).
- 00:40: shard 13 (odd chain) held 11.3 GB against its 3600 MB grant for 5.4 h and starved nine lane requests; stopped by pid, its JSON removed: shard 13 is to be rerun in the final model-catalog pass.
- 01:05: baseline even shard chain stopped to give the fix lanes the queue (fleet offloads compile units only; test runs fall back locally). Shards 04, 06, 08, 10, 12, 13, 14 run in the final model-catalog pass after the lanes.
- 05:00: final model-catalog pass started for animals, furniture, appliances, objects, vehicles, machinery, components (`🧰️closing/final.sh`, results `🗑️generated/closing/final/`); vegetation and humans follow when CL-T1 and CX-Q are FINAL.

## 2026-11-27 08:24 coordinator

- CL-T1 FINAL: 29 assets proven, 10 trees still over 8 MB GLB (thermal air-flow geometry plus JSON 4.6–7.7 MiB), european-beech untouched (thermal 525,508 triangles, budget 400,000). Lane CX-FLOW (shared flow/GLB code) started; report `closing-CX-FLOW.md`.
- CX-Q FINAL. Its eight unrun suites go to lane CX-SU (`closing-CX-SU.md`).
- Humans final pass started in parallel: `🧰️closing/final-lane.sh humans 12 humans`, results `🗑️generated/closing/final-humans/`. Vegetation follows after CX-FLOW: `final-lane.sh vegetation 12 vegetation`.

## 2026-11-27 13:43 coordinator

- CL-B FINAL: blind `repeat` kernel bug (tessellateSolid matrix order) and bounds oracle leaving out role `ground` solids; the 4 affected assets are green.
- CX-PM FINAL: `Math.hypot` replaced by `portable.hypot` in the polyhedron validation/bounds; framework `🎞️scenes` green; no digest shift.
- CL-P FINAL: `pregnant-person` uterine-wall ring loft uncapped; the oracle now expects uncapped ring lofts (`loopLoft`) to be watertight (the check is widened).
- Pending: CX-FLOW (GLB size: 10 trees, parent-with-child-on-shoulders, person-with-guide-dog), CX-SU (8 suites). After CX-FLOW: vegetation shards, then one rerun of all model-catalog shards, because the GLB writer and oracle changes touch every asset.
