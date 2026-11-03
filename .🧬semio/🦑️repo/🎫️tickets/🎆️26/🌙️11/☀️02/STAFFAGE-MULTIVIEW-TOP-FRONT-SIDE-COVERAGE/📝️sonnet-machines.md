# 📝️ sonnet-machines: machinery 16 + vehicles 9

Scripts: `T/🛠️scripts/machines/` (`validate.ts`, `guard.ts`, `extents.ts`, `follow.ts`, `shapes.ts`, `symbols.ts`, `solidify.ts`, `excavator-solids.ts`, `render*.ts`).
Proof renders: `T/🗑️generated/machines/<asset>/` (default pose, every authored band, three views), `symbols/`, `after/`, `excavator-slew/`, `diesel-plume/`.
Backups of files before the lateral-ambiguous conversion: `T/🗑️generated/machines/backup/`.

## Result against `validateAsset` (twin rule removed, D25/D28)

| Stage | Issues in the 25 assets |
|---|---|
| Start of work (symbols, cause, extents) | 31 |
| After my fixes | 0 (all codes except the D28 backlog below) |
| D28 `lateral-ambiguous` backlog | cargo-bike, city-bicycle, electric-kick-scooter, motor-scooter, delivery-van, tower-crane, excavator: converted and re-validated to 0 except tower-crane 2 (slings, tower-head diagonals, see open issues). concrete-mixer-truck converted and written after the D31 pass (8 axis entries intact) |

Existing guard (`three-views` sweep and consistency, every pose and 6 clip instants, all bands): 0 findings in the batch. The shared tolerance of that guard is 3 % of the largest dimension (2.15 m for the crane), so I added my own checks: per-bone centre-of-extent following (`follow.ts`, elevation dx against plan dx and front dy, tolerance 0.05 m), the validator extent rule (2 % or 15 mm) at the default pose for every band, and wheel-axle and track positions.

## Fixes made (catalog files)

1. `cause: "below-printable-feature"` on the 10 unavailable bands (humidifier 500, expansion-valve 200/500, finned-hx 500, plate-hx 500, scissor-lift 500, scroll-compressor 500, solar-collector 500 replaced, sorption-wheel 500, city-bicycle 500, kick-scooter 500).
2. Nine true-extent symbols authored in all three views (bone = rig root, model metres, extents within 2 %): `bicycle-plan`, `van-plan`, `kick-scooter-plan`, heat-pump `site-plan-footprint`, `scooter-plan`, scissor-lift `site-plan-footprint`, crane `site-plan-reach`, `tram-plan`, loader `articulated-steering-plan`. D39: ids renamed to view-neutral `bicycle-symbol`, `van-symbol`, `kick-scooter-symbol`, `heat-pump-symbol`, `scooter-symbol`, `scissor-lift-symbol`, `tower-crane-symbol`, `tram-symbol`, `wheel-loader-symbol` (alternatives updated; no other references outside the catalog and its manifest; the manifest rows are for registry).
3. Extents: mixer 1:50/1:100 mirror arms 1.35 -> 1.275 (front and plan, depth 2.55); van 1:200/1:500 and car 1:200 mirrors added in front and plan (depth 1.99, 1.68); car 1:500 within tolerance.
4. Solar collector: new authored 1:500 band in all three views (D17), manifest scale 500 `unavailable` -> `authored`, validRange 350..1000. Scissor-lift 1:500 reason reworded (true-extent symbol is offered, no overstated "every profile").
5. Excavator: slew-ring bone (vertical axis, under the undercarriage) with the upper structure on it. Clip `dump-and-stow` slews 0 -> 90 -> 0 degrees (`twistDeg` keys). All shapes of the slewing assembly are extruded solids in every view (elevation keeps its profile; front/plan profiles taken from the elevation twin, box profiles synthesised for 25 parts that exist in one view only). New front counterweight shape. Verified: at 90 degrees the boom lies along z in plan and front (render `excavator-slew/…@2.4`).
6. Diesel generator: exhaust emitter anchor moved from the 90 degree `stack` bone to `container` (-1.2, 3.47) with `z -0.9` (right side): `directionDeg 90` is bone-relative in the new emitter frame, so on the vertical `stack` bone it pointed backwards. Plume arrow is vertical in side and front, a dot at the stack in plan (render `diesel-plume`).
7. Anchor `z` (lateral side) added: mixer stack -1.1, excavator stack -0.9, wheel-loader stack -0.9, city-car tailpipe -0.30, vintage-car tailpipe -0.35, heat-pump fan outlet +0.12.
8. Tram: `hvac-vent` and `floor-aisle-mid` on `body-mid` were authored in world coordinates on a bone rotated by 180 degrees (the vent sat 3.3 m under the ground). Now local (3.5, -3.32) and (3.5, -0.15), world (-3.5, 3.62) and (-3.5, 0.45).
9. D28 conversion (`solidify.ts`): shapes on twisted or spread bones whose paper error exceeds 0.2 mm made extruded solids (front/plan profile from the same-id elevation twin, else a box synthesised from the best matching shape of the other view). Open polylines in the elevation become 0.01 m or pen-width quads (an open line has no area, as a solid it vanished); diagonal open lines are left flat in front and plan (as a solid they draw their bounding rectangle).

