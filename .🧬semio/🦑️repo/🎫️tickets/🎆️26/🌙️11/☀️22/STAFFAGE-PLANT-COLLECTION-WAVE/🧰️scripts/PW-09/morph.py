"""🌿 Species morph of a shipped semio.staffage/v2 vegetation asset (PW-09).

The shipped catalog documents are the current source of the existing plants: their generators in the archive ticket
(`ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V1`, `vegetation`) predate the per-scale volumes and multiview rework, so a new plant is
derived from the closest shipped asset by an exact per-bone affine morph and then re-authored (species data, texts, materials, poses).

Every bone b gets a local scale S_b = diag(su, sr, sr) (u along the bone, v and w across it) about its own origin; a child keeps
its attachment point on the scaled parent, so the new rest frame of b is o_b' = o_p' + R_p S_p (x, y, z) with unchanged rotations.
Bone-local geometry (elevation shapes, solids, anchors, loads) is multiplied by S_b; world-space geometry (front and plan shapes,
true-extent symbols) is carried by W_b(p) = o_b' + R_b S_b R_b^T (p - o_b), which is exact for su = sr and for bones along a world
axis, and takes the missing depth of a 2D view from the bone axis otherwise.

@see ../../📋️brief.md
@see ../../../../../🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/authoring-vegetation.md
"""
import json
import math
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[9]
STAFFAGE = ROOT / "🎓️teaching" / "🏛️architecture" / "🧍️staffage"
VEGETATION = STAFFAGE / "🗂️catalog" / "🌳️vegetation"


def load(asset_id):
    return json.loads((VEGETATION / asset_id / "🔣️.json").read_text(encoding="utf-8"))


def _inline(x):
    return isinstance(x, list) and all(isinstance(e, (int, float)) or (isinstance(e, list) and all(isinstance(f, (int, float)) for f in e)) for e in x)


def dumps(x, ind=0):
    pad, sub = "  " * ind, "  " * (ind + 1)
    if isinstance(x, dict):
        if not x:
            return "{}"
        return "{\n" + ",\n".join(f"{sub}{json.dumps(k, ensure_ascii=False)}: {dumps(v, ind + 1)}" for k, v in x.items()) + f"\n{pad}}}"
    if isinstance(x, list):
        if not x:
            return "[]"
        if _inline(x):
            return json.dumps(x, ensure_ascii=False, separators=(", ", ": "))
        return "[\n" + ",\n".join(f"{sub}{dumps(v, ind + 1)}" for v in x) + f"\n{pad}]"
    return json.dumps(x, ensure_ascii=False)


def save(doc, asset_id):
    folder = VEGETATION / asset_id
    folder.mkdir(parents=True, exist_ok=True)
    (folder / "🔣️.json").write_text(dumps(doc) + "\n", encoding="utf-8")
    return folder / "🔣️.json"


def r(x, n=5):
    v = round(x, n)
    return 0 if v == 0 else v


def mat_mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mat_vec(a, v):
    return [sum(a[i][k] * v[k] for k in range(3)) for i in range(3)]


