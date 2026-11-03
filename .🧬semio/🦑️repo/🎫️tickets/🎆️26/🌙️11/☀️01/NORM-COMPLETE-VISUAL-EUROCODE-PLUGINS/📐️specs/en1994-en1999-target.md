# 📐 EN 1994 and EN 1999 Target Specification

Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`, goal `norm`. Scope: EN 1994-1-1, 1-2, 2 (composite steel-concrete) and EN 1999-1-1 to 1-5 (aluminium) with the German national annexes (NA-DE). Target artifacts: `🧩️en1994`, `🪶️en1999` under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/`.

## 0. Reading Guide and Verification Status

- Every item below is a *target* check; none requires source changes by this document.
- Status tags in the catalogue headers: `[V]` = formula transcribed and numerically verified in section 7/8 (hand-checked, reproduced with an independent Python script during authoring); `[T]` = structure and symbols are specified here, but the exact equation/table values MUST be transcribed verbatim from the published standard text by a norm-text reviewer before codification (the implementing agent MUST NOT guess table entries; load tables from data files under `📚️tables/`); `[N]` = NDP value with NA-DE status stated; where NA-DE equals the recommendation it is written "NA-DE = rec."; `NA-DE ?` means the NA-DE value MUST be confirmed against DIN EN 1994-1-1/NA, DIN EN 1994-2/NA, DIN EN 1999-1-1/NA etc.
- Language: all labels `LocalizedLabel::native(en, de)`; both languages are given in the catalogue (EN / DE). No default language.
- Units in the model: SI base (m, N, Pa, kg, s, K). Display units (mm, kN, MPa, kNm) are presentation only. Formulae below are written in mm, N, MPa unless noted, because that is how the standards print them.
- Pass/fail: `utilisation = demand / resistance` (or the stated generalisation); pass iff `utilisation <= 1.0`; report `status in {pass, fail, not-applicable, out-of-scope}`. `out-of-scope` is mandatory when a validity limit of the clause is violated (never silently extrapolate).

### 0.1 Common partial factors

| Symbol | Meaning (EN / DE) | Recommended | NA-DE |
|---|---|---|---|
| γM0 | structural steel cross-section / Baustahl Querschnitt | 1.0 | 1.0 |
| γM1 | steel member stability / Stabilität | 1.0 | 1.0 |
| γM2 | steel net section, connections / Nettoquerschnitt, Anschluss | 1.25 | 1.25 |
| γC | concrete / Beton | 1.5 | 1.5 (persistent/transient) |
| γS | reinforcement / Betonstahl | 1.15 | 1.15 |
| γV | headed studs / Kopfbolzendübel | 1.25 | 1.25 |
| γVS | composite slab longitudinal shear / Verbunddecke | 1.25 | 1.25 |
| γap | profiled sheeting / Profilblech | 1.0 | 1.0 |
| γM1 (Al) | aluminium member / Aluminium Bauteil | 1.1 | 1.1 |
| γM2 (Al) | aluminium net section, connections | 1.25 | 1.25 |
| γMf | fatigue / Ermüdung | see 1999-1-3 (§6.6) | NA-DE ? |

Accidental / fire: all γM,fi = 1.0 (EN 1994-1-2, EN 1999-1-2). NA-DE ? entries MUST be confirmed.

### 0.2 Vocabulary alignment with existing models

Existing `En1994Snapshot` (artifact `🧩️en1994`): `annex`, `structure_kind`, `steel_f_y_pa`, `beams: CompositeBeam {span_m, spacing_m, support, construction, steel: SteelSection{designation,height_m,width_m,tw_m,tf_m}, slab_thickness_m, concrete_f_ck_pa, concrete_e_cm_pa, sheeting: ProfiledSheeting{profile,height_m,rib_width_m,thickness_m,ribs_parallel_to_beam}, studs: HeadedStuds{diameter_m,height_m,f_u_pa,count_per_rib,spacing_m,total_count}, ltb_length_m, bar_spacing_m, wk_limit_m, n_cycles, actions: CharacteristicAction{id,kind,category,stage,q_area_pa,f_k_n,delta_sigma_k_pa,delta_tau_k_pa}}`, `columns: CompositeColumn {kind,length_m,outer_size_m,wall_thickness_m,steel_f_y_pa,concrete_f_ck_pa,reinforcement_f_yk_pa,buckling_curve,actions: ColumnAction{n_k_n,m_k_nm,..}}`, `slabs: CompositeSlab {span_m,support,sheeting,concrete_thickness_m,f_ck_pa,..}`, `fire_rating`, `insulation_thickness_m`, `fatigue_detail`.

Existing `En1999Snapshot` (artifact `🪶️en1999`): `annex`, `materials: AluminiumMaterial{id,designation}`, `sections: AluminiumSection{kind,height,width,flange_thickness,web_thickness,outer_diameter,elements: PlateElement{width,thickness,outstand,welded,weld_position}}`, `members: AluminiumMember{section_id,material_id,length,support: SupportCondition,buckling_length_{y,z,t},ltb_length,restrained_ltb,actions: MemberAction{g_k_line,q_k_line,n_k,v_y_k,v_z_k,m_y_k,m_z_k}}`, `connections: AluminiumConnection{kind,bolts: BoltGroup,welds: WeldGroup{filler_alloy,throat,length,beta_w,haz_extent}}`, `fire_scenarios: FireScenario{theta_a,duration_s}`, `fatigue_details: FatigueDetail{detail_category,delta_sigma_c,delta_sigma_ed,n_cycles}`, `cold_formed: ColdFormedSheet`, `shells: AluminiumShell`.

Gaps relative to this target (to be closed schema-first, section 4): no stage-wise construction/propping model, no shrinkage/creep inputs, no concrete reinforcement layout in beams (hogging), no effective-width geometry (Le, b1, b2, end support), no connector layout per zone, no sheeting profile geometry beyond height/rib width (b0, pitch, bottom rib width, re-entrant), no encased/partially-encased section model, no polygon output, no long-term parameters, no U-frame data, no bridge-specific data, no fire-resistance tabulation model; EN 1999: no alloy/temper/product-form table binding (only `designation`), no HAZ layout, no weld-run model, no plated-girder (stiffener) model, no tension-field, no torsion/warping inputs, no per-element stress distribution, no cut-out/holes, no fatigue S-N spectrum, no shell geometry/BCs beyond a placeholder.

---

## 1. EN 1994 Verification Catalogue

Numbering `C-xx`. Clause ids refer to EN 1994-1-1:2004+A1:2008 unless prefixed (`2:` = EN 1994-2, `F:` = EN 1994-1-2).

### 1.1 Materials, classification, global analysis

**C-01 Concrete and steel material set [V/T]** — cl. 3.1, 3.2, 3.3, 3.5. EN: "Material properties" / DE: "Werkstoffeigenschaften".
- Inputs: concrete class (fck, fcm, fctm, fctk0.05, Ecm) from EN 1992-1-1 Table 3.1, lightweight classes (LC) with ρ; structural steel S235…S460 (fy, fu by thickness t<=40 / 40<t<=80), reinforcement fsk=500 MPa, Es=200 GPa; sheeting S280GD…S350GD (fyp,k, fu), headed stud fu<=500 MPa; Ea=210 GPa, Es=200 GPa (steel), Ecm.
- Formulae: fcd = αcc fck/γC with composite-beam rule fcd,beam = 0.85 fck/γC (explicitly applied as factor 0.85, αcc NOT additionally applied); fyd = fy/γM0; fsd = fsk/γS; fctm,eff for crack control = fctm (or lower value if cracking expected before 28 d).
- NDP: αcc: NA-DE ? (EN 1994 applies 0.85 explicitly; implementation: constant `concrete_beam_factor = 0.85` with annex override; MUST confirm against NA-DE).
- Output: design strengths table. Pass criterion: validity ranges: C20/25 <= class <= C60/75 (NDP; LC16/18 to LC55/60), S235..S460, fu(stud) <= 500.
- Edge cases: out-of-range class -> `out-of-scope`; steel thickness bands switch fy; concrete > C60/75 -> out-of-scope.

**C-02 Classification of steel elements in composite members [V]** — cl. 5.5, Table 5.2. EN: "Cross-section classification" / DE: "Klassifizierung".
- Inputs: section dims, fy, concrete encasement presence, stress distribution.
- Formula: ε = sqrt(235/fy[MPa]). Flange outstand c/tf limits: Class 1 <= 9ε, Class 2 <= 10ε, Class 3 <= 14ε (rolled); web in bending (c/tw): per EN 1993-1-1 Table 5.2; modifications for partial encasement: web with concrete encasement may be treated at one class better if encasement prevents buckling (cl. 5.5.3, concrete-encased web: c/tw limit rule). Flange restrained by shear connectors to the slab: class 1/2 limit by stud spacing (cl. 6.6.5.5 max spacing: sc/tf <= 22 sqrt(235/fy) for restraint of buckling in compression flange, and clear distance to stud rows e/tf <= 9ε).
- Output: class per element and section = max. Class 4 -> effective widths per EN 1993-1-5.
- Edge cases: composite cross-section class depends on moment sign (sagging: only compression above the plastic NA of steel is relevant); reduce classification in hogging with reinforcement.

**C-03 Effective width of concrete flange [V]** — cl. 5.4.1.2, Fig. 5.1. EN: "Effective width" / DE: "Mittragende Breite".
- Inputs: Le (distance between zero-moment points, from Fig. 5.1: sagging simply supported Le=L; end span 0.85 L1; internal span 0.7 L2; cantilever 2 L; hogging at internal support 0.25(L1+L2)), b0 (distance between outer connector rows), bi (half clear distance to adjacent web or half spacing), end/intermediate location.
- Formula: beff = b0 + Σ bei, bei = min(Le/8, bi). At end supports: beff = b0 + Σ βi bei, βi = (0.55 + 0.025 Le/bei) <= 1.0.
- NDP: none; NA-DE = rec.
- Outputs: beff at midspan and at supports; used by C-04..C-09, C-26. For global analysis use beff,1 at midspan constant over span (5.4.1.2(5)).
- Pass: geometric; always reported; warn if beff<b0 impossible.
- Edge cases: b0 = 0 (single stud row), unsymmetric flange (b1<>b2), bei = 0.
- Example: section 7, E1-01.

**C-04 Global analysis method selection [T]** — cl. 5.4.2 (beams in buildings) / 2:5.4.2 (bridges). Method: linear-elastic with cracking effects (15% span cracked region each side of internal supports for adjacent spans ratio >=0.6; EI2 in cracked, EI1 elsewhere), or non-linear, or rigid-plastic (with Class 1/2 and rotation capacity), or redistribution per Table 5.1 (max redistribution of support moments % for classes 1..4 and uncracked/cracked analysis: Class 1/uncracked 40, Class 2/uncracked 30, Class 3 20, Class 4 10; cracked: 25, 15, 10, 0). Output: design effects with limit on redistribution; pass iff applied redistribution <= allowed.
- Edge cases: nonprestressed vs. prestressed; "in beams where adjacent span ratio < 0.6" -> full non-cracked analysis not allowed.

**C-05 Shrinkage, creep and modular ratios [V]** — cl. 5.4.2.2, 3.1(4), 7.2.1(?). EN: "Effects of creep and shrinkage" / DE: "Kriechen und Schwinden".
- Formula: n0 = Ea/Ecm; nL = n0 (1 + ψL φ(t,t0)); ψL = 1.1 (permanent loads), 0.55 (primary shrinkage), 1.5 (prestress by imposed deformations); φ(t,t0) from EN 1992-1-1 Annex B or simplified φt (value for t and t0 of load application). Stage-wise: construction stage uses steel only (unpropped) or propped.
- NDP: ψL values NA-DE = rec. Shrinkage strain εcs (cl. 3.1(4)): total by EN 1992-1-1 3.1.4; autogenous plus drying; NA-DE ? for εcs simplified values (0.3 per mille typical).
- Outputs: n0, nL (per load group), n_sh.
- Edge cases: lightweight concrete shrinkage value; early-age loading t0 small -> larger φt; φt=0 for instantaneous.

**C-06 Section properties (transformed) [V]** — 6.2.1.4, 7.2.1. Compute A*, y_b, I* of composite section for n in {n0, nL, n_sh}: A* = Aa + Ac/n; I* = Ia + Aa(ya−yb)² + Ic/n + (Ac/n)(yc−yb)². Cracked section (hogging): concrete neglected, only reinforcement As with Es.
- Output: I_I (uncracked), I_II (cracked); also section modulus per fibre. Edge: concrete in sheeting ribs neglected (sagging).
- Example E1-10.

### 1.2 Bending resistance of composite beams

**C-07 Plastic resistance moment, sagging, full shear connection [V]** — cl. 6.2.1.2. EN: "Plastic resistance moment Mpl,Rd" / DE: "Plastisches Moment Mpl,Rd".
- Inputs: Aa (steel area), fyd, beff, hc (concrete above ribs), hp (rib height), fck, ha; for sheeting ribs parallel concrete in ribs is neglected (conservative per code, but may be optionally included when ribs run parallel — NOT included by default).
- Formulae: Npl,a = Aa fyd; Ncf = 0.85 fcd beff hc (with fcd = fck/γC, so 0.85 fck/γC); NA position:
  - NA in slab (Npl,a <= Ncf): x = Npl,a / (0.85 fcd beff); Mpl,Rd = Npl,a (ha/2 + hc + hp − x/2) (measured from slab top; hp = 0 for solid slab).
  - NA in steel flange (Ncf < Npl,a, Ncf >= Npl,a − 2 bf tf fyd ... ): steel compression Fsc = (Npl,a − Ncf)/2; Mpl,Rd = Npl,a·ha/2 + Ncf (hc/2 + hp) − Fsc... full formula: Mpl,Rd = Σ Fi zi about the plastic NA with concrete force Ncf at (hc/2 above steel top + hp) and steel tension/compression zones from the plastic neutral axis; implement through general polygon stress-block integration (C-11).
  - NA in web: same with web portion.
- Output: Mpl,Rd, plastic NA depth, dimension flags `na_in_slab|flange|web`.
- Pass: MEd <= Mpl,Rd (class 1/2). Class 3 -> C-09. Class 4 -> effective section.
- Edge cases: Npl,a == Ncf; unsymmetric I; steel-grade mix (flange/web) -> use per element fy; sheeting ribs perpendicular -> concrete in ribs ignored; hp counts only as lever arm.
- Example E1-02 (Mpl,Rd = 423.9 kNm).