## Per-asset table

Bands: A = authored, U = unavailable (alternatives), S = symbol. All authored bands have elevation, front and plan.

| Asset | Bands | Symbols / alternatives | Dims (default pose, all bands) | Articulation follows (plan + front) | Asymmetry and anchors |
|---|---|---|---|---|---|
| concrete-mixer-truck | A 50 100 200 500 | none | OK after mirror fix | chute-swivel 30 degrees and sweep clip OK; drum revolved on its own bone; 4 wheel pairs, track 2.22 both views | exhaust right (z -1.1, anchor z set), cab 2.4 m wide |
| diesel-generator | A all | none | OK | static | exhaust right z -0.9, plume vertical, radiator outlet +x |
| evaporative-humidifier | A 50-200, U 500 (below-printable-feature) | alt scale 200, 100 | OK | drip clip | none |
| excavator | A all | none | OK | boom, stick, bucket, cylinders; slew 0..90 in dump-and-stow now follows in all views | cab left (z 0.1..1.25), exhaust right, counterweight rear |
| expansion-valve | A 50 100, U 200 500 | alt scale 100/50 | OK | opening clip | none |
| finned-heat-exchanger | A 50-200, U 500 | alt scale 200/100 | OK | none | none |
| forklift | A all | none | OK | mast tilt, lift, tines forward 0.98 m in plan and front for every pose and clip (audit finding on the tines was a false positive: the forks bone points up at rest), wheels at track 1.0 | none |
| heat-pump-outdoor-unit | A all | symbol | OK | defrost, vane | fan offset left (z +0.12), vane z -0.24..0.48 |
| oil-pumpjack | A all | none | OK | crank, pitman, beam, horsehead follow in plan and front (counterweights on both sides) | belt guard right |
| plate-heat-exchanger | A 50-200, U 500 | alt scale 200/100 | OK | none | none |
| scissor-lift | A 50 100 200, U 500 | symbol alternative (3 views, D27) | OK | raise cycle: arms in plan/front follow | none |
| scroll-compressor | A 50-200, U 500 | alt scale 200/100 | OK | spin | none |
| solar-thermal-collector | A 50 100 200 500 (new) | none | OK | none | none |
| sorption-wheel | A 50-200, U 500 | alt scale 200/100 | OK | spin | none |
| tower-crane | A all | symbol | OK; slew test at 90 degrees: jib along z in plan and front at 1:50..1:500 | turntable slew, trolley, hoist, skip follow; all slewing shapes solids | cab on the right (z -1..0) rotates with the slew |
| wheel-loader | A all | symbol (straight) | OK | lift arm, bucket; wheels at track 2.2 in both views | exhaust right z -0.9 |
| cargo-bike | A all | none | OK | steering (fork, bar, wheel) now solids; front wheel turned in plan | unmanned (rider seat load only, no grips anchors by design) |
| city-bicycle | A 50 100 200, U 500 | symbol bicycle-plan (3 views, D27) | OK | steering and wheel solids | unmanned |
| city-bus | A all | none | OK | doors, kneel | doors right (z -1.27), both mirrors |
| city-car | A all | none | OK | hatch, bonnet | one near mirror in elevation = right (seen from the right), tailpipe right z -0.30; driver side not drawn (steering wheel is under the roof in plan/front) |
| delivery-van | A all | symbol | OK after mirror fix | rear doors twist +-90 (solids); no steering bone and no turning clip: straight by design | sliding door right (z -0.91), tailpipe z not set (no plan shape to follow) |
| electric-kick-scooter | A 50 100 200, U 500 (2.2 mm, truthful: below-printable-feature) | symbol | OK | fork steering solids | none |
| motor-scooter | A all | symbol | OK | steering (turning-left) solids; no rider, no grip anchors by design | tailpipe z unset |
| tram | A all | symbol | OK | bellows, doors right (z -1.2), pantograph, bogies | anchors on body-mid fixed |
| vintage-car-1929 | A all | none | OK | steering none | tailpipe right z -0.35 |

