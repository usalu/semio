from author_24_lib import *

FIT50 = False
floor_skip = ("shoe", "boot")
sym_skip = tuple(f"{p}-l" for p in ("upper-arm", "forearm", "hand", "thumb", "shoulder", "sleeve"))
PROFILE = [[-0.215, 0], [-0.2089, 0.0838], [-0.1934, 0.1175], [-0.1781, 0.1399], [-0.1502, 0.1681], [-0.1292, 0.1804], [-0.1051, 0.1878], [-0.0658, 0.1921], [-0.0143, 0.1892], [0.0371, 0.1761], [0.0702, 0.1634], [0.1132, 0.1347], [0.13, 0.128], [0.1474, 0.1278], [0.2, 0.14], [0.239, 0.1414], [0.2669, 0.1374], [0.2895, 0.1228], [0.3133, 0.0903], [0.3276, 0.062], [0.335, 0.026], [0.3276, -0.0527], [0.322, -0.075], [0.3024, -0.1105], [0.275, -0.134], [0.2581, -0.1396], [0.2191, -0.141], [0.191, -0.139], [0.1474, -0.1278], [0.13, -0.128], [0.1132, -0.1347], [0.06, -0.168], [0.0119, -0.1836], [-0.04, -0.192], [-0.0922, -0.19], [-0.1175, -0.1847], [-0.14, -0.175], [-0.1694, -0.1503], [-0.1934, -0.1175], [-0.2089, -0.0838], [-0.2135, -0.0484]]


def offset(pts, d):
    n = len(pts)
    area = sum(pts[i][0] * pts[(i + 1) % n][1] - pts[(i + 1) % n][0] * pts[i][1] for i in range(n)) / 2
    sgn = 1 if area > 0 else -1
    out = []
    for i in range(n):
        p0, p1, p2 = pts[i - 1], pts[i], pts[(i + 1) % n]
        nx = ny = 0.0
        for a, b in ((p0, p1), (p1, p2)):
            dx, dy = b[0] - a[0], b[1] - a[1]
            L = math.hypot(dx, dy) or 1
            nx += sgn * dy / L
            ny += -sgn * dx / L
        L = math.hypot(nx, ny) or 1
        k = d / max(L / 2, 0.5)
        out.append([p1[0] - nx / L * k, p1[1] - ny / L * k])
    return out


def circle(cx, cy, r, n=28):
    return [[cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)] for i in range(n)]


def hollow_guitar(b, wall):
    b[:] = [x for x in b if x["id"] not in ("guitar-body", "soundhole", "guitar-ribs", "guitar-top", "guitar-back")]
    inner = offset(PROFILE, wall)
    base = dict(kind="extrude", bone="guitar", material="spruce", role="body", plane="xy", profile=PROFILE)
    b.append({**base, "id": "guitar-ribs", "material": "mahogany", "tone": 0.4, "holes": [inner], "from": -0.055 + wall, "to": 0.055 - wall})
    b.append({**base, "id": "guitar-top", "tone": 0.72, "holes": [circle(0.075, 0.0, 0.0575)], "from": -0.055, "to": -0.055 + wall})
    b.append({**base, "id": "guitar-back", "material": "mahogany", "tone": 0.42, "from": 0.055 - wall, "to": 0.055})


def solid_guitar(b):
    b[:] = [x for x in b if x["id"] not in ("guitar-body", "soundhole", "guitar-ribs", "guitar-top", "guitar-back")]
    b.append(dict(kind="extrude", bone="guitar", material="spruce", role="body", plane="xy", profile=PROFILE, id="guitar-body", tone=0.6, **{"from": -0.055, "to": 0.055}))


def stool4(b):
    b[:] = [x for x in b if not x["id"].startswith(("stool-leg", "stool-rung"))]
    k = 0
    for sx in (1, -1):
        for sz in (1, -1):
            b.append(capsule(f"stool-leg-{k}", "stool", "pine", 0.62, [0.12 * sx, 0.52, 0.12 * sz], [0.17 * sx, 0.015, 0.17 * sz], 0.022, 0.02))
            k += 1
    pts = [(0.1467, 0.1467), (0.1467, -0.1467), (-0.1467, -0.1467), (-0.1467, 0.1467)]
    for i, (a, bb) in enumerate(zip(pts, pts[1:] + pts[:1])):
        b.append(capsule(f"stool-rung-{i}", "stool", "pine", 0.62, [a[0], 0.25, a[1]], [bb[0], 0.25, bb[1]], 0.012))


def repair50(vols):
    hollow_guitar(vols["50"]["body"], 0.0076)
    stool4(vols["50"]["body"])