**C-08 Plastic resistance moment, hogging [T]** — cl. 6.2.1.2(2), 6.2.1.3. Concrete neglected; reinforcement As fsd in beff (hogging); NA found from Σ forces: tension in reinforcement + steel below NA; compression steel above NA. Mpl,Rd = Σ Fi zi. Inputs add: As, fsd, cover, bar centroid. Steel classes: class 1/2 steel; if web slender -> C-02 reduces.
- Pass: MEd,hog <= Mpl,Rd,hog. Edge: NA exactly at flange/web junction; As too small -> NA in web (compress web) fine; reinforcement limits per 5.5 in hogging (ductility: As,min 7.4.2 ensures class).

**C-09 Elastic resistance, class 3/4, staged analysis [V]** — cl. 6.2.1.4, 7.2.1. EN: "Elastic resistance moment" / DE: "Elastische Beanspruchbarkeit".
- Inputs: stage moments Ma,Ed (steel only, before composite action), Mc,Ed (after, with n0/nL), Mc,sh (shrinkage), fyd, fcd (0.85 fck/γC ... stress limit in extreme concrete fibre = 0.85 fcd), fsd.
- Formula: stress limit method: σ = σ_a(Ma,Ed) + k σ_c(Mc,Ed; I*, n) where k = largest factor such that any fibre stress limit is reached: steel yield fyd (tension & compression, with class 3 effective), concrete 0.85 fcd (compression), rebar fsd. MEl,Rd = Ma,Ed + k Mc,Ed. For class 4 use effective section properties in the steel part.
- Outputs: k, MEl,Rd, governing fibre.
- Pass: MEd <= MEl,Rd with Ma,Ed, Mc,Ed from the combination stage.
- Edge cases: unpropped, propped; deformation-constraint (shrinkage) included as imposed deformation via nL(ψ=0.55) in the Mc-part; for composite beams where plastic resistance is permitted (class 1/2) elastic checks are not required.

**C-10 Non-linear resistance [T]** — cl. 6.2.1.4(6): "Non-linear resistance to bending" / DE: "Nichtlineare Tragfähigkeit". Stress-strain: concrete parabolic-rectangular (EN 1992-1-1), steel bilinear (fy, no hardening), reinforcement bilinear, strains limited (εcu3 and steel strain limit per class). Strain-compatibility integration; plane sections remain plane (with full interaction). Output: MRd as max moment with strain limits; M-κ curve. Used for class 3/4 and for partial connection verification of ductility; also needed to feed hinge rotation capacity.

**C-11 Generic stress-block integrator [V]** — implementation support for C-07/C-08/C-12. Inputs: layered polygon list (concrete, rebar points, steel plates) each with fd and compressive/tensile activity; algorithm: find plastic NA (bisection on net axial force = 0 with N-target); returns resultant forces and moment about reference. Must produce result identical to closed-form C-07 within 1e-9 relative. (Language-agnostic test fixtures: section 7.)

**C-12 Partial shear connection in beams — stress-block method [V]** — cl. 6.2.1.3(3..), 6.6.1.2. EN: "Partial shear connection, stress-block method" / DE: "Teilverbund".
- Inputs: Nc (force in the slab = n_connectors_in_half_span × PRd), Ncf, η = Nc/Ncf.
- Formula: concrete force Nc = η Ncf (compression block depth x = Nc/(0.85 fcd beff)); steel stress state: compression in steel part Fsc = (Npl,a − Nc)/2 acting in flange/web top; tension Fst = Nc + Fsc. Resulting Mpl,Rd(η) via equilibrium (two plastic neutral axes: one in slab at x, one in steel).
- Pass: η >= ηmin (C-14) AND MEd <= MRd(η).
- Edge cases: Nc may be limited by Npl,a (then η_max=Npl,a/Ncf if Ncf>Npl,a is full); NA in web.
- Example E1-03.

**C-13 Partial shear connection — linear interpolation [V]** — cl. 6.2.1.3(3), Fig. 6.? EN: "Linear interpolation (conservative)" / DE: "Lineare Interpolation". MRd = Ma,pl,Rd + (Mpl,Rd − Ma,pl,Rd) Nc/Ncf. Applicability: conservative for all ductile connection; Ma,pl,Rd = Wpl,a fyd (bare steel). Result MUST be <= stress-block (C-12) result; the pair is a test invariant.
- Example E1-03.

**C-14 Minimum degree of shear connection [V/N]** — cl. 6.6.1.2. EN: "Minimum degree of connection" / DE: "Mindestverdübelungsgrad".
- Inputs: Le [m], fy [MPa], steel section equal flanges / bottom flange area ratio (3:1 for sagging).
- Formula (steel sections with equal flanges): Le <= 25 m: η >= 1 − (355/fy)(0.75 − 0.03 Le), η >= 0.4; Le > 25 m: η >= 1. Bottom flange 3× top flange: Le <= 20 m: η >= 1 − (355/fy)(0.30 − 0.015 Le), η >= 0.4; Le > 20: η >= 1. Interpolate for other flange ratios; applies if connectors are ductile and sheeting studs where the slab thickness… NA-DE: confirm unchanged; ADDITIONAL: connection by "non-ductile" connectors requires full connection or 6.6.1.3 provisions. Also: for studs with 16<=d<=25 and hsc/d>=4 ductile.
- Edge: ductile condition (stud: 16<=d<=25, hsc>=4d); sheeting with hp>… 6.6.1.2(4) additional restrictions when spacing; for slab with profiled sheeting rule applies as for solid.
- Example E1-03 (Le=8, fy=355: ηmin 0.49).

**C-15 Number and distribution of connectors [V]** — cl. 6.6.1.1, 6.6.1.2, 6.6.1.3. nf = Ncf/PRd (or Npl,a/PRd); required n >= η nf in each critical length (between zero-moment and max-moment, between concentrated load and support); uniform vs. non-uniform distribution allowed for ductile; non-uniform requires plastic resultants per critical section (6.6.1.2(5)): the connector force diagram follows the resistance up to the critical section.
- Output: connector count per half span, spacing table. Edge: concentrated loads, hogging regions with reinforcement, cantilever.

### 1.3 Shear connection

**C-16 Stud shear resistance PRd [V/N]** — cl. 6.6.3.1. EN: "Headed studs resistance" / DE: "Kopfbolzendübel Tragfähigkeit".
- Inputs: d [mm] (16–25), fu [MPa] (<=500), hsc [mm], fck, Ecm [MPa].
- Formulae: PRd = min(PRd,1; PRd,2): PRd,1 = 0.8 fu π d²/4 /γV; PRd,2 = 0.29 α d² sqrt(fck Ecm)/γV; α = 0.2 (hsc/d + 1) for 3 <= hsc/d <= 4; α = 1 for hsc/d > 4.
- NDP: γV recommended 1.25; NA-DE 1.25 (confirm); NA-DE adds rules for lightweight concrete (η-factor on PRd,2: PRd,2 reduced by (0.9 + 0.1 ... ρ/2200)) [T].
- Outputs: PRd,1, PRd,2, governing failure mode (stud steel vs. concrete), PRd.
- Edge: hsc/d < 3 -> out-of-scope; lightweight concrete; unequal fu; in bridges 2:6.6 apply same with additional fatigue.
- Example E1-04: PRd = 81.66 kN (d=19, fu=450, C30/37 Ecm=33000, γV=1.25).

**C-17 Stud resistance in profiled sheeting (kt, kl) [V/T]** — cl. 6.6.4.1–6.6.4.2. EN: "Studs with profiled sheeting" / DE: "Kopfbolzen im Profilblech".
- Ribs parallel to beam (6.6.4.1): PRd,sheet = kl PRd; kl = 0.6 (b0/hp)(hsc/hp − 1) <= 1.0 (for hp <= 85 mm and b0/hp >= 1.0; hsc <= hp + 75 mm).
- Ribs perpendicular (6.6.4.2): PRd,sheet = kt PRd; kt = 0.7/√nr (b0/hp)(hsc/hp − 1) <= kt,max (Table 6.2). Table 6.2: t <= 1.0 mm: nr=1: 0.85 (through-welded studs d <= 20) / 0.75 (holes); nr=2: 0.70 / 0.60. t > 1.0 mm: nr=1: 1.00 / 0.75; nr=2: 0.80 / 0.60. [T: confirm table cell values against the standard text]. Conditions: hp <= 85 mm, b0 >= hp, d <= 20 (22 for holes), hsc >= hp + 2d... ; additional: PRd uses fu and hsc as above but hsc is capped at hp + 75.
- NA-DE: restricts sheeting stud resistance additionally (kt <= value dependent on sheeting thickness, and the max number nr) [N: NA-DE ? list].
- Output: kt/kl, reduced PRd, flags.
- Edge: nr > 2 -> out-of-scope; sheeting thickness 0.9 mm; stud height hsc smaller than hp + 2d (welding) -> warning.
- Example E1-04 (kt=1.26 -> limited 0.85; PRd,sheet = 69.41 kN).

**C-18 Longitudinal shear at connectors, ULS [V]** — cl. 6.6.2, 6.6.3.1: demand per length vl,Ed = ΔFcd/Δx from the force diagram; connector spacing e <= PRd/vl,Ed. Output: required spacing. Edge: hogging regions with ductile connection; end of reinforcement anchorage.

**C-19 Longitudinal shear in slab (transverse reinforcement) [V]** — cl. 6.6.6.1–6.6.6.3, EN 1992-1-1 6.2.4. EN: "Longitudinal shear in the concrete slab" / DE: "Längsschub in der Gurtscheibe".
- Inputs: vEd = (ΔFd/Δx)/Σ(length of shear surface), hf, shear surface type (a-a, b-b, c-c, d-d with Lsurf = 2hsc… defined in Fig. 6.15 as multiples of hf and stud geometry), Asf/sf, θf, fck, fyd.
- Formulae: vEd <= vRd,max = ν fcd sinθf cosθf, ν = 0.6(1 − fck/250); vEd <= Asf fyd cotθf /(sf hf) ... resulting requirement Asf/sf >= vEd hf /(fyd cotθf); 1.0 <= cotθf <= 2.0 (compression flange), 1.0 <= cotθf <= 1.25 (tension flange). Profiled sheeting continuous across the beam contributes Ap fyp,d (6.6.6.4) when welded/anchored studs. Minimum transverse reinforcement (6.6.6.3): >= 0.002 Ac (and ≥ 0.2 % of the concrete area, NDP) wait: value 0.002 hc: NA-DE ? [N].
- Outputs: required Asf/sf [mm²/m], vRd,max utilisation.
- Edge: shear surfaces through the haunch; prestressed slab; lightweight concrete ν; sheeting acting as transverse reinforcement ("sheeting contribution" if continuous over beam / with studs through).
- Example E1-05 (vEd = 2.72 MPa; Asf/sf >= 626 mm²/m with fyd = 500/1.15 and cotθ=1.0).

**C-20 Resistance to separation / transverse reinforcement at studs [T]** — cl. 6.6.5.1–6.6.5.5, 6.6.5.2: detailing: edge distance >= 20 mm (conn. to free edge), stud spacing: longitudinal >= 5d, transverse >= 2.5d (4d if solid slab), max longitudinal spacing 6hc ≤ 800 mm (NDP), stud head cover 20 mm, hsc >= ...; welded studs hp/d limits (d <= 20 mm for through-welding, sheeting thickness max 1.25 mm). Output: detailing check list with each rule a result; fails drive layout editor feedback.

**C-21 Fatigue of studs and slab [T]** — cl. 6.8.3–6.8.7, 2:6.8. ΔτRd = ΔτC/γMf,s, ΔτC=90 MPa at 2e6 cycles, m=8; λv equivalent damage factors (bridges: 2:6.8.6.2); stud stress range Δτ = ΔVEd,E,2... ; combined with weld detail. Concrete: EN 1992-1-1 6.8; checks: γFf Δτ,E,2 <= ΔτC/γMf,s with Δτ,E,2 = λv Δτ(Qfat). Pass criterion as given; the combined check Eq. 6.? (reinforcement/steel flange + concrete compression) [T].

### 1.4 Shear, LTB, bearing

**C-22 Vertical shear resistance [V]** — cl. 6.2.2. EN: "Vertical shear" / DE: "Querkraft".
- Formulae: Vpl,a,Rd = Av fy/(√3 γM0) (Av shear area of steel section: Av = A − 2 b tf + (tw + 2r) tf for rolled I; hw tw for welded); shear buckling: if hw/tw > 72 ε/η (η = 1.2 for fy<=460, S460 and below; 1.0 above) -> Vb,Rd per EN 1993-1-5 ch. 5. Concrete contribution neglected unless the concrete is encased (6.2.2.3 non-encased: concrete ignored). V–M interaction (cl. 6.2.2.4): if VEd > 0.5 Vpl,Rd (or Vb,Rd): ρ = (2VEd/Vpl,Rd − 1)², MRd,V from reduced steel strength (1−ρ)fy on shear area; MEd <= MV,Rd. (For class 3/4 6.2.2.5 with EN 1993-1-5 η3 interaction.)
- Pass: VEd/Vpl,a,Rd <= 1; ΜEd <= MV,Rd.
- Edge: Av for welded webs; shear in hogging with reinforcement (As adds nothing); web openings.
- Example E1-06 (IPE300 S355: Av = 2568 mm², Vpl = 526.3 kN, ρ at 400 kN = 0.270).

**C-23 Lateral-torsional buckling of continuous composite beams (inverted U-frame) [T/V]** — cl. 6.4. EN: "Kippen durchlaufender Verbundträger" / DE: "Biegedrillknicken (umgekehrter U-Rahmen)".
- Inputs: hogging region geometry: Le_ltb between lateral restraints, Ea Ifz (flange weak-axis), It, Ga, slab Ib: k1 = α EaI2/a (α = 2 (end bay) /3 (equal spans)? per 6.4.2(...)), k2 = Et tw³/(4(1−νa²) hs), ks = k1 k2/(k1 + k2); Mcr = (kc C4/L)·[(Ga It + ks L²/π²) Ea Ifz]^{1/2} with C4 from Table 6.? (cross-section/load distribution), kc dependent on the geometry [T: transcribe kc from 6.4.3(?)].
- λLT = sqrt(Mpl,k/Mcr), χLT from EN 1993-1-1 curve (αLT per h/b: 0.21 or 0.34 or 0.49...), Mb,Rd = χLT Mpl,Rd/γM1.
- Simplified check (6.4.3): without calc if conditions (a) adjacent spans ratio >= 0.6, (b) UDL, (c) flanges class 1/2, (d) rolled sections up to depth limit per Table 6.? (IPE 600 / HE 800 with fy) and (e) at least 10 % transverse reinforcement ... -> LTB exempt.
- Outputs: Mcr, λLT, χLT, utilisation Mhog/Mb,Rd.
- Edge: restraint at bottom flange (bracing), haunches, stiffeners at supports; slab cracked EI2 in k1 (so k1 uses cracked stiffness).

