"""W3-CORRECTNESS-FIX: extends the worked example of the en1997 ground derivation fixture to three CPT and three SPT soundings, so the statistics run over investigation points (independent samples) instead of the readings of one CPT. The expected values are blessed afterwards by the oracle (`🐍️.py --bless`).
Usage: /home/deck/semio/.venv/bin/python -I w3-en1997-soundings-fixture.py"""
import json
import pathlib

FIXTURE = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🎯️ground-derivation/🔣️.json")
GAMMA_W = 10000.0


def total_stress(z, gwl=4.0):
    layers = [(0.0, 6.0, 18000.0, 8000.0), (6.0, 9.0, 19000.0, 9000.0), (9.0, 12.0, 20000.0, 10000.0)]
    eff = 0.0
    for top, bottom, gamma, gamma_prime in layers:
        thick = max(min(bottom, z) - top, 0.0)
        dry = min(max(min(bottom, z, gwl) - top, 0.0), thick)
        eff += gamma * dry + gamma_prime * (thick - dry)
    return eff + GAMMA_W * max(z - gwl, 0.0)


def cpt(name, x, rows):
    return {"id": name, "positionX": x, "positionY": 0.0, "groundLevel": 0.0, "depth": 12.0, "kind": "cpt", "readings": [{"id": f"{name}-r{k}", "depth": z, "measurement": total_stress(z) + 15.0 * cu} for k, (z, cu) in enumerate(rows, start=1)]}


def spt(name, x, rows):
    return {"id": name, "positionX": x, "positionY": 0.0, "groundLevel": 0.0, "depth": 12.0, "kind": "spt", "readings": [{"id": f"{name}-r{k}", "depth": z, "measurement": n} for k, (z, n) in enumerate(rows, start=1)]}


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
case = data["cases"][0]
case["description"] = "c_u of a clay (three CPT soundings, three independent samples, statistical) and of a silt (two soundings, cautious), phi' of a sand from three SPT soundings, c_u of the sand (no CPT reading: nothing derivable)"
case["document"]["investigations"] = [
    cpt("cpt-A", 0.0, [(1.0, 40e3), (2.0, 42e3), (3.0, 45e3), (4.0, 41e3), (5.0, 44e3), (7.0, 30e3), (8.0, 28e3)]),
    spt("spt-A", 4.0, [(9.5, 20.0), (10.5, 24.0), (11.5, 28.0)]),
    cpt("cpt-B", 10.0, [(1.5, 38e3), (3.5, 43e3), (4.5, 40e3), (7.5, 33e3)]),
    cpt("cpt-C", 20.0, [(2.5, 46e3), (5.5, 41e3)]),
    spt("spt-B", 14.0, [(10.0, 22.0), (11.0, 26.0)]),
    spt("spt-C", 24.0, [(9.8, 21.0), (11.8, 30.0)]),
]
second = data["cases"][1]
second["description"] = "c_u of a clay from one CPT sounding with 2 readings (one independent sample, n < 3): the lowest reading, cautious"
FIXTURE.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("ok")
