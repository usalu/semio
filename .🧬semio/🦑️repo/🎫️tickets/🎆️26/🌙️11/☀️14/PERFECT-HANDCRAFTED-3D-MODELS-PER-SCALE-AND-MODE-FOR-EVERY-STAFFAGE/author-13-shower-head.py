import sys, json, math; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
SEED = json.load(open("author-13-seed-shower.json"))
CH, AC, EP, RN = "chrome", "abs-chrome", "epdm-hose", "rubber-nozzle"
NX, NY = 0.83878, -0.54457
HC = (0.0592, 0.1269, 0.0)
def J(x): return json.loads(json.dumps(x))
def sf(d, f): d["sectionFaces"] = f; return d

HA = [(0.01, -0.004, 0), (0.06781, -0.02966, -0.01712), (0.135, -0.08925, -0.04596), (0.18719, -0.17022, -0.0773), (0.2, -0.26, -0.10477)]
HB = [(0.2, -0.26, -0.10477), (0.17156, -0.32406, -0.12341), (0.13, -0.3375, -0.13), (0.09594, -0.30219, -0.12341), (0.09, -0.22, -0.10477), (0.10441, -0.07346, -0.0773)]
HCc = [(0.10441, -0.07346, -0.0773), (0.10775, 0.1123, -0.04596), (0.0852, 0.31653, -0.01712), (0.02196, 0.51843, 0)]
def hose(id, pts, r, bone="elbow", weights=None, mat=EP, role="body", region=None, layer=None, caps=(True, True), sparse=1):
    pts = pts[::sparse] if sparse > 1 else pts
    d = loft(id, bone, mat, role, 0.55, [sec(p, r, r) for p in pts], caps[0], caps[1])
    if weights: d["weights"] = weights
    if region: d["region"] = region; d["layer"] = layer
    return d
W3 = [{"bone": "elbow", "weight": 0.4}, {"bone": "shower", "weight": 0.6}]

def head_loft(id, ts, tone=0.9, mat=AC, role="body", n=5, half=0.0685, caps=(True, True), **kw):
    secs = [sec((HC[0] + NX * t, HC[1] + NY * t, 0), half * f, half * f, n) for t, f in ts]
    return loft(id, "shower", mat, role, tone, secs, caps[0], caps[1], **kw)
def head_solids(face=True):
    out = [head_loft("head", [(-0.0135, 0.9), (-0.011, 0.985), (0.0, 1.0), (0.0105, 0.995), (0.0135, 0.93)])]
    if face: out.append(rev("face", "shower", RN, "texture", 0.4, [(0.0055, 0), (0.0055, 0.05), (0.0141, 0.05), (0.0141, 0)], axis=(NX, NY, 0), origin=HC))
    return out

def build50():
    b = [J(SEED["bracket-low"]), J(SEED["bracket-top"]), J(SEED["tube"]), J(SEED["elbow"]), rev("lever-boss", "elbow", CH, "detail", 0.7, [(-0.026, 0), (-0.026, 0.012), (-0.0325, 0.011), (-0.0335, 0.0095), (-0.0335, 0)], axis=(0, 0, 1)), J(SEED["lever"]), J(SEED["holder"]), J(SEED["handle"])]
    b += head_solids()
    b += [hose("hose-a", HA, 0.006), hose("hose-b", HB, 0.006), hose("hose-c", HCc, 0.006, "elbow", W3)]
    return b

def seedcap(k, r=None, idp=None, region=None, layer=None):
    d = J(SEED[f"hose-{k:02d}"])
    if r: d["r0"] = d["r1"] = r
    if idp: d["id"] = f"{idp}-{k:02d}"
    if region: d["region"] = region; d["layer"] = layer
    return d
def build50():
    b = [J(SEED["bracket-low"]), J(SEED["bracket-top"]), J(SEED["tube"]), J(SEED["elbow"]), rev("lever-boss", "elbow", CH, "detail", 0.7, [(-0.026, 0), (-0.026, 0.012), (-0.0325, 0.011), (-0.0335, 0.0095), (-0.0335, 0)], axis=(0, 0, 1)), J(SEED["lever"]), J(SEED["holder"]), J(SEED["handle"])]
    b += head_solids()
    b += [seedcap(k) for k in range(12)]
    return b

ALL = ["+u", "-u", "+v", "-v", "+w", "-w"]
def skin(id, bone, mat, size, at, region, layer, faces=None, kind="box", role="body", tone=0.8, r=None):
    d = box(id, bone, mat, role, tone, size, at, region=region, layer=layer, r=r)
    if faces: d["sectionFaces"] = faces
    return d
