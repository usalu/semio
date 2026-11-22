"""✏️ standing-adult: the three 2D views of every scale drawn from the hand-stated body of that scale: one shape per body solid (its true outline in the view, layered by depth), arms and hands as profiles with their lateral span so they follow every pose, plus the inner strokes stated here.

`redraw(asset, kit)` replaces `representations[scale].geometry` of the four scales.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


lib2d = module("rework-01-lib2d")
lists = module("rework-01-standing-adult-lists")
FEATURE_M = {"50": 0.0075, "100": 0.015, "200": 0.030, "500": 0.075}
SWEPT = ("arm-", "forearm-", "hand-", "thumb-", "index-", "middle-", "ring-", "little-")
KNIT = ("hoodie", "sleeve-", "forearm-")
DENIM = ("jeans-seat", "thigh-", "shank-", "buttock-", "kneecap-")
ORDER = {
    "elevation": [("buttock-", 1.9), ("jeans-seat", 2.0), ("thigh-", 1.0), ("kneecap-", 1.05), ("sock-", 1.01), ("shank-", 1.02), ("sole-", 1.15), ("toe-sole-", 1.15), ("shoe-toe-", 1.1), ("shoe-", 1.1), ("toe-cap-", 1.12), ("lace-", 1.2), ("neck", 3.8), ("hood-roll-", 4.05), ("hoodie", 4.0), ("hood", 3.9), ("hem-rib", 4.1), ("kangaroo-pocket", 4.1), ("collar-rib", 4.12), ("drawstring-", 4.15), ("head", 5.0), ("hair", 5.1), ("face", 5.2), ("nose", 5.25), ("chin", 5.26), ("lips", 5.3), ("eye-", 5.3), ("brow-", 5.3), ("ear-", 5.6), ("sleeve-", 0.5), ("forearm-", 0.51), ("cuff-rib-", 0.53), ("hand-", 0.7), ("thenar-", 0.71), ("hypothenar-", 0.71), ("skin-", 0.74)],
    "front": [("buttock-", 0.8), ("hood-roll-", 4.1), ("hood", 0.85), ("jeans-seat", 0.9), ("thigh-", 1.0), ("kneecap-", 1.05), ("sock-", 1.01), ("shank-", 1.02), ("sole-", 1.08), ("shoe-toe-", 1.12), ("shoe-", 1.1), ("toe-sole-", 1.14), ("toe-cap-", 1.16), ("lace-", 1.2), ("neck", 3.8), ("hoodie", 4.0), ("hem-rib", 4.05), ("kangaroo-pocket", 4.1), ("collar-rib", 4.12), ("drawstring-", 4.15), ("head", 5.0), ("hair", 5.1), ("ear-", 5.15), ("face", 5.2), ("chin", 5.22), ("nose", 5.25), ("lips", 5.3), ("eye-", 5.3), ("brow-", 5.3), ("sleeve-", 4.5), ("forearm-", 4.51), ("cuff-rib-", 4.53), ("hand-", 4.7), ("thenar-", 4.71), ("hypothenar-", 4.71), ("skin-", 4.74)],
    "plan": [("sole-", 1.0), ("toe-sole-", 1.0), ("shoe-toe-", 1.1), ("shoe-", 1.1), ("toe-cap-", 1.12), ("lace-", 1.2), ("sock-", 1.25), ("shank-", 1.3), ("kneecap-", 1.45), ("thigh-", 1.5), ("buttock-", 1.9), ("jeans-seat", 2.0), ("hand-", 2.2), ("thenar-", 2.21), ("hypothenar-", 2.21), ("skin-", 2.24), ("cuff-rib-", 2.4), ("forearm-", 2.5), ("hem-rib", 3.0), ("kangaroo-pocket", 3.5), ("hoodie", 4.0), ("drawstring-", 4.02), ("hood-roll-", 4.3), ("hood", 4.05), ("sleeve-", 4.2), ("collar-rib", 4.4), ("neck", 4.8), ("chin", 5.3), ("lips", 5.35), ("nose", 5.4), ("face", 5.5), ("eye-", 5.55), ("brow-", 5.6), ("ear-", 5.9), ("head", 6.0), ("hair", 6.1)],
}
STROKES = {
    "50": [
        ("jeans-side-seam-l", "thigh-l", "detail", "jeans", ["elevation"], [(-0.004, 0.900, 0.168), (-0.002, 0.700, 0.158), (0.000, 0.500, 0.141)]),
        ("jeans-side-seam-r", "thigh-r", "detail", "jeans", ["elevation"], [(-0.004, 0.900, -0.168), (-0.002, 0.700, -0.158), (0.000, 0.500, -0.141)]),
        ("jeans-shank-seam-l", "shank-l", "detail", "jeans", ["elevation"], [(0.000, 0.480, 0.140), (-0.004, 0.300, 0.131), (0.000, 0.118, 0.128)]),
        ("jeans-shank-seam-r", "shank-r", "detail", "jeans", ["elevation"], [(0.000, 0.480, -0.140), (-0.004, 0.300, -0.131), (0.000, 0.118, -0.128)]),
        ("jeans-fly", "pelvis", "detail", "jeans", ["front"], [(0.090, 0.900, 0.012), (0.088, 0.850, 0.012), (0.080, 0.826, 0.004)]),
        ("jeans-knee-fold-l", "thigh-l", "texture", "jeans", ["front", "elevation"], [(0.052, 0.540, 0.055), (0.058, 0.534, 0.085), (0.052, 0.540, 0.115)]),
        ("jeans-knee-fold-r", "thigh-r", "texture", "jeans", ["front", "elevation"], [(0.052, 0.540, -0.055), (0.058, 0.534, -0.085), (0.052, 0.540, -0.115)]),
        ("hood-fold", "torso", "texture", "hoodie", ["elevation"], [(-0.118, 1.455, 0.0), (-0.140, 1.420, 0.0), (-0.138, 1.360, 0.0)]),
        ("hair-strand-0", "head", "texture", "hair", ["elevation", "front"], [(0.050, 1.712, -0.030), (0.074, 1.690, -0.040), (0.082, 1.668, -0.046)]),
        ("hair-strand-1", "head", "texture", "hair", ["elevation", "front"], [(0.010, 1.720, 0.020), (-0.040, 1.706, 0.034), (-0.076, 1.672, 0.040)]),
        ("hair-strand-2", "head", "texture", "hair", ["elevation", "front"], [(-0.020, 1.640, -0.076), (-0.060, 1.612, -0.066), (-0.082, 1.580, -0.050)]),
        ("hair-parting", "head", "detail", "hair", ["plan"], [(0.070, 1.700, 0.028), (0.010, 1.722, 0.030), (-0.050, 1.706, 0.026)]),
    ],
    "100": [
        ("jeans-side-seam-l", "thigh-l", "detail", "jeans", ["elevation"], [(-0.004, 0.900, 0.168), (0.000, 0.500, 0.141)]),
        ("jeans-side-seam-r", "thigh-r", "detail", "jeans", ["elevation"], [(-0.004, 0.900, -0.168), (0.000, 0.500, -0.141)]),
        ("hair-parting", "head", "detail", "hair", ["plan"], [(0.070, 1.700, 0.028), (-0.050, 1.706, 0.026)]),
    ],
    "200": [],
    "500": [],
}


MASS_ORDER = {"neck": 6.6, "jeans-seat": 3.6, "hand-": 6.8, "shoe-": 3.7}


def depth(sid, view, scale="50"):
    if scale == "500":
        for prefix, z in MASS_ORDER.items():
            if sid.startswith(prefix):
                return z
    for prefix, z in ORDER[view]:
        if sid == prefix or sid.startswith(prefix):
            return z
    raise ValueError(f"no depth for {sid} in {view}")


def near(sid, view, z):
    if view != "elevation" or not sid.endswith("-r") and "-r-" not in sid:
        return z
    return z + (5.5 if z < 1 else 2.0)


def shapes_of(asset, kit, scale):
    views = lib2d.Views(kit.f)
    rep = asset["representations"][scale]
    pens = rep["pens"]
    hatches = {h["id"] for h in rep.get("hatches", [])}
    tolerance = FEATURE_M[scale] / 8
    out = {"elevation": [], "front": [], "plan": []}
    for solid in asset["volumes"][scale]["body"]:
        sid, bone = solid["id"], solid["bone"]
        region, _ = lists.region_of(sid) if not (sid == "head" and scale in ("200", "500")) else ("face-skin", None)
        stroke = solid.get("texture") is True
        role = solid["role"]
        if pens.get(role) is None or (scale == "500" and sid == "hair"):
            continue
        swept = bone.startswith(SWEPT)
        for view in out:
            source = ("elevation" if view == "elevation" else "lateral") if swept else view
            geometry, span = lib2d.geometry(views, solid, source, tolerance)
            shape = {"id": sid, "bone": bone, "role": role, "z": round(near(sid, view, depth(sid, view, scale)), 3), "filled": not stroke, "tone": solid["tone"], "region": region}
            if scale == "500" and sid == "head":
                del shape["region"]
                shape["regions"] = ["face-skin", "hair"]
            if not stroke and view == "elevation":
                if "knit" in hatches and sid.startswith(KNIT):
                    shape["hatch"] = "knit"
            if not stroke and view != "plan" and "denim" in hatches and sid.startswith(DENIM):
                shape["hatch"] = "denim"
            shape.update(geometry)
            if stroke:
                shape["closed"] = False
            if swept and view == "elevation":
                shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": round(span[0], 4), "z1": round(span[1], 4)}
            elif swept:
                lo, hi = lib2d.reach(solid, 1)
                shape["solid"] = {"kind": "extruded", "sweep": "v", "z0": round(lo, 4), "z1": round(hi, 4)}
            out[view].append(shape)
    for sid, bone, role, region, in_views, points in STROKES[scale]:
        if pens.get(role) is None:
            continue
        for view in in_views:
            pts = [views.point(bone, kit.f.local(bone, p), view) for p in points]
            out[view].append({"id": sid, "bone": bone, "role": role, "z": round(depth(sid.split("-")[0] if False else {"jeans": "thigh-", "hood": "hoodie", "hair": "hair"}[sid.split("-")[0]], view) + 0.06 + (2.0 if view == "elevation" and sid.endswith("-r") else 0), 3), "filled": False, "tone": 0.1, "region": region, "kind": "poly", "closed": False, "points": [[round(p[0], 4), round(p[1], 4)] for p in pts]})
    for view in out:
        out[view].sort(key=lambda s: s["z"])
    return out


TEXTS = {
    "50": {
        "silhouette": ("The three views are the true orthographic views of the 1:50 body in the relaxed pose, 34.4 mm tall, one shape per body solid: braincase, face from hairline to chin, nose, lips, chin, eyes, brows and ears under the hair cap, neck, hooded sweatshirt with hem rib, kangaroo pocket, collar rib, drawstrings, hood pouch and rolled hood edge, sleeves, forearms, rib cuffs, both hands with palm, thenar, hypothenar and fifteen phalanges each, jeans seat with gluteal masses, thighs with kneecaps, shanks, socks, sneakers with sole, upper, toe box, toe cap and lace bars. Arms and hands are profiles with their lateral span (side view) and lateral profiles with their depth (front and plan), so they follow every pose.", "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:50-Körpers in der entspannten Pose, 34,4 mm hoch, eine Form je Körper: Hirnschädel, Gesicht von der Haarlinie bis zum Kinn, Nase, Lippen, Kinn, Augen, Brauen und Ohren unter der Haarkappe, Hals, Kapuzensweatshirt mit Saumbund, Kängurutasche, Kragenbund, Kordeln, Kapuzenbeutel und gerolltem Kapuzenrand, Ärmel, Unterarme, Bündchen, beide Hände mit Handfläche, Daumen- und Kleinfingerballen und je fünfzehn Fingergliedern, Jeansgesäß mit Gesäßmuskeln, Oberschenkel mit Kniescheiben, Unterschenkel, Socken, Sneaker mit Sohle, Schaft, Zehenbox, Zehenkappe und Schnürstegen. Arme und Hände sind Profile mit ihrer seitlichen Spanne (Seitenansicht) und seitliche Profile mit ihrer Tiefe (Vorderansicht und Grundriss), sodass sie jeder Pose folgen."),
        "merged": ("Eyelets merge into the five lace bars; hair is one cap with three strand strokes; the trunk of the sweatshirt is one shape over lumbar spine and thorax.", "Ösen gehen in den fünf Schnürstegen auf; das Haar ist eine Kappe mit drei Strähnenstrichen; der Rumpf des Sweatshirts ist eine Form über Lendenwirbelsäule und Brustkorb."),
        "omitted": ("Pores, single hairs, stitching, fabric fibres, fingernails, jeans pockets and belt loops under the hem.", "Poren, einzelne Haare, Nähte, Stofffasern, Fingernägel, Jeanstaschen und Gürtelschlaufen unter dem Saum."),
        "thinElements": ("Lace bars, knee folds, hood fold and hair strands are 0.10 mm texture lines; side seams, fly and hair parting are 0.13 mm detail lines.", "Schnürstege, Kniefalten, Kapuzenfalte und Haarsträhnen sind 0,10-mm-Texturlinien; Seitennähte, Hosenschlitz und Scheitel sind 0,13-mm-Detaillinien."),
    },
    "100": {
        "silhouette": ("The three views are the true orthographic views of the 1:100 body, 17.2 mm tall, one shape per body solid: braincase, face, nose and ears under the hair cap, neck, sweatshirt with hem rib, pocket, hood pouch and rolled hood edge, sleeves, forearms, cuffs, both hands with palm, thenar and every phalanx, jeans seat with gluteal masses, thighs with kneecaps, shanks, socks, sole, upper, toe sole and toe box per foot.", "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:100-Körpers, 17,2 mm hoch, eine Form je Körper: Hirnschädel, Gesicht, Nase und Ohren unter der Haarkappe, Hals, Sweatshirt mit Saumbund, Tasche, Kapuzenbeutel und gerolltem Kapuzenrand, Ärmel, Unterarme, Bündchen, beide Hände mit Handfläche, Daumenballen und jedem Fingerglied, Jeansgesäß mit Gesäßmuskeln, Oberschenkel mit Kniescheiben, Unterschenkel, Socken, je Fuß Sohle, Schaft, Zehensohle und Zehenbox."),
        "merged": ("Lips, chin, eyes and brows merge into the face; collar rib and drawstrings into the sweatshirt; toe cap and toe upper into one toe box.", "Lippen, Kinn, Augen und Brauen gehen im Gesicht auf; Kragenbund und Kordeln im Sweatshirt; Zehenkappe und Vorderschaft in einer Zehenbox."),
        "omitted": ("Laces, folds, hair strands, the hypothenar and everything under 15 mm.", "Schnürung, Falten, Haarsträhnen, der Kleinfingerballen und alles unter 15 mm."),
        "thinElements": ("Side seams and the hair parting are 0.10 mm detail lines; nothing is thickened.", "Seitennähte und Scheitel sind 0,10-mm-Detaillinien; nichts wird verdickt."),
    },
    "200": {
        "silhouette": ("The three views are the true orthographic views of the 1:200 body, 8.6 mm tall, one mass per part: head, hair cap, neck, sweatshirt from hem to collar, hood pouch, jeans seat, and per side sleeve, forearm with cuff, a mitten hand, thigh, shank, shoe and toe box.", "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:200-Körpers, 8,6 mm hoch, eine Masse je Teil: Kopf, Haarkappe, Hals, Sweatshirt vom Saum bis zum Kragen, Kapuzenbeutel, Jeansgesäß und je Seite Ärmel, Unterarm mit Bündchen, eine Fausthandschuh-Hand, Oberschenkel, Unterschenkel, Schuh und Zehenbox."),
        "merged": ("Face, nose and ears into the head; hem rib, pocket and hood edge into the sweatshirt; palm and fingers into a mitten 32 mm thick; sole and upper into the shoe.", "Gesicht, Nase und Ohren im Kopf; Saumbund, Tasche und Kapuzenrand im Sweatshirt; Handfläche und Finger in einer Fausthandschuh-Hand von 32 mm Dicke; Sohle und Schaft im Schuh."),
        "omitted": ("Fingers, kneecaps, socks, gluteal masses as separate forms, every inner line.", "Finger, Kniescheiben, Socken, Gesäßmuskeln als eigene Formen, jede Binnenlinie."),
        "thinElements": ("None; the thinnest drawn form is the mitten hand of 32 mm.", "Keine; die dünnste gezeichnete Form ist die Fausthandschuh-Hand von 32 mm."),
    },
    "500": {
        "silhouette": ("An upright figure of 1.72 m (3.4 mm), the true orthographic views of the 1:500 body: head with hair as one mass, neck, trunk with the hood merged into the upper back, jeans seat, and per side upper arm, forearm, hand, thigh, shank and one shoe. Neck, seat, hands and shoes are drawn over their neighbours so that every visible form stays printable to 1:1000.", "Eine aufrechte Figur von 1,72 m (3,4 mm), die wahren orthogonalen Ansichten des 1:500-Körpers: Kopf mit Haar als eine Masse, Hals, Rumpf mit im oberen Rücken aufgegangener Kapuze, Jeansgesäß und je Seite Oberarm, Unterarm, Hand, Oberschenkel, Unterschenkel und ein Schuh. Hals, Gesäß, Hände und Schuhe liegen über ihren Nachbarn, damit jede sichtbare Form bis 1:1000 druckbar bleibt."),
        "merged": ("Face and hair; hood into the trunk; sole, upper and toe box into one shoe 75 mm high.", "Gesicht und Haar; Kapuze im Rumpf; Sohle, Schaft und Zehenbox in einem Schuh von 75 mm Höhe."),
        "omitted": ("Cuffs, pocket, fingers, kneecaps, socks and every inner line.", "Bündchen, Tasche, Finger, Kniescheiben, Socken und jede Binnenlinie."),
        "thinElements": ("Each hand is a mass 76 mm thick whose outer face lies where the hand of 1:50 lies, the thinnest band that still prints at the end of the valid range; nothing thinner is drawn.", "Jede Hand ist eine Masse von 76 mm Dicke, deren Außenseite dort liegt, wo die Hand bei 1:50 liegt, das dünnste Band, das am Ende des Gültigkeitsbereichs noch druckt; Dünneres wird nicht gezeichnet."),
    },
}


def describe(asset):
    for scale, texts in TEXTS.items():
        decisions = asset["representations"][scale]["decisions"]
        for key, (en, de) in texts.items():
            decisions[key] = {"en": en, "de": de}


def redraw(asset, kit):
    for scale in ("50", "100", "200", "500"):
        asset["representations"][scale]["geometry"] = shapes_of(asset, kit, scale)
    describe(asset)
