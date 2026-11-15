import sys, json; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
S = json.load(open("author-13-seed-trv.json"))
BR, AB = "brass", "abs-plastic"
def J(x): return json.loads(json.dumps(x))
ALL = ["+u", "-u", "+v", "-v", "+w", "-w"]
def build50():
    ix = box("index", "head", AB, "texture", 0.95, (0.011, 0.0085, 0.0075), (0, 0.02475, -0.0488))
    pt = box("pointer", "knob", AB, "texture", 0.95, (0.011, 0.0045, 0.0075), (0, 0.00575, -0.0715))
    return [J(S["body"]), J(S["collar"]), ix, J(S["knob"]), pt]
def thermal(sc):
    t = [{"id": "th-body", "of": "body", "region": "body", "layer": "wall"}, {"id": "th-head", "of": "collar", "region": "head", "layer": "shell"}, {"id": "th-knob", "of": "knob", "region": "knob", "layer": "shell"}]
    return t
def structure(sc):
    r = {50: 0.0085, 100: 0.0155}[sc]
    s = [st(ext("member-body", "body", BR, "body", 0.6, circle(0.0085, 20), "zx", -0.075, 0.012, holes=[circle(0.0065, 20)]), "body", "member"),
         st(rev("member-head", "head", AB, "body", 0.5, [(-0.05, 0), (-0.05, 0.012), (0.0, 0.012), (0.0, 0)], axis=(0, 0, 1)), "head", "member"),
         st(cap("member-knob", "knob", AB, "body", 0.5, (0, 0, -0.07), (0, 0, -0.03), 0.006 if sc == 50 else 0.0155), "knob", "member"),
         st(ell("joint-knob", "knob", AB, "detail", 0.4, (0.01, 0.01, 0.01), (0, 0, -0.05)), "knob", "joint"),
         st(box("support-pipe-joint-upper", "body", BR, "ground", 0.3, (0.026, 0.012, 0.026) if sc == 50 else (0.031, 0.0155, 0.031), (0, -0.048, 0)), "body", "support"),
         st(box("support-pipe-joint-lower", "body", BR, "ground", 0.3, (0.026, 0.012, 0.026) if sc == 50 else (0.031, 0.0155, 0.031), (0, -0.07, 0)), "body", "support"),
         st(ell("mass-body", "body", BR, "detail", 0.3, (0.012, 0.03, 0.012), (0, -0.02, 0)), "body", "mass"),
         st(ell("mass-head", "head", AB, "detail", 0.3, (0.012, 0.012, 0.02), (0, 0, -0.02)), "head", "mass")]
    if sc == 100: s.pop(3)
    return s
def build100():
    body = rev("body", "body", BR, "body", 0.6, [(-0.075, 0.0), (-0.075, 0.0085), (-0.052, 0.0085), (-0.052, 0.012), (-0.042, 0.012), (-0.042, 0.019), (0, 0.019), (0.0, 0.017), (0.012, 0.017), (0.012, 0.0)], axis=(0, 1, 0))
    head = rev("head", "head", AB, "body", 0.5, [(0.019, 0.0), (0.019, 0.027), (0.0155, 0.03), (-0.0715, 0.03), (-0.0745, 0.0185), (-0.0745, 0.0)], axis=(0, 0, 1))
    return [body, head]
def thermal_c(sc):
    return [{"id": "th-body", "of": "body", "region": "body", "layer": "wall"}, {"id": "th-head", "of": "head", "region": "head", "layer": "shell"},
            rev("th-knob", "knob", AB, "body", 0.5, [(-0.052, 0.0), (-0.052, 0.0315), (-0.0745, 0.0315), (-0.0745, 0.0)], axis=(0, 0, 1), region="knob", layer="shell", sectionFaces=["+w", "-w"])]
DEC = {
"50": (["Body: the brass valve body is the drawn revolve (pipe stub, collar, hexagon-like waist and the actuator seat) standing on the pipe axis, the head a hollow actuator sleeve about the horizontal axis with the 8.5 x 11 mm index mark as texture inlay, the knob a rounded cap with a pointer mark as inlay.",
        "Thermal: three single-layer skins reusing the three body solids; areaM2 of the three regions was kept or corrected to the exposed model surface where listed in the report.",
        "Structure: a thin-walled brass pipe tube as body member, the actuator seat as head member, a spindle as knob member with a rotation joint, two 26 mm joint blocks for the upper pin and lower roller of the compression joints and masses for body and head."],
       ["Körper: der Messingventilkörper ist der gezeichnete Drehkörper (Rohrstutzen, Bund, sechskantähnliche Taille und der Stellantriebssitz) auf der Rohrachse, der Kopf eine hohle Stellantriebshülse um die waagerechte Achse mit der 8,5 x 11 mm Indexmarke als Texturinlay, der Knopf eine gerundete Kappe mit Zeigermarke als Einlage.",
        "Thermik: drei einschichtige Häute, die die drei Körper wiederverwenden; areaM2 der drei Regionen blieb erhalten oder wurde auf die freie Modellfläche korrigiert, soweit im Bericht aufgeführt.",
        "Tragwerk: ein dünnwandiges Messingrohr als Glied des Körpers, der Stellantriebssitz als Glied des Kopfes, eine Spindel als Glied des Knopfes mit Drehgelenk, zwei 26-mm-Gelenkblöcke für Stift und Rolle der Klemmverschraubungen und Massen für Körper und Kopf."]),
"100": (["Body: modelled anew with the simplified body profile and the head and knob merged into one 60 mm sleeve of the 1:100 drawing; index and pointer marks are left out.",
         "Thermal: body and head reuse the 1:100 solids, the knob is a skin 1.5 mm larger than the sleeve at its end.",
         "Structure: members with 31 mm knob spindle, 31 mm joint blocks and masses; the knob joint is merged."],
        ["Körper: neu gebaut mit vereinfachtem Körperprofil und Kopf und Knopf in einer 60-mm-Hülse der Zeichnung 1:100 zusammengefasst; Index- und Zeigermarke entfallen.",
         "Thermik: Körper und Kopf verwenden die Körper von 1:100, der Knopf ist eine Haut, 1,5 mm größer als die Hülse an deren Ende.",
         "Tragwerk: Glieder mit 31-mm-Knopfspindel, 31-mm-Gelenkblöcken und Massen; das Knopfgelenk ist zusammengefasst."]),
"200": (["x"], ["x"]),
}
def volumes():
    return {"50": scale_vol(50, build50(), thermal(50), structure(50), *DEC["50"]), "100": scale_vol(100, build100(), thermal_c(100), structure(100), *DEC["100"]), "200": unavailable(200), "500": unavailable(500)}
if __name__ == "__main__":
    write("appliances/thermostatic-radiator-valve", volumes())
    print(run("check", "appliances/thermostatic-radiator-valve"))
