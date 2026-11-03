# Audit VE: Visual Check of Three Views for the Nine Vehicles

Scope: cargo-bike, city-bicycle, city-bus, city-car, delivery-van, electric-kick-scooter, motor-scooter, tram, vintage-car-1929.

## Pass 1: montage sheets (round 1)

Render: `T/🧰️QA11/all.ts` (one bun process, 515 jobs) with the job list from `gen_jobs.py` in the since-deleted `T/🗑️generated/R-VE`. Jobs cover every authored scale, every pose and three instants per clip (0.15, 0.5, 0.9 of duration). One SVG per job with three views (elevation, front, plan).
Rasterise: `T/🧰️VE/raster.py` at 150 dpi. Render failures: 0.
Sheets: 98 captioned sheets (2 columns by 3 rows, 800 px tiles). Every sheet was opened.
Consistency in round 1: 34 of 34 ok. Check CLI: 9 of 9 ok.

Round 1 findings at montage resolution:

| Asset | Scale | Pose / instant | View | Finding |
|---|---|---|---|---|
| electric-kick-scooter, city-bicycle | 1:500 | none | all | Gap: no 1:500 representation (unavailable, see zoom section). |
| all nine | all | all | all | Three views present and non-empty on every sheet. No detached or floating part seen at this resolution. |
| all nine | 1:500 | all | all | Caption and dimension text about 3 px high at sheet scale. Low. |
| motor-scooter | 1:50 to 1:200 | turning-left | plan | Front wheel visibly yawed in plan at 1:50 and 1:100, slightly at 1:200. Steering is a real yaw. |
| delivery-van | 1:50 to 1:200 | stop-and-unload@*, stop-out, loop-end | front | Front tile much wider than the van; dimension at right edge. Cosmetic. |
| motor-scooter | 1:50 to 1:200 | parked-stopped, rolled-to-stop, loop-end, stop-and-shut-down | front | Same wide front tile. Cosmetic. |
| electric-kick-scooter | 1:50, 1:100 | loop-end, loop-start, loop-mid | front | Front tile wider than the scooter. Cosmetic. |
| city-car | 1:200, 1:500 | drive-by@4.32, lane-exit, lane-start | front | Front tile wider than the car. Cosmetic. |
| tram, city-bus | all | all | elevation, front, plan | Pantograph and door states change across instants. Nothing detached seen. |
| vintage-car-1929 | all | all | plan | Wheels hidden under body omitted in plan (physically correct). |
| cargo-bike, city-bicycle | 1:50 to 1:200 | all | elevation, front, plan | Present at every instant. |

## Pass 2: zoomed checks (round 2, R-VE2, 600 dpi)

Render: `T/🧰️QA11/all.ts` with 21 jobs (SVG), rasterised with `raster.py` at 600 dpi, then cropped. R-VE2 was verified new before use and is deleted after this pass. Crops are kept in `T/audit-VE-crops/`.
Check CLI on this run reports suitability problems for cargo-bike, city-bicycle and city-car (listed below). Round 1 reported ok, so the catalog has changed since round 1 (cause not traced).

