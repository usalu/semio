# 🪑️ Multiview Batch MF: 9 Furniture and 11 Appliances (2026-11-24)

Fixer report. Batch `🧰️matrix/batches/MF.tsv`, brief `📋️multiview-brief.md`, paths `S`, `T`, `U` as there. Tools and specs: `🧰️matrix/MF/`. Measured with the authors' gate `T/🔎️scan.ts` and confirmed with `T/📊️matrix.ts --jobs 1 --assets <id>`.

## 1. Result

RESULT-PLACEHOLDER

## 2. Per asset: findings and matrix

TABLE-PLACEHOLDER

## 3. Proof

PROOF-PLACEHOLDER

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

## 5. Files

FILES-PLACEHOLDER

## 6. Open

OPEN-PLACEHOLDER
