"""Author 55: person-cooking props (counter, hob, pot shell, spoon)."""
import sys; sys.path.insert(0, ".")
from author_55_lib import *
SLUG = "person-cooking"; SKIN = "skin"
LAM, GLS, POT, WOOD = "laminate-chipboard", "glass-ceramic", "stainless-steel-pot", "beech-wood"
FB = {"counter", "counter-lift", "pot", "spoon"}; REG = {"counter", "hob", "pot", "spoon"}
def pot_profile(wall, floor, rim=True):
    r, h0, h1 = 0.11, 0.907, 1.048
    lip = 0.01 if rim else 0.0
    pts = [[h0, 0.0], [h0, r], [1.034, r], [1.034, r + lip], [h1, r + lip], [h1, r - wall], [h0 + floor, r - wall], [h0 + floor, 0.0]]
    return [p for i, p in enumerate(pts) if i == 0 or p != pts[i - 1]]
def props(n):
    b = []  # (solid, region)
    C = lambda s, r: b.append((s, r))
    if n == 50:
        C(box("counter-plinth", "counter", LAM, [0.33, 0.05, 0.0], [0.5, 0.1, 0.56], tone=0.25), "counter")
        C(box("counter-body", "counter", LAM, [0.3, 0.48, 0.0], [0.6, 0.76, 0.62], tone=0.55, corner=0.004), "counter")
        C(cap("counter-handle", "counter", "stainless-steel-pot", [0.2, 0.785, 0.2975], [0.4, 0.785, 0.2975], 0.0125, tone=0.8), "counter")
        C(box("worktop", "counter", LAM, [0.3, 0.88, 0.0], [0.64, 0.04, 0.62], tone=0.4, corner=0.004), "counter")
        C(box("hob-plate", "counter", GLS, [0.32, 0.9015, 0.0], [0.52, 0.013, 0.52], tone=0.1), "hob")
        for i, (x, z, r) in enumerate(((0.32, 0.14, 0.075), (0.18, 0.14, 0.05), (0.32, -0.14, 0.075)), 1):
            C(revolve(f"hob-ring-{i}", "counter", GLS, [[0.0, r - 0.006], [0.004, r - 0.006], [0.004, r], [0.0, r]], "v", origin=[x, 0.908, z], tone=0.25, texture=True), "hob")
    elif n == 100:
        C(box("counter-plinth", "counter", LAM, [0.33, 0.05, 0.0], [0.5, 0.1, 0.56], tone=0.25), "counter")
        C(box("counter-body", "counter", LAM, [0.3, 0.48, 0.0], [0.6, 0.76, 0.62], tone=0.55, corner=0.008), "counter")
        C(box("worktop", "counter", LAM, [0.3, 0.88, 0.0], [0.64, 0.04, 0.62], tone=0.4, corner=0.008), "counter")
        C(box("hob-plate", "counter", GLS, [0.32, 0.9025, 0.0], [0.52, 0.015, 0.52], tone=0.1), "hob")
    elif n == 200:
        C(box("counter-body", "counter", LAM, [0.3, 0.43, 0.0], [0.6, 0.86, 0.62], tone=0.55), "counter")
        C(box("worktop", "counter", LAM, [0.3, 0.885, 0.0], [0.64, 0.05, 0.62], tone=0.4), "hob")
    else:
        C(box("counter-block", "counter", LAM, [0.3, 0.455, 0.0], [0.62, 0.91, 0.62], tone=0.5), "counter")
        C(box("hob-slab", "counter", GLS, [0.32, 0.8775, 0.0], [0.52, 0.075, 0.52], tone=0.1), "hob")
    # pot shell (revolved), handle
    wall = {50: 0.0075, 100: 0.0145, 200: 0.03, 500: 0.075}[n]
    if n == 500:
        C(revolve("pot", "pot", POT, [[0.908, 0.0], [0.908, 0.11], [1.048, 0.11], [1.048, 0.0]], "v", origin=[0.32, 0, -0.14], tone=0.7), "pot")
    else:
        C(revolve("pot-shell", "pot", POT, pot_profile(wall, wall, rim=n <= 100), "v", origin=[0.32, 0, -0.14], tone=0.7), "pot")
        hh = {50: (0.0175, 0.014), 100: (0.0175, 0.014), 200: (0.03, 0.03)}[n]
        s = lofts("pot-handle", "pot", POT, [(0.205, hh[1], hh[0]), (0.17, hh[1], hh[0]), (0.12, hh[1] * 0.9, hh[0] * 0.9)], tone=0.2, e=2.6)
        if n == 200: s = lofts("pot-handle", "pot", POT, [(0.205, hh[1], hh[0]), (0.12, hh[1], hh[0])], tone=0.2, e=2.6)
        for q in s["sections"]: q["at"] = [q["at"][0], 1.0025 if n < 200 else 1.0, -0.14]
        C(s, "pot")
    # spoon on bone spoon (u along the spoon), lateral offset +0.046 to the pot centre
    if n == 50:
        C(lofts("spoon-handle", "spoon", WOOD, [(-0.12, 0.0085, 0.0085), (-0.06, 0.0095, 0.0095), (0.04, 0.0085, 0.0085), (0.14, 0.0065, 0.0065)], tone=0.45), "spoon")
        C(ell("spoon-bowl", "spoon", WOOD, [0.182, 0.0, 0.0], [0.042, 0.024, 0.028], tone=0.42), "spoon")
    elif n == 100:
        C(lofts("spoon-handle", "spoon", WOOD, [(-0.12, 0.011, 0.011), (-0.06, 0.012, 0.012), (0.14, 0.0085, 0.0085)], tone=0.45), "spoon")
        C(ell("spoon-bowl", "spoon", WOOD, [0.182, 0.0, 0.0], [0.042, 0.028, 0.028], tone=0.42), "spoon")
    elif n == 200:
        C(cap("spoon-handle", "spoon", WOOD, [-0.12, 0.0, 0.0], [0.14, 0.0, 0.0], 0.015, tone=0.45), "spoon")
        C(ell("spoon-bowl", "spoon", WOOD, [0.184, 0.0, 0.0], [0.042, 0.03, 0.03], tone=0.42), "spoon")
    else:
        C(cap("spoon", "spoon", WOOD, [-0.1, 0.0, 0.0], [0.22, 0.0, 0.0], 0.0375, tone=0.45), "spoon")
    for s_, _ in b:
        if s_["bone"] == "spoon":
            for q in s_.get("sections", []): q["at"] = [q["at"][0], q["at"][1] if False else 0.0, 0.046]
            if s_["kind"] == "capsule": s_["p0"][2] = s_["p1"][2] = 0.046
            if s_["kind"] == "ellipsoid": s_["transform"]["translation"][2] = 0.046
    body = [s for s, _ in b]; reg = {s["id"]: r for s, r in b}
    thermal = [th(s["id"], reg[s["id"]]) for s in body]
    st = []
    rm = {50: 0.02, 100: 0.025, 200: 0.03, 500: 0.0375}[n]
    st.append({**cap("member-counter-lift", "counter", LAM, [0.105, 0.05, 0.0] if n < 500 else [0.1, 0.2, 0.0], [0.555, 0.05, 0.0] if n < 500 else [0.5, 0.2, 0.0], 0.02 if n < 500 else 0.0375, tone=0.5, **({"slender": True} if False else {})), "member": "counter-lift", "part": "member"})
    main = {50: ["counter-body", "worktop", "hob-plate"], 100: ["counter-body", "worktop", "hob-plate"], 200: ["counter-body", "worktop"], 500: ["counter-block"]}[n]
    for sid in main: st.append({"id": f"member-{sid}", "of": sid, "member": "counter", "part": "member"})
    pid = "pot" if n == 500 else "pot-shell"
    st.append({"id": f"member-{pid}", "of": pid, "member": "pot", "part": "member"})
    sid = {50: "spoon-handle", 100: "spoon-handle", 200: "spoon-handle", 500: "spoon"}[n]
    st.append({"id": f"member-{sid}", "of": sid, "member": "spoon", "part": "member"})
    mr = {50: 0.12, 100: 0.12, 200: 0.12, 500: 0.12}[n]
    st.append({**ell("mass-counter", "counter", LAM, [0.3, 0.45, 0.0], [mr] * 3, role="detail", tone=0.6), "member": "counter", "part": "mass"})
    st.append({**ell("mass-pot", "pot", POT, [0.32, 0.98, -0.14], [0.05 if n < 500 else 0.07] * 3, role="detail", tone=0.6), "member": "pot", "part": "mass"})
    return body, thermal, st
