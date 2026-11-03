# 📐 Independent Oracle Strategy

Scope: replace the Rust-transliteration Python oracles of EN 1990-1999 with genuinely independent references. Research only; no source modified. Library facts were read from PyPI JSON on 2026-11-01 unless marked **unverified**.

## 1. Current state (inspected)

| Eurocode | Compliance oracle (`.py`) | Lines / imports | What it is |
|---|---|---|---|
| EN 1990 | `🧬️schema/🧪️tests/⚖️compliance/🐍️.py` | 41, stdlib only | 6.10a/b, k_FI, beta tables with 5 self-asserts; the Rust test does not even spawn it for numbers |
| EN 1991 | `🔮️oracles/⚖️compliance/🐍️.py` | stdlib | snow/imposed/self-weight lookup tables transcribed from Rust |
| EN 1992 | `🧬️schema/🧪️tests/⚖️compliance/🐍️.py` | 583, stdlib | alpha_cc/gamma tables, whole ULS/SLS chain transliterated; `--json` stdin protocol |
| EN 1993 | `🔮️oracles/⚖️compliance/🐍️.py` | stdlib | chi curve, N_Rd, critical temperature (EN 1993-1-2 eq. 4.22) re-typed |
| EN 1994 | same path | stdlib | b_eff, P_Rd stud, eta_min, insulation table re-typed |
| EN 1995 | same path | stdlib | EN 338 strength table copied, 6.x checks, Johansen |
| EN 1996 | same path | stdlib | f_k, Phi_s/i/m re-typed |
| EN 1997 | same path | stdlib | Nq/Nc/Ngamma, inclination factors, Bishop |
| EN 1998 | same path | stdlib | NA spectrum table, Se(T), base shear |
| EN 1999 | same path | stdlib | alloy table, class, N_Rd, M_Rd |

Findings:
- Not one compliance oracle imports a third-party package other than `jsonschema` (snapshot validation). They are "second implementations" by the same author from the same tables: they catch typos in one copy but share every misreading of the standard and every transcribed table value.
- Each `🔮️oracles/🔣️.json` registers only `enXXXX-1-python-independent` for the `*-mutate` capability (`package: ""`, `testOnly: true`, rationale says no library reads `s.norm.enXXXX`). Compliance has no registry entry, so the repo's own "third-party library" requirement is not met for any evaluator.
- Run path: Rust integration tests (`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`) do `Command::new("python3")` on the script, feed the snapshot JSON on stdin (`--json`), compare per check id with `rel <= 0.005 || abs <= 0.005`. Targets are the per-artifact nx/cargo test targets (`@semio-tech/norm-enXXXX-rs`) driven via `repo test` / `mcp__repo__test`. It uses the bare `python3` on PATH, not an explicitly resolved uv environment: any third-party import silently depends on PATH python being `.venv/bin/python` (3.14.6 in both cases here, so it works today by accident, not by contract).
- Oracles recompute from a Rust-shaped snapshot (hierarchical SI snapshot) and are tied to the Rust check-id set, so adding a check requires editing both sides identically.

## 2. Python packages and policy

Locked today (`pyproject.toml` group `test`, `uv.lock`, `.venv`, all `test-oracle` in `🔒️dependencies.json`): `numpy 2.5.0`, `scipy 1.18.0`, `shapely 2.1.2`, `networkx`, `jsonschema`, `deepdiff`, `anastruct 1.7.0` (GPL-3.0-or-later), `PyNiteFEA` (locked 3.0.0 in `.venv`; PyPI latest 3.2.1, MIT), `scikit-fem`, `ifcopenshell 0.8.4.post1`. **Not** installed: structuralcodes, concreteproperties, sectionproperties, eurocodepy, blue-prints, openseespy, sympy, pint, groundhog. Python is pinned `>=3.14,<3.15`.

