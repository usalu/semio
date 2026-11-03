# 🖨️ Printed-Standards Checklist (VERIFY-LIST, wave 2)

Read-only audit. Nothing in the repo was changed. Sources: NDP registry `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` (the path `📇️registry` under `📕️norm` holds contract code, not NDP data), `📌️important/ndp-verification-worklist.md` (stale: 2842 / 2653 / 189), spec-trace `🧭️spec-trace/🦀️.rs` files, and the wave-2 reports in this folder. Working files: `🗑️generated/printed-check/` (`ndp.tsv`, `usage2.txt`, scripts).

## 1. Counts

Registry (live JSON): **3754 rows, 258 verified, 3496 unverified.** Every row is cited by source code (exact id match), and every unverified row is cited. Worklist figures (2842 / 2653 / 189) are stale and should be regenerated.

| Code | Total | Verified | Unverified |
|---|---:|---:|---:|
| EN 1990 | 270 | 0 | 270 |
| EN 1991 | 608 | 0 | 608 |
| EN 1992 | 212 | 0 | 212 |
| EN 1993 | 893 | 3 | 890 |
| EN 1994 | 103 | 0 | 103 |
| EN 1995 | 743 | 255 | 488 |
| EN 1996 | 351 | 0 | 351 |
| EN 1997 | 127 | 0 | 127 |
| EN 1998 | 292 | 0 | 292 |
| EN 1999 | 155 | 0 | 155 |
| **Sum** | **3754** | **258** | **3496** |

Per annex / national document (`source.document`; "DIN ... /NA" = DE national annex, "EN ..." = the base document):

| Document | Total | Verified | Note |
|---|---:|---:|---|
| DIN EN 1990/NA | 270 | 0 | all Annex A1/A2 factors |
| DIN EN 1991/NA + 1-1..1-7/NA, 2, 3, 4/NA | 479 + 129 | 0 | 1991 rows spread over the NA parts |
| DIN EN 1992-1-1/NA | 147 | 0 | |
| DIN EN 1992/NA, -1-2/NA, -2/NA, -3/NA, -4/NA | 53 + 3 + 4 + 3 + 2 | 0 | |
| DIN EN 1993-1-1/NA | 802 (+3 verified) | 3 | 3 verified rows: alpha_T, E, nu |
| DIN EN 1993/NA and -1-2..-1-9, -2/NA | 54 + 18 | 0 | |
| EN 1993-1-3/-1-4/-1-5/-1-6/-1-8/-2 (base) | 16 | 0 | |
| DIN EN 1994 family (NA) + EN 1994-1-1 / EN 1994-2 | 101 + 2 | 0 | |
| DIN EN 1995-1-1/NA, -1-2/NA, -2/NA, 1995/NA | 288 + 2 + 1 + 20 | 0 | |
| EN 14080:2013 Table 5 | 168 (153 unverified) | 15 | verified glulam rows disagree with their notes; see section 2, item 7 |
| EN 338:2016 Table 1 (144) / Table 3 (96) | 240 | 240 | all verified; 46 rows (EN 338 and EN 14080 together) disagree with their own notes; see section 2, item 7 |
| EN 14374 / ETA; EN 16351 | 24 | 0 | |
| DIN EN 1996 family (NA) and EN 1996-1-1 / DIN EN 1996-1-1/NA | 262+2+3+7+20+57 | 0 | |
| DIN 1054:2021-04; DIN EN 1997/NA; EN 1997-2 / NA | 119 + 7 + 1 | 0 | |
| DIN EN 1997-1/NA:2023-11 (labelled as EN 1998 rows) | 36 | 0 | **defect**: 36 EN 1998 rows cite the EN 1997 NA document |
| DIN EN 1998-1/NA:2023-11 | 256 | 0 | |
| DIN EN 1999-1-1/NA; EN 1999-1-2 | 135 + 20 | 0 | |

Value coverage (both numeric = EN and DE columns compared):
- 1301 rows carry **no numeric value at all** (560 are EN 1993-1-10 Table 2.1 cells, "Cell not transcribed"; 37 EN 1998-5 Annex E rows and similar text-only rows).
- 49 rows carry numeric values (EN 1993-1-10 Table 2.1) that the artifact **wrote from memory**, not transcribed from print.
- 49 rows have an EN value but no DE value.
- 164 rows have different EN and DE values (these need the DE column checked first).

Usage: source-file mentions of each id, counted from `🗑️generated/printed-check/usage3.txt` (registry JSON and generated `🗃️table.rs` excluded). Counts quoted in section 2 are from that file. All 3754 rows (3496 unverified) are referenced by source code outside the registry JSON and generated table; the "uses" figure is an indicator of reach, not a verdict.

## 2. Prioritised checklist

Order = safety impact first (unconservative if wrong; governs common designs), then use count. "Uses" = source mentions of the id. "Now" = value currently encoded in the registry (EN `recommended` / DE `naDe`); where the registry and the worklist conflict, both are shown. Direction notes are a reminder for the reviewer, not a verified conclusion.

### P1, safety-critical and high-use

**1. EN 1992-1-1 3.1.6 and 2.4.2.4 (alpha_cc, alpha_ct, gamma_c, gamma_s), DIN EN 1992-1-1/NA**
- `ndp.en1992.m-01.alpha-cc-persistent` (uses 43): EN 1.0 / DE 0.85. Conflict E92-02 in worklist.
- `ndp.en1992.m-01.alpha-ct` (17): EN 1.0 / DE 0.85.
- `ndp.en1992.m-01.alpha-cc-accidental` (7): EN 1.0 / DE 0.85.
- `ndp.en1992.m-01.gamma-c-persistent` (73): 1.5 / 1.5.
- `ndp.en1992.m-01.gamma-c-accidental` (6): EN 1.2 / DE 1.3. Conflict E92-05.
- `ndp.en1992.gamma-s` (47): 1.15 / 1.15. `ndp.en1992.gamma-s-fat` (11): 1.15. `ndp.en1992.gamma-s-acc` (9): 1.0.
- `ndp.en1992.gamma-c-fat` (fatigue, 1.5), `ndp.en1992.gamma-inst` (13): 1.0.
- Impact: alpha_cc sets the concrete design strength on every RC member. A DE value of 0.85 against an EN-style 1.0 changes resistance by about 18 %. Direction: unconservative if the wrong value is applied. Governs nearly all RC designs.

**2. EN 1992-1-1 6.2.2 (shear: C_Rd,c, k1, v_min) and 6.2.3 (cot theta, nu1), DIN EN 1992-1-1/NA**
- `ndp.en1992.crd-c-shear` (7): 0.18 / 0.15. `ndp.en1992.k1-shear` (6): 0.15 / 0.12. Conflict E92-24.
- `ndp.en1992.vmin-shear` (13): 0.035 / 0.0525 (d-dependent DE form). `ndp.en1992.vmin-shear-d800` (12): 0.035 / 0.0375. Conflict E92-25: repo code (H92:184/217) uses 0.035 for both annexes.
- `ndp.en1992.cot-theta-max` (7): 2.5 / 3.0. `ndp.en1992.cot-theta-a` / `-b`: DE form 1.2 + 1.4 sigma_cp/f_cd. Conflict E92-26: repo uses a clamp with no V_Rd,cc term.
- `ndp.en1992.nu1-factor` (7): 1.0 / 0.75. Conflict E92-82: repo uses nu1 = 0.6(1 - f_ck/250).
- `ndp.en1992.beta-interior` (8): 1.15 / 1.1. (A lower beta is less conservative.)
- Impact: shear without stirrups and shear with stirrups. Unconservative if the DE values are not applied correctly. Governs common beams and slabs.

**3. EN 1992-1-1 6.4.4, 6.4.5 (punching), DIN EN 1992-1-1/NA**
- `ndp.en1992.crd-c-punch` (9): 0.18 / 0.18. Conflict E92-39: worklist says the repo gives 0.15/gamma_c for DE (H92:214). Also `ndp.en1992.p-03.2` (6.4.4(1)): EN 0.1 / DE 0.12, worklist conflicts E92-39 and E92-41.
- `ndp.en1992.k1-punch` (8): 0.1. `ndp.en1992.punch-k-out` (9): 1.5. `ndp.en1992.punch-vrdmax-factor` (10): EN 0.0 / DE 1.4.
- `ndp.en1992.punch-small-u0.*` (limit/slope/offset/floor): reduction disabled for EN; DE 4 / 0.1 / 0.6 / 0.15 (wave2 en1992-v11). Unverified.
- Punching with links, `ndp.en1992.p-04.1`: formula conflict E92-43 (0.75 v_Rd,c + 1.5(d/s_r)...).
- Impact: punching on flat slabs and columns is brittle and governs many designs. Unconservative if C_Rd,c or v_min is too high.

