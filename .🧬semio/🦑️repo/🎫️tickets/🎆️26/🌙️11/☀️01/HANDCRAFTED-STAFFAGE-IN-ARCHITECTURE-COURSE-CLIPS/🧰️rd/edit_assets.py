import json
import sys
from pathlib import Path

import os
os.chdir("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
C = Path(".")
RAD = C / "🧱️components" / "panel-radiator" / "🔣️.json"
KETTLE = C / "🔌️appliances" / "electric-kettle" / "🔣️.json"
POT = C / "🪑️objects" / "cooking-pot" / "🔣️.json"

PLATE_FILM = float(sys.argv[1]) if len(sys.argv) > 1 else 0.003
FIN_FILM = float(sys.argv[2]) if len(sys.argv) > 2 else 0.012
VALVE_FILM = float(sys.argv[3]) if len(sys.argv) > 3 else 0.01
WALL_FILM = float(sys.argv[4]) if len(sys.argv) > 4 else 0.0025

ROWS = [
    {"v": 0.515, "supplyTempC": 55, "returnTempC": 45},
    {"v": 0.748, "supplyTempC": 65, "returnTempC": 55},
    {"v": 1, "supplyTempC": 75, "returnTempC": 65},
]


def load(p):
    return json.loads(p.read_text(encoding="utf-8"))


def save(p, d):
    p.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="")


def region(d, rid):
    return next(r for r in d["asset"]["thermal"]["regions"] if r["id"] == rid)


def radiator():
    d = load(RAD)
    spec = {
        "panel-rear": (0.5, PLATE_FILM),
        "panel-front-upper": (1 / 6, PLATE_FILM),
        "panel-front-mid": (0.5, PLATE_FILM),
        "panel-front-lower": (5 / 6, PLATE_FILM),
        "convector": (0.5, FIN_FILM),
        "valve": (0.0, VALVE_FILM),
    }
    for rid, (pos, film) in spec.items():
        r = region(d, rid)
        r["model"] = "hydronic"
        r.pop("operating", None)
        r["hydronic"] = {"key": "heating", "rows": ROWS, "flowPosition": round(pos, 4), "filmM2KW": film}
    save(RAD, d)


def kettle():
    d = load(KETTLE)
    r = region(d, "shell")
    r["model"] = "coupled"
    r.pop("operating", None)
    r["coupled"] = {"region": "water", "wetFraction": 0.6, "filmM2KW": WALL_FILM}
    save(KETTLE, d)


def pot():
    d = load(POT)
    r = region(d, "wall")
    r["model"] = "coupled"
    r.pop("operating", None)
    r["coupled"] = {"region": "water", "wetFraction": 0.5, "filmM2KW": WALL_FILM}
    save(POT, d)


radiator()
kettle()
pot()
print("ok", PLATE_FILM, FIN_FILM, VALVE_FILM, WALL_FILM)
