import sys
from author_30_person import *
import copy

CAT, SLUG = "humans", "person-on-crutches"
ORDER = ["face-skin", "hands", "legs-skin", "boot", "sneaker", "hoodie", "shorts", "hair"]
CRUTCH_BONES = ("crutch-l", "crutch-r")
EXTRA_PROP_IDS = ("tube", "tip", "grip", "bracket", "cuff")


def mats_of(a):
    return {r["id"]: r["material"] for r in a["thermal"]["regions"]}


def part_axis(P, view, sid):
    pts = [p for s in P.d[view] if s["id"] == sid for r in s["rings"] for p in r]
    return pts


def grip(P, sd, a, ft):
    cb = f"crutch-{sd}"
    el = [P.local("elevation", cb, p) for sh in P.d["elevation"] if sh["id"] == f"crutch-grip-{sd}" for r in sh["rings"] for p in r]
    pl = [p for sh in P.d["plan"] if sh["id"] == f"crutch-grip-{sd}" for r in sh["rings"] for p in r]
    if not el:
        return None
    n = len(el)
    cu, cv = sum(p[0] for p in el) / n, sum(p[1] for p in el) / n
    sxx = sum((p[0] - cu) ** 2 for p in el)
    syy = sum((p[1] - cv) ** 2 for p in el)
    sxy = sum((p[0] - cu) * (p[1] - cv) for p in el)
    th = 0.5 * math.atan2(2 * sxy, sxx - syy)
    ax = (math.cos(th), math.sin(th))
    ts = [(p[0] - cu) * ax[0] + (p[1] - cv) * ax[1] for p in el]
    half = (max(ts) - min(ts)) / 2
    mid = (max(ts) + min(ts)) / 2
    wz = (sum(p[1] for p in pl) / len(pl) - P.f[cb]["o"][2]) if pl else 0.0
    r = max(0.0145, 0.55 * ft)
    c0 = (cu + ax[0] * mid, cv + ax[1] * mid)
    hl = max(half - r, 0.01)
    mats = mats_of(a)
    out = capsule(f"crutch-grip-{sd}", cb, mats["crutch-grips"], 0.12, "body", [c0[0] - ax[0] * hl, c0[1] - ax[1] * hl, wz], [c0[0] + ax[0] * hl, c0[1] + ax[1] * hl, wz], r)
    out["_r"] = "crutch-grips"
    return out


def crutch_parts(P, scale, a, sd):
    ft = FEATURE[str(scale)]
    mats = mats_of(a)
    cb = f"crutch-{sd}"
    out = []
    spec = [("tube", cb, "crutch-frame", [f"crutch-tube-{sd}"], [f"crutch-tube-{sd}"], 6 if scale == 50 else 4), ("tip", cb, "crutch-tips", [f"crutch-tip-{sd}"], [f"crutch-tip-{sd}"], 3)]
    for nm, bone, reg, el, w, n in spec:
        rows = rows_of(P, bone, inc_el=el, inc_w=w, exc=(), trim=0.004, nst=64, wcarry=True, stroke_half=(0.0165 if scale >= 200 else None))
        if rows is None:
            continue
        out.append(loft_rows(f"crutch-{nm}-{sd}", bone, mats[reg], 0.6, "body", rows, ft * 0.3, n, minr=0.55 * ft, e=2.0))
        out[-1]["_r"] = reg
    g = grip(P, sd, a, ft)
    if g:
        out.append(g)
    fb = f"forearm-{sd}"
    rows = rows_of(P, fb, inc_el=[f"crutch-bracket-{sd}"], inc_w=[], wfallback=False, trim=0.004)
    if rows and scale <= 100:
        out.append(loft_rows(f"crutch-bracket-{sd}", fb, mats["crutch-frame"], 0.6, "body", rows, ft * 0.3, 3, minr=0.55 * ft, e=2.0))
        out[-1]["_r"] = "crutch-frame"
    rows = rows_of(P, fb, inc_el=[f"crutch-cuff-{sd}"], inc_w=[f"crutch-cuff-{sd}"], trim=(0.012 if scale == 50 else 0.004))
    if rows:
        out.append(loft_rows(f"crutch-cuff-{sd}", fb, mats["crutch-cuffs"], 0.8, "body", rows, ft * 0.3, 3, minr=0.55 * ft, e=2.3))
        out[-1]["_r"] = "crutch-cuffs"
    return out


NEST = {"hoodie", "shorts", "boot", "sneaker", "hair"}
BIG = ("torso", "arm", "forearm", "thigh", "shank", "foot", "pelvis")
DEFAULT_REGION = {"torso": "hoodie", "pelvis": "hoodie", "thigh-l": "legs-skin", "thigh-r": "legs-skin", "shank-l": "legs-skin", "shank-r": "legs-skin", "arm-l": "hoodie", "arm-r": "hoodie", "hand-l": "hands", "hand-r": "hands", "neck": "face-skin", "head": "face-skin", "forearm-l": "hoodie", "forearm-r": "hoodie"}


