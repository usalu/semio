# Oracle Library Coverage

Introspection of the installed packages in `/home/deck/semio/.venv` (Python 3.14), 2026-11-02. Regenerate the raw symbol matrix with
`/home/deck/semio/.venv/bin/python -I ✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔍️coverage/🐍️.py OUT.md` (about 18 000 lines, clause numbers there are regex guesses from docstrings and are noisy; this file is the curated reading of it). Units: libraries work in N, mm, MPa; the adapters convert from SI.

| library | version | licence | covers | does not cover |
|---|---|---|---|---|
| structuralcodes | 0.7.3 | MIT | EN 1992-1-1:2004 materials, shear, crack control, section fibre analysis; fib MC2010/MC2020; EN 1992-1-1:2023 materials and SLS | EN 1992 punching, torsion, second order, anchorage, fire; EN 1990-1991, 1993-1999 |
| concreteproperties | 0.8.0 | MIT | RC section fibre analysis with the EN parabola-rectangle law (`EurocodeParabolicUltimate`, `EurocodeNonLinear`); ultimate bending, M-N and biaxial diagrams, moment-curvature, cracked and service stresses | design-code modules exist only for AS 3600 and NZS 3101, none for EN 1992 |
| sectionproperties | 3.10.2 | MIT | FE section constants A, I, J, I_w, shear areas, shear centre, elastic and plastic moduli, stresses; presets I, RHS, CHS, EHS, channel, angle, tee, zed, cee, box girder, bridge sections, CLT, concrete shapes, Nastran shapes | no code checks, no EN classification |
| blue-prints | 0.8.0 | MIT | clause-numbered formula classes (see below) | EN 1990, 1991, 1994, 1996, 1998, 1999; EN 1993 member buckling (6.3) |
| groundhog | 0.15.0 | GPL-3.0 | bearing capacity factors and API/Vesic capacity, earth pressure (Rankine, Poncelet), pile capacity, Boussinesq stresses, CPT/SPT correlations, phase relations | EN 1997 partial factors, Annex D shape/inclination factors, Annex C log-spiral K_p |
| sympy | 1.14.0 | BSD-3 | exact symbolic solutions of any closed form | no Eurocode content |
| PyNiteFEA | 3.0.0 | MIT | 3D frame/plate FEA: linear, P-Delta, modal, pushover, load combinations | no code content |
| anastruct | 1.7.0 | GPL-3.0 | 2D frames and trusses: linear and geometrically non-linear, hinges, springs | no code content |

## structuralcodes 0.7.3

`structuralcodes.codes.ec2_2004` (EN 1992-1-1:2004, 130 public symbols):
- Materials (Table 3.1, 3.1.3-3.1.7, 3.2): `fcm`, `fctm`, `fctk_5`, `fctk_95`, `Ecm`, `eps_c1`, `eps_cu1`, `eps_c2`, `eps_cu2`, `n_parabolic_rectangular`, `eps_c3`, `eps_cu3`, `fcd`, `fctd`, `fyd`, `epsud`, `k_sargin`, `beta_cc`, `beta_ct`, `beta_E`.
- Creep and shrinkage (3.1.4, Annex B; 25 symbols): `phi`, `phi_0`, `phi_RH`, `beta_fcm`, `beta_t0`, `beta_c`, `eps_cs`, `eps_cd`, `eps_cd_0`, `eps_ca`, `eps_ca_inf`, `h_0`.
- Shear (6.2; `shear` module, 11 functions): `VRdc` (6.2a/b, 6.3N), `VRdmax` (6.9, alpha_cw), `VRds` (6.8), `Asw_s_required` (6.13), `VEdmax_unreinf` (6.5), `Asw_max` (6.12), `VRdc_prin_stress`.
- Crack control (7.3; 24 symbols): `As_min`, `As_min_2`, `As_min_p`, `kc_tension`, `kc_rect_area`, `kc_flanges_area`, `k`, `kt`, `hc_eff`, `rho_p_eff`, `alpha_e`, `eps_sm_eps_cm`, `sr_max_close`, `sr_max_far`, `sr_max_theta`, `wk`, `w_max` (Table 7.1N), `eps_sm_eps_cm_restraint_end`.
- Sections: `GenericSection` with `calculate_bending_strength(theta, n)`, `calculate_nm_interaction_domain`, `calculate_mm_interaction_domain`, `calculate_nmm_interaction_domain`, `calculate_moment_curvature`, `get_balanced_failure_strain`, `gross_properties`; polygon geometry with lumped reinforcement, parabola-rectangle and bilinear laws, steel hardening through `ftk`/`epsuk`.
- Not present: 6.3 torsion, 6.4 punching, 5.8 slenderness, 8 anchorage, 9 detailing, 4.4 cover tables, fire.

