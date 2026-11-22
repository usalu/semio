"""⛓️ Playground swing: closes the structural load path seat -> eye bolts -> chains -> hanger -> beam -> apex sockets -> legs -> feet -> supports.

Hand-chosen numbers: M10 eye bolt (r 5 mm, 7.5 mm at 1:100) from the eye centre down into the seat insert, a hitch
node of r 35 / 40 mm at 1:200 / 1:500;
chain members run exactly from the pin (z = +-0.25) to the seat eye (z = +-0.20); at 1:50 the hanger fork and band and
the four welded leg sockets and gussets join the list as joints; at 1:100 the hanger block is the joint between beam and pin; coarser than 1:50 every foot (plate or block) is the joint between leg and support.

Usage: python3 rework-15b-playground-swing-loadpath.py   (edits the source once, refreshes the candidate document)
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/objects/playground-swing.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-playground-swing/🔣️.json")

EYE_BOLT_RADIUS = {"50": 0.005, "100": 0.0075}
HITCH_RADIUS = {"200": 0.035, "500": 0.04}
SEAT_CORE_Y = {"50": -0.0475, "100": -0.0475}
FOOT = {"50": None, "100": "plate", "200": "foot", "500": "foot"}


def joint(identifier, of, member):
    return {"id": identifier, "of": of, "member": member, "part": "joint"}


def capsule(identifier, bone, p0, p1, radius, member, part):
    return {
        "id": identifier, "kind": "capsule", "bone": bone, "material": "steel-galvanised", "role": "body", "tone": 0.5,
        "p0": p0, "p1": p1, "r0": radius, "r1": radius, "slender": True, "member": member, "part": part,
    }


def plain(body, of, identifier, member):
    """📦 The body box of a plate or hanger as a sharp-edged structure joint of its own (1:100: no rounded corners in the structure list)."""
    box = json.loads(json.dumps(next(solid for solid in body if solid["id"] == of)))
    box.pop("cornerRadius", None)
    return {**box, "id": identifier, "role": "detail", "tone": 0.2, "member": member, "part": "joint"}


def edit(volumes):
    for scale, lists in volumes.items():
        structure = lists["structure"]
        if any(solid["id"] == "member-eye-bolt-l" for solid in structure):
            raise SystemExit("already applied")
        for solid in structure:
            if solid["id"] in ("member-chain-left", "member-chain-right"):
                side = 1 if solid["id"].endswith("left") else -1
                solid["p0"] = [0.0, 0.0, side * 0.25]
                solid["p1"] = [1.9, 0.0, side * 0.2]
        for name, side in (("l", 1), ("r", -1)):
            if scale in EYE_BOLT_RADIUS:
                bolt = capsule(f"member-eye-bolt-{name}", "seat", [0.0, 0.0, side * 0.2], [0.0, SEAT_CORE_Y[scale], side * 0.2], EYE_BOLT_RADIUS[scale], "seat", "member")
                del bolt["slender"]
                structure.append(bolt)
            else:
                structure.append({
                    "id": f"joint-hitch-{'left' if side == 1 else 'right'}", "kind": "ellipsoid", "bone": "seat", "material": "steel-galvanised", "role": "detail", "tone": 0.2,
                    "radii": [HITCH_RADIUS[scale]] * 3, "transform": {"translation": [0.0, 0.0, side * 0.2]}, "member": "seat", "part": "joint",
                })
        if scale == "50":
            for name in ("left", "right"):
                structure.append(joint(f"joint-hanger-band-{name}", f"hanger-band-{name}", "beam"))
                structure.append(joint(f"joint-hanger-cheek-{name}-1", f"hanger-cheek-{name}-1", "beam"))
                structure.append(joint(f"joint-hanger-cheek-{name}-2", f"hanger-cheek-{name}-2", "beam"))
            for frame in ("l", "r"):
                structure.append(joint(f"joint-socket-left-{frame}", f"socket-left-{frame}", "leg-left"))
                structure.append(joint(f"joint-socket-right-{frame}", f"socket-right-{frame}", "leg-right"))
                structure.append(joint(f"joint-gusset-{frame}", f"gusset-{frame}", "beam"))
        if scale == "100":
            for name, side in (("left", 1), ("right", -1)):
                structure.append(plain(lists["body"], f"hanger-{name}", f"joint-hanger-{name}", "beam"))
        if FOOT[scale]:
            for leg in ("left", "right"):
                for frame in ("l", "r"):
                    of = f"{FOOT[scale]}-{leg}-{frame}"
                    structure.append(plain(lists["body"], of, f"joint-foot-{leg}-{frame}", f"leg-{leg}") if scale == "100" else joint(f"joint-foot-{leg}-{frame}", of, f"leg-{leg}"))


with open(SOURCE, encoding="utf-8") as handle:
    source = json.load(handle)
edit(source["volumes"])
with open(SOURCE, "w", encoding="utf-8") as handle:
    json.dump(source, handle, ensure_ascii=False, indent=1)
with open(CANDIDATE, encoding="utf-8") as handle:
    candidate = json.load(handle)
candidate["asset"]["volumes"] = source["volumes"]
with open(CANDIDATE, "w", encoding="utf-8") as handle:
    json.dump(candidate, handle, ensure_ascii=False, indent=1)
