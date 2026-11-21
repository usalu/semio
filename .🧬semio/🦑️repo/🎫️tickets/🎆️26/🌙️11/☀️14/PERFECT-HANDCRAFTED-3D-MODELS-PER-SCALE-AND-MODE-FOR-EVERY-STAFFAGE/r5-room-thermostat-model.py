"""🎛️ Round 5, appliances/room-thermostat: the round-3 model with a scalloped grip ring at 1:50 (round finger scoops between flat lands instead of a zig-zag star) and the 1:100 load path completed with the potentiometer joint and the two masses; writes `volumes/appliances/room-thermostat.json`.

Usage: python3 r5-room-thermostat-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_room_thermostat_model", os.path.join(HERE, "r3-room-thermostat-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)
K, H, D, CU = M.K, M.H, M.D, M.CU


def scalloped(crest, depth, flutes, per_flute, land):
    """🌀 The grip outline of the knob: `flutes` round finger scoops of `depth`, each followed by a flat land of share `land`, `per_flute` points per flute."""
    out = []
    for k in range(flutes * per_flute):
        phase = (k % per_flute) / per_flute
        dip = depth * math.sin(math.pi * phase / (1 - land)) if phase < 1 - land else 0.0
        a = 2 * math.pi * k / (flutes * per_flute)
        out.append([K.r6((crest - dip) * math.cos(a)), K.r6((crest - dip) * math.sin(a))])
    return out


def scale_50(volume):
    """🔍 1:50: the grip ring scalloped like the moulded knob of the reference products."""
    for solid in volume["body"]:
        if solid["id"] == "dial":
            solid["profile"] = scalloped(0.0210, 0.0011, 24, 8, 0.3)
    volume["decisions"][0]["en"] = volume["decisions"][0]["en"].replace("a 42 mm grip ring with 24 finger flutes", "a 42 mm grip ring with 24 round finger scoops, 1.1 mm deep, between flat lands")
    volume["decisions"][0]["de"] = volume["decisions"][0]["de"].replace("Griffring mit 24 Griffmulden von", "Griffring mit 24 runden, 1,1 mm tiefen Griffmulden zwischen flachen Stegen, von")
    return volume


def scale_100(volume):
    """🔎 1:100: potentiometer joint at the spindle and the masses of board and knob at their centres, all at the 15 mm feature inside the 15 mm housing."""
    volume["structure"] += [
        K.extrude("joint-dial", H, K.ABS, "detail", 0.35, K.circle(0.0075, 16, K.DIAL_U, 0), K.W_FACE, K.W_WALL, **M.member(D, "joint")),
        K.box("mass-housing", H, K.ABS, "detail", 0.15, (0.015, 0.030, 0.015), (CU, 0, 0.006), **M.member(H, "mass")),
        K.ellipsoid("mass-dial", D, K.ABS, "detail", 0.15, (0.0075, 0.0075, 0.0075), (0, 0, -0.006), **M.member(D, "mass")),
    ]
    volume["decisions"][2] = {
        "en": "Structure: nothing in this 15 mm deep device is thinner than the feature, so the load path is drawn at the feature size: the base plate as a 15 mm bar from screw to screw, both screws as 15 mm round bosses on the screw axis, flush with the wall (pin and slot can no longer be told apart), the spindle merged with the knob hub into one 15 mm shaft, the potentiometer as a 15 mm joint disc where the shaft enters the housing, the circuit board as a 15 by 30 mm mass block at the centre of the housing, where its mass lies at 1:50 and the knob's mass as a 15 mm ball at its centre inside the knob.",
        "de": "Tragwerk: Nichts in diesem 15 mm tiefen Gerät ist dünner als das Mindestmaß, der Lastpfad ist deshalb in Mindestmaßgröße gezeichnet: die Grundplatte als 15-mm-Stab von Schraube zu Schraube, beide Schrauben als runde 15-mm-Dome auf der Schraubenachse, bündig mit der Wand (Stift und Langloch sind nicht mehr zu unterscheiden), die Achse mit der Knopfnabe zu einer 15-mm-Welle zusammengefasst, das Potentiometer als 15-mm-Gelenkscheibe, wo die Welle ins Gehäuse tritt, die Leiterplatte als Massenblock von 15 mal 30 mm in der Gehäusemitte, wo seine Masse bei 1:50 liegt und die Masse des Knopfs als 15-mm-Kugel in seiner Mitte.",
    }
    return volume


def volumes():
    """📚 All four scales: round 3 with the round-5 changes."""
    v = M.volumes()
    v["50"] = scale_50(v["50"])
    v["100"] = scale_100(v["100"])
    return v


if __name__ == "__main__":
    path = os.path.join(HERE, "volumes", "appliances", "room-thermostat.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"volumes": volumes()}, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("wrote", path)
