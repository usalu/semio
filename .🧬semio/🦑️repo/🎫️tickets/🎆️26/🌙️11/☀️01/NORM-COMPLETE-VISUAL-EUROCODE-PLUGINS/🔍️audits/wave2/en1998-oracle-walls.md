# EN 1998 check-outcome oracle: wall thrust, water, sliding (wave 2)

## Done
- Oracle `🔮️oracles/🚦️check-outcome/🐍️.py`: `mo_active` (Eq. E.4), `wall_thrust` (Annex E: dry layer, submerged layer with gamma' and theta_w for permeable/low permeability, hydrostatic, Westergaard 7/12 k_h gamma_w H^2 for permeable, larger total of 1 +- k_v), `retaining_walls` (`.thrust`, `.water`), `foundation_sliding` (`.sliding`, 5.4.1.1: H_Ed = F_b of the first system of the supported building, H_Rd = (V-u) tan(delta)/gamma_Rd + passive). Cross-check: `wall_thrust` reproduces the independent compliance twin (117191.1716 / 111725.8374 N/m, permeable / low permeability) exactly.
- Tie-beam (`.tie`, `.displacement`) and slope (`.slope`) rows already existed (pass-59..64); no addition needed.
- Fixtures (script `🧰️scripts/en1998-oracle-walls/fixtures.py`): new `seismic-walls-pass`, `seismic-walls-fail` (3 walls: permeable+water, dry, low-permeability+water; 2 foundations). All 11 existing check-outcome fixtures lacked the five new required fields (wall `waterHeightM`, `saturatedGamma`, `permeabilityMS`; foundation `baseFrictionAngleDeg`, `basePorePressurePa`); completed in schema order with 0.0 (outcomes unchanged).
- Feature rows pass-65..70 and fail-71..75 in the check-outcome feature.

## Gates
- Check-outcome dry run (heavy.sh): 75 ok, 0 failed, 0 NO HANDLER. Fixture checker en1998: 335 scenarios, 0 failures.

## Rust vs oracle
- No numeric disagreement on the fixtures. Differences by reading (Rust not changed, not compiled):
  1. Rust `building_base_shear_n` (feeds `.sliding` H_Ed) uses `behaviour_factor` without the Table 5.1 q cap and the 0.8 elevation-irregularity factor that the oracle (and the Rust `baseShear` check) apply. Fixtures use regular elevation and q = 5.85 = q_max, so both agree. For an irregular or over-q building the sliding demand in Rust is lower than the clause-based value (unconservative); recommended shared fix.
  2. Permeability class boundary: Rust `>=` limit 5e-4 m/s (limit recalled, not in the spec); the oracle uses the same, no fixture at the boundary.
  3. Unsupported by the oracle: `.thrust` Fail without utilisation (no limit equilibrium).