**C-24 Stud and bearing at concentrated loads/ stiffeners, web resistance [T]** — cl. 6.2.? EN 1993-1-5 chapter 6 for patch loading with composite effect: for webs with supports use effective-slab-flange? Provide Fn,Rd check; mark [T].

### 1.5 Composite columns (cl. 6.7, simplified method)

**C-25 Scope check [V]** — cl. 6.7.1: doubly symmetric uniform section; steel contribution factor δ = Aa fyd/Npl,Rd in [0.2, 0.9]; relative slenderness λ̄ <= 2.0; for fully encased: cover limits cz <= 0.3 h, cy <= 0.4 b and cz >= 40 mm (or b/6) [T]; reinforcement ratio 0.3 % <= As/Ac <= 6 % (reinforcement contribution beyond 6 % ignored); max steel yield strength fy <= 460 MPa; local buckling limits Table 6.3: CHS d/t <= 90 ε², RHS h/t <= 52 ε, partially encased flange b/tf <= 44 ε; ε = sqrt(235/fy). Output `in-scope|out-of-scope` with the violated limit.

**C-26 Plastic resistance to compression Npl,Rd [V]** — cl. 6.7.3.2, 6.7.3.2(1),(6.30), (6.33). EN: "Plastic resistance to axial compression" / DE: "Plastische Normalkrafttragfähigkeit".
- Encased and partially encased: Npl,Rd = Aa fyd + 0.85 Ac fcd + As fsd. Filled: Npl,Rd = Aa fyd + Ac fcd + As fsd. CHS filled with confinement (when λ̄ <= 0.5 and e/d <= 0.1): Npl,Rd = ηa Aa fyd + Ac fcd (1 + ηc (t/d)(fy/fck)) + As fsd, ηa = 0.25(3 + 2λ̄) <= 1.0; ηc = 4.9 − 18.5 λ̄ + 17 λ̄² >= 0 (e = 0: ηa0 = 0.75, ηc0 = 4.9; e > 0 reduce: ηa = ηa0 + (1−ηa0)10 e/d, ηc = ηc0 (1−10 e/d)). Characteristic Npl,Rk: same with γ = 1.0 and fck, fyk (no 0.85 in filled; 0.85 not applied to γ-free).
- Output: Npl,Rd, Npl,Rk, δ.
- Edge: λ̄ > 0.5 -> no confinement (ηa=1, ηc=0); e/d > 0.1 -> no confinement; long-term influence of concrete (C-28).
- Example E1-07.

**C-27 Relative slenderness and buckling resistance [V]** — cl. 6.7.3.3, 6.7.3.5. EN: "Beulwiderstand" / DE: "Knicknachweis".
- Formulae: λ̄ = sqrt(Npl,Rk/Ncr); Ncr = π² (EI)eff /L²; (EI)eff = Ea Ia + Es Is + Ke Ecm Ic with Ke = 0.6; long-term: Ecm → Ecm,eff = Ecm/(1 + (NG,Ed/NEd) φt) (C-28); χ from EN 1993-1-1 6.3.1.2 with buckling curves of Table 6.5: curve a (α = 0.21): CHS/RHS filled with ρs <= 3 %; curve b (0.34): filled with 3 % < ρs <= 6 % and I section encased y-y; curve c (0.49): partially encased z-z; encased y-y curve b, z-z curve c; NRd = χ Npl,Rd; pass: NEd <= χ Npl,Rd.
- NA-DE: Ke = 0.6 = rec. (confirm); imperfection e0 per Table 6.5 (L/300, L/200, L/150 for a, b, c).
- Output: Ncr, λ̄, φ, χ, NRd.
- Example E1-07 (323.9×10 S355, C30/37, L=4 m: Ncr = 20863 kN, λ̄ = 0.522, χ = 0.917, NRd = 4543 kN).

**C-28 Long-term effects on columns [V/T]** — cl. 6.7.3.3(4). Ecm,eff = Ecm/(1 + (NG,Ed/NEd) φt); φt = φ(t,t0) with t0 = age of first loading; φt may be taken 0 for filled sections when λ̄ <= 0.8 (or 0.5 for encased, with considered relation) [T: exact thresholds δ... ]; used both for Ncr and for (EI)eff,II. Output: reduced Ecm.

**C-29 Interaction polygon (M–N) [V]** — cl. 6.7.3.6. EN: "Interaktionskurve N-M" / DE: "N-M-Interaktion".
- Points (normalised by Npl,Rd, Mpl,Rd): A (Npl,Rd, 0); B (0, Mpl,Rd); C (Npc,Rd, Mpl,Rd) with Npc,Rd = Ac fcd (0.85 fcd for encased? use Npm,Rd = 0.85 fcd Ac encased, Ac fcd filled); D (Npm,Rd/2, Mmax,Rd).
- Moments: Mmax,Rd = Wpa fyd + 0.5 Wpc fcd + Wps fsd (0.85 fcd for encased ⇒ 0.5·0.85); Mpl,Rd = Mmax,Rd − Mn,Rd; Mn,Rd = Wpan fyd + 0.5 Wpcn fcd + Wpsn fsd; hn: neutral-axis distance at B: filled RHS: hn = Ac fcd / (2 b fcd + 4 t (2 fyd − fcd)) (with Asn = 0); Wpcn = (b−2t) hn², Wpan = b hn² − Wpcn (− Wpsn) etc.; for CHS with e/d: use Table formulas [T: exact formulas for hn and Wpan for CHS, encased I-sections from cl. 6.7.3.2(5-7)].
- Interaction curve: μd = Md/Mpl,Rd as a piecewise-linear function of χd = NEd/Npl,Rd through A-C-D-B (the polygon A-D-C-B).
- Pass (uniaxial): MEd <= αM μd Mpl,Rd, αM = 0.9 (S235–S355), 0.8 (S420, S460). With second-order effects: MEd = k·M1 (C-30).
- Biaxial: μdy, μdz each from own polygon; checks My/(μdy Mpl,y) <= αM,y; Mz/(μdz Mpl,z) <= αM,z and My/(μdy Mpl,y)+Mz/(μdz Mpl,z) <= 1.0 (imperfection considered only in the more critical axis).
- Output: polygon vertices (N, M) per axis, μd, utilisation of both axes, rendered in 2D (section 5).
- Edge: μd > 1 allowed between D and C only where polygon exceeds Mpl,Rd (permitted up to Mmax with a limit), if bending is mainly from imperfection, μd limited to 1.0 (6.7.3.6(?)); NEd < 0.3 Npl,Rd? no extra; for tension NEd <= 0 -> use reinforcement only (out-of-scope).
- Example E1-08 (RHS 200×200×10 S355 C40/50: Npl = 3562 kN, Npm = 864 kN, Mmax = 211.85 kNm, Mpl = 206.94 kNm).

**C-30 Second-order and moment amplification [V/T]** — cl. 6.7.3.4. EN: "Theorie II. Ordnung" / DE: "Berechnung nach Theorie II. Ordnung".
- Formulae: MEd = k MEd,1 with k = β/(1 − NEd/Ncr,eff) >= 1; β = 0.66 + 0.44 r >= 0.44 (end-moment ratio r in [−1, 1]), β = 1.0 for loads along member; Ncr,eff with (EI)eff,II = K0 (Ea Ia + Es Is + Ke,II Ecm,eff Ic), K0 = 0.9, Ke,II = 0.5 (NA-DE ? confirm). Moment effect of imperfection added (member imperfection e0, equivalent lateral load) only when not accounted by buckling curve.
- Skip second order if NEd/Ncr,eff <= 0.1 (cl. 6.7.3.4(2)).
- Pass: NEd <= χ-resistance and (C-29) combined.

**C-31 Introduction of loads / shear transfer [T]** — cl. 6.7.4. τRd (Table 6.6): completely concrete-encased 0.30 MPa; completely filled 0.55 MPa; flanges of partially encased 0.20 MPa; web of partially encased 0.40 MPa? [T: swap check]; introduction length ≤ min(2d(b), L/3); studs force PRd per connector; limit for connection: Fl,Rd. Output: required connectors/length.

**C-32 Composite column fire (F:4.3–4.4, Annex G/H) [T]** — tabulated: Table 4.? minimum dimensions for filled CHS/RHS and encased; simple calc: Annex G (concrete-filled hollow sections) with Nfi,Rd = Σ (χ,θ ...). Cross-ref C-40..C-43.

### 1.6 Composite slabs (cl. 9)

**C-33 Sheeting during construction [T]** — cl. 9.3.1: ULS and SLS of profiled sheeting as formwork per EN 1993-1-3: Mpl,sheeting >= wet concrete + construction load (1.5 kN/m² over 3 m x 3 m, NDP NA-DE ? 0.75 kN/m² for the area outside); deflection limit L/180 under concrete weight (δmax <= L/180, and ≤ 20 mm unless ponding included, NDP). Output: stress/deflection utilisation.

**C-34 Composite slab bending — full/partial connection [V]** — cl. 9.7.2–9.7.3. EN: "Verbunddecke Biegung" / DE: "Biegung Verbunddecke".
- Full connection: neutral axis in concrete: Ncf = Ap fyp,d (Ap per metre); x = Ncf/(0.85 fcd b) (with b = 1000); z = h − 0.5 x − ep + (ep − e)·1; Mpl,Rd = Ncf z. Ap = effective section of sheeting (reduced by local buckling per EN 1993-1-3), ep = distance of plastic NA of sheeting from bottom, e = centroid.
- Partial connection: Nc = τu,Rd b Lx (no end anchorage) or Nc = Ncf by end-anchorage studs/deformed anchors (Nc,anchor = ΣPpb,Rd); x = Nc/(0.85 fcd b); z = h − 0.5 x − ep + (ep − e) Nc/Ncf; Mpr = 1.25 Mpa (1 − Nc/(Ap fyp,d)) <= Mpa; Mpl,Rd(Lx) = Nc z + Mpr, valid for each section at distance Lx from the free end; τu,Rd = τu,Rk/γVS.
- Hogging (continuous): reinforcement + concrete compression; rebar As fsd and 0.85 fcd concrete in the rib is neglected.
- Pass: MEd(Lx) <= Mpl,Rd(Lx) at every section, plus midspan.
- Edge: sections where Nc > Ncf cap; negative bending in decks (cracking), partially restrained supports.
- Example E1-09.

**C-35 Composite slab longitudinal shear — m–k method [V]** — cl. 9.7.3(4). EN: "Längsschub nach m-k" / DE: "m-k-Verfahren".
- Formula: Vl,Rd = (b dp/γVS) (m Ap/(b Ls) + k), with m [N/mm²], k [N/mm²] from tests (Annex B.3 evaluation: reduced by 10 %? using characteristic values), Ap [mm²/m], b = 1000 mm, dp [mm] top to centroid of sheeting, Ls shear span: UDL Ls = L/4; two symmetric point loads Ls = distance support-load; general (shear-span as measured by Annex B). γVS = 1.25.
- Pass: VEd <= Vl,Rd. For end-anchorage, formula valid with anchorage as part of resistance (Vl,Rd,anch = Vl,Rd + anchored Ppb,Rd..).
- Example E1-09: Vl,Rd = 26.0 kN/m.

**C-36 Partial connection method (τu) [V]** — see C-34; τu,Rk from tests (Annex B.3.5). Pass similarly.

**C-37 Composite slab vertical shear [V]** — cl. 9.7.5. EN: "Querkraft Verbunddecke" / DE: "Querkraft Verbunddecken".
- Formula: Vv,Rd = bo dp τRd kv (1.2 + 40 ρ); τRd = 0.25 fctk0.05/γC; kv = 1.6 − dp[m] >= 1.0; ρ = Ap/(bo dp) <= 0.02; bo: sum of minimum rib widths per metre width. Pass: VEd <= Vv,Rd.
- Example E1-09: Vv,Rd = 49.17 kN/m (bo = 400 mm, dp = 125, ρ capped at 0.02, kv = 1.475).

**C-38 Composite slab punching shear [T]** — cl. 9.7.6. Vp,Rd = Ccrit · hc · vRd,c-type resistance per EN 1992-1-1 6.4.4 with critical perimeter Ccrit taken per Fig. 9.8 (around the loaded area at 0.5 hc … ; the exact distance is the thing to be transcribed) [T]; Pass: Fp,Ed <= Vp,Rd; flags loaded area geometry; for concentrated loads on slabs (rack, wheel).

**C-39 Composite slab SLS and detailing [T]** — cl. 9.8: deflection of the slab (cracked I with reduced flexural stiffness; end slip effects ignored when end slip < 0.5 mm in tests; otherwise incl. flexibility), crack width over supports (continuous slabs: As,min = 0.2 % for unpropped, 0.4 % propped; wk limit by exposure), detailing 9.2: overall slab thickness h >= 80 mm; hc >= 50 mm; bearing length on steel 50/75 mm, on other 70/100 mm, min aggregate size. Output: items, utilisation.

### 1.7 Serviceability

**C-40 Deflection with creep/shrinkage [V]** — cl. 7.3, 7.2. EN: "Verformung" / DE: "Durchbiegung".
- Method: δ = Σ over actions δi, each with flexural stiffness EaI*(n): permanent loads n = nL (ψL = 1.1), variable loads n = n0, shrinkage: curvature κsh = εcs (Ac/ns) zc / I*(ns) with ns = n0 (1 + 0.55 φ), deflection δsh = κsh L²/8 (simply supported; general by integration over curvature distribution). Cracking effect: hogging regions (15 % span) with I2 (cracked); a combined stiffness EI over length with rule 7.3.1(6)-(7); pre-cambering; propped/unpropped; the first-stage deflection of bare steel beam.
- Limit: L/250 (permanent+variable) / L/300 (variable) NDP: NA-DE ? (per use: L/250 total, L/300 variable for floors; EN 1993-1-1 NA); consider "δ2 + δ3" definition.
- Output: δ1 (permanent steel stage), δ2, δ3, δmax, utilisation.
- Example E1-10: δ(G nL + Q n0) = 10.34 mm; shrinkage 7.42 mm.

**C-41 Stress limitation SLS [V/T]** — cl. 7.2.2: σc <= 0.6 fck (characteristic), σs <= 0.8 fsk (Hmm: 1994-1-1 recommends steel stress at SLS limited to fy under characteristic combination; reinforcement 0.8 fsk; concrete 0.6 fck for exposure XD/XF/XS) [T: NA-DE ? values]; stage-wise sum like C-09.

