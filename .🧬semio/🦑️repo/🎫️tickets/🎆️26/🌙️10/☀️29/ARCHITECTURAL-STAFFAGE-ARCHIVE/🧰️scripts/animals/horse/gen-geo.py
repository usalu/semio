# [scratch] converts hand-authored rest-world outlines to bone-local shape JSON lines for 30-geometry.json
import sys, re, json, math
sys.path.insert(0, "..")
import kin
exec(open("gen-rig.py").read().split("out =")[0].replace("print(sum", "(sum"))
A = kin.Asset({"rig": {"bones": rows}, "anchors": []})
W = A.fk({})
def inv(bone, x, y):
    m = W[bone]; dx, dy = x - m[4], y - m[5]
    return m[0]*dx + m[1]*dy, m[2]*dx + m[3]*dy
def conv(bone, d, rel=False):
    t = d.split(); out = []; i = 0; ox, oy = (W[bone][4], W[bone][5]) if rel else (0, 0)
    while i < len(t):
        c = t[i]; i += 1
        if c == "Z": out.append("Z"); continue
        n = {"M": 1, "L": 1, "C": 3}[c]; out.append(c)
        for _ in range(n):
            x, y = inv(bone, float(t[i]) + ox, float(t[i+1]) + oy); i += 2
            out += [f"{round(x, 3):g}", f"{round(y, 3):g}"]
    return " ".join(out)
S = []
def sh(id, bone, role, lod, z, tone, kind, region=None, material=None, filled=True, **g):
    d = {"id": id, "bone": bone, "role": role, "lod": lod, "z": z, "filled": filled, "tone": tone}
    if region: d["region"] = region
    if material: d["material"] = material
    d["kind"] = kind; d.update(g); S.append(d)
def wpath(id, bone, role, lod, z, tone, d, region=None, material=None, rel=False, filled=True): sh(id, bone, role, lod, z, tone, "path", region, material, filled, d=conv(bone, d, rel))
def lpath(id, bone, role, lod, z, tone, d, region=None, material=None, filled=True): sh(id, bone, role, lod, z, tone, "path", region, material, filled, d=d)
def poly(id, bone, role, lod, z, tone, pts, region=None, material=None, closed=False, filled=False, world=False):
    p = [[round(v, 3) for v in (inv(bone, *q) if world else q)] for q in pts]; sh(id, bone, role, lod, z, tone, "poly", region, material, filled, points=p, closed=closed)
def cap(id, bone, role, lod, z, tone, x0, y0, x1, y1, r0, r1, region=None, material=None): sh(id, bone, role, lod, z, tone, "capsule", region, material, True, x0=x0, y0=y0, x1=x1, y1=y1, r0=r0, r1=r1)
def ell(id, bone, role, lod, z, tone, cx, cy, rx, ry, region=None, material=None, rot=None):
    g = dict(cx=cx, cy=cy, rx=rx, ry=ry)
    if rot is not None: g["rotationDeg"] = rot
    sh(id, bone, role, lod, z, tone, "ellipse", region, material, True, **g)
