# 🧍️ Humans B: HW-10 … HW-18 of the Staffage Human Collection Wave

Finisher report of 2026-11-22 (20:25 to 21:55) for the umbrella ticket. Scope: the workers HW-10 … HW-18 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE` (K), their people in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans` (H). Nothing of HW-01 … HW-09 was touched.

## Result

- 24 started people: 23 had a catalog file, `postal-carrier` had a generator only. All 24 are shipped now with `validateAsset` 0 issues, the staffage check CLI `✓` (`--samples 9`), a row file in `K/📥️manifest-rows/<id>.json` that `validateManifest` accepts against its asset, and a worker report `K/📝️HW-10.md` … `K/📝️HW-18.md`. The multiview sheets (plan, front, side at 1:50, 1:100, 1:200, 1:500) of every one of the 24 were rendered and opened; the changed drawings were rendered and opened again after the change.
- State found: 18 of the 23 catalog files passed the check, 5 failed it (picnicker, painter, dancer, sunbather, queuing); 3 more passed it with a visible drawing defect (picnicker without a head in the plan, parent with a hidden head, pallet jack operator behind his load); 8 carried decision texts of their base figure (painter, poncho, winter coat, sunbather, checking watch, shopping bags, picnicker, parent).
- Never started (no generator, no candidate, no catalog file; not started now): `window-cleaner`, `security-guard` (HW-14), `gardener-with-wheelbarrow` (HW-18).
- Rows were not merged into the manifest. `K/📥️manifest-rows` held 29 files when this was written: the 24 of this scope and 5 of HW-01 … HW-09.

## Per asset

Final proof: `validateAsset` 0 issues and check `✓` for every line; the last column is the measured minimum feature under `office-laser` (limit 0.2 mm) at 1:500 / 1:200 / 1:100 / 1:50.

