# Refactor: reference chip and running-example constants

## A. Reference chip

Variants found (all named `_din_ref`, 24 scene files):
- `demand` (7 scenes): width-aware (`title` argument, flows at ` · `, right-aligned lines, scales into the room), 9.0 room without title.
- `heating`, `cooling`, `ventilation`: single line `Text(...)` (heating, cooling, `natural`/`mechanical` ventilation variants differ only in docstrings; `mechanical` adds `.shift(DOWN * 0.6)`).
- `physics` has no chip.

Now one implementation: `reference_chip(text, title=None, *, lower=0.0)` in `🏗️authoring/🎨️visuals/🐍️.py` (next to `chip`, `window_chip`).
All 24 scenes bind it (`reference_chip = _manim_module.reference_chip`, right after `P_PRIMARY`), call it, and have no local copy. `mechanical` passes `lower=0.6`. READMEs of ventilation and demand clip now name `reference_chip()`.

Test (written first): `visuals/🧪️tests/🐍️reference_chip.py` with cases in `🔣️reference_chip.json`; the oracle is Manim's own `Text` placed with `to_corner(UR, buff=0.30)`.

Why fitting texts are pixel-identical: the shared function builds `body_text(size LABEL_FONT_SIZE-3, P_PRIMARY)`, which equals the bare `Text(...)` of the old heating/cooling/ventilation chips (`apply_body_font` sets the same face, line spacing and no ligatures as `Text` defaults). A text that fits yields one line; the 1-element `VGroup` has the same bounding box, `set_opacity(0.72)` and `to_corner(UR, buff=0.30)` are unchanged, `shift(DOWN * 0.0)` is exact. The test asserts corner, size, opacity and colour equal the plain label to 1e-9. No render was started.

Known deviations (texts that do not fit on one line today, measured with the final profile):
- Texts that `cite()` already breaks with `\n` (heating internal-gains, solar-gains g-value beat, one cooling beat with two norms; and the withdrawn-norm status chip): previously two left-aligned lines of one `Text`; now flowed with ` · ` into right-aligned lines (joined to one line when it fits in 9.0).
- Ventilation `natural` Maas source in English (9.59 wide, > 9.0): now wrapped/scaled into the room instead of running wider than 9.0.
These are the cases where the old single-line variant could overflow; they are the only picture changes and follow from the one width-aware implementation. Demand scenes are unchanged (same code moved).

## B. Running-example constants

- `📏️running-example/🔣️.json`: `gains.utilisationFactor = 0.9`.
- `📏️running-example/🐍️.py`: `ETA_UTILISATION`, `Q_LOSS_SHOWN_KWH_PER_A` (16000), `Q_USE_KWH_PER_A` (4500), `Q_H_KWH_PER_A` (11500).
- `demand-power`: local `ETA_UTILISATION = 0.90` and the derivation removed; `Q_H_KWH_A = _example.Q_H_KWH_PER_A` (name kept, the chain test reads it).
- Heating `🧮️calculation`: `_solve_solar_peak(target=_example.ETA_UTILISATION)`, `_ETA`, `_Q_USE`, `_Q_H`, `_Q_LOSS_SHOWN` read from `_example`; the unused `_ETA_LIGHT/_ETA_HEAVY` lines are gone. Measured before the change: `_ETA=0.9, _Q_USE=4500.0, _Q_H=11500.0, _Q_LOSS_SHOWN=16000.0`, identical values after (same float ops), so on-screen numbers and rounding are unchanged.
- Not changed: the literals `16\,000`, `4\,500`, `11\,500`, `0.90` inside `NARRATION` clauses. The runtime reads `NARRATION` with `ast.literal_eval` (`authoring/🎬️clip/⚙️runtime`), so f-strings are impossible there. Guarded instead by the new test `🔥️heating/🎬️clip/🧮️calculation/🧪️tests/🐍️.py` (narration quotes the shared values in en and de; scene constants equal the shared ones).
- Test extended: `running-example/🧪️tests/🐍️.py::test_annual_heating_demand_is_the_losses_minus_the_utilised_gains` (`Q_H_KWH_PER_A == 11500.0`).

## Tests (uv, final profile, language en; output in `🗑️generated/refactor-chip/`)

220 tests in 27 modules (visuals tests incl. integration and reference_chip, running-example, heating calculation, demand chain and guard, ventilation architecture, narration guard, fonts, physics fundamentals): 218 pass, 2 fail, both outside this change:
1. `visuals…integration.test_physics_scene_constructor_bounds`: radiator height 0.7384 vs 0.676 (physics scene edited by someone else).
2. `demand…chain.test_counted_units_use_the_singular_for_one`: expects English "month", got "Monat" (language state of a combined single-process run; not touched here).
I did not re-run them in isolation.

## Files changed
- Added: `visuals/🧪️tests/🐍️reference_chip.py`, `visuals/🧪️tests/🔣️reference_chip.json`, `🔥️heating/🎬️clip/🧮️calculation/🧪️tests/🐍️.py`.
- Edited: `visuals/🐍️.py`, `running-example/🐍️.py`, `running-example/🔣️.json`, `running-example/🧪️tests/🐍️.py`, `demand-power`, heating `calculation`, ventilation and demand `🎬️clip/README.md`.
- Scenes (local chip removed, `reference_chip` bound): cooling (solar-radiation, heating-comparison, summer-strategy, internal-gains, transmission-humidity, system-sizing); ventilation (mechanical, need, heat-flow, volume, handoff, natural); demand (system-losses, regulations, demand-power, primary-energy, certificate, recap, practice); heating (solar-gains, convection, internal-gains, conduction, fundamentals). In `system-losses` and `certificate` only the `_din_ref` definition, its binding and its call sites changed, no `_regroup` code.