**4. EN 1993-1-1 6.1 (partial factors gamma_M0, gamma_M1, gamma_M2), DIN EN 1993-1-1/NA**
- `ndp.en1993.gamma-m1` (uses 13): EN 1.0 / DE 1.1. `ndp.en1993.gamma-m0` (7): 1.0 / 1.0. gamma_M2 row: check it exists and is 1.25 (DE).
- Impact: governs every steel resistance check. Using the EN value (1.0) for DE gives an unconservative result of about 10 %.

**5. EN 1993-1-10 Table 2.1 (brittle fracture t_max), and EN 1993-1-1 Table 3.1 (fy / fu cells)**
- 560 cells are **not transcribed**. Checks report `CellNotTranscribed` (a gap, not a pass or fail).
- 49 cells are **written from memory**: `ndp.en1993.1-10.t21.s235-j0.r75.t-10` (65 mm) and the other S235 JR/J0/J2, S355 JR/J0/J2/K2 cells at sigma_Ed = 0.75 f_y(t). Source: `🔍️audits/wave2/en1993-v18.md`.
- `ndp.en1993.v-010.1` (Table 3.1, t <= 40 mm, S235/275/355/440 fy/fu = 235/360, 275/430, 355/510, 440/550): from memory, `en1993-vids.md` section 5.
- Impact: t_max governs fracture toughness on every welded member. Table 3.1 fy/fu govern every steel resistance. Unconservative if the fy/fu or t_max values are wrong. Governs common designs.

**6. EN 1990 Annex A1/A2 partial factors and psi (DIN EN 1990/NA Tables A1.2(A/B/C), A2.4, A1.1, A2.1-A2.3)**
- ids: `ndp.en1990.gG_sup.str.A1` (uses 10): 1.35; `ndp.en1990.gQ.str.A1` (7): 1.5; `ndp.en1990.gG_sup.equ.A1` 1.1; `ndp.en1990.gQ.equ.A1` 1.5; `ndp.en1990.gG_inf.*`; `ndp.en1990.gQ.road.*` (A2.4); `ndp.en1990.gM.*` (concrete 1.5, steel 1.0, timber 1.3).
- psi: `ndp.en1990.psi.office.0/1/2` 0.7/0.5/0.3; `psi.congregation.0/1/2` 0.7/0.7/0.6; `psi.construction_load.*`; `psi.footbridge_*.*`; `psi.rail_hslm.*` (1.0/1.0/0.0); `psi.rail_sw2.*` (0.0/1.0 ...).
- Policy placeholders, not normative: `ndp.en1990.a1.override.{gamma,k_fi,psi,xi}.allowed` (1.0 / 0.0 / 1.0 / 1.0). The `specRef` says these are "POLICY PLACEHOLDER (unverified)". Replace from the NA wording.
- Group factors: `ndp.en1990.a2.group.*` (`en1990-v7.md`) are 1.0 placeholders for the eight own components; `gr2.road_traffic` and `gr5.road_traffic` have no value.
- Impact: partial factors and psi govern all combinations. An unconservative gamma (for example gQ 1.35 instead of 1.5) gives unconservative results on every design. Top priority.

**7. Timber strength tables (EN 338:2016 Tables 1/3, EN 14080:2013 Table 5), status "verified", but values conflict with their own notes**
- 46 verified rows (value field differs from the note text). Examples: `ndp.en1995.strength.gl28h.ft0k` value 22.3, note 19.5. `gl24h.ft0k` value 19.2, note 16.5. `gl32h.ft0k` 25.6 / 22.5. `c16.ft0k` 8.5 / 10. `c30.fc0k` 24.0 / 23. `d30.fc90k` 5.3 / 8. `d70.rhok` 800 / 900. `c20.rhomean` 400 / 390.
- Some verified glulam ft0k values look like formula results (0.8 f_mk), not the EN 14080 Table 5 value. Check every row against the printed tables before anything else; the status is not trustworthy for these rows.
- Impact: all timber checks. Unconservative when strengths are too high. Governs all timber designs.

**8. EN 1991-1-4 wind (pressure coefficients) and EN 1991-1-3 snow (NA-DE), DIN EN 1991-1-4/NA, DIN EN 1991-1-3/NA**
- Pitched roof `ndp.en1991.1-4.cpe.pitched0.*` (126 rows): Tables 7.3a / 7.4a. Flat roof (60 rows), hipped roofs (18 rows, Table 7.5), external pressure on vertical walls (30 rows, linear in h/d).
- NA-DE ground snow s_k (20 rows): formula factor max(a + b((H + offset)/scale)^2, min).
- Impact: roof suction and snow govern roof designs. Unconservative when a coefficient is too small in absolute value. Governs common buildings. Each row is "value as used by the artifact", not transcribed.

**9. EN 1991-2 traffic and rail (LM1, LM71, SW, footbridge), DIN EN 1991-2/NA**
- `ndp.en1991.2.lm1-*`, `ndp.en1991.2.rail.*`, `ndp.en1991.2.footbridge-*`, `en1991.2.braking` / centrifugal. From memory per `en1991-v9.md` (Table 4.3, 4.4.1 constants, alpha = 1.0 interim) and `en1990-v4.md` (HSLM-A1..A10 N, D, d, P recalled).
- Impact: bridge verification. Unconservative if the LM1 or braking values are too low. Governs bridge designs.

**10. EN 1992-4 fastenings (anchors), DIN EN 1992-4/NA and DIN EN 1992-1-1/NA**
- Cone factor k (equation 7.2.1.4, the "k1" family): `ndp.en1992.cone-k-post-cracked` 7.7 (6 uses), `ndp.en1992.cone-k-cast-cracked` 8.9 (10 uses), `ndp.en1992.cone-k-post-uncracked` 11.0 (6 uses), `ndp.en1992.cone-k-cast-uncracked` 12.7 (9 uses). The repo uses a single law of 7.5 cracked / 11.0 uncracked (conflict E92-80).
- Pull-out k2: `ndp.en1992.pullout-k2-cracked` 7.5, `-uncracked` 10.5.
- `ndp.en1992.fa-03.1` (1.5) and `ndp.en1992.fa-04.1` (7.2.1.4): **no value**.
- Partial factors `ndp.en1992.gamma-ms-re` 1.15, `ndp.en1992.gamma-ms-fat` 1.35.
- Impact: anchor concrete cone breakout governs anchor capacity. Unconservative if the k value is too high for cast-in or post-installed. Governs anchor designs.

**11. EN 1994-1-1 7.4.3 vs EN 1994-2 6.8.5.4 (tension stiffening), DIN EN 1994-1-1/NA and EN 1994-2**
- `ndp.en1994.2.fatigue-tension-stiffening` (uses 11): 0.2 (EN 1994-2 6.8.5.4).
- `ndp.en1994.1-1.tension-stiffening` (7.4.3): 0.4. Repo keeps 0.4 (`en1994-v11.md`, "OPEN, factor"). Neither is verified.
- `ndp.en1994.2.rebar-fatigue-range` (uses 14): no value.
- Impact: crack width and fatigue stress range. Using 0.4 where the print says 0.2 is conservative for the stress range (larger increment). Check the direction before fixing.

**12. EN 1996-1-1 Tables 8.1/8.2 (chases and recesses) and masonry tables (K, f_vo, Annex E alpha1), DIN EN 1996-1-1/NA**
- `ndp.en1996.chase.*` (35 rows, `en1996-v12.md`): "recalled from the standard". Repo has no table text (spec M-70 only). Values such as `chase.band.b1..b4.t-upper` 0.115 / 0.175 / 0.225 / 0.3 and `chase.v.b1..b5.depth/width`.
- `ndp.en1996.k.*` (48 rows: f_k = K f_b^alpha f_m^beta, `as used by the artifact`), `ndp.en1996.fvk0.*` (28, initial shear strength), `ndp.en1996.annex-e.*` (36, Annex E alpha1 bending moments).
- `ndp.en1996.m95.diaphragm-ratio`: "10 %" (`en1996-v6.md`, recalled).
- E = K_E f_k and G = 0.4 E (`en1998-v10.md`, "EN 1996-1-1 3.7.3 from memory").
- Impact: chases and masonry strengths govern masonry designs. Unconservative if wall strength is too high or chases too deep. Governs common masonry buildings.

**13. EN 1998-1 seismic (Table 3.2/3.3 spectral parameters, gamma_I, cracked stiffness 4.3.1(6), gamma_Rd, top-storey exemption 4.4.2.3)**
- `ndp.en1998.t32.{a..e}.{s,tb,tc,td}` (20 rows) and `ndp.en1998.t33.{a..d}.{s,tb,tc,td}` (20 rows): Table 3.2 / 3.3 values (from memory, `en1998-v8` and `coordinator.md` line 601: "Table 3.1 thresholds hard-coded from memory").
- `ndp.en1998.en1.gamma-i.{i..iv}` (0.8 / 1.0 / 1.2 / 1.4): check the DE NA 4.2.5 values. Bridges use different importance factors (BR-01 spec: 0.85 / 1.0 / 1.3, registry only).
- `ndp.en1998.en1.cracked-stiffness-factor` = 0.5 (4.3.1(6)) (`en1998-v12.md`, "quoted from memory"). Applies to every period, Rayleigh and mode.
- gamma_Rd = 1.3 (4.4.2.3 and 4.3.1); top-floor exemption (`en1998-v11.md`, "from memory").
- Impact: seismic base shear and period. Unconservative if the spectral parameters or the cracked factor are wrong. Governs every seismic check.

