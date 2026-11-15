import json, sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
globals().update({n: getattr(L, n) for n in "box ell cap rev cyl ext loft rep shrink th st ref stref Scale write qz World".split()})

W = World("🛋️furniture", "park-bench")
seed = json.load(open("🗑️generated/author-16/seed-park-bench.json"))["volumes"]["50"]["body"]
SEED = {s["id"]: s for s in seed}
OK, SP = "oak-timber", "steel-painted"
SIDE = {"l": ("end-left", 0.0), "r": ("end-right", 1.56)}
X = {"l": 0.12, "r": 1.68}
BAND = [(0.52, 0.17), (0.52, 0.22), (0.82, 0.266), (0.82, 0.216)]
R_ANCH = {"l": "contact-left", "r": "contact-right"}


def ends(scale):
    out = []
    for k, (bone, ox) in SIDE.items():
        x = X[k]
        if scale == 100:
            out.append(W.box(bone, k + "-foot", SP, 0.35, (x, 0.025, -0.025), (0.124, 0.05, 0.55), r=0.008))
            for nm, z in (("front-leg", -0.26), ("rear-leg", 0.19)):
                out.append(W.box(bone, "%s-%s" % (k, nm), SP, 0.4, (x, 0.2125, z), (0.052, 0.325, 0.056), r=0.004))
            out.append(W.box(bone, k + "-bearer", SP, 0.45, (x, 0.388, -0.035), (0.08, 0.036, 0.51)))
            out.append(W.box(bone, k + "-brace", SP, 0.45, (x, 0.201, -0.035), (0.06, 0.022, 0.45)))
            out.append(W.box(bone, k + "-arm-post", SP, 0.4, (x, 0.5, -0.26), (0.032, 0.2, 0.032)))
            out.append(W.box(bone, k + "-armrest", SP, 0.42, (x, 0.617, -0.08), (0.09, 0.034, 0.44), r=0.012))
            out.append(W.cap(bone, k + "-post", SP, 0.4, (x, 0.4, 0.19), (x, 0.8285, 0.2797), 0.0155, 0.0135))
            out.append(W.ell(bone, k + "-finial", SP, 0.38, (x, 0.8325, 0.2805), (0.018, 0.0175, 0.0185)))
        elif scale == 200:
            out.append(W.box(bone, k + "-foot", SP, 0.35, (x, 0.025, -0.025), (0.124, 0.05, 0.55)))
            for nm, z in (("front-leg", -0.26), ("rear-leg", 0.19)):
                out.append(W.box(bone, "%s-%s" % (k, nm), SP, 0.4, (x, 0.23, z), (0.052, 0.36, 0.05)))
            out.append(W.box(bone, k + "-brace", SP, 0.45, (x, 0.2, -0.035), (0.06, 0.03, 0.45)))
            out.append(W.box(bone, k + "-arm-post", SP, 0.4, (x, 0.5, -0.26), (0.032, 0.2, 0.032)))
            out.append(W.box(bone, k + "-armrest", SP, 0.42, (x, 0.617, -0.08), (0.09, 0.034, 0.44)))
            out.append(W.cap(bone, k + "-post", SP, 0.4, (x, 0.4, 0.19), (x, 0.8285, 0.2797), 0.0175, 0.0175))
        else:
            out.append(W.box(bone, k + "-foot", SP, 0.35, (x, 0.0375, -0.025), (0.16, 0.075, 0.55)))
            out.append(W.box(bone, k + "-front-leg", SP, 0.4, (x, 0.317, -0.26), (0.16, 0.634, 0.075)))
            out.append(W.cap(bone, k + "-rear-post", SP, 0.4, (x, 0.0375, 0.1), (x, 0.8125, 0.2797), 0.0376, 0.0376))
    return out


def s50():
    S = Scale(50)
    S.body = [copy.deepcopy(s) for s in seed]
    for s in S.body:
        if "bolt" in s["id"]:
            s["profile"] = [[0, 0.0045], [0.008, 0.0045]]
    return S


