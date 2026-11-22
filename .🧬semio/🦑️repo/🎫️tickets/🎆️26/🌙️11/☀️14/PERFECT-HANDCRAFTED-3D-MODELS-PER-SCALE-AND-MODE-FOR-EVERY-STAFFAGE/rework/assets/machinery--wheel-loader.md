# machinery/wheel-loader — running report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published and `VALID`, but not signed: the image reader of this session was unreachable for the whole pass (every `Read` call ended in "PreToolUse hook did not respond before its timeout"), so no PNG could be looked at. Sheets were rendered and read only as character art (`rework-wheel-loader-ascii.py`, `sips` to BMP, about 200 columns), which shows silhouette and gross form, not surfaces, seams or small parts. Everything below was decided from numbers and that coarse view.

State found: source `T/volumes/machinery/wheel-loader.json` and catalog `volumes` were identical (154 / 120 / 68 / 40 body solids); baseline `r3-validate.ts --volumes` was `VALID`. Earlier work: `rework/10.md` section 2 (articulation bone, steering cylinders, floating parts closed).

### Reference (20 t articulated wheel loader, Cat 962M / Volvo L120H class, 3.5 m3 bucket, 23.5R25)

| quantity | real machine | model before | model now |
|---|---|---|---|
| length (bucket on ground) x width over bucket x cab height | 8.3–8.5 x 2.9–3.0 x 3.4–3.5 m | 8.42 x 2.95 x 3.45 m | unchanged, ok |
| wheelbase, hitch at mid wheelbase | 3.30–3.35 m | 3.35 m, hitch at x 0 | ok |
| tyre 23.5R25 diameter / section, width over tyres | 1.615 / 0.60 m, 2.74–2.80 m | 1.615 / 0.60, 2.74 m | ok |
| tread | 2.14–2.20 m | 2.14 m | ok |
| ground clearance (under differential bowl) | 0.43–0.47 m | 0.54 m (bowl 540 mm) | **0.4475 m** (bowl 720 mm) |
| axle arm tube / differential bowl / hub flange | 280–320 / 680–740 / 340 mm | 240 / 540 / 280 mm | 300 / 720 / 340 mm |
| rear axle mounting | oscillating cradle seated on the arm tubes beside the bowl | one plate 0.92 m wide through the bowl | two saddle pads on the arm tubes (z 0.26–0.50 m) |
| front axle mounting | bolted to pads under the front frame, floor open at the bowl | belly plate ran through the bowl | two pads between the tower plates; floor ends at x 1.31 m, separate nose skid |
| driveline | transmission under the cab, rear shaft, front shaft in two pieces with a universal joint on the articulation axis, pinion noses | absent at every scale | modelled (see below) |
| hitch bulkheads | plates with an opening for the shaft | closed boxes | extrudes with a round (1:50) / octagonal (1:100) hole |
| lower hitch pin | ends below the propeller shaft | 190 mm, would cross the shaft | 160 mm (y 0.59–0.75 m), shaft axis at 0.8075 m |
| lift cylinder base pins | one stub pin per tower plate | one tube across the frame, crossing the shaft line | two stub pins z 0.38–0.70 m |
| lift arm pivot height / pivot to bucket pin | 2.3–2.4 / 2.7–2.8 m | 2.30 / 2.74 m | ok |
| cab width / hood top | 1.5 m / 2.5–2.6 m | 1.51 m / 2.6 m | ok |
| articulation | 37–40 degrees each way | joint in the rig, limits -40..40 | unchanged; still no pose or clip uses it |

### Rebuilt (`rework-wheel-loader-driveline.py`, `rework-wheel-loader-scales.py`)

