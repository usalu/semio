"""🐴 Horse 1:500: ears as pointed buds instead of capped tubes (joint spheres on the tail were tried and dropped: no visible gain, more triangles than 1:200)."""
import json

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
SOURCE = f"{T}/volumes/animals/horse.json"
KNUCKLES = {}
OFFSET = {"tail-2": -0.022, "tail-3": -0.018, "tail-4": -0.004}


def main():
    document = json.load(open(SOURCE))
    body = document["volumes"]["500"]["body"]
    out = []
    for solid in body:
        if solid["id"].startswith("tail-knuckle"):
            continue
        if solid["id"] in ("near-ear", "far-ear"):
            sign = -1 if solid["id"] == "near-ear" else 1
            keep = {k: solid[k] for k in ("id", "bone", "material", "role", "tone")}
            solid = {"id": keep["id"], "kind": "ellipsoid", "bone": keep["bone"], "material": keep["material"], "role": keep["role"], "tone": keep["tone"], "radii": [0.108, 0.0375, 0.0375], "transform": {"translation": [0.058, 0.0, sign * 0.068], "rotation": [0.0, 0.0, 0.0, 1.0]}}
        out.append(solid)
        if solid["id"] in KNUCKLES:
            r = KNUCKLES[solid["id"]]
            out.append({"id": f"tail-knuckle-{solid['id'][-1]}", "kind": "ellipsoid", "bone": solid["bone"], "material": solid["material"], "role": solid["role"], "tone": solid["tone"], "radii": [r, r, r + 0.004], "transform": {"translation": [0.0, OFFSET[solid["id"]], 0.0], "rotation": [0.0, 0.0, 0.0, 1.0]}})
    document["volumes"]["500"]["body"] = out
    json.dump(document, open(SOURCE, "w"), ensure_ascii=False, indent=1)


main()
