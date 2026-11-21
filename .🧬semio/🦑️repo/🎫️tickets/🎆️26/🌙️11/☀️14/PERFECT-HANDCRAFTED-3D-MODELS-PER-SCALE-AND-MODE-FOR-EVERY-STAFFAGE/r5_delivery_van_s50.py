"""🚐 Round 5 1:50 lists of the delivery van: the round 3 hollow panel van with the ellipsoid door mirrors replaced by lofted housings carrying their glass on the rear face, the front bumper cover standing 15 mm proud of the front end instead of a 7 mm skin, and flared lips on both front arches and the left rear arch (the right rear arch lies under the track of the sliding door)."""
import r3_delivery_van_s50 as r3
from r5_delivery_van_kit import arch_lip, mirror, mirror_glass

SIDES = r3.SIDES


def body():
    out = []
    for solid in r3.body():
        i = solid["id"]
        if i in ("mirror-l", "mirror-r", "mirror-glass-l", "mirror-glass-r"):
            continue
        if i == "bumper-front":
            solid = r3.nose("bumper-front", r3.ARCH, x0=2.115, grow=0.015, top=0.56, material="plastic-trim", role="body", tone=0.2)
        out.append(solid)
    for side, s in SIDES:
        out.append(mirror(f"mirror-{side}", s, 1.415, 1.385, 0.955, 1.135, [(0.0, 0.045, 0.1), (0.15, 0.06, 0.135), (0.85, 0.06, 0.135), (1.0, 0.045, 0.115)], material="plastic-trim", role="detail", tone=0.25))
        out.append(mirror_glass(f"mirror-glass-{side}", s, 1.357, 1.385, 1.045, 0.115, 0.075, 0.008, material="glass", role="glazing", tone=0.9))
        out.append(arch_lip(f"arch-lip-f{side}", s, r3.XF, r3.W, **r3.PAINT))
    out.append(arch_lip("arch-lip-rl", 1, r3.XR, r3.W, **r3.PAINT))
    return out


def thermal():
    parts = {s["id"]: s for s in body()}
    return r3.thermal() + r3.shells([parts[i] for i in ("arch-lip-fl", "arch-lip-fr", "arch-lip-rl")], "body-panels")


def structure():
    return r3.structure()


def decisions():
    return r3.decisions() + [{"en": "Round 5: the door mirrors are lofted housings (0.18 m reach, 0.27 m tall, 0.12 m deep) with their glass on the rear face instead of ellipsoids, the front bumper cover stands 15 mm proud of the front end so its edge reads as a real cover, and both front arches and the left rear arch carry a flared lip; the right rear arch has none because the open sliding door runs over it.", "de": "Runde 5: die Außenspiegel sind geloftete Gehäuse (0,18 m Ausladung, 0,27 m hoch, 0,12 m tief) mit dem Glas auf der Rückseite statt Ellipsoiden, die vordere Stoßfängerverkleidung steht 15 mm vor dem Vorderwagen, sodass ihre Kante als echte Verkleidung lesbar ist, und beide vorderen Radläufe sowie der linke hintere tragen eine ausgestellte Lippe; der rechte hintere hat keine, weil die geöffnete Schiebetür darüber läuft."}]