**14. EN 1998-5 foundations (Annex E water, Annex B liquefaction, sliding)**
- `ndp.en1998.en5.annex-e.permeability-limit` = 5e-4 m/s (`en1998-part5-v2.md`, "recalled from Annex E, NOT in the spec"). Also `en1998-oracle-walls.md`.
- `ndp.en1998.en5.liquefaction.annex-b.csr-factor` (uses 10) = 0.65 (en1998-v2 "unverified"). `ndp.en1998.en5.liquefaction.min-safety-factor` (7) = F_L 1.25 (EN 1998-5 4.1.4).
- `ndp.en1998.en5.pile.stiffness-contrast` (6) = 6.0 (5.4.2, Nikolaou coefficients from memory per `en1998-v2`).
- `ndp.en1998.en5.sliding.gamma-rd` = 1.0 (5.3). `ndp.en1998.en5.vertical-force.{threshold 0.6, ratio-high 0.5, ratio-low 0.33}` (4.1.3.3).
- Passive Annex E sign convention: recalled (`en1998-part5-v2.md`).
- Impact: liquefaction and water thrust on retaining walls. Unconservative if the permeability limit or csr factor is wrong. Governs foundations near water.

**15. EN 1997-1 / DIN 1054:2021-04 (geotechnics, M2 factors and design approaches)**
- `ndp.en1997.m2.phi` (uses 5) = 1.25, `ndp.en1997.m2.c` (5) = 1.25, Table A.4 (DIN 1054:2021-04).
- Design approaches `ndp.en1997.r1..r4.*` (34 rows), `ndp.en1997.xi.*` (34 rows), `ndp.en1997.upl.*` (24 rows), `ndp.en1997.de.*` (24 rows).
- `ndp.en1997.7.6.4.concrete-modulus` (E_cm C30/37 33 GPa, `en1997-v7.md`, "from memory of EN 1992-1-1 Table 3.1").
- `ndp.en1997.2.cpt-nkt` (10 uses) = 15 (EN 1997-2 Annex D).
- Impact: foundations and retaining walls. Unconservative if a factor is too high. Governs common foundation designs.

**16. EN 1993-1-8 hollow joints and EN 1993-1-7 plates (recalled constants)**
- `ndp.en1993.v-440.1..4` (Table 7.8 / 7.11, `en1993.md` section 2 "from memory, cited"), `v-123.1`, `v-113b.1`, `v-420.2`, `v-026.2` (Eq. 5.2 validity), `v-258.1`, `v-376.1`, `v-806.1` (factor 3.0, engineering value), `v-881.1` (14/12 mm), `v-881.2` (6 mm), `v-881.3` (`en1993-v4.md`).
- `ndp.en1993.1-1.phi0` (8 uses) = 0.005 and the 5.3.2(4) exemption (`en1993-v21.md`, "recalled values, not read in print").
- Impact: moderate; joints and imperfections. Unconservative if the imperfection or the joint limit is too lenient.

### P2, medium

**17. EN 1992-1-2 fire, EN 1994-1-2 fire (ks, kc, Table 3.2a) and EN 1995-1-2 fire**
- `ndp.en1994.1-2.ks.{hot,cold}.<theta>` (38 rows): `en1994-v9.md` "EN 1992-1-2 Table 3.2a as recalled". `ndp.en1994.1-2.alpha-unexposed` = 9 W/m²K (`en1994-v8.md`). `ndp.en1994.1-2.rho-concrete` = 2300 (`en1994-v9.md`).
- `ndp.en1992.fire-zone-exponent` = 1.0; the R-04 exponent 1.3 is from memory (`en1992-v2.md` / `en1992-v3.md`). `ndp.en1992.ld-flange-factor` 0.8 and `ld-flange-ratio` 3.0 (Table 7.4N note 2, from memory `[V]`).
- `ndp.en1995.fire.reduced-{bending,compression,tension}-divisor` 200 / 125 / 330, `fire.reduced-time-limit` 20 min, `fire.clt-fall-off-factor` 2.0, `fire.clt-fall-off-protect` 25 mm (`en1995-v3.md`, from memory).
- Impact: fire checks. Unconservative if a reduction is too lenient.

**18. EN 1995-1-1 (timber, k_mod, k_def, gamma_M, floor vibration, creep, bridges)**
- `ndp.en1995.kmod.*` (165 rows), `ndp.en1995.kdef.*` (48 rows), `ndp.en1995.gamma-m` (10 rows), `ndp.en1995.floor.f1-limit` (uses 6): EN 8.0 Hz / DE 6.0 Hz. `ndp.en1995.creep.time-constant-years` = 5.0 (model assumption).
- Not sampled in this audit beyond the list above. Check k_mod and k_def in EN 1995-1-1 Table 3.1 / 3.2 and the DIN NA.
- `en1995-v3.md` section 3 (17 constants from memory): `ndp.en1995.bridge.crowd-force` 280 N, `bridge.crowd-factor` 10.8, `bridge.crowd-psi-{start,full-from,full-to,end}` 0.5 / 1.0 / 1.7 / 2.1 Hz, `annex-c.eta.{glued,nailed,connectors,bolted}` 1 / 4 / 3 / 6.

**19. EN 1991-1-7 and 1991-4 (accidental, silo, bulk solids), DIN EN 1991-1-7/NA and -4/NA**
- `ndp.en1991.e17.impact` (9), `e17.robust` (9), `e17.returns`; `ndp.en1991.e4.silo` (13, EN 1991-4 2.2 Table 2.1 aac thresholds, boundary convention flagged in `adjudicator.md`); `ndp.en1991.e12.annex-a` (10).
- `ndp.en1991.e6-01.1` (stage return period 2 / 2 / 5 / 50 years, `collection-adder.md`).

**20. EN 1996 chase / walls (other NA), DIN EN 1996-1-1/NA** — covered in P1 item 12, also `ndp.en1996.m07.gamma` (11), `m81.phis` (9), `m10.rho2` (9), `m40.flexure` (8), `m14.beta` (8), `m05.fbok` (8), `m04.fxk` (8), `m31.yield-line` (7), `m12.einit` (7), `m06.creep` (7). All `None` or from the artifact.

**21. EN 1999-1-1 (aluminium), DIN EN 1999-1-1/NA, EN 1999-1-3**
- `ndp.en1999.gamma-mf` (uses 5): EN 1.0 / DE 1.35 (EN 1999-1-3 Table 2.1).
- Web stiffener constants (5 rows, `en1999-stiffeners.md`, `en1999-v9.md`): strip factor 15 eps t_w, inertia 1.5 / 0.75, aspect sqrt 2, buckling 0.75. Recalled, not verified.
- Transverse loads 6.7.5 (5 constants) and effective width 0.78 (`en1999-v5.md`). Check constants 15 eps t_w, I_st >= 1.5 h_w t^3 / a^2 (`en1999-v8.md`).
- Impact: moderate. Unconservative if a web buckling constant is too lenient.

### P3, lower use or no value in the registry

- EN 1993-1-10 Table 2.1 remaining cells (see P1 item 5): transcribe all 560 empty cells and verify the 49 remembered cells.
- EN 1993-1-4 stainless (39 reduction factors, 24 grade properties, Ramberg-Osgood exponent `ndp.en1993.v-300.2`, `v-010`, `v-601.1` specific heat). Unverified, "flagged unverified in the code docstring".
- EN 1992-3 silos / EN 1992-2 bridges (`ndp.en1992.lq-*`, `br-*`): no values.
- EN 1992 `e-12.1` (Table 5.5 fire beams): repo `beam_ss` and `beam_cont` columns conflict (E92-86).
- Registry defect, 36 rows: EN 1998 values cite the DE EN 1997-1 NA document (`DIN EN 1997-1/NA:2023-11`). Fix the `source.document` field.
- Registry defect: the 2 rows `ndp.en1992.na-de-edition` (2013) and `ndp.en1990.*` `None`-valued entries.
- The 84 ids cited in code that are not in the registry (for example `ndp.en1990.beta1`, `ndp.en1990.gG_inf`, `ndp.en1990.gM`) are probably dynamic prefixes. Check that they resolve.

## 3. Values explicitly recalled from memory tonight (wave-2 reports)

The grep found 72 wave-2 reports containing "recalled", "from memory" or "printed". The list below picks the ones that name values or ids. Report paths are under `🔍️audits/wave2/` (full path prefix: `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🔍️audits/wave2/`).

