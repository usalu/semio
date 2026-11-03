# R2 ic: physics fundamentals Beat1 device icons

Beat1 shows the vacuum, kettle and shower one after another in the same slot (x 4.8, y 1.95, inside the device card), not side by side. The slot is 0.8 x 0.6 units. The narration is about power (kW), not real sizes, so the decision is equal optical height, with a common slot centre.

## Cause
- `fitted_staffage` fits the group bounds, not the drawn part. The vacuum motion group is 1.35 wide while the drawn pose is about 0.6 wide, so it sat off-centre and drifted out of the slot while running (the run pose adds hose and wand to the right).
- The kettle motion group included steam rays above the body, which crossed the card frame.
- The shower was fitted at height 0.65 against 0.6 for the others.

## Changes in `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (lines about 692 and 708-709), all via `window=`
- vacuum: `width=1.0, height=0.6, window=(-0.6, -0.05, 0.95, 0.75)`, a window covering the whole run clip (body, hose, wand); the wider limit keeps the body readable.
- kettle: `window=(-0.12, 0.0, 0.13, 0.25)`, the body only; steam is clipped inside the window.
- shower: height 0.65 to 0.6, window from sm2 kept.
- Probe scripts (kept): `🧰️ic/probe.py`, `probe2.py`, `measure.py`, `render.sh` (physics variant of `r2_render.sh`), `frames.py`, `card.py`.

## Verification
- Rendered `Beat1_Leistung` (ql, 15 fps, de, jobs 1), frames extracted with PyAV at 28, 31, 34, 36, 40, 50 s. Vacuum idle and running, kettle cold and hot, and shower now stay inside the card, share the slot centre, and have comparable height (about 0.6 for kettle/shower, about 0.5 for the vacuum).
- `staffage_only` guard: 20 tests OK.
- Authoring clip tests (`🏗️authoring/🎬️clip/🧪️tests`): 31 tests OK (1 skipped). There are no physics-specific clip tests.

## Open
- The vacuum body is still smaller than the kettle body in the run pose, because the hose and wand share the window; the idle wand pose is also smaller than before.
- The shower head is intrinsically slim and small (about 0.2 m); no spray is visible at card scale.
- Other beats (Beat4 evaporator/condenser/compressor/valve, Beat2 vacuum and shower head) are not rows of mixed icons in the same sense and were not changed.
