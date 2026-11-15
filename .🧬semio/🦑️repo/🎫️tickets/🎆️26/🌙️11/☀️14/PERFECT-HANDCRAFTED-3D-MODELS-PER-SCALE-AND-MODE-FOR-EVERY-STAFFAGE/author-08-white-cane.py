import sys, json
from author_08_build import *
slug = "person-with-white-cane"
S = seed_of(slug)


def props(scale, P):
    out, reg = [], {}
    ft = FEATURE[scale]
    r = part_solid(P, scale, "pelvis", "bag", "bag-leather", 0.35, "bag", include=["bag"], exp=2.6)
    if r:
        out.append(r[0]); reg["bag"] = "bag"
    if scale in ("100",):
        r = part_solid(P, scale, "torso", "bag-strap", "bag-leather", 0.3, "bag", include=["bag-strap"], roles=("body", "detail"))
    mr = ft / 2 + 0.0006
    cr = 0.0095
    if scale == "50":
        pass
    else:
        sh = capsule("cane-shaft", "cane", "cane-alu-white", 0.95, "body", [0.094, 0, 0], [1.226, 0, 0], cr, slender=True)
        gr = capsule("cane-grip", "cane", "grip-eva", 0.12, "body", [-0.098, 0, 0], [0.099, 0, 0], max(0.0165, mr))
        tr = max(0.0115, mr) if scale != "100" else 0.0115
        tp = ellipsoid("cane-tip", "cane", "nylon-roller", 0.8, "body", [1.275, 0, 0], [max(0.017, mr), tr, tr])
        out += [sh, gr, tp]
        reg.update({"cane-shaft": "cane-shaft", "cane-grip": "cane-grip", "cane-tip": "cane-tip"})
    return out, reg


def glasses_extra(scale, body):
    ft = FEATURE[scale]
    r = ft / 2 + 0.0006
    return [ellipsoid("visor", "head", "sunglass-polymer", 0.05, "body", [0.109, -0.0667, 0.0], [max(0.015, r), max(0.004, r), max(0.056, r)])]


KEEP = {
    "50": ["cane-shaft", "cane-band", "cane-grip", "cane-strap-1", "cane-strap-2", "cane-strap-3", "cane-strap-4", "cane-strap-5", "cane-tip-holder", "cane-roller", "bag-flap", "bag-strap", "lens-l", "lens-r", "sunglasses-arm-l", "sunglasses-arm-r", "bridge", "ear-l", "ear-r", "eye-l", "eye-r", "thumb-l", "thumb-r", "finger-l0", "finger-r0", "button-0", "button-1", "button-2", "front", "cuff-s-l", "cuff-s-r", "cuff-l", "cuff-r"],
    "100": ["lens-l", "lens-r", "bag-strap", "ear-l", "ear-r", "thumb-l", "thumb-r"],
    "200": [],
    "500": [],
}
person = {
    "is_person": is_person,
    "parts": {
        "pelvis": [dict(id="hip", mat="denim-jacket", tone=0.4, region="jacket", exclude=["bag", "bag-flap"])],
        "neck": [dict(id="neck", mat="skin-medium", tone=0.82, region="face-skin", include=["neck"]), dict(id="collar", mat="denim-jacket", tone=0.46, region="jacket", include=["collar"])],
        "head": [dict(id="head", mat="skin-medium", tone=0.82, region="face-skin", include=["head"]), dict(id="hair", mat="hair-brown", tone=0.28, region="hair", include=["hair"], maxn=6)],
    },
}
cfg = {
    "person": person,
    "keep": KEEP,
    "props": props,
    "extra": {"hands": lambda scale, body: [ellipsoid("hand-l-mass", "hand-l", "skin-medium", 0.7, "body", [0.09, 0, 0], [0.04, 0.0381, 0.0381]), ellipsoid("hand-r-mass", "hand-r", "skin-medium", 0.7, "body", [0.09, 0, 0], [0.04, 0.0381, 0.0381])] if not any(b["bone"].startswith("hand") for b in body) else [], "sunglasses": lambda scale, body: glasses_extra(scale, body) if scale in ("200", "500") else [], "hair": None},
    "emitter_default": "face-skin",
    "k": 0.5,
    "member_k": {"torso": 0.6, "head": 0.55},
    "joints": {"50": ["arm-l", "arm-r", "forearm-l", "forearm-r", "thigh-l", "thigh-r", "shank-l", "shank-r", "torso", "head"], "100": ["arm-l", "arm-r", "forearm-l", "forearm-r", "thigh-l", "thigh-r", "shank-l", "shank-r"], "200": ["arm-l", "arm-r", "thigh-l", "thigh-r"], "500": []},
    "masses": {"50": ["torso", "pelvis", "head", "thigh-l", "thigh-r"], "100": ["torso", "pelvis", "head", "thigh-l", "thigh-r"], "200": ["torso", "head", "thigh-l", "thigh-r"], "500": ["torso"]},
    "contact_pads": [],
    "support_mat": "nylon-roller",
    "main": {},
    "decisions": lambda sc, fac: decisions_std(sc, fac, {"en": {"50": "the shoulder bag, sunglasses, buttons, finger masses and the cane with grip loop and rolling tip are kept as hand-placed details.", "100": "the cane is shaft, grip and tip, the bag has its strap, details below 15 mm are gone.", "200": "the figure is merged to one loft per bone, the cane a slender shaft, grip and tip.", "500": "one or two sections per bone; hands and sunglasses are single masses."}, "de": {"50": "Umhängetasche, Sonnenbrille, Knöpfe, Fingermassen und der Stock mit Griffschlaufe und Rollspitze bleiben als von Hand gesetzte Details.", "100": "der Stock besteht aus Schaft, Griff und Spitze, die Tasche hat ihren Gurt, Details unter 15 mm entfallen.", "200": "die Figur ist je Knochen ein Loft, der Stock ein schlanker Schaft mit Griff und Spitze.", "500": "ein bis zwei Schnitte je Knochen; Hände und Sonnenbrille sind einzelne Massen."}}),
}
if cfg["extra"]["hair"] is None:
    def hair_extra(scale, body):
        h = next((b for b in body if b["id"] == "head"), None)
        s = scale_lat(h, 1.06) if False else None
        import copy
        c = copy.deepcopy(h)
        c["id"] = "hair-cap"; c["material"] = "hair-brown"; c["tone"] = 0.28
        for sec in c["sections"]:
            sec["rx"] = r5(sec["rx"] * 1.07); sec["ry"] = r5(sec["ry"] * 1.07)
        return [c]
    cfg["extra"]["hair"] = lambda scale, body: hair_extra(scale, body) if not any(b["id"] == "hair" for b in body) else []
if __name__ == "__main__":
    run(slug, cfg, scales=sys.argv[1:] or SCALES)
