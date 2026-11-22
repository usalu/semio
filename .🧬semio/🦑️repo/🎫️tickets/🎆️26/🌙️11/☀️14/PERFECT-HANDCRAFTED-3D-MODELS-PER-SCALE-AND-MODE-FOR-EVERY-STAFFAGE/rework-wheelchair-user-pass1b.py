"""🦽 wheelchair-user, author pass 1, second chisel: the joints of the skeleton that were still open, and the seat sling at 1:100 and 1:200.

- body of the ilium (arcuate line) from the sacroiliac joint to the hip joint: the bone that carries the trunk onto the femoral heads;
- the acromion ends on the acromial end of the clavicle; 1:200 clavicle bar starts in the sternoclavicular joint;
- metacarpals and the thumb start inside the carpal block; the tarsal block starts in the calcaneus;
- 1:100 and 1:200 get the seat sling (a 10 mm sheet between the side tubes) that carries the cushion; the 1:100 footplate is the 16 mm plate lying on the loop.
"""
import copy

ILIUM_BODY_R = {"50": 0.013, "100": 0.013, "200": 0.0152}
SLING = [([-0.08, 0.15738, 0], 0.229, 0.005), ([0.1, 0.17364, 0], 0.229, 0.005), ([0.33, 0.19441, 0], 0.229, 0.005)]


def find(volume, name, ident):
    return next((s for s in volume[name] if s["id"] == ident), None)


def apply(asset):
    fine = asset["volumes"]["50"]
    for scale, volume in asset["volumes"].items():
        for side, sign in (("l", 1), ("r", -1)):
            if scale in ILIUM_BODY_R and find(volume, "structure", f"ilium-body-{side}") is None:
                r = ILIUM_BODY_R[scale]
                at = next(i for i, s in enumerate(volume["structure"]) if s["id"] == f"ilium-{side}") + 1
                volume["structure"].insert(at, {"id": f"ilium-body-{side}", "kind": "loft", "bone": f"ilium-{side}", "material": "bone", "role": "body", "tone": 0.4, "sections": [{"at": [0, 0, 0], "rx": r, "ry": r}, {"at": [0.10257, 0, 0], "rx": r, "ry": r}], "capStart": True, "capEnd": True, "member": f"ilium-{side}", "part": "member"})
            acromion = find(volume, "structure", f"acromion-{side}")
            if acromion and len(acromion["sections"]) == 3:
                acromion["sections"].append({"at": [0.163, -0.002, 0.003 * sign], "rx": 0.008, "ry": 0.008})
            for ray in "1234":
                meta = find(volume, "structure", f"metacarpal-{ray}-{side}")
                if meta:
                    meta["sections"][0]["at"][0] = 0.02
            thumb = find(volume, "structure", f"thumb-bones-{side}")
            if thumb and scale in ("50", "100"):
                thumb["sections"][0]["at"] = [0.014, -0.004, 0.02 * (-1 if side == "l" else 1)]
            if scale == "50":
                find(volume, "structure", f"tarsals-{side}")["sections"][0]["at"] = [-0.012, -0.026, 0]
            if scale == "200":
                bar = find(volume, "structure", f"shoulder-bar-{side}")
                bar["sections"][0]["at"] = [0, 0, 0]
        if scale == "50":
            find(volume, "structure", "spinous-14")["p0"] = [0.2, 0.03, 0]
        if scale == "100":
            plate = find(volume, "body", "footplate")
            plate["size"][1] = 0.01625
            plate["transform"]["translation"][1] = -0.208375
        if scale in ("100", "200") and find(volume, "body", "seat-sling") is None:
            sling = copy.deepcopy(find(fine, "body", "seat-sling"))
            sling["sections"] = [{"at": at, "rx": rx, "ry": ry, "exponent": 6, "rotation": [0, -0.70711, 0, 0.70711]} for at, rx, ry in SLING]
            sling["sheet"] = True
            at = next(i for i, s in enumerate(volume["body"]) if s["id"] == "seat-cushion")
            volume["body"].insert(at, sling)
            thermal = copy.deepcopy(find(fine, "thermal", "th-seat-sling"))
            at = next(i for i, s in enumerate(volume["thermal"]) if s["id"] == "th-seat-cushion")
            volume["thermal"].insert(at, thermal)
            volume["structure"].append({"id": "st-seat-sling", "of": "seat-sling", "member": "frame", "part": "member"})
    scapulae(asset)


