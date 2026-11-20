"""✏️ Round 3 2D of humans/child-on-kick-scooter (ruling E1): every shape of every scale and view is redrawn as the orthographic view of that scale's handcrafted body, per bone, with the ids, roles, bone bindings, layer order, tones, regions and pens the drawing had.

Filled shapes take the projected outline of their solids (`r3-child-on-kick-scooter-project2d.ts`); strokes follow the section centres of the detail they depict or are stated here by hand; plan limbs stay stadiums that follow the bone tips. Reads the candidate document, rewrites its `representations[*].geometry` in place.
Usage: python3 r3-child-on-kick-scooter-draw2d.py
"""
import copy, importlib.util, json, math, os, subprocess, glob

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-child-on-kick-scooter")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-child-on-kick-scooter-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
FK = mod("fk")
VOL = mod("volumes")
COMMON = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
GRIPS_AREA = 0.027
TEXTS = [
    ("50", "silhouette", "en", "mitten hands with thumbs", "hands closed round the grips with fingers and thumbs"),
    ("50", "silhouette", "de", "Fäustlingshände mit Daumen", "um die Griffe geschlossene Hände mit Fingern und Daumen"),
    ("50", "silhouette", "en", "Every limb keeps its bone binding (plan: stadiums stretched along the bone; front: outlines stretched between the heights of bone origin and tip), so strides", "Every shape is the orthographic outline of the handcrafted 1:50 body in the frame of its bone; thighs, shanks, shoes and toe boxes are solids swept from their side and lateral outlines through their real, tapering depth, the arms keep stadiums in plan, so strides"),
    ("50", "silhouette", "de", "Jedes Glied behält seine Knochenbindung (Grundriss: entlang des Knochens gestreckte Stadien; Vorderansicht: zwischen den Höhen von Knochenursprung und Spitze gestreckte Umrisse); Schritte", "Jede Form ist der orthografische Umriss des von Hand modellierten 1:50-Körpers im Rahmen ihres Knochens; Oberschenkel, Unterschenkel, Schuhe und Zehenkappen sind aus Seiten- und Querumriss durch ihre wahre, sich verjüngende Tiefe gezogene Körper, die Arme bleiben im Grundriss Stadien; Schritte"),
    ("100", "silhouette", "en", "Limb shapes follow their bones by travel only (plan: stadiums stretched from bone origin to tip; front: outlines stretched between origin and tip height), so every pose and clip reads correctly.", "Every shape is the outline of the 1:100 body in the frame of its bone; the leg segments and shoes are solids swept from their side and lateral outlines through their tapering depth, the arms keep stadiums in plan, so every pose and clip reads correctly."),
    ("100", "silhouette", "de", "Gliedformen folgen ihren Knochen nur über den Weg (Grundriss: von Knochenursprung zu Spitze gestreckte Stadien; Vorderansicht: zwischen Ursprungs- und Spitzenhöhe gestreckte Umrisse); so lesen sich alle Haltungen und Clips richtig.", "Jede Form ist der Umriss des 1:100-Körpers im Rahmen ihres Knochens; die Beinsegmente und Schuhe sind aus Seiten- und Querumriss durch ihre sich verjüngende Tiefe gezogene Körper, die Arme bleiben im Grundriss Stadien; so lesen sich alle Haltungen und Clips richtig."),
    ("200", "silhouette", "en", "The shapes sit on the same bones as the side view, so strides stretch the limbs in plan and bending knees rise in front.", "The shapes sit on the same bones as the side view and are the outlines of the 1:200 body; the leg segments and shoes are swept solids, so strides and bending knees read in every view."),
    ("200", "silhouette", "de", "Die Formen liegen an denselben Knochen wie die Seitenansicht; Schritte strecken die Glieder im Grundriss, gebeugte Knie heben sich in der Vorderansicht.", "Die Formen liegen an denselben Knochen wie die Seitenansicht und sind die Umrisse des 1:200-Körpers; die Beinsegmente und Schuhe sind gezogene Körper, Schritte und gebeugte Knie lesen sich daher in jeder Ansicht."),
    ("500", "retained", "en", "the plan keeps one stadium per limb segment with the stride length of the legs.", "the plan keeps one stadium per arm segment and one swept outline per leg segment and shoe."),
    ("500", "retained", "de", "der Grundriss behält je Gliedsegment ein Stadion mit der Schrittlänge der Beine.", "der Grundriss behält je Armsegment ein Stadion und je Beinsegment und Schuh einen gezogenen Umriss."),
    ("500", "omitted", "en", "hands and toes are single blunt stadiums.", "hands are single blunt stadiums and the toe box is part of the shoe (its bend is 0.1 mm on paper)."),
    ("500", "omitted", "de", "Hände und Zehen sind einfache stumpfe Stadien.", "Hände sind einfache stumpfe Stadien, und die Zehenkappe gehört zum Schuh (ihre Beugung misst 0,1 mm auf dem Papier)."),
]
DIMENSIONS = {"heightM": 1.344, "widthM": 0.705, "depthM": 0.344}
ELEV = {"thigh-g-l": ["thigh-l"], "thigh-g-r": ["thigh-r"], "shank-g-l": ["shank-l"], "shank-g-r": ["shank-r"], "hip": ["hip", "seat"], "head": ["head", "nose"], "hair": ["hair-nape", "hair-tail"], "hat": ["helmet"], "backpack": ["backpack", "backpack-pocket"], "head-tube": ["head-tube", "fork-crown"], "stem": ["stem", "=clamp"], "fork": ["fork-r", "fork"], "grip": ["grip-r"], "strut": ["strut-r"], "ear": ["ear-r"], "shoe-l": ["shoe-l", "toe-l"], "shoe-r": ["shoe-r", "toe-r"], "hand-l": ["hand-l", "thumb-l"], "hand-r": ["hand-r", "thumb-r"]}
FRONT = {"foot-l": ["shoe-rear-l", "shoe-l"], "foot-r": ["shoe-rear-r", "shoe-r"], "toe-l": ["shoe-front-l", "toe-l"], "toe-r": ["shoe-front-r", "toe-r"], "upper-arm-l": ["sleeve-up-l", "upper-arm-l"], "upper-arm-r": ["sleeve-up-r", "upper-arm-r"], "forearm-l": ["sleeve-fore-l", "forearm-l"], "forearm-r": ["sleeve-fore-r", "forearm-r"], "hand-l": ["hand-l", "thumb-l"], "hand-r": ["hand-r", "thumb-r"], "pelvis": ["hip", "seat"], "torso": ["torso"], "hair-nape": ["hair-nape", "hair-tail"], "helmet-head": ["helmet"], "deck": ["deck", "strut-l", "strut-r", "fender"], "stem": ["stem", "clamp"], "wheel-front": ["tyre-front"], "wheel-rear": ["tyre-rear"], "bar-stem": ["handlebar"], "stem-column": ["stem", "fork"], "head-tube": ["head-tube", "fork-crown"]}
PLAN = {"sole-l": ["shoe-rear-l"], "sole-r": ["shoe-rear-r"], "foot-l": ["shoe-rear-l", "shoe-l"], "foot-r": ["shoe-rear-r", "shoe-r"], "toe-l": ["shoe-front-l", "toe-l"], "toe-r": ["shoe-front-r", "toe-r"], "waist": ["hip"], "shoulders": ["torso"], "neck": ["neck", "collar"], "face": ["head", "nose"], "scooter-base": ["deck", "tyre-rear"], "scooter-fork": ["fork"], "scooter-front-wheel": ["tyre-front"], "head-tube": ["head-tube", "fork-crown"], "head": ["head"]}
OVERRIDE = {"200|front|deck": ["deck"]}
LEGS = ("thigh", "shank", "foot", "toe")
ARMS = ("arm", "forearm", "hand")
LIMB_PLAN = {"thigh": (0.056, 0.045, R.HIP_Z, 0.0), "shank": (0.045, 0.029, R.HIP_Z, 0.0), "upper-arm": (0.044, 0.036, R.SHOULDER_Z, 0.0), "forearm": (0.036, 0.029, R.SHOULDER_Z, 0.0), "hand": (0.026, 0.030, R.SHOULDER_Z, -0.050)}


