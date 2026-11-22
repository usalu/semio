# Round 3 — machinery/scissor-lift

State: DONE (2026-11-19). Scripts: `r3-scissor-lift-kit.py` (dimensions, linkage kinematics, part helpers), `r3-scissor-lift-s50.py`, `-s100.py`, `-s200.py` (hand-stated parts per scale), `r3-scissor-lift-roots.py` (rig, dimensions, anchors, poses, clips, symbol, areas), `r3-scissor-lift-texts.json` (2D decisions), `r3-scissor-lift-build.py` (candidate under `🗑️generated/r3-scissor-lift/`, volumes source, `--apply` writes the root members into the catalog file; then `volumes-insert.py`). Resume: `python3 r3-scissor-lift-build.py`, validate the candidate with `r3-validate.ts --file`, `--apply`, splice.

## 1. Reference

Reference machine: Genie GS-1932 (19 ft electric slab scissor). Source S1 = Genie specification sheet GS-1530/1532/1930/1932, metric, read 2026-11-19 (https://www.colle.eu/media/12834/genie-gs1930_eng.pdf; also https://www.genielift.com/en/aerial-lifts/Slab-scissor-lifts/gs1930). S2 = EN 280 (asset reference `en-280`). K = my knowledge of the construction of this machine class (not a published number).

| quantity | real value | source |
|---|---|---|
| platform height, maximum | 5.79 m | S1 |
| platform height, stowed | 1.00 m | S1 |
| height stowed, rails up | 2.11 m | S1 |
| length stowed / with deck extended | 1.83 m / 2.74 m | S1 |
| width | 0.813 m (GS-1932; GS-1930 0.76 m) | S1 |
| platform length x width outside | 1.63 x 0.74 m | S1 |
| slide-out deck | 0.91 m | S1 |
| guardrail height / toeboard | 1.10 m / 0.15 m | S1, S2 |
| wheelbase | 1.32 m | S1 |
| ground clearance, centre / pothole guards deployed | 0.06 m / 0.02 m | S1 |
| tyres, solid non-marking | 30 x 11 cm (12 x 4.5 x 8 in): diameter 0.305 m, width 0.11 m | S1 |
| lift capacity / on the extension | 227 kg / 113 kg | S1 |
| raise / lower time | 16 s / 29 s | S1 |
| power | 24 V, four 6 V 225 Ah batteries | S1 |
| weight | 1,503 kg | S1 |
| drive | dual front wheel drive, rear multi-disc brakes, zero inside turning radius | S1 |
| scissor stages | 4 (eight arm weldments: outer and inner arm per stage, nested side by side) | K |
| arm section, arm length between end pins | rectangular tube about 90 x 50 x 4 mm, about 1.46 m | K (fits a 1.63 m platform) |
| stage pitch stowed | (1.00 - 0.10 - 0.46) / 4 = 0.11 m | derived from S1 |
| arm angle stowed / raised | asin(0.11 / 1.46) = 4.32 deg / asin((5.79 - 0.10 - 0.46) / (4 x 1.46)) = 63.6 deg | derived from S1 |
| lift cylinder | one single-acting cylinder inside the stack between the first stages, barrel about 100 mm, rod 50 mm, stroke about 0.3 m | K |
| chassis | steel tub about 0.40 m high with swing-out trays between the axles (batteries; hydraulic power unit and ground controls), entry ladder at the rear, pothole guards under the trays | K |
| platform | steel pan on roller channels; slide-out deck rides on top of the main deck and carries the front rail frame inside the main side rails; folding rails of about 40 mm tube; half-height swing gate at the rear; platform controller on the front right rail | S1 (features), K (sizes) |

## 2. Audit of the asset as found (before this round)

| item | found | real | deviation |
|---|---|---|---|
| `dimensions` | 2.20 x 1.85 x 0.81 m, 1750 kg | 2.11 x 1.83 x 0.813 m, 1503 kg | height +0.09 m, mass +247 kg (no machine of the class has them) |
| scissor stages | 5 stages, ten arms of 1.7 m (pins 1.70 m apart on a 1.85 m machine) | 4 stages, arms about 1.46 m | one stage too many; arms longer than the 1.63 m platform |
| arm angle raised | 38 deg | about 64 deg | the raised scissors were drawn as flat diamonds |
| stowed platform floor | 1.10 m | 1.00 m | +0.10 m |
| tyres | diameter 0.54 m, width 0.16 m | 0.305 m, 0.11 m | 77 % too large: the lift read as a wheeled cart |
| wheelbase | 1.10 m | 1.32 m | -0.22 m |
| ground clearance | 0.10 m | 0.06 m | +0.04 m |
| platform length | 1.85 m (as long as the chassis) | 1.63 m | +0.22 m |
| guard rail | 1.10 m above a 0.10 m deck, 50 mm tubes, toe board 0.15 m | 1.10 m, about 40 mm, 0.15 m | tube size only |
| slide-out deck | plate inside the deck, 0.9 m, no rails of its own | rides on top, 0.91 m, carries the front rail frame | wrong construction; the extended deck had no guard rail |
| lift cylinder | absent from the rig; a square in the front view | one cylinder between the first stages | missing bones, missing part |
| rig joints | a1 foot at x -0.85, slide at +0.846, y 0.45; platform bone 1.85 m | pins 1.455 m apart at y 0.46 | all scissor joints misplaced with the arm length |
| clips | two keys between stowed and raised: the tree linkage opens between them (slide 0.07 m off at mid lift) | closed linkage in every frame | kinematic defect |
| clip tempo | raise in 6 s | 16 s raise, 29 s lower | not corrected, see section 7 |
| 2D drawings (all scales, all views) | follow the wrong rig and dimensions; arms as 0.12 m slabs without pivots at 1:50; front and plan constructed, not projected | | redrawn (E1) |
| solids | scissor pack a block of slabs, tyres as free cylinders, structure partly the body (qa-visual-2: body B-/C/C, structure C) | | re-modelled |
| thermal `areaM2` | arms 19.56, tyres 2.508, deck 4.77, chassis 5.885, battery-box 0.751, motor-pump 0.7, rails 6.0 (adopted from the old model's measurements) | see section 3 | follow the wrong object |

## 3. Root fixes (old -> new)

- `dimensions`: heightM 2.2 -> 2.11, widthM 1.85 -> 1.83, depthM 0.81 kept (0.813), massKg 1750 -> 1503 (S1). Bone masses re-divided to sum 1503 kg: chassis 760, wheel pairs 18 + 18, eight arms 38 each, slide 6, cylinder 28, rod 12, platform 270, slide-out deck 87 (authored estimates).
- `provenance`: reference `genie-gs-1932` added, notes rewritten; `summary` rewritten (four stages, eight arms, cylinder).
- `rig`: bones `a5`, `b5` removed; `lift-cylinder` (revolute on b1 at its centre pivot) and `lift-rod` (prismatic) added; arm length 1.7 -> 1.46; a1 foot (-0.85, 0.45) -> (-0.7275, 0.46); slide (0.8464, 0.45) -> (0.7283, 0.46), travel -0.6 -> -0.82; rest arm angle 3.71 -> 4.321 deg; wheels (+-0.55, 0.27) -> (+-0.66, 0.1525), length 0.27 -> 0.1525; platform now on b4, length 1.85 -> 1.63; deck-ext origin at the platform front end, travel 0.9 -> 0.91; joint limits widened to the real 59 deg of arm travel.
- `anchors`: contacts to x +-0.66, z +-0.35; motor outlet (0.925, 0.3) -> (0.915, 0.26); battery top (-0.4, 0.45) -> (-0.24, 0.3925); load points on the new floor level (platform-local y 0.1).
- `poses`: every pose solved from the closed linkage (`offsets()` in the kit): stowed 4.32 deg, mid-raise and lowering 30 deg (floor about 3.5 m), raised-working 63.6 deg (floor 5.79 m, deck out 0.91 m), cylinder angle and rod travel included; descriptions rewritten.
- `clips`: key times, phases, events and tracks kept; between stowed and raised a key every 0.5 s on a smooth lift curve, each solved exactly, so the linkage stays closed (old: open by up to 0.07 m).
- `symbols`: true-extent symbol redrawn to the new extents (same ids).
- `representations` 50/100/200: all three views redrawn, see section 4; decisions rewritten. 500: reason and limiting feature rewritten for four stages (stays unavailable, limiting size 0.11 m unchanged).
- `thermal`: characteristic lengths arms 1.7 -> 1.46, tyres 0.54 -> 0.305; `areaM2` see section 5.
- `thermal.regions[].areaM2` (old -> new): arms 19.56 -> 7.9, chassis 5.885 -> 4.9, battery-box 0.751 -> 0.52, motor-pump 0.7 -> 0.55, deck 4.77 -> 3.95, rails 6.0 kept, tyres 2.508 -> 0.97. Hand calculations and the two changes beyond factor 1.5 (arms, tyres: the old values describe five stages and 0.54 m tyres) are listed for the reviewer in `area-review.md` (2026-11-19 r3 machinery/scissor-lift).

## 4. 2D drawings (ruling E1)

Every part is stated once by hand in the scale's part script and gives both its solid and its 2D shape: the side elevation is the part's profile on its bone, front and plan are the same profile with the schema's `solid: extruded` lateral span, so all three views are projections of one body in every pose (the old front and plan were constructed rectangles carried by the bone origin). Far-side twins and parts hidden behind others are not drawn in a view (per part `views`), order is far to near. Wheels are circles in the side view. Shape counts elevation / front / plan: 1:50 70 / 143 / 147, 1:100 46 / 120 / 120, 1:200 27 / 70 / 70. Print suitability (`r3-scissor-lift-suit.ts`): 0 violations at all three scales; one was found and fixed (1:100 battery vent strip 0.197 mm under the 0.2 mm feature limit in the plan: no longer drawn at 1:100, it stays in the model). Old shape ids could not be kept where the part no longer exists (`arm-a5`, `sleeve-*`, `lift-cylinder-eye`); `arm-*`, `deck`, `deck-ext`, `control-box`, `battery-cover`, `pump-cover`, `chassis-*`, `tyre-*`, `post-*`, `top-rail-*`, `mid-rail-*`, `toe-board-*` are continued.

## 5. Models per scale

| scale | body | thermal | structure | what is modelled |
|---|---|---|---|---|
| 1:50 | 114 | 114 | 93 | chassis tub with chamfered ends and 0.06 m clearance, trays, battery / power-unit / controller doors with handles, ground control panel, vent louvre, battery vent strip, pothole guards, stub axles, scissor track bars and foot lugs, three-rung ladder; tyres as revolved bodies with rounded shoulders, dished rims, hub caps, five wheel nuts each; eight arm weldments (two 90 x 50 tubes each, rounded edges), pivot bosses, 13 pivot tubes, track rollers, cylinder with eye, barrel, gland, port block, rod and rod eye; platform pan on roller channels with end members, joist and pin lugs; slide-out deck on top with its own inboard rail frame; 40 mm rails, toe boards, rear top rail, half-height gate; controller with joystick and emergency stop. Structure: box frame rails, stack rails, cross members, uprights, axle tubes, tyres on rims, four contact pads, hollow arm tubes 90 x 50 x 4, hollow pivot tubes, pins at every joint, cylinder pipe, rod, platform channels, joists, posts and top rails, deck runners, battery / power-unit / platform masses. |
| 1:100 | 85 | 85 | 73 | authored for 15 mm: no handles, panel, pothole guards, nuts, joystick; tyre with chamfered shoulders and one wheel disc; arms plain tubes with boss at foot and centre; cylinder as barrel and rod; platform pan on one beam per side; gate as frame. Structure: frame and stack rails, three cross members, two uprights per side, axle bars, hollow arm tubes, body pivot tubes referenced as members (they are the members), one pin pair per arm foot. |
| 1:200 | 53 | 55 | 43 | authored for 30 mm: one tub block with trays and door slabs, ladder stiles, disc wheels, arm bars 110 mm deep (the stage pitch: closed stack when stowed, crossing bars when raised) with their pivot tubes, cylinder and rod as rounds, deck slab, corner posts, top / mid rails, toe boards, slide-out deck plate with rail frame against the main rails, controller box. Thermal has two thermal-only outlet patches. Structure: solid frame and stack rails, two cross frames, axle bars, arm bar pairs in the real 90 x 50 outline, one pivot shaft per arm foot, platform beams and end members, posts, top rails. |
| 1:500 | - | - | - | unavailable (representation unavailable: stage pitch 0.11 m = 0.22 mm, rails 0.08 mm); texts updated to four stages. |

Thermal lists: every region is the surface of its real parts by `{ of }` reference; each region has a single layer, so there are no nested shells; no hidden regions; emitters are the vent louvre on the front end plate and the vent strip on the right battery tray (tagged; D2). `decisions`: two bilingual paragraphs per scale.

Final check line: `volumes-check.ts machinery/scissor-lift --product`: PASS (40.1 s), every rule at 1:50, 1:100, 1:200. Silhouette IoU elevation / front / plan: 0.997 / 0.998 / 0.999 (1:50), 0.987 / 0.997 / 0.999 (1:100), 1.000 / 1.000 / 1.000 (1:200). Outer areas at 1:50 against declared: arms 7.86 / 7.9, chassis 4.93 / 4.9, battery-box 0.521 / 0.52, motor-pump 0.522 / 0.55, deck 3.95 / 3.95, rails 5.62 / 6.0, tyres 0.943 / 0.97. `r3-validate.ts machinery/scissor-lift`: VALID (schema 0, validateAsset 0), also `--catalog-only`. Spliced with `volumes-insert.py`.

## 6. What I saw (images opened with the Read tool)

- `r3-sheet` 1:50 body, stowed, six views: front: narrow chassis nose between two small tyres, vent louvre, stack of pivot tubes between the arm pairs, platform with rails and the controller top right. Right side: chassis with battery and power-unit doors, handles and control panel, 0.305 m wheels with rim, cap and nuts at the 1.32 m wheelbase, ladder at the rear, four-stage stack of slightly inclined tubes with pivot bosses, platform frame, pan, toe boards, three posts plus the slide-out frame's rear post, controller. Back: ladder with three rungs, gate. Left side: mirror with battery and controller doors. Top: deck with the slide-out deck plate, controller, chassis nose and ladder projecting, tyres at the corners. Three-quarter (also alone at full size): reads at first sight as a 19 ft slab scissor lift; cylinder and pivot tubes visible inside the stack end.
- 1:50 body, pose `raised-working`: linkage closed at 63.6 deg, tall narrow diamonds, platform level at 5.79 m floor (rail top 6.90 m in the 2D dimension), slide-out deck 0.91 m out with its rail frame, rolling feet travelled to the rear on chassis and platform, cylinder extended between stage 1 centre pivot and stage 2 cross tube.
- 1:50 structure: frame rails, stack rails, hollow arms, orange pins at every pivot, masses for batteries and power unit, platform frame and rail frames; no body block anywhere. 1:50 thermal with `--cut`: seven region colours, power-unit bay and battery doors distinct, cylinder inside the stack.
- 1:100 body (six views) and 1:100 thermal (six views), 1:200 body and 1:200 structure (six views): proportions identical, simplifications as listed; the 1:200 stack reads as one mass with the arm edges.
- `r3-2d` 1:50 stowed (candidate and catalog), 1:50 `raised-working`, 1:200 stowed, 1:100 clip `raise-cycle` t 5.25 s; `r3-sheet` 1:100 clip `raise-cycle` t 17.3 s (lowering, about 35 deg): linkage closed, platform level, cylinder connected.
- Rendered but not opened: 1:100 structure sheet, 1:200 thermal sheet, 1:200 `raised-working`, 1:100 stowed 2D after the last fix (it drew without error).

## 7. Still imperfect, stated plainly

- Number of stages (4), arm section and length, stage pitch, cylinder mounting points, chassis and platform construction details are my knowledge of the class, not published numbers; they reproduce the published stowed and raised heights exactly. Which end carries the fixed scissor pins (here: the ladder end, rollers at the front) is an assumption.
- Clip tempo: the raise still takes 6 s and the lowering 5.5 s (real: 16 s and 29 s). Changing it means re-timing phases, events and thermal tracks of both clips; not done.
- The platform mass marker lies in the deck pan under the real centre of mass (0.33 m above the pin, in the air between the rails), because a marker in the air is outside the body silhouette for `volumes-structure-coverage`.
- Not modelled at 1:50: steering knuckles and tie rod, hoses and cables, hinges, gate mesh, tie-down lugs, decals; the pothole guards are shown retracted in every pose (they deploy when the platform rises).
- Guard-rail tubes and arm tubes are closed square-edged bars (arm edges rounded 6 mm); rail tube corner radii are not modelled.
- `arms` and `tyres` `areaM2` were changed by more than factor 1.5 (see `area-review.md`); the reviewer has to confirm.
- 1:200 front and plan keep the pivot tubes of the inner arms (needed for cross-scale agreement with 1:100).

## Round 5 handcraft

Round-5 author (M02), 2026-11-21. Scripts: `r5-machinery-ranges.py` (band ends), `r5-machinery-range.ts` (print suitability walked over the denominators of a band), `r5-scissor-lift-tempo.py` (clip timing). Renders: `🗑️generated/r5-scissor-lift/before/` (every sheet of `r5-sheets.sh`, montages `m1`-`m3`).

### Reference and model against it (1:50)

Reference: Genie GS-1932 metric specification sheet (S1, table in section 1). The rig and the solids are stated from it, so the comparison is exact where the sheet gives the number:

| quantity | reference (S1) | model | deviation |
|---|---|---|---|
| platform height stowed / raised | 1.00 / 5.79 m | 1.00 / 5.79 m (pin top 0.90 / 5.69 + 0.10 deck) | 0 % |
| height stowed with rails | 2.11 m | 2.11 m (`dimensions`, 2D dimension line) | 0 % |
| length / width | 1.83 / 0.813 m | 1.83 / 0.81 m | 0 / -0.4 % |
| platform length / slide-out | 1.63 / 0.91 m | 1.63 / 0.91 m | 0 % |
| wheelbase / tyre diameter | 1.32 / 0.305 m | 1.32 / 0.305 m | 0 % |
| ground clearance | 0.06 m | 0.06 m | 0 % |
| guard rail / toe board | 1.10 / 0.15 m | 1.10 / 0.15 m | 0 % |
| raise / lower time | 16 / 29 s | before 6 / 5.5 s; now 16 / 29 s | fixed |
| arm length, stage pitch, arm angle | K: 1.46 m, 0.11 m, 4.32 to 63.6 degrees | same, closed linkage in every pose and clip frame | - |

### Grades (before -> after; body / thermal / structure)

- 1:50: A- / A- / A- -> A- / A- / A-. Seen: six-view sheets stowed, `mid-raise`, `raised-working`, `lowering`, clip `raise-cycle` (frame at 5.25 s before re-timing); thermal cut; structure; 2D. Linkage closed and platform level in every pose; rails 40 mm, tyres with rim, cap and nuts, ladder, trays, pothole guards, cylinder between stages 1 and 2.
- 1:100: A- / A- / A- (unchanged). 1:200: A- / A- / A- (unchanged). 1:500 (3D only, the 2D is unavailable by design): B+ / A- / A- (wheels plain discs, stack merged; reads as the same lift).
- 2D 1:50, 1:100, 1:200: A- (true projections of the bodies, E1). 1:500: unavailable (`representations.500` reason stands).

### Changes

- Catalog INVALID before (PRINT `range-end-feature` at the 1:200 band end 1:350: `deck-ext` 0.190 mm; then `tyre-rl/rr` 0.196 mm at 1:330, `ext-post-rear` 0.190 mm at 1:300). Walking the band with `r5-machinery-range.ts` showed the 1:200 drawing is suitable at 1:150 to 1:200 but refused at 1:210 (`tube-b4-mid` 0.191 mm) and from 1:260 on, so the band end was set honestly: `representations.200.validRange.maxDenominator` 350 -> 200.
- Clip tempo at the root (`r5-scissor-lift-tempo.py`): both clips re-timed to the data sheet by a piecewise-linear time map over keys, phases, events, tracks and contacts; the solved linkage keys are kept (spacing 1.3 s raising, 2.4 s lowering; between keys the slide error stays about 1 mm). `raise-cycle` 24 -> 57.5 s (raise 3-19 s, work 19-24.5 s, lower 24.5-53.5 s), `start-lift-stop` 22 -> 55.5 s (key off 54.5 s); descriptions en/de rewritten.
- Volumes unchanged (already A-; nothing to remodel that the brief's bar finds wrong).

### Check

`r3-validate.ts machinery/scissor-lift --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0 (after the band fix; re-run after the re-timing below).

### Remaining

- The thermal and structure cut sheets show a speckled cap where a solid block is cut (chassis tub); it is the sheet renderer's cap shading on large solid sections (also on the diesel-generator walls at 1:100/1:200), not a coincident face of the model.
- Number of stages, arm section, cylinder mounting and the platform mass marker in the deck remain class knowledge (round 3, section 7). 1:500 wheels are discs without tyre shoulders (75 mm feature).

The extension deck’s outer edge remains printable across the 1:200 representation’s range through 1:350. The whole-model paper check passed.

## November 21 running gear and moving joints

All eight lifting arms have opposing 6 mm walls, three transverse bores and a 40.5 mm bore in each sleeve for a 40 mm pin. The physical and structural crossing links stay separate throughout the lift. The closed arm and sleeve area measured 14.681–14.727 m² in the four poses. Air reaches 13.907–13.946 m². The working platform and the extension deck retain their separate posts, perimeter rails and gate in the load-path view. See the [measurements and installed revision](digital-running-gear.md).
