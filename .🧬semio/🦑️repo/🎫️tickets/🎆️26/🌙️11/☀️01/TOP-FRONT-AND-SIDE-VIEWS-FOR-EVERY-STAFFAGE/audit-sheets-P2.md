# 🔍️ Audit Sheets P2: Appliances (read-only)

Verdict: CONDITIONAL. Not clean for sign-off.

- Passed: 288 view sheets rendered (102 at 1:50, 102 at 1:100, 84 at 1:200), each with side, front and plan present and non-empty. Extents match the height, width and depth labels for every asset and scale. Moving parts (vacuum wand and hose, toaster carriage, split-AC vane, laptop lid, MVHR fan, server-rack LEDs) follow the pose in front and plan.
- Blocking: laptop 1:50 and vacuum-cleaner 1:100 and 1:200 have no views. They render as explanation sheets because the 0.20 mm feature check fails (laptop base-lower 0.18 mm and plan-hinge-shoulder 0.19 mm; vacuum hose-9-tube 0.19 mm, hose-5-tube 0.18 mm, hose-3-tube 0.19 mm). The round-4 claim that the office-laser check is ok for all 12 assets is contradicted by this render. The explanation text also lists the non-printable scales under "Available scales".
- For the author: thermostat, TRV and shower head carry a ground-role line through the body that reads like a cable or pipe; the shower hose sits about 0.3 m below the ground datum. Intent not confirmed.
- Not confirmed at montage size: dial, knob and light-state changes (room thermostat, TRV, pendant lamp, shower head, hot-water cylinder).
- Process note: the render process printed `done` and then exited with status 1; the cause was not investigated (the two check failures are the likely source).

Scope: 12 P2 appliances (hot-water-cylinder, kitchen-extractor-hood, laptop, mvhr-unit, pendant-lamp, room-thermostat, server-rack, shower-head, thermostatic-radiator-valve, toaster, vacuum-cleaner, split-ac-indoor-unit). Driver: `T/🧰️P2/render.ts`, mode `all`, scales 1:50, 1:100, 1:200. Output: `T/🗑️generated/R-P2`.

Method: all poses and clips at 0, 50 and 100 percent of each clip duration, every authored scale. Each sheet holds side, front and plan. Montages: `T/🧰️P2/montage.py`, one PNG per asset and scale, read on the Read tool.

## hot-water-cylinder

- 1:50: 9 instants (charge-up@0/450/900, cold, charged, draw-off@0/300/600, half-charged). Side, front and plan present and non-empty in all 9. Height 1.48 m and plan diameter 0.60 m match the dimension labels. Caption fits. The dial and the charge state change only slightly between instants; at this resolution the dial pointer is hard to resolve, so the dial pose is not confirmed in detail.
- 1:100: 9 instants, three views each, same extents. Caption fits.
- 1:200: 9 instants, three views each, same extents. Plan label "0.60 m" sits close to the dimension tick at this scale (possible touch, not checked at full resolution).
- Findings: no defect in the three-view presence, extents or pose-following. Dial state changes are not verifiable at montage resolution.

## kitchen-extractor-hood

- 1:50: 9 instants (extracting, low, off, step-down-and-off@0/150/300, switch-on-warm-up@0/300/600). Three views present and non-empty in all 9. Height 0.92 m and depth 0.90 m match the labels. The canopy skirt changes between step-down and off poses in side and front views, so the pose is followed; the change is subtle at montage size. Caption fits.
- 1:100: 9 instants, three views each, same extents. Caption fits.
- 1:200: 9 instants, three views each, but the hood is small in the sheet. The "0.92 m" and "0.90 m" labels sit against the dimension ticks (touching, not overrun).
- Findings: no defect in presence, extents or pose-following.

## laptop