`ec2_2023` (45 symbols: section 5 materials, Annex B time dependent, section 9 SLS) and `mc2010` (95 symbols) are not used by the oracle cases (edition 2004 is the subject).

## concreteproperties 0.8.0

- `ConcreteSection`: `get_gross_properties`, `calculate_cracked_properties` (`d_nc`, `e_ixx_c_cr`, `m_cr`), `calculate_cracking_moment`, `calculate_service_stress`, `calculate_cracked_stress`, `ultimate_bending_capacity(theta, n)` (compression positive), `moment_interaction_diagram`, `biaxial_bending_diagram`, `moment_curvature_analysis`, `calculate_uncracked_stress`.
- Material laws: `ConcreteLinear`, `ConcreteLinearNoTension`, `EurocodeNonLinear` (EN 1992 Eq. 3.14 service), `EurocodeParabolicUltimate` (EN 1992 3.1.7(1)), `RectangularStressBlock`, `BilinearStressStrain`, `ModifiedMander`, `SteelElasticPlastic`, `SteelHardening`, `StrandHardening`.
- Design codes: `AS3600`, `NZS3101` only. The EN 1992 resistance is therefore obtained by passing f_cd, the Table 3.1 strains and f_yd as material constants (NA parameters stay inputs of the case).

## sectionproperties 3.10.2

- `Section`: `calculate_geometric_properties` (A, I, centroid, radii), `calculate_warping_properties` (J, Gamma = I_w, shear centre, shear areas, Beta), `calculate_plastic_properties` (S, plastic centroid, shape factors), `calculate_stress`, `calculate_frame_properties`.
- Presets used: `i_section` (root radius), `rectangular_hollow_section`, `circular_hollow_section`, `channel_section` (parallel flange), `angle_section`. Also available: `elliptical_hollow_section`, `mono_i_section`, `cee_section`, `zed_section`, `tee`, `box_girder_section`, `clt_rectangular_section`, `concrete_*` shapes.
- Convergence: HEB 240 at 20 mm2 element area: A, I to 1e-9, J within 0.03 percent of the 2 mm2 mesh, I_w within 1e-5.

## blue-prints 0.8.0

Formula classes (`float(Form...)`) and tables, by code and chapter (module count):

| code | modules | chapters |
|---|---|---|
| EN 1992-1-1:2004 | 224 | 3 materials 28, 4 cover 9 (Tables 4.1-4.4N), 5 analysis 53, 6 ULS 72 (shear 6.2-6.9, torsion 6.26-6.31, punching 6.32-6.54, struts 6.55-6.65, fatigue 6.70-6.79), 7 SLS 21 (7.1-7.21 cracking, 7.16 span/depth), 8 detailing 21, 9 detailing 15, 12 plain concrete 5 |
| EN 1992-2:2005 | 1 | 5.101 imperfections |
| FprEN 1992-1-1:2023 | 91 | chapter 8 |
| EN 1993-1-1:2005 | 57 | 2 (2.1, 2.2), 3 (Table 3.1), 5 (5.1, 5.2, 5.7, 5.8), 6 sections 6.1-6.45, 6.54, 6.55, shear areas 6.18a-g |
| EN 1993-1-1:2022 | 33 | chapter 8 |
| EN 1993-1-8:2005 | 3 | Tables 3.1, 3.4, formula 4.2 |
| EN 1993-1-9:2005 | 2 | Annex A |
| EN 1993-5:2007 | 17 | chapter 5 |
| EN 1995-1-1:2004 | 5 | 7.3-7.7 (deflection ratio, vibration) |
| EN 1995-1-1:2023/2025 | 10 | Annex E |
| EN 338:2016 | 3 | Tables 1-3 (strength classes) |
| NEN 9997-1+C2:2017 | 5 | 1.0.1, 2.1a/b, 2.2, 2.4 (no bearing capacity) |

Absent: EN 1993 6.46-6.53 (6.3 member buckling chi, interaction 6.3.3), EN 1990, 1991, 1994, 1996, 1998, 1999, EN 1997 bearing/earth pressure.

## groundhog 0.15.0

