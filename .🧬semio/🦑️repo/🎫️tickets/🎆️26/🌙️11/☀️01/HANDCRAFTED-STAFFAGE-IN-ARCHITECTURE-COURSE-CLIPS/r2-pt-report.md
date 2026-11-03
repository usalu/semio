# R2 pt: physics fundamentals clip tests

New folder `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🧪️tests/` with `🐍️.py` (unittest) and `🔣️.cases.json` (all numbers and names). The clip itself was not changed; no defect found.

## Tests (13, all OK via `m9_tests.sh 🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️`, about 45 s)
- `StaffageCasts`: every literal and resolved cast of the clip is in the manifest with `view` thermal; every `physics-fundamentals-*` manifest cast is used by the clip; each scene group (`scene_spans` of the `staffage_only` guard, one per Scene class plus module level) has exactly one `thermalSpanK`, never absolute.
- `Beat1DeviceIcons`: the vacuum, kettle and shower `_fitted_staffage` calls are read from the Beat1 source with `ast` (kwargs and `move_to` centre) and built with `fitted_staffage` without rendering. Checks: all at the slot centre (4.8, 1.95) with `rate=0.0`, centre within 1e-6, height at most the 0.6 slot, width at most the declared width, one limit exactly reached, and drawn paths stay inside the device card (6.2 x 0.75 at 3.4, 1.95) at cast seconds 0, 10, 30, 150, 190.
- `Boiling`:
  - kettle: boil phase at 184 s (catalog clip `boil`) divided by `KETTLE_RATE` (60, from source) is less than the German clause budget of the `kettle` clause; the whole 200 s clip also ends inside it.
  - kettle physics (CoolProp): 1 kg from 15 to 100 degC divided by the 1930 W element power gives 184 s +- 5 s.
  - pot: water region `heatCapacityJM2K x areaM2` equals 1 kg (CoolProp mean specific heat, 1 percent).
  - pot 419 kJ: m c dT with 4186 J/(kg K) and CoolProp `H(100 degC, sat. liquid) - H(0.01 degC)` both equal 419 kJ +- 1; they agree with each other.
  - pot boil time 319 s lies between Q(room 21 degC to 100)/1173 W and Q(0 to 100)/1173 W (CoolProp), the water power is at most the 1200 W into the pot; `319 / POT_BOIL_RATE` (30.5) is 10.46 s scene time.
- CoolProp quirk: `T=273.15` is below the melting temperature at 1 atm, so 0 degC is evaluated at 0.01 degC, and 100 degC uses saturated liquid (`Q=0`) because 373.15 K at 101 325 Pa is already vapour.

## Registration and guards
- Appended `🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🧪️tests.🐍️` to `🧰️ht/modules.txt`.
- `staffage_only` (22 tests, 322 s): 15 OK, 6 failures and 1 error, none from the new tests, all from concurrent foreign work: `cooling-summer-strategy-collector` not in the manifest and a 25/70 span mix in `❄️cooling/.../🌤️summer-strategy`; `KeyError head_nouns` in the guard config; rule-a primitive allowlist misses for `_beam` (physics fundamentals line 322), optimization `ramp_strip`, `_plane_group` and bim `_plan_ramps`. Verify the physics `_beam` entry with whoever owns the allowlist once the others settle.

## Open
- The kettle beat budget is the clause reading floor (no measured TTS in the test), so it is a lower bound of the real beat duration.