Policy (from `AGENTS.md` and `🧰️framework/.../🧪️test/🔒️oracle-dependencies/🟦️.ts`):
1. Test-only dependencies are allowed, runtime ones are not. Classes: production-runtime, production-build, repository-tooling, test-runner, test-oracle.
2. Shrink-only ratchet (`ratchetDependencies`): a new production-reachable dependency is always refused; a new `test-oracle` dependency is accepted only if an oracle registry entry (`🔮️oracles/🔣️.json`) claims the package (`package` or `packages[]`). Baseline is `🔒️dependencies.json`, written by `verify dependencies write-baseline`.
3. Registry entry shape to copy (see `🏙️bim/.../🏢️model/.../🔮️oracles/🔣️.json`): `kind: third-party-library`, `ecosystem`, `package`, exact `version`, `packages[]` for companions, `capabilities`, `comparisonProfiles` (`floating-point-v1`), SPDX `license`, `testOnly: true`, `productionReachable: false`, `networkDuringExecution: false`, `engine{family,implementation,version}`, `homepage`, `rationale`.
4. Workflow: add to `pyproject.toml` `[dependency-groups].test`, lock with `mcp__repo__dependencies_lock` (repo `deps-*-lock`; `uv lock --check` is the verification), sync with `dependencies_sync`/`dependencies_ensure`, then `verify dependencies*` gates (freeze, literal-external, locks). Not executed in this task.
5. Python-version caveat: precedent exists (energy plugin pins 3.12 for `openstudio`). Pure-Python wheels are fine on 3.14; compiled ones (`cytriangle`, `meshpy`, `openseespy*`) need cp314 wheels: **verify before locking**.
6. Licenses: oracles run in tests only and are not linked into shipped code, so LGPL/GPL are acceptable the way `anastruct` (GPL-3.0) and `ifcopenshell` (LGPL-3.0) already are. Repo is AGPL-3.0, so no incompatibility either way.

## 3. Library survey per Eurocode

Legend: **A** = independent library computes the same quantity; **B** = partial (tables/params only or sub-quantities); **C** = no library, use independent formulation.

| Library (PyPI) | Version / license / py | Covers (verified from PyPI summary + docs unless noted) |
|---|---|---|
| `structuralcodes` | 0.7.3 (2026-10-29), MIT per repo (PyPI metadata field empty, LICENSE file present; **confirm file**), >=3.11; deps meshpy, numpy, scipy, shapely | fib Model Code 2010/2020 and EN 1992-1-1:2004 (concrete constitutive laws, design strengths, shear, creep/shrinkage, anchorage, sections via fibre/marin integration, M-N surfaces). Clause-level coverage per release must be confirmed from its docs/`structuralcodes.codes.ec2_2004` module listing (web search did not return it) |
| `concreteproperties` | 0.8.0, MIT, >=3.12 (depends on sectionproperties, cytriangle, shapely, scipy) | RC section analysis: moment-curvature, M-N interaction, ultimate bending with Eurocode-style parabolic-rectangular and rectangular blocks (selectable stress-strain), cracked section properties |
| `sectionproperties` | 3.10.2, MIT, >=3.11 | FEM section constants: A, I, Iw, J, shear areas, plastic moduli, warping; for EN 1993/1995/1999/1994 sections |
| `blue-prints` (Blueprints) | 0.8.0, MIT, >=3.12, uses sectionproperties | Eurocode objects incl. EN 1992-1-1 (2004 and 2023 formulas/tables), EN 1993, EN 1997 per its docs (EN 1991/1995 not listed on the docs index); coverage per formula must be checked |
| `eurocodepy` | 2027.0.12, LGPL-3.0, >=3.12; heavy deps (mkdocs, marimo, plotly) so lock with `--no-deps`-style care or accept bloat | EC0 combos, EC2 shear/creep, EC3 profiles (no buckling function seen), EC5 timber ULS bending/shear + floor vibration, EC7 shallow bearing / active-passive K, EC8 elastic/design spectra incl. NA params. Self-disclaims fitness |
| `anastruct` | 1.7.0 locked (1.7.1 latest), GPL-3.0-or-later | 2D linear and geometric non-linear frames, buckling; independent of any code: use for internal forces, deflections, second-order amplification |
| `PyNiteFEA` | locked 3.0.0, MIT | 3D frame/plate FEA, P-Delta, load combinations: internal forces, deflections, linear buckling, response spectrum analysis (verify) |
| `scipy`/`numpy` | locked | numerical integration (`quad`), root finding (`brentq`), eigenproblems, Euler buckling by eigenvalue; ground truth for closed forms |
| `openseespy` | 3.8.0.0 (pure wheel, platform sub-packages), license in LICENSE.md (**unverified**; OpenSees is a BSD-style non-commercial-redistribution license, check) | nonlinear fibre sections, modal / response-spectrum analysis for EN 1998; cp314 wheels unconfirmed |
| `groundhog` | 0.15.0, GPL-3.0, >=3.7 | Geotechnics: bearing capacity (Meyerhof/Hansen/Vesic style), pile resistance, settlements, soil mechanics helpers; **not** EN 1997 partial-factor sets, so only the raw formulas |
| `sympy` | 1.14.0, BSD | symbolic re-derivation / series sanity (Nq, Nc, N_gamma closed forms; Euler); `pint` for unit audit |
| `steelpy` | 1.1.1, Apache-2.0 | **AISC** shapes only; not usable for Eurocode steel |
| `ifcopenshell` | locked | IFC profile/section properties for geometry cross-checks of 3D visualisation, not code checks |

