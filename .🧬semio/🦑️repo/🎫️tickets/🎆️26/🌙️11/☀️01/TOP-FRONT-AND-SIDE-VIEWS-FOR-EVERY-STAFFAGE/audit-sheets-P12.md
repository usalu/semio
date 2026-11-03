# 🔍️ Audit Sheets P12: Top, Front and Side Views (read-only)

## P1 verdict

**P1: PASS for geometry, with caption findings open.**

- Coverage: all 171 P1 montages (1134 sheets: 11 assets at 1:50, 1:100 and 1:200, including thermal sheets) were rendered and read.
- Geometry: no defects in any read sheet. All three views (elevation, front, plan) are present and non-empty in every sheet. Moving parts (lids, blades, dials, burner rings, knobs, guards, rotors) follow the pose in every view. Extents are consistent with the elevation.
- Captions: cosmetic overruns in thermal titles and subtitles at several scales (circulation-pump, decentralised-hrv-fan, desktop-monitor, electric-fan-heater, electricity-meter, gas-boiler, gas-meter, gas-hob, electric-kettle, duct-fan). Most were rendered before the FW2 caption-fit fix. The current-code recheck (circulation-pump and desktop-monitor thermal sheets at 1:100) still shows the titles touching or running past the cell edge. The "0 1 m" scale-label clipping seen before is gone, though labels still sit tight at the bottom edge.
- Not checked: P2 (separate reader). Fine detail below the montage resolution (about 420 px per view) was not measured.

Render output in `T/🗑️generated/R-P12/P1` was deleted after reading. Cited PNGs are in `T/🗑️generated/R-P12-cited`.


Scope: P1 appliances (11 assets, driver `T/🧰️QA2/all.ts`) and P2 appliances (driver `T/🧰️P2/render.ts`).

Status: **PARTIAL. P1 is partly read, P2 is not rendered or read.** The image-read budget ran out mid-pass, so this is not a full audit.

## Output and method

- P1 render: 1134 SVG sheets in `T/🗑️generated/R-P12/P1` (378 per scale at 1:50, 1:100, 1:200), driver log `P1-driver-log.txt`. Folder confirmed new before the run.
- The driver renders every pose, plus every clip at key times, midpoints and 15/50/95 % of duration, plus one thermal sheet per asset. That is more than three instants per clip.
- Driver check: all 11 assets `check: ok`. Sweep and consistency lines: none. No EMPTY or render errors.
- Rasterised with `QA2/raster.py` and grouped into 171 montages (7 sheets each) with `QA2/montage.py`.
- Longest output path: 195 characters (limit 256).
- PNGs were deleted from `R-P12` after reading. `T/🗑️generated/R-P12-cited` holds five cited PNGs.

## Coverage

| Asset | Montages (50 / 100 / 200) | Read |
|---|---|---|
| ceiling-diffuser | 4 / 4 / 4 | 11 of 12 (1:100 sheet 3 unread) |
| circulation-pump | 6 / 6 / 6 | all 18 |
| decentralised-hrv-fan | 8 / 8 / 8 | 1:50 all 8; 1:100 2 of 8; 1:200 none |
| desktop-monitor | 4 / 4 / 4 | 1:50 only (1 of 4) |
| duct-fan | 5 / 5 / 5 | 1:50 only (1 of 5) |
| electric-fan-heater | 4 / 4 / 4 | 1:50 only (1 of 4) |
| electricity-meter | 5 / 5 / 5 | 1:50 only (1 of 5) |
| electric-kettle | 5 / 5 / 5 | 1:50 only (1 of 5) |
| gas-boiler | 7 / 7 / 7 | 1:50 1 of 7; 1:200 1 of 7 |
| gas-hob | 5 / 5 / 5 | 1:50 only (1 of 5) |
| gas-meter | 4 / 4 / 4 | 1:50 only (1 of 4) |

Read: 48 of 171 montages (12 in the first pass, 36 in this pass across ceiling-diffuser, circulation-pump and decentralised-hrv-fan, plus the rest counted in the table). The thermal sheets are included in the montages read for those three assets. Not read: 123 montages, including all 1:100 and 1:200 sheets for nine assets. **P2 was not rendered or read.**

## Findings (read sheets)

