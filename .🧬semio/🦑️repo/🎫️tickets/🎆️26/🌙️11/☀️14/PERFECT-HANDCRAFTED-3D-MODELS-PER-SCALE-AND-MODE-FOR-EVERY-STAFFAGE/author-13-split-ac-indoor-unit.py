import sys, json; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
S = json.load(open("author-13-seed-split.json"))
AW, AG, DP = "abs-white", "abs-grey", "display"
def J(x): return json.loads(json.dumps(x))
VANE50 = [(0, -0.015), (0.11, -0.015), (0.11458, -0.01417), (0.11833, -0.01167), (0.12125, -0.0075), (0.12333, -0.00167), (0.12458, 0.00583), (0.125, 0.015), (0.12, 0.015), (0, 0.015)]
def vane(pts, id="vane"): return ext(id, "vane", AG, "body", 0.5, pts, "xy", -0.39, 0.39)
def casing(): return J(S["casing"])
def build50():
    b = [casing(), J(S["end-cap"]), box("intake-grille", "unit", AG, "texture", 0.45, (0.11, 0.0032, 0.74), (0.085, 0.2916, 0), r=0.0015)]
    for k in range(4): b.append(cap(f"grille-slat-z-{k}", "unit", AG, "detail", 0.5, (0.047 + 0.025 * k, 0.285, -0.36), (0.047 + 0.025 * k, 0.285, 0.36), 0.004))
    for k in range(9): b.append(cap(f"grille-slat-x-{k}", "unit", AG, "detail", 0.5, (0.045, 0.281, -0.32 + 0.08 * k), (0.125, 0.281, -0.32 + 0.08 * k), 0.004))
    b.append(box("display-window", "unit", DP, "glazing", 0.1, (0.004, 0.04, 0.06), (0.2085, 0.142, 0.35), r=0.0019))
    b.append(vane(VANE50))
    return b
ALL = ["+u", "-u", "+v", "-v", "+w", "-w"]
def thermal(sc):
    t = [{"id": "th-casing", "of": "casing", "region": "casing", "layer": "shell"}]
    if sc == 50: t[0] = {"id": "th-casing", "of": "casing", "region": "casing", "layer": "shell"}
    g = box("th-grille", "unit", AG, "texture", 0.45, (0.11 + 0.002, 0.009 if sc == 50 else (0.015 if sc == 100 else 0.03), 0.74), (0.085, 0.2916 + (0.0 if sc == 50 else 0.003), 0), region="intake-grille", layer="shell", sectionFaces=["-v", "+u", "-u", "+w", "-w"], r={50: 0.004, 100: None, 200: None}[sc])
    t.append(g)
    th = {50: 0.032, 100: 0.036, 200: 0.036}[sc]
    if sc == 100:
        t.append(ext("th-vane", "vane", AG, "body", 0.5, [(0.0, -0.018), (0.12, -0.018), (0.128, 0.0), (0.128, 0.018), (0.0, 0.018)], "xy", -0.391, 0.391, region="discharge-vane", layer="shell", sectionFaces=["+v", "-u"]))
    else:
        t.append(box("th-vane", "vane", AG, "body", 0.5, (0.127, th, 0.782), (0.0625, 0, 0), region="discharge-vane", layer="shell", sectionFaces=["+v", "-u"], r={50: 0.012, 200: None}[sc]))
    t.append(box("th-cold-jet", "unit", AG, "detail", 0.4, (0.1, 0.1 if sc == 50 else 0.1, 0.5), (0.1, 0.15, 0), region="casing", tags=["cold-jet"], sectionFaces=ALL))
    return t
def structure(sc):
    f = {50: 0.0075, 100: 0.015, 200: 0.03}[sc]
    ax = {50: 0.0082, 100: 0.0155, 200: 0.0165}[sc]
    s = [st(box("member-unit-backplate", "unit", AW, "body", 0.5, (0.03, 0.26, 0.84), (0.0155, 0.145, 0)), "unit", "member")] if sc == 50 else []
    s.append(st(box("member-unit-chassis", "unit", AW, "body", 0.5, (0.1 if sc == 50 else (0.14 if sc == 100 else 0.15), 0.2 if sc == 50 else (0.25 if sc == 100 else 0.24), 0.78 if sc == 50 else (0.8 if sc == 100 else 0.76)), (0.1 if sc == 50 else 0.09, 0.15 if sc == 50 else 0.145, 0)), "unit", "member"))
    s.append(st(cap("member-vane", "vane", AG, "body", 0.5, (0.0, 0.0, -0.39), (0.0, 0.0, 0.39), ax), "vane", "member"))
    if sc == 50: s.append(st(ell("joint-vane", "vane", AG, "detail", 0.4, (0.014, 0.014, 0.04), (0.0, 0.0, 0.2)), "vane", "joint"))
    s += [st(box("support-wall-hook", "unit", AW, "ground", 0.3, (0.031 if sc > 50 else 0.02, 0.031 if sc > 50 else 0.02, 0.1), ((0.0155 if sc == 100 else 0.0165) if sc > 50 else 0.012, 0.27, 0)), "unit", "support"),
          st(cap("support-wall-rest", "unit", AW, "ground", 0.3, (0.0, 0.02, -0.05), (0.0, 0.02, 0.05), 0.0082 if sc == 50 else 0.0155), "unit", "support"),
          st(ell("mass-unit", "unit", AW, "detail", 0.3, (0.07, 0.1, 0.3), (0.1, 0.15, 0)), "unit", "mass"),
          ]
    if sc < 200: s.append(st(box("mass-vane", "vane", AG, "detail", 0.3, (0.04, 0.03, 0.4), (0.06, 0, 0)), "vane", "mass"))
    return s
