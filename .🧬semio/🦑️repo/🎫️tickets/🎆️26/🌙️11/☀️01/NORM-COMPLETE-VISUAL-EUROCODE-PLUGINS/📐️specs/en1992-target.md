# 📐 EN 1992 Target Specification

Norm-complete target for the `en1992` artifact (`s.norm.en1992`): EN 1992-1-1 (general), -1-2 (fire), -2 (bridges), -3 (liquid retaining and containment), -4 (fastenings) with the German National Annexes (DIN EN 1992-1-1/NA, -1-2/NA, -2/NA, -3/NA, DIN EN 1992-4 with its NA). Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`, author role: structural-engineering specification. No source was modified.

## 0. Conventions

- SI base units in the model and in all formulas: N, m, Pa, rad, s, K. Tables below quote engineering units (mm, MPa, kN, kNm) for readability; the snapshot stores SI (the current model stores `f_ck` in Pa, lengths in m). Conversion happens only at the labelled-quantity boundary (`Quantity`/`QuantityKind` of the compliance kernel).
- Sign convention: compression positive in section integration (concrete), tension positive for steel strains in code; axial force `NEd` > 0 compression, `NEd` < 0 tension. Moments are stored as signed vectors (My, Mz); verification uses the resultant direction.
- Every catalogue entry is a pure function `inputs -> outputs + CheckResult` with a `ClauseId` (e.g. `en1992-1-1:6.2.2`), localized names (`LocalizedLabel::native(en, de)`), and a `Remedy` list for failures (see section 8).
- NDP columns: `EC` = EN recommended value, `DE` = value of the German NA. Values marked `[V]` are my best recollection and must be verified against the normative text by the implementer before the clause table is frozen; they are held in a data table (`ndp.en1992.de.json`), never hard coded in logic. Everything else is considered certain.
- Partial factors: ULS persistent/transient `gamma_c = 1.5`, `gamma_s = 1.15`; accidental `gamma_c = 1.2`, `gamma_s = 1.0`; fire `gamma_c,fi = gamma_s,fi = 1.0`; fatigue `gamma_c,fat = 1.5`, `gamma_s,fat = 1.15`; prestress `gamma_p = 1.0` (favourable/unfavourable, DE per NA 2.4.2.2). DE: `alpha_cc = 0.85` (persistent, normal concrete, `alpha_cc = 1.0` for accidental per NA 3.1.6), `alpha_ct = 0.85` (NA 3.1.6(2)), `fcd = alpha_cc*fck/gamma_c`, `fctd = alpha_ct*fctk,0.05/gamma_c`. For concrete above C50/60 NA DE additionally applies `alpha_cc = 0.85` (as `eta` handled via `lambda`, `eta`) with the high-strength reductions of Table 3.1 and 3.1.7(3); `lambda = 0.8 - (fck-50)/400`, `eta = 1.0 - (fck-50)/200` for `fck > 50`.
- Verification result shape: utilisation `eta_u = demand/resistance` (>= 0), status `Pass` if `eta_u <= 1.0`, `Fail` otherwise, `NotApplicable` if the precondition fails, `Incomplete` if inputs are missing (the editor shows what to enter, never an exception).
- Numerical tolerances in tests: closed-form entries 1e-9 relative; numerically integrated entries (strain compatibility, interaction surface) 5e-3 relative against the Python oracle.

## 1. Notation (selected)

`fck, fcm, fctm, fctk,0.05, fcd, fctd, Ecm, epsc2, epsc3, epsc1, epsu, n` concrete; `fyk, fyd, Es, epsuk, k=ft/fy, epsud` steel; `Ac, Ic, u, h0` section; `As, Asw, rho_l, rho_w, d, z, bw, x, xi=x/d, omega` reinforcement; `NEd, VEd, TEd, MEd, MEd,y, MEd,z` actions; `sigma_cp, sigma_s, sigma_c, wk, sr,max, epsilon_sm - epsilon_cm` serviceability; `phi(t,t0), epsilon_cs, epsilon_cd, epsilon_ca` time effects; `P0, Pm0, Pm,t, Delta P` prestress.

## 2. Verification and derivation catalogue

Format per entry: **ID — Clause — Name EN / DE.** Inputs; formulas; NDPs (EC / DE); outputs; criterion; edge cases. IDs are stable keys used by tests, the UI (`ClauseId`), and the viewer legends.

### 2.1 Basis and materials (chapters 2, 3)

**M-01 — 2.4.2, 3.1.6 — Design strength of concrete / Bemessungswerte der Betonfestigkeit.**
Inputs: `fck` [MPa], `alpha_cc`, `alpha_ct`, `gamma_c`, situation (persistent/accidental/fire).
Formulas: `fcd = alpha_cc*fck/gamma_c`; `fctd = alpha_ct*fctk,0.05/gamma_c`; `fctk,0.05 = 0.7*fctm`.
NDP: `alpha_cc` EC 1.0 / DE 0.85 (persistent), 1.0 accidental `[V]`; `alpha_ct` EC 1.0 / DE 0.85; `gamma_c` 1.5 / 1.5 (1.2 accidental).
Outputs: `fcd`, `fctd`. Criterion: derivation. Edge: `fck > 90` outside scope of EN 1992-1-1 (flag `Incomplete`).

**M-02 — Table 3.1 — Concrete property table / Betonkennwerte.**
Inputs: grade `C12/15 ... C90/105` (LC grades for lightweight via EN 1992-1-1 chapter 11 as extension E-11).
Formulas: `fcm = fck + 8`; `fctm = 0.30*fck^(2/3)` for `fck <= C50/60`, `fctm = 2.12*ln(1+fcm/10)` above; `fctk,0.05 = 0.7*fctm`; `fctk,0.95 = 1.3*fctm`; `Ecm = 22*(fcm/10)^0.3` [GPa]; `epsc1 = min(0.7*fcm^0.31, 2.8)` [permille]; `epsc2 = 2.0` for `fck <= 50`, else `2.0 + 0.085*(fck-50)^0.53`; `epsu2 = 3.5` for `fck <= 50`, else `2.6 + 35*((90-fck)/100)^4`; `n = 2.0` for `fck <= 50`, else `1.4 + 23.4*((90-fck)/100)^4`; `epsc3 = 1.75` for `fck <= 50`, else `1.75 + 0.55*(fck-50)/40`; `epsu3 = epsu2` (bilinear).
Outputs: full property row used by every other entry; also `fcm(t)`, `fctm(t)` per M-04.
Criterion: derivation. Edge: non-tabulated `fck` (e.g. 28) is allowed and computed from formulas; flagged `NonStandardGrade`.

**M-03 — 3.1.2(4) and Annex NA — Cube/cylinder mapping and age effects / Zylinder-Würfel-Zuordnung.**
`fck,cube` from the grade table; DE strength gain per NA 3.1.2(5): `fck(t)` for `t < 28 d` and long-term coefficient already in `alpha_cc` (no additional `beta_cc`-based increase).

**M-04 — 3.1.2(5-6) — Strength and tensile strength versus time / Zeitliche Entwicklung.**
`fcm(t) = beta_cc(t)*fcm`; `beta_cc(t) = exp(s*(1 - sqrt(28/t)))`; `s = 0.20` (CEM 42.5R, 52.5N, 52.5R, class R), `0.25` (CEM 32.5R, 42.5N, class N), `0.38` (CEM 32.5N, class S); `fctm(t) = (beta_cc(t))^alpha * fctm`, `alpha = 1` for `t < 28`, `2/3` for `t >= 28`. Outputs used by cracking (M-14, S-30), release of prestress (P-02), creep (A-01). Criterion: derivation. Edge: `t` in days after temperature correction `t_T` (3.1/B.10).

**M-05 — 3.1.5, Fig. 3.2 — Parabola-rectangle stress-strain / Parabel-Rechteck-Diagramm.**
`sigma_c(eps) = fcd*(1 - (1 - eps/epsc2)^n)` for `0 <= eps <= epsc2`; `= fcd` for `epsc2 <= eps <= epsu2`; `0` for `eps < 0` (tension) or `eps > epsu2`.
Outputs: `sigma_c(eps)`, resultant `Fc` and lever of resultant for any strain plane (closed form for rectangular and general polygon via clipping + numerical Gauss-Legendre, see S-01).
Criterion: derivation. Edge: `eps > epsu2` returns `0` and flags "strain out of range".

**M-06 — 3.1.5(1), Fig. 3.3 — Bilinear stress-strain (concrete) / Bilineares Diagramm.** `sigma_c = fcd*eps/epsc3` up to `epsc3`, then `fcd` up to `epsu3`.

**M-07 — 3.1.7(3), Fig. 3.5 — Rectangular stress block / Rechteckspannungsblock.** `lambda`, `eta` as in section 0; block depth `lambda*x`, stress `eta*fcd`; resultant `Fc = eta*fcd*lambda*x*b`, lever arm `z = d - lambda*x/2`. DE: `eta*fcd` is used with the `alpha_cc` already inside `fcd`; when the compression zone narrows towards the compression edge the NA factor `0.9` applies (3.1.7(3), `eta*fcd -> 0.9*eta*fcd`) [V].

**M-08 — 3.2.7, Fig. 3.8 — Reinforcing steel diagrams / Spannungs-Dehnungs-Linie Betonstahl.**
Branch (a) inclined: `sigma_s = fyd + (k*fyk - fyd)/(epsuk - fyd/Es)*(eps - fyd/Es)` for `eps >= fyd/Es`, capped at `epsud = 0.9*epsuk`; `k = (ft/fy)k`. Branch (b) horizontal: `sigma_s = fyd`, no strain limit (or `epsud` per NDP). Elastic branch `sigma_s = Es*eps`. `Es = 200 GPa`.
NDP: `epsud` EC 0.9 `epsuk` / DE 0.9 `epsuk`; ductility classes B500A: `ft/fy >= 1.05`, `epsuk >= 2.5%`; B500B: `>= 1.08`, `>= 5.0%`; B500C: `>= 1.15 and < 1.35`, `>= 7.5%` (DE: B500A/B/C per DIN 488; B500A not allowed for primary members when plastic analysis is used, 5.5, 5.6).
Outputs: `sigma_s(eps)` and tangent stiffness; criterion derivation.

**M-09 — 3.3 — Prestressing steel diagrams / Spannstahl-Linie.** Inclined branch `sigma_p = fp0.1k/gamma_s` to `fpd = fp0.1k/gamma_s` up to `epsud = 0.02` (DE `0.9*epsuk` [V]); elastic `Ep = 195 GPa` (strands) `[V]`; relaxation classes 1/2/3 in P-05.

**M-10 — 3.1.4, Annex B — Creep coefficient / Kriechzahl.** see A-01. **M-11 — 3.1.4(6), B.2 — Shrinkage / Schwinden.** see A-02. Both are materials-level derivations consumed by S-12 (effective modulus), P-04 and D-03.

**M-12 — 3.1.4(2) — Effective modulus / Effektiver E-Modul.** `Ec,eff = Ecm/(1 + phi(inf,t0))`, `alpha_e,eff = Es/Ec,eff`.

**M-13 — 3.1.6(2), 2.3.1 — Elevated temperature and fatigue factors.** See F-01..F-07 and R-01.

**M-14 — 7.1(2) — Cracking moment / Rissmoment.** `Mcr = fctm,fl*Wc` (uncracked, with `fctm,fl = max((1.6 - h/1000)*fctm, fctm)`), `Mcr,P = Mcr + P*(e_p + Wc/Ac)` for prestress. Criterion: state-determination (cracked vs uncracked used by S-24, S-31..S-34).

### 2.2 Durability and cover (4)

**D-01 — 4.2 — Exposure classification / Expositionsklassen.** Inputs: environment descriptors per member face (`X0, XC1-4, XD1-3, XS1-3, XF1-4, XA1-3` (+ DE `WO, WF, WA, WS` moisture classes per NA to 4.2)). Output: set of classes governing durability; `XF` and `XA` classes drive minimum concrete class (D-02). Edge: multiple classes per face; the max over all applies.

**D-02 — Table E.1DE (informative Annex E, normative in NA) — Minimum concrete strength class / Mindestbetonfestigkeitsklasse.** Table lookup: `XC1 C16/20`, `XC2 C16/20`, `XC3 C20/25`, `XC4 C25/30`, `XD1 C30/37`, `XD2 C35/45`, `XD3 C35/45`, `XS1 C30/37`, `XS2 C35/45`, `XS3 C35/45`, `XF1 C25/30`, `XF2 C25/30 (C35/45 without air)`, `XF3 C25/30 (C35/45 without air)`, `XF4 C30/37`, `XA1 C25/30`, `XA2 C35/45`, `XA3 C35/45`, `XM1 C30/37`, `XM2 C35/45`, `XM3 C35/45` `[V]`. Criterion: `fck >= fck,min(exposure)`; remedy: raise grade.

**D-03 — 4.4.1.2 — Minimum cover cmin / Mindestbetondeckung.** Inputs: bar diameter `phi` (or equivalent bundle diameter), structural class `S1..S6`, exposure.
`cmin = max(cmin,b, cmin,dur + Dc,dur,gamma - Dc,dur,st - Dc,dur,add, 10 mm)`.
`cmin,b`: individual bar `phi`; bundle `phi_n = phi*sqrt(nb) <= 55 mm`; DE: additionally `max(phi, dg + 5 mm)` for `dg > 16 mm`.
NDP: `Dc,dur,gamma = 0` EC / DE `0`; `Dc,dur,st = 0`, `Dc,dur,add = 0` EC / DE `0` `[V]`.
`cmin,dur` table (reinforcement, Table 4.4N / DE equivalent): full matrix held in data, not hard coded: rows `X0, XC1, XC2/XC3, XC4, XD1/XS1, XD2/XS2, XD3/XS3`, columns `S1..S6` `[V]`; prestressing steel uses Table 4.5. Cover to prestressing duct: `max(phi_duct, 50 mm)` for post-tension (DE per NA).
Outputs: `cmin`.
Criterion: derivation; `cmin` is checked in D-04.

**D-04 — 4.4.1.1, 4.4.1.3 — Nominal cover cnom / Nennmaß.** `cnom = cmin + Dc,dev`; `Dc,dev` EC 10 mm / DE 10 mm (reduced to 5 mm with quality control `[V]`, 0 mm for precast with measured cover, 4.4.1.3(3-4)); actual `cnom,provided = h - effective-depth geometry` per layer. Structural class derivation (Table 4.3N, DE NA): starting `S4`; `+2` for design life 100 y; `-1` if `fck >= C30/37` (XC1), `-1` slab geometry; `-1` special quality; `-1` special concrete composition; classes limited to `S1..S6`.
Criterion: `cnom,provided >= cnom` per face and per layer. Remedy: increase cover / reduce bar diameter / raise grade. Edge: face-specific exposure; stirrups give the governing cover to the outermost bar.

**D-05 — 4.4.1.2(13) — Cover for bond / Verbund.** `cmin,b >= phi` (cast against ground: `+ 40 mm` base preparation: `cnom >= 75 mm` against prepared ground, 4.4.1.3 DE; `>= 40 mm` with blinding, DE; `>= 75 mm` directly on soil). Included in D-03.

**D-06 — 4.4.1.2, EN 1992-1-2 — Cover for fire.** see R-01 (axis distance).

**D-07 — 4.3 — Durability design life.** Design working life classes (EN 1990 Table 2.1) feed structural class correction in D-04.

### 2.3 Structural analysis (5)

**A-01 — Annex B.1 — Creep coefficient phi(t,t0) / Kriechzahl.**
Inputs: `fcm [MPa]`, `RH [%]`, `h0 = 2Ac/u [mm]`, `t0 [d]` (temperature-adjusted), cement class (`S`,`N`,`R`) giving `alpha`, `t [d]`, loading duration.
Formulas:
`phi(t,t0) = phi0*beta_c(t,t0)`; `phi0 = phi_RH*beta(fcm)*beta(t0)`;
`phi_RH = 1 + (1 - RH/100)/(0.1*h0^(1/3))` for `fcm <= 35`; `phi_RH = [1 + (1 - RH/100)/(0.1*h0^(1/3))*alpha1]*alpha2` for `fcm > 35`;
`beta(fcm) = 16.8/sqrt(fcm)`; `beta(t0) = 1/(0.1 + t0^0.2)`;
`beta_c(t,t0) = ((t - t0)/(beta_H + t - t0))^0.3`; `beta_H = 1.5*[1 + (0.012*RH)^18]*h0 + 250 <= 1500` (`fcm <= 35`); `beta_H = 1.5*[1 + (0.012*RH)^18]*h0 + 250*alpha3 <= 1500*alpha3` (`fcm > 35`);
`alpha1 = (35/fcm)^0.7`, `alpha2 = (35/fcm)^0.2`, `alpha3 = (35/fcm)^0.5`;
`t0` adjusted for cement: `t0 = t0,T*(9/(2 + t0,T^1.2) + 1)^alpha >= 0.5`, `alpha = -1 (S), 0 (N), +1 (R)`; temperature: `t_T = sum exp(-(4000/(273+T(Dt_i)) - 13.65))*Dt_i`.
Outputs: `phi(t,t0)`, `phi(inf,t0)` (with `beta_c = 1`). Non-linear creep for `sigma_c > 0.45 fck(t0)`: `phi_nl = phi*exp(1.5*(k_sigma - 0.45))`, `k_sigma = sigma_c/fck(t0)`.
Criterion: derivation; `Incomplete` if `RH`, `h0`, `t0` missing.
Edge: `h0` from polygon + perimeter exposed to drying only (sealed faces excluded); `RH` outside 40..100 flagged.
See example E-10.

**A-02 — Annex B.2, 3.1.4(6) — Shrinkage strain / Schwinddehnung.**
`epsilon_cs = epsilon_cd + epsilon_ca`.
`epsilon_cd(t) = beta_ds(t,ts)*kh*epsilon_cd,0`; `beta_ds = (t - ts)/((t - ts) + 0.04*sqrt(h0^3))`; `kh`: `h0 = 100: 1.00`, `200: 0.85`, `300: 0.75`, `>= 500: 0.70` (linear interpolation);
`epsilon_cd,0 = 0.85*[(220 + 110*alpha_ds1)*exp(-alpha_ds2*fcm/10)]*1e-6*beta_RH`; `beta_RH = 1.55*[1 - (RH/100)^3]`; `alpha_ds1 = 3 (S), 4 (N), 6 (R)`; `alpha_ds2 = 0.13 (S), 0.12 (N), 0.11 (R)`;
`epsilon_ca(t) = beta_as(t)*epsilon_ca,inf`; `epsilon_ca,inf = 2.5*(fck - 10)*1e-6`; `beta_as = 1 - exp(-0.2*sqrt(t))`.
Outputs: `epsilon_cs(t)`, `epsilon_cs(inf)`. Edge: `RH >= 99%` swelling not covered (flag).

**A-03 — 5.2, 5.3 — Imperfections and geometric models / Imperfektionen.** `theta_i = theta_0*alpha_h*alpha_m`; `theta_0 = 1/200`; `alpha_h = 2/sqrt(l)` bounded `[2/3, 1]`; `alpha_m = sqrt(0.5*(1 + 1/m))`; isolated member `e_i = theta_i*l0/2 = l0/400` (for `alpha_h = alpha_m = 1`); DE additionally `l0/400` with min `e0 = max(h/30, 20 mm)` (6.1(4)). Outputs: `e_i`, `H_i` equivalent horizontal forces, bracing system forces.

**A-04 — 5.3.2 — Effective flange width / Mitwirkende Plattenbreite.** `beff = sum beff,i + bw <= b`; `beff,i = 0.2*bi + 0.1*l0 <= 0.2*l0, <= bi`; `l0` from Fig. 5.2 (`0.85*l`, `0.15*(l1+l2)`, `0.7*l`...). Output: T-section polygon used in S-01.

**A-05 — 5.4 — Linear elastic analysis / Linear-elastische Berechnung.** Cracked/uncracked stiffness (`0.5*EI` cracked allowance, DE NA 5.4 allows rigid ratios), redistribution via A-06.

**A-06 — 5.5 — Linear analysis with limited redistribution / Umlagerung.** `delta >= k1 + k2*xu/d` for `fck <= 50` with `k1 = 0.44`, `k2 = 1.25*(0.6 + 0.0014/epsucu2)`; `delta >= k3 + k4*xu/d` for `fck > 50` (`k3 = 0.54`, `k4 = 1.25*(0.6+0.0014/epsucu2)`); `k5 = 0.7` for classes B/C, `0.8` for A... `xu/d <= 0.45` (B/C, `fck <= 50`), `0.35` (`fck > 50`) [V]; DE: `delta >= 0.64 + 0.8*xu/d` if `fck <= 50` per NA with `Class A` steel limited to `delta >= 0.8` [V]. Outputs: permitted redistribution ratio. Criterion: `delta_actual >= delta_min` and `xu/d <= limit`. Remedy: reduce redistribution or add compression steel.

**A-07 — 5.6 — Plastic analysis, rotation capacity / Plastische Berechnung.** `theta_pl,d <= k_lambda*theta_pl,d(tab)`; `k_lambda = sqrt(lambda/3)`, `lambda = MEd/(VEd*d)`; requires `xu/d <= 0.25 (fck <= 50) / 0.15 (fck > 50)`, steel class B or C; criterion `theta_pl,actual <= theta_pl,d`.

**A-08 — 5.7 — Non-linear analysis (strain-compatible).** Via fem plugin; gating criteria: material design strengths with `gamma_R` global (gamma_O = 1.2 DE `[V]` per 5.7 NA, `Rd = Rm(fcm-based)/gamma_R`).

**A-09 — 5.8.2-5.8.3 — Slenderness and limits / Schlankheit, Grenzschlankheit.**
`lambda = l0/i`, `i = sqrt(I/A)`; `lambda_lim = 20*A*B*C/sqrt(n)`; `n = NEd/(Ac*fcd)`; `A = 1/(1 + 0.2*phi_ef)` (default 0.7); `B = sqrt(1 + 2*omega)` (default 1.1); `omega = As*fyd/(Ac*fcd)`; `C = 1.7 - rm` (default 0.7); `rm = M01/M02`. Effective length `l0` per 5.8.3.2 (braced `l0 = 0.5*l*sqrt((1 + k1/(0.45+k1))*(1 + k2/(0.45+k2)))`, unbraced `l0 = l*max(sqrt(1 + 10*k1*k2/(k1+k2)), (1+k1/(1+k1))*(1+k2/(1+k2)))`, `k = (theta/M)*(EI/l)`).
Criterion: `lambda <= lambda_lim` -> second order effects may be neglected (`NotRequired`). Edge: DE NA 5.8.3.1: isolated members `lambda_lim` also from 25 for `n >= ...`; slender-limit per NA uses `lambda_lim = 25` for `|n| >= 0.41`[V]. Biaxial: check each direction, and `lambda_y/lambda_z <= 2` and `(ey/h)/(ez/b) <= 0.2 or >= 5` for separate checks (5.8.9(3)).

**A-10 — 5.8.8 — Nominal curvature method / Modellstützenverfahren.**
`MEd = M0Ed + M2`; `M2 = NEd*e2`; `e2 = (1/r)*l0^2/c`, `c = pi^2` (~10; constant curvature `c = 8`); `1/r = Kr*Kphi*(1/r0)`; `1/r0 = epsilon_yd/(0.45*d)`; `d = h/2 + is` (`is` = radius of gyration of total reinforcement area); `Kr = (nu - n)/(nu - nbal) <= 1`, `nu = 1 + omega`, `nbal = 0.4`; `Kphi = 1 + beta*phi_ef >= 1`, `beta = 0.35 + fck/200 - lambda/150`; `phi_ef = phi(inf,t0)*M0Eqp/M0Ed`; `M0Ed = max(0.6*M02 + 0.4*M01, 0.4*M02) + NEd*ei`; `e0 >= max(h/30, 20 mm)`.
Output: design moment `MEd` and `e_tot = e0 + ei + e2` for the section check S-04/S-05 (biaxial combined). Criterion: `MEd <= MRd(NEd)` through S-04 with `eta_u = MEd/MRd`.
Edge: stiff-iterative reinforcement `As` unknown -> iterate (`omega` depends on `As`); iteration capped, converged flag; `Kr` uses geometric reinforcement from the model. Biaxial: moments about each axis with `e2` per axis, checked with S-05.

**A-11 — 5.8.7 — Method based on nominal stiffness / Modellstützenverfahren mit Steifigkeit.**
`EI = Kc*Ecd*Ic + Ks*Es*Is`; `Ecd = Ecm/gamma_cE` (`gamma_cE = 1.2`); `Kc = k1*k2/(1 + phi_ef)`; `k1 = sqrt(fck/20)`, `k2 = n*lambda/170 <= 0.20`; `Ks = 1` with `rho >= 0.002`; simplified `Kc = 0.3/(1 + 0.5*phi_ef)` for `rho >= 0.01`, `Ks = 0`. Moment amplification: `MEd = M0Ed*(1 + beta/(NB/NEd - 1))`, `NB = pi^2*EI/l0^2`, `beta = pi^2/c0` (`c0 = 8` constant, `9.6` parabolic...). DE: `Ecd = Ecm/gamma_cE` with `gamma_cE = 1.2`, `K` factors per EC. Criterion: `NEd < NB` (else unstable) and S-04 on amplified moment.

**A-12 — 5.8.9 — Biaxial bending in slender columns.** Section-level check S-05 with amplified moments per axis; Eq. 5.38 shortcuts.

**A-13 — 5.8.5 — General method (non-linear second-order).** Via B-02 curvature integration: iterate `delta(x)` from section `M-N-kappa` relation (moment-curvature using design strengths `fcd` with `fyd/1.1`... ) returns collapse load factor. Criterion: stability factor `gamma_cr >= 1`.

**A-14 — 5.8.6 / 5.8.4 — Second-order effects of slender walls, lateral loads (global).** Global check: `Fv,Ed <= k1*sum Ecd*Ic/L^2*nS/(nS+1.6)`; `k1 = 0.31` (cracked allowance; DE NA `k1 = 0.31` `[V]`); drives "sway/non-sway" classification of the whole frame in the shared compliance kernel.

**A-15 — 5.9 — Lateral instability of slender beams.** `l0t/b <= 50/(h/b)^(1/3)` and `h/b <= 2.5`. Criterion: ratio; edge: ignore if exceeded -> second order lateral.

**A-16 — 5.10 — see section P (prestressed).**

### 2.4 Section resistance — ULS bending, axial, interaction (6.1)

**S-01 — 6.1 — Strain compatibility integrator (the kernel) / Dehnungsebene.**
Inputs: section polygon (outer loop + holes), concrete grade, bar list `{(y_i, z_i, A_si, grade_i)}`, tendon list `{(y_j, z_j, A_pj, steel_j, strain_pre_j)}`, strain plane `(eps0, kappa_y, kappa_z)`.
Formulas: `eps(y,z) = eps0 + kappa_y*z - kappa_z*y`; concrete resultant by clipping the compression zone polygon `eps > 0` and integrating `sigma_c(eps)` (M-05/M-06/M-07) with exact polynomial integration per triangle fan (degree `n` polynomial) -> `N = Int sigma_c dA + sum sigma_si*Asi + sum sigma_pj*Apj`; `My = Int sigma_c*z dA + ...`; `Mz = -Int sigma_c*y dA - ...` (sign convention fixed by schema); pre-strain `eps_p0` added to tendons; bars in compression subtract displaced concrete `sigma_c(eps_i)*A_si`.
Pivot strain domains (Fig. 6.1): domain 1 (steel `epsud`, pivot A), 2 (concrete `eps_c <= epsu2`, pivot A), 3, 4, 4a, 5 (pivot C at `epsc2*(1 - epsc2/epsu2)^(-1)`-strain: `eps_c(h*(1 - epsc2/epsu2))=epsc2`, i.e. at `(1 - epsc2/epsu2)*h` from the compressed edge; for rectangular `3/7*h`), domain 6 (full compression `eps_c = epsc2`).
Outputs: `N(eps)`, `M(eps)`, tangent stiffness matrix for Newton.
Criterion: n/a (kernel). Edge: holes; non-convex polygons; vanishing compression zone; bars outside section flagged; concrete tension cut-off; tendon with `eps` beyond `epsud` -> ruptured status; deterministic result when `kappa_y = kappa_z = 0` (pure axial).

**S-02 — 6.1 — Pure axial resistance / Zentrische Tragfähigkeit.** `NRd,0 = Ac*fcd + As*fyd` (compression, full); `NRd,t = -As*fyd` (pure tension, steel only, concrete cracked); `NRd,0` for columns uses `0.85*...` already inside `fcd`; `eps_c = epsc2` at domain 6 (S-01 exact: `fcd` if `eps >= epsc2`). Used as anchor of 5.39 (S-05). Example E-11.

**S-03 — 6.1 — Rectangular beam, singly reinforced, bending / Biegebemessung Rechteck.**
`mu_Eds = MEds/(b*d^2*fcd)`; `omega = 1 - sqrt(1 - 2*mu)` (rectangular block `lambda=0.8`, `eta=1`: `omega = lambda*xi*eta`, `xi = omega/0.8`); `As1 = (omega*b*d*fcd + NEd)/fyd`; `z = d*(1 - 0.5*omega)`; `eps_s1 = 3.5‰*(1 - xi)/xi` (concrete limit); `MEds = MEd - NEd*zs1`. Limits: `xi <= xi_lim`; DE: `xi <= 0.45` for `fck <= 50` when redistribution used (A-06) and `xi_lim = 0.617` (balanced steel yield) otherwise `eps_s1 >= fyd/Es = 2.175‰` for non-compression-reinforced.
If `mu > mu_lim`: doubly reinforced: `As2 = (MEds - mu_lim*b*d^2*fcd)/((d - d2)*sigma_s2)`, `sigma_s2 = Es*eps_s2 <= fyd`, `eps_s2 = 3.5‰*(xi_lim*d - d2)/(xi_lim*d)`. Outputs: `As1`, `As2`, `z`, `xi`, `eps_s1`. Criterion: `As,provided >= As,required` (`eta_u = As,req/As,prov`). Edge: `mu > 0.5` -> impossible, remedy: increase section; flanged T handled by S-07; `As,min` / `As,max` via D-30.
Example E-01.

**S-04 — 6.1 — Uniaxial N-M interaction (resistance MRd for given NEd) / M-N-Interaktion.** Solve S-01 for `N(eps) = NEd` by 1-D root finding over curvature at fixed pivot (bisection + Newton, monotone in each pivot domain), return `MRd`; also return interaction diagram `{(NRd(x), MRd(x))}` for neutral-axis depths `x in [0, inf)` (plot points: pure tension, `eps_s = epsud`, balanced `x_bal`, domain 5 sweep to `NRd,0`). Criterion: `eta_u = MEd/MRd(NEd)` (or load-path ray intersection, S-06). Edge: `NEd` outside `[NRd,t, NRd,0]` -> Fail with `eta_u = inf`. Example E-11.

**S-05 — 6.1(109) Eq. 5.39 — Biaxial bending (simplified) / Zweiachsige Biegung vereinfacht.**
`(MEdz/MRdz)^a + (MEdy/MRdy)^a <= 1.0`; `a` by linear interpolation: `NEd/NRd = 0.1: a=1.0`, `0.7: a=1.5`, `1.0: a=2.0`; `NRd = Ac*fcd + As*fyd` (S-02); `MRdy, MRdz` from S-04 with `NEd`. Criterion: left side `<= 1`. DE: allowed only for rectangular sections and approx. symmetric reinforcement; else S-06. Example E-11.

**S-06 — 6.1 — Exact biaxial N-My-Mz interaction surface / Exakte Interaktionsfläche.**
Resistance surface `Omega = {(N, My, Mz)}` from S-01 over all strain planes: parametrise by neutral axis angle `alpha in [0, 2pi)` and depth `x in (-inf, inf)` (pivot-aware) -> 2-D mesh of `(N, My, Mz)` points; convex hull + analytic fill (the exact surface is convex for the concrete part only; tension-reinforced zones can produce non-convex dents -> keep raw sampled surface plus `isConvex` flag). Verification: ray from origin through load point `(NEd, MEdy, MEdz)` hits surface at scale `s`; `eta_u = 1/s` (radial) and `eta_N` (constant-N slice, preferred for columns): intersection of constant-`NEd` plane with surface gives a closed curve `(MRdy(phi), MRdz(phi))`; `eta_u = |M_Ed|/|M_Rd(direction)|`.
Output: surface mesh, constant-N slice curve, `eta_u`, governing angle, critical strain plane. Tolerance: `5e-3` against oracle. Edge: surface sampling density adaptive (angle step 5° baseline, 1° refinement near governing); load point on or beyond `NRd,t`/`NRd,0` -> fail trivially; hollow sections (holes) handled by S-01.

**S-07 — 6.1 — T/L/box flanged sections / Plattenbalken.** Neutral axis within flange -> S-03 with `b = beff`; otherwise decompose: `Fc = fcd*(bw*lambda*x + (beff - bw)*min(hf, lambda*x)...)`; closed form `MRd` or via S-01. Output reduces to S-03 outputs with `x > hf` flag.

**S-08 — 6.1(4), 9.2.1.1 — Minimum eccentricity and limits.** `e0 = max(h/30, 20 mm)`; `NEd` axial with `e0` gives `MEd,min`. DE NA 6.1(4): applies to all compression members with `N/(Ac*fcd) > ...` never reduced. Criterion: `MEd >= NEd*e0` added.

**S-09 — 6.1 — Walls, squat, bearing capacity (plain concrete 12).** Unreinforced concrete members (chapter 12): `NRd = b*hw*fcd,pl*Phi`, `fcd,pl = alpha_cc,pl*fck/gamma_c`, `alpha_cc,pl = 0.70` (DE) `[V]`; `Phi = 1.14*(1 - 2*e_tot/hw) - 0.02*l0/hw` (12.6.5.2). Criterion: `NEd <= NRd`; `e_tot = e0 + ei`; DE limit `hw >= 120 mm` cast-in-place `[V]`.

**S-10 — 6.1(6) — Prestressed section bending (pre-strain) / Vorgespannter Querschnitt.** `eps_p(t)` = pre-strain `Pm,t/(Ap*Ep)` + concrete decompression strain + strain increment; ULS via S-01 with `fpd = fp0.1k/gamma_s`, inclined branch; additional reinforcement in compression/tension.

### 2.5 Shear (6.2)

**V-01 — 6.2.1 — Shear design framework / Querkraftnachweis-Ablauf.** Decision: `VEd <= VRd,c` and no calculated shear reinforcement needed (slabs only, or members with minimum links) -> V-02; else V-03/V-04. Cases: reduction of `VEd` near supports with `av <= 2d` -> `beta = av/(2d)` (6.2.2(6), 6.2.3(8), loading on top and direct support) with `beta >= 0.25` for variable strut and the conditions `VEd,red = beta*VEd`; DE NA: restricted to `av/d >= 0.5` [V]. Shear at the face of support vs. `d` from face in the section, 6.2.1(8).

**V-02 — 6.2.2 — Members not requiring design shear reinforcement / Ohne Querkraftbewehrung.**
Inputs: `bw, d, fck, As_l (anchored beyond d), NEd` [for sigma_cp].
`VRd,c = [CRd,c*k*(100*rho_l*fck)^(1/3) + k1*sigma_cp]*bw*d >= (vmin + k1*sigma_cp)*bw*d`;
`k = 1 + sqrt(200/d) <= 2.0`; `rho_l = Asl/(bw*d) <= 0.02`; `sigma_cp = NEd/Ac < 0.2*fcd`; `vmin = 0.035*k^(3/2)*fck^(1/2)` (EC);
NDP: `CRd,c` EC `0.18/gamma_c` / DE `0.15/gamma_c`; `k1` EC `0.15` / DE `0.12`; `vmin` EC `0.035*k^1.5*sqrt(fck)` / DE `(0.0525/gamma_c)*k^1.5*sqrt(fck)` for `d <= 600 mm`, `(0.0375/gamma_c)*k^1.5*sqrt(fck)` for `d > 800 mm` (linear interpolation in between) [V]; DE additional: for `bw/d >= 5`? slabs DE `CRd,c = 0.15/gamma_c` with `k1 = 0.12` and, for loads near supports, `VEd` reduction `beta`; DE for members with `NEd` in tension (6.2.2(2) equation with `k1*sigma_cp` negative) `[V]`.
Also `VRd,c` upper limit `VRd,c <= 0.5*bw*d*nu*fcd` (6.2.2(6)) `nu = 0.6*(1 - fck/250)`.
Outputs: `VRd,c`. Criterion: `VEd <= VRd,c` (`eta_u = VEd/VRd,c`). Edge: `d` in mm in `k`; prestressed single-span uncracked members: `VRd,c = Ib*bw/S*sqrt(fctd^2 + alpha_l*sigma_cp*fctd)` (6.2.2(2)) in zones where `sigma_ct < fctk,0.05/gamma_c`; `alpha_l = lx/lpt2 <= 1`. Example E-04.

**V-03 — 6.2.3 — Members with shear reinforcement, variable strut inclination / Fachwerkmodell, veränderliche Druckstrebenneigung.**
Inputs: `bw, z (= 0.9d default), fywd, Asw, s, alpha (stirrup angle), cot theta, nu1, fcd, alpha_cw, sigma_cp`.
`VRd,s = (Asw/s)*z*fywd*cot theta` (vertical links); `VRd,s = (Asw/s)*z*fywd*(cot theta + cot alpha)*sin alpha` (inclined);
`VRd,max = alpha_cw*bw*z*nu1*fcd/(cot theta + tan theta)` (vertical); `= alpha_cw*bw*z*nu1*fcd*(cot theta + cot alpha)/(1 + cot^2 theta)` (inclined).
Limits: `1 <= cot theta <= 2.5` (EC); DE NA: `1.0 <= cot theta <= (1.2 + 1.4*sigma_cp/fcd)/(1 - VRd,cc/VEd) <= 3.0` (6.2.3(2) NA; `3.0` general; `1.2` for shear cracked state, `VRd,cc = c*0.48*fck^(1/3)*(1 - 1.2*sigma_cp/fcd)*bw*z`, `c = 0.5`) [V]; `nu1`: EC `0.6*(1 - fck/250)`, DE `nu1 = 0.75*nu2`, `nu2 = 1.1 - fck/500 <= 1.0`; `alpha_cw`: EC `1.0` (non-prestressed), DE `1.0` and formula-coupled `fcd` per NA. Required reinforcement: `Asw/s = VEd/(z*fywd*cot theta)`, `fywd = fyk/1.15`. Additional tension force: `Delta Ftd = 0.5*VEd*(cot theta - cot alpha)` (6.2.3(7)) -> shift rule `a_l = z*(cot theta - cot alpha)/2` (9.2.1.3).
Minimum shear reinforcement (9.2.2): `rho_w = Asw/(s*bw*sin alpha) >= rho_w,min = 0.16*fctm/fyk` (DE: `0.16*fctm/fyk` for beams, slabs `0.6*...` reduce `[V]`).
Outputs: `Asw/s required`, `theta`, `VRd,max`, `VRd,s`. Criterion: `VEd <= min(VRd,s, VRd,max)`; select `cot theta` minimising links within limits; fail if `VEd > VRd,max(cot theta = 1)`... (use the largest `VRd,max` at `cot theta` lower bound). Edge: link-shear in flanges (6.2.4): `Asf/sf = vEd*hf/(fyd*cot thetaf)`; `1.0 <= cot thetaf <= 2.0` (`fpl: 1.0 .. 1.25` for tension flanges: `1.0..1.25`); `vEd = Delta Fd/(hf*Delta x)`; crushing `vEd <= nu*fcd*sin thetaf*cos thetaf`. Example E-03.

**V-04 — 6.2.3(103), (5) — Shear with axial force and prestress (stress state).** `sigma_cp` effect on `alpha_cw` (EC: `1 + sigma_cp/fcd` for `0 < sigma_cp <= 0.25 fcd`, `1.25` for `0.25..0.5 fcd`, `2.5*(1 - sigma_cp/fcd)` for `0.5..1.0 fcd`; DE uses `alpha_cw = 1.0` with formula-integrated `sigma_cp`) [V]. Output: `VRd,max` including normal stress. Tendon vertical component: `VEd = VEd0 - Vccd - Vtd`.

**V-05 — 6.2.5 — Shear at interfaces (construction joints) / Schubfuge.**
`vEdi = beta*VEd/(z*bi)`; `vRdi = c*fctd + mu*sigma_n + rho*fyd*(mu*sin alpha + cos alpha) <= 0.5*nu*fcd`;
roughness `c`, `mu`: very smooth `0.025..0.10, 0.5`; smooth `0.20, 0.6`; rough `0.40, 0.7`; indented `0.50, 0.9`; DE `c` rough `0.40`, indented `0.50` with `[V]` limit `fctd`-based `c`; `beta` = ratio of longitudinal force; `sigma_n <= 0.6*fcd`; `rho = As/Ai`. DE: for fatigue/dynamic action `c = 0`. Criterion: `vEdi <= vRdi`. Edge: tension `sigma_n < 0` -> `c*fctd` term vanishes.

**V-06 — 6.2.4 — Shear between web and flanges.** see V-03 edge cases (separate entry for UI: flange link area, anchorage).

**V-07 — 6.2.2(2), 6.2.3 — Shear in slabs w/ concentrated loads close to supports (EC 6.2.2(6)).** `VEd,red = beta*VEd`, `beta = av/(2d)`, compare `VEd,red <= VRd,c` and `VEd <= 0.5*bw*d*nu*fcd`.

**V-08 — 6.2.3 DE — Combined V+T / Querkraft-Torsion.** see T-04.

### 2.6 Torsion (6.3)

**T-01 — 6.3.2(1) — Thin-walled equivalent section / Ersatz-Kasten.** `tef = A/u` (`>= 2*c1` + axis distance, `tef,min = 2*(c + phi_l/2)`); `Ak = (b - tef)*(h - tef)` for rectangle; `uk = 2*((b - tef) + (h - tef))`; general polygon: `A` = enclosed area incl. hollows, `u` = outer circumference; `Ak` from centreline polygon offset by `tef/2`. `tef` variable for I-sections (sub-rectangles).

**T-02 — 6.3.2 — Torsion resistance / Torsionsbemessung.**
`TRd,max = 2*nu*alpha_cw*fcd*Ak*tef*sin theta*cos theta`; `nu` EC `0.6*(1 - fck/250)` / DE `0.75*nu2`... as V-03; `theta` limits as V-03 (`1.0 <= cot theta <= 2.5`; DE: link angle coupled to shear 6.2.3).
Required: stirrups `Asw/s = TEd/(2*Ak*fywd*cot theta)`; longitudinal `sum Asl = TEd*uk*cot theta/(2*Ak*fyd)` (distributed along `uk`).
Criterion: `TEd <= TRd,max`; minimum `TRd,c = 2*Ak*tef*fctd` (no reinforcement needed if `TEd <= TRd,c`). DE: min torsion reinforcement same as 9.2.3: `rho_w >= 0.16*fctm/fyk`, closed links, spacing `s <= uk/8` (9.2.3(3)). Example E-06.

**T-03 — 6.3.2(4) — Torsion in warped sections.** Composite box/T: split into sub-sections proportional to stiffness `T_i = TEd*Ti_i/sum`.

**T-04 — 6.3.2(103), (110) — Combined torsion + shear / Kombination.**
Solid sections: `TEd/TRd,c + VEd/VRd,c <= 1.0` (below this no calculated reinforcement); max: `TEd/TRd,max + VEd/VRd,max <= 1.0`. Box sections: `vEd,T*z...` effective web force `VEd,i = VEd,T + VEd,V` with `VEd,T = TEd*z_i/(2*Ak)`; stirrup and longitudinal requirements added; interplay `theta` identical for V and T. Criterion: utilisation `eta_u = TEd/TRd,max + VEd/VRd,max`. DE: `TEd/TRd,max + VEd/VRd,max <= 1` (6.3.2(4) eq. 6.29) and `VEd <= VRd,c*(1 - TEd/TRd,c)`... [V].

### 2.7 Punching shear (6.4)

**P-01 — 6.4.2 — Basic control perimeters / Rundschnitte.**
Inputs: column geometry (rectangular `c1 x c2`, circular `D`, polygon), position (interior/edge/corner/wall end), openings (6.4.2(3): shadow lines when opening within `6d`), slab thickness `h`, effective depth `d = (dy + dz)/2`, column capital geometry (`hH`, `lH`).
Perimeters: `u0` at column face; `u1` at `2.0d` (EC and DE; control perimeter offset `2d`, rounded corners); `uout` and `uout,ef` outside reinforced zone (`k*d`, `k = 1.5` EC... DE `uout` at `1.5d`... via `kd` with `k=1.5`) [V]; edge/corner reduced perimeters `u1*` (limit by `1.5d` or `0.5*c` from edge); column heads: `rc = lH + 0.5c`, control section at `2d` from column head edge (6.4.2(103)).
Outputs: perimeter polylines (3D visual), lengths `u0, u1, uout`.

**P-02 — 6.4.3 — Load increase factor beta / Lasterhöhungsfaktor.**
General: `beta = 1 + k*(MEd/VEd)*(u1/W1)`; `W1 = integral over u1 of |e| dl` (rectangular: `W1 = c1^2/2 + c1*c2 + 4*c2*d + 16*d^2 + 2*pi*d*c1`); k from Table 6.1 (`c1/c2`).
DE NA simplified (non-sway braced, adjacent spans ratio 0.5..2): interior `beta = 1.1` (NA 6.4.3(6): `1.10` interior, `1.40` edge, `1.50` corner); wall ends and corners `beta = 1.35` `[V]`; if no checked eccentricity.
Outputs: `beta`. Edge: sway-braced unknown -> `beta` computed by full formula; `beta` not less than `1.0`.

**P-03 — 6.4.4 — Punching without shear reinforcement / Ohne Durchstanzbewehrung.**
`vEd = beta*VEd/(u*d)`; `vRd,c = CRd,c*k*(100*rho_l*fck)^(1/3) + k1*sigma_cp >= vmin + k1*sigma_cp`; `k = 1 + sqrt(200/d) <= 2.0`; `rho_l = sqrt(rho_ly*rho_lz) <= 0.02` and `<= 0.5*fcd/fyd` (DE) [V]; `rho_l` in the width `3d + c` each side; `sigma_cp = (sigma_cy + sigma_cz)/2`.
NDP: `CRd,c` EC `0.18/gamma_c` / DE `0.18/gamma_c` (punching, 6.4.4(1) NA) `[V]`; `k1` EC `0.1` / DE `0.12`; `vmin` EC `0.035*k^1.5*sqrt(fck)` / DE `(0.0525/gamma_c)*k^1.5*sqrt(fck)` for `d <= 600 mm`, `(0.0375/gamma_c)` for `d > 800 mm`; for `u1`: near-column enhancement `vRd,c * (2d/a)` with `a < 2d` (6.4.4(2)) (foundations, DE: foundation formula `vEd = beta*(VEd - DVEd)/(u*d)`; `vRd,c = CRd,c*k*(100 rho fck)^(1/3)*2d/a`, `a` = distance from column face, check on controlled perimeters between `d` and `2d`... DE: minimum perimeter iteration over `a`). Foundation check: critical perimeter `a` from `0.5d` to `2d`; minimal `vEd/vRd,c`.
Reduction by soil pressure/loads within perimeter `DVEd`.
Outputs: `vEd`, `vRd,c`, `eta_u = vEd/vRd,c` at `u1`. Max: `vEd <= vRd,max` at `u0`: EC `vRd,max = 0.5*nu*fcd`; DE `vRd,max = 1.4*vRd,c` `[V]` (DE NA 6.4.5(3), applies to slabs with punching reinforcement; the NA text decides the reference perimeter and enhanced values for approved double-headed studs).
Example E-05.

**P-04 — 6.4.5 — Punching shear reinforcement / Durchstanzbewehrung.**
`vRd,cs = 0.75*vRd,c + 1.5*(d/sr)*Asw*fywd,ef*(1/(u1*d))*sin alpha`; `fywd,ef = 250 + 0.25*d <= fywd` [MPa, d mm]; EC: `vRd,cs = 0.75 vRd,c + 1.5 (d/sr) Asw fywd,ef (1/(u1 d)) sin alpha`. DE: `vRd,c` with `CRd,c = 0.15/gamma_c`: for punching with reinforcement `vRd,cs = 0.75 vRd,c + ...` and the maximum `vRd,max = 1.4 vRd,c` (6.4.5(3) NA) `[V]` (`vRd,max = 1.96 vRd,c` for double-headed studs with approval); stud spacing: `sr <= 0.75d`, tangential `st <= 1.5d` (inside `2d`), `st <= 2d` (outside); first stud `0.3d..0.5d` from column; minimum reinforcement `Asw >= 0.08*sqrt(fck)/fyk*(sr*st)/(1.5 sin alpha + cos alpha)`; outer perimeter `uout = beta*VEd/(vRd,c*d)` -> reinforced zone extent `k*d` outside `uout`: radius of reinforcement `rout`.
Outputs: required `Asw` per perimeter, number of perimeters, `rout`, stud layout (3D). Criterion: `vEd <= vRd,cs`, `vEd,u0 <= vRd,max`, `vEd,uout <= vRd,c`. Edge: strut reinforcement of rafts; openings; edge/corner perimeter geometry. Example E-05.

**P-05 — 6.4.5(4) — Shear studs layout per NA / ETA.** studs (DE additional approval `vRd,max` multipliers `[V]`).

**P-06 — 6.4.6 — Punching at columns with internal slab fractions / Flat slab foundation, base plate.** foundations: `VEd,red = VEd - DVEd` with `DVEd = sigma_gd*A_crit`; verification at multiple perimeters.

**P-07 — Annex (DE NA) — Prestressed flat slabs punching.** `vRd,c + 0.1*sigma_cp`; contribution of inclined tendons within `u1`: `Vp = P*sin alpha`.

### 2.8 Strut-and-tie (6.5) and partially loaded areas (6.7)

**ST-01 — 6.5.2 — Concrete strut capacity / Druckstreben.** `sigma_Rd,max = fcd` (no transverse tension, no reinforcement crossing) -> Eq. 6.55; `sigma_Rd,max = 0.6*nu'*fcd` (cracked compression with transverse tension) with `nu' = 1 - fck/250`. Criterion: `sigma_c,strut <= sigma_Rd,max`.

**ST-02 — 6.5.3 — Ties / Zugstreben.** `A_s,tie = Ftd/fyd`; anchored beyond node (8.4.x anchorage in nodes at start of tie: bar anchorage from the beginning of node). Criterion: `A_s,prov >= A_s,tie`.

**ST-03 — 6.5.4 — Nodes / Knoten.** CCC: `sigma_Rd,max = k1*nu'*fcd`, `k1 = 1.0`; CCT: `k2*nu'*fcd`, `k2 = 0.85`; CTT: `k3*nu'*fcd`, `k3 = 0.75`. DE NA values for `k1`, `k2`, `k3` to be taken from the NA text `[V]` (my recollection: `k1 = 1.1`, `k2 = 0.75`, `k3 = 0.75`). Plus triaxial confinement `sigma_Rd,max = 1.1*...` up to `3*...` (6.5.4(5)). Output: nodal stress ratios.