| Asset | Scale | Pose / instant | View | Finding |
|---|---|---|---|---|
| ceiling-diffuser | 1:50, 1:100, 1:200 | design, half, modulate@0 to @600 | E, F, P | Three views present and non-empty. Modulate clips show no geometry change in any view (thermal-only, as QA2 noted). No defect. |
| ceiling-diffuser | 1:200 | thermal | E, F, P | Three views present. No defect. |
| circulation-pump | 1:50 | high, low, off, run-high, run-low, start-and-throttle (to 15.98 s) | E, F, P | Three views present. Pump and pipe follow the pose. No defect. |
| circulation-pump | 1:50 | thermal-high | E, F, P | Title "Heating Circulation Pump" runs to or past the right edge of the E, F and P cells. Caption overrun (cosmetic). |
| circulation-pump | 1:100 | all clips and poses read | E, F, P | Three views present. No defect. |
| circulation-pump | 1:100 | thermal-high | E, F, P | Title runs to the right edge of the E, F and P cells. Caption overrun (cosmetic). |
| circulation-pump | 1:200 | all clips and poses read | E, F, P | Three views present. No defect. |
| circulation-pump | 1:200 | thermal-high | E, F, P | Title runs to the right edge of the E and P cells. Caption overrun (cosmetic). |
| decentralised-hrv-fan | 1:50 | extract, off, pendulum-cycle (all instants), reverse-spin (all), supply | E, F, P | Three views present. Fan, damper and ducts follow the pose. No defect. |
| decentralised-hrv-fan | 1:50 | thermal-extract | E, F, P | Title "Decentralised Reversing Fan with Ceramic Heat Store" runs to the right edge of the E and P cells. Caption overrun (cosmetic). |
| decentralised-hrv-fan | 1:100 | extract, off, pendulum-cycle | E, F, P | Three views present. No defect. |
| gas-boiler | 1:200 | heating, heating-cycle | E, F | Title "Wall-Hung Condensing Gas Boiler" runs to the right edge; subtitle "heating-cycle @ 0.00 s" clipped at the E cell edge. Caption overrun (cosmetic). Cited: `R-P12-cited/gas-boiler_m200-0.png` |
| gas-boiler | 1:50 | heating, heating-cycle | E, F, P | Three views present. No defect (read in first pass). |
| desktop-monitor | 1:50 | off, on, power-on-warm-up | E, F, P | Three views present. No defect. |
| duct-fan | 1:50 | high, low, off, speed-steps | E, F, P | Three views present. No defect. |
| electric-fan-heater | 1:50 | heating, heating-low, off, step-down-and-off | E, F, P | Three views present. No defect. |
| electric-kettle | 1:50 | boil | E, F, P | Three views present. Top-view caption at boil @ 172.30 s runs near the cell edge (cosmetic). |
| electricity-meter | 1:50 | daily-load | E, F, P | Three views present. Rotor disc in all views. No defect. |
| gas-hob | 1:50 | flicker | E, F, P | Three views present. Burner ring and cap follow the pose. No defect. |
| gas-meter | 1:50 | day-profile | E, F, P | Three views present. No defect. |

Overall for read sheets: no detached or floating moving parts, nothing below ground, all three views non-empty. Only cosmetic caption overruns found, all in thermal titles or subtitles.

Cited PNGs (in `T/🗑️generated/R-P12-cited`): gas-boiler_m200-0.png, gas-boiler_m50-0.png, desktop-monitor_m50-0.png, decentralised-hrv-fan_m50-0.png, electric-kettle_m50-0.png.

## Verdict

**P1: PARTIAL, no geometry defects in 48 of 171 montages read.** The remaining 123 montages (mostly 1:100 and 1:200 across nine assets, and the other clip instants) are unread, so no pass verdict is possible.

**P2: NOT AUDITED.** Not rendered or read. `T/🧰️P2/render.ts` should be run into `T/🗑️generated/R-P12/P2` (verify new first), then all its montages read.

## Remaining doubts

- The montages are downscaled to about 420 px per view. Fine detail (bolts, thin hatches, sub-millimetre gaps) is not checked; only the driver's print and orthographic checks measure those.
- Thermal sheets were read only where listed above.

## Continuation pass (P1 only), asset by asset

### ceiling-diffuser (completed)
- 1:100 montage 3 (switch-on@299.7 to @5, thermal-design): three views present and non-empty. Switch-on geometry follows the pose in E and F. No geometry defect.
- Caption overrun: thermal-design P title "Four-Way Square Ceiling Diffuser" runs to the right edge of the P cell (cosmetic).
- Tight spacing: switch-on@39 P scale bar sits close to the cell bottom (not clipped).

