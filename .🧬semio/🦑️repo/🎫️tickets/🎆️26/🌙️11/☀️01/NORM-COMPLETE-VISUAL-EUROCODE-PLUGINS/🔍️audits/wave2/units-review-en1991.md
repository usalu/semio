# Units review of the EN 1991 design-case vocabulary

Scope: every medium and low confidence row of the EN 1991 vocabulary, plus the keys that mean different things in different entries. Evidence is the registry code under `🗿️artifacts/🏋️en1991/…/⚖️checks/📇️catalogue/` (`CAT`): `🧱️gravity` (E11), `🔥️fire` (E12), `❄️snow` (E13), `🌬️wind` (E14), `🌡️actions` (E15 to E17), `🌉️structures` (E2 to E4) and the helpers in `…/⚖️checks/🦀️.rs` (`H`). Line numbers are of the function in its file (`fn` line), `H:` numbers are lines in the helper file.

Result: 212 rows became 228 rows (98 entries; the coverage script finds no missing key), `python -I` shared oracle accepts all 228, evaluators untouched, nothing compiled.

## Mechanics

- TSV edited through `🧰️scripts/catalogue-bridge-en1991-units-review.py` (one-off, kept as input); the TSV is the source of truth now. `catalogue-bridge-en1991-build.py` now exits at the top so it cannot discard the review.
- Generator `catalogue-bridge-gen-inputs.py` extended (backward compatible): unit tokens `per_m` (1/m), `W_m2K` (W/(m²·K)), `J_m2sK` (J/(m²·s½·K)); scoped choices through `@choice key@scope|…`; `@alias key|source` shares a choice set (material2, material3).
- Regenerated with `python -I catalogue-bridge-gen-inputs.py <📚️inputs dir> catalogue-bridge-en1991-inputs.tsv s.norm.en1991`, then `catalogue-bridge-check-coverage.py en1991 <📚️inputs dir>` and the oracle `🔮️oracles/🐍️.py <🔣️.json>`.
- All reviewed rows are now confidence `high` except `a@E3.03` and `m_drive` (medium).
- The vocabulary had no `example` at all, so the demo cases were empty. Examples were added to the reviewed rows (DE annex values: the demo document evaluates with `AnnexChoice::De`), plausible in the stored unit. High rows that were not reviewed still have none (b, h, width, thickness, speed, …), so several demo cases remain not-applicable.

## Unit corrections (the numbers stored change meaning)

