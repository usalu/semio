# ha2 audit: appliances (round 2)

Scope: the 12 assets in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances` (laptop, mvhr-unit, pendant-lamp, plate-heat-exchanger, room-thermostat, server-rack, shower-head, sorption-wheel, split-ac-indoor-unit, thermostatic-radiator-valve, toaster, vacuum-cleaner). Every asset was read through a condensed sheet of its rig, representations, thermal model, poses and clips. Scale and mass sums were checked by script, and the catalog check (`🔨️modules/🗂️catalog/🩺️check`, `--asset`) was run on every touched asset.

## Check results

- Touched and checked: 11 of 12. Each prints `✓ <asset>` (laptop, mvhr-unit, pendant-lamp, plate-heat-exchanger, room-thermostat, shower-head, sorption-wheel, split-ac-indoor-unit, thermostatic-radiator-valve, toaster, vacuum-cleaner). Exit code 0 for all.
- Not touched: server-rack. Its heat budget and clips were checked and found consistent (idle 90 W surfaces + 1.4 kW air ≈ 1.5 kW; full load 300 W + 4.7 kW air ≈ 5 kW).

## Fixes applied (Edit, JSON)

Emitter level is `pose emitter strength × operating level` (`💨️transport/🔥️sources/🟦️.ts:59`). Several poses and tracks bound strengths below 1 on top of an operating level below 1, which halved or quartered the flow. These are fixed by setting the strength to 1.

- **mvhr-unit**
  - Pose `normal`: `supply-air` and `extract-air` 0.5 → 1 (flow was 55 m³/h instead of 110).
  - Clip `boost-spin`, emitter tracks: 0.5 → 1 at t=1 and t=3.
  - Clip `day-cycle`, emitter tracks: 0.5 → 1 at t=0 and t=1200.
  - Clip `day-cycle`: added the missing `operating` event at t=600 ("Boost for the shower").
  - 1:500 reason: spigot gap 0.04 mm → 0.08 mm (42 mm / 500).
- **shower-head**
  - Pose `eco`: `spray-mist` and `steam-plume` 0.75 → 1 (flow was 0.56 of the intended level).
  - Clip `eco-and-off`, emitter tracks: 0.75 → 1 at t=60.
  - Hose at 1:50: `penMm` 0.35 → 0.24 (true width of a 12 mm hose; 0.35 exceeded the real outline). Decision text updated.
- **split-ac-indoor-unit**
  - Pose `cooling-low`: `cold-jet` 0.5 → 1 (low fan speed was 112 m³/h instead of 225).
  - Clip `start-cooling`, `emitter:cold-jet` track: constant 1 (removed the double ramp against the fan).
  - Clip `vane-swing`: motion event gained `target: "vane"` (it had none).
- **plate-heat-exchanger** (largest change)
  - `plates` and `ports` were `heater`/`operating` with 60 W and 5 W and no electrical input. Changed both to `passive` and removed their `operating` blocks.
  - Added anchors `supply-collar` and `exhaust-collar` (ports bone).
  - Added vent emitters `supply-air` (+0°, −3.1 K) and `exhaust-air` (180°, −17.9 K) on the `exchange` key, 0.278 m³/s, reference `en-308-1997`, which reproduces the 85 % EN 308 numbers.
  - Poses `off`, `partial`, `full` bind both emitters at 1.
  - Provenance (en/de) rewritten to match: "220 plates at 2.5 mm pitch" → 200 (200 × 2.5 mm = 0.5 m, the pack depth).
  - Decisions: 1:50 silhouette 10 × 13 mm → 12.4 × 12.7 mm; 1:50 pitch "4 mm" → 2.5 mm; 1:100 silhouette 5 × 6 mm → 6.2 × 6.3 mm; 1:200 silhouette 2.5 × 3 mm → 3.1 × 3.2 mm. Hatching decisions at 1:50 and 1:100 said "None" while a hatch is declared; corrected. "detail none mm" / "Detail keiner mm" → "detail none" / "Detail entfällt" (1:100 and 1:200). Overlay text at 1:50 rewritten (no more heated pack).
- **pendant-lamp**: shade `areaM2` 0.24 → 0.11 (half-dome outer area, 2πr² with r = 0.133 m; 0.24 was a full sphere).
- **laptop**: decision sizes corrected. 1:50 silhouette 0.22 × 0.22 m, 4.4 × 4.4 mm → 0.275 × 0.218 m, 5.5 × 4.4 mm. 1:100 2.2 × 2.2 → 2.8 × 2.2 mm. 1:200 1.1 × 1.1 → 1.4 × 1.1 mm. 1:500 reason 0.4 mm → 0.55 mm wide (en and de).
- **toaster**
  - Decisions: 1:50 "15 mm wide" → 6 mm (0.3 m / 50); 1:100 "30 mm × 20 mm" → 3 mm × 2 mm (en and de).
  - `ignition` event → `operating` (switch-on of electric elements is not ignition).
  - Clip `cool-down` event "Toast taken out": `operating` on `elements` → `motion` on `slices`.
- **vacuum-cleaner**: clip `run-and-park` `ignition` → `operating`.
- **sorption-wheel**
  - Clip `start-and-run`: rotor end angle 720° → 408°. 720° in 290 s is 24 rev/h, but the rated speed is 12 rev/h (≈ 348° in 290 s; 60° + 348° = 408°).
  - Clip `spin`: `loop` false → true (continuous rotor with closing key at 360°).
  - Event `ignition` → `operating`.
- **room-thermostat**, **thermostatic-radiator-valve**: 1:50 `validRange.minDenominator` 10 → 20 (typical range for 1:50 is 20…75; the casts use the default scale 50).

## Judgment items (not changed)

1. **Category.** `plate-heat-exchanger` and `sorption-wheel` are HVAC plant components (air-handler parts), and `evaporative-humidifier` is in the same situation. The ticket summary already lists them under 🏗️machinery, and `finned-heat-exchanger`, `scroll-compressor` and `heat-pump-outdoor-unit` are there. Recommend moving them to `🏗️machinery`. Not moved, as instructed. Moving them also means updating the manifest and the cast-manifest asset ids.
2. **`widthM` naming.** For `laptop` and `split-ac-indoor-unit` the `widthM` value is the side-elevation horizontal extent (laptop 0.275 = depth plus lid tilt; split AC 0.21 = depth), and `depthM` is the lateral width. The catalog checker enforces `widthM` against the default drawing, so the names contradict real left-right width. Schema-owner decision.
3. **Sorption wheel thermal.** The provenance says the disc (60 W) and regen sector (55 W) are chosen to reach 31 °C and 60 °C. That is a temperature authored through power. Physically the adsorption heat (≈1–1.5 kW for this wheel) and 70 °C regeneration air (≈4–5 kW) go through the air. Consider vent emitters. Regen "solar" heat is also not in the environment (solar 0).
4. **Pen weights.** Toaster and vacuum body pens at 1:50 are 0.35 mm (0.25 at 1:100), double the 0.18 mm standard used elsewhere. Shower hose at 1:100 (0.3 mm for 0.12 mm real) and 1:200 (0.25 mm for 0.06 mm real) are thickened far beyond true width; the rule's real-tolerance limit is not stated, so I left them. Option: merge or drop the hose at 1:200.
5. **Pose-level linear scaling.** Exchange and damper "partial" states scale temperature rise linearly (plate HX, split AC). A true half bypass gives ≈ −12 K supply, not −1.6 K. Accepted as an approximation.
6. **Lampholder area** (pendant). 0.012 m² vs ≈0.005 m² for collar plus cap. Small effect (6 W).
7. **Pendant "almost all" of 60 W ends as heat.** The model books 100 %.
8. **Thermostatic valve.** Body carries 3 W operating heat from supply water and the pose binds a 55 °C pipe contact. Possible double count; the contact should carry it.
9. **Split AC latent cooling.** The 2.5 kW rating is ≈2.0 kW sensible (0.125 m³/s × 13.3 K) plus ≈0.5 kW latent. Latent is not modelled. Acceptable.
10. **MVHR loop.** The steady impeller spin has no looping clip. The rule asks for one "where meaningful". Suggest a `normal-spin` loop clip.
11. **Plausibility.** Toaster top plate 100 W on 0.045 m² (≈2.2 kW/m²) implies a surface above typical 100–120 °C. Shower spray 300 W from 12 L/min at 38 °C is a fudge-like source. MVHR casing 45 W behind R = 0.5 insulation may run warm.
12. **Views.** Only `split-ac-indoor-unit` has more than the elevation (front, plan). No cast needs plan; extrema uses front. Other assets: elevation only, sufficient for the current casts.
13. **Sorption `start-and-run`** and **server-rack `power-up`**: the wording ("spin up within 10 s") and the step timing of airflow (5 s) are consistent enough; left.
14. **Vacuum `floor-stroke`** loop (4 s) has no closing key at t = 4; relies on loop-aware interpolation. Passes.

## Unchanged, verified

- Rig mass sums match `massKg` for all 12 assets.
- Dimensions and extents pass the checker's 2 % tolerance (`extent-height`/`extent-width`).
- Tiers: 1:500 unavailable where drawings fail (laptop, MVHR, PHX, sorption, toaster, vacuum, shower, split AC, thermostat, TRV). 200 unavailable for the thermostat and TRV. All other validRanges are inside the typical tiers.
- Thermal region coverage: every filled shape carries a region and every drawn region appears at each scale (checker).
- Heat budgets: MVHR (38 / 75 W split between casing and impeller), toaster (180 + 100 + 150 + 470 = 900 W), vacuum (110 + 40 + 30 + 520 ≈ 700 W), pendant (36 + 6 + 18 = 60 W), rack (see above).
