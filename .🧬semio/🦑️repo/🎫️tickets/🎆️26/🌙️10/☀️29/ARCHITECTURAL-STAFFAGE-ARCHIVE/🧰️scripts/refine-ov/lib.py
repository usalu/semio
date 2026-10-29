import json, math, re
R = "/home/deck/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
FILES = {"objects": f"{S}/🗂️catalog/🪑️objects/🔣️.json", "components": f"{S}/🗂️catalog/🧱️components/🔣️.json", "vegetation": f"{S}/🗂️catalog/🌳️vegetation/🔣️.json", "vehicles": f"{S}/🗂️catalog/🚜️vehicles/🔣️.json"}

def depth(v):
    if isinstance(v, dict): return 1 + max([depth(x) for x in v.values()] + [0])
    if isinstance(v, list): return 1 + max([depth(x) for x in v] + [0])
    return 0

def inline(v):
    if isinstance(v, dict): return "{ " + ", ".join(f'{json.dumps(k, ensure_ascii=False)}: {inline(x)}' for k, x in v.items()) + " }" if v else "{}"
    if isinstance(v, list): return "[" + ", ".join(inline(x) for x in v) + "]"
    if isinstance(v, float): return json.dumps(float(f'{v:.10g}'))
    return json.dumps(v, ensure_ascii=False)

def dump(v, ind=0):
    pad = "  " * ind
    flat = inline(v)
    if depth(v) <= 2 and len(flat) + ind * 2 <= 260 or not isinstance(v, (dict, list)) or not v: return flat
    if isinstance(v, dict):
        return "{\n" + ",\n".join(f'{pad}  {json.dumps(k, ensure_ascii=False)}: {dump(x, ind + 1)}' for k, x in v.items()) + f"\n{pad}}}"
    return "[\n" + ",\n".join(f"{pad}  {dump(x, ind + 1)}" for x in v) + f"\n{pad}]"

def load(cat):
    return json.load(open(FILES[cat]))

def save(cat, doc):
    open(FILES[cat], "w").write(dump(doc) + "\n")

def asset(doc, id):
    return next(a for a in doc["assets"] if a["id"] == id)

def upsert(lst, item, key="id"):
    for i, x in enumerate(lst):
        if x[key] == item[key]:
            lst[i] = item; return
    lst.append(item)

def T(en, de): return {"en": en, "de": de}

class G:
    """shape builder bound to default region/material"""
    def __init__(self): self.items = []
    def add(self, id, bone, role, lod, z, geom, region=None, material=None, filled=True, tone=0.5):
        s = {"id": id, "bone": bone, "role": role, "lod": lod, "z": z, "filled": filled, "tone": tone}
        s.update(geom)
        if region: s["region"] = region
        if material: s["material"] = material
        self.items.append(s); return s

def rect(x, y, w, h, r=None):
    d = {"kind": "rect", "x": x, "y": y, "w": w, "h": h}
    if r: d["r"] = r
    return d
def cap(x0, y0, x1, y1, r0, r1=None): return {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": r0, "r1": r0 if r1 is None else r1}
def ell(cx, cy, rx, ry, rot=None):
    d = {"kind": "ellipse", "cx": cx, "cy": cy, "rx": rx, "ry": ry}
    if rot: d["rotationDeg"] = rot
    return d
def poly(pts, closed=True): return {"kind": "poly", "points": [[round(a, 5), round(b, 5)] for a, b in pts], "closed": closed}
def path(d): return {"kind": "path", "d": d}

def pts_path(pts, close=True):
    s = "M " + " L ".join(f"{a:.4f} {b:.4f}" for a, b in pts)
    return path(s + (" Z" if close else ""))