- 1:50: no views. Only `laptop--1-50--explanation.svg` ("Not printable at 1:50 (office-laser)", base-lower 0.18 mm and plan-hinge-shoulder 0.19 mm, both below 0.20 mm). The explanation lists "Available scales: 1:200, 1:100, 1:50", which includes the scale it declares not printable. Finding: the report says the office-laser check is ok for all 12 assets; this render contradicts it. The 1:50 gap is a visible gap, not a defect in the drawing.
- 1:100: 9 instants (closed, open-idle, open, open-lid@0/1.2/2.4, work-and-sleep@0/300/600). Three views present and non-empty. The lid rises from flat (0.02 m at lid@0 and closed) through 0.18 m at lid@1.2 to 0.22 m open; the side and front views follow, and the plan keeps the 0.31 m depth. Captions fit.
- 1:200: same 9 instants, three views each, same extents (0.22 m open, 0.02 m closed). The plan shows the lid as a thin outline and the "0.31 m" label sits against the tick. Captions fit.
- Findings: no defect in the three views or pose-following at 1:100 and 1:200. Missing 1:50 views conflict with the report's check claim.

## mvhr-unit

- 1:50: 12 instants (boost, boost-spin@0/3/6, day-cycle@0/1800/3600, normal, normal-spin@0/1.5/3, off). Three views present and non-empty in all 12. Height 0.66 m, width 0.60 m, depth 0.30 m match the labels. The fan rotation changes between spin instants in side view, and the four spigots on top and the front ports show in plan. Captions fit.
- 1:100: 12 instants, three views each, same extents. Front view has fewer details (front beads dropped, as the report states). Spin visible in side view.
- 1:200: 12 instants, three views each. Height 0.67 m (print gap for spigots, as the report states). Spigot circles in plan are very small but present. Plan "0.30 m" label sits above its tick. Captions fit.
- Findings: no defect in presence, extents or pose-following. The spigot marks at 1:200 are minimal.

## pendant-lamp

- 1:50: 9 instants (dimmed, on, off, switch-off-cool-down@0/150/300, switch-on-warm-up@0/150/300). Three views present and non-empty in all 9. Height 0.80 m and shade diameter 0.27 m match the labels. Caption fits.
- 1:100: 9 instants, three views each, same extents. Caption fits.
- 1:200: 9 instants, three views each. The lamp is small; the cord and shade are present in side, front and plan. Captions fit.
- Findings: no defect in presence or extents. The only moving state is the light (on, off, warm-up, cool-down), which is hard to tell apart at montage size, so the state change is not confirmed in detail.

## room-thermostat

- 1:50: 9 instants (calling, heating-cycle@0/150/300, idle, set-high, turn-up@0/3/6). Three views present and non-empty in all 9. Width and height 0.09 m and depth 0.03 m match the labels. The dial and the wall plate are visible in side view. Captions fit.
- 1:100: 9 instants, three views each. The body is thickened to 0.04 m depth in plan (hand-picked, per the report).
- Visual finding (zoomed one sheet at 1:50, idle): a thin horizontal line about 0.35 m (side) to 0.6 m (front) long runs out from the thermostat body in every view. It is ground-role, so it is excluded from the extents. A reader can take it for a cable or a pipe. Not confirmed as an intended wall reference; flagged for the author.
- No overrun, no missing view. Pose changes (calling, heating-cycle, turn-up) are not visible at montage size in the dial glyph.

## server-rack

- 1:50: 9 instants (full-load, idle, load-ramp@0/30/60, off, power-up@0/150/300). Three views present and non-empty in all 9. Height 2.00 m, footprint 0.60 m by 1.00 m match the labels. The status LED and vent band change with load and power state in side view, so the pose is followed. Captions fit.
- 1:100: 9 instants, three views each, same extents. The LED pattern is less dense at this scale but still changes.
- 1:200: 9 instants, three views each; small but complete. Captions fit.
- Note on convention: the front view is 1.00 m wide because the front axis is the depth (front.x = plan.y); this holds for every asset and matches `render.ts` checks.
- Findings: no defect.

## shower-head

- 1:50: 9 instants (eco, eco-and-off@0/210/420, off, running, turn-on-and-run@0/150/300). Three views present and non-empty in all 9. Height 1.11 m, depth 0.20 m, width 0.23 m match the labels. The hose loop is visible in side and front; the head sits at the top of the riser. Captions fit.
- Visual finding (side and front, all scales): the extents run y -0.299 to 0.81 m. The horizontal ground datum is drawn about 0.3 m above the lowest point, so the hose tail sits below the ground line in every scale. If the datum is the floor this is a placement defect; if it is a bracket reference it is intentional. Flagged for the author to confirm.
- 1:100: 9 instants, three views each; same datum behaviour. Captions fit.
- 1:200: 9 instants; height 1.11 m, but off, eco-and-off@420 and turn-on-and-run@0 read 1.12 m (0.01 m, within the 0.034 m tolerance). Captions fit.
- Pose changes (eco, off, running) are not clearly visible at montage size.