**ST-04 — 6.5.1 — Automatic STM generation (derivation).** Truss topology from force flow (Schlaich B/D regions); the editor lets the user drag nodes and recomputes forces (static determinate truss solver, min 3 constraints). Outputs: strut/tie forces, angle `theta`, strut widths, tie areas, node ratios. Example E-08.

**PA-01 — 6.7 — Partially loaded areas / Teilflächenbelastung.**
`FRdu = Ac0*fcd*sqrt(Ac1/Ac0) <= 3.0*fcd*Ac0`; `Ac1` max distribution area with similarity to `Ac0`; side limits `b <= 3*b0`, heights `h >= (b2 - b1)` and `(d2 - d1)`; splitting reinforcement from `Fd` `T = 0.25*(1 - a/h)*F` (principal tension, 6.5.3(103)): `T = 0.25*F*(1 - a/h)` for partial discontinuity (`b/H <= 0.5` ... ) and `T = 0.25*(1 - 0.7a/h)*F` for full discontinuity (`H/2 < h < H`) (6.5.3). DE NA 6.7: `FRdu = Ac0*fcd*sqrt(Ac1/Ac0) <= 3*fcd*Ac0` with `fcd` incl. `alpha_cc = 0.85` -> `fcd` with `0.7`? `[V]`. Criterion: `FEd <= FRdu`. Edge: multiple adjacent loaded areas must not overlap `Ac1`; rectangular only in closed form, others numeric.

