FINAL

# 🔭️ Multiview Later Round ML-1: Six Assets Left by MF and MO, Three Checks Left by MF (2026-11-25)

Fixer report after the takeover of 2026-11-25 09:00 (`📋️takeover-brief.md`, `📋️multiview-brief.md`; paths `S`, `T`, `U` as there). Tools: `🧰️matrix/ML-1/`. Every proof line below was run to its end between 08:54 and 10:35 on the catalog documents as they are, by `🧰️matrix/ML-1/takeover.sh`, one request at a time through `repo capacity run` with the reservations of the queue rule (800 MB scans, gates and chain; 1000 MB check CLI; 1200 MB matrix of one asset and sheets; 2500 MB filtered vitest). Numbers called "before" come from the reports of MF and MO and from the predecessor's gate run of 01:00 (`why.ts`); they were not measured again, because the documents before the repair are no longer in the catalog.

## 1. Result

- **5 of the 6 assets are clean with all proofs**: `cafe-chair`, `office-swivel-chair`, `used-fridge-freezer`, `used-tumble-dryer`, `used-concert-harp`. `used-washing-machine` (check b of MF) is repaired and clean too.
- **Open: `used-harmonium`, 21 `fail` cells** (`paper-height`), asset unchanged. The cause is the arithmetic of the matrix, filed in `requests.md`.
- State found at the takeover: the six catalog documents were written by the predecessor (five at 04:29, `used-concert-harp` again at 06:06) and its proof logs were complete but never reported. Nothing had to be put back: each document is byte for byte what the chain produces, and every proof was run again.
- The three checks of MF: (a) `model-bounds`, 3 heater cases fail before and after MF, not caused by MF, open as a note in `requests.md`; (b) the rear feet of `used-washing-machine` now stand on the body; (c) all MF sheets opened, nothing wrong found (section 6).

## 2. Per asset: findings before and after

Gate = `../MR/gate.ts` through `why.ts`: every pose, clip instant and transition of the matrix at every authored scale. Scan = `T/🔎️scan.ts --language en`. Matrix = `T/📊️matrix.ts --language en --jobs 1 --fresh --assets <id>`, 252 cells each.

| Asset | Hard findings before (gate) | Gate after | Scan after (hard / reported) | Matrix before | Matrix after | Peak |
|---|---|---|---|---|---|---|
| `cafe-chair` | 3: `sync-height` 18.4 mm (tolerance 17.6) at 1:50, clip `tip-back` | 0 | 0 / 2 | 14 fail | **0 fail**, 154 ok, 98 group-drift | 236 MB |
| `office-swivel-chair` | 106: `sync-lateral` 28.7 to 32.0 mm at four bands, `sync-forward` 20.4 mm at 1:200 and 1:500 (tolerance 20) | 0 | 0 / 39 | 70 fail | **0 fail**, 21 ok, 231 group-drift | 346 MB |
| `used-fridge-freezer` | 15: `sync-forward` 83.1 to 89.9 mm (tolerance 37.1) at 1:50, 1:100, 1:200 | 0 | 0 / 12 | 42 fail | **0 fail**, 140 ok, 112 group-drift | 620 MB |
| `used-tumble-dryer` | 6: `sync-lateral` 20.4 mm (tolerance 18) at 1:50, 1:100 | 0 | 0 / 5 | 42 fail | **0 fail**, 35 ok, 196 group-drift, 21 unavailable | 349 MB |
| `used-concert-harp` | 157: `anchor-landmark` `shoulder` in the plan, 122.0 to 127.8 mm (tolerance 39.1) | 0 | 0 / 14 | 84 fail | **0 fail**, 35 ok, 217 group-drift | 408 MB |
| `used-washing-machine` | 0 (drawing defect, check b) | 0 | 0 / 0 | 0 fail | **0 fail**, 210 ok, 42 group-drift | 324 MB |
| `used-harmonium` | 0 in the gate; scan `paper-height` ×2 | 0 | **2** / 0 | 21 fail | **21 fail**, 210 ok, 21 unavailable | 547 MB |

`group-drift` is the reported tier of the matrix, not a failure. The gate visited 75 to 127 subjects per scale and asset.

## 3. Proof (takeover run, each to its end)

| Check | Result |
|---|---|
| chain: `fix.ts <id> --out` compared with the catalog document | 6 of 6 byte for byte identical (the step of MF or MO, then `spec/<id>.json`; nothing undone, a second run changes nothing) |
| spec writers `make/office-swivel-chair.py`, `make/used-concert-harp.py` on the written catalog | both specs unchanged |
| gate over poses, clip instants and transitions (`why.ts`), 7 assets | 0 hard findings |
| `T/🔎️scan.ts --language en`, 7 assets | 6 clean, 1 with a hard finding: `used-harmonium` `paper-height` ×2 (1:50 and 1:100, "drawn height 22.54 mm but 1.15·20.000 = 23.00 mm") |
| `validateAsset` | 7 of 7 with 0 issues |
| print at every authored scale and both range ends | 7 of 7 |
| check CLI `--asset <id>` | `✓` for all 7 |
| `🧊️catalog-silhouette` (name filter) | 7 passed, 378 skipped (385) |
| `🧊️three-views` | 14 passed, 761 skipped |
| `🌡️catalog-physics` | 14 passed, 826 skipped |
| `✅️asset-validation` (ids, manifest cases, sample figure) | 10 passed, 73 skipped |
| `🫁️breathing-catalog`, `⚡️asset-power` | no case names one of the assets (290 and 24 skipped) |
| matrix per asset | section 2; exit 0 for all 7 |

