# Fix Heat Calc Loss Overlap (Scene2)

## Problem
`Scene2` (“Vom Wärmestrom zur Jahresenergie”) stacked the loss equation on the house roof, drew `H_T`/`H_V` on top of “Transmission”/“Lüftung”, and used a long climate blurb that collided with the top formula. Chart labels also clipped the θ_e curve.

## Fix
- Banded layout: title → short `H = Q̇/Δθ` → `Q̇_l = … → H_T + H_V` → icons → labels → shared-baseline `H_T`/`H_V` readouts above `SAFE_BOTTOM + 0.55`.
- Smaller/higher icons (`scale=0.48`), label buff `0.38`, shared `read_y`.
- Degree-day chart raised (`O.y = -1.55`), right-side θ_i/Heizgrenze at `x=3.15`, θ_e tag moved off the curve.
- Tests: `scene2ClimateBand` + Manim `DOWN` clearance check; `htHvWindows.scene2Scale = scale=0.48`.

## Verify
- Side-by-side with the reported frame: roof/label/calculation collisions gone.
- Preview: `videos/preview/de/speaker/s/63/v/Scene2.mp4`
- Full heating master not rebuilt (disk ~1.3 Gi free).

## Files
- `🎓️teaching/…/️calculation/🐍️.py` (`Scene2`)
- `🎓️teaching/…/️calculation/️tests/🐍️.py`
- `🎓️teaching/…/️calculation/️tests/🔣️.cases.json`