| key | scope | old | new | evidence |
|---|---|---|---|---|
| s_b | all | m (crane span) | N/m buffer spring stiffness S_B | `buffer_force_n(phi_7, v_1, mass_kg, s_b_n_m)` H:1032, used in `e3_03` CAT/🌉️structures:179 |
| m_drive | all | 1 (number of driven wheel pairs) | N·m moment M | `e3_03`: `H_T = xi*m/a` printed as kN, `a` in m, xi dimensionless |
| p_v | all | Pa | kPa | `e17_09` CAT/🌡️actions:188: `(3.0 + pv) * 1000.0`, `gas_explosion_pd_kn_m2` H:866 (3 + p_v/2 + 0.04/(A_v/V)²) |
| av_over_v | all | 1 | 1/m | same formula, 0.04/(A_v/V)² in kN/m² needs A_v/V in 1/m |
| k | E17.08 | 1 | N/m (new scoped row) | `e17_08` `v*(k*m).sqrt()` N with m kg, default `vehicle_stiffness_n_m` H:852 = 300000 N/m (ndp vehicle-stiffness) |
| k | E4.03, E4.04, E4.05 | 1 | 1, lateral pressure ratio K | `silo_inputs` CAT/🌉️structures:212 (`1.1*(1-sin φ)`) |
| delta_b, delta_c | all | 1 (factors) | m (deformation of barrier / vehicle) | `e11_05` CAT/🧱️gravity:74: `F = ½mv²/(δ_c+δ_b)` in N |
| alpha | E13.03, E13.08, E13.13 | ° | ° (roof pitch) | `mu_1(alpha_deg)` H:343, `snow_guard_force_n_m` H:389 uses `alpha_deg.to_radians()` |
| alpha | E15.03, E15.05, E15.06 | ° | 1 (thermal expansion coefficient in 1/K, label carries the unit) | `restraint_stress_pa(e, a, dt)`: σ = E α ΔT; `expansion_m(alpha, dt, l)`; code uses 10e-6/12e-6 in `e15_01` |
| alpha | E2.14 | ° | 1, range 0.75 to 1.46 | `e2_14` CAT/🌉️structures:112 accepts only 0.75 … 1.46 |
| alpha | E2.15 | ° | 1 (classification factor) | `e2_15` :121 multiplies the SW load |
| alpha_c | all | 1 | W/(m²·K) | `UnitId::WattPerSquareMetreKelvin` exists; `e12_06` CAT/🔥️fire:102 default 25 W/m²K |
| b | E12.02, E12.04 | m | J/(m²·s½·K) (UnitId exists), range 100 to 2200 for E12.02 only | `e12_02` CAT/🔥️fire:19 (`100.0..=2200.0` J/m²s^½K); `e12_04` :70 uses 720/2500 thresholds, no range |
| b | other entries | m | m (width, unscoped row stays) | E11.08, E11.09, E13.x, E14.x, E2.x |
| e | E15.03, E15.05 | Pa | Pa (scoped) | `restraint_stress_pa`, σ printed `/1e6` MPa |
| e | E14.08 | Pa (clash) | m reference dimension | `flat_roof_zone_areas(e_m, b_m, d_m)` H:500 |
| s | E13.07, E13.08, E13.13 | m | Pa snow load | `e13_07` CAT/❄️snow:87 `d = s/3000`, `overhang_line_load_n_m(s_pa)` H:381 |
| s | E17.06 | m | m tie spacing | `tie_force_kn(..., s_m, l_m)` H:882 |
| s | E14.03 | m | 1 orography factor s | `orography_factor(phi, s)` H:430: `1 + 2 s Φ` |
| phi | E14.03 | ° | 1 slope Φ = H/L | `orography_factor` H:430 (Φ thresholds 0.05, 0.3) |
| phi | E4.02 to E4.05 | ° | ° friction angle | `to_radians()` in `silo_inputs`, `e4_02` |
| z | E13.02 | m | 1 CEN zone number Z | `e13_02` CAT/❄️snow:23, `(0.642*z+0.009)` in kN/m² |
| z | other | m | m | unchanged unscoped row |
| n | E14.14 | 1 | Hz | `vortex_critical_velocity(b, n_hz, st)` H:582 (b·n/St in m/s) |
| v | E13.10 | m/s | 1 coefficient of variation V | `return_period_factor(v, p)` H:391 |
| p | E14.06, E16.03 | 1 | 1, range 0 to 1 | `e14_06` CAT/🌬️wind:67 rejects p outside (0, 1) |
| p_h, p_d_h | E17.07 | 1 | 1, range 0 to 1 per element | probabilities |
| mu | E4.03 to E4.05 | 1 | 1, relabelled wall friction (scoped) | `silo_inputs`: `tan(0.75 φ)` |
| mu | E13.01, E13.12 | 1 | 1, shape coefficient (scoped) | `e13_01` |
| r | E12.05 | m | m, label "horizontal distance" | Hasemi `y = (r+h+z')/(l_h+h+z')` |
| r | E14.11 | m | m, label "corner radius" | `psi_r = 1 - 1.9 r/b` |
| d | E12.05 | m | m, "diameter D of the fire" | `e12_05` |
| d | E14.07, E14.08 | m | m, "depth d of the building" | `wall_zone_widths(e, d)` |
| a | E3.03 | m | m, "distance of the guide means" (medium) | `H_T = xi*m/a` |
| a | other | m | m, "contact length" | `e11_08`, `e11_09`, `e2_03`, `e2_06` |
| c_w, c_h | E4.04 | 1 | 1, relabelled "discharge … factor" | `e4_04` CAT/🌉️structures:265, defaults 1.10 and 1.15 |
| v_1 | E3.03 | m/s | m/s, label "buffer impact speed" | `buffer_force_n` H:1032 |
| finish | E11.02 | text | N/m number | `e11_02` CAT/🧱️gravity:30: `... + c.o("finish", 0.0)` added to the line load |
| class | E11.07 | text | 1 number, range 1 to 6 | `e11_07` :101 `c.n("class") as u8`, `_ => Err("forklift class must be 1 to 6")` |
| deck_type | E15.01 | text | 1 number, range 1 to 3 | `e15_01` CAT/🌡️actions:7 `c.n("deck_type") as u8`, 1 steel, 2 composite, 3 concrete |
| coast | E14.02 | text | 1 number, range 0 to 1 | `e14_02` CAT/🌬️wind:28 `c.o("coast", 0.0) > 0.5` |
| zone | E14.01, E14.02 | text | 1 number, range 1 to 4 | `e14_01` :9 `c.n("zone")`, `WindZone::from_u8` H:154 |
| zone | E13.01, E13.02, E13.09 | text | choice 1, 1a, 2, 2a, 3 | `SnowZone::ADMISSIBLE` H:142 |
| zone | E14.08 | text | choice A to J | `face_cpe_candidates` H:562 (walls A to E, roofs F to J) |
| lane | all | 1 | 1, range 1 to 3 | `e2_10` CAT/🌉️structures:78 |
| terrain | all | 1 | 1, range 0 to 4 | `terrain()` CAT/🌬️wind:6, `TerrainCategory::from_u8` |
| water_supplies | all | 1 | 1, range 0 to 2 | `delta_n` CAT/🔥️fire:43 |
| alpha_v | all | 1 | 1, range 0.025 to 0.25 | `e12_04` CAT/🔥️fire:70 |
| alarm, heat_detection, smoke_detection, sprinkler, onsite_brigade, offsite_brigade, safe_access | all | 1 | 1, range 0 to 1 (flag, `> 0.5`) | `delta_n` |
| margin_k | all | 1 | K | `expansion_m(a, dt + margin, l)` CAT/🌡️actions:61 |
| odd confirmations | | | | `q_w` W (`local_fire(d, q_w, z)` H:772, `0.0148 Q^0.4`), `m_e` kg/m (Scruton), `g_k`/`q_k` Pa (`/1000` kN/m² in E17.06), `q_ca` Pa (default 1000), `gamma` N/m³ (`fresh_concrete_pressure_pa` H:845), `weight` N (`5*w`), `mass` kg (`3000*√m`), `deck_acc` removed (no entry reads it; the EN 1990 key of the same name is another vocabulary) |

