"""EN 1990 schema spec shared by the generator steps (ticket-local input script)."""
import json, os, re
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990"
SUB = ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH = SUB + "/🧬️schema"
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"

def camel(s):
    p = s.split("_"); return p[0] + "".join(x.capitalize() for x in p[1:])
def pascal(s): return "".join(x.capitalize() for x in s.split("_"))
def kebab(s): return s.replace("_", "-")

def edit(path, old, new, count=1):
    t = open(path, encoding="utf8").read()
    n = t.count(old)
    assert n == count, f"{path}: expected {count} of {old[:80]!r}, found {n}"
    open(path, "w", encoding="utf8").write(t.replace(old, new))

def edit_once_if_absent(path, marker, old, new):
    t = open(path, encoding="utf8").read()
    if marker in t: return
    edit(path, old, new)

# field: (snake, rust type, label en, label de, description en, description de, ui dict)
F = lambda snake, ty, en, de, desc_en="", desc_de="", **ui: dict(snake=snake, ty=ty, en=en, de=de, den=desc_en, dde=desc_de, ui=ui)

ENTITIES = {
 "FatigueSpectrum": dict(coll="fatigue_spectra", item="fatigue_spectrum", emoji="🔩", keyed=True, label=("Fatigue spectrum bin", "Ermüdungskollektiv-Klasse"), plural=("Fatigue spectrum", "Ermüdungskollektiv"), doc="Palmgren-Miner spectrum bin of a member — EN 1990 §6.4.5 with the EN 1993-1-9 S-N curve; stresses in Pa.", fields=[
   F("id", "String", "Id", "Id"),
   F("member_id", "String", "Member", "Bauteil", widget="reference", role="value", ref=dict(kind="member")),
   F("detail_category", "f64", "Detail category Δσ_C", "Kerbfall Δσ_C", "Reference fatigue strength at 2·10⁶ cycles.", "Referenzwert der Ermüdungsfestigkeit bei 2·10⁶ Lastwechseln.", widget="stepper", unit="Pa", displayUnit="MPa", displayFactor=1e-6, precision=0, step=1e6, snaps=[36e6, 40e6, 50e6, 56e6, 63e6, 71e6, 80e6, 90e6, 100e6, 112e6, 125e6, 140e6, 160e6]),
   F("stress_range", "f64", "Stress range Δσ", "Spannungsschwingbreite Δσ", "Constant stress range of this bin.", "Konstante Spannungsschwingbreite dieser Klasse.", widget="stepper", unit="Pa", displayUnit="MPa", displayFactor=1e-6, precision=1, step=1e6),
   F("cycles", "f64", "Cycles n", "Lastwechsel n", "Number of cycles of this bin in the design working life.", "Anzahl der Lastwechsel dieser Klasse in der Nutzungsdauer.", widget="stepper", unit="1", precision=0, step=10000),
 ]),
 "ReliabilityAnalysis": dict(coll="reliability_analyses", item="reliability_analysis", emoji="🎯", keyed=True, label=("Reliability analysis", "Zuverlässigkeitsanalyse"), plural=("Reliability analysis", "Zuverlässigkeitsanalyse"), doc="FORM run for g = R − E — EN 1990 Annex C.5 to C.7; basic variables by distribution, mean and standard deviation.", fields=[
   F("id", "String", "Id", "Id"),
   F("resistance_distribution", "String", "Resistance distribution", "Verteilung des Widerstands", "normal, lognormal, gumbel or weibull.", "normal, lognormal, gumbel oder weibull.", widget="select", options=dict(normal=("Normal","Normal"), lognormal=("Lognormal","Lognormal"), gumbel=("Gumbel","Gumbel"), weibull=("Weibull","Weibull"))),
   F("resistance_mean", "f64", "Resistance mean μ_R", "Mittelwert des Widerstands μ_R", widget="stepper", unit="N", displayUnit="kN", displayFactor=0.001, precision=1, step=1000),
   F("resistance_sd", "f64", "Resistance standard deviation σ_R", "Standardabweichung des Widerstands σ_R", widget="stepper", unit="N", displayUnit="kN", displayFactor=0.001, precision=1, step=1000),
   F("action_distribution", "String", "Action distribution", "Verteilung der Einwirkung", "normal, lognormal, gumbel or weibull.", "normal, lognormal, gumbel oder weibull.", widget="select", options=dict(normal=("Normal","Normal"), lognormal=("Lognormal","Lognormal"), gumbel=("Gumbel","Gumbel"), weibull=("Weibull","Weibull"))),
   F("action_mean", "f64", "Action mean μ_E", "Mittelwert der Einwirkung μ_E", widget="stepper", unit="N", displayUnit="kN", displayFactor=0.001, precision=1, step=1000),
   F("action_sd", "f64", "Action standard deviation σ_E", "Standardabweichung der Einwirkung σ_E", widget="stepper", unit="N", displayUnit="kN", displayFactor=0.001, precision=1, step=1000),
 ]),
 "TestSeries": dict(coll="test_series", item="test_series", emoji="🧪", keyed=True, label=("Test series", "Versuchsreihe"), plural=("Test series", "Versuchsreihe"), doc="Design value from tests — EN 1990 Annex D.7: X_d = η·(m − k_n·s), logarithmic for a lognormal quantity.", fields=[
   F("id", "String", "Id", "Id"),
   F("quantity", "String", "Quantity", "Größe", "Name of the tested property.", "Name der geprüften Eigenschaft.", widget="text"),
   F("distribution", "String", "Distribution", "Verteilung", "normal or lognormal (Annex D.7.2).", "normal oder lognormal (Anhang D.7.2).", widget="select", options=dict(normal=("Normal","Normal"), lognormal=("Lognormal","Lognormal"))),
   F("sample_size", "u32", "Sample size n", "Stichprobenumfang n", "Number of test results.", "Anzahl der Versuchsergebnisse.", widget="stepper", precision=0, step=1),
   F("mean", "f64", "Sample mean m", "Stichprobenmittel m", widget="stepper", precision=3, step=0.1),
   F("std_dev", "f64", "Sample standard deviation s", "Stichprobenstandardabweichung s", widget="stepper", precision=3, step=0.01),
   F("variation_known", "f64", "Known coefficient of variation V_X", "Bekannter Variationskoeffizient V_X", "0 selects V_X unknown (Table D1 second row).", "0 wählt V_X unbekannt (Tabelle D1, zweite Zeile).", widget="stepper", precision=3, step=0.01),
   F("conversion_factor", "f64", "Conversion factor η_d", "Umrechnungsfaktor η_d", widget="stepper", precision=3, step=0.01),
   F("required", "f64", "Required design value", "Geforderter Bemessungswert", "Design value the structure relies on.", "Bemessungswert, auf den das Tragwerk sich stützt.", widget="stepper", precision=3, step=0.1),
 ]),
}
for k, e in ENTITIES.items():
    e["name"] = k
    e["coll_camel"] = camel(e["coll"])
    e["item_kebab"] = kebab(e["item"])
    e["slug"] = kebab(e["item"])

