import sys, json; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
CS, BA, RF = "chassis-steel", "bezel-abs", "rubber-foot"
ALL = ["+u", "-u", "+v", "-v", "+w", "-w"]
def only(k): return [f for f in ALL if f != k]
def feet(w, d, h=0.012, r=0.003, xs=0.18, zs=0.075):
    return [box(f"foot-{a}{b}", "case", RF, "body", 0.2, (w, h, d), (sx * xs, h / 2, sz * zs), r=r) for a, sx in (("f", 1), ("r", -1)) for b, sz in (("l", 1), ("r", -1))]
def build50():
    b = feet(0.04, 0.03)
    b += [box("case-side", "case", CS, "body", 0.55, (0.413, 0.438, 0.2), (-0.0065, 0.231, 0), r=0.004), box("rear-panel", "case", CS, "body", 0.5, (0.012, 0.438, 0.2), (-0.219, 0.231, 0), r=0.004),
          box("front-bezel", "case", BA, "body", 0.25, (0.025, 0.438, 0.2), (0.2125, 0.231, 0), r=0.005),
          box("psu-vent-rear", "case", CS, "body", 0.4, (0.0265, 0.108, 0.17), (-0.21275, 0.384, 0), r=0.002), box("psu-vent-top", "case", CS, "texture", 0.4, (0.09, 0.0076, 0.17), (-0.155, 0.4462, 0)),
          box("cpu-grille", "case", CS, "texture", 0.4, (0.17, 0.13, 0.008), (0.015, 0.267, 0.098)),
          ext("fan-grille", "case", CS, "texture", 0.3, circle(0.05, 24, 0.2, 0), "yz", -0.2258, -0.2205, holes=[circle(0.036, 24, 0.2, 0)]),
          box("intake-mesh", "case", CS, "texture", 0.35, (0.0075, 0.34, 0.17), (0.2210, 0.22, 0)), box("drive-bay", "case", BA, "texture", 0.1, (0.0075, 0.03, 0.14), (0.2210, 0.41, 0)),
          box("power-button", "case", BA, "texture", 0.8, (0.0075, 0.012, 0.012), (0.2210, 0.036, 0.056), r=0.003)]
    return b
def sk(id, mat, size, at, region, layer, keep, role="detail", tone=0.4, r=None):
    return box(id, "case", mat, role, tone, size, at, region=region, layer=layer, sectionFaces=only(keep), r=r)
def thermal(sc):
    f = {50: 0.0076, 100: 0.0151, 200: 0.0301}[sc]
    if sc == 50:
        t = [box("th-side-panel-l", "case", CS, "body", 0.55, (0.413, 0.438, 0.0076), (-0.0065, 0.231, 0.1004), region="side-panel", layer="sheet", sectionFaces=only("+w")),
             box("th-side-panel-r", "case", CS, "body", 0.55, (0.413, 0.438, 0.0076), (-0.0065, 0.231, -0.1004), region="side-panel", layer="sheet", sectionFaces=only("-w"))]
    else:
        t = [box("th-side-panel", "case", CS, "body", 0.55, (0.413, 0.438, 0.202), (-0.0065, 0.231, 0), region="side-panel", layer="sheet", sectionFaces=["+u", "-u", "+v", "-v"])]
    t.append(box("th-cpu-grille", "case", CS, "texture", 0.4, (0.172, 0.132, 0.01), (0.015, 0.267, 0.1005), region="cpu-grille", layer="grille", sectionFaces=only("+w")))
    t.append(box("th-front-bezel", "case", BA, "body", 0.25, (f, 0.438, 0.2), (0.2248 + f / 2, 0.231, 0), region="front-bezel", layer="bezel", sectionFaces=only("+u")))
    t.append(box("th-psu-vent-rear", "case", CS, "body", 0.4, (f, 0.108 if sc < 200 else 0.16, 0.17 if sc < 200 else 0.2), (-0.2248 - f / 2, 0.384, 0), region="psu-vent", layer="psu-shell", sectionFaces=only("-u")))
    if sc < 200: t.append(box("th-psu-vent-top", "case", CS, "body", 0.4, (0.09, f, 0.17), (-0.155, 0.4496 + f / 2, 0), region="psu-vent", layer="psu-shell", sectionFaces=only("+v")))
    t.append(box("th-rear-panel", "case", CS, "body", 0.5, (f, 0.438, 0.2), (-0.2248 - f / 2, 0.231, 0), region="rear-panel", layer="rear-sheet", sectionFaces=only("-u")))
    fs = {50: ((0.042, 0.0125, 0.032), (0.18, 0.00625, 0.075)), 100: ((0.0521, 0.0152, 0.0481), (0.175, 0.0076, 0.07)), 200: ((0.0301, 0.0301, 0.0301), (0.2, 0.01505, 0.0))}[sc]
    t.append(box("th-feet", "case", RF, "body", 0.2, fs[0], fs[1], region="feet", layer="foot", sectionFaces=["-v"]))
    t.append(rev("th-case-plume", "case", CS, "detail", 0.3, [(0.4, 0.04), (0.43, 0.04), (0.43, 0)], axis=(0, 1, 0), at=(-0.1, 0, 0), region="rear-panel", tags=["case-plume"], sectionFaces=ALL) if sc < 200 else box("th-case-plume", "case", CS, "detail", 0.3, (0.08, 0.06, 0.08), (-0.1, 0.4, 0), region="rear-panel", tags=["case-plume"], sectionFaces=ALL))
    t.append(rev("th-rear-exhaust", "case", CS, "detail", 0.3, [(-0.2, 0.05), (-0.17, 0.05), (-0.17, 0)], axis=(1, 0, 0), at=(0, 0.3, 0), region="rear-panel", tags=["rear-exhaust"], sectionFaces=ALL) if sc < 200 else box("th-rear-exhaust", "case", CS, "detail", 0.3, (0.06, 0.1, 0.1), (-0.19, 0.3, 0), region="rear-panel", tags=["rear-exhaust"], sectionFaces=ALL))
    return t
