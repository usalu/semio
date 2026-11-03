"""H3 scratch library: rest-pose solver, style lookup, shape factories and the standing/seated human builders.
All coordinates are model metres in the rig rest pose; every number is chosen per asset in its own script."""
import json, math, re, os, sys

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\👤️humans"
SCALES = ("50", "100", "200", "500")


def load(a):
    return json.load(open(f"{ROOT}\\{a}\\🔣️.json", encoding="utf8"))


def save(a, doc):
    sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️FW")
    from atomic_json import write_json_atomic
    write_json_atomic(os.path.join(ROOT, a, "🔣️.json"), doc)


class Rest:
    def __init__(s, asset):
        s.b = {b["id"]: b for b in asset["rig"]["bones"]}
        s.w = {}
        def solve(i):
            if i in s.w: return s.w[i]
            b = s.b[i]
            if b["parent"] is None:
                s.w[i] = (b["x"], b["y"], b["angleDeg"]); return s.w[i]
            px, py, pa = solve(b["parent"])
            c, n = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            s.w[i] = (px + b["x"] * c - b["y"] * n, py + b["x"] * n + b["y"] * c, pa + b["angleDeg"])
            return s.w[i]
        for i in s.b: solve(i)

    def o(s, i): return s.w[i][0], s.w[i][1]

    def t(s, i):
        x, y, a = s.w[i]; L = s.b[i]["lengthM"]
        return x + L * math.cos(math.radians(a)), y + L * math.sin(math.radians(a))

    def p(s, i, lx, ly):
        x, y, a = s.w[i]; c, n = math.cos(math.radians(a)), math.sin(math.radians(a))
        return x + lx * c - ly * n, y + lx * n + ly * c


def r4(v): return round(v + 0.0, 4)