GROUP_FIELD = F("group", "String", "Exclusive group", "Ausschlussgruppe", "Variable actions sharing a non-empty group never act together (EN 1990 §4.1.1); one action per loaded span models pattern loading.", "Veränderliche Einwirkungen mit gleicher nichtleerer Gruppe wirken nie gemeinsam (EN 1990 §4.1.1); eine Einwirkung je belasteter Spannweite bildet die Feldweise Laststellung ab.", widget="text")

def ui_json(f):
    u = dict(f["ui"])
    out = {"label": {"en": f["en"], "de": f["de"]}}
    if f["den"]: out["description"] = {"en": f["den"], "de": f["dde"]}
    for k in ("widget", "role", "ref", "unit", "displayUnit", "displayFactor", "precision", "step", "snaps"):
        if k in u: out[k] = u[k]
    if "options" in u: out["options"] = {k: {"en": v[0], "de": v[1]} for k, v in u["options"].items()}
    return out

def json_type(f, root=False):
    t = f["ty"]
    if t == "String": return {"type": "string"}
    if t == "f64": return {"type": "number"}
    if t == "u32": return {"type": "integer", "minimum": 0, "maximum": 4294967295}
    raise ValueError(t)

def prop_json(f, root=False):
    d = json_type(f)
    if not root and f["snake"] != "id" and (f["ui"] or f["en"]): d["x-semio-ui"] = ui_json(f)
    return d

def def_json(e, ref_root, fields=None, root=False):
    fs = fields or e["fields"]
    d = {}
    if not root: d["title"] = e["name"]
    d.update({"type": "object", "additionalProperties": False, "required": [camel(f["snake"]) for f in fs], "properties": {camel(f["snake"]): prop_json(f, root) for f in fs}})
    if not root: d["x-semio-formats"] = ["🔣️jsonschema", "🔗️graphql", "🛰️protobuf"]
    return d

def ts_type(f): return {"String": "string", "f64": "number", "u32": "number"}[f["ty"]]
def ts_reader(f):
    t = f["ty"]
    if t == "String": return "normWireString"
    if t == "f64": return "normWireNumber"
    return 'normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})'
def proto_type(f): return {"String": "string", "f64": "double", "u32": "uint32"}[f["ty"]]
def gql_type(f): return {"String": "String!", "f64": "Float!", "u32": "Int!"}[f["ty"]]
