import os, re, json
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997"
S = ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"

def camel(s):
    p = s.split("_")
    return p[0] + "".join(x.title() for x in p[1:])

def pascal(s):
    return "".join(x.title() for x in s.split("_"))

UNITS = {
    "m": dict(unit="m", precision=2, step=0.05),
    "N": dict(unit="N", displayUnit="kN", displayFactor=0.001, precision=2, step=1000),
    "Pa": dict(unit="Pa", displayUnit="kN/m²", displayFactor=0.001, precision=2, step=1000),
    "J": dict(unit="J", displayUnit="kNm", displayFactor=0.001, precision=2, step=1000),
    "deg": dict(unit="°", precision=1, step=0.5, widget="dial"),
    "s": dict(unit="s", precision=0, step=10),
    "": dict(precision=2, step=0.05),
}

class F:
    def __init__(self, name, kind, unit, en, de, field_unit=None):
        self.name, self.kind, self.unit, self.en, self.de = name, kind, unit, en, de
        self.camel = camel(name)
        self.rust = {"s": "String", "n": "f64", "u": "u32"}[kind]
    def ui(self):
        d = {"label": {"en": self.en, "de": self.de}}
        if self.kind == "s":
            d["widget"] = "text"
            return d
        u = dict(UNITS.get(self.unit, UNITS[""]))
        if self.kind == "u":
            u = dict(precision=0, step=1)
        d.update({"widget": u.pop("widget", "stepper")})
        d.update(u)
        return d

class E:
    def __init__(self, struct, coll, slug, en, de, fields, nested=None, emoji="", uid="id"):
        self.struct, self.coll, self.slug, self.en, self.de = struct, coll, slug, en, de
        self.fields = fields
        self.nested = nested
        self.emoji = emoji
        self.camel_coll = camel(coll)
        self.var = coll[:-1] if coll.endswith("s") else coll

def f_(name, kind, unit, en, de):
    return F(name, kind, unit, en, de)

ANCHOR_TEST = E("AnchorTest", "tests", "anchor-test", "Anchor test", "Ankerprüfung", [
    f_("pull_out_resistance", "n", "N", "Suitability-test pull-out resistance R_a,m", "Herausziehwiderstand Eignungsprüfung R_a,m")])
LAB_POINT = E("LabPoint", "points", "lab-point", "Laboratory point", "Laborpunkt", [
    f_("normal_stress", "n", "Pa", "Normal stress σ / mean stress p", "Normalspannung σ / mittlere Spannung p"),
    f_("shear_stress", "n", "Pa", "Shear stress τ / deviator q", "Scherspannung τ / Deviator q")])

