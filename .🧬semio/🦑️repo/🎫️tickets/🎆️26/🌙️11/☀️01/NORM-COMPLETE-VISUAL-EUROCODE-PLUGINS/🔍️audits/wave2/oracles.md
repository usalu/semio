# Wave 2 Oracles (EN 1990 to EN 1999)

Python side only (no repo build was started). 204 case checks green with `/home/deck/semio/.venv/bin/python -I ✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🐍️.py --cases <all artifact files + shared numerical.json>` (3.5 min), 38 pytest laws green (`🏃️harness/🧪️tests/🐍️.py`, 1.3 min). Baseline of wave 1 was 84 cases; this wave adds 117 (new files `third-party-extended.json`, `third-party-mechanics-extended.json` in each artifact `🧪️tests/🧫️fixtures/🔮️oracle-cases/`). Rebuild requested as "- ready for rebuild: oracle extended third-party cases" in `🗑️generated/build-en1990..en1999/latest.md`.

## What changed

- Adapters (all `🏃️harness/🔌️adapters/<lib>/🐍️.py`, library imported at module top): structuralcodes (+ creep and shrinkage Annex B, A_s,min, h_c,ef and rho_p,eff, s_r,max far, A_sw,max, T-section flexure; `GenericSection` replaced by `BeamSection`), concreteproperties (+ T-section flexure, reinforced masonry flexure with `RectangularStressBlock`; `materials()` split out), sectionproperties (+ tee, welded mono-symmetric I, EHS, composite steel-concrete elastic section with `Material`, EN 1999 aliases), blueprints (+ 28 clause formulas of EN 1992 chapters 3, 4, 6, 8 and EN 1993 chapter 6, the EN 338:2016 strength class tables 1 to 3), groundhog (+ Boussinesq strip, rectangle, circle; phase relations), sympy (+ EN 1998 design spectrum as Piecewise, closed-form FORM, characteristic polynomial of the shear building, `dsolve` of the Janssen slice equation), PyNite and anastruct (+ load arrangement envelope, multi-storey lateral frame, LM1 tandem sweep), scipy (+ T-section by strain compatibility, sharp RHS by rectangle difference, composite plastic neutral axis by brentq, EN 1990 combination enumeration, FORM by SLSQP, shear-building generalized eigenproblem, Janssen ODE by `solve_ivp`, Boussinesq quadrature, tandem influence functions, masonry flexure, composite elastic by parallel axes).
- Schema: citation standard now also admits `EN 338`. Harness test: independent-only cases are allowed only for an explicit list of checks that no library covers (`INDEPENDENT_ONLY`); five new laws.
- Rust: `third_party_libraries_reproduce_the_blessed_oracle_cases` added to EN 1991, EN 1996, EN 1998 and EN 1999 (file ends of `💡️inferences/🧪️tests/🔬️compliance-report` for 1991/1996, `🧪️tests/⚖️compliance/🦀️.rs` for 1998, `✅️compliance-gates/🦀️.rs` for 1999); thresholds of the other six raised to the new case counts. The test runs the harness over the whole case directory, so new case files are picked up without Rust edits. No Rust evaluator consumes `inputs`/`expected` yet (owner agents may feed them to their evaluators; the case format is language-agnostic).
- Registry: `enXXXX-1-<library>` entries regenerated from the case files for all ten artifacts (`🧰️scripts/oracle-wave2-register.py`), SciPy claim `norm-eurocode-scipy-numerics` added to the norm root registry.
- Scripts (inputs, kept): `🧰️scripts/oracle-wave2-gen.py` (case definitions), `oracle-wave2-bless.sh` (generate, bless, verify all), `oracle-wave2-register.py`. Logs: `🗑️generated/oracles-wave2/` (delete when done).

## Findings against our implementation (Rust read, not executed)

