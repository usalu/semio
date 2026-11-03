"""🚗 Adds a passive interior region to both cars and binds every still-unbound shape."""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"
INTERIOR = re.compile(r"^(seat-|steering-|handle-|door-handle-)")
WHEELS = re.compile(r"^(bead-|sidewall-|tread-|rim-barrel-|spoke-|hub-)")

SPECIAL = {
    "vintage-car-1929": {
        "plan-hood-ridge": "hood", "plan-roof-seam": "roof", "fender-lamp": "fenders",
    },
    "city-car": {
        "windscreen-edge": "glazing", "headlamp": "glazing", "tail-lamp": "tailgate", "hatch-seam": "tailgate",
        "bonnet-crease": "bonnet", "bonnet-crease-l-plan": "bonnet", "bonnet-crease-r-plan": "bonnet",
    },
}

SETUP = {
    "vintage-car-1929": {
        "material": {
            "id": "upholstery",
            "title": {"en": "Leather upholstery on horsehair", "de": "Lederpolster auf Rosshaar"},
            "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 400, "conductivityWmK": 0.08,
            "specificHeatJKgK": 1500, "source": "authored-estimate",
        },
        "region": {
            "id": "interior",
            "title": {"en": "Seats, steering column and door handles", "de": "Sitze, Lenksäule und Türgriffe"},
            "kind": "surface", "model": "passive", "material": "upholstery", "areaM2": 3.5,
            "characteristicLengthM": 0.6, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
            "layers": [{
                "id": "padding", "title": {"en": "Upholstery padding 40 mm", "de": "Polsterung 40 mm"},
                "kind": "insulation", "material": "upholstery", "thicknessM": 0.04, "resistanceM2KW": 0.5,
                "heatCapacityJM2K": 24000, "source": "authored-estimate",
            }],
            "heatCapacityJM2K": 24000, "source": "authored-estimate", "reference": "ashrae-2021",
        },
        "exposed": 0.5,
        "note": (
            "The open cabin has one passive interior region for the bench seats, the steering column and the door handles (3.5 m2, leather over 40 mm padding); it is exposed to the air and to the sun through the open sides.",
            "Die offene Kabine hat eine passive Innenraumregion für die Sitzbänke, die Lenksäule und die Türgriffe (3,5 m2, Leder über 40 mm Polsterung); sie liegt frei in der Luft und in der Sonne durch die offenen Seiten.",
        ),
    },
    "city-car": {
        "region": {
            "id": "interior",
            "title": {"en": "Seats, steering wheel and door handles", "de": "Sitze, Lenkrad und Türgriffe"},
            "kind": "surface", "model": "passive", "material": "interior-trim", "areaM2": 5.0,
            "characteristicLengthM": 0.6, "geometry": "horizontal-plate-up", "skyViewFactor": 0.3,
            "layers": [{
                "id": "foam", "title": {"en": "Seat foam and trim 30 mm", "de": "Sitzschaum und Verkleidung 30 mm"},
                "kind": "insulation", "material": "interior-trim", "thicknessM": 0.03, "resistanceM2KW": 0.3,
                "heatCapacityJM2K": 9000, "source": "authored-estimate",
            }],
            "heatCapacityJM2K": 9000, "source": "authored-estimate", "reference": "ashrae-2021",
        },
        "exposed": 0.15,
        "note": (
            "The closed cabin has one passive interior region for the seats, the steering wheel and the door handles (5 m2, foam and trim); it is enclosed by the glazing and the body and couples weakly to the air.",
            "Die geschlossene Kabine hat eine passive Innenraumregion für die Sitze, das Lenkrad und die Türgriffe (5 m2, Schaum und Verkleidung); sie wird von Verglasung und Karosserie umschlossen und koppelt schwach an die Luft.",
        ),
    },
}


def target(name, shape):
    sid = shape["id"]
    if INTERIOR.match(sid):
        return "interior"
    if sid in SPECIAL[name]:
        return SPECIAL[name][sid]
    if WHEELS.match(sid):
        return "tyres"
    return "body-panels"


for name in sys.argv[2:]:
    mode = sys.argv[1]
    path = ROOT / name / "🔣️.json"
    raw = path.read_bytes().decode("utf-8")
    crlf = "\r\n" in raw
    second = raw.replace("\r\n", "\n").split("\n")[1]
    indent = len(second) - len(second.lstrip())
    doc = json.loads(raw)
    a = doc["asset"]
    setup = SETUP[name]
    regions = a["thermal"]["regions"]
    if not any(r["id"] == "interior" for r in regions):
        if "material" in setup and not any(m["id"] == setup["material"]["id"] for m in a["materials"]):
            a["materials"].append(setup["material"])
        regions.insert([r["id"] for r in regions].index("glazing") + 1, setup["region"])
        for pose in a["poses"]:
            pose["thermal"]["regions"]["interior"] = {"exposedFraction": setup["exposed"], "sunlitFraction": 0.3}
        a["thermal"]["assumptions"].append({"en": setup["note"][0], "de": setup["note"][1]})
    bound = {}
    for key, rep in a["representations"].items():
        if rep.get("status") != "authored":
            continue
        for lst in ("elevation", "plan", "section"):
            for sh in rep["geometry"].get(lst) or []:
                if sh.get("region") or sh.get("regions"):
                    continue
                sh["region"] = target(name, sh)
                bound.setdefault(sh["region"], []).append(f"{key}/{sh['id']}")
    print(name, {k: len(v) for k, v in bound.items()})
    if mode == "stage":
        (Path(__file__).resolve().parents[1] / "🗑️generated" / "cp" / f"{name}.json").write_bytes((json.dumps(doc, ensure_ascii=False, indent=indent) + "\n").encode("utf-8"))
    if mode == "apply":
        text = json.dumps(doc, ensure_ascii=False, indent=indent) + "\n"
        path.write_bytes((text.replace("\n", "\r\n") if crlf else text).encode("utf-8"))