**C-42 Crack control: minimum reinforcement [V]** — cl. 7.4.2. EN: "Mindestbewehrung" / DE: "Mindestbewehrung".
- Formula: As,min = ks kc k fct,eff Act/σs; ks = 0.9; kc = 1/(1 + hc/(2 z0)) + 0.3 <= 1.0 (z0 = vertical distance between centroids of uncracked concrete flange and uncracked composite section, short-term n0); k = 0.8; σs = max stress allowed in rebar immediately after cracking (<= fsk; or from Table 7.1 for wk); fct,eff = fctm (or lower).
- Example E1-11: As,min = 835 mm² for Act = 2000×100, σs = 500, fct = 2.9, z0 = 250.

**C-43 Crack width / tension stiffening [T/V]** — cl. 7.4.3. Δσs = 0.4 fctm/(αst ρs) with αst = A Ia/(Aa Ia) (A,I composite uncracked with n0 ... ), ρs = As/Act; σs,0 = MEd/(Ia + Aa(...))...: σs = σs,0 + Δσs; check Table 7.1 (max bar diameter φ* for wk) & Table 7.2 (max spacing); crack width calc per EN 1992-1-1 7.3.4 optional. Pass: σs <= limit for wk (NDP NA-DE ? wk = 0.3 mm / 0.2 mm / 0.4 mm by exposure per EN 1992-1-1/NA).

### 1.8 Fire (EN 1994-1-2)

**C-44 Fire design framework [T]** — F:2, F:3. Combination Efi,d = ΣGk + ψ1,1 Qk,1 + Σψ2,i Qk,i (reduction factor ηfi = (GK+ψfiQk,1)/(γG GK + γQ,1 Qk,1)); nominal curves ISO 834 θg = 20 + 345 log10(8t + 1) (t in min), external, hydrocarbon; parametric (Annex A of EN 1991-1-2) and zone models. Output: θg(t).

**C-45 Thermal and mechanical properties [T]** — F:3.2 (steel: ky,θ, kE,θ, kp,θ tables; concrete: kc,θ, strain; reinforcement ks,θ; thermal: λc, cp,c, thermal strain; steel λa = 54 − 3.33e-2θ (θ<800) , ca polynomial, εth). Data tables loaded from `📚️tables/en1994-1-2-properties.json`.

**C-46 Tabulated data [T]** — F:4.2: Table 4.1 (composite beams with partially encased sections: min dimensions b, bc, etc. for R30–R180 with load level ηfi=0.3/0.5/0.7), Table 4.2/4.3 (filled/encased columns: b×h, As ratio, e/d, min cross-section, utilisation limit), Table D.1 (slab insulation). Pass: all geometric minima satisfied for the requested R-class and load level `ηfi`; interpolation per table rules; out-of-scope outside the ranges.

**C-47 Simple calculation models [T]** — F:4.3 (composite beams: temperature of the steel bottom flange/web/upper flange θa via Annex D (Eq. D.1: θa,t = ... ) and k-reduced moment capacities Mfi,Rd,θ using k_y,θ factors; for concrete slab heff (Annex D.7): heff = h1 + 0.5 h2 (l1+l2)/(l1+l2'...) as table), F:4.3.5 columns: Annex G (CHS/RHS filled) and Annex H (partly encased: three-step), Nfi,Rd = Σ ky,θ... and buckling reduce. Verification Efi,d <= Rfi,d(θ) for t = R duration; algorithm: time-stepping with Δt <= 5 s (EN 1993-1-2 4.2.5.1) — expensive: progress and cancellation. For slabs: insulation criterion heff >= Table D.? (30 min: 60 mm, 60: 80, 90: 100, 120: 120 …) and load-bearing Mfi,Rd with reduced sheeting (ignored) [T: confirm].

### 1.9 Bridges (EN 1994-2)

**C-48 Bridge actions and combinations [T]** — 2:2, EN 1991-2 load models (LM1/LM2/LM3/ LM71): lane definition; shrinkage, temperature (ΔTM,heat/cool, ΔTN), construction stages; effect on global analysis.
**C-49 Global analysis [T]** — 2:5.4: effective width with beff,1 mid-span and at supports; cracked analysis: 15 % each side of supports (or EI1/EI2 with iteration, 5.4.2.3: "cracked sections, where σc > 2 fctm"); shear lag; creep and shrinkage; stage-wise with 'redistribution'.
**C-50 Classification and resistance of beams, 2:6.2 [T]** — same as C-07..C-10 with additional: 2:6.2.1.5? effective flange of web width; plastic resistance only Class 1/2; elastic Class 3/4 with ‘k’ method (and 6.2.1.5 composite sections with concrete creep). Non-uniform torsion; vertical shear with web stiffeners; bending–shear (6.2.2.4): interaction with η3; interaction M–V for class 3/4 with flange contributions.
**C-51 Lateral-torsional buckling of bridges (U-frame) [T]** — 2:6.4: simplified: λLT <= 0.2 or Mpl,Rd criterion; general method: Mcr from U-frame stiffness ks (cl. as C-23, with cracking), Mb,Rd.
**C-52 Fatigue of bridges [T]** — 2:6.8: 6.8.1 detail categories (EN 1993-1-9 plus studs 90 MPa), 6.8.2 partial factors γMf (NA-DE ?), equivalent stress range Δσ,E2 = λ Φ2 ΔσLM3? with λ = λ1 λ2 λ3 λ4 (λmax) [T]; reinforcement fatigue with ΔσRsk from EN 1992-2; shear studs: λv = λv1 λv2 λv3 λv4; Δτ. Pass: γFf Δσ,E2 <= ΔσC/γMf.
**C-53 Longitudinal shear in decks, 2:6.6.2 [T]** — transverse reinforcement as C-19; concentrations at ends (pre-cracking), shear connection in negative regions; uplift 6.6.? ; deck slab cracking control with σs limit 2:7.4. SLS: stress limitation 2:7.2.2 (concrete 0.6 fck, reinforcement 0.8 fsk, steel fy), crack width for decks wk <= 0.3 mm (NA-DE ? 0.3/0.2 by exposure); minimum reinforcement 2:7.4.2 with kc per Eq. (7.? ) bridging kc = 0.9 Act... ; time-dependent: shrinkage & thermal stages.
**C-54 Bridge-specific detailing [T]** — 2:6.6.5, 2:6.7 composite plates/ box girders, 2:6.9 (concrete-filled chords). Not delivered as full 3D editors in phase 1; checks as listed.

---

## 2. EN 1999 Verification Catalogue

Numbering `A-xx`, clause ids refer to EN 1999-1-1:2007+A1:2009+A2:2013 unless prefixed (`1-2:`, `1-3:`, `1-4:`, `1-5:`).

### 2.1 Material, HAZ, classification

**A-01 Alloy/temper data [T]** — cl. 3.2, Table 3.2a (wrought: sheet/strip/plate), 3.2b (extruded), 3.2c (drawn tube), 3.2d (forgings?). EN: "Alloys and tempers" / DE: "Legierungen und Zustände".
- Dataset per row: alloy EN AW-xxxx, temper, product form (ET extruded tube, EP extruded profile, ER/B rod/bar, EO open profile, SH sheet, PL plate, ST/DT drawn tube), thickness band t [mm], fo [MPa], fu [MPa], A50 [%], buckling class BC (A/B), exponent np, HAZ ρo,haz, ρu,haz (and absolute fo,haz, fu,haz), plus ductility class and "welded?" availability. Also Table 3.2 material limits np (Ramberg-Osgood exponent) for deformation.
- Seed rows (authoring-time hand-check; full table MUST be transcribed from the standard text into `📚️tables/en1999-table-3-2.json`, plus cross-check with independent source):
  - EN AW-6082 T6, extruded (open profile), t <= 5: fo 250, fu 290, BC A, ρo,haz 0.48, ρu,haz 0.64 (fo,haz 125, fu,haz 185).
  - EN AW-6082 T6, extruded (open profile), 5 < t <= 15: fo 260, fu 310, BC A, ρo,haz 0.48, ρu,haz 0.60 (fo,haz 125, fu,haz 185).
  - EN AW-6063 T6 extruded t <= 3: fo 160, fu 195, BC B; t 3–25: 160/195? [T] ; ρo,haz 0.41, ρu,haz 0.56.
  - EN AW-6061 T6: fo 240, fu 260, BC A, ρo,haz 0.48, ρu,haz 0.67 [T].
  - EN AW-5083 H12/O (sheet/plate), 6005A T6, 5754, 3005, etc. [T].
  - Elastic constants: E = 70 000 MPa, G = 27 000 MPa, ν = 0.3, α = 23e-6 /K, ρ = 2700 kg/m³.
- Output: row selection by designation, temper, form, thickness; fo/fu/BC/np/ρ factors. Edge: thickness band edge handling (t exactly at upper limit goes to lower band); forging; welded vs. unwelded; "no data" -> out-of-scope.

**A-02 Heat-affected zone extent [V/T]** — cl. 6.1.6.3. EN: "HAZ extent bhaz" / DE: "Wärmeeinflusszone".
- Rules: bhaz (from the weld toe/axis, each side): MIG welding, t <= 6 mm: 20; 6 < t <= 12: 30; 12 < t <= 25: 35; t > 25: 40 [mm] [T: verify numbers]. TIG: bhaz = factor [T] larger; multiple welds / T-junction: HAZ extent increases by 1.33 for two-sided heat flow at T-junction and in thin plates (t<=6) at corners; interpass temperature > 60 °C (6xxx) or > 120 °C (3xxx/5xxx): bhaz × increase (bhaz,eff = bhaz [1 + (Tint − 60)/... ]) [T]; if HAZ spaced less than 3 bhaz overlapping reduce; the HAZ in an unstiffened outstand extends max to the free edge: bhaz limited to the available width; distance from end: bhaz also applies at the weld at the member end along longitudinal; in regions with longitudinal welds, HAZ width bhaz on each side of the weld bead.
- Output: HAZ polygons on each section element; Ahaz total; ρo,haz,eff, ρu,haz,eff.
- Example E2-12 (t = 8 mm MIG, plate 120×8 with one central longitudinal weld, bhaz = 30, Ahaz = 480 mm²).

**A-03 HAZ strength reduction [V]** — cl. 6.1.6.2, 6.1.6.3: fo,haz = ρo,haz fo; fu,haz = ρu,haz fu; applied to affected area: a) tension (A-12), b) compression (A-15), c) bending (A-16 with effective section modulus), d) member buckling (A-20: κ, ωx), e) connections (A-30). Natural ageing recovery of 6xxx (a few days) is NOT modelled; for 7xxx (after natural ageing) with [T]. Welded 5xxx/3xxx in non-heat-treatable alloys: ρ values from table for "O/F condition" retain.

**A-04 Cross-section classification [V/T]** — cl. 6.1.4, Table 6.2. EN: "Classification" / DE: "Querschnittsklassifizierung".
- ε = sqrt(250/fo[MPa]). β = b/t for each element, where: internal elements β = b/t (b = clear width), outstands β = b/t (b clear to free edge); stress gradient: internal elements β = η b/t, η = 0.70 + 0.30ψ (1 >= ψ > −1), η = 0.80/(1 − ψ) (ψ <= −1); outstands: β = b/t when tip in compression ... or η-based with ψ per Table [T]; element with a longitudinal HAZ: classification uses welded limits ("with welds") for unprotected elements; for elements with a transverse weld at the end of the outstand ... use the welded limit.
- Limits (× ε, unwelded / welded), Table 6.2 [T: confirm]: BC A internal: β1 11/9, β2 16/13, β3 22/18; BC A outstand: 3/2.5, 4.5/4, 6/5; BC B internal: 13/10, β2 16.5/13.5, β3 18/15; BC B outstand: 3.5/3, 4.5/3.5, 5/4. Class 1 if β <= β1ε; Class 2 if <= β2 ε; Class 3 if <= β3 ε; else Class 4. Cross-section class = worst element. Flanges with lips/stiffeners: 6.1.4.3 (single-edge stiffeners with ψ; intermediate stiffeners 6.1.5) [T]. Round tubes: D/t limits with parameter (3 ε², ... ) [T].
- Output: class per element, per section, limiting element id.
- Example E2-02 (RHS 150×150×5: β = 28 -> Class 4; I-section 80×120×10/6: flange β = 3.7 Class 2).

**A-05 Local buckling: effective thickness, ρc [V]** — cl. 6.1.5. EN: "Beulabminderung" / DE: "Lokale Beulabminderung".
- Formula: ρc = 1 for β <= β3 ε (cl. 3). For β > β3 ε (class 4): ρc = C1/(β/ε) − C2/(β/ε)²; internal elements: BC A C1 = 32, C2 = 220; BC B C1 = 29, C2 = 198; outstand elements: BC A C1 = 10, C2 = 24; BC B C1 = 9, C2 = 20; reduction ρc <= 1.0; effective thickness teff = ρc t (uniform over element). If element contains HAZ the combined reduction: teff = min(ρc, ρo,haz) t in HAZ region (cl. 6.1.5.? ) [T].
- Cut-outs/holes: net area.
- Output: Aeff, Weff,y, Weff,z with shift of centroid (Class 4).
- Example E2-02: β = 28, BC A internal: ρc = 32/28 − 220/784 = 0.8622.

**A-06 Distortional/stiffened elements [T]** — cl. 6.1.4.3–6.1.5.3 (edge/intermediate stiffeners, effective thickness of stiffened plates; 6.1.4.3 "Elements with edge stiffeners, ψ-reduction" and Annex J (torsional) cross-references). Implementation: iteration for stiffener spring (EN 1999-1-1 6.7.? / EN 1999-1-4 5.5 for cold-formed).

### 2.2 Cross-section resistance (cl. 6.2)

**A-10 General yielding vs. ultimate [V]** — cl. 6.2.1: Fed check design: resistances with fo/γM1 for general yielding and fu/γM2 for net/necked; Ramberg-Osgood and 'plastic' reference not used.

**A-11 Tension, gross section [V]** — cl. 6.2.3. No,Rd = Ag fo/γM1 (general yielding, only valid in regions without HAZ). Output No,Rd. Pass: NEd <= min(No,Rd, Nu,Rd, Nu,Rd,haz).

**A-12 Tension, net section and HAZ [V/T]** — cl. 6.2.3. EN: "Zug mit Nettoquerschnitt und WEZ" / DE: "Zug, Nettoquerschnitt, WEZ".
- Nu,Rd = 0.9 Anet fu/γM2 (net section at holes; 0.9 for fastener holes where tearing), with HAZ: Nu,Rd,haz = Aeff,u fu/γM2 where Aeff,u = Ag − Ahaz (1 − ρu,haz) (transverse weld across full width: Aeff = ρu,haz Ag ...); yielding in HAZ: No,Rd,haz = [Ag − Ahaz(1 − ρo,haz)] fo/γM1; for members with transverse welds the global yielding verification (6.2.3(3)) requires the elongation consideration: No,Rd computed with Ag when HAZ length is short [T: exact rule for "localised weld"].
- Output: No,Rd, Nu,Rd, NRd = min.
- Edge: staggered holes (Anet with s²/4p); angles connected through one leg (cl. 8.5.2.3 [T]); HAZ at hole row: use bhaz limited to element width.
- Example E2-03: flat 100×10 6082-T6 across-weld: NRd = 113.45 kN (yield), Nu = 148.8 kN.