### 2.9 Anchorage, laps, detailing of reinforcement (8)

**B-01 — 8.4.2 — Ultimate bond stress / Verbundspannung.** `fbd = 2.25*eta1*eta2*fctd`; `eta1 = 1.0` good bond, `0.7` otherwise; `eta2 = 1.0` (`phi <= 32`) else `(132 - phi)/100`; `fctd` with `fctk,0.05` capped at C60/75 value (`fctd` for `fck <= 60`); DE `fctd = alpha_ct*fctk,0.05/gamma_c` `[V]` (`alpha_ct = 0.85`). Bond conditions: good if angle `45..90°` from horizontal, or `h <= 250` (any), `h > 250` upper `300 mm` poor, bars within bottom 250 mm or `>= 300` from top.

**B-02 — 8.4.3 — Basic required anchorage length.** `lb,rqd = (phi/4)*(sigma_sd/fbd)`; `sigma_sd <= fyd` (use `fyd` or actual).

**B-03 — 8.4.4 — Design anchorage length.**
`lbd = alpha1*alpha2*alpha3*alpha4*alpha5*lb,rqd >= lb,min`; `lb,min = max(0.3*lb,rqd, 10*phi, 100 mm)` (tension), `max(0.6*lb,rqd, 10*phi, 100 mm)` (compression);
`alpha1 = 1.0` straight, `0.7` for bend/hook/loop (tension, cover `cd > 3*phi` perpendicular);
`alpha2 = 1 - 0.15*(cd - phi)/phi` in `[0.7, 1.0]` (straight), `1 - 0.15*(cd - 3*phi)/phi` in `[0.7, 1.0]` (hook/loop);
`alpha3 = 1 - K*lambda` in `[0.7, 1.0]`, `lambda = (sum Ast - sum Ast,min)/As`, `Ast,min = 0.25*As` (beams), `0` (slabs); `K` = 0.1 / 0.05 / 0 (Fig. 8.4);
`alpha4 = 0.7` welded transverse bar; `alpha5 = 1 - 0.04*p` in `[0.7, 1.0]`, `p` transverse pressure [MPa];
`alpha2*alpha3*alpha5 >= 0.7`. cd = `min(a/2, c1, c)` (straight); `min(a/2, c1)` hooks/loops.
Mandrel diameters: `Dmin >= 4*phi` (`phi <= 16`), `7*phi` (`phi > 16`) (8.3); with 4*phi when `a_b >= ...`. Anchorage by hooks `lb,eq`.
Outputs: `lbd`, `lb,min`, `lb,eq`. Criterion: `l_available >= lbd` (`eta_u = lbd/l_available`). Example E-07.

