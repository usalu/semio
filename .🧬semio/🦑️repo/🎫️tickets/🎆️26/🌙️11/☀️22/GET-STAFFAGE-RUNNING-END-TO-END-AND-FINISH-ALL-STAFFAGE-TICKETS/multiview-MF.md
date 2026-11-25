# 🪑️ Multiview Batch MF: 9 Furniture and 11 Appliances (2026-11-24)

Fixer report. Batch `🧰️matrix/batches/MF.tsv`, brief `📋️multiview-brief.md`, paths `S`, `T`, `U` as there. Tools and specs: `🧰️matrix/MF/`. Measured with the authors' gate `T/🔎️scan.ts` and confirmed with `T/📊️matrix.ts --jobs 1 --assets <id>`.

## 1. Result

- Authors' gate (`T/🔎️scan.ts`, every pose of every authored scale): **20 of 20 clean** (before: 0 clean, 324 hard findings).
- Matrix (`T/📊️matrix.ts`, also clip instants and transitions): **16 of 20 without a `fail` cell**. Four keep failing cells, all in clip instants or transitions that the gate does not visit: `cafe-chair` (14), `office-swivel-chair` (70), `used-fridge-freezer` (42), `used-tumble-dryer` (42). Numbers and causes in section 6.
- No band copy and no in-band view in the batch: no `allowlist.json` was written.
- The manifest was not edited; no declared dimension changed.

## 2. Per asset: findings and matrix

Hard findings of the gate by code, before (start of the batch) and after; matrix cells of the asset by status with the failing check.

| Asset | Hard findings before | Hard after | Reported after | Matrix cells after |
|---|---|---|---|---|
| `cafe-chair` | 2: sync-bone-height 1, sync-forward 1 | none | 12 | 14 fail, 112 group-drift, 126 ok (sync-height 14) |
| `electric-fan-heater` | 18: anchor-landmark 18 | none | 0 | 210 ok, 21 unavailable |
| `gas-hob` | 3: anchor-landmark 3 | none | 6 | 84 group-drift, 147 ok, 21 unavailable |
| `office-swivel-chair` | 4: sync-forward 4 | none | 47 | 70 fail, 182 group-drift (sync-forward 28, sync-lateral 56) |
| `used-bedside-table` | 18: anchor-landmark 12, sync-bone-height 1, sync-forward 4, sync-height 1 | none | 2 | 70 group-drift, 140 ok, 21 unavailable |
| `used-dishwasher` | 8: sync-bone-forward 1, sync-bone-height 4, sync-height 3 | none | 24 | 140 group-drift, 70 ok, 21 unavailable |
| `used-electric-cooker` | 5: anchor-landmark 2, sync-forward 3 | none | 6 | 210 group-drift, 21 ok, 21 unavailable |
| `used-electric-guitar-and-combo` | 16: anchor-landmark 16 | none | 0 | 252 ok |
| `used-electronic-drum-kit` | 20: anchor-landmark 20 | none | 0 | 252 ok |
| `used-electronic-organ` | 24: anchor-landmark 24 | none | 0 | 210 ok, 21 unavailable |
| `used-fridge-freezer` | 76: anchor-landmark 70, sync-bone-forward 3, sync-forward 3 | none | 24 | 42 fail, 126 group-drift, 84 ok (sync-forward 42) |
| `used-high-chair` | 17: sync-bone-forward 10, sync-bone-height 4, sync-forward 3 | none | 7 | 28 group-drift, 203 ok, 21 unavailable |
| `used-hinged-wardrobe` | 13: anchor-landmark 6, sync-bone-forward 5, sync-bone-height 2 | none | 0 | 231 ok, 21 range-end |
| `used-kitchen-base-cabinet` | 1: sync-height 1 | none | 12 | 112 group-drift, 140 ok |
| `used-microwave-oven` | 3: anchor-landmark 1, sync-bone-height 2 | none | 6 | 42 group-drift, 126 ok, 42 unavailable |
| `used-park-armchair` | 20: anchor-landmark 8, sync-bone-forward 4, sync-bone-height 4, sync-forward 4 | none | 16 | 252 group-drift |
| `used-recliner-chair` | 14: anchor-landmark 8, sync-bone-height 2, sync-height 4 | none | 0 | 252 ok |
| `used-tumble-dryer` | 16: anchor-landmark 12, sync-bone-height 1, sync-height 3 | none | 17 | 42 fail, 154 group-drift, 35 ok, 21 unavailable (sync-lateral 42) |
| `used-washing-machine` | 42: anchor-landmark 28, sync-bone-forward 2, sync-forward 6, sync-height 6 | none | 0 | 42 group-drift, 210 ok |
| `used-wingback-armchair` | 4: sync-height 4 | none | 0 | 252 ok |

## 3. Proof

