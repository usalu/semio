"""Builds catalogue-bridge-en1991-inputs.tsv. EN 1991 evaluators work in SI (m, Pa, N, kg, s), temperatures in °C, angles in degrees; units below are inferred from key names and clause context and flagged by confidence."""
import re, pathlib
raise SystemExit("superseded by catalogue-bridge-en1991-units-review.py: the TSV is maintained by hand now, rerunning would discard the units review")
D = """a|m|Dimension a|Maß a|medium|footprint or contact length
a_f|m2|Floor area A_f|Grundfläche A_f|medium|
a_fr|m2|Friction surface area A_fr|Reibungsfläche A_fr|medium|
a_over_u|m|Hydraulic radius A/U|Hydraulischer Radius A/U|medium|silo wall friction
a_ref|m2|Reference area A_ref|Bezugsfläche A_ref|high|
a_t|m2|Total enclosure area A_t|Gesamtumfassungsfläche A_t|high|
a_v|m2|Opening area A_v|Öffnungsfläche A_v|high|
alarm|1|Automatic alarm (1) or not (0)|Automatische Alarmierung (1) oder nicht (0)|medium|flag
alpha|deg|Angle or factor α|Winkel oder Beiwert α|low|roof pitch in E13, load factor α in E2.14/E5; unit depends on entry
alpha_1|deg|Roof pitch α₁|Dachneigung α₁|medium|
alpha_2|deg|Roof pitch α₂|Dachneigung α₂|medium|
alpha_c|1|Convective heat transfer coefficient α_c in W/m²K|Wärmeübergangskoeffizient α_c in W/m²K|medium|no W/(m²K) unit chosen for the stored value
alpha_h|1|Height factor α_h|Höhenbeiwert α_h|medium|
alpha_v|1|Ventilation factor α_v|Lüftungsbeiwert α_v|medium|
altitude|m|Altitude above sea level|Höhe über Meeresspiegel|high|
area|m2|Area A|Fläche A|medium|
av_over_v|1|Vent area ratio A_v/V in 1/m|Entlastungsflächenverhältnis A_v/V in 1/m|medium|
axle|N|Axle load|Achslast|medium|
axle_load|N|Axle load|Achslast|medium|
axles|1|Number of axles|Anzahl der Achsen|medium|
b|m|Width b|Breite b|high|
b1|m|Width b₁|Breite b₁|medium|
b2|m|Width b₂|Breite b₂|medium|
beta_3|1|Factor β₃|Beiwert β₃|medium|
c_d|1|Dynamic factor c_d|Dynamikbeiwert c_d|high|
c_dir|1|Direction factor c_dir|Richtungsbeiwert c_dir|high|
c_e|1|Exposure coefficient c_e|Expositionsbeiwert c_e|high|
c_esl|1|Exceptional snow load factor C_esl|Beiwert C_esl der außergewöhnlichen Schneelast|high|
c_f|1|Force coefficient c_f|Kraftbeiwert c_f|high|
c_f0|1|Force coefficient c_f,0|Kraftbeiwert c_f,0|high|
c_fr|1|Friction coefficient c_fr|Reibungsbeiwert c_fr|high|
c_fx|1|Force coefficient c_f,x|Kraftbeiwert c_f,x|high|
c_h|1|Horizontal pressure factor c_h|Horizontaldruckbeiwert c_h|medium|
c_o|1|Orography factor c_o|Orographiebeiwert c_o|high|
c_pe|1|External pressure coefficient c_pe|Außendruckbeiwert c_pe|high|
c_pi|1|Internal pressure coefficient c_pi|Innendruckbeiwert c_pi|high|
c_s|1|Size factor c_s|Größenbeiwert c_s|high|
c_season|1|Season factor c_season|Jahreszeitenbeiwert c_season|high|
c_t|1|Thermal coefficient C_t|Temperaturbeiwert C_t|high|
c_w|1|Wall friction factor c_w|Wandreibungsbeiwert c_w|medium|
capacity_t|t|Silo capacity|Siloinhalt|medium|tonnes by suffix
category|1|Category or terrain category|Kategorie oder Geländekategorie|low|text in E11 (A..K), number in E14
ceiling_height|m|Ceiling height|Deckenhöhe|high|
charge_kg|kg|Explosive charge|Sprengstoffmenge|high|
class|1|Class|Klasse|low|text class (forklift FL1..) in E11.07
consequence|1|Consequence per hazard|Folge je Gefahr|low|vector in E17.07
cover|m|Cover thickness|Überdeckungsdicke|medium|
curve|1|Temperature-time curve|Temperatur-Zeit-Kurve|low|text ISO, external, hydrocarbon
d|m|Diameter or depth d|Durchmesser oder Tiefe d|medium|
dc|m|Silo diameter d_c|Silodurchmesser d_c|high|
deck|1|Bridge deck type|Brückenüberbautyp|low|text
deck_acc|m_s2|Deck acceleration|Brückenbeschleunigung|medium|
deck_type|1|Deck type|Überbautyp|low|text
delta|1|Logarithmic decrement δ|Logarithmisches Dekrement δ|high|
delta_b|1|Barrier deflection factor δ_b|Beiwert δ_b|medium|
delta_c|1|Collision factor δ_c|Beiwert δ_c|medium|
delta_q2|1|Fire load factor δ_q2|Brandlastfaktor δ_q2|high|
delta_s|1|Structural damping decrement δ_s|Strukturdämpfungsdekrement δ_s|high|
depth|m|Depth|Tiefe|medium|
direction|1|Direction|Richtung|low|text
distance|m|Distance|Abstand|high|
dm_over_m|1|Mass ratio Δm/m|Massenverhältnis Δm/m|medium|
dominant_ratio|1|Dominant opening ratio|Verhältnis der dominanten Öffnung|high|
dt|K|Temperature difference ΔT|Temperaturdifferenz ΔT|high|
dt_m|K|Temperature difference ΔT_M|Temperaturunterschied ΔT_M|high|
dt_n|K|Uniform temperature difference ΔT_N|Gleichmäßige Temperaturänderung ΔT_N|high|
duration_days|d|Duration in days|Dauer in Tagen|high|
e|Pa|Modulus of elasticity E|Elastizitätsmodul E|medium|E15.03/05; also eccentricity in E14.08 (scope not split)
eccentricity|m|Eccentricity|Exzentrizität|medium|
eps_m|1|Surface emissivity ε_m|Emissionsgrad ε_m|high|
f_pl|N|Plastic resistance F_pl|Plastische Tragfähigkeit F_pl|medium|
finish|1|Finish type|Ausbauart|low|text
fire_area|m2|Fire area|Brandfläche|high|
floor_area|m2|Floor area|Geschossfläche|high|
frequency|Hz|Frequency|Frequenz|high|
g_k|Pa|Characteristic permanent action g_k|Charakteristische ständige Einwirkung g_k|medium|N/m² assumed
gamma|N_m3|Unit weight γ|Wichte γ|medium|material or bulk solid
gamma_g|1|Partial factor γ_G|Teilsicherheitsbeiwert γ_G|high|
gamma_q|1|Partial factor γ_Q|Teilsicherheitsbeiwert γ_Q|high|
group|1|Traffic load group|Verkehrslastgruppe|low|
h|m|Height h|Höhe h|high|
h_eq|m|Equivalent opening height|Äquivalente Öffnungshöhe|high|
h_over_d|1|Ratio h/d|Verhältnis h/d|high|
hc|m|Silo height h_c|Silohöhe h_c|high|
heat_detection|1|Automatic heat detection (1) or not (0)|Automatische Wärmemeldung (1) oder nicht (0)|medium|flag
height|m|Height|Höhe|high|
hoist_class|1|Hoisting class|Hubklasse|low|text HC1..HC4
hp_over_h|1|Parapet ratio h_p/h|Attikaverhältnis h_p/h|high|
i|m4|Second moment of area I|Flächenträgheitsmoment I|medium|
k|1|Factor k|Beiwert k|low|ratio horizontal/vertical in E4, stiffness in E17.08 (not split)
k_b|1|Conversion factor k_b|Umrechnungsfaktor k_b|high|
k_sur_cool|1|Surfacing factor for cooling|Belagsfaktor Abkühlung|high|
k_sur_heat|1|Surfacing factor for heating|Belagsfaktor Erwärmung|high|
key_area|m2|Key element area|Fläche des Schlüsselbauteils|high|
l|m|Length l|Länge l|medium|
l_f|m|Influence length L_f|Einflusslänge L_f|medium|
l_h|m|Flame height L_h|Flammenhöhe L_h|medium|
l_phi|m|Determinant length L_Φ|Maßgebende Länge L_Φ|high|
lambda|1|Damage equivalence factor λ|Schadensäquivalenzfaktor λ|medium|
lane|1|Lane number|Fahrstreifennummer|medium|
length|m|Length|Länge|high|
load|N|Load Q|Last Q|medium|
loaded_length|m|Loaded length|Belastete Länge|high|
m_c|kg|Crab mass m_c|Katzmasse m_c|medium|
m_drive|1|Number of driven wheel pairs|Anzahl angetriebener Radpaare|medium|
m_e|kg_m|Equivalent mass per length m_e|Äquivalente Masse je Länge m_e|medium|no kg/m unit token in the stored form
margin_k|1|Margin factor|Zuschlagsfaktor|medium|
mass|kg|Mass|Masse|high|
material|1|Material|Werkstoff|low|text
material2|1|Material of layer 2|Werkstoff der Schicht 2|low|text
material3|1|Material of layer 3|Werkstoff der Schicht 3|low|text
member|1|Member type|Bauteilart|low|text
mode|1|Mode or mode shape|Modus oder Eigenform|low|text
model|1|Load model|Lastmodell|low|text SW/0 or SW/2
mu|1|Shape coefficient μ|Formbeiwert μ|high|
n|1|Number n|Anzahl n|low|count or frequency, by entry
n_1|Hz|Natural frequency n₁|Eigenfrequenz n₁|high|
n_years|a|Return period in years|Wiederkehrperiode in Jahren|high|
north_german_lowland|1|North German lowland (1) or not (0)|Norddeutsches Tiefland (1) oder nicht (0)|high|flag
occupancy|1|Occupancy|Nutzung|low|text
offsite_brigade|1|Off-site fire brigade (1) or not (0)|Werkfeuerwehr extern (1) oder nicht (0)|medium|flag
onsite_brigade|1|On-site fire brigade (1) or not (0)|Werkfeuerwehr (1) oder nicht (0)|medium|flag
p|1|Probability or pressure p|Wahrscheinlichkeit oder Druck p|low|annual probability in E14.06, pressure in E14.01/E16.03
p_d_h|1|Probability of damage per hazard|Schadenswahrscheinlichkeit je Gefahr|low|vector in E17.07
p_h|1|Probability of the hazard|Gefährdungswahrscheinlichkeit|low|vector in E17.07
p_v|Pa|Venting pressure p_v|Entlastungsdruck p_v|medium|
pavement|m|Pavement thickness|Belagdicke|medium|
phi|deg|Friction angle φ|Reibungswinkel φ|medium|bulk solid; orography slope in E14.03 is a ratio
phi_1|1|Dynamic factor φ₁|Dynamikfaktor φ₁|high|
phi_7|1|Dynamic factor φ₇|Dynamikfaktor φ₇|high|
pitch|deg|Roof pitch|Dachneigung|medium|
provided_tie|N|Provided tie force|Vorhandene Zugankerkraft|medium|
psi|1|Combination factor ψ|Kombinationsbeiwert ψ|high|
psi_lambda|1|End-effect factor ψ_λ|Endscheibenbeiwert ψ_λ|high|
q_c|N|Crane load Q_c|Kranlast Q_c|medium|
q_ca|Pa|Construction load Q_ca|Baustellenlast Q_ca|medium|
q_ca_concreting|Pa|Construction load during concreting|Baustellenlast beim Betonieren|medium|
q_ci|N|Construction load Q_c,i (vector)|Baulast Q_c,i (Vektor)|low|vector in E16.06
q_fd_mj|MJ_m2|Design fire load density q_f,d|Bemessungswert der Brandlastdichte q_f,d|high|
q_fk_mj|MJ_m2|Characteristic fire load density q_f,k|Charakteristische Brandlastdichte q_f,k|high|
q_h|N|Hoisted load Q_h|Hublast Q_h|medium|
q_k|Pa|Characteristic imposed load q_k|Charakteristische Nutzlast q_k|medium|
q_max|N|Maximum wheel load Q_max|Maximale Radlast Q_max|medium|
q_p|Pa|Peak velocity pressure q_p|Böengeschwindigkeitsdruck q_p|high|
q_v|N|Vertical load Q_v|Vertikallast Q_v|medium|
q_vk|N|Characteristic vertical load Q_vk|Charakteristische Vertikallast Q_vk|medium|
q_w|W|Heat release rate Q|Wärmefreisetzungsrate Q|medium|no W unit token
r|m|Radius r|Radius r|medium|
r_acc|1|Accepted risk|Akzeptiertes Risiko|medium|
radius|m|Radius|Radius|high|
resistance_min|min|Required fire resistance in minutes|Geforderte Feuerwiderstandsdauer in Minuten|high|
rho|kg_m3|Air density ρ|Luftdichte ρ|high|1.25 typical
road|1|Road type|Straßenart|low|text
roof_type|1|Roof type|Dachform|low|text
rpm|1|Rotational speed in rpm|Drehzahl in U/min|medium|
runway_beams|1|Number of runway beams|Anzahl der Kranbahnträger|medium|
s|m|Distance or snow load s|Abstand oder Schneelast s|low|snow load in Pa for E13.07/08
s_b|m|Crane span s_b|Kranspannweite s_b|medium|
s_k|Pa|Characteristic ground snow load s_k|Charakteristische Schneelast auf dem Boden s_k|high|
safe_access|1|Safe access routes (1) or not (0)|Sichere Zugangswege (1) oder nicht (0)|medium|flag
skew_rad|rad|Skew angle|Schräglaufwinkel|high|
slab_thickness|m|Slab thickness|Plattendicke|high|
smoke_detection|1|Automatic smoke detection (1) or not (0)|Automatische Rauchmeldung (1) oder nicht (0)|medium|flag
span|m|Span|Spannweite|high|
speed|m_s|Speed|Geschwindigkeit|high|
sprinkler|1|Sprinkler system (1) or not (0)|Sprinkleranlage (1) oder nicht (0)|medium|flag
st|1|Strouhal number St|Strouhal-Zahl St|high|
standoff|m|Stand-off distance|Abstand der Detonation|high|
state|1|Snow state|Schneezustand|low|text
storeys|1|Number of storeys|Anzahl der Geschosse|high|
structure|1|Structure type|Bauwerksart|low|text
sum_q_r|N|Sum of wheel loads ΣQ_r|Summe der Radlasten ΣQ_r|medium|
surfacing|1|Surfacing type|Belagart|low|text
t_0_c|C|Initial temperature T₀|Anfangstemperatur T₀|high|
t_f_c|C|Fire temperature|Branddrucktemperatur|high|
t_lim_min|min|Limiting time t_lim in minutes|Grenzzeit t_lim in Minuten|high|
t_max_c|C|Maximum shade air temperature|Höchste Lufttemperatur im Schatten|high|
t_min|min|Time in minutes|Zeit in Minuten|medium|
t_min_c|C|Minimum shade air temperature|Niedrigste Lufttemperatur im Schatten|high|
temps_c|C|Temperature series (vector)|Temperaturreihe (Vektor)|medium|vector in E12.10
terrain|1|Terrain category|Geländekategorie|high|0 to 4
theta_g_c|C|Gas temperature θ_g|Gastemperatur θ_g|high|
theta_m_c|C|Surface temperature θ_m|Oberflächentemperatur θ_m|high|
thickness|m|Layer thickness|Schichtdicke|high|
thickness2|m|Thickness of layer 2|Dicke der Schicht 2|high|
thickness3|m|Thickness of layer 3|Dicke der Schicht 3|high|
times_min|min|Time series in minutes (vector)|Zeitreihe in Minuten (Vektor)|medium|vector in E12.10
tyres|1|Tyre type|Reifenart|low|text
v|m_s|Speed or number v|Geschwindigkeit oder Zahl v|low|
v_1|m_s|Crab speed v₁|Katzgeschwindigkeit v₁|medium|
v_b|m_s|Basic wind velocity v_b|Basiswindgeschwindigkeit v_b|high|
v_b0|m_s|Fundamental basic wind velocity v_b,0|Basiswindgeschwindigkeit v_b,0|high|
v_h|m_s|Hoisting speed v_h|Hubgeschwindigkeit v_h|high|
v_kmh|km_h|Speed in km/h|Geschwindigkeit in km/h|high|
v_m|m_s|Mean wind velocity v_m|Mittlere Windgeschwindigkeit v_m|high|
wall_gamma|N_m3|Unit weight of the wall material|Wichte des Wandbaustoffs|medium|
wall_height|m|Wall height|Wandhöhe|high|
wall_load|N_m|Wall load per metre|Wandlast je Meter|medium|N/m by name
wall_thickness|m|Wall thickness|Wanddicke|high|
water_supplies|1|Number of water supplies|Anzahl der Löschwasserversorgungen|medium|
weight|N|Weight|Gewicht|medium|
wheels|1|Number of wheels|Anzahl der Räder|medium|
width|m|Width|Breite|high|
xi|1|Factor ξ|Beiwert ξ|medium|
y_allowed|m|Allowed displacement|Zulässige Verschiebung|medium|
y_el|m|Elastic displacement|Elastische Verschiebung|medium|
z|m|Height z|Höhe z|high|
z_g|m|Height z_g|Höhe z_g|medium|
z_prime|m|Height z'|Höhe z'|medium|
zone|1|Zone|Zone|low|snow zone or wind zone number or text
cen_zone|1|CEN snow zone|CEN-Schneezone|low|
coast|1|Coastal distance class|Küstenabstandsklasse|low|"""
UNITS_FIX = {"m_s2": "m_s2", "MJ_m2": "MJ_m2"}
VECTOR = {"consequence": "E17.07", "p_d_h": "E17.07", "p_h": "E17.07", "q_ci": "E16.06", "temps_c": "E12.10", "times_min": "E12.10"}
TEXT = {"alarm_x"}
TEXTUAL = {"category", "class", "curve", "deck", "deck_type", "direction", "finish", "group", "hoist_class", "material", "material2", "material3", "member", "mode", "model", "occupancy", "road", "roof_type", "state", "structure", "surfacing", "tyres", "zone", "cen_zone", "coast", "terrain_x"}
NUMERIC_IN_SOME = {"category": "E14", "zone": "E14"}
out = ["# key|scope|kind|unit|example|flags|en|de|confidence|note   (EN 1991 evaluators: SI m, Pa, N, kg, s; temperatures in °C; angles in degrees; every unit is inferred, see the confidence column)"]
for line in D.splitlines():
    key, unit, en, de, conf, note = line.split("|")
    if key in VECTOR:
        out.append(f"{key}|{VECTOR[key]}|v|{unit}|||{en}|{de}|{conf}|{note}")
    elif key in TEXTUAL:
        out.append(f"{key}||x|-|||{en}|{de}|{conf}|{note or 'free text: the accepted values are read by the evaluator'}")
    else:
        out.append(f"{key}||n|{unit}|||{en}|{de}|{conf}|{note}")
open("catalogue-bridge-en1991-inputs.tsv", "w", encoding="utf-8").write("\n".join(out) + "\n")