CF, MH, HH, BS, EY = "short-coat", "mane-tail-hair", "hoof-horn", "bare-skin", "eye-surface"
NEAR, FAR = 0.72, 0.5
# ---- limbs, near and far
for s, zb, tn, nm in (("f", 1.0, FAR, "far"), ("n", 5.0, NEAR, "near")):
    wpath(f"{nm}-thigh", f"thigh-h{s}", "body", 0, zb if s == "f" else 2.9, tn, "M -0.22 -0.30 C -0.24 -0.46 -0.14 -0.54 0.0 -0.56 C 0.15 -0.58 0.28 -0.58 0.34 -0.48 C 0.38 -0.40 0.30 -0.34 0.22 -0.30 L 0.0 -0.2 L -0.22 -0.2 Z", "coat-limbs", CF)
    wpath(f"{nm}-gaskin", f"shank-h{s}", "body", 0, zb if s == "f" else 2.92, tn, "M 0.31 -0.47 C 0.22 -0.54 0.10 -0.64 0.02 -0.74 C 0.0 -0.78 -0.02 -0.80 -0.03 -0.82 L -0.1 -0.83 L -0.16 -0.8 C -0.17 -0.72 -0.2 -0.58 -0.25 -0.42 C -0.2 -0.38 0.1 -0.36 0.2 -0.38 C 0.26 -0.4 0.3 -0.43 0.31 -0.47 Z", "coat-limbs", CF)
    lpath(f"{nm}-hind-cannon", f"cannon-h{s}", "body", 0, zb, tn, "M -0.05 0.045 C 0.0 0.055 0.1 0.038 0.2 0.032 L 0.3 0.03 C 0.33 0.045 0.36 0.045 0.38 0.03 L 0.38 -0.04 C 0.36 -0.055 0.32 -0.055 0.3 -0.036 L 0.2 -0.03 C 0.1 -0.036 0.02 -0.05 -0.05 -0.045 Z", "coat-limbs", CF)
    cap(f"{nm}-hind-pastern", f"pastern-h{s}", "body", 0, zb, tn, -0.015, 0, 0.14, 0, 0.036, 0.029, "coat-limbs", CF)
    ell(f"{nm}-hind-fetlock", f"pastern-h{s}", "body", 0, zb, tn, 0.0, -0.004, 0.05, 0.046, "coat-limbs", CF)
    wpath(f"{nm}-hind-hoof", f"hoof-h{s}", "body", 0, zb, tn * 0.45, "M -0.052 0.004 C -0.022 0.02 -0.002 0.022 0.012 0.012 L 0.0578 -0.069 L -0.0622 -0.069 C -0.064 -0.04 -0.06 -0.018 -0.052 0.004 Z", "hooves", HH, rel=True)
    wpath(f"{nm}-scapula", f"scapula-f{s}", "body", 0, zb, tn, "M 0.78 0.22 C 0.88 0.34 1.04 0.30 1.14 0.14 C 1.22 0.02 1.29 -0.1 1.27 -0.19 C 1.2 -0.28 1.05 -0.3 0.98 -0.25 C 0.9 -0.15 0.76 0.05 0.78 0.22 Z", "coat-limbs", CF)
    wpath(f"{nm}-humerus", f"humerus-f{s}", "body", 0, zb, tn, "M 1.24 -0.2 C 1.22 -0.36 1.12 -0.46 1.04 -0.5 C 0.98 -0.55 0.92 -0.52 0.89 -0.46 C 0.87 -0.38 0.92 -0.28 0.96 -0.2 C 1.0 -0.14 1.1 -0.12 1.17 -0.14 Z", "coat-limbs", CF)
    wpath(f"{nm}-forearm", f"radius-f{s}", "body", 0, zb, tn, "M 1.07 -0.40 C 1.09 -0.52 1.06 -0.70 1.03 -0.82 L 0.93 -0.84 C 0.93 -0.7 0.89 -0.55 0.89 -0.44 C 0.93 -0.40 1.02 -0.37 1.07 -0.40 Z", "coat-limbs", CF)
    lpath(f"{nm}-fore-cannon", f"cannon-f{s}", "body", 0, zb, tn, "M -0.05 0.06 L 0.04 0.062 L 0.08 0.036 L 0.27 0.03 C 0.29 0.04 0.31 0.04 0.32 0.03 L 0.32 -0.035 C 0.30 -0.05 0.27 -0.04 0.25 -0.032 L 0.08 -0.036 L 0.04 -0.06 L -0.05 -0.06 Z", "coat-limbs", CF)
    cap(f"{nm}-fore-pastern", f"pastern-f{s}", "body", 0, zb, tn, -0.015, 0, 0.13, 0, 0.038, 0.03, "coat-limbs", CF)
    ell(f"{nm}-fore-fetlock", f"pastern-f{s}", "body", 0, zb, tn, 0.0, -0.004, 0.05, 0.046, "coat-limbs", CF)
    wpath(f"{nm}-fore-hoof", f"hoof-f{s}", "body", 0, zb, tn * 0.45, "M -0.055 0.004 C -0.025 0.022 0.0 0.024 0.016 0.014 L 0.0585 -0.0749 L -0.0715 -0.0749 C -0.073 -0.045 -0.067 -0.02 -0.055 0.004 Z", "hooves", HH, rel=True)

