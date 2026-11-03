# Render audit R3: non-energy staffage (final visual pass)

Scripts (rerunnable via `r3_run.sh`, which sets `UV_PROJECT_ENVIRONMENT`, `SEMIO_TEACHING_OUTPUT_ROOT` and runs `uv run python`): `r3_icons.py` (contact sheet), `r3_opt.py` (room section winter/summer), `r3_exp.py` (experiments section, no overhang and 1 m overhang), `r3_crop.py` (zoom), `r3_bimtest.py` (BIM test subset).

## Frames viewed and verdicts

| Item | Verdict |
|---|---|
| excel `program` plan occupants (`viewpoint: plan`), two plan scales | Non-empty, true size (shoulders about 0.5 m, head, arms, face visible), inside the net floor, no clipping. One occupant per capsule type, so the 0.4 m spacing never overlaps. **Fixed**: the figure faced right, into the side wall; it now faces into the room (`occupant()` rotates the cast by -90 degrees). |
| bim `information` `_car()` (`vintage-car-1929`, plan) at headings 0, 270, 315 | Non-empty, bumpers, fenders, radiator, headlamps, spoke wheels at the fender ends all visible; nose leads along the heading; stretching to the model car size looks right; colour is the clip yellow. No change. |
| bim `consistency` people (reviewer x2, decider) | Grounded on the baseline, actor green, readable at 0.4 to 0.6 units. No change. |
| optimization room section winter and summer | Occupant grounded and orange, radiator flush on the rear wall (red), pendant lamps with bulb, halo and cone aligned, cooler on the rear wall (teal). No clipping. No change. |
| optimization experiments section (re-render that was never done) | Cooler now sits flush on the cut line at the top of the right wall in both rooms; occupant grounded; lamps lit by crossfade. No change needed. |
| optimization extrema icons (radiator, cooler, lamp) | Radiator red with hatch, cooler teal, lamp yellow; sized consistently. No change. |
| optimization index pictograms (lamp, coin, sun, factory) | All four render; the lamp is the staffage pendant, sun and factory are diagram icons. No change. |
| experiments scatter icons (occupant orange, tilted window white) | The occupant is fine. The tilt-window in section is a thin tilted sash, small but accurate; it reads as a line more than a window. Left as is, since the archive has no elevation window representation. |

## Fix made

- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`, `occupant()`: `.rotate(-np.pi / 2)` before `move_to`, so plan occupants face into the room instead of the right wall.

## Tests (`SEMIO_TEACHING_OUTPUT_ROOT` set)

| Suite | Result |
|---|---|
| optimization `🎬️clip/🧪️tests/🐍️.py` | 70 run, 69 pass, 1 fail (the known JSON schema `$schema` draft-07 assertion, unrelated to staffage; same as in M7) |
| excel `🎬️clip/🧪️tests/🐍️.py` | 85 run, 85 pass (run after the `occupant()` change) |
| bim `BimInformationContract` + `BimConsistencyContract` | Not finished: the run was killed at the 9 minute limit while building BIM models (host under load); no failure was reported before that. Not verified. |

Temp output `🗑️generated/r3*` removed.
