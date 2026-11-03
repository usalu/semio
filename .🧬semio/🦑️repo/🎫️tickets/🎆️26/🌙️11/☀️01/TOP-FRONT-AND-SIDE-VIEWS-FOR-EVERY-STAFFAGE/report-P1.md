# Report P1: Appliances

**Assets:** ceiling-diffuser, circulation-pump, decentralised-hrv-fan, desktop-monitor, duct-fan, electric-fan-heater, electric-kettle, electricity-meter, gas-boiler, gas-hob, gas-meter.

## Scope
- Front and plan views exist at 1:50, 1:100 and 1:200; 1:500 stays unavailable.
- gas-hob: front at all three scales; plan added at 1:200, while its existing 1:50 and 1:100 plans were kept.
- `presentation.viewpoints` is removed.
- The `silhouette` and `omitted` decisions are extended in en and de.
- The scripts are in `T/🧰️P1/`.

## Conventions
- Front and plan use rest-pose world metres with the root at the origin.
- Elevation-visible features sit at −y in plan and −x in front, since front x equals plan y.
- For wall units and meters the elevation is the face view, so front and plan show the side profile.
- Bone binding:
  - kettle lid and button: bound to `lid`
  - monitor panel: on `panel` with `followTip`, so the tilt follows
  - rotors, knobs and flames: fixed. No `solid` placement is used.

## Drawing per scale
- 1:50 is fully detailed.
- 1:100 reduces to masses.
- 1:200 keeps one or two composed masses.
- The electricity meter and gas boiler at 1:200 were thickened, after a minimum feature of 0.197 mm, to pass `office-laser`.

## Checks
- `consistency.ts` is ok for all 11; it sweeps all poses plus 6 instants per clip.
- The `check` CLI passes, and all three scales are suitable under `office-laser`.
- Both were run with `bun` directly, because the `repo` gate stalled.
- The three-views vitest was not run.
- Visual inspection was done in the browser pane:
  - front and plan of all 11 assets at 1:50 and 1:100
  - the gas-hob sheets at its default pose
  - Other poses and clips are covered by the sweep only. QA2 will follow up.

## Doubts
1. gas-hob knob: the elevation shows it at forward x 0.105, height 0.019; the plan shows it at lateral −0.2. Coordinator reading: these are consistent (the same forward position), so no change.
2. Fan heater: the dial bump is omitted at 1:100 and 1:200, leaving front x at −0.115…0.12, which is within tolerance.
3. HRV fan and duct fan: the internal regions `core`, `blades` and `hub` are hidden in front and plan.