Each run to the end in this session on the final catalog documents (`prove.sh`, `matrix.sh`, `verify.sh`).

| Check | Result |
|---|---|
| `T/🔎️scan.ts --language en` on the 20 ids | 20 clean, 0 hard findings |
| `validateAsset` | 0 issues for all 20 (before: 0) |
| check CLI `--asset <id>` | `✓` for all 20 |
| every authored scale prints (default profile) | all 20; with both range ends: 19, `used-hinged-wardrobe` 1:20 (gap `mirror` / `handle-2-bar`) fails as it did before the batch, unchanged document part |
| `🧊️catalog-silhouette` | 20 passed |
| `🧊️three-views` | 40 passed |
| `🌡️catalog-physics` | 40 passed |
| `✅️asset-validation` (ids, manifest cases, sample figure) | 10 passed |
| `🫁️breathing-catalog`, `⚡️asset-power` | no case names an asset of the batch (290 and 24 skipped) |
| `🧊️model-glyph-size`, `🧊️model-glyph-edges` (use `electric-fan-heater`) | 1 and 4 passed |
| `🧊️model-bounds`, cases of the heater | 5 passed, 3 fail at 1:100, 1:200, 1:500 in the `light-solid` view (louvre bands 4 for 8, hand clearance). The repair changed only anchors, contacts and the `structure` lists of this asset; `body` and `thermal` of every scale are identical to the document before the batch, so these failures are not from this batch. Not run before the repair, so this is by comparison of the documents, not by a before run. |
| second run of `fix.ts` (`verify.sh`) | changes nothing for all 20 |
| matrix per asset | table in section 2 |

Rest pose: `restdiff.ts` (evaluated outlines of the default pose before and after) showed no moved outline except the added covered parts and the cases named in section 4. Silhouette guard (`../MR/measure.ts`, before and after): identical for 12 assets, within 0.0001 for `office-swivel-chair`; changed for seven, all inside the gate: `cafe-chair` plan 1:50 0.9908 / 1.9 mm to 0.9905 / 3.8 mm, `used-park-armchair` plan up to 0.9952 / 7.2 mm (legs as tapered solids), `used-recliner-chair` front 1:100 0.9974 / 18.8 mm to 0.9942 / 21.2 mm, `gas-hob` front 1:50 0.9793 to 0.9784, `used-washing-machine` elevation 1:500 (4.4), `used-bedside-table` elevation 1:100 better (26.0 to 10.4 mm), `used-kitchen-base-cabinet` front 1:500 better.

Sheets opened (r3-2d, 1:50): `used-tumble-dryer` `carried` before and after, `used-fridge-freezer` `carried` before and after, `used-washing-machine` `carried`, `used-bedside-table` `carried`, `office-swivel-chair` rest before and after (strips variant, rejected). **Not opened: every other changed view and scale** (the other 15 assets and the bands 1:100 to 1:500); for those the evidence is the outline comparison and the silhouette guard only.

## 4. What was wrong and what was changed

All repairs are `spec/<id>.json`, applied by `fix.ts <id> --write` (the last step of the chain of each asset; none of the 20 has a generator, ops or make script of an earlier round that runs after it). A second run changes nothing.

### 4.1 Flat front and plan shapes on a bone that a pose tilts (cause D), 14 assets

The carry, tilt and drag poses turn the whole piece 2 to 55 degrees about a lateral axis (a bone, or the root for `cafe-chair` and `used-park-armchair`). Front and plan were flat sheets carried at one depth, so the views kept the rest picture while the side elevation tilted. Rule `solidify` of the spec makes every closed front and plan shape of the tilting bones a solid:

- a shape with an elevation twin (same id and bone, or the twin the spec names: the left foot of a front view is the front foot of the side view) becomes the elevation profile swept laterally (`sweep: "w"`) when it is a plain rectangle in its view, else its own outline swept along the bone axis that points at the viewer (`sweep: "u"` or `"v"`) over the depth the twin gives. The rest drawing stays as it was;
- a shape without a twin (a front panel, a control, a sticker, a ring mark) becomes a skin of 2 mm on the face of the bone the view sees, so it stays on its face when the piece tilts;
- a part on a bone that rests a little inclined (legs at 82 to 96 degrees, a back at 98 to 110, a seat at 2 to 4) is swept along its own normal; the section is the drawn outline narrowed by the lean of the sweep. Legs that are also splayed sideways (`used-park-armchair`) are the elevation profile swept laterally with the span of the foot at the joint and the span of the top as `zTip`;
- open strokes (seams, grille slots, scratches) stay flat sheets, as in batch MB-2. They ride at the middle depth of their bone, so in a tilted pose a stroke on the front face lies up to half the depth times the sine of the tilt off its panel (`used-tumble-dryer` `carried`, 8 degrees: 42 mm, 0.8 mm at 1:50). See section 6.