| Asset | Scale | Pose / instant | View | Finding | Crop |
|---|---|---|---|---|---|
| motor-scooter | 1:200 | turning-left | front | Front wheel has a small bar on the tyre crown. It reads as a fender stub, not a full fender arc. Not confirmed as a full fender top. | 01 |
| motor-scooter | 1:200 | turning-left | plan | Front wheel yawed in plan (0.72 m width). Steer confirmed. | n/a |
| motor-scooter | 1:200 | cruising (default) | front | Same stub as turning-left. Not confirmed as a full fender top. | n/a |
| cargo-bike | 1:200 | loaded-on-stand (default) and turning-left | front | No fender arc over the front wheel. A small bracket and a rounded bar sit above the box. Fender top not present in front view. Open. | 02 |
| cargo-bike | 1:200 | turning-left | plan | Steer visible in plan. | n/a |
| cargo-bike | 1:50 | turning-left | plan | Front fork and wheel rotated in plan relative to the frame. Steer confirmed. | 05 |
| cargo-bike | 1:100 | turning-left | plan | Front wheel yawed, same as 1:50. Steer confirmed. | n/a |
| city-bicycle | 1:50 | riding (zero steer) | plan | Front wheel straight in plan. No turning pose is authored, so steer in plan is not demonstrated. | n/a |
| city-bicycle | 1:100 | riding | all | Not printable at 1:100 (office-laser). The render emits an explanation card: arm-f 0.17 mm below 0.20 mm. Steer in plan at 1:100 cannot be shown. | n/a |
| city-car | 1:50 | hatch-open, park-and-open-hatch@4.5 and @8.1 | elevation | Tailgate hinges at the roof edge and is held by a strut that meets the body. Attached. | 03 |
| city-car | 1:50 | bonnet-open | elevation | Bonnet hinges at the windscreen base and lifts forward. Attached. | n/a |
| city-car | 1:50 | hatch-open, park-and-open-hatch | plan | Hatch and bonnet outline shown. Plausible. | n/a |
| delivery-van | 1:50 | side-door-open | elevation, plan | The side door is a sliding leaf, not a hinged door. In the open pose it is a thin vertical track with a rectangle offset from the body edge. It is drawn as a rigid strip that slides on a prismatic bone (round 1 doubt). It does not hinge. Not verified as attached. | 04 |
| delivery-van | 1:50 | rear-doors-open | elevation, plan | Rear door leaf swung out from the rear edge and attached at the body corner. Plausible. | n/a |
| delivery-van | 1:50 | stop-and-unload@1.8, @6, @10.8 | front | Front tile much wider than the van. Cosmetic. | n/a |
| vintage-car-1929 | 1:50 | arrive-and-park@1.2, @4, @7.2 | elevation, front, plan | Doors are closed in every instant. No open-door pose or clip instant to check. Not applicable. | n/a |

Catalog check (electric-kick-scooter and city-bicycle 1:500):
- electric-kick-scooter 1:500: `status` is `unavailable`, with `reason` en and de (length 2.2 mm, below printable feature size). Confirmed.
- city-bicycle 1:500: `status` is `unavailable`, with `reason` en and de (length 3.7 mm, tubes below printable gap). Confirmed.

Check CLI findings on the round-2 run (current catalog):
- cargo-bike 1:500: wheel-r 0.182 mm, below 0.2 mm.
- city-bicycle 1:200: steer-column 0.188 mm, below 0.2 mm. 1:100: arm-f 0.170 mm, below 0.2 mm.
- city-car 1:500: tyre-rl 0.178 mm, tyre-rr 0.180 mm, hatch-panel 0.187 mm, below 0.2 mm.

## Cited crops (5)

1. `T/audit-VE-crops/01-motor-scooter-200-turning-left-front-fender.png`
2. `T/audit-VE-crops/02-cargo-bike-200-turning-left-front-fender.png`
3. `T/audit-VE-crops/03-city-car-50-hatch-open-hinge.png`
4. `T/audit-VE-crops/04-delivery-van-50-side-door-open-track.png`
5. `T/audit-VE-crops/05-cargo-bike-50-turning-left-plan-steer.png`

## Verdict

Verified: the renders are complete and the catalog marks electric-kick-scooter and city-bicycle 1:500 as unavailable with en and de reasons. Steer in plan is verified zoomed for motor-scooter at 1:200 and for cargo-bike at 1:50, 1:100 and 1:200. Motor-scooter 1:50 and 1:100 steer rests on round 1 montage only. City-car hatch and bonnet hinge at their edges. Van rear doors attach at the body corner.

Not verified or failing: motor-scooter has only a fender stub at 1:200, and cargo-bike has no fender arc at 1:200 (both open). The van side door is a sliding rigid strip, not a hinge, and its attachment is not confirmed. City-bicycle steer in plan is not demonstrated (no turning pose; not printable at 1:100). The check CLI now reports suitability problems for cargo-bike, city-bicycle and city-car, which round 1 did not.
