"""Edit helpers of the vehicles silhouette repairs on top of mvlib: drawn arrays are expanded one number per line in the catalog files, volume arrays are compact."""
import sys, json
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
class V(D):
    def indent(s, node):
        i = s.t.rfind("\n", 0, node["s"]) + 1
        j = i
        while s.t[j] == " ": j += 1
        return s.t[i:j]
    def drawn_array(s, node, key, value):
        n = s.field(node, key)
        s.edits.append((n["s"], n["e"], json.dumps(value, ensure_ascii=False, indent=1).replace("\n", "\n" + s.indent(n))))
    def replace_shape(s, node, obj_):
        n = node
        s.edits.append((n["s"], n["e"], json.dumps(obj_, ensure_ascii=False, indent=1).replace("\n", "\n" + s.indent(n))))
    def insert_shape_after(s, node, obj_):
        i = s.t.rfind("\n", 0, node["s"]) + 1
        ind = s.indent(node)
        s.edits.append((node["e"], node["e"], ",\n" + ind + json.dumps(obj_, ensure_ascii=False, indent=1).replace("\n", "\n" + ind)))
    def add_rects(s, scale, view, after_id, base_id, rects, role=None):
        """Inserts rect shapes after one shape, copying z, tone, region, material of a base shape: rects are (id, x, y, w, h)."""
        a = s.shape(scale, view, after_id)
        base = s.obj(s.shape(scale, view, base_id))
        keep = {k: base[k] for k in ("bone", "role", "z", "tone", "region", "material") if k in base}
        if role: keep["role"] = role
        txt = ""
        ind = s.indent(a)
        for rid, x, y, w, h in rects:
            o = {"kind": "rect", "x": x, "y": y, "w": w, "h": h, "id": rid, "filled": True, **keep}
            txt += ",\n" + ind + json.dumps(o, ensure_ascii=False, indent=1).replace("\n", "\n" + ind)
        s.edits.append((a["e"], a["e"], txt))
    def add_shapes(s, scale, view, after_id, base_id, shapes, role=None):
        """Inserts shapes (dicts with kind, id and geometry) after one shape, copying bone, role, z, tone, region, material of a base shape."""
        a = s.shape(scale, view, after_id)
        base = s.obj(s.shape(scale, view, base_id))
        keep = {k: base[k] for k in ("bone", "role", "z", "tone", "region", "material") if k in base}
        if role: keep["role"] = role
        ind = s.indent(a)
        txt = ""
        for o in shapes:
            o = {**o, "filled": o.get("filled", True), **{k: v for k, v in keep.items() if k not in o}}
            txt += ",\n" + ind + json.dumps(o, ensure_ascii=False, indent=1).replace("\n", "\n" + ind)
        s.edits.append((a["e"], a["e"], txt))
    def to_poly(s, scale, view, sid, ids, tol="0.003", xf=None):
        """Replaces one drawn shape by a polygon traced from the outline of the volume solids matching ids."""
        import subprocess
        name = s.slug.split("/")[-1]
        out = subprocess.run(["./vehicles-run.sh", "outline", name, str(scale), view, ids, tol], capture_output=True, text=True).stdout
        pts = json.loads(out.strip().splitlines()[-1])
        if xf:
            import math
            ox, oy, deg = xf; a = math.radians(deg)
            pts = [[round((x - ox) * math.cos(a) + (y - oy) * math.sin(a), 4), round(-(x - ox) * math.sin(a) + (y - oy) * math.cos(a), 4)] for x, y in pts]
        node = s.shape(scale, view, sid)
        o = s.obj(node)
        keep = {k: v for k, v in o.items() if k not in ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "d", "x0", "y0", "x1", "y1", "r0", "r1", "points", "closed", "rotationDeg")}
        s.replace_shape(node, {"kind": "poly", "points": pts, "closed": True, **keep})
    def ext(s, scale):
        import subprocess
        out = subprocess.run(["./vehicles-run.sh", "extj", s.slug.split("/")[-1], str(scale)], capture_output=True, text=True).stdout
        return json.loads(out.strip().splitlines()[-1])
    def rects_from_volume(s, scale, view, after_id, base_id, ids, role=None, pad=0.0):
        """Inserts one rect per matching volume solid, spanning its world extent in the view, as shapes named like the solid."""
        ex = s.ext(scale)
        have = {json.loads(s.t[v["s"]:v["e"]]) for it in s.at("asset", "representations", str(scale), "geometry", view)["items"] for k, v in it["pairs"] if k == "id"}
        rects = []
        for sid, (lo, hi) in ex.items():
            if not any(sid == i or (i.endswith("*") and sid.startswith(i[:-1])) for i in ids) or sid in have: continue
            (a0, a1), (b0, b1) = ((lo[0], hi[0]), (lo[1], hi[1])) if view == "elevation" else ((lo[2], hi[2]), (lo[1], hi[1])) if view == "front" else ((lo[0], hi[0]), (lo[2], hi[2]))
            rects.append((sid, round(a0 - pad, 4), round(b0 - pad, 4), round(a1 - a0 + 2 * pad, 4), round(b1 - b0 + 2 * pad, 4)))
        if rects: s.add_rects(scale, view, after_id, base_id, rects, role=role)
        return [r[0] for r in rects]
    def add_poly(s, scale, view, after_id, base_id, sid, ids, tol="0.003", role=None):
        """Inserts a polygon shape traced from the outline of the volume solids matching ids."""
        import subprocess
        name = s.slug.split("/")[-1]
        out = subprocess.run(["./vehicles-run.sh", "outline", name, str(scale), view, ids, tol], capture_output=True, text=True).stdout
        pts = json.loads(out.strip().splitlines()[-1])
        s.add_shapes(scale, view, after_id, base_id, [{"kind": "poly", "points": pts, "closed": True, "id": sid}], role=role)
    def add_missing(s, scale, view, bone_ref, tol, skip=(), xf=None):
        """Adds one traced polygon per volume solid that has no drawn twin of the same name in the view; the new shapes sit on the identity bone `bone_ref` after the last shape."""
        import subprocess
        name = s.slug.split("/")[-1]
        items = s.at("asset", "representations", str(scale), "geometry", view)["items"]
        drawn = [json.loads(s.t[v["s"]:v["e"]]) for it in items for k, v in it["pairs"] if k == "id"]
        def base(i):
            import re
            return re.sub(r"(-[a-z0-9]{1,2})+$", "", i) if False else i
        def has(i):
            b = i[:-2] if i[-2:] in ("-l", "-r", "-a", "-b") else i
            return any(d == i or d == b or d.startswith(b) or b.startswith(d) for d in drawn)
        vol = [json.loads(s.t[v["s"]:v["e"]]) for it in s.at("asset", "volumes", str(scale), "body")["items"] for k, v in it["pairs"] if k == "id"]
        todo = [i for i in vol if not has(i) and i not in skip]
        anchor = items[-1]
        ref = {}
        for it in items:
            o = json.loads(s.t[it["s"]:it["e"]])
            if o.get("bone") == bone_ref: ref = o; break
        ind = s.indent(anchor)
        txt = ""; added = []
        for i in todo:
            out = subprocess.run(["./vehicles-run.sh", "outline", name, str(scale), view, "=" + i, tol], capture_output=True, text=True).stdout
            try: pts = json.loads(out.strip().splitlines()[-1])
            except Exception: continue
            if len(pts) < 3: continue
            if xf is not None and view == "elevation":
                import math
                ox, oy, deg = xf; a = math.radians(deg)
                pts = [[round((x - ox) * math.cos(a) + (y - oy) * math.sin(a), 4), round(-(x - ox) * math.sin(a) + (y - oy) * math.cos(a), 4)] for x, y in pts]
            o = {"kind": "poly", "points": pts, "closed": True, "id": i, "bone": bone_ref, "role": "body", "z": ref.get("z", 1), "filled": True, "tone": ref.get("tone", 0.5)}
            for k in ("region", "material"):
                if k in ref: o[k] = ref[k]
            txt += ",\n" + ind + json.dumps(o, ensure_ascii=False, indent=1).replace("\n", "\n" + ind); added.append(i)
        if txt: s.edits.append((anchor["e"], anchor["e"], txt))
        return added
    def to_world_poly(s, scale, view, sid, ids, tol="0.004", bone="frame"):
        """Replaces one drawn shape (also one with an extruded solid on a rotated bone) by a world-metre polygon on the identity bone, traced from volume solids."""
        import subprocess
        name = s.slug.split("/")[-1]
        out = subprocess.run(["./vehicles-run.sh", "outline", name, str(scale), view, ids, tol], capture_output=True, text=True).stdout
        pts = json.loads(out.strip().splitlines()[-1])
        node = s.shape(scale, view, sid)
        o = s.obj(node)
        keep = {k: v for k, v in o.items() if k not in ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "d", "x0", "y0", "x1", "y1", "r0", "r1", "points", "closed", "rotationDeg", "solid", "bone", "from", "to", "section", "penMm")}
        s.replace_shape(node, {"kind": "poly", "points": pts, "closed": True, **keep, "bone": bone, "filled": True})
    def rename_to_world_poly(s, scale, view, sid, new_id, ids, tol="0.004"):
        """Like to_world_poly but the shape also gets a new id (a shape per slender member)."""
        import subprocess
        name = s.slug.split("/")[-1]
        out = subprocess.run(["./vehicles-run.sh", "outline", name, str(scale), view, ids, tol], capture_output=True, text=True).stdout
        pts = json.loads(out.strip().splitlines()[-1])
        node = s.shape(scale, view, sid)
        o = s.obj(node)
        keep = {k: v for k, v in o.items() if k not in ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "d", "x0", "y0", "x1", "y1", "r0", "r1", "points", "closed", "rotationDeg", "solid", "bone", "from", "to", "section", "penMm", "id")}
        s.replace_shape(node, {"kind": "poly", "points": pts, "closed": True, "id": new_id, **keep, "bone": "frame", "filled": True})
    def insert_world_poly_after(s, scale, view, after_id, new_id, ids, tol="0.004"):
        import subprocess
        name = s.slug.split("/")[-1]
        out = subprocess.run(["./vehicles-run.sh", "outline", name, str(scale), view, ids, tol], capture_output=True, text=True).stdout
        pts = json.loads(out.strip().splitlines()[-1])
        s.add_shapes(scale, view, after_id, after_id, [{"kind": "poly", "points": pts, "closed": True, "id": new_id, "bone": "frame"}])
    def fmt_vol(s, obj, ind):
        """Formats a volume solid like the catalog: objects expanded, arrays of numbers and objects of numbers on one line."""
        import re
        txt = json.dumps(obj, ensure_ascii=False, indent=2)
        txt = re.sub(r"\[\s*(-?[0-9.eE+\-]+(?:,\s*-?[0-9.eE+\-]+)*)\s*\]", lambda m: "[" + re.sub(r",\s*", ", ", m.group(1)) + "]", txt)
        return txt.replace("\n", "\n" + ind)
    def insert_solid_after(s, scale, after_id, obj):
        node = s.solid(scale, after_id)
        ind = s.indent(node)
        s.edits.append((node["e"], node["e"], ",\n" + ind + s.fmt_vol(obj, ind)))
    def add_key_after(s, node, after_key, key, value):
        """Inserts one member after another member of a drawn shape, formatted like the catalog (one number per line)."""
        n = s.field(node, after_key)
        ind = s.indent(n)
        txt = json.dumps({key: value}, ensure_ascii=False, indent=1)
        body = txt[2:-2].replace("\n", "\n" + ind)
        s.edits.append((n["e"], n["e"], ",\n" + ind + body.strip()))
    def outline_pts(s, scale, view, ids, tol):
        import subprocess
        name = s.slug.split("/")[-1]
        out = subprocess.run(["./vehicles-run.sh", "outline", name, str(scale), view, ids, tol], capture_output=True, text=True).stdout
        try: return json.loads(out.strip().splitlines()[-1])
        except Exception: return None
    def retrace_each(s, scale, view, sid, vol_ids, tol, xf=None, world=False):
        """Replaces one drawn shape by the polygon of the first volume solid (same id, bone kept when xf is given, else the identity bone and world metres) and inserts the polygons of the other solids after it."""
        import math
        node = s.shape(scale, view, sid)
        o = s.obj(node)
        keep = {k: v for k, v in o.items() if k not in ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "d", "x0", "y0", "x1", "y1", "r0", "r1", "points", "closed", "rotationDeg", "solid", "from", "to", "section", "penMm", "id", "bone", "filled")}
        def conv(pts):
            if xf is None: return pts
            ox, oy, deg = xf; a = math.radians(deg)
            return [[round((x - ox) * math.cos(a) + (y - oy) * math.sin(a), 4), round(-(x - ox) * math.sin(a) + (y - oy) * math.cos(a), 4)] for x, y in pts]
        bone = o.get("bone", "frame") if xf is not None else "frame"
        made = []
        for k, vid in enumerate(vol_ids):
            pts = s.outline_pts(scale, view, "=" + vid, tol)
            if not pts or len(pts) < 3: continue
            made.append((sid if not made else vid, conv(pts)))
        if not made:
            return []
        first_id, first_pts = made[0]
        s.replace_shape(node, {"kind": "poly", "points": first_pts, "closed": True, "id": first_id, **keep, "bone": bone, "filled": True})
        if len(made) > 1:
            ind = s.indent(node)
            txt = ""
            items = s.at("asset", "representations", str(scale), "geometry", view)["items"]
            have = {}
            for it in items:
                oo = json.loads(s.t[it["s"]:it["e"]])
                if oo.get("bone") == bone and oo.get("kind") == "poly" and "solid" not in oo: have[oo["id"]] = it
            for vid, pts in made[1:]:
                n = {"kind": "poly", "points": pts, "closed": True, "id": vid, **keep, "bone": bone, "filled": True}
                if vid in have and have[vid]["s"] != node["s"]:
                    s.replace_shape(have[vid], n)
                    continue
                txt += ",\n" + ind + json.dumps(n, ensure_ascii=False, indent=1).replace("\n", "\n" + ind)
            if txt: s.edits.append((node["e"], node["e"], txt))
        return [m_[0] for m_ in made]