**B-04 — 8.7.3 — Lap length / Übergreifungslänge.**
`l0 = alpha1*alpha2*alpha3*alpha5*alpha6*lb,rqd >= l0,min`; `alpha6 = (rho1/25)^0.5 in [1.0, 1.5]` (`rho1` = percentage of lapped bars within `0.65*l0`); `l0,min = max(0.3*alpha6*lb,rqd, 15*phi, 200 mm)`; `alpha3` uses `Ast,min = 1.0*As*(sigma_sd/fyd)`. Transverse reinforcement in laps (8.7.4): `sum Ast >= 1.0*As` (single bar), transverse at both ends if `phi >= 20`, spacing `<= 150 mm`; DE: laps of main bars in a given section: all bars in one section if `<= 50%` - `<= 100%` for single-layer slabs with `phi <= 16`... [V]; clear distance `a >= 2*phi` or `20 mm`, longitudinal offset `>= 0.3*l0`.
Compression laps: `rho1` irrelevant; transverse bars beyond ends.

**B-05 — 8.5 — Bundled bars and equivalent diameter.** `phi_n = phi*sqrt(nb) <= 55 mm`; `nb <= 4` vertical compression & laps, `3` otherwise.

**B-06 — 8.8 — Large-diameter bars (> 32 mm).** Transverse/skin reinforcement, no laps without proof; `As,surf >= 0.01*Act,ext`; anchorage `eta2`.

**B-07 — 8.6, 8.9, 8.10 — Mesh, bundles, prestress anchorage.** transmission length `lpt = alpha1*alpha2*phi*sigma_pm0/fbpt`; `alpha1 = 1.0` gradual release, `1.25` sudden; `alpha2 = 0.25` round wires, `0.19` strands (7-wire); `fbpt = eta_p1*eta1*fctd(t)` (`eta_p1 = 2.7` indented wires, `3.2` strands); design `lpt1 = 0.8*lpt`, `lpt2 = 1.2*lpt` (8.10.2.2); dispersion length `ldisp = sqrt(lpt^2 + d^2)`.

**B-08 — 8.4.5 — Anchorage of links and shear reinforcement (8.5).** Hooks `>= 5*phi` (mandrel `4 phi`) / `10 phi`, `70 mm`; welded transverse bars.

**B-09 — 8.2 — Spacing of bars.** Clear distance `>= max(k1*phi, dg + k2, 20 mm)`; `k1 = 1`, `k2 = 5 mm`; DE `k1 = 1`, `k2 = 5 mm`.

### 2.10 Serviceability (7)

**SL-01 — 7.2 — Stress limitation: concrete (characteristic) / Betondruckspannung.** `sigma_c <= k1*fck`, `k1 = 0.6` for XD, XF, XS; DE `0.6`; non-linear creep when `sigma_c(quasi-permanent) > k2*fck`, `k2 = 0.45`.
Uses S-30 stress state. Inputs: characteristic combination `M, N`. Criterion: ratio `sigma_c/(0.6*fck)`.

**SL-02 — 7.2(5) — Stress limitation: reinforcing steel / Stahlspannung.** `sigma_s <= k3*fyk` (characteristic load, `k3 = 0.8`); `sigma_s <= k4*fyk` (imposed deformations only, `k4 = 1.0`); prestressing steel `sigma_p <= k5*fpk` (`k5 = 0.75`) after losses (`k6 = 0.8` before; `k7 = 0.65` ... maximum at jacking `0.8 fpk`, `0.9 fp0.1k`; immediately after transfer `0.75 fpk`, `0.85 fp0.1k`; after losses `0.65 fpk` quasi-permanent `[V]`). DE: `sigma_p,max = min(0.8 fpk, 0.9 fp0.1k)` jacking; `sigma_pm0 <= min(0.75 fpk, 0.85 fp0.1k)`.

**SL-03 — 7.2 — Stress state of cracked section (linear elastic) / Rissmoment- Spannungen im Zustand II.**
Cracked rectangular: `x` from `b*x^2/2 = alpha_e*As*(d - x) + alpha_e*As'*(x - d')`... closed forms; general polygon: solve `S_x(x) = 0` via bisection with polygon clipping; `I_II`, `sigma_c = M*x/I_II`, `sigma_s = alpha_e*M*(d - x)/I_II`, `alpha_e = Es/Ecm` or `Es/Ec,eff` (quasi-permanent). With axial force: `sigma = N/A + ...` eccentric. Outputs: `x`, `I_II`, `sigma_c`, `sigma_s`, `sigma_p`. Edge: uncracked if `sigma_ct,max < fctm` (M-14), compute State I.

**SL-04 — 7.3.2 — Minimum reinforcement for crack control / Mindestbewehrung.**
`As,min*sigma_s = kc*k*fct,eff*Act`; `k = 1.0` (`h <= 300 mm`) ... `0.65` (`h >= 800 mm`) interpolation (edges) for intrinsic; `k = 0.8` for `b <= 300 mm` and `0.5` for `>= 800 mm` (external restraint); `kc = 1.0` pure tension, `kc = 0.4*[1 - sigma_c/(k1*(h/h*)*fct,eff)] <= 1` (rectangular and webs), `kc = 0.9*Fcr/(Act*fct,eff) >= 0.5` (flanges); `k1 = 1.5` (`NEd` compression) `2h*/3h` for tension; `sigma_c = NEd/(b*h)`; `fct,eff = fctm(t)` (early age: 3 d; `fctm,min = 2.9`? in DE NA `fct,eff = 0.5*fctm` for at least slab thickness h <= 300 `[V]`, bridges: `fctm` min). `sigma_s` limited by Table 7.2N (bar diameter) or Table 7.3N (spacing); DE `Table 7.2DE`/`7.3DE` identical. Outputs: `As,min`, min bar diameter adjustment `phi* = phi*(2.9/fct,eff)`; `phi = phi*·fct,eff·kc·hcr/(2.9·2(h-d))`. Criterion: `As,prov >= As,min`. DE: for restraint, `Gleichung (7.1)`; ratio to be applied to member zone with tension.

**SL-05 — 7.3.3 — Crack control without direct calculation: bar diameter / spacing.** Tables 7.2N (`phi*` vs `sigma_s` and `wk` = 0.4/0.3/0.2) and 7.3N (spacing): `sigma_s` [MPa] from cracked-state S-30; `phi*_s` table values `wk = 0.4`: 160 -> 40, 200 -> 32, 240 -> 20, 280 -> 16, 320 -> 12, 360 -> 10, 400 -> 8, 450 -> 6; `wk = 0.3`: 160 -> 32, 200 -> 25, 240 -> 16, 280 -> 12, 320 -> 10, 360 -> 8, 400 -> 6, 450 -> 5; `wk = 0.2`: 160 -> 25, 200 -> 16, 240 -> 12, 280 -> 8, 320 -> 6, 360 -> 5, 400 -> 4 `[V]`; Table 7.3N max spacing (mm): `wk = 0.4`: 160 -> 300, 200 -> 300, 240 -> 250, 280 -> 200, 320 -> 150, 360 -> 100; `wk = 0.3`: 300, 250, 200, 150, 100, 50; `wk = 0.2`: 200, 150, 100, 50 `[V]`; bending: `phi = phi*·(fct,eff/2.9)·(kc·hcr)/(2(h-d))`; tension: `phi = phi*·(fct,eff/2.9)·hcr/(8(h-d))`. Output: pass/fail on (`phi <= phi_max` or `s <= s_max`).

**SL-06 — 7.3.4 — Calculation of crack widths / Rissbreitenberechnung.**
`wk = sr,max*(epsilon_sm - epsilon_cm)`;
`epsilon_sm - epsilon_cm = max[(sigma_s - kt*(fct,eff/rho_p,eff)*(1 + alpha_e*rho_p,eff))/Es, 0.6*sigma_s/Es]`;
`kt = 0.6` short-term, `0.4` long-term; `rho_p,eff = (As + xi1^2*Ap')/Ac,eff`; `xi1 = sqrt(xi*phi_s/phi_p)`; `Ac,eff = b*hc,eff`, `hc,eff = min(2.5*(h - d), (h - x)/3, h/2)`;
`sr,max = k3*c + k1*k2*k4*phi/rho_p,eff` (`s <= 5*(c + phi/2)`); `k1 = 0.8` (high bond), `1.6` (smooth); `k2 = 0.5` (bending), `1.0` (tension), `(eps1 + eps2)/(2*eps1)` general; `k3 = 3.4`, `k4 = 0.425` (EC and DE); for `s > 5*(c + phi/2)`: `sr,max = 1.3*(h - x)`; biaxial angle: `1/sr,max = cos theta/sr,max,y + sin theta/sr,max,z`.
`wk,lim`: EC `0.4` (X0, XC1) `0.3` other; DE Table 7.1DE: `0.4` for XC1 (X0, XC1: 0.4 / `0.2` for prestressed bonded post-tension, ...), `0.3` all other (XC2..XC4, XD, XS, XF) with decompression for prestressed `0.2`/decompression at `QP`, `[V]`; `wk,lim` quasi-permanent (DE: frequent for prestress).
Outputs: `wk`, `sr,max`, `epsilon_sm - epsilon_cm`, `x`, `Ac,eff`, `rho_p,eff`. Criterion: `wk <= wk,lim` (`eta_u = wk/wk,lim`). Edge: uncracked sections (`sigma_ct < fct,eff`) -> `wk = 0`; combined with SL-04. Example E-09.

**SL-07 — 7.4.2 — Deflection limiting span/depth / Verformungsbegrenzung durch Biegeschlankheit.**
`l/d <= K*[11 + 1.5*sqrt(fck)*rho0/rho + 3.2*sqrt(fck)*(rho0/rho - 1)^(3/2)]` (`rho <= rho0`); `l/d <= K*[11 + 1.5*sqrt(fck)*rho0/(rho - rho') + (1/12)*sqrt(fck)*sqrt(rho'/rho0)]` (`rho > rho0`); `rho0 = sqrt(fck)*1e-3`; `K`: simply supported `1.0`, end span continuous `1.3`, interior span `1.5`, flat slab `1.2`, cantilever `0.4`; stress modification `310/sigma_s` where `sigma_s` at mid-span QP (`~ 500/(fyk*As,req/As,prov)`); flanged `beff/bw > 3`: `*0.8`; spans > 7 m (beams/slabs supporting partitions): `*7/leff`, flat slabs > 8.5 m `*8.5/leff`; DE NA: `l/d <= 35` for slabs vs `150/l` (7.4.2(2) NA: limits `l_eff/d <= 35` for brittle partitions, `>= 150/l`... ) with `K` as EC; DE for `sigma_s` uses `1.5 -> ` `[V]`. Output: `(l/d)_lim`, `(l/d)_act`; criterion `(l/d)_act <= (l/d)_lim`. Example E-09b.

**SL-08 — 7.4.3 — Deflection by calculation / Verformungsberechnung.**
`alpha = zeta*alpha_II + (1 - zeta)*alpha_I`; `zeta = 1 - beta*(sigma_sr/sigma_s)^2` (`0` uncracked); `beta = 1.0` short-term, `0.5` sustained/repeated; `sigma_sr/sigma_s = Mcr/M` (bending); curvature `1/r = zeta*(1/r)_II + (1 - zeta)*(1/r)_I`; creep: use `Ec,eff` (M-12); shrinkage curvature `1/rcs = epsilon_cs*alpha_e*S/I` (`S` = first moment of reinforcement area about section centroid; `alpha_e = Es/Ec,eff`); deflection by integration of curvatures along the member (double integration `w = Int Int kappa dx dx` or virtual work `w = Int kappa*m dx`). Limits (EN 1990/7.4.1): `l/250` total quasi-permanent; `l/500` after construction (brittle finishes); DE NA: `l/250` / `l/500` standard. Output: `w(x)` curve (viewer), `w_max`, `w_after_construction`. Criterion: `w <= l/250` etc.

**SL-09 — 7.1 — Serviceability analysis (cracking & stress states in prestressed members).** Decompression check: `sigma_c <= 0` under QP for XD/XS bonded tendons within 100 mm (DE NA 7.3.1 Table 7.1DE) and `sigma_c,t >= -fctm` at SLS charakteristic etc.

### 2.11 Fatigue (6.8)

**F-01 — 6.8.1-6.8.2 — Fatigue verification framework / Ermüdung.** Verification needed unless exempt (6.8.1(2)): bridges/crane tracks; simplified check `Delta sigma_s,equ` or damage accumulation.
**F-02 — 6.8.3 — Internal forces and stresses for fatigue / Spannungen.** Stress range from S-30 with `Qfat` and permanent `Gk + Pm` (cracked section, no tension in concrete) with `alpha_e` per Tab; stress amplification for steel `eta = ...`; bond difference prestress/reinforcement: `eta = (As + Ap)/(As + Ap*sqrt(xi*phi_s/phi_p))`; `Delta sigma_s = eta*...`.
**F-03 — 6.8.4 — Stress ranges for steel (S-N curves).** Table 6.3N: straight bars `N* = 1e6`, `k1 = 5`, `k2 = 9`, `Delta sigma_Rsk(N*) = 162.5 MPa` (`phi <= 16`... same), welded bars `58.5`, couplers `35`; prestressing steel pretensioned `185`, post-tensioned single strand in plastic duct `185`, curved ducts `150`, couplers `80`. DE NA 6.8.4: `Delta sigma_Rsk = 162.5` (straight, `phi <= 28`? `[V]`); `gamma_s,fat = 1.15`.
**F-04 — 6.8.5 — Verification of steel (damage equivalent stress range).** `gamma_F,fat*Delta sigma_s,equ(N*) <= Delta sigma_Rsk(N*)/gamma_s,fat` with `gamma_F,fat = 1.0`; damage accumulation Palmgren-Miner `D_Ed = sum n_i/N_i <= 1` with S-N slopes `k1`, `k2`.
**F-05 — 6.8.7 — Concrete compression.** `Ecd,max,equ + 0.43*sqrt(1 - Requ) <= 1`; `Requ = Ecd,min,equ/Ecd,max,equ`; `Ecd,max,equ = sigma_cd,max,equ/fcd,fat`; `fcd,fat = k1*beta_cc(t0)*fcd*(1 - fck/250)`, `k1 = 0.85`; simplified `sigma_c,max/fcd,fat <= 0.5 + 0.45*sigma_c,min/fcd,fat` (`<= 0.9` for `fck <= 50`, `0.8` for `fck > 50`).
**F-06 — 6.8.7(2) — Concrete shear fatigue (without links).** `VEd,max,equ/VRd,c <= 0.5 + 0.45*VEd,min,equ/VRd,c <= 0.9` (`fck <= 50`); with links: strut inclination reduced, `cot theta_fat = sqrt(cot theta) <= 1.5`.
**F-07 — 6.8.6 — Equivalent damage λ-method (bridges).** `lambda_s` from EN 1992-2 Annex NN (Eurocode 2-2: 6.8.5): `Delta sigma_s,equ = phi_fat*Delta sigma_s,71*lambda_s`; `lambda_s = lambda_s,1*lambda_s,2*lambda_s,3*lambda_s,4 <= lambda_s,max`; road/rail variants.

### 2.12 Prestress (5.10, 3.3)

**PR-01 — 5.10.2.1 — Maximum stressing force / Höchstkraft.** `Pmax = Ap*sigma_p,max`, `sigma_p,max = min(k1*fpk, k2*fp0.1k)`, `k1 = 0.8`, `k2 = 0.9` (EC, DE) `[V]`.
**PR-02 — 5.10.3 — Initial prestress force.** `Pm0 = Ap*sigma_pm0`, `sigma_pm0 <= min(k7*fpk, k8*fp0.1k)`, `k7 = 0.75`, `k8 = 0.85`.
**PR-03 — 5.10.4 — Immediate losses, pre-tensioning / Sofortige Verluste Vorspannung mit sofortigem Verbund.**
Elastic shortening: `DPel = Ap*Ep*sum[j*Dsigma_c(t)/Ecm(t)]`, `j = (n-1)/(2n)` post-tension; `Delta sigma_el = alpha_e*sigma_c,p` pre-tension. Thermal/relaxation during heat curing per 10.3.2.2.
**PR-04 — 5.10.5 — Immediate losses, post-tensioning / Sofortige Verluste Vorspannung mit nachträglichem Verbund.**
Friction: `DPmu(x) = Pmax*(1 - exp(-mu*(theta + k*x)))`; `mu` per Tab (strand in steel duct `0.19`, DE per approval `mu = 0.17..0.2`), `k = 0.005..0.01 rad/m` (`k` unintentional angle per m `0.005`) `[V]`; `theta` = sum of angular deviations over `x`. Anchorage slip: `DPsl` zone length `x_sl = sqrt(Ep*Ap*sl/(p))` with `p = Pmax*(mu/r + k)` (friction gradient), loss `2*p*x_sl`; deflection diagram (triangular loss region). Elastic shortening: sequence of tendons. Output: `Pm0(x)` along the tendon (visual: colour map on tendon). Example E-11 (friction).
**PR-05 — 5.10.6 — Time-dependent losses / Zeitabhängige Verluste.**
`Delta sigma_p,c+s+r = [epsilon_cs*Ep + 0.8*Delta sigma_pr + (Ep/Ecm)*phi(t,t0)*sigma_c,QP] / [1 + (Ep/Ecm)*(Ap/Ac)*(1 + Ac*zcp^2/Ic)*(1 + 0.8*phi(t,t0))]`;
relaxation `Delta sigma_pr`: class 1 `Delta sigma_pr/sigma_pi = 5.39*rho1000*exp(6.7*mu)*(t/1000)^(0.75(1-mu))*1e-5`; class 2 `0.66*rho1000*exp(9.1*mu)*(t/1000)^(0.75(1-mu))*1e-5`; class 3 `1.98*rho1000*exp(8.0*mu)*(t/1000)^(0.75(1-mu))*1e-5`; `mu = sigma_pi/fpk`; `rho1000 = 8% (1), 2.5% (2), 4% (3)`; `t` in hours (`500000` h for 57 years); DE: `rho1000 = 2.5%` class 2 usual for strands (`1.5% <= rho1000` in some ETA). Long-term `Pm,t = Pm0 - DP_c+s+r`.
Outputs: `Pm,t(x,t)`, `Delta sigma_p,c+s+r`. Criterion: `sigma_p,m,t <= 0.65*fpk` (QP, 7.2(5) DE) [V]; `Pm,t >= Pm,t,min required` for decompression. Example E-11b.
**PR-06 — 5.10.8 — Ultimate limit state bending of prestressed members.** S-10 (pre-strain), `Ap` strain increment, `DEpsilon_p = ...`; unbonded: `Delta sigma_p,ULS` = 100 MPa (DE NA allows `Delta sigma_p,ULS = 50..100`) `[V]`.
**PR-07 — 5.10.9 — Effects of prestressing at ULS.** `gamma_p = 1.0`; `Pk,sup = rsup*Pm,t`, `Pk,inf = rinf*Pm,t`; `rsup/rinf`: `1.05/0.95` (bonded post-tension, pre-tensioning); `1.10/0.90` (unbonded or external) (DE `[V]`); SLS decompression check `rinf`, cracking `rsup`.
**PR-08 — 5.10.10 — Prestress deviation forces / Umlenkkräfte.** `q = P/r` (parabolic: `q = 8*P*f/l^2`); verified against bursting: `PA-01` for anchorage zones (`6.5.3(104)`): `T = 0.25*(1 - a/h)*P`.
**PR-09 — 8.10.3 — Anchorage zones of post-tensioned members.** Bursting: `T = 0.25*(1 - a/h)*Pmax` (partial), spalling at the loaded surface; local zone: `FRdu` per PA-01.
**PR-10 — 5.10.2.2 — Limitation of concrete stress at release.** `sigma_c <= 0.6*fck(t)` at transfer (`0.7` with proven non-linear creep).

