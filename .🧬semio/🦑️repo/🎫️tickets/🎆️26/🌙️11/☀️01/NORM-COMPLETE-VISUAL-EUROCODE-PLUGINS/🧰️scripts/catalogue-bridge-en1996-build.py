"""Builds catalogue-bridge-en1996-inputs.tsv: EN 1996 evaluators work in SI and name the unit in the key suffix (_m, _pa, _n, _nm, _m2, _m3, _n_m3)."""
SUFFIX = [("_n_m3", "N_m3", "N/m³"), ("_nm", "Nm", "N·m"), ("_m2", "m2", "m²"), ("_m3", "m3", "m³"), ("_pa", "Pa", "Pa"), ("_n", "N", "N"), ("_m", "m", "m")]
VECTORS = {"M-21": ["l_m", "t_m", "x_m", "vrd_n"], "M-42": ["bar_y_m", "bar_a_m2"]}
ROWS = """a1_m|Distance a₁ from the wall end|Abstand a₁ vom Wandende|high
a_m2|Cross-section area A|Querschnittsfläche A|high
a_theta_m|Reduced thickness a_θ in fire|Reduzierte Dicke a_θ im Brandfall|medium
ab_m2|Loaded area A_b|Belastete Fläche A_b|high
anchor_rk_n|Characteristic anchor resistance R_k|Charakteristische Ankertragfähigkeit R_k|high
ap_m2|Prestressing steel area A_p|Spannstahlfläche A_p|high
as_m2|Reinforcement area A_s|Bewehrungsfläche A_s|high
asw_m2|Shear reinforcement area A_sw|Querkraftbewehrungsfläche A_sw|high
av_m|Shear span a_v|Schubspannweite a_v|high
b_m|Width b|Breite b|high
basement|Basement wall (1) or not (0)|Kellerwand (1) oder nicht (0)|high
bar_a_m2|Bar area A_s of bar i|Stabfläche A_s des Stabes i|high
bar_y_m|Bar position y of bar i|Stablage y des Stabes i|high
beam_a_m2|Beam cross-section area|Balkenquerschnittsfläche|medium
bearing_m|Bearing length|Auflagerlänge|high
bed_joint_m|Bed joint thickness|Lagerfugendicke|high
cavity_m|Cavity width|Schalenabstand|high
chase_depth_m|Chase depth|Schlitztiefe|high
class|Execution class|Ausführungsklasse|medium
col_a_m2|Confining column cross-section area|Querschnittsfläche der Einfassungsstütze|high
col_as_m2|Confining column reinforcement area|Bewehrungsfläche der Einfassungsstütze|high
col_min_m|Minimum column dimension|Kleinstes Stützenmaß|high
col_spacing_m|Column spacing|Stützenabstand|high
cover_m|Concrete or mortar cover|Betondeckung oder Mörteldeckung|high
cpe|Pressure coefficient c_pe|Druckbeiwert c_pe|high
d_m|Effective depth d|Nutzhöhe d|high
deflection_m|Deflection|Durchbiegung|high
density|Density class or value|Rohdichteklasse oder -wert|low
diameter_m|Bar diameter|Stabdurchmesser|high
e0_m|Eccentricity e₀|Exzentrizität e₀|high
e_pa|Modulus of elasticity E|Elastizitätsmodul E|high
edges|Edge support code (F, S, X: top, bottom, left, right)|Randlagerung (F, S, X: oben, unten, links, rechts)|high
ehe_m|Eccentricity e_he|Exzentrizität e_he|medium
ep_pa|Prestressing modulus E_p|Elastizitätsmodul E_p der Spannstähle|medium
eps_cs|Shrinkage strain ε_cs|Schwinddehnung ε_cs|high
exposure|Exposure class|Expositionsklasse|high
fb_decl_pa|Declared unit strength f_b|Deklarierte Steinfestigkeit f_b|high
fb_pa|Normalised unit strength f_b|Normierte Steinfestigkeit f_b|high
fbok_pa|Characteristic bond strength f_bok|Charakteristische Verbundfestigkeit f_bok|high
fd_pa|Design compressive strength f_d|Bemessungsdruckfestigkeit f_d|high
fk_pa|Characteristic compressive strength f_k|Charakteristische Druckfestigkeit f_k|high
flange_t_m|Flange thickness|Flanschdicke|high
flanges|Number of flanges|Anzahl der Flansche|high
fm_pa|Mortar compressive strength f_m|Mörteldruckfestigkeit f_m|high
fpk_pa|Characteristic prestressing steel strength f_pk|Charakteristische Spannstahlfestigkeit f_pk|high
frost_class|Frost resistance class|Frostwiderstandsklasse|medium
fvk0_pa|Initial shear strength f_vk0|Haftscherfestigkeit f_vk0|high
fvk_pa|Characteristic shear strength f_vk|Charakteristische Schubfestigkeit f_vk|high
fxk1_pa|Flexural strength f_xk1 (parallel to bed joints)|Biegezugfestigkeit f_xk1 (parallel zur Lagerfuge)|high
fxk2_pa|Flexural strength f_xk2 (perpendicular to bed joints)|Biegezugfestigkeit f_xk2 (senkrecht zur Lagerfuge)|high
fyd_pa|Design yield strength f_yd|Bemessungsstreckgrenze f_yd|high
gamma_m|Partial factor γ_M|Teilsicherheitsbeiwert γ_M|high
gamma_soil_n_m3|Unit weight of the soil γ|Wichte des Bodens γ|high
group|Masonry unit group|Steingruppe|high
h_m|Wall height h|Wandhöhe h|high
h_trib_m|Tributary height|Einflusshöhe|high
h_wall_m|Wall height|Wandhöhe|high
hc_m|Height of the load above the support|Höhe der Last über dem Auflager|medium
he_m|Retained height h_e|Anschütthöhe h_e|high
head_joint|Head joint type|Stoßfugenart|high
hef_m|Effective height h_ef|Knicklänge h_ef|high
height_total_m|Total height|Gesamthöhe|high
hole_pct|Hole volume in percent|Lochanteil in Prozent|high
hydrostatic|Groundwater at the wall (1) or not (0)|Grundwasser an der Wand (1) oder nicht (0)|high
k0|At-rest earth pressure coefficient k₀|Ruhedruckbeiwert k₀|high
k_e|Creep eccentricity factor k_e|Beiwert k_e der Kriechausmitte|medium
k_theta|Fire reduction factor k_θ|Brandminderungsbeiwert k_θ|medium
l_m|Length l|Länge l|high
la_m|Arching span l_a|Gewölbespannweite l_a|medium
lb_avail_m|Available anchorage length|Vorhandene Verankerungslänge|high
lc_m|Wall length in compression|Überdrückte Wandlänge|medium
lefm_m|Effective length of the load spread|Mitwirkende Länge der Lastausbreitung|medium
limit_ratio|Limit ratio span/deflection|Grenzverhältnis Stützweite/Durchbiegung|high
m_bot_nm|Bending moment at the bottom|Biegemoment am Fuß|high
m_ed_nm|Design bending moment M_Ed|Bemessungsmoment M_Ed|high
m_hm_nm|Moment at mid-height from the horizontal load|Moment in Wandmitte aus Horizontallast|medium
m_mid_nm|Bending moment at mid-height|Biegemoment in Wandmitte|high
m_top_nm|Bending moment at the top|Biegemoment am Kopf|high
material|Unit material|Steinmaterial|high
mortar|Mortar type|Mörtelart|high
mu_fi|Fire load level μ_fi|Brandlastgrad μ_fi|high
n_bot_n|Axial force at the bottom|Normalkraft am Fuß|high
n_ed_fi_n|Axial force in fire N_Ed,fi|Normalkraft im Brandfall N_Ed,fi|high
n_ed_n|Design axial force N_Ed|Bemessungsnormalkraft N_Ed|high
n_max_n|Maximum axial force|Größte Normalkraft|high
n_mid_n|Axial force at mid-height|Normalkraft in Wandmitte|high
n_min_n|Minimum axial force|Kleinste Normalkraft|high
n_per_m2|Wall ties per m²|Drahtanker je m²|high
n_pier_n|Axial force in the pier|Normalkraft im Pfeiler|high
n_top_n|Axial force at the top|Normalkraft am Kopf|high
ned_c_n|Concentrated load N_Ed,c|Einzellast N_Ed,c|high
opening_w_m|Opening width|Öffnungsbreite|high
overlap_m|Unit overlap|Überbindemaß|high
p0_n|Initial prestressing force P₀|Anfangsvorspannkraft P₀|high
phi|Capacity reduction factor Φ|Abminderungsbeiwert Φ|high
phi_inf|Final creep coefficient φ_∞|Endkriechzahl φ_∞|high
pier_area_m2|Pier area|Pfeilerfläche|high
pier_l_m|Pier length|Pfeilerlänge|high
plaster_m|Plaster thickness|Putzdicke|high
provided_m|Provided anchorage length|Vorhandene Verankerungslänge|high
q|Behaviour factor q|Verhaltensbeiwert q|high
qk_pa|Characteristic surcharge q_k|Charakteristische Auflast q_k|high
qp_pa|Peak velocity pressure q_p|Böengeschwindigkeitsdruck q_p|high
rei|Fire resistance class REI in minutes|Feuerwiderstandsklasse REI in Minuten|high
rho2|Support condition factor ρ₂|Beiwert ρ₂ der Lagerung|medium
rho_e_n_m3|Unit weight of the backfill ρ_e|Wichte der Hinterfüllung ρ_e|high
rho_relax|Relaxation loss ρ|Relaxationsverlust ρ|medium
ring_tie_n|Ring beam tie force|Zugkraft des Ringbalkens|high
s_anchor_m|Anchor spacing|Ankerabstand|high
s_m|Spacing s|Abstand s|high
sides|Number of supported sides|Anzahl der gehaltenen Ränder|high
sigma_d_pa|Design compressive stress σ_d|Bemessungsdruckspannung σ_d|high
sigma_pa|Compressive stress σ|Druckspannung σ|high
sigma_s_pa|Reinforcement stress σ_s|Bewehrungsspannung σ_s|high
simplified|Simplified method (1) or not (0)|Vereinfachtes Verfahren (1) oder nicht (0)|high
situation|Design situation|Bemessungssituation|high
span_m|Span|Stützweite|high
spacing_m|Movement joint spacing|Dehnfugenabstand|high
stiff_len_m|Length of the stiffening wall|Länge der aussteifenden Wand|high
stiff_t_m|Thickness of the stiffening wall|Dicke der aussteifenden Wand|high
storeys|Number of storeys|Anzahl der Geschosse|high
support|Support condition|Lagerungsart|high
system|Masonry system|Mauerwerkssystem|medium
t1_m|Leaf thickness t₁|Schalendicke t₁|high
t2_m|Leaf thickness t₂|Schalendicke t₂|high
t_m|Wall thickness t|Wanddicke t|high
tie_rd_n|Design tie resistance|Bemessungstragfähigkeit des Drahtankers|high
unit_h_m|Unit height|Steinhöhe|high
unit_w_m|Unit width|Steinbreite|high
v_ed_n|Design shear force V_Ed|Bemessungsquerkraft V_Ed|high
vrd_n|Shear resistance V_Rd|Querkrafttragfähigkeit V_Rd|high
w_ed_pa|Design wind or lateral pressure w_Ed|Bemessungswinddruck w_Ed|high
web_shell_ok|Web and shell bedding permitted (1) or not (0)|Steg- und Schalenmörtel zulässig (1) oder nicht (0)|medium
web_spacing_m|Web spacing|Stegabstand|high
x_m|Position x|Lage x|high
z_m3|Section modulus Z|Widerstandsmoment Z|high"""
UNITLESS = {"basement", "class", "cpe", "eps_cs", "flanges", "frost_class", "gamma_m", "group", "hole_pct", "hydrostatic", "k0", "k_e", "k_theta", "limit_ratio", "mu_fi", "n_per_m2", "phi", "phi_inf", "q", "rei", "rho2", "rho_relax", "sides", "simplified", "storeys", "web_shell_ok", "density"}
UNIT_OVERRIDE = {"n_per_m2": ("1", "1/m²"), "rei": ("min", "min"), "hole_pct": ("1", "%"), "z_m3": ("m3", "m³")}
CHOICES = {"edges": None, "exposure": [("xo", "XO", "XO"), ("xf", "XF frost", "XF Frost"), ("xs", "XS salt", "XS Salz")], "head_joint": [("filled", "Filled", "Vermörtelt"), ("unfilled", "Unfilled", "Unvermörtelt")], "material": [("clay", "Clay", "Ziegel"), ("calcium_silicate", "Calcium silicate", "Kalksandstein"), ("concrete", "Concrete", "Beton"), ("aac", "Autoclaved aerated concrete", "Porenbeton"), ("natural_stone", "Natural stone", "Naturstein")], "mortar": [("gp", "General purpose mortar", "Normalmörtel"), ("tl", "Thin layer mortar", "Dünnbettmörtel"), ("lw", "Lightweight mortar", "Leichtmörtel")], "situation": [("persistent", "Persistent", "Ständig"), ("transient", "Transient", "Vorübergehend"), ("accidental", "Accidental", "Außergewöhnlich")], "support": [("pinned", "Pinned", "Gelenkig"), ("simple", "Simply supported", "Frei drehbar"), ("fixed", "Fixed", "Eingespannt")]}
TEXT = {"edges", "system", "density"}
import re, pathlib
DECLARED = {}
for path in pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996").rglob("*.rs"):
    if "target" in path.parts: continue
    for m in re.finditer(r'Entry \{ id: "([^"]+)",[^}]*?inputs: &\[([^\]]*)\]', path.read_text(encoding="utf-8")):
        DECLARED[m.group(1)] = re.findall(r'"([A-Za-z0-9_]+)"', m.group(2))