def build_all(scale):
    a = load_asset(CAT, SLUG)
    P = Prof30(SLUG, scale)
    body = person_solids(P, scale, a, ORDER, mats_of(a), DEFAULT_REGION, skip=lambda r: bool(r) and r.startswith("crutch"), tol=0.4 if scale == 50 else 0.5, wpad={"hoodie": 0.0125}, stroke_half={50: None, 100: None, 200: None, 500: 0.045}[scale])
    ft = FEATURE[str(scale)]
    if False:
        P2 = Prof30(SLUG, 200)
        have = {b["id"] for b in body}
        for bone in ("arm-l", "forearm-l", "hand-l"):
            if bone in have:
                continue
            rows = rows_of(P2, bone, trim=0.002)
            if rows:
                body.append(loft_rows(bone, bone, mats_of(a)["hands" if bone.startswith("hand") else "hoodie"], 0.42, "body", rows, ft * 0.15, 3, minr=0.55 * ft))
                body[-1]["_r"] = "hands" if bone.startswith("hand") else "hoodie"
    body.append(ellipsoid("torso-hood", "torso", mats_of(a)["hoodie"], 0.38, "body", [0.42, 0.09 if scale <= 100 else 0.05, 0.0], [max(0.11, 0.55 * ft), max(0.09 if scale <= 100 else 0.055, 0.55 * ft), 0.11]))
    body[-1]["_r"] = "hoodie"
    for sd in "lr":
        body += crutch_parts(P, scale, a, sd)
    regs = {s["id"]: s.pop("_r", None) for s in body}
    return body, regs


def regmap_of(body, regs, scale):
    m = {}
    for s in body:
        r = regs.get(s["id"])
        if not r:
            continue
        mode = "nest" if (r in NEST and scale == 50 and any(s["bone"].startswith(b) for b in BIG) and not s["id"].startswith(("neck", "head-hair"))) else "outer"
        m.setdefault(r, []).append((s["id"], mode))
    return m


def coarse_extras(a, body, scale):
    ft = FEATURE[str(scale)]
    mats = mats_of(a)
    have = {b["id"] for b in body}
    out = []
    head = next((b for b in body if b["id"] == "head"), None)
    if head is not None and scale >= 200:
        if head["kind"] == "loft":
            us = [q["at"][0] for q in head["sections"]]
            u0, u1 = min(us), max(us)
            rv = max(q["ry"] for q in head["sections"])
            rw = max(q["rx"] for q in head["sections"])
            h = ellipsoid("th-head-hair", "head", mats["hair"], 0.15, "body", [u1 - 0.42 * (u1 - u0), 0.0, 0.0], [0.45 * (u1 - u0), rv * 1.03, rw * 1.03])
        else:
            h = copy.deepcopy(head)
            h.update(id="th-head-hair")
            h["radii"] = [r5(x * 1.03) for x in h["radii"]]
        h["region"] = "hair"
        out.append(h)
    for sd in "lr":
        f = next((x for x in body if x["id"] == f"forearm-{sd}"), None)
        if f is None or scale < 200:
            continue
        sc = f["sections"][-1] if f["kind"] == "loft" else None
        if sc is None:
            continue
        ln = max(0.06, 1.2 * ft)
        L_ = sc["at"][0]
        pad = 0.006
        mk = lambda u: {"at": [r5(u), sc["at"][1], sc["at"][2]], "rx": r5(sc["rx"] + pad), "ry": r5(sc["ry"] + pad), "exponent": 2.3, "rotation": Q_U}
        c = {"id": f"th-crutch-cuff-{sd}", "bone": f"forearm-{sd}", "material": mats["crutch-cuffs"], "role": "body", "tone": 0.8, "kind": "loft", "sections": [mk(L_ - ln - 0.02), mk(L_ - 0.02)], "capStart": True, "capEnd": True, "region": "crutch-cuffs"}
        if f"crutch-cuff-{sd}" not in have:
            out.append(c)
        tb = next((x for x in body if x["id"] == f"crutch-tube-{sd}"), None)
        if tb is not None and scale >= 500:
            r = 0.55 * ft + 0.001
            out.append({"id": f"th-crutch-tip-{sd}", "bone": f"crutch-{sd}", "material": mats["crutch-tips"], "role": "body", "tone": 0.1, "kind": "capsule", "p0": [0.82, 0, tb["sections"][-1]["at"][2]], "p1": [0.9, 0, tb["sections"][-1]["at"][2]], "r0": r5(r), "r1": r5(r), "region": "crutch-tips"})
            out.append({"id": f"th-crutch-grip-{sd}", "bone": f"crutch-{sd}", "material": mats["crutch-grips"], "role": "body", "tone": 0.12, "kind": "capsule", "p0": [0.0, -0.02, tb["sections"][0]["at"][2]], "p1": [0.0, 0.1, tb["sections"][0]["at"][2]], "r0": r5(r), "r1": r5(r), "region": "crutch-grips"})
    return out