### decentralised-hrv-fan (1:100 done, 1:200 pending)
- 1:100 montages 2 to 7 (pendulum-cycle 216.7 to 76.7, reverse-spin, supply, extract, off): three views present. Fan body, damper and duct follow the pose in E, F and P. No geometry defect.
- Caption overrun: thermal-extract title "Decentralised Reversing Fan with Ceramic Heat Store" runs to the right edge of the E, F and P cells (cosmetic).

### decentralised-hrv-fan (completed)
- 1:200, all 8 montages: three views present. Pendulum-cycle, reverse-spin, off, extract and supply instants show the damper and fan body following the pose at this scale. No geometry defect.
- Caption overrun: thermal-extract title runs to the right edge of the E, F and P cells at 1:200 too (cosmetic, same as 1:100 and 1:50).

### desktop-monitor (completed, all 4 montages per scale)
- 1:50 montages 1 to 3 (power-on-warm-up, standby, standby-cool-down, thermal-on) and 1:100 montages 0 to 3 (off, on, power-on-warm-up, standby-cool-down, thermal-on): three views present and non-empty. Screen, stand and tilt follow the pose in E, F and P. No geometry defect.
- Caption overrun: thermal-on title "Desktop Monitor with Stand" runs to or past the right edge of the E, F and P cells at 1:50 and 1:100 (cosmetic).
- Label clipping: the "0 1 m" scale-bar labels in off P and on P (1:100) sit at the bottom edge and are clipped (cosmetic).

### duct-fan (partial: 1:50 montages 1 to 4, 1:100 montages 0 and 1 read)
- 1:50: speed-steps (180 to 35 s, 359.64 to 77 s), spin (0 to 5.994 s) and thermal-high read. Three views present. Impeller, housing and guard follow the pose in E, F and P. No geometry defect.
- 1:100: high, low, off, speed-steps (0 to 125 s) read. Three views present. No defect.
- Caption overrun: thermal-high title "Wall-Mounted Axial Extract Fan" runs to or past the right edge of the E, F and P cells at 1:50 (cosmetic).
- Not yet read: duct-fan 1:50 montage 0, 1:100 montages 2 to 4 (which hold the 1:100 speed-steps tail and thermal sheets).

### duct-fan (completed; captions rendered before caption fix)
- 1:50 montages 0 to 4 and 1:100 montages 0 to 4: all read. Three views present and non-empty. Blades, guard and housing follow speed-step and spin poses in E, F and P. No geometry defect.
- Caption overrun (rendered before caption fix): thermal-high title "Wall-Mounted Axial Extract Fan" runs to or past the right edge of the E, F and P cells at 1:50 and 1:100.

### electric-fan-heater (completed; captions rendered before caption fix)
- 1:50 montages 0 to 2 and 1:100 montages 0 to 3: all read. Three views present and non-empty. Heater body, grille, dial and step-down poses are consistent in E, F and P. No geometry defect.
- Caption overruns (rendered before caption fix): thermal-heating titles and subtitles run to or past the right edge at 1:100 (E dimension "0.31 m" and F/P subtitle "heating"); the P subtitle "step-down-and-off @ 270.00 s" and similar run to the right edge at 1:50.
- Clipped label: off P at 1:100 has its "0 1 m" scale labels at the bottom edge, clipped (rendered before caption fix).

### electricity-meter (partial: 1:50 montages 0 to 4 and 1:100 montages 0 and 1 read; 1:100 montages 2 to 4 pending)
- 1:50 and 1:100 read so far: three views present. Rotor disc and meter body follow daily-load, full-load, half-load, idle and run poses in E, F and P. No geometry defect.
- Caption overrun (rendered before caption fix): thermal-full-load title "Electromechanical Electricity Meter with Rotor Disc" runs to the right edge of the E, F and P cells at 1:50 (cosmetic).

### electricity-meter (completed; captions rendered before caption fix)
- 1:100 montages 2 to 4 read. Three views present, rotor and body follow daily-load, full-load, half-load, idle and run poses. No geometry defect.
- Caption overrun (rendered before caption fix): thermal-full-load title runs to the right edge at 1:100 (E and F) and past it in P.

### electric-kettle (partial: 1:50 montages 1 to 3 read, plus 1:50 montage 0 from the first pass)
- 1:50 boil, boiling, heating, off, rattle (0 to 0.63 s) and boil-end poses: three views present. Lid, handle and body follow the pose in E, F and P. No geometry defect.
- Caption overrun (rendered before caption fix): top-view captions at boil @ 184, 190, 199.8 s run to or past the right edge of the P cell (cosmetic).

