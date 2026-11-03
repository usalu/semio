# W3-THIRDPARTY-2: third-party cross-checks for EN 1990, 1991, 1993, 1994, 1995, 1998, 1999 (EN 1996 none)

## 1. Provisioning
- Mechanism: `/home/deck/semio/pyproject.toml`, `[dependency-groups] test`, locked by `uv.lock`, venv `/home/deck/semio/.venv` (Python 3.14). Already declared and installed: anastruct 1.7.0, PyNiteFEA 3.0.0, sectionproperties 3.10.2, structuralcodes 0.7.3, concreteproperties 0.8.0, blue-prints 0.8.0, scikit-fem 12.0.2, scipy 1.18.0, sympy 1.14.0, numpy 2.5.0, rainflow 3.2.0, groundhog 0.15.0. Oracle adapters exist for anastruct, pynite, sectionproperties, structuralcodes, concreteproperties, blueprints, groundhog, numerical (scipy), sympy.
- Nothing installed or added. Missing candidates (not installed, would need an exact pin in the `test` group and a `uv lock` run with network): `eqsig` (response spectra, EN 1998 3.2.2.2 elastic spectrum from a record), `pyrotd`, `openseespy` (modal and spectrum analysis of frames). They need a recorded ground motion or an own input model; the EN 1998 design spectrum is a closed formula and stays sympy-only. Note: anastruct, PyNiteFEA and scikit-fem are declared with `>=`, the other third-party libraries with `==`; `uv.lock` pins the installed versions, so the pins are exact in effect.
- No library models: EN 1990 Annex C reliability beyond the existing scipy checks, EN 1991 wind, snow, fire formulas (sympy/scipy mirrors only), EN 1996 masonry (no Python library; only the scipy flexure mirror), EN 1993 buckling chi / joints (structuralcodes has no EC3; blue-prints covers only single clauses already wired), EN 1995 connections, EN 1998 spectra.

## 2. Added fixtures (`third-party-w3-2.json` in each oracle-case folder, registered as a row in each `oracle-comparison` feature)
Every case clones the check, quantity names, profiles, tolerances and reference set of an existing wired case with new inputs, so no Rust wiring change is needed; values were blessed by the first reference through the harness, which fails on any spread between the references (numerical-v1 profile).

| Code | Cases | Checks and libraries |
|---|---|---|
| en1990 | 4 | `mechanics.beam-span-envelope` (PyNite, anastruct): equal 4 m, 6+9 m, 8+5 m, gamma_G,inf 0.9 |
| en1991 | 8 | `mechanics.lm1-tandem` spans 4.2, 12, 20.4; `rail-lm71-mid-moment` spans 20, 32; `tandem-section-envelope` (9 m x 2.7, 18 m x 5.4, 24 m x 12 m), PyNite, anastruct, scipy |
| en1993 | 17 | sectionproperties: HEA200, HEB400, IPE200, IPE450, IPE600, RHS 300x200x10 and 100x100x5, CHS 273x8 and 60.3x3.2, PFC300, L150x12, mono-symmetric I; portal sway x3 and alpha_cr eigen x2 (PyNite, anastruct, scipy) |
| en1994 | 7 | composite-elastic x3 (sectionproperties, scipy; the rounded HEA300 only sectionproperties), two-span continuous beam x2, ss-beam, span envelope (PyNite, anastruct) |
| en1995 | 8 | beam-continuous x4 incl. asymmetric midspan forces, span envelope (PyNite, anastruct); EN 1995 7.5 natural frequency x3 (blue-prints) |
| en1998 | 4 | `mechanics.lateral-frame` 2 and 3 storeys (PyNite, anastruct) |
| en1999 | 10 | flexural buckling x4 (sectionproperties A, I_z with N_cr, chi, N_b,Rd from sympy and scipy), sections I, mono-I, CHS, channel, tee, angle (sectionproperties) |
| en1996 | 0 | no third-party library applicable |

Total 58 cases; all agree across their references (one run per code). Script: `🧰️scripts/w3-thirdparty-2-gen.py` (`--write [codes]`; the run takes about 1 to 2 minutes per heavy code, the en1991 envelope sweeps use 4 to 8 CPU minutes, run it with `OMP_NUM_THREADS=2`).

## 3. Differences above 1 percent found while building (none left in the committed files)
1. EN 1994 composite-elastic with fillet radius r > 0: the scipy mirror ignores the radius (A_eff 1.5 %, I_eff 4.3 % apart at HEA300, r = 27 mm). The existing rounded case already used sectionproperties only; the new rounded case does the same. Library vs ours: pending rebuild.
2. `delta_0` (maximum deflection) of two UNEQUAL spans: PyNite 2.005e-5 against anastruct 1.966e-5 (2.0 %). The anastruct adapter samples the deflection at the two elements per span, the true maximum is off midspan for unequal spans. Adapter sampling limit, not a standard question; the quantity is left out of `en1994.mechanics.w3-2-two-span-unequal-spans-udl`. (Equal spans and the pattern cases agree to 1e-9.)
3. Tandem section envelope at x or span not a multiple of the 0.3 m axle travel step: PyNite (step grid) vs scipy (exact positions) differ by 1 to 3 % (V_max 276 vs 282 kN at x = 10 m on 20 m). Grid effect; cases use grid multiples (as the existing ones). Same for rail LM71 at span 10 m (anastruct 0.83 % below the 0.4 m step grid): replaced by span 20 m.
4. Flexure cases are not touched here (EN 1992 is in W3-THIRDPARTY).

## 4. Status of the Rust side
No cargo run (wave rule). The new cases are of checks that are already wired (`oracle_wiring` per artifact selects by check name and reads every `third-party*.json`), so the new values are compared by the next rebuild. Library agreement is shown; "ours" against the library is open until then. Likely failure points to inspect: lateral-frame (4 new cases), tandem-section-envelope grid assumption, section checks for thin RHS (r_out 10 mm), Rust `rail-lm71` span 32. A failing case means either a Rust defect or an input outside the evaluator domain; do not weaken tolerances without a normative reason.

## 5. Files
New: 7 `third-party-w3-2.json` (en1990, 1991, 1993, 1994, 1995, 1998, 1999 oracle-case folders), `🧰️scripts/w3-thirdparty-2-gen.py`. Edited: the 7 `oracle-comparison-<code>-1/🥒️.feature` files (one Examples row each). Requests appended to `📌️important/requests.md`; rebuild lines to `🗑️generated/build-<code>/latest.md`.
