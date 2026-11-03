# Wave 2 Oracles v2 (EN 1990 to EN 1999)

Python only (`/home/deck/semio/.venv/bin/python -I`), no build started. 99 new cases in 9 files (`third-party-v2-extended.json` in `🧪️tests/🧫️fixtures/🔮️oracle-cases/` of EN 1990, 1991, 1992, 1993, 1994, 1996, 1997, 1998, 1999) all green; harness pytest 51 passed (47 + 4 new laws, 2:13 min). The existing Rust test `third_party_libraries_reproduce_the_blessed_oracle_cases` picks the new files up from the directory (no Rust edit). Scripts (inputs, kept): `🧰️scripts/oracle-wave2-v2-gen.py`, `oracle-wave2-v2-bless.sh`; `oracle-wave2-register.py` now skips adapters without a package (the new `published` table adapter) and re-ran for the registries. Logs: `🗑️generated/oracles-v2/` (delete when done). EN 1995 got no new file (not in scope).

## Harness changes
- New adapter `🔌️adapters/published` (values printed in the standard or handbooks: ISO 834 table, EN 1990 Table D1), kind `independent-formulation`.
- groundhog adapter: `requests` joins the inert import stubs (only used by `soilprofile` for downloads), which makes `groundhog.shallowfoundations.settlement` importable; new checks `bearing-resistance`, `stress-strain-settlement`, `oedometer-settlement`, `embedded-wall`. Note: the library's Poncelet coefficients return NaN for wall friction delta below about phi/2 (the adapter falls back to the library's Rankine coefficients for delta = 0, cases use delta >= phi/2).
- numerical/sympy/sectionproperties/anastruct/pynite adapters: new checks listed below. `INDEPENDENT_ONLY` in `🧪️tests/🐍️.py` extended by the checks that no installed library computes.

