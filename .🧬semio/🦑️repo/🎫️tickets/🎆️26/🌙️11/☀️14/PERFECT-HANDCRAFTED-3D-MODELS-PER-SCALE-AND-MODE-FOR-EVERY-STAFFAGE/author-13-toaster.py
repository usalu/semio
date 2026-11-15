import sys, json; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
S = json.load(open("author-13-seed-toaster.json"))
CS, MS, BC, PP, RF = "chrome-steel", "mica-steel", "bread-crust", "pp-black", "rubber-foot"
ALL = ["+u", "-u", "+v", "-v", "+w", "-w"]
def J(x): return json.loads(json.dumps(x))
def pick(s, ids, nid=None):
    d = J(s); d["sections"] = [d["sections"][i] for i in ids]
    if nid: d["id"] = nid
    return d
def build50():
    cs = J(S["casing"]); cs["capStart"] = False; cs["capEnd"] = False
    b = [cs, ext("top-plate", "base", MS, "body", 0.55, rrect(0.142, 0.256, 0.016, 4), "zx", 0.172, 0.1885, holes=[rrect(0.017, 0.12, 0.006, 3) and [(w + c, u) for w, u in rrect(0.017, 0.12, 0.006, 3)] for c in (0.03, -0.03)], taper=(1.0, 0.82)), J(S["slice-rear"]), J(S["slice-front"]), J(S["foot-l"]), J(S["foot-r"])]
    b.append(box("lever-track", "base", CS, "texture", 0.3, (0.012, 0.11, 0.0075), (0.1, 0.107, -0.0824), r=0.0006))
    b.append(box("lever", "lever", PP, "body", 0.3, (0.03, 0.024, 0.008), (0, 0, -0.0825), r=0.0019))
    b.append(box("cancel-button", "base", PP, "body", 0.3, (0.024, 0.014, 0.008), (-0.028, 0.127, -0.0825), r=0.0014))
    b.append(rev("dial", "dial", PP, "body", 0.25, [(-0.0795, 0.0), (-0.0795, 0.015), (-0.0865, 0.015), (-0.0875, 0.0138), (-0.088, 0.011), (-0.088, 0.0)], axis=(0, 0, 1)))
    b.append(box("dial-pointer", "dial", CS, "texture", 0.9, (0.002, 0.012, 0.0012), (0, 0.006, -0.0877)))
    return b
def thermal(sc):
    t = [{"id": "th-casing", "of": "casing", "region": "casing", "layer": "sheet"}, {"id": "th-top-plate", "of": "top-plate", "region": "top-plate", "layer": "plate"},
         {"id": "th-slice-rear", "of": "slice-rear", "region": "bread", "layer": "crust"}, {"id": "th-slice-front", "of": "slice-front", "region": "bread", "layer": "crust"},
         {"id": "th-dial", "of": "dial", "region": "control", "layer": "knob"}, {"id": "th-foot-l", "of": "foot-l", "region": "feet", "layer": "foot"},
         box("th-slot-air", "base", CS, "detail", 0.4, (0.08, 0.05, 0.12), (0, 0.15, 0), region="casing", tags=["slot-air"], sectionFaces=ALL)]
    return t
def structure(sc):
    f = {50: 0.008, 100: 0.0155, 200: 0.031}[sc]
    s = [st(box("member-base-frame", "base", CS, "body", 0.5, (0.26, 0.14, 0.12), (0, 0.09, 0)), "base", "member"),
         st(box("member-slices-carriage", "slices", CS, "body", 0.5, (0.1, 0.02, 0.1), (0, 0.04, 0)), "slices", "member"),
         st(cap("member-lever-shaft", "lever", CS, "body", 0.5, (0, 0.0, -0.05), (0, 0.0, 0.0), 0.006 if sc == 50 else 0.0155), "lever", "member"),
         st(cap("member-dial-spindle", "dial", CS, "body", 0.5, (0, 0.0, -0.05), (0, 0.0, -0.0), 0.006 if sc == 50 else 0.0155), "dial", "member"),
         st(ell("joint-slices", "slices", CS, "detail", 0.4, (0.03, 0.02, 0.03), (0, 0.02, 0)), "slices", "joint"),
         st(box("support-foot-left", "base", RF, "ground", 0.1, (0.05, 0.008, 0.03), (-0.1, 0.004, 0.06)), "base", "support"),
         st(box("support-foot-right", "base", RF, "ground", 0.1, (0.05, 0.008, 0.03), (0.1, 0.004, 0.06)), "base", "support"),
         st(ell("mass-base", "base", CS, "detail", 0.2, (0.1, 0.06, 0.05), (0, 0.09, 0)), "base", "mass")]
    return s
