# [scratch] emits pigeon/30-geometry.json from authored shape definitions (near/far limb shapes share one template)
import json, sys
A = sys.argv[1]
S = []
def sh(id, bone, role, lod, z, tone, kind, region=None, material=None, filled=True, **g):
    d = {"id": id, "bone": bone, "role": role, "lod": lod, "z": z, "filled": filled, "tone": tone}
    if region: d["region"] = region
    if material: d["material"] = material
    d["kind"] = kind; d.update(g); S.append(d)
def path(id, bone, role, lod, z, tone, d, region=None, material=None): sh(id, bone, role, lod, z, tone, "path", region, material, d=d)
def cap(id, bone, role, lod, z, tone, x0, y0, x1, y1, r0, r1, region=None, material=None): sh(id, bone, role, lod, z, tone, "capsule", region, material, x0=x0, y0=y0, x1=x1, y1=y1, r0=r0, r1=r1)
def ell(id, bone, role, lod, z, tone, cx, cy, rx, ry, region=None, material=None): sh(id, bone, role, lod, z, tone, "ellipse", region, material, cx=cx, cy=cy, rx=rx, ry=ry)
def line(id, bone, lod, z, tone, pts, closed=False, filled=False, role="texture", region=None, material=None): sh(id, bone, role, lod, z, tone, "poly", region, material, filled=filled, closed=closed, points=pts)
PB, PH, PW, PT = ("plumage-body", "plumage-neck-head", "plumage-wings", "plumage-tail")
for s, nm, z0, tp, tb in (("f", "far", 1.0, 0.5, 0.36), ("n", "near", 4.0, 0.68, 0.5)):
    path(f"{nm}-trousers", f"shank-{s}", "body", 0, z0 + 0.2, tp, "M -0.012 0.017 C 0.01 0.021 0.028 0.016 0.04 0.008 L 0.044 0.0 L 0.04 -0.008 C 0.028 -0.015 0.01 -0.02 -0.012 -0.017 C -0.017 -0.008 -0.017 0.008 -0.012 0.017 Z", PB, "plumage")
    cap(f"{nm}-thigh", f"thigh-{s}", "body", 0, z0 + 0.1, tp, 0.0, 0.0, 0.045, 0.0, 0.02, 0.017, PB, "plumage")
    cap(f"{nm}-shank-bare", f"shank-{s}", "body", 0, z0 + 0.3, tb, 0.038, 0.0, 0.057, 0.0, 0.0054, 0.0048, "legs-feet", "bare-skin")
    cap(f"{nm}-tarsus", f"tarsus-{s}", "body", 0, z0 + 0.3, tb, 0.0, 0.0, 0.028, 0.0, 0.0052, 0.0044, "legs-feet", "bare-skin")
    cap(f"{nm}-toe-main", f"toes-{s}", "body", 0, z0 + 0.3, tb, 0.0, -0.0015, 0.034, -0.0015, 0.003, 0.0022, "legs-feet", "bare-skin")
    cap(f"{nm}-toe-side", f"toes-{s}", "body", 1, z0 + 0.25, tb - 0.06, 0.0, 0.0, 0.027, -0.0014, 0.0028, 0.002, "legs-feet", "bare-skin")
    cap(f"{nm}-hallux", f"toes-{s}", "body", 1, z0 + 0.25, tb, -0.001, 0.0005, -0.014, 0.0035, 0.0026, 0.0018, "legs-feet", "bare-skin")
    line(f"{nm}-claw", f"toes-{s}", 3, z0 + 0.4, 0.1, [[0.034, -0.0005], [0.0408, -0.0042], [0.034, -0.0043]], True, True, "detail", "legs-feet", "claw-horn")
    line(f"{nm}-tarsus-scales", f"tarsus-{s}", 3, z0 + 0.4, 0.3, [[0.007, 0.0045], [0.007, -0.0045], [0.014, 0.0042], [0.014, -0.0042], [0.021, 0.0038], [0.021, -0.0038]], False, False, "texture", "legs-feet", "bare-skin")
    zw = 1.1 if s == "f" else 5.0
    tw = 0.55 if s == "f" else 0.76
    path(f"{nm}-scapulars", f"humerus-{s}", "body", 0, zw, tw - 0.06, "M -0.012 0 C 0.008 0.022 0.045 0.024 0.066 0.012 C 0.07 0.005 0.07 -0.005 0.066 -0.012 C 0.045 -0.024 0.008 -0.022 -0.012 0 Z", PW, "plumage")
    path(f"{nm}-coverts", f"forearm-{s}", "body", 0, zw + 0.1, tw, "M -0.01 0 C 0.012 0.036 0.05 0.04 0.075 0.022 C 0.082 0.012 0.085 0.004 0.086 0 C 0.085 -0.004 0.082 -0.012 0.075 -0.022 C 0.05 -0.04 0.012 -0.036 -0.01 0 Z", PW, "plumage")
    line(f"{nm}-wing-bar-1", f"forearm-{s}", 1, zw + 0.2, 0.2 if s == "n" else 0.18, [[0.027, 0.026], [0.034, 0.026], [0.034, -0.026], [0.027, -0.026]], True, True, "detail", PW, "plumage")
    line(f"{nm}-wing-bar-2", f"forearm-{s}", 1, zw + 0.2, 0.2 if s == "n" else 0.18, [[0.05, 0.02], [0.057, 0.02], [0.057, -0.02], [0.05, -0.02]], True, True, "detail", PW, "plumage")
    path(f"{nm}-primaries", f"hand-{s}", "body", 0, zw + 0.3, tw - 0.18, "M -0.012 0 C 0.015 0.026 0.07 0.03 0.12 0.016 C 0.14 0.01 0.16 0.004 0.172 0 C 0.16 -0.004 0.14 -0.01 0.12 -0.016 C 0.07 -0.03 0.015 -0.026 -0.012 0 Z", PW, "plumage")
    path(f"{nm}-primary-tips", f"hand-{s}", "detail", 1, zw + 0.4, 0.2, "M 0.12 0.016 C 0.14 0.01 0.16 0.004 0.172 0 C 0.16 -0.004 0.14 -0.01 0.12 -0.016 C 0.13 -0.005 0.13 0.005 0.12 0.016 Z", PW, "plumage")
    line(f"{nm}-primary-lines", f"hand-{s}", 2, zw + 0.5, 0.2, [[0.03, 0.012], [0.16, 0.004], [0.04, 0.0], [0.17, 0.0], [0.03, -0.012], [0.16, -0.004]], False, False, "texture", PW, "plumage")
    line(f"{nm}-covert-lines", f"forearm-{s}", 2, zw + 0.5, 0.35, [[0.0, 0.012], [0.06, 0.02], [0.0, -0.012], [0.06, -0.02], [0.0, 0.0], [0.07, 0.0]], False, False, "texture", PW, "plumage")