## 4. What was changed (by the predecessor; verified here)

The chain of each asset is the step of MF or MO and then `🧰️matrix/ML-1/spec/<id>.json` applied by `fix.ts`.

- **`cafe-chair`** (front 1:50): the rear leg and the back post were one shape on the bone `leg-rear`; the post stands on the bone `back`. `leg-rear-tube-0`, `-1` take the solid the plan draws for the leg, the post is its own shape `back-post-0`, `-1` on `back` (op `solid-from`).
- **`used-tumble-dryer`** (front 1:50, 1:100): the packing tape reached 65 mm past the door edge and swung out with the door. `tape` on the door ends at the door edge (60 mm), the rest is `tape-end` on the cabinet face.
- **`used-fridge-freezer`** (front and plan, four bands): the plan shapes of the hand truck frame were 2 mm skins at the top of the frame. `tube-r`, `tube-l`, `top-bar`, `strut-r`, `strut-l` are swept bodies now (op `sweep-twin`), the front tubes and cross bars are swept over the tube diameter.
- **`office-swivel-chair`** (front at four bands, cushion in the side elevation): backrest shell and seat cushion were boxes over full width and depth, so their corners reached past the curved shell of the plan when the seat swivels. Both are nested (op `nest`): the drawn outline stays on top, 20 hidden pieces follow the crescent and the rounded corners; no rest outline moved.
- **`used-concert-harp`** (plan at four bands, front at 1:50 and 1:100): the soundbox was three flat outlines. It is solids read from the body of the band (`make/used-concert-harp.py`): the back as eight prisms, the soundboard and centre strip swept over their width, the foot as its front face. The foot is left out of the plan at 1:200 and 1:500 and the front of those two bands stays flat, because solids there do not print (0.156 to 0.195 mm against 0.2 mm).
- **`used-washing-machine`** (front 1:50, 1:100; check b): `cabinet-underside`, the lowest 20 mm of the cabinet shell swept over the width, hidden at rest. In `carried` it closes the gap between the lifted panel and the rear feet.
- **`used-harmonium`**: unchanged.

## 5. The three checks MF left

**a. `model-bounds`, heater.** The real test with the name filter `heater`: 5 passed, 3 failed. "keeps the carry grip open and resolves the heater outlet" passes at 1:50 and fails at 1:100 (`light-solid` louvre bands 4, `thermal` 6, wanted 8), at 1:200 and 1:500 (hand clearance closed at all three lateral positions in `light-solid`, `thermal`, `forces`; louvre bands 1; at 1:500 the dial end is missed in `light-solid`). `heater.ts` casts the same rays on any document: the catalog now gives 1 of 4; the predecessor's run of 01:42 on the committed documents of 2026-11-22 (`01324745f80`) and 2026-11-21 (`b4927d2c311`) gave 1 of 4 with the same lines per scale (not run again here, those copies are tool output). So the failures are older than MF and not in `light-solid` alone. Not repaired: `electric-fan-heater` is not an asset of this round, and the cure is in the 3D bodies of 1:100 to 1:500. Filed as a note in `requests.md`.

**b. Washing machine, pose `carried`.** Repaired, section 4. Sheets 1:50 and 1:100 opened: the rear feet stand on the underside, door and panel as before.

**c. Sheets.** Section 6.

## 6. Sheets opened by eye (the product's own 2D drawing, `r3-2d.ts`; side, front, plan in line and solid view)

Assets of this round, drawn from documents identical to the catalog (5 by checksum, the harp rendered again from the catalog), 22 sheets:

| Asset | Sheets | Judgement |
|---|---|---|
| `cafe-chair` | 1:50 `tilted-on-two-legs` | leg and post continuous, no seam |
| `used-tumble-dryer` | 1:50 clip `clean-filter` 2.7 s, 1:100 `door-open` | one tape piece stays on the cabinet, one on the door |
| `used-fridge-freezer` | 1:50 clip `carry` 3.2 s; 1:100, 1:200, 1:500 `carried` | plan shows the frame as tubes behind the fridge; nothing stray |
| `office-swivel-chair` | 1:50, 1:100, 1:200, 1:500 clip `swivel-rotation` 0.5 s; 1:50 `reclined` | no stray piece when turned 45 degrees; reclined clean |
| `used-washing-machine` | 1:50, 1:100 `carried` | feet on the body |
| `used-concert-harp` | 1:50, 1:100, 1:200, 1:500 `played`; 1:50 `stored`, `upright`; before: 1:50 `played`, `stored` | plan: the soundboard is the long tapering trapezoid the side elevation shows. Front 1:50 `played`: the foot of the soundbox is a rectangle above the fully visible pedal base where it was a half ellipse over the pedals. Kept: it is what the solids give and agrees with the upright front; a matter of taste, named in section 8 |