| Value(s) recalled | Registry ids | Report |
|---|---|---|
| All normative readings of EN 1990, "from memory and unverified" | `ndp.en1990.*` | `en1990-v9.md` |
| Override allowances (placeholders), alpha_cr limit 10, Fig. 6.10 coefficients, damping 0.01, HSLM-A1..A10 (N, D, d, P) | `ndp.en1990.a1.override.*`, `ndp.en1990.*`, `ndp.en1990.a2.rail.hslm.*` | `en1990-v4.md` |
| Group factors 1.0 for eight own components | `ndp.en1990.a2.group.*` | `en1990-v7.md` |
| Table 4.3 numbers, 4.4.1 constants, alpha = 1.0 (interim), EN 1990 psi rows | `ndp.en1991.*` | `en1991-v9.md` |
| Stage return period 2 / 2 / 5 / 50 years | `ndp.en1991.e6-01.1` | `collection-adder.md` |
| R-04 exponent 1.3 (EN 1992-1-2 B.6), Table 7.4N flange factor 0.8 / 3 | `ndp.en1992.fire-zone-exponent`, `ndp.en1992.ld-flange-*` | `en1992-v2.md`, `en1992-v3.md` |
| Punching small u0: EN 0 / 0 / 1 / 0; DE 4 / 0.1 / 0.6 / 0.15 | `ndp.en1992.punch-small-u0.*` | `en1992-v11.md` |
| C_Rd,c 0.18/gamma_c (punching, DE NA 6.4.4(1)); interior column u0/d < 4 | `ndp.en1992.crd-c-punch` | `en1992-v12.md` |
| A.1 / A.2.2.1 constants; k = 3.91 check; I_sl definition | EN 1993 Annex A constants | `en1993-v5.md` |
| 49 Table 2.1 cells (S235 JR/J0/J2, S355 JR/J0/J2/K2 at sigma = 0.75 f_y(t)) | `ndp.en1993.1-10.t21.*` (r75 cells) | `en1993-v18.md` |
| phi_0 and 5.3.2(4) exemption | `ndp.en1993.1-1.phi0` | `en1993-v21.md` |
| v-258.1, v-376.1, v-806.1 (3.0), v-881.1 (14/12 mm), v-881.2 (6 mm), v-881.3 | `ndp.en1993.v-258.1`, `v-376.1`, `v-806.1`, `v-881.*` | `en1993-v4.md` |
| Table 3.1 fy/fu (235/360, 275/430, 355/510, 440/550 MPa), v-011.1, v-013.1 (1 %), v-024.1, v-050.1 (3 %) | `ndp.en1993.v-010.1`, `v-011.1`, `v-013.1`, `v-024.1`, `v-050.1` | `en1993-vids.md` |
| Hollow joint Table 7.8 / 7.11 (v-440.1..4), v-123.1, v-113b.1, v-420.2, v-026.2 | `ndp.en1993.v-440.*`, etc. | `en1993.md` (section 2), `en1993-v2.md` (section 2) |
| Hollow-joint formulas (N_1,Rd = 8.9 k_n ...) | EN 1993 hollow-joint clause | `coordinator.md` line 164 |
| 9 W/m2K (unexposed side), solid-slab idealisation | `ndp.en1994.1-2.alpha-unexposed` | `en1994-v8.md` |
| ks,theta hot / cold tables (EN 1992-1-2 Table 3.2a), 2300 kg/m3, HEB root radius 27 mm | `ndp.en1994.1-2.ks.*`, `ndp.en1994.1-2.rho-concrete` | `en1994-v9.md`, `crate-budget-f.md` |
| 0.2 vs 0.4 (EN 1994-2 6.8.5.4 vs EN 1994-1-1 7.4.3), open | `ndp.en1994.2.fatigue-tension-stiffening`, `ndp.en1994.1-1.tension-stiffening` | `en1994-v11.md` |
| 17 constants: fire divisors 200/125/330, reduced time limit 20 min, CLT fall-off 2.0 / 25 mm, crowd force 280 N, crowd factor 10.8, crowd psi 0.5 / 1.0 / 1.7 / 2.1 Hz, eta 1 / 4 / 3 / 6 | `ndp.en1995.fire.*`, `ndp.en1995.bridge.*`, `ndp.en1995.annex-c.eta.*` | `en1995-v3.md` (section 3) |
| Table 8.1 / 8.2 chase rows (35) and 0.2 t, bed-joint, conformance values | `ndp.en1996.chase.*` | `en1996-v12.md`, `en1996-v5.md` |
| 10 % diaphragm limit (1.10) | `ndp.en1996.m95.diaphragm-ratio` | `en1996-v6.md` |
| E_cm C30/37 = 33 GPa | `ndp.en1997.7.6.4.concrete-modulus` | `en1997-v7.md` |
| 34 rows: Annex A coefficients, Annex B 0.65, Table F.1, pile contrast 6, Nikolaou, Youd fines | `ndp.en1998.en5.*`, `ndp.en1998.ref.*` | `en1998-v2.md` |
| G/E = 0.4 (EN 1996-1-1 3.7.3); default 0.25; 0.2 for concrete (EN 1992-1-1 Table 3.1 note) | `wall_plan::SHEAR_TO_YOUNG_MODULUS_RATIO`, `ndp.en1998.*` | `en1998-v10.md`, `en1998-v11.md` |
| Top-floor exemption 4.4.2.3, gamma_Rd 1.3 | `ndp.en1998.*` | `en1998-v11.md`, `en1998-v12.md` |
| Cracked stiffness 0.5 (4.3.1(6)) | `ndp.en1998.en1.cracked-stiffness-factor` | `en1998-v12.md`, `SUMMARY.md` line 108 |
| EN 1998 Table 3.1 thresholds hard-coded from memory | `ndp.en1998.t3*` | `coordinator.md` line 601, `SUMMARY.md` line 185 |
| 5 new constants, strength range C50 (mean 58 MPa) | `ndp.en1998.*` | `en1998-v16.md` |
| Permeability limit 5e-4 m/s (not in spec); Annex E sign convention; water formulation | `ndp.en1998.en5.annex-e.permeability-limit` | `en1998-part5-v2.md`, `en1998-oracle-walls.md` |
| Five web stiffener constants (15 eps t_w; 1.5, 0.75, sqrt 2, 0.75) | `ndp.en1999.web-stiffener.*` | `en1999-stiffeners.md`, `en1999-v9.md` |
| Five transverse-load constants (6.7.5), effective width 0.78 | `ndp.en1999.transverse-load.*` | `en1999-v5.md` |
| Check constants of A-28 (EN 1999-1-1 6.7.6-6.7.7) | `ndp.en1999.*` | `en1999-v8.md` |
| HEB root radius r = 0.027 m (EN 10365 as recalled), IPE300 r 15 mm | catalogue rows | `crate-budget-f.md` |
| EN 1992 Fig. 5.6N values | `ndp.en1992.*` | `coordinator.md` line 201 |
| Bracing rows DE column, AAC thresholds, footbridge band limits | `ndp.en1991.*`, `ndp.en1995.bracing.*` | `adjudicator.md` |

Also flagged in the same sense but not from a recall statement: the EN 1993-1-10 V-900 Table 2.1 cells and the EN 1998-5 Annex E water values (no print in repo, `status-matrix-5.md` item 10: "Printed-table gaps").

## 4. Recommended order for the human

1. Fix the timber rows (item 7). The 46 verified rows contradict their own notes.
2. Print and check the partial factors and psi (item 6), then alpha_cc, gamma_c, gamma_M1 (items 1, 4). These touch every design.
3. Transcribe EN 1993-1-10 Table 2.1 (560 empty cells) and check the 49 remembered cells; check EN 1993-1-10 Table 3.1 fy / fu (item 5).
4. Check EN 1992-1-1 shear, punching and fatigue values (items 2, 3).
5. Check EN 1991-1-4 roof and wall coefficients and the NA snow formula (item 8), then EN 1991-2 bridge loads (item 9).
6. Check EN 1998-1 Table 3.2 / 3.3, gamma_I, cracked factor (item 13), then EN 1998-5 values (item 14).
7. Check EN 1996 chases (Tables 8.1 / 8.2) and masonry tables (item 12), then EN 1992-4 cone k (item 10) and EN 1994-2 0.2 vs 0.4 (item 11).
8. Repair the registry defect (36 EN 1998 rows with the EN 1997 NA document) and regenerate the worklist (it still shows 2842 entries).

## 5. Wave 3 additions

Added 2026-11-03 by W3-PRINT-LIST. Read-only: no registry row, code or spec was changed. Baseline for the registry diff: the staged index copy of `🔣️.json` (3754 rows, the wave-2 state). Current file: 3830 rows, 76 added, 85 changed (46 verified to unverified, 39 unverified with new notes or labels). Counts per item are in `🔍️audits/wave3/print-list.md`.

Impact key:
- **safety-critical (S)**: a wrong value can turn a fail into a pass, or can silently skip a check.
- **conservative (C)**: the artifact currently errs on the safe side. Confirm the direction before changing anything.
- **display only (D)**: information rows, labels, examples or metadata. No pass or fail depends on them.

