"""H1 scratch library: rest-pose helpers and shape builders for handcrafted front and plan views.

Coordinates are model metres in the rig rest pose (see the brief). Every number is decided in the asset scripts; this module only
computes forward kinematics of the rest pose, builds shape dictionaries in the file's key order and writes the asset file.
"""
import json, math, os, re

CAT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"


def r4(v):
    return round(v + 0.0, 5)


class Ctx:
    def __init__(self, aid):
        self.aid = aid
        self.path = f"{CAT}/{aid}/🔣️.json"
        self.doc = json.load(open(self.path, encoding="utf-8"))
        self.a = self.doc["asset"]
        self.bones = {b["id"]: b for b in self.a["rig"]["bones"]}
        self.O, self.T, self.ang = {}, {}, {}
        for b in self.a["rig"]["bones"]:  # parents first in file order
            pid = b["parent"]
            if pid is None:
                px, py, pa = 0.0, 0.0, 0.0
            else:
                pa = self.ang[pid]
                px, py = self.O[pid]
                pl = self.bones[pid]["lengthM"]
                # local origin is expressed in the parent frame (parent origin + rotation)
            x, y = b["x"], b["y"]
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            ox, oy = px + c * x - s * y, py + s * x + c * y
            a = pa + b["angleDeg"]
            self.O[b["id"]] = (ox, oy)
            self.ang[b["id"]] = a
            self.T[b["id"]] = (ox + b["lengthM"] * math.cos(math.radians(a)), oy + b["lengthM"] * math.sin(math.radians(a)))
        self.reps = self.a["representations"]
        self.scales = [int(k) for k, v in self.reps.items() if v["status"] == "authored"]
        self.front, self.plan = [], []
        self.scale = None

    def W(self, bone, lx, ly):
        a = math.radians(self.ang[bone])
        ox, oy = self.O[bone]
        return (ox + math.cos(a) * lx - math.sin(a) * ly, oy + math.sin(a) * lx + math.cos(a) * ly)

    def restbox(self, shape):
        """Rest-pose world bounding box of an elevation shape (control-point bounds for paths)."""
        k = shape["kind"]
        pts = []
        if k == "path":
            d = shape["d"]
            nums = [float(v) for v in re.findall(r"-?\d*\.?\d+(?:e-?\d+)?", d)]
            cmds = re.findall(r"[MLCQZHVmlcqzhv]|-?\d*\.?\d+(?:e-?\d+)?", d)
            cur, i, x, y = None, 0, 0.0, 0.0
            vals = []
            for t in cmds:
                if t.isalpha():
                    cur = t
                    continue
                vals.append(float(t))
                if cur in "MLCQ" and len(vals) % 2 == 0:
                    pts.append((vals[-2], vals[-1]))
                elif cur == "H":
                    pts.append((vals[-1], y)); x = vals[-1]
                elif cur == "V":
                    pts.append((x, vals[-1])); y = vals[-1]
                if pts:
                    x, y = pts[-1]
        elif k == "poly":
            pts = [tuple(p) for p in shape["points"]]
        elif k == "rect":
            pts = [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"])]
        elif k == "ellipse":
            r = max(shape["rx"], shape["ry"])
            pts = [(shape["cx"] - r, shape["cy"] - r), (shape["cx"] + r, shape["cy"] + r)]
        elif k == "capsule":
            pts = [(shape["x0"] - shape["r0"], shape["y0"] - shape["r0"]), (shape["x1"] + shape["r1"], shape["y1"] + shape["r1"])]
        w = [self.W(shape["bone"], px, py) for px, py in pts]
        return min(p[0] for p in w), max(p[0] for p in w), min(p[1] for p in w), max(p[1] for p in w)

    # -- region lookup from the elevation of the current representation ---------------------------------------------------
    def info(self, region):
        for sc in [self.scale] + self.scales:
            for s in self.reps[str(sc)]["geometry"]["elevation"]:
                if s.get("region") == region and "material" in s:
                    return s["material"], s.get("tone", 0.5)
        for sc in self.scales:
            for s in self.reps[str(sc)]["geometry"]["elevation"]:
                if region in (s.get("regions") or []) and "material" in s:
                    return s["material"], s.get("tone", 0.5)
        return None, 0.5

    def start(self, scale):
        self.scale = scale
        self.front, self.plan = [], []
        self.ids = {"front": set(), "plan": set()}

    def add(self, view, geom, id, bone, z, region=None, regions=None, role="body", filled=True, tone=None, hatch=None, pen=None, material=None, solid=None):
        assert bone in self.bones, bone
        assert id not in self.ids[view], f"duplicate {view} id {id}"
        self.ids[view].add(id)
        shape = dict(geom)
        shape.update({"id": id, "bone": bone, "role": role, "z": z, "filled": filled})
        key = region or (regions[0] if regions else None)
        mat, tn = self.info(key) if key else (None, 0.5)
        shape["tone"] = tone if tone is not None else tn
        if region:
            shape["region"] = region
        if regions:
            shape["regions"] = list(regions)
        m = material or mat
        if m:
            shape["material"] = m
        if solid:
            shape["solid"] = solid
        if hatch:
            shape["hatch"] = hatch
        if pen:
            shape["penMm"] = pen
        (self.front if view == "front" else self.plan).append(shape)

    def L(self, bone, x, y):
        """Rest world point -> bone-local point (inverse of W)."""
        a = math.radians(self.ang[bone])
        ox, oy = self.O[bone]
        dx, dy = x - ox, y - oy
        return (math.cos(a) * dx + math.sin(a) * dy, -math.sin(a) * dx + math.cos(a) * dy)

    def elev(self, eid):
        for sh in self.reps[str(self.scale)]["geometry"]["elevation"]:
            if sh["id"] == eid:
                return sh
        raise KeyError(eid)

    def extrude_like(self, view, id, bone, eid, z0, z1, **kw):
        """Front or plan solid whose bone-local profile is the elevation shape `eid` of this scale, extruded across z0..z1 (lateral, left +)."""
        e = self.elev(eid)
        g = {k: e[k] for k in ("kind", "points", "closed", "d", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rotationDeg", "x0", "y0", "x1", "y1", "r0", "r1") if k in e}
        self.add(view, g, id, bone, solid={"kind": "extruded", "z0": r4(z0), "z1": r4(z1)}, **kw)

    def extrude_poly(self, view, id, bone, pts, z0, z1, **kw):
        self.add(view, {"kind": "poly", "points": [[r4(x), r4(y)] for x, y in pts], "closed": True}, id, bone, solid={"kind": "extruded", "z0": r4(z0), "z1": r4(z1)}, **kw)

    def bbox_local(self, shape):
        """Bone-local bounding box (x0, x1, y0, y1) of an elevation shape (control points for paths)."""
        k = shape["kind"]
        if k == "path":
            nums = [float(v) for v in re.findall(r"-?\d*\.?\d+(?:e-?\d+)?", shape["d"])]
            pts = list(zip(nums[0::2], nums[1::2]))
        elif k == "poly":
            pts = [tuple(p) for p in shape["points"]]
        elif k == "rect":
            pts = [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"])]
        elif k == "ellipse":
            pts = [(shape["cx"] - shape["rx"], shape["cy"] - shape["ry"]), (shape["cx"] + shape["rx"], shape["cy"] + shape["ry"])]
        else:
            pts = [(shape["x0"] - shape["r0"], shape["y0"] - shape["r0"]), (shape["x1"] + shape["r1"], shape["y1"] + shape["r1"])]
        return min(p[0] for p in pts), max(p[0] for p in pts), min(p[1] for p in pts), max(p[1] for p in pts)

    def slices(self, view, id, bone, box, widths, zc, along="x", **kw):
        """A rounded outline as stepped extruded slabs: the local box is cut into len(widths) slabs along its x (or y) axis, slab i is extruded over zc +- widths[i]."""
        x0, x1, y0, y1 = box
        n = len(widths)
        for i, w in enumerate(widths):
            lo, hi = (x0, x1) if along == "x" else (y0, y1)
            a = lo + (hi - lo) * i / n - (0.001 if i else 0.0)
            b = lo + (hi - lo) * (i + 1) / n + (0.001 if i < n - 1 else 0.0)
            pts = [(a, y0), (b, y0), (b, y1), (a, y1)] if along == "x" else [(x0, a), (x1, a), (x1, b), (x0, b)]
            self.extrude_poly(view, f"{id}-{i}", bone, pts, zc - w, zc + w, **kw)

    def retag(self, view, id, region):
        for sh in (self.front if view == "front" else self.plan):
            if sh["id"] == id:
                m, t = self.info(region)
                sh["region"] = region
                sh.pop("regions", None)
                if m:
                    sh["material"] = m
                sh["tone"] = t
                return
        raise KeyError(id)

    def drop(self, view, ids):
        lst = self.front if view == "front" else self.plan
        keep = [s for s in lst if s["id"] not in ids]
        lst[:] = keep
        for i in ids:
            self.ids[view].discard(i)

    # builders -----------------------------------------------------------------------------------------------------------
    def cap(self, view, id, bone, p0, p1, r0, r1=None, follow=True, **kw):
        g = {"kind": "capsule", "x0": r4(p0[0]), "y0": r4(p0[1]), "x1": r4(p1[0]), "y1": r4(p1[1]), "r0": r4(r0), "r1": r4(r0 if r1 is None else r1)}
        if follow:
            g["followTip"] = True
        self.add(view, g, id, bone, **kw)

    def ell(self, view, id, bone, c, rx, ry, rot=None, **kw):
        g = {"kind": "ellipse", "cx": r4(c[0]), "cy": r4(c[1]), "rx": r4(rx), "ry": r4(ry)}
        if rot:
            g["rotationDeg"] = rot
        self.add(view, g, id, bone, **kw)

    def rect(self, view, id, bone, x, y, w, h, r=None, follow=False, **kw):
        g = {"kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)}
        if r:
            g["r"] = r4(r)
        if follow:
            g["followTip"] = True
        self.add(view, g, id, bone, **kw)

    def poly(self, view, id, bone, pts, closed=True, follow=False, **kw):
        g = {"kind": "poly", "points": [[r4(x), r4(y)] for x, y in pts], "closed": closed}
        if follow:
            g["followTip"] = True
        if not closed:
            kw.setdefault("filled", False)
        self.add(view, g, id, bone, **kw)

    def path(self, view, id, bone, d, **kw):
        self.add(view, {"kind": "path", "d": d}, id, bone, **kw)

    # -- limb outline for front view ---------------------------------------------------------------------------------------
    def limb(self, view, id, bone, xc, prof, t0=0.0, t1=1.0, wscale=1.0, cap0=0.0, cap1=0.0, follow=True, **kw):
        """Closed outline along a bone between t0 and t1 (0 = bone origin, 1 = bone tip) with half-width profile `prof` = [(t, w)].
        Round caps of height cap0 (beyond t0, away from t1) and cap1 (beyond t1) keep a collapsed, foreshortened limb visible."""
        (ox, oy), (tx, ty) = self.O[bone], self.T[bone]
        def yy(t):
            return oy + t * (ty - oy)
        def w_at(t):
            if t <= prof[0][0]:
                return prof[0][1]
            for (ta, wa), (tb, wb) in zip(prof, prof[1:]):
                if ta <= t <= tb:
                    return wa + (wb - wa) * (t - ta) / (tb - ta) if tb > ta else wa
            return prof[-1][1]
        ts = [t0] + [t for t, _ in prof if t0 < t < t1] + [t1]
        sgn = 1 if ty <= oy else -1
        if t1 < t0:
            raise ValueError("t1 < t0")
        left = [(xc - wscale * w_at(t), yy(t)) for t in ts]
        right = [(xc + wscale * w_at(t), yy(t)) for t in reversed(ts)]
        n = 6
        bot, top = [], []
        w1, w0 = wscale * w_at(t1), wscale * w_at(t0)
        if cap1 > 0:
            bot = [(xc - w1 * math.cos(math.pi * i / n), yy(t1) - sgn * cap1 * math.sin(math.pi * i / n)) for i in range(1, n)]
        if cap0 > 0:
            top = [(xc + w0 * math.cos(math.pi * i / n), yy(t0) + sgn * cap0 * math.sin(math.pi * i / n)) for i in range(1, n)]
        self.poly(view, id, bone, left + bot + right + top, follow=follow, **kw)

    # -- writing -------------------------------------------------------------------------------------------------------------
    def commit(self, scale, text_en, text_de):
        rep = self.reps[str(scale)]
        g = rep["geometry"]
        rep["geometry"] = {"elevation": g["elevation"], "front": self.front, "plan": self.plan}
        d = rep["decisions"]
        for k in ("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"):
            if k in text_en:
                d[k]["en"] = re.split(r" Front view and plan:", d[k]["en"])[0] + " Front view and plan: " + text_en[k]
                d[k]["de"] = re.split(r" Vorder- und Draufsicht:", d[k]["de"])[0] + " Vorder- und Draufsicht: " + text_de[k]

    def save(self):
        self.a.get("presentation", {}).pop("viewpoints", None)
        tmp = self.path + ".h1tmp"
        with open(tmp, "w", encoding="utf-8", newline="\n") as f:
            f.write(json.dumps(self.doc, ensure_ascii=False, indent=2) + "\n")
        os.replace(tmp, self.path)