SCAPULA_BLADE = [(-0.075, -0.01, 0.008), (-0.03, 0.0, 0.03), (0.02, 0.012, 0.048), (0.05, 0.015, 0.052), (0.065, 0.0, 0.035)]
SCAPULA_THICK = {"50": 0.004, "100": 0.0075}
GLENOID = [([0.045, -0.008, 0.055], 0.012), ([0.052, -0.03, 0.078], 0.012), ([0.057, -0.052, 0.098], 0.016)]
SPINE_ROOT = [0.045, 0.008, -0.035]


def matrix_of(q):
    x, y, z, w = q
    return [[1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)], [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)], [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)]]


def scapulae(asset):
    """🦴 The scapula as a triangular blade with glenoid neck and spine instead of an oval disc, in the local frame of the old disc (x up the medial border, y out of the back, z towards the shoulder)."""
    import importlib.util
    import os
    spec = importlib.util.spec_from_file_location("rig_kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rig-lateral-standing-adult.py"))
    kit = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(kit)
    frames = kit.frames(copy.deepcopy(asset["rig"]["bones"]))
    for scale, thick in SCAPULA_THICK.items():
        volume = asset["volumes"][scale]
        axes = {}
        for side in ("l", "r"):
            disc = find(volume, "structure", f"scapula-{side}")
            if disc["kind"] != "ellipsoid":
                return
            local = matrix_of(disc["transform"]["rotation"])
            bone = frames[f"clavicle-{side}"][1]
            axes[side] = [kit.apply(bone, [local[0][k], local[1][k], local[2][k]]) for k in range(3)]
        sign = [1 if sum(axes["r"][k][i] * axes["l"][k][i] * (-1 if i == 2 else 1) for i in range(3)) > 0 else -1 for k in range(3)]
        for side in ("l", "r"):
            s = [1, 1, 1] if side == "l" else sign
            disc = find(volume, "structure", f"scapula-{side}")
            transform = copy.deepcopy(disc["transform"])
            at = lambda p: [round(p[k] * s[k], 5) for k in range(3)]
            for key in ("radii",):
                disc.pop(key)
            disc.update({"kind": "loft", "sections": [{"at": at([x, 0, z]), "rx": thick, "ry": half, "exponent": 2.6, "rotation": [0.5, 0.5, 0.5, 0.5]} for x, z, half in SCAPULA_BLADE], "capStart": True, "capEnd": True, "transform": transform})
            neck = {"id": f"glenoid-{side}", "kind": "loft", "bone": disc["bone"], "material": "bone", "role": "body", "tone": 0.4, "sections": [{"at": at(p), "rx": r, "ry": r} for p, r in GLENOID], "capStart": True, "capEnd": True, "transform": copy.deepcopy(transform), "member": disc["member"], "part": "member"}
            extra = [neck]
            acromion = find(volume, "structure", f"acromion-{side}")
            if scale == "50" and acromion:
                local = matrix_of(transform["rotation"])
                d = [acromion["sections"][0]["at"][i] - transform["translation"][i] for i in range(3)]
                root = [round(sum(local[i][k] * d[i] for i in range(3)), 5) for k in range(3)]
                extra.append({"id": f"scapular-spine-{side}", "kind": "loft", "bone": disc["bone"], "material": "bone", "role": "body", "tone": 0.4, "sections": [{"at": at(SPINE_ROOT), "rx": 0.005, "ry": 0.005}, {"at": root, "rx": 0.007, "ry": 0.007}], "capStart": True, "capEnd": True, "transform": copy.deepcopy(transform), "member": disc["member"], "part": "member"})
            index = volume["structure"].index(disc) + 1
            volume["structure"][index:index] = extra