path("tail-fan-far", "tail-f", "body", 0, 2.5, 0.5, "M 0 0.01 L 0.1 0.02 C 0.108 0.013 0.108 0.005 0.105 0 L 0 -0.008 Z", PT, "plumage")
path("tail-feathers", "tail", "body", 0, 2.8, 0.58, "M 0 0.018 C 0.03 0.021 0.07 0.02 0.1 0.017 C 0.108 0.01 0.108 -0.01 0.1 -0.017 C 0.07 -0.02 0.03 -0.021 0 -0.018 Z", PT, "plumage")
line("tail-band", "tail", 1, 2.9, 0.18, [[0.075, 0.0195], [0.1, 0.017], [0.107, 0.0], [0.1, -0.017], [0.075, -0.0195]], True, True, "detail", PT, "plumage")
path("body", "trunk", "body", 0, 3.0, 0.72, "M -0.035 0.0 C -0.03 0.03 0.0 0.05 0.04 0.058 C 0.09 0.064 0.14 0.06 0.17 0.045 C 0.2 0.028 0.208 -0.01 0.19 -0.035 C 0.165 -0.062 0.115 -0.07 0.07 -0.066 C 0.02 -0.062 -0.025 -0.04 -0.035 0 Z", PB, "plumage")
path("breast-sheen", "trunk", "detail", 2, 3.1, 0.6, "M 0.12 0.04 C 0.16 0.04 0.19 0.02 0.185 -0.02 C 0.17 -0.045 0.14 -0.05 0.12 -0.04 C 0.14 -0.02 0.14 0.02 0.12 0.04 Z", PB, "plumage")
line("belly-feathers", "trunk", 2, 3.2, 0.5, [[0.03, -0.05], [0.06, -0.035], [0.08, -0.06], [0.11, -0.04], [0.13, -0.058], [0.155, -0.035]], False, False, "texture", PB, "plumage")
line("back-feathers", "trunk", 2, 3.2, 0.5, [[0.02, 0.045], [0.05, 0.03], [0.07, 0.052], [0.1, 0.035], [0.125, 0.05]], False, False, "texture", PB, "plumage")
cap("neck-1", "neck-1", "body", 0, 3.5, 0.7, 0.0, 0.0, 0.03, 0.0, 0.026, 0.021, PH, "plumage")
cap("neck-2", "neck-2", "body", 0, 3.5, 0.68, 0.0, 0.0, 0.025, 0.0, 0.021, 0.017, PH, "plumage")
cap("neck-3", "neck-3", "body", 0, 3.5, 0.66, 0.0, 0.0, 0.02, 0.0, 0.017, 0.0145, PH, "plumage")
ell("neck-sheen", "neck-2", "detail", 2, 3.6, 0.42, 0.012, -0.002, 0.012, 0.011, PH, "plumage")
path("skull", "head", "body", 0, 3.7, 0.64, "M -0.007 0.004 C -0.005 0.017 0.008 0.022 0.02 0.021 C 0.03 0.02 0.036 0.015 0.038 0.008 L 0.039 0.002 C 0.036 -0.006 0.03 -0.012 0.02 -0.014 C 0.008 -0.016 -0.005 -0.012 -0.007 -0.004 Z", PH, "plumage")
path("beak-upper", "head", "body", 0, 3.8, 0.34, "M 0.033 0.011 C 0.04 0.012 0.048 0.007 0.054 -0.002 C 0.05 -0.005 0.044 -0.006 0.034 -0.006 Z", "beak", "beak-keratin")
path("beak-lower", "jaw", "body", 0, 3.75, 0.38, "M -0.002 0.0 L 0.02 0.0 C 0.014 -0.0035 0.007 -0.0045 -0.002 -0.0045 Z", "beak", "beak-keratin")
ell("cere", "head", "detail", 0, 3.9, 0.93, 0.0375, 0.0064, 0.0068, 0.0043, "cere", "cere-wax")
line("nostril-slit", "head", 3, 4.0, 0.08, [[0.035, 0.0058], [0.0405, 0.0062]], False, False, "detail", "cere", "cere-wax")
ell("eye-ring", "head", "detail", 1, 3.95, 0.82, 0.024, 0.007, 0.0062, 0.0056, "eye", "eye-tissue")
ell("iris", "head", "detail", 1, 4.0, 0.4, 0.0245, 0.007, 0.0044, 0.0044, "eye", "eye-tissue")
ell("pupil", "head", "detail", 1, 4.1, 0.02, 0.0246, 0.007, 0.0022, 0.0022, "eye", "eye-tissue")
out = '      "geometry": {\n        "elevation": [\n' + ",\n".join("          " + json.dumps(d, ensure_ascii=False) for d in S) + "\n        ]\n      },\n"
open(A + "/pigeon/30-geometry.json", "w").write(out)
print(len(S), "shapes")
