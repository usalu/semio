"""📦 objects/storage-crate-stack: the front drawing of the four turning crates as extruded solids instead of flat shapes.

Crates 2 to 5 turn about the vertical when they are stacked or nested (twist 0..180 degrees, clips `unload-stack` and `load-stack`). Their front shapes were flat drawings of the longitudinal section, which stay as they are while the crate turns under them (`lateral-ambiguous`, up to 8 mm on paper). Every front shape of these crates is now a real part of the moulding, drawn as a profile in the crate frame (x = u up the bone, y = v = -world x) and swept laterally (`sweep: "w"`, z = world z of the rest pose, with `zTip` for the wall draft): the end wall beyond the section plane as its own face with the open hand hole, swept along v through the wall (`sweep: "v"`), its foot, the rim band as the whole 0.60 x 0.40 m block (the cut far skirt lies in its edge and is no shape of its own any more), the floor, both long walls, the stacking channel of the near wall with its foot and ledge, and the flanges. At rest the picture is the section that was drawn before (the full near wall is new and lies in the silhouette edge); in between the product projects the true parts.

Numbers are those of `r3-storage-crate-stack-kit.py` (wall foot outline 0.53 x 0.33 m at u = 12 mm, draft factor 1 + 0.1 / 0.258 per metre, wall 3 mm, hand hole 120 x 35 mm at u 215..250 mm, rim 0.60 x 0.40 m from u = 262 mm, channel of the -w wall 32 mm deep over v -60..60 mm). Crate 1 is fixed and keeps its flat section.

Usage: python3 rework-storage-crate-stack-front-solids.py candidate | write
"""
import sys

from rework_fittings_lib import candidate, read_asset, write_members

ASSET = "objects/storage-crate-stack"
U0, SLOPE, HV, HW, WALL, TIP = 0.012, 0.1 / 0.258, 0.265, 0.165, 0.003, 0.3
TOP, RIM0, HOLE0, HOLE1, GRIP, CHANNEL = 0.295, 0.262, 0.215, 0.25, 0.06, 0.06


def f(u):
    """📐 Draft factor of the wall outline at height u."""
    return 1 + SLOPE * (u - U0)


def r(value):
    """🔢 Rounds to 0.01 mm."""
    return round(value + 0.0, 5)


def taper(a, b):
    """↔️ Lateral span a..b of the wall foot outline carried up the draft: at the joint and at the bone tip."""
    return {"kind": "extruded", "sweep": "w", "z0": r(a * f(0)), "z1": r(b * f(0)), "zTip": [r(a * f(TIP)), r(b * f(TIP))]}


def straight(a, b):
    """⬛ Constant lateral span."""
    return {"kind": "extruded", "sweep": "w", "z0": a, "z1": b}


def end_face(hole):
    """🚪 The end wall in its own plane (x = u, y = w): a trapezoid with the draft, with the open hand hole 120 x 35 mm and its 8 mm chamfers."""
    a, b = HW, r(HW * f(RIM0))
    face = "M %s %s L %s %s L %s %s L %s %s Z" % (U0, -a, RIM0, -b, RIM0, b, U0, a)
    if not hole:
        return face
    g, c = GRIP, 0.008
    return face + " M %s %s L %s %s L %s %s L %s %s L %s %s L %s %s L %s %s L %s %s Z" % (HOLE0, -g + c, r(HOLE0 + c), -g, r(HOLE1 - c), -g, HOLE1, -g + c, HOLE1, g - c, r(HOLE1 - c), g, r(HOLE0 + c), g, HOLE0, g - c)


def box(u0, u1, v0, v1):
    """▭ Rectangular profile."""
    return [[u0, v0], [u1, v0], [u1, v1], [u0, v1]]


def long_wall(v_foot):
    """📏 Profile of a long wall over the half length v_foot (at the wall foot) on both sides of the middle, widening with the draft."""
    return [[U0, -v_foot], [TOP, r(-v_foot * f(TOP))], [TOP, r(v_foot * f(TOP))], [U0, v_foot]]