out = ["# key|scope|kind|unit|example|flags|en|de|confidence|note   (EN 1996 evaluators work in SI: m, Pa, N, N·m; the key suffix names the unit)"]
for key, vals in [(k, v) for k, v in CHOICES.items() if v]:
    for value, en, de in vals: out.append(f"@choice {key}|{value}|{en}|{de}")
vec_keys = {k for ks in VECTORS.values() for k in ks}
def unit_of(key):
    if key in UNIT_OVERRIDE: return UNIT_OVERRIDE[key]
    if key in UNITLESS: return ("1", "1")
    for suffix, token, symbol in SUFFIX:
        if key.endswith(suffix): return (token, symbol)
    raise SystemExit("no unit for " + key)
for line in ROWS.splitlines():
    key, en, de, conf = line.split("|")
    if key in CHOICES and CHOICES[key]:
        out.append(f"{key}||c|-|{CHOICES[key][0][0]}||{en}|{de}|{conf}|"); continue
    if key in TEXT:
        note = "format: four letters of F, S, X" if key == "edges" else "free text"
        out.append(f"{key}||x|-|||{en}|{de}|{conf}|{note}"); continue
    token, symbol = unit_of(key)
    note = "unit from the key suffix" if token not in ("1",) or key in ("rei",) else "dimensionless by name"
    if key in vec_keys:
        scope = ",".join(e for e, ks in VECTORS.items() if key in ks)
        out.append(f"{key}|{scope}|v|{token}|||{en} (one value per element)|{de} (je ein Wert pro Element)|{conf}|{note}; vector in {scope}")
        scalar = sorted(e for e, ks in DECLARED.items() if key in ks and e not in VECTORS or (e in DECLARED and key in ks and key not in VECTORS.get(e, [])))
        if scalar:
            out.append(f"{key}|{','.join(sorted(set(scalar)))}|n|{token}|||{en}|{de}|{conf}|{note}")
        continue
    flags = "r0:1" if key in ("hydrostatic", "basement", "simplified", "web_shell_ok") else ""
    out.append(f"{key}||n|{token}|||{en}|{de}|{conf}|{note}")
open("catalogue-bridge-en1996-inputs.tsv", "w", encoding="utf-8").write("\n".join(out) + "\n")