def keep(shape, **geometry):
    out = {}
    for k, v in geometry.items():
        if k == "kind":
            out[k] = v
    for k in COMMON:
        if k in shape:
            out[k] = shape[k]
    for k, v in geometry.items():
        if k != "kind":
            out[k] = v
    return {"kind": geometry["kind"], **{k: v for k, v in out.items() if k != "kind"}}


def hull(pts):
    """🥚 The convex hull of an outline (Andrew's monotone chain), counter-clockwise."""
    q = sorted(set(map(tuple, pts)))
    def half(seq):
        h = []
        for p in seq:
            while len(h) >= 2 and (h[-1][0] - h[-2][0]) * (p[1] - h[-2][1]) - (h[-1][1] - h[-2][1]) * (p[0] - h[-2][0]) <= 1e-12:
                h.pop()
            h.append(p)
        return h[:-1]
    return [list(p) for p in half(q) + half(q[::-1])]


def rnd(p):
    return [round(p[0], 4) + 0.0, round(p[1], 4) + 0.0]


def main():
    path = os.path.join(OUT, "🔣️.json")
    doc = json.load(open(path))
    a = doc["asset"]
    a["dimensions"].update(DIMENSIONS)
    next(r for r in a["thermal"]["regions"] if r["id"] == "grips")["areaM2"] = GRIPS_AREA
    for den, key, lang, was, now in TEXTS:
        text = a["representations"][den]["decisions"][key][lang]
        if was not in text:
            raise SystemExit(f"decision text of 1:{den} {key} {lang} changed: {was[:40]}")
        a["representations"][den]["decisions"][key][lang] = text.replace(was, now)
    groups = VOL.groups()
    rest = FK.chain(a["rig"], {}, {})
    jobs, plan = [], []
    swept = set()
    notes = []

    def solids(den, names):
        g = groups[str(den)]
        out = []
        for n in names:
            out += [n[1:]] if n.startswith("=") else g.get(n, [])
        return out

    def body(den, i):
        return next(s for s in a["volumes"][str(den)]["body"] if s["id"] == i)

    def centres(den, i, view, bone):
        s = body(den, i)
        pts = [q["at"] for q in s["sections"]] if s["kind"] == "loft" else [s["p0"], s["p1"]]
        f = rest[s["bone"]]
        out = []
        for p in pts:
            if view == "elevation":
                out.append(rnd([p[0], p[1]]))
            else:
                w = FK.pt(f, p[0], p[1], p[2])
                out.append(rnd([w[2], w[1]] if view == "front" else [w[0], w[2]]))
        return out

    def stroke(shape, pts):
        return keep(shape, kind="poly", points=[rnd(p) for p in pts], closed=False)

    def hand(den, view, sh):
        """🖐️ Shapes stated by hand: strokes, wheels, plan stadiums. Returns the new shape or None."""
        i, bone = sh["id"], sh["bone"]
        side = 1 if i.endswith(("-l", "l0", "l1", "l2")) or "-l" in i else -1
        ao = R.AO_Y
        if view == "elevation":
            if i.startswith(("tyre-", "rim-", "hub-")):
                r = {"tyre": R.WHEEL_R, "rim": 0.047 if den == 50 else 0.046, "hub": 0.014}[i.split("-")[0]]
                return keep(sh, kind="ellipse", cx=0, cy=0, rx=r, ry=r)
            if i.startswith("spoke-"):
                k = int(i[-1])
                c, s = math.cos(math.radians(60 * k)), math.sin(math.radians(60 * k))
                return stroke(sh, [(-0.038 * c, -0.038 * s), (0.038 * c, 0.038 * s)])
            if i.startswith("lace-"):
                x, y = ((0.030, 0.0055), (0.048, -0.0015), (0.066, -0.0095))[int(i[-1])]
                return stroke(sh, [R.foot_local(x - 0.007, y + 0.003), R.foot_local(x + 0.007, y - 0.003)])
            if i.startswith("cuff-s"):
                return stroke(sh, [(0.160, -0.0255), (0.160, 0.0255)])
            if i.startswith("finger-"):
                n = "l" if "-l" in i else "r"
                return stroke(sh, centres(den, f"finger-{n}{int(i[-1])}", view, bone)[1:])
            if i == "front":
                return stroke(sh, centres(den, "placket", view, bone))
            if i == "mouth":
                return stroke(sh, [(-0.036, -0.094), (-0.035, -0.080)])
            if i == "backpack-strap":
                return stroke(sh, centres(den, "backpack-strap-r", view, bone))
            if i == "backpack-pocket":
                return stroke(sh, [(0.058, 0.190), (0.066, 0.220), (0.150, 0.224), (0.186, 0.214), (0.194, 0.190)])
            if i == "brake-pad":
                return stroke(sh, [(x, y) for x, y in VOL.K.arc(0.0, R.WHEEL_R, R.FENDER_R0 - 0.0015, 118, 66, 6)])
            if den >= 200 and i == "deck":
                return stroke(sh, [(0.012, 0.071), (0.438, 0.071)] if den == 200 else [(0.012, 0.070), (0.438, 0.070)])
            if den >= 200 and i == "stem":
                return stroke(sh, [R.stem_point(R.CROWN_T - 0.012), R.stem_point(R.BAR_T + 0.012)])
            if den >= 200 and i == "fork":
                c = R.stem_point(R.CROWN_T)
                return stroke(sh, [(0.412, 0.074), (0.470, 0.125), c, (R.WB, R.WHEEL_R)] if den == 500 else [c, (R.WB, R.WHEEL_R)])
            if den == 200 and i == "fender":
                return stroke(sh, VOL.K.arc(0.0, R.WHEEL_R, (R.FENDER_R0 + R.FENDER_R1) / 2, 140, 30, 8) + [(0.092, 0.092)])
            if den == 500 and i == "arm":
                return stroke(sh, [(0.0, 0.0), (0.2286, 0.0)])
            if den == 500 and i == "forearm":
                return stroke(sh, [(0.0, 0.0), (0.1794, 0.0)])
            if den == 500 and i == "hand":
                return stroke(sh, [(0.0, -0.004), (0.072, -0.022)])
        if view == "front":
            if den == 500 and i == "stem-column":
                return {**stroke(sh, [(0.0, 0.066), (0.0, R.BAR_Y)]), "filled": False}
            if i.startswith("knee-"):
                return keep(sh, kind="ellipse", cx=side * R.HIP_Z, cy=R.KNEE_Y, rx=0.044, ry=0.044)
            if i.startswith("brow-"):
                return stroke(sh, [(side * 0.017, ao + 0.044), (side * 0.030, ao + 0.047), (side * 0.043, ao + 0.043)])
            if i == "nose":
                return stroke(sh, [(0.004, ao + 0.020), (0.011, ao - 0.014), (0.0, ao - 0.021), (-0.011, ao - 0.014)])
            if i == "mouth":
                return stroke(sh, [(-0.016, ao - 0.0355), (0.0, ao - 0.037), (0.016, ao - 0.0355)])
            if i.startswith("backpack-strap-"):
                return stroke(sh, centres(den, f"backpack-strap-{'l' if side > 0 else 'r'}", view, bone)[1:7] if den <= 100 else [(side * 0.071, R.C7_Y - 0.002), (side * 0.081, R.C7_Y - 0.054), (side * 0.096, R.C7_Y - 0.200)])
            if i == "jacket-front":
                top = rest["torso"]["o"][1]
                return stroke(sh, [(0.0, top - 0.020), (0.0, top + 0.300)])
            if i == "sternum-strap":
                y = rest["torso"]["o"][1] + 0.262
                return stroke(sh, [(-0.084, y), (0.084, y)])
            if i.startswith("helmet-strap-"):
                n = "l" if side > 0 else "r"
                return stroke(sh, centres(den, f"helmet-strap-front-{n}", view, bone) + centres(den, f"helmet-strap-chin-{n}", view, bone)[1:])
        if view == "plan":
            for k, (r0, r1, z, dx) in LIMB_PLAN.items():
                if bone.split("-")[0] in LEGS:
                    break
                if i in (f"{k}-l", f"{k}-r"):
                    ox = rest[bone]["o"][0]
                    oz = rest[bone]["o"][2]
                    if den == 500:
                        r0, r1 = max(r0, 0.0375), max(r1, 0.0375)
                    return keep(sh, kind="capsule", x0=round(ox, 4), y0=round(oz, 4), x1=round(ox + dx, 4), y1=round(oz, 4), r0=r0, r1=r1, followTip=True)
            if i == "trunk":
                return keep(sh, kind="capsule", x0=0, y0=0, x1=0, y1=0, r0=0.080, r1=0.080, followTip=True)
        return None

    out_shapes = {}
    front500 = a["representations"]["500"]["geometry"]["front"]
    if not any(sh["id"] == "wheel-rear" for sh in front500):
        k = next(i for i, sh in enumerate(front500) if sh["id"] == "wheel-front")
        front500.insert(k, {**front500[k], "id": "wheel-rear", "bone": "wheel-rear", "z": round(front500[k]["z"] - 0.05, 2)})
    for den in (50, 100, 200, 500):
        rep = a["representations"][str(den)]
        for view in ("elevation", "front", "plan"):
            alias = {"elevation": ELEV, "front": FRONT, "plan": PLAN}[view]
            for k, sh in enumerate(rep["geometry"][view]):
                key = f"{den}|{view}|{k}"
                leg = sh["bone"].split("-")[0] in LEGS or (view == "front" and sh["bone"].split("-")[0] in ARMS)
                if den == 500 and view == "plan" and sh["id"].startswith("toe-"):
                    continue
                if leg and sh["id"].startswith("knee-"):
                    out_shapes[key] = keep(sh, kind="ellipse", cx=0, cy=0, rx=0.044, ry=0.045, solid={"kind": "extruded", "sweep": "v", "z0": -0.041, "z1": 0.048})
                    continue
                if leg and (sh["filled"] or sh["id"].startswith("sole-")):
                    names = ["=" + sh["id"].replace("sole-", "sole-rear-")] if sh["id"].startswith("sole-") else alias.get(sh["id"], [sh["id"]])
                    ids = solids(den, names)
                    if not ids:
                        raise SystemExit(f"no solids for {key} {sh['id']}")
                    jobs.append({"key": key, "scale": den, "view": "elevation" if view == "elevation" else "lateral", "solids": ids, "bone": sh["bone"], "sweep": True})
                    plan.append((key, sh))
                    swept.add(key)
                    continue
                h = hand(den, view, sh)
                if h is not None:
                    out_shapes[key] = h
                    continue
                names = OVERRIDE.get(f"{den}|{view}|{sh['id']}", alias.get(sh["id"], [sh["id"]]))
                ids = solids(den, names)
                if not ids:
                    notes.append(f"NO SOLIDS {key} {sh['id']} bone {sh['bone']}")
                    out_shapes[key] = sh
                    continue
                job = {"key": key, "scale": den, "view": view, "solids": ids}
                if view == "elevation":
                    job["bone"] = sh["bone"]
                if view == "plan" and sh["id"] in ("backpack", "backpack-pocket"):
                    job["pose"] = a["presentation"]["defaultPose"]
                if view == "plan" and sh["id"] == "face":
                    job["clip"] = {"axis": 0, "min": rest["head"]["o"][0] + 0.045, "max": 9}
                if not sh["filled"]:
                    job["tol"] = 2.5
                jobs.append(job)
                plan.append((key, sh))
    json.dump(jobs, open(os.path.join(OUT, "jobs.json"), "w"))
    bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    subprocess.run([bun, os.path.join(T, "r3-child-on-kick-scooter-project2d.ts"), path, os.path.join(OUT, "jobs.json"), os.path.join(OUT, "outlines.json")], check=True, cwd=T)
    outlines = json.load(open(os.path.join(OUT, "outlines.json")))
    spans = json.load(open(os.path.join(OUT, "outlines.json.spans.json")))
    for key, sh in plan:
        pts = outlines[key]
        if key in swept:
            lo, hi, lo1, hi1 = spans[key]
            if lo is None:
                lo, hi = lo1, hi1
            if lo1 is None:
                lo1, hi1 = lo, hi
            if hi1 - lo1 < 0.3 * (hi - lo):
                lo1, hi1 = lo, hi
            if hi - lo < 0.3 * (hi1 - lo1):
                lo, hi = lo1, hi1
            lo, hi, lo1, hi1 = [round(q, 4) for q in (lo, hi, lo1, hi1)]
            if abs(lo1 - lo) < 0.002:
                lo1 = round(lo + 0.0021, 4)
            if abs(hi1 - hi) < 0.002:
                hi1 = round(hi - 0.0023, 4)
            out_shapes[key] = keep(sh, kind="poly", points=pts, closed=True, solid={"kind": "extruded", "sweep": "w" if key.split("|")[1] == "elevation" else "v", "z0": lo, "z1": hi, "zTip": [lo1, hi1]})
        elif sh["filled"]:
            out_shapes[key] = keep(sh, kind="poly", points=pts, closed=True)
        else:
            out_shapes[key] = keep(sh, kind="poly", points=pts + [pts[0]], closed=False)
    for den in (50, 100, 200, 500):
        rep = a["representations"][str(den)]
        for view in ("elevation", "front", "plan"):
            rep["geometry"][view] = [out_shapes[f"{den}|{view}|{k}"] for k in range(len(rep["geometry"][view])) if f"{den}|{view}|{k}" in out_shapes]
    json.dump(doc, open(path, "w"), ensure_ascii=False, indent=1)
    for n in notes:
        print(n)
    print("shapes", len(out_shapes), "projected", len(plan))


if __name__ == "__main__":
    main()
