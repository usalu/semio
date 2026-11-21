"""🚧 Builds the round 3 excavator: the corrected roots (dimensions, rig with the real pin geometry and the bucket four-bar, anchors, poses and clip keys with every cylinder closed, the drawings of all views) and the volumes of the named scales. Usage: python3 r3-excavator-build.py [50 100 200 500]; writes the draft asset under 🗑️generated/r3-excavator/draft and the source volumes/adhoc/excavator.json; with --final also volumes/machinery/excavator.json. Scales not named keep what the catalog has."""
import copy
import glob
import importlib.util
import json
import math
import os
import sys

here = os.path.dirname(os.path.abspath(__file__))


def load(name):
    key = name.replace("-", "_")
    if key in sys.modules:
        return sys.modules[key]
    spec = importlib.util.spec_from_file_location(key, os.path.join(here, name + ".py"))
    mod = importlib.util.module_from_spec(spec)
    sys.modules[key] = mod
    spec.loader.exec_module(mod)
    return mod


F = load("r3-excavator-form")
L, K = F.L, F.K
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*machinery/excavator/*.json")[0]
DRAFT = os.path.join(here, "🗑️generated", "r3-excavator", "draft", "excavator", "🔣️.json")
LIM = K.limits()
SAFE = {k: (math.ceil(v[0] * 10) / 10, math.floor(v[1] * 10) / 10) for k, v in LIM.items()}
REST = (K.REST["boom"], K.REST["stick"], K.REST["bucket"])
T = lambda en, de: {"en": en, "de": de}


def swing(a, b, parent):
    """Rest angle and limits of a bar from pin a to pin b relative to the frame angle `parent(theta, sigma, beta)`, over the whole working range."""
    rest = K.chain(*REST)
    base = (K.ang(rest[a], rest[b]) - parent(*REST) + 180) % 360 - 180
    lo = hi = 0.0
    n = 24
    for i in range(n + 1):
        for j in range(n + 1):
            for k in range(n + 1):
                q = (LIM["boom"][0] + (LIM["boom"][1] - LIM["boom"][0]) * i / n, LIM["stick"][0] + (LIM["stick"][1] - LIM["stick"][0]) * j / n, LIM["bucket"][0] + (LIM["bucket"][1] - LIM["bucket"][0]) * k / n)
                p = K.chain(*q)
                d = (K.ang(p[a], p[b]) - parent(*q) - base + 180) % 360 - 180
                lo, hi = min(lo, d), max(hi, d)
    return round(base, 3), math.floor(lo - 1), math.ceil(hi + 1)


def rig():
    rest = K.chain(*REST)
    A, C, E, H, I = (K.PIN[k] for k in "ACEHI")
    mass = lambda kg, x, y: {"kg": kg, "comX": x, "comY": y, "distribution": "point", "source": "authored-estimate"}
    rev = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
    bones = [
        {"id": "undercarriage", "parent": None, "title": T("Tracked undercarriage", "Raupenfahrwerk"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 4.45, "joint": {"type": "fixed"}, "mass": mass(7300, 0, 0.48)},
        {"id": "slew-ring", "parent": "undercarriage", "title": T("Slew ring (vertical axis)", "Drehkranz (senkrechte Achse)"), "x": 0, "y": 0.94, "angleDeg": 90, "lengthM": 0.12, "joint": rev(0, 0)},
        {"id": "upper", "parent": "slew-ring", "title": T("Upper structure with cab (turns on the slew ring)", "Oberwagen mit Kabine (dreht auf dem Drehkranz)"), "x": 0.12, "y": 0, "angleDeg": -90, "lengthM": 3.82, "joint": {"type": "fixed"}, "mass": mass(5300, -0.35, 0.62)},
        {"id": "counterweight", "parent": "upper", "title": T("Counterweight", "Gegengewicht"), "x": -F.TAIL, "y": 0, "angleDeg": 0, "lengthM": F.CW_FRONT, "joint": {"type": "fixed"}, "mass": mass(4200, 0.32, 0.58)},
        {"id": "engine", "parent": "upper", "title": T("Diesel engine and pumps", "Dieselmotor und Pumpen"), "x": -2.03, "y": 0.40, "angleDeg": 0, "lengthM": 0.72, "joint": {"type": "fixed"}, "mass": mass(900, 0.36, 0.33)},
        {"id": "boom", "parent": "upper", "title": T("Boom", "Ausleger"), "x": A[0], "y": A[1], "angleDeg": REST[0], "lengthM": K.BOOM, "joint": rev(math.ceil((LIM["boom"][0] - REST[0]) * 10) / 10, math.floor((LIM["boom"][1] - REST[0]) * 10) / 10), "mass": mass(1750, 2.55, 0.42)},
        {"id": "stick", "parent": "boom", "title": T("Stick", "Löffelstiel"), "x": K.BOOM, "y": 0, "angleDeg": REST[1], "lengthM": K.STICK, "joint": rev(math.ceil((LIM["stick"][0] - REST[1]) * 10) / 10, math.floor((LIM["stick"][1] - REST[1]) * 10) / 10), "mass": mass(900, 1.05, 0.10)},
        {"id": "bucket", "parent": "stick", "title": T("Bucket", "Löffel"), "x": K.STICK, "y": 0, "angleDeg": REST[2], "lengthM": K.TIP, "joint": rev(math.ceil((LIM["bucket"][0] - REST[2]) * 10) / 10, math.floor((LIM["bucket"][1] - REST[2]) * 10) / 10), "mass": mass(820, 0.62, 0.36)},
    ]
    info = {}
    for name, a, b, parent_id, origin, parent, title, rod_title, kg in (
        ("boom", "C", "D", "upper", C, lambda t, s, b: 0.0, T("Boom cylinders, barrels", "Auslegerzylinder, Zylinderrohre"), T("Boom cylinders, rods", "Auslegerzylinder, Kolbenstangen"), (380, 240)),
        ("stick", "E", "F", "boom", E, lambda t, s, b: t, T("Stick cylinder, barrel", "Stielzylinder, Zylinderrohr"), T("Stick cylinder, rod", "Stielzylinder, Kolbenstange"), (230, 140)),
        ("bucket", "H", "K", "stick", H, lambda t, s, b: t + s, T("Bucket cylinder, barrel", "Löffelzylinder, Zylinderrohr"), T("Bucket cylinder, rod", "Löffelzylinder, Kolbenstange"), (130, 70)),
    ):
        c = K.CYL[name]
        base, lo, hi = swing(a, b, parent)
        ext = K.dist(rest[a], rest[b]) - c["closed"]
        barrel = c["closed"] - 0.23
        rod = {"x": round(0.10 + ext, 4), "lengthM": round(c["closed"] - 0.10, 4)}
        info[name + "-rod"] = rod
        bones.append({"id": name + "-cyl", "parent": parent_id, "title": title, "x": origin[0], "y": origin[1], "angleDeg": base, "lengthM": round(barrel, 3), "joint": rev(lo, hi), "mass": mass(kg[0], round(barrel / 2, 2), 0)})
        bones.append({"id": name + "-rod", "parent": name + "-cyl", "title": rod_title, "x": rod["x"], "y": 0, "angleDeg": 0, "lengthM": rod["lengthM"], "joint": {"type": "prismatic", "axisDeg": 0, "minM": round(-ext, 4), "maxM": round(c["stroke"] - ext, 4)}, "mass": mass(kg[1], round(rod["lengthM"] / 2, 2), 0)})
    base, lo, hi = swing("I", "K", lambda t, s, b: t + s)
    bones.append({"id": "idler-link", "parent": "stick", "title": T("Idler links of the bucket linkage", "Umlenkhebel der Löffelkinematik"), "x": I[0], "y": I[1], "angleDeg": base, "lengthM": K.IDLER, "joint": rev(lo, hi), "mass": mass(60, round(K.IDLER / 2, 2), 0)})
    rest_idler = K.ang(rest["I"], rest["K"])
    base2 = round((K.ang(rest["K"], rest["J"]) - rest_idler + 180) % 360 - 180, 3)
    lo = hi = 0.0
    for k in range(200):
        p = K.chain(0, -90, LIM["bucket"][0] + (LIM["bucket"][1] - LIM["bucket"][0]) * k / 199)
        d = (K.ang(p["K"], p["J"]) - K.ang(p["I"], p["K"]) - base2 + 180) % 360 - 180
        lo, hi = min(lo, d), max(hi, d)
    bones.append({"id": "bucket-link", "parent": "idler-link", "title": T("Bucket link (H link)", "Löffellasche (H-Lasche)"), "x": K.IDLER, "y": 0, "angleDeg": base2, "lengthM": K.LINK, "joint": rev(math.floor(lo - 1), math.ceil(hi + 1)), "mass": mass(95, round(K.LINK / 2, 2), 0)})
    return {"kind": "mechanical", "root": "undercarriage", "bones": bones}, info


def bucket_low(th, sg, be):
    """Lowest point of the bucket outline above the ground for the three joint angles."""
    p = K.chain(th, sg, be)
    pts = L.poly(F.BUCKET).pts(0.01) + F.TOOTH
    return min(K.add(p["G"], K.rot(q, th + sg + be))[1] for q in pts) + K.DECK


def linkage_low(th, sg, be):
    """Lowest point of the bucket and of its link pins with their bosses above the ground."""
    p = K.chain(th, sg, be)
    return min(bucket_low(th, sg, be), min(p["K"][1], p["J"][1], p["I"][1]) + K.DECK - 0.11)


def stowed():
    sg, be = -139.0, -140.0
    lo, hi = -10.0, 30.0
    for _ in range(50):
        m = (lo + hi) / 2
        if linkage_low(m, sg, be) < 0.01:
            lo = m
        else:
            hi = m
    return (round((lo + hi) / 2, 2), sg, be)


def at(tx, ty, phi):
    return K.ik((tx, ty - K.DECK), phi, SAFE)


DIG = [(0.0, 8.4, 0.9, -50), (0.5, 8.3, 0.4, -55), (1.0, 8.1, 0.0, -62), (1.5, 7.5, -0.30, -75), (2.0, 6.8, -0.40, -95), (2.5, 6.1, -0.40, -118), (3.0, 5.5, -0.25, -140), (3.5, 5.1, 0.10, -160), (4.0, 5.2, 0.9, -168), (4.5, 5.6, 1.8, -172), (5.0, 6.1, 2.6, -175), (5.5, 6.6, 3.3, -175), (6.0, 7.0, 3.8, -172), (6.5, 7.4, 4.0, -165), (7.0, 7.9, 3.4, -110), (7.5, 8.3, 2.7, -60), (8.0, 8.5, 2.0, -52), (8.5, 8.5, 1.4, -50)]
DUMP = [(0.0, 6.6, 0.0, -100, 0), (0.4, 6.5, 0.5, -125, 12), (0.8, 6.3, 1.3, -150, 35), (1.2, 6.2, 2.2, -165, 62), (1.6, 6.4, 3.1, -172, 82), (2.0, 6.8, 3.8, -175, 90), (2.4, 7.2, 4.0, -170, 90), (2.8, 7.6, 3.9, -140, 90), (3.2, 7.9, 3.5, -95, 90), (3.6, 8.0, 3.2, -60, 90), (4.0, 7.6, 2.8, -70, 78), (4.4, 6.9, 2.3, -95, 55), (4.8, 6.0, 1.8, -125, 30), (5.2, 5.2, 1.3, -150, 12), (5.6, 4.5, 0.95, -170, 3), (6.0, 4.0, 0.75, -185, 0)]


def dense(keys, end, loop, step=0.125):
    """The hand-set path keys (t, x, y, phi, twist) resampled every `step` seconds with a Catmull-Rom curve through them, so that the cylinders, which are solved at every key, stay closed between the keys."""
    n = len(keys)
    out, twists = [], []
    count = int(round(end / step)) + (0 if loop else 1)
    for i in range(count):
        t = i * step
        j = max(k for k in range(n) if keys[k][0] <= t + 1e-9)
        if not loop and j == n - 1:
            out.append((round(t, 3),) + tuple(keys[j][1:4]))
            twists.append(keys[j][4])
            continue
        idx = [(j - 1) % n, j, (j + 1) % n, (j + 2) % n] if loop else [max(j - 1, 0), j, min(j + 1, n - 1), min(j + 2, n - 1)]
        t1 = keys[(j + 1) % n][0] if (j + 1 < n) else end
        u = (t - keys[j][0]) / (t1 - keys[j][0])
        vals = []
        for c in range(1, 5):
            p0, p1, p2, p3 = (keys[q][c] for q in idx)
            vals.append(0.5 * ((2 * p1) + (-p0 + p2) * u + (2 * p0 - 5 * p1 + 4 * p2 - p3) * u * u + (-p0 + 3 * p1 - 3 * p2 + p3) * u ** 3))
        out.append((round(t, 3), round(vals[0], 4), round(vals[1], 4), round(vals[2], 3)))
        twists.append(round(min(max(vals[3], 0.0), 90.0), 3))
    return out, twists


def roots(asset):
    a = copy.deepcopy(asset)
    a["rig"], info = rig()
    for an in a["anchors"]:
        if an["id"].startswith("track-"):
            an["z"] = F.GAUGE if an["z"] > 0 else -F.GAUGE
        if an["id"] == "stack-mouth":
            an.update({"x": -1.68, "y": 1.74, "z": -1.02})
        if an["id"] == "bucket-cog":
            an.update({"x": 0.62, "y": 0.36})
    poses = {"stowed": stowed(), "digging": at(6.6, 0.0, -100), "raised-boom": (40.0, -40.0, -140.0)}
    for pose in a["poses"]:
        pose["bones"] = K.offsets(*poses[pose["id"]])
    raised = K.chain(*poses["raised-boom"])
    cog = K.add(raised["G"], K.rot((0.62, 0.36), sum(poses["raised-boom"])))
    reach, height = round(cog[0], 1), round(cog[1] + K.DECK, 1)
    by = {p["id"]: p for p in a["poses"]}
    by["stowed"]["description"] = T("Parked for transport: boom lowered, stick folded under the boom and the fully curled bucket with its links resting just above the ground; engine and hydraulics cold.", "Für den Transport abgestellt: Ausleger gesenkt, Stiel unter den Ausleger geklappt, der ganz eingerollte Löffel mit seinen Laschen knapp über dem Boden; Motor und Hydraulik kalt.")
    by["digging"]["description"] = T("Bucket teeth on the ground 6.6 m from the swing axis with 1.3 t of soil in the bucket; engine at 80 % load.", "Löffelzähne auf dem Boden 6,6 m von der Schwenkachse mit 1,3 t Boden im Löffel; Motor bei 80 % Last.")
    by["raised-boom"]["description"] = T("Boom raised to 40 degrees with the stick stretched out level: the loaded bucket hangs %s m from the swing axis and %s m above the ground with 1.3 t, the tipping-critical lever arm; the rear track patches unload." % (reach, height), "Ausleger auf 40 Grad gehoben, Stiel waagerecht ausgestreckt: der beladene Löffel hängt %s m von der Schwenkachse und %s m über dem Boden mit 1,3 t, der kippkritische Hebelarm; die hinteren Kettenstandflächen werden entlastet." % (str(reach).replace(".", ","), str(height).replace(".", ",")))
    clips = {c["id"]: c for c in a["clips"]}
    clips["dig-cycle"]["keys"] = [{"t": t, "pose": "digging", "ease": "smooth", "bones": K.offsets(*at(x, y, phi))} for t, x, y, phi in dense([k + (0,) for k in DIG], 9.0, True, 0.0625)[0]]
    keys = []
    samples, twists = dense(DUMP, 6.0, False)
    for (t, x, y, phi), twist in zip(samples, twists):
        bones = K.offsets(*at(x, y, phi))
        bones["slew-ring"] = {"twistDeg": twist}
        keys.append({"t": t, "pose": "digging", "ease": "smooth", "bones": bones})
    last = at(*DUMP[-1][1:4])
    for i in range(1, 4):
        u = i / 4
        w = u * u * (3 - 2 * u)
        keys.append({"t": 6.0 + 0.125 * i, "pose": "digging", "ease": "smooth", "bones": K.offsets(*(last[k] + (poses["stowed"][k] - last[k]) * w for k in range(3)))})
    clips["dump-and-stow"]["keys"] = keys + [k for k in clips["dump-and-stow"]["keys"] if k["pose"] == "stowed"]
    clips["dig-cycle"]["description"] = T("A 9 s loop: reach out, crowd into the ground, curl and drag the bucket full 0.4 m below grade, boom up, dump at 2.7 m height and return. The bucket load rises from 0 to 1.3 t while digging and drops at the dump.", "Eine Schleife von 9 s: Ausholen, in den Boden drücken, den Löffel 0,4 m unter Gelände füllen und heranziehen, Ausleger heben, bei 2,7 m Höhe entleeren und zurück. Die Löffellast steigt beim Graben von 0 auf 1,3 t und fällt beim Entleeren.")
    a["summary"] = T("A 22 t diesel crawler excavator of the 20 t class: long tracked undercarriage with eight load-sharing ground contacts (four under each track), upper structure with cab, engine house, exhaust stack and counterweight, and a 5.7 m boom, 2.9 m stick and 1 m3 bucket driven by two boom cylinders, a stick cylinder and a bucket cylinder that turns the bucket through its two-link linkage.", "Ein 22-t-Kettenbagger der 20-t-Klasse mit Dieselmotor: langes Raupenfahrwerk mit acht lastverteilenden Bodenkontakten (vier unter jeder Kette), Oberwagen mit Kabine, Maschinenhaus, Auspuffrohr und Gegengewicht sowie 5,7-m-Ausleger, 2,9-m-Stiel und 1-m3-Löffel, angetrieben von zwei Auslegerzylindern, einem Stielzylinder und einem Löffelzylinder, der den Löffel über seine Zweilaschen-Kinematik dreht.")
    refs = a["provenance"]["references"]
    have = {r["id"] for r in refs}
    for rid, cite in (("cat-320", "Caterpillar (2017). 320 Hydraulic Excavator, technical specifications AEXQ2214: reach boom 5.7 m, stick R2.9, operating weight 22 500 kg, shipping length 9530 mm, cab height 2960 mm, tail swing radius 2830 mm, counterweight clearance 1050 mm, track length 4450 mm, length to centre of rollers 3650 mm, track gauge 2380 mm, transport width 2980 mm, stick cylinder 140 x 1504 mm, bucket cylinder 120 x 1104 mm."), ("komatsu-pc210lc-11", "Komatsu (2019). PC210LC-11 hydraulic excavator, specifications: operating weight 22 720 to 23 240 kg, boom 5.7 m, arm 2.9 m, shipping length 9650 to 9780 mm, height to top of boom 3140 to 3280 mm, cab height 3045 mm, tail swing radius 3020 mm, counterweight clearance 1085 mm, track gauge 2380 mm, transport width 2980 mm, 600 mm shoes, 49 shoes, 9 track and 2 carrier rollers per side, arm cylinder 135 x 1490 mm, digging depth 6620 mm, reach at ground 9700 mm, cutting height 10 000 mm, loading height 7110 mm.")):
        if rid not in have:
            refs.append({"id": rid, "citation": cite})
    a["provenance"]["notes"] = T("Diesel crawler excavator of the 20 t class with the long undercarriage (22.5 t); pin geometry, cylinder strokes and working envelope follow the Cat 320 and Komatsu PC210LC-11 data sheets. Slewing is the twist of the slew ring bone; the three cylinder pairs and the two links of the bucket linkage close the pin geometry at rest, in every pose and at every authored key. Thermal: engine compartment and stack heated by waste heat at engine-load level 1, hydraulic cylinders by hydraulic losses. The 110 kW class engine data are authored estimates tied to ISO 10265 (ground pressure) and Heywood (diesel exhaust).", "Diesel-Kettenbagger der 20-t-Klasse mit langem Laufwerk (22,5 t); Bolzengeometrie, Zylinderhübe und Arbeitsbereich folgen den Datenblättern von Cat 320 und Komatsu PC210LC-11. Das Schwenken ist die Verdrehung des Drehkranzknochens; die drei Zylinderpaare und die zwei Laschen der Löffelkinematik schließen die Bolzengeometrie in der Ruhelage, in jeder Haltung und an jedem Schlüsselbild. Thermik: Motorraum und Auspuffrohr werden bei Motorlaststufe 1 durch Abwärme beheizt, die Hydraulikzylinder durch hydraulische Verluste. Die Motordaten der 110-kW-Klasse sind Schätzwerte mit Bezug auf ISO 10265 (Bodenpressung) und Heywood (Dieselabgas).")
    a["dimensions"]["massKg"] = sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b)
    return a, info, poses


def main():
    scales = [int(s) for s in sys.argv[1:] if s.isdigit()] or [50, 100, 200, 500]
    doc = json.load(open(CATALOG, encoding="utf8"))
    a, info, poses = roots(doc["asset"])
    rg = L.Rig(a["rig"]["bones"])
    rest = rg.frames()
    for d in scales:
        S = L.Scale(d, rg, rest)
        load("r3-excavator-s%d" % d).build(S, info)
        rep = a["representations"][str(d)]
        rep["geometry"] = S.geometry()
        dec = getattr(S, "drawing", None)
        if dec:
            rep["decisions"].update(dec)
        a["volumes"][str(d)] = S.volume()
        print(d, "body", len(S.body), "thermal", len(S.thermal), "structure", len(S.structure), "shapes", {k: len(v) for k, v in rep["geometry"].items()})
    extra = load("r3-excavator-data") if os.path.exists(os.path.join(here, "r3-excavator-data.py")) else None
    if extra:
        extra.apply(a)
    doc["asset"] = a
    os.makedirs(os.path.dirname(DRAFT), exist_ok=True)
    L.dump_asset(doc, DRAFT)
    for cat in ("adhoc", "machinery") if "--final" in sys.argv else ("adhoc",):
        path = os.path.join(here, "volumes", cat, "excavator.json")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf8") as fh:
            json.dump({"volumes": a["volumes"]}, fh, ensure_ascii=False, indent=1)
    print("poses", {k: tuple(round(v, 2) for v in p) for k, p in poses.items()})
    print("draft", DRAFT)


if __name__ == "__main__":
    main()
