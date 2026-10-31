# [scratch CV-A] pigeon: hand-authored representations (world coordinates of the standing-alert default pose)
from draw import *
ASSET = load_v1("pigeon"); DEFAULT = "standing-alert"
W = pose_world(ASSET, DEFAULT)
BODY = [(-0.09, 0.10), (-0.075, 0.15), (-0.03, 0.18), (0.04, 0.19), (0.10, 0.185), (0.14, 0.15), (0.148, 0.10), (0.12, 0.065), (0.06, 0.05), (-0.01, 0.052), (-0.06, 0.065)]
HEADP = [(0.072, 0.225), (0.078, 0.247), (0.10, 0.25), (0.125, 0.246), (0.139, 0.235), (0.143, 0.226), (0.135, 0.219), (0.12, 0.213), (0.10, 0.208), (0.085, 0.208)]
BEAK = [(0.126, 0.236), (0.153, 0.223), (0.126, 0.216)]
JAWP = [(0.127, 0.219), (0.147, 0.214), (0.128, 0.211)]
N = [(0.067, 0.165), (0.084, 0.189), (0.082, 0.214), (0.095, 0.229)]
TAIL = [(-0.068, 0.091), (-0.168, 0.055)]
HUM = [(0.043, 0.139), (-0.015, 0.123)]; FOREARM = [(-0.015, 0.123), (0.051, 0.159)]
COVERT = [(-0.02, 0.122), (0.0, 0.15), (0.05, 0.17), (0.077, 0.16), (0.066, 0.14), (0.03, 0.128)]
PRIM = [(0.055, 0.165), (0.0, 0.172), (-0.06, 0.152), (-0.125, 0.125), (-0.10, 0.12), (-0.04, 0.124), (0.03, 0.13)]
TOES = [(-0.005, 0.02), (0.04, 0.018), (0.054, 0.008), (0.048, 0.0), (-0.002, 0.0)]
THIGH = [(0.0, 0.1), (0.04, 0.079)]; SHANK = [(0.04, 0.079), (0.003, 0.038)]; TARSUS = [(0.003, 0.038), (0.008, 0.011)]
PL = ("plumage-body", "plumage-neck-head", "plumage-wings", "plumage-tail")

def legs(r, lvl):
    for s, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        dx = 0.0 if s == "n" else 0.0
        n = "far" if s == "f" else "near"
        if lvl == 200:
            r.add(f"{n}-leg", f"shank-{s}", "body", z, False, tone, poly=[(0.04, 0.079), (0.003, 0.038)], closed=False, region="legs-feet", penMm=0.15)
            r.add(f"{n}-foot", f"tarsus-{s}", "body", z, False, tone, poly=[(0.003, 0.038), (0.008, 0.011), (0.05, 0.005)], closed=False, region="legs-feet", penMm=0.15)
            continue
        fw = 1.0
        r.add(f"{n}-thigh", f"thigh-{s}", "body", z, True, tone, d=limb(THIGH[0], THIGH[1], 0.055, 0.035, 0, 0.5), region="plumage-body")
        r.add(f"{n}-shank", f"shank-{s}", "body", z, True, tone, d=limb(SHANK[0], SHANK[1], 0.026 if lvl == 100 else 0.02, 0.022 if lvl == 100 else 0.016, 0, 0.5), region="legs-feet")
        r.add(f"{n}-tarsus", f"tarsus-{s}", "body", z, True, tone, d=limb(TARSUS[0], TARSUS[1], 0.022 if lvl == 100 else 0.016, 0.02 if lvl == 100 else 0.014, 0, 0.5), region="legs-feet")
        r.add(f"{n}-toes", f"toes-{s}", "body", z, True, tone, pts=TOES, region="legs-feet")

def wings(r, lvl):
    for s, z, tone in (("f", 1.5, 0.5), ("n", 4.6, 0.6)):
        n = "far" if s == "f" else "near"
        if lvl == 200 and s == "f":
            r.add("far-wing", "hand-f", "body", z, True, tone, pts=[(x, y+0.004) for x, y in PRIM], region="plumage-wings"); continue
        if lvl >= 100 or s == "n":
            r.add(f"{n}-coverts", f"forearm-{s}", "body", z+0.1, True, tone, pts=COVERT, region="plumage-wings")
        r.add(f"{n}-primaries", f"hand-{s}", "body", z, True, tone-0.05, pts=PRIM, region="plumage-wings")
        if lvl >= 100:
            r.add(f"{n}-shoulder", f"humerus-{s}", "body", z+0.2, True, tone, d=limb(HUM[0], HUM[1], 0.045, 0.035, 0, 0.5), region="plumage-wings")