def s100():
    S = Scale(100)
    b = ends(100)
    b.append(W.box("seat", "seat-plank", OK, 0.6, (0.9, 0.4275, -0.055), (1.8, 0.045, 0.49), r=0.008))
    b.append(W.extx("backrest", "back-band", OK, 0.55, BAND, 0.0, 1.8))
    S.body = b
    return S


def s200():
    S = Scale(200)
    b = ends(200)
    b.append(W.box("seat", "seat-plank", OK, 0.6, (0.9, 0.4275, -0.055), (1.8, 0.045, 0.49)))
    b.append(W.extx("backrest", "back-band", OK, 0.55, BAND, 0.0, 1.8))
    S.body = b
    return S


def s500():
    S = Scale(500)
    b = ends(500)
    b.append(W.box("seat", "seat-block", OK, 0.58, (0.9, 0.4425, -0.055), (1.8, 0.075, 0.49)))
    b.append(W.extx("backrest", "back-block", OK, 0.58, [(0.48, 0.17), (0.48, 0.266), (0.82, 0.266), (0.82, 0.17)], 0.0, 1.8))
    S.body = b
    return S


def region(b):
    i = b["id"]
    if i.startswith("seat"):
        return "seat-timber", "oak-slat"
    if i.startswith("back") and b["material"] == OK:
        return "back-timber", "oak-slat"
    return "steel-frame", "cast-style-steel"


def thermal(S):
    return [ref("th-" + b["id"], b["id"], *region(b)) for b in S.body]


def structure(S):
    d = S.d
    ids = {b["id"] for b in S.body}
    o = []
    o.append(st(W.box("ground", "member-stretcher", SP, 0.3, (0.9, 0.41, -0.035), (1.56, 0.02 if d < 200 else 0.031 if d == 200 else 0.076, 0.025 if d < 200 else 0.031 if d == 200 else 0.076)), "ground", "member"))
    for k, (bone, ox) in SIDE.items():
        if d == 50:
            o += [stref("member-%s-leg" % k, "%s-leg" % k, bone, "member"), stref("member-%s-bearer" % k, "%s-bearer" % k, bone, "member"), stref("member-%s-post" % k, "%s-post" % k, bone, "member")]
        elif d == 100:
            o += [stref("member-%s-front-leg" % k, "%s-front-leg" % k, bone, "member"), stref("member-%s-bearer" % k, "%s-bearer" % k, bone, "member"), stref("member-%s-post" % k, "%s-post" % k, bone, "member")]
        elif d == 200:
            o += [stref("member-%s-front-leg" % k, "%s-front-leg" % k, bone, "member"), stref("member-%s-post" % k, "%s-post" % k, bone, "member")]
        else:
            o += [stref("member-%s-front-leg" % k, "%s-front-leg" % k, bone, "member"), stref("member-%s-rear-post" % k, "%s-rear-post" % k, bone, "member")]
    if d == 50:
        o += [stref("member-seat-slat", "seat-slat", "seat", "member"), stref("member-back-slat", "back-slat", "backrest", "member")]
        for n in ("1", "2", "3"):
            for s in "ab":
                o.append(stref("joint-bolt-%s%s" % (n, s), "back-slat-%s-bolt-%s" % (n, s), "backrest", "joint"))
    elif d == 100:
        o += [stref("member-seat-plank", "seat-plank", "seat", "member"), stref("member-back-band", "back-band", "backrest", "member")]
        for s, x in (("a", 0.12), ("b", 1.68)):
            o.append(st(W.cyl("backrest", "joint-bolt-" + s, SP, 0.2, (x, 0.7, 0.24), (x, 0.7, 0.2), 0.0155, role="detail"), "backrest", "joint"))
    elif d == 200:
        o += [stref("member-seat-plank", "seat-plank", "seat", "member"), stref("member-back-band", "back-band", "backrest", "member")]
    else:
        o += [stref("member-seat-block", "seat-block", "seat", "member"), stref("member-back-block", "back-block", "backrest", "member")]
    sz = {50: (0.1, 0.008, 0.4), 100: (0.1, 0.016, 0.4), 200: (0.12, 0.031, 0.4), 500: (0.16, 0.076, 0.4)}[d]
    for k, (bone, ox) in SIDE.items():
        p = st(W.box(bone, "support-" + R_ANCH[k], SP, 0.15, (X[k], sz[1] / 2, -0.025), sz, role="detail"), bone, "support")
        p["anchor"] = R_ANCH[k]
        o.append(p)
        if d == 500:
            o.append(st(W.box(bone, "mass-" + k, SP, 0.15, (X[k], 0.32, -0.26), (0.12, 0.2, 0.077), role="detail"), bone, "mass"))
        else:
            o.append(st(W.ell(bone, "mass-" + k, SP, 0.15, (X[k], 0.2, -0.05), (0.03 if d < 200 else 0.045, 0.011 if d < 200 else 0.03, 0.2), role="detail"), bone, "mass"))
    return o


