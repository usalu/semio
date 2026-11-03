# Report QA2: Visual QA of P1 Appliances (11 Assets)

## Round 1
- **Check CLI, consistency sweep and three-views vitest:** ok, 22 of 22 tests pass. No asset edits were made.
- **Driver `T/🧰️QA2/all.ts`:** evaluates and depicts directly for every pose, plus clip instants at 15%, 50% and 95% and the thermal view at the default pose, across 1:50, 1:100 and 1:200. Rasterizing is done by `raster.py`. The driver skips the per-instant thermal transient solve that made the CLI too slow.
- **Inspected:** the default sheets at every scale plus thermal for all assets, the full desktop-monitor montage, and a zoom on the circulation-pump front.
- **Motion:** a hash of each view per instant shows the expected changes. Rotations that leave the outline unchanged show only in elevation.

## Open (round 2 requested)
- The 1:50 pose and clip montages of 10 assets have not been inspected.
- The fan-heater warm-up was not sampled after its pose change at t = 5 s.
- The diffuser `modulate` clip shows no change in any view.
- Near copies between 1:100 and 1:200: kettle plan, gas-meter front and plan, gas-hob.
- The electricity-meter plan shows no hot region in the thermal view.

## Round 2
- **Checks:** check ok for 11 of 11, sweep clean, and the three-views vitest passes 22 of 22. The driver now samples every clip key time, including the points just before and after each key.
- **1:200 redraws:** electric-kettle, gas-meter and gas-hob are now simplified masses; the en/de decisions are updated.
- **electricity-meter:** the rating plate is a raised strip, so its hot region reads in plan.
- **Diffuser `modulate` and monitor clips:** they change only the thermal state, so no geometry changes in any view (correct).
- **Wall fans and meters:** the front is a true view along −x, with room-facing parts at −lateral. No redraw was needed.
- **Open:**
  - The hob's 1:50 plan flame ring stays visible in the off pose, because plan has no depth occlusion by height (the same issue as the tealight). Decision: FW adds dynamic depth ordering to the views.
  - The 1:100 sheets and most of the non-default 1:200 sheets are not inspected.

## Round 3
- **Sheets read:** all 1:100 and 1:200 sheets, plus thermal, de-duplicated by view hash. No defects found.
- **Hob:** the 1:50 plan flame ring is bound to `flame`, which sinks 0.036 m when off. It will be hidden in plan once FW's depth ordering lands.
- **Lateral motion:** no appliance pose implies a sideways turn; the 0.06 mm flicker lean is below the drawing resolution.
- **Checks:** check ok for 11 of 11; three-views vitest passes 22 of 22.

## Round 4
- **gas-hob:** the burner ring and cap are bound to the `burner` bone, so depth ordering hides the sunk flame ring in the off-pose plan at 1:50. It is still visible in high and simmer.
- **gas-boiler:** the 1:50/1:100 knob failed the print check (0.180 mm) after a framework change. The front and plan knob are rebound to the static `casing`, which is physically correct because the knob is seen edge-on.
- **Same-region details:** the meter and heater detail z values are raised.
- **Writes:** `T/🧰️P1/lib.py` saves atomically.
- **Checks:** check ok for 11 of 11; three-views vitest passes 22 of 22.

## gas-hob 1:200: burner too thin (QA4)
- **Which instance:** `T/🧰️QA4/suit.ts` isolates the violation per view and pose: only the **side view** (elevation) at 1:200, shape `burner`, in every pose (off, simmer, high); 0.112 m wide by 0.018 m high, 0.154 mm measured. The front block (0.14 by 0.05 m) and the plan disc (0.12 m) are wider. A pose probe (`T/🧰️QA4/gh.ts`) shows the burner at y 0.040 to 0.058 m, above the plate top and in front of the trivet (z 3.5 over 3.2), so it is visible in the drawing and not hidden. No wait for the FW3 broadcast was needed.
- **Fix:** the 1:200 side-view burner step is now 0.012 m and the cap 0.030 m high (was 0.007 and 0.018), top 0.070 m, still below the trivet top (0.088 m, so the height does not change). `T/🧰️QA4/gas_hob_burner.py` writes the elevation shape atomically (`write_json_atomic`); `T/🧰️P1/run.py gas-hob` re-applied the front and plan geometry and the decisions. The 1:200 `silhouette` decision (en and de) now says the burner is drawn 0.03 m high so it stays a printable 0.2 mm feature; front block and plan disc are already wider.
- **Checks:** `check` ok for gas-hob (all scales, all poses, three views, no sweep or consistency finding); three-views vitest for gas-hob: 2 passed. The 1:200 high-pose sheet shows the burner as a small raised step in side and front view and the disc with the knob ring in plan.