def thermal(sc):
    f = {50: 0.0075, 100: 0.015, 200: 0.03}[sc]
    pad = 0.001
    t = [rev("th-tube", "rail", CH, "body", 0.8, [(0.1, {50: 0.0115, 100: 0.0158}[sc]), (0.79, {50: 0.0115, 100: 0.0158}[sc])], axis=(0, 1, 0), origin=(0.06, 0, 0), region="rail", layer="tube", sectionFaces=["+v", "-v"]) if sc < 200 else box("th-tube", "rail", CH, "body", 0.8, (0.0425, 0.69, 0.0425), (0.06, 0.445, 0), region="rail", layer="tube", sectionFaces=["+v", "-v"])]
    for nm, y in ((("low", 0.0975), ("top", 0.7925)) if sc == 50 else ()):
        t.append(skin(f"th-bracket-{nm}", "rail", CH, (0.051, 0.036, 0.051), (0.0255, y, 0), "rail", "tube", ["-u"], tone=0.6))
    vs = 0.062 if sc != 100 else 0.069
    t.append(skin("th-valve", "elbow", CH, (vs, vs + 0.02 if sc != 100 else 0.089, vs), (0.0025 if sc == 100 else 0, 0.0025 if sc == 100 else 0, 0), "valve", "body", ["-u"], tone=0.7))
    if sc == 50: t += [seedcap(k, 0.0063, "th-hose", "hose", "rubber") for k in range(12)]
    else: t += [{"id": f"th-hose-{k:02d}", "of": f"hose-{k:02d}", "region": "hose", "layer": "rubber"} for k in range(12)]
    t.append(cap("th-handle", "shower", AC, "body", 0.82, (-0.03804, -0.08157, 0), (0.03804, 0.08157, 0), {50: 0.0176, 100: 0.0207, 200: 0.0273}[sc], region="handle", layer="shell"))
    t.append(head_loft("th-head", [(-0.0145 if sc == 50 else (-0.0175 if sc == 100 else -0.0255), 0.9), (-0.012 if sc == 50 else (-0.014 if sc == 100 else -0.02), 0.985), (0.0, 1.0), (0.0115 if sc == 50 else (0.013 if sc == 100 else 0.02), 0.995), (0.0145 if sc == 50 else (0.0175 if sc == 100 else 0.0255), 0.93)], region="head", layer="shell"))
    t.append(loft("th-spray", "shower", EP, "detail", 0.5, [sec((0.0705 + NX * (0.01 + 0.015 * k), 0.1195 + NY * (0.01 + 0.015 * k), 0), 0.05 + 0.01 * k, 0.05 + 0.01 * k) for k in range(3)], False, False, region="spray", layer="drops"))
    t[-1]["tags"] = ["spray-mist"]
    t.append(ell("th-steam-plume", "shower", EP, "detail", 0.5, (0.02, 0.02, 0.02), (0.09917, 0.02689 + 0.0, 0.0), region="spray", tags=["steam-plume"]))
    return t


def tube_ring(id, bone, mat, ro, ri, v0, v1, cu=0.0, cw=0.0, tone=0.8):
    return ext(id, bone, mat, "body", tone, circle(ro, 20, cw, cu), "zx", v0, v1, holes=[circle(ri, 20, cw, cu)])
