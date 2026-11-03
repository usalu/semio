import sys, copy
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️M2")
from lib import *

A = "tower-crane"
doc = load(A)
reps = doc["asset"]["representations"]
LA, MA, CO, WR, GL, SK, CS = "lattice", "machinery", "concrete", "wire-rope", "glass", "skip-steel", "steel-s355"
LAT, MACH = {"region": "lattice", "material": CS}, None


def L(e, ref, id, bone, z, pts, region="lattice", mat=CS, tone=None):
    r = e.get(ref) or {}
    kw = {}
    if "penMm" in r:
        kw["penMm"] = r["penMm"]
    return P(id, bone, z, tone if tone is not None else r.get("tone", 0.3), pts, region, mat, role=r.get("role", "body"), filled=False, closed=False, **kw)


def zig(a0, a1, w, period):
    pts, a, s = [], a0, -w
    pts.append((a, s))
    while a + period / 2 <= a1 + 1e-9:
        a += period / 2
        s = -s
        pts.append((a, s))
    return pts


def zigy(y0, y1, w, period):
    return [(x, y) for y, x in zig(y0, y1, w, period)]


SJ = {"kind": "extruded", "z0": -0.8, "z1": 0.8}
SC = {"kind": "extruded", "z0": -0.7, "z1": 0.7}


def JF(e, ref):
    r = e.get(ref) or {}
    kw = {"penMm": r["penMm"]} if "penMm" in r else {}
    return R("jib-frame", "jib", 1.8, r.get("tone", 0.3), 0, -0.8, 55, 1.6, "lattice", CS, role=r.get("role", "body"), filled=False, solid=SJ, **kw)


def TS_(e):
    t = e["jib-tip-sheave"]
    return E("jib-tip-sheave", "jib", 3.3, 0.3, t["cx"], t["cy"], t["rx"], t["ry"], None, CS, solid={"kind": "extruded", "z0": -0.1, "z1": 0.1})