## Covered now (check, references, kind)
| area | check | references | kind |
|---|---|---|---|
| EN 1991 | `4.wind-peak-pressure` (c_r, v_m, I_v, q_p; terrain II, III, IV, below z_min) | sympy (clause order), scipy form (algebraic L^2 + 7 k_I L) | independent |
| EN 1991 | `7.wind-cpe` (Table 7.1 vertical walls, h/d and area interpolation) | scipy interp1d, sympy Piecewise | independent |
| EN 1991 | `snow.shape-mono-pitch` (mu_1, mu_2), `snow.drift` | scipy interp1d/clip, sympy Piecewise/Min | independent |
| EN 1991 | `fire.parametric-curve` (Annex A heating, three cooling branches), `fire.iso834` | scipy, sympy exact rational, published ISO 834 table | independent |
| EN 1996 | `G.phi-m`, `6.2.shear-wall` (compressed length by brentq on the no-tension triangle) | sympy, scipy | independent |
| EN 1997 | `D.4.bearing-resistance` (N, s, b, i factors and R/A') | groundhog (N_q, Vesic N_gamma), scipy, sympy | third-party |
| EN 1997 | `F.stress-strain-settlement` (Annex F.1), `F.oedometer-settlement` (F.2) | groundhog (`stresses_rectangle`, `primaryconsolidationsettlement_nc`), scipy quad with Newmark | third-party |
| EN 1997 | `9.embedded-wall` (cantilever, free earth support), anchors `8.anchor-elongation`, `8.anchor-resistance`, piles `7.pile-group`, `7.pile-resistance` | groundhog + scipy + sympy; PyNite + anastruct + scipy; scipy+sympy; scipy | third-party / independent |
| EN 1999 | `6.3.1.flexural-buckling` (A, I_z, N_cr, chi, N_b,Rd) | sectionproperties FE section, sympy, scipy Euler FE eigenproblem + Ayrton-Perry root | third-party |
| EN 1999 | `6.3.2.ltb` (I_z, I_t, I_w, M_cr, chi_LT, M_b,Rd) | sectionproperties (I_t and I_w from the mesh), sympy 2x2 determinant, scipy Vlasov beam FE eigenproblem | third-party |
| EN 1999 | `8.5.pin-bending-moment` (pins in double shear) | anastruct, PyNite, scipy quadrature | third-party |
| EN 1994 | `6.6.3.stud-resistance` (P_Rd, k_t) | sympy, scipy | independent |
| EN 1994 | `6.4.ltb-restrained` (continuous inverted-U torsional restraint k_s) | sectionproperties, sympy, scipy Vlasov FE with distributed spring | third-party |
| EN 1994 | `6.2.1.4.mkappa-elastic` (M-kappa branch, strip integration) | sectionproperties composite I_eff, scipy strips | third-party |
| EN 1993 | `5.2.alpha-cr-storey` (Eq. 5.2), `5.2.alpha-cr-eigen` (eigenvalue vs P-Delta amplification), `5.3.2.sway-imperfection`, `1-8.4.5.weld-group-box` | PyNite + anastruct; scipy eigen + PyNite `analyze_PDelta`; scipy+sympy; sectionproperties thin box + scipy + sympy | third-party (sway: independent) |
| EN 1990 | `D.7.characteristic-from-tests` (k_n), `fatigue.miner`, `C.form-nonnormal` (lognormal R, Gumbel E; SLSQP vs Rackwitz-Fiessler vs exact P_f integral) | scipy.stats + published table; scipy; scipy | third-party / independent |
| EN 1992 | `F.membrane-reinforcement` (Annex F by minimising over the strut angle), `J.3.corbel-truss`, `KK.age-adjusted-modulus` (Volterra relaxation) | scipy+sympy; anastruct truss + sympy + scipy; scipy | independent / third-party |
| EN 1998 | `3.1.vs30` (travel-time integration, Table 3.1), `4.3.5.2.nonstructural` | scipy, sympy | independent |

## Findings
1. Nothing in the present Rust disagrees with the new oracles: wind q_p and c_pe, mu_1/mu_2 and drift, parametric fire (including fuel-controlled branch), ISO 834, Phi_m, Annex D bearing (Rust matches including i-factors; only the depth factors are extra), Miner, Table D1, vs30, sway imperfection, stud P_Rd and the EN 1999 curves (class 1/2: 0.10, 0.6; class 3/4: 0.20, 0.4) were read against the blessed expectations (the Rust evaluators do not consume `inputs`/`expected` yet, so this is a read comparison, not an execution).
2. Observations sent to the owners (not defects): EN 1993 Eq. 5.2 is 17 percent (stiff beam) and 12.5 percent (flexible beam) above the eigenvalue/P-Delta alpha_cr of the same fixed-base portal (11.70 vs 9.97, 7.19 vs 6.39; scipy eigen and PyNite P-Delta agree to 0.3 percent); EN 1992-2 Annex KK chi from the Annex B creep law is 0.82 to 0.86 (default NDP 0.8); EN 1990 Table D1 prints 1.76 for n = 20 where the Student prediction factor is 1.772; EN 1997 `include_depth` adds a depth factor that Annex D of EN 1997-1 does not contain.
3. Older findings (LM1 tandem 43 percent unsafe, EN 338 table rows) stay open in the v1 report.

## Still without a third-party check, and why
- Wind NA-DE profile (tabulated gust profiles, no library), c_pe for roofs and other figures (tables only), snow shape of cylindrical/multi-span roofs, Annex E fire (delta_q factors), thermal and silo patch loads.
- EN 1996 Annex G Phi_m has no independent mechanics model: a second-order beam-column model with a masonry stress-strain law was not built (Annex G is itself a fit to such a model; a comparison needs a published masonry law). Shear walls: only the compression-length shear form.
- EN 1997 Annex C log-spiral K_p, pile skin friction methods, slope stability; embedded-wall check ignores the partial-factor sets and the net-pressure reduction of Blum.
- EN 1999 HAZ reduction (kappa) and local buckling, EN 1994 6.2.2 vertical shear and slab cracking, EN 1993 built-up members (S_v), cold-formed, joints (component method), fatigue beyond Miner.
- EN 1998 timber/masonry member checks (not in the artifact yet), torsion 4.3.3.2.4.
- EN 1992 annex parts T-05 (warping torsion), AJ-02 (mandrel diameter), AL-01 sandwich model, Dischinger redistribution; EN 1990 Annex D.7 test-based design beyond k_n, FORM beyond two variables.
- EN 1995: untouched in v2.

## Notes for owner agents
- Feed `inputs` (SI) to the evaluator and compare `expected.*.value` under `expected.*.profile` (a few carry an explicit `tolerance`). Check ids are oracle-level ids; map them to the Rust check ids in the owners' tests.
- `5.2.alpha-cr-eigen`, `C.form-vs-exact-*` carry deliberate method spread tolerances; do not tighten them.
- The `🔒️dependencies.json` baseline still lacks all oracle libraries (needs the native `repo verify` command).