Rows confirmed unchanged in unit (now high, with examples): a_f, a_fr, a_over_u, area, axle, axle_load, axles, b1, b2, beta_3, capacity_t, cover, depth, dm_over_m, eccentricity, f_pl, gamma, i, l, l_f, l_h, lambda, load, m_c, m_e, mass, pavement, pitch, provided_tie, q_c, q_ca, q_ca_concreting, q_ci, q_h, q_k, q_max, q_v, q_vk, q_w, r_acc, rpm, runway_beams, sum_q_r, t_min, temps_c, times_min, wall_gamma, wall_load, weight, wheels, xi, y_allowed, y_el, z_g, z_prime.

## Text keys converted to choice rows (EN and DE labels)

| key | values | evidence |
|---|---|---|
| category | A, A1 to A3, B, B1 to B3, C1 to C6, D1 to D3, E1, T1, T2, Z, F, G, H (union of both annex lists) | `categories` H:240; the evaluator rejects the other annex' codes |
| material, material2, material3 | 25 materials (24 of `MATERIALS` H:208 plus `clay`, accepted by `density_kn_m3`) | `material_gamma` CAT/🧱️gravity:5 |
| occupancy | dwelling, residential, office, hospital, school, hotel, library, shopping, mall, theatre, transport | `OCCUPANCIES` H:755 |
| tyres | pneumatic, solid | `e11_07` |
| curve | standard, external, hydrocarbon | `e12_01` |
| cen_zone | alpine, central_east, mediterranean | `e13_02` |
| state | fresh, settled_days, settled_months, wet | `e13_11` |
| roof_type | flat, monopitch, duopitch, hipped | only `contains("flat")` is read (`face_cpe_candidates`); the other values are the model's roof kinds |
| mode, structure | framed, core, cantilever, lattice; concrete, steel, mixed | `e14_15` |
| deck, surfacing | steel, composite, slab, box, beam; unsurfaced, waterproofed, 50mm, 100mm, 150mm, ballast | `e15_02` |
| road | motorway, urban, courtyard_cars, courtyard_lorries | `e17_01` |
| member | deck, pier | `e2_11` |
| direction | vertical, horizontal, jogging | `e2_13` |
| group | gr1a, gr1b, gr2, gr3, gr4, gr5 | `GROUPS` H:961 |
| model | sw0, sw2 | `e2_15` |
| hoist_class | HC1 to HC4 | `e3_01`, `phi_2_params` H:1013 |

## Missing units (kept at `1`, unit in the label)

- 1/K (thermal expansion coefficient α, E15.03/05/06): no `PerKelvin` in `UnitId`.
- 1/min (rpm of E3.07): no rotational-speed unit.

## Remaining uncertain rows

- `m_drive` (E3.03): the old label said "number of driven wheel pairs"; dimensional analysis of `H_T = ξ·M/a` (result N, `a` m) forces a moment, but which EN 1991-3 quantity M is (clause 2.7.4) is not stated in the code. Medium.
- `a@E3.03`: the distance of the guide means is inferred from the same formula. Medium.
- `category`: the choice list is a union; picking a code of the other annex yields an evaluator error ("unknown category … admissible: …"), choices cannot depend on the annex.
- `roof_type`: only "flat" versus not-flat is read; the pitched values are cosmetic.
- `q_w`, `a_fr`, `capacity_t` examples are rough demo values, not standard values.
- `g_k`, `q_k` in E12.07: only the ratio η_fi is computed, so any consistent unit works; Pa kept.
- Entry count is 98 (E13.13 exists in addition to the 97 of the brief); the coverage script is clean for all 98.