**A-13 Compression [V/T]** — cl. 6.2.4. EN: "Druck" / DE: "Druck". Nc,Rd = Aeff fo/γM1 (Aeff from A-05; for Class 1–3 with welds: Aeff = A − Σ ... HAZ; for non-welded class 1–3: Aeff = A). Pass: NEd <= Nc,Rd (no instability). Edge: heavy hole loaded rivet (no reduction).
- Example E2-02: 150×150×5 RHS: Aeff = 2514.3 mm², Nc,Rd = 571.4 kN. Example E2-12: plate with HAZ: Nc,Rd = 167.9 kN.

**A-14 Bending [V/T]** — cl. 6.2.5. EN: "Biegung" / DE: "Biegung".
- Formula: MRd = α Wel fo/γM1 for Class 1–3 where α = shape factor: Class 1 and 2: α = Wpl/Wel limited to αmax(Table 6.4); Class 3: α = 1 + (β3 − β)/(β3 − β2) (α2 − 1) [T: α3 formula]; Class 4: α = Weff/Wel with Weff from A-05. With HAZ: reduce Wel,haz: Wel,haz = Wel − Σ Δ... (reduced section moduli, i.e. replacing teff in HAZ by ρo,haz t; fo and fo,haz).
- Output: MRd,y, MRd,z, section classes.
- Example E2-04 (I 80×120×10/6, 6082-T6 5<t<=15: Wpl = 103000 mm³, Wel = 89222 mm³, α = 1.154; MRd = 24.345 kNm).

**A-15 Shear [V/T]** — cl. 6.2.6. VRd = Av fo/(√3 γM1) with Av = Σ over webs; for stocky webs: [T: effective area of shear] and for slender webs (hw/tw > 39ε? unwelded...) shear-buckling: A-30 plated. Pass: VEd <= VRd. With HAZ: replace fo by fo,haz in HAZ regions.
- Example E2-05: VRd = 81.88 kN.

**A-16 Torsion [V/T]** — cl. 6.2.7. St. Venant: TEd <= TRd = Wt fo/(√3 γM1); warping: stress addition with σw = Bw/Iw ωn; closed sections: Wt = 2 Am tmin; open: Wt = It/tmax; with combined: 6.2.7.2 interaction of shear+torsion: VEd <= VRd reduced by (1 − τt,Ed√3γM1/fo) ... [T].

**A-17 Combined bending and axial [T]** — cl. 6.2.9. Class 1/2: (NEd/NRd)^ψ + [My,Ed/My,Rd]^{ ... } formulas with exponents ψ, ξ, η: Class 1 and 2 bending+axial: [NEd/NRd]^ψ + [My,Ed/My,Rd]^η + [Mz,Ed/Mz,Rd]^ξ <= 1 with ψ, η, ξ per Table 6.? e.g. 1.3/ 1.5... (dependent on shape; for I sections ψ=1, η=... ) [T]; Class 3: elastic stress-based: NEd/NRd + My/MRd,y + Mz/MRd,z <= 1; Class 4: Eq with Aeff and Weff plus eccentricity shifts eNy, eNz (cl. 6.2.9.3 with ΔMy = NEd eNy). Output: utilisation per formula.

**A-18 Combined shear/bending/axial [T]** — cl. 6.2.8: if VEd <= 0.5VRd ignore; else use reduced yield (1 − ρ) fo with ρ = (2VEd/VRd − 1)² on the shear area.

**A-19 Holes/openings and local loads [T]** — cl. 6.2.11–6.2.12: web bearing (cl. 6.7.5 'patch loading' for I-sections, FRd = ... ), flange-induced buckling, crippling. Cross-check with 1-5? Provide formulas as [T].

### 2.3 Member stability (cl. 6.3)

**A-20 Flexural buckling [V/T]** — cl. 6.3.1.1–6.3.1.3. EN: "Biegeknicken" / DE: "Biegeknicken".
- NRd = κ χ ω_x Aeff fo/γM1; λ̄ = sqrt(Aeff fo/Ncr) (class 4: Aeff; class 1–3: A... with generalised) ; Ncr = π² E I/Lcr²; φ = 0.5 (1 + α (λ̄ − λ̄0) + λ̄²); χ = 1/(φ + sqrt(φ² − λ̄²)) <= 1.0; BC A: α = 0.20, λ̄0 = 0.10; BC B: α = 0.32, λ̄0 = 0.00; for welded members with longitudinal weld (and no local weld): κ... (A-21).
- Torsional and torsional-flexural buckling (cl. 6.3.1.4): λ̄T = sqrt(Aeff fo/Ncr,TF), Ncr,TF from the standard formulas (shear centre eccentricity): α, λ̄0 as BC with λ̄0 = 0.20 and α depends BC; use Annex I.
- Pass: NEd <= NRd.
- Edge: λ̄ <= λ̄0 -> χ = 1; Class 4 with Aeff at yield (not at critical); thin-wall tubes. Example E2-06: RHS 100×100×5, L = 2.5 m: Ncr = 316.8 kN, λ̄ = 1.2245, φ = 1.3622, χ = 0.5105, NRd = 220.44 kN.

**A-21 Welded members: HAZ reduction factors κ, ωx (ηc not in this clause) [T]** — cl. 6.3.1.1(?) and 6.3.3.3. Table gives κ for non-welded 1.0 and for longitudinally welded: κ = 1 − (1 − κ0)... where κ0 = ρo,haz? [T]; localized transverse weld: ω_x = 1/(χ + (1 − χ) sin(π xs/lc)^c) <= 1 with xs distance of weld to midspan, c depends [T]; for members with transverse welds at ends ωx = 1. All numeric tables to be transcribed; implement as pure functions with tests via the clause examples + Python reference.

**A-22 Lateral-torsional buckling [V/T]** — cl. 6.3.2. EN: "Biegedrillknicken" / DE: "Biegedrillknicken".
- MRd = χLT Wy fo/γM1 where Wy = Wpl,y α-shape: Class 1/2: Wpl·; class 3: Wel·α3; class 4: Weff; χLT = 1/(φLT + sqrt(φLT² − λ̄LT²)) <= 1; φLT = 0.5(1 + αLT (λ̄LT − λ̄0,LT) + λ̄LT²); class 1/2: αLT = 0.10, λ̄0,LT = 0.6; class 3/4: αLT = 0.20, λ̄0,LT = 0.4; λ̄LT = sqrt(αW Wel fo/Mcr) with α Wel = Wpl (class 1/2), Wel (class 3), Weff (class 4); Mcr = C1 π²EIz/L² sqrt(Iw/Iz + L² G It/(π² E Iz)) (+ za/zg terms from Annex I with end restraints, kz kw); Mcr by Annex I formulas for non-uniform moment/bracing (C1 factor table).
- Edge: closed sections (RHS, CHS) with high Iz: LTB can be ignored when λ̄LT <= λ̄0,LT (hollow sections: LTB neglected by cl. 6.3.2.1(4) for λ̄LT small); with HAZ: ωx,LT (cl. 6.3.3.3(4)).
- Example E2-07: I 80×120×10/6, L = 2 m uniform moment: Iz = 855 133 mm⁴, It = 60 533 mm⁴, Iw = 2.5813e9 mm⁶, Mcr = 17.53 kNm, λ̄LT = 1.236, χLT = 0.594, Mb,Rd = 14.45 kNm.

**A-23 Interaction: axial compression plus bending [T]** — cl. 6.3.3 (Method with η0, γ0, ξ0 exponents; Annex H interaction factors for the alternative). Form (cl. 6.3.3.1 equation family): [NEd/(χy ω_{x,y} NRd)]^{ηc} + [My,Ed/(ω_{x,LT} MRd,y)]^{γc} + [Mz,Ed/MRd,z]^{ξc} <= 1.0 and the z-analogue; ηc = η0 χz^?, γc = γ0 , ξc = ξ0 χz ... with η0, γ0, ξ0 in Table 6.? for Class 1/2 (and 1.0 for class 3/4) [T]; Method B (Annex H factors kyy, kzy, kyz, kzz). The implementation MUST ship Annex H factors as a table-driven function. Output: utilisation per formula, governing interaction.
- Edge: tension plus bending (cl. 6.3.3.3?) -> use A-17; biaxial.

**A-24 Members with local transverse HAZ (ωx) and torsional buckling [T]** — cl. 6.3.3.3, Annex? cross-ref A-21.

### 2.4 Plated structures (cl. 6.7)

**A-25 Plate girders: shear buckling [T]** — cl. 6.7.4 (web of plate girders). VRd = vw (hw tw fo)/(√3 γM1) with vw from slenderness λw = 0.35 (hw/tw) sqrt(fo/E) with tension-field contribution via flange and transverse stiffeners; transverse stiffener & rigid end post cases. Table values [T].
**A-26 Plate girders: bending with web slenderness [T]** — cl. 6.7.2–6.7.3: reduced section modulus Weff via ρc on web & flange (effective thickness); flange-induced web buckling: hw/tw limit.
**A-27 Patch loading/web bearing [T]** — cl. 6.7.5.
**A-28 Stiffeners [T]** — cl. 6.7.6 (transverse stiffener checks, flexural), 6.7.7 (longitudinal stiffeners).

### 2.5 Connections (cl. 8)

**A-30 Bolted connections [V/T]** — cl. 8.5. EN: "Bolzenverbindung" / DE: "Schraubenverbindung".
- Shear: Fv,Rd = 0.6 fub As/γM2 (threaded shear plane, 8.8) ; 0.6 fub A/γM2 (shank), classes 4.6/5.6/8.8/10.9, stainless A2-70/A4-70, aluminium bolts (e.g. 6082-T6: fub = 310 [T]).
- Bearing: Fb,Rd = k1 αb fu d t/γM2; αb = min(e1/(3 d0), p1/(3 d0) − 0.25, fub/fu, 1.0); k1: edge bolts min(2.8 e2/d0 − 1.7, 2.5), inner bolts min(1.4 p2/d0 − 1.7, 2.5); with countersunk bolts reduced (cl. 8.5.5 [T]).
- Tension: Ft,Rd = 0.9 fub As/γM2; combined shear + tension: Fv,Ed/Fv,Rd + Ft,Ed/(1.4 Ft,Rd) <= 1.0; punching Bp,Rd = 0.6 π dm tp fu/γM2.
- Spacing: e1 >= 1.2 d0, e2 >= 1.2 d0, p1 >= 2.2 d0, p2 >= 2.4 d0; max e1 <= 4t + 40 mm; p1,max = min(14 t, 200 mm) compression; unequal.
- Block tearing (8.5.2.3) [T], group effects, slip-resistant bolting (prestressed) [T].
- Example E2-08: M16 8.8 in 8 mm 6082-T6: Fv,Rd = 60.29 kN; Fb,Rd = 58.79 kN (e1 = 40, p1 = 60, e2 = 30, d0 = 18).

**A-31 Rivets [T]** — cl. 8.5.? solid and blind rivets: Fv,Rd = 0.6 fur A0/γM2 for 6082 rivets (fur 160 [T]); bearing same as bolts; pitch/edge limits (e1 >= 1.2 d0 …).

**A-32 Welded connections incl. HAZ [V/T]** — cl. 8.6. EN: "Schweißverbindungen" / DE: "Schweißverbindungen".
- Fillet weld resistance: directional method (cl. 8.6.3.3): √(σ⊥² + 3(τ⊥² + τ∥²)) <= fw/(γMw?) hmm: aluminium: Fw,Rd = fw a /γMw per length where fw = weld metal strength: Table 8.8 (ER 4043A, 5356, 5183 filler: fw 110-210 [T]) with γMw = 1.25; HAZ failure at fusion boundary: fo,haz, fu,haz => Fw,Rd = min(weld metal, HAZ shear: ρu,haz fu/(√3 γM2) × (length leg), ...). Longitudinal/transverse welds with βw factors (cl. 8.6.3.1); weld length minimum 8 a or 40 mm; effective throat a: Fillet a = 0.7 leg; full penetration butt weld: t.
- NA-DE: γMw = 1.25 = rec.; list of fillers per DIN EN 1011-4 [N: NA-DE ?].
- Pass: Fw,Ed <= Fw,Rd,min.
- Edge: welded connection of unequal alloys: use the lower strength; welded in tension on HAZ.

**A-33 Hybrid/other connectors [T]** — cl. 8.5.10 adhesives, 8.9 pin joints, 8.8 lug; 8.10 connections with bolted steel plates; 8.? long joints (βLf reduction for long bolt groups (L > 15 d): βLf = 1 − (Lj − 15 d)/(200 d)) with 0.75 <= βLf <= 1.0.

### 2.6 Fatigue (EN 1999-1-3)

**A-40 Detail categories and S-N curves [V/T]** — 1-3:Annex J (detail catalogue Table J.1–J.8), 1-3:6. EN: "Ermüdung" / DE: "Ermüdung".
- Detail category Δσc (at NC = 2e6) with slope m1 (e.g. 71-3.4, 80-4.3, 45-3.4 notation "Δσc − m"): S-N: N Δσ^{m1} = NC Δσc^{m1} for Δσ >= ΔσD; N Δσ^{m2} = ND ΔσD^{m2} for ΔσL <= Δσ < ΔσD, m2 = m1 + 2 (stated [T]); ΔσD = Δσc (NC/ND)^{1/m1} with ND = 5e6; ΔσL = ΔσD (ND/NL)^{1/m2}, NL = 1e8; below ΔσL damage zero.
- Plain detail table loaded from data. Stress range: Δσ from nominal/geometric (for welded joints: ranges in the plate; for transverse bolts: net section).
- Example E2-09: Δσc = 71, m1 = 3.4: ΔσD = 54.23 MPa, ΔσL = 31.14 MPa.

**A-41 Fatigue verification [V/T]** — cl. 1-3:6.2. Damage-tolerant: γFf Δσ,E,2 <= Δσ,c/γMf; safe-life; equivalent constant amplitude range Δσ,E,2 = λ × Δσ(Qk) (bridges, cranes) [T]; Palmgren-Miner: D = Σ ni/Ni <= 1.0 (design life). γMf per Table (damage tolerant / safe life × low/high consequence) [T/N]. Rainflow/reservoir counting as processing of stress histories (cancellation + progress).
- Example E2-09: 60 MPa × 1e6 cycles -> N = 3.545e6, D1 = 0.282; 40 MPa × 2e6 cycles -> N = 2.586e7, D2 = 0.0773; D = 0.359 <= 1.