def structure(sc):
    f = {50: 1.0, 100: 2.0, 200: 4.0}[sc]
    s = [st(tube_ring("member-rail", "rail", CH, 0.011, 0.0098, 0.1, 0.79, 0.06, 0.0) if sc == 50 else box("member-rail", "rail", CH, "body", 0.8, (0.031 if sc == 200 else 0.018, 0.69, 0.031 if sc == 200 else 0.018), (0.06, 0.445, 0)), "rail", "member"),
         st(box("member-elbow", "elbow", CH, "body", 0.7, (0.04 if sc < 200 else 0.031, 0.06, 0.04 if sc < 200 else 0.031), (0.005, 0, 0)), "elbow", "member"),
         st(box("member-holder", "holder", CH, "body", 0.7, (0.04 if sc < 200 else 0.031, 0.045, 0.04 if sc < 200 else 0.031)), "holder", "member"),
         st(cap("member-shower", "shower", AC, "body", 0.82, (-0.03, -0.065, 0), (0.03, 0.065, 0), 0.011 if sc == 50 else (0.0155 if sc == 100 else 0.016)), "shower", "member")]
    if sc == 50:
        s += [st(ell("joint-lever", "lever", CH, "detail", 0.7, (0.012, 0.012, 0.012), (0.0, 0, -0.035)), "lever", "joint"),
              st(ell("joint-holder", "holder", CH, "detail", 0.7, (0.02, 0.02, 0.02)), "holder", "joint"),
              st(ell("joint-shower", "shower", AC, "detail", 0.82, (0.016, 0.016, 0.016)), "shower", "joint")]
    elif sc == 100:
        s += [st(ell("joint-shower", "shower", AC, "detail", 0.82, (0.02, 0.02, 0.02)), "shower", "joint")]
    pin = 0.008 if sc == 50 else (0.0155 if sc == 100 else 0.0305)
    s += [st(rev("support-wall-low", "rail", CH, "ground", 0.3, [(0.0, 0.0), (0.0, pin), (0.022, pin), (0.022, 0.0)], axis=(1, 0, 0), at=(0, 0.095, 0)) if sc == 50 else box("support-wall-low", "rail", CH, "ground", 0.3, (pin * 2 if sc == 200 else 0.031, 0.031 if sc == 100 else 0.0305 * 1, 0.031), (0.0155, 0.095, 0)), "rail", "support"),
          st(cap("support-wall-top", "rail", CH, "ground", 0.3, (0.0, 0.79, -0.02), (0.0, 0.79, 0.02), 0.008) if sc == 50 else box("support-wall-top", "rail", CH, "ground", 0.3, (0.031, 0.031, 0.04), (0.0155, 0.79, 0)), "rail", "support"),
          st(ell("mass-elbow", "elbow", CH, "detail", 0.7, (0.02, 0.03, 0.02) if sc < 200 else (0.0158, 0.03, 0.0158), (0.0, 0, 0)), "elbow", "mass"),
          st(ell("mass-shower", "shower", AC, "detail", 0.82, (0.02, 0.05, 0.02) if sc < 200 else (0.0158, 0.05, 0.0158), (0.0, 0.0, 0)), "shower", "mass")]
    return s

def build100():
    b = [box("bracket-low", "rail", CH, "body", 0.6, (0.055, 0.045, 0.055), (0.0275, 0.0975, 0), r=0.004), box("bracket-top", "rail", CH, "body", 0.6, (0.055, 0.045, 0.055), (0.0275, 0.7875, 0), r=0.004),
         rev("tube", "rail", CH, "body", 0.8, [(0.1, 0.0), (0.1, 0.015), (0.79, 0.015), (0.79, 0.0)], axis=(0, 1, 0), origin=(0.06, 0, 0)),
         box("elbow", "elbow", CH, "body", 0.7, (0.066, 0.085, 0.066), (0.0025, 0.0025, 0), r=0.006), box("holder", "holder", CH, "body", 0.7, (0.06, 0.07, 0.06), (0, 0, 0), r=0.008),
         J(SEED["handle"])]
    b[-1]["r0"] = b[-1]["r1"] = 0.02
    b.append(head_loft("head", [(-0.017, 0.9), (-0.0135, 0.985), (0.0, 1.0), (0.0125, 0.995), (0.017, 0.93)]))
    b += [seedcap(k, 0.0077) for k in range(12)]
    return b
def build200():
    b = [box("tube", "rail", CH, "body", 0.8, (0.04, 0.81, 0.04), (0.06, 0.405, 0)), box("bracket-top", "rail", CH, "body", 0.6, (0.05, 0.035, 0.05), (0.025, 0.7925, 0), r=0.004), J(SEED["handle"])]
    b[-1]["r0"] = b[-1]["r1"] = 0.026
    b.append(head_loft("head", [(-0.024, 0.92), (-0.019, 0.99), (0.0, 1.0), (0.019, 0.99), (0.024, 0.93)]))
    b += [seedcap(k, 0.0153) for k in range(12)]
    return b