DROP = {
    100: ["tuner-0", "tuner-1", "tuner-2"],
    200: ["tuner-0", "tuner-1", "tuner-2", "ear-l", "ear-r", "nose", "thumb-l", "thumb-r", "bridge"],
    500: ["tuner-0", "tuner-1", "tuner-2", "ear-l", "ear-r", "nose", "thumb-l", "thumb-r", "bridge", "stool-rung-0", "stool-rung-1", "stool-rung-2", "stool-rung-3", "shoulder-l", "shoulder-r", "knee-l", "knee-r"],
}


def post50(vols):
    pass


def special(b, s):
    if s == 100:
        hollow_guitar(b, 0.0151)
    if s == 200:
        hollow_guitar(b, 0.0302)
    if s == 500:
        solid_guitar(b)


def _spec():
    V = seed("seated-guitarist")
    spec = {}
    for t in V["50"]["thermal"]:
        if "of" in t:
            if t["of"] in ("soundhole",):
                continue
            ids = ["guitar-ribs", "guitar-top", "guitar-back", "guitar-body"] if t["of"] == "guitar-body" else [t["of"]]
            if t["of"].startswith("stool-leg") or t["of"].startswith("stool-rung"):
                continue
            for i in ids:
                spec.setdefault(t["region"], [])
                if i not in spec[t["region"]]:
                    spec[t["region"]].append(i)
    spec.setdefault("stool", [])
    spec["stool"] += [f"stool-leg-{k}" for k in range(4)] + [f"stool-rung-{k}" for k in range(4)]
    return spec


spec = _spec()
skip_members = ()
DECISIONS = {
    50: [
        D("Body: lofts of superelliptic sections for the flannel shirt trunk, sleeves (upper arms, forearms), mitten hands with thumbs, jeans over pelvis and thighs, boots, neck-less head with hair, ears and nose. The guitar is a hollow instrument: a ribs ring (extrusion of the body outline with an inner outline 7.6 mm inside as hole), a spruce top plate with a real 115 mm sound hole, a mahogany back plate, a bridge, a lofted neck and a lofted head with three tuners; the plates are 7.6 mm (the smallest feature, the real plates are 3 mm). The stool is a 40 mm pine seat puck on four splayed round legs (as the elevation and front drawings show) with a square ring of four rungs, replacing the three-legged draft whose middle leg stood where the elevation drawing shows none. The shirt is fitted with the fine raster to the front drawing.", "Körper: Lofts mit superelliptischen Querschnitten für den Flanellhemd-Rumpf, die Ärmel (Oberarme, Unterarme), Fäustlingshände mit Daumen, die Jeans über Becken und Oberschenkeln, Stiefel, Kopf mit Haar, Ohren und Nase. Die Gitarre ist ein hohles Instrument: ein Zargenring (Extrusion der Korpuskontur mit einer 7,6 mm eingerückten Innenkontur als Loch), eine Fichtendecke mit echtem 115-mm-Schallloch, ein Mahagoni-Boden, ein Steg, ein gelofteter Hals und ein gelofteter Kopf mit drei Mechaniken; die Platten sind 7,6 mm dick (das kleinste Merkmal, die echten Platten sind 3 mm). Der Hocker ist ein 40-mm-Kiefernsitz auf vier gespreizten runden Beinen (wie Aufriss und Vorderansicht zeigen) mit einem quadratischen Ring aus vier Sprossen und ersetzt den dreibeinigen Entwurf, dessen mittleres Bein dort stand, wo der Aufriss keines zeigt. Das Hemd ist mit dem feinen Raster an die Vorderansicht angepasst."),
        D("Thermal: references to the body solids that carry each region (the guitar region is ribs, top, back, neck, head, bridge and tuners; the stool region seat, legs and rungs); shirt, jeans and boots are nested shells per layer, innermost first; breath outlets are tagged capsules, the plume a horizontal disc held inside the hair top.", "Thermik: Verweise auf die tragenden Körper je Region (die Gitarrenregion besteht aus Zargen, Decke, Boden, Hals, Kopf, Steg und Mechaniken; die Hockerregion aus Sitz, Beinen und Sprossen); Hemd, Jeans und Stiefel sind verschachtelte Schalen je Schicht, innerste zuerst; Atemauslässe sind markierte Kapseln, die Fahne eine waagrechte Scheibe unter dem Haarscheitel."),
        D("Structure: member lofts with a core section along every body bone, cores of the stool seat and legs, a neck core in the guitar, ball joints at the rig joints and mass ellipsoids at the heavy segments; the weight goes from the boots up the legs into the pelvis, from there to the trunk and head, the guitar rests on the right thigh and is held by both arms, the stool legs carry the seat to the floor.", "Tragwerk: Stab-Lofts mit Kernquerschnitt entlang jedes Körperknochens, Kerne von Hockersitz und -beinen, ein Halskern in der Gitarre, Kugelgelenke an den Rig-Gelenken und Massenellipsoide an den schweren Segmenten; die Last geht von den Stiefeln über die Beine ins Becken, von dort in Rumpf und Kopf, die Gitarre ruht auf dem rechten Oberschenkel und wird von beiden Armen gehalten, die Hockerbeine tragen den Sitz zum Boden."),
    ],
    100: [
        D("1:100 is derived from 1:50 and refitted to the 1:100 drawings: the tuners are absorbed, the guitar stays hollow with 15 mm plates and ribs, sound hole and bridge as drawn, the hair is 10 % fuller to keep the exposed face near its area; thermal and structure follow the new solids.", "1:100 ist aus 1:50 abgeleitet und an die 1:100-Zeichnungen angepasst: die Mechaniken gehen auf, die Gitarre bleibt hohl mit 15-mm-Platten und -Zargen, Schallloch und Steg wie gezeichnet, das Haar ist 10 % voller, damit das frei liegende Gesicht nahe an seiner Fläche bleibt; Thermik und Tragwerk folgen den neuen Körpern."),
    ],
    200: [
        D("1:200 merges ears, nose, thumbs, the bridge and the tuners (below 30 mm); the guitar stays a hollow body with 30 mm plates, the stool keeps four legs and four rungs. Thermal: one surface per region; structure: member lofts with two sections.", "1:200 fasst Ohren, Nase, Daumen, Steg und Mechaniken (unter 30 mm) zusammen; die Gitarre bleibt ein Hohlkörper mit 30-mm-Platten, der Hocker behält vier Beine und vier Sprossen. Thermik: eine Oberfläche je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten."),
    ],
    500: [
        D("1:500 is a generalised silhouette model fitted with the fine raster and section shifts: the guitar body is one solid 110 mm slab (two 75 mm plates would not fit its depth), the stool has four 75 mm legs and the seat, rungs and small details are absorbed. Thermal: one solid per region; structure: member lofts with two sections.", "1:500 ist ein verallgemeinertes Silhouettenmodell, mit feinem Raster und Querschnittsverschiebung angepasst: der Gitarrenkorpus ist eine einzige 110-mm-Platte (zwei 75-mm-Platten passen nicht in seine Tiefe), der Hocker hat vier 75-mm-Beine und den Sitz, Sprossen und kleine Details gehen auf. Thermik: ein Körper je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten."),
    ],
}


