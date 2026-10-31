# Content v2 report: NW-A2 (mallard-duck, herring-gull, red-squirrel, hedgehog) — IN PROGRESS

Status 2026-10-29 (interrupted three times; the last probe run was rejected by the user/permission system, so I stopped there).

## Done
- Scratch tooling in `T/🧰️scripts/NW-A2/`: `nw.py` (forward kinematics mirroring `🦴️rig`, leg inverse kinematics, hand-point splines, bone-local shape drawing, rsvg sketch renderer),
  `mallard_phys.py` / `mallard_scn.py` / `mallard_asset.py` / `mallard_reps.py` / `build_mallard.py`, `harness.ts` (validateAsset + measureSuitability + default-pose bounds for chosen files).
- **mallard-duck** written to `S/🗂️catalog/🐕️animals/mallard-duck/🔣️.json` (v2, 1.15 kg, 0.60 x 0.44 m): 20 bones (3-piece S-neck, wings, webbed legs), anchors (feet, waterline buoyant support, nares, plume),
  named poses standing (default), swimming, dabbling, wing-stretch (+ key poses upending, 8 waddle, 6 paddle), clips waddle (loop, plant), paddle (loop), dabble (loop), 8 plumage regions with layers,
  `thermal.body` (Lasiewski and Dawson 1967 basal 4.2 W, 41.5 degC), nares respiration with 4 patterns (rest/walk/swim/dabble, authored-estimate guided by Lasiewski and Calder 1971), plume emitter,
  exhaustive pose bindings. Representations: 1:500 unavailable (neck 0.045 m = 0.09 mm), 1:200 / 1:100 / 1:50 authored with individual shapes, pens, decisions, validRange 150-350 / 75-150 / 20-75.
- Joint-limit issues of the waddle key poses found by harness (thigh 153 degrees) still to widen.

## Not done / open
- Harness result for mallard: validation only joint limits (3 waddle key poses); print suitability still FAILS at all three scales, almost only ink gaps below 0.15 mm between neighbouring outlines
  (neck pieces, head/neck, leg pieces, wing/leg in water poses) and small features (foot 0.155, bill/eye). Plan: deeper overlaps at joints, merge neck pieces at small scales, thicker feet, strokes for thin parts,
  pose-aware probe (`probe.ts`, not yet run).
- herring-gull, red-squirrel, hedgehog: not started (design decided: 1:500 unavailable for squirrel and hedgehog, probably for gull and mallard; spine skirts rig for the hedgehog roll).
- Official `check` per asset not yet run (harness only).