Not found / not verified to exist (do not plan on them): a maintained open-source Python implementation of EN 1993-1-1 member buckling (6.3.1/6.3.3, Annex A/B), of EN 1994, EN 1996, EN 1999, of EN 1991 snow/wind NA-DE tables, or of NA-DE-specific parameters in general. `pyEC3`/`eurocode3`/`pyrotech`-type names from the brief were not confirmed on PyPI and are excluded. Tools in other ecosystems (e.g. Rust `eurocode`-like crates, JS) were not surveyed; the rule "third-party in any ecosystem" permits them later.

## 4. Per-Eurocode mapping (check id → independent reference)

Verdicts: A strong, B partial, C no library (independent formulation). "Closed form" in the Rust is never the oracle for itself.

### EN 1990 (combos, k_FI, beta)
- Combinations 6.10/6.10a/b, psi factors: **C/B**. `eurocodepy` EC0 combos as optional A-grade cross-check; primary oracle is a brute-force enumeration (all leading-action permutations, itertools) over the stated action sets, plus the PyNite/anastruct load-combination engine for load-case superposition (ULS envelope = max over enumerated permutations, no hand-written 6.10 formula). psi/gamma values themselves are normative table data (Table A1.1/A1.2, NA.A1): record them as **cited fixtures** (clause + table), not code.
- beta target / k_FI: scipy `norm.ppf` of Table B2 failure probabilities (independent numerics for beta = -Phi^-1(p_f)); k_FI table is normative data.

### EN 1991 (loads)
- Imposed, self-weight, NA snow zone/altitude, wind profile: **C**. Data are normative tables, and no library holds NA-DE values. Oracle design: (a) the cited-table JSON fixture, extracted from the standard text by a one-time documented script, hash-pinned; (b) piecewise formulas re-evaluated by `sympy` (symbolic, then lambdify) and `scipy` interpolation, run on a dense altitude grid; (c) snow shape-factor mu(alpha) is checked by scipy `interp1d` through the table's break points, not the formula. Mark as "independent formulation", not "third-party validation", in the registry rationale.
- Static loads on members to internal forces: PyNite/anastruct (A).

### EN 1992 (concrete)
- 6.1 flexure (M_Rd, x, stress block): `structuralcodes` EC2 2004 section calc (A) and `concreteproperties` (A, fibre integration with parabola-rectangle law) (second library). Plus own scipy `quad` over the strain profile with the EN 1992 3.1.7 curve (independent numerics).
- 6.2 shear (V_Rd,c, V_Rd,s, V_Rd,max): `structuralcodes` EC2 shear functions and `blue-prints` (A, B per clause).
- 6.3 torsion, 6.4 punching, 5.8 slenderness (lambda_lim, nominal curvature), 6.1 N-M: structuralcodes/blue-prints where present (check at lock time), else C: M-N surface by fibre integration (`concreteproperties.ultimate_bending`/`structuralcodes` `calculate_nm_interaction_domain`) vs our curve (A).
- 7.2 stresses (sigma_c, sigma_s cracked): `concreteproperties` cracked_properties (A) or transformed section via `sectionproperties` (A). 7.4 deflection / creep: `structuralcodes` creep coefficient (B), `eurocodepy` creep (B); deflection via PyNite/anastruct with EI_eff (A).
- 8.4 anchorage l_bd, 8.7 lap, 9 min/max As, 4.4 cover: structuralcodes/blue-prints (B), else C (data tables).
- 1-2 fire (a, b_min): Table 5.x data; C.
- fib MC2010/2020 cross-check is a bonus independent formulation for creep/shrinkage.