def thermal(a, body, regs, scale):
    rm = regmap_of(body, regs, scale)
    extra = emitters(a, scale, region_default="face-skin") + coarse_extras(a, body, scale)
    return thermal_from_map(a, scale, body, rm, extra)


def structure(a, body, regs, scale):
    mem = member_bones(CAT, SLUG, scale)
    jb = {50: [b for b in mem if b.startswith(("arm", "forearm", "thigh", "shank"))] + ["torso", "head"], 100: [b for b in mem if b.startswith(("arm", "forearm", "thigh", "shank"))]}.get(scale, [])
    ms = {50: ["torso", "pelvis", "thigh-l", "thigh-r"], 100: ["torso", "pelvis", "thigh-l", "thigh-r"], 200: ["torso", "thigh-l", "thigh-r"], 500: ["torso"]}[scale]
    out = person_structure(a, scale, body, mem, CRUTCH_BONES, {}, joints=jb, masses=ms, kk={"torso": 0.5, "head": 0.5})
    for b in body:
        if b["id"].startswith(("crutch-tube", "crutch-bracket")):
            out.append({"id": "member-" + b["id"], "of": b["id"], "member": b["bone"], "part": "member"})
    return out


DEC = {
    50: [
        ("Body at 1:50: every bone is a loft whose sections are sampled from this scale's own elevation, front and plan; bones with two garments carry one loft per garment (bare thigh and shank inside a wider loose-shorts loft, the left shank inside the boot shaft, hoodie over the hips with a hood ellipsoid at the back of the neck); the tripod pose keeps the raised left foot. The crutches are modelled by hand: a tapered tube loft along the drawn splay (centre line moves 0.1 m outward from grip to tip), a rubber tip loft, a capsule for the grip and a bracket rod and an elliptical cuff ring on each forearm.", "Körper bei 1:50: jeder Knochen ist ein Loft, dessen Schnitte aus den eigenen Seiten-, Vorder- und Draufsichten dieses Massstabs stammen; Knochen mit zwei Kleidungsstücken tragen je einen Loft (nackter Ober- und Unterschenkel innerhalb des weiteren Lockershorts-Lofts, der linke Unterschenkel im Stiefelschaft, Hoodie über der Hüfte mit einem Kapuzen-Ellipsoid im Nacken); die Tripod-Pose behält den angehobenen linken Fuss. Die Krücken sind von Hand modelliert: ein konischer Rohr-Loft entlang der gezeichneten Spreizung (die Mittellinie wandert vom Griff zur Spitze 0,1 m nach aussen), ein Gummispitzen-Loft, eine Kapsel für den Griff sowie eine Streben-Stange und ein elliptischer Manschettenring an jedem Unterarm."),
        ("Thermal at 1:50: the multi-layer regions (hoodie, shorts, boot, sneaker, hair) are nested shells, one per layer, the outer layer references the body loft; bare skin regions reference the skin lofts (the shorts bury the thigh, the boot buries the left shank); crutch regions reference the crutch solids; emitters are small tagged solids at their anchors. The shorts, grips and tips expose more or less than their declared area; the numbers are in the report.", "Thermik bei 1:50: die mehrschichtigen Regionen (Hoodie, Shorts, Stiefel, Sneaker, Haar) sind ineinander liegende Schalen, eine je Schicht, die äusserste Schicht verweist auf den Körper-Loft; die nackten Hautregionen verweisen auf die Haut-Lofts (die Shorts begraben den Oberschenkel, der Stiefel den linken Unterschenkel); die Krückenregionen verweisen auf die Krückenkörper; Emitter sind kleine markierte Körper an ihren Ankern. Shorts, Griffe und Spitzen zeigen mehr oder weniger als ihre deklarierte Fläche; die Zahlen stehen im Bericht."),
        ("Structure at 1:50: member cores along every load-bearing bone (the leaning trunk, both arms, the three-point leg and the crutch tubes by reference), joints at shoulders, elbows, hips and knees, masses at trunk, pelvis and thighs.", "Tragwerk bei 1:50: Stabkerne entlang jedes tragenden Knochens (der geneigte Rumpf, beide Arme, das Dreipunktbein und die Krückenrohre per Verweis), Gelenke an Schultern, Ellbogen, Hüften und Knien, Massen an Rumpf, Becken und Oberschenkeln."),
    ],
    100: [
        ("Body at 1:100: the same per-garment lofts with at most seven sections each, regenerated from the 1:100 drawings; crutches as tapered tube and tip lofts, a grip capsule, a bracket rod and a cuff ring.", "Körper bei 1:100: dieselben Lofts je Kleidungsstück mit höchstens sieben Schnitten, aus den 1:100-Zeichnungen neu erzeugt; Krücken als konische Rohr- und Spitzen-Lofts, eine Griffkapsel, eine Streben-Stange und ein Manschettenring."),
        ("Thermal at 1:100: one surface solid per part referencing the body; the shorts still cover the thigh skin and the boot the left shank.", "Thermik bei 1:100: ein Flächenkörper je Teil mit Verweis auf den Körper; die Shorts bedecken weiter die Oberschenkelhaut und der Stiefel den linken Unterschenkel."),
        ("Structure at 1:100: members, joints and masses as at 1:50, crutch tubes by reference.", "Tragwerk bei 1:100: Stäbe, Gelenke und Massen wie bei 1:50, Krückenrohre per Verweis."),
    ],
    200: [
        ("Body at 1:200: merged masses of at most five sections per bone from this scale's drawings; the crutch tubes follow the drawn strokes (the left crutch was added to the 1:200 elevation) with a 33 mm tube; hips were added to the 1:200 front and plan drawings.", "Körper bei 1:200: zusammengefasste Massen aus höchstens fünf Schnitten je Knochen aus den Zeichnungen dieses Massstabs; die Krückenrohre folgen den gezeichneten Strichen (die linke Krücke wurde der 1:200-Ansicht ergänzt) mit einem 33-mm-Rohr; die Hüfte wurde in Vorder- und Draufsicht von 1:200 ergänzt."),
        ("Thermal at 1:200: every region has a solid (surface solids referencing the body; real hair cap and crutch cuff rings where the scale draws none); only the total exposed area is compared.", "Thermik bei 1:200: jede Region hat einen Körper (Flächenkörper mit Verweis auf den Körper; echte Haarkappe und Krückenmanschettenringe, wo der Massstab keine zeichnet); verglichen wird nur die Gesamtfläche."),
        ("Structure at 1:200: members per load-bearing bone, masses at trunk and thighs, crutch tubes by reference.", "Tragwerk bei 1:200: Stäbe je tragenden Knochen, Massen an Rumpf und Oberschenkeln, Krückenrohre per Verweis."),
    ],
    500: [
        ("Body at 1:500: torso, head, limbs and shoes as lofts of at most five sections regenerated from the 1:500 drawings (open arm strokes widened to 90 mm), the crutch tubes 82 mm thick because the smallest feature is 75 mm; the 1:500 front torso outline lost two arm blocks, the hips were added to the front and the plan, the left crutch to the elevation.", "Körper bei 1:500: Rumpf, Kopf, Glieder und Schuhe als Lofts aus höchstens fünf Schnitten, aus den 1:500-Zeichnungen neu erzeugt (offene Armstriche auf 90 mm verbreitert), die Krückenrohre sind 82 mm dick, weil das kleinste Merkmal 75 mm beträgt; der 1:500-Vorderumriss des Rumpfs verlor zwei Armblöcke, die Hüfte wurde in Vorder- und Draufsicht ergänzt, die linke Krücke in der Ansicht."),
        ("Thermal at 1:500: every region has a solid; tips, grips, cuffs and the hair cap are small real solids because the scale draws no such part; only the total exposed area is compared.", "Thermik bei 1:500: jede Region hat einen Körper; Spitzen, Griffe, Manschetten und die Haarkappe sind kleine echte Körper, weil der Massstab solche Teile nicht zeichnet; verglichen wird nur die Gesamtfläche."),
        ("Structure at 1:500: members per load-bearing bone, a mass at the trunk, crutch tubes by reference.", "Tragwerk bei 1:500: Stäbe je tragenden Knochen, eine Masse am Rumpf, Krückenrohre per Verweis."),
    ],
}


def run(scale, parts=("body", "thermal", "structure")):
    a = load_asset(CAT, SLUG)
    lists = {}
    body, regs = build_all(scale)
    if "body" in parts:
        save_lists(CAT, SLUG, scale, body=body)
    d = load(CAT, SLUG)["volumes"][str(scale)]
    body = d["body"]
    if "thermal" in parts:
        save_lists(CAT, SLUG, scale, thermal=thermal(a, body, regs, scale))
    if "structure" in parts:
        save_lists(CAT, SLUG, scale, structure=structure(a, body, regs, scale))
    save_lists(CAT, SLUG, scale, decisions=[{"en": e, "de": d} for e, d in DEC[scale]])
    print(fails(CAT, SLUG, scale))


if __name__ == "__main__":
    run(int(sys.argv[1]), tuple(sys.argv[2:]) or ("body", "thermal", "structure"))