D = {50: ("Counter, hob, pot and spoon as real objects: laminate carcass on a recessed plinth with a flush stainless pull, worktop slab, glass-ceramic hob plate with three burner rings (texture), pot as a revolved shell with rolled lip and a flat handle, wooden spoon as a tapered handle with a bowl; thermal solids are the same solids by region, structure members reference the carcass, worktop, hob, pot shell and spoon handle, plus a plinth beam for the counter lift.", "Arbeitsplatte, Kochfeld, Topf und Löffel als echte Gegenstände: Laminat-Korpus auf zurückgesetztem Sockel mit bündigem Edelstahlgriff, Platte, Glaskeramik-Kochfeld mit drei Kochzonenringen (Textur), Topf als Rotationsschale mit Rollrand und flachem Griff, Holzlöffel mit verjüngtem Stiel und Laffe; Thermik-Solids sind dieselben Solids je Region, Tragelemente verweisen auf Korpus, Platte, Kochfeld, Topfschale und Löffelstiel, dazu ein Sockelbalken für den Aufzug."),
     100: ("Cooking props at 1:100: carcass with plinth, worktop, 10 mm hob plate, pot as a 14.5 mm revolved shell with lip and handle, spoon as one tapered handle and a bowl; handle and burner rings merged away.", "Kochrequisiten 1:100: Korpus mit Sockel, Platte, 10-mm-Kochfeld, Topf als 14,5-mm-Rotationsschale mit Rand und Griff, Löffel aus einem verjüngten Stiel und Laffe; Griff und Kochzonenringe entfallen."),
     200: ("Cooking props at 1:200: one carcass block, a worktop slab that carries the hob, the pot as a 30 mm shell with handle, the spoon as a capsule with a bowl.", "Kochrequisiten 1:200: ein Korpusblock, eine Platte, die das Kochfeld trägt, der Topf als 30-mm-Schale mit Griff, der Löffel als Kapsel mit Laffe."),
     500: ("Cooking props at 1:500: counter block, a hob slab sunk into its top for the hob region, pot as a solid cylinder and the spoon as one capsule at the feature size.", "Kochrequisiten 1:500: Korpusblock, eine in die Oberseite versenkte Kochfeldplatte für die Region Kochfeld, Topf als Vollzylinder und Löffel als eine Kapsel in Mindestgröße.")}
def post(d):
    v = d["volumes"]["500"]
    for i, x in enumerate(v["thermal"]):
        if x["id"] == "th-hair": v["thermal"][i] = {**ell("th-hair", "head", "hair", [0.15, 0.012, 0], [0.0385, 0.0385, 0.0385], tone=0.4), "region": "hair"}