### EN 1993 (steel)
- Section classification and A, I, W_pl, W_el, I_w, I_t: `sectionproperties` FEM on rolled-section geometry (A) (independent geometry → constants).
- N_Rd, M_Rd, V_Rd: arithmetic on those constants; reference by `sectionproperties` plastic section analysis (A for W_pl).
- Buckling 6.3.1 (chi): N_cr from **eigenvalue buckling** in PyNite/anastruct (A for N_cr) or analytic via scipy `eigh` on a beam-column stiffness matrix; chi from Table 6.1/6.2 curve through `sympy` re-derivation or direct `scipy.optimize` of the Ayrton-Perry equation (C for chi). LTB M_cr: PyNite eigenvalue buckling with warping (verify availability) else beam on 7-DOF warping FEM in numpy (C).
- 6.3.3 interaction (Annex A/B): C; no library. Use independent matrix formulation: second-order amplification via PyNite P-Delta and compare utilisation vs ratio with reduced-capacity N-M interaction by numerical plastic integration over the section (`sectionproperties` plastic and `shapely` fibre grid) (partial A).
- 1-2 fire critical temperature: closed form eq. 4.22 ↔ scipy `brentq` over the k_y,theta table (Table 3.1) so the oracle solves the inverse of the table instead of the fit (C, independent numerics).
- 1-9 fatigue Palmgren-Miner: C (numpy rainflow sum of S-N curve segments with the cited detail category table).
- `blue-prints` EN 1993 functions (B) as an additional optional reference per clause after lock-time check.

### EN 1994 (composite)
- b_eff (5.4.1.2), P_Rd studs (6.6.3), eta_min (6.6.1.2), insulation table: C. Independent: composite section transformed with n=E_a/E_cm via `sectionproperties` (A for I_y,eff, elastic/plastic neutral axis), plastic moment by `shapely` fibre integration (C). Stud P_Rd equations are two lines; the oracle value comes from the cited clause plus the min over failure modes via scipy `minimize_scalar` test, which is not meaningfully independent; flag as "documented-formula oracle, reviewed twice". Fire table: cited data.

### EN 1995 (timber)
- Strength classes (EN 338 / EN 14080): data; `eurocodepy` material tables (B) as a second independent transcription, automatic diff against our table. Bending/shear/compression 6.1.x, 6.3.2 (k_c), combined 6.2.4: `eurocodepy` timber ULS bending/shear (B); otherwise C with `sympy` re-derivation of lambda_rel,c, k_c.
- Johansen 8.2 (embedment / failure modes): C. Independent approach: **mechanical yield-mode solver**, minimise the load capacity of the dowel as a plastic beam in embedment with numpy/scipy (equilibrium of rigid-plastic dowel with embedment foundation), which reproduces modes a-f without the closed forms.
- SLS deflection / w_inst, w_fin, floor vibration: PyNite/anastruct (A) with E_mean, k_def; vibration: eurocodepy (B).
- 8.x spacing: table data (C).

### EN 1996 (masonry)
- f_k = K f_b^a f_m^b, Phi_s/i/m: C. Data tables with NA-DE variants cannot be validated by a library. Independent: Phi_m from the **second-order numerical beam-column model** (anastruct/PyNite geometric non-linear on an eccentrically loaded elastic-no-tension strip, or scipy `solve_bvp` on the buckling ODE) vs the Annex G/NA closed form (C+A hybrid).

### EN 1997 (geotech)
- N_q, N_c, N_gamma (Prandtl/Reissner/Vesic): `groundhog` bearing capacity (A, raw coefficients), `eurocodepy` bearing (B), `blue-prints` EN 1997 (B), `sympy` symbolic Nq (C-verified closed form). Shape/inclination/base factors (Annex D): eurocodepy/groundhog where defined, else C.
- Sliding / eccentricity: static equilibrium via `shapely` (effective area, B&H eccentric footing) (C).
- Bishop slope stability: **independent**: scipy `minimize` of FoS over a circular-slip grid with numpy slices (own formulation), plus a limit-equilibrium library as optional (not on the list; `slope_stability`-type packages **unverified**). Use Morgenstern-Price/Spencer in numpy as the second method; Bishop vs Spencer cross-check within a few percent (tolerance profile `method-spread`).
- Pile resistance: groundhog pile modules (B/A).

