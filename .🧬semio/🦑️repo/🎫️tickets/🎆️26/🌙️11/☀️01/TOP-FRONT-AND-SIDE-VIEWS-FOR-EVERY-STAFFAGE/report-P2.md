# Report P2: Appliances (11 Assets Plus split-ac Review)

**Assets:** hot-water-cylinder, kitchen-extractor-hood, laptop, mvhr-unit, pendant-lamp, room-thermostat, server-rack, shower-head, thermostatic-radiator-valve, toaster, vacuum-cleaner; split-ac-indoor-unit reviewed.

## State
- Front and plan are drawn per scale at 1:50, 1:100 and 1:200. 1:500 stays unavailable; the thermostat and TRV also have 1:200 unavailable.
- `presentation.viewpoints` is removed; `decisions.silhouette` and `omitted` are extended in en and de.
- Builders: `T/🧰️P2/a_<asset>.py` with `lib.py`, `lib2.py`; drivers `render.ts`, `round3.sh`. Writes are atomic. No rig changes remain (temporary rise bones were removed).
- Moving parts (lid, vane, hose, wand, nozzle, wheels, dials, knobs) are `solid.extruded` from their bone-local elevation profiles.

## Per-Asset Notes
- **hot-water-cylinder:** dial and label are edge-on on the right face and omitted; null-pen detail shapes at 1:100/1:200 are body role.
- **kitchen-extractor-hood:** filter and lamps are hidden from above, so absent in plan.
- **mvhr-unit:** spigots at ±0.07 m (±0.08 m at 1:200 for a print gap); 1:100 front beads dropped.
- **room-thermostat:** slab and dial thickened to 0.022 m at 1:100 (hand-picked).
- **shower-head:** hose bulge −0.13 m in front (hand-picked).
- **thermostatic-radiator-valve:** the elevation pointer was shortened (0.0135 to 0.0045 m) to clear a 1:50 feature violation; this also changes the side view.
- **split-ac-indoor-unit:** the front vane now extrudes the side-view vane profile and follows the vane angle.

## Verification
- Check (`office-laser`), orthographic consistency and the pose/clip sweep: ok for all 12.
- Three-views vitest: 6 of 322 fail, none in P2 assets (date-palm ×2, person-with-rollator, pregnant-person, running-child, seated-reader).
- About 16 previews inspected, mostly default poses at 1:50. `coverage.py --consistency` produced empty output and was re-dispatched.

## Open (Round 4 Dispatched)
- Vacuum plan nozzle barely overlaps the wand end.
- Read every pose and clip instant at every scale; run `coverage.py --consistency`.

## Round 4 (P2)

- **Vacuum plan nozzle:** the wand did overlap the nozzle by 0.108 m in the geometry (wand x 0.666-1.068, nozzle x 0.960-1.240 in the running pose, identical in elevation and plan). The nozzle (z 2.5) was drawn over the wand (z 2.5) and hid the wand end. In front and plan the nozzle is now z 2.4 and the wand z 2.6, so the wand visibly enters the nozzle in every pose (parked, running, running-pulled and the clip instants).
- **Full sweep:** every named pose and clip instants at 0, 50 and 100 percent of every clip, at every authored scale (1:50, 1:100, 1:200; 1:100 only for thermostat and TRV), for all 12 assets: 315 side/front/top sheets, no empty or non-finite view. Contact sheets per asset and scale (9 sheets in one image, 34 images) were rendered; I read the vacuum (1:50, 1:200), laptop (1:50), shower-head (1:50), toaster (1:50), split-ac (1:100), server-rack (1:50), hot-water-cylinder (1:100) and kitchen-extractor-hood (1:100) sheets in full, plus the earlier sheets for thermostat, TRV, mvhr, pendant. Nothing wrong found; the remaining sheets were covered by the finite/non-empty sweep and the numerical extents check only.
- **Check:** `checkAssetDetailed` (office-laser, physics on): ok for all 12 assets.
- **Consistency:** `consistency.ts` writes a line only for a finding; the "empty output" was a clean run. With an output path it writes the status JSON: every asset and every authored scale is `ok`.
- **Not rerun after the z change:** the three-views vitest (last run before the z change: 6 failures, none in P2 assets: date-palm x2, person-with-rollator, pregnant-person, running-child, seated-reader).
- **Cleanup:** `T/🗑️generated/P2` deleted.