Importable numeric modules (needs inert stubs for `plotly`, `pyproj`, `jinja2`, see requests):
- `shallowfoundations.capacity`: `nq_frictionangle_sand` (Prandtl), `ngamma_frictionangle_vesic` / `_meyerhof` / `_davisbooker`, `verticalcapacity_drained_api` / `_undrained_api`, `slidingcapacity_*_api`, `effectivearea_*_api`, envelopes, `failuremechanism_prandtl`, `ShallowFoundationCapacityDrained` / `Undrained`; `stressdistribution` (Boussinesq point, strip, rectangle, circle); `settlement`.
- `excavations.basic`: `earthpressurecoefficients_rankine` (inclined wall, sloping ground), `_poncelet` (Coulomb plane), `_frictionangle` (Mohr circle).
- `siteinvestigation.insitutests.pcpt_correlations` (40 correlations): `frictionangle_sand_kulhawymayne`, `undrainedshearstrength_clay_radlunne`, `behaviourindex_pcpt_robertsonwride`, `relativedensity_*_baldi`, `gmax_*`, `ocr_cpt_lunne`, `k0_sand_mayne`, `constrainedmodulus_pcpt_robertson`, Vs correlations; `spt_correlations`; `classification.phaserelations` (8), `categories`.
- `deepfoundations`: API and Alm-Hamre unit skin friction and end bearing, De Beer, Koppejan (those two need plotly/jinja2), pile settlement curves, negative skin friction.
- EN 1997 relevant: N_q = Prandtl, N_gamma three conventions (none equals EN 1997 D.4 2 (N_q - 1) tan phi), no shape/inclination/depth factors of Annex D.

## sympy 1.14.0

General algebra (17 000 public symbols); used for exact roots (`solve`) and special functions (`erfinv`). No Eurocode content.

## PyNiteFEA 3.0.0 and anastruct 1.7.0

- PyNite `FEModel3D`: `add_node`, `add_member`, `add_member_dist_load`, `add_member_pt_load`, `add_node_load`, `add_load_combo`, `add_spring`, `add_quad`, `add_plate`, `add_steel_section`, `analyze`, `analyze_linear`, `analyze_PDelta`, `analyze_modal` (modal result disagrees with closed form, see `mechanics-kernel.md`, not used), `analyze_pushover`. Member results: `max_moment`, `min_moment`, `moment(x)`, `max_deflection`, node `RxnFY`, `DX`.
- anastruct `SystemElements`: `add_element`, `add_truss_element`, supports (hinged, roll, fixed, rotational, spring), `add_internal_hinge`, `q_load`, `point_load`, `solve` (also geometrically non-linear), `get_element_results` (`Mmin`, `Mmax`, `wtotmin`, `wtotmax`), `get_node_results_system` (`Fy`, `Tz`), `get_node_displacements`.

## Adapter map (what the harness computes)

| adapter | check ids |
|---|---|
| structuralcodes | `en1992.3.1.concrete-laws`, `3.15.fcd`, `6.1.flexure-rect`, `6.1.nm-interaction`, `6.2.shear-vrdc`, `6.2.shear-links`, `7.8.crack-width`, `7.9.strain-difference`, `7.11.sr-max` |
| concreteproperties | `en1992.6.1.flexure-rect`, `6.1.nm-interaction`, `7.2.cracked-section` |
| sectionproperties | `en1993.section.{i-rolled,rhs,chs,channel,angle}`, `en1993.6.3.2.mcr-i` |
| blueprints | `en1992.{3.15.fcd,3.17.sigma-c,5.13n.lambda-lim,5.14.slenderness,5.34.curvature,6.2.shear-vrdc,6.6n.nu,6.30.t-rd-max,6.38.punching-stress,7.8.crack-width,7.9.strain-difference,7.11.sr-max,7.16.span-depth,8.3.anchorage-rqd}`, `en1993.{5.2.alpha-cr,6.2.npl,6.2.mc-rd-class12,6.2.vpl-rd,6.2.av-*,6.2.my-v-rd,6.2.my-n-rd,6.3.mb-rd}`, `en1995.7.5.natural-frequency` |
| groundhog | `en1997.D.4.bearing-factors`, `en1997.9.earth-pressure`, `en1997.2.cpt-friction-angle`, `cpt-undrained-strength`, `cpt-behaviour-index` |
| sympy | `en1993.6.3.1.chi`, `en1993.euler-ncr`, `en1990.C.beta` |
| pynite, anastruct | `mechanics.beam-continuous` (spans, UDL per span, midspan forces, optional clamped start), `mechanics.portal-sway` |

Not covered by any library (stay "independent formulation"): EN 1990 combinations, EN 1991 NA tables, EN 1993 6.3.1/6.3.3 and fire, EN 1994, EN 1996, EN 1998, EN 1999, EN 1995 Johansen and timber classes (blue-prints has EN 338 tables, not wired).
