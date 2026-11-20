"""🩺 Round 4 group G11 kit (cyclist, child on kick scooter, skateboarder): reads the spliced volumes of a catalog asset, replaces hand-stated section rows of body lofts (with the nested thermal shells re-drawn from the new body at their own depth), and builds the anatomical pelvis and skull of the structure list from hand-stated landmarks. Bone-local metres: u along the bone, v sagittal (front is -v, back +v), w lateral (left +)."""
import copy
import glob
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
FLOOR = {"50": 0.0075, "100": 0.015, "200": 0.030, "500": 0.075}


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


K = module("r3-cyclist-kit")


def catalog(slug):
    return glob.glob(f"{S}/*/{slug}/🔣️.json")[0]


def load(slug):
    return json.load(open(catalog(slug), encoding="utf-8"))["asset"]["volumes"]


def save(slug, volumes):
    path = os.path.join(T, "volumes", "humans", slug + ".json")
    json.dump({"volumes": volumes}, open(path, "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
    return path


def find(entries, i):
    for e in entries:
        if e["id"] == i:
            return e
    raise KeyError(i)


def rows(solid, lo, hi, new, side=1.0):
    """✂️ Replaces the sections of a loft whose u lies in [lo, hi] by the stated rows (u, v, w outward, w radius, v radius[, exponent]); the other sections stay."""
    rot = solid["sections"][0].get("rotation", K.ACROSS)
    keep = [q for q in solid["sections"] if not lo - 1e-9 <= q["at"][0] <= hi + 1e-9]
    made = [K.section([r[0], r[1], r[2] * side], r[3], r[4], r[5] if len(r) > 5 else 2.0, rot) for r in new]
    solid["sections"] = sorted(keep + made, key=lambda q: q["at"][0])


def shells(volumes, scale, i, old):
    """🧅 Re-draws every nested thermal shell of body solid i (ids `th-<i>-<layer>` with own sections) from the new body at the depth it had under the old body, over the same stretch of the bone."""
    new = find(volumes[scale]["body"], i)
    for k, e in enumerate(volumes[scale]["thermal"]):
        if not e["id"].startswith("th-" + i + "-") or e.get("kind") != "loft" or e.get("bone") != new["bone"]:
            continue
        us = [q["at"][0] for q in e["sections"]]
        ref = min(old["sections"], key=lambda q: abs(q["at"][0] - us[len(us) // 2]))
        mid = e["sections"][len(us) // 2]
        d = max(0.0005, ref["rx"] - mid["rx"])
        part = copy.deepcopy(new)
        whole = len(e["sections"]) >= len(old["sections"])
        if not whole:
            lo, hi = min(us) - 0.012, max(us) + 0.012
            part["sections"] = [q for q in part["sections"] if lo <= q["at"][0] <= hi]
        if len(part["sections"]) < 2:
            continue
        s = K.shrink(part, d, e["id"])
        for key in ("region", "layer", "material", "tone", "role", "sectionFaces", "capStart", "capEnd"):
            if key in e:
                s[key] = e[key]
        volumes[scale]["thermal"][k] = s


def edit(volumes, scale, base, lo, hi, new, sides=(("l", 1.0), ("r", -1.0)), name="{base}-{n}"):
    """🪡 Applies one stated row replacement to the left and right solid of a limb (or to one unsided solid with sides=None) and re-draws its thermal shells."""
    for n, s in sides or ((None, 1.0),):
        i = name.format(base=base, n=n) if n else base
        solid = find(volumes[scale]["body"], i)
        old = copy.deepcopy(solid)
        rows(solid, lo, hi, new, s)
        shells(volumes, scale, i, old)


def member(solid, bone, part="member"):
    solid["member"] = bone
    solid["part"] = part
    return solid


def swap(volumes, scale, gone, made):
    """🔁 Removes the structure solids named in gone and puts the made ones where the first of them stood."""
    lst = volumes[scale]["structure"]
    at = min((k for k, e in enumerate(lst) if e["id"] in gone), default=len(lst))
    ids = {m["id"] for m in made}
    kept = [e for e in lst if e["id"] not in gone and e["id"] not in ids]
    at = min(at, len(kept))
    volumes[scale]["structure"] = kept[:at] + made + kept[at:]


def pelvis(scale, bone, mat, tone, p):
    """🦴 The bony pelvis from stated landmarks p (metres in the pelvis bone frame): `hip` half spacing of the hip joints, `asis` and `psis` (u, v, w) of the iliac spines, `tuber` (u, v, w) of the ischial tuberosity, `pubis` (u, v) of the symphysis, `prom` (u, v) of the sacral promontory, `coccyx` (u, v), `k` size factor of the bone thicknesses. 1:50 and 1:100: two iliac blades from the acetabulum to the crest (oblique, spines front-lateral and back-medial), sacrum wedge with coccyx, superior pubic rami, ischia and (1:50) ischiopubic rami around the obturator openings. 1:200: blades, sacrum and one pubic arch at 30 mm. 1:500: one basin mass."""
    f = FLOOR[scale] / 2 + 0.0002
    r = lambda x: max(x * p.get("k", 1.0), f)
    out = []
    a, q, t = p["asis"], p["psis"], p["tuber"]
    if scale == "500":
        hw = a[2] + 0.012
        out.append(member(K.stack("pelvis-bone", bone, [(t[0] + 0.01, -p["pubis"][1], t[1] + 0.012, t[2] + 0.02, 2.2), (0.0, -p["pubis"][1] * 0.8, q[1] * 0.8, hw * 0.95, 2.3), (a[0] * 0.6, -a[1] * 0.8, q[1], hw, 2.3), (a[0] + 0.03, 0.002 - a[1] * 0.3, q[1] * 0.9, hw * 0.86, 2.2)], mat, tone), bone))
        return out
    for s, n in ((1.0, "l"), (-1.0, "r")):
        crest = [(a[0] + q[0]) / 2 + 0.03 * p.get("k", 1.0), (a[1] + q[1]) / 2, s * ((a[2] + q[2]) / 2 + 0.02 * p.get("k", 1.0))]
        hint = K.unit([0.0, a[1] - q[1], s * (a[2] - q[2])])
        half = math.dist(a[1:], q[1:]) / 2
        blade = [([0.004, 0.0, s * (p["hip"] + 0.004)], r(0.030), r(0.011)), ([crest[0] * 0.34, crest[1] * 0.4, s * (p["hip"] + 0.012)], half * 0.52, r(0.008)), ([crest[0] * 0.72, crest[1] * 0.85, s * (abs(crest[2]) + 0.006)], half * 0.94, r(0.0065)), (crest, half * 0.90, r(0.0075), 2.6)]
        out.append(member(K.tube(f"ilium-{n}", bone, blade, mat, tone, hint=hint), bone))
        if scale != "200":
            out.append(member(K.tube(f"pubis-{n}", bone, [([-0.006, -0.018, s * (p["hip"] - 0.012)], r(0.012), r(0.012)), ([p["pubis"][0] * 0.6, p["pubis"][1] * 0.75, s * p["hip"] * 0.45], r(0.008), r(0.009)), ([p["pubis"][0], p["pubis"][1], s * 0.006], r(0.009), r(0.014))], mat, tone, hint=K.U), bone))
            out.append(member(K.tube(f"ischium-{n}", bone, [([-0.012, 0.012, s * (p["hip"] - 0.006)], r(0.013), r(0.013)), ([t[0] * 0.6, t[1] * 0.8, s * (t[2] + 0.008)], r(0.011), r(0.012)), ([t[0], t[1], s * t[2]], r(0.014), r(0.017))], mat, tone, hint=K.W), bone))
        if scale == "50":
            out.append(member(K.tube(f"ischiopubic-ramus-{n}", bone, [([t[0] + 0.004, t[1] - 0.004, s * (t[2] - 0.006)], r(0.008), r(0.008)), ([(t[0] + p["pubis"][0]) / 2 - 0.004, (t[1] + p["pubis"][1]) / 2, s * t[2] * 0.6], r(0.006), r(0.007)), ([p["pubis"][0] - 0.012, p["pubis"][1] + 0.006, s * 0.008], r(0.007), r(0.008))], mat, tone, hint=K.W), bone))
    if scale == "200":
        out.append(member(K.tube("pubis", bone, [([-0.006, -0.012, p["hip"] - 0.01], r(0.012), r(0.012)), ([p["pubis"][0], p["pubis"][1] + 0.006, 0.0], r(0.012), r(0.012)), ([-0.006, -0.012, -p["hip"] + 0.01], r(0.012), r(0.012))], mat, tone, hint=K.U), bone))
    pr, co = p["prom"], p["coccyx"]
    sac = [([pr[0], pr[1], 0.0], q[2] + 0.012, r(0.017), 2.4), ([pr[0] * 0.62 + co[0] * 0.38, max(q[1], pr[1] + 0.03) - 0.012, 0.0], q[2] + 0.004, r(0.014), 2.4), ([pr[0] * 0.25 + co[0] * 0.75, max(q[1], pr[1] + 0.03) - 0.004, 0.0], max(q[2] * 0.55, f), r(0.010))]
    if scale == "50":
        sac.append(([co[0], co[1], 0.0], r(0.008), r(0.006)))
    out.append(member(K.tube("sacrum", bone, sac, mat, tone, hint=K.W), bone))
    return out


def skull(scale, bone, mat, tone, h):
    """💀 The skull from stated measures h: `o` (u, v) of the cranial base under the vertex axis, `tilt` degrees of that axis from the bone towards +v, `len`, `wid`, `hei` half measures of the braincase, `face` size factor of the facial skeleton and jaw. 1:50 and 1:100: braincase, facial skeleton (maxilla with cheekbones and brow), mandible as an arch with two rami. 1:200 and 1:500: braincase and one facial wedge."""
    f = FLOOR[scale] / 2 + 0.0002
    c, s_ = math.cos(math.radians(h.get("tilt", 0.0))), math.sin(math.radians(h.get("tilt", 0.0)))
    P = lambda a, b, w=0.0: [h["o"][0] + a * c - b * s_, h["o"][1] + a * s_ + b * c, w]
    A, B = [c, s_, 0.0], [-s_, c, 0.0]
    rot = K.quat(K.W, B)
    k = h.get("face", 1.0)
    out = [member(K.ellipsoid("skull", bone, [h["hei"], h["len"], h["wid"]], P(h["hei"] + 0.002, 0.004), mat, tone, rotation=K.rot_w(h.get("tilt", 0.0)) if h.get("tilt") else None), bone)]

    def wedge(i, rws):
        secs = [K.section(P(a, (bk - fr) / 2), hw, (fr + bk) / 2, e, rot) for a, fr, bk, hw, e in rws]
        return member(K.loft(i, bone, secs, mat, tone), bone)

    front = h["len"] - 0.004
    if scale in ("200", "500"):
        out.append(wedge("facial-skeleton", [(-0.082 * k, front - 0.004, -0.030, max(0.026 * k, f), 2.0), (-0.045 * k, front + 0.002, -0.004, 0.046 * k, 2.2), (0.0, front + 0.002, 0.006, 0.060 * k, 2.3), (0.05, front - 0.002, 0.0, 0.056, 2.2)]))
        return out
    out.append(wedge("maxilla", [(-0.034 * k, front - 0.002, -0.040, 0.031 * k, 2.2), (-0.012 * k, front + 0.002, -0.026, 0.047 * k, 2.3), (0.018, front, -0.012, 0.064 * k, 2.4), (0.050, front - 0.002, -0.004, 0.058, 2.2), (0.064, front - 0.010, -0.010, 0.050, 2.0)]))
    r = lambda x: max(x, f)
    arch = [(P(-0.050 * k, -0.012, 0.047 * k), r(0.014 * k), r(0.006)), (P(-0.064 * k, -0.044, 0.041 * k), r(0.013 * k), r(0.006)), (P(-0.074 * k, -(front - 0.022), 0.022 * k), r(0.013 * k), r(0.0065)), (P(-0.076 * k, -(front - 0.010), 0.0), r(0.015 * k), r(0.007)), (P(-0.074 * k, -(front - 0.022), -0.022 * k), r(0.013 * k), r(0.0065)), (P(-0.064 * k, -0.044, -0.041 * k), r(0.013 * k), r(0.006)), (P(-0.050 * k, -0.012, -0.047 * k), r(0.014 * k), r(0.006))]
    out.append(member(K.tube("mandible", bone, arch, mat, tone, hint=A), bone))
    for s, n in ((1.0, "l"), (-1.0, "r")):
        out.append(member(K.tube(f"mandible-ramus-{n}", bone, [(P(-0.056 * k, -0.008, s * 0.047 * k), r(0.016 * k), r(0.0055)), (P(-0.026 * k, -0.006, s * 0.050 * k), r(0.014 * k), r(0.0055)), (P(0.004, -0.004, s * 0.053 * k), r(0.010 * k), r(0.0065))], mat, tone, hint=B), bone))
    return out