1. EN 1991 `lm1_tandem_moment_nm` (`💡️inferences/⚖️checks/🦀️.rs:813`) returns alpha_Q Q L / 4 (one axle). The LM1 tandem (2 x Q_1k at 1.2 m) on a 9.6 m span gives M_max 1265.6 kNm (PyNite, anastruct and influence functions agree) against 720 kNm: 43 percent unsafe; reactions R_max 562.5 kN. Cases `en1991.mechanics.lm1-tandem-span-*`.
2. EN 1995 strength table (`⚖️kernel/📚️strength-tables/🦀️.rs`) against blue-prints EN 338:2016 Tables 1 and 3: 22 differing entries in the sampled classes (C20 rho_mean 390 vs 400; C27/C35/C40/C45/C50 f_c,90,k 2.6/2.8/2.9/3.1/3.2 vs 2.5/2.7/2.8/2.9/3.0; C45 E_0,05 10000 vs 10100; C50 f_t,0,k 33 vs 33.5; D24 f_c,90,k 7.8 vs 4.9, f_v,k 4.0 vs 3.7, E_0,05 8500 vs 8400, E_90 640 vs 670, G 600 vs 630; D40 f_c,0,k 26 vs 27, f_c,90,k 8.3 vs 5.5, f_v,k 4.0 vs 4.2; D70 f_c,0,k 34 vs 36, f_c,90,k 13.5 vs 12, f_v,k 6 vs 5, rho_k 900 vs 800). The Rust rows look like EN 338:2009; the blue-prints transcription itself is unverified against the printed EN 338:2016 (the `verified` flags of the rows should wait for that check). Cases `en1995.338.strength-class-*` carry the library values with `table-exact-v1`.
3. EN 1996 x9 reinforced flexure (M-40, 72.58 kNm) agrees with concreteproperties and the strain-compatibility derivation to 3e-6.
4. T-section flexure (structuralcodes, concreteproperties, quadrature) agrees to 1e-5; the EN 1992 Rust evaluator has no T-section check to compare yet.
5. blue-prints `Form6Dot18SubCTSectionRolled` gives A - b t_f + (t_w + 2r) t_f / 2, which is not EN 1993-1-1:2005 6.2.6(3)c (0.9 (A - b t_f)); not used as an oracle (edition mismatch), `6.41` and `6.44` return booleans and are not used either.

## Coverage per artifact (clause check, libraries, kind, cases, status)

### EN1990 (12 cases, 11 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1990.6.4.combinations` | numerical | independent | 3 | new |
| `en1990.C.beta` | numerical+sympy | third-party | 4 | new |
| `en1990.C.beta` | numerical+sympy | third-party | 1 | wave1 |
| `en1990.C.form-linear` | numerical+sympy | independent | 2 | new |
| `mechanics.beam-envelope` | anastruct+pynite | third-party | 2 | new |

### EN1991 (6 cases, 6 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1991.4.janssen-pressure` | numerical+sympy | independent | 3 | new |
| `mechanics.lm1-tandem` | anastruct+numerical+pynite | third-party | 3 | new |

