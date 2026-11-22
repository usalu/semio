"""🦽 wheelchair-user: hands, rear wheels, axle carriers and long bones of 1:100, 1:200 and 1:500, each stated for the smallest feature of its scale (15 / 30 / 75 mm).

Every number is chosen by hand. Left parts are stated, right parts are their mirror images (lateral coordinate negated).
- 1:100: five rays stay; the bare little fingertip, the hand-rim standoffs and the finger bones are typified to 15 mm.
- 1:200: the four gloved fingers are one curled mitt, the bare fingertips one pad, the thumb one gloved roll; the hand skeleton is metacarpus, proximal and distal phalanx rows; the six hand-rim standoffs are one standoff web turned with the hand rim; rim and hand rim keep their true section as hoops.
- 1:500: the hand is one mitt with a fingertip pad, the hand skeleton one plate; the hub is a plain drum, the axle carrier one sleeve; tyre, rim and hand rim keep their true section as hoops.
- 1:200 and 1:500: humerus, radius, ulna, femur (with head and neck) and tibia keep the shaft sections of 1:50 as slender members.
"""
import copy
import math


def about_w(degrees):
    half = math.radians(degrees) / 2
    return [0, 0, round(math.sin(half), 5), round(math.cos(half), 5)]


def ellipsoid(ident, bone, material, tone, radii, centre, degrees=0.0, **extra):
    transform = {"translation": centre}
    if degrees:
        transform["rotation"] = about_w(degrees)
    return {"id": ident, "kind": "ellipsoid", "bone": bone, "material": material, "role": "body", "tone": tone, "radii": radii, "transform": transform, **extra}


def mirrored(solid):
    out = copy.deepcopy(solid)
    out["id"] = out["id"][:-2] + "-r"
    for key in ("bone", "member"):
        if key in out and out[key].endswith("-l"):
            out[key] = out[key][:-2] + "-r"
    if out["kind"] == "ellipsoid":
        t = out["transform"]["translation"]
        out["transform"]["translation"] = [t[0], t[1], -t[2] + 0.0]
    elif out["kind"] == "loft":
        for section in out["sections"]:
            section["at"] = [section["at"][0], section["at"][1], -section["at"][2] + 0.0]
    elif out["kind"] == "capsule":
        for key in ("p0", "p1"):
            out[key] = [out[key][0], out[key][1], -out[key][2] + 0.0]
    elif out["kind"] == "revolve":
        out["profile"] = [[-p[0], p[1]] for p in reversed(out["profile"])]
    return out


def both(solid):
    return [solid, mirrored(solid)]


BONE = {"material": "bone", "tone": 0.62}
MEMBER = {"member": "hand-l", "part": "member"}
HAND_RIM_WEB = [[0.2915, 0.2625], [0.3008, 0.2625], [0.30144, 0.26088], [0.30408, 0.25757], [0.30789, 0.25574], [0.31211, 0.25574], [0.31592, 0.25757], [0.31856, 0.26088], [0.3195, 0.265], [0.31856, 0.26912], [0.31592, 0.27243], [0.31211, 0.27426], [0.30789, 0.27426], [0.30408, 0.27243], [0.30144, 0.26912], [0.3008, 0.2675], [0.2915, 0.2675], [0.2915, 0.2625]]
HUB_DRUM = [[0.243, 0], [0.243, 0.038], [0.319, 0.038], [0.319, 0]]
FEMUR_HEAD = [{"at": [0, 0, 0], "rx": 0.022, "ry": 0.022}, {"at": [0.008, -0.004, 0.034], "rx": 0.0135, "ry": 0.0135}]
LONG = {"humerus": "arm", "femur": "thigh", "tibia": "shank", "radius": "forearm", "ulna": "forearm"}


def find(items, ident):
    return next((s for s in items if s["id"] == ident), None)


def drop(volume, ids):
    ids = set(ids)
    for name in ("body", "thermal", "structure"):
        volume[name] = [s for s in volume[name] if s["id"] not in ids and s.get("of") not in ids]