- 1:50 body: `axle-r`, `axle-f` new revolve profile (arm 150 mm radius, flange 170, bowl 360); `axle-cradle-r` as two pads (repeat); new `axle-pad-f`; `belly-plate` shortened, new `nose-skid`; `hitch-bulkhead`, `front-bulkhead` as `yz` extrudes with a shaft hole; `hitch-pin-lower` 160 mm; `lift-cyl-crossmember` as two stub pins; new `transmission` (cast case with sloped sump, hidden behind the rails in the side view), `transmission-mount`, `pinion-r`, `propshaft-r`, `propshaft-m`, `propshaft-f`, `pinion-f` (revolves with flanges; mid and front shaft meet on the articulation axis so the joint can work).
- 1:100: same parts for the 15 mm feature (bowl without the flange step, shafts plain 90 mm with pinion noses merged in, case from rail to rail, octagonal holes); added `fuel-tank` (rounded block), `seat`, `seat-back`, `seat-pedestal`, `steering-column`, which were missing although many features large.
- 1:200: axle with bowl (was a plain 260 mm cylinder), pads, open floor with skid, transmission block, one 100 mm shaft per frame, stub pins, `fuel-tank` as plain block.
- 1:500: axles with bowl, rear pads; no driveline (hidden in the frame bodies).
- Thermal: new solids referenced in `front-frame` / `plate` (`seat-pedestal` in `cab`), as their neighbours are. Structure: `member-articulation` shortened with the lower pin; the load path is unchanged (tyre, rim, axle, pads, rails, hitch plates and pins, tower, arm, bucket); new driveline parts are not members.
- 2D: the `axle-r` / `axle-f` shapes of the front and plan drawings at 1:50, 1:100 and 1:200 redrawn by hand to the new housing (12 shapes; positional text replacement, verified that nothing else in the catalog file changed). Checked in the product's own front drawing in pose `dump-high`: the bowl shows between the tyres.
- `decisions`: one sentence block per scale (en, de) appended.
- Rig, poses, clips: not changed.

### Validation

- `r3-validate.ts machinery/wheel-loader --volumes`: `VALID machinery/wheel-loader (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0` (after the last change).
- Published with `volumes-insert.py` twice (after the driveline stage and after the scales stage); `--catalog-only`: `VALID ... schema 0, validateAsset 0`.
- Rendered after the last change: all four scales x body / thermal / structure, the five poses, two instants of each clip (no `NOT DRAWN`). Read only as character art: carry, dig, dump-high and the clip instants show a plausible machine (bucket on the ground in dig, arm at full height and bucket dumped in dump-high).

### Remains open (for the next pass)

1. Nothing was judged by eye. Every scale, list, pose and clip must be looked at as an image by an author whose image reader works; this pass cannot vouch for surfaces, seams, proportions of small parts or interpenetrations that the rules do not measure.
2. Rig: no pose or clip uses the articulation (`rework/10.md`: a twisting pose makes `validateAsset` demand about 100 2D shapes per scale of the front half as extruded/revolved `solid`s). The rear axle has no oscillation bone (real: +-12 degrees about a longitudinal trunnion; `T/rig-lateral-rest.md` did not exist).
3. 1:50 body: `fuel-tank` (loft 1.36 m wide at y 1.0–1.4 m) is pierced by both rear rails; a real tank hangs under or sits between the rails. Moving it changes the side silhouette and needs the 2D `fuel-tank` shapes redrawn. `hood` is a superellipse loft without the stepped rear, doors or radiator guard; `cab-riser` is a plain box; no hydraulic tank, hoses, beacon, tow pin, bucket spill guard or wear plates; the front axle arm tubes pass through the tower plates without a cut-out.
4. 1:100: 2D and 3D both leave out tail lamps and work lamps (60 x 90 x 140 mm, above the 15 mm feature), tyres have no tread lugs (115 mm wide) and are solid discs to the axis; many solids are still identical to 1:50.
5. 1:200: head lamps and mirrors (above 30 mm) are in neither 2D nor 3D; rims are plain discs.
6. 1:500: `front-body` and `rear-body` are full-width blocks; the 2D front and plan drawings do not draw the axles at all, so the new bowl is in 3D only (within the silhouette limits).
7. Thermal: lists are almost entirely `{ of }` references in one catch-all region `front-frame` (which also holds rear-frame parts); regions were not re-partitioned and `areaM2` was not re-measured by hand (the rules pass).
8. Structure: not re-idealised; the masses are ellipsoid markers; the cylinders are members but the Z-bar load path was not re-checked against a hand calculation.
9. 2D: only the axle shapes were redrawn. The new transmission is hidden in all three views by design; the 2D side elevation was not compared shape by shape with the body (`r3-project2d --check` not run).

### Fundamental issues (outside this asset)

- The session's `Read` tool was unusable (hook timeout on every call, text files included), so the brief's central instruction "render, read the image" could not be followed. `rework-wheel-loader-ascii.py` is a stopgap, not a substitute.
- `r3-validate.ts --volumes` takes about 170 s for this asset (brief says about 25 s).
- The catalog file mixes two formats (`representations` with `indent=1` one number per line, `volumes` in the compact style of `volumes-insert.py`), so a 2D root fix needs positional text surgery; there is no `2d-insert` tool that keeps the rest byte-identical.
- `T/rig-lateral-rest.md` does not exist, so a longitudinal or lateral rest axis (rear axle trunnion) could not be authored to a stated rule.
