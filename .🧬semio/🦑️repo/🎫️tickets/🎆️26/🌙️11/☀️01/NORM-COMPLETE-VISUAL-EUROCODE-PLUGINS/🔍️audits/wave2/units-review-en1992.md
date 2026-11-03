# EN 1992 design-case vocabulary: units review

Scope: the 445-row EN 1992 input vocabulary of the 172 catalogue entries (174 registry ids incl. aliases), generated from `🧰️scripts/catalogue-bridge-en1992-inputs.tsv`. The previous rows were name-rule guesses (`catalogue-bridge-en1992-build.py`, all labels `Input <key>` / `Eingabe <key>`). Every row was re-derived from the evaluator arithmetic of the `Case`-based registry (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/.../⚖️checks/<module>/🦀️.rs`). The stored unit is the unit of the number the evaluator reads (`c.n/c.pos/c.o/c.opt/c.l`): N, mm, N/mm², N·mm, angles as the code converts them.

## Result

- 447 old rows (the report text cites 445) become 491 rows (42 more rows from scope splits of keys with different meanings, the stale `nz` that no entry declares was dropped, 3 rows were missing: `hole_y`, `hole_z` (vector, mm; T-01, T-04, SG-01, SG-02) and `state` (choice; SG-02), which the evaluators read but the vocabulary did not declare).
- 0 generic labels left: every row has an EN and DE label with symbol and meaning.
- Unit or kind changed or row added: 140 rows (29 of the 31 old text rows became 38 closed `choice` rows, scoped copies included; all 33 vector rows now carry their element unit instead of `1`; 75 number rows changed unit; see the table). 351 rows keep unit and kind (labels only).
- 116 `@choice` values for 38 choice rows, scoped where the accepted set differs (`kind` 4 ways, `system` 2, `shape` 2, `situation` 2, `steel_class` 2, `bond` 2).
- Coverage `catalogue-bridge-check-coverage.py en1992 <registry>/📚️inputs`: 174 entries, 491 rows, missing []. Shared oracle `⚖️compliance/🧰️design-case/🔮️oracles/🐍️.py <vocabulary.json>` (python -I): `{"searches": 12, "levers": 2, "vocabularies": [491]}`. No cargo, no build.
- Scoped rows win over the unscoped row of the same key (`design_case::InputSpec` lookup and the demo seed), so scoped exceptions overlay one unscoped default; scoped sets never overlap each other.

## Regeneration

```
python3 -I 🧰️scripts/catalogue-bridge-en1992-units.py            # writes catalogue-bridge-en1992-inputs.tsv (this script is now the source, the old builder is superseded)
python3 -I 🧰️scripts/catalogue-bridge-gen-inputs.py <registry>/📚️inputs 🧰️scripts/catalogue-bridge-en1992-inputs.tsv s.norm.en1992
python3 -I 🧰️scripts/catalogue-bridge-check-coverage.py en1992 <registry>/📚️inputs
python3 -I ✏️s/🔌️plugins/📕️norm/⚖️compliance/🧰️design-case/🔮️oracles/🐍️.py <registry>/📚️inputs/🔣️.json
python3 -I 🧰️scripts/en1992-v4/demo-gen.py                        # reseeds the demo rs and the demo-document fixture from the new examples
python3 -I 🧰️scripts/en1992-units-report.py <old 🔣️.json> > table  # the change table below
```

Tool edits: `catalogue-bridge-gen-inputs.py` got the units `pct` (`Percent`, %) and `mm2_m` (`SquareMillimetrePerMetre`, mm²/m) (scoped `@choice key@scope` and `per_m` were added concurrently by another editor and are used as is); `catalogue-bridge-check-coverage.py` now also matches `entry_of(...)` (before it saw 0 entries and passed vacuously); `catalogue-bridge-en1992-build.py` docstring marks it superseded. Evaluator code is untouched.

## Real defects the old guesses carried (physically wrong stored unit)

| key | old | new | why |
|---|---|---|---|
| `e_p` | N/mm² | mm | eccentricity of the prestress in `P (e + W/A)` (M-14, SL-09, BR-06) |
| `theta_pl` | ° | rad | compared with `(κu − κy) l_pl` in rad (A-07) |
| `tendon_angle` | ° | rad | `p_force * tendon_angle.sin()` without `to_radians` (P-07) |
| `theta_m`, `zone_theta`, `theta` (M-13, R-07) | °, 1, 1 | °C | arguments of the temperature tables `kc_theta`, `ks_theta` (R-04, R-07) |
| `alpha_m` | m | 1 | factor α_M of the anchor steel shear (FA-07) |
| `phi_m` | m | mm | mandrel diameter `F_bt (1/a_b + 1/2φ)/f_cd` in mm (DT-13, AJ-02) |
| `pm_t_per_m` | m | N/mm | `(P_m,t − p R)/t` with p in N/mm², R and t in mm (LQ-06) |
| `r_m`, `e_d` | m, mm | N | `Outcome::force(E_d, R_m/γ)` (A-08) |
| `h_ed` | mm | N | horizontal force H_Ed of the corbel (AJ-03) |
| `a_c` | mm² | mm | shear span in `F a_c/z_0` (AJ-03) |
| `amp_y`, `amp_z`, `xu_d`, `xy_d`, `c0`, `n_s` | mm/mm/mm/mm/mm/N | 1 | factors, ratios and a storey count |
| `ei` | 1 (example 1e13 N·mm²) | mm | it is the imperfection eccentricity e_i = l_0/400 in A-10 (not a stiffness) |
| `ic`, `is_moment`, `i_b`, `s_first`, `is`, `i` | mixed | mm⁴, mm⁴, mm⁴, mm³, mm, mm | second moments, first moment, radii of gyration |
| `ac_per_m`, `as_main`, `as_secondary`, `as_provided` (DT-11) | mm² | mm²/m | the evaluators use `1000 mm` strips (DT-06, DT-11) |
| `t` | 1 | d (M-04, M-10, M-11, A-01, A-02, B-07), mm (LQ-06, AF-01) | age in days vs thickness |
| `rh`, `rho1`, `rho1000`, `moisture` | 1 | % | divided by 100 or percent tables in the code |
| `p_transverse`, `pressure`, `sx`, `sy`, `txy` | 1 | N/mm² | stresses |
| `mx`, `my`, `mxy` (AL-01) | N·mm | N | moments per unit width, N·mm/mm = N |
| `nx`, `ny`, `nxy` (AL-01) | 1 | N/mm | membrane forces per unit width |
| `q` (SL-08) | 1 | N/mm | `m = q x (l − x)/2` |
| `delta_fd`, `delta_v`, `gk`, `qk` | 1 | N | forces |
| `t_w` (T-05) | mm | N·mm | `T_w = dB/dx`, `V_f = T_w/h_s` |
| `k` (PR-04) | 1 | 1/m | wobble `exp(−μ(θ + k x))` with x in m |
| `x`, `theta` (PR-04), `kappa`, `ranges`, `ax`, `ay`, `b_i`, `bar_*`, `poly_*`, `t_*`, ... | 1 | mm, m, rad, 1/m, N/mm², ... | vector elements now carry their unit |
| `bar_y` (R-03) | vector | number mm | R-03 reads it as `c.o` (one bar) |
| `n_ed` FA and DT-02 | N (compression) | N (tension positive) | scoped sign convention, examples 20 kN for the anchors |
| text keys (29) | text | choice with EN/DE labels | closed accepted sets read from the `match`/`==` arms; `classes` and `faces` stay text (comma list, letters) |

Defaults in the code that the stored examples now respect: `ground` none, `structure` building, `tendon_kind` post_curved_steel, `exposure` XC4 (cover table rows only), `beta_computed` 0 (it was 1.15 = switched on).

## Unit requests (no matching `UnitId`; kept as `1` with the unit in the label)

- `mm²/mm` (A_sw/s: `asw_s`, `asw_s_provided`), numerically equal to `mm` but a different quantity; `UnitId::SquareMillimetrePerMetre` differs by a factor 1000.
- `N·mm²` (`bimoment` B, `sum_ecd_ic` ΣE_cd I_c); stiffness N·mm² also appears in outcomes.
- `1/K` (`alpha_c`, thermal expansion).
- Strain in permille (`eps`, `eps_uk`, `eps_ud`, `eps_ud_p`, `t_pre`): `Percent` and `Ppm` exist, permille does not.
- `N·mm/mm` plate moments (`mx`, `my`, `mxy`) are stored as `N` (numerically exact, N·m/m = N); a dedicated moment-per-width unit would be clearer.
- `N·mm` per width for `pm_t_per_m` is not needed (it is N/mm = kN/m, exists).

## Uncertain rows (kept, derived by judgement or flagged `medium` in the vocabulary)

| key | scope | open point |
|---|---|---|
| `alpha1` | B-04 | read as α_1 factor of the lap length (undeclared read); EN 1992 uses α_1 for the shape; meaning not named in code |
| `alpha_deg` | S-01 | direction of the neutral axis for `state(alpha, x)`; convention of the angle not documented in code (default 90) |
| `alpha_m` | FA-07 | α_M of the fixity; EN 1992-4 gives 1.0 or 2.0 |
| `bar_z` | R-03 | declared by R-03 but not read by `r03` (it reads `a` and `bar_y`), kept as vector |
| `e_p` | M-14, SL-09, BR-06 | sign convention of the eccentricity (positive below the centroid in both M-14 `M_cr + P (e + W/A)` and SL-09 `−P e/W`; BR-06 `M − P e`); only the sign is a convention, the unit is mm |
| `k_conf` | B-03 | K of Figure 8.4 (0.1, 0.05, 0), only a coefficient in code |
| `m` | A-03 | `c.o("m", 1.0)`: number of vertical members; real meaning read from the formula α_m = √(0.5(1 + 1/m)) |
| `mx`, `my`, `mxy` | AL-01 | stored as `N` (N·mm/mm) for lack of a moment-per-width unit |
| `pm_t_per_m` | LQ-06 | the key says per metre; the arithmetic is N/mm = kN/m (same quantity) |
| `sum_ecd_ic`, `bimoment`, `asw_s`, `asw_s_provided`, `alpha_c` | A-14, T-05, V-03, T-02, LQ-02 | unit with no `UnitId` (see requests) |
| `t_w` | T-05 | T_w = dB/dx taken as N·mm from `V_f = T_w/h_s` |
| `theta` | SL-06 | undeclared read; radians from `th.cos()`; not seeded |
| `theta_m` | R-04 | °C; the physical meaning of M (point of the zone method) is not stated in code |
| `width` | R-01, D-06 | `c.o("mu_fi", c.o("width", 0))`: beam width, but for columns the same slot takes μ_fi |

Examples were also adjusted where the unit changed (old values in the table). The fixture `🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json` and `🗂️cases/🌱️demo/🦀️.rs` were reseeded with `demo-gen.py`; `cargo test` was not run.

## Full change table (key, scope, old, new, evidence = evaluator file and line of the function that reads the key)

Evidence paths are relative to `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/`. `status` is `unit/kind` when stored unit or kind changed, `added` for new rows, `label` when only the label (and example) changed.

| key | scope | old kind unit (old label) | new kind unit | new EN label | status | evidence |
|---|---|---|---|---|---|---|
| `a` | R-01,D-06,R-03,R-05 | number mm (Input a) | number mm ex 40 | Axis distance a (centre of bar to nearest exposed face) | label | 🔥️fire-design/🦀️.rs:79; 🔥️fire-design/🦀️.rs:145; 🔥️fire-design/🦀️.rs:179 |
| `a` | PR-09 | number mm (Input a) | number mm ex 200 | Width a of the anchor plate | label | 🧵️prestress-and-fatigue/🦀️.rs:104 |
| `a` | DT-12 | number mm (Input a) | number mm ex 400 | Column dimension a | label | 🔗️detailing/🦀️.rs:272 |
| `a` | P-03 | number mm (Input a) | number mm | Distance a of the checked perimeter from the loaded area (used when a < 2d) | label | 🕳️punching-and-struts/🦀️.rs:167 |
| `a` | PA-01 | number mm (Input a) | number mm | Width a of the loaded area | label | 🕳️punching-and-struts/🦀️.rs:356 |
| `a` | AL-01 | number mm (Input a) | number mm ex 40 | Distance a of the layer centre from the surface | label | 🧩️annex-parts/🦀️.rs:132 |
| `a_b` | all | number mm² (Input a_b) | number mm ex 200 | Centre-to-centre distance a_b of the bars | unit/kind | 🔗️detailing/🦀️.rs:284; 🧩️annex-parts/🦀️.rs:45 |
| `a_c` | all | number mm² (Input a_c) | number mm ex 150 | Shear span a_c of the corbel (load to column face) | unit/kind | 🧩️annex-parts/🦀️.rs:73 |
| `a_links` | all | number mm² (Input a_links) | number mm² ex 400 | Provided corner links A_sw | label | 🧩️annex-parts/🦀️.rs:45 |
| `a_t` | all | number mm² (Input a_t) | number mm² ex 600 | Top reinforcement A_t over the column | label | 🔗️detailing/🦀️.rs:212 |
| `ac` | all | number mm² (Input ac) | number mm² ex 90000 | Concrete cross-section area A_c | label | 🧵️prestress-and-fatigue/🦀️.rs:69; 🔗️detailing/🦀️.rs:133; 🔗️detailing/🦀️.rs:225; 🔗️detailing/🦀️.rs:248 |
| `ac0` | all | number mm² (Input ac0) | number mm² ex 40000 | Loaded area A_c0 | label | 🕳️punching-and-struts/🦀️.rs:356 |
| `ac1` | all | number mm² (Input ac1) | number mm² ex 360000 | Maximum dispersal area A_c1 | label | 🕳️punching-and-struts/🦀️.rs:356 |
| `ac_per_m` | all | number mm² (Input ac_per_m) | number mm²/m ex 500000 | Wall concrete area per metre of wall length A_c (thickness × 1000 mm) | unit/kind | 🔗️detailing/🦀️.rs:264 |
| `act` | all | number mm² (Input act) | number mm² ex 40000 | Area of the concrete tension zone A_ct | label | 🪟️serviceability/🦀️.rs:85 |
| `act_ext` | all | number mm² (Input act_ext) | number mm² ex 3000 | Area of the external tension zone A_ct,ext | label | 🔗️detailing/🦀️.rs:91; 🔗️detailing/🦀️.rs:173 |
| `action` | all | text - (Input action) | choice ex bending | Type of action on the member | unit/kind | 🪟️serviceability/🦀️.rs:124; 🪟️serviceability/🦀️.rs:193 |
| `air_entrained` | all | number 1 (Input air_entrained) | number 1 ex 0 | Air-entrained concrete (1 = yes) | label | 🧱️materials-and-time/🦀️.rs:377 |
| `alpha1` | all | number 1 (Input alpha1) | number 1 ex 1 | Anchorage shape factor α_1 of the lap | label | 🔗️detailing/🦀️.rs:66 |
| `alpha3` | all | number 1 (Input alpha3) | number 1 ex 1 | Factor α_3 for transverse reinforcement of the lap | label | 🔗️detailing/🦀️.rs:66 |
| `alpha_c` | all | number 1 (Input alpha_c) | number 1 ex 1e-05 | Coefficient of thermal expansion α_c [1/K] | label | 🌉️bridges-and-liquid/🦀️.rs:101 |
| `alpha_deg` | all | number ° (Input alpha_deg) | number ° ex 90 | Inclination α of the shear reinforcement to the member axis | label | ✂️shear-torsion/🦀️.rs:121; ✂️shear-torsion/🦀️.rs:206; 🔗️detailing/🦀️.rs:153; 🕳️punching-and-struts/🦀️.rs:200 |
| `alpha_deg` | S-01 | number ° (Input alpha_deg) | number ° ex 90 | Direction angle α of the neutral axis | label | 📐️section-resistance/🦀️.rs:446 |
| `alpha_e` | all | number 1 (Input alpha_e) | number 1 ex 6.06 | Modular ratio α_e = E_s/E_cm | label | 🧵️prestress-and-fatigue/🦀️.rs:137; 🪟️serviceability/🦀️.rs:193 |
| `alpha_l` | all | number 1 (Input alpha_l) | number 1 ex 1 | Factor α_l = l_x/l_pt2 for prestress transfer (at most 1) | label | ✂️shear-torsion/🦀️.rs:140 |
| `alpha_m` | all | number m (Input alpha_m) | number 1 ex 1 | Moment factor α_M of the anchor fixity | unit/kind | 🪝️fastenings/🦀️.rs:164 |
| `alpha_seis` | all | number 1 (Input alpha_seis) | number 1 ex 0.8 | Seismic reduction factor α_seis | label | 🪝️fastenings/🦀️.rs:245 |
| `alpha_v_deg` | all | number ° (Input alpha_v_deg) | number ° ex 0 | Angle α_V between the shear load and the edge normal | label | 🪝️fastenings/🦀️.rs:191 |
| `amp_y` | all | number mm (Input amp_y) | number 1 ex 1.2 | Moment amplification factor about the y axis | unit/kind | 🏛️analysis-models/🦀️.rs:178 |
| `amp_z` | all | number mm (Input amp_z) | number 1 ex 1.2 | Moment amplification factor about the z axis | unit/kind | 🏛️analysis-models/🦀️.rs:178 |
| `angle` | all | text - (Input angle) | choice ex 135 | Bend angle of the link hook | unit/kind | 🔗️detailing/🦀️.rs:119 |
| `ap` | all | number mm² (Input ap) | number mm² ex 1000 | Prestressing steel area A_p | label | 🧵️prestress-and-fatigue/🦀️.rs:21; 🧵️prestress-and-fatigue/🦀️.rs:51; 🧵️prestress-and-fatigue/🦀️.rs:137 |
| `approval` | all | number 1 (Input approval) | number 1 ex 0 | Shear studs with European technical approval (1 = yes) | label | 🕳️punching-and-struts/🦀️.rs:189 |
| `as` | all | number mm² (Input as) | number mm² ex 1571 | Tension reinforcement area A_s | label | 🪟️serviceability/🦀️.rs:15; 🔗️detailing/🦀️.rs:133; 🔗️detailing/🦀️.rs:225; 🏛️analysis-models/🦀️.rs:88 |
| `as` | B-03,B-04 | number mm² (Input as) | number mm² ex 314 | Area of the single bar A_s (anchorage, lap) | label | 🔗️detailing/🦀️.rs:40; 🔗️detailing/🦀️.rs:66 |
| `as` | FA-02,FA-07 | number mm² (Input as) | number mm² ex 157 | Stressed cross-section A_s of the anchor | label | 🪝️fastenings/🦀️.rs:74; 🪝️fastenings/🦀️.rs:164 |
| `as2` | all | number mm² (Input as2) | number mm² ex 628 | Compression reinforcement area A_s2 | label | 🪟️serviceability/🦀️.rs:15 |
| `as_main` | all | number mm² (Input as_main) | number mm²/m ex 1000 | Main reinforcement of the slab per metre | unit/kind | 🔗️detailing/🦀️.rs:182 |
| `as_provided` | all | number mm² (Input as_provided) | number mm² ex 1571 | Provided reinforcement area A_s,prov | label | 🔗️detailing/🦀️.rs:272; 🔗️detailing/🦀️.rs:294; 🧵️prestress-and-fatigue/🦀️.rs:104; 🪟️serviceability/🦀️.rs:85; 🕳️punching-and-struts/🦀️.rs:281; 🧩️annex-parts/🦀️.rs:45; 🧩️annex-parts/🦀️.rs:96; 📐️section-resistance/🦀️.rs:476; 🪝️fastenings/🦀️.rs:256; 🔗️detailing/🦀️.rs:173 |
| `as_provided` | DT-11 | number mm² (Input as_provided) | number mm²/m ex 565 | Provided mesh per metre A_s,prov | unit/kind | 🔗️detailing/🦀️.rs:264 |
| `as_req` | all | number mm² (Input as_req) | number mm² ex 1200 | Required reinforcement area A_s,req | label | 🪟️serviceability/🦀️.rs:263 |
| `as_secondary` | all | number mm² (Input as_secondary) | number mm²/m ex 400 | Secondary reinforcement of the slab per metre | unit/kind | 🔗️detailing/🦀️.rs:182 |
| `as_span` | all | number mm² (Input as_span) | number mm² ex 600 | Span reinforcement A_s,span | label | 🔗️detailing/🦀️.rs:142 |
| `as_support` | all | number mm² (Input as_support) | number mm² ex 600 | Reinforcement at the support A_s,sup | label | 🔗️detailing/🦀️.rs:142 |
| `as_surf` | all | number mm² (Input as_surf) | number mm² ex 1500 | Provided surface reinforcement A_s,surf | label | 🧩️annex-parts/🦀️.rs:34 |
| `as_surf_provided` | all | number mm² (Input as_surf_provided) | number mm² ex 1500 | Provided surface reinforcement A_s,surf | label | 🔗️detailing/🦀️.rs:91 |
| `as_through` | all | number mm² (Input as_through) | number mm² ex 400 | Integrity reinforcement through the column | label | 🔗️detailing/🦀️.rs:212 |
| `asl` | all | number mm² (Input asl) | number mm² ex 1257 | Longitudinal tension reinforcement A_sl | label | ✂️shear-torsion/🦀️.rs:140; ✂️shear-torsion/🦀️.rs:357 |
| `asv` | all | number mm² (Input asv) | number mm² ex 400 | Provided vertical wall reinforcement A_sv | label | 🔗️detailing/🦀️.rs:248 |
| `asw` | DT-03 | number mm² (Input asw) | number mm² ex 157 | Area A_sw of one link | label | 🔗️detailing/🦀️.rs:153 |
| `asw` | P-04 | number mm² (Input asw) | number mm² ex 5000 | Shear stud area A_sw per perimeter | label | 🕳️punching-and-struts/🦀️.rs:200 |
| `asw_s` | all | number mm² (Input asw_s) | number 1 ex 0.9 | Link reinforcement A_sw/s [mm²/mm] | unit/kind | ✂️shear-torsion/🦀️.rs:160; ✂️shear-torsion/🦀️.rs:93 |
| `asw_s_provided` | all | number mm² (Input asw_s_provided) | number 1 ex 0.9 | Provided torsion links A_sw/s [mm²/mm] | unit/kind | ✂️shear-torsion/🦀️.rs:329 |
| `at_laps` | all | number 1 (Input at_laps) | number 1 ex 0 | Splice region (maximum 8 % reinforcement) (1 = yes) | label | 🔗️detailing/🦀️.rs:225 |
| `av` | all | number mm (Input av) | number mm ex 400 | Shear span a_v (support face to load) | label | ✂️shear-torsion/🦀️.rs:131; ✂️shear-torsion/🦀️.rs:240 |
| `ax` | all | vector 1 (Input ax (vector)) | vector mm | Anchor x coordinates | unit/kind | 🪝️fastenings/🦀️.rs:29 |
| `ay` | all | vector 1 (Input ay (vector)) | vector mm | Anchor y coordinates | unit/kind | 🪝️fastenings/🦀️.rs:29 |
| `az` | all | number mm (Input az) | number mm ex 20 | Damaged zone a_z at the faces | label | 🔥️fire-design/🦀️.rs:179 |
| `b` | all | number mm (Input b) | number mm ex 300 | Section width b | label | 🪟️serviceability/🦀️.rs:15 |
| `b` | DT-12 | number mm (Input b) | number mm ex 1500 | Footing width b | label | 🔗️detailing/🦀️.rs:272 |
| `b_e` | all | number mm (Input b_e) | number mm ex 300 | Effective width b_e of the edge column strip | label | 🔗️detailing/🦀️.rs:212 |
| `b_i` | all | vector 1 (Input b_i (vector)) | vector mm | Flange overhang widths b_i | unit/kind | 🏛️analysis-models/🦀️.rs:23 |
| `bar_a` | all | vector 1 (Input bar_a (vector)) | vector mm² | Bar areas A_s,i | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `bar_y` | all | vector 1 (Input bar_y (vector)) | vector mm | Bar y coordinates | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `bar_y` | R-03 | vector 1 (Input bar_y (vector)) | number mm | Bar y coordinate | unit/kind | 🔥️fire-design/🦀️.rs:145 |
| `bar_z` | all | vector 1 (Input bar_z (vector)) | vector mm | Bar z coordinates | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `bars` | all | number 1 (Input bars) | number 1 ex 4 | Number of longitudinal bars | label | 🔗️detailing/🦀️.rs:225 |
| `beff_over_bw` | all | number 1 (Input beff_over_bw) | number 1 ex 1 | Flange ratio b_eff/b_w | label | 🪟️serviceability/🦀️.rs:263 |
| `beta` | V-05 | number 1 (Input beta) | number 1 ex 1 | Ratio β of the longitudinal force of the added concrete to the total | label | ✂️shear-torsion/🦀️.rs:206 |
| `beta` | SL-08 | number 1 (Input beta) | number 1 ex 0.5 | Load duration factor β (1.0 short-term, 0.5 long-term) | label | 🪟️serviceability/🦀️.rs:280 |
| `beta` | P-06 | number 1 (Input beta) | number 1 ex 1.15 | Load increase factor β of the foundation | label | 🕳️punching-and-struts/🦀️.rs:242 |
| `beta_cc_t0` | all | number 1 (Input beta_cc_t0) | number 1 ex 0.6 | Strength development factor β_cc(t_0) | label | 🧵️prestress-and-fatigue/🦀️.rs:179 |
| `beta_computed` | all | number 1 (Input beta_computed) | number 1 ex 0 | Compute β from the moment instead of the NDP value (1 = yes) | label | 🕳️punching-and-struts/🦀️.rs:116 |
| `bf` | all | number mm (Input bf) | number mm ex 1200 | Flange width b_f | label | 📐️section-resistance/🦀️.rs:581 |
| `bi` | all | number mm (Input bi) | number mm ex 300 | Width b_i of the interface | label | ✂️shear-torsion/🦀️.rs:206 |
| `bimoment` | all | number N·mm (Input bimoment) | number 1 ex 4000000000.0 | Bimoment B [N·mm²] | unit/kind | 🧩️annex-parts/🦀️.rs:96 |
| `bisect` | all | number 1 (Input bisect) | number 1 ex 14 | Number of bisection steps | label | 🏛️analysis-models/🦀️.rs:188 |
| `bond` | all | text - (Input bond) | choice ex good | Bond condition | unit/kind | 🔗️detailing/🦀️.rs:22; 🔗️detailing/🦀️.rs:29; 🔗️detailing/🦀️.rs:99 |
| `bond` | PR-07 | text - (Input bond) | choice ex bonded | Tendon bond | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:91 |
| `box` | all | number 1 (Input box) | number 1 ex 0 | Box section (adds the web shear from torsion) (1 = yes) | label | ✂️shear-torsion/🦀️.rs:357 |
| `bracing` | all | text - (Input bracing) | choice ex braced | Bracing of the column | unit/kind | 🏛️analysis-models/🦀️.rs:88 |
| `brittle_partitions` | all | number 1 (Input brittle_partitions) | number 1 ex 0 | Brittle partitions that deflection could damage (1 = yes) | label | 🪟️serviceability/🦀️.rs:263 |
| `bt` | all | number mm (Input bt) | number mm ex 300 | Mean width b_t of the tension zone | label | 🔗️detailing/🦀️.rs:133 |
| `bw` | all | number mm (Input bw) | number mm ex 300 | Web width b_w | label | ✂️shear-torsion/🦀️.rs:121 |
| `c` | all | number mm (Input c) | number mm ex 35 | Concrete cover c to the bars | label | 🪟️serviceability/🦀️.rs:193; ✂️shear-torsion/🦀️.rs:308 |
| `c` | DT-12 | number mm (Input c) | number mm ex 400 | Column dimension c | label | 🔗️detailing/🦀️.rs:272 |
| `c` | A-10 | number mm (Input c) | number 1 ex 9.87 | Curvature distribution factor c (π² for a sinusoidal distribution) | unit/kind | 🏛️analysis-models/🦀️.rs:131 |
| `c0` | all | number mm (Input c0) | number 1 ex 8 | Moment distribution factor c_0 (8 for a constant first-order moment) | unit/kind | 🏛️analysis-models/🦀️.rs:149 |
| `c1` | all | number mm (Input c1) | number mm ex 400 | Column dimension c_1 (column face to the free slab edge direction) | label | 🕳️punching-and-struts/🦀️.rs:98; 🕳️punching-and-struts/🦀️.rs:138; 🕳️punching-and-struts/🦀️.rs:155 |
| `c1` | FA-06 | number mm (Input c1) | number mm ex 40 | Edge distance c_1 of the anchor | label | 🪝️fastenings/🦀️.rs:151 |
| `c1` | FA-09 | number mm (Input c1) | number mm ex 150 | Edge distance c_1 of the anchor | label | 🪝️fastenings/🦀️.rs:191 |
| `c2` | all | number mm (Input c2) | number mm ex 400 | Column dimension c_2 | label | 🕳️punching-and-struts/🦀️.rs:98; 🕳️punching-and-struts/🦀️.rs:155 |
| `c2_left` | all | number mm (Input c2_left) | number mm | Distance c_2 to the left side edge | label | 🪝️fastenings/🦀️.rs:191 |
| `c2_right` | all | number mm (Input c2_right) | number mm | Distance c_2 to the right side edge | label | 🪝️fastenings/🦀️.rs:191 |
| `c_provided` | all | number mm (Input c_provided) | number mm ex 45 | Provided nominal cover c_nom | label | 🧱️materials-and-time/🦀️.rs:402; 🧱️materials-and-time/🦀️.rs:417 |
| `capital_l` | all | number mm (Input capital_l) | number mm ex 0 | Projection l_H of the column capital | label | 🕳️punching-and-struts/🦀️.rs:98 |
| `cd` | all | number mm (Input cd) | number mm ex 45 | Cover or half bar distance c_d | label | 🔗️detailing/🦀️.rs:40; 🔗️detailing/🦀️.rs:66 |
| `cement` | all | text - (Input cement) | choice ex N | Cement class | unit/kind | 🧱️materials-and-time/🦀️.rs:196 |
| `checklist` | all | vector 1 (Input checklist (vector)) | vector 1 | Checklist items (1 = satisfied, 0 = open) | label | 🌉️bridges-and-liquid/🦀️.rs:142 |
| `chi` | all | number 1 (Input chi) | number 1 ex 0.8 | Ageing coefficient χ | label | 🧩️annex-parts/🦀️.rs:117 |
| `circular` | all | number 1 (Input circular) | number 1 ex 0 | Circular column (1 = yes) | label | 🕳️punching-and-struts/🦀️.rs:98 |
| `class` | all | number 1 (Input class) | number 1 ex 1 | Tightness class (0 to 3) | label | 🌉️bridges-and-liquid/🦀️.rs:83 |
| `classes` | all | text - (Input classes) | text | Exposure classes, comma separated (e.g. XC1,XC3) | unit/kind | 🧱️materials-and-time/🦀️.rs:300; 🧱️materials-and-time/🦀️.rs:357 |
| `clear` | all | number mm (Input clear) | number mm ex 60 | Clear distance between bars | label | 🔗️detailing/🦀️.rs:127 |
| `clear_distance` | all | number mm (Input clear_distance) | number mm ex 60 | Clear distance of the lapped bars | label | 🔗️detailing/🦀️.rs:66 |
| `confinement` | all | number 1 (Input confinement) | number 1 ex 1 | Confinement factor of the node (1 to 3) | label | 🕳️punching-and-struts/🦀️.rs:289 |
| `cot_alpha` | all | number 1 (Input cot_alpha) | number 1 ex 0 | cot α of the inclined links | label | 🔗️detailing/🦀️.rs:142 |
| `cot_theta` | all | number 1 (Input cot_theta) | number 1 ex 2 | Strut inclination cot θ | label | ✂️shear-torsion/🦀️.rs:160; ✂️shear-torsion/🦀️.rs:329 |
| `cot_theta_f` | all | number 1 (Input cot_theta_f) | number 1 ex 1.2 | Flange strut inclination cot θ_f | label | ✂️shear-torsion/🦀️.rs:224 |
| `cracked` | all | number 1 (Input cracked) | number 1 ex 1 | Cracked concrete (1 = yes) | label | 🪝️fastenings/🦀️.rs:83 |
| `cycles` | all | vector 1 (Input cycles (vector)) | vector 1 | Numbers of cycles n_i | label | 🧵️prestress-and-fatigue/🦀️.rs:160 |
| `d` | all | number mm (Input d) | number mm ex 450 | Effective depth d | label | ✂️shear-torsion/🦀️.rs:140 |
| `d2` | all | number mm (Input d2) | number mm ex 50 | Distance d_2 of the compression reinforcement from the compressed edge | label | 🪟️serviceability/🦀️.rs:15; 📐️section-resistance/🦀️.rs:476 |
| `d_f` | all | number mm (Input d_f) | number mm ex 250 | Effective depth d_f of the flange | label | 🧩️annex-parts/🦀️.rs:96 |
| `d_nom` | all | number mm (Input d_nom) | number mm ex 16 | Nominal anchor diameter d_nom | label | 🪝️fastenings/🦀️.rs:191 |
| `dc_add` | all | number mm (Input dc_add) | number mm ex 0 | Reduction Δc_dur,add for additional protection | label | 🧱️materials-and-time/🦀️.rs:384 |
| `dc_dev` | all | number mm (Input dc_dev) | number mm ex 10 | Construction tolerance Δc_dev | label | 🧱️materials-and-time/🦀️.rs:402 |
| `dc_gamma` | all | number mm (Input dc_gamma) | number mm ex 0 | Safety margin Δc_dur,γ | label | 🧱️materials-and-time/🦀️.rs:384 |
| `dc_st` | all | number mm (Input dc_st) | number mm ex 0 | Reduction Δc_dur,st for stainless steel | label | 🧱️materials-and-time/🦀️.rs:384 |
| `decompression` | all | number 1 (Input decompression) | number 1 ex 0 | Decompression required (bonded tendons, XD or XS) (1 = yes) | label | 🪟️serviceability/🦀️.rs:326 |
| `delta` | all | number 1 (Input delta) | number 1 ex 0.8 | Ratio δ of the redistributed to the elastic moment | label | 🏛️analysis-models/🦀️.rs:40 |
| `delta_fd` | all | number 1 (Input delta_fd) | number N ex 100000 | Change ΔF_d of the flange force | unit/kind | ✂️shear-torsion/🦀️.rs:224; ✂️shear-torsion/🦀️.rs:378 |
| `delta_n_ed` | all | number N (Input delta_n_ed) | number N ex 5000 | Design tension force range ΔN_Ed | label | 🪝️fastenings/🦀️.rs:245 |
| `delta_n_rk_s_0` | all | number N (Input delta_n_rk_s_0) | number N ex 40000 | Characteristic fatigue resistance ΔN_Rk,s,0 of the anchor | label | 🪝️fastenings/🦀️.rs:245 |
| `delta_sigma` | all | number N/mm² (Input delta_sigma) | number N/mm² ex 30 | Stress increase Δσ_p of the unbonded tendon at ULS | label | 🧵️prestress-and-fatigue/🦀️.rs:84 |
| `delta_sigma_71` | all | number N/mm² (Input delta_sigma_71) | number N/mm² ex 162.5 | Stress range Δσ_s,71 under load model LM71 | label | 🧵️prestress-and-fatigue/🦀️.rs:199 |
| `delta_sigma_c` | all | number N/mm² (Input delta_sigma_c) | number N/mm² ex 3 | Concrete stress change Δσ_c at the tendon level | label | 🧵️prestress-and-fatigue/🦀️.rs:37 |
| `delta_sigma_equ` | all | number N/mm² (Input delta_sigma_equ) | number N/mm² ex 120 | Damage-equivalent stress range Δσ_s,equ | label | 🧵️prestress-and-fatigue/🦀️.rs:160 |
| `delta_sigma_tendon` | all | number N/mm² (Input delta_sigma_tendon) | number N/mm² ex 80 | Tendon stress range Δσ_p | label | 🌉️bridges-and-liquid/🦀️.rs:37 |
| `delta_t` | all | number K (Input delta_t) | number K ex 20 | Temperature change ΔT of the member | label | 🌉️bridges-and-liquid/🦀️.rs:101 |
| `delta_v` | all | number 1 (Input delta_v) | number N | Load reduction ΔV_Ed inside the control perimeter | unit/kind | 🕳️punching-and-struts/🦀️.rs:155 |
| `delta_x` | all | number mm (Input delta_x) | number mm ex 1000 | Length Δx of the change of flange force | label | ✂️shear-torsion/🦀️.rs:224; ✂️shear-torsion/🦀️.rs:378 |
| `dense_reinforcement` | all | number 1 (Input dense_reinforcement) | number 1 ex 0 | Dense reinforcement (spacing at most 150 mm) (1 = yes) | label | 🪝️fastenings/🦀️.rs:94 |
| `design_life` | all | number a (Input design_life) | number a ex 50 | Design working life | label | 🧱️materials-and-time/🦀️.rs:402; 🧱️materials-and-time/🦀️.rs:423 |
| `dg` | all | number mm (Input dg) | number mm ex 16 | Maximum aggregate size d_g | label | 🧱️materials-and-time/🦀️.rs:384; 🔗️detailing/🦀️.rs:127 |
| `dh` | all | number mm (Input dh) | number mm ex 30 | Anchor head diameter d_h | label | 🪝️fastenings/🦀️.rs:124; 🪝️fastenings/🦀️.rs:151 |
| `dim` | all | number mm (Input dim) | number mm ex 300 | Section dimension (width b_min or slab thickness h_s) | label | 🔥️fire-design/🦀️.rs:79 |
| `direct_support` | all | number 1 (Input direct_support) | number 1 ex 1 | Direct support (1 = yes) | label | ✂️shear-torsion/🦀️.rs:131 |
| `duration` | all | text - (Input duration) | choice ex long | Load duration | unit/kind | 🪟️serviceability/🦀️.rs:193 |
| `d` | FA-04,FA-06 | number mm (Input d) | number mm ex 16 | Anchor shaft diameter d | label | 🪝️fastenings/🦀️.rs:124; 🪝️fastenings/🦀️.rs:151 |
| `e0` | all | number mm (Input e0) | number mm ex 20 | Eccentricity e_0 of the first-order load | label | 🏛️analysis-models/🦀️.rs:131; 🏛️analysis-models/🦀️.rs:188; 🔥️fire-design/🦀️.rs:179 |
| `e1` | all | number mm (Input e1) | number mm ex 100 | Clear distance e_1 from the column face to the first free slab edge | label | 🕳️punching-and-struts/🦀️.rs:98; 🕳️punching-and-struts/🦀️.rs:21 |
| `e2` | all | number mm (Input e2) | number mm ex 100 | Clear distance e_2 from the column face to the second free slab edge | label | 🕳️punching-and-struts/🦀️.rs:98; 🕳️punching-and-struts/🦀️.rs:21 |
| `e_d` | all | number mm (Input e_d) | number N ex 1000000 | Design value E_d of the action effect | unit/kind | 🏛️analysis-models/🦀️.rs:74 |
| `e_nx` | all | number mm (Input e_nx) | number mm ex 0 | Eccentricity e_N,x of the tension load | label | 🪝️fastenings/🦀️.rs:94 |
| `e_ny` | all | number mm (Input e_ny) | number mm ex 0 | Eccentricity e_N,y of the tension load | label | 🪝️fastenings/🦀️.rs:94 |
| `e_p` | all | number N/mm² (Input e_p) | number mm ex 100 | Eccentricity e_p of the prestress (positive below the centroid) | unit/kind | 🧱️materials-and-time/🦀️.rs:285; 🪟️serviceability/🦀️.rs:326; 🌉️bridges-and-liquid/🦀️.rs:59 |
| `e_par` | all | number mm (Input e_par) | number mm ex 100 | Load eccentricity e_par parallel to the slab edge | label | 🕳️punching-and-struts/🦀️.rs:116 |
| `e_tot` | all | number mm (Input e_tot) | number mm ex 50 | Total eccentricity e_tot of the load | label | 📐️section-resistance/🦀️.rs:608 |
| `e_v` | all | number mm (Input e_v) | number mm ex 0 | Eccentricity e_V of the shear load | label | 🪝️fastenings/🦀️.rs:191 |
| `ecm` | all | number N/mm² (Input ecm) | number N/mm² ex 33000 | Secant modulus E_cm | label | 🧵️prestress-and-fatigue/🦀️.rs:69 |
| `ecm_t` | all | number N/mm² (Input ecm_t) | number N/mm² ex 32000 | Secant modulus E_cm(t) at transfer | label | 🧵️prestress-and-fatigue/🦀️.rs:37 |
| `edge_reinforcement` | all | text - (Input edge_reinforcement) | choice ex none | Edge reinforcement of the concrete edge | unit/kind | 🪝️fastenings/🦀️.rs:191 |
| `ei` | all | number 1 (Input ei) | number mm ex 20 | Eccentricity e_i from imperfections | unit/kind | 🏛️analysis-models/🦀️.rs:131 |
| `ep` | all | number N/mm² (Input ep) | number N/mm² ex 195000 | Modulus E_p of the prestressing steel | label | 🧵️prestress-and-fatigue/🦀️.rs:37; 🧱️materials-and-time/🦀️.rs:268 |
| `eps` | all | number 1 (Input eps) | number 1 ex 2 | Strain ε in permille | label | 🧱️materials-and-time/🦀️.rs:213; 🧱️materials-and-time/🦀️.rs:227; 🧱️materials-and-time/🦀️.rs:252; 🧱️materials-and-time/🦀️.rs:268 |
| `eps1` | all | number 1 (Input eps1) | number 1 ex 0.5 | Larger edge strain ε_1 of the tension zone | label | 🪟️serviceability/🦀️.rs:193 |
| `eps2` | all | number 1 (Input eps2) | number 1 ex 0.25 | Smaller edge strain ε_2 of the tension zone | label | 🪟️serviceability/🦀️.rs:193 |
| `eps_ca` | all | number 1 (Input eps_ca) | number 1 ex 5e-05 | Autogenous shrinkage strain ε_ca | label | 🌉️bridges-and-liquid/🦀️.rs:101 |
| `eps_cd` | all | number 1 (Input eps_cd) | number 1 ex 0.0001 | Drying shrinkage strain ε_cd | label | 🌉️bridges-and-liquid/🦀️.rs:101 |
| `eps_cs` | all | number 1 (Input eps_cs) | number 1 ex 0.0003 | Shrinkage strain ε_cs (absolute, positive) | label | 🧵️prestress-and-fatigue/🦀️.rs:69; 🪟️serviceability/🦀️.rs:280 |
| `eps_free` | all | number 1 (Input eps_free) | number 1 ex 0.0002 | Free strain ε_free from shrinkage and temperature | label | 🌉️bridges-and-liquid/🦀️.rs:101 |
| `eps_ud` | all | number 1 (Input eps_ud) | number 1 ex 20 | Ultimate strain ε_ud of the prestressing steel in permille | label | 🧱️materials-and-time/🦀️.rs:268 |
| `eps_ud_p` | all | number 1 (Input eps_ud_p) | number 1 ex 20 | Ultimate strain ε_ud of the prestressing steel in permille | label | 📐️section-resistance/🦀️.rs:406 |
| `eps_uk` | all | number 1 (Input eps_uk) | number 1 ex 50 | Characteristic strain ε_uk at maximum load in permille | label | 🧱️materials-and-time/🦀️.rs:252; 📐️section-resistance/🦀️.rs:406 |
| `es` | all | number N/mm² (Input es) | number N/mm² ex 200000 | Modulus E_s of the reinforcing steel | label | 🧱️materials-and-time/🦀️.rs:252 |
| `exposure` | all | text - (Input exposure) | choice ex XC4 | Exposure class | unit/kind | 🧱️materials-and-time/🦀️.rs:300; 🌉️bridges-and-liquid/🦀️.rs:12 |
| `f_bt` | all | number N (Input f_bt) | number N ex 20000 | Tensile force F_bt of the bar at the bend | label | 🔗️detailing/🦀️.rs:284 |
| `f_cr` | all | number N (Input f_cr) | number N ex 100000 | Tensile force F_cr of the flange before cracking | label | 🪟️serviceability/🦀️.rs:85 |
| `f_ed` | all | number N (Input f_ed) | number N ex 300000 | Design force F_Ed | label | 🕳️punching-and-struts/🦀️.rs:356; 🧩️annex-parts/🦀️.rs:73 |
| `f_td` | all | number N (Input f_td) | number N ex 150000 | Tie force F_td | label | 🕳️punching-and-struts/🦀️.rs:281 |
| `f_v_ed` | all | number N (Input f_v_ed) | number N ex 100000 | Total vertical load F_V,Ed | label | 🏛️analysis-models/🦀️.rs:224 |
| `faces` | all | text - (Input faces) | text | Fire-exposed faces as letters l, r, b, t (e.g. blr) | unit/kind | 🔥️fire-design/🦀️.rs:112 |
| `fatigue` | all | number 1 (Input fatigue) | number 1 ex 0 | Fatigue loading (neglects cohesion) (1 = yes) | label | ✂️shear-torsion/🦀️.rs:206 |
| `fbd` | all | number N/mm² (Input fbd) | number N/mm² ex 3 | Ultimate bond stress f_bd | label | 🔗️detailing/🦀️.rs:22 |
| `fck` | all | number N/mm² (Input fck) | number N/mm² ex 30 | Characteristic cylinder strength f_ck | label | 🧱️materials-and-time/🦀️.rs:147; 🧱️materials-and-time/🦀️.rs:24 |
| `fck_t` | all | number N/mm² (Input fck_t) | number N/mm² ex 20 | Characteristic strength f_ck(t) at transfer | label | 🧵️prestress-and-fatigue/🦀️.rs:112 |
| `fct_eff` | all | number N/mm² (Input fct_eff) | number N/mm² ex 2.9 | Effective tensile strength f_ct,eff | label | 🪟️serviceability/🦀️.rs:85; 🌉️bridges-and-liquid/🦀️.rs:47 |
| `fctd` | all | number N/mm² (Input fctd) | number N/mm² ex 1.2 | Design tensile strength f_ctd | label | ✂️shear-torsion/🦀️.rs:140 |
| `fctd_t` | all | number N/mm² (Input fctd_t) | number N/mm² ex 1.1 | Design tensile strength f_ctd(t) at release | label | 🔗️detailing/🦀️.rs:99 |
| `fp01k` | all | number N/mm² (Input fp01k) | number N/mm² ex 1640 | 0.1 % proof stress f_p0.1k of the prestressing steel | label | 🧵️prestress-and-fatigue/🦀️.rs:21; 🧱️materials-and-time/🦀️.rs:268 |
| `fpk` | all | number N/mm² (Input fpk) | number N/mm² ex 1860 | Tensile strength f_pk of the prestressing steel | label | 🧵️prestress-and-fatigue/🦀️.rs:21; 🧱️materials-and-time/🦀️.rs:268 |
| `fuk` | all | number N/mm² (Input fuk) | number N/mm² ex 800 | Characteristic tensile strength f_uk of the anchor steel | label | 🪝️fastenings/🦀️.rs:74; 🪝️fastenings/🦀️.rs:69 |
| `full_discontinuity` | all | number 1 (Input full_discontinuity) | number 1 ex 0 | Fully discontinuous load distribution (1 = yes) | label | 🕳️punching-and-struts/🦀️.rs:356 |
| `fyk` | all | number N/mm² (Input fyk) | number N/mm² ex 500 | Characteristic yield strength f_yk of the reinforcement | label | 🧱️materials-and-time/🦀️.rs:252; 🧱️materials-and-time/🦀️.rs:65 |
| `fyk` | FA-02,FA-07 | number N/mm² (Input fyk) | number N/mm² ex 500 | Characteristic yield strength f_yk of the anchor steel | label | 🪝️fastenings/🦀️.rs:74; 🪝️fastenings/🦀️.rs:69 |
| `gamma_f_fat` | all | number 1 (Input gamma_f_fat) | number 1 ex 1 | Partial factor γ_F,fat of the fatigue load | label | 🧵️prestress-and-fatigue/🦀️.rs:160 |
| `gamma_g` | all | number 1 (Input gamma_g) | number 1 ex 1.35 | Partial factor γ_G of the permanent action | label | 🔥️fire-design/🦀️.rs:204 |
| `gamma_inst` | all | number 1 (Input gamma_inst) | number 1 ex 1.2 | Installation safety factor γ_inst | label | 🪝️fastenings/🦀️.rs:89 |
| `gamma_q` | all | number 1 (Input gamma_q) | number 1 ex 1.5 | Partial factor γ_Q of the variable action | label | 🔥️fire-design/🦀️.rs:204 |
| `gk` | all | number 1 (Input gk) | number N ex 200000 | Characteristic permanent action effect G_k | unit/kind | 🔥️fire-design/🦀️.rs:204 |
| `ground` | all | text - (Input ground) | choice ex none | Ground below the member | unit/kind | 🧱️materials-and-time/🦀️.rs:417 |
| `grout` | all | number 1 (Input grout) | number 1 ex 0 | Grout layer under the base plate (1 = yes) | label | 🪝️fastenings/🦀️.rs:164 |
| `h` | all | number mm (Input h) | number mm ex 500 | Section height h | label | 🪟️serviceability/🦀️.rs:15 |
| `h0` | all | number mm (Input h0) | number mm ex 187.5 | Notional size h_0 = 2 A_c/u | label | 🧱️materials-and-time/🦀️.rs:480; 🧱️materials-and-time/🦀️.rs:516 |
| `h_ct` | all | number mm (Input h_ct) | number mm ex 100 | Height h_ct of the external tension zone | label | 🧩️annex-parts/🦀️.rs:34 |
| `h_ed` | all | number mm (Input h_ed) | number N ex 20000 | Horizontal force H_Ed on the corbel | unit/kind | 🧩️annex-parts/🦀️.rs:73 |
| `h_member` | all | number mm (Input h_member) | number mm ex 500 | Overall height of the member h (for the size factor k) | label | 🪟️serviceability/🦀️.rs:85 |
| `h_min` | all | number mm (Input h_min) | number mm ex 150 | Minimum member thickness h_min of the anchor | label | 🪝️fastenings/🦀️.rs:135 |
| `h_s` | all | number mm (Input h_s) | number mm ex 400 | Distance h_s between the flange centroids | label | 🧩️annex-parts/🦀️.rs:96 |
| `hcr` | all | number mm (Input hcr) | number mm ex 250 | Height h_cr of the section tension zone | label | 🪟️serviceability/🦀️.rs:124 |
| `hd_over_h` | all | number 1 (Input hd_over_h) | number 1 ex 10 | Ratio h_D/h of the hydrostatic head to the wall thickness | label | 🌉️bridges-and-liquid/🦀️.rs:83 |
| `hef` | all | number mm (Input hef) | number mm ex 100 | Embedment depth h_ef | label | 🪝️fastenings/🦀️.rs:94 |
| `hef_eff` | all | number mm (Input hef_eff) | number mm | Effective embedment depth h_ef for the cone failure | label | 🪝️fastenings/🦀️.rs:94 |
| `hf` | all | number mm (Input hf) | number mm ex 150 | Flange thickness h_f | label | ✂️shear-torsion/🦀️.rs:224; 📐️section-resistance/🦀️.rs:581 |
| `hw` | all | number mm (Input hw) | number mm ex 200 | Wall thickness h_w | label | 📐️section-resistance/🦀️.rs:608 |
| `i` | all | number 1 (Input i) | number mm | Radius of gyration i of the column section | unit/kind | 🏛️analysis-models/🦀️.rs:88 |
| `i_b` | all | number 1 (Input i_b) | number mm⁴ | Second moment of area I of the uncracked section | unit/kind | ✂️shear-torsion/🦀️.rs:140 |
| `ic` | all | number mm⁴ (Input ic) | number mm⁴ ex 6750000000.0 | Second moment of area I_c of the concrete section | label | 🧵️prestress-and-fatigue/🦀️.rs:69 |
| `imposed_only` | all | number 1 (Input imposed_only) | number 1 ex 0 | Stress caused by imposed deformation only (1 = yes) | label | 🪟️serviceability/🦀️.rs:39 |
| `inclined` | all | number 1 (Input inclined) | number 1 ex 0 | Inclined top branch of the steel diagram (1 = yes) | label | 🧱️materials-and-time/🦀️.rs:252 |
| `is` | all | number 1 (Input is) | number mm ex 150 | Radius of gyration i_s of the reinforcement | unit/kind | 🏛️analysis-models/🦀️.rs:131 |
| `is_moment` | all | number 1 (Input is_moment) | number mm⁴ ex 63000000.0 | Second moment of area I_s of the reinforcement | unit/kind | 🏛️analysis-models/🦀️.rs:149 |
| `isolated` | all | number 1 (Input isolated) | number 1 ex 0 | Isolated column (1 = yes) | label | 🏛️analysis-models/🦀️.rs:88 |
| `k` | M-08,S-11 | number 1 (Input k) | number 1 ex 1.08 | Ratio k = f_t/f_y of the steel diagram | label | 🧱️materials-and-time/🦀️.rs:252; 📐️section-resistance/🦀️.rs:406 |
| `k` | PR-04 | number 1 (Input k) | number 1/m ex 0.005 | Unintentional angular displacement k of the duct [rad/m] | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:51; 🧵️prestress-and-fatigue/🦀️.rs:6 |
| `k1` | A-09 | number 1 (Input k1) | number 1 ex 0.1 | Relative stiffness k_1 of the restraint at end 1 | label | 🏛️analysis-models/🦀️.rs:88; 🏛️analysis-models/🦀️.rs:80 |
| `k1` | SL-06 | number 1 (Input k1) | number 1 ex 0.8 | Bond coefficient k_1 (0.8 for high-bond bars) | label | 🪟️serviceability/🦀️.rs:193 |
| `k2` | all | number 1 (Input k2) | number 1 ex 0.1 | Relative stiffness k_2 of the restraint at end 2 | label | 🏛️analysis-models/🦀️.rs:88; 🏛️analysis-models/🦀️.rs:80 |
| `k_conf` | all | number 1 (Input k_conf) | number 1 ex 0.1 | Confinement coefficient K of the transverse bars | label | 🔗️detailing/🦀️.rs:40 |
| `kappa` | all | vector 1 (Input kappa (vector)) | vector 1/m | Curvatures κ to evaluate | unit/kind | 📐️section-resistance/🦀️.rs:646 |
| `kappa_points` | all | number 1 (Input kappa_points) | number 1 ex 28 | Number of curvature points | label | 🏛️analysis-models/🦀️.rs:188 |
| `kc` | all | number 1 (Input kc) | number 1 ex 0.4 | Coefficient k_c of the stress distribution | label | 🪟️serviceability/🦀️.rs:124 |
| `kind` | F-03,F-04,F-07,BR-04 | text - (Input kind) | choice ex straight | S-N curve of the steel | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:119 |
| `kind` | R-01,D-06 | text - (Input kind) | choice ex beam | Member kind | unit/kind | 🔥️fire-design/🦀️.rs:36 |
| `kind` | DT-12 | text - (Input kind) | choice ex strip | Foundation kind | unit/kind | 🔗️detailing/🦀️.rs:272 |
| `kind` | ST-01 | text - (Input kind) | choice ex free | Strut kind | unit/kind | 🕳️punching-and-struts/🦀️.rs:267 |
| `ks` | all | number 1 (Input ks) | number 1 ex 0.8 | Reduction factor k_s(θ) of the steel strength in fire | label | 🔥️fire-design/🦀️.rs:179 |
| `l` | all | number mm (Input l) | number mm ex 6000 | Length or clear height l | label | 🏛️analysis-models/🦀️.rs:88; 🏛️analysis-models/🦀️.rs:224; 🧵️prestress-and-fatigue/🦀️.rs:98; 🌉️bridges-and-liquid/🦀️.rs:59 |
| `l0` | all | number mm (Input l0) | number mm ex 6000 | Effective length l_0 | label | 🏛️analysis-models/🦀️.rs:23; 🏛️analysis-models/🦀️.rs:88; 🏛️analysis-models/🦀️.rs:131; 🏛️analysis-models/🦀️.rs:149; 🏛️analysis-models/🦀️.rs:188; 📐️section-resistance/🦀️.rs:608 |
| `l0_fi` | all | number mm (Input l0_fi) | number mm ex 3000 | Effective length l_0,fi in fire | label | 🔥️fire-design/🦀️.rs:179 |
| `l0t` | all | number mm (Input l0t) | number mm ex 6000 | Unrestrained length l_0t of the compression flange | label | 🏛️analysis-models/🦀️.rs:231 |
| `l_available` | all | number mm (Input l_available) | number mm ex 800 | Available length l_available | label | 🔗️detailing/🦀️.rs:40; 🔗️detailing/🦀️.rs:66 |
| `l_bd` | all | number mm (Input l_bd) | number mm ex 600 | Required anchorage length l_bd | label | 🪝️fastenings/🦀️.rs:256 |
| `l_m` | all | number m (Input l_m) | number m ex 6 | Height l of the member in metres | label | 🏛️analysis-models/🦀️.rs:6 |
| `l_s` | all | number mm (Input l_s) | number mm ex 500 | Available anchorage length l_s in the cone | label | 🪝️fastenings/🦀️.rs:256 |
| `lambda1` | all | number 1 (Input lambda1) | number 1 ex 0.8 | Damage factor λ_s,1 | label | 🧵️prestress-and-fatigue/🦀️.rs:199 |
| `lambda2` | all | number 1 (Input lambda2) | number 1 ex 1 | Traffic volume factor λ_s,2 | label | 🧵️prestress-and-fatigue/🦀️.rs:199 |
| `lambda3` | all | number 1 (Input lambda3) | number 1 ex 1 | Design life factor λ_s,3 | label | 🧵️prestress-and-fatigue/🦀️.rs:199 |
| `lambda4` | all | number 1 (Input lambda4) | number 1 ex 1 | Multiple lanes factor λ_s,4 | label | 🧵️prestress-and-fatigue/🦀️.rs:199 |
| `lambda_max` | all | number 1 (Input lambda_max) | number 1 ex 1.4 | Upper limit λ_s,max | label | 🧵️prestress-and-fatigue/🦀️.rs:199 |
| `lambda_shear` | all | number 1 (Input lambda_shear) | number 1 ex 3 | Shear slenderness λ = a_v/d | label | 🏛️analysis-models/🦀️.rs:55 |
| `lambda_y` | all | number 1 (Input lambda_y) | number 1 ex 40 | Slenderness λ_y about the y axis | label | 🏛️analysis-models/🦀️.rs:88 |
| `lambda_z` | all | number 1 (Input lambda_z) | number 1 ex 40 | Slenderness λ_z about the z axis | label | 🏛️analysis-models/🦀️.rs:88 |
| `lever` | all | number 1 (Input lever) | number mm | Lever arm l_a of the anchor shear load | unit/kind | 🪝️fastenings/🦀️.rs:164 |
| `li` | all | number 1 (Input li) | number m ex 6 | Span l_i for the tie force | unit/kind | 🔗️detailing/🦀️.rs:294 |
| `load_x` | all | vector 1 (Input load_x (vector)) | vector N | Node loads in x direction | unit/kind | 🕳️punching-and-struts/🦀️.rs:334 |
| `load_y` | all | vector 1 (Input load_y (vector)) | vector N | Node loads in y direction | unit/kind | 🕳️punching-and-struts/🦀️.rs:334 |
| `loading` | all | text - (Input loading) | choice ex tension | Loading of the bar | unit/kind | 🔗️detailing/🦀️.rs:40; 🔗️detailing/🦀️.rs:66 |
| `m` | A-03 | number 1 (Input m) | number 1 ex 1 | Number m of vertical members carrying the horizontal load | label | 🏛️analysis-models/🦀️.rs:6 |
| `m` | A-05 | number 1 (Input m) | number N·mm ex 150000000 | Bending moment M for the cracked stiffness | unit/kind | 🏛️analysis-models/🦀️.rs:31 |
| `m1` | all | number N·mm (Input m1) | number N·mm ex 100000000 | Moment M_1 before the system change | label | 🧩️annex-parts/🦀️.rs:117 |
| `m2` | all | number N·mm (Input m2) | number N·mm ex 200000000 | Moment M_2 after the system change | label | 🧩️annex-parts/🦀️.rs:117 |
| `m_0ed` | all | number N·mm (Input m_0ed) | number N·mm ex 100000000 | First-order moment M_0Ed | label | 🏛️analysis-models/🦀️.rs:149 |
| `m_char` | all | number N·mm (Input m_char) | number N·mm ex 180000000 | Moment M in the characteristic combination | label | 🪟️serviceability/🦀️.rs:22 |
| `m_ed` | all | number N·mm (Input m_ed) | number N·mm ex 250000000 | Design moment M_Ed | label | 🪟️serviceability/🦀️.rs:58; 📐️section-resistance/🦀️.rs:476 |
| `m_ed_fi` | all | number N·mm (Input m_ed_fi) | number N·mm ex 100000000 | Design moment M_Ed,fi in fire | label | 🔥️fire-design/🦀️.rs:145 |
| `m_edge` | all | number N·mm (Input m_edge) | number N·mm ex 50000000 | Moment M_Ed at the edge column | label | 🔗️detailing/🦀️.rs:212 |
| `m_edy` | all | number N·mm (Input m_edy) | number N·mm ex 150000000 | Design moment M_Edy about the y axis | label | 📐️section-resistance/🦀️.rs:555; 📐️section-resistance/🦀️.rs:563 |
| `m_edz` | all | number N·mm (Input m_edz) | number N·mm ex 120000000 | Design moment M_Edz about the z axis | label | 📐️section-resistance/🦀️.rs:555; 📐️section-resistance/🦀️.rs:563 |
| `m_max` | all | number N·mm (Input m_max) | number N·mm ex 200000000 | Maximum moment M_max of the load cycle | label | 🧵️prestress-and-fatigue/🦀️.rs:137 |
| `m_min` | all | number N·mm (Input m_min) | number N·mm ex 50000000 | Minimum moment M_min of the load cycle | label | 🧵️prestress-and-fatigue/🦀️.rs:137 |
| `m_qp` | all | number N·mm (Input m_qp) | number N·mm ex 150000000 | Moment M in the quasi-permanent combination | label | 🪟️serviceability/🦀️.rs:22; 🪟️serviceability/🦀️.rs:193 |
| `m_rdy` | all | number N·mm (Input m_rdy) | number N·mm ex 235600000 | Resisting moment M_Rdy about the y axis | label | 📐️section-resistance/🦀️.rs:555 |
| `m_rdz` | all | number N·mm (Input m_rdz) | number N·mm ex 235600000 | Resisting moment M_Rdz about the z axis | label | 📐️section-resistance/🦀️.rs:555 |
| `m_total` | all | number N·mm (Input m_total) | number N·mm ex 600000000 | Total prestress moment | label | 🌉️bridges-and-liquid/🦀️.rs:59 |
| `m_x` | all | number N·mm (Input m_x) | number N·mm ex 2000000 | Moment M_x on the anchor group | label | 🪝️fastenings/🦀️.rs:45 |
| `m_y` | all | number N·mm (Input m_y) | number N·mm ex 1000000 | Moment M_y on the anchor group | label | 🪝️fastenings/🦀️.rs:45 |
| `member_i` | all | vector 1 (Input member_i (vector)) | vector 1 | Start node index of each truss member | label | 🕳️punching-and-struts/🦀️.rs:334 |
| `member_j` | all | vector 1 (Input member_j (vector)) | vector 1 | End node index of each truss member | label | 🕳️punching-and-struts/🦀️.rs:334 |
| `method` | all | text - (Input method) | choice ex elastic | Load distribution method | unit/kind | 🪝️fastenings/🦀️.rs:45 |
| `mode` | all | text - (Input mode) | choice ex closing | Corner loading | unit/kind | 🧩️annex-parts/🦀️.rs:45 |
| `moisture` | all | number 1 (Input moisture) | number % ex 1.5 | Moisture content u of the concrete by mass | unit/kind | 🔥️fire-design/🦀️.rs:116; fire_field:solve |
| `mu` | all | number 1 (Input mu) | number 1 ex 0.19 | Friction coefficient μ of the duct [1/rad] | label | 🧵️prestress-and-fatigue/🦀️.rs:51 |
| `mu_fi` | all | number 1 (Input mu_fi) | number 1 ex 0.7 | Load level μ_fi in fire | label | 🔥️fire-design/🦀️.rs:79 |
| `mx` | all | number N·mm (Input mx) | number N ex 20000 | Plate moment m_x per unit width [N·mm/mm] | unit/kind | 🧩️annex-parts/🦀️.rs:132 |
| `mxy` | all | number N·mm (Input mxy) | number N ex 5000 | Plate twisting moment m_xy per unit width [N·mm/mm] | unit/kind | 🧩️annex-parts/🦀️.rs:132 |
| `my` | all | number N·mm (Input my) | number N ex 20000 | Plate moment m_y per unit width [N·mm/mm] | unit/kind | 🧩️annex-parts/🦀️.rs:132 |
| `n_bars` | all | number 1 (Input n_bars) | number 1 ex 4 | Number of bars in the corner tie | label | 🧩️annex-parts/🦀️.rs:45 |
| `n_ed` | all | number N (Input n_ed) | number N ex 500000 | Axial force N_Ed (compression positive) | label | 🪟️serviceability/🦀️.rs:22; 📐️section-resistance/🦀️.rs:458; 🏛️analysis-models/🦀️.rs:88 |
| `n_ed` | DT-02 | number N (Input n_ed) | number N | Axial force N_Ed (tension positive) | label | 🔗️detailing/🦀️.rs:142 |
| `n_ed` | FA-01,FA-02,FA-03,FA-04,FA-05,FA-06,FA-07,FA-10 | number N (Input n_ed) | number N ex 20000 | Tension load N_Ed (tension positive) | label | 🪝️fastenings/🦀️.rs:45; 🪝️fastenings/🦀️.rs:74; 🪝️fastenings/🦀️.rs:114; 🪝️fastenings/🦀️.rs:124; 🪝️fastenings/🦀️.rs:135; 🪝️fastenings/🦀️.rs:151; 🪝️fastenings/🦀️.rs:164; 🪝️fastenings/🦀️.rs:216 |
| `n_ed_fi` | all | number N (Input n_ed_fi) | number N ex 500000 | Axial force N_Ed,fi in fire | label | 🔥️fire-design/🦀️.rs:179 |
| `n_ed_re` | all | number N (Input n_ed_re) | number N ex 100000 | Force N_Ed,re on the anchor reinforcement | label | 🪝️fastenings/🦀️.rs:256 |
| `n_rd` | all | number N (Input n_rd) | number N ex 3574000 | Axial resistance N_Rd | label | 📐️section-resistance/🦀️.rs:555 |
| `n_rd_c` | all | number N (Input n_rd_c) | number N ex 100000 | Design resistance N_Rd,c (concrete cone) | label | 🪝️fastenings/🦀️.rs:216 |
| `n_rd_cb` | all | number N (Input n_rd_cb) | number N ex 100000 | Design resistance N_Rd,cb (blow-out) | label | 🪝️fastenings/🦀️.rs:216 |
| `n_rd_p` | all | number N (Input n_rd_p) | number N ex 100000 | Design resistance N_Rd,p (pull-out) | label | 🪝️fastenings/🦀️.rs:216 |
| `n_rd_s` | all | number N (Input n_rd_s) | number N ex 100000 | Design resistance N_Rd,s (steel) | label | 🪝️fastenings/🦀️.rs:216 |
| `n_rd_sp` | all | number N (Input n_rd_sp) | number N ex 100000 | Design resistance N_Rd,sp (splitting) | label | 🪝️fastenings/🦀️.rs:216 |
| `n_rk_p` | all | number N (Input n_rk_p) | number N ex 80000 | Characteristic pull-out resistance N_Rk,p | label | 🪝️fastenings/🦀️.rs:135 |
| `n_rk_p0` | all | number N (Input n_rk_p0) | number N ex 80000 | Basic pull-out resistance N_Rk,p0 at f_ck = 20 N/mm² | label | 🪝️fastenings/🦀️.rs:124 |
| `n_s` | all | number N (Input n_s) | number 1 ex 5 | Number of storeys n_s | unit/kind | 🏛️analysis-models/🦀️.rs:224 |
| `n_tendons` | all | number 1 (Input n_tendons) | number 1 ex 4 | Number n of tendons tensioned in sequence | label | 🧵️prestress-and-fatigue/🦀️.rs:37 |
| `narrowing` | all | number 1 (Input narrowing) | number 1 ex 0 | Narrowing compression zone (NA-DE) (1 = yes) | label | 🧱️materials-and-time/🦀️.rs:238 |
| `nb` | all | number 1 (Input nb) | number 1 ex 1 | Number n_b of bars in the bundle | label | 🧱️materials-and-time/🦀️.rs:384; 🔗️detailing/🦀️.rs:85 |
| `ni` | all | vector 1 (Input ni (vector)) | vector N | Tension forces N_i of the anchors | unit/kind | 🪝️fastenings/🦀️.rs:230 |
| `node` | all | text - (Input node) | choice ex CCC | Node kind | unit/kind | 🕳️punching-and-struts/🦀️.rs:289 |
| `node_x` | all | vector 1 (Input node_x (vector)) | vector mm | Node x coordinates | unit/kind | 🕳️punching-and-struts/🦀️.rs:334 |
| `node_y` | all | vector 1 (Input node_y (vector)) | vector mm | Node y coordinates | unit/kind | 🕳️punching-and-struts/🦀️.rs:334 |
| `nonlinear_creep_proven` | all | number 1 (Input nonlinear_creep_proven) | number 1 ex 0 | Non-linear creep taken into account (1 = yes) | label | 🧵️prestress-and-fatigue/🦀️.rs:112 |
| `nx` | all | number 1 (Input nx) | number N/mm ex 100 | Membrane force n_x per unit width | unit/kind | 🧩️annex-parts/🦀️.rs:132 |
| `nxy` | all | number 1 (Input nxy) | number N/mm ex 20 | Membrane shear force n_xy per unit width | unit/kind | 🧩️annex-parts/🦀️.rs:132 |
| `ny` | all | number 1 (Input ny) | number N/mm ex 50 | Membrane force n_y per unit width | unit/kind | 🧩️annex-parts/🦀️.rs:132 |
| `opening_a` | all | number 1 (Input opening_a) | number mm | Distance of the opening from the column centre | unit/kind | 🕳️punching-and-struts/🦀️.rs:55 |
| `opening_clear` | all | number 1 (Input opening_clear) | number mm | Clear distance of the opening from the column | unit/kind | 🕳️punching-and-struts/🦀️.rs:98 |
| `opening_w` | all | number 1 (Input opening_w) | number mm | Width of the opening | unit/kind | 🕳️punching-and-struts/🦀️.rs:55 |
| `p` | all | number N (Input p) | number N ex 500000 | Prestressing force P | label | 🧱️materials-and-time/🦀️.rs:285; 🧵️prestress-and-fatigue/🦀️.rs:98; 🪟️serviceability/🦀️.rs:326; 🌉️bridges-and-liquid/🦀️.rs:59 |
| `p_force` | all | number N (Input p_force) | number N ex 500000 | Prestressing force P of the tendons | label | 🕳️punching-and-struts/🦀️.rs:259 |
| `p_transverse` | all | number 1 (Input p_transverse) | number N/mm² ex 0 | Transverse pressure p across the anchorage | unit/kind | 🔗️detailing/🦀️.rs:40; 🔗️detailing/🦀️.rs:66 |
| `part` | all | text - (Input part) | choice ex web | Part of the section | unit/kind | 🪟️serviceability/🦀️.rs:85 |
| `phi` | all | number mm (Input phi) | number mm ex 20 | Bar diameter φ | label | 🔗️detailing/🦀️.rs:29; 🧱️materials-and-time/🦀️.rs:384 |
| `phi` | PR-05,AK-01 | number mm (Input phi) | number 1 ex 2 | Creep coefficient φ(t,t_0) | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:69; 🧩️annex-parts/🦀️.rs:117 |
| `phi_ef` | all | number 1 (Input phi_ef) | number 1 ex 2 | Effective creep ratio φ_ef | label | 🏛️analysis-models/🦀️.rs:88; 🏛️analysis-models/🦀️.rs:131; 🏛️analysis-models/🦀️.rs:149 |
| `phi_fat` | all | number 1 (Input phi_fat) | number 1 ex 1 | Dynamic factor Φ_fat | label | 🧵️prestress-and-fatigue/🦀️.rs:199 |
| `phi_inf` | all | number 1 (Input phi_inf) | number 1 ex 2.4 | Final creep coefficient φ(∞,t_0) | label | 🪟️serviceability/🦀️.rs:15; 🧱️materials-and-time/🦀️.rs:278 |
| `phi_l` | all | number mm (Input phi_l) | number mm ex 25 | Longitudinal bar diameter φ_l | label | ✂️shear-torsion/🦀️.rs:308 |
| `phi_m` | all | number m (Input phi_m) | number mm ex 200 | Mandrel diameter φ_m | unit/kind | 🔗️detailing/🦀️.rs:284; 🧩️annex-parts/🦀️.rs:45 |
| `phi_p` | all | number mm (Input phi_p) | number mm ex 12 | Prestressing steel diameter φ_p | label | 🧵️prestress-and-fatigue/🦀️.rs:137; 🪟️serviceability/🦀️.rs:193 |
| `phi_s` | all | number mm (Input phi_s) | number mm ex 16 | Reinforcing bar diameter φ_s | label | 🧵️prestress-and-fatigue/🦀️.rs:137 |
| `pm0` | all | number N (Input pm0) | number N ex 500000 | Initial prestress force P_m0 | label | 🧵️prestress-and-fatigue/🦀️.rs:29 |
| `pm_t` | all | number N (Input pm_t) | number N ex 450000 | Prestress force P_m,t after losses | label | 🧵️prestress-and-fatigue/🦀️.rs:91 |
| `pm_t_per_m` | all | number m (Input pm_t_per_m) | number N/mm ex 2500 | Prestress force P_m,t per unit height of the tank wall | unit/kind | 🌉️bridges-and-liquid/🦀️.rs:135 |
| `pmax` | all | number N (Input pmax) | number N ex 1000000 | Maximum tendon force P_max | label | 🧵️prestress-and-fatigue/🦀️.rs:51; 🧵️prestress-and-fatigue/🦀️.rs:104 |
| `point_y` | all | vector 1 (Input point_y (vector)) | vector mm | y coordinates of the temperature points | unit/kind | 🔥️fire-design/🦀️.rs:116 |
| `point_z` | all | vector 1 (Input point_z (vector)) | vector mm | z coordinates of the temperature points | unit/kind | 🔥️fire-design/🦀️.rs:116 |
| `points` | all | number 1 (Input points) | number 1 ex 24 | Number of points of the interaction curve | label | 📐️section-resistance/🦀️.rs:530; 📐️section-resistance/🦀️.rs:646 |
| `poly_y` | all | vector 1 (Input poly_y (vector)) | vector mm | Outline y coordinates | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `poly_z` | all | vector 1 (Input poly_z (vector)) | vector mm | Outline z coordinates | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `hole_y` | all | (none) | vector mm | Hole outline y coordinates | added | 📐️section-resistance/🦀️.rs:406; ✂️shear-torsion/🦀️.rs:308 |
| `hole_z` | all | (none) | vector mm | Hole outline z coordinates | added | 📐️section-resistance/🦀️.rs:406; ✂️shear-torsion/🦀️.rs:308 |
| `position` | all | text - (Input position) | choice ex interior | Column position | unit/kind | 🕳️punching-and-struts/🦀️.rs:16 |
| `precast` | all | number 1 (Input precast) | number 1 ex 0 | Precast element (1 = yes) | label | 🌉️bridges-and-liquid/🦀️.rs:12 |
| `pressure` | all | number 1 (Input pressure) | number N/mm² ex 0.15 | Liquid pressure p at the wall base | unit/kind | 🌉️bridges-and-liquid/🦀️.rs:135 |
| `prestressed` | all | number 1 (Input prestressed) | number 1 ex 0 | Prestressed member (1 = yes) | label | 🌉️bridges-and-liquid/🦀️.rs:5 |
| `prestressed_bonded` | all | number 1 (Input prestressed_bonded) | number 1 ex 0 | Prestressed with bonded tendons (1 = yes) | label | 🪟️serviceability/🦀️.rs:227 |
| `psi_fi` | all | number 1 (Input psi_fi) | number 1 ex 0.5 | Combination factor ψ_fi for the fire situation | label | 🔥️fire-design/🦀️.rs:204 |
| `psi_h_nb` | all | number 1 (Input psi_h_nb) | number 1 ex 1 | Factor ψ_h,nb for the member thickness | label | 🪝️fastenings/🦀️.rs:151 |
| `pure_tension` | all | number 1 (Input pure_tension) | number 1 ex 0 | Pure tension (1 = yes) | label | 🪟️serviceability/🦀️.rs:85 |
| `q` | all | number 1 (Input q) | number N/mm ex 20 | Uniformly distributed load q (N/mm = kN/m) | unit/kind | 🪟️serviceability/🦀️.rs:280 |
| `q_wheel` | all | number N (Input q_wheel) | number N ex 150000 | Wheel load Q_wheel | label | 🌉️bridges-and-liquid/🦀️.rs:66 |
| `quality` | all | number 1 (Input quality) | number 1 ex 0 | Production quality control ensured (1 = yes) | label | 🧱️materials-and-time/🦀️.rs:402 |
| `qk` | all | number 1 (Input qk) | number N ex 100000 | Characteristic variable action effect Q_k | unit/kind | 🔥️fire-design/🦀️.rs:204 |
| `r_class` | all | number min (Input r_class) | number min ex 90 | Fire resistance class R | label | 🔥️fire-design/🦀️.rs:79 |
| `r_m` | all | number m (Input r_m) | number N ex 1500000 | Mean resistance R_m | unit/kind | 🏛️analysis-models/🦀️.rs:74 |
| `radius` | all | number mm (Input radius) | number mm ex 15000 | Radius of curvature or of the tank wall | label | 🧵️prestress-and-fatigue/🦀️.rs:98; 🌉️bridges-and-liquid/🦀️.rs:135 |
| `radius_m` | all | number m (Input radius_m) | number m ex 5 | Radius of curvature r of the duct | label | 🧵️prestress-and-fatigue/🦀️.rs:51 |
| `ranges` | all | vector 1 (Input ranges (vector)) | vector N/mm² | Stress ranges Δσ_i | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:160 |
| `rax` | all | number 1 (Input rax) | number 1 ex 0.5 | Restraint degree R_ax | label | 🌉️bridges-and-liquid/🦀️.rs:101 |
| `relax_class` | all | number 1 (Input relax_class) | number 1 ex 2 | Relaxation class (1, 2 or 3) | label | 🧵️prestress-and-fatigue/🦀️.rs:69; 🧵️prestress-and-fatigue/🦀️.rs:11 |
| `release` | all | text - (Input release) | choice ex gradual | Release of the prestress | unit/kind | 🔗️detailing/🦀️.rs:99 |
| `restraint` | all | number 1 (Input restraint) | number 1 ex 0 | Restrained imposed deformation (1 = yes) | label | 🪟️serviceability/🦀️.rs:124 |
| `rh` | all | number 1 (Input rh) | number % ex 50 | Relative humidity RH | unit/kind | 🧱️materials-and-time/🦀️.rs:480; 🧱️materials-and-time/🦀️.rs:516 |
| `rho` | all | number 1 (Input rho) | number 1 ex 0.002 | Reinforcement ratio ρ across the interface | label | ✂️shear-torsion/🦀️.rs:206 |
| `rho1` | all | number 1 (Input rho1) | number % ex 50 | Share ρ_1 of lapped bars within 0.65 l_0 | unit/kind | 🔗️detailing/🦀️.rs:66 |
| `rho1000` | all | number 1 (Input rho1000) | number % ex 2.5 | Relaxation loss ρ_1000 after 1000 h | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:69; 🧵️prestress-and-fatigue/🦀️.rs:17 |
| `rho_ly` | all | number 1 (Input rho_ly) | number 1 ex 0.01 | Longitudinal reinforcement ratio ρ_ly | label | 🕳️punching-and-struts/🦀️.rs:155 |
| `rho_lz` | all | number 1 (Input rho_lz) | number 1 ex 0.01 | Longitudinal reinforcement ratio ρ_lz | label | 🕳️punching-and-struts/🦀️.rs:155 |
| `rm` | all | number 1 (Input rm) | number 1 ex 1 | Moment ratio r_m = M_01/M_02 | label | 🏛️analysis-models/🦀️.rs:88 |
| `roughness` | all | text - (Input roughness) | choice ex rough | Roughness of the interface | unit/kind | ✂️shear-torsion/🦀️.rs:202 |
| `s` | all | number mm (Input s) | number mm ex 200 | Spacing s of bars or links | label | 🪟️serviceability/🦀️.rs:124; 🔗️detailing/🦀️.rs:153 |
| `s` | DT-12 | number mm (Input s) | number mm ex 1500 | Pile spacing s | label | 🔗️detailing/🦀️.rs:272 |
| `s2` | all | number mm (Input s2) | number mm | Spacing s_2 between the anchors | label | 🪝️fastenings/🦀️.rs:151; 🪝️fastenings/🦀️.rs:191 |
| `s_first` | all | number 1 (Input s_first) | number mm³ | First moment of area S of the part above the centroid | unit/kind | ✂️shear-torsion/🦀️.rs:140 |
| `s_surf` | all | number mm (Input s_surf) | number mm ex 150 | Spacing of the surface reinforcement | label | 🧩️annex-parts/🦀️.rs:34 |
| `sag` | all | number mm (Input sag) | number mm ex 40 | Sag f of the tendon | label | 🧵️prestress-and-fatigue/🦀️.rs:98; 🌉️bridges-and-liquid/🦀️.rs:59 |
| `shape` | B-03 | text - (Input shape) | choice ex straight | Shape of the bar end | unit/kind | 🔗️detailing/🦀️.rs:40 |
| `shape` | B-08 | text - (Input shape) | choice ex hook_135 | Shape of the link anchorage | unit/kind | 🔗️detailing/🦀️.rs:119 |
| `sigma_c` | all | number N/mm² (Input sigma_c) | number N/mm² ex 10 | Concrete stress σ_c | label | 🧵️prestress-and-fatigue/🦀️.rs:112; 🕳️punching-and-struts/🦀️.rs:289; 🧩️annex-parts/🦀️.rs:117 |
| `sigma_c_max` | all | number N/mm² (Input sigma_c_max) | number N/mm² ex 12 | Maximum concrete stress σ_c,max under the frequent load | label | 🧵️prestress-and-fatigue/🦀️.rs:179 |
| `sigma_c_min` | all | number N/mm² (Input sigma_c_min) | number N/mm² ex 2 | Minimum concrete stress σ_c,min | label | 🧵️prestress-and-fatigue/🦀️.rs:179 |
| `sigma_c_qp` | all | number N/mm² (Input sigma_c_qp) | number N/mm² ex 8 | Concrete stress σ_c,QP at the tendon level (compression positive) | label | 🧵️prestress-and-fatigue/🦀️.rs:69 |
| `sigma_cp` | all | number N/mm² (Input sigma_cp) | number N/mm² ex 1 | Concrete compressive stress σ_cp from axial force or prestress | label | ✂️shear-torsion/🦀️.rs:5 |
| `sigma_gd` | all | number N/mm² (Input sigma_gd) | number N/mm² ex 0.5 | Soil pressure σ_gd under the footing | label | 🕳️punching-and-struts/🦀️.rs:242 |
| `sigma_jack` | all | number N/mm² (Input sigma_jack) | number N/mm² ex 1400 | Jacking stress σ_p,max | label | 🧵️prestress-and-fatigue/🦀️.rs:21 |
| `sigma_n` | all | number N/mm² (Input sigma_n) | number N/mm² ex 0.5 | Normal stress σ_n across the interface (compression positive) | label | ✂️shear-torsion/🦀️.rs:206 |
| `sigma_p` | all | number N/mm² (Input sigma_p) | number N/mm² ex 1000 | Prestressing steel stress σ_p | label | 🪟️serviceability/🦀️.rs:39 |
| `sigma_pi` | all | number N/mm² (Input sigma_pi) | number N/mm² ex 1300 | Initial prestress σ_pi after immediate losses | label | 🧵️prestress-and-fatigue/🦀️.rs:69 |
| `sigma_pm0` | all | number N/mm² (Input sigma_pm0) | number N/mm² ex 1400 | Prestress σ_pm0 at release | label | 🔗️detailing/🦀️.rs:99 |
| `sigma_pm_inf` | all | number N/mm² (Input sigma_pm_inf) | number N/mm² ex 1100 | Prestress σ_pm,∞ after all losses | label | 🧵️prestress-and-fatigue/🦀️.rs:84 |
| `sigma_s` | all | number N/mm² (Input sigma_s) | number N/mm² ex 250 | Reinforcement stress σ_s | label | 🪟️serviceability/🦀️.rs:39; 🪟️serviceability/🦀️.rs:124 |
| `sigma_s_allowed` | all | number N/mm² (Input sigma_s_allowed) | number N/mm² ex 200 | Allowed stress σ_s in the reinforcement | label | 🪟️serviceability/🦀️.rs:85 |
| `sigma_sd` | all | number N/mm² (Input sigma_sd) | number N/mm² ex 250 | Design stress σ_sd of the bar at the anchorage | label | 🔗️detailing/🦀️.rs:22 |
| `situation` | all | text - (Input situation) | choice ex persistent | Design situation | unit/kind | 🧱️materials-and-time/🦀️.rs:147; ✂️shear-torsion/🦀️.rs:121 |
| `situation` | B-05 | text - (Input situation) | choice ex other | Bar loading in the bundle | unit/kind | 🔗️detailing/🦀️.rs:85 |
| `slab` | all | number 1 (Input slab) | number 1 ex 0 | Slab (structural class reduced by one) (1 = yes) | label | 🧱️materials-and-time/🦀️.rs:334 |
| `slip` | all | number mm (Input slip) | number mm ex 6 | Anchorage slip Δ_sl | label | 🧵️prestress-and-fatigue/🦀️.rs:51 |
| `span` | all | number mm (Input span) | number mm ex 9000 | Span l | label | 🪟️serviceability/🦀️.rs:263; 🪟️serviceability/🦀️.rs:280 |
| `special` | all | number 1 (Input special) | number 1 ex 0 | Special quality control (structural class reduced by one) (1 = yes) | label | 🧱️materials-and-time/🦀️.rs:334 |
| `splitting_reinforcement` | all | number 1 (Input splitting_reinforcement) | number 1 ex 1 | Splitting reinforcement provided (1 = yes) | label | 🪝️fastenings/🦀️.rs:135 |
| `sr` | all | number mm (Input sr) | number mm ex 300 | Radial spacing s_r of the studs | label | 🕳️punching-and-struts/🦀️.rs:200 |
| `sr_max` | all | number mm (Input sr_max) | number mm ex 300 | Maximum crack spacing s_r,max | label | 🌉️bridges-and-liquid/🦀️.rs:101 |
| `sr_y` | all | number mm (Input sr_y) | number mm ex 300 | Crack spacing s_r,y in direction y | label | 🪟️serviceability/🦀️.rs:193 |
| `sr_z` | all | number mm (Input sr_z) | number mm ex 300 | Crack spacing s_r,z in direction z | label | 🪟️serviceability/🦀️.rs:193 |
| `st` | all | number 1 (Input st) | number mm ex 300 | Tangential spacing s_t of the studs | unit/kind | 🕳️punching-and-struts/🦀️.rs:200 |
| `stage` | all | text - (Input stage) | choice ex after_losses | Stage of the prestressing steel | unit/kind | 🪟️serviceability/🦀️.rs:39 |
| `state` | all | (none) | choice ex cracked | Section state | added | 📐️section-resistance/📐️section-geometry/🦀️.rs:336 |
| `steel_class` | A-06 | text - (Input steel_class) | choice ex B | Ductility class of the reinforcement | unit/kind | 🏛️analysis-models/🦀️.rs:40 |
| `steel_class` | A-07 | text - (Input steel_class) | choice ex B | Ductility class of the reinforcement | unit/kind | 🏛️analysis-models/🦀️.rs:55 |
| `step_deg` | all | number ° (Input step_deg) | number ° ex 15 | Angle step of the biaxial slice | label | 📐️section-resistance/🦀️.rs:563 |
| `steps` | all | number 1 (Input steps) | number 1 ex 120 | Number of integration steps | label | 🏛️analysis-models/🦀️.rs:188 |
| `stiffness` | all | vector 1 (Input stiffness (vector)) | vector 1 | Torsional stiffnesses T_i of the parts (relative values) | label | ✂️shear-torsion/🦀️.rs:349 |
| `structural_class` | all | number 1 (Input structural_class) | number 1 ex 4 | Structural class S (1 to 6) | label | 🧱️materials-and-time/🦀️.rs:384 |
| `structure` | all | text - (Input structure) | choice ex building | Kind of structure | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:132 |
| `strut_force` | all | number N (Input strut_force) | number N ex 300000 | Strut force F_c | label | 🕳️punching-and-struts/🦀️.rs:273 |
| `sub` | all | text - (Input sub) | choice ex one_way | Slab spanning | unit/kind | 🔥️fire-design/🦀️.rs:36 |
| `sum_ast` | all | number mm² (Input sum_ast) | number mm² ex 600 | Transverse reinforcement ΣA_st in the anchorage or lap | label | 🔗️detailing/🦀️.rs:40; 🔗️detailing/🦀️.rs:66 |
| `sum_ast_min` | all | number mm² (Input sum_ast_min) | number mm² ex 300 | Minimum transverse reinforcement ΣA_st,min | label | 🔗️detailing/🦀️.rs:40 |
| `sum_ecd_ic` | all | number 1 (Input sum_ecd_ic) | number 1 ex 100000000000000.0 | Sum ΣE_cd I_c of the bracing elements [N·mm²] | label | 🏛️analysis-models/🦀️.rs:224 |
| `support_dof` | all | vector 1 (Input support_dof (vector)) | vector 1 | Supported direction of each support (0 = x, 1 = y) | label | 🕳️punching-and-struts/🦀️.rs:302 |
| `support_node` | all | vector 1 (Input support_node (vector)) | vector 1 | Node index of each support | label | 🕳️punching-and-struts/🦀️.rs:302 |
| `sx` | all | number 1 (Input sx) | number N/mm² ex 2 | In-plane stress σ_x (tension positive) | unit/kind | 🧩️annex-parts/🦀️.rs:19 |
| `sy` | all | number 1 (Input sy) | number N/mm² ex 1 | In-plane stress σ_y (tension positive) | unit/kind | 🧩️annex-parts/🦀️.rs:19 |
| `system` | PR-03 | text - (Input system) | choice ex pre | Tensioning system | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:37 |
| `system` | SL-07 | text - (Input system) | choice ex simply | Structural system of the member | unit/kind | 🪟️serviceability/🦀️.rs:259 |
| `t` | M-04 | number 1 (Input t) | number d ex 28 | Concrete age t | unit/kind | 🧱️materials-and-time/🦀️.rs:200 |
| `t` | M-10,M-11,A-01,A-02 | number 1 (Input t) | number d ex 18250 | Age t of the concrete at the end of the period | unit/kind | 🧱️materials-and-time/🦀️.rs:480; 🧱️materials-and-time/🦀️.rs:516 |
| `t` | B-07 | number 1 (Input t) | number d ex 3 | Age t of the concrete at release | unit/kind | 🔗️detailing/🦀️.rs:99 |
| `t` | LQ-06 | number 1 (Input t) | number mm ex 300 | Wall thickness t | unit/kind | 🌉️bridges-and-liquid/🦀️.rs:135 |
| `t` | AF-01 | number 1 (Input t) | number mm ex 200 | Layer thickness t of the membrane element | unit/kind | 🧩️annex-parts/🦀️.rs:19 |
| `t0` | all | number 1 (Input t0) | number d ex 28 | Age t_0 at loading | unit/kind | 🧱️materials-and-time/🦀️.rs:480 |
| `t_a` | all | vector 1 (Input t_a (vector)) | vector mm² | Tendon areas A_p,i | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `t_ed` | all | number N·mm (Input t_ed) | number N·mm ex 20000000 | Design torsional moment T_Ed | label | ✂️shear-torsion/🦀️.rs:329; ✂️shear-torsion/🦀️.rs:357 |
| `t_f` | all | number mm (Input t_f) | number mm ex 150 | Flange thickness t_f | label | 🧩️annex-parts/🦀️.rs:96 |
| `t_hours` | all | number h (Input t_hours) | number h ex 1000 | Time t after tensioning | label | 🧵️prestress-and-fatigue/🦀️.rs:69 |
| `t_min` | all | number min (Input t_min) | number min ex 90 | Duration of the standard fire | label | 🔥️fire-design/🦀️.rs:116; 🔥️fire-design/🦀️.rs:107 |
| `t_pre` | all | vector 1 (Input t_pre (vector)) | vector 1 | Pre-strains of the tendons in permille | label | 📐️section-resistance/🦀️.rs:406 |
| `t_w` | all | number mm (Input t_w) | number N·mm ex 2000000 | Warping torque T_w = dB/dx | unit/kind | 🧩️annex-parts/🦀️.rs:96 |
| `t_y` | all | vector 1 (Input t_y (vector)) | vector mm | Tendon y coordinates | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `t_z` | all | vector 1 (Input t_z (vector)) | vector mm | Tendon z coordinates | unit/kind | 📐️section-resistance/🦀️.rs:406 |
| `tendon` | all | text - (Input tendon) | choice ex strand | Kind of tendon | unit/kind | 🔗️detailing/🦀️.rs:99 |
| `tendon_angle` | all | number ° (Input tendon_angle) | number rad ex 0 | Inclination of the tendon at the column | unit/kind | 🕳️punching-and-struts/🦀️.rs:259 |
| `tendon_kind` | all | text - (Input tendon_kind) | choice ex post_curved_steel | Kind of prestressing steel | unit/kind | 🌉️bridges-and-liquid/🦀️.rs:37 |
| `tension_flange` | all | number 1 (Input tension_flange) | number 1 ex 0 | Flange in tension (1 = yes) | label | ✂️shear-torsion/🦀️.rs:224 |
| `theta` | PR-04 | vector 1 (Input theta (vector)) | vector rad | Angular deviations θ of the tendon at the positions x | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:51 |
| `theta` | M-13,R-07 | number 1 (Input theta) | number °C ex 500 | Temperature θ | unit/kind | 🔥️fire-design/🦀️.rs:140 |
| `theta` | SL-06 | number 1 (Input theta) | number rad | Angle θ between the crack direction and the y axis | unit/kind | 🪟️serviceability/🦀️.rs:193 |
| `theta_m` | all | number ° (Input theta_m) | number °C ex 300 | Temperature θ_M at the reference point M of the zone method | unit/kind | 🔥️fire-design/🦀️.rs:168 |
| `theta_pl` | all | number ° (Input theta_pl) | number rad ex 0.01 | Plastic rotation θ_pl | unit/kind | 🏛️analysis-models/🦀️.rs:55 |
| `thin_early_age` | all | number 1 (Input thin_early_age) | number 1 ex 0 | Thin member in early age (NA-DE) (1 = yes) | label | 🪟️serviceability/🦀️.rs:85 |
| `tie` | all | text - (Input tie) | choice ex peripheral | Kind of tie | unit/kind | 🔗️detailing/🦀️.rs:294 |
| `transient` | all | number 1 (Input transient) | number 1 ex 0 | Transient situation (1 = yes) | label | 🏛️analysis-models/🦀️.rs:231 |
| `ts` | all | number 1 (Input ts) | number d ex 3 | Age t_s at the start of drying | unit/kind | 🧱️materials-and-time/🦀️.rs:516 |
| `tw` | all | number mm (Input tw) | number mm ex 200 | Wall thickness t_w | label | 🔗️detailing/🦀️.rs:248 |
| `txy` | all | number 1 (Input txy) | number N/mm² ex 3 | In-plane shear stress τ_xy | unit/kind | 🧩️annex-parts/🦀️.rs:19 |
| `type` | all | text - (Input type) | choice ex cast_in | Anchor type | unit/kind | 🪝️fastenings/🦀️.rs:83 |
| `u` | all | number mm (Input u) | number mm ex 1600 | Perimeter u_k of the torsion section | label | 🔗️detailing/🦀️.rs:165 |
| `u1_reduced` | all | number mm (Input u1_reduced) | number mm ex 1600 | Reduced control perimeter u_1* | label | 🕳️punching-and-struts/🦀️.rs:116 |
| `v_ccd` | all | number N (Input v_ccd) | number N ex 0 | Shear component V_ccd of the inclined compression chord | label | ✂️shear-torsion/🦀️.rs:191 |
| `v_ed` | all | number N (Input v_ed) | number N ex 200000 | Design shear force V_Ed | label | ✂️shear-torsion/🦀️.rs:160; 🕳️punching-and-struts/🦀️.rs:155 |
| `v_ed` | FA-01,FA-07,FA-08,FA-09,FA-10,FA-13 | number N (Input v_ed) | number N ex 10000 | Design shear load V_Ed | label | 🪝️fastenings/🦀️.rs:45; 🪝️fastenings/🦀️.rs:164; 🪝️fastenings/🦀️.rs:179; 🪝️fastenings/🦀️.rs:191; 🪝️fastenings/🦀️.rs:216; 🪝️fastenings/🦀️.rs:252 |
| `v_ed0` | all | number N (Input v_ed0) | number N ex 100000 | Shear force V_Ed0 without the prestress effect | label | ✂️shear-torsion/🦀️.rs:191 |
| `v_max` | all | number N (Input v_max) | number N ex 200000 | Maximum shear force V_max of the load cycle | label | 🧵️prestress-and-fatigue/🦀️.rs:191 |
| `v_min` | all | number N (Input v_min) | number N ex 100000 | Minimum shear force V_min of the load cycle | label | 🧵️prestress-and-fatigue/🦀️.rs:191 |
| `v_rd_c` | all | number N (Input v_rd_c) | number N ex 80000 | Shear resistance V_Rd,c without links | label | 🧵️prestress-and-fatigue/🦀️.rs:191; 🔗️detailing/🦀️.rs:204 |
| `v_rd_cp` | all | number N (Input v_rd_cp) | number N ex 80000 | Design resistance V_Rd,cp (pry-out) | label | 🪝️fastenings/🦀️.rs:216 |
| `v_rd_max` | all | number N (Input v_rd_max) | number N ex 500000 | Shear resistance V_Rd,max of the struts | label | 🔗️detailing/🦀️.rs:204 |
| `v_rd_s` | all | number N (Input v_rd_s) | number N ex 100000 | Design resistance V_Rd,s (steel shear) | label | 🪝️fastenings/🦀️.rs:216 |
| `v_td` | all | number N (Input v_td) | number N ex 0 | Shear component V_td of the inclined tension chord | label | ✂️shear-torsion/🦀️.rs:191 |
| `w` | R-04 | number 1 (Input w) | number mm ex 100 | Width w of the zone for the zone method | unit/kind | 🔥️fire-design/🦀️.rs:168 |
| `w` | ST-01 | number 1 (Input w) | number mm ex 100 | Width w of the strut | unit/kind | 🕳️punching-and-struts/🦀️.rs:273 |
| `w_before` | all | number mm (Input w_before) | number mm ex 5 | Deflection w_before before the finishes | label | 🪟️serviceability/🦀️.rs:280 |
| `w_el` | all | number mm (Input w_el) | number mm³ | Elastic section modulus W_el of the anchor | unit/kind | 🪝️fastenings/🦀️.rs:164 |
| `w_strut` | all | number mm (Input w_strut) | number mm ex 100 | Width w of the corbel strut | label | 🧩️annex-parts/🦀️.rs:73 |
| `wall_length_m` | all | number m (Input wall_length_m) | number m ex 8 | Wall length | label | 🔗️detailing/🦀️.rs:294 |
| `welded` | all | number 1 (Input welded) | number 1 ex 0 | Welded transverse bars (1 = yes) | label | 🔗️detailing/🦀️.rs:40 |
| `wheel_print` | all | number mm (Input wheel_print) | number mm ex 400 | Side length of the wheel print | label | 🌉️bridges-and-liquid/🦀️.rs:66 |
| `width` | all | number mm (Input width) | number mm ex 300 | Beam width b or column load level, read as width (R-01) | label | 🔥️fire-design/🦀️.rs:79 |
| `width_m` | all | number m (Input width_m) | number m ex 6 | Width of the internal tie zone | label | 🔗️detailing/🦀️.rs:294 |
| `wk` | all | number mm (Input wk) | number mm ex 0.3 | Calculated crack width w_k | label | 🌉️bridges-and-liquid/🦀️.rs:47; 🌉️bridges-and-liquid/🦀️.rs:83; 🌉️bridges-and-liquid/🦀️.rs:121 |
| `wk_lim` | all | number mm (Input wk_lim) | number mm ex 0.3 | Crack width limit w_k,lim | label | 🪟️serviceability/🦀️.rs:124; 🪟️serviceability/🦀️.rs:231; 🌉️bridges-and-liquid/🦀️.rs:101 |
| `x` | M-07,S-01,SL-06 | number mm (Input x) | number mm ex 150 | Neutral axis depth x | label | 🧱️materials-and-time/🦀️.rs:238; 📐️section-resistance/🦀️.rs:446; 🪟️serviceability/🦀️.rs:193 |
| `x` | PR-04 | vector 1 (Input x (vector)) | vector m | Positions x along the tendon | unit/kind | 🧵️prestress-and-fatigue/🦀️.rs:51 |
| `x_compression` | all | number mm (Input x_compression) | number mm ex 150 | Compression zone height x | label | 🌉️bridges-and-liquid/🦀️.rs:83 |
| `x_max` | all | number mm (Input x_max) | number mm ex 100 | Upper x limit of the concrete member | label | 🪝️fastenings/🦀️.rs:29 |
| `x_min` | all | number mm (Input x_min) | number mm ex -100 | Lower x limit of the concrete member | label | 🪝️fastenings/🦀️.rs:29 |
| `xi` | all | number 1 (Input xi) | number 1 ex 0.5 | Ratio ξ of the bond strength of prestressing steel to reinforcing steel | label | 🧵️prestress-and-fatigue/🦀️.rs:137; 🪟️serviceability/🦀️.rs:193 |
| `xi_lim` | all | number 1 (Input xi_lim) | number 1 ex 0.617 | Limit depth ratio ξ_lim = x/d | label | 📐️section-resistance/🦀️.rs:476 |
| `xu_d` | all | number mm (Input xu_d) | number 1 ex 0.3 | Neutral axis depth ratio x_u/d | unit/kind | 🏛️analysis-models/🦀️.rs:40; 🏛️analysis-models/🦀️.rs:55 |
| `xy_d` | all | number mm (Input xy_d) | number 1 ex 0.3 | Yield depth ratio x_y/d | unit/kind | 🏛️analysis-models/🦀️.rs:55 |
| `y_compression` | all | number mm (Input y_compression) | number mm | y coordinate of the compressed edge | label | 🪝️fastenings/🦀️.rs:45 |
| `y_max` | all | number mm (Input y_max) | number mm ex 100 | Upper y limit of the concrete member | label | 🪝️fastenings/🦀️.rs:29 |
| `y_min` | all | number mm (Input y_min) | number mm ex -100 | Lower y limit of the concrete member | label | 🪝️fastenings/🦀️.rs:29 |
| `y_n` | all | number mm (Input y_n) | number mm ex 0 | y coordinate of the normal force | label | 🪝️fastenings/🦀️.rs:45 |
| `z` | all | number mm (Input z) | number mm ex 405 | Lever arm z | label | ✂️shear-torsion/🦀️.rs:121 |
| `z0` | all | number mm (Input z0) | number mm ex 300 | Lever arm z_0 of the corbel | label | 🧩️annex-parts/🦀️.rs:73 |
| `z_compression` | all | number mm (Input z_compression) | number mm | Distance z of the compression load from the anchor group | label | 🪝️fastenings/🦀️.rs:94 |
| `z_h` | all | number mm (Input z_h) | number mm ex 0 | Height z_H of the horizontal force above the main tie | label | 🧩️annex-parts/🦀️.rs:73 |
| `z_i` | all | number mm (Input z_i) | number mm ex 300 | Lever arm z_i of the web wall | label | ✂️shear-torsion/🦀️.rs:357 |
| `zcp` | all | number mm (Input zcp) | number mm ex 100 | Eccentricity z_cp of the tendons from the centroid | label | 🧵️prestress-and-fatigue/🦀️.rs:69 |
| `zone_theta` | all | vector 1 (Input zone_theta (vector)) | vector °C | Temperatures θ_i of the zones | unit/kind | 🔥️fire-design/🦀️.rs:168 |