def front(scale):
    e = {s["id"]: s for s in reps[str(scale)]["geometry"]["elevation"]}
    fine = scale <= 100
    s = [R("ballast-slab", "base", 1, 0.45, -1.75, 0, 3.5, 1, "ballast", CO),
         P("mast-foot", "base", 1.5, 0.8, [(-1.5, 1), (1.5, 1), (1.15, 1.5), (-1.15, 1.5)], LA, CS)]
    if scale == 500:
        s += [R("mast-silhouette", "mast", 1.8, 0.8, -1, 1, 2, 39, LA, CS),
              P("tower-head", "slewing", 1.8, 0.8, [(-0.9, 41.2), (0.9, 41.2), (0.12, 48.6), (-0.12, 48.6)], LA, CS),
              R("jib-silhouette", "jib", 1.8, 0.8, 0, -0.8, 55, 1.6, LA, CS, solid=SJ),
              R("counter-silhouette", "counter-jib", 1.7, 0.8, 0, -0.7, 16.5, 1.4, LA, CS, solid=SC)]
    else:
        pn = "mast-chords"
        s += [L(e, pn, "mast-chord-l", "mast", 1.8, [(1, 1), (1, 40)]), L(e, pn, "mast-chord-r", "mast", 1.8, [(-1, 1), (-1, 40)]),
              L(e, "mast-diagonals", "mast-diagonals", "mast", 1.8, zigy(1, 39, 1, 4 if scale == 200 else 2)),
              L(e, "mast-end-ties" if scale == 200 else "mast-horizontals", "mast-ties", "mast", 1.8, [(-1, 40), (1, 40)])]
        if fine:
            for k in range(3, 39, 2):
                s.append(L(e, "mast-horizontals", f"mast-tie-{k}", "mast", 1.8, [(-1, k), (1, k)]))
        s += [L(e, "tower-head-legs" if scale == 200 else "tower-head", "tower-head-leg-l", "slewing", 1.8, [(0.9, 41.2), (0.12, 48.6)]),
              L(e, "tower-head-legs" if scale == 200 else "tower-head", "tower-head-leg-r", "slewing", 1.8, [(-0.9, 41.2), (-0.12, 48.6)]),
              L(e, "tower-head-diagonals", "tower-head-diagonals", "slewing", 1.8, zigy(41.2, 46.8, 0.5, 2.8))]
        if "tower-head-ties" in e:
            s.append(L(e, "tower-head-ties", "tower-head-ties", "slewing", 1.8, [(-0.5, 44), (0.5, 44)]))
        s += [
              JF(e, "jib-chords" if "jib-chords" in e else "jib-outline"),
              R("counter-jib", "counter-jib", 1.7, 0.8, 0, -0.7, 16.5, 1.4, LA, CS, solid=SC)]
        if fine:
            s += [R("head-sheave", "slewing", 3.3, 0.3, -0.1, 48.35, 0.2, 0.5, None, CS), TS_(e),
                  R("trolley-sheave", "trolley", 4.2, 0.25, -0.1, 43.1, 0.2, 0.36, None, CS), R("block-sheave", "hoist-block", 4.6, 0.5, -0.1, 36.87, 0.2, 0.36, None, SK)]
    s += [R("slew-platform", "slewing", 2, 0.55, -1.2, 40.5, 2.4, 0.7, ["house", "slew-drive"] if scale == 500 else "house", MA),
          R("cab", "slewing", 2, 0.55, -1, 41.2, 1, 2.2, "cab", MA, r=0.1),
          R("cab-glass", "slewing", 3, 0.85, -0.85, 41.7, 0.7, 1.5, "cab-glazing", GL, role="glazing"),
          R("counterweight-slabs", "counterweight", 0.9, 0.4, -1.2, 41.5, 2.4, 2.2, "counterweight", CO),
          R("winch-house", "winch", 0.95, 0.55, -0.7, 45.15, 1.4, 1.8, ["winch-house", "hoist-motor"] if scale == 500 else "winch-house", MA),
          R("trolley-frame", "trolley", 4, 0.55, -1, 43.05, 2, 0.55, "trolley", MA),
          R("block-body", "hoist-block", 4.5, 0.45, -0.28, 36.8, 0.56, 0.5, "skip", SK),
          P("skip-body", "skip", 4.4, 0.45, [(-0.7, 35.8), (0.7, 35.8), (0.45, 34.8), (-0.45, 34.8)], "skip", SK),
          L(e, "slings", "sling-l", "skip", 4.4, [(-0.12, 36.8), (-0.7, 35.8)], "pendants", WR), L(e, "slings", "sling-r", "skip", 4.4, [(0.12, 36.8), (0.7, 35.8)], "pendants", WR),
          L(e, "hook", "hook", "hoist-block", 4.5, [(0, 36.8), (0, 36.08)], "skip", SK)]
    if scale <= 100:
        s += [R("trolley-wheels", "trolley", 4.1, 0.2, -0.95, 43.48, 1.9, 0.12, None, MA), R("skip-gate", "skip", 4.5, 0.25, -0.2, 34.65, 0.4, 0.15, "skip", SK)]
    if scale <= 200:
        s += [R("slew-ring", "slewing", 2.1, 0.35, -1, 40, 2, 0.5, "slew-drive", MA)]
    if scale == 200:
        s += [R("winch-motor-grille", "winch", 2.1, 0.25, -0.4, 45.55, 0.8, 0.8, "hoist-motor", MA)]
    for k, (ref, lat) in enumerate((("pendant-fore-1", 0.15), ("pendant-fore-2", -0.15), ("pendant-back", 0))):
        s.append(L(e, ref, ref, "slewing", 4, [(lat, 45.2), (lat * 0.8, 48.6)], "pendants", WR))
    for i in range(1, 9):
        y1 = 43.05 - 5 * (i - 1)
        s.append(L(e, f"rope-{i}-line", f"rope-{i}-line", f"rope-{i}", 3, [(0, y1 - 5.75), (0, y1)], "rope", WR))
    return s