### EN1992 (73 cases, 37 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1992.3.1.concrete-laws` | structuralcodes | third-party | 3 | wave1 |
| `en1992.3.1.creep-shrinkage` | structuralcodes | third-party | 3 | new |
| `en1992.3.15.fcd` | blueprints+structuralcodes | third-party | 1 | wave1 |
| `en1992.3.19.lambda` | blueprints | third-party | 2 | new |
| `en1992.3.21.eta` | blueprints | third-party | 2 | new |
| `en1992.3.6.creep-deformation` | blueprints | third-party | 1 | new |
| `en1992.4.1.c-nom` | blueprints | third-party | 1 | new |
| `en1992.4.2.c-min` | blueprints | third-party | 1 | new |
| `en1992.5.13n.lambda-lim` | blueprints | third-party | 1 | wave1 |
| `en1992.5.14.slenderness` | blueprints | third-party | 1 | wave1 |
| `en1992.5.34.curvature` | blueprints | third-party | 1 | wave1 |
| `en1992.6.1.flexure-rect` | concreteproperties+numerical+structuralcodes | third-party | 1 | wave1 |
| `en1992.6.1.flexure-rect` | concreteproperties+structuralcodes | third-party | 4 | wave1 |
| `en1992.6.1.flexure-t` | concreteproperties+numerical+structuralcodes | third-party | 3 | new |
| `en1992.6.1.nm-interaction` | concreteproperties+structuralcodes | third-party | 2 | wave1 |
| `en1992.6.10.nu1` | blueprints | third-party | 2 | new |
| `en1992.6.11.alpha-cw` | blueprints | third-party | 3 | new |
| `en1992.6.2.shear-link-limit` | structuralcodes | third-party | 1 | new |
| `en1992.6.2.shear-links` | structuralcodes | third-party | 3 | wave1 |
| `en1992.6.2.shear-vrdc` | blueprints+structuralcodes | third-party | 8 | wave1 |
| `en1992.6.26.tau-wall` | blueprints | third-party | 1 | new |
| `en1992.6.28.asl-torsion` | blueprints | third-party | 1 | new |
| `en1992.6.30.t-rd-max` | blueprints | third-party | 1 | wave1 |
| `en1992.6.38.punching-stress` | blueprints | third-party | 1 | wave1 |
| `en1992.6.39.beta-moment` | blueprints | third-party | 1 | new |
| `en1992.6.43.beta-rectangular` | blueprints | third-party | 1 | new |
| `en1992.6.47.punching-vrdc` | blueprints | third-party | 1 | new |
| `en1992.6.51.punching-eccentric` | blueprints | third-party | 1 | new |
| `en1992.6.6n.nu` | blueprints | third-party | 1 | wave1 |
| `en1992.6.8.vrds` | blueprints | third-party | 1 | new |
| `en1992.6.9.vrdmax` | blueprints | third-party | 1 | new |
| `en1992.7.11.sr-max` | blueprints+structuralcodes | third-party | 1 | wave1 |
| `en1992.7.14.sr-max-far` | structuralcodes | third-party | 1 | new |
| `en1992.7.16.span-depth` | blueprints | third-party | 2 | wave1 |
| `en1992.7.2.cracked-section` | concreteproperties | third-party | 1 | wave1 |
| `en1992.7.3.effective-tension-area` | structuralcodes | third-party | 2 | new |
| `en1992.7.3.min-reinforcement` | structuralcodes | third-party | 2 | new |
| `en1992.7.8.crack-width` | blueprints+structuralcodes | third-party | 1 | wave1 |
| `en1992.7.9.strain-difference` | blueprints+structuralcodes | third-party | 1 | wave1 |
| `en1992.8.10.lap-length` | blueprints | third-party | 1 | new |
| `en1992.8.14.bundle-diameter` | blueprints | third-party | 1 | new |
| `en1992.8.2.fbd` | blueprints | third-party | 1 | new |
| `en1992.8.3.anchorage-rqd` | blueprints | third-party | 1 | wave1 |
| `en1992.8.4.anchorage-design` | blueprints | third-party | 2 | new |
| `mechanics.beam-continuous` | anastruct+pynite | third-party | 1 | wave1 |