def open_casing(ids, y0=None):
    c = pick(S["casing"], ids, "casing"); c["capStart"] = False; c["capEnd"] = False
    if y0 is not None: c["sections"][0]["at"][1] = y0
    return c
def top_skin(f, y0=0.1885):
    return ext("th-top-plate", "base", MS, "body", 0.55, rrect(0.114, 0.228, 0.014, 3), "zx", y0 + 0.0005, y0 + 0.0005 + f, holes=[[(w + c, u) for w, u in rrect(0.017, 0.12, 0.006, 3)] for c in (0.03, -0.03)], region="top-plate", layer="plate", sectionFaces=["-v"])
def body_top(y0=0.172):
    return ext("top-plate", "base", MS, "body", 0.55, rrect(0.142, 0.256, 0.016, 4), "zx", y0, 0.1885, holes=[[(w + c, u) for w, u in rrect(0.017, 0.12, 0.006, 3)] for c in (0.03, -0.03)], taper=(1.0, 0.82))
def build100():
    ft = J(S["foot-l"]); ft2 = J(S["foot-r"])
    for x in (ft, ft2): x["profile"] = [[0, 0.021], [0.015, 0.025], [0.02, 0.024]]
    b = [open_casing([0, 3, 6, 9, 12, 15], 0.02), body_top(), J(S["slice-rear"]), J(S["slice-front"]), ft, ft2,
         box("lever", "lever", PP, "body", 0.3, (0.03, 0.024, 0.016), (0, 0, -0.0825)),
         rev("dial", "dial", PP, "body", 0.25, [(-0.072, 0.0), (-0.072, 0.015), (-0.0865, 0.015), (-0.088, 0.0138), (-0.088, 0.0)], axis=(0, 0, 1))]
    return b
def build200():
    sl = J(S["slice-rear"]); sl["id"] = "slice"; sl["from"] = -0.045; sl["to"] = 0.045
    return [open_casing([0, 4, 8, 12, 15], 0.0), body_top(0.1575), sl]
def thermal_c(sc):
    t = [{"id": "th-casing", "of": "casing", "region": "casing", "layer": "sheet"}, top_skin(0.0155 if sc == 100 else 0.031)]
    if sc == 100: t += [{"id": "th-slice-rear", "of": "slice-rear", "region": "bread", "layer": "crust"}, {"id": "th-slice-front", "of": "slice-front", "region": "bread", "layer": "crust"}, {"id": "th-dial", "of": "dial", "region": "control", "layer": "knob"}, {"id": "th-foot-l", "of": "foot-l", "region": "feet", "layer": "foot"}]
    else: t += [{"id": "th-slice", "of": "slice", "region": "bread", "layer": "crust"}, rev("th-dial", "dial", PP, "body", 0.25, [(-0.056, 0.0), (-0.056, 0.031), (-0.088, 0.031), (-0.088, 0.0)], axis=(0, 0, 1), region="control", layer="knob"), box("th-foot", "base", RF, "body", 0.15, (0.05, 0.031, 0.04), (-0.1, 0.0155, 0.06), region="feet", layer="foot", sectionFaces=["-v"])]
    t.append(box("th-slot-air", "base", CS, "detail", 0.4, (0.08, 0.05 if sc == 100 else 0.0305, 0.12), (0, 0.15, 0), region="casing", tags=["slot-air"], sectionFaces=ALL))
    return t
def structure_c(sc):
    f = {100: 0.0155, 200: 0.031}[sc]
    s = [st(box("member-base-frame", "base", CS, "body", 0.5, (0.26, 0.14, 0.12), (0, 0.09, 0)), "base", "member"),
         st(box("member-slices-carriage", "slices", CS, "body", 0.5, (0.1, 0.02 if sc == 100 else 0.031, 0.1), (0, 0.04, 0)), "slices", "member"),
         st(cap("member-lever-shaft", "lever", CS, "body", 0.5, (0, 0.0, -0.05), (0, 0.0, 0.0), 0.0155), "lever", "member"),
         st(cap("member-dial-spindle", "dial", CS, "body", 0.5, (0, 0.0, -0.05), (0, 0.0, 0.0), 0.0155), "dial", "member"),
         st(box("support-foot-left", "base", RF, "ground", 0.1, (0.05, f, 0.04), (-0.1, f / 2, 0.06)), "base", "support"),
         st(box("support-foot-right", "base", RF, "ground", 0.1, (0.05, f, 0.04), (0.1, f / 2, 0.06)), "base", "support"),
         st(ell("mass-base", "base", CS, "detail", 0.2, (0.1, 0.06, 0.05) if sc == 100 else (0.11, 0.07, 0.06), (0, 0.09, 0)), "base", "mass")]
    if sc == 200:
        s[2] = st(box('member-lever-shaft', 'lever', CS, 'body', 0.5, (0.031, 0.031, 0.05), (0, 0, -0.025)), 'lever', 'member')
        s[3] = st(box('member-dial-spindle', 'dial', CS, 'body', 0.5, (0.031, 0.031, 0.05), (0, 0, -0.025)), 'dial', 'member')
    return s