def after(items, ident, fresh):
    at = next((i for i, s in enumerate(items) if s["id"] == ident), len(items) - 1)
    items[at + 1:at + 1] = fresh


def radii(solid, value):
    if solid is None:
        return
    if solid["kind"] == "loft":
        for section in solid["sections"]:
            section["rx"] = section["ry"] = value
    elif solid["kind"] == "capsule":
        solid["r0"] = solid["r1"] = value
    else:
        solid["radii"] = [value, value, value]


def long_bones(volumes, scale):
    fine, coarse = volumes["50"]["structure"], volumes[scale]["structure"]
    for side in ("l", "r"):
        for name, bone in LONG.items():
            shaft = copy.deepcopy(find(fine, f"{name}-{side}"))
            shaft["slender"] = True
            if name == "femur":
                sign = 1 if side == "l" else -1
                shaft["sections"] = [{**s, "at": [s["at"][0], s["at"][1], sign * s["at"][2] + 0.0]} for s in copy.deepcopy(FEMUR_HEAD)] + shaft["sections"]
            old = find(coarse, f"{name}-{side}")
            if old is not None:
                coarse[coarse.index(old)] = shaft
            elif name == "radius":
                merged = find(coarse, f"forearm-bones-{side}")
                coarse[coarse.index(merged)] = shaft
            else:
                after(coarse, f"radius-{side}", [shaft])


def scale_100(volumes):
    volume = volumes["100"]
    for side in ("l", "r"):
        tip = find(volume["body"], f"finger-4-{side}")
        for section in tip["sections"]:
            section["rx"] = section["ry"] = round(section["rx"] * 1.07, 5)
        radii(find(volume["body"], f"hand-rim-tabs-{side}"), 0.0076)
        for ident in [f"metacarpal-{n}-{side}" for n in range(1, 5)] + [f"phalanges-{n}-{side}" for n in range(1, 5)] + [f"thumb-bones-{side}"]:
            radii(find(volume["structure"], ident), 0.0076)
        for name in ("radius", "ulna", "fibula"):
            fine = copy.deepcopy(find(volumes["50"]["structure"], f"{name}-{side}"))
            fine["slender"] = True
            old = find(volume["structure"], f"{name}-{side}")
            volume["structure"][volume["structure"].index(old)] = fine


def wheels(volume, scale):
    for side in ("l", "r"):
        drop(volume, [f"hand-rim-tabs-{side}"])
        for ident in (f"rear-rim-{side}", f"hand-rim-{side}") + ((f"rear-tyre-{side}",) if scale == "500" else ()):
            find(volume["body"], ident)["slender"] = True
        rim = find(volume["body"], f"hand-rim-{side}")
        rim["profile"] = copy.deepcopy(HAND_RIM_WEB) if side == "l" else [[-p[0], p[1]] for p in reversed(HAND_RIM_WEB)]
        if scale == "500":
            hub = find(volume["body"], f"rear-hub-{side}")
            hub["profile"] = copy.deepcopy(HUB_DRUM) if side == "l" else [[-p[0], p[1]] for p in reversed(HUB_DRUM)]
            drop(volume, [f"axle-plate-{side}"])
            radii(find(volume["body"], f"axle-receiver-{side}"), 0.038)
        else:
            find(volume["body"], f"axle-plate-{side}")["size"][2] = 0.012