Order: impact tier first (S, then C, then D), then standard and part. Within each tier, the registry rows come before the report and request items. Every row names the printed clause or table to read. "Value as used by the artifact" means the number was hard-coded in code before wave 3 and moved into a row without a print source (`🔍️audits/wave3/literals-fix.md`).

### 5.1 Safety-critical

#### EN 1990

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S01 | S | Annex C, C.7 (secondary action, alpha_e) | `ndp.en1990.form.alpha_e_secondary_factor` (new) | 0.4 / 0.4 | C.7 rule not transcribed; value recalled. Used by the E90.29 two-action check. | `🔍️audits/wave3/en1990.md` (Not done) |
| W3-S01b | S | Annex D, Table D2 (design fractile factor k_d,n, V_X known row) | none (code table `k_d_n` in `reliability_math`, nodes 4.36 ... 3.04, linear in 1/n between nodes) | n = 3 ... infinity, as coded | Row recalled from the standard, not printed in the repo. A wrong node changes the direct D.7.2 design value x_d = eta_d m (1 - k_d,n V) of the informative check `en1990.annex-d.<id>.direct`. A scipy closed form bounds the coded row (checked by hand, within 0.07). | `🔍️audits/wave3/en1990.md` (round 2, Table D2) |
| W3-S01c | S | Annex D, Table D2 (design fractile factor k_d,n, V_X unknown row) | none (`k_d_n(n, false)` returns `None`) | not transcribed | No source at hand, so no value exists; the artifact reports no direct value for a series with unknown V_X and never guesses one. Print needed to transcribe the V-unknown row before the direct check can cover it. | `🔍️audits/wave3/en1990.md` (round 3) |