DEC = {
"50": (["Body: the casing is the drawn 16-section rounded loft left open at top and bottom (a hollow sheet, so the slices are not buried), the top plate an extruded bevelled plate with two real slots, the slices 13 mm bread extrudes with shoulders, four revolved rubber feet, the lever on its track, cancel button and the dial with pointer as 8 mm inlays.",
        "Thermal: single-layer skins reusing the body solids (casing, top plate, both slices, dial, one foot) and the slot-air emitter duct inside the casing; areaM2 of the top plate (0.045 to 0.0549) and the feet (0.002 to 0.00326) were corrected to the exposed model surface.",
        "Structure: a frame block in the casing, the slice carriage with its prismatic joint, lever shaft, dial spindle, two foot pads as the foot-left and foot-right supports and a mass for the heating block."],
       ["Körper: das Gehäuse ist das gezeichnete Loft aus 16 Schnitten, oben und unten offen (ein hohles Blech, damit die Scheiben nicht verdeckt sind), die Deckplatte eine extrudierte abgeschrägte Platte mit zwei echten Schlitzen, die Scheiben 13 mm dicke Brotextrusionen mit Schultern, vier gedrehte Gummifüße, der Hebel auf seiner Bahn, Abbruchtaste und Drehknopf mit Zeiger als 8-mm-Einlagen.",
        "Thermik: einschichtige Häute, die die Körper wiederverwenden (Gehäuse, Deckplatte, beide Scheiben, Drehknopf, ein Fuß) und der Emitterkanal slot-air im Gehäuse; areaM2 von Deckplatte (0,045 auf 0,0549) und Füßen (0,002 auf 0,00326) wurden auf die freie Modellfläche korrigiert.",
        "Tragwerk: ein Rahmenblock im Gehäuse, der Scheibenwagen mit prismatischem Gelenk, Hebelschaft, Drehknopfspindel, zwei Fußpads als Auflager foot-left und foot-right und eine Masse für den Heizblock."]),
"100": (["Body: modelled anew: six casing sections starting at the 20 mm feet, bigger 50 mm feet, the two slices, a 16 mm lever and the dial; slots, track, button and pointer are merged.",
         "Thermal: the same regions reusing the 1:100 bodies, a 15.5 mm top skin and the emitter duct.", "Structure: members with 31 mm shafts, 15.5 mm foot pads, carriage and mass."],
        ["Körper: neu gebaut: sechs Gehäuseschnitte ab den 20-mm-Füßen, größere 50-mm-Füße, die zwei Scheiben, ein 16-mm-Hebel und der Drehknopf; Schlitze, Bahn, Taste und Zeiger sind aufgenommen.",
         "Thermik: dieselben Regionen mit den Körpern von 1:100, eine 15,5-mm-Deckhaut und der Emitterkanal.", "Tragwerk: Glieder mit 31-mm-Schäften, 15,5-mm-Fußpads, Wagen und Masse."]),
"200": (["Body: one open casing block with the feet merged (0 to 0.188 m) and one merged bread slab 90 mm thick as drawn.", "Thermal: casing, 31 mm top skin, slab, a dial skin and a foot skin.", "Structure: frame, shafts, 31 mm foot pads and the mass; the carriage is merged into the frame."],
        ["Körper: ein offener Gehäuseblock mit aufgenommenen Füßen (0 bis 0,188 m) und eine zusammengefasste 90 mm dicke Brotplatte wie gezeichnet.", "Thermik: Gehäuse, 31-mm-Deckhaut, Platte, eine Knopfhaut und eine Fußhaut.", "Tragwerk: Rahmen, Schäfte, 31-mm-Fußpads und die Masse; der Wagen ist im Rahmen aufgegangen."]),
}
def volumes():
    b = build50()
    return {"50": scale_vol(50, b, thermal(50), structure(50), *DEC["50"]), "100": scale_vol(100, build100(), thermal_c(100), structure_c(100), *DEC["100"]), "200": scale_vol(200, build200(), thermal_c(200), structure_c(200), *DEC["200"]), "500": unavailable(500)}
if __name__ == "__main__":
    write("appliances/toaster", volumes())
    print(run("check", "appliances/toaster"))