Rule `cover` adds what the view hides at rest and shows when the piece tilts, as the elevation profile swept laterally: the carcass and the legs under a top in the plan, the rear feet behind the front feet in the front. Without them the tilted plan had no body where the foot contacts lie.

| Asset | Pose and tilt | Shapes made solid | Covered parts added | Else |
|---|---|---|---|---|
| `cafe-chair` | `tilted-on-two-legs`, root 12° | 48 | plan 1:500: the two front legs as strokes (under the seat at rest) | twins by `-0`, `-1` |
| `used-park-armchair` | `dragged`, root 20° | 126 | none | right parts take the left elevation twin; legs with `zTip` |
| `used-bedside-table` | `carried` 12° | 51 | plan: carcass, four legs; front: rear legs (copy of the tapered front legs) | elevation 1:100 gets the drawer knob that its plan draws (26 mm in `drawer-open`) |
| `used-high-chair` | `carried` 8° | 77 | none | |
| `used-hinged-wardrobe` | `carried` 5° | 24 | front 1:50, 1:100, 1:200: the left side panel behind the right one | |
| `used-recliner-chair` | `carried` 6°, `reclined` back 35° | 54 | front: rear glides | front back cushion: span of the sweep fitted to the elevation outline (`../MR/fit.ts`, `front-sweep`, range kept) at 1:50, 1:100, 1:200; at 1:500, where it is a plain rectangle, the elevation profile swept laterally |
| `used-wingback-armchair` | `carried` 55° | 10 | none | |
| `used-dishwasher` | `carried` 6° | 47 | none | plan 1:200: the L-shaped hose bundle becomes two solids, coil and aquastop, each with its own height (one prism over the full height was 21 mm too long when tilted); tapes lie on their coils |
| `used-electric-cooker` | `carried` 4° | 110 | front 1:50, 1:100: rear feet | front casing takes the side as its twin |
| `used-fridge-freezer` | `carried` on the hand truck 36.4°, truck frame 38° | 109 | front 1:50, 1:100: rear rollers | front and plan 1:500 get the top bar of the hand truck that 1:200 draws (the grips lie on it) |
| `used-microwave-oven` | `carried` 4° | 45 | none | |
| `used-tumble-dryer` | `carried` 8° | 56 | plan: cabinet and base under the top cover | |
| `used-washing-machine` | `carried` 20° | 76 | none | 1:500: the block reaches to the front of the fascia (0.55 to 0.58 m) in elevation and plan, see 4.4 |

### 4.2 Lumped contacts and anchors without `z` (D45), 5 assets

| Asset | Before | After |
|---|---|---|
| `electric-fan-heater` | `foot-back`, `foot-front`, no `z`, each standing for two feet | `foot-back-right`, `foot-back-left`, `foot-front-right`, `foot-front-left` at z ∓0.075; the support pads of the four volumes are one per foot (1:50, 1:100 a copy of the foot solid per foot, 1:200 and 1:500 a box of 0.03 m per foot); assumption text says four feet |
| `used-electric-guitar-and-combo` | `stand-feet`, one anchor at the stand axis for the three tripod feet | `stand-foot-left`, `stand-foot-front`, `stand-foot-rear` at radius 0.27 m, 120° apart, footprint 32 mm each; one pad per foot (the repeated pad written out with its rotation) |
| `used-electronic-drum-kit` | `kick-tower-base`, no `z`, between the two rails | `kick-tower-rail-r`, `kick-tower-rail-l` at z ∓0.18, footprint 0.32 × 0.03 m; one pad per rail |
| `used-electronic-organ` | `bench-feet`, one anchor in the middle of the bench for four feet | `bench-foot-back-right`, `-back-left`, `-front-right`, `-front-left` at the four legs (v ±0.125, z -0.39 and 0.33), footprint 30 × 40 mm; one pad per foot |
| `gas-hob` | emitter `flame-top` without `z` while the burner is drawn 30 mm left of the centre plane | `z: 0.03`; the front of 1:50 gets the central flame tongue the side elevation draws (it showed the two side tongues only), so the tip lies on a drawn flame |

Poses, clips and `structure.supports` name the new contacts in place of the lumped one. No fixture under `S/🧫️fixtures` names any of these anchors.

### 4.3 A coarse band that lacked a part (rest pose)

- `used-kitchen-base-cabinet` 1:500: the front had no blanket, which side elevation and plan draw; in `carried` (lift 0.55 m) the front began 562 mm above the floor. The blanket of 1:200 is added to the front (pen 0.2 mm like the elevation of that band). Silhouette front 0.9848 / 14.1 mm before, 1.0000 / 4.7 mm after.
- `office-swivel-chair`: the plan of the backrest shell and pad is a thin sheet swept along v whose outline was drawn along the bone axis (u 0.06 to 0.73 m) although the shell is 0.50 m long and curved; reclined by 18° it reached 42 mm further back than the side elevation. The sheets are laid along the chord of the shell between the two points their ends show at rest (`zTip`), outline at rest unchanged (0 outlines differ), reclined within 13 mm. Replacing the sheet by the five lateral strips of the elevation was tried first and not kept: the plan became a stepped outline.