def post_fit(b, s):
    if s == 100:
        for x in b:
            if x["id"] == "hair":
                for q in x["sections"][2:]:
                    q["rx"] *= 1.1
                    q["ry"] *= 1.1
    {100: lambda: hollow_guitar(b, 0.0151), 200: lambda: hollow_guitar(b, 0.0302), 500: lambda: solid_guitar(b)}[s]()


def struct_extra(a, body, scale):
    p = primary(body, "stool")
    out = []
    if p is not None and p["kind"] == "revolve":
        c = scaled(p, 0.7, "member-stool")
        c["member"], c["part"], c["role"] = "stool", "member", "body"
        out.append(c)
    f = FEATURE[scale]
    for x in body:
        if x["id"].startswith("stool-leg") and x["kind"] == "capsule":
            c = copy.deepcopy(x)
            c["id"] = "member-" + x["id"]
            c["r0"], c["r1"] = max(0.6 * x["r0"], 0.55 * f), max(0.6 * x["r1"], 0.55 * f)
            c["member"], c["part"] = "stool", "member"
            out.append(c)
    return out


def post_finish(vols):
    for sc in ("50", "100", "200", "500"):
        for m in vols[sc]["structure"]:
            if m["id"] in ("member-toe-l", "member-toe-r"):
                m["p0"] = [0.0, 0.03, 0.0]
                m["p1"] = [0.06, 0.03, 0.0]
            if m["id"].startswith("joint-upper-arm"):
                m["radii"] = [min(x, 0.5 * 0.0245 + 0.003) for x in m["radii"]]
    for sc in ("50", "100", "200", "500"):
        hair = next(x for x in vols[sc]["body"] if x["id"] == "hair")
        top = max(q["at"][0] for q in hair["sections"])
        for t in vols[sc]["thermal"]:
            if t["id"] == "th-plume":
                t["transform"]["translation"][0] = top - max(0.004, t["radii"][0])
                t["transform"]["rotation"] = [0, 0, 0.1564, 0.9877]


KIT50 = ["--sections --rounds 2 --bounds 0.85,1.2 --solid shirt,head,hair,upper-arm-l,upper-arm-r,forearm-l,forearm-r,thigh-l,thigh-r,shank-l,shank-r"]