### 2.13 Slabs, flat slabs, detailing of members (9)

**DT-01 — 9.2.1.1 — Minimum and maximum tension reinforcement in beams / Mindest- und Höchstbewehrung Balken.**
`As,min = max(0.26*fctm/fyk*bt*d, 0.0013*bt*d)`; DE: `As,min` also for crack control SL-04; `As,max = 0.04*Ac`; bar spacing.
**DT-02 — 9.2.1.2-9.2.1.4 — Reinforcement at end supports and shift rule.** Support tension `As >= 0.25*As,span` (simply supported end), force `FEd = |VEd|*a_l/z + NEd` at support `a_l = z*(cot theta - cot alpha)/2`; shift rule: anchorage length to extend `a_l` from the point of inflection; intermediate support: top tension bars anchor.
**DT-03 — 9.2.2 — Shear reinforcement beams.** `rho_w,min = 0.16*fctm/fyk`; max spacing `sl,max = 0.75*d*(1 + cot alpha)`; transverse `st,max = 0.75*d <= 600 mm`; link legs.
**DT-04 — 9.2.3 — Torsion links.** closed, `s <= min(u/8, sl,max, b)`.
**DT-05 — 9.2.4 — Surface reinforcement (high beams, lumped bars).** `As,surf,min = 0.01*Act,ext` perpendicular to main bar; spacing `<= 150 mm`.
**DT-06 — 9.3.1 — Solid slabs: reinforcement.** `As,min` as DT-01; secondary `>= 0.2*As`; spacing `smax = min(3*h, 400 mm)` principal, `min(3.5*h, 450 mm)` secondary; concentrated loads `2*h, 250 mm`; DE `smax = 250 mm` in cracked zones (`h >= 250`) `[V]`; support: `As >= 0.25*As,span`; free edges: edge reinforcement; corners: twisting moment reinforcement (`mxy`, top and bottom `>= ...`).
**DT-07 — 9.3.2 — Slabs shear reinforcement.** `h >= 200 mm` for links; `VEd <= 1/3*VRd,max`: min links `0.6*` of beam; no links permitted if `VEd <= VRd,c`.
**DT-08 — 9.4 — Flat slabs: reinforcement at internal columns, edges.** Top reinforcement over columns within `0.125*width` each side of column; `As >= 0.5*At/(...)`: `0.5*At` within width `0.125*panel width` each side (edge column), bottom `>= 2` bars through column `As >= 0.5*...` integrity (continuous bars `phi 12` (DE two `phi` through column as accidental, `As = FEd/fyd`, `FEd = VEd`) (9.4.1(3) tie to prevent progressive collapse), moments in edge/corner columns `M_Ed` transferred `<= 0.17*b_e*d^2*fck` (Annex I.1.2).
**DT-09 — 9.5 — Columns detailing.** `phi_min = 8 mm` (DE `phi >= 12 mm`, `rho_min = 0.15*NEd/fyd >= 0.002*Ac`), `As,max = 0.04*Ac` (`0.08*Ac` at laps); minimum bars in rectangular `4`, circular `4`; links `phi_t >= max(6, phi/4)`, spacing `scl,tmax = min(20*phi_min, b, 400 mm)` (DE: `scl,tmax = 12*phi_min` if near sections `>= h`, `0.6*scl,tmax` at distances `h` above/below beams, slabs, and laps with `phi > 14`, `0.6 scl,tmax` and `phi_t`). Bars restraint: every bar at corner restrained, bars `>= 150 mm` from restrained bar.
**DT-10 — 9.6 — Walls.** `As,v,min = 0.002*Ac` (DE `0.15*NEd/fyd >= 0.0015*Ac` unreinforced wall `[V]`), max `0.04*Ac`; spacing `<= min(3*tw, 400 mm)`; horizontal: `As,h,min = max(0.25*As,v, 0.001*Ac)`; spacing `<= 400 mm`; links where `As,v > 0.02*Ac`; DE `h >= 100 mm`? plain `[V]`.
**DT-11 — 9.7 — Deep beams.** Surface reinforcement orthogonal mesh `0.001*Ac >= 150 mm2/m`, spacing `<= min(2*h, 300 mm)` per face; main reinforcement distributed.
**DT-12 — 9.8 — Foundations.** Pile caps: tie anchored beyond node, `Ftd` via STM (ST-04), distributed bands over piles `1.5*pile diameter`; footings: strip, `Ftd` per Annex J; column-foundation joint: punching P-06, anchorage B-03; `As,min` `0.0015*...`; column starter bars lap.
**DT-13 — 9.9 — Regions with discontinuity.** Annex J (nodes, corner joints, frame corners): corner force `Ftd`, anchorage radius `r >= phi_m` etc.
**DT-14 — 9.10 — Tying systems.** Peripheral ties `Ftie,per = min(20*...) = li*q1 <= q2` (`q1 = 10 kN/m`, `q2 = 70 kN`), internal ties `Ftie,int = 20 kN/m` (`>= 15 kN/m`), horizontal column/wall ties `Ftie,col = 150 kN/m` or `<= 2% ... ` or DE (`ft = 20 kN`, `Ftie, per` per DE NA 9.10 `[V]`), vertical ties. Criterion: `As,prov*fyk/1.0 >= Ftie` (accidental `gamma_s = 1.0`).

### 2.14 Fire (EN 1992-1-2)

**R-01 — 5.1-5.7 — Tabulated data / Tabellenverfahren.** Per member type: columns (Table 5.2a: `mu_fi`, `b_min/a`; method B `omega`, `lambda`, `e/b` Table 5.2b), walls (5.4: load-bearing, non-load-bearing, fire walls), tensile members (5.5 ties), beams (Table 5.5/5.6/5.7: simply supported, continuous, `b_min/a`, with `asd = a + 10 mm` for corner bars `b <= bmin`), slabs (5.8: simply supported `hs/a`, continuous 5.9, flat slab 5.10, ribbed slabs 5.11-5.13), `a` = axis distance of main reinforcement from nearest fire-exposed surface (`a = c + phi_stirrup + phi_main/2`), classes `R30, R60, R90, R120, R180, R240`; interpolation linear between tabulated points; DE NA: Table 5.2a `mu_fi` formula `mu_fi = NEd,fi/NRd` with `NEd,fi = eta_fi*NEd`, `eta_fi = (Gk + psi_fi*Qk,1)/(gamma_G*Gk + gamma_Q,1*Qk,1)` (DE `psi_fi = psi_1,1` or `psi_2,1`, `eta_fi = 0.7` simplification only for normal buildings). Full tables stored as data files (`en1992-1-2.table.5.x.json`, 12 tables, each with rating rows and dimension/axis distance columns). Outputs: `(b_min, a)` required per R-class, `R_achieved`. Criterion: `b >= b_min(R)` and `a >= a_min(R)` with interpolation. Edge: 5.3.1: additional `10 mm` for `fck >= 70 (spalling)`, `h <= ...` ; spalling check: `fck > C80/95`? `[V]`. Example E-12.
**R-02 — Annex A — Temperature profiles / Temperaturprofile.** Standard fire ISO 834 `theta_g = 20 + 345*log10(8t + 1)` [t in min, `theta` in deg C]; temperature in sections: tabulated profiles (Figs. A.1-A.x for slabs `h = 200 mm`, beams `b_min = 300..600` at times 30..240 min) for normal weight concrete `siliceous`; digital implementation: 2-D finite difference solver of `rho_c*c_p*dT/dt = div(lambda_c grad T)` with surface boundary `q = alpha_c*(theta_g - theta) + eps_m*eps_f*sigma*((theta_g+273)^4 - (theta+273)^4)`, `alpha_c = 25 W/m2K`, `eps_f = 1.0`, `eps_m = 0.7`, with temperature-dependent `c_p(theta)` (including moisture peak 3%: 1470 J/kgK at 100..115 deg C), `lambda_c` upper limit (`2 - 0.2451*(theta/100) + 0.0107*(theta/100)^2`), see R-04. Output: temperature field `T(y,z,t)`, isotherm contours (viewer).
**R-03 — 4.2.3, Annex B — 500 degC isotherm method / Isothermenmethode.**
Reduction `a_z`: determine 500 deg C isotherm from R-02; reduced section: remove concrete with `theta > 500`; `a_z` effective damaged zone `a500` = distance to isotherm; concrete strength: `fcd,fi(20) = fck` (`gamma_c,fi = 1.0`, `alpha_cc` = 1.0), steel `fyk(theta)`; steel temperature `theta_s` at each bar from R-02; `ks(theta)` reduction (EN 1992-1-2 Table 3.2a, hot-rolled bars `1.0` to 400, `0.78` at 500, `0.47` at 600, `0.23` at 700, `0.11` at 800, `0.06` at 900, `0.04` at 1000, `0.0` at 1200 deg C `[V]`); bending resistance `MRd,fi = As*ks*fyk*(d - lambda*x/2)`; `MEd,fi <= MRd,fi`. Concrete in cooler zone `theta <= 500`: full strength. Criterion: `MRd,fi >= MEd,fi`. Edge: re-use S-01 with reduced polygon and per-bar `ks`; `a_z` corner rounding `r = ...`.
**R-04 — Annex B.2 — Zone method / Zonenmethode.**
Divide into zones `i = 1..n`, mean temperature `theta_i`, strength factor `kc(theta_M)` with `kc,m = (1 - 0.2/n)*(1/n)*sum kc(theta_i)`; damage thickness `az = w*(1 - kc,m/kc(theta_M))`, `w` = half width of wall/slab segment; `theta_M` temperature at wall/segment mid-point; reduced section `b' = b - 2az`. Steel via `ks(theta_s)`. Output: `az`, reduced section, `MRd,fi`.
**R-05 — Annex C.3.3 — Calculation of columns exposed to fire (method B/ "simplified calculation method").** Column slenderness at fire `lambda_fi`, `e_fi`, second order, `M_Rd,fi(NEd,fi)` via section integral; restricted to `lambda_fi <= 80`.
**R-06 — 2.4.2, 4.2 — Fire action and load combination.** `Ed,fi = Sum gamma_GA*Gk + psi_1,1 or psi_2,1*Qk,1 + sum psi_2,i*Qk,i + Ad(t)`; `gamma_GA = 1.0`; `eta_fi`. Resistance at fire design strengths `gamma_M,fi = 1.0`. Output: `Ed,fi` for R-01..R-05.
**R-07 — 3.2, 3.3 — Material properties at elevated T.** `kc(theta)` Table 3.1 siliceous: 20: 1.00; 100: 1.00; 200: 0.95; 300: 0.85; 400: 0.75; 500: 0.60; 600: 0.45; 700: 0.30; 800: 0.15; 900: 0.08; 1000: 0.04; 1100: 0.01; 1200: 0.00 `[V]`; `epsc1(theta)`, `epsc,u1(theta)`; thermal elongation `epsc(theta)`; steel `ks(theta)`, `Ep` ... stored in data tables.
**R-08 — 4.5, 5.6 — Shear, torsion, anchorage at fire.** `Ed,fi` with the reduced section; punching fire via `tabulated 6.5.2`; spalling `4.5.1`: sections tabulated spalling for `fck > C80/95`, polypropylene fibres  `>= 2 kg/m3`.

### 2.15 Bridges (EN 1992-2 + NA(D)) / Brücken

**BR-01 — 3.1.2, 3.1.5 DE — Materials for bridges.** `alpha_cc = 0.85` for bridges persistent, `fck <= C70/85`, additional DE NA 3.1.6 factor `0.8` for long-term cc in bridges with prestress `[V]`; `fcd,fat` (F-05); stainless/galvanised excluded.
**BR-02 — 4.4.1.2 — Cover in bridges.** `cmin,dur` per structural class S4+ (Table 4.4N Bridge: design life 100 yrs -> `S6`; 4.4.1.3 DE `Dc,dev = 10 mm` (5 mm precast)) `[V]`; additional `Dc,dur,add` for chloride: +10 mm DE (`XD3`/`XS3`) `[V]`.
**BR-03 — 6.2.2, 6.2.3 — Shear in bridges.** `CRd,c = 0.18/gamma_c` EN 1992-2 `/ 0.15 /gamma_c` DE; `vmin` as DE; `cot theta` limits 1.0..2.5 (DE bridges `1.0 <= cot theta <= 3.0` `[V]`), `alpha_cw = 1.0`; shear between web and flange; DE requires `Nachweis der Schubfugen`  V-05 with `c=0` under fatigue; slabs with transverse prestress: reduced `VRd,max`.
**BR-04 — 6.8 — Fatigue of bridges.** F-07 lambda coefficients (`lambda_s,1` for road bridges from span and traffic category; rail `lambda_s,1` table), `Delta sigma_Rsk` DE; check at tendons curved ducts and couplers.
**BR-05 — 7.3 — Crack control bridges.** `wk,lim` DE bridges: prestressed `wk <= 0.2 mm` frequent? or decompression for bonded `XD/XS` (DE ZTV-ING requires decompression under frequent + `0.7·...`) `[V]`; minimum reinforcement `As,min` with `kc`/`k` and `fct,eff = max(fctm(3 d), 3.0 MPa)` `[V]`; crack control temperature effect.
**BR-06 — 5.10 — Prestress for bridges.** Segmental, external tendons: deviation forces PR-08; secondary effects (hyperstatic) `P*e` of continuous girder: `M_sec = M_total - P*e`; time-dependent loss via step-by-step (construction stage); EN 1992-2 7.3.1 `sigma_c <= 0` decompression frequent; torsion in box girders T-02..T-04 with warping.
**BR-07 — 6.9 — Bearings, 6.10 — Prestressed anchorage, 6.5 STM for diaphragms, cantilever slabs punching under wheel loads (EN 1991-2 LM1 wheel print `0.4x0.4 m`, load dispersion 45° to mid-plane) -> P-03 with wheel loads, `beta = 1.0`**.
**BR-08 — 12 (bridges) — Piers and compression members with large geometry.** Biaxial effects 5.8.9, creep 5.8.4 `phi_ef` etc. (A-09..A-12); EN 1992-2 5.8.7.2 modifications: `k1 = sqrt(fck/20)`, `k2 = n*(lambda/170)` same.
**BR-09 — EN 1992-2 6.8.4 — Reinforcement fatigue stress range of bridges (document check).** The stress range of reinforcing steel under the frequent fatigue combination is verified against the characteristic resistance of the S-N curve divided by `gamma_s,fat`: `gamma_F,fat * Delta sigma_s,equ <= Delta sigma_Rsk / gamma_s,fat` (EN 1992-1-1 6.8.4(1), EN 1992-2 6.8.4, `Delta sigma_Rsk` Table 6.3N/NA(D), `gamma_s,fat = 1.15`). Document counterpart of the catalogue entry BR-04; limit NDP `ndp.en1992.f-03.1` stays `[V]`.

### 2.16 Liquid retaining and containment structures (EN 1992-3 + NA)

**LQ-01 — 7.3.1 — Tightness classes / Dichtheitsklassen.** `0 / 1 / 2 / 3`, `wk,lim`: class 0: EN 1992-1-1; class 1: `wk,lim = 0.2 mm` (`hD/h <= 5`) up to `0.05 mm` at `hD/h >= 35` (linear: `wk1 = 0.2`, `0.05`); class 2: `wk2 = 0.2 mm`... Table 7.105: `hD/h <= 5: 0.20`, `hD/h = 35: 0.05` linear [V]; class 3: no cracks through; class 2 crack widths limited by `wk,max` `0.1`/`0.05 mm` plus partial depth compression zone `>= min(50 mm, 0.2 h)` (7.3.1(103)); class 3 full cross-section tension not allowed. `hD` = hydrostatic head.
**LQ-02 — 7.3.2 — Minimum reinforcement and crack control for restrained members.** Early-age cracks: `As,min = kc*k*fct,eff*Act/sigma_s` with `fct,eff = fctm(t)` for `t = 3 d` or `fctm` with  `rho_p,eff` per Annex M `M.1`: cracking strain `epsilon_sm - epsilon_cm = Rax*epsilon_free` (Eq. M.1): `Rax` = restraint factor (0.5 for bridged on-existing slab..., Table); `wk = sr,max*(epsilon_sm - epsilon_cm)`; `epsilon_free = epsilon_ca + epsilon_cd + alpha_c*(deltaT)`; `delta T` hydration heat.
**LQ-03 — 7.3.4 — Crack width calculation for liquid structure.** `wk` per SL-06 with `kt = 0.4`, `sr,max = (3.4c + 0.425 k1 k2 phi/rho_p,eff)`; for hydrostatic head `hD` on bending/tension.
**LQ-04 — 7.3.1(105) — Self-healing crack width limits.** `wk,lim` allowed up to `0.2 mm` where `hD/h <= 5` and `wk <= 0.1 mm` — DE NA tightness class 1 Table 7.105DE `[V]`.
**LQ-05 — 4.4 — Cover for liquid tanks.** `cmin` increased by `5 mm` `Dc,dur,add`? `[V]`; Aggressive liquids: EN 206 `XA` classes, protective linings (EN 1992-3 4.2).
**LQ-06 — 7.4 — Thermal and shrinkage restraint; prestressed tanks.** Hoop prestress `p = n*P/(R*...)`, losses via PR-04..PR-05; tendon spacing; strands ring `P = Fring/(...)`; decompression in the vertical direction; `sigma_c(compression zone) >= 1.0 MPa`  `[V]`.
**LQ-07 — Section 8 — Details: joint design, water bars, wall-slab corners.** (data-driven checklist: no calc).
**LQ-08 — EN 1992-3 7.3 — Crack width of liquid retaining members (document check).** Per tightness class the calculated crack width is limited, `w_k <= w_k,lim(tightness class, hD/h)` (EN 1992-3 7.3.1(105), Table 7.105 `[V]`), with `w_k = s_r,max * (epsilon_sm - epsilon_cm)` per EN 1992-1-1 7.3.4 (EN 1992-3 7.3.4); class 0 has no requirement (`NotApplicable`). Document counterpart of LQ-01 and LQ-03.

### 2.17 Fastenings to concrete (EN 1992-4 + NA)

Anchor group modelling: anchors `n` with coordinates `(xi, yi)`, spacing `s1, s2`, edge distances `c1..c4`, embedment `hef`, diameter `d`, head/plate geometry, fixture base plate rigidity (stiff plate assumption `elastic` distribution), cracked/uncracked, load vector (N, V, Mx, My, Mz) -> anchor loads `NEd,i`, `VEd,i`.

**FA-01 — 6.2.2 — Anchor load distribution / Lastverteilung.** `NEd,i = NEd/n ± M*Ai/I`, elastic plate theory; compression zone in plate; `VEd,i` distributed equally (or by hole clearance with oversize holes, 6.2.2) ; lever arm `ℓ` for bending of fixture; eccentricity `eN = M/N`.
**FA-02 — 7.2.1.2 — Steel failure, tension.** `NRk,s = As*fuk` (headed `As = pi*d^2/4` stress area); `NRd,s = NRk,s/gamma_Ms`; `gamma_Ms,N = 1.2*fuk/fyk >= 1.4` for `fuk <= 800` and `fyk/fuk <= 0.8`; `1.5` otherwise; group: each anchor.
**FA-03 — 7.2.1.3 — Concrete cone (breakout) tension / Betonausbruch.**
`NRk,c = N0Rk,c*(Ac,N/A0c,N)*psi_s,N*psi_re,N*psi_ec,N*psi_M,N`;
`N0Rk,c = k1*sqrt(fck)*hef^1.5` [N, fck MPa, hef mm]; `k1 = 7.7` (post-installed, cracked) `11.0` (uncracked); cast-in headed: `8.9` cracked, `12.7` uncracked; `A0c,N = scr,N^2 = (3*hef)^2`, `ccr,N = 1.5*hef`; `Ac,N` actual influence area (rectangle bounded by edges and neighbouring cones); `psi_s,N = 0.7 + 0.3*c/ccr,N <= 1`; `psi_re,N = 0.5 + hef/200 <= 1` (for dense reinforcement `s <= 150`, `phi <= 10`: shell reinforcement); `psi_ec,N = 1/(1 + 2*eN/scr,N) <= 1`; `psi_M,N = 1.5 - 0.5*c? ` (compression, `psi_M,N = 2 - 2*z/hef >= 1` ... `[V]`); `hef` min 40 mm (post), 60 mm; `NRd,c = NRk,c/gamma_Mc`, `gamma_Mc = gamma_c*gamma_inst` (cast-in `1.5*1.0`); shallow member: `hef` reduced to `max(cmax/1.5, s/3)`; DE NA 4: `gamma_c = 1.5`.
**FA-04 — 7.2.1.4 — Pull-out failure.** `NRk,p = psi_c*A_h*8*fck`? precisely: `NRk,p = k2*Ah*fck` with `k2 = 7.5` (cracked), `10.5` (uncracked), `Ah = pi/4*(dh^2 - d^2)` bearing area of head; post-installed: from ETA `NRk,p = psi_c*NRk,p^0`, `psi_c = (fck/20)^0.5` `[V]`.
**FA-05 — 7.2.1.5 — Splitting failure.** `NRk,sp = N0Rk,sp*(Ac,N/A0c,N)*psi_s,N*psi_re,N*psi_ec,N*psi_h,sp`, `N0Rk,sp = min(NRk,c^0, NRk,p)`, `psi_h,sp = (h/hmin)^(2/3) <= (2*hef/hmin)^(2/3)`, `h_min` from ETA; characteristic edge/spacing `c_cr,sp`, `scr,sp = 2*ccr,sp`; exempt if cracked + reinforcement taking splitting (7.2.1.5(3)).
**FA-06 — 7.2.1.6 — Blow-out (side face).** `NRk,cb = N0Rk,cb*(Ac,Nb/A0c,Nb)*psi_s,Nb*psi_g,Nb*psi_ec,Nb*psi_h,Nb`; `N0Rk,cb = 8.7*c1*sqrt(Ah)*sqrt(fck)`; applies if `c1 <= 0.5*hef`; `Ah` bearing area.
**FA-07 — 7.2.2.2 — Steel failure, shear without lever arm.** `VRk,s = 0.5*As*fuk` (`0.6`?): without lever arm `VRk,s = 0.5*As*fuk`; with lever arm: `VRk,s = alpha_M*Mrk,s/lz`, `Mrk,s = 1.2*Wel*fuk`, `alpha_M = 1.0` (fixed) or `2.0` (clamped); `gamma_Ms,V = 1.0*fuk/fyk >= 1.25` for `fuk <= 800`, `fyk/fuk <= 0.8`, `1.5` otherwise; reduction `k7 = 0.8` for groups w/ grout.
**FA-08 — 7.2.2.3 — Pry-out (concrete) / Betonausbruch auf der lastabgewandten Seite.** `VRk,cp = k8*NRk,c`, `k8 = 1.0` (`hef < 60`) / `2.0` (`hef >= 60`); uses `NRk,c` of the group for tension free (cracked); `VRd,cp = VRk,cp/gamma_Mc`.
**FA-09 — 7.2.2.5 — Edge failure (concrete edge) shear.**
`VRk,c = V0Rk,c*(Ac,V/A0c,V)*psi_s,V*psi_h,V*psi_alpha,V*psi_ec,V*psi_re,V`;
`V0Rk,c = k9*d_nom^alpha*l_f^beta*sqrt(fck)*c1^1.5`; `alpha = 0.1*(lf/c1)^0.5`, `beta = 0.1*(d_nom/c1)^0.2`; `k9 = 1.7` (cracked), `2.4` (uncracked); `A0c,V = 4.5*c1^2`; `Ac,V` actual (limited by `1.5*c1`, `h`, `c2`); `psi_s,V = 0.7 + 0.3*c2/(1.5*c1) <= 1`; `psi_h,V = (1.5*c1/h)^0.5 >= 1`; `psi_alpha,V = sqrt(1/(cos^2 alpha_V + (sin alpha_V/2.5)^2))` ; `psi_ec,V = 1/(1 + 2*eV/(3*c1)) <= 1`; `psi_re,V = 1.0` (no edge reinf.), `1.2` (edge reinf. `phi>=12`), `1.4` (+ closely spaced stirrups `s<=100`); `c1 >= 10*hef` (...) small edges; `V0Rk,c` upper limits for far edges.
**FA-10 — 7.2.3 — Combined tension and shear / Kombination.** Steel: `(NEd/NRd,s)^2 + (VEd/VRd,s)^2 <= 1`; concrete: `(NEd/NRd)^(1.5) + (VEd/VRd)^(1.5) <= 1` for each failure mode pairing (`NRd = min(NRd,p, NRd,c, NRd,sp, NRd,cb)`, `VRd = min(VRd,c, VRd,cp)`); alternative simplified: `NEd/NRd + VEd/VRd <= 1.2`.
**FA-11 — 7.2.1.2 — Group steel failure and 7.2.1.3: group effects.** `Ac,N` for groups: union of cone footprint rectangles with overlap; `eN` eccentricity of group tension; per-anchor loads of tension anchors only for `psi_ec,N`.
**FA-12 — 5 — Fatigue of anchors (Annex 5) and seismic (C1/C2 categories).** Cyclic: `Delta NRk,s,0,n` per ETA; seismic reductions `alpha_gap`, `alpha_seis` (EN 1992-4 Annex C). Marked as data-driven extension.
**FA-13 — 7.2.2.4 — Concrete pry-out in groups.** see FA-08.
**FA-14 — 6.3 — Reinforcement to take up anchor loads (anchor reinforcement) / Rückhängebewehrung.** Supplementary reinforcement: tension `Ns = NEd,... ` anchorage in cone `ls >= ...`; design `As,re = NEd,re/fyd`; verifies via ST-04 (struts at 35° cone).

### 2.18 Model integrity (informational, not a normative check)

**DI-01 — none (model integrity; EN 1992-1-1 NA title page for the edition) — Duplicate ids and national annex edition.** Document-level rows of the inference root, not reachable from the checks crate: (a) every entity id of a collection must be unique; a duplicate is reported as an integrity row with a remedy (no EN 1992 clause supports it, so it is never part of the compliance verdict of a clause); (b) the DE values follow the edition `ndp.en1992.na-de-edition` of DIN EN 1992-1-1/NA, shown as an informational row `en1992.annex.na-de-edition` (utilisation 0, edition unverified `[V]`).

## 3. Cross-cutting derived quantities and cross-checks

- Derived section properties: `Ac, Ic_y, Ic_z, Iyz, Wc, centroid, principal axes, perimeter u, h0` for arbitrary polygons (outer + holes) via Green's theorem; transformed section (`alpha_e`) for State I/II/I-II.
- Quantities: axis distances `a_i`, effective depths per layer `d_i`, lever arms, `Asl` per face, steel ratio `rho_l`.
- Consistency: sum of rebar counted per layer vs. geometry (bars inside polygon with cover `>= cmin` D-04 and mutual distance B-09).
- Combination matrix across `gamma_F`, `psi` from EN 1990 (compliance kernel); every verification takes the `design situation` + `combination` it belongs to (ULS persistent, accidental, seismic, SLS characteristic/frequent/quasi-permanent, fire, fatigue).

## 4. Schema-first entity model

Language-neutral schema owned by `s.norm.en1992` (Rust `🦀️.rs`, TS, GraphQL, JSON schema, proto generated from the same leaves; the current `En1992Snapshot` is extended, not forked; legacy flat fields like `width/height/effective_depth/cover` are removed in the same change, no migration layer). Entity names below are the target taxonomy (domain-driven tree: `materials/`, `sections/`, `members/`, `slabs/`, `columns/`, `prestress/`, `anchors/`, `actions/`, `exposure/`).

### 4.1 Snapshot root (`En1992Snapshot`)

- `annex: AnnexChoice` (`De`, `En`, selectable; explicit, no default).
- `title`, `designWorkingLifeYears`, `deltaCDev`, `cementType` (existing), `structuralClassOverrides`.
- `situations: Vec<DesignSituation>` (ULS persistent/accidental/seismic/fire/fatigue, SLS characteristic/frequent/QP).
- `materials`: `concreteGrades`, `reinforcementGrades`, `prestressSteels`, `fireMaterialTables` (data-reference ids only).
- `sections: Vec<RcSection>`, `members: Vec<RcMember>`, `slabs: Vec<Slab>`, `columns: Vec<Column>`, `tendons: Vec<Tendon>`, `anchors: Vec<AnchorGroup>`, `exposures: Vec<ExposureSpec>`, `actions: Vec<ActionSet>`, `analysis: AnalysisSettings` (nominal method choice, second-order selection, redistribution `delta`, `phi_ef` source).

### 4.2 Section entities

- `PolygonLoop { id, points: Vec<Point2>, role: Outer|Hole }` — SI metres, local y/z axes, counter-clockwise outer, clockwise holes; validated simple, non-self-intersecting, hole inside outer.
- `RcSection { id, labelEn, labelDe, loops: Vec<PolygonLoop>, concreteGradeId, bars: Vec<Bar>, links: Vec<LinkSet>, cover: CoverSpec, exposureId, sectionTemplate: Option<Rect|T|L|I|Box|Circle|Ring> }`.
- `Bar { id, position: Point2, diameter, areaOverride?, gradeId, layerTag, role: Longitudinal|Corner|Skin, bondCondition: Good|Poor, preStrain? }` — individual bar objects (not only "count * diameter" layers); parametric `BarArray { start, end, count, diameter }` expands to bars (generator, not stored as truth: stored bars are explicit events).
- `LinkSet { id, diameter, spacing, legs, shape: Closed|Open|Spiral|Cross|Inclined, angleAlpha, zoneFrom, zoneTo, gradeId, hookType }` plus explicit `LinkPolyline` geometry (3D drawing).
- `CoverSpec { cNomTop, cNomBottom, cNomLeft, cNomRight, cMinDur (derived), deltaCDev, structuralClass (derived) }` — derived values are `Inference` outputs, not persisted state.

### 4.3 Members and structural elements

- `RcMember { id, label*, kind: Beam|Column|Wall|Slab|Footing|PileCap|Corbel|DeepBeam|Tank|Bridge, sectionRefs: Vec<SectionStation{x, sectionId}>, span, support: Vec<SupportCondition{x, kind: Pinned|Fixed|Free|Spring, stiffness}>, bucklingLengths{y,z}, loads: Vec<ActionSetRef>, analysisMode: Statics|Fem, stations: Vec<VerificationStation{x}> }` (member-length discretisation, stations auto-refined).
- `Slab { id, outline: PolygonLoop, thickness (or zones), supports: Vec<LineSupport|ColumnSupport|WallSupport>, openings: Vec<PolygonLoop>, reinforcement: Vec<MeshLayer{direction, spacing, diameter, coverTopBottom, zone polygon}>, systemType: OneWay|TwoWay|Flat|Ribbed|Waffle, punching: Vec<PunchingPoint> }`.
- `PunchingPoint { id, column: ColumnFootprint{shape, c1, c2 / D, position: Interior|Edge|Corner|WallEnd, location Point2}, capital?: ColumnHead{hH, lH}, openings (refs), VEd, MEdx, MEdy, betaMode: NaSimplified|Computed, studLayout: Option<StudLayout{type, perimeters: Vec<Perimeter{radius, count, area, angle}>, sr, st}> }`.
- `Column { id, section, height, endConditions: {kTop, kBottom, braced: bool}, imperfection mode, creepRatio, loadEnvelope }`.
- `Footing/PileCap` with piles & strut-and-tie model ref.

### 4.4 Prestress entities

- `Tendon { id, steelId, nStrands, areaPerStrand, ductDiameter, bond: Bonded|Unbonded|External|Pre-tensioned, profile: TendonProfile, stressing: {ends: Left|Right|Both, jackingStress, lockSlip, time}, frictionMu, wobbleK, relaxationClass, rho1000, release t0 }`.
- `TendonProfile { stations: Vec<{x, y, z, tangent?}>, interpolation: CubicSpline|Parabolic|PolylineWithArcs }` — control points editable; derived angular deviation `theta(x)`, eccentricity `e(x)`, curvature `kappa(x)`.
- `PrestressStage { id, time, tendons: Vec<id>, sequence }` for construction stages.

### 4.5 Anchors

- `AnchorGroup { id, anchors: Vec<AnchorPoint{position, d, dNom, hef, hOffset, steelGrade{fuk,fyk}, type: CastInHeaded|PostInstalledMechanical|Bonded|Undercut, ETAref}>, plate: BasePlate{polygon, thickness, stiffness: Rigid|Flexible, holeClearance}, concrete: {gradeId, cracked, thickness h, edges: Vec<EdgeDistance{c, direction}>, reinforcementPresence}, loads: Vec<AnchorAction{N, Vx, Vy, Mx, My, Mz, eccentricity}>}`.

### 4.6 Exposure and actions

- `ExposureSpec { id, faces: Vec<{faceRef, classes: Vec<ExposureClass>}>, relativeHumidity, ageAtLoading, temperatureHistory?, fireRating, tightnessClass, hydrostaticHead }`.
- `ActionSet { id, source, combination: {ULS_P, ULS_A, ULS_S, SLS_char, SLS_freq, SLS_qp, FIRE, FATIGUE}, internalForces: Vec<ForceStation{x, N, Vy, Vz, T, My, Mz}>` or derived from `loads` (line loads, point loads, UDL) via analysis; imposed deformations; fatigue spectrum `Vec<{DeltaSigma, n}>`.
- `Quantity` kinds reuse `QuantityKind` from the compliance kernel; per-field units fixed by schema and shown in the UI.

### 4.7 Events (CQRS / event sourcing, no CRUD)

Mutations are domain events, e.g. `SectionVertexMoved{sectionId, vertexIndex, to}`, `SectionVertexInserted`, `BarMoved{barId, to}`, `BarAdded`, `BarDiameterChanged`, `BarArrayGenerated`, `LinkSpacingChanged`, `TendonControlPointMoved`, `TendonStressingChanged`, `ColumnFootprintResized`, `StudRingAdded`, `AnchorMoved`, `ActionEnvelopeSet`, `ExposureAssigned`, `ConcreteGradeAssigned`. Each carries `commandId`, `author`, `lamport`; no CRDTs; concurrent move of the same bar resolves by last-writer on the aggregate (single-writer per aggregate with optimistic version). Scope tags: persisted shared (model), persisted local-only (viewport camera, layer toggles), ephemeral shared (cursors, in-progress drag preview), ephemeral local-only (hover).

## 5. Visualisation specification

All viewers are domain-neutral framework widgets configured by artifact-specific scene builders (domain-specific extension); scene description is schema-first (`SceneGraph`), rendered by the shared 3D surface (as in `fem`/`bim`/`block` plugins). Every view has EN/DE labels, colour-blind safe palette, keyboard-accessible controls, textual summary of what is shown (accessible alternative), progress + cancel on expensive meshes (S-06 surface, R-02 thermal solver).

**VZ-01 — 3D reinforced member with rebar cages.** Concrete extruded from `RcSection` loops along the member axis (transparent, wire edges optional, clipping plane/section cut slider); longitudinal bars as cylinders (true diameter, colour by grade; red tint if utilised > 1, heat gradient by `sigma_s/fyd` along the bar); stirrups from `LinkSet` as swept rounded polylines (mandrel radius, hooks 135°), per-zone spacing (dense near supports) with instancing for performance (GPU instanced meshes, `> 10k` bars LOD); cover shown as translucent offset shell with label `cnom`, red if `cnom,provided < cnom`; bar labels (e.g. `4Ø20`), anchorage lengths `lbd` drawn as dimension spans; lap zones highlighted; bend radii. Section cut plane: 2-D cross-section view with dimensioned geometry.
**VZ-02 — Tendon profile.** 3D spline through the member with colour = `P(x)` (friction/slip/time-dependent loss map); eccentricity `e(x)` and angular deviation; ducts as tubes; anchor plates and anchorage zone bursting vectors; deviation force arrows `q(x)`; 2-D side elevation with `Pm0(x)`, `Pm,t(x)` plot beneath, synchronised cursor.
**VZ-03 — 3D N-My-Mz interaction surface (S-06).** Triangulated surface in `(My, Mz, N)` space (N vertical), colour by `N`, translucent; constant-N slice curve highlighted at `NEd`; load point as sphere (green inside, red outside) with line to origin and to the radial hit point on the surface; hover shows strain plane (2-D mini section with neutral axis & strains), `eta_u`; toggles for uniaxial diagrams `My-N`, `Mz-N`, EN 5.39 ellipse-like curve overlay for comparison, multiple load points (envelope cloud from `ActionSet`), `NRd,0` and `NRd,t` markers. Rotatable, with projection snaps to `N-My`, `N-Mz`, `My-Mz` views.
**VZ-04 — 2-D stress/strain block diagrams.** Section elevation with strain plane (pivots A, B, C marked, `eps_c`, `eps_s1`, `eps_s2`, neutral axis `x`), stress block (parabola-rectangle or rectangle `lambda, eta`), forces `Fc, Fs1, Fs2`, lever arm `z`; for SLS the elastic triangular stress diagram (State II) with `sigma_c`, `sigma_s`, `x`; sliders drive neutral axis directly (drag the neutral axis).
**VZ-05 — Punching control perimeters in 3D.** Slab as plate with column; perimeters `u0`, `u1` (2d), `uout` as extruded translucent bands/poly-lines at mid-height, effective-depth marks; opening shadow lines; stud rings as small cylinders with correct heights, rings labelled; heat map of `vEd/vRd` along `u1` (when `beta` computed from moments); load path arrows; edge/corner reduced perimeters; slider for `d` and stud perimeters; `rout` boundary circle.
**VZ-06 — Crack pattern.** Elevation of the member with predicted crack spacing `sr,max` and crack widths `wk` (hatched cracks of width proportional to `wk`, colour by `wk/wk,lim`); bending vs. shear crack inclination `theta`; early-age restraint crack map for walls; effective tension zone `Ac,eff` shaded; crack-control chart (`wk` vs `sigma_s`) for tables 7.2N/7.3N.
**VZ-07 — Deflection shape.** Deformed shape of member exaggerated (scale slider), ghost of undeformed, curvature diagram beneath, `zeta` distribution (cracked/uncracked zones shaded), `w_max` marker, limits `l/250`, `l/500` lines; time slider t0..t_inf animating creep growth.
**VZ-08 — Fire isotherms.** 2-D section with temperature contours (20..1200 deg C, ISO 834 time slider), 500 deg C isotherm outline, reduced section (zone method), per-bar temperature labels and `ks`, time-vs-capacity plot (R-class timeline); 3D extruded stacking to show front/back heating only in the 3D viewer.
**VZ-09 — Strut-and-tie.** Truss in 2-D (struts hatched blue, ties red proportional to force), nodes classified CCC/CCT/CTT with utilisation colours; draggable nodes.
**VZ-10 — Shear/torsion truss.** Variable strut inclination `theta` as animated truss over the beam; `VRd,s`, `VRd,max` bars; shear diagram `VEd(x)` with `asw(x)` stepped required-versus-provided; torsion shear flow circle `tef`, `Ak` path.
**VZ-11 — Anchor group.** 3D anchors in concrete block with failure cones (concrete cone pyramid `scr,N=3 hef`, edge breakout wedge, splitting planes, blow-out), base plate with load arrows; utilisation colour per failure mode; transparent concrete; hovered anchor shows loads.
**VZ-12 — Utilisation heat map.** Every member coloured by max `eta_u` over all checks (legend green < 0.8 < amber <= 1.0 < red), clickable to list of failing clauses.
**VZ-13 — Material laws and time effects.** Stress-strain plots (parabola-rectangle, bilinear, steel branches with design/characteristic), creep `phi(t)` and shrinkage `epsilon_cs(t)` curves vs. time (log axis), tendon losses vs. time, S-N curves with `Delta sigma_equ` marker.
**VZ-14 — Interaction diagrams 2D.** `M-N` per axis, `M-kappa`, `N-kappa` curves; sectional hover.

## 6. Realtime editor interactions

Interaction rule: direct manipulation in the viewport, immediate local preview (ephemeral shared state), commit as event on pointer release; every committed event triggers incremental recomputation of affected catalogue entries only (dependency graph from catalogue inputs to outputs). Budget targets are measured on a mid-range desktop (4 cores).

| Interaction | Edit event | Recomputed | Preview budget | Commit budget |
|---|---|---|---|---|
| Drag a rebar | `BarMoved` | D-04 cover, B-09 spacing, S-03/S-04 (via S-01), SL-03, SL-06, VZ-01, VZ-03 point | < 16 ms (S-04 single point) | < 100 ms full set |
| Drag section polygon vertex | `SectionVertexMoved` | section properties, A-01 `h0`, all S/V/T, SL, bars-inside-polygon validation | < 16 ms properties, < 50 ms S-04 | < 150 ms |
| Insert/delete vertex, apply template (rect/T/L/box/circle) | `SectionVertexInserted` | same | | < 200 ms |
| Change bar diameter/count | `BarDiameterChanged` / `BarAdded` | same + DT-01, B-03 | < 16 ms | < 100 ms |
| Slider: concrete grade, steel grade | `ConcreteGradeAssigned` | everything material-dependent | < 50 ms | < 200 ms |
| Slider: NEd, MEdy, MEdz (move load point in 3D surface) | ephemeral `LoadPreview` | S-05, S-06 `eta_u` (surface cached; only slice+ray) | < 8 ms | persisted on release |
| Slider: stirrup spacing/diameter, `cot theta` | `LinkSpacingChanged` | V-03, T-02, DT-03, VZ-10 | < 8 ms | < 50 ms |
| Drag tendon control point | `TendonControlPointMoved` | PR-04 friction, slip, PR-05, S-10, V-04 `Vp`, SL-09, deflection SL-08, VZ-02 | < 33 ms (profile + friction) | < 300 ms |
| Tendon stressing sliders (jacking stress, lock slip) | `TendonStressingChanged` | PR-01..PR-07 | < 16 ms | < 100 ms |
| Drag column footprint / resize column / move opening | `ColumnFootprintResized` | P-01, P-02, P-03, P-04 | < 16 ms (perimeter geometry) | < 100 ms |
| Add stud ring / drag stud radius | `StudRingAdded` | P-04, VZ-05 | < 16 ms | < 100 ms |
| Drag strut-and-tie node | `StmNodeMoved` | ST-01..ST-04 | < 8 ms | < 50 ms |
| Drag anchor / group / edge distance | `AnchorMoved` | FA-01..FA-14 | < 16 ms | < 100 ms |
| Scrub fire time / R-class slider | ephemeral `FireTimePreview` | R-02 field (cached) | < 16 ms playback | solve < 2 s (cancellable, progressive) |
| Scrub creep age t / RH slider | ephemeral | A-01, A-02, SL-08, PR-05 | < 16 ms | < 100 ms |
| Change exposure class | `ExposureAssigned` | D-01..D-04 and wk,lim, LQ-01 | | < 50 ms |

Requirements: (1) inline utilisation chips next to every edited control (colour + text, not colour only); (2) Remedy suggestions with one-click apply as events: "increase `As` to x mm2", "choose Ø16 at 125 mm", "increase cover to X", "add stud ring", "increase concrete grade to C35/45", "increase depth to d >= y" (suggestions computed by inverse solvers S-03/V-03/SL-06/B-03/P-04, each a pure function reusing the catalogue); (3) undo/redo via event inverses; (4) keyboard-only operation for every drag (arrow keys nudge 1 mm / shift 10 mm / ctrl 0.1 mm), screen-reader announcements of changed utilisation; (5) snap rules (to cover offset, to grid spacing, to symmetric partner), locking, multi-select with group move; (6) numeric fields always available beside gestures (typed entry with units, localised decimals); (7) no JSON editor panel; the JSON/raw view is a read-only debug tab; (8) heavy operations (S-06 sampling, R-02 PDE, A-13 general method, non-linear FEM) run in workers with progress and cancellation and show the last converged result while recomputing; (9) connection loss: local-first, edits applied locally and queued; ephemeral shared state (other users' drag previews) is dropped on reconnect.

Layout: inputs window (tree of sections/members/slabs), viewport, results window (catalogue grouped by clause chapter with utilisation bars, sort by worst), details drawer (formula with substituted numbers, NDP source, EN/DE text, link to clause). Customisable panels (dock/undock, field-meta driven).

## 7. Test strategy (language-agnostic first)

- Language-agnostic vectors: JSON/CSV test vectors per catalogue entry stored in `🧪️tests/vectors/<ID>.json` with inputs, expected outputs, tolerance, source of expectation (hand calc or oracle). The same vectors run in Rust and TS implementations.
- Third-party oracles (Python, `🔮️oracles`): `scipy`/`numpy` (integration, root finding, convex hull `scipy.spatial.ConvexHull`), `shapely` (polygon clipping/area/centroid), `sympy` (closed-form check of closed forms), `pycba`/`anastruct` (statics for STM, deflection), and an independent strain-compatibility implementation (`concreteproperties` Python package if available for N-M surfaces and moment-curvature; cross-check the surface within `1%`), `ezdxf` not needed. Fire: `scipy` finite difference vs. our 2-D solver, tabulated EN 1992-1-2 Annex A profile samples as truth. Creep/shrinkage: independent Python transcription of Annex B formulas.
- Property tests: monotonicity (`MRd` increases with `As`), symmetry (square section `MRdy = MRdz`), objectivity (rotating the polygon by 90 degrees and swapping My/Mz yields the same `eta_u`), idempotence of events, determinism.
- Golden tests per worked example (section 9); fuzz tests on polygons (random convex/concave, with holes) comparing area/centroid/inertia to shapely.
- Mutation tests / negative cases: inputs out of range (`fck > 90`, `rho_l > 0.04`, `cot theta` out of limits) must yield `NotApplicable`/`Incomplete` rather than a number.
- NDP table tests: both annex choices run on the same inputs; difference vectors recorded (e.g. `CRd,c` 0.18 vs 0.15 gamma_c).

## 8. Output shape, remedies and localisation

Every `CheckResult` carries: `clause`, `titleEn/De`, `status`, `utilisation`, `demand: Quantity`, `resistance: Quantity`, `steps: Vec<FormulaStep{latexOrAst, symbolicEn/De, value, unit}>`, `ndp: Vec<NdpUse{name, ecValue, naValue, used}>`, `remedies: Vec<Remedy>`, `subject: SubjectRef` (section/member/station), `notes`. Report window prints this tree per member with chapter navigation. All texts use `LocalizedLabel::native(en, de)`, no default language.

## 9. Worked examples (hand-checked; all become golden tests)

All use DE-NA values unless noted. Rounding: results quoted to 3-4 significant digits, tests tolerance 1e-3 relative (closed form) as stated. All numbers here were verified by independent script calculation (Python, standard library only) while writing this spec; the script was not committed.

**E-01 — Rectangular beam bending (S-03), singly reinforced.**
Inputs: `b = 300 mm`, `h = 500 mm`, `d = 450 mm`, C30/37, B500B, `MEd = 250 kNm`, `NEd = 0`.
`fcd = 0.85*30/1.5 = 17.0 MPa`; `fyd = 500/1.15 = 434.78 MPa`.
`mu = 250e6/(300*450^2*17.0) = 0.2421`; `omega = 1 - sqrt(1 - 2*0.2421) = 0.2818`; `xi = x/d = omega/0.8 = 0.3522`, `x = 158.5 mm`; `z = d*(1 - 0.5*omega) = 450*(1 - 0.1409) = 386.6 mm`; `eps_s1 = 3.5*(1 - 0.3522)/0.3522 = 6.44 permille > 2.174` (steel yields).
`As1 = M/(z*fyd) = 250e6/(386.6*434.78) = 1487 mm2` (also `= omega*b*d*fcd/fyd = 1487`).
Expected: `As,req = 1487 mm2` (tolerance 0.2%); `xi = 0.352 <= 0.617` (no compression steel). Provide `5 phi 20 = 1571 mm2` -> `eta_u = 0.947`. `As,min = max(0.26*2.9/500*300*450, 0.0013*300*450) = max(203.6, 175.5) = 203.6 mm2` (`fctm = 2.896`; test uses table value `2.9`) ok.

**E-02 — Parabola-rectangle stress (M-05) and rectangular block equivalence (M-07).**
C30/37: `fcd = 17.0`, `epsc2 = 2.0 permille`, `n = 2`. At `eps = 1.0 permille`: `sigma_c = 17.0*(1 - (1 - 0.5)^2) = 12.75 MPa`. At `eps = 2.0` and `3.5`: `17.0`. At `eps = -0.1`: `0`. Block check: `Int_0^{3.5} sigma deps / 3.5 = 17*(1 - 2/(3*3.5)*... )`: mean stress over `0..epsu2` = `17*(1 - epsc2/(3*epsu2)) = 17*(1 - 2/10.5) = 13.76`; `lambda*eta = 0.8` yields `13.6` (Rectangular-block approximation within 1.2% of the exact parabola-rectangle resultant); resultant position `delta*x`: exact `0.416*x` vs `0.4*x` (lambda/2); tolerance 0.5%.

**E-03 — Shear with links, variable strut inclination (V-03), DE NA.**
Inputs: `bw = 300`, `d = 450`, `z = 0.9*d = 405 mm`, C30/37, B500, `VEd = 200 kN` (beyond `2d` from support), `sigma_cp = 0`, vertical links.
`nu2 = min(1.1 - 30/500, 1.0) = 1.0` (the raw value 1.04 is capped), `nu1 = 0.75*nu2 = 0.75` `[V]`.
`VRd,cc = 0.5*0.48*30^(1/3)*300*405 = 90.6 kN`; `cot theta,max = 1.2/(1 - 90.6/200) = 2.194` (`<= 3.0`, governs); choose `cot theta = 2.194`.
`VRd,max = 300*405*0.75*17/(2.194 + 0.4558) = 584.6 kN` -> `eta = 200/584.6 = 0.342` ok.
`Asw/s = 200e3/(405*434.78*2.194) = 0.5177 mm2/mm = 517.7 mm2/m` (both legs). `phi 8`, 2 legs (101 mm2): `s <= 195 mm` -> `s = 190 mm`. `Asw/s,min = 0.16*2.9/500*300*1000 = 278.4 mm2/m` ok; `smax = 0.75*d = 337 mm` ok.
EC recommended for comparison: `cot theta = 2.5`: `Asw/s = 454.3 mm2/m`; `nu1 = 0.6*(1 - 30/250) = 0.528`, `VRd,max = 300*405*0.528*17/(2.5 + 0.4) = 376 kN`.

**E-04 — Shear without links (V-02), DE NA.**
Inputs: slab strip/beam `bw = 300`, `d = 450`, `As,l = 1257 mm2 (4 phi 20)`, C30/37, `sigma_cp = 0`.
`k = 1 + sqrt(200/450) = 1.667`; `rho_l = 1257/(300*450) = 0.00931`; `CRd,c = 0.15/1.5 = 0.10`; `vRd,c = 0.10*1.667*(100*0.00931*30)^(1/3) = 0.10*1.667*3.034 = 0.5057 MPa`; `vmin = (0.0525/1.5)*1.667^1.5*sqrt(30) = 0.035*2.152*5.477 = 0.4125 MPa` -> governing `0.5057`. `VRd,c = 0.5057*300*450 = 68.3 kN`.
EC recommended: `CRd,c = 0.12` (0.18/1.5): `0.6068`, `VRd,c = 81.9 kN`. (Note: DE value lower than EC for `d = 450`.) With `VEd = 80 kN`: DE `eta_u = 1.17` Fail -> remedy: links (E-03 procedure).

**E-05 — Punching, interior column (P-01..P-04).**
Inputs: slab `h = 250`, `d = 210`, C30/37, B500, interior square column `400 x 400`, `rho_ly = rho_lz = 0.010`, `VEd = 700 kN`, `beta = 1.10` (DE), `sigma_cp = 0`.
`u1 = 4*400 + 2*pi*2*210 = 1600 + 2638.9 = 4238.9 mm`. `vEd = 1.10*700e3/(4238.9*210) = 0.8650 MPa`.
`k = 1 + sqrt(200/210) = 1.976`; `CRd,c = 0.18/1.5 = 0.12` [V]; `vRd,c = 0.12*1.976*(100*0.01*30)^(1/3) = 0.12*1.976*3.107 = 0.7367 MPa`; `vmin = 0.035*1.976^1.5*sqrt(30) = 0.5324` -> `vRd,c = 0.7367`. `eta_u = 0.865/0.7367 = 1.174` -> **Fail** without punching reinforcement (`VRd,c = 596 kN`).
With stud reinforcement: `fywd,ef = 250 + 0.25*210 = 302.5 MPa`; `sr = 0.75*d = 157.5 mm`; required `Asw` per perimeter from `vEd = vRd,cs = 0.75*vRd,c + 1.5*(d/sr)*Asw*fywd,ef/(u1*d)`: `0.865 = 0.5525 + 1.5*1.3333*Asw*302.5/(4238.9*210)` -> `Asw = 0.3125*890177/(2*302.5) = 459.7 mm2` per perimeter (`~ 12 studs phi 8 = 603 mm2` ok). Outer perimeter: `uout = beta*VEd/(vRd,c*d) = 1.1*700e3/(0.7367*210) = 4977 mm`; `rout = (4977 - 1600)/(2*pi) = 537 mm` from the column face; the outermost stud ring must lie within `1.5d = 315 mm` of `rout` (ring radii `0.5d = 105`, `105 + 157.5 = 262.5`, `420` mm ... as many as needed, here at least three rings). Maximum check: `vRd,max = 1.4*vRd,c = 1.031 MPa` `[V]` -> `0.865 <= 1.031` ok (the perimeter to which the NA factor applies must be confirmed).

**E-06 — Torsion, rectangular solid section (T-01, T-02, T-04).**
Inputs: `b = 300`, `h = 500`, C30/37, B500, `TEd = 20 kNm`, `VEd = 150 kN`, `cot theta = 1.0`, `nu = 0.75` (as E-03).
`A = 150000 mm2`, `u = 1600 mm`, `tef = 93.75 mm`; `Ak = (300 - 93.75)*(500 - 93.75) = 83789 mm2`; `uk = 2*(206.25 + 406.25) = 1225 mm`.
`TRd,max = 2*0.75*1.0*17*83789*93.75*sin45*cos45 = 100.2 kNm`.
Stirrups: `Asw/s = 20e6/(2*83789*434.78*1.0) = 0.2745 mm2/mm = 274.5 mm2/m` (one leg of a closed link). Longitudinal: `sum Asl = 20e6*1225*1.0/(2*83789*434.78) = 336.3 mm2`, distributed along `uk`.
Interaction: `VRd,max(cot theta = 1) = 300*405*0.75*17/2 = 774.6 kN`; `TEd/TRd,max + VEd/VRd,max = 0.1996 + 0.1936 = 0.393 <= 1` ok.

**E-07 — Anchorage length (B-01..B-03).**
Inputs: `phi = 20`, B500, C30/37 (`fctk,0.05 = 2.0 MPa`), good bond, `sigma_sd = fyd = 434.78`, straight bar, `cd = 45 mm` (side cover governs), no transverse reinf. effects (`alpha3 = 1.0`).
`fctd = 0.85*2.0/1.5 = 1.133 MPa`; `fbd = 2.25*1.0*1.0*1.133 = 2.55 MPa`; `lb,rqd = (20/4)*(434.78/2.55) = 852.9 mm` (434.78/2.55 = 170.5 -> 852.5; test value 853 +/- 1); `alpha2 = 1 - 0.15*(45 - 20)/20 = 0.8125`; `lbd = 0.8125*852.9 = 693 mm`; `lb,min = max(0.3*852.9, 200, 100) = 255.9 mm` -> `lbd = 693 mm`. Lap (tension, `rho1 = 50%`): `alpha6 = (50/25)^0.5 = 1.414 <= 1.5`: `l0 = 1.0*0.8125*1.0*1.0*1.414*852.9 = 980 mm`, `l0,min = max(0.3*1.414*852.9 = 361.8, 15*20 = 300, 200) = 361.8` -> `l0 = 980 mm`.

**E-08 — Strut-and-tie, two-pile cap (ST-01..ST-04).**
Inputs: pile centre distance `s = 1.2 m`, square column `c = 0.4 m` centred, `NEd = 2000 kN`, `z = 0.65 m`, B500.
Node offset from pile centre: `s/2 - c/4 = 0.6 - 0.1 = 0.5 m`; strut angle `tan theta = z/0.5 = 1.30 -> theta = 52.4°`; tie force `Ftd = (NEd/2)*0.5/z = 1000*0.5/0.65 = 769.2 kN`; strut force `Fc = 1000/sin(52.43°) = 1261.6 kN`; `As,tie = 769.2e3/434.78 = 1769 mm2` (e.g. `6 phi 20 = 1885 mm2`).
Strut capacity (EC recommended, node CCT): `sigma_Rd,max = k2*nu'*fcd = 0.85*(1 - 30/250)*17 = 0.85*0.88*17 = 12.7 MPa` (DE `k2 = 0.75`? [V]) giving required strut/node width `1261.6e3/(12.7*b)`.

**E-09 — Crack width calculation (SL-06) and span/depth (SL-07).**
Inputs: `b = 300`, `h = 500`, `d = 450`, `As = 1257 mm2` (4 phi 20), `c = 40 mm`, C30/37 (`Ecm = 33 GPa`, `fctm = 2.9`), `Es = 200 GPa`, `Ms,QP = 150 kNm`, long-term `kt = 0.4`, `alpha_e = 6.06` (short-term modulus for the example).
`x`: `300*x^2/2 = 6.06*1257*(450 - x)` -> `x = 127.9 mm`; `I_II = 300*127.9^3/3 + 6.06*1257*(322.1)^2 = 2.093e8 + 7.903e8 = 9.996e8 mm4`; `sigma_s = 6.06*150e6*322.1/9.996e8 = 292.9 MPa`.
`hc,eff = min(2.5*(500-450) = 125, (500 - 127.9)/3 = 124.0, 250) = 124.0 mm`; `rho_p,eff = 1257/(300*124.0) = 0.03378`.
`epsilon_sm - epsilon_cm = (292.9 - 0.4*2.9/0.03378*(1 + 6.06*0.03378))/200000 = (292.9 - 34.34*1.2047)/200000 = (292.9 - 41.37)/200000 = 1.258e-3` vs `0.6*292.9/200000 = 0.879e-3` -> `1.258e-3`.
`sr,max = 3.4*40 + 0.8*0.5*0.425*20/0.03378 = 136 + 100.6 = 236.6 mm`; `wk = 236.6*1.258e-3 = 0.298 mm`. DE limit XC2..XC4 `wk,lim = 0.3 mm` -> `eta_u = 0.99` Pass (marginal); XC1 `0.4` -> `0.74`.
Span/depth (E-09b): `fck = 30`, `rho0 = sqrt(30)*1e-3 = 0.005477`; `rho = 0.0033`: `l/d <= 1.0*[11 + 1.5*5.477*0.005477/0.0033 + 3.2*5.477*(0.005477/0.0033 - 1)^1.5] = 11 + 13.63 + 17.53*0.5338... = 34.03` (K = 1.0, simply supported; compute: `0.005477/0.0033 = 1.6597`; `(0.6597)^1.5 = 0.5358`; `3.2*5.477 = 17.526`; `9.39`; total `11 + 13.64 + 9.39 = 34.03`). For `rho = 0.010 > rho0` (`rho' = 0`): `11 + 1.5*5.477*0.005477/0.010 + (1/12)*5.477*sqrt(0)= 11 + 4.5 = 15.5`; tests use `rho' = 0`: `15.5`.

**E-10 — Creep and shrinkage (Annex B, A-01, A-02).**
Inputs: C30/37 (`fcm = 38 MPa`), `RH = 50%`, section `300 x 500` (`h0 = 2*150000/1600 = 187.5 mm`), `t0 = 28 d`, cement N, loading to infinity.
`alpha1 = (35/38)^0.7 = 0.9441`; `alpha2 = (35/38)^0.2 = 0.9837`; `phi_RH = [1 + (1 - 0.5)/(0.1*187.5^(1/3))*0.9441]*0.9837 = [1 + 0.5/0.5724*0.9441]*0.9837 = 1.7949`; `beta(fcm) = 16.8/sqrt(38) = 2.7253`; `beta(t0) = 1/(0.1 + 28^0.2) = 0.4884`; `phi0 = 1.7949*2.7253*0.4884 = 2.389`; `phi(inf, 28) = 2.39`.
Shrinkage: `beta_RH = 1.55*(1 - 0.125) = 1.356`; `epsilon_cd,0 = 0.85*(220 + 440)*exp(-0.12*3.8)*1e-6*1.356 = 0.85*660*0.6338*1.356e-6 = 482.2e-6`; `kh(187.5) = 1.0 - (87.5/100)*0.15 = 0.869`; `epsilon_cd,inf = 418.9e-6`; `epsilon_ca,inf = 2.5*(30 - 10)*1e-6 = 50e-6`; `epsilon_cs,inf = 468.9e-6` (0.469 permille). Tolerance 0.2%.

**E-11 — Column N-M interaction, biaxial check (S-02, S-04, S-05, S-06) and slender column (A-09, A-10).**
Inputs: square column `400 x 400`, C30/37, B500B, 4 phi 25 at corners (`As = 4*490.9 = 1963.5 mm2`), bar centres `50 mm` from each face, `NEd = 1500 kN`.
S-02: `NRd,0 = 400^2*17.0 + 1963.5*434.78 = 2720 + 853.7 = 3574 kN`; pure tension `NRd,t = -853.7 kN`.
S-04 (uniaxial about one axis, numerically integrated, parabola-rectangle): `MRd(N = 1500) = 235.6 kNm` at `x = 248.8 mm` (bar at compression face yields: `eps_s2 = 2.8 permille > 2.174`, tension bar `eps_s1 = -1.42 permille` -> `sigma_s = -284 MPa`); intermediate points: `x = 100: N = 451 kN, M = 200.3 kNm`; `x = 150: 809 kN, 239.2 kNm`; `x = 200: 1084 kN, 254.1 kNm`; `x = 250: 1511 kN, 234.9 kNm`; `x = 300: 1947 kN, 202.9 kNm`; `x = 400: 2691 kN, 123.6 kNm` (tolerance 0.5%; oracle: independent numpy integration).
S-05: `NEd/NRd = 1500/3574 = 0.4197` -> `a = 1.0 + 0.5*(0.4197 - 0.1)/0.6 = 1.266`; `MEdy = 150 kNm`, `MEdz = 120 kNm`: `(150/235.6)^1.266 + (120/235.6)^1.266 = 0.5645 + 0.4255 = 0.990 <= 1.0` -> **Pass** (tight, `eta_u = 0.99`); S-06 exact surface must give `eta_u` (constant-N slice, direction `atan2(120,150) = 38.7°`) within `+/- 3%` of the 5.39 result (exact is slightly less conservative or comparable for a square, doubly symmetric column; test tolerance reported, not asserted equal).
Slender column (A-09/A-10) same section: `l0 = 4.0 m`, `i = 400/sqrt(12) = 115.5 mm`, `lambda = 34.6`; `n = 1500e3/(160000*17) = 0.5515`; `omega = 1963.5*434.78/(160000*17) = 0.3139`; `A = 0.7`, `B = sqrt(1 + 2*0.3139) = 1.276`, `C = 0.7` (`rm = 1.0`); `lambda_lim = 20*0.7*1.276*0.7/sqrt(0.5515) = 16.84` -> `lambda > lambda_lim`: second-order required. `epsilon_yd = 2.174e-3`; `d = h/2 + is = 200 + 150 = 350`; `1/r0 = 2.174e-3/(0.45*350) = 1.380e-5 /mm`; `nu = 1.3139`, `Kr = (1.3139 - 0.5515)/(1.3139 - 0.4) = 0.834`; `Kphi = 1.0` (`phi_ef = 0`); `1/r = 1.151e-5 /mm`; `e2 = 1.151e-5*4000^2/pi^2 = 18.66 mm`. With `M0Ed = 60 kNm (e0 = 40 mm)`, `ei = l0/400 = 10 mm`: `MEd = 1500*(40 + 10 + 18.66)/1000 = 103.0 kNm`; check vs `MRd = 235.6` -> `eta_u = 0.437` Pass.

**E-12 — Fire, tabulated beam (R-01).**
Inputs: simply supported beam `300 x 500`, exposed on 3 sides, main bars phi 20, links phi 8, nominal cover to links `20 mm`: axis distance `a = 20 + 8 + 20/2 = 38 mm`. Table 5.5 column 2 (simply supported, `[V]`): R30 `80/15`, R60 `120/25`, R90 `150/35`, R120 `200/45`, R180 `240/60`, R240 `280/75` (`b_min/a`, mm). Result: `b = 300 >= 150` and `a = 38 >= 35` -> R90 satisfied; R120 needs `a >= 45` (`38 < 45`) -> fail with deficit `7 mm`; `R_achieved = 90`. Interpolation test: required `a` for R105 = linear between 35 and 45 = 40 mm.
500 degC isotherm cross-check (R-03): same section, R90; the 2-D thermal solver is compared to Annex A profile samples with tolerance 5 degC; `ks(theta_s)` per bar by linear interpolation.

**E-13 — Prestress losses (PR-04, PR-05).**
Friction: `mu = 0.19`, `k = 0.005 /m`, tendon length `x = 20 m`, total angle `theta = 0.15 rad`: `P(x)/P0 = exp(-0.19*(0.15 + 0.005*20)) = exp(-0.0475) = 0.9536` (loss 4.64%). Relaxation class 2 (`rho1000 = 2.5%`), `mu = 0.70`, `t = 500000 h`: `Delta sigma_pr/sigma_pi = 0.66*2.5*exp(9.1*0.70)*(500)^(0.75*0.30)*1e-5 = 0.66*2.5*584.3*4.05*1e-5 = 0.0390` (3.90%). Tolerance 0.5%.

**E-14 — Fatigue of reinforcing steel (F-03, F-04).** Straight bar, `N* = 1e6`, `Delta sigma_s,equ = 120 MPa`: resistance `162.5/1.15 = 141.3 MPa`, `eta_u = 0.849` Pass. Damage accumulation (`k1 = 5`): stress range `200 MPa` for `n = 1e5` cycles: `N = 1e6*(141.3/200)^5 = 1.76e5`; `D = 1e5/1.76e5 = 0.568` (<= 1).

**E-15 — Anchorage concrete cone, single cast-in headed anchor (FA-03) and edge variant.**
Inputs: `hef = 100 mm`, C30/37, cracked, cast-in headed (`k1 = 8.9`), far from edges: `N0Rk,c = 8.9*sqrt(30)*100^1.5 = 8.9*5.477*1000 = 48.75 kN`; `NRd,c = 48.75/1.5 = 32.5 kN`; `NEd = 25 kN` -> `eta_u = 0.77` Pass.
Edge variant: `c1 = 100 mm` (`ccr,N = 150`): `Ac,N = (100 + 150)*(300) = 75000`, `A0c,N = 300^2 = 90000`, ratio `0.8333`; `psi_s,N = 0.7 + 0.3*100/150 = 0.9`; `NRk,c = 48.75*0.8333*0.9 = 36.56 kN`; `NRd,c = 24.37 kN`; `eta_u = 25/24.37 = 1.026` -> **Fail** (edge case: borderline failing by 2.6%); remedy: `hef` 110 or move anchor.

**E-16 — Partially loaded area (PA-01).** Bearing plate `Ac0 = 200 x 200 = 40000 mm2`, distribution area `Ac1 = 600 x 600 = 360000 mm2` (similar rectangle), C30/37, `fcd = 17.0`: `FRdu = 40000*17.0*sqrt(360000/40000) = 680000*3 = 2040 kN`; cap `3.0*fcd*Ac0 = 2040 kN` equal -> `FRdu = 2040 kN`; with `Ac1 = 800 x 800`: `sqrt(16) = 4 -> 2720 kN` capped to `2040 kN` (cap governs). `FEd = 1800 kN` -> `eta_u = 0.882`.

**E-17 — Interface shear (V-05).** Rough interface `c = 0.40`, `mu = 0.7`, C30/37 (`fctd = 1.133`), `sigma_n = 0` , `rho = 0.0020` (`As/Ai`), `alpha = 90°`: `vRdi = 0.40*1.133 + 0 + 0.0020*434.78*(0.7*1 + 0) = 0.4533 + 0.6087 = 1.062 MPa`; cap `0.5*nu*fcd = 0.5*0.528*17 = 4.49` ok; `vEdi = 1.0*100e3/(0.9*450*300) = 0.823 MPa` -> `eta_u = 0.775` Pass (EC recommended `nu = 0.6*(1 - fck/250) = 0.528`).

**E-18 — Slab minimum shear check at support (V-07).** `av = 0.6 m`, `d = 0.25 m`: `av/(2d) = 1.2 > 1` -> no reduction (`beta = 1`); with `av = 0.3 m`: `beta = 0.6` (`>= 0.25` ok) -> `VEd,red = 0.6*VEd`.

(These tests are the minimum; each catalogue entry additionally gets at least one oracle-compared vector, and E-01..E-18 also serve as UI demo documents under `📚️examples`.)

## 10. Data and implementation tree (guidance)

`en1992/` -> `materials/` (concrete table, steel, creep/shrinkage), `durability/` (exposure, cover tables), `sections/` (polygon, integrator `S-01`), `resistance/` (bending, shear, torsion, punching, strut-tie, partial area), `detailing/` (anchorage, laps, ch. 9), `serviceability/` (stress, crack, deflection), `prestress/`, `fatigue/`, `fire/` (tables + thermal solver + isotherm/zone), `bridges/`, `liquid/`, `fastenings/`, `analysis/` (second order, redistribution), `ndp/` (EC and DE tables as data), `scene/` (visualisation builders), `editor/` (commands/events), each with language-agnostic vectors `tests/vectors`. Clause data (tables 4.x, 5.x fire, 7.2N/7.3N, F-03, anchors ETA fields) are JSON data files with source clause ids so that both implementations load the same numbers; no table values duplicated in code.

## 11. Verification flags to resolve before implementation

Items marked `[V]` are from memory and must be confirmed against DIN EN 1992-1-1/NA:2013-04 (+A1), -1-2/NA, -2/NA, -3/NA, DIN EN 1992-4 (including the A1/ A2 corrigenda): primarily DE `nu1`/`nu2` cap and `cot theta` limit formula, `vRd,max` for punching with reinforcement, `beta` for edge/corner, `CRd,c` for punching, node factors `k1..k3`, `vmin` interpolation 600..800 mm, table 4.3/4.4 DE cover matrices, `k_c` and `fct,eff` rules in 7.3.2, DE crack width limits, exposure min. concrete classes, tying forces, EN 1992-4 `psi_M,N` and `k1` for cast-in headed anchors, and fire tables 5.x. The worked examples that depend on `[V]` items are marked and must be recomputed once the NDP table is frozen (the arithmetic in the examples is otherwise verified).