### EN 1998 (seismic)
- Se(T), Sd(T) spectra, NA-DE zone/soil table, q-factor spectrum: `eurocodepy` EC8 (A for spectrum shape; NA-DE values absent → cited table fixture). `sympy`/numpy piecewise re-evaluation is the C backup.
- Base shear F_b = S_d(T1) m lambda, T1 = C_t H^(3/4): C formula; independent: **modal analysis** of a lumped-mass shear building with `scipy.linalg.eigh`, `PyNite`/`anastruct` frame K-M matrices, modal participation + SRSS/CQC (A for T1, mode shapes, F_b vs lateral force method), and openseespy `eigen` (optional, if cp314/licence cleared).
- psi_E combinations: C enumeration like EN 1990.

### EN 1999 (aluminium)
- Alloy table (EN 1999-1-1 Table 3.2): data fixture; no library. Class, N_Rd, M_Rd with rho_o,haz: `sectionproperties` for section constants (A), plastic modulus with `shapely` (C), HAZ reduction by explicit effective-area subtraction (C independent: geometric removal of heat-affected strips vs rho factor). Fatigue (Annex J): numpy Miller sum (C). Bolt/weld: C (cited formula, reviewed twice).

## 5. Oracle design

### 5.1 Language-agnostic fixture format (schema-first)
One schema `🧬️schema/📜️oracle-case/🔣️.json` (JSON Schema draft 2020-12) under the shared `⚖️compliance` kernel and one case file per quantity group under each artifact's `🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json`:

```json
{
  "schemaVersion": 1,
  "artifact": "en1992", "annex": "de",
  "cases": [{
    "id": "en1992.6.1.flexure.rect-c30-b300-h500-as3-0",
    "check": "en1992.6.1.flexure",
    "clause": "EN 1992-1-1:2004 6.1; NA-DE",
    "inputs": {"fck": 30e6, "fyk": 500e6, "b": 0.3, "d": 0.45, "As": 0.0012},
    "expected": {"M_Rd": {"value": 183.2e3, "unit": "N*m", "tolPolicy": "closed-form-v1"}},
    "references": [{"oracle": "en1992-1-structuralcodes", "fn": "..."}, {"oracle": "en1992-1-concreteproperties", "fn": "..."}]
  }]
}
```
- Inputs are SI base units only (matching the snapshot), cases are generated from parameter grids (min/typical/max, boundary values at every piecewise break: chi at lambda=0.2, mu at 30/60 deg, T at TB/TC/TD, phi=0) plus randomized property-based samples with a fixed seed recorded in the file.
- Each case lists which oracle(s) produced the expected value, so the fixture is the blessed truth and the Python oracle is the generator/verifier; Rust consumes the fixture as a plain data test (no Python needed in the default loop), and a separate test target regenerates with Python and diffs (the repo already uses blessed fixtures + oracle re-verification; see `🏙️bim` registry).
- Same file is consumed by TS/WASM/other implementations (multi-implementation rule).

### 5.2 Shared harness
`✏️s/🔌️plugins/📕️norm/⚖️compliance/🔮️oracle-harness/🐍️.py` (shared Python module) providing:
- `load_cases(schema_path)`, schema-validate with `jsonschema` (already locked).
- `run(adapter)`: adapters are small per-library modules (`adapters/structuralcodes.py`, `concreteproperties.py`, `sectionproperties.py`, `pynite.py`, `anastruct.py`, `numerical.py`, `eurocodepy.py`, `groundhog.py`) each exporting `compute(check_id, inputs) -> {name: value}`; missing library → explicit `SKIP: not installed` only in a non-gating mode, `FAIL` in `repo test`.
- Entry point mirrors the existing protocol (`--json` stdin → `{"checks":[{"id","utilization", "quantities":{...}}]}`) so the Rust tests keep working and also support `--cases <file>` and `--bless`.
- Resolved interpreter: the Rust tests should spawn the repo-resolved Python (`repo` tool resolution of the uv `.venv`, e.g. env var set by the test host) instead of bare `python3`, otherwise third-party imports are not guaranteed. Needs a framework decision; today it works only because PATH python is the venv.
- Never imports anything from Rust or from the repository's own norm tables; table data lives in the cited-data fixtures with `source` (standard, table, clause) and a sha256, authored independently from the Rust tables (second keying, diffed).

### 5.3 Registry entries
One `🔮️oracles/🔣️.json` entry per library per artifact, e.g. `en1992-1-structuralcodes` (package `structuralcodes`, version pinned, MIT, capability `en1992-1-evaluate`, profile `eurocode-quantities-v1`), plus `en1992-1-concreteproperties`, `en1992-1-scipy-numerics` (companion `packages[]`: scipy, numpy). Rationale must state which quantity is delegated and which is not. Keep `enXXXX-1-python-independent` only for the mutate capability and relabel compliance second implementations honestly as `kind: independent-implementation` (not counted as library validation).