### EN1993 (37 cases, 13 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1993.5.2.alpha-cr` | blueprints | third-party | 1 | wave1 |
| `en1993.6.10.nc-rd` | blueprints | third-party | 1 | new |
| `en1993.6.14.mc-rd-class3` | blueprints | third-party | 1 | new |
| `en1993.6.2.av-channel` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.av-chs` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.av-rhs-depth` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.av-rolled-i` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.mc-rd-class12` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.my-n-rd` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.my-v-rd` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.npl` | blueprints | third-party | 1 | wave1 |
| `en1993.6.2.vpl-rd` | blueprints | third-party | 1 | wave1 |
| `en1993.6.21.tau-i` | blueprints | third-party | 1 | new |
| `en1993.6.3.2.mcr-i` | sectionproperties | third-party | 2 | wave1 |
| `en1993.6.3.mb-rd` | blueprints | third-party | 1 | wave1 |
| `en1993.6.32.mnrd-rectangular` | blueprints | third-party | 1 | new |
| `en1993.6.39.aw-hollow` | blueprints | third-party | 1 | new |
| `en1993.6.39.mnyrd-hollow` | blueprints | third-party | 1 | new |
| `en1993.6.7.nu-rd` | blueprints | third-party | 1 | new |
| `en1993.section.angle` | sectionproperties | third-party | 1 | wave1 |
| `en1993.section.channel` | sectionproperties | third-party | 1 | wave1 |
| `en1993.section.chs` | numerical+sectionproperties | third-party | 1 | wave1 |
| `en1993.section.ehs` | sectionproperties | third-party | 1 | new |
| `en1993.section.i-rolled` | sectionproperties | third-party | 5 | wave1 |
| `en1993.section.mono-i` | sectionproperties | third-party | 2 | new |
| `en1993.section.rhs` | sectionproperties | third-party | 1 | wave1 |
| `en1993.section.rhs-sharp` | numerical+sectionproperties | third-party | 2 | new |
| `en1993.section.tee` | sectionproperties | third-party | 1 | new |
| `mechanics.beam-continuous` | anastruct+pynite | third-party | 1 | wave1 |
| `mechanics.portal-sway` | anastruct+pynite | third-party | 1 | wave1 |

### EN1994 (11 cases, 7 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1994.5.4.composite-elastic` | numerical+sectionproperties | third-party | 2 | new |
| `en1994.5.4.composite-elastic` | sectionproperties | third-party | 1 | new |
| `en1994.6.2.composite-plastic` | numerical | independent | 3 | new |
| `mechanics.beam-continuous` | anastruct+pynite | third-party | 4 | wave1 |
| `mechanics.beam-envelope` | anastruct+pynite | third-party | 1 | new |

### EN1995 (24 cases, 19 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1995.338.strength-class` | blueprints | third-party | 18 | new |
| `en1995.7.5.natural-frequency` | blueprints | third-party | 1 | wave1 |
| `mechanics.beam-continuous` | anastruct+pynite | third-party | 4 | wave1 |
| `mechanics.beam-envelope` | anastruct+pynite | third-party | 1 | new |

### EN1996 (3 cases, 3 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1996.6.6.1.flexure-reinforced` | concreteproperties+numerical | third-party | 3 | new |

### EN1997 (18 cases, 7 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1997.2.cpt-behaviour-index` | groundhog | third-party | 1 | wave1 |
| `en1997.2.cpt-friction-angle` | groundhog | third-party | 3 | wave1 |
| `en1997.2.cpt-undrained-strength` | groundhog | third-party | 1 | wave1 |
| `en1997.2.phase-relations` | groundhog | third-party | 2 | new |
| `en1997.9.earth-pressure` | groundhog | third-party | 3 | wave1 |
| `en1997.D.4.bearing-factors` | groundhog | third-party | 3 | wave1 |
| `en1997.F.boussinesq` | groundhog+numerical | third-party | 5 | new |

### EN1998 (10 cases, 10 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1998.3.2.2.5.design-spectrum` | sympy | independent | 5 | new |
| `en1998.4.3.shear-building-modal` | numerical | independent | 1 | new |
| `en1998.4.3.shear-building-modal` | numerical+sympy | independent | 2 | new |
| `mechanics.lateral-frame` | anastruct+pynite | third-party | 2 | new |

### EN1999 (7 cases, 7 new in this wave)

| clause check | libraries | kind | cases | status |
|---|---|---|---|---|
| `en1999.section.angle` | sectionproperties | third-party | 1 | new |
| `en1999.section.channel` | sectionproperties | third-party | 1 | new |
| `en1999.section.chs` | numerical+sectionproperties | third-party | 1 | new |
| `en1999.section.ehs` | sectionproperties | third-party | 1 | new |
| `en1999.section.i` | sectionproperties | third-party | 1 | new |
| `en1999.section.rhs-sharp` | numerical+sectionproperties | third-party | 1 | new |
| `en1999.section.tee` | sectionproperties | third-party | 1 | new |

