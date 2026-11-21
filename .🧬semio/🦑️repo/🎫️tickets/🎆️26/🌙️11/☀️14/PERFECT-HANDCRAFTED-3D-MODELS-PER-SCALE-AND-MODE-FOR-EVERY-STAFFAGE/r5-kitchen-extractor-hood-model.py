"""🏔️ Round 5 kitchen extractor hood: keeps the hand-stated lists of `r3-kitchen-extractor-hood-model.py` and restates what the round-5 look found: at 1:100, 1:200 and 1:500 the back sheet lay with its slanted edges in the outer planes of the side slopes and with its top in the top face of the top plate (coplanar faces flickered as stripes along the wall); it is now set 2 mm inside both, buried in the slopes and the top plate, and its wall face, like the closed rim slab of 1:200 and 1:500, 0.5 mm proud of the slope ends. The 1:200 and 1:500 structure gets the canopy's own load path back (rim frame and the two folded hips), which only 1:50 and 1:100 had. At 1:100, 1:200 and 1:500 the canopy shell is closed (rim slab, filter field, lamps, top plate), so the inward faces need no `sectionFaces` stand-in (C31 g): the flood finds them inside; at 1:50 the open duct lets the extract air into the shell and the stand-in stays; writes `volumes/appliances/kitchen-extractor-hood.json`."""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("model3", os.path.join(HERE, "r3-kitchen-extractor-hood-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)
k = M.k
FLAT = k.back_plate
INSET = 0.002


def back_plate_inset(sid, bone, material, role, tone, t, **kw):
    """🧱 The back sheet set 2 mm inside the side slopes and under the top face of the top plate, so its edges are buried and never coplanar with an outer face; its wall face stands 0.5 mm proud of the end faces of the slopes, against the wall."""
    return k.ext(sid, bone, material, role, tone, "yz", [(k.RIM, -k.HALF_W + INSET), (k.RIM, k.HALF_W - INSET), (k.TOP - INSET, k.TOP_HW - INSET), (k.TOP - INSET, -k.TOP_HW + INSET)], -0.0005, t, **kw)


def canopy_frame(wall, leg, t, head, foot):
    """🪢 The canopy's own load path at a coarse scale: the rim band as a closed frame and the two folded hips from the top plate corners down to it."""
    return [
        k.st(M.rim("member-rim", 0.6, None, wall=wall), M.C, "member"),
        k.st(M.hip_fold("member-hip-left", 0.6, 1, leg, t, head, foot), M.C, "member"),
        k.st(M.hip_fold("member-hip-right", 0.6, -1, leg, t, head, foot), M.C, "member"),
    ]


def main():
    vols = {"50": M.scale_50()}
    k.back_plate = back_plate_inset
    vols["100"] = M.scale_100()
    vols["200"] = M.scale_200()
    vols["500"] = M.scale_500()
    k.back_plate = FLAT
    for den in ("200", "500"):
        vols[den]["body"][0] = k.box("rim", M.C, M.SS, "body", 0.78, (-0.0005, 0, -k.HALF_W), (k.DEPTH, k.RIM, k.HALF_W))
    vols["200"]["structure"][2:2] = canopy_frame(0.03, 0.045, 0.03, 0.03, 0.05)
    vols["500"]["structure"][1:1] = canopy_frame(0.05, 0.06, 0.05, 0.05, 0.07)
    for den in ("100", "200", "500"):
        for t in vols[den]["thermal"]:
            if t.get("region") == "canopy" and t.get("sectionFaces", ["+u"]) != ["+u"]:
                del t["sectionFaces"]
    texts = json.load(open(os.path.join(HERE, "r5-kitchen-extractor-hood-texts.json"), encoding="utf-8"))
    for den, t in texts.items():
        vols[den]["decisions"][2] = t
    from runpy import run_path
    run_path(os.path.join(HERE, 'digital-hood.py'))['author'](vols)
    with open(M.SOURCE, "w", encoding="utf-8") as fh:
        json.dump({"volumes": vols}, fh, indent=1, ensure_ascii=False)
        fh.write("\n")
    for key in ("50", "100", "200", "500"):
        print(key, {l: len(vols[key][l]) for l in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
