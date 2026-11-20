"""🦴 Rig roots of the round 3 delivery van: the rear door hinges on the outer rear corners of a 1.97 m body, and the static axle share of an empty front-engined van."""
MEMBERS = ["rig", "anchors"]
BONES = {"rear-door-r": {"x": -2.61, "z": -0.86}, "rear-door-l": {"x": -2.61, "z": 0.86}}


def apply(asset):
    for bone in asset["rig"]["bones"]:
        for key, value in BONES.get(bone["id"], {}).items():
            bone[key] = value
        if bone["id"] == "chassis":
            bone["mass"]["comX"] = 0.48
    for anchor in asset["anchors"]:
        if anchor["id"] == "tailpipe":
            anchor["x"], anchor["y"] = -2.63, 0.30
    return asset