# ---- near-side limb details
for s, nm in (("n", "near"),):
    z = 5.0
    poly("near-hoof-wall-hind", "hoof-hn", "texture", 3, z + 0.2, 0.25, [[-0.02, 0.012], [0.03, -0.045]], None, None)
    poly("near-hoof-wall-fore", "hoof-fn", "texture", 3, z + 0.2, 0.25, [[-0.025, 0.014], [0.03, -0.052]], None, None)
    poly("near-coronet-hind", "hoof-hn", "detail", 2, z + 0.2, 0.2, [[-0.05, 0.004], [-0.02, 0.018], [0.012, 0.012]], None, None)
    poly("near-coronet-fore", "hoof-fn", "detail", 2, z + 0.2, 0.2, [[-0.055, 0.004], [-0.025, 0.022], [0.016, 0.014]], None, None)
    poly("near-knee-line", "radius-fn", "detail", 3, z + 0.5, 0.25, [[0.34, 0.075], [0.4, 0.04], [0.4, -0.05]], None, None)
    poly("near-hock-line", "shank-hn", "detail", 3, z + 0.5, 0.25, [[0.50, 0.05], [0.52, 0.0], [0.5, -0.06]], None, None)
    poly("near-fetlock-tuft", "pastern-hn", "texture", 3, z + 0.2, 0.3, [[-0.03, -0.045], [-0.01, -0.06], [0.02, -0.05]], None, None)
    poly("near-fetlock-tuft-fore", "pastern-fn", "texture", 3, z + 0.2, 0.3, [[-0.03, -0.045], [-0.01, -0.06], [0.02, -0.05]], None, None)
    poly("elbow-line", "humerus-fn", "detail", 3, z + 0.5, 0.25, [[0.08, 0.1], [0.2, 0.075], [0.3, 0.06]], None, None)
    poly("shoulder-line", "scapula-fn", "detail", 2, z + 0.5, 0.25, [[0.1, 0.14], [0.3, 0.1], [0.5, 0.04]], None, None)
    poly("stifle-line", "thigh-hn", "detail", 2, z + 0.5, 0.25, [[0.26, -0.2], [0.34, -0.12], [0.44, -0.12]], None, None)