### 2.7 Fire (EN 1999-1-2)

**A-50 Thermal action and material [V/T]** — 1-2:3, 4. Properties: ca = 0.41 θa + 903 J/(kg K) (alloy 6xxx range; [T] per alloy group); λa = 0.07 θa + 190 (3xxx/5xxx) or 0.07 θa +  (6xxx) [T]; ρa = 2700; emissivity εres = εm for unprotected aluminium (0.3 bright / 0.7 painted [N]); strength reduction k0.2proof,θ and ku,θ per alloy and θ (Table 3.1, 3.2) [T].
**A-51 Temperature of unprotected members [V/T]** — 1-2:4.2.5: Δθa,t = ksh (Am/V)/(ca ρa) hnet,d Δt with hnet,d = hnet,c + hnet,r; hnet,c = αc (θg − θa); hnet,r = Φ εres σ[(θr + 273)^4 − (θa + 273)^4]; Δt <= 5 s; ksh = 1.0 for aluminium (conservative) [T]; protected members (insulation) formula with φ = (cp ρp/ca ρa) dp Ap/V (EN 1999-1-2 4.3.4) [T].
- Example E2-10: first step at t=5 s, θg = 96.54 °C: Δθa = 0.856 K.
**A-52 Fire resistance [V/T]** — 1-2:4.2.1: Rfi,d,t = ... with ratio k0.2proof,θ at critical temperature; checks: Efi,d <= Rfi,d,t (Class 1–3 cross-section with fo → k0.2proof,θ fo; class 4 using effective section at elevated T); member buckling at fire: χfi with λ̄θ = λ̄ sqrt(k0.2proof,θ/kE,θ) [T]; critical temperature θcr (iterate) for utilisation μ0 = Efi,d/Rd,20°C. Pass: θa,max(t) <= θcr(μ0) for t = required R.

### 2.8 Cold-formed aluminium (EN 1999-1-4)

**A-60 Effective cross-section and stiffened elements [T]** — 1-4:5.1–5.5: sheeting with thickness 0.5..6 mm; flat element with edge/intermediate stiffeners (spring stiffness K from geometry); effective widths ρ = ...; profile corner round-up; interaction with eff. A, W; distortional buckling χd via λ̄d.
**A-61 Resistance: bending, web crippling, shear [T]** — 1-4:6.1 etc. MRd = Weff fo/γM1 (with Weff at max stress); web crippling RwRd = k α ... (6.1.7); combined bending + crippling (6.1.11); shear VRd = hw/sinφ tw fv/γM1 with fv = 0.9? 0.58 fo(1 − 0.1...) per λw [T].
**A-62 Purlins/sheeting: SLS [T]** — deflection with effective I_eff(σ) and local ponding; fastener spacing; load tables from tests (Annex A).

### 2.9 Shells (EN 1999-1-5)

**A-70 Cylindrical shell buckling [V/T]** — 1-5:6–7: classical elastic critical stress axial: σx,Rcr = 0.605 E t/r (for medium-length cylinders ω = L/√(rt) in range 1.7 <= ω <= 0.5 r/t); Annex: expressions for short/long cylinders (Cx factor); relative slenderness λ̄x = sqrt(fo/σx,Rcr); buckling reduction χx = 1 for λ̄x <= λ̄x0; χx = 1 − β(λ̄x − λ̄x0)/(λ̄p − λ̄x0) for ...; χx = α/λ̄x² for λ̄ >= λ̄p, with imperfection parameter α (fabrication quality class A/B/C) [T]; also circumferential compression (external pressure) and shear (torsion) with analogous formulas; interaction exponents (cl. 8). Output: σx,Rd = χx fo/γM1.
- Example E2-11: r = 500 mm, t = 3 mm, E = 70 000: σx,Rcr = 254.1 MPa; λ̄x (fo = 200) = 0.887.
**A-71 Shell geometry and stiffeners [T]** — 1-5:5 (stiffened shell: ring stiffeners, orthotropic smearing), 1-5:9 (shell with thickness steps, Annex C).
**A-72 Welded shells with HAZ [T]** — 1-5 with ρo,haz of A-03 on axial compression resistance (reduction of fo by HAZ for fabricated tanks).

### 2.10 Serviceability

**A-80 Deflection [V/T]** — cl. 7.2: elastic EI (using Ieff when class 4, bf by effective width), limits L/250..L/360 by use (NDP NA-DE ?), vibration (7.2.3). Output: δmax, utilisation.
**A-81 Temperature and creep [T]** — cl. 2.3.5.? thermal expansion α = 23e-6; creep of aluminium negligible except at elevated T (>100 °C): excluded.

---

## 3. NDP Registry (selection)

Format: clause | NDP | recommended | NA-DE.

| Clause | NDP | Rec. | NA-DE |
|---|---|---|---|
| 1994-1-1 2.4.1.1 | γV | 1.25 | 1.25 |
| 1994-1-1 2.4.1.2 | γM,Mf, γVS | 1.0 (γMf: 1.0 / 1.15) , γVS 1.25 | 1.25 |
| 1994-1-1 5.4.2.2 | ψL | 1.1/0.55/1.5 | = rec. |
| 1994-1-1 6.6.3.1 | γV; lightweight PRd,2 reduction | 1.25 | 1.25; [N] reduction ? |
| 1994-1-1 6.6.4.2 | kt,max table 6.2 | per Table | NA-DE ? (additional limit) |
| 1994-1-1 6.6.6.3 | min transverse reinforcement | 0.002 | NA-DE ? |
| 1994-1-1 6.7.3.4 | K0, Ke,II | 0.9, 0.5 | NA-DE ? |
| 1994-1-1 6.7.3.3 | Ke | 0.6 | NA-DE ? |
| 1994-1-1 6.7.3.6 | αM | 0.9/0.8 | NA-DE ? |
| 1994-1-1 7.2.2 | stress limits | 0.6 fck / 0.8 fsk | NA-DE ? |
| 1994-1-1 7.3.1 | deflection limits | L/250 | NA-DE ? |
| 1994-1-1 9.? | sheeting construction load 1.5 kN/m² | 1.5 | NA-DE ? |
| 1994-2 6.8.2 | γMf, γFf | 1.0, 1.0 | NA-DE ? |
| 1999-1-1 6.1.3 | γM1, γM2 | 1.1, 1.25 | 1.1, 1.25 |
| 1999-1-1 8.1.1 | γMw | 1.25 | 1.25 |
| 1999-1-1 8.2 | bolt/rivet material | per Table | NA-DE ? |
| 1999-1-3 | γMf | table | NA-DE ? |
| 1999-1-4 | γM1, γM2 | 1.1/1.25 | NA-DE ? |
| 1999-1-5 | γM0, γM1 | 1.1 | NA-DE ? |

The code MUST represent each NDP as `NdpValue { id, clause, recommended, national: Option<per-annex>, status: Confirmed|NeedsConfirmation }`; `NeedsConfirmation` values are displayed with a visible badge in the editor and block "certified" reports.

---

## 4. Schema-First Entity Models

All schemas are authored in the schema layer first (`🧬️schema`), code derives from them. Names map onto existing `En1994Snapshot`/`En1999Snapshot` vocabulary; new fields append. Quantity types carry units; stored SI. Events (CQRS): each entity has commands (`Add/Move/Set…/Remove…`) emitting events; no CRUD entity writers.

### 4.1 EN 1994 model

```
En1994Document
  annex: AnnexChoice (DE | recommended)
  materials: ConcreteGrade[], SteelGrade[], ReinfGrade[], SheetingGrade[], StudGrade[]
  creepShrinkage: CreepShrinkageSet[]          // phi(t,t0), epsilon_cs, psi_L per action, RH, h0, cement class
  members: CompositeBeam[] | CompositeColumn[] | CompositeSlab[] | BridgeGirder[]
  actionCases: ActionCase[]                    // stage, kind, category, characteristic magnitudes
  combinations: Combination[]                  // ULS-fund, SLS-char, quasi-permanent, fatigue, fire
  fire: FireDesignCase[]
  results: (derived, not persisted)

CompositeBeam
  id; span: Length; support: SupportScheme; zones: MomentZone[]          // sagging/hogging zero-moment points, Le per Fig 5.1
  steel: SteelSection (rolled|welded, plates with per-plate fy), haunch?: Haunch
  slab: Slab { kind: Solid|Profiled|Precast, h, hc, concrete: ConcreteGradeRef,
               width: { b1, b2, b0, edge: Free|End|Intermediate }, beffMode: Auto|Override }
  sheeting: ProfiledSheeting { profile, hp, pitch, b0 (mean rib width), bBottom, reEntrant: bool, t, Ap, ep, e, fyp, orientation: Perp|Para|Angle(alpha) }
  reinforcement: ReinfLayout { top: Bar[], bottom: Bar[], transverse: TransverseReinf[] }
  studs: StudLayout { kind: Headed, d, hsc, fu, weldedThrough: bool,
                      rows: [{x_start, x_end, nPerRib, nr, spacing}] , zones: ConnectorZone[] }   // PRd per zone computed
  construction: ConstructionStageSet { propped: bool, props: Prop[], stages: Stage[] }
  restraints: LateralRestraint[] { x, kind: Rigid|UFrame|Torsional }
  bracing: UFrameData { a, k1, k2 }
  fatigue: FatigueSpectrum?, crack: CrackControl { wkLimit, exposure }, sls: SlsLimits
  actions: ActionRef[]

CompositeColumn
  id; length: Length; kind: Encased | PartiallyEncased | FilledCHS | FilledRHS
  steel: SteelSection; concrete: ConcreteGradeRef; rebar: RebarLayout { barDia, cover, count, positions }
  fillGeometry: { outerSize, wall, cornerRadius?, cover cy, cz }
  bucklingCurves { y, z }, effectiveLength { Ly, Lz }, imperfection
  endMoments: [{N, My_top, My_bot, Mz_top, Mz_bot}], longTerm: { NG, phi_t }
  loadIntroduction: Option<LoadIntroduction>
  fire: FireData

CompositeSlab
  id; span: Length; support: Simple|Continuous; spans[]; sheeting: ProfiledSheeting
  concrete: { hc, grade }; mkTest: { m, k, tauU, source }; endAnchorage: Option<{kind,count,PpbRd}>
  rebar: ReinfLayout; loads: LoadPattern (UDL|line|punctual with footprint)
  construction: { wetConcrete, constructionLoad, ponding }

BridgeGirder (EN 1994-2)
  spans: Span[]; girders: SteelSection[]; deck: Slab; cracking: CrackedZoneRule; creep stages; vehicleLoads: LM1|LM2|LM3|LM71
```

Results model (derived, never stored): `CheckResult { clauseId, name: LocalizedLabel, status, utilisation, inputs: Quantity[], intermediate: Quantity[], resistance: Quantity, demand: Quantity, remedy: Remedy[] }` (use `semio_s_plugin_norm_compliance` types; clause ids in the registry). Visual payloads derived per check: `Section2D`, `BeamScene3D`, `StressBlock2D`, `InteractionPolygon2D`, `HeatmapField`.

### 4.2 EN 1999 model

```
En1999Document
  annex; materials: AlloyRef { designation, temper, form, thickness band -> Table 3.2 row (read-only derived) }
  sections: Extrusion | PlatedSection | ColdFormedProfile | Tube
  members: AluminiumMember[]; connections: AluminiumConnection[]
  hazLayouts: HazLayout[]; fatigueDetails: FatigueDetail[]; fireScenarios; shells; coldFormed

Extrusion
  id; outline: Polyline2D (closed paths, holes), elements: PlateElement[] (derived by decomposition or manual)
  PlateElement { id, a: Point, b: Point, thickness, kind: Internal|Outstand|TaperedFlange, stiffeners: Stiffener[], bcOverride? }
  derived: A, Iy, Iz, It, Iw, Wel, Wpl, shear centre, classification, Weff, Aeff

HazLayout
  welds: WeldRun[] { path3D, process: MIG|TIG, interpass, tjunction: bool, bhazOverride? }
  derived: HazZone[] { elementId, tStart, tEnd, width, factors rho_o, rho_u }

PlateGirder
  web, flanges, transverse stiffeners[], longitudinal stiffeners[], endposts, panels: Panel[] (a, hw, tw)

AluminiumMember
  id; section; material; length; support; Lcr { y, z, t, LT }; restraints[];
  weldsOnMember: HazLayout; actions[]; imperfection bow; deflection limit

AluminiumConnection
  id; kind: Bolted|Riveted|Welded|Mixed; geometry: BoltPattern { d, d0, grade, rows[], pitches, edges }, WeldRun[]
  material per part; load vector per action

FatigueDetail
  id; detail: DetailRef(1999-1-3 catalogue); history: StressHistory|Spectrum; nCycles; consequence: Low|High; strategy: DamageTolerant|SafeLife

ColdFormedProfile, ShellModel { geometry: Cylinder|Cone|Sphere, r, t(z), L, boundary: BC1|BC2|BC3, stiffeners, fabrication class A|B|C, loads }
```

### 4.3 Table data as schema

Standard tables are data, not code: `📚️tables/` JSON with a JSON schema (`row`: key fields, `source: {standard, clause, table, row}`, `status: verified|needs-review`). The implementation must refuse to certify with `needs-review` rows.

### 4.4 Events (examples)
`BeamSpanSet`, `SteelSectionChanged`, `StudRowAdded`, `StudRowMoved`, `SheetingProfileSelected`, `SlabThicknessChanged`, `ReinforcementLayerAdded`, `ConstructionStageAdded`, `ColumnShapeChanged`, `PolygonQueryPointMoved`; `ExtrusionVertexMoved`, `PlateThicknessChanged`, `WeldRunAdded`, `HazOverrideSet`, `MemberLcrChanged`, `BoltPatternChanged`, `FatigueSpectrumLoaded`. All carry author, causality, and are replayed into the snapshot.

---

## 5. Visualisation Specification

Renderer: use the shared 3D engine and 2D chart facilities used by `🏗️fem` and `🏙️bim` (no new runtime dependency). Every 3D scene has a defined camera preset set, a semantic layer toggle list (A11y: each layer is a labelled toggle), a keyboard navigation map, and a data-table fallback.

