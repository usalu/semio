# 🔎️ R11 Audit — w04 (WP-04 storey + phase) and w05-modify (WP-05)

Status: both **PARTIAL / nearly done**. Gate not run; last logs (w04 check5, w05 check6) fail only in peer areas
(schedule window w13, railings BALUSTER/INFILL + model-graph railing arity + chain `Ramp` w09).

## WP-04
Present + wired: `🎢️set-element-storey` (12 cases), `set-element-phase` (8 cases): enum, KINDS, oracle, IO grammar +
binary protocol, feature rows; `🎭️phase-visibility` inference (in model graph, plan, projection) + unit tests; per-phase
quantities + feature "moving a wall … quantities per storey"; phase combobox (9 entity kinds) and storey combobox via
`🧩️entities/🕰️phasing`; IFC export (`Semio_Authoring.Phase`) and import `phase_of`; house IFC fixture phases.
Missing: outliner storey drag (dispatch set-element-storey); view phase consumed by plan/world windows (View.phase exists);
examples (house/office) storey move + phases; phase-visibility `.feature` + gating/cache-transparency tests; IFC re-bless.

## WP-05
Present + wired: nine leaves (copy/mirror/array/align-elements, offset-wall, trim-extend-wall, split-slab, split-beam,
set-wall-end-join) with schema, tests, fixtures, enum, KINDS, oracle, feature rows; `🧙️modify` helper; gestures
`👯️duplicate` (copy, mirror, array linear/radial) and `✂️reshape` (offset, trim, extend, align, split); arm-utility rows;
command table; en+de labels; join fields in entities.
Missing: previews in the window transient (transient dir holds only a placeholder); hotkey bindings unverified
(`⌨️completeness` test); examples (copy/mirror/array use in house); sum-law tests verified only loosely.

## Remaining tasks
1. Gate (`cargo test --lib`) once r11-baseline is done; generators.
2. WP-04: outliner storey drag + keyboard alternative (accessible), view phase in plan/world, examples, phase-visibility
   feature + oracle + gating tests, IFC re-bless.
3. WP-05: gesture previews (window transient), hotkeys + completeness test, examples, sum-law verification.