MF assets that nobody had opened, 55 sheets from the catalog: the tilted pose at every authored band (`carried` for `used-bedside-table`, `used-dishwasher`, `used-electric-cooker`, `used-high-chair`, `used-hinged-wardrobe`, `used-kitchen-base-cabinet`, `used-microwave-oven`, `used-recliner-chair`, `used-wingback-armchair`; `dragged` for `used-park-armchair`; `reclined` for `used-recliner-chair`), the rest pose at 1:50 of these ten and of `used-electric-guitar-and-combo`, `used-electronic-drum-kit`, `used-electronic-organ`, plus `electric-fan-heater` `off`, `gas-hob` `high`, `used-bedside-table` `drawer-open`, `used-kitchen-base-cabinet` 1:500 `stored`. Six sheets do not exist because the asset declares the band unavailable (1:500 of bedside table, dishwasher, cooker, high chair, microwave; 1:200 of the microwave).

Judgement: no drawing is wrong or uglier. The three views tilt together in every sheet; covered parts (carcass under the top, rear feet) appear only where the tilt shows them; `used-kitchen-base-cabinet` 1:500 `carried` shows the blanket under the lifted cabinet. Two differences of 20 mm between side and front height are visible in the dimension lines and lie inside the tolerance (`used-recliner-chair` `reclined` 0.92 against 0.94 m, `used-wingback-armchair` `carried` 1.84 against 1.86 m). The sheets of 1:100 to 1:500 were judged at 1000 pixels width; one detail was enlarged.

## 7. Open

- **`used-harmonium`: 21 `fail` cells**, `paper-height` (1:50 ×7 modes, 1:100 ×7, the alternative `500>alt1:scale:100` ×7; 23 mm, "drawn height 11.27 mm but 1.15·10.000 = 11.50 mm"). The declared section leaves 1.127 m, the declared height is 1.15 m, D18 is 23 mm: the difference is the tolerance exactly, and the suite passes it in metres and fails it in paper millimetres by rounding. An outline of the crest in front of the cut clears the scan but breaks the silhouette guard (28.4 mm at 1:50, limit 15), so the asset stays unchanged. Request: "multiview ML-1 — `paper-height`: a tie at the tolerance …".
- **`electric-fan-heater`**: 3 of 4 `model-bounds` cases fail, section 5a. Request note: "multiview ML-1 — `🧊️model-bounds`: 3 of the 4 heater cases …".
- `used-concert-harp`: the front of 1:200 and 1:500 keeps the flat soundbox outlines (reported tier only, no failing cell).

## 8. Decisions worth a second look

- `used-concert-harp`: foot of the soundbox drawn as its front face (rectangle in the played front view); foot left out of the plan at 1:200 and 1:500.
- `used-tumble-dryer`: the tape is cut at the door edge instead of drawing the swinging flap in the plan too.
- `office-swivel-chair`: the front plate of the backrest stands 33 to 36 mm further forward at its level; its front projection at rest is unchanged.
- `used-washing-machine`: the front panel skin stays at 0.584 to 0.586 m, on the door glass, because at the true face the paint order of the pose would put it over the door ring (request of batch MO on `shapeDepth`).

## 9. Files

Catalog documents updated by this round (`🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/…/🔣️.json`): `🛋️furniture/cafe-chair`, `🛋️furniture/office-swivel-chair`, `🔌️appliances/used-fridge-freezer`, `🔌️appliances/used-tumble-dryer`, `🔌️appliances/used-washing-machine`, `🪑️objects/used-concert-harp`. The takeover wrote no catalog document. Not changed: `🪑️objects/used-harmonium`, `🔌️appliances/electric-fan-heater`.

Under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`:

- Updated: `multiview-ML-1.md` (rewritten), `requests.md` (two sections "multiview ML-1 — …": `paper-height` by the predecessor, `model-bounds` by the takeover).
- Created by the takeover: `🧰️matrix/ML-1/takeover.sh`.
- Created by the predecessor, kept: `🧰️matrix/ML-1/` with `env.sh`, `fix.ts`, `apply.ts`, `files.ts`, `print.ts`, `validate.ts`, `trial.ts`, `tt.sh`, `why.ts`, `why.sh`, `at.ts`, `heater.ts`, `looks.sh`, `apply.sh`, `steps.sh`, `prove.sh`, `after.sh`, `make/lib.py`, `make/office-swivel-chair.py`, `make/used-concert-harp.py`, `spec/` (`cafe-chair.json`, `office-swivel-chair.json`, `used-concert-harp.json`, `used-fridge-freezer.json`, `used-tumble-dryer.json`, `used-washing-machine.json`).
- Removed: `🗑️generated/matrix/ML-1` (backups, trial copies, logs, sheets, matrix folders).