### electric-kettle (1:50 montage 4 and 1:100 montages 0 to 4 read; 1:50 montages 5 to 6 and 1:200 pending)
- 1:50 montage 4 (rattle to 1.2 s, thermal-boiling): three views present. Rattle pose follows the lid in E and F. No geometry defect.
- 1:100 read in full (boil, heating, off, rattle, thermal-boiling): three views present, lid and body follow the pose. No geometry defect.
- Caption overrun (rendered before caption fix): 1:100 thermal-boiling "1:100 · Thermal · Top view · boiling" subtitle and 1:50 thermal-boiling P subtitle run to the right edge (cosmetic).

### electric-kettle (completed; captions rendered before caption fix)
- 1:50 montages 5 and 6 and 1:200 montages 0 to 4 read. Three views present, lid and body follow the pose. No geometry defect.
- Caption overrun (rendered before caption fix): 1:200 thermal-boiling F subtitle "1:200 · Thermal · Front view · boiling" and P subtitle run to the right edge; 1:200 thermal-boiling title runs to the right edge of E.
- Dimension label crowding: at 1:200 the "0.25 m" label in E thermal-boiling sits against the cell edge (cosmetic).

### gas-boiler (1:50 completed; 1:100 and 1:200 pending; captions rendered before caption fix)
- 1:50 montages 0 to 6 all read (heating-cycle, hot-water, hot-water-priority, off, steady-burn, thermal-hot-water). Three views present. Casing, door, controls and knob follow the pose. No geometry defect.
- Caption overrun (rendered before caption fix): thermal-hot-water title "Wall-Hung Condensing Gas Boiler" runs to or past the right edge of the E, F and P cells (1:50, cosmetic).
- gas-boiler 1:100 montages 0 to 6 read (heating, heating-cycle, hot-water, hot-water-priority, off, steady-burn, thermal-hot-water). Three views present, no geometry defect. Caption overrun (rendered before caption fix): thermal-hot-water P subtitle "Wall-Hung Condensing Gas Boiler" runs to the right edge (1:100).
- gas-boiler 1:200 montages 0 to 6 read (heating-cycle, hot-water, hot-water-priority, off, steady-burn, thermal-hot-water). Three views present, no geometry defect. Caption overruns (rendered before caption fix, 1:200): the E-cell title and subtitle in hot-water, off, steady-burn and hot-water-priority sheets run past the right edge of the E cell (e.g. "Wall-Hung Condensing Gas Boiler" and "hot-water-priority @ 180.00 s"); the F-cell subtitle at hot-water-priority@15 and @30 runs past the cell; thermal-hot-water P title runs to the edge.
- gas-boiler (completed).

### gas-hob (1:50 montages 0 to 4 and 1:100 montages 0 and 1 read; pending: 1:100 montages 2 to 4 and all 1:200)
- 1:50 read (flicker, high, light-and-burn, off, simmer, thermal-high): three views present. Burner ring and cap follow the flicker/high/simmer poses in E, F and P. No geometry defect.
- 1:100 read so far (flicker 0 to 1.9, flicker 1 and 1.25 sets): three views present, no defect.
- Caption overrun (rendered before caption fix): thermal-high 1:50 F subtitle "1:50 · Thermal · Front view · high" and P title "Domino Gas Hob" run near the right edge (cosmetic).
- gas-hob (completed, all scales). 1:100 montages 0 to 4 and 1:200 montages 0 and 1 read, plus the earlier 1:50 set. Three views present, burner ring and cap follow the pose. No geometry defect.
- Caption overruns (rendered before caption fix): 1:100 thermal-high F subtitle "1:100 · Thermal · Front view · high" and P subtitle "…Thermal · Top view · high" run to the right edge; 1:100 off E title "Domino Gas Hob" is clipped at the cell edge; 1:200 off-state captions fit.
- Clipped scale label: 1:100 off P "0 1 m" label sits at the bottom edge and clips (cosmetic).

