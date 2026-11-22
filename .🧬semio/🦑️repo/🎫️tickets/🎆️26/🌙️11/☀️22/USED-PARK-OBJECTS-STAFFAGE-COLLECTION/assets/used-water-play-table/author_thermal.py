"""🌡️ Thermal model of `used-water-play-table`: fourteen passive regions with their single layers, the hand-summed outer areas of the 1:50 parts and the region bindings of every pose.

@see ./author_physics.py — materials and references
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Volume" (`volumes-thermal-coverage`)
"""
import math

from author_geometry import (
    APRON_X, APRON_Y, APRON_Z, BEARER, BEARER_HALF_X, BEARER_Z, BS_FL, BS_RIM, BS_WATER, BS_X, BS_Y0, BS_Z, CAP, CHUTE_R, CLAMP, CRANK, DECK, END_X,
    FOOT_R, FOOT_Y0, GATE_H, GATE_HOLE, GATE_Z, GRIP, HELIX, MUD, PLATE, PLINTH, POOLS, POST_R, POST_Y0, POST_Y1, SHAFT, SPOUT, TR_IN_Z, TUBE, WATER_D,
    chute_line,
)
from author_parts import tx
from author_physics import basin_sheet_area, shoelace, trough_sheet_area

MEASURED = {"screw-steel": 0.572}


def area_estimates():
    """📐 Total outer areas of the regions (m2), hand-summed from the 1:50 parts: both faces of every sheet, the visible faces of every block, the wetted and free faces of the water; `MEASURED` holds the flood measure of the validator where the model shows a different outer area than the hand sum."""
    side = lambda r, h: 2 * math.pi * r * h
    disc = lambda r: math.pi * r * r
    bx = lambda a, b, c: 2 * (a * b + b * c + a * c)
    h0, h1, hw = GATE_HOLE
    pools = sum(bx(x1 - x0, WATER_D, 2 * TR_IN_Z) for x0, x1 in POOLS.values())
    basin_in = BS_X[1] - BS_X[0] - 0.003
    plate_a, _, _ = shoelace(PLATE)
    (xa, ya), (xb, yb) = chute_line()
    chute_len = math.hypot(xa - xb, ya - yb)
    t0, t1, ri, ro = TUBE
    flight = (HELIX[1] - HELIX[0]) / HELIX[4] * 2 * math.pi * (HELIX[3] ** 2 - HELIX[2] ** 2) * 1.12
    _, ri_m, ro_m, h_m = MUD
    est = {
        "trough-steel": 2 * trough_sheet_area() + side(SPOUT[3], SPOUT[1] - SPOUT[0]) * 2,
        "gates-steel": 2 * 2 * (2 * GATE_Z * GATE_H - 2 * hw * (h1 - h0)),
        "basin-steel": 2 * basin_sheet_area(),
        "screw-steel": 2 * plate_a * 2 + side(SHAFT[2], SHAFT[1] - SHAFT[0]) + 2 * flight + bx(CRANK[1] - CRANK[0], CRANK[3] - CRANK[2], 2 * CRANK[4]) + 2 * math.pi * CHUTE_R * chute_len + side(CLAMP[2], CLAMP[1] - CLAMP[0]) + 2 * disc(CAP[3]) + 4 * 0.06 * 0.075 + 0.02,
        "tube-pc": side(ro, t1 - t0) + side(ri, t1 - t0) + 2 * (disc(ro) - disc(ri)),
        "grip-pe": side(GRIP[2], GRIP[1] - GRIP[0]) + side(GRIP[4], GRIP[3] - GRIP[1]) + disc(GRIP[4]),
        "timber-greyed": 2 * bx(2 * APRON_X, APRON_Y[1] - APRON_Y[0], APRON_Z[1] - APRON_Z[0]) + bx(END_X[1] - END_X[0], APRON_Y[1] - APRON_Y[0], 2 * APRON_Z[0]),
        "timber-fresh": bx(END_X[1] - END_X[0], APRON_Y[1] - APRON_Y[0], 2 * APRON_Z[0]),
        "posts-robinia": 4 * (side(POST_R, POST_Y1 - POST_Y0) + 2 * disc(POST_R)),
        "understructure-timber": bx(DECK[1] - DECK[0], DECK[3] - DECK[2], DECK[5] - DECK[4]) + 2 * bx(2 * BEARER_HALF_X, BEARER[1] - BEARER[0], 2 * BEARER_Z),
        "footing-concrete": 4 * (side(FOOT_R, -FOOT_Y0) + 2 * disc(FOOT_R) + side(POST_R, -POST_Y0) + disc(POST_R)) + bx(PLINTH[1] - PLINTH[0], PLINTH[3] - PLINTH[2], 2 * PLINTH[4]),
        "water-pools": pools,
        "water-basin": bx(basin_in, BS_WATER, 2 * BS_Z - 0.003),
        "wet-sand": 2.04 * math.pi * (ro_m ** 2 - ri_m ** 2) + (2 * 0.1535 + 0.028) * 2 * TR_IN_Z + 0.1535 * 0.028,
    }
    return {k: MEASURED.get(k, v) for k, v in est.items()}


