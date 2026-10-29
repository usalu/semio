# [scratch] converts the authored dairy-cow outlines (world coordinates at the standing rest frame) to bone-local geometry JSON
import sys, json, importlib.util
D = sys.argv[1]
spec = importlib.util.spec_from_file_location("cl", __file__.replace("cow-geom.py", "cow-lib.py")); cl = importlib.util.module_from_spec(spec); spec.loader.exec_module(cl)
bones = json.loads("{" + open(D + "/10-rig.json").read().rstrip().rstrip(",") + "}")["rig"]["bones"]
RW = cl.rest_world(bones)
SH = []
def add(id, bone, role, lod, z, tone, region, material, geom, filled=True):
    s = {"id": id, "bone": bone, "role": role, "lod": lod, "z": z, "filled": filled, "tone": tone}
    if region: s["region"] = region
    if material: s["material"] = material
    s.update(geom); SH.append(s)
def wpath(bone, d, at=(0, 0), sc=1.0):
    m = cl.inv(RW[bone]); toks = d.split(); out = []; i = 0
    while i < len(toks):
        c = toks[i]; i += 1
        if c == "Z": out.append("Z"); continue
        n = {"M": 2, "L": 2, "C": 6}[c]; v = [float(x) for x in toks[i:i+n]]; i += n
        pts = []
        for k in range(0, n, 2):
            x, y = cl.app(m, v[k]*sc + at[0], v[k+1]*sc + at[1]); pts += [round(x, 3), round(y, 3)]
        out.append(c + " " + " ".join(f"{p:g}" for p in pts))
    return {"kind": "path", "d": " ".join(out)}
def wpoly(bone, pts, closed=False):
    m = cl.inv(RW[bone]); return {"kind": "poly", "closed": closed, "points": [[round(v, 3) for v in cl.app(m, x, y)] for x, y in pts]}
def wellipse(bone, cx, cy, rx, ry, rot=0):
    m = cl.inv(RW[bone]); x, y = cl.app(m, cx, cy); g = {"kind": "ellipse", "cx": round(x, 3), "cy": round(y, 3), "rx": rx, "ry": ry}
    import math
    r = rot - math.degrees(math.atan2(RW[bone][1], RW[bone][0]))
    if abs(r) > 0.5: g["rotationDeg"] = round(r, 1)
    return g
def wcap(bone, x0, y0, x1, y1, r0, r1):
    m = cl.inv(RW[bone]); a = cl.app(m, x0, y0); b = cl.app(m, x1, y1)
    return {"kind": "capsule", "x0": round(a[0], 3), "y0": round(a[1], 3), "x1": round(b[0], 3), "y1": round(b[1], 3), "r0": r0, "r1": r1}
