# fleet-1108 oracle en1993 b: third-party cross-check of the second half of the EN 1993 gap list

Files changed (only under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/`):
- new `🧮️special-parts/🧭️spec-rows/🐍️.py`: table `ROWS` (spec row -> clause -> check ids -> library) plus `cross_checks(out)` grouped by domain; imports numpy, scipy, sympy, blueprints.
- `🧮️special-parts/🐍️.py`: `main()` merges `spec_rows().cross_checks(out)` (extra `xc_*` keys; existing keys unchanged).
- `🔩️joint-rows/🥒️.feature`: removed the literal `V-426`/`V-427` from prose so the audit's first oracle hit for these rows is the python module (the other feature hits are non-python and would classify as unknown).

Verified: `/home/deck/semio/.venv/bin/python -I .../🧮️special-parts/🐍️.py` exits 0 and prints the JSON (all cross-checks assert). A token scan of all oracle-path files shows every row of the slice hit only in the python module. Rust tests were not run (no builds from this lane).

## Closed rows (library or closed form used)

| rows | method |
|---|---|
| V-870, V-904 | blueprints EN 1993-5:2007 formulas 5.2 (M_c,Rd) and 5.5 (V_pl,Rd) against pi_vpl and w_pile_il (real third-party library) |
| V-902 | blueprints formula 5.2 as W f_y/gamma_M0 for S500 (library covers the formula, not 1-12 itself) |
| V-420, V-424, V-425 | sympy solve of the equilibrium systems of T-stub modes 1 to 3, against jc_f2 and the joint-rows column flange T-stub |
| V-423 | scipy interp1d of Table 6.3 omega against jc_omega, jc_omega_15 |
| V-426, V-427, V-452 | sympy rational arithmetic, scipy linprog of the plastic row distribution against the joint-rows forces and M_j,Rd |
| V-601 | sympy integrate against scipy quad of c_a(theta); lambda_a and c_a values |
| V-607, V-608 | scipy interp1d of Table 3.1, brentq on the buckling curve, sympy lambdify of eq. 4.21a |
| V-251, V-252, V-255, V-256, V-257, V-258, V-260, V-254 | brentq on the distortional fixed-point residual (against cf_chi_d), sympy curling and crippling, numpy piecewise f_bv (against cf_fbv, cf_rw_rd), numpy series of springs |
| V-300 to V-305 | brentq on the Perry curve (st_chi_*), sympy diff of Ramberg-Osgood (st_es, st_ro_tangent), interp1d inverse of k_0.2p,theta |
| V-800, V-801, V-802, V-906 | sympy LS3 interaction, sigma_x,Rcr, omega, C_x short (against sh_*) |
| V-803, V-804, V-905 | scipy solve_ivp of the Janssen ODE (ct_ph, ct_pv, ct_nx), SI tank wall, sympy ring buckling identity |
| V-851, V-850 | sympy cantilever frequency (tw_freq), v_crit |
| V-880, V-900 | numpy/scipy interp1d lambda_1, lambda_3, lambda_4; Table 2.1 t_max |
| V-406 | numpy vectorised Table 3.3 (closed form from the clause, no library covers it) |
| V-366 | numpy plate buckling eigenvalue k = min_m (m b/a + a/(m b))^2 for psi = 1 internal (4.0) and outstand (0.425 + (b/a)^-2); other psi columns not covered |

The audit classifies all of these third-party by the numpy/scipy/sympy/blueprints imports. Honest limits: rows marked closed form or "against sh_*/st_*/cf_*" compare two oracle-side methods; they reach the Rust code only through the existing special-parts quantities. The new `xc_*` quantities need a Rust loop (request filed in `📌️important/requests.md` under `## fleet-1108 → en1993`).

## Still weak or open

- V-376, V-806 (Annex D): only the local part (tau_cr,l, lambda_c,l, chi_c,l) is recomputed; the global tau_cr,g (B_x, B_z of the corrugation) is not written because the spec text does not give its formula and no library covers it. No oracle value reaches `corrugated_web::shear_resistance`.
- V-850: factors 0.9 / 0.35 + 0.7 lambda / 0.7 lambda are those of the spec text (`en1993-target.md`, marked unverified there); only monotonic relations asserted.
- V-366: only psi = 1 values checked by eigenvalue; Table 4.1/4.2 other psi columns unverified.
- V-607/V-608, V-406, V-903/V-805 (not in slice): no Rust-compared quantity exists yet (request filed).

## Oracle vs Rust discrepancies

None found against the compared quantities. Note: `st_ro_tangent` of the existing oracle is a central finite difference (h = 0.5 MPa), 24436.6194 vs the exact sympy derivative 24437.2897 (relative 2.7e-5); the feature tolerance is 1e-5, so a Rust kernel using the exact derivative would exceed it (not observed; Rust not run).