def body_coarse(sc):
    if sc == 100:
        return [casing(), box("intake-grille", "unit", AG, "texture", 0.45, (0.12, 0.0035, 0.74), (0.085, 0.2882, 0), r=0.0015), box("display-window", "unit", DP, "glazing", 0.1, (0.004, 0.04, 0.06), (0.2085, 0.142, 0.35)),
                vane([(0, -0.017), (0.11, -0.017), (0.12, -0.014), (0.127, 0.0), (0.127, 0.017), (0, 0.017)])]
    return [casing(), box("intake-grille", "unit", AG, "texture", 0.45, (0.11, 0.0035, 0.74), (0.085, 0.2882, 0)), vane([(0, -0.018), (0.11, -0.018), (0.125, -0.005), (0.125, 0.018), (0, 0.018)])]
DEC = {
"50": (["Body: the casing is the drawn extrude with its rounded front, flat back and the slot step; two end caps close the ends, the intake grille is a texture plate with four lengthwise and nine cross slats of 8 mm, the display window a glazing plate and the vane a 30 mm profile extruded over 0.78 m that turns about the slot edge.",
        "Thermal: the casing skin is the casing itself (the wall face is excluded), the grille a plate skin 9 mm thick whose lower face is a section face, the vane a skin around the vane whose upper face and root are section faces and the cold-jet emitter a 0.1 m duct inside the casing.",
        "Structure: back plate and chassis as the unit member, the vane axle as member with a hinge joint, a hook block at the upper wall hook, a roller across w at the lower wall rest, masses for unit and vane."],
       ["Körper: das Gehäuse ist der gezeichnete Extrusionskörper mit gerundeter Front, ebener Rückseite und Ausblasstufe; zwei Endkappen schließen die Enden, das Ansauggitter ist eine Texturplatte mit vier Längs- und neun Querstäben von 8 mm, das Anzeigefenster eine Verglasungsplatte und die Lamelle ein 30-mm-Profil über 0,78 m, das sich um die Schlitzkante dreht.",
        "Thermik: die Gehäusehaut ist das Gehäuse selbst (die Wandfläche ist ausgenommen), das Gitter eine 9 mm dicke Plattenhaut mit Schnittfläche unten, die Lamelle eine Haut um die Lamelle mit Schnittflächen oben und an der Wurzel und der Emitter cold-jet ein 0,1-m-Kanal im Gehäuse.",
        "Tragwerk: Rückplatte und Chassis als Glied des Geräts, die Lamellenachse als Glied mit Scharniergelenk, ein Hakenblock am oberen Wandhaken, eine Rolle quer zu w am unteren Wandanschlag, Massen für Gerät und Lamelle."]),
"100": (["Body: modelled anew: casing without end caps (merged), one grille plate instead of the slats, the display window and a 34 mm vane profile with a rounded tip.",
         "Thermal: the three regions keep one skin each, the grille skin and vane skin at 15 and 36 mm.",
         "Structure: one 0.14 m chassis member for back plate and chassis, vane axle of 31 mm, 31 mm hook block and roller and two masses; the hinge joint is merged into the axle."],
        ["Körper: neu gebaut: Gehäuse ohne Endkappen (aufgenommen), eine Gitterplatte statt der Stäbe, das Anzeigefenster und ein 34-mm-Lamellenprofil mit gerundeter Spitze.",
         "Thermik: die drei Regionen behalten je eine Haut, Gitter- und Lamellenhaut mit 15 und 36 mm.",
         "Tragwerk: ein 0,14-m-Chassisglied für Rückplatte und Chassis, Lamellenachse von 31 mm, 31-mm-Hakenblock und Rolle und zwei Massen; das Scharniergelenk ist in der Achse aufgegangen."]),
"200": (["Body: casing, one grille plate and a 36 mm wedge vane; display window and end caps are merged.",
         "Thermal: three skins at 30 mm and more.", "Structure: chassis member, vane axle, supports and masses at 31 mm."],
        ["Körper: Gehäuse, eine Gitterplatte und eine 36-mm-Keillamelle; Anzeigefenster und Endkappen sind aufgenommen.",
         "Thermik: drei Häute mit 30 mm und mehr.", "Tragwerk: Chassisglied, Lamellenachse, Auflager und Massen mit 31 mm."]),
}
def volumes():
    v = {"50": scale_vol(50, build50(), thermal(50), structure(50), *DEC["50"])}
    for sc in (100, 200): v[str(sc)] = scale_vol(sc, body_coarse(sc), thermal(sc), structure(sc), *DEC[str(sc)])
    v["500"] = unavailable(500); return v
if __name__ == "__main__":
    write("appliances/split-ac-indoor-unit", volumes())
    print(run("check", "appliances/split-ac-indoor-unit"))