### 5.1 3D composite beam (EN 1994 beams)
Scene graph: steel I (extruded or welded plates, classification colour tint), slab as extruded polygon: solid, or profile-extruded sheeting (rib shape with re-entrant option, pitch, hp, b0), concrete above ribs (hc), reinforcement bars (instanced cylinders), studs (instanced cylinder+head, at each rib position, one or two per rib, coloured by utilisation of zone), effective-width ghost (translucent) with beff dimension line, plastic neutral axis plane (translucent horizontal plane intersecting the section with draggable handle; the PNA depth label shows x or z from top), elastic neutral axis plane (dashed), support symbols, lateral restraints. Overlays: force-flow arrows (Ncf, Npl,a, Nc), connector force diagram as bar chart under the beam (demand vs PRd per zone), stress diagrams at a draggable cut plane (steel stress, concrete stress, strain), shear surfaces a-a..d-d outlined and colour-mapped by vEd/vRd, deflection exaggerated shape with creep/shrinkage components.
Interactions: drag the PNA plane -> shows equilibrium residual (live), drag stud rows along the beam, add/remove studs with snap, drag sheeting orientation handle, scrub construction stage timeline, section cut plane.

### 5.2 Composite column cross-section + M–N polygon (2D)
Left: cross-section drawing (CHS/RHS filled, encased I with rebar, partially encased) with dimension handles, plastic stress-block overlay at the selected point (A, B, C, D), strain plane. Right: interaction chart (N vertical or horizontal as in the code figure; axes χ = N/Npl,Rd and μ = M/Mpl,Rd) showing polygon A-C-D-B (+ the reduced αM polygon), demand points (NEd, MEd) as markers for each load combination, with the safe/unsafe hatch; second-order amplified point (k) with arrow; for biaxial: a pair of charts + combined indicator. Slenderness panel: λ̄ vs χ buckling curve with the member's point. Accessibility: chart has table alternative listing vertices and demands.

### 5.3 Aluminium extrusion 3D with HAZ
Extruded member in 3D (length axis), cross-section outline with element ids and thickness; HAZ zones as translucent coloured bands on surfaces (colour ramp by ρo,haz: dark = low), weld run centerlines with bead; distance dimensioning bhaz (dimension handle); effective section ghost (class 4, ρc applied as thinned plates); local buckling mode shapes (half-wavelength animated sine wave on a plate element, internal element and outstand variants), flexural buckling shape (half sine), LTB shape (twist + lateral) with amplitude slider; classification colour mode (class 1–4). Section editor: polyline vertex dragging, wall thickness handles, preset library (I, H, box, T, channel, tube, multi-chamber), snap/grid, symmetry mode.

### 5.4 Utilisation heat map
Per member scalar field along length (x) and over the section perimeter (point-wise utilisation): bending/shear/axial/interaction, with the governing check id at each x; colour scale accessible (viridis + diverging for margin), legend with numeric stops; picking shows full check breakdown; selectable load combination and envelope. Also a table listing max utilisation per check, sortable.

### 5.5 Other 2D charts
Stress blocks (beam, plastic equilibrium), M–N polygon, buckling curves χ(λ̄), S–N curve with detail categories and spectrum bins, Mcr vs L curve, deflection vs time (creep), m–k regression, HAZ strength map, fire temperature–time and k0.2proof,θ versus θ with θcr marker, effective width distribution along the span (Le zones).

---

## 6. Realtime Editor Interactions and Live Targets

Principles: edit in the viewport, no raw JSON; every command is an event; re-evaluation is incremental (dependency graph of checks), cancellable and progress-reporting for expensive ones (time-stepping fire, nonlinear M-κ, FE-like). Latency budgets (desktop reference hardware): trivial closed-form checks (<2 ms each; whole beam check set < 20 ms), M–N polygon + polygon draw < 8 ms, 3D rebuild on parameter drag at 60 fps (instancing for studs, partial geometry update), HAZ recomputation < 5 ms, S–N damage with 1e5 cycles of history < 150 ms (progress + cancel beyond 300 ms), fire time-stepping (Δt 5 s, 2 h) < 400 ms (cancel/progress), non-linear M–κ < 100 ms; results must stay consistent after cancellation (last full result remains displayed, stale marked).

Editor interactions (EN 1994):
1. Drag span handle / support markers -> Le zones, beff, Ls update live.
2. Select section from catalogue (IPE/HEA/HEB/…, welded builder) with autocomplete; change grade -> fy by thickness band.
3. Drag slab thickness / hc / hp; sheeting profile gallery with geometry thumbnails.
4. Stud layout editor: add row, drag row along beam, multiplicity per rib, auto-layout to η >= ηmin ("Fill to degree of connection" command); instant PRd, kt, η, nf.
5. Plastic NA gizmo (drag) with equilibrium residual; stage timeline scrubbing with stresses.
6. Reinforcement layers draggable in hogging region; As,min indicator, crack check.
7. Column: shape morph (encased -> filled), size handles, rebar placement; polygon + demand markers update live as NEd/MEd sliders move; "what-if" slider for L and buckling curve.
8. Slab: m–k parameters, Ls auto, point load footprint for punching, continuous slab spans.
9. Fire: R-class selector shows tabulated verdict instantly and the simple calc after debounce 150 ms.
10. Inline utilisation chips next to every input; failing checks expose remedy actions (e.g. "add 6 studs", "increase hc to 105 mm", "use S355", "add transverse bar Ø10/150") as one-click commands (preview in viewport before apply).
11. Undo/redo across events; multi-user presence cursors; local-only vs shared state distinction.
12. Report window: clause-by-clause report with formulas, substituted numbers, units, status; export PDF/CSV; language toggle EN/DE.

Editor interactions (EN 1999): extrusion vertex drag with live section properties + class + Weff; weld run placement by clicking on faces; HAZ width override; slider for member length / support condition; bolt pattern designer (rows, pitches, edges) with live spacing code limits and capacities; S–N detail picker with 3D detail icon; shell radius/thickness/BC with live buckling stress; alloy/temper picker filtering by product form and thickness (invalid combos disabled with reason).

Accessibility/i18n: all gizmos keyboard-operable (arrow keys step, shift fine), screen reader announcements of utilisation changes (throttled), colour-blind-safe palettes, no meaning by colour only; EN first, DE second, no default language.

Live-recompute targets summary:

| Interaction | Re-evaluation set | Budget |
|---|---|---|
| Drag stud count | C-14..C-18, C-12/13, C-19 | <5 ms |
| Drag slab depth | C-03, C-07, C-09, C-16, C-19, C-40, C-42 | <15 ms |
| Drag PNA | C-11 residual only | <1 ms |
| Column size | C-25..C-30 + polygon | <10 ms |
| Extrusion vertex | A-04, A-05, A-14.. section props | <10 ms |
| Weld move | A-02, A-03, A-12..A-22 | <10 ms |
| Spectrum edit | A-40/41 | <150 ms |
| R-class change | C-46 immediate, C-47 debounced | <1 ms / <400 ms |

---

## 7. Worked Examples EN 1994 (hand-checked)

Each example is a test fixture: inputs, expected outputs with tolerance (1e-6 relative on closed forms; 0.5 % on iterated). Every value reproduced independently with a separate Python script during authoring. Third-party oracle: Python oracle in `🔮️oracles/` MUST recompute from the formulas using only `math`; additionally cross-check with the sections libraries where available (e.g., `pycba`, `sectionproperties` for Iy/Wpl). Constants: Ea = 210 000 MPa, fy = 355 MPa, γM0 = 1.0, C30/37: fck = 30, fcd = 20, 0.85 fcd = 17 MPa, Ecm = 33 000 MPa, fctm = 2.9, fctk0.05 = 2.0.

**E1-01 Effective width (C-03)** — Le = L = 8.0 m simply supported; b1 = b2 = 1.5 m (beam spacing 3 m); b0 = 0. be1 = min(8000/8, 1500) = 1000 mm; beff = 0 + 2·1000 = **2000 mm**. Variant: end support with Le = 8, bei = 1000: βi = 0.55 + 0.025·8000/1000 = 0.75; beff,end = 2·0.75·1000 = 1500 mm. Variant L = 16 m: be = min(2000, 1500) = 1500; beff = 3000 mm.

**E1-02 Plastic moment, full connection (C-07)** — IPE 300, S355 (A = 5381 mm², h = 300, b = 150, tf = 10.7, tw = 7.1, Wpl = 628.4e3 mm³), solid slab hc = 100 mm, hp = 0, beff = 2000 mm. Npl,a = 5381·355 = 1 910.255 kN; Ncf = 17·2000·100 = 3400 kN > Npl,a -> NA in slab; x = 1 910 255/(17·2000) = 56.18 mm; Mpl,Rd = 1 910.255 kN × (150 + 100 − 28.09) mm = **423.90 kNm**. Ma,pl,Rd = 628.4e3·355 = 223.08 kNm.

**E1-03 Partial connection (C-12, C-13, C-14)** — same section; η = 0.5: Nc = 955.13 kN; linear: MRd = 223.08 + (423.90 − 223.08)·0.5 = **323.49 kNm**. Stress block: x = 955 128/(17·2000) = 28.09 mm; Fsc = (1 910.255 − 955.128)/2 = 477.56 kN; compression depth in flange = 477 564/(355·150) = 8.97 mm (< tf, so NA in flange); MRd,sb = **364.35 kNm** > 323.49 (invariant: stress-block >= linear). ηmin (Le = 8 m, fy = 355): 1 − (355/355)(0.75 − 0.24) = **0.49**. With PRd = 81.66 kN: nf = 1 910.255/81.66 = 23.39 -> 24 per half span for full connection; η = 0.5 -> 12 studs per half span (n >= 0.49 × 23.39 = 11.46 -> 12).

**E1-04 Stud resistance (C-16, C-17)** — d = 19 mm, fu = 450 MPa, hsc = 100 (solid), fck = 30, Ecm = 33 000, γV = 1.25: hsc/d = 5.26 > 4 -> α = 1; PRd,1 = 0.8·450·283.529/1.25 = **81.656 kN**; PRd,2 = 0.29·361·sqrt(30·33 000)/1.25 = **83.332 kN**; PRd = 81.656 kN (stud failure). Profiled sheeting perpendicular, hp = 50, b0 = 100, hsc = 95, nr = 1, t = 0.9 mm, through-welded: kt = 0.7/1·(100/50)·(95/50 − 1) = 1.26 -> kt,max(t <= 1 mm, nr = 1) = 0.85 -> kt = 0.85; PRd,sheet = 0.85·81.656 = **69.41 kN** (hsc = 95: α = 1 since 95/19 = 5).

**E1-05 Longitudinal shear in slab (C-19)** — studs e = 150 mm, single row, PRd = 81.656 kN: vl = 81 656/150 = 544.4 N/mm; shear plane a–a (2 hf, hf = 100): vEd = 544.4/(2·100) = **2.722 MPa**. ν = 0.6(1 − 30/250) = 0.528; vRd,max = 0.528·20·sin45°cos45° = 5.28 MPa -> passes (0.52). Asf/sf >= vEd hf/(fyd cotθ) with fyd = 500/1.15 = 434.78, cotθ = 1.0: 2.722·100/434.78 = 0.626 mm²/mm = **626 mm²/m**.

**E1-06 Vertical shear and interaction (C-22)** — IPE 300 S355: Av = 5381 − 2·150·10.7 + (7.1 + 2·15)·10.7 = 2 567.97 mm²; Vpl,a,Rd = 2 567.97·355/√3 = **526.33 kN**; hw/tw = 278.6/7.1 = 39.24 < 72ε/η = 72·0.8137/1.2 = 48.82 -> no shear buckling. VEd = 400 kN > 0.5·526.33 = 263.2 -> ρ = (2·400/526.33 − 1)² = **0.2704**; reduced fy on shear area = 0.7296·355 = 259.0 MPa.

**E1-07 Filled CHS column (C-25..C-28)** — CHS 323.9×10 S355 (Aa = 9 861.46 mm², Ia = 121.583e6 mm⁴), concrete C30/37 inside (Ac = 72 535.6 mm², Ic = 418.690e6 mm⁴), L = 4000 mm pinned, γ's 1.0 (steel), 1.5. (EI)eff = 210 000·121.583e6 + 0.6·33 000·418.690e6 = 2.5532e13 + 8.2901e12 = 3.3822e13 N mm²; Ncr = π²·3.3822e13/4000² = **20 863 kN**. Npl,Rk with ηa = 1, ηc = 0 (the converged λ̄ = 0.5216 exceeds 0.5, so confinement is off; note the quadratic ηc is also 0 for 0.456 <= λ̄ <= 0.633 and MUST be applied only for λ̄ <= 0.5): Npl,Rk = 3 500.8 + 2 176.1 = 5 676.9 kN; **λ̄ = sqrt(5 676.9/20 863) = 0.5216** > 0.5 -> confinement not applied. Npl,Rd = 3 500.8 + 72 535.6·20/1000 = 3 500.8 + 1 450.7 = **4 951.5 kN**; δ = 3 500.8/4 951.5 = 0.707 ∈ [0.2, 0.9]; d/t = 32.4 <= 90·235/355 = 59.6. Curve a (ρs = 0, α = 0.21): φ = 0.5(1 + 0.21(0.5216 − 0.2) + 0.2721) = 0.6698; χ = 1/(0.6698 + sqrt(0.6698² − 0.2721)) = **0.9174**; NRd = 0.9174·4 951.5 = **4 542.6 kN**. Variant: λ̄ <= 0.5 would give ηa = 0.25(3 + 2λ̄), ηc = 4.9 − 18.5λ̄ + 17λ̄² >= 0.

**E1-08 M–N polygon (C-29)** — filled RHS 200×200×10 S355 (sharp corners simplification), C40/50 (fcd = 26.667): Aa = 7 600, Ac = 32 400 mm²; Wpa = 200·200²/4 − 180·180²/4 = 542 000 mm³; Wpc = 180·180²/4 = 1 458 000 mm³. Npl,Rd = 7 600·355 + 32 400·26.667 = **3 562 kN**; Npm = Npc = 864 kN; hn = Ac fcd/(2 b fcd + 4 t(2 fyd − fcd)) = 864 000/(10 666.7 + 40·683.33) = 22.74 mm; Wpcn = 180·22.74² = 93 071 mm³; Wpan = 200·22.74² − 93 071 = 10 327 mm³; Mmax = 542 000·355 + 0.5·1 458 000·26.667 = **211.85 kNm**; Mn = 10 327·355 + 0.5·93 071·26.667 = 4.911 kNm; Mpl = **206.94 kNm**. Vertices: A (3562, 0), C (864, 206.94), D (432, 211.85), B (0, 206.94). Lookup: NEd = 1 700 kN (between C and A): μd = (3 562 − 1 700)/(3 562 − 864) = **0.690**; MRd = αM μd Mpl = 0.9·0.690·206.94 = 128.5 kNm (αM = 0.9, S355). At NEd = 600 kN (between D and C): μ = 1.0237 + (1 − 1.0237)(600 − 432)/432 = 1.0145.