| Worker | Asset | Found at 20:23 | Done | Check ✓, min feature mm |
|---|---|---|---|---|
| HW-10 | `picnicker-sitting-on-ground` | ✗ check: 1:500 and 1:200 unsuitable (head 0.150, hair 0.165, blanket 0.162 mm), idle-breathing residual −89.7 N; no head in the plan; no row | regenerated: true plan outlines of head, hair, beard, ears, neck; idle clip in the cross-legged pose; blanket pen 0.20 mm at 1:500; volume decisions without guitar and stool | ✓ 0.212 / 0.210 / 0.259 / 0.300 |
| HW-10 | `person-eating-standing` | ✓, no row | unchanged; row | ✓ 0.236 / 0.232 / 0.208 / 0.277 |
| HW-10 | `person-drinking-coffee` | ✓, no row | unchanged; row | ✓ 0.214 / 0.218 / 0.282 / 0.255 |
| HW-11 | `waving-person` | ✓, row | unchanged; row verified | ✓ 0.272 / 0.260 / 0.252 / 0.202 |
| HW-11 | `photographer-with-camera` | ✓, row | unchanged; row verified | ✓ 0.244 / 0.280 / 0.263 / 0.322 |
| HW-11 | `person-taking-selfie` | ✓, row | unchanged; row verified | ✓ 0.258 / 0.294 / 0.254 / 0.250 |
| HW-12 | `standing-violinist` | ✓, row | unchanged; row verified | ✓ 0.234 / 0.280 / 0.264 / 0.294 |
| HW-12 | `painter-at-easel` | ✗ check: 1:500 unsuitable (palette 0.142, brush 0.154, hand-l 0.182 mm); decisions still described the cane; no row | regenerated: 1:500 pens for palette, brush and left fist; cane texts replaced, anchor `cane-grip` removed; row | ✓ 0.200 / 0.211 / 0.311 / 0.388 |
| HW-12 | `dancer` | ✗ check: pirouette@1.925 residual −2.43 N; no row | pirouette re-keyed without velocity jumps (2.8 s, 225 instants in equilibrium); dance garment and symbol texts; row | ✓ 0.230 / 0.240 / 0.318 / 0.320 |
| HW-13 | `person-sweeping` | ✓, row | unchanged; row verified | ✓ 0.210 / 0.280 / 0.276 / 0.306 |
| HW-13 | `person-hanging-laundry` | ✓, row | unchanged; row verified | ✓ 0.200 / 0.220 / 0.249 / 0.278 |
| HW-13 | `person-carrying-shopping-bags` | ✓, row with key poses (manifest-mismatch) | row lists named poses only; volume decisions state the paper bags instead of the shoulder bag | ✓ 0.202 / 0.230 / 0.247 / 0.286 |
| HW-14 | `postal-carrier` | generator only, no catalog file, no row | generated unchanged, shipped; row | ✓ 0.200 / 0.225 / 0.266 / 0.304 |
| HW-15 | `person-with-guide-dog` | ✓, no row | one breathing pattern title (cane → guide dog); row | ✓ 0.230 / 0.220 / 0.246 / 0.350 |
| HW-15 | `person-signing` | ✓, no row | unchanged; row | ✓ 0.242 / 0.280 / 0.261 / 0.294 |
| HW-15 | `person-with-arm-sling` | ✓, no row | unchanged; row | ✓ 0.202 / 0.225 / 0.294 / 0.330 |
| HW-16 | `person-in-rain-poncho` | ✓, decisions described the commuter (coat, scarf, bag, 1.78 m); no row | decisions rewritten; row | ✓ 0.206 / 0.284 / 0.286 / 0.340 |
| HW-16 | `person-in-winter-coat` | ✓, decisions described the commuter with shoulder bag; no row | decisions rewritten; row | ✓ 0.202 / 0.236 / 0.294 / 0.248 |
| HW-16 | `sunbather-on-towel` | ✗ check: 1:500 unsuitable (towel 0.150 mm), fanning-face@0.25/0.75 polygon union error; decisions described sweatshirt, jeans, sneakers; no row | regenerated: towel pen 0.20 mm at 1:500, hands in the front view swept along the hand bone; decisions rewritten; row | ✓ 0.220 / 0.230 / 0.282 / 0.318 |
| HW-17 | `person-queuing` | ✗ check: support-switched heel-l, heel-r; no row | supports are the balls of the feet, heel support solids removed (patch on the worker's file); row | ✓ 0.214 / 0.292 / 0.270 / 0.211 |
| HW-17 | `person-checking-watch` | ✓, decisions described texting with a phone; no row | decisions rewritten; row | ✓ 0.242 / 0.284 / 0.283 / 0.330 |
| HW-17 | `parent-with-child-on-shoulders` | ✓, but the parent's head hidden in side and front view (child's trunk inside the head, thighs across the face); no row | regenerated: seat lowered 7 cm, moved back, thighs spread, trunk painted under the head; height 2.211 → 2.137 m; texts of the walk poses; row | ✓ 0.206 / 0.210 / 0.254 / 0.328 |
| HW-18 | `worker-with-pallet-jack` | ✓, but the load covered legs and pelvis in the front view; no row | regenerated: truck and load painted under the operator in the front view; row | ✓ 0.256 / 0.216 / 0.300 / 0.287 |
| HW-18 | `person-carrying-ladder` | ✓, no row | unchanged; row | ✓ 0.206 / 0.208 / 0.332 / 0.437 |

## What stays imperfect

- `sunbather-on-towel`: the roughest drawing of the scope. At 1:50 the plan shows the seams between the twelve stacked trunk pieces as cross lines and the feet are blunt forms. It reads as a person lying on a towel; it is not at the level of the best humans.
- `parent-with-child-on-shoulders`: in the front view the parent's face is still largely covered by the child's legs and the parent's raised hands. The regenerated file differs from the worker's shipped file in 8453 values even before my three edits (pose fits to 0.02°, re-stated volumes); I shipped the regenerated one because it validates, checks and reads better.
- `picnicker-sitting-on-ground`: the pose `leaning-back` is in equilibrium but one breath away from a switch of the solver's support distribution (request filed); no clip uses it any more. In that pose `ground: auto` lifts the blanket 17 mm off the ground.
- `painter-at-easel` and `postal-carrier` sit exactly on the 0.200 mm feature limit at 1:500.
- `person-queuing`: its generator stops on today's `standing-adult` (changed at 16:58 by another ticket); the shipped file is the worker's plus `patch.py person-queuing`. Other generators deriving from `standing-adult` were not re-run and may not reproduce their shipped files either.
- Decision texts: rewritten compactly for poncho, winter coat, sunbather and checking watch (two tiers: 1:50/1:100 and 1:200/1:500), and stale sentences were removed from the volume decisions, which are short in places. `dancer` keeps the jogger's body and says so; `postal-carrier` still says leather bag for the satchel in a few volume sentences; thermal assumptions of `waving-person` and `person-drinking-coffee` still mention the base's fanning and shiver clips.
- Four people have no `idle-breathing` clip but two activity clips (guide dog, parent, pallet jack, ladder). The ticket brief asks for one looping activity clip plus idle breathing or a gait; all four have a gait.
- Several text patches live in `K/🧰️scripts/FIN-B/patch.py` (idempotent, named per asset) and not inside the worker's generator: re-running a generator of HW-12 (dancer), HW-13 (bags), HW-15 (guide dog), HW-16, HW-17 (watch, queuing) or HW-10 (picnicker volume decisions), HW-17 (parent texts) must be followed by `python3 patch.py <slug> --ship`.
- Tests: the catalog vitest suites were not run. Scoped by name to my changed people, `✅️asset-validation`, `⚡️asset-power` and `🔭️multiview-coverage` skipped every case (83, 24, 23 skipped): they take their cases from the manifest, so they reach these people only after the merge. The other finisher's finding on the Hausdorff gate of `catalog-silhouette` (requests.md) applies to this scope as well; I did not measure it per asset.
- Two slips of mine, both undone: at about 20:53 a run of `gen_person_queuing.py` wrote a half-built `person-queuing` into the catalog before it stopped; the file was restored byte-identical from my backup in the next command. At about 21:33 a wrong relative path wrote one draft to `/Users/ueli/Documents/.🧬semio/…` outside the repo; the file was moved into the ticket and the empty folders removed.