def lpath(d): return {"kind": "path", "d": d}
def lcap(x0, y0, x1, y1, r0, r1): return {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": r0, "r1": r1}
def lpoly(pts, closed=False): return {"kind": "poly", "closed": closed, "points": pts}
def lell(cx, cy, rx, ry): return {"kind": "ellipse", "cx": cx, "cy": cy, "rx": rx, "ry": ry}

WH, BL, BS, CH = "white-hair", "black-hair", "bare-skin", "claw-horn"
HOOF = "M -0.03 0.02 C -0.035 -0.03 -0.04 -0.06 -0.04 -0.085 C -0.045 -0.11 -0.04 -0.128 -0.03 -0.13 L 0.105 -0.13 C 0.115 -0.12 0.105 -0.095 0.085 -0.07 C 0.07 -0.055 0.05 -0.03 0.04 0.005 C 0.03 0.03 -0.02 0.035 -0.03 0.02 Z"
HORN = "M -0.042 -0.078 L 0.08 -0.06 C 0.1 -0.085 0.115 -0.115 0.105 -0.13 L -0.03 -0.13 C -0.045 -0.12 -0.046 -0.1 -0.042 -0.078 Z"
SHANK = "M -0.05 0.06 C 0.05 0.075 0.18 0.055 0.3 0.042 L 0.44 0.034 C 0.48 0.03 0.5 0.0 0.5 -0.02 C 0.5 -0.06 0.48 -0.075 0.45 -0.07 L 0.3 -0.048 C 0.18 -0.085 0.05 -0.105 -0.04 -0.085 C -0.075 -0.05 -0.075 0.02 -0.05 0.06 Z"
CANNON_H = "M -0.03 0.035 L 0.05 0.032 L 0.29 0.024 C 0.31 0.02 0.31 -0.02 0.29 -0.026 L 0.06 -0.034 C 0.0 -0.04 -0.04 -0.05 -0.05 -0.03 C -0.055 0 -0.05 0.03 -0.03 0.035 Z"
FOREARM = "M -0.04 0.055 C 0.03 0.07 0.12 0.06 0.2 0.05 L 0.36 0.04 C 0.39 0.045 0.42 0.04 0.42 0.0 C 0.42 -0.04 0.4 -0.045 0.37 -0.04 L 0.2 -0.04 C 0.12 -0.06 0.03 -0.075 -0.04 -0.06 C -0.08 -0.03 -0.08 0.03 -0.04 0.055 Z"
CANNON_F = "M -0.03 0.04 L 0.26 0.028 C 0.285 0.022 0.285 -0.022 0.26 -0.03 L -0.03 -0.04 C -0.045 -0.01 -0.045 0.01 -0.03 0.04 Z"
THIGH = "M 0.2 1.04 C 0.3 1.02 0.38 0.96 0.38 0.86 C 0.38 0.78 0.33 0.72 0.26 0.7 C 0.12 0.7 0.0 0.76 -0.1 0.86 C -0.17 0.93 -0.2 1.0 -0.18 1.06 C -0.1 1.09 0.08 1.09 0.2 1.04 Z"
SCAPULA = "M 1.04 1.38 C 1.14 1.42 1.26 1.42 1.32 1.36 C 1.42 1.24 1.46 1.1 1.44 0.98 C 1.42 0.9 1.36 0.9 1.32 0.92 C 1.2 1.0 1.12 1.1 1.06 1.2 C 1.02 1.28 1.02 1.34 1.04 1.38 Z"
def limbs(s, near):
    z = 5.0 if near else 1.0; tone = 0.9 if near else 0.55; n = "near" if near else "far"
    # hind
    add(f"{n}-thigh", f"thigh-h{s}", "body", 0, z, tone, "coat-limbs", WH, wpath(f"thigh-h{s}", THIGH))
    add(f"{n}-gaskin", f"shank-h{s}", "body", 0, z, tone, "coat-limbs", WH, lpath(SHANK))
    add(f"{n}-hind-cannon", f"metatarsus-h{s}", "body", 0, z, tone, "lower-legs", BS, lpath(CANNON_H))
    fh = RW[f"hoof-h{s}"][4:6]
    add(f"{n}-hind-pastern", f"hoof-h{s}", "body", 0, z, tone, "lower-legs", BS, wpath(f"hoof-h{s}", HOOF, at=fh))
    add(f"{n}-hind-hoof", f"hoof-h{s}", "body", 0, z + 0.05, 0.22 if near else 0.2, "hooves", CH, wpath(f"hoof-h{s}", HORN, at=fh))
    # fore
    add(f"{n}-shoulder", f"scapula-f{s}", "body", 0, z, tone, "coat-limbs", WH, wpath(f"scapula-f{s}", SCAPULA))
    add(f"{n}-upper-arm", f"humerus-f{s}", "body", 0, z, tone, "coat-limbs", WH, lcap(0.0, 0.0, 0.36, 0.0, 0.085, 0.062))
    add(f"{n}-forearm", f"forearm-f{s}", "body", 0, z, tone, "coat-limbs", WH, lpath(FOREARM))
    add(f"{n}-fore-cannon", f"cannon-f{s}", "body", 0, z, tone, "lower-legs", BS, lpath(CANNON_F))
    ff = RW[f"hoof-f{s}"][4:6]
    add(f"{n}-fore-pastern", f"hoof-f{s}", "body", 0, z, tone, "lower-legs", BS, wpath(f"hoof-f{s}", HOOF, at=ff, sc=1.05))
    add(f"{n}-fore-hoof", f"hoof-f{s}", "body", 0, z + 0.05, 0.22 if near else 0.2, "hooves", CH, wpath(f"hoof-f{s}", HORN, at=ff, sc=1.05))
limbs("f", False)
add("far-ear", "ear-f", "body", 0, 2.0, 0.14, "ears", BL, lpath("M 0.0 0.025 C 0.06 0.065 0.15 0.06 0.21 0.0 C 0.15 -0.055 0.06 -0.055 0.0 -0.025 Z"))
for i, (b, r0, r1, l) in enumerate((("tail-1", 0.04, 0.032, 0.3), ("tail-2", 0.032, 0.022, 0.45))):
    add(b.replace("tail", "tail-root" if i == 0 else "tail-mid").replace("tail-root-1", "tail-root").replace("tail-mid-2", "tail-mid"), b, "body", 0, 2.8, 0.14, "coat-tail", BL, lcap(0, 0, l, 0, r0, r1))
add("tail-switch", "tail-3", "body", 0, 2.8, 0.93, "coat-tail", WH, lpath("M 0.0 0.018 C 0.06 0.022 0.12 0.055 0.22 0.055 C 0.3 0.055 0.35 0.025 0.35 0.0 C 0.35 -0.025 0.3 -0.055 0.22 -0.055 C 0.12 -0.055 0.06 -0.022 0.0 -0.018 Z"))
PELV = "M -0.27 1.4 C -0.35 1.35 -0.37 1.25 -0.35 1.17 C -0.33 1.07 -0.29 0.95 -0.23 0.85 C -0.1 0.78 0.1 0.76 0.24 0.8 C 0.3 0.86 0.33 0.93 0.34 1.0 L 0.36 1.4 C 0.22 1.42 0.12 1.46 0.02 1.45 C -0.1 1.44 -0.2 1.42 -0.27 1.4 Z"
LUMB = "M 0.3 1.4 C 0.42 1.4 0.55 1.37 0.7 1.375 L 0.84 1.38 L 0.86 0.66 C 0.8 0.58 0.66 0.55 0.54 0.58 C 0.44 0.62 0.38 0.7 0.34 0.8 C 0.31 0.88 0.3 0.95 0.3 1.0 Z"
THOR = "M 0.8 1.38 C 0.95 1.4 1.08 1.45 1.2 1.46 C 1.34 1.47 1.44 1.42 1.5 1.32 C 1.55 1.2 1.55 1.05 1.5 0.93 C 1.47 0.78 1.42 0.68 1.33 0.62 C 1.2 0.57 1.02 0.56 0.88 0.6 L 0.8 0.62 Z"
NECK = "M 1.3 1.45 C 1.52 1.44 1.74 1.4 1.93 1.37 C 2.0 1.34 2.0 1.22 1.95 1.14 C 1.84 1.08 1.72 1.04 1.62 0.98 C 1.55 0.92 1.52 0.84 1.46 0.78 C 1.4 0.72 1.52 0.9 1.5 1.1 C 1.48 1.28 1.4 1.4 1.3 1.45 Z"
UDDER = "M -0.1 0.8 C -0.14 0.66 -0.12 0.52 -0.04 0.48 C 0.08 0.45 0.26 0.46 0.36 0.52 C 0.43 0.58 0.46 0.68 0.42 0.8 Z"
TRUNK = "M -0.27 1.4 C -0.1 1.44 0.0 1.46 0.12 1.45 C 0.3 1.42 0.5 1.37 0.7 1.375 C 0.95 1.4 1.08 1.45 1.2 1.46 C 1.34 1.47 1.44 1.42 1.5 1.32 C 1.55 1.2 1.55 1.05 1.5 0.93 C 1.47 0.78 1.42 0.68 1.33 0.62 C 1.2 0.57 1.02 0.56 0.88 0.58 C 0.74 0.56 0.62 0.55 0.54 0.58 C 0.44 0.62 0.38 0.7 0.34 0.8 L 0.24 0.8 C 0.14 0.72 0.04 0.64 -0.06 0.58 C -0.12 0.56 -0.17 0.6 -0.2 0.68 C -0.27 0.84 -0.33 1.04 -0.35 1.17 C -0.37 1.25 -0.35 1.35 -0.27 1.4 Z"
add("trunk", "lumbar", "body", 0, 3.1, 0.9, "coat-trunk", WH, wpath("lumbar", TRUNK))
add("neck", "neck", "body", 0, 3.3, 0.9, "coat-head-neck", WH, wpath("neck", NECK))
add("udder", "udder", "body", 0, 3.4, 0.7, "udder", BS, wpath("udder", UDDER))
add("teat-front", "udder", "body", 1, 3.45, 0.62, "udder", BS, wcap("udder", 0.3, 0.52, 0.3, 0.39, 0.02, 0.016))
add("teat-rear", "udder", "body", 1, 3.45, 0.62, "udder", BS, wcap("udder", 0.0, 0.5, 0.0, 0.37, 0.02, 0.016))
add("rumen-flank", "lumbar", "detail", 1, 3.5, 0.8, "flank-rumen", WH, wellipse("lumbar", 0.56, 1.06, 0.17, 0.2))
add("patch-rump", "lumbar", "detail", 1, 3.6, 0.14, "coat-patches", BL, wpath("lumbar", "M -0.3 1.36 C -0.2 1.4 0.05 1.43 0.3 1.39 C 0.3 1.2 0.2 1.12 0.08 1.14 C -0.05 1.16 -0.12 1.04 -0.24 1.1 C -0.32 1.18 -0.34 1.28 -0.3 1.36 Z"))
add("patch-barrel", "lumbar", "detail", 1, 3.6, 0.14, "coat-patches", BL, wpath("lumbar", "M 0.5 1.37 C 0.6 1.38 0.74 1.37 0.82 1.37 C 0.84 1.26 0.8 1.1 0.72 1.0 C 0.66 0.94 0.6 0.99 0.55 0.96 C 0.48 1.0 0.44 1.22 0.5 1.37 Z"))
add("patch-chest", "lumbar", "detail", 1, 3.6, 0.14, "coat-patches", BL, wpath("lumbar", "M 0.86 1.37 C 0.96 1.4 1.04 1.43 1.14 1.45 C 1.12 1.3 1.1 1.15 1.0 1.08 C 0.92 1.04 0.86 1.1 0.86 1.37 Z"))
add("head", "head", "body", 0, 4.0, 0.16, "coat-patches", BL, lpath("M -0.04 0.1 C 0.1 0.108 0.25 0.088 0.36 0.072 C 0.43 0.066 0.49 0.075 0.52 0.055 C 0.545 0.03 0.545 -0.03 0.52 -0.06 C 0.47 -0.075 0.4 -0.065 0.34 -0.07 C 0.25 -0.09 0.15 -0.14 0.06 -0.15 C -0.03 -0.14 -0.07 -0.05 -0.06 0.04 Z"))
add("blaze", "head", "detail", 1, 4.1, 0.93, "coat-head-neck", WH, lpath("M 0.04 0.09 C 0.14 0.098 0.26 0.08 0.4 0.066 C 0.42 0.04 0.4 0.03 0.36 0.034 C 0.26 0.05 0.14 0.058 0.05 0.05 Z"))
add("muzzle", "head", "body", 0, 4.2, 0.5, "muzzle", BS, lpath("M 0.44 0.07 C 0.48 0.074 0.5 0.07 0.52 0.055 C 0.545 0.03 0.545 -0.03 0.52 -0.06 C 0.49 -0.066 0.46 -0.066 0.44 -0.062 C 0.46 -0.03 0.46 0.03 0.44 0.07 Z"))
add("jaw", "jaw", "body", 0, 3.9, 0.55, "coat-head-neck", WH, lpath("M -0.02 0.045 L 0.12 0.03 C 0.24 0.02 0.31 0.016 0.355 0.03 C 0.375 0.0 0.365 -0.03 0.34 -0.04 L 0.2 -0.05 C 0.1 -0.06 0.04 -0.09 -0.01 -0.07 C -0.05 -0.05 -0.05 0.02 -0.02 0.045 Z"))
add("eye", "head", "body", 0, 5.0, 0.04, "eyes", BS, lell(0.17, 0.052, 0.014, 0.011))
add("nostril", "head", "detail", 2, 4.3, 0.05, None, None, lell(0.515, 0.03, 0.012, 0.007))
add("mouth-line", "head", "detail", 1, 4.3, 0.1, None, None, lpoly([[0.53, -0.04], [0.44, -0.06], [0.34, -0.07]]), False)
add("near-ear", "ear-n", "body", 0, 4.5, 0.14, "ears", BL, lpath("M 0.0 0.025 C 0.06 0.065 0.15 0.06 0.21 0.0 C 0.15 -0.055 0.06 -0.055 0.0 -0.025 Z"))
limbs("n", True)
add("flank-fold", "lumbar", "detail", 2, 5.5, 0.4, None, None, wpoly("lumbar", [(0.32, 1.0), (0.37, 0.92), (0.4, 0.84)]), False)
add("hook-bone", "lumbar", "detail", 2, 5.5, 0.4, None, None, wpoly("lumbar", [(0.0, 1.43), (0.08, 1.38), (0.14, 1.3)]), False)
add("ribs", "lumbar", "texture", 3, 5.5, 0.45, None, None, wpoly("lumbar", [(0.95, 1.3), (0.97, 1.0), (0.99, 0.72)]), False)
add("ribs-2", "lumbar", "texture", 3, 5.5, 0.45, None, None, wpoly("lumbar", [(0.82, 1.28), (0.85, 1.0), (0.87, 0.7)]), False)
add("milk-vein", "lumbar", "detail", 3, 5.5, 0.45, None, None, wpoly("lumbar", [(0.45, 0.66), (0.6, 0.64), (0.8, 0.62)]), False)
add("dewlap-fold", "neck", "detail", 2, 5.5, 0.4, None, None, wpoly("neck", [(1.58, 1.0), (1.5, 0.9), (1.44, 0.82)]), False)
add("neck-crest", "neck", "texture", 3, 5.5, 0.45, None, None, wpoly("neck", [(1.4, 1.37), (1.62, 1.34), (1.82, 1.3)]), False)
add("hair-hock", "shank-hn", "texture", 3, 5.5, 0.45, None, None, lpoly([[0.1, -0.03], [0.3, -0.02]]), False)
add("hind-dewclaws", "hoof-hn", "detail", 3, 5.6, 0.1, None, None, lcap(-0.03, -0.01, 0.0, -0.03, 0.01, 0.008))
add("fore-dewclaws", "hoof-fn", "detail", 3, 5.6, 0.1, None, None, lcap(-0.03, -0.01, 0.0, -0.03, 0.01, 0.008))
lines = ",\n".join("          " + json.dumps(s, ensure_ascii=False) for s in SH)
open(D + "/30-geometry.json", "w").write('      "geometry": {\n        "elevation": [\n' + lines + '\n        ]\n      },\n')
print(len(SH), "shapes")
