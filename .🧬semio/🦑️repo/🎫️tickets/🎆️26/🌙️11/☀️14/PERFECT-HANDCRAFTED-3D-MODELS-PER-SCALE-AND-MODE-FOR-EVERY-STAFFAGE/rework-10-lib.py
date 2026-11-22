"""🧰 Chisel helpers of author 10: read and write a volumes source, replace or drop solids by id while keeping the thermal and structure `{ of }` references consistent, and a few hand-geometry helpers (thick open path, belt loop around two wheels).

Use: exec(open('rework-10-lib.py').read()) at the top of a rework-10-<slug>-*.py script.
"""
import json, math, glob


class Source:
    """📦 One `T/volumes/<category>/<slug>.json` opened for editing."""

    def __init__(self, slug, category="machinery"):
        self.path = f"volumes/{category}/{slug}.json"
        self.doc = json.load(open(self.path))
        self.v = self.doc["volumes"]

    def ids(self, scale, lst="body"):
        return [s["id"] for s in self.v[str(scale)][lst]]

    def get(self, scale, sid, lst="body"):
        return next(s for s in self.v[str(scale)][lst] if s["id"] == sid)

    def drop(self, scale, ids, lists=("body", "thermal", "structure")):
        vol = self.v[str(scale)]
        ids = set(ids)
        for lst in lists:
            vol[lst] = [s for s in vol[lst] if s["id"] not in ids and not (lst != "body" and s.get("of") in ids)]

    def put(self, scale, solids, region=None, lst="body", layer=None):
        vol = self.v[str(scale)]
        for solid in solids:
            at = next((i for i, s in enumerate(vol[lst]) if s["id"] == solid["id"]), None)
            if at is None:
                vol[lst].append(solid)
            else:
                vol[lst][at] = solid
            if lst == "body" and region is not None and not any(t.get("of") == solid["id"] for t in vol["thermal"]):
                ref = {"id": "th-" + solid["id"], "of": solid["id"], "region": region}
                if layer is not None:
                    ref["layer"] = layer
                vol["thermal"].append(ref)

    def decide(self, scale, en, de):
        self.v[str(scale)]["decisions"].append({"en": en, "de": de})

    def save(self):
        json.dump(self.doc, open(self.path, "w"), ensure_ascii=False, indent=1)


def r4(p):
    return [[round(x, 4), round(y, 4)] for x, y in p]


def arc(c, r, a0, a1, n):
    """⭕ Points on a circle from angle a0 to a1 (degrees), n segments."""
    return [[c[0] + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), c[1] + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))] for i in range(n + 1)]


def tangent_angle(p, c, r, side):
    """📐 Angle (degrees) on circle (c, r) of the tangent point seen from the outer point p; side +1 = the tangent point counter-clockwise of the line p to c."""
    d = math.hypot(c[0] - p[0], c[1] - p[1])
    base = math.degrees(math.atan2(p[1] - c[1], p[0] - c[0]))
    return base + side * math.degrees(math.acos(r / d))


def belt(c_rear, c_front, r, y0, x_run, n=10):
    """🔗 Closed track loop: flat bottom run at y0 between -x_run and +x_run, ramps tangent to the front and rear wheel circles of radius r, straight top run."""
    top = math.degrees(math.atan2(c_front[1] - c_rear[1], c_front[0] - c_rear[0])) + 90
    a_f = tangent_angle([x_run, y0], c_front, r, +1)
    a_r = tangent_angle([-x_run, y0], c_rear, r, -1)
    return r4([[-x_run, y0], [x_run, y0]] + arc(c_front, r, a_f, top, n) + arc(c_rear, r, top, a_r + 360 if a_r < top else a_r, n))


def thick(path, t, side=1):
    """🧱 Closed polygon of an open path given constant thickness t to its left (side +1) or right (-1), mitred at the corners."""
    def normal(a, b):
        l = math.hypot(b[0] - a[0], b[1] - a[1])
        return [-(b[1] - a[1]) / l * side, (b[0] - a[0]) / l * side]
    inner = []
    for i, p in enumerate(path):
        ns = [normal(path[j], path[j + 1]) for j in (i - 1, i) if 0 <= j < len(path) - 1]
        m = [sum(n[0] for n in ns), sum(n[1] for n in ns)]
        l = math.hypot(*m)
        m = [m[0] / l, m[1] / l]
        k = t / (m[0] * ns[0][0] + m[1] * ns[0][1])
        inner.append([p[0] + m[0] * k, p[1] + m[1] * k])
    return r4(list(path) + inner[::-1])


def cyl(sid, bone, material, origin, axis, r, length, tone=0.4, role="detail", **more):
    """🛢️ Plain turned cylinder as a revolve."""
    return {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[0, 0], [0, r], [length, r], [length, 0]], "axis": axis, "origin": origin, **more}


def box(sid, bone, material, size, at, tone=0.4, role="detail", **more):
    """📦 Box centred at `at`."""
    return {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": at}, **more}


def ext(sid, bone, material, profile, plane, lo, hi, tone=0.4, role="body", **more):
    """🧇 Prismatic part."""
    return {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": r4(profile), "plane": plane, "from": lo, "to": hi, **more}


def tube(sid, bone, material, points, r, tone=0.3, role="detail", **more):
    """🪈 Bent round tube through points."""
    return {"id": sid, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": [{"at": p, "rx": r, "ry": r} for p in points], "capStart": True, "capEnd": True, **more}


class Catalog:
    """🗂️ One catalog asset file opened for a root fix: members are replaced as text (one-space indented JSON, as the files are written), everything else stays byte-identical; `write(candidate=True)` writes the candidate document for `r3-validate.ts --file` instead."""

    def __init__(self, slug):
        self.slug = slug
        self.path = glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{slug}/*.json")[0]
        self.text = open(self.path, encoding="utf-8").read()
        self.asset = json.loads(self.text)["asset"]
        self.touched = set()

    def touch(self, *keys):
        self.touched.update(keys)

    def _range(self, text, key):
        import re
        found = re.search(r'\n  "' + key + r'"\s*:\s*', text)
        depth, index, quoted = 0, found.end(), False
        while True:
            ch = text[index]
            if quoted:
                if ch == "\\":
                    index += 1
                elif ch == '"':
                    quoted = False
            elif ch == '"':
                quoted = True
                if depth == 0:
                    pass
            elif ch in "{[":
                depth += 1
            elif ch in "}]":
                depth -= 1
                if depth == 0:
                    return found.end(), index + 1
            elif depth == 0 and ch in ",\n":
                return found.end(), index
            index += 1

    def render(self):
        text = self.text
        for key in self.touched:
            start, end = self._range(text, key)
            body = json.dumps(self.asset[key], ensure_ascii=False, indent=1).replace("\n", "\n  ")
            text = text[:start] + body + text[end:]
        check = json.loads(text)["asset"]
        assert check == self.asset, "render did not round-trip"
        return text

    def write(self, candidate=False):
        import os
        text = self.render()
        if candidate:
            folder = f"🗑️generated/r3-{self.slug}"
            os.makedirs(folder, exist_ok=True)
            open(f"{folder}/🔣️.json", "w", encoding="utf-8").write(text)
            return f"{folder}/🔣️.json"
        assert open(self.path, encoding="utf-8").read() == self.text, "catalog file changed under me; rerun"
        open(self.path, "w", encoding="utf-8").write(text)
        return self.path