def structure(sc):
    w = {50: 0.02, 100: 0.02, 200: 0.031}[sc]; sh = {50: 0.008, 100: 0.0155, 200: 0.031}[sc]
    s = [st(box(f"member-case-post-{a}{b}", "case", CS, "body", 0.5, (w, 0.43, w), (sx * 0.19, 0.225, sz * 0.075)), "case", "member") for a, sx in (("f", 1), ("r", -1)) for b, sz in (("l", 1), ("r", -1))]
    s.append(st(box("member-case-base", "case", CS, "body", 0.5, (0.4, 0.02 if sc < 200 else 0.031, 0.17), (0, 0.022, 0)), "case", "member"))
    if sc == 200: s.pop(4)
    if sc == 50: s.append(st(box("member-case-top", "case", CS, "body", 0.5, (0.4, 0.02, 0.17), (0, 0.44, 0)), "case", "member"))
    s.append(st(ext("support-feet-contact", "case", RF, "ground", 0.1, [(-0.2, -0.09), (0.2, -0.09), (0.2, 0.09), (-0.2, 0.09)], "zx", -sh, 0.0, holes=[[(-0.16, -0.06), (0.16, -0.06), (0.16, 0.06), (-0.16, 0.06)]]) if False else ext("support-feet-contact", "case", RF, "ground", 0.1, [(-0.09, -0.2), (0.09, -0.2), (0.09, 0.2), (-0.09, 0.2)], "zx", -sh, 0.0, holes=[[(-0.06, -0.16), (0.06, -0.16), (0.06, 0.16), (-0.06, 0.16)]]), "case", "support"))
    s.append(st(ell("mass-case", "case", CS, "detail", 0.15, (0.15, 0.15, 0.07), (-0.02, 0.2, 0)), "case", "mass"))
    return s
def body_coarse(sc):
    if sc == 100:
        b = feet(0.05, 0.04, xs=0.175, zs=0.07, r=0.003)
        b[0]["size"][0] = 0.05
        return b + [box("case-side", "case", CS, "body", 0.55, (0.413, 0.438, 0.2), (-0.0065, 0.231, 0), r=0.004), box("rear-panel", "case", CS, "body", 0.5, (0.014, 0.438, 0.2), (-0.218, 0.231, 0), r=0.004),
                    box("front-bezel", "case", BA, "body", 0.25, (0.03, 0.438, 0.2), (0.21, 0.231, 0), r=0.005), box("psu-vent-rear", "case", CS, "body", 0.4, (0.03, 0.108, 0.17), (-0.21, 0.384, 0)),
                    box("cpu-grille", "case", CS, "texture", 0.4, (0.17, 0.13, 0.0155), (0.015, 0.267, 0.0925)), box("intake-mesh", "case", CS, "texture", 0.35, (0.0155, 0.34, 0.16), (0.2175, 0.22, 0))]
    return [box("plinth", "case", RF, "body", 0.2, (0.4, 0.024, 0.16), (0, 0.012, 0)), box("case", "case", CS, "body", 0.5, (0.45, 0.438, 0.2), (0, 0.231, 0), r=0.006)]