DEC = {
"50": (["Body: the rail is a revolved 22 mm chrome tube between two rounded wall brackets, the elbow a rounded valve block with a lever boss and lever, the holder a rounded clamp; the hand shower is a 33 mm handle capsule with a lofted square head (rounded corners, bevelled edges, 27 mm thick, as drawn in front and top view) and a rubber nozzle face as texture row; the hose is the twelve capsule segments of the drawn loop, bound with blended weights to elbow and shower.",
        "Thermal: every region has one single-layer skin: tube skin and two bracket skins (wall faces are section faces), a valve skin, three hose skins (open ends), a handle skin, a head skin 2 mm outside the body head, and the hidden spray as a short cone at the nozzle with the spray-mist emitter tag plus the steam-plume ball; declared areaM2 values that were estimates were replaced by the exposed model areas (rail, valve, handle, head, spray).",
        "Structure: thin-walled rail tube, valve core, holder block and handle core as members, joints for lever, holder (prismatic) and shower, a pin along u at the lower wall bracket and a roller across w at the upper bracket, masses for valve and shower."],
       ["Körper: die Stange ist ein gedrehtes 22-mm-Chromrohr zwischen zwei gerundeten Wandhaltern, der Eckventilblock ein gerundeter Block mit Hebelnabe und Hebel, der Halter eine gerundete Klemme; die Handbrause ist eine 33-mm-Griffkapsel mit gelofteten quadratischen Kopf (gerundete Ecken, abgeschrägte Kanten, 27 mm dick wie in Vorder- und Draufsicht gezeichnet) und einer Gummidüsenfläche als Texturreihe; der Schlauch sind die zwölf Kapselsegmente der gezeichneten Schlaufe mit gemischten Gewichten an Eckventil und Brause.",
        "Thermik: jede Region hat eine einschichtige Haut: Rohrhaut und zwei Halterhäute (Wandflächen sind Schnittflächen), eine Ventilhaut, drei Schlauchhäute (offene Enden), eine Griffhaut, eine Kopfhaut 2 mm außerhalb des Kopfkörpers und der verdeckte Strahl als kurzer Kegel an der Düse mit dem Emitter spray-mist sowie die Dampfkugel steam-plume; geschätzte areaM2-Werte wurden durch die freien Modellflächen ersetzt (Stange, Ventil, Griff, Kopf, Strahl).",
        "Tragwerk: dünnwandiges Stangenrohr, Ventilkern, Halterblock und Griffkern als Glieder, Gelenke für Hebel, Halter (prismatisch) und Brause, ein Stift entlang u am unteren Wandhalter und eine Rolle quer zu w am oberen, Massen für Ventil und Brause."]),
"100": (["Body: modelled anew against the 1:100 drawings: tube 30 mm, brackets 55 x 45 mm, valve block 66 x 85 mm with the lever merged, holder 60 mm, handle 40 mm, head 34 mm thick; the hose is three slender lofts of the true 12 mm (the drawing keeps the true width), no nozzle face.",
         "Thermal: tube, valve, hose, handle, head and spray keep one skin each; the bracket skins are merged into the rail region's tube skin and the hose lofts have fewer sections.",
         "Structure: the same four members with box sections, only the shower joint remains, wall supports are 31 mm blocks, masses as before."],
        ["Körper: neu gegen die Zeichnungen 1:100 gebaut: Rohr 30 mm, Halter 55 x 45 mm, Ventilblock 66 x 85 mm mit aufgenommenem Hebel, Halter 60 mm, Griff 40 mm, Kopf 34 mm dick; der Schlauch besteht aus drei schlanken Lofts in wahrer Breite von 12 mm (die Zeichnung behält die wahre Breite), keine Düsenfläche.",
         "Thermik: Rohr, Ventil, Schlauch, Griff, Kopf und Strahl behalten je eine Haut; die Halterhäute gehen in der Rohrhaut der Region Stange auf und die Schlauchlofts haben weniger Schnitte.",
         "Tragwerk: dieselben vier Glieder mit Kastenquerschnitten, nur das Brausegelenk bleibt, Wandauflager sind 31-mm-Blöcke, Massen wie zuvor."]),
"200": (["Body: a 40 mm square pole carries the top bracket (brackets, holder, elbow and lever merged into the pole), the handle is 52 mm and the head 48 mm thick; the hose stays a true 12 mm slender loft in three pieces.",
         "Thermal: pole skin, valve skin, hose skins with coarse sections, 52 mm handle skin, 51 mm head skin and the spray cone.",
         "Structure: 31 mm members for rail, elbow, holder and the handle core, 31 mm supports and the two masses."],
        ["Körper: ein quadratischer Mast von 40 mm trägt den oberen Halter (Halter, Klemme, Eckventil und Hebel gehen im Mast auf), der Griff ist 52 mm und der Kopf 48 mm dick; der Schlauch bleibt ein schlankes Loft in wahrer Breite von 12 mm in drei Stücken.",
         "Thermik: Masthaut, Ventilhaut, Schlauchhäute mit groben Schnitten, 52-mm-Griffhaut, 51-mm-Kopfhaut und der Strahlkegel.",
         "Tragwerk: 31-mm-Glieder für Stange, Eckventil, Halter und Griffkern, 31-mm-Auflager und die beiden Massen."]),
}
def volumes():
    return {"50": scale_vol(50, build50(), thermal(50), structure(50), *DEC["50"]), "100": scale_vol(100, build100(), thermal(100), structure(100), *DEC["100"]), "200": scale_vol(200, build200(), thermal(200), structure(200), *DEC["200"]), "500": unavailable(500)}
if __name__ == "__main__":
    write("appliances/shower-head", volumes())
    print(run("check", "appliances/shower-head"))