## thermostatic-radiator-valve

- 1:50: 9 instants (close-down@0/4/8, closed, open, open-up@0/4/8, part-open). Three views present and non-empty in all 9. Height 0.15 m and body 0.06 m by 0.09 m match the labels (the pointer was shortened per the report; the side view reads consistently).
- 1:100: 9 instants, three views each. Plan depth reads 0.09 m (print gap against the 0.10 m label at 1:50, as expected for a fine-feature scale). Captions fit.
- Same horizontal ground line through the body as the room thermostat and shower head, about 1 m long at 1:50, excluded from extents. Same datum question for the author.
- Setting-knob and stem position changes between closed and open are not distinguishable at montage size, so the pose change is not confirmed in detail.
- 1:200: not authored for this asset (report and render).

## toaster

- 1:50: 9 instants (cool-down@0/300/600, idle, popped, toast-cycle@0/150/300, toasting). Three views present and non-empty in all 9. The carriage moves: side-view height is 0.24 m when popped or at cool-down@0 and toast-cycle@300 and 0.19 m at idle or cool-down@600, and the slices show above the body in the front view when popped. Footprint 0.30 m by 0.17 m matches the labels. Captions fit.
- 1:100: 9 instants, three views each, same carriage behaviour. Captions fit.
- 1:200: 9 instants, three views each; small but complete. Captions fit.
- Findings: no defect. The carriage follows the pose in front and plan.

## vacuum-cleaner

- 1:50: 9 instants (floor-stroke@0/2/4, parked, run-and-park@0/750/1500, running, running-pulled). Three views present and non-empty in all 9. Wand and hose follow the pose in side view: height 0.76 m for floor-stroke@0/4 and running, 0.81 m for floor-stroke@2 and running-pulled, 0.83 m for parked and run-and-park. The nozzle sits at the floor in floor-stroke and running, and the wand enters the nozzle in plan. Depth 0.30 m matches. Captions fit.
- 1:100 and 1:200: no views. Only explanation sheets (`vacuum-cleaner--1-100--explanation`, `--1-200--explanation`): hose-9-tube at 0.19 mm (1:100) and hose-5-tube and hose-3-tube at 0.18 and 0.19 mm (1:200), below the 0.20 mm limit. The explanation text lists 1:100 under "Available scales" while declaring it not printable. Same inconsistency as the laptop sheet.
- Finding: the report's Round 4 claim that every asset and scale is ok is contradicted by this render (check FAIL for vacuum-cleaner at 1:100 and 1:200). The plan nozzle overlap fix is visible at 1:50.

## split-ac-indoor-unit

- 1:50: 9 instants (cooling, cooling-low, off, start-cooling@0/300/600, vane-swing@0/3/6). Three views present and non-empty in all 9. The side-view vane flap changes with the swing: height 0.29 m off, 0.32 to 0.35 m cooling and start-cooling, 0.33 m vane-swing@0/6, 0.38 m vane-swing@3. The front vane follows the same angle, and the plan keeps 0.85 m depth. Captions fit.
- 1:100: 9 instants, three views each, same vane behaviour (0.29 to 0.37 m). Captions fit.
- 1:200: 9 instants, three views each (0.30 to 0.39 m). Captions fit.
- Findings: no defect. The vane follows the pose in front and plan at every scale.

## Cited PNGs and cleanup

- `T/🗑️generated/R-P2-cited` holds the five cited montages and one zoom: vacuum-cleaner-1-50.png, toaster-1-50.png, split-ac-indoor-unit-1-50.png, shower-head-1-50.png, room-thermostat-idle-1-50-zoom.png.
- `T/🗑️generated/R-P2` (render output, montages, logs) deleted after the pass, as requested. Rendered SVGs were not kept.
- Catalog and source files: untouched (`git status` count for the staffage folder is 295 before and after the pass).