#### EN 1991

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S02 | S | Part 1-2, Annex C, C.2 (local fire, Hasemi curve and L_h / z' constants) | `ndp.en1991.1-2.local-flux-intercept` 136300; `-flux-plateau` 100000; `-flux-slope` 121000; `-flux-tail` 15000; `-flux-tail-exponent` -3.7; `-lh-coefficient` 2.9; `-lh-exponent` 0.33; `-q-denominator` 1110000; `-zprime-factor` 2.4 (all new, both annexes) | as listed | Recalled constants, not from the spec or print (`en1991.md`, Unverified). Local fire demand depends on every one of them. | `en1991.md`; literals |
| W3-S03 | S | Part 2, 4.6.2 (fatigue load model FLM1) | `ndp.en1991.2.fatigue-flm1-q-factor` 0.7; `ndp.en1991.2.fatigue-flm1-udl-factor` 0.3 (new) | 0.7 / 0.3 | FLM1 data recalled. FLM2, FLM4, FLM5 and the crane fatigue table S0..S9 are not done. | `en1991.md` (Not done) |
| W3-S04 | S | Part 2, 4.6.4 (fatigue load model FLM3 axle and spacing) | `ndp.en1991.2.fatigue-flm3-axle` 120000; `-spacing-1` 1.2; `-spacing-2` 6.0; `-spacing-3` 1.2; `-track` 2.0; `-wheel` 0.4 (new) | as listed (units as stored) | FLM3 data recalled, not printed. Fatigue damage scales with every value. | `en1991.md`; literals |
| W3-S05 | S | Part 2, 4.7.3.1 (collision forces, deck level and soffit) | `ndp.en1991.2.collision-deck-fx` 1000000; `-deck-fy` 500000; `ndp.en1991.2.collision-soffit-fx` 500000 (new) | as listed (N) | Value as used by the artifact. No print in repo. | literals |
| W3-S06 | S | Part 2, footbridge vertical comfort trigger, EN 1990 A2.4.3.2(1) | `ndp.en1991.2.footbridge-band-vertical-max` 5.0 / 5.0 Hz (existing row) | 5.0 Hz | Wave 3 (`correctness-fix.md` section 4) made the comfort check trigger depend on this value. A wrong value silently skips the comfort verification below it. Locator is EN 1990 but the row sits under EN 1991-2. | `correctness-fix.md` section 4 |
| W3-S07 | S | Part 2, footbridge Q_flk (floor load in the comfort check) | none (code constant, 10 kN floor) | 10 kN | No source. Clause text gives 10 % of the vertical load, or an alternative 60 % of the service vehicle. Print needed to pick the rule. | `correctness.md` item 17c |

#### EN 1992

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S08 | S | 6.2.3(2) (DE shear strength factor v_Rd,cc) | `ndp.en1992.vrdcc-k` (new) | EN none / DE 0.48 | DE-only value, value as used by the artifact. Repo already uses a clamp with no V_Rd,cc term (wave 2, E92-26). | literals-fix row 8 |
| W3-S09 | S | 6.2.3(103) (DE nu_2 strength reduction) | `ndp.en1992.nu2-offset` 1.1 and `ndp.en1992.nu2-fck-divisor` 500 (new) | EN none / DE 1.1 and 500 | Locator "6.2.3(103)" is not an EN 1992-1-1 paragraph number. Find the paragraph in the DE text before using the value. | literals-fix row 9 |
| W3-S10 | S | 5.2(5) (initial imperfection theta_0 = 1/200) | `ndp.en1992.imperfection-theta0-divisor` (new) | 200 / 200 | Value as used by the artifact. Drives every second-order check. | literals-fix row 17 |
| W3-S11 | S | 7.4.1(4) quasi-permanent deflection span divisor | `ndp.en1992.defl-limit-qp` (new) | 250 / 250 | Serviceability verdict. Value as used by the artifact. | literals-fix rows 1-2 |
| W3-S12 | S | 7.4.1(5) deflection span divisor after construction | `ndp.en1992.defl-limit-after` (new) | 500 / 500 | Serviceability verdict. Value as used by the artifact. | literals-fix rows 1-2 |
| W3-S13 | S | Table 4.1 (EN 1992-4, anchor partial factor gamma_Ms, values other than "other") | `ndp.en1992.anchor-gamma-ms.n.min` 1.4; `.n.slope` 1.2; `.v.min` 1.25; `.v.slope` 1.0; `.ratio-max` 0.8; `.f-uk-max` 800 (new) | as listed | No print. Row document is recorded as DIN EN 1992-1-1/NA, but the table belongs to EN 1992-4 (DIN EN 1992-4/NA). Fix the source first. | literals-fix row 4; `literals.md` |

#### EN 1993

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S14 | S | 5.3.3 (bracing deflection and imperfection span divisors) | `ndp.en1993.bracing.deflection-limit-divisor` 500; `ndp.en1993.bracing.imperfection-divisor` 500 (new) | 500 / 500 | Value as used by the artifact. Imperfection drives storey-bracing checks. | literals-fix rows 10-11 |
| W3-S15 | S | 6.4.1 (built-up member bow imperfection e_0 = L/500) | `ndp.en1993.built-up.bow-divisor` (new) | 500 / 500 | Locator 6.4.1 to be checked against the printed paragraph. | literals-fix row 11 |
| W3-S16 | S | EN 1993-1-10 Table 2.1 (toughness, r75 cells, S355 K2, S235 J0, S235 J2) | `ndp.en1993.1-10.t21.s235-j2.r75.*` (t10 125, t0 105, t-10 90, t-20 75, t-30 65, t-40 55, t-50 45); `ndp.en1993.1-10.t21.s235-j0.r75.*` (t10 90, t0 75, t-10 65, t-20 55, t-30 45, t-40 40, t-50 35) | as listed | Cells come from memory (`en1993-v18.md`). Reviewer recollection differs in both directions: S235 J2 135 115 100 85 75 65 55 and S235 J0 90 75 60 50 40 35 30. Check each cell against the print. | `correctness.md` item 6 and "Caveats" |
| W3E-S01 | S | EN 1993-1-1 5.3.3 (imperfections for analysis of bracing systems, Eq. 5.12 q_d = ΣN_Ed 8 (e_0 + δ_q)/L², δ_q the in-plane deflection of the bracing system) | `ndp.en1993.bracing.deflection-limit-divisor` | 500 / 500 | Two candidates, neither read from the print: L/500 (this row, the same divisor as the bow e_0 = α_m L/500) and L/2500 (recalled by the ORACLES-2 reviewer for the stiffness criterion of δ_q). The repo spec has no text. Do not change the value before the print is read; the check output and the registry note are marked UNVERIFIED (`storey-checks`), the oracle row carries `unverifiedBasis`. If the print says L/2500 the registry row changes (Rust and oracle read it). | `wave3/oracles-2.md` finding 3; `wave3/en1993-corr.md` |
| W3E-S02 | S | EN 1993-1-9 Table 3.1 (γ_Mf: assessment method × consequence class) | `ndp.en1993.1-9.gamma-mf.damage-tolerant.low` 1.00; `.damage-tolerant.high` 1.15; `.safe-life.low` 1.15; `.safe-life.high` 1.35 (new, EN = DE) | as listed | The four cells follow `📐️specs/en1993-target.md` V-502 (low/high consequence), not the print; DIN EN 1993-1-9/NA kept equal (V-502 notes the NA may restrict damage tolerant to inspected details). The flat row `ndp.en1993.gamma-mf` 1.15 remains for the crane runway check (EN 1993-6, no method input). Which cell a detail uses is the new input `consequence` (default Low = the reading all earlier results assumed, which is the unconservative column). | `wave3/en1993-corr.md` |
| W3E-S03 | S | EN 1993-1-5 Annex A.3 (k_τ with longitudinal stiffeners, k_τ,sl = 9 (h_w/a)² (I_sl/(t³ h_w))^(3/4) ≥ 2.1/t (I_sl/h_w)^(1/3)) | none (formula in code `plated_ext::k_tau_stiffened`) | a/h_w ≥ 1: 5.34 + 4.00 (h_w/a)²; a/h_w < 1: 4.00 + 5.34 (h_w/a)² (both + k_τ,sl) | The base branches are in the repo spec (V-367); the spec text of k_τ,sl is garbled, the lower bound 2.1/t (I_sl/h_w)^(1/3) is fixed by dimensions (dimensionless) but its coefficient 2.1 and exponent 3/4 are from memory. Compare with the print. | `wave3/en1993-corr.md` |

#### EN 1994

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S17 | S | 9.8.2 (composite slab deflection span divisor) | `ndp.en1994.slab.deflection-divisor` (new) | 250 / 250 | The beam row has DE 300, the slab row 250 (open observation). DE slab value needs checking. | literals-fix open observations |

#### EN 1995

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S18 | S | EN 338:2016 Tables 1 and 3, EN 14080:2013 Table 5 (status change) | 46 rows: `ndp.en1995.strength.c16.ft0k`, `c20.rhomean`, `c27.fc90k`, `c30.fc0k`, `c30.ft0k`, `c35.fc90k`, `c40.fc90k`, `c45.fc90k`, `c45.e005`, `c50.fc90k`, `c50.ft0k` (11 C-class); 25 D-class rows (`d18`, `d24` x5, `d30` x3, `d35` x2, `d40` x4, `d50` x3, `d60` x2, `d70` x5); `gl24h` x3, `gl28h` x4, `gl32h` x3 (EN 14080) | as listed (values unchanged) | All 46 were verified in wave 2 and are unverified now. Nine EN 338 rows disagree with the library blue-prints (2016 edition): C20 rho_mean 400 (requests say 390); C27 f_c,90 2.5 (2.6); C35 f_c,90 2.7 (2.8); C40 f_c,90 2.8 (2.9); C45 f_c,90 2.9 (3.1), E_0,05 10100 (10000); C50 f_c,90 3.0 (3.2), f_t,0 33.5 (33); D24 f_c,90 4.9 (7.8), f_v 3.7 (4.0), E_0,05 8400 (8500), E_90 670 (640), G 630 (600); D40 f_c,0 27 (26), f_c,90 5.5 (8.3), f_v 4.2 (4.0); D70 f_c,0 36 (34), f_c,90 12 (13.5), f_v 5.0 (6), rho_k 800 (900). The registry now equals the second number of each pair (the library), and the first number does not occur in the registry. The request says neither source was adopted. Decide per row against the printed EN 338:2016 Table 1 or 3. | `requests.md` lines 498 and 1200; `correctness.md`; wave-2 item 7 |

#### EN 1996

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S19 | S | 6.2 (f_vlt, vertical load capacity factors) | `ndp.en1996.fvlt.f-b-factor` 0.065 (EN); `ndp.en1996.fvlt.de-k` 0.45 (DE); `ndp.en1996.fvlt.de-share.g1` 0.032, `.g2` 0.026, `.g34` 0.02 (DE) (new) | as listed | Value as used by the artifact. Split EN and DE rows exist, but the DE share values need the print. | literals-fix row 6 |
| W3-S20 | S | M-60 (minimum cover by exposure class) | `ndp.en1996.m60.cover.mx1` 0.02; `mx2` 0.02; `mx3` 0.025; `mx4` 0.03; `mx5` 0.04 (both annexes, new) | as listed | Locator "M-60" is an internal id, not a clause. Enter the EN 1996-1-1 clause (8.2 and the NA) before using the value. Durability verdict. | literals-fix row 7 |

#### EN 1997

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S21 | S | 2.4.5.2 (independent samples per investigation point for characteristic values) | `ndp.en1997.2.characteristic-samples-per-sounding` (new) | 1.0 / 1.0 | Modelling choice, not a print value. Sets k_n and X_k for every CPT or SPT derived layer. Wave 3 changed the code to one sample per sounding (`correctness-fix.md` section 2). Print needed for the clause text. | `correctness-fix.md` section 2 |
| W3-S22 | S | Table A.9 / A.10 / A.11 (correlation factors xi for static, ground-test and dynamic tests) | 34 rows: `ndp.en1997.xi.static.n1.1` 1.4 / 1.35, `n1.2` 1.4 / 1.35, `n2.1` 1.3 / 1.25, `n2.2` 1.2 / 1.15, `n3.1` 1.2 / 1.15, `n3.2` 1.05, `n4.1` 1.1 / 1.05, `n4.2` 1.0, `n5.1` 1.0, `n5.2` 1.0 (static); `ndp.en1997.xi.ground.*` (14 rows, n = 1 to 10, xi3 and xi4); `ndp.en1997.xi.dynamic.*` (10 rows, n = 2 to 20, xi5 and xi6) | as listed | Relabelled from code EN 1998 to EN 1997 in wave 3 (the 36-row defect in wave 2). Values unchanged. Repo has no print. DE static values cite DIN 1054 Table A 7.1. | `checklist` section 2 item 15; `🔍️audits/wave3/correctness.md` |
| W3-S23 | S | Annex H.2 (SLS limits: angular distortion and tilt) | `ndp.en1997.sls.angular-distortion-limit` 0.002 / 0.002; `ndp.en1997.sls.tilt-limit` 0.004 / 0.004 | as listed | Relabelled from EN 1998 to EN 1997 in wave 3. Serviceability verdict. | registry diff |
| W3-S24 | S | Ground anchors, Tables A.5 to A.14 (gamma_a, resistance set R1 to R4) | `ndp.en1997.r3.a` EN 1.0 / DE 1.1 (split in wave 3); `ndp.en1997.r1.a` 1.1; `ndp.en1997.r2.a` 1.1; `ndp.en1997.r4.a` 1.1 | as listed | r3 EN 1.0 comes from groundhog, which is a library, not the print. DE 1.1 needs the DIN 1054:2021-04 print. r1, r2 and r4 are 1.1 for both annexes. | `requests.md` line 1283 |

#### EN 1998

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S25 | S | 5.4.3.1.2(6), 5.5.3.1.2 and 5.5.3.1.3 (DCM and DCH beam detailing: critical length factor, hoop spacing, absolute limits) | `ndp.en1998.en1.rc.beam.critical-length.depth-factor.dch` 1.5; `.dcm` 1.0; `ndp.en1998.en1.rc.beam.hoop-spacing.bar-diameter-factor.dch` 6.0; `.dcm` 8.0; `ndp.en1998.en1.rc.beam.hoop-spacing.depth-divisor` 4.0; `ndp.en1998.en1.rc.beam.hoop-spacing.hoop-diameter-factor` 24.0; `ndp.en1998.en1.rc.beam.hoop-spacing.max-m.dch` 0.175; `.dcm` 0.225 (8 rows, new) | as listed | Author's knowledge (`en1998.md`, Unverified). Not yet implemented in code: 1.5 times l_cr at column base, and omega_wd minimum only on the base region. Locators need checking against the print. | `en1998.md`, Unverified |
| W3-S26 | S | 5.4.3.2.2 and 5.5.3.2.2 (DCM and DCH column detailing: critical length, hoop spacing, nu_d limits, confinement) | 18 column and confinement rows: `critical-length.clear-length-divisor` 6.0; `.full-height-slenderness` 3.0; `.min-m` 0.45; `hoop-spacing.bar-diameter-factor.dch` 6.0 / `.dcm` 8.0; `hoop-spacing.core-divisor.dch` 3.0 / `.dcm` 2.0; `hoop-spacing.max-m.dch` 0.125 / `.dcm` 0.175; plus `nu-d-max.dch` 0.55 / `.dcm` 0.65; `confinement.mu-phi-coefficient` 30; `confinement.offset` 0.035; `hoop-diameter.dch-longitudinal-factor` 0.4; `hoop-diameter.min-m.dch` 0.006 / `.dcm` 0.006; `restrained-bar-spacing.max-m.dch` 0.15 / `.dcm` 0.2 (all new) | as listed | Author's knowledge. nu_d limits come from the target spec (SEI-50), not from print. mu_phi factor 1.5 for steel class B is not implemented. Clause 5.4.3.2.2(11) and (8) to check. | `en1998.md`, Unverified |
| W3-S27 | S | EN 1998-3 Annex A, A.3.3 (shear capacity coefficients) and the definitions of mu_pl and rho_tot | `ndp.en1998.en3.shear.axial-limit` 0.55; `.base` 0.16; `.ductility-cap` 5.0; `.ductility-slope` 0.05; `.rho-min` 0.5; `.slenderness-cap` 5.0; `.slenderness-slope` 0.16 (all A.3.3) | as listed | Verify all seven coefficients. Also verify the definitions mu_pl = theta_Ed / theta_y - 1 and rho_tot = (A_s1 + A_s2) / (b h). | `requests.md` line 1984 |
| W3-S28 | S | EN 1998-3 Annex A, section constants (fibre model used for the cyclic shear capacity and the neutral axis) | `ndp.en1998.en3.annex-a.section.eps-c2-permille` 2.0; `.eps-cu-permille` 3.5; `.eps-su-permille` 45.0; `.max-concrete-mpa` 58.0; `.parabola-exponent` 2.0; `ndp.en1998.en3.annex-a.steel-modulus-mpa` 200000 | as listed | Unverified rows, used by the neutral axis x (`en1998.md`, Unverified). | `en1998.md` |
| W3-S29 | S | NA-DE, return period for every limit state (DE annex) | `ndp.en1998.en3.na.return-period` EN none / DE 475 (changed) | DE 475 | Source moved from DIN EN 1998-1/NA to the NA of EN 1998-3 "to be identified". Under the DE annex every limit state uses T_R 475 and the design value collapses SD, NC and DL demand. The EN column is empty. | `en1998.md` item 2 and Unverified; registry diff |

#### EN 1999

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S30 | S | 6.7.6 and 6.7.7 (web stiffeners: strip factor, inertia factors, aspect limit, buckling length) | `ndp.en1999.web-stiffener.strip-factor` 15; `.inertia-factor-long` 0.75; `.inertia-factor-short` 1.5; `.aspect-limit` sqrt(2); `.buckling-length-factor` 0.75 | as listed | `asWritten` now cites EN 1993-1-5 9.1(2), 9.3.3(3) and 9.4(2). The EN 1999-1-1 6.7.6 and 6.7.7 printed values are needed. Clause number for buckling length (9.4(2)) from memory. | `requests.md` line 1481; `en1999-stiffeners.md` |
| W3-S31 | S | 3.2 Tables 3.1 and 3.2 (EN 1999-1-2 fire, reduction factors per alloy) and 4.2.5 (emissivity), EN 1991-1-2 3.2 (alpha_c) | `ndp.en1999.fire.k-0.2proof.*` (10 rows, Table 3.1, from `.20` 1.0 to `.550` 0.0); `ndp.en1999.fire.k-e.*` (10 rows, Table 3.2); `ndp.en1999.fire-alpha-c` 25; `ndp.en1999.fire-emissivity` 0.3 | as listed | Printed EN 1999-1-2 Tables 3.1 and 3.2 per alloy needed to verify the 20 table rows. | `requests.md` line 1949 |
| W3C-E01 | S | EN 1999-1-1 Table 8.2 (minimum spacing p2 perpendicular to the load, with e1/e2 1.2 d0 and p1 2.2 d0) | `ndp.en1999.table-8-2.p2-over-d0` 2.4 | 2.4 / 2.4 | Recalled from the printed table, not checked (the repo holds no copy). Read via the registry by the check `en1999.8.5.gauge`; the e1/e2 (1.2) and p1 (2.2) entries are still code constants and need the same print check. Also check any footnote on staggered rows. | `en1999.md` (wave 3c) |
| W3C-E02 | S | EN 1999-1-1 8.5 (fastener group under an eccentric shear, instantaneous-centre method) | `ndp.en1999.fastener-ic.curve-mu` 3.4; `ndp.en1999.fastener-ic.curve-exponent` 0.55 | 3.4 / 0.55 | EN 1999-1-1 gives no instantaneous-centre rule; the curve R/R_ult = (1 - exp(-mu r/r_max))^k (Crawford and Kulak 1971, AISC Steel Construction Manual Part 7: 10 /in * 0.34 in, 0.55) is common practice for 3/4 in steel bolts and is applied to aluminium fastener groups by analogy. Decide whether the method is acceptable for aluminium and verify the constants. The elastic method stays the default. | `en1999.md` (wave 3c) |

#### Report and request items without a registry row (safety-critical)

| ID | Impact | Clause / table | Registry key(s) | Current value | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-S32 | S | EN 1995-1-2 3.4.3.3 (T-103 charring: 25 mm cap on the beta_0 branch, k2 stage for type F boards) | none | k2 = 1 - 0.018 h_p (code), no 25 mm cap | Clause not in repo spec. Nothing changed; values stay unverified. | `en1995.md` (Unverified / open) |
| W3-S33 | S | EN 1995-1-1 8.5.1.3 (embedment of bolts and dowels in plywood) | none | nail / panel formula 0.11 rho_k d^-0.3 as stand-in | Info outcome `plywood_bolt_unverified` added, the check still uses the stand-in. | `en1995.md` (row 47 and Unverified) |
| W3-S34 | S | EN 1995-1-1 7.3.3 with DIN EN 1995-1-1/NA velocity limit for 6 <= f1 < 8 Hz | none | not implemented | No check at all. A missing check passes silently. Needs the NA text. | `en1995.md` (Unverified) |
| W3-S35 | S | EN 1996-1-2 tabulated fire data (M-90), other than clay group 1 | none | only clay group 1 row | No other rows added. Print needed for the remaining groups. | `en1996.md` item 6 |
| W3-S36 | S | EN 1996-1-1 Annex E, Table E (fixed-edge moment coefficients alpha_1, edge classes C to J) | `ndp.en1996.annex-e.<class>` (basis rows); requested `m31.annex-e-fixed` not in registry | yield-line stand-in | Cells left unkeyed because the print is absent. Requested row not added. | `en1996.md` item 1 and 2; `requests.md` line 548 |
| W3-S37 | S | EN 1996-1-1 8.2.3 (bed-joint reinforcement, minimum bed-joint thickness >= bar diameter + 5 mm) | requested `m70.bed-joint-bar` (not in registry) | n/a | Requested, not added. Text to be checked against the print. | `requests.md` line 548 |


### 5.2 Conservative (confirm direction before changing)

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-C01 | C | EN 1992-4 Table 4.1, "other" partial factor gamma_Ms | `ndp.en1992.anchor-gamma-ms.other` (new) | 1.5 / 1.5 | The artifact uses 1.5 for tension and shear outside the validity limits. Recalled alternative 1.4 for tension. If the print says 1.4 the artifact is conservative for tension. Shear direction unknown. | literals.md item 4; literals-fix open observations |
| W3-C02 | C | EN 1994-1-1 7.4.3 (0.4) against EN 1994-2 6.8.5.4 (0.2), tension stiffening | `ndp.en1994.1-1.tension-stiffening` 0.4; `ndp.en1994.2.fatigue-tension-stiffening` 0.2 (existing rows) | 0.4 / 0.4 and 0.2 / 0.2 | Code adds the increment to the range, which is the safe side for the stress range. The factor reading is unverifiable here. | `correctness.md` item 7b |
| W3E-C01 | C | EN 1993-1-8 Table 3.4 (punching shear of bolt heads and nuts, d_m) | none (formula in code `joints::punching_mean_diameter`) | d_m = mean of 1.5 d and 1.5 d · 2/√3 | The code (and the oracle) idealise the head as a regular hexagon of 1.5 d across flats; the product standard heads (EN ISO 4014, e_min about 1.09 s) give a d_m about 3 % smaller, so B_p,Rd is about 3 % high. Confirm which dimension the print uses (head or nut, the smaller one). CORR-2: replaced by 16 registry rows `ndp.en1993.1-8.3.6.head-nut.m{12,16,20,22,24,27,30,36}.across-{flats,points}` (EN ISO 4032 nut s_min and e_min, recalled, unverified): confirm each against the printed product standard (and the Table 3.4 wording). | `wave3/en1993-corr.md` |

### 5.3 Display only

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-D01 | D | EN 1991-3 2.9 (crane test loads, static and dynamic) | `ndp.en1991.3.test-static` 1.1; `ndp.en1991.3.test-dynamic` 1.25 (EN = DE) | 1.1 / 1.25 | Spec contradicts itself (E3.04: 1.1 static and 1.25 dynamic; phi_6 line: dynamic about 1.1). Crane practice gives 1.25 static. Information rows only, no verdict depends on them. | `correctness.md` issue 17; `correctness-fix.md` section 3 |
| W3-D02 | D | Terzaghi one-dimensional consolidation (T90, time factor) | `ndp.en1997.6.6.t90-time-factor` 0.848 (new) | 0.848 / 0.848 | Mathematical root of the Terzaghi series (0.848085). No normative clause. Used to derive c_v from lab tests, so a wrong constant would scale time estimates, not safety verdicts. | `en1997.md` item 3 and "Unverified" |
| W3-D03 | D | EN 1994 example limits: frequency limit, psi_vib, phi, eps_cs and lambda_p, c_p, rho_p | none (code example values, confidence low) | 3 Hz, 0.3, 2.5, 0.3 permille | Example values, no normative limit encoded. Frequency limit is a user input. | `en1994.md` (Unverified / honest limits) |
| W3-D04 | D | EN 1996-3 applicability (advisory for shear and non-load-bearing walls) | requested `m80.advisory` (not in registry) | n/a | Text only. | `requests.md` line 548 |
| W3-D05 | D | EN 1991-2 walking band (1.7 to 2.1 Hz) | `ndp.en1991.2.footbridge-band-*` rows | as listed | Walking verdict now shows only inside the comfort trigger. No pass or fail depends on it. | `correctness-fix.md` section 4; `correctness.md` issue 17c |
| W3-D06 | D | Locator metadata | `ndp.en1992.nu2-*` locator "6.2.3(103)"; `ndp.en1996.m60.cover.*` locator "M-60"; `ndp.en1993.built-up.bow-divisor` locator "6.4.1"; `ndp.en1998.en3.na.return-period` EN value none | n/a | Labels and locators to correct once the print is read. No value depends on them. | this section |

### 5.4 Missing or newly stated NDP values (W3-NDP-VALUES)

Added 2026-11-03 by W3-NDP-VALUES; full list with ids in `🔍️audits/wave3/ndp-values-missing.tsv`, reasoning in `🔍️audits/wave3/ndp-values.md`. Impact tier S unless noted.

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-V01 | S | EN 1997-1 7.7 (pile head deflection limit), 9.7 (Blum embedment increase), EN 1998-5 7.3.2 (seismic sliding gamma_Rh,E) | `ndp.en1997.7.7.lateral-deflection-limit` 0.010 m; `ndp.en1997.9.blum-embedment-increase` 1.2; `ndp.en1997.9.seismic.gamma-sliding` 1.0 | 0.010 / 0.010, 1.2 / 1.2, 1.0 / 1.0 | The DE column was empty and the check silently used the same constant as an in-code fallback. The DE column now states it explicitly (unverified). Read DIN 1054:2021-04 for the real national values. | `ndp-values.md` |
| W3-V02 | S | EN 1992-1-1 4.4.1.1 (Dc,dev), 2.2 (Dc,dur,gamma = 0), 6.8.4 Table 6.3N (Delta sigma_Rsk straight bars) | `ndp.en1992.d-04.1` 10 mm; `ndp.en1992.d-03.1` 0 mm; `ndp.en1992.f-03.1` 162.5 MPa | 10 / 10, 0 / 0, 162.5 / 162.5 | New numbers on former citation rows, taken from `📐️specs/en1992-target.md` lines 81, 76, 333 (project-written spec, not the print). DE 162.5 carries an open question mark on the bar diameter limit. | `ndp-values.md` |
| W3-V03 | S | EN 1993-1-1 5.3.4 (k), 6.3.2.3 (lambda_bar_LT,0), EN 1993-1-11 (gamma_R), EN 1993-2 (gamma_M0) | `ndp.en1993.v-026.1` 0.5; `ndp.en1993.v-111.1` 0.4; `ndp.en1993.v-860.1` 1.5; `ndp.en1993.v-880.1` 1.0 | each equal in both annexes | New numbers on former citation rows, from `📐️specs/en1993-target.md` lines 69, 140, 292, 295 (spec marks 5.3.4 and 6.3.2.3 "[verify]", 1-11 with a question mark). | `ndp-values.md` |
| W3-V04 | S | EN 1991-1-6 Table 3.1 (stage return period, DE column) | `ndp.en1991.1-6.stage-return.{days3,months3,year1,longer}` EN 2 / 5 / 10 / 50 | EN only | The modified DE NA table is not keyed. Under the DE annex the stage return-period check reports "awaiting values". Transcribe the DE column. | `ndp-values.md` |
| W3-V05 | S | EN 1994-2 6.6.5 / 6.6.6.3 (stud edge distance 20 mm, transverse spacing 2.5 d / 4 d, minimum transverse reinforcement 0.002) | `ndp.en1994.2.stud-edge-distance`, `.stud-cross-spacing`, `.min-transverse-reinforcement` | EN only | DE column empty; the check text cites the rows but the code holds the constants as literals. Read the DE NA and route the constants through the rows. | `ndp-values.md` |
| W3-V06 | S | EN 1995-1-2 6.2 (reduced-load connection constant k) | `ndp.en1995.fire.connection-reduced-load-k` | none / none | No value known; the outcome `connection_fire_time` is only produced when the case supplies `k_fire`. | `ndp-values.md` |
| W3-V07 | S | EN 1999-1-1 6.7.6 / EN 1999-1-5 (ring stiffener design margin gamma_r) | `ndp.en1999.ring-stiffener.required-inertia-factor` | none / none | Not transcribed. The check evaluates with 1.0 and reports "awaiting verified margin" (documented in the row). | `ndp-values.md` |
| W3-V08 | D | Rule and formula blocks cited by checks (en1990 10, en1991 41, en1992 24, en1993 57, en1994 13, en1996 79, en1999 2 rows) | ids in `ndp-values-missing.tsv` class "citation of a rule or formula block" | none / none | The rows are provenance for a clause with several constants (for example `ndp.en1996.m10.rho2`, `ndp.en1993.v-420.1`). No single number exists; the constants live in their own rows or in the code. Splitting each block into numbered constant rows needs the print. | `ndp-values.md` |

### 5.5 EN 1998 wave-3 recollections (W3-EN1998-NORM d, added 2026-11-03 by W3-OPEN-9899)

All 18 rows are `unverified` recollections or literature bases, written by `🧰️scripts/en1998-w3d-ndp.py`; EN and DE are equal. Evidence: `🔍️audits/wave3/en1998-norm.md`.

| ID | Impact | Clause / table | Registry key(s) | Current value (EN / DE) | Why in doubt | Source |
|---|---|---|---|---|---|---|
| W3-G01 | S | EN 1998-5 4.2.2 Table 4.1 (average soil damping ratios and reduction factors for a_g S/g = 0.10 / 0.20 / 0.30) | `ndp.en1998.en5.site-response.vs-ratio.{a010,a020,a030}`, `ndp.en1998.en5.site-response.damping.{a010,a020,a030}` | v_s/v_s,max 0.90 / 0.70 / 0.60; damping 0.03 / 0.06 / 0.10 | Recollection of the printed table (G/G_max = (v_s/v_s,max)^2 = 0.80 / 0.50 / 0.36). Read the table and its interpolation rule; the profile shear-modulus check degrades G with it. | `en1998-norm.md` |
| W3-G02 | S | EN 1998-5 4.1.4 (pore pressure ratio r_u in liquefiable layers) | `ndp.en1998.en5.liquefaction.ru.alpha` 0.7; `ndp.en1998.en5.liquefaction.ru.cycle-exponent` 0.34 | 0.7 / 0.34 | Not a provision of EN 1998-5: r_u = 0.5 + asin(2 (N/N_L)^(1/alpha) - 1)/pi (Seed, Martin and Lysmer 1976), N/N_L = F_L^(-1/b) with b = 0.34 (Idriss and Boulanger). Decide whether the law is acceptable or the standard gives another one. | `en1998-norm.md` |
| W3-G03 | C | EN 1998-5 4.1.4 / Annex (lateral spreading ground slope range) | `ndp.en1998.en5.liquefaction.spreading.min-slope-percent` 0.1; `...max-slope-percent` 6.0 | 0.1 % / 6.0 % | Youd, Hansen and Bartlett (2002) range of their lateral spreading model, not a provision of EN 1998-5. Steeper ground is reported as flow slide (slope stability governs). Confirm the range. | `en1998-norm.md` |
| W3-G04 | S | EN 1998-5 5.4.2 (pile in liquefied soil) | `ndp.en1998.en5.pile.liquefied.length-factor` 1.0; `...fixity-factor` 1.0 | 1.0 / 1.0 | K = 1 hinged-hinged Euler column (Bhattacharya 2004) and fixity one Winkler length 1/beta below the liquefied layer: engineering assumptions. Confirm against the print. | `en1998-norm.md` |
| W3-G05 | S | EN 1998-2 chapter 7 and EN 1998-1 chapter 10 (isolation limits) | `ndp.en1998.{en1,en2}.isolation.min-period-ratio` 3.0; `.max-period-s` 3.0 s; `.max-damping` 0.30 | 3.0 / 3.0 s / 0.30 for both parts | T_eff >= 3 T_fixed and T_eff <= 3 s come from the target spec rows SEI-53 and BR-10; the 30 % damping limit is a recollection. Read the limits of the equivalent linear analysis (and the NA-DE) in both parts. | `en1998-norm.md` |