def rep(lvl):
    r = Rep(W); legs(r, lvl); wings(r, lvl)
    r.add("far-tail", "tail-f", "body", 1.0, True, 0.55, d=limb(TAIL[0], TAIL[1], 0.05, 0.05, 0, 0.5), region="plumage-tail")
    r.add("tail", "tail", "body", 2.8, True, 0.66, d=limb((TAIL[0][0], TAIL[0][1]-0.004), (TAIL[1][0], TAIL[1][1]-0.004), 0.05, 0.05, 0, 0.5), region="plumage-tail")
    r.add("body", "trunk", "body", 3.0, True, 0.72, pts=BODY, region="plumage-body")
    if lvl == 200:
        r.add("neck", "neck-2", "body", 3.3, True, 0.72, d=limb(N[0], N[3], 0.07, 0.05, 0, 0.5), region="plumage-neck-head")
        r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP + [(0.153, 0.222)], regions=["plumage-neck-head", "beak", "cere", "eye"])
        return r.shapes
    for i, b in enumerate(("neck-1", "neck-2", "neck-3")):
        ws = [(0.075, 0.065), (0.065, 0.052), (0.052, 0.046)][i]
        r.add(f"neck-{i+1}", b, "body", 3.3, True, 0.72, d=limb(N[i], N[i+1], ws[0], ws[1], 0, 0.5), region="plumage-neck-head")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP, region="plumage-neck-head")
    r.add("beak", "head", "body", 4.1, True, 0.3, pts=BEAK, region="beak")
    r.add("jaw", "jaw", "body", 3.9, True, 0.3, pts=JAWP, region="beak")
    r.ellipse("cere", "head", "body", 4.2, True, 0.9, (0.134, 0.229), 0.006, 0.0045, region="cere")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (0.119, 0.233), 0.0055 if lvl == 50 else 0.01, 0.0055 if lvl == 50 else 0.01, region="eye")
    if lvl == 50:
        r.ellipse("eye-ring", "head", "detail", 5.1, False, 0.2, (0.119, 0.233), 0.008, 0.008)
        r.add("neck-sheen", "neck-2", "detail", 5.5, False, 0.2, poly=[(0.07, 0.19), (0.085, 0.20), (0.09, 0.215)], closed=False)
        r.add("wing-bar", "forearm-n", "detail", 5.5, False, 0.25, poly=[(0.0, 0.14), (0.04, 0.158), (0.065, 0.152)], closed=False)
        for i, (a, b) in enumerate([((-0.04, 0.15), (-0.11, 0.126)), ((-0.02, 0.14), (-0.09, 0.122)), ((0.0, 0.13), (-0.06, 0.124))]):
            r.add(f"primary-{i+1}", "hand-n", "texture", 5.6, False, 0.35, poly=[a, b], closed=False)
        r.add("breast-feather-1", "trunk", "texture", 5.5, False, 0.35, poly=[(0.06, 0.12), (0.09, 0.11), (0.115, 0.13)], closed=False)
        r.add("breast-feather-2", "trunk", "texture", 5.5, False, 0.35, poly=[(0.03, 0.09), (0.07, 0.08), (0.10, 0.09)], closed=False)
        r.add("tail-band", "tail", "texture", 5.5, False, 0.35, poly=[(-0.135, 0.075), (-0.14, 0.04)], closed=False)
    return r.shapes

PENS = {200: dict(body=0.15, detail=None, texture=None, glazing=None, ground=0.2), 100: dict(body=0.2, detail=0.13, texture=None, glazing=None, ground=0.3), 50: dict(body=0.3, detail=0.15, texture=0.1, glazing=None, ground=0.4)}
if __name__ == "__main__":
    for den in (50, 100, 200):
        rp = {"pens": PENS[den], "geometry": {"elevation": rep(den)}}
        preview(ASSET, rp, DEFAULT, f"{OUT}/pigeon-{den}-zoom.png", 1000/den, zoom={50: 4, 100: 8, 200: 16}[den], pad_m=0.02)
        print(den, len(rp["geometry"]["elevation"]), [round(v, 3) for v in bbox_of(rp["geometry"]["elevation"], W)])
