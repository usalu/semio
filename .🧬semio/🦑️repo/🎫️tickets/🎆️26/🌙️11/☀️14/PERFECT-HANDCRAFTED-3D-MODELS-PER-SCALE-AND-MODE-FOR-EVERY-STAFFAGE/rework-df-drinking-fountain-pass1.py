"""🚰 Pass 1 (second round) of objects/drinking-fountain: splash beads on the bowl, tie rod with yoke seat as built, waste clamp through the bowl floor."""
import json, math, pathlib

SOURCE = pathlib.Path(__file__).parent / "volumes/objects/drinking-fountain.json"
BUBBLER_ON_BASIN = (0.015, 0.1)
JET_ON_BUBBLER = (0.149903, -0.098439)
JET_ANGLE = math.radians(-130.0)
SEAT = [[0.735, 0.056], [0.735, 0.04], [0.723, 0.04], [0.723, 0.056]]
SEAT_100 = [[0.73, 0.056], [0.73, 0.04], [0.715, 0.04], [0.715, 0.056]]


def bowl_inner_height(profile, radius):
    """📏 Height of the inner bowl surface above the bowl floor at a radius."""
    turn = max(range(len(profile)), key=lambda i: profile[i][1])
    inner = sorted(profile[turn + 1:], key=lambda p: p[1])
    for (u0, r0), (u1, r1) in zip(inner, inner[1:]):
        if r0 <= radius <= r1:
            return u0 + (u1 - u0) * (radius - r0) / (r1 - r0)
    raise ValueError(radius)


def jet_frame(u, v, w):
    """🧭 Basin coordinates to the rest frame of the jet bone."""
    bu = u - BUBBLER_ON_BASIN[0] - JET_ON_BUBBLER[0]
    bv = v - BUBBLER_ON_BASIN[1] - JET_ON_BUBBLER[1]
    c, s = math.cos(JET_ANGLE), math.sin(JET_ANGLE)
    return [round(bu * c + bv * s, 6), round(-bu * s + bv * c, 6), w]


def insert_after(profile, point, extra):
    """🪛 Insert points after a profile point."""
    i = profile.index(point)
    return profile[: i + 1] + extra + profile[i + 1:]


def by_id(solids, ident):
    """🔎 Solid of a list by id."""
    return next(s for s in solids if s["id"] == ident)


document = json.loads(SOURCE.read_text())
fine = document["volumes"]["50"]
bowl = by_id(fine["body"], "basin")["profile"]
for ident, (v, w, r) in {"drop-0": (-0.064, -0.014, 0.004), "drop-1": (-0.056, 0.03, 0.004)}.items():
    drop = by_id(fine["body"], ident)
    radius = math.hypot(v, w)
    drop["radii"] = [r, r, r]
    drop["transform"] = {"translation": jet_frame(bowl_inner_height(bowl, radius) + 0.0015, v, w)}

capital = by_id(fine["body"], "capital")
if SEAT[0] not in capital["profile"]:
    capital["profile"] = insert_after(capital["profile"], [0.783575, 0.056], SEAT)
member = by_id(fine["structure"], "member-pedestal")
if SEAT[0] not in member["profile"]:
    member["profile"] = insert_after(member["profile"], [0.783575, 0.056], SEAT)

ties = {s["id"]: s for s in fine["structure"] if s["id"] in ("tie-rod", "tie-nut-foot", "tie-nut-top", "tie-yoke")}
if all("of" not in s for s in ties.values()):
    at = next(i for i, s in enumerate(fine["body"]) if s["id"] == "button-boss")
    parts = []
    for ident, solid in ties.items():
        body = {k: v for k, v in solid.items() if k not in ("member", "part")}
        body.update({"role": "detail", "tone": 0.3})
        parts.append(body)
        index = fine["structure"].index(solid)
        fine["structure"][index] = {"id": ("member-" if solid["part"] == "member" else "joint-") + ident, "of": ident, "member": solid["member"], "part": solid["part"]}
    fine["body"][at:at] = parts

mid = document["volumes"]["100"]
by_id(mid["structure"], "member-pedestal")["profile"] = [
    [0.015, 0.106], [0.015, 0.2], [0.03, 0.2], [0.03, 0.118], [0.36, 0.118], [0.36, 0.086], [0.705, 0.074], [0.705, 0.082],
    [0.767, 0.082], [0.788, 0.095], [0.806, 0.119], [0.812, 0.1205], [0.803, 0.095], [0.7957, 0.065], [0.7908, 0.03], [0.7895, 0.024],
    [0.7795, 0.024], [0.7803, 0.03], [0.7836, 0.056], *SEAT_100, [0.705, 0.056], [0.705, 0.064348], [0.36, 0.074], [0.348, 0.074],
    [0.348, 0.106], [0.015, 0.106]]
by_id(mid["structure"], "joint-waste")["profile"] = [[-0.0255, 0.0], [-0.0255, 0.03], [-0.0105, 0.03], [-0.0105, 0.024], [0.003, 0.024], [0.003, 0.0]]

SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
print("[DEBUG] written", [by_id(fine["body"], i)["transform"] for i in ("drop-0", "drop-1")])