def scale_200(volumes):
    volume = volumes["200"]
    wheels(volume, "200")
    for side in ("l", "r"):
        drop(volume, [f"glove-finger-{n}-{side}" for n in range(1, 5)] + [f"finger-{n}-{side}" for n in range(1, 5)] + [f"thumb-{side}", f"carpals-{side}", f"thumb-bones-{side}"] + [f"metacarpal-{n}-{side}" for n in range(1, 5)] + [f"phalanges-{n}-{side}" for n in range(1, 5)])
        sign = 1 if side == "l" else -1
        thumb = find(volume["body"], f"glove-thumb-{side}")
        thumb["sections"].append({"at": [0.09, -0.044, -0.046 * sign], "rx": 0.0152, "ry": 0.0152})
        radii(thumb, 0.0152)
        radii(find(volume["structure"], f"joint-hand-{side}"), 0.0152)
        radii(find(volume["structure"], f"mass-hand-{side}"), 0.0152)
    mitt = ellipsoid("glove-fingers-l", "hand-l", "leather-glove", 0.15, [0.036, 0.0155, 0.04], [0.115, -0.012, 0], -32)
    pad = ellipsoid("fingertips-l", "hand-l", "skin", 0.27, [0.027, 0.0152, 0.037], [0.139, -0.047, 0], -95)
    for solid in both(mitt) + both(pad):
        side = solid["id"][-1]
        after(volume["body"], f"glove-thenar-{side}", [solid])
        skin = solid["material"] == "skin"
        after(volume["thermal"], f"th-glove-thenar-{side}-gloves", [{"id": f"th-{solid['id']}", "of": solid["id"], "region": "hand-skin"} if skin else {"id": f"th-{solid['id']}-gloves", "of": solid["id"], "region": "gloves", "layer": "gloves"}])
    skeleton = [
        ellipsoid("metacarpus-l", "hand-l", **BONE, radii=[0.047, 0.0152, 0.034], centre=[0.05, -0.001, 0], **MEMBER),
        ellipsoid("phalanges-l", "hand-l", **BONE, radii=[0.03, 0.0152, 0.034], centre=[0.115, -0.012, 0], degrees=-32, **MEMBER),
        ellipsoid("phalanges-distal-l", "hand-l", **BONE, radii=[0.024, 0.0152, 0.033], centre=[0.139, -0.045, 0], degrees=-95, **MEMBER),
    ]
    for solid in skeleton:
        for one in both(solid):
            after(volume["structure"], f"joint-hand-{one['id'][-1]}", [one])
    long_bones(volumes, "200")


def scale_500(volumes):
    volume = volumes["500"]
    wheels(volume, "500")
    for side in ("l", "r"):
        gone = [s["id"] for s in volume["body"] + volume["structure"] if s.get("bone") == f"hand-{side}"] + [f"glove-cuff-{side}"]
        anchor = volume["body"][[i for i, s in enumerate(volume["body"]) if s["id"] == f"sleeve-cuff-{side}"][0]]
        drop(volume, gone)
        volume["thermal"] = [s for s in volume["thermal"] if s["id"] not in (f"th-glove-{side}-gloves",)]
    mitt = ellipsoid("glove-l", "hand-l", "leather-glove", 0.15, [0.078, 0.038, 0.046], [0.05, -0.004, 0], -8)
    pad = ellipsoid("fingertips-l", "hand-l", "skin", 0.27, [0.04, 0.038, 0.042], [0.128, -0.036, 0])
    for solid in both(mitt) + both(pad):
        side = solid["id"][-1]
        after(volume["body"], f"sleeve-cuff-{side}", [solid])
        skin = solid["material"] == "skin"
        after(volume["thermal"], f"th-sleeve-cuff-{side}", [{"id": f"th-{solid['id']}", "of": solid["id"], "region": "hand-skin"} if skin else {"id": f"th-{solid['id']}-gloves", "of": solid["id"], "region": "gloves", "layer": "gloves"}])
    plate = ellipsoid("hand-bones-l", "hand-l", **BONE, radii=[0.07, 0.0377, 0.04], centre=[0.07, -0.01, 0], degrees=-12, **MEMBER)
    for one in both(plate):
        after(volume["structure"], f"joint-forearm-{one['id'][-1]}", [one])
    long_bones(volumes, "500")


def apply(asset):
    volumes = asset["volumes"]
    scale_100(volumes)
    scale_200(volumes)
    scale_500(volumes)