def parts(fine, hole):
    """🧩 The parts of one crate: suffix, the flat shape whose style it takes, depth offset against that shape, profile, solid."""
    wall = [("retained-wall", "retained-wall", 0, end_face(hole), {"kind": "extruded", "sweep": "v", "z0": r(HV * f(U0) - WALL), "z1": r(HV * f(RIM0))})]
    wall += [("retained-foot", "retained-wall", 0, box(0.0, 0.014, 0.256, 0.26), straight(-0.16, 0.16)),
             ("retained-rim", "retained-wall", 0, box(RIM0, TIP, -0.3, 0.3), straight(-0.2, 0.2)),
             ("near-wall", "retained-wall", -0.01, long_wall(HV), taper(-HW, -HW + WALL)),
             ("near-section", "near-section", 0, long_wall(CHANNEL), taper(-0.133, -0.13)),
             ("far-section", "far-section" if fine else "near-section", 0, long_wall(HV), taper(HW - WALL, HW))]
    if fine:
        wall += [("bottom-section", "bottom-section", 0, box(U0, 0.02, -HV, HV), straight(-0.16, 0.16)),
                 ("near-foot-section", "near-foot-section", 0, box(0.0, 0.014, -CHANNEL, CHANNEL), straight(-0.128, -0.124)),
                 ("far-foot-section", "far-foot-section", 0, box(0.0, 0.014, -0.26, 0.26), straight(0.156, 0.16)),
                 ("ledge-section", "ledge-section", 0, box(0.232, TIP, -CHANNEL, CHANNEL), straight(-0.17857, -0.148))]
    return wall


FLANGES = [("near-flange-section", box(0.292, TIP, -0.3, 0.3), straight(-0.2, -0.1805)),
           ("far-flange-section", box(0.292, TIP, -0.3, 0.3), straight(0.1805, 0.2))]
STYLE = ("role", "filled", "tone", "region", "material", "penMm", "hatch")
SENTENCE = {
    "en": " The front shapes of the four turning crates are extruded parts of the moulding (end wall around the hand hole, foot, rim, floor, long walls, channel, ledge, flanges), so the section stays true while a crate is turned.",
    "de": " Die Vorderansichtsformen der vier drehbaren Kisten sind extrudierte Teile des Spritzgusskörpers (Stirnwand um das Griffloch, Fuß, Rand, Boden, Längswände, Kanal, Leiste, Flansche); der Schnitt bleibt richtig, während eine Kiste gedreht wird.",
}
COARSE = {
    "en": " The front shapes of the four turning crates are extruded parts (end wall, foot, rim, long walls, channel), so the section stays true while a crate is turned.",
    "de": " Die Vorderansichtsformen der vier drehbaren Kisten sind extrudierte Teile (Stirnwand, Fuß, Rand, Längswände, Kanal); der Schnitt bleibt richtig, während eine Kiste gedreht wird.",
}


def main():
    """▶️ Redraws the front lists of all four scales and writes a candidate or the catalog file."""
    representations = read_asset(ASSET)["representations"]
    for scale, representation in representations.items():
        front = representation["geometry"]["front"]
        if any("solid" in shape for shape in front):
            raise SystemExit("already applied")
        result = []
        for k in range(2, 6):
            old = {shape["id"]: shape for shape in front if shape["bone"] == "crate-" + str(k)}
            fine = "c%d-bottom-section" % k in old
            hole = old["c%d-retained-wall" % k]["kind"] == "path"
            rows = parts(fine, hole) + [(name, name, 0, profile, solid) for name, profile, solid in FLANGES if "c%d-%s" % (k, name) in old]
            made = set()
            for name, like, lift, profile, solid in rows:
                source = old["c%d-%s" % (k, like)]
                shape = {"id": "c%d-%s" % (k, name), "bone": "crate-" + str(k), "z": round(source["z"] + lift, 4)}
                shape.update({key: source[key] for key in STYLE if key in source})
                shape.update({"kind": "path", "d": profile, "solid": solid} if isinstance(profile, str) else {"kind": "poly", "points": profile, "closed": True, "solid": solid})
                result.append(shape)
                made.add(shape["id"])
            if set(old) - made - {"c%d-far-skirt-section" % k}:
                raise SystemExit("flat shapes left without a part: " + ", ".join(sorted(set(old) - made)))
        result.sort(key=lambda shape: shape["z"])
        representation["geometry"]["front"] = [shape for shape in front if shape["bone"] == "crate-1"] + result
        text = SENTENCE if scale in ("50", "100") else COARSE
        for language in ("en", "de"):
            representation["decisions"]["thinElements"][language] += text[language]
    if sys.argv[1:] == ["write"]:
        write_members(ASSET, {"representations": representations})
    else:
        print(candidate(ASSET, {"representations": representations}))


if __name__ == "__main__":
    main()