**E1-09 Composite slab (C-34, C-35, C-37)** — h = 150 mm, hp = 50, hc = 100, C30/37, sheeting Ap = 1 100 mm²/m, fyp,d = 350 MPa (γap = 1), ep = 20, e = 25 mm, dp = h − e = 125 mm, b = 1000. Full connection: Ncf = 385 kN/m; x = 385 000/(0.85·20·1000) = 22.65 mm; z = 150 − 11.32 − 20 + (20 − 25)(1) = 113.68 mm; Mpl,Rd = **43.77 kNm/m**. Partial (Nc = 0.6 Ncf = 231 kN/m): x = 13.59 mm; z = 150 − 6.79 − 20 + (20 − 25)(0.6) = 120.21 mm; Mpa = 4.0 kNm/m (given); Mpr = 1.25·4.0·(1 − 0.6) = 2.0; Mpl,Rd = 231·0.12021 + 2.0 = **29.77 kNm/m**. m–k: m = 200, k = 0.04, Ls = L/4 = 1000 mm (L = 4 m UDL), γVS = 1.25: Vl,Rd = 1000·125/1.25·(200·1100/(1000·1000) + 0.04) = 100 000·0.26 N/m = **26.0 kN/m**. For q = 7 kN/m², VEd = 14.0 kN/m -> utilisation 0.54. Vertical shear: bo = 400 mm/m, ρ = 1 100/(400·125) = 0.022 -> capped 0.02; kv = 1.6 − 0.125 = 1.475; τRd = 0.25·2.0/1.5 = 0.3333; Vv,Rd = 400·125·0.3333·1.475·(1.2 + 0.8) = **49.17 kN/m**.

**E1-10 SLS deflection (C-06, C-40)** — IPE 300 + beff 2000 × hc 100 (solid), L = 8 m, G = 6 kN/m (long-term), Q = 4 kN/m. n0 = 6.364; φ = 2.5; nL = 6.364(1 + 1.1·2.5) = 23.86; ns = 6.364(1 + 0.55·2.5) = 15.11. Uncracked I(n0) = **293.53e6 mm⁴**, I(nL) = **221.62e6 mm⁴** (centroid depth below slab top 79.2 / 128.2 mm). δ = 5L⁴/(384 Ea)(G/I(nL) + Q/I(n0)) = **10.34 mm** (L/774). Shrinkage ε_cs = 0.3 per mille: I(ns) = 247.61e6, zc = 57.82 mm, κ = 0.3e-3·(2000·100/15.11)·57.82/247.61e6 = 9.27e-7 1/mm; δsh = κL²/8 = **7.42 mm** (simply supported, no cracking).

**E1-11 Crack control As,min (C-42)** — hogging: Act = 2000·100 mm², fct,eff = 2.9 MPa, z0 = 250 mm, hc = 100: kc = 1/(1 + 100/500) + 0.3 = 1.133 -> 1.0; ks = 0.9, k = 0.8, σs = 500 MPa: As,min = 0.9·1.0·0.8·2.9·200 000/500 = **835.2 mm²**.

**E1-12 Fire tabulated and simple (C-46, C-47)** — slab R60: heff >= value from table (data load [T]); the numeric fixtures MUST be built from the transcribed tables and cross-checked by hand once tables are loaded: until then this example is an acceptance placeholder requiring a values pass. ISO 834: θg(t = 60 min) = 20 + 345·log10(481) = **945.3 °C**; θg(30) = 20 + 345·log10(241) = **841.8 °C**; θg(5 s) = 96.54 °C (used by A-51).

**E1-13 LTB U-frame (C-23)** — acceptance example is built from the transcribed kc/C4: fixture to be completed when the clause text equations are loaded [T]; the simplified exemption check is testable now: IPE 400, S355, adjacent span ratio >= 0.6, UDL, class 1/2 -> exempt if the span of the hogging region satisfies the Table 6.? limit [T].

---

## 8. Worked Examples EN 1999 (hand-checked)

Constants: E = 70 000, G = 27 000, ν = 0.3, γM1 = 1.1, γM2 = 1.25. Alloy EN AW-6082 T6 extruded. Rows: t <= 5 mm: fo = 250, fu = 290, BC A; 5 < t <= 15 mm: fo = 260, fu = 310, BC A; ρo,haz = 0.48; ρu,haz = 0.64 (t <= 5) / 0.60 (5 < t <= 15) [T: verify table rows].

**E2-01 Table 3.2 and HAZ factors (A-01, A-03)** — row 6082-T6 EP 5<t<=15: fo = 260, fu = 310, fo,haz = 0.48·260 = **124.8** MPa, fu,haz = 0.60·310 = **186.0** MPa (table lists 125/185; the table values win; factors derived are cross-check only).

**E2-02 Classification and local buckling (A-04, A-05, A-13)** — RHS 150×150×5 (t = 5: fo = 250, ε = 1.0, BC A), internal clear width b = 140: β = 28 > β3 = 22 -> Class 4. ρc = 32/28 − 220/28² = 1.142857 − 0.280612 = **0.86224**. A = 150² − 140² = 2 900 mm²; Aeff = 2 900 − 4·140·5·(1 − 0.86224) = **2 514.3 mm²**; Nc,Rd = 2 514.3·250/1.1 = **571.4 kN**. Sub-example: RHS 100×100×5 (b = 90, β = 18): Class 3 (16 < 18 <= 22). I 80×120×10/6, 5<t<=15: ε = 0.98058; flange outstand β = (80 − 6)/2/10 = 3.7: β1ε = 2.94 < 3.7 <= β2ε = 4.41 -> Class 2; web β = η b/t = 0.4·100/6 = 6.67 <= 11ε -> Class 1; section Class 2.

**E2-03 Tension with HAZ (A-12)** — flat 100×10 6082-T6 (5<t<=15), full-width transverse weld: Ahaz = 1000 mm²: No,Rd,haz = 1000·0.48·260/1.1 = **113.45 kN**; Nu,Rd,haz = 1000·0.60·310/1.25 = **148.80 kN**; NRd = 113.45 kN. Comparison unwelded: No,Rd = 1000·260/1.1 = 236.36 kN, Nu,Rd = 0.9·1000·310/1.25 = 223.2 kN.

**E2-04 Bending Class 2 (A-14)** — I 80×120×10/6 (b = 80, h = 120, tf = 10, tw = 6): Iy = 80·120³/12 − 74·100³/12 = 5.3533e6 mm⁴; Wel = 89 222 mm³; Wpl = 80·10·110 + 6·100²/4 = 103 000 mm³; α = 1.1544; MRd = 1.1544·89 222·260/1.1 = **24.345 kNm** (= Wpl fo/γM1).

**E2-05 Shear (A-15)** — same section: Av = tw·hw = 6·100 = 600 mm²: VRd = 600·260/(√3·1.1) = **81.88 kN**.

**E2-06 Flexural buckling (A-20)** — RHS 100×100×5 (Class 3, fo = 250), L = 2.5 m pinned: I = (100⁴ − 90⁴)/12 = 2.8658e6 mm⁴; Ncr = π²·70 000·2.8658e6/2500² = **316.79 kN**; λ̄ = sqrt(1900·250/316 788) = **1.2245**; BC A: α = 0.20, λ̄0 = 0.10: φ = 0.5(1 + 0.2·1.1245 + 1.4994) = 1.3622; χ = 1/(1.3622 + sqrt(1.3622² − 1.4994)) = **0.5105**; NRd = 0.5105·1900·250/1.1 = **220.44 kN** (no HAZ: κ = 1).

**E2-07 Lateral-torsional buckling (A-22)** — I 80×120×10/6, L = 2000 mm uniform moment, fork supports: Iz = 2·10·80³/12 + 100·6³/12 = 855 133 mm⁴; It = (2·80·10³ + 100·6³)/3 = 60 533 mm⁴; Iw = tf b³/24 (h − tf)² = 2.5813e9 mm⁶; Mcr = π²EIz/L² sqrt(Iw/Iz + L²G It/(π²EIz)) = **17.53 kNm**; Class 2: λ̄LT = sqrt(103 000·260/17.53e6) = **1.2360**; αLT = 0.10, λ̄0,LT = 0.6: φLT = 0.5(1 + 0.1·0.6360 + 1.5277) = 1.2957; χLT = **0.5937**; Mb,Rd = 0.5937·103 000·260/1.1/1e6 = **14.45 kNm**.

**E2-08 Bolted joint (A-30)** — M16 8.8 (As = 157, fub = 800), plate 8 mm 6082-T6 (fu = 310), d0 = 18, e1 = 40, p1 = 60, e2 = 30: Fv,Rd = 0.6·800·157/1.25 = **60.29 kN**; αb = min(40/54, 60/54 − 0.25, 800/310, 1) = 0.7407; k1 = min(2.8·30/18 − 1.7, 2.5) = 2.5; Fb,Rd = 2.5·0.7407·310·16·8/1.25 = **58.79 kN**; governing 58.79 kN. Tension Ft,Rd = 0.9·800·157/1.25 = 90.4 kN.

**E2-09 Fatigue (A-40, A-41)** — detail 71-3.4: Δσc = 71, m1 = 3.4, m2 = 5.4, ND = 5e6, NL = 1e8: ΔσD = 71·(2/5)^{1/3.4} = **54.23 MPa**; ΔσL = 54.23·(0.05)^{1/5.4} = **31.14 MPa**. Spectrum: block 1 Δσ = 60 MPa, n = 1e6: N = 2e6·(71/60)^3.4 = **3.545e6**, D1 = 0.2821; block 2 Δσ = 40 MPa, n = 2e6 (below ΔσD): N = 5e6·(54.23/40)^5.4 = **2.586e7**, D2 = 0.0773; D = **0.3594** <= 1 (pass; remaining life factor 2.78). All with γFf = 1 and γMf = 1 (confirm per consequence class).

**E2-10 Fire first step (A-51)** — unprotected section Am/V = 200 m⁻¹, θa(0) = 20 °C, ISO 834 gas, Δt = 5 s (t = 0.0833 min): θg = 20 + 345·log10(8·0.0833 + 1) = 96.54 °C; ca = 0.41·20 + 903 = 911.2 J/kgK; αc = 25 W/m²K; εres = 0.3: hnet = 25·76.54 + 0.3·5.67e-8·(369.69⁴ − 293⁴) = 2 105.3 W/m²; Δθa = 1.0·200/(911.2·2700)·2 105.3·5 = **0.856 K** (ksh = 1.0 [T]).

**E2-11 Shell elastic critical stress (A-70)** — cylinder r = 500 mm, t = 3 mm, medium length: σx,Rcr = 0.605·70 000·3/500 = **254.1 MPa**; λ̄x = sqrt(200/254.1) = **0.8872**; the χx evaluation requires the transcribed α, λ̄x0, β, η for fabrication class [T].

**E2-12 HAZ extent and effective area (A-02, A-13)** — plate 120×8 (6082-T6, 5<t<=15, fo = 260) with one centred longitudinal MIG weld: 6 < t <= 12 -> bhaz = 30 mm each side (60 total) [T]; Ahaz = 60·8 = 480 mm²; Aeff = 960 − 480·(1 − 0.48) = **710.4 mm²**; NRd = 710.4·260/1.1 = **167.91 kN**. T-junction variant: bhaz × 1.33 = 39.9 mm (Ahaz = 638.4 mm²), Aeff = 960 − 638.4·0.52 = 628.0 mm², NRd = 148.4 kN [T].

(Total: 13 EN 1994 and 12 EN 1999 worked examples; E1-12, E1-13, E2-11 are partially [T] and MUST be completed after table transcription.)

---

## 9. Test and Oracle Plan

1. Language-agnostic JSON fixtures per catalogue item (`🧪️tests/fixtures/<id>.json`): inputs, expected outputs (with units), tolerance, status expectation. Every worked example above is one fixture; add negative fixtures (out-of-scope, fail, edge).
2. Third-party reproduction: Python oracles in `🔮️oracles/` implement each formula independently with `math`/`numpy` only; additional libraries as references: `sectionproperties` (steel/aluminium section properties, warping Iw, It), `scipy` (root-finding for plastic neutral axis, polygon), `numpy` for S-N/Miner; third-party composite examples (EN 1994 design guide numbers; ECCS publications) as published datasets are cross-checks with source citations in the fixture.
3. Table-load tests: each `📚️tables` row must carry `source` and `status`; CI fails if a `needs-review` row is used by a `certified` check.
4. Invariants: stress-block MRd >= linear MRd; η from Nc/Ncf in [0, 1]; polygon vertices monotone; χ <= 1; ρc <= 1; HAZ factors in (0, 1]; Σ forces = 0 at the PNA within 1e-9.
5. Property tests: dimension scaling; unit round-trips (mm/m, N/kN); monotonicity (larger section -> not lower resistance except through class change).
6. UI tests: edit -> recompute budgets (section 6); accessibility checks; EN/DE label presence (no missing label keys).

## 10. Implementation Order (suggested)
1. Tables and NDP registry (data + schema + `needs-review` gate). 2. Section models (steel, extrusion with decomposition into plates) and stress-block integrator. 3. EN 1994 beam ULS (C-03, C-07, C-12/13/14, C-16/17, C-19, C-22) + viewport. 4. Columns (C-25..C-30) + polygon chart. 5. Slabs (C-33..C-39). 6. SLS (C-40..C-43). 7. EN 1999 HAZ + classification + resistance (A-01..A-17) + 3D extrusion/HAZ. 8. Stability (A-20..A-24), plated (A-25..A-28), connections (A-30..A-33). 9. Fatigue, fire, cold-formed, shells. 10. Bridges and fire for composite.

## 11. Open Items Requiring Norm-Text Transcription (`[T]` summary)
Table 6.2 kt,max cells; stud lightweight PRd,2 reduction; Table 6.6 τRd; kc and C4 of the U-frame LTB (6.4.3); composite column hn/Wpan formulae for CHS and encased; punching perimeter (9.7.6); EN 1994-1-2 tables (4.1–4.3, D.1) and Annexes D, G, H; EN 1994-2 fatigue λ factors; all EN 1999 tables (3.2, 6.2 Class limits confirm, 6.4 αmax, interaction exponents of 6.3.3, κ/ωx table of 6.3.1/6.3.3.3, bhaz values for TIG and T-junction, plate girder shear coefficients, weld filler strengths, bolt/rivet materials); EN 1999-1-2 alloy property tables; EN 1999-1-3 S-N catalogue and γMf; EN 1999-1-4 effective width rules; EN 1999-1-5 α/β/η tables; all NA-DE `?` entries. No test may rely on a `[T]` value until transcribed and double-checked by a second reviewer.