def regions(area):
    """🔥 The thermal regions: passive weathered surfaces in the summer sun of a sand play area, no rated power anywhere."""
    def lay(lid, en, de, kind, mat, t, lam, rho, cp, src, ref):
        r = t / lam
        return {"id": lid, "title": tx(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": round(r, 5) if r >= 1e-5 else float(f"{r:.3g}"), "heatCapacityJM2K": round(rho * cp * t), "source": src, "reference": ref}

    def reg(rid, en, de, kind, mat, geom, lc, svf, layer, src="literature", ref="duffie-beckman-2013"):
        return {"id": rid, "title": tx(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": round(area[rid], 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"], "source": src, "reference": ref}

    steel = lambda lid, en, de, t: lay(lid, en, de, "metal", "stainless-scaled", t, 15, 7900, 500, "literature", "en-10088-2")
    return [
        reg("trough-steel", "Stainless channel with flanges, end plates, guides and spout, limescale tide lines and gum on the flange", "Edelstahlrinne mit Flanschen, Stirnblechen, Führungen und Auslauf, Kalkränder und Kaugummi auf dem Flansch", "surface", "stainless-scaled", "horizontal-plate-up", 0.49, 0.8, steel("channel-sheet", "Stainless sheet 1.5 mm", "Edelstahlblech 1,5 mm", 0.0015), ref="incropera-2018"),
        reg("gates-steel", "Two slide sluices, scaled at the waterline", "Zwei Schieber, an der Wasserlinie verkalkt", "surface", "stainless-scaled", "vertical-plate", 0.175, 0.5, steel("gate-plate", "Stainless plate 3 mm", "Edelstahlplatte 3 mm", 0.003), ref="incropera-2018"),
        reg("basin-steel", "Lower stainless basin with its flanges", "Unteres Edelstahlbecken mit Flanschen", "surface", "stainless-scaled", "horizontal-plate-up", 0.42, 0.7, steel("basin-sheet", "Stainless sheet 1.5 mm", "Edelstahlblech 1,5 mm", 0.0015), ref="incropera-2018"),
        reg("screw-steel", "Screw shaft, flight, crank, collar and foot bearing, bracket plates, pins, strap and chute", "Schneckenwelle, Wendel, Kurbel, Bund und Fußlager, Halterplatten, Bolzen, Lasche und Rutsche", "surface", "stainless-scaled", "cylinder", 0.042, 0.5, steel("fitting-wall", "Stainless tube wall and flats 2 mm", "Edelstahlrohrwand und Flachstahl 2 mm", 0.002), ref="incropera-2018"),
        reg("tube-pc", "Clear polycarbonate screw tube, scratched and clouded by limescale", "Klares Schneckenrohr aus Polycarbonat, verkratzt und kalkgetrübt", "glazing", "polycarbonate-clouded", "cylinder", 0.16, 0.6, lay("tube-wall", "Polycarbonate wall 5 mm", "Polycarbonatwand 5 mm", "polymer", "polycarbonate-clouded", 0.005, 0.2, 1200, 1200, "authored-estimate", "iso-10456-2007"), src="authored-estimate", ref="iso-10456-2007"),
        reg("grip-pe", "Crank grip, sun-faded polyethylene", "Kurbelgriff, sonnengebleichtes Polyethylen", "surface", "pe-grip", "cylinder", 0.032, 0.6, lay("grip-wall", "Polyethylene 16 mm to the axis", "Polyethylen 16 mm bis zur Achse", "polymer", "pe-grip", 0.016, 0.4, 950, 1900, "authored-estimate", "iso-10456-2007"), src="authored-estimate", ref="iso-10456-2007"),
        reg("timber-greyed", "Greyed and checked Douglas aprons and head board, worn round on top, a felt-tip tag and sticker residue", "Vergraute und rissige Douglasienzargen und Kopfbrett, oben rund abgegriffen, Filzstift-Tag und Aufkleberreste", "surface", "douglas-greyed", "vertical-plate", 0.24, 0.5, lay("apron-board", "Douglas board 40 mm", "Douglasienbrett 40 mm", "timber", "douglas-greyed", 0.04, 0.13, 500, 1600, "literature", "en-338")),
        reg("timber-fresh", "Replaced end board at the outlet, fresh planed Douglas", "Ersetztes Stirnbrett am Auslauf, frisch gehobelte Douglasie", "surface", "douglas-fresh", "vertical-plate", 0.24, 0.5, lay("fresh-board", "Douglas board 40 mm", "Douglasienbrett 40 mm", "timber", "douglas-fresh", 0.04, 0.12, 470, 1600, "literature", "en-338")),
        reg("posts-robinia", "Robinia round posts, checked and mossy at the foot", "Robinien-Rundpfosten, rissig und am Fuß bemoost", "surface", "robinia-weathered", "cylinder", 0.12, 0.3, lay("post-core", "Robinia to the post axis 60 mm", "Robinie bis zur Pfostenachse 60 mm", "timber", "robinia-weathered", 0.06, 0.17, 750, 1600, "literature", "en-350"), ref="en-350"),
        reg("understructure-timber", "Bearers and deck under the channel, shaded", "Riegel und Boden unter der Rinne, verschattet", "surface", "douglas-greyed", "horizontal-plate-down", 0.5, 0.1, lay("deck-board", "Douglas deck 40 mm", "Douglasienboden 40 mm", "timber", "douglas-greyed", 0.04, 0.13, 500, 1600, "literature", "en-338")),
        reg("footing-concrete", "Concrete point footings and the basin plinth, heads flush with the sand", "Betonpunktfundamente und Beckensockel, Köpfe bündig mit dem Sand", "surface", "concrete-weathered", "horizontal-plate-up", 0.3, 0.5, lay("footing-core", "Concrete to the footing axis 150 mm", "Beton bis zur Fundamentachse 150 mm", "masonry", "concrete-weathered", 0.15, 1.65, 2300, 1000, "literature", "iso-10456-2007"), ref="iso-10456-2007"),
        reg("water-pools", "Standing water in the three pools, 35 mm deep", "Stehendes Wasser in den drei Becken, 35 mm tief", "water", "water", "horizontal-plate-up", 0.49, 0.8, lay("pool-water", "Water 35 mm", "Wasser 35 mm", "shell", "water", 0.035, 0.6, 1000, 4186, "literature", "incropera-2018"), ref="incropera-2018"),
        reg("water-basin", "Water of the lower basin, 110 mm deep", "Wasser des unteren Beckens, 110 mm tief", "water", "water", "horizontal-plate-up", 0.42, 0.7, lay("basin-water", "Water 110 mm", "Wasser 110 mm", "shell", "water", 0.11, 0.6, 1000, 4186, "literature", "incropera-2018"), ref="incropera-2018"),
        reg("wet-sand", "Wet sand: the splash rim under the spout and the silt in pool 3", "Nasser Sand: der Spritzrand unter dem Auslauf und der Schlick in Becken 3", "surface", "sand-wet", "horizontal-plate-up", 0.44, 0.9, lay("wet-sand-layer", "Wet sand 25 mm", "Nasser Sand 25 mm", "masonry", "sand-wet", 0.025, 2.0, 1900, 1380, "literature", "duffie-beckman-2013")),
    ]


def binding(state):
    """🌞 Region bindings of a pose: a July afternoon in the sand play area, the table half under a lime tree; the water and the wet sand evaporate, the footings and the plinth sit in the sand."""
    dry = state in ("maintenance", "winter-drained")
    sun = 0.25 if state == "winter-drained" else 1.0
    b = {
        "trough-steel": {"exposedFraction": 0.75, "sunlitFraction": 0.6 * sun},
        "gates-steel": {"exposedFraction": 1, "sunlitFraction": 0.4 * sun},
        "basin-steel": {"exposedFraction": 0.75, "sunlitFraction": 0.55 * sun, "contact": {"fraction": 0.15, "tempC": 22}},
        "screw-steel": {"exposedFraction": 1, "sunlitFraction": 0.45 * sun},
        "tube-pc": {"exposedFraction": 1, "sunlitFraction": 0.55 * sun},
        "grip-pe": {"exposedFraction": 1, "sunlitFraction": 0.7 * sun},
        "timber-greyed": {"exposedFraction": 0.85, "sunlitFraction": 0.45 * sun},
        "timber-fresh": {"exposedFraction": 0.85, "sunlitFraction": 0.35 * sun},
        "posts-robinia": {"exposedFraction": 0.45, "sunlitFraction": 0.15 * sun, "contact": {"fraction": 0.55, "tempC": 16}},
        "understructure-timber": {"exposedFraction": 0.5, "sunlitFraction": 0.0},
        "footing-concrete": {"exposedFraction": 0.08, "sunlitFraction": 0.05 * sun, "contact": {"fraction": 0.92, "tempC": 16}},
        "water-pools": {"exposedFraction": 0.45, "sunlitFraction": 0.6, "wettedness": 1, "contact": {"fraction": 0.55, "tempC": 30}},
        "water-basin": {"exposedFraction": 0.3, "sunlitFraction": 0.5, "wettedness": 1, "contact": {"fraction": 0.7, "tempC": 26}},
        "wet-sand": {"exposedFraction": 0.55, "sunlitFraction": 0.7 * sun, "wettedness": 0.3 if dry else 0.85, "contact": {"fraction": 0.45, "tempC": 18}},
    }
    if dry:
        b["water-pools"] = {"exposedFraction": 0.02, "sunlitFraction": 0.0, "wettedness": 0, "contact": {"fraction": 0.98, "tempC": 16}}
        b["water-basin"] = {"exposedFraction": 0.02, "sunlitFraction": 0.0, "wettedness": 0, "contact": {"fraction": 0.98, "tempC": 14}}
        b["trough-steel"] = {"exposedFraction": 0.85, "sunlitFraction": 0.65 * sun}
    if state == "maintenance":
        b["tube-pc"] = {"exposedFraction": 1, "sunlitFraction": 0.7}
        b["screw-steel"] = {"exposedFraction": 1, "sunlitFraction": 0.55, "wettedness": 0.5}
    return {"regions": b, "emitters": {}}