### 5.4 Tolerance policy (profiles in the `comparisonProfiles` vocabulary)
| Profile | Rule |
|---|---|
| `closed-form-v1` | rel 1e-9 (same formula, same constants) |
| `eurocode-quantities-v1` | rel 1e-4 for resistances/utilisation vs library; rel 2e-3 where the library uses a rounded constant (document constant, e.g. EN 338 E-modulus, 0.85 vs library's alpha_cc) |
| `numerical-v1` | rel 5e-3 vs integration/FEM (mesh/fibre discretisation); convergence-checked by refining and requiring monotone error |
| `method-spread-v1` | stated engineering spread between two methods (e.g. Bishop vs Spencer 3 percent) with both outputs recorded |
| `table-exact-v1` | data tables: exact equality of cited values after unit conversion |
The existing 0.5 percent gate stays as the maximum for any Rust-vs-oracle utilisation. Differences between libraries (e.g. structuralcodes vs concreteproperties) are recorded as a `spread` and must be inside tolerance too, otherwise the case is flagged "oracle disagreement" for manual review rather than silently choosing one.
- Utilisation comparison: compare **resistances/quantities** first (M_Rd, chi, N_cr, Se) then utilisation, so a loads-chain error cannot hide a resistance error.
- NA-DE parameters never come from a library; the fixture holds them with citation, and library calls pass them in (e.g. alpha_cc 0.85, gamma_c 1.5) so only the clause mechanics is compared.

### 5.5 Where no independent library exists (explicit)
These stay "independent formulation" and must be labelled so in the registry, not as third-party validation: EN 1990 combos/beta (brute-force enumeration), EN 1991 NA tables, EN 1993 6.3.3 interaction and fire/fatigue, EN 1994 stud/b_eff/fire, EN 1995 Johansen/spacing, EN 1996 f_k and Phi, EN 1997 Annex D factors and sliding, EN 1998 NA-DE spectrum parameters and q-reduction, EN 1999 HAZ/bolt/weld/fatigue. For these the strategy is (i) numerical/physical reformulation (numerical integration of the stress field instead of the stress block formula, eigenvalue instead of Euler, yield-mode solver instead of the Johansen cases, modal analysis instead of the lateral force method, limit equilibrium search instead of the Bishop formula), (ii) independently keyed normative tables with citation hashes, (iii) review by a second keying pass (diff of two transcriptions).

## 6. Rollout plan
1. Lock test-only deps in the `test` group: `structuralcodes`, `concreteproperties`, `sectionproperties`, `eurocodepy`, `blue-prints`, `groundhog`, `sympy`; defer `openseespy`. Via `mcp__repo__dependencies_lock` then `dependencies_sync`; register each in `🔮️oracles/🔣️.json` first (ratchet requires it); verify cp314 wheels for compiled deps (`cytriangle`, `meshpy`) before committing. `eurocodepy` drags docs deps (mkdocs, marimo, plotly, pyzmq): consider whether the ratchet accepts the transitive set or whether to vendor a pin of its source (not recommended; evaluate size first).
2. Land the schema + harness + `--cases` mode, then migrate in order of risk: EN 1992 (strongest A coverage), EN 1993 sections and N_cr, EN 1997 bearing, EN 1998 spectrum/modal, EN 1995 material diff, then the C-class ones.
3. Replace `python3` spawns in the six Rust compliance tests with the resolved interpreter helper.
4. Delete the transliterated formula bodies once the fixture + library path covers a check (no legacy kept). EN 1990's 41-line script is replaced by the enumeration oracle.

## 7. Caveats
- structuralcodes' exact EN 1992 clause list, and eurocodepy/blue-prints per-function coverage were not confirmed in this pass (PyPI summaries and doc index only). The first rollout step is a coverage matrix generated by introspecting the installed packages (`dir()` of their EC modules) into the ticket folder.
- Licenses read from PyPI metadata; structuralcodes (empty field) and openseespy (LICENSE.md) need the license file read.
- Eurocodes cover 2004-era editions; second-generation EN 1992-1-1:2023 and EN 1993-1-1:2022 differ (blue-prints supports 2023). Fixtures must carry the edition.