### 4.4 Decisions worth a second look

- `used-washing-machine` 1:500: the emitter `drawer-vent` lies in the fascia, 25 mm ahead of the block (tolerance 20 mm). The block was widened by the fascia (30 mm, 0.06 mm on paper) instead of moving the emitter. Silhouette elevation 1:500: 1.0000 / 0.0 mm before, 0.9565 / 31.1 mm after (gate 0.92 and 2 features); the 3D body of that band was not changed.
- `gas-hob`: the emitter moved 30 mm sideways to the burner. Thermal and flow tests of the asset pass (section 3).

### 4.5 Paint order of a tilted carcass with doors

The paint order of a pose follows the nearest point of each shape. A carcass made one full-depth solid in the front comes forward at its foot when tilted back and was then painted over the door that hangs higher: `used-fridge-freezer` `carried`, 1:200, left 65 mm of the fridge door visible (found as a print failure at 1:350). The front carcass of the fridge is therefore three solids (plinth zone, behind the freezer door, behind the fridge door; `cuts` of the spec), the two behind the doors 2 mm narrower so they stay hidden at rest. The same defect of the engine is filed in `requests.md` by another fixer (`shapeDepthIn`).

## 5. Files

Updated, 20 catalog documents `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/<id>/🔣️.json` (cafe-chair, office-swivel-chair, used-bedside-table, used-high-chair, used-hinged-wardrobe, used-kitchen-base-cabinet, used-park-armchair, used-recliner-chair, used-wingback-armchair) and `…/🔌️appliances/<id>/🔣️.json` (electric-fan-heater, gas-hob, used-dishwasher, used-electric-cooker, used-electric-guitar-and-combo, used-electronic-drum-kit, used-electronic-organ, used-fridge-freezer, used-microwave-oven, used-tumble-dryer, used-washing-machine).

Created under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `multiview-MF.md`; `🧰️matrix/MF/` with `fix.ts` (the repair), `apply.ts`, `files.ts`, `validate.ts`, `cells.py`, `dump.py` (copies from MB-2), `print.ts`, `restdiff.ts`, `apply.sh`, `redo.sh`, `verify.sh`, `prove.sh`, `matrix.sh`, `vt.sh`, `t.sh`, `run.sh`, `look.sh`, `d.sh`, `rig.py`, `ids.py`, `where.py`, `groups.py`, `table.py` and `spec/<id>.json` (20).

Removed: `🗑️generated/matrix/MF` (backups, trial copies, logs, sheets, matrix folders). `requests.md`: nothing added (the paint-order defect of 4.5 is already filed there by another fixer).

## 6. Open

Four assets are not clean in the matrix. Measured with `../MR/gate.ts` (poses, clip instants, transitions) before and after:

| Asset | Hard findings before | After | What is left |
|---|---|---|---|
| `cafe-chair` | 37 | 3 | `sync-height` 1:50, clip `tip-back` at 1.748 s: 18.3 mm against a tolerance of 18 mm (front 19 mm higher than the elevation). Legs and back posts that lean more than their thickness stay flat in the plan of 1:50. |
| `office-swivel-chair` | 134 | 106 | `sync-lateral` 29 to 32 mm (tolerance 20) at all four scales while the seat swivels (transitions and clip `swivel-rotation`), and `sync-forward` 20 mm at 1:200 and 1:500 in that clip. Unchanged by this batch; the swivel was not worked on. |
| `used-fridge-freezer` | 1,019 | 15 | `sync-forward` 83 to 90 mm (tolerance 37) at 1:50, 1:100, 1:200 in the transition `carried>stored` at 0.5 (plan reaches further back than the elevation). Cause not found. |
| `used-tumble-dryer` | 169 | 6 | `sync-lateral` 20 mm (tolerance 18) at 1:50 and 1:100 in clip `clean-filter` at 2.706 s, door open. Same number as before the batch; the door solids were not changed. |

Also open:

- Open strokes on a tilting bone stay flat (4.1). The clean remedy needs a solid form for a stroke or a face bone; neither was introduced.
- `used-washing-machine` `carried`, front: the two rear feet show below the body with a gap, because the front has no body solid under the front panel. A full-depth body was not added: with the paint-order rule (4.5) it would be painted over the door.
- Sheets of 15 assets and of the coarser bands were not opened (section 3).
- `used-hinged-wardrobe` does not print at 1:20, as before.
