"""🚕 Round 5 1:200 lists of the city-car: the round 3 abstraction for 30 mm features with the lower body opened like the finer scales (nose, tail panel, 60 mm side walls, floor, wheel-house plates, belt rails under the glass mass), lofted mirror housings and wrap-around headlamp lofts; thermal follows, structure as round 3."""
import r3_city_car_s200 as r3
from r3_city_car_kit import XF, XR, box, hull_half, loft, plan_loft, ref, sec
from r5_city_car_kit import FLOOR, WHEELHOUSE_Z, X_FIREWALL, X_TAIL, floor_pan, headlamp, hull_range, mirror_housing, side_wall, wheelhouse_plate

SIDES = r3.SIDES
PAINT = r3.PAINT
WALL = 0.06
TRIM = dict(material="plastic-trim", role="body", tone=0.12)
FLOOR_XS = [X_TAIL, -1.665, -1.558, -1.52, -1.3, -1.08, -1.042, -0.935, -0.93, -0.915, -0.62, -0.55, X_FIREWALL]


def body():
    out = [hull_range("nose", X_FIREWALL, 1.95, 4, **PAINT), hull_range("tail-panel", -1.95, X_TAIL, 4, **PAINT), floor_pan("floor", WALL, FLOOR_XS, 0.03, material="plastic-trim", role="body", tone=0.1)]
    for side, s in SIDES:
        out.append(side_wall(f"side-wall-{side}", s, 4, WALL, **PAINT))
        out.append(loft(f"belt-rail-{side}", [sec([x, 0.975, s * (hull_half(x) - 0.065)], 0.035, 0.07, 6, r3.QX) for x in (-1.765, 0.0, 0.915)], **PAINT))
        out.append(wheelhouse_plate(f"wheelhouse-plate-{side}", s, XR, WHEELHOUSE_Z + 0.015, 0.30, 0.015, **TRIM))
        out.append(wheelhouse_plate(f"wheelhouse-plate-f{side}", s, XF, 0.55, 0.27, 0.015, **TRIM))
    out.append(loft("belt-rail-rear", [sec([-1.745, 0.975, z], 0.035, 0.07, 6) for z in (-0.73, 0.0, 0.73)], **PAINT))
    for solid in r3.body():
        if solid["id"] in ("hull", "underbody"):
            continue
        s = 1 if solid["id"].endswith("-l") else -1
        if solid["id"] in ("mirror-l", "mirror-r"):
            out.append(mirror_housing(solid["id"], s, 0.775, 0.95, 0.77, 1.085, [(0.0, 0.04, 0.045), (0.3, 0.06, 0.062), (0.85, 0.06, 0.062), (1.0, 0.045, 0.05)], material="plastic-trim", role="detail", tone=0.3))
        elif solid["id"] in ("headlamp-l", "headlamp-r"):
            out.append(headlamp(solid["id"], s, 0.715, [1.64, 1.75, 1.85, 1.905], 0.045, 0.02, material="glass", role="glazing", tone=0.9))
        else:
            out.append(solid)
    return out


INWARD = {"nose": ["-u"], "tail-panel": ["+u"], "floor": ["+v"], "belt-rail-rear": ["+u"], "side-wall-l": ["-w"], "side-wall-r": ["+w"], "belt-rail-l": ["-w"], "belt-rail-r": ["+w"], "wheelhouse-plate-l": ["-w"], "wheelhouse-plate-r": ["+w"]}


def thermal():
    panels = ["nose", "tail-panel", "floor", "belt-rail-rear"] + [f"{i}-{side}" for i in ("side-wall", "belt-rail", "wheelhouse-plate") for side in "lr"] + ["wheelhouse-plate-fl", "wheelhouse-plate-fr"]
    out = [ref(f"th-{i}", i, "body-panels", faces=INWARD.get(i)) for i in panels]
    cabin = dict(material="interior-trim", role="detail", tone=0.3, region="interior")
    out.append(box("th-front-cushions", [0.5, 0.12, 1.2], [0.06, 0.53, 0], radius=0.04, **cabin))
    out.append(box("th-rear-cushion", [0.48, 0.12, 1.22], [-0.72, 0.565, 0], radius=0.04, **cabin))
    out.append(plan_loft("th-dashboard", [(0.74, 0.62, 0.79, 0.72), (1.0, 0.52, 0.88, 0.72)], 5, **cabin))
    return out + [solid for solid in r3.thermal() if solid["id"] not in ("th-hull", "th-underbody", "th-cabin-floor")]


def structure():
    return r3.structure()


def decisions():
    return [
        {"en": "At 1:200 the car is drawn for 30 mm features: nose, tail panel and 60 mm side walls on the master hull lines with the wheel arches as four chords, a floor, wheel-house plates and belt rails, so the lower body is the same open shell as at the finer scales; above the belt the glasshouse is one glass mass under the roof with plate pillars on it; bonnet, tailgate, two-part wheels, lofted mirror housings and wrap-around headlamps.", "de": "Im Maßstab 1:200 ist das Auto für 30-mm-Merkmale gezeichnet: Bug, Heckblech und 60 mm starke Seitenwände auf den Rumpflinien mit den Radläufen als vier Sehnen, ein Boden, Radhausbleche und Brüstungsholme, sodass der Unterbau dieselbe offene Schale wie in den feineren Maßstäben ist; über der Brüstung ist das Glashaus eine Glasmasse unter dem Dach mit aufgelegten Plattenpfosten; Haube, Heckklappe, zweiteilige Räder, geloftete Spiegelgehäuse und umlaufende Scheinwerfer."},
        {"en": "Seats, steering wheel, wipers, handles and seams are below what 1:200 prints and are left out of the body; the seat cushions, seat backs and dashboard, the engine, the exhaust pipe and the brake discs are real hidden solids in the thermal list, and the panel faces that look into the cabin are section faces as at the finer scales.", "de": "Sitze, Lenkrad, Wischer, Griffe und Fugen liegen unter dem, was 1:200 druckt, und fehlen im Body; Sitzpolster, Sitzlehnen und Armaturenbrett, Motor, Abgasrohr und Bremsscheiben sind echte verdeckte Körper der thermischen Liste, und die Blechflächen, die in die Kabine blicken, sind wie in den feineren Maßstäben Schnittflächen."},
        r3.decisions()[-1],
    ]
