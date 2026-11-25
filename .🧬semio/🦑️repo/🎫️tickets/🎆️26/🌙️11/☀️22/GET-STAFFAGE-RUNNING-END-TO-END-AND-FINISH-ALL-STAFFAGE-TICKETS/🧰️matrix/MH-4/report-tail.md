## 4. Open, with numbers

Each line is a finding the gate still reports over every subject after the repair (section 3 lists them per asset with counts). In every case the repair that clears it was built and measured, and withdrawn because the real `🧊️catalog-silhouette` guard of the default pose left its limit with it or another check turned red; `U/requests.md` ("multiview MH-4 — `🧊️catalog-silhouette` and the multiview gate ask for opposite things") has the measurements.

| Asset | Finding left | Why the repair is not in the file |
|---|---|---|
| `person-with-white-cane` | 1:500 `thigh-l` `sync-bone-height` 93 mm in every subject (the flat front thigh is drawn 84 to 93 mm above its place); 1:50 `hand-r` `sync-bone-forward` 67 mm and `sync-forward` 51 mm, `arm-r` 45 mm; 1:200 `thigh-r` height where the gate names it | the thigh at its place: front guard 80.2 to 167.0 mm (limit 150) although the IoU rises; a plan hand as a body: 14.6 to 19.6 and 27.7 mm (limit 15.0); the thigh as a solid at 1:200 leaves 0.19999 mm of the hips visible (print profile 0.2 mm) |
| `person-with-arm-sling` | `sync-forward` 38 mm (1:50) and 37 mm (1:100) against 36 mm: the elevation hand in the sling reaches further than the plan hand | the plan hand as a body: guard 8.4 to 36.9, 40.6 and 43.2 mm (three constructions) |
| `postal-carrier` | `hand-l` `sync-bone-forward` 72 to 73 mm and `sync-forward` 58 mm at 1:50 and 1:100 | the plan hand as a body: guard 11.3 to 36.4 and 43.5 mm (limit 15), 36.5 and 43.3 mm at 1:100 (limit 30) |
| `person-with-rollator` | `coat` plan 151 to 168 mm at 1:50, 1:100, 1:200 when the trunk stoops; hands at 1:50 63 mm; `sync-forward` 55 mm at 1:50 in the half turn of the rollator | the side outline swept laterally is a rectangle in the plan: 34.4, 34.6 and 64.5 mm at the round back (limits 15, 30, 60); a prism of the plan outline is a box, 53 to 100 mm too long; hands: front guard 82.6 mm; the tyre is a ring path that the fixer's op flattens wrongly (elevation guard 84.8 mm) |
| `person-with-guide-dog` | hands at 1:50 (77 and 81 mm), dog thigh plan at 1:100 (71 mm) and what the full gate of section 3 lists | hands: front guard 60.0 mm, plan 35.8 mm; dog thigh and shank as the side outline swept laterally: 18.8 to 39.7 mm at 1:50; `dog-neck` plan 45.0 mm at 1:100; `thigh-r` front at 1:200: sliver of the hips below the print profile |
| `person-stretching` | arms at 1:50 (the three views are three different bodies), the plan of trunk, hem, sleeves and right foot (section 3) | the plan IoU of the document as it was is 0.9219 (1:200) and 0.9321 (1:500) against 0.92: every plan repair measured takes it below (0.9185, 0.9155, 0.9078, 0.896; 0.8345 at 1:500 with all of them); the sleeve as one body at 1:50 moves the front guard from 13.7 to 19.3 mm and the plan guard to 21.0 mm |

Not repaired on purpose: the reported tier (`group-drift-*`, twins of the fixer's gate on bones with several shapes) where it does not reach the extents of the whole figure; the two kinds the suite's own change covers (anchors on descendant bones, per-bone hard tier for twins of one id only).

## 5. Files

Updated (asset documents, `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<id>/🔣️.json`): the assets whose section 3 shows a proof.

Updated: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (two entries `## multiview MH-4 — …`).

Created (all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`):
- `multiview-MH-4.md`
- `🧰️matrix/MH-4/ops/<id>.json` for the 12 assets
- `🧰️matrix/MH-4/`: `env.sh`, `lock.sh`, `cap.sh`, `fit.ts`, `gate.ts`, `plan.ts`, `rest.ts`, `probe.ts`, `culprit.ts`, `frames.ts`, `scan-file.ts`, `suit.ts`, `auto.py`, `codes.py`, `singles.py`, `show.py`, `worst.py`, `report.py`, `report-head.md`, `report-tail.md`, `notes/<id>.md`, `make-report.sh`, `try.sh`, `do.sh`, `apply.sh`, `finish.sh`, `prove.sh`, `look.sh`, `peek.sh`, `zoom.sh`, `bisect.sh`, `gate-all.sh`, `hard.sh`, `l.sh`, `all.sh`, `status.sh`, `short.sh`, `done.sh`, `w.sh`

Removed: `🗑️generated/matrix/MH-4` (backups, dry runs, logs, sheets, matrix folders) when the batch ended.

Manifest: no declared dimension changed. Shared engine, schema, validator, tests, fixtures, README: not touched.

## 6. The queue rule of 23:00

From 23:00 on every run goes through `repo capacity run` (`cap.sh`: 7000 MB matrix, 5000 MB check CLI, 2500 MB gate, measure and vitest, 1000 MB scan and small scripts). Proved before the rule and not repeated: `pregnant-person`, `person-with-umbrella`, `person-taking-selfie` (their proof lines carry no "(through repo capacity run)"). The matrix of `person-with-white-cane` was in flight at 23:00, was left to end and is superseded by the repeated proof of that asset. The script chains of the batch were stopped at 23:02 (the bun process in flight was left alone) and started again through the queue; the dry-run gates of `person-with-guide-dog` and `person-stretching` that the stop interrupted were repeated through the queue.
