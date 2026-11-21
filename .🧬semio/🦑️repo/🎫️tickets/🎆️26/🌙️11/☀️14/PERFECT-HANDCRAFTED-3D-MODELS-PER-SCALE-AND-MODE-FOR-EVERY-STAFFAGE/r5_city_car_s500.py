"""🚘 Round 5 1:500 lists of the city-car: the round 3 massing for 75 mm features with the ball mirrors replaced by lofted mirror housings of the same extent; thermal and structure as round 3."""
import r3_city_car_s500 as r3
from r5_city_car_kit import mirror_housing


def body():
    out = []
    for solid in r3.body():
        if solid["id"] in ("mirror-l", "mirror-r"):
            s = 1 if solid["id"].endswith("l") else -1
            out.append(mirror_housing(solid["id"], s, 0.765, 0.95, 0.765, 1.08, [(0.0, 0.05, 0.055), (0.3, 0.07, 0.07), (0.85, 0.07, 0.07), (1.0, 0.055, 0.06)], material="plastic-trim", role="detail", tone=0.3))
        else:
            out.append(solid)
    return out


def thermal():
    return r3.thermal()


def structure():
    return r3.structure()


def decisions():
    return r3.decisions()