## Shared-code requests still open

Appended to `requests.md` of this ticket under `## STAFFAGE-HUMAN-COLLECTION-WAVE — … (humans b)`:

1. `unionPolygons` cannot place an extruded shape seen along its profile (sunbather hand; worked around in the asset).
2. The structure solver leaves a force residual when its support distribution switches (picnicker −90 N, dancer −137 N lateral; worked around in the clips).
3. Nine check samples miss the jerks at clip keys (dancer: −2839 N and +4140 N at key instants).
4. The plan head of `seated-guitarist` is a slab stretched for its own playing pose (base asset, not fixed).
5. `standing-adult` changed under the wave (information for the owner of that change).

## Files

Created:

- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/postal-carrier/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/picnicker-sitting-on-ground.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-eating-standing.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-drinking-coffee.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/painter-at-easel.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/dancer.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-with-guide-dog.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-signing.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-with-arm-sling.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-in-rain-poncho.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-in-winter-coat.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/sunbather-on-towel.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-queuing.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-checking-watch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/parent-with-child-on-shoulders.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/worker-with-pallet-jack.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-carrying-ladder.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/postal-carrier.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-10.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-11.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-12.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-13.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-14.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-15.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-16.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-17.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📝️HW-18.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/check.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/checkfile.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/com.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/crop.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/depict.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/diff.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/env.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/look.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/montage.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/order.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/patch.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/png.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/reports.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/rows.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/stale.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/struct.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/suit.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-B/validate.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-10/planrings.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/humans-b.md`

Updated:

- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/picnicker-sitting-on-ground/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/painter-at-easel/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/dancer/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-carrying-shopping-bags/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-with-guide-dog/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-in-rain-poncho/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-in-winter-coat/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sunbather-on-towel/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-queuing/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-checking-watch/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/parent-with-child-on-shoulders/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/worker-with-pallet-jack/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/📥️manifest-rows/person-carrying-shopping-bags.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗂️work-board.md (rows HW-10 … HW-18 only)`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-10/picnicker-sitting-on-ground.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-10/picnicker-sitting-on-ground-content.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-12/lib.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-12/build-painter.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-12/texts-painter.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-13/manifest_rows.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-16/lib.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-16/sunbather.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-17/gen_person_queuing.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-17/gen_parent_with_child_on_shoulders.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-18/lib.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-18/pallet-jack.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md (five sections appended)`

Removed:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/FIN-B/ (created and deleted in this session: drafts, backups, check logs, sheets)`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-10/ (created and deleted in this session: drafts, backups, check logs, sheets)`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-12/ (created and deleted in this session: drafts, backups, check logs, sheets)`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-14/ (created and deleted in this session: drafts, backups, check logs, sheets)`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-16/ (created and deleted in this session: drafts, backups, check logs, sheets)`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-17/ (created and deleted in this session: drafts, backups, check logs, sheets)`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-18/ (created and deleted in this session: drafts, backups, check logs, sheets)`

`K/🗑️generated` still holds `FIN-A`, `HW-02` and `HW-04` of the other finisher; I left them.