DEC = {
"50": (["Body: four rubber feet, the steel side sheet, rear panel with a perforated fan ring and the PSU vent (rear plate and top grille), the ABS front bezel with intake mesh, drive bay and power button as 7.5 mm texture inlays and the processor grille on the +z side panel; depths of the front panel details are estimates.",
        "Thermal: single-layer skins outside the body: the two side-panel skins (outer faces only), grille, bezel, PSU vent (rear and top), rear panel and feet, plus the 0.12 m rear-exhaust duct in the case; faces that continue into the construction are section faces.",
        "Structure: four 20 mm posts with a base and a top plate as case members, a ground contact frame under the feet as the feet-contact support and a compact mass block 0.02 m behind the centre."],
       ["Körper: vier Gummifüße, das Stahl-Seitenblech, das Rückblech mit durchbrochenem Lüfterring und dem Netzteilgitter (Rückplatte und Deckelgitter), die ABS-Frontblende mit Ansaugnetz, Laufwerksschacht und Einschalter als 7,5-mm-Einlagen und das Prozessorgitter am Seitenblech +z; die Tiefen der Frontdetails sind Schätzungen.",
        "Thermik: einschichtige Häute außerhalb des Körpers: die zwei Seitenblechhäute (nur Außenflächen), Gitter, Blende, Netzteilgitter (hinten und oben), Rückblech und Füße sowie der 0,12-m-Abluftkanal im Gehäuse; Flächen, die in die Konstruktion weiterlaufen, sind Schnittflächen.",
        "Tragwerk: vier 20-mm-Pfosten mit Boden- und Deckplatte als Gehäuseglieder, ein Bodenkontaktrahmen unter den Füßen als Auflager feet-contact und ein kompakter Massenblock 0,02 m hinter der Mitte."]),
"100": (["Body: modelled anew against the 1:100 drawings: feet 50 mm, bezel 30 mm and rear panel 14 mm with the PSU rear plate, 15.5 mm plates for processor grille and intake mesh; fan ring, drive bay, button and PSU top grille are left out.",
         "Thermal: the same eight skins at 15 mm.", "Structure: posts, base plate, ground frame and mass; the top plate is merged."],
        ["Körper: neu gegen die Zeichnungen 1:100 gebaut: Füße 50 mm, Blende 30 mm und Rückblech 14 mm mit der Netzteil-Rückplatte, 15,5-mm-Platten für Prozessorgitter und Ansaugnetz; Lüfterring, Laufwerksschacht, Taster und Netzteil-Deckelgitter entfallen.",
         "Thermik: dieselben acht Häute mit 15 mm.", "Tragwerk: Pfosten, Bodenplatte, Bodenrahmen und Masse; die Deckplatte ist aufgenommen."]),
"200": (["Body: one case block and one plinth (feet merged) as drawn at 1:200; bezel, rear panel, vents and grilles merge into the case.",
         "Thermal: the eight regions keep a skin at 30 mm.", "Structure: 31 mm posts, base, ground frame and mass."],
        ["Körper: ein Gehäuseblock und ein Sockel (Füße zusammengefasst) wie im Maßstab 1:200 gezeichnet; Blende, Rückblech, Gitter und Lüftung gehen im Gehäuse auf.",
         "Thermik: die acht Regionen behalten eine Haut mit 30 mm.", "Tragwerk: 31-mm-Pfosten, Boden, Bodenrahmen und Masse."]),
}
def volumes():
    v = {"50": scale_vol(50, build50(), thermal(50), structure(50), *DEC["50"])}
    for sc in (100, 200): v[str(sc)] = scale_vol(sc, body_coarse(sc), thermal(sc), structure(sc), *DEC[str(sc)])
    v["500"] = unavailable(500); return v
if __name__ == "__main__":
    write("appliances/tower-server", volumes())
    print(run("check", "appliances/tower-server"))
