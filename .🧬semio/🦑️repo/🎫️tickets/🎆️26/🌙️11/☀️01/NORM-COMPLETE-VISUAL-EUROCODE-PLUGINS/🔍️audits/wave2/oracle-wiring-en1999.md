# 🎯️ Oracle wiring EN 1999

New test module `🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`, mounted as `mod oracle_wiring` in the (previously empty) `ComplianceTests` region of `⚖️checks/🦀️.rs`. The existing `third_party_libraries_reproduce_the_blessed_oracle_cases` lives in the root artifact crate (`🪶️en1999/🦀️.rs`, manifest above the leaf), so the driver cannot walk up from there; the checks crate sits below the leaf and already has `compliance-testing`. The old test is untouched.

## Counts
- Cases: 20. Checks: 11 distinct, 9 wired, 2 without adapter (tee, ehs). Quantities compared: 81. Known differences: 0. Gap cases: 7.

## Wired checks and entry points
| check | evaluator |
|---|---|
| en1999.section.i | `part_1_1::section_geometry`, `plastic_moduli` (equal flanges only; the case has them) |
| en1999.section.rhs-sharp, .chs, .channel, .angle | `section_geometry`, `plastic_moduli` |
| en1999.section.polygon-holes | `polygon_props` (outer ring minus holes) |
| en1999.6.3.1.flexural-buckling | `section_geometry`, `n_cr`, `lambda_bar`, `chi_from_lambda`; N_b,Rd = χ A f_o/γ_M1 composed from our χ and A |
| en1999.6.3.2.ltb | `section_geometry`, `m_cr_ltb` (C1 = 1), `chi_from_lambda`; λ_LT = sqrt(W_el,y f_o/M_cr) and M_b,Rd composed from our M_cr, χ_LT, W_el,y |
| en1999.8.5.pin-bending-moment | `joints::pin_bending_moment` |

## Gaps (no entry point)
- `en1999.section.tee-extruded-60` (A, Iy, Iz, It, Wpl_y) and `en1999.section.ehs-extruded-120x80` (A, Iy, Iz, It): `section_geometry` knows no tee and no elliptical hollow kind (an unknown kind falls into the I-section branch, which would be wrong). Suggested: kinds `tee`, `ehs` in `part_1_1::section_geometry`.
- `en1999.section.channel-extruded-100` quantity `y_sc` (shear-centre offset) and `en1999.section.angle-extruded-60x6` quantities `I_major`, `I_minor` (principal axes): suggested `part_1_1::shear_centre_offset(section)`, `part_1_1::principal_moments(section)`.
- `en1999.section.free-outline-{solid,box,i}-torsion` (3 cases): `It`, `Iw` of a free outline; `outline_checks::PolyProps` has no torsion constants. Suggested: `outline_checks::torsion_constants(rings) -> (it, iw)`.

## Expected visible mismatches (hand-derived, no `[open] oracle` note)
- I-section `It` (ours 7.96e-8, blessed 7.75e-8: flange-web junction ignored, 2.7 percent against 0.5 percent); channel A, Iy (no root radius, 0.6 / 0.7 percent); angle Iy (the closed form is not centroidal; 53 percent); pin bending moment (ours F b/8 for a = c = 0: 125 N m against 424.9 N m; the EN 1999 8.5 form F/8 (b + 4c + 2a) is not implemented); LTB `It` (ours 7.96e-8 against 7.73e-8, 2.999 percent, borderline against the 3 percent spread). Everything else is expected to agree.

## Files changed
- created `...⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`
- edited `...⚖️checks/🦀️.rs` (mount). Cargo.toml unchanged.

## Assumptions / unverified compile points
No build was run. `AluminiumSection` is built field by field (id, kind, height, width, flange_thickness, web_thickness, outer_diameter, elements). Polygon rings are read from `Inputs::json` with `serde_json::Value` methods only (no `serde_json` dependency named in the crate).