def transpose(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def rot_w(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rot_v(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def rot_u(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def rest_rotation(bone):
    return mat_mul(mat_mul(rot_w(bone["angleDeg"]), rot_v(bone.get("abductionDeg", 0))), rot_u(bone.get("twistDeg", 0)))


def quat_matrix(q):
    x, y, z, w = q
    return [[1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)],
            [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
            [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)]]


def frames(bones, scale=None, shift=None):
    """🦴 Rest frames {id: (o, R)} of a rig; `scale` {id: (su, sr)} scales the attachment offsets by the parent, `shift` {id: [dx, dy, dz]} moves a bone in its parent frame."""
    scale, shift = scale or {}, shift or {}
    by = {b["id"]: b for b in bones}
    out = {}

    def solve(i):
        if i in out:
            return out[i]
        b = by[i]
        local = [b["x"], b["y"], b.get("z", 0)]
        if b["parent"] is None:
            o, R = [local[0], local[1], local[2]], rest_rotation(b)
        else:
            po, pR = solve(b["parent"])
            su, sv, sw = scale.get(b["parent"], (1, 1, 1))
            d = [local[0] * su + shift.get(i, [0, 0, 0])[0], local[1] * sv + shift.get(i, [0, 0, 0])[1], local[2] * sw + shift.get(i, [0, 0, 0])[2]]
            off = mat_vec(pR, d)
            o, R = [po[k] + off[k] for k in range(3)], mat_mul(pR, rest_rotation(b))
        out[i] = (o, R)
        return out[i]

    for b in bones:
        solve(b["id"])
    return out


NUM = re.compile(r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?|[A-Za-z]")


def map_path(d, f):
    """〰️ Maps every coordinate pair of an absolute M/L/C/Q/Z path by f."""
    tokens = NUM.findall(d)
    out, nums = [], []
    for t in tokens:
        if t.isalpha():
            if t.upper() not in "MLCQZ" or t != t.upper():
                raise ValueError(f"unsupported path command {t}")
            out.append(t)
        else:
            nums.append(float(t))
            if len(nums) == 2:
                x, y = f(nums[0], nums[1])
                out.append(f"{r(x, 4):g} {r(y, 4):g}")
                nums = []
    if nums:
        raise ValueError("odd path coordinates")
    return " ".join(out)


class Morph:
    """🧬 The per-bone affine morph of one asset document (in place). Crown geometry (`is_blended`: foliage, fruit, crown outlines) follows one
    world-diagonal affine map about the origin of the `crown` bone, `p' = o' + A (p - o)`, so a crown mass lands at the same place whichever bone
    a scale binds it to and in whichever view it is drawn (a diagonal map needs no depth)."""

    def __init__(self, asset, k=1.0, bones=None, shift=None, density=None, crown=None, is_blended=None):
        self.asset = asset
        self.k = k
        spec = lambda f: (f[0], f[1], f[1]) if len(f) == 2 else tuple(f)
        self.factors = {b["id"]: spec((bones or {}).get(b["id"], (k, k))) for b in asset["rig"]["bones"]}
        self.shift = shift or {}
        self.density = density or {}
        self.old = frames(asset["rig"]["bones"])
        self.new = frames(asset["rig"]["bones"], self.factors, self.shift)
        self.length = {b["id"]: b["lengthM"] for b in asset["rig"]["bones"]}
        self.crown = crown or {"bone": asset["rig"]["root"], "A": [k, k, k]}
        self.blend = list(self.crown.get("bones", []))
        self.is_blended = is_blended or (lambda item: False)
        self.segments = {}
        for b in asset["rig"]["bones"]:
            o, R = self.old[b["id"]]
            self.segments[b["id"]] = (o, [o[i] + R[i][0] * self.length[b["id"]] for i in range(3)])

    def S(self, bone):
        return list(self.factors[bone])

    def world_matrix(self, bone):
        _, R = self.old[bone]
        s = self.S(bone)
        D = [[s[0], 0, 0], [0, s[1], 0], [0, 0, s[2]]]
        return mat_mul(mat_mul(R, D), transpose(R))

    def world(self, bone, p):
        o, _ = self.old[bone]
        o2, _ = self.new[bone]
        q = mat_vec(self.world_matrix(bone), [p[i] - o[i] for i in range(3)])
        return [o2[i] + q[i] for i in range(3)]

    def distance(self, bone, p):
        a, c = self.segments[bone]
        d = [c[i] - a[i] for i in range(3)]
        L2 = sum(x * x for x in d)
        t = 0 if L2 == 0 else max(0, min(1, sum((p[i] - a[i]) * d[i] for i in range(3)) / L2))
        return math.sqrt(sum((p[i] - a[i] - t * d[i]) ** 2 for i in range(3)))

    def wmap(self, bone, p, blended):
        if not blended:
            return self.world(bone, p)
        o, o2, A = self.old[self.crown["bone"]][0], self.new[self.crown["bone"]][0], self.crown["A"]
        return [o2[i] + A[i] * (p[i] - o[i]) for i in range(3)]

    def kloc(self, p=None):
        A = self.crown["A"]
        return (A[0] * A[1] * A[2]) ** (1 / 3)

    def kview(self, view):
        A = self.crown["A"]
        return (math.sqrt(A[0] * A[1]),) * 2 if view == "elevation" else (A[2], A[1]) if view == "front" else (A[0], A[2])

    def to_world(self, bone, q):
        o, R = self.old[bone]
        v = mat_vec(R, q)
        return [o[i] + v[i] for i in range(3)]

    def to_local(self, bone, p):
        o, R = self.new[bone]
        return mat_vec(transpose(R), [p[i] - o[i] for i in range(3)])

    def local_map(self, bone, q, blended):
        """📌 A bone-local point (u, v, w) to its new bone-local place."""
        if not blended:
            return [q[i] * self.S(bone)[i] for i in range(3)]
        return self.to_local(bone, self.wmap(bone, self.to_world(bone, q), True))

    def depth(self, bone, view, a, b):
        """🧭 The missing world coordinate of a 2D view point, from the bone axis."""
        o, R = self.old[bone]
        u = [R[0][0], R[1][0], R[2][0]]
        L = self.length[bone]
        if view == "front":
            t = (b - o[1]) / u[1] if abs(u[1]) > 0.3 else L / 2
            return o[0] + max(0, min(L, t)) * u[0]
        t = (a - o[0]) / u[0] if abs(u[0]) > 0.3 else L / 2
        return o[1] + max(0, min(L, t)) * u[1]

    def view_point(self, bone, view, a, b, blended=False):
        if view == "front":
            q = self.wmap(bone, [self.depth(bone, view, a, b), b, a], blended)
            return q[2], q[1]
        q = self.wmap(bone, [a, self.depth(bone, view, a, b), b], blended)
        return q[0], q[2]

    def view_axes(self, bone, view):
        M = self.world_matrix(bone)
        if view == "front":
            return math.hypot(M[2][2], M[1][2]), math.hypot(M[1][1], M[2][1])
        return math.hypot(M[0][0], M[2][0]), math.hypot(M[2][2], M[0][2])

    def shape(self, s, view):
        bone = s["bone"]
        blended = self.is_blended(s)
        if view == "elevation":
            if blended:
                def f(x, y):
                    q = self.local_map(bone, [x, y, 0], True)
                    return q[0], q[1]
            else:
                su, sr, _ = self.S(bone)
                f = lambda x, y: (x * su, y * sr)
                ax, ay = su, sr
        else:
            f = lambda x, y: self.view_point(bone, view, x, y, blended)
            if not blended:
                ax, ay = self.view_axes(bone, view)
        kind = s["kind"]
        if blended and kind in ("ellipse", "capsule"):
            c = (s["cx"], s["cy"]) if kind == "ellipse" else ((s["x0"] + s["x1"]) / 2, (s["y0"] + s["y1"]) / 2)
            if view == "elevation":
                p = self.to_world(bone, [c[0], c[1], 0])
            elif view == "front":
                p = [self.depth(bone, view, *c), c[1], c[0]]
            else:
                p = [c[0], self.depth(bone, view, *c), c[1]]
            ax, ay = self.kview(view)
        if kind == "path":
            s["d"] = map_path(s["d"], f)
        elif kind == "poly":
            s["points"] = [[r(c, 4) for c in f(*p)] for p in s["points"]]
        elif kind == "ellipse":
            s["cx"], s["cy"] = [r(c, 4) for c in f(s["cx"], s["cy"])]
            s["rx"], s["ry"] = r(s["rx"] * ax, 4), r(s["ry"] * ay, 4)
        elif kind == "capsule":
            s["x0"], s["y0"] = [r(c, 4) for c in f(s["x0"], s["y0"])]
            s["x1"], s["y1"] = [r(c, 4) for c in f(s["x1"], s["y1"])]
            m = math.sqrt(ax * ay)
            s["r0"], s["r1"] = r(s["r0"] * m, 4), r(s["r1"] * m, 4)
        elif kind == "rect":
            x0, y0 = f(s["x"], s["y"])
            x1, y1 = f(s["x"] + s["w"], s["y"] + s["h"])
            s["x"], s["y"], s["w"], s["h"] = r(min(x0, x1), 4), r(min(y0, y1), 4), r(abs(x1 - x0), 4), r(abs(y1 - y0), 4)
        else:
            raise ValueError(f"shape kind {kind}")
        if "solid" in s:
            raise ValueError("solid shapes are not morphed")

    def solid(self, s):
        if "of" in s and "kind" not in s:
            return
        bone = s["bone"]
        blended = self.is_blended(s)
        S = self.S(bone)
        uniform = abs(S[0] - S[1]) < 1e-12
        tr = s.get("transform", {})
        Q = quat_matrix(tr["rotation"]) if "rotation" in tr else [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        D = [[S[0], 0, 0], [0, S[1], 0], [0, 0, S[2]]]
        A = mat_mul(mat_mul(transpose(Q), D), Q)
        sc = tr.get("scale")
        if sc is not None and not isinstance(sc, list):
            sc = [sc, sc, sc]
        t = tr.get("translation", [0, 0, 0])
        t2 = self.local_map(bone, t, blended)
        kt = self.kloc(self.to_world(bone, t)) if blended else None
        if "translation" in tr or (blended and any(abs(t2[i] - t[i]) > 1e-9 for i in range(3)) and s["kind"] in ("polyhedron", "ellipsoid", "box", "loft", "revolve", "capsule", "extrude")):
            tr["translation"] = [r(c, 6) for c in t2]
            s["transform"] = tr
        Qt = transpose(Q)
        kind = s["kind"]

        def inner(p):
            """A point of the solid's own frame to its new place in that frame."""
            q = mat_vec(Q, p)
            q = [q[i] + t[i] for i in range(3)]
            q2 = self.local_map(bone, q, True)
            return mat_vec(Qt, [q2[i] - t2[i] for i in range(3)])

        if kind == "polyhedron":
            pre = sc or [1, 1, 1]
            vs = [[v[i] * pre[i] for i in range(3)] for v in s["vertices"]]
            s["vertices"] = [[r(c, 6) for c in (inner(v) if blended else mat_vec(A, v))] for v in vs]
            tr.pop("scale", None)
        elif kind == "loft":
            for sec in s["sections"]:
                if blended:
                    kk = self.kloc(self.to_world(bone, mat_vec(Q, sec["at"])))
                    sec["at"] = [r(c, 6) for c in inner(sec["at"])]
                    sec["rx"], sec["ry"] = r(sec["rx"] * kk, 6), r(sec["ry"] * kk, 6)
                else:
                    sec["at"] = [r(sec["at"][i] * S[i], 6) for i in range(3)]
                    if "rotation" in sec and not uniform:
                        Rs = mat_mul(Q, quat_matrix(sec["rotation"]))
                        fx = math.sqrt(sum((Rs[i][0] * S[i]) ** 2 for i in range(3)))
                        fy = math.sqrt(sum((Rs[i][1] * S[i]) ** 2 for i in range(3)))
                    else:
                        fx = fy = S[0] if uniform else math.sqrt(S[1] * S[2])
                    sec["rx"], sec["ry"] = r(sec["rx"] * fx, 6), r(sec["ry"] * fy, 6)
        elif kind == "ellipsoid":
            if blended:
                s["radii"] = [r(c * kt, 6) for c in s["radii"]]
            else:
                s["radii"] = [r(s["radii"][i] * math.sqrt(sum(A[j][i] ** 2 for j in range(3))), 6) for i in range(3)]
        elif kind == "revolve":
            if blended:
                s["profile"] = [[r(p[0] * kt, 6), r(p[1] * kt, 6)] for p in s["profile"]]
                s["origin"] = [r(c, 6) for c in inner(s["origin"])]
            else:
                a = s["axis"]
                n = math.sqrt(sum(c * c for c in a))
                axial = math.sqrt(sum((a[i] * S[i]) ** 2 for i in range(3))) / n
                radial = S[1] if abs(a[0]) / n > 0.99 else math.sqrt(S[0] * S[1]) if not uniform else S[0]
                s["profile"] = [[r(p[0] * axial, 6), r(p[1] * radial, 6)] for p in s["profile"]]
                s["origin"] = [r(s["origin"][i] * S[i], 6) for i in range(3)]
        elif kind == "capsule":
            if blended:
                s["p0"], s["p1"] = [r(c, 6) for c in inner(s["p0"])], [r(c, 6) for c in inner(s["p1"])]
                s["r0"], s["r1"] = r(s["r0"] * kt, 6), r(s["r1"] * kt, 6)
            else:
                s["p0"] = [r(s["p0"][i] * S[i], 6) for i in range(3)]
                s["p1"] = [r(s["p1"][i] * S[i], 6) for i in range(3)]
                s["r0"], s["r1"] = r(s["r0"] * S[1], 6), r(s["r1"] * S[1], 6)
        elif kind == "box":
            if blended:
                s["size"] = [r(c * kt, 6) for c in s["size"]]
            else:
                s["size"] = [r(s["size"][i] * math.sqrt(sum(A[j][i] ** 2 for j in range(3))), 6) for i in range(3)]
            if "cornerRadius" in s:
                s["cornerRadius"] = r(s["cornerRadius"] * (kt if blended else S[1]), 6)
        elif kind == "extrude":
            if blended:
                raise ValueError("blended extrude")
            plane = {"xy": (0, 1, 2), "yz": (1, 2, 0), "zx": (2, 0, 1)}[s.get("plane", "xy")]
            s["profile"] = [[r(p[0] * S[plane[0]], 6), r(p[1] * S[plane[1]], 6)] for p in s["profile"]]
            if "holes" in s:
                s["holes"] = [[[r(p[0] * S[plane[0]], 6), r(p[1] * S[plane[1]], 6)] for p in h] for h in s["holes"]]
            s["from"], s["to"] = r(s["from"] * S[plane[2]], 6), r(s["to"] * S[plane[2]], 6)
        else:
            raise ValueError(f"solid kind {kind}")
        if "repeat" in s:
            rep = s["repeat"]
            if "origin" in rep:
                rep["origin"] = [r(c, 6) for c in (inner(rep["origin"]) if blended else [rep["origin"][i] * A[i][i] for i in range(3)])]
            if "translation" in rep:
                rep["translation"] = [r(rep["translation"][i] * (kt if blended else S[i]), 6) for i in range(3)]

    def symbol_shape(self, s, view):
        """📍 World-space symbol geometry: crown parts follow the foliage blend, stem parts the nearest woody bone."""
        blended = self.is_blended(s)
        wood = [b["id"] for b in self.asset["rig"]["bones"] if b["id"] not in self.blend]

        def nearest(p):
            return min(wood, key=lambda b: self.distance(b, p))

        def lift(x, y):
            if view == "elevation":
                return [x, y, 0]
            if view == "front":
                return [0, y, x]
            return [x, 0, y]

        def f(x, y):
            p = lift(x, y)
            q = self.wmap(None, p, True) if blended else self.world(nearest(p), p)
            return (q[0], q[1]) if view == "elevation" else (q[2], q[1]) if view == "front" else (q[0], q[2])

        kind = s["kind"]
        if kind == "path":
            s["d"] = map_path(s["d"], f)
        elif kind == "poly":
            s["points"] = [[r(c, 4) for c in f(*p)] for p in s["points"]]
        elif kind == "ellipse":
            p = lift(s["cx"], s["cy"])
            kx, ky = self.kview(view) if blended else (self.factors[nearest(p)][1],) * 2
            s["cx"], s["cy"] = [r(c, 4) for c in f(s["cx"], s["cy"])]
            s["rx"], s["ry"] = r(s["rx"] * kx, 4), r(s["ry"] * ky, 4)
        else:
            raise ValueError(kind)

    def apply(self):
        a = self.asset
        for b in a["rig"]["bones"]:
            su, sr, sw = self.factors[b["id"]]
            if b["parent"] is not None:
                psu, psv, psw = self.factors[b["parent"]]
                d = self.shift.get(b["id"], [0, 0, 0])
                b["x"], b["y"] = r(b["x"] * psu + d[0], 4), r(b["y"] * psv + d[1], 4)
                if "z" in b or d[2]:
                    b["z"] = r(b.get("z", 0) * psw + d[2], 4)
            b["lengthM"] = r(b["lengthM"] * su, 4)
            if "mass" in b:
                m = b["mass"]
                m["kg"] = r(m["kg"] * su * sr * sw * self.density.get(b["id"], 1), 2)
                m["comX"], m["comY"] = r(m["comX"] * su, 4), r(m["comY"] * sr, 4)
            if "section" in b:
                sec = b["section"]
                sec["areaM2"] = float(f"{sec['areaM2'] * sr * sw:.5g}")
                sec["secondMomentM4"] = float(f"{sec['secondMomentM4'] * sr ** 3 * sw:.4g}")
                sec["extremeFibreM"] = r(sec["extremeFibreM"] * sr, 4)
        for an in a["anchors"]:
            blended = self.is_blended(an)
            q = self.local_map(an["bone"], [an["x"], an["y"], an.get("z", 0)], blended)
            an["x"], an["y"] = r(q[0], 4), r(q[1], 4)
            if "z" in an:
                an["z"] = r(q[2], 4)
            su, sr, _ = self.factors[an["bone"]]
            for key in ("footprintLengthM", "footprintWidthM"):
                if key in an:
                    an[key] = r(an[key] * sr, 3)
        for rep in a["representations"].values():
            if rep.get("status") != "authored":
                continue
            for view, shapes in rep["geometry"].items():
                for s in shapes:
                    self.shape(s, view)
            if rep.get("sections"):
                raise ValueError("sections are not morphed")
        for sym in a["symbols"]:
            if sym["units"] != "model-m":
                continue
            for view, shapes in sym["geometry"].items():
                for s in shapes:
                    self.symbol_shape(s, view)
        for vol in a["volumes"].values():
            if vol.get("status") != "authored":
                continue
            for lst in ("body", "thermal", "structure"):
                for s in vol.get(lst, []):
                    self.solid(s)
        for p in a["poses"]:
            for ld in p.get("loads", []):
                self.load(ld)
        for e in a["thermal"].get("emitters", []):
            if "diameterM" in e:
                e["diameterM"] = r(e["diameterM"] * self.k, 3)

    def load(self, ld):
        if "bone" in ld and "x" in ld:
            q = self.local_map(ld["bone"], [ld["x"], ld.get("y", 0), ld.get("z", 0)], self.is_blended({"bone": ld["bone"]}))
            ld["x"] = r(q[0], 4)
            if "y" in ld:
                ld["y"] = r(q[1], 4)
            if "z" in ld:
                ld["z"] = r(q[2], 4)
        if "distributed" in ld:
            dd = ld["distributed"]
            su, sr, _ = self.factors[dd["bone"]]
            dd["fromM"], dd["toM"] = r(dd["fromM"] * su, 4), r(dd["toM"] * su, 4)


def texts(x, path=""):
    """📝 Every {en, de} text with its path."""
    if isinstance(x, dict):
        if set(x.keys()) == {"en", "de"} and all(isinstance(v, str) for v in x.values()):
            yield path, x
            return
        for k, v in x.items():
            yield from texts(v, f"{path}.{k}")
    elif isinstance(x, list):
        for i, v in enumerate(x):
            yield from texts(v, f"{path}[{v['id'] if isinstance(v, dict) and 'id' in v else i}]")


def replace_words(x, en, de):
    """🔤 Ordered word replacements in every text, per language."""
    for _, t in texts(x):
        for a, b in en:
            t["en"] = t["en"].replace(a, b)
        for a, b in de:
            t["de"] = t["de"].replace(a, b)


def rename(x, mapping):
    """🏷 Renames slugs (exact string values and dict keys) everywhere, including `region:`/`load:`/`emitter:` track targets."""
    if isinstance(x, dict):
        return {mapping.get(k, k): rename(v, mapping) for k, v in x.items()}
    if isinstance(x, list):
        return [rename(v, mapping) for v in x]
    if isinstance(x, str):
        if x in mapping:
            return mapping[x]
        if ":" in x and not " " in x:
            parts = x.split(":")
            return ":".join(mapping.get(p, p) for p in parts)
    return x


def de_num(v, n=2):
    return f"{v:.{n}f}".replace(".", ",")


def _anchor2(points, origin):
    return min(points, key=lambda p: (p[0] - origin[0]) ** 2 + (p[1] - origin[1]) ** 2)


def shrink(asset, morph, is_part, factors):
    """🫒 Shrinks small parts (fruit) per scale about the point that faces their bone origin, so they stay attached: {scale: factor}."""
    for key, rep in asset["representations"].items():
        f = factors.get(key, 1)
        if rep.get("status") != "authored" or f == 1:
            continue
        for view, shapes in rep["geometry"].items():
            for s in shapes:
                if not is_part(s):
                    continue
                if view == "elevation":
                    origin = (0, 0)
                else:
                    o = morph.new[s["bone"]][0]
                    origin = (o[2], o[1]) if view == "front" else (o[0], o[2])
                if s["kind"] == "ellipse":
                    c = (s["cx"], s["cy"])
                    d = math.hypot(origin[0] - c[0], origin[1] - c[1]) or 1
                    a = (c[0] + (origin[0] - c[0]) / d * s["rx"], c[1] + (origin[1] - c[1]) / d * s["ry"])
                    s["cx"], s["cy"] = r(a[0] + f * (c[0] - a[0]), 4), r(a[1] + f * (c[1] - a[1]), 4)
                    s["rx"], s["ry"] = r(s["rx"] * f, 4), r(s["ry"] * f, 4)
                elif s["kind"] == "path":
                    pts = []
                    map_path(s["d"], lambda x, y: pts.append((x, y)) or (x, y))
                    a = _anchor2(pts, origin)
                    s["d"] = map_path(s["d"], lambda x, y: (a[0] + f * (x - a[0]), a[1] + f * (y - a[1])))
                elif s["kind"] == "poly":
                    a = _anchor2(s["points"], origin)
                    s["points"] = [[r(a[0] + f * (p[0] - a[0]), 4), r(a[1] + f * (p[1] - a[1]), 4)] for p in s["points"]]
                else:
                    raise ValueError(s["kind"])
    for key, vol in asset["volumes"].items():
        f = factors.get(key, 1)
        if vol.get("status") != "authored" or f == 1:
            continue
        for lst in ("body", "thermal", "structure"):
            for s in vol.get(lst, []):
                if "kind" not in s or not is_part(s):
                    continue
                if s["kind"] != "polyhedron":
                    raise ValueError(s["kind"])
                tr = s.get("transform", {})
                Q = quat_matrix(tr["rotation"]) if "rotation" in tr else [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
                t = tr.get("translation", [0, 0, 0])
                a = min(s["vertices"], key=lambda v: sum((mat_vec(Q, v)[i] + t[i]) ** 2 for i in range(3)))
                s["vertices"] = [[r(a[i] + f * (v[i] - a[i]), 6) for i in range(3)] for v in s["vertices"]]


def each_shape(asset):
    """✏️ Every authored shape (representations and symbols) with its view."""
    for rep in asset["representations"].values():
        if rep.get("status") == "authored":
            for view, shapes in rep["geometry"].items():
                for s in shapes:
                    yield view, s
    for sym in asset["symbols"]:
        for view, shapes in sym["geometry"].items():
            for s in shapes:
                yield view, s


def each_solid(asset):
    """🧊 Every solid with geometry of every authored volume."""
    for vol in asset["volumes"].values():
        if vol.get("status") == "authored":
            for lst in ("body", "thermal", "structure"):
                for s in vol.get(lst, []):
                    if "kind" in s:
                        yield lst, s


def measured_dimensions(measure):
    """📐 Height, width and depth of the default pose as the mean over the authored scales (each within 2 % of it)."""
    views = [v["views"] for v in measure["scales"].values()]
    mean = lambda f: sum(f(v) for v in views) / len(views)
    return {
        "heightM": round(mean(lambda v: v["elevation"]["maxY"] - min(0, v["elevation"]["minY"])), 3),
        "widthM": round(mean(lambda v: v["elevation"]["maxX"] - v["elevation"]["minX"]), 3),
        "depthM": round(mean(lambda v: v["front"]["maxX"] - v["front"]["minX"]), 3),
    }


class WorldAffine:
    """🌐 A world-diagonal affine map `p' = P + A (p - P)` of one subtree of a planar rig (all bones with `pivot` None), about the rest origin
    `P` of its root. Directions turn, so the rest angles and lengths of the subtree are re-derived: a bone of world angle θ gets
    atan2(A_y sin θ, A_x cos θ) and the length factor |A u|. Bone-local geometry is carried by `L_b(q) = R_b'^T (map(o_b + R_b q) - o_b')`,
    which is exact for planar geometry and polyhedra; front, plan and true-extent symbols are mapped directly (no depth needed). Items that
    `extra` selects (a crown drawn on a bone outside the subtree) follow the map as well."""

    def __init__(self, asset, A, pivot=None, extra=None, keep=()):
        self.asset = asset
        self.A = A
        self.keep = set(keep)
        bones = asset["rig"]["bones"]
        for b in bones:
            if b.get("abductionDeg", 0) or b.get("twistDeg", 0):
                raise ValueError("WorldAffine needs a planar rig")
        self.by = {b["id"]: b for b in bones}
        self.old = frames(bones)
        children = {}
        for b in bones:
            children.setdefault(b["parent"], []).append(b["id"])
        root = pivot or asset["rig"]["root"]
        self.sub, stack = set(), [root]
        while stack:
            i = stack.pop()
            self.sub.add(i)
            stack.extend(children.get(i, []))
        self.P = self.old[root][0] if pivot else [0, 0, 0]
        self.extra = extra or (lambda item: False)
        self.theta = {}
        for i, (o, R) in self.old.items():
            th = math.atan2(R[1][0], R[0][0])
            self.theta[i] = math.atan2(A[1] * math.sin(th), A[0] * math.cos(th)) if i in self.sub and i not in self.keep else th
        self.new = {}
        for i, (o, R) in self.old.items():
            self.new[i] = (self.map(o) if i in self.sub else o, rot_w(math.degrees(self.theta[i])))

    def map(self, p):
        return [self.P[i] + self.A[i] * (p[i] - self.P[i]) for i in range(3)]

    def moves(self, item):
        return item.get("bone") in self.sub or self.extra(item)

    def lmap(self, bone, q):
        if bone in self.keep:
            return list(q)
        o, R = self.old[bone]
        o2, R2 = self.new[bone]
        p = self.map([o[i] + sum(R[i][k] * q[k] for k in range(3)) for i in range(3)])
        return mat_vec(transpose(R2), [p[i] - o2[i] for i in range(3)])

    def linear(self, bone):
        return [[self.lmap(bone, e)[i] - self.lmap(bone, [0, 0, 0])[i] for e in ([1, 0, 0], [0, 1, 0], [0, 0, 1])] for i in range(3)]

    def rig(self):
        for b in self.asset["rig"]["bones"]:
            i = b["id"]
            if i not in self.sub:
                continue
            o, R = self.old[i]
            u = [R[0][0], R[1][0], R[2][0]]
            stretch = 1 if i in self.keep else math.sqrt(sum((self.A[k] * u[k]) ** 2 for k in range(3)))
            parent = b["parent"]
            if parent is not None:
                po2, pR2 = self.new[parent]
                off = mat_vec(transpose(pR2), [self.new[i][0][k] - po2[k] for k in range(3)])
                b["x"], b["y"] = r(off[0], 4), r(off[1], 4)
                if "z" in b or abs(off[2]) > 1e-9:
                    b["z"] = r(off[2], 4)
                b["angleDeg"] = r(math.degrees(self.theta[i] - self.theta[parent]), 3)
            else:
                b["angleDeg"] = r(math.degrees(self.theta[i]), 3)
            b["lengthM"] = r(b["lengthM"] * stretch, 4)
            M = self.linear(i)
            across = math.sqrt(math.hypot(M[0][1], M[1][1]) * abs(M[2][2]))
            if "mass" in b:
                b["mass"]["kg"] = r(b["mass"]["kg"] * stretch * across * across, 2)
                b["mass"]["comX"], b["mass"]["comY"] = r(b["mass"]["comX"] * stretch, 4), r(b["mass"]["comY"] * across, 4)
            if "section" in b:
                sec = b["section"]
                sec["areaM2"] = float(f"{sec['areaM2'] * across * across:.5g}")
                sec["secondMomentM4"] = float(f"{sec['secondMomentM4'] * across ** 4:.4g}")
                sec["extremeFibreM"] = r(sec["extremeFibreM"] * across, 4)

    def shape(self, s, view):
        bone = s["bone"]
        if view == "elevation":
            f = lambda x, y: tuple(self.lmap(bone, [x, y, 0])[:2])
            M = self.linear(bone)
            ax, ay = math.hypot(M[0][0], M[1][0]), math.hypot(M[0][1], M[1][1])
        elif bone in self.keep:
            i, j = (2, 1) if view == "front" else (0, 2)
            o, o2 = self.old[bone][0], self.new[bone][0]
            f = lambda x, y: (x + o2[i] - o[i], y + o2[j] - o[j])
            ax = ay = 1
        else:
            i, j = (2, 1) if view == "front" else (0, 2)
            f = lambda x, y: (self.P[i] + self.A[i] * (x - self.P[i]), self.P[j] + self.A[j] * (y - self.P[j]))
            ax, ay = self.A[i], self.A[j]
        self._geometry(s, f, ax, ay)

    def _geometry(self, s, f, ax, ay):
        kind = s["kind"]
        if kind == "path":
            s["d"] = map_path(s["d"], f)
        elif kind == "poly":
            s["points"] = [[r(c, 4) for c in f(*p)] for p in s["points"]]
        elif kind == "ellipse":
            s["cx"], s["cy"] = [r(c, 4) for c in f(s["cx"], s["cy"])]
            s["rx"], s["ry"] = r(s["rx"] * ax, 4), r(s["ry"] * ay, 4)
        elif kind == "capsule":
            s["x0"], s["y0"] = [r(c, 4) for c in f(s["x0"], s["y0"])]
            s["x1"], s["y1"] = [r(c, 4) for c in f(s["x1"], s["y1"])]
            m = math.sqrt(ax * ay)
            s["r0"], s["r1"] = r(s["r0"] * m, 4), r(s["r1"] * m, 4)
        else:
            raise ValueError(kind)

    def symbol(self, s, view):
        if view == "elevation":
            i, j = 0, 1
        elif view == "front":
            i, j = 2, 1
        else:
            i, j = 0, 2
        f = lambda x, y: (self.P[i] + self.A[i] * (x - self.P[i]), self.P[j] + self.A[j] * (y - self.P[j]))
        self._geometry(s, f, self.A[i], self.A[j])

    def solid(self, s):
        if "kind" not in s:
            return
        bone = s["bone"]
        M = self.linear(bone)
        tr = s.get("transform", {})
        Q = quat_matrix(tr["rotation"]) if "rotation" in tr else [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        sc = tr.get("scale")
        if sc is not None and not isinstance(sc, list):
            sc = [sc, sc, sc]
        t = tr.get("translation", [0, 0, 0])
        t2 = self.lmap(bone, t)
        tr["translation"] = [r(c, 6) for c in t2]
        s["transform"] = tr
        B = mat_mul(mat_mul(transpose(Q), M), Q)
        col = [math.sqrt(sum(B[j][i] ** 2 for j in range(3))) for i in range(3)]
        mean = (col[0] * col[1] * col[2]) ** (1 / 3)
        kind = s["kind"]
        if kind == "polyhedron":
            pre = sc or [1, 1, 1]
            s["vertices"] = [[r(c, 6) for c in mat_vec(B, [v[i] * pre[i] for i in range(3)])] for v in s["vertices"]]
            tr.pop("scale", None)
        elif kind == "loft":
            pts = [sec["at"] for sec in s["sections"]]
            for n, sec in enumerate(s["sections"]):
                a, c = pts[max(0, n - 1)], pts[min(len(pts) - 1, n + 1)]
                d = [c[i] - a[i] for i in range(3)]
                L = math.sqrt(sum(x * x for x in d)) or 1
                d = [x / L for x in d]
                Md = mat_vec(B, d)
                along = math.sqrt(sum(x * x for x in Md))
                k = math.sqrt(max(1e-12, (col[0] * col[1] * col[2]) / max(along, 1e-9)))
                sec["at"] = [r(c, 6) for c in mat_vec(B, sec["at"])]
                sec["rx"], sec["ry"] = r(sec["rx"] * k, 6), r(sec["ry"] * k, 6)
        elif kind == "ellipsoid":
            s["radii"] = [r(s["radii"][i] * col[i], 6) for i in range(3)]
        elif kind == "revolve":
            a = s["axis"]
            n = math.sqrt(sum(c * c for c in a))
            Ma = mat_vec(B, [c / n for c in a])
            axial = math.sqrt(sum(c * c for c in Ma))
            radial = math.sqrt(max(1e-12, (col[0] * col[1] * col[2]) / axial))
            s["profile"] = [[r(p[0] * axial, 6), r(p[1] * radial, 6)] for p in s["profile"]]
            s["origin"] = [r(c, 6) for c in mat_vec(B, s["origin"])]
        elif kind == "capsule":
            s["p0"], s["p1"] = [r(c, 6) for c in mat_vec(B, s["p0"])], [r(c, 6) for c in mat_vec(B, s["p1"])]
            s["r0"], s["r1"] = r(s["r0"] * mean, 6), r(s["r1"] * mean, 6)
        elif kind == "box":
            s["size"] = [r(s["size"][i] * col[i], 6) for i in range(3)]
        else:
            raise ValueError(kind)
        if "repeat" in s:
            rep = s["repeat"]
            if "origin" in rep:
                rep["origin"] = [r(c, 6) for c in mat_vec(B, rep["origin"])]
            if "translation" in rep:
                rep["translation"] = [r(c, 6) for c in mat_vec(B, rep["translation"])]

    def apply(self):
        a = self.asset
        self.rig()
        for an in a["anchors"]:
            if self.moves(an):
                q = self.lmap(an["bone"], [an["x"], an["y"], an.get("z", 0)])
                an["x"], an["y"] = r(q[0], 4), r(q[1], 4)
                if "z" in an or abs(q[2]) > 1e-9:
                    an["z"] = r(q[2], 4)
        for rep in a["representations"].values():
            if rep.get("status") != "authored":
                continue
            for view, shapes in rep["geometry"].items():
                for s in shapes:
                    if self.moves(s):
                        self.shape(s, view)
        for sym in a["symbols"]:
            if sym["units"] != "model-m":
                continue
            for view, shapes in sym["geometry"].items():
                for s in shapes:
                    if self.moves(s) or self.extra(s):
                        self.symbol(s, view)
        for vol in a["volumes"].values():
            if vol.get("status") != "authored":
                continue
            for lst in ("body", "thermal", "structure"):
                for s in vol.get(lst, []):
                    if "kind" in s and self.moves(s):
                        self.solid(s)
        for p in a["poses"]:
            for ld in p.get("loads", []):
                if "bone" in ld and "x" in ld and self.moves(ld):
                    q = self.lmap(ld["bone"], [ld["x"], ld.get("y", 0), ld.get("z", 0)])
                    ld["x"] = r(q[0], 4)
                    if "y" in ld:
                        ld["y"] = r(q[1], 4)
                    if "z" in ld:
                        ld["z"] = r(q[2], 4)
                if "distributed" in ld and ld["distributed"]["bone"] in self.sub:
                    dd = ld["distributed"]
                    o, R = self.old[dd["bone"]]
                    st = 1 if dd["bone"] in self.keep else math.sqrt(sum((self.A[k] * R[k][0]) ** 2 for k in range(3)))
                    dd["fromM"], dd["toM"] = r(dd["fromM"] * st, 4), r(dd["toM"] * st, 4)


def inflate(asset, dy, is_leafy, centre_bones):
    """☁️ Thickens leaf masses vertically by `dy` about the mean height of the finest body of their bone (`centre_bones`), or about their own
    mean height when bound elsewhere; a vertical scale about a height needs no depth, so every view and scale agrees. A solid declared `slender`
    or `sheet` (a single leaf, a twig) is carried unchanged by the rise of its point nearest to the centre height (where it is attached): thickening it would end its declaration. Twigs stay as they are: they stand on a limb that does not move and end inside a mass that only grows."""
    fr = frames(asset["rig"]["bones"])

    def world(bone, q):
        o, R = fr[bone]
        v = mat_vec(R, q)
        return [o[i] + v[i] for i in range(3)]

    def local(bone, p):
        o, R = fr[bone]
        return mat_vec(transpose(R), [p[i] - o[i] for i in range(3)])

    def solid_points(s):
        tr = s.get("transform", {})
        Q = quat_matrix(tr["rotation"]) if "rotation" in tr else [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        t = tr.get("translation", [0, 0, 0])
        if s["kind"] == "polyhedron":
            return [world(s["bone"], [mat_vec(Q, v)[i] + t[i] for i in range(3)]) for v in s["vertices"]]
        if s["kind"] == "loft":
            return [world(s["bone"], [mat_vec(Q, sec["at"])[i] + t[i] for i in range(3)]) for sec in s["sections"]]
        return [world(s["bone"], t)]

    finest = min((k for k, v in asset["volumes"].items() if v.get("status") == "authored"), key=int)
    centre = {}
    for s in asset["volumes"][finest]["body"]:
        if "kind" in s and s["bone"] in centre_bones and is_leafy(s):
            centre.setdefault(s["bone"], []).extend(p[1] for p in solid_points(s))
    centre = {b: sum(v) / len(v) for b, v in centre.items()}
    lift = lambda cy: (lambda p: [p[0], cy + dy * (p[1] - cy), p[2]])

    for rep in asset["representations"].values():
        if rep.get("status") != "authored":
            continue
        for view, shapes in rep["geometry"].items():
            for s in shapes:
                if not is_leafy(s) or "twig" in s["id"]:
                    continue
                pts = []
                if view == "elevation":
                    to_w = lambda x, y: world(s["bone"], [x, y, 0])
                elif view == "front":
                    to_w = lambda x, y: [0, y, x]
                else:
                    to_w = lambda x, y: [x, 0, y]
                if view == "plan":
                    continue
                if s["kind"] == "path":
                    map_path(s["d"], lambda x, y: pts.append(to_w(x, y)) or (x, y))
                elif s["kind"] == "poly":
                    pts = [to_w(*p) for p in s["points"]]
                elif s["kind"] == "ellipse":
                    pts = [to_w(s["cx"], s["cy"])]
                cy = centre.get(s["bone"], sum(p[1] for p in pts) / max(1, len(pts)))
                m = lift(cy)
                if view == "elevation":
                    f = lambda x, y: tuple(local(s["bone"], m(world(s["bone"], [x, y, 0])))[:2])
                    o, R = fr[s["bone"]]
                    vx, vy = math.hypot(R[0][0], dy * R[1][0]), math.hypot(R[0][1], dy * R[1][1])
                else:
                    f = lambda x, y: (x, cy + dy * (y - cy))
                    vx, vy = 1, dy
                if s["kind"] == "path":
                    s["d"] = map_path(s["d"], f)
                elif s["kind"] == "poly":
                    s["points"] = [[r(c, 4) for c in f(*p)] for p in s["points"]]
                elif s["kind"] == "ellipse":
                    s["cx"], s["cy"] = [r(c, 4) for c in f(s["cx"], s["cy"])]
                    s["rx"], s["ry"] = r(s["rx"] * vx, 4), r(s["ry"] * vy, 4)
                else:
                    raise ValueError(s["kind"])
    for vol in asset["volumes"].values():
        if vol.get("status") != "authored":
            continue
        for lst in ("body", "thermal", "structure"):
            for s in vol.get(lst, []):
                if "kind" not in s or not is_leafy(s):
                    continue
                pts = solid_points(s)
                cy = centre.get(s["bone"], sum(p[1] for p in pts) / len(pts))
                m = lift(cy)
                tr = s.get("transform", {})
                Q = quat_matrix(tr["rotation"]) if "rotation" in tr else [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
                sc = tr.get("scale")
                if sc is not None and not isinstance(sc, list):
                    sc = [sc, sc, sc]
                t = tr.get("translation", [0, 0, 0])
                t2 = local(s["bone"], m(world(s["bone"], t)))
                if "twig" in s["id"]:
                    continue
                if s.get("slender") or s.get("sheet"):
                    low = min(pts, key=lambda q: q[1]) if cy <= sum(q[1] for q in pts) / len(pts) else max(pts, key=lambda q: q[1])
                    shift = mat_vec(transpose(fr[s["bone"]][1]), [0, m(low)[1] - low[1], 0])
                    tr["translation"] = [r(t[i] + shift[i], 6) for i in range(3)]
                    s["transform"] = tr
                    continue
                o, R = fr[s["bone"]]
                D = [[1, 0, 0], [0, dy, 0], [0, 0, 1]]
                M = mat_mul(mat_mul(transpose(R), D), R)
                B = mat_mul(mat_mul(transpose(Q), M), Q)
                col = [math.sqrt(sum(B[j][i] ** 2 for j in range(3))) for i in range(3)]
                if s["kind"] == "polyhedron":
                    pre = sc or [1, 1, 1]
                    s["vertices"] = [[r(c, 6) for c in mat_vec(B, [v[i] * pre[i] for i in range(3)])] for v in s["vertices"]]
                    tr.pop("scale", None)
                elif s["kind"] == "loft":
                    for sec in s["sections"]:
                        Rs = mat_mul(mat_mul(R, Q), quat_matrix(sec.get("rotation", [0, 0, 0, 1])))
                        fx, fy = math.hypot(Rs[0][0], dy * Rs[1][0], Rs[2][0]), math.hypot(Rs[0][1], dy * Rs[1][1], Rs[2][1])
                        sec["at"] = [r(c, 6) for c in mat_vec(B, sec["at"])]
                        sec["rx"], sec["ry"] = r(sec["rx"] * fx, 6), r(sec["ry"] * fy, 6)
                elif s["kind"] == "ellipsoid":
                    s["radii"] = [r(s["radii"][i] * col[i], 6) for i in range(3)]
                else:
                    raise ValueError(s["kind"])
                tr["translation"] = [r(c, 6) for c in t2]
                s["transform"] = tr
                if "repeat" in s:
                    rep = s["repeat"]
                    if "origin" in rep:
                        rep["origin"] = [r(c, 6) for c in mat_vec(B, rep["origin"])]
                    if "translation" in rep:
                        rep["translation"] = [r(c, 6) for c in mat_vec(B, rep["translation"])]
    for an in asset["anchors"]:
        if is_leafy(an) and an["bone"] in centre:
            q = local(an["bone"], lift(centre[an["bone"]])(world(an["bone"], [an["x"], an["y"], an.get("z", 0)])))
            an["x"], an["y"] = r(q[0], 4), r(q[1], 4)
            if "z" in an:
                an["z"] = r(q[2], 4)


def principal_axes(points):
    """🧭 Principal directions (rows) of a point cloud by Jacobi rotations of its covariance."""
    n = len(points)
    mean = [sum(p[i] for p in points) / n for i in range(3)]
    C = [[sum((p[i] - mean[i]) * (p[j] - mean[j]) for p in points) for j in range(3)] for i in range(3)]
    V = [[1.0, 0, 0], [0, 1.0, 0], [0, 0, 1.0]]
    for _ in range(50):
        off = max(abs(C[0][1]), abs(C[0][2]), abs(C[1][2]))
        if off < 1e-15 * (abs(C[0][0]) + abs(C[1][1]) + abs(C[2][2]) + 1e-30):
            break
        for a, b in ((0, 1), (0, 2), (1, 2)):
            if abs(C[a][b]) < 1e-300:
                continue
            th = 0.5 * math.atan2(2 * C[a][b], C[b][b] - C[a][a])
            c, s = math.cos(th), math.sin(th)
            J = [[1.0, 0, 0], [0, 1.0, 0], [0, 0, 1.0]]
            J[a][a], J[b][b], J[a][b], J[b][a] = c, c, s, -s
            C = mat_mul(mat_mul(transpose(J), C), J)
            V = mat_mul(V, J)
    return mean, [[V[k][i] for k in range(3)] for i in range(3)]


def floor_solid(s, minimum):
    """📏 Thickens a solid along its thinnest principal direction (about its centre) until that extent reaches `minimum`."""
    kind = s["kind"]
    if kind == "polyhedron":
        mean, axes = principal_axes(s["vertices"])
        spans = []
        for ax in axes:
            d = [sum(ax[i] * (v[i] - mean[i]) for i in range(3)) for v in s["vertices"]]
            spans.append(max(d) - min(d))
        k = min(range(3), key=lambda i: spans[i])
        f = minimum / spans[k]
        if f <= 1:
            return
        ax = axes[k]
        out = []
        for v in s["vertices"]:
            d = sum(ax[i] * (v[i] - mean[i]) for i in range(3))
            out.append([r(v[i] + (f - 1) * d * ax[i], 6) for i in range(3)])
        s["vertices"] = out
    elif kind == "ellipsoid":
        s["radii"] = [r(max(x, minimum / 2), 6) for x in s["radii"]]
    elif kind == "loft":
        for sec in s["sections"]:
            sec["rx"], sec["ry"] = r(max(sec["rx"], minimum / 2), 6), r(max(sec["ry"], minimum / 2), 6)
    elif kind == "capsule":
        s["r0"], s["r1"] = r(max(s["r0"], minimum / 2), 6), r(max(s["r1"], minimum / 2), 6)
    elif kind == "revolve":
        ax = [p[0] for p in s["profile"]]
        span, wide = max(ax) - min(ax), 2 * max(p[1] for p in s["profile"])
        if span <= wide and span < minimum:
            m = (max(ax) + min(ax)) / 2
            s["profile"] = [[r(m + (p[0] - m) * minimum / span, 6), p[1]] for p in s["profile"]]
        elif wide < minimum:
            s["profile"] = [[p[0], r(p[1] * minimum / wide, 6)] for p in s["profile"]]
    else:
        raise ValueError(kind)


def apply_floors(asset, floors):
    """📏 Applies the recorded feature floors `[{scale, list, id, minimumM}]` (design input written from the validation)."""
    for fl in floors:
        for s in asset["volumes"][fl["scale"]][fl["list"]]:
            if s["id"] == fl["id"] and "kind" in s:
                floor_solid(s, fl["minimumM"])


THIN = re.compile(r'Solid "([^"]+)" of (body|thermal|structure) is ([0-9.]+) mm thin, below ([0-9.]+) mm')


def floors_from(issues, known=()):
    """📏 Feature floors from the `volumes-feature` issues of a validation (1 % above the limit), merged with the known ones."""
    out = {(f["scale"], f["list"], f["id"]): f for f in known}
    for issue in issues:
        m = THIN.search(issue["message"]["en"])
        if issue["code"] != "volumes-feature" or not m:
            continue
        scale = issue["path"].split(".")[1]
        key = (scale, m.group(2), m.group(1))
        need = round(float(m.group(4)) / 1000 * 1.01, 5)
        out[key] = {"scale": scale, "list": m.group(2), "id": m.group(1), "minimumM": max(need, out.get(key, {}).get("minimumM", 0))}
    return sorted(out.values(), key=lambda f: (int(f["scale"]), f["list"], f["id"]))


def _points(s):
    if s["kind"] == "path":
        pts = []
        map_path(s["d"], lambda x, y: pts.append((x, y)) or (x, y))
        return pts
    if s["kind"] == "poly":
        return [tuple(p) for p in s["points"]]
    if s["kind"] == "ellipse":
        return [(s["cx"] - s["rx"], s["cy"] - s["ry"]), (s["cx"] + s["rx"], s["cy"] + s["ry"])]
    raise ValueError(s["kind"])


def fit_symbols(asset, measure):
    """🎯 Fits every true-extent symbol view to the measured default-pose extent of the drawings (mean over the authored scales): the symbol is a
    conventional simplification at true extent, so its box is the box of the drawings."""
    scales = list(measure["scales"].values())
    for sym in asset["symbols"]:
        if sym["units"] != "model-m":
            continue
        for view, shapes in sym["geometry"].items():
            pts = [p for s in shapes for p in _points(s)]
            x0, x1 = min(p[0] for p in pts), max(p[0] for p in pts)
            y0, y1 = min(p[1] for p in pts), max(p[1] for p in pts)
            t = {k: sum(v["views"][view][k] for v in scales) / len(scales) for k in ("minX", "maxX", "minY", "maxY")}
            if view != "plan":
                t["minY"] = min(0, t["minY"])
            sx, sy = (t["maxX"] - t["minX"]) / (x1 - x0), (t["maxY"] - t["minY"]) / (y1 - y0)
            f = lambda x, y: (t["minX"] + sx * (x - x0), t["minY"] + sy * (y - y0))
            for s in shapes:
                if s["kind"] == "path":
                    s["d"] = map_path(s["d"], f)
                elif s["kind"] == "poly":
                    s["points"] = [[r(c, 4) for c in f(*p)] for p in s["points"]]
                else:
                    s["cx"], s["cy"] = [r(c, 4) for c in f(s["cx"], s["cy"])]
                    s["rx"], s["ry"] = r(s["rx"] * sx, 4), r(s["ry"] * sy, 4)