## Added after the first report

- **Per-wheel contact anchors** (`contacts.ts`): city-car, vintage-car-1929, delivery-van, city-bus, tram (8 patches at the wheelset x, z +-0.715), concrete-mixer-truck, wheel-loader, forklift (front +-0.5, rear +-0.24), scissor-lift. Each axle anchor is now `<id>-l` (z +) and `<id>-r` (z -) with the half footprint width; pose and clip `contacts` lists updated; 0 validator issues, guard clean. Not split: excavator tracks, generator skids, pumpjack skids (4 patches by their own assumption text), bicycles and scooters (single track).
- **Tailpipes**: van `tailpipe` anchor z -0.31 and plan `exhaust-tip` added at 1:100 and 1:200 (1:50 had it); scooter anchor z -0.15 (muffler and pipe already drawn at that z in plan).
- **City-car driver side** (left-hand drive, D6): 1:50 front gets the steering wheel at z +0.29 and plan gets the steering wheel and both front headrests (render `after/city-car…`: wheel on the left, up the page in plan, right of centre in front). Elevation keeps the far steering wheel seen through the right-hand glass. Closed-body bands at 1:100 and coarser have no interior.
- **glazing-hatch** (new validator rule): hatch removed from 6 glazing shapes of city-bus and city-car.
- **D28 backlog**: my section of `📝️sonnet-contracts-backlog.md` lists only the 2 tower-crane items (mixer chute entries were already fixed after that run). D37 `member` solids will replace the left-flat diagonals (crane slings, tower-head, 5 plan lines) when they land.
- **Inspected renders of D28-converted assets** (`T/🗑️generated/machines/after/`): city-bicycle parked-on-stand (steer -15), motor-scooter turning-left, electric-kick-scooter braking-stop, delivery-van parked, concrete-mixer-truck chute-swing at 1:100, plus cargo-bike turning-left, tower-crane parked-weathervane (1:200 and 1:500), excavator dump-and-stow @0, @1.2, @2.4. In all of them the steered or slewed parts turn in plan and front together with the side view.

## D37 members (tower-crane)

`members.ts`: tower-head legs (4, d 0.10), lattice diagonals (16, d 0.06, tapered with the legs), base frame (4) and base cross (2) at 1:50, 1:100 and 1:200, and 4 slings (d 0.02) at all four bands replace the flat diagonal lines in elevation, front and plan. Renders at rest (lifting-load) and slewed 60 degrees (parked-weathervane) at 1:50, 1:100, 1:200, 1:500 in `🗑️generated/machines/after/` and `members-preview/`: the lattice and slings turn with the slew in all three views. Backup: `backup/tower-crane-before-members.json`. Electric kick-scooter `bms-controller` confirmed hidden in every view (deck is a closed box); the region's `hidden` text now says so for all three views.

## Open issues

- none: `lateral-ambiguous` = 0 and `validateAsset` total 0 for the batch.
- concrete-mixer-truck: converted and written (8 shapes), 0 issues, `axis: u` entries of D31 preserved.
- The three-views guard tolerance is 3 % of the largest dimension; consider a tighter per-asset rule.
- Manifest changes for registry: solar-thermal-collector scale 500 unavailable -> authored; symbol ids unchanged.
