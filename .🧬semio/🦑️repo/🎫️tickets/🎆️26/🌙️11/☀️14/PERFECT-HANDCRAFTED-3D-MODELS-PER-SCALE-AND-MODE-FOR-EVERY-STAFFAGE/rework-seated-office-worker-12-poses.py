"""🧘 Stage 12 of the seated office worker: the resting left hand of `reading-screen` lies on the left thigh.

Before, the wrist was extended 38 degrees and the fingers stood in the front rail of the desk. The upper arm now hangs plumb, the forearm slopes onto the thigh and the hand follows the thigh; the numbers are stated here and were set by looking at the left side sheet.

Run: `python3 rework-seated-office-worker-12-poses.py [--arm A --forearm F --hand H]` (candidate), `--apply` writes `poses` and `clips` of the catalog file.
"""
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"


def number(flag, default):
    """🔢 A number from the command line."""
    return float(sys.argv[sys.argv.index(flag) + 1]) if flag in sys.argv else default


doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
pose = next(p for p in a["poses"] if p["id"] == "reading-screen")["bones"]
pose["arm-l"]["angleDeg"] = number("--arm", 0)
pose["forearm-l"]["angleDeg"] = number("--forearm", 57)
pose["hand-l"].update({"angleDeg": number("--hand", 20), "abductionDeg": 8})
for clip in a["clips"]:
    for key in clip["keys"]:
        if key.get("pose") == "reading-screen":
            for b in ("arm-l", "forearm-l", "hand-l"):
                if b in key.get("bones", {}):
                    key["bones"][b] = dict(pose[b])
write(candidate_path(A), doc)
if "--apply" in sys.argv:
    live = read(catalog_path(A))
    for name in ("poses", "clips"):
        live["asset"][name] = a[name]
    write(catalog_path(A), live)
    print("applied")