def pts_bbox(s):
    k = s["kind"]
    if k == "capsule": return (min(s["x0"] - s["r0"], s["x1"] - s["r1"]), min(s["y0"] - s["r0"], s["y1"] - s["r1"]), max(s["x0"] + s["r0"], s["x1"] + s["r1"]), max(s["y0"] + s["r0"], s["y1"] + s["r1"]))
    if k == "ellipse": return (s["cx"] - s["rx"], s["cy"] - s["ry"], s["cx"] + s["rx"], s["cy"] + s["ry"])
    if k == "rect": return (s["x"], s["y"], s["x"] + s["w"], s["y"] + s["h"])
    if k == "poly":
        xs = [p[0] for p in s["points"]]; ys = [p[1] for p in s["points"]]; return (min(xs), min(ys), max(xs), max(ys))
    n = [float(v) for v in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", s["d"])]
    return (min(n[0::2]), min(n[1::2]), max(n[0::2]), max(n[1::2]))


class Styles:
    """Looks up tone, region(s) and hatch of an elevation shape by id (same scale first)."""
    def __init__(s, asset):
        s.asset = asset
        s.reps = {k: v for k, v in asset["representations"].items() if v.get("status") == "authored"}
        s.by = {k: {sh["id"]: sh for sh in v["geometry"]["elevation"]} for k, v in s.reps.items()}

    def get(s, scale, ids, tone=None, plain=False):
        if isinstance(ids, str): ids = [ids]
        order = [scale] + [k for k in ("100", "50", "200", "500") if k != scale]
        hatches = {h["id"] for h in s.reps[scale].get("hatches", [])}
        for i in ids:
            for k in order:
                sh = s.by.get(k, {}).get(i)
                if sh:
                    d = {"tone": sh.get("tone", 0.5)}
                    if "region" in sh: d["region"] = sh["region"]
                    if "regions" in sh: d["regions"] = sh["regions"]
                    if "material" in sh: d["material"] = sh["material"]
                    h = s.by[scale].get(i, {}).get("hatch") if i in s.by[scale] else None
                    if h and h in hatches and not plain: d["hatch"] = h
                    if tone is not None: d["tone"] = tone
                    return d
        raise KeyError(f"no style for {ids} at {scale}")

    def bbox(s, scale, ids):
        if isinstance(ids, str): ids = [ids]
        order = [scale] + [k for k in ("100", "50", "200", "500") if k != scale]
        for i in ids:
            for k in order:
                sh = s.by.get(k, {}).get(i)
                if sh: return pts_bbox(sh)
        raise KeyError(ids)


class Out:
    """Collects the shapes of one view at one scale, checks id uniqueness and pen availability."""
    def __init__(s, pens):
        s.pens = pens; s.items = []; s.ids = set()

    def add(s, d):
        assert d["id"] not in s.ids, "duplicate id " + d["id"]
        if d["role"] != "body" and s.pens.get(d["role"]) is None: return
        s.ids.add(d["id"]); s.items.append(d)

    def fin(s): return s.items


def _finish(d, id, bone, role, z, filled, style):
    d.update({"id": id, "bone": bone, "role": role, "z": z, "filled": filled})
    if filled:
        d.update(style)
    else:
        d["tone"] = style.get("tone", 0.5)
    return d


def cap(o, id, bone, x0, y0, x1, y1, r0, r1, z, style, follow=False, role="body"):
    d = {"kind": "capsule", "x0": r4(x0), "y0": r4(y0), "x1": r4(x1), "y1": r4(y1), "r0": r4(r0), "r1": r4(r1)}
    if follow: d["followTip"] = True
    o.add(_finish(d, id, bone, role, z, True, style))


def ell(o, id, bone, cx, cy, rx, ry, z, style, role="body", rot=None):
    d = {"kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(rx), "ry": r4(ry)}
    if rot: d["rotationDeg"] = rot
    o.add(_finish(d, id, bone, role, z, True, style))


def rect(o, id, bone, x, y, w, h, z, style, r=None, follow=False, role="body"):
    d = {"kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)}
    if r: d["r"] = r4(r)
    if follow: d["followTip"] = True
    o.add(_finish(d, id, bone, role, z, True, style))


def poly(o, id, bone, pts, z, style, follow=False, role="body", closed=True, filled=True):
    d = {"kind": "poly", "points": [[r4(a), r4(b)] for a, b in pts], "closed": closed}
    if follow: d["followTip"] = True
    o.add(_finish(d, id, bone, role, z, filled, style))


def line(o, id, bone, pts, z, tone=0.5, role="detail", follow=False):
    poly(o, id, bone, pts, z, {"tone": tone}, follow=follow, role=role, closed=False, filled=False)


def limb(cx, ya, yb, wa, wb, bulge=0.0):
    """Closed outline of a vertical limb from (cx, ya) of half width wa to (cx, yb) of half width wb, with a mid bulge."""
    ym = (ya + yb) / 2; wm = (wa + wb) / 2 + bulge
    return [(cx - wa, ya), (cx - wm, ym), (cx - wb, yb), (cx + wb, yb), (cx + wm, ym), (cx + wa, ya)]


def tier(scale): return {"500": 0, "200": 1, "100": 2, "50": 3}[scale]


BASE_EN = {"500": "head, body, arms and legs as flat masses on the ground line (plan: the same masses seen from above)",
           "200": "head with hair cap, neck, body, arms and legs with shoulder, elbow and knee joints, hands and shoes (plan: head with face, shoulders, trunk, limbs and shoes from above)",
           "100": "head with hair, ears and face, neck, body with hip line, joint balls at shoulders, elbows and knees, hands with fists, shoes with toe caps (plan: head with face, shoulders, trunk, arms with hands, legs and shoes)",
           "50": "eyes, brows, nose and mouth, ears, hair with fringe, soles, hems and cuffs besides everything of 1:100 (plan: nose, ears, parting, shoulder seams and laces besides everything of 1:100); limbs follow the pose by the bone travel, so strides, bent knees, lean and raised arms change both views"}
BASE_DE = {"500": "Kopf, Körper, Arme und Beine als flache Massen auf der Standlinie (Grundriss: dieselben Massen von oben)",
           "200": "Kopf mit Haarkappe, Hals, Körper, Arme und Beine mit Schulter-, Ellbogen- und Kniegelenken, Hände und Schuhe (Grundriss: Kopf mit Gesicht, Schultern, Rumpf, Glieder und Schuhe von oben)",
           "100": "Kopf mit Haar, Ohren und Gesicht, Hals, Körper mit Hüftlinie, Gelenkkugeln an Schultern, Ellbogen und Knien, Hände mit Fäusten, Schuhe mit Kappen (Grundriss: Kopf mit Gesicht, Schultern, Rumpf, Arme mit Händen, Beine und Schuhe)",
           "50": "Augen, Brauen, Nase und Mund, Ohren, Haar mit Pony, Sohlen, Säume und Bündchen zusätzlich zu allem aus 1:100 (Grundriss: Nase, Ohren, Scheitel, Schulternähte und Schnürung zusätzlich zu allem aus 1:100); die Glieder folgen der Pose über den Knochenweg, sodass Schritte, gebeugte Knie, Neigung und erhobene Arme beide Ansichten verändern"}


def notes(extra_en=None, extra_de=None):
    e, d = extra_en or {}, extra_de or {}
    return {sc: {"en": BASE_EN[sc] + ". " + e.get(sc, "").strip(), "de": BASE_DE[sc] + ". " + d.get(sc, "").strip()} for sc in SCALES}


MARK_EN, MARK_DE =" Front view:", " Vorderansicht:"


def write(asset_id, build, notes, keep_text=False):
    """Rebuilds front and plan of every authored scale with build(asset, scale, front_out, plan_out) and extends the decisions."""
    doc = load(asset_id); a = doc["asset"]
    for sc, rep in a["representations"].items():
        if rep.get("status") != "authored": continue
        of, op = Out(rep["pens"]), Out(rep["pens"])
        build(a, sc, of, op)
        g = rep["geometry"]
        rep["geometry"] = {"elevation": g["elevation"], "front": of.fin(), "plan": op.fin(), **{k: v for k, v in g.items() if k not in ("elevation", "front", "plan")}}
        n = notes[sc]; sil = rep["decisions"]["silhouette"]
        for lang, mark in (("en", MARK_EN), ("de", MARK_DE)):
            txt = sil[lang]
            if mark in txt: txt = txt[:txt.index(mark)]
            sil[lang] = txt.rstrip() + mark + " " + n[lang].strip()
    a["presentation"].pop("viewpoints", None)
    save(asset_id, doc)
    return {sc: (len(r["geometry"]["front"]), len(r["geometry"]["plan"])) for sc, r in a["representations"].items() if r.get("status") == "authored"}


def sol(o, z0, z1):
    """Marks the last shape as an extruded solid: its bone-local profile is placed with the full bone transform."""
    if o.items and o.items[-1]["id"] in o.ids:
        o.items[-1]["solid"] = {"kind": "extruded", "z0": r4(z0), "z1": r4(z1)}
