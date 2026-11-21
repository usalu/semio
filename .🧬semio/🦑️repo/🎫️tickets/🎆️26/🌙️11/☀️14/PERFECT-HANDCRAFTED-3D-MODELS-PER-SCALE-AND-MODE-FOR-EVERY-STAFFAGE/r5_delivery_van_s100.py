"""🚐 Round 5 1:100 lists of the delivery van: the round 3 lists with the ellipsoid door mirrors replaced by lofted housings."""
import r3_delivery_van_s100 as r3
from r5_delivery_van_kit import arch_lip, mirror, mirror_glass

SIDES = r3.SIDES


def body():
    out = []
    for solid in r3.body():
        i = solid["id"]
        if i in ("mirror-l", "mirror-r", "mirror-glass-l", "mirror-glass-r"):
            continue
        out.append(solid)
    for side, s in SIDES:
        out.append(mirror(f"mirror-{side}", s, 1.415, 1.385, 0.955, 1.135, [(0.0, 0.045, 0.1), (0.15, 0.06, 0.135), (0.85, 0.06, 0.135), (1.0, 0.045, 0.115)], material="plastic-trim", role="detail", tone=0.25))
    return out


def thermal():
    return r3.thermal()


def structure():
    return r3.structure()


def decisions():
    return r3.decisions() + [{"en": "Round 5: the door mirrors are lofted housings of the real reach and height instead of ellipsoids.", "de": "Runde 5: die Außenspiegel sind geloftete Gehäuse in wahrer Ausladung und Höhe statt Ellipsoiden."}]
