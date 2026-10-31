from mallard_scn import *

def base(advance=0.25):
    bones, anchors = build()
    asset = {"id": ID, "category": "animals"}
    asset["rig"] = {"kind": "anatomical", "root": "trunk", "bones": bones}
    asset["anchors"] = anchors
    asset["materials"] = MATERIALS
    P, wk, pk, N, M = build_scenario(asset)
    asset["poses"] = list(P.values())
    asset["clips"] = clips(wk, pk, N, M, advance)
    th, skin = thermal(None)
    asset["thermal"] = th
    return asset

if __name__ == "__main__":
    import sys
    a = base()
    for pid in sys.argv[1:] or ["standing"]:
        print("==", pid)
        print_rig(a, pid)