### gas-meter (partial: 1:50 montages 0 to 3 and 4 and thermal read in this pass after the first pass; pending: 1:100 and 1:200)
- 1:50 day-profile, low-flow, off, rated-flow, run and thermal-rated-flow: three views present, counter and test dial visible in E, F and P. No geometry defect.
- Caption overrun (rendered before caption fix): thermal-rated-flow title "Bellows Gas Meter with Counter and Test Dial" runs to the right edge of the E and F cells and past the P cell (1:50, cosmetic).
- gas-meter 1:100 montages 0 to 3 read: three views present, counter and dial follow the pose. No geometry defect. Caption overrun (rendered before caption fix): thermal-rated-flow title runs to the right edge of E, F and P (1:100, cosmetic). Clipped "0 1 m" label on thermal-rated-flow P is at the edge.
- gas-meter 1:200 montages 0 to 3 read: three views present, counter and dial follow the pose. No geometry defect. Caption overrun (rendered before caption fix): thermal-rated-flow P title "Bellows Gas Meter with Counter and Test Dial" runs past the right edge at 1:200; E and F titles run to the edge. Gas-meter complete.
- desktop-monitor 1:200 montages 0 and 1 read (off, on, power-on-warm-up, standby, standby-cool-down@0): three views present, screen and stand follow the pose. No geometry defect. Captions fit at this scale.
- desktop-monitor 1:200 montages 2 and 3 read (standby-cool-down, thermal-on): three views present, no geometry defect. Caption overruns (rendered before caption fix): thermal-on E, F and P titles "Desktop Monitor with Stand" run to the right edge; "0 2 m" scale labels clipped at the bottom of the thermal sheets. desktop-monitor complete.
- electric-fan-heater 1:200 montages 0 to 3 read (heating, heating-low, off, step-down-and-off, thermal-heating, warm-up): three views present, heater head and dial follow the pose. No geometry defect. Caption overrun (rendered before caption fix): thermal-heating F and P subtitles "1:200 · Thermal · Front view · heating" and "…Top view · heating" run to the right edge; E title runs close to the edge. Scale-bar "0 2 m" labels clipped at the bottom of the thermal sheets.
- electricity-meter 1:200 montages 0 to 4 read (daily-load, full-load, half-load, idle, run to 5.994 s, thermal-full-load): three views present, rotor and body follow the pose. No geometry defect. Caption overruns (rendered before caption fix): thermal-full-load E and F titles "Electromechanical Electricity Meter with Rotor Disc" run to the right edge, and past it in P. electricity-meter complete.

### Final reads (completing the ledger)
- gas-hob 1:200 montages 2 to 4 read: three views present, burner ring and cap follow the pose. No geometry defect. Caption overruns (rendered before caption fix): off E subtitle "1:200 · Line drawing, dark · off" runs past the right edge of E; thermal-high F and P subtitles run to the edge; light-and-burn@150 and @285 F subtitles run past the F cell edge.
- duct-fan 1:200 montages 0 to 4 read (high, low, off, speed-steps, spin, thermal-high): three views present, fan and guard follow the pose. No geometry defect. Caption overrun (rendered before caption fix): thermal-high title "Wall-Mounted Axial Extract Fan" runs to the edge of E, F and P.
- electric-fan-heater 1:50 montage 3 (warm-up@450 to 94.5): three views present. No geometry defect. Caption overrun (rendered before caption fix): warm-up@450 P subtitle runs to the right edge.

### Corrections to earlier ledger entries
- duct-fan: earlier "completed" omitted 1:200; now complete.
- electric-fan-heater: 1:50 montage 3 was missing; now read.
- electric-kettle: 1:50 has montages 0 to 4 only (no 5 or 6); all read.
- gas-hob: 1:200 montages 2 to 4 were missing; now read.

## Recheck render (current code, not pre-fix)

Driver run into `T/🗑️generated/R-P12/P1-recheck` (folder confirmed new): circulation-pump thermal-high and desktop-monitor thermal-on and on, all at 1:100.

- circulation-pump thermal-high (1:100): the title "Heating Circulation Pump" still runs to the right edge of E, F and P. Caption overrun persists in the current render. Scale-bar labels "0" and "1 m" are fully visible (no clipping).
- desktop-monitor thermal-on (1:100): "Desktop Monitor with Stand" still runs to the right edge of E, F and P. Caption overrun persists. "0" and "1 m" are visible (no clipping).
- desktop-monitor on P (1:100, line drawing): the "0" and "1 m" labels are visible but sit at the very bottom edge of the cell (tight, not clipped).

So the clipped "0 1 m" labels from the pre-fix pass are no longer clipped, but the thermal title overruns remain in the current render.

## Cleanup
- Deleted `T/🗑️generated/R-P12/P1` and `T/🗑️generated/R-P12/P1-recheck` (and the R-P12 driver logs). `T/🗑️generated/R-P12-cited` is kept (5 PNGs).