DEC = {
    "50": ("Detailed model from the seed: cast-style end frames (foot, tapered leg, brace, bearer, raked post with finial, arm post, rounded armrest), four oak seat slats with gaps, three raked back slats with bolt heads.", "Detailmodell aus dem Startmodell: gegossene Seitenteile (Fuß, konisches Bein, Strebe, Auflager, geneigter Pfosten mit Knauf, Armlehnenstütze, gerundete Armlehne), vier Eichensitzlatten mit Fugen, drei geneigte Rückenlatten mit Schraubenköpfen."),
    "100": ("Modelled anew: legs as plain boxes, seat slats merged into one plank, back slats merged into one raked band as the 1:100 drawing shows; bolts and brace are left out.", "Neu modelliert: Beine als schlichte Quader, Sitzlatten zu einer Bohle zusammengefasst, Rückenlatten zu einem geneigten Band wie in der Zeichnung 1:100; Schrauben und Strebe entfallen."),
    "200": ("Modelled anew: sharp legs and posts of 30 mm minimum section, no bearer, no finial, seat plank and raked band.", "Neu modelliert: scharfkantige Beine und Pfosten mit mindestens 30 mm Querschnitt, ohne Auflager und Knauf, Sitzbohle und geneigtes Band."),
    "500": ("Modelled anew: each end is a 75 mm foot, a 75 mm front column and a raked post; seat and back are 75 mm blocks; arm rest and slats are merged.", "Neu modelliert: jede Seite besteht aus einem 75-mm-Fuß, einer 75-mm-Vordersäule und einem geneigten Pfosten; Sitz und Lehne sind 75-mm-Blöcke; Armlehne und Latten sind zusammengefasst."),
}
DEC_T = {
    "50": ("Thermal: references by material to seat slats, back slats and steel parts (single layers). Structure: legs, bearers and posts of both ends, seat and back slats as members, the six bolt groups as joints, floor pads at both contact anchors, compact masses in the end frames, a stretcher plate under the seat on the ground bone.", "Thermik: Verweise nach Material auf Sitzlatten, Rückenlatten und Stahlteile (einzelne Schichten). Tragwerk: Beine, Auflager und Pfosten beider Seiten, Sitz- und Rückenlatten als Stäbe, die sechs Schraubengruppen als Gelenke, Bodenplatten an beiden Auflagern, kompakte Massen in den Seitenteilen, eine Verbindungsplatte unter dem Sitz am Bodenknochen."),
    "100": ("Thermal and structure follow the 1:100 body; two bolt joints.", "Thermik und Tragwerk folgen dem Körper 1:100; zwei Schraubengelenke."),
    "200": ("Thermal and structure follow the 1:200 body; the bolts are merged into the members.", "Thermik und Tragwerk folgen dem Körper 1:200; die Schrauben sind in die Stäbe aufgegangen."),
    "500": ("Thermal and structure follow the merged 1:500 body; bolts and joints are merged into the blocks.", "Thermik und Tragwerk folgen dem zusammengefassten Körper 1:500; Schrauben und Gelenke sind in den Blöcken aufgegangen."),
}

if __name__ == "__main__":
    sc = {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
    for k, S in sc.items():
        S.thermal = thermal(S)
        S.structure = structure(S)
        S.say(*DEC[k])
        S.say(*DEC_T[k])
    write("furniture", "park-bench", sc)