# ---- trunk
wpath("rump", "pelvis", "body", 0, 3.0, NEAR, "M -0.30 0.10 C -0.31 0.24 -0.20 0.32 -0.05 0.325 C 0.06 0.33 0.14 0.315 0.22 0.30 C 0.2 0.1 0.24 -0.2 0.2 -0.36 C 0.1 -0.42 0.0 -0.42 -0.1 -0.42 C -0.22 -0.38 -0.30 -0.2 -0.30 0.10 Z", "coat-trunk", CF)
wpath("loin", "lumbar", "body", 0, 3.1, NEAR, "M 0.19 0.30 C 0.28 0.29 0.40 0.275 0.51 0.275 C 0.55 0.1 0.46 -0.2 0.51 -0.47 C 0.40 -0.46 0.30 -0.42 0.19 -0.39 C 0.15 -0.1 0.22 0.1 0.19 0.30 Z", "coat-trunk", CF)
wpath("barrel", "thorax", "body", 0, 3.2, NEAR, "M 0.49 0.275 C 0.65 0.28 0.80 0.325 0.96 0.325 C 1.08 0.325 1.16 0.29 1.22 0.22 C 1.30 0.12 1.32 -0.06 1.28 -0.20 C 1.24 -0.34 1.18 -0.44 1.06 -0.50 C 0.92 -0.55 0.68 -0.53 0.49 -0.46 C 0.46 -0.2 0.53 0.1 0.49 0.275 Z", "coat-trunk", CF)
poly("hip-line", "pelvis", "detail", 2, 5.5, 0.25, [[0.12, 0.28], [0.06, 0.1], [0.1, -0.1]], None, None)
poly("flank-fold", "lumbar", "detail", 3, 5.5, 0.25, [[0.0, -0.3], [0.12, -0.28], [0.26, -0.34]], None, None)
poly("croup-line", "pelvis", "detail", 3, 5.5, 0.25, [[-0.27, 0.2], [-0.1, 0.1], [0.04, 0.0]], None, None)
lpath("rib-lines", "thorax", "texture", 2, 5.5, 0.35, "M 0.30 -0.05 C 0.36 -0.16 0.34 -0.3 0.28 -0.4 M 0.42 -0.05 C 0.48 -0.18 0.46 -0.32 0.4 -0.44 M 0.54 0.0 C 0.6 -0.12 0.6 -0.26 0.54 -0.4", None, None, filled=False)
lpath("withers-line", "thorax", "texture", 2, 5.5, 0.35, "M 0.35 0.26 C 0.46 0.22 0.56 0.2 0.64 0.12", None, None, filled=False)
lpath("flank-hair", "lumbar", "texture", 2, 5.5, 0.35, "M 0.05 0.1 L 0.16 0.08 M 0.14 -0.1 L 0.25 -0.12 M 0.25 0.02 L 0.36 0.0", None, None, filled=False)
# ---- neck, mane, head
lpath("neck-base", "neck-1", "body", 0, 3.3, NEAR, "M -0.02 0.265 C 0.08 0.25 0.20 0.21 0.34 0.18 C 0.375 0.1 0.375 -0.08 0.34 -0.16 C 0.25 -0.17 0.12 -0.22 0.0 -0.30 C -0.06 -0.30 -0.10 -0.25 -0.08 -0.17 C -0.1 0 -0.1 0.15 -0.02 0.265 Z", "coat-neck", CF)
lpath("neck-mid", "neck-2", "body", 0, 3.3, NEAR, "M -0.06 0.18 C 0.08 0.18 0.19 0.16 0.29 0.15 C 0.335 0.1 0.335 -0.07 0.29 -0.12 C 0.19 -0.125 0.08 -0.145 -0.06 -0.16 Z", "coat-neck", CF)
lpath("neck-top", "neck-3", "body", 0, 3.3, NEAR, "M -0.06 0.155 C 0.07 0.145 0.15 0.13 0.23 0.12 C 0.26 0.06 0.26 -0.04 0.23 -0.1 C 0.15 -0.11 0.07 -0.12 -0.06 -0.13 Z", "coat-neck", CF)
lpath("mane-1", "neck-1", "detail", 1, 3.5, 0.3, "M -0.04 0.25 C 0.06 0.30 0.2 0.27 0.34 0.21 L 0.34 0.14 C 0.2 0.19 0.08 0.2 -0.02 0.19 Z", "mane", MH)
lpath("mane-2", "neck-2", "detail", 1, 3.5, 0.3, "M 0.0 0.21 C 0.1 0.215 0.2 0.19 0.29 0.18 L 0.29 0.12 C 0.2 0.13 0.1 0.14 0.0 0.14 Z", "mane", MH)
lpath("mane-3", "neck-3", "detail", 1, 3.5, 0.3, "M 0.0 0.18 C 0.08 0.17 0.16 0.16 0.24 0.15 L 0.24 0.1 C 0.16 0.11 0.08 0.12 0.0 0.12 Z", "mane", MH)
lpath("mane-strands", "neck-1", "texture", 2, 3.6, 0.2, "M 0.0 0.25 L 0.02 0.12 M 0.1 0.26 L 0.12 0.12 M 0.2 0.24 L 0.22 0.1 M 0.3 0.2 L 0.31 0.08", None, MH, filled=False)
lpath("mane-strands-2", "neck-2", "texture", 2, 3.6, 0.2, "M 0.04 0.2 L 0.06 0.1 M 0.14 0.19 L 0.15 0.08 M 0.24 0.17 L 0.25 0.06", None, MH, filled=False)
lpath("jugular-groove", "neck-1", "detail", 3, 5.5, 0.3, "M 0.02 -0.12 C 0.12 -0.08 0.22 -0.06 0.33 -0.05", None, None, filled=False)
lpath("head", "head", "body", 0, 4.0, NEAR, "M -0.04 0.06 C 0.0 0.11 0.10 0.125 0.18 0.11 C 0.30 0.09 0.42 0.08 0.515 0.07 C 0.57 0.065 0.60 0.04 0.60 0.0 C 0.60 -0.02 0.59 -0.03 0.57 -0.035 L 0.40 -0.06 L 0.24 -0.07 C 0.17 -0.07 0.12 -0.09 0.06 -0.12 C 0.0 -0.14 -0.05 -0.1 -0.06 -0.03 C -0.065 0.01 -0.06 0.04 -0.04 0.06 Z", "coat-head", CF)
lpath("jaw", "jaw", "body", 0, 3.9, NEAR, "M -0.10 0.12 C -0.16 0.04 -0.15 -0.07 -0.07 -0.13 C 0.0 -0.14 0.08 -0.1 0.2 -0.065 L 0.35 -0.03 C 0.39 -0.025 0.41 0.0 0.40 0.02 L 0.40 0.045 C 0.3 0.05 0.2 0.05 0.1 0.055 L 0.0 0.07 L -0.1 0.12 Z", "coat-head", CF)
lpath("muzzle", "head", "body", 0, 4.2, 0.3, "M 0.5 0.07 C 0.55 0.067 0.595 0.045 0.6 0.0 C 0.6 -0.02 0.59 -0.03 0.57 -0.035 L 0.49 -0.05 C 0.5 -0.02 0.5 0.03 0.5 0.07 Z", "muzzle-skin", BS)
ell("nostril", "head", "detail", 1, 4.3, 0.05, 0.57, 0.035, 0.016, 0.008, "muzzle-skin", BS, rot=-20)
ell("eye", "head", "detail", 1, 5.0, 0.04, 0.17, 0.03, 0.016, 0.011, "eyes", EY)
lpath("brow", "head", "detail", 2, 5.0, 0.2, "M 0.14 0.052 C 0.17 0.06 0.2 0.056 0.22 0.045", None, None, filled=False)
lpath("cheek-line", "head", "detail", 2, 5.0, 0.25, "M 0.06 -0.01 C 0.12 -0.06 0.2 -0.1 0.24 -0.06", None, None, filled=False)
lpath("mouth-line", "head", "detail", 2, 5.0, 0.2, "M 0.575 -0.032 L 0.4 -0.05 L 0.28 -0.052", None, None, filled=False)
lpath("nose-bridge", "head", "texture", 3, 5.0, 0.35, "M 0.22 0.1 L 0.46 0.068", None, None, filled=False)
lpath("forelock", "head", "detail", 1, 4.4, 0.3, "M -0.04 0.06 C 0.0 0.14 0.08 0.17 0.14 0.15 C 0.12 0.12 0.1 0.09 0.1 0.06 Z", "mane", MH)
lpath("halter-noseband", "head", "detail", 2, 4.6, 0.12, "M 0.38 0.082 L 0.42 0.078 L 0.43 -0.075 L 0.39 -0.07 Z", None, "halter-webbing")
lpath("halter-cheek", "head", "detail", 2, 4.6, 0.12, "M 0.39 0.08 L 0.12 0.06 L 0.1 0.04 L 0.39 0.06 Z", None, "halter-webbing")
lpath("halter-crown", "head", "detail", 2, 4.6, 0.12, "M 0.12 0.06 L 0.1 0.04 L 0.04 -0.1 L 0.07 -0.1 Z", None, "halter-webbing")
ell("halter-ring-shape", "head", "detail", 3, 4.7, 0.5, 0.36, -0.07, 0.012, 0.012, None, "halter-webbing")
lpath("near-ear", "ear-n", "body", 0, 4.5, 0.6, "M -0.01 0.025 C 0.04 0.045 0.1 0.03 0.155 0.0 C 0.1 -0.03 0.04 -0.04 -0.01 -0.025 Z", "ears", CF)
lpath("far-ear", "ear-f", "body", 0, 2.0, FAR, "M -0.01 0.025 C 0.04 0.045 0.1 0.03 0.155 0.0 C 0.1 -0.03 0.04 -0.04 -0.01 -0.025 Z", "ears", CF)
lpath("ear-inner", "ear-n", "detail", 2, 4.6, 0.3, "M 0.02 0.0 L 0.12 0.0", None, None, filled=False)
# ---- tail
cap("tail-dock", "tail-1", "body", 0, 2.6, 0.32, 0.0, 0.0, 0.22, 0.0, 0.045, 0.06, "tail", MH)
cap("tail-upper", "tail-2", "detail", 1, 2.6, 0.3, 0.0, 0.0, 0.22, 0.0, 0.06, 0.07, "tail", MH)
cap("tail-middle", "tail-3", "detail", 1, 2.6, 0.3, 0.0, 0.0, 0.24, 0.0, 0.07, 0.065, "tail", MH)
lpath("tail-plume", "tail-4", "detail", 1, 2.6, 0.3, "M 0 0.065 C 0.1 0.075 0.25 0.07 0.34 0.04 C 0.42 0.0 0.40 -0.03 0.34 -0.045 C 0.25 -0.07 0.1 -0.075 0 -0.065 Z", "tail", MH)
lpath("tail-strands", "tail-3", "texture", 2, 2.9, 0.2, "M 0.02 0.04 L 0.3 0.05 M 0.04 -0.02 L 0.34 0.0 M 0.02 -0.05 L 0.3 -0.05", None, MH, filled=False)
lpath("tail-strands-2", "tail-4", "texture", 2, 2.9, 0.2, "M 0.02 0.04 L 0.38 0.02 M 0.02 -0.03 L 0.36 -0.02", None, MH, filled=False)
out = '      "geometry": {\n        "elevation": [\n' + ",\n".join("          " + json.dumps(d, ensure_ascii=False) for d in S) + "\n        ]\n      },\n"
open("30-geometry.json", "w").write(out)
print(len(S), "shapes")