## Clauses still without a third-party check and why

- EN 1990: Annex A1 serviceability combination cases beyond 6.14b/6.15b/6.16b and Annex D design by testing: no library; the combinations are independent enumeration only (kind independent-formulation, single reference).
- EN 1991: wind (4.2 to 4.5 profiles, c_pe tables), snow shape factors, fire (ISO 834, parametric curve, Annex E), thermal, accidental, crane, silo patch loads, tank: tables and closed forms without any library; Janssen has two references (ODE integration and sympy dsolve), the tandem has three (PyNite, anastruct, influence functions). Remaining ones are transliteration-only candidates and were deliberately not added as oracles.
- EN 1992: 6.4 punching beyond single formulas (control perimeters), 6.3 torsion beyond Eq. 6.26/6.28/6.30, 5.8 second order, 7.4 beyond 7.16, 9 detailing minima, fire (EN 1992-1-2), prestress losses (3.28 to 3.30 present in blue-prints, not wired): no section-level library covers them.
- EN 1993: 6.3.1 buckling (sympy Ayrton-Perry exists from wave 1), 6.3.3 interaction (blue-prints lacks 6.46 to 6.53), class 4 effective widths (EN 1993-1-5), joints (1-8 Table 3.4 exists in blue-prints but not wired), fatigue, fire: not covered by any installed library.
- EN 1994: shear connector resistance 6.6.3, 6.2.2 vertical shear, 6.4 lateral-torsional buckling, creep and shrinkage effects of the slab (structuralcodes creep is wired for EN 1992 only), 7 cracking of the slab: no library; plastic moment is an independent bisection of the neutral axis (kind independent-formulation).
- EN 1995: connections (Johansen), fire, vibration beyond 7.5, glulam EN 14080 tables (blue-prints has EN 338 only), 6.3 stability and instability factors: no library.
- EN 1996: vertical resistance 6.1 and slenderness (Annex G phi_m), shear walls, arching, fire: no library; only reinforced flexure 6.6.1 has references (concreteproperties plus quadrature).
- EN 1997: bearing resistance Annex D shape, inclination and depth factors, Annex C log-spiral K_p, Annex F settlements by the stress-strain method (groundhog settlement imports `requests`, not installed), pile resistance, slope stability: groundhog does not implement them; stress increments and phase relations are covered.
- EN 1998: spectra are independent-formulation (sympy Piecewise, single reference); modal analysis and lateral frame statics are covered, lateral-force-method distribution F_i = F_b z_i m_i / sum(z_j m_j) (4.11) and torsional effects 4.3.3.2.4, behaviour factor tables, ductility classes: no library; PyNite `analyze_modal` was not used (disagrees with closed forms, see `mechanics-kernel.md`).
- EN 1999: member buckling (6.3), effective thickness with HAZ (6.1.6), classification (6.1.4), local buckling: no library; only gross section constants are cross-checked by sectionproperties (and exact rectangle differences for the sharp RHS).

## Notes for owner agents

- Blessed expected values for each artifact are in its `🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json`; feed `inputs` (SI) to the evaluator and compare `expected.*.value` under `expected.*.profile` (tolerances in `🏃️harness/📏️tolerance`). Cases for quantities our code does not yet compute are the to-do list.
- `en1998.4.3.shear-building-three-storey` carries Gamma_1 normalised to a unit top displacement.
- groundhog still needs the inert import finder for `plotly`, `pyproj`, `jinja2`; `settlement` additionally needs `requests` (not used).
- The dependency baseline `🔒️dependencies.json` still lacks the oracle libraries (needs the native `repo verify` command, see `oracle-deps-lock.md`).