ENTITIES = [
    E("Anchor", "anchors", "anchor", "Ground anchor", "Verpressanker", [
        f_("wall_id", "s", "", "Supported wall id", "Gestützte Wand-ID"),
        f_("tendon", "s", "", "Tendon (strand | bar)", "Zugglied (Litze | Stab)"),
        f_("head_depth", "n", "m", "Anchor head depth", "Tiefe Ankerkopf"),
        f_("inclination_deg", "n", "deg", "Inclination below horizontal θ", "Neigung unter der Horizontalen θ"),
        f_("free_length", "n", "m", "Free length L_free", "Freie Länge L_frei"),
        f_("fixed_length", "n", "m", "Fixed length L_f", "Verpresslänge L_f"),
        f_("fixed_diameter", "n", "m", "Grout body diameter D", "Verpresskörperdurchmesser D"),
        f_("bond_stress", "n", "Pa", "Ultimate skin friction τ_f", "Grenzmantelreibung τ_f"),
        f_("design_force", "n", "N", "Design anchor force A_d", "Bemessungsankerkraft A_d"),
        f_("tendon_yield_force", "n", "N", "Characteristic tendon force P_t,k", "Charakteristische Zuggliedkraft P_t,k"),
        f_("min_free_length", "n", "m", "Minimum free length", "Mindestfreispiel"),
        f_("test_factor", "n", "", "Correlation factor ξ_a", "Korrelationsbeiwert ξ_a"),
    ], nested=("tests", "Vec<AnchorTest>", ANCHOR_TEST), emoji="⚓"),
    E("EmbeddedWall", "embedded_walls", "embedded-wall", "Embedded wall", "Eingespannte oder verankerte Wand", [
        f_("kind", "s", "", "Support (freeEarth | cantilever)", "Stützung (freeEarth | cantilever)"),
        f_("retained_height", "n", "m", "Retained height h", "Geländesprung h"),
        f_("embedment", "n", "m", "Provided embedment t", "Vorhandene Einbindetiefe t"),
        f_("prop_depth", "n", "m", "Prop depth below top", "Tiefe der Stützung unter Oberkante"),
        f_("thickness", "n", "m", "Wall thickness", "Wanddicke"),
        f_("backfill_gamma", "n", "", "Soil unit weight γ", "Bodenwichte γ"),
        f_("phi_deg", "n", "deg", "Friction angle φ′", "Reibungswinkel φ′"),
        f_("delta_active_deg", "n", "deg", "Active wall friction δ_a", "Wandreibung aktiv δ_a"),
        f_("delta_passive_deg", "n", "deg", "Passive wall friction δ_p", "Wandreibung passiv δ_p"),
        f_("surcharge", "n", "Pa", "Surcharge q", "Auflast q"),
        f_("undrained_strength", "n", "Pa", "Undrained strength c_u (0 = drained)", "Undränierte Scherfestigkeit c_u (0 = dräniert)"),
    ], emoji="🧱"),
    E("PileGroup", "pile_groups", "pile-group", "Pile group", "Pfahlgruppe", [
        f_("pile_id", "s", "", "Pile id", "Pfahl-ID"),
        f_("rows", "u", "", "Rows m", "Reihen m"),
        f_("columns", "u", "", "Columns n", "Spalten n"),
        f_("spacing", "n", "m", "Spacing s", "Achsabstand s"),
        f_("cap_vertical", "n", "N", "Cap vertical load V_d", "Vertikallast Kopfplatte V_d"),
        f_("cap_moment_x", "n", "J", "Cap moment M_x", "Moment M_x"),
        f_("cap_moment_y", "n", "J", "Cap moment M_y", "Moment M_y"),
        f_("single_resistance", "n", "N", "Single-pile resistance R_c,d", "Einzelpfahlwiderstand R_c,d"),
        f_("shaft_cohesion_sum", "n", "Pa", "Block perimeter Σ c_u·h [Pa·m]", "Blockumfang Σ c_u·h [Pa·m]"),
        f_("base_cohesion", "n", "Pa", "Block base c_u", "Blocksohle c_u"),
    ], emoji="🏗️"),
    E("LabTest", "lab_tests", "lab-test", "Laboratory test", "Laborversuch", [
        f_("layer_id", "s", "", "Layer id", "Schicht-ID"),
        f_("kind", "s", "", "Kind (oedometer | triaxial | directShear)", "Art (oedometer | triaxial | directShear)"),
        f_("void_ratio", "n", "", "Initial void ratio e₀", "Anfangsporenzahl e₀"),
        f_("preload_stress", "n", "Pa", "Preload stress σ₁", "Vorbelastung σ₁"),
        f_("drainage_height", "n", "m", "Drainage path h_dr", "Dränweg h_dr"),
        f_("consolidation_time", "n", "s", "Time to 90 % consolidation t₉₀", "Zeit bis 90 % Konsolidation t₉₀"),
    ], nested=("points", "Vec<LabPoint>", LAB_POINT), emoji="🧪"),
    E("LayerWater", "layer_waters", "layer-water", "Layer water regime", "Wasserverhältnisse der Schicht", [
        f_("layer_id", "s", "", "Layer id", "Schicht-ID"),
        f_("piezometric_depth", "n", "m", "Piezometric level depth", "Tiefe Standrohrspiegel"),
        f_("permeability", "n", "", "Permeability k [m/s]", "Durchlässigkeit k [m/s]"),
        f_("head_difference", "n", "m", "Head difference Δh", "Potenzialdifferenz Δh"),
        f_("flow_length", "n", "m", "Flow length l", "Fließlänge l"),
    ], emoji="💧"),
]
for e in ENTITIES:
    e.fields_all = [F("id", "s", "", "ID", "ID")] + e.fields