def plan(scale):
    e = {s["id"]: s for s in reps[str(scale)]["geometry"]["elevation"]}
    fine = scale <= 100
    s = [R("ballast-slab", "base", 1, 0.45, -3.5, -1.75, 7, 3.5, "ballast", CO), R("mast-foot", "base", 1.5, 0.8, -1.5, -1.5, 3, 3, LA, CS),
         R("mast-silhouette", "mast", 1.8, 0.8, -1, -1, 2, 2, LA, CS)]
    s += [R("slew-platform", "slewing", 2, 0.55, -2.4, -1.2, 5.8, 2.4, ["house", "slew-drive"] if scale == 500 else "house", MA),
          R("cab", "slewing", 2.1, 0.55, 1.4, -1, 1.9, 1, "cab", MA, r=0.1)]
    if scale == 500:
        s += [R("tower-head", "slewing", 2.5, 0.8, -1.2, -0.9, 2.4, 1.8, LA, CS), R("jib-silhouette", "jib", 3, 0.8, 0, -0.8, 55, 1.6, LA, CS),
              R("counter-silhouette", "counter-jib", 2.9, 0.8, -16.5, -0.7, 16.5, 1.4, LA, CS)]
    else:
        s += [L(e, "tower-head-legs" if scale == 200 else "tower-head", "tower-head-frame", "slewing", 2.5, [(-1.2, -0.9), (1.2, -0.9), (1.2, 0.9), (-1.2, 0.9), (-1.2, -0.9)]),
              L(e, "tower-head-diagonals", "tower-head-cross-1", "slewing", 2.5, [(-1.2, -0.9), (1.2, 0.9)]), L(e, "tower-head-diagonals", "tower-head-cross-2", "slewing", 2.5, [(1.2, -0.9), (-1.2, 0.9)]),
              L(e, "jib-chords" if "jib-chords" in e else "jib-outline", "jib-chord-l", "jib", 3, [(0, 0.8), (55, 0.8)]), L(e, "jib-chords" if "jib-chords" in e else "jib-outline", "jib-chord-r", "jib", 3, [(0, -0.8), (55, -0.8)]),
              L(e, "jib-diagonals", "jib-diagonals", "jib", 3, zig(0, 54, 0.8, 4 if scale == 200 else 3)),
              L(e, "counter-chords" if "counter-chords" in e else "counter-outline", "counter-chord-l", "counter-jib", 2.9, [(-16.5, 0.7), (0, 0.7)]),
              L(e, "counter-chords" if "counter-chords" in e else "counter-outline", "counter-chord-r", "counter-jib", 2.9, [(-16.5, -0.7), (0, -0.7)]),
              L(e, "counter-diagonals", "counter-diagonals", "counter-jib", 2.9, zig(-16.5, 0, 0.7, 3))]
        if fine:
            s += [R("head-sheave", "slewing", 3.3, 0.3, -0.25, -0.1, 0.5, 0.2, None, CS), R("jib-tip-sheave", "jib", 3.3, 0.3, 54.7, -0.1, 0.6, 0.2, None, CS),
                  L(e, "jib-rail", "jib-rail-l", "jib", 3.1, [(0, 0.45), (55, 0.45)]), L(e, "jib-rail", "jib-rail-r", "jib", 3.1, [(0, -0.45), (55, -0.45)])]
    s += [R("counterweight-slabs", "counterweight", 1.9, 0.4, -16.1, -1.2, 2.4, 2.4, "counterweight", CO),
          R("winch-house", "winch", 3.3, 0.55, -10, -0.7, 4, 1.4, ["winch-house", "hoist-motor"] if scale == 500 else "winch-house", MA),
          R("trolley-frame", "trolley", 2.9, 0.55, 2.3, -1, 1.4, 2, "trolley", MA),
          R("block-body", "hoist-block", 2, 0.45, 2.72, -0.28, 0.56, 0.56, "skip", SK), R("skip-body", "skip", 1.9, 0.45, 2.3, -0.7, 1.4, 1.4, "skip", SK)]
    if scale <= 200:
        s += [R("winch-motor-grille", "winch", 3.4, 0.25, -9.6, -0.4, 1.2, 0.8, "hoist-motor", MA)]
    for ref, lat, x0, x1 in (("pendant-fore-1", 0.15, 0, 22), ("pendant-fore-2", -0.15, 0, 50), ("pendant-back", 0, -15.6, 0)):
        s.append(L(e, ref, ref, "slewing", 4, [(x0, lat), (x1, lat)], "pendants", WR))
    return s


TXT = {
    500: ("a composed silhouette: from the front the mast, A-frame head, jib end and counterweight as toned masses with the pendants and eight rope pieces as lines; from above the ballast, mast foot, slewing platform, cab, jib, counter-jib, winch house and counterweight.",
          "eine zusammengesetzte Silhouette: von vorn Mast, A-Bock, Auslegerende und Gegengewicht als getönte Massen mit den Abspannungen und acht Seilstücken als Linien; von oben Ballast, Mastfuß, Drehbühne, Kabine, Ausleger, Gegenausleger, Windenhaus und Gegengewicht."),
    200: ("the mast as a lattice of two chords, zigzag diagonals and end ties seen end-on, the jib as an open frame with crossing diagonals; from above the jib and counter-jib as chords with zigzag diagonals; trolley, block, skip and ropes follow their prismatic bones.",
          "der Mast als Gitter aus zwei Gurten, Zickzackdiagonalen und Endriegeln in Stirnansicht, der Ausleger als offener Rahmen mit Kreuzdiagonalen; von oben Ausleger und Gegenausleger als Gurte mit Zickzackdiagonalen; Laufkatze, Flasche, Kübel und Seile folgen ihren Schubknochen."),
    100: ("as at 1:200 with horizontal ties every 2 m, sheaves edge-on, trolley wheels, the skip gate and the trolley rails from above.",
          "wie bei 1:200 mit Querriegeln alle 2 m, hochkant stehenden Seilscheiben, Laufkatzenrädern, Kübelklappe und den Katzfahrschienen von oben."),
    50: ("as at 1:100 with the full set of ties and rails; the lattice pens are those of the elevation.",
         "wie bei 1:100 mit dem vollen Satz Riegel und Schienen; die Gitterstifte sind die des Aufrisses."),
}
for sc in (50, 100, 200, 500):
    put(doc, sc, front(sc), plan(sc), *TXT[sc])
save(A, doc)
print("ok")
